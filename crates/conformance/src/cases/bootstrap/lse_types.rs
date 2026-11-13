//! `BS-52` (SPEC.md §17.23, §13.6 to §13.8, §14.4): the conservative checker
//! of LexLean Semantic Expressions as language-1.2 source,
//! `LexLeanBoot.LseTypes`, held to the byte to `lexicon::lse::check_signature`
//! and `check_value`: the inferred type and the type of every natural literal,
//! or the message of the first error, with the oracle's metavariables and
//! capture-avoiding renames.
//!
//! What the oracle's lookup closure answers is handed over as a table of rows
//! and the answer for a constant no row names (`Missing` in the closure,
//! `Opaque` in the package-local check). The checker is structural recursion
//! over a bound on its nesting the host passes, so both roots admit
//! `recursion`. Each planted defect names the one vector that catches it.

use std::collections::{BTreeMap, BTreeSet};

use lexlean::calculus::Value;
use lexlean::lexicon::entry::Denotation;
use lexlean::lexicon::lse::{
    check_signature, check_value, parse, Checked, ConstInfo, Lse, QualifiedId, TypeError,
};
use lexlean::lexicon::package::{load_package, LexiconPackage, LoadContext};
use serde_json::Value as Json2;

use super::lexicon_lse::{drawn_lse, enc_lse};
use super::rust_validate::{eligibility_of, rustc_differential};
use super::{list, option, text, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.LseTypes";
const ROOTS: [(&str, &[&str]); 2] = [
    ("checkSignature", &["allocation", "overflow", "recursion"]),
    ("checkValue", &["allocation", "overflow", "recursion"]),
];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// The depth the expressions are read at, and the bound on the checker's
/// nesting the host passes.
const DEPTH: u64 = 64;
const NESTING: u64 = 192;
/// Seeded expressions under the evaluator, and under rustc.
const EVALUATOR_SEEDED: usize = 200;
const RUSTC_SEEDED: usize = 1_500;

const MISSING: u64 = 0;
const OPAQUE: u64 = 1;

// ---------------------------------------------------------------- the table

/// One row of the lookup table: what the oracle's closure answers for one
/// constant.
#[derive(Clone)]
struct Row {
    id: QualifiedId,
    signature: Option<Lse>,
    defined: bool,
}

#[derive(Clone)]
struct Table {
    rows: Vec<Row>,
    fallback: u64,
}

impl Table {
    fn info(&self, id: &QualifiedId) -> ConstInfo<'_> {
        match self.rows.iter().find(|row| row.id == *id) {
            Some(row) => match &row.signature {
                Some(signature) => ConstInfo::Signature {
                    signature,
                    defined: row.defined,
                },
                None => ConstInfo::NoSignature,
            },
            None if self.fallback == OPAQUE => ConstInfo::Opaque,
            None => ConstInfo::Missing,
        }
    }

    /// The rows the checker can consult for these expressions: the
    /// constants they reference and, through the signatures of those, every
    /// constant reachable from them. No other row is ever looked up.
    fn reachable(&self, roots: &[&Lse]) -> Vec<&Row> {
        let mut wanted: Vec<QualifiedId> = roots.iter().flat_map(|root| root.referenced_consts()).collect();
        let mut seen: BTreeSet<String> = BTreeSet::new();
        let mut out = Vec::new();
        while let Some(id) = wanted.pop() {
            if !seen.insert(id.to_string()) {
                continue;
            }
            if let Some(row) = self.rows.iter().find(|row| row.id == id) {
                if let Some(signature) = &row.signature {
                    wanted.extend(signature.referenced_consts());
                }
                out.push(row);
            }
        }
        out.sort_by_key(|row| row.id.to_string());
        out
    }

    fn value(&self, roots: &[&Lse]) -> (Value, Value) {
        let rows = self
            .reachable(roots)
            .into_iter()
            .map(|row| Value::Adt {
                constructor: 0,
                fields: vec![
                    text(&row.id.package),
                    text(&row.id.entry),
                    nat(if row.signature.is_some() { 3 } else { 2 }),
                    enc_lse(row.signature.as_ref().unwrap_or(&Lse::SortProp)),
                    Value::Bool { value: row.defined },
                ],
            })
            .collect();
        (list(rows), nat(self.fallback))
    }
}

fn nat(value: u64) -> Value {
    Value::Nat {
        value: value.to_string(),
    }
}

fn outcome(result: Result<Checked, TypeError>) -> Value {
    match result {
        Ok(checked) => Value::Ok {
            value: Box::new(Value::Adt {
                constructor: 0,
                fields: vec![
                    enc_lse(&checked.ty),
                    list(
                        checked
                            .literal_types
                            .iter()
                            .map(|literal| option(literal.as_ref().map(enc_lse)))
                            .collect(),
                    ),
                ],
            }),
        },
        Err(error) => Value::Error {
            value: Box::new(text(&error.message)),
        },
    }
}

fn signature_run(signature: &Lse, table: &Table) -> Run {
    let lookup = |id: &QualifiedId| table.info(id);
    let (rows, fallback) = table.value(&[signature]);
    (
        vec![enc_lse(signature), rows, fallback, nat(NESTING)],
        serde_json::to_value(outcome(check_signature(signature, &lookup))).expect("a value is JSON"),
    )
}

fn value_run(value: &Lse, signature: Option<&Lse>, table: &Table) -> Run {
    let lookup = |id: &QualifiedId| table.info(id);
    let mut roots = vec![value];
    roots.extend(signature);
    let (rows, fallback) = table.value(&roots);
    (
        vec![
            enc_lse(value),
            option(signature.map(enc_lse)),
            rows,
            fallback,
            nat(NESTING),
        ],
        serde_json::to_value(outcome(check_value(value, signature, &lookup)))
            .expect("a value is JSON"),
    )
}

fn parsed(source: &str) -> Lse {
    parse(source, DEPTH).unwrap_or_else(|error| panic!("`{source}` parses: {error}"))
}

fn q(text: &str) -> QualifiedId {
    QualifiedId::parse(text).expect("a qualified ID")
}

/// The hand-built glossary `t` the named vectors are checked against.
fn glossary(fallback: u64) -> Table {
    let rows = [
        ("t::nat", Some("(sort (type 0))"), false),
        ("t::zero", Some("(const t::nat)"), false),
        ("t::succ", Some("(pi ((explicit n (const t::nat))) (const t::nat))"), false),
        ("t::add", Some("(pi ((explicit a (const t::nat)) (explicit b (const t::nat))) (const t::nat))"), false),
        ("t::eq", Some("(pi ((implicit A (sort (type 0))) (explicit a (local A)) (explicit b (local A))) (sort prop))"), false),
        ("t::double", Some("(pi ((explicit n (const t::nat))) (const t::nat))"), true),
        ("t::nattype", Some("(sort (type 0))"), true),
        ("t::mk", Some("(pi ((explicit x (const t::nat))) (const t::nattype))"), false),
        ("t::word", None, false),
        ("t::list", Some("(pi ((explicit A (sort (type 0)))) (sort (type 0)))"), false),
        ("t::pair", Some("(pi ((explicit A (sort (type 0))) (explicit B (sort (type 0)))) (sort (type 0)))"), false),
        ("t::prop", Some("(sort prop)"), false),
        ("t::k", Some("(pi ((explicit y (const t::nat))) (let x (const t::nat) (local y) (app (const t::eq) (local x) (local y))))"), false),
    ];
    Table {
        rows: rows
            .iter()
            .map(|(id, signature, defined)| Row {
                id: q(id),
                signature: signature.map(parsed),
                defined: *defined,
            })
            .collect(),
        fallback,
    }
}

// ------------------------------------------------------------------ vectors

struct Vector {
    label: &'static str,
    root: usize,
    run: Run,
}

#[allow(clippy::too_many_lines)]
fn named() -> Vec<Vector> {
    let closed = glossary(MISSING);
    let open = glossary(OPAQUE);
    let mut out = Vec::new();
    let signatures: &[(&'static str, &str, bool)] = &[
        ("a sort", "(sort prop)", false),
        ("a constant type", "(const t::nat)", false),
        ("a pi of types", "(pi ((explicit a (const t::nat))) (const t::nat))", false),
        ("an unresolved constant", "(const t::missing)", false),
        ("a constant without a signature", "(const t::word)", false),
        ("a term is no type", "(const t::zero)", false),
        ("a natural literal is a term", "(nat 3)", false),
        ("a lam is a term", "(lam ((explicit x (sort prop))) (local x))", false),
        ("a local out of scope", "(local x)", false),
        ("an application of a sort", "(app (sort prop) (const t::zero))", false),
        ("an application of a literal", "(app (nat 1) (const t::zero))", false),
        ("two arguments too many", "(app (const t::succ) (const t::zero) (const t::zero))", false),
        ("one argument too many", "(app (const t::zero) (const t::zero))", false),
        ("an argument of the wrong type", "(app (const t::succ) (sort prop))", false),
        ("a partial application", "(app (const t::add) (const t::zero))", false),
        ("an equation is a proposition", "(app (const t::eq) (const t::zero) (const t::zero))", false),
        ("a literal with an inferred type", "(app (const t::eq) (const t::zero) (nat 1))", false),
        ("literals of an undetermined type", "(app (const t::eq) (nat 1) (nat 2))", false),
        ("an opaque head", "(app (const other::f) (const t::zero))", true),
        ("a defined result type unfolds", "(app (const t::mk) (const t::zero) (const t::zero))", false),
        ("a let", "(let x (sort prop) (const t::prop) (local x))", false),
        ("a let whose value disagrees", "(let x (const t::nat) (sort prop) (local x))", false),
        ("a binder that is no type", "(pi ((explicit a (const t::zero))) (sort prop))", false),
        ("a body that is no type", "(pi ((explicit a (const t::nat))) (local a))", false),
        ("a type of a list", "(app (const t::list) (const t::nat))", false),
        ("a sort of a universe", "(sort (type (max u 1)))", false),
    ];
    for &(label, source, opaque) in signatures {
        let table = if opaque { &open } else { &closed };
        out.push(Vector {
            label,
            root: 0,
            run: signature_run(&parsed(source), table),
        });
    }
    let values: &[(&'static str, &str, Option<&str>)] = &[
        ("a value matching its signature", "(lam ((explicit n (const t::nat))) (app (const t::succ) (local n)))", Some("(pi ((explicit m (const t::nat))) (const t::nat))")),
        ("a value that is no function", "(const t::zero)", Some("(pi ((explicit m (const t::nat))) (const t::nat))")),
        ("a value of more binders", "(lam ((explicit a (const t::nat)) (explicit b (const t::nat))) (local a))", Some("(pi ((explicit m (const t::nat))) (const t::nat))")),
        ("a value of another binder mode", "(lam ((implicit n (const t::nat))) (local n))", Some("(pi ((explicit n (const t::nat))) (const t::nat))")),
        ("a value without a signature", "(app (const t::succ) (const t::zero))", None),
        ("a value of an unresolved head", "(app (const t::nope) (nat 1))", None),
        ("a renamed binder avoids capture", "(lam ((explicit c (sort (type 0))) (explicit b (pi ((implicit a (sort prop))) (local c)))) (sort prop))", Some("(pi ((explicit a (sort (type 0))) (explicit b (pi ((explicit c (sort prop))) (local a)))) (sort (type 1)))")),
        ("a renamed let avoids capture", "(lam ((explicit x (const t::nat))) (app (const t::k) (local x)))", None),
        ("applications of different arity", "(lam ((explicit x (app (const t::list) (const t::nat)))) (sort prop))", Some("(pi ((explicit x (app (const t::pair) (const t::nat) (const t::nat)))) (sort (type 1)))")),
        ("a value whose binders are named apart", "(lam ((explicit n (sort (type 0))) (explicit z (local n))) (local z))", Some("(pi ((explicit m (sort (type 0))) (explicit y (local m))) (local m))")),
        ("a value matched by a defined signature", "(const t::zero)", Some("(const t::nattype)")),
        ("a literal value", "(nat 7)", Some("(const t::nat)")),
        ("a proposition value", "(app (const t::eq) (const t::zero) (const t::zero))", Some("(sort prop)")),
    ];
    for &(label, value, signature) in values {
        let signature = signature.map(parsed);
        out.push(Vector {
            label,
            root: 1,
            run: value_run(&parsed(value), signature.as_ref(), &closed),
        });
    }
    out
}

// --------------------------------------------------------------- committed

/// Every lexicon package the repository commits that loads: the builtin
/// packages and every directory with a `lexicon.toml`.
fn committed_packages() -> Vec<LexiconPackage> {
    let bootstrap = lexlean::lexicon::load_bootstrap().expect("the bootstrap loads");
    let mut out: Vec<LexiconPackage> = Vec::new();
    for language in ["1.0", "1.1", "1.2"] {
        let Ok(bootstrap) = lexlean::lexicon::load_bootstrap_for(language) else {
            continue;
        };
        let ctx = LoadContext {
            language,
            forbidden_controls: &bootstrap.structural.forbidden_controls,
            max_scope_depth: DEPTH,
        };
        for row in &bootstrap.builtin_packages {
            if let Ok(package) = lexlean::lexicon::load_builtin_package(row, &ctx) {
                out.push(package);
            }
        }
    }
    let root = repo_root();
    let walker = walkdir::WalkDir::new(root.as_std_path())
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|entry| {
            let name = entry.file_name().to_string_lossy();
            !(entry.file_type().is_dir()
                && (matches!(name.as_ref(), "target" | ".git" | ".lexlean" | "language")
                    || support::holds_cache_tag(entry)))
        });
    for entry in walker.flatten() {
        if entry.file_name() != "lexicon.toml" {
            continue;
        }
        let Some(directory) = entry.path().parent() else {
            continue;
        };
        let Ok(manifest) = std::fs::read_to_string(entry.path()) else {
            continue;
        };
        let Some(language) = manifest
            .parse::<toml::Table>()
            .ok()
            .and_then(|table| table.get("language").and_then(|v| v.as_str().map(str::to_owned)))
        else {
            continue;
        };
        let mut files: Vec<(String, Vec<u8>)> = walkdir::WalkDir::new(directory)
            .into_iter()
            .flatten()
            .filter(|file| file.file_type().is_file())
            .filter_map(|file| {
                let relative = file
                    .path()
                    .strip_prefix(directory)
                    .ok()?
                    .to_string_lossy()
                    .replace('\\', "/");
                (relative == "lexicon.toml" || relative.starts_with("entries/"))
                    .then(|| std::fs::read(file.path()).ok().map(|bytes| (relative, bytes)))
                    .flatten()
            })
            .collect();
        files.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));
        let ctx = LoadContext {
            language: &language,
            forbidden_controls: &bootstrap.structural.forbidden_controls,
            max_scope_depth: DEPTH,
        };
        if let Ok(package) = load_package("committed", &files, None, &ctx) {
            out.push(package);
        }
    }
    out
}

fn rows_of(package: &LexiconPackage) -> Vec<Row> {
    package
        .entries
        .values()
        .map(|entry| Row {
            id: QualifiedId {
                package: package.id.clone(),
                entry: entry.id.clone(),
            },
            signature: entry.signature.clone(),
            defined: matches!(entry.denotation, Denotation::Defined { .. }),
        })
        .collect()
}

/// The runs of every committed signature and defined value: against its own
/// package with every other constant opaque, as a package loads, and against
/// the package and the builtin packages with every other constant missing.
fn committed_runs() -> (Vec<Run>, Vec<Run>, usize) {
    let packages = committed_packages();
    let builtin_ids: BTreeSet<String> = lexlean::lexicon::load_bootstrap()
        .expect("the bootstrap loads")
        .builtin_packages
        .iter()
        .map(|row| row.id.clone())
        .collect();
    let builtin_rows: Vec<Row> = packages
        .iter()
        .filter(|package| builtin_ids.contains(&package.id))
        .flat_map(rows_of)
        .collect();
    let mut signatures = Vec::new();
    let mut values = Vec::new();
    let mut seen = BTreeSet::new();
    for package in &packages {
        let local = Table {
            rows: rows_of(package),
            fallback: OPAQUE,
        };
        let mut wide_rows = rows_of(package);
        for row in &builtin_rows {
            if row.id.package != package.id {
                wide_rows.push(row.clone());
            }
        }
        let wide = Table {
            rows: wide_rows,
            fallback: MISSING,
        };
        for entry in package.entries.values() {
            let key = format!("{}::{}", package.id, entry.id);
            if !seen.insert((key, entry.signature.as_ref().map(|s| s.print(false)))) {
                continue;
            }
            if let Some(signature) = &entry.signature {
                signatures.push(signature_run(signature, &local));
                signatures.push(signature_run(signature, &wide));
            }
            if let Denotation::Defined { value, .. } = &entry.denotation {
                values.push(value_run(value, entry.signature.as_ref(), &local));
                values.push(value_run(value, entry.signature.as_ref(), &wide));
            }
        }
    }
    (signatures, values, packages.len())
}

/// `count` seeded expressions, each checked as a signature and as a value
/// against a signature of its own, under the hand-built glossary.
fn seeded_runs(count: usize) -> (Vec<Run>, Vec<Run>) {
    let mut seeded = Seeded(0x0B55_0052);
    let tables = [glossary(MISSING), glossary(OPAQUE)];
    let mut signatures = Vec::with_capacity(count);
    let mut values = Vec::with_capacity(count);
    for _ in 0..count {
        let table = seeded.pick(&tables);
        let depth = 1 + seeded.below(3);
        let expression = drawn_lse(&mut seeded, depth);
        signatures.push(signature_run(&expression, table));
        let against = if seeded.chance(70) {
            {
            let depth = 1 + seeded.below(3);
            Some(drawn_lse(&mut seeded, depth))
        }
        } else {
            None
        };
        values.push(value_run(&expression, against.as_ref(), table));
    }
    (signatures, values)
}

// ------------------------------------------------------------------- plants

struct Plant {
    label: &'static str,
    root: usize,
    site: &'static str,
    replacement: &'static str,
    vector: &'static str,
}

const PLANTS: &[Plant] = &[
    Plant {
        label: "the actual side's metavariable is not solved",
        root: 0,
        site: "| Option.some m => (urOk (ckSolve st2 m e))",
        replacement: "| Option.some m => (urOk st2)",
        vector: "a literal with an inferred type",
    },
    Plant {
        label: "a sort of a universe is no type",
        root: 0,
        site: "(if ((kindOf z) <? 2) then (irOk z st2)",
        replacement: "(if ((kindOf z) <? 1) then (irOk z st2)",
        vector: "a pi of types",
    },
    Plant {
        label: "a lam is a type",
        root: 0,
        site: "(if (if (ke =? 8) then true else (ke =? 6)) then (let txtExpr",
        replacement: "(if (ke =? 8) then (let txtExpr",
        vector: "a lam is a term",
    },
    Plant {
        label: "binder modes are not compared",
        root: 1,
        site: "(if ((project be mode) =? (project ba mode)) then",
        replacement: "(if true then",
        vector: "a value of another binder mode",
    },
    Plant {
        label: "binder counts are not compared",
        root: 1,
        site: "(if (ne =? na) then (unifyPiLoop",
        replacement: "(if true then (unifyPiLoop",
        vector: "a value of more binders",
    },
    Plant {
        label: "an extra argument is accepted",
        root: 0,
        site: "(if (mayUnfold f st z) then (inferRest f st remaining scope) else",
        replacement: "(if true then (inferRest f st remaining scope) else",
        vector: "two arguments too many",
    },
    Plant {
        label: "one argument is plural",
        root: 0,
        site: "(if (count =? 1) then 0x else (lit \"s\"))",
        replacement: "(lit \"s\")",
        vector: "one argument too many",
    },
    Plant {
        label: "a binder is not renamed apart",
        root: 1,
        site: "(if (boundIn avoid n) then (let named",
        replacement: "(if false then (let named",
        vector: "a renamed binder avoids capture",
    },
    Plant {
        label: "a let is not renamed apart",
        root: 1,
        site: "(if (boundIn avoid bound) then",
        replacement: "(if false then",
        vector: "a renamed let avoids capture",
    },
    Plant {
        label: "an implicit binder takes an explicit argument",
        root: 0,
        site: "(if ((project binder mode) =? 0) then (match remaining",
        replacement: "(if true then (match remaining",
        vector: "an equation is a proposition",
    },
    Plant {
        label: "a literal is not recorded",
        root: 0,
        site: "| LexiconLse::Lse.LNat digits => (let made : Sr := (literalMeta st);",
        replacement: "| LexiconLse::Lse.LNat digits => (let made : Sr := (freshMeta st);",
        vector: "a literal with an inferred type",
    },
    Plant {
        label: "a defined constant does not unfold",
        root: 0,
        site: "(if ((first found) =? 3) then (second found) else false)",
        replacement: "false",
        vector: "a defined result type unfolds",
    },
    Plant {
        label: "a constant no row names is missing",
        root: 0,
        site: "| Option.none => ((project st fallback), false)),",
        replacement: "| Option.none => (0, false)),",
        vector: "an opaque head",
    },
    Plant {
        label: "a lam's type forgets its binders",
        root: 1,
        site: "(irOk (ctor LexiconLse::Lse.LPi binders (project inner ty)) (project inner ck))",
        replacement: "(irOk (project inner ty) (project inner ck))",
        vector: "a value matching its signature",
    },
    Plant {
        label: "a let's body does not see its name",
        root: 0,
        site: "(infer f (project agreed ck) body (cons (n, t) scope))",
        replacement: "(infer f (project agreed ck) body scope)",
        vector: "a let",
    },
    Plant {
        label: "a partial application drops the next binder",
        root: 0,
        site: "| List.nil => (irOk (ctor LexiconLse::Lse.LPi (chainOf binders) body) st)",
        replacement: "| List.nil => (irOk (ctor LexiconLse::Lse.LPi (chainOf rest) body) st)",
        vector: "a partial application",
    },
    Plant {
        label: "zonking forgets solutions",
        root: 0,
        site: "| Option.some solution => (zonk f metas solution)",
        replacement: "| Option.some solution => (ctor Option.some [LexiconLse::Lse] (ctor LexiconLse::Lse.LLocal name))",
        vector: "a literal with an inferred type",
    },
    Plant {
        label: "applications of different arity unify their heads",
        root: 1,
        site: "(if ((prim length (chainItems fuel xs) : Nat) =? (prim length (chainItems fuel ys) : Nat)) then 1 else 3)",
        replacement: "1",
        vector: "applications of different arity",
    },
    Plant {
        label: "the actual binder keeps its name",
        root: 1,
        site: "(let renamed : LexiconLse::Lse := (ctor LexiconLse::Lse.LLocal (project be name));",
        replacement: "(let renamed : LexiconLse::Lse := (ctor LexiconLse::Lse.LLocal (project ba name));",
        vector: "a value whose binders are named apart",
    },
    Plant {
        label: "a constant without a signature resolves",
        root: 0,
        site: "(if (k =? 2) then (irFail st",
        replacement: "(if false then (irFail st",
        vector: "a constant without a signature",
    },
];

fn plants(vectors: &[Vector]) {
    let mut missed = Vec::new();
    for plant in PLANTS {
        let vector = vectors
            .iter()
            .find(|vector| vector.label == plant.vector)
            .unwrap_or_else(|| panic!("{}: no vector `{}`", plant.label, plant.vector));
        assert_eq!(
            vector.root, plant.root,
            "{}: the vector is a run of the root planted",
            plant.label
        );
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/LseTypes.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/LseTypes.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[plant.root].0);
        match super::evaluate(&low, std::slice::from_ref(&vector.run), 1).first() {
            Some(first) => eprintln!(
                "BS-52: plant detected: {}: {}",
                plant.label,
                first.chars().take(160).collect::<String>()
            ),
            None => missed.push(plant.label),
        }
    }
    assert!(
        missed.is_empty(),
        "planted defects their own vector does not detect: {missed:?}"
    );
}

fn export(lowered: &BTreeMap<&str, super::Lowered>) {
    let exported = harness::exported("bootstrap");
    let bundle = exported.bundle();
    let mut realized = 0usize;
    for exported_root in &exported.output.result.roots {
        let Some((member, low)) = exported_root
            .root
            .strip_prefix(&format!("{MODULE}."))
            .and_then(|member| lowered.get(member).map(|low| (member, low)))
        else {
            continue;
        };
        let largest = exported_root
            .outputs
            .iter()
            .flat_map(|output| &output.certification.certificates)
            .map(|certificate| certificate.file.byte_length)
            .max()
            .expect("a certificate");
        eprintln!(
            "BS-52: {}: largest certificate part {largest} bytes",
            exported_root.root
        );
        assert!(
            largest <= CERTIFICATE_BUDGET,
            "{}: a certificate part of {largest} bytes is beyond {CERTIFICATE_BUDGET}",
            exported_root.root
        );
        for output in &exported_root.outputs {
            let package: Json2 = serde_json::from_slice(
                &std::fs::read(
                    bundle
                        .join(&output.package.directory)
                        .join("package.json")
                        .as_std_path(),
                )
                .expect("package.json"),
            )
            .expect("package JSON");
            assert_eq!(
                package["program"],
                serde_json::to_value(&low.program).expect("a program is JSON"),
                "{member}: the exported package realizes the program the differential ran"
            );
            realized += 1;
        }
    }
    assert_eq!(realized, ROOTS.len(), "one certified package per root");
    let packages = harness::tree(&bundle.join("packages"));
    let committed_packages = harness::tree(&harness::fixture_dir("bootstrap").join("packages"));
    assert_eq!(
        packages.keys().collect::<Vec<_>>(),
        committed_packages.keys().collect::<Vec<_>>(),
        "the exported package files are the committed ones"
    );
    for (relative, content) in &packages {
        assert!(
            committed_packages.get(relative) == Some(content),
            "packages/{relative} differs from the committed package"
        );
    }
}

/// Every error the checker can state, by a part of its message (R2).
const REFUSALS: &[&str] = &[
    "does not resolve",
    "has no signature and cannot appear in an expression",
    "is not in scope",
    "which is not a function",
    "explicit argument but",
    "explicit arguments but",
    "is a term, not a type",
    "is not a type: its type is",
    "arguments but",
    "binder `",
    "expected `",
    "does not match the signature",
];

#[allow(clippy::too_many_lines)]
pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("LseTypes")
            && checked_units.units.contains_key("LexiconLse"),
        "LseTypes and LexiconLse are linked modules of the project"
    );
    eligibility_of(&project, "LseTypes", MODULE, &ROOTS);

    let vectors = named();
    let labels: BTreeSet<&str> = vectors.iter().map(|vector| vector.label).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let mut by_root: [Vec<Run>; 2] = Default::default();
    for vector in &vectors {
        by_root[vector.root].push(vector.run.clone());
    }
    let outputs: Vec<String> = vectors.iter().map(|vector| vector.run.1.to_string()).collect();
    for refusal in REFUSALS {
        let needle = super::hex(refusal.as_bytes());
        assert!(
            outputs.iter().any(|output| output.contains(&needle)),
            "no vector reaches the error `{refusal}`"
        );
    }

    let (committed_signatures, committed_values, package_count) = committed_runs();
    assert!(
        committed_signatures.len() >= 200,
        "the committed packages carry {} signature runs",
        committed_signatures.len()
    );
    let normative = support::normative_host();
    let (evaluator_seeded, rustc_seeded) = if normative {
        (seeded_runs(EVALUATOR_SEEDED), seeded_runs(RUSTC_SEEDED))
    } else {
        Default::default()
    };
    eprintln!(
        "BS-52: {} vectors, {} committed packages with {} signature and {} value runs, {} and {} seeded expressions under the evaluator and rustc ({:?})",
        vectors.len(),
        package_count,
        committed_signatures.len(),
        committed_values.len(),
        evaluator_seeded.0.len(),
        rustc_seeded.0.len(),
        started.elapsed()
    );

    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, super::Lowered> = ROOTS
        .iter()
        .map(|(root, _)| (*root, super::lower(&checked, &project, MODULE, root)))
        .collect();
    let mut committed = [committed_signatures, committed_values];
    let evaluator_seeded = [evaluator_seeded.0, evaluator_seeded.1];
    let rustc_seeded = [rustc_seeded.0, rustc_seeded.1];
    if normative {
        for (index, (root, _)) in ROOTS.iter().enumerate() {
            let mut runs: Vec<Run> = by_root[index].clone();
            runs.extend(committed[index].iter().cloned());
            runs.extend(evaluator_seeded[index].iter().cloned());
            let disagreements = super::evaluate(&lowered[root], &runs, 5);
            assert!(
                disagreements.is_empty(),
                "{root} and the Rust oracle disagree: {disagreements:?}"
            );
            eprintln!(
                "BS-52: {root}: the evaluator agrees on {} runs ({:?})",
                runs.len(),
                started.elapsed()
            );
        }
        plants(&vectors);
    } else {
        eprintln!("BS-52: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut compiled: [Vec<Run>; 2] = Default::default();
    for (index, runs) in compiled.iter_mut().enumerate() {
        runs.append(&mut by_root[index]);
        runs.append(&mut committed[index]);
        runs.extend(rustc_seeded[index].iter().cloned());
    }
    let suites: Vec<(&str, &super::Lowered, &[Run])> = ROOTS
        .iter()
        .enumerate()
        .map(|(index, (root, _))| (*root, &lowered[root], compiled[index].as_slice()))
        .collect();
    let total = rustc_differential("BS-52", &suites, "oracle");
    eprintln!("BS-52: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-52") {
        return;
    }
    export(&lowered);
    eprintln!("BS-52: passed in {:?}", started.elapsed());
}
