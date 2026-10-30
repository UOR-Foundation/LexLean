use std::cell::RefCell;
use std::collections::BTreeMap;

use camino::Utf8PathBuf;

use super::*;
use crate::artifact::digest_set::DigestSet;

fn env(toolchain: &str, mathlib: &str) -> ReplayEnvironment {
    ReplayEnvironment {
        toolchain: toolchain.to_owned(),
        mathlib_revision: mathlib.to_owned(),
    }
}

fn base_env() -> ReplayEnvironment {
    env("leanprover/lean4:v4.32.1", "0123456789abcdef")
}

fn decl(name: &str, deps: &[&str]) -> DeclarationFacts {
    DeclarationFacts {
        name: name.to_owned(),
        module: "M".to_owned(),
        level_params: vec!["u".to_owned()],
        attrs: "theorem".to_owned(),
        type_text: format!("C {name} []\nroot 0"),
        value: Some(format!("C {name}.proof []\nroot 0")),
        deps: deps.iter().map(|d| (*d).to_owned()).collect(),
    }
}

/// `base` under two branches, joined by `top`, and `lone` beside them all.
fn diamond() -> Vec<DeclarationFacts> {
    vec![
        decl("base", &[]),
        decl("left", &["base"]),
        decl("right", &["base"]),
        decl("top", &["left", "right"]),
        decl("lone", &[]),
    ]
}

fn keys(facts: &[DeclarationFacts]) -> BTreeMap<String, DigestSet> {
    ReplayGraph::build(&base_env(), facts, &BTreeMap::new())
        .expect("keys")
        .keys()
        .clone()
}

fn changed<'a>(
    before: &'a BTreeMap<String, DigestSet>,
    after: &BTreeMap<String, DigestSet>,
) -> Vec<&'a str> {
    before
        .iter()
        .filter(|(name, key)| after[*name] != **key)
        .map(|(name, _)| name.as_str())
        .collect()
}

#[test]
fn a_key_is_a_digest_set_with_a_blake3_label() {
    let key = keys(&diamond())["base"];
    let label = key.label();
    assert!(label.starts_with("blake3:"));
    assert_eq!(label.len(), "blake3:".len() + 64);
    assert!(label["blake3:".len()..]
        .bytes()
        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)));
    assert_eq!(DigestSet::from_json(&key.to_json()), Ok(key));
    // Both digests cover the same bytes: neither is the other's input.
    assert_ne!(key.sha256.to_hex(), key.blake3.to_hex());
}

#[test]
fn equal_inputs_give_equal_keys_whatever_the_order_of_the_facts() {
    let forward = keys(&diamond());
    let mut reversed = diamond();
    reversed.reverse();
    assert_eq!(forward, keys(&reversed));
}

#[test]
fn every_input_of_a_key_changes_it() {
    let before = keys(&diamond())["lone"];
    let edit = |f: &dyn Fn(&mut DeclarationFacts)| {
        let mut facts = diamond();
        let target = facts.iter_mut().find(|d| d.name == "lone").expect("lone");
        f(target);
        let name = target.name.clone();
        keys(&facts)[&name]
    };
    assert_ne!(before, edit(&|d| d.name = "lone2".to_owned()));
    assert_ne!(before, edit(&|d| d.level_params = vec!["v".to_owned()]));
    assert_ne!(before, edit(&|d| d.level_params.clear()));
    assert_ne!(before, edit(&|d| d.attrs = "def".to_owned()));
    assert_ne!(before, edit(&|d| d.type_text.push('x')));
    assert_ne!(before, edit(&|d| d.value = Some("other".to_owned())));
    assert_ne!(before, edit(&|d| d.value = None));
    // The declaring module is not an input: moving a declaration moves no key.
    assert_eq!(before, edit(&|d| d.module = "Elsewhere".to_owned()));
}

#[test]
fn fields_that_run_together_do_not_collide() {
    // Length-prefixed fields: moving a byte between two neighbours is a change.
    let mut a = decl("a", &[]);
    let mut b = decl("a", &[]);
    a.attrs = "ab".to_owned();
    a.type_text = "c".to_owned();
    b.attrs = "a".to_owned();
    b.type_text = "bc".to_owned();
    assert_ne!(canonical_declaration(&a), canonical_declaration(&b));
    assert_ne!(keys(&[a])["a"], keys(&[b])["a"]);
}

#[test]
fn a_changed_dependency_changes_every_key_above_it_and_none_beside_or_below() {
    let before = keys(&diamond());
    let edit = |name: &str| {
        let mut facts = diamond();
        facts
            .iter_mut()
            .find(|d| d.name == name)
            .expect("name")
            .type_text
            .push('!');
        changed(&before, &keys(&facts))
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>()
    };
    assert_eq!(edit("base"), ["base", "left", "right", "top"]);
    assert_eq!(edit("left"), ["left", "top"]);
    assert_eq!(edit("right"), ["right", "top"]);
    assert_eq!(edit("top"), ["top"]);
    assert_eq!(edit("lone"), ["lone"]);
}

#[test]
fn a_dependency_edge_is_part_of_the_key() {
    let before = keys(&diamond());
    let mut facts = diamond();
    facts
        .iter_mut()
        .find(|d| d.name == "right")
        .expect("right")
        .deps
        .clear();
    let after = keys(&facts);
    assert_eq!(changed(&before, &after), ["right", "top"]);
}

#[test]
fn declarations_that_name_one_another_are_one_group() {
    let facts = vec![
        decl("T", &["T.mk", "T.rec"]),
        decl("T.mk", &["T"]),
        decl("T.rec", &["T", "T.mk"]),
        decl("user", &["T.rec"]),
    ];
    let graph = ReplayGraph::build(&base_env(), &facts, &BTreeMap::new()).expect("graph");
    let group = graph.group("T").copied().expect("group");
    assert_eq!(graph.group("T.mk"), Some(&group));
    assert_eq!(graph.group("T.rec"), Some(&group));
    assert_ne!(graph.group("user"), Some(&group));
    // Members of a group have their own keys.
    assert_ne!(graph.key("T"), graph.key("T.mk"));

    let before = graph.keys().clone();
    let mut edited = facts.clone();
    edited[1].type_text.push('!');
    let after = keys(&edited);
    assert_eq!(changed(&before, &after), ["T", "T.mk", "T.rec", "user"]);
}

#[test]
fn an_anchor_stands_for_a_declaration_outside_the_facts() {
    let facts = vec![decl("uses", &["Nat.add"])];
    let anchor = |text: &[u8]| BTreeMap::from([("Nat.add".to_owned(), DigestSet::of(text))]);
    let one = ReplayGraph::build(&base_env(), &facts, &anchor(b"one")).expect("graph");
    let two = ReplayGraph::build(&base_env(), &facts, &anchor(b"two")).expect("graph");
    assert_ne!(one.key("uses"), two.key("uses"));
    assert!(one.key("Nat.add").is_none());
}

#[test]
fn facts_that_repeat_dangle_or_do_not_parse_are_refused_with_llv7020() {
    let build = |facts: &[DeclarationFacts], anchors: &BTreeMap<String, DigestSet>| {
        ReplayGraph::build(&base_env(), facts, anchors).expect_err("refused")
    };
    let none = BTreeMap::new();
    let repeated = build(&[decl("a", &[]), decl("a", &[])], &none);
    let dangling = build(&[decl("a", &["missing"])], &none);
    let both = build(
        &[decl("a", &[])],
        &BTreeMap::from([("a".to_owned(), DigestSet::of(b"x"))]),
    );
    let unparsed = parse_facts("not json\n").expect_err("refused");
    let unknown_member = parse_facts(
        "{\"name\":\"a\",\"module\":\"M\",\"level_params\":[],\"attrs\":\"\",\"type\":\"\",\"value\":null,\"deps\":[],\"extra\":1}\n",
    )
    .expect_err("refused");
    for diagnostic in [&repeated, &dangling, &both, &unparsed, &unknown_member] {
        assert_eq!(
            diagnostic.code.as_str(),
            "LLV7022",
            "{}",
            diagnostic.message
        );
    }
    assert!(
        dangling.message.contains("`missing`"),
        "{}",
        dangling.message
    );
    assert!(repeated.message.contains("`a`"), "{}", repeated.message);
}

#[test]
fn facts_lines_read_back_what_the_exporter_writes() {
    let line = "{\"attrs\":\"theorem all=a\",\"deps\":[\"b\"],\"level_params\":[\"u\"],\"module\":\"M\",\"name\":\"a\",\"type\":\"C b []\\nroot 0\",\"value\":null}\n";
    let facts = parse_facts(&format!("{line}\n{line}")).expect("facts");
    assert_eq!(facts.len(), 2);
    assert_eq!(facts[0].type_text, "C b []\nroot 0");
    assert_eq!(facts[0].value, None);
}

#[test]
fn a_changed_toolchain_or_mathlib_revision_changes_every_key() {
    let facts = diamond();
    let under = |environment: &ReplayEnvironment| {
        ReplayGraph::build(environment, &facts, &BTreeMap::new())
            .expect("keys")
            .keys()
            .clone()
    };
    let before = under(&base_env());
    for other in [
        env("leanprover/lean4:v4.32.2", "0123456789abcdef"),
        env("leanprover/lean4:v4.32.1", "fedcba9876543210"),
    ] {
        let after = under(&other);
        assert_eq!(changed(&before, &after).len(), before.len());
    }
}

#[test]
fn a_unit_key_covers_its_granularity_and_its_members_without_their_order() {
    let all = keys(&diamond());
    let (a, b) = (all["left"], all["right"]);
    assert_eq!(
        unit_key(Granularity::Module, &[a, b]),
        unit_key(Granularity::Module, &[b, a])
    );
    assert_ne!(
        unit_key(Granularity::Module, &[a, b]),
        unit_key(Granularity::Declaration, &[a, b])
    );
    assert_ne!(
        unit_key(Granularity::Module, &[a, b]),
        unit_key(Granularity::Module, &[a])
    );
    let graph = ReplayGraph::build(&base_env(), &diamond(), &BTreeMap::new()).expect("graph");
    assert_eq!(
        graph
            .unit(Granularity::Module, ["left", "right"])
            .expect("unit"),
        unit_key(Granularity::Module, &[a, b])
    );
    assert_eq!(
        graph
            .unit(Granularity::Module, ["nope"])
            .expect_err("refused")
            .code
            .as_str(),
        "LLV7022"
    );
}

#[test]
fn a_deep_chain_is_keyed_without_recursion() {
    let depth = 50_000;
    let mut facts = vec![decl("d0", &[])];
    for i in 1..depth {
        facts.push(decl(&format!("d{i}"), &[&format!("d{}", i - 1)]));
    }
    let graph = ReplayGraph::build(&base_env(), &facts, &BTreeMap::new()).expect("graph");
    assert_eq!(graph.keys().len(), depth);
}

struct Store {
    _dir: tempfile::TempDir,
    cache: ReplayCache,
}

fn store() -> Store {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = Utf8PathBuf::from_path_buf(dir.path().join("replay")).expect("utf-8");
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

fn units(environment: &ReplayEnvironment, facts: &[DeclarationFacts]) -> Vec<ReplayUnit> {
    let graph = ReplayGraph::build(environment, facts, &BTreeMap::new()).expect("graph");
    facts
        .iter()
        .map(|d| ReplayUnit {
            label: d.name.clone(),
            granularity: Granularity::Declaration,
            key: *graph.key(&d.name).expect("key"),
        })
        .collect()
}

fn run(
    store: &Store,
    environment: &ReplayEnvironment,
    exe: &ExecutableIdentity,
    units: &[ReplayUnit],
) -> (
    Result<ReplayReport, crate::diagnostic::Diagnostic>,
    Vec<String>,
) {
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

#[test]
fn a_warm_replay_reuses_unchanged_units_and_replays_the_changed_and_those_above() {
    let store = store();
    let exe = executable(b"leanchecker 1");
    let cold = units(&base_env(), &diamond());
    let (outcome, called) = run(&store, &base_env(), &exe, &cold);
    assert_eq!(called.len(), 5);
    assert_eq!(outcome.expect("cold").replayed.len(), 5);

    let (outcome, called) = run(&store, &base_env(), &exe, &cold);
    assert!(called.is_empty(), "{called:?}");
    assert_eq!(outcome.expect("warm").reused.len(), 5);

    let mut edited = diamond();
    edited
        .iter_mut()
        .find(|d| d.name == "left")
        .expect("left")
        .type_text
        .push('!');
    let warm = units(&base_env(), &edited);
    let (outcome, called) = run(&store, &base_env(), &exe, &warm);
    assert_eq!(called, ["left", "top"]);
    let report = outcome.expect("one-line edit");
    assert_eq!(report.reused, ["base", "right", "lone"]);
}

#[test]
fn an_entry_names_its_executable_and_another_executable_is_not_reused() {
    let store = store();
    let exe = executable(b"leanchecker 1");
    let all = units(&base_env(), &diamond());
    run(&store, &base_env(), &exe, &all).0.expect("cold");
    let text = std::fs::read_to_string(store.cache.entry_path(&all[0].key)).expect("entry");
    assert!(text.contains(&exe.digest.blake3.to_hex()), "{text}");
    assert!(text.contains(&exe.digest.sha256.to_hex()), "{text}");
    assert!(text.contains("\"tool\":\"leanchecker\""), "{text}");

    let other = executable(b"leanchecker 2");
    assert_eq!(
        store
            .cache
            .lookup(&all[0].key, &base_env(), &other)
            .expect("lookup"),
        Lookup::Miss(Miss::ExecutableChanged)
    );
    let (outcome, called) = run(&store, &base_env(), &other, &all);
    assert_eq!(called.len(), 5);
    assert_eq!(outcome.expect("other executable").reused.len(), 0);
    // The replay by the other executable replaced the entries.
    let (_, called) = run(&store, &base_env(), &other, &all);
    assert!(called.is_empty());
    let (_, called) = run(&store, &base_env(), &exe, &all);
    assert_eq!(called.len(), 5);
}

#[test]
fn only_a_replay_that_succeeded_is_recorded() {
    let store = store();
    let exe = executable(b"leanchecker 1");
    let all = units(&base_env(), &diamond());
    let outcome = replay_units(&store.cache, &base_env(), &exe, &all, |unit| {
        Ok(ReplayResult {
            exit_code: i32::from(unit.label == "left"),
            stdout: String::new(),
            stderr: "kernel: error".to_owned(),
        })
    });
    let diagnostic = outcome.expect_err("failing replay");
    assert_eq!(diagnostic.code.as_str(), "LLV7003");
    let failing = all.iter().find(|u| u.label == "left").expect("left");
    assert_eq!(
        store
            .cache
            .lookup(&failing.key, &base_env(), &exe)
            .expect("lookup"),
        Lookup::Miss(Miss::Absent)
    );
    // The replayer's own error ends the run and records nothing either.
    let store = self::store();
    let outcome = replay_units(&store.cache, &base_env(), &exe, &all, |unit| {
        Err(crate::diagnostic::Diagnostic::new(
            crate::code!("LLV7003"),
            unit.label.clone(),
        ))
    });
    assert!(outcome.is_err());
    assert_eq!(
        store
            .cache
            .lookup(&all[0].key, &base_env(), &exe)
            .expect("lookup"),
        Lookup::Miss(Miss::Absent)
    );
}

#[test]
fn a_changed_toolchain_or_mathlib_revision_reuses_nothing_and_leaves_old_entries_intact() {
    let store = store();
    let exe = executable(b"leanchecker 1");
    let first = units(&base_env(), &diamond());
    run(&store, &base_env(), &exe, &first).0.expect("first");
    let files_before = entry_files(&store);
    let bytes_before: Vec<Vec<u8>> = files_before
        .iter()
        .map(|p| std::fs::read(p).expect("read"))
        .collect();

    for other in [
        env("leanprover/lean4:v4.32.2", "0123456789abcdef"),
        env("leanprover/lean4:v4.32.1", "fedcba9876543210"),
    ] {
        let (outcome, called) = run(&store, &other, &exe, &units(&other, &diamond()));
        assert_eq!(called.len(), 5);
        assert_eq!(outcome.expect("other environment").reused.len(), 0);
    }
    assert_eq!(entry_files(&store).len(), 5 * 3);
    for (path, bytes) in files_before.iter().zip(&bytes_before) {
        assert_eq!(&std::fs::read(path).expect("read"), bytes, "{path:?}");
    }
    let (outcome, called) = run(&store, &base_env(), &exe, &first);
    assert!(called.is_empty());
    assert_eq!(outcome.expect("first again").reused.len(), 5);
}

fn entry_files(store: &Store) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    for shard in std::fs::read_dir(
        store
            .cache
            .entry_path(&DigestSet::of(b""))
            .parent()
            .unwrap()
            .parent()
            .unwrap(),
    )
    .expect("entries")
    {
        for file in std::fs::read_dir(shard.expect("shard").path()).expect("shard") {
            out.push(file.expect("file").path());
        }
    }
    out.sort();
    out
}

fn one_entry() -> (Store, ExecutableIdentity, ReplayUnit, std::path::PathBuf) {
    let store = store();
    let exe = executable(b"leanchecker 1");
    let unit = units(&base_env(), &diamond()).remove(0);
    run(&store, &base_env(), &exe, std::slice::from_ref(&unit))
        .0
        .expect("cold");
    let path = store.cache.entry_path(&unit.key).into_std_path_buf();
    (store, exe, unit, path)
}

fn assert_rejected_and_untouched(
    store: &Store,
    exe: &ExecutableIdentity,
    unit: &ReplayUnit,
    path: &std::path::Path,
) {
    let before = std::fs::read(path).expect("entry");
    let diagnostic = store
        .cache
        .lookup(&unit.key, &base_env(), exe)
        .expect_err("rejected");
    assert_eq!(
        diagnostic.code.as_str(),
        "LLV7021",
        "{}",
        diagnostic.message
    );
    // The replay that would follow does not run, and the file is as found.
    let (outcome, called) = run(store, &base_env(), exe, std::slice::from_ref(unit));
    assert_eq!(outcome.expect_err("rejected").code.as_str(), "LLV7021");
    assert!(called.is_empty());
    assert_eq!(std::fs::read(path).expect("entry"), before);
}

#[test]
fn an_entry_whose_bytes_changed_while_its_key_was_kept_is_rejected() {
    let (store, exe, unit, path) = one_entry();
    let text = std::fs::read_to_string(&path).expect("entry");
    assert!(text.contains("replayed base"));
    std::fs::write(&path, text.replace("replayed base", "replayed fake")).expect("tamper");
    assert_rejected_and_untouched(&store, &exe, &unit, &path);
}

#[test]
fn an_entry_whose_executable_digest_was_edited_is_rejected_not_missed() {
    let (store, exe, unit, path) = one_entry();
    let text = std::fs::read_to_string(&path).expect("entry");
    let other = executable(b"leanchecker 2");
    std::fs::write(
        &path,
        text.replace(&exe.digest.sha256.to_hex(), &other.digest.sha256.to_hex()),
    )
    .expect("tamper");
    assert_rejected_and_untouched(&store, &exe, &unit, &path);
}

#[test]
fn an_entry_stored_under_another_key_is_rejected() {
    let (store, exe, unit, path) = one_entry();
    let other = units(&base_env(), &diamond()).remove(4);
    let target = store.cache.entry_path(&other.key).into_std_path_buf();
    std::fs::create_dir_all(target.parent().unwrap()).expect("directory");
    std::fs::copy(&path, &target).expect("copy");
    let diagnostic = store
        .cache
        .lookup(&other.key, &base_env(), &exe)
        .expect_err("rejected");
    assert_eq!(diagnostic.code.as_str(), "LLV7021");
    assert!(
        diagnostic.message.contains(&unit.key.label()),
        "{}",
        diagnostic.message
    );
    assert!(
        diagnostic.message.contains(&other.key.label()),
        "{}",
        diagnostic.message
    );
}

#[test]
fn an_entry_that_does_not_parse_or_has_the_wrong_shape_is_rejected() {
    let (store, exe, unit, path) = one_entry();
    let good = std::fs::read_to_string(&path).expect("entry");
    let cases: Vec<(&str, String)> = vec![
        ("truncated", good[..good.len() / 2].to_owned()),
        ("empty", String::new()),
        ("not an object", "[]\n".to_owned()),
        (
            "no digest",
            good.replace("\"entry_digest\"", "\"entry_digezt\""),
        ),
        ("unknown member", good.replacen('{', "{\"extra\":1,", 1)),
    ];
    for (what, bytes) in cases {
        std::fs::write(&path, bytes).expect("tamper");
        let diagnostic = store
            .cache
            .lookup(&unit.key, &base_env(), &exe)
            .expect_err(what);
        assert_eq!(
            diagnostic.code.as_str(),
            "LLV7021",
            "{what}: {}",
            diagnostic.message
        );
    }
}

#[test]
fn an_entry_that_is_not_a_regular_file_is_rejected() {
    let (store, exe, unit, path) = one_entry();
    std::fs::remove_file(&path).expect("remove");
    std::fs::create_dir(&path).expect("directory");
    let diagnostic = store
        .cache
        .lookup(&unit.key, &base_env(), &exe)
        .expect_err("rejected");
    assert_eq!(diagnostic.code.as_str(), "LLV7021");
}

#[test]
fn an_entry_whose_environment_is_not_the_one_its_key_covers_is_rejected() {
    // A self-consistent forgery: the digest is recomputed, the key is kept,
    // and the environment is not the caller's. The key still names it.
    let (store, exe, unit, path) = one_entry();
    let text = std::fs::read_to_string(&path).expect("entry");
    let forged = CacheEntry {
        key: unit.key,
        granularity: Granularity::Declaration,
        label: "base".to_owned(),
        environment: env("leanprover/lean4:v9", "0123456789abcdef"),
        executable: exe.clone(),
        result: ReplayResult {
            exit_code: 0,
            stdout: "replayed base\n".to_owned(),
            stderr: String::new(),
        },
    };
    std::fs::write(&path, forged.to_file_bytes()).expect("forge");
    assert_ne!(text.into_bytes(), forged.to_file_bytes());
    let diagnostic = store
        .cache
        .lookup(&unit.key, &base_env(), &exe)
        .expect_err("rejected");
    assert_eq!(diagnostic.code.as_str(), "LLV7021");
    assert!(
        diagnostic.message.contains("environment"),
        "{}",
        diagnostic.message
    );
}

#[test]
fn an_entry_records_a_zero_exit_only() {
    let (store, exe, unit, path) = one_entry();
    let forged = CacheEntry {
        key: unit.key,
        granularity: Granularity::Declaration,
        label: "base".to_owned(),
        environment: base_env(),
        executable: exe.clone(),
        result: ReplayResult {
            exit_code: 1,
            stdout: String::new(),
            stderr: String::new(),
        },
    };
    std::fs::write(&path, forged.to_file_bytes()).expect("forge");
    let diagnostic = store
        .cache
        .lookup(&unit.key, &base_env(), &exe)
        .expect_err("rejected");
    assert_eq!(diagnostic.code.as_str(), "LLV7021");
}

#[test]
fn an_executable_is_hashed_both_ways_from_its_bytes() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = Utf8PathBuf::from_path_buf(dir.path().join("tool")).expect("utf-8");
    std::fs::write(&path, b"abc").expect("write");
    let identity = executable_identity("leanchecker", &path).expect("identity");
    assert_eq!(identity.digest, DigestSet::of(b"abc"));
    assert_eq!(identity.tool, "leanchecker");
    let missing = executable_identity("leanchecker", &path.join("nope")).expect_err("missing");
    assert_eq!(missing.code.as_str(), "LLV7010");
}
