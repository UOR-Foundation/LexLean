//! The `tooling` suite: TL-01..TL-11, compiler-tooling modules (SPEC.md
//! §17.20).
//!
//! Every plant starts from a committed fixture that checks, changes exactly
//! one member of its data, and must be refused with the code and for the
//! reason the plant names, so a refusal is the plant's own and not an
//! unrelated defect's. The fixtures are checked as the compiler embeds them
//! (`Project::with_tooling`): the public API and the command line refuse a
//! tooling module outright (TL-02), so they cannot exercise anything else.

use serde_json::{json, Value as Json};

use lexlean::Selection;

use crate::support::{self, P};

/// The committed fixtures: the core term language and recursion, command
/// registration with invocation and evaluation, and a traversal of the real
/// Lean compiler's `Code` family.
const FIXTURES: [&str; 3] = ["commands", "core", "traversal"];

const MAIN: &str = "src/Main.lex.tex";
const OPEN: &str = "\\toolingdata{";
const CLOSE: &str = "}\n\\end{toolingmodule}";

// ---------------------------------------------------------------------------
// Fixture access.

fn bounds(text: &str) -> (usize, usize) {
    let start = text.find(OPEN).expect("tooling data") + OPEN.len();
    let end = text.rfind(CLOSE).expect("end of tooling data");
    (start, end)
}

/// The module data of a project copy.
fn data(project: &P) -> Json {
    let text = project.read(MAIN);
    let (start, end) = bounds(&text);
    serde_json::from_str(&text[start..end]).expect("module data")
}

/// The module data exactly as written, for a plant that is about its bytes.
fn data_text(project: &P) -> String {
    let text = project.read(MAIN);
    let (start, end) = bounds(&text);
    text[start..end].to_owned()
}

fn put_text(project: &P, raw: &str) {
    let text = project.read(MAIN);
    let (start, end) = bounds(&text);
    project.write(MAIN, &format!("{}{}{}", &text[..start], raw, &text[end..]));
}

/// Write module data canonically: `serde_json` keeps object members sorted
/// and writes no space, which is the canonical form (§21.7).
fn put(project: &P, value: &Json) {
    put_text(project, &serde_json::to_string(value).expect("serializes"));
}

fn declarations(value: &mut Json) -> &mut Vec<Json> {
    value["declarations"].as_array_mut().expect("declarations")
}

/// The first declaration of a name, whatever its kind.
fn named<'a>(value: &'a mut Json, name: &str) -> &'a mut Json {
    declarations(value)
        .iter_mut()
        .find(|declaration| declaration["name"] == name)
        .unwrap_or_else(|| panic!("`{name}` is declared"))
}

fn remove(value: &mut Json, name: &str) {
    let before = declarations(value).len();
    declarations(value).retain(|declaration| declaration["name"] != name);
    assert_eq!(
        declarations(value).len() + 1,
        before,
        "`{name}` is declared"
    );
}

/// A constructor of a registered type.
fn constructor<'a>(value: &'a mut Json, ty: &str, name: &str) -> &'a mut Json {
    named(value, ty)["constructors"]
        .as_array_mut()
        .expect("constructors")
        .iter_mut()
        .find(|candidate| candidate["name"] == name)
        .unwrap_or_else(|| panic!("`{ty}.{name}` is declared"))
}

/// The body of a function.
fn body<'a>(value: &'a mut Json, function: &str) -> &'a mut Json {
    &mut named(value, function)["body"]
}

/// The branch of a `match` body whose constructor is `full`.
fn branch<'a>(matched: &'a mut Json, full: &str) -> &'a mut Json {
    matched["branches"]
        .as_array_mut()
        .expect("branches")
        .iter_mut()
        .find(|candidate| candidate["constructor"] == full)
        .unwrap_or_else(|| panic!("a branch for `{full}`"))
}

// ---------------------------------------------------------------------------
// Checking as the compiler embeds a project.

fn embedded(project: &P) -> lexlean::project::Project {
    lexlean::project::Project::load(&project.root.join("lexlean.toml"))
        .expect("the project loads")
        .with_tooling()
}

/// The linked project, through the internal pipeline with tooling enabled and
/// the lock computed in memory.
fn linked(project: &P) -> Result<lexlean::link::CheckedProject, lexlean::LexLeanError> {
    let inner = embedded(project);
    let (lock, packages) = lexlean::lock::compute_lock(&inner, false).expect("the lock computes");
    lexlean::link::check_project(&inner, &Selection::Entrypoints, &lock, packages)
}

fn accepted(label: &str, project: &P) -> lexlean::link::CheckedProject {
    linked(project).unwrap_or_else(|error| panic!("{label}: {error:?}"))
}

fn messages(error: &lexlean::LexLeanError) -> Vec<(String, String)> {
    error
        .diagnostics
        .iter()
        .map(|diagnostic| {
            (
                diagnostic.code.as_str().to_owned(),
                diagnostic.message.clone(),
            )
        })
        .collect()
}

/// The project is refused with `code`, and the message names `needle`.
fn refusal(label: &str, project: &P, code: &str, needle: &str) -> Result<(), String> {
    let Err(error) = linked(project) else {
        return Err(format!("{label}: the module was accepted"));
    };
    let found = messages(&error);
    if found
        .iter()
        .any(|(found_code, message)| found_code == code && message.contains(needle))
    {
        Ok(())
    } else {
        Err(format!(
            "{label}: expected {code} naming {needle:?}, found {found:?}"
        ))
    }
}

/// Every plant of a case is run, and the case fails with all that were not
/// refused as named, so that one run shows them all.
#[derive(Default)]
struct Plants {
    /// The plants not refused as named.
    failures: Vec<String>,
    /// The plants run.
    run: usize,
}

impl Plants {
    /// A copy of `fixture` with one edit of its data, refused with `code`.
    fn plant(
        &mut self,
        fixture: &str,
        label: &str,
        code: &str,
        needle: &str,
        edit: impl FnOnce(&mut Json),
    ) {
        let project = P::tooling(fixture);
        let mut value = data(&project);
        edit(&mut value);
        put(&project, &value);
        self.refused(label, &project, code, needle);
    }

    /// A project refused as named, whatever made it so.
    fn refused(&mut self, label: &str, project: &P, code: &str, needle: &str) {
        self.run += 1;
        if let Err(failure) = refusal(label, project, code, needle) {
            self.failures.push(failure);
        }
    }

    fn finish(self, case: &str) {
        assert!(
            self.failures.is_empty(),
            "{case}: {} plants were not refused as named:\n{}",
            self.failures.len(),
            self.failures.join("\n")
        );
        eprintln!(
            "{case}: {} planted defects, each refused as named",
            self.run
        );
    }
}

/// What the committed schema says of a value.
fn schema_violations(value: &Json) -> Vec<crate::schema::Violation> {
    crate::schema::validate(&support::tooling_schema(), value)
}

/// Every fixture is accepted before anything is planted in it.
fn baselines() {
    for fixture in FIXTURES {
        let project = P::tooling(fixture);
        let checked = accepted(fixture, &project);
        assert_eq!(checked.modules.len(), 1, "{fixture}: one module");
    }
}

// ---------------------------------------------------------------------------
// TL-01: the closed schema.

fn tl_01() {
    let mut plants = Plants::default();
    baselines();
    for fixture in FIXTURES {
        let project = P::tooling(fixture);
        let value = data(&project);
        let violations = schema_violations(&value);
        assert!(
            violations.is_empty(),
            "{fixture} violates the committed schema: {violations:?}"
        );
        // The committed bytes are canonical, so the decoder's comparison
        // accepts them and a re-encoding changes nothing.
        assert_eq!(
            serde_json::to_string(&value).expect("serializes"),
            data_text(&project),
            "{fixture}: the data is canonical"
        );
        // The compiler's own enumeration of the schema must not be looser
        // than the file: every declaration kind of the fixtures is in it.
        let defined = support::tooling_schema()["$defs"]["declaration"]["oneOf"]
            .as_array()
            .expect("alternatives")
            .iter()
            .filter_map(|alternative| alternative["properties"]["kind"]["const"].as_str())
            .map(str::to_owned)
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(
            defined,
            ["call", "command", "evaluate", "function", "invoke", "plumbing", "type"]
                .iter()
                .map(|kind| (*kind).to_owned())
                .collect(),
            "the schema's declaration kinds are exactly the closed set"
        );
    }

    // A defect the schema states: the schema and the compiler both refuse it,
    // and the compiler refuses it with LLT4013 alone.
    type Edit = fn(&mut Json);
    let schema_plants: [(&str, &str, Edit); 11] = [
        ("an unknown member of the module", "core", |v| {
            v["extra"] = json!(1);
        }),
        ("a Lean text member of a function", "core", |v| {
            named(v, "Core.sum")["lean"] = json!("def sum := 0");
        }),
        ("a template member of a command", "commands", |v| {
            named(v, "Commands.require")["template"] = json!("syntax \"x\" : command");
        }),
        ("an unknown member of a term", "core", |v| {
            body(v, "Core.describe")["comment"] = json!("note");
        }),
        ("an unknown member of a signature", "core", |v| {
            named(v, "Nat.add")["signature"]["note"] = json!(true);
        }),
        ("an unknown member of a command atom", "commands", |v| {
            named(v, "Commands.require")["atoms"][0]["pattern"] = json!(".*");
        }),
        ("a declaration kind outside the closed set", "core", |v| {
            declarations(v).push(json!({"kind": "template", "name": "Core.template"}));
        }),
        ("a term kind outside the closed set", "core", |v| {
            *body(v, "Core.describe") = json!({"kind": "do", "steps": []});
        }),
        ("an opaque flag written with its default", "core", |v| {
            named(v, "Nat")["opaque"] = json!(false);
        }),
        ("a traversal flag written with its default", "core", |v| {
            named(v, "Core.sum")["traversal"] = json!(false);
        }),
        ("another spec tag", "core", |v| {
            v["spec"] = json!("lexlean/tooling-module/2");
        }),
    ];
    for (label, fixture, edit) in schema_plants {
        let project = P::tooling(fixture);
        let mut value = data(&project);
        edit(&mut value);
        assert!(
            !schema_violations(&value).is_empty(),
            "{label}: the committed schema must refuse it"
        );
        put(&project, &value);
        plants.refused(label, &project, "LLT4013", "");
        let error = linked(&project).err().expect("refused");
        assert!(
            messages(&error).iter().all(|(code, _)| code == "LLT4013"),
            "{label}: the refusal is LLT4013 alone, found {:?}",
            messages(&error)
        );
    }

    // A defect only the decoder can see: the bytes. Each is schema-valid.
    let project = P::tooling("core");
    let mut value = data(&project);
    value["imports"] = json!(["Zeta", "Alpha"]);
    put(&project, &value);
    plants.refused("an unsorted import list", &project, "LLT4013", "sorted");

    // Whitespace outside strings is the grammar's and not part of the value,
    // so a re-indented module is the same module (§17.10 states the same of
    // the other module data); an encoding that differs in anything else is
    // not canonical.
    let project = P::tooling("core");
    let raw = data_text(&project);
    let tail = ",\"spec\":\"lexlean/tooling-module/1\"}";
    assert!(
        raw.ends_with(tail),
        "the canonical value ends with its spec"
    );
    let reordered = format!(
        "{{\"spec\":\"lexlean/tooling-module/1\",{}}}",
        &raw[1..raw.len() - tail.len()]
    );
    put_text(&project, &reordered);
    assert!(
        schema_violations(&data(&project)).is_empty(),
        "a reordered value is schema-valid"
    );
    plants.refused("members out of order", &project, "LLT4013", "canonical");

    let project = P::tooling("core");
    let raw = data_text(&project);
    assert!(raw.contains("\"α\""), "the fixture names a type variable α");
    put_text(&project, &raw.replacen("\"α\"", "\"\\u03b1\"", 1));
    plants.refused("an escaped scalar", &project, "LLT4013", "canonical");

    let project = P::tooling("core");
    let pretty = serde_json::to_string_pretty(&data(&project)).expect("serializes");
    put_text(&project, &pretty);
    accepted("a re-indented module", &project);

    // Only the byte level can see a duplicate member.
    let project = P::tooling("core");
    let raw = data_text(&project);
    put_text(
        &project,
        &raw.replacen("{\"authority\"", "{\"spec\":\"x\",\"authority\"", 1),
    );
    plants.refused("a repeated member", &project, "LLT4013", "");

    // Nesting past the decoder's own limit is a refusal, not a stack overflow.
    let project = P::tooling("core");
    let mut value = data(&project);
    let mut nested = json!({"kind": "nat", "value": "0"});
    for _ in 0..200 {
        nested = json!({
            "kind": "let", "binder": "x",
            "type": {"kind": "const", "name": "Nat", "arguments": []},
            "value": {"kind": "nat", "value": "0"}, "body": nested
        });
    }
    *body(&mut value, "Core.describe") = nested;
    put(&project, &value);
    plants.refused(
        "a term nested 200 levels",
        &project,
        "LLT4013",
        "recursion limit",
    );

    // The charge to `max_ir_nodes` is made like every other module's.
    let project = P::tooling("core");
    let config = project.read("lexlean.toml");
    project.write(
        "lexlean.toml",
        &config.replace("max_ir_nodes = 2000000", "max_ir_nodes = 100"),
    );
    let error = linked(&project).err().expect("over the node limit");
    support::expect_code(&error, "LLS8002");

    // The structure of a module is the grammar's: one data argument, one form.
    for (label, edit) in [
        (
            "a second data argument",
            (|text: &str| {
                text.replace(
                    "\\end{toolingmodule}",
                    "\\toolingdata{{}}\n\\end{toolingmodule}",
                )
            }) as fn(&str) -> String,
        ),
        ("a second module form", |text: &str| {
            text.replace(
                "\\end{toolingmodule}\n",
                "\\end{toolingmodule}\n\\begin{coremodule}\n\\coredata{{}}\n\\end{coremodule}\n",
            )
        }),
    ] {
        let project = P::tooling("core");
        project.write(MAIN, &edit(&project.read(MAIN)));
        plants.refused(label, &project, "LLP2003", "expected `\\end`");
    }
    plants.finish("TL-01");

    // No change of any single node of a fixture panics the compiler (R5).
    let started = std::time::Instant::now();
    let (accepted_count, refused_count) = tl_panics();
    eprintln!(
        "TL-01: {} single-node changes of the fixtures, {accepted_count} accepted and {refused_count} refused by a registered code, none panicked, in {:.1} s",
        accepted_count + refused_count,
        started.elapsed().as_secs_f64()
    );
    assert!(
        accepted_count > 0 && refused_count > 1000,
        "the sweep accepted {accepted_count} and refused {refused_count} changes"
    );
}

// ---------------------------------------------------------------------------
// R5: no value of a module panics the compiler.

/// Every JSON pointer of a value, root first.
fn pointers(value: &Json, at: &str, out: &mut Vec<String>) {
    out.push(at.to_owned());
    match value {
        Json::Array(items) => {
            for (index, item) in items.iter().enumerate() {
                pointers(item, &format!("{at}/{index}"), out);
            }
        }
        Json::Object(members) => {
            for (key, item) in members {
                let key = key.replace('~', "~0").replace('/', "~1");
                pointers(item, &format!("{at}/{key}"), out);
            }
        }
        _ => {}
    }
}

/// One change to the node at `pointer`: 0 removes it, 1 to 5 replace it by
/// `null`, `0`, `""`, `{}`, `[]`, and 6 repeats it (an array element only).
/// Whether the change applies to that node.
fn alter(value: &mut Json, pointer: &str, kind: usize) -> bool {
    let Some((parent, last)) = pointer.rsplit_once('/') else {
        return false;
    };
    let Some(holder) = value.pointer_mut(parent) else {
        return false;
    };
    let replacement = match kind {
        1 => Some(Json::Null),
        2 => Some(json!(0)),
        3 => Some(json!("")),
        4 => Some(json!({})),
        5 => Some(json!([])),
        _ => None,
    };
    match holder {
        Json::Array(items) => {
            let index: usize = last.parse().expect("an array index");
            match (kind, replacement) {
                (0, _) => {
                    items.remove(index);
                }
                (6, _) => {
                    let copy = items[index].clone();
                    items.insert(index, copy);
                }
                (_, Some(new)) => items[index] = new,
                _ => return false,
            }
            true
        }
        Json::Object(members) => {
            let key = last.replace("~1", "/").replace("~0", "~");
            match (kind, replacement) {
                (0, _) => {
                    members.remove(&key);
                }
                (_, Some(new)) => {
                    members.insert(key, new);
                }
                _ => return false,
            }
            true
        }
        _ => false,
    }
}

/// Every single-node change of every fixture is accepted or refused with one
/// of the registered codes, and none of them panics (R5: user input never
/// panics). The count of each outcome arms the check: a sweep that refused
/// nothing, or accepted nothing, would say nothing.
fn tl_panics() -> (usize, usize) {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let pin = lexlean::ir::tooling::AuthorityPin {
        lean_version: lexlean::LEAN_VERSION,
        lean_githash: lexlean::LEAN_GITHASH,
    };
    let failures = std::sync::Mutex::new(Vec::<String>::new());
    let (accepted_count, refused_count) = (AtomicUsize::new(0), AtomicUsize::new(0));
    for fixture in FIXTURES {
        let base = data(&P::tooling(fixture));
        let mut nodes = Vec::new();
        pointers(&base, "", &mut nodes);
        support::for_each_parallel(&nodes, 4, |pointer| {
            for kind in 0..=6 {
                let mut value = base.clone();
                if !alter(&mut value, pointer, kind) {
                    continue;
                }
                let text = serde_json::to_string(&value).expect("serializes");
                let outcome = std::panic::catch_unwind(|| {
                    lexlean::ir::tooling::ToolingModule::parse(&text, lexlean::LANGUAGE_1_2, pin)
                });
                match outcome {
                    Err(_) => failures.lock().expect("failures").push(format!(
                        "{fixture} {pointer} change {kind}: the compiler panicked"
                    )),
                    Ok(Ok(_)) => {
                        accepted_count.fetch_add(1, Ordering::Relaxed);
                    }
                    Ok(Err(failure)) => {
                        refused_count.fetch_add(1, Ordering::Relaxed);
                        if !["LLT4013", "LLT4014", "LLT4015"].contains(&failure.code.as_str()) {
                            failures.lock().expect("failures").push(format!(
                                "{fixture} {pointer} change {kind}: refused with {}",
                                failure.code.as_str()
                            ));
                        }
                    }
                }
            }
        });
    }
    let failures = failures.into_inner().expect("failures");
    assert!(
        failures.is_empty(),
        "{} of the changes panicked or were refused with another code, the first ones:\n{}",
        failures.len(),
        failures
            .iter()
            .take(10)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
    (
        accepted_count.load(Ordering::Relaxed),
        refused_count.load(Ordering::Relaxed),
    )
}

// ---------------------------------------------------------------------------
// TL-02: the context.

fn tl_02() {
    let mut plants = Plants::default();
    // A user project: the library API refuses, whatever the request.
    let project = P::tooling("core");
    project.relock();
    let error = project.check_fails_with("LLT4013");
    assert!(
        messages(&error)
            .iter()
            .any(|(_, message)| message.contains("not in a user project")),
        "the refusal names the context: {:?}",
        messages(&error)
    );
    project.assert_no_backend_output_with("LLT4013");
    project.verify_fails_with("LLT4013");
    let error = project
        .engine()
        .build(lexlean::BuildRequest {
            selection: Selection::Entrypoints,
        })
        .err()
        .expect("build is refused");
    support::expect_code(&error, "LLT4013");

    // The command line is the same request.
    for command in ["check", "build", "verify"] {
        let (exit, stdout, stderr) = project.cli(&["--diagnostic-format", "json", command]);
        assert_ne!(exit, 0, "`lexlean {command}` refuses a tooling module");
        assert!(
            stdout.contains("LLT4013") || stderr.contains("LLT4013"),
            "`lexlean {command}` names LLT4013: {stdout} {stderr}"
        );
    }

    // No option admits one: an unknown option is a usage error, not a switch.
    let (exit, _, _) = project.cli(&["check", "--tooling"]);
    assert_ne!(exit, 0, "there is no option that admits a tooling module");
    assert!(
        !lexlean::project::Project::load(&project.root.join("lexlean.toml"))
            .expect("loads")
            .tooling,
        "a loaded project is never a tooling project"
    );

    // No configuration member admits one: the configuration is closed.
    let admitting = P::tooling("core");
    let config = admitting.read("lexlean.toml");
    admitting.write(
        "lexlean.toml",
        &format!("{config}\n[tooling]\nadmit = true\n"),
    );
    assert!(
        lexlean::project::Project::load(&admitting.root.join("lexlean.toml")).is_err(),
        "a `tooling` configuration member is refused"
    );
    let admitting = P::tooling("core");
    let config = admitting.read("lexlean.toml");
    admitting.write(
        "lexlean.toml",
        &config.replacen(
            "language = \"1.2\"\n",
            "language = \"1.2\"\ntooling = true\n",
            1,
        ),
    );
    assert!(
        lexlean::project::Project::load(&admitting.root.join("lexlean.toml")).is_err(),
        "a `tooling` member of the project is refused"
    );

    // Language 1.1 refuses it even in an embedded project.
    let eleven = P::tooling("core");
    let config = eleven.read("lexlean.toml");
    eleven.write(
        "lexlean.toml",
        &config.replace("language = \"1.2\"", "language = \"1.1\""),
    );
    eleven.edit(MAIN, "lexlean.std.nat@1.2.0", "lexlean.std.nat@1.1.0");
    plants.refused("language 1.1", &eleven, "LLT4013", "requires language 1.2");

    // In an embedded project a tooling module has no production root, no
    // eligibility report, and nothing of a semantic or core module.
    for fixture in FIXTURES {
        let embedded_project = P::tooling(fixture);
        let checked = accepted(fixture, &embedded_project);
        for module in checked.modules.values() {
            assert!(
                module.production.is_none(),
                "{fixture}: no eligibility report"
            );
            assert!(
                module.document.semantic.is_none(),
                "{fixture}: no semantic module"
            );
            assert!(module.document.core.is_none(), "{fixture}: no core module");
            assert!(
                module.document.tooling.is_some(),
                "{fixture}: the tooling module"
            );
        }
    }
    plants.finish("TL-02");
}

// ---------------------------------------------------------------------------
// TL-03: the authority and the registry.

fn pins() -> String {
    let root = support::repo_root();
    std::fs::read_to_string(root.join("model/authorities.toml").as_std_path())
        .expect("model/authorities.toml")
}

fn tl_03() {
    let mut plants = Plants::default();
    baselines();
    let authorities = pins();
    assert!(
        authorities.contains(lexlean::LEAN_GITHASH),
        "model/authorities.toml pins the compiler's Lean commit"
    );
    let adapter = lexlean::production::lcnf::adapter().expect("the embedded adapter");
    assert_eq!(adapter.authority.lean_githash, lexlean::LEAN_GITHASH);
    assert_eq!(adapter.authority.lean_version, lexlean::LEAN_VERSION);
    for fixture in FIXTURES {
        let value = data(&P::tooling(fixture));
        assert_eq!(value["authority"]["lean_githash"], lexlean::LEAN_GITHASH);
        assert_eq!(value["authority"]["lean_version"], lexlean::LEAN_VERSION);
    }

    plants.plant(
        "core",
        "an authority at another commit",
        "LLT4014",
        "pins Lean",
        |v| {
            v["authority"]["lean_githash"] = json!("0".repeat(40));
        },
    );
    plants.plant(
        "core",
        "an authority at another release",
        "LLT4014",
        "pins Lean",
        |v| {
            v["authority"]["lean_version"] = json!("4.0.0");
        },
    );
    plants.plant(
        "core",
        "a name declared twice",
        "LLT4014",
        "declared twice",
        |v| {
            let copy = named(v, "Nat.add").clone();
            declarations(v).push(copy);
        },
    );
    plants.plant(
        "core",
        "a call declared as a type too",
        "LLT4014",
        "declared twice",
        |v| {
            let mut copy = named(v, "Bool").clone();
            copy["name"] = json!("Nat.add");
            declarations(v).push(copy);
        },
    );
    plants.plant(
        "core",
        "a signature naming an unregistered constant",
        "LLT4014",
        "not registered",
        |v| {
            named(v, "Nat.add")["signature"] = json!({"kind": "const", "name": "Nat.Unregistered"});
        },
    );
    plants.plant(
        "core",
        "a signature with an unbound variable",
        "LLT4014",
        "unbound",
        |v| {
            named(v, "Nat.add")["signature"] = json!({
                "kind": "arrow",
                "domain": {"kind": "var", "name": "ghost"},
                "codomain": {"kind": "const", "name": "Nat"}
            });
        },
    );
    plants.plant(
        "commands",
        "a signature naming a removed type",
        "LLT4014",
        "not a registered type",
        |v| {
            remove(v, "Lean.Syntax");
        },
    );
    plants.plant(
        "core",
        "an unsorted plumbing row",
        "LLT4014",
        "sorted",
        |v| {
            let row = declarations(v)
                .iter_mut()
                .find(|d| d["kind"] == "plumbing")
                .expect("a plumbing row");
            row["names"].as_array_mut().expect("names").reverse();
        },
    );
    plants.plant(
        "core",
        "a repeated plumbing name",
        "LLT4014",
        "sorted",
        |v| {
            let row = declarations(v)
                .iter_mut()
                .find(|d| d["kind"] == "plumbing")
                .expect("a plumbing row");
            let first = row["names"][0].clone();
            row["names"].as_array_mut().expect("names").insert(0, first);
        },
    );
    plants.plant(
        "core",
        "an empty plumbing row",
        "LLT4014",
        "names no constant",
        |v| {
            let row = declarations(v)
                .iter_mut()
                .find(|d| d["kind"] == "plumbing")
                .expect("a plumbing row");
            row["names"] = json!([]);
        },
    );
    plants.plant("core", "plumbing in two rows", "LLT4014", "two rows", |v| {
        let row = declarations(v)
            .iter()
            .find(|d| d["kind"] == "plumbing")
            .expect("a plumbing row")
            .clone();
        declarations(v).push(row);
    });
    plants.plant(
        "core",
        "a plumbing constant that is also declared",
        "LLT4014",
        "also declared",
        |v| {
            let row = declarations(v)
                .iter_mut()
                .find(|d| d["kind"] == "plumbing")
                .expect("a plumbing row");
            row["names"] = json!(["Nat"]);
        },
    );
    plants.plant(
        "core",
        "a source path that leaves the toolchain",
        "LLT4014",
        "source",
        |v| {
            named(v, "Nat")["source"]["file"] = json!("../Init/Prelude.lean");
        },
    );
    plants.plant(
        "core",
        "an absolute source path",
        "LLT4014",
        "source",
        |v| {
            named(v, "Nat")["source"]["file"] = json!("/etc/passwd");
        },
    );
    plants.plant("core", "an uppercase digest", "LLT4014", "digest", |v| {
        let digest = named(v, "Nat")["source"]["sha256"]
            .as_str()
            .expect("digest")
            .to_uppercase();
        named(v, "Nat")["source"]["sha256"] = json!(digest);
    });
    plants.plant("core", "a short digest", "LLT4014", "digest", |v| {
        named(v, "Nat")["source"]["sha256"] = json!("abcd");
    });

    plants.finish("TL-03");

    // The pins are the toolchain's own files: with the pinned toolchain
    // installed, every digest is the SHA-256 of that file of its source tree.
    if !support::lean_backed("TL-03") {
        return;
    }
    use sha2::{Digest, Sha256};
    let toolchain = support::real_elan_home()
        .join("toolchains")
        .join(support::mangled_toolchain_name());
    let mut checked = 0usize;
    for fixture in FIXTURES {
        let value = data(&P::tooling(fixture));
        for declaration in value["declarations"].as_array().expect("declarations") {
            let Some(pin) = declaration.get("source") else {
                continue;
            };
            let file = pin["file"].as_str().expect("file");
            let path = toolchain.join(file);
            let bytes = std::fs::read(&path)
                .unwrap_or_else(|error| panic!("{fixture}: {}: {error}", path.display()));
            let digest: String = Sha256::digest(&bytes)
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect();
            assert_eq!(
                pin["sha256"], digest,
                "{fixture}: `{}` pins a digest that is not the pinned toolchain's {file}",
                declaration["name"]
            );
            checked += 1;
        }
    }
    assert!(checked > 0, "no source pin was compared");
}

// ---------------------------------------------------------------------------
// Term and type builders for plants.

const LCNF: &str = "Lean.Compiler.LCNF.";

fn ty(name: &str, arguments: Vec<Json>) -> Json {
    json!({"kind": "const", "name": name, "arguments": arguments})
}

fn var(name: &str) -> Json {
    json!({"kind": "var", "name": name})
}

fn nat(value: &str) -> Json {
    json!({"kind": "nat", "value": value})
}

fn string(value: &str) -> Json {
    json!({"kind": "string", "value": value})
}

fn call(function: &str, type_arguments: Vec<Json>, arguments: Vec<Json>) -> Json {
    json!({"kind": "call", "function": function, "type_arguments": type_arguments, "arguments": arguments})
}

/// A value of a path into a declaration.
fn at<'a>(value: &'a mut Json, declaration: &str, pointer: &str) -> &'a mut Json {
    named(value, declaration)
        .pointer_mut(pointer)
        .unwrap_or_else(|| panic!("`{declaration}` has {pointer}"))
}

fn position(value: &mut Json, name: &str) -> usize {
    declarations(value)
        .iter()
        .position(|declaration| declaration["name"] == name)
        .unwrap_or_else(|| panic!("`{name}` is declared"))
}

// ---------------------------------------------------------------------------
// TL-04: registered types.

fn tl_04() {
    baselines();
    let mut plants = Plants::default();
    let code = format!("{LCNF}Code");
    plants.plant(
        "traversal",
        "an index of an unregistered type",
        "LLT4014",
        "is not a registered type",
        |v| {
            named(v, &code)["parameters"][0]["kind"]["of"] = json!(format!("{LCNF}Unregistered"));
        },
    );
    plants.plant(
        "traversal",
        "an index of a type whose constructors have fields",
        "LLT4014",
        "is not an index type",
        |v| {
            named(v, &code)["parameters"][0]["kind"]["of"] = json!(format!("{LCNF}FunDecl"));
        },
    );
    plants.plant(
        "core",
        "a repeated constructor",
        "LLT4014",
        "`zero` twice",
        |v| {
            let copy = constructor(v, "Nat", "zero").clone();
            named(v, "Nat")["constructors"]
                .as_array_mut()
                .expect("constructors")
                .push(copy);
        },
    );
    plants.plant(
        "core",
        "a field naming an unregistered type",
        "LLT4014",
        "`Unregistered` is not a registered type",
        |v| {
            constructor(v, "List", "cons")["fields"][0]["type"] = ty("Unregistered", vec![]);
        },
    );
    plants.plant(
        "core",
        "a type applied to too few arguments",
        "LLT4014",
        "a type constructor is not the type of a value",
        |v| {
            constructor(v, "List", "cons")["fields"][1]["type"] = ty("List", vec![]);
        },
    );
    plants.plant(
        "core",
        "a field using a type variable that is not a parameter",
        "LLT4014",
        "`β` is not in scope",
        |v| {
            constructor(v, "List", "cons")["fields"][0]["type"] =
                json!({"kind": "parameter", "name": "β", "arguments": []});
        },
    );
    plants.plant(
        "traversal",
        "a requirement of an unknown parameter",
        "LLT4014",
        "requires the unknown parameter `ghost`",
        |v| {
            constructor(v, &code, "oset")["requires"]["parameter"] = json!("ghost");
        },
    );
    plants.plant(
        "core",
        "a requirement of a parameter that is not an index",
        "LLT4014",
        "which is not an index",
        |v| {
            constructor(v, "Prod", "mk")["requires"] =
                json!({"parameter": "α", "constructor": "Bool.true"});
        },
    );
    plants.plant(
        "traversal",
        "a requirement of a constructor of another type",
        "LLT4014",
        "not a nullary constructor of `Lean.Compiler.LCNF.Purity`",
        |v| {
            constructor(v, &code, "oset")["requires"]["constructor"] =
                json!(format!("{LCNF}FunDecl.mk"));
        },
    );
    plants.plant(
        "core",
        "an opaque type with constructors",
        "LLT4014",
        "is opaque, so it pins no constructors",
        |v| {
            named(v, "String")["constructors"] = json!([{"name": "mk", "fields": []}]);
        },
    );
    plants.plant(
        "commands",
        "an abbreviation with constructors",
        "LLT4014",
        "has no parameters and no constructors",
        |v| {
            named(v, "Lean.Elab.Command.CommandElab")["constructors"] =
                json!([{"name": "mk", "fields": []}]);
        },
    );
    plants.plant(
        "commands",
        "an abbreviation of itself",
        "LLT4014",
        "abbreviates itself",
        |v| {
            named(v, "Lean.Elab.Command.CommandElab")["alias"] =
                ty("Lean.Elab.Command.CommandElab", vec![]);
        },
    );
    plants.plant(
        "commands",
        "two abbreviations of each other",
        "LLT4014",
        "does not unfold in finitely many steps",
        |v| {
            let mut other = named(v, "Lean.Elab.Command.CommandElab").clone();
            other["name"] = json!("Lean.Elab.Command.Cycle");
            other["alias"] = ty("Lean.Elab.Command.CommandElab", vec![]);
            named(v, "Lean.Elab.Command.CommandElab")["alias"] =
                ty("Lean.Elab.Command.Cycle", vec![]);
            declarations(v).push(other);
        },
    );
    plants.finish("TL-04");
}

// ---------------------------------------------------------------------------
// TL-05: the typing of a call.

fn tl_05() {
    baselines();
    let mut plants = Plants::default();
    plants.plant(
        "core",
        "a call with an extra argument",
        "LLT4015",
        "`Nat.add` takes 2 arguments, 3 are given",
        |v| {
            at(v, "Core.sum", "/body/branches/1/body/arguments")
                .as_array_mut()
                .expect("arguments")
                .push(nat("0"));
        },
    );
    plants.plant(
        "core",
        "a call with a missing argument",
        "LLT4015",
        "`Nat.add` takes 2 arguments, 1 are given",
        |v| {
            at(v, "Core.sum", "/body/branches/1/body/arguments")
                .as_array_mut()
                .expect("arguments")
                .pop();
        },
    );
    plants.plant(
        "commands",
        "a call without its type argument",
        "LLT4015",
        "`Lean.getConstInfo` takes 1 type arguments, 0 are given",
        |v| {
            at(v, "Commands.elab", "/body/body/arguments/0/arguments/0")["type_arguments"] =
                json!([]);
        },
    );
    plants.plant(
        "commands",
        "an instance stated as a type argument",
        "LLT4015",
        "`Pure.pure` takes 2 type arguments, 3 are given",
        |v| {
            at(v, "Commands.settle", "/body/arguments/0")["type_arguments"]
                .as_array_mut()
                .expect("type arguments")
                .push(ty("Unit", vec![]));
        },
    );
    plants.plant(
        "core",
        "an argument of another type",
        "LLT4015",
        "argument 1 of `Nat.ble` has type String, expected Nat",
        |v| {
            *at(v, "Core.describe", "/body/value/arguments/0") = string("ten");
        },
    );
    plants.plant(
        "core",
        "a result of another type",
        "LLT4015",
        "`Core.predecessor` is declared to return String, its body has type Nat",
        |v| {
            *at(v, "Core.predecessor", "/result") = ty("String", vec![]);
        },
    );
    plants.plant(
        "core",
        "a result of another type through a type argument",
        "LLT4015",
        "is declared to return (List String), its body has type (List Nat)",
        |v| {
            *at(v, "Core.double", "/result") = ty("List", vec![ty("String", vec![])]);
        },
    );
    plants.plant(
        "core",
        "a callee no registry row declares",
        "LLT4015",
        "neither a registered call nor a function of this module",
        |v| {
            at(v, "Core.predecessor", "/body")["function"] = json!("Nat.nonexistent");
        },
    );
    plants.plant("commands", "a default given and one more", "LLT4015", "`Lean.Name.toString` takes 1 arguments, 3 are given", |v| {
        let extra = json!({"kind": "construct", "constructor": "Bool.false", "type_arguments": [], "arguments": []});
        at(v, "Commands.showPlain", "/body/arguments")
            .as_array_mut()
            .expect("arguments")
            .push(extra);
    });
    plants.plant(
        "commands",
        "a required argument omitted before the defaults",
        "LLT4015",
        "`Lean.Meta.MetaM.run'` takes 1 arguments, 0 are given",
        |v| {
            at(v, "Commands.settle", "/body")["arguments"] = json!([]);
        },
    );
    plants.plant(
        "commands",
        "a lambda that is not the abbreviation's unfolding",
        "LLT4015",
        "`Commands.wrong` is declared to return Lean.Elab.Command.CommandElab",
        |v| {
            let wrong = json!({
                "kind": "function", "name": "Commands.wrong", "public": true, "partial": false,
                "type_parameters": [], "parameters": [],
                "result": ty("Lean.Elab.Command.CommandElab", vec![]),
                "body": {
                    "kind": "lambda",
                    "parameters": [{"name": "x", "type": ty("Nat", vec![])}],
                    "body": call("Commands.run", vec![], vec![])
                }
            });
            declarations(v).push(wrong);
        },
    );
    plants.finish("TL-05");
}

// ---------------------------------------------------------------------------
// TL-06: terms.

fn tl_06() {
    baselines();
    let mut plants = Plants::default();
    plants.plant(
        "core",
        "a match that omits a constructor",
        "LLT4015",
        "the match on `Option` is not exhaustive: it omits some",
        |v| {
            at(v, "Core.orZero", "/body/branches")
                .as_array_mut()
                .expect("branches")
                .pop();
        },
    );
    plants.plant(
        "core",
        "a branch for a constructor of another type",
        "LLT4015",
        "`Nat.succ` is not a constructor of `Option`",
        |v| {
            branch(body(v, "Core.orZero"), "Option.some")["constructor"] = json!("Nat.succ");
        },
    );
    plants.plant(
        "traversal",
        "a branch for a constructor that exists at another index",
        "LLT4015",
        "is not a constructor of `Lean.Compiler.LCNF.Code` at its index",
        |v| {
            let inc = format!("{LCNF}Code.inc");
            let fields = constructor(v, &format!("{LCNF}Code"), "inc")["fields"]
                .as_array()
                .expect("fields")
                .len();
            let binders: Vec<String> = (0..fields).map(|i| format!("w{i}")).collect();
            at(v, "Traversal.code", "/body/branches")
                .as_array_mut()
                .expect("branches")
                .push(json!({"constructor": inc, "binders": binders, "body": nat("0")}));
        },
    );
    plants.plant(
        "core",
        "a branch binding the wrong number of fields",
        "LLT4015",
        "`Option.some` has 1 fields, 0 names are bound",
        |v| {
            branch(body(v, "Core.orZero"), "Option.some")["binders"] = json!([]);
        },
    );
    plants.plant(
        "core",
        "a constructor matched twice",
        "LLT4015",
        "`Option.none` is matched twice",
        |v| {
            let copy = branch(body(v, "Core.orZero"), "Option.none").clone();
            at(v, "Core.orZero", "/body/branches")
                .as_array_mut()
                .expect("branches")
                .push(copy);
        },
    );
    plants.plant(
        "core",
        "branches of different types",
        "LLT4015",
        "a branch has type String, expected Nat",
        |v| {
            branch(body(v, "Core.orZero"), "Option.some")["body"] = string("v");
        },
    );
    plants.plant(
        "core",
        "a field of a type with several constructors",
        "LLT4015",
        "`Option` has 2 constructors, so it has no field `val`",
        |v| {
            *body(v, "Core.orZero") = json!({"kind": "field", "value": var("o"), "field": "val"});
        },
    );
    plants.plant(
        "core",
        "a condition that is not a Bool",
        "LLT4015",
        "the condition has type Nat, expected Bool",
        |v| {
            *at(v, "Core.describe", "/body/body/arguments/0/condition") = nat("1");
        },
    );
    plants.plant(
        "core",
        "an unbound variable",
        "LLT4015",
        "the variable `ys` is not in scope",
        |v| {
            *at(v, "Core.double", "/body/arguments/1") = var("ys");
        },
    );
    plants.plant(
        "core",
        "a let whose value is not its declared type",
        "LLT4015",
        "the value of `big` has type Bool, expected Nat",
        |v| {
            *at(v, "Core.describe", "/body/type") = ty("Nat", vec![]);
        },
    );
    plants.plant(
        "core",
        "a lambda parameter of the wrong type",
        "LLT4015",
        "argument 1 of `Nat.add` has type String, expected Nat",
        |v| {
            *at(v, "Core.double", "/body/arguments/0/parameters/0/type") = ty("String", vec![]);
        },
    );
    plants.plant(
        "core",
        "an application to an argument of the wrong type",
        "LLT4015",
        "argument 1 of the applied function has type String, expected Nat",
        |v| {
            *at(v, "Core.twice", "/body/arguments/0/arguments/0") = string("x");
        },
    );
    plants.plant(
        "core",
        "an application of a value that is not a function",
        "LLT4015",
        "a value of type Nat is applied, and it is not a function",
        |v| {
            *at(v, "Core.twice", "/body/function") = var("n");
        },
    );
    plants.plant(
        "core",
        "a list element of the wrong type",
        "LLT4015",
        "element 1 has type String, expected Nat",
        |v| {
            *at(v, "Core.list", "/body/elements/0") = string("a");
        },
    );
    plants.plant(
        "core",
        "an array element of the wrong type",
        "LLT4015",
        "element 1 has type String, expected Nat",
        |v| {
            *at(v, "Core.array", "/body/elements/0") = string("a");
        },
    );
    plants.plant(
        "core",
        "a constructor with its fields swapped",
        "LLT4015",
        "field `fst` of `Prod.mk` has type String, expected Nat",
        |v| {
            at(v, "Core.pair", "/body/arguments")
                .as_array_mut()
                .expect("arguments")
                .reverse();
        },
    );
    plants.plant(
        "core",
        "a constructor with the wrong number of type arguments",
        "LLT4015",
        "`Prod.mk` takes 2 type arguments, 1 are given",
        |v| {
            at(v, "Core.pair", "/body/type_arguments")
                .as_array_mut()
                .expect("type arguments")
                .pop();
        },
    );
    plants.plant(
        "core",
        "a binder that shadows another",
        "LLT4015",
        "`xs` shadows a variable in scope",
        |v| {
            branch(body(v, "Core.sum"), "List.cons")["binders"] = json!(["xs", "t"]);
        },
    );
    plants.plant(
        "core",
        "a repeated parameter",
        "LLT4015",
        "`Core.twice` repeats the parameter `f`",
        |v| {
            *at(v, "Core.twice", "/parameters/1/name") = json!("f");
        },
    );
    plants.plant(
        "core",
        "a natural number written with a leading zero",
        "LLT4015",
        "`007` is not a canonical natural number",
        |v| {
            branch(body(v, "Core.sum"), "List.nil")["body"] = nat("007");
        },
    );
    plants.finish("TL-06");
}

// ---------------------------------------------------------------------------
// TL-07: recursion.

fn tl_07() {
    baselines();
    let mut plants = Plants::default();
    plants.plant("core", "a self call of a function that is not partial", "LLT4015", "`Core.sum` is not defined before this use, and is neither this function (when partial) nor in its mutual group", |v| {
        named(v, "Core.sum")["partial"] = json!(false);
    });
    plants.plant("core", "a call of a function defined later", "LLT4015", "`Core.predecessor` is not defined before this use", |v| {
        let early = json!({
            "kind": "function", "name": "Core.early", "public": true, "partial": false,
            "type_parameters": [], "parameters": [{"name": "n", "type": ty("Nat", vec![])}],
            "result": ty("Nat", vec![]), "body": call("Core.predecessor", vec![], vec![var("n")])
        });
        let at_sum = position(v, "Core.sum");
        declarations(v).insert(at_sum, early);
    });
    plants.plant(
        "core",
        "a mutual group that is not one run",
        "LLT4015",
        "the mutual group `Parity` is not one contiguous run of functions",
        |v| {
            let between = json!({
                "kind": "function", "name": "Core.between", "public": true, "partial": false,
                "type_parameters": [], "parameters": [{"name": "n", "type": ty("Nat", vec![])}],
                "result": ty("Nat", vec![]), "body": var("n")
            });
            let at_odd = position(v, "Core.isOdd");
            declarations(v).insert(at_odd, between);
        },
    );
    plants.plant("core", "a mutual group with a member that is not partial", "LLT4015", "`Core.isOdd` is in the mutual group `Parity` and recurses without a termination argument: it is declared `partial`", |v| {
        named(v, "Core.isOdd")["partial"] = json!(false);
    });
    plants.plant(
        "core",
        "a call into a group that is not the caller's",
        "LLT4015",
        "`Core.isOdd` is not defined before this use",
        |v| {
            named(v, "Core.isOdd")["mutual"] = json!("Other");
        },
    );
    plants.plant(
        "core",
        "a function that calls the member of a group it is no longer in",
        "LLT4015",
        "`Core.isOdd` is not defined before this use",
        |v| {
            named(v, "Core.isEven")["mutual"] = json!(null);
            named(v, "Core.isEven")
                .as_object_mut()
                .expect("object")
                .remove("mutual");
        },
    );
    plants.finish("TL-07");
}

// ---------------------------------------------------------------------------
// TL-08: complete traversal.

fn tl_08() {
    baselines();
    let mut plants = Plants::default();
    let code = format!("{LCNF}Code");
    plants.plant(
        "traversal",
        "a skipped field",
        "LLT4015",
        "the traversal `Traversal.code` skips the field `k` of `Lean.Compiler.LCNF.Code.fun`",
        |v| {
            *at(v, "Traversal.code", "/body/branches/1/body/arguments/1") = nat("0");
        },
    );
    plants.plant("traversal", "a handler that is not a traversal", "LLT4015", "the traversal `Traversal.cases` does not recurse into `Lean.Compiler.LCNF.Alt` for the field `alts`", |v| {
        named(v, "Traversal.alt")
            .as_object_mut()
            .expect("object")
            .remove("traversal");
    });
    plants.plant(
        "traversal",
        "a default branch",
        "LLT4015",
        "the traversal `Traversal.alt` has a default branch",
        |v| {
            at(v, "Traversal.alt", "/body/branches")[1] = json!({"binders": [], "body": nat("0")});
        },
    );
    plants.plant("traversal", "a handler applied to the subject, not the field", "LLT4015", "does not recurse into `Lean.Compiler.LCNF.Code` for the field `k` of `Lean.Compiler.LCNF.Code.let`", |v| {
        let tree = ty(&code, vec![json!({"kind": "index", "constructor": format!("{LCNF}Purity.pure")})]);
        *at(v, "Traversal.code", "/body/branches/0/body") = json!({
            "kind": "let", "binder": "again", "type": tree, "value": var("k"),
            "body": call("Nat.add", vec![], vec![nat("1"), call("Traversal.code", vec![], vec![var("c")])])
        });
    });
    plants.plant(
        "traversal",
        "a traversal that is not partial",
        "LLT4015",
        "recurses without a termination argument: it is declared `partial`",
        |v| {
            named(v, "Traversal.code")["partial"] = json!(false);
        },
    );
    plants.plant(
        "traversal",
        "a traversal whose subject is not first",
        "LLT4015",
        "`Traversal.alt` takes 2 arguments, 1 are given",
        |v| {
            at(v, "Traversal.alt", "/parameters")
                .as_array_mut()
                .expect("parameters")
                .insert(0, json!({"name": "extra", "type": ty("Nat", vec![])}));
        },
    );
    plants.plant("traversal", "a body that is not a match on the subject", "LLT4015", "the body of the traversal `Traversal.funDecl` is a match on its subject", |v| {
        let inner = body(v, "Traversal.funDecl").clone();
        *body(v, "Traversal.funDecl") = json!({
            "kind": "let", "binder": "result", "type": ty("Nat", vec![]), "value": inner, "body": var("result")
        });
    });
    plants.plant(
        "traversal",
        "a registry that gains a constructor",
        "LLT4015",
        "the match on `Lean.Compiler.LCNF.Code` is not exhaustive: it omits extra",
        |v| {
            named(v, &code)["constructors"]
                .as_array_mut()
                .expect("constructors")
                .push(json!({"name": "extra", "fields": []}));
        },
    );
    plants.plant("traversal", "a registry that gains a field", "LLT4015", "`Lean.Compiler.LCNF.Code.return` has 2 fields, 1 names are bound", |v| {
        constructor(v, &code, "return")["fields"]
            .as_array_mut()
            .expect("fields")
            .push(json!({"name": "extra", "type": ty(&code, vec![json!({"kind": "index", "constructor": format!("{LCNF}Purity.pure")})])}));
    });
    plants.finish("TL-08");
    if support::lean_backed("TL-08") {
        assert_elaborates(
            "the traversal of the real Code family",
            &P::tooling("traversal"),
        );
    }
}

// ---------------------------------------------------------------------------
// TL-09: commands, invocations, evaluations.

fn tl_09() {
    baselines();
    let mut plants = Plants::default();
    plants.plant(
        "commands",
        "an elaborator with a parameter",
        "LLT4015",
        "has no parameters and the type `Lean.Elab.Command.CommandElab`",
        |v| {
            named(v, "Commands.elab")["parameters"] =
                json!([{"name": "x", "type": ty("Nat", vec![])}]);
        },
    );
    plants.plant("commands", "an elaborator of the unfolded type", "LLT4015", "the elaborator `Commands.unfolded` of `Commands.require` has no parameters and the type `Lean.Elab.Command.CommandElab`", |v| {
        let mut unfolded = named(v, "Commands.elab").clone();
        unfolded["name"] = json!("Commands.unfolded");
        unfolded["result"] = json!({
            "kind": "function",
            "parameters": [ty("Lean.Syntax", vec![])],
            "result": ty("Lean.Elab.Command.CommandElabM", vec![ty("Unit", vec![])])
        });
        let at_command = position(v, "Commands.require");
        declarations(v).insert(at_command, unfolded);
        named(v, "Commands.require")["elaborator"] = json!("Commands.unfolded");
    });
    plants.plant(
        "commands",
        "a grammar without a leading token",
        "LLT4015",
        "begins with a token that names it",
        |v| {
            named(v, "Commands.require")["atoms"][0] = json!({"kind": "ident"});
        },
    );
    plants.plant(
        "commands",
        "a token that is not plain text",
        "LLT4015",
        "has a token that is not plain text",
        |v| {
            named(v, "Commands.require")["atoms"][0] =
                json!({"kind": "token", "text": "lexlean_require\" "});
        },
    );
    plants.plant(
        "commands",
        "an elaborator that is not public",
        "LLT4015",
        "is a public function",
        |v| {
            named(v, "Commands.elab")["public"] = json!(false);
        },
    );
    plants.plant(
        "commands",
        "an elaborator that is not a function of the module",
        "LLT4015",
        "`Commands.nowhere` of `Commands.require` is not a function of this module",
        |v| {
            named(v, "Commands.require")["elaborator"] = json!("Commands.nowhere");
        },
    );
    plants.plant(
        "commands",
        "an invocation of an unregistered command",
        "LLT4015",
        "`Commands.nope` is not a command registered before this invocation",
        |v| {
            let invoke = declarations(v)
                .iter_mut()
                .find(|d| d["kind"] == "invoke")
                .expect("an invocation");
            invoke["command"] = json!("Commands.nope");
        },
    );
    plants.plant(
        "commands",
        "an invocation with a missing argument",
        "LLT4015",
        "`Commands.require` takes 2 arguments, 1 are given",
        |v| {
            let invoke = declarations(v)
                .iter_mut()
                .find(|d| d["kind"] == "invoke")
                .expect("an invocation");
            invoke["arguments"].as_array_mut().expect("arguments").pop();
        },
    );
    plants.plant(
        "commands",
        "an invocation with an argument of the wrong kind",
        "LLT4015",
        "does not match the command's grammar",
        |v| {
            let invoke = declarations(v)
                .iter_mut()
                .find(|d| d["kind"] == "invoke")
                .expect("an invocation");
            invoke["arguments"][0] =
                json!({"kind": "type", "value": {"kind": "const", "name": "Nat"}});
        },
    );
    plants.plant(
        "commands",
        "an evaluation of an unregistered function",
        "LLT4015",
        "`Commands.nope` is not a function of this module",
        |v| {
            let evaluate = declarations(v)
                .iter_mut()
                .find(|d| d["kind"] == "evaluate")
                .expect("an evaluation");
            evaluate["function"] = json!("Commands.nope");
        },
    );
    plants.plant(
        "commands",
        "an evaluation with an argument the function does not take",
        "LLT4015",
        "`Commands.run` takes 0 arguments, 1 are given",
        |v| {
            let evaluate = declarations(v)
                .iter_mut()
                .find(|d| d["kind"] == "evaluate")
                .expect("an evaluation");
            evaluate["arguments"] = json!([nat("1")]);
        },
    );
    plants.plant(
        "commands",
        "an evaluation of a function that is not a command action",
        "LLT4015",
        "`Commands.show` is evaluated and returns String, not a command action",
        |v| {
            let evaluate = declarations(v)
                .iter_mut()
                .find(|d| d["kind"] == "evaluate")
                .expect("an evaluation");
            evaluate["function"] = json!("Commands.show");
            evaluate["arguments"] = json!([{"kind": "name", "value": "Nat.add"}]);
        },
    );
    plants.finish("TL-09");
    if !support::lean_backed("TL-09") {
        return;
    }
    // The pinned Lean runs the committed elaborator: it accepts the constant
    // the committed invocation names, and refuses one that does not exist,
    // which the compiler cannot know (an identifier is only a name).
    assert_elaborates("the committed command", &P::tooling("commands"));
    let missing = forged(
        "commands",
        "an invocation of a constant that does not exist",
        |v| {
            let invoke = declarations(v)
                .iter_mut()
                .find(|d| d["kind"] == "invoke")
                .expect("an invocation");
            invoke["arguments"][0]["value"] = json!("Nat.missing");
        },
    );
    assert_lean_refuses("a missing constant", &missing, "Nat.missing");
}

// ---------------------------------------------------------------------------
// The generated text, and the pinned Lean.

/// The rendered build of a project copy, as the compiler embeds it.
fn rendered(project: &P) -> lexlean::api::RenderedBuild {
    crate::tooling::render(&project.root.join("lexlean.toml"))
        .unwrap_or_else(|failure| panic!("the project renders: {failure}"))
}

/// What the pinned Lean says of a generated module: its exit code and every
/// message, from a real process.
fn elaborate(project: &P, text: &str) -> (i32, String) {
    let _guard = support::env_shared();
    let inner = embedded(project);
    let toolchain =
        lexlean::verify::toolchain::preflight(&inner.config.limits).expect("the pinned toolchain");
    let scratch = project.root.join(".lexlean/tooling-scratch");
    std::fs::create_dir_all(scratch.as_std_path()).expect("scratch");
    let source = scratch.join("Tool.lean");
    std::fs::write(source.as_std_path(), text).expect("write the generated module");
    let normalizer = lexlean::verify::child::Normalizer::new(
        &scratch,
        &project.root,
        &project.root,
        &toolchain.root,
    );
    let bin = toolchain.root.join("bin");
    let record = lexlean::verify::child::run(
        &lexlean::verify::child::ChildSpec {
            tool: "lean",
            module: Some("Tool".to_owned()),
            program: &toolchain.lean.path,
            executable_sha256: toolchain.lean.sha256,
            argv: vec![source.to_string()],
            cwd: &scratch,
            extra_env: Vec::new(),
            home: lexlean::verify::child::ChildHome::Toolchain {
                toolchain_bin: &bin,
            },
        },
        &inner.config.limits,
        &normalizer,
    )
    .expect("lean runs");
    (
        record.exit_code,
        format!("{}{}", record.stdout, record.stderr),
    )
}

/// The generated Lean of a project copy's one module.
fn lean_of(project: &P) -> String {
    rendered(project).modules[0].lean_text.clone()
}

/// The pinned Lean elaborates the project's generated module without a
/// message.
fn assert_elaborates(label: &str, project: &P) {
    let (exit, messages) = elaborate(project, &lean_of(project));
    assert!(
        exit == 0 && messages.trim().is_empty(),
        "{label}: Lean exited {exit} and said: {messages}"
    );
}

/// The pinned Lean refuses the project's generated module, and says `needle`.
fn assert_lean_refuses(label: &str, project: &P, needle: &str) {
    let (exit, messages) = elaborate(project, &lean_of(project));
    assert!(
        exit != 0 && messages.contains(needle),
        "{label}: Lean exited {exit} and said: {messages:?}, expected {needle:?}"
    );
}

/// Apply an edit to a copy of the fixture that the compiler still accepts.
fn forged(fixture: &str, label: &str, edit: impl FnOnce(&mut Json)) -> P {
    let project = P::tooling(fixture);
    let mut value = data(&project);
    edit(&mut value);
    put(&project, &value);
    accepted(label, &project);
    project
}

fn tl_10() {
    baselines();
    for fixture in FIXTURES {
        let project = P::tooling(fixture);
        let mut value = data(&project);
        let build = rendered(&project);
        let module = &build.modules[0];
        let lean = &module.lean_text;
        let declarations = declarations(&mut value).clone();
        let count = |kind: &str| declarations.iter().filter(|d| d["kind"] == kind).count();

        // The fixed header, then nothing but renderings.
        assert!(
            lean.starts_with(
                "module\npublic meta import Lean\nset_option autoImplicit false\nset_option linter.unusedVariables false\n"
            ),
            "{fixture}: the fixed header"
        );
        assert!(
            !lean.contains("--") && !lean.contains("/-"),
            "{fixture}: the Lean text has no comment"
        );
        let definitions =
            lean.matches("meta def ").count() + lean.matches("meta partial def ").count();
        assert_eq!(
            definitions,
            count("function"),
            "{fixture}: one definition per function"
        );
        assert_eq!(
            lean.matches("\nsyntax (name := ").count(),
            count("command"),
            "{fixture}: one syntax per command"
        );
        assert_eq!(
            lean.matches("\nattribute [command_elab ").count(),
            count("command"),
            "{fixture}: one registration per command"
        );
        assert_eq!(
            lean.matches("\n#eval ").count(),
            count("evaluate"),
            "{fixture}: one evaluation per evaluate"
        );
        for forbidden in [
            "inductive ",
            "structure ",
            "opaque ",
            "axiom ",
            "instance ",
            "theorem ",
            "example ",
            "sorry",
        ] {
            assert!(
                !lean.contains(forbidden),
                "{fixture}: a registry row has no Lean of its own, but the text holds `{forbidden}`"
            );
        }

        // Output coverage is complete (§19.6, §20.5) and every non-blank byte
        // of both artifacts maps back to the source.
        lexlean::backend::check_output_closure(lean, &module.coverage.lean)
            .unwrap_or_else(|failure| panic!("{fixture}: Lean coverage: {failure:?}"));
        lexlean::backend::check_output_closure(&module.tex_text, &module.coverage.latex)
            .unwrap_or_else(|failure| panic!("{fixture}: LaTeX coverage: {failure:?}"));
        for (artifact, text) in [(0usize, lean.as_str()), (1, module.tex_text.as_str())] {
            for (index, byte) in text.bytes().enumerate() {
                assert!(
                    byte.is_ascii_whitespace() || module.map.remap(artifact, index).is_some(),
                    "{fixture}: byte {index} of artifact {artifact} has a mapping"
                );
            }
        }

        // Each declaration with output has one piece, and its source range is
        // exactly that declaration's own object of the module data.
        // (The ranges are those of the file's own bytes: the notation.)
        let source = project.read_raw(MAIN);
        let with_output: Vec<&Json> = declarations
            .iter()
            .filter(|d| {
                matches!(
                    d["kind"].as_str(),
                    Some("function" | "command" | "invoke" | "evaluate")
                )
            })
            .collect();
        let mut pieces: Vec<&lexlean::artifact::source_map::Mapping> =
            module
                .map
                .mappings
                .iter()
                .filter(|mapping| {
                    mapping.artifact == 0
                        && module.map.nodes.iter().any(|node| {
                            node.id == mapping.node && node.kind == "tooling-declaration"
                        })
                })
                .collect();
        pieces.sort_by_key(|mapping| mapping.gen_start);
        assert_eq!(
            pieces.len(),
            with_output.len(),
            "{fixture}: one mapped piece per declaration with output"
        );
        for (piece, declaration) in pieces.iter().zip(&with_output) {
            let (start, end) = piece.src_range.expect("a source range");
            let object: Json = serde_json::from_str(
                &lexlean::surface::generic::parse(&source[start..end])
                    .unwrap_or_else(|error| {
                        panic!("{fixture}: the piece's source is not a declaration: {error}")
                    })
                    .to_canonical_string(),
            )
            .expect("canonical JSON");
            assert_eq!(
                &object, *declaration,
                "{fixture}: a piece maps to its own object"
            );
        }

        // The document has one entry per declaration, in order.
        let entries = module.tex_text.matches("\\subsection*{").count();
        assert_eq!(
            entries,
            declarations.len(),
            "{fixture}: one LaTeX entry per declaration"
        );
        let kinds: Vec<&str> = module
            .tex_text
            .lines()
            .filter_map(|line| line.strip_prefix("\\noindent Kind: \\texttt{"))
            .filter_map(|rest| rest.split('}').next())
            .collect();
        let expected: Vec<&str> = declarations
            .iter()
            .filter_map(|d| d["kind"].as_str())
            .collect();
        assert_eq!(
            kinds, expected,
            "{fixture}: each entry states its kind, in order"
        );
    }
    if !support::lean_backed("TL-10") {
        return;
    }
    for fixture in FIXTURES {
        assert_elaborates(fixture, &P::tooling(fixture));
    }
    // A registry that misstates an arity, with a tool that is consistent with
    // the misstatement: the compiler cannot see it, and Lean refuses it.
    let arity = forged(
        "core",
        "a registry that gives Nat.pred another arity",
        |v| {
            named(v, "Nat.pred")["signature"] = json!({
                "kind": "arrow",
                "domain": {"kind": "const", "name": "Nat"},
                "codomain": {"kind": "arrow",
                    "domain": {"kind": "const", "name": "Nat"},
                    "codomain": {"kind": "const", "name": "Nat"}}
            });
            *body(v, "Core.predecessor") = call("Nat.pred", vec![], vec![var("n"), var("n")]);
        },
    );
    assert_lean_refuses("an arity", &arity, "Function expected");
    // A registry that omits a constructor, with a match that is exhaustive
    // over what it lists.
    let omitted = forged("core", "a registry that omits Option.none", |v| {
        named(v, "Option")["constructors"]
            .as_array_mut()
            .expect("constructors")
            .retain(|c| c["name"] != "none");
        at(v, "Core.orZero", "/body/branches")
            .as_array_mut()
            .expect("branches")
            .retain(|b| b["constructor"] != "Option.none");
    });
    assert_lean_refuses("a constructor list", &omitted, "Missing cases");
}

fn tl_11() {
    baselines();
    let root = support::repo_root();
    let directories = crate::tooling::fixtures(root.as_std_path()).expect("fixtures");
    let committed: Vec<String> = directories
        .iter()
        .map(|directory| {
            directory
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    assert_eq!(
        committed,
        FIXTURES
            .iter()
            .map(|name| (*name).to_owned())
            .collect::<Vec<_>>(),
        "every committed fixture is one of the suite's"
    );
    let files = crate::tooling::check_golden(root.as_std_path(), false)
        .unwrap_or_else(|failure| panic!("{failure}"));
    assert!(files > 0, "no golden file was compared");
    let reproduced = crate::tooling::check_reproducibility(root.as_std_path())
        .unwrap_or_else(|failure| panic!("{failure}"));
    assert_eq!(
        files, reproduced,
        "the goldens and the builds are the same files"
    );

    // The platform-independent tree names no absolute path and carries a
    // semantic ID that depends on the module and on nothing of the machine.
    for fixture in FIXTURES {
        let (first, one) =
            crate::tooling::files(&root.join("tests/tooling").join(fixture).into_std_path_buf())
                .expect("builds");
        drop(first);
        assert!(
            one.iter().any(|(path, _)| path.ends_with(".lean"))
                && one.iter().any(|(path, _)| path.ends_with(".tex")),
            "{fixture}: the tree holds the Lean and the LaTeX"
        );
    }

    // Everything of a module is part of its identity: each edit keeps the
    // module valid and changes the semantic ID.
    type Edit = fn(&mut Json);
    let edits: [(&str, &str, Edit); 7] = [
        ("a term", "core", |v| {
            branch(body(v, "Core.sum"), "List.nil")["body"] = nat("1");
        }),
        ("a registry row", "core", |v| {
            named(v, "Nat.add")["role"] = json!("addition");
        }),
        ("a plumbing name", "core", |v| {
            let row = declarations(v)
                .iter_mut()
                .find(|d| d["kind"] == "plumbing")
                .expect("plumbing");
            row["names"].as_array_mut().expect("names").pop();
        }),
        ("a source pin", "core", |v| {
            named(v, "Nat")["source"]["file"] = json!("src/lean/Init/Core.lean");
        }),
        ("a field name", "core", |v| {
            constructor(v, "List", "cons")["fields"][0]["name"] = json!("first");
        }),
        ("a command atom", "commands", |v| {
            named(v, "Commands.require")["atoms"][0] =
                json!({"kind": "token", "text": "lexlean_demand "});
        }),
        ("a traversal flag", "traversal", |v| {
            named(v, "Traversal.alt")
                .as_object_mut()
                .expect("object")
                .remove("traversal");
            named(v, "Traversal.alt")["public"] = json!(false);
        }),
    ];
    for (label, fixture, edit) in edits {
        let before = accepted(label, &P::tooling(fixture)).semantic_id;
        let project = P::tooling(fixture);
        let mut value = data(&project);
        edit(&mut value);
        put(&project, &value);
        match linked(&project) {
            Ok(checked) => assert_ne!(
                checked.semantic_id, before,
                "{label} changes the semantic ID"
            ),
            // The one edit that is not valid on its own is not an identity
            // edit; it must be refused rather than silently kept.
            Err(error) => assert_eq!(label, "a traversal flag", "{label}: {error:?}"),
        }
    }

    // The schema lies outside the compiler-semantics ID (§7, §21.2): adding the
    // tooling language changed no existing identity, so the committed locks of
    // all three languages record the ID their compiler computes.
    assert!(
        root.join("tooling/schemas/tooling-module.schema.json")
            .is_file()
            && !root.join("schemas/tooling-module.schema.json").exists(),
        "the tooling schema is under tooling/schemas and not under schemas"
    );
    let spec = support::spec_text();
    let section = spec
        .split("### 21.2 Compiler-semantics ID")
        .nth(1)
        .and_then(|rest| rest.split("### 21.3").next())
        .expect("§21.2");
    assert!(
        !section.contains("tooling-module.schema.json"),
        "§21.2 lists no tooling schema among the files of the ID"
    );
    for (example, language) in [
        ("examples/nat-add-zero/lexlean.lock", "1.0"),
        ("examples/semantic-1.1/lexlean.lock", "1.1"),
        ("examples/higher-order/lexlean.lock", "1.2"),
    ] {
        let bytes = std::fs::read(root.join(example).as_std_path()).expect("lock");
        let lock = lexlean::api::parse_lock_bytes(example, &bytes).expect("lock parses");
        assert_eq!(lock.language, language);
        assert_eq!(
            lock.compiler_semantics,
            lexlean::compiler_semantics_id_for(language),
            "{example} records the {language} ID"
        );
    }
}

// ---------------------------------------------------------------------------

/// Run the case for one `tooling` ID.
pub fn run(id: &str) {
    match id {
        "TL-01" => tl_01(),
        "TL-02" => tl_02(),
        "TL-03" => tl_03(),
        "TL-04" => tl_04(),
        "TL-05" => tl_05(),
        "TL-06" => tl_06(),
        "TL-07" => tl_07(),
        "TL-08" => tl_08(),
        "TL-09" => tl_09(),
        "TL-10" => tl_10(),
        "TL-11" => tl_11(),
        _ => panic!("no conformance case is wired for {id}"),
    }
}
