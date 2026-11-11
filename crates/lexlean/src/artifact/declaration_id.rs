//! Declaration IDs (SPEC.md §21.9): a content address for every declaration
//! of a language-1.2 semantic module, and for every theorem's statement.
//!
//! A declaration's ID covers its canonical JSON and the IDs of the project
//! declarations it references, so the IDs form a hash chain: changing one
//! declaration changes its ID and the IDs of the declarations that reach it by
//! reference, and nothing else. Declarations that reference one another are
//! one strongly connected group and are hashed together.

use std::collections::{BTreeMap, BTreeSet};

use crate::artifact::canonical_json::Json;
use crate::artifact::content_id::{FramedHasher, Sha256Digest};
use crate::ir::semantic::{SemanticDeclaration, SemanticModule};

/// The schema tag of a published declarations file.
pub const DECLARATIONS_SPEC: &str = "lexlean/declarations/1";

/// A project declaration: its module and its name.
type Key = (String, String);

/// One declaration's identity, in source order within its module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclarationRow {
    /// The declaration's name.
    pub name: String,
    /// The declaration's kind (`definition`, `theorem`, ...).
    pub kind: &'static str,
    /// The declaration ID (§21.9).
    pub id: Sha256Digest,
    /// For a theorem, the ID of its statement without its proof.
    pub statement_id: Option<Sha256Digest>,
    /// The project declarations it references: `None` for its own module.
    pub references: Vec<(Option<String>, String)>,
}

/// The fixed inputs every recipe frames first.
struct Context<'a> {
    compiler_semantics: Sha256Digest,
    language: &'a str,
}

impl Context<'_> {
    fn hasher(&self, domain: &str) -> FramedHasher {
        let mut hasher = FramedHasher::new(domain);
        hasher.frame("compiler-semantics", &self.compiler_semantics.0);
        hasher.frame("language", self.language.as_bytes());
        hasher.frame("toolchain", crate::LEAN_TOOLCHAIN.as_bytes());
        hasher
    }
}

/// A declaration as hashed: its canonical JSON with references to its own
/// module written without `module`, and the project declarations it
/// references.
struct Prepared {
    json: Json,
    references: BTreeSet<Key>,
    statement: Option<(Json, BTreeSet<Key>)>,
}

/// Every declaration ID of the semantic modules of a project, by module, each
/// module's rows in source order.
///
/// # Panics
///
/// Never on a linked project: declarations serialize to canonical JSON.
#[must_use]
pub fn declaration_ids(
    compiler_semantics: Sha256Digest,
    language: &str,
    modules: &[(&str, &SemanticModule)],
) -> BTreeMap<String, Vec<DeclarationRow>> {
    let context = Context {
        compiler_semantics,
        language,
    };
    let known: BTreeSet<Key> = modules
        .iter()
        .flat_map(|(module, semantic)| {
            semantic
                .declarations
                .iter()
                .map(|declaration| ((*module).to_owned(), declaration.name().to_owned()))
        })
        .collect();
    let mut prepared: BTreeMap<Key, Prepared> = BTreeMap::new();
    for (module, semantic) in modules {
        for declaration in &semantic.declarations {
            prepared.insert(
                ((*module).to_owned(), declaration.name().to_owned()),
                prepare(module, declaration, &known),
            );
        }
    }

    let ids = hash_groups(&context, &prepared);

    let mut out = BTreeMap::new();
    for (module, semantic) in modules {
        let rows = semantic
            .declarations
            .iter()
            .map(|declaration| {
                let key = ((*module).to_owned(), declaration.name().to_owned());
                let entry = &prepared[&key];
                DeclarationRow {
                    name: key.1.clone(),
                    kind: declaration.kind(),
                    id: ids[&key],
                    statement_id: entry.statement.as_ref().map(|(json, references)| {
                        let mut hasher = context.hasher("lexlean-statement-v1");
                        hasher.frame("statement", json.to_canonical_string().as_bytes());
                        frame_references(&mut hasher, module, references, &ids);
                        hasher.finish()
                    }),
                    references: entry
                        .references
                        .iter()
                        .map(|(target, name)| {
                            ((target != module).then(|| target.clone()), name.clone())
                        })
                        .collect(),
                }
            })
            .collect();
        out.insert((*module).to_owned(), rows);
    }
    out
}

/// The canonical JSON of one declaration's rows, the published file (§21.9).
#[must_use]
pub fn declarations_json(module: &str, rows: &[DeclarationRow]) -> Json {
    Json::object(vec![
        ("spec", Json::Str(DECLARATIONS_SPEC.to_owned())),
        ("module", Json::Str(module.to_owned())),
        (
            "declarations",
            Json::Arr(
                rows.iter()
                    .map(|row| {
                        let mut fields = vec![
                            ("name", Json::Str(row.name.clone())),
                            ("kind", Json::Str(row.kind.to_owned())),
                            ("id", Json::Str(row.id.to_hex())),
                            (
                                "references",
                                Json::Arr(
                                    row.references
                                        .iter()
                                        .map(|(module, name)| {
                                            let mut reference =
                                                vec![("name", Json::Str(name.clone()))];
                                            if let Some(module) = module {
                                                reference
                                                    .push(("module", Json::Str(module.clone())));
                                            }
                                            Json::object(reference)
                                        })
                                        .collect(),
                                ),
                            ),
                        ];
                        if let Some(statement) = row.statement_id {
                            fields.push(("statement_id", Json::Str(statement.to_hex())));
                        }
                        Json::object(fields)
                    })
                    .collect(),
            ),
        ),
    ])
}

fn prepare(module: &str, declaration: &SemanticDeclaration, known: &BTreeSet<Key>) -> Prepared {
    let raw = Json::parse(
        &serde_json::to_vec(declaration).expect("a declaration serializes to canonical JSON"),
    )
    .expect("a declaration uses canonical JSON values");
    let mut references = BTreeSet::new();
    let json = normalize(module, raw, known, &mut references);
    let own = (module.to_owned(), declaration.name().to_owned());
    references.remove(&own);
    let statement = matches!(declaration, SemanticDeclaration::Theorem { .. }).then(|| {
        let mut without_proof = json.clone();
        if let Json::Obj(fields) = &mut without_proof {
            fields.remove("proof");
        }
        let mut statement_references = BTreeSet::new();
        collect(module, &without_proof, known, &mut statement_references);
        statement_references.remove(&own);
        (without_proof, statement_references)
    });
    Prepared {
        json,
        references,
        statement,
    }
}

/// A member reference is an object whose keys are exactly `name`, or `module`
/// and `name`, with string values (§17.12).
fn member_reference(fields: &BTreeMap<String, Json>) -> Option<(Option<&str>, &str)> {
    let Some(Json::Str(name)) = fields.get("name") else {
        return None;
    };
    match (fields.len(), fields.get("module")) {
        (1, None) => Some((None, name)),
        (2, Some(Json::Str(module))) => Some((Some(module), name)),
        _ => None,
    }
}

/// The project declaration a member reference depends on: the declaration it
/// names, or, for a member the compiler generates for a declaration (a
/// constructor or a field, named `Owner.member`), its owner.
fn resolve(module: &str, target: Option<&str>, name: &str, known: &BTreeSet<Key>) -> Option<Key> {
    let target = target.unwrap_or(module);
    let mut candidate = name;
    loop {
        let key = (target.to_owned(), candidate.to_owned());
        if known.contains(&key) {
            return Some(key);
        }
        candidate = &candidate[..candidate.rfind('.')?];
    }
}

/// Write references to the own module without `module`, and record every
/// reference that names a project declaration.
fn normalize(
    module: &str,
    value: Json,
    known: &BTreeSet<Key>,
    references: &mut BTreeSet<Key>,
) -> Json {
    match value {
        Json::Obj(fields) => {
            if let Some((target, name)) = member_reference(&fields) {
                if let Some(key) = resolve(module, target, name, known) {
                    let own = key.0 == module;
                    references.insert(key);
                    if own {
                        let mut local = BTreeMap::new();
                        local.insert("name".to_owned(), fields["name"].clone());
                        return Json::Obj(local);
                    }
                }
                return Json::Obj(fields);
            }
            Json::Obj(
                fields
                    .into_iter()
                    .map(|(key, value)| (key, normalize(module, value, known, references)))
                    .collect(),
            )
        }
        Json::Arr(items) => Json::Arr(
            items
                .into_iter()
                .map(|item| normalize(module, item, known, references))
                .collect(),
        ),
        other => other,
    }
}

/// The project declarations a normalized value references.
fn collect(module: &str, value: &Json, known: &BTreeSet<Key>, references: &mut BTreeSet<Key>) {
    match value {
        Json::Obj(fields) => {
            if let Some((target, name)) = member_reference(fields) {
                if let Some(key) = resolve(module, target, name, known) {
                    references.insert(key);
                }
                return;
            }
            for value in fields.values() {
                collect(module, value, known, references);
            }
        }
        Json::Arr(items) => {
            for item in items {
                collect(module, item, known, references);
            }
        }
        _ => {}
    }
}

/// Frame each reference, by `(module, name)`: its name, its module when it is
/// another module's, and its declaration ID.
fn frame_references(
    hasher: &mut FramedHasher,
    module: &str,
    references: &BTreeSet<Key>,
    ids: &BTreeMap<Key, Sha256Digest>,
) {
    for key in references {
        if key.0 != module {
            hasher.frame("reference-module", key.0.as_bytes());
        }
        hasher.frame("reference", key.1.as_bytes());
        hasher.frame("reference-id", &ids[key].0);
    }
}

/// Hash every strongly connected group of the reference graph, referenced
/// groups first, and return every declaration's ID.
fn hash_groups(
    context: &Context<'_>,
    prepared: &BTreeMap<Key, Prepared>,
) -> BTreeMap<Key, Sha256Digest> {
    let mut ids = BTreeMap::new();
    // Tarjan's algorithm emits each component after every component it
    // reaches, which is the order the hash chain needs.
    for group in components(prepared) {
        let members: BTreeSet<&Key> = group.iter().collect();
        let mut hasher = context.hasher("lexlean-declaration-group-v1");
        for key in &members {
            hasher.frame(
                "declaration",
                prepared[*key].json.to_canonical_string().as_bytes(),
            );
        }
        // References leave the group from members of possibly different
        // modules; each is framed relative to the module of the group's
        // first member, which every member of a group shares (a cycle never
        // crosses modules, since imports are acyclic).
        let module = members
            .iter()
            .next()
            .map(|key| key.0.clone())
            .unwrap_or_default();
        let outside: BTreeSet<Key> = members
            .iter()
            .flat_map(|key| prepared[*key].references.iter().cloned())
            .filter(|key| !members.contains(key))
            .collect();
        frame_references(&mut hasher, &module, &outside, &ids);
        let group_digest = hasher.finish();
        for key in members {
            let mut member = FramedHasher::new("lexlean-declaration-v1");
            member.frame("group", &group_digest.0);
            member.frame("name", key.1.as_bytes());
            ids.insert(key.clone(), member.finish());
        }
    }
    ids
}

/// The strongly connected components of the reference graph, each after the
/// components it references (iterative Tarjan, so depth is not bounded by the
/// call stack).
fn components(prepared: &BTreeMap<Key, Prepared>) -> Vec<Vec<Key>> {
    let keys: Vec<&Key> = prepared.keys().collect();
    let index_of: BTreeMap<&Key, usize> = keys.iter().enumerate().map(|(i, k)| (*k, i)).collect();
    let edges: Vec<Vec<usize>> = keys
        .iter()
        .map(|key| {
            prepared[*key]
                .references
                .iter()
                .filter_map(|target| index_of.get(target).copied())
                .collect()
        })
        .collect();
    let count = keys.len();
    let mut index = vec![usize::MAX; count];
    let mut low = vec![0; count];
    let mut on_stack = vec![false; count];
    let mut stack = Vec::new();
    let mut next = 0;
    let mut out = Vec::new();
    for root in 0..count {
        if index[root] != usize::MAX {
            continue;
        }
        let mut work: Vec<(usize, usize)> = vec![(root, 0)];
        index[root] = next;
        low[root] = next;
        next += 1;
        stack.push(root);
        on_stack[root] = true;
        while let Some(&mut (node, ref mut edge)) = work.last_mut() {
            if let Some(&target) = edges[node].get(*edge) {
                *edge += 1;
                if index[target] == usize::MAX {
                    index[target] = next;
                    low[target] = next;
                    next += 1;
                    stack.push(target);
                    on_stack[target] = true;
                    work.push((target, 0));
                } else if on_stack[target] {
                    low[node] = low[node].min(index[target]);
                }
            } else {
                work.pop();
                if let Some(&(parent, _)) = work.last() {
                    low[parent] = low[parent].min(low[node]);
                }
                if low[node] == index[node] {
                    let mut group = Vec::new();
                    while let Some(member) = stack.pop() {
                        on_stack[member] = false;
                        group.push(keys[member].clone());
                        if member == node {
                            break;
                        }
                    }
                    out.push(group);
                }
            }
        }
    }
    out
}
