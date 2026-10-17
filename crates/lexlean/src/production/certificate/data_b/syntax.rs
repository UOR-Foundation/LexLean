//! A rendered crate as kernel terms of `LexLeanTarget.RustSyntax` (SPEC.md
//! §17.17): the closed Rust AST the printer writes as `lib.rs`, construct for
//! construct, without the origins, which carry no meaning. Certificate B
//! states its theorem about this term; that the printer writes the same AST
//! as text is the printer's own obligation (§17.16).
//!
//! Every constructor is applied to the arguments it has, as the elaborator
//! writes them: a numeral is `OfNat.ofNat`, a list is `List.cons` over its
//! element type, an optional type annotation is `Option.some`/`Option.none`.

use super::super::data_a::Writer;
use crate::calculus::rust::ast::{
    Block, Callee, CaptureRead, Crate, Ctor, Dispatch, Expr, Ident, ItemDef, Let, Lit, Pat, Type,
};
use crate::calculus::rust::runtime::Item;
use crate::calculus::rust::Profile;
use crate::calculus::{IntKind, OrderingValue};
use crate::production::kernel::Term;

const RS: &str = "LexLeanTarget.RustSyntax";
const TS: &str = "LexLeanTarget.TargetSyntax";

impl Writer<'_, '_> {
    /// A constant of `LexLeanTarget.RustSyntax`.
    pub(super) fn rs(&mut self, name: &str) -> Term {
        self.b.konst(&format!("{RS}.{name}"))
    }

    fn rs_apply(&mut self, name: &str, args: &[Term]) -> Term {
        let head = self.rs(name);
        self.b.apps(head, args)
    }

    /// `[…] : List <element>` for a `RustSyntax` type.
    fn rs_list(&mut self, element: &str, items: &[Term]) -> Term {
        let element = self.rs(element);
        self.b.list(element, items)
    }

    fn rs_option(&mut self, element: &str, held: Option<Term>) -> Term {
        let element = self.rs(element);
        match held {
            Some(value) => {
                let some = self.b.konst0("Option.some");
                self.b.apps(some, &[element, value])
            }
            None => {
                let none = self.b.konst0("Option.none");
                self.b.app(none, element)
            }
        }
    }

    fn order_term(&mut self, value: OrderingValue) -> Term {
        let name = match value {
            OrderingValue::Lt => "less",
            OrderingValue::Eq => "same",
            OrderingValue::Gt => "more",
        };
        self.b.konst(&format!("{TS}.Order.{name}"))
    }

    fn kind_of(&mut self, width: IntKind) -> Term {
        self.int_kind(width)
    }

    /// An identifier as a `RustSyntax.Ident` term.
    pub(super) fn rs_ident(&mut self, name: &Ident) -> Term {
        let generated = |this: &mut Self, kind: &str, index: u64| {
            let kind = this.rs(&format!("IdentKind.{kind}"));
            let index = this.nat_u(index);
            this.rs_apply("Ident.generated", &[kind, index])
        };
        match name {
            Ident::Local(n) => generated(self, "binding", *n),
            Ident::Holder(n) => generated(self, "holder", *n),
            Ident::Operand(n) => generated(self, "operand", *n),
            Ident::Callee(n) => generated(self, "callee", *n),
            Ident::Boxed(n) => generated(self, "boxed", *n),
            Ident::Part(n) => generated(self, "part", *n),
            Ident::Capture(n) => generated(self, "capture", *n),
            Ident::Param(n) => generated(self, "param", *n),
            Ident::Function(n) => generated(self, "function", *n),
            Ident::Export(text) => {
                let text = self.b.string(text);
                self.rs_apply("Ident.exported", &[text])
            }
        }
    }

    /// The `IdentKind` constructor and index of a generated identifier, as
    /// the terms a rule names them by.
    pub(super) fn rs_ident_parts(&mut self, name: &Ident) -> Option<(Term, Term)> {
        let (kind, n) = match name {
            Ident::Local(n) => ("binding", *n),
            Ident::Holder(n) => ("holder", *n),
            Ident::Operand(n) => ("operand", *n),
            Ident::Callee(n) => ("callee", *n),
            Ident::Boxed(n) => ("boxed", *n),
            Ident::Part(n) => ("part", *n),
            Ident::Capture(n) => ("capture", *n),
            Ident::Param(n) => ("param", *n),
            Ident::Function(n) => ("function", *n),
            Ident::Export(_) => return None,
        };
        let kind = self.rs(&format!("IdentKind.{kind}"));
        let index = self.nat_u(n);
        Some((kind, index))
    }

    /// A written type as a `RustSyntax.RustType` term.
    pub(super) fn rs_type(&mut self, written: &Type) -> Term {
        match written {
            Type::Unit => self.rs("RustType.unit"),
            Type::Bool => self.rs("RustType.bool"),
            Type::Nat => self.rs("RustType.nat"),
            Type::Int => self.rs("RustType.int"),
            Type::Fixed(width) => {
                let kind = self.kind_of(*width);
                self.rs_apply("RustType.fixed", &[kind])
            }
            Type::Ordering => self.rs("RustType.ordering"),
            Type::Str => self.rs("RustType.str"),
            Type::Bytes => self.rs("RustType.bytes"),
            Type::Option(held) => {
                let held = self.rs_type(held);
                self.rs_apply("RustType.option", &[held])
            }
            Type::Result(ok, error) => {
                let ok = self.rs_type(ok);
                let error = self.rs_type(error);
                self.rs_apply("RustType.result", &[ok, error])
            }
            Type::List(element) => {
                let element = self.rs_type(element);
                self.rs_apply("RustType.list", &[element])
            }
            Type::Pair(left, right) => {
                let left = self.rs_type(left);
                let right = self.rs_type(right);
                self.rs_apply("RustType.pair", &[left, right])
            }
            Type::Adt(n) => {
                let n = self.nat_u(*n);
                self.rs_apply("RustType.adt", &[n])
            }
            Type::Fn(n) => {
                let n = self.nat_u(*n);
                self.rs_apply("RustType.fn", &[n])
            }
            Type::Rc(held) => {
                let held = self.rs_type(held);
                self.rs_apply("RustType.rc", &[held])
            }
            Type::Fallible(held) => {
                let held = self.rs_type(held);
                self.rs_apply("RustType.fallible", &[held])
            }
            Type::Ref(held) => {
                let held = self.rs_type(held);
                self.rs_apply("RustType.ref", &[held])
            }
        }
    }

    /// `Option RustType`: the annotation of a let.
    pub(super) fn rs_annotation(&mut self, written: Option<&Type>) -> Term {
        let held = written.map(|declared| self.rs_type(declared));
        self.rs_option("RustType", held)
    }

    fn rs_lit(&mut self, literal: &Lit) -> Term {
        match literal {
            Lit::Unit => self.rs("Lit.unit"),
            Lit::Bool(flag) => {
                let flag = self.boolean(*flag);
                self.rs_apply("Lit.bool", &[flag])
            }
            Lit::Nat(number) => {
                let number = self.nat_u(*number);
                self.rs_apply("Lit.nat", &[number])
            }
            Lit::Int(number) => {
                let number = self.int_literal("Int", &number.to_string());
                self.rs_apply("Lit.int", &[number])
            }
            Lit::Fixed(width, number) => {
                let kind = self.kind_of(*width);
                let number = self.int_literal("Int", &number.to_string());
                self.rs_apply("Lit.fixed", &[kind, number])
            }
            Lit::Str(text) => {
                let text = self.b.string(text);
                self.rs_apply("Lit.str", &[text])
            }
            Lit::Bytes(octets) => {
                let items: Vec<Term> = octets
                    .iter()
                    .map(|octet| self.int_literal("UInt8", &octet.to_string()))
                    .collect();
                let byte = self.b.konst("UInt8");
                let list = self.b.list(byte, &items);
                let to_array = self.b.konst0("List.toArray");
                let array = self.b.apps(to_array, &[byte, list]);
                let mk = self.b.konst("ByteArray.mk");
                let bytes = self.b.app(mk, array);
                self.rs_apply("Lit.bytes", &[bytes])
            }
            Lit::Ordering(value) => {
                let value = self.order_term(*value);
                self.rs_apply("Lit.ordering", &[value])
            }
        }
    }

    fn rs_ctor(&mut self, built: &Ctor) -> Term {
        match built {
            Ctor::None(held) => {
                let held = self.rs_type(held);
                self.rs_apply("Ctor.none", &[held])
            }
            Ctor::Some => self.rs("Ctor.some"),
            Ctor::Ok(ok, error) => {
                let ok = self.rs_type(ok);
                let error = self.rs_type(error);
                self.rs_apply("Ctor.ok", &[ok, error])
            }
            Ctor::Err(ok, error) => {
                let ok = self.rs_type(ok);
                let error = self.rs_type(error);
                self.rs_apply("Ctor.err", &[ok, error])
            }
            Ctor::Adt { adt, constructor } => {
                let adt = self.nat_u(*adt);
                let constructor = self.nat_u(*constructor);
                self.rs_apply("Ctor.adt", &[adt, constructor])
            }
            Ctor::Closure { fn_type, function } => {
                let fn_type = self.nat_u(*fn_type);
                let function = self.nat_u(*function);
                self.rs_apply("Ctor.closure", &[fn_type, function])
            }
            Ctor::Cons => self.rs("Ctor.cons"),
            Ctor::Nil(element) => {
                let element = self.rs_type(element);
                self.rs_apply("Ctor.nil", &[element])
            }
        }
    }

    /// A pattern as a `RustSyntax.Pat` term.
    pub(super) fn rs_pat(&mut self, pattern: &Pat) -> Term {
        match pattern {
            Pat::Wild => self.rs("Pat.wild"),
            Pat::Bind(name) => {
                let name = self.rs_ident(name);
                self.rs_apply("Pat.bind", &[name])
            }
            Pat::Unit => self.rs("Pat.unit"),
            Pat::Tuple(parts) => {
                let parts = self.rs_pats(parts);
                self.rs_apply("Pat.tuple", &[parts])
            }
            Pat::None => self.rs("Pat.none"),
            Pat::Some(inner) => {
                let inner = self.rs_pat(inner);
                self.rs_apply("Pat.some", &[inner])
            }
            Pat::Ok(inner) => {
                let inner = self.rs_pat(inner);
                self.rs_apply("Pat.ok", &[inner])
            }
            Pat::Err(inner) => {
                let inner = self.rs_pat(inner);
                self.rs_apply("Pat.err", &[inner])
            }
            Pat::Ordering(value) => {
                let value = self.order_term(*value);
                self.rs_apply("Pat.ordering", &[value])
            }
            Pat::Adt {
                adt,
                constructor,
                fields,
            } => {
                let adt = self.nat_u(*adt);
                let constructor = self.nat_u(*constructor);
                let fields = self.rs_pats(fields);
                self.rs_apply("Pat.adt", &[adt, constructor, fields])
            }
        }
    }

    /// `List Pat`.
    pub(super) fn rs_pats(&mut self, patterns: &[Pat]) -> Term {
        let items: Vec<Term> = patterns
            .iter()
            .map(|pattern| self.rs_pat(pattern))
            .collect();
        self.rs_list("Pat", &items)
    }

    /// A runtime item as a `RustSyntax.Item` term.
    pub(super) fn rs_item(&mut self, runtime: Item) -> Term {
        let plain = |this: &mut Self, name: &str| this.rs(&format!("Item.{name}"));
        let wide = |this: &mut Self, name: &str, width: IntKind| {
            let kind = this.kind_of(width);
            this.rs_apply(&format!("Item.{name}"), &[kind])
        };
        match runtime {
            Item::NatAdd => plain(self, "natAdd"),
            Item::NatSub => plain(self, "natSub"),
            Item::NatMul => plain(self, "natMul"),
            Item::NatQuot => plain(self, "natQuot"),
            Item::NatRem => plain(self, "natRem"),
            Item::NatEq => plain(self, "natEq"),
            Item::NatLe => plain(self, "natLe"),
            Item::NatLt => plain(self, "natLt"),
            Item::NatSucc => plain(self, "natSucc"),
            Item::IntAdd => plain(self, "intAdd"),
            Item::IntSub => plain(self, "intSub"),
            Item::IntMul => plain(self, "intMul"),
            Item::IntNeg => plain(self, "intNeg"),
            Item::IntQuot => plain(self, "intQuot"),
            Item::IntRem => plain(self, "intRem"),
            Item::BoolNot => plain(self, "boolNot"),
            Item::BoolAnd => plain(self, "boolAnd"),
            Item::BoolOr => plain(self, "boolOr"),
            Item::Equal => plain(self, "equal"),
            Item::Compare => plain(self, "compare"),
            Item::CheckedAdd(width) => wide(self, "checkedAdd", width),
            Item::CheckedSub(width) => wide(self, "checkedSub", width),
            Item::CheckedMul(width) => wide(self, "checkedMul", width),
            Item::CheckedQuot(width) => wide(self, "checkedQuot", width),
            Item::CheckedNeg(width) => wide(self, "checkedNeg", width),
            Item::BitAnd(width) => wide(self, "bitAnd", width),
            Item::BitOr(width) => wide(self, "bitOr", width),
            Item::BitXor(width) => wide(self, "bitXor", width),
            Item::BitNot(width) => wide(self, "bitNot", width),
            Item::ShiftLeft(width) => wide(self, "shiftLeft", width),
            Item::ShiftRight(width) => wide(self, "shiftRight", width),
            Item::Convert(width) => wide(self, "convert", width),
            Item::AppendList => plain(self, "appendList"),
            Item::AppendBytes => plain(self, "appendBytes"),
            Item::LengthList => plain(self, "lengthList"),
            Item::LengthBytes => plain(self, "lengthBytes"),
            Item::LengthString => plain(self, "lengthString"),
            Item::IndexList => plain(self, "indexList"),
            Item::IndexBytes => plain(self, "indexBytes"),
            Item::SliceList => plain(self, "sliceList"),
            Item::SliceBytes => plain(self, "sliceBytes"),
            Item::Utf8Encode => plain(self, "utf8Encode"),
            Item::Utf8Decode => plain(self, "utf8Decode"),
            Item::CompareBytes => plain(self, "compareBytes"),
            Item::SplitExact => plain(self, "splitExact"),
            Item::Join => plain(self, "join"),
            Item::FormatInt => plain(self, "formatInt"),
            Item::FormatFixed(width) => wide(self, "formatFixed", width),
            Item::ParseInt => plain(self, "parseInt"),
            Item::ParseFixed(width) => wide(self, "parseFixed", width),
        }
    }

    /// `List Expr`.
    pub(super) fn rs_exprs(&mut self, all: &[Expr]) -> Term {
        let items: Vec<Term> = all.iter().map(|one| self.rs_expr(one)).collect();
        self.rs_list("Expr", &items)
    }

    /// One expression as a `RustSyntax.Expr` term.
    pub(super) fn rs_expr(&mut self, written: &Expr) -> Term {
        let read = |this: &mut Self, name: &str, held: &Ident| {
            let held = this.rs_ident(held);
            this.rs_apply(&format!("Expr.{name}"), &[held])
        };
        match written {
            Expr::Lit(literal, _) => {
                let literal = self.rs_lit(literal);
                self.rs_apply("Expr.lit", &[literal])
            }
            Expr::Move(held, _) => read(self, "move", held),
            Expr::Clone(held, _) => read(self, "clone", held),
            Expr::Copy(held, _) => read(self, "copy", held),
            Expr::Deref(held, _) => read(self, "deref", held),
            Expr::Not(inner, _) => {
                let inner = self.rs_expr(inner);
                self.rs_apply("Expr.not", &[inner])
            }
            Expr::Unbox(held, _) => read(self, "unbox", held),
            Expr::Box(inner, _) => {
                let inner = self.rs_expr(inner);
                self.rs_apply("Expr.box", &[inner])
            }
            Expr::Call {
                callee,
                args,
                propagate,
                at: _,
            } => {
                let callee = self.rs_callee(callee);
                let args = self.rs_exprs(args);
                let propagate = self.boolean(*propagate);
                self.rs_apply("Expr.call", &[callee, args, propagate])
            }
            Expr::Apply {
                holder,
                args,
                propagate,
                at: _,
            } => {
                let holder = self.rs_ident(holder);
                let args = self.rs_exprs(args);
                let propagate = self.boolean(*propagate);
                self.rs_apply("Expr.apply", &[holder, args, propagate])
            }
            Expr::Construct {
                ctor: built,
                args,
                at: _,
            } => {
                let built = self.rs_ctor(built);
                let args = self.rs_exprs(args);
                self.rs_apply("Expr.construct", &[built, args])
            }
            Expr::Pair(left, right, _) => {
                let left = self.rs_expr(left);
                let right = self.rs_expr(right);
                self.rs_apply("Expr.pair", &[left, right])
            }
            Expr::If {
                condition,
                then_branch,
                else_branch,
                at: _,
            } => {
                let condition = self.rs_expr(condition);
                let then_branch = self.rs_block(then_branch);
                let else_branch = self.rs_block(else_branch);
                self.rs_apply("Expr.cond", &[condition, then_branch, else_branch])
            }
            Expr::Match {
                scrutinee,
                arms,
                at: _,
            } => {
                let scrutinee = self.rs_expr(scrutinee);
                let arms = self.rs_arms(arms);
                self.rs_apply("Expr.matchOn", &[scrutinee, arms])
            }
            Expr::Block(inner) => {
                let inner = self.rs_block(inner);
                self.rs_apply("Expr.block", &[inner])
            }
            Expr::Uncons(held) => read(self, "uncons", held),
            Expr::IsZero(held, _) => read(self, "isZero", held),
            Expr::NonZero(held, _) => read(self, "nonZero", held),
            Expr::Predecessor(held) => read(self, "predecessor", held),
            Expr::Widen(inner, _) => {
                let inner = self.rs_expr(inner);
                self.rs_apply("Expr.widen", &[inner])
            }
            Expr::Succeed(inner, _) => {
                let inner = self.rs_expr(inner);
                self.rs_apply("Expr.succeed", &[inner])
            }
        }
    }

    fn rs_callee(&mut self, callee: &Callee) -> Term {
        match callee {
            Callee::Function(n) => {
                let n = self.nat_u(*n);
                self.rs_apply("Callee.function", &[n])
            }
            Callee::Runtime(runtime) => {
                let item = self.rs_item(*runtime);
                self.rs_apply("Callee.runtime", &[item])
            }
        }
    }

    /// One arm of a Rust match as a `RustSyntax.Arm` term.
    pub(super) fn rs_arm(&mut self, pattern: &Pat, body: &Block) -> Term {
        let pattern = self.rs_pat(pattern);
        let body = self.rs_block(body);
        self.rs_apply("Arm.mk", &[pattern, body])
    }

    /// `List Arm`.
    pub(super) fn rs_arms(&mut self, arms: &[(Pat, Block)]) -> Term {
        let items: Vec<Term> = arms
            .iter()
            .map(|(pattern, body)| self.rs_arm(pattern, body))
            .collect();
        self.rs_list("Arm", &items)
    }

    /// One let as a `RustSyntax.Let` term.
    pub(super) fn rs_let(&mut self, written: &Let) -> Term {
        let pattern = self.rs_pat(&written.pat);
        let annotation = self.rs_annotation(written.ty.as_ref());
        let value = self.rs_expr(&written.value);
        self.rs_apply("Let.mk", &[pattern, annotation, value])
    }

    /// `List Let`.
    pub(super) fn rs_lets(&mut self, lets: &[Let]) -> Term {
        let items: Vec<Term> = lets.iter().map(|one| self.rs_let(one)).collect();
        self.rs_list("Let", &items)
    }

    /// One block as a `RustSyntax.Block` term.
    pub(super) fn rs_block(&mut self, written: &Block) -> Term {
        let lets = self.rs_lets(&written.lets);
        let tail = self.rs_expr(&written.tail);
        self.rs_apply("Block.mk", &[lets, tail])
    }

    fn rs_capture(&mut self, read: CaptureRead) -> Term {
        let name = match read {
            CaptureRead::Copy => "copy",
            CaptureRead::Clone => "clone",
            CaptureRead::Unbox => "unbox",
            CaptureRead::Unit => "unit",
        };
        self.rs(&format!("CaptureRead.{name}"))
    }

    fn rs_dispatch(&mut self, arm: &Dispatch) -> Term {
        let function = self.nat_u(arm.function);
        let captures: Vec<Term> = arm
            .captures
            .iter()
            .map(|read| self.rs_capture(*read))
            .collect();
        let captures = self.rs_list("CaptureRead", &captures);
        let fallible = self.boolean(arm.function_fallible);
        self.rs_apply("Dispatch.mk", &[function, captures, fallible])
    }

    /// `List RustType`.
    pub(super) fn rs_types(&mut self, types: &[Type]) -> Term {
        let items: Vec<Term> = types
            .iter()
            .map(|declared| self.rs_type(declared))
            .collect();
        self.rs_list("RustType", &items)
    }

    /// One parameter of a function item as a `RustSyntax.Parameter` term.
    pub(super) fn rs_parameter(&mut self, pattern: &Pat, declared: &Type) -> Term {
        let pattern = self.rs_pat(pattern);
        let declared = self.rs_type(declared);
        self.rs_apply("Parameter.mk", &[pattern, declared])
    }

    /// `List Parameter`.
    pub(super) fn rs_parameters(&mut self, parameters: &[(Pat, Type)]) -> Term {
        let items: Vec<Term> = parameters
            .iter()
            .map(|(pattern, declared)| self.rs_parameter(pattern, declared))
            .collect();
        self.rs_list("Parameter", &items)
    }

    fn rs_item_def(&mut self, definition: &ItemDef) -> Term {
        match definition {
            ItemDef::Enum {
                name,
                variants,
                at: _,
            } => {
                let name = self.rs_type(name);
                let items: Vec<Term> = variants
                    .iter()
                    .map(|(number, fields)| {
                        let number = self.nat_u(*number);
                        let fields = self.rs_types(fields);
                        self.rs_apply("Variant.mk", &[number, fields])
                    })
                    .collect();
                let variants = self.rs_list("Variant", &items);
                self.rs_apply("ItemDef.enum", &[name, variants])
            }
            ItemDef::Apply {
                fn_type,
                parameters,
                result,
                fallible,
                arms,
                at: _,
            } => {
                let fn_type = self.nat_u(*fn_type);
                let parameters = self.rs_types(parameters);
                let result = self.rs_type(result);
                let fallible = self.boolean(*fallible);
                let items: Vec<Term> = arms.iter().map(|arm| self.rs_dispatch(arm)).collect();
                let arms = self.rs_list("Dispatch", &items);
                self.rs_apply(
                    "ItemDef.apply",
                    &[fn_type, parameters, result, fallible, arms],
                )
            }
            ItemDef::Function {
                name,
                parameters,
                result,
                body,
                at: _,
            } => {
                let name = self.rs_ident(name);
                let parameters = self.rs_parameters(parameters);
                let result = self.rs_type(result);
                let body = self.rs_block(body);
                self.rs_apply("ItemDef.function", &[name, parameters, result, body])
            }
        }
    }

    /// A crate as a `RustSyntax.Crate` term.
    pub(super) fn rs_crate(&mut self, rendered: &Crate) -> Term {
        let profile = self.rs(match rendered.profile {
            Profile::Core => "Profile.core",
            Profile::Std => "Profile.std",
        });
        let items: Vec<Term> = rendered
            .items
            .iter()
            .map(|item| self.rs_item_def(item))
            .collect();
        let items = self.rs_list("ItemDef", &items);
        self.rs_apply("Crate.mk", &[profile, items])
    }
}
