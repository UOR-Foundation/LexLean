module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanBoot.IrTerm
public import LexLeanStd.Json
public import LexLeanStd.Order
public import LexLeanBoot.LexiconLse
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.IrProof

public inductive Proof where
  | PSeq (_ : Proof)
  | PIntro (_ : List (Nat))
  | PExact (_ : LexLeanBoot.IrTerm.Term)
  | PApplyOne (_ : LexLeanBoot.IrTerm.Term)
  | PApply (_ : LexLeanBoot.IrTerm.Term) (_ : Proof)
  | PRfl
  | PWitness (_ : LexLeanBoot.IrTerm.Term)
  | PLeft
  | PRight
  | PHave (_ : Nat) (_ : LexLeanBoot.IrTerm.Term) (_ : Proof)
  | PRewrite (_ : Option (Nat)) (_ : Proof)
  | PSimp (_ : Option (Nat)) (_ : LexLeanBoot.IrTerm.Term)
  | PConstructor (_ : Proof)
  | PCases (_ : LexLeanBoot.IrTerm.Term) (_ : Proof)
  | PInduction (_ : LexLeanBoot.IrTerm.Term) (_ : Proof)
  | PCalc (_ : LexLeanBoot.IrTerm.GlobalRef) (_ : LexLeanBoot.IrTerm.Term) (_ : Proof)
  | PNil
  | PItem (_ : Proof) (_ : Proof)
  | PRule (_ : Bool) (_ : LexLeanBoot.IrTerm.Term) (_ : Proof)
  | PCase (_ : ByteArray) (_ : ByteArray) (_ : List ((Prod (Nat) (ByteArray)))) (_ : Proof) (_ : Proof)
  | PStep (_ : LexLeanBoot.IrTerm.Term) (_ : LexLeanBoot.IrTerm.Term) (_ : Proof)

public inductive DeclKind where
  | KTypeDefinition
  | KTermDefinition
  | KPredicateDefinition
  | KTheorem
  | KLemma
  | KCorollary

public inductive AxiomPolicy where
  | ANone
  | AAllow (_ : List (ByteArray))
  | AExact (_ : List (ByteArray))

public inductive DeclBody where
  | BDefinition (_ : ByteArray) (_ : LexLeanBoot.IrTerm.Term) (_ : LexLeanBoot.IrTerm.Term)
  | BTheorem (_ : LexLeanBoot.IrTerm.Term) (_ : Proof)

public structure Declaration where
  component : ByteArray
  leanName : ByteArray
  kind : DeclKind
  params : LexLeanBoot.IrTerm.Term
  body : DeclBody
  policy : AxiomPolicy

public structure Walk where
  remaining : List (ByteArray)
  same : Bool

@[expose] public def jarr (items : LexLeanStd.Json.Json) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JArr (items)

@[expose] public def jobj (members : LexLeanStd.Json.Json) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JObj (members)

@[expose] public def jnil : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JNil

@[expose] public def tagged (tag : String) (rest : LexLeanStd.Json.Json) : LexLeanStd.Json.Json := jobj (LexLeanBoot.IrTerm.member ("k") (LexLeanBoot.IrTerm.jtext (tag)) (rest))

@[expose] public def bindAll (locals : List (Nat)) (state : LexLeanBoot.IrTerm.Renumber) : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := (let folded : (Prod (List (Nat)) (LexLeanBoot.IrTerm.Renumber)) := (LexLeanCollections.listFold ((fun (acc : (Prod (List (Nat)) (LexLeanBoot.IrTerm.Renumber))) (localId : Nat) => (let bound : (Prod (Nat) (LexLeanBoot.IrTerm.Renumber)) := LexLeanBoot.IrTerm.bind ((acc).2) (localId); (((bound).1 :: (acc).1), (bound).2)))) ((([] : List (Nat)), state)) (locals) : (Prod (List (Nat)) (LexLeanBoot.IrTerm.Renumber))); ((LexLeanCollections.listFold ((fun (chain : LexLeanStd.Json.Json) (index : Nat) => LexLeanStd.Json.Json.JItem (LexLeanBoot.IrTerm.jnat (index)) (chain))) (jnil) ((folded).1) : LexLeanStd.Json.Json), (folded).2))

@[expose] public def caseLocals (binders : List ((Prod (Nat) (ByteArray)))) : List (Nat) := LexLeanStd.Order.reverseList (Nat) ((LexLeanCollections.listFold ((fun (acc : List (Nat)) (entry : (Prod (Nat) (ByteArray))) => ((entry).1 :: acc))) (([] : List (Nat))) (binders) : List (Nat)))

@[expose] public def targetJson (target : Option (Nat)) (state : LexLeanBoot.IrTerm.Renumber) : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := (match target with | Option.none => (LexLeanBoot.IrTerm.jtext ("goal"), state) | Option.some localId => (let resolved : (Prod (LexLeanBoot.IrTerm.Resolved) (LexLeanBoot.IrTerm.Renumber)) := LexLeanBoot.IrTerm.resolve (state) (localId); ((match (resolved).1 with | LexLeanBoot.IrTerm.Resolved.RBound index => LexLeanBoot.IrTerm.jnat (index) | LexLeanBoot.IrTerm.Resolved.RFree free => LexLeanBoot.IrTerm.jnat (free)), (resolved).2)))

@[expose] public def termWith (key : String) (term : LexLeanBoot.IrTerm.Term) (state : LexLeanBoot.IrTerm.Renumber) (rest : LexLeanStd.Json.Json) : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := (let written : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := LexLeanBoot.IrTerm.termJson (term) (state); (LexLeanBoot.IrTerm.member (key) ((written).1) (rest), (written).2))

@[expose] public def proofJson : (proof : Proof) -> (state : LexLeanBoot.IrTerm.Renumber) -> (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber))
  | Proof.PSeq steps, state => (let inner : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := proofJson (steps) (state); (tagged ("seq") (LexLeanBoot.IrTerm.member ("s") (jarr ((inner).1)) (jnil)), (inner).2))
  | Proof.PIntro locals, state => (let bound : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := bindAll (locals) (state); (tagged ("intro") (LexLeanBoot.IrTerm.member ("l") (jarr ((bound).1)) (jnil)), (bound).2))
  | Proof.PExact term, state => (let written : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := termWith ("t") (term) (state) (jnil); (tagged ("exact") ((written).1), (written).2))
  | Proof.PApplyOne term, state => (let written : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := termWith ("t") (term) (state) (jnil); (tagged ("apply-one") ((written).1), (written).2))
  | Proof.PApply function premises, state => (let head : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := LexLeanBoot.IrTerm.termJson (function) (state); (let inner : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := proofJson (premises) ((head).2); (tagged ("apply") (LexLeanBoot.IrTerm.member ("f") ((head).1) (LexLeanBoot.IrTerm.member ("p") (jarr ((inner).1)) (jnil))), (inner).2)))
  | Proof.PRfl, state => (tagged ("rfl") (jnil), state)
  | Proof.PWitness term, state => (let written : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := termWith ("t") (term) (state) (jnil); (tagged ("witness") ((written).1), (written).2))
  | Proof.PLeft, state => (tagged ("left") (jnil), state)
  | Proof.PRight, state => (tagged ("right") (jnil), state)
  | Proof.PHave localId proposition nested, state => (let stated : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := LexLeanBoot.IrTerm.termJson (proposition) (state); (let index : (Prod (Nat) (LexLeanBoot.IrTerm.Renumber)) := LexLeanBoot.IrTerm.bind ((stated).2) (localId); (let inner : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := proofJson (nested) ((index).2); (tagged ("have") (LexLeanBoot.IrTerm.member ("l") (LexLeanBoot.IrTerm.jnat ((index).1)) (LexLeanBoot.IrTerm.member ("p") ((stated).1) (LexLeanBoot.IrTerm.member ("q") ((inner).1) (jnil)))), (inner).2))))
  | Proof.PRewrite target rules, state => (let aimed : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := targetJson (target) (state); (let inner : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := proofJson (rules) ((aimed).2); (tagged ("rw") (LexLeanBoot.IrTerm.member ("at") ((aimed).1) (LexLeanBoot.IrTerm.member ("r") (jarr ((inner).1)) (jnil))), (inner).2)))
  | Proof.PSimp target rules, state => (let aimed : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := targetJson (target) (state); (let inner : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := LexLeanBoot.IrTerm.termJson (rules) ((aimed).2); (tagged ("simp-only") (LexLeanBoot.IrTerm.member ("at") ((aimed).1) (LexLeanBoot.IrTerm.member ("r") (jarr ((inner).1)) (jnil))), (inner).2)))
  | Proof.PConstructor branches, state => (let inner : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := proofJson (branches) (state); (tagged ("constructor") (LexLeanBoot.IrTerm.member ("b") (jarr ((inner).1)) (jnil)), (inner).2))
  | Proof.PCases scrutinee branchesOf, state => (let head : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := LexLeanBoot.IrTerm.termJson (scrutinee) (state); (let inner : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := proofJson (branchesOf) ((head).2); (tagged ("cases") (LexLeanBoot.IrTerm.member ("t") ((head).1) (LexLeanBoot.IrTerm.member ("c") (jarr ((inner).1)) (jnil))), (inner).2)))
  | Proof.PInduction scrutinee branchesOf, state => (let head : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := LexLeanBoot.IrTerm.termJson (scrutinee) (state); (let inner : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := proofJson (branchesOf) ((head).2); (tagged ("induction") (LexLeanBoot.IrTerm.member ("t") ((head).1) (LexLeanBoot.IrTerm.member ("c") (jarr ((inner).1)) (jnil))), (inner).2)))
  | Proof.PCalc relation start steps, state => (let begun : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := LexLeanBoot.IrTerm.termJson (start) (state); (let inner : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := proofJson (steps) ((begun).2); (tagged ("calc") (LexLeanBoot.IrTerm.member ("rel") (LexLeanBoot.IrTerm.globalTermJson (relation) (LexLeanBoot.LexiconLse.Universe.UNil)) (LexLeanBoot.IrTerm.member ("s") ((begun).1) (LexLeanBoot.IrTerm.member ("steps") (jarr ((inner).1)) (jnil)))), (inner).2)))
  | Proof.PNil, state => (jnil, state)
  | Proof.PItem head tail, state => (let written : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := proofJson (head) (state); (let rest : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := proofJson (tail) ((written).2); (LexLeanStd.Json.Json.JItem ((written).1) ((rest).1), (rest).2)))
  | Proof.PRule backward term tail, state => (let written : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := LexLeanBoot.IrTerm.termJson (term) (state); (let rest : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := proofJson (tail) ((written).2); (LexLeanStd.Json.Json.JItem (jobj (LexLeanBoot.IrTerm.member ("rev") (LexLeanStd.Json.Json.JBool (backward)) (LexLeanBoot.IrTerm.member ("t") ((written).1) (jnil)))) ((rest).1), (rest).2)))
  | Proof.PCase ctorName leanName binders branch tail, state => (let bound : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := bindAll (caseLocals (binders)) (state); (let inner : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := proofJson (branch) ((bound).2); (let rest : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := proofJson (tail) ((inner).2); (LexLeanStd.Json.Json.JItem (jobj (LexLeanBoot.IrTerm.member ("ctor") (LexLeanStd.Json.Json.JStr (ctorName)) (LexLeanBoot.IrTerm.member ("lean") (LexLeanStd.Json.Json.JStr (leanName)) (LexLeanBoot.IrTerm.member ("b") (jarr ((bound).1)) (LexLeanBoot.IrTerm.member ("p") ((inner).1) (jnil)))))) ((rest).1), (rest).2))))
  | Proof.PStep term justification tail, state => (let written : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := LexLeanBoot.IrTerm.termJson (term) (state); (let justified : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := LexLeanBoot.IrTerm.termJson (justification) ((written).2); (let rest : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := proofJson (tail) ((justified).2); (LexLeanStd.Json.Json.JItem (jobj (LexLeanBoot.IrTerm.member ("t") ((written).1) (LexLeanBoot.IrTerm.member ("p") ((justified).1) (jnil)))) ((rest).1), (rest).2))))

@[expose] public def kindName (kind : DeclKind) : LexLeanStd.Json.Json := (match kind with | DeclKind.KTypeDefinition => LexLeanBoot.IrTerm.jtext ("typedefinition") | DeclKind.KTermDefinition => LexLeanBoot.IrTerm.jtext ("termdefinition") | DeclKind.KPredicateDefinition => LexLeanBoot.IrTerm.jtext ("predicatedefinition") | DeclKind.KTheorem => LexLeanBoot.IrTerm.jtext ("theorem") | DeclKind.KLemma => LexLeanBoot.IrTerm.jtext ("lemma") | DeclKind.KCorollary => LexLeanBoot.IrTerm.jtext ("corollary"))

@[expose] public def textChain (names : List (ByteArray)) : LexLeanStd.Json.Json := (LexLeanCollections.listFold ((fun (chain : LexLeanStd.Json.Json) (named : ByteArray) => LexLeanStd.Json.Json.JItem (LexLeanStd.Json.Json.JStr (named)) (chain))) (jnil) (LexLeanStd.Order.reverseList (ByteArray) (names)) : LexLeanStd.Json.Json)

@[expose] public def policyJson (policy : AxiomPolicy) : LexLeanStd.Json.Json := (match policy with | AxiomPolicy.ANone => jobj (LexLeanBoot.IrTerm.member ("kind") (LexLeanBoot.IrTerm.jtext ("none")) (LexLeanBoot.IrTerm.member ("axioms") (jarr (jnil)) (jnil))) | AxiomPolicy.AAllow names => jobj (LexLeanBoot.IrTerm.member ("kind") (LexLeanBoot.IrTerm.jtext ("allow")) (LexLeanBoot.IrTerm.member ("axioms") (jarr (textChain (names))) (jnil))) | AxiomPolicy.AExact names => jobj (LexLeanBoot.IrTerm.member ("kind") (LexLeanBoot.IrTerm.jtext ("exact")) (LexLeanBoot.IrTerm.member ("axioms") (jarr (textChain (names))) (jnil))))

@[expose] public def bodyJson (body : DeclBody) (state : LexLeanBoot.IrTerm.Renumber) : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := (match body with | DeclBody.BDefinition entry typed defined => (let written : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := LexLeanBoot.IrTerm.termJson (typed) (state); (let valued : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := LexLeanBoot.IrTerm.termJson (defined) ((written).2); (tagged ("definition") (LexLeanBoot.IrTerm.member ("entry") (LexLeanStd.Json.Json.JStr (entry)) (LexLeanBoot.IrTerm.member ("t") ((written).1) (LexLeanBoot.IrTerm.member ("v") ((valued).1) (jnil)))), (valued).2))) | DeclBody.BTheorem statement proof => (let stated : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := LexLeanBoot.IrTerm.termJson (statement) (state); (let proved : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := proofJson (proof) ((stated).2); (tagged ("theorem") (LexLeanBoot.IrTerm.member ("s") ((stated).1) (LexLeanBoot.IrTerm.member ("p") ((proved).1) (jnil))), (proved).2))))

@[expose] public def declarationJson (declaration : Declaration) (state : LexLeanBoot.IrTerm.Renumber) : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := (let bound : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := LexLeanBoot.IrTerm.termJson ((declaration).params) (state); (let written : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := bodyJson ((declaration).body) ((bound).2); (jobj (LexLeanBoot.IrTerm.member ("component") (LexLeanStd.Json.Json.JStr ((declaration).component)) (LexLeanBoot.IrTerm.member ("lean_name") (LexLeanStd.Json.Json.JStr ((declaration).leanName)) (LexLeanBoot.IrTerm.member ("kind") (kindName ((declaration).kind)) (LexLeanBoot.IrTerm.member ("params") (jarr ((bound).1)) (LexLeanBoot.IrTerm.member ("body") ((written).1) (LexLeanBoot.IrTerm.member ("policy") (policyJson ((declaration).policy)) (jnil))))))), (written).2)))

@[expose] public def proofText (proof : Proof) : ByteArray := LexLeanStd.Json.writeJson ((proofJson (proof) (LexLeanBoot.IrTerm.hashing)).1)

@[expose] public def declarationText (declaration : Declaration) : ByteArray := LexLeanStd.Json.writeJson ((declarationJson (declaration) (LexLeanBoot.IrTerm.hashing)).1)

@[expose] public def listed (names : List (ByteArray)) (named : ByteArray) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (candidate : ByteArray) => (if found then true else (LexLeanRuntime.equal (candidate) (named) : Bool)))) (false) (names) : Bool)

@[expose] public def sortedUnique (names : List (ByteArray)) : List (ByteArray) := LexLeanStd.Order.reverseList (ByteArray) ((LexLeanCollections.listFold ((fun (kept : List (ByteArray)) (entry : (Prod (ByteArray) (ByteArray))) => (match kept with | List.nil => ((entry).1 :: kept) | List.cons head _ => (if (LexLeanRuntime.equal (head) ((entry).1) : Bool) then kept else ((entry).1 :: kept))))) (([] : List (ByteArray))) (LexLeanStd.Order.sortEntries ((LexLeanCollections.listFold ((fun (acc : List ((Prod (ByteArray) (ByteArray)))) (named : ByteArray) => ((named, named) :: acc))) (([] : List ((Prod (ByteArray) (ByteArray))))) (LexLeanStd.Order.reverseList (ByteArray) (names)) : List ((Prod (ByteArray) (ByteArray)))))) : List (ByteArray)))

@[expose] public def walkStep (walk : Walk) (named : ByteArray) : Walk := (match (walk).remaining with | List.nil => ({ remaining := ([] : List (ByteArray)), same := false } : Walk) | List.cons head rest => ({ remaining := rest, same := (if (walk).same then (LexLeanRuntime.equal (head) (named) : Bool) else false) } : Walk))

@[expose] public def sameNames (left : List (ByteArray)) (right : List (ByteArray)) : Bool := (let walked : Walk := (LexLeanCollections.listFold ((fun (walk : Walk) (named : ByteArray) => walkStep (walk) (named))) (({ remaining := right, same := true } : Walk)) (left) : Walk); (if (walked).same then (match (walked).remaining with | List.nil => true | List.cons _ _ => false) else false))

@[expose] public def permits (policy : AxiomPolicy) (observed : List (ByteArray)) : Bool := (match policy with | AxiomPolicy.ANone => (match observed with | List.nil => true | List.cons _ _ => false) | AxiomPolicy.AAllow allowed => (LexLeanCollections.listFold ((fun (allSoFar : Bool) (named : ByteArray) => (if allSoFar then listed (allowed) (named) else false))) (true) (observed) : Bool) | AxiomPolicy.AExact exactNames => sameNames (sortedUnique (observed)) (exactNames))

end LexLeanBoot.IrProof
