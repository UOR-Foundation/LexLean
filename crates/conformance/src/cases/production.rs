//! The `production` suite: PD-01..PD-07, language-1.2 production
//! eligibility (SPEC.md §17.13).

use std::collections::BTreeSet;

use crate::support::{self, P};
use lexlean::production::eligibility::{
    primitive_index, primitive_key, term_key, type_key, PRIMITIVES, STRUCTURAL_KEYS,
};
use lexlean::production::registry;

const MAIN: &str = "src/Main.lex.tex";

/// The eligibility report of the committed production example's `Main`.
fn main_report(project: &P) -> serde_json::Value {
    let build = support::rendered(project);
    let (_, bytes) = build
        .files
        .iter()
        .find(|(path, _)| path == "production/Production/Main.eligibility.json")
        .expect("the production example publishes Main's eligibility report");
    serde_json::from_slice(bytes).expect("report is JSON")
}

fn root<'a>(report: &'a serde_json::Value, name: &str) -> &'a serde_json::Value {
    report["roots"]
        .as_array()
        .expect("roots")
        .iter()
        .find(|root| root["root"] == format!("Production.Main.{name}"))
        .unwrap_or_else(|| panic!("root {name} is reported"))
}

fn strings(value: &serde_json::Value) -> Vec<String> {
    value
        .as_array()
        .expect("array")
        .iter()
        .map(|item| item.as_str().expect("string").to_owned())
        .collect()
}

fn closure(root: &serde_json::Value) -> Vec<String> {
    root["runtime_closure"]
        .as_array()
        .expect("closure")
        .iter()
        .map(|member| member["instance"].as_str().expect("instance").to_owned())
        .collect()
}

fn effects(root: &serde_json::Value, target: &str) -> Vec<String> {
    root["targets"]
        .as_array()
        .expect("targets")
        .iter()
        .find(|row| row["target"] == target)
        .unwrap_or_else(|| panic!("target {target} is reported"))["effects"]
        .as_array()
        .expect("effects")
        .iter()
        .map(|row| row["effect"].as_str().expect("effect").to_owned())
        .collect()
}

/// The `kind` (or `operation`) values of one `$defs` entry of the
/// language-1.2 semantic-module schema: every `const` or `enum` of that
/// property anywhere beneath the entry.
fn schema_kinds(definition: &str, property: &str) -> BTreeSet<String> {
    fn walk(value: &serde_json::Value, property: &str, out: &mut BTreeSet<String>) {
        match value {
            serde_json::Value::Object(map) => {
                if let Some(node) = map.get("properties").and_then(|p| p.get(property)) {
                    if let Some(constant) = node.get("const").and_then(|c| c.as_str()) {
                        out.insert(constant.to_owned());
                    }
                    for item in node
                        .get("enum")
                        .and_then(|e| e.as_array())
                        .into_iter()
                        .flatten()
                    {
                        out.insert(item.as_str().expect("enum string").to_owned());
                    }
                }
                for child in map.values() {
                    walk(child, property, out);
                }
            }
            serde_json::Value::Array(items) => {
                for item in items {
                    walk(item, property, out);
                }
            }
            _ => {}
        }
    }
    let schema = support::schema("semantic-module-v2");
    let mut kinds = BTreeSet::new();
    walk(&schema["$defs"][definition], property, &mut kinds);
    kinds
}

/// One minimal well-formed value of every type kind and every term kind, as
/// semantic-module JSON. The analysis must map each to its own registry key.
const TYPE_SAMPLES: [&str; 27] = [
    r#"{"kind":"type"}"#,
    r#"{"kind":"parameter","name":"T"}"#,
    r#"{"kind":"nat"}"#,
    r#"{"kind":"bool"}"#,
    r#"{"kind":"prop"}"#,
    r#"{"kind":"unit"}"#,
    r#"{"kind":"int"}"#,
    r#"{"kind":"int8"}"#,
    r#"{"kind":"int16"}"#,
    r#"{"kind":"int32"}"#,
    r#"{"kind":"int64"}"#,
    r#"{"kind":"uint8"}"#,
    r#"{"kind":"uint16"}"#,
    r#"{"kind":"uint32"}"#,
    r#"{"kind":"uint64"}"#,
    r#"{"kind":"string"}"#,
    r#"{"kind":"bytes"}"#,
    r#"{"kind":"ordering"}"#,
    r#"{"kind":"option","value":{"kind":"nat"}}"#,
    r#"{"kind":"result","ok":{"kind":"nat"},"error":{"kind":"nat"}}"#,
    r#"{"kind":"list","element":{"kind":"nat"}}"#,
    r#"{"kind":"named","member":{"name":"T"},"arguments":[]}"#,
    r#"{"kind":"product","left":{"kind":"nat"},"right":{"kind":"nat"}}"#,
    r#"{"kind":"function","parameters":[{"kind":"nat"}],"result":{"kind":"nat"}}"#,
    r#"{"kind":"map","key":{"kind":"nat"},"value":{"kind":"nat"}}"#,
    r#"{"kind":"set","element":{"kind":"nat"}}"#,
    r#"{"kind":"contract_violation"}"#,
];

const TERM_SAMPLES: [&str; 42] = [
    r#"{"kind":"var","name":"x"}"#,
    r#"{"kind":"nat","value":"0"}"#,
    r#"{"kind":"integer","representation":"int","value":"0"}"#,
    r#"{"kind":"string","value":""}"#,
    r#"{"kind":"bytes","hex":""}"#,
    r#"{"kind":"primitive","operation":"length","arguments":[],"result":{"kind":"nat"}}"#,
    r#"{"kind":"bool","value":true}"#,
    r#"{"kind":"unit"}"#,
    r#"{"kind":"nil","element":{"kind":"nat"}}"#,
    r#"{"kind":"cons","head":{"kind":"unit"},"tail":{"kind":"unit"}}"#,
    r#"{"kind":"record","type":{"name":"T"},"fields":[]}"#,
    r#"{"kind":"constructor","constructor":{"name":"T.c"},"arguments":[]}"#,
    r#"{"kind":"instance_value","class":{"name":"C"},"arguments":[],"resolved":{"name":"i"}}"#,
    r#"{"kind":"project","value":{"kind":"unit"},"field":"f"}"#,
    r#"{"kind":"call","function":{"name":"f"},"arguments":[]}"#,
    r#"{"kind":"if","condition":{"kind":"unit"},"then_value":{"kind":"unit"},"else_value":{"kind":"unit"}}"#,
    r#"{"kind":"match","scrutinee":{"kind":"unit"},"branches":[]}"#,
    r#"{"kind":"eq","left":{"kind":"unit"},"right":{"kind":"unit"}}"#,
    r#"{"kind":"le","left":{"kind":"unit"},"right":{"kind":"unit"}}"#,
    r#"{"kind":"lt","left":{"kind":"unit"},"right":{"kind":"unit"}}"#,
    r#"{"kind":"add","left":{"kind":"unit"},"right":{"kind":"unit"}}"#,
    r#"{"kind":"beq","left":{"kind":"unit"},"right":{"kind":"unit"}}"#,
    r#"{"kind":"ble","left":{"kind":"unit"},"right":{"kind":"unit"}}"#,
    r#"{"kind":"blt","left":{"kind":"unit"},"right":{"kind":"unit"}}"#,
    r#"{"kind":"and","left":{"kind":"unit"},"right":{"kind":"unit"}}"#,
    r#"{"kind":"prop_and","left":{"kind":"unit"},"right":{"kind":"unit"}}"#,
    r#"{"kind":"or","left":{"kind":"unit"},"right":{"kind":"unit"}}"#,
    r#"{"kind":"not","value":{"kind":"unit"}}"#,
    r#"{"kind":"implies","premise":{"kind":"unit"},"conclusion":{"kind":"unit"}}"#,
    r#"{"kind":"iff","left":{"kind":"unit"},"right":{"kind":"unit"}}"#,
    r#"{"kind":"forall","binder":{"name":"x","type":{"kind":"nat"}},"body":{"kind":"unit"}}"#,
    r#"{"kind":"let","binder":{"name":"x","type":{"kind":"nat"}},"value":{"kind":"unit"},"body":{"kind":"unit"}}"#,
    r#"{"kind":"pair","left":{"kind":"unit"},"right":{"kind":"unit"}}"#,
    r#"{"kind":"first","value":{"kind":"unit"}}"#,
    r#"{"kind":"second","value":{"kind":"unit"}}"#,
    r#"{"kind":"lambda","parameters":[],"captures":[],"body":{"kind":"unit"}}"#,
    r#"{"kind":"apply","function":{"kind":"unit"},"arguments":[]}"#,
    r#"{"kind":"function_ref","function":{"name":"f"}}"#,
    r#"{"kind":"map_literal","key":{"kind":"nat"},"value":{"kind":"nat"},"entries":[]}"#,
    r#"{"kind":"set_literal","element":{"kind":"nat"},"elements":[]}"#,
    r#"{"kind":"graph_literal","node":{"kind":"nat"},"nodes":[],"edges":[]}"#,
    r#"{"kind":"checked_apply","model":{"name":"M"},"type_arguments":[],"arguments":[],"checks":[]}"#,
];

/// Every registry row as it was reviewed: construct key, disposition,
/// allocation, overflowing representations, and recursion. A changed column
/// is a change to LexLean's production contract, so it must change here too.
#[rustfmt::skip]
const REGISTRY_ROWS: [(&str, &str, bool, &[&str], bool); 137] = [
    ("type.type", "formal-only", false, &[], false),
    ("type.parameter", "runtime", false, &[], false),
    ("type.nat", "runtime", false, &[], false),
    ("type.bool", "runtime", false, &[], false),
    ("type.prop", "formal-only", false, &[], false),
    ("type.unit", "runtime", false, &[], false),
    ("type.int", "runtime", false, &[], false),
    ("type.int8", "runtime", false, &[], false),
    ("type.int16", "runtime", false, &[], false),
    ("type.int32", "runtime", false, &[], false),
    ("type.int64", "runtime", false, &[], false),
    ("type.uint8", "runtime", false, &[], false),
    ("type.uint16", "runtime", false, &[], false),
    ("type.uint32", "runtime", false, &[], false),
    ("type.uint64", "runtime", false, &[], false),
    ("type.string", "runtime", true, &[], false),
    ("type.bytes", "runtime", true, &[], false),
    ("type.ordering", "runtime", false, &[], false),
    ("type.option", "runtime", false, &[], false),
    ("type.result", "runtime", false, &[], false),
    ("type.list", "runtime", true, &[], false),
    ("type.named", "runtime", false, &[], false),
    ("type.product", "runtime", false, &[], false),
    ("type.function", "runtime", false, &[], false),
    ("type.map", "runtime", true, &[], false),
    ("type.set", "runtime", true, &[], false),
    ("type.contract_violation", "runtime", false, &[], false),
    ("term.var", "runtime", false, &[], false),
    ("term.nat", "runtime", false, &[], false),
    ("term.integer", "runtime", false, &[], false),
    ("term.string", "runtime", true, &[], false),
    ("term.bytes", "runtime", true, &[], false),
    ("term.primitive", "runtime", false, &[], false),
    ("term.bool", "runtime", false, &[], false),
    ("term.unit", "runtime", false, &[], false),
    ("term.nil", "runtime", false, &[], false),
    ("term.cons", "runtime", true, &[], false),
    ("term.record", "runtime", false, &[], false),
    ("term.constructor", "runtime", false, &[], false),
    ("term.instance_value", "runtime", false, &[], false),
    ("term.project", "runtime", false, &[], false),
    ("term.call", "runtime", false, &[], false),
    ("term.if", "runtime", false, &[], false),
    ("term.match", "runtime", false, &[], false),
    ("term.eq", "formal-only", false, &[], false),
    ("term.le", "formal-only", false, &[], false),
    ("term.lt", "formal-only", false, &[], false),
    ("term.add", "runtime", false, &["nat"], false),
    ("term.beq", "runtime", false, &[], false),
    ("term.ble", "runtime", false, &[], false),
    ("term.blt", "runtime", false, &[], false),
    ("term.and", "runtime", false, &[], false),
    ("term.prop_and", "formal-only", false, &[], false),
    ("term.or", "runtime", false, &[], false),
    ("term.not", "runtime", false, &[], false),
    ("term.implies", "formal-only", false, &[], false),
    ("term.iff", "formal-only", false, &[], false),
    ("term.forall", "formal-only", false, &[], false),
    ("term.let", "runtime", false, &[], false),
    ("term.pair", "runtime", false, &[], false),
    ("term.first", "runtime", false, &[], false),
    ("term.second", "runtime", false, &[], false),
    ("term.lambda", "runtime", false, &[], false),
    ("term.apply", "runtime", false, &[], false),
    ("term.function_ref", "runtime", false, &[], false),
    ("term.map_literal", "runtime", true, &[], false),
    ("term.set_literal", "runtime", true, &[], false),
    ("term.graph_literal", "runtime", true, &[], false),
    ("term.checked_apply", "runtime", false, &[], false),
    ("constructor.nat_succ", "runtime", false, &["nat"], false),
    ("primitive.subtract", "runtime", false, &["int"], false),
    ("primitive.multiply", "runtime", false, &["int", "nat"], false),
    ("primitive.quotient", "runtime", false, &["int"], false),
    ("primitive.remainder", "runtime", false, &[], false),
    ("primitive.negate", "runtime", false, &["int"], false),
    ("primitive.checked_convert", "runtime", false, &[], false),
    ("primitive.checked_add", "runtime", false, &[], false),
    ("primitive.checked_subtract", "runtime", false, &[], false),
    ("primitive.checked_multiply", "runtime", false, &[], false),
    ("primitive.checked_negate", "runtime", false, &[], false),
    ("primitive.checked_quotient", "runtime", false, &[], false),
    ("primitive.bit_and", "runtime", false, &[], false),
    ("primitive.bit_or", "runtime", false, &[], false),
    ("primitive.bit_xor", "runtime", false, &[], false),
    ("primitive.bit_not", "runtime", false, &[], false),
    ("primitive.shift_left", "runtime", false, &[], false),
    ("primitive.shift_right", "runtime", false, &[], false),
    ("primitive.append", "runtime", true, &[], false),
    ("primitive.length", "runtime", false, &[], false),
    ("primitive.index", "runtime", false, &[], false),
    ("primitive.slice", "runtime", true, &[], false),
    ("primitive.utf8_encode", "runtime", true, &[], false),
    ("primitive.utf8_decode", "runtime", true, &[], false),
    ("primitive.compare_bytes", "runtime", false, &[], false),
    ("primitive.equal", "runtime", false, &[], false),
    ("primitive.split_exact", "runtime", true, &[], false),
    ("primitive.join", "runtime", true, &[], false),
    ("primitive.parse_decimal", "runtime", false, &["int", "nat"], false),
    ("primitive.format_decimal", "runtime", true, &[], false),
    ("primitive.map_insert", "runtime", true, &[], false),
    ("primitive.map_remove", "runtime", true, &[], false),
    ("primitive.map_lookup", "runtime", false, &[], false),
    ("primitive.map_contains", "runtime", false, &[], false),
    ("primitive.map_size", "runtime", false, &[], false),
    ("primitive.map_keys", "runtime", true, &[], false),
    ("primitive.map_values", "runtime", true, &[], false),
    ("primitive.map_entries", "runtime", false, &[], false),
    ("primitive.map_fold", "runtime", false, &[], false),
    ("primitive.set_insert", "runtime", true, &[], false),
    ("primitive.set_remove", "runtime", true, &[], false),
    ("primitive.set_contains", "runtime", false, &[], false),
    ("primitive.set_size", "runtime", false, &[], false),
    ("primitive.set_elements", "runtime", false, &[], false),
    ("primitive.set_union", "runtime", true, &[], false),
    ("primitive.set_intersection", "runtime", true, &[], false),
    ("primitive.set_difference", "runtime", true, &[], false),
    ("primitive.set_fold", "runtime", false, &[], false),
    ("primitive.list_fold", "runtime", false, &[], false),
    ("primitive.iterate", "runtime", false, &[], false),
    ("primitive.iterate_until", "runtime", false, &[], false),
    ("primitive.graph_successors", "runtime", false, &[], false),
    ("primitive.graph_reachable", "runtime", true, &[], false),
    ("primitive.graph_topological", "runtime", true, &[], false),
    ("primitive.less_than", "runtime", false, &[], false),
    ("declaration.structure", "runtime", false, &[], false),
    ("declaration.class", "runtime", false, &[], false),
    ("declaration.instance", "runtime", false, &[], false),
    ("declaration.inductive", "runtime", false, &[], false),
    ("declaration.inductive.recursive", "runtime", true, &[], false),
    ("declaration.definition", "runtime", false, &[], false),
    ("declaration.definition.recursive", "runtime", false, &[], true),
    ("declaration.theorem", "erased", false, &[], false),
    ("declaration.artifact", "runtime", false, &[], false),
    ("declaration.contract", "erased", false, &[], false),
    ("declaration.realization", "runtime", false, &[], false),
    ("declaration.evidence", "erased", false, &[], false),
    ("declaration.model", "runtime", false, &[], false),
];

/// Run the case for one PD conformance ID.
///
/// # Panics
///
/// Panics when the case's assertion fails, and for an unwired ID.
#[allow(clippy::too_many_lines)]
pub fn run(id: &str) {
    match id {
        "PD-01" => {
            let registry = registry().expect("the embedded production registry parses");
            // Every key the analysis can produce has exactly one row, and
            // every row names a key the analysis can produce.
            for (index, operation) in PRIMITIVES.iter().enumerate() {
                assert_eq!(
                    primitive_index(*operation),
                    index,
                    "{operation:?} indexes itself"
                );
            }
            let unique: BTreeSet<&str> = PRIMITIVES.iter().map(|p| primitive_key(*p)).collect();
            assert_eq!(unique.len(), PRIMITIVES.len(), "primitive keys are unique");
            let mut produced: BTreeSet<String> = STRUCTURAL_KEYS
                .iter()
                .map(|key| (*key).to_owned())
                .collect();
            assert_eq!(
                produced.len(),
                STRUCTURAL_KEYS.len(),
                "structural keys are unique"
            );
            produced.extend(unique.iter().map(|key| (*key).to_owned()));
            let rows: BTreeSet<String> = registry.constructs.keys().cloned().collect();
            assert_eq!(
                rows, produced,
                "language/production-1.2.toml has exactly one row per classifiable construct"
            );
            assert_eq!(
                registry.order.len(),
                rows.len(),
                "no construct row is repeated"
            );
            // The schema's closed construct kinds each have a row, so a kind
            // added to the language without a disposition cannot pass.
            for (definition, property, prefix) in [
                ("type", "kind", "type."),
                ("term", "kind", "term."),
                ("declaration", "kind", "declaration."),
                ("term", "operation", "primitive."),
            ] {
                let kinds = schema_kinds(definition, property);
                assert!(
                    !kinds.is_empty(),
                    "the schema enumerates {definition} {property}s"
                );
                for kind in kinds {
                    let key = format!("{prefix}{kind}");
                    assert!(
                        rows.contains(&key),
                        "schema {definition} `{kind}` has no row `{key}`"
                    );
                }
            }
            // Every IR variant maps to its own key: the key is the variant's
            // serialized tag, so no two constructs can share a row.
            let mut sampled_types = BTreeSet::new();
            for sample in TYPE_SAMPLES {
                let ty: lexlean::SnapshotType = serde_json::from_str(sample).expect("type sample");
                let kind = serde_json::from_str::<serde_json::Value>(sample).expect("json")["kind"]
                    .as_str()
                    .expect("kind")
                    .to_owned();
                assert_eq!(type_key(&ty), format!("type.{kind}"), "{sample}");
                sampled_types.insert(kind);
            }
            assert_eq!(
                sampled_types,
                schema_kinds("type", "kind"),
                "every type kind is sampled"
            );
            let mut sampled_terms = BTreeSet::new();
            for sample in TERM_SAMPLES {
                let term: lexlean::SnapshotTerm =
                    serde_json::from_str(sample).expect("term sample");
                let kind = serde_json::from_str::<serde_json::Value>(sample).expect("json")["kind"]
                    .as_str()
                    .expect("kind")
                    .to_owned();
                assert_eq!(term_key(&term), format!("term.{kind}"), "{sample}");
                sampled_terms.insert(kind);
            }
            assert_eq!(
                sampled_terms,
                schema_kinds("term", "kind"),
                "every term kind is sampled"
            );
            for operation in PRIMITIVES {
                let tag = serde_json::to_value(operation).expect("operation serializes");
                assert_eq!(
                    primitive_key(operation),
                    format!("primitive.{}", tag.as_str().expect("tag")),
                );
            }
            // The closed sets and the dispositions that make them load-bearing.
            let targets: Vec<(&str, bool)> = registry
                .targets
                .values()
                .map(|target| (target.id.as_str(), target.allocation))
                .collect();
            assert_eq!(targets, [("rust-core", false), ("rust-std", true)]);
            assert!(registry
                .targets
                .values()
                .all(|target| target.natural_bits == 64 && target.integer_bits == 64));
            let effects: Vec<&str> = registry.effects.keys().map(String::as_str).collect();
            assert_eq!(effects, ["allocation", "overflow", "recursion"]);
            // Every column of every row, in registry order.
            let observed: Vec<(String, &str, bool, Vec<String>, bool)> = registry
                .order
                .iter()
                .map(|key| {
                    let row = &registry.constructs[key];
                    (
                        key.clone(),
                        row.disposition.as_str(),
                        row.allocation,
                        row.overflow.clone(),
                        row.recursion,
                    )
                })
                .collect();
            let expected: Vec<(String, &str, bool, Vec<String>, bool)> = REGISTRY_ROWS
                .iter()
                .map(|(key, disposition, allocation, overflow, recursion)| {
                    (
                        (*key).to_owned(),
                        *disposition,
                        *allocation,
                        overflow.iter().map(|value| (*value).to_owned()).collect(),
                        *recursion,
                    )
                })
                .collect();
            assert_eq!(
                observed, expected,
                "the registry rows are exactly the reviewed rows"
            );
            // The schemas carry exactly the registered targets and effects.
            for (schema, definition) in [
                ("semantic-module-v2", "production"),
                ("semantic-snapshot-v2", "semantic_production"),
            ] {
                let production = &support::schema(schema)["$defs"][definition]["properties"];
                assert_eq!(
                    strings(&production["targets"]["items"]["enum"]),
                    ["rust-core", "rust-std"]
                );
                assert_eq!(
                    strings(&production["effects"]["items"]["enum"]),
                    ["allocation", "overflow", "recursion"]
                );
            }
        }
        "PD-02" => {
            let project = P::copy_example("production");
            project.check_ok();
            let checked = support::checked_project(&project);
            let kernel = &checked.modules["Kernel"];
            let semantic = kernel.document.semantic.as_ref().expect("semantic Kernel");
            let kinds: Vec<(&str, &str)> = semantic
                .declarations
                .iter()
                .map(|declaration| (declaration.kind(), declaration.name()))
                .collect();
            assert!(kinds.contains(&("theorem", "area_square")), "{kinds:?}");
            assert!(kinds.contains(&("definition", "positive")), "{kinds:?}");
            assert!(
                kernel.production.is_none(),
                "a module with formal content and no root is never analysed"
            );
            let report = main_report(&project);
            let roots: Vec<String> = report["roots"]
                .as_array()
                .expect("roots")
                .iter()
                .map(|root| root["root"].as_str().expect("root").to_owned())
                .collect();
            assert_eq!(
                roots,
                [
                    "Production.Main.checkedSum",
                    "Production.Main.shapeArea",
                    "Production.Main.quadruple",
                    "Production.Main.halvings",
                    "Production.Main.sumAll",
                    "Production.Main.headOr",
                ]
            );
            // Theorems over the roots live beside them in the same module.
            let main = checked.modules["Main"]
                .document
                .semantic
                .as_ref()
                .expect("semantic Main");
            assert!(main
                .declarations
                .iter()
                .any(|declaration| declaration.name() == "quadruple_three"));
            // A language-1.2 project with formal content and no root builds
            // without any production artifact.
            let formal = P::copy_example("recursion");
            let build = support::rendered(&formal);
            assert!(
                build
                    .files
                    .iter()
                    .all(|(path, _)| !path.starts_with("production/")),
                "no root, no production analysis"
            );
        }
        "PD-03" => {
            let project = P::copy_example("production");
            let report = main_report(&project);
            assert_eq!(
                closure(root(&report, "checkedSum")),
                ["Production.Main.checkedSum"]
            );
            assert_eq!(
                closure(root(&report, "shapeArea")),
                ["Production.Main.shapeArea", "Production.Kernel.area"]
            );
            assert_eq!(
                closure(root(&report, "quadruple")),
                ["Production.Main.quadruple", "Production.Kernel.applyTwice"]
            );
            let halvings = root(&report, "halvings");
            assert_eq!(
                closure(halvings),
                ["Production.Main.halvings", "Production.Kernel.countdown"]
            );
            assert_eq!(
                strings(&halvings["erased"]),
                ["Production.Kernel.countdown_decreases"],
                "termination evidence is erased, never realized"
            );
            let headed = root(&report, "headOr");
            assert_eq!(
                closure(headed),
                ["Production.Main.headOr", "Production.Kernel.firstOr (Nat)"]
            );
            let generic = &headed["runtime_closure"][1];
            assert_eq!(strings(&generic["type_arguments"]), ["Nat"]);
            assert_eq!(
                strings(&generic["path"]),
                ["Production.Main.headOr", "Production.Kernel.firstOr (Nat)"]
            );
            // No theorem is ever part of a runtime closure.
            for root in report["roots"].as_array().expect("roots") {
                for member in closure(root) {
                    assert!(
                        !member.ends_with("_decreases") && !member.ends_with("area_square"),
                        "{member} is proof-only"
                    );
                }
            }
            // Dropping the dependency from the source drops it from the
            // closure: the closure is computed, not declared.
            let mutated = P::copy_example("production");
            mutated.edit(
                MAIN,
                r#"{"arguments":[{"kind":"var","name":"number"},{"kind":"nat","value":"0"}],"function":{"module":"Kernel","name":"countdown"},"kind":"call"}"#,
                r#"{"kind":"var","name":"number"}"#,
            );
            mutated.edit(
                MAIN,
                r#""effects":["overflow","recursion"]"#,
                r#""effects":[]"#,
            );
            let mutated_report = main_report(&mutated);
            let halvings = root(&mutated_report, "halvings");
            assert_eq!(closure(halvings), ["Production.Main.halvings"]);
            assert!(strings(&halvings["erased"]).is_empty());
        }
        "PD-04" => {
            let project = P::copy_example("production");
            let report = main_report(&project);
            let sum = root(&report, "sumAll");
            assert_eq!(
                effects(sum, "rust-std"),
                ["allocation", "overflow", "recursion"]
            );
            let checked = root(&report, "checkedSum");
            assert!(effects(checked, "rust-core").is_empty());
            assert!(effects(checked, "rust-std").is_empty());
            // The same root, declared for a target without allocation, is
            // rejected before any backend runs.
            let mutated = P::copy_example("production");
            mutated.edit(
                MAIN,
                r#""effects":["allocation","overflow","recursion"],"targets":["rust-std"]"#,
                r#""effects":["allocation","overflow","recursion"],"targets":["rust-core","rust-std"]"#,
            );
            let error = mutated.check_fails_with("LLT4005");
            let text = error.to_string();
            assert!(
                text.contains("`Production.Main.sumAll` is not eligible for target `rust-core`")
                    && text.contains("requires heap allocation"),
                "{text}"
            );
            mutated.assert_no_backend_output_with("LLT4005");
        }
        "PD-05" => {
            for (fixture, fragment) in [
                (
                    "production-hidden-dependency",
                    "reached by LanguageTwelve.Main.entry -> LanguageTwelve.Helper.relay -> LanguageTwelve.Deep.count",
                ),
                ("production-unbounded-type", "construct `type.list` requires heap allocation, which target `rust-core` does not provide"),
                ("production-runtime-incompatible-type", "root result holds a proposition"),
                ("production-effect-mismatch", "effect mismatch: construct `term.add` realizes effect `overflow`"),
                ("production-higher-order-escape", "root parameter `step` holds a function, so a closure would escape the root"),
                ("production-formal-only-dependency", "construct `type.prop` is formal-only"),
                ("production-literal-width", "literal 18446744073709551616 does not fit the 64-bit nat representation"),
                ("production-root-not-executable", "`LanguageTwelve.Main.identity` is not declared executable"),
                ("production-polymorphic-root", "production root declares the type parameters (Item); a root is realized at one type (in `LanguageTwelve.Main.identity`, reached by LanguageTwelve.Main.identity; 4 violation(s) in total)"),
                ("production-phantom-type-parameter", "production root declares the type parameters (Item)"),
                ("production-universe-boundary", "root parameter `carrier` holds a type universe"),
                ("production-named-type-boundary", "root parameter `witness` holds a proposition"),
            ] {
                let project = P::negative(fixture);
                let error = project.check_fails_with("LLT4005");
                let text = error.to_string();
                assert!(text.contains(fragment), "{fixture}: expected {fragment:?}, got {text}");
                assert!(
                    project.engine().build(lexlean::BuildRequest {
                        selection: lexlean::Selection::Entrypoints,
                    }).is_err(),
                    "{fixture} never builds"
                );
                project.assert_no_backend_output_with("LLT4005");
            }
            let malformed = P::negative("production-unknown-target");
            let error = malformed.check_fails_with("LLT4001");
            assert!(
                error
                    .to_string()
                    .contains("unregistered target `rust-wasm`"),
                "{error}"
            );
            // A mutation of the committed example: an admitted effect removed.
            let mutated = P::copy_example("production");
            mutated.edit(
                MAIN,
                r#""name":"shapeArea","parameters":[{"name":"width","type":{"kind":"nat"}},{"name":"height","type":{"kind":"nat"}}],"production":{"effects":["overflow"]"#,
                r#""name":"shapeArea","parameters":[{"name":"width","type":{"kind":"nat"}},{"name":"height","type":{"kind":"nat"}}],"production":{"effects":[]"#,
            );
            let error = mutated.check_fails_with("LLT4005");
            assert!(
                error.to_string().contains("`Production.Main.shapeArea`"),
                "{error}"
            );
            assert!(error.to_string().contains("primitive.multiply"), "{error}");
        }
        "PD-06" => {
            let first = P::copy_example("production");
            let second = P::copy_example("production");
            let one = support::rendered(&first);
            let two = support::rendered(&second);
            let report_of = |build: &lexlean::api::RenderedBuild| {
                build
                    .files
                    .iter()
                    .find(|(path, _)| path == "production/Production/Main.eligibility.json")
                    .map(|(_, bytes)| bytes.clone())
                    .expect("report published")
            };
            let bytes = report_of(&one);
            assert_eq!(
                bytes,
                report_of(&two),
                "the report is deterministic across checkouts"
            );
            let value: serde_json::Value = serde_json::from_slice(&bytes).expect("JSON");
            support::assert_schema("production-eligibility", "the production report", &value);
            let canonical = lexlean::artifact::canonical_json::Json::parse(&bytes)
                .expect("canonical JSON")
                .to_file_bytes();
            assert_eq!(bytes, canonical, "the report is canonical JSON");
            assert!(
                !String::from_utf8_lossy(&bytes).contains(first.root.as_str()),
                "the report holds no absolute path"
            );
            let row = one
                .manifest
                .outputs
                .iter()
                .find(|row| row.path == "production/Production/Main.eligibility.json")
                .expect("the manifest records the report");
            assert_eq!(row.kind, "production-eligibility");
            assert_eq!(
                row.sha256,
                lexlean::artifact::content_id::Sha256Digest::of(&bytes)
            );
            // Every reported effect names its sources, and every construct is
            // a runtime row of the registry.
            for root in value["roots"].as_array().expect("roots") {
                for construct in root["constructs"].as_array().expect("constructs") {
                    assert_eq!(construct["disposition"], "runtime");
                }
                for target in root["targets"].as_array().expect("targets") {
                    for effect in target["effects"].as_array().expect("effects") {
                        let effect_name = effect["effect"].as_str().expect("effect");
                        assert!(
                            strings(&root["declared_effects"])
                                .iter()
                                .any(|e| e == effect_name),
                            "{effect_name} is admitted"
                        );
                        assert!(!effect["sources"].as_array().expect("sources").is_empty());
                    }
                }
            }
            let snapshot = first.engine().snapshot(lexlean::CheckRequest {
                selection: lexlean::Selection::Entrypoints,
            });
            let snapshot = snapshot.expect("snapshot");
            let snapshot_json = serde_json::to_value(&snapshot).expect("snapshot serializes");
            support::assert_schema(
                "semantic-snapshot-v2",
                "the production example snapshot",
                &snapshot_json,
            );
        }
        "PD-07" => {
            let root = support::repo_root();
            let read = |relative: &str| {
                std::fs::read_to_string(root.join(relative).as_std_path()).expect("source")
            };
            let eligibility = read(repo_model::exhaustive::ELIGIBILITY_SOURCE);
            let semantic = read(repo_model::exhaustive::SEMANTIC_SOURCE);
            repo_model::exhaustive::audit_eligibility(&eligibility, &semantic)
                .expect("the committed analysis is explicit");
            for variant in
                repo_model::exhaustive::enum_variants(&semantic, "SemanticTerm").expect("terms")
            {
                assert!(
                    eligibility.contains(&format!("SemanticTerm::{variant}")),
                    "{variant}"
                );
            }
            // Planted defaults: a wildcard or binding arm replacing an
            // explicit one, a rest pattern, an `if let`, a tuple default, an
            // equality test on the IR, an unnamed variant, and a variant
            // listed but never matched.
            let arm = r#"SemanticPrimitive::GraphTopological => "primitive.graph_topological","#;
            assert!(eligibility.contains(arm));
            for (planted, expected) in [
                (
                    eligibility.replacen(arm, r#"_ => "primitive.graph_topological","#, 1),
                    "a wildcard arm",
                ),
                (
                    eligibility.replacen(
                        "SemanticTerm::Unit => \"term.unit\",",
                        "SemanticTerm::Unit => \"term.unit\",\n        SemanticTerm::Bool { .. } => \"term.bool\",",
                        1,
                    ),
                    "a rest pattern",
                ),
                (
                    format!("{eligibility}\nfn planted(t: &SemanticType) -> bool {{ if let SemanticType::Nat = t {{ true }} else {{ false }} }}\n"),
                    "`if let`",
                ),
                (
                    eligibility.replace("SemanticInteger::UInt64", "SemanticInteger::UInt32"),
                    "`SemanticInteger::UInt64` has no explicit production disposition",
                ),
                (
                    eligibility.replacen(arm, r#"_other => "primitive.graph_topological","#, 1),
                    "a binding or wildcard catch-all arm",
                ),
                (
                    format!("{eligibility}\nfn planted(t: &SemanticType, u: &SemanticType) -> u8 {{\n    match (t, u) {{\n        (SemanticType::Nat, SemanticType::Nat) => 0,\n        (_, _) => 1,\n    }}\n}}\n"),
                    "a tuple pattern with a binding or wildcard element",
                ),
                (
                    format!("{eligibility}\nfn planted(t: &SemanticType) -> bool {{\n    *t == SemanticType::Nat\n}}\n"),
                    "an equality test on an IR enum",
                ),
                (
                    // Named only in the operation list, classified nowhere.
                    eligibility
                        .replacen(arm, "", 1)
                        .replacen("        SemanticPrimitive::GraphTopological => 52,\n", "", 1),
                    "`SemanticPrimitive::GraphTopological` has no explicit production disposition",
                ),
            ] {
                let error = repo_model::exhaustive::audit_eligibility(&planted, &semantic)
                    .expect_err("a planted default is caught");
                assert!(error.contains(expected), "expected {expected:?}, got {error}");
            }
        }
        other => panic!("no production case is wired for {other}"),
    }
}
