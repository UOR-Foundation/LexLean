//! Replay keys (SPEC.md §22.13): the canonical serialisation of a declaration
//! and the Merkle DAG of keys over it.
//!
//! [`canonical_declaration`] is the one function that decides which bytes
//! represent a declaration. Everything a key covers about a declaration is
//! read through it, so a replay granularity (one declaration, one module) is
//! only a choice of which keys a unit collects.

use std::collections::BTreeMap;

use crate::artifact::digest_set::{DigestSet, DigestSetWriter};
use crate::code;
use crate::diagnostic::Diagnostic;
use crate::verify::replay_cache::facts::DeclarationFacts;

const DECLARATION_FORMAT: &str = "lexlean/replay-declaration/1";
const KEY_FORMAT: &str = "lexlean/replay-key/1";
const UNIT_FORMAT: &str = "lexlean/replay-unit/1";

/// What the keys of one run are valid for: the key of every declaration
/// contains both strings, so a changed toolchain or Mathlib revision changes
/// every key and leaves nothing recorded under the old ones reachable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayEnvironment {
    /// The Lean toolchain string, as `lean-toolchain` holds it.
    pub toolchain: String,
    /// The Mathlib revision, a commit hash; empty for a project without Mathlib.
    pub mathlib_revision: String,
}

/// What a unit of replay is. A unit's key covers its granularity, so a
/// declaration replayed alone and the same declaration inside a module are
/// different records: what `leanchecker` ran over differs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Granularity {
    /// One declaration.
    Declaration,
    /// One module.
    Module,
}

impl Granularity {
    /// The word the entry and the unit key spell it with.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Declaration => "declaration",
            Self::Module => "module",
        }
    }

    /// Read the word [`Self::as_str`] writes.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        match text {
            "declaration" => Some(Self::Declaration),
            "module" => Some(Self::Module),
            _ => None,
        }
    }
}

/// Fields of a frame: a tag, a space, the decimal byte length of the payload,
/// a colon, the payload, a line feed. The length makes the encoding
/// unambiguous whatever bytes a payload holds.
#[derive(Default)]
struct Frame(Vec<u8>);

impl Frame {
    fn field(&mut self, tag: &str, payload: &[u8]) -> &mut Self {
        self.0.extend_from_slice(tag.as_bytes());
        self.0.push(b' ');
        self.0
            .extend_from_slice(payload.len().to_string().as_bytes());
        self.0.push(b':');
        self.0.extend_from_slice(payload);
        self.0.push(b'\n');
        self
    }

    fn digest(&mut self, tag: &str, digest: &DigestSet) -> &mut Self {
        self.field(&format!("{tag}-sha256"), digest.sha256.to_hex().as_bytes());
        self.field(&format!("{tag}-blake3"), digest.blake3.to_hex().as_bytes())
    }
}

/// The canonical bytes of a declaration: its name, universe parameters, kind
/// attributes and type as the exporter wrote them, and the DigestSet of its
/// value or proof term (or `none`). The declaring module is not among them, so
/// moving a declaration changes no key. Dependencies are not among them either:
/// they enter a key through the keys of the declarations they name.
#[must_use]
pub fn canonical_declaration(facts: &DeclarationFacts) -> Vec<u8> {
    let mut frame = Frame::default();
    frame.field("format", DECLARATION_FORMAT.as_bytes());
    frame.field("name", facts.name.as_bytes());
    frame.field("levels", facts.level_params.len().to_string().as_bytes());
    for level in &facts.level_params {
        frame.field("level", level.as_bytes());
    }
    frame.field("attrs", facts.attrs.as_bytes());
    frame.field("type", facts.type_text.as_bytes());
    match &facts.value {
        Some(value) => {
            frame.digest("value", &DigestSet::of(value.as_bytes()));
        }
        None => {
            frame.field("value", b"none");
        }
    }
    frame.0
}

/// The digest of a group of declarations that name one another: their
/// canonical bytes in name order. A declaration that is alone is a group of one.
fn group_digest(members: &[Vec<u8>]) -> DigestSet {
    let mut writer = DigestSetWriter::new();
    let mut frame = Frame::default();
    for member in members {
        frame.0.clear();
        frame.field("member", member);
        writer.update(&frame.0);
    }
    writer.finish()
}

/// The key of a declaration: the environment, its canonical bytes, its group
/// digest, and the keys of what it names outside its group.
fn declaration_key(
    environment: &ReplayEnvironment,
    canonical: &[u8],
    group: &DigestSet,
    dependencies: &mut Vec<DigestSet>,
) -> DigestSet {
    dependencies.sort();
    dependencies.dedup();
    let mut frame = Frame::default();
    frame.field("format", KEY_FORMAT.as_bytes());
    frame.field("toolchain", environment.toolchain.as_bytes());
    frame.field("mathlib", environment.mathlib_revision.as_bytes());
    frame.field("declaration", canonical);
    frame.digest("group", group);
    frame.field("deps", dependencies.len().to_string().as_bytes());
    for dependency in dependencies.iter() {
        frame.digest("dep", dependency);
    }
    DigestSet::of(&frame.0)
}

/// The key of a unit: its granularity and the keys it covers, sorted.
#[must_use]
pub fn unit_key(granularity: Granularity, members: &[DigestSet]) -> DigestSet {
    let mut sorted = members.to_vec();
    sorted.sort();
    sorted.dedup();
    let mut frame = Frame::default();
    frame.field("format", UNIT_FORMAT.as_bytes());
    frame.field("granularity", granularity.as_str().as_bytes());
    frame.field("members", sorted.len().to_string().as_bytes());
    for member in &sorted {
        frame.digest("member", member);
    }
    DigestSet::of(&frame.0)
}

fn refused(message: String) -> Diagnostic {
    Diagnostic::new(code!("LLV7022"), message)
}

/// The keys of a set of declarations.
#[derive(Debug, Clone)]
pub struct ReplayGraph {
    keys: BTreeMap<String, DigestSet>,
    groups: BTreeMap<String, DigestSet>,
}

impl ReplayGraph {
    /// Key every declaration of `facts`. A name the facts do not declare must
    /// be in `anchors`, the keys of declarations an earlier run keyed (the
    /// imported modules); one that is neither is `LLV7022`, as is a name that
    /// is declared twice or both declared and anchored.
    pub fn build(
        environment: &ReplayEnvironment,
        facts: &[DeclarationFacts],
        anchors: &BTreeMap<String, DigestSet>,
    ) -> Result<Self, Diagnostic> {
        let mut index: BTreeMap<&str, usize> = BTreeMap::new();
        for (position, declaration) in facts.iter().enumerate() {
            if index.insert(declaration.name.as_str(), position).is_some() {
                return Err(refused(format!(
                    "declaration `{}` is stated twice",
                    declaration.name
                )));
            }
            if anchors.contains_key(&declaration.name) {
                return Err(refused(format!(
                    "declaration `{}` is both stated and anchored",
                    declaration.name
                )));
            }
        }
        let mut edges: Vec<Vec<usize>> = Vec::with_capacity(facts.len());
        for declaration in facts {
            let mut targets = Vec::new();
            for name in &declaration.deps {
                if let Some(&target) = index.get(name.as_str()) {
                    targets.push(target);
                } else if !anchors.contains_key(name) {
                    return Err(refused(format!(
                        "declaration `{}` names `{name}`, which the facts neither state nor anchor",
                        declaration.name
                    )));
                }
            }
            edges.push(targets);
        }

        let mut keys: BTreeMap<String, DigestSet> = BTreeMap::new();
        let mut groups: BTreeMap<String, DigestSet> = BTreeMap::new();
        // Components come dependencies first, so every outside key a member
        // needs exists when its component is reached.
        for component in components(&edges) {
            let mut members = component;
            members.sort_by(|a, b| facts[*a].name.cmp(&facts[*b].name));
            let canonical: Vec<Vec<u8>> = members
                .iter()
                .map(|member| canonical_declaration(&facts[*member]))
                .collect();
            let group = group_digest(&canonical);
            for (slot, &member) in members.iter().enumerate() {
                let mut dependencies = Vec::new();
                for name in &facts[member].deps {
                    if let Some(&target) = index.get(name.as_str()) {
                        if !members.contains(&target) {
                            dependencies.push(keys[&facts[target].name]);
                        }
                    } else {
                        dependencies.push(anchors[name]);
                    }
                }
                let key = declaration_key(environment, &canonical[slot], &group, &mut dependencies);
                keys.insert(facts[member].name.clone(), key);
                groups.insert(facts[member].name.clone(), group);
            }
        }
        Ok(Self { keys, groups })
    }

    /// The key of a declaration.
    #[must_use]
    pub fn key(&self, name: &str) -> Option<&DigestSet> {
        self.keys.get(name)
    }

    /// The group digest of a declaration.
    #[must_use]
    pub fn group(&self, name: &str) -> Option<&DigestSet> {
        self.groups.get(name)
    }

    /// Every key, by declaration name.
    #[must_use]
    pub const fn keys(&self) -> &BTreeMap<String, DigestSet> {
        &self.keys
    }

    /// The key of the unit of the named declarations.
    pub fn unit<'a>(
        &self,
        granularity: Granularity,
        names: impl IntoIterator<Item = &'a str>,
    ) -> Result<DigestSet, Diagnostic> {
        let mut members = Vec::new();
        for name in names {
            members.push(*self.key(name).ok_or_else(|| {
                refused(format!("unit names `{name}`, which the facts do not state"))
            })?);
        }
        Ok(unit_key(granularity, &members))
    }
}

/// Strongly connected components of a directed graph, each dependency's
/// component before the components that reach it (Tarjan, with an explicit
/// stack: a chain of lemmas thousands deep must not be the process's stack).
fn components(edges: &[Vec<usize>]) -> Vec<Vec<usize>> {
    const UNSEEN: usize = usize::MAX;
    let count = edges.len();
    let mut order = vec![UNSEEN; count];
    let mut low = vec![0usize; count];
    let mut on_stack = vec![false; count];
    let mut stack: Vec<usize> = Vec::new();
    let mut next = 0usize;
    let mut out: Vec<Vec<usize>> = Vec::new();
    for root in 0..count {
        if order[root] != UNSEEN {
            continue;
        }
        let mut call: Vec<(usize, usize)> = vec![(root, 0)];
        order[root] = next;
        low[root] = next;
        next += 1;
        stack.push(root);
        on_stack[root] = true;
        while let Some(&(vertex, cursor)) = call.last() {
            if let Some(&target) = edges[vertex].get(cursor) {
                call.last_mut().expect("a frame").1 += 1;
                if order[target] == UNSEEN {
                    order[target] = next;
                    low[target] = next;
                    next += 1;
                    stack.push(target);
                    on_stack[target] = true;
                    call.push((target, 0));
                } else if on_stack[target] {
                    low[vertex] = low[vertex].min(order[target]);
                }
            } else {
                call.pop();
                if let Some(&(parent, _)) = call.last() {
                    low[parent] = low[parent].min(low[vertex]);
                }
                if low[vertex] == order[vertex] {
                    let mut component = Vec::new();
                    loop {
                        let member = stack.pop().expect("a component member");
                        on_stack[member] = false;
                        component.push(member);
                        if member == vertex {
                            break;
                        }
                    }
                    out.push(component);
                }
            }
        }
    }
    out
}
