//! The reader of `RuntimeSyntax` data: the `supportCrate` of `RuntimeSupport`.
//!
//! The same discipline as `source`: the module is decoded and never
//! evaluated; the reader accepts exactly the constructors of `RuntimeSyntax`,
//! each with its own arity, and refuses every other shape by name.

use super::rt::{Block, Crate, Expr, Function, Generic, Item, Lit, Param, Pat, Profile, Sig, Type};
use super::source::{embedded_file, fail, with_reader, Reader};
use crate::artifact::canonical_json::Json;

/// Where the module is in the embedded tree.
pub const MODULE_PATH: &str = "language/rust-runtime-1.2/RuntimeSupport.lex.tex";

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
            other => return fail(format!("`RtLit.{other}` is not a literal")),
        })
    }

    fn path(&self, term: &'a Json) -> Result<Vec<String>, String> {
        self.list(term, "a path", |this, at| this.string(at, "a path segment"))
    }

    fn rt_pat(&self, term: &'a Json) -> Result<Pat, String> {
        let (short, a) = self.constructor(term, SYNTAX, "RtPat")?;
        Ok(match short {
            "wild" => {
                Self::arity(a, 0, "RtPat", short)?;
                Pat::Wild
            }
            "bind" => Pat::Bind(self.string(&Self::arity(a, 1, "RtPat", short)?[0], "a binding")?),
            "path" => Pat::Path(self.path(&Self::arity(a, 1, "RtPat", short)?[0])?),
            other => return fail(format!("`RtPat.{other}` is not a pattern")),
        })
    }

    fn rt_expr(&self, term: &'a Json) -> Result<Expr, String> {
        let (short, a) = self.constructor(term, SYNTAX, "RtExpr")?;
        let take = |count| Self::arity(a, count, "RtExpr", short);
        let boxed = |this: &Self, at: &'a Json| this.rt_expr(at).map(Box::new);
        let exprs = |this: &Self, at: &'a Json| this.list(at, "arguments", Self::rt_expr);
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
            "borrow" => Expr::Borrow(boxed(self, &take(1)?[0])?),
            "equals" => {
                let a = take(2)?;
                Expr::Equals(boxed(self, &a[0])?, boxed(self, &a[1])?)
            }
            "matchOn" => {
                let a = take(2)?;
                Expr::Match(
                    boxed(self, &a[0])?,
                    self.list(&a[1], "arms", |this, at| {
                        let (short, a) = this.constructor(at, SYNTAX, "RtArm")?;
                        let a = Self::arity(a, 2, "RtArm", short)?;
                        Ok((this.rt_pat(&a[0])?, this.rt_expr(&a[1])?))
                    })?,
                )
            }
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
            bounds: self.list(bounds, "bounds", |this, at| this.string(at, "a bound"))?,
        })
    }

    fn rt_sig(&self, term: &'a Json) -> Result<Sig, String> {
        let [name, receiver, params, result] =
            self.record(term, "RtSig", ["name", "receiver", "parameters", "result"])?;
        Ok(Sig {
            name: self.string(name, "a function name")?,
            receiver: self.boolean(receiver, "a receiver")?,
            params: self.list(params, "parameters", |this, at| {
                let [name, ty] = this.record(at, "RtParam", ["name", "type"])?;
                Ok(Param {
                    name: this.string(name, "a parameter")?,
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

    fn rt_item(&self, term: &'a Json) -> Result<Item, String> {
        let (short, a) = self.constructor(term, SYNTAX, "RtItem")?;
        let take = |count| Self::arity(a, count, "RtItem", short);
        let strings =
            |this: &Self, at: &'a Json| this.list(at, "names", |t, x| t.string(x, "a name"));
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
                Item::UnitStruct(self.string(&a[0], "a struct")?, strings(self, &a[1])?)
            }
            "alias" => {
                let a = take(3)?;
                Item::Alias(
                    self.string(&a[0], "an alias")?,
                    strings(self, &a[1])?,
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
    with_reader(MODULE_PATH, source, |reader| {
        let Some(support) = reader.definitions.get("supportCrate") else {
            return fail("does not define `supportCrate`");
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
