//! From an extraction to a package (SPEC.md §13.12): which declarations get
//! entries, what the entries say, and why the others do not.
//!
//! The planner is pure: the same extraction and package identity give the same
//! bytes. It never weakens a signature to fit; a declaration it cannot express
//! is refused with the registered code of its reason, and so is everything
//! that mentions it.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use serde::Serialize;

use crate::artifact::content_id::Sha256Digest;
use crate::code;
use crate::diagnostic::{Diagnostic, DiagnosticCode};
use crate::glossary::extract::{Decl, Elim, Extraction};
use crate::glossary::naming;
use crate::lexicon::entry::{is_lean_name, parse_entry, EntryContext};
use crate::lexicon::lse::{self, is_lse_identifier, ConstInfo, QualifiedId};
use crate::lexicon::package::entry_path;

/// The package a plan produces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageSpec {
    /// The package ID.
    pub package: String,
    /// The exact version.
    pub version: String,
    /// The language the package is written for.
    pub language: String,
}

/// The registered code of a refusal reason (SPEC.md §13.12, §26.3). A
/// declaration that mentions a refused, excluded, or absent one is `LLT4106`
/// whatever the root was.
#[must_use]
pub fn code_of(reason: &str) -> DiagnosticCode {
    match reason {
        "sort-polymorphic" => code!("LLT4101"),
        "strict-implicit-binder" => code!("LLT4102"),
        "string-literal"
        | "primitive-projection"
        | "unsupported-expression"
        | "internal"
        | "level-metavariable"
        | "level-parameter" => code!("LLT4103"),
        "core-level-mismatch" => code!("LLT4104"),
        "name-not-conservative" => code!("LLT4105"),
        "entry-check" => code!("LLT4107"),
        _ => code!("LLT4106"),
    }
}

/// One refused declaration of the report.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Refused {
    /// The Lean name.
    pub name: String,
    /// Did the request name it?
    pub requested: bool,
    /// The reason, or `dependency` for a declaration refused because of one
    /// it mentions.
    pub reason: String,
    /// The registered code.
    pub code: String,
    /// The detail.
    pub detail: String,
    /// For a dependency refusal, the declaration whose own refusal is the
    /// root.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<String>,
    /// For a dependency refusal, the reason of that root.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_reason: Option<String>,
}

/// Counts under one reason.
#[derive(Debug, Clone, Serialize, PartialEq, Eq, Default)]
pub struct ReasonCount {
    /// The registered code.
    pub code: String,
    /// How many requested declarations.
    pub count: u64,
}

/// Eliminator outcomes over the translated inductive types.
#[derive(Debug, Clone, Serialize, PartialEq, Eq, Default)]
pub struct EliminatorCounts {
    /// Entries that carry a descriptor.
    pub emitted: u64,
    /// Entries without one, by reason.
    pub omitted: BTreeMap<String, u64>,
}

/// The coverage report (`lexlean/glossary-report/1`). It holds counts and
/// names only: no time, path, or host, so it is part of the deterministic
/// output.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Report {
    /// The schema tag.
    pub spec: String,
    /// The package ID.
    pub package: String,
    /// The package version.
    pub version: String,
    /// The language.
    pub language: String,
    /// Requested declarations: `translated + refused + excluded`.
    pub requested: u64,
    /// Requested declarations that have an entry.
    pub translated: u64,
    /// Requested declarations that were refused.
    pub refused: u64,
    /// Requested declarations set aside by kind, not refused.
    pub excluded: u64,
    /// The excluded count by kind.
    pub excluded_by_kind: BTreeMap<String, u64>,
    /// Requested declarations refused for a reason of their own.
    pub refused_by_reason: BTreeMap<String, ReasonCount>,
    /// Requested declarations refused because they mention a refused,
    /// excluded, or absent one, by the reason of the root.
    pub refused_by_dependency: BTreeMap<String, u64>,
    /// Entries that are not requested declarations: the types, classes,
    /// functions, and constructors the requested signatures mention.
    pub support_entries: u64,
    /// Eliminator descriptors.
    pub eliminators: EliminatorCounts,
    /// Entries whose type had elaboration annotations erased (`outParam`,
    /// `optParam`, `autoParam`, `semiOutParam`).
    pub entries_with_erased_annotations: u64,
    /// Entries whose identifier needed a hash suffix.
    pub id_collisions: u64,
    /// Entries that share a canonical surface with another.
    pub surface_collisions: u64,
    /// Every refusal, by Lean name.
    pub refusals: Vec<Refused>,
}

impl Report {
    /// The report as its file bytes: pretty JSON, sorted keys, one final LF.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        // Serializing counts, strings, and sorted maps cannot fail.
        let mut bytes = serde_json::to_vec_pretty(self).unwrap_or_default();
        bytes.push(b'\n');
        bytes
    }

    /// A short human summary.
    #[must_use]
    pub fn summary(&self) -> String {
        let mut text = format!(
            "{}@{}: {} requested, {} translated, {} refused, {} excluded; {} support entries\n",
            self.package,
            self.version,
            self.requested,
            self.translated,
            self.refused,
            self.excluded,
            self.support_entries
        );
        for (reason, count) in &self.refused_by_reason {
            text.push_str(&format!(
                "  refused {} {}: {}\n",
                count.code, reason, count.count
            ));
        }
        for (root, count) in &self.refused_by_dependency {
            text.push_str(&format!(
                "  refused LLT4106 dependency on {root}: {count}\n"
            ));
        }
        for (kind, count) in &self.excluded_by_kind {
            text.push_str(&format!("  excluded {kind}: {count}\n"));
        }
        text
    }
}

/// A plan: the package files and the report.
#[derive(Debug, Clone)]
pub struct Plan {
    /// `(package-relative path, bytes)`, sorted bytewise, as the package
    /// loader takes them.
    pub files: Vec<(String, Vec<u8>)>,
    /// The coverage report.
    pub report: Report,
}

/// Why a declaration has no entry.
#[derive(Debug, Clone)]
struct Why {
    reason: String,
    detail: String,
    /// The declaration at the end of the chain, when the refusal is inherited.
    inherited_from: Option<(String, String)>,
}

fn toml_str(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            c if u32::from(c) < 0x20 || u32::from(c) == 0x7f => {
                out.push_str(&format!("\\u{:04X}", u32::from(c)));
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn toml_list(items: &[String]) -> String {
    let rendered: Vec<String> = items.iter().map(|item| toml_str(item)).collect();
    format!("[{}]", rendered.join(", "))
}

/// The category, frame, and forms of an entry (SPEC.md §13.12).
struct Shape {
    category: &'static str,
    frame: &'static str,
    /// `(form id, channel, surface, features)`.
    form: (String, &'static str, String, Vec<String>),
    /// The math render, when the frame has one.
    render: Option<String>,
}

fn shape(decl: &Decl, id: &str, arity: usize) -> Shape {
    let class = decl.class.as_deref().unwrap_or("term");
    let surface = naming::camel_surface(&decl.name);
    let last = id.rsplit('.').next().unwrap_or(id).to_owned();
    let call_render = |arity: usize| -> String {
        match arity {
            0 => format!("(operator-name {surface})"),
            1 => format!("(seq (operator-name {surface}) (paren (slot 0)))"),
            n => {
                let slots: Vec<String> = (0..n).map(|i| format!("(slot {i})")).collect();
                format!(
                    "(seq (operator-name {surface}) (paren (seq {})))",
                    slots.join(" (token comma) (space) ")
                )
            }
        }
    };
    let math = |category: &'static str, frame: &'static str| Shape {
        category,
        frame,
        form: (last.clone(), "math", surface.clone(), Vec::new()),
        render: Some(call_render(arity)),
    };
    let prose = |category: &'static str, frame: &'static str| {
        naming::prose_words(&decl.name).map(|words| Shape {
            category,
            frame,
            form: (last.clone(), "text", words, vec!["lower-case".to_owned()]),
            render: None,
        })
    };
    match (class, arity) {
        ("proof", 0) => math("proof-constant", "atom"),
        ("proof", _) => math("proof-constant", "call"),
        ("type", 0) => math("type-noun", "atom"),
        ("type", _) => math("function", "call"),
        ("predicate", 0) => math("predicate-constant", "atom"),
        ("predicate", 1) => {
            prose("adjective-predicate", "adjective").unwrap_or_else(|| math("function", "call"))
        }
        ("predicate", 2) => {
            prose("transitive-predicate", "transitive").unwrap_or_else(|| math("function", "call"))
        }
        ("predicate", _) => math("function", "call"),
        (_, 0) => math("term-constant", "atom"),
        _ => math("function", "call"),
    }
}

/// Everything the text of one entry depends on.
struct Rendered<'a> {
    decl: &'a Decl,
    id: &'a str,
    signature: &'a str,
    arity: usize,
    eliminator: Option<&'a str>,
}

fn entry_text(r: &Rendered<'_>) -> String {
    let shape = shape(r.decl, r.id, r.arity);
    let mut text = String::new();
    text.push_str("spec = \"lexlean/entry/1\"\n");
    text.push_str(&format!("id = {}\n", toml_str(r.id)));
    text.push_str(&format!("category = {}\n", toml_str(shape.category)));
    text.push_str(&format!("signature = {}\n", toml_str(r.signature)));
    if !r.decl.universes.is_empty() {
        text.push_str(&format!("universes = {}\n", toml_list(&r.decl.universes)));
    }
    text.push_str(&format!("surface_arity = {}\n", r.arity));
    text.push_str(&format!("frame = {}\n\n", toml_str(shape.frame)));
    text.push_str("[denotation]\nkind = \"lean\"\n");
    text.push_str(&format!("module = {}\n", toml_str(&r.decl.module)));
    text.push_str(&format!("name = {}\n\n", toml_str(&r.decl.name)));
    let (form_id, channel, surface, features) = &shape.form;
    text.push_str("[[form]]\n");
    text.push_str(&format!("id = {}\n", toml_str(form_id)));
    text.push_str(&format!("channel = {}\n", toml_str(channel)));
    text.push_str(&format!("surface = {}\n", toml_str(surface)));
    text.push_str("canonical_source = true\n");
    text.push_str(&format!("features = {}\n", toml_list(features)));
    if let Some(render) = &shape.render {
        text.push_str(&format!("\n[render]\nmath = {}\n", toml_str(render)));
    }
    if let Some(eliminator) = r.eliminator {
        text.push('\n');
        text.push_str(eliminator);
    }
    text
}

fn eliminator_text(elim: &Elim, ids: &BTreeMap<&str, String>, package: &str) -> Option<String> {
    if !is_lean_name(&elim.cases) || !is_lean_name(&elim.induction) {
        return None;
    }
    let mut text = format!(
        "[eliminator]\ncases_lean_name = {}\ninduction_lean_name = {}\n",
        toml_str(&elim.cases),
        toml_str(&elim.induction)
    );
    for constructor in &elim.constructors {
        let id = ids.get(constructor.name.as_str())?;
        let mut seen = BTreeSet::new();
        for binder in constructor
            .fields
            .iter()
            .chain(&constructor.induction_hypotheses)
        {
            if !is_lse_identifier(binder) || !seen.insert(binder.as_str()) {
                return None;
            }
        }
        text.push_str(&format!(
            "\n[[eliminator.constructor]]\nentry = {}\nlean_name = {}\nfields = {}\ninduction_hypotheses = {}\n",
            toml_str(&format!("{package}::{id}")),
            toml_str(&constructor.name),
            toml_list(&constructor.fields),
            toml_list(&constructor.induction_hypotheses),
        ));
    }
    Some(text)
}

/// Replace every `(const @Lean.Name` placeholder with the qualified ID of the
/// name's entry. `Err` names a placeholder with no entry.
fn resolve(signature: &str, ids: &BTreeMap<&str, String>, package: &str) -> Result<String, String> {
    let mut out = String::with_capacity(signature.len());
    let mut rest = signature;
    while let Some(at) = rest.find("(const @") {
        out.push_str(&rest[..at + "(const ".len()]);
        let after = &rest[at + "(const @".len()..];
        let end = after.find([' ', ')']).unwrap_or(after.len());
        let name = &after[..end];
        let id = ids.get(name).ok_or_else(|| name.to_owned())?;
        out.push_str(package);
        out.push_str("::");
        out.push_str(id);
        rest = &after[end..];
    }
    out.push_str(rest);
    Ok(out)
}

/// Mark `start` and everything that mentions it refused with `why`, skipping
/// declarations that are already refused.
fn propagate<'a>(
    start: &'a str,
    root_reason: &str,
    rdeps: &BTreeMap<&'a str, Vec<&'a str>>,
    refused: &mut BTreeMap<&'a str, Why>,
    root_detail: &str,
) {
    let mut queue: VecDeque<&str> = VecDeque::from([start]);
    while let Some(current) = queue.pop_front() {
        let Some(dependents) = rdeps.get(current) else {
            continue;
        };
        for &dependent in dependents {
            if refused.contains_key(dependent) {
                continue;
            }
            let (root_name, reason) = match refused.get(current) {
                Some(Why {
                    inherited_from: Some((name, reason)),
                    ..
                }) => (name.clone(), reason.clone()),
                _ => (current.to_owned(), root_reason.to_owned()),
            };
            refused.insert(
                dependent,
                Why {
                    reason: "dependency".to_owned(),
                    detail: format!("mentions `{current}`: {root_detail}"),
                    inherited_from: Some((root_name, reason)),
                },
            );
            queue.push_back(dependent);
        }
    }
}

/// Plan the package for an extraction.
///
/// # Errors
///
/// `LLI9001` when the generated package fails the package loader, which means
/// the planner emitted what its own checks accepted and the loader did not.
#[allow(clippy::too_many_lines)]
pub fn plan(extraction: &Extraction, spec: &PackageSpec) -> Result<Plan, Vec<Diagnostic>> {
    let by_name: BTreeMap<&str, &Decl> = extraction
        .declarations
        .iter()
        .map(|decl| (decl.name.as_str(), decl))
        .collect();

    // 1. Declarations with a reason of their own, and the reverse mention
    //    graph.
    let mut refused: BTreeMap<&str, Why> = BTreeMap::new();
    let mut rdeps: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for decl in &extraction.declarations {
        if let Some(refusal) = &decl.refusal {
            refused.insert(
                decl.name.as_str(),
                Why {
                    reason: refusal.reason.clone(),
                    detail: refusal.detail.clone(),
                    inherited_from: None,
                },
            );
        } else if decl.signature.is_none() {
            refused.insert(
                decl.name.as_str(),
                Why {
                    reason: "internal".to_owned(),
                    detail: "the extraction has neither a signature nor a refusal".to_owned(),
                    inherited_from: None,
                },
            );
        }
        for reference in &decl.refs {
            rdeps
                .entry(reference.as_str())
                .or_default()
                .push(decl.name.as_str());
        }
    }
    // A mention of a name the extraction does not hold is a mention of an
    // absent declaration.
    let absent: Vec<&str> = rdeps
        .keys()
        .copied()
        .filter(|name| !by_name.contains_key(name))
        .collect();
    for name in absent {
        for &dependent in &rdeps[name] {
            refused.entry(dependent).or_insert_with(|| Why {
                reason: "dependency".to_owned(),
                detail: format!("mentions `{name}`, which the extraction does not hold"),
                inherited_from: Some((name.to_owned(), "unknown-declaration".to_owned())),
            });
        }
    }
    // 2. Propagate along mentions, from every declaration refused so far.
    let starts: Vec<(&str, String, String)> = refused
        .iter()
        .map(|(name, why)| (*name, why.reason.clone(), why.detail.clone()))
        .collect();
    for (name, reason, detail) in &starts {
        propagate(name, reason, &rdeps, &mut refused, detail);
    }

    // 3. Identifiers for what is left.
    let mut groups: BTreeMap<String, Vec<&str>> = BTreeMap::new();
    for decl in &extraction.declarations {
        if !refused.contains_key(decl.name.as_str()) {
            groups
                .entry(naming::base_entry_id(&decl.name))
                .or_default()
                .push(decl.name.as_str());
        }
    }
    let mut ids: BTreeMap<&str, String> = BTreeMap::new();
    let mut taken: BTreeSet<String> = groups
        .iter()
        .filter(|(_, names)| names.len() == 1)
        .map(|(base, _)| base.clone())
        .collect();
    let mut id_collisions = 0u64;
    for (base, names) in &groups {
        if let [only] = names.as_slice() {
            ids.insert(only, base.clone());
            continue;
        }
        for &name in names {
            id_collisions += 1;
            let digest = Sha256Digest::of(name.as_bytes()).to_hex();
            let mut length = 6;
            let mut candidate = format!("{base}-{}", &digest[..length]);
            while taken.contains(&candidate) && length < digest.len() {
                length += 2;
                candidate = format!("{base}-{}", &digest[..length]);
            }
            taken.insert(candidate.clone());
            ids.insert(name, candidate);
        }
    }

    // 4. Texts, entry validation, and the signature check.
    let bootstrap = crate::lexicon::load_bootstrap_for(&spec.language).map_err(|d| vec![d])?;
    let context = EntryContext {
        is_core: false,
        forbidden_controls: &bootstrap.structural.forbidden_controls,
        max_scope_depth: 1024,
    };
    let mut resolved: BTreeMap<&str, (String, usize)> = BTreeMap::new();
    let mut parsed: BTreeMap<String, crate::lexicon::entry::Entry> = BTreeMap::new();
    let mut failed: Vec<(&str, String)> = Vec::new();
    for decl in &extraction.declarations {
        let name = decl.name.as_str();
        if refused.contains_key(name) {
            continue;
        }
        let id = &ids[name];
        let signature = match resolve(decl.signature.as_deref().unwrap_or(""), &ids, &spec.package)
        {
            Ok(signature) => signature,
            Err(missing) => {
                failed.push((name, format!("the placeholder `{missing}` has no entry")));
                continue;
            }
        };
        let arity = match lse::parse(&signature, 1024) {
            Ok(lse) => lse.outer_explicit_binders(),
            Err(error) => {
                failed.push((name, format!("the signature does not parse: {error}")));
                continue;
            }
        };
        let text = entry_text(&Rendered {
            decl,
            id,
            signature: &signature,
            arity,
            eliminator: None,
        });
        match parse_entry(&entry_path(id), &text, &context) {
            Ok(entry) => {
                parsed.insert(id.clone(), entry);
                resolved.insert(name, (signature, arity));
            }
            Err(diagnostics) => failed.push((
                name,
                diagnostics
                    .first()
                    .map_or_else(String::new, |d| d.message.clone()),
            )),
        }
    }
    {
        let lookup = |id: &QualifiedId| -> ConstInfo<'_> {
            if id.package != spec.package {
                return ConstInfo::Opaque;
            }
            match parsed.get(&id.entry).and_then(|e| e.signature.as_ref()) {
                Some(signature) => ConstInfo::Signature {
                    signature,
                    defined: false,
                },
                None => ConstInfo::Missing,
            }
        };
        for (&name, (signature, _)) in &resolved {
            let Ok(parsed_signature) = lse::parse(signature, 1024) else {
                continue;
            };
            if let Err(error) = lse::check_signature(&parsed_signature, &lookup) {
                failed.push((name, format!("the signature is not well-typed: {error}")));
            }
        }
    }
    for (name, detail) in &failed {
        refused.entry(name).or_insert_with(|| Why {
            reason: "entry-check".to_owned(),
            detail: detail.clone(),
            inherited_from: None,
        });
    }
    for (name, detail) in &failed {
        propagate(name, "entry-check", &rdeps, &mut refused, detail);
    }

    // 5. Final texts, with eliminators over the constructors that survived.
    let mut files: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    let mut eliminators = EliminatorCounts::default();
    let mut erased_entries = 0u64;
    let mut surfaces: BTreeMap<String, u64> = BTreeMap::new();
    let mut translated_requested = 0u64;
    let mut support = 0u64;
    for decl in &extraction.declarations {
        let name = decl.name.as_str();
        if refused.contains_key(name) {
            continue;
        }
        let Some((signature, arity)) = resolved.get(name) else {
            continue;
        };
        let id = &ids[name];
        let mut eliminator: Option<String> = None;
        if let Some(elim) = &decl.eliminator {
            let all_present = elim.constructors.iter().all(|c| {
                !refused.contains_key(c.name.as_str()) && ids.contains_key(c.name.as_str())
            });
            let candidate = if all_present {
                eliminator_text(elim, &ids, &spec.package)
            } else {
                None
            };
            let reason = match &candidate {
                Some(_) => None,
                None if all_present => Some("invalid-binder-or-name"),
                None => Some("constructor-refused"),
            };
            if let Some(text) = candidate {
                let with = entry_text(&Rendered {
                    decl,
                    id,
                    signature,
                    arity: *arity,
                    eliminator: Some(&text),
                });
                if parse_entry(&entry_path(id), &with, &context).is_ok() {
                    eliminator = Some(text);
                    eliminators.emitted += 1;
                } else {
                    *eliminators
                        .omitted
                        .entry("descriptor-rejected".to_owned())
                        .or_default() += 1;
                }
            } else if let Some(reason) = reason {
                *eliminators.omitted.entry(reason.to_owned()).or_default() += 1;
            }
        } else if let Some(why) = &decl.eliminator_omitted {
            *eliminators.omitted.entry(why.clone()).or_default() += 1;
        }
        let text = entry_text(&Rendered {
            decl,
            id,
            signature,
            arity: *arity,
            eliminator: eliminator.as_deref(),
        });
        if decl.erased > 0 {
            erased_entries += 1;
        }
        *surfaces
            .entry(naming::camel_surface(&decl.name))
            .or_default() += 1;
        if decl.requested {
            translated_requested += 1;
        } else {
            support += 1;
        }
        files.insert(entry_path(id), text.into_bytes());
    }
    let surface_collisions = surfaces.values().filter(|count| **count > 1).sum();

    // 6. The manifest, and the whole package through the package loader.
    let manifest = format!(
        "spec = \"lexlean/lexicon/1\"\npackage = {}\nversion = {}\nlanguage = {}\nimports = [{}]\n",
        toml_str(&spec.package),
        toml_str(&spec.version),
        toml_str(&spec.language),
        toml_str(&format!("lexlean.core@{}.0", spec.language)),
    );
    files.insert("lexicon.toml".to_owned(), manifest.into_bytes());
    let file_list: Vec<(String, Vec<u8>)> = files.into_iter().collect();
    let load_context = crate::lexicon::package::LoadContext {
        language: &spec.language,
        forbidden_controls: &bootstrap.structural.forbidden_controls,
        max_scope_depth: 1024,
    };
    if let Err(diagnostics) =
        crate::lexicon::package::load_package(&spec.package, &file_list, None, &load_context)
    {
        let first = diagnostics
            .first()
            .map_or_else(String::new, |d| d.message.clone());
        return Err(vec![Diagnostic::new(
            code!("LLI9001"),
            format!("the generated package is rejected by the package loader: {first}"),
        )]);
    }

    // 7. The report.
    let mut report = Report {
        spec: "lexlean/glossary-report/1".to_owned(),
        package: spec.package.clone(),
        version: spec.version.clone(),
        language: spec.language.clone(),
        requested: 0,
        translated: translated_requested,
        refused: 0,
        excluded: extraction.excluded.values().sum(),
        excluded_by_kind: extraction.excluded.clone(),
        refused_by_reason: BTreeMap::new(),
        refused_by_dependency: BTreeMap::new(),
        support_entries: support,
        eliminators,
        entries_with_erased_annotations: erased_entries,
        id_collisions,
        surface_collisions,
        refusals: Vec::new(),
    };
    let mut refusals: Vec<Refused> = Vec::new();
    for decl in &extraction.declarations {
        let Some(why) = refused.get(decl.name.as_str()) else {
            continue;
        };
        let (root, root_reason) = match &why.inherited_from {
            Some((root, reason)) => (Some(root.clone()), Some(reason.clone())),
            None => (None, None),
        };
        if decl.requested {
            report.refused += 1;
            match &root_reason {
                Some(root_reason) => {
                    *report
                        .refused_by_dependency
                        .entry(root_reason.clone())
                        .or_default() += 1;
                }
                None => {
                    let entry = report
                        .refused_by_reason
                        .entry(why.reason.clone())
                        .or_insert_with(|| ReasonCount {
                            code: code_of(&why.reason).as_str().to_owned(),
                            count: 0,
                        });
                    entry.count += 1;
                }
            }
        }
        refusals.push(Refused {
            name: decl.name.clone(),
            requested: decl.requested,
            reason: why.reason.clone(),
            code: code_of(&why.reason).as_str().to_owned(),
            detail: why.detail.clone(),
            root,
            root_reason,
        });
    }
    refusals.sort_by(|a, b| a.name.cmp(&b.name));
    report.refusals = refusals;
    report.requested = report.translated + report.refused + report.excluded;
    Ok(Plan {
        files: file_list,
        report,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decl(name: &str, signature: &str, class: &str) -> Decl {
        Decl {
            name: name.to_owned(),
            module: "Init.Test".to_owned(),
            kind: "theorem".to_owned(),
            requested: true,
            signature: Some(signature.to_owned()),
            class: Some(class.to_owned()),
            universes: Vec::new(),
            erased: 0,
            refs: Vec::new(),
            refusal: None,
            eliminator: None,
            eliminator_omitted: None,
        }
    }

    fn spec() -> PackageSpec {
        PackageSpec {
            package: "test.pkg".to_owned(),
            version: "1.0.0".to_owned(),
            language: "1.0".to_owned(),
        }
    }

    #[test]
    fn a_mention_of_a_refused_declaration_is_refused_with_the_root() {
        let mut bad = decl("Bad", "(sort prop)", "predicate");
        bad.signature = None;
        bad.refusal = Some(crate::glossary::extract::Refusal {
            reason: "sort-polymorphic".to_owned(),
            detail: "Sort u".to_owned(),
        });
        let mut user = decl("User", "(app (const @Bad))", "proof");
        user.refs = vec!["Bad".to_owned()];
        let extraction = Extraction {
            declarations: vec![bad, user],
            excluded: BTreeMap::new(),
        };
        let plan = plan(&extraction, &spec()).expect("plans");
        assert_eq!(plan.report.translated, 0);
        assert_eq!(plan.report.refused, 2);
        assert_eq!(
            plan.report.refused_by_reason["sort-polymorphic"].code,
            "LLT4101"
        );
        assert_eq!(plan.report.refused_by_dependency["sort-polymorphic"], 1);
        let user = plan
            .report
            .refusals
            .iter()
            .find(|r| r.name == "User")
            .unwrap();
        assert_eq!(user.code, "LLT4106");
        assert_eq!(user.root.as_deref(), Some("Bad"));
        assert_eq!(plan.report.requested, 2);
    }
}
