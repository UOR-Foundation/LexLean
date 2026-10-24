//! The migration of a language-1.1 project to language 1.2 (SPEC.md §17.12).
//!
//! Language 1.2 refuses the data form of a module (`\semanticdata{...}`,
//! `AU-13`), so the migration is not a change of the `language` row alone: each
//! module is rewritten into the authoring notation through
//! [`crate::sdk::to_notation`], the builtin glossary references of every source
//! and every path lexicon package move from `1.1.0` to `1.2.0`, and the lock is
//! regenerated as `lexlean/lock/2`.
//!
//! Nothing is written until the migrated project has been proven: every new
//! file is first written into a scratch copy of the project, which is locked
//! and checked as a language-1.2 project, so a module the notation cannot
//! express, a source language 1.2 does not link, and a package it does not
//! resolve are all refused before the first byte of the real project changes.
//!
//! The proof reads the project without the project mutation lock, so the files
//! it read are fingerprinted; under the lock each one is read again and the
//! migration is refused, writing nothing, if any has changed since, so an
//! editor save or a `fmt` landing in between is never overwritten by text
//! derived from the file it replaced.

use std::collections::BTreeMap;
use std::io::Write;

use camino::{Utf8Path, Utf8PathBuf};
use serde::Deserialize;
use sha2::{Digest, Sha256};

use super::{acquire_lock, host_failure, CheckRequest, Engine, LockRequest};
use crate::code;
use crate::config::{toml_string, LexiconSource};
use crate::diagnostic::{Diagnostic, Span};
use crate::error::LexLeanError;
use crate::project::Selection;
use crate::sdk::{self, ModuleKind};

/// The schema tag of a language-1.1 semantic module and the one language 1.2
/// reads in its place.
const MODULE_1_1: &str = "lexlean/semantic-module/1";
const MODULE_1_2: &str = "lexlean/semantic-module/2";

/// The version of every builtin package of each language.
const BUILTIN_1_1: &str = "@1.1.0";
const BUILTIN_1_2: &str = "@1.2.0";

/// A migration request.
pub struct MigrateRequest {
    /// Prove the migration and report it without writing anything.
    pub check_only: bool,
}

/// What a migration did, or with `check_only` would do.
pub struct MigrateResult {
    /// The modules rewritten, by name.
    pub modules: Vec<String>,
    /// The path lexicon manifests moved to language 1.2, project-relative.
    pub manifests: Vec<String>,
    /// Whether the project's files were rewritten.
    pub written: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    spec: String,
    package: String,
    version: String,
    language: String,
    imports: Vec<String>,
}

fn unsupported(path: &str, message: impl Into<String>) -> Diagnostic {
    Diagnostic::new(code!("LLC0103"), message).with_span(Span::whole_file(path))
}

fn unwritable(path: &str, message: impl Into<String>) -> Diagnostic {
    Diagnostic::new(code!("LLP2003"), message)
        .with_span(Span::whole_file(path))
        .with_help("the module is not migrated; no file of the project was written")
}

/// `package@1.1.0` of a builtin package as `package@1.2.0`; any other
/// reference is a path or Git package, whose version does not move with the
/// language, and is unchanged.
fn retarget(reference: &str) -> String {
    match reference.strip_suffix(BUILTIN_1_1) {
        Some(package) if package.starts_with("lexlean.") => format!("{package}{BUILTIN_1_2}"),
        _ => reference.to_owned(),
    }
}

/// The source of `text` with the builtin glossaries of its header moved to
/// 1.2.0. Only `\useglossary{...}` lines are read, so a module's data or
/// prose that happens to spell a version is never touched.
fn retarget_glossaries(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for line in text.split_inclusive('\n') {
        let body = line.trim_end_matches('\n');
        let moved = body
            .strip_prefix("\\useglossary{")
            .and_then(|rest| rest.strip_suffix('}'))
            .map(|reference| format!("\\useglossary{{{}}}", retarget(reference)));
        match moved {
            Some(rewritten) => {
                out.push_str(&rewritten);
                out.push_str(&line[body.len()..]);
            }
            None => out.push_str(line),
        }
    }
    out
}

/// One module's source in the notation, or why it has none.
fn migrated_source(relative: &str, text: &str) -> Result<String, Diagnostic> {
    match sdk::module_kind(text) {
        Some(ModuleKind::Semantic) => {}
        Some(other) => {
            return Err(unwritable(
                relative,
                format!(
                    "`{relative}` holds a {other:?} module; only semantic modules are migrated"
                ),
            ));
        }
        None => {
            return Err(unwritable(
                relative,
                format!("`{relative}` states no semantic module"),
            ));
        }
    }
    if sdk::is_notation(text) {
        return Err(unsupported(
            relative,
            format!(
                "`{relative}` is written in the authoring notation, which language 1.1 does not read; \
                 only a data-form (`\\semanticdata`) module is migrated"
            ),
        ));
    }
    let mut module =
        sdk::read_module(text).map_err(|failure| unwritable(relative, failure.to_string()))?;
    // Language 1.1 `check` refuses a data form that is not its own canonical
    // JSON (`LLT4001`); the migration refuses it too, rather than canonicalizing
    // a module the source language does not accept.
    if sdk::data_is_canonical(text) != Some(true) {
        return Err(unwritable(
            relative,
            format!(
                "`{relative}` is not a language-1.1 data-form module: its data is not canonical JSON, \
                 which language 1.1 refuses as well (`LLT4001`); run `lexlean fmt` on the 1.1 project first"
            ),
        ));
    }
    if module.get("spec").and_then(serde_json::Value::as_str) != Some(MODULE_1_1) {
        return Err(unwritable(
            relative,
            format!("`{relative}` is not a `{MODULE_1_1}` module, the schema language 1.1 reads"),
        ));
    }
    module["spec"] = serde_json::Value::String(MODULE_1_2.to_owned());
    // Neither conversion has a position to report: the offset of a printing
    // failure is the module's, not a place in the source.
    let data = sdk::write_module(text, &module)
        .map_err(|failure| unwritable(relative, failure.message))?;
    let notation =
        sdk::to_notation(&data).map_err(|failure| unwritable(relative, failure.message))?;
    // The notation is the module only if it reads back to it: a printing that
    // loses a member would otherwise migrate to a different module.
    let reread =
        sdk::read_module(&notation).map_err(|failure| unwritable(relative, failure.to_string()))?;
    if reread != module {
        return Err(unwritable(
            relative,
            format!("the notation of `{relative}` does not read back to the module"),
        ));
    }
    Ok(retarget_glossaries(&notation))
}

/// A path lexicon manifest moved to language 1.2, in the canonical layout.
fn migrated_manifest(relative: &str, text: &str) -> Result<String, Diagnostic> {
    let manifest: Manifest = toml::from_str(text)
        .map_err(|failure| unsupported(relative, format!("invalid lexicon manifest: {failure}")))?;
    if manifest.spec != "lexlean/lexicon/1" || manifest.language != "1.1" {
        return Err(unsupported(
            relative,
            format!(
                "`{relative}` is a `{}` manifest of language `{}`; only a language-1.1 manifest is migrated",
                manifest.spec, manifest.language
            ),
        ));
    }
    let imports: Vec<String> = manifest
        .imports
        .iter()
        .map(|import| toml_string(&retarget(import)))
        .collect();
    Ok(format!(
        "spec = {}\npackage = {}\nversion = {}\nlanguage = \"1.2\"\nimports = [{}]\n",
        toml_string(&manifest.spec),
        toml_string(&manifest.package),
        toml_string(&manifest.version),
        imports.join(", ")
    ))
}

/// Every file the migration rewrites, project-relative, with its new bytes.
type Rewrites = BTreeMap<String, Vec<u8>>;

/// The SHA-256 of every file the rewrites were derived from, by
/// project-relative path: exactly the bytes the proof read.
type Inputs = BTreeMap<String, [u8; 32]>;

/// What the proof is of: the new files, the bytes they were derived from, and
/// the names of the modules and manifests among them.
struct Plan {
    files: Rewrites,
    inputs: Inputs,
    modules: Vec<String>,
    manifests: Vec<String>,
}

fn fingerprint(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}

/// The raw bytes of a project file and the text the migration reads from them.
fn read_input(engine: &Engine, relative: &str) -> Result<(String, [u8; 32]), LexLeanError> {
    let path = engine
        .project()
        .confined_file(relative)
        .map_err(LexLeanError::from_diagnostic)?;
    let bytes = std::fs::read(path.as_std_path())
        .map_err(|io_error| host_failure(format!("{relative}: {io_error}")))?;
    let text = crate::source::normalize::normalize(relative, &bytes, true)
        .map(|normalized| normalized.text)
        .map_err(LexLeanError::from_diagnostics)?;
    Ok((text, fingerprint(&bytes)))
}

/// The new bytes of every module source, manifest, and the configuration, with
/// the fingerprints of what they were derived from.
fn rewrites(engine: &Engine) -> Result<Plan, LexLeanError> {
    let project = engine.project();
    if project.config.language != "1.1" {
        return Err(LexLeanError::from_diagnostic(unsupported(
            &project.config_name,
            format!(
                "language `{}` is not migrated to 1.2; only a language-1.1 project is",
                project.config.language
            ),
        )));
    }
    let mut files = Rewrites::new();
    let mut inputs = Inputs::new();
    let mut failures = Vec::new();
    let mut modules = Vec::new();
    let sources = project
        .all_modules()
        .map_err(LexLeanError::from_diagnostics)?;
    for (module, relative) in &sources {
        let (text, digest) = read_input(engine, relative)?;
        inputs.insert(relative.clone(), digest);
        match migrated_source(relative, &text) {
            Ok(text) => {
                files.insert(relative.clone(), text.into_bytes());
                modules.push(module.clone());
            }
            Err(diagnostic) => failures.push(diagnostic),
        }
    }
    let mut manifests = Vec::new();
    for source in &project.config.lexicon_sources {
        if let LexiconSource::Path { path, .. } = source {
            let relative = format!("{path}/lexicon.toml");
            let (text, digest) = read_input(engine, &relative)?;
            inputs.insert(relative.clone(), digest);
            match migrated_manifest(&relative, &text) {
                Ok(text) => {
                    files.insert(relative.clone(), text.into_bytes());
                    manifests.push(relative);
                }
                Err(diagnostic) => failures.push(diagnostic),
            }
        }
    }
    if !failures.is_empty() {
        return Err(LexLeanError::from_diagnostics(failures));
    }
    let mut config = project.config.clone();
    config.language = "1.2".to_owned();
    let (_, digest) = read_input(engine, &project.config_name)?;
    inputs.insert(project.config_name.clone(), digest);
    files.insert(
        project.config_name.clone(),
        config.canonical_toml().into_bytes(),
    );
    Ok(Plan {
        files,
        inputs,
        modules,
        manifests,
    })
}

/// Write `files` into the scratch copy, lock it, and check it as a
/// language-1.2 project. Returns the lock the real project is given.
fn prove(engine: &Engine, files: &Rewrites) -> Result<Vec<u8>, LexLeanError> {
    let staging = tempfile::Builder::new()
        .prefix("lexlean-migrate-")
        .tempdir()
        .map_err(|io_error| host_failure(format!("migrate staging: {io_error}")))?;
    let scratch = crate::project::utf8_path(staging.path())
        .map(|path| path.join("project"))
        .map_err(LexLeanError::from_diagnostic)?;
    engine.scratch_project_copy(&scratch)?;
    for (relative, bytes) in files {
        std::fs::write(scratch.join(relative).as_std_path(), bytes)
            .map_err(|io_error| host_failure(format!("migrate staging {relative}: {io_error}")))?;
    }
    let migrated = Engine::load(&scratch.join(&engine.project().config_name))?;
    let lock = migrated.lock(LockRequest {
        check_only: false,
        allow_network: false,
    })?;
    migrated.check(CheckRequest {
        selection: Selection::All,
    })?;
    Ok(lock.bytes)
}

/// One staged replacement of a project file: the new bytes already written
/// beside the target, and the bytes it replaces for a rollback.
struct Staged {
    target: Utf8PathBuf,
    shown: String,
    temp: tempfile::NamedTempFile,
    original: Option<Vec<u8>>,
}

/// Stage `bytes` beside `relative`. Under the project mutation lock, the file
/// as it is now must be the one the proof read (`proven`, when the migration
/// derived the new bytes from it), or the migration is refused.
fn stage(
    engine: &Engine,
    relative: &str,
    bytes: &[u8],
    proven: Option<&[u8; 32]>,
) -> Result<Staged, LexLeanError> {
    let project = engine.project();
    let target = project
        .confined_creatable(relative)
        .map_err(LexLeanError::from_diagnostic)?;
    let parent = target
        .parent()
        .map_or_else(|| project.root.clone(), Utf8Path::to_path_buf);
    let original = match std::fs::read(target.as_std_path()) {
        Ok(existing) => Some(existing),
        Err(io_error) if io_error.kind() == std::io::ErrorKind::NotFound => None,
        Err(io_error) => return Err(host_failure(format!("{relative}: {io_error}"))),
    };
    if let Some(proven) = proven {
        if original.as_deref().map(fingerprint).as_ref() != Some(proven) {
            return Err(host_failure(format!(
                "{relative}: changed since the migration was proven; nothing was written, run the migration again"
            )));
        }
    }
    let mut temp = tempfile::NamedTempFile::new_in(parent.as_std_path())
        .map_err(|io_error| host_failure(format!("{relative}: staging: {io_error}")))?;
    temp.write_all(bytes)
        .and_then(|()| temp.as_file().sync_all())
        .map_err(|io_error| host_failure(format!("{relative}: staging: {io_error}")))?;
    if let Ok(metadata) = std::fs::metadata(target.as_std_path()) {
        // The permissions are the file's own; failing to carry them leaves the
        // temporary's ordinary ones, which the rename then publishes.
        let _ = std::fs::set_permissions(temp.path(), metadata.permissions());
    }
    Ok(Staged {
        target,
        shown: relative.to_owned(),
        temp,
        original,
    })
}

/// Restore every file of `published`, last published first, and return the
/// failures: a restore that fails does not stop the others, so every file that
/// can be returned to what it was is, and every one that cannot is named.
fn rollback(published: Vec<(Utf8PathBuf, Option<Vec<u8>>)>) -> Vec<String> {
    published
        .into_iter()
        .rev()
        .filter_map(|(path, before)| {
            let restored = match before {
                Some(bytes) => std::fs::write(path.as_std_path(), bytes),
                None => std::fs::remove_file(path.as_std_path()),
            };
            restored.err().map(|io_error| format!("{path}: {io_error}"))
        })
        .collect()
}

/// Publish every staged file; when one cannot be, restore the ones already
/// published, so the project is not left half migrated. Every restore is
/// attempted, and the files that could not be restored are reported together
/// with the failure that made the restore necessary.
fn publish(staged: Vec<Staged>) -> Result<(), LexLeanError> {
    let mut published: Vec<(Utf8PathBuf, Option<Vec<u8>>)> = Vec::new();
    for entry in staged {
        let Staged {
            target,
            shown,
            temp,
            original,
        } = entry;
        match temp.persist(target.as_std_path()) {
            Ok(_) => published.push((target, original)),
            Err(persist_error) => {
                let stranded = rollback(published);
                if stranded.is_empty() {
                    return Err(host_failure(format!(
                        "{shown}: publish: {}",
                        persist_error.error
                    )));
                }
                return Err(host_failure(format!(
                    "{shown}: publish failed ({}) and {} published file(s) could not be restored, \
                     so the project is partly migrated: {}",
                    persist_error.error,
                    stranded.len(),
                    stranded.join("; ")
                )));
            }
        }
    }
    Ok(())
}

/// Migrate the language-1.1 project of `engine` to language 1.2 (§17.12).
///
/// # Errors
///
/// A project that is not language 1.1 (`LLC0103`), a module or manifest that
/// cannot be migrated (`LLP2003`, `LLC0103`), and any failure of the migrated
/// project to lock and check as language 1.2, each with its own diagnostic.
/// On every error no file of the project has been written.
pub fn migrate(engine: &Engine, request: &MigrateRequest) -> Result<MigrateResult, LexLeanError> {
    engine.bound("migrate", migrate_inner(engine, request))
}

fn migrate_inner(engine: &Engine, request: &MigrateRequest) -> Result<MigrateResult, LexLeanError> {
    let mut plan = rewrites(engine)?;
    let lock = prove(engine, &plan.files)?;
    if request.check_only {
        return Ok(MigrateResult {
            modules: plan.modules,
            manifests: plan.manifests,
            written: false,
        });
    }
    plan.files
        .insert(engine.project().config.lockfile.clone(), lock);
    write(engine, &plan.files, &plan.inputs)?;
    Ok(MigrateResult {
        modules: plan.modules,
        manifests: plan.manifests,
        written: true,
    })
}

/// Take the project mutation lock, check that every file the proof read is
/// unchanged, and publish the new files together.
///
/// The lock lives in the build root, which taking it creates when it is
/// absent. A refusal or failure after that point removes the build root again
/// when this call created it, so a refused migration leaves nothing behind
/// (SPEC.md §23.4).
fn write(engine: &Engine, files: &Rewrites, inputs: &Inputs) -> Result<(), LexLeanError> {
    let project = engine.project();
    let build_root = project
        .confined_creatable(&project.config.build_root)
        .map_err(LexLeanError::from_diagnostic)?;
    let created_build_root = !build_root.as_std_path().exists();
    let guard = acquire_lock(project)?;
    let outcome = stage_all(engine, files, inputs).and_then(publish);
    drop(guard);
    match outcome {
        Ok(()) => Ok(()),
        Err(failure) if created_build_root => match remove_build_root(&build_root) {
            Ok(()) => Err(failure),
            Err(io_error) => Err(host_failure(format!(
                "{failure}; the build root `{build_root}` this migration created could not be removed: {io_error}"
            ))),
        },
        Err(failure) => Err(failure),
    }
}

/// Stage every file, refusing the migration when one is not the file the
/// proof read.
fn stage_all(
    engine: &Engine,
    files: &Rewrites,
    inputs: &Inputs,
) -> Result<Vec<Staged>, LexLeanError> {
    let mut staged = Vec::with_capacity(files.len());
    for (relative, bytes) in files {
        staged.push(stage(engine, relative, bytes, inputs.get(relative))?);
    }
    Ok(staged)
}

/// Remove the build root a migration created: the lock file it took, then the
/// directory, which stays when anything else has since been put in it.
fn remove_build_root(build_root: &Utf8Path) -> std::io::Result<()> {
    match std::fs::remove_file(build_root.join(".lock").as_std_path()) {
        Err(io_error) if io_error.kind() != std::io::ErrorKind::NotFound => return Err(io_error),
        _ => {}
    }
    std::fs::remove_dir(build_root.as_std_path())
}

#[cfg(test)]
mod tests;
