//! The `replay-cache` suite: RC-01..RC-06 (SPEC.md §22.13).

use std::cell::RefCell;
use std::collections::BTreeMap;

use camino::Utf8PathBuf;
use lexlean::artifact::digest_set::DigestSet;
use lexlean::verify::replay_cache::{
    canonical_declaration, executable_identity, export_facts, replay_units, result_of, unit_key,
    DeclarationFacts, ExecutableIdentity, Granularity, Lookup, Miss, ReplayCache,
    ReplayEnvironment, ReplayGraph, ReplayReport, ReplayResult, ReplayUnit,
};

use crate::support;

pub(crate) fn run(id: &str) {
    match id {
        "RC-01" => keys_are_digest_sets_over_one_serialisation(),
        "RC-02" => keys_form_a_merkle_dag(),
        "RC-03" => a_warm_replay_reuses_by_key(),
        "RC-04" => tampered_entries_are_rejected(),
        "RC-05" => a_changed_environment_invalidates_the_cache(),
        "RC-06" => facts_from_real_modules_replay_cold_and_warm(),
        _ => panic!("no replay-cache conformance case is wired for {id}"),
    }
}

fn environment(toolchain: &str, mathlib: &str) -> ReplayEnvironment {
    ReplayEnvironment {
        toolchain: toolchain.to_owned(),
        mathlib_revision: mathlib.to_owned(),
    }
}

fn pinned() -> ReplayEnvironment {
    environment(
        lexlean::LEAN_TOOLCHAIN,
        "0123456789abcdef0123456789abcdef01234567",
    )
}

fn decl(name: &str, deps: &[&str]) -> DeclarationFacts {
    DeclarationFacts {
        name: name.to_owned(),
        module: "Demo".to_owned(),
        level_params: vec!["u".to_owned()],
        attrs: "theorem".to_owned(),
        type_text: format!("C {name} []\nroot 0"),
        value: Some(format!("C {name}.proof []\nroot 0")),
        deps: deps.iter().map(|d| (*d).to_owned()).collect(),
    }
}

/// `base`, two branches over it, `top` over both, and `lone` apart.
fn diamond() -> Vec<DeclarationFacts> {
    vec![
        decl("base", &[]),
        decl("left", &["base"]),
        decl("right", &["base"]),
        decl("top", &["left", "right"]),
        decl("lone", &[]),
    ]
}

fn graph(environment: &ReplayEnvironment, facts: &[DeclarationFacts]) -> ReplayGraph {
    ReplayGraph::build(environment, facts, &BTreeMap::new()).expect("the facts are keyed")
}

fn keys_of(facts: &[DeclarationFacts]) -> BTreeMap<String, DigestSet> {
    graph(&pinned(), facts).keys().clone()
}

fn changed(
    before: &BTreeMap<String, DigestSet>,
    after: &BTreeMap<String, DigestSet>,
) -> Vec<String> {
    before
        .iter()
        .filter(|(name, key)| after[*name] != **key)
        .map(|(name, _)| name.clone())
        .collect()
}

/// RC-01.
fn keys_are_digest_sets_over_one_serialisation() {
    let key = keys_of(&diamond())["lone"];
    let label = key.label();
    assert_eq!(label.len(), "blake3:".len() + 64, "{label}");
    assert!(label.starts_with("blake3:"));
    let digits = &label["blake3:".len()..];
    assert!(digits
        .bytes()
        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)));
    assert_eq!(digits, key.blake3.to_hex());
    let json = key.to_json().to_canonical_string();
    assert_eq!(
        json,
        format!(
            "{{\"blake3\":\"{}\",\"sha256\":\"{}\"}}",
            key.blake3.to_hex(),
            key.sha256.to_hex()
        )
    );
    assert_eq!(DigestSet::from_json(&key.to_json()), Ok(key));

    // Both digests cover the same bytes: the canonical serialisation hashes
    // to the same pair whichever way it is fed in.
    let facts = decl("lone", &[]);
    assert_eq!(
        DigestSet::of(&canonical_declaration(&facts)),
        DigestSet::of(&canonical_declaration(&facts))
    );

    let edit = |change: &dyn Fn(&mut DeclarationFacts)| {
        let mut all = diamond();
        let target = all.iter_mut().find(|d| d.name == "lone").expect("lone");
        change(target);
        let name = target.name.clone();
        keys_of(&all)[&name]
    };
    assert_eq!(key, edit(&|_| {}), "equal inputs give equal keys");
    for (what, other) in [
        ("name", edit(&|d| d.name = "lone2".to_owned())),
        (
            "universe parameters",
            edit(&|d| d.level_params = vec!["v".to_owned()]),
        ),
        ("kind attributes", edit(&|d| d.attrs = "def".to_owned())),
        ("type", edit(&|d| d.type_text.push('x'))),
        ("value", edit(&|d| d.value = Some("other".to_owned()))),
        ("no value", edit(&|d| d.value = None)),
    ] {
        assert_ne!(key, other, "a changed {what} changes the key");
    }
    let mut with_dependency = diamond();
    with_dependency
        .iter_mut()
        .find(|d| d.name == "lone")
        .expect("lone")
        .deps
        .push("base".to_owned());
    assert_ne!(
        key,
        keys_of(&with_dependency)["lone"],
        "a dependency changes the key"
    );
    let moved = edit(&|d| d.module = "Elsewhere".to_owned());
    assert_eq!(key, moved, "the declaring module is not an input");
    for other in [
        environment("leanprover/lean4:v4.32.2", &pinned().mathlib_revision),
        environment(&pinned().toolchain, "other"),
    ] {
        assert_ne!(
            key,
            graph(&other, &diamond()).keys()["lone"],
            "the environment is an input"
        );
    }
}

/// RC-02.
fn keys_form_a_merkle_dag() {
    let before = keys_of(&diamond());
    let edit = |name: &str| {
        let mut facts = diamond();
        facts
            .iter_mut()
            .find(|d| d.name == name)
            .expect("name")
            .type_text
            .push('!');
        changed(&before, &keys_of(&facts))
    };
    assert_eq!(edit("base"), ["base", "left", "right", "top"]);
    assert_eq!(edit("left"), ["left", "top"]);
    assert_eq!(edit("right"), ["right", "top"]);
    assert_eq!(edit("top"), ["top"]);
    assert_eq!(edit("lone"), ["lone"]);

    let group = vec![
        decl("T", &["T.mk", "T.rec"]),
        decl("T.mk", &["T"]),
        decl("T.rec", &["T", "T.mk"]),
        decl("user", &["T.rec"]),
    ];
    let built = graph(&pinned(), &group);
    let digest = built.group("T").copied().expect("group digest");
    assert_eq!(built.group("T.mk"), Some(&digest));
    assert_eq!(built.group("T.rec"), Some(&digest));
    assert_ne!(built.group("user"), Some(&digest));
    let mut edited = group.clone();
    edited[1].type_text.push('!');
    assert_eq!(
        changed(built.keys(), &keys_of(&edited)),
        ["T", "T.mk", "T.rec", "user"]
    );

    let refused = |facts: &[DeclarationFacts]| {
        ReplayGraph::build(&pinned(), facts, &BTreeMap::new()).expect_err("refused")
    };
    let repeated = refused(&[decl("a", &[]), decl("a", &[])]);
    let dangling = refused(&[decl("a", &["missing"])]);
    let anchored = ReplayGraph::build(
        &pinned(),
        &[decl("a", &["missing"])],
        &BTreeMap::from([("missing".to_owned(), DigestSet::of(b"anchor"))]),
    );
    assert!(
        anchored.is_ok(),
        "a given key anchors a name the facts do not state"
    );
    let unparsed = lexlean::verify::replay_cache::parse_facts("{\"name\":").expect_err("refused");
    for diagnostic in [repeated, dangling, unparsed] {
        assert_eq!(
            diagnostic.code.as_str(),
            "LLV7020",
            "{}",
            diagnostic.message
        );
    }
}

struct Store {
    _dir: tempfile::TempDir,
    cache: ReplayCache,
}

fn store() -> Store {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = Utf8PathBuf::from_path_buf(dir.path().join("replay")).expect("utf-8 path");
    Store {
        cache: ReplayCache::open(root),
        _dir: dir,
    }
}

fn executable(bytes: &[u8]) -> ExecutableIdentity {
    ExecutableIdentity {
        tool: "leanchecker".to_owned(),
        digest: DigestSet::of(bytes),
    }
}

fn units(
    environment: &ReplayEnvironment,
    facts: &[DeclarationFacts],
    granularity: Granularity,
) -> Vec<ReplayUnit> {
    let built = graph(environment, facts);
    match granularity {
        Granularity::Declaration => facts
            .iter()
            .map(|d| ReplayUnit {
                label: d.name.clone(),
                granularity,
                key: *built.key(&d.name).expect("key"),
            })
            .collect(),
        Granularity::Module => {
            let mut by_module: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
            for d in facts {
                by_module
                    .entry(d.module.as_str())
                    .or_default()
                    .push(d.name.as_str());
            }
            by_module
                .into_iter()
                .map(|(module, names)| ReplayUnit {
                    label: module.to_owned(),
                    granularity,
                    key: built.unit(granularity, names).expect("unit"),
                })
                .collect()
        }
    }
}

fn replay(
    store: &Store,
    environment: &ReplayEnvironment,
    exe: &ExecutableIdentity,
    units: &[ReplayUnit],
) -> (Result<ReplayReport, lexlean::Diagnostic>, Vec<String>) {
    let called = RefCell::new(Vec::new());
    let outcome = replay_units(&store.cache, environment, exe, units, |unit| {
        called.borrow_mut().push(unit.label.clone());
        Ok(ReplayResult {
            exit_code: 0,
            stdout: format!("replayed {}\n", unit.label),
            stderr: String::new(),
        })
    });
    (outcome, called.into_inner())
}

/// RC-03.
fn a_warm_replay_reuses_by_key() {
    let exe = executable(b"leanchecker 1");
    for granularity in [Granularity::Declaration, Granularity::Module] {
        let store = store();
        let mut facts = diamond();
        for (index, d) in facts.iter_mut().enumerate() {
            d.module = format!("M{}", index / 2);
        }
        let cold = units(&pinned(), &facts, granularity);
        let total = cold.len();
        let (outcome, called) = replay(&store, &pinned(), &exe, &cold);
        assert_eq!(called.len(), total);
        assert_eq!(outcome.expect("cold").replayed.len(), total);

        let (outcome, called) = replay(&store, &pinned(), &exe, &cold);
        assert!(called.is_empty(), "{granularity:?}: {called:?}");
        assert_eq!(outcome.expect("warm").reused.len(), total);

        // One declaration edited: the units whose members reach it replay.
        facts
            .iter_mut()
            .find(|d| d.name == "left")
            .expect("left")
            .type_text
            .push('!');
        let warm = units(&pinned(), &facts, granularity);
        let (outcome, called) = replay(&store, &pinned(), &exe, &warm);
        let report = outcome.expect("warm after an edit");
        assert_eq!(called, report.replayed);
        match granularity {
            // `left` and `top` are above the edit; the others keep their keys.
            Granularity::Declaration => assert_eq!(called, ["left", "top"]),
            // M0 = base, left; M1 = right, top; M2 = lone. M0 holds `left`;
            // M1 holds `top`, which reaches it; M2 does not.
            Granularity::Module => {
                assert_eq!(called, ["M0", "M1"]);
                assert_eq!(report.reused, ["M2"]);
            }
        }
    }

    // A result is recorded only for a replay that succeeded.
    let store = store();
    let all = units(&pinned(), &diamond(), Granularity::Declaration);
    let failed = replay_units(&store.cache, &pinned(), &exe, &all, |unit| {
        Ok(ReplayResult {
            exit_code: i32::from(unit.label == "left"),
            stdout: String::new(),
            stderr: "error".to_owned(),
        })
    });
    assert_eq!(
        failed.expect_err("a failing replay").code.as_str(),
        "LLV7003"
    );
    let left = all.iter().find(|u| u.label == "left").expect("left");
    assert_eq!(
        store
            .cache
            .lookup(&left.key, &pinned(), &exe)
            .expect("lookup"),
        Lookup::Miss(Miss::Absent)
    );

    // Each entry names the executable that produced it, and another is a miss.
    let (outcome, _) = replay(&store, &pinned(), &exe, &all);
    outcome.expect("replay after the failure");
    let text = std::fs::read_to_string(store.cache.entry_path(&all[0].key)).expect("entry");
    assert!(
        text.contains(&exe.digest.sha256.to_hex()) && text.contains(&exe.digest.blake3.to_hex()),
        "{text}"
    );
    let other = executable(b"leanchecker 2");
    assert_eq!(
        store
            .cache
            .lookup(&all[0].key, &pinned(), &other)
            .expect("lookup"),
        Lookup::Miss(Miss::ExecutableChanged)
    );
    let (outcome, called) = replay(&store, &pinned(), &other, &all);
    assert_eq!(
        called.len(),
        all.len(),
        "another executable replays every unit"
    );
    assert!(outcome.expect("other executable").reused.is_empty());
}

/// RC-04.
fn tampered_entries_are_rejected() {
    let exe = executable(b"leanchecker 1");
    let unit = units(&pinned(), &diamond(), Granularity::Declaration).remove(0);
    let other_unit = units(&pinned(), &diamond(), Granularity::Declaration).remove(4);
    let fresh = || {
        let store = store();
        replay(&store, &pinned(), &exe, std::slice::from_ref(&unit))
            .0
            .expect("cold");
        let path = store.cache.entry_path(&unit.key);
        (store, path)
    };
    let assert_rejected = |store: &Store, path: &camino::Utf8Path, key: &ReplayUnit, what: &str| {
        let before = std::fs::read(path).ok();
        let refused = store
            .cache
            .lookup(&key.key, &pinned(), &exe)
            .expect_err(what);
        assert_eq!(
            refused.code.as_str(),
            "LLV7019",
            "{what}: {}",
            refused.message
        );
        let (outcome, called) = replay(store, &pinned(), &exe, std::slice::from_ref(key));
        assert_eq!(outcome.expect_err(what).code.as_str(), "LLV7019", "{what}");
        assert!(called.is_empty(), "{what}: the replayer ran");
        assert_eq!(
            std::fs::read(path).ok(),
            before,
            "{what}: the file was replaced"
        );
    };

    // Bytes changed, key kept.
    let (store, path) = fresh();
    let text = std::fs::read_to_string(&path).expect("entry");
    std::fs::write(&path, text.replace("replayed base", "replayed fake")).expect("tamper");
    assert_rejected(&store, &path, &unit, "result edited");

    // The producing executable's digest edited.
    let (store, path) = fresh();
    let text = std::fs::read_to_string(&path).expect("entry");
    let forged = executable(b"leanchecker 2").digest.blake3.to_hex();
    std::fs::write(&path, text.replace(&exe.digest.blake3.to_hex(), &forged)).expect("tamper");
    assert_rejected(&store, &path, &unit, "executable edited");

    // Stored under another key.
    let (store, path) = fresh();
    let target = store.cache.entry_path(&other_unit.key);
    std::fs::create_dir_all(target.parent().expect("parent")).expect("directory");
    std::fs::copy(&path, &target).expect("copy");
    assert_rejected(&store, &target, &other_unit, "stored under another key");

    // Truncated, and no longer an entry.
    let (store, path) = fresh();
    let text = std::fs::read_to_string(&path).expect("entry");
    std::fs::write(&path, &text[..text.len() / 2]).expect("tamper");
    assert_rejected(&store, &path, &unit, "truncated");
    std::fs::write(&path, b"").expect("tamper");
    assert_rejected(&store, &path, &unit, "emptied");
}

/// RC-05.
fn a_changed_environment_invalidates_the_cache() {
    let store = store();
    let exe = executable(b"leanchecker 1");
    let first = units(&pinned(), &diamond(), Granularity::Declaration);
    replay(&store, &pinned(), &exe, &first).0.expect("first");
    let read_all = || -> Vec<(std::path::PathBuf, Vec<u8>)> {
        let mut out = Vec::new();
        let entries = store
            .cache
            .entry_path(&DigestSet::of(b""))
            .parent()
            .expect("shard")
            .parent()
            .expect("entries")
            .to_path_buf();
        for shard in std::fs::read_dir(entries.as_std_path()).expect("entries") {
            for file in std::fs::read_dir(shard.expect("shard").path()).expect("shard") {
                let path = file.expect("file").path();
                out.push((path.clone(), std::fs::read(&path).expect("read")));
            }
        }
        out.sort();
        out
    };
    let before = read_all();
    for other in [
        environment("leanprover/lean4:v4.32.2", &pinned().mathlib_revision),
        environment(
            &pinned().toolchain,
            "fedcba9876543210fedcba9876543210fedcba98",
        ),
    ] {
        let under = units(&other, &diamond(), Granularity::Declaration);
        let (outcome, called) = replay(&store, &other, &exe, &under);
        assert_eq!(
            called.len(),
            under.len(),
            "nothing recorded under the first environment is reused"
        );
        assert!(outcome.expect("other environment").reused.is_empty());
    }
    let after = read_all();
    assert_eq!(after.len(), before.len() * 3);
    for entry in &before {
        assert!(
            after.contains(entry),
            "an entry of the first environment was changed"
        );
    }
    let (outcome, called) = replay(&store, &pinned(), &exe, &first);
    assert!(called.is_empty());
    assert_eq!(outcome.expect("first again").reused.len(), first.len());
}

/// RC-06: the pinned Lean exports the facts, the pinned `leanchecker`
/// replays, and a one-line edit replays only what it reaches.
fn facts_from_real_modules_replay_cold_and_warm() {
    if !support::lean_backed("RC-06") {
        // The keying itself is platform independent and RC-01..RC-05 ran it.
        let _ = unit_key(Granularity::Module, &[]);
        return;
    }
    use lexlean::config::Limits;
    use lexlean::verify::child::{run, ChildHome, ChildSpec, Normalizer};
    let limits = Limits {
        max_file_bytes: 16_777_216,
        max_total_source_bytes: 67_108_864,
        max_primitive_atoms: 2_000_000,
        max_token_lattice_edges: 4_000_000,
        max_parse_states: 4_000_000,
        max_ir_nodes: 2_000_000,
        max_scope_depth: 1024,
        max_import_depth: 128,
        max_diagnostics: 256,
        max_child_output_bytes: 67_108_864,
        child_timeout_ms: 300_000,
    };
    let toolchain = lexlean::verify::toolchain::preflight(&limits).expect("the pinned toolchain");
    let work = tempfile::tempdir().expect("tempdir");
    let dir = Utf8PathBuf::from_path_buf(work.path().to_path_buf()).expect("utf-8 path");
    let normalizer = Normalizer::new(&dir, &dir, &dir, &toolchain.root);
    let lean_path = dir.to_string();
    let bin = toolchain.root.join("bin");
    let write = |module: &str, text: &str| {
        std::fs::write(dir.join(format!("{module}.lean")).as_std_path(), text)
            .expect("write source");
    };
    let compile = |module: &str| {
        let record = run(
            &ChildSpec {
                tool: "lean",
                module: Some(module.to_owned()),
                program: &toolchain.lean.path,
                executable_sha256: toolchain.lean.sha256,
                argv: vec![
                    format!("--root={dir}"),
                    "-o".to_owned(),
                    dir.join(format!("{module}.olean")).to_string(),
                    dir.join(format!("{module}.lean")).to_string(),
                ],
                cwd: &dir,
                extra_env: vec![("LEAN_PATH".to_owned(), lean_path.clone())],
                home: ChildHome::Toolchain {
                    toolchain_bin: &bin,
                },
            },
            &limits,
            &normalizer,
        )
        .expect("lean runs");
        assert_eq!(
            record.exit_code, 0,
            "{module}: {}{}",
            record.stdout, record.stderr
        );
    };

    // C imports B and uses only A, so an edit to B's declaration is not above it.
    let a = "def a : Nat := 1\ntheorem a_pos : 0 < a := by decide\n";
    let b = |step: u32| {
        format!("import A\ndef b : Nat := a + {step}\ntheorem b_gt : a < b := by unfold b; omega\n")
    };
    let c = "import B\ndef c : Nat := a + 2\n";
    write("A", a);
    write("B", &b(1));
    write("C", c);
    for module in ["A", "B", "C"] {
        compile(module);
    }

    let executable =
        executable_identity("leanchecker", &toolchain.leanchecker.path).expect("executable");
    assert_eq!(
        executable.digest.sha256, toolchain.leanchecker.sha256,
        "the same bytes the preflight hashed"
    );
    let env = environment(lexlean::LEAN_TOOLCHAIN, "");
    let unit_of = |module: &str| {
        let facts = export_facts(&toolchain, module, &lean_path, &dir, &limits, &normalizer)
            .expect("facts");
        let built = graph(&env, &facts);
        let own: Vec<&str> = facts
            .iter()
            .filter(|f| f.module == module)
            .map(|f| f.name.as_str())
            .collect();
        assert!(!own.is_empty(), "{module} declares constants");
        ReplayUnit {
            label: module.to_owned(),
            granularity: Granularity::Module,
            key: built.unit(Granularity::Module, own).expect("unit"),
        }
    };
    let replay_module = |unit: &ReplayUnit| -> Result<ReplayResult, lexlean::Diagnostic> {
        lexlean::verify::leanchecker::replay_module(
            &toolchain,
            &unit.label,
            &lean_path,
            &dir,
            &limits,
            &normalizer,
        )
        .map(|record| result_of(&record))
    };
    let measure = |store: &Store, label: &str| {
        let started = std::time::Instant::now();
        let units: Vec<ReplayUnit> = ["A", "B", "C"].iter().map(|m| unit_of(m)).collect();
        let keyed = started.elapsed();
        let report =
            replay_units(&store.cache, &env, &executable, &units, replay_module).expect("replay");
        eprintln!(
            "RC-06 {label}: replayed {:?}, reused {:?}; facts and keys {} ms, replays {} ms",
            report.replayed,
            report.reused,
            keyed.as_millis(),
            (started.elapsed() - keyed).as_millis()
        );
        (units, report)
    };

    let store = store();
    let (_, cold) = measure(&store, "cold");
    assert_eq!(cold.replayed, ["A", "B", "C"]);
    let (_, again) = measure(&store, "unchanged");
    assert_eq!(again.reused, ["A", "B", "C"]);

    // The one-line edit: B's `b` becomes `a + 3`, and B and C are rebuilt.
    write("B", &b(3));
    compile("B");
    compile("C");
    let (units, warm) = measure(&store, "after the edit");
    assert_eq!(warm.replayed, ["B"], "only the edited module replays");
    assert_eq!(warm.reused, ["A", "C"]);

    // The warm verdicts equal a cold run of the edited modules.
    let fresh = self::store();
    let (fresh_units, fresh_report) = measure(&fresh, "cold again, edited");
    assert_eq!(fresh_report.replayed, ["A", "B", "C"]);
    for (unit, fresh_unit) in units.iter().zip(&fresh_units) {
        assert_eq!(unit.key, fresh_unit.key);
        let read = |store: &Store, unit: &ReplayUnit| match store
            .cache
            .lookup(&unit.key, &env, &executable)
            .expect("lookup")
        {
            Lookup::Hit(entry) => entry.result,
            Lookup::Miss(miss) => panic!("{}: {miss:?}", unit.label),
        };
        assert_eq!(
            read(&store, unit),
            read(&fresh, fresh_unit),
            "{}",
            unit.label
        );
    }
}
