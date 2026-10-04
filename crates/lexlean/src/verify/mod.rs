//! Verification (SPEC.md §22): the complete fixed pipeline, with no
//! optional stage and no suppression. Any failed stage removes the staging
//! tree and produces no verified artifact (I11).
//!
//! Fixed decisions this module makes where the specification leaves the
//! mechanics to the implementation:
//!
//! - **`leanchecker` identity.** The pinned `leanchecker` has no version
//!   flag. Its recorded, checked identity is the normalized output of the
//!   fixed identity probe `lake env <leanchecker> LexLeanIdentityProbe`
//!   (the preflighted executable by absolute path, on a module name no
//!   workspace defines), which the pinned toolchain
//!   answers with exactly one deterministic line and a nonzero exit; the
//!   executable digest is recorded alongside.
//! - **Verified-set reuse.** A verification whose attestation ID already
//!   has a published directory reuses it only after every staged file is
//!   byte-equal to the published one and neither side has extra files.
//! - **Process records.** Every child the pipeline runs (probe, module
//!   compilations, `leanchecker` replays, the audit, and both PDF provider
//!   processes per module) is recorded in the attestation and checked for
//!   unexpected absolute paths.

pub mod axiom;
pub mod child;
pub mod leanchecker;
pub mod source_audit;
pub mod toolchain;
pub mod workspace;

use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;

use camino::{Utf8Path, Utf8PathBuf};

use crate::api::{RenderedBuild, RenderedModule};
use crate::artifact::canonical_json::Json;
use crate::artifact::content_id::{attestation_id, Sha256Digest};
use crate::artifact::source_map::{MapRole, Mapping};
use crate::code;
use crate::diagnostic::{Diagnostic, Span};
use crate::error::LexLeanError;
use crate::ir::term::LocalId;
use crate::link::CheckedProject;
use crate::lock::Lock;
use crate::project::Project;
use crate::source::coverage::Origin;
use crate::verify::child::{run as run_child, ChildHome, ChildRecord, ChildSpec, Normalizer};
use crate::verify::toolchain::Toolchain;

/// The outcome of a successful verification.
pub struct VerifyOutcome {
    /// The attestation ID.
    pub attestation_id: Sha256Digest,
    /// The published verified directory.
    pub root: Utf8PathBuf,
}

fn fail(diagnostic: Diagnostic) -> LexLeanError {
    LexLeanError::from_diagnostic(diagnostic)
}

fn internal(message: impl std::fmt::Display) -> Diagnostic {
    Diagnostic::new(code!("LLI9001"), format!("phase verify: {message}"))
}

/// The prose-free generated-source audit (§18.2, LN-11) over one file.
pub fn generated_source_audit(text: &str, allow_print_axioms: bool) -> Result<(), String> {
    source_audit::audit(text, allow_print_axioms)
}

fn generated_core_source_audit(text: &str) -> Result<(), String> {
    source_audit::audit_core(text)
}

fn generated_semantic_source_audit(text: &str) -> Result<(), String> {
    source_audit::audit_semantic(text)
}

fn write_staged(root: &std::path::Path, relative: &str, bytes: &[u8]) -> Result<(), LexLeanError> {
    let destination = root.join(relative);
    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent).map_err(|io_error| {
            fail(Diagnostic::new(
                code!("LLB6003"),
                format!("staging {relative}: {io_error}"),
            ))
        })?;
    }
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&destination)
        .map_err(|io_error| {
            fail(Diagnostic::new(
                code!("LLB6003"),
                format!("staging {relative}: {io_error}"),
            ))
        })?;
    file.write_all(bytes)
        .and_then(|()| file.sync_all())
        .map_err(|io_error| {
            fail(Diagnostic::new(
                code!("LLB6003"),
                format!("staging {relative}: {io_error}"),
            ))
        })
}

/// One parsed Lean message location: `path:line:col: severity: message`
/// with its indented continuation lines. Lean 4.32.1 prints
/// `path:line:col[-line:col]: severity[(name)]: message`
/// (`Lean.mkErrorStringWithPos`): the end position appears only under
/// `printMessageEndPos`, and named errors carry their error name in
/// parentheses, as in `error(lean.unknownIdentifier)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeanMessage {
    /// The reported path as printed.
    pub path: String,
    /// One-based line.
    pub line: usize,
    /// Zero-based column in Unicode scalar values.
    pub column: usize,
    /// `error`, `warning`, or `info`.
    pub severity: String,
    /// The error name Lean printed in parentheses after the severity, when
    /// any (`lean.unknownIdentifier`).
    pub name: Option<String>,
    /// The message text with continuation lines joined by LF.
    pub message: String,
}

/// Parse `severity` or `severity(name)` into its two parts; only Lean's
/// severities open a message.
fn parse_severity(label: &str) -> Option<(String, Option<String>)> {
    let label = label.trim();
    let (severity, name) = match label.split_once('(') {
        Some((severity, rest)) => {
            let name = rest.strip_suffix(')')?;
            if name.is_empty() || name.chars().any(char::is_whitespace) {
                return None;
            }
            (severity, Some(name.to_owned()))
        }
        None => (label, None),
    };
    if !matches!(severity, "error" | "warning" | "info" | "information") {
        return None;
    }
    Some((severity.to_owned(), name))
}

/// Parse a Lean line/column pair.
fn parse_position(line: &str, column: &str) -> Option<(usize, usize)> {
    let line_number = line.trim().parse::<usize>().ok()?;
    let column = column.trim().parse::<usize>().ok()?;
    Some((line_number, column))
}

/// Parse one line as a message opener.
fn parse_message_opener(line: &str) -> Option<LeanMessage> {
    // The path may contain `:` on no supported host, so the split takes
    // the first three fields.
    let mut parts = line.splitn(4, ':');
    let path = parts.next()?;
    let line_field = parts.next()?;
    let column_field = parts.next()?;
    let rest = parts.next()?;
    match column_field.split_once('-') {
        // `line:col-endline:endcol: severity: message`: the column field
        // holds `col-endline` and the fourth field starts with `endcol:`.
        Some((column, end_line)) => {
            let (line_number, column) = parse_position(line_field, column)?;
            end_line.trim().parse::<usize>().ok()?;
            let (end_column, remainder) = rest.split_once(':')?;
            end_column.trim().parse::<usize>().ok()?;
            finish_opener(path, line_number, column, remainder)
        }
        None => {
            let (line_number, column) = parse_position(line_field, column_field)?;
            finish_opener(path, line_number, column, rest)
        }
    }
}

fn finish_opener(path: &str, line: usize, column: usize, rest: &str) -> Option<LeanMessage> {
    let rest = rest.trim_start();
    let (label, message) = rest.split_once(':')?;
    let (severity, name) = parse_severity(label)?;
    Some(LeanMessage {
        path: path.to_owned(),
        line,
        column,
        severity,
        name,
        message: message.trim().to_owned(),
    })
}

/// Parse every Lean message from combined process output (§20.4). Lines
/// that do not open a message continue the previous one.
#[must_use]
pub fn parse_lean_messages(output: &str) -> Vec<LeanMessage> {
    let mut messages: Vec<LeanMessage> = Vec::new();
    for line in output.lines() {
        match parse_message_opener(line) {
            Some(message) => messages.push(message),
            None => {
                if let Some(last) = messages.last_mut() {
                    if !line.trim().is_empty() {
                        last.message.push('\n');
                        last.message.push_str(line.trim_end());
                    }
                }
            }
        }
    }
    messages
}

/// The non-blank output lines preceding the first located message: text
/// no message accounts for (a wrapper's stray line, an unlocated warning).
#[must_use]
pub fn lean_output_preamble(output: &str) -> String {
    output
        .lines()
        .take_while(|line| parse_message_opener(line).is_none())
        .filter(|line| !line.trim().is_empty())
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n")
}

/// The byte offset of a Lean location in generated text: Lean columns count
/// Unicode scalar values (§20.1), so the column is converted per line.
#[must_use]
pub fn lean_position_to_byte(text: &str, line: usize, column: usize) -> Option<usize> {
    let mut offset = 0usize;
    for (index, text_line) in text.split('\n').enumerate() {
        if index + 1 == line {
            let within: usize = text_line.chars().take(column).map(char::len_utf8).sum();
            return Some(offset + within);
        }
        offset += text_line.len() + 1;
    }
    None
}

fn source_span(checked: &CheckedProject, module: &RenderedModule, range: (usize, usize)) -> Span {
    let path = module
        .map
        .sources
        .first()
        .map_or_else(String::new, |source| match source {
            crate::artifact::source_map::MapSource::File { path, .. } => path.clone(),
            _ => String::new(),
        });
    let position = |byte: usize| {
        checked
            .modules
            .get(&module.module)
            .map_or((1, 1), |checked_module| {
                let prefix =
                    &checked_module.normalized[..byte.min(checked_module.normalized.len())];
                let line = prefix.matches('\n').count() + 1;
                let column = prefix
                    .rsplit('\n')
                    .next()
                    .map_or(1, |tail| tail.chars().count() + 1);
                (line, column)
            })
    };
    let (line_start, column_start) = position(range.0);
    let (line_end, column_end) = position(range.1);
    Span {
        path,
        byte_start: range.0,
        byte_end: range.1,
        line_start,
        column_start,
        line_end,
        column_end,
    }
}

/// The source span of one generated declaration (§20.1: the primary
/// location is the thing that failed). The Lean backend emits a
/// declaration's name under the `declaration` role and its own reference
/// origin, mapped to the declaration's whole source range, so the first
/// coverage row carrying that origin under that role locates it. An
/// axiom-policy, replay, or audit failure is about that declaration, not
/// about the project manifest.
fn declaration_span(
    checked: &CheckedProject,
    module: &RenderedModule,
    document_module: &str,
    component: &str,
) -> Option<Span> {
    let origin = Origin::Reference {
        module: document_module.to_owned(),
        component: component.to_owned(),
    };
    module
        .coverage
        .lean
        .iter()
        .filter(|row| row.origin == origin)
        .find_map(|row| {
            let mapping = module.map.remap(0, row.byte_start)?;
            if mapping.role == MapRole::Declaration {
                mapping.src_range
            } else {
                None
            }
        })
        .map(|range| source_span(checked, module, range))
}

/// Anchor an axiom-audit rejection at the declaration it is about, when
/// the parser named one (§20.1). The parser reports the full Lean name
/// `<lean module>.<lean name>`; the declaration owning it is the one whose
/// generated module is the name's prefix.
fn audit_diagnostic(
    declaration_spans: &BTreeMap<String, Span>,
    failure: axiom::AuditFailure,
) -> Diagnostic {
    let Some(full_name) = failure.declaration else {
        return failure.diagnostic;
    };
    if let Some(span) = declaration_spans.get(&full_name) {
        return failure.diagnostic.with_span(span.clone());
    }
    failure.diagnostic
}

/// Release rendered data after its canonical bytes and generated sources
/// have been staged. None of these fields participates in Lean diagnostic
/// remapping, so retaining them would combine renderer and elaborator peaks.
fn release_non_diagnostic_buffers(build: &mut RenderedBuild) {
    build.files.clear();
    build.files.shrink_to_fit();
    for module in &mut build.modules {
        module.tex_text.clear();
        module.tex_text.shrink_to_fit();
        module.coverage.source.clear();
        module.coverage.source.shrink_to_fit();
        module.coverage.latex.clear();
        module.coverage.latex.shrink_to_fit();
        module.map.artifacts.clear();
        module.map.artifacts.shrink_to_fit();
        module.map.nodes.clear();
        module.map.nodes.shrink_to_fit();
    }
}

/// Release the fields retained solely to remap a generated Lean diagnostic.
/// A module cannot subsequently emit such a diagnostic after its successful
/// elaboration, so releasing each module at that point lowers the parent
/// process throughout the remaining topological sequence.
fn release_module_diagnostic_buffers(module: &mut RenderedModule) {
    module.lean_text.clear();
    module.lean_text.shrink_to_fit();
    module.coverage.lean.clear();
    module.coverage.lean.shrink_to_fit();
    module.map.sources.clear();
    module.map.sources.shrink_to_fit();
    module.map.mappings.clear();
    module.map.mappings.shrink_to_fit();
}

/// Release every generated diagnostic buffer at the elaboration boundary.
fn release_rendered_buffers(build: &mut RenderedBuild) {
    for module in &mut build.modules {
        release_module_diagnostic_buffers(module);
    }
}

/// The declaration mapping enclosing a generated byte position: the
/// nearest preceding `declaration`-role mapping.
fn enclosing_declaration(module: &RenderedModule, offset: usize) -> Option<&Mapping> {
    module
        .map
        .mappings
        .iter()
        .filter(|mapping| {
            mapping.artifact == 0
                && mapping.role == MapRole::Declaration
                && mapping.gen_start <= offset
        })
        .max_by_key(|mapping| mapping.gen_start)
}

/// Is `c` a character of a generated Lean identifier?
fn is_lean_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// The generated local names (`llv<n>`, `llh<n>`, §17.8, each optionally
/// carrying the `_` prefix an unreferenced binder gets) a Lean message
/// mentions, in first-mention order.
fn generated_names_in(message: &str) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    let mut previous: Option<char> = None;
    let mut rest = message;
    while !rest.is_empty() {
        let word: String = rest
            .chars()
            .take_while(|c| is_lean_ident_char(*c))
            .collect();
        if word.is_empty() {
            let mut chars = rest.chars();
            previous = chars.next();
            rest = chars.as_str();
            continue;
        }
        let starts_word = previous.is_none_or(|c| !is_lean_ident_char(c));
        let stem = word.strip_prefix('_').unwrap_or(&word);
        let generated = stem.len() > 3
            && (stem.starts_with("llv") || stem.starts_with("llh"))
            && stem[3..].chars().all(|c| c.is_ascii_digit());
        if starts_word && generated && !names.contains(&word) {
            names.push(word.clone());
        }
        previous = word.chars().next_back();
        rest = &rest[word.len()..];
    }
    names
}

/// Notes mapping generated local names a Lean message mentions back to
/// their source spellings (§17.8: spellings are retained for diagnostics).
/// A generated name is resolved within the declaration enclosing the
/// reported position: its Lean coverage row carries the local's identity,
/// and the source coverage rows (or the proof-introduced spellings) carry
/// the spelling.
fn generated_name_notes(
    checked: &CheckedProject,
    module: &RenderedModule,
    offset: usize,
    message: &str,
) -> Vec<String> {
    let names = generated_names_in(message);
    if names.is_empty() {
        return Vec::new();
    }
    let Some(checked_module) = checked.modules.get(&module.module) else {
        return Vec::new();
    };
    let declaration_start =
        enclosing_declaration(module, offset).map_or(0, |mapping| mapping.gen_start);
    let declaration_end = module
        .map
        .mappings
        .iter()
        .filter(|mapping| {
            mapping.artifact == 0
                && mapping.role == MapRole::Declaration
                && mapping.gen_start > declaration_start
        })
        .map(|mapping| mapping.gen_start)
        .min()
        .unwrap_or(module.lean_text.len());
    let mut notes = Vec::new();
    for name in names {
        let local = module.coverage.lean.iter().find_map(|row| {
            let within = row.byte_start >= declaration_start && row.byte_end <= declaration_end;
            let text = module.lean_text.get(row.byte_start..row.byte_end);
            match (&row.origin, within, text) {
                (Origin::Local(id), true, Some(text)) if text == name => Some(*id),
                _ => None,
            }
        });
        let Some(id) = local else {
            continue;
        };
        let spelling = checked_module
            .coverage_source
            .iter()
            .find(|row| matches!(row.binding, Origin::Local(local) if local == id))
            .and_then(|row| checked_module.normalized.get(row.byte_start..row.byte_end))
            .map(str::to_owned)
            .or_else(|| {
                let id = u64::try_from(id).ok()?;
                checked_module.proof_spellings.get(&LocalId(id)).cloned()
            });
        if let Some(spelling) = spelling {
            if spelling != name {
                notes.push(format!(
                    "generated name `{name}` is the source binder `{spelling}`"
                ));
            }
        }
    }
    notes
}

/// What a module compilation produced, for remapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LeanOutcome {
    /// A nonzero exit: every error and warning is a rejection (LLV7002).
    Rejected,
    /// A zero exit with output: every warning or informational message is
    /// a verification failure (LLV7006, §20.2, §22.3).
    Noisy,
}

/// Remap every Lean message location against the generated module maps
/// (§20.4): the smallest enclosing mapping wins; a position no mapping
/// encloses falls back to the enclosing declaration component with the
/// generated range kept as a note. Warnings and errors remap alike; a
/// generated local name the message mentions is explained by a note.
fn remap_lean_output(
    checked: &CheckedProject,
    build: &RenderedBuild,
    module_lean_name: &str,
    output: &str,
    outcome: LeanOutcome,
) -> Vec<Diagnostic> {
    let (code, verb) = match outcome {
        LeanOutcome::Rejected => (code!("LLV7002"), "rejected"),
        LeanOutcome::Noisy => (code!("LLV7006"), "produced unexpected output for"),
    };
    let raw = |text: &str| {
        Diagnostic::new(
            code,
            format!("Lean {verb} `{module_lean_name}`: {}", text.trim_end()),
        )
    };
    let Some(module) = build
        .modules
        .iter()
        .find(|module| module.lean_module == module_lean_name)
    else {
        return vec![raw(output)];
    };
    let mut diagnostics = Vec::new();
    if outcome == LeanOutcome::Noisy {
        let preamble = lean_output_preamble(output);
        if !preamble.is_empty() {
            diagnostics.push(raw(&preamble));
        }
    }
    for message in parse_lean_messages(output) {
        if outcome == LeanOutcome::Rejected && message.severity == "info" {
            continue;
        }
        let kind = match &message.name {
            Some(name) => format!("{} {name}", message.severity),
            None => message.severity.clone(),
        };
        let mut diagnostic = Diagnostic::new(
            code,
            format!(
                "Lean {verb} `{module_lean_name}` ({kind}): {}",
                message.message
            ),
        );
        let offset = lean_position_to_byte(&module.lean_text, message.line, message.column);
        let generated_note = format!(
            "generated location: {}:{}:{}",
            message.path, message.line, message.column
        );
        match offset {
            Some(offset) => {
                match module
                    .map
                    .remap(0, offset)
                    .and_then(|mapping| mapping.src_range)
                {
                    Some(range) => {
                        diagnostic = diagnostic
                            .with_span(source_span(checked, module, range))
                            .with_note(generated_note);
                    }
                    None => {
                        if let Some(range) = enclosing_declaration(module, offset)
                            .and_then(|mapping| mapping.src_range)
                        {
                            diagnostic = diagnostic.with_span(source_span(checked, module, range));
                        }
                        let generated_end = module.lean_text[offset..]
                            .find(char::is_whitespace)
                            .map_or(module.lean_text.len(), |length| offset + length);
                        diagnostic = diagnostic.with_note(format!(
                            "{generated_note} (unmapped generated bytes {offset}..{generated_end})"
                        ));
                    }
                }
                for note in generated_name_notes(checked, module, offset, &message.message) {
                    diagnostic = diagnostic.with_note(note);
                }
            }
            None => {
                diagnostic = diagnostic.with_note(generated_note);
            }
        }
        diagnostics.push(diagnostic);
    }
    if diagnostics.is_empty() {
        diagnostics.push(raw(output));
    }
    diagnostics
}

/// The fixed `leanchecker` identity probe (module documentation): the
/// module name no workspace defines and the exact normalized answer the
/// pinned toolchain gives.
pub const LEANCHECKER_IDENTITY_MODULE: &str = "LexLeanIdentityProbe";
/// The expected normalized identity output.
pub const LEANCHECKER_IDENTITY_OUTPUT: &str =
    "uncaught exception: Could not find any oleans for: LexLeanIdentityProbe\n";

/// Run the identity probe and record its output as `leanchecker`'s
/// version output; a different answer is a toolchain mismatch (LLV7001).
fn leanchecker_identity(
    toolchain: &Toolchain,
    lean_path: &str,
    workspace_root: &Utf8Path,
    limits: &crate::config::Limits,
    normalizer: &Normalizer,
) -> Result<String, Diagnostic> {
    let record = leanchecker::run_leanchecker(
        toolchain,
        LEANCHECKER_IDENTITY_MODULE,
        lean_path,
        workspace_root,
        limits,
        normalizer,
    )?;
    let combined = format!("{}{}", record.stdout.trim_end(), record.stderr.trim_end());
    let observed = format!("{}\n", combined.trim_end());
    if record.exit_code == 0 || observed != LEANCHECKER_IDENTITY_OUTPUT {
        return Err(Diagnostic::new(
            code!("LLV7001"),
            format!(
                "leanchecker identity probe answered exit {} with `{}`, expected `{}`",
                record.exit_code,
                observed.trim_end(),
                LEANCHECKER_IDENTITY_OUTPUT.trim_end()
            ),
        ));
    }
    Ok(observed)
}

/// Compare a staged directory against a published one: every file
/// byte-equal, no extra or missing files on either side.
fn validate_existing(staged: &std::path::Path, published: &Utf8Path) -> Result<(), String> {
    fn files_of(root: &std::path::Path) -> BTreeMap<String, std::path::PathBuf> {
        let mut out = BTreeMap::new();
        for entry in walkdir::WalkDir::new(root).into_iter().flatten() {
            if entry.file_type().is_file() {
                if let Ok(relative) = entry.path().strip_prefix(root) {
                    out.insert(
                        relative.to_string_lossy().replace('\\', "/"),
                        entry.path().to_path_buf(),
                    );
                }
            }
        }
        out
    }
    let staged_files = files_of(staged);
    let published_files = files_of(published.as_std_path());
    for name in staged_files.keys() {
        if !published_files.contains_key(name) {
            return Err(format!("`{name}` is missing from the published set"));
        }
    }
    for name in published_files.keys() {
        if !staged_files.contains_key(name) {
            return Err(format!(
                "`{name}` is an unexplained extra file in the published set"
            ));
        }
    }
    for (name, staged_path) in &staged_files {
        // The attestation records the current run's host-bound process
        // records; content-addressing already guarantees equality of the
        // body that determines the ID, so the file must be byte-equal too.
        let published_path = &published_files[name];
        let (Ok(a), Ok(b)) = (std::fs::read(staged_path), std::fs::read(published_path)) else {
            return Err(format!("`{name}` could not be read for comparison"));
        };
        if a != b {
            return Err(format!("`{name}` differs from the published bytes"));
        }
    }
    Ok(())
}

/// Fail on any warning or unexpected output from a successful Lean process
/// (§20.2, §22.3): `accepted_stdout` is the only stdout allowed.
fn require_silent(
    record: &ChildRecord,
    stage: &str,
    accepted_stdout: bool,
) -> Result<(), Diagnostic> {
    let stdout_noise = !accepted_stdout && !record.stdout.trim().is_empty();
    if stdout_noise || !record.stderr.trim().is_empty() {
        return Err(Diagnostic::new(
            code!("LLV7006"),
            format!(
                "unexpected output during {stage}: {}{}",
                record.stdout.trim_end(),
                record.stderr.trim_end()
            ),
        ));
    }
    Ok(())
}

/// Proof processes run one at a time. A single Atlas environment approaches
/// the memory available on the normative GitHub runner; overlapping two made
/// the hosted runner lose its control-plane heartbeat while swapping. Lean is
/// still free to use its own internal parallelism, while this fixed outer
/// width makes the verifier's peak resident set bounded and reproducible.
const PROCESS_WIDTH: usize = 1;

/// Run one deterministic batch of independent verification processes. All
/// workers are joined before an error is returned so no child can outlive a
/// failed verification or write into a staging tree after it is discarded.
fn run_process_batch<T, F>(items: &[usize], job: F) -> Result<Vec<T>, Diagnostic>
where
    T: Send,
    F: Fn(usize) -> Result<T, Diagnostic> + Sync,
{
    let joined = std::thread::scope(|scope| {
        let job = &job;
        let handles: Vec<_> = items
            .iter()
            .map(|&item| scope.spawn(move || job(item)))
            .collect();
        handles
            .into_iter()
            .map(std::thread::ScopedJoinHandle::join)
            .collect::<Vec<_>>()
    });
    let mut values = Vec::with_capacity(items.len());
    for result in joined {
        let result = result.map_err(|_| internal("verification process worker panicked"))?;
        values.push(result?);
    }
    Ok(values)
}

/// Run the complete verification pipeline (§22.1) over a rendered build.
/// The caller holds the project mutation lock for the whole run (§21.8).
#[allow(clippy::too_many_lines)]
pub fn run(
    project: &Project,
    lock: &Lock,
    checked: &CheckedProject,
    build: &mut RenderedBuild,
) -> Result<VerifyOutcome, LexLeanError> {
    let limits = project.config.limits;

    // Every production root is lowered before any Lean work, and before the
    // toolchain is touched: the lowered program grows with the size of the types a generic
    // definition is instantiated at, and a root whose program or certificates
    // would exceed the project's limits is refused here, as `LLS8002`, before
    // anything is generated from it (§17.17).
    let linked = crate::production::lower::linked_modules(checked);
    let mut lowered_roots = Vec::new();
    for root in crate::production::lower::roots(checked).map_err(fail)? {
        lowered_roots.push(
            crate::production::lower::lower_root(
                &linked,
                &root.module,
                &root.name,
                root.report,
                &limits,
            )
            .map_err(fail)?,
        );
    }
    // Stage 4: toolchain preflight (§22.2).
    let mut toolchain: Toolchain = toolchain::preflight(&limits).map_err(fail)?;
    let toolchain_bin = toolchain.root.join("bin");

    // Stage 5: Lake workspace preflight (§10.4) and module-name conflicts
    // (§18.8, §18.9).
    workspace::preflight(project, lock).map_err(fail)?;
    let semantic_hex32: String = checked.semantic_id.to_hex()[..32].to_owned();
    let probe_name = format!("LexLeanProbe.P{semantic_hex32}");
    let audit_name = format!("LexLeanAudit.A{semantic_hex32}");
    let mut all_names: Vec<String> = build
        .modules
        .iter()
        .map(|module| module.lean_module.clone())
        .collect();
    all_names.push(probe_name.clone());
    all_names.push(audit_name.clone());
    all_names.push(crate::production::lcnf::driver_name(&semantic_hex32));

    // Every external reached by any module (§18.8): direct globals, defined
    // values, and case constructors.
    let mut externals: BTreeMap<String, crate::ir::term::ExternalConstRef> =
        checked.external_used.clone();
    for checked_module in checked.modules.values() {
        // Core-module external closure was established during linking.  Its
        // expression DAG is deliberately released after rendering so Lean,
        // rather than a duplicate Rust graph, owns the verification peak.
        if checked_module.document.core.is_none() {
            externals.extend(crate::backend::lean::document_externals(
                &checked_module.document,
                &checked.closure,
            ));
        }
    }
    let probe = crate::backend::lean::probe_module(&semantic_hex32, &externals, &checked.closure)
        .map_err(fail)?;
    debug_assert_eq!(probe.name, probe_name);

    // Stage 9 preparation: the audit module family is fixed by the build.
    let mut module_declarations: Vec<(String, Vec<String>)> = Vec::new();
    for module in &build.modules {
        module_declarations.push((module.lean_module.clone(), module.declaration_names.clone()));
    }
    let audit_modules = crate::backend::lean::audit_modules(&semantic_hex32, &module_declarations);
    let mut declaration_names: Vec<String> = audit_modules
        .iter()
        .flat_map(|module| module.declarations.iter().cloned())
        .collect();
    declaration_names.sort();
    all_names.extend(audit_modules.iter().map(|module| module.name.clone()));
    workspace::reject_module_conflicts(project, &all_names).map_err(fail)?;

    // Later replay and policy failures need compact source anchors, not the
    // full generated coverage and mapping tables. Freeze those anchors now
    // so the tables can be released at the elaboration boundary.
    let mut module_spans: BTreeMap<String, Span> = BTreeMap::new();
    let mut declaration_spans: BTreeMap<String, Span> = BTreeMap::new();
    for module in &build.modules {
        module_spans.insert(
            module.lean_module.clone(),
            source_span(checked, module, (0, 0)),
        );
        let document = &checked.modules[&module.module].document;
        for declaration in document.declarations() {
            let full_name = format!("{}.{}", module.lean_module, declaration.lean_name);
            if let Some(span) =
                declaration_span(checked, module, &document.name, &declaration.component)
            {
                declaration_spans.insert(full_name, span);
            }
        }
    }

    // Generated-source audit before any Lean invocation (§18.2): the build
    // modules, the probe, and every audit module itself.
    for module in &build.modules {
        let is_core = checked
            .modules
            .get(&module.module)
            .is_some_and(|checked| checked.document.core.is_some());
        let is_semantic = checked
            .modules
            .get(&module.module)
            .is_some_and(|checked| checked.document.semantic.is_some());
        let audited = if is_core {
            generated_core_source_audit(&module.lean_text)
        } else if is_semantic {
            generated_semantic_source_audit(&module.lean_text)
        } else {
            generated_source_audit(&module.lean_text, false)
        };
        if let Err(reason) = audited {
            return Err(fail(internal(format!(
                "`{}`: {reason}",
                module.lean_module
            ))));
        }
    }
    if let Err(reason) = generated_source_audit(&probe.text, false) {
        return Err(fail(internal(format!("probe module: {reason}"))));
    }
    for audit in &audit_modules {
        if let Err(reason) = generated_source_audit(&audit.text, true) {
            return Err(fail(internal(format!(
                "audit module `{}`: {reason}",
                audit.name
            ))));
        }
    }

    // Staging under the build root with owner-only permissions (§25.6).
    let verified_root = project
        .root
        .join(&project.config.build_root)
        .join("verified");
    std::fs::create_dir_all(verified_root.as_std_path()).map_err(|io_error| {
        fail(Diagnostic::new(
            code!("LLB6003"),
            format!("{verified_root}: {io_error}"),
        ))
    })?;
    let staging = tempfile::Builder::new()
        .prefix(".staging-")
        .tempdir_in(verified_root.as_std_path())
        .map_err(|io_error| {
            fail(Diagnostic::new(
                code!("LLB6003"),
                format!("staging: {io_error}"),
            ))
        })?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(staging.path(), std::fs::Permissions::from_mode(0o700));
    }
    let staging_utf8 = Utf8PathBuf::from_path_buf(staging.path().to_path_buf())
        .map_err(|_| fail(Diagnostic::new(code!("LLS8001"), "non-UTF-8 staging path")))?;
    // The external PDF provider's isolated directory is a sibling of the
    // staging tree, never inside it (§19.7 step 4, §25.6): the provider is
    // an untrusted external program, and a program that walks out of its
    // working directory must not reach the artifacts being staged for
    // publication. Both live under the configured build root.
    let provider_root = project.root.join(&project.config.build_root).join("pdf");
    std::fs::create_dir_all(provider_root.as_std_path()).map_err(|io_error| {
        fail(Diagnostic::new(
            code!("LLB6003"),
            format!("{provider_root}: {io_error}"),
        ))
    })?;
    let workspace_root = if project.config.lean_workspace == "." {
        project.root.clone()
    } else {
        project.root.join(&project.config.lean_workspace)
    };
    let normalizer = Normalizer::new(
        &staging_utf8,
        &project.root,
        &workspace_root,
        &toolchain.root,
    );

    // Copy the platform-independent build artifacts into the verified set
    // (§22.8).
    for (relative, bytes) in &build.files {
        let renamed = if relative == "manifest.json" {
            "build-manifest.json".to_owned()
        } else {
            relative.clone()
        };
        write_staged(staging.path(), &renamed, bytes)?;
    }

    // Stage the compilation tree.
    let src_root = staging_utf8.join("lean-src");
    let olean_root = staging_utf8.join("oleans");
    std::fs::create_dir_all(olean_root.as_std_path()).map_err(|io_error| {
        fail(Diagnostic::new(
            code!("LLB6003"),
            format!("staging oleans: {io_error}"),
        ))
    })?;
    for module in &build.modules {
        let module_path = module.lean_module.replace('.', "/");
        write_staged(
            staging.path(),
            &format!("lean-src/{module_path}.lean"),
            module.lean_text.as_bytes(),
        )?;
    }
    release_non_diagnostic_buffers(build);
    let lean_path_env = format!("{olean_root}");
    let mut process_records: Vec<ChildRecord> = Vec::new();

    // The leanchecker identity (module documentation), before any replay:
    // the preflight's toolchain-relative path and digest line, then the
    // fixed probe's normalized answer.
    let probe_answer = leanchecker_identity(
        &toolchain,
        &lean_path_env,
        &workspace_root,
        &limits,
        &normalizer,
    )
    .map_err(fail)?;
    toolchain.leanchecker.version_output =
        format!("{}\n{probe_answer}", toolchain.leanchecker.version_output);

    // Stage 6: the external-interface probe (§18.8).
    write_staged(
        staging.path(),
        &format!("probe/{probe_name}.lean"),
        probe.text.as_bytes(),
    )?;
    let probe_source = staging_utf8
        .join("probe")
        .join(format!("{probe_name}.lean"));
    let probe_record = run_child(
        &ChildSpec {
            tool: "lean",
            module: Some(probe_name.clone()),
            program: &toolchain.lake.path,
            executable_sha256: toolchain.lean.sha256,
            argv: vec![
                "env".to_owned(),
                "lean".to_owned(),
                probe_source.to_string(),
            ],
            cwd: &workspace_root,
            extra_env: vec![("LEAN_PATH".to_owned(), lean_path_env.clone())],
            home: ChildHome::Toolchain {
                toolchain_bin: &toolchain_bin,
            },
        },
        &limits,
        &normalizer,
    )
    .map_err(fail)?;
    if probe_record.exit_code != 0 {
        // Lean writes messages to stdout under `lake env lean`; both
        // normalized streams are reported, and every failing line is
        // attributed to its entry (S11).
        let combined = format!("{}{}", probe_record.stdout, probe_record.stderr);
        let mut diagnostic = Diagnostic::new(
            code!("LLT4003"),
            format!(
                "an external-interface probe failed to elaborate: {}",
                combined.trim_end()
            ),
        );
        let mut noted: BTreeSet<String> = BTreeSet::new();
        for message in parse_lean_messages(&combined) {
            if let Some(row) = probe.entry_at_line(message.line) {
                if noted.insert(row.entry.clone()) {
                    diagnostic = diagnostic.with_note(format!(
                        "probe line {} belongs to entry `{}` (probe index {}): {}",
                        row.line, row.entry, row.index, message.message
                    ));
                }
            }
        }
        return Err(fail(diagnostic));
    }
    require_silent(&probe_record, "the external-interface probe", false).map_err(fail)?;
    write_staged(
        staging.path(),
        "probe/process.json",
        &probe_record.to_json().to_file_bytes(),
    )?;
    process_records.push(probe_record);

    // Stage 7: module elaboration in topological import order (§22.3).
    // A topological frontier contains no dependency edges, so its members
    // may run concurrently without making an imported olean observable
    // before the process that owns it has completed successfully.
    let mut placed: BTreeSet<String> = BTreeSet::new();
    let mut remaining: Vec<usize> = (0..build.modules.len()).collect();
    while !remaining.is_empty() {
        let ready: Vec<usize> = remaining
            .iter()
            .copied()
            .filter(|index| {
                let document = &checked.modules[&build.modules[*index].module].document;
                document
                    .imports
                    .iter()
                    .all(|import| placed.contains(import))
            })
            .collect();
        if ready.is_empty() {
            return Err(fail(internal("module order did not converge")));
        }
        for batch in ready.chunks(PROCESS_WIDTH) {
            for &module_index in batch {
                let module_path = build.modules[module_index].lean_module.replace('.', "/");
                let olean = olean_root.join(format!("{module_path}.olean"));
                if let Some(parent) = olean.parent() {
                    std::fs::create_dir_all(parent.as_std_path()).map_err(|io_error| {
                        fail(Diagnostic::new(
                            code!("LLB6003"),
                            format!("staging oleans: {io_error}"),
                        ))
                    })?;
                }
            }
            let records = run_process_batch(batch, |module_index| {
                let module_name = &build.modules[module_index].lean_module;
                let module_path = module_name.replace('.', "/");
                let source = src_root.join(format!("{module_path}.lean"));
                let olean = olean_root.join(format!("{module_path}.olean"));
                run_child(
                    &ChildSpec {
                        tool: "lean",
                        module: Some(module_name.clone()),
                        program: &toolchain.lake.path,
                        executable_sha256: toolchain.lean.sha256,
                        argv: vec![
                            "env".to_owned(),
                            "lean".to_owned(),
                            "-R".to_owned(),
                            src_root.to_string(),
                            "-o".to_owned(),
                            olean.to_string(),
                            source.to_string(),
                        ],
                        cwd: &workspace_root,
                        extra_env: vec![("LEAN_PATH".to_owned(), lean_path_env.clone())],
                        home: ChildHome::Toolchain {
                            toolchain_bin: &toolchain_bin,
                        },
                    },
                    &limits,
                    &normalizer,
                )
            })
            .map_err(fail)?;
            for (&module_index, record) in batch.iter().zip(records) {
                let module_name = build.modules[module_index].lean_module.clone();
                if record.exit_code != 0 {
                    // Lean reports compile errors on stdout under `lake env
                    // lean`; remap over both streams (§20.4).
                    let combined = format!("{}\n{}", record.stdout, record.stderr);
                    return Err(LexLeanError::from_diagnostics(remap_lean_output(
                        checked,
                        build,
                        &module_name,
                        &combined,
                        LeanOutcome::Rejected,
                    )));
                }
                // Any warning or unknown informational message fails
                // verification (§20.2, §22.3), remapped like an error.
                if !record.stdout.trim().is_empty() || !record.stderr.trim().is_empty() {
                    let combined = format!("{}\n{}", record.stdout, record.stderr);
                    return Err(LexLeanError::from_diagnostics(remap_lean_output(
                        checked,
                        build,
                        &module_name,
                        &combined,
                        LeanOutcome::Noisy,
                    )));
                }
                let module_path = module_name.replace('.', "/");
                let olean = olean_root.join(format!("{module_path}.olean"));
                if !olean.as_std_path().is_file() {
                    return Err(fail(Diagnostic::new(
                        code!("LLV7002"),
                        format!("`{module_name}` produced no olean"),
                    )));
                }
                write_staged(
                    staging.path(),
                    &format!("process/lean/{module_name}.json"),
                    &record.to_json().to_file_bytes(),
                )?;
                process_records.push(record);
                release_module_diagnostic_buffers(&mut build.modules[module_index]);
            }
        }
        for &module_index in &ready {
            placed.insert(build.modules[module_index].module.clone());
        }
        remaining.retain(|index| !ready.contains(index));
    }

    // Generated diagnostics have all been remapped and every canonical
    // build byte is already staged. Replay, audit, policy, and publication
    // use only compact module metadata and the staged files.
    release_rendered_buffers(build);

    // Stage 8: separate-process leanchecker replay per module, sorted
    // (§22.4).
    let mut sorted_modules: Vec<usize> = (0..build.modules.len()).collect();
    sorted_modules.sort_by(|a, b| {
        build.modules[*a]
            .lean_module
            .cmp(&build.modules[*b].lean_module)
    });
    for batch in sorted_modules.chunks(PROCESS_WIDTH) {
        let records = run_process_batch(batch, |module_index| {
            let module_name = &build.modules[module_index].lean_module;
            leanchecker::replay_module(
                &toolchain,
                module_name,
                &lean_path_env,
                &workspace_root,
                &limits,
                &normalizer,
            )
            // A replay failure is about this module, so it points at the
            // module's source rather than at the project manifest (§20.1).
            .map_err(|diagnostic| {
                module_spans
                    .get(module_name)
                    .map_or(diagnostic.clone(), |span| {
                        diagnostic.with_span(span.clone())
                    })
            })
        })
        .map_err(fail)?;
        for (&module_index, record) in batch.iter().zip(records) {
            let module_name = &build.modules[module_index].lean_module;
            write_staged(
                staging.path(),
                &format!("process/leanchecker/{module_name}.json"),
                &record.to_json().to_file_bytes(),
            )?;
            process_records.push(record);
        }
    }

    // Stage 9–10: process-sized audit modules and exact output parsing
    // (§18.9, §22.5). Each member imports one generated module, so the
    // final policy audit cannot recreate the monolithic replay peak.
    let mut observed: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut audit_output = String::new();
    for audit in &audit_modules {
        let source_relative = format!("audit/{}.lean", audit.name);
        write_staged(staging.path(), &source_relative, audit.text.as_bytes())?;
    }
    let audit_indices: Vec<usize> = (0..audit_modules.len()).collect();
    for batch in audit_indices.chunks(PROCESS_WIDTH) {
        let records = run_process_batch(batch, |audit_index| {
            let audit = &audit_modules[audit_index];
            let source_relative = format!("audit/{}.lean", audit.name);
            let audit_source = staging_utf8.join(&source_relative);
            run_child(
                &ChildSpec {
                    tool: "lean",
                    module: Some(audit.name.clone()),
                    program: &toolchain.lake.path,
                    executable_sha256: toolchain.lean.sha256,
                    argv: vec![
                        "env".to_owned(),
                        "lean".to_owned(),
                        audit_source.to_string(),
                    ],
                    cwd: &workspace_root,
                    extra_env: vec![("LEAN_PATH".to_owned(), lean_path_env.clone())],
                    home: ChildHome::Toolchain {
                        toolchain_bin: &toolchain_bin,
                    },
                },
                &limits,
                &normalizer,
            )
        })
        .map_err(fail)?;
        for (&audit_index, audit_record) in batch.iter().zip(records) {
            let audit = &audit_modules[audit_index];
            if audit_record.exit_code != 0 {
                return Err(fail(Diagnostic::new(
                    code!("LLV7004"),
                    format!(
                        "axiom audit member `{}` for `{}` failed: {}{}",
                        audit.name,
                        audit.generated_module,
                        audit_record.stdout.trim_end(),
                        audit_record.stderr.trim_end()
                    ),
                )));
            }
            let stage = format!("axiom audit member `{}`", audit.name);
            require_silent(&audit_record, &stage, true).map_err(fail)?;
            let member_observed =
                axiom::parse_audit_output(&audit_record.stdout, &audit.declarations)
                    .map_err(|failure| fail(audit_diagnostic(&declaration_spans, failure)))?;
            for (name, axioms) in member_observed {
                if observed.insert(name.clone(), axioms).is_some() {
                    return Err(fail(internal(format!(
                        "axiom audit member `{}` repeated declaration `{name}`",
                        audit.name
                    ))));
                }
            }
            audit_output.push_str(&audit_record.stdout);
            write_staged(
                staging.path(),
                &format!("audit/{}.process.json", audit.name),
                &audit_record.to_json().to_file_bytes(),
            )?;
            process_records.push(audit_record);
        }
    }
    if observed.len() != declaration_names.len() {
        return Err(fail(internal(format!(
            "axiom audit family observed {} of {} declarations",
            observed.len(),
            declaration_names.len()
        ))));
    }
    write_staged(staging.path(), "audit/output.txt", audit_output.as_bytes())?;

    // Stage 11: per-declaration policy enforcement (§22.6).
    let mut declaration_rows: Vec<Json> = Vec::new();
    for module in &build.modules {
        let document = &checked.modules[&module.module].document;
        for declaration in document.declarations() {
            let full_name = format!("{}.{}", module.lean_module, declaration.lean_name);
            let observed_set = observed.get(&full_name).cloned().unwrap_or_default();
            if !declaration.policy.permits(&observed_set) {
                let mut diagnostic = Diagnostic::new(
                    code!("LLV7005"),
                    format!(
                        "`{full_name}` violates its {} axiom policy: observed [{}]",
                        declaration.policy.kind(),
                        observed_set.join(", ")
                    ),
                );
                if let Some(span) = declaration_spans.get(&full_name).cloned() {
                    diagnostic = diagnostic.with_span(span);
                }
                return Err(fail(diagnostic));
            }
            declaration_rows.push(Json::object(vec![
                ("name", Json::Str(full_name)),
                (
                    "policy",
                    Json::object(vec![
                        ("kind", Json::Str(declaration.policy.kind().to_owned())),
                        (
                            "axioms",
                            Json::Arr(
                                declaration
                                    .policy
                                    .axioms()
                                    .iter()
                                    .cloned()
                                    .map(Json::Str)
                                    .collect(),
                            ),
                        ),
                    ]),
                ),
                (
                    "observed",
                    Json::Arr(observed_set.into_iter().map(Json::Str).collect()),
                ),
                ("result", Json::Str("ok".to_owned())),
            ]));
        }
        if let Some(core) = &document.core {
            for declaration in &core.declarations {
                let Some(observed_set) = observed.get(&declaration.name).cloned() else {
                    continue;
                };
                if !declaration.policy.permits(&observed_set) {
                    return Err(fail(Diagnostic::new(
                        code!("LLV7005"),
                        format!(
                            "`{}` violates its {} axiom policy: observed [{}]",
                            declaration.name,
                            declaration.policy.kind(),
                            observed_set.join(", ")
                        ),
                    )));
                }
                declaration_rows.push(Json::object(vec![
                    ("name", Json::Str(declaration.name.clone())),
                    (
                        "policy",
                        Json::object(vec![
                            ("kind", Json::Str(declaration.policy.kind().to_owned())),
                            (
                                "axioms",
                                Json::Arr(
                                    declaration
                                        .policy
                                        .axioms()
                                        .iter()
                                        .cloned()
                                        .map(Json::Str)
                                        .collect(),
                                ),
                            ),
                        ]),
                    ),
                    (
                        "observed",
                        Json::Arr(observed_set.into_iter().map(Json::Str).collect()),
                    ),
                    ("result", Json::Str("ok".to_owned())),
                ]));
            }
        }
        if let Some(semantic) = &document.semantic {
            for (index, declaration) in semantic.declarations.iter().enumerate() {
                // §17.12 (models): every Lean declaration a model declaration
                // generates stays within its exact axiom set, and together
                // they observe exactly that set.
                if crate::ir::semantic::model::declaration_construct(declaration).is_some() {
                    let mut union: BTreeSet<String> = BTreeSet::new();
                    let generated: Vec<String> = semantic
                        .elaborated(index)
                        .unwrap_or_default()
                        .iter()
                        .map(|derived| derived.name().to_owned())
                        .chain(
                            semantic
                                .elaboration
                                .checks(index)
                                .iter()
                                .map(|check| check.name.clone()),
                        )
                        .collect();
                    for name in generated {
                        let full_name = format!("{}.{name}", module.lean_module);
                        let observed_set = observed.get(&full_name).cloned().ok_or_else(|| {
                            fail(internal(format!(
                                "generated declaration `{full_name}` is absent from the axiom audit"
                            )))
                        })?;
                        if observed_set
                            .iter()
                            .any(|axiom| !declaration.axioms().contains(axiom))
                        {
                            return Err(fail(Diagnostic::new(
                                code!("LLV7005"),
                                format!(
                                    "`{full_name}`, generated by `{}`, violates its {} axiom policy: observed [{}]",
                                    declaration.name(),
                                    declaration.axiom_policy_kind(),
                                    observed_set.join(", ")
                                ),
                            )));
                        }
                        union.extend(observed_set.iter().cloned());
                        declaration_rows.push(Json::object(vec![
                            ("name", Json::Str(full_name)),
                            (
                                "policy",
                                Json::object(vec![
                                    ("kind", Json::Str("allow".to_owned())),
                                    (
                                        "axioms",
                                        Json::Arr(
                                            declaration
                                                .axioms()
                                                .iter()
                                                .cloned()
                                                .map(Json::Str)
                                                .collect(),
                                        ),
                                    ),
                                ]),
                            ),
                            (
                                "observed",
                                Json::Arr(observed_set.into_iter().map(Json::Str).collect()),
                            ),
                            ("result", Json::Str("ok".to_owned())),
                        ]));
                    }
                    if union.into_iter().collect::<Vec<_>>() != declaration.axioms() {
                        return Err(fail(Diagnostic::new(
                            code!("LLV7005"),
                            format!(
                                "`{}.{}` violates its {} axiom policy: its generated declarations observe exactly a different set",
                                module.lean_module,
                                declaration.name(),
                                declaration.axiom_policy_kind()
                            ),
                        )));
                    }
                    continue;
                }
                let full_name = format!("{}.{}", module.lean_module, declaration.name());
                let observed_set = observed.get(&full_name).cloned().ok_or_else(|| {
                    fail(internal(format!(
                        "semantic declaration `{full_name}` is absent from the axiom audit"
                    )))
                })?;
                if observed_set != declaration.axioms() {
                    return Err(fail(Diagnostic::new(
                        code!("LLV7005"),
                        format!(
                            "`{full_name}` violates its {} axiom policy: observed [{}]",
                            declaration.axiom_policy_kind(),
                            observed_set.join(", ")
                        ),
                    )));
                }
                declaration_rows.push(Json::object(vec![
                    ("name", Json::Str(full_name)),
                    (
                        "policy",
                        Json::object(vec![
                            (
                                "kind",
                                Json::Str(declaration.axiom_policy_kind().to_owned()),
                            ),
                            (
                                "axioms",
                                Json::Arr(
                                    declaration
                                        .axioms()
                                        .iter()
                                        .cloned()
                                        .map(Json::Str)
                                        .collect(),
                                ),
                            ),
                        ]),
                    ),
                    (
                        "observed",
                        Json::Arr(observed_set.into_iter().map(Json::Str).collect()),
                    ),
                    ("result", Json::Str("ok".to_owned())),
                ]));
            }
        }
    }

    // Stage 12: named-root extraction (§22.10). Lean's compiler front end
    // reports the computational closure of every production root while the
    // module-system intermediates it reads are still staged; the closure must
    // equal the eligibility closure, and its canonical form is published.
    let production_reports: Vec<&crate::production::ModuleReport> = checked
        .modules
        .values()
        .filter_map(|module| module.production.as_ref())
        .filter(|report| !report.roots.is_empty())
        .collect();
    let mut compiler_input_row: Option<Json> = None;
    if !production_reports.is_empty() {
        use crate::production::lcnf::{self, Rejection};
        let rejection = |rejection: Rejection| match rejection {
            Rejection::Drift(reason) => {
                Diagnostic::new(code!("LLV7012"), format!("named-root extraction: {reason}"))
            }
            Rejection::Rejected(reason) => {
                Diagnostic::new(code!("LLV7011"), format!("named-root extraction: {reason}"))
            }
        };
        let roots: Vec<String> = production_reports
            .iter()
            .flat_map(|report| report.roots.iter().map(|root| root.root.clone()))
            .collect();
        let modules: Vec<String> = build
            .modules
            .iter()
            .map(|module| module.lean_module.clone())
            .collect();
        let driver = lcnf::driver(&semantic_hex32, &roots, &modules)
            .map_err(|reason| fail(internal(reason)))?;
        write_staged(
            staging.path(),
            &format!("extract/{}.lean", driver.name),
            driver.text.as_bytes(),
        )?;
        let source = staging_utf8
            .join("extract")
            .join(format!("{}.lean", driver.name));
        let record = run_child(
            &ChildSpec {
                tool: "lean",
                module: Some(driver.name.clone()),
                program: &toolchain.lake.path,
                executable_sha256: toolchain.lean.sha256,
                argv: vec!["env".to_owned(), "lean".to_owned(), source.to_string()],
                cwd: &workspace_root,
                extra_env: vec![("LEAN_PATH".to_owned(), lean_path_env.clone())],
                home: ChildHome::Toolchain {
                    toolchain_bin: &toolchain_bin,
                },
            },
            &limits,
            &normalizer,
        )
        .map_err(fail)?;
        if record.exit_code != 0 {
            let combined = format!("{}{}", record.stdout, record.stderr);
            return Err(fail(rejection(lcnf::classify_failure(&driver, &combined))));
        }
        require_silent(&record, "named-root extraction", true).map_err(fail)?;
        let input = lcnf::compiler_input(
            &driver,
            &record.stdout,
            &roots,
            &modules,
            &production_reports,
        )
        .map_err(|reason| fail(rejection(reason)))?;
        let bytes = input.to_file_bytes();
        write_staged(
            staging.path(),
            "extract/process.json",
            &record.to_json().to_file_bytes(),
        )?;
        write_staged(staging.path(), "production/compiler-input.json", &bytes)?;
        compiler_input_row = Some(Json::object(vec![
            ("byte_length", Json::from_usize(bytes.len())),
            ("sha256", Json::Str(Sha256Digest::of(&bytes).to_hex())),
        ]));
        process_records.push(record);
    }

    // Stage 12b: certificate A (§17.17). Every production root is lowered
    // and certified; the certificates compile beside the shipped library and
    // calculus modules, are replayed, and their axioms audited exactly.
    let mut preservation_row: Option<Json> = None;
    if !production_reports.is_empty() {
        use crate::production::certificate::certificate;
        use crate::production::lower::roots;
        use crate::production::preserve::{self, CertifiedRendering, CertifiedRoot};
        use crate::production::rust_cert;
        // Lean's first error, from its severity on: the location before it
        // names the staging directory, which is not part of any diagnostic.
        let first_error = |output: &str| lean_error(output);
        let rejected = |module: &str, output: &str, place: &str| {
            Diagnostic::new(
                code!("LLV7013"),
                format!(
                    "certificate A: `{module}` was rejected{place}: {}",
                    first_error(output)
                ),
            )
        };
        let mut certified: Vec<CertifiedRoot> = Vec::new();
        for (index, root) in roots(checked).map_err(fail)?.iter().enumerate() {
            let lowered = lowered_roots[index].clone();
            let module = format!("LexLeanPreserve.C{semantic_hex32}.R{index}");
            let generated = certificate(
                &linked,
                &root.module,
                &root.name,
                root.report,
                &lowered,
                &module,
            )
            .map_err(fail)?;
            // Certificate B: the root's crate in each of its targets
            // simulates the lowered program.
            let mut renderings = Vec::new();
            for row in &root.report.targets {
                let profile =
                    crate::calculus::rust::Profile::named(&row.target).ok_or_else(|| {
                        fail(internal(format!("`{}` is not a Rust profile", row.target)))
                    })?;
                let rendered =
                    crate::calculus::rust::lower(&lowered.program, profile).map_err(|reason| {
                        fail(internal(format!(
                            "{} has no rendering in {}: {reason}",
                            root.report.root, row.target
                        )))
                    })?;
                let module_b = rust_cert::module_for(&module, &row.target).ok_or_else(|| {
                    fail(internal(format!("`{}` is not a Rust target", row.target)))
                })?;
                let certificate_b = rust_cert::certificate_b(&lowered.program, &rendered, &module_b)
                    .map_err(|reason| {
                        fail(Diagnostic::new(
                            code!("LLV7015"),
                            format!(
                                "certificate B: `{module_b}`: no derivation relates {} to its {} rendering: {reason}",
                                root.report.root, row.target
                            ),
                        ))
                    })?;
                // Certificate E: A and B composed, for the root's rendering.
                let module_e = rust_cert::module_for(&format!("{module}.Compose"), &row.target)
                    .ok_or_else(|| {
                        fail(internal(format!("`{}` is not a Rust target", row.target)))
                    })?;
                let fallible = crate::calculus::rust::fallible_functions(&lowered.program)
                    .map_err(|reason| fail(internal(reason)))?
                    .get(lowered.entry() as usize)
                    .copied()
                    .ok_or_else(|| fail(internal("a lowered program without its entry")))?;
                let composed = crate::production::certificate::certificate_e(
                    &linked,
                    &root.module,
                    &root.name,
                    &lowered,
                    &module,
                    &module_b,
                    &module_e,
                    fallible,
                )
                .map_err(fail)?;
                renderings.push(CertifiedRendering {
                    target: row.target.clone(),
                    certificate: certificate_b.into_certificate(),
                    composed,
                    crate_text: crate::calculus::rust::ast::print(&rendered),
                });
            }
            for generated in std::iter::once(&generated).chain(
                renderings
                    .iter()
                    .flat_map(|rendering| [&rendering.certificate, &rendering.composed]),
            ) {
                if generated.text.len() as u64 > limits.max_file_bytes {
                    return Err(fail(Diagnostic::new(
                        code!("LLS8002"),
                        format!(
                            "max_file_bytes exceeded in phase certificates: configured {}, `{}` is {} bytes",
                            limits.max_file_bytes,
                            generated.module,
                            generated.text.len()
                        ),
                    )));
                }
            }
            certified.push(CertifiedRoot {
                root: root.report.root.clone(),
                targets: root
                    .report
                    .targets
                    .iter()
                    .map(|row| row.target.clone())
                    .collect(),
                certificate: generated,
                program: lowered.program.to_file_bytes(),
                renderings,
            });
        }
        let certificates: Vec<crate::production::certificate::Certificate> = certified
            .iter()
            .map(|root| root.certificate.clone())
            .collect();
        let rendered_certificates: Vec<crate::production::certificate::Certificate> = certified
            .iter()
            .flat_map(|root| {
                root.renderings
                    .iter()
                    .map(|rendering| rendering.certificate.clone())
            })
            .collect();
        let composed_certificates: Vec<crate::production::certificate::Certificate> = certified
            .iter()
            .flat_map(|root| {
                root.renderings
                    .iter()
                    .map(|rendering| rendering.composed.clone())
            })
            .collect();
        let staged_certificates: Vec<crate::production::certificate::Certificate> = certificates
            .iter()
            .chain(&rendered_certificates)
            .chain(&composed_certificates)
            .cloned()
            .collect();
        let is_b = |module: &str| {
            rendered_certificates
                .iter()
                .any(|certificate| certificate.module == module)
        };
        let is_e = |module: &str| {
            composed_certificates
                .iter()
                .any(|certificate| certificate.module == module)
        };
        // Where a rejection lies: the declaration of the certificate Lean's
        // first error is in, and the root it certifies.
        let place = |module: &str, output: &str, text: &str| {
            let declaration = preserve::failing_declaration(text, output)
                .map(|declaration| format!(" in `{declaration}`"))
                .unwrap_or_default();
            let root = certified
                .iter()
                .find(|root| module.starts_with(&root.certificate.module))
                .map(|root| format!(" of root `{}`", root.root))
                .unwrap_or_default();
            format!("{declaration}{root}")
        };
        let reject = |module: &str, output: &str, exit_code: i32, text: &str| {
            let place = place(module, output, text);
            if let Some(exhausted) = resource_death(module, exit_code, output, &limits) {
                exhausted
            } else if is_e(module) {
                Diagnostic::new(
                    code!("LLV7016"),
                    format!(
                        "certificate E: `{module}` was rejected{place}: {}",
                        first_error(output)
                    ),
                )
            } else if is_b(module) {
                Diagnostic::new(
                    code!("LLV7015"),
                    format!(
                        "certificate B: `{module}` was rejected{place}: {}",
                        first_error(output)
                    ),
                )
            } else {
                rejected(module, output, &place)
            }
        };
        let generated_modules: Vec<String> = build
            .modules
            .iter()
            .map(|module| module.lean_module.clone())
            .collect();
        let stage = preserve::stage(&generated_modules, &staged_certificates).map_err(fail)?;
        let preserve_src = staging_utf8.join("preserve-src");
        let preserve_oleans = staging_utf8.join("preserve-oleans");
        let preserve_path =
            std::env::join_paths([preserve_oleans.as_std_path(), olean_root.as_std_path()])
                .map_err(|error| fail(internal(format!("the preservation search path: {error}"))))?
                .to_string_lossy()
                .into_owned();
        for file in stage
            .environment
            .iter()
            .chain(&stage.certificates)
            .chain([&stage.audit])
        {
            write_staged(
                staging.path(),
                &format!("preserve-src/{}", file.path),
                file.text.as_bytes(),
            )?;
        }
        let compile = |file: &preserve::StagedFile, output: bool| {
            let source = preserve_src.join(&file.path);
            let mut argv = vec![
                "env".to_owned(),
                "lean".to_owned(),
                "-R".to_owned(),
                preserve_src.to_string(),
            ];
            if output {
                let olean = preserve_oleans.join(file.path.replace(".lean", ".olean"));
                if let Some(parent) = olean.parent() {
                    std::fs::create_dir_all(parent.as_std_path()).map_err(|io_error| {
                        Diagnostic::new(code!("LLB6003"), format!("staging oleans: {io_error}"))
                    })?;
                }
                argv.push("-o".to_owned());
                argv.push(olean.to_string());
            }
            argv.push(source.to_string());
            run_child(
                &ChildSpec {
                    tool: "lean",
                    module: Some(file.module.clone()),
                    program: &toolchain.lake.path,
                    executable_sha256: toolchain.lean.sha256,
                    argv,
                    cwd: &workspace_root,
                    extra_env: vec![("LEAN_PATH".to_owned(), preserve_path.clone())],
                    home: ChildHome::Toolchain {
                        toolchain_bin: &toolchain_bin,
                    },
                },
                &limits,
                &normalizer,
            )
        };
        for file in &stage.environment {
            let record = compile(file, true).map_err(fail)?;
            if record.exit_code != 0
                || !record.stdout.trim().is_empty()
                || !record.stderr.trim().is_empty()
            {
                let combined = format!("{}{}", record.stdout, record.stderr);
                if let Some(exhausted) =
                    resource_death(&file.module, record.exit_code, &combined, &limits)
                {
                    return Err(fail(exhausted));
                }
                return Err(fail(Diagnostic::new(
                    code!("LLV7014"),
                    format!(
                        "preservation environment: `{}` does not compile silently under the pinned Lean: {}",
                        file.module,
                        first_error(&format!("{}{}", record.stdout, record.stderr))
                    ),
                )));
            }
            write_staged(
                staging.path(),
                &format!("process/preserve/{}.json", file.module),
                &record.to_json().to_file_bytes(),
            )?;
            process_records.push(record);
        }
        for file in &stage.certificates {
            let record = compile(file, true).map_err(fail)?;
            if record.exit_code != 0
                || !record.stdout.trim().is_empty()
                || !record.stderr.trim().is_empty()
            {
                let combined = format!("{}{}", record.stdout, record.stderr);
                return Err(fail(reject(
                    &file.module,
                    &combined,
                    record.exit_code,
                    &file.text,
                )));
            }
            write_staged(
                staging.path(),
                &format!("process/preserve/{}.json", file.module),
                &record.to_json().to_file_bytes(),
            )?;
            process_records.push(record);
            let replay = leanchecker::run_leanchecker(
                &toolchain,
                &file.module,
                &preserve_path,
                &workspace_root,
                &limits,
                &normalizer,
            )
            .map_err(fail)?;
            if replay.exit_code != 0 {
                let combined = format!("{}{}", replay.stdout, replay.stderr);
                return Err(fail(reject(
                    &file.module,
                    &combined,
                    replay.exit_code,
                    &file.text,
                )));
            }
            write_staged(
                staging.path(),
                &format!("process/leanchecker/{}.json", file.module),
                &replay.to_json().to_file_bytes(),
            )?;
            process_records.push(replay);
        }
        let audit_record = compile(&stage.audit, false).map_err(fail)?;
        let audit_output = format!("{}{}", audit_record.stdout, audit_record.stderr);
        if audit_record.exit_code != 0 {
            if let Some(exhausted) = resource_death(
                &stage.audit.module,
                audit_record.exit_code,
                &audit_output,
                &limits,
            ) {
                return Err(fail(exhausted));
            }
            return Err(fail(rejected(&stage.audit.module, &audit_output, "")));
        }
        preserve::classify_audit(
            &audit_output,
            &certificates,
            &rendered_certificates,
            &composed_certificates,
        )
        .map_err(fail)?;
        write_staged(
            staging.path(),
            &format!("process/preserve/{}.json", stage.audit.module),
            &audit_record.to_json().to_file_bytes(),
        )?;
        process_records.push(audit_record);
        for file in &stage.certificates {
            write_staged(
                staging.path(),
                &format!("preserve/{}", file.path),
                file.text.as_bytes(),
            )?;
        }
        // The program each root's certificates are about and the crate each
        // target's certificates B and E are about, published beside them and
        // bound by `preservation.json`: what a user compiles is rendered from
        // the first by the second's printer, and certificate E states nothing
        // of the export wrappers or the package around it (§17.17).
        for (index, root) in certified.iter().enumerate() {
            write_staged(
                staging.path(),
                &format!("preserve/program/R{index}.json"),
                &root.program,
            )?;
            for rendering in &root.renderings {
                write_staged(
                    staging.path(),
                    &format!("preserve/crate/R{index}.{}.rs", rendering.target),
                    rendering.crate_text.as_bytes(),
                )?;
            }
        }
        write_staged(
            staging.path(),
            "preserve/audit.txt",
            audit_output.as_bytes(),
        )?;
        let bytes = preserve::record(&certified).to_file_bytes();
        write_staged(staging.path(), "preserve/preservation.json", &bytes)?;
        preservation_row = Some(Json::object(vec![
            ("byte_length", Json::from_usize(bytes.len())),
            ("sha256", Json::Str(Sha256Digest::of(&bytes).to_hex())),
        ]));
        let _ = std::fs::remove_dir_all(preserve_src.as_std_path());
        let _ = std::fs::remove_dir_all(preserve_oleans.as_std_path());
    }

    // Stage 13: optional configured PDF rendering (§19.7): one row and two
    // process records per module.
    let mut pdf_rows: Vec<Json> = Vec::new();
    if let Some(provider) = &project.config.pdf {
        for module in &build.modules {
            let tex_path = staging_utf8.join(&module.tex_path);
            let tex_bytes = std::fs::read(tex_path.as_std_path()).map_err(|io_error| {
                fail(Diagnostic::new(
                    code!("LLB6003"),
                    format!("reading staged {}: {io_error}", module.tex_path),
                ))
            })?;
            let result = crate::backend::pdf::run_provider(
                project,
                provider,
                &tex_bytes,
                &module.lean_module,
                &provider_root,
                &normalizer,
            )
            .map_err(fail)?;
            write_staged(
                staging.path(),
                &format!("pdf/{}.pdf", module.lean_module),
                &result.pdf_bytes,
            )?;
            let version_record = result.version;
            let compile_record = result.compile;
            write_staged(
                staging.path(),
                &format!("pdf/{}.version.json", module.lean_module),
                &version_record.to_json().to_file_bytes(),
            )?;
            write_staged(
                staging.path(),
                &format!("pdf/{}.compile.json", module.lean_module),
                &compile_record.to_json().to_file_bytes(),
            )?;
            pdf_rows.push(Json::object(vec![
                ("module", Json::Str(module.lean_module.clone())),
                ("recipe_id", Json::Str(result.recipe_id.to_hex())),
                ("pdf_sha256", Json::Str(result.pdf_sha256.to_hex())),
                ("byte_length", Json::from_usize(result.pdf_bytes.len())),
                ("version", version_record.to_json()),
                ("compile", compile_record.to_json()),
            ]));
            process_records.push(version_record);
            process_records.push(compile_record);
        }
    }

    // Stage 14: no unexpected absolute paths in successful output (§22.7),
    // over every process record.
    for record in &process_records {
        if normalizer.has_unexpected_absolute_path(&record.stdout)
            || normalizer.has_unexpected_absolute_path(&record.stderr)
        {
            return Err(fail(Diagnostic::new(
                code!("LLV7006"),
                format!(
                    "unexpected absolute path in the output of `{}`{}",
                    record.tool,
                    record
                        .module
                        .as_ref()
                        .map_or_else(String::new, |module| format!(" for `{module}`"))
                ),
            )));
        }
    }

    // Remove the compilation scratch tree; the fixed §22.8 artifact set
    // keeps exactly `oleans/*.olean` (the module-system `.olean.private`,
    // `.olean.server`, and `.ir` intermediates are not part of the set and
    // are removed once every consumer — later modules, the replay, and the
    // audit — has run).
    let _ = std::fs::remove_dir_all(src_root.as_std_path());
    for entry in walkdir::WalkDir::new(olean_root.as_std_path())
        .into_iter()
        .flatten()
    {
        if entry.file_type().is_file()
            && entry
                .path()
                .extension()
                .is_none_or(|extension| extension != "olean")
        {
            std::fs::remove_file(entry.path()).map_err(|io_error| {
                fail(Diagnostic::new(
                    code!("LLB6003"),
                    format!("removing {}: {io_error}", entry.path().display()),
                ))
            })?;
        }
    }

    // Copy oleans into the verified set and hash them.
    let mut olean_rows: Vec<Json> = Vec::new();
    for module in &build.modules {
        let module_path = module.lean_module.replace('.', "/");
        let olean = olean_root.join(format!("{module_path}.olean"));
        let bytes = std::fs::read(olean.as_std_path()).map_err(|io_error| {
            fail(Diagnostic::new(
                code!("LLV7002"),
                format!("{olean}: {io_error}"),
            ))
        })?;
        olean_rows.push(Json::object(vec![
            ("module", Json::Str(module.lean_module.clone())),
            ("byte_length", Json::from_usize(bytes.len())),
            ("sha256", Json::Str(Sha256Digest::of(&bytes).to_hex())),
        ]));
    }

    // Stage 15: the attestation (§22.9). No timestamp is hashed. The
    // running executable must be readable to be recorded; nothing is
    // fabricated in its place.
    let lexlean_executable_sha256 = std::env::current_exe()
        .and_then(std::fs::read)
        .map(|bytes| Sha256Digest::of(&bytes))
        .map_err(|io_error| {
            fail(internal(format!(
                "reading the running executable: {io_error}"
            )))
        })?;
    let tool_json = |tool: &toolchain::Tool| {
        Json::object(vec![
            ("version_output", Json::Str(tool.version_output.clone())),
            ("executable_sha256", Json::Str(tool.sha256.to_hex())),
        ])
    };
    let mut body_fields = vec![
        // Language 1.2 routes to its own attestation schema, which records
        // the compiler input; the 1.0 and 1.1 shape is frozen (§22.9).
        (
            "spec",
            Json::Str(
                if project.config.language == crate::LANGUAGE_1_2 {
                    "lexlean/attestation/2"
                } else {
                    "lexlean/attestation/1"
                }
                .to_owned(),
            ),
        ),
        ("status", Json::Str("verified".to_owned())),
        ("semantic_id", Json::Str(checked.semantic_id.to_hex())),
        ("source_id", Json::Str(checked.source_id.to_hex())),
        ("build_id", Json::Str(build.build_id.to_hex())),
        (
            "host",
            Json::object(vec![
                ("os", Json::Str(std::env::consts::OS.to_owned())),
                ("arch", Json::Str(std::env::consts::ARCH.to_owned())),
            ]),
        ),
        (
            "lexlean",
            Json::object(vec![
                ("version", Json::Str(crate::COMPILER_VERSION.to_owned())),
                (
                    "compiler_semantics",
                    Json::Str(crate::compiler_semantics_id_for(&project.config.language).to_hex()),
                ),
                (
                    "executable_sha256",
                    Json::Str(lexlean_executable_sha256.to_hex()),
                ),
            ]),
        ),
        (
            "toolchain",
            Json::object(vec![
                ("lean", tool_json(&toolchain.lean)),
                ("lake", tool_json(&toolchain.lake)),
                ("leanchecker", tool_json(&toolchain.leanchecker)),
            ]),
        ),
        (
            "lake_workspace",
            Json::Arr(
                lock.workspace_files
                    .iter()
                    .map(|(path, sha256)| {
                        Json::object(vec![
                            ("path", Json::Str(path.clone())),
                            ("sha256", Json::Str(sha256.to_hex())),
                        ])
                    })
                    .collect(),
            ),
        ),
        (
            "build_manifest",
            Json::object(vec![
                ("byte_length", Json::from_usize(build.manifest_bytes.len())),
                (
                    "sha256",
                    Json::Str(Sha256Digest::of(&build.manifest_bytes).to_hex()),
                ),
            ]),
        ),
        ("oleans", Json::Arr(olean_rows)),
        (
            "processes",
            Json::Arr(process_records.iter().map(ChildRecord::to_json).collect()),
        ),
        ("declarations", Json::Arr(declaration_rows)),
    ];
    if let Some(row) = compiler_input_row {
        body_fields.push(("compiler_input", row));
    }
    if let Some(row) = preservation_row {
        body_fields.push(("preservation", row));
    }
    if !pdf_rows.is_empty() {
        body_fields.push(("pdf", Json::Arr(pdf_rows)));
    }
    let body = Json::object(body_fields);
    let this_attestation_id = attestation_id(&body.to_canonical_string());
    let full = match body {
        Json::Obj(mut object) => {
            object.insert(
                "attestation_id".to_owned(),
                Json::Str(this_attestation_id.to_hex()),
            );
            Json::Obj(object)
        }
        other => other,
    };
    write_staged(staging.path(), "attestation.json", &full.to_file_bytes())?;

    // Stage 16: atomic publication (§21.8).
    let target = verified_root.join(this_attestation_id.to_hex());
    if target.as_std_path().exists() {
        // A repeated verification of identical content reuses the
        // published set only after every staged file validates against it
        // (§21.8); anything else refuses to overwrite unexplained bytes.
        return match validate_existing(staging.path(), &target) {
            Ok(()) => Ok(VerifyOutcome {
                attestation_id: this_attestation_id,
                root: target,
            }),
            Err(reason) => Err(fail(Diagnostic::new(
                code!("LLB6003"),
                format!(
                    "existing verified directory {target} does not validate against this run: {reason}; refusing to overwrite unexplained bytes"
                ),
            ))),
        };
    }
    let staged = staging.keep();
    std::fs::rename(&staged, target.as_std_path()).map_err(|io_error| {
        let _ = std::fs::remove_dir_all(&staged);
        fail(Diagnostic::new(
            code!("LLB6003"),
            format!("publishing {target}: {io_error}"),
        ))
    })?;
    crate::artifact::fsync_dir(verified_root.as_std_path());
    Ok(VerifyOutcome {
        attestation_id: this_attestation_id,
        root: target,
    })
}

/// A helper for tests: the reserved probe and audit module names for a
/// semantic ID (§18.8, §18.9).
#[must_use]
pub fn reserved_module_names(semantic_id: Sha256Digest) -> (String, String) {
    let hex32: String = semantic_id.to_hex()[..32].to_owned();
    (
        format!("LexLeanProbe.P{hex32}"),
        format!("LexLeanAudit.A{hex32}"),
    )
}

#[cfg(test)]
mod tests {
    use std::sync::{mpsc, Mutex};
    use std::time::Duration;

    use super::{internal, run_process_batch};

    #[test]
    fn process_batch_is_concurrent_and_result_order_is_stable() {
        let (sender, receiver) = mpsc::channel();
        let receiver = Mutex::new(receiver);
        let values = run_process_batch(&[7, 3], |item| {
            if item == 7 {
                receiver
                    .lock()
                    .map_err(|_| internal("test receiver lock was poisoned"))?
                    .recv_timeout(Duration::from_secs(2))
                    .map_err(|error| internal(format!("test receive: {error}")))?;
            } else {
                sender
                    .send(())
                    .map_err(|error| internal(format!("test send: {error}")))?;
            }
            Ok(item)
        })
        .expect("the independent jobs run together");
        assert_eq!(values, vec![7, 3]);
    }
}

/// Lean's first error with its continuation, from its severity on and
/// bounded: the location before it names the staging directory, which is not
/// part of any diagnostic, and the lines after it carry the mismatch it
/// reports, which say which construct the rejection concerns.
fn lean_error(output: &str) -> String {
    const LINES: usize = 12;
    const BYTES: usize = 1500;
    let lines: Vec<&str> = output.lines().collect();
    let Some(start) = lines.iter().position(|line| line.contains("error")) else {
        return "no error was reported".to_owned();
    };
    let first = lines[start];
    let mut out = first[first.find("error").unwrap_or(0)..].trim().to_owned();
    for line in lines[start + 1..].iter().take(LINES - 1) {
        out.push('\n');
        out.push_str(line.trim_end());
    }
    if out.len() > BYTES {
        let mut cut = BYTES;
        while !out.is_char_boundary(cut) {
            cut -= 1;
        }
        out.truncate(cut);
        out.push_str(" ...");
    }
    out
}

/// A pinned Lean that ran out of a resource while checking a generated
/// certificate: its heartbeat or recursion budget, its memory, or a kill by
/// the system with no message. That is a limit, not a rejected certificate and
/// not drift in the environment (`LLS8002`): the generated module is too large
/// for what the machine allows, and no change to the certificate's content
/// would be a defect to report.
fn resource_death(
    module: &str,
    exit_code: i32,
    output: &str,
    limits: &crate::config::Limits,
) -> Option<Diagnostic> {
    const MARKERS: [&str; 6] = [
        "maximum number of heartbeats",
        "(deterministic) timeout",
        "maximum recursion depth",
        "out of memory",
        "stack overflow",
        "Stack overflow",
    ];
    let marked = MARKERS.iter().find(|marker| output.contains(**marker));
    let killed = exit_code == -1;
    if marked.is_none() && !killed {
        return None;
    }
    Some(Diagnostic::new(
        code!("LLS8002"),
        format!(
            "the pinned Lean exhausted a resource checking `{module}` ({}): the generated module is within max_file_bytes {} but beyond what the machine checks, and the project's types are too large for it",
            marked.map_or("it was killed by the system", |marker| *marker),
            limits.max_file_bytes
        ),
    ))
}

#[cfg(test)]
mod resource_tests {
    use super::{lean_error, resource_death};

    fn limits() -> crate::config::Limits {
        crate::config::Limits {
            max_file_bytes: 4_194_304,
            max_total_source_bytes: 67_108_864,
            max_primitive_atoms: 2_000_000,
            max_token_lattice_edges: 4_000_000,
            max_parse_states: 4_000_000,
            max_ir_nodes: 2_000_000,
            max_scope_depth: 1024,
            max_import_depth: 128,
            max_diagnostics: 256,
            max_child_output_bytes: 16_777_216,
            child_timeout_ms: 300_000,
        }
    }

    /// A pinned Lean that gave up on its heartbeat budget, ran out of memory,
    /// or was killed with no message is an exhausted resource, whichever
    /// certificate it was checking; a type error is not.
    #[test]
    fn a_resource_death_is_a_limit_and_a_type_error_is_not() {
        let heartbeats = "R0.lean:12:3: error: (deterministic) timeout at `isDefEq`, maximum number of heartbeats (200000) has been reached";
        let diagnostic = resource_death("M.R0", 1, heartbeats, &limits()).expect("a limit");
        assert_eq!(diagnostic.code.as_str(), "LLS8002");
        assert!(resource_death("M.R0", 1, "error: out of memory", &limits()).is_some());
        assert!(resource_death("M.R0", -1, "", &limits()).is_some());
        assert!(resource_death(
            "M.R0",
            1,
            "R0.lean:3:1: error: Application type mismatch",
            &limits()
        )
        .is_none());
    }

    /// The first error is reported with the lines that say what it
    /// concerns, bounded.
    #[test]
    fn the_first_error_carries_its_continuation_within_a_bound() {
        let long = format!(
            "x.lean:1:1: warning: w\nx.lean:2:2: error: Application type mismatch: The argument\n  hr\nhas type\n{}",
            "  y\n".repeat(400)
        );
        let text = lean_error(&long);
        assert!(text.starts_with("error: Application type mismatch"));
        assert!(text.contains("has type"));
        assert!(text.len() <= 1600);
        assert_eq!(lean_error("fine"), "no error was reported");
    }
}
