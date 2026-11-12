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
namespace LexLeanBoot.LexiconLse

public inductive Tok where
  | TOpen
  | TClose
  | TAtom (_ : ByteArray)

public structure Failure where
  kind : Nat
  message : ByteArray

public inductive Node where
  | NAtom (_ : ByteArray)
  | NList (_ : Node)
  | NNil
  | NItem (_ : Node) (_ : Node)

public inductive Universe where
  | UNum (_ : Nat)
  | UVar (_ : ByteArray)
  | USucc (_ : Universe)
  | UMax (_ : Universe)
  | UIMax (_ : Universe) (_ : Universe)
  | UNil
  | UItem (_ : Universe) (_ : Universe)

public inductive Lse where
  | LSortProp
  | LSortType (_ : Universe)
  | LConst (_ : ByteArray) (_ : ByteArray) (_ : Universe)
  | LLocal (_ : ByteArray)
  | LApp (_ : Lse) (_ : Lse)
  | LPi (_ : Lse) (_ : Lse)
  | LLam (_ : Lse) (_ : Lse)
  | LLet (_ : ByteArray) (_ : Lse) (_ : Lse) (_ : Lse)
  | LNat (_ : ByteArray)
  | LNil
  | LArg (_ : Lse) (_ : Lse)
  | LBinder (_ : Nat) (_ : ByteArray) (_ : Lse) (_ : Lse)

public inductive Work where
  | WVisit (_ : Node)
  | WBuild (_ : Node)

public inductive UWork where
  | UVisit (_ : Node)
  | UBuild (_ : ByteArray) (_ : Nat)

public structure Tk where
  pos : Nat
  start : Nat
  inAtom : Bool
  toks : List (Tok)
  error : Option (Failure)

public structure Rt where
  pos : Nat
  stack : List (List (Node))
  root : Option (Node)
  error : Option (Failure)
  done : Bool

public structure Cv where
  work : List (Work)
  values : List (Lse)
  error : Option (Failure)

public structure Uv where
  work : List (UWork)
  values : List (Universe)
  error : Option (Failure)

public structure Header where
  mode : Nat
  name : ByteArray
  ty : Node

@[expose] public def lit (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def cat (left : ByteArray) (right : ByteArray) : ByteArray := (LexLeanRuntime.append (left) (right) : ByteArray)

@[expose] public def quoted (text : ByteArray) : ByteArray := cat (cat (lit ("`")) (text)) (lit ("`"))

@[expose] public def failed (message : ByteArray) : Failure := ({ kind := 0, message := message } : Failure)

@[expose] public def byteNum (subject : ByteArray) (position : Nat) : Nat := (match LexLeanStd.Text.byteAt (subject) (position) with | Option.some byte => (let __widen0 : UInt8 := byte; ((((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((128 : UInt8)) : UInt8)) ((128 : UInt8)) : Bool) then 128 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((64 : UInt8)) : UInt8)) ((64 : UInt8)) : Bool) then 64 else 0)) + ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((32 : UInt8)) : UInt8)) ((32 : UInt8)) : Bool) then 32 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((16 : UInt8)) : UInt8)) ((16 : UInt8)) : Bool) then 16 else 0))) + (((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((8 : UInt8)) : UInt8)) ((8 : UInt8)) : Bool) then 8 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((4 : UInt8)) : UInt8)) ((4 : UInt8)) : Bool) then 4 else 0)) + ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((2 : UInt8)) : UInt8)) ((2 : UInt8)) : Bool) then 2 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((1 : UInt8)) : UInt8)) ((1 : UInt8)) : Bool) then 1 else 0))))) | Option.none => 256)

@[expose] public def isLowerNum (value : Nat) : Bool := (if (Nat.blt (value) (97)) then false else (Nat.ble (value) (122)))

@[expose] public def isAlphaNum (value : Nat) : Bool := (if isLowerNum (value) then true else (if (Nat.blt (value) (65)) then false else (Nat.ble (value) (90))))

@[expose] public def isDigitNum (value : Nat) : Bool := (if (Nat.blt (value) (48)) then false else (Nat.ble (value) (57)))

@[expose] public def isAlnumNum (value : Nat) : Bool := (if isAlphaNum (value) then true else isDigitNum (value))

@[expose] public def scanIdentifier (subject : ByteArray) (n : Nat) (position : Nat) : Option (Nat) := (if (Nat.blt (position) (n)) then (if isAlnumNum (byteNum (subject) (position)) then Option.some ((position + 1)) else Option.none) else Option.none)

@[expose] public def isLseIdentifier (text : ByteArray) : Bool := (let n : Nat := (LexLeanRuntime.length (text) : Nat); (if (Nat.beq (n) (0)) then false else (if isAlphaNum (byteNum (text) (0)) then (Nat.beq (((LexLeanCollections.iterateUntil ((fun (position : Nat) => scanIdentifier (text) (n) (position))) (n) (1) : (Prod (Nat) (Bool)))).1) (n)) else false)))

public structure Sg where
  pos : Nat
  segStart : Bool
  firstSeg : Bool
  ok : Bool

@[expose] public def sgStop (st : Sg) : Option (Sg) := Option.some (({ pos := (st).pos, segStart := (st).segStart, firstSeg := (st).firstSeg, ok := false } : Sg))

@[expose] public def sgStep (st : Sg) (segStart : Bool) (firstSeg : Bool) : Option (Sg) := Option.some (({ pos := ((st).pos + 1), segStart := segStart, firstSeg := firstSeg, ok := true } : Sg))

@[expose] public def idStep (subject : ByteArray) (n : Nat) (hyphenFirst : Bool) (st : Sg) : Option (Sg) := (if (st).ok then (if (Nat.blt ((st).pos) (n)) then (let value : Nat := byteNum (subject) ((st).pos); (if (Nat.beq (value) (46)) then (if (st).segStart then sgStop (st) else sgStep (st) (true) (false)) else (if (st).segStart then (if isLowerNum (value) then sgStep (st) (false) ((st).firstSeg) else sgStop (st)) else (if (if isLowerNum (value) then true else isDigitNum (value)) then sgStep (st) (false) ((st).firstSeg) else (if (if (Nat.beq (value) (45)) then (if hyphenFirst then true else (if (st).firstSeg then false else true)) else false) then sgStep (st) (false) ((st).firstSeg) else sgStop (st)))))) else Option.none) else Option.none)

@[expose] public def idScan (text : ByteArray) (hyphenFirst : Bool) : Bool := (let n : Nat := (LexLeanRuntime.length (text) : Nat); (let final : Sg := ((LexLeanCollections.iterateUntil ((fun (st : Sg) => idStep (text) (n) (hyphenFirst) (st))) ((n + 1)) (({ pos := 0, segStart := true, firstSeg := true, ok := true } : Sg)) : (Prod (Sg) (Bool)))).1; (if (final).ok then (if (final).segStart then false else true) else false)))

@[expose] public def isPackageId (text : ByteArray) : Bool := idScan (text) (false)

@[expose] public def isEntryId (text : ByteArray) : Bool := idScan (text) (true)

@[expose] public def scanDigits (subject : ByteArray) (n : Nat) (position : Nat) : Option (Nat) := (if (Nat.blt (position) (n)) then (if isDigitNum (byteNum (subject) (position)) then Option.some ((position + 1)) else Option.none) else Option.none)

@[expose] public def allDigits (text : ByteArray) : Bool := (let n : Nat := (LexLeanRuntime.length (text) : Nat); (Nat.beq (((LexLeanCollections.iterateUntil ((fun (position : Nat) => scanDigits (text) (n) (position))) ((n + 1)) (0) : (Prod (Nat) (Bool)))).1) (n)))

@[expose] public def decimalFault (text : ByteArray) : Option (Failure) := (let n : Nat := (LexLeanRuntime.length (text) : Nat); (if (if (Nat.beq (n) (0)) then true else (if allDigits (text) then false else true)) then Option.some (failed (cat (quoted (text)) (lit (" is not a decimal")))) else (if (if (Nat.blt (1) (n)) then (Nat.beq (byteNum (text) (0)) (48)) else false) then Option.some (failed (cat (quoted (text)) (lit (" has a redundant leading zero")))) else (if (if (Nat.blt (20) (n)) then true else (if (Nat.beq (n) (20)) then (LexLeanRuntime.equal ((LexLeanRuntime.compareBytes (text) (lit ("18446744073709551615")) : Ordering)) ((LexLeanRuntime.compareBytes (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 1))]) (_root_.ByteArray.mk #[]) : Ordering)) : Bool) else false)) then Option.some (failed (cat (quoted (text)) (lit (" does not fit 64 bits")))) else Option.none))))

@[expose] public def decimalValue (text : ByteArray) : Nat := (match LexLeanStd.Numeral.parseDecimal (text) with | Option.some value => value | Option.none => 0)

@[expose] public def tkFlush (text : ByteArray) (st : Tk) : List (Tok) := (if (st).inAtom then (Tok.TAtom (LexLeanStd.Text.sliceOr (text) ((st).start) ((LexLeanRuntime.subtract ((st).pos) ((st).start) : Nat))) :: (st).toks) else (st).toks)

@[expose] public def tkStep (text : ByteArray) (n : Nat) (st : Tk) : Option (Tk) := (match (st).error with | Option.some _ => Option.none | Option.none => (let position : Nat := (st).pos; (if (Nat.blt (position) (n)) then (let value : Nat := byteNum (text) (position); (if (Nat.beq (value) (40)) then Option.some (({ pos := (position + 1), start := (st).start, inAtom := false, toks := (Tok.TOpen :: tkFlush (text) (st)), error := Option.none } : Tk)) else (if (Nat.beq (value) (41)) then Option.some (({ pos := (position + 1), start := (st).start, inAtom := false, toks := (Tok.TClose :: tkFlush (text) (st)), error := Option.none } : Tk)) else (if (if (Nat.beq (value) (32)) then true else (Nat.beq (value) (10))) then Option.some (({ pos := (position + 1), start := (st).start, inAtom := false, toks := tkFlush (text) (st), error := Option.none } : Tk)) else (if (if (Nat.blt (value) (32)) then true else (Nat.beq (value) (127))) then Option.some (({ pos := position, start := (st).start, inAtom := (st).inAtom, toks := (st).toks, error := Option.some (failed (lit ("whitespace is one or more ASCII spaces or LF"))) } : Tk)) else (if (st).inAtom then Option.some (({ pos := (position + 1), start := (st).start, inAtom := true, toks := (st).toks, error := Option.none } : Tk)) else Option.some (({ pos := (position + 1), start := position, inAtom := true, toks := (st).toks, error := Option.none } : Tk)))))))) else Option.none)))

@[expose] public def tokenize (text : ByteArray) : Except (Failure) (List (Tok)) := (let n : Nat := (LexLeanRuntime.length (text) : Nat); (let final : Tk := ((LexLeanCollections.iterateUntil ((fun (st : Tk) => tkStep (text) (n) (st))) ((n + 1)) (({ pos := 0, start := 0, inAtom := false, toks := ([] : List (Tok)), error := Option.none } : Tk)) : (Prod (Tk) (Bool)))).1; (match (final).error with | Option.some failure => Except.error (failure) | Option.none => Except.ok (LexLeanStd.Order.reverseList (Tok) (tkFlush (text) (final))))))

@[expose] public def chainOf (items : List (Node)) : Node := (LexLeanCollections.listFold ((fun (chain : Node) (item : Node) => Node.NItem (item) (chain))) (Node.NNil) (items) : Node)

@[expose] public def rtFail (st : Rt) (failure : Failure) : Rt := ({ pos := (st).pos, stack := (st).stack, root := (st).root, error := Option.some (failure), done := false } : Rt)

@[expose] public def rtPlace (st : Rt) (node : Node) (stack : List (List (Node))) : Rt := (match stack with | List.nil => ({ pos := ((st).pos + 1), stack := ([] : List (List (Node))), root := Option.some (node), error := Option.none, done := false } : Rt) | List.cons parent more => ({ pos := ((st).pos + 1), stack := ((node :: parent) :: more), root := (st).root, error := Option.none, done := false } : Rt))

@[expose] public def rtToken (maxDepth : Nat) (st : Rt) (tok : Tok) : Rt := (match tok with | Tok.TOpen => (if (Nat.blt ((LexLeanRuntime.length ((st).stack) : Nat)) (maxDepth)) then ({ pos := ((st).pos + 1), stack := (([] : List (Node)) :: (st).stack), root := (st).root, error := Option.none, done := false } : Rt) else rtFail (st) (({ kind := 1, message := cat (lit ("max_scope_depth exceeded: configured ")) (LexLeanStd.Numeral.decimal (maxDepth)) } : Failure))) | Tok.TClose => (match (st).stack with | List.nil => rtFail (st) (failed (lit ("unbalanced `)`"))) | List.cons top rest => rtPlace (st) (Node.NList (chainOf (top))) (rest)) | Tok.TAtom text => rtPlace (st) (Node.NAtom (text)) ((st).stack))

@[expose] public def rtStep (toks : List (Tok)) (maxDepth : Nat) (st : Rt) : Option (Rt) := (match (st).error with | Option.some _ => Option.none | Option.none => (if (st).done then Option.none else (match (LexLeanRuntime.index (toks) ((st).pos) : Option (Tok)) with | Option.none => (match (st).stack with | List.nil => (match (st).root with | Option.some _ => Option.some (({ pos := (st).pos, stack := ([] : List (List (Node))), root := (st).root, error := Option.none, done := true } : Rt)) | Option.none => Option.some (rtFail (st) (failed (lit ("unexpected end of expression"))))) | List.cons _ _ => Option.some (rtFail (st) (failed (lit ("unexpected end of expression"))))) | Option.some tok => (match (st).root with | Option.some _ => Option.some (rtFail (st) (failed (lit ("trailing tokens after the expression")))) | Option.none => Option.some (rtToken (maxDepth) (st) (tok))))))

@[expose] public def readTree (toks : List (Tok)) (maxDepth : Nat) : Except (Failure) (Node) := (let final : Rt := ((LexLeanCollections.iterateUntil ((fun (st : Rt) => rtStep (toks) (maxDepth) (st))) (((LexLeanRuntime.length (toks) : Nat) + 2)) (({ pos := 0, stack := ([] : List (List (Node))), root := Option.none, error := Option.none, done := false } : Rt)) : (Prod (Rt) (Bool)))).1; (match (final).error with | Option.some failure => Except.error (failure) | Option.none => (match (final).root with | Option.some node => Except.ok (node) | Option.none => Except.error (failed (lit ("unexpected end of expression"))))))

@[expose] public def itemStep (state : (Prod (Node) (List (Node)))) : Option ((Prod (Node) (List (Node)))) := (match (state).1 with | Node.NAtom _ => Option.none | Node.NList _ => Option.none | Node.NNil => Option.none | Node.NItem head tail => Option.some ((tail, (head :: (state).2))))

@[expose] public def itemsOf (chain : Node) (fuel : Nat) : List (Node) := LexLeanStd.Order.reverseList (Node) ((((LexLeanCollections.iterateUntil ((fun (state : (Prod (Node) (List (Node)))) => itemStep (state))) ((fuel + 1)) ((chain, ([] : List (Node)))) : (Prod ((Prod (Node) (List (Node)))) (Bool)))).1).2)

@[expose] public def describe (node : Node) : ByteArray := (match node with | Node.NAtom text => quoted (text) | Node.NList chain => (match chain with | Node.NItem head _ => (match head with | Node.NAtom text => cat (cat (lit ("`(")) (text)) (lit (" ...)`")) | Node.NList _ => lit ("a list") | Node.NNil => lit ("a list") | Node.NItem _ _ => lit ("a list")) | Node.NAtom _ => lit ("a list") | Node.NList _ => lit ("a list") | Node.NNil => lit ("a list")) | Node.NNil => lit ("a list") | Node.NItem _ _ => lit ("a list"))

@[expose] public def nodeAt (items : List (Node)) (position : Nat) : Node := (match (LexLeanRuntime.index (items) (position) : Option (Node)) with | Option.some node => node | Option.none => Node.NNil)

@[expose] public def atomOf (node : Node) (what : ByteArray) (_fuel : Nat) : Except (Failure) (ByteArray) := (match node with | Node.NAtom text => Except.ok (text) | Node.NList _ => Except.error (failed (cat (cat (cat (lit ("expected ")) (what)) (lit (", found "))) (describe (node)))) | Node.NNil => Except.error (failed (cat (cat (cat (lit ("expected ")) (what)) (lit (", found "))) (describe (node)))) | Node.NItem _ _ => Except.error (failed (cat (cat (cat (lit ("expected ")) (what)) (lit (", found "))) (describe (node)))))

@[expose] public def qualifiedId (text : ByteArray) : Except (Failure) ((Prod (ByteArray) (ByteArray))) := (match LexLeanStd.Text.find (text) (lit ("::")) with | Option.none => Except.error (failed (cat (quoted (text)) (lit (" is not a qualified `package::entry` ID")))) | Option.some cut => (let package : ByteArray := LexLeanStd.Text.sliceOr (text) (0) (cut); (let entry : ByteArray := LexLeanStd.Text.sliceOr (text) ((cut + 2)) ((LexLeanRuntime.subtract ((LexLeanRuntime.length (text) : Nat)) ((cut + 2)) : Nat)); (if isPackageId (package) then (if isEntryId (entry) then Except.ok ((package, entry)) else Except.error (failed (cat (quoted (entry)) (lit (" is not a valid local entry ID"))))) else Except.error (failed (cat (quoted (package)) (lit (" is not a valid package ID"))))))))

@[expose] public def uvFail (st : Uv) (failure : Failure) : Uv := ({ work := (st).work, values := (st).values, error := Option.some (failure) } : Uv)

@[expose] public def pushVisitsU (items : List (Node)) (work : List (UWork)) : List (UWork) := (LexLeanCollections.listFold ((fun (acc : List (UWork)) (item : Node) => (UWork.UVisit (item) :: acc))) (work) (LexLeanStd.Order.reverseList (Node) (items)) : List (UWork))

@[expose] public def takeUniverses (values : List (Universe)) (count : Nat) : (Prod (List (Universe)) (List (Universe))) := ((LexLeanCollections.iterateUntil ((fun (state : (Prod (List (Universe)) (List (Universe)))) => (match (state).2 with | List.nil => Option.none | List.cons top rest => Option.some (((top :: (state).1), rest))))) (count) ((([] : List (Universe)), values)) : (Prod ((Prod (List (Universe)) (List (Universe)))) (Bool)))).1

@[expose] public def universeChain (items : List (Universe)) : Universe := (LexLeanCollections.listFold ((fun (chain : Universe) (item : Universe) => Universe.UItem (item) (chain))) (Universe.UNil) (LexLeanStd.Order.reverseList (Universe) (items)) : Universe)

@[expose] public def uvVisit (fuel : Nat) (st : Uv) (rest : List (UWork)) (node : Node) : Uv := (match node with | Node.NAtom text => (if allDigits (text) then (match decimalFault (text) with | Option.some failure => uvFail (st) (failure) | Option.none => ({ work := rest, values := (Universe.UNum (decimalValue (text)) :: (st).values), error := Option.none } : Uv)) else (if isLseIdentifier (text) then ({ work := rest, values := (Universe.UVar (text) :: (st).values), error := Option.none } : Uv) else uvFail (st) (failed (cat (quoted (text)) (lit (" is not a universe")))))) | Node.NList chain => (let items : List (Node) := itemsOf (chain) (fuel); (match items with | List.nil => uvFail (st) (failed (lit ("expected a universe form"))) | List.cons headNode arguments => (match headNode with | Node.NAtom head => (let argc : Nat := (LexLeanRuntime.length (arguments) : Nat); (let known : Bool := (if (LexLeanRuntime.equal (head) (lit ("succ")) : Bool) then true else (if (LexLeanRuntime.equal (head) (lit ("max")) : Bool) then true else (LexLeanRuntime.equal (head) (lit ("imax")) : Bool))); (let fits : Bool := (if (LexLeanRuntime.equal (head) (lit ("succ")) : Bool) then (Nat.beq (argc) (1)) else (if (LexLeanRuntime.equal (head) (lit ("max")) : Bool) then (Nat.ble (2) (argc)) else (if (LexLeanRuntime.equal (head) (lit ("imax")) : Bool) then (Nat.beq (argc) (2)) else false))); (if fits then ({ work := pushVisitsU (arguments) ((UWork.UBuild (head) (argc) :: rest)), values := (st).values, error := Option.none } : Uv) else (if known then uvFail (st) (failed (cat (cat (lit ("`(")) (head)) (lit (" ...)` has the wrong arity")))) else uvFail (st) (failed (cat (quoted (head)) (lit (" is not a universe form"))))))))) | Node.NList _ => uvFail (st) (failed (lit ("expected a universe form"))) | Node.NNil => uvFail (st) (failed (lit ("expected a universe form"))) | Node.NItem _ _ => uvFail (st) (failed (lit ("expected a universe form")))))) | Node.NNil => uvFail (st) (failed (lit ("expected a universe form"))) | Node.NItem _ _ => uvFail (st) (failed (lit ("expected a universe form"))))

@[expose] public def uvBuild (st : Uv) (rest : List (UWork)) (head : ByteArray) (count : Nat) : Uv := (let taken : (Prod (List (Universe)) (List (Universe))) := takeUniverses ((st).values) (count); (let parts : List (Universe) := (taken).1; (let built : Universe := (if (LexLeanRuntime.equal (head) (lit ("succ")) : Bool) then (match parts with | List.nil => Universe.UNil | List.cons a _ => Universe.USucc (a)) else (if (LexLeanRuntime.equal (head) (lit ("imax")) : Bool) then (match parts with | List.nil => Universe.UNil | List.cons a more => (match more with | List.nil => Universe.UNil | List.cons b _ => Universe.UIMax (a) (b))) else Universe.UMax (universeChain (parts)))); ({ work := rest, values := (built :: (taken).2), error := Option.none } : Uv))))

@[expose] public def uvStep (fuel : Nat) (st : Uv) : Option (Uv) := (match (st).error with | Option.some _ => Option.none | Option.none => (match (st).work with | List.nil => Option.none | List.cons item rest => (match item with | UWork.UVisit node => Option.some (uvVisit (fuel) (st) (rest) (node)) | UWork.UBuild head count => Option.some (uvBuild (st) (rest) (head) (count)))))

@[expose] public def universeOf (node : Node) (fuel : Nat) : Except (Failure) (Universe) := (let final : Uv := ((LexLeanCollections.iterateUntil ((fun (st : Uv) => uvStep (fuel) (st))) (((LexLeanRuntime.multiply (fuel) (2) : Nat) + 2)) (({ work := (UWork.UVisit (node) :: ([] : List (UWork))), values := ([] : List (Universe)), error := Option.none } : Uv)) : (Prod (Uv) (Bool)))).1; (match (final).error with | Option.some failure => Except.error (failure) | Option.none => (match (final).values with | List.nil => Except.error (failed (lit ("expected a universe"))) | List.cons top _ => Except.ok (top))))

@[expose] public def universesOf (items : List (Node)) (fuel : Nat) : Except (Failure) (Universe) := (match (LexLeanCollections.listFold ((fun (acc : Except (Failure) (List (Universe))) (item : Node) => (match acc with | Except.ok kept => (match universeOf (item) (fuel) with | Except.ok level => Except.ok ((level :: kept)) | Except.error failure => Except.error (failure)) | Except.error failure => Except.error (failure)))) (Except.ok (([] : List (Universe)))) (items) : Except (Failure) (List (Universe))) with | Except.ok reversed => Except.ok (universeChain (LexLeanStd.Order.reverseList (Universe) (reversed))) | Except.error failure => Except.error (failure))

@[expose] public def sortError : Failure := failed (lit ("a sort is `(sort prop)` or `(sort (type u))`"))

@[expose] public def leafSort (arguments : List (Node)) (fuel : Nat) : Except (Failure) (Lse) := (if (Nat.beq ((LexLeanRuntime.length (arguments) : Nat)) (1)) then (match nodeAt (arguments) (0) with | Node.NAtom text => (if (LexLeanRuntime.equal (text) (lit ("prop")) : Bool) then Except.ok (Lse.LSortProp) else Except.error (sortError)) | Node.NList inner => (let items : List (Node) := itemsOf (inner) (fuel); (if (Nat.beq ((LexLeanRuntime.length (items) : Nat)) (2)) then (match nodeAt (items) (0) with | Node.NAtom keyword => (if (LexLeanRuntime.equal (keyword) (lit ("type")) : Bool) then (match universeOf (nodeAt (items) (1)) (fuel) with | Except.ok level => Except.ok (Lse.LSortType (level)) | Except.error failure => Except.error (failure)) else Except.error (sortError)) | Node.NList _ => Except.error (sortError) | Node.NNil => Except.error (sortError) | Node.NItem _ _ => Except.error (sortError)) else Except.error (sortError))) | Node.NNil => Except.error (sortError) | Node.NItem _ _ => Except.error (sortError)) else Except.error (sortError))

@[expose] public def constError : Failure := failed (lit ("a const is `(const qualified-id [universes])`"))

@[expose] public def leafConst (arguments : List (Node)) (fuel : Nat) : Except (Failure) (Lse) := (let argc : Nat := (LexLeanRuntime.length (arguments) : Nat); (match nodeAt (arguments) (0) with | Node.NAtom spelled => (if (Nat.beq (argc) (1)) then (match qualifiedId (spelled) with | Except.ok parts => Except.ok (Lse.LConst ((parts).1) ((parts).2) (Universe.UNil)) | Except.error failure => Except.error (failure)) else (if (Nat.beq (argc) (2)) then (match nodeAt (arguments) (1) with | Node.NList universes => (let items : List (Node) := itemsOf (universes) (fuel); (match items with | List.nil => Except.error (constError) | List.cons _ _ => (match qualifiedId (spelled) with | Except.ok parts => (match universesOf (items) (fuel) with | Except.ok chain => Except.ok (Lse.LConst ((parts).1) ((parts).2) (chain)) | Except.error failure => Except.error (failure)) | Except.error failure => Except.error (failure)))) | Node.NAtom _ => Except.error (constError) | Node.NNil => Except.error (constError) | Node.NItem _ _ => Except.error (constError)) else Except.error (constError))) | Node.NList _ => Except.error (constError) | Node.NNil => Except.error (constError) | Node.NItem _ _ => Except.error (constError)))

@[expose] public def leafLocal (arguments : List (Node)) : Except (Failure) (Lse) := (if (Nat.beq ((LexLeanRuntime.length (arguments) : Nat)) (1)) then (match nodeAt (arguments) (0) with | Node.NAtom name => (if isLseIdentifier (name) then Except.ok (Lse.LLocal (name)) else Except.error (failed (cat (quoted (name)) (lit (" is not a valid local ID"))))) | Node.NList _ => Except.error (failed (lit ("a local is `(local id)`"))) | Node.NNil => Except.error (failed (lit ("a local is `(local id)`"))) | Node.NItem _ _ => Except.error (failed (lit ("a local is `(local id)`")))) else Except.error (failed (lit ("a local is `(local id)`"))))

@[expose] public def leafNat (arguments : List (Node)) : Except (Failure) (Lse) := (if (Nat.beq ((LexLeanRuntime.length (arguments) : Nat)) (1)) then (match nodeAt (arguments) (0) with | Node.NAtom digits => (match decimalFault (digits) with | Option.some failure => Except.error (failure) | Option.none => Except.ok (Lse.LNat (digits))) | Node.NList _ => Except.error (failed (lit ("a nat is `(nat decimal)`"))) | Node.NNil => Except.error (failed (lit ("a nat is `(nat decimal)`"))) | Node.NItem _ _ => Except.error (failed (lit ("a nat is `(nat decimal)`")))) else Except.error (failed (lit ("a nat is `(nat decimal)`"))))

@[expose] public def leafOf (head : ByteArray) (arguments : List (Node)) (fuel : Nat) : Except (Failure) (Lse) := (if (LexLeanRuntime.equal (head) (lit ("sort")) : Bool) then leafSort (arguments) (fuel) else (if (LexLeanRuntime.equal (head) (lit ("const")) : Bool) then leafConst (arguments) (fuel) else (if (LexLeanRuntime.equal (head) (lit ("local")) : Bool) then leafLocal (arguments) else (if (LexLeanRuntime.equal (head) (lit ("nat")) : Bool) then leafNat (arguments) else Except.error (failed (cat (quoted (head)) (lit (" is not an LSE form"))))))))

@[expose] public def binderMode (text : ByteArray) : Option (Nat) := (if (LexLeanRuntime.equal (text) (lit ("explicit")) : Bool) then Option.some (0) else (if (LexLeanRuntime.equal (text) (lit ("implicit")) : Bool) then Option.some (1) else (if (LexLeanRuntime.equal (text) (lit ("instance")) : Bool) then Option.some (2) else Option.none)))

@[expose] public def binderHeader (node : Node) (fuel : Nat) : Except (Failure) (Header) := (match node with | Node.NList chain => (let items : List (Node) := itemsOf (chain) (fuel); (if (Nat.beq ((LexLeanRuntime.length (items) : Nat)) (3)) then (match atomOf (nodeAt (items) (0)) (lit ("a binder mode")) (fuel) with | Except.ok modeWord => (match binderMode (modeWord) with | Option.some mode => (match atomOf (nodeAt (items) (1)) (lit ("a local ID")) (fuel) with | Except.ok name => (if isLseIdentifier (name) then Except.ok (({ mode := mode, name := name, ty := nodeAt (items) (2) } : Header)) else Except.error (failed (cat (quoted (name)) (lit (" is not a valid local ID"))))) | Except.error failure => Except.error (failure)) | Option.none => Except.error (failed (cat (quoted (modeWord)) (lit (" is not a binder mode"))))) | Except.error failure => Except.error (failure)) else Except.error (failed (lit ("a binder is `(mode name type)`"))))) | Node.NAtom _ => Except.error (failed (cat (lit ("expected a binder, found ")) (describe (node)))) | Node.NNil => Except.error (failed (cat (lit ("expected a binder, found ")) (describe (node)))) | Node.NItem _ _ => Except.error (failed (cat (lit ("expected a binder, found ")) (describe (node)))))

@[expose] public def cvFail (st : Cv) (failure : Failure) : Cv := ({ work := (st).work, values := (st).values, error := Option.some (failure) } : Cv)

@[expose] public def pushVisits (items : List (Node)) (work : List (Work)) : List (Work) := (LexLeanCollections.listFold ((fun (acc : List (Work)) (item : Node) => (Work.WVisit (item) :: acc))) (work) (LexLeanStd.Order.reverseList (Node) (items)) : List (Work))

@[expose] public def binderVisits (binders : List (Node)) (work : List (Work)) (fuel : Nat) : Except (Failure) (List (Work)) := (LexLeanCollections.listFold ((fun (acc : Except (Failure) (List (Work))) (binder : Node) => (match acc with | Except.ok pending => (match binderHeader (binder) (fuel) with | Except.ok header => Except.ok ((Work.WVisit ((header).ty) :: pending)) | Except.error failure => Except.error (failure)) | Except.error failure => Except.error (failure)))) (Except.ok (work)) (LexLeanStd.Order.reverseList (Node) (binders)) : Except (Failure) (List (Work)))

@[expose] public def isBinderForm (head : ByteArray) : Bool := (if (LexLeanRuntime.equal (head) (lit ("pi")) : Bool) then true else (LexLeanRuntime.equal (head) (lit ("lam")) : Bool))

@[expose] public def isLeafForm (head : ByteArray) : Bool := (if (LexLeanRuntime.equal (head) (lit ("sort")) : Bool) then true else (if (LexLeanRuntime.equal (head) (lit ("const")) : Bool) then true else (if (LexLeanRuntime.equal (head) (lit ("local")) : Bool) then true else (LexLeanRuntime.equal (head) (lit ("nat")) : Bool))))

@[expose] public def visitBinderForm (st : Cv) (rest : List (Work)) (node : Node) (head : ByteArray) (arguments : List (Node)) (fuel : Nat) : Cv := (if (Nat.beq ((LexLeanRuntime.length (arguments) : Nat)) (2)) then (match nodeAt (arguments) (0) with | Node.NList binderChain => (let binders : List (Node) := itemsOf (binderChain) (fuel); (match binders with | List.nil => cvFail (st) (failed (cat (quoted (head)) (lit (" has at least one binder")))) | List.cons _ _ => (match binderVisits (binders) ((Work.WVisit (nodeAt (arguments) (1)) :: (Work.WBuild (node) :: rest))) (fuel) with | Except.ok work => ({ work := work, values := (st).values, error := Option.none } : Cv) | Except.error failure => cvFail (st) (failure)))) | Node.NAtom _ => cvFail (st) (failed (lit ("expected a binder list"))) | Node.NNil => cvFail (st) (failed (lit ("expected a binder list"))) | Node.NItem _ _ => cvFail (st) (failed (lit ("expected a binder list")))) else cvFail (st) (failed (cat (cat (cat (cat (lit ("`")) (head)) (lit ("` is `("))) (head)) (lit (" (binders...) body)`")))))

@[expose] public def visitLet (st : Cv) (rest : List (Work)) (node : Node) (arguments : List (Node)) (fuel : Nat) : Cv := (if (Nat.beq ((LexLeanRuntime.length (arguments) : Nat)) (4)) then (match atomOf (nodeAt (arguments) (0)) (lit ("a local ID")) (fuel) with | Except.ok name => (if isLseIdentifier (name) then ({ work := (Work.WVisit (nodeAt (arguments) (1)) :: (Work.WVisit (nodeAt (arguments) (2)) :: (Work.WVisit (nodeAt (arguments) (3)) :: (Work.WBuild (node) :: rest)))), values := (st).values, error := Option.none } : Cv) else cvFail (st) (failed (cat (quoted (name)) (lit (" is not a valid local ID"))))) | Except.error failure => cvFail (st) (failure)) else cvFail (st) (failed (lit ("`let` is `(let name type value body)`"))))

@[expose] public def visitForm (st : Cv) (rest : List (Work)) (node : Node) (head : ByteArray) (arguments : List (Node)) (fuel : Nat) : Cv := (if isLeafForm (head) then (match leafOf (head) (arguments) (fuel) with | Except.ok value => ({ work := rest, values := (value :: (st).values), error := Option.none } : Cv) | Except.error failure => cvFail (st) (failure)) else (if (LexLeanRuntime.equal (head) (lit ("app")) : Bool) then (if (Nat.blt ((LexLeanRuntime.length (arguments) : Nat)) (2)) then cvFail (st) (failed (lit ("`app` has at least a function and one argument"))) else ({ work := pushVisits (arguments) ((Work.WBuild (node) :: rest)), values := (st).values, error := Option.none } : Cv)) else (if isBinderForm (head) then visitBinderForm (st) (rest) (node) (head) (arguments) (fuel) else (if (LexLeanRuntime.equal (head) (lit ("let")) : Bool) then visitLet (st) (rest) (node) (arguments) (fuel) else cvFail (st) (failed (cat (quoted (head)) (lit (" is not an LSE form"))))))))

@[expose] public def cvVisit (st : Cv) (rest : List (Work)) (node : Node) (fuel : Nat) : Cv := (match node with | Node.NList chain => (match itemsOf (chain) (fuel) with | List.nil => cvFail (st) (failed (lit ("expected an LSE form"))) | List.cons headNode arguments => (match headNode with | Node.NAtom head => visitForm (st) (rest) (node) (head) (arguments) (fuel) | Node.NList _ => cvFail (st) (failed (lit ("expected an LSE form"))) | Node.NNil => cvFail (st) (failed (lit ("expected an LSE form"))) | Node.NItem _ _ => cvFail (st) (failed (lit ("expected an LSE form"))))) | Node.NAtom _ => cvFail (st) (failed (cat (lit ("expected `(`, found ")) (describe (node)))) | Node.NNil => cvFail (st) (failed (cat (lit ("expected `(`, found ")) (describe (node)))) | Node.NItem _ _ => cvFail (st) (failed (cat (lit ("expected `(`, found ")) (describe (node)))))

@[expose] public def takeValues (values : List (Lse)) (count : Nat) : (Prod (List (Lse)) (List (Lse))) := ((LexLeanCollections.iterateUntil ((fun (state : (Prod (List (Lse)) (List (Lse)))) => (match (state).2 with | List.nil => Option.none | List.cons top rest => Option.some (((top :: (state).1), rest))))) (count) ((([] : List (Lse)), values)) : (Prod ((Prod (List (Lse)) (List (Lse)))) (Bool)))).1

@[expose] public def argChain (items : List (Lse)) : Lse := (LexLeanCollections.listFold ((fun (chain : Lse) (item : Lse) => Lse.LArg (item) (chain))) (Lse.LNil) (LexLeanStd.Order.reverseList (Lse) (items)) : Lse)

@[expose] public def binderChainOf (binders : List (Node)) (types : List (Lse)) (fuel : Nat) : Lse := ((LexLeanCollections.listFold ((fun (state : (Prod (List (Lse)) (Lse))) (binder : Node) => (match (state).1 with | List.nil => state | List.cons ty more => (match binderHeader (binder) (fuel) with | Except.ok header => (more, Lse.LBinder ((header).mode) ((header).name) (ty) ((state).2)) | Except.error _ => (more, (state).2))))) ((LexLeanStd.Order.reverseList (Lse) (types), Lse.LNil)) (LexLeanStd.Order.reverseList (Node) (binders)) : (Prod (List (Lse)) (Lse)))).2

@[expose] public def cvBuild (st : Cv) (rest : List (Work)) (node : Node) (fuel : Nat) : Cv := (match node with | Node.NList chain => (let items : List (Node) := itemsOf (chain) (fuel); (match items with | List.nil => cvFail (st) (failed (lit ("expected a list"))) | List.cons headNode arguments => (match headNode with | Node.NAtom head => (if (LexLeanRuntime.equal (head) (lit ("app")) : Bool) then (let taken : (Prod (List (Lse)) (List (Lse))) := takeValues ((st).values) ((LexLeanRuntime.length (arguments) : Nat)); (match (taken).1 with | List.nil => cvFail (st) (failed (lit ("missing function"))) | List.cons function parts => ({ work := rest, values := (Lse.LApp (function) (argChain (parts)) :: (taken).2), error := Option.none } : Cv))) else (if isBinderForm (head) then (match nodeAt (arguments) (0) with | Node.NList binderChain => (let binders : List (Node) := itemsOf (binderChain) (fuel); (let taken : (Prod (List (Lse)) (List (Lse))) := takeValues ((st).values) (((LexLeanRuntime.length (binders) : Nat) + 1)); (match LexLeanStd.Order.reverseList (Lse) ((taken).1) with | List.nil => cvFail (st) (failed (lit ("missing body"))) | List.cons body typesReversed => (let bound : Lse := binderChainOf (binders) (LexLeanStd.Order.reverseList (Lse) (typesReversed)) (fuel); ({ work := rest, values := ((if (LexLeanRuntime.equal (head) (lit ("pi")) : Bool) then Lse.LPi (bound) (body) else Lse.LLam (bound) (body)) :: (taken).2), error := Option.none } : Cv))))) | Node.NAtom _ => cvFail (st) (failed (lit ("expected a binder list"))) | Node.NNil => cvFail (st) (failed (lit ("expected a binder list"))) | Node.NItem _ _ => cvFail (st) (failed (lit ("expected a binder list")))) else (let taken : (Prod (List (Lse)) (List (Lse))) := takeValues ((st).values) (3); (match (taken).1 with | List.nil => cvFail (st) (failed (lit ("missing type"))) | List.cons ty afterTy => (match afterTy with | List.nil => cvFail (st) (failed (lit ("missing value"))) | List.cons value afterValue => (match afterValue with | List.nil => cvFail (st) (failed (lit ("missing body"))) | List.cons body _ => (match atomOf (nodeAt (arguments) (0)) (lit ("a local ID")) (fuel) with | Except.ok name => ({ work := rest, values := (Lse.LLet (name) (ty) (value) (body) :: (taken).2), error := Option.none } : Cv) | Except.error failure => cvFail (st) (failure)))))))) | Node.NList _ => cvFail (st) (failed (lit ("expected a form, found a list"))) | Node.NNil => cvFail (st) (failed (lit ("expected a form, found a list"))) | Node.NItem _ _ => cvFail (st) (failed (lit ("expected a form, found a list")))))) | Node.NAtom _ => cvFail (st) (failed (lit ("expected a list"))) | Node.NNil => cvFail (st) (failed (lit ("expected a list"))) | Node.NItem _ _ => cvFail (st) (failed (lit ("expected a list"))))

@[expose] public def cvStep (fuel : Nat) (st : Cv) : Option (Cv) := (match (st).error with | Option.some _ => Option.none | Option.none => (match (st).work with | List.nil => Option.none | List.cons item rest => (match item with | Work.WVisit node => Option.some (cvVisit (st) (rest) (node) (fuel)) | Work.WBuild node => Option.some (cvBuild (st) (rest) (node) (fuel)))))

@[expose] public def lseOf (root : Node) (fuel : Nat) : Except (Failure) (Lse) := (let final : Cv := ((LexLeanCollections.iterateUntil ((fun (st : Cv) => cvStep (fuel) (st))) (((LexLeanRuntime.multiply (fuel) (2) : Nat) + 2)) (({ work := (Work.WVisit (root) :: ([] : List (Work))), values := ([] : List (Lse)), error := Option.none } : Cv)) : (Prod (Cv) (Bool)))).1; (match (final).error with | Option.some failure => Except.error (failure) | Option.none => (match (final).values with | List.nil => Except.error (failed (lit ("malformed expression"))) | List.cons value more => (match more with | List.nil => Except.ok (value) | List.cons _ _ => Except.error (failed (lit ("malformed expression")))))))

@[expose] public def parseLse (text : ByteArray) (maxDepth : Nat) : Except (Failure) (Lse) := (match tokenize (text) with | Except.error failure => Except.error (failure) | Except.ok toks => (match readTree (toks) (maxDepth) with | Except.error failure => Except.error (failure) | Except.ok root => lseOf (root) ((LexLeanRuntime.length (toks) : Nat))))

@[expose] public def modeText (mode : Nat) : ByteArray := (if (Nat.beq (mode) (1)) then lit ("implicit") else (if (Nat.beq (mode) (2)) then lit ("instance") else lit ("explicit")))

@[expose] public def fresh (name : ByteArray) (rename : Bool) (counter : Nat) : (Prod (ByteArray) (Nat)) := (if rename then (cat (lit ("x")) (LexLeanStd.Numeral.decimal (counter)), (counter + 1)) else (name, counter))

@[expose] public def lookup (scope : List ((Prod (ByteArray) (ByteArray)))) (name : ByteArray) : ByteArray := (match (LexLeanCollections.listFold ((fun (found : Option (ByteArray)) (entry : (Prod (ByteArray) (ByteArray))) => (match found with | Option.some _ => found | Option.none => (if (LexLeanRuntime.equal ((entry).1) (name) : Bool) then Option.some ((entry).2) else Option.none)))) (Option.none) (scope) : Option (ByteArray)) with | Option.some shown => shown | Option.none => name)

mutual
@[expose] public def universeText : (level : Universe) -> ByteArray
  | Universe.UNum value => LexLeanStd.Numeral.decimal (value)
  | Universe.UVar name => name
  | Universe.USucc inner => cat (cat (lit ("(succ ")) (universeText (inner))) (lit (")"))
  | Universe.UMax items => cat (cat (lit ("(max")) (universeItems (items))) (lit (")"))
  | Universe.UIMax left right => cat (cat (cat (cat (lit ("(imax ")) (universeText (left))) (lit (" "))) (universeText (right))) (lit (")"))
  | Universe.UNil => _root_.ByteArray.mk #[]
  | Universe.UItem _ _ => _root_.ByteArray.mk #[]
termination_by structural level => level

@[expose] public def universeItems : (items : Universe) -> ByteArray
  | Universe.UNum _ => _root_.ByteArray.mk #[]
  | Universe.UVar _ => _root_.ByteArray.mk #[]
  | Universe.USucc _ => _root_.ByteArray.mk #[]
  | Universe.UMax _ => _root_.ByteArray.mk #[]
  | Universe.UIMax _ _ => _root_.ByteArray.mk #[]
  | Universe.UNil => _root_.ByteArray.mk #[]
  | Universe.UItem head tail => cat (cat (lit (" ")) (universeText (head))) (universeItems (tail))
termination_by structural items => items
end

@[expose] public def constText (package : ByteArray) (entry : ByteArray) (universes : Universe) : ByteArray := (let heading : ByteArray := cat (cat (cat (lit ("(const ")) (package)) (lit ("::"))) (entry); (match universes with | Universe.UNil => cat (heading) (lit (")")) | Universe.UItem _ _ => (let listed : ByteArray := universeItems (universes); cat (cat (cat (heading) (lit (" ("))) (LexLeanStd.Text.sliceOr (listed) (1) ((LexLeanRuntime.subtract ((LexLeanRuntime.length (listed) : Nat)) (1) : Nat)))) (lit ("))"))) | Universe.UNum _ => cat (heading) (lit (")")) | Universe.UVar _ => cat (heading) (lit (")")) | Universe.USucc _ => cat (heading) (lit (")")) | Universe.UMax _ => cat (heading) (lit (")")) | Universe.UIMax _ _ => cat (heading) (lit (")"))))

mutual
@[expose] public def showExpr : (value : Lse) -> (rename : Bool) -> (scope : List ((Prod (ByteArray) (ByteArray)))) -> (counter : Nat) -> (Prod (ByteArray) (Nat))
  | Lse.LSortProp, _rename, _scope, counter => (lit ("(sort prop)"), counter)
  | Lse.LSortType level, _rename, _scope, counter => (cat (cat (lit ("(sort (type ")) (universeText (level))) (lit ("))")), counter)
  | Lse.LConst package entry universes, _rename, _scope, counter => (constText (package) (entry) (universes), counter)
  | Lse.LLocal name, rename, scope, counter => (cat (cat (lit ("(local ")) ((if rename then lookup (scope) (name) else name))) (lit (")")), counter)
  | Lse.LApp function arguments, rename, scope, counter => (let shownFunction : (Prod (ByteArray) (Nat)) := showExpr (function) (rename) (scope) (counter); (let shownArguments : (Prod (ByteArray) (Nat)) := showArgs (arguments) (rename) (scope) ((shownFunction).2); (cat (cat (cat (lit ("(app ")) ((shownFunction).1)) ((shownArguments).1)) (lit (")")), (shownArguments).2)))
  | Lse.LPi binders body, rename, scope, counter => (let shownBinders : (Prod ((Prod (ByteArray) (List ((Prod (ByteArray) (ByteArray)))))) (Nat)) := showBinders (binders) (rename) (scope) (counter); (let shownBody : (Prod (ByteArray) (Nat)) := showExpr (body) (rename) (((shownBinders).1).2) ((shownBinders).2); (cat (cat (cat (cat (lit ("(pi (")) (((shownBinders).1).1)) (lit (") "))) ((shownBody).1)) (lit (")")), (shownBody).2)))
  | Lse.LLam binders body, rename, scope, counter => (let shownBinders : (Prod ((Prod (ByteArray) (List ((Prod (ByteArray) (ByteArray)))))) (Nat)) := showBinders (binders) (rename) (scope) (counter); (let shownBody : (Prod (ByteArray) (Nat)) := showExpr (body) (rename) (((shownBinders).1).2) ((shownBinders).2); (cat (cat (cat (cat (lit ("(lam (")) (((shownBinders).1).1)) (lit (") "))) ((shownBody).1)) (lit (")")), (shownBody).2)))
  | Lse.LLet name ty bound body, rename, scope, counter => (let shownName : (Prod (ByteArray) (Nat)) := fresh (name) (rename) (counter); (let inner : List ((Prod (ByteArray) (ByteArray))) := ((name, (shownName).1) :: scope); (let shownType : (Prod (ByteArray) (Nat)) := showExpr (ty) (rename) (inner) ((shownName).2); (let shownValue : (Prod (ByteArray) (Nat)) := showExpr (bound) (rename) (inner) ((shownType).2); (let shownBody : (Prod (ByteArray) (Nat)) := showExpr (body) (rename) (inner) ((shownValue).2); (cat (cat (cat (cat (cat (cat (cat (lit ("(let ")) ((shownName).1)) (lit (" "))) ((shownType).1)) (lit (" "))) ((shownValue).1)) (lit (" "))) (cat ((shownBody).1) (lit (")"))), (shownBody).2))))))
  | Lse.LNat digits, _rename, _scope, counter => (cat (cat (lit ("(nat ")) (digits)) (lit (")")), counter)
  | Lse.LNil, _rename, _scope, counter => (_root_.ByteArray.mk #[], counter)
  | Lse.LArg _ _, _rename, _scope, counter => (_root_.ByteArray.mk #[], counter)
  | Lse.LBinder _ _ _ _, _rename, _scope, counter => (_root_.ByteArray.mk #[], counter)
termination_by structural value _ _ _ => value

@[expose] public def showArgs : (arguments : Lse) -> (rename : Bool) -> (scope : List ((Prod (ByteArray) (ByteArray)))) -> (counter : Nat) -> (Prod (ByteArray) (Nat))
  | Lse.LArg head tail, rename, scope, counter => (let shownHead : (Prod (ByteArray) (Nat)) := showExpr (head) (rename) (scope) (counter); (let shownTail : (Prod (ByteArray) (Nat)) := showArgs (tail) (rename) (scope) ((shownHead).2); (cat (cat (lit (" ")) ((shownHead).1)) ((shownTail).1), (shownTail).2)))
  | Lse.LNil, _rename, _scope, counter => (_root_.ByteArray.mk #[], counter)
  | Lse.LSortProp, _rename, _scope, counter => (_root_.ByteArray.mk #[], counter)
  | Lse.LSortType _, _rename, _scope, counter => (_root_.ByteArray.mk #[], counter)
  | Lse.LConst _ _ _, _rename, _scope, counter => (_root_.ByteArray.mk #[], counter)
  | Lse.LLocal _, _rename, _scope, counter => (_root_.ByteArray.mk #[], counter)
  | Lse.LApp _ _, _rename, _scope, counter => (_root_.ByteArray.mk #[], counter)
  | Lse.LPi _ _, _rename, _scope, counter => (_root_.ByteArray.mk #[], counter)
  | Lse.LLam _ _, _rename, _scope, counter => (_root_.ByteArray.mk #[], counter)
  | Lse.LLet _ _ _ _, _rename, _scope, counter => (_root_.ByteArray.mk #[], counter)
  | Lse.LNat _, _rename, _scope, counter => (_root_.ByteArray.mk #[], counter)
  | Lse.LBinder _ _ _ _, _rename, _scope, counter => (_root_.ByteArray.mk #[], counter)
termination_by structural arguments _ _ _ => arguments

@[expose] public def showBinders : (binders : Lse) -> (rename : Bool) -> (scope : List ((Prod (ByteArray) (ByteArray)))) -> (counter : Nat) -> (Prod ((Prod (ByteArray) (List ((Prod (ByteArray) (ByteArray)))))) (Nat))
  | Lse.LBinder mode name ty rest, rename, scope, counter => (let shownName : (Prod (ByteArray) (Nat)) := fresh (name) (rename) (counter); (let shownType : (Prod (ByteArray) (Nat)) := showExpr (ty) (rename) (scope) ((shownName).2); (let piece : ByteArray := cat (cat (cat (cat (cat (cat (lit ("(")) (modeText (mode))) (lit (" "))) ((shownName).1)) (lit (" "))) ((shownType).1)) (lit (")")); (let inner : List ((Prod (ByteArray) (ByteArray))) := ((name, (shownName).1) :: scope); (match rest with | Lse.LNil => ((piece, inner), (shownType).2) | Lse.LBinder _ _ _ _ => (let shownRest : (Prod ((Prod (ByteArray) (List ((Prod (ByteArray) (ByteArray)))))) (Nat)) := showBinders (rest) (rename) (inner) ((shownType).2); ((cat (cat (piece) (lit (" "))) (((shownRest).1).1), ((shownRest).1).2), (shownRest).2)) | Lse.LSortProp => ((piece, inner), (shownType).2) | Lse.LSortType _ => ((piece, inner), (shownType).2) | Lse.LConst _ _ _ => ((piece, inner), (shownType).2) | Lse.LLocal _ => ((piece, inner), (shownType).2) | Lse.LApp _ _ => ((piece, inner), (shownType).2) | Lse.LPi _ _ => ((piece, inner), (shownType).2) | Lse.LLam _ _ => ((piece, inner), (shownType).2) | Lse.LLet _ _ _ _ => ((piece, inner), (shownType).2) | Lse.LNat _ => ((piece, inner), (shownType).2) | Lse.LArg _ _ => ((piece, inner), (shownType).2))))))
  | Lse.LNil, _rename, scope, counter => ((_root_.ByteArray.mk #[], scope), counter)
  | Lse.LSortProp, _rename, scope, counter => ((_root_.ByteArray.mk #[], scope), counter)
  | Lse.LSortType _, _rename, scope, counter => ((_root_.ByteArray.mk #[], scope), counter)
  | Lse.LConst _ _ _, _rename, scope, counter => ((_root_.ByteArray.mk #[], scope), counter)
  | Lse.LLocal _, _rename, scope, counter => ((_root_.ByteArray.mk #[], scope), counter)
  | Lse.LApp _ _, _rename, scope, counter => ((_root_.ByteArray.mk #[], scope), counter)
  | Lse.LPi _ _, _rename, scope, counter => ((_root_.ByteArray.mk #[], scope), counter)
  | Lse.LLam _ _, _rename, scope, counter => ((_root_.ByteArray.mk #[], scope), counter)
  | Lse.LLet _ _ _ _, _rename, scope, counter => ((_root_.ByteArray.mk #[], scope), counter)
  | Lse.LNat _, _rename, scope, counter => ((_root_.ByteArray.mk #[], scope), counter)
  | Lse.LArg _ _, _rename, scope, counter => ((_root_.ByteArray.mk #[], scope), counter)
termination_by structural binders _ _ _ => binders
end

@[expose] public def printLse (value : Lse) (rename : Bool) : ByteArray := (showExpr (value) (rename) (([] : List ((Prod (ByteArray) (ByteArray))))) (0)).1

end LexLeanBoot.LexiconLse
