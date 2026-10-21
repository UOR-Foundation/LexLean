module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanStd.Text

@[expose] public def isSpace (byte : UInt8) : Bool := ((((LexLeanRuntime.equal (byte) ((32 : UInt8)) : Bool) || (LexLeanRuntime.equal (byte) ((9 : UInt8)) : Bool)) || (LexLeanRuntime.equal (byte) ((10 : UInt8)) : Bool)) || ((LexLeanRuntime.equal (byte) ((12 : UInt8)) : Bool) || (LexLeanRuntime.equal (byte) ((13 : UInt8)) : Bool)))

@[expose] public def byteAt (subject : ByteArray) (position : Nat) : Option (UInt8) := (LexLeanRuntime.index (subject) (position) : Option (UInt8))

@[expose] public def spaceAt (subject : ByteArray) (position : Nat) : Bool := (match byteAt (subject) (position) with | Option.some byte => isSpace (byte) | Option.none => false)

@[expose] public def sliceOr (subject : ByteArray) (start : Nat) (count : Nat) : ByteArray := (match (LexLeanRuntime.slice (subject) (start) (count) : Option (ByteArray)) with | Option.some piece => piece | Option.none => _root_.ByteArray.mk #[])

@[expose] public def startsWith (subject : ByteArray) («prefix» : ByteArray) : Bool := (match (LexLeanRuntime.slice (subject) (0) ((LexLeanRuntime.length («prefix») : Nat)) : Option (ByteArray)) with | Option.some head => (LexLeanRuntime.equal (head) («prefix») : Bool) | Option.none => false)

@[expose] public def endsWith (subject : ByteArray) (suffix : ByteArray) : Bool := (let total : Nat := (LexLeanRuntime.length (subject) : Nat); (let size : Nat := (LexLeanRuntime.length (suffix) : Nat); (if (Nat.ble (size) (total)) then (match (LexLeanRuntime.slice (subject) ((LexLeanRuntime.subtract (total) (size) : Nat)) (size) : Option (ByteArray)) with | Option.some tail => (LexLeanRuntime.equal (tail) (suffix) : Bool) | Option.none => false) else false)))

@[expose] public def trimEnd (subject : ByteArray) : Nat := ((LexLeanCollections.iterateUntil ((fun («end» : Nat) => (if (Nat.blt (0) («end»)) then (if spaceAt (subject) ((LexLeanRuntime.subtract («end») (1) : Nat)) then Option.some ((LexLeanRuntime.subtract («end») (1) : Nat)) else Option.none) else Option.none))) ((LexLeanRuntime.length (subject) : Nat)) ((LexLeanRuntime.length (subject) : Nat)) : (Prod (Nat) (Bool)))).1

@[expose] public def trimStart (subject : ByteArray) («end» : Nat) : Nat := ((LexLeanCollections.iterateUntil ((fun (start : Nat) => (if (Nat.blt (start) («end»)) then (if spaceAt (subject) (start) then Option.some ((start + 1)) else Option.none) else Option.none))) («end») (0) : (Prod (Nat) (Bool)))).1

@[expose] public def trim (subject : ByteArray) : ByteArray := (let «end» : Nat := trimEnd (subject); (let start : Nat := trimStart (subject) («end»); sliceOr (subject) (start) ((LexLeanRuntime.subtract («end») (start) : Nat))))

@[expose] public def reverse (pieces : List (ByteArray)) : List (ByteArray) := (LexLeanCollections.listFold ((fun (reversed : List (ByteArray)) (piece : ByteArray) => (piece :: reversed))) (([] : List (ByteArray))) (pieces) : List (ByteArray))

@[expose] public def splitStep (subject : ByteArray) (delimiter : UInt8) (state : (Prod (Nat) ((Prod (Nat) (List (ByteArray)))))) : Option ((Prod (Nat) ((Prod (Nat) (List (ByteArray)))))) := (let position : Nat := (state).1; (let start : Nat := ((state).2).1; (let pieces : List (ByteArray) := ((state).2).2; (match byteAt (subject) (position) with | Option.some byte => (if (LexLeanRuntime.equal (byte) (delimiter) : Bool) then Option.some (((position + 1), ((position + 1), (sliceOr (subject) (start) ((LexLeanRuntime.subtract (position) (start) : Nat)) :: pieces)))) else Option.some (((position + 1), (start, pieces)))) | Option.none => Option.none))))

@[expose] public def splitOn (subject : ByteArray) (delimiter : UInt8) : List (ByteArray) := (let total : Nat := (LexLeanRuntime.length (subject) : Nat); (let final : (Prod (Nat) ((Prod (Nat) (List (ByteArray))))) := ((LexLeanCollections.iterateUntil ((fun (state : (Prod (Nat) ((Prod (Nat) (List (ByteArray)))))) => splitStep (subject) (delimiter) (state))) (total) ((0, (0, ([] : List (ByteArray))))) : (Prod ((Prod (Nat) ((Prod (Nat) (List (ByteArray)))))) (Bool)))).1; (let start : Nat := ((final).2).1; reverse ((sliceOr (subject) (start) ((LexLeanRuntime.subtract (total) (start) : Nat)) :: ((final).2).2)))))

@[expose] public def chomp (line : ByteArray) : ByteArray := (if endsWith (line) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 13))]) then sliceOr (line) (0) ((LexLeanRuntime.subtract ((LexLeanRuntime.length (line) : Nat)) (1) : Nat)) else line)

@[expose] public def lineStep (subject : ByteArray) (state : (Prod (Nat) ((Prod (Nat) (List (ByteArray)))))) : Option ((Prod (Nat) ((Prod (Nat) (List (ByteArray)))))) := (let position : Nat := (state).1; (let start : Nat := ((state).2).1; (let pieces : List (ByteArray) := ((state).2).2; (match byteAt (subject) (position) with | Option.some byte => (if (LexLeanRuntime.equal (byte) ((10 : UInt8)) : Bool) then Option.some (((position + 1), ((position + 1), (chomp (sliceOr (subject) (start) ((LexLeanRuntime.subtract (position) (start) : Nat))) :: pieces)))) else Option.some (((position + 1), (start, pieces)))) | Option.none => Option.none))))

@[expose] public def lines (subject : ByteArray) : List (ByteArray) := (let total : Nat := (LexLeanRuntime.length (subject) : Nat); (let final : (Prod (Nat) ((Prod (Nat) (List (ByteArray))))) := ((LexLeanCollections.iterateUntil ((fun (state : (Prod (Nat) ((Prod (Nat) (List (ByteArray)))))) => lineStep (subject) (state))) (total) ((0, (0, ([] : List (ByteArray))))) : (Prod ((Prod (Nat) ((Prod (Nat) (List (ByteArray)))))) (Bool)))).1; (let start : Nat := ((final).2).1; (let pieces : List (ByteArray) := ((final).2).2; (if (Nat.blt (start) (total)) then reverse ((sliceOr (subject) (start) ((LexLeanRuntime.subtract (total) (start) : Nat)) :: pieces)) else reverse (pieces))))))

@[expose] public def joinWith (pieces : List (ByteArray)) (separator : ByteArray) : ByteArray := (match (LexLeanCollections.listFold ((fun (joined : Option (ByteArray)) (piece : ByteArray) => (match joined with | Option.none => Option.some (piece) | Option.some sofar => Option.some ((LexLeanRuntime.append ((LexLeanRuntime.append (sofar) (separator) : ByteArray)) (piece) : ByteArray))))) (Option.none) (pieces) : Option (ByteArray)) with | Option.some joined => joined | Option.none => _root_.ByteArray.mk #[])

@[expose] public def matchAt (subject : ByteArray) (needle : ByteArray) (position : Nat) : Bool := (match (LexLeanRuntime.slice (subject) (position) ((LexLeanRuntime.length (needle) : Nat)) : Option (ByteArray)) with | Option.some window => (LexLeanRuntime.equal (window) (needle) : Bool) | Option.none => false)

@[expose] public def find (subject : ByteArray) (needle : ByteArray) : Option (Nat) := (let total : Nat := (LexLeanRuntime.length (subject) : Nat); (let size : Nat := (LexLeanRuntime.length (needle) : Nat); (let position : Nat := ((LexLeanCollections.iterateUntil ((fun (position : Nat) => (if matchAt (subject) (needle) (position) then Option.none else (if (Nat.blt ((position + size)) (total)) then Option.some ((position + 1)) else Option.none)))) ((total + 1)) (0) : (Prod (Nat) (Bool)))).1; (if matchAt (subject) (needle) (position) then Option.some (position) else Option.none))))

end LexLeanStd.Text
