//! Certificates A and B end to end (SPEC.md §17.17): lower every production
//! root of a project, generate its certificate A and, for each target, the
//! certificate B of its crate, stage the shipped preservation environment
//! beside the project's generated modules, compile every module with the
//! pinned Lean, replay every certificate through `leanchecker`, and audit the
//! exact axioms of the library and of every root theorem.

use std::path::{Path, PathBuf};
use std::process::Command;

use lexlean::calculus::rust::{lower, Profile};
use lexlean::calculus::{Expr, Program, Ty, Value};
use lexlean::production::certificate::{certificate, Certificate};
use lexlean::production::lower::{linked_modules, lower_root, roots};
use lexlean::production::preserve::{audit, workspace, Workspace};
use lexlean::production::rust_cert::{certificate_b, module_for};

use crate::support::{self, mangled_toolchain_name, real_elan_home, P};

/// The bound of a certificate B whose size is not what its test is about.
const UNBOUNDED: u64 = u64::MAX;

/// One certified root.
#[derive(Debug, Clone)]
pub struct Certified {
    /// The root's qualified Lean name.
    pub root: String,
    /// The targets the root is eligible for; one certificate serves both,
    /// because the realization program is target-independent.
    pub targets: Vec<String>,
    /// The certificate.
    pub certificate: Certificate,
    /// The lowered program the certificate is about.
    pub program: lexlean::calculus::Program,
    /// The function a caller invokes: the root's entry when a parameter
    /// carries an invariant, else the root, function 0.
    pub entry: u64,
    /// Certificate B of the program's crate in each target.
    pub renderings: Vec<(String, Certificate)>,
    /// Certificate E, A and B composed, in each target.
    pub composed: Vec<(String, Certificate)>,
}

impl Certified {
    /// Every certificate of the root: A, then B and E in each target.
    #[must_use]
    pub fn all(&self) -> Vec<Certificate> {
        std::iter::once(self.certificate.clone())
            .chain(self.renderings.iter().map(|(_, b)| b.clone()))
            .chain(self.composed.iter().map(|(_, e)| e.clone()))
            .collect()
    }
}

/// The certificates of every production root of `project`, in root order.
///
/// # Panics
///
/// Panics when a root fails to lower or to certify: an eligible root
/// always lowers, and its certificate always generates.
#[must_use]
pub fn certificates(project: &P) -> Vec<Certified> {
    let checked = support::checked_project(project);
    let modules = linked_modules(&checked);
    let hex32: String = checked.semantic_id.to_hex()[..32].to_owned();
    let limits = support::limits(project);
    roots(&checked)
        .expect("the eligibility reports")
        .iter()
        .enumerate()
        .map(|(index, root)| {
            let lowered = lower_root(&modules, &root.module, &root.name, root.report, &limits)
                .unwrap_or_else(|diagnostic| {
                    panic!("{}: lowering failed: {diagnostic:?}", root.report.root)
                });
            let module = format!("LexLeanPreserve.C{hex32}.R{index}");
            let certificate = certificate(
                &modules,
                &root.module,
                &root.name,
                root.report,
                &lowered,
                &module,
                limits.max_file_bytes,
            )
            .unwrap_or_else(|diagnostic| {
                panic!(
                    "{}: certificate generation failed: {diagnostic:?}",
                    root.report.root
                )
            });
            let renderings = root
                .report
                .targets
                .iter()
                .map(|row| {
                    let profile = Profile::named(&row.target).expect("a Rust profile");
                    let krate = lower(&lowered.program, profile).unwrap_or_else(|reason| {
                        panic!(
                            "{} has no {} rendering: {reason}",
                            root.report.root, row.target
                        )
                    });
                    let module_b = module_for(&module, &row.target).expect("a Rust target");
                    let certificate_b =
                        certificate_b(&lowered.program, &krate, &module_b, limits.max_file_bytes)
                            .unwrap_or_else(|reason| {
                                panic!(
                                    "{} on {}: certificate B: {reason}",
                                    root.report.root, row.target
                                )
                            });
                    (row.target.clone(), certificate_b.into_certificate())
                })
                .collect();
            let fallible = lexlean::calculus::rust::fallible_functions(&lowered.program)
                .expect("a valid program")[lowered.entry() as usize];
            let composed = root
                .report
                .targets
                .iter()
                .map(|row| {
                    let module_b = module_for(&module, &row.target).expect("a Rust target");
                    let module_e =
                        module_for(&format!("{module}.Compose"), &row.target).expect("a target");
                    let certificate_e = lexlean::production::certificate::certificate_e(
                        &modules,
                        &root.module,
                        &root.name,
                        &lowered,
                        &module,
                        &module_b,
                        &module_e,
                        fallible,
                        limits.max_file_bytes,
                    )
                    .unwrap_or_else(|diagnostic| {
                        panic!(
                            "{} on {}: certificate E: {diagnostic:?}",
                            root.report.root, row.target
                        )
                    });
                    (row.target.clone(), certificate_e)
                })
                .collect();
            Certified {
                root: root.report.root.clone(),
                targets: root
                    .report
                    .targets
                    .iter()
                    .map(|row| row.target.clone())
                    .collect(),
                certificate,
                entry: lowered.entry(),
                program: lowered.program,
                renderings,
                composed,
            }
        })
        .collect()
}

/// Regenerate certificates A and E of every root of `project` under half
/// their size, and report, for each root, the codes the generator refused A
/// with and E with (in each target): `LLS8002` when generation stops at the
/// limit it was given, which is what bounds its work for any program.
///
/// # Panics
///
/// Panics when a root fails to lower.
#[must_use]
pub fn halved(project: &P, certified: &[Certified]) -> Vec<(String, String, Vec<String>)> {
    let checked = support::checked_project(project);
    let modules = linked_modules(&checked);
    let hex32: String = checked.semantic_id.to_hex()[..32].to_owned();
    let limits = support::limits(project);
    let code = |result: Result<Certificate, lexlean::diagnostic::Diagnostic>| match result {
        Ok(_) => "accepted".to_owned(),
        Err(diagnostic) => diagnostic.code.as_str().to_owned(),
    };
    roots(&checked)
        .expect("the eligibility reports")
        .iter()
        .enumerate()
        .map(|(index, root)| {
            let lowered = lower_root(&modules, &root.module, &root.name, root.report, &limits)
                .unwrap_or_else(|diagnostic| {
                    panic!("{}: lowering failed: {diagnostic:?}", root.report.root)
                });
            let module = format!("LexLeanPreserve.C{hex32}.R{index}");
            let held = &certified[index];
            let a = code(certificate(
                &modules,
                &root.module,
                &root.name,
                root.report,
                &lowered,
                &module,
                held.certificate.text.len() as u64 / 2,
            ));
            let fallible = lexlean::calculus::rust::fallible_functions(&lowered.program)
                .expect("a valid program")[lowered.entry() as usize];
            let e = held
                .composed
                .iter()
                .map(|(target, composed)| {
                    let module_b = module_for(&module, target).expect("a Rust target");
                    let module_e =
                        module_for(&format!("{module}.Compose"), target).expect("a target");
                    code(lexlean::production::certificate::certificate_e(
                        &modules,
                        &root.module,
                        &root.name,
                        &lowered,
                        &module,
                        &module_b,
                        &module_e,
                        fallible,
                        composed.text.len() as u64 / 2,
                    ))
                })
                .collect();
            (root.report.root.clone(), a, e)
        })
        .collect()
}

/// The size of certificate B of every root of `project` in each of its
/// targets, derived under `limit`, without certificates A and E: the
/// lowering, the rendering, and the aligner, which are what a match with many
/// arms strains.
///
/// # Errors
///
/// Returns the aligner's refusal.
///
/// # Panics
///
/// Panics when a root fails to lower or to render.
pub fn aligned(project: &P, limit: u64) -> Result<Vec<usize>, String> {
    let checked = support::checked_project(project);
    let modules = linked_modules(&checked);
    let limits = support::limits(project);
    let mut sizes = Vec::new();
    for (index, root) in roots(&checked)
        .expect("the eligibility reports")
        .iter()
        .enumerate()
    {
        let lowered = lower_root(&modules, &root.module, &root.name, root.report, &limits)
            .unwrap_or_else(|diagnostic| panic!("lowering failed: {diagnostic:?}"));
        for row in &root.report.targets {
            let profile = Profile::named(&row.target).expect("a Rust profile");
            let krate = lower(&lowered.program, profile).expect("a rendering");
            let module = module_for(&format!("LexLeanPreserve.Aligned.R{index}"), &row.target)
                .expect("a Rust target");
            sizes.push(
                certificate_b(&lowered.program, &krate, &module, limit)?
                    .text
                    .len(),
            );
        }
    }
    Ok(sizes)
}

/// The staged workspace of `project`'s certificates.
#[must_use]
pub fn staged(project: &P, certified: &[Certified]) -> Workspace {
    let rendered = support::rendered(project);
    let user: Vec<(String, String)> = rendered
        .modules
        .iter()
        .map(|module| (module.lean_module.clone(), module.lean_text.clone()))
        .collect();
    let certificates: Vec<Certificate> = certified.iter().flat_map(Certified::all).collect();
    workspace(&user, &certificates).expect("the workspace stages")
}

fn toolchain_bin() -> PathBuf {
    real_elan_home()
        .join("toolchains")
        .join(mangled_toolchain_name())
        .join("bin")
}

/// The outcome of running a staged workspace through Lean.
#[derive(Debug, Clone)]
pub struct Checked {
    /// The module that failed to compile or replay, with Lean's output.
    pub failure: Option<(String, String)>,
    /// The audit module's output when every module compiled and replayed.
    pub audit_output: String,
}

/// A pinned-toolchain command: the toolchain's `bin` leads `PATH`, so a
/// tool that spawns `lean` (as `leanchecker` does) finds the pinned binary
/// rather than an elan proxy, whatever `ELAN_HOME` a concurrent test holds.
fn pinned(program: &str, root: &Path) -> Command {
    let bin = toolchain_bin();
    let path = std::env::join_paths(std::iter::once(bin.clone()).chain(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    )))
    .expect("a search path");
    let mut command = Command::new(bin.join(program));
    command
        .current_dir(root)
        .env("PATH", path)
        .env("LEAN_PATH", root.join("build"));
    command
}

fn lean(root: &Path, arguments: &[String]) -> std::process::Output {
    pinned("lean", root)
        .args(arguments)
        .output()
        .expect("lean runs")
}

fn joined(output: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

/// Run `job` over `items` on a few threads, returning the results in
/// `items` order. Certificates within one wave import the environment and
/// nothing of one another, so they compile and replay independently; the pool is small
/// because each Lean process holds the whole environment in memory.
fn parallel<T: Sync, R: Send>(items: &[T], job: impl Fn(&T) -> R + Sync) -> Vec<R> {
    let workers = std::thread::available_parallelism()
        .map_or(1, std::num::NonZeroUsize::get)
        .clamp(1, 3);
    let next = std::sync::atomic::AtomicUsize::new(0);
    let results: std::sync::Mutex<Vec<Option<R>>> =
        std::sync::Mutex::new(items.iter().map(|_| None).collect());
    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| loop {
                let index = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                let Some(item) = items.get(index) else {
                    break;
                };
                let result = job(item);
                results.lock().expect("the results")[index] = Some(result);
            });
        }
    });
    results
        .into_inner()
        .expect("the results")
        .into_iter()
        .map(|result| result.expect("every job ran"))
        .collect()
}

/// Write `workspace` below `root`, compile the environment in order and
/// every certificate after it, replay every certificate, and compile the
/// audit module.
#[must_use]
pub fn check(workspace: &Workspace, root: &Path) -> Checked {
    for file in workspace.files.iter().chain([&workspace.audit]) {
        let path = root.join("src").join(&file.path);
        std::fs::create_dir_all(path.parent().expect("a parent")).expect("mkdir");
        std::fs::write(&path, &file.text).expect("write");
    }
    let compile = |file: &lexlean::production::preserve::StagedFile| {
        let source = root.join("src").join(&file.path);
        let olean = root
            .join("build")
            .join(file.path.trim_end_matches(".lean"))
            .with_extension("olean");
        std::fs::create_dir_all(olean.parent().expect("a parent")).expect("mkdir");
        let output = lean(
            root,
            &[
                "-o".to_owned(),
                olean.display().to_string(),
                source.display().to_string(),
            ],
        );
        (!output.status.success()).then(|| (file.module.clone(), joined(&output)))
    };
    let (certificates, environment): (Vec<_>, Vec<_>) = workspace
        .files
        .iter()
        .partition(|file| workspace.certificates.contains(&file.module));
    for file in environment {
        if let Some(failure) = compile(file) {
            return Checked {
                failure: Some(failure),
                audit_output: String::new(),
            };
        }
    }
    // Certificate E imports the certificates A and B it composes, so the
    // certificates compile in waves: each wave is every certificate whose
    // imported certificates an earlier wave compiled.
    let imports = |file: &lexlean::production::preserve::StagedFile| -> Vec<String> {
        file.text
            .lines()
            .filter_map(|line| line.strip_prefix("import "))
            .map(|module| module.trim().to_owned())
            .filter(|module| workspace.certificates.contains(module))
            .collect()
    };
    let mut compiled: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    let mut pending = certificates;
    while !pending.is_empty() {
        let (wave, rest): (Vec<_>, Vec<_>) = pending
            .into_iter()
            .partition(|file| imports(file).iter().all(|module| compiled.contains(module)));
        assert!(
            !wave.is_empty(),
            "the certificates import one another in a cycle"
        );
        if let Some(failure) = parallel(&wave, |file| compile(file))
            .into_iter()
            .flatten()
            .next()
        {
            return Checked {
                failure: Some(failure),
                audit_output: String::new(),
            };
        }
        compiled.extend(wave.iter().map(|file| file.module.clone()));
        pending = rest;
    }
    let replayed = parallel(&workspace.certificates, |module| {
        let output = pinned("leanchecker", root)
            .arg(module)
            .output()
            .expect("leanchecker runs");
        (!output.status.success())
            .then(|| (module.clone(), format!("leanchecker: {}", joined(&output))))
    });
    if let Some(failure) = replayed.into_iter().flatten().next() {
        return Checked {
            failure: Some(failure),
            audit_output: String::new(),
        };
    }
    let audit_source = root.join("src").join(&workspace.audit.path);
    let output = lean(root, &[audit_source.display().to_string()]);
    let text = joined(&output);
    if !output.status.success() {
        return Checked {
            failure: Some((workspace.audit.module.clone(), text)),
            audit_output: String::new(),
        };
    }
    Checked {
        failure: None,
        audit_output: text,
    }
}

/// What certifying a project established.
#[derive(Debug, Clone)]
pub struct Report {
    /// Every certified root.
    pub certified: Vec<Certified>,
    /// The differential cases whose certificate observation equalled the
    /// interpreter's outcome.
    pub differential: usize,
    /// The audit module's output.
    pub audit_output: String,
    /// The differential module's output.
    pub differential_output: String,
    /// Each certificate's module, observation, and cases.
    pub denotes: Vec<(String, String, Vec<crate::differential::Case>)>,
    /// The crates of every root in every target it is eligible for.
    pub crates: Vec<crate::machine::Rendered>,
    /// The cases on which the declared Rust machine, evaluating each crate,
    /// agreed with the interpreter.
    pub machine: usize,
    /// Every root's first certificate E restated with a planted defect,
    /// and the pinned Lean's verdict on it.
    pub composed_plants: Vec<ComposedPlant>,
    /// How many witnesses of certificates E the pinned Lean checked, and how
    /// many of them apply the theorem at the bounds of `u64` or `i64`.
    pub witnessed: (usize, usize),
}

/// A defect planted in a certificate E's statement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompositionMutation {
    /// The root's Rust function is claimed with the other result shape:
    /// `R<T>` for a plain function, or a plain result for a fallible one.
    Fallibility,
    /// The rendering of another function is claimed to realize the root.
    Function,
    /// The hypothesis that the arguments are representable is made
    /// unusable, `RepresentableL [..] ∧ False`: the theorem is then vacuous,
    /// and no proof notices, because the proof never uses the hypothesis.
    /// What notices is a witness that applies the theorem to arguments.
    Hypothesis,
}

impl CompositionMutation {
    /// Every mutation.
    pub const ALL: [Self; 3] = [Self::Fallibility, Self::Function, Self::Hypothesis];

    /// The declaration Lean's first error must lie in: the composition
    /// itself for a defect in what it states, the witness that applies it
    /// for a statement that proves but cannot be used.
    #[must_use]
    pub fn declaration_prefix(self) -> &'static str {
        match self {
            Self::Fallibility | Self::Function => "root",
            Self::Hypothesis => "witness_",
        }
    }

    /// The places of `text` this defect applies to: each statement of a
    /// result shape, each function the entry is claimed to be.
    fn marks(self, text: &str) -> Vec<usize> {
        let mark = match self {
            Self::Fallibility => "Rust.RealizesFn ",
            Self::Function => "Rust.fnIdent ",
            Self::Hypothesis => return hypothesis_ends(text),
        };
        text.match_indices(mark)
            .map(|(at, found)| at + found.len())
            .collect()
    }

    /// How many places `text` has for this defect.
    #[must_use]
    pub fn places(self, text: &str) -> usize {
        self.marks(text).len()
    }

    /// `text` with this defect planted at its `nth` place, or `None` when it
    /// has none.
    #[must_use]
    pub fn plant(self, text: &str, nth: usize) -> Option<String> {
        let at = *self.marks(text).get(nth)?;
        Some(match self {
            Self::Fallibility => {
                let (word, other) = if text[at..].starts_with("true ") {
                    ("true", "false")
                } else {
                    ("false", "true")
                };
                format!("{}{}{}", &text[..at], other, &text[at + word.len()..])
            }
            Self::Function => {
                let digits: String = text[at..]
                    .chars()
                    .take_while(char::is_ascii_digit)
                    .collect();
                let index: u64 = digits.parse().ok()?;
                format!("{}{}{}", &text[..at], index + 1, &text[at + digits.len()..])
            }
            Self::Hypothesis => format!("{} ∧ False{}", &text[..at], &text[at..]),
        })
    }
}

/// The byte offset of the end of each `RepresentableL [..]` hypothesis of
/// `text`: where a conjunct can be appended to it.
fn hypothesis_ends(text: &str) -> Vec<usize> {
    let mark = "(__e_hrep : LexLeanPreservation.Rust.RepresentableL [";
    text.match_indices(mark)
        .filter_map(|(at, found)| {
            let mut depth = 1_usize;
            for (offset, character) in text[at + found.len()..].char_indices() {
                match character {
                    '[' => depth += 1,
                    ']' => {
                        depth -= 1;
                        if depth == 0 {
                            return Some(at + found.len() + offset + 1);
                        }
                    }
                    _ => {}
                }
            }
            None
        })
        .collect()
}

/// Whether a calculus value is one a Rust caller can pass: a natural number
/// below `2^64` and an integer in the `i64` range, wherever they occur
/// (the library's `Representable`).
fn representable(value: &Value) -> bool {
    match value {
        Value::Nat { value } => value
            .parse::<u128>()
            .is_ok_and(|number| number < 1_u128 << 64),
        Value::Int { value } => value.parse::<i64>().is_ok(),
        Value::Some { value } | Value::Ok { value } | Value::Error { value } => {
            representable(value)
        }
        Value::List { items } => items.iter().all(representable),
        Value::Pair { left, right } => representable(left) && representable(right),
        Value::Adt {
            constructor: _,
            fields,
        } => fields.iter().all(representable),
        Value::Closure {
            function: _,
            captures,
        } => captures.iter().all(representable),
        Value::Unit
        | Value::Bool { value: _ }
        | Value::U8 { value: _ }
        | Value::U16 { value: _ }
        | Value::U32 { value: _ }
        | Value::U64 { value: _ }
        | Value::I8 { value: _ }
        | Value::I16 { value: _ }
        | Value::I32 { value: _ }
        | Value::I64 { value: _ }
        | Value::String { value: _ }
        | Value::Bytes { hex: _ }
        | Value::Ordering { value: _ }
        | Value::None => true,
    }
}

/// The witnesses of a certificate E: definitions that apply its theorem to
/// the arguments of the differential's cases that a Rust caller can pass,
/// the first, one that breaks an invariant, and the ones at the bounds of
/// `u64` and `i64`, proving the theorem's hypothesis by unfolding it. A
/// theorem whose hypothesis cannot be met, or cannot be used, fails to apply.
fn witnesses(
    certificate_a: &Certificate,
    composed: &Certificate,
    cases: &[crate::differential::Case],
) -> Vec<String> {
    let bound = |case: &crate::differential::Case| {
        let text = case.arguments.join(" ");
        [
            "18446744073709551615",
            "9223372036854775807",
            "-9223372036854775808",
        ]
        .iter()
        .any(|literal| text.contains(literal))
    };
    let usable: Vec<usize> = (0..cases.len())
        .filter(|at| cases[*at].values.iter().all(representable))
        .collect();
    let mut chosen: Vec<usize> = Vec::new();
    for pick in [
        usable.first(),
        usable.iter().find(|at| cases[**at].invalid.is_some()),
        usable.iter().find(|at| bound(&cases[**at])),
        usable.iter().rev().find(|at| bound(&cases[**at])),
    ]
    .into_iter()
    .flatten()
    {
        if !chosen.contains(pick) {
            chosen.push(*pick);
        }
    }
    // The names the encoders of the arguments unfold through: the ones
    // certificate A defines and the library's.
    let mut names: Vec<String> = Vec::new();
    for line in certificate_a.text.lines() {
        if let Some(rest) = line.strip_prefix("def ") {
            let name: String = rest
                .chars()
                .take_while(|character| character.is_alphanumeric() || *character == '_')
                .collect();
            if ["__enc_", "__aux_", "__items_", "__L_", "__O_"]
                .iter()
                .any(|prefix| name.starts_with(prefix))
            {
                names.push(format!("{}.{name}", certificate_a.module));
            }
        }
    }
    for (at, found) in composed.text.match_indices("LexLeanPreservation.enc") {
        let name: String = composed.text[at + found.len()..]
            .chars()
            .take_while(|character| character.is_alphanumeric())
            .collect();
        let full = format!("{found}{name}");
        if !names.contains(&full) {
            names.push(full);
        }
    }
    let unfold = names.join(", ");
    chosen
        .iter()
        .enumerate()
        .map(|(position, at)| {
            format!(
                "def witness_{position} := {} {} (by simp [LexLeanPreservation.Rust.RepresentableL, LexLeanPreservation.Rust.Representable, {unfold}])\n",
                composed.theorem,
                cases[*at].arguments.join(" ")
            )
        })
        .collect()
}

/// One planted certificate E and the pinned Lean's verdict.
#[derive(Debug, Clone)]
pub struct ComposedPlant {
    /// The certified root.
    pub root: String,
    /// The target whose certificate E was planted.
    pub target: String,
    /// The defect.
    pub mutation: CompositionMutation,
    /// The place of the defect within the statement.
    pub nth: usize,
    /// The declaration the first error lies in; `root` is the composition.
    pub declaration: Option<String>,
    /// Whether the pinned Lean accepted the planted module.
    pub accepted: bool,
    /// The pinned Lean's output.
    pub output: String,
    /// The size of the planted module's text.
    pub text_len: usize,
}

/// The function each root's crate is invoked through in each target, as the
/// committed record of the verified example `example` states it.
///
/// # Panics
///
/// Panics when the example has no committed record.
#[must_use]
pub fn recorded_entries(example: &str) -> std::collections::BTreeMap<(String, String), u64> {
    let path = support::repo_root()
        .join("examples")
        .join(example)
        .join("expected/verify/preserve/preservation.json");
    let record: serde_json::Value =
        serde_json::from_slice(&std::fs::read(path.as_std_path()).expect("a committed record"))
            .expect("the record is JSON");
    let mut out = std::collections::BTreeMap::new();
    for row in record["roots"].as_array().expect("roots") {
        let root = row["root"].as_str().expect("a root").to_owned();
        for rendering in row["renderings"].as_array().expect("renderings") {
            let target = rendering["target"].as_str().expect("a target").to_owned();
            let function = rendering["entry"]["function"].as_u64().expect("an entry");
            out.insert((root.clone(), target), function);
        }
    }
    out
}

/// Certify every production root of `project` end to end, then run the
/// differential: each certificate's observation, evaluated by Lean on
/// seeded inputs, must equal the calculus interpreter's outcome.
///
/// # Panics
///
/// Panics naming the first module that fails to compile or replay, the
/// first axiom disagreement, or every differential disagreement.
#[must_use]
pub fn certify(project: &P, name: &str) -> Report {
    let certified = certificates(project);
    let staged = staged(project, &certified);
    let scratch = tempfile::Builder::new()
        .prefix("lexlean-preserve-")
        .tempdir()
        .expect("tempdir");
    let checked = check(&staged, scratch.path());
    if let Some((module, output)) = checked.failure {
        panic!("{name}: `{module}` was rejected:\n{output}");
    }
    let certificates: Vec<Certificate> = certified.iter().flat_map(Certified::all).collect();
    audit(&checked.audit_output, &certificates)
        .unwrap_or_else(|reason| panic!("{name}: axiom audit: {reason}"));
    let cases = crate::differential::cases(project);
    let witnessed = apply_witnesses(&certified, &cases, scratch.path(), name);
    let composed_plants = plant_composed(&certified, scratch.path(), &cases);
    // Each root's cases through the root are observed by `denote`, and
    // those through its entry by `denoteEntry` (§17.17).
    let denotes: Vec<(String, String, Vec<crate::differential::Case>)> = certified
        .iter()
        .flat_map(|entry| {
            let (direct, entered): (Vec<_>, Vec<_>) = cases
                .get(&entry.root)
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .partition(|case| case.function == 0);
            let module = entry.certificate.module.clone();
            let mut rows = vec![(module.clone(), entry.certificate.denote.clone(), direct)];
            if !entered.is_empty() {
                rows.push((module.clone(), format!("{module}.denoteEntry"), entered));
            }
            rows
        })
        .collect();
    let path = scratch.path().join("src/LexLeanPreserve/Differential.lean");
    std::fs::write(&path, crate::differential::module(&denotes)).expect("write");
    let output = lean(scratch.path(), &[path.display().to_string()]);
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.status.success(),
        "{name}: the differential module was rejected:\n{text}"
    );
    let differential = crate::differential::compare(&text, &denotes)
        .unwrap_or_else(|failures| panic!("{name}: differential:\n{failures}"));
    let crates = crate::machine::crates(&certified);
    let path = scratch.path().join("src/LexLeanPreserve/Machine.lean");
    std::fs::write(&path, crate::machine::module(&crates, &cases)).expect("write");
    let output = lean(scratch.path(), &[path.display().to_string()]);
    let machine_text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.status.success(),
        "{name}: the machine module was rejected:\n{machine_text}"
    );
    let machine = crate::machine::compare(&machine_text, &crates, &cases)
        .unwrap_or_else(|failures| panic!("{name}: machine:\n{failures}"));
    Report {
        certified,
        differential,
        audit_output: checked.audit_output,
        differential_output: text,
        denotes,
        crates,
        machine,
        composed_plants,
        witnessed,
    }
}

/// Compile the witnesses of every certificate E beside the checked
/// workspace: each applies the end-to-end theorem of a root to arguments a
/// Rust caller can pass, so a hypothesis that cannot be met or used fails
/// here. Returns how many witnesses there are and how many of them are at
/// the bounds of `u64` or `i64`.
fn apply_witnesses(
    certified: &[Certified],
    cases: &std::collections::BTreeMap<String, Vec<crate::differential::Case>>,
    root: &Path,
    name: &str,
) -> (usize, usize) {
    let mut text = String::new();
    let mut modules = std::collections::BTreeSet::new();
    let (mut count, mut boundary) = (0, 0);
    let mut body = String::new();
    for entry in certified {
        for (_, composed) in &entry.composed {
            let found = witnesses(
                &entry.certificate,
                composed,
                cases.get(&entry.root).map_or(&[][..], Vec::as_slice),
            );
            assert!(
                !found.is_empty(),
                "{name}: `{}` has no argument of the differential a Rust caller can pass to witness it",
                composed.module
            );
            modules.insert(composed.module.clone());
            count += found.len();
            boundary += found
                .iter()
                .filter(|line| {
                    [
                        "18446744073709551615",
                        "9223372036854775807",
                        "-9223372036854775808",
                    ]
                    .iter()
                    .any(|literal| line.contains(literal))
                })
                .count();
            // A witness is named by its position within one certificate.
            for (position, line) in found.iter().enumerate() {
                let renamed = line.replacen(
                    &format!("def witness_{position} "),
                    &format!("def witness_{}_{position} ", modules.len()),
                    1,
                );
                body.push_str(&renamed);
            }
        }
    }
    for module in &modules {
        text.push_str(&format!("import {module}\n"));
    }
    text.push_str("set_option linter.unusedSimpArgs false\n");
    text.push_str(&body);
    let path = root.join("src/LexLeanPreserve/Witnesses.lean");
    std::fs::write(&path, text).expect("write");
    let output = lean(root, &[path.display().to_string()]);
    assert!(
        output.status.success(),
        "{name}: a witness of certificate E was rejected:\n{}",
        joined(&output)
    );
    (count, boundary)
}

/// Restate every root's first certificate E with each planted defect and
/// compile it beside the checked workspace below `root`.
fn plant_composed(
    certified: &[Certified],
    root: &Path,
    cases: &std::collections::BTreeMap<String, Vec<crate::differential::Case>>,
) -> Vec<ComposedPlant> {
    let mut jobs = Vec::new();
    for entry in certified {
        let Some((target, composed)) = entry.composed.first() else {
            continue;
        };
        for mutation in CompositionMutation::ALL {
            let places = mutation.places(&composed.text);
            assert!(places > 0, "{mutation:?} applies to `{}`", composed.module);
            for nth in 0..places {
                let mut text = mutation
                    .plant(&composed.text, nth)
                    .expect("the place was counted");
                if mutation == CompositionMutation::Hypothesis {
                    // A vacuous statement proves, so only applying it shows.
                    text.push_str("\nset_option linter.unusedSimpArgs false\n");
                    for witness in witnesses(
                        &entry.certificate,
                        composed,
                        cases.get(&entry.root).map_or(&[][..], Vec::as_slice),
                    ) {
                        text.push_str(&witness);
                    }
                }
                jobs.push((entry.root.clone(), target.clone(), mutation, nth, text));
            }
        }
    }
    let directory = root.join("src/LexLeanPreserve/Planted");
    std::fs::create_dir_all(&directory).expect("mkdir");
    parallel(&jobs, |(root_name, target, mutation, nth, text)| {
        let index = jobs
            .iter()
            .position(|job| {
                job.0 == *root_name && job.1 == *target && job.2 == *mutation && job.3 == *nth
            })
            .expect("the job");
        let path = directory.join(format!("E{index}.lean"));
        std::fs::write(&path, text).expect("write");
        let output = lean(root, &[path.display().to_string()]);
        let output_text = joined(&output);
        ComposedPlant {
            root: root_name.clone(),
            target: target.clone(),
            mutation: *mutation,
            nth: *nth,
            declaration: failing_declaration(text, &output_text),
            accepted: output.status.success(),
            output: output_text,
            text_len: text.len(),
        }
    })
}

/// Certify every production root of the example `name` end to end.
#[must_use]
pub fn certify_example(name: &str) -> Report {
    certify(&P::copy_example(name), name)
}

/// A defect planted in a lowered program after lowering, while the
/// certificate's proof is still derived from the source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mutation {
    /// The first conditional's branches are swapped.
    Branches,
    /// The first natural addition subtracts.
    Arithmetic,
    /// The first constructed document value takes the next constructor,
    /// or the first boolean is negated.
    Constructor,
    /// The first call calls a neighbouring function.
    Recursion,
    /// The first natural literal is one larger.
    Literal,
    /// The first checked fixed-width operation is bounded as another
    /// operation: an addition or a multiplication as a subtraction, a
    /// subtraction as an addition.
    Overflow,
    /// The first slice takes its bounds in the other order.
    Buffer,
    /// The entry calls the root without one validated parameter's check
    /// (§17.17): an invariant is no longer checked at the boundary.
    Validation,
    /// A validator omits the check of one component of its type: the
    /// elements of a list, the values of a map, a field of a document.
    Clause,
    /// An order-checking validator admits an equal successor: a map with a
    /// repeated key, or a set with a repeated element, passes.
    Weakening,
}

impl Mutation {
    /// The mutations of the lowered closure.
    pub const LOWERING: [Self; 7] = [
        Self::Branches,
        Self::Arithmetic,
        Self::Overflow,
        Self::Constructor,
        Self::Buffer,
        Self::Recursion,
        Self::Literal,
    ];

    /// The mutations of the boundary.
    pub const BOUNDARY: [Self; 3] = [Self::Validation, Self::Clause, Self::Weakening];
}

/// Whether `expr` is a validator's key comparison: `less_than` as the
/// templates write it, `true` exactly when the order says `lt`.
fn key_comparison(arms: &[lexlean::calculus::Arm]) -> bool {
    use lexlean::calculus::Shape;
    let constant = |expr: &Expr, wanted: Shape| matches!(expr, Expr::Build { shape, operands, .. } if *shape == wanted && operands.is_empty());
    arms.len() == 3
        && arms[0].shape == Shape::Lt
        && constant(&arms[0].body, Shape::True)
        && arms[1].shape == Shape::Eq
        && constant(&arms[1].body, Shape::False)
}

/// Plant `mutation` at `expr` itself, when it applies there.
fn apply(expr: &mut Expr, mutation: Mutation, program: &Program) -> bool {
    use lexlean::calculus::{Prim, Shape};
    match (mutation, &mut *expr) {
        (
            Mutation::Validation,
            Expr::Cond {
                condition,
                then_branch,
                else_branch,
            },
        ) if matches!(**condition, Expr::Call { .. })
            && matches!(
                **else_branch,
                Expr::Build {
                    shape: Shape::None,
                    ..
                }
            ) =>
        {
            let kept = (**then_branch).clone();
            *expr = kept;
            return true;
        }
        (
            Mutation::Weakening,
            Expr::Match {
                ty: _,
                scrutinee,
                arms,
            },
        ) if matches!(
            **scrutinee,
            Expr::Prim {
                operation: Prim::Compare,
                ..
            }
        ) && key_comparison(arms) =>
        {
            if let Expr::Build { shape, .. } = &mut arms[1].body {
                *shape = Shape::True;
            }
            return true;
        }
        (
            Mutation::Clause,
            Expr::Cond {
                condition,
                then_branch,
                else_branch,
            },
        ) if matches!(**condition, Expr::Call { .. })
            && matches!(
                **else_branch,
                Expr::Build {
                    shape: Shape::False,
                    ..
                }
            ) =>
        {
            let kept = (**then_branch).clone();
            *expr = kept;
            return true;
        }
        _ => {}
    }
    match (mutation, &mut *expr) {
        (
            Mutation::Overflow,
            Expr::Prim {
                operation,
                operands: _,
            },
        ) => match operation {
            Prim::CheckedAdd | Prim::CheckedMul => {
                *operation = Prim::CheckedSub;
                true
            }
            Prim::CheckedSub => {
                *operation = Prim::CheckedAdd;
                true
            }
            _ => false,
        },
        (
            Mutation::Buffer,
            Expr::Prim {
                operation: Prim::Slice,
                operands,
            },
        ) if operands.len() == 3 => {
            operands.swap(1, 2);
            true
        }
        (
            Mutation::Branches,
            Expr::Cond {
                condition: _,
                then_branch,
                else_branch,
            },
        ) => {
            std::mem::swap(then_branch, else_branch);
            true
        }
        (
            Mutation::Arithmetic,
            Expr::Prim {
                operation,
                operands: _,
            },
        ) if *operation == Prim::NatAdd => {
            *operation = Prim::NatSub;
            true
        }
        (
            Mutation::Constructor,
            Expr::Build {
                shape,
                ty,
                operands: _,
            },
        ) => match (shape, ty) {
            (Shape::Adt { constructor }, Ty::Adt { index }) => {
                let count = program.adts[usize::try_from(*index).expect("an index")]
                    .constructors
                    .len() as u64;
                if count > 1 {
                    *constructor = (*constructor + 1) % count;
                    true
                } else {
                    false
                }
            }
            (shape @ Shape::True, _) => {
                *shape = Shape::False;
                true
            }
            (shape @ Shape::False, _) => {
                *shape = Shape::True;
                true
            }
            _ => false,
        },
        (
            Mutation::Recursion,
            Expr::Call {
                function,
                operands: _,
            },
        ) => {
            *function = if *function + 1 < program.functions.len() as u64 {
                *function + 1
            } else {
                function.saturating_sub(1)
            };
            true
        }
        (
            Mutation::Literal,
            Expr::Value {
                ty: _,
                value: Value::Nat { value },
            },
        ) => {
            let number: u128 = value.parse().expect("a natural literal");
            *value = (number + 1).to_string();
            true
        }
        _ => false,
    }
}

/// Where a mutation is planted: the `skip`-th place it applies in a
/// function, counted in evaluation order, and how many places it applied.
struct Site {
    skip: usize,
    seen: usize,
}

fn mutate_expr(expr: &mut Expr, mutation: Mutation, program: &Program, site: &mut Site) -> bool {
    let saved = expr.clone();
    if apply(expr, mutation, program) {
        site.seen += 1;
        if site.skip == 0 {
            return true;
        }
        site.skip -= 1;
        *expr = saved;
    }
    match expr {
        Expr::Value { .. } | Expr::Var { .. } => false,
        Expr::Let { bound, body, .. } => {
            mutate_expr(bound, mutation, program, site)
                || mutate_expr(body, mutation, program, site)
        }
        Expr::Cond {
            condition,
            then_branch,
            else_branch,
        } => {
            mutate_expr(condition, mutation, program, site)
                || mutate_expr(then_branch, mutation, program, site)
                || mutate_expr(else_branch, mutation, program, site)
        }
        Expr::Match {
            scrutinee, arms, ..
        } => {
            mutate_expr(scrutinee, mutation, program, site)
                || arms
                    .iter_mut()
                    .any(|arm| mutate_expr(&mut arm.body, mutation, program, site))
        }
        Expr::Build { operands, .. }
        | Expr::Call { operands, .. }
        | Expr::Prim { operands, .. }
        | Expr::Closure {
            captures: operands, ..
        } => operands
            .iter_mut()
            .any(|operand| mutate_expr(operand, mutation, program, site)),
        Expr::Apply { target, operands } => {
            mutate_expr(target, mutation, program, site)
                || operands
                    .iter_mut()
                    .any(|operand| mutate_expr(operand, mutation, program, site))
        }
        Expr::First { value } | Expr::Second { value } | Expr::Field { value, .. } => {
            mutate_expr(value, mutation, program, site)
        }
    }
}

/// Whether the function of `origin` is one `mutation` is planted in: the
/// entry for a validation, the validators for a clause or a weakening, the
/// closure for every other.
fn eligible(origin: &lexlean::production::lower::Origin, mutation: Mutation) -> bool {
    use lexlean::production::lower::Origin;
    match origin {
        Origin::Entry { .. } => mutation == Mutation::Validation,
        Origin::Validator { .. } => matches!(mutation, Mutation::Clause | Mutation::Weakening),
        Origin::Definition { .. }
        | Origin::Instance { .. }
        | Origin::Lambda { .. }
        | Origin::Template { .. } => !Mutation::BOUNDARY.contains(&mutation),
    }
}

/// Every place `mutation` applies in `lowered`, as (function, ordinal of the
/// place within the function), in function order.
#[must_use]
pub fn sites(
    lowered: &lexlean::production::lower::Lowered,
    mutation: Mutation,
) -> Vec<(usize, usize)> {
    let program = &lowered.program;
    let mut out = Vec::new();
    for (index, function) in program.functions.iter().enumerate() {
        if !eligible(&lowered.layout.functions[index], mutation) {
            continue;
        }
        let mut probe = function.body.clone();
        let mut site = Site {
            skip: usize::MAX,
            seen: 0,
        };
        mutate_expr(&mut probe, mutation, program, &mut site);
        out.extend((0..site.seen).map(|ordinal| (index, ordinal)));
    }
    out
}

/// The lowered program with `mutation` planted at the `ordinal`-th place it
/// applies in `function`.
#[must_use]
pub fn mutate_at(
    lowered: &lexlean::production::lower::Lowered,
    mutation: Mutation,
    function: usize,
    ordinal: usize,
) -> Option<Program> {
    let program = &lowered.program;
    let mut mutated = program.clone();
    let mut site = Site {
        skip: ordinal,
        seen: 0,
    };
    mutate_expr(
        &mut mutated.functions[function].body,
        mutation,
        program,
        &mut site,
    )
    .then_some(mutated)
}

/// The sites to plant: the first, one in the middle, and the last, so that a
/// mutation is exercised at the start, in the interior, and at the end of
/// whatever it applies to.
#[must_use]
pub fn spread<T: Clone>(all: &[T]) -> Vec<T> {
    let mut chosen: Vec<usize> = vec![0, all.len() / 2, all.len().saturating_sub(1)];
    chosen.dedup();
    chosen.sort_unstable();
    chosen.dedup();
    chosen
        .into_iter()
        .filter_map(|index| all.get(index).cloned())
        .collect()
}

/// One planted mutation and what Lean made of its certificate.
#[derive(Debug, Clone)]
pub struct Planted {
    /// The root whose program was mutated.
    pub root: String,
    /// The mutation.
    pub mutation: Mutation,
    /// The function whose body was mutated.
    pub function: usize,
    /// The place within that function, in evaluation order.
    pub ordinal: usize,
    /// How the lowering produced that function.
    pub origin: lexlean::production::lower::Origin,
    /// The declarations the first error may lie in: the relation of the
    /// function changed, or, for a library template whose relation the
    /// certificate states at its callers, a relation of one of them.
    pub expected: Vec<String>,
    /// Lean's output on the mutated certificate; empty when it was
    /// accepted, which is a failure of the certificate.
    pub rejection: String,
    /// The size of the mutated certificate's text, which decides whether a
    /// heartbeat verdict on it could be read as the machine's limit.
    pub text_len: usize,
    /// The certificate's declaration Lean's first error lies in, which is
    /// what makes the failure targeted: the relation of the mutated
    /// function, not an unrelated one.
    pub declaration: Option<String>,
}

/// The top-level declaration of `text` that the first error of Lean's
/// `output` lies in.
#[must_use]
pub fn failing_declaration(text: &str, output: &str) -> Option<String> {
    lexlean::production::preserve::failing_declaration(text, output)
}

/// `lowered` as a lowering would produce it if it did not validate the
/// parameter at `position`: the entry omits the check and its layout says
/// the parameter has no validator, consistently, so that only a judgement
/// from the source types can see it.
///
/// # Panics
///
/// Panics when `lowered` has no entry or that parameter has no validator.
#[must_use]
pub fn skip_parameter_validator(
    lowered: &lexlean::production::lower::Lowered,
    position: usize,
) -> lexlean::production::lower::Lowered {
    use lexlean::production::lower::Origin;
    let entry = lowered.entry() as usize;
    let Origin::Entry { validators } = lowered.layout.functions[entry].clone() else {
        panic!("a root with an entry");
    };
    assert!(validators[position].is_some(), "a validated parameter");
    let ordinal = validators[..position].iter().flatten().count();
    let mut out = lowered.clone();
    out.program =
        mutate_at(lowered, Mutation::Validation, entry, ordinal).expect("the entry has that check");
    let mut skipped = validators;
    skipped[position] = None;
    out.layout.functions[entry] = Origin::Entry {
        validators: skipped,
    };
    out
}

/// `lowered` as a lowering would produce it if it decided that the type
/// checked by validator `index` carries no invariant: the validator is the
/// trivial one.
#[must_use]
pub fn trivialize_validator(
    lowered: &lexlean::production::lower::Lowered,
    index: usize,
) -> lexlean::production::lower::Lowered {
    use lexlean::calculus::Shape;
    use lexlean::production::lower::{Origin, Validation};
    let mut out = lowered.clone();
    let Origin::Validator {
        ty,
        module,
        kind: _,
    } = lowered.layout.functions[index].clone()
    else {
        panic!("a validator");
    };
    out.layout.functions[index] = Origin::Validator {
        ty,
        module,
        kind: Validation::Trivial,
    };
    out.program.functions[index].body = Expr::Build {
        shape: Shape::True,
        ty: Ty::Bool,
        operands: Vec::new(),
    };
    out
}

/// The declarations of certificate A in which a defect in the body of
/// `function` is found: its own relation, or for a template member, the
/// relation of every function that calls into its instance.
fn expected_declarations(
    lowered: &lexlean::production::lower::Lowered,
    function: usize,
) -> Vec<String> {
    use lexlean::production::lower::{referenced, Origin};
    match &lowered.layout.functions[function] {
        Origin::Definition { .. } | Origin::Instance { .. } | Origin::Lambda { .. } => {
            vec![format!("__rel_{function}")]
        }
        Origin::Validator { .. } => vec![format!("__vrel_{function}")],
        Origin::Entry { .. } => vec!["entry".to_owned()],
        Origin::Template {
            template,
            types,
            entry,
            ..
        } => {
            let in_instance = |index: usize| {
                matches!(
                    &lowered.layout.functions[index],
                    Origin::Template { template: t, types: ty, entry: e, .. }
                        if t == template && ty == types && e == entry
                )
            };
            let mut callers = Vec::new();
            for (index, body) in lowered.program.functions.iter().enumerate() {
                if in_instance(index) {
                    continue;
                }
                let mut called = Vec::new();
                referenced(&body.body, &mut called);
                if called.iter().any(|callee| in_instance(*callee as usize)) {
                    callers.push(format!("__rel_{index}"));
                }
            }
            callers
        }
    }
}

/// Plant `mutations` after lowering, while the proof is still derived from
/// the source: each at the first, a middle, and the last place it applies in
/// the project's roots; regenerate that root's certificate against the
/// mutated program, and compile it.
///
/// # Panics
///
/// Panics when the unmutated environment does not compile.
#[must_use]
pub fn plant(project: &P, mutations: &[Mutation]) -> Vec<Planted> {
    let checked = support::checked_project(project);
    let modules = linked_modules(&checked);
    let rendered = support::rendered(project);
    let user: Vec<(String, String)> = rendered
        .modules
        .iter()
        .map(|module| (module.lean_module.clone(), module.lean_text.clone()))
        .collect();
    let base = workspace(&user, &[]).expect("the workspace stages");
    let scratch = tempfile::Builder::new()
        .prefix("lexlean-plant-")
        .tempdir()
        .expect("tempdir");
    let compiled = check(&base, scratch.path());
    assert!(compiled.failure.is_none(), "{:?}", compiled.failure);
    let roots = roots(&checked).expect("the eligibility reports");
    let limits = support::limits(project);
    let lowerings: Vec<_> = roots
        .iter()
        .map(|root| {
            lower_root(&modules, &root.module, &root.name, root.report, &limits)
                .expect("an eligible root lowers")
        })
        .collect();
    let mut jobs = Vec::new();
    for mutation in mutations.iter().copied() {
        let all: Vec<(usize, usize, usize)> = lowerings
            .iter()
            .enumerate()
            .flat_map(|(root, lowered)| {
                sites(lowered, mutation)
                    .into_iter()
                    .map(move |(function, ordinal)| (root, function, ordinal))
            })
            .collect();
        // A mutation of the boundary is planted at every place it applies:
        // each validated parameter in turn, each component check of each
        // validator, each order comparison.
        let chosen = if Mutation::BOUNDARY.contains(&mutation) {
            all
        } else {
            spread(&all)
        };
        for (root, function, ordinal) in chosen {
            jobs.push((mutation, root, function, ordinal));
        }
    }
    let directory = scratch.path().join("src/LexLeanPreserve/Planted");
    std::fs::create_dir_all(&directory).expect("mkdir");
    parallel(&jobs, |(mutation, root, function, ordinal)| {
        let (mutation, root, function, ordinal) = (*mutation, *root, *function, *ordinal);
        let mut lowered = lowerings[root].clone();
        let origin = lowered.layout.functions[function].clone();
        let expected = expected_declarations(&lowered, function);
        lowered.program =
            mutate_at(&lowered, mutation, function, ordinal).expect("the site was counted");
        let index = jobs
            .iter()
            .position(|job| *job == (mutation, root, function, ordinal))
            .expect("the job");
        let module = format!("LexLeanPreserve.Planted.P{index}");
        let certificate = certificate(
            &modules,
            &roots[root].module,
            &roots[root].name,
            roots[root].report,
            &lowered,
            &module,
            UNBOUNDED,
        )
        .expect("the certificate is generated from the source");
        let path = directory.join(format!("P{index}.lean"));
        std::fs::write(&path, &certificate.text).expect("write");
        let output = lean(scratch.path(), &[path.display().to_string()]);
        let rejection = if output.status.success() {
            String::new()
        } else {
            joined(&output)
        };
        Planted {
            root: roots[root].report.root.clone(),
            mutation,
            function,
            ordinal,
            origin,
            expected,
            declaration: failing_declaration(&certificate.text, &rejection),
            text_len: certificate.text.len(),
            rejection,
        }
    })
}

/// A defect planted in a rendered crate after rendering, while the program
/// it realizes is unchanged: what certificate B exists to catch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RustMutation {
    /// The first `if`'s branches are swapped.
    Branches,
    /// The first natural addition subtracts.
    Arithmetic,
    /// The first checked fixed-width operation checks another operation's
    /// bound: an addition or a multiplication a subtraction's, a
    /// subtraction an addition's, so it overflows where the program does
    /// not.
    Overflow,
    /// The first checked fixed-width operation works at another width: the
    /// machine's item of that width takes no operand of the program's, as
    /// Rust's typing takes none.
    Width,
    /// The first list, bytes, or text item calls its sibling for another
    /// sequence: `length_string` as `length_bytes`, which counts bytes
    /// where the program counts characters.
    Sequence,
    /// The first construction of an enum variant builds a sibling variant.
    Constructor,
    /// The first byte-buffer literal gains a byte.
    Buffer,
    /// The first call of a crate function calls a neighbouring one.
    Recursion,
    /// The first natural literal is one larger.
    Literal,
    /// The crate's entry calls the root without the check of its first
    /// validator: the rendering no longer refuses an invalid input.
    Validation,
}

impl RustMutation {
    /// Every mutation.
    pub const ALL: [Self; 10] = [
        Self::Branches,
        Self::Arithmetic,
        Self::Overflow,
        Self::Width,
        Self::Sequence,
        Self::Constructor,
        Self::Buffer,
        Self::Recursion,
        Self::Literal,
        Self::Validation,
    ];
}

/// The width a checked operation is mutated to: the next wider kind, or
/// the next narrower for the widest.
fn other_width(kind: lexlean::calculus::IntKind) -> lexlean::calculus::IntKind {
    use lexlean::calculus::IntKind;
    match kind {
        IntKind::U8 => IntKind::U16,
        IntKind::U16 => IntKind::U32,
        IntKind::U32 => IntKind::U64,
        IntKind::U64 => IntKind::U32,
        IntKind::I8 => IntKind::I16,
        IntKind::I16 => IntKind::I32,
        IntKind::I32 => IntKind::I64,
        IntKind::I64 => IntKind::I32,
    }
}

/// Whether a block ends in `None`, or `Ok(None)` in a function that can
/// fail: the refusal of an entry.
fn refuses(block: &lexlean::calculus::rust::ast::Block) -> bool {
    use lexlean::calculus::rust::ast::{Ctor, Expr as R};
    fn none(expr: &R) -> bool {
        match expr {
            R::Construct {
                ctor: Ctor::None(_),
                ..
            } => true,
            R::Succeed(inner, _) => none(inner),
            _ => false,
        }
    }
    none(&block.tail)
}

/// Plant `mutation` at `expr` itself, when it applies there.
fn apply_rust(
    expr: &mut lexlean::calculus::rust::ast::Expr,
    mutation: RustMutation,
    variants: &dyn Fn(u64) -> u64,
    functions: u64,
) -> bool {
    use lexlean::calculus::rust::ast::{Callee, Ctor, Expr as R, Lit};
    use lexlean::calculus::rust::runtime::Item;
    match (mutation, &mut *expr) {
        (
            RustMutation::Validation,
            R::If {
                then_branch,
                else_branch,
                ..
            },
        ) if refuses(then_branch) || refuses(else_branch) => {
            let kept = if refuses(else_branch) {
                then_branch.clone()
            } else {
                else_branch.clone()
            };
            *expr = R::Block(kept);
            true
        }
        (
            RustMutation::Branches,
            R::If {
                then_branch,
                else_branch,
                ..
            },
        ) => {
            std::mem::swap(then_branch, else_branch);
            true
        }
        (
            RustMutation::Arithmetic,
            R::Call {
                callee: callee @ Callee::Runtime(Item::NatAdd),
                ..
            },
        ) => {
            *callee = Callee::Runtime(Item::NatSub);
            true
        }
        (
            RustMutation::Overflow,
            R::Call {
                callee: Callee::Runtime(item),
                ..
            },
        ) => match *item {
            Item::CheckedAdd(kind) | Item::CheckedMul(kind) => {
                *item = Item::CheckedSub(kind);
                true
            }
            Item::CheckedSub(kind) => {
                *item = Item::CheckedAdd(kind);
                true
            }
            _ => false,
        },
        (
            RustMutation::Width,
            R::Call {
                callee: Callee::Runtime(item),
                ..
            },
        ) => match *item {
            Item::CheckedAdd(kind) => {
                *item = Item::CheckedAdd(other_width(kind));
                true
            }
            Item::CheckedSub(kind) => {
                *item = Item::CheckedSub(other_width(kind));
                true
            }
            Item::CheckedMul(kind) => {
                *item = Item::CheckedMul(other_width(kind));
                true
            }
            _ => false,
        },
        (
            RustMutation::Sequence,
            R::Call {
                callee: Callee::Runtime(item),
                ..
            },
        ) => {
            let sibling = match *item {
                Item::LengthString | Item::LengthList => Some(Item::LengthBytes),
                Item::LengthBytes => Some(Item::LengthList),
                Item::AppendList => Some(Item::AppendBytes),
                Item::AppendBytes => Some(Item::AppendList),
                Item::IndexList => Some(Item::IndexBytes),
                Item::IndexBytes => Some(Item::IndexList),
                Item::SliceList => Some(Item::SliceBytes),
                Item::SliceBytes => Some(Item::SliceList),
                _ => None,
            };
            sibling.is_some_and(|sibling| {
                *item = sibling;
                true
            })
        }
        (
            RustMutation::Constructor,
            R::Construct {
                ctor: Ctor::Adt { adt, constructor },
                ..
            },
        ) => {
            let count = variants(*adt);
            if count > 1 {
                *constructor = (*constructor + 1) % count;
                true
            } else {
                false
            }
        }
        (RustMutation::Buffer, R::Lit(Lit::Bytes(bytes), _)) => {
            bytes.push(0);
            true
        }
        (
            RustMutation::Recursion,
            R::Call {
                callee: Callee::Function(function),
                ..
            },
        ) if functions > 1 => {
            *function = (*function + 1) % functions;
            true
        }
        (RustMutation::Literal, R::Lit(Lit::Nat(value), _)) => {
            *value = value.wrapping_add(1);
            true
        }
        _ => false,
    }
}

fn mutate_rust(
    expr: &mut lexlean::calculus::rust::ast::Expr,
    mutation: RustMutation,
    variants: &dyn Fn(u64) -> u64,
    functions: u64,
    site: &mut Site,
) -> bool {
    let saved = expr.clone();
    if apply_rust(expr, mutation, variants, functions) {
        site.seen += 1;
        if site.skip == 0 {
            return true;
        }
        site.skip -= 1;
        *expr = saved;
    }
    rust_children(expr)
        .into_iter()
        .any(|child| mutate_rust(child, mutation, variants, functions, site))
}

/// The expressions directly below `expr`, in rendering order.
fn rust_children(
    expr: &mut lexlean::calculus::rust::ast::Expr,
) -> Vec<&mut lexlean::calculus::rust::ast::Expr> {
    use lexlean::calculus::rust::ast::{Block, Expr as R};
    fn block(block: &mut Block) -> Vec<&mut R> {
        let mut out: Vec<&mut R> = block
            .lets
            .iter_mut()
            .map(|binding| &mut binding.value)
            .collect();
        out.push(&mut block.tail);
        out
    }
    match expr {
        R::Box(inner, _) | R::Widen(inner, _) | R::Succeed(inner, _) | R::Not(inner, _) => {
            vec![inner.as_mut()]
        }
        R::Call { args, .. } | R::Apply { args, .. } | R::Construct { args, .. } => {
            args.iter_mut().collect()
        }
        R::Pair(left, right, _) => vec![left.as_mut(), right.as_mut()],
        R::If {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            let mut out = vec![condition.as_mut()];
            out.extend(block(then_branch));
            out.extend(block(else_branch));
            out
        }
        R::Match {
            scrutinee, arms, ..
        } => {
            let mut out = vec![scrutinee.as_mut()];
            for (_, body) in arms {
                out.extend(block(body));
            }
            out
        }
        R::Block(inner) => block(inner),
        R::Lit(..)
        | R::Move(..)
        | R::Clone(..)
        | R::Copy(..)
        | R::Deref(..)
        | R::Unbox(..)
        | R::Uncons(..)
        | R::IsZero(..)
        | R::NonZero(..)
        | R::Predecessor(..) => Vec::new(),
    }
}

/// The number of the program function a crate function renders.
fn function_number(name: &lexlean::calculus::rust::ast::Ident) -> Option<usize> {
    use lexlean::calculus::rust::ast::Ident;
    match name {
        Ident::Function(number) => usize::try_from(*number).ok(),
        Ident::Local(_)
        | Ident::Holder(_)
        | Ident::Operand(_)
        | Ident::Callee(_)
        | Ident::Boxed(_)
        | Ident::Part(_)
        | Ident::Capture(_)
        | Ident::Param(_)
        | Ident::Export(_) => None,
    }
}

/// Run `plant` on the body of the crate function `number`, which is
/// `mutation`'s to change: every function, except that a validation is
/// planted only in the entry, the crate's last function.
fn crate_function<R>(
    krate: &mut lexlean::calculus::rust::ast::Crate,
    mutation: RustMutation,
    mut visit: impl FnMut(
        usize,
        &mut lexlean::calculus::rust::ast::Block,
        &dyn Fn(u64) -> u64,
        u64,
    ) -> Option<R>,
) -> Option<R> {
    use lexlean::calculus::rust::ast::{ItemDef, Type};
    let variants: Vec<(Type, u64)> = krate
        .items
        .iter()
        .filter_map(|item| match item {
            ItemDef::Enum { name, variants, .. } => Some((name.clone(), variants.len() as u64)),
            _ => None,
        })
        .collect();
    let count = |adt: u64| {
        variants
            .iter()
            .find_map(|(name, count)| (*name == Type::Adt(adt)).then_some(*count))
            .unwrap_or(0)
    };
    let functions = krate
        .items
        .iter()
        .filter(|item| matches!(item, ItemDef::Function { .. }))
        .count() as u64;
    let last = krate
        .items
        .iter()
        .filter_map(|item| match item {
            ItemDef::Function { name, .. } => function_number(name),
            _ => None,
        })
        .max();
    for item in &mut krate.items {
        if let ItemDef::Function { name, body, .. } = item {
            let Some(number) = function_number(name) else {
                continue;
            };
            if mutation == RustMutation::Validation && Some(number) != last {
                continue;
            }
            if let Some(found) = visit(number, body, &count, functions) {
                return Some(found);
            }
        }
    }
    None
}

/// Every place `mutation` applies in `krate`, as (program function, ordinal
/// of the place within it), in function order.
#[must_use]
pub fn crate_sites(
    krate: &lexlean::calculus::rust::ast::Crate,
    mutation: RustMutation,
) -> Vec<(usize, usize)> {
    let mut probe = krate.clone();
    let mut out = Vec::new();
    let _: Option<()> =
        crate_function(&mut probe, mutation, |number, body, variants, functions| {
            let mut site = Site {
                skip: usize::MAX,
                seen: 0,
            };
            for expr in body
                .lets
                .iter_mut()
                .map(|binding| &mut binding.value)
                .chain([&mut body.tail])
            {
                mutate_rust(expr, mutation, variants, functions, &mut site);
            }
            out.extend((0..site.seen).map(|ordinal| (number, ordinal)));
            None
        });
    out
}

/// `krate` with `mutation` planted at the `ordinal`-th place it applies in
/// the crate function `function`.
#[must_use]
pub fn mutate_crate_at(
    krate: &lexlean::calculus::rust::ast::Crate,
    mutation: RustMutation,
    function: usize,
    ordinal: usize,
) -> Option<lexlean::calculus::rust::ast::Crate> {
    let mut mutated = krate.clone();
    crate_function(
        &mut mutated,
        mutation,
        |number, body, variants, functions| {
            if number != function {
                return None;
            }
            let mut site = Site {
                skip: ordinal,
                seen: 0,
            };
            body.lets
                .iter_mut()
                .map(|binding| &mut binding.value)
                .chain([&mut body.tail])
                .any(|expr| mutate_rust(expr, mutation, variants, functions, &mut site))
                .then_some(())
        },
    )?;
    Some(mutated)
}

/// `krate` with `mutation` planted at its first applicable site, if any.
#[must_use]
pub fn mutate_crate(
    krate: &lexlean::calculus::rust::ast::Crate,
    mutation: RustMutation,
) -> Option<lexlean::calculus::rust::ast::Crate> {
    let (function, ordinal) = *crate_sites(krate, mutation).first()?;
    mutate_crate_at(krate, mutation, function, ordinal)
}

/// One renderer fixture rendered in one profile, with its certificate B.
#[derive(Debug, Clone)]
pub struct Rendering {
    /// The fixture's name.
    pub fixture: String,
    /// The profile's target.
    pub target: String,
    /// The fixture's program.
    pub program: Program,
    /// Its crate in the profile.
    pub krate: lexlean::calculus::rust::ast::Crate,
    /// Certificate B relating the two.
    pub certificate: Certificate,
    /// Whether the program has a boundary entry (§17.17), its last
    /// function.
    pub entry: bool,
}

/// Every renderer fixture in every profile that renders it, each with the
/// certificate B the aligner derives.
///
/// # Panics
///
/// Panics when a profile refuses a fixture for any reason but `rust-core`'s
/// refusal of the heap, or when the aligner derives no certificate.
#[must_use]
pub fn fixture_renderings() -> Vec<Rendering> {
    let mut out = Vec::new();
    for (index, case) in crate::calculus::cases().into_iter().enumerate() {
        for profile in Profile::ALL {
            let krate = match lower(&case.fixture.program, profile) {
                Ok(krate) => krate,
                Err(reason) => {
                    assert!(
                        profile == Profile::Core && reason.contains("requires heap allocation"),
                        "{} ({}): {reason}",
                        case.fixture.name,
                        profile.target()
                    );
                    continue;
                }
            };
            let module = module_for(
                &format!("LexLeanPreserve.Fixture.F{index}"),
                profile.target(),
            )
            .expect("a Rust target");
            let certificate = certificate_b(&case.fixture.program, &krate, &module, UNBOUNDED)
                .unwrap_or_else(|reason| {
                    panic!(
                        "{} ({}): certificate B: {reason}",
                        case.fixture.name,
                        profile.target()
                    )
                });
            out.push(Rendering {
                fixture: case.fixture.name.clone(),
                target: profile.target().to_owned(),
                program: case.fixture.program.clone(),
                krate,
                certificate: certificate.into_certificate(),
                entry: false,
            });
        }
    }
    out
}

/// The renderings of every root of `certified` that has a boundary entry,
/// each with the certificate B the aligner derived for it.
///
/// # Panics
///
/// Panics when a profile refuses a rendering the certificate covers.
#[must_use]
pub fn entry_renderings(certified: &[Certified]) -> Vec<Rendering> {
    let mut out = Vec::new();
    for entry in certified.iter().filter(|entry| entry.entry != 0) {
        for (target, certificate) in &entry.renderings {
            let profile = Profile::named(target).expect("a profile");
            out.push(Rendering {
                fixture: entry.root.clone(),
                target: target.clone(),
                program: entry.program.clone(),
                krate: lower(&entry.program, profile).expect("the program renders"),
                certificate: certificate.clone(),
                entry: true,
            });
        }
    }
    out
}

/// The rules of the correspondence a certificate's derivations use.
#[must_use]
pub fn rules_used(text: &str) -> std::collections::BTreeSet<String> {
    text.split("(Corr.")
        .skip(1)
        .map(|rest| {
            rest.chars()
                .take_while(char::is_ascii_alphanumeric)
                .collect()
        })
        .collect()
}

/// One crate mutation and what became of it.
#[derive(Debug, Clone)]
pub struct RustPlanted {
    /// The fixture whose crate was mutated.
    pub fixture: String,
    /// The profile it was rendered in.
    pub target: String,
    /// The mutation.
    pub mutation: RustMutation,
    /// The program function whose rendering was mutated.
    pub function: usize,
    /// The place within it, in rendering order.
    pub ordinal: usize,
    /// The aligner's reason when it derives nothing for the mutated crate.
    pub unaligned: Option<String>,
    /// Lean's output on the derivation the aligner wrote for the mutated
    /// crate, when it wrote one; empty when Lean accepted it.
    pub realigned: Option<String>,
    /// The declaration of the realigned derivation Lean's first error lies
    /// in.
    pub realigned_declaration: Option<String>,
    /// Lean's output on the unmutated derivation restated over the mutated
    /// crate; empty when Lean accepted it.
    pub stale: String,
    /// The declaration of the restated derivation Lean's first error lies
    /// in: the derivation `fun<k>` of the mutated function.
    pub stale_declaration: Option<String>,
}

/// Plant every mutation at the first, a middle, and the last place it
/// applies across the renderings: ask the aligner for the mutated crate's
/// certificate, and restate the unmutated certificate over the mutated crate.
#[must_use]
pub fn plant_renderings(renderings: &[Rendering]) -> Vec<(RustPlanted, Vec<(String, String)>)> {
    let mut out = Vec::new();
    for mutation in RustMutation::ALL {
        let all: Vec<(usize, usize, usize)> = renderings
            .iter()
            .enumerate()
            .filter(|(_, rendering)| mutation != RustMutation::Validation || rendering.entry)
            .flat_map(|(index, rendering)| {
                crate_sites(&rendering.krate, mutation)
                    .into_iter()
                    .map(move |(function, ordinal)| (index, function, ordinal))
            })
            .collect();
        // A validation is planted at every validated parameter of every
        // entry; any other mutation at the first, a middle, and the last.
        let chosen = if mutation == RustMutation::Validation {
            all
        } else {
            spread(&all)
        };
        for (index, function, ordinal) in chosen {
            let rendering = &renderings[index];
            let mutated = mutate_crate_at(&rendering.krate, mutation, function, ordinal)
                .expect("the site was counted");
            let base = format!("LexLeanPreserve.Planted.{mutation:?}F{index}N{function}O{ordinal}");
            let stale_module = format!("{base}.Stale");
            let stale_text = rendering
                .certificate
                .text
                .replace(
                    &lexlean::production::rust_term::crate_term(&rendering.krate),
                    &lexlean::production::rust_term::crate_term(&mutated),
                )
                .replace(&rendering.certificate.module, &stale_module);
            let mut modules = vec![(stale_module, stale_text)];
            let realigned_module = format!("{base}.Realigned");
            let unaligned =
                match certificate_b(&rendering.program, &mutated, &realigned_module, UNBOUNDED) {
                    Ok(certificate) => {
                        modules.push((realigned_module, certificate.text));
                        None
                    }
                    Err(reason) => Some(reason),
                };
            out.push((
                RustPlanted {
                    fixture: rendering.fixture.clone(),
                    target: rendering.target.clone(),
                    mutation,
                    function,
                    ordinal,
                    unaligned,
                    realigned: None,
                    realigned_declaration: None,
                    stale: String::new(),
                    stale_declaration: None,
                },
                modules,
            ));
        }
    }
    out
}

/// What checking the renderings' certificates established.
#[derive(Debug, Clone)]
pub struct RenderingsChecked {
    /// The audit module's output.
    pub audit_output: String,
    /// Every planted mutation, with Lean's verdicts filled in.
    pub planted: Vec<RustPlanted>,
}

/// Stage every rendering's certificate in the preservation environment,
/// compile, replay, and audit them, then compile every planted module
/// against the same environment.
///
/// # Panics
///
/// Panics naming the first certificate that fails to compile or replay,
/// or the first axiom disagreement.
#[must_use]
pub fn check_renderings(
    renderings: &[Rendering],
    planted: Vec<(RustPlanted, Vec<(String, String)>)>,
) -> RenderingsChecked {
    let certificates: Vec<Certificate> = renderings
        .iter()
        .map(|rendering| rendering.certificate.clone())
        .collect();
    let staged = workspace(&[], &certificates).expect("the workspace stages");
    let scratch = tempfile::Builder::new()
        .prefix("lexlean-renderings-")
        .tempdir()
        .expect("tempdir");
    let checked = check(&staged, scratch.path());
    if let Some((module, output)) = checked.failure {
        panic!("renderings: `{module}` was rejected:\n{output}");
    }
    audit(&checked.audit_output, &certificates)
        .unwrap_or_else(|reason| panic!("renderings: axiom audit: {reason}"));
    let jobs: Vec<(usize, String, String)> = planted
        .iter()
        .enumerate()
        .flat_map(|(index, (_, modules))| {
            modules
                .iter()
                .map(move |(module, text)| (index, module.clone(), text.clone()))
        })
        .collect();
    let verdicts = parallel(&jobs, |(_, module, text)| {
        let path = scratch
            .path()
            .join("src")
            .join(lexlean::production::preserve::module_path(module));
        std::fs::create_dir_all(path.parent().expect("a parent")).expect("mkdir");
        std::fs::write(&path, text).expect("write");
        let output = lean(scratch.path(), &[path.display().to_string()]);
        if output.status.success() {
            String::new()
        } else {
            joined(&output)
        }
    });
    let mut planted: Vec<RustPlanted> = planted.into_iter().map(|(plant, _)| plant).collect();
    for ((index, module, text), verdict) in jobs.iter().zip(verdicts) {
        let declaration = failing_declaration(text, &verdict);
        if module.ends_with(".Stale") {
            planted[*index].stale = verdict;
            planted[*index].stale_declaration = declaration;
        } else {
            planted[*index].realigned = Some(verdict);
            planted[*index].realigned_declaration = declaration;
        }
    }
    RenderingsChecked {
        audit_output: checked.audit_output,
        planted,
    }
}

/// What the rustc differential established for one project.
#[derive(Debug, Clone)]
pub struct RustcRun {
    /// The crates built and run: one per root and target.
    pub crates: usize,
    /// The runs whose printed outcome equalled the interpreter's.
    pub runs: usize,
    /// The runs through a boundary entry, valid and invalid inputs.
    pub entered: usize,
}

/// Render every certified root of `project` in each of its targets as a
/// package, build the packages with the pinned `rustc` under cargo, run
/// each on the seeded inputs of the differential, through the root and
/// through its entry, and compare every printed outcome with the
/// interpreter's, which the declared machine reproduces (SP-07). That the
/// compiler agrees with the declaration the certificates are about is
/// build evidence, never a premise of a proof.
///
/// # Panics
///
/// Panics when a package does not build or lint, or on every run whose
/// outcome differs.
#[must_use]
pub fn rustc_differential(project: &P, certified: &[Certified], example: &str) -> RustcRun {
    use crate::rust_build::{cargo_in, workspace, Built};
    use crate::rust_harness::{render_calls, Calls};
    use lexlean::calculus::rust::package::{
        package, Errors, Export, Manifest, Passing, MANIFEST_SPEC,
    };
    let cases = crate::differential::cases(project);
    let recorded = recorded_entries(example);
    let dir = tempfile::Builder::new()
        .prefix("lexlean-rustc-roots-")
        .tempdir()
        .expect("tempdir");
    let mut built = Vec::new();
    let mut expected: std::collections::BTreeMap<String, Vec<serde_json::Value>> =
        std::collections::BTreeMap::new();
    let (mut runs, mut entered) = (0, 0);
    for (position, entry) in certified.iter().enumerate() {
        let fallible =
            lexlean::calculus::rust::fallible_functions(&entry.program).expect("a valid program");
        for target in &entry.targets {
            // The function a caller of the crate invokes is the one the
            // record of the verified example states for this root and
            // target (`preservation.json`, which the attestation binds), and
            // it is the one the generated lowering agrees with. This test's
            // package exports it alone: exporting the root of a program with
            // a boundary beside its entry would let a caller bypass the
            // validators.
            let invoked = *recorded
                .get(&(entry.root.clone(), target.clone()))
                .unwrap_or_else(|| panic!("{} on {target}: no recorded entry", entry.root));
            assert_eq!(
                invoked, entry.entry,
                "{} on {target}: the record states function {invoked}, the lowering {}",
                entry.root, entry.entry
            );
            let functions: Vec<(u64, &str)> = if invoked == 0 {
                vec![(0, "root")]
            } else {
                vec![(invoked, "entry")]
            };
            let profile = Profile::named(target).expect("a profile");
            let name = format!(
                "root_{position}_{}",
                if profile == Profile::Core {
                    "core"
                } else {
                    "std"
                }
            );
            let manifest = Manifest {
                spec: MANIFEST_SPEC.to_owned(),
                name: name.clone(),
                version: "1.0.0".to_owned(),
                profile: target.clone(),
                program: entry.program.clone(),
                exports: functions
                    .iter()
                    .map(|(function, exported)| Export {
                        function: *function,
                        name: (*exported).to_owned(),
                        parameters: vec![
                            Passing::Own;
                            entry.program.functions[*function as usize].types.len()
                        ],
                        errors: if fallible[*function as usize] {
                            Errors::Overflow
                        } else {
                            Errors::None
                        },
                    })
                    .collect(),
                sources: Vec::new(),
            };
            let rendered = package(&manifest)
                .unwrap_or_else(|reason| panic!("{} on {target}: {reason}", entry.root));
            let owned: Vec<Vec<Passing>> = functions
                .iter()
                .map(|(function, _)| {
                    vec![Passing::Own; entry.program.functions[*function as usize].types.len()]
                })
                .collect();
            let arguments: Vec<Vec<Vec<Value>>> = functions
                .iter()
                .map(|(function, _)| {
                    cases
                        .get(&entry.root)
                        .map(|cases| {
                            cases
                                .iter()
                                .filter(|case| case.function == *function)
                                .map(|case| case.values.clone())
                                .collect()
                        })
                        .unwrap_or_default()
                })
                .collect();
            let calls: Vec<Calls<'_>> = functions
                .iter()
                .enumerate()
                .map(|(index, (function, exported))| Calls {
                    entry: *function,
                    function: exported,
                    passing: &owned[index],
                    arguments: &arguments[index],
                })
                .collect();
            let harness = render_calls(&entry.program, profile, &name, &calls)
                .unwrap_or_else(|reason| panic!("{} on {target}: {reason}", entry.root));
            let mut outcomes = Vec::new();
            for (function, _) in &functions {
                for case in cases
                    .get(&entry.root)
                    .map_or(&[][..], Vec::as_slice)
                    .iter()
                    .filter(|case| case.function == *function)
                {
                    outcomes.push(
                        lexlean::calculus::rust::observable(&case.outcome)
                            .unwrap_or_else(|| panic!("{}: {:?}", entry.root, case.outcome)),
                    );
                    runs += 1;
                    entered += usize::from(*function != 0);
                }
            }
            expected.insert(name.clone(), outcomes);
            built.push(Built {
                name,
                files: rendered.files,
                harness,
            });
        }
    }
    workspace(dir.path(), &built);
    let build = cargo_in(
        dir.path(),
        &["build", "--offline", "--workspace", "--quiet"],
    );
    assert!(
        build.status.success(),
        "a root's package does not build:\n{}",
        String::from_utf8_lossy(&build.stderr)
    );
    let mut failures = Vec::new();
    for (name, outcomes) in &expected {
        let binary = dir
            .path()
            .join("target/debug")
            .join(format!("run_{name}{}", std::env::consts::EXE_SUFFIX));
        let ran = std::process::Command::new(&binary)
            .output()
            .expect("the harness runs");
        assert!(
            ran.status.success(),
            "{name}: {}",
            String::from_utf8_lossy(&ran.stderr)
        );
        let stdout = String::from_utf8(ran.stdout).expect("utf8");
        let printed: Vec<serde_json::Value> = stdout
            .lines()
            .filter(|line| !line.starts_with("work "))
            .map(|line| {
                serde_json::from_str(line).unwrap_or_else(|e| panic!("{name}: {e}: {line}"))
            })
            .collect();
        assert_eq!(printed.len(), outcomes.len(), "{name}: one outcome per run");
        for (run, (printed, expected)) in printed.iter().zip(outcomes).enumerate() {
            if printed != expected {
                failures.push(format!(
                    "{name} run {run}: rustc {printed} != interpreter {expected}"
                ));
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    RustcRun {
        crates: built.len(),
        runs,
        entered,
    }
}

/// The fully qualified declarations a library module states at top level,
/// as its namespaces qualify them: what `library.toml` must list, one row
/// each, so that no theorem of the library escapes the axiom audit.
#[must_use]
pub fn library_declarations(text: &str) -> Vec<String> {
    let mut namespaces: Vec<String> = Vec::new();
    let mut out = Vec::new();
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("namespace ") {
            namespaces.push(rest.trim().to_owned());
            continue;
        }
        if let Some(rest) = line.strip_prefix("end ") {
            if namespaces.last().is_some_and(|last| last == rest.trim()) {
                namespaces.pop();
                continue;
            }
        }
        let mut rest = line;
        if let Some(after) = rest.strip_prefix("@[") {
            match after.find(']') {
                Some(close) => rest = after[close + 1..].trim_start(),
                None => continue,
            }
        }
        loop {
            let stripped = ["public ", "private ", "protected "]
                .iter()
                .find_map(|modifier| rest.strip_prefix(modifier));
            match stripped {
                Some(after) => rest = after,
                None => break,
            }
        }
        for keyword in ["theorem ", "def ", "structure ", "inductive ", "abbrev "] {
            if let Some(after) = rest.strip_prefix(keyword) {
                let name: String = after
                    .chars()
                    .take_while(|c| !c.is_whitespace() && !matches!(c, '(' | '{' | ':'))
                    .collect();
                if !name.is_empty() {
                    let mut qualified = namespaces.clone();
                    qualified.push(name);
                    out.push(qualified.join("."));
                }
                break;
            }
        }
    }
    out
}
