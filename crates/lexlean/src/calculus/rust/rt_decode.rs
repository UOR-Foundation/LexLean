//! The reader of `RuntimeSyntax` data: the `supportCrate` of `RuntimeSupport`.
//!
//! The same discipline as `source`: the module is decoded and never
//! evaluated; the reader accepts exactly the constructors of `RuntimeSyntax`,
//! each with its own arity, and refuses every other shape by name.

use super::rt::{
    Arm, Block, Crate, Expr, Function, Generic, Item, Lit, Op, Param, Pat, Profile, Sig, Type,
    Variant,
};
use super::source::{embedded_file, fail, with_reader, Reader};
use crate::artifact::canonical_json::Json;

/// Where the module is in the embedded tree.
pub const MODULE_PATH: &str = "language/rust-runtime-1.2/RuntimeSupport.lex.tex";

/// Where the module of the std runtime's types, impls, and helpers is.
pub const STD_PATH: &str = "language/rust-runtime-1.2/RuntimeStd.lex.tex";

const SYNTAX: &str = "RuntimeSyntax";

impl<'a> Reader<'a> {
    fn rt_type(&self, term: &'a Json) -> Result<Type, String> {
        let (short, a) = self.constructor(term, SYNTAX, "RtType")?;
        let take = |count| Self::arity(a, count, "RtType", short);
        let boxed = |this: &Self, at: &'a Json| this.rt_type(at).map(Box::new);
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
            "fixed" => Type::Fixed(self.kind(&take(1)?[0])?),
            "ordering" => {
                take(0)?;
                Type::Ordering
            }
            "selfType" => {
                take(0)?;
                Type::SelfType
            }
            "param" => Type::Param(self.nat(&take(1)?[0], "a parameter index")?),
            "ref" => Type::Ref(boxed(self, &take(1)?[0])?),
            "pair" => {
                let a = take(2)?;
                Type::Pair(boxed(self, &a[0])?, boxed(self, &a[1])?)
            }
            "named" => {
                let a = take(2)?;
                Type::Named(
                    self.string(&a[0], "a type name")?,
                    self.list(&a[1], "type arguments", Self::rt_type)?,
                )
            }
            "wide" => {
                Type::Wide(self.boolean(&take(1)?[0], "a flag, true for u128 and false for i128")?)
            }
            "size" => {
                take(0)?;
                Type::Size
            }
            "char" => {
                take(0)?;
                Type::Char
            }
            "str" => {
                take(0)?;
                Type::Str
            }
            "slice" => Type::Slice(boxed(self, &take(1)?[0])?),
            "refMut" => Type::RefMut(boxed(self, &take(1)?[0])?),
            other => return fail(format!("`RtType.{other}` is not a type")),
        })
    }

    fn rt_lit(&self, term: &'a Json) -> Result<Lit, String> {
        let (short, a) = self.constructor(term, SYNTAX, "RtLit")?;
        Ok(match short {
            "unit" => {
                Self::arity(a, 0, "RtLit", short)?;
                Lit::Unit
            }
            "bool" => Lit::Bool(self.boolean(&Self::arity(a, 1, "RtLit", short)?[0], "a literal")?),
            "nat" => Lit::Nat(self.nat(&Self::arity(a, 1, "RtLit", short)?[0], "a literal")?),
            "char" => {
                Lit::Char(self.string(&Self::arity(a, 1, "RtLit", short)?[0], "a character")?)
            }
            "str" => Lit::Str(self.string(&Self::arity(a, 1, "RtLit", short)?[0], "a string")?),
            other => return fail(format!("`RtLit.{other}` is not a literal")),
        })
    }

    fn path(&self, term: &'a Json) -> Result<Vec<String>, String> {
        self.list(term, "a path", |this, at| this.string(at, "a path segment"))
    }

    fn rt_pat(&self, term: &'a Json) -> Result<Pat, String> {
        let (short, a) = self.constructor(term, SYNTAX, "RtPat")?;
        let take = |count| Self::arity(a, count, "RtPat", short);
        let pats = |this: &Self, at: &'a Json| this.list(at, "patterns", Self::rt_pat);
        Ok(match short {
            "wild" => {
                take(0)?;
                Pat::Wild
            }
            "bind" => Pat::Bind(self.string(&take(1)?[0], "a binding")?),
            "path" => Pat::Path(self.path(&take(1)?[0])?),
            "bindMut" => Pat::BindMut(self.string(&take(1)?[0], "a mutable binding")?),
            "tuple" => Pat::Tuple(pats(self, &take(1)?[0])?),
            "tupleStruct" => {
                let a = take(2)?;
                Pat::TupleStruct(self.path(&a[0])?, pats(self, &a[1])?)
            }
            "structPat" => {
                let a = take(3)?;
                Pat::Struct(self.path(&a[0])?, self.names(&a[1])?, pats(self, &a[2])?)
            }
            "lit" => Pat::Lit(self.rt_lit(&take(1)?[0])?),
            other => return fail(format!("`RtPat.{other}` is not a pattern")),
        })
    }

    fn names(&self, term: &'a Json) -> Result<Vec<String>, String> {
        self.list(term, "names", |this, at| this.string(at, "a name"))
    }

    fn op(&self, term: &'a Json) -> Result<Op, String> {
        let (short, a) = self.constructor(term, SYNTAX, "RtOp")?;
        Self::arity(a, 0, "RtOp", short)?;
        Ok(match short {
            "add" => Op::Add,
            "sub" => Op::Sub,
            "mul" => Op::Mul,
            "div" => Op::Div,
            "rem" => Op::Rem,
            "andAnd" => Op::AndAnd,
            "orOr" => Op::OrOr,
            "ne" => Op::Ne,
            "lt" => Op::Lt,
            "le" => Op::Le,
            "gt" => Op::Gt,
            "ge" => Op::Ge,
            other => return fail(format!("`RtOp.{other}` is not an operator")),
        })
    }

    fn rt_arm(&self, term: &'a Json) -> Result<Arm, String> {
        let (short, a) = self.constructor(term, SYNTAX, "RtArm")?;
        match short {
            "mk" => {
                let a = Self::arity(a, 2, "RtArm", short)?;
                Ok(Arm {
                    pattern: self.rt_pat(&a[0])?,
                    guard: None,
                    body: self.rt_expr(&a[1])?,
                })
            }
            "guarded" => {
                let a = Self::arity(a, 3, "RtArm", short)?;
                Ok(Arm {
                    pattern: self.rt_pat(&a[0])?,
                    guard: Some(self.rt_expr(&a[1])?),
                    body: self.rt_expr(&a[2])?,
                })
            }
            other => fail(format!("`RtArm.{other}` is not an arm")),
        }
    }

    fn rt_expr(&self, term: &'a Json) -> Result<Expr, String> {
        let (short, a) = self.constructor(term, SYNTAX, "RtExpr")?;
        let take = |count| Self::arity(a, count, "RtExpr", short);
        let boxed = |this: &Self, at: &'a Json| this.rt_expr(at).map(Box::new);
        let exprs = |this: &Self, at: &'a Json| this.list(at, "arguments", Self::rt_expr);
        let optional = |this: &Self, at: &'a Json| {
            this.optional(at, "an optional expression", |t, x| {
                t.rt_expr(x).map(Box::new)
            })
        };
        Ok(match short {
            "var" => Expr::Var(self.string(&take(1)?[0], "a variable")?),
            "lit" => Expr::Lit(self.rt_lit(&take(1)?[0])?),
            "path" => Expr::Path(self.path(&take(1)?[0])?),
            "call" => {
                let a = take(2)?;
                Expr::Call(self.path(&a[0])?, exprs(self, &a[1])?)
            }
            "method" => {
                let a = take(3)?;
                Expr::Method(
                    boxed(self, &a[0])?,
                    self.string(&a[1], "a method")?,
                    exprs(self, &a[2])?,
                )
            }
            "field" => {
                let a = take(2)?;
                Expr::Field(boxed(self, &a[0])?, self.nat(&a[1], "a field")?)
            }
            "fieldNamed" => {
                let a = take(2)?;
                Expr::FieldNamed(boxed(self, &a[0])?, self.string(&a[1], "a field")?)
            }
            "borrow" => Expr::Borrow(boxed(self, &take(1)?[0])?),
            "equals" => {
                let a = take(2)?;
                Expr::Equals(boxed(self, &a[0])?, boxed(self, &a[1])?)
            }
            "matchOn" => {
                let a = take(2)?;
                Expr::Match(boxed(self, &a[0])?, self.list(&a[1], "arms", Self::rt_arm)?)
            }
            "letE" => {
                let a = take(3)?;
                Expr::Let(
                    self.rt_pat(&a[0])?,
                    self.optional(&a[1], "a type", Self::rt_type)?,
                    boxed(self, &a[2])?,
                )
            }
            "assign" => {
                let a = take(2)?;
                Expr::Assign(boxed(self, &a[0])?, boxed(self, &a[1])?)
            }
            "ifElse" => {
                let a = take(3)?;
                Expr::If(
                    boxed(self, &a[0])?,
                    Box::new(self.rt_block(&a[1])?),
                    Box::new(self.rt_block(&a[2])?),
                )
            }
            "whileLoop" => {
                let a = take(2)?;
                Expr::While(boxed(self, &a[0])?, Box::new(self.rt_block(&a[1])?))
            }
            "whileLet" => {
                let a = take(3)?;
                Expr::WhileLet(
                    self.rt_pat(&a[0])?,
                    boxed(self, &a[1])?,
                    Box::new(self.rt_block(&a[2])?),
                )
            }
            "loopE" => Expr::Loop(Box::new(self.rt_block(&take(1)?[0])?)),
            "forIn" => {
                let a = take(3)?;
                Expr::For(
                    self.rt_pat(&a[0])?,
                    boxed(self, &a[1])?,
                    Box::new(self.rt_block(&a[2])?),
                )
            }
            "breakE" => {
                take(0)?;
                Expr::Break
            }
            "returnE" => Expr::Return(optional(self, &take(1)?[0])?),
            "tuple" => Expr::Tuple(exprs(self, &take(1)?[0])?),
            "structLit" => {
                let a = take(3)?;
                Expr::StructLit(self.path(&a[0])?, self.names(&a[1])?, exprs(self, &a[2])?)
            }
            "binary" => {
                let a = take(3)?;
                Expr::Binary(self.op(&a[0])?, boxed(self, &a[1])?, boxed(self, &a[2])?)
            }
            "notE" => Expr::Not(boxed(self, &take(1)?[0])?),
            "cast" => {
                let a = take(2)?;
                Expr::Cast(boxed(self, &a[0])?, self.rt_type(&a[1])?)
            }
            "index" => {
                let a = take(2)?;
                Expr::Index(boxed(self, &a[0])?, boxed(self, &a[1])?)
            }
            "rangeSlice" => {
                let a = take(3)?;
                Expr::RangeSlice(
                    boxed(self, &a[0])?,
                    optional(self, &a[1])?,
                    optional(self, &a[2])?,
                )
            }
            "deref" => Expr::Deref(boxed(self, &take(1)?[0])?),
            "borrowMut" => Expr::BorrowMut(boxed(self, &take(1)?[0])?),
            "methodOf" => {
                let a = take(4)?;
                Expr::MethodOf(
                    boxed(self, &a[0])?,
                    self.string(&a[1], "a method")?,
                    self.list(&a[2], "type arguments", Self::rt_type)?,
                    exprs(self, &a[3])?,
                )
            }
            "blockE" => Expr::Block(Box::new(self.rt_block(&take(1)?[0])?)),
            other => return fail(format!("`RtExpr.{other}` is not an expression")),
        })
    }

    fn rt_block(&self, term: &'a Json) -> Result<Block, String> {
        let (short, a) = self.constructor(term, SYNTAX, "RtBlock")?;
        let a = Self::arity(a, 2, "RtBlock", short)?;
        Ok(Block {
            statements: self.list(&a[0], "statements", Self::rt_expr)?,
            tail: self.rt_expr(&a[1])?,
        })
    }

    fn rt_generic(&self, term: &'a Json) -> Result<Generic, String> {
        let [name, bounds] = self.record(term, "RtGeneric", ["name", "bounds"])?;
        Ok(Generic {
            name: self.string(name, "a generic")?,
            bounds: self.names(bounds)?,
        })
    }

    fn rt_sig(&self, term: &'a Json) -> Result<Sig, String> {
        let [name, receiver, exclusive, params, result] = self.record(
            term,
            "RtSig",
            ["name", "receiver", "exclusive", "parameters", "result"],
        )?;
        Ok(Sig {
            name: self.string(name, "a function name")?,
            receiver: self.boolean(receiver, "a receiver")?,
            exclusive: self.boolean(exclusive, "an exclusive receiver")?,
            params: self.list(params, "parameters", |this, at| {
                let [name, mutable, ty] =
                    this.record(at, "RtParam", ["name", "mutable", "type"])?;
                Ok(Param {
                    name: this.string(name, "a parameter")?,
                    mutable: this.boolean(mutable, "a mutable parameter")?,
                    ty: this.rt_type(ty)?,
                })
            })?,
            result: self.rt_type(result)?,
        })
    }

    fn rt_fn(&self, term: &'a Json) -> Result<Function, String> {
        let [sig, generics, public, body] =
            self.record(term, "RtFn", ["signature", "generics", "public", "body"])?;
        Ok(Function {
            sig: self.rt_sig(sig)?,
            generics: self.list(generics, "generics", Self::rt_generic)?,
            public: self.boolean(public, "a visibility")?,
            body: self.rt_block(body)?,
        })
    }

    fn rt_variant(&self, term: &'a Json) -> Result<Variant, String> {
        let (short, a) = self.constructor(term, SYNTAX, "RtVariant")?;
        let take = |count| Self::arity(a, count, "RtVariant", short);
        Ok(match short {
            "unitV" => Variant::Unit(self.string(&take(1)?[0], "a variant")?),
            "tupleV" => {
                let a = take(2)?;
                Variant::Tuple(
                    self.string(&a[0], "a variant")?,
                    self.list(&a[1], "types", Self::rt_type)?,
                )
            }
            "structV" => {
                let a = take(3)?;
                Variant::Record(
                    self.string(&a[0], "a variant")?,
                    self.names(&a[1])?,
                    self.list(&a[2], "types", Self::rt_type)?,
                )
            }
            other => return fail(format!("`RtVariant.{other}` is not a variant")),
        })
    }

    fn rt_item(&self, term: &'a Json) -> Result<Item, String> {
        let (short, a) = self.constructor(term, SYNTAX, "RtItem")?;
        let take = |count| Self::arity(a, count, "RtItem", short);
        let types = |this: &Self, at: &'a Json| this.list(at, "types", Self::rt_type);
        Ok(match short {
            "useItem" => {
                let a = take(2)?;
                Item::Use(
                    self.path(&a[0])?,
                    self.optional(&a[1], "an alias", |this, at| this.string(at, "an alias"))?,
                )
            }
            "unitStruct" => {
                let a = take(2)?;
                Item::UnitStruct(self.string(&a[0], "a struct")?, self.names(&a[1])?)
            }
            "alias" => {
                let a = take(3)?;
                Item::Alias(
                    self.string(&a[0], "an alias")?,
                    self.names(&a[1])?,
                    self.rt_type(&a[2])?,
                )
            }
            "staticItem" => {
                let a = take(3)?;
                Item::Static(
                    self.string(&a[0], "a static")?,
                    self.rt_type(&a[1])?,
                    self.rt_expr(&a[2])?,
                )
            }
            "traitItem" => {
                let a = take(2)?;
                Item::Trait(
                    self.string(&a[0], "a trait")?,
                    self.list(&a[1], "signatures", Self::rt_sig)?,
                )
            }
            "implItem" => {
                let a = take(4)?;
                Item::Impl(
                    self.list(&a[0], "generics", Self::rt_generic)?,
                    self.optional(&a[1], "a trait", |this, at| this.string(at, "a trait"))?,
                    self.rt_type(&a[2])?,
                    self.list(&a[3], "functions", Self::rt_fn)?,
                )
            }
            "fnItem" => Item::Fn(self.rt_fn(&take(1)?[0])?),
            "tupleStruct" => {
                let a = take(4)?;
                Item::TupleStruct(
                    self.string(&a[0], "a struct")?,
                    self.names(&a[1])?,
                    self.names(&a[2])?,
                    types(self, &a[3])?,
                )
            }
            "recordStruct" => {
                let a = take(5)?;
                Item::RecordStruct(
                    self.string(&a[0], "a struct")?,
                    self.names(&a[1])?,
                    self.names(&a[2])?,
                    self.names(&a[3])?,
                    types(self, &a[4])?,
                )
            }
            "enumItem" => {
                let a = take(4)?;
                Item::Enum(
                    self.string(&a[0], "an enum")?,
                    self.names(&a[1])?,
                    self.names(&a[2])?,
                    self.list(&a[3], "variants", Self::rt_variant)?,
                )
            }
            other => return fail(format!("`RtItem.{other}` is not an item")),
        })
    }

    fn rt_profile(&self, term: &'a Json) -> Result<Profile, String> {
        let (short, a) = self.constructor(term, SYNTAX, "RtProfile")?;
        Self::arity(a, 0, "RtProfile", short)?;
        match short {
            "core" => Ok(Profile::Core),
            "std" => Ok(Profile::Std),
            other => fail(format!("`RtProfile.{other}` is not a profile")),
        }
    }

    fn rt_crate(&self, term: &'a Json) -> Result<Crate, String> {
        let [profile, items] = self.record(term, "RtCrate", ["profile", "items"])?;
        Ok(Crate {
            profile: self.rt_profile(profile)?,
            items: self.list(items, "the items of a crate", Self::rt_item)?,
        })
    }
}

/// Decode `supportCrate` of a `RuntimeSupport` module.
///
/// # Errors
///
/// The first rule the module breaks: it is not a semantic module, it does not
/// define `supportCrate`, or a term is not a constructor of `RuntimeSyntax`
/// with its arity.
pub fn decode(source: &str) -> Result<Crate, String> {
    decode_named(source, "supportCrate")
}

/// Decode the crate a `RuntimeSyntax` module defines as `name`: the support
/// crate of `RuntimeSupport`, or the probe of `RuntimeProbe`.
///
/// # Errors
///
/// The first rule the module breaks, as for [`decode`].
pub fn decode_named(source: &str, name: &str) -> Result<Crate, String> {
    decode_at(MODULE_PATH, source, name)
}

/// Decode `stdSupportCrate` of a `RuntimeStd` module.
///
/// # Errors
///
/// The first rule the module breaks, as for [`decode`].
pub fn decode_std(source: &str) -> Result<Crate, String> {
    decode_at(STD_PATH, source, "stdSupportCrate")
}

fn decode_at(path: &str, source: &str, name: &str) -> Result<Crate, String> {
    with_reader(path, source, |reader| {
        let Some(support) = reader.definitions.get(name) else {
            return fail(format!("does not define `{name}`"));
        };
        reader.rt_crate(support)
    })
}

/// The text of the embedded module.
///
/// # Errors
///
/// The module is not in the embedded tree, or is not UTF-8.
pub fn embedded() -> Result<&'static str, String> {
    embedded_file(MODULE_PATH)
}

/// The text of the embedded `RuntimeStd` module.
///
/// # Errors
///
/// The module is not in the embedded tree, or is not UTF-8.
pub fn embedded_std() -> Result<&'static str, String> {
    embedded_file(STD_PATH)
}
