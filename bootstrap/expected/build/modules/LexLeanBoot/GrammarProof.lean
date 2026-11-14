module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Text
public import LexLeanBoot.PrimitiveScan
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.GrammarProof

public inductive TextToken where
  | TokenAtom (_ : Nat)
  | TokenIsland (_ : Nat) (_ : Nat) (_ : Nat) (_ : Nat) (_ : Bool)

public structure Keyword where
  atom : Nat
  entry : ByteArray

public inductive SentenceKind where
  | Assume (_ : List (TextToken))
  | Apply (_ : TextToken)
  | CloseWith (_ : TextToken)
  | CloseByReflexivity
  | Witness (_ : TextToken)
  | SelectLeft
  | SelectRight

public structure Sentence where
  kind : SentenceKind
  keywords : List (Keyword)

public structure Scan where
  cursor : Nat
  tokens : List (TextToken)
  error : Option (LexLeanBoot.PrimitiveScan.Rejection)

public structure Commas where
  cursor : Nat
  islands : List (TextToken)

public structure Found where
  word : ByteArray
  atom : Nat

public structure Forbid where
  cursor : Nat
  found : Option (Found)

public structure Compose where
  spelled : ByteArray
  reach : Nat
  next : Nat

@[expose] public def lit (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def cat (left : ByteArray) (right : ByteArray) : ByteArray := (LexLeanRuntime.append (left) (right) : ByteArray)

@[expose] public def both (left : Bool) (right : Bool) : Bool := (if left then right else false)

@[expose] public def reverseTokens (items : List (TextToken)) : List (TextToken) := (LexLeanCollections.listFold ((fun (turned : List (TextToken)) (item : TextToken) => (item :: turned))) (([] : List (TextToken))) (items) : List (TextToken))

@[expose] public def isControl (atom : LexLeanBoot.PrimitiveScan.Atom) (spelled : ByteArray) : Bool := both ((Nat.beq (LexLeanBoot.PrimitiveScan.classCode (atom)) (0))) ((LexLeanRuntime.equal ((atom).text) (spelled) : Bool))

@[expose] public def opens (atom : LexLeanBoot.PrimitiveScan.Atom) : Bool := (if isControl (atom) (lit ("\\(")) then true else isControl (atom) (lit ("\\[")))

@[expose] public def refusal (path : ByteArray) (code : ByteArray) (message : ByteArray) (atom : LexLeanBoot.PrimitiveScan.Atom) : LexLeanBoot.PrimitiveScan.Rejection := ({ code := code, message := message, place := Option.some (LexLeanBoot.PrimitiveScan.placeOfAtom (path) (atom)), help := Option.none } : LexLeanBoot.PrimitiveScan.Rejection)

@[expose] public def failed (scan : Scan) (problem : LexLeanBoot.PrimitiveScan.Rejection) : Scan := ({ cursor := (scan).cursor, tokens := (scan).tokens, error := Option.some (problem) } : Scan)

@[expose] public def pushed (scan : Scan) (next : Nat) (token : TextToken) : Scan := ({ cursor := next, tokens := (token :: (scan).tokens), error := (scan).error } : Scan)

@[expose] public def closerStep (atoms : List (LexLeanBoot.PrimitiveScan.Atom)) (closer : ByteArray) (stop : Nat) (position : Nat) : Option (Nat) := (if (Nat.blt (position) (stop)) then (match (LexLeanRuntime.index (atoms) (position) : Option (LexLeanBoot.PrimitiveScan.Atom)) with | Option.some atom => (if isControl (atom) (closer) then Option.none else (if opens (atom) then Option.none else Option.some ((position + 1)))) | Option.none => Option.none) else Option.none)

@[expose] public def islandAt (path : ByteArray) (atoms : List (LexLeanBoot.PrimitiveScan.Atom)) (stop : Nat) (scan : Scan) (position : Nat) (atom : LexLeanBoot.PrimitiveScan.Atom) : Scan := (let shown : Bool := isControl (atom) (lit ("\\[")); (let closer : ByteArray := (if shown then lit ("\\]") else lit ("\\)")); (let unclosed : Scan := failed (scan) (refusal (path) (lit ("LLL1006")) (cat (cat (lit ("unclosed mathematical island; expected `")) (closer)) (lit ("`"))) (atom)); (let reached : Nat := ((LexLeanCollections.iterateUntil ((fun (scanned : Nat) => closerStep (atoms) (closer) (stop) (scanned))) (((LexLeanRuntime.subtract (stop) (position) : Nat) + 1)) ((position + 1)) : (Prod (Nat) (Bool)))).1; (if (Nat.blt (reached) (stop)) then (match (LexLeanRuntime.index (atoms) (reached) : Option (LexLeanBoot.PrimitiveScan.Atom)) with | Option.some candidate => (if isControl (candidate) (closer) then pushed (scan) ((reached + 1)) (TextToken.TokenIsland (position) ((position + 1)) (reached) (reached) (shown)) else failed (scan) (refusal (path) (lit ("LLL1006")) (lit ("mathematical islands do not nest")) (candidate))) | Option.none => unclosed) else unclosed)))))

@[expose] public def tokenAt (path : ByteArray) (atoms : List (LexLeanBoot.PrimitiveScan.Atom)) (stop : Nat) (scan : Scan) (position : Nat) (atom : LexLeanBoot.PrimitiveScan.Atom) : Scan := (let code : Nat := LexLeanBoot.PrimitiveScan.classCode (atom); (if (Nat.beq (code) (6)) then ({ cursor := (position + 1), tokens := (scan).tokens, error := (scan).error } : Scan) else (if both ((Nat.beq (code) (4))) ((LexLeanRuntime.equal ((atom).text) (lit ("$")) : Bool)) then failed (scan) (refusal (path) (lit ("LLP2001")) (lit ("dollar math delimiters are forbidden; use \\( ... \\) or \\[ ... \\]")) (atom)) else (if opens (atom) then islandAt (path) (atoms) (stop) (scan) (position) (atom) else (if (if isControl (atom) (lit ("\\)")) then true else isControl (atom) (lit ("\\]"))) then failed (scan) (refusal (path) (lit ("LLL1006")) (cat (cat (lit ("`")) ((atom).text)) (lit ("` closes no mathematical island"))) (atom)) else pushed (scan) ((position + 1)) (TextToken.TokenAtom (position)))))))

@[expose] public def tokenStep (path : ByteArray) (atoms : List (LexLeanBoot.PrimitiveScan.Atom)) (stop : Nat) (scan : Scan) : Option (Scan) := (match (scan).error with | Option.some _ => Option.none | Option.none => (let position : Nat := (scan).cursor; (if (Nat.blt (position) (stop)) then (match (LexLeanRuntime.index (atoms) (position) : Option (LexLeanBoot.PrimitiveScan.Atom)) with | Option.some atom => Option.some (tokenAt (path) (atoms) (stop) (scan) (position) (atom)) | Option.none => Option.none) else Option.none)))

@[expose] public def textTokens (path : ByteArray) (atoms : List (LexLeanBoot.PrimitiveScan.Atom)) (start : Nat) (stop : Nat) : Except (LexLeanBoot.PrimitiveScan.Rejection) (List (TextToken)) := (let final : Scan := ((LexLeanCollections.iterateUntil ((fun (scan : Scan) => tokenStep (path) (atoms) (stop) (scan))) (((LexLeanRuntime.subtract (stop) (start) : Nat) + 1)) (({ cursor := start, tokens := ([] : List (TextToken)), error := Option.none } : Scan)) : (Prod (Scan) (Bool)))).1; (match (final).error with | Option.some problem => Except.error (problem) | Option.none => Except.ok (reverseTokens ((final).tokens))))

@[expose] public def firstAtom (token : TextToken) : Nat := (match token with | TextToken.TokenAtom which => which | TextToken.TokenIsland opening _ _ _ _ => opening)

@[expose] public def atomOr (atoms : List (LexLeanBoot.PrimitiveScan.Atom)) (which : Nat) : LexLeanBoot.PrimitiveScan.Atom := (match (LexLeanRuntime.index (atoms) (which) : Option (LexLeanBoot.PrimitiveScan.Atom)) with | Option.some atom => atom | Option.none => ({ kind := LexLeanBoot.PrimitiveScan.AtomClass.Whitespace, byteStart := 0, byteEnd := 0, lineStart := 1, columnStart := 1, lineEnd := 1, columnEnd := 1, text := lit ("") } : LexLeanBoot.PrimitiveScan.Atom))

@[expose] public def tokenOr (tokens : List (TextToken)) (position : Nat) : TextToken := (match (LexLeanRuntime.index (tokens) (position) : Option (TextToken)) with | Option.some token => token | Option.none => TextToken.TokenAtom (0))

@[expose] public def wordIs (atoms : List (LexLeanBoot.PrimitiveScan.Atom)) (tokens : List (TextToken)) (position : Nat) (expected : ByteArray) : Bool := (match (LexLeanRuntime.index (tokens) (position) : Option (TextToken)) with | Option.some token => (match token with | TextToken.TokenAtom which => (match (LexLeanRuntime.index (atoms) (which) : Option (LexLeanBoot.PrimitiveScan.Atom)) with | Option.some atom => both ((Nat.beq (LexLeanBoot.PrimitiveScan.classCode (atom)) (1))) ((LexLeanRuntime.equal ((atom).text) (expected) : Bool)) | Option.none => false) | TextToken.TokenIsland _ _ _ _ _ => false) | Option.none => false)

@[expose] public def islandIs (tokens : List (TextToken)) (position : Nat) : Bool := (match (LexLeanRuntime.index (tokens) (position) : Option (TextToken)) with | Option.some token => (match token with | TextToken.TokenAtom _ => false | TextToken.TokenIsland _ _ _ _ _ => true) | Option.none => false)

@[expose] public def commaIs (atoms : List (LexLeanBoot.PrimitiveScan.Atom)) (tokens : List (TextToken)) (position : Nat) : Bool := (match (LexLeanRuntime.index (tokens) (position) : Option (TextToken)) with | Option.some token => (match token with | TextToken.TokenAtom which => (match (LexLeanRuntime.index (atoms) (which) : Option (LexLeanBoot.PrimitiveScan.Atom)) with | Option.some atom => both ((Nat.beq (LexLeanBoot.PrimitiveScan.classCode (atom)) (4))) ((LexLeanRuntime.equal ((atom).text) (lit (",")) : Bool)) | Option.none => false) | TextToken.TokenIsland _ _ _ _ _ => false) | Option.none => false)

@[expose] public def keywordAt (tokens : List (TextToken)) (position : Nat) (entry : ByteArray) : Keyword := ({ atom := firstAtom (tokenOr (tokens) (position)), entry := entry } : Keyword)

@[expose] public def forbiddenWords : List (ByteArray) := (lit ("aesop") :: (lit ("admit") :: (lit ("cases") :: (lit ("constructor") :: (lit ("induction") :: (lit ("all_goals") :: (lit ("any_goals") :: (lit ("apply?") :: (lit ("assumption") :: (lit ("case") :: (lit ("decide") :: (lit ("exact") :: (lit ("exact?") :: (lit ("first") :: (lit ("intro") :: (lit ("linarith") :: (lit ("native_decide") :: (lit ("next") :: (lit ("omega") :: (lit ("refine") :: (lit ("repeat") :: (lit ("rfl") :: (lit ("rw") :: (lit ("rwa") :: (lit ("simp") :: (lit ("simp_all") :: (lit ("simpa") :: (lit ("sorry") :: (lit ("tauto") :: (lit ("trivial") :: (lit ("try") :: ([] : List (ByteArray)))))))))))))))))))))))))))))))))

@[expose] public def separator (spelled : ByteArray) : Bool := LexLeanBoot.PrimitiveScan.bytesInList (spelled) ((lit (",") :: (lit (".") :: (lit (";") :: (lit (":") :: (lit ("(") :: (lit (")") :: ([] : List (ByteArray)))))))))

@[expose] public def composeStep (atoms : List (LexLeanBoot.PrimitiveScan.Atom)) (state : Compose) : Option (Compose) := (match (LexLeanRuntime.index (atoms) ((state).next) : Option (LexLeanBoot.PrimitiveScan.Atom)) with | Option.some atom => (let code : Nat := LexLeanBoot.PrimitiveScan.classCode (atom); (if both ((Nat.beq ((atom).byteStart) ((state).reach))) (both ((if (Nat.beq (code) (1)) then true else (if (Nat.beq (code) (2)) then true else (Nat.beq (code) (4))))) ((if separator ((atom).text) then false else true))) then Option.some (({ spelled := cat ((state).spelled) ((atom).text), reach := (atom).byteEnd, next := ((state).next + 1) } : Compose)) else Option.none)) | Option.none => Option.none)

@[expose] public def composedAt (atoms : List (LexLeanBoot.PrimitiveScan.Atom)) (atom : LexLeanBoot.PrimitiveScan.Atom) (which : Nat) : ByteArray := (((LexLeanCollections.iterateUntil ((fun (state : Compose) => composeStep (atoms) (state))) (((LexLeanRuntime.length (atoms) : Nat) + 1)) (({ spelled := (atom).text, reach := (atom).byteEnd, next := (which + 1) } : Compose)) : (Prod (Compose) (Bool)))).1).spelled

@[expose] public def forbiddenAt (atoms : List (LexLeanBoot.PrimitiveScan.Atom)) (position : Nat) (which : Nat) : Option (Found) := (let atom : LexLeanBoot.PrimitiveScan.Atom := atomOr (atoms) (which); (let composed : ByteArray := composedAt (atoms) (atom) (which); (let hit : Found := ({ word := composed, atom := which } : Found); (if LexLeanBoot.PrimitiveScan.bytesInList (composed) (forbiddenWords) then Option.some (hit) else (if both ((Nat.beq (position) (0))) ((match LexLeanStd.Text.find (composed) (lit ("_")) with | Option.some _ => true | Option.none => false)) then Option.some (hit) else (if both ((Nat.beq (LexLeanBoot.PrimitiveScan.classCode (atom)) (4))) ((if LexLeanStd.Text.startsWith (composed) (lit ("<;>")) then true else (LexLeanRuntime.equal ((atom).text) (lit ("·")) : Bool))) then Option.some (hit) else Option.none))))))

@[expose] public def forbidStep (atoms : List (LexLeanBoot.PrimitiveScan.Atom)) (state : Forbid) (token : TextToken) : Forbid := (let position : Nat := (state).cursor; (match (state).found with | Option.some _ => ({ cursor := (position + 1), found := (state).found } : Forbid) | Option.none => (match token with | TextToken.TokenAtom which => ({ cursor := (position + 1), found := forbiddenAt (atoms) (position) (which) } : Forbid) | TextToken.TokenIsland _ _ _ _ _ => ({ cursor := (position + 1), found := Option.none } : Forbid))))

@[expose] public def forbiddenForm (atoms : List (LexLeanBoot.PrimitiveScan.Atom)) (tokens : List (TextToken)) : Option (Found) := ((LexLeanCollections.listFold ((fun (state : Forbid) (token : TextToken) => forbidStep (atoms) (state) (token))) (({ cursor := 0, found := Option.none } : Forbid)) (tokens) : Forbid)).found

@[expose] public def failure (path : ByteArray) (atoms : List (LexLeanBoot.PrimitiveScan.Atom)) (tokens : List (TextToken)) : LexLeanBoot.PrimitiveScan.Rejection := (match forbiddenForm (atoms) (tokens) with | Option.some hit => refusal (path) (lit ("LLF5005")) (cat (cat (lit ("`")) ((hit).word)) (lit ("` is a forbidden proof form (§16.12): raw tactics, automation, tactic combinators, and proof holes are never accepted"))) (atomOr (atoms) ((hit).atom)) | Option.none => (let message : ByteArray := lit ("not a registered proof sentence; the exact simple sentences are fixed (§16.2)"); (match (LexLeanRuntime.index (tokens) (0) : Option (TextToken)) with | Option.some token => refusal (path) (lit ("LLF5005")) (message) (atomOr (atoms) (firstAtom (token))) | Option.none => ({ code := lit ("LLF5005"), message := message, place := Option.some (({ path := path, byteStart := 0, byteEnd := 0, lineStart := 1, columnStart := 1, lineEnd := 1, columnEnd := 1 } : LexLeanBoot.PrimitiveScan.Place)), help := Option.none } : LexLeanBoot.PrimitiveScan.Rejection))))

@[expose] public def accepted (kind : SentenceKind) (keywords : List (Keyword)) : Except (LexLeanBoot.PrimitiveScan.Rejection) (Sentence) := Except.ok (({ kind := kind, keywords := keywords } : Sentence))

@[expose] public def commaStep (atoms : List (LexLeanBoot.PrimitiveScan.Atom)) (tokens : List (TextToken)) (state : Commas) : Option (Commas) := (let position : Nat := (state).cursor; (if both (commaIs (atoms) (tokens) (position)) (islandIs (tokens) ((position + 1))) then Option.some (({ cursor := (position + 2), islands := (tokenOr (tokens) ((position + 1)) :: (state).islands) } : Commas)) else Option.none))

@[expose] public def assumed (atoms : List (LexLeanBoot.PrimitiveScan.Atom)) (tokens : List (TextToken)) (count : Nat) (refused : Except (LexLeanBoot.PrimitiveScan.Rejection) (Sentence)) : Except (LexLeanBoot.PrimitiveScan.Rejection) (Sentence) := (if islandIs (tokens) (1) then (let final : Commas := ((LexLeanCollections.iterateUntil ((fun (state : Commas) => commaStep (atoms) (tokens) (state))) ((count + 1)) (({ cursor := 2, islands := (tokenOr (tokens) (1) :: ([] : List (TextToken))) } : Commas)) : (Prod (Commas) (Bool)))).1; (if (Nat.beq ((final).cursor) (count)) then accepted (SentenceKind.Assume (reverseTokens ((final).islands))) ((keywordAt (tokens) (0) (lit ("assume")) :: ([] : List (Keyword)))) else refused)) else refused)

@[expose] public def closed (atoms : List (LexLeanBoot.PrimitiveScan.Atom)) (tokens : List (TextToken)) (count : Nat) (refused : Except (LexLeanBoot.PrimitiveScan.Rejection) (Sentence)) : Except (LexLeanBoot.PrimitiveScan.Rejection) (Sentence) := (if both (wordIs (atoms) (tokens) (1) (lit ("the"))) (wordIs (atoms) (tokens) (2) (lit ("goal"))) then (let opening : List (Keyword) := (keywordAt (tokens) (0) (lit ("close")) :: (keywordAt (tokens) (1) (lit ("the")) :: (keywordAt (tokens) (2) (lit ("goal")) :: ([] : List (Keyword))))); (if wordIs (atoms) (tokens) (3) (lit ("with")) then (if both (islandIs (tokens) (4)) ((Nat.beq (count) (5))) then accepted (SentenceKind.CloseWith (tokenOr (tokens) (4))) ((LexLeanRuntime.append (opening) ((keywordAt (tokens) (3) (lit ("with")) :: ([] : List (Keyword)))) : List (Keyword))) else refused) else (if both (wordIs (atoms) (tokens) (3) (lit ("by"))) (both (wordIs (atoms) (tokens) (4) (lit ("reflexivity"))) ((Nat.beq (count) (5)))) then accepted (SentenceKind.CloseByReflexivity) ((LexLeanRuntime.append (opening) ((keywordAt (tokens) (3) (lit ("by")) :: (keywordAt (tokens) (4) (lit ("reflexivity")) :: ([] : List (Keyword))))) : List (Keyword))) else refused))) else refused)

@[expose] public def selected (atoms : List (LexLeanBoot.PrimitiveScan.Atom)) (tokens : List (TextToken)) (count : Nat) (refused : Except (LexLeanBoot.PrimitiveScan.Rejection) (Sentence)) : Except (LexLeanBoot.PrimitiveScan.Rejection) (Sentence) := (if wordIs (atoms) (tokens) (1) (lit ("the")) then (let left : Bool := wordIs (atoms) (tokens) (2) (lit ("left")); (let right : Bool := (if left then false else wordIs (atoms) (tokens) (2) (lit ("right"))); (if both ((if left then true else right)) (both (wordIs (atoms) (tokens) (3) (lit ("alternative"))) ((Nat.beq (count) (4)))) then accepted ((if left then SentenceKind.SelectLeft else SentenceKind.SelectRight)) ((keywordAt (tokens) (0) (lit ("select")) :: (keywordAt (tokens) (1) (lit ("the")) :: (keywordAt (tokens) (2) ((if left then lit ("left") else lit ("right"))) :: (keywordAt (tokens) (3) (lit ("alternative")) :: ([] : List (Keyword))))))) else refused))) else refused)

@[expose] public def parseProofSentence (path : ByteArray) (atoms : List (LexLeanBoot.PrimitiveScan.Atom)) (tokens : List (TextToken)) : Except (LexLeanBoot.PrimitiveScan.Rejection) (Sentence) := (let count : Nat := (LexLeanRuntime.length (tokens) : Nat); (let refused : Except (LexLeanBoot.PrimitiveScan.Rejection) (Sentence) := Except.error (failure (path) (atoms) (tokens)); (if wordIs (atoms) (tokens) (0) (lit ("Assume")) then assumed (atoms) (tokens) (count) (refused) else (if wordIs (atoms) (tokens) (0) (lit ("Apply")) then (if both (islandIs (tokens) (1)) ((Nat.beq (count) (2))) then accepted (SentenceKind.Apply (tokenOr (tokens) (1))) ((keywordAt (tokens) (0) (lit ("apply-verb")) :: ([] : List (Keyword)))) else refused) else (if wordIs (atoms) (tokens) (0) (lit ("Close")) then closed (atoms) (tokens) (count) (refused) else (if wordIs (atoms) (tokens) (0) (lit ("Use")) then (if both (islandIs (tokens) (1)) (both (wordIs (atoms) (tokens) (2) (lit ("as"))) (both (wordIs (atoms) (tokens) (3) (lit ("the"))) (both (wordIs (atoms) (tokens) (4) (lit ("witness"))) ((Nat.beq (count) (5)))))) then accepted (SentenceKind.Witness (tokenOr (tokens) (1))) ((keywordAt (tokens) (0) (lit ("use")) :: (keywordAt (tokens) (2) (lit ("as")) :: (keywordAt (tokens) (3) (lit ("the")) :: (keywordAt (tokens) (4) (lit ("witness")) :: ([] : List (Keyword))))))) else refused) else (if wordIs (atoms) (tokens) (0) (lit ("Select")) then selected (atoms) (tokens) (count) (refused) else refused)))))))

end LexLeanBoot.GrammarProof
