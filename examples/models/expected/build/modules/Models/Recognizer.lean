module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanRuntime.Models
public import Models.Glyphs
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Models.Recognizer

@[expose] public def hiddenWeights.bytes : ByteArray := ByteArray.mk #[4, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 4, 3, 254, 0, 0, 0, 0, 255, 0, 0, 2, 253, 1, 0, 0, 255, 0, 0, 0, 2, 2, 254, 1, 1, 1, 1, 1, 1, 1, 0, 252, 0, 2, 0, 3, 0]
@[expose] public def hiddenWeights : List (List (Int)) := (((4 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int))))))))) :: (((0 : Int) :: ((4 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int))))))))) :: (((0 : Int) :: ((0 : Int) :: ((4 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int))))))))) :: (((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((4 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int))))))))) :: (((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((4 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int))))))))) :: (((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((4 : Int) :: ((0 : Int) :: ([] : List (Int))))))))) :: (((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((4 : Int) :: ([] : List (Int))))))))) :: (((3 : Int) :: ((-2 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((-1 : Int) :: ([] : List (Int))))))))) :: (((0 : Int) :: ((0 : Int) :: ((2 : Int) :: ((-3 : Int) :: ((1 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int))))))))) :: (((-1 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((2 : Int) :: ((2 : Int) :: ((-2 : Int) :: ([] : List (Int))))))))) :: (((1 : Int) :: ((1 : Int) :: ((1 : Int) :: ((1 : Int) :: ((1 : Int) :: ((1 : Int) :: ((1 : Int) :: ([] : List (Int))))))))) :: (((0 : Int) :: ((-4 : Int) :: ((0 : Int) :: ((2 : Int) :: ((0 : Int) :: ((3 : Int) :: ((0 : Int) :: ([] : List (Int))))))))) :: ([] : List (List (Int)))))))))))))))
public theorem hiddenWeights.decoded : (LexLeanModels.tensorMatches 1 true hiddenWeights.flatten hiddenWeights.bytes && (hiddenWeights.length == 12) && (hiddenWeights.map List.length == List.replicate 12 7)) = true := by decide

@[expose] public def hiddenBias.bytes : ByteArray := ByteArray.mk #[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 254, 255, 255, 255, 3, 0, 0, 0, 251, 255, 255, 255, 2, 0, 0, 0]
@[expose] public def hiddenBias : List (Int) := ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((1 : Int) :: ((-2 : Int) :: ((3 : Int) :: ((-5 : Int) :: ((2 : Int) :: ([] : List (Int))))))))))))))
public theorem hiddenBias.decoded : (LexLeanModels.tensorMatches 4 true hiddenBias hiddenBias.bytes && (hiddenBias.length == 12)) = true := by decide

@[expose] public def outputWeights.bytes : ByteArray := ByteArray.mk #[2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2, 0, 2, 0, 0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 2, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 2, 2, 0, 0, 0, 0, 0, 2, 0, 2, 2, 0, 2, 2, 0, 0, 0, 0, 0, 2, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 2, 2, 2, 2, 0, 2, 2, 0, 0, 0, 0, 0]
@[expose] public def outputWeights : List (List (Int)) := (((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int)))))))))))))) :: (((0 : Int) :: ((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int)))))))))))))) :: (((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ((2 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int)))))))))))))) :: (((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ((0 : Int) :: ((2 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int)))))))))))))) :: (((0 : Int) :: ((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ((0 : Int) :: ((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int)))))))))))))) :: (((2 : Int) :: ((0 : Int) :: ((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int)))))))))))))) :: (((2 : Int) :: ((0 : Int) :: ((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int)))))))))))))) :: (((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int)))))))))))))) :: (((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int)))))))))))))) :: (((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ((2 : Int) :: ((2 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int)))))))))))))) :: ([] : List (List (Int)))))))))))))
public theorem outputWeights.decoded : (LexLeanModels.tensorMatches 1 true outputWeights.flatten outputWeights.bytes && (outputWeights.length == 10) && (outputWeights.map List.length == List.replicate 10 12)) = true := by decide

@[expose] public def outputBias.bytes : ByteArray := ByteArray.mk #[250, 255, 255, 255, 254, 255, 255, 255, 251, 255, 255, 255, 251, 255, 255, 255, 252, 255, 255, 255, 251, 255, 255, 255, 250, 255, 255, 255, 253, 255, 255, 255, 249, 255, 255, 255, 250, 255, 255, 255]
@[expose] public def outputBias : List (Int) := ((-6 : Int) :: ((-2 : Int) :: ((-5 : Int) :: ((-5 : Int) :: ((-4 : Int) :: ((-5 : Int) :: ((-6 : Int) :: ((-3 : Int) :: ((-7 : Int) :: ((-6 : Int) :: ([] : List (Int))))))))))))
public theorem outputBias.decoded : (LexLeanModels.tensorMatches 4 true outputBias outputBias.bytes && (outputBias.length == 10)) = true := by decide

@[expose] public def digitNames.bytes : ByteArray := ByteArray.mk #[122, 101, 114, 111, 10, 111, 110, 101, 10, 116, 119, 111, 10, 116, 104, 114, 101, 101, 10, 102, 111, 117, 114, 10, 102, 105, 118, 101, 10, 115, 105, 120, 10, 115, 101, 118, 101, 110, 10, 101, 105, 103, 104, 116, 10, 110, 105, 110, 101, 10]
@[expose] public def digitNames : List (String) := ("zero" :: ("one" :: ("two" :: ("three" :: ("four" :: ("five" :: ("six" :: ("seven" :: ("eight" :: ("nine" :: ([] : List (String))))))))))))
public theorem digitNames.decoded : LexLeanModels.linesMatch digitNames digitNames.bytes = true := by decide

@[expose] public def banner.bytes : ByteArray := ByteArray.mk #[76, 76, 77, 0, 1]
@[expose] public def banner : ByteArray := ByteArray.mk #[76, 76, 77, 0, 1]
public theorem banner.decoded : banner = banner.bytes := rfl

@[expose, reducible] public def Recognizes (glyph : Models.Glyphs.Glyph) (digit : Nat) : Prop := (digit = Models.Glyphs.digitOf (glyph))

@[expose] public def recognizesCheck (glyph : Models.Glyphs.Glyph) (digit : Nat) : Bool := (Nat.beq (digit) (Models.Glyphs.digitOf (glyph)))

public theorem recognizes_check_sound (glyph : Models.Glyphs.Glyph) (digit : Nat) : ((recognizesCheck (glyph) (digit) = true) -> Recognizes (glyph) (digit)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Recognizes, recognizesCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem recognizes_check_complete (glyph : Models.Glyphs.Glyph) (digit : Nat) : (Recognizes (glyph) (digit) -> (recognizesCheck (glyph) (digit) = true)) := by
  intros
  subst_vars
  try set_option linter.unusedSimpArgs false in simp only [Recognizes, recognizesCheck, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
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
  try set_option linter.unusedSimpArgs false in simp only [sameDigit, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq, LexLeanRuntime.subtract, LexLeanRuntime.multiply] at *
  all_goals omega

public theorem labeled_agreement : (((LexLeanRuntime.length (labeledGlyphs) : Nat) = 40) /\ ((LexLeanCollections.listFold ((fun (count : Nat) («example» : (Prod (Models.Glyphs.Glyph) (Nat))) => (if sameDigit (DigitNet ((«example»).1)) ((«example»).2) then (count + 1) else count))) (0) (labeledGlyphs) : Nat) = 37)) := by
  decide

public theorem DigitEvidence.labeled_agreement : LexLeanModels.Agreement ((Models.Recognizer.DigitNet)) ((Models.Recognizer.sameDigit)) (Models.Recognizer.labeledGlyphs) (40) (37) := by decide
public theorem DigitEvidence.digit_net_exact : LexLeanModels.EquivalentTotal ((Models.Recognizer.DigitNet)) ((Models.Glyphs.digitOf)) := Models.Recognizer.digit_net_exact
public theorem DigitEvidence.digit_net_correct : LexLeanModels.SatisfiesTotal ((Models.Recognizer.Recognizes)) ((Models.Recognizer.DigitNet)) := Models.Recognizer.digit_net_correct

@[expose] public def DigitModel (__input : Models.Glyphs.Glyph) : Nat := Models.Recognizer.DigitNet (__input)

@[expose] public def RawDigitModel (__input : Models.Glyphs.Glyph) : Nat := Models.Recognizer.DigitNet (__input)

@[expose] public def digitName (digit : Nat) : Option (String) := (LexLeanRuntime.index (digitNames) (digit) : Option (String))

end Models.Recognizer
