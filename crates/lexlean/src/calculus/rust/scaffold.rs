//! The scaffolding of a Rust package as data (SPEC.md §17.16).
//!
//! What a package says about itself beside its program — the edition, whether
//! it is published, the lint gate with each exception and
//! the Clippy group it comes from, and the crate attributes — is not text of
//! this crate. It is one language-1.2 semantic module, `RustPackage`, held at
//! [`MODULE_PATH`] in the embedded tree, so the compiler-semantics ID (§21.2)
//! hashes it with every other language file, and this file is its *reader*.
//!
//! The module is decoded, never evaluated. The meaning of a semantic term is
//! decided by the generated Lean and its kernel; an interpreter of terms here
//! would be an unverified second reading of the language this crate ships,
//! to learn what a closed list of strings is. The decoder instead accepts
//! exactly the shapes the module uses (literals, constructors of its own
//! inductives, records, lists) and refuses every other, and it requires the
//! module to declare exactly the names read below with the members read
//! below, so a change to the module's vocabulary is a refusal and never a
//! silent reading of less.
//!
//! The crate attributes are not stored. `#![no_std]` is the target row of
//! `language/production-1.2.toml` with `allocation = false`, and
//! `#![forbid(lint)]` restates each gate on rustc's lints at level `forbid`, so
//! the fact lives in one place and the manifest and the crate cannot differ.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use super::Profile;
use crate::artifact::canonical_json::Json;
use crate::surface::module::{self, SEMANTIC};

/// Where the module is in the embedded tree.
pub const MODULE_PATH: &str = "language/package-1.2/library/src/RustPackage.lex.tex";

/// A lint level, in Cargo's spelling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    /// Permitted.
    Allow,
    /// Reported.
    Warn,
    /// An error that an inner attribute can lower.
    Deny,
    /// An error that nothing inside the crate can lower.
    Forbid,
}

impl Level {
    const ALL: [Self; 4] = [Self::Allow, Self::Warn, Self::Deny, Self::Forbid];

    /// Cargo's and rustc's spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allow => "allow",
            Self::Warn => "warn",
            Self::Deny => "deny",
            Self::Forbid => "forbid",
        }
    }
}

/// A Clippy lint group.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClippyGroup {
    /// Code that is outright wrong or useless.
    Correctness,
    /// Code that is most likely wrong or useless.
    Suspicious,
    /// Code that should be written in a more idiomatic way.
    Style,
    /// Code that does something simple in a complex way.
    Complexity,
    /// Code that can be written to run faster.
    Perf,
    /// A strict, opinionated group.
    Pedantic,
    /// Lints that restrict the language.
    Restriction,
    /// Lints under development.
    Nursery,
    /// Lints about the manifest.
    Cargo,
}

impl ClippyGroup {
    const ALL: [Self; 9] = [
        Self::Correctness,
        Self::Suspicious,
        Self::Style,
        Self::Complexity,
        Self::Perf,
        Self::Pedantic,
        Self::Restriction,
        Self::Nursery,
        Self::Cargo,
    ];

    /// Clippy's spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Correctness => "correctness",
            Self::Suspicious => "suspicious",
            Self::Style => "style",
            Self::Complexity => "complexity",
            Self::Perf => "perf",
            Self::Pedantic => "pedantic",
            Self::Restriction => "restriction",
            Self::Nursery => "nursery",
            Self::Cargo => "cargo",
        }
    }
}

/// One lint or lint group and the level the gate holds it at. A priority
/// other than zero is Cargo's, so that a group is set below the lints that
/// except from it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gate {
    /// The lint or group.
    pub lint: String,
    /// Its level.
    pub level: Level,
    /// Cargo's priority among the lint table's entries.
    pub priority: i64,
}

/// A lint of the default set that a package allows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintRow {
    /// The Clippy lint.
    pub lint: String,
    /// Its Clippy group, which is never `correctness` (§17.16).
    pub group: ClippyGroup,
    /// Why its advice would make the rendering depart from the calculus.
    pub reason: String,
}

/// The scaffolding of a package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scaffold {
    /// The Rust edition.
    pub edition: String,
    /// Whether Cargo may publish the crate.
    pub publish: bool,
    /// The gate on rustc's lints.
    pub rustc: Vec<Gate>,
    /// The gate on Clippy's lints before any exception.
    pub clippy: Vec<Gate>,
    /// The exceptions, in the order the manifest lists them.
    pub allowed: Vec<LintRow>,
}

/// The declarations the module makes, and nothing else, in the order
/// written.
const DECLARED: [&str; 11] = [
    "Level",
    "ClippyGroup",
    "Gate",
    "LintRow",
    "Scaffold",
    "scaffold",
    "rustcGates",
    "clippyGates",
    "allowedLints",
    "outsideCorrectness",
    "allowedLintsOutsideCorrectness",
];

fn fail<T>(reason: impl Into<String>) -> Result<T, String> {
    Err(format!("package scaffold {MODULE_PATH}: {}", reason.into()))
}

fn members<'a>(json: &'a Json, what: &str) -> Result<&'a BTreeMap<String, Json>, String> {
    match json {
        Json::Obj(members) => Ok(members),
        _ => fail(format!("{what} is not an object")),
    }
}

fn member<'a>(map: &'a BTreeMap<String, Json>, name: &str, what: &str) -> Result<&'a Json, String> {
    map.get(name)
        .map_or_else(|| fail(format!("{what} has no `{name}`")), Ok)
}

fn text<'a>(json: &'a Json, what: &str) -> Result<&'a str, String> {
    match json {
        Json::Str(text) => Ok(text),
        _ => fail(format!("{what} is not a string")),
    }
}

fn array<'a>(json: &'a Json, what: &str) -> Result<&'a [Json], String> {
    match json {
        Json::Arr(items) => Ok(items),
        _ => fail(format!("{what} is not an array")),
    }
}

/// The names of the members of an array of objects with a `name`.
fn names(json: &Json, what: &str) -> Result<Vec<String>, String> {
    array(json, what)?
        .iter()
        .map(|item| {
            let item = members(item, what)?;
            Ok(text(member(item, "name", what)?, what)?.to_owned())
        })
        .collect()
}

fn kind<'a>(map: &'a BTreeMap<String, Json>, what: &str) -> Result<&'a str, String> {
    text(member(map, "kind", what)?, what)
}

/// A literal term of kind `expected`, as its `value` member.
fn literal<'a>(term: &'a Json, expected: &str, what: &str) -> Result<&'a Json, String> {
    let map = members(term, what)?;
    if kind(map, what)? != expected {
        return fail(format!("{what} is not a {expected} literal"));
    }
    member(map, "value", what)
}

fn string_term(term: &Json, what: &str) -> Result<String, String> {
    Ok(text(literal(term, "string", what)?, what)?.to_owned())
}

fn bool_term(term: &Json, what: &str) -> Result<bool, String> {
    match literal(term, "bool", what)? {
        Json::Bool(value) => Ok(*value),
        _ => fail(format!("{what} is not a Boolean")),
    }
}

fn integer_term(term: &Json, what: &str) -> Result<i64, String> {
    text(literal(term, "integer", what)?, what)?
        .parse()
        .map_err(|_| format!("package scaffold {MODULE_PATH}: {what} is not an integer"))
}

/// A constructor of the inductive `of` with no fields, as its index among
/// `constructors`.
fn constructor_term(
    term: &Json,
    of: &str,
    constructors: &[&str],
    what: &str,
) -> Result<usize, String> {
    let map = members(term, what)?;
    if kind(map, what)? != "constructor" {
        return fail(format!("{what} is not a constructor of `{of}`"));
    }
    if !array(member(map, "arguments", what)?, what)?.is_empty() {
        return fail(format!("{what} gives a constructor of `{of}` an argument"));
    }
    let name = text(
        member(
            members(member(map, "constructor", what)?, what)?,
            "name",
            what,
        )?,
        what,
    )?;
    name.strip_prefix(of)
        .and_then(|rest| rest.strip_prefix('.'))
        .and_then(|short| constructors.iter().position(|known| *known == short))
        .map_or_else(
            || fail(format!("{what} is `{name}`, not a constructor of `{of}`")),
            Ok,
        )
}

/// The fields of a record term of `of`, each by its name, exactly once.
fn record_term<'a, const N: usize>(
    term: &'a Json,
    of: &str,
    fields: [&str; N],
    what: &str,
) -> Result<[&'a Json; N], String> {
    let map = members(term, what)?;
    if kind(map, what)? != "record" {
        return fail(format!("{what} is not a record of `{of}`"));
    }
    let named = members(member(map, "type", what)?, what)?;
    if text(member(named, "name", what)?, what)? != of {
        return fail(format!("{what} is not a record of `{of}`"));
    }
    let given = array(member(map, "fields", what)?, what)?;
    if given.len() != N {
        return fail(format!("{what} does not state the {N} fields of `{of}`"));
    }
    let mut by_name = BTreeMap::new();
    for field in given {
        let field = members(field, what)?;
        let name = text(member(field, "field", what)?, what)?;
        if by_name
            .insert(name, member(field, "value", what)?)
            .is_some()
        {
            return fail(format!("{what} states `{name}` twice"));
        }
    }
    let mut out = Vec::with_capacity(N);
    for name in fields {
        out.push(
            by_name
                .get(name)
                .copied()
                .map_or_else(|| fail(format!("{what} does not state `{name}`")), Ok)?,
        );
    }
    out.try_into()
        .map_err(|_| format!("package scaffold {MODULE_PATH}: {what} has the wrong fields"))
}

/// The elements of a list term, each decoded by `element`.
fn list_term<T>(
    term: &Json,
    what: &str,
    mut element: impl FnMut(&Json) -> Result<T, String>,
) -> Result<Vec<T>, String> {
    let mut out = Vec::new();
    let mut rest = term;
    loop {
        let map = members(rest, what)?;
        match kind(map, what)? {
            "nil" => return Ok(out),
            "cons" => {
                out.push(element(member(map, "head", what)?)?);
                rest = member(map, "tail", what)?;
            }
            _ => return fail(format!("{what} is not a list of `cons` and `nil`")),
        }
    }
}

fn level_term(term: &Json, what: &str) -> Result<Level, String> {
    let names = Level::ALL.map(Level::as_str);
    constructor_term(term, "Level", &names, what).map(|at| Level::ALL[at])
}

fn group_term(term: &Json, what: &str) -> Result<ClippyGroup, String> {
    let names = ClippyGroup::ALL.map(ClippyGroup::as_str);
    constructor_term(term, "ClippyGroup", &names, what).map(|at| ClippyGroup::ALL[at])
}

fn gates(term: &Json, what: &str) -> Result<Vec<Gate>, String> {
    list_term(term, what, |row| {
        let [lint, level, priority] =
            record_term(row, "Gate", ["lint", "level", "priority"], what)?;
        Ok(Gate {
            lint: string_term(lint, what)?,
            level: level_term(level, what)?,
            priority: integer_term(priority, what)?,
        })
    })
}

/// A name that is plain in a TOML key, a string, and a Rust attribute: the
/// printers write it without escaping.
fn plain(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.' | '/'))
}

/// Whether no row of `rows` is in Clippy's `correctness` group: the claim of
/// §17.16, which the module also states as a theorem about the same list.
#[must_use]
pub fn outside_correctness(rows: &[LintRow]) -> bool {
    rows.iter().all(|row| row.group != ClippyGroup::Correctness)
}

/// A call of `name` with the given arguments, as a term of the data form.
fn call_of(term: &Json, name: &str, arguments: &[&Json]) -> bool {
    let Json::Obj(map) = term else { return false };
    map.get("kind") == Some(&Json::Str("call".to_owned()))
        && matches!(map.get("function"), Some(Json::Obj(function))
            if function.get("name") == Some(&Json::Str(name.to_owned())))
        && matches!(map.get("arguments"), Some(Json::Arr(given))
            if given.len() == arguments.len() && given.iter().zip(arguments).all(|(a, b)| a == *b))
}

/// The claim that no allowed lint is in `correctness` (§17.16) as the module
/// states it must be the claim [`outside_correctness`] computes: the
/// predicate is false exactly on a row of that group and otherwise recurs on
/// the rest, and the theorem equates the predicate over `allowedLints` with
/// `true`. A predicate or a theorem weakened to say less is refused here, so
/// that the proof the module carries is a proof of this claim.
fn check_claim(
    predicate: &BTreeMap<String, Json>,
    theorem: &BTreeMap<String, Json>,
) -> Result<(), String> {
    let name = "`outsideCorrectness`";
    let statement = members(
        member(theorem, "statement", "the theorem")?,
        "the statement",
    )?;
    let allowed = Json::Obj(
        [
            ("arguments".to_owned(), Json::Arr(Vec::new())),
            (
                "function".to_owned(),
                Json::object(vec![("name", Json::Str("allowedLints".to_owned()))]),
            ),
            ("kind".to_owned(), Json::Str("call".to_owned())),
        ]
        .into(),
    );
    if kind(statement, "the theorem")? != "eq"
        || !call_of(
            member(statement, "left", "the theorem")?,
            "outsideCorrectness",
            &[&allowed],
        )
        || literal(
            member(statement, "right", "the theorem")?,
            "bool",
            "the theorem",
        )? != &Json::Bool(true)
    {
        return fail("`allowedLintsOutsideCorrectness` does not state `outsideCorrectness allowedLints = true`");
    }
    let body = members(member(predicate, "body", name)?, name)?;
    let branches = array(member(body, "branches", name)?, name)?;
    let [nil, cons] = branches else {
        return fail(format!("{name} is not a match on a list"));
    };
    let (nil, cons) = (members(nil, name)?, members(cons, name)?);
    if !matches!(member(nil, "body", name)?, Json::Obj(map)
        if map.get("kind") == Some(&Json::Str("bool".to_owned()))
            && map.get("value") == Some(&Json::Bool(true)))
    {
        return fail(format!("{name} is not true of the empty list"));
    }
    let rest = Json::object(vec![
        ("kind", Json::Str("var".to_owned())),
        ("name", Json::Str("rest".to_owned())),
    ]);
    let by_group = members(member(cons, "body", name)?, name)?;
    let groups = array(member(by_group, "branches", name)?, name)?;
    if groups.len() != ClippyGroup::ALL.len() {
        return fail(format!("{name} does not match every Clippy group"));
    }
    for (group, branch) in ClippyGroup::ALL.iter().zip(groups) {
        let branch = members(branch, name)?;
        let constructor = text(
            member(
                members(member(branch, "constructor", name)?, name)?,
                "name",
                name,
            )?,
            name,
        )?;
        let outcome = member(branch, "body", name)?;
        let right = if *group == ClippyGroup::Correctness {
            matches!(outcome, Json::Obj(map)
                if map.get("kind") == Some(&Json::Str("bool".to_owned()))
                    && map.get("value") == Some(&Json::Bool(false)))
        } else {
            call_of(outcome, "outsideCorrectness", &[&rest])
        };
        if constructor != format!("ClippyGroup.{}", group.as_str()) || !right {
            return fail(format!("{name} is not false exactly on `correctness`"));
        }
    }
    Ok(())
}

/// Read the scaffold from the source of the module.
///
/// # Errors
///
/// Returns the reason the source is not a module of the shape this reader
/// accepts, or states a scaffold the renderer cannot honor: a repeated lint,
/// a name that is not plain, an allowed lint in Clippy's `correctness` group,
/// or a claim about that group that says less than [`outside_correctness`].
pub fn decode(source: &str) -> Result<Scaffold, String> {
    let Some(found) = module::locate(source) else {
        return fail("holds no module");
    };
    if !found.kind.is(&SEMANTIC) {
        return fail("is not a semantic module");
    }
    let data = match found.data {
        Some((start, end)) => source[start..end].to_owned(),
        None => {
            let spec = (found.kind.spec)().map_or_else(|| fail("has no schema"), Ok)?;
            found
                .kind
                .data_text(&source[found.body.0..found.body.1], spec)
                .map_err(|error| format!("package scaffold {MODULE_PATH}: {}", error.message))?
        }
    };
    let json = Json::parse(data.as_bytes())
        .map_err(|error| format!("package scaffold {MODULE_PATH}: {error}"))?;
    let root = members(&json, "the module")?;
    let declarations = array(
        member(root, "declarations", "the module")?,
        "`declarations`",
    )?;
    let mut by_name: BTreeMap<&str, &BTreeMap<String, Json>> = BTreeMap::new();
    for declaration in declarations {
        let declaration = members(declaration, "a declaration")?;
        let name = text(member(declaration, "name", "a declaration")?, "a name")?;
        if !DECLARED.contains(&name) {
            return fail(format!(
                "declares `{name}`, which the renderer does not read"
            ));
        }
        if by_name.insert(name, declaration).is_some() {
            return fail(format!("declares `{name}` twice"));
        }
    }
    for name in DECLARED {
        if !by_name.contains_key(name) {
            return fail(format!("does not declare `{name}`"));
        }
    }
    let declared = |name: &str, expected: &str| -> Result<&BTreeMap<String, Json>, String> {
        let declaration = by_name[name];
        if kind(declaration, name)? == expected {
            Ok(declaration)
        } else {
            fail(format!("`{name}` is not a {expected}"))
        }
    };

    // The vocabulary: the inductives and structures the terms below are
    // records and constructors of.
    for (name, expected) in [
        ("Level", Level::ALL.map(Level::as_str).to_vec()),
        (
            "ClippyGroup",
            ClippyGroup::ALL.map(ClippyGroup::as_str).to_vec(),
        ),
    ] {
        let inductive = declared(name, "inductive")?;
        let constructors = array(member(inductive, "constructors", name)?, name)?;
        let mut found = Vec::new();
        for constructor in constructors {
            let constructor = members(constructor, name)?;
            if !array(member(constructor, "fields", name)?, name)?.is_empty() {
                return fail(format!("`{name}` has a constructor with fields"));
            }
            found.push(text(member(constructor, "name", name)?, name)?.to_owned());
        }
        if found != expected {
            return fail(format!(
                "`{name}` has the constructors {found:?}, not {expected:?}"
            ));
        }
    }
    for (name, expected) in [
        ("Gate", vec!["lint", "level", "priority"]),
        ("LintRow", vec!["lint", "group", "reason"]),
        ("Scaffold", vec!["edition", "publish"]),
    ] {
        let structure = declared(name, "structure")?;
        let found = names(member(structure, "fields", name)?, name)?;
        if found != expected {
            return fail(format!(
                "`{name}` has the fields {found:?}, not {expected:?}"
            ));
        }
    }
    check_claim(
        declared("outsideCorrectness", "definition")?,
        declared("allowedLintsOutsideCorrectness", "theorem")?,
    )?;

    let body = |name: &str| -> Result<&Json, String> {
        let definition = declared(name, "definition")?;
        if !array(member(definition, "parameters", name)?, name)?.is_empty() {
            return fail(format!("`{name}` takes parameters"));
        }
        member(definition, "body", name)
    };
    let [edition, publish] = record_term(
        body("scaffold")?,
        "Scaffold",
        ["edition", "publish"],
        "`scaffold`",
    )?;
    let scaffold = Scaffold {
        edition: string_term(edition, "`edition`")?,
        publish: bool_term(publish, "`publish`")?,
        rustc: gates(body("rustcGates")?, "`rustcGates`")?,
        clippy: gates(body("clippyGates")?, "`clippyGates`")?,
        allowed: list_term(body("allowedLints")?, "`allowedLints`", |row| {
            let [lint, group, reason] = record_term(
                row,
                "LintRow",
                ["lint", "group", "reason"],
                "`allowedLints`",
            )?;
            Ok(LintRow {
                lint: string_term(lint, "`lint`")?,
                group: group_term(group, "`group`")?,
                reason: string_term(reason, "`reason`")?,
            })
        })?,
    };
    scaffold.check()?;
    Ok(scaffold)
}

impl Scaffold {
    /// What the renderer relies on, which the module cannot state in its
    /// types.
    fn check(&self) -> Result<(), String> {
        for value in [&self.edition] {
            if !plain(value) {
                return fail(format!("`{value}` is not a plain name"));
            }
        }
        let mut seen = BTreeSet::new();
        for gate in self.rustc.iter().chain(&self.clippy) {
            if !plain(&gate.lint) {
                return fail(format!("`{}` is not a plain lint name", gate.lint));
            }
        }
        for table in [&self.rustc, &self.clippy] {
            seen.clear();
            for gate in table {
                if !seen.insert(gate.lint.as_str()) {
                    return fail(format!("gates `{}` twice", gate.lint));
                }
            }
        }
        seen.clear();
        for row in &self.allowed {
            if !plain(&row.lint) {
                return fail(format!("`{}` is not a plain lint name", row.lint));
            }
            if !seen.insert(row.lint.as_str()) {
                return fail(format!("allows `{}` twice", row.lint));
            }
            if row.reason.trim().is_empty() {
                return fail(format!("allows `{}` with no reason", row.lint));
            }
        }
        if !outside_correctness(&self.allowed) {
            return fail("allows a lint in Clippy's `correctness` group (§17.16)");
        }
        Ok(())
    }

    /// The crate attributes of a package in `profile`, one per line, in the
    /// order the library states them: `#![no_std]` where the profile's
    /// target row of the production registry allows no allocation, then
    /// `#![forbid(lint)]` for every gate at level `forbid`.
    ///
    /// # Errors
    ///
    /// Returns the reason the production registry has no row for the
    /// profile.
    pub fn attributes(&self, profile: Profile) -> Result<String, String> {
        let registry = crate::production::registry()?;
        let target = registry.targets.get(profile.target()).ok_or_else(|| {
            format!(
                "the production registry has no target `{}`",
                profile.target()
            )
        })?;
        Ok(self.attributes_with(target.allocation))
    }

    /// The crate attributes of a package whose target has `allocation`.
    #[must_use]
    pub fn attributes_with(&self, allocation: bool) -> String {
        let mut out = String::new();
        if !allocation {
            out.push_str("#![no_std]\n");
        }
        for gate in self.rustc.iter().filter(|gate| gate.level == Level::Forbid) {
            out.push_str(&format!("#![{}({})]\n", Level::Forbid.as_str(), gate.lint));
        }
        out
    }

    /// Every string the module states as content: what no renderer source
    /// may hold as text of its own.
    #[must_use]
    pub fn content(&self) -> Vec<String> {
        let mut out = vec![self.edition.clone(), format!("publish = {}", self.publish)];
        for gate in self.rustc.iter().chain(&self.clippy) {
            out.push(gate.lint.clone());
            if gate.priority != 0 {
                out.push(format!("priority = {}", gate.priority));
            }
        }
        for row in &self.allowed {
            out.push(row.lint.clone());
            out.push(row.reason.clone());
        }
        out
    }
}

/// The scaffold of the embedded module.
///
/// # Errors
///
/// As [`decode`], or the module is not in the embedded tree.
pub fn get() -> Result<&'static Scaffold, String> {
    static SCAFFOLD: OnceLock<Result<Scaffold, String>> = OnceLock::new();
    SCAFFOLD
        .get_or_init(|| {
            let (_, bytes) = crate::embedded::FILES
                .iter()
                .find(|(path, _)| *path == MODULE_PATH)
                .map_or_else(|| fail("is not in the embedded tree"), Ok)?;
            let source = std::str::from_utf8(bytes)
                .map_err(|_| format!("package scaffold {MODULE_PATH}: is not UTF-8"))?;
            decode(source)
        })
        .as_ref()
        .map_err(Clone::clone)
}

/// The string literals of Rust source, comments excluded: the audit of
/// [`audit`] reads the renderer's files as these.
fn literals(source: &str) -> Vec<String> {
    let bytes = source.as_bytes();
    let mut out = Vec::new();
    let mut at = 0;
    while at < bytes.len() {
        match bytes[at] {
            b'/' if bytes.get(at + 1) == Some(&b'/') => {
                while at < bytes.len() && bytes[at] != b'\n' {
                    at += 1;
                }
            }
            b'/' if bytes.get(at + 1) == Some(&b'*') => {
                at += 2;
                while at + 1 < bytes.len() && !(bytes[at] == b'*' && bytes[at + 1] == b'/') {
                    at += 1;
                }
                at += 2;
            }
            b'\'' => {
                // A character literal or a lifetime: skip the quote and the
                // character after it, so `'"'` opens no string.
                at += if bytes.get(at + 1) == Some(&b'\\') {
                    3
                } else {
                    2
                };
            }
            b'"' => {
                let mut literal = Vec::new();
                at += 1;
                while at < bytes.len() && bytes[at] != b'"' {
                    let escaped = bytes[at] == b'\\';
                    if escaped {
                        at += 1;
                    }
                    if at < bytes.len() {
                        // An escape that names white space is white space,
                        // so `\n` separates words as the byte does.
                        literal.push(match bytes[at] {
                            b'n' | b't' | b'r' if escaped => b' ',
                            other => other,
                        });
                    }
                    at += 1;
                }
                out.push(String::from_utf8_lossy(&literal).into_owned());
                at += 1;
            }
            _ => at += 1,
        }
    }
    out
}

/// Whether `literal` states `value`: as a word of its own, or, for a value
/// too short to be a word of its own in prose (a group named `all`), as the
/// whole literal or as a key, a word followed by `=`.
fn states(literal: &str, value: &str) -> bool {
    let word = |c: char| c.is_ascii_alphanumeric() || c == '_';
    let short = value.len() < 4;
    literal.match_indices(value).any(|(start, _)| {
        let before = literal[..start].chars().next_back();
        let rest = &literal[start + value.len()..];
        let delimited = !before.is_some_and(word) && !rest.chars().next().is_some_and(word);
        delimited
            && (!short
                || (before.is_none() && rest.is_empty())
                || rest.trim_start().starts_with('='))
    })
}

/// The scaffold audit (`RB-10`): no source of the renderer holds as a string
/// literal any string the module states as content. A manifest template, a
/// table of lints, or a header written as text beside the module is refused,
/// whatever it is named, because the module would then no longer be the one
/// place the scaffolding is stated. The reader itself is given as `reader`
/// and is read for the same literals, so a constant parked in it is refused
/// too; only the vocabulary it matches on (levels, groups, field names) is
/// outside the content.
///
/// # Errors
///
/// Returns each source and literal that states module content.
pub fn audit(scaffold: &Scaffold, sources: &[(&str, &str)]) -> Result<(), String> {
    let content = scaffold.content();
    let mut found = Vec::new();
    for (name, source) in sources {
        for literal in literals(source) {
            for value in &content {
                if literal == *value || states(&literal, value) {
                    found.push(format!("{name}: the literal {literal:?} states `{value}`"));
                    break;
                }
            }
        }
    }
    if found.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "the renderer holds the package scaffolding as text beside its module: {}",
            found.join("; ")
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{audit, decode, get, literals, outside_correctness, states, MODULE_PATH};
    use crate::calculus::rust::Profile;

    #[test]
    fn the_embedded_module_decodes() {
        let scaffold = get().expect("the scaffold");
        assert!(!scaffold.edition.is_empty());
        assert!(scaffold.allowed.len() > 1);
        assert!(outside_correctness(&scaffold.allowed));
        let forbidden = &scaffold.rustc[0].lint;
        assert_eq!(
            scaffold.attributes(Profile::Core).expect("attributes"),
            format!("#![no_std]\n#![forbid({forbidden})]\n")
        );
        assert_eq!(
            scaffold.attributes(Profile::Std).expect("attributes"),
            format!("#![forbid({forbidden})]\n")
        );
    }

    #[test]
    fn literals_exclude_comments_and_characters() {
        let source = "// \"no\"\nlet a = \"yes\"; let c = '\"'; /* \"no\" */ let b = \"a\\\"b\";";
        assert_eq!(literals(source), ["yes", "a\"b"]);
    }

    #[test]
    fn a_literal_states_a_value_as_a_word() {
        assert!(states("a lint_x b", "lint_x"));
        assert!(!states("a lint_xy b", "lint_x"));
        assert!(states("v", "v"));
        assert!(!states("a v b", "v"));
    }

    #[test]
    fn a_module_that_is_not_the_shape_is_refused_not_read() {
        assert!(decode("").is_err());
        assert!(
            decode("\\begin{semanticmodule}\n\\semanticdata{{}}\n\\end{semanticmodule}").is_err()
        );
        let (_, bytes) = crate::embedded::FILES
            .iter()
            .find(|(path, _)| *path == MODULE_PATH)
            .expect("the module");
        let text = std::str::from_utf8(bytes).expect("UTF-8");
        let moved = text.replacen(
            "ctor ClippyGroup.complexity",
            "ctor ClippyGroup.correctness",
            1,
        );
        assert_ne!(moved, text);
        let error = decode(&moved).expect_err("correctness is refused");
        assert!(error.contains("correctness"), "{error}");
    }

    #[test]
    fn a_planted_constant_is_refused() {
        let scaffold = get().expect("the scaffold");
        assert!(audit(scaffold, &[("clean.rs", "let x = \"unrelated\";")]).is_ok());
        let lint = &scaffold.allowed[0].lint;
        let planted = format!("const M: &str = \"[lints.clippy] {lint} = allow\";");
        let error = audit(scaffold, &[("planted.rs", &planted)]).expect_err("a planted constant");
        assert!(error.contains(lint.as_str()), "{error}");
    }
}
