module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Json
public import LexLeanStd.Numeral
public import LexLeanStd.Order
public import LexLeanStd.Text
public import LexLeanBoot.SurfaceLex
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.CoreJson

public inductive Level where
  | LZero
  | LSucc (_ : Level)
  | LMax (_ : Level) (_ : Level)
  | LIMax (_ : Level) (_ : Level)
  | LParam (_ : ByteArray)

public inductive Binder where
  | BExplicit
  | BImplicit
  | BStrict
  | BInstance

public inductive Meta where
  | MBool (_ : Bool)
  | MNat (_ : ByteArray)
  | MInt (_ : ByteArray)
  | MStr (_ : ByteArray)
  | MName (_ : ByteArray)

public structure Entry where
  key : ByteArray
  value : Meta

public inductive Node where
  | NBVar (_ : Nat)
  | NSort (_ : Level)
  | NConst (_ : ByteArray) (_ : List (Level))
  | NApp (_ : Nat) (_ : Nat)
  | NLambda (_ : ByteArray) (_ : Binder) (_ : Nat) (_ : Nat)
  | NForall (_ : ByteArray) (_ : Binder) (_ : Nat) (_ : Nat)
  | NLet (_ : ByteArray) (_ : Nat) (_ : Nat) (_ : Nat) (_ : Bool)
  | NNat (_ : ByteArray)
  | NString (_ : List (Nat))
  | NMdata (_ : List (Entry)) (_ : Nat)
  | NProj (_ : ByteArray) (_ : ByteArray) (_ : Nat) (_ : Nat)

public structure Module where
  spec : ByteArray
  imports : List (ByteArray)
  nodes : List (Node)
  proofNodes : List (Nat)
  declarations : List (LexLeanStd.Json.Json)

@[expose] public def lit (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def cat (left : ByteArray) (right : ByteArray) : ByteArray := (LexLeanRuntime.append (left) (right) : ByteArray)

@[expose] public def jStep (text : ByteArray) (state : (Prod (Nat) (ByteArray))) : Option ((Prod (Nat) (ByteArray))) := (match LexLeanStd.Text.byteAt (text) ((state).1) with | Option.some byte => Option.some ((((state).1 + 1), cat ((state).2) ((if (Nat.beq ((let __widen0 : UInt8 := byte; ((((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((128 : UInt8)) : UInt8)) ((128 : UInt8)) : Bool) then 128 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((64 : UInt8)) : UInt8)) ((64 : UInt8)) : Bool) then 64 else 0)) + ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((32 : UInt8)) : UInt8)) ((32 : UInt8)) : Bool) then 32 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((16 : UInt8)) : UInt8)) ((16 : UInt8)) : Bool) then 16 else 0))) + (((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((8 : UInt8)) : UInt8)) ((8 : UInt8)) : Bool) then 8 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((4 : UInt8)) : UInt8)) ((4 : UInt8)) : Bool) then 4 else 0)) + ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((2 : UInt8)) : UInt8)) ((2 : UInt8)) : Bool) then 2 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((1 : UInt8)) : UInt8)) ((1 : UInt8)) : Bool) then 1 else 0)))))) (39)) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 34))] else LexLeanStd.Text.sliceOr (text) ((state).1) (1))))) | Option.none => Option.none)

@[expose] public def j (literal : String) : ByteArray := (let text : ByteArray := lit (literal); (((LexLeanCollections.iterateUntil ((fun (state : (Prod (Nat) (ByteArray))) => jStep (text) (state))) (((LexLeanRuntime.length (text) : Nat) + 1)) ((0, _root_.ByteArray.mk #[])) : (Prod ((Prod (Nat) (ByteArray))) (Bool)))).1).2)

@[expose] public def num (number : Nat) : ByteArray := LexLeanStd.Numeral.decimal (number)

@[expose] public def quoted (text : ByteArray) : ByteArray := LexLeanBoot.SurfaceLex.quoteText (text)

@[expose] public def joined (pieces : List (ByteArray)) (separator : ByteArray) : ByteArray := ((LexLeanCollections.listFold ((fun (state : (Prod (Bool) (ByteArray))) (piece : ByteArray) => (false, (if (state).1 then piece else cat (cat ((state).2) (separator)) (piece))))) ((true, lit (""))) (pieces) : (Prod (Bool) (ByteArray)))).2

@[expose] public def binderLetter (info : Binder) : ByteArray := (match info with | Binder.BExplicit => lit ("e") | Binder.BImplicit => lit ("i") | Binder.BStrict => lit ("s") | Binder.BInstance => lit ("c"))

@[expose] public def levelJson : (level : Level) -> ByteArray
  | Level.LZero => j ("{'k':'z'}")
  | Level.LSucc inner => cat (cat (j ("{'a':")) (levelJson (inner))) (j (",'k':'s'}"))
  | Level.LMax low high => cat (cat (cat (cat (j ("{'a':[")) (levelJson (low))) (lit (","))) (levelJson (high))) (j ("],'k':'m'}"))
  | Level.LIMax low high => cat (cat (cat (cat (j ("{'a':[")) (levelJson (low))) (lit (","))) (levelJson (high))) (j ("],'k':'i'}"))
  | Level.LParam label => cat (cat (j ("{'a':")) (quoted (label))) (j (",'k':'p'}"))

@[expose] public def metaJson (entry : Entry) : ByteArray := cat (cat (cat (cat (j ("{'n':")) (quoted ((entry).key))) (j (",'v':"))) ((match (entry).value with | Meta.MBool flag => cat (cat (j ("{'t':'b','v':")) ((if flag then lit ("true") else lit ("false")))) (lit ("}")) | Meta.MNat text => cat (cat (j ("{'t':'n','v':")) (quoted (text))) (lit ("}")) | Meta.MInt text => cat (cat (j ("{'t':'i','v':")) (quoted (text))) (lit ("}")) | Meta.MStr text => cat (cat (j ("{'t':'s','v':")) (quoted (text))) (lit ("}")) | Meta.MName text => cat (cat (j ("{'t':'m','v':")) (quoted (text))) (lit ("}"))))) (lit ("}"))

@[expose] public def levelsJson (levels : List (Level)) : ByteArray := joined (LexLeanStd.Order.reverseList (ByteArray) ((LexLeanCollections.listFold ((fun (out : List (ByteArray)) (level : Level) => (levelJson (level) :: out))) (([] : List (ByteArray))) (levels) : List (ByteArray)))) (lit (","))

@[expose] public def binderJson (letter : ByteArray) (label : ByteArray) (info : Binder) (domain : Nat) (body : Nat) : ByteArray := cat (cat (cat (cat (cat (cat (cat (cat (j ("{'b':'")) (binderLetter (info))) (j ("','k':'"))) (letter)) (j ("','n':"))) (quoted (label))) (cat (j (",'t':")) (num (domain)))) (cat (j (",'v':")) (num (body)))) (lit ("}"))

@[expose] public def nodeJson (node : Node) : ByteArray := (match node with | Node.NBVar position => cat (cat (j ("{'i':")) (num (position))) (j (",'k':'b'}")) | Node.NSort level => cat (cat (j ("{'k':'s','l':")) (levelJson (level))) (lit ("}")) | Node.NConst label levels => cat (cat (cat (cat (j ("{'k':'c','n':")) (quoted (label))) (j (",'u':["))) (levelsJson (levels))) (lit ("]}")) | Node.NApp function argument => cat (cat (cat (cat (j ("{'f':")) (num (function))) (j (",'k':'a','x':"))) (num (argument))) (lit ("}")) | Node.NLambda label info domain body => binderJson (lit ("l")) (label) (info) (domain) (body) | Node.NForall label info domain body => binderJson (lit ("p")) (label) (info) (domain) (body) | Node.NLet label domain bound rest dependent => cat (cat (cat (cat (cat (cat (cat (cat (j ("{'body':")) (num (rest))) (j (",'d':"))) ((if dependent then lit ("true") else lit ("false")))) (j (",'k':'e','n':"))) (quoted (label))) (cat (j (",'t':")) (num (domain)))) (cat (j (",'v':")) (num (bound)))) (lit ("}")) | Node.NNat text => cat (cat (j ("{'k':'n','v':")) (quoted (text))) (lit ("}")) | Node.NString scalars => cat (cat (j ("{'k':'t','v':[")) (joined (LexLeanStd.Order.reverseList (ByteArray) ((LexLeanCollections.listFold ((fun (out : List (ByteArray)) (scalar : Nat) => (num (scalar) :: out))) (([] : List (ByteArray))) (scalars) : List (ByteArray)))) (lit (",")))) (lit ("]}")) | Node.NMdata entries inner => cat (cat (cat (cat (j ("{'k':'d','m':[")) (joined (LexLeanStd.Order.reverseList (ByteArray) ((LexLeanCollections.listFold ((fun (out : List (ByteArray)) (entry : Entry) => (metaJson (entry) :: out))) (([] : List (ByteArray))) (entries) : List (ByteArray)))) (lit (",")))) (j ("],'x':"))) (num (inner))) (lit ("}")) | Node.NProj label «structure» position inner => cat (cat (cat (cat (cat (cat (cat (cat (j ("{'i':")) (num (position))) (j (",'k':'j','n':"))) (quoted (label))) (j (",'s':"))) (quoted («structure»))) (j (",'x':"))) (num (inner))) (lit ("}")))

@[expose] public def mapped (Item : Type) (items : List (Item)) (render : ((Item) -> (ByteArray))) : List (ByteArray) := LexLeanStd.Order.reverseList (ByteArray) ((LexLeanCollections.listFold ((fun (out : List (ByteArray)) (item : Item) => ((render (item)) :: out))) (([] : List (ByteArray))) (items) : List (ByteArray)))

@[expose] public def coreJson (module : Module) : ByteArray := joined ((j ("{'declarations':[") :: (joined (mapped (LexLeanStd.Json.Json) ((module).declarations) ((fun (declaration : LexLeanStd.Json.Json) => LexLeanStd.Json.writeJson (declaration)))) (lit (",")) :: (j ("],'imports':[") :: (joined (mapped (ByteArray) ((module).imports) ((fun (name : ByteArray) => quoted (name)))) (lit (",")) :: (j ("],'nodes':[") :: (joined (mapped (Node) ((module).nodes) ((fun (node : Node) => nodeJson (node)))) (lit (",")) :: (j ("],'proof_nodes':[") :: (joined (mapped (Nat) ((module).proofNodes) ((fun (index : Nat) => num (index)))) (lit (",")) :: (j ("],'spec':") :: (quoted ((module).spec) :: (lit ("}") :: ([] : List (ByteArray)))))))))))))) (lit (""))

end LexLeanBoot.CoreJson
