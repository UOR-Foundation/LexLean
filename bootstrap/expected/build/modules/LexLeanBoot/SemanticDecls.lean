module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Json
public import LexLeanStd.Order
public import LexLeanBoot.SemanticTerms
public import LexLeanBoot.SemanticTypes
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.SemanticDecls

public inductive SDecl where
  | DStructure (_ : ByteArray) (_ : List (ByteArray)) (_ : List ((Prod (ByteArray) (LexLeanBoot.SemanticTypes.SemanticType)))) (_ : List ((Prod (ByteArray) (LexLeanBoot.SemanticTypes.SemanticType))))
  | DClass (_ : ByteArray) (_ : List (ByteArray)) (_ : List ((Prod (ByteArray) (LexLeanBoot.SemanticTypes.SemanticType)))) (_ : List ((Prod (ByteArray) (LexLeanBoot.SemanticTypes.SemanticType))))
  | DInstance (_ : ByteArray) (_ : Option (ByteArray)) (_ : ByteArray) (_ : LexLeanBoot.SemanticTypes.SemanticType) (_ : Nat) (_ : LexLeanBoot.SemanticTerms.STerm)
  | DInductive (_ : ByteArray) (_ : List (ByteArray)) (_ : List ((Prod (ByteArray) (LexLeanBoot.SemanticTypes.SemanticType)))) (_ : List ((Prod (ByteArray) (LexLeanBoot.SemanticTypes.SemanticType)))) (_ : Option (ByteArray))
  | DDefinition (_ : ByteArray) (_ : List (ByteArray)) (_ : List ((Prod (ByteArray) (LexLeanBoot.SemanticTypes.SemanticType)))) (_ : LexLeanBoot.SemanticTypes.SemanticType) (_ : Option (ByteArray)) (_ : LexLeanBoot.SemanticTerms.STerm) (_ : List (ByteArray)) (_ : Bool) (_ : Option (ByteArray)) (_ : Option ((Prod (LexLeanBoot.SemanticTerms.STerm) (List ((Prod (Option (ByteArray)) (ByteArray))))))) (_ : Option ((Prod (List (ByteArray)) (List (ByteArray)))))
  | DTheorem (_ : ByteArray) (_ : List (ByteArray)) (_ : List ((Prod (ByteArray) (LexLeanBoot.SemanticTypes.SemanticType)))) (_ : LexLeanBoot.SemanticTerms.STerm) (_ : LexLeanBoot.SemanticTerms.SProof) (_ : List (ByteArray))

@[expose] public def lit (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def member (key : String) (entry : LexLeanStd.Json.Json) (rest : LexLeanStd.Json.Json) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JMember (lit (key)) (entry) (rest)

@[expose] public def jnil : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JNil

@[expose] public def jtext (text : ByteArray) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JStr (text)

@[expose] public def kinded (kind : String) (rest : LexLeanStd.Json.Json) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JObj (member ("kind") (jtext (lit (kind))) (rest))

@[expose] public def fieldsJson (entries : List ((Prod (ByteArray) (LexLeanBoot.SemanticTypes.SemanticType)))) : LexLeanStd.Json.Json := LexLeanBoot.SemanticTerms.paramsJson (entries)

@[expose] public def namesWhenAny (key : String) (names : List (ByteArray)) (rest : LexLeanStd.Json.Json) : LexLeanStd.Json.Json := (match names with | List.nil => rest | List.cons _ _ => member (key) (LexLeanBoot.SemanticTerms.namesJson (names)) (rest))

@[expose] public def textWhenSome (key : String) (value : Option (ByteArray)) (rest : LexLeanStd.Json.Json) : LexLeanStd.Json.Json := (match value with | Option.some text => member (key) (jtext (text)) (rest) | Option.none => rest)

@[expose] public def constructorsJson (constructors : List ((Prod (ByteArray) (LexLeanBoot.SemanticTypes.SemanticType)))) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JArr (LexLeanBoot.SemanticTerms.chainOf ((LexLeanCollections.listFold ((fun (acc : List (LexLeanStd.Json.Json)) (entry : (Prod (ByteArray) (LexLeanBoot.SemanticTypes.SemanticType))) => (LexLeanStd.Json.Json.JObj (member ("name") (jtext ((entry).1)) (member ("fields") (LexLeanStd.Json.Json.JArr (LexLeanBoot.SemanticTypes.typeJson ((entry).2))) (jnil))) :: acc))) (([] : List (LexLeanStd.Json.Json))) (LexLeanStd.Order.reverseList ((Prod (ByteArray) (LexLeanBoot.SemanticTypes.SemanticType))) (constructors)) : List (LexLeanStd.Json.Json))))

@[expose] public def terminationJson (termination : Option ((Prod (LexLeanBoot.SemanticTerms.STerm) (List ((Prod (Option (ByteArray)) (ByteArray))))))) (rest : LexLeanStd.Json.Json) : LexLeanStd.Json.Json := (match termination with | Option.some evidenced => member ("termination") (LexLeanStd.Json.Json.JObj (member ("measure") (LexLeanBoot.SemanticTerms.termJson ((evidenced).1)) (member ("evidence") (LexLeanBoot.SemanticTerms.membersJson ((evidenced).2)) (jnil)))) (rest) | Option.none => rest)

@[expose] public def productionJson (production : Option ((Prod (List (ByteArray)) (List (ByteArray))))) (rest : LexLeanStd.Json.Json) : LexLeanStd.Json.Json := (match production with | Option.some declared => member ("production") (LexLeanStd.Json.Json.JObj (member ("targets") (LexLeanBoot.SemanticTerms.namesJson ((declared).1)) (member ("effects") (LexLeanBoot.SemanticTerms.namesJson ((declared).2)) (jnil)))) (rest) | Option.none => rest)

@[expose] public def declarationJson (declaration : SDecl) : LexLeanStd.Json.Json := (match declaration with | SDecl.DStructure named typeParameters parameters fields => kinded ("structure") (member ("name") (jtext (named)) (member ("type_parameters") (LexLeanBoot.SemanticTerms.namesJson (typeParameters)) (member ("parameters") (LexLeanBoot.SemanticTerms.paramsJson (parameters)) (member ("fields") (fieldsJson (fields)) (jnil))))) | SDecl.DClass named typeParameters parameters fields => kinded ("class") (member ("name") (jtext (named)) (member ("type_parameters") (LexLeanBoot.SemanticTerms.namesJson (typeParameters)) (member ("parameters") (LexLeanBoot.SemanticTerms.paramsJson (parameters)) (member ("fields") (fieldsJson (fields)) (jnil))))) | SDecl.DInstance named classModule className arguments priority assigned => kinded ("instance") (member ("name") (jtext (named)) (member ("class") (LexLeanBoot.SemanticTerms.memberJson (classModule) (className)) (member ("arguments") (LexLeanStd.Json.Json.JArr (LexLeanBoot.SemanticTypes.typeJson (arguments))) (member ("priority") (LexLeanStd.Json.Json.JInt (false) (priority)) (member ("fields") (LexLeanStd.Json.Json.JArr (LexLeanBoot.SemanticTerms.termJson (assigned))) (jnil)))))) | SDecl.DInductive named typeParameters parameters constructors grouped => kinded ("inductive") (member ("name") (jtext (named)) (member ("type_parameters") (LexLeanBoot.SemanticTerms.namesJson (typeParameters)) (member ("parameters") (LexLeanBoot.SemanticTerms.paramsJson (parameters)) (member ("constructors") (constructorsJson (constructors)) (textWhenSome ("mutual") (grouped) (jnil)))))) | SDecl.DDefinition named typeParameters parameters outcome recursiveArg body axiomNames isExecutable grouped terminating produced => kinded ("definition") (member ("name") (jtext (named)) (namesWhenAny ("type_parameters") (typeParameters) (member ("parameters") (LexLeanBoot.SemanticTerms.paramsJson (parameters)) (member ("result") (LexLeanBoot.SemanticTypes.typeJson (outcome)) (textWhenSome ("recursive_argument") (recursiveArg) (member ("body") (LexLeanBoot.SemanticTerms.termJson (body)) (namesWhenAny ("axioms") (axiomNames) ((if isExecutable then member ("executable") (LexLeanStd.Json.Json.JBool (true)) (textWhenSome ("mutual") (grouped) (terminationJson (terminating) (productionJson (produced) (jnil)))) else textWhenSome ("mutual") (grouped) (terminationJson (terminating) (productionJson (produced) (jnil)))))))))))) | SDecl.DTheorem named typeParameters parameters statement proof axiomNames => kinded ("theorem") (member ("name") (jtext (named)) (namesWhenAny ("type_parameters") (typeParameters) (member ("parameters") (LexLeanBoot.SemanticTerms.paramsJson (parameters)) (member ("statement") (LexLeanBoot.SemanticTerms.termJson (statement)) (member ("proof") (LexLeanBoot.SemanticTerms.proofJson (proof)) (namesWhenAny ("axioms") (axiomNames) (jnil))))))))

@[expose] public def semanticDeclarationText (declaration : SDecl) : ByteArray := LexLeanStd.Json.writeJson (declarationJson (declaration))

@[expose] public def semanticModuleText (spec : ByteArray) (declarations : List (SDecl)) : ByteArray := LexLeanStd.Json.writeJson (LexLeanStd.Json.Json.JObj (member ("spec") (jtext (spec)) (member ("declarations") (LexLeanStd.Json.Json.JArr (LexLeanBoot.SemanticTerms.chainOf ((LexLeanCollections.listFold ((fun (acc : List (LexLeanStd.Json.Json)) (declaration : SDecl) => (declarationJson (declaration) :: acc))) (([] : List (LexLeanStd.Json.Json))) (LexLeanStd.Order.reverseList (SDecl) (declarations)) : List (LexLeanStd.Json.Json))))) (jnil))))

end LexLeanBoot.SemanticDecls
