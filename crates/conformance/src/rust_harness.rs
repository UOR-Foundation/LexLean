//! The harnesses that run rendered Rust packages (SPEC.md §17.16): binary
//! crates that call an exported function on argument lists and print each
//! observable outcome in its exact JSON form, then the work the runtime
//! counted.
//!
//! A harness is test text, not a rendering: it lives here, outside the
//! shipped crate, and reaches the renderer only through
//! [`rust::Literals`], which builds each argument literal through the closed
//! AST. The library and function names it is given are checked to be plain
//! Rust identifiers before they are written.

use std::collections::BTreeSet;
use std::fmt::Write as _;

use lexlean::calculus::rust::package::Passing;
use lexlean::calculus::rust::{self, Literals, Profile};
use lexlean::calculus::{Program, Ty, Value};

fn index(number: u64) -> Result<usize, String> {
    usize::try_from(number).map_err(|error| error.to_string())
}

/// Whether `name` is a plain lowercase Rust identifier a harness may write:
/// not one of the harness's own names (`harness_*`, `show_adt*`, `quote`).
fn identifier(name: &str) -> Result<(), String> {
    let mut characters = name.chars();
    if !name.starts_with("harness_")
        && !name.starts_with("show_adt")
        && name != "quote"
        && characters
            .next()
            .is_some_and(|first| first.is_ascii_lowercase())
        && characters.all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
        })
    {
        Ok(())
    } else {
        Err(format!("`{name}` is not a plain Rust identifier"))
    }
}

/// The ADTs whose values a value of `ty` can hold, and whether it can hold
/// a string.
fn shown(program: &Program, ty: &Ty, adts: &mut BTreeSet<u64>) -> bool {
    match ty {
        Ty::String => true,
        Ty::Option { value } => shown(program, value, adts),
        Ty::List { element } => shown(program, element, adts),
        Ty::Result { ok, error } => shown(program, ok, adts) | shown(program, error, adts),
        Ty::Pair { left, right } => shown(program, left, adts) | shown(program, right, adts),
        Ty::Adt { index } => {
            if !adts.insert(*index) {
                return false;
            }
            usize::try_from(*index)
                .ok()
                .and_then(|adt| program.adts.get(adt))
                .map(|adt| {
                    adt.constructors
                        .iter()
                        .flatten()
                        .cloned()
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default()
                .iter()
                .fold(false, |found, field| shown(program, field, adts) | found)
        }
        Ty::Unit
        | Ty::Bool
        | Ty::Nat
        | Ty::Int
        | Ty::Fixed { .. }
        | Ty::Bytes
        | Ty::Ordering
        | Ty::Fn { .. } => false,
    }
}

/// The expression printing a value of type `ty` held in `name` in its exact
/// JSON form.
fn show(ty: &Ty, name: &str) -> Result<String, String> {
    let number = |kind: &str| {
        format!(
            "format!(\"{{{{\\\"kind\\\":\\\"{kind}\\\",\\\"value\\\":\\\"{{}}\\\"}}}}\", {name})"
        )
    };
    Ok(match ty {
        Ty::Unit => format!("{{ let () = {name}; \"{{\\\"kind\\\":\\\"unit\\\"}}\".to_owned() }}"),
        Ty::Bool => {
            format!("format!(\"{{{{\\\"kind\\\":\\\"bool\\\",\\\"value\\\":{{}}}}}}\", {name})")
        }
        Ty::Nat => number("nat"),
        Ty::Int => number("int"),
        Ty::Fixed { width } => number(width.name()),
        Ty::String => format!(
            "format!(\"{{{{\\\"kind\\\":\\\"string\\\",\\\"value\\\":{{}}}}}}\", quote({name}.text()))"
        ),
        Ty::Bytes => format!(
            "format!(\"{{{{\\\"kind\\\":\\\"bytes\\\",\\\"hex\\\":\\\"{{}}\\\"}}}}\", {name}.octets().iter().map(|b| format!(\"{{b:02x}}\")).collect::<String>())"
        ),
        Ty::Ordering => format!(
            "format!(\"{{{{\\\"kind\\\":\\\"ordering\\\",\\\"value\\\":\\\"{{}}\\\"}}}}\", match {name} {{ core::cmp::Ordering::Less => \"lt\", core::cmp::Ordering::Equal => \"eq\", core::cmp::Ordering::Greater => \"gt\" }})"
        ),
        Ty::Option { value } => {
            let inner = show(value, "x")?;
            format!("match {name} {{ None => \"{{\\\"kind\\\":\\\"none\\\"}}\".to_owned(), Some(x) => format!(\"{{{{\\\"kind\\\":\\\"some\\\",\\\"value\\\":{{}}}}}}\", {inner}) }}")
        }
        Ty::Result { ok, error } => {
            let ok_show = show(ok, "x")?;
            let error_show = show(error, "x")?;
            format!("match {name} {{ Ok(x) => format!(\"{{{{\\\"kind\\\":\\\"ok\\\",\\\"value\\\":{{}}}}}}\", {ok_show}), Err(x) => format!(\"{{{{\\\"kind\\\":\\\"error\\\",\\\"value\\\":{{}}}}}}\", {error_show}) }}")
        }
        Ty::List { element } => {
            let inner = show(element, "x")?;
            format!("{{ let mut parts = Vec::new(); let mut cursor = {name}; while let Some((x, rest)) = cursor.uncons() {{ parts.push({inner}); cursor = rest; }} format!(\"{{{{\\\"kind\\\":\\\"list\\\",\\\"items\\\":[{{}}]}}}}\", parts.join(\",\")) }}")
        }
        Ty::Pair { left, right } => {
            let left_show = show(left, "l")?;
            let right_show = show(right, "r")?;
            format!("{{ let (l, r) = {name}; format!(\"{{{{\\\"kind\\\":\\\"pair\\\",\\\"left\\\":{{}},\\\"right\\\":{{}}}}}}\", {left_show}, {right_show}) }}")
        }
        Ty::Adt { index } => format!("show_adt{index}({name})"),
        Ty::Fn { .. } => return Err("a function value is not observable".to_owned()),
    })
}

/// Prints a string in the exact JSON form.
const QUOTE: &str = r#"
fn quote(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
"#;

/// A binary crate that runs `entry` of the rendered library crate `program`
/// on `arguments` and prints the observable outcome, the value in its exact
/// JSON form or `{"kind":"overflow"}`, then the work the runtime counted.
///
/// # Errors
///
/// Returns the reason the program, the arguments, or the result type cannot
/// be rendered in `profile`.
pub fn render_harness(
    program: &Program,
    profile: Profile,
    entry: u64,
    arguments: &[Value],
) -> Result<String, String> {
    let owned = vec![Passing::Own; arguments.len()];
    render_caller(
        program,
        profile,
        entry,
        arguments,
        &Caller {
            library: "program",
            function: &format!("f{entry}"),
            passing: &owned,
        },
    )
}

/// How a harness reaches the function it runs: the library crate it uses,
/// the function's name there, and how each argument is passed.
pub struct Caller<'a> {
    /// The library crate.
    pub library: &'a str,
    /// The function's name in it.
    pub function: &'a str,
    /// How each argument is passed.
    pub passing: &'a [Passing],
}

/// A binary crate that runs `entry` of `program` through `caller` on
/// `arguments` and prints the observable outcome, then the work the runtime
/// counted, as [`render_harness`].
///
/// # Errors
///
/// As [`render_harness`].
pub fn render_caller(
    program: &Program,
    profile: Profile,
    entry: u64,
    arguments: &[Value],
    caller: &Caller<'_>,
) -> Result<String, String> {
    render_calls(
        program,
        profile,
        caller.library,
        &[Calls {
            entry,
            function: caller.function,
            passing: caller.passing,
            arguments: &[arguments.to_vec()],
        }],
    )
}

/// A Rust tuple of `items`: `()` when there are none.
fn tuple(items: &[String]) -> String {
    if items.is_empty() {
        "()".to_owned()
    } else {
        format!("({},)", items.join(", "))
    }
}

/// One function a harness runs, and every argument list it runs it on.
pub struct Calls<'a> {
    /// The program function it runs.
    pub entry: u64,
    /// Its name in the library crate.
    pub function: &'a str,
    /// How each argument is passed.
    pub passing: &'a [Passing],
    /// Every argument list it runs on.
    pub arguments: &'a [Vec<Value>],
}

/// A binary crate that runs each of `calls` through the library crate
/// `library` on each of its argument lists, in order, and prints one
/// observable outcome per run, as [`render_harness`], then the work the
/// runtime counted.
///
/// # Errors
///
/// As [`render_harness`], for any call.
pub fn render_calls(
    program: &Program,
    profile: Profile,
    library: &str,
    calls: &[Calls<'_>],
) -> Result<String, String> {
    identifier(library)?;
    let mut literals = Literals::new(program, profile)?;
    let fallible = rust::fallible_functions(program)?;
    // An ordering argument is written with the core type, which the
    // library uses but does not export.
    let mut out = format!(
        "#![forbid(unsafe_code)]\n#[allow(unused_imports)]\nuse core::cmp::Ordering;\nuse {library}::*;\n"
    );
    let mut adts = BTreeSet::new();
    let mut quoted = false;
    for call in calls {
        identifier(call.function)?;
        let function = program
            .functions
            .get(index(call.entry)?)
            .ok_or_else(|| format!("function {} is not declared", call.entry))?;
        quoted |= shown(program, &function.result, &mut adts);
    }
    if quoted {
        out.push_str(QUOTE);
    }
    for adt in adts {
        let declared = program
            .adts
            .get(index(adt)?)
            .ok_or_else(|| format!("ADT {adt} is not declared"))?;
        let _ = write!(
            out,
            "\nfn show_adt{adt}(value: Adt{adt}) -> String {{ match value {{ "
        );
        for (constructor, fields) in declared.constructors.iter().enumerate() {
            let mut patterns = Vec::new();
            let mut shows = Vec::new();
            for (position, ty) in fields.iter().enumerate() {
                let held = format!("x{position}");
                let read = if literals.boxed(adt, ty)? {
                    format!("(*{held}).clone()")
                } else {
                    held.clone()
                };
                patterns.push(held);
                shows.push(show(ty, &read)?);
            }
            let tagged = |fields: &str| {
                format!("format!(\"{{{{\\\"kind\\\":\\\"adt\\\",\\\"constructor\\\":{constructor},\\\"fields\\\":[{{}}]}}}}\", {fields})")
            };
            if fields.is_empty() {
                let _ = write!(out, "Adt{adt}::C{constructor} => {}, ", tagged("\"\""));
            } else {
                let _ = write!(
                    out,
                    "Adt{adt}::C{constructor}({}) => {}, ",
                    patterns.join(", "),
                    tagged(&format!("[{}].join(\",\")", shows.join(", ")))
                );
            }
        }
        out.push_str("} }\n");
    }
    // Each function's runs are their own Rust function, so no frame holds
    // the argument lists of more than one, and they run on a thread with a
    // fixed 64 MiB stack: an unoptimized frame holding one long list literal
    // still overflows a 1 MiB main-thread stack (the Windows default), and
    // the platform's default must not decide whether a harness runs.
    let mut groups = Vec::new();
    for (group, call) in calls.iter().enumerate() {
        let function = program
            .functions
            .get(index(call.entry)?)
            .ok_or_else(|| format!("function {} is not declared", call.entry))?;
        let mut rows = Vec::new();
        for arguments in call.arguments {
            lexlean::calculus::check::check_arguments(program, call.entry, arguments)?;
            let rendered = arguments
                .iter()
                .zip(&function.types)
                .map(|(argument, ty)| literals.literal(argument, ty))
                .collect::<Result<Vec<_>, _>>()?;
            rows.push(tuple(&rendered));
        }
        let names: Vec<String> = (0..function.types.len())
            .map(|position| format!("a{position}"))
            .collect();
        let passed: Vec<String> = names
            .iter()
            .zip(call.passing.iter().chain(std::iter::repeat(&Passing::Own)))
            .map(|(name, passing)| {
                if *passing == Passing::Borrow {
                    format!("&{name}")
                } else {
                    name.clone()
                }
            })
            .collect();
        let result_show = show(&function.result, "value")?;
        // An entry that cannot overflow returns its value directly.
        let run = if fallible.get(index(call.entry)?).copied().unwrap_or(false) {
            format!(
                "        match {}({}) {{\n            Ok(value) => println!(\"{{}}\", {result_show}),\n            Err(Overflow) => println!(\"{{{{\\\"kind\\\":\\\"overflow\\\"}}}}\"),\n        }}\n",
                call.function,
                passed.join(", ")
            )
        } else {
            format!(
                "        let value = {}({});\n        println!(\"{{}}\", {result_show});\n",
                call.function,
                passed.join(", ")
            )
        };
        let _ = write!(
            out,
            "\n#[inline(never)]\nfn harness_group{group}() {{\n    for {} in [{}] {{\n{run}    }}\n}}\n",
            tuple(&names),
            rows.join(", ")
        );
        groups.push(format!("    harness_group{group}();\n"));
    }
    out.push_str("\nfn harness_runs() {\n");
    out.push_str(&groups.concat());
    out.push_str("    println!(\"work {}\", work());\n}\n");
    out.push_str("\nfn main() {\n    let runner = std::thread::Builder::new().stack_size(64 << 20).spawn(harness_runs).expect(\"the harness thread starts\");\n    if runner.join().is_err() {\n        std::process::exit(101);\n    }\n}\n");
    Ok(out)
}
