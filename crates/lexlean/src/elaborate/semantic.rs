//! Exact conversion of successfully linked semantic export headers to prose
//! types. Bodies remain semantic IR: no opaque or glossary-supplied type is
//! substituted when an existing LSE cannot express a header.
use std::collections::BTreeMap;
use std::sync::Arc;

use crate::ir::semantic::{
    MemberRef, SemanticDeclaration, SemanticModule, SemanticParameter, SemanticTerm, SemanticType,
};
use crate::ir::term::{
    Binder, BinderMode, CoreRef, DocumentDeclRef, ExternalConstRef, GlobalRef, LocalId, Term,
    Universe,
};
use crate::lexicon::entry::{Category, Denotation};
use crate::lexicon::resolve::Closure;

use super::resolve::LocalAlloc;

#[derive(Debug, Clone)]
enum Header {
    Type,
    Definition(Box<SemanticType>),
    Theorem(Box<SemanticTerm>),
    Unavailable(&'static str),
}

/// A validated semantic declaration's source key and actual backend name.
/// The header is converted lazily: an unused declaration whose signature
/// has no prose-LSE representation must not disable its semantic module.
#[derive(Debug, Clone)]
pub struct SemanticExport {
    /// The key visible to the importing source, renamed with its imports.
    pub module: String,
    /// The exact semantic member name (never a prose component encoding).
    pub component: String,
    /// The full backend name.
    pub lean_name: String,
    own_module: String,
    modules: Arc<BTreeMap<String, String>>,
    type_parameters: Vec<String>,
    parameters: Vec<SemanticParameter>,
    header: Header,
}

impl SemanticExport {
    /// A source-visible reference keeps its logical key and actual Lean name.
    #[must_use]
    pub fn reference(&self) -> DocumentDeclRef {
        DocumentDeclRef {
            module: self.module.clone(),
            component: self.component.clone(),
            lean_name: self.lean_name.clone(),
        }
    }

    /// Semantic declaration categories are not interchangeable even when
    /// their erased signatures would happen to have the same shape.
    ///
    /// # Errors
    /// The semantic kind does not admit this glossary category.
    pub fn check_category(&self, category: Category) -> Result<(), String> {
        let accepted = match &self.header {
            Header::Type => category == Category::TypeNoun,
            Header::Theorem(_) => category == Category::ProofConstant,
            Header::Definition(result) => {
                if **result == SemanticType::Prop {
                    matches!(category, Category::PredicateConstant | Category::AdjectivePredicate | Category::IntransitivePredicate | Category::TransitivePredicate | Category::InfixPredicate)
                } else if **result == SemanticType::Type {
                    category == Category::TypeNoun
                } else {
                    matches!(category, Category::TermConstant | Category::Function | Category::PrefixFunction | Category::PostfixFunction | Category::InfixFunction | Category::NounFunction | Category::BinaryNounFunction)
                }
            }
            Header::Unavailable(kind) => return Err(format!("semantic declaration `{}::{}` has category `{kind}`; document glossaries expose inductive, structure, definition and theorem declarations", self.module, self.component)),
        };
        if accepted {
            Ok(())
        } else {
            Err(format!(
                "semantic declaration `{}::{}` cannot be used as glossary category `{}`",
                self.module,
                self.component,
                category.as_str()
            ))
        }
    }

    /// Derive the entire actual telescope independently of the glossary.
    /// Every local comes from the caller's allocator, so it cannot capture
    /// the independently converted entry or an enclosing prose binder.
    ///
    /// # Errors
    /// A category, term or required constant has no exact prose-LSE representation.
    pub fn ty(&self, closure: &Closure, alloc: &mut LocalAlloc) -> Result<Term, String> {
        if let Header::Unavailable(kind) = &self.header {
            return Err(format!(
                "semantic declaration `{}::{}` has unsupported glossary category `{kind}`",
                self.module, self.component
            ));
        }
        let mut conversion = Convert {
            export: self,
            closure,
            alloc,
            locals: BTreeMap::new(),
        };
        let mut binders = Vec::new();
        for name in &self.type_parameters {
            binders.push(conversion.bind(name, Term::Sort(Universe::Num(1))));
        }
        for parameter in &self.parameters {
            let ty = conversion.ty(&parameter.r#type)?;
            binders.push(conversion.bind(&parameter.name, ty));
        }
        let result = match &self.header {
            Header::Type => Term::Sort(Universe::Num(1)),
            Header::Definition(result) => conversion.ty(result)?,
            Header::Theorem(statement) => conversion.term(statement)?,
            Header::Unavailable(kind) => {
                return Err(format!("unsupported semantic glossary category `{kind}`"))
            }
        };
        Ok(pi(binders, result))
    }
}

/// Retain only headers, not duplicate computational bodies. Call this only
/// after linking, production checks and source closure have accepted a module.
#[must_use]
pub fn exports(
    module: &str,
    lean_module: &str,
    semantic: &SemanticModule,
    imported_lean: &BTreeMap<String, String>,
) -> Vec<SemanticExport> {
    let mut modules = imported_lean.clone();
    modules.insert(module.to_owned(), lean_module.to_owned());
    let modules = Arc::new(modules);
    semantic
        .declarations
        .iter()
        .map(|declaration| {
            let (type_parameters, parameters, header) = match declaration {
                SemanticDeclaration::Inductive {
                    type_parameters,
                    parameters,
                    ..
                }
                | SemanticDeclaration::Structure {
                    type_parameters,
                    parameters,
                    ..
                } => (type_parameters.clone(), parameters.clone(), Header::Type),
                SemanticDeclaration::Definition {
                    type_parameters,
                    parameters,
                    result,
                    ..
                } => (
                    type_parameters.clone(),
                    parameters.clone(),
                    Header::Definition(Box::new(result.clone())),
                ),
                SemanticDeclaration::Theorem {
                    type_parameters,
                    parameters,
                    statement,
                    ..
                } => (
                    type_parameters.clone(),
                    parameters.clone(),
                    Header::Theorem(Box::new(statement.clone())),
                ),
                other => (Vec::new(), Vec::new(), Header::Unavailable(other.kind())),
            };
            SemanticExport {
                module: module.to_owned(),
                own_module: module.to_owned(),
                component: declaration.name().to_owned(),
                lean_name: crate::backend::semantic::identifier(&format!(
                    "{lean_module}.{}",
                    declaration.name()
                )),
                modules: modules.clone(),
                type_parameters,
                parameters,
                header,
            }
        })
        .collect()
}

fn pi(binders: Vec<Binder>, body: Term) -> Term {
    if binders.is_empty() {
        body
    } else {
        Term::Pi {
            binders,
            body: Box::new(body),
        }
    }
}
fn app(function: Term, arguments: Vec<Term>) -> Term {
    if arguments.is_empty() {
        function
    } else {
        Term::App {
            function: Box::new(function),
            explicit_args: arguments,
            omitted_implicit_binders: Vec::new(),
        }
    }
}
fn core(core: CoreRef, arguments: Vec<Term>) -> Term {
    app(Term::Global(GlobalRef::Core(core), Vec::new()), arguments)
}

struct Convert<'a, 'b> {
    export: &'a SemanticExport,
    closure: &'a Closure,
    alloc: &'b mut LocalAlloc,
    locals: BTreeMap<String, LocalId>,
}
impl Convert<'_, '_> {
    fn bind(&mut self, name: &str, ty: Term) -> Binder {
        let id = self.alloc.fresh();
        self.locals.insert(name.to_owned(), id);
        Binder {
            id,
            mode: BinderMode::Explicit,
            ty,
            spelling: name.to_owned(),
        }
    }
    fn local(&self, name: &str) -> Result<Term, String> {
        self.locals
            .get(name)
            .copied()
            .map(Term::Local)
            .ok_or_else(|| format!("semantic header has no scoped binder `{name}`"))
    }
    // These names are fixed by the semantic backend, never inferred from a
    // claimed glossary signature. Use an actual closure reference so normal
    // external probing and source attribution remain unchanged.
    fn builtin(&self, name: &str) -> Result<Term, String> {
        for package in &self.closure.packages {
            for (id, entry) in &package.entries {
                if let Denotation::Lean {
                    module,
                    name: actual,
                } = &entry.denotation
                {
                    if module == "Init" && actual == name {
                        if let Some(signature_hash) = entry.signature_hash {
                            return Ok(Term::Global(
                                GlobalRef::External(ExternalConstRef {
                                    package: package.id.clone(),
                                    entry: format!("{}::{id}", package.id),
                                    lean_module: module.clone(),
                                    lean_name: actual.clone(),
                                    signature_hash,
                                }),
                                Vec::new(),
                            ));
                        }
                    }
                }
            }
        }
        Err(format!("semantic header uses `{name}`, which has no declared glossary denotation in the closure"))
    }
    fn member(&self, member: &MemberRef) -> Result<Term, String> {
        if member.module.is_none() {
            if let Some((owner, _)) = member.name.split_once('.') {
                if matches!(
                    owner,
                    "Nat" | "Bool" | "Unit" | "Option" | "List" | "Result"
                ) {
                    let name = member.name.replacen("Result.", "Except.", 1);
                    return self.builtin(&name);
                }
            }
        }
        let module = member.module.as_deref().unwrap_or(&self.export.own_module);
        let lean = self.export.modules.get(module).ok_or_else(|| {
            format!(
                "semantic header member `{module}::{}` is outside its validated direct imports",
                member.name
            )
        })?;
        Ok(Term::Global(
            GlobalRef::Document(DocumentDeclRef {
                module: module.to_owned(),
                component: member.name.clone(),
                lean_name: crate::backend::semantic::identifier(&format!("{lean}.{}", member.name)),
            }),
            Vec::new(),
        ))
    }
    fn ty(&mut self, ty: &SemanticType) -> Result<Term, String> {
        use SemanticType as T;
        Ok(match ty {
            T::Type => Term::Sort(Universe::Num(1)),
            T::Prop => Term::Sort(Universe::Num(0)),
            T::Parameter { name } => self.local(name)?,
            T::Named { member, arguments } => {
                let function = self.member(member)?;
                let args = arguments
                    .iter()
                    .map(|argument| self.ty(argument))
                    .collect::<Result<Vec<_>, _>>()?;
                app(function, args)
            }
            T::Function { parameters, result } => {
                let mut binders = Vec::new();
                for parameter in parameters {
                    let ty = self.ty(parameter)?;
                    binders.push(Binder {
                        id: self.alloc.fresh(),
                        mode: BinderMode::Explicit,
                        ty,
                        spelling: String::new(),
                    });
                }
                pi(binders, self.ty(result)?)
            }
            T::Option { value } => {
                let value = self.ty(value)?;
                app(self.builtin("Option")?, vec![value])
            }
            T::Result { ok, error } => {
                let error = self.ty(error)?;
                let ok = self.ty(ok)?;
                app(self.builtin("Except")?, vec![error, ok])
            }
            T::List { element } | T::Set { element } => {
                let element = self.ty(element)?;
                app(self.builtin("List")?, vec![element])
            }
            T::Product { left, right }
            | T::Map {
                key: left,
                value: right,
            } => {
                let left = self.ty(left)?;
                let right = self.ty(right)?;
                let pair = app(self.builtin("Prod")?, vec![left, right]);
                if matches!(ty, T::Map { .. }) {
                    app(self.builtin("List")?, vec![pair])
                } else {
                    pair
                }
            }
            T::ContractViolation | T::ReasoningFailure => {
                let boolean = self.builtin("Bool")?;
                app(self.builtin("Prod")?, vec![boolean.clone(), boolean])
            }
            T::Nat => self.builtin("Nat")?,
            T::Bool => self.builtin("Bool")?,
            T::Unit => self.builtin("Unit")?,
            T::Int => self.builtin("Int")?,
            T::Int8 => self.builtin("Int8")?,
            T::Int16 => self.builtin("Int16")?,
            T::Int32 => self.builtin("Int32")?,
            T::Int64 => self.builtin("Int64")?,
            T::UInt8 => self.builtin("UInt8")?,
            T::UInt16 => self.builtin("UInt16")?,
            T::UInt32 => self.builtin("UInt32")?,
            T::UInt64 => self.builtin("UInt64")?,
            T::String => self.builtin("String")?,
            T::Bytes => self.builtin("ByteArray")?,
            T::Ordering => self.builtin("Ordering")?,
        })
    }
    #[allow(clippy::too_many_lines)]
    fn term(&mut self, term: &SemanticTerm) -> Result<Term, String> {
        use SemanticTerm as S;
        Ok(match term {
            S::Var { name } => self.local(name)?,
            S::Nat { value } => Term::NatLiteral { decimal: value.clone(), expected_type: Box::new(self.builtin("Nat")?) },
            S::Bool { value } => self.builtin(if *value { "Bool.true" } else { "Bool.false" })?,
            S::Unit => self.builtin("Unit.unit")?,
            S::Call { function, type_arguments, arguments } | S::Constructor { constructor: function, type_arguments, arguments } => {
                let head = self.member(function)?;
                let mut args = type_arguments.iter().map(|ty| self.ty(ty)).collect::<Result<Vec<_>, _>>()?;
                args.extend(arguments.iter().map(|term| self.term(term)).collect::<Result<Vec<_>, _>>()?);
                app(head, args)
            }
            S::FunctionRef { function, type_arguments } => {
                let head = self.member(function)?;
                app(head, type_arguments.iter().map(|ty| self.ty(ty)).collect::<Result<Vec<_>, _>>()?)
            }
            S::Apply { function, arguments } => {
                let function = self.term(function)?;
                app(function, arguments.iter().map(|arg| self.term(arg)).collect::<Result<Vec<_>, _>>()?)
            }
            S::Add { left, right } | S::Le { left, right } | S::Lt { left, right } | S::Beq { left, right } | S::Ble { left, right } | S::Blt { left, right } | S::And { left, right } | S::Or { left, right } => {
                let name = match term { S::Add { .. } => "Nat.add", S::Le { .. } => "Nat.le", S::Lt { .. } => "Nat.lt", S::Beq { .. } => "Nat.beq", S::Ble { .. } => "Nat.ble", S::Blt { .. } => "Nat.blt", S::And { .. } => "Bool.and", _ => "Bool.or" };
                app(self.builtin(name)?, vec![self.term(left)?, self.term(right)?])
            }
            S::Not { value } => app(self.builtin("Bool.not")?, vec![self.term(value)?]),
            S::Eq { left, right } => core(CoreRef::Eq, vec![self.term(left)?, self.term(right)?]),
            S::PropAnd { left, right } => core(CoreRef::And, vec![self.term(left)?, self.term(right)?]),
            S::Iff { left, right } => core(CoreRef::Iff, vec![self.term(left)?, self.term(right)?]),
            S::Implies { premise, conclusion } => {
                let premise = self.term(premise)?; let conclusion = self.term(conclusion)?;
                pi(vec![Binder { id: self.alloc.fresh(), mode: BinderMode::Explicit, ty: premise, spelling: String::new() }], conclusion)
            }
            S::Forall { binder, body } => {
                let saved = self.locals.clone(); let ty = self.ty(&binder.r#type)?;
                let binder = self.bind(&binder.name, ty); let body = self.term(body)?; self.locals = saved;
                pi(vec![binder], body)
            }
            S::Lambda { parameters, body, .. } => {
                let saved = self.locals.clone(); let mut binders = Vec::new();
                for parameter in parameters { let ty = self.ty(&parameter.r#type)?; binders.push(self.bind(&parameter.name, ty)); }
                let body = self.term(body)?; self.locals = saved;
                Term::Lambda { binders, body: Box::new(body) }
            }
            S::Let { binder, value, body } => {
                let saved = self.locals.clone(); let ty = self.ty(&binder.r#type)?; let value = self.term(value)?;
                let binder = self.bind(&binder.name, ty); let body = self.term(body)?; self.locals = saved;
                Term::Let { binder: Box::new(binder), value: Box::new(value), body: Box::new(body) }
            }
            other => return Err(format!("semantic theorem header has no exact prose LSE representation for `{}`; name that expression with a typed semantic definition before referring to the theorem", term_kind(other))),
        })
    }
}
fn term_kind(term: &SemanticTerm) -> &'static str {
    match term {
        SemanticTerm::Integer { .. } => "integer",
        SemanticTerm::String { .. } => "string",
        SemanticTerm::Bytes { .. } => "bytes",
        SemanticTerm::Primitive { .. } => "primitive",
        SemanticTerm::Nil { .. } => "nil",
        SemanticTerm::Cons { .. } => "cons",
        SemanticTerm::Record { .. } => "record",
        SemanticTerm::InstanceValue { .. } => "instance_value",
        SemanticTerm::Project { .. } => "project",
        SemanticTerm::If { .. } => "if",
        SemanticTerm::Match { .. } => "match",
        SemanticTerm::Le { .. } => "le",
        SemanticTerm::Lt { .. } => "lt",
        SemanticTerm::Add { .. } => "add",
        SemanticTerm::Beq { .. } => "beq",
        SemanticTerm::Ble { .. } => "ble",
        SemanticTerm::Blt { .. } => "blt",
        SemanticTerm::And { .. } => "and",
        SemanticTerm::Or { .. } => "or",
        SemanticTerm::Not { .. } => "not",
        SemanticTerm::Pair { .. } => "pair",
        SemanticTerm::First { .. } => "first",
        SemanticTerm::Second { .. } => "second",
        SemanticTerm::MapLiteral { .. } => "map_literal",
        SemanticTerm::SetLiteral { .. } => "set_literal",
        SemanticTerm::GraphLiteral { .. } => "graph_literal",
        SemanticTerm::CheckedApply { .. } => "checked_apply",
        _ => "represented term",
    }
}
