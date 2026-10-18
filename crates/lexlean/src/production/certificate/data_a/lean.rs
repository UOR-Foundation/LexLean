//! The Lean terms certificate A states, as kernel terms: types, encoders, the
//! target program's literals, and the source's denotation (`src`), each the
//! term the pinned Lean elaborates the text of §17.17 to.

use super::Writer;
use crate::calculus::{
    Arm, Expr, Function, IntKind, OrderingValue, Prim, Program, Shape, Ty, Value,
};
use crate::ir::semantic::{MemberRef, SemanticType};
use crate::production::kernel::Term;

impl Writer<'_, '_> {
    // --- Numbers and Booleans ---------------------------------------------

    /// `n : Nat` as the elaborator writes a numeral.
    pub(in crate::production::certificate) fn nat_lit(&mut self, n: &str) -> Term {
        let raw = self.b.nat_text(n);
        let nat = self.b.konst("Nat");
        let of_nat = self.b.konst0("OfNat.ofNat");
        let inst = self.b.konst("instOfNatNat");
        let inst = self.b.app(inst, raw);
        self.b.apps(of_nat, &[nat, raw, inst])
    }

    pub(in crate::production::certificate) fn nat_u(&mut self, n: u64) -> Term {
        self.nat_lit(&n.to_string())
    }

    pub(in crate::production::certificate) fn tru(&mut self) -> Term {
        self.b.konst("Bool.true")
    }

    pub(in crate::production::certificate) fn fls(&mut self) -> Term {
        self.b.konst("Bool.false")
    }

    pub(in crate::production::certificate) fn boolean(&mut self, value: bool) -> Term {
        if value {
            self.tru()
        } else {
            self.fls()
        }
    }

    /// `a && b`.
    pub(super) fn and(&mut self, a: Term, b: Term) -> Term {
        let and = self.b.konst("Bool.and");
        self.b.apps(and, &[a, b])
    }

    /// `!a`.
    pub(super) fn not(&mut self, a: Term) -> Term {
        let not = self.b.konst("Bool.not");
        self.b.app(not, a)
    }

    /// `if c then t else e` over a Boolean, at type `ty`.
    pub(super) fn ite(&mut self, ty: Term, c: Term, t: Term, e: Term) -> Term {
        let bool = self.b.konst("Bool");
        let tru = self.tru();
        let eq = self.b.konst1("Eq");
        let prop = self.b.apps(eq, &[bool, c, tru]);
        let decide = self.b.konst("instDecidableEqBool");
        let decide = self.b.apps(decide, &[c, tru]);
        let ite = self.b.konst1("ite");
        self.b.apps(ite, &[ty, prop, decide, t, e])
    }

    /// `a = b` at `ty` (a Prop), where `ty : Type`.
    pub(super) fn eq(&mut self, ty: Term, a: Term, b: Term) -> Term {
        let eq = self.b.konst1("Eq");
        self.b.apps(eq, &[ty, a, b])
    }

    pub(super) fn nat_add(&mut self, a: Term, b: Term) -> Term {
        let nat = self.b.konst("Nat");
        let add = self.b.konst_at(
            "HAdd.hAdd",
            vec![
                crate::ir::core::CoreLevel::Zero,
                crate::ir::core::CoreLevel::Zero,
                crate::ir::core::CoreLevel::Zero,
            ],
        );
        let inst = self.b.konst0("instHAdd");
        let add_nat = self.b.konst("instAddNat");
        let inst = self.b.apps(inst, &[nat, add_nat]);
        self.b.apps(add, &[nat, nat, nat, inst, a, b])
    }

    /// `Nat.blt a b`.
    pub(super) fn nat_blt(&mut self, a: Term, b: Term) -> Term {
        let blt = self.b.konst("Nat.blt");
        self.b.apps(blt, &[a, b])
    }

    // --- Types ------------------------------------------------------------

    /// The Lean type of a closed source type.
    pub(super) fn lean_ty(&mut self, ty: &SemanticType) -> Result<Term, String> {
        Ok(match ty {
            SemanticType::Nat => self.b.konst("Nat"),
            SemanticType::Bool => self.b.konst("Bool"),
            SemanticType::Unit => self.b.konst("Unit"),
            SemanticType::Int => self.b.konst("Int"),
            SemanticType::Int8 => self.b.konst("Int8"),
            SemanticType::Int16 => self.b.konst("Int16"),
            SemanticType::Int32 => self.b.konst("Int32"),
            SemanticType::Int64 => self.b.konst("Int64"),
            SemanticType::UInt8 => self.b.konst("UInt8"),
            SemanticType::UInt16 => self.b.konst("UInt16"),
            SemanticType::UInt32 => self.b.konst("UInt32"),
            SemanticType::UInt64 => self.b.konst("UInt64"),
            SemanticType::String => self.b.konst("String"),
            SemanticType::Bytes => self.b.konst("ByteArray"),
            SemanticType::Ordering => self.b.konst("Ordering"),
            SemanticType::ContractViolation | SemanticType::ReasoningFailure => {
                let bool = self.b.konst("Bool");
                self.b.pair_ty(bool, bool)
            }
            SemanticType::Option { value } => {
                let inner = self.lean_ty(value)?;
                let option = self.b.konst0("Option");
                self.b.app(option, inner)
            }
            SemanticType::Result { ok, error } => {
                let error = self.lean_ty(error)?;
                let ok = self.lean_ty(ok)?;
                let except = self.b.konst00("Except");
                self.b.apps(except, &[error, ok])
            }
            SemanticType::List { element } | SemanticType::Set { element } => {
                let element = self.lean_ty(element)?;
                self.b.list_ty(element)
            }
            SemanticType::Product { left, right } => {
                let left = self.lean_ty(left)?;
                let right = self.lean_ty(right)?;
                self.b.pair_ty(left, right)
            }
            SemanticType::Map { key, value } => {
                let key = self.lean_ty(key)?;
                let value = self.lean_ty(value)?;
                let entry = self.b.pair_ty(key, value);
                self.b.list_ty(entry)
            }
            SemanticType::Function { parameters, result } => {
                let mut out = self.lean_ty(result)?;
                for parameter in parameters.iter().rev() {
                    let domain = self.lean_ty(parameter)?;
                    out = self.b.arrow(domain, out);
                }
                out
            }
            SemanticType::Named { member, arguments } => {
                let head = self.document_name(member)?;
                let head = self.b.konst(&head);
                let mut args = Vec::new();
                for argument in arguments {
                    args.push(self.lean_ty(argument)?);
                }
                self.b.apps(head, &args)
            }
            SemanticType::Type | SemanticType::Prop | SemanticType::Parameter { name: _ } => {
                return Err(format!(
                    "`{}` has no runtime encoding",
                    self.gen.source.type_text(ty)
                ));
            }
        })
    }

    /// The Lean name of a document type.
    pub(super) fn document_name(&self, member: &MemberRef) -> Result<String, String> {
        let module = member.module.clone().ok_or("an unanchored document type")?;
        Ok(format!(
            "{}.{}",
            self.gen.source.lean_module(&module)?,
            member.name
        ))
    }

    /// `Value`.
    pub(super) fn value_ty(&mut self) -> Term {
        self.ts("Value")
    }

    // --- Encoders ---------------------------------------------------------

    /// The encoder of a closed first-order type into target values.
    pub(super) fn enc(&mut self, ty: &SemanticType) -> Result<Term, String> {
        match self.gen.nested.get(&self.gen.source.type_text(ty)) {
            Some(nested) => {
                let (index, kind) = (nested.index, nested.ty.clone());
                self.nested_enc(index, &kind)
            }
            None => self.plain_enc(ty),
        }
    }

    fn value_ctor(&mut self, name: &str) -> Term {
        self.ts(&format!("Value.{name}"))
    }

    fn plain_enc(&mut self, ty: &SemanticType) -> Result<Term, String> {
        Ok(match ty {
            SemanticType::Nat => self.value_ctor("nat"),
            SemanticType::Bool => self.value_ctor("bool"),
            SemanticType::Int => self.value_ctor("int"),
            SemanticType::Int8 => self.value_ctor("i8"),
            SemanticType::Int16 => self.value_ctor("i16"),
            SemanticType::Int32 => self.value_ctor("i32"),
            SemanticType::Int64 => self.value_ctor("i64"),
            SemanticType::UInt8 => self.value_ctor("u8"),
            SemanticType::UInt16 => self.value_ctor("u16"),
            SemanticType::UInt32 => self.value_ctor("u32"),
            SemanticType::UInt64 => self.value_ctor("u64"),
            SemanticType::String => self.value_ctor("string"),
            SemanticType::Bytes => self.value_ctor("bytes"),
            SemanticType::Unit => self.lp("encUnit"),
            SemanticType::Ordering => self.lp("encOrdering"),
            SemanticType::ContractViolation | SemanticType::ReasoningFailure => {
                let bool = self.b.konst("Bool");
                let value = self.value_ctor("bool");
                let pair = self.lp("encPair");
                self.b.apps(pair, &[bool, bool, value, value])
            }
            SemanticType::Option { value } => {
                let alpha = self.lean_ty(value)?;
                let inner = self.enc(value)?;
                let enc = self.lp("encOption");
                self.b.apps(enc, &[alpha, inner])
            }
            SemanticType::Result { ok, error } => {
                let epsilon = self.lean_ty(error)?;
                let alpha = self.lean_ty(ok)?;
                let ee = self.enc(error)?;
                let ea = self.enc(ok)?;
                let enc = self.lp("encExcept");
                self.b.apps(enc, &[epsilon, alpha, ee, ea])
            }
            SemanticType::List { element } | SemanticType::Set { element } => {
                let alpha = self.lean_ty(element)?;
                let inner = self.enc(element)?;
                let enc = self.lp("encList");
                self.b.apps(enc, &[alpha, inner])
            }
            SemanticType::Product { left, right } => {
                let alpha = self.lean_ty(left)?;
                let beta = self.lean_ty(right)?;
                let ea = self.enc(left)?;
                let eb = self.enc(right)?;
                let enc = self.lp("encPair");
                self.b.apps(enc, &[alpha, beta, ea, eb])
            }
            SemanticType::Map { key, value } => {
                let alpha = self.lean_ty(key)?;
                let beta = self.lean_ty(value)?;
                let ea = self.enc(key)?;
                let eb = self.enc(value)?;
                let pair = self.lp("encPair");
                let pair = self.b.apps(pair, &[alpha, beta, ea, eb]);
                let entry = self.b.pair_ty(alpha, beta);
                let enc = self.lp("encList");
                self.b.apps(enc, &[entry, pair])
            }
            SemanticType::Named {
                member: _,
                arguments: _,
            } => {
                let text = self.gen.source.type_text(ty);
                let index = self
                    .gen
                    .adts
                    .get(&text)
                    .ok_or_else(|| format!("`{text}` has no ADT in the layout"))?;
                let name = format!("__enc_{index}");
                self.own(&name)
            }
            SemanticType::Function {
                parameters: _,
                result: _,
            }
            | SemanticType::Type
            | SemanticType::Prop
            | SemanticType::Parameter { name: _ } => {
                return Err(format!(
                    "`{}` has no value encoding",
                    self.gen.source.type_text(ty)
                ));
            }
        })
    }

    /// The encoder of a type nested in a recursive group: its bundle's `enc`.
    fn nested_enc(&mut self, index: u64, ty: &SemanticType) -> Result<Term, String> {
        match ty {
            SemanticType::List { element: _ }
            | SemanticType::Set { element: _ }
            | SemanticType::Map { key: _, value: _ }
            | SemanticType::ContractViolation
            | SemanticType::ReasoningFailure => {
                let bundle = self.own(&format!("__L_{index}"));
                let enc = self.lp("ListEnc.enc");
                let alpha = self.nested_element(ty)?;
                Ok(self.b.apps(enc, &[alpha, bundle]))
            }
            SemanticType::Option { value } => {
                let bundle = self.own(&format!("__O_{index}"));
                let enc = self.lp("OptEnc.enc");
                let alpha = self.lean_ty(value)?;
                Ok(self.b.apps(enc, &[alpha, bundle]))
            }
            SemanticType::Result { ok: _, error: _ }
            | SemanticType::Product { left: _, right: _ }
            | SemanticType::Named {
                member: _,
                arguments: _,
            }
            | SemanticType::Function {
                parameters: _,
                result: _,
            }
            | SemanticType::Type
            | SemanticType::Prop
            | SemanticType::Parameter { name: _ }
            | SemanticType::Nat
            | SemanticType::Bool
            | SemanticType::Unit
            | SemanticType::Int
            | SemanticType::Int8
            | SemanticType::Int16
            | SemanticType::Int32
            | SemanticType::Int64
            | SemanticType::UInt8
            | SemanticType::UInt16
            | SemanticType::UInt32
            | SemanticType::UInt64
            | SemanticType::String
            | SemanticType::Bytes
            | SemanticType::Ordering => Ok(self.own(&format!("__aux_{index}"))),
        }
    }

    /// The element type a list-like nested container holds.
    pub(super) fn nested_element(&mut self, ty: &SemanticType) -> Result<Term, String> {
        let element = super::super::element_of(ty)?;
        self.lean_ty(&element)
    }

    // --- The target program -----------------------------------------------

    pub(in crate::production::certificate) fn int_kind(&mut self, kind: IntKind) -> Term {
        self.ts(&format!("IntKind.{}", kind.name()))
    }

    pub(in crate::production::certificate) fn ty_term(&mut self, ty: &Ty) -> Term {
        match ty {
            Ty::Unit => self.ts("Ty.unit"),
            Ty::Bool => self.ts("Ty.bool"),
            Ty::Nat => self.ts("Ty.nat"),
            Ty::Int => self.ts("Ty.int"),
            Ty::Fixed { width } => {
                let fixed = self.ts("Ty.fixed");
                let kind = self.int_kind(*width);
                self.b.app(fixed, kind)
            }
            Ty::String => self.ts("Ty.string"),
            Ty::Bytes => self.ts("Ty.bytes"),
            Ty::Ordering => self.ts("Ty.ordering"),
            Ty::Option { value } => {
                let inner = self.ty_term(value);
                let head = self.ts("Ty.option");
                self.b.app(head, inner)
            }
            Ty::Result { ok, error } => {
                let ok = self.ty_term(ok);
                let error = self.ty_term(error);
                let head = self.ts("Ty.result");
                self.b.apps(head, &[ok, error])
            }
            Ty::List { element } => {
                let inner = self.ty_term(element);
                let head = self.ts("Ty.list");
                self.b.app(head, inner)
            }
            Ty::Pair { left, right } => {
                let left = self.ty_term(left);
                let right = self.ty_term(right);
                let head = self.ts("Ty.pair");
                self.b.apps(head, &[left, right])
            }
            Ty::Adt { index } => {
                let index = self.nat_u(*index);
                let head = self.ts("Ty.adt");
                self.b.app(head, index)
            }
            Ty::Fn { parameters, result } => {
                let ty = self.ts("Ty");
                let items: Vec<Term> = parameters.iter().map(|p| self.ty_term(p)).collect();
                let parameters = self.b.list(ty, &items);
                let result = self.ty_term(result);
                let head = self.ts("Ty.fn");
                self.b.apps(head, &[parameters, result])
            }
        }
    }

    /// A numeral of a fixed-width or mathematical integer type.
    pub(in crate::production::certificate) fn int_literal(
        &mut self,
        type_name: &str,
        text: &str,
    ) -> Term {
        let (negative, digits) = match text.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, text),
        };
        let ty = self.b.konst(type_name);
        let raw = self.b.nat_text(digits);
        let of_nat = self.b.konst0("OfNat.ofNat");
        let instance = if type_name == "Int" {
            self.b.konst("instOfNat")
        } else {
            self.b.konst(&format!("{type_name}.instOfNat"))
        };
        let instance = self.b.app(instance, raw);
        let positive = self.b.apps(of_nat, &[ty, raw, instance]);
        if !negative {
            return positive;
        }
        let neg = self.b.konst0("Neg.neg");
        let instance = if type_name == "Int" {
            self.b.konst("Int.instNegInt")
        } else {
            self.b.konst(&format!("{type_name}.instNeg"))
        };
        self.b.apps(neg, &[ty, instance, positive])
    }

    pub(super) fn bytes_term(&mut self, hex: &str) -> Result<Term, String> {
        let digits = hex.as_bytes();
        if !digits.len().is_multiple_of(2) {
            return Err(format!("byte literal `{hex}` has odd length"));
        }
        let mut items = Vec::new();
        for pair in digits.chunks(2) {
            let text = std::str::from_utf8(pair).map_err(|_| "a non-ASCII byte literal")?;
            let value = u8::from_str_radix(text, 16)
                .map_err(|_| format!("byte literal `{hex}` is not hexadecimal"))?;
            items.push(self.int_literal("UInt8", &value.to_string()));
        }
        let byte = self.b.konst("UInt8");
        let list = self.b.list(byte, &items);
        let to_array = self.b.konst0("List.toArray");
        let array = self.b.apps(to_array, &[byte, list]);
        let mk = self.b.konst("ByteArray.mk");
        Ok(self.b.app(mk, array))
    }

    pub(super) fn value_term(&mut self, value: &Value) -> Result<Term, String> {
        Ok(match value {
            Value::Unit => self.ts("Value.unit"),
            Value::Bool { value } => {
                let head = self.ts("Value.bool");
                let flag = self.boolean(*value);
                self.b.app(head, flag)
            }
            Value::Nat { value } => {
                let head = self.ts("Value.nat");
                let number = self.nat_lit(value);
                self.b.app(head, number)
            }
            Value::Int { value } => {
                let head = self.ts("Value.int");
                let number = self.int_literal("Int", value);
                self.b.app(head, number)
            }
            Value::U8 { value } => self.fixed_value("u8", "UInt8", value),
            Value::U16 { value } => self.fixed_value("u16", "UInt16", value),
            Value::U32 { value } => self.fixed_value("u32", "UInt32", value),
            Value::U64 { value } => self.fixed_value("u64", "UInt64", value),
            Value::I8 { value } => self.fixed_value("i8", "Int8", value),
            Value::I16 { value } => self.fixed_value("i16", "Int16", value),
            Value::I32 { value } => self.fixed_value("i32", "Int32", value),
            Value::I64 { value } => self.fixed_value("i64", "Int64", value),
            Value::String { value } => {
                let head = self.ts("Value.string");
                let text = self.b.string(value);
                self.b.app(head, text)
            }
            Value::Bytes { hex } => {
                let head = self.ts("Value.bytes");
                let bytes = self.bytes_term(hex)?;
                self.b.app(head, bytes)
            }
            Value::Ordering { value } => {
                let head = self.ts("Value.ordering");
                let order = self.ts(match value {
                    OrderingValue::Lt => "Order.less",
                    OrderingValue::Eq => "Order.same",
                    OrderingValue::Gt => "Order.more",
                });
                self.b.app(head, order)
            }
            Value::None => self.ts("Value.none"),
            Value::Some { value } => {
                let head = self.ts("Value.some");
                let inner = self.value_term(value)?;
                self.b.app(head, inner)
            }
            Value::Ok { value } => {
                let head = self.ts("Value.ok");
                let inner = self.value_term(value)?;
                self.b.app(head, inner)
            }
            Value::Error { value } => {
                let head = self.ts("Value.error");
                let inner = self.value_term(value)?;
                self.b.app(head, inner)
            }
            Value::List { items } => {
                let head = self.ts("Value.list");
                let ty = self.ts("Value");
                let terms = items
                    .iter()
                    .map(|item| self.value_term(item))
                    .collect::<Result<Vec<_>, _>>()?;
                let list = self.b.list(ty, &terms);
                self.b.app(head, list)
            }
            Value::Pair { left, right } => {
                let head = self.ts("Value.pair");
                let left = self.value_term(left)?;
                let right = self.value_term(right)?;
                self.b.apps(head, &[left, right])
            }
            Value::Adt {
                constructor,
                fields,
            } => {
                let head = self.ts("Value.adt");
                let ty = self.ts("Value");
                let number = self.nat_u(*constructor);
                let terms = fields
                    .iter()
                    .map(|item| self.value_term(item))
                    .collect::<Result<Vec<_>, _>>()?;
                let list = self.b.list(ty, &terms);
                self.b.apps(head, &[number, list])
            }
            Value::Closure { function, captures } => {
                let head = self.ts("Value.closure");
                let ty = self.ts("Value");
                let number = self.nat_u(*function);
                let terms = captures
                    .iter()
                    .map(|item| self.value_term(item))
                    .collect::<Result<Vec<_>, _>>()?;
                let list = self.b.list(ty, &terms);
                self.b.apps(head, &[number, list])
            }
        })
    }

    fn fixed_value(&mut self, constructor: &str, type_name: &str, text: &str) -> Term {
        let head = self.ts(&format!("Value.{constructor}"));
        let number = self.int_literal(type_name, text);
        self.b.app(head, number)
    }

    pub(in crate::production::certificate) fn shape_term(&mut self, shape: Shape) -> Term {
        let name = match shape {
            Shape::None => "none",
            Shape::Some => "some",
            Shape::Ok => "ok",
            Shape::Error => "error",
            Shape::Nil => "nil",
            Shape::Cons => "cons",
            Shape::Zero => "zero",
            Shape::Succ => "succ",
            Shape::Pair => "pair",
            Shape::True => "true",
            Shape::False => "false",
            Shape::Unit => "unit",
            Shape::Lt => "lt",
            Shape::Eq => "eq",
            Shape::Gt => "gt",
            Shape::Adt { constructor } => {
                let head = self.ts("Shape.adt");
                let number = self.nat_u(constructor);
                return self.b.app(head, number);
            }
        };
        self.ts(&format!("Shape.{name}"))
    }

    fn prim_term(&mut self, prim: &Prim) -> Term {
        let name = match prim {
            Prim::NatAdd => "natAdd",
            Prim::NatSub => "natSub",
            Prim::NatMul => "natMul",
            Prim::NatQuot => "natQuot",
            Prim::NatRem => "natRem",
            Prim::NatEq => "natEq",
            Prim::NatLe => "natLe",
            Prim::NatLt => "natLt",
            Prim::IntAdd => "intAdd",
            Prim::IntSub => "intSub",
            Prim::IntMul => "intMul",
            Prim::IntNeg => "intNeg",
            Prim::IntQuot => "intQuot",
            Prim::IntRem => "intRem",
            Prim::CheckedAdd => "checkedAdd",
            Prim::CheckedSub => "checkedSub",
            Prim::CheckedMul => "checkedMul",
            Prim::CheckedNeg => "checkedNeg",
            Prim::CheckedQuot => "checkedQuot",
            Prim::BitAnd => "bitAnd",
            Prim::BitOr => "bitOr",
            Prim::BitXor => "bitXor",
            Prim::BitNot => "bitNot",
            Prim::ShiftLeft => "shiftLeft",
            Prim::ShiftRight => "shiftRight",
            Prim::Equal => "equal",
            Prim::BoolNot => "boolNot",
            Prim::BoolAnd => "boolAnd",
            Prim::BoolOr => "boolOr",
            Prim::Append => "append",
            Prim::Length => "length",
            Prim::Index => "index",
            Prim::Slice => "slice",
            Prim::Utf8Encode => "utf8Encode",
            Prim::Utf8Decode => "utf8Decode",
            Prim::CompareBytes => "compareBytes",
            Prim::SplitExact => "splitExact",
            Prim::Join => "join",
            Prim::FormatDecimal => "formatDecimal",
            Prim::Compare => "compare",
            Prim::Convert { target } => {
                let head = self.ts("Prim.convert");
                let kind = self.int_kind(*target);
                return self.b.app(head, kind);
            }
            Prim::ParseDecimal { target } => {
                let head = self.ts("Prim.parseDecimal");
                let ty = self.ty_term(target);
                return self.b.app(head, ty);
            }
        };
        self.ts(&format!("Prim.{name}"))
    }

    /// A match arm as a `TargetSyntax.Arm` term.
    pub(in crate::production::certificate) fn arm_term(
        &mut self,
        arm: &Arm,
    ) -> Result<Term, String> {
        let Arm {
            shape,
            binders,
            body,
        } = arm;
        let head = self.ts("Arm.arm");
        let shape = self.shape_term(*shape);
        let binders = self.numbers_term(binders);
        let body = self.expr_term(body)?;
        Ok(self.b.apps(head, &[shape, binders, body]))
    }

    /// A list of match arms.
    pub(in crate::production::certificate) fn arms_list(
        &mut self,
        arms: &[Arm],
    ) -> Result<Term, String> {
        let ty = self.ts("Arm");
        let items = arms
            .iter()
            .map(|arm| self.arm_term(arm))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(self.b.list(ty, &items))
    }

    fn exprs_term(&mut self, exprs: &[Expr]) -> Result<Term, String> {
        let ty = self.ts("Expr");
        let items = exprs
            .iter()
            .map(|e| self.expr_term(e))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(self.b.list(ty, &items))
    }

    pub(in crate::production::certificate) fn numbers_term(&mut self, numbers: &[u64]) -> Term {
        let nat = self.b.konst("Nat");
        let items: Vec<Term> = numbers.iter().map(|n| self.nat_u(*n)).collect();
        self.b.list(nat, &items)
    }

    /// A program expression as a `TargetSyntax.Expr` term.
    pub(in crate::production::certificate) fn expr_term(
        &mut self,
        expr: &Expr,
    ) -> Result<Term, String> {
        let key = std::ptr::from_ref(expr);
        if let Some(&done) = self.exprs.get(&key) {
            return Ok(done);
        }
        let out = match expr {
            Expr::Value { ty, value } => {
                let head = self.ts("Expr.value");
                let ty = self.ty_term(ty);
                let value = self.value_term(value)?;
                self.b.apps(head, &[ty, value])
            }
            Expr::Var { name } => {
                let head = self.ts("Expr.var");
                let name = self.nat_u(*name);
                self.b.app(head, name)
            }
            Expr::Let {
                name,
                ty,
                bound,
                body,
            } => {
                let head = self.ts("Expr.let");
                let name = self.nat_u(*name);
                let ty = self.ty_term(ty);
                let bound = self.expr_term(bound)?;
                let body = self.expr_term(body)?;
                self.b.apps(head, &[name, ty, bound, body])
            }
            Expr::Cond {
                condition,
                then_branch,
                else_branch,
            } => {
                let head = self.ts("Expr.cond");
                let c = self.expr_term(condition)?;
                let t = self.expr_term(then_branch)?;
                let e = self.expr_term(else_branch)?;
                self.b.apps(head, &[c, t, e])
            }
            Expr::Match {
                ty,
                scrutinee,
                arms,
            } => {
                let head = self.ts("Expr.match");
                let ty = self.ty_term(ty);
                let scrutinee = self.expr_term(scrutinee)?;
                let arms = self.arms_list(arms)?;
                self.b.apps(head, &[ty, scrutinee, arms])
            }
            Expr::Build {
                shape,
                ty,
                operands,
            } => {
                let head = self.ts("Expr.build");
                let shape = self.shape_term(*shape);
                let ty = self.ty_term(ty);
                let operands = self.exprs_term(operands)?;
                self.b.apps(head, &[shape, ty, operands])
            }
            Expr::Call { function, operands } => {
                let head = self.ts("Expr.call");
                let function = self.nat_u(*function);
                let operands = self.exprs_term(operands)?;
                self.b.apps(head, &[function, operands])
            }
            Expr::Closure { function, captures } => {
                let head = self.ts("Expr.closure");
                let function = self.nat_u(*function);
                let captures = self.exprs_term(captures)?;
                self.b.apps(head, &[function, captures])
            }
            Expr::Apply { target, operands } => {
                let head = self.ts("Expr.apply");
                let target = self.expr_term(target)?;
                let operands = self.exprs_term(operands)?;
                self.b.apps(head, &[target, operands])
            }
            Expr::Prim {
                operation,
                operands,
            } => {
                let head = self.ts("Expr.prim");
                let operation = self.prim_term(operation);
                let operands = self.exprs_term(operands)?;
                self.b.apps(head, &[operation, operands])
            }
            Expr::First { value } => {
                let head = self.ts("Expr.first");
                let value = self.expr_term(value)?;
                self.b.app(head, value)
            }
            Expr::Second { value } => {
                let head = self.ts("Expr.second");
                let value = self.expr_term(value)?;
                self.b.app(head, value)
            }
            Expr::Field { value, index } => {
                let head = self.ts("Expr.field");
                let value = self.expr_term(value)?;
                let index = self.nat_u(*index);
                self.b.apps(head, &[value, index])
            }
        };
        self.exprs.insert(key, out);
        Ok(out)
    }

    /// The function `function` of the program as a `TargetSyntax.Function`.
    pub(in crate::production::certificate) fn function_term(
        &mut self,
        function: &Function,
    ) -> Result<Term, String> {
        let Function {
            parameters,
            types,
            result,
            body,
        } = function;
        let head = self.ts("Function.mk");
        let parameters = self.numbers_term(parameters);
        let ty = self.ts("Ty");
        let items: Vec<Term> = types.iter().map(|t| self.ty_term(t)).collect();
        let types = self.b.list(ty, &items);
        let result = self.ty_term(result);
        let body = self.expr_term(body)?;
        Ok(self.b.apps(head, &[parameters, types, result, body]))
    }

    /// `__prog`'s value: the lowered program as a `TargetSyntax.Program`.
    pub(in crate::production::certificate) fn program_value(&mut self) -> Result<Term, String> {
        let program: &Program = self.gen.program;
        let adt_ty = self.ts("Adt");
        let ty = self.ts("Ty");
        let ty_list = self.b.list_ty(ty);
        let mut adts = Vec::new();
        for adt in &program.adts {
            let mut constructors = Vec::new();
            for fields in &adt.constructors {
                let items: Vec<Term> = fields.iter().map(|f| self.ty_term(f)).collect();
                constructors.push(self.b.list(ty, &items));
            }
            let list = self.b.list(ty_list, &constructors);
            let mk = self.ts("Adt.mk");
            adts.push(self.b.app(mk, list));
        }
        let function_ty = self.ts("Function");
        let mut functions = Vec::new();
        for function in &program.functions {
            functions.push(self.function_term(function)?);
        }
        let adts = self.b.list(adt_ty, &adts);
        let functions = self.b.list(function_ty, &functions);
        let mk = self.ts("Program.mk");
        Ok(self.b.apps(mk, &[adts, functions]))
    }
}
