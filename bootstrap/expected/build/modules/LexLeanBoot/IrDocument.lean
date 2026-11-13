module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanBoot.ContentId
public import LexLeanBoot.IrProof
public import LexLeanBoot.IrTerm
public import LexLeanStd.Json
public import LexLeanStd.Order
public import LexLeanStd.Text
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.IrDocument

public inductive PhraseItem where
  | WWord (_ : ByteArray) (_ : ByteArray)
  | WMath (_ : LexLeanBoot.IrTerm.Term)
  | WPunct (_ : ByteArray)

public inductive Block where
  | BSection (_ : ByteArray) (_ : List (PhraseItem)) (_ : LexLeanBoot.IrTerm.Term) (_ : Block)
  | BDecl (_ : LexLeanBoot.IrProof.Declaration)
  | BNil
  | BItem (_ : Block) (_ : Block)

public structure DocumentModule where
  name : ByteArray
  leanModule : ByteArray
  sourcePath : ByteArray
  sourceSha256 : ByteArray
  origin : Option (ByteArray)
  glossary : List (ByteArray)
  imports : List (ByteArray)
  importLean : List ((Prod (ByteArray) (ByteArray)))
  title : List (PhraseItem)
  blocks : Block
  core : Option (ByteArray)
  semantic : Option (LexLeanStd.Json.Json)
  tooling : Option (LexLeanStd.Json.Json)

@[expose] public def jnil : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JNil

@[expose] public def jobj (members : LexLeanStd.Json.Json) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JObj (members)

@[expose] public def jarr (items : LexLeanStd.Json.Json) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JArr (items)

@[expose] public def jbytes (text : ByteArray) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JStr (text)

@[expose] public def chainOf (items : List (LexLeanStd.Json.Json)) : LexLeanStd.Json.Json := (LexLeanCollections.listFold ((fun (chain : LexLeanStd.Json.Json) (item : LexLeanStd.Json.Json) => LexLeanStd.Json.Json.JItem (item) (chain))) (jnil) (LexLeanStd.Order.reverseList (LexLeanStd.Json.Json) (items)) : LexLeanStd.Json.Json)

@[expose] public def textArray (texts : List (ByteArray)) : LexLeanStd.Json.Json := jarr (chainOf ((LexLeanCollections.listFold ((fun (acc : List (LexLeanStd.Json.Json)) (text : ByteArray) => (jbytes (text) :: acc))) (([] : List (LexLeanStd.Json.Json))) (LexLeanStd.Order.reverseList (ByteArray) (texts)) : List (LexLeanStd.Json.Json))))

@[expose] public def itemJson (item : PhraseItem) (state : LexLeanBoot.IrTerm.Renumber) : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := (match item with | PhraseItem.WWord entry form => (jobj (LexLeanBoot.IrTerm.member ("k") (LexLeanBoot.IrTerm.jtext ("word")) (LexLeanBoot.IrTerm.member ("entry") (jbytes (entry)) (LexLeanBoot.IrTerm.member ("form") (jbytes (form)) (jnil)))), state) | PhraseItem.WMath term => (let written : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := LexLeanBoot.IrTerm.termJson (term) (state); (jobj (LexLeanBoot.IrTerm.member ("k") (LexLeanBoot.IrTerm.jtext ("math")) (LexLeanBoot.IrTerm.member ("t") ((written).1) (jnil))), (written).2)) | PhraseItem.WPunct entry => (jobj (LexLeanBoot.IrTerm.member ("k") (LexLeanBoot.IrTerm.jtext ("punct")) (LexLeanBoot.IrTerm.member ("entry") (jbytes (entry)) (jnil))), state))

@[expose] public def phraseJson (items : List (PhraseItem)) (state : LexLeanBoot.IrTerm.Renumber) : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := (let folded : (Prod (List (LexLeanStd.Json.Json)) (LexLeanBoot.IrTerm.Renumber)) := (LexLeanCollections.listFold ((fun (acc : (Prod (List (LexLeanStd.Json.Json)) (LexLeanBoot.IrTerm.Renumber))) (item : PhraseItem) => (let written : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := itemJson (item) ((acc).2); (((written).1 :: (acc).1), (written).2)))) ((([] : List (LexLeanStd.Json.Json)), state)) (items) : (Prod (List (LexLeanStd.Json.Json)) (LexLeanBoot.IrTerm.Renumber))); (jarr ((LexLeanCollections.listFold ((fun (chain : LexLeanStd.Json.Json) (written : LexLeanStd.Json.Json) => LexLeanStd.Json.Json.JItem (written) (chain))) (jnil) ((folded).1) : LexLeanStd.Json.Json)), (folded).2))

@[expose] public def blockJson : (block : Block) -> (state : LexLeanBoot.IrTerm.Renumber) -> (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber))
  | Block.BSection component heading params nested, state => (let headed : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := phraseJson (heading) (state); (let bound : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := LexLeanBoot.IrTerm.termJson (params) ((headed).2); (let inner : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := blockJson (nested) ((bound).2); (jobj (LexLeanBoot.IrTerm.member ("k") (LexLeanBoot.IrTerm.jtext ("section")) (LexLeanBoot.IrTerm.member ("component") (jbytes (component)) (LexLeanBoot.IrTerm.member ("heading") ((headed).1) (LexLeanBoot.IrTerm.member ("params") (jarr ((bound).1)) (LexLeanBoot.IrTerm.member ("blocks") (jarr ((inner).1)) (jnil)))))), (inner).2))))
  | Block.BDecl declaration, state => (let written : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := LexLeanBoot.IrProof.declarationJson (declaration) (state); (jobj (LexLeanBoot.IrTerm.member ("k") (LexLeanBoot.IrTerm.jtext ("declaration")) (LexLeanBoot.IrTerm.member ("d") ((written).1) (jnil))), (written).2))
  | Block.BNil, state => (jnil, state)
  | Block.BItem head tail, state => (let written : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := blockJson (head) (state); (let rest : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := blockJson (tail) ((written).2); (LexLeanStd.Json.Json.JItem ((written).1) ((rest).1), (rest).2)))

@[expose] public def defaultImport (linked : DocumentModule) (imported : ByteArray) : ByteArray := (let leanName : ByteArray := (linked).leanModule; (let short : Nat := (LexLeanRuntime.length ((linked).name) : Nat); (let total : Nat := (LexLeanRuntime.length (leanName) : Nat); (let stem : ByteArray := (if LexLeanStd.Text.endsWith (leanName) ((linked).name) then LexLeanStd.Text.sliceOr (leanName) (0) ((LexLeanRuntime.subtract (total) (short) : Nat)) else _root_.ByteArray.mk #[]); (LexLeanRuntime.append (stem) (imported) : ByteArray)))))

@[expose] public def foreignImports (linked : DocumentModule) : LexLeanStd.Json.Json := (LexLeanCollections.listFold ((fun (rest : LexLeanStd.Json.Json) (entry : (Prod (ByteArray) (ByteArray))) => (if (LexLeanRuntime.equal ((entry).2) (defaultImport (linked) ((entry).1)) : Bool) then rest else LexLeanStd.Json.Json.JMember ((entry).1) (jbytes ((entry).2)) (rest)))) (jnil) ((linked).importLean) : LexLeanStd.Json.Json)

@[expose] public def optionalTail (linked : DocumentModule) : LexLeanStd.Json.Json := (let withTooling : LexLeanStd.Json.Json := (match (linked).tooling with | Option.some value => LexLeanBoot.IrTerm.member ("tooling") (value) (jnil) | Option.none => jnil); (let withSemantic : LexLeanStd.Json.Json := (match (linked).semantic with | Option.some value => LexLeanBoot.IrTerm.member ("semantic") (value) (withTooling) | Option.none => withTooling); (let withCore : LexLeanStd.Json.Json := (match (linked).core with | Option.some text => LexLeanBoot.IrTerm.member ("core") (jbytes (text)) (withSemantic) | Option.none => withSemantic); (let foreign : LexLeanStd.Json.Json := foreignImports (linked); (let withForeign : LexLeanStd.Json.Json := (match foreign with | LexLeanStd.Json.Json.JNil => withCore | LexLeanStd.Json.Json.JBool _ => withCore | LexLeanStd.Json.Json.JInt _ _ => withCore | LexLeanStd.Json.Json.JStr _ => withCore | LexLeanStd.Json.Json.JArr _ => withCore | LexLeanStd.Json.Json.JObj _ => withCore | LexLeanStd.Json.Json.JItem _ _ => withCore | LexLeanStd.Json.Json.JMember _ _ _ => LexLeanBoot.IrTerm.member ("import_lean") (jobj (foreign)) (withCore)); (match (linked).origin with | Option.some named => LexLeanBoot.IrTerm.member ("origin") (jbytes (named)) (withForeign) | Option.none => withForeign))))))

@[expose] public def moduleJson (linked : DocumentModule) : LexLeanStd.Json.Json := (let titled : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := phraseJson ((linked).title) (LexLeanBoot.IrTerm.hashing); (let inner : (Prod (LexLeanStd.Json.Json) (LexLeanBoot.IrTerm.Renumber)) := blockJson ((linked).blocks) ((titled).2); jobj (LexLeanBoot.IrTerm.member ("name") (jbytes ((linked).name)) (LexLeanBoot.IrTerm.member ("lean_module") (jbytes ((linked).leanModule)) (LexLeanBoot.IrTerm.member ("glossary") (textArray ((linked).glossary)) (LexLeanBoot.IrTerm.member ("imports") (textArray ((linked).imports)) (LexLeanBoot.IrTerm.member ("title") ((titled).1) (LexLeanBoot.IrTerm.member ("blocks") (jarr ((inner).1)) (optionalTail (linked))))))))))

@[expose] public def linkedJson (modules : List ((Prod (ByteArray) (DocumentModule)))) : LexLeanStd.Json.Json := jobj (LexLeanBoot.IrTerm.member ("spec") (LexLeanBoot.IrTerm.jtext ("lexlean/linked-ir/1")) (LexLeanBoot.IrTerm.member ("modules") (jarr (chainOf ((LexLeanCollections.listFold ((fun (acc : List (LexLeanStd.Json.Json)) (entry : (Prod (ByteArray) (DocumentModule))) => (moduleJson ((entry).2) :: acc))) (([] : List (LexLeanStd.Json.Json))) (LexLeanStd.Order.reverseList ((Prod (ByteArray) (DocumentModule))) (LexLeanStd.Order.sortByKey (DocumentModule) (modules))) : List (LexLeanStd.Json.Json))))) (jnil)))

@[expose] public def moduleText (linked : DocumentModule) : ByteArray := LexLeanStd.Json.writeJson (moduleJson (linked))

@[expose] public def linkedText (modules : List ((Prod (ByteArray) (DocumentModule)))) : ByteArray := LexLeanStd.Json.writeJson (linkedJson (modules))

@[expose] public def projectSemanticId (compilerSemantics : ByteArray) (language : ByteArray) (modules : List ((Prod (ByteArray) (DocumentModule)))) (lexiconClosure : ByteArray) : ByteArray := LexLeanBoot.ContentId.semanticId (compilerSemantics) (language) (LexLeanStd.Json.writeJson (linkedJson (modules))) (lexiconClosure)

end LexLeanBoot.IrDocument
