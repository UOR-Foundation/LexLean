module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Collections.Tables

@[expose] public def declare (table : List (Prod (String) (Nat))) (name : String) (slot : Nat) : List (Prod (String) (Nat)) := (LexLeanCollections.mapInsert (table) (name) (slot) : List (Prod (String) (Nat)))

@[expose] public def buildTable (names : List (String)) : List (Prod (String) (Nat)) := (LexLeanCollections.listFold ((fun (table : List (Prod (String) (Nat))) (name : String) => declare (table) (name) ((LexLeanCollections.mapSize (table) : Nat)))) (([] : List (Prod (String) (Nat)))) (names) : List (Prod (String) (Nat)))

@[expose] public def callGraph : List (Prod (String) (List (String))) := ([("emit", ["report"]), ("lex", []), ("main", ["emit", "parse"]), ("parse", ["lex"]), ("report", ["lex"])] : List (Prod (String) (List (String))))

@[expose] public def callOrder : Option (List (String)) := (LexLeanCollections.graphTopological (callGraph) : Option (List (String)))

@[expose] public def reachableFromParse : List (String) := (LexLeanCollections.graphReachable (callGraph) ("parse") : List (String))

end Collections.Tables
