//! Certificate A end to end (SPEC.md §17.17): lower every production root of
//! a project, generate its certificate, stage the shipped preservation
//! environment beside the project's generated modules, compile every module
//! with the pinned Lean, replay every certificate through `leanchecker`, and
//! audit the exact axioms of the library and of every root theorem.

use std::path::{Path, PathBuf};
use std::process::Command;

use lexlean::calculus::{Expr, Program, Ty, Value};
use lexlean::production::certificate::{certificate, Certificate};
use lexlean::production::lower::{linked_modules, lower_root, roots};
use lexlean::production::preserve::{audit, workspace, Workspace};

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
            Certified {
                root: root.report.root.clone(),
                targets: root
                    .report
                    .targets
                    .iter()
                    .map(|row| row.target.clone())
                    .collect(),
                certificate,
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
    let certificates: Vec<Certificate> = certified
        .iter()
        .map(|entry| entry.certificate.clone())
        .collect();
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

fn lean(root: &Path, arguments: &[String]) -> std::process::Output {
    Command::new(toolchain_bin().join("lean"))
        .args(arguments)
        .current_dir(root)
        .env("LEAN_PATH", root.join("build"))
        .output()
        .expect("lean runs")
}

/// Write `workspace` below `root`, compile every module in order, replay
/// every certificate, and compile the audit module.
#[must_use]
pub fn check(workspace: &Workspace, root: &Path) -> Checked {
    for file in workspace.files.iter().chain([&workspace.audit]) {
        let path = root.join("src").join(&file.path);
        std::fs::create_dir_all(path.parent().expect("a parent")).expect("mkdir");
        std::fs::write(&path, &file.text).expect("write");
    }
    for file in &workspace.files {
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
        if !output.status.success() {
            return Checked {
                failure: Some((
                    file.module.clone(),
                    format!(
                        "{}{}",
                        String::from_utf8_lossy(&output.stdout),
                        String::from_utf8_lossy(&output.stderr)
                    ),
                )),
                audit_output: String::new(),
            };
        }
    }
    for module in &workspace.certificates {
        let output = Command::new(toolchain_bin().join("leanchecker"))
            .arg(module)
            .current_dir(root)
            .env("LEAN_PATH", root.join("build"))
            .output()
            .expect("leanchecker runs");
        if !output.status.success() {
            return Checked {
                failure: Some((
                    module.clone(),
                    format!(
                        "leanchecker: {}{}",
                        String::from_utf8_lossy(&output.stdout),
                        String::from_utf8_lossy(&output.stderr)
                    ),
                )),
                audit_output: String::new(),
            };
        }
    }
    let audit_source = root.join("src").join(&workspace.audit.path);
    let output = lean(root, &[audit_source.display().to_string()]);
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
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
    let certificates: Vec<Certificate> = certified
        .iter()
        .map(|entry| entry.certificate.clone())
        .collect();
    audit(&checked.audit_output, &certificates)
        .unwrap_or_else(|reason| panic!("{name}: axiom audit: {reason}"));
    let cases = crate::differential::cases(project);
    let denotes: Vec<(String, String, Vec<crate::differential::Case>)> = certified
        .iter()
        .map(|entry| {
            (
                entry.certificate.module.clone(),
                entry.certificate.denote.clone(),
                cases.get(&entry.root).cloned().unwrap_or_default(),
            )
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
    Report {
        certified,
        differential,
    }
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
}

impl Mutation {
    /// Every mutation.
    pub const ALL: [Self; 5] = [
        Self::Branches,
        Self::Arithmetic,
        Self::Constructor,
        Self::Recursion,
        Self::Literal,
    ];
}

fn mutate_expr(expr: &mut Expr, mutation: Mutation, program: &Program) -> bool {
    use lexlean::calculus::{Prim, Shape};
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

/// `program` with `mutation` planted at its first applicable site, if any.
#[must_use]
pub fn mutate(program: &Program, mutation: Mutation) -> Option<Program> {
    let mut mutated = program.clone();
    let original = program.clone();
    for function in &mut mutated.functions {
        if mutate_expr(&mut function.body, mutation, &original) {
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

/// Plant every mutation in the first root of `project` whose program
/// admits it, regenerate that root's certificate against the mutated
/// program, and compile it.
///
/// # Panics
///
/// Panics when the unmutated environment does not compile.
#[must_use]
pub fn plant(project: &P) -> Vec<Planted> {
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
    for mutation in Mutation::ALL {
        for root in &roots {
            let mut lowered = lower_root(&modules, &root.module, &root.name, root.report)
                .expect("an eligible root lowers");
            let Some(program) = mutate(&lowered.program, mutation) else {
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
