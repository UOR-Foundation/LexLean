module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanBoot.GrammarProof
public import LexLeanStd.Numeral
public import LexLeanBoot.PrimitiveScan
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.PropositionTerms

public structure TEdge where
  form : Nat
  stop : Nat
  category : Nat
  inflected : Bool

public inductive TypePhrase where
  | TNoun (_ : List (Nat)) (_ : Nat) (_ : Nat)
  | TMath (_ : LexLeanBoot.GrammarProof.TextToken)

public structure Binder where
  phrase : TypePhrase
  spelling : ByteArray
  island : LexLeanBoot.GrammarProof.TextToken
  identStart : Nat
  identEnd : Nat
  keywords : List (LexLeanBoot.GrammarProof.Keyword)

public structure BAlt where
  upto : Nat
  binder : Binder

public inductive Term where
  | TIsland (_ : LexLeanBoot.GrammarProof.TextToken)
  | TNounOf (_ : List (Nat)) (_ : Nat) (_ : Nat) (_ : Term) (_ : List (LexLeanBoot.GrammarProof.Keyword))
  | TArgNil
  | TArgCons (_ : Term) (_ : Term)

public structure TAlt where
  upto : Nat
  term : Term

public structure Bud where
  states : Nat
  edges : Nat
  memo : List (Nat)

public structure Ctx where
  path : ByteArray
  atoms : List (LexLeanBoot.PrimitiveScan.Atom)
  tokens : List (LexLeanBoot.GrammarProof.TextToken)
  edges : List (List (TEdge))
  maxStates : Nat
  maxDepth : Nat
  maxEdges : Nat

public structure Looked where
  bud : Bud
  found : List (TEdge)
  error : Option (LexLeanBoot.PrimitiveScan.Rejection)

public structure Match where
  form : Nat
  category : Nat
  inflected : Bool
  upto : Nat

public structure Matched where
  bud : Bud
  found : List (Match)
  error : Option (LexLeanBoot.PrimitiveScan.Rejection)

public inductive Frame where
  | XStart (_ : Nat) (_ : Nat)
  | XMatches (_ : Nat) (_ : Nat) (_ : LexLeanBoot.GrammarProof.Keyword) (_ : List (TAlt)) (_ : List (Match))
  | XAfterFirst (_ : Nat) (_ : Nat) (_ : LexLeanBoot.GrammarProof.Keyword) (_ : List (TAlt)) (_ : List (Match)) (_ : Match) (_ : LexLeanBoot.GrammarProof.Keyword)
  | XFirsts (_ : Nat) (_ : Nat) (_ : LexLeanBoot.GrammarProof.Keyword) (_ : List (TAlt)) (_ : List (Match)) (_ : Match) (_ : LexLeanBoot.GrammarProof.Keyword) (_ : List (TAlt))
  | XAfterSecond (_ : Nat) (_ : Nat) (_ : LexLeanBoot.GrammarProof.Keyword) (_ : List (TAlt)) (_ : List (Match)) (_ : Match) (_ : LexLeanBoot.GrammarProof.Keyword) (_ : List (TAlt)) (_ : TAlt) (_ : LexLeanBoot.GrammarProof.Keyword)

public structure Mach where
  stack : List (Frame)
  returned : List (TAlt)
  error : Option (LexLeanBoot.PrimitiveScan.Rejection)
  bud : Bud

public structure Scan where
  bud : Bud
  found : Option (LexLeanBoot.PrimitiveScan.Rejection)
  error : Option (LexLeanBoot.PrimitiveScan.Rejection)

@[expose] public def lit (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def cat (left : ByteArray) (right : ByteArray) : ByteArray := (LexLeanRuntime.append (left) (right) : ByteArray)

@[expose] public def both (left : Bool) (right : Bool) : Bool := (if left then right else false)

@[expose] public def same (left : ByteArray) (right : ByteArray) : Bool := (LexLeanRuntime.equal (left) (right) : Bool)

@[expose] public def num (number : Nat) : ByteArray := LexLeanStd.Numeral.decimal (number)

@[expose] public def natIn (needle : Nat) (items : List (Nat)) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (item : Nat) => (if found then true else (Nat.beq (item) (needle))))) (false) (items) : Bool)

@[expose] public def keywordEntry (word : ByteArray) : Option (ByteArray) := (if (if same (word) (lit ("a")) then true else same (word) (lit ("A"))) then Option.some (lit ("a")) else (if (if same (word) (lit ("an")) then true else same (word) (lit ("An"))) then Option.some (lit ("an")) else (if same (word) (lit ("and")) then Option.some (lit ("and")) else (if same (word) (lit ("as")) then Option.some (lit ("as")) else (if same (word) (lit ("defined")) then Option.some (lit ("defined")) else (if same (word) (lit ("every")) then Option.some (lit ("every")) else (if same (word) (lit ("exactly")) then Option.some (lit ("exactly")) else (if same (word) (lit ("exists")) then Option.some (lit ("exists")) else (if (if same (word) (lit ("for")) then true else same (word) (lit ("For"))) then Option.some (lit ("for")) else (if same (word) (lit ("holds")) then Option.some (lit ("holds")) else (if (if same (word) (lit ("if")) then true else same (word) (lit ("If"))) then Option.some (lit ("if")) else (if same (word) (lit ("implies")) then Option.some (lit ("implies")) else (if same (word) (lit ("is")) then Option.some (lit ("is")) else (if (if same (word) (lit ("not")) then true else same (word) (lit ("Not"))) then Option.some (lit ("not")) else (if same (word) (lit ("one")) then Option.some (lit ("one")) else (if same (word) (lit ("only")) then Option.some (lit ("only")) else (if same (word) (lit ("or")) then Option.some (lit ("or")) else (if same (word) (lit ("such")) then Option.some (lit ("such")) else (if same (word) (lit ("that")) then Option.some (lit ("that")) else (if (if same (word) (lit ("the")) then true else same (word) (lit ("The"))) then Option.some (lit ("the")) else (if same (word) (lit ("then")) then Option.some (lit ("then")) else (if (if same (word) (lit ("there")) then true else same (word) (lit ("There"))) then Option.some (lit ("there")) else (if same (word) (lit ("when")) then Option.some (lit ("when")) else (if same (word) (lit ("of")) then Option.some (lit ("of")) else Option.none))))))))))))))))))))))))

@[expose] public def proofKeyword (word : ByteArray) : Bool := LexLeanBoot.PrimitiveScan.bytesInList (word) ((lit ("Assume") :: (lit ("Apply") :: (lit ("Close") :: (lit ("Select") :: (lit ("Use") :: (lit ("alternative") :: (lit ("as") :: (lit ("by") :: (lit ("goal") :: (lit ("left") :: (lit ("reflexivity") :: (lit ("right") :: (lit ("the") :: (lit ("with") :: (lit ("witness") :: ([] : List (ByteArray))))))))))))))))))

@[expose] public def atomOr (ctx : Ctx) (position : Nat) : LexLeanBoot.PrimitiveScan.Atom := (match (LexLeanRuntime.index ((ctx).atoms) (position) : Option (LexLeanBoot.PrimitiveScan.Atom)) with | Option.some atom => atom | Option.none => ({ kind := LexLeanBoot.PrimitiveScan.AtomClass.Whitespace, byteStart := 0, byteEnd := 0, lineStart := 1, columnStart := 1, lineEnd := 1, columnEnd := 1, text := lit ("") } : LexLeanBoot.PrimitiveScan.Atom))

@[expose] public def tokenAt (ctx : Ctx) (position : Nat) : Option (LexLeanBoot.GrammarProof.TextToken) := (LexLeanRuntime.index ((ctx).tokens) (position) : Option (LexLeanBoot.GrammarProof.TextToken))

@[expose] public def lastAtom (token : LexLeanBoot.GrammarProof.TextToken) : Nat := (match token with | LexLeanBoot.GrammarProof.TextToken.TokenAtom which => which | LexLeanBoot.GrammarProof.TextToken.TokenIsland _ _ _ closing _ => closing)

@[expose] public def placeAt (ctx : Ctx) (position : Nat) : LexLeanBoot.PrimitiveScan.Place := LexLeanBoot.PrimitiveScan.placeOfAtom ((ctx).path) (atomOr (ctx) (position))

@[expose] public def placed (code : ByteArray) (message : ByteArray) (place : Option (LexLeanBoot.PrimitiveScan.Place)) : LexLeanBoot.PrimitiveScan.Rejection := ({ code := code, message := message, place := place, help := Option.none } : LexLeanBoot.PrimitiveScan.Rejection)

@[expose] public def wordAt (ctx : Ctx) (position : Nat) : Option (Nat) := (match tokenAt (ctx) (position) with | Option.some token => (match token with | LexLeanBoot.GrammarProof.TextToken.TokenAtom which => (if (Nat.beq (LexLeanBoot.PrimitiveScan.classCode (atomOr (ctx) (which))) (1)) then Option.some (which) else Option.none) | LexLeanBoot.GrammarProof.TextToken.TokenIsland _ _ _ _ _ => Option.none) | Option.none => Option.none)

@[expose] public def isWord (ctx : Ctx) (position : Nat) (word : ByteArray) : Option (LexLeanBoot.GrammarProof.Keyword) := (match wordAt (ctx) (position) with | Option.some which => (if same ((atomOr (ctx) (which)).text) (word) then (match keywordEntry (word) with | Option.some entry => Option.some (({ atom := which, entry := entry } : LexLeanBoot.GrammarProof.Keyword)) | Option.none => Option.none) else Option.none) | Option.none => Option.none)

@[expose] public def islandAt (ctx : Ctx) (position : Nat) : Option (LexLeanBoot.GrammarProof.TextToken) := (match tokenAt (ctx) (position) with | Option.some token => (match token with | LexLeanBoot.GrammarProof.TextToken.TokenAtom _ => Option.none | LexLeanBoot.GrammarProof.TextToken.TokenIsland _ _ _ _ _ => Option.some (token)) | Option.none => Option.none)

@[expose] public def chargeState (ctx : Ctx) (bud : Bud) : (Prod (Bud) (Option (LexLeanBoot.PrimitiveScan.Rejection))) := (let states : Nat := ((bud).states + 1); (let after : Bud := ({ states := states, edges := (bud).edges, memo := (bud).memo } : Bud); (if (Nat.blt ((ctx).maxStates) (states)) then (after, Option.some (placed (lit ("LLS8002")) (cat (cat (cat (cat (lit ("max_parse_states exceeded in phase parse: configured ")) (num ((ctx).maxStates))) (lit (", observed "))) (num (states))) (lit (" states"))) (Option.none))) else (after, Option.none))))

@[expose] public def depthRefusal (ctx : Ctx) (depth : Nat) (position : Nat) : Option (LexLeanBoot.PrimitiveScan.Rejection) := (if (Nat.blt ((ctx).maxDepth) (depth)) then Option.some (placed (lit ("LLS8002")) (cat (cat (cat (lit ("max_scope_depth exceeded in phase parse (proposition nesting): configured ")) (num ((ctx).maxDepth))) (lit (", nesting depth "))) (num (depth))) ((match tokenAt (ctx) (position) with | Option.some token => Option.some (placeAt (ctx) (LexLeanBoot.GrammarProof.firstAtom (token))) | Option.none => Option.none))) else Option.none)

@[expose] public def edgesAt (ctx : Ctx) (bud : Bud) (position : Nat) : Looked := (let found : List (TEdge) := (match (LexLeanRuntime.index ((ctx).edges) (position) : Option (List (TEdge))) with | Option.some listed => listed | Option.none => ([] : List (TEdge))); (if natIn (position) ((bud).memo) then ({ bud := bud, found := found, error := Option.none } : Looked) else (let count : Nat := ((bud).edges + (LexLeanRuntime.length (found) : Nat)); (if (Nat.blt ((ctx).maxEdges) (count)) then ({ bud := ({ states := (bud).states, edges := count, memo := (bud).memo } : Bud), found := ([] : List (TEdge)), error := Option.some (placed (lit ("LLS8002")) (cat (cat (cat (cat (lit ("max_token_lattice_edges exceeded in phase lexical resolution: configured ")) (num ((ctx).maxEdges))) (lit (", observed "))) (num (count))) (lit (" distinct lattice edges"))) (Option.none)) } : Looked) else ({ bud := ({ states := (bud).states, edges := count, memo := (position :: (bud).memo) } : Bud), found := found, error := Option.none } : Looked)))))

@[expose] public def tokenAfter (ctx : Ctx) («from» : Nat) (atomEnd : Nat) : Nat := ((LexLeanCollections.iterateUntil ((fun (position : Nat) => (match tokenAt (ctx) (position) with | Option.some token => (if (Nat.blt (LexLeanBoot.GrammarProof.firstAtom (token)) (atomEnd)) then Option.some ((position + 1)) else Option.none) | Option.none => Option.none))) (((LexLeanRuntime.length ((ctx).tokens) : Nat) + 1)) («from») : (Prod (Nat) (Bool)))).1

@[expose] public def formMatches (ctx : Ctx) (bud : Bud) (position : Nat) (low : Nat) (high : Nat) : Matched := (match tokenAt (ctx) (position) with | Option.some token => (match token with | LexLeanBoot.GrammarProof.TextToken.TokenAtom which => (let looked : Looked := edgesAt (ctx) (bud) (which); (match (looked).error with | Option.some problem => ({ bud := (looked).bud, found := ([] : List (Match)), error := Option.some (problem) } : Matched) | Option.none => ({ bud := (looked).bud, found := (LexLeanCollections.listFold ((fun (out : List (Match)) (edge : TEdge) => (LexLeanRuntime.append (out) ((if both ((Nat.ble (low) ((edge).category))) ((Nat.ble ((edge).category) (high))) then (({ form := (edge).form, category := (edge).category, inflected := (edge).inflected, upto := tokenAfter (ctx) (position) ((edge).stop) } : Match) :: ([] : List (Match))) else ([] : List (Match)))) : List (Match)))) (([] : List (Match))) ((looked).found) : List (Match)), error := Option.none } : Matched))) | LexLeanBoot.GrammarProof.TextToken.TokenIsland _ _ _ _ _ => ({ bud := bud, found := ([] : List (Match)), error := Option.none } : Matched)) | Option.none => ({ bud := bud, found := ([] : List (Match)), error := Option.none } : Matched))

@[expose] public def atomRange (ctx : Ctx) (start : Nat) (stop : Nat) : (Prod (Nat) (Nat)) := (let firstToken : LexLeanBoot.GrammarProof.TextToken := (match tokenAt (ctx) (start) with | Option.some token => token | Option.none => LexLeanBoot.GrammarProof.TextToken.TokenAtom (0)); (let lastIndex : Nat := (if (Nat.blt (start) ((LexLeanRuntime.subtract (stop) (1) : Nat))) then (LexLeanRuntime.subtract (stop) (1) : Nat) else start); (let lastToken : LexLeanBoot.GrammarProof.TextToken := (match tokenAt (ctx) (lastIndex) with | Option.some token => token | Option.none => LexLeanBoot.GrammarProof.TextToken.TokenAtom (0)); (LexLeanBoot.GrammarProof.firstAtom (firstToken), (lastAtom (lastToken) + 1)))))

@[expose] public def wsStep (ctx : Ctx) (stop : Nat) (position : Nat) : Option (Nat) := (if both ((Nat.blt (position) (stop))) ((Nat.beq (LexLeanBoot.PrimitiveScan.classCode (atomOr (ctx) (position))) (6))) then Option.some ((position + 1)) else Option.none)

@[expose] public def skipWs (ctx : Ctx) (stop : Nat) (position : Nat) : Nat := ((LexLeanCollections.iterateUntil ((fun (current : Nat) => wsStep (ctx) (stop) (current))) (((LexLeanRuntime.length ((ctx).atoms) : Nat) + 1)) (position) : (Prod (Nat) (Bool)))).1

@[expose] public def composeStep (ctx : Ctx) (state : (Prod (Nat) (Nat))) : Option ((Prod (Nat) (Nat))) := (match (LexLeanRuntime.index ((ctx).atoms) ((state).1) : Option (LexLeanBoot.PrimitiveScan.Atom)) with | Option.some atom => (let code : Nat := LexLeanBoot.PrimitiveScan.classCode (atom); (if both ((Nat.beq ((atom).byteStart) ((state).2))) ((if (if (Nat.beq (code) (1)) then true else (Nat.beq (code) (2))) then true else both ((Nat.beq (code) (4))) ((if same ((atom).text) (lit ("_")) then true else same ((atom).text) (lit ("'")))))) then Option.some ((((state).1 + 1), (atom).byteEnd)) else Option.none)) | Option.none => Option.none)

@[expose] public def spelledFrom (ctx : Ctx) (start : Nat) (stop : Nat) : ByteArray := (((LexLeanCollections.iterateUntil ((fun (state : (Prod (ByteArray) (Nat))) => (if (Nat.blt ((state).2) (stop)) then Option.some ((cat ((state).1) ((atomOr (ctx) ((state).2)).text), ((state).2 + 1))) else Option.none))) (((LexLeanRuntime.subtract (stop) (start) : Nat) + 1)) ((lit (""), start)) : (Prod ((Prod (ByteArray) (Nat))) (Bool)))).1).1

@[expose] public def binderLocal (ctx : Ctx) (position : Nat) (phrase : TypePhrase) : Option (BAlt) := (match tokenAt (ctx) (position) with | Option.some token => (match token with | LexLeanBoot.GrammarProof.TextToken.TokenAtom _ => Option.none | LexLeanBoot.GrammarProof.TextToken.TokenIsland _ innerStart innerEnd _ _ => (let index : Nat := skipWs (ctx) (innerEnd) (innerStart); (let atom : LexLeanBoot.PrimitiveScan.Atom := atomOr (ctx) (index); (if both ((Nat.beq (LexLeanBoot.PrimitiveScan.classCode (atom)) (1))) ((Nat.blt (index) ((LexLeanRuntime.length ((ctx).atoms) : Nat)))) then (let identEnd : Nat := (((LexLeanCollections.iterateUntil ((fun (state : (Prod (Nat) (Nat))) => composeStep (ctx) (state))) (((LexLeanRuntime.length ((ctx).atoms) : Nat) + 1)) (((index + 1), (atom).byteEnd)) : (Prod ((Prod (Nat) (Nat))) (Bool)))).1).1; (if (Nat.beq (skipWs (ctx) (innerEnd) (identEnd)) (innerEnd)) then Option.some (({ upto := (position + 1), binder := ({ phrase := phrase, spelling := spelledFrom (ctx) (index) (identEnd), island := token, identStart := index, identEnd := identEnd, keywords := ([] : List (LexLeanBoot.GrammarProof.Keyword)) } : Binder) } : BAlt)) else Option.none)) else Option.none)))) | Option.none => Option.none)

@[expose] public def withKeywords (alternative : BAlt) (keywords : List (LexLeanBoot.GrammarProof.Keyword)) : BAlt := (let held : Binder := (alternative).binder; ({ upto := (alternative).upto, binder := ({ phrase := (held).phrase, spelling := (held).spelling, island := (held).island, identStart := (held).identStart, identEnd := (held).identEnd, keywords := (LexLeanRuntime.append (keywords) ((held).keywords) : List (LexLeanBoot.GrammarProof.Keyword)) } : Binder) } : BAlt))

@[expose] public def binderStart (ctx : Ctx) (state : (Prod (List (BAlt)) (Matched))) (start : (Prod (Nat) (List (LexLeanBoot.GrammarProof.Keyword)))) : (Prod (List (BAlt)) (Matched)) := (let held : Matched := (state).2; (match (held).error with | Option.some _ => state | Option.none => (let matched : Matched := formMatches (ctx) ((held).bud) ((start).1) (0) (0); (match (matched).error with | Option.some _ => ((state).1, matched) | Option.none => (let nouns : List (BAlt) := (LexLeanCollections.listFold ((fun (out : List (BAlt)) (found : Match) => (if (found).inflected then out else (match binderLocal (ctx) ((found).upto) (TypePhrase.TNoun (((found).form :: ([] : List (Nat)))) ((atomRange (ctx) ((start).1) ((found).upto)).1) ((atomRange (ctx) ((start).1) ((found).upto)).2)) with | Option.some alternative => (LexLeanRuntime.append (out) ((withKeywords (alternative) ((start).2) :: ([] : List (BAlt)))) : List (BAlt)) | Option.none => out)))) ((state).1) ((matched).found) : List (BAlt)); (let island : List (BAlt) := (match islandAt (ctx) ((start).1) with | Option.some token => (match binderLocal (ctx) (((start).1 + 1)) (TypePhrase.TMath (token)) with | Option.some alternative => (withKeywords (alternative) ((start).2) :: ([] : List (BAlt))) | Option.none => ([] : List (BAlt))) | Option.none => ([] : List (BAlt))); ((LexLeanRuntime.append (nouns) (island) : List (BAlt)), matched)))))))

@[expose] public def context (path : ByteArray) (atoms : List (LexLeanBoot.PrimitiveScan.Atom)) (tokens : List (LexLeanBoot.GrammarProof.TextToken)) (edges : List (List (TEdge))) (maxEdges : Nat) (maxStates : Nat) (maxDepth : Nat) : Ctx := ({ path := path, atoms := atoms, tokens := tokens, edges := edges, maxStates := maxStates, maxDepth := maxDepth, maxEdges := maxEdges } : Ctx)

@[expose] public def fresh (statesBefore : Nat) (edgesBefore : Nat) : Bud := ({ states := statesBefore, edges := edgesBefore, memo := ([] : List (Nat)) } : Bud)

@[expose] public def coveredStep (ctx : Ctx) (index : Nat) (state : (Prod (Nat) (Scan))) : Option ((Prod (Nat) (Scan))) := (let start : Nat := (state).1; (let scan : Scan := (state).2; (if both ((Nat.ble (start) (index))) ((match (scan).found with | Option.some _ => false | Option.none => (match (scan).error with | Option.some _ => false | Option.none => true))) then (let looked : Looked := edgesAt (ctx) ((scan).bud) (start); (match (looked).error with | Option.some problem => Option.some (((start + 1), ({ bud := (looked).bud, found := (scan).found, error := Option.some (problem) } : Scan))) | Option.none => (if (LexLeanCollections.listFold ((fun (hit : Bool) (edge : TEdge) => (if hit then true else both ((Nat.ble (start) (index))) ((Nat.blt (index) ((edge).stop)))))) (false) ((looked).found) : Bool) then Option.some (((start + 1), ({ bud := (looked).bud, found := Option.some (placed (lit ("")) (lit ("")) (Option.none)), error := Option.none } : Scan))) else Option.some (((start + 1), ({ bud := (looked).bud, found := Option.none, error := Option.none } : Scan)))))) else Option.none)))

@[expose] public def unknownStep (ctx : Ctx) (rangeStart : Nat) (state : Scan) (index : Nat) : Scan := (match (state).found with | Option.some _ => state | Option.none => (match (state).error with | Option.some _ => state | Option.none => (let atom : LexLeanBoot.PrimitiveScan.Atom := atomOr (ctx) (index); (let code : Nat := LexLeanBoot.PrimitiveScan.classCode (atom); (if (if (Nat.beq (code) (6)) then true else both ((Nat.beq (code) (1))) ((match keywordEntry ((atom).text) with | Option.some _ => true | Option.none => proofKeyword ((atom).text)))) then state else (let final : Scan := (((LexLeanCollections.iterateUntil ((fun (pair : (Prod (Nat) (Scan))) => coveredStep (ctx) (index) (pair))) (((LexLeanRuntime.subtract (index) (rangeStart) : Nat) + 2)) ((rangeStart, ({ bud := (state).bud, found := Option.none, error := Option.none } : Scan))) : (Prod ((Prod (Nat) (Scan))) (Bool)))).1).2; (match (final).error with | Option.some _ => final | Option.none => (match (final).found with | Option.some _ => ({ bud := (final).bud, found := Option.none, error := Option.none } : Scan) | Option.none => ({ bud := (final).bud, found := Option.some (placed (lit ("LLL1004")) (cat (cat (lit ("`")) ((atom).text)) (lit ("` is not a declared atom in any visible glossary"))) (Option.some (placeAt (ctx) (index)))), error := Option.none } : Scan)))))))))

@[expose] public def unknownAtom (path : ByteArray) (atoms : List (LexLeanBoot.PrimitiveScan.Atom)) (tokens : List (LexLeanBoot.GrammarProof.TextToken)) (edges : List (List (TEdge))) (maxEdges : Nat) (maxStates : Nat) (maxDepth : Nat) (statesBefore : Nat) (edgesBefore : Nat) : Except (LexLeanBoot.PrimitiveScan.Rejection) (Option (LexLeanBoot.PrimitiveScan.Rejection)) := (let ctx : Ctx := context (path) (atoms) (tokens) (edges) (maxEdges) (maxStates) (maxDepth); (let indices : List (Nat) := (LexLeanCollections.listFold ((fun (out : List (Nat)) (token : LexLeanBoot.GrammarProof.TextToken) => (match token with | LexLeanBoot.GrammarProof.TextToken.TokenAtom which => (LexLeanRuntime.append (out) ((which :: ([] : List (Nat)))) : List (Nat)) | LexLeanBoot.GrammarProof.TextToken.TokenIsland _ _ _ _ _ => out))) (([] : List (Nat))) (tokens) : List (Nat)); (match (LexLeanRuntime.index (indices) (0) : Option (Nat)) with | Option.none => Except.ok (Option.none) | Option.some rangeStart => (let final : Scan := (LexLeanCollections.listFold ((fun (state : Scan) (index : Nat) => unknownStep (ctx) (rangeStart) (state) (index))) (({ bud := fresh (statesBefore) (edgesBefore), found := Option.none, error := Option.none } : Scan)) (indices) : Scan); (match (final).error with | Option.some problem => Except.error (problem) | Option.none => Except.ok ((final).found))))))

@[expose] public def binderAlternatives (path : ByteArray) (atoms : List (LexLeanBoot.PrimitiveScan.Atom)) (tokens : List (LexLeanBoot.GrammarProof.TextToken)) (edges : List (List (TEdge))) (maxEdges : Nat) (maxStates : Nat) (maxDepth : Nat) (statesBefore : Nat) (edgesBefore : Nat) (position : Nat) (required : Bool) : Except (LexLeanBoot.PrimitiveScan.Rejection) (List (BAlt)) := (let ctx : Ctx := context (path) (atoms) (tokens) (edges) (maxEdges) (maxStates) (maxDepth); (let charge : (Prod (Bud) (Option (LexLeanBoot.PrimitiveScan.Rejection))) := chargeState (ctx) (fresh (statesBefore) (edgesBefore)); (match (charge).2 with | Option.some problem => Except.error (problem) | Option.none => (let starts : List ((Prod (Nat) (List (LexLeanBoot.GrammarProof.Keyword)))) := (if required then (LexLeanRuntime.append ((match isWord (ctx) (position) (lit ("a")) with | Option.some keyword => (((position + 1), (keyword :: ([] : List (LexLeanBoot.GrammarProof.Keyword)))) :: ([] : List ((Prod (Nat) (List (LexLeanBoot.GrammarProof.Keyword)))))) | Option.none => ([] : List ((Prod (Nat) (List (LexLeanBoot.GrammarProof.Keyword))))))) ((match isWord (ctx) (position) (lit ("an")) with | Option.some keyword => (((position + 1), (keyword :: ([] : List (LexLeanBoot.GrammarProof.Keyword)))) :: ([] : List ((Prod (Nat) (List (LexLeanBoot.GrammarProof.Keyword)))))) | Option.none => ([] : List ((Prod (Nat) (List (LexLeanBoot.GrammarProof.Keyword))))))) : List ((Prod (Nat) (List (LexLeanBoot.GrammarProof.Keyword))))) else ((position, ([] : List (LexLeanBoot.GrammarProof.Keyword))) :: ([] : List ((Prod (Nat) (List (LexLeanBoot.GrammarProof.Keyword))))))); (let final : (Prod (List (BAlt)) (Matched)) := (LexLeanCollections.listFold ((fun (state : (Prod (List (BAlt)) (Matched))) (start : (Prod (Nat) (List (LexLeanBoot.GrammarProof.Keyword)))) => binderStart (ctx) (state) (start))) ((([] : List (BAlt)), ({ bud := (charge).1, found := ([] : List (Match)), error := Option.none } : Matched))) (starts) : (Prod (List (BAlt)) (Matched))); (match ((final).2).error with | Option.some problem => Except.error (problem) | Option.none => Except.ok ((final).1)))))))

@[expose] public def nounOf (ctx : Ctx) (position : Nat) (found : Match) (args : List (Term)) (keywords : List (LexLeanBoot.GrammarProof.Keyword)) : Term := (let range : (Prod (Nat) (Nat)) := atomRange (ctx) ((position + 1)) ((found).upto); Term.TNounOf (((found).form :: ([] : List (Nat)))) ((range).1) ((range).2) ((LexLeanCollections.listFold ((fun (rest : Term) (arg : Term) => Term.TArgCons (arg) (rest))) (Term.TArgNil) ((LexLeanCollections.listFold ((fun (turned : List (Term)) (arg : Term) => (arg :: turned))) (([] : List (Term))) (args) : List (Term))) : Term)) (keywords))

@[expose] public def mk (m : Mach) (stack : List (Frame)) (returned : List (TAlt)) : Mach := ({ stack := stack, returned := returned, error := (m).error, bud := (m).bud } : Mach)

@[expose] public def failed (m : Mach) (bud : Bud) (problem : LexLeanBoot.PrimitiveScan.Rejection) : Mach := ({ stack := (m).stack, returned := (m).returned, error := Option.some (problem), bud := bud } : Mach)

@[expose] public def startStep (ctx : Ctx) (m : Mach) (rest : List (Frame)) (position : Nat) (depth : Nat) : Mach := (let charge : (Prod (Bud) (Option (LexLeanBoot.PrimitiveScan.Rejection))) := chargeState (ctx) ((m).bud); (match (charge).2 with | Option.some problem => failed (m) ((charge).1) (problem) | Option.none => (match depthRefusal (ctx) (depth) (position) with | Option.some problem => failed (m) ((charge).1) (problem) | Option.none => (let alts : List (TAlt) := (match islandAt (ctx) (position) with | Option.some token => (({ upto := (position + 1), term := Term.TIsland (token) } : TAlt) :: ([] : List (TAlt))) | Option.none => ([] : List (TAlt))); (match isWord (ctx) (position) (lit ("the")) with | Option.none => ({ stack := rest, returned := alts, error := Option.none, bud := (charge).1 } : Mach) | Option.some theKw => (let matched : Matched := formMatches (ctx) ((charge).1) ((position + 1)) (1) (2); (match (matched).error with | Option.some problem => failed (m) ((matched).bud) (problem) | Option.none => ({ stack := (Frame.XMatches (position) (depth) (theKw) (alts) ((matched).found) :: rest), returned := ([] : List (TAlt)), error := Option.none, bud := (matched).bud } : Mach))))))))

@[expose] public def matchesStep (ctx : Ctx) (m : Mach) (rest : List (Frame)) (position : Nat) (depth : Nat) (theKw : LexLeanBoot.GrammarProof.Keyword) (alts : List (TAlt)) (pending : List (Match)) : Mach := (match pending with | List.nil => mk (m) (rest) (alts) | List.cons found later => (match isWord (ctx) ((found).upto) (lit ("of")) with | Option.some ofKw => mk (m) ((Frame.XStart (((found).upto + 1)) ((depth + 1)) :: (Frame.XAfterFirst (position) (depth) (theKw) (alts) (later) (found) (ofKw) :: rest))) (([] : List (TAlt))) | Option.none => mk (m) ((Frame.XMatches (position) (depth) (theKw) (alts) (later) :: rest)) (([] : List (TAlt)))))

@[expose] public def firstsStep (ctx : Ctx) (m : Mach) (rest : List (Frame)) (position : Nat) (depth : Nat) (theKw : LexLeanBoot.GrammarProof.Keyword) (alts : List (TAlt)) (pending : List (Match)) (found : Match) (ofKw : LexLeanBoot.GrammarProof.Keyword) (firsts : List (TAlt)) : Mach := (match firsts with | List.nil => mk (m) ((Frame.XMatches (position) (depth) (theKw) (alts) (pending) :: rest)) (([] : List (TAlt))) | List.cons held later => (if (Nat.beq ((found).category) (1)) then mk (m) ((Frame.XFirsts (position) (depth) (theKw) ((LexLeanRuntime.append (alts) ((({ upto := (held).upto, term := nounOf (ctx) (position) (found) (((held).term :: ([] : List (Term)))) ((theKw :: (ofKw :: ([] : List (LexLeanBoot.GrammarProof.Keyword))))) } : TAlt) :: ([] : List (TAlt)))) : List (TAlt))) (pending) (found) (ofKw) (later) :: rest)) (([] : List (TAlt))) else (match isWord (ctx) ((held).upto) (lit ("and")) with | Option.some andKw => mk (m) ((Frame.XStart (((held).upto + 1)) ((depth + 1)) :: (Frame.XAfterSecond (position) (depth) (theKw) (alts) (pending) (found) (ofKw) (later) (held) (andKw) :: rest))) (([] : List (TAlt))) | Option.none => mk (m) ((Frame.XFirsts (position) (depth) (theKw) (alts) (pending) (found) (ofKw) (later) :: rest)) (([] : List (TAlt))))))

@[expose] public def secondsStep (ctx : Ctx) (m : Mach) (rest : List (Frame)) (position : Nat) (depth : Nat) (theKw : LexLeanBoot.GrammarProof.Keyword) (alts : List (TAlt)) (pending : List (Match)) (found : Match) (ofKw : LexLeanBoot.GrammarProof.Keyword) (firsts : List (TAlt)) (held : TAlt) (andKw : LexLeanBoot.GrammarProof.Keyword) : Mach := (let folded : (Prod (Bud) ((Prod (List (TAlt)) (Option (LexLeanBoot.PrimitiveScan.Rejection))))) := (LexLeanCollections.listFold ((fun (state : (Prod (Bud) ((Prod (List (TAlt)) (Option (LexLeanBoot.PrimitiveScan.Rejection)))))) (latter : TAlt) => (match ((state).2).2 with | Option.some _ => state | Option.none => (let charge : (Prod (Bud) (Option (LexLeanBoot.PrimitiveScan.Rejection))) := chargeState (ctx) ((state).1); (match (charge).2 with | Option.some problem => ((charge).1, (((state).2).1, Option.some (problem))) | Option.none => ((charge).1, ((LexLeanRuntime.append (((state).2).1) ((({ upto := (latter).upto, term := nounOf (ctx) (position) (found) (((held).term :: ((latter).term :: ([] : List (Term))))) ((theKw :: (ofKw :: (andKw :: ([] : List (LexLeanBoot.GrammarProof.Keyword)))))) } : TAlt) :: ([] : List (TAlt)))) : List (TAlt)), Option.none))))))) (((m).bud, (alts, Option.none))) ((m).returned) : (Prod (Bud) ((Prod (List (TAlt)) (Option (LexLeanBoot.PrimitiveScan.Rejection)))))); (match ((folded).2).2 with | Option.some problem => failed (m) ((folded).1) (problem) | Option.none => ({ stack := (Frame.XFirsts (position) (depth) (theKw) (((folded).2).1) (pending) (found) (ofKw) (firsts) :: rest), returned := ([] : List (TAlt)), error := Option.none, bud := (folded).1 } : Mach)))

@[expose] public def machStep (ctx : Ctx) (m : Mach) : Option (Mach) := (match (m).error with | Option.some _ => Option.none | Option.none => (match (m).stack with | List.nil => Option.none | List.cons frame rest => Option.some ((match frame with | Frame.XStart position depth => startStep (ctx) (m) (rest) (position) (depth) | Frame.XMatches position depth theKw alts pending => matchesStep (ctx) (m) (rest) (position) (depth) (theKw) (alts) (pending) | Frame.XAfterFirst position depth theKw alts pending found ofKw => mk (m) ((Frame.XFirsts (position) (depth) (theKw) (alts) (pending) (found) (ofKw) ((m).returned) :: rest)) (([] : List (TAlt))) | Frame.XFirsts position depth theKw alts pending found ofKw firsts => firstsStep (ctx) (m) (rest) (position) (depth) (theKw) (alts) (pending) (found) (ofKw) (firsts) | Frame.XAfterSecond position depth theKw alts pending found ofKw firsts held andKw => secondsStep (ctx) (m) (rest) (position) (depth) (theKw) (alts) (pending) (found) (ofKw) (firsts) (held) (andKw)))))

@[expose] public def termPhrase (path : ByteArray) (atoms : List (LexLeanBoot.PrimitiveScan.Atom)) (tokens : List (LexLeanBoot.GrammarProof.TextToken)) (edges : List (List (TEdge))) (maxEdges : Nat) (maxStates : Nat) (maxDepth : Nat) (statesBefore : Nat) (edgesBefore : Nat) (position : Nat) (depth : Nat) : Except (LexLeanBoot.PrimitiveScan.Rejection) (List (TAlt)) := (let ctx : Ctx := context (path) (atoms) (tokens) (edges) (maxEdges) (maxStates) (maxDepth); (let final : Mach := ((LexLeanCollections.iterateUntil ((fun (m : Mach) => machStep (ctx) (m))) (((LexLeanRuntime.multiply ((maxStates + 2)) (64) : Nat) + 64)) (({ stack := (Frame.XStart (position) (depth) :: ([] : List (Frame))), returned := ([] : List (TAlt)), error := Option.none, bud := fresh (statesBefore) (edgesBefore) } : Mach)) : (Prod (Mach) (Bool)))).1; (match (final).error with | Option.some problem => Except.error (problem) | Option.none => Except.ok ((final).returned))))

end LexLeanBoot.PropositionTerms
