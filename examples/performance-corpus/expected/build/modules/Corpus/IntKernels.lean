module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Corpus.IntKernels

@[expose] public def clamp (value : Int) (low : Int) (high : Int) : Int := (if (LexLeanCollections.lessThan (value) (low) : Bool) then low else (if (LexLeanCollections.lessThan (high) (value) : Bool) then high else value))

@[expose] public def requantize (x : Int) (offset : Int) (multiplier : Int) (divisor : Int) (low : Int) (high : Int) : Int := (let shifted : Int := (LexLeanRuntime.subtract (x) ((LexLeanRuntime.negate (offset) : Int)) : Int); (let scaled : Int := (LexLeanRuntime.subtract ((LexLeanRuntime.multiply (shifted) (multiplier) : Int)) ((LexLeanRuntime.negate ((LexLeanRuntime.quotient (divisor) ((2 : Int)) ((0 : Int)) : Int)) : Int)) : Int); clamp ((LexLeanRuntime.quotient (scaled) (divisor) ((0 : Int)) : Int)) (low) (high)))

public theorem requantize_mid : (requantize ((100 : Int)) ((20 : Int)) ((9365133 : Int)) ((4294967296 : Int)) ((0 : Int)) ((255 : Int)) = (0 : Int)) := by
  decide

public theorem requantize_clamps_high : (requantize ((7 : Int)) ((0 : Int)) ((1000 : Int)) ((1 : Int)) ((0 : Int)) ((255 : Int)) = (255 : Int)) := by
  decide

end Corpus.IntKernels
