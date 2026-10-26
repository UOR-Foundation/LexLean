//! `RuntimeSyntax` (SPEC.md §17.16): the closed set of constructs the runtime's
//! support items and heap functions are written in, and the printer that gives
//! each its Rust text.
//!
//! The set is a second module beside `RustSyntax`, not a widening of it: a
//! generated program never contains a trait, an impl, a generic, a loop over a
//! buffer, or a mutable borrow, and the calculus that proves a generated
//! program correct is not asked about one. What the runtime needs beyond that
//! is exactly what is listed here, so an item outside the set cannot be
//! written, decoded, or printed.

use super::super::IntKind;

/// The profile of the crate the items belong to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Profile {
    /// `core` only: nothing the crate holds allocates.
    Core,
    /// `core`, `alloc`, and `std`.
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
    /// A declared struct, enum, or alias, or a type of the closed table.
    Named(String, Vec<Type>),
    /// `u128` (unsigned) or `i128` (signed).
    Wide(bool),
    /// `usize`.
    Size,
    Char,
    /// `str`.
    Str,
    /// `[T]`.
    Slice(Box<Type>),
    /// `&mut T`.
    RefMut(Box<Type>),
}

/// A literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lit {
    Unit,
    Bool(bool),
    Nat(u64),
    Char(String),
    Str(String),
}

/// A binary operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    AndAnd,
    OrOr,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

/// A pattern.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pat {
    Wild,
    Bind(String),
    Path(Vec<String>),
    BindMut(String),
    Tuple(Vec<Pat>),
    TupleStruct(Vec<String>, Vec<Pat>),
    /// A struct or an enum's record variant: the path, then each field name
    /// with its pattern.
    Struct(Vec<String>, Vec<String>, Vec<Pat>),
    Lit(Lit),
}

/// A `match` arm.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arm {
    pub pattern: Pat,
    pub guard: Option<Expr>,
    pub body: Expr,
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
    /// A field of a struct by name: `value.name`.
    FieldNamed(Box<Expr>, String),
    Borrow(Box<Expr>),
    Equals(Box<Expr>, Box<Expr>),
    Match(Box<Expr>, Vec<Arm>),
    /// `let [mut] pattern[: type] = value`, a statement that binds for the
    /// statements after it.
    Let(Pat, Option<Type>, Box<Expr>),
    Assign(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Block>, Box<Block>),
    While(Box<Expr>, Box<Block>),
    WhileLet(Pat, Box<Expr>, Box<Block>),
    Loop(Box<Block>),
    For(Pat, Box<Expr>, Box<Block>),
    Break,
    Return(Option<Box<Expr>>),
    Tuple(Vec<Expr>),
    /// A struct literal: the path, then each field name with its value.
    StructLit(Vec<String>, Vec<String>, Vec<Expr>),
    Binary(Op, Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Cast(Box<Expr>, Type),
    Index(Box<Expr>, Box<Expr>),
    RangeSlice(Box<Expr>, Option<Box<Expr>>, Option<Box<Expr>>),
    Deref(Box<Expr>),
    BorrowMut(Box<Expr>),
    /// A method call with type arguments: `recv.name::<T>(args)`.
    MethodOf(Box<Expr>, String, Vec<Type>, Vec<Expr>),
    Block(Box<Block>),
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
    pub mutable: bool,
    pub ty: Type,
}

/// A function's signature, which a trait states and an impl repeats.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sig {
    pub name: String,
    /// Takes `&self`, or `&mut self` when `exclusive`.
    pub receiver: bool,
    pub exclusive: bool,
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

/// A variant of an enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Variant {
    Unit(String),
    Tuple(String, Vec<Type>),
    Record(String, Vec<String>, Vec<Type>),
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
    /// `#[derive(..)] pub struct Name<..>(types);`: name, generics, derives,
    /// fields.
    TupleStruct(String, Vec<String>, Vec<String>, Vec<Type>),
    /// `#[derive(..)] pub struct Name<..> { names: types }`.
    RecordStruct(String, Vec<String>, Vec<String>, Vec<String>, Vec<Type>),
    /// `#[derive(..)] pub enum Name<..> { variants }`.
    Enum(String, Vec<String>, Vec<String>, Vec<Variant>),
}

/// The crate: a profile and its items in order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Crate {
    pub profile: Profile,
    pub items: Vec<Item>,
}

fn list<T>(items: &[T], each: impl Fn(&T) -> String) -> String {
    items.iter().map(each).collect::<Vec<_>>().join(", ")
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
            Type::RefMut(inner) => format!("&mut {}", inner.print(scope)),
            Type::Pair(a, b) => format!("({}, {})", a.print(scope), b.print(scope)),
            Type::Named(name, arguments) if arguments.is_empty() => name.clone(),
            Type::Named(name, arguments) => {
                format!(
                    "{name}<{}>",
                    list(arguments, |argument| argument.print(scope))
                )
            }
            Type::Wide(true) => "u128".to_owned(),
            Type::Wide(false) => "i128".to_owned(),
            Type::Size => "usize".to_owned(),
            Type::Char => "char".to_owned(),
            Type::Str => "str".to_owned(),
            Type::Slice(inner) => format!("[{}]", inner.print(scope)),
        }
    }
}

impl Lit {
    fn print(&self) -> String {
        match self {
            Lit::Unit => "()".to_owned(),
            Lit::Bool(value) => value.to_string(),
            Lit::Nat(value) => value.to_string(),
            // Rust's `Debug` of a character or a string is its literal.
            Lit::Char(text) => format!("{:?}", text.chars().next().unwrap_or('\0')),
            Lit::Str(text) => format!("{text:?}"),
        }
    }
}

impl Op {
    fn text(self) -> &'static str {
        match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
            Op::Rem => "%",
            Op::AndAnd => "&&",
            Op::OrOr => "||",
            Op::Ne => "!=",
            Op::Lt => "<",
            Op::Le => "<=",
            Op::Gt => ">",
            Op::Ge => ">=",
        }
    }

    /// The binding strength of the operator, as Rust's.
    fn precedence(self) -> u8 {
        match self {
            Op::OrOr => 3,
            Op::AndAnd => 4,
            Op::Ne | Op::Lt | Op::Le | Op::Gt | Op::Ge => 5,
            Op::Add | Op::Sub => 9,
            Op::Mul | Op::Div | Op::Rem => 10,
        }
    }
}

impl Pat {
    fn print(&self) -> String {
        match self {
            Pat::Wild => "_".to_owned(),
            Pat::Bind(name) => name.clone(),
            Pat::Path(path) => path.join("::"),
            Pat::BindMut(name) => format!("mut {name}"),
            Pat::Tuple(items) => format!("({})", list(items, Pat::print)),
            Pat::TupleStruct(path, items) => {
                format!("{}({})", path.join("::"), list(items, Pat::print))
            }
            Pat::Struct(path, names, items) => {
                let fields = names
                    .iter()
                    .zip(items)
                    .map(|(name, item)| match item {
                        Pat::Bind(bound) if bound == name => name.clone(),
                        Pat::BindMut(bound) if bound == name => format!("mut {name}"),
                        other => format!("{name}: {}", other.print()),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{} {{ {fields} }}", path.join("::"))
            }
            Pat::Lit(literal) => literal.print(),
        }
    }
}

const ASSIGN: u8 = 1;
const RANGE: u8 = 2;
const CAST: u8 = 11;
const UNARY: u8 = 12;
const POSTFIX: u8 = 13;
const ATOM: u8 = 14;

/// Whether the left operand of a binary operator is written in parentheses.
/// A comparison does not chain, so the operand of equal strength is; and
/// `a as T < b` reads `<` as the opening of the arguments of a type, so a left
/// operand whose text ends in a cast is, whatever its own strength.
fn binary_left_parenthesized(op: Op, left: &Expr) -> bool {
    let own = op.precedence();
    let needed = if own == 5 { own + 1 } else { own };
    (op == Op::Lt && left.ends_in_cast()) || left.precedence() < needed
}

impl Expr {
    /// Whether the text of the expression ends in a cast, which no
    /// parenthesis closes: the cast itself, or a binary operator whose right
    /// operand is written without parentheses and ends in one.
    fn ends_in_cast(&self) -> bool {
        match self {
            Expr::Cast(..) => true,
            Expr::Binary(op, _, right) => {
                right.precedence() > op.precedence() && right.ends_in_cast()
            }
            _ => false,
        }
    }

    /// Whether the text holds a struct literal outside every parenthesis,
    /// bracket and brace, where the head of an `if`, a loop, or a `match`
    /// reads its brace as the body.
    #[must_use]
    pub fn struct_exposed(&self) -> bool {
        let bare =
            |inner: &Expr, needed: u8| inner.precedence() >= needed && inner.struct_exposed();
        match self {
            Expr::StructLit(..) => true,
            Expr::Method(receiver, ..)
            | Expr::MethodOf(receiver, ..)
            | Expr::Field(receiver, _)
            | Expr::FieldNamed(receiver, _)
            | Expr::Index(receiver, _)
            | Expr::RangeSlice(receiver, ..) => bare(receiver, POSTFIX),
            Expr::Borrow(inner)
            | Expr::BorrowMut(inner)
            | Expr::Deref(inner)
            | Expr::Not(inner) => bare(inner, UNARY),
            Expr::Cast(inner, _) => bare(inner, CAST),
            Expr::Equals(a, b) => bare(a, 6) || bare(b, 6),
            Expr::Binary(op, a, b) => {
                (!binary_left_parenthesized(*op, a) && a.struct_exposed())
                    || bare(b, op.precedence() + 1)
            }
            Expr::Assign(place, value) => place.struct_exposed() || value.struct_exposed(),
            Expr::Return(Some(value)) => value.struct_exposed(),
            Expr::Return(None)
            | Expr::Var(_)
            | Expr::Lit(_)
            | Expr::Path(_)
            | Expr::Call(..)
            | Expr::Match(..)
            | Expr::Let(..)
            | Expr::If(..)
            | Expr::While(..)
            | Expr::WhileLet(..)
            | Expr::Loop(_)
            | Expr::For(..)
            | Expr::Break
            | Expr::Tuple(_)
            | Expr::Block(_) => false,
        }
    }

    /// The binding strength of the expression's outermost construct.
    fn precedence(&self) -> u8 {
        match self {
            Expr::Assign(..) => ASSIGN,
            Expr::RangeSlice(..) => ATOM,
            Expr::Binary(op, ..) => op.precedence(),
            Expr::Equals(..) => 5,
            Expr::Cast(..) => CAST,
            Expr::Not(_) | Expr::Deref(_) | Expr::Borrow(_) | Expr::BorrowMut(_) => UNARY,
            Expr::Return(_) | Expr::Break | Expr::Let(..) => ASSIGN,
            Expr::Method(..)
            | Expr::MethodOf(..)
            | Expr::Field(..)
            | Expr::FieldNamed(..)
            | Expr::Index(..)
            | Expr::Call(..) => POSTFIX,
            Expr::Var(_)
            | Expr::Lit(_)
            | Expr::Path(_)
            | Expr::Match(..)
            | Expr::If(..)
            | Expr::While(..)
            | Expr::WhileLet(..)
            | Expr::Loop(_)
            | Expr::For(..)
            | Expr::Tuple(_)
            | Expr::StructLit(..)
            | Expr::Block(_) => ATOM,
        }
    }

    /// The text, parenthesised when the construct binds more loosely than
    /// `needed`: `rustc` under `warnings = deny` refuses a parenthesis that
    /// the grammar does not need.
    fn at(&self, needed: u8) -> String {
        let text = self.print();
        if self.precedence() < needed {
            format!("({text})")
        } else {
            text
        }
    }

    fn print(&self) -> String {
        let args = |items: &[Expr]| list(items, Expr::print);
        match self {
            Expr::Var(name) => name.clone(),
            Expr::Lit(literal) => literal.print(),
            Expr::Path(path) => path.join("::"),
            Expr::Call(path, arguments) => format!("{}({})", path.join("::"), args(arguments)),
            Expr::Method(receiver, name, arguments) => {
                format!("{}.{name}({})", receiver.at(POSTFIX), args(arguments))
            }
            Expr::MethodOf(receiver, name, types, arguments) => format!(
                "{}.{name}::<{}>({})",
                receiver.at(POSTFIX),
                list(types, |ty| ty.print(&[])),
                args(arguments)
            ),
            Expr::Field(inner, at) => format!("{}.{at}", inner.at(POSTFIX)),
            Expr::FieldNamed(inner, name) => format!("{}.{name}", inner.at(POSTFIX)),
            Expr::Borrow(inner) => format!("&{}", inner.at(UNARY)),
            Expr::BorrowMut(inner) => format!("&mut {}", inner.at(UNARY)),
            Expr::Deref(inner) => format!("*{}", inner.at(UNARY)),
            Expr::Not(inner) => format!("!{}", inner.at(UNARY)),
            Expr::Equals(a, b) => format!("{} == {}", a.at(6), b.at(6)),
            Expr::Binary(op, a, b) => {
                let own = op.precedence();
                let left = if binary_left_parenthesized(*op, a) {
                    format!("({})", a.print())
                } else {
                    a.print()
                };
                format!("{left} {} {}", op.text(), b.at(own + 1))
            }
            Expr::Cast(inner, ty) => format!("{} as {}", inner.at(CAST), ty.print(&[])),
            Expr::Index(inner, at) => format!("{}[{}]", inner.at(POSTFIX), at.print()),
            Expr::RangeSlice(inner, from, to) => format!(
                "{}[{}..{}]",
                inner.at(POSTFIX),
                from.as_ref()
                    .map_or_else(String::new, |from| from.at(RANGE + 1)),
                to.as_ref().map_or_else(String::new, |to| to.at(RANGE + 1))
            ),
            Expr::Match(scrutinee, arms) => format!(
                "match {} {{ {} }}",
                scrutinee.print(),
                arms.iter()
                    .map(|arm| {
                        let guard = arm
                            .guard
                            .as_ref()
                            .map_or_else(String::new, |guard| format!(" if {}", guard.print()));
                        format!("{}{guard} => {},", arm.pattern.print(), arm.body.print())
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Expr::Let(pattern, ty, value) => {
                let ty = ty
                    .as_ref()
                    .map_or_else(String::new, |ty| format!(": {}", ty.print(&[])));
                format!("let {}{ty} = {}", pattern.print(), value.print())
            }
            // `x = x + e` is printed as `x += e`: the same value, and the form the
            // package's lint gate (`clippy::assign_op_pattern`) requires.
            Expr::Assign(place, value) => match &**value {
                Expr::Binary(
                    op @ (Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Rem),
                    left,
                    right,
                ) if left == place => {
                    format!("{} {}= {}", place.print(), op.text(), right.print())
                }
                _ => format!("{} = {}", place.print(), value.print()),
            },
            Expr::If(condition, then, otherwise) => {
                let tail = if otherwise.is_empty() {
                    String::new()
                } else {
                    format!(" else {}", otherwise.print())
                };
                format!("if {} {}{tail}", condition.print(), then.print())
            }
            Expr::While(condition, body) => format!("while {} {}", condition.print(), body.print()),
            Expr::WhileLet(pattern, value, body) => {
                format!(
                    "while let {} = {} {}",
                    pattern.print(),
                    value.print(),
                    body.print()
                )
            }
            Expr::Loop(body) => format!("loop {}", body.print()),
            Expr::For(pattern, iterator, body) => {
                format!(
                    "for {} in {} {}",
                    pattern.print(),
                    iterator.print(),
                    body.print()
                )
            }
            Expr::Break => "break".to_owned(),
            Expr::Return(None) => "return".to_owned(),
            Expr::Return(Some(value)) => format!("return {}", value.print()),
            Expr::Tuple(items) => format!("({})", args(items)),
            Expr::StructLit(path, names, values) => {
                let fields = names
                    .iter()
                    .zip(values)
                    .map(|(name, value)| match value {
                        Expr::Var(bound) if bound == name => name.clone(),
                        other => format!("{name}: {}", other.print()),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{} {{ {fields} }}", path.join("::"))
            }
            Expr::Block(block) => block.print(),
        }
    }
}

impl Block {
    /// Whether the block is `{}`: no statement and a unit tail.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.statements.is_empty() && self.tail == Expr::Lit(Lit::Unit)
    }

    /// `{ statement; …; tail }`. A unit tail after a statement is the
    /// statement's own end, and `rustc` under `warnings = deny` refuses the
    /// spelled-out `()`.
    fn print(&self) -> String {
        let mut text = String::from("{");
        for statement in &self.statements {
            text.push_str(&format!(" {};", statement.print()));
        }
        if self.tail == Expr::Lit(Lit::Unit) {
            if self.statements.is_empty() {
                return "{ }".to_owned();
            }
            text.push_str(" }");
        } else {
            text.push_str(&format!(" {} }}", self.tail.print()));
        }
        text
    }
}

fn bounded(generics: &[Generic]) -> String {
    if generics.is_empty() {
        return String::new();
    }
    format!(
        "<{}>",
        list(generics, |generic| {
            if generic.bounds.is_empty() {
                generic.name.clone()
            } else {
                format!("{}: {}", generic.name, generic.bounds.join(" + "))
            }
        })
    )
}

fn named_generics(names: &[String]) -> String {
    if names.is_empty() {
        String::new()
    } else {
        format!("<{}>", names.join(", "))
    }
}

fn derive(derives: &[String]) -> String {
    if derives.is_empty() {
        String::new()
    } else {
        format!("#[derive({})]\n", derives.join(", "))
    }
}

impl Sig {
    fn print(&self, scope: &[&str]) -> String {
        let mut parameters: Vec<String> = Vec::new();
        if self.receiver {
            parameters.push(if self.exclusive { "&mut self" } else { "&self" }.to_owned());
        }
        parameters.extend(self.params.iter().map(|param| {
            let shown = if param.mutable { "mut " } else { "" };
            format!("{shown}{}: {}", param.name, param.ty.print(scope))
        }));
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
        text.push_str(&format!(
            "fn {name}{}({tail} {}",
            bounded(&self.generics),
            self.body.print()
        ));
        text
    }
}

impl Variant {
    fn print(&self, scope: &[&str]) -> String {
        match self {
            Variant::Unit(name) => name.clone(),
            Variant::Tuple(name, types) => {
                format!("{name}({})", list(types, |ty| ty.print(scope)))
            }
            Variant::Record(name, fields, types) => format!(
                "{name} {{ {} }}",
                fields
                    .iter()
                    .zip(types)
                    .map(|(field, ty)| format!("{field}: {}", ty.print(scope)))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
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
                format!("{}pub struct {name};\n", derive(derives))
            }
            Item::Alias(name, parameters, ty) => {
                let scope: Vec<&str> = parameters.iter().map(String::as_str).collect();
                format!(
                    "pub type {name}{} = {};\n",
                    named_generics(parameters),
                    ty.print(&scope)
                )
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
            Item::TupleStruct(name, generics, derives, types) => {
                let scope: Vec<&str> = generics.iter().map(String::as_str).collect();
                format!(
                    "{}pub struct {name}{}({});\n",
                    derive(derives),
                    named_generics(generics),
                    list(types, |ty| ty.print(&scope))
                )
            }
            Item::RecordStruct(name, generics, derives, fields, types) => {
                let scope: Vec<&str> = generics.iter().map(String::as_str).collect();
                format!(
                    "{}pub struct {name}{} {{ {} }}\n",
                    derive(derives),
                    named_generics(generics),
                    fields
                        .iter()
                        .zip(types)
                        .map(|(field, ty)| format!("{field}: {}", ty.print(&scope)))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Item::Enum(name, generics, derives, variants) => {
                let scope: Vec<&str> = generics.iter().map(String::as_str).collect();
                format!(
                    "{}pub enum {name}{} {{ {} }}\n",
                    derive(derives),
                    named_generics(generics),
                    list(variants, |variant| variant.print(&scope))
                )
            }
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
