//! The projector: `model/*.toml` as a generated view of the specification
//! project (R1, `NS-02`, `NS-04`; SPEC.md §27.5).
//!
//! The register, the diagnostic registry, the ledger and the authority
//! register are language-1.2 semantic modules of `spec/` (SPEC.md §17.21). This
//! reads those modules with the authoring SDK (§24.8), which needs no project
//! and links nothing, and writes the four model files from them. The reader
//! accepts exactly the shapes the sources are made of and refuses everything
//! else with the position of the declaration, so a module that merely looks
//! like data cannot be projected by accident: the failure model is closed (R5).
//!
//! This program is Rust. It can falsify a projection and never establishes
//! one (C9): what establishes it is the byte comparison with the committed
//! files and the plants that make that comparison fail.

use std::fmt::Write as _;
use std::path::Path;

use serde_json::{json, Value};

use crate::ModelError;

/// The directory of the specification project's sources, relative to the
/// repository root.
pub const SOURCE_DIR: &str = "spec/src";

/// What a field of a row holds in the source.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Text,
    Number,
    Optional,
    List,
}

/// One row type of the source and the table of the model file it projects to.
struct Table {
    /// The structure's name in `Rows`.
    row: &'static str,
    /// The `[[…]]` array of tables in the model file.
    array: &'static str,
    /// The fields in file order: source name, kind, and whether the model file
    /// carries the field.
    fields: &'static [(&'static str, Kind, bool)],
}

const IDS: Table = Table {
    row: "IdRow",
    array: "id",
    fields: &[
        ("id", Kind::Text, true),
        ("level", Kind::Text, true),
        ("suite", Kind::Text, true),
        ("statement", Kind::Text, true),
        // The §31 "Primary specification" column: it has no place in
        // `ids.toml`, so it is carried by the source alone.
        ("sections", Kind::Text, false),
    ],
};

const ERRORS: Table = Table {
    row: "ErrorRow",
    array: "error",
    fields: &[
        ("code", Kind::Text, true),
        ("class", Kind::Text, true),
        ("exit", Kind::Number, true),
        ("title", Kind::Text, true),
        ("statement", Kind::Text, true),
    ],
};

const LEDGER: Table = Table {
    row: "LedgerClaim",
    array: "claim",
    fields: &[
        ("id", Kind::Text, true),
        ("level", Kind::Text, true),
        ("statement", Kind::Text, true),
        ("authority", Kind::Text, true),
    ],
};

const AUTHORITIES: Table = Table {
    row: "AuthorityRow",
    array: "authority",
    fields: &[
        ("id", Kind::Text, true),
        ("name", Kind::Text, true),
        ("citation", Kind::Text, true),
        ("checksum", Kind::Text, true),
        ("checksumReason", Kind::Optional, true),
        ("vendored", Kind::Optional, true),
        ("statement", Kind::Text, true),
        ("realizedBy", Kind::List, true),
        ("issuer", Kind::Text, true),
        ("canonicalIdentifier", Kind::Text, true),
        ("edition", Kind::Text, true),
        ("sourceRole", Kind::Text, true),
        ("immutableUrl", Kind::Text, true),
        ("revision", Kind::Text, true),
        ("acquiredSha256", Kind::Text, true),
        ("signature", Kind::Text, true),
        ("mediaType", Kind::Text, true),
        ("license", Kind::Text, true),
        ("redistribution", Kind::Text, true),
        ("retrievalDate", Kind::Text, true),
        ("supersessionPolicy", Kind::Text, true),
    ],
};

const TABLES: [&Table; 4] = [&IDS, &ERRORS, &LEDGER, &AUTHORITIES];

/// The modules of the project that are not register families, with the
/// definitions each holds: the header comment and the schema tag of each model
/// file.
const HEADERS: [(&str, Kind); 8] = [
    ("idsHeader", Kind::List),
    ("idsSpec", Kind::Text),
    ("errorsHeader", Kind::List),
    ("errorsSpec", Kind::Text),
    ("ledgerHeader", Kind::List),
    ("ledgerSpec", Kind::Text),
    ("authoritiesHeader", Kind::List),
    ("authoritiesSpec", Kind::Text),
];

/// What the sources project to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Projection {
    /// The bytes of `model/ids.toml`.
    pub ids: String,
    /// The bytes of `model/errors.toml`.
    pub errors: String,
    /// The bytes of `model/ledger.toml`.
    pub ledger: String,
    /// The bytes of `model/authorities.toml`.
    pub authorities: String,
    /// The bytes of `SPEC.md`.
    pub spec: String,
    /// Each register row's §31 "Primary specification" column, in register
    /// order.
    pub sections: Vec<(String, String)>,
}

impl Projection {
    /// The projected files as `(path under the repository root, bytes)`.
    #[must_use]
    pub fn files(&self) -> [(&'static str, &str); 5] {
        [
            ("SPEC.md", &self.spec),
            ("model/ids.toml", &self.ids),
            ("model/errors.toml", &self.errors),
            ("model/ledger.toml", &self.ledger),
            ("model/authorities.toml", &self.authorities),
        ]
    }
}

impl Projection {
    /// R1, `NS-03`: each committed model file equals its projection, byte for
    /// byte. A file is a generated view, so the failure says to edit the
    /// module.
    ///
    /// # Errors
    ///
    /// Fails naming the first model file that differs or cannot be read.
    pub fn check_committed(&self, root: &Path) -> Result<(), ModelError> {
        let mut differing = Vec::new();
        for (path, projected) in self.files() {
            let committed = std::fs::read_to_string(root.join(path))
                .map_err(|e| ModelError::Io(root.join(path), e))?;
            if committed != projected {
                differing.push(path);
            }
        }
        if differing.is_empty() {
            return Ok(());
        }
        Err(ModelError::Inconsistent(format!(
            "R1: {} differ{} from the projection of {SOURCE_DIR}: a generated view is never \
             edited, so edit the module and run `just model-write` (NS-03)",
            differing.join(", "),
            if differing.len() == 1 { "s" } else { "" }
        )))
    }

    /// `NS-03`: the first row whose "Primary specification" cell in the §31
    /// table (`(id, cell)`, in any order) is not the one its source carries,
    /// described; `None` when every cell of the table is its source's.
    #[must_use]
    pub fn first_section_difference(&self, table: &[(String, String)]) -> Option<String> {
        table.iter().find_map(|(id, cell)| {
            let source = self
                .sections
                .iter()
                .find(|(source_id, _)| source_id == id)
                .map(|(_, source)| source.as_str());
            (source != Some(cell.as_str())).then(|| {
                format!(
                    "RP-07: `{id}` cites `{cell}` in the table but its source cites `{}` (NS-03)",
                    source.unwrap_or("nothing")
                )
            })
        })
    }
}

/// A literal read from a source.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Lit {
    Text(String),
    Number(u64),
    Optional(Option<String>),
    List(Vec<String>),
}

/// A source file read for projection, with where its declarations are.
struct Module {
    file: String,
    declarations: Vec<Value>,
    spans: Vec<(usize, usize)>,
}

impl Module {
    fn load(root: &Path, name: &str) -> Result<Self, ModelError> {
        let relative = format!("{SOURCE_DIR}/{name}.lex.tex");
        let path = root.join(&relative);
        let text = std::fs::read_to_string(&path).map_err(|e| ModelError::Io(path, e))?;
        let refuse = |message: String| {
            ModelError::Inconsistent(format!("R1: {relative}: {message} (NS-04)"))
        };
        let data = lexlean::sdk::read_module(&text).map_err(|error| {
            refuse(format!(
                "not a module of the authoring notation: {}{}",
                error.message,
                error
                    .offset
                    .map_or_else(String::new, |offset| format!(" at byte {offset}"))
            ))
        })?;
        let spans = lexlean::sdk::declaration_spans(&text)
            .map_err(|error| refuse(error.message))?
            .ok_or_else(|| refuse("the module is not written in the notation".to_owned()))?;
        let object = data
            .as_object()
            .ok_or_else(|| refuse("the module is not an object".to_owned()))?;
        if object.len() != 2 || object.get("spec") != Some(&json!("lexlean/semantic-module/2")) {
            return Err(refuse(
                "the module is not a `lexlean/semantic-module/2` with only `spec` and `declarations`"
                    .to_owned(),
            ));
        }
        let Some(Value::Array(declarations)) = object.get("declarations") else {
            return Err(refuse("the module has no declarations".to_owned()));
        };
        if declarations.len() != spans.len() {
            return Err(refuse(
                "the declarations and their source spans differ in number".to_owned(),
            ));
        }
        Ok(Self {
            file: relative,
            declarations: declarations.clone(),
            spans,
        })
    }

    /// A refusal naming the declaration at `index`, by position.
    fn refuse(&self, index: usize, message: &str) -> ModelError {
        let name = self
            .declarations
            .get(index)
            .and_then(|declaration| declaration.get("name"))
            .and_then(Value::as_str)
            .unwrap_or("?");
        let at = self.spans.get(index).map_or(0, |span| span.0);
        ModelError::Inconsistent(format!(
            "R1: {}: declaration {} `{name}` (byte {at} of the module body): {message} (NS-04)",
            self.file,
            index + 1
        ))
    }

    /// The module's `definition` at `index`: its name and the body that a
    /// literal of `result` is read from.
    fn definition(&self, index: usize, result: &Value) -> Result<(&str, &Value), ModelError> {
        let declaration = &self.declarations[index];
        let object = declaration
            .as_object()
            .ok_or_else(|| self.refuse(index, "not an object"))?;
        let keys: Vec<&str> = object.keys().map(String::as_str).collect();
        if keys != ["body", "executable", "kind", "name", "parameters", "result"]
            || object["kind"] != "definition"
        {
            return Err(self.refuse(
                index,
                "a row is a `definition` with exactly a name, no parameters, a result, a body and `executable`",
            ));
        }
        if object["executable"] != true || object["parameters"] != json!([]) {
            return Err(self.refuse(
                index,
                "a row is an executable definition without parameters",
            ));
        }
        if &object["result"] != result {
            return Err(self.refuse(index, "the definition's result type is not the row's"));
        }
        let name = object["name"]
            .as_str()
            .ok_or_else(|| self.refuse(index, "the name is not text"))?;
        Ok((name, &object["body"]))
    }
}

/// The name of the definition that holds the row `key`: the words of the key
/// joined in camel case (`RP-01` is `rp01`, `LLB6001` is `llb6001`,
/// `AUTH-LEAN-REL-4-32-1` is `authLeanRel4321`).
fn definition_name(key: &str) -> String {
    let mut name = String::new();
    for (position, word) in key
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty())
        .enumerate()
    {
        let lower = word.to_ascii_lowercase();
        if position == 0 {
            name.push_str(&lower);
        } else {
            let mut chars = lower.chars();
            if let Some(first) = chars.next() {
                name.push(first.to_ascii_uppercase());
                name.push_str(chars.as_str());
            }
        }
    }
    name
}

fn snake(name: &str) -> String {
    let mut out = String::new();
    for c in name.chars() {
        if c.is_ascii_uppercase() {
            out.push('_');
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

fn exact(object: &serde_json::Map<String, Value>, keys: &[&str]) -> bool {
    object.len() == keys.len() && keys.iter().all(|key| object.contains_key(*key))
}

/// A string literal term.
fn text_of(term: &Value) -> Option<String> {
    let object = term.as_object()?;
    if !exact(object, &["kind", "value"]) || object["kind"] != "string" {
        return None;
    }
    object["value"].as_str().map(str::to_owned)
}

fn list_of(term: &Value) -> Option<Vec<String>> {
    let mut items = Vec::new();
    let mut current = term;
    loop {
        let object = current.as_object()?;
        match object.get("kind")?.as_str()? {
            "nil" => {
                return (exact(object, &["element", "kind"])
                    && object["element"] == json!({"kind": "string"}))
                .then_some(items);
            }
            "cons" if exact(object, &["head", "kind", "tail"]) => {
                items.push(text_of(&object["head"])?);
                current = &object["tail"];
            }
            _ => return None,
        }
    }
}

fn optional_of(term: &Value) -> Option<Option<String>> {
    let object = term.as_object()?;
    if !exact(
        object,
        &["arguments", "constructor", "kind", "type_arguments"],
    ) || object["kind"] != "constructor"
        || object["type_arguments"] != json!([{"kind": "string"}])
    {
        return None;
    }
    let arguments = object["arguments"].as_array()?;
    match (
        object["constructor"].get("name")?.as_str()?,
        arguments.as_slice(),
    ) {
        ("Option.none", []) => Some(None),
        ("Option.some", [argument]) => Some(Some(text_of(argument)?)),
        _ => None,
    }
}

fn literal(term: &Value, kind: Kind) -> Option<Lit> {
    match kind {
        Kind::Text => text_of(term).map(Lit::Text),
        Kind::List => list_of(term).map(Lit::List),
        Kind::Optional => optional_of(term).map(Lit::Optional),
        Kind::Number => {
            let object = term.as_object()?;
            if !exact(object, &["kind", "value"]) || object["kind"] != "nat" {
                return None;
            }
            object["value"].as_str()?.parse().ok().map(Lit::Number)
        }
    }
}

fn type_json(kind: Kind) -> Value {
    match kind {
        Kind::Text => json!({"kind": "string"}),
        Kind::Number => json!({"kind": "nat"}),
        Kind::Optional => json!({"kind": "option", "value": {"kind": "string"}}),
        Kind::List => json!({"element": {"kind": "string"}, "kind": "list"}),
    }
}

fn named(row: &str) -> Value {
    json!({"arguments": [], "kind": "named", "member": {"module": "Rows", "name": row}})
}

/// `Rows` declares the four row structures and nothing else.
fn check_rows(module: &Module) -> Result<(), ModelError> {
    if module.declarations.len() != TABLES.len() {
        return Err(module.refuse(
            module.declarations.len().min(TABLES.len()),
            "`Rows` holds exactly the four row structures",
        ));
    }
    for (index, table) in TABLES.iter().enumerate() {
        let expected = json!({
            "fields": table.fields.iter()
                .map(|(name, kind, _)| json!({"name": name, "type": type_json(*kind)}))
                .collect::<Vec<_>>(),
            "kind": "structure",
            "name": table.row,
            "parameters": [],
            "type_parameters": [],
        });
        if module.declarations[index] != expected {
            return Err(module.refuse(
                index,
                &format!(
                    "the structure `{}` is not the one the projector reads",
                    table.row
                ),
            ));
        }
    }
    Ok(())
}

/// The rows of `module` for `table`, in source order, each a list of the
/// literals of its fields. `key` is the row's identifier field.
fn rows(module: &Module, table: &Table) -> Result<Vec<Vec<Lit>>, ModelError> {
    let mut out = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for index in 0..module.declarations.len() {
        let (name, body) = module.definition(index, &named(table.row))?;
        let record = body
            .as_object()
            .filter(|object| {
                exact(object, &["fields", "kind", "type"])
                    && object["kind"] == "record"
                    && object["type"] == json!({"module": "Rows", "name": table.row})
            })
            .ok_or_else(|| {
                module.refuse(index, "the body is not a record of the row's structure")
            })?;
        let fields = record["fields"]
            .as_array()
            .filter(|fields| fields.len() == table.fields.len())
            .ok_or_else(|| module.refuse(index, "the record does not have the row's fields"))?;
        let mut values = Vec::new();
        for (field, (expected, kind, _)) in fields.iter().zip(table.fields) {
            let valid = field
                .as_object()
                .filter(|object| exact(object, &["field", "value"]) && object["field"] == *expected)
                .and_then(|object| literal(&object["value"], *kind));
            values.push(valid.ok_or_else(|| {
                module.refuse(
                    index,
                    &format!(
                        "field `{expected}` is missing, out of order, or not a literal of its type"
                    ),
                )
            })?);
        }
        let Some(Lit::Text(key)) = values.first() else {
            return Err(module.refuse(index, "a row has no identifier"));
        };
        if definition_name(key) != name {
            return Err(module.refuse(
                index,
                &format!(
                    "the definition is not named for its row `{key}` (`{}`)",
                    definition_name(key)
                ),
            ));
        }
        if !seen.insert(key.clone()) {
            return Err(module.refuse(index, &format!("`{key}` is registered twice")));
        }
        out.push(values);
    }
    Ok(out)
}

fn quote(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            c if c.is_control() => {
                let _ = write!(out, "\\u{:04X}", u32::from(c));
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn emit(header: &[String], spec: &str, table: &Table, rows: &[Vec<Lit>]) -> String {
    let mut out = header.join("\n");
    let _ = write!(out, "\n\nspec = {}\n", quote(spec));
    for row in rows {
        let _ = write!(out, "\n[[{}]]\n", table.array);
        for ((name, _, projected), value) in table.fields.iter().zip(row) {
            if !projected {
                continue;
            }
            let key = snake(name);
            match value {
                Lit::Text(text) => {
                    let _ = writeln!(out, "{key} = {}", quote(text));
                }
                Lit::Number(number) => {
                    let _ = writeln!(out, "{key} = {number}");
                }
                Lit::Optional(Some(text)) => {
                    let _ = writeln!(out, "{key} = {}", quote(text));
                }
                Lit::Optional(None) => {}
                Lit::List(items) => {
                    let items: Vec<String> = items.iter().map(|item| quote(item)).collect();
                    let _ = writeln!(out, "{key} = [{}]", items.join(", "));
                }
            }
        }
    }
    out
}

/// The header comment and schema tag the `Headers` module holds, by definition.
fn headers(module: &Module) -> Result<Vec<(Vec<String>, String)>, ModelError> {
    if module.declarations.len() != HEADERS.len() {
        return Err(module.refuse(
            module.declarations.len().min(HEADERS.len()),
            "`Headers` holds exactly the header and schema tag of the four model files",
        ));
    }
    let mut values = Vec::new();
    for (index, (expected, kind)) in HEADERS.iter().enumerate() {
        let (name, body) = module.definition(index, &type_json(*kind))?;
        if name != *expected {
            return Err(module.refuse(index, &format!("expected the definition `{expected}`")));
        }
        values.push(literal(body, *kind).ok_or_else(|| {
            module.refuse(index, "the body is not a literal of the definition's type")
        })?);
    }
    let mut out = Vec::new();
    for pair in values.chunks(2) {
        let (Lit::List(lines), Lit::Text(spec)) = (&pair[0], &pair[1]) else {
            return Err(module.refuse(0, "a header is a list of lines followed by a tag"));
        };
        if lines.iter().any(|line| !line.starts_with('#')) {
            return Err(module.refuse(0, "a header line is a TOML comment"));
        }
        out.push((lines.clone(), spec.clone()));
    }
    Ok(out)
}

/// The families of the register, in order, from `Order`.
fn families(module: &Module) -> Result<Vec<String>, ModelError> {
    if module.declarations.len() != 1 {
        return Err(module.refuse(
            module.declarations.len().min(1),
            "`Order` holds exactly `registerFamilies`",
        ));
    }
    let (name, body) = module.definition(0, &type_json(Kind::List))?;
    if name != "registerFamilies" {
        return Err(module.refuse(0, "expected the definition `registerFamilies`"));
    }
    let Some(Lit::List(families)) = literal(body, Kind::List) else {
        return Err(module.refuse(0, "the body is not a list of text"));
    };
    let mut unique = std::collections::BTreeSet::new();
    for family in &families {
        let shaped = family.len() == 2 && family.bytes().all(|b| b.is_ascii_uppercase());
        if !shaped || !unique.insert(family.clone()) {
            return Err(module.refuse(
                0,
                &format!("`{family}` is not a family of two capital letters listed once"),
            ));
        }
    }
    Ok(families)
}

/// One item of the document, read from a definition of a chapter module.
enum Item {
    Heading(u64, u64, String),
    Block(Block, Vec<String>),
    Derived(String),
    Continue(Vec<String>),
}

#[derive(Clone)]
enum Block {
    Paragraph,
    Table,
    Fence(String),
}

/// The constructors of `DocRows.Item` with the kind of each argument.
const ITEMS: [(&str, &[Kind]); 6] = [
    ("heading", &[Kind::Number, Kind::Number, Kind::Text]),
    ("para", &[Kind::List]),
    ("table", &[Kind::List]),
    ("fence", &[Kind::Text, Kind::List]),
    ("derived", &[Kind::Text]),
    ("cont", &[Kind::List]),
];

/// The most lines one item holds: the notation reads a nested term to a fixed
/// depth (SPEC.md §17.21.3), and a longer block continues in a `cont` item.
pub const ITEM_LINES: usize = 60;

fn item_type() -> Value {
    json!({"arguments": [], "kind": "named", "member": {"module": "DocRows", "name": "Item"}})
}

/// `DocRows` declares the one inductive the chapters are made of.
fn check_item_type(module: &Module) -> Result<(), ModelError> {
    let list = type_json(Kind::List);
    let expected = json!({
        "constructors": [
            {"fields": [{"kind": "nat"}, {"kind": "nat"}, {"kind": "string"}], "name": "heading"},
            {"fields": [list], "name": "para"},
            {"fields": [list], "name": "table"},
            {"fields": [{"kind": "string"}, list], "name": "fence"},
            {"fields": [{"kind": "string"}], "name": "derived"},
            {"fields": [list], "name": "cont"},
        ],
        "kind": "inductive",
        "name": "Item",
        "parameters": [],
        "type_parameters": [],
    });
    if module.declarations != [expected] {
        return Err(module.refuse(
            0,
            "`DocRows` holds exactly the inductive `Item` the projector reads",
        ));
    }
    Ok(())
}

/// A source cannot hold a raw percent sign (SPEC.md §12.1, `LLL1002`), and its
/// formatter would write one back, so the document spells it as the fullwidth
/// percent sign, which no text of the document uses.
fn percent(text: &str) -> String {
    text.replace('\u{ff05}', "%")
}

fn read_item(module: &Module, index: usize) -> Result<Item, ModelError> {
    let (name, body) = module.definition(index, &item_type())?;
    if name != format!("i{:04}", index + 1) {
        return Err(module.refuse(
            index,
            &format!("items are numbered in order: expected `i{:04}`", index + 1),
        ));
    }
    let object = body
        .as_object()
        .filter(|object| {
            exact(
                object,
                &["arguments", "constructor", "kind", "type_arguments"],
            ) && object["kind"] == "constructor"
                && object["type_arguments"] == json!([])
        })
        .ok_or_else(|| module.refuse(index, "the body is not a constructor of `DocRows.Item`"))?;
    let constructor = object["constructor"]
        .as_object()
        .filter(|constructor| constructor["module"] == "DocRows")
        .and_then(|constructor| constructor["name"].as_str())
        .and_then(|name| name.strip_prefix("Item."))
        .and_then(|name| ITEMS.iter().find(|(known, _)| *known == name))
        .ok_or_else(|| module.refuse(index, "the constructor is not one of `DocRows.Item`"))?;
    let arguments = object["arguments"]
        .as_array()
        .filter(|arguments| arguments.len() == constructor.1.len())
        .ok_or_else(|| module.refuse(index, "the constructor has the wrong number of arguments"))?;
    let mut values = Vec::new();
    for (argument, kind) in arguments.iter().zip(constructor.1) {
        let value = literal(argument, *kind).ok_or_else(|| {
            module.refuse(
                index,
                "an argument is not a literal of its constructor's type",
            )
        })?;
        values.push(match value {
            Lit::Text(text) => Lit::Text(percent(&text)),
            Lit::List(lines) => Lit::List(lines.iter().map(|line| percent(line)).collect()),
            other => other,
        });
    }
    let lines = |lit: &Lit| match lit {
        Lit::List(lines) if lines.len() <= ITEM_LINES => Some(lines.clone()),
        _ => None,
    };
    let refuse_lines = || {
        module.refuse(
            index,
            &format!(
                "an item holds at most {ITEM_LINES} lines; the rest continues in a `cont` item"
            ),
        )
    };
    Ok(match (constructor.0, values.as_slice()) {
        ("heading", [Lit::Number(level), Lit::Number(gap), Lit::Text(text)]) => {
            if !(1..=4).contains(level) {
                return Err(module.refuse(index, "a heading has a level from 1 to 4"));
            }
            Item::Heading(*level, *gap, text.clone())
        }
        ("para", [lit]) => Item::Block(Block::Paragraph, lines(lit).ok_or_else(refuse_lines)?),
        ("table", [lit]) => Item::Block(Block::Table, lines(lit).ok_or_else(refuse_lines)?),
        ("fence", [Lit::Text(info), lit]) => Item::Block(
            Block::Fence(info.clone()),
            lines(lit).ok_or_else(refuse_lines)?,
        ),
        ("derived", [Lit::Text(name)]) => Item::Derived(name.clone()),
        ("cont", [lit]) => Item::Continue(lines(lit).ok_or_else(refuse_lines)?),
        _ => return Err(module.refuse(index, "the constructor's arguments are not read")),
    })
}

/// The lines a `derived` item stands for, from the register and the registry.
fn derived(name: &str, register: &[Vec<Lit>], errors: &[Vec<Lit>]) -> Option<Vec<String>> {
    let text = |lit: &Lit| match lit {
        Lit::Text(text) => text.clone(),
        _ => String::new(),
    };
    Some(match name {
        "register-table" => {
            let mut lines = vec![
                "| ID | Suite | Normative statement | Primary specification |".to_owned(),
                "|---|---|---|---|".to_owned(),
            ];
            for row in register {
                lines.push(format!(
                    "| `{}` | `{}` | {} | {} |",
                    text(&row[0]),
                    text(&row[2]),
                    text(&row[3]),
                    text(&row[4])
                ));
            }
            lines
        }
        "error-table" => {
            let mut lines = vec!["| Code | Meaning |".to_owned(), "|---|---|".to_owned()];
            for row in errors {
                lines.push(format!("| `{}` | {} |", text(&row[0]), text(&row[4])));
            }
            lines
        }
        "total-ids" => vec![format!(
            "**Total required capability IDs:** {}.",
            register.len()
        )],
        _ => return None,
    })
}

/// The chapter modules named by `DocOrder`, in order.
fn chapter_list(module: &Module) -> Result<Vec<String>, ModelError> {
    if module.declarations.len() != 1 {
        return Err(module.refuse(
            module.declarations.len().min(1),
            "`DocOrder` holds exactly `documentChapters`",
        ));
    }
    let (name, body) = module.definition(0, &type_json(Kind::List))?;
    let Some(Lit::List(chapters)) =
        literal(body, Kind::List).filter(|_| name == "documentChapters")
    else {
        return Err(module.refuse(0, "expected `documentChapters`, a list of text"));
    };
    let mut unique = std::collections::BTreeSet::new();
    for chapter in &chapters {
        let shaped = chapter
            .strip_prefix("Chapter")
            .is_some_and(|digits| digits.len() == 2 && digits.bytes().all(|b| b.is_ascii_digit()));
        if !shaped || !unique.insert(chapter.clone()) {
            return Err(module.refuse(
                0,
                &format!("`{chapter}` is not a chapter module named `Chapter` and two digits, listed once"),
            ));
        }
    }
    Ok(chapters)
}

/// SPEC.md, rendered from the chapter modules and the registers.
fn document(
    root: &Path,
    chapters: &[String],
    register: &[Vec<Lit>],
    errors: &[Vec<Lit>],
) -> Result<String, ModelError> {
    check_item_type(&Module::load(root, "DocRows")?)?;
    let mut lines: Vec<String> = Vec::new();
    for chapter in chapters {
        let module = Module::load(root, chapter)?;
        // The block a `cont` item continues, and whether a fence is still open.
        let mut open: Option<(Block, usize)> = None;
        let close = |lines: &mut Vec<String>, open: &mut Option<(Block, usize)>| {
            if let Some((Block::Fence(_), _)) = open.take() {
                lines.push("```".to_owned());
            }
        };
        for index in 0..module.declarations.len() {
            match read_item(&module, index)? {
                Item::Heading(level, gap, text) => {
                    close(&mut lines, &mut open);
                    lines.extend((0..gap).map(|_| String::new()));
                    lines.push(format!(
                        "{} {text}",
                        "#".repeat(usize::try_from(level).unwrap_or(0))
                    ));
                }
                Item::Block(block, body) => {
                    close(&mut lines, &mut open);
                    lines.push(String::new());
                    if let Block::Fence(info) = &block {
                        lines.push(format!("```{info}"));
                    }
                    lines.extend(body);
                    open = Some((block, index));
                }
                Item::Derived(name) => {
                    close(&mut lines, &mut open);
                    let expanded = derived(&name, register, errors).ok_or_else(|| {
                        module.refuse(index, &format!("`{name}` is not a derived block"))
                    })?;
                    lines.push(String::new());
                    lines.extend(expanded);
                }
                Item::Continue(body) => {
                    if open.is_none() {
                        return Err(module
                            .refuse(index, "a `cont` item follows a paragraph, table or fence"));
                    }
                    lines.extend(body);
                }
            }
        }
        close(&mut lines, &mut open);
    }
    let mut text = lines.join("\n");
    text.push('\n');
    Ok(text)
}

/// Project the specification project's register modules to the model files.
///
/// # Errors
///
/// Fails closed (R5) when a module is missing, is not a well-formed module of
/// the authoring notation, or holds any declaration, term, field, name or
/// ordering the projector does not read, and when a module of the source
/// directory belongs to no part of the projection.
pub fn project(root: &Path) -> Result<Projection, ModelError> {
    check_rows(&Module::load(root, "Rows")?)?;
    let headers = headers(&Module::load(root, "Headers")?)?;
    let families = families(&Module::load(root, "Order")?)?;

    let mut register = Vec::new();
    let mut sections = Vec::new();
    for family in &families {
        let module = Module::load(root, &format!("Ids{family}"))?;
        for (index, row) in rows(&module, &IDS)?.into_iter().enumerate() {
            let (Lit::Text(id), Lit::Text(listed)) = (&row[0], &row[4]) else {
                return Err(module.refuse(index, "a register row has an identifier and sections"));
            };
            if !id.starts_with(&format!("{family}-")) {
                return Err(module.refuse(
                    index,
                    &format!("`{id}` is not a row of the family `{family}`"),
                ));
            }
            sections.push((id.clone(), listed.clone()));
            register.push(row);
        }
    }
    let mut seen = std::collections::BTreeSet::new();
    for row in &register {
        let Lit::Text(id) = &row[0] else { continue };
        if !seen.insert(id) {
            return Err(ModelError::Inconsistent(format!(
                "R1: {SOURCE_DIR}: `{id}` is registered in two families (NS-04)"
            )));
        }
    }

    let chapter_names = chapter_list(&Module::load(root, "DocOrder")?)?;
    let known: std::collections::BTreeSet<String> = [
        "Rows",
        "Headers",
        "Order",
        "Errors",
        "Ledger",
        "Authorities",
        "DocRows",
        "DocOrder",
    ]
    .into_iter()
    .map(str::to_owned)
    .chain(families.iter().map(|family| format!("Ids{family}")))
    .chain(chapter_names.iter().cloned())
    .collect();
    let directory = root.join(SOURCE_DIR);
    let listing =
        std::fs::read_dir(&directory).map_err(|e| ModelError::Io(directory.clone(), e))?;
    for entry in listing {
        let entry = entry.map_err(|e| ModelError::Io(directory.clone(), e))?;
        let file = entry.file_name().to_string_lossy().into_owned();
        let stem = file.strip_suffix(".lex.tex");
        if !stem.is_some_and(|stem| known.contains(stem)) {
            return Err(ModelError::Inconsistent(format!(
                "R1: {SOURCE_DIR}/{file}: a file that belongs to no part of the projection (NS-04)"
            )));
        }
    }

    let errors = rows(&Module::load(root, "Errors")?, &ERRORS)?;
    let ledger = rows(&Module::load(root, "Ledger")?, &LEDGER)?;
    let authorities = rows(&Module::load(root, "Authorities")?, &AUTHORITIES)?;
    let [(ids_header, ids_spec), (errors_header, errors_spec), (ledger_header, ledger_spec), (authorities_header, authorities_spec)] =
        <[_; 4]>::try_from(headers).map_err(|_| {
            ModelError::Inconsistent("R1: Headers does not hold four headers (NS-04)".to_owned())
        })?;
    let spec = document(root, &chapter_names, &register, &errors)?;
    Ok(Projection {
        spec,
        ids: emit(&ids_header, &ids_spec, &IDS, &register),
        errors: emit(&errors_header, &errors_spec, &ERRORS, &errors),
        ledger: emit(&ledger_header, &ledger_spec, &LEDGER, &ledger),
        authorities: emit(
            &authorities_header,
            &authorities_spec,
            &AUTHORITIES,
            &authorities,
        ),
        sections,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The committed model files are exactly the projection of the sources.
    #[test]
    fn the_sources_project_to_the_committed_model() {
        let root = crate::repo_root();
        let projection = project(&root).expect("the sources project");
        for (path, projected) in projection.files() {
            let committed = std::fs::read_to_string(root.join(path)).expect("model file");
            if committed != projected {
                let at = committed
                    .bytes()
                    .zip(projected.bytes())
                    .position(|(a, b)| a != b)
                    .unwrap_or(committed.len().min(projected.len()));
                panic!(
                    "{path}: first difference at byte {at}: {:?} vs {:?}",
                    &committed[at.saturating_sub(40)..(at + 40).min(committed.len())],
                    &projected[at.saturating_sub(40)..(at + 40).min(projected.len())]
                );
            }
        }
    }
}
