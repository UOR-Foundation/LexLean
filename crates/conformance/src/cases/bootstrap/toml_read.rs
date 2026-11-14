//! `BS-53` (SPEC.md §17.23, §13.1, §13.2, §12.3): a reader for the TOML
//! subset the compiler's own committed files use (basic strings with the
//! escapes `\\`, `\"`, `\n`, `\t`, `\r`, integers, booleans, string arrays
//! including multi-line ones, `#` line comments, `[table]` and
//! `[[array-of-table]]` headers including a dotted one) as language-1.2
//! source, `LexLeanBoot.TomlRead`. No inline table, no literal or
//! multi-line string, and no dotted key is read, since none of the files in
//! scope (`lexicon.toml` manifests, entry files, `language/bootstrap.toml`,
//! `language/renderer-tokens.toml`) ever write one.
//!
//! `tomlParse` has no Rust original to hold to; it is held instead to the
//! `toml` crate, read generically into a `toml::Table`, on every committed
//! file in scope and on a seeded flat document. Each planted defect names
//! the one vector that catches it.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use lexlean::calculus::Value;
use serde_json::Value as Json2;

use super::rust_validate::{eligibility_of, rustc_differential};
use super::{list, option, pair, text, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.TomlRead";
const SOURCE: &str = "bootstrap/src/TomlRead.lex.tex";
/// The same file, relative to the bootstrap project's own root.
const PROJECT_SOURCE: &str = "src/TomlRead.lex.tex";
const ROOTS: [(&str, &[&str]); 1] = [("tomlParse", &["allocation", "overflow"])];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// Seeded flat documents under the evaluator, and under rustc.
const EVALUATOR_SEEDED: usize = 200;
const RUSTC_SEEDED: usize = 1_500;

fn nat(value: u64) -> Value {
    Value::Nat {
        value: value.to_string(),
    }
}

// ------------------------------------------------------------- the sorts

/// The constructors of an inductive of the module, in declaration order,
/// read from its text, so that the encoder names a constructor and never
/// numbers it, and one moved in the module fails here.
fn constructors(sort: &str) -> &'static Vec<String> {
    static TABLES: OnceLock<BTreeMap<String, Vec<String>>> = OnceLock::new();
    let tables = TABLES.get_or_init(|| {
        let source = std::fs::read_to_string(repo_root().join(SOURCE).as_std_path())
            .expect("the module's source");
        let mut out = BTreeMap::new();
        for block in source.split("inductive{").skip(1) {
            let block = block.split("\n}\n").next().expect("a block");
            let name = block
                .split("name: ")
                .nth(1)
                .and_then(|rest| rest.split(',').next())
                .expect("the inductive's name")
                .trim()
                .to_owned();
            let names: Vec<String> = block
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
            out.insert(name, names);
        }
        out
    });
    tables
        .get(sort)
        .unwrap_or_else(|| panic!("the module has no inductive `{sort}`"))
}

fn adt(sort: &str, constructor: &str, fields: Vec<Value>) -> Value {
    let found = constructors(sort)
        .iter()
        .position(|name| name == constructor)
        .unwrap_or_else(|| panic!("`{sort}` has no constructor `{constructor}`"));
    Value::Adt {
        constructor: u64::try_from(found).expect("an index"),
        fields,
    }
}

// ------------------------------------------------------------- the encoders

fn vstr(value: &str) -> Value {
    adt("Value", "VStr", vec![text(value)])
}
fn vbool(value: bool) -> Value {
    adt("Value", "VBool", vec![Value::Bool { value }])
}
fn vnat(value: u64) -> Value {
    adt("Value", "VNat", vec![nat(value)])
}
fn varr(items: &[&str]) -> Value {
    adt(
        "Value",
        "VArr",
        vec![list(items.iter().map(|item| text(item)).collect())],
    )
}

/// One row: an ordered `(key, value)` list, as `Pst`'s `curRow` builds it.
fn row(fields: Vec<(&str, Value)>) -> Value {
    list(
        fields
            .into_iter()
            .map(|(key, value)| pair(text(key), value))
            .collect(),
    )
}

/// One `Section`, a structure, so its one constructor is `0`.
fn section(path: &[&str], is_array: bool, rows: Vec<Value>) -> Value {
    Value::Adt {
        constructor: 0,
        fields: vec![
            list(path.iter().map(|segment| text(segment)).collect()),
            Value::Bool { value: is_array },
            list(rows),
        ],
    }
}

fn doc(sections: Vec<Value>) -> Value {
    list(sections)
}

fn parse_run(source: &str, expected: Option<Value>) -> Run {
    (
        vec![text(source)],
        serde_json::to_value(option(expected)).expect("a value is JSON"),
    )
}

// ------------------------------------------------------------------ vectors

struct Vector {
    label: &'static str,
    run: Run,
}

#[allow(clippy::too_many_lines)]
fn named() -> Vec<Vector> {
    let top_empty = || doc(vec![section(&[], false, vec![row(vec![])])]);
    vec![
        Vector {
            label: "an empty document",
            run: parse_run("", Some(top_empty())),
        },
        Vector {
            label: "a blank line only",
            run: parse_run("\n", Some(top_empty())),
        },
        Vector {
            label: "a comment only",
            run: parse_run("# hello\n", Some(top_empty())),
        },
        Vector {
            label: "a string value",
            run: parse_run(
                "a = \"b\"\n",
                Some(doc(vec![section(
                    &[],
                    false,
                    vec![row(vec![("a", vstr("b"))])],
                )])),
            ),
        },
        Vector {
            label: "a bool value true",
            run: parse_run(
                "a = true\n",
                Some(doc(vec![section(
                    &[],
                    false,
                    vec![row(vec![("a", vbool(true))])],
                )])),
            ),
        },
        Vector {
            label: "a bool value false",
            run: parse_run(
                "a = false\n",
                Some(doc(vec![section(
                    &[],
                    false,
                    vec![row(vec![("a", vbool(false))])],
                )])),
            ),
        },
        Vector {
            label: "an integer value",
            run: parse_run(
                "a = 42\n",
                Some(doc(vec![section(
                    &[],
                    false,
                    vec![row(vec![("a", vnat(42))])],
                )])),
            ),
        },
        Vector {
            label: "a leading zero integer is not canonical",
            run: parse_run("a = 01\n", None),
        },
        Vector {
            label: "a single-line string array",
            run: parse_run(
                "a = [\"x\", \"y\"]\n",
                Some(doc(vec![section(
                    &[],
                    false,
                    vec![row(vec![("a", varr(&["x", "y"]))])],
                )])),
            ),
        },
        Vector {
            label: "an empty array",
            run: parse_run(
                "a = []\n",
                Some(doc(vec![section(
                    &[],
                    false,
                    vec![row(vec![("a", varr(&[]))])],
                )])),
            ),
        },
        Vector {
            label: "a multi-line array with trailing commas",
            run: parse_run(
                "a = [\n  \"x\",\n  \"y\",\n]\n",
                Some(doc(vec![section(
                    &[],
                    false,
                    vec![row(vec![("a", varr(&["x", "y"]))])],
                )])),
            ),
        },
        Vector {
            label: "a multi-line array without a final trailing comma",
            run: parse_run(
                "a = [\n  \"x\",\n  \"y\"\n]\n",
                Some(doc(vec![section(
                    &[],
                    false,
                    vec![row(vec![("a", varr(&["x", "y"]))])],
                )])),
            ),
        },
        Vector {
            label: "an escaped backslash",
            run: parse_run(
                "a = \"\\\\end\"\n",
                Some(doc(vec![section(
                    &[],
                    false,
                    vec![row(vec![("a", vstr("\\end"))])],
                )])),
            ),
        },
        Vector {
            label: "an escaped quote",
            run: parse_run(
                "a = \"a\\\"b\"\n",
                Some(doc(vec![section(
                    &[],
                    false,
                    vec![row(vec![("a", vstr("a\"b"))])],
                )])),
            ),
        },
        Vector {
            label: "an escaped newline tab and carriage return",
            run: parse_run(
                "a = \"x\\n\\t\\ry\"\n",
                Some(doc(vec![section(
                    &[],
                    false,
                    vec![row(vec![("a", vstr("x\n\t\ry"))])],
                )])),
            ),
        },
        Vector {
            label: "two keys in one row keep file order",
            run: parse_run(
                "a = 1\nb = 2\n",
                Some(doc(vec![section(
                    &[],
                    false,
                    vec![row(vec![("a", vnat(1)), ("b", vnat(2))])],
                )])),
            ),
        },
        Vector {
            label: "a table header",
            run: parse_run(
                "a = 1\n[t]\nb = 2\n",
                Some(doc(vec![
                    section(&[], false, vec![row(vec![("a", vnat(1))])]),
                    section(&["t"], false, vec![row(vec![("b", vnat(2))])]),
                ])),
            ),
        },
        Vector {
            label: "an array-of-table header repeated",
            run: parse_run(
                "[[t]]\nb = 1\n[[t]]\nb = 2\n",
                Some(doc(vec![
                    section(&[], false, vec![row(vec![])]),
                    section(
                        &["t"],
                        true,
                        vec![row(vec![("b", vnat(1))]), row(vec![("b", vnat(2))])],
                    ),
                ])),
            ),
        },
        Vector {
            label: "a dotted array-of-table header",
            run: parse_run(
                "[t]\nc = 1\n[[t.u]]\nd = 2\n",
                Some(doc(vec![
                    section(&[], false, vec![row(vec![])]),
                    section(&["t"], false, vec![row(vec![("c", vnat(1))])]),
                    section(&["t", "u"], true, vec![row(vec![("d", vnat(2))])]),
                ])),
            ),
        },
        Vector {
            label: "a duplicate key in one row is refused",
            run: parse_run("a = 1\na = 2\n", None),
        },
        Vector {
            label: "redefining a table is refused",
            run: parse_run("[t]\na = 1\n[t]\nb = 2\n", None),
        },
        Vector {
            label: "an unterminated string is refused",
            run: parse_run("a = \"x\n", None),
        },
        Vector {
            label: "an unterminated multi-line array is refused",
            run: parse_run("a = [\n  \"x\",\n", None),
        },
        Vector {
            label: "a malformed header is refused",
            run: parse_run("[]\n", None),
        },
        Vector {
            label: "an empty array header path is refused",
            run: parse_run("[[]]\n", None),
        },
        Vector {
            label: "a bare value is refused",
            run: parse_run("a = x\n", None),
        },
        Vector {
            label: "a line with no equals and no header is refused",
            run: parse_run("nonsense\n", None),
        },
        Vector {
            label: "a stray byte after an array element is refused",
            run: parse_run("a = [\"x\" y]\n", None),
        },
        Vector {
            label: "an unsupported escape is refused",
            run: parse_run("a = \"\\b\"\n", None),
        },
        Vector {
            label: "a comment after a value is refused",
            run: parse_run("a = 1 # comment\n", None),
        },
        Vector {
            label: "a dotted key is refused",
            run: parse_run("a.b = 1\n", None),
        },
        Vector {
            label: "a literal string is refused",
            run: parse_run("a = 'x'\n", None),
        },
        Vector {
            label: "a multi-line basic string is refused",
            run: parse_run("a = \"\"\"x\"\"\"\n", None),
        },
        Vector {
            label: "an inline table is refused",
            run: parse_run("a = {b = 1}\n", None),
        },
        Vector {
            label: "a float is refused",
            run: parse_run("a = 1.5\n", None),
        },
        Vector {
            label: "a date is refused",
            run: parse_run("a = 2024-01-01\n", None),
        },
        Vector {
            label: "CRLF line endings parse like LF",
            run: parse_run(
                "a = \"x\"\r\n",
                Some(doc(vec![section(
                    &[],
                    false,
                    vec![row(vec![("a", vstr("x"))])],
                )])),
            ),
        },
    ]
}

// --------------------------------------------------------------- committed

/// Every TOML file the compiler's own code reads in scope (`lexicon.toml`
/// manifests, entry files, `language/bootstrap.toml`,
/// `language/renderer-tokens.toml`): its path and text.
fn committed() -> Vec<(String, String)> {
    let root = repo_root();
    let walker = walkdir::WalkDir::new(root.as_std_path())
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|entry| {
            let name = entry.file_name().to_string_lossy();
            !(entry.file_type().is_dir()
                && (matches!(name.as_ref(), "target" | ".git" | ".lexlean")
                    || support::holds_cache_tag(entry)))
        });
    let mut out = Vec::new();
    for entry in walker.flatten() {
        let path = entry.path();
        let shown = path.to_string_lossy().replace('\\', "/");
        if !entry.file_type().is_file() {
            continue;
        }
        let in_scope = shown.ends_with("/lexicon.toml") && !shown.contains("/tests/")
            || (shown.contains("/entries/")
                && shown.ends_with(".toml")
                && shown.starts_with(&format!("{root}/language/").replace('\\', "/")))
            || shown == format!("{root}/language/bootstrap.toml").replace('\\', "/")
            || shown == format!("{root}/language/renderer-tokens.toml").replace('\\', "/");
        if !in_scope {
            continue;
        }
        let Ok(content) = std::fs::read_to_string(path) else {
            continue;
        };
        out.push((shown, content));
    }
    out
}

/// A `toml::Table`'s own scalar and string-array fields, skipping any nested
/// table or array of tables (which `tomlParse`'s top row also skips, each
/// becoming its own `Section`).
fn simple_fields(table: &toml::Table) -> Vec<(String, toml::Value)> {
    table
        .iter()
        .filter(|(_, value)| !is_table_or_array_of_tables(value))
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect()
}

fn is_table_or_array_of_tables(value: &toml::Value) -> bool {
    is_table_array(value) || matches!(value, toml::Value::Table(_))
}

fn is_table_array(value: &toml::Value) -> bool {
    matches!(value, toml::Value::Array(items) if !items.is_empty() && items.iter().all(|item| matches!(item, toml::Value::Table(_))))
}

/// `table`'s own scalar and string-array fields only (what one row holds).
fn simple_table(table: &toml::Table) -> toml::Table {
    table
        .iter()
        .filter(|(_, value)| !is_table_or_array_of_tables(value))
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect()
}

/// `table`'s own sections, keyed by path and whether it is an array of
/// tables, each recursed into (`[eliminator]` with `[[eliminator.constructor]]`
/// nests one table deep). `toml::Table` does not preserve file order, so
/// callers order both the sections and each row's fields from the text
/// themselves (`section_order`, `file_order`).
fn sections_of(
    table: &toml::Table,
    path: &[String],
) -> BTreeMap<(Vec<String>, bool), Vec<toml::Table>> {
    let mut out = BTreeMap::new();
    out.insert((path.to_vec(), false), vec![simple_table(table)]);
    for (key, value) in table {
        let mut child = path.to_vec();
        child.push(key.clone());
        if let toml::Value::Table(nested) = value {
            out.extend(sections_of(nested, &child));
        } else if is_table_array(value) {
            let toml::Value::Array(items) = value else {
                unreachable!()
            };
            let rows: Vec<toml::Table> = items
                .iter()
                .map(|item| {
                    let toml::Value::Table(row) = item else {
                        unreachable!()
                    };
                    simple_table(row)
                })
                .collect();
            out.insert((child, true), rows);
        }
    }
    out
}

/// The `[path]`/`[[path]]` sections of `text`, in the order their header (or,
/// for the top, the file itself) first appears.
fn section_order(text: &str) -> Vec<(Vec<String>, bool)> {
    let mut out = vec![(Vec::new(), false)];
    let mut seen: std::collections::HashSet<(Vec<String>, bool)> = out.iter().cloned().collect();
    let mut in_array_value = false;
    for raw in text.lines() {
        let trimmed = raw.trim_end_matches('\r').trim();
        if in_array_value {
            if trimmed.ends_with(']') {
                in_array_value = false;
            }
            continue;
        }
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some(key) = header_path(trimmed) {
            if seen.insert(key.clone()) {
                out.push(key);
            }
            continue;
        }
        if let Some(eq) = trimmed.find('=') {
            if trimmed[eq + 1..].trim() == "[" {
                in_array_value = true;
            }
        }
    }
    out
}

/// Every `key = value` line of `text`, as the section it belongs to (its
/// path, whether that section is an array of tables, and which occurrence
/// of it this line is in, counting from `0`), its key, in file order.
fn file_order(text: &str) -> Vec<(Vec<String>, bool, usize, String)> {
    let mut out = Vec::new();
    let mut path: Vec<String> = Vec::new();
    let mut is_array = false;
    let mut row_index = 0usize;
    let mut counts: std::collections::HashMap<Vec<String>, usize> =
        std::collections::HashMap::new();
    let mut in_array_value = false;
    for raw in text.lines() {
        let trimmed = raw.trim_end_matches('\r').trim();
        if in_array_value {
            if trimmed.ends_with(']') {
                in_array_value = false;
            }
            continue;
        }
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some((new_path, is_arr)) = header_path(trimmed) {
            row_index = if is_arr {
                let counter = counts.entry(new_path.clone()).or_insert(0);
                let at = *counter;
                *counter += 1;
                at
            } else {
                0
            };
            path = new_path;
            is_array = is_arr;
            continue;
        }
        if let Some(eq) = trimmed.find('=') {
            let key = trimmed[..eq].trim().to_owned();
            let value = trimmed[eq + 1..].trim();
            out.push((path.clone(), is_array, row_index, key));
            if value == "[" {
                in_array_value = true;
            }
        }
    }
    out
}

/// `trimmed`'s header path and whether it is an array of tables, when
/// `trimmed` is a `[path]` or `[[path]]` line.
fn header_path(trimmed: &str) -> Option<(Vec<String>, bool)> {
    if !trimmed.starts_with('[') {
        return None;
    }
    let (is_array, inner) = if let Some(rest) = trimmed.strip_prefix("[[") {
        (true, rest.strip_suffix("]]").unwrap_or(rest))
    } else {
        let rest = trimmed.strip_prefix('[').unwrap_or(trimmed);
        (false, rest.strip_suffix(']').unwrap_or(rest))
    };
    Some((inner.split('.').map(str::to_owned).collect(), is_array))
}

fn json_bytes_to_string(value: &Json2) -> String {
    let hex = value["hex"]
        .as_str()
        .unwrap_or_else(|| panic!("a bytes value: {value}"));
    let raw: Vec<u8> = (0..hex.len())
        .step_by(2)
        .map(|at| u8::from_str_radix(&hex[at..at + 2], 16).expect("a hex digit"))
        .collect();
    String::from_utf8(raw).expect("UTF-8 bytes")
}

/// Does the evaluator's encoded `Value` (a TOML scalar or string array)
/// equal this `toml::Value`?
fn json_value_matches(mine: &Json2, theirs: &toml::Value) -> bool {
    let ctor = mine["constructor"].as_u64().expect("a constructor index");
    let names = constructors("Value");
    let at = |name: &str| names.iter().position(|n| n == name).expect("a constructor") as u64;
    match theirs {
        toml::Value::String(s) => {
            ctor == at("VStr") && json_bytes_to_string(&mine["fields"][0]) == *s
        }
        toml::Value::Boolean(b) => {
            ctor == at("VBool") && mine["fields"][0]["value"].as_bool() == Some(*b)
        }
        toml::Value::Integer(i) => {
            ctor == at("VNat") && mine["fields"][0]["value"].as_str() == Some(&i.to_string())
        }
        toml::Value::Array(items) => {
            ctor == at("VArr") && {
                let got = mine["fields"][0]["items"].as_array().expect("array items");
                got.len() == items.len()
                    && got.iter().zip(items).all(|(g, item)| {
                        item.as_str()
                            .map(|s| json_bytes_to_string(g) == s)
                            .unwrap_or(false)
                    })
            }
        }
        _ => false,
    }
}

/// The evaluator's top (`path = []`, not an array) section's row, or `None`
/// if the document has no top row (never happens: `tomlParse` always closes
/// one, even an empty one).
fn json_top_row(doc_json: &Json2) -> Option<&Json2> {
    doc_json["items"].as_array()?.iter().find_map(|section| {
        let path = section["fields"][0]["items"].as_array()?;
        let is_array = section["fields"][1]["value"].as_bool()?;
        if path.is_empty() && !is_array {
            section["fields"][2]["items"].as_array()?.first()
        } else {
            None
        }
    })
}

fn json_row_get<'a>(row: &'a Json2, key: &str) -> Option<&'a Json2> {
    row["items"].as_array()?.iter().find_map(|entry| {
        if json_bytes_to_string(&entry["left"]) == key {
            Some(&entry["right"])
        } else {
            None
        }
    })
}

// -------------------------------------------------------------------- seeds

const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

fn seeded_string(seeded: &mut Seeded) -> String {
    let len = 1 + seeded.below(6);
    (0..len).map(|_| *seeded.pick(ALPHABET) as char).collect()
}

/// A flat, top-level-only document (no header), and the `toml::Table` it
/// equals, so the same conversion the committed files use also checks it.
fn seeded_doc(seeded: &mut Seeded) -> (String, toml::Table) {
    let count = 1 + seeded.below(5);
    let mut source = String::new();
    let mut table = toml::Table::new();
    for index in 0..count {
        let key = format!("k{index}");
        let (text, value) = match seeded.below(4) {
            0 => {
                let value = seeded_string(seeded);
                (format!("\"{value}\""), toml::Value::String(value))
            }
            1 => {
                let value = seeded.chance(50);
                (value.to_string(), toml::Value::Boolean(value))
            }
            2 => {
                let value = seeded.below(10_000) as i64;
                (value.to_string(), toml::Value::Integer(value))
            }
            _ => {
                let items: Vec<String> = (0..seeded.below(4))
                    .map(|_| seeded_string(seeded))
                    .collect();
                let text = format!(
                    "[{}]",
                    items
                        .iter()
                        .map(|item| format!("\"{item}\""))
                        .collect::<Vec<_>>()
                        .join(", ")
                );
                let value =
                    toml::Value::Array(items.into_iter().map(toml::Value::String).collect());
                (text, value)
            }
        };
        source.push_str(&key);
        source.push_str(" = ");
        source.push_str(&text);
        source.push('\n');
        table.insert(key, value);
    }
    (source, table)
}

fn encode_scalar(value: &toml::Value) -> Value {
    match value {
        toml::Value::String(s) => vstr(s),
        toml::Value::Boolean(b) => vbool(*b),
        toml::Value::Integer(i) => vnat(u64::try_from(*i).expect("a non-negative integer")),
        toml::Value::Array(items) => {
            let strings: Vec<&str> = items
                .iter()
                .map(|item| item.as_str().expect("a string array element"))
                .collect();
            varr(&strings)
        }
        other => panic!("an unsupported value: {other:?}"),
    }
}

/// `text`'s sections and rows as `tomlParse` would build them: `table` (the
/// `toml` crate's own read of the same bytes) gives the values, `text`
/// itself gives the file order `toml::Table` does not keep.
fn expected_for(text: &str, table: &toml::Table) -> Value {
    let by_path = sections_of(table, &[]);
    let order = file_order(text);
    doc(section_order(text)
        .into_iter()
        .map(|(path, is_array)| {
            let rows = by_path
                .get(&(path.clone(), is_array))
                .unwrap_or_else(|| panic!("`{path:?}` (array: {is_array}) has no toml rows"));
            let encoded_rows: Vec<Value> = rows
                .iter()
                .enumerate()
                .map(|(row_index, row_table)| {
                    let keys: Vec<&str> = order
                        .iter()
                        .filter(|(p, a, r, _)| *p == path && *a == is_array && *r == row_index)
                        .map(|(_, _, _, key)| key.as_str())
                        .collect();
                    row(keys
                        .into_iter()
                        .map(|key| {
                            let value = row_table.get(key).unwrap_or_else(|| {
                                panic!("`{key}` is in file order but not in the toml row")
                            });
                            (key, encode_scalar(value))
                        })
                        .collect())
                })
                .collect();
            let path_strs: Vec<&str> = path.iter().map(String::as_str).collect();
            section(&path_strs, is_array, encoded_rows)
        })
        .collect())
}

/// A document with exactly one line altered to lie just outside the subset
/// (an unsupported escape, a float, a date, a literal or multi-line string,
/// an inline table, or a dotted key), so `tomlParse` must refuse it even
/// though the `toml` crate accepts it.
fn seeded_boundary(seeded: &mut Seeded) -> String {
    let (mut source, _) = seeded_doc(seeded);
    let poison = match seeded.below(7) {
        0 => "p = \"\\b\"\n",
        1 => "p = 1.5\n",
        2 => "p = 2024-01-01\n",
        3 => "p = 'x'\n",
        4 => "p = \"\"\"x\"\"\"\n",
        5 => "p = {q = 1}\n",
        _ => "p.q = 1\n",
    };
    source.push_str(poison);
    source
}

fn seeded_runs(count: usize) -> Vec<Run> {
    let mut seeded = Seeded(0x0B55_0053);
    let mut out = Vec::new();
    for _ in 0..count {
        if seeded.chance(30) {
            let source = seeded_boundary(&mut seeded);
            out.push(parse_run(&source, None));
        } else {
            let (source, table) = seeded_doc(&mut seeded);
            let expected = expected_for(&source, &table);
            out.push(parse_run(&source, Some(expected)));
        }
    }
    out
}

// ------------------------------------------------------------------- plants

struct Plant {
    label: &'static str,
    site: &'static str,
    replacement: &'static str,
    vector: &'static str,
}

const PLANTS: &[Plant] = &[
    Plant {
        label: "a duplicate key is not rejected",
        site: "(if (rowHasKey (project st curRow) key) then",
        replacement: "(if false then",
        vector: "a duplicate key in one row is refused",
    },
    Plant {
        label: "redefining a table is not rejected",
        site: "(if isArr then (false, (mergeInto (second acc) path isArr row)) else (true, (second acc)))",
        replacement: "(if isArr then (false, (mergeInto (second acc) path isArr row)) else (false, (second acc)))",
        vector: "redefining a table is refused",
    },
    Plant {
        label: "a comment line is read as content",
        site: "(LexLeanStd.Text::startsWith t (lit \"#\"))",
        replacement: "false",
        vector: "a comment only",
    },
    Plant {
        label: "the newline escape passes its backslash through",
        site: "(if (prim equal b (110 : UInt8) : Bool) then 0x0a else",
        replacement: "(if (prim equal b (110 : UInt8) : Bool) then 0x6e else",
        vector: "an escaped newline tab and carriage return",
    },
    Plant {
        label: "an empty header path is accepted",
        site: "(if ((prim length inner : Nat) =? 0) then (ctor Option.none [((List Bytes) * Bool)]) else (ctor Option.some [((List Bytes) * Bool)] ((LexLeanStd.Text::splitOn inner (46 : UInt8)), isArr)))",
        replacement: "(ctor Option.some [((List Bytes) * Bool)] ((LexLeanStd.Text::splitOn inner (46 : UInt8)), isArr))",
        vector: "an empty array header path is refused",
    },
    Plant {
        label: "a stray byte after an array element is accepted",
        site: "| Option.some cb => (if (prim equal cb (44 : UInt8) : Bool) then (ctor Option.some [Es] (record Es {pos := (skipWs line (afterPos + 1)), out := (cons (first pair) (project st out)), status := 0})) else (ctor Option.some [Es] (record Es {pos := afterPos, out := (cons (first pair) (project st out)), status := 2})))",
        replacement: "| Option.some cb => (if (prim equal cb (44 : UInt8) : Bool) then (ctor Option.some [Es] (record Es {pos := (skipWs line (afterPos + 1)), out := (cons (first pair) (project st out)), status := 0})) else (ctor Option.some [Es] (record Es {pos := afterPos, out := (cons (first pair) (project st out)), status := 1})))",
        vector: "a stray byte after an array element is refused",
    },
    Plant {
        label: "an unsupported escape is accepted",
        site: "(if (if (prim equal b (34 : UInt8) : Bool) then true else (if (prim equal b (92 : UInt8) : Bool) then true else (if (prim equal b (110 : UInt8) : Bool) then true else (if (prim equal b (116 : UInt8) : Bool) then true else (prim equal b (114 : UInt8) : Bool))))) then",
        replacement: "(if true then",
        vector: "an unsupported escape is refused",
    },
    Plant {
        label: "a dotted key is accepted",
        site: "(if (if ((prim length key : Nat) =? 0) then true else (match (LexLeanStd.Text::find key (lit \".\")) | Option.some spot => true | Option.none => false)) then",
        replacement: "(if ((prim length key : Nat) =? 0) then",
        vector: "a dotted key is refused",
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
        let source = planted.read_raw(PROJECT_SOURCE);
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            PROJECT_SOURCE,
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[0].0);
        match super::evaluate(&low, std::slice::from_ref(&vector.run), 1).first() {
            Some(first) => eprintln!(
                "BS-53: plant detected: {}: {}",
                plant.label,
                first.chars().take(160).collect::<String>()
            ),
            None => {
                let got = super::evaluate_once(&low, &vector.run.0);
                eprintln!("BS-53: plant missed: {}: {got}", plant.label);
                missed.push(plant.label);
            }
        }
    }
    assert!(
        missed.is_empty(),
        "planted defects their own vector does not detect: {missed:?}"
    );
}

// --------------------------------------------------------------------- case

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
            "BS-53: {}: largest certificate part {largest} bytes",
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

#[allow(clippy::too_many_lines)]
pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("TomlRead"),
        "TomlRead is a linked module of the project"
    );
    eligibility_of(&project, "TomlRead", MODULE, &ROOTS);

    let vectors = named();
    let labels: std::collections::BTreeSet<&str> =
        vectors.iter().map(|vector| vector.label).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let refused = vectors
        .iter()
        .filter(|vector| vector.run.1 == serde_json::to_value(Value::None).expect("a value"))
        .count();
    assert!(refused >= 10, "at least ten vectors are refused: {refused}");

    let corpus = committed();
    assert!(
        corpus.len() >= 400,
        "the committed files in scope carry {} TOML documents",
        corpus.len()
    );
    let mut committed_runs: Vec<Run> = Vec::new();
    let mut spot_checked = 0usize;
    for (path, text) in &corpus {
        let table: toml::Table = text
            .parse()
            .unwrap_or_else(|error| panic!("{path}: the toml crate reads it: {error}"));
        let expected = expected_for(text, &table);
        committed_runs.push(parse_run(text, Some(expected)));
    }
    eprintln!(
        "BS-53: {} vectors, {} committed documents ({:?})",
        vectors.len(),
        corpus.len(),
        started.elapsed()
    );

    let normative = support::normative_host();
    let evaluator_seeded = if normative {
        seeded_runs(EVALUATOR_SEEDED)
    } else {
        Vec::new()
    };
    let rustc_seeded = if normative {
        seeded_runs(RUSTC_SEEDED)
    } else {
        Vec::new()
    };

    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, super::Lowered> = ROOTS
        .iter()
        .map(|(root, _)| (*root, super::lower(&checked, &project, MODULE, root)))
        .collect();
    let low = &lowered["tomlParse"];

    if normative {
        for (path, text) in corpus.iter().take(60) {
            let got = super::evaluate_once(low, &[super::text(text)]);
            let value = got["value"].clone();
            let row = json_top_row(&value).unwrap_or_else(|| panic!("{path}: a top row: {got}"));
            let table: toml::Table = text.parse().expect("already read once");
            for (key, theirs) in simple_fields(&table) {
                let mine = json_row_get(row, &key)
                    .unwrap_or_else(|| panic!("{path}: the top row has `{key}`"));
                assert!(
                    json_value_matches(mine, &theirs),
                    "{path}: `{key}` disagrees with the toml crate: {mine} vs {theirs:?}"
                );
                spot_checked += 1;
            }
        }
        eprintln!("BS-53: {spot_checked} fields spot-checked against the toml crate");

        let mut runs: Vec<Run> = vectors.iter().map(|vector| vector.run.clone()).collect();
        runs.extend(committed_runs.iter().cloned());
        runs.extend(evaluator_seeded.iter().cloned());
        let disagreements = super::evaluate(low, &runs, 5);
        assert!(
            disagreements.is_empty(),
            "tomlParse and the toml crate disagree: {disagreements:?}"
        );
        eprintln!(
            "BS-53: tomlParse: the evaluator agrees on {} runs ({:?})",
            runs.len(),
            started.elapsed()
        );
        plants(&vectors);
    } else {
        eprintln!("BS-53: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut compiled: Vec<Run> = vectors.into_iter().map(|vector| vector.run).collect();
    compiled.extend(committed_runs);
    compiled.extend(rustc_seeded);
    let suites: Vec<(&str, &super::Lowered, &[Run])> =
        vec![("tomlParse", low, compiled.as_slice())];
    let total = rustc_differential("BS-53", &suites, "the toml crate");
    eprintln!("BS-53: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-53") {
        return;
    }
    export(&lowered);
    eprintln!("BS-53: passed in {:?}", started.elapsed());
}
