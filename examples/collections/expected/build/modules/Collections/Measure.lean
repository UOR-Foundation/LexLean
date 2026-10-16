module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import Collections.Tables
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Collections.Measure

@[expose] public def distinctNames : Nat := (LexLeanCollections.mapSize (Collections.Tables.buildTable (("a" :: ("b" :: ("a" :: ([] : List (String))))))) : Nat)

public theorem distinct_names : (distinctNames = 2) := by
  decide

end Collections.Measure
