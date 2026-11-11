module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanBoot.AxiomOutput
public import LexLeanStd.Json
public import LexLeanStd.Order
public import LexLeanStd.Text
public import LexLeanBoot.SurfaceGeneric
public import LexLeanBoot.SurfaceLex
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.SurfaceTooling

public structure Sp where
  pos : Nat
  phase : Nat
  spans : List ((Prod (Nat) (Nat)))
  error : Option (LexLeanBoot.SurfaceLex.Failure)

@[expose] public def lit (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def failure (offset : Nat) (message : ByteArray) : LexLeanBoot.SurfaceLex.Failure := ({ offset := offset, message := message } : LexLeanBoot.SurfaceLex.Failure)

@[expose] public def hasSpec : (members : LexLeanStd.Json.Json) -> Bool
  | LexLeanStd.Json.Json.JMember key inner rest => (if (LexLeanRuntime.equal (key) (lit ("spec")) : Bool) then (match inner with | LexLeanStd.Json.Json.JStr _ => true | LexLeanStd.Json.Json.JBool _ => false | LexLeanStd.Json.Json.JInt _ _ => false | LexLeanStd.Json.Json.JArr _ => false | LexLeanStd.Json.Json.JObj _ => false | LexLeanStd.Json.Json.JNil => false | LexLeanStd.Json.Json.JItem _ _ => false | LexLeanStd.Json.Json.JMember _ _ _ => false) else hasSpec (rest))
  | LexLeanStd.Json.Json.JBool _ => false
  | LexLeanStd.Json.Json.JInt _ _ => false
  | LexLeanStd.Json.Json.JStr _ => false
  | LexLeanStd.Json.Json.JArr _ => false
  | LexLeanStd.Json.Json.JObj _ => false
  | LexLeanStd.Json.Json.JNil => false
  | LexLeanStd.Json.Json.JItem _ _ => false

@[expose] public def withoutSpec : (members : LexLeanStd.Json.Json) -> LexLeanStd.Json.Json
  | LexLeanStd.Json.Json.JMember key inner rest => (if (LexLeanRuntime.equal (key) (lit ("spec")) : Bool) then withoutSpec (rest) else LexLeanStd.Json.Json.JMember (key) (inner) (withoutSpec (rest)))
  | LexLeanStd.Json.Json.JBool flag => LexLeanStd.Json.Json.JBool (flag)
  | LexLeanStd.Json.Json.JInt negative magnitude => LexLeanStd.Json.Json.JInt (negative) (magnitude)
  | LexLeanStd.Json.Json.JStr text => LexLeanStd.Json.Json.JStr (text)
  | LexLeanStd.Json.Json.JArr items => LexLeanStd.Json.Json.JArr (items)
  | LexLeanStd.Json.Json.JObj fields => LexLeanStd.Json.Json.JObj (fields)
  | LexLeanStd.Json.Json.JNil => LexLeanStd.Json.Json.JNil
  | LexLeanStd.Json.Json.JItem element rest => LexLeanStd.Json.Json.JItem (element) (rest)

@[expose] public def printToolingModule (given : LexLeanStd.Json.Json) : Except (LexLeanBoot.SurfaceLex.Failure) (ByteArray) := (match given with | LexLeanStd.Json.Json.JObj members => (if hasSpec (members) then Except.ok ((LexLeanRuntime.append (LexLeanBoot.SurfaceGeneric.printGeneric (LexLeanStd.Json.Json.JObj (withoutSpec (members)))) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 10))]) : ByteArray)) else Except.error (failure (0) (lit ("a module has a `spec`")))) | LexLeanStd.Json.Json.JBool _ => Except.error (failure (0) (lit ("a module is an object"))) | LexLeanStd.Json.Json.JInt _ _ => Except.error (failure (0) (lit ("a module is an object"))) | LexLeanStd.Json.Json.JStr _ => Except.error (failure (0) (lit ("a module is an object"))) | LexLeanStd.Json.Json.JArr _ => Except.error (failure (0) (lit ("a module is an object"))) | LexLeanStd.Json.Json.JNil => Except.error (failure (0) (lit ("a module is an object"))) | LexLeanStd.Json.Json.JItem _ _ => Except.error (failure (0) (lit ("a module is an object"))) | LexLeanStd.Json.Json.JMember _ _ _ => Except.error (failure (0) (lit ("a module is an object"))))

@[expose] public def whiteStep (text : ByteArray) (n : Nat) (position : Nat) : Option (Nat) := (if (Nat.blt (position) (n)) then (let white : Nat := LexLeanBoot.AxiomOutput.whiteLength (text) (position); (if (Nat.blt (0) (white)) then Option.some ((position + white)) else Option.none)) else Option.none)

@[expose] public def firstNonWhite (text : ByteArray) : Nat := (let n : Nat := (LexLeanRuntime.length (text) : Nat); (let reached : Nat := ((LexLeanCollections.iterateUntil ((fun (position : Nat) => whiteStep (text) (n) (position))) ((n + 1)) (0) : (Prod (Nat) (Bool)))).1; (if (Nat.blt (reached) (n)) then reached else 0)))

@[expose] public def memberStep (state : (Prod (LexLeanStd.Json.Json) (List ((Prod (ByteArray) (LexLeanStd.Json.Json)))))) : Option ((Prod (LexLeanStd.Json.Json) (List ((Prod (ByteArray) (LexLeanStd.Json.Json)))))) := (match (state).1 with | LexLeanStd.Json.Json.JMember key inner rest => Option.some ((rest, ((key, inner) :: (state).2))) | LexLeanStd.Json.Json.JBool _ => Option.none | LexLeanStd.Json.Json.JInt _ _ => Option.none | LexLeanStd.Json.Json.JStr _ => Option.none | LexLeanStd.Json.Json.JArr _ => Option.none | LexLeanStd.Json.Json.JObj _ => Option.none | LexLeanStd.Json.Json.JNil => Option.none | LexLeanStd.Json.Json.JItem _ _ => Option.none)

@[expose] public def membersOf (chain : LexLeanStd.Json.Json) (fuel : Nat) : List ((Prod (ByteArray) (LexLeanStd.Json.Json))) := (((LexLeanCollections.iterateUntil ((fun (state : (Prod (LexLeanStd.Json.Json) (List ((Prod (ByteArray) (LexLeanStd.Json.Json)))))) => memberStep (state))) ((fuel + 1)) ((chain, ([] : List ((Prod (ByteArray) (LexLeanStd.Json.Json)))))) : (Prod ((Prod (LexLeanStd.Json.Json) (List ((Prod (ByteArray) (LexLeanStd.Json.Json)))))) (Bool)))).1).2

@[expose] public def specOffset (text : ByteArray) : Nat := (match LexLeanStd.Text.find (text) (lit ("spec")) with | Option.some found => found | Option.none => 0)

@[expose] public def parseToolingModule (text : ByteArray) (spec : ByteArray) : Except (LexLeanBoot.SurfaceLex.Failure) (LexLeanStd.Json.Json) := (match LexLeanBoot.SurfaceGeneric.parseGeneric (text) with | Except.error stopped => Except.error (stopped) | Except.ok value => (match value with | LexLeanStd.Json.Json.JObj chain => (let members : List ((Prod (ByteArray) (LexLeanStd.Json.Json))) := membersOf (chain) ((LexLeanRuntime.length (text) : Nat)); (if LexLeanBoot.SurfaceGeneric.hasKey (members) (lit ("spec")) then Except.error (failure (specOffset (text)) (lit ("the `spec` of a module is implied by its language and is not written"))) else Except.ok (LexLeanStd.Json.Json.JObj (LexLeanBoot.SurfaceGeneric.memberChain (LexLeanStd.Order.sortByKey (LexLeanStd.Json.Json) (((lit ("spec"), LexLeanStd.Json.Json.JStr (spec)) :: members))))))) | LexLeanStd.Json.Json.JBool _ => Except.error (failure (firstNonWhite (text)) (lit ("a module is an object"))) | LexLeanStd.Json.Json.JInt _ _ => Except.error (failure (firstNonWhite (text)) (lit ("a module is an object"))) | LexLeanStd.Json.Json.JStr _ => Except.error (failure (firstNonWhite (text)) (lit ("a module is an object"))) | LexLeanStd.Json.Json.JArr _ => Except.error (failure (firstNonWhite (text)) (lit ("a module is an object"))) | LexLeanStd.Json.Json.JNil => Except.error (failure (firstNonWhite (text)) (lit ("a module is an object"))) | LexLeanStd.Json.Json.JItem _ _ => Except.error (failure (firstNonWhite (text)) (lit ("a module is an object"))) | LexLeanStd.Json.Json.JMember _ _ _ => Except.error (failure (firstNonWhite (text)) (lit ("a module is an object")))))

@[expose] public def lastEnd (toks : List (LexLeanBoot.SurfaceLex.Spanned)) (position : Nat) : Nat := (if (Nat.beq (position) (0)) then 0 else (match (LexLeanRuntime.index (toks) ((LexLeanRuntime.subtract (position) (1) : Nat)) : Option (LexLeanBoot.SurfaceLex.Spanned)) with | Option.some found => (found).stop | Option.none => 0))

@[expose] public def spAt (st : Sp) (position : Nat) (phase : Nat) : Sp := ({ pos := position, phase := phase, spans := (st).spans, error := Option.none } : Sp)

@[expose] public def spFail (st : Sp) (offset : Nat) (message : ByteArray) : Sp := ({ pos := (st).pos, phase := 9, spans := (st).spans, error := Option.some (failure (offset) (message)) } : Sp)

@[expose] public def spRead (toks : List (LexLeanBoot.SurfaceLex.Spanned)) (size : Nat) (st : Sp) (depth : Nat) (inside : Bool) : Sp := (let start : Nat := (st).pos; (let outcome : LexLeanBoot.SurfaceGeneric.Rd := LexLeanBoot.SurfaceGeneric.readFrom (toks) (size) (start) (depth); (match (outcome).error with | Option.some stopped => ({ pos := (st).pos, phase := 9, spans := (st).spans, error := Option.some (stopped) } : Sp) | Option.none => (let after : Nat := (outcome).pos; (if inside then (let spans : List ((Prod (Nat) (Nat))) := ((LexLeanBoot.SurfaceGeneric.offsetAt (toks) (size) (start), lastEnd (toks) (after)) :: (st).spans); (if (Nat.beq (LexLeanBoot.SurfaceGeneric.codeAt (toks) (after)) (6)) then ({ pos := (after + 1), phase := 2, spans := spans, error := Option.none } : Sp) else ({ pos := after, phase := 3, spans := spans, error := Option.none } : Sp))) else spAt (st) (after) (4))))))

@[expose] public def spMember (toks : List (LexLeanBoot.SurfaceLex.Spanned)) (size : Nat) (st : Sp) : Sp := (let position : Nat := (st).pos; (let code : Nat := LexLeanBoot.SurfaceGeneric.codeAt (toks) (position); (if (Nat.beq (code) (1)) then spAt (st) (position) (5) else (if (if (Nat.beq (code) (11)) then true else (Nat.beq (code) (12))) then (if (Nat.beq (LexLeanBoot.SurfaceGeneric.codeAt (toks) ((position + 1))) (7)) then (if (LexLeanRuntime.equal (LexLeanBoot.SurfaceGeneric.textAt (toks) (position)) (lit ("declarations")) : Bool) then (if (Nat.beq (LexLeanBoot.SurfaceGeneric.codeAt (toks) ((position + 2))) (2)) then spAt (st) ((position + 3)) (2) else spFail (st) (LexLeanBoot.SurfaceGeneric.offsetAt (toks) (size) ((position + 2))) (lit ("expected `[` after `declarations`"))) else spRead (toks) (size) (spAt (st) ((position + 2)) (1)) (1) (false)) else spFail (st) (LexLeanBoot.SurfaceGeneric.offsetAt (toks) (size) ((position + 1))) (lit ("expected `:` after a member name"))) else spFail (st) (LexLeanBoot.SurfaceGeneric.offsetAt (toks) (size) (position)) (lit ("expected a member name"))))))

@[expose] public def spEnd (toks : List (LexLeanBoot.SurfaceLex.Spanned)) (size : Nat) (st : Sp) : Sp := (let position : Nat := (st).pos; (if (Nat.beq (LexLeanBoot.SurfaceGeneric.codeAt (toks) (position)) (1)) then (if (Nat.blt ((position + 1)) ((LexLeanRuntime.length (toks) : Nat))) then spFail (st) (LexLeanBoot.SurfaceGeneric.offsetAt (toks) (size) ((position + 1))) (lit ("unexpected text after the module")) else spAt (st) ((position + 1)) (8)) else spFail (st) (LexLeanBoot.SurfaceGeneric.offsetAt (toks) (size) (position)) (lit ("expected `,` or `}`"))))

@[expose] public def spPhase (toks : List (LexLeanBoot.SurfaceLex.Spanned)) (size : Nat) (st : Sp) : Sp := (let phase : Nat := (st).phase; (let position : Nat := (st).pos; (if (Nat.beq (phase) (0)) then (if (Nat.beq (LexLeanBoot.SurfaceGeneric.codeAt (toks) (position)) (0)) then spAt (st) ((position + 1)) (1) else spFail (st) (LexLeanBoot.SurfaceGeneric.offsetAt (toks) (size) (position)) (lit ("expected `{`"))) else (if (Nat.beq (phase) (1)) then spMember (toks) (size) (st) else (if (Nat.beq (phase) (2)) then (if (Nat.beq (LexLeanBoot.SurfaceGeneric.codeAt (toks) (position)) (3)) then spAt (st) (position) (3) else spRead (toks) (size) (st) (2) (true)) else (if (Nat.beq (phase) (3)) then (if (Nat.beq (LexLeanBoot.SurfaceGeneric.codeAt (toks) (position)) (3)) then spAt (st) ((position + 1)) (4) else spFail (st) (LexLeanBoot.SurfaceGeneric.offsetAt (toks) (size) (position)) (lit ("expected `,` or `]`"))) else (if (Nat.beq (phase) (4)) then (if (Nat.beq (LexLeanBoot.SurfaceGeneric.codeAt (toks) (position)) (6)) then spAt (st) ((position + 1)) (1) else spAt (st) (position) (5)) else spEnd (toks) (size) (st))))))))

@[expose] public def spStep (toks : List (LexLeanBoot.SurfaceLex.Spanned)) (size : Nat) (st : Sp) : Option (Sp) := (if (Nat.blt ((st).phase) (8)) then Option.some (spPhase (toks) (size) (st)) else Option.none)

@[expose] public def declarationSpans (text : ByteArray) : Except (LexLeanBoot.SurfaceLex.Failure) (List ((Prod (Nat) (Nat)))) := (match LexLeanBoot.SurfaceLex.lexText (text) with | Except.error stopped => Except.error (stopped) | Except.ok toks => (let size : Nat := (LexLeanRuntime.length (text) : Nat); (let final : Sp := ((LexLeanCollections.iterateUntil ((fun (st : Sp) => spStep (toks) (size) (st))) (((LexLeanRuntime.multiply ((LexLeanRuntime.length (toks) : Nat)) (2) : Nat) + 4)) (({ pos := 0, phase := 0, spans := ([] : List ((Prod (Nat) (Nat)))), error := Option.none } : Sp)) : (Prod (Sp) (Bool)))).1; (match (final).error with | Option.some stopped => Except.error (stopped) | Option.none => (if (Nat.beq ((final).phase) (8)) then Except.ok (LexLeanStd.Order.reverseList ((Prod (Nat) (Nat))) ((final).spans)) else Except.error (failure (size) (lit ("the reader ran out of steps"))))))))

end LexLeanBoot.SurfaceTooling
