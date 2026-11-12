module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Json
public import LexLeanStd.Numeral
public import LexLeanStd.Order
public import LexLeanBoot.LexiconLse
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.IrTerm

public inductive CoreRef where
  | CEq
  | CAnd
  | COr
  | CNot
  | CIff
  | CExists
  | CExistsUnique

public inductive GlobalRef where
  | GCore (_ : CoreRef)
  | GExternal (_ : ByteArray) (_ : ByteArray) (_ : ByteArray) (_ : ByteArray) (_ : ByteArray)
  | GDocument (_ : ByteArray) (_ : ByteArray) (_ : ByteArray)
  | GDefined (_ : ByteArray) (_ : ByteArray) (_ : ByteArray)

public inductive Term where
  | TSort (_ : LexLeanBoot.LexiconLse.Universe)
  | TLocal (_ : Nat)
  | TGlobal (_ : GlobalRef) (_ : LexLeanBoot.LexiconLse.Universe)
  | TApp (_ : Term) (_ : Term) (_ : List (Nat))
  | TPi (_ : Term) (_ : Term)
  | TLam (_ : Term) (_ : Term)
  | TLet (_ : Nat) (_ : Nat) (_ : ByteArray) (_ : Term) (_ : Term) (_ : Term)
  | TNat (_ : ByteArray) (_ : Term)
  | TNil
  | TItem (_ : Term) (_ : Term)
  | TBind (_ : Nat) (_ : Nat) (_ : ByteArray) (_ : Term) (_ : Term)

public structure Renumber where
  table : List ((Prod (Nat) (Nat)))
  next : Nat
  keepFree : Bool

public inductive Resolved where
  | RBound (_ : Nat)
  | RFree (_ : Nat)

@[expose] public def lit (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def jtext (literal : String) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JStr (lit (literal))

@[expose] public def member (key : String) (entry : LexLeanStd.Json.Json) (rest : LexLeanStd.Json.Json) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JMember (lit (key)) (entry) (rest)

@[expose] public def capped (number : Nat) : Nat := (if (Nat.ble (number) (9223372036854775807)) then number else 9223372036854775807)

@[expose] public def jnat (number : Nat) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JInt (false) (capped (number))

@[expose] public def coreName (core : CoreRef) : ByteArray := (match core with | CoreRef.CEq => lit ("eq") | CoreRef.CAnd => lit ("and") | CoreRef.COr => lit ("or") | CoreRef.CNot => lit ("not") | CoreRef.CIff => lit ("iff") | CoreRef.CExists => lit ("exists") | CoreRef.CExistsUnique => lit ("exists-unique"))

@[expose] public def universeJson : (level : LexLeanBoot.LexiconLse.Universe) -> LexLeanStd.Json.Json
  | LexLeanBoot.LexiconLse.Universe.UNum number => LexLeanStd.Json.Json.JObj (member ("num") (jnat (number)) (LexLeanStd.Json.Json.JNil))
  | LexLeanBoot.LexiconLse.Universe.UVar named => LexLeanStd.Json.Json.JObj (member ("var") (LexLeanStd.Json.Json.JStr (named)) (LexLeanStd.Json.Json.JNil))
  | LexLeanBoot.LexiconLse.Universe.USucc inner => LexLeanStd.Json.Json.JObj (member ("succ") (universeJson (inner)) (LexLeanStd.Json.Json.JNil))
  | LexLeanBoot.LexiconLse.Universe.UMax levels => LexLeanStd.Json.Json.JObj (member ("max") (LexLeanStd.Json.Json.JArr (universeJson (levels))) (LexLeanStd.Json.Json.JNil))
  | LexLeanBoot.LexiconLse.Universe.UIMax left right => LexLeanStd.Json.Json.JObj (member ("imax") (LexLeanStd.Json.Json.JArr (LexLeanStd.Json.Json.JItem (universeJson (left)) (LexLeanStd.Json.Json.JItem (universeJson (right)) (LexLeanStd.Json.Json.JNil)))) (LexLeanStd.Json.Json.JNil))
  | LexLeanBoot.LexiconLse.Universe.UNil => LexLeanStd.Json.Json.JNil
  | LexLeanBoot.LexiconLse.Universe.UItem head tail => LexLeanStd.Json.Json.JItem (universeJson (head)) (universeJson (tail))

@[expose] public def noLevels (levels : LexLeanBoot.LexiconLse.Universe) : Bool := (match levels with | LexLeanBoot.LexiconLse.Universe.UNil => true | LexLeanBoot.LexiconLse.Universe.UNum _ => false | LexLeanBoot.LexiconLse.Universe.UVar _ => false | LexLeanBoot.LexiconLse.Universe.USucc _ => false | LexLeanBoot.LexiconLse.Universe.UMax _ => false | LexLeanBoot.LexiconLse.Universe.UIMax _ _ => false | LexLeanBoot.LexiconLse.Universe.UItem _ _ => false)

@[expose] public def globalJson (global : GlobalRef) : LexLeanStd.Json.Json := (match global with | GlobalRef.GCore core => LexLeanStd.Json.Json.JObj (member ("kind") (jtext ("core")) (member ("name") (LexLeanStd.Json.Json.JStr (coreName (core))) (LexLeanStd.Json.Json.JNil))) | GlobalRef.GExternal _ entry leanModule leanName signature => LexLeanStd.Json.Json.JObj (member ("kind") (jtext ("external")) (member ("entry") (LexLeanStd.Json.Json.JStr (entry)) (member ("lean_module") (LexLeanStd.Json.Json.JStr (leanModule)) (member ("lean_name") (LexLeanStd.Json.Json.JStr (leanName)) (member ("signature_sha256") (LexLeanStd.Json.Json.JStr (LexLeanStd.Numeral.hex (signature))) (LexLeanStd.Json.Json.JNil)))))) | GlobalRef.GDocument sourceModule component leanName => LexLeanStd.Json.Json.JObj (member ("kind") (jtext ("document")) (member ("module") (LexLeanStd.Json.Json.JStr (sourceModule)) (member ("component") (LexLeanStd.Json.Json.JStr (component)) (member ("lean_name") (LexLeanStd.Json.Json.JStr (leanName)) (LexLeanStd.Json.Json.JNil))))) | GlobalRef.GDefined _ entry signature => LexLeanStd.Json.Json.JObj (member ("kind") (jtext ("defined")) (member ("entry") (LexLeanStd.Json.Json.JStr (entry)) (member ("signature_sha256") (LexLeanStd.Json.Json.JStr (LexLeanStd.Numeral.hex (signature))) (LexLeanStd.Json.Json.JNil)))))

@[expose] public def lookupLocal (table : List ((Prod (Nat) (Nat)))) (localId : Nat) : Option (Nat) := (LexLeanCollections.listFold ((fun (found : Option (Nat)) (entry : (Prod (Nat) (Nat))) => (if (Nat.beq ((entry).1) (localId)) then Option.some ((entry).2) else found))) (Option.none) (table) : Option (Nat))

@[expose] public def fresh (state : Renumber) (localId : Nat) : (Prod (Nat) (Renumber)) := ((state).next, ({ table := ((localId, (state).next) :: (state).table), next := ((state).next + 1), keepFree := (state).keepFree } : Renumber))

@[expose] public def bind (state : Renumber) (localId : Nat) : (Prod (Nat) (Renumber)) := (match lookupLocal ((state).table) (localId) with | Option.some index => (index, state) | Option.none => fresh (state) (localId))

@[expose] public def resolve (state : Renumber) (localId : Nat) : (Prod (Resolved) (Renumber)) := (match lookupLocal ((state).table) (localId) with | Option.some index => (Resolved.RBound (index), state) | Option.none => (if (state).keepFree then (Resolved.RFree (localId), state) else (let bound : (Prod (Nat) (Renumber)) := fresh (state) (localId); (Resolved.RBound ((bound).1), (bound).2))))

@[expose] public def hashing : Renumber := ({ table := ([] : List ((Prod (Nat) (Nat)))), next := 0, keepFree := false } : Renumber)

@[expose] public def comparing : Renumber := ({ table := ([] : List ((Prod (Nat) (Nat)))), next := 0, keepFree := true } : Renumber)

@[expose] public def localJson (resolved : Resolved) : LexLeanStd.Json.Json := (match resolved with | Resolved.RBound index => LexLeanStd.Json.Json.JObj (member ("k") (jtext ("local")) (member ("id") (jnat (index)) (LexLeanStd.Json.Json.JNil))) | Resolved.RFree localId => LexLeanStd.Json.Json.JObj (member ("k") (jtext ("local")) (member ("free") (jnat (localId)) (LexLeanStd.Json.Json.JNil))))

@[expose] public def modeName (mode : Nat) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JStr (LexLeanBoot.LexiconLse.modeText (mode))

@[expose] public def binderJson (index : Nat) (mode : Nat) (spelling : ByteArray) (typed : LexLeanStd.Json.Json) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JObj (member ("id") (jnat (index)) (member ("m") (modeName (mode)) (member ("s") (LexLeanStd.Json.Json.JStr (spelling)) (member ("t") (typed) (LexLeanStd.Json.Json.JNil)))))

@[expose] public def implicitJson (implicits : List (Nat)) : LexLeanStd.Json.Json := (LexLeanCollections.listFold ((fun (acc : LexLeanStd.Json.Json) (index : Nat) => LexLeanStd.Json.Json.JItem (jnat (index)) (acc))) (LexLeanStd.Json.Json.JNil) (LexLeanStd.Order.reverseList (Nat) (implicits)) : LexLeanStd.Json.Json)

@[expose] public def appJson (function : LexLeanStd.Json.Json) (arguments : LexLeanStd.Json.Json) (implicits : List (Nat)) : LexLeanStd.Json.Json := (let tail : LexLeanStd.Json.Json := (match implicits with | List.nil => LexLeanStd.Json.Json.JNil | List.cons _ _ => member ("i") (LexLeanStd.Json.Json.JArr (implicitJson (implicits))) (LexLeanStd.Json.Json.JNil)); LexLeanStd.Json.Json.JObj (member ("k") (jtext ("app")) (member ("f") (function) (member ("a") (LexLeanStd.Json.Json.JArr (arguments)) (tail)))))

@[expose] public def globalTermJson (global : GlobalRef) (levels : LexLeanBoot.LexiconLse.Universe) : LexLeanStd.Json.Json := (let tail : LexLeanStd.Json.Json := (if noLevels (levels) then LexLeanStd.Json.Json.JNil else member ("u") (LexLeanStd.Json.Json.JArr (universeJson (levels))) (LexLeanStd.Json.Json.JNil)); LexLeanStd.Json.Json.JObj (member ("k") (jtext ("global")) (member ("g") (globalJson (global)) (tail))))

@[expose] public def termJson : (term : Term) -> (state : Renumber) -> (Prod (LexLeanStd.Json.Json) (Renumber))
  | Term.TSort level, state => (LexLeanStd.Json.Json.JObj (member ("k") (jtext ("sort")) (member ("u") (universeJson (level)) (LexLeanStd.Json.Json.JNil))), state)
  | Term.TLocal localId, state => (let resolved : (Prod (Resolved) (Renumber)) := resolve (state) (localId); (localJson ((resolved).1), (resolved).2))
  | Term.TGlobal global levels, state => (globalTermJson (global) (levels), state)
  | Term.TApp function arguments implicits, state => (let head : (Prod (LexLeanStd.Json.Json) (Renumber)) := termJson (function) (state); (let args : (Prod (LexLeanStd.Json.Json) (Renumber)) := termJson (arguments) ((head).2); (appJson ((head).1) ((args).1) (implicits), (args).2)))
  | Term.TPi binders body, state => (let bound : (Prod (LexLeanStd.Json.Json) (Renumber)) := termJson (binders) (state); (let inner : (Prod (LexLeanStd.Json.Json) (Renumber)) := termJson (body) ((bound).2); (LexLeanStd.Json.Json.JObj (member ("k") (jtext ("pi")) (member ("b") (LexLeanStd.Json.Json.JArr ((bound).1)) (member ("v") ((inner).1) (LexLeanStd.Json.Json.JNil)))), (inner).2)))
  | Term.TLam binders body, state => (let bound : (Prod (LexLeanStd.Json.Json) (Renumber)) := termJson (binders) (state); (let inner : (Prod (LexLeanStd.Json.Json) (Renumber)) := termJson (body) ((bound).2); (LexLeanStd.Json.Json.JObj (member ("k") (jtext ("lam")) (member ("b") (LexLeanStd.Json.Json.JArr ((bound).1)) (member ("v") ((inner).1) (LexLeanStd.Json.Json.JNil)))), (inner).2)))
  | Term.TLet localId mode spelling typed bound body, state => (let typeJson : (Prod (LexLeanStd.Json.Json) (Renumber)) := termJson (typed) (state); (let valueJson : (Prod (LexLeanStd.Json.Json) (Renumber)) := termJson (bound) ((typeJson).2); (let index : (Prod (Nat) (Renumber)) := bind ((valueJson).2) (localId); (let inner : (Prod (LexLeanStd.Json.Json) (Renumber)) := termJson (body) ((index).2); (LexLeanStd.Json.Json.JObj (member ("k") (jtext ("let")) (member ("b") (binderJson ((index).1) (mode) (spelling) ((typeJson).1)) (member ("x") ((valueJson).1) (member ("v") ((inner).1) (LexLeanStd.Json.Json.JNil))))), (inner).2)))))
  | Term.TNat decimal expected, state => (let inner : (Prod (LexLeanStd.Json.Json) (Renumber)) := termJson (expected) (state); (LexLeanStd.Json.Json.JObj (member ("k") (jtext ("nat")) (member ("d") (LexLeanStd.Json.Json.JStr (decimal)) (member ("t") ((inner).1) (LexLeanStd.Json.Json.JNil)))), (inner).2))
  | Term.TNil, state => (LexLeanStd.Json.Json.JNil, state)
  | Term.TItem head tail, state => (let first_ : (Prod (LexLeanStd.Json.Json) (Renumber)) := termJson (head) (state); (let rest : (Prod (LexLeanStd.Json.Json) (Renumber)) := termJson (tail) ((first_).2); (LexLeanStd.Json.Json.JItem ((first_).1) ((rest).1), (rest).2)))
  | Term.TBind localId mode spelling typed tail, state => (let typeJson : (Prod (LexLeanStd.Json.Json) (Renumber)) := termJson (typed) (state); (let index : (Prod (Nat) (Renumber)) := bind ((typeJson).2) (localId); (let rest : (Prod (LexLeanStd.Json.Json) (Renumber)) := termJson (tail) ((index).2); (LexLeanStd.Json.Json.JItem (binderJson ((index).1) (mode) (spelling) ((typeJson).1)) ((rest).1), (rest).2))))

@[expose] public def withoutSpellings : (value : LexLeanStd.Json.Json) -> LexLeanStd.Json.Json
  | LexLeanStd.Json.Json.JBool flag => LexLeanStd.Json.Json.JBool (flag)
  | LexLeanStd.Json.Json.JInt negative magnitude => LexLeanStd.Json.Json.JInt (negative) (magnitude)
  | LexLeanStd.Json.Json.JStr text => LexLeanStd.Json.Json.JStr (text)
  | LexLeanStd.Json.Json.JArr items => LexLeanStd.Json.Json.JArr (withoutSpellings (items))
  | LexLeanStd.Json.Json.JObj members => LexLeanStd.Json.Json.JObj (withoutSpellings (members))
  | LexLeanStd.Json.Json.JNil => LexLeanStd.Json.Json.JNil
  | LexLeanStd.Json.Json.JItem element rest => LexLeanStd.Json.Json.JItem (withoutSpellings (element)) (withoutSpellings (rest))
  | LexLeanStd.Json.Json.JMember key inner rest => (if (LexLeanRuntime.equal (key) (lit ("s")) : Bool) then withoutSpellings (rest) else LexLeanStd.Json.Json.JMember (key) (withoutSpellings (inner)) (withoutSpellings (rest)))

@[expose] public def termText (term : Term) : ByteArray := LexLeanStd.Json.writeJson ((termJson (term) (hashing)).1)

@[expose] public def canonicalKey (term : Term) : ByteArray := LexLeanStd.Json.writeJson (withoutSpellings ((termJson (term) (hashing)).1))

@[expose] public def eqKey (term : Term) : ByteArray := LexLeanStd.Json.writeJson (withoutSpellings ((termJson (term) (comparing)).1))

@[expose] public def nodeCount : (term : Term) -> Nat
  | Term.TSort _ => 1
  | Term.TLocal _ => 1
  | Term.TGlobal _ _ => 1
  | Term.TApp function arguments _ => ((1 + nodeCount (function)) + nodeCount (arguments))
  | Term.TPi binders body => ((1 + nodeCount (binders)) + nodeCount (body))
  | Term.TLam binders body => ((1 + nodeCount (binders)) + nodeCount (body))
  | Term.TLet _ _ _ typed bound body => (((1 + nodeCount (typed)) + nodeCount (bound)) + nodeCount (body))
  | Term.TNat _ expected => (1 + nodeCount (expected))
  | Term.TNil => 0
  | Term.TItem head tail => (nodeCount (head) + nodeCount (tail))
  | Term.TBind _ _ _ typed tail => (nodeCount (typed) + nodeCount (tail))

@[expose] public def larger (left : Nat) (right : Nat) : Nat := (if (Nat.blt (left) (right)) then right else left)

@[expose] public def termDepth : (term : Term) -> Nat
  | Term.TSort _ => 1
  | Term.TLocal _ => 1
  | Term.TGlobal _ _ => 1
  | Term.TApp function arguments _ => (1 + larger (termDepth (function)) (termDepth (arguments)))
  | Term.TPi binders body => (1 + larger (termDepth (binders)) (termDepth (body)))
  | Term.TLam binders body => (1 + larger (termDepth (binders)) (termDepth (body)))
  | Term.TLet _ _ _ typed bound body => (1 + larger (larger (termDepth (typed)) (termDepth (bound))) (termDepth (body)))
  | Term.TNat _ expected => (1 + termDepth (expected))
  | Term.TNil => 0
  | Term.TItem head tail => larger (termDepth (head)) (termDepth (tail))
  | Term.TBind _ _ _ typed tail => larger (termDepth (typed)) (termDepth (tail))

end LexLeanBoot.IrTerm
