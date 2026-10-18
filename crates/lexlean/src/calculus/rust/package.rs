//! Rust packages (SPEC.md §17.16): a target program rendered in a profile,
//! with exported functions, a Cargo manifest, and provenance binding the
//! bytes to the program, the runtime, and LexLean's compiler semantics.
//!
//! A package is described by a closed manifest (`lexlean/rust-package/1`).
//! Every exported name, parameter passing mode, and declared failure mode is
//! checked against the program before anything is rendered, so a package
//! whose interface misstates the program it realizes is refused, never
//! repaired.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::super::{Expr as Term, Program, Ty};
use super::ast::{self, Block, Callee, Expr, Ident, ItemDef, Lit, Origin, Pat, Type};
use super::lower::{index, Lowering};
use super::runtime::{self, Item};
use super::{elements, validate, Profile};
use crate::artifact::content_id::Sha256Digest;

/// The schema tag of a package manifest.
pub const MANIFEST_SPEC: &str = "lexlean/rust-package/1";

/// Where a package keeps its library: the path its `Cargo.toml` names and
/// the file the renderer writes, one fact of the package's layout and not a
/// free parameter of the scaffold (a path that differed would be a crate
/// that does not build).
pub const LIBRARY_PATH: &str = "src/lib.rs";

/// The schema tag of a package's provenance.
pub const PROVENANCE_SPEC: &str = "lexlean/rust-provenance/1";

/// How an exported function takes one parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Passing {
    /// By value: the caller gives up the value.
    Own,
    /// By shared reference: the function takes its own copy of the handle.
    Borrow,
    /// By copy: only for a type that is `Copy`.
    Copy,
}

/// Whether an exported function can fail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Errors {
    /// It cannot fail and returns its value.
    None,
    /// It can overflow and returns `R<T>`.
    Overflow,
}

/// One exported function.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Export {
    /// The program function it exports.
    pub function: u64,
    /// Its Rust name.
    pub name: String,
    /// How it takes each parameter.
    pub parameters: Vec<Passing>,
    /// Whether it can fail.
    pub errors: Errors,
}

/// A package manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub spec: String,
    /// The crate name.
    pub name: String,
    /// The crate version.
    pub version: String,
    /// The machine profile: `rust-core` or `rust-std`.
    pub profile: String,
    /// The target program the package realizes.
    pub program: Program,
    /// The exported functions.
    pub exports: Vec<Export>,
    /// The semantic IDs (§21.4) of the LexLean builds whose linked IR states
    /// the program, as lowercase SHA-256 hex, strictly ascending; empty for
    /// a program no LexLean build states. The renderer checks their form
    /// and records them; that they name the build stating the program is
    /// checked against the verified build (§17.16).
    pub sources: Vec<String>,
    /// The effects (§17.13) the package states of itself, strictly ascending:
    /// for a package a production root exports, the set eligibility requires
    /// of the root on the profile; for a fixture, those its program shows
    /// ([`shown_effects`]), never `allocation`. They are not the effects the
    /// realization can exhibit (issue 105): an entry's argument check recurses
    /// over a recursive argument type and the graph templates' `nat_add` fuel
    /// bound can overflow, and neither is required of a root. A host that runs
    /// a package whose effects include `recursion` gives it a stack its
    /// input-dependent depth fits. The renderer checks the form (registered,
    /// ascending) and that `rust-core` states no `allocation`; it does not
    /// derive the list.
    pub effects: Vec<String>,
}

/// A rendered package: its files by path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    pub files: BTreeMap<String, Vec<u8>>,
}

fn fail<T>(reason: impl Into<String>) -> Result<T, String> {
    Err(reason.into())
}

/// Every keyword of Rust 2021: strict, reserved, and weak.
const KEYWORDS: &[&str] = &[
    "as",
    "async",
    "await",
    "break",
    "const",
    "continue",
    "crate",
    "dyn",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "self",
    "Self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "try",
    "type",
    "unsafe",
    "use",
    "where",
    "while",
    "abstract",
    "become",
    "box",
    "do",
    "final",
    "macro",
    "override",
    "priv",
    "typeof",
    "unsized",
    "virtual",
    "yield",
    "union",
    "macro_rules",
    "gen",
    "raw",
    "safe",
];

/// Crate names a package may not take: the sysroot crates and the harness.
const RESERVED_CRATES: &[&str] = &["std", "core", "alloc", "proc_macro", "test", "harness"];

fn snake(name: &str) -> bool {
    let mut characters = name.chars();
    characters
        .next()
        .is_some_and(|first| first.is_ascii_lowercase())
        && characters.all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
        })
        && !name.contains("__")
        && !name.ends_with('_')
        && name.len() <= 64
}

/// Why an exported name would collide with a name the rendering or the
/// runtime declares, or would not be a plain Rust identifier.
fn collision(name: &str) -> Option<String> {
    if !snake(name) {
        return Some(format!(
            "`{name}` is not a lowercase snake-case Rust identifier of at most 64 characters"
        ));
    }
    if KEYWORDS.contains(&name) {
        return Some(format!("`{name}` is a Rust keyword"));
    }
    let mut characters = name.chars();
    if characters
        .next()
        .is_some_and(|first| ast::GENERATED_PREFIXES.contains(&first))
        && !name[1..].is_empty()
        && name[1..]
            .chars()
            .all(|character| character.is_ascii_digit())
    {
        return Some(format!("`{name}` has the shape of a generated name"));
    }
    if runtime::ROOT_NAMES.contains(&name) {
        return Some(format!("`{name}` is declared by the runtime"));
    }
    let runtime_paths: BTreeSet<String> = Item::all()
        .into_iter()
        .map(|item| {
            let path = item.path();
            path.split("::").next().unwrap_or(&path).to_owned()
        })
        .collect();
    if runtime_paths.contains(name) {
        return Some(format!("`{name}` is declared by the runtime"));
    }
    None
}

/// Whether a value of `ty` can hold a function value anywhere: directly, or
/// inside the fields of a named type, at any depth.
#[must_use]
pub fn holds_function(program: &Program, ty: &Ty) -> bool {
    fn walk(program: &Program, ty: &Ty, seen: &mut BTreeSet<u64>) -> bool {
        match ty {
            Ty::Fn { .. } => true,
            Ty::Option { value } | Ty::List { element: value } => walk(program, value, seen),
            Ty::Result {
                ok: left,
                error: right,
            }
            | Ty::Pair { left, right } => walk(program, left, seen) || walk(program, right, seen),
            Ty::Adt { index } => {
                seen.insert(*index)
                    && usize::try_from(*index)
                        .ok()
                        .and_then(|adt| program.adts.get(adt))
                        .is_some_and(|adt| {
                            adt.constructors
                                .iter()
                                .flatten()
                                .any(|field| walk(program, field, seen))
                        })
            }
            Ty::Unit
            | Ty::Bool
            | Ty::Nat
            | Ty::Int
            | Ty::Fixed { .. }
            | Ty::String
            | Ty::Bytes
            | Ty::Ordering => false,
        }
    }
    walk(program, ty, &mut BTreeSet::new())
}

/// The functions an expression calls or builds a closure of.
fn reaches(expr: &Term, out: &mut BTreeSet<u64>) {
    let children: Vec<&Term> = match expr {
        Term::Value { .. } | Term::Var { .. } => Vec::new(),
        Term::Let { bound, body, .. } => vec![bound, body],
        Term::Cond {
            condition,
            then_branch,
            else_branch,
        } => vec![condition, then_branch, else_branch],
        Term::Match {
            scrutinee, arms, ..
        } => std::iter::once(scrutinee.as_ref())
            .chain(arms.iter().map(|arm| &arm.body))
            .collect(),
        Term::Build { operands, .. } | Term::Prim { operands, .. } => operands.iter().collect(),
        Term::Call { function, operands } => {
            out.insert(*function);
            operands.iter().collect()
        }
        Term::Closure { function, captures } => {
            out.insert(*function);
            captures.iter().collect()
        }
        Term::Apply { target, operands } => {
            std::iter::once(target.as_ref()).chain(operands).collect()
        }
        Term::First { value } | Term::Second { value } | Term::Field { value, .. } => {
            vec![value]
        }
    };
    for child in children {
        reaches(child, out);
    }
}

/// Whether some function of `program` reaches itself through calls and
/// closures: the recursion a package shows of itself.
fn recurs(program: &Program) -> bool {
    let edges: Vec<BTreeSet<u64>> = program
        .functions
        .iter()
        .map(|function| {
            let mut out = BTreeSet::new();
            reaches(&function.body, &mut out);
            out
        })
        .collect();
    // Colors: 0 unseen, 1 on the path, 2 finished. The path is an explicit
    // stack, so a long chain of functions does not deepen the call stack.
    let mut color = vec![0u8; edges.len()];
    for root in 0..edges.len() {
        if color[root] != 0 {
            continue;
        }
        let mut path = vec![(root, edges[root].iter().copied().collect::<Vec<_>>())];
        color[root] = 1;
        while let Some((node, pending)) = path.last_mut() {
            let node = *node;
            match pending.pop() {
                None => {
                    color[node] = 2;
                    path.pop();
                }
                Some(next) => match usize::try_from(next).ok().filter(|at| *at < edges.len()) {
                    None => {}
                    Some(at) if color[at] == 1 => return true,
                    Some(at) if color[at] == 0 => {
                        color[at] = 1;
                        path.push((at, edges[at].iter().copied().collect()));
                    }
                    Some(_) => {}
                },
            }
        }
    }
    false
}

/// The effects a package's own program shows of itself, ascending: what a
/// package that no production root exports states of itself. It is not a
/// check on a root's effects, and finer than them: it sees the argument check
/// a root's entry carries recurse over a recursive type, and the graph
/// templates' `nat_add` fuel bound overflow, neither of which the root's
/// eligibility reports. The program is a canonical one.
#[must_use]
pub fn shown_effects(program: &Program, exports: &[Export]) -> Vec<String> {
    let mut out = BTreeSet::new();
    if exports
        .iter()
        .any(|export| export.errors == Errors::Overflow)
    {
        out.insert("overflow".to_owned());
    }
    if recurs(program) {
        out.insert("recursion".to_owned());
    }
    out.into_iter().collect()
}

impl Manifest {
    /// The manifest with `effects` those its program shows of itself
    /// ([`shown_effects`]), for a package no production root exports.
    #[must_use]
    pub fn showing_effects(mut self) -> Self {
        self.effects = shown_effects(&self.program, &self.exports);
        self
    }

    /// Read a manifest from JSON.
    ///
    /// # Errors
    ///
    /// Returns the reason the bytes are not a manifest.
    pub fn parse(bytes: &[u8]) -> Result<Self, String> {
        crate::artifact::canonical_json::Json::parse(bytes)
            .map_err(|error| format!("package manifest is malformed: {error}"))?;
        serde_json::from_slice(bytes)
            .map_err(|error| format!("package manifest is malformed: {error}"))
    }

    /// The canonical file bytes.
    ///
    /// # Errors
    ///
    /// Returns the reason the manifest has no canonical JSON form: an
    /// integer outside the range canonical JSON admits.
    pub fn to_file_bytes(&self) -> Result<Vec<u8>, String> {
        let text = serde_json::to_string(self)
            .map_err(|error| format!("package manifest does not serialize: {error}"))?;
        Ok(
            crate::artifact::canonical_json::Json::parse(text.as_bytes())
                .map_err(|error| format!("package manifest is not canonical JSON: {error}"))?
                .to_file_bytes(),
        )
    }
}

/// The export wrappers of `manifest`, checked against the program.
fn exports(manifest: &Manifest, lowering: &mut Lowering<'_>) -> Result<Vec<ItemDef>, String> {
    let program = lowering.program;
    if manifest.exports.is_empty() {
        return fail("a package exports at least one function");
    }
    let mut names = BTreeSet::new();
    let mut out = Vec::new();
    for export in &manifest.exports {
        let name = &export.name;
        if let Some(reason) = collision(name) {
            return fail(format!("identifier collision: export {reason}"));
        }
        if !names.insert(name.clone()) {
            return fail(format!("identifier collision: `{name}` is exported twice"));
        }
        let function = program
            .functions
            .get(index(export.function)?)
            .ok_or_else(|| {
                format!(
                    "export `{name}` names function {}, which is not declared",
                    export.function
                )
            })?;
        if export.parameters.len() != function.types.len() {
            return fail(format!(
                "export `{name}` passes {} parameters; function {} takes {}",
                export.parameters.len(),
                export.function,
                function.types.len()
            ));
        }
        if function
            .types
            .iter()
            .chain([&function.result])
            .any(|ty| holds_function(program, ty))
        {
            return fail(format!(
                "unsupported type: the boundary of export `{name}` holds a function value, and a package boundary is first-order (§17.13)"
            ));
        }
        let fallible = lowering
            .fallible
            .get(index(export.function)?)
            .copied()
            .unwrap_or(false);
        match (export.errors, fallible) {
            (Errors::None, true) => {
                return fail(format!(
                    "arithmetic mismatch: export `{name}` declares no errors, but function {} can overflow",
                    export.function
                ))
            }
            (Errors::Overflow, false) => {
                return fail(format!(
                    "arithmetic mismatch: export `{name}` declares overflow, but function {} cannot overflow",
                    export.function
                ))
            }
            _ => {}
        }
        let at = Origin::of("export");
        let mut parameters = Vec::new();
        let mut args = Vec::new();
        for (position, (passing, ty)) in export.parameters.iter().zip(&function.types).enumerate() {
            let lowered = lowering.ty(ty)?;
            let param = Ident::Param(position as u64);
            let (ty, arg) = match passing {
                Passing::Own => (lowered, Expr::Move(param.clone(), at.clone())),
                Passing::Borrow if lowered.copy() => (
                    Type::Ref(Box::new(lowered)),
                    Expr::Deref(param.clone(), at.clone()),
                ),
                Passing::Borrow => (
                    Type::Ref(Box::new(lowered)),
                    Expr::Clone(param.clone(), at.clone()),
                ),
                Passing::Copy if lowered.copy() => (lowered, Expr::Copy(param.clone(), at.clone())),
                Passing::Copy => {
                    return fail(format!(
                        "ownership mismatch: export `{name}` copies parameter {position}, whose type {ty:?} is not Copy"
                    ))
                }
            };
            // A unit carries nothing, so it is passed as the literal `()`.
            if ty == Type::Unit || ty == Type::Ref(Box::new(Type::Unit)) {
                parameters.push((Pat::Wild, ty));
                args.push(Expr::Lit(Lit::Unit, at.clone()));
            } else {
                parameters.push((Pat::Bind(param), ty));
                args.push(arg);
            }
        }
        let result = lowering.ty(&function.result)?;
        out.push(ItemDef::Function {
            name: Ident::Export(name.clone()),
            parameters,
            result: if fallible {
                Type::Fallible(Box::new(result))
            } else {
                result
            },
            body: Block::of(Expr::Call {
                callee: Callee::Function(export.function),
                args,
                propagate: false,
                at: at.clone(),
            }),
            at,
        });
    }
    Ok(out)
}

fn sha256(bytes: &[u8]) -> String {
    Sha256Digest::of(bytes).to_hex()
}

/// The Cargo manifest of a package, printed over the scaffold record
/// ([`super::scaffold`]): rustc's warnings and every Clippy lint of the
/// default set are denied, except the record's allowed lints. This is a
/// printer for Cargo's tables; every value it prints is the record's.
fn cargo_toml(manifest: &Manifest) -> Result<String, String> {
    Ok(cargo_toml_of(
        super::scaffold::get()?,
        &manifest.name,
        &manifest.version,
    ))
}

/// The Cargo manifest a package named `name` at `version` has under
/// `scaffold`.
#[must_use]
pub fn cargo_toml_of(scaffold: &super::scaffold::Scaffold, name: &str, version: &str) -> String {
    let gate = |gate: &super::scaffold::Gate| {
        if gate.priority == 0 {
            format!("{} = \"{}\"\n", gate.lint, gate.level.as_str())
        } else {
            format!(
                "{} = {{ level = \"{}\", priority = {} }}\n",
                gate.lint,
                gate.level.as_str(),
                gate.priority
            )
        }
    };
    let mut out = format!(
        "[package]\nname = \"{}\"\nversion = \"{}\"\nedition = \"{}\"\npublish = {}\n\n[lib]\npath = \"{}\"\n\n[lints.rust]\n",
        name, version, scaffold.edition, scaffold.publish, LIBRARY_PATH
    );
    for rustc in &scaffold.rustc {
        out.push_str(&gate(rustc));
    }
    out.push_str("\n[lints.clippy]\n");
    for clippy in &scaffold.clippy {
        out.push_str(&gate(clippy));
    }
    for row in &scaffold.allowed {
        out.push_str(&gate(&super::scaffold::Gate {
            lint: row.lint.clone(),
            level: super::scaffold::Level::Allow,
            priority: 0,
        }));
    }
    out
}

/// A package's checked crate, its canonical program, and its profile.
///
/// # Errors
///
/// As [`package`].
pub fn lower(manifest: &Manifest) -> Result<(ast::Crate, Program, Profile), String> {
    if manifest.spec != MANIFEST_SPEC {
        return fail(format!(
            "package manifest has spec `{}`, expected `{MANIFEST_SPEC}`",
            manifest.spec
        ));
    }
    if !snake(&manifest.name)
        || KEYWORDS.contains(&manifest.name.as_str())
        || RESERVED_CRATES.contains(&manifest.name.as_str())
    {
        return fail(format!(
            "identifier collision: `{}` is not an available crate name",
            manifest.name
        ));
    }
    // Cargo reads each part as a `u64`; a part is its canonical decimal.
    let version_parts: Vec<&str> = manifest.version.split('.').collect();
    if version_parts.len() != 3
        || version_parts.iter().any(|part| {
            !part.chars().all(|character| character.is_ascii_digit())
                || part
                    .parse::<u64>()
                    .map_or(true, |number| number.to_string() != *part)
        })
    {
        return fail(format!(
            "`{}` is not a version of three canonical decimals each at most {}",
            manifest.version,
            u64::MAX
        ));
    }
    let profile = Profile::named(&manifest.profile)
        .ok_or_else(|| format!("`{}` is not a Rust profile", manifest.profile))?;
    if manifest.sources.iter().any(|source| {
        source.len() != 64
            || !source
                .chars()
                .all(|character| character.is_ascii_digit() || ('a'..='f').contains(&character))
    }) {
        return fail("every source is a lowercase SHA-256 hex identity");
    }
    if manifest.sources.windows(2).any(|pair| pair[0] >= pair[1]) {
        return fail("sources are strictly ascending");
    }
    let registry = crate::production::registry()?;
    if let Some(unknown) = manifest
        .effects
        .iter()
        .find(|effect| !registry.effects.contains_key(effect.as_str()))
    {
        return fail(format!("`{unknown}` is not a registered effect"));
    }
    if manifest.effects.windows(2).any(|pair| pair[0] >= pair[1]) {
        return fail("effects are strictly ascending");
    }
    if profile == Profile::Core && manifest.effects.iter().any(|effect| effect == "allocation") {
        return fail("hidden allocation: a rust-core package states the effect `allocation`");
    }
    let program = manifest.program.canonical()?;
    let mut lowering = Lowering::new(&program, profile).map_err(|reason| {
        if reason.contains("requires heap allocation") {
            format!("hidden allocation: {reason}")
        } else {
            reason
        }
    })?;
    let mut krate = lowering.lower().map_err(|reason| {
        if reason.contains("requires heap allocation") {
            format!("hidden allocation: {reason}")
        } else {
            reason
        }
    })?;
    krate.items.extend(exports(manifest, &mut lowering)?);
    validate::validate(&krate)?;
    let mut realized = elements(&program, &lowering);
    realized.insert("export".to_owned());
    validate::correspond(&krate, &realized)?;
    Ok((krate, program, profile))
}

/// Render a package.
///
/// # Errors
///
/// Returns the reason the manifest is malformed, names an invalid program,
/// misstates the program's interface (an identifier collision, an ownership
/// mismatch, an unsupported boundary type, or an arithmetic mismatch), or
/// needs heap allocation its profile does not provide.
pub fn package(manifest: &Manifest) -> Result<Package, String> {
    let (krate, program, profile) = lower(manifest)?;
    let library = ast::print(&krate)?.into_bytes();
    let cargo = cargo_toml(manifest)?.into_bytes();
    let runtime = match profile {
        Profile::Core => runtime::CORE.to_owned(),
        Profile::Std => format!("{}{}", runtime::CORE, runtime::STD),
    };
    let provenance = serde_json::json!({
        "spec": PROVENANCE_SPEC,
        "name": manifest.name,
        "version": manifest.version,
        "profile": profile.target(),
        "program": program.id()?.to_hex(),
        "compiler_semantics": crate::compiler_semantics_id_for(crate::LANGUAGE_1_2).to_hex(),
        "runtime": sha256(runtime.as_bytes()),
        "sources": manifest.sources,
        "effects": manifest.effects,
        "exports": manifest.exports.iter().map(|export| serde_json::json!({
            "name": export.name,
            "function": export.function,
        })).collect::<Vec<_>>(),
        "files": {
            "Cargo.toml": sha256(&cargo),
            (LIBRARY_PATH): sha256(&library),
        },
    });
    let provenance =
        crate::artifact::canonical_json::Json::parse(provenance.to_string().as_bytes())
            .map_err(|error| format!("provenance is not canonical JSON: {error}"))?
            .to_file_bytes();
    let mut files = BTreeMap::new();
    files.insert("Cargo.toml".to_owned(), cargo);
    files.insert(LIBRARY_PATH.to_owned(), library);
    files.insert("provenance.json".to_owned(), provenance);
    Ok(Package { files })
}
