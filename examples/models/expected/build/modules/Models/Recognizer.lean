module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanRuntime.Models
public import LexLeanRuntime.Laws
public import Models.Glyphs
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Models.Recognizer

@[expose] public def hiddenWeights.bytes : ByteArray := (let __byte0 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 0); let __byte1 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 1); let __byte2 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 2); let __byte4 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 4); _root_.ByteArray.mk #[__byte4, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte4, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte4, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte4, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte4, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte4, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte4, (_root_.UInt8.ofNat (nat_lit 3)), (_root_.UInt8.ofNat (nat_lit 254)), __byte0, __byte0, __byte0, __byte0, (_root_.UInt8.ofNat (nat_lit 255)), __byte0, __byte0, __byte2, (_root_.UInt8.ofNat (nat_lit 253)), __byte1, __byte0, __byte0, (_root_.UInt8.ofNat (nat_lit 255)), __byte0, __byte0, __byte0, __byte2, __byte2, (_root_.UInt8.ofNat (nat_lit 254)), __byte1, __byte1, __byte1, __byte1, __byte1, __byte1, __byte1, __byte0, (_root_.UInt8.ofNat (nat_lit 252)), __byte0, __byte2, __byte0, (_root_.UInt8.ofNat (nat_lit 3)), __byte0])
@[expose] public def hiddenWeights : List (List (Int)) := (((4 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int))))))))) :: (((0 : Int) :: ((4 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int))))))))) :: (((0 : Int) :: ((0 : Int) :: ((4 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int))))))))) :: (((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((4 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int))))))))) :: (((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((4 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int))))))))) :: (((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((4 : Int) :: ((0 : Int) :: ([] : List (Int))))))))) :: (((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((4 : Int) :: ([] : List (Int))))))))) :: (((3 : Int) :: ((-2 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((-1 : Int) :: ([] : List (Int))))))))) :: (((0 : Int) :: ((0 : Int) :: ((2 : Int) :: ((-3 : Int) :: ((1 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int))))))))) :: (((-1 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((2 : Int) :: ((2 : Int) :: ((-2 : Int) :: ([] : List (Int))))))))) :: (((1 : Int) :: ((1 : Int) :: ((1 : Int) :: ((1 : Int) :: ((1 : Int) :: ((1 : Int) :: ((1 : Int) :: ([] : List (Int))))))))) :: (((0 : Int) :: ((-4 : Int) :: ((0 : Int) :: ((2 : Int) :: ((0 : Int) :: ((3 : Int) :: ((0 : Int) :: ([] : List (Int))))))))) :: ([] : List (List (Int)))))))))))))))
public theorem hiddenWeights.decoded : (LexLeanModels.tensorMatches 1 true hiddenWeights.flatten hiddenWeights.bytes && (hiddenWeights.length == 12) && (hiddenWeights.map List.length == List.replicate 12 7)) = true := by decide

@[expose] public def hiddenBias.bytes : ByteArray := (let __byte0 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 0); let __byte255 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 255); _root_.ByteArray.mk #[__byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, (_root_.UInt8.ofNat (nat_lit 1)), __byte0, __byte0, __byte0, (_root_.UInt8.ofNat (nat_lit 254)), __byte255, __byte255, __byte255, (_root_.UInt8.ofNat (nat_lit 3)), __byte0, __byte0, __byte0, (_root_.UInt8.ofNat (nat_lit 251)), __byte255, __byte255, __byte255, (_root_.UInt8.ofNat (nat_lit 2)), __byte0, __byte0, __byte0])
@[expose] public def hiddenBias : List (Int) := ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((1 : Int) :: ((-2 : Int) :: ((3 : Int) :: ((-5 : Int) :: ((2 : Int) :: ([] : List (Int))))))))))))))
public theorem hiddenBias.decoded : (LexLeanModels.tensorMatches 4 true hiddenBias hiddenBias.bytes && (hiddenBias.length == 12)) = true := by decide

@[expose] public def outputWeights.bytes : ByteArray := (let __byte0 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 0); let __byte2 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 2); _root_.ByteArray.mk #[__byte2, __byte2, __byte2, __byte2, __byte2, __byte2, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte2, __byte2, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte2, __byte2, __byte0, __byte2, __byte2, __byte0, __byte2, __byte0, __byte0, __byte0, __byte0, __byte0, __byte2, __byte2, __byte2, __byte2, __byte0, __byte0, __byte2, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte2, __byte2, __byte0, __byte0, __byte2, __byte2, __byte0, __byte0, __byte0, __byte0, __byte0, __byte2, __byte0, __byte2, __byte2, __byte0, __byte2, __byte2, __byte0, __byte0, __byte0, __byte0, __byte0, __byte2, __byte0, __byte2, __byte2, __byte2, __byte2, __byte2, __byte0, __byte0, __byte0, __byte0, __byte0, __byte2, __byte2, __byte2, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte2, __byte2, __byte2, __byte2, __byte2, __byte2, __byte2, __byte0, __byte0, __byte0, __byte0, __byte0, __byte2, __byte2, __byte2, __byte2, __byte0, __byte2, __byte2, __byte0, __byte0, __byte0, __byte0, __byte0])
@[expose] public def outputWeights : List (List (Int)) := (((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int)))))))))))))) :: (((0 : Int) :: ((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int)))))))))))))) :: (((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ((2 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int)))))))))))))) :: (((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ((0 : Int) :: ((2 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int)))))))))))))) :: (((0 : Int) :: ((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ((0 : Int) :: ((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int)))))))))))))) :: (((2 : Int) :: ((0 : Int) :: ((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int)))))))))))))) :: (((2 : Int) :: ((0 : Int) :: ((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int)))))))))))))) :: (((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int)))))))))))))) :: (((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int)))))))))))))) :: (((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int)))))))))))))) :: ([] : List (List (Int)))))))))))))
public theorem outputWeights.decoded : (LexLeanModels.tensorMatches 1 true outputWeights.flatten outputWeights.bytes && (outputWeights.length == 10) && (outputWeights.map List.length == List.replicate 10 12)) = true := by decide

@[expose] public def outputBias.bytes : ByteArray := (let __byte255 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 255); _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 250)), __byte255, __byte255, __byte255, (_root_.UInt8.ofNat (nat_lit 254)), __byte255, __byte255, __byte255, (_root_.UInt8.ofNat (nat_lit 251)), __byte255, __byte255, __byte255, (_root_.UInt8.ofNat (nat_lit 251)), __byte255, __byte255, __byte255, (_root_.UInt8.ofNat (nat_lit 252)), __byte255, __byte255, __byte255, (_root_.UInt8.ofNat (nat_lit 251)), __byte255, __byte255, __byte255, (_root_.UInt8.ofNat (nat_lit 250)), __byte255, __byte255, __byte255, (_root_.UInt8.ofNat (nat_lit 253)), __byte255, __byte255, __byte255, (_root_.UInt8.ofNat (nat_lit 249)), __byte255, __byte255, __byte255, (_root_.UInt8.ofNat (nat_lit 250)), __byte255, __byte255, __byte255])
@[expose] public def outputBias : List (Int) := ((-6 : Int) :: ((-2 : Int) :: ((-5 : Int) :: ((-5 : Int) :: ((-4 : Int) :: ((-5 : Int) :: ((-6 : Int) :: ((-3 : Int) :: ((-7 : Int) :: ((-6 : Int) :: ([] : List (Int))))))))))))
public theorem outputBias.decoded : (LexLeanModels.tensorMatches 4 true outputBias outputBias.bytes && (outputBias.length == 10)) = true := by decide

@[expose] public def digitNames.bytes : ByteArray := (let __byte10 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 10); let __byte101 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 101); let __byte105 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 105); let __byte110 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 110); let __byte111 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 111); _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 122)), __byte101, (_root_.UInt8.ofNat (nat_lit 114)), __byte111, __byte10, __byte111, __byte110, __byte101, __byte10, (_root_.UInt8.ofNat (nat_lit 116)), (_root_.UInt8.ofNat (nat_lit 119)), __byte111, __byte10, (_root_.UInt8.ofNat (nat_lit 116)), (_root_.UInt8.ofNat (nat_lit 104)), (_root_.UInt8.ofNat (nat_lit 114)), __byte101, __byte101, __byte10, (_root_.UInt8.ofNat (nat_lit 102)), __byte111, (_root_.UInt8.ofNat (nat_lit 117)), (_root_.UInt8.ofNat (nat_lit 114)), __byte10, (_root_.UInt8.ofNat (nat_lit 102)), __byte105, (_root_.UInt8.ofNat (nat_lit 118)), __byte101, __byte10, (_root_.UInt8.ofNat (nat_lit 115)), __byte105, (_root_.UInt8.ofNat (nat_lit 120)), __byte10, (_root_.UInt8.ofNat (nat_lit 115)), __byte101, (_root_.UInt8.ofNat (nat_lit 118)), __byte101, __byte110, __byte10, __byte101, __byte105, (_root_.UInt8.ofNat (nat_lit 103)), (_root_.UInt8.ofNat (nat_lit 104)), (_root_.UInt8.ofNat (nat_lit 116)), __byte10, __byte110, __byte105, __byte110, __byte101, __byte10])
@[expose] public def digitNames : List (String) := ("zero" :: ("one" :: ("two" :: ("three" :: ("four" :: ("five" :: ("six" :: ("seven" :: ("eight" :: ("nine" :: ([] : List (String))))))))))))
public theorem digitNames.decoded : LexLeanModels.linesMatch digitNames digitNames.bytes = true := by decide

@[expose] public def banner.bytes : ByteArray := _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 76)), (_root_.UInt8.ofNat (nat_lit 76)), (_root_.UInt8.ofNat (nat_lit 77)), (_root_.UInt8.ofNat (nat_lit 0)), (_root_.UInt8.ofNat (nat_lit 1))]
@[expose] public def banner : ByteArray := _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 76)), (_root_.UInt8.ofNat (nat_lit 76)), (_root_.UInt8.ofNat (nat_lit 77)), (_root_.UInt8.ofNat (nat_lit 0)), (_root_.UInt8.ofNat (nat_lit 1))]
public theorem banner.decoded : banner = banner.bytes := rfl

@[expose, reducible] public def Recognizes (glyph : Models.Glyphs.Glyph) (digit : Nat) : Prop := (digit = Models.Glyphs.digitOf (glyph))

@[expose] public def recognizesCheck (glyph : Models.Glyphs.Glyph) (digit : Nat) : Bool := (Nat.beq (digit) (Models.Glyphs.digitOf (glyph)))

public theorem recognizes_check_sound (glyph : Models.Glyphs.Glyph) (digit : Nat) : ((recognizesCheck (glyph) (digit) = true) -> Recognizes (glyph) (digit)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Recognizes, recognizesCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem recognizes_check_complete (glyph : Models.Glyphs.Glyph) (digit : Nat) : (Recognizes (glyph) (digit) -> (recognizesCheck (glyph) (digit) = true)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Recognizes, recognizesCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem DigitContract.postcondition_sound : LexLeanModels.Sound2 ((Models.Recognizer.recognizesCheck)) ((Models.Recognizer.Recognizes)) := Models.Recognizer.recognizes_check_sound
public theorem DigitContract.postcondition_complete : LexLeanModels.Complete2 ((Models.Recognizer.recognizesCheck)) ((Models.Recognizer.Recognizes)) := Models.Recognizer.recognizes_check_complete

public theorem digit_net_width (glyph : Models.Glyphs.Glyph) : ((LexLeanRuntime.length (Models.Glyphs.encode (glyph)) : Nat) = 7) := by
  exact Models.Glyphs.encode_width (glyph)

@[expose] public def DigitNet.encoded (glyph : Models.Glyphs.Glyph) : List (Int) := Models.Glyphs.encode (glyph)
@[expose] public def DigitNet.layer1 (__values : List (Int)) : List (Int) := ((LexLeanCollections.listFold ((fun (__acc : (Prod (List (Int)) (List (Int)))) (__row : List (Int)) => (match (__acc).2 with | List.nil => __acc | List.cons __bias __biases => ((LexLeanRuntime.append ((__acc).1) (((LexLeanRuntime.subtract (__bias) ((LexLeanRuntime.negate (((LexLeanCollections.listFold ((fun (__sum : (Prod (Int) (List (Int)))) (__weight : Int) => (match (__sum).2 with | List.nil => __sum | List.cons __value __rest => ((LexLeanRuntime.subtract ((__sum).1) ((LexLeanRuntime.negate ((LexLeanRuntime.multiply (__weight) (__value) : Int)) : Int)) : Int), __rest)))) (((0 : Int), __values)) (__row) : (Prod (Int) (List (Int))))).1) : Int)) : Int) :: ([] : List (Int)))) : List (Int)), __biases)))) ((([] : List (Int)), Models.Recognizer.hiddenBias)) (Models.Recognizer.hiddenWeights) : (Prod (List (Int)) (List (Int))))).1
@[expose] public def DigitNet.layer2 (__values : List (Int)) : List (Int) := (LexLeanCollections.listFold ((fun (__out : List (Int)) (__value : Int) => (LexLeanRuntime.append (__out) (((if (LexLeanCollections.lessThan (__value) ((0 : Int)) : Bool) then (0 : Int) else __value) :: ([] : List (Int)))) : List (Int)))) (([] : List (Int))) (__values) : List (Int))
@[expose] public def DigitNet.layer3 (__values : List (Int)) : List (Int) := (LexLeanCollections.listFold ((fun (__out : List (Int)) (__value : Int) => (LexLeanRuntime.append (__out) (((let __quotient : Int := (LexLeanRuntime.quotient (__value) ((4 : Int)) ((0 : Int)) : Int); (if (LexLeanCollections.lessThan (__quotient) ((0 : Int)) : Bool) then (0 : Int) else (if (LexLeanCollections.lessThan ((127 : Int)) (__quotient) : Bool) then (127 : Int) else __quotient))) :: ([] : List (Int)))) : List (Int)))) (([] : List (Int))) (__values) : List (Int))
@[expose] public def DigitNet.layer4 (__values : List (Int)) : List (Int) := ((LexLeanCollections.listFold ((fun (__acc : (Prod (List (Int)) (List (Int)))) (__row : List (Int)) => (match (__acc).2 with | List.nil => __acc | List.cons __bias __biases => ((LexLeanRuntime.append ((__acc).1) (((LexLeanRuntime.subtract (__bias) ((LexLeanRuntime.negate (((LexLeanCollections.listFold ((fun (__sum : (Prod (Int) (List (Int)))) (__weight : Int) => (match (__sum).2 with | List.nil => __sum | List.cons __value __rest => ((LexLeanRuntime.subtract ((__sum).1) ((LexLeanRuntime.negate ((LexLeanRuntime.multiply (__weight) (__value) : Int)) : Int)) : Int), __rest)))) (((0 : Int), __values)) (__row) : (Prod (Int) (List (Int))))).1) : Int)) : Int) :: ([] : List (Int)))) : List (Int)), __biases)))) ((([] : List (Int)), Models.Recognizer.outputBias)) (Models.Recognizer.outputWeights) : (Prod (List (Int)) (List (Int))))).1
@[expose] public def DigitNet.logits (glyph : Models.Glyphs.Glyph) : List (Int) := Models.Recognizer.DigitNet.layer4 (Models.Recognizer.DigitNet.layer3 (Models.Recognizer.DigitNet.layer2 (Models.Recognizer.DigitNet.layer1 (Models.Recognizer.DigitNet.encoded (glyph)))))
@[expose] public def DigitNet (glyph : Models.Glyphs.Glyph) : Nat := (match ((LexLeanCollections.listFold ((fun (__best : (Prod (Option ((Prod (Nat) (Int)))) (List (Nat)))) (__score : Int) => (match (__best).2 with | List.nil => __best | List.cons __label __labels => ((match (__best).1 with | Option.none => Option.some ((__label, __score)) | Option.some __leader => (if (LexLeanCollections.lessThan ((__leader).2) (__score) : Bool) then Option.some ((__label, __score)) else Option.some (__leader))), __labels)))) ((Option.none, (0 :: (1 :: (2 :: (3 :: (4 :: (5 :: (6 :: (7 :: (8 :: (9 :: ([] : List (Nat)))))))))))))) (Models.Recognizer.DigitNet.logits (glyph)) : (Prod (Option ((Prod (Nat) (Int)))) (List (Nat))))).1 with | Option.none => 0 | Option.some __winner => (__winner).1)

public theorem digit_net_exact (glyph : Models.Glyphs.Glyph) : (DigitNet (glyph) = Models.Glyphs.digitOf (glyph)) := by
  cases glyph with
  | g0 =>
    decide
  | g1 =>
    decide
  | g2 =>
    decide
  | g3 =>
    decide
  | g4 =>
    decide
  | g5 =>
    decide
  | g6 =>
    decide
  | g7 =>
    decide
  | g8 =>
    decide
  | g9 =>
    decide

public theorem digit_net_correct (glyph : Models.Glyphs.Glyph) : Recognizes (glyph) (DigitNet (glyph)) := by
  cases glyph with
  | g0 =>
    decide
  | g1 =>
    decide
  | g2 =>
    decide
  | g3 =>
    decide
  | g4 =>
    decide
  | g5 =>
    decide
  | g6 =>
    decide
  | g7 =>
    decide
  | g8 =>
    decide
  | g9 =>
    decide

@[expose] public def labeledGlyphs : List ((Prod (Models.Glyphs.Glyph) (Nat))) := ((Models.Glyphs.Glyph.g3, 3) :: ((Models.Glyphs.Glyph.g0, 0) :: ((Models.Glyphs.Glyph.g7, 7) :: ((Models.Glyphs.Glyph.g4, 4) :: ((Models.Glyphs.Glyph.g1, 1) :: ((Models.Glyphs.Glyph.g8, 8) :: ((Models.Glyphs.Glyph.g5, 5) :: ((Models.Glyphs.Glyph.g2, 1) :: ((Models.Glyphs.Glyph.g9, 9) :: ((Models.Glyphs.Glyph.g6, 6) :: ((Models.Glyphs.Glyph.g3, 3) :: ((Models.Glyphs.Glyph.g0, 0) :: ((Models.Glyphs.Glyph.g7, 7) :: ((Models.Glyphs.Glyph.g4, 4) :: ((Models.Glyphs.Glyph.g1, 1) :: ((Models.Glyphs.Glyph.g8, 8) :: ((Models.Glyphs.Glyph.g5, 5) :: ((Models.Glyphs.Glyph.g2, 2) :: ((Models.Glyphs.Glyph.g9, 9) :: ((Models.Glyphs.Glyph.g6, 7) :: ((Models.Glyphs.Glyph.g3, 3) :: ((Models.Glyphs.Glyph.g0, 0) :: ((Models.Glyphs.Glyph.g7, 7) :: ((Models.Glyphs.Glyph.g4, 4) :: ((Models.Glyphs.Glyph.g1, 1) :: ((Models.Glyphs.Glyph.g8, 8) :: ((Models.Glyphs.Glyph.g5, 5) :: ((Models.Glyphs.Glyph.g2, 2) :: ((Models.Glyphs.Glyph.g9, 9) :: ((Models.Glyphs.Glyph.g6, 6) :: ((Models.Glyphs.Glyph.g3, 3) :: ((Models.Glyphs.Glyph.g0, 0) :: ((Models.Glyphs.Glyph.g7, 7) :: ((Models.Glyphs.Glyph.g4, 8) :: ((Models.Glyphs.Glyph.g1, 1) :: ((Models.Glyphs.Glyph.g8, 8) :: ((Models.Glyphs.Glyph.g5, 5) :: ((Models.Glyphs.Glyph.g2, 2) :: ((Models.Glyphs.Glyph.g9, 9) :: ((Models.Glyphs.Glyph.g6, 6) :: ([] : List ((Prod (Models.Glyphs.Glyph) (Nat))))))))))))))))))))))))))))))))))))))))))))

@[expose] public def sameDigit (expected : Nat) (observed : Nat) : Bool := (Nat.beq (expected) (observed))

public theorem same_digit_sound (expected : Nat) (observed : Nat) : ((sameDigit (expected) (observed) = true) -> (expected = observed)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [sameDigit, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply, LexLeanRuntime.negate, LexLeanRuntime.equal, beq_iff_eq, decide_eq_true_eq, decide_eq_false_iff_not, Nat.add_assoc, Int.add_assoc, Int.neg_neg, Int.emod_neg, Int.neg_zero, ↓reduceIte, LexLeanLaws.lessThan_nat, LexLeanLaws.lessThan_int, LexLeanLaws.lessThan_int8, LexLeanLaws.lessThan_int16, LexLeanLaws.lessThan_int32, LexLeanLaws.lessThan_int64, LexLeanLaws.lessThan_uint8, LexLeanLaws.lessThan_uint16, LexLeanLaws.lessThan_uint32, LexLeanLaws.lessThan_uint64, LexLeanLaws.quotient_nat, LexLeanLaws.quotient_int, LexLeanLaws.remainder_nat, LexLeanLaws.remainder_int, LexLeanLaws.tdiv_ediv, LexLeanLaws.tmod_emod, LexLeanLaws.length_list, LexLeanLaws.append_list, LexLeanLaws.setFold_listFold, LexLeanLaws.mapFold_listFold, LexLeanLaws.setSize_length, LexLeanLaws.mapSize_length, LexLeanLaws.setElements_self, LexLeanLaws.mapEntries_self, LexLeanLaws.listFold_nil, LexLeanLaws.listFold_append, LexLeanLaws.listFold_prepend, LexLeanLaws.listFold_extend, LexLeanLaws.listFold_add_nat, LexLeanLaws.listFold_add_nat_self, LexLeanLaws.listFold_add_int, LexLeanLaws.listFold_add_int_self, LexLeanLaws.listFold_sub_int, LexLeanLaws.listFold_sub_int_self, LexLeanLaws.sumNat_nil, LexLeanLaws.sumNat_cons, LexLeanLaws.sumInt_nil, LexLeanLaws.sumInt_cons, LexLeanLaws.sumNat_add, LexLeanLaws.sumInt_add, LexLeanLaws.sumInt_sub, LexLeanLaws.sumInt_neg, LexLeanLaws.sumInt_neg_self, LexLeanLaws.sumNat_mul, LexLeanLaws.sumInt_mul, LexLeanLaws.sumNat_mul_self, LexLeanLaws.sumInt_mul_self, LexLeanLaws.sumNat_mul_right, LexLeanLaws.sumInt_mul_right, LexLeanLaws.sumNat_const, LexLeanLaws.sumInt_const, LexLeanLaws.sumNat_append, LexLeanLaws.sumInt_append, LexLeanLaws.sumNat_reverse, LexLeanLaws.sumInt_reverse, UInt8.le_iff_toNat_le, UInt8.lt_iff_toNat_lt, UInt16.le_iff_toNat_le, UInt16.lt_iff_toNat_lt, UInt32.le_iff_toNat_le, UInt32.lt_iff_toNat_lt, UInt64.le_iff_toNat_le, UInt64.lt_iff_toNat_lt, Int8.le_iff_toInt_le, Int8.lt_iff_toInt_lt, Int16.le_iff_toInt_le, Int16.lt_iff_toInt_lt, Int32.le_iff_toInt_le, Int32.lt_iff_toInt_lt, Int64.le_iff_toInt_le, Int64.lt_iff_toInt_lt, UInt8.reduceToNat, ← UInt8.toNat_inj, UInt16.reduceToNat, ← UInt16.toNat_inj, UInt32.reduceToNat, ← UInt32.toNat_inj, UInt64.reduceToNat, ← UInt64.toNat_inj, Int8.reduceToInt, ← Int8.toInt_inj, Int16.reduceToInt, ← Int16.toInt_inj, Int32.reduceToInt, ← Int32.toInt_inj, Int64.reduceToInt, ← Int64.toInt_inj] at *
  try set_option linter.unusedSimpArgs false in simp only [UInt8.toNat, UInt16.toNat, UInt32.toNat, UInt64.toNat, Int8.toInt, Int16.toInt, Int32.toInt, Int64.toInt, BitVec.toInt_eq_toNat_cond] at *
  all_goals (repeat' split)
  all_goals omega

public theorem labeled_agreement : (((LexLeanRuntime.length (labeledGlyphs) : Nat) = 40) /\ ((LexLeanCollections.listFold ((fun (count : Nat) («example» : (Prod (Models.Glyphs.Glyph) (Nat))) => (if sameDigit (DigitNet ((«example»).1)) ((«example»).2) then (count + 1) else count))) (0) (labeledGlyphs) : Nat) = 37)) := by
  decide

public theorem DigitEvidence.labeled_agreement : LexLeanModels.Agreement ((Models.Recognizer.DigitNet)) ((Models.Recognizer.sameDigit)) (Models.Recognizer.labeledGlyphs) (40) (37) := by decide
public theorem DigitEvidence.digit_net_exact : LexLeanModels.EquivalentTotal ((Models.Recognizer.DigitNet)) ((Models.Glyphs.digitOf)) := Models.Recognizer.digit_net_exact
public theorem DigitEvidence.digit_net_correct : LexLeanModels.SatisfiesTotal ((Models.Recognizer.Recognizes)) ((Models.Recognizer.DigitNet)) := Models.Recognizer.digit_net_correct

@[expose] public def DigitModel (__input : Models.Glyphs.Glyph) : Nat := Models.Recognizer.DigitNet (__input)

@[expose] public def RawDigitModel (__input : Models.Glyphs.Glyph) : Nat := Models.Recognizer.DigitNet (__input)

@[expose] public def digitName (digit : Nat) : Option (String) := (LexLeanRuntime.index (digitNames) (digit) : Option (String))

public structure Dyadic where
  sign : Nat
  mantissa : Int
  exponent : Int

@[expose] public def dyadicTriple (value : Option (Dyadic)) : Option ((Prod (Nat) ((Prod (Int) (Int))))) := (match value with | Option.none => Option.none | Option.some d => Option.some (((d).sign, ((d).mantissa, (d).exponent))))

@[expose, reducible] public def DyadicMatches (observed : Option (Dyadic)) (sign : Nat) (mantissa : Int) (exponent : Int) : Prop := (dyadicTriple (observed) = Option.some ((sign, (mantissa, exponent))))

@[expose] public def byteToInt (b : UInt8) : Int := (LexLeanRuntime.subtract ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (b) ((1 : UInt8)) : UInt8)) ((0 : UInt8)) : Bool) then (0 : Int) else (1 : Int))) ((LexLeanRuntime.negate ((LexLeanRuntime.subtract ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (b) ((2 : UInt8)) : UInt8)) ((0 : UInt8)) : Bool) then (0 : Int) else (2 : Int))) ((LexLeanRuntime.negate ((LexLeanRuntime.subtract ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (b) ((4 : UInt8)) : UInt8)) ((0 : UInt8)) : Bool) then (0 : Int) else (4 : Int))) ((LexLeanRuntime.negate ((LexLeanRuntime.subtract ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (b) ((8 : UInt8)) : UInt8)) ((0 : UInt8)) : Bool) then (0 : Int) else (8 : Int))) ((LexLeanRuntime.negate ((LexLeanRuntime.subtract ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (b) ((16 : UInt8)) : UInt8)) ((0 : UInt8)) : Bool) then (0 : Int) else (16 : Int))) ((LexLeanRuntime.negate ((LexLeanRuntime.subtract ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (b) ((32 : UInt8)) : UInt8)) ((0 : UInt8)) : Bool) then (0 : Int) else (32 : Int))) ((LexLeanRuntime.negate ((LexLeanRuntime.subtract ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (b) ((64 : UInt8)) : UInt8)) ((0 : UInt8)) : Bool) then (0 : Int) else (64 : Int))) ((LexLeanRuntime.negate ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (b) ((128 : UInt8)) : UInt8)) ((0 : UInt8)) : Bool) then (0 : Int) else (128 : Int))) : Int)) : Int)) : Int)) : Int)) : Int)) : Int)) : Int)) : Int)) : Int)) : Int)) : Int)) : Int)) : Int)) : Int)

@[expose] public def decodeBinary32 (bits : Int) : Option (Dyadic) := (if ((LexLeanCollections.lessThan (bits) ((0 : Int)) : Bool) || (LexLeanCollections.lessThan ((4294967295 : Int)) (bits) : Bool)) then Option.none else (let signBit : Int := (LexLeanRuntime.quotient (bits) ((2147483648 : Int)) ((0 : Int)) : Int); (let sign : Nat := (if (LexLeanCollections.lessThan ((0 : Int)) (signBit) : Bool) then 1 else 0); (let rest : Int := (LexLeanRuntime.remainder (bits) ((2147483648 : Int)) ((0 : Int)) : Int); (let biased : Int := (LexLeanRuntime.quotient (rest) ((8388608 : Int)) ((0 : Int)) : Int); (let fraction : Int := (LexLeanRuntime.remainder (rest) ((8388608 : Int)) ((0 : Int)) : Int); (if (!(LexLeanCollections.lessThan (biased) ((255 : Int)) : Bool)) then Option.none else (if (!(LexLeanCollections.lessThan ((0 : Int)) (biased) : Bool)) then Option.some (({ sign := sign, mantissa := fraction, exponent := (-149 : Int) } : Dyadic)) else Option.some (({ sign := sign, mantissa := (LexLeanRuntime.subtract ((8388608 : Int)) ((LexLeanRuntime.negate (fraction) : Int)) : Int), exponent := (LexLeanRuntime.subtract (biased) ((150 : Int)) : Int) } : Dyadic))))))))))

@[expose] public def decodeBinary64 (bits : Int) : Option (Dyadic) := (if (LexLeanCollections.lessThan (bits) ((0 : Int)) : Bool) then (let rest : Int := (LexLeanRuntime.subtract ((LexLeanRuntime.subtract (bits) ((-4611686018427387904 : Int)) : Int)) ((-4611686018427387904 : Int)) : Int); (if (LexLeanCollections.lessThan (rest) ((0 : Int)) : Bool) then Option.none else (let biased : Int := (LexLeanRuntime.quotient (rest) ((4503599627370496 : Int)) ((0 : Int)) : Int); (let fraction : Int := (LexLeanRuntime.remainder (rest) ((4503599627370496 : Int)) ((0 : Int)) : Int); (if (!(LexLeanCollections.lessThan (biased) ((2047 : Int)) : Bool)) then Option.none else (if (!(LexLeanCollections.lessThan ((0 : Int)) (biased) : Bool)) then Option.some (({ sign := 1, mantissa := fraction, exponent := (-1074 : Int) } : Dyadic)) else Option.some (({ sign := 1, mantissa := (LexLeanRuntime.subtract ((4503599627370496 : Int)) ((LexLeanRuntime.negate (fraction) : Int)) : Int), exponent := (LexLeanRuntime.subtract (biased) ((1075 : Int)) : Int) } : Dyadic)))))))) else (let q0 : Int := (LexLeanRuntime.quotient (bits) ((4503599627370496 : Int)) ((0 : Int)) : Int); (if (!(LexLeanCollections.lessThan ((2047 : Int)) (q0) : Bool)) then (let fraction : Int := (LexLeanRuntime.remainder (bits) ((4503599627370496 : Int)) ((0 : Int)) : Int); (if (!(LexLeanCollections.lessThan (q0) ((2047 : Int)) : Bool)) then Option.none else (if (!(LexLeanCollections.lessThan ((0 : Int)) (q0) : Bool)) then Option.some (({ sign := 0, mantissa := fraction, exponent := (-1074 : Int) } : Dyadic)) else Option.some (({ sign := 0, mantissa := (LexLeanRuntime.subtract ((4503599627370496 : Int)) ((LexLeanRuntime.negate (fraction) : Int)) : Int), exponent := (LexLeanRuntime.subtract (q0) ((1075 : Int)) : Int) } : Dyadic))))) else (let rest : Int := (LexLeanRuntime.subtract ((LexLeanRuntime.subtract (bits) ((4611686018427387904 : Int)) : Int)) ((4611686018427387904 : Int)) : Int); (let biased : Int := (LexLeanRuntime.quotient (rest) ((4503599627370496 : Int)) ((0 : Int)) : Int); (if (!(LexLeanCollections.lessThan (biased) ((2047 : Int)) : Bool)) then Option.none else (let fraction : Int := (LexLeanRuntime.remainder (rest) ((4503599627370496 : Int)) ((0 : Int)) : Int); (if (!(LexLeanCollections.lessThan ((0 : Int)) (biased) : Bool)) then Option.some (({ sign := 1, mantissa := fraction, exponent := (-1074 : Int) } : Dyadic)) else Option.some (({ sign := 1, mantissa := (LexLeanRuntime.subtract ((4503599627370496 : Int)) ((LexLeanRuntime.negate (fraction) : Int)) : Int), exponent := (LexLeanRuntime.subtract (biased) ((1075 : Int)) : Int) } : Dyadic))))))))))

@[expose] public def decodeBinary32Bytes (bytes : ByteArray) (offset : Nat) : Option (Dyadic) := (match (LexLeanRuntime.index (bytes) (offset) : Option (UInt8)) with | Option.none => Option.none | Option.some b0 => (match (LexLeanRuntime.index (bytes) ((offset + 1)) : Option (UInt8)) with | Option.none => Option.none | Option.some b1 => (match (LexLeanRuntime.index (bytes) ((offset + 2)) : Option (UInt8)) with | Option.none => Option.none | Option.some b2 => (match (LexLeanRuntime.index (bytes) ((offset + 3)) : Option (UInt8)) with | Option.none => Option.none | Option.some b3 => decodeBinary32 ((LexLeanRuntime.subtract (byteToInt (b0)) ((LexLeanRuntime.negate ((LexLeanRuntime.subtract ((LexLeanRuntime.multiply ((256 : Int)) (byteToInt (b1)) : Int)) ((LexLeanRuntime.negate ((LexLeanRuntime.subtract ((LexLeanRuntime.multiply ((65536 : Int)) (byteToInt (b2)) : Int)) ((LexLeanRuntime.negate ((LexLeanRuntime.multiply ((16777216 : Int)) (byteToInt (b3)) : Int)) : Int)) : Int)) : Int)) : Int)) : Int)) : Int))))))

@[expose] public def decodeBinary64Bytes (bytes : ByteArray) (offset : Nat) : Option (Dyadic) := (match (LexLeanRuntime.index (bytes) (offset) : Option (UInt8)) with | Option.none => Option.none | Option.some b0 => (match (LexLeanRuntime.index (bytes) ((offset + 1)) : Option (UInt8)) with | Option.none => Option.none | Option.some b1 => (match (LexLeanRuntime.index (bytes) ((offset + 2)) : Option (UInt8)) with | Option.none => Option.none | Option.some b2 => (match (LexLeanRuntime.index (bytes) ((offset + 3)) : Option (UInt8)) with | Option.none => Option.none | Option.some b3 => (match (LexLeanRuntime.index (bytes) ((offset + 4)) : Option (UInt8)) with | Option.none => Option.none | Option.some b4 => (match (LexLeanRuntime.index (bytes) ((offset + 5)) : Option (UInt8)) with | Option.none => Option.none | Option.some b5 => (match (LexLeanRuntime.index (bytes) ((offset + 6)) : Option (UInt8)) with | Option.none => Option.none | Option.some b6 => (match (LexLeanRuntime.index (bytes) ((offset + 7)) : Option (UInt8)) with | Option.none => Option.none | Option.some b7 => (let v7 : Int := byteToInt (b7); (let low7 : Int := (if (LexLeanCollections.lessThan (v7) ((128 : Int)) : Bool) then v7 else (LexLeanRuntime.subtract (v7) ((128 : Int)) : Int)); (let low63 : Int := (LexLeanRuntime.subtract (byteToInt (b0)) ((LexLeanRuntime.negate ((LexLeanRuntime.subtract ((LexLeanRuntime.multiply ((256 : Int)) (byteToInt (b1)) : Int)) ((LexLeanRuntime.negate ((LexLeanRuntime.subtract ((LexLeanRuntime.multiply ((65536 : Int)) (byteToInt (b2)) : Int)) ((LexLeanRuntime.negate ((LexLeanRuntime.subtract ((LexLeanRuntime.multiply ((16777216 : Int)) (byteToInt (b3)) : Int)) ((LexLeanRuntime.negate ((LexLeanRuntime.subtract ((LexLeanRuntime.multiply ((4294967296 : Int)) (byteToInt (b4)) : Int)) ((LexLeanRuntime.negate ((LexLeanRuntime.subtract ((LexLeanRuntime.multiply ((1099511627776 : Int)) (byteToInt (b5)) : Int)) ((LexLeanRuntime.negate ((LexLeanRuntime.subtract ((LexLeanRuntime.multiply ((281474976710656 : Int)) (byteToInt (b6)) : Int)) ((LexLeanRuntime.negate ((LexLeanRuntime.multiply ((72057594037927936 : Int)) (low7) : Int)) : Int)) : Int)) : Int)) : Int)) : Int)) : Int)) : Int)) : Int)) : Int)) : Int)) : Int)) : Int)) : Int)) : Int); (let signedBits : Int := (if (LexLeanCollections.lessThan (v7) ((128 : Int)) : Bool) then low63 else (LexLeanRuntime.subtract ((LexLeanRuntime.subtract (low63) ((4611686018427387904 : Int)) : Int)) ((4611686018427387904 : Int)) : Int)); decodeBinary64 (signedBits)))))))))))))

@[expose] public def dyadicMul (a : Dyadic) (b : Dyadic) : Dyadic := ({ sign := (if (Nat.beq ((a).sign) ((b).sign)) then 0 else 1), mantissa := (LexLeanRuntime.multiply ((a).mantissa) ((b).mantissa) : Int), exponent := (LexLeanRuntime.subtract ((a).exponent) ((LexLeanRuntime.negate ((b).exponent) : Int)) : Int) } : Dyadic)

@[expose] public def dyadicRoundShift (d : Dyadic) (shift : Int) : Int := (if (!(LexLeanCollections.lessThan ((0 : Int)) ((d).mantissa) : Bool)) then (0 : Int) else (let k : Int := (LexLeanRuntime.subtract ((d).exponent) ((LexLeanRuntime.negate (shift) : Int)) : Int); (let mag : Int := (if (!(LexLeanCollections.lessThan (k) ((0 : Int)) : Bool)) then (((LexLeanCollections.iterateUntil ((fun (st : (Prod (Int) (Int))) => (if (!(LexLeanCollections.lessThan ((0 : Int)) ((st).1) : Bool)) then Option.none else Option.some (((LexLeanRuntime.subtract ((st).1) ((1 : Int)) : Int), (LexLeanRuntime.multiply ((2 : Int)) ((st).2) : Int)))))) (512) ((k, (d).mantissa)) : (Prod ((Prod (Int) (Int))) (Bool)))).1).2 else (let s : Int := (LexLeanRuntime.negate (k) : Int); (let stepped : (Prod (Int) (Int)) := (((LexLeanCollections.iterateUntil ((fun (st : (Prod (Int) ((Prod (Int) (Int))))) => (if (!(LexLeanCollections.lessThan ((0 : Int)) ((st).1) : Bool)) then Option.none else (let qs : (Prod (Int) (Int)) := (st).2; Option.some (((LexLeanRuntime.subtract ((st).1) ((1 : Int)) : Int), ((LexLeanRuntime.quotient ((qs).1) ((2 : Int)) ((0 : Int)) : Int), (if ((!(LexLeanCollections.lessThan ((0 : Int)) ((qs).2) : Bool)) && (!(LexLeanCollections.lessThan ((0 : Int)) ((LexLeanRuntime.remainder ((qs).1) ((2 : Int)) ((0 : Int)) : Int)) : Bool))) then (0 : Int) else (1 : Int))))))))) (512) (((LexLeanRuntime.subtract (s) ((1 : Int)) : Int), ((d).mantissa, (0 : Int)))) : (Prod ((Prod (Int) ((Prod (Int) (Int))))) (Bool)))).1).2; (let half : Int := (stepped).1; (let sticky : Int := (stepped).2; (let baseMag : Int := (LexLeanRuntime.quotient (half) ((2 : Int)) ((0 : Int)) : Int); (let hb : Int := (LexLeanRuntime.remainder (half) ((2 : Int)) ((0 : Int)) : Int); (if (!(LexLeanCollections.lessThan ((0 : Int)) (hb) : Bool)) then baseMag else (if (LexLeanCollections.lessThan ((0 : Int)) (sticky) : Bool) then (LexLeanRuntime.subtract (baseMag) ((-1 : Int)) : Int) else (LexLeanRuntime.subtract (baseMag) ((LexLeanRuntime.negate ((LexLeanRuntime.remainder (baseMag) ((2 : Int)) ((0 : Int)) : Int)) : Int)) : Int)))))))))); (if (Nat.beq ((d).sign) (0)) then mag else (LexLeanRuntime.negate (mag) : Int)))))

@[expose] public def dyadicRatioRound (a : Dyadic) (b : Dyadic) (c : Dyadic) (shift : Int) : Option (Int) := (if (!(LexLeanCollections.lessThan ((0 : Int)) ((c).mantissa) : Bool)) then Option.none else (if ((!(LexLeanCollections.lessThan ((0 : Int)) ((a).mantissa) : Bool)) || (!(LexLeanCollections.lessThan ((0 : Int)) ((b).mantissa) : Bool))) then Option.some ((0 : Int)) else (let product : Int := (LexLeanRuntime.multiply ((a).mantissa) ((b).mantissa) : Int); (let divisor : Int := (c).mantissa; (let k : Int := (LexLeanRuntime.subtract ((LexLeanRuntime.subtract ((LexLeanRuntime.subtract ((a).exponent) ((LexLeanRuntime.negate ((b).exponent) : Int)) : Int)) ((c).exponent) : Int)) ((LexLeanRuntime.negate (shift) : Int)) : Int); (let abSign : Nat := (if (Nat.beq ((a).sign) ((b).sign)) then 0 else 1); (let outSign : Nat := (if (Nat.beq (abSign) ((c).sign)) then 0 else 1); (if (!(LexLeanCollections.lessThan (k) ((0 : Int)) : Bool)) then (let initQ : Int := (LexLeanRuntime.quotient (product) (divisor) ((0 : Int)) : Int); (let initR : Int := (LexLeanRuntime.remainder (product) (divisor) ((0 : Int)) : Int); (let stepped : (Prod ((Prod (Int) ((Prod (Int) (Int))))) (Bool)) := (LexLeanCollections.iterateUntil ((fun (st : (Prod (Int) ((Prod (Int) (Int))))) => (if (!(LexLeanCollections.lessThan ((0 : Int)) ((st).1) : Bool)) then Option.none else (let qr : (Prod (Int) (Int)) := (st).2; (let r2 : Int := (LexLeanRuntime.multiply ((2 : Int)) ((qr).2) : Int); (let dq : Int := (LexLeanRuntime.quotient (r2) (divisor) ((0 : Int)) : Int); (let nr : Int := (LexLeanRuntime.remainder (r2) (divisor) ((0 : Int)) : Int); Option.some (((LexLeanRuntime.subtract ((st).1) ((1 : Int)) : Int), ((LexLeanRuntime.subtract ((LexLeanRuntime.multiply ((2 : Int)) ((qr).1) : Int)) ((LexLeanRuntime.negate (dq) : Int)) : Int), nr)))))))))) (512) ((k, (initQ, initR))) : (Prod ((Prod (Int) ((Prod (Int) (Int))))) (Bool))); (if (stepped).2 then (let qr : (Prod (Int) (Int)) := ((stepped).1).2; (let q : Int := (qr).1; (let r : Int := (qr).2; (let twoR : Int := (LexLeanRuntime.multiply ((2 : Int)) (r) : Int); (let rounded : Int := (if (LexLeanCollections.lessThan (twoR) (divisor) : Bool) then q else (if (LexLeanCollections.lessThan (divisor) (twoR) : Bool) then (LexLeanRuntime.subtract (q) ((-1 : Int)) : Int) else (LexLeanRuntime.subtract (q) ((LexLeanRuntime.negate ((LexLeanRuntime.remainder (q) ((2 : Int)) ((0 : Int)) : Int)) : Int)) : Int))); Option.some ((if (Nat.beq (outSign) (0)) then rounded else (LexLeanRuntime.negate (rounded) : Int)))))))) else Option.none)))) else (let s : Int := (LexLeanRuntime.negate (k) : Int); (let initQ : Int := (LexLeanRuntime.quotient (product) (divisor) ((0 : Int)) : Int); (let initSticky : Int := (if (!(LexLeanCollections.lessThan ((0 : Int)) ((LexLeanRuntime.remainder (product) (divisor) ((0 : Int)) : Int)) : Bool)) then (0 : Int) else (1 : Int)); (let stepped : (Prod ((Prod (Int) ((Prod (Int) (Int))))) (Bool)) := (LexLeanCollections.iterateUntil ((fun (st : (Prod (Int) ((Prod (Int) (Int))))) => (if (!(LexLeanCollections.lessThan ((0 : Int)) ((st).1) : Bool)) then Option.none else (let qs : (Prod (Int) (Int)) := (st).2; Option.some (((LexLeanRuntime.subtract ((st).1) ((1 : Int)) : Int), ((LexLeanRuntime.quotient ((qs).1) ((2 : Int)) ((0 : Int)) : Int), (if ((!(LexLeanCollections.lessThan ((0 : Int)) ((qs).2) : Bool)) && (!(LexLeanCollections.lessThan ((0 : Int)) ((LexLeanRuntime.remainder ((qs).1) ((2 : Int)) ((0 : Int)) : Int)) : Bool))) then (0 : Int) else (1 : Int))))))))) (512) (((LexLeanRuntime.subtract (s) ((1 : Int)) : Int), (initQ, initSticky))) : (Prod ((Prod (Int) ((Prod (Int) (Int))))) (Bool))); (if (stepped).2 then (let qs : (Prod (Int) (Int)) := ((stepped).1).2; (let half : Int := (qs).1; (let sticky : Int := (qs).2; (let q : Int := (LexLeanRuntime.quotient (half) ((2 : Int)) ((0 : Int)) : Int); (let hb : Int := (LexLeanRuntime.remainder (half) ((2 : Int)) ((0 : Int)) : Int); (let rounded : Int := (if (!(LexLeanCollections.lessThan ((0 : Int)) (hb) : Bool)) then q else (if (LexLeanCollections.lessThan ((0 : Int)) (sticky) : Bool) then (LexLeanRuntime.subtract (q) ((-1 : Int)) : Int) else (LexLeanRuntime.subtract (q) ((LexLeanRuntime.negate ((LexLeanRuntime.remainder (q) ((2 : Int)) ((0 : Int)) : Int)) : Int)) : Int))); Option.some ((if (Nat.beq (outSign) (0)) then rounded else (LexLeanRuntime.negate (rounded) : Int))))))))) else Option.none)))))))))))))

@[expose] public def scaleRatioRound32 (aBits : Int) (bBits : Int) (cBits : Int) : Option (Int) := (match decodeBinary32 (aBits) with | Option.none => Option.none | Option.some a => (match decodeBinary32 (bBits) with | Option.none => Option.none | Option.some b => (match decodeBinary32 (cBits) with | Option.none => Option.none | Option.some c => dyadicRatioRound (a) (b) (c) ((32 : Int)))))

@[expose] public def scaleMultipliersRound32 (inputScaleBits : Int) (weightScaleBits : List (Int)) (outputScaleBits : Int) : Option (List (Int)) := (let rev : Option (List (Int)) := (LexLeanCollections.listFold ((fun (acc : Option (List (Int))) (wBits : Int) => (match acc with | Option.none => Option.none | Option.some xs => (match scaleRatioRound32 (inputScaleBits) (wBits) (outputScaleBits) with | Option.none => Option.none | Option.some m => Option.some ((m :: xs)))))) (Option.some (([] : List (Int)))) (weightScaleBits) : Option (List (Int))); (match rev with | Option.none => Option.none | Option.some xs => Option.some ((LexLeanCollections.listFold ((fun (out : List (Int)) (x : Int) => (x :: out))) (([] : List (Int))) (xs) : List (Int)))))

public theorem binary32_pos_zero : DyadicMatches (decodeBinary32 ((0 : Int))) (0) ((0 : Int)) ((-149 : Int)) := by
  decide

public theorem binary32_neg_zero : DyadicMatches (decodeBinary32 ((2147483648 : Int))) (1) ((0 : Int)) ((-149 : Int)) := by
  decide

public theorem binary32_min_subnormal : DyadicMatches (decodeBinary32 ((1 : Int))) (0) ((1 : Int)) ((-149 : Int)) := by
  decide

public theorem binary32_max_subnormal : DyadicMatches (decodeBinary32 ((8388607 : Int))) (0) ((8388607 : Int)) ((-149 : Int)) := by
  decide

public theorem binary32_min_normal : DyadicMatches (decodeBinary32 ((8388608 : Int))) (0) ((8388608 : Int)) ((-149 : Int)) := by
  decide

public theorem binary32_one : DyadicMatches (decodeBinary32 ((1065353216 : Int))) (0) ((8388608 : Int)) ((-23 : Int)) := by
  decide

public theorem binary32_neg_one_half : DyadicMatches (decodeBinary32 ((3217031168 : Int))) (1) ((12582912 : Int)) ((-23 : Int)) := by
  decide

public theorem binary32_unit_scale : DyadicMatches (decodeBinary32 ((998277249 : Int))) (0) ((8421505 : Int)) ((-31 : Int)) := by
  decide

public theorem binary32_max_normal : DyadicMatches (decodeBinary32 ((2139095039 : Int))) (0) ((16777215 : Int)) ((104 : Int)) := by
  decide

public theorem binary32_rejects_pos_inf : (dyadicTriple (decodeBinary32 ((2139095040 : Int))) = Option.none) := by
  decide

public theorem binary32_rejects_neg_inf : (dyadicTriple (decodeBinary32 ((4286578688 : Int))) = Option.none) := by
  decide

public theorem binary32_rejects_nan : (dyadicTriple (decodeBinary32 ((2143289344 : Int))) = Option.none) := by
  decide

public theorem binary32_rejects_negative_input : (dyadicTriple (decodeBinary32 ((-1 : Int))) = Option.none) := by
  decide

public theorem binary32_rejects_overflow_input : (dyadicTriple (decodeBinary32 ((4294967296 : Int))) = Option.none) := by
  decide

public theorem binary32_banner_subnormal : DyadicMatches (decodeBinary32Bytes (banner) (0)) (0) ((5065804 : Int)) ((-149 : Int)) := by
  decide

public theorem binary32_banner_normal : DyadicMatches (decodeBinary32Bytes (banner) (1)) (0) ((8408396 : Int)) ((-148 : Int)) := by
  decide

public theorem binary64_pos_zero : DyadicMatches (decodeBinary64 ((0 : Int))) (0) ((0 : Int)) ((-1074 : Int)) := by
  decide

public theorem binary64_neg_zero : DyadicMatches (decodeBinary64 ((-9223372036854775808 : Int))) (1) ((0 : Int)) ((-1074 : Int)) := by
  decide

public theorem binary64_max_subnormal : DyadicMatches (decodeBinary64 ((4503599627370495 : Int))) (0) ((4503599627370495 : Int)) ((-1074 : Int)) := by
  decide

public theorem binary64_min_normal : DyadicMatches (decodeBinary64 ((4503599627370496 : Int))) (0) ((4503599627370496 : Int)) ((-1074 : Int)) := by
  decide

public theorem binary64_min_subnormal : DyadicMatches (decodeBinary64 ((1 : Int))) (0) ((1 : Int)) ((-1074 : Int)) := by
  decide

public theorem binary64_one : DyadicMatches (decodeBinary64 ((4607182418800017408 : Int))) (0) ((4503599627370496 : Int)) ((-52 : Int)) := by
  decide

public theorem binary64_neg_one_half : DyadicMatches (decodeBinary64 ((-4613937818241073152 : Int))) (1) ((6755399441055744 : Int)) ((-52 : Int)) := by
  decide

public theorem binary64_max_normal : DyadicMatches (decodeBinary64 ((9218868437227405311 : Int))) (0) ((9007199254740991 : Int)) ((971 : Int)) := by
  decide

public theorem binary64_rejects_pos_inf : (dyadicTriple (decodeBinary64 ((9218868437227405312 : Int))) = Option.none) := by
  decide

public theorem binary64_rejects_neg_inf : (dyadicTriple (decodeBinary64 ((-4503599627370496 : Int))) = Option.none) := by
  decide

public theorem binary64_rejects_nan : (dyadicTriple (decodeBinary64 ((9221120237041090560 : Int))) = Option.none) := by
  decide

public theorem binary64_banner_first : DyadicMatches (decodeBinary64Bytes ((LexLeanRuntime.append (banner) (banner) : ByteArray)) (0)) (0) ((7964866531642444 : Int)) ((161 : Int)) := by
  decide

public theorem binary64_banner_last : DyadicMatches (decodeBinary64Bytes ((LexLeanRuntime.append (banner) (banner) : ByteArray)) (2)) (0) ((4588589715357773 : Int)) ((-1059 : Int)) := by
  decide

public theorem dyadic_round_shift_exact : (dyadicRoundShift (({ sign := 0, mantissa := (8388608 : Int), exponent := (-150 : Int) } : Dyadic)) ((150 : Int)) = (8388608 : Int)) := by
  decide

public theorem dyadic_round_shift_tie_even_down : (dyadicRoundShift (({ sign := 0, mantissa := (5 : Int), exponent := (-1 : Int) } : Dyadic)) ((0 : Int)) = (2 : Int)) := by
  decide

public theorem dyadic_round_shift_tie_even_up : (dyadicRoundShift (({ sign := 0, mantissa := (7 : Int), exponent := (-1 : Int) } : Dyadic)) ((0 : Int)) = (4 : Int)) := by
  decide

public theorem dyadic_round_shift_negative_tie_even : (dyadicRoundShift (({ sign := 1, mantissa := (7 : Int), exponent := (-1 : Int) } : Dyadic)) ((0 : Int)) = (-4 : Int)) := by
  decide

public theorem scale_ratio_exact_cancel : (scaleRatioRound32 ((998277249 : Int)) ((1000593162 : Int)) ((1008981770 : Int)) = Option.some ((8421505 : Int))) := by
  decide

public theorem scale_ratio_rounds_up : (scaleRatioRound32 ((998277249 : Int)) ((1000693826 : Int)) ((1017370378 : Int)) = Option.some ((4250229 : Int))) := by
  decide

public theorem scale_ratio_rounds_down : (scaleRatioRound32 ((1023410176 : Int)) ((1017370378 : Int)) ((1039516303 : Int)) = Option.some ((22369621 : Int))) := by
  decide

public theorem scale_ratio_tie_odd_to_even : (scaleRatioRound32 ((1065353217 : Int)) ((1069547520 : Int)) ((1140850688 : Int)) = Option.some ((12582914 : Int))) := by
  decide

public theorem scale_ratio_tie_even_stays_even : (scaleRatioRound32 ((1065353219 : Int)) ((1069547520 : Int)) ((1140850688 : Int)) = Option.some ((12582916 : Int))) := by
  decide

public theorem scale_ratio_negative_scale : (scaleRatioRound32 ((3145760897 : Int)) ((1000693826 : Int)) ((1017370378 : Int)) = Option.some ((-4250229 : Int))) := by
  decide

public theorem scale_ratio_subnormal_scales : (scaleRatioRound32 ((3 : Int)) ((1084227584 : Int)) ((2 : Int)) = Option.some ((32212254720 : Int))) := by
  decide

public theorem scale_ratio_rejects_zero_divisor : (scaleRatioRound32 ((1065353216 : Int)) ((1065353216 : Int)) ((0 : Int)) = Option.none) := by
  decide

public theorem scale_ratio_rejects_nonfinite : (scaleRatioRound32 ((1065353216 : Int)) ((2139095040 : Int)) ((1065353216 : Int)) = Option.none) := by
  decide

public theorem scale_multipliers_channel_batch : (scaleMultipliersRound32 ((998277249 : Int)) (((1000593162 : Int) :: ((1000693826 : Int) :: ([] : List (Int))))) ((1017370378 : Int)) = Option.some (((4210752 : Int) :: ((4250229 : Int) :: ([] : List (Int)))))) := by
  decide

end Models.Recognizer
