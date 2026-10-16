module
public import Init
public import LexLeanRuntime.Portable
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Models.Glyphs

public inductive Glyph where
  | g0
  | g1
  | g2
  | g3
  | g4
  | g5
  | g6
  | g7
  | g8
  | g9

@[expose] public def encode (glyph : Glyph) : List (Int) := (match glyph with | Glyph.g0 => ((1 : Int) :: ((1 : Int) :: ((1 : Int) :: ((1 : Int) :: ((1 : Int) :: ((1 : Int) :: ((0 : Int) :: ([] : List (Int))))))))) | Glyph.g1 => ((0 : Int) :: ((1 : Int) :: ((1 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int))))))))) | Glyph.g2 => ((1 : Int) :: ((1 : Int) :: ((0 : Int) :: ((1 : Int) :: ((1 : Int) :: ((0 : Int) :: ((1 : Int) :: ([] : List (Int))))))))) | Glyph.g3 => ((1 : Int) :: ((1 : Int) :: ((1 : Int) :: ((1 : Int) :: ((0 : Int) :: ((0 : Int) :: ((1 : Int) :: ([] : List (Int))))))))) | Glyph.g4 => ((0 : Int) :: ((1 : Int) :: ((1 : Int) :: ((0 : Int) :: ((0 : Int) :: ((1 : Int) :: ((1 : Int) :: ([] : List (Int))))))))) | Glyph.g5 => ((1 : Int) :: ((0 : Int) :: ((1 : Int) :: ((1 : Int) :: ((0 : Int) :: ((1 : Int) :: ((1 : Int) :: ([] : List (Int))))))))) | Glyph.g6 => ((1 : Int) :: ((0 : Int) :: ((1 : Int) :: ((1 : Int) :: ((1 : Int) :: ((1 : Int) :: ((1 : Int) :: ([] : List (Int))))))))) | Glyph.g7 => ((1 : Int) :: ((1 : Int) :: ((1 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ((0 : Int) :: ([] : List (Int))))))))) | Glyph.g8 => ((1 : Int) :: ((1 : Int) :: ((1 : Int) :: ((1 : Int) :: ((1 : Int) :: ((1 : Int) :: ((1 : Int) :: ([] : List (Int))))))))) | Glyph.g9 => ((1 : Int) :: ((1 : Int) :: ((1 : Int) :: ((1 : Int) :: ((0 : Int) :: ((1 : Int) :: ((1 : Int) :: ([] : List (Int))))))))))

@[expose] public def digitOf (glyph : Glyph) : Nat := (match glyph with | Glyph.g0 => 0 | Glyph.g1 => 1 | Glyph.g2 => 2 | Glyph.g3 => 3 | Glyph.g4 => 4 | Glyph.g5 => 5 | Glyph.g6 => 6 | Glyph.g7 => 7 | Glyph.g8 => 8 | Glyph.g9 => 9)

@[expose] public def segmentCount (glyph : Glyph) : Nat := (match glyph with | Glyph.g0 => 6 | Glyph.g1 => 2 | Glyph.g2 => 5 | Glyph.g3 => 5 | Glyph.g4 => 4 | Glyph.g5 => 5 | Glyph.g6 => 6 | Glyph.g7 => 3 | Glyph.g8 => 7 | Glyph.g9 => 6)

@[expose] public def isEight (glyph : Glyph) : Bool := (match glyph with | Glyph.g0 => false | Glyph.g1 => false | Glyph.g2 => false | Glyph.g3 => false | Glyph.g4 => false | Glyph.g5 => false | Glyph.g6 => false | Glyph.g7 => false | Glyph.g8 => true | Glyph.g9 => false)

public theorem encode_width (glyph : Glyph) : ((LexLeanRuntime.length (encode (glyph)) : Nat) = 7) := by
  cases glyph with
  | g0 =>
    rfl
  | g1 =>
    rfl
  | g2 =>
    rfl
  | g3 =>
    rfl
  | g4 =>
    rfl
  | g5 =>
    rfl
  | g6 =>
    rfl
  | g7 =>
    rfl
  | g8 =>
    rfl
  | g9 =>
    rfl

end Models.Glyphs
