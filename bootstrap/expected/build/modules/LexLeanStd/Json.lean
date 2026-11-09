module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Numeral
public import LexLeanStd.Order
public import LexLeanStd.Text
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanStd.Json

public inductive Json where
  | JBool (_ : Bool)
  | JInt (_ : Bool) (_ : Nat)
  | JStr (_ : ByteArray)
  | JArr (_ : Json)
  | JObj (_ : Json)
  | JNil
  | JItem (_ : Json) (_ : Json)
  | JMember (_ : ByteArray) (_ : Json) (_ : Json)

@[expose] public def escapedPiece (subject : ByteArray) (position : Nat) (byte : UInt8) : ByteArray := (let value : Nat := (let __widen0 : UInt8 := byte; ((((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((128 : UInt8)) : UInt8)) ((128 : UInt8)) : Bool) then 128 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((64 : UInt8)) : UInt8)) ((64 : UInt8)) : Bool) then 64 else 0)) + ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((32 : UInt8)) : UInt8)) ((32 : UInt8)) : Bool) then 32 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((16 : UInt8)) : UInt8)) ((16 : UInt8)) : Bool) then 16 else 0))) + (((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((8 : UInt8)) : UInt8)) ((8 : UInt8)) : Bool) then 8 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((4 : UInt8)) : UInt8)) ((4 : UInt8)) : Bool) then 4 else 0)) + ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((2 : UInt8)) : UInt8)) ((2 : UInt8)) : Bool) then 2 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((1 : UInt8)) : UInt8)) ((1 : UInt8)) : Bool) then 1 else 0))))); (if (Nat.beq (value) (34)) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 92)), (_root_.UInt8.ofNat (nat_lit 34))] else (if (Nat.beq (value) (92)) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 92)), (_root_.UInt8.ofNat (nat_lit 92))] else (if (Nat.beq (value) (8)) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 92)), (_root_.UInt8.ofNat (nat_lit 98))] else (if (Nat.beq (value) (12)) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 92)), (_root_.UInt8.ofNat (nat_lit 102))] else (if (Nat.beq (value) (10)) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 92)), (_root_.UInt8.ofNat (nat_lit 110))] else (if (Nat.beq (value) (13)) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 92)), (_root_.UInt8.ofNat (nat_lit 114))] else (if (Nat.beq (value) (9)) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 92)), (_root_.UInt8.ofNat (nat_lit 116))] else (if (Nat.blt (value) (32)) then (LexLeanRuntime.append (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 92)), (_root_.UInt8.ofNat (nat_lit 117)), (_root_.UInt8.ofNat (nat_lit 48)), (_root_.UInt8.ofNat (nat_lit 48))]) (LexLeanStd.Numeral.hexByte (byte)) : ByteArray) else LexLeanStd.Text.sliceOr (subject) (position) (1))))))))))

@[expose] public def escapeStep (subject : ByteArray) (state : (Prod (Nat) (ByteArray))) : Option ((Prod (Nat) (ByteArray))) := (match (LexLeanRuntime.index (subject) ((state).1) : Option (UInt8)) with | Option.some byte => Option.some ((((state).1 + 1), (LexLeanRuntime.append ((state).2) (escapedPiece (subject) ((state).1) (byte)) : ByteArray))) | Option.none => Option.none)

@[expose] public def quoted (subject : ByteArray) : ByteArray := (LexLeanRuntime.append ((LexLeanRuntime.append (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 34))]) ((((LexLeanCollections.iterateUntil ((fun (state : (Prod (Nat) (ByteArray))) => escapeStep (subject) (state))) ((LexLeanRuntime.length (subject) : Nat)) ((0, _root_.ByteArray.mk #[])) : (Prod ((Prod (Nat) (ByteArray))) (Bool)))).1).2) : ByteArray)) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 34))]) : ByteArray)

@[expose] public def escapeString (text : ByteArray) : ByteArray := quoted (text)

@[expose] public def integerText (negative : Bool) (magnitude : Nat) : ByteArray := (if (negative && (Nat.blt (0) (magnitude))) then (LexLeanRuntime.append (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 45))]) (LexLeanStd.Numeral.decimal (magnitude)) : ByteArray) else LexLeanStd.Numeral.decimal (magnitude))

@[expose] public def arrayText (items : List (ByteArray)) : ByteArray := (LexLeanRuntime.append ((LexLeanRuntime.append (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 91))]) (LexLeanStd.Text.joinWith (items) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 44))])) : ByteArray)) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 93))]) : ByteArray)

@[expose] public def keepLast (kept : List ((Prod (ByteArray) (ByteArray)))) (entry : (Prod (ByteArray) (ByteArray))) : List ((Prod (ByteArray) (ByteArray))) := (match kept with | List.nil => (entry :: kept) | List.cons head rest => (if (LexLeanRuntime.equal ((head).1) ((entry).1) : Bool) then (entry :: rest) else (entry :: kept)))

@[expose] public def memberText (entry : (Prod (ByteArray) (ByteArray))) : ByteArray := (LexLeanRuntime.append ((LexLeanRuntime.append (quoted ((entry).1)) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 58))]) : ByteArray)) ((entry).2) : ByteArray)

@[expose] public def objectText (entries : List ((Prod (ByteArray) (ByteArray)))) : ByteArray := (let unique : List ((Prod (ByteArray) (ByteArray))) := (LexLeanCollections.listFold ((fun (kept : List ((Prod (ByteArray) (ByteArray)))) (entry : (Prod (ByteArray) (ByteArray))) => keepLast (kept) (entry))) (([] : List ((Prod (ByteArray) (ByteArray))))) (LexLeanStd.Order.sortEntries (entries)) : List ((Prod (ByteArray) (ByteArray)))); (let texts : List (ByteArray) := (LexLeanCollections.listFold ((fun (texts : List (ByteArray)) (entry : (Prod (ByteArray) (ByteArray))) => (memberText (entry) :: texts))) (([] : List (ByteArray))) (unique) : List (ByteArray)); (LexLeanRuntime.append ((LexLeanRuntime.append (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 123))]) (LexLeanStd.Text.joinWith (texts) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 44))])) : ByteArray)) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 125))]) : ByteArray)))

mutual
@[expose] public def writeValue : (value : Json) -> ByteArray
  | Json.JBool flag => (if flag then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 116)), (_root_.UInt8.ofNat (nat_lit 114)), (_root_.UInt8.ofNat (nat_lit 117)), (_root_.UInt8.ofNat (nat_lit 101))] else _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 102)), (_root_.UInt8.ofNat (nat_lit 97)), (_root_.UInt8.ofNat (nat_lit 108)), (_root_.UInt8.ofNat (nat_lit 115)), (_root_.UInt8.ofNat (nat_lit 101))])
  | Json.JInt negative magnitude => integerText (negative) (magnitude)
  | Json.JStr text => quoted (text)
  | Json.JArr items => arrayText (renderItems (items))
  | Json.JObj members => objectText (renderMembers (members))
  | Json.JNil => _root_.ByteArray.mk #[]
  | Json.JItem _ _ => _root_.ByteArray.mk #[]
  | Json.JMember _ _ _ => _root_.ByteArray.mk #[]
termination_by structural value => value

@[expose] public def renderItems : (items : Json) -> List (ByteArray)
  | Json.JBool _ => ([] : List (ByteArray))
  | Json.JInt _ _ => ([] : List (ByteArray))
  | Json.JStr _ => ([] : List (ByteArray))
  | Json.JArr _ => ([] : List (ByteArray))
  | Json.JObj _ => ([] : List (ByteArray))
  | Json.JNil => ([] : List (ByteArray))
  | Json.JItem element rest => (writeValue (element) :: renderItems (rest))
  | Json.JMember _ _ _ => ([] : List (ByteArray))
termination_by structural items => items

@[expose] public def renderMembers : (members : Json) -> List ((Prod (ByteArray) (ByteArray)))
  | Json.JBool _ => ([] : List ((Prod (ByteArray) (ByteArray))))
  | Json.JInt _ _ => ([] : List ((Prod (ByteArray) (ByteArray))))
  | Json.JStr _ => ([] : List ((Prod (ByteArray) (ByteArray))))
  | Json.JArr _ => ([] : List ((Prod (ByteArray) (ByteArray))))
  | Json.JObj _ => ([] : List ((Prod (ByteArray) (ByteArray))))
  | Json.JNil => ([] : List ((Prod (ByteArray) (ByteArray))))
  | Json.JItem _ _ => ([] : List ((Prod (ByteArray) (ByteArray))))
  | Json.JMember key inner rest => ((key, writeValue (inner)) :: renderMembers (rest))
termination_by structural members => members
end

@[expose] public def writeJson (value : Json) : ByteArray := writeValue (value)

@[expose] public def writeJsonFile (value : Json) : ByteArray := (LexLeanRuntime.append (writeValue (value)) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 10))]) : ByteArray)

end LexLeanStd.Json
