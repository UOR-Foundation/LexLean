//! Closed compiler-tooling modules (SPEC.md §17.20).
//!
//! A tooling module is semantic data: a typed registry of the authority
//! interface a compiler tool consumes (types with exact constructor lists,
//! calls with exact structured signatures), typed functions over it, and the
//! registration of entry points. Both backends are generated from this one
//! linked value. There is no member that holds Lean text: a signature is a
//! tree, a command is a list of grammar atoms, a call names a registry row.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::diagnostic::DiagnosticCode;

/// The schema discriminator of a tooling module (SPEC.md §17.20).
pub const TOOLING_SPEC: &str = "lexlean/tooling-module/1";

/// A failed decode or check: the registered code and the reason.
#[derive(Debug, Clone)]
pub struct ToolingFailure {
    /// The registered code.
    pub code: DiagnosticCode,
    /// The reason, naming the construct.
    pub reason: String,
}

impl ToolingFailure {
    pub(crate) fn schema(reason: impl Into<String>) -> Self {
        Self {
            code: crate::code!("LLT4013"),
            reason: reason.into(),
        }
    }

    pub(crate) fn registry(reason: impl Into<String>) -> Self {
        Self {
            code: crate::code!("LLT4014"),
            reason: reason.into(),
        }
    }

    pub(crate) fn check(reason: impl Into<String>) -> Self {
        Self {
            code: crate::code!("LLT4015"),
            reason: reason.into(),
        }
    }
}

/// The external authority the registry cites: Lean at one pinned revision.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolAuthority {
    /// The authority row, `LEAN-LCNF-4-32-1`.
    pub id: String,
    /// The Lean version.
    pub lean_version: String,
    /// The Lean source commit.
    pub lean_githash: String,
}

/// The file of the toolchain source that defines a registry row, and its
/// SHA-256.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SourcePin {
    /// The path within the toolchain source.
    pub file: String,
    /// Lowercase hexadecimal SHA-256.
    pub sha256: String,
}

/// A universe level in a registered signature.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum SigLevel {
    /// A universe parameter, such as `u_1`.
    Parameter {
        /// The parameter name.
        name: String,
    },
}

/// How a signature binder is supplied (SPEC.md §17.20).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BinderInfo {
    /// A binder the caller supplies.
    Explicit,
    /// A binder Lean infers: `{x : T}`.
    Implicit,
    /// A binder Lean synthesizes by instance search: `[C x]`.
    Instance,
}

/// A registered signature: the exact type of a Lean constant, as a tree.
/// Binder kinds, defaults and universes are part of it, so that a drift in
/// any of them is a difference the probe sees.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum Sig {
    /// A constant: a registered type or the name of a sort-valued constant.
    Const {
        /// The constant.
        name: String,
    },
    /// A bound variable.
    Var {
        /// The variable.
        name: String,
    },
    /// An application.
    App {
        /// The head.
        function: Box<Sig>,
        /// The arguments, at least one.
        arguments: Vec<Sig>,
    },
    /// A non-dependent arrow.
    Arrow {
        /// The domain.
        domain: Box<Sig>,
        /// The codomain.
        codomain: Box<Sig>,
    },
    /// A dependent binder.
    Pi {
        /// The binder name, absent for an instance binder.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        /// How the binder is supplied.
        info: BinderInfo,
        /// The binder's type.
        domain: Box<Sig>,
        /// The rest.
        codomain: Box<Sig>,
    },
    /// `Type` (no level) or `Type u`.
    Type {
        /// The universe parameter, when there is one.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        level: Option<SigLevel>,
    },
    /// `optParam T d`: an explicit binder with a default.
    OptParam {
        /// The binder's type.
        domain: Box<Sig>,
        /// The default.
        default: Box<Sig>,
    },
    /// `{ }`: the structure instance whose fields all take their defaults.
    EmptyStructure,
    /// A natural number: a default value written as a literal, in decimal
    /// digits without a leading zero.
    Nat {
        /// The decimal digits.
        value: String,
    },
}

/// The kind of a type parameter of a registered type.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum ParameterKind {
    /// A type: `(α : Type)`.
    Type,
    /// A type constructor of one argument: `(m : Type → Type)`.
    Constructor,
    /// An index drawn from a registered type's nullary constructors:
    /// `(pu : Purity)`.
    Index {
        /// The registered type.
        of: String,
    },
}

/// A type parameter of a registered type.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TypeParameter {
    /// The name, scoped to the declaration.
    pub name: String,
    /// The kind.
    pub kind: ParameterKind,
}

/// A type of the tooling language.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum ToolType {
    /// A registered type, applied to its arguments. Applied to fewer than
    /// its parameters it is a type constructor (`CoreM`).
    Const {
        /// The registered type.
        name: String,
        /// The arguments.
        arguments: Vec<ToolType>,
    },
    /// A type variable in scope, applied when it is a type constructor.
    Parameter {
        /// The variable.
        name: String,
        /// The arguments, empty for a plain type.
        arguments: Vec<ToolType>,
    },
    /// A function type.
    Function {
        /// The parameter types.
        parameters: Vec<ToolType>,
        /// The result.
        result: Box<ToolType>,
    },
    /// An index value: a nullary constructor of an index type, `Purity.pure`.
    Index {
        /// The full constructor name.
        constructor: String,
    },
}

/// The index value that makes a constructor available.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IndexRequirement {
    /// The type parameter that carries the index.
    pub parameter: String,
    /// The nullary constructor it must equal.
    pub constructor: String,
}

/// A field of a constructor.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolField {
    /// The field name.
    pub name: String,
    /// The field type.
    #[serde(rename = "type")]
    pub ty: ToolType,
}

/// A constructor of a registered type.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolConstructor {
    /// The constructor's own name, in Lean's order within the type.
    pub name: String,
    /// The fields.
    pub fields: Vec<ToolField>,
    /// The index value that makes it available, when it is not available at
    /// every index.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requires: Option<IndexRequirement>,
}

/// A binder of a function or lambda.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolParameter {
    /// The name.
    pub name: String,
    /// The type.
    #[serde(rename = "type")]
    pub ty: ToolType,
}

/// A branch of a `match`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolBranch {
    /// The constructor's full name, absent for the default branch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constructor: Option<String>,
    /// The names bound to the fields, in order; empty for the default.
    pub binders: Vec<String>,
    /// The branch body.
    pub body: ToolTerm,
}

/// A term of the tooling language.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum ToolTerm {
    /// A variable in scope.
    Var {
        /// The variable.
        name: String,
    },
    /// A natural number.
    Nat {
        /// The decimal digits.
        value: String,
    },
    /// A string.
    String {
        /// The text.
        value: String,
    },
    /// A character, by scalar value.
    Char {
        /// The Unicode scalar value.
        value: u32,
    },
    /// A name literal, resolved when Lean elaborates it: ` ``Lean.Name `.
    Name {
        /// The dotted name.
        value: String,
    },
    /// A call of a registered call or of a function of this module.
    Call {
        /// The callee.
        function: String,
        /// The type arguments, for the callee's implicit type parameters in
        /// order.
        type_arguments: Vec<ToolType>,
        /// The explicit arguments.
        arguments: Vec<ToolTerm>,
    },
    /// A constructor of a registered type.
    Construct {
        /// The full constructor name.
        constructor: String,
        /// The type arguments, for the type's parameters in order.
        type_arguments: Vec<ToolType>,
        /// The fields.
        arguments: Vec<ToolTerm>,
    },
    /// A field of a value of a registered type with one constructor.
    Field {
        /// The value.
        value: Box<ToolTerm>,
        /// The field.
        field: String,
    },
    /// A list.
    List {
        /// The element type.
        element: ToolType,
        /// The elements.
        elements: Vec<ToolTerm>,
    },
    /// An array.
    Array {
        /// The element type.
        element: ToolType,
        /// The elements.
        elements: Vec<ToolTerm>,
    },
    /// A local definition.
    Let {
        /// The name.
        binder: String,
        /// The declared type.
        #[serde(rename = "type")]
        ty: ToolType,
        /// The value.
        value: Box<ToolTerm>,
        /// The body.
        body: Box<ToolTerm>,
    },
    /// A function.
    Lambda {
        /// The parameters.
        parameters: Vec<ToolParameter>,
        /// The body.
        body: Box<ToolTerm>,
    },
    /// An application of a function value.
    Apply {
        /// The function.
        function: Box<ToolTerm>,
        /// The arguments.
        arguments: Vec<ToolTerm>,
    },
    /// A conditional on a `Bool`.
    If {
        /// The condition.
        condition: Box<ToolTerm>,
        /// The branch taken when it holds.
        then_branch: Box<ToolTerm>,
        /// The branch taken when it does not.
        else_branch: Box<ToolTerm>,
    },
    /// An exhaustive case analysis over the constructors of a registered
    /// type, flat, one constructor per branch.
    Match {
        /// The scrutinee.
        scrutinee: Box<ToolTerm>,
        /// The branches.
        branches: Vec<ToolBranch>,
    },
}

/// One atom of a registered command's grammar.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum CommandAtom {
    /// A fixed token.
    Token {
        /// The token text.
        text: String,
    },
    /// An identifier.
    Ident,
    /// A type written as a registered signature.
    Type,
}

/// An argument of an invocation of a registered command.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum InvokeArgument {
    /// An identifier.
    Ident {
        /// The dotted name.
        value: String,
    },
    /// A signature.
    Type {
        /// The signature tree.
        value: Sig,
    },
}

/// A declaration of a tooling module.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum ToolDeclaration {
    /// A type of the authority interface, with its complete constructor list.
    Type {
        /// The full name.
        name: String,
        /// The parameters.
        parameters: Vec<TypeParameter>,
        /// The constructors, in Lean's order.
        constructors: Vec<ToolConstructor>,
        /// Whether the tool never matches, constructs, or projects this
        /// type, so that its constructors are not pinned (`constructors` is
        /// then empty). Every type a registered type's fields mention is
        /// registered, as opaque when that is all the tool needs of it.
        #[serde(default, skip_serializing_if = "is_false")]
        opaque: bool,
        /// The type this one abbreviates, when it is a Lean `abbrev`
        /// (`CommandElab`): a use of either is a use of the other, and an
        /// abbreviation has no constructors. Lean elaborates the generated
        /// text against its own abbreviation, so a registry that misstates
        /// it is refused there.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        alias: Option<ToolType>,
        /// The defining source file and its digest.
        source: SourcePin,
    },
    /// A function of the authority interface, with its exact signature.
    Call {
        /// The full name.
        name: String,
        /// The exact signature.
        signature: Sig,
        /// What the tool uses it for.
        role: String,
        /// The defining source file and its digest.
        source: SourcePin,
    },
    /// Constants the elaborator inserts that no term names: instances,
    /// matchers, recursors, projections, and core-library data.
    Plumbing {
        /// The full names, sorted.
        names: Vec<String>,
    },
    /// A function of the tool.
    Function {
        /// The full name.
        name: String,
        /// Whether other modules may use it.
        public: bool,
        /// Whether it may recurse without a termination argument.
        partial: bool,
        /// The group of mutually recursive functions it belongs to.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        mutual: Option<String>,
        /// Whether it is a traversal of its first parameter's type family
        /// (SPEC.md §17.20): an exhaustive match whose every branch uses
        /// each field that lies in the family, with a handler in its group
        /// for every other type of the family.
        #[serde(default, skip_serializing_if = "is_false")]
        traversal: bool,
        /// The type parameters.
        type_parameters: Vec<String>,
        /// The parameters.
        parameters: Vec<ToolParameter>,
        /// The result type.
        result: ToolType,
        /// The body.
        body: ToolTerm,
    },
    /// The registration of a command: its grammar and its elaborator.
    Command {
        /// The syntax-kind name.
        name: String,
        /// The grammar.
        atoms: Vec<CommandAtom>,
        /// The function that elaborates it, of type
        /// `Lean.Elab.Command.CommandElab`.
        elaborator: String,
    },
    /// An invocation of a registered command.
    Invoke {
        /// The command's syntax-kind name.
        command: String,
        /// The arguments, one per non-token atom.
        arguments: Vec<InvokeArgument>,
    },
    /// An evaluation of a function of the tool.
    Evaluate {
        /// The function.
        function: String,
        /// The arguments.
        arguments: Vec<ToolTerm>,
    },
}

impl ToolDeclaration {
    /// The kind name, as in the source.
    #[must_use]
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Type { .. } => "type",
            Self::Call { .. } => "call",
            Self::Plumbing { .. } => "plumbing",
            Self::Function { .. } => "function",
            Self::Command { .. } => "command",
            Self::Invoke { .. } => "invoke",
            Self::Evaluate { .. } => "evaluate",
        }
    }
}

fn is_false(value: &bool) -> bool {
    !*value
}

/// A linked tooling module.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolingModule {
    /// `lexlean/tooling-module/1`.
    pub spec: String,
    /// The authority the registry cites.
    pub authority: ToolAuthority,
    /// Generated project modules imported in full (`import all`), sorted.
    pub imports: Vec<String>,
    /// The declarations, in order: a name is defined before it is used.
    pub declarations: Vec<ToolDeclaration>,
}

/// What the compiler pins the authority to.
#[derive(Debug, Clone, Copy)]
pub struct AuthorityPin<'a> {
    /// The pinned Lean version.
    pub lean_version: &'a str,
    /// The pinned Lean source commit.
    pub lean_githash: &'a str,
}

impl ToolingModule {
    /// Decode canonical JSON and enforce the closed schema and every check
    /// of SPEC.md §17.20.
    ///
    /// # Errors
    ///
    /// `LLT4013` for a malformed, noncanonical, or out-of-schema module,
    /// `LLT4014` for an invalid authority registry, `LLT4015` for an
    /// ill-typed, non-exhaustive, unlicensed-recursion, or incomplete
    /// traversal.
    pub fn parse(
        text: &str,
        language: &str,
        pin: AuthorityPin<'_>,
    ) -> Result<Self, ToolingFailure> {
        if language != crate::LANGUAGE_1_2 {
            return Err(ToolingFailure::schema(
                "a tooling module requires language 1.2",
            ));
        }
        let module: Self = serde_json::from_str(text).map_err(|error| {
            ToolingFailure::schema(format!("invalid tooling-module JSON: {error}"))
        })?;
        let canonical = crate::artifact::canonical_json::Json::parse(text.as_bytes())
            .map_err(ToolingFailure::schema)?
            .to_canonical_string();
        if canonical != text {
            return Err(ToolingFailure::schema(
                "tooling-module JSON is not canonical",
            ));
        }
        // Serde skips unknown members of a tagged variant's payload only when
        // `deny_unknown_fields` is absent; it is present everywhere, and the
        // comparison below also closes a member that deserialization could
        // drop silently (a written default).
        let source: serde_json::Value = serde_json::from_str(text).map_err(|error| {
            ToolingFailure::schema(format!("invalid tooling-module JSON: {error}"))
        })?;
        let typed = serde_json::to_value(&module).expect("tooling module serializes");
        if let Some(path) = ignored_member(&source, &typed, "$") {
            return Err(ToolingFailure::schema(format!(
                "tooling-module JSON member `{path}` is outside the closed schema"
            )));
        }
        module.validate(pin)?;
        Ok(module)
    }

    fn validate(&self, pin: AuthorityPin<'_>) -> Result<(), ToolingFailure> {
        if self.spec != TOOLING_SPEC {
            return Err(ToolingFailure::schema(format!(
                "the schema is `{}`, not `{TOOLING_SPEC}`",
                self.spec
            )));
        }
        if self.authority.lean_version != pin.lean_version
            || self.authority.lean_githash != pin.lean_githash
        {
            return Err(ToolingFailure::registry(format!(
                "the authority `{}` pins Lean {} ({}), the compiler's pin is {} ({})",
                self.authority.id,
                self.authority.lean_version,
                self.authority.lean_githash,
                pin.lean_version,
                pin.lean_githash
            )));
        }
        let mut sorted = self.imports.clone();
        sorted.sort();
        sorted.dedup();
        if sorted != self.imports {
            return Err(ToolingFailure::schema(
                "imports are not sorted and distinct",
            ));
        }
        crate::ir::tooling_check::check(self)
    }

    /// The number of IR nodes this module contributes to `max_ir_nodes`.
    #[must_use]
    pub fn node_count(&self) -> u64 {
        fn ty(node: &ToolType) -> u64 {
            1 + match node {
                ToolType::Const { arguments, .. } | ToolType::Parameter { arguments, .. } => {
                    arguments.iter().map(ty).sum::<u64>()
                }
                ToolType::Function { parameters, result } => {
                    parameters.iter().map(ty).sum::<u64>() + ty(result)
                }
                ToolType::Index { .. } => 0,
            }
        }
        fn sig(node: &Sig) -> u64 {
            1 + match node {
                Sig::App {
                    function,
                    arguments,
                } => sig(function) + arguments.iter().map(sig).sum::<u64>(),
                Sig::Arrow { domain, codomain }
                | Sig::Pi {
                    domain, codomain, ..
                } => sig(domain) + sig(codomain),
                Sig::OptParam { domain, default } => sig(domain) + sig(default),
                Sig::Const { .. }
                | Sig::Var { .. }
                | Sig::Type { .. }
                | Sig::EmptyStructure
                | Sig::Nat { .. } => 0,
            }
        }
        fn term(node: &ToolTerm) -> u64 {
            1 + match node {
                ToolTerm::Var { .. }
                | ToolTerm::Nat { .. }
                | ToolTerm::String { .. }
                | ToolTerm::Char { .. }
                | ToolTerm::Name { .. } => 0,
                ToolTerm::Call {
                    type_arguments,
                    arguments,
                    ..
                }
                | ToolTerm::Construct {
                    type_arguments,
                    arguments,
                    ..
                } => {
                    type_arguments.iter().map(ty).sum::<u64>()
                        + arguments.iter().map(term).sum::<u64>()
                }
                ToolTerm::Field { value, .. } => term(value),
                ToolTerm::List { element, elements } | ToolTerm::Array { element, elements } => {
                    ty(element) + elements.iter().map(term).sum::<u64>()
                }
                ToolTerm::Let {
                    ty: declared,
                    value,
                    body,
                    ..
                } => ty(declared) + term(value) + term(body),
                ToolTerm::Lambda { parameters, body } => {
                    parameters.iter().map(|p| ty(&p.ty)).sum::<u64>() + term(body)
                }
                ToolTerm::Apply {
                    function,
                    arguments,
                } => term(function) + arguments.iter().map(term).sum::<u64>(),
                ToolTerm::If {
                    condition,
                    then_branch,
                    else_branch,
                } => term(condition) + term(then_branch) + term(else_branch),
                ToolTerm::Match {
                    scrutinee,
                    branches,
                } => term(scrutinee) + branches.iter().map(|b| 1 + term(&b.body)).sum::<u64>(),
            }
        }
        self.declarations
            .iter()
            .map(|declaration| {
                1 + match declaration {
                    ToolDeclaration::Type { constructors, .. } => constructors
                        .iter()
                        .map(|c| 1 + c.fields.iter().map(|f| ty(&f.ty)).sum::<u64>())
                        .sum::<u64>(),
                    ToolDeclaration::Call { signature, .. } => sig(signature),
                    ToolDeclaration::Plumbing { names } => names.len() as u64,
                    ToolDeclaration::Function {
                        parameters,
                        result,
                        body,
                        ..
                    } => {
                        parameters.iter().map(|p| ty(&p.ty)).sum::<u64>() + ty(result) + term(body)
                    }
                    ToolDeclaration::Command { atoms, .. } => atoms.len() as u64,
                    ToolDeclaration::Invoke { arguments, .. } => arguments
                        .iter()
                        .map(|a| match a {
                            InvokeArgument::Ident { .. } => 1,
                            InvokeArgument::Type { value } => sig(value),
                        })
                        .sum::<u64>(),
                    ToolDeclaration::Evaluate { arguments, .. } => {
                        arguments.iter().map(term).sum::<u64>()
                    }
                }
            })
            .sum()
    }

    /// The registered types, by full name.
    #[must_use]
    pub fn types(&self) -> BTreeMap<&str, &ToolDeclaration> {
        self.declarations
            .iter()
            .filter_map(|declaration| match declaration {
                ToolDeclaration::Type { name, .. } => Some((name.as_str(), declaration)),
                _ => None,
            })
            .collect()
    }

    /// Every declaration name this module defines.
    #[must_use]
    pub fn defined_names(&self) -> BTreeSet<&str> {
        self.declarations
            .iter()
            .filter_map(|declaration| match declaration {
                ToolDeclaration::Type { name, .. }
                | ToolDeclaration::Call { name, .. }
                | ToolDeclaration::Function { name, .. }
                | ToolDeclaration::Command { name, .. } => Some(name.as_str()),
                _ => None,
            })
            .collect()
    }
}

/// The first member of `source` that did not survive into `typed`, as a
/// path. A member that serialization omits as an explicit default is not
/// one; `tooling` has no such member.
fn ignored_member(
    source: &serde_json::Value,
    typed: &serde_json::Value,
    path: &str,
) -> Option<String> {
    use serde_json::Value;
    match (source, typed) {
        (Value::Object(source), Value::Object(typed)) => {
            for (key, value) in source {
                match typed.get(key) {
                    Some(inner) => {
                        if let Some(found) = ignored_member(value, inner, &format!("{path}.{key}"))
                        {
                            return Some(found);
                        }
                    }
                    None => return Some(format!("{path}.{key}")),
                }
            }
            None
        }
        (Value::Array(source), Value::Array(typed)) => {
            if source.len() != typed.len() {
                return Some(format!("{path}[]"));
            }
            for (index, (value, inner)) in source.iter().zip(typed).enumerate() {
                if let Some(found) = ignored_member(value, inner, &format!("{path}[{index}]")) {
                    return Some(found);
                }
            }
            None
        }
        _ => None,
    }
}
