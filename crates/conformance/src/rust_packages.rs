//! The committed Rust packages of the calculus fixtures and the negative
//! package manifests (SPEC.md §17.16).
//!
//! Every fixture whose entry takes and returns first-order data is packaged
//! in each profile that admits its program, exporting the entry as `run`.
//! The negative manifests each misstate one thing about a package, and each
//! states the error the renderer must report.

use std::collections::BTreeMap;

use lexlean::calculus::rust::package::{
    holds_function, Errors, Export, Manifest, Passing, MANIFEST_SPEC,
};
use lexlean::calculus::rust::{self, Profile};
use lexlean::calculus::{Expr, Function, Program, Ty, Value, PROGRAM_SPEC};

use crate::calculus::cases;
use crate::support::repo_root;

/// The semantic ID (§21.4) of the published build of the `compiler` project,
/// whose `TargetFixtures` module states every fixture's program: the one
/// source every committed package binds.
///
/// # Panics
///
/// Panics if the committed build manifest is missing or malformed.
#[must_use]
pub fn compiler_semantic_id() -> String {
    let path = repo_root().join("compiler/expected/build/manifest.json");
    let manifest: serde_json::Value =
        serde_json::from_slice(&std::fs::read(path.as_std_path()).expect("the compiler build"))
            .expect("the compiler build manifest is JSON");
    manifest["semantic_id"]
        .as_str()
        .expect("the compiler build has a semantic ID")
        .to_owned()
}

/// The declarations the semantic module of a committed `.lex.tex` source
/// states.
pub(crate) fn declarations(source: &str) -> Result<Vec<serde_json::Value>, String> {
    let start = source
        .find("\\semanticdata{")
        .ok_or("the module states no semantic data")?
        + "\\semanticdata{".len();
    let end = source
        .rfind("\\end{semanticmodule}")
        .and_then(|end| source[..end].rfind('}'))
        .ok_or("the semantic data is not closed")?;
    let data: serde_json::Value = serde_json::from_str(&source[start..end])
        .map_err(|error| format!("the semantic data is not JSON: {error}"))?;
    data["declarations"]
        .as_array()
        .cloned()
        .ok_or_else(|| "the semantic data has no declarations".to_owned())
}

/// Check that a committed package binds the LexLean build that states its
/// program (§17.16): its sources are exactly the semantic ID of the
/// published `compiler` build, which the committed verification records
/// name; that build's source map of the module stating the fixture
/// (`TargetFixtures`, or `ReasoningFixtures` for a reasoning fixture) binds
/// the committed module source; and that source states the package's
/// program as the declaration `<fixture>Program`.
///
/// # Errors
///
/// Returns the first link of the binding that does not hold.
pub fn bound_to_source(committed: &Committed) -> Result<(), String> {
    let root = repo_root();
    let semantic = compiler_semantic_id();
    if committed.manifest.sources != [semantic.clone()] {
        return Err(format!(
            "sources {:?} are not the semantic ID {semantic} of the verified compiler build",
            committed.manifest.sources
        ));
    }
    let audit = root.join("compiler/expected/verify/audit");
    let reserved = format!("LexLeanAudit.A{}.", &semantic[..32]);
    let verified = std::fs::read_dir(audit.as_std_path())
        .map_err(|error| format!("{audit}: {error}"))?
        .flatten()
        .any(|entry| entry.file_name().to_string_lossy().starts_with(&reserved));
    if !verified {
        return Err(format!(
            "no verification of the build {semantic} is committed"
        ));
    }
    let module = crate::calculus::fixture_module(&committed.fixture);
    let map: serde_json::Value = serde_json::from_slice(
        &std::fs::read(
            root.join(format!(
                "compiler/expected/build/maps/LexLeanTarget/{module}.map.json"
            ))
            .as_std_path(),
        )
        .map_err(|error| format!("the {module} source map: {error}"))?,
    )
    .map_err(|error| format!("the {module} source map: {error}"))?;
    let source = std::fs::read(
        root.join(format!("compiler/src/{module}.lex.tex"))
            .as_std_path(),
    )
    .map_err(|error| format!("the {module} source: {error}"))?;
    let digest = lexlean::artifact::content_id::Sha256Digest::of(&source).to_hex();
    if map["semantic_id"] != semantic.as_str()
        || map["sources"][0]["path"] != format!("src/{module}.lex.tex").as_str()
        || map["sources"][0]["sha256"] != digest.as_str()
    {
        return Err(format!(
            "the build {semantic} does not bind the committed {module} source {digest}"
        ));
    }
    let name = format!("{}Program", crate::calculus::identifier(&committed.fixture));
    let text = String::from_utf8(source).map_err(|error| error.to_string())?;
    let stated = declarations(&text)?
        .into_iter()
        .find(|declaration| declaration["name"] == name.as_str())
        .ok_or_else(|| format!("the build states no declaration `{name}`"))?;
    if stated["body"] != lexlean::calculus::term::program(&committed.manifest.program) {
        return Err(format!(
            "the declaration `{name}` of the build states another program"
        ));
    }
    Ok(())
}

/// The crate name of a fixture's package in `profile`.
#[must_use]
pub fn crate_name(fixture: &str, profile: Profile) -> String {
    let suffix = match profile {
        Profile::Core => "core",
        Profile::Std => "std",
    };
    format!("{}_{suffix}", fixture.replace('-', "_"))
}

/// The manifest exporting `entry` of `program` as `run` in `profile`, each
/// parameter owned, its errors as the program's.
///
/// # Panics
///
/// Panics if the program is invalid.
#[must_use]
pub fn manifest(name: &str, program: &Program, entry: u64, profile: Profile) -> Manifest {
    let function = &program.functions[usize::try_from(entry).expect("entry")];
    let fallible = rust::fallible_functions(program).expect("a valid program")
        [usize::try_from(entry).expect("entry")];
    Manifest {
        spec: MANIFEST_SPEC.to_owned(),
        name: crate_name(name, profile),
        version: "1.0.0".to_owned(),
        profile: profile.target().to_owned(),
        program: program.clone(),
        exports: vec![Export {
            function: entry,
            name: "run".to_owned(),
            parameters: vec![Passing::Own; function.types.len()],
            errors: if fallible {
                Errors::Overflow
            } else {
                Errors::None
            },
        }],
        sources: vec![compiler_semantic_id()],
    }
}

/// One committed package.
pub struct Committed {
    /// Its directory under `compiler/rust/<target>/`.
    pub directory: String,
    /// The fixture whose program, entry, and arguments it runs.
    pub fixture: String,
    /// The profile it renders in.
    pub profile: Profile,
    /// Its manifest.
    pub manifest: Manifest,
}

/// Every committed package: one for every fixture with a first-order entry
/// boundary and every profile that renders its program, and two that pass
/// parameters by reference and by copy.
///
/// # Panics
///
/// Panics if a fixture is invalid.
#[must_use]
pub fn packages() -> Vec<Committed> {
    let mut out = Vec::new();
    for case in cases() {
        let fixture = &case.fixture;
        let function = &fixture.program.functions[usize::try_from(fixture.entry).expect("entry")];
        if function
            .types
            .iter()
            .chain([&function.result])
            .any(|ty| holds_function(&fixture.program, ty))
        {
            continue;
        }
        for profile in Profile::ALL {
            if rust::render(&fixture.program, profile).is_ok() {
                out.push(Committed {
                    directory: fixture.name.clone(),
                    fixture: fixture.name.clone(),
                    profile,
                    manifest: manifest(&fixture.name, &fixture.program, fixture.entry, profile),
                });
            }
        }
    }
    // Two packages pass parameters by reference and by copy: a list
    // borrowed, and fixed-width numbers copied, borrowed, and owned.
    let (lists, lists_entry) = fixture_program("lists");
    let mut borrowed = manifest("lists-borrowed", &lists, lists_entry, Profile::Std);
    borrowed.exports[0].parameters = vec![Passing::Borrow];
    out.push(Committed {
        directory: "lists-borrowed".to_owned(),
        fixture: "lists".to_owned(),
        profile: Profile::Std,
        manifest: borrowed,
    });
    let (fixed, fixed_entry) = fixture_program("fixed-width-u8");
    for profile in Profile::ALL {
        let mut passing = manifest("fixed-width-u8-passing", &fixed, fixed_entry, profile);
        passing.exports[0].parameters = vec![Passing::Copy, Passing::Borrow, Passing::Own];
        out.push(Committed {
            directory: "fixed-width-u8-passing".to_owned(),
            fixture: "fixed-width-u8".to_owned(),
            profile,
            manifest: passing,
        });
    }
    out
}

fn fixture_program(name: &str) -> (Program, u64) {
    let case = cases()
        .into_iter()
        .find(|case| case.fixture.name == name)
        .unwrap_or_else(|| panic!("fixture {name}"));
    (case.fixture.program, case.fixture.entry)
}

/// Every negative manifest by name: its JSON bytes and the error it must
/// fail with.
///
/// # Panics
///
/// Panics if a base fixture is missing or invalid.
#[must_use]
pub fn negatives() -> BTreeMap<String, (Vec<u8>, String)> {
    let (sum, sum_entry) = fixture_program("sum-to");
    let (lists, lists_entry) = fixture_program("lists");
    let (adts, adts_entry) = fixture_program("adt-evaluation");
    let (closures, closures_entry) = fixture_program("closures");
    let (booleans, booleans_entry) = fixture_program("booleans");
    let base = || manifest("sum-to", &sum, sum_entry, Profile::Std);
    let named = |export: &str| {
        let mut out = base();
        out.exports[0].name = export.to_owned();
        out
    };
    let mut cases: Vec<(&str, Manifest, &str)> = vec![
        (
            "identifier-keyword",
            named("match"),
            "identifier collision: export `match` is a Rust keyword",
        ),
        (
            "identifier-generated",
            named("v3"),
            "identifier collision: export `v3` has the shape of a generated name",
        ),
        (
            "identifier-runtime-function",
            named("nat_add"),
            "identifier collision: export `nat_add` is declared by the runtime",
        ),
        (
            "identifier-runtime-item",
            named("tick"),
            "identifier collision: export `tick` is declared by the runtime",
        ),
        (
            "identifier-not-snake-case",
            named("Run"),
            "identifier collision: export `Run` is not a lowercase snake-case Rust identifier",
        ),
        (
            "identifier-exported-twice",
            {
                let mut out = base();
                out.exports.push(out.exports[0].clone());
                out
            },
            "identifier collision: `run` is exported twice",
        ),
        (
            "identifier-reserved-crate",
            {
                let mut out = base();
                out.name = "core".to_owned();
                out
            },
            "identifier collision: `core` is not an available crate name",
        ),
        (
            "ownership-copy-list",
            {
                let mut out = manifest("lists", &lists, lists_entry, Profile::Std);
                out.exports[0].parameters = vec![Passing::Copy];
                out
            },
            "ownership mismatch: export `run` copies parameter 0",
        ),
        (
            "unsupported-function-boundary",
            {
                let mut out = manifest("closures", &closures, closures_entry, Profile::Std);
                // Function 1 takes a function value.
                out.exports[0].function = 1;
                out.exports[0].parameters = vec![Passing::Own, Passing::Own];
                out
            },
            "unsupported type: the boundary of export `run` holds a function value",
        ),
        (
            "hidden-allocation-list",
            manifest("lists", &lists, lists_entry, Profile::Core),
            "hidden allocation: a list requires heap allocation, which rust-core does not provide",
        ),
        (
            "hidden-allocation-recursive-type",
            manifest("adt-evaluation", &adts, adts_entry, Profile::Core),
            "hidden allocation: ADT 0 contains itself and requires heap allocation, which rust-core does not provide",
        ),
        (
            "arithmetic-undeclared-overflow",
            {
                let mut out = base();
                out.exports[0].errors = Errors::None;
                out
            },
            "arithmetic mismatch: export `run` declares no errors, but function",
        ),
        (
            "arithmetic-invented-overflow",
            {
                let mut out = manifest("booleans", &booleans, booleans_entry, Profile::Std);
                out.exports[0].errors = Errors::Overflow;
                out
            },
            "arithmetic mismatch: export `run` declares overflow, but function",
        ),
        (
            "export-arity",
            {
                let mut out = base();
                out.exports[0].parameters.push(Passing::Own);
                out
            },
            "export `run` passes 2 parameters; function",
        ),
        (
            "version-noncanonical",
            {
                let mut out = base();
                out.version = "1.01.0".to_owned();
                out
            },
            "`1.01.0` is not a version of three canonical decimals",
        ),
        (
            "version-overflow",
            {
                let mut out = base();
                out.version = "99999999999999999999.0.0".to_owned();
                out
            },
            "`99999999999999999999.0.0` is not a version of three canonical decimals each at most 18446744073709551615",
        ),
        (
            "unsupported-function-in-record",
            {
                // The export takes a record whose only field is a function
                // value, so a function crosses the boundary inside a named
                // type.
                let fn_type = Ty::Fn {
                    parameters: vec![Ty::Nat],
                    result: Box::new(Ty::Nat),
                };
                let record = Program {
                    spec: PROGRAM_SPEC.to_owned(),
                    adts: vec![lexlean::calculus::Adt {
                        constructors: vec![vec![fn_type]],
                    }],
                    functions: vec![Function {
                        parameters: vec![0],
                        types: vec![Ty::Adt { index: 0 }],
                        result: Ty::Nat,
                        body: Expr::Value {
                            ty: Ty::Nat,
                            value: Value::Nat {
                                value: "1".to_owned(),
                            },
                        },
                    }],
                };
                let mut out = base();
                out.program = record;
                out.exports[0].errors = Errors::None;
                out
            },
            "unsupported type: the boundary of export `run` holds a function value, and a package boundary is first-order",
        ),
        (
            "unsupported-uninhabited-value",
            {
                // Function 1 returns a function value only by calling itself,
                // so no value of its type exists and function 0's call of it
                // would be unreachable code.
                let fn_type = Ty::Fn {
                    parameters: vec![Ty::Nat],
                    result: Box::new(Ty::Nat),
                };
                let unending = Program {
                    spec: PROGRAM_SPEC.to_owned(),
                    adts: Vec::new(),
                    functions: vec![
                        Function {
                            parameters: Vec::new(),
                            types: Vec::new(),
                            result: Ty::Nat,
                            body: Expr::Apply {
                                target: Box::new(Expr::Call {
                                    function: 1,
                                    operands: Vec::new(),
                                }),
                                operands: vec![Expr::Value {
                                    ty: Ty::Nat,
                                    value: Value::Nat {
                                        value: "3".to_owned(),
                                    },
                                }],
                            },
                        },
                        Function {
                            parameters: Vec::new(),
                            types: Vec::new(),
                            result: fn_type,
                            body: Expr::Call {
                                function: 1,
                                operands: Vec::new(),
                            },
                        },
                    ],
                };
                let mut out = base();
                out.program = unending;
                out.exports[0].parameters = Vec::new();
                out.exports[0].errors = Errors::None;
                out
            },
            "unsupported type: the program computes a value of",
        ),
        (
            "sources-unordered",
            {
                let mut out = base();
                out.sources = vec!["b".repeat(64), "a".repeat(64)];
                out
            },
            "sources are strictly ascending",
        ),
    ];
    let mut out = BTreeMap::new();
    for (name, manifest, error) in cases.drain(..) {
        out.insert(
            name.to_owned(),
            (
                manifest.to_file_bytes().expect("canonical manifest"),
                error.to_owned(),
            ),
        );
    }
    // A member the closed form does not have.
    let mut extra: serde_json::Value =
        serde_json::from_slice(&base().to_file_bytes().expect("canonical manifest"))
            .expect("manifest JSON");
    extra["unchecked"] = serde_json::json!(true);
    out.insert(
        "malformed-unknown-member".to_owned(),
        (
            serde_json::to_vec(&extra).expect("JSON"),
            "package manifest is malformed: unknown field `unchecked`".to_owned(),
        ),
    );
    out
}

/// Every generated file of the packages and negative manifests, by path.
///
/// # Panics
///
/// Panics if a committed fixture does not package.
#[must_use]
pub fn files() -> BTreeMap<String, Vec<u8>> {
    let mut out = BTreeMap::new();
    for committed in packages() {
        let directory = format!(
            "compiler/rust/{}/{}",
            committed.profile.target(),
            committed.directory
        );
        let package = rust::package::package(&committed.manifest)
            .unwrap_or_else(|reason| panic!("{directory}: {reason}"));
        out.insert(
            format!("{directory}/package.json"),
            committed
                .manifest
                .to_file_bytes()
                .expect("canonical manifest"),
        );
        for (path, bytes) in package.files {
            out.insert(format!("{directory}/{path}"), bytes);
        }
    }
    for (name, (bytes, error)) in negatives() {
        out.insert(format!("compiler/rust/negative/{name}.json"), bytes);
        out.insert(
            format!("compiler/rust/negative/{name}.error"),
            format!("{error}\n").into_bytes(),
        );
    }
    out
}
