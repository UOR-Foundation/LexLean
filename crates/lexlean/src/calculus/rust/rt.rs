//! `RuntimeSyntax` (SPEC.md §17.16): the closed set of constructs the runtime's
//! support items are written in, and the printer that gives each its Rust text.
//!
//! The set is a second module beside `RustSyntax`, not a widening of it: a
//! generated program never contains a trait, an impl, or a generic, and the
//! calculus that proves a generated program correct is not asked about one.
//! What the runtime needs beyond that is exactly what is listed here, so an
//! item outside the set cannot be written, decoded, or printed.

use super::super::IntKind;

/// The profile of the crate the items belong to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Profile {
    /// `core` only: nothing the crate holds allocates.
    Core,
    /// `core` and `std`.
    Std,
}

/// A type of the closed set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Unit,
    Bool,
    Nat,
    Fixed(IntKind),
    Ordering,
    /// `Self`, inside a trait or an impl.
    SelfType,
    /// The generic parameter at an index of the generics in scope: those of
    /// the impl, then those of the function.
    Param(u64),
    Ref(Box<Type>),
    Pair(Box<Type>, Box<Type>),
    /// A declared struct or alias, or a type of the closed table.
    Named(String, Vec<Type>),
}

/// A literal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lit {
    Unit,
    Bool(bool),
    Nat(u64),
}

/// A pattern.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pat {
    Wild,
    Bind(String),
    Path(Vec<String>),
}

/// An expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Var(String),
    Lit(Lit),
    Path(Vec<String>),
    Call(Vec<String>, Vec<Expr>),
    Method(Box<Expr>, String, Vec<Expr>),
    Field(Box<Expr>, u64),
    Borrow(Box<Expr>),
    Equals(Box<Expr>, Box<Expr>),
    Match(Box<Expr>, Vec<(Pat, Expr)>),
}

/// A function body: statements whose values are dropped, then the value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub statements: Vec<Expr>,
    pub tail: Expr,
}

/// A generic parameter and the traits that bound it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Generic {
    pub name: String,
    pub bounds: Vec<String>,
}

/// A parameter other than the receiver.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
    pub name: String,
    pub ty: Type,
}

/// A function's signature, which a trait states and an impl repeats.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sig {
    pub name: String,
    /// Takes `&self`.
    pub receiver: bool,
    pub params: Vec<Param>,
    pub result: Type,
}

/// A function with its body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub sig: Sig,
    pub generics: Vec<Generic>,
    pub public: bool,
    pub body: Block,
}

/// An item of the closed set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    /// `use path as alias;`
    Use(Vec<String>, Option<String>),
    /// `#[derive(..)] pub struct Name;`
    UnitStruct(String, Vec<String>),
    /// `pub type Name<..> = ty;`, its parameters named in order.
    Alias(String, Vec<String>, Type),
    /// `static NAME: ty = init;`
    Static(String, Type, Expr),
    /// `pub trait Name { sigs }`
    Trait(String, Vec<Sig>),
    /// `impl<..> Trait for ty { fns }`, or inherent without the trait.
    Impl(Vec<Generic>, Option<String>, Type, Vec<Function>),
    Fn(Function),
}

/// The crate: a profile and its items in order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Crate {
    pub profile: Profile,
    pub items: Vec<Item>,
}

impl Type {
    fn print(&self, scope: &[&str]) -> String {
        match self {
            Type::Unit => "()".to_owned(),
            Type::Bool => "bool".to_owned(),
            Type::Nat => "u64".to_owned(),
            Type::Fixed(kind) => kind.name().to_owned(),
            Type::Ordering => "Ordering".to_owned(),
            Type::SelfType => "Self".to_owned(),
            Type::Param(at) => usize::try_from(*at)
                .ok()
                .and_then(|at| scope.get(at))
                .map_or_else(|| format!("P{at}"), |name| (*name).to_owned()),
            Type::Ref(inner) => format!("&{}", inner.print(scope)),
            Type::Pair(a, b) => format!("({}, {})", a.print(scope), b.print(scope)),
            Type::Named(name, arguments) if arguments.is_empty() => name.clone(),
            Type::Named(name, arguments) => format!(
                "{name}<{}>",
                arguments
                    .iter()
                    .map(|argument| argument.print(scope))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

impl Pat {
    fn print(&self) -> String {
        match self {
            Pat::Wild => "_".to_owned(),
            Pat::Bind(name) => name.clone(),
            Pat::Path(path) => path.join("::"),
        }
    }
}

impl Expr {
    fn print(&self) -> String {
        let list = |items: &[Expr]| items.iter().map(Expr::print).collect::<Vec<_>>().join(", ");
        match self {
            Expr::Var(name) => name.clone(),
            Expr::Lit(Lit::Unit) => "()".to_owned(),
            Expr::Lit(Lit::Bool(value)) => value.to_string(),
            Expr::Lit(Lit::Nat(value)) => value.to_string(),
            Expr::Path(path) => path.join("::"),
            Expr::Call(path, arguments) => format!("{}({})", path.join("::"), list(arguments)),
            Expr::Method(receiver, name, arguments) => {
                format!("{}.{name}({})", receiver.print(), list(arguments))
            }
            Expr::Field(inner, at) => format!("{}.{at}", inner.print()),
            Expr::Borrow(inner) => format!("&{}", inner.print()),
            Expr::Equals(a, b) => format!("{} == {}", a.print(), b.print()),
            Expr::Match(scrutinee, arms) => format!(
                "match {} {{ {} }}",
                scrutinee.print(),
                arms.iter()
                    .map(|(pattern, value)| format!("{} => {},", pattern.print(), value.print()))
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
        }
    }
}

fn bounded(generics: &[Generic]) -> String {
    if generics.is_empty() {
        return String::new();
    }
    format!(
        "<{}>",
        generics
            .iter()
            .map(|generic| {
                if generic.bounds.is_empty() {
                    generic.name.clone()
                } else {
                    format!("{}: {}", generic.name, generic.bounds.join(" + "))
                }
            })
            .collect::<Vec<_>>()
            .join(", ")
    )
}

impl Sig {
    fn print(&self, scope: &[&str]) -> String {
        let mut parameters: Vec<String> = Vec::new();
        if self.receiver {
            parameters.push("&self".to_owned());
        }
        parameters.extend(
            self.params
                .iter()
                .map(|param| format!("{}: {}", param.name, param.ty.print(scope))),
        );
        let result = match self.result {
            Type::Unit => String::new(),
            _ => format!(" -> {}", self.result.print(scope)),
        };
        format!("fn {}({}){result}", self.name, parameters.join(", "))
    }
}

impl Function {
    fn print(&self, outer: &[Generic]) -> String {
        let scope: Vec<&str> = outer
            .iter()
            .chain(&self.generics)
            .map(|generic| generic.name.as_str())
            .collect();
        let sig = self.sig.print(&scope);
        let (head, tail) = sig.split_once('(').expect("a signature has parameters");
        let name = head.trim_start_matches("fn ");
        let mut text = String::new();
        if self.public {
            text.push_str("pub ");
        }
        text.push_str(&format!("fn {name}{}({tail} {{", bounded(&self.generics)));
        for statement in &self.body.statements {
            text.push_str(&format!(" {};", statement.print()));
        }
        // A unit tail after a statement is the statement's own end, and rustc
        // under `warnings = deny` refuses the spelled-out `()`.
        if self.body.tail == Expr::Lit(Lit::Unit) && !self.body.statements.is_empty() {
            text.push_str(" }");
        } else {
            text.push_str(&format!(" {} }}", self.body.tail.print()));
        }
        text
    }
}

impl Item {
    /// The Rust text of the item, one or more whole lines.
    #[must_use]
    pub fn print(&self) -> String {
        match self {
            Item::Use(path, None) => format!("use {};\n", path.join("::")),
            Item::Use(path, Some(alias)) => format!("use {} as {alias};\n", path.join("::")),
            Item::UnitStruct(name, derives) => {
                let derive = if derives.is_empty() {
                    String::new()
                } else {
                    format!("#[derive({})]\n", derives.join(", "))
                };
                format!("{derive}pub struct {name};\n")
            }
            Item::Alias(name, parameters, ty) => {
                let scope: Vec<&str> = parameters.iter().map(String::as_str).collect();
                let shown = if parameters.is_empty() {
                    String::new()
                } else {
                    format!("<{}>", parameters.join(", "))
                };
                format!("pub type {name}{shown} = {};\n", ty.print(&scope))
            }
            Item::Static(name, ty, init) => {
                format!("static {name}: {} = {};\n", ty.print(&[]), init.print())
            }
            Item::Trait(name, sigs) => {
                let mut text = format!("pub trait {name} {{\n");
                for sig in sigs {
                    text.push_str(&format!("    {};\n", sig.print(&[])));
                }
                text.push_str("}\n");
                text
            }
            Item::Impl(generics, tr, ty, functions) => {
                let scope: Vec<&str> = generics.iter().map(|g| g.name.as_str()).collect();
                let of = tr
                    .as_ref()
                    .map_or_else(String::new, |tr| format!("{tr} for "));
                let mut text = format!("impl{} {of}{} {{\n", bounded(generics), ty.print(&scope));
                for function in functions {
                    text.push_str(&format!("    {}\n", function.print(generics)));
                }
                text.push_str("}\n");
                text
            }
            Item::Fn(function) => format!("{}\n", function.print(&[])),
        }
    }
}

/// One `use` line for the consecutive uses of `uses` that share a parent
/// path, braced when there are several, so that two imports of one module are
/// one line, the way the library table lists them.
fn print_uses(uses: &[(&Vec<String>, &Option<String>)]) -> String {
    let leaf = |(path, alias): &(&Vec<String>, &Option<String>)| {
        let last = path.last().cloned().unwrap_or_default();
        alias
            .as_ref()
            .map_or(last.clone(), |alias| format!("{last} as {alias}"))
    };
    let (path, _) = uses[0];
    let parent = path[..path.len() - 1].join("::");
    if uses.len() == 1 {
        return format!("use {parent}::{};\n", leaf(&uses[0]));
    }
    let leaves: Vec<String> = uses.iter().map(leaf).collect();
    format!("use {parent}::{{{}}};\n", leaves.join(", "))
}

/// The text of `items`, in order.
#[must_use]
pub fn print(krate: &Crate) -> String {
    let mut chunks: Vec<String> = Vec::new();
    let mut at = 0;
    while at < krate.items.len() {
        if let Item::Use(path, alias) = &krate.items[at] {
            let parent = &path[..path.len().saturating_sub(1)];
            let mut group = vec![(path, alias)];
            while let Some(Item::Use(next, alias)) = krate.items.get(at + group.len()) {
                if next.len() == path.len() && &next[..next.len() - 1] == parent {
                    group.push((next, alias));
                } else {
                    break;
                }
            }
            at += group.len();
            chunks.push(print_uses(&group));
        } else {
            chunks.push(krate.items[at].print());
            at += 1;
        }
    }
    chunks.join("\n")
}
