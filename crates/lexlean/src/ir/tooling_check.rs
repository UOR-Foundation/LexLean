//! The checks of a tooling module (SPEC.md §17.20): registry closure, the
//! type of every term, exhaustive index-aware matches, licensed recursion,
//! and complete traversal. Lean elaborates the generated text again; these
//! checks are what keeps a module from being accepted on Lean's word alone.

use std::collections::{BTreeMap, BTreeSet};

use crate::ir::tooling::{
    BinderInfo, CommandAtom, InvokeArgument, ParameterKind, Sig, ToolConstructor, ToolDeclaration,
    ToolParameter, ToolTerm, ToolType, ToolingFailure, ToolingModule, TypeParameter,
};

/// The names the language itself fixes. They are registered types of the
/// module like any other (their constructors and fields are declared), but a
/// literal or a conditional gives them a meaning.
pub const BOOL: &str = "Bool";
/// The natural numbers.
pub const NAT: &str = "Nat";
/// Strings.
pub const STRING: &str = "String";
/// Characters.
pub const CHAR: &str = "Char";
/// `Lean.Name`.
pub const NAME: &str = "Lean.Name";
/// Lists.
pub const LIST: &str = "List";
/// Arrays.
pub const ARRAY: &str = "Array";
/// The unit type.
pub const UNIT: &str = "Unit";
/// `Lean.Syntax`.
pub const SYNTAX: &str = "Lean.Syntax";
/// The monad of commands.
pub const COMMAND_M: &str = "Lean.Elab.Command.CommandElabM";
/// The type of a command elaborator, an abbreviation of
/// `Lean.Syntax → CommandElabM Unit` that Lean's `command_elab` attribute
/// requires by name.
pub const COMMAND_ELAB: &str = "Lean.Elab.Command.CommandElab";

/// A registered type.
#[derive(Debug, Clone)]
pub struct TypeInfo<'a> {
    pub(crate) parameters: &'a [TypeParameter],
    pub(crate) constructors: &'a [ToolConstructor],
    pub(crate) alias: Option<&'a ToolType>,
    pub(crate) opaque: bool,
}

/// The typing of a registered call or a function, derived once.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FnType {
    /// The implicit type parameters, in order.
    pub type_parameters: Vec<String>,
    /// The explicit parameters.
    pub parameters: Vec<ToolType>,
    /// How many trailing parameters carry a default.
    pub defaulted: usize,
    /// The result.
    pub result: ToolType,
}

/// What the checks know of the module.
pub struct Registry<'a> {
    types: BTreeMap<&'a str, TypeInfo<'a>>,
    calls: BTreeMap<&'a str, FnType>,
    /// Every call the registry declares, whatever its place among the rows: a
    /// signature may name a call that a later row declares (`Array.foldl`
    /// names `Array.size` in the default of a parameter).
    call_names: BTreeSet<&'a str>,
    functions: BTreeMap<&'a str, FnType>,
    commands: BTreeMap<&'a str, &'a [CommandAtom]>,
    plumbing: BTreeSet<&'a str>,
}

fn is_ident(name: &str) -> bool {
    !name.is_empty()
        && name.split('.').all(|segment| {
            let mut chars = segment.chars();
            chars
                .next()
                .is_some_and(|first| first.is_alphabetic() || first == '_')
                && segment.chars().all(|c| {
                    c.is_alphanumeric()
                        || matches!(c, '_' | '\'' | '!' | '?' | '\u{2080}'..='\u{2089}')
                })
        })
}

fn require_ident(what: &str, name: &str) -> Result<(), ToolingFailure> {
    if is_ident(name) {
        Ok(())
    } else {
        Err(ToolingFailure::schema(format!(
            "{what} `{name}` is not a Lean name"
        )))
    }
}

fn is_sha256(text: &str) -> bool {
    text.len() == 64
        && text
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// Substitute type variables.
pub fn subst(ty: &ToolType, map: &BTreeMap<&str, ToolType>) -> ToolType {
    match ty {
        ToolType::Const { name, arguments } => ToolType::Const {
            name: name.clone(),
            arguments: arguments.iter().map(|a| subst(a, map)).collect(),
        },
        ToolType::Parameter { name, arguments } => {
            let arguments: Vec<ToolType> = arguments.iter().map(|a| subst(a, map)).collect();
            match map.get(name.as_str()) {
                None => ToolType::Parameter {
                    name: name.clone(),
                    arguments,
                },
                Some(replacement) => apply_type(replacement.clone(), arguments),
            }
        }
        ToolType::Function { parameters, result } => ToolType::Function {
            parameters: parameters.iter().map(|p| subst(p, map)).collect(),
            result: Box::new(subst(result, map)),
        },
        ToolType::Index { constructor } => ToolType::Index {
            constructor: constructor.clone(),
        },
    }
}

/// Apply a type (constructor) to further arguments.
fn apply_type(head: ToolType, extra: Vec<ToolType>) -> ToolType {
    if extra.is_empty() {
        return head;
    }
    match head {
        ToolType::Const {
            name,
            mut arguments,
        } => {
            arguments.extend(extra);
            ToolType::Const { name, arguments }
        }
        ToolType::Parameter {
            name,
            mut arguments,
        } => {
            arguments.extend(extra);
            ToolType::Parameter { name, arguments }
        }
        other => other,
    }
}

/// A short rendering for diagnostics.
#[must_use]
pub fn show_type(ty: &ToolType) -> String {
    match ty {
        ToolType::Const { name, arguments } | ToolType::Parameter { name, arguments } => {
            if arguments.is_empty() {
                name.clone()
            } else {
                format!(
                    "({name} {})",
                    arguments
                        .iter()
                        .map(show_type)
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            }
        }
        ToolType::Function { parameters, result } => format!(
            "({} → {})",
            parameters
                .iter()
                .map(show_type)
                .collect::<Vec<_>>()
                .join(" → "),
            show_type(result)
        ),
        ToolType::Index { constructor } => constructor.clone(),
    }
}

impl<'a> Registry<'a> {
    /// A type with every abbreviation unfolded one level, everywhere.
    pub(crate) fn unfold(&self, ty: &ToolType) -> ToolType {
        match ty {
            ToolType::Const { name, arguments } => {
                match self.types.get(name.as_str()).and_then(|info| info.alias) {
                    Some(target) if arguments.is_empty() => target.clone(),
                    _ => ToolType::Const {
                        name: name.clone(),
                        arguments: arguments.iter().map(|a| self.unfold(a)).collect(),
                    },
                }
            }
            ToolType::Parameter { name, arguments } => ToolType::Parameter {
                name: name.clone(),
                arguments: arguments.iter().map(|a| self.unfold(a)).collect(),
            },
            ToolType::Function { parameters, result } => ToolType::Function {
                parameters: parameters.iter().map(|p| self.unfold(p)).collect(),
                result: Box::new(self.unfold(result)),
            },
            ToolType::Index { constructor } => ToolType::Index {
                constructor: constructor.clone(),
            },
        }
    }

    /// Whether a type still names an abbreviation.
    fn mentions_alias(&self, ty: &ToolType) -> bool {
        match ty {
            ToolType::Const { name, arguments } => {
                self.types
                    .get(name.as_str())
                    .is_some_and(|info| info.alias.is_some())
                    || arguments.iter().any(|a| self.mentions_alias(a))
            }
            ToolType::Parameter { arguments, .. } => {
                arguments.iter().any(|a| self.mentions_alias(a))
            }
            ToolType::Function { parameters, result } => {
                parameters.iter().any(|p| self.mentions_alias(p)) || self.mentions_alias(result)
            }
            ToolType::Index { .. } => false,
        }
    }

    /// A type with every abbreviation unfolded completely.
    pub(crate) fn normal(&self, ty: &ToolType) -> ToolType {
        let mut cursor = ty.clone();
        for _ in 0..64 {
            let next = self.unfold(&cursor);
            if next == cursor {
                break;
            }
            cursor = next;
        }
        cursor
    }

    fn type_info(&self, name: &str) -> Result<&TypeInfo<'a>, ToolingFailure> {
        self.types
            .get(name)
            .ok_or_else(|| ToolingFailure::registry(format!("`{name}` is not a registered type")))
    }

    /// The constructor with a full name, with its type.
    fn constructor(
        &self,
        full: &str,
    ) -> Result<(&str, &TypeInfo<'a>, &'a ToolConstructor), ToolingFailure> {
        let (owner, short) = full
            .rsplit_once('.')
            .ok_or_else(|| ToolingFailure::check(format!("`{full}` is not a constructor name")))?;
        let info = self.types.get(owner).ok_or_else(|| {
            ToolingFailure::check(format!("`{full}`: `{owner}` is not a registered type"))
        })?;
        if info.opaque {
            return Err(ToolingFailure::check(format!(
                "`{owner}` is opaque: its constructors are not pinned, so `{full}` cannot be named"
            )));
        }
        let ctor = info
            .constructors
            .iter()
            .find(|ctor| ctor.name == short)
            .ok_or_else(|| {
                ToolingFailure::check(format!("`{owner}` has no constructor `{short}`"))
            })?;
        let owner = self
            .types
            .get_key_value(owner)
            .map(|(key, _)| *key)
            .expect("found above");
        Ok((owner, info, ctor))
    }

    fn well_formed(
        &self,
        ty: &ToolType,
        scope: &BTreeSet<&str>,
        value: bool,
    ) -> Result<(), ToolingFailure> {
        match ty {
            ToolType::Const { name, arguments } => {
                let info = self.type_info(name)?;
                if arguments.len() > info.parameters.len() {
                    return Err(ToolingFailure::check(format!(
                        "`{name}` takes {} arguments, {} are given",
                        info.parameters.len(),
                        arguments.len()
                    )));
                }
                if value && arguments.len() != info.parameters.len() {
                    return Err(ToolingFailure::check(format!(
                        "`{name}` takes {} arguments, {} are given: a type constructor is not the type of a value",
                        info.parameters.len(),
                        arguments.len()
                    )));
                }
                for (argument, parameter) in arguments.iter().zip(info.parameters) {
                    match (&parameter.kind, argument) {
                        (ParameterKind::Index { of }, ToolType::Index { constructor }) => {
                            let (owner, _, ctor) = self.constructor(constructor)?;
                            if owner != of || !ctor.fields.is_empty() {
                                return Err(ToolingFailure::check(format!(
                                    "`{constructor}` is not a nullary constructor of `{of}`"
                                )));
                            }
                        }
                        (ParameterKind::Index { of }, _) => {
                            if !matches!(argument, ToolType::Parameter { .. }) {
                                return Err(ToolingFailure::check(format!(
                                    "the index argument of `{name}` is an index of `{of}`"
                                )));
                            }
                            self.well_formed(argument, scope, true)?;
                        }
                        (ParameterKind::Type, _) => self.well_formed(argument, scope, true)?,
                        (ParameterKind::Constructor, _) => {
                            self.well_formed(argument, scope, false)?;
                        }
                    }
                }
                Ok(())
            }
            ToolType::Parameter { name, arguments } => {
                if !scope.contains(name.as_str()) {
                    return Err(ToolingFailure::check(format!(
                        "the type variable `{name}` is not in scope"
                    )));
                }
                for argument in arguments {
                    self.well_formed(argument, scope, true)?;
                }
                Ok(())
            }
            ToolType::Function { parameters, result } => {
                if parameters.is_empty() {
                    return Err(ToolingFailure::check("a function type has a parameter"));
                }
                for parameter in parameters {
                    self.well_formed(parameter, scope, true)?;
                }
                self.well_formed(result, scope, true)
            }
            ToolType::Index { constructor } => {
                self.constructor(constructor)?;
                if value {
                    return Err(ToolingFailure::check(format!(
                        "`{constructor}` is an index, not the type of a value"
                    )));
                }
                Ok(())
            }
        }
    }
}

/// The typing of every registered call and function of a module, by name.
///
/// # Errors
///
/// The failure of the registry's own checks.
pub fn typings(module: &ToolingModule) -> Result<BTreeMap<String, FnType>, ToolingFailure> {
    let registry = build(module)?;
    let mut all: BTreeMap<String, FnType> = BTreeMap::new();
    for (name, typing) in registry.calls.iter().chain(registry.functions.iter()) {
        all.insert((*name).to_owned(), typing.clone());
    }
    Ok(all)
}

/// Register the types, the calls, and the functions' typings.
fn build(module: &ToolingModule) -> Result<Registry<'_>, ToolingFailure> {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut registry = Registry {
        types: BTreeMap::new(),
        calls: BTreeMap::new(),
        call_names: BTreeSet::new(),
        functions: BTreeMap::new(),
        commands: BTreeMap::new(),
        plumbing: BTreeSet::new(),
    };
    // Pass 1: names and the registered types. Types may refer to one
    // another in any order (`Code`, `FunDecl`, `Alt` are one mutual family).
    for declaration in &module.declarations {
        match declaration {
            ToolDeclaration::Type {
                name,
                parameters,
                constructors,
                opaque,
                alias,
                source,
            } => {
                require_ident("type", name)?;
                if !seen.insert(name) {
                    return Err(ToolingFailure::registry(format!(
                        "`{name}` is declared twice"
                    )));
                }
                pin(name, &source.file, &source.sha256)?;
                registry.types.insert(
                    name,
                    TypeInfo {
                        parameters,
                        constructors,
                        alias: alias.as_ref(),
                        opaque: *opaque,
                    },
                );
            }
            ToolDeclaration::Call { name, .. }
            | ToolDeclaration::Function { name, .. }
            | ToolDeclaration::Command { name, .. } => {
                require_ident(declaration.kind(), name)?;
                if !seen.insert(name) {
                    return Err(ToolingFailure::registry(format!(
                        "`{name}` is declared twice"
                    )));
                }
                if matches!(declaration, ToolDeclaration::Call { .. }) {
                    registry.call_names.insert(name);
                }
            }
            ToolDeclaration::Plumbing { names } => {
                if names.is_empty() {
                    return Err(ToolingFailure::registry("a plumbing row names no constant"));
                }
                let mut sorted = names.clone();
                sorted.sort();
                sorted.dedup();
                if &sorted != names {
                    return Err(ToolingFailure::registry(
                        "plumbing names are not sorted and distinct",
                    ));
                }
                for name in names {
                    require_ident("plumbing constant", name)?;
                    if !registry.plumbing.insert(name) {
                        return Err(ToolingFailure::registry(format!(
                            "`{name}` is plumbing in two rows"
                        )));
                    }
                }
            }
            ToolDeclaration::Invoke { .. } | ToolDeclaration::Evaluate { .. } => {}
        }
    }
    for declaration in &module.declarations {
        if let ToolDeclaration::Type {
            name,
            parameters,
            constructors,
            opaque,
            alias,
            ..
        } = declaration
        {
            if *opaque && (!constructors.is_empty() || alias.is_some()) {
                return Err(ToolingFailure::registry(format!(
                    "`{name}` is opaque, so it pins no constructors and is no abbreviation"
                )));
            }
            check_type(&registry, name, parameters, constructors, alias.as_ref())?;
        }
    }
    for name in &registry.plumbing {
        if seen.contains(name) {
            return Err(ToolingFailure::registry(format!(
                "`{name}` is plumbing and also declared"
            )));
        }
    }
    // Pass 2: the calls' typings, from their structured signatures.
    for declaration in &module.declarations {
        if let ToolDeclaration::Call {
            name,
            signature,
            role,
            source,
        } = declaration
        {
            pin(name, &source.file, &source.sha256)?;
            if role.is_empty() {
                return Err(ToolingFailure::registry(format!("`{name}` has no role")));
            }
            check_sig(&registry, signature, &mut Vec::new(), name)?;
            let typing = derive(&registry, name, signature)?;
            registry.calls.insert(name, typing);
        }
    }
    // Pass 3: the functions' typings and commands.
    for declaration in &module.declarations {
        match declaration {
            ToolDeclaration::Function {
                name,
                type_parameters,
                parameters,
                result,
                ..
            } => {
                let scope: BTreeSet<&str> = type_parameters.iter().map(String::as_str).collect();
                if scope.len() != type_parameters.len() {
                    return Err(ToolingFailure::check(format!(
                        "`{name}` repeats a type parameter"
                    )));
                }
                let mut names: BTreeSet<&str> = BTreeSet::new();
                for parameter in parameters {
                    require_ident("parameter", &parameter.name)?;
                    if !names.insert(&parameter.name) {
                        return Err(ToolingFailure::check(format!(
                            "`{name}` repeats the parameter `{}`",
                            parameter.name
                        )));
                    }
                    registry.well_formed(&parameter.ty, &scope, true)?;
                }
                registry.well_formed(result, &scope, true)?;
                registry.functions.insert(
                    name,
                    FnType {
                        type_parameters: type_parameters.clone(),
                        parameters: parameters.iter().map(|p| p.ty.clone()).collect(),
                        defaulted: 0,
                        result: result.clone(),
                    },
                );
            }
            ToolDeclaration::Command { name, atoms, .. } => {
                registry.commands.insert(name, atoms);
            }
            _ => {}
        }
    }
    Ok(registry)
}

/// Check every declaration of a module (SPEC.md §17.20): the registry, then
/// every body, command, invocation, and evaluation.
///
/// # Errors
///
/// `LLT4014` for the registry, `LLT4015` for the rest.
pub fn check(module: &ToolingModule) -> Result<(), ToolingFailure> {
    let registry = build(module)?;
    // Bodies, in order. A function sees the functions before it, itself when
    // partial, and its mutual group.
    let mut defined: BTreeSet<&str> = BTreeSet::new();
    let mut calls_of: BTreeMap<&str, BTreeSet<String>> = BTreeMap::new();
    let mut groups: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    let mut previous_group: Option<&str> = None;
    for declaration in &module.declarations {
        match declaration {
            ToolDeclaration::Function {
                name,
                partial,
                mutual,
                type_parameters,
                parameters,
                result,
                body,
                ..
            } => {
                match (mutual.as_deref(), previous_group) {
                    (Some(label), Some(last)) if label == last => {}
                    (Some(label), _) if groups.contains_key(label) => {
                        return Err(ToolingFailure::check(format!(
                            "the mutual group `{label}` is not one contiguous run of functions"
                        )));
                    }
                    _ => {}
                }
                if let Some(label) = mutual {
                    groups.entry(label).or_default().push(name);
                    if !partial {
                        return Err(ToolingFailure::check(format!(
                            "`{name}` is in the mutual group `{label}` and recurses without a termination argument: it is declared `partial`"
                        )));
                    }
                }
                previous_group = mutual.as_deref();
                let group: BTreeSet<&str> = mutual
                    .as_deref()
                    .map(|label| {
                        module
                            .declarations
                            .iter()
                            .filter_map(|other| match other {
                                ToolDeclaration::Function {
                                    name: other_name,
                                    mutual: Some(other_label),
                                    ..
                                } if other_label == label => Some(other_name.as_str()),
                                _ => None,
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                let scope: BTreeSet<&str> = type_parameters.iter().map(String::as_str).collect();
                let mut env = Env {
                    registry: &registry,
                    scope: &scope,
                    locals: parameters
                        .iter()
                        .map(|p| (p.name.clone(), p.ty.clone()))
                        .collect(),
                    defined: &defined,
                    group: &group,
                    current: name,
                    partial: *partial,
                    calls: BTreeSet::new(),
                };
                let observed = env.infer(body)?;
                if observed != *result && registry.normal(&observed) != registry.normal(result) {
                    return Err(ToolingFailure::check(format!(
                        "`{name}` is declared to return {}, its body has type {}",
                        show_type(result),
                        show_type(&observed)
                    )));
                }
                calls_of.insert(name, env.calls);
                defined.insert(name);
            }
            ToolDeclaration::Command {
                name,
                atoms,
                elaborator,
            } => {
                check_command(&registry, module, name, atoms, elaborator)?;
            }
            ToolDeclaration::Invoke { command, arguments } => {
                let atoms = registry.commands.get(command.as_str()).ok_or_else(|| {
                    ToolingFailure::check(format!(
                        "`{command}` is not a command registered before this invocation"
                    ))
                })?;
                check_invoke(&registry, command, atoms, arguments)?;
            }
            ToolDeclaration::Evaluate {
                function,
                arguments,
            } => {
                let typing = registry.functions.get(function.as_str()).ok_or_else(|| {
                    ToolingFailure::check(format!("`{function}` is not a function of this module"))
                })?;
                if !defined.contains(function.as_str()) {
                    return Err(ToolingFailure::check(format!(
                        "`{function}` is evaluated before it is defined"
                    )));
                }
                let scope = BTreeSet::new();
                let group = BTreeSet::new();
                let mut env = Env {
                    registry: &registry,
                    scope: &scope,
                    locals: BTreeMap::new(),
                    defined: &defined,
                    group: &group,
                    current: "",
                    partial: false,
                    calls: BTreeSet::new(),
                };
                if !typing.type_parameters.is_empty() {
                    return Err(ToolingFailure::check(format!(
                        "`{function}` is evaluated and has type parameters"
                    )));
                }
                env.expect_arguments(&format!("`{function}`"), &typing.parameters, 0, arguments)?;
                let unit_command = ToolType::Const {
                    name: COMMAND_M.to_owned(),
                    arguments: vec![ToolType::Const {
                        name: UNIT.to_owned(),
                        arguments: Vec::new(),
                    }],
                };
                if typing.result != unit_command {
                    return Err(ToolingFailure::check(format!(
                        "`{function}` is evaluated and returns {}, not a command action",
                        show_type(&typing.result)
                    )));
                }
            }
            _ => {}
        }
    }
    let _ = calls_of;
    check_traversals(module, &registry)
}

/// Every registered type a type mentions.
fn mentioned<'t>(ty: &'t ToolType, out: &mut BTreeSet<&'t str>) {
    match ty {
        ToolType::Const { name, arguments } => {
            out.insert(name);
            for argument in arguments {
                mentioned(argument, out);
            }
        }
        ToolType::Parameter { arguments, .. } => {
            for argument in arguments {
                mentioned(argument, out);
            }
        }
        ToolType::Function { parameters, result } => {
            for parameter in parameters {
                mentioned(parameter, out);
            }
            mentioned(result, out);
        }
        ToolType::Index { .. } => {}
    }
}

/// The types of the recursive family of `root`: those that reach `root`
/// through constructor fields and are reached from it.
fn family<'a>(registry: &Registry<'a>, root: &str) -> BTreeSet<&'a str> {
    let edges = |name: &str| -> BTreeSet<&'a str> {
        let mut out = BTreeSet::new();
        if let Some(info) = registry.types.get(name) {
            for ctor in info.constructors {
                for field in &ctor.fields {
                    mentioned(&field.ty, &mut out);
                }
            }
        }
        out.retain(|candidate| registry.types.contains_key(candidate));
        out
    };
    let reach = |from: &str| -> BTreeSet<&'a str> {
        let mut seen: BTreeSet<&'a str> = BTreeSet::new();
        let mut stack: Vec<&'a str> = edges(from).into_iter().collect();
        while let Some(next) = stack.pop() {
            if seen.insert(next) {
                stack.extend(edges(next));
            }
        }
        seen
    };
    let from_root = reach(root);
    let mut family: BTreeSet<&'a str> = BTreeSet::new();
    if let Some((key, _)) = registry.types.get_key_value(root) {
        if from_root.contains(root) {
            family.insert(key);
        }
    }
    for candidate in &from_root {
        if reach(candidate).contains(root) {
            family.insert(candidate);
        }
    }
    family
}

/// Whether a term mentions one of the variables.
fn mentions_any(term: &ToolTerm, names: &BTreeSet<String>) -> bool {
    names.iter().any(|name| uses(term, name))
}

/// Whether a handler call takes, as its subject, a value derived from the
/// tainted variables: one of them, or a field of one.
fn derived(argument: &ToolTerm, tainted: &BTreeSet<String>) -> bool {
    match argument {
        ToolTerm::Var { name } => tainted.contains(name),
        ToolTerm::Field { value, .. } => derived(value, tainted),
        _ => false,
    }
}

/// Whether the term calls one of `handlers` on a value derived from the
/// `tainted` variables. A value is derived when it is such a variable, a name
/// a `let` binds to a term that mentions one, a branch binder of a `match`
/// on such a term, or a parameter of a lambda handed, with such a term, to the
/// same call (a fold over `alts` passes the elements of `alts` to its
/// function). Names are bound once along a path (the checker refuses
/// shadowing), so the sets need no scoping beyond being passed down.
fn reaches(term: &ToolTerm, tainted: &BTreeSet<String>, handlers: &BTreeSet<&str>) -> bool {
    let through = |arguments: &[ToolTerm]| -> bool {
        let carries = arguments
            .iter()
            .any(|a| !matches!(a, ToolTerm::Lambda { .. }) && mentions_any(a, tainted));
        arguments.iter().any(|argument| match argument {
            ToolTerm::Lambda { parameters, body } if carries => {
                let mut inner = tainted.clone();
                inner.extend(parameters.iter().map(|p| p.name.clone()));
                reaches(body, &inner, handlers)
            }
            other => reaches(other, tainted, handlers),
        })
    };
    match term {
        ToolTerm::Var { .. }
        | ToolTerm::Nat { .. }
        | ToolTerm::String { .. }
        | ToolTerm::Char { .. }
        | ToolTerm::Name { .. } => false,
        ToolTerm::Call {
            function,
            arguments,
            ..
        } => {
            (handlers.contains(function.as_str())
                && arguments
                    .first()
                    .is_some_and(|first| derived(first, tainted)))
                || through(arguments)
        }
        ToolTerm::Construct { arguments, .. } => through(arguments),
        ToolTerm::Field { value, .. } => reaches(value, tainted, handlers),
        ToolTerm::List { elements, .. } | ToolTerm::Array { elements, .. } => elements
            .iter()
            .any(|element| reaches(element, tainted, handlers)),
        ToolTerm::Let {
            binder,
            value,
            body,
            ..
        } => {
            let mut inner = tainted.clone();
            if mentions_any(value, tainted) {
                inner.insert(binder.clone());
            }
            reaches(value, tainted, handlers) || reaches(body, &inner, handlers)
        }
        ToolTerm::Lambda { body, .. } => reaches(body, tainted, handlers),
        ToolTerm::Apply {
            function,
            arguments,
        } => {
            let carries = arguments.iter().any(|a| mentions_any(a, tainted));
            let function_reaches = match function.as_ref() {
                ToolTerm::Lambda { parameters, body } if carries => {
                    let mut inner = tainted.clone();
                    inner.extend(parameters.iter().map(|p| p.name.clone()));
                    reaches(body, &inner, handlers)
                }
                other => reaches(other, tainted, handlers),
            };
            function_reaches || arguments.iter().any(|a| reaches(a, tainted, handlers))
        }
        ToolTerm::If {
            condition,
            then_branch,
            else_branch,
        } => {
            reaches(condition, tainted, handlers)
                || reaches(then_branch, tainted, handlers)
                || reaches(else_branch, tainted, handlers)
        }
        ToolTerm::Match {
            scrutinee,
            branches,
        } => {
            reaches(scrutinee, tainted, handlers)
                || branches.iter().any(|branch| {
                    let mut inner = tainted.clone();
                    if mentions_any(scrutinee, tainted) {
                        inner.extend(branch.binders.iter().cloned());
                    }
                    reaches(&branch.body, &inner, handlers)
                })
        }
    }
}

/// Whether `name` occurs free in a term.
fn uses(term: &ToolTerm, name: &str) -> bool {
    match term {
        ToolTerm::Var { name: variable } => variable == name,
        ToolTerm::Nat { .. }
        | ToolTerm::String { .. }
        | ToolTerm::Char { .. }
        | ToolTerm::Name { .. } => false,
        ToolTerm::Call { arguments, .. } | ToolTerm::Construct { arguments, .. } => {
            arguments.iter().any(|argument| uses(argument, name))
        }
        ToolTerm::Field { value, .. } => uses(value, name),
        ToolTerm::List { elements, .. } | ToolTerm::Array { elements, .. } => {
            elements.iter().any(|element| uses(element, name))
        }
        ToolTerm::Let {
            binder,
            value,
            body,
            ..
        } => uses(value, name) || (binder != name && uses(body, name)),
        ToolTerm::Lambda { parameters, body } => {
            !parameters.iter().any(|p| p.name == name) && uses(body, name)
        }
        ToolTerm::Apply {
            function,
            arguments,
        } => uses(function, name) || arguments.iter().any(|argument| uses(argument, name)),
        ToolTerm::If {
            condition,
            then_branch,
            else_branch,
        } => uses(condition, name) || uses(then_branch, name) || uses(else_branch, name),
        ToolTerm::Match {
            scrutinee,
            branches,
        } => {
            uses(scrutinee, name)
                || branches.iter().any(|branch| {
                    !branch.binders.iter().any(|b| b == name) && uses(&branch.body, name)
                })
        }
    }
}

/// The complete-traversal obligations (SPEC.md §17.20).
fn check_traversals(module: &ToolingModule, registry: &Registry<'_>) -> Result<(), ToolingFailure> {
    let functions: Vec<&ToolDeclaration> = module
        .declarations
        .iter()
        .filter(|declaration| {
            matches!(
                declaration,
                ToolDeclaration::Function {
                    traversal: true,
                    ..
                }
            )
        })
        .collect();
    // The subject type and group of each traversal, by function.
    let mut subjects: BTreeMap<&str, (&str, Option<&str>)> = BTreeMap::new();
    for declaration in &functions {
        if let ToolDeclaration::Function {
            name,
            parameters,
            mutual,
            ..
        } = declaration
        {
            if let Some(ToolType::Const {
                name: subject_type, ..
            }) = parameters.first().map(|p| &p.ty)
            {
                if let Some((key, _)) = registry.types.get_key_value(subject_type.as_str()) {
                    subjects.insert(name, (key, mutual.as_deref()));
                }
            }
        }
    }
    for declaration in &functions {
        let ToolDeclaration::Function {
            name,
            parameters,
            mutual,
            partial,
            body,
            ..
        } = declaration
        else {
            continue;
        };
        if !partial {
            return Err(ToolingFailure::check(format!(
                "`{name}` is a traversal and so recurses: it is `partial`"
            )));
        }
        let subject = parameters.first().ok_or_else(|| {
            ToolingFailure::check(format!("the traversal `{name}` has a subject parameter"))
        })?;
        let ToolType::Const {
            name: subject_type, ..
        } = &subject.ty
        else {
            return Err(ToolingFailure::check(format!(
                "the subject of the traversal `{name}` is a value of a registered type"
            )));
        };
        let (key, _) = registry
            .types
            .get_key_value(subject_type.as_str())
            .ok_or_else(|| {
                ToolingFailure::check(format!(
                "the subject of the traversal `{name}` is of the unregistered type `{subject_type}`"
            ))
            })?;
        let ToolTerm::Match {
            scrutinee,
            branches,
        } = body
        else {
            return Err(ToolingFailure::check(format!(
                "the body of the traversal `{name}` is a match on its subject"
            )));
        };
        if !matches!(scrutinee.as_ref(), ToolTerm::Var { name: variable } if *variable == subject.name)
        {
            return Err(ToolingFailure::check(format!(
                "the body of the traversal `{name}` is a match on its subject `{}`",
                subject.name
            )));
        }
        if branches.iter().any(|branch| branch.constructor.is_none()) {
            return Err(ToolingFailure::check(format!(
                "the traversal `{name}` has a default branch, which would skip constructors the registry may add"
            )));
        }
        let members = family(registry, subject_type);
        let info = registry.type_info(subject_type)?;
        for branch in branches {
            let Some(full) = &branch.constructor else {
                continue;
            };
            let short = full.rsplit_once('.').map_or(full.as_str(), |(_, s)| s);
            let ctor = info
                .constructors
                .iter()
                .find(|ctor| ctor.name == short)
                .expect("checked by the match");
            for (binder, field) in branch.binders.iter().zip(&ctor.fields) {
                let mut found = BTreeSet::new();
                mentioned(&field.ty, &mut found);
                let targets: Vec<&&str> = found
                    .iter()
                    .filter(|candidate| members.contains(**candidate))
                    .collect();
                if targets.is_empty() {
                    continue;
                }
                if !uses(&branch.body, binder) {
                    return Err(ToolingFailure::check(format!(
                        "the traversal `{name}` skips the field `{}` of `{full}`, which lies in the recursive family of `{subject_type}` (it reaches `{}`)",
                        field.name, targets[0]
                    )));
                }
                // Reaching a member of the family is recursing through its
                // handler: a call, in the branch, of a traversal of that type
                // in this function's group (or this function itself) on a
                // value derived from the field. A handler applied to the
                // subject or to a constant would leave the field unvisited
                // while still mentioning it.
                let mut tainted: BTreeSet<String> = BTreeSet::new();
                tainted.insert(binder.clone());
                for target in targets {
                    let handlers: BTreeSet<&str> = subjects
                        .iter()
                        .filter(|(callee, (callee_type, callee_group))| {
                            callee_type == target
                                && (**callee == name.as_str()
                                    || (callee_group.is_some()
                                        && *callee_group == mutual.as_deref()))
                        })
                        .map(|(callee, _)| *callee)
                        .collect();
                    let handled = reaches(&branch.body, &tainted, &handlers);
                    if !handled {
                        return Err(ToolingFailure::check(format!(
                            "the traversal `{name}` does not recurse into `{target}` for the field `{}` of `{full}`: its branch calls no traversal of `{target}` in the group on a value derived from the field",
                            field.name
                        )));
                    }
                }
            }
        }
        let _ = key;
    }
    // Every other type of a family has a handler in the same group.
    for (name, (subject_type, group)) in &subjects {
        for member in family(registry, subject_type) {
            if member == *subject_type {
                continue;
            }
            let handled = subjects.values().any(|(other_type, other_group)| {
                *other_type == member && other_group == group && group.is_some()
            });
            if !handled {
                return Err(ToolingFailure::check(format!(
                    "the traversal `{name}` of `{subject_type}` has no handler in its mutual group for `{member}`, which lies in the same recursive family"
                )));
            }
        }
    }
    Ok(())
}

fn pin(name: &str, file: &str, sha256: &str) -> Result<(), ToolingFailure> {
    if file.is_empty() || file.starts_with('/') || file.contains("..") {
        return Err(ToolingFailure::registry(format!(
            "`{name}`: the source file `{file}` is not a path within the toolchain source"
        )));
    }
    if !is_sha256(sha256) {
        return Err(ToolingFailure::registry(format!(
            "`{name}`: the source digest is not a lowercase SHA-256"
        )));
    }
    Ok(())
}

fn check_type(
    registry: &Registry<'_>,
    name: &str,
    parameters: &[TypeParameter],
    constructors: &[ToolConstructor],
    alias: Option<&ToolType>,
) -> Result<(), ToolingFailure> {
    if let Some(target) = alias {
        if !constructors.is_empty() || !parameters.is_empty() {
            return Err(ToolingFailure::registry(format!(
                "`{name}` is an abbreviation, so it has no parameters and no constructors"
            )));
        }
        registry
            .well_formed(target, &BTreeSet::new(), true)
            .map_err(|failure| ToolingFailure::registry(failure.reason))?;
        // An abbreviation unfolds in finitely many steps.
        let mut cursor = target.clone();
        for _ in 0..64 {
            let next = registry.unfold(&cursor);
            if next == cursor {
                // An abbreviation of itself unfolds to itself, which a fixed
                // point alone does not tell from a type with nothing left to
                // unfold.
                if registry.mentions_alias(&cursor) {
                    return Err(ToolingFailure::registry(format!(
                        "`{name}` abbreviates itself"
                    )));
                }
                return Ok(());
            }
            cursor = next;
        }
        return Err(ToolingFailure::registry(format!(
            "`{name}` abbreviates a type that does not unfold in finitely many steps"
        )));
    }
    let mut scope: BTreeSet<&str> = BTreeSet::new();
    for parameter in parameters {
        require_ident("type parameter", &parameter.name)?;
        if !scope.insert(&parameter.name) {
            return Err(ToolingFailure::registry(format!(
                "`{name}` repeats the parameter `{}`",
                parameter.name
            )));
        }
        if let ParameterKind::Index { of } = &parameter.kind {
            let info = registry.type_info(of)?;
            if info.constructors.iter().any(|ctor| !ctor.fields.is_empty()) {
                return Err(ToolingFailure::registry(format!(
                    "`{name}`: `{of}` has a constructor with fields, so it is not an index type"
                )));
            }
        }
    }
    let mut ctor_names: BTreeSet<&str> = BTreeSet::new();
    for ctor in constructors {
        if !is_ident(&ctor.name) || ctor.name.contains('.') {
            return Err(ToolingFailure::registry(format!(
                "`{name}`: `{}` is not a constructor name",
                ctor.name
            )));
        }
        if !ctor_names.insert(&ctor.name) {
            return Err(ToolingFailure::registry(format!(
                "`{name}` has the constructor `{}` twice",
                ctor.name
            )));
        }
        let mut fields: BTreeSet<&str> = BTreeSet::new();
        for field in &ctor.fields {
            require_ident("field", &field.name)?;
            if !fields.insert(&field.name) {
                return Err(ToolingFailure::registry(format!(
                    "`{name}.{}` has the field `{}` twice",
                    ctor.name, field.name
                )));
            }
            registry
                .well_formed(&field.ty, &scope, true)
                .map_err(|failure| ToolingFailure::registry(failure.reason))?;
        }
        if let Some(requires) = &ctor.requires {
            let parameter = parameters
                .iter()
                .find(|parameter| parameter.name == requires.parameter)
                .ok_or_else(|| {
                    ToolingFailure::registry(format!(
                        "`{name}.{}` requires the unknown parameter `{}`",
                        ctor.name, requires.parameter
                    ))
                })?;
            let ParameterKind::Index { of } = &parameter.kind else {
                return Err(ToolingFailure::registry(format!(
                    "`{name}.{}` requires `{}`, which is not an index",
                    ctor.name, requires.parameter
                )));
            };
            let (owner, _, target) = registry.constructor(&requires.constructor)?;
            if owner != of || !target.fields.is_empty() {
                return Err(ToolingFailure::registry(format!(
                    "`{name}.{}` requires `{}`, which is not a nullary constructor of `{of}`",
                    ctor.name, requires.constructor
                )));
            }
        }
    }
    Ok(())
}

/// Every name a signature uses is the module's own: a registered type, call,
/// constructor, plumbing constant, or a variable it binds.
fn check_sig(
    registry: &Registry<'_>,
    sig: &Sig,
    bound: &mut Vec<String>,
    context: &str,
) -> Result<(), ToolingFailure> {
    match sig {
        Sig::Const { name } => {
            let known = registry.types.contains_key(name.as_str())
                || registry.plumbing.contains(name.as_str())
                || registry.constructor(name).is_ok()
                || registry.call_names.contains(name.as_str());
            if !known {
                return Err(ToolingFailure::registry(format!(
                    "the signature of `{context}` names `{name}`, which is not registered"
                )));
            }
            Ok(())
        }
        Sig::Var { name } => {
            if bound.iter().any(|b| b == name) {
                Ok(())
            } else {
                Err(ToolingFailure::registry(format!(
                    "the signature of `{context}` uses the unbound variable `{name}`"
                )))
            }
        }
        Sig::App {
            function,
            arguments,
        } => {
            if arguments.is_empty() {
                return Err(ToolingFailure::registry(format!(
                    "the signature of `{context}` has an application with no argument"
                )));
            }
            check_sig(registry, function, bound, context)?;
            for argument in arguments {
                check_sig(registry, argument, bound, context)?;
            }
            Ok(())
        }
        Sig::Arrow { domain, codomain } => {
            check_domain(registry, domain, bound, context)?;
            check_sig(registry, codomain, bound, context)
        }
        Sig::Pi {
            name,
            info,
            domain,
            codomain,
        } => {
            match (info, name) {
                (BinderInfo::Instance, _) => {}
                (_, None) => {
                    return Err(ToolingFailure::registry(format!(
                        "the signature of `{context}` has an unnamed binder that is not an instance"
                    )));
                }
                _ => {}
            }
            check_domain(registry, domain, bound, context)?;
            if let Some(name) = name {
                bound.push(name.clone());
            }
            let result = check_sig(registry, codomain, bound, context);
            if name.is_some() {
                bound.pop();
            }
            result
        }
        Sig::Type { .. } | Sig::EmptyStructure => Ok(()),
        Sig::Nat { value } => {
            if value.is_empty()
                || !value.bytes().all(|b| b.is_ascii_digit())
                || (value.len() > 1 && value.starts_with('0'))
            {
                return Err(ToolingFailure::registry(format!(
                    "the signature of `{context}` has `{value}`, which is not a canonical natural number"
                )));
            }
            Ok(())
        }
        Sig::OptParam { .. } => Err(ToolingFailure::registry(format!(
            "the signature of `{context}` has `optParam` outside a binder's type"
        ))),
    }
}

fn check_domain(
    registry: &Registry<'_>,
    domain: &Sig,
    bound: &mut Vec<String>,
    context: &str,
) -> Result<(), ToolingFailure> {
    if let Sig::OptParam { domain, default } = domain {
        check_sig(registry, domain, bound, context)?;
        return check_sig(registry, default, bound, context);
    }
    check_sig(registry, domain, bound, context)
}

/// The typing a signature gives a call: implicit type binders become the
/// type parameters, explicit binders the parameters (trailing defaulted ones
/// optional), instance binders nothing (Lean synthesizes them).
fn derive(registry: &Registry<'_>, name: &str, signature: &Sig) -> Result<FnType, ToolingFailure> {
    let mut type_parameters: Vec<String> = Vec::new();
    let mut parameters: Vec<ToolType> = Vec::new();
    let mut defaulted = 0usize;
    let mut cursor = signature;
    loop {
        match cursor {
            Sig::Pi {
                name: binder,
                info,
                domain,
                codomain,
            } => {
                match info {
                    BinderInfo::Instance => {}
                    BinderInfo::Implicit if !is_kind(registry, domain) => {
                        // Lean infers an implicit argument that is not a type
                        // (the instances a structure type carries as
                        // parameters), so no use states it.
                    }
                    BinderInfo::Implicit => {
                        let binder = binder.clone().ok_or_else(|| {
                            ToolingFailure::registry(format!(
                                "`{name}` has an unnamed implicit binder"
                            ))
                        })?;
                        if !parameters.is_empty() {
                            return Err(ToolingFailure::registry(format!(
                                "`{name}`: the implicit binder `{binder}` follows an explicit one"
                            )));
                        }
                        type_parameters.push(binder);
                    }
                    BinderInfo::Explicit => {
                        let (ty, optional) = domain_type(registry, name, domain, &type_parameters)?;
                        parameters.push(ty);
                        defaulted = if optional { defaulted + 1 } else { 0 };
                    }
                }
                cursor = codomain;
            }
            Sig::Arrow { domain, codomain } => {
                let (ty, optional) = domain_type(registry, name, domain, &type_parameters)?;
                parameters.push(ty);
                defaulted = if optional { defaulted + 1 } else { 0 };
                cursor = codomain;
            }
            result => {
                let result = sig_type(registry, name, result, &type_parameters)?;
                return Ok(FnType {
                    type_parameters,
                    parameters,
                    defaulted,
                    result,
                });
            }
        }
    }
}

/// Whether a binder's domain is a kind: `Type`, a type constructor kind
/// (`Type → Type`), or a registered index type, so that the binder is a type
/// parameter that every use states.
fn is_kind(registry: &Registry<'_>, domain: &Sig) -> bool {
    match domain {
        Sig::Type { .. } => true,
        Sig::Arrow { domain, codomain } => is_kind(registry, domain) && is_kind(registry, codomain),
        Sig::Const { name } => registry.types.get(name.as_str()).is_some_and(|info| {
            !info.opaque
                && !info.constructors.is_empty()
                && info.constructors.iter().all(|ctor| ctor.fields.is_empty())
        }),
        _ => false,
    }
}

fn domain_type(
    registry: &Registry<'_>,
    name: &str,
    domain: &Sig,
    type_parameters: &[String],
) -> Result<(ToolType, bool), ToolingFailure> {
    match domain {
        Sig::OptParam { domain, .. } => {
            Ok((sig_type(registry, name, domain, type_parameters)?, true))
        }
        other => Ok((sig_type(registry, name, other, type_parameters)?, false)),
    }
}

/// A signature read as the type of a value.
fn sig_type(
    registry: &Registry<'_>,
    name: &str,
    sig: &Sig,
    type_parameters: &[String],
) -> Result<ToolType, ToolingFailure> {
    let unsupported = |what: &str| {
        ToolingFailure::registry(format!(
            "`{name}`: {what} is not a type of the tooling language"
        ))
    };
    match sig {
        Sig::Const { name: constant } => {
            if registry.types.contains_key(constant.as_str()) {
                Ok(ToolType::Const {
                    name: constant.clone(),
                    arguments: Vec::new(),
                })
            } else if let Ok((_, _, ctor)) = registry.constructor(constant) {
                if ctor.fields.is_empty() {
                    Ok(ToolType::Index {
                        constructor: constant.clone(),
                    })
                } else {
                    Err(unsupported(constant))
                }
            } else {
                Err(unsupported(constant))
            }
        }
        Sig::Var { name: variable } => {
            if type_parameters.iter().any(|p| p == variable) {
                Ok(ToolType::Parameter {
                    name: variable.clone(),
                    arguments: Vec::new(),
                })
            } else {
                Err(unsupported(&format!("the variable `{variable}`")))
            }
        }
        Sig::App {
            function,
            arguments,
        } => {
            let convert = |arguments: &[Sig]| -> Result<Vec<ToolType>, ToolingFailure> {
                arguments
                    .iter()
                    .map(|argument| sig_type(registry, name, argument, type_parameters))
                    .collect()
            };
            match sig_type(registry, name, function, type_parameters)? {
                ToolType::Const {
                    name: head_name,
                    arguments: head,
                } => {
                    // The arguments past the type's parameters are the
                    // instances Lean carries in the type and infers.
                    let wanted = registry
                        .types
                        .get(head_name.as_str())
                        .map_or(arguments.len(), |info| {
                            info.parameters.len().saturating_sub(head.len())
                        });
                    let mut all = head;
                    all.extend(convert(&arguments[..wanted.min(arguments.len())])?);
                    Ok(ToolType::Const {
                        name: head_name,
                        arguments: all,
                    })
                }
                ToolType::Parameter {
                    name: head_name,
                    arguments: head,
                } => {
                    let mut all = head;
                    all.extend(convert(arguments)?);
                    Ok(ToolType::Parameter {
                        name: head_name,
                        arguments: all,
                    })
                }
                _ => Err(unsupported("an application of a function type")),
            }
        }
        Sig::Arrow { domain, codomain } => {
            let (first, _) = domain_type(registry, name, domain, type_parameters)?;
            let mut parameters = vec![first];
            let mut cursor = codomain.as_ref();
            while let Sig::Arrow { domain, codomain } = cursor {
                parameters.push(domain_type(registry, name, domain, type_parameters)?.0);
                cursor = codomain;
            }
            Ok(ToolType::Function {
                parameters,
                result: Box::new(sig_type(registry, name, cursor, type_parameters)?),
            })
        }
        Sig::Pi { .. } => Err(unsupported("a dependent binder inside a parameter type")),
        Sig::Type { .. } | Sig::OptParam { .. } | Sig::EmptyStructure | Sig::Nat { .. } => {
            Err(unsupported("a sort or a default value"))
        }
    }
}

fn check_command(
    registry: &Registry<'_>,
    module: &ToolingModule,
    name: &str,
    atoms: &[CommandAtom],
    elaborator: &str,
) -> Result<(), ToolingFailure> {
    match atoms.first() {
        Some(CommandAtom::Token { text }) if !text.trim().is_empty() => {}
        _ => {
            return Err(ToolingFailure::check(format!(
                "the command `{name}` begins with a token that names it"
            )));
        }
    }
    for atom in atoms {
        if let CommandAtom::Token { text } = atom {
            if text.is_empty() || text.contains('"') || text.contains('\\') || text.contains('\n') {
                return Err(ToolingFailure::check(format!(
                    "the command `{name}` has a token that is not plain text"
                )));
            }
        }
    }
    let typing = registry.functions.get(elaborator).ok_or_else(|| {
        ToolingFailure::check(format!(
            "the elaborator `{elaborator}` of `{name}` is not a function of this module"
        ))
    })?;
    // Lean's `command_elab` attribute requires the declared type to be the
    // abbreviation `CommandElab` by name.
    let elab = ToolType::Const {
        name: COMMAND_ELAB.to_owned(),
        arguments: Vec::new(),
    };
    let unfolded = ToolType::Function {
        parameters: vec![ToolType::Const {
            name: SYNTAX.to_owned(),
            arguments: Vec::new(),
        }],
        result: Box::new(ToolType::Const {
            name: COMMAND_M.to_owned(),
            arguments: vec![ToolType::Const {
                name: UNIT.to_owned(),
                arguments: Vec::new(),
            }],
        }),
    };
    if !typing.type_parameters.is_empty()
        || !typing.parameters.is_empty()
        || typing.result != elab
        || registry.normal(&elab) != unfolded
    {
        return Err(ToolingFailure::check(format!(
            "the elaborator `{elaborator}` of `{name}` has no parameters and the type `{COMMAND_ELAB}`, which the registry states is `Lean.Syntax → {COMMAND_M} Unit`"
        )));
    }
    let public = module.declarations.iter().any(|declaration| {
        matches!(declaration, ToolDeclaration::Function { name: function, public: true, .. } if function == elaborator)
    });
    if !public {
        return Err(ToolingFailure::check(format!(
            "the elaborator `{elaborator}` of `{name}` is a public function"
        )));
    }
    Ok(())
}

fn check_invoke(
    registry: &Registry<'_>,
    command: &str,
    atoms: &[CommandAtom],
    arguments: &[InvokeArgument],
) -> Result<(), ToolingFailure> {
    let wanted: Vec<&CommandAtom> = atoms
        .iter()
        .filter(|atom| !matches!(atom, CommandAtom::Token { .. }))
        .collect();
    if wanted.len() != arguments.len() {
        return Err(ToolingFailure::check(format!(
            "`{command}` takes {} arguments, {} are given",
            wanted.len(),
            arguments.len()
        )));
    }
    for (atom, argument) in wanted.iter().zip(arguments) {
        match (atom, argument) {
            (CommandAtom::Ident, InvokeArgument::Ident { value }) => {
                require_ident("argument", value)?;
            }
            (CommandAtom::Type, InvokeArgument::Type { value }) => {
                check_sig(registry, value, &mut Vec::new(), command)?;
            }
            _ => {
                return Err(ToolingFailure::check(format!(
                    "an argument of `{command}` does not match the command's grammar"
                )));
            }
        }
    }
    Ok(())
}

/// The scope of a term's check.
struct Env<'r, 'a> {
    registry: &'r Registry<'a>,
    scope: &'r BTreeSet<&'r str>,
    locals: BTreeMap<String, ToolType>,
    defined: &'r BTreeSet<&'r str>,
    group: &'r BTreeSet<&'r str>,
    current: &'r str,
    partial: bool,
    calls: BTreeSet<String>,
}

impl Env<'_, '_> {
    fn well_formed(&self, ty: &ToolType) -> Result<(), ToolingFailure> {
        self.registry.well_formed(ty, self.scope, true)
    }

    fn named(&self, name: &str) -> ToolType {
        ToolType::Const {
            name: name.to_owned(),
            arguments: Vec::new(),
        }
    }

    /// A new local. A name is bound once along any path, so that what a name
    /// stands for is read from its one binding: the complete-traversal check
    /// follows a field through the names derived from it.
    fn bind(&mut self, name: &str, ty: ToolType) -> Result<(), ToolingFailure> {
        require_ident("variable", name)?;
        if self.locals.insert(name.to_owned(), ty).is_some() {
            return Err(ToolingFailure::check(format!(
                "`{name}` shadows a variable in scope"
            )));
        }
        Ok(())
    }

    fn expect(
        &self,
        what: &str,
        expected: &ToolType,
        observed: &ToolType,
    ) -> Result<(), ToolingFailure> {
        if expected == observed || self.registry.normal(expected) == self.registry.normal(observed)
        {
            Ok(())
        } else {
            Err(ToolingFailure::check(format!(
                "{what} has type {}, expected {}",
                show_type(observed),
                show_type(expected)
            )))
        }
    }

    fn expect_arguments(
        &mut self,
        callee: &str,
        parameters: &[ToolType],
        defaulted: usize,
        arguments: &[ToolTerm],
    ) -> Result<(), ToolingFailure> {
        if arguments.len() > parameters.len() || arguments.len() + defaulted < parameters.len() {
            return Err(ToolingFailure::check(format!(
                "{callee} takes {} arguments, {} are given",
                parameters.len() - defaulted,
                arguments.len()
            )));
        }
        for (index, (argument, parameter)) in arguments.iter().zip(parameters).enumerate() {
            let observed = self.infer(argument)?;
            self.expect(
                &format!("argument {} of {callee}", index + 1),
                parameter,
                &observed,
            )?;
        }
        Ok(())
    }

    fn infer(&mut self, term: &ToolTerm) -> Result<ToolType, ToolingFailure> {
        match term {
            ToolTerm::Var { name } => self.locals.get(name).cloned().ok_or_else(|| {
                ToolingFailure::check(format!("the variable `{name}` is not in scope"))
            }),
            ToolTerm::Nat { value } => {
                if value.is_empty()
                    || !value.bytes().all(|b| b.is_ascii_digit())
                    || (value.len() > 1 && value.starts_with('0'))
                {
                    return Err(ToolingFailure::check(format!(
                        "`{value}` is not a canonical natural number"
                    )));
                }
                self.literal_type(NAT)
            }
            ToolTerm::String { .. } => self.literal_type(STRING),
            ToolTerm::Char { value } => {
                if char::from_u32(*value).is_none() {
                    return Err(ToolingFailure::check(format!(
                        "{value} is not a Unicode scalar value"
                    )));
                }
                self.literal_type(CHAR)
            }
            ToolTerm::Name { value } => {
                require_ident("name literal", value)?;
                self.literal_type(NAME)
            }
            ToolTerm::Call {
                function,
                type_arguments,
                arguments,
            } => self.call(function, type_arguments, arguments),
            ToolTerm::Construct {
                constructor,
                type_arguments,
                arguments,
            } => {
                let (owner, info, ctor) = self.registry.constructor(constructor)?;
                if type_arguments.len() != info.parameters.len() {
                    return Err(ToolingFailure::check(format!(
                        "`{constructor}` takes {} type arguments, {} are given",
                        info.parameters.len(),
                        type_arguments.len()
                    )));
                }
                for (argument, parameter) in type_arguments.iter().zip(info.parameters) {
                    match &parameter.kind {
                        ParameterKind::Constructor => {
                            self.registry.well_formed(argument, self.scope, false)?;
                        }
                        ParameterKind::Index { .. } => {
                            self.registry.well_formed(argument, self.scope, false)?;
                        }
                        ParameterKind::Type => self.well_formed(argument)?,
                    }
                }
                if let Some(requires) = &ctor.requires {
                    let position = info
                        .parameters
                        .iter()
                        .position(|p| p.name == requires.parameter)
                        .expect("checked at registration");
                    let index = ToolType::Index {
                        constructor: requires.constructor.clone(),
                    };
                    if type_arguments[position] != index {
                        return Err(ToolingFailure::check(format!(
                            "`{constructor}` exists only at the index `{}`",
                            requires.constructor
                        )));
                    }
                }
                let map: BTreeMap<&str, ToolType> = info
                    .parameters
                    .iter()
                    .map(|p| p.name.as_str())
                    .zip(type_arguments.iter().cloned())
                    .collect();
                if arguments.len() != ctor.fields.len() {
                    return Err(ToolingFailure::check(format!(
                        "`{constructor}` takes {} fields, {} are given",
                        ctor.fields.len(),
                        arguments.len()
                    )));
                }
                for (field, argument) in ctor.fields.iter().zip(arguments) {
                    let observed = self.infer(argument)?;
                    self.expect(
                        &format!("field `{}` of `{constructor}`", field.name),
                        &subst(&field.ty, &map),
                        &observed,
                    )?;
                }
                Ok(ToolType::Const {
                    name: owner.to_owned(),
                    arguments: type_arguments.clone(),
                })
            }
            ToolTerm::Field { value, field } => {
                let observed = self.infer(value)?;
                let ToolType::Const { name, arguments } = &observed else {
                    return Err(ToolingFailure::check(format!(
                        "the field `{field}` is read from a value of type {}, which is not a registered type",
                        show_type(&observed)
                    )));
                };
                let info = self.registry.type_info(name)?;
                if info.opaque {
                    return Err(ToolingFailure::check(format!(
                        "`{name}` is opaque: the field `{field}` cannot be read"
                    )));
                }
                let [ctor] = info.constructors else {
                    return Err(ToolingFailure::check(format!(
                        "`{name}` has {} constructors, so it has no field `{field}`",
                        info.constructors.len()
                    )));
                };
                let declared = ctor
                    .fields
                    .iter()
                    .find(|candidate| &candidate.name == field)
                    .ok_or_else(|| {
                        ToolingFailure::check(format!("`{name}` has no field `{field}`"))
                    })?;
                let map: BTreeMap<&str, ToolType> = info
                    .parameters
                    .iter()
                    .map(|p| p.name.as_str())
                    .zip(arguments.iter().cloned())
                    .collect();
                Ok(subst(&declared.ty, &map))
            }
            ToolTerm::List { element, elements } => self.literal_sequence(LIST, element, elements),
            ToolTerm::Array { element, elements } => {
                self.literal_sequence(ARRAY, element, elements)
            }
            ToolTerm::Let {
                binder,
                ty,
                value,
                body,
            } => {
                self.well_formed(ty)?;
                let observed = self.infer(value)?;
                self.expect(&format!("the value of `{binder}`"), ty, &observed)?;
                self.bind(binder, ty.clone())?;
                let result = self.infer(body);
                self.locals.remove(binder);
                result
            }
            ToolTerm::Lambda { parameters, body } => {
                if parameters.is_empty() {
                    return Err(ToolingFailure::check("a lambda has a parameter"));
                }
                let saved = self.locals.clone();
                for ToolParameter { name, ty } in parameters {
                    self.well_formed(ty)?;
                    self.bind(name, ty.clone())?;
                }
                let result = self.infer(body);
                self.locals = saved;
                Ok(ToolType::Function {
                    parameters: parameters.iter().map(|p| p.ty.clone()).collect(),
                    result: Box::new(result?),
                })
            }
            ToolTerm::Apply {
                function,
                arguments,
            } => {
                let observed = self.infer(function)?;
                let observed = self.registry.normal(&observed);
                let ToolType::Function { parameters, result } = observed else {
                    return Err(ToolingFailure::check(format!(
                        "a value of type {} is applied, and it is not a function",
                        show_type(&observed)
                    )));
                };
                self.expect_arguments("the applied function", &parameters, 0, arguments)?;
                Ok(*result)
            }
            ToolTerm::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let observed = self.infer(condition)?;
                self.expect("the condition", &self.named(BOOL), &observed)?;
                let then_type = self.infer(then_branch)?;
                let else_type = self.infer(else_branch)?;
                self.expect("the else branch", &then_type, &else_type)?;
                Ok(then_type)
            }
            ToolTerm::Match {
                scrutinee,
                branches,
            } => self.match_term(scrutinee, branches),
        }
    }

    fn literal_type(&self, name: &str) -> Result<ToolType, ToolingFailure> {
        self.registry.type_info(name).map_err(|_| {
            ToolingFailure::check(format!(
                "a literal of type `{name}` needs `{name}` among the registered types"
            ))
        })?;
        Ok(self.named(name))
    }

    fn literal_sequence(
        &mut self,
        constructor: &str,
        element: &ToolType,
        elements: &[ToolTerm],
    ) -> Result<ToolType, ToolingFailure> {
        self.registry.type_info(constructor).map_err(|_| {
            ToolingFailure::check(format!(
                "a literal needs `{constructor}` among the registered types"
            ))
        })?;
        self.well_formed(element)?;
        for (index, item) in elements.iter().enumerate() {
            let observed = self.infer(item)?;
            self.expect(&format!("element {}", index + 1), element, &observed)?;
        }
        Ok(ToolType::Const {
            name: constructor.to_owned(),
            arguments: vec![element.clone()],
        })
    }

    fn call(
        &mut self,
        function: &str,
        type_arguments: &[ToolType],
        arguments: &[ToolTerm],
    ) -> Result<ToolType, ToolingFailure> {
        let typing = if let Some(typing) = self.registry.calls.get(function) {
            typing
        } else if let Some(typing) = self.registry.functions.get(function) {
            let allowed = self.defined.contains(function)
                || (function == self.current && self.partial)
                || self.group.contains(function);
            if !allowed {
                return Err(ToolingFailure::check(format!(
                    "`{function}` is not defined before this use, and is neither this function (when partial) nor in its mutual group"
                )));
            }
            self.calls.insert(function.to_owned());
            typing
        } else {
            return Err(ToolingFailure::check(format!(
                "`{function}` is neither a registered call nor a function of this module"
            )));
        };
        if type_arguments.len() != typing.type_parameters.len() {
            return Err(ToolingFailure::check(format!(
                "`{function}` takes {} type arguments, {} are given",
                typing.type_parameters.len(),
                type_arguments.len()
            )));
        }
        for argument in type_arguments {
            self.registry.well_formed(argument, self.scope, false)?;
        }
        let map: BTreeMap<&str, ToolType> = typing
            .type_parameters
            .iter()
            .map(String::as_str)
            .zip(type_arguments.iter().cloned())
            .collect();
        let parameters: Vec<ToolType> = typing.parameters.iter().map(|p| subst(p, &map)).collect();
        let result = subst(&typing.result, &map);
        let defaulted = typing.defaulted;
        self.expect_arguments(&format!("`{function}`"), &parameters, defaulted, arguments)?;
        Ok(result)
    }

    fn match_term(
        &mut self,
        scrutinee: &ToolTerm,
        branches: &[crate::ir::tooling::ToolBranch],
    ) -> Result<ToolType, ToolingFailure> {
        let observed = self.infer(scrutinee)?;
        let ToolType::Const { name, arguments } = &observed else {
            return Err(ToolingFailure::check(format!(
                "a match is on a value of type {}, which is not a registered type",
                show_type(&observed)
            )));
        };
        let info = self.registry.type_info(name)?.clone();
        if info.opaque {
            return Err(ToolingFailure::check(format!(
                "`{name}` is opaque: it cannot be matched"
            )));
        }
        if branches.is_empty() {
            return Err(ToolingFailure::check("a match has a branch"));
        }
        let map: BTreeMap<&str, ToolType> = info
            .parameters
            .iter()
            .map(|p| p.name.as_str())
            .zip(arguments.iter().cloned())
            .collect();
        // The constructors available at the scrutinee's index.
        let available: Vec<&ToolConstructor> = info
            .constructors
            .iter()
            .filter(|ctor| {
                ctor.requires.as_ref().is_none_or(|requires| {
                    let position = info
                        .parameters
                        .iter()
                        .position(|p| p.name == requires.parameter)
                        .expect("checked at registration");
                    match &arguments[position] {
                        ToolType::Index { constructor } => *constructor == requires.constructor,
                        _ => true,
                    }
                })
            })
            .collect();
        let mut covered: BTreeSet<&str> = BTreeSet::new();
        let mut default_seen = false;
        let mut result: Option<ToolType> = None;
        for (index, branch) in branches.iter().enumerate() {
            if default_seen {
                return Err(ToolingFailure::check(
                    "a branch follows the default branch of a match",
                ));
            }
            let saved = self.locals.clone();
            match &branch.constructor {
                None => {
                    if !branch.binders.is_empty() {
                        return Err(ToolingFailure::check("the default branch binds nothing"));
                    }
                    if index == 0 && branches.len() == 1 {
                        return Err(ToolingFailure::check(
                            "a match of only a default branch is not a case analysis",
                        ));
                    }
                    default_seen = true;
                }
                Some(full) => {
                    let prefix = format!("{name}.");
                    let short = full.strip_prefix(&prefix).ok_or_else(|| {
                        ToolingFailure::check(format!("`{full}` is not a constructor of `{name}`"))
                    })?;
                    let ctor = available
                        .iter()
                        .find(|ctor| ctor.name == short)
                        .copied()
                        .ok_or_else(|| {
                            ToolingFailure::check(format!(
                                "`{full}` is not a constructor of `{name}` at its index"
                            ))
                        })?;
                    if !covered.insert(&ctor.name) {
                        return Err(ToolingFailure::check(format!("`{full}` is matched twice")));
                    }
                    if branch.binders.len() != ctor.fields.len() {
                        return Err(ToolingFailure::check(format!(
                            "`{full}` has {} fields, {} names are bound",
                            ctor.fields.len(),
                            branch.binders.len()
                        )));
                    }
                    let mut names: BTreeSet<&str> = BTreeSet::new();
                    for (binder, field) in branch.binders.iter().zip(&ctor.fields) {
                        if !names.insert(binder) {
                            return Err(ToolingFailure::check(format!(
                                "`{binder}` is bound twice in one branch"
                            )));
                        }
                        self.bind(binder, subst(&field.ty, &map))?;
                    }
                }
            }
            let body_type = self.infer(&branch.body);
            self.locals = saved;
            let body_type = body_type?;
            match &result {
                None => result = Some(body_type),
                Some(first) => self.expect("a branch", first, &body_type)?,
            }
        }
        if !default_seen {
            let missing: Vec<&str> = available
                .iter()
                .map(|ctor| ctor.name.as_str())
                .filter(|short| !covered.contains(short))
                .collect();
            if !missing.is_empty() {
                return Err(ToolingFailure::check(format!(
                    "the match on `{name}` is not exhaustive: it omits {}",
                    missing.join(", ")
                )));
            }
        }
        Ok(result.expect("a match has a branch"))
    }
}
