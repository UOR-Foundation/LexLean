module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Coverage.Boundary

public inductive Grove where
  | node (_ : List (Nat)) (_ : List (Grove)) (_ : List (Prod (String) (Int))) (_ : Option (Grove)) (_ : List ((Prod (Nat) (Grove))))
  | leaf (_ : List (Prod (Nat) (Int)))
  | settled (_ : Except (Nat) (Grove))

public structure Ledger where
  entries : List (Prod (String) (Int))
  owner : String

@[expose] public def groveTags (g : Grove) : Nat := (match g with | Grove.node tags _ _ _ _ => (LexLeanCollections.setSize (tags) : Nat) | Grove.leaf weights => (LexLeanCollections.mapSize (weights) : Nat) | Grove.settled _ => 0)

@[expose] public def groveRoot (g : Grove) (ledger : Ledger) (picks : Option (List (Nat))) (spare : List (List (Prod (Nat) (Bool)))) : (Prod (Nat) ((Prod (Ledger) ((Prod (Option (List (Nat))) (List (List (Prod (Nat) (Bool))))))))) := (groveTags (g), (ledger, (picks, spare)))

end Coverage.Boundary
