//! The closed Rust AST of the reference renderings (SPEC.md §17.16).
//!
//! Generated code is built as this AST, checked by [`validate`], and only
//! then printed by [`print`] into canonical bytes. The AST has no node for
//! arbitrary text: every identifier is generated from a closed kind and an
//! index, or is an exported name the package manifest validated; every
//! called function is a program function or a runtime [`Item`]; every type
//! is a calculus type or one of the representation types of §17.16. So no
//! rendering can inject code outside the safe subset, a raw expression, or
//! a name it did not declare. Every expression, binding, and item carries
//! the [`Origin`] it realizes, which [`validate::correspond`] checks against
//! the construct it is, instance by instance.

use std::fmt::Write as _;

use super::super::{IntKind, OrderingValue};
use super::runtime::Item;
use super::Profile;

/// A generated identifier.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Ident {
    /// A local of the target program, `v<n>`.
    Local(u64),
    /// A match scrutinee, `m<n>`.
    Holder(u64),
    /// A primitive operand, `a<n>`.
    Operand(u64),
    /// An applied closure, `c<n>`.
    Callee(u64),
    /// A boxed field taken out of its box, `r<n>`.
    Boxed(u64),
    /// A projected pair component or field, `h<n>`.
    Part(u64),
    /// A captured value inside a closure's dispatch, `k<n>`.
    Capture(u64),
    /// A parameter of a closure's dispatch or an exported function, `p<n>`.
    Param(u64),
    /// A program function, `f<n>`.
    Function(u64),
    /// An exported function, named by the package manifest.
    Export(String),
}

impl Ident {
    fn text(&self) -> String {
        match self {
            Self::Local(n) => format!("v{n}"),
            Self::Holder(n) => format!("m{n}"),
            Self::Operand(n) => format!("a{n}"),
            Self::Callee(n) => format!("c{n}"),
            Self::Boxed(n) => format!("r{n}"),
            Self::Part(n) => format!("h{n}"),
            Self::Capture(n) => format!("k{n}"),
            Self::Param(n) => format!("p{n}"),
            Self::Function(n) => format!("f{n}"),
            Self::Export(name) => name.clone(),
        }
    }
}

/// The one-letter prefixes of generated identifiers, which an exported name
/// may not imitate.
pub const GENERATED_PREFIXES: [char; 9] = ['v', 'm', 'a', 'c', 'r', 'h', 'k', 'p', 'f'];

/// The calculus element (§17.14) a construct realizes, as lowering derives
/// it from the term, shape, literal, or structural realization the
/// construct renders, and the width of a fixed-width element: the
/// operand width of a fixed-width primitive, the target of a conversion or
/// a parse, or the kind of a fixed-width literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Origin {
    pub element: String,
    pub width: Option<IntKind>,
}

impl Origin {
    /// The origin of an element without a width.
    #[must_use]
    pub fn of(element: &str) -> Self {
        Self {
            element: element.to_owned(),
            width: None,
        }
    }

    /// The origin of an element at `width`.
    #[must_use]
    pub fn at_width(element: &str, width: Option<IntKind>) -> Self {
        Self {
            element: element.to_owned(),
            width,
        }
    }
}

/// A type of generated code.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    Unit,
    Bool,
    /// `nat`, as `u64`.
    Nat,
    /// `int`, as `i64`.
    Int,
    Fixed(IntKind),
    Ordering,
    Str,
    Bytes,
    Option(Box<Type>),
    Result(Box<Type>, Box<Type>),
    List(Box<Type>),
    Pair(Box<Type>, Box<Type>),
    Adt(u64),
    /// The defunctionalized enum of function type `n`.
    Fn(u64),
    /// A field or capture whose type holds its owner.
    Rc(Box<Type>),
    /// The result of a fallible function, `R<T>`.
    Fallible(Box<Type>),
    /// A borrowed parameter of an exported function.
    Ref(Box<Type>),
}

impl Type {
    fn text(&self) -> String {
        match self {
            Self::Unit => "()".to_owned(),
            Self::Bool => "bool".to_owned(),
            Self::Nat => "u64".to_owned(),
            Self::Int => "i64".to_owned(),
            Self::Fixed(kind) => kind.name().to_owned(),
            Self::Ordering => "Ordering".to_owned(),
            Self::Str => "Str".to_owned(),
            Self::Bytes => "Bytes".to_owned(),
            Self::Option(inner) => format!("Option<{}>", inner.text()),
            Self::Result(ok, error) => format!("Result<{}, {}>", ok.text(), error.text()),
            Self::List(element) => format!("List<{}>", element.text()),
            Self::Pair(left, right) => format!("({}, {})", left.text(), right.text()),
            Self::Adt(n) => format!("Adt{n}"),
            Self::Fn(n) => format!("Fn{n}"),
            Self::Rc(inner) => format!("std::rc::Rc<{}>", inner.text()),
            Self::Fallible(inner) => format!("R<{}>", inner.text()),
            Self::Ref(inner) => format!("&{}", inner.text()),
        }
    }

    /// Whether a value of the type lives on, or holds, heap storage.
    #[must_use]
    pub fn heap(&self) -> bool {
        match self {
            Self::Str | Self::Bytes | Self::List(_) | Self::Rc(_) => true,
            Self::Option(inner) | Self::Fallible(inner) | Self::Ref(inner) => inner.heap(),
            Self::Result(left, right) | Self::Pair(left, right) => left.heap() || right.heap(),
            Self::Unit
            | Self::Bool
            | Self::Nat
            | Self::Int
            | Self::Fixed(_)
            | Self::Ordering
            | Self::Adt(_)
            | Self::Fn(_) => false,
        }
    }

    /// Whether the type is `Copy` in Rust: scalars and their options,
    /// results, and pairs.
    #[must_use]
    pub fn copy(&self) -> bool {
        match self {
            Self::Unit | Self::Bool | Self::Nat | Self::Int | Self::Fixed(_) | Self::Ordering => {
                true
            }
            Self::Option(inner) => inner.copy(),
            Self::Result(left, right) | Self::Pair(left, right) => left.copy() && right.copy(),
            Self::Str
            | Self::Bytes
            | Self::List(_)
            | Self::Adt(_)
            | Self::Fn(_)
            | Self::Rc(_)
            | Self::Fallible(_)
            | Self::Ref(_) => false,
        }
    }
}

/// A literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lit {
    Unit,
    Bool(bool),
    Nat(u64),
    Int(i64),
    Fixed(IntKind, i128),
    Str(String),
    Bytes(Vec<u8>),
    Ordering(OrderingValue),
}

/// A constructor of a value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ctor {
    None(Type),
    Some,
    Ok(Type, Type),
    Err(Type, Type),
    /// Constructor `c` of ADT `adt`.
    Adt {
        adt: u64,
        constructor: u64,
    },
    /// The closure of `function` in function type `fn_type`.
    Closure {
        fn_type: u64,
        function: u64,
    },
    Cons,
    Nil(Type),
}

/// A pattern.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pat {
    Wild,
    Bind(Ident),
    Unit,
    Tuple(Vec<Pat>),
    None,
    Some(Box<Pat>),
    Ok(Box<Pat>),
    Err(Box<Pat>),
    Ordering(OrderingValue),
    Adt {
        adt: u64,
        constructor: u64,
        fields: Vec<Pat>,
    },
}

/// The called function of a call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Callee {
    Function(u64),
    Runtime(Item),
}

/// An expression. Every construct carries the [`Origin`] it realizes; the
/// reads that are parts of a list match or a successor arm carry none of
/// their own.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Lit(Lit, Origin),
    /// A binding read by moving it.
    Move(Ident, Origin),
    /// A binding read by cloning it.
    Clone(Ident, Origin),
    /// A binding of a `Copy` type, read by copying it.
    Copy(Ident, Origin),
    /// A borrowed binding of a `Copy` type, read through the reference.
    Deref(Ident, Origin),
    /// The negation of a Boolean: `!e`.
    Not(Box<Expr>, Origin),
    /// The value inside a bound box, cloned: `(*r).clone()`.
    Unbox(Ident, Origin),
    /// A value put in a box.
    Box(Box<Expr>, Origin),
    /// A call; `propagate` adds `?` to a fallible call.
    Call {
        callee: Callee,
        args: Vec<Expr>,
        propagate: bool,
        at: Origin,
    },
    /// An application of a closure bound to `holder`.
    Apply {
        holder: Ident,
        args: Vec<Expr>,
        propagate: bool,
        at: Origin,
    },
    Construct {
        ctor: Ctor,
        args: Vec<Expr>,
        at: Origin,
    },
    Pair(Box<Expr>, Box<Expr>, Origin),
    If {
        condition: Box<Expr>,
        then_branch: Box<Block>,
        else_branch: Box<Block>,
        at: Origin,
    },
    /// A match; one without arms is on a value of a type no value
    /// inhabits.
    Match {
        scrutinee: Box<Expr>,
        arms: Vec<(Pat, Block)>,
        at: Origin,
    },
    Block(Box<Block>),
    /// `m.uncons()`.
    Uncons(Ident),
    /// `m == 0`.
    IsZero(Ident, Origin),
    /// `m != 0`.
    NonZero(Ident, Origin),
    /// `m - 1`, read where `m` is not zero.
    Predecessor(Ident),
    /// `i128::from(e)`, the operand of a conversion.
    Widen(Box<Expr>, Origin),
    /// `Ok(e)`: an infallible value where a fallible one is expected.
    Succeed(Box<Expr>, Origin),
}

impl Expr {
    /// The origin of the construct, if the expression is one.
    #[must_use]
    pub fn origin(&self) -> Option<&Origin> {
        match self {
            Self::Lit(_, at)
            | Self::Move(_, at)
            | Self::Clone(_, at)
            | Self::Copy(_, at)
            | Self::Deref(_, at)
            | Self::Not(_, at)
            | Self::Unbox(_, at)
            | Self::Box(_, at)
            | Self::Call { at, .. }
            | Self::Apply { at, .. }
            | Self::Construct { at, .. }
            | Self::Pair(_, _, at)
            | Self::If { at, .. }
            | Self::Match { at, .. }
            | Self::Widen(_, at)
            | Self::Succeed(_, at)
            | Self::IsZero(_, at)
            | Self::NonZero(_, at) => Some(at),
            Self::Block(_) | Self::Uncons(_) | Self::Predecessor(_) => None,
        }
    }
}

/// A `let` binding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Let {
    pub pat: Pat,
    pub ty: Option<Type>,
    pub value: Expr,
    pub at: Origin,
}

/// A block: bindings, then a tail expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub lets: Vec<Let>,
    pub tail: Expr,
}

impl Block {
    /// A block of one expression.
    #[must_use]
    pub fn of(tail: Expr) -> Self {
        Block {
            lets: Vec::new(),
            tail,
        }
    }
}

/// How a captured value is read inside a closure's dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureRead {
    Copy,
    Clone,
    Unbox,
    /// A unit capture: matched by `_` and passed as `()`.
    Unit,
}

/// One arm of a closure's dispatch: the closure of `function`, its
/// captures, and whether its call propagates failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dispatch {
    pub function: u64,
    pub captures: Vec<CaptureRead>,
    /// The function fails, so the call ends in `?`, or the dispatch is
    /// fallible while the function is not, so the call is wrapped in `Ok`.
    pub function_fallible: bool,
}

/// A top-level item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemDef {
    /// An enum of an ADT (variants `C<n>`) or a function type (variants
    /// `F<function>`): each variant's number and field types.
    Enum {
        name: Type,
        variants: Vec<(u64, Vec<Type>)>,
        at: Origin,
    },
    /// The `apply` method of function type `fn_type`.
    Apply {
        fn_type: u64,
        parameters: Vec<Type>,
        result: Type,
        fallible: bool,
        arms: Vec<Dispatch>,
        at: Origin,
    },
    /// A function.
    Function {
        name: Ident,
        parameters: Vec<(Pat, Type)>,
        result: Type,
        body: Block,
        at: Origin,
    },
}

/// A rendered library crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Crate {
    pub profile: Profile,
    pub items: Vec<ItemDef>,
}

// --- printing ----------------------------------------------------------------

/// A string literal written by LexLean's own escaper: printable ASCII other
/// than `"` and `\` as itself, those two escaped, and every other code point
/// as `\u{..}`, so the bytes never depend on the Unicode tables of the
/// toolchain that built LexLean.
fn quoted(text: &str) -> String {
    let mut out = String::from("\"");
    for character in text.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            ' '..='~' => out.push(character),
            other => {
                let _ = write!(out, "\\u{{{:x}}}", u32::from(other));
            }
        }
    }
    out.push('"');
    out
}

fn lit(literal: &Lit) -> String {
    let signed = |value: String, kind: &str, minimum: &str| {
        if value == minimum {
            format!("{kind}::MIN")
        } else {
            format!("{value}{kind}")
        }
    };
    match literal {
        Lit::Unit => "()".to_owned(),
        Lit::Bool(value) => value.to_string(),
        Lit::Nat(value) => format!("{value}u64"),
        Lit::Int(value) => signed(value.to_string(), "i64", &i64::MIN.to_string()),
        Lit::Fixed(kind, value) => {
            let (low, _) = kind.range();
            if kind.signed() && *value == low {
                format!("{}::MIN", kind.name())
            } else {
                format!("{value}{}", kind.name())
            }
        }
        Lit::Str(text) => format!("Str::lit({})", quoted(text)),
        Lit::Bytes(octets) => {
            let bytes: Vec<String> = octets.iter().map(|byte| format!("0x{byte:02x}")).collect();
            format!("Bytes::lit(&[{}])", bytes.join(", "))
        }
        Lit::Ordering(value) => ordering(*value).to_owned(),
    }
}

fn ordering(value: OrderingValue) -> &'static str {
    match value {
        OrderingValue::Lt => "Ordering::Less",
        OrderingValue::Eq => "Ordering::Equal",
        OrderingValue::Gt => "Ordering::Greater",
    }
}

fn pat(pattern: &Pat) -> String {
    match pattern {
        Pat::Wild => "_".to_owned(),
        Pat::Bind(ident) => ident.text(),
        Pat::Unit => "()".to_owned(),
        Pat::Tuple(items) => format!("({})", items.iter().map(pat).collect::<Vec<_>>().join(", ")),
        Pat::None => "None".to_owned(),
        Pat::Some(inner) => format!("Some({})", pat(inner)),
        Pat::Ok(inner) => format!("Ok({})", pat(inner)),
        Pat::Err(inner) => format!("Err({})", pat(inner)),
        Pat::Ordering(value) => ordering(*value).to_owned(),
        Pat::Adt {
            adt,
            constructor,
            fields,
        } => {
            if fields.is_empty() {
                format!("Adt{adt}::C{constructor}")
            } else {
                format!(
                    "Adt{adt}::C{constructor}({})",
                    fields.iter().map(pat).collect::<Vec<_>>().join(", ")
                )
            }
        }
    }
}

/// The ` -> T` of a signature: nothing for unit, which Rust spells by
/// omission.
fn returns(result: &str) -> String {
    if result == "()" {
        String::new()
    } else {
        format!(" -> {result}")
    }
}

struct Printer {
    out: String,
}

impl Printer {
    fn line(&mut self, depth: usize, text: &str) {
        for _ in 0..depth {
            self.out.push_str("    ");
        }
        self.out.push_str(text);
        self.out.push('\n');
    }

    /// An expression in place: compound expressions open blocks that are
    /// printed on their own lines.
    #[allow(clippy::too_many_lines)]
    fn expr(&mut self, expr: &Expr, depth: usize) -> String {
        match expr {
            Expr::Lit(literal, _) => lit(literal),
            Expr::Move(ident, _) | Expr::Copy(ident, _) => ident.text(),
            Expr::Clone(ident, _) => format!("{}.clone()", ident.text()),
            Expr::Deref(ident, _) => format!("*{}", ident.text()),
            // A comparison or subtraction under `!` is parenthesized, so the
            // negation never binds to its left operand alone.
            Expr::Not(inner, _) => match inner.as_ref() {
                Expr::IsZero(..) | Expr::NonZero(..) | Expr::Predecessor(_) => {
                    format!("!({})", self.expr(inner, depth))
                }
                _ => format!("!{}", self.expr(inner, depth)),
            },
            Expr::Unbox(ident, _) => format!("(*{}).clone()", ident.text()),
            Expr::Box(inner, _) => format!("std::rc::Rc::new({})", self.expr(inner, depth)),
            Expr::Call {
                callee,
                args,
                propagate,
                ..
            } => {
                let name = match callee {
                    Callee::Function(n) => format!("f{n}"),
                    Callee::Runtime(item) => item.path(),
                };
                let args: Vec<String> = args.iter().map(|arg| self.expr(arg, depth)).collect();
                format!(
                    "{name}({}){}",
                    args.join(", "),
                    if *propagate { "?" } else { "" }
                )
            }
            Expr::Apply {
                holder,
                args,
                propagate,
                ..
            } => {
                let args: Vec<String> = args.iter().map(|arg| self.expr(arg, depth)).collect();
                format!(
                    "{}.apply({}){}",
                    holder.text(),
                    args.join(", "),
                    if *propagate { "?" } else { "" }
                )
            }
            Expr::Construct { ctor, args, .. } => {
                let args: Vec<String> = args.iter().map(|arg| self.expr(arg, depth)).collect();
                let joined = args.join(", ");
                match ctor {
                    Ctor::None(ty) => format!("None::<{}>", ty.text()),
                    Ctor::Some => format!("Some({joined})"),
                    Ctor::Ok(ok, error) => {
                        format!("Ok::<{}, {}>({joined})", ok.text(), error.text())
                    }
                    Ctor::Err(ok, error) => {
                        format!("Err::<{}, {}>({joined})", ok.text(), error.text())
                    }
                    Ctor::Adt { adt, constructor } => {
                        if args.is_empty() {
                            format!("Adt{adt}::C{constructor}")
                        } else {
                            format!("Adt{adt}::C{constructor}({joined})")
                        }
                    }
                    Ctor::Closure { fn_type, function } => {
                        if args.is_empty() {
                            format!("Fn{fn_type}::F{function}")
                        } else {
                            format!("Fn{fn_type}::F{function}({joined})")
                        }
                    }
                    Ctor::Cons => format!("List::cons({joined})"),
                    Ctor::Nil(ty) => format!("List::<{}>::nil()", ty.text()),
                }
            }
            Expr::Pair(left, right, _) => {
                format!("({}, {})", self.expr(left, depth), self.expr(right, depth))
            }
            Expr::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                let condition = self.expr(condition, depth);
                let then_text = self.block_inline(then_branch, depth);
                let else_text = self.block_inline(else_branch, depth);
                // An empty unit `else` is left implicit, as Rust spells it.
                if else_text == "{}" {
                    format!("if {condition} {then_text}")
                } else {
                    format!("if {condition} {then_text} else {else_text}")
                }
            }
            Expr::Match {
                scrutinee, arms, ..
            } => {
                let scrutinee = self.expr(scrutinee, depth);
                if arms.is_empty() {
                    return format!("match {scrutinee} {{}}");
                }
                let mut out = format!("match {scrutinee} {{\n");
                for (pattern, body) in arms {
                    let body = self.block_inline(body, depth + 1);
                    let _ = writeln!(
                        out,
                        "{}{} => {body}",
                        "    ".repeat(depth + 1),
                        pat(pattern)
                    );
                }
                let _ = write!(out, "{}}}", "    ".repeat(depth));
                out
            }
            Expr::Block(block) => self.block_inline(block, depth),
            Expr::Uncons(ident) => format!("{}.uncons()", ident.text()),
            Expr::IsZero(ident, _) => format!("{} == 0", ident.text()),
            Expr::NonZero(ident, _) => format!("{} != 0", ident.text()),
            Expr::Predecessor(ident) => format!("{} - 1", ident.text()),
            Expr::Widen(inner, _) => format!("i128::from({})", self.expr(inner, depth)),
            Expr::Succeed(inner, _) => format!("Ok({})", self.expr(inner, depth)),
        }
    }

    /// A block as an expression: `{` on this line, its bindings and tail
    /// one level deeper, `}` at this depth. A unit tail is left implicit,
    /// as Rust spells it, so an empty unit block is `{}`.
    fn block_inline(&mut self, block: &Block, depth: usize) -> String {
        let unit_tail = matches!(block.tail, Expr::Lit(Lit::Unit, _));
        if unit_tail && block.lets.is_empty() {
            return "{}".to_owned();
        }
        let mut out = String::from("{\n");
        let inner = "    ".repeat(depth + 1);
        for binding in &block.lets {
            let value = self.expr(&binding.value, depth + 1);
            match &binding.ty {
                Some(ty) => {
                    let _ = writeln!(
                        out,
                        "{inner}let {}: {} = {value};",
                        pat(&binding.pat),
                        ty.text()
                    );
                }
                None => {
                    let _ = writeln!(out, "{inner}let {} = {value};", pat(&binding.pat));
                }
            }
        }
        if !unit_tail {
            let tail = self.expr(&block.tail, depth + 1);
            let _ = writeln!(out, "{inner}{tail}");
        }
        let _ = write!(out, "{}}}", "    ".repeat(depth));
        out
    }

    fn item(&mut self, item: &ItemDef) {
        match item {
            ItemDef::Enum { name, variants, .. } => {
                self.line(0, "#[derive(Clone)]");
                self.line(0, &format!("pub enum {} {{", name.text()));
                for line in enum_lines(name, variants) {
                    self.line(1, &line);
                }
                self.line(0, "}");
            }
            ItemDef::Apply {
                fn_type,
                parameters,
                result,
                fallible,
                arms,
                ..
            } => {
                let unread = if arms.is_empty() { "_" } else { "" };
                let params: Vec<String> = parameters
                    .iter()
                    .enumerate()
                    .map(|(at, ty)| format!("{unread}p{at}: {}", ty.text()))
                    .collect();
                let result = if *fallible {
                    format!("R<{}>", result.text())
                } else {
                    result.text()
                };
                self.line(0, &format!("impl Fn{fn_type} {{"));
                self.line(
                    1,
                    &format!(
                        "pub fn apply(&self, {}){} {{",
                        params.join(", "),
                        returns(&result)
                    ),
                );
                if arms.is_empty() {
                    // A function type no closure inhabits is an empty enum,
                    // matched without arms.
                    self.line(2, "match *self {}");
                } else {
                    self.line(2, "match self {");
                    for arm in arms {
                        let names: Vec<String> = arm
                            .captures
                            .iter()
                            .enumerate()
                            .map(|(at, read)| match read {
                                CaptureRead::Unit => "_".to_owned(),
                                _ => format!("k{at}"),
                            })
                            .collect();
                        let mut passed: Vec<String> = arm
                            .captures
                            .iter()
                            .enumerate()
                            .map(|(at, read)| match read {
                                CaptureRead::Copy => format!("*k{at}"),
                                CaptureRead::Clone => format!("k{at}.clone()"),
                                CaptureRead::Unbox => format!("(**k{at}).clone()"),
                                CaptureRead::Unit => "()".to_owned(),
                            })
                            .collect();
                        passed.extend((0..parameters.len()).map(|at| format!("p{at}")));
                        let call = format!("f{}({})", arm.function, passed.join(", "));
                        // A unit is passed to `Ok` as the literal `()`, as
                        // Rust's lint gate requires.
                        let call = match (*fallible && !arm.function_fallible, result.as_str()) {
                            (true, "R<()>") => format!("{{ let () = {call}; Ok(()) }}"),
                            (true, _) => format!("Ok({call})"),
                            (false, _) => call,
                        };
                        let pattern = if names.is_empty() {
                            format!("Fn{fn_type}::F{}", arm.function)
                        } else {
                            format!("Fn{fn_type}::F{}({})", arm.function, names.join(", "))
                        };
                        self.line(3, &format!("{pattern} => {call},"));
                    }
                    self.line(2, "}");
                }
                self.line(1, "}");
                self.line(0, "}");
            }
            ItemDef::Function {
                name,
                parameters,
                result,
                body,
                ..
            } => {
                let params: Vec<String> = parameters
                    .iter()
                    .map(|(pattern, ty)| format!("{}: {}", pat(pattern), ty.text()))
                    .collect();
                let body = self.block_inline(body, 0);
                self.line(
                    0,
                    &format!(
                        "pub fn {}({}){} {body}",
                        name.text(),
                        params.join(", "),
                        returns(&result.text())
                    ),
                );
            }
        }
    }
}

fn enum_lines(name: &Type, variants: &[(u64, Vec<Type>)]) -> Vec<String> {
    let prefix = match name {
        Type::Fn(_) => "F",
        _ => "C",
    };
    variants
        .iter()
        .map(|(index, fields)| {
            if fields.is_empty() {
                format!("{prefix}{index},")
            } else {
                let fields: Vec<String> = fields.iter().map(Type::text).collect();
                format!("{prefix}{index}({}),", fields.join(", "))
            }
        })
        .collect()
}

/// The canonical bytes of a crate: its profile's header and runtime, then
/// every item in order, each followed by one blank line. The layout is
/// fixed by this printer, so no formatter is involved.
#[must_use]
pub fn print(krate: &Crate) -> String {
    let mut printer = Printer {
        out: match krate.profile {
            Profile::Core => format!(
                "#![no_std]\n#![forbid(unsafe_code)]\n{}",
                super::runtime::CORE
            ),
            Profile::Std => format!(
                "#![forbid(unsafe_code)]\n{}{}",
                super::runtime::CORE,
                super::runtime::STD
            ),
        },
    };
    for item in &krate.items {
        printer.out.push('\n');
        printer.item(item);
    }
    printer.out
}

/// The canonical text of one expression.
#[must_use]
pub fn print_expr(expr: &Expr) -> String {
    Printer { out: String::new() }.expr(expr, 0)
}

/// The canonical text of a block at depth 0.
#[must_use]
pub fn print_block(block: &Block) -> String {
    Printer { out: String::new() }.block_inline(block, 0)
}

#[cfg(test)]
mod tests {
    use super::quoted;

    #[test]
    fn strings_are_escaped_by_lexlean_alone() {
        assert_eq!(quoted("a\"b\\c ~"), "\"a\\\"b\\\\c ~\"");
        assert_eq!(
            quoted("\n\t\u{0}\u{7f}\u{e9}\u{301}\u{200b}\u{feff}\u{1f600}"),
            "\"\\u{a}\\u{9}\\u{0}\\u{7f}\\u{e9}\\u{301}\\u{200b}\\u{feff}\\u{1f600}\""
        );
    }
}
