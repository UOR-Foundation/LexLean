//! The production export harness (SPEC.md §24.7): the committed applications
//! and expected packages under `tests/production-export/`, the exports the
//! `production-export` suite shares, and the GNAF evidence built around a
//! production root. `cargo xtask check-production-export` reports the same
//! comparison `conformance_px_01` enforces, and `--write` is the only path
//! that rewrites the expected packages.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use camino::{Utf8Path, Utf8PathBuf};
use lexlean::api::production::{ProductionOutput, ReplayReport};
use lexlean::Sha256Digest;

use crate::production_consumer::{self, AppEvidence, Application};
use crate::support::{self, P};

/// The examples the consumer exports.
pub const EXAMPLES: [&str; 2] = ["production", "production-coverage"];

/// The root the GNAF evidence is built for, and where it is written.
pub const GNAF_ROOT: &str = "Production.Main.sumAll";
/// The project-relative GNAF request path.
pub const GNAF_REQUEST: &str = "evidence/sum.request.json";
/// The project-relative GNAF certificate path.
pub const GNAF_CERTIFICATE: &str = "evidence/sum.certificate.json";

/// The committed directory of one example's application and packages.
#[must_use]
pub fn fixture_dir(example: &str) -> Utf8PathBuf {
    support::repo_root()
        .join("tests/production-export")
        .join(example)
}

/// The committed application of one example.
///
/// # Panics
///
/// When `roots.json` is missing or malformed.
#[must_use]
pub fn application(example: &str) -> Application {
    let path = fixture_dir(example).join("roots.json");
    serde_json::from_slice(&std::fs::read(path.as_std_path()).expect("roots.json"))
        .expect("roots.json is an application")
}

/// GNAF evidence for a production root: a request whose reference program is
/// the root's lowered program, whose plans compute the same function, and a
/// certificate the host checker certifies.
pub struct Gnaf {
    /// The request's bytes.
    pub request: Vec<u8>,
    /// The certificate's bytes.
    pub certificate: Vec<u8>,
    /// The certificate's identity.
    pub certificate_id: Sha256Digest,
    /// The verdict the checker gives it.
    pub verdict: serde_json::Value,
}

/// The lowered program of one production root of `project`.
///
/// # Panics
///
/// When the root is not a production root or does not lower.
#[must_use]
pub fn lowered_program(project: &P, root: &str) -> lexlean::calculus::Program {
    let checked = support::checked_project(project);
    let linked = lexlean::production::lower::linked_modules(&checked);
    let roots = lexlean::production::lower::roots(&checked).expect("roots");
    let found = roots
        .iter()
        .find(|candidate| candidate.report.root == root)
        .unwrap_or_else(|| panic!("`{root}` is a production root"));
    lexlean::production::lower::lower_root(
        &linked,
        &found.module,
        &found.name,
        found.report,
        &support::limits(project),
    )
    .expect("the root lowers")
    .program
}

/// Build GNAF evidence around `sumAll`: the committed `argmin-complete`
/// request with its reference replaced by the root's lowered program, its
/// domain by lists of naturals, and its two plans by the root's own summing
/// function; the certificate measures every system and bounds the universe
/// by the argmin the host model computes.
///
/// # Panics
///
/// When the template is missing or the host model refuses the request.
#[must_use]
pub fn gnaf_evidence(project: &P) -> Gnaf {
    use lexlean::gnaf::certification::{self, Certificate, Premise, Route};
    let program = lowered_program(project, GNAF_ROOT);
    let template: serde_json::Value = serde_json::from_slice(
        &std::fs::read(
            support::repo_root()
                .join("compiler/gnaf/argmin-complete.json")
                .as_std_path(),
        )
        .expect("the committed GNAF fixture"),
    )
    .expect("fixture JSON");
    let mut request = template["request"].clone();
    let reference = serde_json::to_value(&program).expect("program JSON");
    let summing = reference["functions"][1].clone();
    let nats = |values: &[u64]| {
        serde_json::json!({
            "kind": "list",
            "items": values.iter().map(|value| serde_json::json!({"kind": "nat", "value": value.to_string()})).collect::<Vec<_>>(),
        })
    };
    request["reference"] = reference;
    request["domain"] = serde_json::json!([
        nats(&[7]),
        nats(&[3, 4]),
        nats(&[1, 2, 3, 4, 5, 6]),
        nats(&[]),
    ]);
    request["carrier"]["result"] = serde_json::json!({"kind": "nat"});
    request["carrier"]["plans"] = serde_json::json!([
        {"function": summing.clone(), "prepares": []},
        {"function": summing, "prepares": []},
    ]);
    request["universe"] = serde_json::json!("0".repeat(64));
    let mut parsed: lexlean::gnaf::Request =
        serde_json::from_value(request).expect("a request value");
    parsed.universe = lexlean::gnaf::universe_id(&parsed).expect("universe");
    let request_bytes = canonical(&parsed);
    let loaded = lexlean::gnaf::load(&request_bytes).expect("the request is valid");
    let (members, steps) = match lexlean::gnaf::answer(&loaded).expect("an answer") {
        lexlean::gnaf::Answer::Argmin { members, steps } => (members, steps),
        other => panic!("the sum universe has an argmin: {other:?}"),
    };
    let systems = match &loaded.carrier {
        carrier @ lexlean::gnaf::Carrier::Grammar { .. } => lexlean::gnaf::expand(carrier).len(),
        other => panic!("a grammar carrier: {other:?}"),
    };
    let certificate = Certificate {
        spec: "lexlean/gnaf-certificate/1".to_owned(),
        verifier: "lexlean/gnaf-certification/1".to_owned(),
        universe: loaded.universe,
        objective: loaded.objective,
        claim: loaded.claim.clone(),
        scope: loaded.scope,
        route: Route::LowerBound {
            bound: steps,
            attainers: members,
        },
        premises: vec![Premise::Measured; systems],
    };
    let certificate_bytes = canonical(&certificate);
    let certificate_id = certification::certificate_id(&certificate).expect("identity");
    let verdict = certification::check(&loaded, &certificate).expect("a verdict");
    assert!(
        matches!(verdict, certification::Verdict::Certified { .. }),
        "the sum certificate is certified: {verdict:?}"
    );
    Gnaf {
        request: request_bytes,
        certificate: certificate_bytes,
        certificate_id,
        verdict: serde_json::to_value(&verdict).expect("verdict JSON"),
    }
}

/// Canonical JSON file bytes of a serializable value.
///
/// # Panics
///
/// When the value has no canonical JSON form.
#[must_use]
pub fn canonical<T: serde::Serialize>(value: &T) -> Vec<u8> {
    lexlean::artifact::canonical_json::Json::parse(
        serde_json::to_string(value).expect("serializes").as_bytes(),
    )
    .expect("canonical")
    .to_file_bytes()
}

/// One example exported by the consumer.
pub struct Exported {
    /// The project copy.
    pub project: P,
    /// The build's output.
    pub output: ProductionOutput,
    /// The replay of the published bundle.
    pub report: ReplayReport,
    /// The GNAF evidence offered, for the example that carries one.
    pub gnaf: Option<Gnaf>,
}

impl Exported {
    /// The absolute bundle directory.
    #[must_use]
    pub fn bundle(&self) -> Utf8PathBuf {
        self.project.root.join(&self.output.bundle)
    }
}

/// Export one example through the public consumer, with GNAF evidence for
/// `sumAll` in `examples/production`.
///
/// # Errors
///
/// The contract's failure, as text.
pub fn export(example: &str) -> Result<Exported, String> {
    // The export reads the environment (the toolchain, `PATH`), which a case
    // that sets it for a moment must not change under it.
    let _environment = support::env_shared();
    let project = P::copy_example(example);
    let gnaf = (example == "production").then(|| gnaf_evidence(&project));
    let mut evidence = Vec::new();
    if let Some(gnaf) = &gnaf {
        project.write(
            GNAF_REQUEST,
            std::str::from_utf8(&gnaf.request).expect("utf8"),
        );
        project.write(
            GNAF_CERTIFICATE,
            std::str::from_utf8(&gnaf.certificate).expect("utf8"),
        );
        evidence.push(AppEvidence {
            root: GNAF_ROOT.to_owned(),
            request: GNAF_REQUEST.to_owned(),
            certificate: GNAF_CERTIFICATE.to_owned(),
            certificate_id: gnaf.certificate_id,
        });
    }
    let (output, report) = production_consumer::export(
        &project.root.join("lexlean.toml"),
        &application(example),
        &evidence,
    )
    .map_err(|error| format!("{example}: {:?}", error.diagnostics))?;
    Ok(Exported {
        project,
        output,
        report,
        gnaf,
    })
}

/// The consumer's export of one example, shared by the suite's cases.
///
/// # Panics
///
/// When the example does not export.
#[must_use]
pub fn exported(example: &str) -> &'static Exported {
    static PRODUCTION: OnceLock<Exported> = OnceLock::new();
    static COVERAGE: OnceLock<Exported> = OnceLock::new();
    let cell = match example {
        "production" => &PRODUCTION,
        "production-coverage" => &COVERAGE,
        other => panic!("`{other}` is not an exported example"),
    };
    cell.get_or_init(|| export(example).unwrap_or_else(|reason| panic!("{reason}")))
}

/// Every file under `root`, by `/`-separated relative path.
#[must_use]
pub fn tree(root: &Utf8Path) -> BTreeMap<String, Vec<u8>> {
    let mut out = BTreeMap::new();
    for entry in walkdir::WalkDir::new(root.as_std_path())
        .into_iter()
        .flatten()
    {
        if entry.file_type().is_file() {
            let relative = entry
                .path()
                .strip_prefix(root.as_std_path())
                .expect("under root")
                .to_string_lossy()
                .replace('\\', "/");
            out.insert(relative, std::fs::read(entry.path()).expect("read"));
        }
    }
    out
}

/// Compare every exported package with the committed expected packages, or
/// with `write`, replace the expected packages with the exported ones.
///
/// # Errors
///
/// Every difference, or the export's failure.
pub fn check(write: bool) -> Result<(), String> {
    let mut unmet = Vec::new();
    for example in EXAMPLES {
        let exported = export(example)?;
        let packages = tree(&exported.bundle().join("packages"));
        let expected_root = fixture_dir(example).join("packages");
        if write {
            let _ = std::fs::remove_dir_all(expected_root.as_std_path());
            for (relative, bytes) in &packages {
                let path = expected_root.join(relative);
                std::fs::create_dir_all(path.parent().expect("parent").as_std_path())
                    .map_err(|error| error.to_string())?;
                std::fs::write(path.as_std_path(), bytes).map_err(|error| error.to_string())?;
            }
            continue;
        }
        let expected = tree(&expected_root);
        for (relative, bytes) in &packages {
            match expected.get(relative) {
                Some(committed) if committed == bytes => {}
                Some(_) => unmet.push(format!("{example}: packages/{relative} differs")),
                None => unmet.push(format!("{example}: packages/{relative} is not committed")),
            }
        }
        for relative in expected.keys() {
            if !packages.contains_key(relative) {
                unmet.push(format!("{example}: packages/{relative} was not exported"));
            }
        }
    }
    if unmet.is_empty() {
        Ok(())
    } else {
        Err(unmet.join("\n"))
    }
}
