module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Buffer
public import LexLeanStd.Numeral
public import LexLeanStd.Text
public import LexLeanBoot.PrimitiveScan
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.StructuralGrammar

public structure Arg where
  start : Nat
  stop : Nat
  text : ByteArray

public structure Sentence where
  start : Nat
  period : Nat

public inductive PolicyKind where
  | PolicyNone
  | PolicyAllow
  | PolicyExact

public structure Policy where
  kind : PolicyKind
  names : List (ByteArray)
  control : Nat
  namesArg : Option (Arg)

public inductive Origin where
  | Structural (_ : ByteArray) (_ : ByteArray)
  | Metadata (_ : ByteArray)

public structure Row where
  path : ByteArray
  byteStart : Nat
  byteEnd : Nat
  atomClass : LexLeanBoot.PrimitiveScan.AtomClass
  binding : Origin

public structure CalcStep where
  relation : Arg
  term : Arg
  proof : Arg

public inductive Proof where
  | ProofNil
  | ProofCons (_ : Proof) (_ : Proof)
  | ProofEnv (_ : Proof) (_ : Nat)
  | ProofSentence (_ : Sentence)
  | ProofHave (_ : Arg) (_ : Sentence) (_ : Proof) (_ : Nat)
  | ProofRewrite (_ : Arg) (_ : List ((Prod (Bool) (Arg)))) (_ : Nat)
  | ProofSimplify (_ : Arg) (_ : List (Arg)) (_ : Nat)
  | ProofApply (_ : Arg) (_ : Proof) (_ : Nat)
  | ProofConstructor (_ : Proof) (_ : Nat)
  | ProofCases (_ : Bool) (_ : Arg) (_ : Proof) (_ : Nat)
  | ProofCalculate (_ : Arg) (_ : List (CalcStep)) (_ : Nat)
  | LabeledCons (_ : Nat) (_ : Proof) (_ : Proof)
  | CaseCons (_ : Arg) (_ : List (ByteArray)) (_ : Arg) (_ : Proof) (_ : Proof)

public inductive DeclKind where
  | TypeDefinition
  | TermDefinition
  | PredicateDefinition
  | Theorem
  | Lemma
  | Corollary

public structure Decl where
  kind : DeclKind
  component : Arg
  entry : Option (Arg)
  policy : Policy
  sentence : Sentence
  proof : Option (Proof)
  opening : Nat
  stop : Nat

public inductive Block where
  | BlockNil
  | BlockCons (_ : Block) (_ : Block)
  | BlockSection (_ : Arg) (_ : Nat) (_ : Nat) (_ : Option ((Prod (Nat) (Nat)))) (_ : Block) (_ : Nat)
  | BlockDecl (_ : Decl)

public structure DataForm where
  data : Arg
  surface : Bool
  opening : Nat

public structure ModuleAst where
  rows : List (Row)
  name : ByteArray
  nameArg : Arg
  uses : List (Arg)
  imports : List (Arg)
  titleStart : Nat
  titleStop : Nat
  blocks : Block
  core : Option (DataForm)
  semantic : Option (DataForm)
  tooling : Option (DataForm)

public structure Ctx where
  path : ByteArray
  atoms : List (LexLeanBoot.PrimitiveScan.Atom)
  count : Nat
  maxDepth : Nat

public structure Ps where
  pos : Nat
  rows : List (Row)
  error : Option (LexLeanBoot.PrimitiveScan.Rejection)

public structure ArgAt where
  arg : Arg
  ps : Ps

public structure EnvAt where
  opening : Nat
  name : ByteArray
  ps : Ps

public structure SentenceAt where
  sentence : Sentence
  ps : Ps

public structure PolicyAt where
  policy : Policy
  ps : Ps

public structure ItemAt where
  item : Proof
  ps : Ps

public structure Quoting where
  index : Nat
  inString : Bool
  escaped : Bool

public structure ArgScan where
  pos : Nat
  depth : Nat
  pieces : List (ByteArray)
  inString : Bool
  escaped : Bool
  closing : Option (LexLeanBoot.PrimitiveScan.Atom)

public structure SentenceScan where
  pos : Nat
  depth : Nat
  island : Nat
  status : Nat

public structure LabelScan where
  index : Nat
  value : Nat
  valid : Bool

public structure BlocksCtx where
  terminator : ByteArray
  depth : Nat
  acc : List (Block)

public structure ProofCtx where
  opening : Nat
  envName : ByteArray
  depth : Nat
  items : List (Proof)

public structure SectionFrame where
  outer : BlocksCtx
  component : Arg
  headingStart : Nat
  headingStop : Nat
  params : Option ((Prod (Nat) (Nat)))
  opening : Nat

public structure DeclFrame where
  outer : BlocksCtx
  env : ByteArray
  kind : DeclKind
  component : Arg
  entry : Option (Arg)
  policy : Policy
  sentence : Sentence
  opening : Nat

public structure CaseRow where
  entry : Arg
  binds : List (ByteArray)
  bindArg : Arg
  proof : Proof

public structure LabeledHeader where
  outer : ProofCtx
  word : ByteArray
  envName : ByteArray
  function : Option (Arg)
  acc : List ((Prod (Nat) (Proof)))
  opening : Nat

public structure CasesHeader where
  outer : ProofCtx
  induction : Bool
  envName : ByteArray
  scrutinee : Arg
  acc : List (CaseRow)
  opening : Nat

public inductive ProofFrame where
  | FrameHave (_ : ProofCtx) (_ : Arg) (_ : Sentence) (_ : Nat)
  | FrameLabeled (_ : LabeledHeader) (_ : Nat)
  | FrameCase (_ : CasesHeader) (_ : Arg) (_ : List (ByteArray)) (_ : Arg)

public inductive Mode where
  | InBlocks (_ : BlocksCtx)
  | InProof (_ : ProofCtx) (_ : DeclFrame) (_ : List (ProofFrame))
  | Finished (_ : Block)

public structure Machine where
  ps : Ps
  sections : List (SectionFrame)
  mode : Mode

@[expose] public def lit (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def cat (left : ByteArray) (right : ByteArray) : ByteArray := (LexLeanRuntime.append (left) (right) : ByteArray)

@[expose] public def same (left : ByteArray) (right : ByteArray) : Bool := (LexLeanRuntime.equal (left) (right) : Bool)

@[expose] public def ticked (name : ByteArray) : ByteArray := cat (cat (lit ("`")) (name)) (lit ("`"))

@[expose] public def atomAt (ctx : Ctx) (position : Nat) : Option (LexLeanBoot.PrimitiveScan.Atom) := (LexLeanRuntime.index ((ctx).atoms) (position) : Option (LexLeanBoot.PrimitiveScan.Atom))

@[expose] public def isAtom (atom : LexLeanBoot.PrimitiveScan.Atom) (code : Nat) (spelled : ByteArray) : Bool := (if (Nat.beq (LexLeanBoot.PrimitiveScan.classCode (atom)) (code)) then same ((atom).text) (spelled) else false)

@[expose] public def atomIs (ctx : Ctx) (position : Nat) (code : Nat) (spelled : ByteArray) : Bool := (match atomAt (ctx) (position) with | Option.some atom => isAtom (atom) (code) (spelled) | Option.none => false)

@[expose] public def skipStep (ctx : Ctx) (position : Nat) : Option (Nat) := (match atomAt (ctx) (position) with | Option.some atom => (if (Nat.beq (LexLeanBoot.PrimitiveScan.classCode (atom)) (6)) then Option.some ((position + 1)) else Option.none) | Option.none => Option.none)

@[expose] public def skipWs (ctx : Ctx) (start : Nat) : Nat := ((LexLeanCollections.iterateUntil ((fun (position : Nat) => skipStep (ctx) (position))) ((ctx).count) (start) : (Prod (Nat) (Bool)))).1

@[expose] public def moved (ps : Ps) (position : Nat) : Ps := ({ pos := position, rows := (ps).rows, error := (ps).error } : Ps)

@[expose] public def wholeFile (path : ByteArray) : LexLeanBoot.PrimitiveScan.Place := ({ path := path, byteStart := 0, byteEnd := 0, lineStart := 1, columnStart := 1, lineEnd := 1, columnEnd := 1 } : LexLeanBoot.PrimitiveScan.Place)

@[expose] public def placeHere (ctx : Ctx) (position : Nat) : LexLeanBoot.PrimitiveScan.Place := (let last : Nat := (LexLeanRuntime.subtract ((ctx).count) (1) : Nat); (match atomAt (ctx) ((if (Nat.blt (position) (last)) then position else last)) with | Option.some atom => LexLeanBoot.PrimitiveScan.placeOfAtom ((ctx).path) (atom) | Option.none => wholeFile ((ctx).path)))

@[expose] public def fail (ctx : Ctx) (ps : Ps) (code : ByteArray) (message : ByteArray) : Ps := (match (ps).error with | Option.some _ => ps | Option.none => (let position : Nat := skipWs (ctx) ((ps).pos); ({ pos := position, rows := (ps).rows, error := Option.some (({ code := code, message := message, place := Option.some (placeHere (ctx) (position)), help := Option.none } : LexLeanBoot.PrimitiveScan.Rejection)) } : Ps)))

@[expose] public def refuse (ctx : Ctx) (ps : Ps) (message : ByteArray) : Ps := fail (ctx) (ps) (lit ("LLP2003")) (message)

@[expose] public def structural (entry : ByteArray) : Origin := Origin.Structural (lit ("lexlean.core")) (entry)

@[expose] public def metadata (owner : ByteArray) : Origin := Origin.Metadata (cat (lit ("lexlean.core::")) (owner))

@[expose] public def rowOf (ctx : Ctx) (atom : LexLeanBoot.PrimitiveScan.Atom) (binding : Origin) : Row := ({ path := (ctx).path, byteStart := (atom).byteStart, byteEnd := (atom).byteEnd, atomClass := (atom).kind, binding := binding } : Row)

@[expose] public def cover (ctx : Ctx) (ps : Ps) (atom : LexLeanBoot.PrimitiveScan.Atom) (binding : Origin) : Ps := ({ pos := (ps).pos, rows := (rowOf (ctx) (atom) (binding) :: (ps).rows), error := (ps).error } : Ps)

@[expose] public def foundText (expected : ByteArray) (found : ByteArray) : ByteArray := cat (cat (cat (lit ("expected ")) (ticked (expected))) (lit (", found "))) (ticked (found))

@[expose] public def foundEnd (expected : ByteArray) : ByteArray := cat (cat (lit ("expected ")) (ticked (expected))) (lit (", found end of file"))

@[expose] public def expectAtom (ctx : Ctx) (ps : Ps) (code : Nat) (spelled : ByteArray) (entry : ByteArray) : Ps := (let position : Nat := skipWs (ctx) ((ps).pos); (match atomAt (ctx) (position) with | Option.some atom => (if isAtom (atom) (code) (spelled) then cover (ctx) (moved (ps) ((position + 1))) (atom) (structural (entry)) else refuse (ctx) (moved (ps) (position)) (foundText (spelled) ((atom).text))) | Option.none => refuse (ctx) (moved (ps) (position)) (foundEnd (spelled))))

@[expose] public def expectControl (ctx : Ctx) (ps : Ps) (name : ByteArray) : Ps := expectAtom (ctx) (ps) (0) (name) (LexLeanStd.Text.sliceOr (name) (1) ((LexLeanRuntime.subtract ((LexLeanRuntime.length (name) : Nat)) (1) : Nat)))

@[expose] public def quoteStep (subject : ByteArray) (state : Quoting) : Option (Quoting) := (match LexLeanStd.Text.byteAt (subject) ((state).index) with | Option.some byte => (let next : Nat := ((state).index + 1); (let inside : Bool := (state).inString; (if (state).escaped then Option.some (({ index := next, inString := inside, escaped := false } : Quoting)) else (if (if inside then (LexLeanRuntime.equal (byte) ((92 : UInt8)) : Bool) else false) then Option.some (({ index := next, inString := inside, escaped := true } : Quoting)) else (if (LexLeanRuntime.equal (byte) ((34 : UInt8)) : Bool) then Option.some (({ index := next, inString := (!inside), escaped := false } : Quoting)) else Option.some (({ index := next, inString := inside, escaped := false } : Quoting))))))) | Option.none => Option.none)

@[expose] public def quoteRun (subject : ByteArray) (inside : Bool) (escaped : Bool) : Quoting := ((LexLeanCollections.iterateUntil ((fun (state : Quoting) => quoteStep (subject) (state))) ((LexLeanRuntime.length (subject) : Nat)) (({ index := 0, inString := inside, escaped := escaped } : Quoting)) : (Prod (Quoting) (Bool)))).1

@[expose] public def argNext (keep : Bool) (scan : ArgScan) (atom : LexLeanBoot.PrimitiveScan.Atom) (depth : Nat) (pieces : List (ByteArray)) : Option (ArgScan) := (let quoting : Quoting := (if keep then quoteRun ((atom).text) ((scan).inString) ((scan).escaped) else ({ index := 0, inString := (scan).inString, escaped := (scan).escaped } : Quoting)); Option.some (({ pos := ((scan).pos + 1), depth := depth, pieces := pieces, inString := (quoting).inString, escaped := (quoting).escaped, closing := Option.none } : ArgScan)))

@[expose] public def argStep (ctx : Ctx) (keep : Bool) (scan : ArgScan) : Option (ArgScan) := (match (scan).closing with | Option.some _ => Option.none | Option.none => (match atomAt (ctx) ((scan).pos) with | Option.none => Option.none | Option.some atom => (let quoted : Bool := (scan).inString; (let depth : Nat := (scan).depth; (let pieces : List (ByteArray) := (scan).pieces; (if (if quoted then false else isAtom (atom) (3) (lit ("{"))) then argNext (keep) (scan) (atom) ((depth + 1)) ((lit ("{") :: pieces)) else (if (if quoted then false else isAtom (atom) (3) (lit ("}"))) then (if (Nat.beq (depth) (0)) then Option.some (({ pos := (scan).pos, depth := depth, pieces := pieces, inString := quoted, escaped := (scan).escaped, closing := Option.some (atom) } : ArgScan)) else argNext (keep) (scan) (atom) ((LexLeanRuntime.subtract (depth) (1) : Nat)) ((lit ("}") :: pieces))) else (if (if quoted then false else (Nat.beq (LexLeanBoot.PrimitiveScan.classCode (atom)) (6))) then argNext (keep) (scan) (atom) (depth) (pieces) else argNext (keep) (scan) (atom) (depth) (((atom).text :: pieces))))))))))

@[expose] public def joined (pieces : List (ByteArray)) : ByteArray := LexLeanStd.Buffer.joinAll (LexLeanStd.Text.reverse (pieces))

@[expose] public def braceWith (ctx : Ctx) (ps : Ps) (keep : Bool) : ArgAt := (let opened : Ps := expectAtom (ctx) (ps) (3) (lit ("{")) (lit ("brace-open")); (let start : Nat := (opened).pos; (let scan : ArgScan := ((LexLeanCollections.iterateUntil ((fun (state : ArgScan) => argStep (ctx) (keep) (state))) (((ctx).count + 1)) (({ pos := start, depth := 0, pieces := ([] : List (ByteArray)), inString := false, escaped := false, closing := Option.none } : ArgScan)) : (Prod (ArgScan) (Bool)))).1; (let stop : Nat := (scan).pos; (match (scan).closing with | Option.some atom => ({ arg := ({ start := start, stop := stop, text := joined ((scan).pieces) } : Arg), ps := cover (ctx) (moved (opened) ((stop + 1))) (atom) (structural (lit ("brace-close"))) } : ArgAt) | Option.none => ({ arg := ({ start := start, stop := stop, text := _root_.ByteArray.mk #[] } : Arg), ps := refuse (ctx) (moved (opened) (stop)) (lit ("unclosed `{` argument")) } : ArgAt))))))

@[expose] public def brace (ctx : Ctx) (ps : Ps) : ArgAt := braceWith (ctx) (ps) (false)

@[expose] public def verbatimStep (ctx : Ctx) (state : (Prod (Nat) (List (ByteArray)))) : Option ((Prod (Nat) (List (ByteArray)))) := (match atomAt (ctx) ((state).1) with | Option.some atom => (if isAtom (atom) (0) (lit ("\\end")) then Option.none else Option.some ((((state).1 + 1), ((atom).text :: (state).2)))) | Option.none => Option.none)

@[expose] public def verbatimBody (ctx : Ctx) (ps : Ps) : ArgAt := (let start : Nat := (ps).pos; (let final : (Prod (Nat) (List (ByteArray))) := ((LexLeanCollections.iterateUntil ((fun (state : (Prod (Nat) (List (ByteArray)))) => verbatimStep (ctx) (state))) (((ctx).count + 1)) ((start, ([] : List (ByteArray)))) : (Prod ((Prod (Nat) (List (ByteArray)))) (Bool)))).1; (let stop : Nat := (final).1; ({ arg := ({ start := start, stop := stop, text := joined ((final).2) } : Arg), ps := (match atomAt (ctx) (stop) with | Option.some _ => moved (ps) (stop) | Option.none => refuse (ctx) (moved (ps) (stop)) (lit ("unclosed environment"))) } : ArgAt))))

@[expose] public def coverStep (ctx : Ctx) (binding : Origin) (stop : Nat) (state : (Prod (Nat) (List (Row)))) : Option ((Prod (Nat) (List (Row)))) := (if (Nat.blt ((state).1) (stop)) then (match atomAt (ctx) ((state).1) with | Option.some atom => (if (Nat.beq (LexLeanBoot.PrimitiveScan.classCode (atom)) (6)) then Option.some ((((state).1 + 1), (state).2)) else Option.some ((((state).1 + 1), (rowOf (ctx) (atom) (binding) :: (state).2)))) | Option.none => Option.none) else Option.none)

@[expose] public def coverRange (ctx : Ctx) (ps : Ps) (range : Arg) (binding : Origin) : Ps := (let stop : Nat := (range).stop; ({ pos := (ps).pos, rows := (((LexLeanCollections.iterateUntil ((fun (state : (Prod (Nat) (List (Row)))) => coverStep (ctx) (binding) (stop) (state))) ((LexLeanRuntime.subtract (stop) ((range).start) : Nat)) (((range).start, (ps).rows)) : (Prod ((Prod (Nat) (List (Row)))) (Bool)))).1).2, error := (ps).error } : Ps))

@[expose] public def coverMetadata (ctx : Ctx) (ps : Ps) (range : Arg) (owner : ByteArray) : Ps := coverRange (ctx) (ps) (range) (metadata (owner))

@[expose] public def coverPayload (ctx : Ctx) (ps : Ps) (range : Arg) (owner : ByteArray) : Ps := (match atomAt (ctx) ((range).start) with | Option.none => ps | Option.some head => (let last : LexLeanBoot.PrimitiveScan.Atom := (match atomAt (ctx) ((LexLeanRuntime.subtract ((range).stop) (1) : Nat)) with | Option.some found => found | Option.none => head); ({ pos := (ps).pos, rows := (({ path := (ctx).path, byteStart := (head).byteStart, byteEnd := (last).byteEnd, atomClass := (head).kind, binding := metadata (owner) } : Row) :: (ps).rows), error := (ps).error } : Ps)))

@[expose] public def envOrigin (name : ByteArray) : Origin := structural (cat (lit ("env.")) (name))

@[expose] public def beginEnv (ctx : Ctx) (ps : Ps) : EnvAt := (let began : Ps := expectControl (ctx) (ps) (lit ("\\begin")); (let name : ArgAt := brace (ctx) (began); ({ opening := (LexLeanRuntime.subtract ((began).pos) (1) : Nat), name := ((name).arg).text, ps := coverRange (ctx) ((name).ps) ((name).arg) (envOrigin (((name).arg).text)) } : EnvAt)))

@[expose] public def expectEnd (ctx : Ctx) (ps : Ps) (name : ByteArray) : Ps := (let ended : Ps := expectControl (ctx) (ps) (lit ("\\end")); (let closing : ArgAt := brace (ctx) (ended); (if same (((closing).arg).text) (name) then coverRange (ctx) ((closing).ps) ((closing).arg) (envOrigin (name)) else refuse (ctx) ((closing).ps) (cat (cat (cat (cat (lit ("expected `\\end{")) (name)) (lit ("}`, found `\\end{"))) (((closing).arg).text)) (lit ("}`"))))))

@[expose] public def atControl (ctx : Ctx) (position : Nat) (name : ByteArray) : Bool := atomIs (ctx) (skipWs (ctx) (position)) (0) (name)

@[expose] public def atBeginOf (ctx : Ctx) (position : Nat) (name : ByteArray) : Bool := (if atControl (ctx) (position) (lit ("\\begin")) then (let «open» : Nat := skipWs (ctx) ((skipWs (ctx) (position) + 1)); (if atomIs (ctx) («open») (3) (lit ("{")) then (match atomAt (ctx) ((«open» + 1)) with | Option.some word => (if same ((word).text) (name) then atomIs (ctx) ((«open» + 2)) (3) (lit ("}")) else false) | Option.none => false) else false)) else false)

@[expose] public def sentenceMove (scan : SentenceScan) (depth : Nat) (island : Nat) : Option (SentenceScan) := Option.some (({ pos := ((scan).pos + 1), depth := depth, island := island, status := 0 } : SentenceScan))

@[expose] public def sentenceStop (scan : SentenceScan) (status : Nat) : Option (SentenceScan) := Option.some (({ pos := (scan).pos, depth := (scan).depth, island := (scan).island, status := status } : SentenceScan))

@[expose] public def sentenceControl (scan : SentenceScan) (spelled : ByteArray) : Option (SentenceScan) := (let depth : Nat := (scan).depth; (let island : Nat := (scan).island; (if (Nat.beq (island) (0)) then (if same (spelled) (lit ("\\(")) then sentenceMove (scan) (depth) (1) else (if same (spelled) (lit ("\\[")) then sentenceMove (scan) (depth) (2) else (if (if (Nat.beq (depth) (0)) then (if same (spelled) (lit ("\\begin")) then true else same (spelled) (lit ("\\end"))) else false) then sentenceStop (scan) (2) else sentenceMove (scan) (depth) (0)))) else (if (if (Nat.beq (island) (1)) then same (spelled) (lit ("\\)")) else same (spelled) (lit ("\\]"))) then sentenceMove (scan) (depth) (0) else sentenceMove (scan) (depth) (island)))))

@[expose] public def sentenceStep (ctx : Ctx) (scan : SentenceScan) : Option (SentenceScan) := (if (Nat.beq ((scan).status) (0)) then (match atomAt (ctx) ((scan).pos) with | Option.none => sentenceStop (scan) (2) | Option.some atom => (let code : Nat := LexLeanBoot.PrimitiveScan.classCode (atom); (let depth : Nat := (scan).depth; (let island : Nat := (scan).island; (if (Nat.beq (code) (0)) then sentenceControl (scan) ((atom).text) else (if (Nat.beq (code) (3)) then (if (Nat.beq (island) (0)) then (if same ((atom).text) (lit ("{")) then sentenceMove (scan) ((depth + 1)) (island) else (if same ((atom).text) (lit ("}")) then sentenceMove (scan) ((LexLeanRuntime.subtract (depth) (1) : Nat)) (island) else sentenceMove (scan) (depth) (island))) else sentenceMove (scan) (depth) (island)) else (if (if (Nat.beq (code) (4)) then (if (Nat.beq (island) (0)) then (if (Nat.beq (depth) (0)) then same ((atom).text) (lit (".")) else false) else false) else false) then sentenceStop (scan) (1) else sentenceMove (scan) (depth) (island)))))))) else Option.none)

@[expose] public def sentence (ctx : Ctx) (ps : Ps) : SentenceAt := (let start : Nat := skipWs (ctx) ((ps).pos); (let scan : SentenceScan := ((LexLeanCollections.iterateUntil ((fun (state : SentenceScan) => sentenceStep (ctx) (state))) (((ctx).count + 1)) (({ pos := start, depth := 0, island := 0, status := 0 } : SentenceScan)) : (Prod (SentenceScan) (Bool)))).1; (let stop : Nat := (scan).pos; (if (Nat.beq ((scan).status) (1)) then ({ sentence := ({ start := start, period := stop } : Sentence), ps := moved (ps) ((stop + 1)) } : SentenceAt) else ({ sentence := ({ start := start, period := stop } : Sentence), ps := fail (ctx) (moved (ps) (stop)) (lit ("LLP2001")) (lit ("a sentence must end with a period")) } : SentenceAt)))))

@[expose] public def componentByte (byte : UInt8) : Bool := (if LexLeanBoot.PrimitiveScan.inRange (byte) ((97 : UInt8)) ((122 : UInt8)) then true else (if LexLeanBoot.PrimitiveScan.isAsciiDigit (byte) then true else (LexLeanRuntime.equal (byte) ((45 : UInt8)) : Bool)))

@[expose] public def componentStep (text : ByteArray) (position : Nat) : Option (Nat) := (match LexLeanStd.Text.byteAt (text) (position) with | Option.some byte => (if componentByte (byte) then Option.some ((position + 1)) else Option.none) | Option.none => Option.none)

@[expose] public def isComponentId (text : ByteArray) : Bool := (match LexLeanStd.Text.byteAt (text) (0) with | Option.some head => (if LexLeanBoot.PrimitiveScan.inRange (head) ((97 : UInt8)) ((122 : UInt8)) then (Nat.beq (((LexLeanCollections.iterateUntil ((fun (position : Nat) => componentStep (text) (position))) ((LexLeanRuntime.length (text) : Nat)) (1) : (Prod (Nat) (Bool)))).1) ((LexLeanRuntime.length (text) : Nat))) else false) | Option.none => false)

@[expose] public def leanStep (text : ByteArray) (state : (Prod (Nat) (Bool))) : Option ((Prod (Nat) (Bool))) := (match LexLeanStd.Text.byteAt (text) ((state).1) with | Option.some byte => (let next : Nat := ((state).1 + 1); (let startish : Bool := (if LexLeanBoot.PrimitiveScan.isAsciiAlpha (byte) then true else (LexLeanRuntime.equal (byte) ((95 : UInt8)) : Bool)); (if (state).2 then (if startish then Option.some ((next, false)) else Option.none) else (if (LexLeanRuntime.equal (byte) ((46 : UInt8)) : Bool) then Option.some ((next, true)) else (if (if startish then true else LexLeanBoot.PrimitiveScan.isAsciiDigit (byte)) then Option.some ((next, false)) else Option.none))))) | Option.none => Option.none)

@[expose] public def isLeanName (text : ByteArray) : Bool := (let final : (Prod (Nat) (Bool)) := ((LexLeanCollections.iterateUntil ((fun (state : (Prod (Nat) (Bool))) => leanStep (text) (state))) ((LexLeanRuntime.length (text) : Nat)) ((0, true)) : (Prod ((Prod (Nat) (Bool))) (Bool)))).1; (if (Nat.beq ((final).1) ((LexLeanRuntime.length (text) : Nat))) then (!(final).2) else false))

@[expose] public def before (left : ByteArray) (right : ByteArray) : Bool := (LexLeanRuntime.equal ((LexLeanRuntime.compareBytes (left) (right) : Ordering)) ((LexLeanRuntime.compareBytes (_root_.ByteArray.mk #[]) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 1))]) : Ordering)) : Bool)

@[expose] public def insertStep (state : (Prod (List (ByteArray)) (Option (ByteArray)))) (item : ByteArray) : (Prod (List (ByteArray)) (Option (ByteArray))) := (match (state).2 with | Option.some pending => (if before (pending) (item) then ((item :: (pending :: (state).1)), Option.none) else ((item :: (state).1), Option.some (pending))) | Option.none => ((item :: (state).1), Option.none))

@[expose] public def insertName (sorted : List (ByteArray)) (name : ByteArray) : List (ByteArray) := (let folded : (Prod (List (ByteArray)) (Option (ByteArray))) := (LexLeanCollections.listFold ((fun (state : (Prod (List (ByteArray)) (Option (ByteArray)))) (item : ByteArray) => insertStep (state) (item))) ((([] : List (ByteArray)), Option.some (name))) (sorted) : (Prod (List (ByteArray)) (Option (ByteArray)))); LexLeanStd.Text.reverse ((match (folded).2 with | Option.some pending => (pending :: (folded).1) | Option.none => (folded).1)))

@[expose] public def nameStep (state : (Prod (List (ByteArray)) (Option (ByteArray)))) (piece : ByteArray) : (Prod (List (ByteArray)) (Option (ByteArray))) := (match (state).2 with | Option.some _ => state | Option.none => (if (Nat.beq ((LexLeanRuntime.length (piece) : Nat)) (0)) then ((state).1, Option.some (lit ("empty entry in an axiom name list"))) else (if isLeanName (piece) then (if LexLeanBoot.PrimitiveScan.bytesInList (piece) ((state).1) then ((state).1, Option.some (cat (lit ("duplicate axiom name ")) (ticked (piece)))) else ((piece :: (state).1), Option.none)) else ((state).1, Option.some (cat (ticked (piece)) (lit (" is not a conservative ASCII Lean name")))))))

@[expose] public def policyNamed (ctx : Ctx) (ps : Ps) (kind : PolicyKind) (controlName : ByteArray) (owner : ByteArray) : PolicyAt := (let controlled : Ps := expectControl (ctx) (ps) (controlName); (let control : Nat := (LexLeanRuntime.subtract ((controlled).pos) (1) : Nat); (let listed : ArgAt := brace (ctx) (controlled); (let covered : Ps := coverMetadata (ctx) ((listed).ps) ((listed).arg) (owner); (let folded : (Prod (List (ByteArray)) (Option (ByteArray))) := (LexLeanCollections.listFold ((fun (state : (Prod (List (ByteArray)) (Option (ByteArray)))) (piece : ByteArray) => nameStep (state) (piece))) ((([] : List (ByteArray)), Option.none)) (LexLeanStd.Text.splitOn (((listed).arg).text) ((59 : UInt8))) : (Prod (List (ByteArray)) (Option (ByteArray)))); ({ policy := ({ kind := kind, names := (LexLeanCollections.listFold ((fun (sorted : List (ByteArray)) (name : ByteArray) => insertName (sorted) (name))) (([] : List (ByteArray))) ((folded).1) : List (ByteArray)), control := control, namesArg := Option.some ((listed).arg) } : Policy), ps := (match (folded).2 with | Option.some message => refuse (ctx) (covered) (message) | Option.none => covered) } : PolicyAt))))))

@[expose] public def policyOf (ctx : Ctx) (ps : Ps) : PolicyAt := (let position : Nat := (ps).pos; (if atControl (ctx) (position) (lit ("\\noaxioms")) then (let controlled : Ps := expectControl (ctx) (ps) (lit ("\\noaxioms")); ({ policy := ({ kind := PolicyKind.PolicyNone, names := ([] : List (ByteArray)), control := (LexLeanRuntime.subtract ((controlled).pos) (1) : Nat), namesArg := Option.none } : Policy), ps := controlled } : PolicyAt)) else (if atControl (ctx) (position) (lit ("\\allowaxioms")) then policyNamed (ctx) (ps) (PolicyKind.PolicyAllow) (lit ("\\allowaxioms")) (lit ("allowaxioms")) else (if atControl (ctx) (position) (lit ("\\exactaxioms")) then policyNamed (ctx) (ps) (PolicyKind.PolicyExact) (lit ("\\exactaxioms")) (lit ("exactaxioms")) else ({ policy := ({ kind := PolicyKind.PolicyNone, names := ([] : List (ByteArray)), control := 0, namesArg := Option.none } : Policy), ps := refuse (ctx) (ps) (lit ("every definition and theorem-like declaration requires exactly one axiom policy")) } : PolicyAt)))))

@[expose] public def labelStep (subject : ByteArray) (scan : LabelScan) : Option (LabelScan) := (if (scan).valid then (match LexLeanStd.Text.byteAt (subject) ((scan).index) with | Option.some byte => (let next : Nat := ((scan).index + 1); (if LexLeanBoot.PrimitiveScan.isAsciiDigit (byte) then (let digit : Nat := LexLeanStd.Numeral.decimalValue (byte); (if (Nat.ble ((scan).value) ((LexLeanRuntime.quotient ((LexLeanRuntime.subtract (18446744073709551615) (digit) : Nat)) (10) (0) : Nat))) then Option.some (({ index := next, value := ((LexLeanRuntime.multiply ((scan).value) (10) : Nat) + digit), valid := true } : LabelScan)) else Option.some (({ index := next, value := 0, valid := false } : LabelScan)))) else Option.some (({ index := next, value := 0, valid := false } : LabelScan)))) | Option.none => Option.none) else Option.none)

@[expose] public def parseLabel (text : ByteArray) : Option (Nat) := (let start : Nat := (if LexLeanBoot.PrimitiveScan.byteIs (text) (0) ((43 : UInt8)) then 1 else 0); (if (Nat.blt (start) ((LexLeanRuntime.length (text) : Nat))) then (let final : LabelScan := ((LexLeanCollections.iterateUntil ((fun (state : LabelScan) => labelStep (text) (state))) ((LexLeanRuntime.length (text) : Nat)) (({ index := start, value := 0, valid := true } : LabelScan)) : (Prod (LabelScan) (Bool)))).1; (if (final).valid then Option.some ((final).value) else Option.none)) else Option.none))

@[expose] public def chainItems (items : List (Proof)) : Proof := (LexLeanCollections.listFold ((fun (chain : Proof) (item : Proof) => Proof.ProofCons (item) (chain))) (Proof.ProofNil) (items) : Proof)

@[expose] public def chainLabeled (items : List ((Prod (Nat) (Proof)))) : Proof := (LexLeanCollections.listFold ((fun (chain : Proof) (item : (Prod (Nat) (Proof))) => Proof.LabeledCons ((item).1) ((item).2) (chain))) (Proof.ProofNil) (items) : Proof)

@[expose] public def chainCases (items : List (CaseRow)) : Proof := (LexLeanCollections.listFold ((fun (chain : Proof) (item : CaseRow) => Proof.CaseCons ((item).entry) ((item).binds) ((item).bindArg) ((item).proof) (chain))) (Proof.ProofNil) (items) : Proof)

@[expose] public def chainBlocks (items : List (Block)) : Block := (LexLeanCollections.listFold ((fun (chain : Block) (item : Block) => Block.BlockCons (item) (chain))) (Block.BlockNil) (items) : Block)

@[expose] public def pushItem (current : ProofCtx) (item : Proof) : ProofCtx := ({ opening := (current).opening, envName := (current).envName, depth := (current).depth, items := (item :: (current).items) } : ProofCtx)

@[expose] public def pushBlock (current : BlocksCtx) (item : Block) : BlocksCtx := ({ terminator := (current).terminator, depth := (current).depth, acc := (item :: (current).acc) } : BlocksCtx)

@[expose] public def rewriteStep (ctx : Ctx) (state : (Prod (Ps) (List ((Prod (Bool) (Arg)))))) : Option ((Prod (Ps) (List ((Prod (Bool) (Arg)))))) := (let ps : Ps := (state).1; (match (ps).error with | Option.some _ => Option.none | Option.none => (if atControl (ctx) ((ps).pos) (lit ("\\forward")) then (let rule : ArgAt := brace (ctx) (expectControl (ctx) (ps) (lit ("\\forward"))); Option.some (((rule).ps, ((false, (rule).arg) :: (state).2)))) else (if atControl (ctx) ((ps).pos) (lit ("\\backward")) then (let rule : ArgAt := brace (ctx) (expectControl (ctx) (ps) (lit ("\\backward"))); Option.some (((rule).ps, ((true, (rule).arg) :: (state).2)))) else Option.none))))

@[expose] public def rewriteItem (ctx : Ctx) (ps : Ps) (opening : Nat) : ItemAt := (let target : ArgAt := brace (ctx) (ps); (let covered : Ps := coverMetadata (ctx) ((target).ps) ((target).arg) (lit ("env.rewrite")); (let final : (Prod (Ps) (List ((Prod (Bool) (Arg))))) := ((LexLeanCollections.iterateUntil ((fun (state : (Prod (Ps) (List ((Prod (Bool) (Arg)))))) => rewriteStep (ctx) (state))) (((ctx).count + 1)) ((covered, ([] : List ((Prod (Bool) (Arg)))))) : (Prod ((Prod (Ps) (List ((Prod (Bool) (Arg)))))) (Bool)))).1; (let ended : Ps := expectEnd (ctx) ((final).1) (lit ("rewrite")); ({ item := Proof.ProofRewrite ((target).arg) (LexLeanBoot.PrimitiveScan.reversed ((Prod (Bool) (Arg))) ((final).2)) (opening), ps := (if (Nat.beq ((LexLeanRuntime.length ((final).2) : Nat)) (0)) then fail (ctx) (ended) (lit ("LLF5003")) (lit ("a rewrite requires at least one rule")) else ended) } : ItemAt)))))

@[expose] public def simplifyStep (ctx : Ctx) (state : (Prod (Ps) (List (Arg)))) : Option ((Prod (Ps) (List (Arg)))) := (let ps : Ps := (state).1; (match (ps).error with | Option.some _ => Option.none | Option.none => (if atControl (ctx) ((ps).pos) (lit ("\\rule")) then (let rule : ArgAt := brace (ctx) (expectControl (ctx) (ps) (lit ("\\rule"))); Option.some (((rule).ps, ((rule).arg :: (state).2)))) else Option.none)))

@[expose] public def simplifyItem (ctx : Ctx) (ps : Ps) (opening : Nat) : ItemAt := (let target : ArgAt := brace (ctx) (ps); (let covered : Ps := coverMetadata (ctx) ((target).ps) ((target).arg) (lit ("env.simplify")); (let final : (Prod (Ps) (List (Arg))) := ((LexLeanCollections.iterateUntil ((fun (state : (Prod (Ps) (List (Arg)))) => simplifyStep (ctx) (state))) (((ctx).count + 1)) ((covered, ([] : List (Arg)))) : (Prod ((Prod (Ps) (List (Arg)))) (Bool)))).1; (let ended : Ps := expectEnd (ctx) ((final).1) (lit ("simplify")); ({ item := Proof.ProofSimplify ((target).arg) (LexLeanBoot.PrimitiveScan.reversed (Arg) ((final).2)) (opening), ps := (if (Nat.beq ((LexLeanRuntime.length ((final).2) : Nat)) (0)) then fail (ctx) (ended) (lit ("LLF5003")) (lit ("a simplify requires at least one rule")) else ended) } : ItemAt)))))

@[expose] public def calcStep (ctx : Ctx) (state : (Prod (Ps) (List (CalcStep)))) : Option ((Prod (Ps) (List (CalcStep)))) := (let ps : Ps := (state).1; (match (ps).error with | Option.some _ => Option.none | Option.none => (if atControl (ctx) ((ps).pos) (lit ("\\step")) then (let relation : ArgAt := brace (ctx) (expectControl (ctx) (ps) (lit ("\\step"))); (let term : ArgAt := brace (ctx) (coverMetadata (ctx) ((relation).ps) ((relation).arg) (lit ("step"))); (let proof : ArgAt := brace (ctx) ((term).ps); Option.some (((proof).ps, (({ relation := (relation).arg, term := (term).arg, proof := (proof).arg } : CalcStep) :: (state).2)))))) else Option.none)))

@[expose] public def calculateItem (ctx : Ctx) (ps : Ps) (opening : Nat) : ItemAt := (let start : ArgAt := brace (ctx) (expectControl (ctx) (ps) (lit ("\\start"))); (let final : (Prod (Ps) (List (CalcStep))) := ((LexLeanCollections.iterateUntil ((fun (state : (Prod (Ps) (List (CalcStep)))) => calcStep (ctx) (state))) (((ctx).count + 1)) (((start).ps, ([] : List (CalcStep)))) : (Prod ((Prod (Ps) (List (CalcStep)))) (Bool)))).1; (let ended : Ps := expectEnd (ctx) ((final).1) (lit ("calculate")); ({ item := Proof.ProofCalculate ((start).arg) (LexLeanBoot.PrimitiveScan.reversed (CalcStep) ((final).2)) (opening), ps := (if (Nat.beq ((LexLeanRuntime.length ((final).2) : Nat)) (0)) then fail (ctx) (ended) (lit ("LLF5003")) (lit ("a calculation requires at least one step")) else ended) } : ItemAt))))

@[expose] public def nestingMessage (ctx : Ctx) (phase : ByteArray) (depth : Nat) : ByteArray := cat (cat (cat (cat (cat (lit ("max_scope_depth exceeded in phase parse (")) (phase)) (lit ("): configured "))) (LexLeanBoot.PrimitiveScan.decimal ((ctx).maxDepth))) (lit (", nesting depth "))) (LexLeanBoot.PrimitiveScan.decimal (depth))

@[expose] public def enterProof (ctx : Ctx) (ps : Ps) (sections : List (SectionFrame)) (opening : Nat) (envName : ByteArray) (depth : Nat) (declaration : DeclFrame) (frames : List (ProofFrame)) : Machine := ({ ps := (if (Nat.blt ((ctx).maxDepth) (depth)) then fail (ctx) (ps) (lit ("LLS8002")) (nestingMessage (ctx) (lit ("proof environment nesting")) (depth)) else ps), sections := sections, mode := Mode.InProof (({ opening := opening, envName := envName, depth := depth, items := ([] : List (Proof)) } : ProofCtx)) (declaration) (frames) } : Machine)

@[expose] public def finishDecl (ctx : Ctx) (ps : Ps) (sections : List (SectionFrame)) (frame : DeclFrame) (proof : Option (Proof)) : Machine := (let ended : Ps := expectEnd (ctx) (ps) ((frame).env); ({ ps := ended, sections := sections, mode := Mode.InBlocks (pushBlock ((frame).outer) (Block.BlockDecl (({ kind := (frame).kind, component := (frame).component, entry := (frame).entry, policy := (frame).policy, sentence := (frame).sentence, proof := proof, opening := (frame).opening, stop := (ended).pos } : Decl)))) } : Machine))

@[expose] public def labeledNext (ctx : Ctx) (ps : Ps) (sections : List (SectionFrame)) (declaration : DeclFrame) (frames : List (ProofFrame)) (header : LabeledHeader) : Machine := (let word : ByteArray := (header).word; (if atBeginOf (ctx) ((ps).pos) (word) then (let began : EnvAt := beginEnv (ctx) (ps); (let label : ArgAt := brace (ctx) ((began).ps); (let covered : Ps := coverMetadata (ctx) ((label).ps) ((label).arg) (cat (lit ("env.")) (word)); (match parseLabel (((label).arg).text) with | Option.some number => enterProof (ctx) (covered) (sections) ((began).opening) (word) ((((header).outer).depth + 1)) (declaration) ((ProofFrame.FrameLabeled (header) (number) :: frames)) | Option.none => ({ ps := refuse (ctx) (covered) (cat (ticked (((label).arg).text)) (lit (" is not a decimal label"))), sections := sections, mode := Mode.InProof ((header).outer) (declaration) (frames) } : Machine))))) else (let ended : Ps := expectEnd (ctx) (ps) ((header).envName); (let chain : Proof := chainLabeled ((header).acc); ({ ps := ended, sections := sections, mode := Mode.InProof (pushItem ((header).outer) ((match (header).function with | Option.some applied => Proof.ProofApply (applied) (chain) ((header).opening) | Option.none => Proof.ProofConstructor (chain) ((header).opening)))) (declaration) (frames) } : Machine)))))

@[expose] public def casesNext (ctx : Ctx) (ps : Ps) (sections : List (SectionFrame)) (declaration : DeclFrame) (frames : List (ProofFrame)) (header : CasesHeader) : Machine := (if atBeginOf (ctx) ((ps).pos) (lit ("case")) then (let began : EnvAt := beginEnv (ctx) (ps); (let entry : ArgAt := brace (ctx) ((began).ps); (let bind : ArgAt := brace (ctx) (expectControl (ctx) (coverMetadata (ctx) ((entry).ps) ((entry).arg) (lit ("env.case"))) (lit ("\\bind"))); (let covered : Ps := coverMetadata (ctx) ((bind).ps) ((bind).arg) (lit ("bind")); (let spelled : ByteArray := ((bind).arg).text; enterProof (ctx) (covered) (sections) ((began).opening) (lit ("case")) ((((header).outer).depth + 1)) (declaration) ((ProofFrame.FrameCase (header) ((entry).arg) ((if (Nat.beq ((LexLeanRuntime.length (spelled) : Nat)) (0)) then ([] : List (ByteArray)) else LexLeanStd.Text.splitOn (spelled) ((59 : UInt8)))) ((bind).arg) :: frames))))))) else (let ended : Ps := expectEnd (ctx) (ps) ((header).envName); ({ ps := ended, sections := sections, mode := Mode.InProof (pushItem ((header).outer) (Proof.ProofCases ((header).induction) ((header).scrutinee) (chainCases ((header).acc)) ((header).opening))) (declaration) (frames) } : Machine)))

@[expose] public def returnProof (ctx : Ctx) (ps : Ps) (sections : List (SectionFrame)) (declaration : DeclFrame) (frames : List (ProofFrame)) (env : Proof) : Machine := (match frames with | List.nil => finishDecl (ctx) (ps) (sections) (declaration) (Option.some (env)) | List.cons frame rest => (match frame with | ProofFrame.FrameHave outer name proposition opening => (let ended : Ps := expectEnd (ctx) (ps) (lit ("have")); ({ ps := ended, sections := sections, mode := Mode.InProof (pushItem (outer) (Proof.ProofHave (name) (proposition) (env) (opening))) (declaration) (rest) } : Machine)) | ProofFrame.FrameLabeled header number => labeledNext (ctx) (ps) (sections) (declaration) (rest) (({ outer := (header).outer, word := (header).word, envName := (header).envName, function := (header).function, acc := ((number, env) :: (header).acc), opening := (header).opening } : LabeledHeader)) | ProofFrame.FrameCase header entry binds bindArg => casesNext (ctx) (ps) (sections) (declaration) (rest) (({ outer := (header).outer, induction := (header).induction, envName := (header).envName, scrutinee := (header).scrutinee, acc := (({ entry := entry, binds := binds, bindArg := bindArg, proof := env } : CaseRow) :: (header).acc), opening := (header).opening } : CasesHeader))))

@[expose] public def leafItem (_ps : Ps) (sections : List (SectionFrame)) (current : ProofCtx) (declaration : DeclFrame) (frames : List (ProofFrame)) (leaf : ItemAt) : Machine := ({ ps := (leaf).ps, sections := sections, mode := Mode.InProof (pushItem (current) ((leaf).item)) (declaration) (frames) } : Machine)

@[expose] public def haveItem (ctx : Ctx) (ps : Ps) (sections : List (SectionFrame)) (current : ProofCtx) (declaration : DeclFrame) (frames : List (ProofFrame)) (opening : Nat) : Machine := (let hypothesis : ArgAt := brace (ctx) (ps); (let proposition : SentenceAt := sentence (ctx) (coverMetadata (ctx) ((hypothesis).ps) ((hypothesis).arg) (lit ("env.have"))); (let began : EnvAt := beginEnv (ctx) ((proposition).ps); (if same ((began).name) (lit ("proof")) then enterProof (ctx) ((began).ps) (sections) ((began).opening) (lit ("proof")) (((current).depth + 1)) (declaration) ((ProofFrame.FrameHave (current) ((hypothesis).arg) ((proposition).sentence) (opening) :: frames)) else ({ ps := refuse (ctx) ((began).ps) (lit ("a have establishes its proposition with a nested proof")), sections := sections, mode := Mode.InProof (current) (declaration) (frames) } : Machine)))))

@[expose] public def proofEnvItem (ctx : Ctx) (ps : Ps) (sections : List (SectionFrame)) (current : ProofCtx) (declaration : DeclFrame) (frames : List (ProofFrame)) : Machine := (let began : EnvAt := beginEnv (ctx) (ps); (let name : ByteArray := (began).name; (let opening : Nat := (began).opening; (let after : Ps := (began).ps; (if same (name) (lit ("have")) then haveItem (ctx) (after) (sections) (current) (declaration) (frames) (opening) else (if same (name) (lit ("rewrite")) then leafItem (after) (sections) (current) (declaration) (frames) (rewriteItem (ctx) (after) (opening)) else (if same (name) (lit ("simplify")) then leafItem (after) (sections) (current) (declaration) (frames) (simplifyItem (ctx) (after) (opening)) else (if same (name) (lit ("apply")) then (let function : ArgAt := brace (ctx) (after); labeledNext (ctx) ((function).ps) (sections) (declaration) (frames) (({ outer := current, word := lit ("premise"), envName := lit ("apply"), function := Option.some ((function).arg), acc := ([] : List ((Prod (Nat) (Proof)))), opening := opening } : LabeledHeader))) else (if same (name) (lit ("constructor")) then labeledNext (ctx) (after) (sections) (declaration) (frames) (({ outer := current, word := lit ("branch"), envName := lit ("constructor"), function := Option.none, acc := ([] : List ((Prod (Nat) (Proof)))), opening := opening } : LabeledHeader)) else (if (if same (name) (lit ("cases")) then true else same (name) (lit ("induction"))) then (let scrutinee : ArgAt := brace (ctx) (after); casesNext (ctx) ((scrutinee).ps) (sections) (declaration) (frames) (({ outer := current, induction := same (name) (lit ("induction")), envName := name, scrutinee := (scrutinee).arg, acc := ([] : List (CaseRow)), opening := opening } : CasesHeader))) else (if same (name) (lit ("calculate")) then leafItem (after) (sections) (current) (declaration) (frames) (calculateItem (ctx) (after) (opening)) else ({ ps := fail (ctx) (after) (lit ("LLL1004")) (cat (ticked (name)) (lit (" is not a proof environment"))), sections := sections, mode := Mode.InProof (current) (declaration) (frames) } : Machine))))))))))))

@[expose] public def proofStep (ctx : Ctx) (ps : Ps) (sections : List (SectionFrame)) (current : ProofCtx) (declaration : DeclFrame) (frames : List (ProofFrame)) : Machine := (let position : Nat := skipWs (ctx) ((ps).pos); (match atomAt (ctx) (position) with | Option.none => ({ ps := refuse (ctx) (moved (ps) (position)) (cat (cat (lit ("unclosed ")) (ticked ((current).envName))) (lit (" environment"))), sections := sections, mode := Mode.InProof (current) (declaration) (frames) } : Machine) | Option.some atom => (if isAtom (atom) (0) (lit ("\\end")) then (let ended : Ps := expectEnd (ctx) (ps) ((current).envName); returnProof (ctx) (ended) (sections) (declaration) (frames) (Proof.ProofEnv (chainItems ((current).items)) ((current).opening))) else (if isAtom (atom) (0) (lit ("\\begin")) then proofEnvItem (ctx) (ps) (sections) (current) (declaration) (frames) else (let said : SentenceAt := sentence (ctx) (ps); ({ ps := (said).ps, sections := sections, mode := Mode.InProof (pushItem (current) (Proof.ProofSentence ((said).sentence))) (declaration) (frames) } : Machine))))))

@[expose] public def declKindOf (name : ByteArray) : Option (DeclKind) := (if same (name) (lit ("typedefinition")) then Option.some (DeclKind.TypeDefinition) else (if same (name) (lit ("termdefinition")) then Option.some (DeclKind.TermDefinition) else (if same (name) (lit ("predicatedefinition")) then Option.some (DeclKind.PredicateDefinition) else (if same (name) (lit ("theorem")) then Option.some (DeclKind.Theorem) else (if same (name) (lit ("lemma")) then Option.some (DeclKind.Lemma) else (if same (name) (lit ("corollary")) then Option.some (DeclKind.Corollary) else Option.none))))))

@[expose] public def isTheoremLike (kind : DeclKind) : Bool := (match kind with | DeclKind.TypeDefinition => false | DeclKind.TermDefinition => false | DeclKind.PredicateDefinition => false | DeclKind.Theorem => true | DeclKind.Lemma => true | DeclKind.Corollary => true)

@[expose] public def componentArg (ctx : Ctx) (ps : Ps) (owner : ByteArray) : ArgAt := (let component : ArgAt := brace (ctx) (ps); (let text : ByteArray := ((component).arg).text; ({ arg := (component).arg, ps := (if isComponentId (text) then coverMetadata (ctx) ((component).ps) ((component).arg) (owner) else refuse (ctx) ((component).ps) (cat (ticked (text)) (lit (" is not a component ID")))) } : ArgAt)))

@[expose] public def openDeclaration (ctx : Ctx) (ps : Ps) (sections : List (SectionFrame)) (outer : BlocksCtx) (opening : Nat) (env : ByteArray) (kind : DeclKind) : Machine := (let owner : ByteArray := cat (lit ("env.")) (env); (let component : ArgAt := componentArg (ctx) (ps) (owner); (let «theorem» : Bool := isTheoremLike (kind); (let entry : Option (ArgAt) := (if «theorem» then Option.none else (let named : ArgAt := brace (ctx) ((component).ps); Option.some (({ arg := (named).arg, ps := coverMetadata (ctx) ((named).ps) ((named).arg) (owner) } : ArgAt)))); (let policy : PolicyAt := policyOf (ctx) ((match entry with | Option.some named => (named).ps | Option.none => (component).ps)); (let said : SentenceAt := sentence (ctx) ((policy).ps); (let after : Ps := (said).ps; (let frame : DeclFrame := ({ outer := outer, env := env, kind := kind, component := (component).arg, entry := (match entry with | Option.some named => Option.some ((named).arg) | Option.none => Option.none), policy := (policy).policy, sentence := (said).sentence, opening := opening } : DeclFrame); (let holds : Bool := atBeginOf (ctx) ((after).pos) (lit ("proof")); (if «theorem» then (if holds then (let began : EnvAt := beginEnv (ctx) (after); enterProof (ctx) ((began).ps) (sections) ((began).opening) (lit ("proof")) (1) (frame) (([] : List (ProofFrame)))) else ({ ps := fail (ctx) (after) (lit ("LLF5005")) (lit ("a theorem-like declaration without a proof is not valid")), sections := sections, mode := Mode.InBlocks (outer) } : Machine)) else (if holds then ({ ps := refuse (ctx) (after) (lit ("no definitional component accepts a proof environment")), sections := sections, mode := Mode.InBlocks (outer) } : Machine) else finishDecl (ctx) (after) (sections) (frame) (Option.none))))))))))))

@[expose] public def openSection (ctx : Ctx) (ps : Ps) (sections : List (SectionFrame)) (current : BlocksCtx) (opening : Nat) : Machine := (let nested : Nat := ((current).depth + 1); (if (Nat.blt ((ctx).maxDepth) (nested)) then ({ ps := fail (ctx) (ps) (lit ("LLS8002")) (nestingMessage (ctx) (lit ("section nesting")) (nested)), sections := sections, mode := Mode.InBlocks (current) } : Machine) else (let component : ArgAt := componentArg (ctx) (ps) (lit ("env.section")); (let heading : ArgAt := brace (ctx) (expectControl (ctx) ((component).ps) (lit ("\\heading"))); (let params : Option (ArgAt) := (if atControl (ctx) (((heading).ps).pos) (lit ("\\parameters")) then Option.some (brace (ctx) (expectControl (ctx) ((heading).ps) (lit ("\\parameters")))) else Option.none); (let after : Ps := (match params with | Option.some given => (given).ps | Option.none => (heading).ps); ({ ps := after, sections := (({ outer := current, component := (component).arg, headingStart := ((heading).arg).start, headingStop := ((heading).arg).stop, params := (match params with | Option.some given => Option.some ((((given).arg).start, ((given).arg).stop)) | Option.none => Option.none), opening := opening } : SectionFrame) :: sections), mode := Mode.InBlocks (({ terminator := lit ("section"), depth := nested, acc := ([] : List (Block)) } : BlocksCtx)) } : Machine)))))))

@[expose] public def closeBlocks (ctx : Ctx) (ps : Ps) (sections : List (SectionFrame)) (current : BlocksCtx) : Machine := (let ended : Ps := expectEnd (ctx) (ps) ((current).terminator); (let chain : Block := chainBlocks ((current).acc); (match sections with | List.nil => ({ ps := ended, sections := sections, mode := Mode.Finished (chain) } : Machine) | List.cons top rest => ({ ps := ended, sections := rest, mode := Mode.InBlocks (pushBlock ((top).outer) (Block.BlockSection ((top).component) ((top).headingStart) ((top).headingStop) ((top).params) (chain) ((top).opening))) } : Machine))))

@[expose] public def blocksStep (ctx : Ctx) (ps : Ps) (sections : List (SectionFrame)) (current : BlocksCtx) : Machine := (let position : Nat := skipWs (ctx) ((ps).pos); (let term : ByteArray := (current).terminator; (match atomAt (ctx) (position) with | Option.none => ({ ps := refuse (ctx) (moved (ps) (position)) (cat (cat (lit ("unclosed ")) (ticked (term))) (lit (" environment"))), sections := sections, mode := Mode.InBlocks (current) } : Machine) | Option.some atom => (if isAtom (atom) (0) (lit ("\\end")) then closeBlocks (ctx) (ps) (sections) (current) else (if isAtom (atom) (0) (lit ("\\begin")) then (let began : EnvAt := beginEnv (ctx) (ps); (let name : ByteArray := (began).name; (if same (name) (lit ("section")) then openSection (ctx) ((began).ps) (sections) (current) ((began).opening) else (match declKindOf (name) with | Option.some kind => openDeclaration (ctx) ((began).ps) (sections) (current) ((began).opening) (name) (kind) | Option.none => ({ ps := fail (ctx) ((began).ps) (lit ("LLL1004")) (cat (ticked (name)) (lit (" is not a block environment"))), sections := sections, mode := Mode.InBlocks (current) } : Machine))))) else ({ ps := refuse (ctx) (moved (ps) (position)) (cat (cat (cat (cat (lit ("expected a block or `\\end{")) (term)) (lit ("}`, found `"))) ((atom).text)) (lit ("`"))), sections := sections, mode := Mode.InBlocks (current) } : Machine))))))

@[expose] public def machineStep (ctx : Ctx) (machine : Machine) : Option (Machine) := (let ps : Ps := (machine).ps; (match (ps).error with | Option.some _ => Option.none | Option.none => (match (machine).mode with | Mode.InBlocks current => Option.some (blocksStep (ctx) (ps) ((machine).sections) (current)) | Mode.InProof current declaration frames => Option.some (proofStep (ctx) (ps) ((machine).sections) (current) (declaration) (frames)) | Mode.Finished _ => Option.none)))

@[expose] public def argList (ctx : Ctx) (control : ByteArray) (owner : ByteArray) (state : (Prod (Ps) (List (Arg)))) : Option ((Prod (Ps) (List (Arg)))) := (let ps : Ps := (state).1; (match (ps).error with | Option.some _ => Option.none | Option.none => (if atControl (ctx) ((ps).pos) (control) then (let given : ArgAt := brace (ctx) (expectControl (ctx) (ps) (control)); Option.some ((coverMetadata (ctx) ((given).ps) ((given).arg) (owner), ((given).arg :: (state).2)))) else Option.none)))

@[expose] public def headerRows (ctx : Ctx) (ps : Ps) (control : ByteArray) (owner : ByteArray) : (Prod (Ps) (List (Arg))) := ((LexLeanCollections.iterateUntil ((fun (state : (Prod (Ps) (List (Arg)))) => argList (ctx) (control) (owner) (state))) (((ctx).count + 1)) ((ps, ([] : List (Arg)))) : (Prod ((Prod (Ps) (List (Arg)))) (Bool)))).1

@[expose] public def dataForm (ctx : Ctx) (ps : Ps) (envName : ByteArray) (control : ByteArray) (dataOwner : ByteArray) (sourceOwner : ByteArray) : (Prod (DataForm) (Ps)) := (let began : EnvAt := beginEnv (ctx) (ps); (let after : Ps := (began).ps; (let surface : Bool := (!atControl (ctx) ((after).pos) (control)); (let data : ArgAt := (if surface then verbatimBody (ctx) (moved (after) (skipWs (ctx) ((after).pos))) else braceWith (ctx) (expectControl (ctx) (after) (control)) (true)); (let covered : Ps := coverPayload (ctx) ((data).ps) ((data).arg) ((if surface then sourceOwner else dataOwner)); (let ended : Ps := expectEnd (ctx) (expectEnd (ctx) (covered) (envName)) (lit ("lexlean")); (({ data := (data).arg, surface := surface, opening := (began).opening } : DataForm), ended)))))))

@[expose] public def blocksOf (ctx : Ctx) (ps : Ps) : (Prod (Block) (Ps)) := (let final : Machine := ((LexLeanCollections.iterateUntil ((fun (machine : Machine) => machineStep (ctx) (machine))) (((ctx).count + 1)) (({ ps := ps, sections := ([] : List (SectionFrame)), mode := Mode.InBlocks (({ terminator := lit ("lexlean"), depth := 0, acc := ([] : List (Block)) } : BlocksCtx)) } : Machine)) : (Prod (Machine) (Bool)))).1; (match (final).mode with | Mode.Finished chain => (chain, (final).ps) | Mode.InBlocks _ => (Block.BlockNil, (final).ps) | Mode.InProof _ _ _ => (Block.BlockNil, (final).ps)))

@[expose] public def noForm (_seed : Nat) : Option (DataForm) := Option.none

@[expose] public def finishModule (ctx : Ctx) (ps : Ps) (nameArg : Arg) (uses : List (Arg)) (imports : List (Arg)) (title : Arg) (blocks : Block) (core : Option (DataForm)) (semantic : Option (DataForm)) (tooling : Option (DataForm)) : Except (LexLeanBoot.PrimitiveScan.Rejection) (ModuleAst) := (let position : Nat := skipWs (ctx) ((ps).pos); (let last : Ps := (match atomAt (ctx) (position) with | Option.some atom => refuse (ctx) (moved (ps) (position)) (cat (cat (lit ("unexpected ")) (ticked ((atom).text))) (lit (" after \\end{lexlean}"))) | Option.none => ps); (match (last).error with | Option.some rejection => Except.error (rejection) | Option.none => Except.ok (({ rows := LexLeanBoot.PrimitiveScan.reversed (Row) ((last).rows), name := (nameArg).text, nameArg := nameArg, uses := uses, imports := imports, titleStart := (title).start, titleStop := (title).stop, blocks := blocks, core := core, semantic := semantic, tooling := tooling } : ModuleAst)))))

@[expose] public def parseBody (ctx : Ctx) (ps : Ps) (nameArg : Arg) (uses : List (Arg)) (imports : List (Arg)) (title : Arg) : Except (LexLeanBoot.PrimitiveScan.Rejection) (ModuleAst) := (let position : Nat := (ps).pos; (if atBeginOf (ctx) (position) (lit ("coremodule")) then (let form : (Prod (DataForm) (Ps)) := dataForm (ctx) (ps) (lit ("coremodule")) (lit ("\\coredata")) (lit ("coredata")) (lit ("coresource")); finishModule (ctx) ((form).2) (nameArg) (uses) (imports) (title) (Block.BlockNil) (Option.some ((form).1)) (noForm (0)) (noForm (0))) else (if atBeginOf (ctx) (position) (lit ("semanticmodule")) then (let form : (Prod (DataForm) (Ps)) := dataForm (ctx) (ps) (lit ("semanticmodule")) (lit ("\\semanticdata")) (lit ("semanticdata")) (lit ("semanticsource")); finishModule (ctx) ((form).2) (nameArg) (uses) (imports) (title) (Block.BlockNil) (noForm (0)) (Option.some ((form).1)) (noForm (0))) else (if atBeginOf (ctx) (position) (lit ("toolingmodule")) then (let form : (Prod (DataForm) (Ps)) := dataForm (ctx) (ps) (lit ("toolingmodule")) (lit ("\\toolingdata")) (lit ("toolingdata")) (lit ("toolingsource")); finishModule (ctx) ((form).2) (nameArg) (uses) (imports) (title) (Block.BlockNil) (noForm (0)) (noForm (0)) (Option.some ((form).1))) else (let parsed : (Prod (Block) (Ps)) := blocksOf (ctx) (ps); finishModule (ctx) ((parsed).2) (nameArg) (uses) (imports) (title) ((parsed).1) (noForm (0)) (noForm (0)) (noForm (0)))))))

@[expose] public def parseModule (path : ByteArray) (atoms : List (LexLeanBoot.PrimitiveScan.Atom)) (expectedName : ByteArray) (maxScopeDepth : Nat) : Except (LexLeanBoot.PrimitiveScan.Rejection) (ModuleAst) := (let ctx : Ctx := ({ path := path, atoms := atoms, count := (LexLeanRuntime.length (atoms) : Nat), maxDepth := maxScopeDepth } : Ctx); (let began : EnvAt := beginEnv (ctx) (({ pos := 0, rows := ([] : List (Row)), error := Option.none } : Ps)); (let headed : Ps := (if same ((began).name) (lit ("lexlean")) then (began).ps else refuse (ctx) ((began).ps) (lit ("a module begins with \\begin{lexlean}{module-name}"))); (let nameArg : ArgAt := brace (ctx) (headed); (let spelled : ByteArray := ((nameArg).arg).text; (let covered : Ps := coverMetadata (ctx) ((nameArg).ps) ((nameArg).arg) (lit ("env.lexlean")); (let named : Ps := (if same (spelled) (expectedName) then covered else refuse (ctx) (covered) (cat (cat (cat (lit ("the declared module ")) (ticked (spelled))) (lit (" does not match the source path module "))) (ticked (expectedName)))); (let uses : (Prod (Ps) (List (Arg))) := headerRows (ctx) (named) (lit ("\\useglossary")) (lit ("useglossary")); (let imports : (Prod (Ps) (List (Arg))) := headerRows (ctx) ((uses).1) (lit ("\\importmodule")) (lit ("importmodule")); (let ordered : Ps := (if atControl (ctx) (((imports).1).pos) (lit ("\\useglossary")) then refuse (ctx) ((imports).1) (lit ("glossary imports occur before module imports in the header")) else (imports).1); (let title : ArgAt := brace (ctx) (expectControl (ctx) (ordered) (lit ("\\title"))); (let single : Ps := (if atControl (ctx) (((title).ps).pos) (lit ("\\title")) then refuse (ctx) ((title).ps) (lit ("each module has exactly one title")) else (title).ps); parseBody (ctx) (single) ((nameArg).arg) (LexLeanBoot.PrimitiveScan.reversed (Arg) ((uses).2)) (LexLeanBoot.PrimitiveScan.reversed (Arg) ((imports).2)) ((title).arg)))))))))))))

end LexLeanBoot.StructuralGrammar
