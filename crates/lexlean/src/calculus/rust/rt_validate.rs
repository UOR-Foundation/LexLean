//! The validator of a `RuntimeSyntax` crate (SPEC.md §17.16).
//!
//! `RustSyntax` is validated by `validate`; the runtime's second closed module
//! is validated here, by rules of its own, because a trait, an impl, and a
//! generic are constructs the first module does not have. Every refusal names
//! its rule, and an item or name outside the closed tables is refused by name.

use std::collections::BTreeMap;

use super::rt::{Block, Crate, Expr, Function, Generic, Item, Pat, Profile, Sig, Type};

/// The types a crate may name without declaring them, with their arity.
const TYPES: [(&str, usize); 2] = [("Result", 2), ("AtomicU64", 0)];

/// The functions a crate may call by path without declaring them, with their
/// arity.
const CALLS: [(&str, &str, usize); 1] = [("AtomicU64", "new", 1)];

/// The methods a crate may call on a value without declaring them, with the
/// number of arguments after the receiver.
const METHODS: [(&str, usize); 3] = [("fetch_add", 2), ("load", 1), ("cmp", 1)];

/// The roots a `use` path may start at, by profile. `alloc` is in neither: a
/// crate that names it allocates, and the core profile allocates nothing.
fn roots(profile: Profile) -> &'static [&'static str] {
    match profile {
        Profile::Core => &["core"],
        Profile::Std => &["core", "std"],
    }
}

struct Scope<'a> {
    traits: BTreeMap<&'a str, &'a [Sig]>,
    structs: Vec<&'a str>,
    aliases: BTreeMap<&'a str, usize>,
    statics: Vec<&'a str>,
    functions: BTreeMap<&'a str, usize>,
    imported: Vec<String>,
}

fn rule<T>(rule: &str, what: impl std::fmt::Display) -> Result<T, String> {
    Err(format!("runtime support: {rule}: {what}"))
}

/// Validate `krate`.
///
/// # Errors
///
/// The first rule the crate breaks, named: a `use` outside the profile, a name
/// declared twice or not declared, a trait an impl does not complete, a bound
/// that names no trait, a method a bound does not provide, a variable not in
/// scope, or a call of the wrong arity.
pub fn validate(krate: &Crate) -> Result<(), String> {
    let mut scope = Scope {
        traits: BTreeMap::new(),
        structs: Vec::new(),
        aliases: BTreeMap::new(),
        statics: Vec::new(),
        functions: BTreeMap::new(),
        imported: Vec::new(),
    };
    let mut declared = Vec::<&str>::new();
    for item in &krate.items {
        match item {
            Item::Use(path, alias) => {
                let Some(root) = path.first() else {
                    return rule("use-path", "empty");
                };
                if !roots(krate.profile).contains(&root.as_str()) {
                    return rule(
                        "profile-root",
                        format!(
                            "`use {}` starts at `{root}`, which the profile does not carry",
                            path.join("::")
                        ),
                    );
                }
                let last = alias.as_ref().or_else(|| path.last());
                scope.imported.extend(last.cloned());
            }
            Item::UnitStruct(name, _) => {
                once(&mut declared, name)?;
                scope.structs.push(name);
            }
            Item::Alias(name, parameters, _) => {
                once(&mut declared, name)?;
                scope.aliases.insert(name, parameters.len());
            }
            Item::Static(name, _, _) => {
                once(&mut declared, name)?;
                scope.statics.push(name);
            }
            Item::Trait(name, sigs) => {
                once(&mut declared, name)?;
                scope.traits.insert(name, sigs);
            }
            Item::Fn(function) => {
                once(&mut declared, &function.sig.name)?;
                scope
                    .functions
                    .insert(&function.sig.name, function.sig.params.len());
            }
            Item::Impl(..) => {}
        }
    }
    for item in &krate.items {
        match item {
            Item::Use(..) | Item::UnitStruct(..) => {}
            Item::Alias(_, parameters, ty) => {
                scope.ty(ty, parameters.len(), false)?;
            }
            Item::Static(_, ty, init) => {
                scope.ty(ty, 0, false)?;
                scope.expr(init, &Frame::empty())?;
            }
            Item::Trait(name, sigs) => {
                let mut seen = Vec::new();
                for sig in sigs {
                    if seen.contains(&sig.name.as_str()) {
                        return rule(
                            "trait-method",
                            format!("`{name}` declares `{}` twice", sig.name),
                        );
                    }
                    seen.push(&sig.name);
                    scope.sig(sig, 0, true)?;
                }
            }
            Item::Fn(function) => {
                if !function.public {
                    return rule(
                        "function-visibility",
                        format!(
                            "`{}` is a free function that is not public",
                            function.sig.name
                        ),
                    );
                }
                if function.sig.receiver {
                    return rule(
                        "function-receiver",
                        format!("free function `{}` takes `&self`", function.sig.name),
                    );
                }
                scope.function(function, &[], None)?;
            }
            Item::Impl(generics, tr, ty, functions) => {
                scope.generics(generics)?;
                scope.ty(ty, generics.len(), false)?;
                match tr {
                    Some(tr) => scope.completes(tr, generics, ty, functions)?,
                    None => {
                        for function in functions {
                            scope.function(function, generics, Some(ty))?;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn once<'a>(declared: &mut Vec<&'a str>, name: &'a str) -> Result<(), String> {
    if declared.contains(&name) {
        return rule("duplicate-name", format!("`{name}` is declared twice"));
    }
    declared.push(name);
    Ok(())
}

/// What is in scope in a body: the generics (impl, then function), the
/// variables with their types, and the type `Self` stands for.
struct Frame<'a> {
    generics: Vec<&'a Generic>,
    variables: Vec<(String, Option<Type>)>,
    selfs: Option<&'a Type>,
}

impl Frame<'_> {
    fn empty() -> Self {
        Frame {
            generics: Vec::new(),
            variables: Vec::new(),
            selfs: None,
        }
    }
}

impl<'a> Scope<'a> {
    fn generics(&self, generics: &[Generic]) -> Result<(), String> {
        for generic in generics {
            for bound in &generic.bounds {
                if !self.traits.contains_key(bound.as_str()) {
                    return rule(
                        "bound-trait",
                        format!("`{}: {bound}` names no declared trait", generic.name),
                    );
                }
            }
        }
        Ok(())
    }

    /// `ty` is well formed with `params` generic parameters in scope.
    fn ty(&self, ty: &Type, params: usize, has_self: bool) -> Result<(), String> {
        match ty {
            Type::Unit | Type::Bool | Type::Nat | Type::Fixed(_) | Type::Ordering => Ok(()),
            Type::SelfType if has_self => Ok(()),
            Type::SelfType => rule("self-type", "`Self` outside a trait or an impl"),
            Type::Param(at) if usize::try_from(*at).is_ok_and(|at| at < params) => Ok(()),
            Type::Param(at) => rule(
                "type-parameter",
                format!("parameter {at} of {params} in scope"),
            ),
            Type::Ref(inner) => self.ty(inner, params, has_self),
            Type::Pair(a, b) => {
                self.ty(a, params, has_self)?;
                self.ty(b, params, has_self)
            }
            Type::Named(name, arguments) => {
                let expected = if self.structs.contains(&name.as_str()) {
                    0
                } else if let Some(count) = self.aliases.get(name.as_str()) {
                    *count
                } else if let Some((_, count)) = TYPES.iter().find(|(known, _)| known == name) {
                    *count
                } else {
                    return rule(
                        "type-name",
                        format!("`{name}` is neither declared nor in the closed table"),
                    );
                };
                if expected != arguments.len() {
                    return rule(
                        "type-arity",
                        format!(
                            "`{name}` takes {expected} arguments, given {}",
                            arguments.len()
                        ),
                    );
                }
                arguments
                    .iter()
                    .try_for_each(|argument| self.ty(argument, params, has_self))
            }
        }
    }

    fn sig(&self, sig: &Sig, params: usize, has_self: bool) -> Result<(), String> {
        for param in &sig.params {
            self.ty(&param.ty, params, has_self)?;
        }
        self.ty(&sig.result, params, has_self)
    }

    /// The trait methods named `method` among those the bounds of `ty` give.
    fn provided(&self, bounds: &[String], method: &str) -> bool {
        bounds.iter().any(|bound| {
            self.traits
                .get(bound.as_str())
                .is_some_and(|sigs| sigs.iter().any(|sig| sig.name == method))
        })
    }

    fn function(
        &self,
        function: &'a Function,
        outer: &'a [Generic],
        selfs: Option<&'a Type>,
    ) -> Result<(), String> {
        self.generics(&function.generics)?;
        let generics: Vec<&Generic> = outer.iter().chain(&function.generics).collect();
        self.sig(&function.sig, generics.len(), selfs.is_some())?;
        let mut variables = Vec::new();
        if function.sig.receiver {
            let Some(selfs) = selfs else {
                return rule(
                    "function-receiver",
                    format!("`{}` takes `&self` outside an impl", function.sig.name),
                );
            };
            variables.push(("self".to_owned(), Some(Type::Ref(Box::new(selfs.clone())))));
        }
        for param in &function.sig.params {
            variables.push((param.name.clone(), Some(param.ty.clone())));
        }
        let frame = Frame {
            generics,
            variables,
            selfs,
        };
        self.block(&function.body, &frame)
    }

    /// An impl of `tr` states each method of the trait once with the trait's
    /// signature, and nothing else, and no method is `pub`.
    fn completes(
        &self,
        tr: &str,
        generics: &'a [Generic],
        ty: &'a Type,
        functions: &'a [Function],
    ) -> Result<(), String> {
        let Some(sigs) = self.traits.get(tr) else {
            return rule("impl-trait", format!("`impl {tr}` names no declared trait"));
        };
        for sig in *sigs {
            let mut found = functions
                .iter()
                .filter(|function| function.sig.name == sig.name);
            match (found.next(), found.next()) {
                (Some(function), None) if function.sig == *sig => {}
                (Some(_), None) => {
                    return rule(
                        "impl-signature",
                        format!(
                            "`impl {tr}` states `{}` with a signature other than the trait's",
                            sig.name
                        ),
                    );
                }
                (None, _) => {
                    return rule(
                        "impl-complete",
                        format!("`impl {tr}` does not state `{}`", sig.name),
                    );
                }
                _ => {
                    return rule(
                        "impl-complete",
                        format!("`impl {tr}` states `{}` twice", sig.name),
                    );
                }
            }
        }
        for function in functions {
            if !sigs.iter().any(|sig| sig.name == function.sig.name) {
                return rule(
                    "impl-extra",
                    format!(
                        "`impl {tr}` states `{}`, which the trait does not",
                        function.sig.name
                    ),
                );
            }
            if function.public {
                return rule(
                    "impl-visibility",
                    format!(
                        "`{}` of `impl {tr}` is public; a trait method has the trait's visibility",
                        function.sig.name
                    ),
                );
            }
            self.function(function, generics, Some(ty))?;
        }
        Ok(())
    }

    fn block(&self, block: &Block, frame: &Frame<'_>) -> Result<(), String> {
        for statement in &block.statements {
            self.expr(statement, frame)?;
        }
        self.expr(&block.tail, frame)
    }

    /// The type of `expr`, when the closed rules can say.
    fn type_of(&self, expr: &Expr, frame: &Frame<'_>) -> Option<Type> {
        match expr {
            Expr::Var(name) => frame
                .variables
                .iter()
                .rev()
                .find(|(held, _)| held == name)
                .and_then(|(_, ty)| ty.clone()),
            Expr::Borrow(inner) => self.type_of(inner, frame).map(|ty| Type::Ref(Box::new(ty))),
            Expr::Field(inner, at) => {
                let mut ty = self.type_of(inner, frame)?;
                while let Type::Ref(inner) = ty {
                    ty = *inner;
                }
                if ty == Type::SelfType {
                    ty = frame.selfs?.clone();
                }
                match (ty, at) {
                    (Type::Pair(a, _), 0) => Some(*a),
                    (Type::Pair(_, b), 1) => Some(*b),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn expr(&self, expr: &Expr, frame: &Frame<'_>) -> Result<(), String> {
        match expr {
            Expr::Lit(_) => Ok(()),
            Expr::Var(name) => {
                if frame.variables.iter().any(|(held, _)| held == name)
                    || self.statics.contains(&name.as_str())
                {
                    Ok(())
                } else {
                    rule("variable-scope", format!("`{name}` is not in scope"))
                }
            }
            Expr::Path(path) => self.path(path),
            Expr::Call(path, arguments) => {
                let expected = match path.as_slice() {
                    [name] => self.functions.get(name.as_str()).copied(),
                    [owner, name] => CALLS
                        .iter()
                        .find(|(o, n, _)| o == owner && n == name)
                        .map(|(_, _, count)| *count),
                    _ => None,
                };
                let Some(expected) = expected else {
                    return rule(
                        "call-name",
                        format!(
                            "`{}` is neither declared nor in the closed table",
                            path.join("::")
                        ),
                    );
                };
                if expected != arguments.len() {
                    return rule(
                        "call-arity",
                        format!(
                            "`{}` takes {expected} arguments, given {}",
                            path.join("::"),
                            arguments.len()
                        ),
                    );
                }
                arguments
                    .iter()
                    .try_for_each(|argument| self.expr(argument, frame))
            }
            Expr::Method(receiver, name, arguments) => {
                self.expr(receiver, frame)?;
                if let Some((_, count)) = METHODS.iter().find(|(known, _)| known == name) {
                    if *count != arguments.len() {
                        return rule(
                            "method-arity",
                            format!(
                                "`{name}` takes {count} arguments, given {}",
                                arguments.len()
                            ),
                        );
                    }
                } else {
                    self.trait_method(receiver, name, arguments.len(), frame)?;
                }
                arguments
                    .iter()
                    .try_for_each(|argument| self.expr(argument, frame))
            }
            Expr::Field(inner, _) | Expr::Borrow(inner) => self.expr(inner, frame),
            Expr::Equals(a, b) => {
                self.expr(a, frame)?;
                self.expr(b, frame)
            }
            Expr::Match(scrutinee, arms) => {
                self.expr(scrutinee, frame)?;
                if arms.is_empty() {
                    return rule("match-arms", "a match with no arm");
                }
                for (pattern, value) in arms {
                    let mut inner = Frame {
                        generics: frame.generics.clone(),
                        variables: frame.variables.clone(),
                        selfs: frame.selfs,
                    };
                    match pattern {
                        Pat::Wild => {}
                        Pat::Bind(name) => inner.variables.push((name.clone(), None)),
                        Pat::Path(path) => self.path(path)?,
                    }
                    self.expr(value, &inner)?;
                }
                Ok(())
            }
        }
    }

    /// A method of a declared trait, called on a value whose type is a
    /// parameter that a bound provides it for, or on a scalar the impls cover.
    fn trait_method(
        &self,
        receiver: &Expr,
        name: &str,
        given: usize,
        frame: &Frame<'_>,
    ) -> Result<(), String> {
        let Some(sig) = self
            .traits
            .values()
            .flat_map(|sigs| sigs.iter())
            .find(|sig| sig.name == name)
        else {
            return rule(
                "method-name",
                format!("`{name}` is a method of no declared trait and not in the closed table"),
            );
        };
        if sig.params.len() != given {
            return rule(
                "method-arity",
                format!(
                    "`{name}` takes {} arguments, given {given}",
                    sig.params.len()
                ),
            );
        }
        let mut ty = self.type_of(receiver, frame);
        while let Some(Type::Ref(inner)) = ty {
            ty = Some(*inner);
        }
        if let Some(Type::Param(at)) = ty {
            let bounds = usize::try_from(at)
                .ok()
                .and_then(|at| frame.generics.get(at))
                .map_or(&[][..], |generic| generic.bounds.as_slice());
            if !self.provided(bounds, name) {
                return rule(
                    "method-bound",
                    format!("`{name}` is called on a parameter whose bounds do not provide it"),
                );
            }
        }
        Ok(())
    }

    fn path(&self, path: &[String]) -> Result<(), String> {
        let head = path.first().map(String::as_str).unwrap_or_default();
        if self.imported.iter().any(|name| name == head)
            || self.structs.contains(&head)
            || self.statics.contains(&head)
        {
            Ok(())
        } else {
            rule(
                "path-name",
                format!(
                    "`{}` starts at `{head}`, which no `use` or item declares",
                    path.join("::")
                ),
            )
        }
    }
}
