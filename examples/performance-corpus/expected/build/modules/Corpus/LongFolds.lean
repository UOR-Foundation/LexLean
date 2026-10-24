module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Corpus.LongFolds

@[expose] public def moments (xs : List (Nat)) : (Prod (Nat) ((Prod (Nat) (Nat)))) := (LexLeanCollections.listFold ((fun (acc : (Prod (Nat) ((Prod (Nat) (Nat))))) (v : Nat) => (((acc).1 + 1), ((((acc).2).1 + v), (((acc).2).2 + (LexLeanRuntime.multiply (v) (v) : Nat)))))) ((0, (0, 0))) (xs) : (Prod (Nat) ((Prod (Nat) (Nat)))))

public theorem moments_three : (moments ((1 :: (2 :: (3 :: ([] : List (Nat)))))) = (3, (6, 14))) := by
  decide

public theorem moments_empty : (moments (([] : List (Nat))) = (0, (0, 0))) := by
  decide

end Corpus.LongFolds
