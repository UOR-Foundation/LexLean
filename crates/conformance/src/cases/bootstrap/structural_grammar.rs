//! `BS-21` (SPEC.md §17.23, §15.1, §15.2): the fixed structural grammar as
//! language-1.2 source, `LexLeanBoot.StructuralGrammar`, held to the byte to
//! `lexlean::grammar::structural::parse_module`.
//!
//! The root reads the atoms `scanSource` of `BS-03` returns, so the host
//! composes the two. It parses by a machine with an explicit stack under
//! `iterate_until`, so it admits no `recursion` and owes no stack bound.
//!
//! Each planted defect names the one vector that catches it. `BS-05` and
//! `BS-11` each lost a plant when their corpus grew, because a plant caught
//! by "some run of the corpus" is only as good as that corpus: here a change
//! to the seeded corpus cannot cost a plant its witness.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use lexlean::calculus::Value;
use lexlean::grammar::structural::{
    parse_module, AtomRange, BlockAst, BraceArg, CalcStepAst, CaseAst, DeclAst, ModuleAst,
    PolicyAst, PolicyKind, ProofEnvAst, ProofItemAst, SentenceAst,
};
use lexlean::ir::declaration::DeclKind;
use lexlean::source::atom::{Atom, AtomClass};
use lexlean::source::coverage::Origin;
use serde_json::Value as Json;

use super::rust_validate::{eligibility_of, rustc_differential};
use super::scan::{atom_value, class_index, rejection_value};
use super::{list, nat_val, option, pair, text, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.StructuralGrammar";
const SOURCE: &str = "bootstrap/src/StructuralGrammar.lex.tex";
const ROOTS: [(&str, &[&str]); 1] = [("parseModule", &["allocation", "overflow"])];

/// The most a part of a certificate of the root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// The limit every vector that does not name its own runs under: the
/// `max_scope_depth` of the committed projects.
const DEPTH: u64 = 1_024;
/// The atoms of a committed module the evaluator reads whole, and the atoms a
/// longer one gives it, half from each end: the two ends of a data-form module
/// are a module the oracle accepts, whose notation runs to its `\end`.
const EVALUATOR_ATOMS: (usize, usize) = (1_500, 128);
/// The same for rustc, which runs the parser in a fraction of the evaluator's
/// time, so it reads whole modules the evaluator gets only the ends of.
const RUSTC_ATOMS: (usize, usize) = (4_096, 128);
/// The named vectors, the ones the oracle refuses, and the planted defects, so
/// that a vector or a plant dropped in an edit fails here.
const VECTORS: usize = 267;
const REFUSED: usize = 146;
const PLANT_COUNT: usize = 44;
/// Seeded edits of the vectors and the small committed modules.
const MUTANTS: usize = 2_000;
/// The largest committed module an edit starts from, in bytes.
const MUTANT_BASE: usize = 1_024;

// ---------------------------------------------------------------- the sorts

/// The constructors of each inductive of the module, in declaration order,
/// read from its text, so that the encoders name a constructor and never
/// number it, and one moved in the module fails here.
fn sorts() -> &'static BTreeMap<String, Vec<String>> {
    static SORTS: OnceLock<BTreeMap<String, Vec<String>>> = OnceLock::new();
    SORTS.get_or_init(|| {
        let source =
            std::fs::read_to_string(repo_root().join(SOURCE).as_std_path()).expect("the module");
        let mut out = BTreeMap::new();
        for block in source.split("inductive{").skip(1) {
            let block = block.split("\n}\n").next().expect("a block");
            let name = block
                .split("name: ")
                .nth(1)
                .and_then(|rest| rest.split(',').next())
                .expect("a sort name")
                .trim()
                .to_owned();
            let constructors: Vec<String> = block
                .split("{name: ")
                .skip(1)
                .map(|rest| {
                    rest.split(',')
                        .next()
                        .expect("a constructor")
                        .trim()
                        .to_owned()
                })
                .collect();
            out.insert(name, constructors);
        }
        out
    })
}

fn adt(sort: &str, constructor: &str, fields: Vec<Value>) -> Value {
    let found = sorts()
        .get(sort)
        .unwrap_or_else(|| panic!("the module has no sort `{sort}`"))
        .iter()
        .position(|name| name == constructor)
        .unwrap_or_else(|| panic!("`{sort}` has no constructor `{constructor}`"));
    Value::Adt {
        constructor: u64::try_from(found).expect("an index"),
        fields,
    }
}

/// A record: a structure is the one constructor of its own sort.
fn record(fields: Vec<Value>) -> Value {
    Value::Adt {
        constructor: 0,
        fields,
    }
}

// ------------------------------------------------------------- the encoders

fn number(value: u64) -> Value {
    Value::Nat {
        value: value.to_string(),
    }
}

fn boolean(value: bool) -> Value {
    Value::Bool { value }
}

fn enc_arg(arg: &BraceArg) -> Value {
    record(vec![
        nat_val(arg.range.0),
        nat_val(arg.range.1),
        text(&arg.text),
    ])
}

fn enc_args(args: &[BraceArg]) -> Value {
    list(args.iter().map(enc_arg).collect())
}

fn enc_sentence(sentence: &SentenceAst) -> Value {
    assert_eq!(
        sentence.range.1, sentence.period,
        "a sentence's content ends at its period"
    );
    record(vec![nat_val(sentence.range.0), nat_val(sentence.period)])
}

fn enc_policy(policy: &PolicyAst) -> Value {
    let kind = match policy.kind {
        PolicyKind::None => "PolicyNone",
        PolicyKind::Allow => "PolicyAllow",
        PolicyKind::Exact => "PolicyExact",
    };
    record(vec![
        adt("PolicyKind", kind, Vec::new()),
        list(policy.names.iter().map(|name| text(name)).collect()),
        nat_val(policy.control),
        option(policy.names_arg.as_ref().map(enc_arg)),
    ])
}

fn enc_origin(origin: &Origin) -> Value {
    match origin {
        Origin::Structural { package, entry } => {
            adt("Origin", "Structural", vec![text(package), text(entry)])
        }
        Origin::Metadata { owner } => adt("Origin", "Metadata", vec![text(owner)]),
        other => panic!("the structural parser binds no {other:?}"),
    }
}

fn enc_class(class: AtomClass) -> Value {
    Value::Adt {
        constructor: class_index(class),
        fields: Vec::new(),
    }
}

fn enc_items(items: &[ProofItemAst]) -> Value {
    items
        .iter()
        .rev()
        .fold(adt("Proof", "ProofNil", Vec::new()), |chain, item| {
            adt("Proof", "ProofCons", vec![enc_item(item), chain])
        })
}

fn enc_env(env: &ProofEnvAst) -> Value {
    adt(
        "Proof",
        "ProofEnv",
        vec![enc_items(&env.items), nat_val(env.begin)],
    )
}

fn enc_labeled(rows: &[(u64, ProofEnvAst)]) -> Value {
    rows.iter().rev().fold(
        adt("Proof", "ProofNil", Vec::new()),
        |chain, (label, env)| {
            adt(
                "Proof",
                "LabeledCons",
                vec![number(*label), enc_env(env), chain],
            )
        },
    )
}

fn enc_cases(rows: &[CaseAst]) -> Value {
    rows.iter()
        .rev()
        .fold(adt("Proof", "ProofNil", Vec::new()), |chain, row| {
            adt(
                "Proof",
                "CaseCons",
                vec![
                    enc_arg(&row.entry),
                    list(row.binds.iter().map(|bind| text(bind)).collect()),
                    enc_arg(&row.bind_arg),
                    enc_env(&row.proof),
                    chain,
                ],
            )
        })
}

fn enc_step(step: &CalcStepAst) -> Value {
    record(vec![
        enc_arg(&step.relation),
        enc_arg(&step.term),
        enc_arg(&step.proof),
    ])
}

fn enc_item(item: &ProofItemAst) -> Value {
    match item {
        ProofItemAst::Sentence(sentence) => {
            adt("Proof", "ProofSentence", vec![enc_sentence(sentence)])
        }
        ProofItemAst::Have {
            name,
            proposition,
            proof,
            begin,
        } => adt(
            "Proof",
            "ProofHave",
            vec![
                enc_arg(name),
                enc_sentence(proposition),
                enc_env(proof),
                nat_val(*begin),
            ],
        ),
        ProofItemAst::Rewrite {
            target,
            rules,
            begin,
        } => adt(
            "Proof",
            "ProofRewrite",
            vec![
                enc_arg(target),
                list(
                    rules
                        .iter()
                        .map(|(reverse, rule)| pair(boolean(*reverse), enc_arg(rule)))
                        .collect(),
                ),
                nat_val(*begin),
            ],
        ),
        ProofItemAst::Simplify {
            target,
            rules,
            begin,
        } => adt(
            "Proof",
            "ProofSimplify",
            vec![enc_arg(target), enc_args(rules), nat_val(*begin)],
        ),
        ProofItemAst::Apply {
            function,
            premises,
            begin,
        } => adt(
            "Proof",
            "ProofApply",
            vec![enc_arg(function), enc_labeled(premises), nat_val(*begin)],
        ),
        ProofItemAst::Constructor { branches, begin } => adt(
            "Proof",
            "ProofConstructor",
            vec![enc_labeled(branches), nat_val(*begin)],
        ),
        ProofItemAst::CasesLike {
            induction,
            scrutinee,
            cases,
            begin,
        } => adt(
            "Proof",
            "ProofCases",
            vec![
                boolean(*induction),
                enc_arg(scrutinee),
                enc_cases(cases),
                nat_val(*begin),
            ],
        ),
        ProofItemAst::Calculate {
            start,
            steps,
            begin,
        } => adt(
            "Proof",
            "ProofCalculate",
            vec![
                enc_arg(start),
                list(steps.iter().map(enc_step).collect()),
                nat_val(*begin),
            ],
        ),
    }
}

fn enc_decl(decl: &DeclAst) -> Value {
    let kind = match decl.kind {
        DeclKind::TypeDefinition => "TypeDefinition",
        DeclKind::TermDefinition => "TermDefinition",
        DeclKind::PredicateDefinition => "PredicateDefinition",
        DeclKind::Theorem => "Theorem",
        DeclKind::Lemma => "Lemma",
        DeclKind::Corollary => "Corollary",
    };
    record(vec![
        adt("DeclKind", kind, Vec::new()),
        enc_arg(&decl.component),
        option(decl.entry.as_ref().map(enc_arg)),
        enc_policy(&decl.policy),
        enc_sentence(&decl.sentence),
        option(decl.proof.as_ref().map(enc_env)),
        nat_val(decl.begin),
        nat_val(decl.end),
    ])
}

fn enc_range(range: AtomRange) -> Value {
    pair(nat_val(range.0), nat_val(range.1))
}

fn enc_blocks(blocks: &[BlockAst]) -> Value {
    blocks
        .iter()
        .rev()
        .fold(adt("Block", "BlockNil", Vec::new()), |chain, block| {
            adt("Block", "BlockCons", vec![enc_block(block), chain])
        })
}

fn enc_block(block: &BlockAst) -> Value {
    match block {
        BlockAst::Section {
            component,
            heading,
            params,
            blocks,
            begin,
        } => adt(
            "Block",
            "BlockSection",
            vec![
                enc_arg(component),
                nat_val(heading.0),
                nat_val(heading.1),
                option(params.map(enc_range)),
                enc_blocks(blocks),
                nat_val(*begin),
            ],
        ),
        BlockAst::Declaration(decl) => adt("Block", "BlockDecl", vec![enc_decl(decl)]),
    }
}

fn enc_form(data: &BraceArg, surface: bool, begin: usize) -> Value {
    record(vec![enc_arg(data), boolean(surface), nat_val(begin)])
}

fn enc_module(module: &ModuleAst) -> Value {
    let rows = module
        .structural_rows
        .iter()
        .map(|row| {
            record(vec![
                text(&row.path),
                nat_val(row.byte_start),
                nat_val(row.byte_end),
                enc_class(row.class),
                enc_origin(&row.binding),
            ])
        })
        .collect();
    record(vec![
        list(rows),
        text(&module.name),
        enc_arg(&module.name_arg),
        enc_args(&module.uses),
        enc_args(&module.imports),
        nat_val(module.title.0),
        nat_val(module.title.1),
        enc_blocks(&module.blocks),
        option(
            module
                .core
                .as_ref()
                .map(|form| enc_form(&form.data, form.surface, form.begin)),
        ),
        option(
            module
                .semantic
                .as_ref()
                .map(|form| enc_form(&form.data, form.surface, form.begin)),
        ),
        option(
            module
                .tooling
                .as_ref()
                .map(|form| enc_form(&form.data, form.surface, form.begin)),
        ),
    ])
}

/// The root's run on these atoms, and the oracle's message when it refuses.
fn run_on(path: &str, atoms: &[Atom], expected: &str, depth: u64) -> (Run, Option<String>) {
    let (value, refusal) = match parse_module(path, atoms, expected, depth) {
        Ok(module) => (
            Value::Ok {
                value: Box::new(enc_module(&module)),
            },
            None,
        ),
        Err(diagnostic) => (
            Value::Error {
                value: Box::new(rejection_value(&diagnostic)),
            },
            Some(diagnostic.message.clone()),
        ),
    };
    (
        (
            vec![
                text(path),
                list(atoms.iter().map(atom_value).collect()),
                text(expected),
                number(depth),
            ],
            serde_json::to_value(value).expect("a value is JSON"),
        ),
        refusal,
    )
}

// ------------------------------------------------------------------ vectors

/// One named vector: a text the oracle's scanner reads, the atoms the root is
/// given, and the arguments beside them.
struct Vector {
    label: String,
    path: String,
    expected: String,
    source: String,
    depth: u64,
    /// The atoms without their white space, which the scanner never returns:
    /// neighbours that do not touch hold the parser to atoms and not to bytes.
    gapped: bool,
}

impl Vector {
    fn atoms(&self) -> Vec<Atom> {
        let atoms = lexlean::source::scan::scan(&self.path, &self.source, u64::MAX)
            .unwrap_or_else(|failure| panic!("{}: {failure:?}", self.label));
        if self.gapped {
            atoms
                .into_iter()
                .filter(|atom| atom.class != AtomClass::Whitespace)
                .collect()
        } else {
            atoms
        }
    }

    fn run(&self) -> (Run, Option<String>) {
        run_on(&self.path, &self.atoms(), &self.expected, self.depth)
    }
}

fn wrapped(body: &str) -> String {
    format!("\\begin{{lexlean}}{{M}}\n\\title{{T}}\n{body}\n\\end{{lexlean}}\n")
}

fn declared(env: &str, tail: &str) -> String {
    wrapped(&format!("\\begin{{{env}}}{tail}\\end{{{env}}}"))
}

fn proved(body: &str) -> String {
    declared(
        "theorem",
        &format!("{{t}}\\noaxioms S.\\begin{{proof}}{body}\\end{{proof}}"),
    )
}

fn form(env: &str, body: &str) -> String {
    wrapped(&format!("\\begin{{{env}}}{body}\\end{{{env}}}"))
}

fn sectioned(levels: usize) -> String {
    let mut body = String::new();
    for level in 0..levels {
        body.push_str(&format!("\\begin{{section}}{{s{level}}}\\heading{{H}}"));
    }
    for _ in 0..levels {
        body.push_str("\\end{section}");
    }
    wrapped(&body)
}

fn nested_haves(levels: usize) -> String {
    let mut body = String::new();
    for level in 0..levels {
        body.push_str(&format!("\\begin{{have}}{{h{level}}}P.\\begin{{proof}}"));
    }
    body.push_str("Done.");
    for _ in 0..levels {
        body.push_str("\\end{proof}\\end{have}");
    }
    proved(&body)
}

#[allow(clippy::too_many_lines)]
fn named() -> Vec<Vector> {
    let mut out: Vec<Vector> = Vec::new();
    let mut add = |label: &str, source: String| {
        out.push(Vector {
            label: label.to_owned(),
            path: "m".to_owned(),
            expected: "M".to_owned(),
            source,
            depth: DEPTH,
            gapped: false,
        });
    };

    // The header (§15.1).
    add("empty", String::new());
    add("white space only", " \n ".to_owned());
    add("a word first", "x".to_owned());
    add("begin at the end of the file", "\\begin".to_owned());
    add("begin without a brace", "\\begin x".to_owned());
    add("an open name", "\\begin{lexlean".to_owned());
    add(
        "another first environment",
        "\\begin{x}{M}\\title{T}\\end{lexlean}".to_owned(),
    );
    add("two refusals: the first stands", "\\begin{x}{N}".to_owned());
    add("no name", "\\begin{lexlean}".to_owned());
    add(
        "another name",
        "\\begin{lexlean}{N}\\title{T}\\end{lexlean}".to_owned(),
    );
    add(
        "a name with white space",
        "\\begin{lexlean}{ M }\\title{T}\\end{lexlean}".to_owned(),
    );
    add(
        "a name of two atoms",
        "\\begin{lexlean}{M.N}\\title{T}\\end{lexlean}".to_owned(),
    );
    add("minimal", wrapped(""));
    add("no title", "\\begin{lexlean}{M}\\end{lexlean}".to_owned());
    add(
        "two titles",
        "\\begin{lexlean}{M}\\title{T}\\title{U}\\end{lexlean}".to_owned(),
    );
    add(
        "uses and imports",
        "\\begin{lexlean}{M}\n\\useglossary{a b@1.0.0}\n\\useglossary{c}\n\\importmodule{D.E}\n\\importmodule{F}\n\\title{A title}\n\\end{lexlean}".to_owned(),
    );
    add(
        "a use after an import",
        "\\begin{lexlean}{M}\\importmodule{F}\\useglossary{c}\\title{T}\\end{lexlean}".to_owned(),
    );
    add("an open title", "\\begin{lexlean}{M}\\title{T".to_owned());
    add(
        "a title of nested braces",
        "\\begin{lexlean}{M}\\title{a{b}c}\\end{lexlean}".to_owned(),
    );
    add(
        "nested braces in a use",
        "\\begin{lexlean}{M}\\useglossary{a{b}c}\\title{T}\\end{lexlean}".to_owned(),
    );
    add(
        "white space in a use",
        "\\begin{lexlean}{M}\\useglossary{a  b\nc}\\title{T}\\end{lexlean}".to_owned(),
    );
    add("text after the end", format!("{}x", wrapped("")));
    add(
        "white space around the module",
        format!("\n \n{}\n \n", wrapped("")),
    );

    // Blocks.
    add("text among the blocks", wrapped("x"));
    add(
        "a block left open",
        "\\begin{lexlean}{M}\\title{T}".to_owned(),
    );
    add("another end", wrapped("\\end{section}"));
    add("an unknown block", wrapped("\\begin{foo}\\end{foo}"));
    add(
        "a section",
        wrapped("\\begin{section}{s}\\heading{A heading}\\end{section}"),
    );
    add(
        "a section with parameters",
        wrapped("\\begin{section}{s}\\heading{H}\\parameters{p q}\\end{section}"),
    );
    add(
        "a section with an upper-case id",
        wrapped("\\begin{section}{Abc}\\heading{H}\\end{section}"),
    );
    add(
        "a section with an id of a digit",
        wrapped("\\begin{section}{1}\\heading{H}\\end{section}"),
    );
    add(
        "a section with an empty id",
        wrapped("\\begin{section}{}\\heading{H}\\end{section}"),
    );
    add(
        "a section with a hyphen",
        wrapped("\\begin{section}{a-b1}\\heading{H}\\end{section}"),
    );
    add(
        "a section with an underscore",
        wrapped("\\begin{section}{a_b}\\heading{H}\\end{section}"),
    );
    add(
        "a section without a heading",
        wrapped("\\begin{section}{s}\\end{section}"),
    );
    add(
        "a section left open",
        "\\begin{lexlean}{M}\\title{T}\\begin{section}{s}\\heading{H}".to_owned(),
    );
    add(
        "a section ended as another",
        wrapped("\\begin{section}{s}\\heading{H}\\end{lexlean}"),
    );
    add("sections and a theorem among them", wrapped(
        "\\begin{section}{a}\\heading{H}\\begin{termdefinition}{d}{e}\\noaxioms S.\\end{termdefinition}\\begin{section}{b}\\heading{I}\\end{section}\\end{section}\\begin{lemma}{l}\\noaxioms P.\\begin{proof}Q.\\end{proof}\\end{lemma}",
    ));
    add("nested sections", sectioned(3));

    // Declarations.
    for env in ["typedefinition", "termdefinition", "predicatedefinition"] {
        add(
            &format!("a {env}"),
            declared(env, "{d}{a b}\\noaxioms The sentence."),
        );
        add(
            &format!("a {env} with a proof"),
            declared(env, "{d}{e}\\noaxioms S.\\begin{proof}P.\\end{proof}"),
        );
    }
    for env in ["theorem", "lemma", "corollary"] {
        add(
            &format!("a {env}"),
            declared(env, "{t}\\noaxioms S.\\begin{proof}P.\\end{proof}"),
        );
        add(
            &format!("a {env} without a proof"),
            declared(env, "{t}\\noaxioms S."),
        );
    }
    add(
        "a definition with a bad id",
        declared("termdefinition", "{D}{e}\\noaxioms S."),
    );
    add(
        "a definition without an entry",
        declared("termdefinition", "{d}\\noaxioms S."),
    );
    add("no policy", declared("termdefinition", "{d}{e} S."));
    add(
        "an allow policy",
        declared("termdefinition", "{d}{e}\\allowaxioms{b;a;propext}S."),
    );
    add(
        "an exact policy",
        declared(
            "theorem",
            "{t}\\exactaxioms{Quot.sound;Classical.choice}S.\\begin{proof}P.\\end{proof}",
        ),
    );
    add(
        "a policy without a brace",
        declared("termdefinition", "{d}{e}\\allowaxioms S."),
    );
    add(
        "an empty name list",
        declared("termdefinition", "{d}{e}\\allowaxioms{}S."),
    );
    add(
        "an empty name among names",
        declared("termdefinition", "{d}{e}\\allowaxioms{a;;b}S."),
    );
    add(
        "a trailing separator",
        declared("termdefinition", "{d}{e}\\allowaxioms{a;}S."),
    );
    add(
        "a name of a digit first",
        declared("termdefinition", "{d}{e}\\allowaxioms{1a}S."),
    );
    add(
        "a name with a trailing dot",
        declared("termdefinition", "{d}{e}\\allowaxioms{a.}S."),
    );
    add(
        "a name with a leading dot",
        declared("termdefinition", "{d}{e}\\allowaxioms{.a}S."),
    );
    add(
        "a name with an empty segment",
        declared("termdefinition", "{d}{e}\\allowaxioms{a..b}S."),
    );
    add(
        "a name of underscores and digits",
        declared("termdefinition", "{d}{e}\\exactaxioms{_a1.b_2}S."),
    );
    add(
        "a name with a hyphen",
        declared("termdefinition", "{d}{e}\\exactaxioms{a-b}S."),
    );
    add(
        "a name beyond ASCII",
        declared("termdefinition", "{d}{e}\\exactaxioms{\u{e9}}S."),
    );
    add(
        "a duplicate name",
        declared("termdefinition", "{d}{e}\\allowaxioms{a;b;a}S."),
    );
    add(
        "names that sort by byte",
        declared("termdefinition", "{d}{e}\\allowaxioms{b;B;a.b;a;_}S."),
    );
    add(
        "a refused name after a duplicate",
        declared("termdefinition", "{d}{e}\\allowaxioms{a;a;1}S."),
    );

    // Sentences.
    add(
        "a sentence to the end of the file",
        "\\begin{lexlean}{M}\\title{T}\\begin{termdefinition}{d}{e}\\noaxioms S".to_owned(),
    );
    add(
        "a sentence stopped by an end",
        declared("termdefinition", "{d}{e}\\noaxioms S"),
    );
    add(
        "a sentence stopped by a begin",
        wrapped("\\begin{termdefinition}{d}{e}\\noaxioms S\\begin{lemma}\\end{termdefinition}"),
    );
    add(
        "a period inside braces",
        declared("termdefinition", "{d}{e}\\noaxioms {a.b} c."),
    );
    add(
        "a period inside an inline island",
        declared("termdefinition", "{d}{e}\\noaxioms \\(a.b\\) c."),
    );
    add(
        "a period inside a display island",
        declared("termdefinition", "{d}{e}\\noaxioms \\[a.b\\] c."),
    );
    add(
        "a display island that the other closer does not close",
        declared("termdefinition", "{d}{e}\\noaxioms \\[a\\)b.\\] c."),
    );
    add(
        "an inline island that the other closer does not close",
        declared("termdefinition", "{d}{e}\\noaxioms \\(a\\]b.\\) c."),
    );
    add(
        "a closing brace at depth zero",
        declared("termdefinition", "{d}{e}\\noaxioms } a."),
    );
    add(
        "a begin inside braces",
        declared("termdefinition", "{d}{e}\\noaxioms {\\begin} a."),
    );
    add(
        "an end inside an island",
        declared("termdefinition", "{d}{e}\\noaxioms \\(\\end\\) a."),
    );
    add(
        "a brace inside an island",
        declared("termdefinition", "{d}{e}\\noaxioms \\({\\) a."),
    );
    add(
        "an island opener inside an island",
        declared("termdefinition", "{d}{e}\\noaxioms \\(\\[\\) a."),
    );
    add(
        "a period word",
        declared("termdefinition", "{d}{e}\\noaxioms a.b."),
    );
    add(
        "the end of the declaration after a sentence",
        declared("termdefinition", "{d}{e}\\noaxioms a. b."),
    );

    // Proofs (§16).
    add("an empty proof", proved(""));
    add(
        "sentences",
        proved("Assume \\(x\\). Close the goal by reflexivity."),
    );
    add(
        "a proof left open",
        "\\begin{lexlean}{M}\\title{T}\\begin{theorem}{t}\\noaxioms S.\\begin{proof}".to_owned(),
    );
    add(
        "a proof ended as another",
        wrapped("\\begin{theorem}{t}\\noaxioms S.\\begin{proof}\\end{have}\\end{theorem}"),
    );
    add(
        "a proof's theorem ended as another",
        wrapped("\\begin{theorem}{t}\\noaxioms S.\\begin{proof}\\end{proof}\\end{lemma}"),
    );
    add(
        "an unknown proof environment",
        proved("\\begin{foo}\\end{foo}"),
    );
    add(
        "a have",
        proved("\\begin{have}{h}P \\(x\\).\\begin{proof}Q.\\end{proof}\\end{have}R."),
    );
    add(
        "a have without a proof",
        proved("\\begin{have}{h}P.\\end{have}"),
    );
    add(
        "a have with another environment",
        proved("\\begin{have}{h}P.\\begin{foo}\\end{foo}\\end{have}"),
    );
    add(
        "a have ended as another",
        proved("\\begin{have}{h}P.\\begin{proof}Q.\\end{proof}\\end{apply}"),
    );
    add(
        "a rewrite forward",
        proved("\\begin{rewrite}{goal}\\forward{\\(a\\)}\\end{rewrite}"),
    );
    add(
        "a rewrite backward",
        proved("\\begin{rewrite}{ h }\\backward{\\(a\\)}\\end{rewrite}"),
    );
    add(
        "a rewrite of both",
        proved("\\begin{rewrite}{goal}\\backward{a}\\forward{b c}\\backward{d}\\end{rewrite}"),
    );
    add(
        "a rewrite without a rule",
        proved("\\begin{rewrite}{goal}\\end{rewrite}"),
    );
    add(
        "a rewrite ended as another without a rule",
        proved("\\begin{rewrite}{goal}\\end{simplify}"),
    );
    add(
        "a rewrite with a foreign control",
        proved("\\begin{rewrite}{goal}\\rule{a}\\end{rewrite}"),
    );
    add(
        "a simplify",
        proved("\\begin{simplify}{goal}\\rule{a}\\rule{b}\\end{simplify}"),
    );
    add(
        "a simplify without a rule",
        proved("\\begin{simplify}{goal}\\end{simplify}"),
    );
    add("an apply", proved("\\begin{apply}{\\(f\\)}\\begin{premise}{1}P.\\end{premise}\\begin{premise}{2}Q.\\end{premise}\\end{apply}"));
    add(
        "an apply without premises",
        proved("\\begin{apply}{f}\\end{apply}"),
    );
    add(
        "an apply ended as another",
        proved("\\begin{apply}{f}\\end{proof}"),
    );
    add(
        "a premise ended as another",
        proved("\\begin{apply}{f}\\begin{premise}{1}P.\\end{branch}\\end{apply}"),
    );
    add(
        "a spaced begin of a premise",
        proved("\\begin{apply}{f}\\begin {premise}{1}P.\\end{premise}\\end{apply}"),
    );
    add(
        "a spaced name of a premise",
        proved("\\begin{apply}{f}\\begin{ premise}{1}P.\\end{premise}\\end{apply}"),
    );
    add(
        "a branch in an apply",
        proved("\\begin{apply}{f}\\begin{branch}{1}P.\\end{branch}\\end{apply}"),
    );
    add("a constructor", proved("\\begin{constructor}\\begin{branch}{1}P.\\end{branch}\\begin{branch}{2}Q.\\end{branch}\\end{constructor}"));
    add(
        "a constructor without branches",
        proved("\\begin{constructor}\\end{constructor}"),
    );
    add(
        "a premise in a constructor",
        proved("\\begin{constructor}\\begin{premise}{1}P.\\end{premise}\\end{constructor}"),
    );
    for (label, spelled) in [
        ("zero", "0"),
        ("padded", "007"),
        ("signed", "+5"),
        ("a sign alone", "+"),
        ("negative", "-1"),
        ("empty", ""),
        ("two signs", "++1"),
        ("a word", "x"),
        ("a digit and a word", "5a"),
        ("white space in it", "1 2"),
        ("the largest", "18446744073709551615"),
        ("one more", "18446744073709551616"),
        ("signed largest", "+18446744073709551615"),
        ("a digit too many", "184467440737095516150"),
        ("far too many", "99999999999999999999999999"),
        ("padded largest", "00000000000018446744073709551615"),
        ("a scalar", "\u{663}"),
    ] {
        add(&format!("a premise labeled {label}"), proved(&format!("\\begin{{apply}}{{f}}\\begin{{premise}}{{{spelled}}}P.\\end{{premise}}\\end{{apply}}")));
    }
    add(
        "a branch labeled by a word",
        proved("\\begin{constructor}\\begin{branch}{x}P.\\end{branch}\\end{constructor}"),
    );
    add("cases", proved("\\begin{cases}{\\(n\\)}\\begin{case}{p::zero}\\bind{}P.\\end{case}\\begin{case}{p::succ}\\bind{m;h}Q.\\end{case}\\end{cases}"));
    add(
        "induction",
        proved("\\begin{induction}{n}\\begin{case}{p::zero}\\bind{}P.\\end{case}\\end{induction}"),
    );
    add(
        "cases without a case",
        proved("\\begin{cases}{n}\\end{cases}"),
    );
    add(
        "induction ended as cases",
        proved("\\begin{induction}{n}\\end{cases}"),
    );
    add(
        "a case without a bind",
        proved("\\begin{cases}{n}\\begin{case}{p::zero}P.\\end{case}\\end{cases}"),
    );
    add(
        "a bind of separators",
        proved("\\begin{cases}{n}\\begin{case}{c}\\bind{;}P.\\end{case}\\end{cases}"),
    );
    add(
        "a bind with white space",
        proved("\\begin{cases}{n}\\begin{case}{c d}\\bind{a b; c}P.\\end{case}\\end{cases}"),
    );
    add(
        "a calculation",
        proved(
            "\\begin{calculate}\\start{a}\\step{p::eq}{b}{c}\\step{q r}{d e}{f}\\end{calculate}",
        ),
    );
    add(
        "a calculation without a start",
        proved("\\begin{calculate}\\step{p}{b}{c}\\end{calculate}"),
    );
    add(
        "a calculation without a step",
        proved("\\begin{calculate}\\start{a}\\end{calculate}"),
    );
    add(
        "a step short of an argument",
        proved("\\begin{calculate}\\start{a}\\step{p}{b}\\end{calculate}"),
    );
    add(
        "a calculation ended as another",
        proved("\\begin{calculate}\\start{a}\\step{p}{b}{c}\\end{proof}"),
    );
    add("nested haves", nested_haves(3));
    add(
        "every proof form in one proof",
        proved("A. \\begin{have}{h}B.\\begin{proof}\\begin{apply}{f}\\begin{premise}{1}\\begin{constructor}\\begin{branch}{2}\\begin{cases}{c}\\begin{case}{d}\\bind{e}\\begin{calculate}\\start{g}\\step{i}{j}{k}\\end{calculate}\\end{case}\\end{cases}\\end{branch}\\end{constructor}\\end{premise}\\end{apply}\\end{proof}\\end{have}\\begin{rewrite}{goal}\\forward{l}\\end{rewrite}\\begin{simplify}{goal}\\rule{m}\\end{simplify}Z."),
    );

    // The closed data forms (§17.20, §17.21).
    for (env, control, label) in [
        ("coremodule", "coredata", "core"),
        ("semanticmodule", "semanticdata", "semantic"),
        ("toolingmodule", "toolingdata", "tooling"),
    ] {
        add(
            &format!("{label} data"),
            form(
                env,
                &format!("\\{control}{{{{\"a b\": \"c  d\", \"e\": [1, {{\"f\": 2}}]}}}}"),
            ),
        );
        add(
            &format!("{label} empty data"),
            form(env, &format!("\\{control}{{}}")),
        );
        add(
            &format!("{label} spaced data"),
            form(env, &format!(" \n\\{control} \n{{ x }}")),
        );
        add(
            &format!("{label} notation"),
            form(env, "\n  definition{name: a, body: \"b  c\"}\n"),
        );
        add(&format!("{label} empty notation"), form(env, ""));
        add(
            &format!("{label} left open"),
            format!("\\begin{{lexlean}}{{M}}\\title{{T}}\\begin{{{env}}}x"),
        );
        add(
            &format!("{label} data left open"),
            format!("\\begin{{lexlean}}{{M}}\\title{{T}}\\begin{{{env}}}\\{control}{{\"a"),
        );
        add(
            &format!("{label} ended as another"),
            wrapped(&format!("\\begin{{{env}}}x\\end{{foo}}")),
        );
        add(
            &format!("{label} without its module end"),
            format!("\\begin{{lexlean}}{{M}}\\title{{T}}\\begin{{{env}}}x\\end{{{env}}}"),
        );
        add(
            &format!("{label} with text after"),
            format!("{}y", form(env, "x")),
        );
    }
    add(
        "a brace inside a string",
        form("coremodule", "\\coredata{\"}\" {\"a\": \"{\"}}"),
    );
    add(
        "an escaped quote in a string",
        form("coremodule", "\\coredata{\"a\\\" }  b\"}"),
    );
    add(
        "an escaped backslash in a string",
        form("coremodule", "\\coredata{\"a\\\\\"   }"),
    );
    add(
        "white space outside a string",
        form("coremodule", "\\coredata{ a  \"b  c\"  d }"),
    );
    add(
        "a line feed in a string",
        form("semanticmodule", "\\semanticdata{\"a\nb\"}"),
    );
    add(
        "a scalar in a string",
        form("toolingmodule", "\\toolingdata{\"\u{2115}  \u{65e5}\"}"),
    );
    add(
        "another form's data",
        form("coremodule", "\\semanticdata{x}"),
    );
    add(
        "a spaced begin of a form",
        wrapped("\\begin {coremodule}x\\end{coremodule}"),
    );
    add(
        "a spaced name of a form",
        wrapped("\\begin{ coremodule}x\\end{coremodule}"),
    );
    add(
        "a word after a form's name",
        wrapped("\\begin{coremodule}\\end {coremodule}"),
    );
    add(
        "notation with a control",
        form("semanticmodule", "a \\begin{x} \\(y\\) b"),
    );
    add(
        "a form and blocks",
        wrapped(
            "\\begin{coremodule}x\\end{coremodule}\\begin{section}{s}\\heading{H}\\end{section}",
        ),
    );
    add(
        "two forms",
        wrapped("\\begin{coremodule}x\\end{coremodule}\\begin{toolingmodule}y\\end{toolingmodule}"),
    );

    // The limit, at each side of each nesting and at the extremes.
    let limits: &[(&str, String, u64)] = &[
        ("no sections under a limit of zero", wrapped(""), 0),
        ("a section under a limit of zero", sectioned(1), 0),
        ("a section under a limit of one", sectioned(1), 1),
        ("two sections under a limit of one", sectioned(2), 1),
        ("two sections under a limit of two", sectioned(2), 2),
        ("a proof under a limit of zero", proved("P."), 0),
        ("a proof under a limit of one", proved("P."), 1),
        ("a nested proof under a limit of one", nested_haves(1), 1),
        ("a nested proof under a limit of two", nested_haves(1), 2),
        ("three nested under a limit of three", nested_haves(3), 3),
        ("three nested under a limit of four", nested_haves(3), 4),
        (
            "a premise under a limit of one",
            proved("\\begin{apply}{f}\\begin{premise}{1}P.\\end{premise}\\end{apply}"),
            1,
        ),
        (
            "a branch under a limit of one",
            proved("\\begin{constructor}\\begin{branch}{1}P.\\end{branch}\\end{constructor}"),
            1,
        ),
        (
            "a case under a limit of one",
            proved("\\begin{cases}{n}\\begin{case}{c}\\bind{}P.\\end{case}\\end{cases}"),
            1,
        ),
        ("a section under the largest limit", sectioned(1), u64::MAX),
        (
            "a proof under a limit named in its message",
            nested_haves(1),
            1_000_000_007,
        ),
    ];
    for (label, source, depth) in limits {
        out.push(Vector {
            label: (*label).to_owned(),
            path: "m".to_owned(),
            expected: "M".to_owned(),
            source: source.clone(),
            depth: *depth,
            gapped: false,
        });
    }
    // The path a span and a coverage row name, and the name it is held to.
    for (label, path, expected) in [
        ("another path", "src/Demo.lex.tex", "M"),
        ("a path beyond ASCII", "\u{fc}/\u{2115}.lex.tex", "M"),
        ("an empty path", "", "M"),
        ("an empty name", "m", ""),
    ] {
        for (kind, source) in [("accepted", wrapped("")), ("refused", wrapped("x"))] {
            out.push(Vector {
                label: format!("{label}, {kind}"),
                path: path.to_owned(),
                expected: expected.to_owned(),
                source,
                depth: DEPTH,
                gapped: false,
            });
        }
    }
    out.push(Vector {
        label: "an empty file under a path".to_owned(),
        path: "src/Demo.lex.tex".to_owned(),
        expected: "M".to_owned(),
        source: String::new(),
        depth: DEPTH,
        gapped: false,
    });
    // Without white space, the same grammar reads the same atoms.
    let gapped: Vec<Vector> = out
        .iter()
        .step_by(4)
        .map(|vector| Vector {
            label: format!("{}, gapped", vector.label),
            path: vector.path.clone(),
            expected: vector.expected.clone(),
            source: vector.source.clone(),
            depth: vector.depth,
            gapped: true,
        })
        .collect();
    out.extend(gapped);
    out
}

/// Every refusal `parse_module` can state, by a part of its message, so that
/// a corpus that never reached one fails here and not silently (R2).
const REFUSALS: &[&str] = &[
    "found end of file",
    ", found `",
    "unclosed `{` argument",
    "unclosed environment",
    "expected `\\end{",
    "a sentence must end with a period",
    "is not a conservative ASCII Lean name",
    "empty entry in an axiom name list",
    "duplicate axiom name",
    "every definition and theorem-like declaration requires exactly one axiom policy",
    "is not a decimal label",
    "max_scope_depth exceeded in phase parse (proof environment nesting)",
    "max_scope_depth exceeded in phase parse (section nesting)",
    "unclosed `proof` environment",
    "a have establishes its proposition with a nested proof",
    "a rewrite requires at least one rule",
    "a simplify requires at least one rule",
    "a calculation requires at least one step",
    "is not a proof environment",
    "is not a component ID",
    "a theorem-like declaration without a proof is not valid",
    "no definitional component accepts a proof environment",
    "expected a block or",
    "unclosed `lexlean` environment",
    "is not a block environment",
    "a module begins with",
    "does not match the source path module",
    "glossary imports occur before module imports",
    "each module has exactly one title",
    "after \\end{lexlean}",
];

/// The module name a committed source declares, which is the name the
/// compiler holds it to.
fn declared_name(source: &str) -> String {
    source
        .split_once("{lexlean}{")
        .and_then(|(_, rest)| rest.split_once('}'))
        .map_or_else(|| "M".to_owned(), |(name, _)| name.to_owned())
}

fn scanned(path: &str, source: &str) -> Option<Vec<Atom>> {
    lexlean::source::scan::scan(path, source, u64::MAX).ok()
}

/// The runs of the committed modules, one a module: each whole up to the first
/// bound, or the atoms of both its ends up to the second.
fn committed(sources: &[(String, String)], (whole, ends): (usize, usize)) -> Vec<Run> {
    sources
        .iter()
        .filter_map(|(path, source)| {
            let mut all = scanned(path, source)?;
            if all.len() > whole {
                all.drain(ends / 2..all.len() - ends / 2);
            }
            Some(run_on(path, &all, &declared_name(source), DEPTH).0)
        })
        .collect()
}

/// A whole atom or a run of them, of each shape the grammar reads.
const PIECES: &[&str] = &[
    "\\begin{section}{s}\\heading{H}",
    "\\end{section}",
    "\\begin{termdefinition}{d}{e}",
    "\\end{termdefinition}",
    "\\begin{theorem}{t}",
    "\\end{theorem}",
    "\\begin{proof}",
    "\\end{proof}",
    "\\begin{have}{h}",
    "\\end{have}",
    "\\begin{rewrite}{goal}",
    "\\end{rewrite}",
    "\\begin{apply}{f}",
    "\\end{apply}",
    "\\begin{premise}{1}",
    "\\end{premise}",
    "\\begin{cases}{n}",
    "\\begin{case}{c}\\bind{}",
    "\\end{case}",
    "\\begin{calculate}\\start{a}",
    "\\step{p}{q}{r}",
    "\\forward{x}",
    "\\backward{y}",
    "\\rule{z}",
    "\\noaxioms",
    "\\allowaxioms{a;b}",
    "\\exactaxioms{c}",
    "\\title{T}",
    "\\useglossary{g}",
    "\\importmodule{I}",
    "\\begin",
    "\\end",
    "\\(",
    "\\)",
    "\\[",
    "\\]",
    "{",
    "}",
    ".",
    ";",
    "\"",
    "\\\"",
    "\\\\",
    "\\coredata",
    "\\begin{coremodule}",
    "\\end{coremodule}",
    " ",
    "\n",
    "x",
    "12",
    "S.",
    "\u{2115}",
];

/// One seeded edit of a text, at an atom boundary: a deletion, an insertion,
/// or a replacement.
fn edited(seeded: &mut Seeded, source: &str) -> Option<String> {
    let atoms = scanned("m", source)?;
    if atoms.is_empty() {
        return Some((*seeded.pick(PIECES)).to_owned());
    }
    let mut out = source.to_owned();
    for _ in 0..=seeded.below(3) {
        let atoms = scanned("m", &out)?;
        if atoms.is_empty() {
            break;
        }
        let at = seeded.pick(&atoms);
        let (start, stop) = (at.byte_start, at.byte_end);
        match seeded.below(3) {
            0 => out.replace_range(start..stop, ""),
            1 => out.insert_str(start, seeded.pick(PIECES)),
            _ => out.replace_range(start..stop, seeded.pick(PIECES)),
        }
    }
    Some(out)
}

/// The seeded corpus, `count` edits of the texts of the vectors and of the
/// smallest committed modules.
fn mutants(count: usize, vectors: &[Vector], sources: &[(String, String)]) -> Vec<Run> {
    let mut seeded = Seeded(0x0B55_0021);
    let bases: Vec<&str> = vectors
        .iter()
        .map(|vector| vector.source.as_str())
        .chain(
            sources
                .iter()
                .filter(|(_, source)| source.len() <= MUTANT_BASE)
                .map(|(_, source)| source.as_str()),
        )
        .collect();
    let mut out = Vec::with_capacity(count);
    while out.len() < count {
        let base = *seeded.pick(&bases);
        let Some(text) = edited(&mut seeded, base) else {
            continue;
        };
        let Some(atoms) = scanned("m", &text) else {
            continue;
        };
        let depth = if seeded.chance(10) {
            u64::try_from(seeded.below(5)).expect("a depth")
        } else {
            DEPTH
        };
        let expected = if seeded.chance(10) {
            "N".to_owned()
        } else {
            declared_name(&text)
        };
        out.push(run_on("m", &atoms, &expected, depth).0);
    }
    out
}

// ------------------------------------------------------------------- plants

/// A defect planted in the source, and the named vector that catches it.
struct Plant {
    label: &'static str,
    site: &'static str,
    replacement: &'static str,
    vector: &'static str,
}

const PLANTS: &[Plant] = &[
    Plant {
        label: "white space inside a plain argument is kept",
        site: "(if (if quoted then false else ((PrimitiveScan::classCode atom) =? 6)) then (argNext keep scan atom depth pieces)",
        replacement: "(if false then (argNext keep scan atom depth pieces)",
        vector: "white space in a use",
    },
    Plant {
        label: "a data payload drops white space inside its strings",
        site: "(braceWith ctx (expectControl ctx after control) true)",
        replacement: "(braceWith ctx (expectControl ctx after control) false)",
        vector: "core data",
    },
    Plant {
        label: "an escape does not hold the next scalar",
        site: "(record Quoting {index := next, inString := inside, escaped := true})",
        replacement: "(record Quoting {index := next, inString := inside, escaped := false})",
        vector: "an escaped quote in a string",
    },
    Plant {
        label: "an opening brace opens no level of an argument",
        site: "(argNext keep scan atom (depth + 1) (cons (lit \"{\") pieces))",
        replacement: "(argNext keep scan atom depth (cons (lit \"{\") pieces))",
        vector: "nested braces in a use",
    },
    Plant {
        label: "the closing brace of an inner level is not text",
        site: "(cons (lit \"}\") pieces)",
        replacement: "pieces",
        vector: "nested braces in a use",
    },
    Plant {
        label: "the opening brace is covered as a closing one",
        site: "(lit \"brace-open\")",
        replacement: "(lit \"brace-close\")",
        vector: "minimal",
    },
    Plant {
        label: "white space in metadata is covered",
        site: "(if ((PrimitiveScan::classCode atom) =? 6) then (ctor Option.some [(Nat * (List Row))] (((first state) + 1), (second state)))",
        replacement: "(if false then (ctor Option.some [(Nat * (List Row))] (((first state) + 1), (second state)))",
        vector: "white space in a use",
    },
    Plant {
        label: "a payload row ends with its first atom",
        site: "byteEnd := (project last byteEnd)",
        replacement: "byteEnd := (project head byteEnd)",
        vector: "core notation",
    },
    Plant {
        label: "notation keeps the white space before it",
        site: "(verbatimBody ctx (moved after (skipWs ctx (project after pos))))",
        replacement: "(verbatimBody ctx after)",
        vector: "core notation",
    },
    Plant {
        label: "notation does not stop at an end",
        site: "(if (isAtom atom 0 (lit \"\\\\end\")) then (ctor Option.none [(Nat * (List Bytes))])",
        replacement: "(if false then (ctor Option.none [(Nat * (List Bytes))])",
        vector: "core notation",
    },
    Plant {
        label: "a period inside braces ends a sentence",
        site: "(if (depth =? 0) then (same (project atom text) (lit \".\")) else false)",
        replacement: "(same (project atom text) (lit \".\"))",
        vector: "a period inside braces",
    },
    Plant {
        label: "a period inside an island ends a sentence",
        site: "(if (code =? 4) then (if (island =? 0) then",
        replacement: "(if (code =? 4) then (if true then",
        vector: "a period inside an inline island",
    },
    Plant {
        label: "either closer closes a display island",
        site: "(if (island =? 1) then (same spelled (lit \"\\\\)\")) else (same spelled (lit \"\\\\]\")))",
        replacement: "(if (same spelled (lit \"\\\\)\")) then true else (same spelled (lit \"\\\\]\")))",
        vector: "a display island that the other closer does not close",
    },
    Plant {
        label: "a closing brace at depth zero goes below zero",
        site: "(sentenceMove scan (prim subtract depth 1 : Nat) island)",
        replacement: "(sentenceMove scan (depth + 1) island)",
        vector: "a closing brace at depth zero",
    },
    Plant {
        label: "a begin inside braces stops a sentence",
        site: "(if (if (depth =? 0) then (if (same spelled (lit \"\\\\begin\")) then true else (same spelled (lit \"\\\\end\"))) else false) then (sentenceStop scan 2)",
        replacement: "(if (if (same spelled (lit \"\\\\begin\")) then true else (same spelled (lit \"\\\\end\"))) then (sentenceStop scan 2)",
        vector: "a begin inside braces",
    },
    Plant {
        label: "a corollary needs no proof",
        site: "| DeclKind.Corollary => true",
        replacement: "| DeclKind.Corollary => false",
        vector: "a corollary",
    },
    Plant {
        label: "axiom names sort the wrong way",
        site: "(if (before pending item) then",
        replacement: "(if (before item pending) then",
        vector: "names that sort by byte",
    },
    Plant {
        label: "a duplicate axiom name passes",
        site: "(if (PrimitiveScan::bytesInList piece (first state)) then",
        replacement: "(if false then",
        vector: "a duplicate name",
    },
    Plant {
        label: "the name refused after a first refusal replaces it",
        site: "| Option.some stopped => state",
        replacement: "| Option.some stopped => ((first state), (ctor Option.none [Bytes]))",
        vector: "a refused name after a duplicate",
    },
    Plant {
        label: "a Lean name may begin with a digit",
        site: "(if (second state) then (if startish then",
        replacement: "(if (second state) then (if true then",
        vector: "a name of a digit first",
    },
    Plant {
        label: "a Lean name may end in a dot",
        site: "(not (second final))",
        replacement: "true",
        vector: "a name with a trailing dot",
    },
    Plant {
        label: "a component ID may begin in upper case",
        site: "(PrimitiveScan::inRange head (97 : UInt8) (122 : UInt8))",
        replacement: "(PrimitiveScan::inRange head (65 : UInt8) (122 : UInt8))",
        vector: "a section with an upper-case id",
    },
    Plant {
        label: "a component ID holds no hyphen",
        site: "(prim equal byte (45 : UInt8) : Bool)",
        replacement: "false",
        vector: "a section with a hyphen",
    },
    Plant {
        label: "a label takes no sign",
        site: "(if (PrimitiveScan::byteIs text 0 (43 : UInt8)) then 1 else 0)",
        replacement: "0",
        vector: "a premise labeled signed",
    },
    Plant {
        label: "a label refuses the largest number",
        site: "18446744073709551615",
        replacement: "18446744073709551614",
        vector: "a premise labeled the largest",
    },
    Plant {
        label: "a sign alone is the label zero",
        site: "(if (start <? (prim length text : Nat)) then",
        replacement: "(if true then",
        vector: "a premise labeled a sign alone",
    },
    Plant {
        label: "the nesting of proofs admits one more",
        site: "(if ((project ctx maxDepth) <? depth) then (fail",
        replacement: "(if (((project ctx maxDepth) + 1) <? depth) then (fail",
        vector: "a nested proof under a limit of one",
    },
    Plant {
        label: "the nesting of sections admits one more",
        site: "(if ((project ctx maxDepth) <? nested) then",
        replacement: "(if (((project ctx maxDepth) + 1) <? nested) then",
        vector: "two sections under a limit of one",
    },
    Plant {
        label: "a proof nests from zero",
        site: "(lit \"proof\") 1 frame (nil ProofFrame)",
        replacement: "(lit \"proof\") 0 frame (nil ProofFrame)",
        vector: "a proof under a limit of zero",
    },
    Plant {
        label: "a forward rule reads as backward",
        site: "(cons (false, (project rule arg)) (second state))",
        replacement: "(cons (true, (project rule arg)) (second state))",
        vector: "a rewrite forward",
    },
    Plant {
        label: "a simplify may have no rule",
        site: "(fail ctx ended (lit \"LLF5003\") (lit \"a simplify requires at least one rule\"))",
        replacement: "ended",
        vector: "a simplify without a rule",
    },
    Plant {
        label: "the relation of a step is covered as another owner",
        site: "(project relation arg) (lit \"step\")",
        replacement: "(project relation arg) (lit \"start\")",
        vector: "a calculation",
    },
    Plant {
        label: "an empty bind is one empty spelling",
        site: "(if ((prim length spelled : Nat) =? 0) then (nil Bytes) else (LexLeanStd.Text::splitOn spelled (59 : UInt8)))",
        replacement: "(LexLeanStd.Text::splitOn spelled (59 : UInt8))",
        vector: "cases",
    },
    Plant {
        label: "white space between begin and its brace hides an environment",
        site: "(let open : Nat := (skipWs ctx ((skipWs ctx position) + 1));",
        replacement: "(let open : Nat := ((skipWs ctx position) + 1);",
        vector: "a spaced begin of a premise",
    },
    Plant {
        label: "a later refusal replaces the first",
        site: "| Option.some earlier => ps",
        replacement: "| Option.some earlier => (record Ps {pos := (project ps pos), rows := (project ps rows), error := (ctor Option.some [PrimitiveScan::Rejection] (record PrimitiveScan::Rejection {code := code, message := message, place := (project earlier place), help := (project earlier help)}))})",
        vector: "two refusals: the first stands",
    },
    Plant {
        label: "a use after an import is not named",
        site: "(if (atControl ctx (project (first imports) pos) (lit \"\\\\useglossary\")) then",
        replacement: "(if false then",
        vector: "a use after an import",
    },
    Plant {
        label: "a second title passes",
        site: "(if (atControl ctx (project (project title ps) pos) (lit \"\\\\title\")) then",
        replacement: "(if false then",
        vector: "two titles",
    },
    Plant {
        label: "text after the module passes",
        site: "(let last : Ps := (match (atomAt ctx position)",
        replacement: "(let last : Ps := (match (atomAt ctx (position + (project ctx count)))",
        vector: "text after the end",
    },
    Plant {
        label: "the end of a declaration is before its last brace",
        site: "stop := (project ended pos)",
        replacement: "stop := (project ps pos)",
        vector: "a termdefinition",
    },
    Plant {
        label: "the parameters of a section end where they start",
        site: "((project (project given arg) start), (project (project given arg) stop))",
        replacement: "((project (project given arg) start), (project (project given arg) start))",
        vector: "a section with parameters",
    },
    Plant {
        label: "a refusal at the end of the file names no atom",
        site: "(if (position <? last) then position else last)",
        replacement: "position",
        vector: "a block left open",
    },
    Plant {
        label: "the span of an empty file ends on line two",
        site: "lineEnd := 1, columnEnd := 1",
        replacement: "lineEnd := 2, columnEnd := 1",
        vector: "empty",
    },
    Plant {
        label: "the name of a module is not held to its path",
        site: "(if (same spelled expectedName) then covered else",
        replacement: "(if true then covered else",
        vector: "another name",
    },
    Plant {
        label: "the first environment need not be lexlean",
        site: "(if (same (project began name) (lit \"lexlean\")) then",
        replacement: "(if true then",
        vector: "another first environment",
    },
];

fn plants(vectors: &[Vector]) {
    let mut missed = Vec::new();
    for plant in PLANTS {
        let vector = vectors
            .iter()
            .find(|vector| vector.label == plant.vector)
            .unwrap_or_else(|| panic!("{}: no vector `{}`", plant.label, plant.vector));
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/StructuralGrammar.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/StructuralGrammar.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[0].0);
        match super::evaluate(&low, &[vector.run().0], 1).first() {
            Some(first) => eprintln!(
                "BS-21: plant detected: {}: {}",
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

fn export(root_program: &super::Lowered) {
    let exported = harness::exported("bootstrap");
    let bundle = exported.bundle();
    let mut realized = 0usize;
    for exported_root in &exported.output.result.roots {
        if exported_root.root != format!("{MODULE}.{}", ROOTS[0].0) {
            continue;
        }
        let largest = exported_root
            .outputs
            .iter()
            .flat_map(|output| &output.certification.certificates)
            .map(|certificate| certificate.file.byte_length)
            .max()
            .expect("a certificate");
        eprintln!(
            "BS-21: {}: largest certificate part {largest} bytes",
            exported_root.root
        );
        assert!(
            largest <= CERTIFICATE_BUDGET,
            "{}: a certificate part of {largest} bytes is beyond {CERTIFICATE_BUDGET}",
            exported_root.root
        );
        for output in &exported_root.outputs {
            let package: Json = serde_json::from_slice(
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
                serde_json::to_value(&root_program.program).expect("a program is JSON"),
                "{}: the exported package realizes the program the differential ran",
                exported_root.root
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

pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("StructuralGrammar")
            && checked_units.units.contains_key("PrimitiveScan"),
        "StructuralGrammar and PrimitiveScan are linked modules of the project"
    );
    eligibility_of(&project, "StructuralGrammar", MODULE, &ROOTS);
    assert_eq!(
        sorts().keys().map(String::as_str).collect::<Vec<_>>(),
        [
            "Block",
            "DeclKind",
            "Mode",
            "Origin",
            "PolicyKind",
            "Proof",
            "ProofFrame"
        ],
        "the sorts of the module"
    );

    let vectors = named();
    let labels: BTreeSet<&str> = vectors.iter().map(|vector| vector.label.as_str()).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let mut messages = Vec::new();
    let runs: Vec<Run> = vectors
        .iter()
        .map(|vector| {
            let (run, refusal) = vector.run();
            messages.extend(refusal);
            run
        })
        .collect();
    for refusal in REFUSALS {
        assert!(
            messages.iter().any(|message| message.contains(refusal)),
            "no vector reaches the refusal `{refusal}`"
        );
    }
    let refused = messages.len();
    assert_eq!(
        (vectors.len(), refused, PLANTS.len()),
        (VECTORS, REFUSED, PLANT_COUNT),
        "the named vectors, those refused, and the planted defects"
    );
    let sources = super::scan::sources();
    let evaluator_modules = committed(&sources, EVALUATOR_ATOMS);
    let rustc_modules = committed(&sources, RUSTC_ATOMS);
    let seeded = if support::normative_host() {
        mutants(MUTANTS, &vectors, &sources)
    } else {
        Vec::new()
    };
    let count_ok = |corpus: &[Run]| corpus.iter().filter(|run| run.1["kind"] == "ok").count();
    eprintln!(
        "BS-21: {} vectors ({} refused), {} committed modules ({} accepted under the evaluator's window, {} under rustc's), {} mutants ({} accepted) ({:?})",
        vectors.len(),
        refused,
        evaluator_modules.len(),
        count_ok(&evaluator_modules),
        count_ok(&rustc_modules),
        seeded.len(),
        count_ok(&seeded),
        started.elapsed()
    );

    let checked = support::checked_project(&project);
    let root = super::lower(&checked, &project, MODULE, ROOTS[0].0);
    if support::normative_host() {
        for (label, corpus) in [
            ("vectors", &runs),
            ("committed modules", &evaluator_modules),
            ("mutants", &seeded),
        ] {
            let disagreements = super::evaluate(&root, corpus, 5);
            assert!(
                disagreements.is_empty(),
                "parseModule and parse_module disagree on the {label}: {disagreements:?}"
            );
            eprintln!(
                "BS-21: the evaluator agrees on {} {label} ({:?})",
                corpus.len(),
                started.elapsed()
            );
        }
        plants(&vectors);
    } else {
        eprintln!("BS-21: platform-bound host: the evaluator, the mutants, and the planted defects run on the normative host");
    }

    let mut compiled = runs;
    compiled.extend(rustc_modules);
    compiled.extend(seeded);
    let total = rustc_differential(
        "BS-21",
        &[(ROOTS[0].0, &root, compiled.as_slice())],
        "oracle",
    );
    eprintln!("BS-21: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-21") {
        return;
    }
    export(&root);
    eprintln!("BS-21: passed in {:?}", started.elapsed());
}
