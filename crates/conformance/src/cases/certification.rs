//! The `gnaf-certification` suite: GC-01..GC-10, GNAF certification over
//! requests (SPEC.md §17.18).

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::sync::OnceLock;

use lexlean::gnaf::certification::{
    self as certification, Bridge, Certificate, CertificateFixture, Certified, Established,
    Obligation, Premise, Refusal, Route, Schema, Trial, Verdict, CERTIFICATE_SPEC, FIXTURE_SPEC,
    MAX_NUMBER, MAX_POSITIONS, MAX_PREMISES, VERIFIER,
};
use lexlean::gnaf::{
    self, Answer, ClaimClass, Objective, Rejection, Request, Scope, Selector, Status,
};
use lexlean::Sha256Digest;
use serde_json::{json, Value as Json};

use crate::certification as generated;
use crate::certification_library as library;
use crate::support::{self, repo_root};

/// The committed fixtures, read from disk exactly as published.
fn committed() -> Vec<(PathBuf, Vec<u8>, CertificateFixture)> {
    let dir = repo_root().join(generated::DIRECTORY);
    let mut out = Vec::new();
    for entry in std::fs::read_dir(dir.as_std_path()).expect("compiler/certificates exists") {
        let path = entry.expect("entry").path();
        let bytes = std::fs::read(&path).expect("fixture bytes");
        let fixture: CertificateFixture = serde_json::from_slice(&bytes)
            .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        out.push((path, bytes, fixture));
    }
    out.sort_by(|left, right| left.0.cmp(&right.0));
    assert!(
        !out.is_empty(),
        "GNAF certification has hand-constructed certificates"
    );
    out
}

/// The certification model, a source.
const MODEL: &str = "compiler/src/Certification.lex.tex";

fn read(relative: &str) -> String {
    std::fs::read_to_string(repo_root().join(relative).as_std_path())
        .unwrap_or_else(|error| panic!("{relative}: {error}"))
}

fn read_json(relative: &str) -> Json {
    serde_json::from_str(&read(relative)).unwrap_or_else(|error| panic!("{relative}: {error}"))
}

fn schema(name: &str) -> Json {
    read_json(&format!("schemas/{name}"))
}

/// A committed fixture by name.
fn fixture(name: &str) -> CertificateFixture {
    static FIXTURES: OnceLock<Vec<CertificateFixture>> = OnceLock::new();
    FIXTURES
        .get_or_init(|| {
            committed()
                .into_iter()
                .map(|(_, _, fixture)| fixture)
                .collect()
        })
        .iter()
        .find(|fixture| fixture.name == name)
        .unwrap_or_else(|| panic!("no certificate fixture {name}"))
        .clone()
}

/// The GNAF requests, generated once: generating them is the expensive part
/// of every host check.
fn requests() -> &'static Vec<(String, Request)> {
    static REQUESTS: OnceLock<Vec<(String, Request)>> = OnceLock::new();
    REQUESTS.get_or_init(|| {
        crate::gnaf::fixtures()
            .into_iter()
            .map(|fixture| (fixture.name, fixture.request))
            .collect()
    })
}

fn request(name: &str) -> Request {
    requests()
        .iter()
        .find(|(candidate, _)| candidate == name)
        .unwrap_or_else(|| panic!("no GNAF fixture {name}"))
        .1
        .clone()
}

/// The verdict the host gives a committed fixture's certificate and request.
fn rerun(fixture: &CertificateFixture) -> Verdict {
    certification::check(&request(&fixture.request), &fixture.certificate)
        .unwrap_or_else(|error| panic!("{}: {error}", fixture.name))
}

/// The verdict the host gives a committed fixture, recomputed.
fn verdict(name: &str) -> Verdict {
    let fixture = fixture(name);
    let recomputed = rerun(&fixture);
    assert_eq!(
        recomputed, fixture.expected,
        "{name}: the committed verdict"
    );
    recomputed
}

/// The verdict of `certificate` for the request named `name`.
fn check(name: &str, certificate: &Certificate) -> Verdict {
    certification::check(&request(name), certificate).expect("within capacity")
}

fn incomplete(obligation: Obligation, established: Established) -> Verdict {
    Verdict::Incomplete {
        obligation,
        established,
    }
}

fn refused(refusal: Refusal) -> Verdict {
    Verdict::Refused { refusal }
}

fn certified(result: Certified) -> Verdict {
    Verdict::Certified { result }
}

fn is_certified(verdict: &Verdict) -> bool {
    matches!(verdict, Verdict::Certified { .. })
}

/// A load expected to fail closed with `code` naming `message`.
fn refuses(bytes: &[u8], code: &str, message: &str) {
    let error = certification::load(bytes)
        .err()
        .unwrap_or_else(|| panic!("an invalid certificate loads: expected `{message}`"));
    support::expect_code(&error, code);
    assert!(
        error.to_string().contains(message),
        "expected `{message}` in: {error}"
    );
}

fn bytes(json: &Json) -> Vec<u8> {
    serde_json::to_vec(json).expect("serializes")
}

fn certificate_json(name: &str) -> Json {
    serde_json::to_value(fixture(name).certificate).expect("certificate serializes")
}

/// The semantic data of a committed `compiler/src` module.
fn module_data(text: &str) -> Json {
    crate::support::module_json(text)
}

/// Every declaration of a committed module by name.
fn declarations(path: &str) -> Vec<Json> {
    module_data(&read(path))["declarations"]
        .as_array()
        .expect("declarations")
        .clone()
}

/// Every name a term references.
fn references(term: &Json, out: &mut BTreeSet<String>) {
    match term {
        Json::Object(map) => {
            for (key, value) in map {
                if matches!(key.as_str(), "function" | "constructor") {
                    if let Some(name) = value.get("name").and_then(Json::as_str) {
                        out.insert(name.to_owned());
                    }
                }
                references(value, out);
            }
        }
        Json::Array(items) => items.iter().for_each(|item| references(item, out)),
        Json::Null | Json::Bool(_) | Json::Number(_) | Json::String(_) => {}
    }
}

/// Run the case for one GC conformance ID.
///
/// # Panics
///
/// Panics when the capability does not hold.
pub fn run(id: &str) {
    match id {
        "GC-01" => gc_01(),
        "GC-02" => gc_02(),
        "GC-03" => gc_03(),
        "GC-04" => gc_04(),
        "GC-05" => gc_05(),
        "GC-06" => gc_06(),
        "GC-07" => gc_07(),
        "GC-08" => gc_08(),
        "GC-09" => gc_09(),
        "GC-10" => gc_10(),
        "GC-11" => gc_11(),
        _ => panic!("no GNAF certification case {id}"),
    }
}

/// §17.18: closed certificate form, canonical fixtures, content-addressed
/// identity, fail-closed load, and the host's capacities.
#[allow(clippy::too_many_lines)]
fn gc_01() {
    let certificate_schema = schema("gnaf-certificate.schema.json");
    let fixture_schema = schema("gnaf-certificate-fixture.schema.json");
    let mut routes = BTreeSet::new();
    for (path, committed_bytes, fixture) in committed() {
        let name = path
            .file_stem()
            .expect("stem")
            .to_string_lossy()
            .into_owned();
        assert_eq!(fixture.name, name, "a fixture is named by its file");
        assert_eq!(fixture.spec, FIXTURE_SPEC);
        assert_eq!(fixture.certificate.spec, CERTIFICATE_SPEC);
        // The one fixture whose point is a foreign verifier tag.
        if name == "verifier-mismatch" {
            assert_ne!(fixture.certificate.verifier, VERIFIER, "{name}");
        } else {
            assert_eq!(fixture.certificate.verifier, VERIFIER, "{name}");
        }
        assert_eq!(
            committed_bytes,
            gnaf::to_file_bytes(&fixture),
            "{name}: committed bytes are canonical"
        );
        let json: Json = serde_json::from_slice(&committed_bytes).expect("json");
        let violations = crate::schema::validate(&fixture_schema, &json);
        assert!(violations.is_empty(), "{name}: {violations:?}");
        let violations = crate::schema::validate(&certificate_schema, &json["certificate"]);
        assert!(violations.is_empty(), "{name}: {violations:?}");
        let loaded = certification::load_fixture(&committed_bytes)
            .unwrap_or_else(|error| panic!("{name}: {error}"));
        assert_eq!(
            loaded.id,
            certification::certificate_id(&loaded.certificate).expect("an identity"),
            "{name}: the stated identity is the certificate's"
        );
        // The certificate binds the universe of the request it names.
        let posed = request(&fixture.request);
        assert!(
            fixture.certificate.universe == posed.universe
                || ["reused-enlarged"].contains(&name.as_str()),
            "{name}: the certificate states its request's universe identity"
        );
        assert_eq!(rerun(&fixture), fixture.expected, "{name}: the verdict");
        routes.insert(match fixture.certificate.route {
            Route::LowerBound { .. } => "lower_bound",
            Route::FrontierCover { .. } => "frontier_cover",
            Route::Reduction => "reduction",
            Route::Representation { .. } => "representation",
            Route::Normality { .. } => "normality",
        });
    }
    assert_eq!(
        routes.len(),
        5,
        "every route has a committed certificate: {routes:?}"
    );
    crate::calculus::check(repo_root().as_std_path(), false)
        .expect("the committed certificates and schemas equal their generator");
    // The schemas share the GNAF request schema's claim and rejection
    // definitions, taken from one place.
    let request_schema = schema("gnaf-fixture.schema.json");
    for definition in ["claim", "rejection", "digest", "action_kind"] {
        assert_eq!(
            fixture_schema["$defs"][definition], request_schema["$defs"][definition],
            "{definition} is the request fixture's own definition"
        );
    }

    let base = certificate_json("lb-argmin-over-systems");
    refuses(b"{", "LLB6007", "malformed");
    let mut unknown = base.clone();
    unknown["optimizer"] = json!("fastest");
    refuses(&bytes(&unknown), "LLB6007", "unknown field");
    let mut spec = base.clone();
    spec["spec"] = json!("lexlean/gnaf-certificate/0");
    refuses(
        &bytes(&spec),
        "LLB6007",
        "expected `lexlean/gnaf-certificate/1`",
    );
    let mut premise = base.clone();
    premise["premises"][0] = json!({"kind": "excluded"});
    refuses(&bytes(&premise), "LLB6007", "malformed");
    let mut route = base.clone();
    route["route"] = json!({"kind": "optimizer"});
    refuses(&bytes(&route), "LLB6007", "malformed");
    let mut not_hex = base.clone();
    not_hex["universe"] = json!("UNIVERSE");
    refuses(&bytes(&not_hex), "LLB6007", "malformed");
    let mut negative = base.clone();
    negative["route"]["bound"] = json!(-1);
    refuses(&bytes(&negative), "LLB6007", "malformed");
    for invalid in [&unknown, &spec, &premise, &route, &not_hex, &negative] {
        assert!(
            !crate::schema::validate(&certificate_schema, invalid).is_empty(),
            "the schema refuses what the loader refuses: {invalid}"
        );
    }
    // A fixture stating another identity is refused, however it is edited.
    let committed_fixture = fixture("lb-argmin-over-systems");
    let mut forged = serde_json::to_value(&committed_fixture).expect("json");
    forged["id"] = json!(Sha256Digest::of(b"another certificate").to_hex());
    let error = certification::load_fixture(&bytes(&forged)).expect_err("a forged identity");
    support::expect_code(&error, "LLB6007");
    assert!(error.to_string().contains("identity"), "{error}");
    let mut edited = serde_json::to_value(&committed_fixture).expect("json");
    edited["certificate"]["route"]["bound"] = json!(166);
    let error = certification::load_fixture(&bytes(&edited)).expect_err("an edited certificate");
    support::expect_code(&error, "LLB6007");
    let mut fixture_spec = serde_json::to_value(&committed_fixture).expect("json");
    fixture_spec["spec"] = json!("lexlean/gnaf-certificate-fixture/0");
    support::expect_code(
        &certification::load_fixture(&bytes(&fixture_spec)).expect_err("a spec"),
        "LLB6007",
    );
    assert!(
        !crate::schema::validate(
            &fixture_schema,
            &serde_json::to_value(&forged).expect("json")
        )
        .is_empty()
            || forged["id"] != json!(committed_fixture.id.to_hex()),
        "a forged identity is a different document"
    );

    // The capacities: premises, positions in a list, and numbers.
    let mut premises = base.clone();
    premises["premises"] = json!(vec![json!({"kind": "measured"}); MAX_PREMISES as usize + 1]);
    refuses(&bytes(&premises), "LLS8002", "premises");
    let mut positions = base.clone();
    positions["premises"][0] = json!({"kind": "pays_at_least",
        "positions": (0..=MAX_POSITIONS).collect::<Vec<u64>>()});
    refuses(&bytes(&positions), "LLS8002", "positions");
    let mut number = base.clone();
    number["route"]["bound"] = json!(MAX_NUMBER + 1);
    refuses(&bytes(&number), "LLS8002", "bound");
    let mut attainer = base.clone();
    attainer["route"]["attainers"] = json!([MAX_NUMBER + 1]);
    refuses(&bytes(&attainer), "LLS8002", "attainers");
    let mut slope = certificate_json("bridge-attained");
    slope["route"]["bridge"]["slope"] = json!(MAX_NUMBER + 1);
    refuses(&bytes(&slope), "LLS8002", "slope");
    // The check refuses a certificate the host's integers cannot hold,
    // before reading a premise.
    let over = Certificate {
        route: Route::LowerBound {
            bound: MAX_NUMBER + 1,
            attainers: vec![3],
        },
        ..fixture("lb-argmin-over-systems").certificate
    };
    support::expect_code(
        &certification::check(&request("argmin-over-systems"), &over).expect_err("over capacity"),
        "LLS8002",
    );
    // A certificate with the most premises the host reads loads.
    let mut most = base;
    most["premises"] = json!(vec![json!({"kind": "measured"}); MAX_PREMISES as usize]);
    certification::load(&bytes(&most)).expect("the most premises load");
}

/// §17.18: the kernel is the oracle for every committed verdict, and Lean
/// rejects a verdict the checker does not give.
#[allow(clippy::too_many_lines)]
fn gc_02() {
    let fixtures = generated::fixtures();
    let module = generated::module(&fixtures);
    for fixture in &fixtures {
        let id = crate::calculus::identifier(&fixture.name);
        for theorem in ["Certificate", "Verdict"] {
            assert!(
                crate::support::declares(&module, &format!("{id}{theorem}")),
                "{id}{theorem}"
            );
        }
    }
    assert_eq!(
        read(generated::PATH),
        module,
        "the committed module equals its generator"
    );
    let main = read("compiler/src/Main.lex.tex");
    for import in ["Certification", "GnafCertificates"] {
        assert!(
            main.contains(&format!("\\importmodule{{{import}}}")),
            "the entrypoint reaches {import}, so verify checks it"
        );
    }
    // The model is checked by the kernel against what the host computes: the
    // theorem of each fixture states the host's verdict.
    for (_, _, fixture) in committed() {
        let theorem = declarations(generated::PATH)
            .into_iter()
            .find(|declaration| {
                declaration["name"]
                    == format!("{}Verdict", crate::calculus::identifier(&fixture.name))
            })
            .unwrap_or_else(|| panic!("{}: a verdict theorem", fixture.name));
        assert_eq!(theorem["kind"], "theorem");
        assert_eq!(
            theorem["statement"]["right"],
            certification::verdict_term(&fixture.expected),
            "{}: the theorem states the committed verdict",
            fixture.name
        );
    }
    if support::lean_backed("GC-02") {
        let verified = support::verified_compiler();
        for unit in ["Certification", "GnafCertificates"] {
            assert!(
                verified.outcome.units.contains_key(unit),
                "{unit} is verified"
            );
        }
        // Lean refuses each planted verdict, in a copy of the certificate
        // module stating it.
        let planted = planted_verdicts();
        for name in PLANTED {
            assert!(
                planted.iter().any(|rejected| rejected == name),
                "{name}: Lean rejects the planted verdict: {planted:#?}"
            );
        }
    }
}

/// The names of the verdict theorems planted below.
const PLANTED: [&str; 5] = [
    "plantedOmitted",
    "plantedUnattained",
    "plantedNormal",
    "plantedWeakened",
    "plantedPreserved",
];

/// The planted theorems Lean rejects, from a module importing the certificate module and stating
/// verdicts the checker does not give: a certified optimum for a certificate
/// that omits a competitor's premise, a bound its attainer does not meet, a
/// locally normal system, and a certificate whose competitors offer nothing.
fn planted_verdicts() -> Vec<String> {
    let text = "import LexLeanTarget.GnafCertificates\nopen LexLeanTarget\n\n\
        theorem plantedOmitted : Certification.check GnafFixtures.argminOverSystemsRequest \
        GnafCertificates.lbPremiseOmittedCertificate = Certification.Verdict.certified \
        (Certification.Certified.optimum [3] 167) := rfl\n\n\
        theorem plantedUnattained : Certification.check GnafFixtures.argminOverSystemsRequest \
        GnafCertificates.lbUnattainedCertificate = Certification.Verdict.certified \
        (Certification.Certified.optimum [3] 166) := rfl\n\n\
        theorem plantedNormal : Certification.check GnafFixtures.argminOverSystemsRequest \
        GnafCertificates.normalNotGlobalCertificate = Certification.Verdict.certified \
        (Certification.Certified.optimum [4] 328) := rfl\n\n\
        theorem plantedWeakened : Certification.check GnafFixtures.boundOverUnresolvedRequest \
        GnafCertificates.lbUnresolvedUnboundedCertificate = Certification.Verdict.certified \
        (Certification.Certified.argmin [0] 880) := rfl\n\n\
        theorem plantedPreserved : Certification.certificateReductionsPreserve \
        GnafFixtures.internalBestOnShortListsRequest \
        GnafCertificates.reductionIneligibleCertificate = true := rfl\n";
    let root = tempfile::Builder::new()
        .prefix("lexlean-planted-verdicts-")
        .tempdir()
        .expect("tempdir");
    let output = library::compile(
        root.path(),
        "PlantedVerdicts",
        text,
        &[
            library::environment().clone(),
            crate::preservation::shipped().build.clone(),
        ],
    );
    assert!(
        !output.status.success(),
        "Lean rejects the planted verdicts"
    );
    // Each theorem is one line, the first at line 4 and the rest two lines
    // apart, so an error is attributed to its theorem by its line.
    let messages = crate::preservation::joined(&output);
    assert_eq!(
        messages.matches("Not a definitional equality").count(),
        PLANTED.len(),
        "each planted verdict is rejected for being false, not for a typo: {messages}"
    );
    PLANTED
        .iter()
        .enumerate()
        .filter(|(index, _)| messages.contains(&format!("PlantedVerdicts.lean:{}:", 4 + 2 * index)))
        .map(|(_, name)| (*name).to_owned())
        .collect()
}

/// §17.18: universal lower bounds and their attainment.
#[allow(clippy::too_many_lines)]
fn gc_03() {
    // A dispatching system is the optimum although the best single plan is
    // not, and a bound every system respects is attained by it.
    let optimum = verdict("lb-argmin-over-systems");
    assert_eq!(
        optimum,
        certified(Certified::Optimum {
            members: vec![3],
            steps: 167
        })
    );
    let answer = gnaf::answer(&request("argmin-over-systems")).expect("answers");
    assert_eq!(
        answer,
        Answer::Argmin {
            members: vec![3],
            steps: 167
        },
        "the certificate agrees with enumeration where enumeration answers"
    );
    assert!(matches!(
        gnaf::statuses(&request("argmin-over-systems"))
            .expect("evaluates")
            .get(3),
        Some((
            _,
            Selector::Dispatch { .. },
            Status::Admitted { steps: 167, .. }
        ))
    ));
    // The complete argmin, with every tie an attainer.
    assert_eq!(
        verdict("lb-argmin-complete"),
        certified(Certified::Argmin {
            members: vec![3],
            steps: 167
        })
    );
    let tie = verdict("lb-argmin-tie");
    assert_eq!(
        tie,
        certified(Certified::Argmin {
            members: vec![4, 5],
            steps: 167
        })
    );
    assert_eq!(
        gnaf::answer(&request("argmin-tie")).expect("answers"),
        Answer::Argmin {
            members: vec![4, 5],
            steps: 167
        }
    );
    assert_eq!(
        verdict("lb-tie-dropped"),
        incomplete(
            Obligation::AttainerOmitted { position: 5 },
            Established::CorrectOnly {
                system: 4,
                steps: 167,
                size: 46
            }
        ),
        "an argmin that drops a tied attainer is not certified"
    );
    // Enumeration cannot answer a universe with systems whose cost is
    // unknown at the request's fuel; the certificate bounds them without
    // evaluating them in full.
    let unresolved = request("bound-over-unresolved");
    assert_eq!(
        gnaf::answer(&unresolved).expect("answers"),
        Answer::Incomplete
    );
    let certificate = fixture("lb-over-unresolved").certificate;
    assert_eq!(
        verdict("lb-over-unresolved"),
        certified(Certified::Argmin {
            members: vec![0],
            steps: 880
        })
    );
    let statuses = gnaf::statuses(&unresolved).expect("evaluates");
    let mut bounded_unresolved = 0;
    for (position, (_, _, status)) in statuses.iter().enumerate() {
        if *status == Status::Unresolved {
            let Premise::PaysAtLeast { positions } = &certificate.premises[position] else {
                panic!("an unresolved system is bounded by a prefix of the domain")
            };
            assert!(
                positions.len() < unresolved.domain.len(),
                "system {position}: the prefix is not the whole domain"
            );
            bounded_unresolved += 1;
        }
    }
    assert_eq!(
        bounded_unresolved, 3,
        "three of six systems have an unknown cost"
    );
    // The same certificate with those systems offering nothing leaves the
    // first of them unproven.
    assert_eq!(
        verdict("lb-unresolved-unbounded"),
        incomplete(
            Obligation::MemberUnproven { position: 1 },
            Established::CorrectOnly {
                system: 0,
                steps: 880,
                size: 26
            }
        )
    );
    // A diverging plan's systems can never be shown above a bound by the
    // arguments they agree on.
    assert_eq!(
        verdict("lb-diverging-plan"),
        incomplete(
            Obligation::MemberUnproven { position: 2 },
            Established::CorrectOnly {
                system: 0,
                steps: 180,
                size: 15
            }
        )
    );
    // Invalid bounds.
    assert_eq!(
        verdict("lb-undercut"),
        incomplete(
            Obligation::MemberBelowBound { position: 3 },
            Established::CorrectOnly {
                system: 3,
                steps: 167,
                size: 46
            }
        ),
        "a system pays less than the stated bound"
    );
    assert_eq!(
        verdict("lb-unattained"),
        incomplete(
            Obligation::AttainerAboveBound { position: 3 },
            Established::CorrectOnly {
                system: 3,
                steps: 167,
                size: 46
            }
        ),
        "no system attains the stated bound"
    );
    assert_eq!(
        verdict("lb-no-attainer"),
        incomplete(Obligation::BoundUnattained, Established::NotCertified)
    );
    assert_eq!(
        verdict("lb-infeasible-attainer"),
        incomplete(
            Obligation::AttainerInadmissible { position: 0 },
            Established::NotCertified
        )
    );
    // The planted omission of one competitor's premise.
    assert_eq!(
        verdict("lb-premise-omitted"),
        refused(Refusal::MissingPremise { position: 5 })
    );
    assert_eq!(
        verdict("lb-extra-premise"),
        refused(Refusal::ExtraPremise { count: 6 })
    );
    assert_eq!(
        verdict("lb-premise-weakened"),
        incomplete(
            Obligation::MemberUnproven { position: 0 },
            Established::CorrectOnly {
                system: 3,
                steps: 167,
                size: 46
            }
        )
    );
    // A tied system bounded exactly at the bound is not strictly above it, so
    // an argmin may not leave it out of the attainers.
    assert_eq!(
        verdict("lb-tie-bounded"),
        incomplete(
            Obligation::MemberUnproven { position: 5 },
            Established::CorrectOnly {
                system: 4,
                steps: 167,
                size: 46
            }
        )
    );
    // Evidence is of distinct arguments, of a system that does not agree
    // elsewhere, of a cost the fuel resolves, and of a system the premise
    // measures.
    for (name, position) in [("lb-positions-repeated", 0), ("lb-excluded-admitted", 0)] {
        assert_eq!(
            verdict(name),
            incomplete(
                Obligation::MemberUnproven { position },
                Established::CorrectOnly {
                    system: 3,
                    steps: 167,
                    size: 46
                }
            ),
            "{name}"
        );
    }
    assert_eq!(
        verdict("lb-unresolved-measured"),
        incomplete(
            Obligation::UnresolvedMember { position: 1 },
            Established::CorrectOnly {
                system: 0,
                steps: 880,
                size: 26
            }
        ),
        "a system the fuel does not resolve is not an excluded one"
    );
    assert_eq!(
        verdict("lb-attainer-bounded"),
        incomplete(
            Obligation::AttainerUnmeasured { position: 3 },
            Established::NotCertified
        ),
        "an attainer is measured, not bounded"
    );
    for name in ["lb-attainers-unordered", "lb-attainer-out-of-range"] {
        assert_eq!(
            verdict(name),
            incomplete(Obligation::AttainersMalformed, Established::NotCertified),
            "{name}: attainers are positions of the expansion in increasing order"
        );
    }
    // The preparation each invocation is charged is part of a bound.
    assert_eq!(
        verdict("lb-preparation-charged"),
        certified(Certified::Optimum {
            members: vec![0],
            steps: 180
        })
    );
    // Attainers are positions of the expansion: one past the last premise is
    // none; a position beyond the domain is no evidence however large its
    // bound; a measured system above an argmin's bound, and a measured tie
    // under an optimum, are no omissions; a Pareto claim has no scalar route.
    assert_eq!(
        verdict("lb-attainer-at-count"),
        incomplete(Obligation::AttainersMalformed, Established::NotCertified)
    );
    assert_eq!(
        verdict("lb-position-out-of-domain"),
        incomplete(
            Obligation::MemberUnproven { position: 0 },
            Established::CorrectOnly {
                system: 3,
                steps: 167,
                size: 46
            }
        )
    );
    assert_eq!(
        verdict("lb-argmin-measured"),
        certified(Certified::Argmin {
            members: vec![3],
            steps: 167
        })
    );
    assert_eq!(
        verdict("lb-optimum-tie-measured"),
        certified(Certified::Optimum {
            members: vec![4],
            steps: 167
        })
    );
    assert_eq!(
        verdict("route-lower-bound-pareto"),
        refused(Refusal::RouteIncompatible)
    );
    // The same tie asked as an argmin leaves the measured tie an omission.
    let tie_as_argmin = fixture("lb-optimum-tie-measured").certificate;
    assert_eq!(
        check(
            "argmin-tie",
            &Certificate {
                claim: ClaimClass::ArgminComplete,
                ..tie_as_argmin
            }
        ),
        incomplete(
            Obligation::AttainerOmitted { position: 5 },
            Established::CorrectOnly {
                system: 4,
                steps: 167,
                size: 46
            }
        )
    );
    // An exclusion names an argument of the domain, on which the system
    // disagrees: one past the last is none, an argument the system exhausts
    // the fuel on proves nothing, and a bound over a prefix the system
    // disagrees on is no evidence of its cost.
    for (name, position, established) in [
        (
            "lb-excluded-at-length",
            0,
            Established::CorrectOnly {
                system: 3,
                steps: 167,
                size: 46,
            },
        ),
        (
            "lb-excluded-unresolved",
            1,
            Established::CorrectOnly {
                system: 0,
                steps: 880,
                size: 26,
            },
        ),
        (
            "lb-pays-inadmissible-prefix",
            1,
            Established::CorrectOnly {
                system: 0,
                steps: 180,
                size: 15,
            },
        ),
    ] {
        assert_eq!(
            verdict(name),
            incomplete(Obligation::MemberUnproven { position }, established),
            "{name}"
        );
    }
    // Every mutation of a certified lower bound is uncertified: a premise
    // dropped or weakened, the bound moved either way, an attainer dropped
    // or added.
    for name in [
        "lb-argmin-over-systems",
        "lb-argmin-complete",
        "lb-argmin-tie",
        "lb-over-unresolved",
    ] {
        let base = fixture(name);
        let posed = base.request.clone();
        let Route::LowerBound { bound, attainers } = base.certificate.route.clone() else {
            panic!("{name} is a lower bound")
        };
        assert!(is_certified(&rerun(&base)), "{name}");
        let variants = |route: Route, premises: Vec<Premise>| Certificate {
            route,
            premises,
            ..base.certificate.clone()
        };
        for position in 0..base.certificate.premises.len() {
            let mut dropped = base.certificate.premises.clone();
            dropped.remove(position);
            let verdict = check(&posed, &variants(base.certificate.route.clone(), dropped));
            assert!(
                matches!(verdict, Verdict::Refused { .. }),
                "{name}: premise {position} dropped: {verdict:?}"
            );
            if !attainers.contains(&(position as u64)) {
                let mut weakened = base.certificate.premises.clone();
                weakened[position] = Premise::PaysAtLeast {
                    positions: Vec::new(),
                };
                let verdict = check(&posed, &variants(base.certificate.route.clone(), weakened));
                assert!(
                    matches!(
                        verdict,
                        Verdict::Incomplete {
                            obligation: Obligation::MemberUnproven { position: found },
                            ..
                        } if found == position as u64
                    ),
                    "{name}: premise {position} weakened: {verdict:?}"
                );
            }
        }
        for moved in [bound - 1, bound + 1] {
            let route = Route::LowerBound {
                bound: moved,
                attainers: attainers.clone(),
            };
            let verdict = check(&posed, &variants(route, base.certificate.premises.clone()));
            assert!(
                !is_certified(&verdict),
                "{name}: bound {moved}: {verdict:?}"
            );
        }
        let route = Route::LowerBound {
            bound,
            attainers: Vec::new(),
        };
        assert!(
            !is_certified(&check(
                &posed,
                &variants(route, base.certificate.premises.clone())
            )),
            "{name}: no attainers"
        );
    }
}

/// §17.18: complete Pareto frontiers.
fn gc_04() {
    let vec02 = verdict("frontier-vec02");
    assert_eq!(
        vec02,
        certified(Certified::Frontier {
            members: vec![0, 1, 2]
        })
    );
    assert_eq!(
        gnaf::answer(&request(crate::gnaf::PARETO_ENVELOPE)).expect("answers"),
        Answer::Frontier {
            members: vec![0, 1, 2]
        },
        "GNAF-VEC-02: three of four systems are the frontier, as enumeration finds"
    );
    assert_eq!(
        verdict("frontier-pareto-optimal"),
        certified(Certified::Frontier {
            members: vec![0, 3]
        })
    );
    // Equal costs dominate neither: every tied system is a member.
    assert_eq!(
        verdict("frontier-tie"),
        certified(Certified::Frontier {
            members: vec![0, 4, 5]
        })
    );
    assert_eq!(
        gnaf::answer(&request("frontier-tie")).expect("answers"),
        Answer::Frontier {
            members: vec![0, 4, 5]
        }
    );
    assert_eq!(
        verdict("frontier-tie-dropped"),
        incomplete(
            Obligation::FrontierMemberOmitted { position: 5 },
            Established::NotCertified
        )
    );
    assert_eq!(
        verdict("frontier-member-omitted"),
        incomplete(
            Obligation::FrontierMemberOmitted { position: 0 },
            Established::NotCertified
        ),
        "GNAF-REJ-29: a frontier with a member omitted is not certified"
    );
    assert_eq!(
        verdict("frontier-dominated-member"),
        incomplete(
            Obligation::FrontierMemberDominated { position: 3 },
            Established::NotCertified
        )
    );
    assert_eq!(
        verdict("frontier-premise-weakened"),
        incomplete(
            Obligation::MemberUnproven { position: 3 },
            Established::NotCertified
        )
    );
    // The members and the non-members of the committed frontier, by dominance.
    let posed = request(crate::gnaf::PARETO_ENVELOPE);
    let rows: Vec<(u64, Vec<u64>)> = gnaf::statuses(&posed)
        .expect("evaluates")
        .into_iter()
        .filter_map(|(position, _, status)| match status {
            Status::Admitted { steps, size } => Some((position, vec![steps, size])),
            Status::Inadmissible | Status::Unresolved => None,
        })
        .collect();
    let base = fixture("frontier-vec02").certificate;
    let Route::FrontierCover { members } = base.route.clone() else {
        panic!("a frontier cover")
    };
    for (position, cost) in &rows {
        let dominated = rows.iter().any(|(_, other)| gnaf::dominates(other, cost));
        assert_eq!(
            members.contains(position),
            !dominated,
            "system {position}: a member exactly when nothing dominates it"
        );
    }
    // Members are positions of the expansion in increasing order, each
    // measured; every other system is excluded, reduced, or dominated.
    for name in ["frontier-member-at-count", "frontier-members-unordered"] {
        assert_eq!(
            verdict(name),
            incomplete(Obligation::AttainersMalformed, Established::NotCertified),
            "{name}"
        );
    }
    assert_eq!(
        verdict("frontier-member-unproven"),
        incomplete(
            Obligation::MemberUnproven { position: 0 },
            Established::NotCertified
        )
    );
    assert_eq!(
        verdict("frontier-nonmember-unproven"),
        incomplete(
            Obligation::MemberUnproven { position: 3 },
            Established::NotCertified
        )
    );
    // A Pareto claim's reduction is a frontier.
    assert_eq!(
        verdict("reduction-pareto"),
        certified(Certified::Frontier {
            members: vec![0, 3]
        })
    );
    // A system the fuel leaves unresolved is neither a member nor a
    // dominated non-member: the same request posed as a frontier.
    let over_unresolved = Request {
        objective: Objective::Vector,
        claim: ClaimClass::FrontierComplete,
        ..request("bound-over-unresolved")
    };
    let base_unresolved = fixture("lb-over-unresolved").certificate;
    let measured_all = vec![Premise::Measured; base_unresolved.premises.len()];
    for (members, position) in [(vec![0], 1), (vec![0, 1], 1)] {
        let verdict = certification::check(
            &over_unresolved,
            &Certificate {
                objective: Objective::Vector,
                claim: ClaimClass::FrontierComplete,
                route: Route::FrontierCover {
                    members: members.clone(),
                },
                premises: measured_all.clone(),
                ..base_unresolved.clone()
            },
        )
        .expect("within capacity");
        assert_eq!(
            verdict,
            incomplete(
                Obligation::UnresolvedMember { position },
                Established::NotCertified
            ),
            "members {members:?}: an unresolved system is open"
        );
    }
    for name in [
        "frontier-unresolved-member",
        "frontier-unresolved-nonmember",
    ] {
        assert_eq!(
            verdict(name),
            incomplete(
                Obligation::UnresolvedMember { position: 1 },
                Established::NotCertified
            ),
            "{name}: the kernel decides it, too"
        );
    }
    // Every mutation of the certified frontier is uncertified: a premise
    // dropped or weakened, a member removed or added.
    for position in 0..base.premises.len() {
        let mut dropped = base.premises.clone();
        dropped.remove(position);
        let verdict = check(
            crate::gnaf::PARETO_ENVELOPE,
            &Certificate {
                premises: dropped,
                ..base.clone()
            },
        );
        assert!(
            matches!(verdict, Verdict::Refused { .. }),
            "premise {position} dropped: {verdict:?}"
        );
        let mut weakened = base.premises.clone();
        weakened[position] = Premise::PaysAtLeast {
            positions: Vec::new(),
        };
        let verdict = check(
            crate::gnaf::PARETO_ENVELOPE,
            &Certificate {
                premises: weakened,
                ..base.clone()
            },
        );
        // A member must be measured; a non-member may still be dominated by
        // the evidence it keeps, so only a member's weakening is refused.
        if members.contains(&(position as u64)) {
            assert!(
                !is_certified(&verdict),
                "premise {position} weakened: {verdict:?}"
            );
        }
    }
    for removed in 0..members.len() {
        let mut fewer = members.clone();
        fewer.remove(removed);
        let verdict = check(
            crate::gnaf::PARETO_ENVELOPE,
            &Certificate {
                route: Route::FrontierCover { members: fewer },
                ..base.clone()
            },
        );
        assert!(
            !is_certified(&verdict),
            "member {removed} removed: {verdict:?}"
        );
    }
}

/// §17.18: the transformation schemas and proof-complete reduction.
#[allow(clippy::too_many_lines)]
fn gc_05() {
    // A short domain specializes every dispatch; a mixed domain collapses
    // only the dispatches of one plan.
    let short = request("internal-best-on-short-lists");
    let improvements: Vec<Option<Schema>> = gnaf::statuses(&short)
        .expect("evaluates")
        .into_iter()
        .map(|(_, selector, _)| {
            certification::improving_schema(&short, selector).expect("evaluates")
        })
        .collect();
    assert_eq!(
        improvements,
        vec![
            None,
            None,
            Some(Schema::Collapse),
            Some(Schema::SpecializeSmall),
            Some(Schema::SpecializeSmall),
            Some(Schema::Collapse),
        ]
    );
    let mixed = request("argmin-over-systems");
    let improvements: Vec<Option<Schema>> = gnaf::statuses(&mixed)
        .expect("evaluates")
        .into_iter()
        .map(|(_, selector, _)| {
            certification::improving_schema(&mixed, selector).expect("evaluates")
        })
        .collect();
    assert_eq!(
        improvements,
        vec![
            None,
            None,
            Some(Schema::Collapse),
            None,
            None,
            Some(Schema::Collapse),
        ]
    );
    // Preservation on the committed instances: where a schema improves a
    // system, the rewritten system agrees on every argument it agrees on,
    // in strictly fewer steps. The library proves this for every grammar;
    // this is the instance-level evidence.
    let mut checked = 0;
    for name in [
        "internal-best-on-short-lists",
        "argmin-over-systems",
        "argmin-complete",
        "argmin-tie",
        "bound-over-unresolved",
    ] {
        let posed = request(name);
        let selectors: Vec<Selector> = gnaf::expand(&posed.carrier);
        for selector in &selectors {
            let Some(schema) =
                certification::improving_schema(&posed, *selector).expect("evaluates")
            else {
                continue;
            };
            let target = rewritten(*selector, schema);
            assert!(
                selectors.contains(&target),
                "{name}: the rewriting of {selector:?} is a system of the universe"
            );
            let (before, _, size) = certification::trials(&posed, *selector).expect("evaluates");
            let (after, _, smaller) = certification::trials(&posed, target).expect("evaluates");
            assert!(smaller <= size, "{name}: the rewriting is no larger");
            for (position, (was, now)) in before.iter().zip(&after).enumerate() {
                if let certification::Trial::Agrees { steps } = was {
                    let certification::Trial::Agrees { steps: fewer } = now else {
                        panic!("{name}: {selector:?} agrees on {position}, its rewriting does not")
                    };
                    assert!(fewer < steps, "{name}: {selector:?} argument {position}");
                    checked += 1;
                }
            }
        }
    }
    assert!(
        checked >= 16,
        "the instances cover many arguments: {checked}"
    );
    // The closed semantics of preservation (`Certification.preserves`,
    // `sameObservations`, `strictlyCheaper`): the host transcription agrees
    // with `improves` on every schema application of the committed requests,
    // and the committed module states, for each certificate with a reduction,
    // whether every one of its reductions preserves.
    // The relations themselves, for every pair of systems of a request (not
    // only a schema's rewriting): read back from the trials of each system on
    // each argument, independently of the transcription.
    let mut pairs = 0;
    for name in [
        "internal-best-on-short-lists",
        "argmin-over-systems",
        "threshold-edge",
        "bound-over-unresolved",
    ] {
        let posed = request(name);
        let selectors = gnaf::expand(&posed.carrier);
        let trials: Vec<_> = selectors
            .iter()
            .map(|selector| {
                certification::trials(&posed, *selector)
                    .expect("evaluates")
                    .0
            })
            .collect();
        for (before, selector) in trials.iter().zip(&selectors) {
            for (after, target) in trials.iter().zip(&selectors) {
                let mut keeps = true;
                let mut cheaper = true;
                for (was, now) in before.iter().zip(after) {
                    if let Trial::Agrees { steps } = was {
                        match now {
                            Trial::Agrees { steps: fewer } => cheaper &= fewer < steps,
                            Trial::Disagrees | Trial::Unknown => {
                                keeps = false;
                                cheaper = false;
                            }
                        }
                    }
                }
                assert_eq!(
                    certification::observations(&posed, *selector, *target).expect("evaluates"),
                    (keeps, cheaper),
                    "{name}: {selector:?} against {target:?}"
                );
                pairs += 1;
            }
        }
    }
    assert!(pairs >= 80, "many pairs of systems: {pairs}");
    // The relations are decided by the kernel on pairs that are no schema's
    // rewriting, one of each class, derived by the host from the trials and
    // independently of `improves`: a pair whose observations are not kept, a
    // pair kept at no saving, and a pair kept at a saving.
    let committed_pairs = generated::pairs();
    let classes: BTreeSet<(bool, bool)> = committed_pairs
        .iter()
        .map(|pair| (pair.keeps, pair.cheaper))
        .collect();
    assert_eq!(
        classes,
        BTreeSet::from([(false, false), (true, false), (true, true)]),
        "every class of pair is stated"
    );
    let module_theorems = declarations(generated::PATH);
    for pair in &committed_pairs {
        for cheaper in [false, true] {
            let name = generated::pair_theorem(pair, cheaper);
            let theorem = module_theorems
                .iter()
                .find(|declaration| declaration["name"] == name)
                .unwrap_or_else(|| panic!("{name} is stated"));
            assert_eq!(
                theorem["statement"]["right"],
                crate::lx::boolean(if cheaper { pair.cheaper } else { pair.keeps }),
                "{name}"
            );
        }
    }
    // The definitions are the conjunction they say they are: `preserves`
    // reads `improves` and both relations, so it is not `improves` alone, and
    // a reduction premise is read through `preserves`.
    let model = declarations("compiler/src/Certification.lex.tex");
    let references_of = |name: &str| {
        let declaration = model
            .iter()
            .find(|declaration| declaration["name"] == name)
            .unwrap_or_else(|| panic!("{name} is defined"));
        let mut out = BTreeSet::new();
        references(declaration, &mut out);
        out
    };
    for (name, used) in [
        (
            "preserves",
            vec!["improves", "sameObservations", "strictlyCheaper", "rewrite"],
        ),
        ("reductionsPreserve", vec!["preserves"]),
        ("sameObservations", vec!["sameObservationsAt"]),
        ("strictlyCheaper", vec!["strictlyCheaperAt"]),
        (
            "sameObservationsAt",
            vec!["keepsObservation", "argumentStatus"],
        ),
        (
            "strictlyCheaperAt",
            vec!["cheaperObservation", "argumentStatus"],
        ),
    ] {
        let seen = references_of(name);
        for reference in used {
            assert!(
                seen.iter().any(|seen| seen.ends_with(reference)),
                "{name} reads {reference}: {seen:?}"
            );
        }
    }
    // A request that is no grammar's has no preserving reduction.
    let rejected = fixture("request-rejected-cached");
    assert!(!certification::reductions_preserve(
        &request(&rejected.request),
        &rejected.certificate
    )
    .expect("evaluates"));
    let model_names: BTreeSet<String> = declarations("compiler/src/Certification.lex.tex")
        .iter()
        .filter_map(|declaration| declaration["name"].as_str().map(str::to_owned))
        .collect();
    for name in [
        "argumentStatus",
        "keepsObservation",
        "cheaperObservation",
        "sameObservationsAt",
        "strictlyCheaperAt",
        "sameObservations",
        "strictlyCheaper",
        "preserves",
        "reductionsPreserve",
        "certificateReductionsPreserve",
    ] {
        assert!(model_names.contains(name), "the module defines {name}");
    }
    let mut applications = 0;
    for name in [
        "internal-best-on-short-lists",
        "argmin-over-systems",
        "argmin-complete",
        "argmin-tie",
        "bound-over-unresolved",
        "threshold-edge",
    ] {
        let posed = request(name);
        for selector in gnaf::expand(&posed.carrier) {
            let improving = certification::improving_schema(&posed, selector).expect("evaluates");
            for schema in [
                Schema::Collapse,
                Schema::SpecializeSmall,
                Schema::SpecializeLarge,
            ] {
                let preserved =
                    certification::preserves(&posed, selector, schema).expect("evaluates");
                if improving == Some(schema) {
                    assert!(
                        preserved,
                        "{name}: {selector:?} improved by {schema:?} preserves its observations"
                    );
                }
                if preserved {
                    applications += 1;
                    assert!(
                        !certification::is_normal(&posed, selector).expect("evaluates"),
                        "{name}: {selector:?} is improved by {schema:?}, so it is not normal"
                    );
                }
            }
        }
    }
    assert!(
        applications >= 8,
        "many preserving applications: {applications}"
    );
    let module = declarations(generated::PATH);
    let mut stated = 0;
    for (_, _, fixture) in committed() {
        let reduces = fixture
            .certificate
            .premises
            .iter()
            .any(|premise| matches!(premise, Premise::Reduced { .. }));
        let theorem = module.iter().find(|declaration| {
            declaration["name"]
                == format!("{}Preserves", crate::calculus::identifier(&fixture.name))
        });
        assert_eq!(
            theorem.is_some(),
            reduces,
            "{}: a preservation theorem exactly when a premise reduces",
            fixture.name
        );
        if let Some(theorem) = theorem {
            let host = certification::reductions_preserve(
                &request(&fixture.request),
                &fixture.certificate,
            )
            .expect("evaluates");
            assert_eq!(
                theorem["statement"]["right"],
                crate::lx::boolean(host),
                "{}: the theorem states the host's reading",
                fixture.name
            );
            stated += 1;
        }
    }
    assert!(
        stated >= 6,
        "reduction certificates state preservation: {stated}"
    );
    for (name, preserved) in [
        ("reduction-short-lists", true),
        ("reduction-mixed-domain", true),
        ("reduction-threshold-edge", true),
        ("reduction-ineligible", false),
        ("reduction-collapse-distinct", false),
        ("reduction-small-at-threshold", false),
    ] {
        let fixture = fixture(name);
        assert_eq!(
            certification::reductions_preserve(&request(&fixture.request), &fixture.certificate)
                .expect("evaluates"),
            preserved,
            "{name}"
        );
    }
    // A reduction over a system the fuel leaves unresolved is open at it.
    assert_eq!(
        verdict("reduction-over-unresolved"),
        incomplete(
            Obligation::UnresolvedMember { position: 1 },
            Established::NotCertified
        )
    );
    // A plan that calls the entry makes every schema ineligible: the
    // rewriting would change what the plan calls.
    let mut entry_calling = short.clone();
    if let gnaf::Carrier::Grammar { plans, .. } = &mut entry_calling.carrier {
        plans[0].function = lexlean::calculus::Function {
            parameters: vec![0],
            types: plans[0].function.types.clone(),
            result: plans[0].function.result.clone(),
            body: lexlean::calculus::Expr::Call {
                function: 0,
                operands: vec![lexlean::calculus::Expr::Var { name: 0 }],
            },
        };
    }
    for selector in gnaf::expand(&entry_calling.carrier) {
        assert_eq!(
            certification::improving_schema(&entry_calling, selector).expect("evaluates"),
            None,
            "{selector:?}: a plan calling the entry blocks every schema"
        );
    }
    // The reductions.
    assert_eq!(
        verdict("reduction-short-lists"),
        certified(Certified::Optimum {
            members: vec![0],
            steps: 36
        })
    );
    assert_eq!(
        gnaf::answer(&short).expect("answers"),
        Answer::Argmin {
            members: vec![0],
            steps: 36
        },
        "the reduction agrees with enumeration"
    );
    assert_eq!(
        verdict("reduction-mixed-domain"),
        certified(Certified::Optimum {
            members: vec![3],
            steps: 167
        })
    );
    assert_eq!(
        verdict("reduction-incomplete"),
        incomplete(
            Obligation::ReductionIncomplete { position: 2 },
            Established::NotCertified
        ),
        "a system neither measured nor reduced leaves the reduction incomplete"
    );
    assert_eq!(
        verdict("reduction-ineligible"),
        incomplete(
            Obligation::ReductionIncomplete { position: 0 },
            Established::NotCertified
        ),
        "a schema that does not apply reduces nothing"
    );
    assert_eq!(
        verdict("reduction-collapse-distinct"),
        incomplete(
            Obligation::ReductionIncomplete { position: 3 },
            Established::NotCertified
        ),
        "a dispatch of two different plans is not collapsed"
    );
    // Lists as long as the threshold run the large plan: the large
    // specialization applies and the small one does not.
    assert_eq!(
        verdict("reduction-threshold-edge"),
        certified(Certified::Optimum {
            members: vec![0],
            steps: 48
        })
    );
    assert_eq!(
        verdict("reduction-small-at-threshold"),
        incomplete(
            Obligation::ReductionIncomplete { position: 2 },
            Established::NotCertified
        ),
        "no list is shorter than the threshold"
    );
    // The library registers the preservation theorems.
    let registered: BTreeSet<String> = library::registry()
        .declaration
        .into_iter()
        .map(|declaration| declaration.name)
        .collect();
    for theorem in [
        "schema_cheaper",
        "dispatch_cheaper",
        "eval_tail",
        "call_succ",
    ] {
        assert!(
            registered.contains(&format!("LexLeanCertification.{theorem}")),
            "{theorem} is registered"
        );
    }
}

/// The system a schema rewrites a dispatch to.
fn rewritten(selector: Selector, schema: Schema) -> Selector {
    let Selector::Dispatch { small, large, .. } = selector else {
        panic!("a schema rewrites a dispatch")
    };
    Selector::Fixed {
        plan: match schema {
            Schema::Collapse | Schema::SpecializeSmall => small,
            Schema::SpecializeLarge => large,
        },
    }
}

/// §17.18: local normality is never optimality.
#[allow(clippy::too_many_lines)]
fn gc_06() {
    let posed = request("argmin-over-systems");
    let selectors = gnaf::expand(&posed.carrier);
    let statuses = gnaf::statuses(&posed).expect("evaluates");
    // A GNAF-normal system that is not the argmin.
    let suboptimal = 4usize;
    assert!(certification::is_normal(&posed, selectors[suboptimal]).expect("evaluates"));
    let Status::Admitted { steps, .. } = statuses[suboptimal].2 else {
        panic!("admitted")
    };
    assert!(steps > 167, "the normal system pays more than the optimum");
    assert_eq!(
        verdict("normal-not-global"),
        incomplete(
            Obligation::OptimalityUnproved,
            Established::LocallyNormalized { system: 4 }
        ),
        "a normal system is locally normalized and nothing more"
    );
    assert!(
        !gnaf::certifies(
            &posed,
            &Answer::Argmin {
                members: vec![4],
                steps
            }
        )
        .expect("answers"),
        "the model does not certify the normal system's cost as the argmin"
    );
    assert!(gnaf::certifies(
        &posed,
        &Answer::Argmin {
            members: vec![3],
            steps: 167
        }
    )
    .expect("answers"));
    // The same system claimed as the attainer of the bound it cannot meet.
    assert_eq!(
        verdict("normal-claimed-global"),
        incomplete(
            Obligation::MemberBelowBound { position: 0 },
            Established::CorrectOnly {
                system: 4,
                steps: 328,
                size: 46
            }
        )
    );
    // A system a schema improves is not normal.
    assert_eq!(
        verdict("normality-improvable"),
        incomplete(
            Obligation::SystemNotNormal,
            Established::CorrectOnly {
                system: 2,
                steps: 235,
                size: 22
            }
        )
    );
    assert!(!certification::is_normal(&posed, selectors[2]).expect("evaluates"));
    // The optimum is normal, and normality still does not certify it.
    assert!(certification::is_normal(&posed, selectors[3]).expect("evaluates"));
    assert_eq!(
        verdict("normality-optimum"),
        incomplete(
            Obligation::OptimalityUnproved,
            Established::LocallyNormalized { system: 3 }
        )
    );
    // No normality certificate is certified: not for any system, claim, or
    // premise list.
    let base = fixture("normal-not-global").certificate;
    for system in 0..=selectors.len() as u64 {
        for claim in [ClaimClass::GlobalOptimal, ClaimClass::ArgminComplete] {
            for measured in [true, false] {
                let premises = base
                    .premises
                    .iter()
                    .map(|_| {
                        if measured {
                            Premise::Measured
                        } else {
                            Premise::PaysAtLeast {
                                positions: Vec::new(),
                            }
                        }
                    })
                    .collect();
                let verdict = check(
                    "argmin-over-systems",
                    &Certificate {
                        route: Route::Normality { system },
                        claim: claim.clone(),
                        premises,
                        ..base.clone()
                    },
                );
                assert!(
                    !is_certified(&verdict),
                    "normality of {system} under {claim:?}: {verdict:?}"
                );
            }
        }
    }
    // A certified optimum is GNAF-normal; the converse is false.
    let mut optima = 0;
    for (_, _, fixture) in committed() {
        let Verdict::Certified {
            result: Certified::Optimum { members, .. } | Certified::Argmin { members, .. },
        } = &fixture.expected
        else {
            continue;
        };
        let posed = request(&fixture.request);
        let selectors = gnaf::expand(&posed.carrier);
        for member in members {
            assert!(
                certification::is_normal(&posed, selectors[*member as usize]).expect("evaluates"),
                "{}: the certified optimum {member} is GNAF-normal",
                fixture.name
            );
            optima += 1;
        }
    }
    assert!(optima >= 6, "the committed optima are checked: {optima}");
    // Nothing in the model decides optimality by normality: `isNormal` is
    // read by `normalityVerdict`, which cannot build a certified result, and
    // by no definition that can.
    let model = declarations(MODEL);
    let builds_certified = |declaration: &Json| {
        let mut names = BTreeSet::new();
        references(declaration, &mut names);
        names.contains("Certified.optimum")
            || names.contains("Certified.argmin")
            || names.contains("Certified.frontier")
    };
    let readers: BTreeSet<String> = model
        .iter()
        .filter(|declaration| {
            let mut names = BTreeSet::new();
            references(&declaration["body"], &mut names);
            names.contains("isNormal")
        })
        .map(|declaration| declaration["name"].as_str().expect("name").to_owned())
        .collect();
    assert_eq!(
        readers,
        BTreeSet::from(["normalityVerdict".to_owned()]),
        "only the normality route reads GNAF-normality"
    );
    for declaration in &model {
        if readers.contains(declaration["name"].as_str().expect("name")) {
            assert!(
                !builds_certified(declaration),
                "{}: the normality route builds no certified result",
                declaration["name"]
            );
        }
    }
    let registered: BTreeSet<String> = library::registry()
        .declaration
        .into_iter()
        .map(|declaration| declaration.name)
        .collect();
    for theorem in [
        "normality_never_certifies",
        "certified_optimum_is_normal",
        "optimum_is_normal",
        "normalityVerdict_not_certified",
    ] {
        assert!(
            registered.contains(&format!("LexLeanCertification.{theorem}")),
            "{theorem} is registered"
        );
    }
}

/// §17.18: a representation metric needs a bridge to the objective.
fn gc_07() {
    let posed = request("bridge-straight-line");
    let statuses = gnaf::statuses(&posed).expect("evaluates");
    let costs: Vec<(u64, u64)> = statuses
        .iter()
        .map(|(_, _, status)| match status {
            Status::Admitted { steps, size } => (*steps, *size),
            Status::Inadmissible | Status::Unresolved => panic!("every plan is admitted"),
        })
        .collect();
    assert_eq!(costs, vec![(160, 14), (83, 8), (120, 11), (203, 17)]);
    assert_eq!(
        verdict("bridge-attained"),
        certified(Certified::Optimum {
            members: vec![1],
            steps: 83
        })
    );
    let bridge = match fixture("bridge-attained").certificate.route {
        Route::Representation {
            bridge: Some(bridge),
            ..
        } => bridge,
        other => panic!("a bridge: {other:?}"),
    };
    // The bridge holds of every system, and the least-size system attains it.
    for (steps, size) in &costs {
        assert!(
            *steps >= bridge.slope * size + bridge.offset,
            "the bridge holds: cost {steps}, size {size}"
        );
    }
    assert_eq!(costs[1].0, bridge.slope * costs[1].1 + bridge.offset);
    assert_eq!(
        gnaf::answer(&posed).expect("answers"),
        Answer::Argmin {
            members: vec![1],
            steps: 83
        },
        "the certified optimum is the enumerated one"
    );
    assert_eq!(
        verdict("bridge-missing"),
        incomplete(
            Obligation::BridgeMissing,
            Established::RepresentationMinimal { system: 1, size: 8 }
        ),
        "without a bridge, representation minimality is all that is established"
    );
    assert_eq!(
        verdict("bridge-violated"),
        incomplete(
            Obligation::BridgeViolated { position: 1 },
            Established::NotCertified
        )
    );
    assert_eq!(
        verdict("bridge-unattained"),
        incomplete(
            Obligation::BoundUnattained,
            Established::CorrectOnly {
                system: 1,
                steps: 83,
                size: 8
            }
        ),
        "a valid bridge the least-size system does not meet bounds nothing it attains"
    );
    assert_eq!(
        verdict("bridge-not-minimal"),
        incomplete(
            Obligation::NormalNotMinimal { position: 1 },
            Established::NotCertified
        )
    );
    assert_eq!(
        verdict("bridge-out-of-range"),
        incomplete(Obligation::NormalOutOfRange, Established::NotCertified)
    );
    // A bridge needs evidence about every system: an unproven one, a
    // reduced one, and one the fuel leaves unresolved leave it open.
    assert_eq!(
        verdict("bridge-unproven"),
        incomplete(
            Obligation::BridgeUnproved { position: 0 },
            Established::NotCertified
        )
    );
    assert_eq!(
        verdict("bridge-reduced"),
        incomplete(
            Obligation::BridgeUnproved { position: 2 },
            Established::NotCertified
        )
    );
    assert_eq!(
        verdict("bridge-unresolved"),
        incomplete(
            Obligation::UnresolvedMember { position: 1 },
            Established::NotCertified
        ),
        "the same verdict as a committed fixture the kernel decides"
    );
    let optimum_unresolved = Request {
        claim: ClaimClass::GlobalOptimal,
        ..request("bound-over-unresolved")
    };
    let sizes: Vec<u64> = gnaf::expand(&optimum_unresolved.carrier)
        .into_iter()
        .map(|selector| {
            certification::trials(&optimum_unresolved, selector)
                .expect("evaluates")
                .2
        })
        .collect();
    let smallest = sizes
        .iter()
        .position(|size| sizes.iter().all(|other| size <= other))
        .expect("a smallest system") as u64;
    let first_unresolved = gnaf::statuses(&optimum_unresolved)
        .expect("evaluates")
        .iter()
        .position(|(_, _, status)| *status == Status::Unresolved)
        .expect("an unresolved system") as u64;
    let unresolved_base = fixture("lb-over-unresolved").certificate;
    assert_eq!(
        certification::check(
            &optimum_unresolved,
            &Certificate {
                claim: ClaimClass::GlobalOptimal,
                route: Route::Representation {
                    normal: smallest,
                    bridge: Some(Bridge {
                        slope: 0,
                        offset: 0
                    }),
                },
                premises: vec![Premise::Measured; unresolved_base.premises.len()],
                ..unresolved_base
            },
        )
        .expect("within capacity"),
        incomplete(
            Obligation::UnresolvedMember {
                position: first_unresolved
            },
            Established::NotCertified
        ),
        "a bridge over a system the fuel leaves unresolved is open at it"
    );
    // A bridge that is not valid is never certified, whatever its numbers:
    // every system at every slope and offset below the committed ones.
    let base = fixture("bridge-attained").certificate;
    let mut valid = 0;
    for slope in 0..=8 {
        for offset in 0..=70 {
            let verdict = check(
                "bridge-straight-line",
                &Certificate {
                    route: Route::Representation {
                        normal: 1,
                        bridge: Some(Bridge { slope, offset }),
                    },
                    ..base.clone()
                },
            );
            let holds = costs
                .iter()
                .all(|(steps, size)| *steps >= slope * size + offset);
            let attains = costs[1].0 == slope * costs[1].1 + offset;
            assert_eq!(
                is_certified(&verdict),
                holds && attains,
                "slope {slope}, offset {offset}: {verdict:?}"
            );
            valid += usize::from(holds && attains);
        }
    }
    assert!(valid >= 2, "several bridges are valid and attained");
    // A system bounded exactly at the bridge's line meets it: the bound is
    // evidence of cost at least the line, not above it.
    let posed = request("bridge-straight-line");
    let mut tight = 0;
    for slope in 0..=8 {
        for offset in 0..=70 {
            let holds = costs
                .iter()
                .all(|(steps, size)| *steps >= slope * size + offset);
            if !(holds && costs[1].0 == slope * costs[1].1 + offset) {
                continue;
            }
            let mut premises = base.premises.clone();
            for (position, (_, size)) in costs
                .iter()
                .enumerate()
                .filter(|(position, _)| *position != 1)
            {
                let limit = slope * size + offset;
                let selector = gnaf::expand(&posed.carrier)[position];
                let (trials, charge, _) =
                    certification::trials(&posed, selector).expect("evaluates");
                let found = (0..1_u64 << trials.len()).find_map(|mask| {
                    let positions: Vec<u64> = (0..trials.len() as u64)
                        .filter(|index| mask >> index & 1 == 1)
                        .collect();
                    let sum: Option<u64> = positions
                        .iter()
                        .map(|index| match trials[*index as usize] {
                            Trial::Agrees { steps } => Some(steps),
                            Trial::Disagrees | Trial::Unknown => None,
                        })
                        .sum::<Option<u64>>()
                        .map(|steps| steps + charge);
                    (sum == Some(limit) && limit < costs[position].0).then_some(positions)
                });
                if let Some(positions) = found {
                    premises[position] = Premise::PaysAtLeast { positions };
                    tight += 1;
                }
            }
            let verdict = check(
                "bridge-straight-line",
                &Certificate {
                    route: Route::Representation {
                        normal: 1,
                        bridge: Some(Bridge { slope, offset }),
                    },
                    premises,
                    ..base.clone()
                },
            );
            assert!(
                is_certified(&verdict),
                "slope {slope}, offset {offset}: a system bounded exactly at the line meets it: {verdict:?}"
            );
        }
    }
    assert!(tight > 0, "some system is bounded exactly at some line");
}

/// §17.18: every verdict is a closed claim status.
#[allow(clippy::too_many_lines)]
fn gc_08() {
    for (name, expected) in [
        ("verifier-mismatch", Refusal::VerifierMismatch),
        ("claim-mismatch", Refusal::ClaimMismatch),
        ("scope-mismatch", Refusal::ScopeMismatch),
        ("objective-mismatch", Refusal::ObjectiveMismatch),
        ("route-incompatible", Refusal::RouteIncompatible),
        ("reused-enlarged", Refusal::UniverseMismatch),
        (
            "request-rejected",
            Refusal::RequestRejected {
                rejection: Rejection::UncoveredScope,
            },
        ),
        (
            "request-rejected-cached",
            Refusal::RequestRejected {
                rejection: Rejection::CachedUniverse,
            },
        ),
        (
            "lb-premise-omitted",
            Refusal::MissingPremise { position: 5 },
        ),
        ("lb-extra-premise", Refusal::ExtraPremise { count: 6 }),
    ] {
        assert_eq!(verdict(name), refused(expected), "{name}");
    }
    // No request whose carrier is not a grammar reaches a certificate's
    // premises: each is rejected as the request is, which is why the model
    // has no refusal of its own for the carrier.
    let cached = fixture("request-rejected-cached").certificate;
    for (name, rejection) in [
        (
            "reject-internal-plan-universe",
            Rejection::InternalPlanUniverse,
        ),
        (
            "reject-optimizer-defined-universe",
            Rejection::OptimizerDefinedUniverse,
        ),
        ("reject-cached-universe", Rejection::CachedUniverse),
        ("reject-discovered-universe", Rejection::DiscoveredUniverse),
    ] {
        assert_eq!(
            check(name, &cached),
            refused(Refusal::RequestRejected { rejection }),
            "{name}"
        );
    }
    // The same problem on an enlarged machine is another universe: nothing
    // certified for one is certified for the other (GNAF-VEC-08).
    let small = request("argmin-over-systems");
    let enlarged = request("vec08-enlarged");
    assert_ne!(small.universe, enlarged.universe);
    assert_eq!(enlarged.machine.fuel, small.machine.fuel * 2);
    assert_eq!(
        gnaf::answer(&enlarged).expect("answers"),
        gnaf::answer(&small).expect("answers"),
        "the enlarged machine answers the same, as another universe"
    );
    let reused = Certificate {
        universe: small.universe,
        ..fixture("lb-argmin-over-systems").certificate
    };
    assert_eq!(
        check("vec08-enlarged", &reused),
        refused(Refusal::UniverseMismatch)
    );
    // A bound certified for the smaller machine is not read for the larger
    // one even when its premises are re-evaluated: the identity binds.
    let restated = Certificate {
        universe: enlarged.universe,
        ..reused
    };
    assert!(
        is_certified(&check("vec08-enlarged", &restated)),
        "restated for its own universe, the evidence is evaluated afresh"
    );
    // Every incomplete verdict names its first obligation and a status that
    // claims no optimality; every certified one is a closed result.
    let mut statuses = BTreeSet::new();
    let mut certified_results = BTreeSet::new();
    let mut incompletes = 0;
    for (_, _, fixture) in committed() {
        match &fixture.expected {
            Verdict::Refused { .. } => {}
            Verdict::Certified { result } => {
                certified_results.insert(match result {
                    Certified::Optimum { .. } => "optimum",
                    Certified::Argmin { .. } => "argmin",
                    Certified::Frontier { .. } => "frontier",
                });
            }
            Verdict::Incomplete { established, .. } => {
                incompletes += 1;
                statuses.insert(match established {
                    Established::NotCertified => "not_certified",
                    Established::CorrectOnly { .. } => "correct_only",
                    Established::LocallyNormalized { .. } => "locally_normalized",
                    Established::RepresentationMinimal { .. } => "representation_minimal",
                });
            }
        }
    }
    assert_eq!(
        statuses,
        BTreeSet::from([
            "correct_only",
            "locally_normalized",
            "not_certified",
            "representation_minimal"
        ])
    );
    assert_eq!(
        certified_results,
        BTreeSet::from(["argmin", "frontier", "optimum"])
    );
    assert!(incompletes >= 20, "{incompletes}");
    // A claim class the checker does not certify is refused whatever the
    // evidence: every other class of §12.4 is a claim mismatch.
    let base = fixture("lb-argmin-over-systems").certificate;
    for claim in [
        ClaimClass::Exact,
        ClaimClass::NormalForm,
        ClaimClass::RepresentationMinimal,
        ClaimClass::BestKnown,
        ClaimClass::InstanceOptimal { alpha: 2, beta: 1 },
    ] {
        assert_eq!(
            check(
                "argmin-over-systems",
                &Certificate {
                    claim,
                    ..base.clone()
                }
            ),
            refused(Refusal::ClaimMismatch)
        );
    }
    for scope in [Scope::CalculusPrograms, Scope::RustPrograms] {
        assert_eq!(
            check(
                "argmin-over-systems",
                &Certificate {
                    scope,
                    ..base.clone()
                }
            ),
            refused(Refusal::ScopeMismatch)
        );
    }
    assert_eq!(
        check(
            "argmin-over-systems",
            &Certificate {
                objective: Objective::Vector,
                ..base
            }
        ),
        refused(Refusal::ObjectiveMismatch)
    );
}

/// §17.18: the certification library is kernel-checked and audited.
#[allow(clippy::too_many_lines)]
fn gc_09() {
    let registry = library::registry();
    assert_eq!(registry.spec, library::REGISTRY_SPEC);
    assert_eq!(registry.toolchain, lexlean::LEAN_TOOLCHAIN);
    assert_eq!(
        registry.modules,
        library::MODULES
            .iter()
            .map(|stem| library::module_name(stem))
            .collect::<Vec<_>>()
    );
    // The library is exactly its registered modules, each a native source
    // (`GC-11` holds the rest of what it is: no Lean file, and the generated
    // Lean is the project's build).
    let on_disk: BTreeSet<String> = std::fs::read_dir(
        repo_root()
            .join(library::PROJECT_DIR)
            .join("src")
            .as_std_path(),
    )
    .expect("the library's sources")
    .map(|entry| {
        let name = entry
            .expect("entry")
            .file_name()
            .to_string_lossy()
            .into_owned();
        name.strip_suffix(".lex.tex")
            .unwrap_or_else(|| panic!("{name}: a source of the library"))
            .to_owned()
    })
    .collect();
    assert_eq!(
        on_disk,
        library::MODULES
            .iter()
            .map(|stem| (*stem).to_owned())
            .collect(),
        "the library has exactly the registered modules"
    );
    // Every declaration a Lean command can name is audited under the axioms
    // its row pins, and those are among the three the library may reach.
    let registered: BTreeSet<String> = registry
        .declaration
        .iter()
        .map(|declaration| declaration.name.clone())
        .collect();
    assert!(
        registered.len() > 100,
        "the native modules state their declarations"
    );
    for declaration in &registry.declaration {
        assert!(
            declaration
                .axioms
                .iter()
                .all(|axiom| library::AXIOMS.contains(&axiom.as_str())),
            "{}: only the expected axioms are pinned",
            declaration.name
        );
    }
    // The headline theorems are registered with the library's statements.
    for theorem in [
        "check_sound",
        "lower_bound_attainment",
        "frontier_complete",
        "normality_never_certifies",
        "certified_optimum_is_normal",
        "schema_cheaper",
        "universe_position",
    ] {
        assert!(
            registered.contains(&format!("LexLeanCertification.{theorem}")),
            "{theorem} is registered"
        );
    }
    if !support::lean_backed("GC-09") {
        return;
    }
    let search = library::search();
    let root = tempfile::Builder::new()
        .prefix("lexlean-certification-library-")
        .tempdir()
        .expect("tempdir");
    // The library holds of every certified committed certificate: its
    // soundness theorem is instantiated at each, and the instances and the
    // library are audited and replayed.
    let mut instances = String::from(
        "import LexLeanCertification.Soundness\nimport LexLeanTarget.GnafCertificates\n\
         open LexLeanTarget LexLeanCertification\n\nnamespace LexLeanCertification.Instances\n\n",
    );
    let mut names = Vec::new();
    for (_, _, fixture) in committed() {
        let id = crate::calculus::identifier(&fixture.name);
        let verdict_theorem = format!("GnafCertificates.{id}Verdict");
        match (&fixture.expected, &fixture.certificate.route) {
            (Verdict::Certified { result }, _) => {
                instances.push_str(&format!(
                    "def {id}Sound := check_sound _ _ _ {verdict_theorem}\n\n"
                ));
                names.push(format!("{id}Sound"));
                match result {
                    Certified::Optimum { .. } => {
                        instances.push_str(&format!(
                            "def {id}Bounded := lower_bound_attainment _ _ _ _ (Or.inl {verdict_theorem})\n\n\
                             def {id}Normal := certified_optimum_is_normal _ _ _ _ (Or.inl {verdict_theorem})\n\n"
                        ));
                        names.push(format!("{id}Bounded"));
                        names.push(format!("{id}Normal"));
                    }
                    Certified::Argmin { .. } => {
                        instances.push_str(&format!(
                            "def {id}Bounded := lower_bound_attainment _ _ _ _ (Or.inr {verdict_theorem})\n\n\
                             def {id}Normal := certified_optimum_is_normal _ _ _ _ (Or.inr {verdict_theorem})\n\n"
                        ));
                        names.push(format!("{id}Bounded"));
                        names.push(format!("{id}Normal"));
                    }
                    Certified::Frontier { .. } => {
                        instances.push_str(&format!(
                            "def {id}Complete := frontier_complete _ _ _ {verdict_theorem}\n\n"
                        ));
                        names.push(format!("{id}Complete"));
                    }
                }
            }
            (_, Route::Normality { system }) => {
                instances.push_str(&format!(
                    "theorem {id}Uncertified (x : Certification.Certified) :\n    \
                     Certification.check GnafFixtures.{request}Request GnafCertificates.{id}Certificate \
                     ≠ .certified x := normality_never_certifies _ _ {system} rfl x\n\n",
                    request = crate::calculus::identifier(&fixture.request)
                ));
                names.push(format!("{id}Uncertified"));
            }
            _ => {}
        }
    }
    // The library's preservation theorems are stated in the module's own
    // relations, and the ascriptions below fix their statements: a library
    // that defined a second relation, or proved another statement, would not
    // elaborate here.
    instances.push_str(
        "def preservesFromImproves : \u{2200} (req : Gnaf.Request) (g : Gnaf.Grammar) \
         (s : Gnaf.Selector) (schema : Certification.Schema),\n    \
         Certification.improves req g s schema = true \u{2192} \
         Certification.preserves req g s schema = true := improves_preserves\n\n\
         def preservesAtEveryFuel : \u{2200} (req : Gnaf.Request) (g : Gnaf.Grammar) \
         (s q : Gnaf.Selector) (schema : Certification.Schema),\n    \
         Certification.rewrite schema s = some q \u{2192} \
         Certification.eligible req g s schema = true \u{2192} \u{2200} (F : Nat),\n    \
         Certification.sameObservationsAt req g F s q req.domain = true \u{2227} \
         Certification.strictlyCheaperAt req g F s q req.domain = true := schema_preserves_at\n\n",
    );
    names.push("preservesFromImproves".to_owned());
    names.push("preservesAtEveryFuel".to_owned());
    instances.push_str("end LexLeanCertification.Instances\n");
    let output = library::compile(root.path(), "CertificationInstances", &instances, &search);
    assert!(
        output.status.success(),
        "Lean accepts the soundness theorem at every certified certificate:\n{}",
        crate::preservation::joined(&output)
    );
    assert!(names.len() >= 20, "many instances: {}", names.len());
    // Exact axioms for every library declaration and every instance.
    let mut audit = String::from("import CertificationInstances\nopen LexLeanCertification\n\n");
    for declaration in &registry.declaration {
        audit.push_str(&format!("#print axioms {}\n", declaration.name));
    }
    for name in &names {
        audit.push_str(&format!(
            "#print axioms LexLeanCertification.Instances.{name}\n"
        ));
    }
    let output = library::compile(root.path(), "Audit", &audit, &{
        let mut with_instances = vec![root.path().join("build")];
        with_instances.extend(search.clone());
        with_instances
    });
    let printed = library::printed_axioms(&crate::preservation::joined(&output));
    for declaration in &registry.declaration {
        assert_eq!(
            printed.get(&declaration.name),
            Some(&declaration.axioms),
            "{}: exactly the registered axioms",
            declaration.name
        );
    }
    for name in &names {
        let printed = printed
            .get(&format!("LexLeanCertification.Instances.{name}"))
            .unwrap_or_else(|| panic!("{name}: audited"));
        assert!(
            printed
                .iter()
                .all(|axiom| library::AXIOMS.contains(&axiom.as_str())),
            "{name}: only the expected axioms: {printed:?}"
        );
    }
    // The kernel replays the library, its instances, and what they import.
    let replay = library::replay(root.path(), "CertificationInstances", &search);
    assert!(
        replay.status.success(),
        "leanchecker replays the library and its instances:\n{}",
        crate::preservation::joined(&replay)
    );
    // The library detects a weakened checker: a copy of the generated
    // `Certification` that accepts a lower bound it should not (a tie
    // admitted as strictly above) is refused by the soundness proof.
    let rejected = weakened_library();
    assert!(
        rejected.0.contains("Lower") || rejected.0.contains("Check"),
        "the library's proof of the lower-bound route fails on a weakened checker: {rejected:?}"
    );
    // A false lemma planted in a staged copy of a library module is refused
    // by Lean: the schema's strict saving reversed.
    let false_lemma = planted_false_lemma();
    assert!(
        false_lemma.contains("error"),
        "Lean refuses a library whose schema_cheaper says the saving is an increase: {false_lemma}"
    );
    // An axiom added to a theorem is seen by the exact axiom audit: its proof
    // replaced by `sorryAx` is accepted by the kernel, and what Lean prints
    // of it is no longer what its row pins.
    let added = planted_axiom();
    let registered_axioms = registry
        .declaration
        .iter()
        .find(|declaration| declaration.name == PLANTED_THEOREM)
        .map(|declaration| declaration.axioms.clone())
        .expect("the planted theorem is registered");
    assert_ne!(
        added.get(PLANTED_THEOREM),
        Some(&registered_axioms),
        "the audit sees the planted axiom"
    );
    assert!(
        added
            .get(PLANTED_THEOREM)
            .is_some_and(|axioms| axioms.iter().any(|axiom| axiom == "sorryAx")),
        "Lean reports the planted axiom: {added:?}"
    );
}

/// The theorem the plants change.
const PLANTED_THEOREM: &str = "LexLeanCertification.schema_cheaper";

/// The native source of `stem`.
/// The source of a native module as its data form: the canonical JSON the
/// textual edits below are made in, whichever form the committed source is
/// written in (§17.21.8).
fn native_source(stem: &str) -> String {
    let text = std::fs::read_to_string(repo_root().join(library::source_path(stem)).as_std_path())
        .unwrap_or_else(|error| panic!("{stem}: {error}"));
    lexlean::sdk::to_data_form(&text).unwrap_or_else(|error| panic!("{stem}: {error}"))
}

/// The Lean of a native module whose source is `source`.
fn generated_text(source: &str) -> String {
    let core = lexlean::ir::core::CoreModule::from_source(source, "1.2")
        .expect("the planted module is core data");
    lexlean::backend::core::lean_text(&core).expect("the planted module renders")
}

/// The `type` and `value` node numbers of the row named `name` in a native
/// source, which states each row's keys in sorted order.
fn row_nodes(source: &str, name: &str) -> (u64, u64) {
    let start = source
        .find(&format!("\"name\":\"{name}\""))
        .unwrap_or_else(|| panic!("{name} is a row"));
    let rest = &source[start..];
    let number = |key: &str| -> u64 {
        let at = rest.find(key).unwrap_or_else(|| panic!("{name} has {key}"));
        rest[at + key.len()..]
            .chars()
            .take_while(char::is_ascii_digit)
            .collect::<String>()
            .parse()
            .expect("a node number")
    };
    (number("\"type\":"), number("\"value\":"))
}

/// A staged build directory holding the library compiled in order, with
/// `stem` compiled from `text` instead of the native module, over the
/// environment; returns the directory and Lean's output on `stem`.
fn staged_with(stem: &str, text: &str) -> (tempfile::TempDir, std::process::Output) {
    let root = tempfile::Builder::new()
        .prefix("lexlean-planted-library-")
        .tempdir()
        .expect("tempdir");
    // Lean resolves a module by its first path component, so the staged
    // module needs the library's other compiled modules beside it.
    let staged = root.path().join("build/LexLeanCertification");
    std::fs::create_dir_all(&staged).expect("mkdir");
    for entry in
        std::fs::read_dir(library::library().join("LexLeanCertification")).expect("readdir")
    {
        let entry = entry.expect("entry");
        std::fs::copy(entry.path(), staged.join(entry.file_name())).expect("copy");
    }
    let output = library::compile(
        root.path(),
        &library::module_name(stem),
        text,
        &[
            library::environment().clone(),
            crate::preservation::shipped().build.clone(),
        ],
    );
    (root, output)
}

/// What Lean prints of the axioms of [`PLANTED_THEOREM`] when its proof is
/// `sorryAx`: the four nodes the proof needs are appended to the module's
/// table and the row's `value` points at them.
fn planted_axiom() -> BTreeMap<String, Vec<String>> {
    let source = native_source("Schemas");
    let (statement, proof) = row_nodes(&source, PLANTED_THEOREM);
    let json: serde_json::Value = serde_json::from_str(
        source
            .split("\\coredata{")
            .nth(1)
            .and_then(|rest| rest.split("}\n\\end{coremodule}").next())
            .expect("the core data"),
    )
    .expect("the data is JSON");
    let next = json["nodes"].as_array().expect("nodes").len();
    let table = format!(
        ",{{\"k\":\"c\",\"n\":\"sorryAx\",\"u\":[{{\"k\":\"z\"}}]}},\
         {{\"f\":{next},\"k\":\"a\",\"x\":{statement}}},\
         {{\"k\":\"c\",\"n\":\"Bool.false\",\"u\":[]}},\
         {{\"f\":{},\"k\":\"a\",\"x\":{}}}",
        next + 1,
        next + 2
    );
    let joined = "],\"proof_nodes\":";
    assert_eq!(source.matches(joined).count(), 1, "one node table");
    let planted = source
        .replacen(joined, &format!("{table}{joined}"), 1)
        .replacen(
            &format!("\"type\":{statement},\"value\":{proof}}}"),
            &format!("\"type\":{statement},\"value\":{}}}", next + 3),
            1,
        );
    assert_ne!(planted, source, "the proof is replaced");
    let (root, output) = staged_with("Schemas", &generated_text(&planted));
    assert!(
        output.status.success(),
        "the kernel accepts a proof by `sorryAx`:\n{}",
        crate::preservation::joined(&output)
    );
    let audit = format!("import LexLeanCertification.Schemas\n#print axioms {PLANTED_THEOREM}\n");
    let audited = library::compile(
        root.path(),
        "PlantedAudit",
        &audit,
        &[
            library::environment().clone(),
            crate::preservation::shipped().build.clone(),
        ],
    );
    library::printed_axioms(&crate::preservation::joined(&audited))
}

/// Lean's output on a staged copy of `Schemas` whose `schema_cheaper` states
/// what `improves_preserves` states and keeps its own proof: the row's
/// statement is exchanged for another theorem's, so the module's proof no
/// longer proves what it claims.
fn planted_false_lemma() -> String {
    let source = native_source("Schemas");
    let (statement, proof) = row_nodes(&source, PLANTED_THEOREM);
    let (other, _) = row_nodes(&source, "LexLeanCertification.improves_preserves");
    assert_ne!(statement, other, "the two statements differ");
    let plant = format!("\"type\":{statement},\"value\":{proof}}}");
    assert_eq!(
        source.matches(&plant).count(),
        1,
        "the lemma's statement is planted once"
    );
    let planted = source.replace(&plant, &format!("\"type\":{other},\"value\":{proof}}}"));
    let (_root, output) = staged_with("Schemas", &generated_text(&planted));
    assert!(
        !output.status.success(),
        "Lean accepts a library whose schema_cheaper claims another statement"
    );
    crate::preservation::joined(&output)
}

/// The module of the library that Lean rejects when compiled over a copy of
/// `Certification` whose strict lower-bound test is weakened, and its output.
fn weakened_library() -> (String, String) {
    let sources = {
        let mut text = std::fs::read_to_string(
            repo_root()
                .join("compiler/expected/build/modules/LexLeanTarget/Certification.lean")
                .as_std_path(),
        )
        .unwrap_or_else(|_| {
            support::rendered(&support::P::compiler())
                .modules
                .iter()
                .find(|module| module.lean_module == "LexLeanTarget.Certification")
                .expect("Certification")
                .lean_text
                .clone()
        });
        let plant = "(if (Nat.blt (bound) (cost)) then Option.none else Option.some (Obligation.memberUnproven (position)))";
        assert_eq!(
            text.matches(plant).count(),
            1,
            "the strict test is planted once"
        );
        text = text.replace(
            plant,
            "(if (Nat.ble (bound) (cost)) then Option.none else Option.some (Obligation.memberUnproven (position)))",
        );
        text
    };
    let root = tempfile::Builder::new()
        .prefix("lexlean-weakened-checker-")
        .tempdir()
        .expect("tempdir");
    let upstream = vec![
        library::environment().clone(),
        crate::preservation::shipped().build.clone(),
    ];
    // Lean resolves a module by its first path component, so the planted
    // `Certification` shadows the whole `LexLeanTarget` of the environment:
    // the environment's compiled modules are copied beside it.
    let target = root.path().join("build/LexLeanTarget");
    std::fs::create_dir_all(&target).expect("mkdir");
    for entry in std::fs::read_dir(library::environment().join("LexLeanTarget")).expect("readdir") {
        let entry = entry.expect("entry");
        std::fs::copy(entry.path(), target.join(entry.file_name())).expect("copy");
    }
    // The planted `Certification` shadows the environment's for the library.
    let planted = library::compile(
        root.path(),
        "LexLeanTarget.Certification",
        &sources,
        &upstream,
    );
    assert!(
        planted.status.success(),
        "the weakened checker is valid Lean"
    );
    let mut search = vec![root.path().join("build")];
    search.extend(upstream);
    for stem in library::MODULES {
        let output = library::compile(
            root.path(),
            &library::module_name(stem),
            &library::library_text(stem),
            &search,
        );
        if !output.status.success() {
            return (stem.to_owned(), crate::preservation::joined(&output));
        }
    }
    panic!("the library accepts a weakened checker");
}

/// §17.18: the certification library is LexLean source.
#[allow(clippy::too_many_lines)]
fn gc_11() {
    let (registry, modules) = library::native();
    assert!(
        !registry.declaration.is_empty(),
        "the native modules state declarations"
    );
    assert_eq!(modules.len(), library::MODULES.len());
    support::assert_library_has_no_handwritten_lean(
        library::PREFIX,
        "tests/certification-library/oracle",
    );
    // The library is LexLean source and nothing else: no Lean file lies
    // anywhere in its project, and its sources are the listed modules.
    let project = repo_root().join(library::PROJECT_DIR);
    let mut stack = vec![project.as_std_path().to_path_buf()];
    let mut sources = BTreeSet::new();
    while let Some(directory) = stack.pop() {
        for entry in std::fs::read_dir(&directory).expect("the library project") {
            let path = entry.expect("an entry").path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            let name = path.to_string_lossy().into_owned();
            assert!(
                !name.ends_with(".lean"),
                "{name}: an independently authored Lean file lies in the library project"
            );
            // Compared as path components: a Windows checkout mixes the
            // separators of the repository root and of the library's
            // relative directory.
            if let Some(stem) = path
                .strip_prefix(project.as_std_path().join("src"))
                .ok()
                .map(|rest| rest.to_string_lossy().replace('\\', "/"))
                .and_then(|rest| rest.strip_suffix(".lex.tex").map(str::to_owned))
            {
                sources.insert(stem);
            }
        }
    }
    assert_eq!(
        sources,
        library::MODULES
            .iter()
            .map(|stem| (*stem).to_owned())
            .collect::<BTreeSet<_>>(),
        "the native sources are the listed modules"
    );
    // The registry lists the modules and nothing else.
    let table: toml::Table =
        std::fs::read_to_string(repo_root().join(library::REGISTRY).as_std_path())
            .expect("the registry")
            .parse()
            .expect("the registry is TOML");
    assert_eq!(
        table.keys().map(String::as_str).collect::<Vec<_>>(),
        ["modules", "spec", "toolchain"],
        "the registry holds no declaration list"
    );
    // The library is an ordinary closed project: it locks, checks, and
    // builds, and the Lean it generates is, byte for byte, the Lean the kernel
    // check compiles, so neither a stale nor an edited generated module can
    // stand in for the source.
    let project = support::P::certification_project();
    project.relock();
    let built = project.build_ok();
    let build_root = project.build_dir(&built.build_id.expect("a build id"));
    for stem in library::MODULES {
        let generated = std::fs::read_to_string(
            build_root
                .join("modules")
                .join(format!("LexLeanCertification/{stem}.lean"))
                .as_std_path(),
        )
        .unwrap_or_else(|error| panic!("{stem}: no generated module: {error}"));
        assert_eq!(
            generated,
            library::library_text(stem),
            "{stem}: the project's generated Lean is the Lean the kernel check compiles"
        );
    }
    // A planted drift of a source is visible in what is generated.
    let drift = support::P::certification_project();
    let trials = drift.read("src/Trials.lex.tex");
    assert!(trials.contains("\"transparency\":\"semireducible\""));
    drift.write(
        "src/Trials.lex.tex",
        &trials.replacen(
            "\"transparency\":\"semireducible\"",
            "\"transparency\":\"reducible\"",
            1,
        ),
    );
    drift.relock();
    let rebuilt = drift.build_ok();
    let drifted = std::fs::read_to_string(
        drift
            .build_dir(&rebuilt.build_id.expect("a build id"))
            .join("modules/LexLeanCertification/Trials.lean")
            .as_std_path(),
    )
    .expect("the generated module");
    assert_ne!(
        drifted,
        library::library_text("Trials"),
        "a drifted source generates different Lean"
    );
    // A source that is missing, imports a foreign module, or weakens a policy
    // is refused.
    let first = library::source_path(library::MODULES[0]);
    let original = native_source(library::MODULES[0]);
    let planted = |edit: &dyn Fn(&str) -> Option<String>| {
        library::check_sources(&|path| {
            if path == first {
                edit(&original).ok_or_else(|| format!("{path}: no such file"))
            } else {
                std::fs::read_to_string(repo_root().join(path).as_std_path())
                    .map_err(|error| format!("{path}: {error}"))
            }
        })
        .map(|_| ())
    };
    let error = planted(&|_| None).expect_err("a missing source is refused");
    assert!(error.contains("no such file"), "{error}");
    assert!(
        original.contains("\"imports\":[\"Init\""),
        "{original:.200}"
    );
    let error = planted(&|text| {
        Some(text.replace("\"imports\":[\"Init\"", "\"imports\":[\"Init\",\"Mathlib\""))
    })
    .expect_err("a foreign import is refused");
    assert!(error.contains("imports `Mathlib`"), "{error}");
    let error = planted(&|text| Some(text.replace("\"kind\":\"exact\"", "\"kind\":\"allow\"")))
        .expect_err("an inexact policy is refused");
    assert!(error.contains("does not pin its axioms exactly"), "{error}");
    let error = planted(&|text| {
        Some(text.replace(
            "\"spec\":\"lexlean/core-module/2\"",
            "\"spec\":\"lexlean/core-module/1\"",
        ))
    })
    .expect_err("a first-schema module carrying second-schema data is refused");
    assert!(error.contains("requires"), "{error}");
}

/// §17.18: generated, deterministic, content-addressed.
#[allow(clippy::too_many_lines)]
fn gc_10() {
    let files = generated::files();
    for (path, bytes) in &files {
        assert_eq!(
            std::fs::read(repo_root().join(path).as_std_path())
                .unwrap_or_else(|error| panic!("{path}: {error}")),
            *bytes,
            "{path} equals its generator"
        );
    }
    assert_eq!(generated::files(), files, "generation is deterministic");
    for expected in [
        generated::PATH,
        "schemas/gnaf-certificate.schema.json",
        "schemas/gnaf-certificate-fixture.schema.json",
    ] {
        assert!(files.contains_key(expected), "{expected} is generated");
    }
    // The dependency manifest names the certificate profiles, the capacity,
    // and what a certificate does not claim.
    let manifest = read_json("compiler/gnaf.manifest.json");
    let profiles: BTreeSet<&str> = manifest["profiles"]
        .as_array()
        .expect("profiles")
        .iter()
        .filter_map(|profile| profile["name"].as_str())
        .collect();
    for profile in [
        "certificate",
        "certificate-proof",
        "certificate-transcription",
    ] {
        assert!(profiles.contains(profile), "the manifest names {profile}");
    }
    let lines = |field: &str| -> String {
        manifest[field]
            .as_array()
            .expect("lines")
            .iter()
            .filter_map(Json::as_str)
            .collect::<Vec<_>>()
            .join("\n")
    };
    assert!(lines("restrictions").contains(&format!("{MAX_PREMISES} premises")));
    assert!(lines("non_claims").contains("is not a certified global claim"));
    let main = read("compiler/src/Main.lex.tex");
    for import in ["Certification", "GnafCertificates"] {
        assert!(
            main.contains(&format!("\\importmodule{{{import}}}")),
            "{import}"
        );
    }
    // Identities are recomputed from content, and moved by any change.
    let base = fixture("lb-argmin-over-systems");
    assert_eq!(
        base.id,
        certification::certificate_id(&base.certificate).expect("an identity")
    );
    let mut ids = BTreeSet::new();
    ids.insert(base.id);
    let variants: Vec<Certificate> = vec![
        Certificate {
            route: Route::LowerBound {
                bound: 168,
                attainers: vec![3],
            },
            ..base.certificate.clone()
        },
        Certificate {
            route: Route::LowerBound {
                bound: 167,
                attainers: vec![2],
            },
            ..base.certificate.clone()
        },
        Certificate {
            route: Route::Reduction,
            ..base.certificate.clone()
        },
        Certificate {
            premises: base.certificate.premises[1..].to_vec(),
            ..base.certificate.clone()
        },
        Certificate {
            premises: base
                .certificate
                .premises
                .iter()
                .enumerate()
                .map(|(position, premise)| {
                    if position == 0 {
                        Premise::Measured
                    } else {
                        premise.clone()
                    }
                })
                .collect(),
            ..base.certificate.clone()
        },
        Certificate {
            claim: ClaimClass::ArgminComplete,
            ..base.certificate.clone()
        },
        Certificate {
            verifier: "lexlean/gnaf-certification/0".to_owned(),
            ..base.certificate.clone()
        },
    ];
    for variant in &variants {
        let id = certification::certificate_id(variant).expect("an identity");
        assert!(ids.insert(id), "a changed certificate has another identity");
    }
    // The verdict moves with the evidence, not with the identity.
    assert_ne!(
        check("argmin-over-systems", &variants[0]),
        base.expected,
        "a moved bound is another verdict"
    );
    // An identity is the certificate's content: two committed fixtures share
    // one exactly when their certificates are equal (the binding refusals
    // present one certificate to different requests).
    let all = committed();
    for left in &all {
        for right in &all {
            assert_eq!(
                left.2.id == right.2.id,
                left.2.certificate == right.2.certificate,
                "{} and {}: an identity is its certificate's content",
                left.2.name,
                right.2.name
            );
        }
    }
    // The negative: nothing in a certificate names an optimizer, a cache, or
    // a discovered candidate; the closed forms are exactly the schemas'.
    let certificate_schema = schema("gnaf-certificate.schema.json");
    assert_eq!(
        certificate_schema["properties"]["verifier"]["type"], "string",
        "the verifier is a stated name"
    );
    assert_eq!(certificate_schema["additionalProperties"], false);
}
