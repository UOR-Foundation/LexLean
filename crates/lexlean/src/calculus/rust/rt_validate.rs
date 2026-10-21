//! The validator of a `RuntimeSyntax` crate (SPEC.md §17.16).
//!
//! `RustSyntax` is validated by `validate`; the runtime's second closed module
//! is validated here, by rules of its own, because a trait, an impl, a
//! generic, a loop, and a mutable borrow are constructs the first module does
//! not have. Every refusal names its rule, and an item or name outside the
//! closed tables is refused by name. The rules are the vocabulary's and the
//! profile's: the types of values are `rustc`'s, which compiles every printed
//! crate under `warnings = deny`.

use std::collections::BTreeMap;

use super::rt::{
    Arm, Block, Crate, Expr, Function, Generic, Item, Pat, Profile, Sig, Type, Variant,
};

/// The types a crate may name without declaring them, with their arity and
/// whether a value of the type allocates.
const TYPES: [(&str, usize, bool); 5] = [
    ("Result", 2, false),
    ("Option", 1, false),
    ("AtomicU64", 0, false),
    ("Rc", 1, true),
    ("Vec", 1, true),
];

/// `String` is a type of the table that allocates and takes no argument.
const STRING: &str = "String";

/// The integer types, whose `from` and `try_from` a crate may call and whose
/// constants it may name.
const INTEGERS: [&str; 10] = [
    "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128",
];

/// The functions a crate may call by path without declaring them, with their
/// arity and whether the call allocates.
const CALLS: [(&str, &str, usize, bool); 12] = [
    ("AtomicU64", "new", 1, false),
    ("Rc", "new", 1, true),
    ("Rc", "from", 1, true),
    ("Rc", "try_unwrap", 1, true),
    ("Rc", "get_mut", 1, true),
    ("Vec", "new", 0, true),
    ("Vec", "with_capacity", 1, true),
    ("String", "new", 0, true),
    ("mem", "replace", 2, false),
    ("str", "from_utf8", 1, false),
    ("usize", "from", 1, false),
    ("usize", "try_from", 1, false),
];

/// The constructors a crate may call without declaring them.
const WRAPPERS: [(&str, usize); 3] = [("Some", 1), ("Ok", 1), ("Err", 1)];

/// The methods a crate may call on a value without declaring them, with the
/// number of arguments after the receiver and whether the call allocates.
const METHODS: [(&str, usize, bool); 27] = [
    ("fetch_add", 2, false),
    ("load", 1, false),
    ("cmp", 1, false),
    ("len", 0, false),
    ("is_empty", 0, false),
    ("chars", 0, false),
    ("next", 0, false),
    ("min", 1, false),
    ("max", 1, false),
    ("push", 1, true),
    ("pop", 0, true),
    ("truncate", 1, true),
    ("push_str", 1, true),
    ("extend_from_slice", 1, true),
    ("iter", 0, false),
    ("rev", 0, false),
    ("get", 1, false),
    ("copied", 0, false),
    ("ok", 0, false),
    ("clone", 0, false),
    ("to_string", 0, true),
    ("as_bytes", 0, false),
    ("strip_prefix", 1, false),
    ("starts_with", 1, false),
    ("split", 1, false),
    ("collect", 0, true),
    ("is_ascii_digit", 0, false),
];

/// The methods that take type arguments: `recv.parse::<T>()`.
const TURBOFISH: [(&str, usize, usize); 2] = [("parse", 1, 0), ("collect", 1, 0)];

/// The traits of the standard library an impl may name, with the one method
/// each states and whether it takes `&mut self`.
const STD_TRAITS: [(&str, &str, bool); 2] = [("Clone", "clone", false), ("Drop", "drop", true)];

/// The names of the closed table of the standard library, each once, in the
/// order the tables above give them: the types, the calls (`Owner::name`),
/// the constructors, the methods, the methods that take type arguments, and
/// the traits an impl may name. `SPEC.md` §17.16 lists them and `RB-19`
/// requires the two to be equal.
#[must_use]
pub fn table() -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    out.extend(TYPES.iter().map(|(name, ..)| (*name).to_owned()));
    out.push(STRING.to_owned());
    out.extend(CALLS.iter().map(|(owner, name, ..)| format!("{owner}::{name}")));
    out.extend(WRAPPERS.iter().map(|(name, _)| (*name).to_owned()));
    out.extend(METHODS.iter().map(|(name, ..)| (*name).to_owned()));
    for (name, ..) in TURBOFISH {
        if !out.iter().any(|held| held == name) {
            out.push(name.to_owned());
        }
    }
    out.extend(STD_TRAITS.iter().map(|(name, ..)| (*name).to_owned()));
    out
}

/// The names of the table whose use allocates, which the core profile refuses.
#[must_use]
pub fn allocating() -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    out.extend(TYPES.iter().filter(|(.., heap)| *heap).map(|(name, ..)| (*name).to_owned()));
    out.push(STRING.to_owned());
    out.extend(
        CALLS
            .iter()
            .filter(|(.., heap)| *heap)
            .map(|(owner, name, ..)| format!("{owner}::{name}")),
    );
    out.extend(METHODS.iter().filter(|(.., heap)| *heap).map(|(name, ..)| (*name).to_owned()));
    out
}

/// The roots a `use` path may start at, by profile. `alloc` is in neither: a
/// crate that names it allocates through a path the table does not list.
fn roots(profile: Profile) -> &'static [&'static str] {
    match profile {
        Profile::Core => &["core"],
        Profile::Std => &["core", "std"],
    }
}

struct Scope<'a> {
    profile: Profile,
    traits: BTreeMap<&'a str, &'a [Sig]>,
    /// A struct: the names of its fields (a tuple struct's are `0`, `1`, …)
    /// and its number of generic parameters.
    structs: BTreeMap<&'a str, (Vec<String>, usize)>,
    enums: BTreeMap<&'a str, (&'a [Variant], usize)>,
    aliases: BTreeMap<&'a str, usize>,
    statics: Vec<&'a str>,
    functions: BTreeMap<&'a str, usize>,
    /// Every method an impl or a trait states, with its number of arguments.
    methods: BTreeMap<&'a str, Vec<usize>>,
    imported: Vec<String>,
}

fn rule<T>(rule: &str, what: impl std::fmt::Display) -> Result<T, String> {
    Err(format!("runtime support: {rule}: {what}"))
}

/// The refusal of a name that allocates in the core profile: the hidden
/// allocation the validator refuses in every `rust-core` crate.
fn heap<T>(what: &str) -> Result<T, String> {
    rule(
        "hidden-allocation",
        format!("rust-core renders `{what}`, which requires heap allocation"),
    )
}

/// Validate `krate`.
///
/// # Errors
///
/// The first rule the crate breaks, named: a `use` outside the profile, a
/// name declared twice or not declared, a trait an impl does not complete, a
/// bound that names no trait, a method a bound does not provide, a variable
/// not in scope, a call of the wrong arity, a `break` outside a loop, a
/// struct literal whose fields are not the struct's, or a name that allocates
/// in the core profile.
pub fn validate(krate: &Crate) -> Result<(), String> {
    let mut scope = Scope {
        profile: krate.profile,
        traits: BTreeMap::new(),
        structs: BTreeMap::new(),
        enums: BTreeMap::new(),
        aliases: BTreeMap::new(),
        statics: Vec::new(),
        functions: BTreeMap::new(),
        methods: BTreeMap::new(),
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
                scope.structs.insert(name, (Vec::new(), 0));
            }
            Item::TupleStruct(name, generics, _, types) => {
                once(&mut declared, name)?;
                let fields = (0..types.len()).map(|at| at.to_string()).collect();
                scope.structs.insert(name, (fields, generics.len()));
            }
            Item::RecordStruct(name, generics, _, fields, types) => {
                once(&mut declared, name)?;
                if fields.len() != types.len() {
                    return rule(
                        "struct-fields",
                        format!(
                            "`{name}` has {} field names and {} types",
                            fields.len(),
                            types.len()
                        ),
                    );
                }
                scope.structs.insert(name, (fields.clone(), generics.len()));
            }
            Item::Enum(name, generics, _, variants) => {
                once(&mut declared, name)?;
                let mut seen = Vec::new();
                for variant in variants {
                    let shown = variant.name();
                    if seen.contains(&shown) {
                        return rule("variant-name", format!("`{name}` declares `{shown}` twice"));
                    }
                    seen.push(shown);
                }
                scope.enums.insert(name, (variants, generics.len()));
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
                for sig in sigs {
                    scope
                        .methods
                        .entry(&sig.name)
                        .or_default()
                        .push(sig.params.len());
                }
            }
            Item::Fn(function) => {
                once(&mut declared, &function.sig.name)?;
                scope
                    .functions
                    .insert(&function.sig.name, function.sig.params.len());
            }
            Item::Impl(_, _, _, functions) => {
                for function in functions {
                    scope
                        .methods
                        .entry(&function.sig.name)
                        .or_default()
                        .push(function.sig.params.len());
                }
            }
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
            Item::TupleStruct(_, generics, _, types) => {
                for ty in types {
                    scope.ty(ty, generics.len(), false)?;
                }
            }
            Item::RecordStruct(_, generics, _, _, types) => {
                for ty in types {
                    scope.ty(ty, generics.len(), false)?;
                }
            }
            Item::Enum(_, generics, _, variants) => {
                for variant in variants {
                    for ty in variant.types() {
                        scope.ty(ty, generics.len(), false)?;
                    }
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

impl Variant {
    fn name(&self) -> &str {
        match self {
            Variant::Unit(name) | Variant::Tuple(name, _) | Variant::Record(name, ..) => name,
        }
    }

    fn types(&self) -> &[Type] {
        match self {
            Variant::Unit(_) => &[],
            Variant::Tuple(_, types) | Variant::Record(_, _, types) => types,
        }
    }
}

fn once<'a>(declared: &mut Vec<&'a str>, name: &'a str) -> Result<(), String> {
    if declared.contains(&name) {
        return rule("duplicate-name", format!("`{name}` is declared twice"));
    }
    declared.push(name);
    Ok(())
}

/// What is in scope in a body: the generics (impl, then function), the
/// variables with their mutability and type, the type `Self` stands for, and
/// whether a `break` has a loop to leave.
struct Frame<'a> {
    generics: Vec<&'a Generic>,
    variables: Vec<(String, bool, Option<Type>)>,
    selfs: Option<&'a Type>,
    in_loop: bool,
}

impl<'a> Frame<'a> {
    fn empty() -> Self {
        Frame {
            generics: Vec::new(),
            variables: Vec::new(),
            selfs: None,
            in_loop: false,
        }
    }

    fn with(&self, bound: Vec<(String, bool)>, in_loop: bool) -> Frame<'a> {
        let mut variables = self.variables.clone();
        variables.extend(
            bound
                .into_iter()
                .map(|(name, mutable)| (name, mutable, None)),
        );
        Frame {
            generics: self.generics.clone(),
            variables,
            selfs: self.selfs,
            in_loop: self.in_loop || in_loop,
        }
    }
}

/// The names a pattern binds, each with whether it is `mut`; a name bound
/// twice is refused.
fn binds(pattern: &Pat, out: &mut Vec<(String, bool)>) -> Result<(), String> {
    let one = |name: &String, mutable: bool, out: &mut Vec<(String, bool)>| {
        if out.iter().any(|(held, _)| held == name) {
            return rule(
                "pattern-duplicate",
                format!("`{name}` is bound twice in one pattern"),
            );
        }
        out.push((name.clone(), mutable));
        Ok(())
    };
    match pattern {
        Pat::Wild | Pat::Path(_) | Pat::Lit(_) => Ok(()),
        Pat::Bind(name) => one(name, false, out),
        Pat::BindMut(name) => one(name, true, out),
        Pat::Tuple(items) | Pat::TupleStruct(_, items) | Pat::Struct(_, _, items) => {
            items.iter().try_for_each(|item| binds(item, out))
        }
    }
}

impl<'a> Scope<'a> {
    fn generics(&self, generics: &[Generic]) -> Result<(), String> {
        for generic in generics {
            for bound in &generic.bounds {
                if !self.traits.contains_key(bound.as_str())
                    && !STD_TRAITS.iter().any(|(known, ..)| known == bound)
                {
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
            Type::Unit
            | Type::Bool
            | Type::Nat
            | Type::Fixed(_)
            | Type::Ordering
            | Type::Wide(_)
            | Type::Size
            | Type::Char
            | Type::Str => Ok(()),
            Type::SelfType if has_self => Ok(()),
            Type::SelfType => rule("self-type", "`Self` outside a trait or an impl"),
            Type::Param(at) if usize::try_from(*at).is_ok_and(|at| at < params) => Ok(()),
            Type::Param(at) => rule(
                "type-parameter",
                format!("parameter {at} of {params} in scope"),
            ),
            Type::Ref(inner) | Type::RefMut(inner) | Type::Slice(inner) => {
                self.ty(inner, params, has_self)
            }
            Type::Pair(a, b) => {
                self.ty(a, params, has_self)?;
                self.ty(b, params, has_self)
            }
            Type::Named(name, arguments) => {
                let expected = if let Some((_, generics)) = self.structs.get(name.as_str()) {
                    *generics
                } else if let Some((_, generics)) = self.enums.get(name.as_str()) {
                    *generics
                } else if let Some(count) = self.aliases.get(name.as_str()) {
                    *count
                } else if let Some((_, count, allocates)) =
                    TYPES.iter().find(|(known, ..)| known == name)
                {
                    if *allocates && self.profile == Profile::Core {
                        return heap(name);
                    }
                    *count
                } else if name == STRING {
                    if self.profile == Profile::Core {
                        return heap(name);
                    }
                    0
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
        if sig.exclusive && !sig.receiver {
            return rule(
                "function-receiver",
                format!("`{}` is exclusive without a receiver", sig.name),
            );
        }
        for param in &sig.params {
            self.ty(&param.ty, params, has_self)?;
        }
        self.ty(&sig.result, params, has_self)
    }

    /// Whether the bounds give a method called `method`.
    fn provided(&self, bounds: &[String], method: &str) -> bool {
        bounds.iter().any(|bound| {
            self.traits
                .get(bound.as_str())
                .is_some_and(|sigs| sigs.iter().any(|sig| sig.name == method))
                || STD_TRAITS
                    .iter()
                    .any(|(known, provided, _)| known == bound && *provided == method)
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
            let held = if function.sig.exclusive {
                Type::RefMut(Box::new(selfs.clone()))
            } else {
                Type::Ref(Box::new(selfs.clone()))
            };
            variables.push(("self".to_owned(), function.sig.exclusive, Some(held)));
        }
        for param in &function.sig.params {
            if variables.iter().any(|(held, ..)| held == &param.name) {
                return rule(
                    "param-duplicate",
                    format!(
                        "`{}` names two parameters of `{}`",
                        param.name, function.sig.name
                    ),
                );
            }
            variables.push((param.name.clone(), param.mutable, Some(param.ty.clone())));
        }
        let frame = Frame {
            generics,
            variables,
            selfs,
            in_loop: false,
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
        self.generics(generics)?;
        let std_sig;
        let sigs: &[Sig] = if let Some(sigs) = self.traits.get(tr) {
            sigs
        } else if let Some((_, method, exclusive)) =
            STD_TRAITS.iter().find(|(known, ..)| *known == tr)
        {
            std_sig = [Sig {
                name: (*method).to_owned(),
                receiver: true,
                exclusive: *exclusive,
                params: Vec::new(),
                result: if *exclusive {
                    Type::Unit
                } else {
                    Type::SelfType
                },
            }];
            &std_sig
        } else {
            return rule("impl-trait", format!("`impl {tr}` names no declared trait"));
        };
        for sig in sigs {
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

    /// The statements in order, each `let` binding for the rest, then the
    /// tail.
    fn block(&self, block: &Block, frame: &Frame<'a>) -> Result<(), String> {
        let mut inner = frame.with(Vec::new(), false);
        for statement in &block.statements {
            match statement {
                Expr::Let(pattern, ty, value) => {
                    if let Some(ty) = ty {
                        self.ty(ty, inner.generics.len(), inner.selfs.is_some())?;
                    }
                    self.expr(value, &inner)?;
                    let mut bound = Vec::new();
                    binds(pattern, &mut bound)?;
                    self.pattern(pattern)?;
                    inner = inner.with(bound, false);
                }
                other => self.expr(other, &inner)?,
            }
        }
        self.expr(&block.tail, &inner)
    }

    /// The type of `expr`, when the closed rules can say.
    fn type_of(&self, expr: &Expr, frame: &Frame<'_>) -> Option<Type> {
        match expr {
            Expr::Var(name) => frame
                .variables
                .iter()
                .rev()
                .find(|(held, ..)| held == name)
                .and_then(|(_, _, ty)| ty.clone()),
            Expr::Borrow(inner) => self.type_of(inner, frame).map(|ty| Type::Ref(Box::new(ty))),
            Expr::Field(inner, at) => {
                let mut ty = self.type_of(inner, frame)?;
                while let Type::Ref(inner) | Type::RefMut(inner) = ty {
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

    /// The root variable of a place and whether the place may be assigned.
    fn assignable(&self, place: &Expr, frame: &Frame<'_>) -> Result<(), String> {
        let mut at = place;
        let mut through = false;
        loop {
            match at {
                Expr::Field(inner, _) | Expr::FieldNamed(inner, _) | Expr::Index(inner, _) => {
                    at = inner;
                }
                Expr::Deref(inner) => {
                    through = true;
                    at = inner;
                }
                Expr::Var(name) => {
                    let Some((_, mutable, ty)) =
                        frame.variables.iter().rev().find(|(held, ..)| held == name)
                    else {
                        return rule("variable-scope", format!("`{name}` is not in scope"));
                    };
                    let exclusive = matches!(ty, Some(Type::RefMut(_)));
                    let _ = through;
                    return if *mutable || exclusive {
                        Ok(())
                    } else {
                        rule(
                            "assign-immutable",
                            format!("`{name}` is assigned and is not `mut` or `&mut`"),
                        )
                    };
                }
                _ => return rule("assign-place", "an assignment to a value that is no place"),
            }
        }
    }

    fn path(&self, path: &[String]) -> Result<(), String> {
        let head = path.first().map(String::as_str).unwrap_or_default();
        if let Some((variants, _)) = self.enums.get(head) {
            return match path.get(1) {
                Some(variant) if variants.iter().any(|held| held.name() == variant) => Ok(()),
                _ => rule(
                    "variant-name",
                    format!("`{}` is not a variant of `{head}`", path.join("::")),
                ),
            };
        }
        if self.imported.iter().any(|name| name == head)
            || self.structs.contains_key(head)
            || self.statics.contains(&head)
            || INTEGERS.contains(&head)
            || matches!(head, "None" | "Self" | "usize")
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

    fn pattern(&self, pattern: &Pat) -> Result<(), String> {
        match pattern {
            Pat::Wild | Pat::Bind(_) | Pat::BindMut(_) | Pat::Lit(_) => Ok(()),
            Pat::Path(path) => self.path(path),
            Pat::Tuple(items) => items.iter().try_for_each(|item| self.pattern(item)),
            Pat::TupleStruct(path, items) => {
                if !(path.len() == 1 && WRAPPERS.iter().any(|(known, _)| known == &path[0])) {
                    self.path(path)?;
                    if let (Some(variants), Some(variant)) = (
                        path.first().and_then(|head| self.enums.get(head.as_str())),
                        path.get(1),
                    ) {
                        let arity = variants
                            .0
                            .iter()
                            .find(|held| held.name() == variant)
                            .map(|held| held.types().len());
                        if arity != Some(items.len()) {
                            return rule(
                                "pattern-fields",
                                format!(
                                    "`{}` is matched with {} patterns",
                                    path.join("::"),
                                    items.len()
                                ),
                            );
                        }
                    }
                }
                items.iter().try_for_each(|item| self.pattern(item))
            }
            Pat::Struct(path, names, items) => {
                self.path(path)?;
                if names.len() != items.len() {
                    return rule(
                        "pattern-fields",
                        "a struct pattern with unequal names and patterns",
                    );
                }
                if let Some(held) = self.record_fields(path) {
                    if let Some(extra) = names.iter().find(|name| !held.contains(name)) {
                        return rule(
                            "pattern-fields",
                            format!("`{}` has no field `{extra}`", path.join("::")),
                        );
                    }
                }
                items.iter().try_for_each(|item| self.pattern(item))
            }
        }
    }

    /// The field names of the struct or record variant a path names.
    fn record_fields(&self, path: &[String]) -> Option<Vec<String>> {
        match path {
            [name] => self
                .structs
                .get(name.as_str())
                .map(|(fields, _)| fields.clone()),
            [owner, variant] => self.enums.get(owner.as_str()).and_then(|(variants, _)| {
                variants.iter().find_map(|held| match held {
                    Variant::Record(name, fields, _) if name == variant => Some(fields.clone()),
                    _ => None,
                })
            }),
            _ => None,
        }
    }

    fn expr(&self, expr: &Expr, frame: &Frame<'a>) -> Result<(), String> {
        match expr {
            Expr::Lit(_) | Expr::Break => {
                if matches!(expr, Expr::Break) && !frame.in_loop {
                    return rule("break-position", "a `break` outside a loop");
                }
                Ok(())
            }
            Expr::Var(name) => {
                if frame.variables.iter().any(|(held, ..)| held == name)
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
                    [name] => self
                        .functions
                        .get(name.as_str())
                        .copied()
                        .or_else(|| {
                            WRAPPERS
                                .iter()
                                .find(|(known, _)| known == name)
                                .map(|(_, count)| *count)
                        })
                        .or_else(|| self.tuple_struct_arity(name)),
                    [owner, name] => {
                        let table = CALLS
                            .iter()
                            .find(|(o, n, ..)| o == owner && n == name)
                            .or_else(|| {
                                INTEGERS
                                    .contains(&owner.as_str())
                                    .then(|| {
                                        CALLS.iter().find(|(o, n, ..)| *o == "usize" && n == name)
                                    })
                                    .flatten()
                            });
                        if let Some((_, _, count, allocates)) = table {
                            if *allocates && self.profile == Profile::Core {
                                return heap(&path.join("::"));
                            }
                            Some(*count)
                        } else {
                            self.variant_arity(owner, name)
                        }
                    }
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
                self.method(receiver, name, arguments.len(), frame)?;
                arguments
                    .iter()
                    .try_for_each(|argument| self.expr(argument, frame))
            }
            Expr::MethodOf(receiver, name, types, arguments) => {
                self.expr(receiver, frame)?;
                let Some((_, count, args)) = TURBOFISH.iter().find(|(known, ..)| known == name)
                else {
                    return rule(
                        "method-name",
                        format!("`{name}` takes no type arguments in the closed table"),
                    );
                };
                if types.len() != *count || arguments.len() != *args {
                    return rule(
                        "method-arity",
                        format!("`{name}` takes {count} type arguments and {args} arguments"),
                    );
                }
                if *name == "collect" && self.profile == Profile::Core {
                    return heap(name);
                }
                for ty in types {
                    self.ty(ty, frame.generics.len(), frame.selfs.is_some())?;
                }
                arguments
                    .iter()
                    .try_for_each(|argument| self.expr(argument, frame))
            }
            Expr::Field(inner, _) | Expr::FieldNamed(inner, _) | Expr::Borrow(inner) => {
                self.expr(inner, frame)
            }
            Expr::Deref(inner) | Expr::BorrowMut(inner) | Expr::Not(inner) => {
                self.expr(inner, frame)
            }
            Expr::Equals(a, b) | Expr::Binary(_, a, b) | Expr::Index(a, b) => {
                self.expr(a, frame)?;
                self.expr(b, frame)
            }
            Expr::Cast(inner, ty) => {
                self.ty(ty, frame.generics.len(), frame.selfs.is_some())?;
                self.expr(inner, frame)
            }
            Expr::RangeSlice(inner, from, to) => {
                self.expr(inner, frame)?;
                for bound in [from, to].into_iter().flatten() {
                    self.expr(bound, frame)?;
                }
                Ok(())
            }
            Expr::Let(..) => rule("let-position", "a `let` that is not a statement of a block"),
            Expr::Assign(place, value) => {
                self.assignable(place, frame)?;
                self.expr(place, frame)?;
                self.expr(value, frame)
            }
            Expr::If(condition, then, otherwise) => {
                self.expr(condition, frame)?;
                self.block(then, frame)?;
                self.block(otherwise, frame)
            }
            Expr::While(condition, body) => {
                self.expr(condition, frame)?;
                self.block(body, &frame.with(Vec::new(), true))
            }
            Expr::Loop(body) => self.block(body, &frame.with(Vec::new(), true)),
            Expr::WhileLet(pattern, value, body) | Expr::For(pattern, value, body) => {
                self.expr(value, frame)?;
                self.pattern(pattern)?;
                let mut bound = Vec::new();
                binds(pattern, &mut bound)?;
                self.block(body, &frame.with(bound, true))
            }
            Expr::Return(value) => value
                .as_ref()
                .map_or(Ok(()), |value| self.expr(value, frame)),
            Expr::Tuple(items) => {
                if items.len() < 2 {
                    return rule("tuple-arity", format!("a tuple of {}", items.len()));
                }
                items.iter().try_for_each(|item| self.expr(item, frame))
            }
            Expr::StructLit(path, names, values) => {
                self.path(path)?;
                let Some(held) = self.record_fields(path) else {
                    return rule(
                        "struct-fields",
                        format!(
                            "`{}` is no record struct or record variant",
                            path.join("::")
                        ),
                    );
                };
                let mut given = names.clone();
                given.sort();
                let mut declared = held;
                declared.sort();
                if given != declared || names.len() != values.len() {
                    return rule(
                        "struct-fields",
                        format!("`{}` is given {names:?}, not {declared:?}", path.join("::")),
                    );
                }
                values.iter().try_for_each(|value| self.expr(value, frame))
            }
            Expr::Match(scrutinee, arms) => {
                self.expr(scrutinee, frame)?;
                if arms.is_empty() {
                    return rule("match-arms", "a match with no arm");
                }
                for Arm {
                    pattern,
                    guard,
                    body,
                } in arms
                {
                    self.pattern(pattern)?;
                    let mut bound = Vec::new();
                    binds(pattern, &mut bound)?;
                    let inner = frame.with(bound, false);
                    if let Some(guard) = guard {
                        self.expr(guard, &inner)?;
                    }
                    self.expr(body, &inner)?;
                }
                Ok(())
            }
            Expr::Block(block) => self.block(block, frame),
        }
    }

    fn tuple_struct_arity(&self, name: &str) -> Option<usize> {
        self.structs.get(name).map(|(fields, _)| fields.len())
    }

    fn variant_arity(&self, owner: &str, name: &str) -> Option<usize> {
        self.enums.get(owner).and_then(|(variants, _)| {
            variants.iter().find_map(|held| match held {
                Variant::Tuple(variant, types) if variant == name => Some(types.len()),
                _ => None,
            })
        })
    }

    /// A method of the closed table, or of a trait or impl the crate
    /// declares, called on a value whose type is a parameter that a bound
    /// provides it for, or on a value the impls cover.
    fn method(
        &self,
        receiver: &Expr,
        name: &str,
        given: usize,
        frame: &Frame<'_>,
    ) -> Result<(), String> {
        if let Some((_, count, allocates)) = METHODS.iter().find(|(known, ..)| *known == name) {
            if *allocates && self.profile == Profile::Core {
                return heap(name);
            }
            if *count != given {
                return rule(
                    "method-arity",
                    format!("`{name}` takes {count} arguments, given {given}"),
                );
            }
            return Ok(());
        }
        let Some(counts) = self.methods.get(name) else {
            return rule(
                "method-name",
                format!(
                    "`{name}` is a method of no declared trait or impl and not in the closed table"
                ),
            );
        };
        if !counts.contains(&given) {
            return rule(
                "method-arity",
                format!("`{name}` takes {counts:?} arguments, given {given}"),
            );
        }
        let mut ty = self.type_of(receiver, frame);
        while let Some(Type::Ref(inner) | Type::RefMut(inner)) = ty {
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
}
