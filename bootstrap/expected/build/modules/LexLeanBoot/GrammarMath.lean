module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Numeral
public import LexLeanStd.Text
public import LexLeanBoot.PrimitiveScan
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.GrammarMath

public inductive LeafKind where
  | LForm (_ : Nat)
  | LIdent (_ : ByteArray)

public inductive Math where
  | MNumeral (_ : ByteArray) (_ : Nat) (_ : Nat)
  | MLeaf (_ : List (LeafKind)) (_ : Nat) (_ : Nat)
  | MLexeme (_ : ByteArray) (_ : Nat) (_ : Nat)
  | MReference (_ : ByteArray) (_ : ByteArray) (_ : Nat) (_ : Nat)
  | MCall (_ : Math) (_ : Math) (_ : Nat) (_ : Nat) (_ : Nat) (_ : Nat) (_ : List (Nat))
  | MPrefix (_ : List (Nat)) (_ : Nat) (_ : Nat) (_ : Math)
  | MPostfix (_ : List (Nat)) (_ : Nat) (_ : Nat) (_ : Math)
  | MInfix (_ : List (Nat)) (_ : Nat) (_ : Nat) (_ : Math) (_ : Math)
  | MParen (_ : Math) (_ : Nat) (_ : Nat)
  | MArgNil
  | MArgCons (_ : Math) (_ : Math)

public structure Edge where
  form : Nat
  stop : Nat
  frame : Nat
  precedence : Nat
  associativity : Nat

public structure Val where
  ast : Math
  startsAt : Nat
  endsAt : Nat
  code : ByteArray

public structure Choice where
  position : Nat
  upto : Nat

public structure Met where
  position : Nat
  ends : List (Nat)

public structure Bud where
  states : Nat
  edges : Nat
  memo : List (Nat)

public inductive Frame where
  | FExpr (_ : Nat) (_ : Nat)
  | FLoop (_ : Nat) (_ : Nat) (_ : Option (Nat))
  | FInfix (_ : Nat) (_ : Nat) (_ : Val) (_ : List (Nat)) (_ : Nat) (_ : Nat) (_ : Option (Nat))
  | FPrimary (_ : Nat)
  | FParen (_ : Nat) (_ : Nat)
  | FPrefix (_ : List (Nat)) (_ : Nat) (_ : Nat) (_ : Nat)
  | FPostfix (_ : Nat)
  | FCall (_ : Val) (_ : Nat) (_ : List (Nat)) (_ : List (Val)) (_ : Nat)

public structure Ctx where
  path : ByteArray
  atoms : List (LexLeanBoot.PrimitiveScan.Atom)
  stop : Nat
  edges : List (List (Edge))
  maxStates : Nat
  maxDepth : Nat
  maxEdges : Nat
  chosen : List (Choice)

public structure Mach where
  cursor : Nat
  stack : List (Frame)
  value : Option (Val)
  error : Option (LexLeanBoot.PrimitiveScan.Rejection)
  bud : Bud
  met : List (Met)

public structure Looked where
  bud : Bud
  found : List (Edge)
  error : Option (LexLeanBoot.PrimitiveScan.Rejection)

public structure Braced where
  cursor : Nat
  spelled : ByteArray
  closed : Bool

public structure Composed where
  spelled : ByteArray
  next : Nat
  reach : Nat

public structure Pass where
  result : Except (LexLeanBoot.PrimitiveScan.Rejection) (Val)
  bud : Bud
  met : List (Met)

public structure Outer where
  queue : List (List (Choice))
  enqueued : List (ByteArray)
  covers : List (Val)
  failure : Option (LexLeanBoot.PrimitiveScan.Rejection)
  bud : Bud

@[expose] public def lit (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def cat (left : ByteArray) (right : ByteArray) : ByteArray := (LexLeanRuntime.append (left) (right) : ByteArray)

@[expose] public def both (left : Bool) (right : Bool) : Bool := (if left then right else false)

@[expose] public def same (left : ByteArray) (right : ByteArray) : Bool := (LexLeanRuntime.equal (left) (right) : Bool)

@[expose] public def num (number : Nat) : ByteArray := LexLeanStd.Numeral.decimal (number)

@[expose] public def encN (number : Nat) : ByteArray := cat (num (number)) (lit (","))

@[expose] public def encB (piece : ByteArray) : ByteArray := cat (cat (num ((LexLeanRuntime.length (piece) : Nat))) (lit (":"))) (piece)

@[expose] public def encNs (items : List (Nat)) : ByteArray := (LexLeanCollections.listFold ((fun (out : ByteArray) (item : Nat) => cat (out) (encN (item)))) (encN ((LexLeanRuntime.length (items) : Nat))) (items) : ByteArray)

@[expose] public def reverseNats (items : List (Nat)) : List (Nat) := (LexLeanCollections.listFold ((fun (turned : List (Nat)) (item : Nat) => (item :: turned))) (([] : List (Nat))) (items) : List (Nat))

@[expose] public def reverseVals (items : List (Val)) : List (Val) := (LexLeanCollections.listFold ((fun (turned : List (Val)) (item : Val) => (item :: turned))) (([] : List (Val))) (items) : List (Val))

@[expose] public def reverseMets (items : List (Met)) : List (Met) := (LexLeanCollections.listFold ((fun (turned : List (Met)) (item : Met) => (item :: turned))) (([] : List (Met))) (items) : List (Met))

@[expose] public def natIn (needle : Nat) (items : List (Nat)) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (item : Nat) => (if found then true else (Nat.beq (item) (needle))))) (false) (items) : Bool)

@[expose] public def bytesIn (needle : ByteArray) (items : List (ByteArray)) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (item : ByteArray) => (if found then true else same (item) (needle)))) (false) (items) : Bool)

@[expose] public def insertNat (sorted : List (Nat)) (number : Nat) : List (Nat) := (let folded : (Prod (Bool) (List (Nat))) := (LexLeanCollections.listFold ((fun (state : (Prod (Bool) (List (Nat)))) (item : Nat) => (if (state).1 then (true, (item :: (state).2)) else (if (Nat.blt (number) (item)) then (true, (item :: (number :: (state).2))) else (if (Nat.beq (number) (item)) then (true, (item :: (state).2)) else (false, (item :: (state).2))))))) ((false, ([] : List (Nat)))) (sorted) : (Prod (Bool) (List (Nat)))); reverseNats ((if (folded).1 then (folded).2 else (number :: (folded).2))))

@[expose] public def atomAt (ctx : Ctx) (position : Nat) : Option (LexLeanBoot.PrimitiveScan.Atom) := (LexLeanRuntime.index ((ctx).atoms) (position) : Option (LexLeanBoot.PrimitiveScan.Atom))

@[expose] public def atomOr (ctx : Ctx) (position : Nat) : LexLeanBoot.PrimitiveScan.Atom := (match atomAt (ctx) (position) with | Option.some atom => atom | Option.none => ({ kind := LexLeanBoot.PrimitiveScan.AtomClass.Whitespace, byteStart := 0, byteEnd := 0, lineStart := 1, columnStart := 1, lineEnd := 1, columnEnd := 1, text := lit ("") } : LexLeanBoot.PrimitiveScan.Atom))

@[expose] public def classAt (ctx : Ctx) (position : Nat) : Nat := (match atomAt (ctx) (position) with | Option.some atom => LexLeanBoot.PrimitiveScan.classCode (atom) | Option.none => 7)

@[expose] public def isAtom (atom : LexLeanBoot.PrimitiveScan.Atom) (code : Nat) (spelled : ByteArray) : Bool := both ((Nat.beq (LexLeanBoot.PrimitiveScan.classCode (atom)) (code))) (same ((atom).text) (spelled))

@[expose] public def wsStep (ctx : Ctx) (position : Nat) : Option (Nat) := (if (Nat.blt (position) ((ctx).stop)) then (if (Nat.beq (classAt (ctx) (position)) (6)) then Option.some ((position + 1)) else Option.none) else Option.none)

@[expose] public def skipWs (ctx : Ctx) (position : Nat) : Nat := ((LexLeanCollections.iterateUntil ((fun (current : Nat) => wsStep (ctx) (current))) (((LexLeanRuntime.length ((ctx).atoms) : Nat) + 1)) (position) : (Prod (Nat) (Bool)))).1

@[expose] public def placeAt (ctx : Ctx) (position : Nat) : LexLeanBoot.PrimitiveScan.Place := LexLeanBoot.PrimitiveScan.placeOfAtom ((ctx).path) (atomOr (ctx) (position))

@[expose] public def spanHere (ctx : Ctx) (position : Nat) : LexLeanBoot.PrimitiveScan.Place := (let here : Nat := skipWs (ctx) (position); (if (Nat.blt (here) ((ctx).stop)) then placeAt (ctx) (here) else placeAt (ctx) ((LexLeanRuntime.subtract ((ctx).stop) (1) : Nat))))

@[expose] public def refusal (code : ByteArray) (message : ByteArray) (place : Option (LexLeanBoot.PrimitiveScan.Place)) : LexLeanBoot.PrimitiveScan.Rejection := ({ code := code, message := message, place := place, help := Option.none } : LexLeanBoot.PrimitiveScan.Rejection)

@[expose] public def placed (code : ByteArray) (message : ByteArray) (place : LexLeanBoot.PrimitiveScan.Place) : LexLeanBoot.PrimitiveScan.Rejection := refusal (code) (message) (Option.some (place))

@[expose] public def chargeState (ctx : Ctx) (bud : Bud) : (Prod (Bud) (Option (LexLeanBoot.PrimitiveScan.Rejection))) := (let states : Nat := ((bud).states + 1); (let after : Bud := ({ states := states, edges := (bud).edges, memo := (bud).memo } : Bud); (if (Nat.blt ((ctx).maxStates) (states)) then (after, Option.some (refusal (lit ("LLS8002")) (cat (cat (cat (cat (lit ("max_parse_states exceeded in phase parse: configured ")) (num ((ctx).maxStates))) (lit (", observed "))) (num (states))) (lit (" states"))) (Option.none))) else (after, Option.none))))

@[expose] public def edgesAt (ctx : Ctx) (bud : Bud) (position : Nat) : Looked := (let found : List (Edge) := (match (LexLeanRuntime.index ((ctx).edges) (position) : Option (List (Edge))) with | Option.some listed => listed | Option.none => ([] : List (Edge))); (if natIn (position) ((bud).memo) then ({ bud := bud, found := found, error := Option.none } : Looked) else (let count : Nat := ((bud).edges + (LexLeanRuntime.length (found) : Nat)); (if (Nat.blt ((ctx).maxEdges) (count)) then ({ bud := ({ states := (bud).states, edges := count, memo := (bud).memo } : Bud), found := ([] : List (Edge)), error := Option.some (refusal (lit ("LLS8002")) (cat (cat (cat (cat (lit ("max_token_lattice_edges exceeded in phase lexical resolution: configured ")) (num ((ctx).maxEdges))) (lit (", observed "))) (num (count))) (lit (" distinct lattice edges"))) (Option.none)) } : Looked) else ({ bud := ({ states := (bud).states, edges := count, memo := (position :: (bud).memo) } : Bud), found := found, error := Option.none } : Looked)))))

@[expose] public def mathVal (ast : Math) (startsAt : Nat) (endsAt : Nat) (code : ByteArray) : Val := ({ ast := ast, startsAt := startsAt, endsAt := endsAt, code := code } : Val)

@[expose] public def kindCode (kinds : List (LeafKind)) : ByteArray := (LexLeanCollections.listFold ((fun (out : ByteArray) (kind : LeafKind) => cat (out) ((match kind with | LeafKind.LForm form => cat (lit ("F")) (encN (form)) | LeafKind.LIdent spelled => cat (lit ("I")) (encB (spelled)))))) (encN ((LexLeanRuntime.length (kinds) : Nat))) (kinds) : ByteArray)

@[expose] public def argChain (args : List (Val)) : Math := (LexLeanCollections.listFold ((fun (rest : Math) (arg : Val) => Math.MArgCons ((arg).ast) (rest))) (Math.MArgNil) (reverseVals (args)) : Math)

@[expose] public def callVal (head : Val) (args : List (Val)) (upto : Nat) (opening : Nat) (closing : Nat) (commas : List (Nat)) : Val := mathVal (Math.MCall ((head).ast) (argChain (args)) ((head).startsAt) (upto) (opening) (closing) (commas)) ((head).startsAt) (upto) (cat (cat (cat (cat (cat (lit ("C")) ((head).code)) ((LexLeanCollections.listFold ((fun (out : ByteArray) (arg : Val) => cat (out) ((arg).code))) (encN ((LexLeanRuntime.length (args) : Nat))) (args) : ByteArray))) (encN (upto))) (cat (encN (opening)) (encN (closing)))) (encNs (commas)))

@[expose] public def opVal (tag : ByteArray) (candidates : List (Nat)) (opStart : Nat) (opEnd : Nat) (arg : Val) (prefixed : Bool) : Val := (let code : ByteArray := cat (cat (cat (cat (tag) (encNs (candidates))) (encN (opStart))) (encN (opEnd))) ((arg).code); (if prefixed then mathVal (Math.MPrefix (candidates) (opStart) (opEnd) ((arg).ast)) (opStart) ((arg).endsAt) (code) else mathVal (Math.MPostfix (candidates) (opStart) (opEnd) ((arg).ast)) ((arg).startsAt) (opEnd) (code)))

@[expose] public def infixVal (candidates : List (Nat)) (opStart : Nat) (opEnd : Nat) (lhs : Val) (rhs : Val) : Val := mathVal (Math.MInfix (candidates) (opStart) (opEnd) ((lhs).ast) ((rhs).ast)) ((lhs).startsAt) ((rhs).endsAt) (cat (cat (cat (cat (cat (lit ("X")) (encNs (candidates))) (encN (opStart))) (encN (opEnd))) ((lhs).code)) ((rhs).code))

@[expose] public def mk (m : Mach) (cursor : Nat) (stack : List (Frame)) (value : Option (Val)) : Mach := ({ cursor := cursor, stack := stack, value := value, error := (m).error, bud := (m).bud, met := (m).met } : Mach)

@[expose] public def failed (m : Mach) (problem : LexLeanBoot.PrimitiveScan.Rejection) : Mach := ({ cursor := (m).cursor, stack := (m).stack, value := (m).value, error := Option.some (problem), bud := (m).bud, met := (m).met } : Mach)

@[expose] public def withBud (m : Mach) (bud : Bud) : Mach := ({ cursor := (m).cursor, stack := (m).stack, value := (m).value, error := (m).error, bud := bud, met := (m).met } : Mach)

@[expose] public def braceStep (ctx : Ctx) (state : Braced) : Option (Braced) := (if (state).closed then Option.none else (let position : Nat := (state).cursor; (if (Nat.blt (position) ((ctx).stop)) then (match atomAt (ctx) (position) with | Option.some atom => (if isAtom (atom) (3) (lit ("}")) then Option.some (({ cursor := (position + 1), spelled := (state).spelled, closed := true } : Braced)) else (if (Nat.beq (LexLeanBoot.PrimitiveScan.classCode (atom)) (6)) then Option.some (({ cursor := (position + 1), spelled := (state).spelled, closed := false } : Braced)) else Option.some (({ cursor := (position + 1), spelled := cat ((state).spelled) ((atom).text), closed := false } : Braced)))) | Option.none => Option.none) else Option.none)))

@[expose] public def braces (ctx : Ctx) (position : Nat) : Except (LexLeanBoot.PrimitiveScan.Rejection) (Braced) := (let here : Nat := skipWs (ctx) (position); (if both ((Nat.blt (here) ((ctx).stop))) (isAtom (atomOr (ctx) (here)) (3) (lit ("{"))) then (let final : Braced := ((LexLeanCollections.iterateUntil ((fun (state : Braced) => braceStep (ctx) (state))) (((LexLeanRuntime.length ((ctx).atoms) : Nat) + 1)) (({ cursor := (here + 1), spelled := lit (""), closed := false } : Braced)) : (Prod (Braced) (Bool)))).1; (if (final).closed then Except.ok (final) else Except.error (placed (lit ("LLP2003")) (lit ("unclosed `{` argument")) (spanHere (ctx) ((final).cursor))))) else Except.error (placed (lit ("LLP2003")) (lit ("expected `{`")) (spanHere (ctx) (here)))))

@[expose] public def composeStep (ctx : Ctx) (state : Composed) : Option (Composed) := (match atomAt (ctx) ((state).next) with | Option.some atom => (let code : Nat := LexLeanBoot.PrimitiveScan.classCode (atom); (if both ((Nat.beq ((atom).byteStart) ((state).reach))) ((if (if (Nat.beq (code) (1)) then true else (Nat.beq (code) (2))) then true else both ((Nat.beq (code) (4))) ((if same ((atom).text) (lit ("_")) then true else same ((atom).text) (lit ("'")))))) then Option.some (({ spelled := cat ((state).spelled) ((atom).text), next := ((state).next + 1), reach := (atom).byteEnd } : Composed)) else Option.none)) | Option.none => Option.none)

@[expose] public def composedAt (ctx : Ctx) (atom : LexLeanBoot.PrimitiveScan.Atom) (position : Nat) : Composed := ((LexLeanCollections.iterateUntil ((fun (state : Composed) => composeStep (ctx) (state))) (((LexLeanRuntime.length ((ctx).atoms) : Nat) + 1)) (({ spelled := (atom).text, next := (position + 1), reach := (atom).byteEnd } : Composed)) : (Prod (Composed) (Bool)))).1

@[expose] public def chosenAt (ctx : Ctx) (position : Nat) : Option (Nat) := (LexLeanCollections.listFold ((fun (found : Option (Nat)) (choice : Choice) => (if (Nat.beq ((choice).position) (position)) then Option.some ((choice).upto) else found))) (Option.none) ((ctx).chosen) : Option (Nat))

@[expose] public def leafStep (ctx : Ctx) (m : Mach) (rest : List (Frame)) (depth : Nat) (position : Nat) (atom : LexLeanBoot.PrimitiveScan.Atom) (leaves : List (Edge)) : Mach := (let stop : Nat := (ctx).stop; (let identifier : Option (Composed) := (if (Nat.beq (LexLeanBoot.PrimitiveScan.classCode (atom)) (1)) then (let composed : Composed := composedAt (ctx) (atom) (position); (if (Nat.ble ((composed).next) (stop)) then Option.some (composed) else Option.none)) else Option.none); (let leafEnds : List (Nat) := (LexLeanCollections.listFold ((fun (sorted : List (Nat)) (edge : Edge) => insertNat (sorted) ((edge).stop))) (([] : List (Nat))) (leaves) : List (Nat)); (let ends : List (Nat) := (match identifier with | Option.some composed => insertNat (leafEnds) ((composed).next) | Option.none => leafEnds); (match (LexLeanRuntime.index (ends) (0) : Option (Nat)) with | Option.none => failed (m) (placed (lit ("LLL1004")) (cat (cat (lit ("unknown atom `")) ((atom).text)) (lit ("` in a mathematical island"))) (placeAt (ctx) (position))) | Option.some shortest => (let upto : Nat := (match chosenAt (ctx) (position) with | Option.some picked => (if natIn (picked) (ends) then picked else shortest) | Option.none => shortest); (let met : List (Met) := (if (Nat.blt (1) ((LexLeanRuntime.length (ends) : Nat))) then (({ position := position, ends := ends } : Met) :: (m).met) else (m).met); (let forms : List (LeafKind) := (LexLeanCollections.listFold ((fun (kinds : List (LeafKind)) (edge : Edge) => (if (Nat.beq ((edge).stop) (upto)) then (LeafKind.LForm ((edge).form) :: kinds) else kinds))) (([] : List (LeafKind))) (leaves) : List (LeafKind)); (let kinds : List (LeafKind) := (LexLeanCollections.listFold ((fun (turned : List (LeafKind)) (kind : LeafKind) => (kind :: turned))) ((match identifier with | Option.some composed => (if (Nat.beq ((composed).next) (upto)) then (LeafKind.LIdent ((composed).spelled) :: ([] : List (LeafKind))) else ([] : List (LeafKind))) | Option.none => ([] : List (LeafKind)))) (forms) : List (LeafKind)); (let leaf : Val := mathVal (Math.MLeaf (kinds) (position) (upto)) (position) (upto) (cat (cat (cat (lit ("L")) (kindCode (kinds))) (encN (position))) (encN (upto))); ({ cursor := upto, stack := (Frame.FPostfix (depth) :: rest), value := Option.some (leaf), error := (m).error, bud := (m).bud, met := met } : Mach)))))))))))

@[expose] public def profileAgrees (head : Edge) (edges : List (Edge)) (withFrame : Bool) : Bool := (LexLeanCollections.listFold ((fun (fine : Bool) (edge : Edge) => (if fine then both ((Nat.beq ((edge).stop) ((head).stop))) (both ((Nat.beq ((edge).precedence) ((head).precedence))) ((if withFrame then both ((Nat.beq ((edge).associativity) ((head).associativity))) ((Nat.beq ((edge).frame) ((head).frame))) else true))) else false))) (true) (edges) : Bool)

@[expose] public def formsOf (edges : List (Edge)) : List (Nat) := reverseNats ((LexLeanCollections.listFold ((fun (out : List (Nat)) (edge : Edge) => ((edge).form :: out))) (([] : List (Nat))) (edges) : List (Nat)))

@[expose] public def keepEdges (edges : List (Edge)) (stop : Nat) (low : Nat) (high : Nat) : List (Edge) := (LexLeanCollections.listFold ((fun (out : List (Edge)) (edge : Edge) => (LexLeanRuntime.append (out) ((if both ((Nat.ble ((edge).stop) (stop))) (both ((Nat.ble (low) ((edge).frame))) ((Nat.ble ((edge).frame) (high)))) then (edge :: ([] : List (Edge))) else ([] : List (Edge)))) : List (Edge)))) (([] : List (Edge))) (edges) : List (Edge))

@[expose] public def formStep (ctx : Ctx) (m : Mach) (rest : List (Frame)) (depth : Nat) (position : Nat) (atom : LexLeanBoot.PrimitiveScan.Atom) : Mach := (let looked : Looked := edgesAt (ctx) ((m).bud) (position); (let charged : Mach := withBud (m) ((looked).bud); (match (looked).error with | Option.some problem => failed (charged) (problem) | Option.none => (let stop : Nat := (ctx).stop; (let prefixes : List (Edge) := keepEdges ((looked).found) (stop) (2) (2); (let leaves : List (Edge) := keepEdges ((looked).found) (stop) (0) (1); (match (LexLeanRuntime.index (prefixes) (0) : Option (Edge)) with | Option.some head => (if profileAgrees (head) (prefixes) (false) then (if (Nat.beq ((LexLeanRuntime.length (leaves) : Nat)) (0)) then mk (charged) ((head).stop) ((Frame.FExpr ((head).precedence) ((depth + 1)) :: (Frame.FPrefix (formsOf (prefixes)) (position) ((head).stop) (depth) :: rest))) (Option.none) else failed (charged) (placed (lit ("LLP2002")) (lit ("this surface is both a prefix operator and a leaf; LexLean does not guess")) (placeAt (ctx) (position)))) else failed (charged) (placed (lit ("LLP2002")) (lit ("prefix candidates disagree on their profile; LexLean does not guess")) (placeAt (ctx) (position)))) | Option.none => leafStep (ctx) (charged) (rest) (depth) (position) (atom) (leaves))))))))

@[expose] public def primaryStep (ctx : Ctx) (m : Mach) (rest : List (Frame)) (depth : Nat) : Mach := (let charge : (Prod (Bud) (Option (LexLeanBoot.PrimitiveScan.Rejection))) := chargeState (ctx) ((m).bud); (let counted : Mach := withBud (m) ((charge).1); (match (charge).2 with | Option.some problem => failed (counted) (problem) | Option.none => (if (Nat.blt ((ctx).maxDepth) (depth)) then failed (counted) (placed (lit ("LLS8002")) (cat (cat (cat (cat (lit ("max_scope_depth exceeded in phase parse (mathematical grouping): configured ")) (num ((ctx).maxDepth))) (lit (", nesting depth "))) (num (depth))) (lit (""))) (spanHere (ctx) ((m).cursor))) else (let position : Nat := skipWs (ctx) ((m).cursor); (if (Nat.blt (position) ((ctx).stop)) then (let atom : LexLeanBoot.PrimitiveScan.Atom := atomOr (ctx) (position); (let code : Nat := LexLeanBoot.PrimitiveScan.classCode (atom); (if (Nat.beq (code) (2)) then mk (counted) ((position + 1)) ((Frame.FPostfix (depth) :: rest)) (Option.some (mathVal (Math.MNumeral ((atom).text) (position) ((position + 1))) (position) ((position + 1)) (cat (cat (cat (lit ("N")) (encB ((atom).text))) (encN (position))) (encN ((position + 1)))))) else (if isAtom (atom) (3) (lit ("(")) then mk (counted) ((position + 1)) ((Frame.FExpr (0) ((depth + 1)) :: (Frame.FParen (position) (depth) :: rest))) (Option.none) else (if (if isAtom (atom) (3) (lit ("{")) then true else isAtom (atom) (3) (lit ("}"))) then failed (counted) (placed (lit ("LLP2004")) (lit ("braces do not group mathematical terms")) (placeAt (ctx) (position))) else (if isAtom (atom) (0) (lit ("\\lexeme")) then (match braces (ctx) ((position + 1)) with | Except.ok braced => mk (counted) ((braced).cursor) ((Frame.FPostfix (depth) :: rest)) (Option.some (mathVal (Math.MLexeme ((braced).spelled) (position) ((braced).cursor)) (position) ((braced).cursor) (cat (cat (cat (lit ("E")) (encB ((braced).spelled))) (encN (position))) (encN ((braced).cursor))))) | Except.error problem => failed (counted) (problem)) else (if isAtom (atom) (0) (lit ("\\reference")) then (match braces (ctx) ((position + 1)) with | Except.ok braced => (let spelled : ByteArray := (braced).spelled; (match LexLeanStd.Text.find (spelled) (lit ("::")) with | Option.some split => (let qualifier : ByteArray := LexLeanStd.Text.sliceOr (spelled) (0) (split); (let component : ByteArray := LexLeanStd.Text.sliceOr (spelled) ((split + 2)) ((LexLeanRuntime.subtract ((LexLeanRuntime.length (spelled) : Nat)) ((split + 2)) : Nat)); mk (counted) ((braced).cursor) ((Frame.FPostfix (depth) :: rest)) (Option.some (mathVal (Math.MReference (qualifier) (component) (position) ((braced).cursor)) (position) ((braced).cursor) (cat (cat (cat (cat (lit ("R")) (encB (qualifier))) (encB (component))) (encN (position))) (encN ((braced).cursor))))))) | Option.none => failed (counted) (placed (lit ("LLR3005")) (cat (cat (lit ("`")) (spelled)) (lit ("` is not a `Module::component` reference"))) (placeAt (ctx) (position))))) | Except.error problem => failed (counted) (problem)) else formStep (ctx) (mk (counted) (position) (rest) (Option.none)) (rest) (depth) (position) (atom)))))))) else failed (mk (counted) (position) (rest) (Option.none)) (placed (lit ("LLP2001")) (lit ("expected a mathematical expression")) (spanHere (ctx) (position)))))))))

@[expose] public def loopStep (ctx : Ctx) (m : Mach) (rest : List (Frame)) (minBp : Nat) (depth : Nat) (last : Option (Nat)) (lhs : Val) : Mach := (let position : Nat := skipWs (ctx) ((m).cursor); (let done : Mach := mk (m) (position) (rest) (Option.some (lhs)); (if (Nat.blt (position) ((ctx).stop)) then (let looked : Looked := edgesAt (ctx) ((m).bud) (position); (let charged : Mach := withBud (done) ((looked).bud); (match (looked).error with | Option.some problem => failed (charged) (problem) | Option.none => (let operators : List (Edge) := keepEdges ((looked).found) ((ctx).stop) (3) (4); (match (LexLeanRuntime.index (operators) (0) : Option (Edge)) with | Option.none => charged | Option.some head => (if profileAgrees (head) (operators) (true) then (let precedence : Nat := (head).precedence; (if (Nat.blt (precedence) (minBp)) then charged else (if (Nat.beq ((head).frame) (3)) then mk (charged) ((head).stop) ((Frame.FLoop (minBp) (depth) (last) :: rest)) (Option.some (opVal (lit ("P")) (formsOf (operators)) (position) ((head).stop) (lhs) (false))) else (if (match last with | Option.some held => (Nat.beq (held) (precedence)) | Option.none => false) then failed (charged) (placed (lit ("LLP2004")) (lit ("a nonassociative chain requires explicit parentheses")) (spanHere (ctx) (position))) else (let associativity : Nat := (head).associativity; (let next : Nat := (if (Nat.beq (associativity) (2)) then precedence else (precedence + 1)); (let remember : Option (Nat) := (if (if (Nat.beq (associativity) (1)) then true else (Nat.beq (associativity) (2))) then Option.none else Option.some (precedence)); mk (charged) ((head).stop) ((Frame.FExpr (next) ((depth + 1)) :: (Frame.FInfix (minBp) (depth) (lhs) (formsOf (operators)) (position) ((head).stop) (remember) :: rest))) (Option.none)))))))) else failed (charged) (placed (lit ("LLP2002")) (lit ("operator candidates at this surface disagree on precedence, associativity, or frame; LexLean does not guess")) (spanHere (ctx) (position))))))))) else done)))

@[expose] public def valueOr (m : Mach) : Val := (match (m).value with | Option.some held => held | Option.none => mathVal (Math.MArgNil) (0) (0) (lit ("")))

@[expose] public def runFrame (ctx : Ctx) (m : Mach) (rest : List (Frame)) (frame : Frame) : Mach := (let held : Val := valueOr (m); (match frame with | Frame.FExpr minBp depth => (let charge : (Prod (Bud) (Option (LexLeanBoot.PrimitiveScan.Rejection))) := chargeState (ctx) ((m).bud); (let counted : Mach := withBud (m) ((charge).1); (match (charge).2 with | Option.some problem => failed (counted) (problem) | Option.none => mk (counted) ((m).cursor) ((Frame.FPrimary (depth) :: (Frame.FLoop (minBp) (depth) (Option.none) :: rest))) (Option.none)))) | Frame.FPrimary depth => primaryStep (ctx) (m) (rest) (depth) | Frame.FLoop minBp depth last => loopStep (ctx) (m) (rest) (minBp) (depth) (last) (held) | Frame.FInfix minBp depth lhs candidates opStart opEnd remember => mk (m) ((m).cursor) ((Frame.FLoop (minBp) (depth) (remember) :: rest)) (Option.some (infixVal (candidates) (opStart) (opEnd) (lhs) (held))) | Frame.FParen start depth => (let position : Nat := skipWs (ctx) ((m).cursor); (if both ((Nat.blt (position) ((ctx).stop))) (isAtom (atomOr (ctx) (position)) (3) (lit (")"))) then mk (m) ((position + 1)) ((Frame.FPostfix (depth) :: rest)) (Option.some (mathVal (Math.MParen ((held).ast) (start) ((position + 1))) (start) ((position + 1)) (cat (cat (cat (lit ("G")) ((held).code)) (encN (start))) (encN ((position + 1)))))) else failed (mk (m) (position) (rest) (Option.none)) (placed (lit ("LLP2004")) (lit ("unclosed grouping parenthesis")) (spanHere (ctx) (position))))) | Frame.FPrefix candidates opStart opEnd depth => mk (m) ((m).cursor) ((Frame.FPostfix (depth) :: rest)) (Option.some (opVal (lit ("Q")) (candidates) (opStart) (opEnd) (held) (true))) | Frame.FPostfix depth => (let position : Nat := skipWs (ctx) ((m).cursor); (if both ((Nat.blt (position) ((ctx).stop))) (isAtom (atomOr (ctx) (position)) (3) (lit ("("))) then mk (m) ((position + 1)) ((Frame.FExpr (0) ((depth + 1)) :: (Frame.FCall (held) (position) (([] : List (Nat))) (([] : List (Val))) (depth) :: rest))) (Option.none) else mk (m) (position) (rest) (Option.some (held)))) | Frame.FCall head opening commas args depth => (let gathered : List (Val) := (held :: args); (let position : Nat := skipWs (ctx) ((m).cursor); (let here : LexLeanBoot.PrimitiveScan.Atom := atomOr (ctx) (position); (if both ((Nat.blt (position) ((ctx).stop))) (isAtom (here) (4) (lit (","))) then mk (m) ((position + 1)) ((Frame.FExpr (0) ((depth + 1)) :: (Frame.FCall (head) (opening) ((position :: commas)) (gathered) (depth) :: rest))) (Option.none) else (if both ((Nat.blt (position) ((ctx).stop))) (isAtom (here) (3) (lit (")"))) then mk (m) ((position + 1)) ((Frame.FPostfix (depth) :: rest)) (Option.some (callVal (head) (reverseVals (gathered)) ((position + 1)) (opening) (position) (reverseNats (commas)))) else failed (mk (m) (position) (rest) (Option.none)) (placed (lit ("LLP2004")) (lit ("expected `,` or `)` in a call argument list")) (spanHere (ctx) (position))))))))))

@[expose] public def machStep (ctx : Ctx) (m : Mach) : Option (Mach) := (match (m).error with | Option.some _ => Option.none | Option.none => (match (m).stack with | List.nil => Option.none | List.cons frame rest => Option.some (runFrame (ctx) (m) (rest) (frame))))

@[expose] public def cover (ctx : Ctx) (start : Nat) (bud : Bud) : Pass := (let final : Mach := ((LexLeanCollections.iterateUntil ((fun (m : Mach) => machStep (ctx) (m))) ((LexLeanRuntime.multiply (((LexLeanRuntime.subtract ((ctx).stop) (start) : Nat) + 4)) (16) : Nat)) (({ cursor := start, stack := (Frame.FExpr (0) (1) :: ([] : List (Frame))), value := Option.none, error := Option.none, bud := bud, met := ([] : List (Met)) } : Mach)) : (Prod (Mach) (Bool)))).1; (let met : List (Met) := reverseMets ((final).met); (match (final).error with | Option.some problem => ({ result := Except.error (problem), bud := (final).bud, met := met } : Pass) | Option.none => (let position : Nat := skipWs (ctx) ((final).cursor); (if (Nat.blt (position) ((ctx).stop)) then (let atom : LexLeanBoot.PrimitiveScan.Atom := atomOr (ctx) (position); (let code : Nat := LexLeanBoot.PrimitiveScan.classCode (atom); (let quick : Bool := (if (if (Nat.beq (code) (1)) then true else (if (Nat.beq (code) (2)) then true else (if (Nat.beq (code) (3)) then true else (Nat.beq (code) (6))))) then true else same ((atom).text) (lit (","))); (let looked : Looked := (if quick then ({ bud := (final).bud, found := ([] : List (Edge)), error := Option.none } : Looked) else edgesAt (ctx) ((final).bud) (position)); (let unexpected : LexLeanBoot.PrimitiveScan.Rejection := placed (lit ("LLP2004")) (cat (cat (lit ("unexpected `")) ((atom).text)) (lit ("` after the expression"))) (placeAt (ctx) (position)); (match (looked).error with | Option.some problem => ({ result := Except.error (problem), bud := (looked).bud, met := met } : Pass) | Option.none => ({ result := Except.error ((if (if quick then true else (Nat.blt (0) ((LexLeanRuntime.length ((looked).found) : Nat)))) then unexpected else placed (lit ("LLL1004")) (cat (cat (lit ("unknown atom `")) ((atom).text)) (lit ("` in a mathematical island"))) (placeAt (ctx) (position)))), bud := (looked).bud, met := met } : Pass))))))) else ({ result := Except.ok (valueOr (final)), bud := (final).bud, met := met } : Pass))))))

@[expose] public def choiceKey (chosen : List (Choice)) : ByteArray := (LexLeanCollections.listFold ((fun (out : ByteArray) (choice : Choice) => cat (cat (out) (encN ((choice).position))) (encN ((choice).upto)))) (lit ("")) (chosen) : ByteArray)

@[expose] public def insertChoice (chosen : List (Choice)) (position : Nat) (upto : Nat) : List (Choice) := (let folded : (Prod (Bool) (List (Choice))) := (LexLeanCollections.listFold ((fun (state : (Prod (Bool) (List (Choice)))) (choice : Choice) => (if (state).1 then (true, (choice :: (state).2)) else (if (Nat.blt (position) ((choice).position)) then (true, (choice :: (({ position := position, upto := upto } : Choice) :: (state).2))) else (false, (choice :: (state).2)))))) ((false, ([] : List (Choice)))) (chosen) : (Prod (Bool) (List (Choice)))); (LexLeanCollections.listFold ((fun (turned : List (Choice)) (choice : Choice) => (choice :: turned))) (([] : List (Choice))) ((if (folded).1 then (folded).2 else (({ position := position, upto := upto } : Choice) :: (folded).2))) : List (Choice)))

@[expose] public def choicePresent (chosen : List (Choice)) (position : Nat) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (choice : Choice) => (if found then true else (Nat.beq ((choice).position) (position))))) (false) (chosen) : Bool)

@[expose] public def enqueue (chosen : List (Choice)) (met : List (Met)) (queue : List (List (Choice))) (enqueued : List (ByteArray)) : (Prod (List (List (Choice))) (List (ByteArray))) := (LexLeanCollections.listFold ((fun (state : (Prod (List (List (Choice))) (List (ByteArray)))) (point : Met) => (if choicePresent (chosen) ((point).position) then state else ((LexLeanCollections.listFold ((fun (inner : (Prod (Bool) ((Prod (List (List (Choice))) (List (ByteArray)))))) (upto : Nat) => (if (inner).1 then (false, (inner).2) else (let next : List (Choice) := insertChoice (chosen) ((point).position) (upto); (let key : ByteArray := choiceKey (next); (if bytesIn (key) (((inner).2).2) then (false, (inner).2) else (false, ((next :: ((inner).2).1), (key :: ((inner).2).2))))))))) ((true, state)) ((point).ends) : (Prod (Bool) ((Prod (List (List (Choice))) (List (ByteArray))))))).2))) ((queue, enqueued)) (met) : (Prod (List (List (Choice))) (List (ByteArray))))

@[expose] public def outerStep (ctx : Ctx) (start : Nat) (state : Outer) : Option (Outer) := (match (state).queue with | List.nil => Option.none | List.cons chosen rest => (let pass : Pass := cover (({ path := (ctx).path, atoms := (ctx).atoms, stop := (ctx).stop, edges := (ctx).edges, maxStates := (ctx).maxStates, maxDepth := (ctx).maxDepth, maxEdges := (ctx).maxEdges, chosen := chosen } : Ctx)) (start) ((state).bud); (let grown : (Prod (List (List (Choice))) (List (ByteArray))) := enqueue (chosen) ((pass).met) (rest) ((state).enqueued); Option.some ((match (pass).result with | Except.ok found => ({ queue := (grown).1, enqueued := (grown).2, covers := (if (LexLeanCollections.listFold ((fun (seen : Bool) (held : Val) => (if seen then true else same ((held).code) ((found).code)))) (false) ((state).covers) : Bool) then (state).covers else (LexLeanRuntime.append ((state).covers) ((found :: ([] : List (Val)))) : List (Val))), failure := (state).failure, bud := (pass).bud } : Outer) | Except.error problem => ({ queue := (grown).1, enqueued := (grown).2, covers := (state).covers, failure := (match (state).failure with | Option.some _ => (state).failure | Option.none => Option.some (problem)), bud := (pass).bud } : Outer))))))

@[expose] public def parseMath (path : ByteArray) (atoms : List (LexLeanBoot.PrimitiveScan.Atom)) (start : Nat) (stop : Nat) (edges : List (List (Edge))) (maxEdges : Nat) (maxStates : Nat) (maxDepth : Nat) (statesBefore : Nat) (edgesBefore : Nat) : Except (LexLeanBoot.PrimitiveScan.Rejection) (List (Math)) := (let ctx : Ctx := ({ path := path, atoms := atoms, stop := stop, edges := edges, maxStates := maxStates, maxDepth := maxDepth, maxEdges := maxEdges, chosen := ([] : List (Choice)) } : Ctx); (let final : Outer := ((LexLeanCollections.iterateUntil ((fun (state : Outer) => outerStep (ctx) (start) (state))) (((LexLeanRuntime.multiply ((maxStates + 2)) (((LexLeanRuntime.subtract (stop) (start) : Nat) + 2)) : Nat) + 2)) (({ queue := (([] : List (Choice)) :: ([] : List (List (Choice)))), enqueued := (lit ("") :: ([] : List (ByteArray))), covers := ([] : List (Val)), failure := Option.none, bud := ({ states := statesBefore, edges := edgesBefore, memo := ([] : List (Nat)) } : Bud) } : Outer)) : (Prod (Outer) (Bool)))).1; (if (Nat.beq ((LexLeanRuntime.length ((final).covers) : Nat)) (0)) then (match (final).failure with | Option.some problem => Except.error (problem) | Option.none => Except.error (placed (lit ("LLI9001")) (lit ("phase parse: the token lattice produced neither a cover nor a failure")) (({ path := path, byteStart := 0, byteEnd := 0, lineStart := 1, columnStart := 1, lineEnd := 1, columnEnd := 1 } : LexLeanBoot.PrimitiveScan.Place)))) else Except.ok ((LexLeanCollections.listFold ((fun (out : List (Math)) (held : Val) => (LexLeanRuntime.append (out) (((held).ast :: ([] : List (Math)))) : List (Math)))) (([] : List (Math))) ((final).covers) : List (Math))))))

end LexLeanBoot.GrammarMath
