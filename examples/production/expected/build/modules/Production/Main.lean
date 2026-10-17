module
public import Init
public import LexLeanRuntime.Portable
public import Production.Kernel
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Production.Main

@[expose] public def checkedSum (left : UInt32) (right : UInt32) : Option (UInt32) := (LexLeanRuntime.checkedAdd (left) (right) : Option (UInt32))

@[expose] public def shapeArea (width : Nat) (height : Nat) : Nat := Production.Kernel.area (Production.Kernel.Shape.rectangle (width) (height))

@[expose] public def quadruple (value : Nat) : Nat := Production.Kernel.applyTwice ((fun (number : Nat) => (number + number))) (value)

@[expose] public def halvings (number : Nat) : Nat := Production.Kernel.countdown (number) (0)

@[expose] public def sumAll (values : List (Nat)) : Nat := Production.Kernel.total (values)

@[expose] public def headOr (values : List (Nat)) : Nat := Production.Kernel.firstOr (Nat) (values) (0)

public theorem quadruple_three : (quadruple (3) = 12) := by
  decide

public theorem sum_three : (sumAll ((1 :: (2 :: (3 :: ([] : List (Nat)))))) = 6) := by
  decide

end Production.Main
