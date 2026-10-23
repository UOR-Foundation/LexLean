module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Corpus.MapsSets

@[expose] public def count (counts : List (Prod (Nat) (Nat))) (v : Nat) : Nat := (match (LexLeanCollections.mapLookup (counts) (v) : Option (Nat)) with | Option.none => 0 | Option.some n => n)

@[expose] public def tally (xs : List (Nat)) (key : Nat) : (Prod (Nat) (Nat)) := (let counts : List (Prod (Nat) (Nat)) := (LexLeanCollections.listFold ((fun (m : List (Prod (Nat) (Nat))) (v : Nat) => (LexLeanCollections.mapInsert (m) (v) ((count (m) (v) + 1)) : List (Prod (Nat) (Nat))))) (([] : List (Prod (Nat) (Nat)))) (xs) : List (Prod (Nat) (Nat))); (let seen : List (Nat) := (LexLeanCollections.listFold ((fun (s : List (Nat)) (v : Nat) => (LexLeanCollections.setInsert (s) (v) : List (Nat)))) (([] : List (Nat))) (xs) : List (Nat)); (count (counts) (key), (LexLeanCollections.setSize (seen) : Nat))))

public theorem tally_repeats : (tally ((2 :: (1 :: (2 :: (3 :: ([] : List (Nat))))))) (2) = (2, 3)) := by
  decide

public theorem tally_absent : (tally ((1 :: ([] : List (Nat)))) (2) = (0, 1)) := by
  decide

end Corpus.MapsSets
