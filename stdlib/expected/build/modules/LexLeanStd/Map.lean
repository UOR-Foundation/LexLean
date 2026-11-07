module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Order
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanStd.Map

@[expose] public def reverseEntries (entries : List ((Prod (ByteArray) (ByteArray)))) : List ((Prod (ByteArray) (ByteArray))) := (LexLeanCollections.listFold ((fun (reversed : List ((Prod (ByteArray) (ByteArray)))) (entry : (Prod (ByteArray) (ByteArray))) => (entry :: reversed))) (([] : List ((Prod (ByteArray) (ByteArray))))) (entries) : List ((Prod (ByteArray) (ByteArray))))

@[expose] public def reverseKeys (items : List (ByteArray)) : List (ByteArray) := (LexLeanCollections.listFold ((fun (reversed : List (ByteArray)) (item : ByteArray) => (item :: reversed))) (([] : List (ByteArray))) (items) : List (ByteArray))

@[expose] public def lastWins (kept : List ((Prod (ByteArray) (ByteArray)))) (entry : (Prod (ByteArray) (ByteArray))) : List ((Prod (ByteArray) (ByteArray))) := (match kept with | List.nil => (entry :: kept) | List.cons head rest => (if (LexLeanRuntime.equal ((head).1) ((entry).1) : Bool) then (entry :: rest) else (entry :: kept)))

@[expose] public def mapFromEntries (entries : List ((Prod (ByteArray) (ByteArray)))) : List ((Prod (ByteArray) (ByteArray))) := reverseEntries ((LexLeanCollections.listFold ((fun (kept : List ((Prod (ByteArray) (ByteArray)))) (entry : (Prod (ByteArray) (ByteArray))) => lastWins (kept) (entry))) (([] : List ((Prod (ByteArray) (ByteArray))))) (LexLeanStd.Order.sortEntries (entries)) : List ((Prod (ByteArray) (ByteArray)))))

@[expose] public def insertEntryStep (fresh : (Prod (ByteArray) (ByteArray))) (state : (Prod (Bool) (List ((Prod (ByteArray) (ByteArray)))))) (entry : (Prod (ByteArray) (ByteArray))) : (Prod (Bool) (List ((Prod (ByteArray) (ByteArray))))) := (match (state).1 with | Bool.true => (true, (entry :: (state).2)) | Bool.false => (if (LexLeanRuntime.equal ((fresh).1) ((entry).1) : Bool) then (true, (fresh :: (state).2)) else (if LexLeanStd.Order.lessBytes ((fresh).1) ((entry).1) then (true, (entry :: (fresh :: (state).2))) else (false, (entry :: (state).2)))))

@[expose] public def mapInsert (entries : List ((Prod (ByteArray) (ByteArray)))) (key : ByteArray) (value : ByteArray) : List ((Prod (ByteArray) (ByteArray))) := (let fresh : (Prod (ByteArray) (ByteArray)) := (key, value); (let done : (Prod (Bool) (List ((Prod (ByteArray) (ByteArray))))) := (LexLeanCollections.listFold ((fun (state : (Prod (Bool) (List ((Prod (ByteArray) (ByteArray)))))) (entry : (Prod (ByteArray) (ByteArray))) => insertEntryStep (fresh) (state) (entry))) ((false, ([] : List ((Prod (ByteArray) (ByteArray)))))) (entries) : (Prod (Bool) (List ((Prod (ByteArray) (ByteArray)))))); (match (done).1 with | Bool.true => reverseEntries ((done).2) | Bool.false => reverseEntries ((fresh :: (done).2)))))

@[expose] public def getStep (key : ByteArray) (found : Option (ByteArray)) (entry : (Prod (ByteArray) (ByteArray))) : Option (ByteArray) := (if (LexLeanRuntime.equal (key) ((entry).1) : Bool) then Option.some ((entry).2) else found)

@[expose] public def mapGet (entries : List ((Prod (ByteArray) (ByteArray)))) (key : ByteArray) : Option (ByteArray) := (LexLeanCollections.listFold ((fun (found : Option (ByteArray)) (entry : (Prod (ByteArray) (ByteArray))) => getStep (key) (found) (entry))) (Option.none) (entries) : Option (ByteArray))

@[expose] public def removeEntryStep (key : ByteArray) (kept : List ((Prod (ByteArray) (ByteArray)))) (entry : (Prod (ByteArray) (ByteArray))) : List ((Prod (ByteArray) (ByteArray))) := (if (LexLeanRuntime.equal (key) ((entry).1) : Bool) then kept else (entry :: kept))

@[expose] public def mapRemove (entries : List ((Prod (ByteArray) (ByteArray)))) (key : ByteArray) : List ((Prod (ByteArray) (ByteArray))) := reverseEntries ((LexLeanCollections.listFold ((fun (kept : List ((Prod (ByteArray) (ByteArray)))) (entry : (Prod (ByteArray) (ByteArray))) => removeEntryStep (key) (kept) (entry))) (([] : List ((Prod (ByteArray) (ByteArray))))) (entries) : List ((Prod (ByteArray) (ByteArray)))))

@[expose] public def mapExtend (entries : List ((Prod (ByteArray) (ByteArray)))) (other : List ((Prod (ByteArray) (ByteArray)))) : List ((Prod (ByteArray) (ByteArray))) := (LexLeanCollections.listFold ((fun (merged : List ((Prod (ByteArray) (ByteArray)))) (entry : (Prod (ByteArray) (ByteArray))) => mapInsert (merged) ((entry).1) ((entry).2))) (entries) (other) : List ((Prod (ByteArray) (ByteArray))))

@[expose] public def containsStep (key : ByteArray) (found : Bool) (item : ByteArray) : Bool := (if found then true else (LexLeanRuntime.equal (key) (item) : Bool))

@[expose] public def setContains (items : List (ByteArray)) (key : ByteArray) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (item : ByteArray) => containsStep (key) (found) (item))) (false) (items) : Bool)

@[expose] public def dropStep (key : ByteArray) (kept : List (ByteArray)) (item : ByteArray) : List (ByteArray) := (if (LexLeanRuntime.equal (key) (item) : Bool) then kept else (item :: kept))

@[expose] public def setRemove (items : List (ByteArray)) (key : ByteArray) : List (ByteArray) := reverseKeys ((LexLeanCollections.listFold ((fun (kept : List (ByteArray)) (item : ByteArray) => dropStep (key) (kept) (item))) (([] : List (ByteArray))) (items) : List (ByteArray)))

@[expose] public def setUnion (left : List (ByteArray)) (right : List (ByteArray)) : List (ByteArray) := (LexLeanCollections.listFold ((fun (merged : List (ByteArray)) (item : ByteArray) => LexLeanStd.Order.insertSorted (merged) (item))) (left) (right) : List (ByteArray))

@[expose] public def keepIfIn (other : List (ByteArray)) (kept : List (ByteArray)) (item : ByteArray) : List (ByteArray) := (if setContains (other) (item) then (item :: kept) else kept)

@[expose] public def setIntersection (left : List (ByteArray)) (right : List (ByteArray)) : List (ByteArray) := reverseKeys ((LexLeanCollections.listFold ((fun (kept : List (ByteArray)) (item : ByteArray) => keepIfIn (right) (kept) (item))) (([] : List (ByteArray))) (left) : List (ByteArray)))

@[expose] public def keepIfOut (other : List (ByteArray)) (kept : List (ByteArray)) (item : ByteArray) : List (ByteArray) := (if setContains (other) (item) then kept else (item :: kept))

@[expose] public def setDifference (left : List (ByteArray)) (right : List (ByteArray)) : List (ByteArray) := reverseKeys ((LexLeanCollections.listFold ((fun (kept : List (ByteArray)) (item : ByteArray) => keepIfOut (right) (kept) (item))) (([] : List (ByteArray))) (left) : List (ByteArray)))

end LexLeanStd.Map
