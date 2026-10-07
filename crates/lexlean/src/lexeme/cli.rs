//! The `lexlean lexeme` command family (SPEC.md §33.6).
//!
//! Five verbs, one per stage of the pipeline, in pipeline order: `hash`
//! canonicalizes and digests, `sign` binds authorship to a key, `stamp` records
//! an external time anchor, `append` publishes the entry to the log, and
//! `verify` re-checks an entry without trusting the machine that made it.
//! `bootstrap` runs the whole sequence for the ledger's own specification.
//!
//! Every verb is offline except the one line in the documentation that obtains a
//! token, because a verification tool that reaches the network on the
//! inventor's machine is a verification tool whose answer depends on that
//! machine. `stamp` therefore builds an RFC 3161 request and attaches the
//! reply that a client produced elsewhere.

use camino::{Utf8Path, Utf8PathBuf};
use clap::Args;

use crate::api::CommandIds;
use crate::code;
use crate::diagnostic::Diagnostic;
use crate::error::LexLeanError;
use crate::lexeme::canonical;
use crate::lexeme::entry::Entry;
use crate::lexeme::ledger::Ledger;
use crate::lexeme::signature;
use crate::lexeme::timestamp::{self, Timestamp};
use crate::lexeme::verify::{self, Verdict};
use crate::lexeme::{DEFAULT_TSA_URL, TIMESTAMP_ARTIFACT};

/// What one lexeme command produced, returned to the CLI for rendering.
#[derive(Debug, Default)]
pub struct LexemeOutcome {
    /// The modules or files the command touched, for the JSON result object.
    pub modules: Vec<String>,
    /// The artifacts the command wrote.
    pub artifacts: Vec<String>,
    /// The content identifiers the command established.
    pub ids: CommandIds,
    /// The one-line human summary.
    pub summary: String,
    /// The canonical JSON payload a `verify` or `hash` reports.
    pub payload: Option<crate::artifact::canonical_json::Json>,
    /// A failure to report *after* the payload is recorded, which is how a
    /// failed verdict still reaches stdout: the verdict is the product of
    /// `verify`, so discarding it because the verdict is negative would throw
    /// away the reason.
    pub failure: Option<LexLeanError>,
}

/// The `lexeme` verb family.
#[derive(Args, Debug)]
pub struct LexemeCommand {
    /// Which stage of the pipeline to run.
    #[command(subcommand)]
    pub action: LexemeAction,
}

/// The verbs, in pipeline order.
#[derive(clap::Subcommand, Debug)]
pub enum LexemeAction {
    /// Canonicalize sources and record the content digest of an entry.
    Hash(LexemeHashArgs),
    /// Bind the content digest to a key.
    Sign(LexemeSignArgs),
    /// Record or request an external time anchor.
    Stamp(LexemeStampArgs),
    /// Append an entry to the ledger and publish a tree head.
    Append(LexemeAppendArgs),
    /// Re-check an entry against itself and, when given, the published log.
    Verify(LexemeVerifyArgs),
    /// Run the whole pipeline for the ledger's own specification.
    Bootstrap(LexemeBootstrapArgs),
}

/// `lexeme hash`: canonicalize a project's Lean sources into an entry.
#[derive(Args, Debug)]
pub struct LexemeHashArgs {
    /// A Lean source file to hash; repeatable.
    #[arg(long = "source", value_name = "PATH", required = true)]
    pub sources: Vec<Utf8PathBuf>,
    /// The directory the `--source` paths are relative to; the paths enter the
    /// entry as this prefix plus their relative form.
    #[arg(long, default_value = ".")]
    pub source_root: Utf8PathBuf,
    /// The entry title; a subject, never a normative claim.
    #[arg(long, default_value = "")]
    pub title: String,
    /// The toolchain the sources elaborate under.
    #[arg(long)]
    pub toolchain: String,
    /// Where to write the canonical entry JSON.
    #[arg(long)]
    pub out: Utf8PathBuf,
}

/// `lexeme sign`: a detached Ed25519 signature over the content digest.
#[derive(Args, Debug)]
pub struct LexemeSignArgs {
    /// The entry to sign.
    #[arg(long)]
    pub entry: Utf8PathBuf,
    /// A 32-byte Ed25519 seed; the key never enters the entry.
    #[arg(long, conflicts_with = "piv_slot")]
    pub seed: Option<Utf8PathBuf>,
    /// A PIV slot, which binds the key to a device and requires its PIN.
    #[arg(long, conflicts_with = "seed", requires = "pin")]
    pub piv_slot: Option<String>,
    /// The device PIN, demanded for every signature.
    #[arg(long)]
    pub pin: Option<String>,
    /// The PIV command-line tool; overridable for a different yubico-piv-tool.
    #[arg(long, default_value = "yubico-piv-tool")]
    pub piv_tool: String,
    /// Where to write the signed entry; the input file when absent.
    #[arg(long)]
    pub out: Option<Utf8PathBuf>,
}

/// `lexeme stamp`: the external time anchor of §33.4.
#[derive(Args, Debug)]
pub struct LexemeStampArgs {
    /// The entry to stamp.
    #[arg(long)]
    pub entry: Utf8PathBuf,
    /// Write a DER `TimeStampReq` here instead of attaching a reply.
    #[arg(long, conflicts_with = "tsr")]
    pub request: Option<Utf8PathBuf>,
    /// The DER `TimeStampResp` a client obtained.
    #[arg(long)]
    pub tsr: Option<Utf8PathBuf>,
    /// The DER certificate of the issuing TSA.
    #[arg(long, requires = "tsr")]
    pub certificate: Option<Utf8PathBuf>,
    /// The DER certificate of the pinned TSA root.
    #[arg(long, requires = "tsr")]
    pub root: Option<Utf8PathBuf>,
    /// The authority URL the token records.
    #[arg(long, default_value = DEFAULT_TSA_URL)]
    pub tsa: String,
    /// Where to write the stamped entry; the input file when absent.
    #[arg(long)]
    pub out: Option<Utf8PathBuf>,
}

/// `lexeme append`: publish an entry to the ledger.
#[derive(Args, Debug)]
pub struct LexemeAppendArgs {
    /// The ledger directory.
    #[arg(long)]
    pub ledger: Utf8PathBuf,
    /// The signed entry to append.
    #[arg(long)]
    pub entry: Utf8PathBuf,
    /// The 32-byte Ed25519 seed of the log key, which signs each published
    /// head (§33.7).
    #[arg(long)]
    pub log_key: Utf8PathBuf,
    /// Where to write the entry carrying its inclusion proof; the input file
    /// when absent.
    #[arg(long)]
    pub out: Option<Utf8PathBuf>,
}

/// `lexeme verify`: the five checks of §33.6.
#[derive(Args, Debug)]
pub struct LexemeVerifyArgs {
    /// The entry to verify.
    #[arg(long)]
    pub entry: Utf8PathBuf,
    /// The ledger directory, supplying the published head for step 5.
    #[arg(long)]
    pub ledger: Option<Utf8PathBuf>,
}

/// `lexeme bootstrap`: the ledger's own specification as its first entry.
#[derive(Args, Debug)]
pub struct LexemeBootstrapArgs {
    /// The ledger directory to create.
    #[arg(long)]
    pub ledger: Utf8PathBuf,
    /// The Lean specification directory whose `.lean` files form the entry.
    #[arg(long)]
    pub specification: Utf8PathBuf,
    /// The tool artifact directories whose files the entry also commits to.
    #[arg(long = "artifact", value_name = "DIR")]
    pub artifacts: Vec<Utf8PathBuf>,
    /// The entry title.
    #[arg(long, default_value = "LexLean lexeme ledger specification")]
    pub title: String,
    /// The toolchain the specification elaborates under.
    #[arg(long)]
    pub toolchain: String,
    /// A 32-byte Ed25519 seed for the bootstrap key.
    #[arg(long)]
    pub seed: Option<Utf8PathBuf>,
    /// The 32-byte Ed25519 seed of the log key, which signs the head (§33.7).
    #[arg(long)]
    pub log_key: Option<Utf8PathBuf>,
    /// The directory the bootstrap entry is written to.
    #[arg(long)]
    pub out: Utf8PathBuf,
}

/// A single-diagnostic CLI failure.
fn usage(reason: impl Into<String>) -> LexLeanError {
    LexLeanError::from_diagnostic(Diagnostic::new(code!("LLC0001"), reason))
}

/// A single-diagnostic canonicalization failure.
fn source_failure(reason: String, location: String) -> LexLeanError {
    LexLeanError::from_diagnostic(Diagnostic::new(
        code!("LLG1001"),
        format!("{reason} ({location})"),
    ))
}

/// A single-diagnostic ledger failure.
fn ledger_failure(reason: impl Into<String>) -> LexLeanError {
    LexLeanError::from_diagnostic(Diagnostic::new(code!("LLG1009"), reason))
}

/// Read an entry from a file, refusing anything that is not canonical JSON.
fn read_entry(path: &Utf8Path) -> Result<Entry, LexLeanError> {
    let bytes = std::fs::read(path).map_err(|error| {
        LexLeanError::from_diagnostic(Diagnostic::new(
            code!("LLG1002"),
            format!("{}: {error}", path.as_str()),
        ))
    })?;
    let value = crate::artifact::canonical_json::Json::parse(&bytes).map_err(|reason| {
        LexLeanError::from_diagnostic(Diagnostic::new(
            code!("LLG1002"),
            format!("{}: {reason}", path.as_str()),
        ))
    })?;
    Entry::from_json(&value)
}

/// Write canonical JSON through a temporary file and a rename.
fn write_json(
    path: &Utf8Path,
    value: &crate::artifact::canonical_json::Json,
) -> Result<(), LexLeanError> {
    let temporary = path.with_extension("tmp");
    std::fs::write(&temporary, value.to_file_bytes())
        .map_err(|error| ledger_failure(format!("{}: {error}", temporary.as_str())))?;
    std::fs::rename(&temporary, path)
        .map_err(|error| ledger_failure(format!("{}: {error}", path.as_str())))
}

/// Run one lexeme verb.
pub fn run(
    command: &LexemeCommand,
    working_directory: &Utf8Path,
) -> Result<LexemeOutcome, LexLeanError> {
    match &command.action {
        LexemeAction::Hash(arguments) => hash(arguments, working_directory),
        LexemeAction::Sign(arguments) => sign(arguments, working_directory),
        LexemeAction::Stamp(arguments) => stamp(arguments),
        LexemeAction::Append(arguments) => append(arguments, working_directory),
        LexemeAction::Verify(arguments) => verify(arguments, working_directory),
        LexemeAction::Bootstrap(arguments) => bootstrap(arguments, working_directory),
    }
}

/// Resolve a path against the working directory when it is relative.
fn resolve(working_directory: &Utf8Path, path: &Utf8Path) -> Utf8PathBuf {
    if path.is_relative() {
        working_directory.join(path)
    } else {
        path.to_path_buf()
    }
}

/// Read the `.lean` sources of a directory in ascending path order.
fn lean_sources(root: &Utf8Path) -> Result<Vec<(String, String)>, LexLeanError> {
    let mut found: Vec<(String, String)> = Vec::new();
    collect_lean(root, root, &mut found)?;
    found.sort_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
    Ok(found)
}

/// Walk one directory for `.lean` files, recording project-relative paths.
fn collect_lean(
    root: &Utf8Path,
    directory: &Utf8Path,
    found: &mut Vec<(String, String)>,
) -> Result<(), LexLeanError> {
    let entries = std::fs::read_dir(directory)
        .map_err(|error| ledger_failure(format!("{}: {error}", directory.as_str())))?;
    for entry in entries.flatten() {
        let path = Utf8PathBuf::from_path_buf(entry.path())
            .map_err(|path| ledger_failure(format!("{} is not UTF-8", path.display())))?;
        if path.is_dir() {
            collect_lean(root, &path, found)?;
            continue;
        }
        if path.extension() != Some("lean") {
            continue;
        }
        let relative = path
            .strip_prefix(root)
            .map_err(|error| ledger_failure(format!("{}: {error}", path.as_str())))?
            .to_string();
        let text = std::fs::read_to_string(&path)
            .map_err(|error| ledger_failure(format!("{}: {error}", path.as_str())))?;
        found.push((relative, text));
    }
    Ok(())
}

/// `lexeme hash`.
fn hash(
    arguments: &LexemeHashArgs,
    working_directory: &Utf8Path,
) -> Result<LexemeOutcome, LexLeanError> {
    let root = resolve(working_directory, &arguments.source_root);
    let mut sources: Vec<(String, String)> = Vec::with_capacity(arguments.sources.len());
    for source in &arguments.sources {
        let path = resolve(working_directory, source);
        let relative = path
            .strip_prefix(&root)
            .map_or_else(|_| source.to_string(), |rest| rest.to_string());
        let text = std::fs::read_to_string(&path).map_err(|error| {
            source_failure(format!("{}: {error}", path.as_str()), relative.clone())
        })?;
        canonical::canonicalize(&text)
            .map_err(|(reason, line)| source_failure(reason, format!("{relative}:{line}")))?;
        sources.push((relative, text));
    }
    let entry = Entry::new(
        &arguments.title,
        &arguments.toolchain,
        &sources
            .iter()
            .map(|(path, text)| (path.as_str(), text.as_str()))
            .collect::<Vec<(&str, &str)>>(),
    )
    .map_err(|(reason, location)| source_failure(reason, location))?;
    let out = resolve(working_directory, &arguments.out);
    write_json(&out, &entry.to_json())?;
    Ok(LexemeOutcome {
        modules: sources.iter().map(|(path, _)| path.clone()).collect(),
        artifacts: vec![out.to_string()],
        summary: format!(
            "content digest sha256:{} over {} declarations from {} sources ({})\n",
            entry.content_digest,
            entry.declarations.len(),
            sources.len(),
            arguments.toolchain
        ),
        payload: Some(entry.to_json()),
        ..LexemeOutcome::default()
    })
}

/// `lexeme sign`.
fn sign(
    arguments: &LexemeSignArgs,
    working_directory: &Utf8Path,
) -> Result<LexemeOutcome, LexLeanError> {
    let path = resolve(working_directory, &arguments.entry);
    let mut entry = read_entry(&path)?;
    let digest = entry.content_digest.0;
    let signature = match (&arguments.seed, &arguments.piv_slot) {
        (Some(seed), _) => {
            let seed_path = resolve(working_directory, seed);
            let bytes = std::fs::read(&seed_path).map_err(|error| {
                ledger_failure(format!("{}: {error}", seed_path.as_str()))
            })?;
            signature::sign_with_seed(&digest, &bytes)?
        }
        (None, Some(slot)) => signature::sign_with_piv(
            &digest,
            slot,
            &arguments.piv_tool,
            arguments.pin.as_deref(),
        )?,
        (None, None) => {
            return Err(usage(
                "signing needs either --seed for a software key or --piv-slot with --pin for a device key",
            ))
        }
    };
    entry.with_signature(signature);
    let out = arguments
        .out
        .as_ref()
        .map_or_else(|| path.clone(), |target| resolve(working_directory, target));
    write_json(&out, &entry.to_json())?;
    Ok(LexemeOutcome {
        artifacts: vec![out.to_string()],
        summary: format!(
            "signed content digest sha256:{} with {}\n",
            entry.content_digest,
            entry.signature.key.kind.as_str()
        ),
        payload: Some(entry.to_json()),
        ..LexemeOutcome::default()
    })
}

/// `lexeme stamp`.
fn stamp(arguments: &LexemeStampArgs) -> Result<LexemeOutcome, LexLeanError> {
    let path = &arguments.entry;
    let mut entry = read_entry(path)?;

    if let Some(request) = &arguments.request {
        let parsed = entry.signature.timestamplable_bytes().to_vec();
        let digest = timestamp::digest_for_algorithm(timestamp::preferred_digest_oid(), &parsed)
            .ok_or_else(|| ledger_failure("the preferred digest algorithm is unavailable"))?;
        let der = timestamp::build_timestamp_request(&digest, timestamp::preferred_digest_oid())
            .map_err(ledger_failure)?;
        std::fs::write(request, &der)
            .map_err(|error| ledger_failure(format!("{}: {error}", request.as_str())))?;
        let query = crate::artifact::canonical_json::Json::object(vec![
            (
                "spec",
                crate::artifact::canonical_json::Json::Str("lexlean/tsa-query/1".to_owned()),
            ),
            (
                "tsa",
                crate::artifact::canonical_json::Json::Str(arguments.tsa.clone()),
            ),
            (
                "artifact",
                crate::artifact::canonical_json::Json::Str(TIMESTAMP_ARTIFACT.to_owned()),
            ),
            (
                "hash_algorithm",
                crate::artifact::canonical_json::Json::Str(
                    timestamp::preferred_digest_oid().to_owned(),
                ),
            ),
            (
                "hashed_message",
                crate::artifact::canonical_json::Json::Str(signature::hex_lower(&digest)),
            ),
        ]);
        let sidecar = request.with_extension("tsq.json");
        write_json(&sidecar, &query)?;
        return Ok(LexemeOutcome {
            artifacts: vec![request.to_string(), sidecar.to_string()],
            summary: format!(
                "wrote an RFC 3161 request over sha256:{} of the {} bytes of the detached signature; obtain the token from {} and attach it with --tsr\n",
                signature::hex_lower(&digest),
                parsed.len(),
                arguments.tsa
            ),
            ..LexemeOutcome::default()
        });
    }

    let (Some(tsr), Some(certificate), Some(root)) = (
        arguments.tsr.as_ref(),
        arguments.certificate.as_ref(),
        arguments.root.as_ref(),
    ) else {
        return Err(usage(
            "stamp needs either --request to build an RFC 3161 request, or --tsr with --certificate and --root to attach a token",
        ));
    };
    let mut anchor = Timestamp {
        tsa: arguments.tsa.clone(),
        gen_time: String::new(),
        artifact: TIMESTAMP_ARTIFACT.to_owned(),
        tsr_der: std::fs::read(tsr)
            .map_err(|error| ledger_failure(format!("{}: {error}", tsr.as_str())))?,
        tsa_certificate: std::fs::read(certificate)
            .map_err(|error| ledger_failure(format!("{}: {error}", certificate.as_str())))?,
        tsa_root: std::fs::read(root)
            .map_err(|error| ledger_failure(format!("{}: {error}", root.as_str())))?,
    };
    // Check the token before recording it: an entry that carries a token which
    // does not verify is worse than one that carries none, because it looks
    // timed.
    //
    // The code is carried over rather than restated as `LLG1005`. Prefixing the
    // reason is right, because a token that does not verify before it is
    // recorded is the fact being reported, but restating the code would report
    // an untrusted chain and a missing verifier as a malformed token, and those
    // three are registered apart.
    anchor.verify(&parsed_signature(&entry)?).map_err(|error| {
        let Some(first) = error.diagnostics.first() else {
            return ledger_failure("the token does not verify before it is recorded");
        };
        LexLeanError::from_diagnostic(Diagnostic::new(
            first.code,
            format!(
                "the token does not verify before it is recorded: {}",
                first.message
            ),
        ))
    })?;
    anchor.gen_time = anchor
        .tst_info()
        .map_err(|error| {
            LexLeanError::from_diagnostic(Diagnostic::new(
                crate::code!("LLG1005"),
                error.diagnostics[0].message.clone(),
            ))
        })?
        .gen_time;
    entry.with_timestamp(anchor.clone());
    let out = arguments.out.as_ref().unwrap_or(path);
    write_json(out, &entry.to_json())?;
    Ok(LexemeOutcome {
        artifacts: vec![out.to_string()],
        summary: format!(
            "recorded an RFC 3161 anchor at {} by {}\n",
            anchor.gen_time, anchor.tsa
        ),
        payload: Some(entry.to_json()),
        ..LexemeOutcome::default()
    })
}

/// The signature bytes an entry's token must cover.
fn parsed_signature(entry: &Entry) -> Result<Vec<u8>, LexLeanError> {
    if !entry.signature.well_formed() {
        return Err(usage(
            "an entry must be signed before it can be stamped: the token covers the detached signature",
        ));
    }
    Ok(entry.signature.timestamplable_bytes().to_vec())
}

/// `lexeme append`.
fn append(
    arguments: &LexemeAppendArgs,
    working_directory: &Utf8Path,
) -> Result<LexemeOutcome, LexLeanError> {
    let ledger_path = resolve(working_directory, &arguments.ledger);
    let entry_path = resolve(working_directory, &arguments.entry);
    let entry = read_entry(&entry_path)?;
    let key_path = resolve(working_directory, &arguments.log_key);
    let log_key = std::fs::read(&key_path)
        .map_err(|error| ledger_failure(format!("{}: {error}", key_path.as_str())))?;
    let mut ledger = Ledger::open(ledger_path.as_std_path())?;
    let appended = ledger.append(entry, &log_key)?;
    verify::verify_heads(&ledger)?;
    // The entry is rewritten carrying the proof the append produced, so that
    // the artifact a reader holds is the one whose four remaining claims can be
    // checked without the ledger directory.
    let out = arguments.out.as_ref().map_or_else(
        || entry_path.clone(),
        |target| resolve(working_directory, target),
    );
    write_json(&out, &appended.to_json())?;
    Ok(LexemeOutcome {
        artifacts: vec![out.to_string(), ledger_path.to_string()],
        summary: format!(
            "appended leaf {} of {}; root sha256:{}\n",
            appended
                .inclusion
                .as_ref()
                .map_or(0, |inclusion| inclusion.leaf_index),
            appended
                .inclusion
                .as_ref()
                .map_or(0, |inclusion| inclusion.tree_size),
            appended
                .inclusion
                .as_ref()
                .map_or_else(|| "-".to_owned(), |inclusion| inclusion.root_hash.to_hex())
        ),
        payload: Some(appended.to_json()),
        ..LexemeOutcome::default()
    })
}

/// `lexeme verify`.
fn verify(
    arguments: &LexemeVerifyArgs,
    working_directory: &Utf8Path,
) -> Result<LexemeOutcome, LexLeanError> {
    let entry = read_entry(&resolve(working_directory, &arguments.entry))?;
    let ledger = arguments
        .ledger
        .as_ref()
        .map(|path| Ledger::open(resolve(working_directory, path).as_std_path()))
        .transpose()?;
    let verdict: Verdict = verify::verify_entry(&entry, ledger.as_ref())?;
    // A negative verdict is a report, not a refusal: the payload carries the
    // five checks, and the exit code carries the registered code of the first
    // one that failed, so a caller can branch on the reason without parsing.
    let failure = if verdict.verified() {
        None
    } else {
        Some(verdict.failure())
    };
    Ok(LexemeOutcome {
        summary: format!("{}\n", verdict.statement()),
        payload: Some(verdict.to_json()),
        failure,
        ..LexemeOutcome::default()
    })
}

/// `lexeme bootstrap`: the ledger's own specification as its first entry.
fn bootstrap(
    arguments: &LexemeBootstrapArgs,
    working_directory: &Utf8Path,
) -> Result<LexemeOutcome, LexLeanError> {
    let specification = resolve(working_directory, &arguments.specification);
    let lean = lean_sources(&specification)?;
    if lean.is_empty() {
        return Err(source_failure(
            "the specification directory holds no Lean source".to_owned(),
            specification.to_string(),
        ));
    }
    let mut sources: Vec<(String, String)> = lean;
    // The CLI and the browser verifier are the two artifacts a third party
    // needs, so the bootstrap entry commits to their bytes the same way it
    // commits to the specification's.
    for artifact_root in &arguments.artifacts {
        let path = resolve(working_directory, artifact_root);
        let files = plain_files(&path)?;
        sources.extend(files);
    }
    let mut entry = Entry::new(
        &arguments.title,
        &arguments.toolchain,
        &sources
            .iter()
            .map(|(path, text)| (path.as_str(), text.as_str()))
            .collect::<Vec<(&str, &str)>>(),
    )
    .map_err(|(reason, location)| source_failure(reason, location))?;

    let ledger_path = resolve(working_directory, &arguments.ledger);
    std::fs::create_dir_all(ledger_path.as_std_path())
        .map_err(|error| ledger_failure(format!("{}: {error}", ledger_path.as_str())))?;
    let mut ledger = Ledger::open(ledger_path.as_std_path())?;

    if let Some(seed) = &arguments.seed {
        let seed_path = resolve(working_directory, seed);
        let bytes = std::fs::read(seed_path.as_std_path()).map_err(|error| {
            ledger_failure(format!("{}: {error}", seed_path.as_std_path().display()))
        })?;
        let digest = entry.content_digest.0;
        entry.with_signature(signature::sign_with_seed(&digest, &bytes)?);
    } else {
        return Err(usage(
            "bootstrap needs --seed: the first entry is the one whose authorship the tool claims for itself",
        ));
    }
    let log_key = match &arguments.log_key {
        Some(path) => {
            let key_path = resolve(working_directory, path);
            std::fs::read(&key_path).map_err(|error| {
                ledger_failure(format!("{}: {error}", key_path.as_str()))
            })?
        }
        None => return Err(usage(
            "bootstrap needs --log-key: §33.7 requires every published head to be signed by the log key",
        )),
    };
    let appended = ledger.append(entry, &log_key)?;
    verify::verify_heads(&ledger)?;
    let out = resolve(working_directory, &arguments.out);
    write_json(&out, &appended.to_json())?;
    Ok(LexemeOutcome {
        modules: sources.iter().map(|(path, _)| path.clone()).collect(),
        artifacts: vec![out.to_string(), ledger_path.to_string()],
        summary: format!(
            "bootstrapped `{}` as leaf 0 of {}; root sha256:{}\n",
            arguments.title,
            appended
                .inclusion
                .as_ref()
                .map_or(0, |inclusion| inclusion.tree_size),
            appended
                .inclusion
                .as_ref()
                .map_or_else(|| "-".to_owned(), |inclusion| inclusion.root_hash.to_hex())
        ),
        payload: Some(appended.to_json()),
        ..LexemeOutcome::default()
    })
}

/// Every file under a directory, as `(relative path, bytes)`, path-ordered.
fn plain_files(root: &Utf8Path) -> Result<Vec<(String, String)>, LexLeanError> {
    let mut found: Vec<(String, String)> = Vec::new();
    collect_plain(root, root, &mut found)?;
    found.sort_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
    Ok(found)
}

/// Walk one directory for the bootstrap artifact set.
fn collect_plain(
    root: &Utf8Path,
    directory: &Utf8Path,
    found: &mut Vec<(String, String)>,
) -> Result<(), LexLeanError> {
    let entries = std::fs::read_dir(directory)
        .map_err(|error| ledger_failure(format!("{}: {error}", directory.as_str())))?;
    for entry in entries.flatten() {
        let path = Utf8PathBuf::from_path_buf(entry.path())
            .map_err(|path| ledger_failure(format!("{} is not UTF-8", path.display())))?;
        if path.is_dir() {
            collect_plain(root, &path, found)?;
            continue;
        }
        let relative = path
            .strip_prefix(root)
            .map_err(|error| ledger_failure(format!("{}: {error}", path.as_str())))?
            .to_string();
        let text = std::fs::read_to_string(&path)
            .map_err(|error| ledger_failure(format!("{}: {error}", path.as_str())))?;
        found.push((relative, text));
    }
    Ok(())
}
