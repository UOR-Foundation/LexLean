module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Coverage.Colls

@[expose] public def mapOps (m : List (Prod (String) (Nat))) (k : String) (v : Nat) : (Prod (List (Prod (String) (Nat))) ((Prod (List (Prod (String) (Nat))) ((Prod (Option (Nat)) ((Prod (Bool) ((Prod (List (String)) ((Prod (List (Nat)) ((Prod (Nat) (List ((Prod (String) (Nat))))))))))))))))) := ((LexLeanCollections.mapInsert (m) (k) (v) : List (Prod (String) (Nat))), ((LexLeanCollections.mapRemove (m) (k) : List (Prod (String) (Nat))), ((LexLeanCollections.mapLookup (m) (k) : Option (Nat)), ((LexLeanCollections.mapContains (m) (k) : Bool), ((LexLeanCollections.mapKeys (m) : List (String)), ((LexLeanCollections.mapValues (m) : List (Nat)), ((LexLeanCollections.mapSize (m) : Nat), (LexLeanCollections.mapEntries (m) : List ((Prod (String) (Nat)))))))))))

@[expose] public def mapFolding (m : List (Prod (Nat) (Nat))) (bonus : Nat) : Nat := (LexLeanCollections.mapFold ((fun (acc : Nat) (key : Nat) (value : Nat) => ((acc + key) + (value + bonus)))) (0) (m) : Nat)

@[expose] public def setOps (s : List (Nat)) (t : List (Nat)) (x : Nat) : (Prod (List (Nat)) ((Prod (List (Nat)) ((Prod (Bool) ((Prod (List (Nat)) ((Prod (List (Nat)) ((Prod (List (Nat)) ((Prod (Nat) (List (Nat))))))))))))))) := ((LexLeanCollections.setInsert (s) (x) : List (Nat)), ((LexLeanCollections.setRemove (s) (x) : List (Nat)), ((LexLeanCollections.setContains (s) (x) : Bool), ((LexLeanCollections.setUnion (s) (t) : List (Nat)), ((LexLeanCollections.setIntersection (s) (t) : List (Nat)), ((LexLeanCollections.setDifference (s) (t) : List (Nat)), ((LexLeanCollections.setSize (s) : Nat), (LexLeanCollections.setElements (s) : List (Nat)))))))))

@[expose] public def folds (xs : List (Nat)) (s : List (Int)) : (Prod (Nat) (Int)) := ((LexLeanCollections.listFold ((fun (a : Nat) (x : Nat) => (a + x))) (0) (xs) : Nat), (LexLeanCollections.setFold ((fun (a : Int) (x : Int) => (LexLeanRuntime.subtract (a) (x) : Int))) ((5 : Int)) (s) : Int))

@[expose] public def iterations (n : Nat) (start : Nat) : (Prod (Nat) ((Prod (Nat) (Bool)))) := ((LexLeanCollections.iterate ((fun (x : Nat) => (x + 1))) (n) (start) : Nat), (LexLeanCollections.iterateUntil ((fun (x : Nat) => (if (Nat.blt (x) (10)) then Option.some ((x + 2)) else Option.none))) (n) (start) : (Prod (Nat) (Bool))))

@[expose] public def graphs (g : List (Prod (Nat) (List (Nat)))) (start : Nat) : (Prod (List (Nat)) ((Prod (List (Nat)) (Option (List (Nat)))))) := ((LexLeanCollections.graphSuccessors (g) (start) : List (Nat)), ((LexLeanCollections.graphReachable (g) (start) : List (Nat)), (LexLeanCollections.graphTopological (g) : Option (List (Nat)))))

@[expose] public def stringGraph (g : List (Prod (String) (List (String)))) : Option (List (String)) := (LexLeanCollections.graphTopological (g) : Option (List (String)))

@[expose] public def keyTypes (a : List (Prod ((Prod (Nat) (String))) (Bool))) (k : (Prod (Nat) (String))) (i : List (Prod (Int8) (Nat))) (j : Int8) (b : List (Prod (Bool) (Nat))) (c : Bool) (d : List (Prod (Int) (Nat))) (e : Int) (u : List (Prod (UInt64) (Nat))) (w : UInt64) : (Prod (Option (Bool)) ((Prod (Option (Nat)) ((Prod (Option (Nat)) ((Prod (Option (Nat)) (Option (Nat))))))))) := ((LexLeanCollections.mapLookup (a) (k) : Option (Bool)), ((LexLeanCollections.mapLookup (i) (j) : Option (Nat)), ((LexLeanCollections.mapLookup (b) (c) : Option (Nat)), ((LexLeanCollections.mapLookup (d) (e) : Option (Nat)), (LexLeanCollections.mapLookup (u) (w) : Option (Nat))))))

@[expose] public def literals (x : Nat) (_s : String) : (Prod (List (Prod (String) (Nat))) ((Prod (List (Nat)) (List (Prod (Nat) (List (Nat))))))) := (([("a", x), ("b", 2)] : List (Prod (String) (Nat))), (([1, 3] : List (Nat)), ([(1, [2]), (2, [1])] : List (Prod (Nat) (List (Nat))))))

end Coverage.Colls
