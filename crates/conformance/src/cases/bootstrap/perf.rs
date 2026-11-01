//! The performance of the certified components against the Rust they replace
//! (#277): for each committed `rust-std` package of the bootstrap and of the
//! standard library, the wall time and the peak heap of one call under the
//! pinned rustc in release, and of its Rust oracle on the same input in this
//! process, over the committed corpus and a ladder of input sizes.
//!
//! It is not a conformance case and states no time target (#106 C8): it
//! measures, and judges only that the package and the oracle agree on every
//! run it times. `bootstrap/PERFORMANCE.md` records what it printed.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::time::Instant;

use camino::{Utf8Path, Utf8PathBuf};
use lexlean::calculus::rust::package::Manifest;
use lexlean::calculus::rust::{ast, rt_validate, validate, Profile};
use lexlean::calculus::{check, Value};
use serde_json::{json, Value as Json};
use sha2::{Digest as _, Sha256};

use super::{audit_run, hex, lex_run, scaling_input, text, token_audit_run, Run, Seeded};
use super::{calculus_check, runtime_validate, rust_print, rust_validate, scan};
use crate::production_export as harness;
use crate::rust_build::{cargo_in, workspace, Built};
use crate::rust_harness::{self, Calls};
use crate::support;

/// What the process's allocator observes, so that a call's peak heap can be
/// read: the binary that runs [`measure`] installs a counting allocator.
pub trait Heap: Sync {
    /// Starts a measurement, returning the bytes held now.
    fn mark(&self) -> usize;
    /// The most bytes held since `mark` was returned, above it.
    fn peak_since(&self, mark: usize) -> usize;
}

/// The wall time one input is timed for, in nanoseconds, after its first
/// call: short calls are repeated and their median taken.
const BUDGET: u128 = 200_000_000;
/// The most calls one input is timed on.
const REPEATS: usize = 25;
/// The stack of the threads the calls run on, both sides: the recursive roots
/// recurse with their input, and the ladders are larger than the harness
/// stack of `PD-08` is sized for.
pub const STACK: usize = 4 << 30;
/// The environment variable naming the directory the harness workspace and
/// the record are written to, in place of `target/bootstrap-perf`.
pub const OUT_ENV: &str = "LEXLEAN_PERF_DIR";
/// A byte literal longer than this is moved out of the harness source into a
/// file it includes, so that rustc does not parse it as an array expression.
const INLINE_BYTES: usize = 4_096;

/// One input of a root: its size, its arguments as calculus data, the
/// outcome the oracle gives, and the oracle's call on the same input.
struct Sample {
    size: u64,
    arguments: Vec<Value>,
    expected: Json,
    oracle: Box<dyn Fn()>,
}

/// A root measured on its samples.
struct Subject {
    row: &'static str,
    project: &'static str,
    package: &'static str,
    oracle: &'static str,
    unit: &'static str,
    samples: Vec<Sample>,
}

fn sample(size: usize, run: Run, oracle: impl Fn() + 'static) -> Sample {
    Sample {
        size: size as u64,
        arguments: run.0,
        expected: run.1,
        oracle: Box::new(oracle),
    }
}

/// The longest prefix of `source` of at most `bytes` bytes that ends at a
/// line's end, or the whole source when it is shorter.
fn lines_prefix(source: &str, bytes: usize) -> &str {
    if source.len() <= bytes {
        return source;
    }
    let cut = source[..bytes].rfind('\n').map_or(bytes, |end| end + 1);
    let mut end = cut;
    while !source.is_char_boundary(end) {
        end -= 1;
    }
    &source[..end]
}

/// The environment variable bounding the prefix ladders, in bytes; a
/// superlinear component makes the whole of a large source impractical.
pub const MAX_ENV: &str = "LEXLEAN_PERF_MAX";

/// Prefix sizes from 1 KiB doubling up to the whole of `length`, or to the
/// bound [`MAX_ENV`] sets (256 KiB unless set).
fn ladder(length: usize) -> Vec<usize> {
    let bound = std::env::var(MAX_ENV)
        .ok()
        .and_then(|bytes| bytes.parse().ok())
        .unwrap_or(256 << 10);
    let length = length.min(bound);
    let mut out = Vec::new();
    let mut size = 1_024;
    while size < length {
        out.push(size);
        size *= 2;
    }
    out.push(length);
    out
}

fn bytes_value(value: &[u8]) -> Value {
    Value::Bytes { hex: hex(value) }
}

fn nat_value(value: usize) -> Value {
    Value::Nat {
        value: value.to_string(),
    }
}

fn byte_list(items: impl Iterator<Item = impl AsRef<[u8]>>) -> Value {
    Value::List {
        items: items.map(|item| bytes_value(item.as_ref())).collect(),
    }
}

fn outcome(value: &Value) -> Json {
    serde_json::to_value(value).expect("a value is JSON")
}

/// What a root of the buffer returns when its bytes fit the capacity.
fn written(joined: &[u8]) -> Json {
    outcome(&Value::Ok {
        value: Box::new(bytes_value(joined)),
    })
}

/// The messages the digests are timed on, seeded so that they are the same on
/// every host.
fn messages() -> Vec<Vec<u8>> {
    let mut seeded = Seeded(0x5A25_6E5D_0000_0277);
    [64, 1_024, 16_384, 262_144, 1_048_576, 4_194_304]
        .into_iter()
        .map(|length| {
            (0..length)
                .map(|_| u8::try_from(seeded.below(256)).expect("a byte"))
                .collect()
        })
        .collect()
}

/// The longest target module, the Lean text the audit reads token by token
/// in production; the library's other modules are notation, which the lexer
/// refuses within its first few kilobytes.
fn longest_module() -> (String, String, Vec<String>) {
    use lexlean::production::preserve;
    let library = preserve::library().expect("the shipped registry");
    let mut imports: Vec<String> = preserve::TARGET_MODULES
        .iter()
        .map(|module| (*module).to_owned())
        .collect();
    imports.push("Init".to_owned());
    for module in &library.modules {
        imports.push(format!("{}.{module}", preserve::LIBRARY_PREFIX));
    }
    let (name, text) = preserve::TARGET_MODULES
        .iter()
        .map(|module| {
            (
                (*module).to_owned(),
                preserve::module_text(module)
                    .expect("module text")
                    .to_owned(),
            )
        })
        .max_by_key(|(_, text)| text.len())
        .expect("a target module");
    // The module's own imports, so that the audit walks the whole text rather
    // than refusing the first of them.
    imports.extend(
        text.lines()
            .filter_map(|line| line.strip_prefix("public import "))
            .map(str::to_owned),
    );
    (name, text, imports)
}

/// The environment variable naming how many of a subject's largest inputs to
/// leave out, when a harness holding them does not compile in the memory there
/// is: its literals, not the package, are what rustc cannot hold.
pub const DROP_ENV: &str = "LEXLEAN_PERF_DROP";

/// The subjects whose committed corpus is whole programs or crates, rendered
/// as literals, of which one input per doubling of size is timed.
const WHOLE: [&str; 4] = [
    "check_program",
    "validate_crate",
    "correspond_crate",
    "print_crate",
];

/// The largest of `samples` in each doubling of size.
fn octaves(samples: Vec<Sample>) -> Vec<Sample> {
    let mut kept: BTreeMap<u32, Sample> = BTreeMap::new();
    for item in samples {
        let octave = item.size.max(1).ilog2();
        if kept.get(&octave).is_none_or(|held| held.size < item.size) {
            kept.insert(octave, item);
        }
    }
    kept.into_values().collect()
}

#[allow(clippy::too_many_lines)]
fn subjects(selected: &dyn Fn(&str) -> bool) -> Vec<Subject> {
    let mut out = Vec::new();
    let mut add = |row: &'static str,
                   project: &'static str,
                   package: &'static str,
                   oracle: &'static str,
                   unit: &'static str,
                   build: &dyn Fn() -> Vec<Sample>| {
        if selected(package) {
            eprintln!("perf: the corpus of {package}");
            out.push(Subject {
                row,
                project,
                package,
                oracle,
                unit,
                samples: {
                    let mut samples = build();
                    if WHOLE.contains(&package) {
                        samples = octaves(samples);
                    }
                    samples.sort_by_key(|item| item.size);
                    let left_out: usize = std::env::var(DROP_ENV)
                        .ok()
                        .and_then(|count| count.parse().ok())
                        .unwrap_or(0);
                    samples.truncate(samples.len().saturating_sub(left_out));
                    samples
                },
            });
        }
    };

    add(
        "BS-01",
        "bootstrap",
        "parse_audit_output",
        "verify::axiom::parse_audit_output",
        "bytes",
        &|| {
            [125, 250, 500, 1_000, 2_000, 4_000, 8_000]
                .into_iter()
                .map(|records| {
                    let (stdout, names) = scaling_input(records);
                    let run = audit_run(&stdout, &names);
                    sample(stdout.len(), run, move || {
                        std::hint::black_box(lexlean::verify::axiom::parse_audit_output(
                            &stdout, &names,
                        ))
                        .ok();
                    })
                })
                .collect()
        },
    );
    add(
        "BS-01",
        "bootstrap",
        "is_closed_lean_name",
        "ir::core::is_closed_lean_name",
        "bytes",
        &|| {
            [1, 4, 16, 64, 256, 1_024, 4_096]
                .into_iter()
                .map(|segments| {
                    let name = (0..segments)
                        .map(|index| format!("Seg{index}"))
                        .collect::<Vec<_>>()
                        .join(".");
                    let run = (
                        vec![text(&name)],
                        outcome(&Value::Bool {
                            value: lexlean::ir::core::is_closed_lean_name(&name),
                        }),
                    );
                    sample(name.len(), run, move || {
                        std::hint::black_box(lexlean::ir::core::is_closed_lean_name(&name));
                    })
                })
                .collect()
        },
    );
    add(
        "BS-02",
        "bootstrap",
        "lex",
        "production::preserve::lex",
        "bytes",
        &|| {
            let (name, module, _) = longest_module();
            eprintln!("perf: lex ladder on {name} ({} bytes)", module.len());
            ladder(module.len())
                .into_iter()
                .filter_map(|size| {
                    let input = lines_prefix(&module, size).to_owned();
                    lexlean::production::preserve::lex(&input).ok()?;
                    let run = lex_run(&input);
                    Some(sample(input.len(), run, move || {
                        std::hint::black_box(lexlean::production::preserve::lex(&input)).ok();
                    }))
                })
                .collect()
        },
    );
    add(
        "BS-02",
        "bootstrap",
        "audit_tokens",
        "production::preserve::audit_tokens",
        "bytes",
        &|| {
            let (_, module, imports) = longest_module();
            let set: BTreeSet<String> = imports.iter().cloned().collect();
            ladder(module.len())
                .into_iter()
                .filter_map(|size| {
                    let input = lines_prefix(&module, size).to_owned();
                    lexlean::production::preserve::lex(&input).ok()?;
                    let run = token_audit_run(&input, &imports);
                    let set = set.clone();
                    Some(sample(input.len(), run, move || {
                        std::hint::black_box(lexlean::production::preserve::audit_tokens(
                            &input, &set,
                        ))
                        .ok();
                    }))
                })
                .collect()
        },
    );
    add(
        "BS-03",
        "bootstrap",
        "scan_source",
        "source::scan::scan",
        "bytes",
        &|| {
            let (path, source) = scan::sources()
                .into_iter()
                .max_by_key(|(_, source)| source.len())
                .expect("a committed source");
            eprintln!("perf: scan ladder on {path} ({} bytes)", source.len());
            let mut tally = scan::Tally::default();
            ladder(source.len())
                .into_iter()
                .map(|size| {
                    let input = lines_prefix(&source, size).to_owned();
                    let run = scan::scan_run(&mut tally, &path, &input, 2_000_000);
                    let path = path.clone();
                    sample(input.len(), run, move || {
                        std::hint::black_box(lexlean::source::scan::scan(&path, &input, 2_000_000))
                            .ok();
                    })
                })
                .collect()
        },
    );
    add(
        "BS-03",
        "bootstrap",
        "reject_atoms",
        "source::scan::reject_forbidden_atoms",
        "atoms",
        &|| {
            let (path, source) = scan::sources()
                .into_iter()
                .max_by_key(|(_, source)| source.len())
                .expect("a committed source");
            let forbidden = scan::forbidden_controls();
            let mut tally = scan::Tally::default();
            ladder(source.len())
                .into_iter()
                .filter_map(|size| {
                    let input = lines_prefix(&source, size);
                    let atoms = lexlean::source::scan::scan(&path, input, 2_000_000).ok()?;
                    let run = scan::reject_run(&mut tally, &path, &atoms, &forbidden);
                    let (path, forbidden) = (path.clone(), forbidden.clone());
                    let count = atoms.len();
                    Some(sample(count, run, move || {
                        std::hint::black_box(lexlean::source::scan::reject_forbidden_atoms(
                            &path, &atoms, &forbidden,
                        ))
                        .ok();
                    }))
                })
                .collect()
        },
    );
    add(
        "BS-04",
        "bootstrap",
        "check_program",
        "calculus::check::check",
        "nodes",
        &|| {
            calculus_check::committed()
                .into_iter()
                .map(|source| {
                    let run = calculus_check::program_run(&source.program);
                    let program = source.program;
                    sample(calculus_check::program_size(&program), run, move || {
                        std::hint::black_box(check::check(&program)).ok();
                    })
                })
                .collect()
        },
    );
    add(
        "BS-04",
        "bootstrap",
        "check_arguments",
        "calculus::check::check_arguments",
        "nodes",
        &|| {
            calculus_check::committed()
                .into_iter()
                .filter_map(|source| {
                    let (entry, arguments) = source.entry?;
                    let run = calculus_check::arguments_run(&source.program, entry, &arguments);
                    let program = source.program;
                    Some(sample(
                        calculus_check::program_size(&program),
                        run,
                        move || {
                            std::hint::black_box(check::check_arguments(
                                &program, entry, &arguments,
                            ))
                            .ok();
                        },
                    ))
                })
                .collect()
        },
    );
    add(
        "BS-05",
        "bootstrap",
        "validate_crate",
        "calculus::rust::validate::validate",
        "expressions",
        &|| {
            rust_validate::rendered(&rust_validate::committed().0)
                .into_iter()
                .map(|rendered| {
                    let run = rust_validate::validate_run(&rendered.krate);
                    let krate = rendered.krate;
                    sample(rust_validate::weight(&krate), run, move || {
                        std::hint::black_box(validate::validate(&krate)).ok();
                    })
                })
                .collect()
        },
    );
    add(
        "BS-05",
        "bootstrap",
        "correspond_crate",
        "calculus::rust::validate::correspond",
        "expressions",
        &|| {
            rust_validate::rendered(&rust_validate::committed().0)
                .into_iter()
                .map(|rendered| {
                    let run = rust_validate::correspond_run(&rendered.krate, &rendered.elements);
                    let (krate, elements) = (rendered.krate, rendered.elements);
                    sample(rust_validate::weight(&krate), run, move || {
                        std::hint::black_box(validate::correspond(&krate, &elements)).ok();
                    })
                })
                .collect()
        },
    );
    add(
        "BS-06",
        "bootstrap",
        "print_crate",
        "calculus::rust::ast::print",
        "bytes out",
        &|| {
            let headers = [
                (Profile::Core, rust_print::header(Profile::Core)),
                (Profile::Std, rust_print::header(Profile::Std)),
            ];
            let mut seen = BTreeSet::new();
            rust_print::packaged()
                .into_iter()
                .filter(|packaged| seen.insert(format!("{:?}", packaged.krate)))
                .filter_map(|packaged| {
                    let krate = packaged.krate;
                    let header = &headers
                        .iter()
                        .find(|(profile, _)| *profile == krate.profile)
                        .expect("a profile")
                        .1;
                    let run = rust_print::exact(&krate, header)?;
                    let printed = ast::print(&krate).ok()?.len();
                    Some(sample(printed, run, move || {
                        std::hint::black_box(ast::print(&krate)).ok();
                    }))
                })
                .collect()
        },
    );
    add(
        "BS-06",
        "bootstrap",
        "print_expr",
        "calculus::rust::ast::print_expr",
        "bytes out",
        &|| {
            let mut seen = BTreeSet::new();
            let bodies: Vec<_> = rust_print::packaged()
                .iter()
                .flat_map(|packaged| rust_print::bodies(&packaged.krate))
                .filter(|body| seen.insert(format!("{body:?}")))
                .collect();
            // Every fourth distinct body, so the harness stays a few
            // thousand literals; the largest is always kept.
            let largest = bodies
                .iter()
                .enumerate()
                .max_by_key(|(_, body)| ast::print_expr(body).len())
                .map(|(index, _)| index);
            bodies
                .into_iter()
                .enumerate()
                .filter(|(index, _)| index % 4 == 0 || Some(*index) == largest)
                .filter_map(|(_, body)| {
                    let run = rust_print::exact_expr(&body)?;
                    let printed = ast::print_expr(&body).len();
                    Some(sample(printed, run, move || {
                        std::hint::black_box(ast::print_expr(&body));
                    }))
                })
                .collect()
        },
    );
    add(
        "BS-09",
        "bootstrap",
        "validate_runtime",
        "calculus::rust::rt_validate::validate",
        "nodes",
        &|| {
            runtime_validate::named()
                .into_iter()
                .map(|(_, krate)| {
                    let run = runtime_validate::run_of(&krate);
                    let nodes = rust_validate::value_nodes(&run.0[0]);
                    sample(nodes, run, move || {
                        std::hint::black_box(rt_validate::validate(&krate)).ok();
                    })
                })
                .collect()
        },
    );
    add(
        "SL-02",
        "stdlib",
        "sha256",
        "sha2::Sha256",
        "bytes",
        &|| {
            messages()
                .into_iter()
                .map(|message| {
                    let run = (
                        vec![bytes_value(&message)],
                        outcome(&Value::Adt {
                            constructor: 0,
                            fields: Sha256::digest(&message)
                                .chunks(4)
                                .map(|word| Value::U32 {
                                    value: u32::from_be_bytes(word.try_into().expect("four bytes"))
                                        .to_string(),
                                })
                                .collect(),
                        }),
                    );
                    sample(message.len(), run, move || {
                        std::hint::black_box(Sha256::digest(&message));
                    })
                })
                .collect()
        },
    );
    add(
        "SL-02",
        "stdlib",
        "sha256_hex",
        "sha2::Sha256 and hex",
        "bytes",
        &|| {
            messages()
                .into_iter()
                .map(|message| {
                    let digest = hex(&Sha256::digest(&message));
                    let run = (
                        vec![bytes_value(&message)],
                        outcome(&bytes_value(digest.as_bytes())),
                    );
                    sample(message.len(), run, move || {
                        std::hint::black_box(hex(&Sha256::digest(&message)));
                    })
                })
                .collect()
        },
    );
    let sizes = [1_024, 4_096, 16_384, 65_536, 262_144];
    let subject = |length: usize| -> Vec<u8> {
        (0..length)
            .map(|index| b"abcdefghijklmnop"[index % 16])
            .collect()
    };
    // The buffer's roots with a capacity that just fits, to the size of the
    // largest committed Rust library and past it.
    add(
        "SL-05",
        "stdlib",
        "write_each",
        "Vec::push",
        "bytes",
        &|| {
            sizes
                .into_iter()
                .map(|length| {
                    let data = subject(length);
                    let run = (vec![nat_value(length), bytes_value(&data)], written(&data));
                    sample(length, run, move || {
                        let mut out = Vec::new();
                        for byte in &data {
                            out.push(*byte);
                        }
                        std::hint::black_box(out);
                    })
                })
                .collect()
        },
    );
    add(
        "SL-05",
        "stdlib",
        "write_all",
        "Vec::extend_from_slice",
        "bytes",
        &|| {
            sizes
                .into_iter()
                .map(|length| {
                    let data = subject(length);
                    let pieces: Vec<Vec<u8>> = data.chunks(64).map(<[u8]>::to_vec).collect();
                    let run = (
                        vec![nat_value(length), byte_list(pieces.iter())],
                        written(&data),
                    );
                    sample(length, run, move || {
                        let mut out = Vec::new();
                        for piece in &pieces {
                            out.extend_from_slice(piece);
                        }
                        std::hint::black_box(out);
                    })
                })
                .collect()
        },
    );
    add(
        "SL-05",
        "stdlib",
        "write_numbers",
        "write! into a Vec",
        "bytes",
        &|| {
            [256_u64, 1_024, 4_096, 16_384]
                .into_iter()
                .map(|count| {
                    let numbers: Vec<u64> = (0..count)
                        .map(|index| index.wrapping_mul(0x9E37_79B9_7F4A_7C15))
                        .collect();
                    let joined: Vec<u8> = numbers
                        .iter()
                        .flat_map(|number| format!("{number},").into_bytes())
                        .collect();
                    let run = (
                        vec![
                            nat_value(joined.len()),
                            Value::List {
                                items: numbers
                                    .iter()
                                    .map(|number| Value::Nat {
                                        value: number.to_string(),
                                    })
                                    .collect(),
                            },
                        ],
                        written(&joined),
                    );
                    sample(joined.len(), run, move || {
                        let mut out = Vec::new();
                        for number in &numbers {
                            std::io::Write::write_fmt(&mut out, format_args!("{number},"))
                                .expect("a write to a vector");
                        }
                        std::hint::black_box(out);
                    })
                })
                .collect()
        },
    );
    add(
        "SL-01",
        "stdlib",
        "starts_with",
        "<[u8]>::starts_with",
        "bytes",
        &|| {
            sizes
                .into_iter()
                .map(|length| {
                    let whole = subject(length);
                    let prefix = whole[..length / 2].to_vec();
                    let run = (
                        vec![bytes_value(&whole), bytes_value(&prefix)],
                        outcome(&Value::Bool {
                            value: whole.starts_with(&prefix),
                        }),
                    );
                    sample(length, run, move || {
                        std::hint::black_box(whole.starts_with(&prefix));
                    })
                })
                .collect()
        },
    );
    add(
        "SL-01",
        "stdlib",
        "ends_with",
        "<[u8]>::ends_with",
        "bytes",
        &|| {
            sizes
                .into_iter()
                .map(|length| {
                    let whole = subject(length);
                    let suffix = whole[length / 2..].to_vec();
                    let run = (
                        vec![bytes_value(&whole), bytes_value(&suffix)],
                        outcome(&Value::Bool {
                            value: whole.ends_with(&suffix),
                        }),
                    );
                    sample(length, run, move || {
                        std::hint::black_box(whole.ends_with(&suffix));
                    })
                })
                .collect()
        },
    );
    add(
        "SL-01",
        "stdlib",
        "trim",
        "<[u8]>::trim_ascii",
        "bytes",
        &|| {
            sizes
                .into_iter()
                .map(|length| {
                    let mut padded = vec![b' '; length / 4];
                    padded.extend(subject(length / 2));
                    padded.extend(vec![b'\t'; length / 4]);
                    let run = (
                        vec![bytes_value(&padded)],
                        outcome(&bytes_value(padded.trim_ascii())),
                    );
                    sample(length, run, move || {
                        std::hint::black_box(padded.trim_ascii());
                    })
                })
                .collect()
        },
    );
    add(
        "SL-01",
        "stdlib",
        "split_on",
        "<[u8]>::split",
        "bytes",
        &|| {
            sizes
                .into_iter()
                .map(|length| {
                    let mut whole = subject(length);
                    for index in (15..length).step_by(16) {
                        whole[index] = b',';
                    }
                    let run = (
                        vec![
                            bytes_value(&whole),
                            Value::U8 {
                                value: b','.to_string(),
                            },
                        ],
                        outcome(&byte_list(whole.split(|byte| *byte == b','))),
                    );
                    sample(length, run, move || {
                        std::hint::black_box(
                            whole
                                .split(|byte| *byte == b',')
                                .map(<[u8]>::to_vec)
                                .collect::<Vec<_>>(),
                        );
                    })
                })
                .collect()
        },
    );
    add("SL-01", "stdlib", "lines", "str::lines", "bytes", &|| {
        sizes
            .into_iter()
            .map(|length| {
                let mut whole = subject(length);
                for index in (39..length).step_by(40) {
                    whole[index] = b'\n';
                }
                let whole = String::from_utf8(whole).expect("ASCII");
                let run = (
                    vec![bytes_value(whole.as_bytes())],
                    outcome(&byte_list(whole.lines().map(str::as_bytes))),
                );
                sample(length, run, move || {
                    std::hint::black_box(whole.lines().map(str::to_owned).collect::<Vec<_>>());
                })
            })
            .collect()
    });
    add(
        "SL-01",
        "stdlib",
        "join_with",
        "<[Vec<u8>]>::join",
        "bytes",
        &|| {
            sizes
                .into_iter()
                .map(|length| {
                    let pieces: Vec<Vec<u8>> =
                        subject(length).chunks(16).map(<[u8]>::to_vec).collect();
                    let run = (
                        vec![byte_list(pieces.iter()), bytes_value(b", ")],
                        outcome(&bytes_value(&pieces.join(&b", "[..]))),
                    );
                    sample(length, run, move || {
                        std::hint::black_box(pieces.join(&b", "[..]));
                    })
                })
                .collect()
        },
    );
    add("SL-01", "stdlib", "find", "str::find", "bytes", &|| {
        sizes
            .into_iter()
            .map(|length| {
                let mut haystack = "a".repeat(length - 2);
                haystack.push_str("ab");
                let needle = "ab".to_owned();
                let found = haystack.find(&needle);
                let run = (
                    vec![
                        bytes_value(haystack.as_bytes()),
                        bytes_value(needle.as_bytes()),
                    ],
                    outcome(&match found {
                        Some(at) => Value::Some {
                            value: Box::new(nat_value(at)),
                        },
                        None => Value::None,
                    }),
                );
                sample(length, run, move || {
                    std::hint::black_box(haystack.find(&needle));
                })
            })
            .collect()
    });
    out
}

/// Time `call` on its first run and then until [`BUDGET`] or [`REPEATS`]:
/// the median nanoseconds, the first run's peak heap above what was held
/// before it, and the number of runs.
fn time(heap: &dyn Heap, call: &dyn Fn()) -> (u128, usize, usize) {
    let mut times = Vec::new();
    let mut peak = 0;
    let mut total = 0;
    loop {
        let mark = heap.mark();
        let started = Instant::now();
        call();
        let took = started.elapsed().as_nanos();
        if times.is_empty() {
            peak = heap.peak_since(mark);
        }
        times.push(took);
        total += took;
        if total >= BUDGET || times.len() >= REPEATS {
            break;
        }
    }
    times.sort_unstable();
    (times[times.len() / 2], peak, times.len())
}

/// The counting allocator and the timed call the harness of a package adds
/// to the one `rust_harness` renders: the same measurement as [`time`].
const PRELUDE: &str = r"use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering as Atomic};
struct Counting;
static HELD: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let block = unsafe { System.alloc(layout) };
        if !block.is_null() { PEAK.fetch_max(HELD.fetch_add(layout.size(), Atomic::Relaxed) + layout.size(), Atomic::Relaxed); }
        block
    }
    unsafe fn dealloc(&self, block: *mut u8, layout: Layout) {
        unsafe { System.dealloc(block, layout) };
        HELD.fetch_sub(layout.size(), Atomic::Relaxed);
    }
    unsafe fn realloc(&self, block: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        let moved = unsafe { System.realloc(block, layout, size) };
        if !moved.is_null() {
            if size >= layout.size() { PEAK.fetch_max(HELD.fetch_add(size - layout.size(), Atomic::Relaxed) + size - layout.size(), Atomic::Relaxed); }
            else { HELD.fetch_sub(layout.size() - size, Atomic::Relaxed); }
        }
        moved
    }
}
#[global_allocator]
static ALLOCATOR: Counting = Counting;
fn heap_mark() -> usize { let now = HELD.load(Atomic::Relaxed); PEAK.store(now, Atomic::Relaxed); now }
fn heap_peak(mark: usize) -> usize { PEAK.load(Atomic::Relaxed).saturating_sub(mark) }
";

/// The parameters and the result type of the export `run` of a rendered
/// library, read from its one-line signature.
fn signature(library: &str, arity: usize) -> (Vec<String>, Option<String>) {
    let line = library
        .lines()
        .rfind(|line| line.starts_with("pub fn run("))
        .expect("the library exports run");
    let inner = &line["pub fn run(".len()..];
    let (parameters, rest) = if arity == 0 {
        (Vec::new(), inner.strip_prefix(')').expect("no parameters"))
    } else {
        let mut starts: Vec<usize> = (0..arity)
            .map(|position| inner.find(&format!("p{position}: ")).expect("a parameter"))
            .collect();
        let close = inner
            .rfind(") ->")
            .or_else(|| inner.rfind(") {"))
            .expect("the close");
        starts.push(close + 2);
        let parameters = starts
            .windows(2)
            .map(|pair| inner[pair[0]..pair[1] - 2].to_owned())
            .collect();
        (parameters, &inner[close + 1..])
    };
    let result = rest
        .trim()
        .strip_prefix("->")
        .map(|result| result.trim().trim_end_matches('{').trim().to_owned());
    (parameters, result)
}

/// The timed call through `run`: [`time`] in the harness's own text.
fn timed_run(library: &str, arity: usize) -> String {
    let (parameters, result) = signature(library, arity);
    let names: Vec<String> = (0..arity).map(|position| format!("p{position}")).collect();
    let copies: Vec<String> = names.iter().map(|name| format!("{name}.clone()")).collect();
    let result = result.map_or(String::new(), |result| format!(" -> {result}"));
    format!(
        "\n#[inline(never)]\nfn timed_run({}){result} {{\n    let mut times: Vec<u128> = Vec::new();\n    let mut first = None;\n    let mut peak = 0;\n    let mut total: u128 = 0;\n    loop {{\n        let copies = ({},);\n        let mark = heap_mark();\n        let started = std::time::Instant::now();\n        let value = std::hint::black_box(run({}));\n        let took = started.elapsed().as_nanos();\n        if first.is_none() {{ peak = heap_peak(mark); first = Some(value); }} else {{ drop(value); }}\n        times.push(took);\n        total += took;\n        if total >= {BUDGET} || times.len() >= {REPEATS} {{ break; }}\n    }}\n    times.sort_unstable();\n    eprintln!(\"perf {{}} {{}} {{}}\", times[times.len() / 2], peak, times.len());\n    first.expect(\"one run\")\n}}\n",
        parameters.join(", "),
        copies.join(", "),
        (0..arity)
            .map(|position| format!("copies.{position}"))
            .collect::<Vec<_>>()
            .join(", "),
    )
}

/// `source` with each byte literal longer than [`INLINE_BYTES`] written to
/// a file of `dir` and included from there.
fn outline(source: &str, dir: &std::path::Path) -> String {
    const OPEN: &str = "Bytes::lit(&[";
    let mut out = String::with_capacity(source.len());
    let mut rest = source;
    let mut blobs = 0;
    while let Some(at) = rest.find(OPEN) {
        let body = &rest[at + OPEN.len()..];
        let close = body.find(']').expect("a closed byte literal");
        out.push_str(&rest[..at]);
        let elements = &body[..close];
        if elements.len() / 6 > INLINE_BYTES {
            let octets: Vec<u8> = elements
                .split(", ")
                .map(|element| {
                    u8::from_str_radix(element.trim().trim_start_matches("0x"), 16)
                        .expect("a hexadecimal byte")
                })
                .collect();
            let name = format!("blob{blobs}.bin");
            blobs += 1;
            std::fs::write(dir.join(&name), octets).expect("a blob");
            // The source's own `)` follows and closes `lit`.
            let _ = write!(out, "Bytes::lit(include_bytes!(\"{name}\")");
        } else {
            out.push_str(OPEN);
            out.push_str(elements);
            out.push(']');
        }
        rest = &body[close + 1..];
    }
    out.push_str(rest);
    out
}

/// The host's available memory in GiB (`/proc/meminfo`), else `-`, recorded
/// with the load.
fn memory() -> String {
    let available = || -> Option<u64> {
        let info = std::fs::read_to_string("/proc/meminfo").ok()?;
        let line = info
            .lines()
            .find(|line| line.starts_with("MemAvailable:"))?;
        line.split_whitespace().nth(1)?.parse().ok()
    };
    available().map_or_else(|| "-".to_owned(), |kib: u64| format!("{}", kib >> 20))
}

/// The machine's one-minute load average where the host reports one
/// (`/proc/loadavg`), else `-`: the timings are only as quiet as the host.
fn load() -> String {
    std::fs::read_to_string("/proc/loadavg")
        .ok()
        .and_then(|line| line.split(' ').next().map(str::to_owned))
        .unwrap_or_else(|| "-".to_owned())
}

/// A least-squares slope of `ln y` against `ln x`: the exponent of the
/// growth, 1 linear and 2 quadratic.
#[allow(clippy::cast_precision_loss)]
fn exponent(points: &[(u64, u128)]) -> Option<f64> {
    let points: Vec<(f64, f64)> = points
        .iter()
        .filter(|(x, y)| *x > 0 && *y > 0)
        .map(|(x, y)| ((*x as f64).ln(), (*y as f64).ln()))
        .collect();
    let distinct: BTreeSet<u64> = points.iter().map(|(x, _)| x.to_bits()).collect();
    if distinct.len() < 3 {
        return None;
    }
    let n = points.len() as f64;
    let (sx, sy) = points
        .iter()
        .fold((0.0, 0.0), |(sx, sy), (x, y)| (sx + x, sy + y));
    let (mx, my) = (sx / n, sy / n);
    let (num, den) = points.iter().fold((0.0, 0.0), |(num, den), (x, y)| {
        (num + (x - mx) * (y - my), den + (x - mx) * (x - mx))
    });
    (den > 0.0).then(|| num / den)
}

/// The points within six doublings of the largest size, where a fixed cost no
/// longer hides the growth; all of them when those are too few to fit.
fn upper(points: &[(u64, u128)]) -> Vec<(u64, u128)> {
    let largest = points.iter().map(|point| point.0).max().unwrap_or(0);
    let near: Vec<(u64, u128)> = points
        .iter()
        .copied()
        .filter(|point| point.0.saturating_mul(64) >= largest)
        .collect();
    let distinct: BTreeSet<u64> = near.iter().map(|point| point.0).collect();
    if distinct.len() < 3 {
        points.to_vec()
    } else {
        near
    }
}

/// Measure every subject `selected` admits and write the record into
/// `target/bootstrap-perf/`; returns the Markdown table it printed.
///
/// # Panics
///
/// Panics when a package does not build, a harness fails, or a package and
/// its oracle disagree on a run they were timed on.
#[allow(clippy::too_many_lines, clippy::cast_precision_loss)]
pub fn measure(heap: &dyn Heap, selected: &dyn Fn(&str) -> bool) -> String {
    let root = support::repo_root();
    // The root is the one this binary was compiled in; a target directory
    // shared by several checkouts can hand it another checkout's.
    let here = std::env::current_dir().expect("the working directory");
    assert_eq!(
        std::fs::canonicalize(root.as_std_path()).expect("the compiled-in root"),
        std::fs::canonicalize(&here).expect("the working directory"),
        "the compiled-in root {root} is not the checkout this runs in"
    );
    eprintln!("perf: the corpus is read from {root}");
    let subjects = subjects(selected);
    // Each committed package with a harness that times `run` on the inputs
    // its oracle is timed on; all are built before any is timed, so that the
    // two sides of a component are timed back to back.
    let out = std::env::var(OUT_ENV)
        .map(Utf8PathBuf::from)
        .unwrap_or_else(|_| root.join("target/bootstrap-perf"));
    let dir = out.join("ws");
    if dir.exists() {
        std::fs::remove_dir_all(dir.as_std_path()).expect("the old workspace is removed");
    }
    std::fs::create_dir_all(dir.as_std_path()).expect("the workspace");
    let mut crates = Vec::new();
    let mut harnesses = Vec::new();
    for subject in &subjects {
        let package: &Utf8Path = &root
            .join("tests/production-export")
            .join(subject.project)
            .join("packages/rust-std")
            .join(subject.package);
        let files = harness::tree(package);
        let manifest = Manifest::parse(&files["package.json"]).expect("a committed manifest");
        let library = String::from_utf8(files["src/lib.rs"].clone()).expect("UTF-8");
        let export = &manifest.exports[0];
        let arguments: Vec<Vec<Value>> = subject
            .samples
            .iter()
            .map(|item| item.arguments.clone())
            .collect();
        let rendered = rust_harness::render_calls(
            &manifest.program,
            Profile::Std,
            &manifest.name,
            &[Calls {
                entry: export.function,
                function: "timed_run",
                passing: &export.parameters,
                arguments: &arguments,
            }],
        )
        .unwrap_or_else(|failure| panic!("{}: {failure}", subject.package));
        let arity = manifest.program.functions[usize::try_from(export.function).expect("entry")]
            .types
            .len();
        let source = rendered.replacen("#![forbid(unsafe_code)]\n", PRELUDE, 1)
            + &timed_run(&library, arity);
        harnesses.push(source);
        crates.push(Built {
            name: manifest.name.clone(),
            files,
            harness: String::new(),
        });
    }
    workspace(dir.as_std_path(), &crates);
    for (built, source) in crates.iter().zip(&harnesses) {
        let src = dir
            .join("h")
            .join(format!("run_{}", built.name))
            .join("src");
        let outlined = outline(source, src.as_std_path());
        std::fs::write(src.join("main.rs"), outlined).expect("the harness");
    }
    let mut manifest = std::fs::read_to_string(dir.join("Cargo.toml")).expect("the workspace");
    manifest.push_str("\n[profile.release]\nopt-level = 3\ndebug = false\nincremental = false\n");
    // The harness only builds the inputs and calls the package; it is not
    // optimized, so its literals compile in seconds.
    for built in &crates {
        let _ = write!(
            manifest,
            "\n[profile.release.package.run_{}]\nopt-level = 0\n",
            built.name
        );
    }
    std::fs::write(dir.join("Cargo.toml"), manifest).expect("the workspace");
    let started = Instant::now();
    let build = cargo_in(
        dir.as_std_path(),
        &["build", "--release", "--offline", "--workspace", "--quiet"],
    );
    assert!(
        build.status.success(),
        "the packages do not build:\n{}",
        String::from_utf8_lossy(&build.stderr)
    );
    eprintln!("perf: the packages built in {:?}", started.elapsed());
    let mut record = Vec::new();
    let mut table = String::from(
        "| row | package | oracle | inputs | size unit | largest | certified ms (sum) | oracle ms (sum) | ratio (sum) | ratio at largest | growth certified | growth oracle | peak heap certified / oracle at largest | load oracle / certified |\n| --- | --- | --- | ---: | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |\n",
    );
    for (subject, built) in subjects.iter().zip(&crates) {
        // The oracle, in this process, one input at a time, then at once the
        // package on the same inputs; the load is read before each side.
        let free = memory();
        let oracle_load = load();
        let oracles: Vec<(u128, usize, usize)> = subject
            .samples
            .iter()
            .map(|item| time(heap, &*item.oracle))
            .collect();
        let oracles = &oracles;
        let certified_load = load();
        let binary = dir.join("target/release").join(format!(
            "run_{}{}",
            built.name,
            std::env::consts::EXE_SUFFIX
        ));
        let started = Instant::now();
        let ran = std::process::Command::new(binary.as_std_path())
            .env(rust_harness::HARNESS_STACK_ENV, STACK.to_string())
            .output()
            .expect("the harness runs");
        assert!(
            ran.status.success(),
            "{}: the harness failed:\n{}",
            subject.package,
            String::from_utf8_lossy(&ran.stderr)
        );
        let after_load = load();
        eprintln!(
            "perf: {} ran in {:?} (load {oracle_load} / {certified_load} / {after_load}, {free} GiB available)",
            subject.package,
            started.elapsed()
        );
        let stdout = String::from_utf8(ran.stdout).expect("UTF-8");
        let outcomes: Vec<Json> = stdout
            .lines()
            .filter(|line| !line.starts_with("work "))
            .map(|line| serde_json::from_str(line).expect("an outcome"))
            .collect();
        let stderr = String::from_utf8(ran.stderr).expect("UTF-8");
        let certified: Vec<(u128, usize, usize)> = stderr
            .lines()
            .filter_map(|line| line.strip_prefix("perf "))
            .map(|line| {
                let fields: Vec<&str> = line.split(' ').collect();
                (
                    fields[0].parse().expect("ns"),
                    fields[1].parse().expect("bytes"),
                    fields[2].parse().expect("runs"),
                )
            })
            .collect();
        assert_eq!(
            outcomes.len(),
            subject.samples.len(),
            "{}: one outcome per input",
            subject.package
        );
        assert_eq!(
            certified.len(),
            subject.samples.len(),
            "{}: one timing per input",
            subject.package
        );
        for (index, (printed, item)) in outcomes.iter().zip(&subject.samples).enumerate() {
            assert_eq!(
                printed, &item.expected,
                "{}: input {index} ({} {}): the package and the oracle disagree",
                subject.package, item.size, subject.unit
            );
        }
        let mut rows = Vec::new();
        for ((item, cert), orac) in subject.samples.iter().zip(&certified).zip(oracles) {
            rows.push(json!({
                "size": item.size,
                "certified_ns": cert.0 as u64,
                "certified_peak_bytes": cert.1,
                "certified_runs": cert.2,
                "oracle_ns": orac.0 as u64,
                "oracle_peak_bytes": orac.1,
                "oracle_runs": orac.2,
            }));
        }
        let sum_c: u128 = certified.iter().map(|c| c.0).sum();
        let sum_o: u128 = oracles.iter().map(|o| o.0).sum();
        let largest = subject
            .samples
            .iter()
            .enumerate()
            .max_by_key(|(index, item)| (item.size, certified[*index].0))
            .map(|(index, _)| index)
            .expect("an input");
        // What the largest input returned: a run the oracle refuses early
        // times the refusal, and this is how that is seen.
        let head: String = subject.samples[largest]
            .expected
            .to_string()
            .chars()
            .take(120)
            .collect();
        eprintln!("perf: {} returns at its largest: {head}", subject.package);
        let points_c: Vec<(u64, u128)> = subject
            .samples
            .iter()
            .zip(&certified)
            .map(|(item, c)| (item.size, c.0))
            .collect();
        let points_o: Vec<(u64, u128)> = subject
            .samples
            .iter()
            .zip(oracles)
            .map(|(item, o)| (item.size, o.0))
            .collect();
        let growth_c = exponent(&upper(&points_c));
        let growth_o = exponent(&upper(&points_o));
        let ratio = |c: u128, o: u128| c as f64 / (o.max(1)) as f64;
        let show = |growth: Option<f64>| growth.map_or("-".to_owned(), |g| format!("{g:.2}"));
        let _ = writeln!(
            table,
            "| `{}` | `{}` | `{}` | {} | {} | {} | {:.1} | {:.3} | {:.0}x | {:.0}x | {} | {} | {} / {} KiB | {oracle_load} / {certified_load} ({free} GiB) |",
            subject.row,
            subject.package,
            subject.oracle,
            subject.samples.len(),
            subject.unit,
            subject.samples[largest].size,
            sum_c as f64 / 1e6,
            sum_o as f64 / 1e6,
            ratio(sum_c, sum_o),
            ratio(certified[largest].0, oracles[largest].0),
            show(growth_c),
            show(growth_o),
            certified[largest].1 / 1024,
            oracles[largest].1 / 1024,
        );
        record.push(json!({
            "row": subject.row,
            "project": subject.project,
            "package": subject.package,
            "oracle": subject.oracle,
            "unit": subject.unit,
            "growth_certified": growth_c,
            "growth_oracle": growth_o,
            "load_oracle": oracle_load,
            "load_certified": certified_load,
            "load_after": after_load,
            "available_gib": free,
            "samples": rows,
        }));
    }
    std::fs::write(
        out.join("results.json"),
        serde_json::to_string_pretty(&json!({
            "host": {
                "os": std::env::consts::OS,
                "arch": std::env::consts::ARCH,
                "threads": std::thread::available_parallelism().map(std::num::NonZero::get).unwrap_or(1),
            },
            "subjects": record,
        }))
        .expect("JSON"),
    )
    .expect("the record");
    std::fs::write(out.join("table.md"), &table).expect("the table");
    table
}
