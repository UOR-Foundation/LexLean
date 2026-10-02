//! Certificate A end to end (SPEC.md §17.17): lower every production root of
//! a project, generate its certificate, stage the shipped preservation
//! environment beside the project's generated modules, compile every module
//! with the pinned Lean, replay every certificate through `leanchecker`, and
//! audit the exact axioms of the library and of every root theorem.

use std::path::{Path, PathBuf};
use std::process::Command;

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

/// Certify every production root of the example `name` end to end.
///
/// # Panics
///
/// Panics naming the first module that fails to compile or replay, or the
/// first axiom disagreement.
#[must_use]
pub fn certify_example(name: &str) -> Vec<Certified> {
    let project = P::copy_example(name);
    let certified = certificates(&project);
    let staged = staged(&project, &certified);
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
    certified
}
