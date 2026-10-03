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
    roots(&checked)
        .expect("the eligibility reports")
        .iter()
        .enumerate()
        .map(|(index, root)| {
            let lowered = lower_root(&modules, &root.module, &root.name, root.report)
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
                    let certificate_b = certificate_b(&lowered.program, &krate, &module_b)
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
                program: lowered.program,
                renderings,
                composed,
            }
        })
        .collect()
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
        assert!(!wave.is_empty(), "the certificates import one another in a cycle");
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
}

/// A defect planted in a certificate E's statement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompositionMutation {
    /// The root's Rust function is claimed with the other result shape:
    /// `R<T>` for a plain function, or a plain result for a fallible one.
    Fallibility,
    /// The rendering of another function is claimed to realize the root.
    Function,
}

impl CompositionMutation {
    /// Every mutation.
    pub const ALL: [Self; 2] = [Self::Fallibility, Self::Function];

    /// `text` with this defect planted, or `None` when it does not apply.
    #[must_use]
    pub fn plant(self, text: &str) -> Option<String> {
        let (from, to) = match self {
            Self::Fallibility if text.contains("Rust.RealizesFn true ") => {
                ("Rust.RealizesFn true ", "Rust.RealizesFn false ")
            }
            Self::Fallibility => ("Rust.RealizesFn false ", "Rust.RealizesFn true "),
            Self::Function => ("Rust.fnIdent 0)", "Rust.fnIdent 1)"),
        };
        text.contains(from).then(|| text.replacen(from, to, 1))
    }
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
    /// Whether the pinned Lean accepted the planted module.
    pub accepted: bool,
    /// The pinned Lean's output.
    pub output: String,
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
    let composed_plants = plant_composed(&certified, scratch.path());
    let cases = crate::differential::cases(project);
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
    }
}

/// Restate every root's first certificate E with each planted defect and
/// compile it beside the checked workspace below `root`.
fn plant_composed(certified: &[Certified], root: &Path) -> Vec<ComposedPlant> {
    let mut jobs = Vec::new();
    for entry in certified {
        let Some((target, composed)) = entry.composed.first() else {
            continue;
        };
        for mutation in CompositionMutation::ALL {
            let text = mutation
                .plant(&composed.text)
                .unwrap_or_else(|| panic!("{mutation:?} applies to `{}`", composed.module));
            jobs.push((entry.root.clone(), target.clone(), mutation, text));
        }
    }
    let directory = root.join("src/LexLeanPreserve/Planted");
    std::fs::create_dir_all(&directory).expect("mkdir");
    parallel(&jobs, |(root_name, target, mutation, text)| {
        let index = jobs
            .iter()
            .position(|job| job.0 == *root_name && job.1 == *target && job.2 == *mutation)
            .expect("the job");
        let path = directory.join(format!("E{index}.lean"));
        std::fs::write(&path, text).expect("write");
        let output = lean(root, &[path.display().to_string()]);
        ComposedPlant {
            root: root_name.clone(),
            target: target.clone(),
            mutation: *mutation,
            accepted: output.status.success(),
            output: joined(&output),
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
    /// The entry calls the root without its first validator's check
    /// (§17.17): an invariant is no longer checked at the boundary.
    Validation,
    /// The last order-checking validator admits an equal successor: a map
    /// with a repeated key, or a set with a repeated element, passes.
    Weakening,
}

impl Mutation {
    /// The mutations of the lowered closure.
    pub const LOWERING: [Self; 5] = [
        Self::Branches,
        Self::Arithmetic,
        Self::Constructor,
        Self::Recursion,
        Self::Literal,
    ];

    /// The mutations of the boundary.
    pub const BOUNDARY: [Self; 2] = [Self::Validation, Self::Weakening];
}

/// Whether `expr` is a validator's key comparison: `less_than` as the
/// templates write it, `true` exactly when the order says `lt`.
fn key_comparison(arms: &[lexlean::calculus::Arm]) -> bool {
    use lexlean::calculus::Shape;
    let constant = |expr: &Expr, wanted: Shape| {
        matches!(expr, Expr::Build { shape, operands, .. } if *shape == wanted && operands.is_empty())
    };
    arms.len() == 3
        && arms[0].shape == Shape::Lt
        && constant(&arms[0].body, Shape::True)
        && arms[1].shape == Shape::Eq
        && constant(&arms[1].body, Shape::False)
}

fn mutate_expr(expr: &mut Expr, mutation: Mutation, program: &Program) -> bool {
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
            && matches!(**else_branch, Expr::Build { shape: Shape::None, .. }) =>
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
        ) if matches!(**scrutinee, Expr::Prim { operation: Prim::Compare, .. })
            && key_comparison(arms) =>
        {
            if let Expr::Build { shape, .. } = &mut arms[1].body {
                *shape = Shape::True;
            }
            return true;
        }
        _ => {}
    }
    let here = match (mutation, &mut *expr) {
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
    };
    if here {
        return true;
    }
    match expr {
        Expr::Value { .. } | Expr::Var { .. } => false,
        Expr::Let { bound, body, .. } => {
            mutate_expr(bound, mutation, program) || mutate_expr(body, mutation, program)
        }
        Expr::Cond {
            condition,
            then_branch,
            else_branch,
        } => {
            mutate_expr(condition, mutation, program)
                || mutate_expr(then_branch, mutation, program)
                || mutate_expr(else_branch, mutation, program)
        }
        Expr::Match {
            scrutinee, arms, ..
        } => {
            mutate_expr(scrutinee, mutation, program)
                || arms
                    .iter_mut()
                    .any(|arm| mutate_expr(&mut arm.body, mutation, program))
        }
        Expr::Build { operands, .. }
        | Expr::Call { operands, .. }
        | Expr::Prim { operands, .. }
        | Expr::Closure {
            captures: operands, ..
        } => operands
            .iter_mut()
            .any(|operand| mutate_expr(operand, mutation, program)),
        Expr::Apply { target, operands } => {
            mutate_expr(target, mutation, program)
                || operands
                    .iter_mut()
                    .any(|operand| mutate_expr(operand, mutation, program))
        }
        Expr::First { value } | Expr::Second { value } | Expr::Field { value, .. } => {
            mutate_expr(value, mutation, program)
        }
    }
}

/// The lowered program with `mutation` planted at its first applicable
/// site, if any: a boundary mutation in the entry or a validator, from the
/// last back; any other in the closure, from the root on.
#[must_use]
pub fn mutate(lowered: &lexlean::production::lower::Lowered, mutation: Mutation) -> Option<Program> {
    use lexlean::production::lower::Origin;
    let program = &lowered.program;
    let mut mutated = program.clone();
    let boundary = Mutation::BOUNDARY.contains(&mutation);
    let count = mutated.functions.len();
    for position in 0..count {
        let index = if boundary { count - 1 - position } else { position };
        let at_boundary = matches!(
            lowered.layout.functions[index],
            Origin::Validator { .. } | Origin::Entry { .. }
        );
        if at_boundary != boundary {
            continue;
        }
        if mutate_expr(&mut mutated.functions[index].body, mutation, program) {
            return Some(mutated);
        }
    }
    None
}

/// One planted mutation and what Lean made of its certificate.
#[derive(Debug, Clone)]
pub struct Planted {
    /// The root whose program was mutated.
    pub root: String,
    /// The mutation.
    pub mutation: Mutation,
    /// Lean's output on the mutated certificate; empty when it was
    /// accepted, which is a failure of the certificate.
    pub rejection: String,
}

/// Plant each of `mutations` in the first root of `project` whose program
/// admits it, regenerate that root's certificate against the mutated
/// program, and compile it.
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
    let mut out = Vec::new();
    for mutation in mutations.iter().copied() {
        for root in &roots {
            let mut lowered = lower_root(&modules, &root.module, &root.name, root.report)
                .expect("an eligible root lowers");
            let Some(program) = mutate(&lowered, mutation) else {
                continue;
            };
            lowered.program = program;
            let module = format!("LexLeanPreserve.Planted.{mutation:?}");
            let certificate = certificate(
                &modules,
                &root.module,
                &root.name,
                root.report,
                &lowered,
                &module,
            )
            .expect("the certificate is generated from the source");
            let path = scratch
                .path()
                .join("src")
                .join(lexlean::production::preserve::module_path(&module));
            std::fs::create_dir_all(path.parent().expect("a parent")).expect("mkdir");
            std::fs::write(&path, &certificate.text).expect("write");
            let output = lean(scratch.path(), &[path.display().to_string()]);
            out.push(Planted {
                root: root.report.root.clone(),
                mutation,
                rejection: if output.status.success() {
                    String::new()
                } else {
                    format!(
                        "{}{}",
                        String::from_utf8_lossy(&output.stdout),
                        String::from_utf8_lossy(&output.stderr)
                    )
                },
            });
            break;
        }
    }
    out
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
}

impl RustMutation {
    /// Every mutation.
    pub const ALL: [Self; 9] = [
        Self::Branches,
        Self::Arithmetic,
        Self::Overflow,
        Self::Width,
        Self::Sequence,
        Self::Constructor,
        Self::Buffer,
        Self::Recursion,
        Self::Literal,
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

fn mutate_rust(
    expr: &mut lexlean::calculus::rust::ast::Expr,
    mutation: RustMutation,
    variants: &dyn Fn(u64) -> u64,
    functions: u64,
) -> bool {
    use lexlean::calculus::rust::ast::{Callee, Ctor, Expr as R, Lit};
    use lexlean::calculus::rust::runtime::Item;
    let here = match (mutation, &mut *expr) {
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
            *value += 1;
            true
        }
        _ => false,
    };
    here || rust_children(expr)
        .into_iter()
        .any(|child| mutate_rust(child, mutation, variants, functions))
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

/// `krate` with `mutation` planted at its first applicable site, if any.
#[must_use]
pub fn mutate_crate(
    krate: &lexlean::calculus::rust::ast::Crate,
    mutation: RustMutation,
) -> Option<lexlean::calculus::rust::ast::Crate> {
    use lexlean::calculus::rust::ast::{ItemDef, Type};
    let variants = |adt: u64| {
        krate
            .items
            .iter()
            .find_map(|item| match item {
                ItemDef::Enum { name, variants, .. } if *name == Type::Adt(adt) => {
                    Some(variants.len() as u64)
                }
                _ => None,
            })
            .unwrap_or(0)
    };
    let functions = krate
        .items
        .iter()
        .filter(|item| matches!(item, ItemDef::Function { .. }))
        .count() as u64;
    let mut mutated = krate.clone();
    for item in &mut mutated.items {
        if let ItemDef::Function { body, .. } = item {
            let planted = body
                .lets
                .iter_mut()
                .map(|binding| &mut binding.value)
                .chain([&mut body.tail])
                .any(|expr| mutate_rust(expr, mutation, &variants, functions));
            if planted {
                return Some(mutated);
            }
        }
    }
    None
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
            let certificate =
                certificate_b(&case.fixture.program, &krate, &module).unwrap_or_else(|reason| {
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
    /// The aligner's reason when it derives nothing for the mutated crate.
    pub unaligned: Option<String>,
    /// Lean's output on the derivation the aligner wrote for the mutated
    /// crate, when it wrote one; empty when Lean accepted it.
    pub realigned: Option<String>,
    /// Lean's output on the unmutated derivation restated over the mutated
    /// crate; empty when Lean accepted it.
    pub stale: String,
}

/// Plant every mutation in the first rendering that admits it: ask the
/// aligner for the mutated crate's certificate, and restate the
/// unmutated certificate over the mutated crate.
#[must_use]
pub fn plant_renderings(renderings: &[Rendering]) -> Vec<(RustPlanted, Vec<(String, String)>)> {
    let mut out = Vec::new();
    for mutation in RustMutation::ALL {
        for rendering in renderings {
            let Some(mutated) = mutate_crate(&rendering.krate, mutation) else {
                continue;
            };
            let base = format!("LexLeanPreserve.Planted.{mutation:?}");
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
            let unaligned = match certificate_b(&rendering.program, &mutated, &realigned_module) {
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
                    unaligned,
                    realigned: None,
                    stale: String::new(),
                },
                modules,
            ));
            break;
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
    for ((index, module, _), verdict) in jobs.iter().zip(verdicts) {
        if module.ends_with(".Stale") {
            planted[*index].stale = verdict;
        } else {
            planted[*index].realigned = Some(verdict);
        }
    }
    RenderingsChecked {
        audit_output: checked.audit_output,
        planted,
    }
}
