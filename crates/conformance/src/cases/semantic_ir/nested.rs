//! SM-37: a language-1.2 `nested_induction` proof (SPEC.md §16.9, §17.10).

use serde_json::{json, Value};

use crate::support::{self, P};

/// The committed example's theorem, in the module data.
fn theorem(module: &mut Value) -> &mut Value {
    module["declarations"]
        .as_array_mut()
        .expect("declarations")
        .iter_mut()
        .find(|declaration| declaration["name"] == "leaves_le_size")
        .expect("the example's theorem")
}

/// The branches of one level of the proof: the constructors of `Json` at
/// level 0, then those of each auxiliary member of its recursive family.
fn branches(proof: &mut Value, level: usize) -> &mut Vec<Value> {
    let node = if level == 0 {
        &mut proof["branches"]
    } else {
        &mut proof["members"][level - 1]["branches"]
    };
    node.as_array_mut().expect("branches")
}

/// How many levels the example's proof has: `Json`, `List Json`,
/// `List (String * Json)`, and `String * Json`.
const LEVELS: usize = 4;

/// A copy of the committed example, locked for this compiler: the example
/// carries no lock, because every lock is regenerated on the identity current
/// at landing.
fn example_copy() -> P {
    let copy = P::copy_example("nested-induction");
    copy.relock();
    copy
}

/// A copy of the example whose theorem's proof the edit changed, and the
/// error of checking it.
fn planted(edit: &dyn Fn(&mut Value)) -> P {
    let copy = example_copy();
    let text = copy.read("src/Main.lex.tex");
    let mut module = support::module_json(&text);
    edit(&mut theorem(&mut module)["proof"]);
    copy.write(
        "src/Main.lex.tex",
        &support::with_module_json(&text, &module),
    );
    copy
}

/// The plant is refused by the linker (LLT4001) with a message that names the
/// misuse, and no backend output is written.
fn refuse(what: &str, edit: &dyn Fn(&mut Value), message: &str) {
    refuse_project(what, &planted(edit), message);
}

/// The project is refused by the linker (LLT4001) with a message that names
/// the misuse, and no backend output is written.
fn refuse_project(what: &str, project: &P, message: &str) {
    let error = project.check_fails_with("LLT4001");
    assert!(
        error.to_string().contains(message),
        "{what}: expected {message:?}, got {error}"
    );
    project.assert_no_backend_output();
}

/// A whole module, in the authoring notation, for the misuses that need a
/// different type or theorem than the example's.
fn module(declarations: &str) -> String {
    format!(
        "\\begin{{lexlean}}{{Main}}\n\\useglossary{{lexlean.std.nat@1.2.0}}\n\\title{{Natural number addition}}\n\\begin{{semanticmodule}}\n{declarations}\n\\end{{semanticmodule}}\n\\end{{lexlean}}\n"
    )
}

fn project_of(text: &str) -> P {
    let project = P::example();
    project.edit("lexlean.toml", "language = \"1.0\"", "language = \"1.2\"");
    project.write("src/Main.lex.tex", text);
    project.relock();
    project
}

/// The Lean text of every module of a project's build.
fn built_lean(project: &P) -> String {
    let built = project.build_ok();
    let build_dir = project.build_dir(&built.build_id.expect("a build id"));
    std::fs::read_dir(build_dir.join("modules").as_std_path())
        .expect("the modules")
        .flatten()
        .flat_map(|entry| {
            std::fs::read_dir(entry.path())
                .into_iter()
                .flatten()
                .flatten()
        })
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "lean"))
        .map(|entry| std::fs::read_to_string(entry.path()).expect("Lean text"))
        .collect::<String>()
}

/// A `Rose` of naturals, which holds a `List` of itself and has a type
/// argument, and a theorem over it by `nested_induction`.
const ROSE: &str = r"inductive{
  name: Rose,
  type_parameters: [Item],
  parameters: [],
  constructors: [{name: node, fields: ['Item, (List (Rose 'Item))]}]
}

definition{
  name: count,
  parameters: [rose : (Rose Nat)],
  result: Nat,
  body: (match rose | Rose.node label children => (1 + (countAll children))),
  mutual: Count,
  recursive_argument: rose
}

definition{
  name: countAll,
  parameters: [roses : (List (Rose Nat))],
  result: Nat,
  body: (match roses | List.nil => 0 | List.cons head tail => ((count head) + (countAll tail))),
  mutual: Count,
  recursive_argument: roses
}

theorem{
  name: count_positive,
  parameters: [rose : (Rose Nat)],
  statement: (1 <= (count rose)),
  proof: nested_induction{
    scrutinee: rose,
    branches: [
      {
        constructor: node,
        binders: [label, children, childrenHypothesis],
        proof: linear_arithmetic{definitions: [count]}
      }
    ],
    members: [
      {
        type: (List (Rose Nat)),
        binder: roses,
        statement: (0 <= (countAll roses)),
        branches: [
          {constructor: nil, binders: [], proof: linear_arithmetic{definitions: [countAll]}},
          {
            constructor: cons,
            binders: [head, tail, headHypothesis, tailHypothesis],
            proof: linear_arithmetic{definitions: [countAll]}
          }
        ]
      }
    ]
  },
  axioms: [Quot.sound, propext]
}";

/// The same `Rose`, and a theorem over a `Rose` of `Rose`s of naturals: the
/// recursor of `Rose` has two motives at any instantiation, so the family of
/// `Rose (Rose Nat)` is itself and `List (Rose (Rose Nat))`, and `Rose Nat`,
/// which the field `label` becomes, is another type that joins nothing.
const ROSE_OF_ROSE: &str = r"inductive{
  name: Rose,
  type_parameters: [Item],
  parameters: [],
  constructors: [{name: node, fields: ['Item, (List (Rose 'Item))]}]
}

theorem{
  name: rose_of_rose_refl,
  parameters: [rose : (Rose (Rose Nat))],
  statement: (rose = rose),
  proof: nested_induction{
    scrutinee: rose,
    branches: [
      {constructor: node, binders: [label, children, childrenHypothesis], proof: reflexivity{}}
    ],
    members: [
      {
        type: (List (Rose (Rose Nat))),
        binder: roses,
        statement: (roses = roses),
        branches: [
          {constructor: nil, binders: [], proof: reflexivity{}},
          {
            constructor: cons,
            binders: [head, tail, headHypothesis, tailHypothesis],
            proof: reflexivity{}
          }
        ]
      }
    ]
  },
  axioms: []
}";

/// A `Chain` of one optional `Chain`: the family holds `Option Chain`.
const OPTION_CHAIN: &str = r"inductive{
  name: Chain,
  type_parameters: [],
  parameters: [],
  constructors: [{name: link, fields: [(Option Chain)]}]
}

theorem{
  name: chain_refl,
  parameters: [chain : Chain],
  statement: (chain = chain),
  proof: nested_induction{
    scrutinee: chain,
    branches: [
      {constructor: link, binders: [rest, restHypothesis], proof: reflexivity{}}
    ],
    members: [
      {
        type: (Option Chain),
        binder: choice,
        statement: (choice = choice),
        branches: [
          {constructor: some, binders: [value, valueHypothesis], proof: reflexivity{}},
          {constructor: none, binders: [], proof: reflexivity{}}
        ]
      }
    ]
  },
  axioms: []
}";

/// An `Attempt` that wraps a `Result` of itself or a natural number: the
/// family holds `Result Attempt Nat`, whose `ok` is the member.
const RESULT_ATTEMPT: &str = r"inductive{
  name: Attempt,
  type_parameters: [],
  parameters: [],
  constructors: [{name: wrap, fields: [(Result Attempt Nat)]}]
}

theorem{
  name: attempt_refl,
  parameters: [attempt : Attempt],
  statement: (attempt = attempt),
  proof: nested_induction{
    scrutinee: attempt,
    branches: [
      {constructor: wrap, binders: [outcome, outcomeHypothesis], proof: reflexivity{}}
    ],
    members: [
      {
        type: (Result Attempt Nat),
        binder: settled,
        statement: (settled = settled),
        branches: [
          {constructor: ok, binders: [value, valueHypothesis], proof: reflexivity{}},
          {constructor: error, binders: [code], proof: reflexivity{}}
        ]
      }
    ]
  },
  axioms: []
}";

/// The positional goals of the generated Lean, in order.
fn goal_lines(lean: &str) -> Vec<&str> {
    lean.lines()
        .filter(|line| line.trim_start().starts_with("next"))
        .map(str::trim)
        .collect()
}

/// Run the case of `SM-37`.
pub fn run() {
    let accepted = example_copy();
    let checked = support::checked_project(&accepted);
    let core = serde_json::to_value(
        checked.modules["Main"]
            .document
            .semantic
            .as_ref()
            .expect("a semantic module"),
    )
    .expect("module JSON");
    support::assert_schema("semantic-module-v2", "a nested_induction proof", &core);
    // The committed source is the formatter's fixed point, the formatter
    // keeps the proof, and the notation reads back to the same module.
    accepted.fmt_check_ok();
    let source = accepted.read("src/Main.lex.tex");
    assert!(source.contains("\"kind\":\"nested_induction\""), "{source}");
    let notation = lexlean::sdk::to_notation(&source).expect("the notation");
    assert!(notation.contains("nested_induction{"), "{notation}");
    assert_eq!(
        lexlean::sdk::read_module(&notation).expect("reads back"),
        lexlean::sdk::read_module(&source).expect("reads"),
        "the notation is the same module"
    );
    // The lowering: the recursor Lean generated, the propositions of the
    // auxiliary members as its motives, and one positional goal for each
    // minor premise, in the order of the recursor (the constructors of
    // `Json`, then `nil` and `cons` of `List Json`, `nil` and `cons` of
    // `List (String * Json)`, and `mk` of the pair).
    let lean = built_lean(&accepted);
    assert_eq!(
        goal_lines(&lean),
        [
            "next =>",
            "next content =>",
            "next items itemsHypothesis =>",
            "next entries entriesHypothesis =>",
            "next =>",
            "next head tail headHypothesis tailHypothesis =>",
            "next =>",
            "next head tail headHypothesis tailHypothesis =>",
            "next key content contentHypothesis =>",
        ],
        "{lean}"
    );
    assert!(
        lean.contains("induction value using Json.rec\n")
            && lean.contains("(motive_2 := fun items =>")
            && lean.contains("(motive_3 := fun entries =>")
            && lean.contains("(motive_4 := fun entry =>"),
        "{lean}"
    );
    // The pinned Lean checks the theorem, with exactly the axioms its policy
    // states.
    support::verify_ok_backed("SM-37", &accepted);
    // The goals follow the recursor, not the order the branches are written
    // in: every level written in reverse lowers to the same Lean.
    let reversed = planted(&|proof| {
        for level in 0..LEVELS {
            branches(proof, level).reverse();
        }
    });
    reversed.check_ok();
    assert_eq!(
        built_lean(&reversed),
        lean,
        "the branch order is not the recursor's"
    );
    // A type with a type argument, and a theorem over it.
    let rose = project_of(&module(ROSE));
    rose.check_ok();
    let rose_lean = built_lean(&rose);
    assert!(
        rose_lean.contains("induction rose using Rose.rec\n")
            && rose_lean.contains("(motive_2 := fun roses =>"),
        "{rose_lean}"
    );
    support::verify_ok_backed("SM-37", &rose);
    // A type used at one of its own types: two motives, not four.
    let nested = project_of(&module(ROSE_OF_ROSE));
    nested.check_ok();
    let nested_lean = built_lean(&nested);
    assert!(
        nested_lean.contains("induction rose using Rose.rec\n")
            && nested_lean.contains("(motive_2 := fun roses =>")
            && !nested_lean.contains("(motive_3 :="),
        "{nested_lean}"
    );
    support::verify_ok_backed("SM-37", &nested);
    // `List (Rose Nat)` is not a member of the family of `Rose (Rose Nat)`.
    let extra = project_of(&module(&ROSE_OF_ROSE.replace(
        "members: [\n      {\n        type: (List (Rose (Rose Nat))),",
        "members: [\n      {\n        type: (List (Rose Nat)),",
    )));
    refuse_project(
        "a member of another instantiation",
        &extra,
        "nested_induction is missing the member",
    );
    // A member under `Option` and one under `Result`: the minor premises of
    // `Option` are `none` then `some`, and those of `Result` (`Except`) are
    // `error` then `ok`. The goals have different binder counts, so a
    // recursor whose order the table has wrong would not elaborate.
    for (what, source, goals_expected, motive) in [
        (
            "an Option member",
            OPTION_CHAIN,
            [
                "next rest restHypothesis =>",
                "next =>",
                "next value valueHypothesis =>",
            ],
            "induction chain using Chain.rec\n",
        ),
        (
            "a Result member",
            RESULT_ATTEMPT,
            [
                "next outcome outcomeHypothesis =>",
                "next code =>",
                "next value valueHypothesis =>",
            ],
            "induction attempt using Attempt.rec\n",
        ),
    ] {
        let project = project_of(&module(source));
        project.check_ok();
        let text = built_lean(&project);
        assert_eq!(goal_lines(&text), goals_expected, "{what}: {text}");
        assert!(
            text.contains(motive)
                && text.contains("(motive_2 := fun")
                && !text.contains("(motive_3 :="),
            "{what}: {text}"
        );
        support::verify_ok_backed("SM-37", &project);
    }

    // The misuses are refused by the linker, each as LLT4001 before any
    // backend output.
    // The members of the recursive family: missing, reordered, repeated, and
    // not of the family, each as Lean's recursor takes them.
    for member in 0..LEVELS - 1 {
        refuse(
            &format!("member {member} omitted"),
            &|proof| {
                proof["members"]
                    .as_array_mut()
                    .expect("members")
                    .remove(member);
            },
            "nested_induction is missing the member",
        );
    }
    for (first, second) in [(0, 1), (1, 2), (0, 2)] {
        refuse(
            &format!("members {first} and {second} swapped"),
            &|proof| {
                proof["members"]
                    .as_array_mut()
                    .expect("members")
                    .swap(first, second);
            },
            "nested_induction members are out of order",
        );
    }
    refuse(
        "the first member repeated",
        &|proof| {
            let members = proof["members"].as_array_mut().expect("members");
            members[1] = members[0].clone();
        },
        "nested_induction names the member `List (Json)` more than once",
    );
    refuse(
        "a member outside the family",
        &|proof| {
            let members = proof["members"].as_array_mut().expect("members");
            let mut extra = members[0].clone();
            extra["type"] = json!({"kind": "list", "element": {"kind": "nat"}});
            members.push(extra);
        },
        "which is not a member of the recursive family of `Json`",
    );
    refuse(
        "two members that share a binder",
        &|proof| {
            proof["members"][1]["binder"] = proof["members"][0]["binder"].clone();
        },
        "is used by two members",
    );
    refuse(
        "a proposition that is not a proposition",
        &|proof| {
            proof["members"][0]["statement"] = json!({
                "kind": "call",
                "function": {"name": "sizeList"},
                "arguments": [{"kind": "var", "name": "items"}]
            });
        },
        "has type Nat, expected Prop",
    );
    // Each level: a missing branch, a branch with one binder too many and
    // one too few, a repeated branch, and a branch for no constructor.
    for level in 0..LEVELS {
        let count = {
            let mut module = support::module_json(&accepted.read("src/Main.lex.tex"));
            let count = branches(&mut theorem(&mut module)["proof"], level).len();
            count
        };
        for index in 0..count {
            refuse(
                &format!("level {level}: branch {index} missing"),
                &|proof| {
                    branches(proof, level).remove(index);
                },
                "are not exhaustive",
            );
            refuse(
                &format!("level {level}: branch {index} with a binder too many"),
                &|proof| {
                    branches(proof, level)[index]["binders"]
                        .as_array_mut()
                        .expect("binders")
                        .push("surplus".into());
                },
                "binder(s), received",
            );
            let has_binders = {
                let mut module = support::module_json(&accepted.read("src/Main.lex.tex"));
                !branches(&mut theorem(&mut module)["proof"], level)[index]["binders"]
                    .as_array()
                    .expect("binders")
                    .is_empty()
            };
            if has_binders {
                refuse(
                    &format!("level {level}: branch {index} with a binder too few"),
                    &|proof| {
                        branches(proof, level)[index]["binders"]
                            .as_array_mut()
                            .expect("binders")
                            .pop();
                    },
                    "binder(s), received",
                );
            }
        }
        refuse(
            &format!("level {level}: a branch repeated"),
            &|proof| {
                let level = branches(proof, level);
                level.push(level[0].clone());
            },
            "duplicate proof branch",
        );
        refuse(
            &format!("level {level}: a branch for no constructor"),
            &|proof| {
                branches(proof, level)[0]["constructor"] = "absent".into();
            },
            "is not a constructor of",
        );
    }
    // `induction` on the same type is still refused, and says which form
    // takes it.
    refuse(
        "induction on a nested type",
        &|proof| {
            proof["kind"] = "induction".into();
            proof.as_object_mut().expect("proof").remove("members");
        },
        "its type holds itself under a container, so use nested_induction (or cases)",
    );
    // A language-1.1 module has no such proof.
    {
        let on_nat = "theorem{name: t, parameters: [n : Nat], statement: (n = n), proof: nested_induction{scrutinee: n, branches: [{constructor: zero, binders: [], proof: reflexivity{}}], members: [{type: (List Nat), binder: xs, statement: (xs = xs), branches: [{constructor: nil, binders: [], proof: reflexivity{}}]}]}, axioms: []}";
        let text = project_of(&module(on_nat)).read("src/Main.lex.tex");
        let old = P::example();
        old.edit("lexlean.toml", "language = \"1.0\"", "language = \"1.1\"");
        old.write_raw(
            "src/Main.lex.tex",
            &text
                .replace("@1.2.0", "@1.1.0")
                .replace("lexlean/semantic-module/2", "lexlean/semantic-module/1"),
        );
        old.relock();
        refuse_project(
            "language 1.1",
            &old,
            "`nested_induction proof` is a language-1.2 construct",
        );
    }
    // The scrutinee must be a nested inductive of the document that is in no
    // mutual group, and a parameter of the theorem.
    let json = "inductive{\n  name: Json,\n  type_parameters: [],\n  parameters: [],\n  constructors: [{name: null, fields: []}, {name: array, fields: [(List Json)]}]\n}\n\n";
    let list_member = "members: [{type: (List Json), binder: items, statement: (items = items), branches: [{constructor: nil, binders: [], proof: reflexivity{}}, {constructor: cons, binders: [head, tail, headHypothesis, tailHypothesis], proof: reflexivity{}}]}]";
    for (what, declarations, message) in [
        (
            "a natural number",
            "theorem{name: t, parameters: [n : Nat], statement: (n = n), proof: nested_induction{scrutinee: n, branches: [{constructor: zero, binders: [], proof: reflexivity{}}], members: [{type: (List Nat), binder: xs, statement: (xs = xs), branches: [{constructor: nil, binders: [], proof: reflexivity{}}]}]}, axioms: []}".to_owned(),
            "is not of a nested inductive type of the document",
        ),
        (
            "a type with no nested field",
            format!("inductive{{\n  name: Tree,\n  type_parameters: [],\n  parameters: [],\n  constructors: [{{name: leaf, fields: []}}, {{name: node, fields: [Tree, Tree]}}]\n}}\n\ntheorem{{name: t, parameters: [tree : Tree], statement: (tree = tree), proof: nested_induction{{scrutinee: tree, branches: [{{constructor: leaf, binders: [], proof: reflexivity{{}}}}, {{constructor: node, binders: [left, right, leftHypothesis, rightHypothesis], proof: reflexivity{{}}}}], {list_member}}}, axioms: []}}"),
            "holds no member of itself under List, Option, Result, or a product",
        ),
        (
            "a type in a mutual group",
            format!("inductive{{\n  name: Expr,\n  type_parameters: [],\n  parameters: [],\n  constructors: [{{name: literal, fields: [Nat]}}, {{name: block, fields: [(List Stmt)]}}],\n  mutual: Syntax\n}}\n\ninductive{{\n  name: Stmt,\n  type_parameters: [],\n  parameters: [],\n  constructors: [{{name: assign, fields: [String, Expr]}}],\n  mutual: Syntax\n}}\n\ntheorem{{name: t, parameters: [expr : Expr], statement: (expr = expr), proof: nested_induction{{scrutinee: expr, branches: [{{constructor: literal, binders: [value], proof: reflexivity{{}}}}], {list_member}}}, axioms: []}}"),
            "which is in a mutual group",
        ),
        (
            "a local that is not a parameter of the theorem",
            format!("{json}inductive{{\n  name: Box,\n  type_parameters: [],\n  parameters: [],\n  constructors: [{{name: wrap, fields: [Json]}}]\n}}\n\ntheorem{{name: t, parameters: [box : Box], statement: (box = box), proof: cases{{scrutinee: box, branches: [{{constructor: wrap, binders: [inner], proof: nested_induction{{scrutinee: inner, branches: [{{constructor: null, binders: [], proof: reflexivity{{}}}}, {{constructor: array, binders: [items, itemsHypothesis], proof: reflexivity{{}}}}], {list_member}}}}}]}}, axioms: []}}"),
            "is not a parameter of the theorem",
        ),
    ] {
        refuse_project(what, &project_of(&module(&declarations)), message);
    }
}
