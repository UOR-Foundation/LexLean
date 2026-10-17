//! The Lean denotation of a source term as a kernel term: what the generated
//! module states the term to be, fully elaborated (SPEC.md §17.17).
//!
//! A term the module writes as a call is a call of the constant, with the type
//! arguments the source gives; a primitive is the runtime function of the
//! term's module at the instance the elaborator selects for the operand
//! types; a `match` is the constructor-by-constructor eliminator of the
//! scrutinee's type, which the module's own matcher unfolds to.

use super::super::{element_of, integer_lean, primitive_name};
use super::{LocalTerm, Scope, Writer};
use crate::ir::core::CoreLevel;
use crate::ir::semantic::{
    MemberRef, SemanticAssignment, SemanticBranch, SemanticEdge, SemanticMapEntry,
    SemanticPrimitive, SemanticTerm, SemanticType,
};
use crate::production::kernel::Term;
use crate::production::source::{Constructor, Local, Site};

impl Writer<'_, '_> {
    // --- Scopes -----------------------------------------------------------

    /// `scope` with `locals` bound to the free variables `frees` in order.
    pub(super) fn bind(
        &self,
        scope: &Scope,
        locals: Vec<(Local, SemanticType)>,
        frees: &[Term],
        env: Term,
    ) -> Scope {
        let mut out = scope.clone();
        out.ctx = scope.ctx.with(locals);
        out.terms.extend(frees.iter().map(|free| LocalTerm {
            value: *free,
            companion: None,
        }));
        out.env = env;
        out
    }

    // --- Names ------------------------------------------------------------

    /// The unquoted Lean name of a member read at `site`.
    pub(super) fn member_name(&self, member: &MemberRef, site: &Site) -> Result<String, String> {
        if member.module.is_none() {
            if member.name == "Result.error" {
                return Ok("Except.error".to_owned());
            }
            if member.name == "Result.ok" {
                return Ok("Except.ok".to_owned());
            }
            let owner = member.name.rsplit_once('.').map(|(owner, _)| owner);
            if super::super::BUILTIN_OWNERS
                .iter()
                .any(|(name, _)| Some(*name) == owner)
            {
                return Ok(member.name.clone());
            }
        }
        let module = crate::production::source::Source::module_of(member, site);
        Ok(format!(
            "{}.{}",
            self.gen.source.lean_module(&module)?,
            member.name
        ))
    }

    pub(super) fn member_const(&mut self, member: &MemberRef, site: &Site) -> Result<Term, String> {
        let name = self.member_name(member, site)?;
        Ok(self.b.konst(&name))
    }

    /// A runtime function of the module `site` is read in.
    fn runtime(&mut self, site: &Site, namespace: &str, name: &str) -> Result<Term, String> {
        let _ = site;
        Ok(self.b.konst(&format!("{namespace}.{name}")))
    }

    // --- Instances --------------------------------------------------------

    fn fixed_name(ty: &SemanticType) -> Result<&'static str, String> {
        Ok(match ty {
            SemanticType::Int8 => "Int8",
            SemanticType::Int16 => "Int16",
            SemanticType::Int32 => "Int32",
            SemanticType::Int64 => "Int64",
            SemanticType::UInt8 => "UInt8",
            SemanticType::UInt16 => "UInt16",
            SemanticType::UInt32 => "UInt32",
            SemanticType::UInt64 => "UInt64",
            SemanticType::Type
            | SemanticType::Parameter { name: _ }
            | SemanticType::Nat
            | SemanticType::Bool
            | SemanticType::Prop
            | SemanticType::Unit
            | SemanticType::Int
            | SemanticType::String
            | SemanticType::Bytes
            | SemanticType::Ordering
            | SemanticType::Option { value: _ }
            | SemanticType::Result { ok: _, error: _ }
            | SemanticType::List { element: _ }
            | SemanticType::Named {
                member: _,
                arguments: _,
            }
            | SemanticType::Product { left: _, right: _ }
            | SemanticType::Function {
                parameters: _,
                result: _,
            }
            | SemanticType::Map { key: _, value: _ }
            | SemanticType::Set { element: _ }
            | SemanticType::ContractViolation
            | SemanticType::ReasoningFailure => {
                return Err("a fixed-width operation at a non-fixed type".to_owned());
            }
        })
    }

    fn fixed_instance(&mut self, site: &Site, ty: &SemanticType) -> Result<Term, String> {
        let name = Self::fixed_name(ty)?;
        self.runtime(site, "LexLeanRuntime", &format!("instFixed{name}"))
    }

    /// The `ToMathInt` instance of an operand of `checkedConvert`.
    fn math_int_instance(&mut self, site: &Site, ty: &SemanticType) -> Result<Term, String> {
        match ty {
            SemanticType::Int => self.runtime(site, "LexLeanRuntime", "instToMathIntInt"),
            SemanticType::Type
            | SemanticType::Parameter { .. }
            | SemanticType::Nat
            | SemanticType::Bool
            | SemanticType::Prop
            | SemanticType::Unit
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
            | SemanticType::Ordering
            | SemanticType::Option { .. }
            | SemanticType::Result { .. }
            | SemanticType::List { .. }
            | SemanticType::Named { .. }
            | SemanticType::Product { .. }
            | SemanticType::Function { .. }
            | SemanticType::Map { .. }
            | SemanticType::Set { .. }
            | SemanticType::ContractViolation
            | SemanticType::ReasoningFailure => {
                let name = Self::fixed_name(ty)?;
                let to = self.runtime(site, "LexLeanRuntime", "Fixed.toToMathInt")?;
                let fixed = self.lean_ty(ty)?;
                let instance = self.runtime(site, "LexLeanRuntime", &format!("instFixed{name}"))?;
                Ok(self.b.apps(to, &[fixed, instance]))
            }
        }
    }

    /// The `BEq` instance of an operand of `equal`.
    fn beq_instance(&mut self, ty: &SemanticType) -> Result<Term, String> {
        let name = match ty {
            SemanticType::Nat => "Nat",
            SemanticType::Bool => "Bool",
            SemanticType::String => "String",
            SemanticType::Ordering => "Ordering",
            SemanticType::Int8
            | SemanticType::Int16
            | SemanticType::Int32
            | SemanticType::Int64
            | SemanticType::UInt8
            | SemanticType::UInt16
            | SemanticType::UInt32
            | SemanticType::UInt64 => Self::fixed_name(ty)?,
            SemanticType::Bytes => return Ok(self.b.konst("ByteArray.instBEq")),
            SemanticType::Type
            | SemanticType::Parameter { name: _ }
            | SemanticType::Prop
            | SemanticType::Unit
            | SemanticType::Int
            | SemanticType::Option { value: _ }
            | SemanticType::Result { ok: _, error: _ }
            | SemanticType::List { element: _ }
            | SemanticType::Named {
                member: _,
                arguments: _,
            }
            | SemanticType::Product { left: _, right: _ }
            | SemanticType::Function {
                parameters: _,
                result: _,
            }
            | SemanticType::Map { key: _, value: _ }
            | SemanticType::Set { element: _ }
            | SemanticType::ContractViolation
            | SemanticType::ReasoningFailure => {
                return Err("an equality at a non-scalar type".to_owned());
            }
        };
        let carrier = self.lean_ty(ty)?;
        let decide = self.b.konst(&format!("instDecidableEq{name}"));
        let beq = self.b.konst0("instBEqOfDecidableEq");
        Ok(self.b.apps(beq, &[carrier, decide]))
    }

    /// The `Decimal` instance of the type a decimal is parsed to or printed
    /// from.
    fn decimal_instance(&mut self, site: &Site, ty: &SemanticType) -> Result<Term, String> {
        if matches!(ty, SemanticType::Int) {
            return self.runtime(site, "LexLeanRuntime", "instDecimalInt");
        }
        let name = Self::fixed_name(ty)?;
        let carrier = self.lean_ty(ty)?;
        let head = self.runtime(site, "LexLeanRuntime", "instDecimalOfFixedOfToString")?;
        let fixed = self.fixed_instance(site, ty)?;
        let to_string = self.b.konst(&format!("instToString{name}"));
        Ok(self.b.apps(head, &[carrier, fixed, to_string]))
    }

    /// A primitive's runtime call on operands of the given types, as the
    /// module elaborates `(LexLeanRuntime.op args : result)`.
    #[allow(clippy::too_many_lines)]
    pub(super) fn runtime_call(
        &mut self,
        site: &Site,
        operation: SemanticPrimitive,
        types: &[SemanticType],
        result: &SemanticType,
        args: &[Term],
    ) -> Result<Term, String> {
        let first = types.first();
        let (namespace, name) = primitive_name(operation)?;
        let int64 = SemanticType::Option {
            value: Box::new(SemanticType::Int64),
        };
        let special = match operation {
            SemanticPrimitive::ToInt | SemanticPrimitive::ToNat => {
                return Err(format!(
                    "{operation:?} is certified only through its elaboration"
                ));
            }
            SemanticPrimitive::CheckedAdd => Some("checkedAddInt64"),
            SemanticPrimitive::CheckedSubtract => Some("checkedSubtractInt64"),
            SemanticPrimitive::CheckedMultiply => Some("checkedMultiplyInt64"),
            SemanticPrimitive::CheckedNegate => Some("checkedNegateInt64"),
            SemanticPrimitive::CheckedQuotient => Some("checkedQuotientInt64"),
            SemanticPrimitive::Subtract
            | SemanticPrimitive::Multiply
            | SemanticPrimitive::Quotient
            | SemanticPrimitive::Remainder
            | SemanticPrimitive::Negate
            | SemanticPrimitive::CheckedConvert
            | SemanticPrimitive::BitAnd
            | SemanticPrimitive::BitOr
            | SemanticPrimitive::BitXor
            | SemanticPrimitive::BitNot
            | SemanticPrimitive::ShiftLeft
            | SemanticPrimitive::ShiftRight
            | SemanticPrimitive::Append
            | SemanticPrimitive::Length
            | SemanticPrimitive::Index
            | SemanticPrimitive::Slice
            | SemanticPrimitive::Utf8Encode
            | SemanticPrimitive::Utf8Decode
            | SemanticPrimitive::CompareBytes
            | SemanticPrimitive::Equal
            | SemanticPrimitive::SplitExact
            | SemanticPrimitive::Join
            | SemanticPrimitive::ParseDecimal
            | SemanticPrimitive::FormatDecimal
            | SemanticPrimitive::MapInsert
            | SemanticPrimitive::MapRemove
            | SemanticPrimitive::MapLookup
            | SemanticPrimitive::MapContains
            | SemanticPrimitive::MapSize
            | SemanticPrimitive::MapKeys
            | SemanticPrimitive::MapValues
            | SemanticPrimitive::MapEntries
            | SemanticPrimitive::MapFold
            | SemanticPrimitive::SetInsert
            | SemanticPrimitive::SetRemove
            | SemanticPrimitive::SetContains
            | SemanticPrimitive::SetSize
            | SemanticPrimitive::SetElements
            | SemanticPrimitive::SetUnion
            | SemanticPrimitive::SetIntersection
            | SemanticPrimitive::SetDifference
            | SemanticPrimitive::SetFold
            | SemanticPrimitive::ListFold
            | SemanticPrimitive::Iterate
            | SemanticPrimitive::IterateUntil
            | SemanticPrimitive::GraphSuccessors
            | SemanticPrimitive::GraphReachable
            | SemanticPrimitive::GraphTopological
            | SemanticPrimitive::LessThan => None,
        };
        if let Some(special) = special {
            if *result == int64 {
                let head = self.runtime(site, "LexLeanRuntime", special)?;
                return Ok(self.b.apps(head, args));
            }
        }
        let head = self.runtime(site, namespace, name)?;
        let mut leading: Vec<Term> = Vec::new();
        match operation {
            SemanticPrimitive::ToInt | SemanticPrimitive::ToNat => {
                return Err(format!(
                    "{operation:?} is certified only through its elaboration"
                ));
            }
            SemanticPrimitive::CheckedAdd
            | SemanticPrimitive::CheckedSubtract
            | SemanticPrimitive::CheckedMultiply
            | SemanticPrimitive::CheckedNegate
            | SemanticPrimitive::CheckedQuotient
            | SemanticPrimitive::BitAnd
            | SemanticPrimitive::BitOr
            | SemanticPrimitive::BitXor
            | SemanticPrimitive::BitNot
            | SemanticPrimitive::ShiftLeft
            | SemanticPrimitive::ShiftRight => {
                let ty = first.ok_or("a fixed-width operation without an operand")?;
                leading.push(self.lean_ty(ty)?);
                leading.push(self.fixed_instance(site, ty)?);
            }
            SemanticPrimitive::CheckedConvert => {
                let from = first.ok_or("a conversion without an operand")?;
                let to = match result {
                    SemanticType::Option { value } => value.as_ref(),
                    SemanticType::Type
                    | SemanticType::Parameter { .. }
                    | SemanticType::Nat
                    | SemanticType::Bool
                    | SemanticType::Prop
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
                    | SemanticType::Ordering
                    | SemanticType::Result { .. }
                    | SemanticType::List { .. }
                    | SemanticType::Named { .. }
                    | SemanticType::Product { .. }
                    | SemanticType::Function { .. }
                    | SemanticType::Map { .. }
                    | SemanticType::Set { .. }
                    | SemanticType::ContractViolation
                    | SemanticType::ReasoningFailure => {
                        return Err("a conversion to a non-option".to_owned())
                    }
                };
                leading.push(self.lean_ty(from)?);
                leading.push(self.lean_ty(to)?);
                leading.push(self.math_int_instance(site, from)?);
                leading.push(self.fixed_instance(site, to)?);
            }
            SemanticPrimitive::Subtract
            | SemanticPrimitive::Multiply
            | SemanticPrimitive::Negate => {
                let ty = first.ok_or("an arithmetic operation without an operand")?;
                leading.push(self.lean_ty(ty)?);
                let instance = match (operation, ty) {
                    (SemanticPrimitive::Subtract, SemanticType::Nat) => "instSubNat",
                    (SemanticPrimitive::Subtract, _) => "Int.instSub",
                    (SemanticPrimitive::Multiply, SemanticType::Nat) => "instMulNat",
                    (SemanticPrimitive::Multiply, _) => "Int.instMul",
                    (_, _) => "Int.instNegInt",
                };
                leading.push(self.b.konst(instance));
            }
            SemanticPrimitive::Quotient | SemanticPrimitive::Remainder => {
                let ty = first.ok_or("a division without an operand")?;
                leading.push(self.lean_ty(ty)?);
                let instance = if matches!(ty, SemanticType::Nat) {
                    "instQuotientNat"
                } else {
                    "instQuotientInt"
                };
                leading.push(self.runtime(site, "LexLeanRuntime", instance)?);
            }
            SemanticPrimitive::Append | SemanticPrimitive::Length | SemanticPrimitive::Slice => {
                let ty = first.ok_or("a sequence operation without an operand")?;
                leading.push(self.lean_ty(ty)?);
                let class = match operation {
                    SemanticPrimitive::ToInt | SemanticPrimitive::ToNat => {
                        return Err(format!(
                            "{operation:?} is certified only through its elaboration"
                        ));
                    }
                    SemanticPrimitive::Append => "Appendable",
                    SemanticPrimitive::Length => "Lengthable",
                    SemanticPrimitive::Subtract
                    | SemanticPrimitive::Multiply
                    | SemanticPrimitive::Quotient
                    | SemanticPrimitive::Remainder
                    | SemanticPrimitive::Negate
                    | SemanticPrimitive::CheckedConvert
                    | SemanticPrimitive::CheckedAdd
                    | SemanticPrimitive::CheckedSubtract
                    | SemanticPrimitive::CheckedMultiply
                    | SemanticPrimitive::CheckedNegate
                    | SemanticPrimitive::CheckedQuotient
                    | SemanticPrimitive::BitAnd
                    | SemanticPrimitive::BitOr
                    | SemanticPrimitive::BitXor
                    | SemanticPrimitive::BitNot
                    | SemanticPrimitive::ShiftLeft
                    | SemanticPrimitive::ShiftRight
                    | SemanticPrimitive::Index
                    | SemanticPrimitive::Slice
                    | SemanticPrimitive::Utf8Encode
                    | SemanticPrimitive::Utf8Decode
                    | SemanticPrimitive::CompareBytes
                    | SemanticPrimitive::Equal
                    | SemanticPrimitive::SplitExact
                    | SemanticPrimitive::Join
                    | SemanticPrimitive::ParseDecimal
                    | SemanticPrimitive::FormatDecimal
                    | SemanticPrimitive::MapInsert
                    | SemanticPrimitive::MapRemove
                    | SemanticPrimitive::MapLookup
                    | SemanticPrimitive::MapContains
                    | SemanticPrimitive::MapSize
                    | SemanticPrimitive::MapKeys
                    | SemanticPrimitive::MapValues
                    | SemanticPrimitive::MapEntries
                    | SemanticPrimitive::MapFold
                    | SemanticPrimitive::SetInsert
                    | SemanticPrimitive::SetRemove
                    | SemanticPrimitive::SetContains
                    | SemanticPrimitive::SetSize
                    | SemanticPrimitive::SetElements
                    | SemanticPrimitive::SetUnion
                    | SemanticPrimitive::SetIntersection
                    | SemanticPrimitive::SetDifference
                    | SemanticPrimitive::SetFold
                    | SemanticPrimitive::ListFold
                    | SemanticPrimitive::Iterate
                    | SemanticPrimitive::IterateUntil
                    | SemanticPrimitive::GraphSuccessors
                    | SemanticPrimitive::GraphReachable
                    | SemanticPrimitive::GraphTopological
                    | SemanticPrimitive::LessThan => "Sliceable",
                };
                leading.push(self.sequence_instance(site, class, ty)?);
            }
            SemanticPrimitive::Index => {
                let ty = first.ok_or("an index without an operand")?;
                leading.push(self.lean_ty(ty)?);
                match ty {
                    SemanticType::Bytes => {
                        leading.push(self.b.konst("UInt8"));
                        leading.push(self.runtime(
                            site,
                            "LexLeanRuntime",
                            "instIndexableByteArrayUInt8",
                        )?);
                    }
                    SemanticType::Type
                    | SemanticType::Parameter { .. }
                    | SemanticType::Nat
                    | SemanticType::Bool
                    | SemanticType::Prop
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
                    | SemanticType::Ordering
                    | SemanticType::Option { .. }
                    | SemanticType::Result { .. }
                    | SemanticType::List { .. }
                    | SemanticType::Named { .. }
                    | SemanticType::Product { .. }
                    | SemanticType::Function { .. }
                    | SemanticType::Map { .. }
                    | SemanticType::Set { .. }
                    | SemanticType::ContractViolation
                    | SemanticType::ReasoningFailure => {
                        let element = self.lean_ty(&element_of(ty)?)?;
                        leading.push(element);
                        let head = self.runtime(site, "LexLeanRuntime", "instIndexableList")?;
                        leading.push(self.b.app(head, element));
                    }
                }
            }
            SemanticPrimitive::Equal => {
                let ty = first.ok_or("an equality without an operand")?;
                leading.push(self.lean_ty(ty)?);
                leading.push(self.beq_instance(ty)?);
            }
            SemanticPrimitive::ParseDecimal => {
                let target = match result {
                    SemanticType::Option { value } => value.as_ref(),
                    SemanticType::Type
                    | SemanticType::Parameter { .. }
                    | SemanticType::Nat
                    | SemanticType::Bool
                    | SemanticType::Prop
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
                    | SemanticType::Ordering
                    | SemanticType::Result { .. }
                    | SemanticType::List { .. }
                    | SemanticType::Named { .. }
                    | SemanticType::Product { .. }
                    | SemanticType::Function { .. }
                    | SemanticType::Map { .. }
                    | SemanticType::Set { .. }
                    | SemanticType::ContractViolation
                    | SemanticType::ReasoningFailure => {
                        return Err("a decimal parse to a non-option".to_owned())
                    }
                };
                leading.push(self.lean_ty(target)?);
                leading.push(self.decimal_instance(site, target)?);
            }
            SemanticPrimitive::FormatDecimal => {
                let ty = first.ok_or("a decimal format without an operand")?;
                leading.push(self.lean_ty(ty)?);
                leading.push(self.decimal_instance(site, ty)?);
            }
            SemanticPrimitive::Utf8Encode
            | SemanticPrimitive::Utf8Decode
            | SemanticPrimitive::CompareBytes
            | SemanticPrimitive::SplitExact
            | SemanticPrimitive::Join => {}
            SemanticPrimitive::MapInsert
            | SemanticPrimitive::MapRemove
            | SemanticPrimitive::MapLookup
            | SemanticPrimitive::MapContains
            | SemanticPrimitive::MapSize
            | SemanticPrimitive::MapKeys
            | SemanticPrimitive::MapValues
            | SemanticPrimitive::MapEntries
            | SemanticPrimitive::MapFold
            | SemanticPrimitive::SetInsert
            | SemanticPrimitive::SetRemove
            | SemanticPrimitive::SetContains
            | SemanticPrimitive::SetSize
            | SemanticPrimitive::SetElements
            | SemanticPrimitive::SetUnion
            | SemanticPrimitive::SetIntersection
            | SemanticPrimitive::SetDifference
            | SemanticPrimitive::SetFold
            | SemanticPrimitive::ListFold
            | SemanticPrimitive::Iterate
            | SemanticPrimitive::IterateUntil
            | SemanticPrimitive::GraphSuccessors
            | SemanticPrimitive::GraphReachable
            | SemanticPrimitive::GraphTopological
            | SemanticPrimitive::LessThan => {
                return self.collection_call(site, operation, types, args);
            }
        }
        // `quotient` and `remainder` take their zero case as a third operand.
        let mut all = leading;
        all.extend_from_slice(args);
        Ok(self.b.apps(head, &all))
    }

    fn sequence_instance(
        &mut self,
        site: &Site,
        class: &str,
        ty: &SemanticType,
    ) -> Result<Term, String> {
        match ty {
            SemanticType::Bytes => {
                self.runtime(site, "LexLeanRuntime", &format!("inst{class}ByteArray"))
            }
            SemanticType::String => {
                self.runtime(site, "LexLeanRuntime", &format!("inst{class}String"))
            }
            SemanticType::Type
            | SemanticType::Parameter { .. }
            | SemanticType::Nat
            | SemanticType::Bool
            | SemanticType::Prop
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
            | SemanticType::Ordering
            | SemanticType::Option { .. }
            | SemanticType::Result { .. }
            | SemanticType::List { .. }
            | SemanticType::Named { .. }
            | SemanticType::Product { .. }
            | SemanticType::Function { .. }
            | SemanticType::Map { .. }
            | SemanticType::Set { .. }
            | SemanticType::ContractViolation
            | SemanticType::ReasoningFailure => {
                let element = self.lean_ty(&element_of(ty)?)?;
                let head = self.runtime(site, "LexLeanRuntime", &format!("inst{class}List"))?;
                Ok(self.b.app(head, element))
            }
        }
    }

    // --- Constructors and eliminators ---------------------------------------

    /// A constructor applied to its arguments, the closed type `ty` being the
    /// type it builds.
    pub(super) fn constructed(
        &mut self,
        constructor: &Constructor,
        ty: &SemanticType,
        member: &MemberRef,
        site: &Site,
        args: &[Term],
    ) -> Result<Term, String> {
        Ok(match constructor {
            Constructor::Bool(value) => self.boolean(*value),
            Constructor::Zero => self.b.konst("Nat.zero"),
            Constructor::Succ => {
                let head = self.b.konst("Nat.succ");
                self.b.apps(head, args)
            }
            Constructor::Nil => {
                let element = self.lean_ty(&element_of(ty)?)?;
                let head = self.b.konst0("List.nil");
                self.b.app(head, element)
            }
            Constructor::Cons => {
                let element = self.lean_ty(&element_of(ty)?)?;
                let head = self.b.konst0("List.cons");
                let mut all = vec![element];
                all.extend_from_slice(args);
                self.b.apps(head, &all)
            }
            Constructor::OptionNone | Constructor::OptionSome => {
                let value = super::super::option_value(ty)?;
                let value = self.lean_ty(&value)?;
                let name = if matches!(constructor, Constructor::OptionNone) {
                    "Option.none"
                } else {
                    "Option.some"
                };
                let head = self.b.konst0(name);
                let mut all = vec![value];
                all.extend_from_slice(args);
                self.b.apps(head, &all)
            }
            Constructor::Ok | Constructor::Error => {
                let (ok, error) = match ty {
                    SemanticType::Result { ok, error } => (ok.as_ref(), error.as_ref()),
                    SemanticType::Type
                    | SemanticType::Parameter { .. }
                    | SemanticType::Nat
                    | SemanticType::Bool
                    | SemanticType::Prop
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
                    | SemanticType::Ordering
                    | SemanticType::Option { .. }
                    | SemanticType::List { .. }
                    | SemanticType::Named { .. }
                    | SemanticType::Product { .. }
                    | SemanticType::Function { .. }
                    | SemanticType::Map { .. }
                    | SemanticType::Set { .. }
                    | SemanticType::ContractViolation
                    | SemanticType::ReasoningFailure => {
                        return Err("a result constructor at a non-result type".to_owned())
                    }
                };
                let ok = self.lean_ty(ok)?;
                let error = self.lean_ty(error)?;
                let name = if matches!(constructor, Constructor::Ok) {
                    "Except.ok"
                } else {
                    "Except.error"
                };
                let head = self.b.konst00(name);
                let mut all = vec![error, ok];
                all.extend_from_slice(args);
                self.b.apps(head, &all)
            }
            Constructor::Violation(first, second) => {
                let bool = self.b.konst("Bool");
                let a = self.boolean(*first);
                let b = self.boolean(*second);
                self.b.pair(bool, bool, a, b)
            }
            Constructor::Document { ty: _, index: _ } => {
                let head = self.member_const(member, site)?;
                let mut all = Vec::new();
                if let SemanticType::Named {
                    member: _,
                    arguments,
                } = ty
                {
                    for argument in arguments {
                        all.push(self.lean_ty(argument)?);
                    }
                }
                all.extend_from_slice(args);
                self.b.apps(head, &all)
            }
        })
    }

    /// The eliminator of the type of `scrutinee` at the constant motive
    /// `result : Type`, applied to the minor premises of its constructors in
    /// declaration order.
    pub(super) fn cases_on_const(
        &mut self,
        scrutinee_ty: &SemanticType,
        scrutinee: Term,
        result: Term,
        minors: &[Term],
    ) -> Result<Term, String> {
        let carrier = self.lean_ty(scrutinee_ty)?;
        let lifted = self.b.lift(result, 1, 0);
        let motive = self.b.lam("_", super::EXPLICIT, carrier, lifted);
        let one = CoreLevel::Succ(Box::new(CoreLevel::Zero));
        self.cases_on(scrutinee_ty, scrutinee, motive, one, minors)
    }

    /// The eliminator of the type of `scrutinee` at `motive`, a function of
    /// the scrutinee into the sort of level `level`, applied to the minor
    /// premises of its constructors in declaration order.
    pub(super) fn cases_on(
        &mut self,
        scrutinee_ty: &SemanticType,
        scrutinee: Term,
        motive: Term,
        level: CoreLevel,
        minors: &[Term],
    ) -> Result<Term, String> {
        let (head, params): (Term, Vec<Term>) = match scrutinee_ty {
            SemanticType::Bool => (self.b.konst_at("Bool.casesOn", vec![level]), Vec::new()),
            SemanticType::Nat => (self.b.konst_at("Nat.casesOn", vec![level]), Vec::new()),
            SemanticType::List { element } | SemanticType::Set { element } => {
                let element = self.lean_ty(element)?;
                (
                    self.b
                        .konst_at("List.casesOn", vec![level, CoreLevel::Zero]),
                    vec![element],
                )
            }
            SemanticType::Option { value } => {
                let value = self.lean_ty(value)?;
                (
                    self.b
                        .konst_at("Option.casesOn", vec![level, CoreLevel::Zero]),
                    vec![value],
                )
            }
            SemanticType::Result { ok, error } => {
                let error = self.lean_ty(error)?;
                let ok = self.lean_ty(ok)?;
                (
                    self.b.konst_at(
                        "Except.casesOn",
                        vec![level, CoreLevel::Zero, CoreLevel::Zero],
                    ),
                    vec![error, ok],
                )
            }
            SemanticType::Named { member, arguments } => {
                let owner = self.document_name(member)?;
                let mut params = Vec::new();
                for argument in arguments {
                    params.push(self.lean_ty(argument)?);
                }
                (
                    self.b.konst_at(&format!("{owner}.casesOn"), vec![level]),
                    params,
                )
            }
            SemanticType::Product { left, right } => {
                let left = self.lean_ty(left)?;
                let right = self.lean_ty(right)?;
                (
                    self.b.konst_at(
                        "Prod.casesOn",
                        vec![level, CoreLevel::Zero, CoreLevel::Zero],
                    ),
                    vec![left, right],
                )
            }
            SemanticType::Map { key, value } => {
                let key = self.lean_ty(key)?;
                let value = self.lean_ty(value)?;
                let entry = self.b.pair_ty(key, value);
                (
                    self.b
                        .konst_at("List.casesOn", vec![level, CoreLevel::Zero]),
                    vec![entry],
                )
            }
            SemanticType::ContractViolation | SemanticType::ReasoningFailure => {
                let bool = self.b.konst("Bool");
                (
                    self.b.konst_at(
                        "Prod.casesOn",
                        vec![level, CoreLevel::Zero, CoreLevel::Zero],
                    ),
                    vec![bool, bool],
                )
            }
            SemanticType::Type
            | SemanticType::Parameter { name: _ }
            | SemanticType::Prop
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
            | SemanticType::Ordering
            | SemanticType::Function {
                parameters: _,
                result: _,
            } => {
                return Err(format!(
                    "a match on `{}` has no eliminator here",
                    self.gen.source.type_text(scrutinee_ty)
                ));
            }
        };
        let mut all = params;
        all.push(motive);
        all.push(scrutinee);
        all.extend_from_slice(minors);
        Ok(self.b.apps(head, &all))
    }

    // --- The denotation ---------------------------------------------------

    pub(super) fn src_all(
        &mut self,
        terms: &[SemanticTerm],
        scope: &Scope,
    ) -> Result<Vec<Term>, String> {
        terms.iter().map(|term| self.src(term, scope)).collect()
    }

    /// The closed type of `term` in `scope`.
    pub(super) fn infer(&self, term: &SemanticTerm, scope: &Scope) -> Result<SemanticType, String> {
        self.gen
            .source
            .infer(term, &scope.ctx.sources(), &scope.ctx.site)
    }

    /// The Lean denotation of a source term.
    #[allow(clippy::too_many_lines)]
    pub(super) fn src(&mut self, term: &SemanticTerm, scope: &Scope) -> Result<Term, String> {
        let site = scope.ctx.site.clone();
        Ok(match term {
            SemanticTerm::Var { name } => scope.lookup(name)?.1.value,
            SemanticTerm::Nat { value } => self.nat_lit(value),
            SemanticTerm::Integer {
                representation,
                value,
            } => self.int_literal(integer_lean(*representation), value),
            SemanticTerm::String { value } => self.b.string(value),
            SemanticTerm::Bytes { hex } => self.bytes_term(hex)?,
            SemanticTerm::Primitive {
                operation,
                arguments,
                result,
            } => {
                let result = self.gen.source.close(result, &site);
                let types = arguments
                    .iter()
                    .map(|argument| self.infer(argument, scope))
                    .collect::<Result<Vec<_>, _>>()?;
                let args = self.src_all(arguments, scope)?;
                self.runtime_call(&site, *operation, &types, &result, &args)?
            }
            SemanticTerm::Bool { value } => self.boolean(*value),
            SemanticTerm::Unit => self.b.konst("Unit.unit"),
            SemanticTerm::Nil { element } => {
                let element = self.gen.source.close(element, &site);
                let element = self.lean_ty(&element)?;
                let head = self.b.konst0("List.nil");
                self.b.app(head, element)
            }
            SemanticTerm::Cons { head, tail } => {
                let ty = self.infer(term, scope)?;
                let element = self.lean_ty(&element_of(&ty)?)?;
                let head = self.src(head, scope)?;
                let tail = self.src(tail, scope)?;
                let cons = self.b.konst0("List.cons");
                self.b.apps(cons, &[element, head, tail])
            }
            SemanticTerm::Record {
                r#type: _,
                type_arguments: _,
                fields,
            } => {
                let ty = self.infer(term, scope)?;
                self.record_value(&ty, fields, scope)?
            }
            SemanticTerm::Constructor {
                constructor,
                type_arguments,
                arguments,
            } => {
                let ty = self.infer(term, scope)?;
                let closed: Vec<SemanticType> = type_arguments
                    .iter()
                    .map(|argument| self.gen.source.close(argument, &site))
                    .collect();
                let (kind, _) = self.gen.source.constructor(constructor, &closed, &site)?;
                let args = self.src_all(arguments, scope)?;
                self.constructed(&kind, &ty, constructor, &site, &args)?
            }
            SemanticTerm::InstanceValue {
                class: _,
                arguments: _,
                resolved,
            } => self.member_const(resolved, &site)?,
            SemanticTerm::Project { value, field } => {
                let ty = self.infer(value, scope)?;
                let owner = match &ty {
                    SemanticType::Named { member, arguments } => {
                        let mut params = Vec::new();
                        for argument in arguments {
                            params.push(self.lean_ty(argument)?);
                        }
                        (self.document_name(member)?, params)
                    }
                    SemanticType::Type
                    | SemanticType::Parameter { .. }
                    | SemanticType::Nat
                    | SemanticType::Bool
                    | SemanticType::Prop
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
                    | SemanticType::Ordering
                    | SemanticType::Option { .. }
                    | SemanticType::Result { .. }
                    | SemanticType::List { .. }
                    | SemanticType::Product { .. }
                    | SemanticType::Function { .. }
                    | SemanticType::Map { .. }
                    | SemanticType::Set { .. }
                    | SemanticType::ContractViolation
                    | SemanticType::ReasoningFailure => {
                        return Err("a projection from a non-document".to_owned())
                    }
                };
                let head = self.b.konst(&format!("{}.{field}", owner.0));
                let value = self.src(value, scope)?;
                let mut all = owner.1;
                all.push(value);
                self.b.apps(head, &all)
            }
            SemanticTerm::Call {
                function,
                type_arguments,
                arguments,
            } => {
                let head = self.member_const(function, &site)?;
                let mut all = Vec::new();
                for argument in type_arguments {
                    let closed = self.gen.source.close(argument, &site);
                    all.push(self.lean_ty(&closed)?);
                }
                for argument in arguments {
                    all.push(self.src(argument, scope)?);
                }
                self.b.apps(head, &all)
            }
            SemanticTerm::If {
                condition,
                then_value,
                else_value,
            } => {
                let ty = self.infer(term, scope)?;
                let ty = self.lean_ty(&ty)?;
                let condition = self.src(condition, scope)?;
                let then_value = self.src(then_value, scope)?;
                let else_value = self.src(else_value, scope)?;
                if self.node_of(scope, term).is_some() {
                    let bool = self.b.konst("Bool");
                    let (tru, fls) = (self.tru(), self.fls());
                    let (on_true, on_false) = (self.b.free(), self.b.free());
                    let true_eq = self.eq(bool, condition, tru);
                    let false_eq = self.eq(bool, condition, fls);
                    let then_arm = self.arm_lambda(&[], (on_true, true_eq), then_value);
                    let else_arm = self.arm_lambda(&[], (on_false, false_eq), else_value);
                    let one = CoreLevel::Succ(Box::new(CoreLevel::Zero));
                    self.equation_cases(
                        &SemanticType::Bool,
                        condition,
                        ty,
                        one,
                        vec![(0, else_arm), (1, then_arm)],
                    )?
                } else {
                    self.ite(ty, condition, then_value, else_value)
                }
            }
            SemanticTerm::Match {
                scrutinee,
                branches,
            } => self.src_match(term, scrutinee, branches, scope)?,
            SemanticTerm::Add { left, right } => {
                let left = self.src(left, scope)?;
                let right = self.src(right, scope)?;
                self.nat_add(left, right)
            }
            SemanticTerm::Beq { left, right } => {
                let left = self.src(left, scope)?;
                let right = self.src(right, scope)?;
                let head = self.b.konst("Nat.beq");
                self.b.apps(head, &[left, right])
            }
            SemanticTerm::Ble { left, right } => {
                let left = self.src(left, scope)?;
                let right = self.src(right, scope)?;
                let head = self.b.konst("Nat.ble");
                self.b.apps(head, &[left, right])
            }
            SemanticTerm::Blt { left, right } => {
                let left = self.src(left, scope)?;
                let right = self.src(right, scope)?;
                self.nat_blt(left, right)
            }
            SemanticTerm::And { left, right } => {
                let left = self.src(left, scope)?;
                let right = self.src(right, scope)?;
                self.and(left, right)
            }
            SemanticTerm::Or { left, right } => {
                let left = self.src(left, scope)?;
                let right = self.src(right, scope)?;
                let head = self.b.konst("Bool.or");
                self.b.apps(head, &[left, right])
            }
            SemanticTerm::Not { value } => {
                let value = self.src(value, scope)?;
                self.not(value)
            }
            SemanticTerm::Let {
                binder,
                value,
                body,
            } => {
                let (local, generic) = self.gen.let_local(binder, &scope.ctx);
                let ty = self.lean_ty(&local.ty.clone())?;
                let bound = self.src(value, scope)?;
                let free = self.b.free();
                let inner = self.bind(scope, vec![(local, generic)], &[free], scope.env);
                let body = self.src(body, &inner)?;
                self.b.let_over(
                    &super::super::identifier(&binder.name),
                    ty,
                    bound,
                    free,
                    body,
                )
            }
            SemanticTerm::Pair { left, right } => {
                let ty = self.infer(term, scope)?;
                let (alpha, beta) = match &ty {
                    SemanticType::Product { left, right } => (left.as_ref(), right.as_ref()),
                    SemanticType::Type
                    | SemanticType::Parameter { .. }
                    | SemanticType::Nat
                    | SemanticType::Bool
                    | SemanticType::Prop
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
                    | SemanticType::Ordering
                    | SemanticType::Option { .. }
                    | SemanticType::Result { .. }
                    | SemanticType::List { .. }
                    | SemanticType::Named { .. }
                    | SemanticType::Function { .. }
                    | SemanticType::Map { .. }
                    | SemanticType::Set { .. }
                    | SemanticType::ContractViolation
                    | SemanticType::ReasoningFailure => {
                        return Err("a pair of a non-product type".to_owned())
                    }
                };
                let alpha = self.lean_ty(alpha)?;
                let beta = self.lean_ty(beta)?;
                let left = self.src(left, scope)?;
                let right = self.src(right, scope)?;
                self.b.pair(alpha, beta, left, right)
            }
            SemanticTerm::First { value } | SemanticTerm::Second { value } => {
                let ty = self.infer(value, scope)?;
                let (alpha, beta) = match &ty {
                    SemanticType::Product { left, right } => (left.as_ref(), right.as_ref()),
                    SemanticType::Type
                    | SemanticType::Parameter { .. }
                    | SemanticType::Nat
                    | SemanticType::Bool
                    | SemanticType::Prop
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
                    | SemanticType::Ordering
                    | SemanticType::Option { .. }
                    | SemanticType::Result { .. }
                    | SemanticType::List { .. }
                    | SemanticType::Named { .. }
                    | SemanticType::Function { .. }
                    | SemanticType::Map { .. }
                    | SemanticType::Set { .. }
                    | SemanticType::ContractViolation
                    | SemanticType::ReasoningFailure => {
                        return Err("a projection of a non-product".to_owned())
                    }
                };
                let alpha = self.lean_ty(alpha)?;
                let beta = self.lean_ty(beta)?;
                let value = self.src(value, scope)?;
                let name = if matches!(term, SemanticTerm::First { value: _ }) {
                    "Prod.fst"
                } else {
                    "Prod.snd"
                };
                let head = self.b.konst00(name);
                self.b.apps(head, &[alpha, beta, value])
            }
            SemanticTerm::Lambda {
                parameters,
                captures: _,
                body,
            } => {
                let bound: Vec<(Local, SemanticType)> = parameters
                    .iter()
                    .map(|binder| self.gen.let_local(binder, &scope.ctx))
                    .collect();
                let mut frees = Vec::new();
                let mut types = Vec::new();
                for (local, _) in &bound {
                    frees.push(self.b.free());
                    types.push(self.lean_ty(&local.ty.clone())?);
                }
                let inner = self.bind(scope, bound.clone(), &frees, scope.env);
                let mut out = self.src(body, &inner)?;
                for (position, (local, _)) in bound.iter().enumerate().rev() {
                    out = self.b.lam_over(
                        &super::super::identifier(&local.name),
                        super::EXPLICIT,
                        types[position],
                        frees[position],
                        out,
                    );
                }
                out
            }
            SemanticTerm::Apply {
                function,
                arguments,
            } => {
                let function = self.src(function, scope)?;
                let args = self.src_all(arguments, scope)?;
                self.b.apps(function, &args)
            }
            SemanticTerm::FunctionRef {
                function,
                type_arguments,
            } => {
                let head = self.member_const(function, &site)?;
                let mut all = Vec::new();
                for argument in type_arguments {
                    let closed = self.gen.source.close(argument, &site);
                    all.push(self.lean_ty(&closed)?);
                }
                self.b.apps(head, &all)
            }
            SemanticTerm::MapLiteral {
                key,
                value,
                entries,
            } => {
                let key_ty = self.gen.source.close(key, &site);
                let value_ty = self.gen.source.close(value, &site);
                let alpha = self.lean_ty(&key_ty)?;
                let beta = self.lean_ty(&value_ty)?;
                let mut items = Vec::new();
                for SemanticMapEntry { key, value } in entries {
                    let key = self.src(key, scope)?;
                    let value = self.src(value, scope)?;
                    items.push(self.b.pair(alpha, beta, key, value));
                }
                let entry = self.b.pair_ty(alpha, beta);
                self.b.list(entry, &items)
            }
            SemanticTerm::SetLiteral { element, elements } => {
                let element_ty = self.gen.source.close(element, &site);
                let ty = self.lean_ty(&element_ty)?;
                let items = self.src_all(elements, scope)?;
                self.b.list(ty, &items)
            }
            SemanticTerm::GraphLiteral { node, nodes, edges } => {
                let node_ty = self.gen.source.close(node, &site);
                let node_ty = self.lean_ty(&node_ty)?;
                let targets_ty = self.b.list_ty(node_ty);
                let mut items = Vec::new();
                for source in nodes {
                    let mut targets = Vec::new();
                    for SemanticEdge {
                        source: from,
                        target,
                    } in edges
                    {
                        if from == source {
                            targets.push(self.src(target, scope)?);
                        }
                    }
                    let source = self.src(source, scope)?;
                    let list = self.b.list(node_ty, &targets);
                    items.push(self.b.pair(node_ty, targets_ty, source, list));
                }
                let entry = self.b.pair_ty(node_ty, targets_ty);
                self.b.list(entry, &items)
            }
            SemanticTerm::CheckedApply {
                model: _,
                type_arguments: _,
                arguments: _,
                checks: _,
            } => {
                return Err(
                    "a checked model application is certified only through its elaboration"
                        .to_owned(),
                );
            }
            SemanticTerm::Eq { left: _, right: _ }
            | SemanticTerm::Le { left: _, right: _ }
            | SemanticTerm::Lt { left: _, right: _ }
            | SemanticTerm::PropAnd { left: _, right: _ }
            | SemanticTerm::Implies {
                premise: _,
                conclusion: _,
            }
            | SemanticTerm::Iff { left: _, right: _ }
            | SemanticTerm::Forall { binder: _, body: _ } => {
                return Err(format!(
                    "the formal-only construct `{}` has no runtime denotation",
                    super::super::super::eligibility::term_key(term)
                ));
            }
        })
    }

    /// A record value: the structure's constructor on its type arguments and
    /// its fields in declaration order.
    fn record_value(
        &mut self,
        ty: &SemanticType,
        fields: &[SemanticAssignment],
        scope: &Scope,
    ) -> Result<Term, String> {
        let shape = self.gen.source.document(ty)?;
        let (member, arguments) = match ty {
            SemanticType::Named { member, arguments } => (member, arguments),
            SemanticType::Type
            | SemanticType::Parameter { .. }
            | SemanticType::Nat
            | SemanticType::Bool
            | SemanticType::Prop
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
            | SemanticType::Ordering
            | SemanticType::Option { .. }
            | SemanticType::Result { .. }
            | SemanticType::List { .. }
            | SemanticType::Product { .. }
            | SemanticType::Function { .. }
            | SemanticType::Map { .. }
            | SemanticType::Set { .. }
            | SemanticType::ContractViolation
            | SemanticType::ReasoningFailure => {
                return Err("a record of a non-document type".to_owned())
            }
        };
        let owner = self.document_name(member)?;
        let mut all = Vec::new();
        for argument in arguments {
            all.push(self.lean_ty(argument)?);
        }
        for field in &shape.field_names {
            let SemanticAssignment { field: _, value } = fields
                .iter()
                .find(|assignment| &assignment.field == field)
                .ok_or_else(|| format!("field `{field}` is not assigned"))?;
            all.push(self.src(value, scope)?);
        }
        let head = self.b.konst(&format!("{owner}.mk"));
        Ok(self.b.apps(head, &all))
    }

    /// A source `match` as the eliminator of the scrutinee's type.
    fn src_match(
        &mut self,
        term: &SemanticTerm,
        scrutinee: &SemanticTerm,
        branches: &[SemanticBranch],
        scope: &Scope,
    ) -> Result<Term, String> {
        let scrutinee_ty = self.infer(scrutinee, scope)?;
        let result_ty = self.infer(term, scope)?;
        let result = self.lean_ty(&result_ty)?;
        let discriminant = self.src(scrutinee, scope)?;
        let numbered = self.node_of(scope, term).is_some();
        let carrier = self.lean_ty(&scrutinee_ty)?;
        let mut alternatives = Vec::new();
        for branch in branches {
            let locals = self.gen.branch_locals(branch, scrutinee, &scope.ctx)?;
            let mut frees = Vec::new();
            let mut types = Vec::new();
            for (local, _) in &locals {
                frees.push(self.b.free());
                types.push(self.lean_ty(&local.ty.clone())?);
            }
            let inner = self.bind(scope, locals.clone(), &frees, scope.env);
            let mut body = self.src(&branch.body, &inner)?;
            let (constructor, _) =
                self.gen
                    .source
                    .branch(branch, &scrutinee_ty, &scope.ctx.site)?;
            if numbered {
                let value =
                    self.constructor_value(&constructor, &scrutinee_ty, branch, &frees, scope)?;
                let equation = self.eq(carrier, discriminant, value);
                let hypothesis = self.b.free();
                let binders: Vec<(String, Term, Term)> = locals
                    .iter()
                    .enumerate()
                    .map(|(position, (local, _))| {
                        (
                            super::super::identifier(&local.name),
                            types[position],
                            frees[position],
                        )
                    })
                    .collect();
                body = self.arm_lambda(&binders, (hypothesis, equation), body);
            } else {
                for (position, (local, _)) in locals.iter().enumerate().rev() {
                    body = self.b.lam_over(
                        &super::super::identifier(&local.name),
                        super::EXPLICIT,
                        types[position],
                        frees[position],
                        body,
                    );
                }
            }
            alternatives.push((self.constructor_position(&constructor)?, body));
        }
        if numbered {
            let one = CoreLevel::Succ(Box::new(CoreLevel::Zero));
            return self.equation_cases(&scrutinee_ty, discriminant, result, one, alternatives);
        }
        alternatives.sort_by_key(|(position, _)| *position);
        let minors: Vec<Term> = alternatives.into_iter().map(|(_, body)| body).collect();
        self.cases_on_const(&scrutinee_ty, discriminant, result, &minors)
    }

    /// A constructor's position among its type's, in declaration order.
    pub(super) fn constructor_position(&self, constructor: &Constructor) -> Result<usize, String> {
        Ok(match constructor {
            Constructor::Bool(false) | Constructor::Zero | Constructor::Nil => 0,
            Constructor::Bool(true) | Constructor::Succ | Constructor::Cons => 1,
            Constructor::OptionNone => 0,
            Constructor::OptionSome => 1,
            Constructor::Error => 0,
            Constructor::Ok => 1,
            Constructor::Violation(_, _) => 0,
            Constructor::Document { ty: _, index } => *index,
        })
    }
}
