//! The source side of semantic preservation (SPEC.md §17.17): the closed
//! types and constructors of a production root's runtime closure, read from
//! the linked semantic IR exactly as the eligibility analysis reads it.
//!
//! Lowering ([`super::lower`]) and the preservation certificate
//! ([`super::certificate`]) both walk the source through this module, so the
//! two agree on what a source term *is* --- its closed type, the declaration
//! a reference names, the constructor a pattern selects --- while each
//! decides independently what that term becomes. Every type it returns is
//! closed: type parameters are substituted and every document reference
//! carries its module.

// A `match` naming both `Some` and `None` is used where `if let` and
// `while let` would hide the second case; the audit forbids those forms here.
#![allow(
    clippy::single_match,
    clippy::single_match_else,
    clippy::while_let_loop
)]
// The compiler is the second line of defence behind the audit: a binding
// catch-all over an enum (`other =>`) is a default too.
#![deny(
    clippy::wildcard_enum_match_arm,
    clippy::match_wildcard_for_single_variants
)]

use std::collections::BTreeMap;

use super::eligibility::{
    anchor, anchor_member, substitute, type_text, BuiltinOwner, LinkedModule, Owner, BUILTIN_OWNERS,
};
use crate::ir::semantic::{
    MemberRef, SemanticBranch, SemanticConstructor, SemanticDeclaration, SemanticField,
    SemanticParameter, SemanticType,
};

/// Where a term is read: its module and the instantiation of the type
/// parameters of the declaration it belongs to.
#[derive(Clone)]
pub(crate) struct Site {
    pub(crate) module: String,
    pub(crate) substitution: BTreeMap<String, SemanticType>,
}

/// One local in scope: its source name and closed type.
#[derive(Clone)]
pub(crate) struct Local {
    pub(crate) name: String,
    pub(crate) ty: SemanticType,
}

/// A constructor a term builds or a branch selects.
#[derive(Clone)]
pub(crate) enum Constructor {
    /// `Bool.true` or `Bool.false`.
    Bool(bool),
    /// `Nat.zero`.
    Zero,
    /// `Nat.succ`.
    Succ,
    /// `List.nil`.
    Nil,
    /// `List.cons`.
    Cons,
    /// `Option.none`.
    OptionNone,
    /// `Option.some`.
    OptionSome,
    /// `Result.ok`.
    Ok,
    /// `Result.error`.
    Error,
    /// A `ContractViolation` constructor, as the pair of Booleans it lowers
    /// to (§17.12 rule 9).
    Violation(bool, bool),
    /// The constructor at `index` of a document inductive.
    Document {
        /// The closed document type the constructor builds.
        ty: SemanticType,
        /// Its position in declaration order.
        index: usize,
    },
}

/// The shape of a closed document type: its constructors' closed field
/// types, in declaration order. A structure or class has exactly one.
pub(crate) struct DocumentShape {
    /// The kind of declaration: `inductive`, `structure`, or `class`.
    pub(crate) kind: &'static str,
    /// Constructor names in declaration order (a structure's is `mk`).
    pub(crate) names: Vec<String>,
    /// Closed field types per constructor.
    pub(crate) fields: Vec<Vec<SemanticType>>,
    /// Field names of a structure or class, in declaration order.
    pub(crate) field_names: Vec<String>,
}

/// The linked modules of one project, read as a closed source.
pub(crate) struct Source<'a> {
    pub(crate) modules: &'a BTreeMap<String, LinkedModule<'a>>,
}

fn internal(reason: impl Into<String>) -> String {
    reason.into()
}

impl<'a> Source<'a> {
    /// The declaration `name` of `module`.
    pub(crate) fn declaration(&self, module: &str, name: &str) -> Option<&'a SemanticDeclaration> {
        // Every backend reads a module's elaborated declarations: each model
        // declaration as the ordinary definitions it means, and every
        // checked application as its elaboration (§17.12).
        self.modules.get(module).and_then(|linked| {
            linked
                .semantic
                .lowered_declarations()
                .into_iter()
                .find(|declaration| declaration.name() == name)
        })
    }

    /// The generated Lean module of a source module.
    pub(crate) fn lean_module(&self, module: &str) -> Result<&'a str, String> {
        self.modules
            .get(module)
            .map(|linked| linked.lean_module)
            .ok_or_else(|| internal(format!("module `{module}` is not linked")))
    }

    /// The fully qualified Lean name of a member read in `module`.
    pub(crate) fn lean_name(&self, module: &str, member: &MemberRef) -> String {
        Owner {
            module,
            modules: self.modules,
        }
        .lean_name(member)
    }

    /// The canonical report spelling of a closed type.
    pub(crate) fn type_text(&self, ty: &SemanticType) -> String {
        type_text(
            ty,
            &Owner {
                module: "",
                modules: self.modules,
            },
        )
    }

    /// Close a type written at `site`: substitute the instantiation, then
    /// anchor every local reference in the site's module.
    pub(crate) fn close(&self, ty: &SemanticType, site: &Site) -> SemanticType {
        anchor(&substitute(ty, &site.substitution), &site.module)
    }

    /// The module a reference read at `site` names.
    pub(crate) fn module_of(member: &MemberRef, site: &Site) -> String {
        member.module.clone().unwrap_or_else(|| site.module.clone())
    }

    /// The instantiation of `type_parameters` by closed `arguments`.
    pub(crate) fn instantiation(
        type_parameters: &[String],
        arguments: &[SemanticType],
    ) -> Result<BTreeMap<String, SemanticType>, String> {
        if type_parameters.len() != arguments.len() {
            return Err(internal(format!(
                "{} type parameter(s) instantiated with {} argument(s)",
                type_parameters.len(),
                arguments.len()
            )));
        }
        Ok(type_parameters
            .iter()
            .cloned()
            .zip(arguments.iter().cloned())
            .collect())
    }

    /// The signature of a definition instance: its closed parameter types
    /// and result.
    pub(crate) fn signature(
        &self,
        module: &str,
        name: &str,
        arguments: &[SemanticType],
    ) -> Result<(Vec<SemanticParameter>, SemanticType), String> {
        match self.declaration(module, name) {
            Some(SemanticDeclaration::Definition {
                name: _,
                type_parameters,
                parameters,
                result,
                recursive_argument: _,
                body: _,
                axioms: _,
                executable: _,
                mutual: _,
                termination: _,
                production: _,
            }) => {
                let site = Site {
                    module: module.to_owned(),
                    substitution: Self::instantiation(type_parameters, arguments)?,
                };
                Ok((
                    parameters
                        .iter()
                        .map(|parameter| SemanticParameter {
                            name: parameter.name.clone(),
                            r#type: self.close(&parameter.r#type, &site),
                        })
                        .collect(),
                    self.close(result, &site),
                ))
            }
            Some(
                SemanticDeclaration::Structure {
                    name: _,
                    type_parameters: _,
                    parameters: _,
                    fields: _,
                }
                | SemanticDeclaration::Class {
                    name: _,
                    type_parameters: _,
                    parameters: _,
                    fields: _,
                }
                | SemanticDeclaration::Instance {
                    name: _,
                    class: _,
                    arguments: _,
                    priority: _,
                    fields: _,
                }
                | SemanticDeclaration::Inductive {
                    name: _,
                    type_parameters: _,
                    parameters: _,
                    constructors: _,
                    mutual: _,
                }
                | SemanticDeclaration::Theorem {
                    name: _,
                    type_parameters: _,
                    parameters: _,
                    statement: _,
                    proof: _,
                    axioms: _,
                }
                | SemanticDeclaration::Artifact {
                    name: _,
                    role: _,
                    sha256: _,
                    length: _,
                    schema: _,
                    r#type: _,
                    axioms: _,
                }
                | SemanticDeclaration::Contract {
                    name: _,
                    type_parameters: _,
                    input: _,
                    output: _,
                    state: _,
                    precondition: _,
                    postcondition: _,
                    invariant: _,
                    validators: _,
                    axioms: _,
                }
                | SemanticDeclaration::Realization {
                    name: _,
                    type_parameters: _,
                    input: _,
                    output: _,
                    state: _,
                    descriptor: _,
                    executable: _,
                    axioms: _,
                }
                | SemanticDeclaration::Evidence {
                    name: _,
                    type_parameters: _,
                    contract: _,
                    realization: _,
                    claims: _,
                    axioms: _,
                }
                | SemanticDeclaration::Model {
                    name: _,
                    type_parameters: _,
                    contract: _,
                    realization: _,
                    evidence: _,
                    entry: _,
                    axioms: _,
                }
                | SemanticDeclaration::Logic {
                    name: _,
                    type_parameters: _,
                    state: _,
                    relation: _,
                    invariant: _,
                    ranking: _,
                    axioms: _,
                }
                | SemanticDeclaration::InferenceRule {
                    name: _,
                    type_parameters: _,
                    logic: _,
                    binding: _,
                    guard: _,
                    conclusion: _,
                    soundness: _,
                    progress: _,
                    executable: _,
                    axioms: _,
                }
                | SemanticDeclaration::Verifier {
                    name: _,
                    type_parameters: _,
                    subject: _,
                    candidate: _,
                    specification: _,
                    check: _,
                    sound: _,
                    complete: _,
                    axioms: _,
                }
                | SemanticDeclaration::Reasoner {
                    name: _,
                    type_parameters: _,
                    logic: _,
                    observation: _,
                    observe: _,
                    rules: _,
                    strategy: _,
                    answer: _,
                    verifier: _,
                    claims: _,
                    executable: _,
                    axioms: _,
                },
            )
            | None => Err(internal(format!(
                "`{module}.{name}` is not a linked definition"
            ))),
        }
    }

    /// The constructors and closed field types of a closed document type.
    pub(crate) fn document(&self, ty: &SemanticType) -> Result<DocumentShape, String> {
        let (member, arguments) = match ty {
            SemanticType::Named { member, arguments } => (member, arguments),
            SemanticType::Type
            | SemanticType::Parameter { name: _ }
            | SemanticType::Nat
            | SemanticType::Bool
            | SemanticType::Prop
            | SemanticType::Unit
            | SemanticType::Int
            | SemanticType::Int8
            | SemanticType::Int16
            | SemanticType::Int32
            | SemanticType::Int64
            | SemanticType::UInt8
            | SemanticType::UInt16
            | SemanticType::UInt32
            | SemanticType::UInt64
            | SemanticType::String
            | SemanticType::Bytes
            | SemanticType::Ordering
            | SemanticType::Option { value: _ }
            | SemanticType::Result { ok: _, error: _ }
            | SemanticType::List { element: _ }
            | SemanticType::Product { left: _, right: _ }
            | SemanticType::Function {
                parameters: _,
                result: _,
            }
            | SemanticType::Map { key: _, value: _ }
            | SemanticType::Set { element: _ }
            | SemanticType::ContractViolation
            | SemanticType::ReasoningFailure => {
                return Err(internal("a document shape of a non-document type"));
            }
        };
        let module = member
            .module
            .clone()
            .ok_or_else(|| internal("an unanchored document type"))?;
        let structure = |kind: &'static str,
                         type_parameters: &[String],
                         fields: &[SemanticField]|
         -> Result<DocumentShape, String> {
            let site = Site {
                module: module.clone(),
                substitution: Self::instantiation(type_parameters, arguments)?,
            };
            Ok(DocumentShape {
                kind,
                names: vec!["mk".to_owned()],
                fields: vec![fields
                    .iter()
                    .map(|SemanticField { name: _, r#type }| self.close(r#type, &site))
                    .collect()],
                field_names: fields
                    .iter()
                    .map(|SemanticField { name, r#type: _ }| name.clone())
                    .collect(),
            })
        };
        match self.declaration(&module, &member.name) {
            Some(SemanticDeclaration::Structure {
                name: _,
                type_parameters,
                parameters: _,
                fields,
            }) => structure("structure", type_parameters, fields),
            Some(SemanticDeclaration::Class {
                name: _,
                type_parameters,
                parameters: _,
                fields,
            }) => structure("class", type_parameters, fields),
            Some(SemanticDeclaration::Inductive {
                name: _,
                type_parameters,
                parameters: _,
                constructors,
                mutual: _,
            }) => {
                let site = Site {
                    module: module.clone(),
                    substitution: Self::instantiation(type_parameters, arguments)?,
                };
                Ok(DocumentShape {
                    kind: "inductive",
                    names: constructors
                        .iter()
                        .map(|SemanticConstructor { name, fields: _ }| name.clone())
                        .collect(),
                    fields: constructors
                        .iter()
                        .map(|SemanticConstructor { name: _, fields }| {
                            fields
                                .iter()
                                .map(|field| self.close(field, &site))
                                .collect()
                        })
                        .collect(),
                    field_names: Vec::new(),
                })
            }
            Some(
                SemanticDeclaration::Instance {
                    name: _,
                    class: _,
                    arguments: _,
                    priority: _,
                    fields: _,
                }
                | SemanticDeclaration::Definition {
                    name: _,
                    type_parameters: _,
                    parameters: _,
                    result: _,
                    recursive_argument: _,
                    body: _,
                    axioms: _,
                    executable: _,
                    mutual: _,
                    termination: _,
                    production: _,
                }
                | SemanticDeclaration::Theorem {
                    name: _,
                    type_parameters: _,
                    parameters: _,
                    statement: _,
                    proof: _,
                    axioms: _,
                }
                | SemanticDeclaration::Artifact {
                    name: _,
                    role: _,
                    sha256: _,
                    length: _,
                    schema: _,
                    r#type: _,
                    axioms: _,
                }
                | SemanticDeclaration::Contract {
                    name: _,
                    type_parameters: _,
                    input: _,
                    output: _,
                    state: _,
                    precondition: _,
                    postcondition: _,
                    invariant: _,
                    validators: _,
                    axioms: _,
                }
                | SemanticDeclaration::Realization {
                    name: _,
                    type_parameters: _,
                    input: _,
                    output: _,
                    state: _,
                    descriptor: _,
                    executable: _,
                    axioms: _,
                }
                | SemanticDeclaration::Evidence {
                    name: _,
                    type_parameters: _,
                    contract: _,
                    realization: _,
                    claims: _,
                    axioms: _,
                }
                | SemanticDeclaration::Model {
                    name: _,
                    type_parameters: _,
                    contract: _,
                    realization: _,
                    evidence: _,
                    entry: _,
                    axioms: _,
                }
                | SemanticDeclaration::Logic {
                    name: _,
                    type_parameters: _,
                    state: _,
                    relation: _,
                    invariant: _,
                    ranking: _,
                    axioms: _,
                }
                | SemanticDeclaration::InferenceRule {
                    name: _,
                    type_parameters: _,
                    logic: _,
                    binding: _,
                    guard: _,
                    conclusion: _,
                    soundness: _,
                    progress: _,
                    executable: _,
                    axioms: _,
                }
                | SemanticDeclaration::Verifier {
                    name: _,
                    type_parameters: _,
                    subject: _,
                    candidate: _,
                    specification: _,
                    check: _,
                    sound: _,
                    complete: _,
                    axioms: _,
                }
                | SemanticDeclaration::Reasoner {
                    name: _,
                    type_parameters: _,
                    logic: _,
                    observation: _,
                    observe: _,
                    rules: _,
                    strategy: _,
                    answer: _,
                    verifier: _,
                    claims: _,
                    executable: _,
                    axioms: _,
                },
            )
            | None => Err(internal(format!(
                "`{module}.{}` is not a linked data type",
                member.name
            ))),
        }
    }

    /// The constructor a term or branch names at `site`, with the closed
    /// type it builds. `type_arguments` are a constructor term's explicit
    /// arguments; a branch passes the scrutinee's type instead.
    pub(crate) fn constructor(
        &self,
        constructor: &MemberRef,
        type_arguments: &[SemanticType],
        site: &Site,
    ) -> Result<(Constructor, SemanticType), String> {
        let (owner, local) = constructor
            .name
            .rsplit_once('.')
            .ok_or_else(|| internal(format!("constructor `{}` names no type", constructor.name)))?;
        let builtin = match &constructor.module {
            Some(_) => None,
            None => BUILTIN_OWNERS
                .iter()
                .find(|(name, _)| *name == owner)
                .map(|(_, builtin)| *builtin),
        };
        match builtin {
            Some(builtin) => {
                if type_arguments.len() != builtin.arity() {
                    return Err(internal(format!(
                        "built-in constructor `{}` has {} type argument(s)",
                        constructor.name,
                        type_arguments.len()
                    )));
                }
                let argument = |index: usize| Box::new(type_arguments[index].clone());
                let ty = match builtin {
                    BuiltinOwner::Bool => SemanticType::Bool,
                    BuiltinOwner::ContractViolation => SemanticType::ContractViolation,
                    BuiltinOwner::ReasoningFailure => SemanticType::ReasoningFailure,
                    BuiltinOwner::Nat => SemanticType::Nat,
                    BuiltinOwner::List => SemanticType::List {
                        element: argument(0),
                    },
                    BuiltinOwner::Option => SemanticType::Option { value: argument(0) },
                    BuiltinOwner::Result => SemanticType::Result {
                        ok: argument(0),
                        error: argument(1),
                    },
                };
                let kind = match (builtin, local) {
                    (BuiltinOwner::Bool, "true") => Constructor::Bool(true),
                    (BuiltinOwner::Bool, "false") => Constructor::Bool(false),
                    (BuiltinOwner::Nat, "zero") => Constructor::Zero,
                    (BuiltinOwner::Nat, "succ") => Constructor::Succ,
                    (BuiltinOwner::List, "nil") => Constructor::Nil,
                    (BuiltinOwner::List, "cons") => Constructor::Cons,
                    (BuiltinOwner::Option, "none") => Constructor::OptionNone,
                    (BuiltinOwner::Option, "some") => Constructor::OptionSome,
                    (BuiltinOwner::Result, "ok") => Constructor::Ok,
                    (BuiltinOwner::Result, "error") => Constructor::Error,
                    (BuiltinOwner::ContractViolation, "precondition") => {
                        Constructor::Violation(false, false)
                    }
                    (BuiltinOwner::ContractViolation, "input_invariant") => {
                        Constructor::Violation(false, true)
                    }
                    (BuiltinOwner::ContractViolation, "postcondition") => {
                        Constructor::Violation(true, false)
                    }
                    (BuiltinOwner::ContractViolation, "output_invariant") => {
                        Constructor::Violation(true, true)
                    }
                    // `(answered, replay)` (§17.12, reasoning).
                    (BuiltinOwner::ReasoningFailure, "exhausted") => {
                        Constructor::Violation(false, false)
                    }
                    (BuiltinOwner::ReasoningFailure, "unsolved") => {
                        Constructor::Violation(false, true)
                    }
                    (BuiltinOwner::ReasoningFailure, "rejected") => {
                        Constructor::Violation(true, false)
                    }
                    (BuiltinOwner::ReasoningFailure, "invalid_step") => {
                        Constructor::Violation(true, true)
                    }
                    (
                        BuiltinOwner::Bool
                        | BuiltinOwner::ContractViolation
                        | BuiltinOwner::ReasoningFailure
                        | BuiltinOwner::Nat
                        | BuiltinOwner::List
                        | BuiltinOwner::Option
                        | BuiltinOwner::Result,
                        unknown,
                    ) => {
                        return Err(internal(format!(
                            "built-in type `{owner}` has no constructor `{unknown}`"
                        )));
                    }
                };
                Ok((kind, ty))
            }
            None => {
                let ty = SemanticType::Named {
                    member: anchor_member(
                        &MemberRef {
                            module: constructor.module.clone(),
                            name: owner.to_owned(),
                        },
                        &site.module,
                    ),
                    arguments: type_arguments.to_vec(),
                };
                let shape = self.document(&ty)?;
                let index = shape
                    .names
                    .iter()
                    .position(|name| name == local)
                    .ok_or_else(|| internal(format!("`{owner}` has no constructor `{local}`")))?;
                Ok((
                    Constructor::Document {
                        ty: ty.clone(),
                        index,
                    },
                    ty,
                ))
            }
        }
    }

    /// The constructor a branch selects from a scrutinee of closed type
    /// `scrutinee`, and the closed types of the values it binds.
    pub(crate) fn branch(
        &self,
        branch: &SemanticBranch,
        scrutinee: &SemanticType,
        site: &Site,
    ) -> Result<(Constructor, Vec<SemanticType>), String> {
        let arguments: Vec<SemanticType> = match scrutinee {
            SemanticType::List { element: inner } | SemanticType::Option { value: inner } => {
                vec![inner.as_ref().clone()]
            }
            SemanticType::Result { ok, error } => vec![ok.as_ref().clone(), error.as_ref().clone()],
            SemanticType::Named {
                member: _,
                arguments,
            } => arguments.clone(),
            SemanticType::Nat
            | SemanticType::Bool
            | SemanticType::ContractViolation
            | SemanticType::ReasoningFailure => Vec::new(),
            SemanticType::Type
            | SemanticType::Parameter { name: _ }
            | SemanticType::Prop
            | SemanticType::Unit
            | SemanticType::Int
            | SemanticType::Int8
            | SemanticType::Int16
            | SemanticType::Int32
            | SemanticType::Int64
            | SemanticType::UInt8
            | SemanticType::UInt16
            | SemanticType::UInt32
            | SemanticType::UInt64
            | SemanticType::String
            | SemanticType::Bytes
            | SemanticType::Ordering
            | SemanticType::Product { left: _, right: _ }
            | SemanticType::Function {
                parameters: _,
                result: _,
            }
            | SemanticType::Map { key: _, value: _ }
            | SemanticType::Set { element: _ } => {
                return Err(internal(format!(
                    "a match over `{}`, which has no constructors",
                    self.type_text(scrutinee)
                )));
            }
        };
        let (constructor, ty) = self.constructor(&branch.constructor, &arguments, site)?;
        if &ty != scrutinee {
            return Err(internal(format!(
                "branch `{}` does not belong to `{}`",
                branch.constructor.name,
                self.type_text(scrutinee)
            )));
        }
        let fields = self.fields(&constructor, scrutinee)?;
        Ok((constructor, fields))
    }

    /// The closed field types of a constructor of closed type `ty`.
    pub(crate) fn fields(
        &self,
        constructor: &Constructor,
        ty: &SemanticType,
    ) -> Result<Vec<SemanticType>, String> {
        let mismatch = || internal("a constructor at a foreign type");
        match constructor {
            Constructor::Bool(_)
            | Constructor::Zero
            | Constructor::Nil
            | Constructor::OptionNone
            | Constructor::Violation(_, _) => Ok(Vec::new()),
            Constructor::Succ => Ok(vec![SemanticType::Nat]),
            Constructor::Cons => match ty {
                SemanticType::List { element } => Ok(vec![element.as_ref().clone(), ty.clone()]),
                SemanticType::Type
                | SemanticType::Parameter { name: _ }
                | SemanticType::Nat
                | SemanticType::Bool
                | SemanticType::Prop
                | SemanticType::Unit
                | SemanticType::Int
                | SemanticType::Int8
                | SemanticType::Int16
                | SemanticType::Int32
                | SemanticType::Int64
                | SemanticType::UInt8
                | SemanticType::UInt16
                | SemanticType::UInt32
                | SemanticType::UInt64
                | SemanticType::String
                | SemanticType::Bytes
                | SemanticType::Ordering
                | SemanticType::Option { value: _ }
                | SemanticType::Result { ok: _, error: _ }
                | SemanticType::Named {
                    member: _,
                    arguments: _,
                }
                | SemanticType::Product { left: _, right: _ }
                | SemanticType::Function {
                    parameters: _,
                    result: _,
                }
                | SemanticType::Map { key: _, value: _ }
                | SemanticType::Set { element: _ }
                | SemanticType::ContractViolation
                | SemanticType::ReasoningFailure => Err(mismatch()),
            },
            Constructor::OptionSome | Constructor::Ok | Constructor::Error => {
                let field = match (constructor, ty) {
                    (Constructor::OptionSome, SemanticType::Option { value }) => value,
                    (Constructor::Ok, SemanticType::Result { ok, error: _ }) => ok,
                    (Constructor::Error, SemanticType::Result { ok: _, error }) => error,
                    (
                        Constructor::Bool(_)
                        | Constructor::Zero
                        | Constructor::Succ
                        | Constructor::Nil
                        | Constructor::Cons
                        | Constructor::OptionNone
                        | Constructor::OptionSome
                        | Constructor::Ok
                        | Constructor::Error
                        | Constructor::Violation(_, _)
                        | Constructor::Document { ty: _, index: _ },
                        SemanticType::Type
                        | SemanticType::Parameter { name: _ }
                        | SemanticType::Nat
                        | SemanticType::Bool
                        | SemanticType::Prop
                        | SemanticType::Unit
                        | SemanticType::Int
                        | SemanticType::Int8
                        | SemanticType::Int16
                        | SemanticType::Int32
                        | SemanticType::Int64
                        | SemanticType::UInt8
                        | SemanticType::UInt16
                        | SemanticType::UInt32
                        | SemanticType::UInt64
                        | SemanticType::String
                        | SemanticType::Bytes
                        | SemanticType::Ordering
                        | SemanticType::Option { value: _ }
                        | SemanticType::Result { ok: _, error: _ }
                        | SemanticType::List { element: _ }
                        | SemanticType::Named {
                            member: _,
                            arguments: _,
                        }
                        | SemanticType::Product { left: _, right: _ }
                        | SemanticType::Function {
                            parameters: _,
                            result: _,
                        }
                        | SemanticType::Map { key: _, value: _ }
                        | SemanticType::Set { element: _ }
                        | SemanticType::ContractViolation
                        | SemanticType::ReasoningFailure,
                    ) => return Err(mismatch()),
                };
                Ok(vec![field.as_ref().clone()])
            }
            Constructor::Document { ty: built, index } => {
                if built != ty {
                    return Err(mismatch());
                }
                let shape = self.document(built)?;
                shape
                    .fields
                    .get(*index)
                    .cloned()
                    .ok_or_else(|| internal("a constructor index out of range"))
            }
        }
    }
}
