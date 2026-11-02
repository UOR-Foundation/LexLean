module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanStd.Order

@[expose] public def lessBytes (left : ByteArray) (right : ByteArray) : Bool := (LexLeanRuntime.equal ((LexLeanRuntime.compareBytes (left) (right) : Ordering)) ((LexLeanRuntime.compareBytes (_root_.ByteArray.mk #[]) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 1))]) : Ordering)) : Bool)

@[expose] public def reverseList (Item : Type) (items : List (Item)) : List (Item) := (LexLeanCollections.listFold ((fun (reversed : List (Item)) (item : Item) => (item :: reversed))) (([] : List (Item))) (items) : List (Item))

@[expose] public def mergeStep (Value : Type) (state : (Prod (List ((Prod (ByteArray) (Value)))) ((Prod (List ((Prod (ByteArray) (Value)))) (List ((Prod (ByteArray) (Value)))))))) : Option ((Prod (List ((Prod (ByteArray) (Value)))) ((Prod (List ((Prod (ByteArray) (Value)))) (List ((Prod (ByteArray) (Value)))))))) := (let left : List ((Prod (ByteArray) (Value))) := (state).1; (let right : List ((Prod (ByteArray) (Value))) := ((state).2).1; (let merged : List ((Prod (ByteArray) (Value))) := ((state).2).2; (match left with | List.nil => (match right with | List.nil => Option.none | List.cons rightHead rightTail => Option.some ((([] : List ((Prod (ByteArray) (Value)))), (rightTail, (rightHead :: merged))))) | List.cons leftHead leftTail => (match right with | List.nil => Option.some ((leftTail, (([] : List ((Prod (ByteArray) (Value)))), (leftHead :: merged)))) | List.cons rightHead rightTail => (if lessBytes ((rightHead).1) ((leftHead).1) then Option.some ((left, (rightTail, (rightHead :: merged)))) else Option.some ((leftTail, (right, (leftHead :: merged))))))))))

@[expose] public def mergeEntries (Value : Type) (left : List ((Prod (ByteArray) (Value)))) (right : List ((Prod (ByteArray) (Value)))) : List ((Prod (ByteArray) (Value))) := (let fuel : Nat := (((LexLeanRuntime.length (left) : Nat) + (LexLeanRuntime.length (right) : Nat)) + 1); reverseList ((Prod (ByteArray) (Value))) (((((LexLeanCollections.iterateUntil ((fun (state : (Prod (List ((Prod (ByteArray) (Value)))) ((Prod (List ((Prod (ByteArray) (Value)))) (List ((Prod (ByteArray) (Value)))))))) => mergeStep (Value) (state))) (fuel) ((left, (right, ([] : List ((Prod (ByteArray) (Value))))))) : (Prod ((Prod (List ((Prod (ByteArray) (Value)))) ((Prod (List ((Prod (ByteArray) (Value)))) (List ((Prod (ByteArray) (Value)))))))) (Bool)))).1).2).2))

@[expose] public def passStep (Value : Type) (state : (Prod (List (List ((Prod (ByteArray) (Value))))) (List (List ((Prod (ByteArray) (Value))))))) : Option ((Prod (List (List ((Prod (ByteArray) (Value))))) (List (List ((Prod (ByteArray) (Value))))))) := (let runs : List (List ((Prod (ByteArray) (Value)))) := (state).1; (let merged : List (List ((Prod (ByteArray) (Value)))) := (state).2; (match runs with | List.nil => Option.none | List.cons run rest => (match rest with | List.nil => Option.some ((([] : List (List ((Prod (ByteArray) (Value))))), (run :: merged))) | List.cons next others => Option.some ((others, (mergeEntries (Value) (run) (next) :: merged)))))))

@[expose] public def mergePass (Value : Type) (runs : List (List ((Prod (ByteArray) (Value))))) : List (List ((Prod (ByteArray) (Value)))) := reverseList (List ((Prod (ByteArray) (Value)))) ((((LexLeanCollections.iterateUntil ((fun (state : (Prod (List (List ((Prod (ByteArray) (Value))))) (List (List ((Prod (ByteArray) (Value))))))) => passStep (Value) (state))) (((LexLeanRuntime.length (runs) : Nat) + 1)) ((runs, ([] : List (List ((Prod (ByteArray) (Value))))))) : (Prod ((Prod (List (List ((Prod (ByteArray) (Value))))) (List (List ((Prod (ByteArray) (Value))))))) (Bool)))).1).2)

@[expose] public def sortStep (Value : Type) (runs : List (List ((Prod (ByteArray) (Value))))) : Option (List (List ((Prod (ByteArray) (Value))))) := (if (Nat.ble ((LexLeanRuntime.length (runs) : Nat)) (1)) then Option.none else Option.some (mergePass (Value) (runs)))

@[expose] public def sortByKey (Value : Type) (entries : List ((Prod (ByteArray) (Value)))) : List ((Prod (ByteArray) (Value))) := (let runs : List (List ((Prod (ByteArray) (Value)))) := reverseList (List ((Prod (ByteArray) (Value)))) ((LexLeanCollections.listFold ((fun (singles : List (List ((Prod (ByteArray) (Value))))) (entry : (Prod (ByteArray) (Value))) => ((entry :: ([] : List ((Prod (ByteArray) (Value))))) :: singles))) (([] : List (List ((Prod (ByteArray) (Value)))))) (entries) : List (List ((Prod (ByteArray) (Value)))))); (match ((LexLeanCollections.iterateUntil ((fun (state : List (List ((Prod (ByteArray) (Value))))) => sortStep (Value) (state))) (((LexLeanRuntime.length (entries) : Nat) + 1)) (runs) : (Prod (List (List ((Prod (ByteArray) (Value))))) (Bool)))).1 with | List.nil => ([] : List ((Prod (ByteArray) (Value)))) | List.cons sorted _ => sorted))

@[expose] public def keysOf (Value : Type) (entries : List ((Prod (ByteArray) (Value)))) : List (ByteArray) := reverseList (ByteArray) ((LexLeanCollections.listFold ((fun (keys : List (ByteArray)) (entry : (Prod (ByteArray) (Value))) => ((entry).1 :: keys))) (([] : List (ByteArray))) (entries) : List (ByteArray)))

@[expose] public def withUnit (items : List (ByteArray)) : List ((Prod (ByteArray) (Unit))) := reverseList ((Prod (ByteArray) (Unit))) ((LexLeanCollections.listFold ((fun (entries : List ((Prod (ByteArray) (Unit)))) (item : ByteArray) => ((item, ()) :: entries))) (([] : List ((Prod (ByteArray) (Unit))))) (items) : List ((Prod (ByteArray) (Unit)))))

@[expose] public def dedupStep (kept : List (ByteArray)) (item : ByteArray) : List (ByteArray) := (match kept with | List.nil => (item :: kept) | List.cons last _ => (if (LexLeanRuntime.equal (last) (item) : Bool) then kept else (item :: kept)))

@[expose] public def sortBytes (items : List (ByteArray)) : List (ByteArray) := keysOf (Unit) (sortByKey (Unit) (withUnit (items)))

@[expose] public def sortEntries (entries : List ((Prod (ByteArray) (ByteArray)))) : List ((Prod (ByteArray) (ByteArray))) := sortByKey (ByteArray) (entries)

@[expose] public def sortedSet (items : List (ByteArray)) : List (ByteArray) := reverseList (ByteArray) ((LexLeanCollections.listFold ((fun (kept : List (ByteArray)) (item : ByteArray) => dedupStep (kept) (item))) (([] : List (ByteArray))) (sortBytes (items)) : List (ByteArray)))

@[expose] public def insertStep (key : ByteArray) (state : (Prod (Bool) (List (ByteArray)))) (item : ByteArray) : (Prod (Bool) (List (ByteArray))) := (match (state).1 with | Bool.true => (true, (item :: (state).2)) | Bool.false => (if (LexLeanRuntime.equal (key) (item) : Bool) then (true, (item :: (state).2)) else (if lessBytes (key) (item) then (true, (item :: (key :: (state).2))) else (false, (item :: (state).2)))))

@[expose] public def insertSorted (items : List (ByteArray)) (key : ByteArray) : List (ByteArray) := (let done : (Prod (Bool) (List (ByteArray))) := (LexLeanCollections.listFold ((fun (state : (Prod (Bool) (List (ByteArray)))) (item : ByteArray) => insertStep (key) (state) (item))) ((false, ([] : List (ByteArray)))) (items) : (Prod (Bool) (List (ByteArray)))); (match (done).1 with | Bool.true => reverseList (ByteArray) ((done).2) | Bool.false => reverseList (ByteArray) ((key :: (done).2))))

@[expose] public def ascendStep (state : (Prod (Bool) (Option (ByteArray)))) (item : ByteArray) : (Prod (Bool) (Option (ByteArray))) := (match (state).2 with | Option.none => ((state).1, Option.some (item)) | Option.some previous => (((state).1 && lessBytes (previous) (item)), Option.some (item)))

@[expose] public def isAscending (items : List (ByteArray)) : Bool := ((LexLeanCollections.listFold ((fun (state : (Prod (Bool) (Option (ByteArray)))) (item : ByteArray) => ascendStep (state) (item))) ((true, Option.none)) (items) : (Prod (Bool) (Option (ByteArray))))).1

end LexLeanStd.Order
