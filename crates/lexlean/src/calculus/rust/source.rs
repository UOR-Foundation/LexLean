//! The runtime profile as data (SPEC.md §17.16).
//!
//! The functions of the runtime both profiles carry are not text of this
//! crate. They are the definitions of one language-1.2 semantic module,
//! `RustRuntime`, held at [`MODULE_PATH`] in the embedded tree, which states
//! each as an `ItemDef` of `RustSyntax` and proves its refinement against
//! `RustSemantics`; this file is its *reader*, and `runtime::core` prints what
//! it reads.
//!
//! The module is decoded, never evaluated. The meaning of an `ItemDef` is
//! decided by the generated Lean and its kernel; an interpreter of terms here
//! would be an unverified second reading of the language this crate ships. The
//! decoder accepts exactly the constructors of `RustSyntax`, each with its own
//! arity, plus the closed reference to another definition of the module, and
//! refuses every other shape, so a change to the vocabulary is a refusal and
//! never a silent reading of less. A definition that takes parameters is
//! refused: the module holds data, and a call is only the name of a datum.

use std::collections::BTreeMap;

use super::super::{IntKind, OrderingValue};
use super::ast::{
    Block, Callee, CaptureRead, Crate, Ctor, Dispatch, Expr, Ident, ItemDef, Let, Lit, Machine,
    Origin, Pat, Type,
};
use super::runtime::Item;
use super::Profile;
use crate::artifact::canonical_json::Json;
use crate::surface::module::{self, SEMANTIC};

/// Where the module is in the embedded tree.
pub const MODULE_PATH: &str = "language/rust-runtime-1.2/RustRuntime.lex.tex";

/// The stack of the thread that decodes the module.
const DECODER_STACK: usize = crate::READER_STACK;

/// The origin every decoded construct carries: the runtime realizes no
/// calculus element, so nothing is checked against a correspondence table.
fn origin() -> Origin {
    Origin::of("runtime")
}

fn fail<T>(reason: impl Into<String>) -> Result<T, String> {
    Err(format!("runtime source {MODULE_PATH}: {}", reason.into()))
}

/// The two profile crates the module defines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    /// `coreCrate`: the items both profiles carry.
    pub core: Crate,
}

struct Reader<'a> {
    definitions: BTreeMap<&'a str, &'a Json>,
}

impl<'a> Reader<'a> {
    fn members(json: &'a Json, what: &str) -> Result<&'a BTreeMap<String, Json>, String> {
        match json {
            Json::Obj(members) => Ok(members),
            _ => fail(format!("{what} is not an object")),
        }
    }

    fn member(map: &'a BTreeMap<String, Json>, name: &str, what: &str) -> Result<&'a Json, String> {
        map.get(name)
            .map_or_else(|| fail(format!("{what} has no `{name}`")), Ok)
    }

    fn text(json: &'a Json, what: &str) -> Result<&'a str, String> {
        match json {
            Json::Str(text) => Ok(text),
            _ => fail(format!("{what} is not a string")),
        }
    }

    /// The term, with a reference to a definition of the module replaced by
    /// the datum it names.
    fn resolve(&self, term: &'a Json, what: &str) -> Result<&'a Json, String> {
        let mut term = term;
        for _ in 0..=self.definitions.len() {
            let map = Self::members(term, what)?;
            if Self::text(Self::member(map, "kind", what)?, what)? != "call" {
                return Ok(term);
            }
            let function = Self::members(Self::member(map, "function", what)?, what)?;
            let name = Self::text(Self::member(function, "name", what)?, what)?;
            let arguments = match Self::member(map, "arguments", what)? {
                Json::Arr(items) => items,
                _ => {
                    return fail(format!(
                        "{what} gives `{name}` arguments that are not a list"
                    ))
                }
            };
            if !arguments.is_empty() {
                return fail(format!("{what} applies `{name}`, which is data"));
            }
            term = self.definitions.get(name).copied().map_or_else(
                || {
                    fail(format!(
                        "{what} names `{name}`, which the module does not define"
                    ))
                },
                Ok,
            )?;
        }
        fail(format!("{what} names definitions that refer to each other"))
    }

    /// The constructor of `of` that `term` is, with its arguments.
    fn constructor(
        &self,
        term: &'a Json,
        module: &str,
        of: &str,
    ) -> Result<(&'a str, &'a [Json]), String> {
        let what = of;
        let term = self.resolve(term, what)?;
        let map = Self::members(term, what)?;
        if Self::text(Self::member(map, "kind", what)?, what)? != "constructor" {
            return fail(format!("a term is not a constructor of `{of}`"));
        }
        let named = Self::members(Self::member(map, "constructor", what)?, what)?;
        if Self::text(Self::member(named, "module", what)?, what)? != module {
            return fail(format!("a term is not a constructor of `{module}::{of}`"));
        }
        let name = Self::text(Self::member(named, "name", what)?, what)?;
        let short = name
            .strip_prefix(of)
            .and_then(|rest| rest.strip_prefix('.'))
            .map_or_else(
                || fail(format!("`{name}` is not a constructor of `{of}`")),
                Ok,
            )?;
        match Self::member(map, "arguments", what)? {
            Json::Arr(arguments) => Ok((short, arguments)),
            _ => fail(format!("`{name}` has arguments that are not a list")),
        }
    }

    fn arity<'t>(
        arguments: &'t [Json],
        count: usize,
        of: &str,
        short: &str,
    ) -> Result<&'t [Json], String> {
        if arguments.len() == count {
            Ok(arguments)
        } else {
            fail(format!(
                "`{of}.{short}` takes {count} arguments, the module gives {}",
                arguments.len()
            ))
        }
    }

    fn literal(&self, term: &'a Json, expected: &str, what: &str) -> Result<&'a Json, String> {
        let term = self.resolve(term, what)?;
        let map = Self::members(term, what)?;
        if Self::text(Self::member(map, "kind", what)?, what)? != expected {
            return fail(format!("{what} is not a {expected} literal"));
        }
        Self::member(map, "value", what)
    }

    fn nat(&self, term: &'a Json, what: &str) -> Result<u64, String> {
        Self::text(self.literal(term, "nat", what)?, what)?
            .parse()
            .map_or_else(|_| fail(format!("{what} is not a natural number")), Ok)
    }

    fn int(&self, term: &'a Json, what: &str) -> Result<i128, String> {
        let term = self.resolve(term, what)?;
        let map = Self::members(term, what)?;
        if Self::text(Self::member(map, "kind", what)?, what)? != "integer" {
            return fail(format!("{what} is not an integer literal"));
        }
        Self::text(Self::member(map, "value", what)?, what)?
            .parse()
            .map_or_else(|_| fail(format!("{what} is not an integer")), Ok)
    }

    fn boolean(&self, term: &'a Json, what: &str) -> Result<bool, String> {
        match self.literal(term, "bool", what)? {
            Json::Bool(value) => Ok(*value),
            _ => fail(format!("{what} is not a Boolean")),
        }
    }

    fn string(&self, term: &'a Json, what: &str) -> Result<String, String> {
        Ok(Self::text(self.literal(term, "string", what)?, what)?.to_owned())
    }

    fn bytes(&self, term: &'a Json, what: &str) -> Result<Vec<u8>, String> {
        let term = self.resolve(term, what)?;
        let map = Self::members(term, what)?;
        if Self::text(Self::member(map, "kind", what)?, what)? != "bytes" {
            return fail(format!("{what} is not a byte-string literal"));
        }
        let hex = Self::text(Self::member(map, "hex", what)?, what)?;
        if hex.len() % 2 != 0 {
            return fail(format!("{what} has an odd number of hexadecimal digits"));
        }
        (0..hex.len())
            .step_by(2)
            .map(|at| {
                hex.get(at..at + 2)
                    .and_then(|pair| u8::from_str_radix(pair, 16).ok())
                    .map_or_else(|| fail(format!("{what} is not hexadecimal")), Ok)
            })
            .collect()
    }

    /// The elements of a list term.
    fn list<T>(
        &self,
        term: &'a Json,
        what: &str,
        mut element: impl FnMut(&Self, &'a Json) -> Result<T, String>,
    ) -> Result<Vec<T>, String> {
        let mut out = Vec::new();
        let mut rest = term;
        loop {
            let resolved = self.resolve(rest, what)?;
            let map = Self::members(resolved, what)?;
            match Self::text(Self::member(map, "kind", what)?, what)? {
                "nil" => return Ok(out),
                "cons" => {
                    out.push(element(self, Self::member(map, "head", what)?)?);
                    rest = Self::member(map, "tail", what)?;
                }
                _ => return fail(format!("{what} is not a list of `cons` and `nil`")),
            }
        }
    }

    /// The fields of a record of `RustSyntax::of`, each by name, once.
    fn record<const N: usize>(
        &self,
        term: &'a Json,
        of: &str,
        fields: [&str; N],
    ) -> Result<[&'a Json; N], String> {
        let term = self.resolve(term, of)?;
        let map = Self::members(term, of)?;
        if Self::text(Self::member(map, "kind", of)?, of)? != "record" {
            return fail(format!("a term is not a record of `{of}`"));
        }
        let named = Self::members(Self::member(map, "type", of)?, of)?;
        if Self::text(Self::member(named, "name", of)?, of)? != of {
            return fail(format!("a term is not a record of `{of}`"));
        }
        let Json::Arr(given) = Self::member(map, "fields", of)? else {
            return fail(format!("a record of `{of}` has fields that are not a list"));
        };
        if given.len() != N {
            return fail(format!("a record of `{of}` does not state its {N} fields"));
        }
        let mut by_name = BTreeMap::new();
        for field in given {
            let field = Self::members(field, of)?;
            let name = Self::text(Self::member(field, "field", of)?, of)?;
            if by_name
                .insert(name, Self::member(field, "value", of)?)
                .is_some()
            {
                return fail(format!("a record of `{of}` states `{name}` twice"));
            }
        }
        let mut out = Vec::with_capacity(N);
        for name in fields {
            out.push(by_name.get(name).copied().map_or_else(
                || fail(format!("a record of `{of}` does not state `{name}`")),
                Ok,
            )?);
        }
        out.try_into().map_or_else(
            |_| fail(format!("a record of `{of}` has the wrong fields")),
            Ok,
        )
    }

    /// An `Option` term: its payload, if present.
    fn optional<T>(
        &self,
        term: &'a Json,
        what: &str,
        element: impl FnOnce(&Self, &'a Json) -> Result<T, String>,
    ) -> Result<Option<T>, String> {
        let term = self.resolve(term, what)?;
        let map = Self::members(term, what)?;
        if Self::text(Self::member(map, "kind", what)?, what)? != "constructor" {
            return fail(format!("{what} is not an `Option`"));
        }
        let named = Self::members(Self::member(map, "constructor", what)?, what)?;
        let arguments = match Self::member(map, "arguments", what)? {
            Json::Arr(arguments) => arguments,
            _ => return fail(format!("{what} has arguments that are not a list")),
        };
        match (
            Self::text(Self::member(named, "name", what)?, what)?,
            arguments.as_slice(),
        ) {
            ("Option.none", []) => Ok(None),
            ("Option.some", [payload]) => element(self, payload).map(Some),
            _ => fail(format!("{what} is not an `Option`")),
        }
    }

    fn kind(&self, term: &'a Json) -> Result<IntKind, String> {
        let (short, arguments) = self.constructor(term, "TargetSyntax", "IntKind")?;
        Self::arity(arguments, 0, "IntKind", short)?;
        IntKind::ALL
            .into_iter()
            .find(|kind| kind.name() == short)
            .map_or_else(|| fail(format!("`IntKind.{short}` is not a width")), Ok)
    }

    fn order(&self, term: &'a Json) -> Result<OrderingValue, String> {
        let (short, arguments) = self.constructor(term, "TargetSyntax", "Order")?;
        Self::arity(arguments, 0, "Order", short)?;
        match short {
            "less" => Ok(OrderingValue::Lt),
            "same" => Ok(OrderingValue::Eq),
            "more" => Ok(OrderingValue::Gt),
            _ => fail(format!("`Order.{short}` is not an order")),
        }
    }

    fn ident(&self, term: &'a Json) -> Result<Ident, String> {
        let (short, arguments) = self.constructor(term, "RustSyntax", "Ident")?;
        match short {
            "generated" => {
                let arguments = Self::arity(arguments, 2, "Ident", short)?;
                let (kind, _) = self.constructor(&arguments[0], "RustSyntax", "IdentKind")?;
                let index = self.nat(&arguments[1], "an identifier index")?;
                Ok(match kind {
                    "binding" => Ident::Local(index),
                    "holder" => Ident::Holder(index),
                    "operand" => Ident::Operand(index),
                    "callee" => Ident::Callee(index),
                    "boxed" => Ident::Boxed(index),
                    "part" => Ident::Part(index),
                    "capture" => Ident::Capture(index),
                    "param" => Ident::Param(index),
                    "function" => Ident::Function(index),
                    other => return fail(format!("`IdentKind.{other}` is not an identifier kind")),
                })
            }
            "exported" => {
                let arguments = Self::arity(arguments, 1, "Ident", short)?;
                Ok(Ident::Export(
                    self.string(&arguments[0], "an exported name")?,
                ))
            }
            other => fail(format!("`Ident.{other}` is not an identifier")),
        }
    }

    fn ty(&self, term: &'a Json) -> Result<Type, String> {
        let (short, a) = self.constructor(term, "RustSyntax", "RustType")?;
        let take = |count| Self::arity(a, count, "RustType", short);
        let boxed = |this: &Self, at: &'a Json| this.ty(at).map(Box::new);
        Ok(match short {
            "unit" => {
                take(0)?;
                Type::Unit
            }
            "bool" => {
                take(0)?;
                Type::Bool
            }
            "nat" => {
                take(0)?;
                Type::Nat
            }
            "int" => {
                take(0)?;
                Type::Int
            }
            "fixed" => Type::Fixed(self.kind(&take(1)?[0])?),
            "ordering" => {
                take(0)?;
                Type::Ordering
            }
            "str" => {
                take(0)?;
                Type::Str
            }
            "bytes" => {
                take(0)?;
                Type::Bytes
            }
            "option" => Type::Option(boxed(self, &take(1)?[0])?),
            "result" => {
                let a = take(2)?;
                Type::Result(boxed(self, &a[0])?, boxed(self, &a[1])?)
            }
            "list" => Type::List(boxed(self, &take(1)?[0])?),
            "pair" => {
                let a = take(2)?;
                Type::Pair(boxed(self, &a[0])?, boxed(self, &a[1])?)
            }
            "adt" => Type::Adt(self.nat(&take(1)?[0], "an ADT number")?),
            "fn" => Type::Fn(self.nat(&take(1)?[0], "a function type number")?),
            "rc" => Type::Rc(boxed(self, &take(1)?[0])?),
            "fallible" => Type::Fallible(boxed(self, &take(1)?[0])?),
            "ref" => Type::Ref(boxed(self, &take(1)?[0])?),
            "overflow" => {
                take(0)?;
                Type::Overflow
            }
            "i128" => {
                take(0)?;
                Type::I128
            }
            other => return fail(format!("`RustType.{other}` is not a type")),
        })
    }

    fn literal_value(&self, term: &'a Json) -> Result<Lit, String> {
        let (short, a) = self.constructor(term, "RustSyntax", "Lit")?;
        let take = |count| Self::arity(a, count, "Lit", short);
        Ok(match short {
            "unit" => {
                take(0)?;
                Lit::Unit
            }
            "bool" => Lit::Bool(self.boolean(&take(1)?[0], "a Boolean literal")?),
            "nat" => Lit::Nat(self.nat(&take(1)?[0], "a natural literal")?),
            "int" => {
                let value = self.int(&take(1)?[0], "an integer literal")?;
                Lit::Int(
                    i64::try_from(value)
                        .map_or_else(|_| fail("an integer literal leaves `i64`"), Ok)?,
                )
            }
            "fixed" => {
                let a = take(2)?;
                Lit::Fixed(self.kind(&a[0])?, self.int(&a[1], "a fixed-width literal")?)
            }
            "str" => Lit::Str(self.string(&take(1)?[0], "a string literal")?),
            "bytes" => Lit::Bytes(self.bytes(&take(1)?[0], "a byte-string literal")?),
            "ordering" => Lit::Ordering(self.order(&take(1)?[0])?),
            "overflow" => {
                take(0)?;
                Lit::Overflow
            }
            other => return fail(format!("`Lit.{other}` is not a literal")),
        })
    }

    fn ctor(&self, term: &'a Json) -> Result<Ctor, String> {
        let (short, a) = self.constructor(term, "RustSyntax", "Ctor")?;
        let take = |count| Self::arity(a, count, "Ctor", short);
        Ok(match short {
            "none" => Ctor::None(self.ty(&take(1)?[0])?),
            "some" => {
                take(0)?;
                Ctor::Some
            }
            "ok" => {
                let a = take(2)?;
                Ctor::Ok(self.ty(&a[0])?, self.ty(&a[1])?)
            }
            "err" => {
                let a = take(2)?;
                Ctor::Err(self.ty(&a[0])?, self.ty(&a[1])?)
            }
            "adt" => {
                let a = take(2)?;
                Ctor::Adt {
                    adt: self.nat(&a[0], "an ADT number")?,
                    constructor: self.nat(&a[1], "a constructor number")?,
                }
            }
            "closure" => {
                let a = take(2)?;
                Ctor::Closure {
                    fn_type: self.nat(&a[0], "a function type number")?,
                    function: self.nat(&a[1], "a function number")?,
                }
            }
            "cons" => {
                take(0)?;
                Ctor::Cons
            }
            "nil" => Ctor::Nil(self.ty(&take(1)?[0])?),
            other => return fail(format!("`Ctor.{other}` is not a constructor")),
        })
    }

    fn pat(&self, term: &'a Json) -> Result<Pat, String> {
        let (short, a) = self.constructor(term, "RustSyntax", "Pat")?;
        let take = |count| Self::arity(a, count, "Pat", short);
        Ok(match short {
            "wild" => {
                take(0)?;
                Pat::Wild
            }
            "bind" => Pat::Bind(self.ident(&take(1)?[0])?),
            "unit" => {
                take(0)?;
                Pat::Unit
            }
            "tuple" => Pat::Tuple(self.list(&take(1)?[0], "a tuple pattern", Self::pat)?),
            "none" => {
                take(0)?;
                Pat::None
            }
            "some" => Pat::Some(Box::new(self.pat(&take(1)?[0])?)),
            "ok" => Pat::Ok(Box::new(self.pat(&take(1)?[0])?)),
            "err" => Pat::Err(Box::new(self.pat(&take(1)?[0])?)),
            "ordering" => Pat::Ordering(self.order(&take(1)?[0])?),
            "adt" => {
                let a = take(3)?;
                Pat::Adt {
                    adt: self.nat(&a[0], "an ADT number")?,
                    constructor: self.nat(&a[1], "a constructor number")?,
                    fields: self.list(&a[2], "the fields of a pattern", Self::pat)?,
                }
            }
            other => return fail(format!("`Pat.{other}` is not a pattern")),
        })
    }

    /// Whether the Lean constructor `short` with `width` is the Rust variant
    /// whose `Debug` form is `variant`: the variant's name with a lower-case
    /// first letter, and its width, if it has one, in lower case.
    fn is_variant(variant: &str, short: &str, width: Option<IntKind>) -> bool {
        let (head, rest) = match variant.split_once('(') {
            Some((head, rest)) => (head, Some(rest.trim_end_matches(')').to_lowercase())),
            None => (variant, None),
        };
        let mut letters = head.chars();
        let name: String = letters
            .next()
            .map(|first| first.to_lowercase().chain(letters).collect())
            .unwrap_or_default();
        name == short && rest == width.map(|kind| kind.name().to_owned())
    }

    fn item(&self, term: &'a Json) -> Result<Item, String> {
        let (short, a) = self.constructor(term, "RustSyntax", "Item")?;
        let width = match a {
            [] => None,
            [kind] => Some(self.kind(kind)?),
            _ => return fail(format!("`Item.{short}` takes at most a width")),
        };
        Item::all()
            .into_iter()
            .find(|item| Self::is_variant(&format!("{item:?}"), short, width))
            .map_or_else(
                || fail(format!("`Item.{short}` is not a runtime function")),
                Ok,
            )
    }

    fn machine(&self, term: &'a Json) -> Result<Machine, String> {
        let (short, a) = self.constructor(term, "RustSyntax", "Machine")?;
        let width = match a {
            [] => None,
            [kind] => Some(self.kind(kind)?),
            _ => return fail(format!("`Machine.{short}` takes at most a width")),
        };
        Machine::all()
            .into_iter()
            .find(|machine| Self::is_variant(&format!("{machine:?}"), short, width))
            .map_or_else(
                || fail(format!("`Machine.{short}` is not a machine primitive")),
                Ok,
            )
    }

    fn callee(&self, term: &'a Json) -> Result<Callee, String> {
        let (short, a) = self.constructor(term, "RustSyntax", "Callee")?;
        let take = |count| Self::arity(a, count, "Callee", short);
        Ok(match short {
            "function" => Callee::Function(self.nat(&take(1)?[0], "a function number")?),
            "runtime" => Callee::Runtime(self.item(&take(1)?[0])?),
            "machine" => Callee::Machine(self.machine(&take(1)?[0])?),
            other => return fail(format!("`Callee.{other}` is not a callee")),
        })
    }

    fn block(&self, term: &'a Json) -> Result<Block, String> {
        let (short, a) = self.constructor(term, "RustSyntax", "Block")?;
        if short != "mk" {
            return fail(format!("`Block.{short}` is not a block"));
        }
        let a = Self::arity(a, 2, "Block", short)?;
        Ok(Block {
            lets: self.list(&a[0], "the bindings of a block", Self::binding)?,
            tail: self.expr(&a[1])?,
        })
    }

    fn binding(&self, term: &'a Json) -> Result<Let, String> {
        let (short, a) = self.constructor(term, "RustSyntax", "Let")?;
        if short != "mk" {
            return fail(format!("`Let.{short}` is not a binding"));
        }
        let a = Self::arity(a, 3, "Let", short)?;
        Ok(Let {
            pat: self.pat(&a[0])?,
            ty: self.optional(&a[1], "the type of a binding", Self::ty)?,
            value: self.expr(&a[2])?,
            at: origin(),
        })
    }

    fn arm(&self, term: &'a Json) -> Result<(Pat, Block), String> {
        let (short, a) = self.constructor(term, "RustSyntax", "Arm")?;
        if short != "mk" {
            return fail(format!("`Arm.{short}` is not an arm"));
        }
        let a = Self::arity(a, 2, "Arm", short)?;
        Ok((self.pat(&a[0])?, self.block(&a[1])?))
    }

    fn expr(&self, term: &'a Json) -> Result<Expr, String> {
        let (short, a) = self.constructor(term, "RustSyntax", "Expr")?;
        let take = |count| Self::arity(a, count, "Expr", short);
        let boxed = |this: &Self, at: &'a Json| this.expr(at).map(Box::new);
        let exprs = |this: &Self, at: &'a Json| this.list(at, "operands", Self::expr);
        Ok(match short {
            "lit" => Expr::Lit(self.literal_value(&take(1)?[0])?, origin()),
            "move" => Expr::Move(self.ident(&take(1)?[0])?, origin()),
            "clone" => Expr::Clone(self.ident(&take(1)?[0])?, origin()),
            "copy" => Expr::Copy(self.ident(&take(1)?[0])?, origin()),
            "deref" => Expr::Deref(self.ident(&take(1)?[0])?, origin()),
            "not" => Expr::Not(boxed(self, &take(1)?[0])?, origin()),
            "unbox" => Expr::Unbox(self.ident(&take(1)?[0])?, origin()),
            "box" => Expr::Box(boxed(self, &take(1)?[0])?, origin()),
            "call" => {
                let a = take(3)?;
                Expr::Call {
                    callee: self.callee(&a[0])?,
                    args: exprs(self, &a[1])?,
                    propagate: self.boolean(&a[2], "a propagation flag")?,
                    at: origin(),
                }
            }
            "apply" => {
                let a = take(3)?;
                Expr::Apply {
                    holder: self.ident(&a[0])?,
                    args: exprs(self, &a[1])?,
                    propagate: self.boolean(&a[2], "a propagation flag")?,
                    at: origin(),
                }
            }
            "construct" => {
                let a = take(2)?;
                Expr::Construct {
                    ctor: self.ctor(&a[0])?,
                    args: exprs(self, &a[1])?,
                    at: origin(),
                }
            }
            "pair" => {
                let a = take(2)?;
                Expr::Pair(boxed(self, &a[0])?, boxed(self, &a[1])?, origin())
            }
            "cond" => {
                let a = take(3)?;
                Expr::If {
                    condition: boxed(self, &a[0])?,
                    then_branch: Box::new(self.block(&a[1])?),
                    else_branch: Box::new(self.block(&a[2])?),
                    at: origin(),
                }
            }
            "matchOn" => {
                let a = take(2)?;
                Expr::Match {
                    scrutinee: boxed(self, &a[0])?,
                    arms: self.list(&a[1], "the arms of a match", Self::arm)?,
                    at: origin(),
                }
            }
            "block" => Expr::Block(Box::new(self.block(&take(1)?[0])?)),
            "uncons" => Expr::Uncons(self.ident(&take(1)?[0])?),
            "isZero" => Expr::IsZero(self.ident(&take(1)?[0])?, origin()),
            "nonZero" => Expr::NonZero(self.ident(&take(1)?[0])?, origin()),
            "predecessor" => Expr::Predecessor(self.ident(&take(1)?[0])?),
            "widen" => Expr::Widen(boxed(self, &take(1)?[0])?, origin()),
            "succeed" => Expr::Succeed(boxed(self, &take(1)?[0])?, origin()),
            "loop" => {
                let a = take(2)?;
                Expr::Loop {
                    state: self.ident(&a[0])?,
                    body: Box::new(self.block(&a[1])?),
                }
            }
            other => return fail(format!("`Expr.{other}` is not an expression")),
        })
    }

    fn capture(&self, term: &'a Json) -> Result<CaptureRead, String> {
        let (short, a) = self.constructor(term, "RustSyntax", "CaptureRead")?;
        Self::arity(a, 0, "CaptureRead", short)?;
        match short {
            "copy" => Ok(CaptureRead::Copy),
            "clone" => Ok(CaptureRead::Clone),
            "unbox" => Ok(CaptureRead::Unbox),
            "unit" => Ok(CaptureRead::Unit),
            other => fail(format!("`CaptureRead.{other}` is not a capture read")),
        }
    }

    fn dispatch(&self, term: &'a Json) -> Result<Dispatch, String> {
        let [function, captures, fallible] = self.record(
            term,
            "Dispatch",
            ["function", "captures", "functionFallible"],
        )?;
        Ok(Dispatch {
            function: self.nat(function, "a function number")?,
            captures: self.list(captures, "the captures of a dispatch", Self::capture)?,
            function_fallible: self.boolean(fallible, "a fallibility flag")?,
        })
    }

    fn variant(&self, term: &'a Json) -> Result<(u64, Vec<Type>), String> {
        let [number, fields] = self.record(term, "Variant", ["number", "fields"])?;
        Ok((
            self.nat(number, "a variant number")?,
            self.list(fields, "the fields of a variant", Self::ty)?,
        ))
    }

    fn parameter(&self, term: &'a Json) -> Result<(Pat, Type), String> {
        let [pattern, ty] = self.record(term, "Parameter", ["pattern", "type"])?;
        Ok((self.pat(pattern)?, self.ty(ty)?))
    }

    fn item_def(&self, term: &'a Json) -> Result<ItemDef, String> {
        let (short, a) = self.constructor(term, "RustSyntax", "ItemDef")?;
        let take = |count| Self::arity(a, count, "ItemDef", short);
        Ok(match short {
            "enum" => {
                let a = take(2)?;
                ItemDef::Enum {
                    name: self.ty(&a[0])?,
                    variants: self.list(&a[1], "the variants of an enum", Self::variant)?,
                    at: origin(),
                }
            }
            "apply" => {
                let a = take(5)?;
                ItemDef::Apply {
                    fn_type: self.nat(&a[0], "a function type number")?,
                    parameters: self.list(&a[1], "the parameters of an apply", Self::ty)?,
                    result: self.ty(&a[2])?,
                    fallible: self.boolean(&a[3], "a fallibility flag")?,
                    arms: self.list(&a[4], "the arms of an apply", Self::dispatch)?,
                    at: origin(),
                }
            }
            "function" => {
                let a = take(4)?;
                ItemDef::Function {
                    name: self.ident(&a[0])?,
                    parameters: self.list(
                        &a[1],
                        "the parameters of a function",
                        Self::parameter,
                    )?,
                    result: self.ty(&a[2])?,
                    body: self.block(&a[3])?,
                    at: origin(),
                }
            }
            other => return fail(format!("`ItemDef.{other}` is not an item")),
        })
    }

    fn profile(&self, term: &'a Json) -> Result<Profile, String> {
        let (short, a) = self.constructor(term, "RustSyntax", "Profile")?;
        Self::arity(a, 0, "Profile", short)?;
        match short {
            "core" => Ok(Profile::Core),
            "std" => Ok(Profile::Std),
            other => fail(format!("`Profile.{other}` is not a profile")),
        }
    }

    fn krate(&self, term: &'a Json) -> Result<Crate, String> {
        let [profile, items] = self.record(term, "Crate", ["profile", "items"])?;
        Ok(Crate {
            profile: self.profile(profile)?,
            items: self.list(items, "the items of a crate", Self::item_def)?,
        })
    }
}

/// Decode the module's source.
///
/// # Errors
///
/// The first rule the module breaks: it is not a semantic module, it defines
/// a name twice or one with parameters, it does not define `coreCrate` as a
/// core crate, or a term is not a constructor of `RustSyntax` with its
/// arity.
pub fn decode(source: &str) -> Result<Source, String> {
    // The module is a list of a hundred functions and the notation, the data,
    // and the terms nest with it, so the decoder runs on a stack of its own
    // rather than on the caller's, which a test thread or a debug build gives
    // two megabytes.
    std::thread::scope(|scope| {
        std::thread::Builder::new()
            .stack_size(DECODER_STACK)
            .spawn_scoped(scope, || decode_here(source))
            .map_err(|error| format!("runtime source {MODULE_PATH}: {error}"))?
            .join()
            .map_err(|_| format!("runtime source {MODULE_PATH}: the decoder panicked"))?
    })
}

fn decode_here(source: &str) -> Result<Source, String> {
    let Some(found) = module::locate(source) else {
        return fail("holds no module");
    };
    if !found.kind.is(&SEMANTIC) {
        return fail("is not a semantic module");
    }
    let data = match found.data {
        Some((start, end)) => source[start..end].to_owned(),
        None => {
            let spec = (found.kind.spec)().map_or_else(|| fail("has no schema"), Ok)?;
            found
                .kind
                .data_text(&source[found.body.0..found.body.1], spec)
                .map_err(|error| format!("runtime source {MODULE_PATH}: {}", error.message))?
        }
    };
    let json = Json::parse(data.as_bytes())
        .map_err(|error| format!("runtime source {MODULE_PATH}: {error}"))?;
    let root = Reader::members(&json, "the module")?;
    let Json::Arr(declarations) = Reader::member(root, "declarations", "the module")? else {
        return fail("`declarations` is not a list");
    };
    let mut definitions = BTreeMap::new();
    for declaration in declarations {
        let declaration = Reader::members(declaration, "a declaration")?;
        let name = Reader::text(
            Reader::member(declaration, "name", "a declaration")?,
            "a name",
        )?;
        match Reader::text(Reader::member(declaration, "kind", name)?, name)? {
            "definition" => {
                match Reader::member(declaration, "parameters", name)? {
                    Json::Arr(parameters) if parameters.is_empty() => {}
                    _ => return fail(format!("`{name}` takes parameters; the module holds data")),
                }
                let body = Reader::member(declaration, "body", name)?;
                if definitions.insert(name, body).is_some() {
                    return fail(format!("defines `{name}` twice"));
                }
            }
            // The kernel decides a theorem; the reader has no use for one.
            "theorem" => {}
            other => {
                return fail(format!(
                    "`{name}` is a {other}, which the reader does not read"
                ))
            }
        }
    }
    let reader = Reader { definitions };
    let Some(core) = reader.definitions.get("coreCrate") else {
        return fail("does not define `coreCrate`");
    };
    let core = reader.krate(core)?;
    if core.profile != Profile::Core {
        return fail("`coreCrate` is not a core crate");
    }
    Ok(Source { core })
}

/// The text of the embedded module.
///
/// # Errors
///
/// The module is not in the embedded tree, or is not UTF-8.
pub fn embedded() -> Result<&'static str, String> {
    let (_, bytes) = crate::embedded::FILES
        .iter()
        .find(|(path, _)| *path == MODULE_PATH)
        .map_or_else(|| fail("is not in the embedded tree"), Ok)?;
    std::str::from_utf8(bytes).map_err(|_| format!("runtime source {MODULE_PATH}: is not UTF-8"))
}
