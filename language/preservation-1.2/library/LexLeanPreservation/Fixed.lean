import LexLeanPreservation.Primitives
namespace LexLeanPreservation
open LexLeanTarget.TargetSyntax LexLeanTarget.TargetSemantics

/-! Fixed-width primitives (SPEC.md §17.17): one relation per operation and
width. A checked operation's overflow is a value (`none`), never a target
overflow, so every lemma here has the exact width predicate `true`. -/

theorem prim_checkedAdd_u8 (a : UInt8) (b : UInt8) :
    obs (primitive .checkedAdd [Value.u8 a, Value.u8 b]) =
      some (Rel true (encOption Value.u8 (LexLeanRuntime.checkedAdd a b : Option UInt8))) := by
  show obs (match (LexLeanRuntime.checkedAdd a b : Option UInt8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u8 r)) 0) = _
  cases (LexLeanRuntime.checkedAdd a b : Option UInt8) <;> rfl

theorem prim_checkedSub_u8 (a : UInt8) (b : UInt8) :
    obs (primitive .checkedSub [Value.u8 a, Value.u8 b]) =
      some (Rel true (encOption Value.u8 (LexLeanRuntime.checkedSubtract a b : Option UInt8))) := by
  show obs (match (LexLeanRuntime.checkedSubtract a b : Option UInt8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u8 r)) 0) = _
  cases (LexLeanRuntime.checkedSubtract a b : Option UInt8) <;> rfl

theorem prim_checkedMul_u8 (a : UInt8) (b : UInt8) :
    obs (primitive .checkedMul [Value.u8 a, Value.u8 b]) =
      some (Rel true (encOption Value.u8 (LexLeanRuntime.checkedMultiply a b : Option UInt8))) := by
  show obs (match (LexLeanRuntime.checkedMultiply a b : Option UInt8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u8 r)) 0) = _
  cases (LexLeanRuntime.checkedMultiply a b : Option UInt8) <;> rfl

theorem prim_checkedQuot_u8 (a : UInt8) (b : UInt8) :
    obs (primitive .checkedQuot [Value.u8 a, Value.u8 b]) =
      some (Rel true (encOption Value.u8 (LexLeanRuntime.checkedQuotient a b : Option UInt8))) := by
  show obs (match (LexLeanRuntime.checkedQuotient a b : Option UInt8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u8 r)) 0) = _
  cases (LexLeanRuntime.checkedQuotient a b : Option UInt8) <;> rfl

theorem prim_bitAnd_u8 (a b : UInt8) :
    obs (primitive .bitAnd [Value.u8 a, Value.u8 b]) = some (Rel true (Value.u8 (LexLeanRuntime.bitAnd a b))) := rfl
theorem prim_bitOr_u8 (a b : UInt8) :
    obs (primitive .bitOr [Value.u8 a, Value.u8 b]) = some (Rel true (Value.u8 (LexLeanRuntime.bitOr a b))) := rfl
theorem prim_bitXor_u8 (a b : UInt8) :
    obs (primitive .bitXor [Value.u8 a, Value.u8 b]) = some (Rel true (Value.u8 (LexLeanRuntime.bitXor a b))) := rfl
theorem prim_bitNot_u8 (a : UInt8) :
    obs (primitive .bitNot [Value.u8 a]) = some (Rel true (Value.u8 (LexLeanRuntime.bitNot a))) := rfl

theorem prim_shiftLeft_u8 (a : UInt8) (s : UInt32) :
    obs (primitive .shiftLeft [Value.u8 a, Value.u32 s]) =
      some (Rel true (encOption Value.u8 (LexLeanRuntime.shiftLeft a s : Option UInt8))) := by
  show obs (match (LexLeanRuntime.shiftLeft a s : Option UInt8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u8 r)) 0) = _
  cases (LexLeanRuntime.shiftLeft a s : Option UInt8) <;> rfl

theorem prim_shiftRight_u8 (a : UInt8) (s : UInt32) :
    obs (primitive .shiftRight [Value.u8 a, Value.u32 s]) =
      some (Rel true (encOption Value.u8 (LexLeanRuntime.shiftRight a s : Option UInt8))) := by
  show obs (match (LexLeanRuntime.shiftRight a s : Option UInt8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u8 r)) 0) = _
  cases (LexLeanRuntime.shiftRight a s : Option UInt8) <;> rfl

theorem prim_equal_u8 (a b : UInt8) :
    obs (primitive .equal [Value.u8 a, Value.u8 b]) = some (Rel true (.bool (LexLeanRuntime.equal a b))) := rfl
theorem prim_formatDecimal_u8 (a : UInt8) :
    obs (primitive .formatDecimal [Value.u8 a]) = some (Rel true (.string (LexLeanRuntime.formatDecimal a))) := rfl
theorem prim_parseDecimal_u8 (s : String) :
    obs (primitive (.parseDecimal (.fixed .u8)) [Value.string s]) =
      some (Rel true (encOption Value.u8 (LexLeanRuntime.parseDecimal s : Option UInt8))) := by
  show obs (match (LexLeanRuntime.parseDecimal s : Option UInt8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u8 r)) 0) = _
  cases (LexLeanRuntime.parseDecimal s : Option UInt8) <;> rfl

theorem prim_convert_int_u8 (a : Int) :
    obs (primitive (.convert .u8) [Value.int a]) =
      some (Rel true (encOption Value.u8 (LexLeanRuntime.checkedConvert a : Option UInt8))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u8 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt8) <;> rfl

theorem prim_convert_u8_u8 (a : UInt8) :
    obs (primitive (.convert .u8) [Value.u8 a]) =
      some (Rel true (encOption Value.u8 (LexLeanRuntime.checkedConvert a : Option UInt8))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u8 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt8) <;> rfl

theorem prim_convert_u16_u8 (a : UInt16) :
    obs (primitive (.convert .u8) [Value.u16 a]) =
      some (Rel true (encOption Value.u8 (LexLeanRuntime.checkedConvert a : Option UInt8))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u8 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt8) <;> rfl

theorem prim_convert_u32_u8 (a : UInt32) :
    obs (primitive (.convert .u8) [Value.u32 a]) =
      some (Rel true (encOption Value.u8 (LexLeanRuntime.checkedConvert a : Option UInt8))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u8 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt8) <;> rfl

theorem prim_convert_u64_u8 (a : UInt64) :
    obs (primitive (.convert .u8) [Value.u64 a]) =
      some (Rel true (encOption Value.u8 (LexLeanRuntime.checkedConvert a : Option UInt8))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u8 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt8) <;> rfl

theorem prim_convert_i8_u8 (a : Int8) :
    obs (primitive (.convert .u8) [Value.i8 a]) =
      some (Rel true (encOption Value.u8 (LexLeanRuntime.checkedConvert a : Option UInt8))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u8 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt8) <;> rfl

theorem prim_convert_i16_u8 (a : Int16) :
    obs (primitive (.convert .u8) [Value.i16 a]) =
      some (Rel true (encOption Value.u8 (LexLeanRuntime.checkedConvert a : Option UInt8))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u8 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt8) <;> rfl

theorem prim_convert_i32_u8 (a : Int32) :
    obs (primitive (.convert .u8) [Value.i32 a]) =
      some (Rel true (encOption Value.u8 (LexLeanRuntime.checkedConvert a : Option UInt8))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u8 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt8) <;> rfl

theorem prim_convert_i64_u8 (a : Int64) :
    obs (primitive (.convert .u8) [Value.i64 a]) =
      some (Rel true (encOption Value.u8 (LexLeanRuntime.checkedConvert a : Option UInt8))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u8 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt8) <;> rfl

theorem prim_checkedAdd_u16 (a : UInt16) (b : UInt16) :
    obs (primitive .checkedAdd [Value.u16 a, Value.u16 b]) =
      some (Rel true (encOption Value.u16 (LexLeanRuntime.checkedAdd a b : Option UInt16))) := by
  show obs (match (LexLeanRuntime.checkedAdd a b : Option UInt16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u16 r)) 0) = _
  cases (LexLeanRuntime.checkedAdd a b : Option UInt16) <;> rfl

theorem prim_checkedSub_u16 (a : UInt16) (b : UInt16) :
    obs (primitive .checkedSub [Value.u16 a, Value.u16 b]) =
      some (Rel true (encOption Value.u16 (LexLeanRuntime.checkedSubtract a b : Option UInt16))) := by
  show obs (match (LexLeanRuntime.checkedSubtract a b : Option UInt16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u16 r)) 0) = _
  cases (LexLeanRuntime.checkedSubtract a b : Option UInt16) <;> rfl

theorem prim_checkedMul_u16 (a : UInt16) (b : UInt16) :
    obs (primitive .checkedMul [Value.u16 a, Value.u16 b]) =
      some (Rel true (encOption Value.u16 (LexLeanRuntime.checkedMultiply a b : Option UInt16))) := by
  show obs (match (LexLeanRuntime.checkedMultiply a b : Option UInt16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u16 r)) 0) = _
  cases (LexLeanRuntime.checkedMultiply a b : Option UInt16) <;> rfl

theorem prim_checkedQuot_u16 (a : UInt16) (b : UInt16) :
    obs (primitive .checkedQuot [Value.u16 a, Value.u16 b]) =
      some (Rel true (encOption Value.u16 (LexLeanRuntime.checkedQuotient a b : Option UInt16))) := by
  show obs (match (LexLeanRuntime.checkedQuotient a b : Option UInt16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u16 r)) 0) = _
  cases (LexLeanRuntime.checkedQuotient a b : Option UInt16) <;> rfl

theorem prim_bitAnd_u16 (a b : UInt16) :
    obs (primitive .bitAnd [Value.u16 a, Value.u16 b]) = some (Rel true (Value.u16 (LexLeanRuntime.bitAnd a b))) := rfl
theorem prim_bitOr_u16 (a b : UInt16) :
    obs (primitive .bitOr [Value.u16 a, Value.u16 b]) = some (Rel true (Value.u16 (LexLeanRuntime.bitOr a b))) := rfl
theorem prim_bitXor_u16 (a b : UInt16) :
    obs (primitive .bitXor [Value.u16 a, Value.u16 b]) = some (Rel true (Value.u16 (LexLeanRuntime.bitXor a b))) := rfl
theorem prim_bitNot_u16 (a : UInt16) :
    obs (primitive .bitNot [Value.u16 a]) = some (Rel true (Value.u16 (LexLeanRuntime.bitNot a))) := rfl

theorem prim_shiftLeft_u16 (a : UInt16) (s : UInt32) :
    obs (primitive .shiftLeft [Value.u16 a, Value.u32 s]) =
      some (Rel true (encOption Value.u16 (LexLeanRuntime.shiftLeft a s : Option UInt16))) := by
  show obs (match (LexLeanRuntime.shiftLeft a s : Option UInt16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u16 r)) 0) = _
  cases (LexLeanRuntime.shiftLeft a s : Option UInt16) <;> rfl

theorem prim_shiftRight_u16 (a : UInt16) (s : UInt32) :
    obs (primitive .shiftRight [Value.u16 a, Value.u32 s]) =
      some (Rel true (encOption Value.u16 (LexLeanRuntime.shiftRight a s : Option UInt16))) := by
  show obs (match (LexLeanRuntime.shiftRight a s : Option UInt16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u16 r)) 0) = _
  cases (LexLeanRuntime.shiftRight a s : Option UInt16) <;> rfl

theorem prim_equal_u16 (a b : UInt16) :
    obs (primitive .equal [Value.u16 a, Value.u16 b]) = some (Rel true (.bool (LexLeanRuntime.equal a b))) := rfl
theorem prim_formatDecimal_u16 (a : UInt16) :
    obs (primitive .formatDecimal [Value.u16 a]) = some (Rel true (.string (LexLeanRuntime.formatDecimal a))) := rfl
theorem prim_parseDecimal_u16 (s : String) :
    obs (primitive (.parseDecimal (.fixed .u16)) [Value.string s]) =
      some (Rel true (encOption Value.u16 (LexLeanRuntime.parseDecimal s : Option UInt16))) := by
  show obs (match (LexLeanRuntime.parseDecimal s : Option UInt16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u16 r)) 0) = _
  cases (LexLeanRuntime.parseDecimal s : Option UInt16) <;> rfl

theorem prim_convert_int_u16 (a : Int) :
    obs (primitive (.convert .u16) [Value.int a]) =
      some (Rel true (encOption Value.u16 (LexLeanRuntime.checkedConvert a : Option UInt16))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u16 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt16) <;> rfl

theorem prim_convert_u8_u16 (a : UInt8) :
    obs (primitive (.convert .u16) [Value.u8 a]) =
      some (Rel true (encOption Value.u16 (LexLeanRuntime.checkedConvert a : Option UInt16))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u16 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt16) <;> rfl

theorem prim_convert_u16_u16 (a : UInt16) :
    obs (primitive (.convert .u16) [Value.u16 a]) =
      some (Rel true (encOption Value.u16 (LexLeanRuntime.checkedConvert a : Option UInt16))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u16 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt16) <;> rfl

theorem prim_convert_u32_u16 (a : UInt32) :
    obs (primitive (.convert .u16) [Value.u32 a]) =
      some (Rel true (encOption Value.u16 (LexLeanRuntime.checkedConvert a : Option UInt16))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u16 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt16) <;> rfl

theorem prim_convert_u64_u16 (a : UInt64) :
    obs (primitive (.convert .u16) [Value.u64 a]) =
      some (Rel true (encOption Value.u16 (LexLeanRuntime.checkedConvert a : Option UInt16))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u16 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt16) <;> rfl

theorem prim_convert_i8_u16 (a : Int8) :
    obs (primitive (.convert .u16) [Value.i8 a]) =
      some (Rel true (encOption Value.u16 (LexLeanRuntime.checkedConvert a : Option UInt16))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u16 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt16) <;> rfl

theorem prim_convert_i16_u16 (a : Int16) :
    obs (primitive (.convert .u16) [Value.i16 a]) =
      some (Rel true (encOption Value.u16 (LexLeanRuntime.checkedConvert a : Option UInt16))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u16 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt16) <;> rfl

theorem prim_convert_i32_u16 (a : Int32) :
    obs (primitive (.convert .u16) [Value.i32 a]) =
      some (Rel true (encOption Value.u16 (LexLeanRuntime.checkedConvert a : Option UInt16))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u16 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt16) <;> rfl

theorem prim_convert_i64_u16 (a : Int64) :
    obs (primitive (.convert .u16) [Value.i64 a]) =
      some (Rel true (encOption Value.u16 (LexLeanRuntime.checkedConvert a : Option UInt16))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u16 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt16) <;> rfl

theorem prim_checkedAdd_u32 (a : UInt32) (b : UInt32) :
    obs (primitive .checkedAdd [Value.u32 a, Value.u32 b]) =
      some (Rel true (encOption Value.u32 (LexLeanRuntime.checkedAdd a b : Option UInt32))) := by
  show obs (match (LexLeanRuntime.checkedAdd a b : Option UInt32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u32 r)) 0) = _
  cases (LexLeanRuntime.checkedAdd a b : Option UInt32) <;> rfl

theorem prim_checkedSub_u32 (a : UInt32) (b : UInt32) :
    obs (primitive .checkedSub [Value.u32 a, Value.u32 b]) =
      some (Rel true (encOption Value.u32 (LexLeanRuntime.checkedSubtract a b : Option UInt32))) := by
  show obs (match (LexLeanRuntime.checkedSubtract a b : Option UInt32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u32 r)) 0) = _
  cases (LexLeanRuntime.checkedSubtract a b : Option UInt32) <;> rfl

theorem prim_checkedMul_u32 (a : UInt32) (b : UInt32) :
    obs (primitive .checkedMul [Value.u32 a, Value.u32 b]) =
      some (Rel true (encOption Value.u32 (LexLeanRuntime.checkedMultiply a b : Option UInt32))) := by
  show obs (match (LexLeanRuntime.checkedMultiply a b : Option UInt32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u32 r)) 0) = _
  cases (LexLeanRuntime.checkedMultiply a b : Option UInt32) <;> rfl

theorem prim_checkedQuot_u32 (a : UInt32) (b : UInt32) :
    obs (primitive .checkedQuot [Value.u32 a, Value.u32 b]) =
      some (Rel true (encOption Value.u32 (LexLeanRuntime.checkedQuotient a b : Option UInt32))) := by
  show obs (match (LexLeanRuntime.checkedQuotient a b : Option UInt32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u32 r)) 0) = _
  cases (LexLeanRuntime.checkedQuotient a b : Option UInt32) <;> rfl

theorem prim_bitAnd_u32 (a b : UInt32) :
    obs (primitive .bitAnd [Value.u32 a, Value.u32 b]) = some (Rel true (Value.u32 (LexLeanRuntime.bitAnd a b))) := rfl
theorem prim_bitOr_u32 (a b : UInt32) :
    obs (primitive .bitOr [Value.u32 a, Value.u32 b]) = some (Rel true (Value.u32 (LexLeanRuntime.bitOr a b))) := rfl
theorem prim_bitXor_u32 (a b : UInt32) :
    obs (primitive .bitXor [Value.u32 a, Value.u32 b]) = some (Rel true (Value.u32 (LexLeanRuntime.bitXor a b))) := rfl
theorem prim_bitNot_u32 (a : UInt32) :
    obs (primitive .bitNot [Value.u32 a]) = some (Rel true (Value.u32 (LexLeanRuntime.bitNot a))) := rfl

theorem prim_shiftLeft_u32 (a : UInt32) (s : UInt32) :
    obs (primitive .shiftLeft [Value.u32 a, Value.u32 s]) =
      some (Rel true (encOption Value.u32 (LexLeanRuntime.shiftLeft a s : Option UInt32))) := by
  show obs (match (LexLeanRuntime.shiftLeft a s : Option UInt32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u32 r)) 0) = _
  cases (LexLeanRuntime.shiftLeft a s : Option UInt32) <;> rfl

theorem prim_shiftRight_u32 (a : UInt32) (s : UInt32) :
    obs (primitive .shiftRight [Value.u32 a, Value.u32 s]) =
      some (Rel true (encOption Value.u32 (LexLeanRuntime.shiftRight a s : Option UInt32))) := by
  show obs (match (LexLeanRuntime.shiftRight a s : Option UInt32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u32 r)) 0) = _
  cases (LexLeanRuntime.shiftRight a s : Option UInt32) <;> rfl

theorem prim_equal_u32 (a b : UInt32) :
    obs (primitive .equal [Value.u32 a, Value.u32 b]) = some (Rel true (.bool (LexLeanRuntime.equal a b))) := rfl
theorem prim_formatDecimal_u32 (a : UInt32) :
    obs (primitive .formatDecimal [Value.u32 a]) = some (Rel true (.string (LexLeanRuntime.formatDecimal a))) := rfl
theorem prim_parseDecimal_u32 (s : String) :
    obs (primitive (.parseDecimal (.fixed .u32)) [Value.string s]) =
      some (Rel true (encOption Value.u32 (LexLeanRuntime.parseDecimal s : Option UInt32))) := by
  show obs (match (LexLeanRuntime.parseDecimal s : Option UInt32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u32 r)) 0) = _
  cases (LexLeanRuntime.parseDecimal s : Option UInt32) <;> rfl

theorem prim_convert_int_u32 (a : Int) :
    obs (primitive (.convert .u32) [Value.int a]) =
      some (Rel true (encOption Value.u32 (LexLeanRuntime.checkedConvert a : Option UInt32))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u32 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt32) <;> rfl

theorem prim_convert_u8_u32 (a : UInt8) :
    obs (primitive (.convert .u32) [Value.u8 a]) =
      some (Rel true (encOption Value.u32 (LexLeanRuntime.checkedConvert a : Option UInt32))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u32 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt32) <;> rfl

theorem prim_convert_u16_u32 (a : UInt16) :
    obs (primitive (.convert .u32) [Value.u16 a]) =
      some (Rel true (encOption Value.u32 (LexLeanRuntime.checkedConvert a : Option UInt32))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u32 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt32) <;> rfl

theorem prim_convert_u32_u32 (a : UInt32) :
    obs (primitive (.convert .u32) [Value.u32 a]) =
      some (Rel true (encOption Value.u32 (LexLeanRuntime.checkedConvert a : Option UInt32))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u32 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt32) <;> rfl

theorem prim_convert_u64_u32 (a : UInt64) :
    obs (primitive (.convert .u32) [Value.u64 a]) =
      some (Rel true (encOption Value.u32 (LexLeanRuntime.checkedConvert a : Option UInt32))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u32 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt32) <;> rfl

theorem prim_convert_i8_u32 (a : Int8) :
    obs (primitive (.convert .u32) [Value.i8 a]) =
      some (Rel true (encOption Value.u32 (LexLeanRuntime.checkedConvert a : Option UInt32))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u32 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt32) <;> rfl

theorem prim_convert_i16_u32 (a : Int16) :
    obs (primitive (.convert .u32) [Value.i16 a]) =
      some (Rel true (encOption Value.u32 (LexLeanRuntime.checkedConvert a : Option UInt32))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u32 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt32) <;> rfl

theorem prim_convert_i32_u32 (a : Int32) :
    obs (primitive (.convert .u32) [Value.i32 a]) =
      some (Rel true (encOption Value.u32 (LexLeanRuntime.checkedConvert a : Option UInt32))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u32 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt32) <;> rfl

theorem prim_convert_i64_u32 (a : Int64) :
    obs (primitive (.convert .u32) [Value.i64 a]) =
      some (Rel true (encOption Value.u32 (LexLeanRuntime.checkedConvert a : Option UInt32))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u32 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt32) <;> rfl

theorem prim_checkedAdd_u64 (a : UInt64) (b : UInt64) :
    obs (primitive .checkedAdd [Value.u64 a, Value.u64 b]) =
      some (Rel true (encOption Value.u64 (LexLeanRuntime.checkedAdd a b : Option UInt64))) := by
  show obs (match (LexLeanRuntime.checkedAdd a b : Option UInt64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u64 r)) 0) = _
  cases (LexLeanRuntime.checkedAdd a b : Option UInt64) <;> rfl

theorem prim_checkedSub_u64 (a : UInt64) (b : UInt64) :
    obs (primitive .checkedSub [Value.u64 a, Value.u64 b]) =
      some (Rel true (encOption Value.u64 (LexLeanRuntime.checkedSubtract a b : Option UInt64))) := by
  show obs (match (LexLeanRuntime.checkedSubtract a b : Option UInt64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u64 r)) 0) = _
  cases (LexLeanRuntime.checkedSubtract a b : Option UInt64) <;> rfl

theorem prim_checkedMul_u64 (a : UInt64) (b : UInt64) :
    obs (primitive .checkedMul [Value.u64 a, Value.u64 b]) =
      some (Rel true (encOption Value.u64 (LexLeanRuntime.checkedMultiply a b : Option UInt64))) := by
  show obs (match (LexLeanRuntime.checkedMultiply a b : Option UInt64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u64 r)) 0) = _
  cases (LexLeanRuntime.checkedMultiply a b : Option UInt64) <;> rfl

theorem prim_checkedQuot_u64 (a : UInt64) (b : UInt64) :
    obs (primitive .checkedQuot [Value.u64 a, Value.u64 b]) =
      some (Rel true (encOption Value.u64 (LexLeanRuntime.checkedQuotient a b : Option UInt64))) := by
  show obs (match (LexLeanRuntime.checkedQuotient a b : Option UInt64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u64 r)) 0) = _
  cases (LexLeanRuntime.checkedQuotient a b : Option UInt64) <;> rfl

theorem prim_bitAnd_u64 (a b : UInt64) :
    obs (primitive .bitAnd [Value.u64 a, Value.u64 b]) = some (Rel true (Value.u64 (LexLeanRuntime.bitAnd a b))) := rfl
theorem prim_bitOr_u64 (a b : UInt64) :
    obs (primitive .bitOr [Value.u64 a, Value.u64 b]) = some (Rel true (Value.u64 (LexLeanRuntime.bitOr a b))) := rfl
theorem prim_bitXor_u64 (a b : UInt64) :
    obs (primitive .bitXor [Value.u64 a, Value.u64 b]) = some (Rel true (Value.u64 (LexLeanRuntime.bitXor a b))) := rfl
theorem prim_bitNot_u64 (a : UInt64) :
    obs (primitive .bitNot [Value.u64 a]) = some (Rel true (Value.u64 (LexLeanRuntime.bitNot a))) := rfl

theorem prim_shiftLeft_u64 (a : UInt64) (s : UInt32) :
    obs (primitive .shiftLeft [Value.u64 a, Value.u32 s]) =
      some (Rel true (encOption Value.u64 (LexLeanRuntime.shiftLeft a s : Option UInt64))) := by
  show obs (match (LexLeanRuntime.shiftLeft a s : Option UInt64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u64 r)) 0) = _
  cases (LexLeanRuntime.shiftLeft a s : Option UInt64) <;> rfl

theorem prim_shiftRight_u64 (a : UInt64) (s : UInt32) :
    obs (primitive .shiftRight [Value.u64 a, Value.u32 s]) =
      some (Rel true (encOption Value.u64 (LexLeanRuntime.shiftRight a s : Option UInt64))) := by
  show obs (match (LexLeanRuntime.shiftRight a s : Option UInt64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u64 r)) 0) = _
  cases (LexLeanRuntime.shiftRight a s : Option UInt64) <;> rfl

theorem prim_equal_u64 (a b : UInt64) :
    obs (primitive .equal [Value.u64 a, Value.u64 b]) = some (Rel true (.bool (LexLeanRuntime.equal a b))) := rfl
theorem prim_formatDecimal_u64 (a : UInt64) :
    obs (primitive .formatDecimal [Value.u64 a]) = some (Rel true (.string (LexLeanRuntime.formatDecimal a))) := rfl
theorem prim_parseDecimal_u64 (s : String) :
    obs (primitive (.parseDecimal (.fixed .u64)) [Value.string s]) =
      some (Rel true (encOption Value.u64 (LexLeanRuntime.parseDecimal s : Option UInt64))) := by
  show obs (match (LexLeanRuntime.parseDecimal s : Option UInt64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u64 r)) 0) = _
  cases (LexLeanRuntime.parseDecimal s : Option UInt64) <;> rfl

theorem prim_convert_int_u64 (a : Int) :
    obs (primitive (.convert .u64) [Value.int a]) =
      some (Rel true (encOption Value.u64 (LexLeanRuntime.checkedConvert a : Option UInt64))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u64 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt64) <;> rfl

theorem prim_convert_u8_u64 (a : UInt8) :
    obs (primitive (.convert .u64) [Value.u8 a]) =
      some (Rel true (encOption Value.u64 (LexLeanRuntime.checkedConvert a : Option UInt64))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u64 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt64) <;> rfl

theorem prim_convert_u16_u64 (a : UInt16) :
    obs (primitive (.convert .u64) [Value.u16 a]) =
      some (Rel true (encOption Value.u64 (LexLeanRuntime.checkedConvert a : Option UInt64))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u64 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt64) <;> rfl

theorem prim_convert_u32_u64 (a : UInt32) :
    obs (primitive (.convert .u64) [Value.u32 a]) =
      some (Rel true (encOption Value.u64 (LexLeanRuntime.checkedConvert a : Option UInt64))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u64 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt64) <;> rfl

theorem prim_convert_u64_u64 (a : UInt64) :
    obs (primitive (.convert .u64) [Value.u64 a]) =
      some (Rel true (encOption Value.u64 (LexLeanRuntime.checkedConvert a : Option UInt64))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u64 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt64) <;> rfl

theorem prim_convert_i8_u64 (a : Int8) :
    obs (primitive (.convert .u64) [Value.i8 a]) =
      some (Rel true (encOption Value.u64 (LexLeanRuntime.checkedConvert a : Option UInt64))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u64 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt64) <;> rfl

theorem prim_convert_i16_u64 (a : Int16) :
    obs (primitive (.convert .u64) [Value.i16 a]) =
      some (Rel true (encOption Value.u64 (LexLeanRuntime.checkedConvert a : Option UInt64))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u64 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt64) <;> rfl

theorem prim_convert_i32_u64 (a : Int32) :
    obs (primitive (.convert .u64) [Value.i32 a]) =
      some (Rel true (encOption Value.u64 (LexLeanRuntime.checkedConvert a : Option UInt64))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u64 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt64) <;> rfl

theorem prim_convert_i64_u64 (a : Int64) :
    obs (primitive (.convert .u64) [Value.i64 a]) =
      some (Rel true (encOption Value.u64 (LexLeanRuntime.checkedConvert a : Option UInt64))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option UInt64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.u64 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option UInt64) <;> rfl

theorem prim_checkedAdd_i8 (a : Int8) (b : Int8) :
    obs (primitive .checkedAdd [Value.i8 a, Value.i8 b]) =
      some (Rel true (encOption Value.i8 (LexLeanRuntime.checkedAdd a b : Option Int8))) := by
  show obs (match (LexLeanRuntime.checkedAdd a b : Option Int8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i8 r)) 0) = _
  cases (LexLeanRuntime.checkedAdd a b : Option Int8) <;> rfl

theorem prim_checkedSub_i8 (a : Int8) (b : Int8) :
    obs (primitive .checkedSub [Value.i8 a, Value.i8 b]) =
      some (Rel true (encOption Value.i8 (LexLeanRuntime.checkedSubtract a b : Option Int8))) := by
  show obs (match (LexLeanRuntime.checkedSubtract a b : Option Int8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i8 r)) 0) = _
  cases (LexLeanRuntime.checkedSubtract a b : Option Int8) <;> rfl

theorem prim_checkedMul_i8 (a : Int8) (b : Int8) :
    obs (primitive .checkedMul [Value.i8 a, Value.i8 b]) =
      some (Rel true (encOption Value.i8 (LexLeanRuntime.checkedMultiply a b : Option Int8))) := by
  show obs (match (LexLeanRuntime.checkedMultiply a b : Option Int8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i8 r)) 0) = _
  cases (LexLeanRuntime.checkedMultiply a b : Option Int8) <;> rfl

theorem prim_checkedQuot_i8 (a : Int8) (b : Int8) :
    obs (primitive .checkedQuot [Value.i8 a, Value.i8 b]) =
      some (Rel true (encOption Value.i8 (LexLeanRuntime.checkedQuotient a b : Option Int8))) := by
  show obs (match (LexLeanRuntime.checkedQuotient a b : Option Int8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i8 r)) 0) = _
  cases (LexLeanRuntime.checkedQuotient a b : Option Int8) <;> rfl

theorem prim_checkedNeg_i8 (a : Int8) :
    obs (primitive .checkedNeg [Value.i8 a]) =
      some (Rel true (encOption Value.i8 (LexLeanRuntime.checkedNegate a : Option Int8))) := by
  show obs (match (LexLeanRuntime.checkedNegate a : Option Int8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i8 r)) 0) = _
  cases (LexLeanRuntime.checkedNegate a : Option Int8) <;> rfl

theorem prim_bitAnd_i8 (a b : Int8) :
    obs (primitive .bitAnd [Value.i8 a, Value.i8 b]) = some (Rel true (Value.i8 (LexLeanRuntime.bitAnd a b))) := rfl
theorem prim_bitOr_i8 (a b : Int8) :
    obs (primitive .bitOr [Value.i8 a, Value.i8 b]) = some (Rel true (Value.i8 (LexLeanRuntime.bitOr a b))) := rfl
theorem prim_bitXor_i8 (a b : Int8) :
    obs (primitive .bitXor [Value.i8 a, Value.i8 b]) = some (Rel true (Value.i8 (LexLeanRuntime.bitXor a b))) := rfl
theorem prim_bitNot_i8 (a : Int8) :
    obs (primitive .bitNot [Value.i8 a]) = some (Rel true (Value.i8 (LexLeanRuntime.bitNot a))) := rfl

theorem prim_shiftLeft_i8 (a : Int8) (s : UInt32) :
    obs (primitive .shiftLeft [Value.i8 a, Value.u32 s]) =
      some (Rel true (encOption Value.i8 (LexLeanRuntime.shiftLeft a s : Option Int8))) := by
  show obs (match (LexLeanRuntime.shiftLeft a s : Option Int8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i8 r)) 0) = _
  cases (LexLeanRuntime.shiftLeft a s : Option Int8) <;> rfl

theorem prim_shiftRight_i8 (a : Int8) (s : UInt32) :
    obs (primitive .shiftRight [Value.i8 a, Value.u32 s]) =
      some (Rel true (encOption Value.i8 (LexLeanRuntime.shiftRight a s : Option Int8))) := by
  show obs (match (LexLeanRuntime.shiftRight a s : Option Int8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i8 r)) 0) = _
  cases (LexLeanRuntime.shiftRight a s : Option Int8) <;> rfl

theorem prim_equal_i8 (a b : Int8) :
    obs (primitive .equal [Value.i8 a, Value.i8 b]) = some (Rel true (.bool (LexLeanRuntime.equal a b))) := rfl
theorem prim_formatDecimal_i8 (a : Int8) :
    obs (primitive .formatDecimal [Value.i8 a]) = some (Rel true (.string (LexLeanRuntime.formatDecimal a))) := rfl
theorem prim_parseDecimal_i8 (s : String) :
    obs (primitive (.parseDecimal (.fixed .i8)) [Value.string s]) =
      some (Rel true (encOption Value.i8 (LexLeanRuntime.parseDecimal s : Option Int8))) := by
  show obs (match (LexLeanRuntime.parseDecimal s : Option Int8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i8 r)) 0) = _
  cases (LexLeanRuntime.parseDecimal s : Option Int8) <;> rfl

theorem prim_convert_int_i8 (a : Int) :
    obs (primitive (.convert .i8) [Value.int a]) =
      some (Rel true (encOption Value.i8 (LexLeanRuntime.checkedConvert a : Option Int8))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i8 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int8) <;> rfl

theorem prim_convert_u8_i8 (a : UInt8) :
    obs (primitive (.convert .i8) [Value.u8 a]) =
      some (Rel true (encOption Value.i8 (LexLeanRuntime.checkedConvert a : Option Int8))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i8 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int8) <;> rfl

theorem prim_convert_u16_i8 (a : UInt16) :
    obs (primitive (.convert .i8) [Value.u16 a]) =
      some (Rel true (encOption Value.i8 (LexLeanRuntime.checkedConvert a : Option Int8))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i8 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int8) <;> rfl

theorem prim_convert_u32_i8 (a : UInt32) :
    obs (primitive (.convert .i8) [Value.u32 a]) =
      some (Rel true (encOption Value.i8 (LexLeanRuntime.checkedConvert a : Option Int8))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i8 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int8) <;> rfl

theorem prim_convert_u64_i8 (a : UInt64) :
    obs (primitive (.convert .i8) [Value.u64 a]) =
      some (Rel true (encOption Value.i8 (LexLeanRuntime.checkedConvert a : Option Int8))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i8 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int8) <;> rfl

theorem prim_convert_i8_i8 (a : Int8) :
    obs (primitive (.convert .i8) [Value.i8 a]) =
      some (Rel true (encOption Value.i8 (LexLeanRuntime.checkedConvert a : Option Int8))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i8 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int8) <;> rfl

theorem prim_convert_i16_i8 (a : Int16) :
    obs (primitive (.convert .i8) [Value.i16 a]) =
      some (Rel true (encOption Value.i8 (LexLeanRuntime.checkedConvert a : Option Int8))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i8 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int8) <;> rfl

theorem prim_convert_i32_i8 (a : Int32) :
    obs (primitive (.convert .i8) [Value.i32 a]) =
      some (Rel true (encOption Value.i8 (LexLeanRuntime.checkedConvert a : Option Int8))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i8 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int8) <;> rfl

theorem prim_convert_i64_i8 (a : Int64) :
    obs (primitive (.convert .i8) [Value.i64 a]) =
      some (Rel true (encOption Value.i8 (LexLeanRuntime.checkedConvert a : Option Int8))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int8) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i8 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int8) <;> rfl

theorem prim_checkedAdd_i16 (a : Int16) (b : Int16) :
    obs (primitive .checkedAdd [Value.i16 a, Value.i16 b]) =
      some (Rel true (encOption Value.i16 (LexLeanRuntime.checkedAdd a b : Option Int16))) := by
  show obs (match (LexLeanRuntime.checkedAdd a b : Option Int16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i16 r)) 0) = _
  cases (LexLeanRuntime.checkedAdd a b : Option Int16) <;> rfl

theorem prim_checkedSub_i16 (a : Int16) (b : Int16) :
    obs (primitive .checkedSub [Value.i16 a, Value.i16 b]) =
      some (Rel true (encOption Value.i16 (LexLeanRuntime.checkedSubtract a b : Option Int16))) := by
  show obs (match (LexLeanRuntime.checkedSubtract a b : Option Int16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i16 r)) 0) = _
  cases (LexLeanRuntime.checkedSubtract a b : Option Int16) <;> rfl

theorem prim_checkedMul_i16 (a : Int16) (b : Int16) :
    obs (primitive .checkedMul [Value.i16 a, Value.i16 b]) =
      some (Rel true (encOption Value.i16 (LexLeanRuntime.checkedMultiply a b : Option Int16))) := by
  show obs (match (LexLeanRuntime.checkedMultiply a b : Option Int16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i16 r)) 0) = _
  cases (LexLeanRuntime.checkedMultiply a b : Option Int16) <;> rfl

theorem prim_checkedQuot_i16 (a : Int16) (b : Int16) :
    obs (primitive .checkedQuot [Value.i16 a, Value.i16 b]) =
      some (Rel true (encOption Value.i16 (LexLeanRuntime.checkedQuotient a b : Option Int16))) := by
  show obs (match (LexLeanRuntime.checkedQuotient a b : Option Int16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i16 r)) 0) = _
  cases (LexLeanRuntime.checkedQuotient a b : Option Int16) <;> rfl

theorem prim_checkedNeg_i16 (a : Int16) :
    obs (primitive .checkedNeg [Value.i16 a]) =
      some (Rel true (encOption Value.i16 (LexLeanRuntime.checkedNegate a : Option Int16))) := by
  show obs (match (LexLeanRuntime.checkedNegate a : Option Int16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i16 r)) 0) = _
  cases (LexLeanRuntime.checkedNegate a : Option Int16) <;> rfl

theorem prim_bitAnd_i16 (a b : Int16) :
    obs (primitive .bitAnd [Value.i16 a, Value.i16 b]) = some (Rel true (Value.i16 (LexLeanRuntime.bitAnd a b))) := rfl
theorem prim_bitOr_i16 (a b : Int16) :
    obs (primitive .bitOr [Value.i16 a, Value.i16 b]) = some (Rel true (Value.i16 (LexLeanRuntime.bitOr a b))) := rfl
theorem prim_bitXor_i16 (a b : Int16) :
    obs (primitive .bitXor [Value.i16 a, Value.i16 b]) = some (Rel true (Value.i16 (LexLeanRuntime.bitXor a b))) := rfl
theorem prim_bitNot_i16 (a : Int16) :
    obs (primitive .bitNot [Value.i16 a]) = some (Rel true (Value.i16 (LexLeanRuntime.bitNot a))) := rfl

theorem prim_shiftLeft_i16 (a : Int16) (s : UInt32) :
    obs (primitive .shiftLeft [Value.i16 a, Value.u32 s]) =
      some (Rel true (encOption Value.i16 (LexLeanRuntime.shiftLeft a s : Option Int16))) := by
  show obs (match (LexLeanRuntime.shiftLeft a s : Option Int16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i16 r)) 0) = _
  cases (LexLeanRuntime.shiftLeft a s : Option Int16) <;> rfl

theorem prim_shiftRight_i16 (a : Int16) (s : UInt32) :
    obs (primitive .shiftRight [Value.i16 a, Value.u32 s]) =
      some (Rel true (encOption Value.i16 (LexLeanRuntime.shiftRight a s : Option Int16))) := by
  show obs (match (LexLeanRuntime.shiftRight a s : Option Int16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i16 r)) 0) = _
  cases (LexLeanRuntime.shiftRight a s : Option Int16) <;> rfl

theorem prim_equal_i16 (a b : Int16) :
    obs (primitive .equal [Value.i16 a, Value.i16 b]) = some (Rel true (.bool (LexLeanRuntime.equal a b))) := rfl
theorem prim_formatDecimal_i16 (a : Int16) :
    obs (primitive .formatDecimal [Value.i16 a]) = some (Rel true (.string (LexLeanRuntime.formatDecimal a))) := rfl
theorem prim_parseDecimal_i16 (s : String) :
    obs (primitive (.parseDecimal (.fixed .i16)) [Value.string s]) =
      some (Rel true (encOption Value.i16 (LexLeanRuntime.parseDecimal s : Option Int16))) := by
  show obs (match (LexLeanRuntime.parseDecimal s : Option Int16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i16 r)) 0) = _
  cases (LexLeanRuntime.parseDecimal s : Option Int16) <;> rfl

theorem prim_convert_int_i16 (a : Int) :
    obs (primitive (.convert .i16) [Value.int a]) =
      some (Rel true (encOption Value.i16 (LexLeanRuntime.checkedConvert a : Option Int16))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i16 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int16) <;> rfl

theorem prim_convert_u8_i16 (a : UInt8) :
    obs (primitive (.convert .i16) [Value.u8 a]) =
      some (Rel true (encOption Value.i16 (LexLeanRuntime.checkedConvert a : Option Int16))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i16 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int16) <;> rfl

theorem prim_convert_u16_i16 (a : UInt16) :
    obs (primitive (.convert .i16) [Value.u16 a]) =
      some (Rel true (encOption Value.i16 (LexLeanRuntime.checkedConvert a : Option Int16))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i16 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int16) <;> rfl

theorem prim_convert_u32_i16 (a : UInt32) :
    obs (primitive (.convert .i16) [Value.u32 a]) =
      some (Rel true (encOption Value.i16 (LexLeanRuntime.checkedConvert a : Option Int16))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i16 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int16) <;> rfl

theorem prim_convert_u64_i16 (a : UInt64) :
    obs (primitive (.convert .i16) [Value.u64 a]) =
      some (Rel true (encOption Value.i16 (LexLeanRuntime.checkedConvert a : Option Int16))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i16 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int16) <;> rfl

theorem prim_convert_i8_i16 (a : Int8) :
    obs (primitive (.convert .i16) [Value.i8 a]) =
      some (Rel true (encOption Value.i16 (LexLeanRuntime.checkedConvert a : Option Int16))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i16 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int16) <;> rfl

theorem prim_convert_i16_i16 (a : Int16) :
    obs (primitive (.convert .i16) [Value.i16 a]) =
      some (Rel true (encOption Value.i16 (LexLeanRuntime.checkedConvert a : Option Int16))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i16 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int16) <;> rfl

theorem prim_convert_i32_i16 (a : Int32) :
    obs (primitive (.convert .i16) [Value.i32 a]) =
      some (Rel true (encOption Value.i16 (LexLeanRuntime.checkedConvert a : Option Int16))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i16 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int16) <;> rfl

theorem prim_convert_i64_i16 (a : Int64) :
    obs (primitive (.convert .i16) [Value.i64 a]) =
      some (Rel true (encOption Value.i16 (LexLeanRuntime.checkedConvert a : Option Int16))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int16) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i16 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int16) <;> rfl

theorem prim_checkedAdd_i32 (a : Int32) (b : Int32) :
    obs (primitive .checkedAdd [Value.i32 a, Value.i32 b]) =
      some (Rel true (encOption Value.i32 (LexLeanRuntime.checkedAdd a b : Option Int32))) := by
  show obs (match (LexLeanRuntime.checkedAdd a b : Option Int32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i32 r)) 0) = _
  cases (LexLeanRuntime.checkedAdd a b : Option Int32) <;> rfl

theorem prim_checkedSub_i32 (a : Int32) (b : Int32) :
    obs (primitive .checkedSub [Value.i32 a, Value.i32 b]) =
      some (Rel true (encOption Value.i32 (LexLeanRuntime.checkedSubtract a b : Option Int32))) := by
  show obs (match (LexLeanRuntime.checkedSubtract a b : Option Int32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i32 r)) 0) = _
  cases (LexLeanRuntime.checkedSubtract a b : Option Int32) <;> rfl

theorem prim_checkedMul_i32 (a : Int32) (b : Int32) :
    obs (primitive .checkedMul [Value.i32 a, Value.i32 b]) =
      some (Rel true (encOption Value.i32 (LexLeanRuntime.checkedMultiply a b : Option Int32))) := by
  show obs (match (LexLeanRuntime.checkedMultiply a b : Option Int32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i32 r)) 0) = _
  cases (LexLeanRuntime.checkedMultiply a b : Option Int32) <;> rfl

theorem prim_checkedQuot_i32 (a : Int32) (b : Int32) :
    obs (primitive .checkedQuot [Value.i32 a, Value.i32 b]) =
      some (Rel true (encOption Value.i32 (LexLeanRuntime.checkedQuotient a b : Option Int32))) := by
  show obs (match (LexLeanRuntime.checkedQuotient a b : Option Int32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i32 r)) 0) = _
  cases (LexLeanRuntime.checkedQuotient a b : Option Int32) <;> rfl

theorem prim_checkedNeg_i32 (a : Int32) :
    obs (primitive .checkedNeg [Value.i32 a]) =
      some (Rel true (encOption Value.i32 (LexLeanRuntime.checkedNegate a : Option Int32))) := by
  show obs (match (LexLeanRuntime.checkedNegate a : Option Int32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i32 r)) 0) = _
  cases (LexLeanRuntime.checkedNegate a : Option Int32) <;> rfl

theorem prim_bitAnd_i32 (a b : Int32) :
    obs (primitive .bitAnd [Value.i32 a, Value.i32 b]) = some (Rel true (Value.i32 (LexLeanRuntime.bitAnd a b))) := rfl
theorem prim_bitOr_i32 (a b : Int32) :
    obs (primitive .bitOr [Value.i32 a, Value.i32 b]) = some (Rel true (Value.i32 (LexLeanRuntime.bitOr a b))) := rfl
theorem prim_bitXor_i32 (a b : Int32) :
    obs (primitive .bitXor [Value.i32 a, Value.i32 b]) = some (Rel true (Value.i32 (LexLeanRuntime.bitXor a b))) := rfl
theorem prim_bitNot_i32 (a : Int32) :
    obs (primitive .bitNot [Value.i32 a]) = some (Rel true (Value.i32 (LexLeanRuntime.bitNot a))) := rfl

theorem prim_shiftLeft_i32 (a : Int32) (s : UInt32) :
    obs (primitive .shiftLeft [Value.i32 a, Value.u32 s]) =
      some (Rel true (encOption Value.i32 (LexLeanRuntime.shiftLeft a s : Option Int32))) := by
  show obs (match (LexLeanRuntime.shiftLeft a s : Option Int32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i32 r)) 0) = _
  cases (LexLeanRuntime.shiftLeft a s : Option Int32) <;> rfl

theorem prim_shiftRight_i32 (a : Int32) (s : UInt32) :
    obs (primitive .shiftRight [Value.i32 a, Value.u32 s]) =
      some (Rel true (encOption Value.i32 (LexLeanRuntime.shiftRight a s : Option Int32))) := by
  show obs (match (LexLeanRuntime.shiftRight a s : Option Int32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i32 r)) 0) = _
  cases (LexLeanRuntime.shiftRight a s : Option Int32) <;> rfl

theorem prim_equal_i32 (a b : Int32) :
    obs (primitive .equal [Value.i32 a, Value.i32 b]) = some (Rel true (.bool (LexLeanRuntime.equal a b))) := rfl
theorem prim_formatDecimal_i32 (a : Int32) :
    obs (primitive .formatDecimal [Value.i32 a]) = some (Rel true (.string (LexLeanRuntime.formatDecimal a))) := rfl
theorem prim_parseDecimal_i32 (s : String) :
    obs (primitive (.parseDecimal (.fixed .i32)) [Value.string s]) =
      some (Rel true (encOption Value.i32 (LexLeanRuntime.parseDecimal s : Option Int32))) := by
  show obs (match (LexLeanRuntime.parseDecimal s : Option Int32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i32 r)) 0) = _
  cases (LexLeanRuntime.parseDecimal s : Option Int32) <;> rfl

theorem prim_convert_int_i32 (a : Int) :
    obs (primitive (.convert .i32) [Value.int a]) =
      some (Rel true (encOption Value.i32 (LexLeanRuntime.checkedConvert a : Option Int32))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i32 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int32) <;> rfl

theorem prim_convert_u8_i32 (a : UInt8) :
    obs (primitive (.convert .i32) [Value.u8 a]) =
      some (Rel true (encOption Value.i32 (LexLeanRuntime.checkedConvert a : Option Int32))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i32 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int32) <;> rfl

theorem prim_convert_u16_i32 (a : UInt16) :
    obs (primitive (.convert .i32) [Value.u16 a]) =
      some (Rel true (encOption Value.i32 (LexLeanRuntime.checkedConvert a : Option Int32))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i32 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int32) <;> rfl

theorem prim_convert_u32_i32 (a : UInt32) :
    obs (primitive (.convert .i32) [Value.u32 a]) =
      some (Rel true (encOption Value.i32 (LexLeanRuntime.checkedConvert a : Option Int32))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i32 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int32) <;> rfl

theorem prim_convert_u64_i32 (a : UInt64) :
    obs (primitive (.convert .i32) [Value.u64 a]) =
      some (Rel true (encOption Value.i32 (LexLeanRuntime.checkedConvert a : Option Int32))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i32 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int32) <;> rfl

theorem prim_convert_i8_i32 (a : Int8) :
    obs (primitive (.convert .i32) [Value.i8 a]) =
      some (Rel true (encOption Value.i32 (LexLeanRuntime.checkedConvert a : Option Int32))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i32 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int32) <;> rfl

theorem prim_convert_i16_i32 (a : Int16) :
    obs (primitive (.convert .i32) [Value.i16 a]) =
      some (Rel true (encOption Value.i32 (LexLeanRuntime.checkedConvert a : Option Int32))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i32 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int32) <;> rfl

theorem prim_convert_i32_i32 (a : Int32) :
    obs (primitive (.convert .i32) [Value.i32 a]) =
      some (Rel true (encOption Value.i32 (LexLeanRuntime.checkedConvert a : Option Int32))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i32 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int32) <;> rfl

theorem prim_convert_i64_i32 (a : Int64) :
    obs (primitive (.convert .i32) [Value.i64 a]) =
      some (Rel true (encOption Value.i32 (LexLeanRuntime.checkedConvert a : Option Int32))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int32) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i32 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int32) <;> rfl

theorem prim_checkedAdd_i64 (a : Int64) (b : Int64) :
    obs (primitive .checkedAdd [Value.i64 a, Value.i64 b]) =
      some (Rel true (encOption Value.i64 (LexLeanRuntime.checkedAddInt64 a b : Option Int64))) := by
  show obs (match (LexLeanRuntime.checkedAddInt64 a b : Option Int64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i64 r)) 0) = _
  cases (LexLeanRuntime.checkedAddInt64 a b : Option Int64) <;> rfl

theorem prim_checkedSub_i64 (a : Int64) (b : Int64) :
    obs (primitive .checkedSub [Value.i64 a, Value.i64 b]) =
      some (Rel true (encOption Value.i64 (LexLeanRuntime.checkedSubtractInt64 a b : Option Int64))) := by
  show obs (match (LexLeanRuntime.checkedSubtractInt64 a b : Option Int64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i64 r)) 0) = _
  cases (LexLeanRuntime.checkedSubtractInt64 a b : Option Int64) <;> rfl

theorem prim_checkedNeg_i64 (a : Int64) :
    obs (primitive .checkedNeg [Value.i64 a]) =
      some (Rel true (encOption Value.i64 (LexLeanRuntime.checkedNegateInt64 a : Option Int64))) := by
  show obs (match (LexLeanRuntime.checkedNegateInt64 a : Option Int64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i64 r)) 0) = _
  cases (LexLeanRuntime.checkedNegateInt64 a : Option Int64) <;> rfl

theorem prim_bitAnd_i64 (a b : Int64) :
    obs (primitive .bitAnd [Value.i64 a, Value.i64 b]) = some (Rel true (Value.i64 (LexLeanRuntime.bitAnd a b))) := rfl
theorem prim_bitOr_i64 (a b : Int64) :
    obs (primitive .bitOr [Value.i64 a, Value.i64 b]) = some (Rel true (Value.i64 (LexLeanRuntime.bitOr a b))) := rfl
theorem prim_bitXor_i64 (a b : Int64) :
    obs (primitive .bitXor [Value.i64 a, Value.i64 b]) = some (Rel true (Value.i64 (LexLeanRuntime.bitXor a b))) := rfl
theorem prim_bitNot_i64 (a : Int64) :
    obs (primitive .bitNot [Value.i64 a]) = some (Rel true (Value.i64 (LexLeanRuntime.bitNot a))) := rfl

theorem prim_shiftLeft_i64 (a : Int64) (s : UInt32) :
    obs (primitive .shiftLeft [Value.i64 a, Value.u32 s]) =
      some (Rel true (encOption Value.i64 (LexLeanRuntime.shiftLeft a s : Option Int64))) := by
  show obs (match (LexLeanRuntime.shiftLeft a s : Option Int64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i64 r)) 0) = _
  cases (LexLeanRuntime.shiftLeft a s : Option Int64) <;> rfl

theorem prim_shiftRight_i64 (a : Int64) (s : UInt32) :
    obs (primitive .shiftRight [Value.i64 a, Value.u32 s]) =
      some (Rel true (encOption Value.i64 (LexLeanRuntime.shiftRight a s : Option Int64))) := by
  show obs (match (LexLeanRuntime.shiftRight a s : Option Int64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i64 r)) 0) = _
  cases (LexLeanRuntime.shiftRight a s : Option Int64) <;> rfl

theorem prim_equal_i64 (a b : Int64) :
    obs (primitive .equal [Value.i64 a, Value.i64 b]) = some (Rel true (.bool (LexLeanRuntime.equal a b))) := rfl
theorem prim_formatDecimal_i64 (a : Int64) :
    obs (primitive .formatDecimal [Value.i64 a]) = some (Rel true (.string (LexLeanRuntime.formatDecimal a))) := rfl
theorem prim_parseDecimal_i64 (s : String) :
    obs (primitive (.parseDecimal (.fixed .i64)) [Value.string s]) =
      some (Rel true (encOption Value.i64 (LexLeanRuntime.parseDecimal s : Option Int64))) := by
  show obs (match (LexLeanRuntime.parseDecimal s : Option Int64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i64 r)) 0) = _
  cases (LexLeanRuntime.parseDecimal s : Option Int64) <;> rfl

theorem prim_convert_int_i64 (a : Int) :
    obs (primitive (.convert .i64) [Value.int a]) =
      some (Rel true (encOption Value.i64 (LexLeanRuntime.checkedConvert a : Option Int64))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i64 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int64) <;> rfl

theorem prim_convert_u8_i64 (a : UInt8) :
    obs (primitive (.convert .i64) [Value.u8 a]) =
      some (Rel true (encOption Value.i64 (LexLeanRuntime.checkedConvert a : Option Int64))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i64 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int64) <;> rfl

theorem prim_convert_u16_i64 (a : UInt16) :
    obs (primitive (.convert .i64) [Value.u16 a]) =
      some (Rel true (encOption Value.i64 (LexLeanRuntime.checkedConvert a : Option Int64))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i64 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int64) <;> rfl

theorem prim_convert_u32_i64 (a : UInt32) :
    obs (primitive (.convert .i64) [Value.u32 a]) =
      some (Rel true (encOption Value.i64 (LexLeanRuntime.checkedConvert a : Option Int64))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i64 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int64) <;> rfl

theorem prim_convert_u64_i64 (a : UInt64) :
    obs (primitive (.convert .i64) [Value.u64 a]) =
      some (Rel true (encOption Value.i64 (LexLeanRuntime.checkedConvert a : Option Int64))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i64 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int64) <;> rfl

theorem prim_convert_i8_i64 (a : Int8) :
    obs (primitive (.convert .i64) [Value.i8 a]) =
      some (Rel true (encOption Value.i64 (LexLeanRuntime.checkedConvert a : Option Int64))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i64 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int64) <;> rfl

theorem prim_convert_i16_i64 (a : Int16) :
    obs (primitive (.convert .i64) [Value.i16 a]) =
      some (Rel true (encOption Value.i64 (LexLeanRuntime.checkedConvert a : Option Int64))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i64 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int64) <;> rfl

theorem prim_convert_i32_i64 (a : Int32) :
    obs (primitive (.convert .i64) [Value.i32 a]) =
      some (Rel true (encOption Value.i64 (LexLeanRuntime.checkedConvert a : Option Int64))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i64 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int64) <;> rfl

theorem prim_convert_i64_i64 (a : Int64) :
    obs (primitive (.convert .i64) [Value.i64 a]) =
      some (Rel true (encOption Value.i64 (LexLeanRuntime.checkedConvert a : Option Int64))) := by
  show obs (match (LexLeanRuntime.checkedConvert a : Option Int64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i64 r)) 0) = _
  cases (LexLeanRuntime.checkedConvert a : Option Int64) <;> rfl


/-! The 64-bit signed multiplication and quotient go through recursive
magnitude loops. Each module's copy of a loop is a distinct constant that no
elaborator unfolds on symbolic operands, so the relations take the loop by
its defining clauses; a certificate discharges each clause by `rfl`. -/

def multiplyStep (mm : Nat → UInt64 → UInt64 → UInt64 → Bool → Option UInt64) (fuel : Nat)
    (source multiplicand accumulator : UInt64) (negative : Bool) : Option UInt64 :=
  let high := 9223372036854775808 <= source
  let limit := if negative then 9223372036854775808 else 9223372036854775807
  let halfLimit := if negative then 4611686018427387904 else 4611686018427387903
  if halfLimit < accumulator then none
  else
    let doubled := accumulator + accumulator
    if high then
      if limit < multiplicand || limit - multiplicand < doubled then none
      else mm fuel (source + source) multiplicand (doubled + multiplicand) negative
    else
      mm fuel (source + source) multiplicand doubled negative

theorem multiply_unique (mm : Nat → UInt64 → UInt64 → UInt64 → Bool → Option UInt64)
    (h0 : ∀ s m a n, mm 0 s m a n = some a)
    (hs : ∀ f s m a n, mm (f + 1) s m a n = multiplyStep mm f s m a n) :
    ∀ f s m a n, mm f s m a n = LexLeanRuntime.multiplyMagnitudeInt64 f s m a n
  | 0, s, m, a, n => h0 s m a n
  | f + 1, s, m, a, n => by
    rw [hs]
    unfold multiplyStep
    simp only [multiply_unique mm h0 hs f]
    rfl

/-- `checkedMultiplyInt64` over a given magnitude loop. -/
def checkedMultiplyWith (mm : Nat → UInt64 → UInt64 → UInt64 → Bool → Option UInt64) (left right : Int64) :
    Option Int64 :=
  let negative := (left < 0) != (right < 0)
  match mm 64 (LexLeanRuntime.magnitudeInt64 right) (LexLeanRuntime.magnitudeInt64 left) 0 negative with
  | none => none
  | some value => some (LexLeanRuntime.signedMagnitudeInt64 negative value)

theorem prim_checkedMul_i64 (mm : Nat → UInt64 → UInt64 → UInt64 → Bool → Option UInt64)
    (h0 : ∀ s m a n, mm 0 s m a n = some a)
    (hs : ∀ f s m a n, mm (f + 1) s m a n = multiplyStep mm f s m a n) (a b : Int64) :
    obs (primitive .checkedMul [Value.i64 a, Value.i64 b]) =
      some (Rel true (encOption Value.i64 (checkedMultiplyWith mm a b))) := by
  have h : checkedMultiplyWith mm a b = LexLeanRuntime.checkedMultiplyInt64 a b := by
    unfold checkedMultiplyWith
    simp only [multiply_unique mm h0 hs]
    rfl
  rw [h]
  show obs (match (LexLeanRuntime.checkedMultiplyInt64 a b : Option Int64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i64 r)) 0) = _
  cases (LexLeanRuntime.checkedMultiplyInt64 a b : Option Int64) <;> rfl

def divideStep (dm : Nat → UInt64 → UInt64 → UInt64 → UInt64 → UInt64) (fuel : Nat)
    (source divisor remainder quotient : UInt64) : UInt64 :=
  let high := 9223372036854775808 <= source
  let source := source + source
  let remainder := remainder + remainder + if high then 1 else 0
  let quotient := quotient + quotient
  if divisor <= remainder then
    dm fuel source divisor (remainder - divisor) (quotient + 1)
  else
    dm fuel source divisor remainder quotient

theorem divide_unique (dm : Nat → UInt64 → UInt64 → UInt64 → UInt64 → UInt64)
    (h0 : ∀ s d r q, dm 0 s d r q = q)
    (hs : ∀ f s d r q, dm (f + 1) s d r q = divideStep dm f s d r q) :
    ∀ f s d r q, dm f s d r q = LexLeanRuntime.divideMagnitudeInt64 f s d r q
  | 0, s, d, r, q => h0 s d r q
  | f + 1, s, d, r, q => by
    rw [hs]
    unfold divideStep
    simp only [divide_unique dm h0 hs f]
    rfl

/-- `checkedQuotientInt64` over a given magnitude loop. -/
def checkedQuotientWith (dm : Nat → UInt64 → UInt64 → UInt64 → UInt64 → UInt64) (left right : Int64) :
    Option Int64 :=
  if right == 0 then none
  else if left == (-9223372036854775808 : Int64) && right == (-1 : Int64) then none
  else
    let negative := (left < 0) != (right < 0)
    some (LexLeanRuntime.signedMagnitudeInt64 negative
      (dm 64 (LexLeanRuntime.magnitudeInt64 left) (LexLeanRuntime.magnitudeInt64 right) 0 0))

theorem prim_checkedQuot_i64 (dm : Nat → UInt64 → UInt64 → UInt64 → UInt64 → UInt64)
    (h0 : ∀ s d r q, dm 0 s d r q = q)
    (hs : ∀ f s d r q, dm (f + 1) s d r q = divideStep dm f s d r q) (a b : Int64) :
    obs (primitive .checkedQuot [Value.i64 a, Value.i64 b]) =
      some (Rel true (encOption Value.i64 (checkedQuotientWith dm a b))) := by
  have h : checkedQuotientWith dm a b = LexLeanRuntime.checkedQuotientInt64 a b := by
    unfold checkedQuotientWith
    simp only [divide_unique dm h0 hs]
    rfl
  rw [h]
  show obs (match (LexLeanRuntime.checkedQuotientInt64 a b : Option Int64) with
    | none => Outcome.value Value.none 0 | some r => Outcome.value (Value.some (Value.i64 r)) 0) = _
  cases (LexLeanRuntime.checkedQuotientInt64 a b : Option Int64) <;> rfl

theorem prim_formatDecimal_int (a : Int) :
    obs (primitive .formatDecimal [Value.int a]) = some (Rel true (.string (LexLeanRuntime.formatDecimal a))) := rfl

/-- A parsed integer fits the 64-bit realization, or the text does not parse. -/
def decimalFits (s : String) : Bool :=
  match (LexLeanRuntime.parseDecimal s : Option Int) with
  | none => true
  | some n => intFits n

theorem prim_parseDecimal_int (s : String) :
    obs (primitive (.parseDecimal .int) [Value.string s]) =
      some (Rel (decimalFits s) (encOption Value.int (LexLeanRuntime.parseDecimal s : Option Int))) := by
  show obs (match (LexLeanRuntime.parseDecimal s : Option Int) with
    | none => Outcome.value Value.none 0
    | some parsed => match (LexLeanRuntime.checkedConvert parsed : Option Int64) with
      | none => Outcome.overflow 0
      | some _ => Outcome.value (Value.some (Value.int parsed)) 0) = _
  unfold decimalFits
  cases (LexLeanRuntime.parseDecimal s : Option Int) with
  | none => rfl
  | some n =>
    show obs (match (LexLeanRuntime.checkedConvert n : Option Int64) with
      | none => Outcome.overflow 0
      | some _ => Outcome.value (Value.some (Value.int n)) 0) = some (Rel (intFits n) (encOption Value.int (some n)))
    unfold intFits
    cases (LexLeanRuntime.checkedConvert n : Option Int64) <;> rfl

end LexLeanPreservation
