//! Checks a rendered crate before it is printed (SPEC.md §17.16): its
//! profile admits every type and runtime function it uses, every name it
//! uses is declared and bound exactly once, every value is moved at most
//! once on any path, every fallible call propagates its failure and only a
//! fallible function propagates, and every construct it emits corresponds
//! to an element of the target program it realizes.

use std::collections::{BTreeMap, BTreeSet};

use super::super::IntKind;
use super::ast::{Block, Callee, Crate, Ctor, Expr, Ident, ItemDef, Lit, Origin, Pat, Type};
use super::Profile;

fn fail<T>(reason: impl Into<String>) -> Result<T, String> {
    Err(reason.into())
}

/// What the crate declares: its enums with their variants' arities, its
/// function types' fallibility, and its functions' fallibility.
struct Declared {
    adts: BTreeMap<u64, BTreeMap<u64, usize>>,
    closures: BTreeMap<u64, BTreeMap<u64, usize>>,
    applies: BTreeMap<u64, bool>,
    functions: BTreeMap<u64, bool>,
}

fn declared(krate: &Crate) -> Result<Declared, String> {
    let mut out = Declared {
        adts: BTreeMap::new(),
        closures: BTreeMap::new(),
        applies: BTreeMap::new(),
        functions: BTreeMap::new(),
    };
    for item in &krate.items {
        match item {
            ItemDef::Enum { name, variants, .. } => {
                let arities = variants
                    .iter()
                    .map(|(number, fields)| (*number, fields.len()))
                    .collect();
                let fresh = match name {
                    Type::Adt(n) => out.adts.insert(*n, arities).is_none(),
                    Type::Fn(n) => out.closures.insert(*n, arities).is_none(),
                    other => {
                        return fail(format!("an enum named {other:?} is not a declared type"))
                    }
                };
                if !fresh {
                    return fail(format!("hygiene: {name:?} is declared twice"));
                }
            }
            ItemDef::Apply {
                fn_type, fallible, ..
            } => {
                if out.applies.insert(*fn_type, *fallible).is_some() {
                    return fail(format!("hygiene: Fn{fn_type}::apply is declared twice"));
                }
            }
            ItemDef::Function { name, result, .. } => {
                if let Ident::Function(n) = name {
                    if out
                        .functions
                        .insert(*n, matches!(result, Type::Fallible(_)))
                        .is_some()
                    {
                        return fail(format!("hygiene: f{n} is declared twice"));
                    }
                }
            }
        }
    }
    Ok(out)
}

struct Checker<'a> {
    profile: Profile,
    declared: &'a Declared,
    /// Every name bound in the current function, so none is bound twice.
    bound: BTreeSet<Ident>,
    /// The names in scope at this point.
    scope: Vec<Ident>,
    /// The names moved on the current path.
    moved: BTreeSet<Ident>,
    /// Whether a `?` occurs in the current function.
    propagates: bool,
    /// Whether the current function is fallible, so its tail may return a
    /// fallible call's result without propagating it.
    fallible: bool,
}

impl Checker<'_> {
    fn ty(&self, ty: &Type) -> Result<(), String> {
        if self.profile == Profile::Core && ty.heap() {
            return fail(format!(
                "hidden allocation: rust-core renders the heap type {ty:?}, which rust-core does not provide"
            ));
        }
        match ty {
            Type::Adt(n) if !self.declared.adts.contains_key(n) => {
                fail(format!("type Adt{n} is not declared"))
            }
            Type::Fn(n) if !self.declared.closures.contains_key(n) => {
                fail(format!("type Fn{n} is not declared"))
            }
            Type::Option(inner)
            | Type::List(inner)
            | Type::Rc(inner)
            | Type::Fallible(inner)
            | Type::Ref(inner) => self.ty(inner),
            Type::Result(left, right) | Type::Pair(left, right) => {
                self.ty(left)?;
                self.ty(right)
            }
            _ => Ok(()),
        }
    }

    fn heap(&self, what: &str) -> Result<(), String> {
        if self.profile == Profile::Core {
            return fail(format!(
                "hidden allocation: rust-core renders {what}, which requires heap allocation"
            ));
        }
        Ok(())
    }

    fn bind(&mut self, pattern: &Pat) -> Result<(), String> {
        match pattern {
            Pat::Wild | Pat::Unit | Pat::None | Pat::Ordering(_) => Ok(()),
            Pat::Bind(ident) => {
                if !self.bound.insert(ident.clone()) {
                    return fail(format!(
                        "hygiene: `{}` is bound twice in one function",
                        ident_text(ident)
                    ));
                }
                self.scope.push(ident.clone());
                Ok(())
            }
            Pat::Tuple(items) => items.iter().try_for_each(|item| self.bind(item)),
            Pat::Some(inner) | Pat::Ok(inner) | Pat::Err(inner) => self.bind(inner),
            Pat::Adt {
                adt,
                constructor,
                fields,
            } => {
                let arity = self
                    .declared
                    .adts
                    .get(adt)
                    .and_then(|variants| variants.get(constructor))
                    .ok_or_else(|| format!("pattern Adt{adt}::C{constructor} is not declared"))?;
                if *arity != fields.len() {
                    return fail(format!(
                        "pattern Adt{adt}::C{constructor} binds {} fields of {arity}",
                        fields.len()
                    ));
                }
                fields.iter().try_for_each(|field| self.bind(field))
            }
        }
    }

    /// A read of `ident` that leaves it in place.
    fn read(&self, ident: &Ident) -> Result<(), String> {
        if !self.scope.contains(ident) {
            return fail(format!("`{}` is not bound", ident_text(ident)));
        }
        if self.moved.contains(ident) {
            return fail(format!(
                "ownership: `{}` is read after it was moved",
                ident_text(ident)
            ));
        }
        Ok(())
    }

    fn consume(&mut self, ident: &Ident) -> Result<(), String> {
        if !self.scope.contains(ident) {
            return fail(format!("`{}` is not bound", ident_text(ident)));
        }
        if !self.moved.insert(ident.clone()) {
            return fail(format!("ownership: `{}` is moved twice", ident_text(ident)));
        }
        Ok(())
    }

    fn call(
        &mut self,
        fallible: bool,
        propagate: bool,
        tail: bool,
        what: &str,
    ) -> Result<(), String> {
        match (fallible, propagate) {
            // The tail of a fallible function returns the call's result.
            (true, false) if tail && self.fallible => Ok(()),
            (true, false) => fail(format!(
                "arithmetic: the fallible call of {what} does not propagate its overflow"
            )),
            (false, true) => fail(format!(
                "arithmetic: the infallible call of {what} propagates a failure it cannot have"
            )),
            _ => {
                self.propagates |= propagate;
                Ok(())
            }
        }
    }

    fn block(&mut self, block: &Block, tail: bool) -> Result<(), String> {
        let depth = self.scope.len();
        for binding in &block.lets {
            if let Some(ty) = &binding.ty {
                self.ty(ty)?;
            }
            self.expr(&binding.value, false)?;
            self.bind(&binding.pat)?;
        }
        let checked = self.expr(&block.tail, tail);
        self.scope.truncate(depth);
        checked
    }

    /// Branches are alternatives: each starts from the same moved set, and
    /// a name moved on any of them is moved afterwards.
    fn branches<'b>(
        &mut self,
        branches: impl IntoIterator<Item = (Option<&'b Pat>, &'b Block)>,
        tail: bool,
    ) -> Result<(), String> {
        let before = self.moved.clone();
        let mut after = before.clone();
        for (pattern, body) in branches {
            self.moved = before.clone();
            let depth = self.scope.len();
            if let Some(pattern) = pattern {
                self.bind(pattern)?;
            }
            let checked = self.block(body, tail);
            self.scope.truncate(depth);
            checked?;
            after.extend(self.moved.iter().cloned());
        }
        self.moved = after;
        Ok(())
    }

    /// Check `expr`; `tail` says it is the value the current function
    /// returns.
    #[allow(clippy::too_many_lines)]
    fn expr(&mut self, expr: &Expr, tail: bool) -> Result<(), String> {
        // The value a fallible function returns is `R<T>`: an `Ok`, a
        // fallible call or application returning its own result, or a
        // branch or block whose tails are such values.
        let returns_result = matches!(
            expr,
            Expr::Succeed(..)
                | Expr::If { .. }
                | Expr::Match { .. }
                | Expr::Block(_)
                | Expr::Apply {
                    propagate: false,
                    ..
                }
                | Expr::Call {
                    propagate: false,
                    ..
                }
        );
        if tail && self.fallible && !returns_result {
            return fail(format!(
                "arithmetic: the value a fallible function returns, {}, is not a fallible value",
                constructs_of(expr)
            ));
        }
        match expr {
            Expr::Lit(literal, _) => match literal {
                Lit::Str(_) => self.heap("a string literal"),
                Lit::Bytes(_) => self.heap("a byte string literal"),
                Lit::Unit
                | Lit::Bool(_)
                | Lit::Nat(_)
                | Lit::Int(_)
                | Lit::Fixed(..)
                | Lit::Ordering(_) => Ok(()),
            },
            Expr::Move(ident, _) => self.consume(ident),
            Expr::Not(inner, _) => {
                if matches!(
                    inner.as_ref(),
                    Expr::IsZero(..) | Expr::NonZero(..) | Expr::Predecessor(_)
                ) {
                    return fail(
                        "a negated zero test or predecessor: a zero test is negated as its complement, and a predecessor is not a Boolean",
                    );
                }
                self.expr(inner, false)
            }
            Expr::Clone(ident, _)
            | Expr::Copy(ident, _)
            | Expr::Deref(ident, _)
            | Expr::Uncons(ident)
            | Expr::IsZero(ident, _)
            | Expr::NonZero(ident, _)
            | Expr::Predecessor(ident) => {
                if matches!(expr, Expr::Uncons(_)) {
                    self.heap("a list match")?;
                }
                self.read(ident)
            }
            Expr::Unbox(ident, _) => {
                self.heap("a boxed read")?;
                self.read(ident)
            }
            Expr::Box(inner, _) => {
                self.heap("a box")?;
                self.expr(inner, false)
            }
            Expr::Call {
                callee,
                args,
                propagate,
                ..
            } => {
                for arg in args {
                    self.expr(arg, false)?;
                }
                match callee {
                    Callee::Function(n) => {
                        let fallible = *self
                            .declared
                            .functions
                            .get(n)
                            .ok_or_else(|| format!("f{n} is not declared"))?;
                        self.call(fallible, *propagate, tail, &format!("f{n}"))
                    }
                    Callee::Runtime(item) => {
                        if item.heap() {
                            self.heap(&format!("the runtime function `{}`", item.path()))?;
                        }
                        self.call(
                            item.fallible(),
                            *propagate,
                            tail,
                            &format!("`{}`", item.path()),
                        )
                    }
                }
            }
            Expr::Apply {
                holder,
                args,
                propagate,
                ..
            } => {
                self.read(holder)?;
                for arg in args {
                    self.expr(arg, false)?;
                }
                // The applied closure's type is known only through its
                // binding, so its fallibility is checked by `apply_types`.
                self.propagates |= *propagate;
                Ok(())
            }
            Expr::Construct { ctor, args, .. } => {
                for arg in args {
                    self.expr(arg, false)?;
                }
                match ctor {
                    Ctor::None(ty) | Ctor::Nil(ty) => {
                        if matches!(ctor, Ctor::Nil(_)) {
                            self.heap("an empty list")?;
                        }
                        self.ty(ty)
                    }
                    Ctor::Ok(ok, error) | Ctor::Err(ok, error) => {
                        self.ty(ok)?;
                        self.ty(error)
                    }
                    Ctor::Cons => self.heap("a list cell"),
                    Ctor::Some => Ok(()),
                    Ctor::Adt { adt, constructor } => {
                        let arity = self
                            .declared
                            .adts
                            .get(adt)
                            .and_then(|variants| variants.get(constructor))
                            .ok_or_else(|| format!("Adt{adt}::C{constructor} is not declared"))?;
                        if *arity != args.len() {
                            return fail(format!(
                                "Adt{adt}::C{constructor} receives {} fields of {arity}",
                                args.len()
                            ));
                        }
                        Ok(())
                    }
                    Ctor::Closure { fn_type, function } => {
                        let arity = self
                            .declared
                            .closures
                            .get(fn_type)
                            .and_then(|variants| variants.get(function))
                            .ok_or_else(|| format!("Fn{fn_type}::F{function} is not declared"))?;
                        if *arity != args.len() {
                            return fail(format!(
                                "Fn{fn_type}::F{function} captures {} values of {arity}",
                                args.len()
                            ));
                        }
                        Ok(())
                    }
                }
            }
            Expr::Pair(left, right, _) => {
                self.expr(left, false)?;
                self.expr(right, false)
            }
            Expr::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                self.expr(condition, false)?;
                self.branches(
                    [(None, then_branch.as_ref()), (None, else_branch.as_ref())],
                    tail,
                )
            }
            Expr::Match {
                scrutinee, arms, ..
            } => {
                self.expr(scrutinee, false)?;
                self.branches(
                    arms.iter().map(|(pattern, body)| (Some(pattern), body)),
                    tail,
                )
            }
            Expr::Block(block) => self.block(block, tail),
            Expr::Widen(inner, _) | Expr::Succeed(inner, _) => self.expr(inner, false),
        }
    }
}

/// The kind of an expression, for a diagnostic.
fn constructs_of(expr: &Expr) -> String {
    let mut found = Found::default();
    found.expr(expr);
    found
        .instances
        .into_iter()
        .map(|instance| instance.construct)
        .min()
        .unwrap_or_else(|| "a read".to_owned())
}

fn ident_text(ident: &Ident) -> String {
    match ident {
        Ident::Local(n) => format!("v{n}"),
        Ident::Holder(n) => format!("m{n}"),
        Ident::Operand(n) => format!("a{n}"),
        Ident::Callee(n) => format!("c{n}"),
        Ident::Boxed(n) => format!("r{n}"),
        Ident::Part(n) => format!("h{n}"),
        Ident::Capture(n) => format!("k{n}"),
        Ident::Param(n) => format!("p{n}"),
        Ident::Function(n) => format!("f{n}"),
        Ident::Export(name) => name.clone(),
    }
}

/// Every closure applied through a binding of function type `n` is bound by
/// `let c: Fn<n> = ...`, so its `apply` is known; a call through it
/// propagates exactly when that `apply` is fallible.
fn apply_types(
    block: &Block,
    declared: &Declared,
    types: &mut BTreeMap<Ident, u64>,
    tail: bool,
) -> Result<(), String> {
    for binding in &block.lets {
        if let (Pat::Bind(ident), Some(Type::Fn(n))) = (&binding.pat, &binding.ty) {
            types.insert(ident.clone(), *n);
        }
        apply_types_expr(&binding.value, declared, types, false)?;
    }
    apply_types_expr(&block.tail, declared, types, tail)
}

/// `tail` says the expression is the value a fallible function returns,
/// where a fallible application returns its result without propagating.
fn apply_types_expr(
    expr: &Expr,
    declared: &Declared,
    types: &mut BTreeMap<Ident, u64>,
    tail: bool,
) -> Result<(), String> {
    match expr {
        Expr::Apply {
            holder,
            args,
            propagate,
            ..
        } => {
            let fn_type = types.get(holder).ok_or_else(|| {
                format!(
                    "`{}` is applied without a function type",
                    ident_text(holder)
                )
            })?;
            let fallible = declared
                .applies
                .get(fn_type)
                .ok_or_else(|| format!("Fn{fn_type}::apply is not declared"))?;
            if fallible != propagate && !(tail && *fallible) {
                return fail(format!(
                    "arithmetic: an application of Fn{fn_type} {} a failure its apply {}",
                    if *propagate { "propagates" } else { "drops" },
                    if *fallible { "can have" } else { "cannot have" }
                ));
            }
            args.iter()
                .try_for_each(|arg| apply_types_expr(arg, declared, types, false))
        }
        Expr::Box(inner, _)
        | Expr::Widen(inner, _)
        | Expr::Succeed(inner, _)
        | Expr::Not(inner, _) => apply_types_expr(inner, declared, types, false),
        Expr::Call { args, .. } | Expr::Construct { args, .. } => args
            .iter()
            .try_for_each(|arg| apply_types_expr(arg, declared, types, false)),
        Expr::Pair(left, right, _) => {
            apply_types_expr(left, declared, types, false)?;
            apply_types_expr(right, declared, types, false)
        }
        Expr::If {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            apply_types_expr(condition, declared, types, false)?;
            apply_types(then_branch, declared, types, tail)?;
            apply_types(else_branch, declared, types, tail)
        }
        Expr::Match {
            scrutinee, arms, ..
        } => {
            apply_types_expr(scrutinee, declared, types, false)?;
            arms.iter()
                .try_for_each(|(_, body)| apply_types(body, declared, types, tail))
        }
        Expr::Block(block) => apply_types(block, declared, types, tail),
        Expr::Lit(..)
        | Expr::Move(..)
        | Expr::Copy(..)
        | Expr::Deref(..)
        | Expr::Clone(..)
        | Expr::Unbox(..)
        | Expr::Uncons(_)
        | Expr::IsZero(..)
        | Expr::NonZero(..)
        | Expr::Predecessor(_) => Ok(()),
    }
}

/// Check a crate before it is printed.
///
/// # Errors
///
/// Returns the first violation: a heap type, function, or construct in
/// `rust-core` (hidden allocation), an undeclared or doubly declared name, a
/// name bound twice in one function (hygiene), a value moved twice or read
/// after it was moved (ownership), or a failure dropped, invented, or
/// returned from a function that declares none (arithmetic).
pub fn validate(krate: &Crate) -> Result<(), String> {
    let declared = declared(krate)?;
    for item in &krate.items {
        let mut checker = Checker {
            profile: krate.profile,
            declared: &declared,
            bound: BTreeSet::new(),
            scope: Vec::new(),
            moved: BTreeSet::new(),
            propagates: false,
            fallible: false,
        };
        match item {
            ItemDef::Enum { variants, .. } => {
                for ty in variants.iter().flat_map(|(_, fields)| fields) {
                    checker.ty(ty)?;
                }
            }
            ItemDef::Apply {
                fn_type,
                parameters,
                result,
                fallible,
                arms,
                ..
            } => {
                for ty in parameters.iter().chain([result]) {
                    checker.ty(ty)?;
                }
                for arm in arms {
                    let is_function = declared.functions.get(&arm.function).ok_or_else(|| {
                        format!("Fn{fn_type} dispatches to undeclared f{}", arm.function)
                    })?;
                    if *is_function != arm.function_fallible || (arm.function_fallible && !fallible)
                    {
                        return fail(format!(
                            "arithmetic: Fn{fn_type}::apply misstates the failure of f{}",
                            arm.function
                        ));
                    }
                    if arm
                        .captures
                        .iter()
                        .any(|read| matches!(read, super::ast::CaptureRead::Unbox))
                    {
                        checker.heap("a boxed capture")?;
                    }
                }
            }
            ItemDef::Function {
                name,
                parameters,
                result,
                body,
                ..
            } => {
                checker.ty(result)?;
                for (pattern, ty) in parameters {
                    checker.ty(ty)?;
                    checker.bind(pattern)?;
                }
                let fallible = matches!(result, Type::Fallible(_));
                checker.fallible = fallible;
                checker.block(body, true)?;
                if checker.propagates && !fallible {
                    return fail(format!(
                        "arithmetic: `{}` propagates a failure its result type does not carry",
                        ident_text(name)
                    ));
                }
                let mut types = BTreeMap::new();
                for (pattern, ty) in parameters {
                    if let (Pat::Bind(ident), Type::Fn(n)) = (pattern, ty) {
                        types.insert(ident.clone(), *n);
                    }
                }
                apply_types(body, &declared, &mut types, fallible)?;
            }
        }
    }
    Ok(())
}

// --- correspondence ----------------------------------------------------------

/// One emitted construct: its kind, the origin lowering gave it, and its own
/// width when it is a fixed-width runtime function or literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instance {
    pub construct: String,
    pub origin: Origin,
    pub width: Option<IntKind>,
}

/// Every construct a crate emits, instance by instance, and every type
/// construct its items, bindings, and constructors name.
#[must_use]
pub fn instances(krate: &Crate) -> (Vec<Instance>, BTreeSet<String>) {
    let mut found = Found::default();
    for item in &krate.items {
        match item {
            ItemDef::Enum { name, variants, at } => {
                found.at(
                    match name {
                        Type::Fn(_) => "enum:closures",
                        _ => "enum:adt",
                    },
                    at,
                );
                for ty in variants.iter().flat_map(|(_, fields)| fields) {
                    found.ty(ty);
                }
            }
            ItemDef::Apply {
                parameters,
                result,
                at,
                ..
            } => {
                found.at("apply:dispatch", at);
                for ty in parameters.iter().chain([result]) {
                    found.ty(ty);
                }
            }
            ItemDef::Function {
                parameters,
                result,
                body,
                at,
                ..
            } => {
                found.at("function", at);
                for (_, ty) in parameters {
                    found.ty(ty);
                }
                found.ty(result);
                found.block(body);
            }
        }
    }
    (found.instances, found.types)
}

/// Every construct kind a crate emits, its types included.
#[must_use]
pub fn constructs(krate: &Crate) -> BTreeSet<String> {
    let (instances, mut types) = instances(krate);
    types.extend(instances.into_iter().map(|instance| instance.construct));
    types
}

#[derive(Default)]
struct Found {
    instances: Vec<Instance>,
    types: BTreeSet<String>,
}

impl Found {
    fn at(&mut self, construct: &str, origin: &Origin) {
        self.wide(construct, origin, None);
    }

    fn wide(&mut self, construct: &str, origin: &Origin, width: Option<IntKind>) {
        self.instances.push(Instance {
            construct: construct.to_owned(),
            origin: origin.clone(),
            width,
        });
    }

    fn ty(&mut self, ty: &Type) {
        let kind = match ty {
            Type::Unit => "unit",
            Type::Bool => "bool",
            Type::Nat => "nat",
            Type::Int => "int",
            Type::Fixed(_) => "fixed",
            Type::Ordering => "ordering",
            Type::Str => "string",
            Type::Bytes => "bytes",
            Type::Option(_) => "option",
            Type::Result(..) => "result",
            Type::List(_) => "list",
            Type::Pair(..) => "pair",
            Type::Adt(_) => "adt",
            Type::Fn(_) => "fn",
            Type::Rc(_) => "rc",
            Type::Fallible(_) => "fallible",
            Type::Ref(_) => "ref",
        };
        self.types.insert(format!("type:{kind}"));
        match ty {
            Type::Option(inner)
            | Type::List(inner)
            | Type::Rc(inner)
            | Type::Fallible(inner)
            | Type::Ref(inner) => self.ty(inner),
            Type::Result(left, right) | Type::Pair(left, right) => {
                self.ty(left);
                self.ty(right);
            }
            _ => {}
        }
    }

    fn block(&mut self, block: &Block) {
        for binding in &block.lets {
            if let Some(ty) = &binding.ty {
                self.ty(ty);
            }
            let construct = match binding.pat {
                Pat::Tuple(_) => "destructure:pair",
                _ => "let",
            };
            self.at(construct, &binding.at);
            self.expr(&binding.value);
        }
        self.expr(&block.tail);
    }

    fn pattern(&mut self, pattern: &Pat, origin: &Origin) {
        let kind = match pattern {
            Pat::None | Pat::Some(_) => "match:option",
            Pat::Ok(_) | Pat::Err(_) => "match:result",
            Pat::Ordering(_) => "match:ordering",
            Pat::Adt { .. } => "match:adt",
            Pat::Wild | Pat::Bind(_) | Pat::Tuple(_) | Pat::Unit => return,
        };
        self.at(kind, origin);
    }

    fn expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Lit(literal, origin) => {
                let (kind, width) = match literal {
                    Lit::Unit => ("unit", None),
                    Lit::Bool(_) => ("bool", None),
                    Lit::Nat(_) => ("nat", None),
                    Lit::Int(_) => ("int", None),
                    Lit::Fixed(kind, _) => ("fixed", Some(*kind)),
                    Lit::Str(_) => ("string", None),
                    Lit::Bytes(_) => ("bytes", None),
                    Lit::Ordering(_) => ("ordering", None),
                };
                self.wide(&format!("lit:{kind}"), origin, width);
            }
            Expr::Move(_, origin)
            | Expr::Clone(_, origin)
            | Expr::Copy(_, origin)
            | Expr::Deref(_, origin) => self.at("read", origin),
            Expr::Not(inner, origin) => {
                self.at("if", origin);
                self.expr(inner);
            }
            Expr::Unbox(_, origin) => self.at("box", origin),
            Expr::Box(inner, origin) => {
                self.at("box", origin);
                self.expr(inner);
            }
            Expr::Call {
                callee, args, at, ..
            } => {
                match callee {
                    Callee::Function(_) => self.at("call:function", at),
                    Callee::Runtime(item) => {
                        let path = item.path();
                        let base = path.rsplit("::").next().unwrap_or(&path).to_owned();
                        self.wide(
                            &format!("call:runtime:{}", runtime_kind(&base)),
                            at,
                            item.width(),
                        );
                    }
                }
                args.iter().for_each(|arg| self.expr(arg));
            }
            Expr::Apply { args, at, .. } => {
                self.at("apply", at);
                args.iter().for_each(|arg| self.expr(arg));
            }
            Expr::Construct { ctor, args, at } => {
                let kind = match ctor {
                    Ctor::None(ty) | Ctor::Nil(ty) => {
                        self.ty(ty);
                        if matches!(ctor, Ctor::None(_)) {
                            "none"
                        } else {
                            "nil"
                        }
                    }
                    Ctor::Some => "some",
                    Ctor::Ok(ok, error) | Ctor::Err(ok, error) => {
                        self.ty(ok);
                        self.ty(error);
                        if matches!(ctor, Ctor::Ok(..)) {
                            "ok"
                        } else {
                            "err"
                        }
                    }
                    Ctor::Adt { .. } => "adt",
                    Ctor::Closure { .. } => "closure",
                    Ctor::Cons => "cons",
                };
                self.at(&format!("construct:{kind}"), at);
                args.iter().for_each(|arg| self.expr(arg));
            }
            Expr::Pair(left, right, origin) => {
                self.at("construct:pair", origin);
                self.expr(left);
                self.expr(right);
            }
            Expr::If {
                condition,
                then_branch,
                else_branch,
                at,
            } => {
                self.at("if", at);
                self.expr(condition);
                self.block(then_branch);
                self.block(else_branch);
            }
            Expr::Match {
                scrutinee,
                arms,
                at,
            } => {
                if arms.is_empty() {
                    self.at("match:empty", at);
                } else if matches!(scrutinee.as_ref(), Expr::Uncons(_)) {
                    self.at("match:list", at);
                } else {
                    for (pattern, _) in arms {
                        self.pattern(pattern, at);
                    }
                }
                self.expr(scrutinee);
                for (_, body) in arms {
                    self.block(body);
                }
            }
            Expr::Block(block) => self.block(block),
            Expr::IsZero(_, origin) | Expr::NonZero(_, origin) => self.at("match:nat", origin),
            Expr::Uncons(_) | Expr::Predecessor(_) => {}
            Expr::Widen(inner, origin) => {
                self.at("widen", origin);
                self.expr(inner);
            }
            Expr::Succeed(inner, origin) => {
                self.at("succeed", origin);
                self.expr(inner);
            }
        }
    }
}

/// The calculus primitive a runtime function realizes, by its path's last
/// segment: fixed-width paths share one name per operation.
fn runtime_kind(base: &str) -> String {
    for (prefix, kind) in [
        ("checked_neg_", "checked_neg"),
        ("format_", "format_decimal"),
        ("parse_", "parse_decimal"),
        ("append_", "append"),
        ("length_", "length"),
        ("index_", "index"),
        ("slice_", "slice"),
    ] {
        if base.starts_with(prefix) {
            return kind.to_owned();
        }
    }
    match base {
        "nat_succ" => "succ".to_owned(),
        other => other.to_owned(),
    }
}

/// For every construct a rendering can emit, the calculus elements (or
/// structural realizations, §17.14) it may realize. An emitted construct is
/// justified exactly when its row names the element it was lowered from and
/// the realized program uses that element.
pub const CORRESPONDENCE: &[(&str, &[&str])] = &[
    ("function", &["function", "export"]),
    ("enum:adt", &["type:adt"]),
    ("enum:closures", &["type:fn", "expr:closure"]),
    ("apply:dispatch", &["type:fn", "expr:closure"]),
    (
        "let",
        &[
            "expr:let",
            "expr:match",
            "expr:prim",
            "expr:apply",
            "expr:call",
            "expr:closure",
            "expr:cond",
            "expr:field",
            "shape:some",
            "shape:ok",
            "shape:error",
            "shape:cons",
            "shape:adt",
            "shape:succ",
            "shape:zero",
            "shape:unit",
            "shape:true",
            "indirection",
            "overflow",
        ],
    ),
    (
        "read",
        &[
            "expr:var",
            "expr:match",
            "expr:prim",
            "expr:call",
            "expr:apply",
            "expr:closure",
            "expr:first",
            "expr:second",
            "expr:field",
            "expr:cond",
            "shape:true",
            "shape:zero",
            "shape:some",
            "shape:ok",
            "shape:error",
            "shape:cons",
            "shape:adt",
            "export",
            "type:fn",
            "type:adt",
            "type:pair",
            "type:result",
        ],
    ),
    ("box", &["indirection"]),
    (
        "destructure:pair",
        &["expr:first", "expr:second", "shape:pair"],
    ),
    ("type:unit", &["type:unit"]),
    ("type:bool", &["type:bool"]),
    ("type:nat", &["type:nat", "type:fixed"]),
    ("type:int", &["type:int", "type:fixed"]),
    ("type:fixed", &["type:fixed"]),
    ("type:ordering", &["type:ordering"]),
    ("type:string", &["type:string"]),
    ("type:bytes", &["type:bytes"]),
    ("type:option", &["type:option"]),
    ("type:result", &["type:result"]),
    ("type:list", &["type:list"]),
    ("type:pair", &["type:pair"]),
    ("type:adt", &["type:adt"]),
    ("type:fn", &["type:fn", "expr:closure"]),
    ("type:rc", &["indirection"]),
    ("type:fallible", &["overflow"]),
    ("type:ref", &["export"]),
    (
        "lit:unit",
        &[
            "value:unit",
            "shape:unit",
            "expr:var",
            "expr:call",
            "expr:apply",
            "expr:closure",
            "shape:some",
            "shape:ok",
            "shape:error",
            "shape:cons",
            "shape:adt",
            "export",
            "overflow",
        ],
    ),
    (
        "lit:bool",
        &["value:bool", "shape:true", "shape:false", "expr:cond"],
    ),
    ("lit:nat", &["value:nat", "shape:zero"]),
    ("lit:int", &["value:int"]),
    (
        "lit:fixed",
        &[
            "value:u8",
            "value:u16",
            "value:u32",
            "value:u64",
            "value:i8",
            "value:i16",
            "value:i32",
            "value:i64",
        ],
    ),
    ("lit:string", &["value:string"]),
    ("lit:bytes", &["value:bytes"]),
    (
        "lit:ordering",
        &["value:ordering", "shape:lt", "shape:eq", "shape:gt"],
    ),
    ("construct:none", &["value:none", "shape:none"]),
    ("construct:some", &["value:some", "shape:some"]),
    ("construct:ok", &["value:ok", "shape:ok"]),
    ("construct:err", &["value:error", "shape:error"]),
    ("construct:adt", &["value:adt", "shape:adt"]),
    ("construct:closure", &["expr:closure"]),
    ("construct:cons", &["value:list", "shape:cons"]),
    ("construct:nil", &["value:list", "shape:nil"]),
    ("construct:pair", &["value:pair", "shape:pair"]),
    ("if", &["expr:cond", "shape:true", "shape:zero"]),
    ("match:nat", &["shape:zero"]),
    ("match:list", &["shape:nil"]),
    ("match:option", &["shape:none"]),
    ("match:result", &["shape:ok"]),
    ("match:ordering", &["shape:lt"]),
    ("match:adt", &["shape:adt", "expr:field"]),
    (
        "match:empty",
        &["type:fn", "type:adt", "type:pair", "type:result"],
    ),
    ("call:function", &["expr:call", "export"]),
    ("apply", &["expr:apply"]),
    ("widen", &["prim:convert"]),
    ("succeed", &["overflow"]),
    ("call:runtime:succ", &["shape:succ"]),
    ("call:runtime:nat_add", &["prim:nat_add"]),
    ("call:runtime:nat_sub", &["prim:nat_sub"]),
    ("call:runtime:nat_mul", &["prim:nat_mul"]),
    ("call:runtime:nat_quot", &["prim:nat_quot"]),
    ("call:runtime:nat_rem", &["prim:nat_rem"]),
    ("call:runtime:nat_eq", &["prim:nat_eq"]),
    ("call:runtime:nat_le", &["prim:nat_le"]),
    ("call:runtime:nat_lt", &["prim:nat_lt"]),
    ("call:runtime:int_add", &["prim:int_add"]),
    ("call:runtime:int_sub", &["prim:int_sub"]),
    ("call:runtime:int_mul", &["prim:int_mul"]),
    ("call:runtime:int_neg", &["prim:int_neg"]),
    ("call:runtime:int_quot", &["prim:int_quot"]),
    ("call:runtime:int_rem", &["prim:int_rem"]),
    ("call:runtime:bool_not", &["prim:bool_not"]),
    ("call:runtime:bool_and", &["prim:bool_and"]),
    ("call:runtime:bool_or", &["prim:bool_or"]),
    ("call:runtime:equal", &["prim:equal"]),
    ("call:runtime:compare", &["prim:compare"]),
    ("call:runtime:checked_add", &["prim:checked_add"]),
    ("call:runtime:checked_sub", &["prim:checked_sub"]),
    ("call:runtime:checked_mul", &["prim:checked_mul"]),
    ("call:runtime:checked_quot", &["prim:checked_quot"]),
    ("call:runtime:checked_neg", &["prim:checked_neg"]),
    ("call:runtime:bit_and", &["prim:bit_and"]),
    ("call:runtime:bit_or", &["prim:bit_or"]),
    ("call:runtime:bit_xor", &["prim:bit_xor"]),
    ("call:runtime:bit_not", &["prim:bit_not"]),
    ("call:runtime:shift_left", &["prim:shift_left"]),
    ("call:runtime:shift_right", &["prim:shift_right"]),
    ("call:runtime:convert", &["prim:convert"]),
    ("call:runtime:append", &["prim:append"]),
    ("call:runtime:length", &["prim:length"]),
    ("call:runtime:index", &["prim:index"]),
    ("call:runtime:slice", &["prim:slice"]),
    ("call:runtime:utf8_encode", &["prim:utf8_encode"]),
    ("call:runtime:utf8_decode", &["prim:utf8_decode"]),
    ("call:runtime:compare_bytes", &["prim:compare_bytes"]),
    ("call:runtime:split_exact", &["prim:split_exact"]),
    ("call:runtime:join", &["prim:join"]),
    ("call:runtime:format_decimal", &["prim:format_decimal"]),
    ("call:runtime:parse_decimal", &["prim:parse_decimal"]),
];

fn row(construct: &str) -> Result<&'static [&'static str], String> {
    CORRESPONDENCE
        .iter()
        .find(|(name, _)| *name == construct)
        .map(|(_, realized)| *realized)
        .ok_or_else(|| {
            format!("the construct `{construct}` has no target-semantics correspondence")
        })
}

/// Check that every construct `krate` emits is in [`CORRESPONDENCE`] and
/// realizes the element it was lowered from, at that element's width, and
/// that the program, whose calculus elements and structural realizations
/// are `elements`, uses that element; and that every type the crate names
/// realizes a type the program uses.
///
/// # Errors
///
/// Returns the first construct with no row, whose row does not name its
/// origin, whose width differs from its origin's, or whose origin the
/// program does not use, or the first type no program element justifies.
pub fn correspond(krate: &Crate, elements: &BTreeSet<String>) -> Result<(), String> {
    let (instances, types) = instances(krate);
    for Instance {
        construct,
        origin,
        width,
    } in &instances
    {
        let realized = row(construct)?;
        if !realized.contains(&origin.element.as_str()) {
            return fail(format!(
                "the construct `{construct}` does not realize `{}`, the element it was lowered from",
                origin.element
            ));
        }
        if !elements.contains(&origin.element) {
            return fail(format!(
                "the construct `{construct}` realizes `{}`, which the program does not use",
                origin.element
            ));
        }
        if width != &origin.width && (width.is_some() || construct.starts_with("call:runtime:")) {
            return fail(format!(
                "the construct `{construct}` works at width {width:?}, but `{}` is at {:?}",
                origin.element, origin.width
            ));
        }
    }
    for construct in types {
        let realized = row(&construct)?;
        if !realized.iter().any(|element| elements.contains(*element)) {
            return fail(format!(
                "the construct `{construct}` realizes none of {realized:?}, which the program does not use"
            ));
        }
    }
    Ok(())
}
