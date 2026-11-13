module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Json
public import LexLeanStd.Order
public import LexLeanBoot.SemanticTypes
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.SemanticTerms

public inductive STerm where
  | TVar (_ : ByteArray)
  | TNat (_ : ByteArray)
  | TInteger (_ : LexLeanBoot.SemanticTypes.SemanticInteger) (_ : ByteArray)
  | TString (_ : ByteArray)
  | TBytes (_ : ByteArray)
  | TPrimitive (_ : LexLeanBoot.SemanticTypes.SemanticPrimitive) (_ : STerm) (_ : LexLeanBoot.SemanticTypes.SemanticType)
  | TBool (_ : Bool)
  | TUnit
  | TNil (_ : LexLeanBoot.SemanticTypes.SemanticType)
  | TCons (_ : STerm) (_ : STerm)
  | TRecord (_ : Option (ByteArray)) (_ : ByteArray) (_ : LexLeanBoot.SemanticTypes.SemanticType) (_ : STerm)
  | TConstructor (_ : Option (ByteArray)) (_ : ByteArray) (_ : LexLeanBoot.SemanticTypes.SemanticType) (_ : STerm)
  | TInstanceValue (_ : Option (ByteArray)) (_ : ByteArray) (_ : LexLeanBoot.SemanticTypes.SemanticType) (_ : Option (ByteArray)) (_ : ByteArray)
  | TProject (_ : STerm) (_ : ByteArray)
  | TCall (_ : Option (ByteArray)) (_ : ByteArray) (_ : LexLeanBoot.SemanticTypes.SemanticType) (_ : STerm)
  | TIf (_ : STerm) (_ : STerm) (_ : STerm)
  | TMatch (_ : STerm) (_ : STerm)
  | TEq (_ : STerm) (_ : STerm)
  | TLe (_ : STerm) (_ : STerm)
  | TLt (_ : STerm) (_ : STerm)
  | TAdd (_ : STerm) (_ : STerm)
  | TBeq (_ : STerm) (_ : STerm)
  | TBle (_ : STerm) (_ : STerm)
  | TBlt (_ : STerm) (_ : STerm)
  | TAnd (_ : STerm) (_ : STerm)
  | TPropAnd (_ : STerm) (_ : STerm)
  | TOr (_ : STerm) (_ : STerm)
  | TNot (_ : STerm)
  | TImplies (_ : STerm) (_ : STerm)
  | TIff (_ : STerm) (_ : STerm)
  | TForall (_ : ByteArray) (_ : LexLeanBoot.SemanticTypes.SemanticType) (_ : STerm)
  | TLet (_ : ByteArray) (_ : LexLeanBoot.SemanticTypes.SemanticType) (_ : STerm) (_ : STerm)
  | TPair (_ : STerm) (_ : STerm)
  | TFirst (_ : STerm)
  | TSecond (_ : STerm)
  | TLambda (_ : List ((Prod (ByteArray) (LexLeanBoot.SemanticTypes.SemanticType)))) (_ : List (ByteArray)) (_ : STerm)
  | TApply (_ : STerm) (_ : STerm)
  | TFunctionRef (_ : Option (ByteArray)) (_ : ByteArray) (_ : LexLeanBoot.SemanticTypes.SemanticType)
  | TMapLiteral (_ : LexLeanBoot.SemanticTypes.SemanticType) (_ : LexLeanBoot.SemanticTypes.SemanticType) (_ : STerm)
  | TSetLiteral (_ : LexLeanBoot.SemanticTypes.SemanticType) (_ : STerm)
  | TGraphLiteral (_ : LexLeanBoot.SemanticTypes.SemanticType) (_ : STerm) (_ : STerm)
  | TCheckedApply (_ : Option (ByteArray)) (_ : ByteArray) (_ : LexLeanBoot.SemanticTypes.SemanticType) (_ : STerm) (_ : List (ByteArray))
  | KEnd
  | KItem (_ : STerm) (_ : STerm)
  | KAssign (_ : ByteArray) (_ : STerm) (_ : STerm)
  | KBranch (_ : Option (ByteArray)) (_ : ByteArray) (_ : List (ByteArray)) (_ : STerm) (_ : STerm)
  | KEntry (_ : STerm) (_ : STerm) (_ : STerm)
  | KEdge (_ : STerm) (_ : STerm) (_ : STerm)

public inductive Reflection where
  | RList (_ : ByteArray) (_ : ByteArray) (_ : Option (ByteArray)) (_ : ByteArray) (_ : Option (ByteArray)) (_ : ByteArray) (_ : ByteArray)
  | RRecord (_ : ByteArray) (_ : Option (ByteArray)) (_ : ByteArray) (_ : Option (ByteArray)) (_ : ByteArray) (_ : List ((Prod (ByteArray) (ByteArray))))

public inductive SProof where
  | PReflexivity
  | PDecide
  | PLinearArithmetic (_ : List ((Prod (Option (ByteArray)) (ByteArray))))
  | PSimplify (_ : List ((Prod (Option (ByteArray)) (ByteArray))))
  | PConstructor (_ : SProof)
  | PCases (_ : ByteArray) (_ : SProof)
  | PTermCases (_ : STerm) (_ : List ((Prod (Option (ByteArray)) (ByteArray))))
  | PInduction (_ : ByteArray) (_ : List (ByteArray)) (_ : SProof)
  | PNestedInduction (_ : ByteArray) (_ : SProof) (_ : SProof)
  | PCongruence
  | PBooleanReflection (_ : Reflection)
  | PLaw (_ : ByteArray) (_ : LexLeanBoot.SemanticTypes.SemanticType) (_ : STerm) (_ : List ((Prod (Option (ByteArray)) (ByteArray))))
  | PApply (_ : Option (ByteArray)) (_ : ByteArray) (_ : LexLeanBoot.SemanticTypes.SemanticType) (_ : STerm)
  | QEnd
  | QItem (_ : SProof) (_ : SProof)
  | QBranch (_ : ByteArray) (_ : List (ByteArray)) (_ : SProof) (_ : SProof)
  | QMember (_ : LexLeanBoot.SemanticTypes.SemanticType) (_ : ByteArray) (_ : STerm) (_ : SProof) (_ : SProof)

@[expose] public def lit (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def member (key : String) (entry : LexLeanStd.Json.Json) (rest : LexLeanStd.Json.Json) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JMember (lit (key)) (entry) (rest)

@[expose] public def kinded (kind : String) (rest : LexLeanStd.Json.Json) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JObj (member ("kind") (LexLeanStd.Json.Json.JStr (lit (kind))) (rest))

@[expose] public def memberJson (owner : Option (ByteArray)) (named : ByteArray) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JObj ((match owner with | Option.some source => member ("module") (LexLeanStd.Json.Json.JStr (source)) (member ("name") (LexLeanStd.Json.Json.JStr (named)) (LexLeanStd.Json.Json.JNil)) | Option.none => member ("name") (LexLeanStd.Json.Json.JStr (named)) (LexLeanStd.Json.Json.JNil)))

@[expose] public def paramJson (named : ByteArray) (typed : LexLeanBoot.SemanticTypes.SemanticType) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JObj (member ("name") (LexLeanStd.Json.Json.JStr (named)) (member ("type") (LexLeanBoot.SemanticTypes.typeJson (typed)) (LexLeanStd.Json.Json.JNil)))

@[expose] public def chainOf (items : List (LexLeanStd.Json.Json)) : LexLeanStd.Json.Json := (LexLeanCollections.listFold ((fun (chain : LexLeanStd.Json.Json) (item : LexLeanStd.Json.Json) => LexLeanStd.Json.Json.JItem (item) (chain))) (LexLeanStd.Json.Json.JNil) (LexLeanStd.Order.reverseList (LexLeanStd.Json.Json) (items)) : LexLeanStd.Json.Json)

@[expose] public def paramsJson (entries : List ((Prod (ByteArray) (LexLeanBoot.SemanticTypes.SemanticType)))) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JArr (chainOf ((LexLeanCollections.listFold ((fun (acc : List (LexLeanStd.Json.Json)) (entry : (Prod (ByteArray) (LexLeanBoot.SemanticTypes.SemanticType))) => (paramJson ((entry).1) ((entry).2) :: acc))) (([] : List (LexLeanStd.Json.Json))) (LexLeanStd.Order.reverseList ((Prod (ByteArray) (LexLeanBoot.SemanticTypes.SemanticType))) (entries)) : List (LexLeanStd.Json.Json))))

@[expose] public def namesJson (names : List (ByteArray)) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JArr (chainOf ((LexLeanCollections.listFold ((fun (acc : List (LexLeanStd.Json.Json)) (named : ByteArray) => (LexLeanStd.Json.Json.JStr (named) :: acc))) (([] : List (LexLeanStd.Json.Json))) (LexLeanStd.Order.reverseList (ByteArray) (names)) : List (LexLeanStd.Json.Json))))

@[expose] public def membersJson (refs : List ((Prod (Option (ByteArray)) (ByteArray)))) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JArr (chainOf ((LexLeanCollections.listFold ((fun (acc : List (LexLeanStd.Json.Json)) (entry : (Prod (Option (ByteArray)) (ByteArray))) => (memberJson ((entry).1) ((entry).2) :: acc))) (([] : List (LexLeanStd.Json.Json))) (LexLeanStd.Order.reverseList ((Prod (Option (ByteArray)) (ByteArray))) (refs)) : List (LexLeanStd.Json.Json))))

@[expose] public def typesWhenAny (key : String) (types : LexLeanBoot.SemanticTypes.SemanticType) (rest : LexLeanStd.Json.Json) : LexLeanStd.Json.Json := (match types with | LexLeanBoot.SemanticTypes.SemanticType.SType => rest | LexLeanBoot.SemanticTypes.SemanticType.SNat => rest | LexLeanBoot.SemanticTypes.SemanticType.SBool => rest | LexLeanBoot.SemanticTypes.SemanticType.SProp => rest | LexLeanBoot.SemanticTypes.SemanticType.SUnit => rest | LexLeanBoot.SemanticTypes.SemanticType.SInt => rest | LexLeanBoot.SemanticTypes.SemanticType.SInt8 => rest | LexLeanBoot.SemanticTypes.SemanticType.SInt16 => rest | LexLeanBoot.SemanticTypes.SemanticType.SInt32 => rest | LexLeanBoot.SemanticTypes.SemanticType.SInt64 => rest | LexLeanBoot.SemanticTypes.SemanticType.SUInt8 => rest | LexLeanBoot.SemanticTypes.SemanticType.SUInt16 => rest | LexLeanBoot.SemanticTypes.SemanticType.SUInt32 => rest | LexLeanBoot.SemanticTypes.SemanticType.SUInt64 => rest | LexLeanBoot.SemanticTypes.SemanticType.SString => rest | LexLeanBoot.SemanticTypes.SemanticType.SBytes => rest | LexLeanBoot.SemanticTypes.SemanticType.SOrdering => rest | LexLeanBoot.SemanticTypes.SemanticType.SContractViolation => rest | LexLeanBoot.SemanticTypes.SemanticType.SReasoningFailure => rest | LexLeanBoot.SemanticTypes.SemanticType.SParameter _ => rest | LexLeanBoot.SemanticTypes.SemanticType.SOption _ => rest | LexLeanBoot.SemanticTypes.SemanticType.SResult _ _ => rest | LexLeanBoot.SemanticTypes.SemanticType.SList _ => rest | LexLeanBoot.SemanticTypes.SemanticType.SNamed _ _ _ => rest | LexLeanBoot.SemanticTypes.SemanticType.SProduct _ _ => rest | LexLeanBoot.SemanticTypes.SemanticType.SFunction _ _ => rest | LexLeanBoot.SemanticTypes.SemanticType.SMap _ _ => rest | LexLeanBoot.SemanticTypes.SemanticType.SSet _ => rest | LexLeanBoot.SemanticTypes.SemanticType.SNil => rest | LexLeanBoot.SemanticTypes.SemanticType.SItem _ _ => member (key) (LexLeanStd.Json.Json.JArr (LexLeanBoot.SemanticTypes.typeJson (types))) (rest))

@[expose] public def termJson : (term : STerm) -> LexLeanStd.Json.Json
  | STerm.TVar named => kinded ("var") (member ("name") (LexLeanStd.Json.Json.JStr (named)) (LexLeanStd.Json.Json.JNil))
  | STerm.TNat digits => kinded ("nat") (member ("value") (LexLeanStd.Json.Json.JStr (digits)) (LexLeanStd.Json.Json.JNil))
  | STerm.TInteger family digits => kinded ("integer") (member ("representation") (LexLeanStd.Json.Json.JStr (LexLeanBoot.SemanticTypes.integerName (family))) (member ("value") (LexLeanStd.Json.Json.JStr (digits)) (LexLeanStd.Json.Json.JNil)))
  | STerm.TString content => kinded ("string") (member ("value") (LexLeanStd.Json.Json.JStr (content)) (LexLeanStd.Json.Json.JNil))
  | STerm.TBytes digits => kinded ("bytes") (member ("hex") (LexLeanStd.Json.Json.JStr (digits)) (LexLeanStd.Json.Json.JNil))
  | STerm.TPrimitive operation arguments outcome => kinded ("primitive") (member ("operation") (LexLeanStd.Json.Json.JStr ((LexLeanBoot.SemanticTypes.primitiveFacts (operation)).serdeName)) (member ("arguments") (LexLeanStd.Json.Json.JArr (termJson (arguments))) (member ("result") (LexLeanBoot.SemanticTypes.typeJson (outcome)) (LexLeanStd.Json.Json.JNil))))
  | STerm.TBool flag => kinded ("bool") (member ("value") (LexLeanStd.Json.Json.JBool (flag)) (LexLeanStd.Json.Json.JNil))
  | STerm.TUnit => kinded ("unit") (LexLeanStd.Json.Json.JNil)
  | STerm.TNil element => kinded ("nil") (member ("element") (LexLeanBoot.SemanticTypes.typeJson (element)) (LexLeanStd.Json.Json.JNil))
  | STerm.TCons head tail => kinded ("cons") (member ("head") (termJson (head)) (member ("tail") (termJson (tail)) (LexLeanStd.Json.Json.JNil)))
  | STerm.TRecord recordRefA recordRefB typeArgs assigned => kinded ("record") (member ("type") (memberJson (recordRefA) (recordRefB)) (typesWhenAny ("type_arguments") (typeArgs) (member ("fields") (LexLeanStd.Json.Json.JArr (termJson (assigned))) (LexLeanStd.Json.Json.JNil))))
  | STerm.TConstructor builtA builtB typeArgs arguments => kinded ("constructor") (member ("constructor") (memberJson (builtA) (builtB)) (typesWhenAny ("type_arguments") (typeArgs) (member ("arguments") (LexLeanStd.Json.Json.JArr (termJson (arguments))) (LexLeanStd.Json.Json.JNil))))
  | STerm.TInstanceValue classRefA classRefB typeArgs resolvedA resolvedB => kinded ("instance_value") (member ("class") (memberJson (classRefA) (classRefB)) (member ("arguments") (LexLeanStd.Json.Json.JArr (LexLeanBoot.SemanticTypes.typeJson (typeArgs))) (member ("resolved") (memberJson (resolvedA) (resolvedB)) (LexLeanStd.Json.Json.JNil))))
  | STerm.TProject subject field => kinded ("project") (member ("value") (termJson (subject)) (member ("field") (LexLeanStd.Json.Json.JStr (field)) (LexLeanStd.Json.Json.JNil)))
  | STerm.TCall functionA functionB typeArgs arguments => kinded ("call") (member ("function") (memberJson (functionA) (functionB)) (typesWhenAny ("type_arguments") (typeArgs) (member ("arguments") (LexLeanStd.Json.Json.JArr (termJson (arguments))) (LexLeanStd.Json.Json.JNil))))
  | STerm.TIf condition whenTrue whenFalse => kinded ("if") (member ("condition") (termJson (condition)) (member ("then_value") (termJson (whenTrue)) (member ("else_value") (termJson (whenFalse)) (LexLeanStd.Json.Json.JNil))))
  | STerm.TMatch scrutinee branches => kinded ("match") (member ("scrutinee") (termJson (scrutinee)) (member ("branches") (LexLeanStd.Json.Json.JArr (termJson (branches))) (LexLeanStd.Json.Json.JNil)))
  | STerm.TEq left right => kinded ("eq") (member ("left") (termJson (left)) (member ("right") (termJson (right)) (LexLeanStd.Json.Json.JNil)))
  | STerm.TLe left right => kinded ("le") (member ("left") (termJson (left)) (member ("right") (termJson (right)) (LexLeanStd.Json.Json.JNil)))
  | STerm.TLt left right => kinded ("lt") (member ("left") (termJson (left)) (member ("right") (termJson (right)) (LexLeanStd.Json.Json.JNil)))
  | STerm.TAdd left right => kinded ("add") (member ("left") (termJson (left)) (member ("right") (termJson (right)) (LexLeanStd.Json.Json.JNil)))
  | STerm.TBeq left right => kinded ("beq") (member ("left") (termJson (left)) (member ("right") (termJson (right)) (LexLeanStd.Json.Json.JNil)))
  | STerm.TBle left right => kinded ("ble") (member ("left") (termJson (left)) (member ("right") (termJson (right)) (LexLeanStd.Json.Json.JNil)))
  | STerm.TBlt left right => kinded ("blt") (member ("left") (termJson (left)) (member ("right") (termJson (right)) (LexLeanStd.Json.Json.JNil)))
  | STerm.TAnd left right => kinded ("and") (member ("left") (termJson (left)) (member ("right") (termJson (right)) (LexLeanStd.Json.Json.JNil)))
  | STerm.TPropAnd left right => kinded ("prop_and") (member ("left") (termJson (left)) (member ("right") (termJson (right)) (LexLeanStd.Json.Json.JNil)))
  | STerm.TOr left right => kinded ("or") (member ("left") (termJson (left)) (member ("right") (termJson (right)) (LexLeanStd.Json.Json.JNil)))
  | STerm.TNot subject => kinded ("not") (member ("value") (termJson (subject)) (LexLeanStd.Json.Json.JNil))
  | STerm.TImplies premise conclusion => kinded ("implies") (member ("premise") (termJson (premise)) (member ("conclusion") (termJson (conclusion)) (LexLeanStd.Json.Json.JNil)))
  | STerm.TIff left right => kinded ("iff") (member ("left") (termJson (left)) (member ("right") (termJson (right)) (LexLeanStd.Json.Json.JNil)))
  | STerm.TForall binderA binderB body => kinded ("forall") (member ("binder") (paramJson (binderA) (binderB)) (member ("body") (termJson (body)) (LexLeanStd.Json.Json.JNil)))
  | STerm.TLet binderA binderB bound body => kinded ("let") (member ("binder") (paramJson (binderA) (binderB)) (member ("value") (termJson (bound)) (member ("body") (termJson (body)) (LexLeanStd.Json.Json.JNil))))
  | STerm.TPair left right => kinded ("pair") (member ("left") (termJson (left)) (member ("right") (termJson (right)) (LexLeanStd.Json.Json.JNil)))
  | STerm.TFirst subject => kinded ("first") (member ("value") (termJson (subject)) (LexLeanStd.Json.Json.JNil))
  | STerm.TSecond subject => kinded ("second") (member ("value") (termJson (subject)) (LexLeanStd.Json.Json.JNil))
  | STerm.TLambda parameters captures body => kinded ("lambda") (member ("parameters") (paramsJson (parameters)) (member ("captures") (namesJson (captures)) (member ("body") (termJson (body)) (LexLeanStd.Json.Json.JNil))))
  | STerm.TApply function arguments => kinded ("apply") (member ("function") (termJson (function)) (member ("arguments") (LexLeanStd.Json.Json.JArr (termJson (arguments))) (LexLeanStd.Json.Json.JNil)))
  | STerm.TFunctionRef functionA functionB typeArgs => kinded ("function_ref") (member ("function") (memberJson (functionA) (functionB)) (typesWhenAny ("type_arguments") (typeArgs) (LexLeanStd.Json.Json.JNil)))
  | STerm.TMapLiteral keyType valueType entries => kinded ("map_literal") (member ("key") (LexLeanBoot.SemanticTypes.typeJson (keyType)) (member ("value") (LexLeanBoot.SemanticTypes.typeJson (valueType)) (member ("entries") (LexLeanStd.Json.Json.JArr (termJson (entries))) (LexLeanStd.Json.Json.JNil))))
  | STerm.TSetLiteral element elements => kinded ("set_literal") (member ("element") (LexLeanBoot.SemanticTypes.typeJson (element)) (member ("elements") (LexLeanStd.Json.Json.JArr (termJson (elements))) (LexLeanStd.Json.Json.JNil)))
  | STerm.TGraphLiteral node nodes edges => kinded ("graph_literal") (member ("node") (LexLeanBoot.SemanticTypes.typeJson (node)) (member ("nodes") (LexLeanStd.Json.Json.JArr (termJson (nodes))) (member ("edges") (LexLeanStd.Json.Json.JArr (termJson (edges))) (LexLeanStd.Json.Json.JNil))))
  | STerm.TCheckedApply modelA modelB typeArgs arguments checks => kinded ("checked_apply") (member ("model") (memberJson (modelA) (modelB)) (typesWhenAny ("type_arguments") (typeArgs) (member ("arguments") (LexLeanStd.Json.Json.JArr (termJson (arguments))) (member ("checks") (namesJson (checks)) (LexLeanStd.Json.Json.JNil)))))
  | STerm.KEnd => LexLeanStd.Json.Json.JNil
  | STerm.KItem head tail => LexLeanStd.Json.Json.JItem (termJson (head)) (termJson (tail))
  | STerm.KAssign field assigned tail => LexLeanStd.Json.Json.JItem (LexLeanStd.Json.Json.JObj (member ("field") (LexLeanStd.Json.Json.JStr (field)) (member ("value") (termJson (assigned)) (LexLeanStd.Json.Json.JNil)))) (termJson (tail))
  | STerm.KBranch owner named bound body tail => LexLeanStd.Json.Json.JItem (LexLeanStd.Json.Json.JObj (member ("constructor") (memberJson (owner) (named)) (member ("binders") (namesJson (bound)) (member ("body") (termJson (body)) (LexLeanStd.Json.Json.JNil))))) (termJson (tail))
  | STerm.KEntry key entry tail => LexLeanStd.Json.Json.JItem (LexLeanStd.Json.Json.JObj (member ("key") (termJson (key)) (member ("value") (termJson (entry)) (LexLeanStd.Json.Json.JNil)))) (termJson (tail))
  | STerm.KEdge source target tail => LexLeanStd.Json.Json.JItem (LexLeanStd.Json.Json.JObj (member ("source") (termJson (source)) (member ("target") (termJson (target)) (LexLeanStd.Json.Json.JNil)))) (termJson (tail))

@[expose] public def reflectionJson (reflection : Reflection) : LexLeanStd.Json.Json := (match reflection with | Reflection.RList parameter values booleanA booleanB propositionA propositionB comparison => kinded ("list") (member ("parameter") (LexLeanStd.Json.Json.JStr (parameter)) (member ("values") (LexLeanStd.Json.Json.JStr (values)) (member ("boolean_definition") (memberJson (booleanA) (booleanB)) (member ("proposition_definition") (memberJson (propositionA) (propositionB)) (member ("comparison") (LexLeanStd.Json.Json.JStr (comparison)) (LexLeanStd.Json.Json.JNil)))))) | Reflection.RRecord recordName booleanA booleanB propositionA propositionB fields => kinded ("record") (member ("record") (LexLeanStd.Json.Json.JStr (recordName)) (member ("boolean_definition") (memberJson (booleanA) (booleanB)) (member ("proposition_definition") (memberJson (propositionA) (propositionB)) (member ("fields") (LexLeanStd.Json.Json.JArr (chainOf ((LexLeanCollections.listFold ((fun (acc : List (LexLeanStd.Json.Json)) (entry : (Prod (ByteArray) (ByteArray))) => (LexLeanStd.Json.Json.JObj (member ("field") (LexLeanStd.Json.Json.JStr ((entry).1)) (member ("expected") (LexLeanStd.Json.Json.JStr ((entry).2)) (LexLeanStd.Json.Json.JNil))) :: acc))) (([] : List (LexLeanStd.Json.Json))) (LexLeanStd.Order.reverseList ((Prod (ByteArray) (ByteArray))) (fields)) : List (LexLeanStd.Json.Json))))) (LexLeanStd.Json.Json.JNil))))))

@[expose] public def proofJson : (proof : SProof) -> LexLeanStd.Json.Json
  | SProof.PReflexivity => kinded ("reflexivity") (LexLeanStd.Json.Json.JNil)
  | SProof.PDecide => kinded ("decide") (LexLeanStd.Json.Json.JNil)
  | SProof.PLinearArithmetic definitions => kinded ("linear_arithmetic") ((match definitions with | List.nil => LexLeanStd.Json.Json.JNil | List.cons _ _ => member ("definitions") (membersJson (definitions)) (LexLeanStd.Json.Json.JNil)))
  | SProof.PSimplify definitions => kinded ("simplify") (member ("definitions") (membersJson (definitions)) (LexLeanStd.Json.Json.JNil))
  | SProof.PConstructor branches => kinded ("constructor") (member ("branches") (LexLeanStd.Json.Json.JArr (proofJson (branches))) (LexLeanStd.Json.Json.JNil))
  | SProof.PCases scrutinee branches => kinded ("cases") (member ("scrutinee") (LexLeanStd.Json.Json.JStr (scrutinee)) (member ("branches") (LexLeanStd.Json.Json.JArr (proofJson (branches))) (LexLeanStd.Json.Json.JNil)))
  | SProof.PTermCases scrutinee definitions => kinded ("term_cases") (member ("scrutinee") (termJson (scrutinee)) (member ("definitions") (membersJson (definitions)) (LexLeanStd.Json.Json.JNil)))
  | SProof.PInduction scrutinee generalized branches => kinded ("induction") (member ("scrutinee") (LexLeanStd.Json.Json.JStr (scrutinee)) ((match generalized with | List.nil => member ("branches") (LexLeanStd.Json.Json.JArr (proofJson (branches))) (LexLeanStd.Json.Json.JNil) | List.cons _ _ => member ("generalizing") (namesJson (generalized)) (member ("branches") (LexLeanStd.Json.Json.JArr (proofJson (branches))) (LexLeanStd.Json.Json.JNil)))))
  | SProof.PNestedInduction scrutinee branches nested => kinded ("nested_induction") (member ("scrutinee") (LexLeanStd.Json.Json.JStr (scrutinee)) (member ("branches") (LexLeanStd.Json.Json.JArr (proofJson (branches))) (member ("members") (LexLeanStd.Json.Json.JArr (proofJson (nested))) (LexLeanStd.Json.Json.JNil))))
  | SProof.PCongruence => kinded ("congruence") (LexLeanStd.Json.Json.JNil)
  | SProof.PBooleanReflection reflection => kinded ("boolean_reflection") (member ("reflection") (reflectionJson (reflection)) (LexLeanStd.Json.Json.JNil))
  | SProof.PLaw lawName typeArgs arguments premises => kinded ("law") (member ("name") (LexLeanStd.Json.Json.JStr (lawName)) (typesWhenAny ("type_arguments") (typeArgs) (member ("arguments") (LexLeanStd.Json.Json.JArr (termJson (arguments))) ((match premises with | List.nil => LexLeanStd.Json.Json.JNil | List.cons _ _ => member ("premises") (membersJson (premises)) (LexLeanStd.Json.Json.JNil))))))
  | SProof.PApply theoremRefA theoremRefB typeArgs arguments => kinded ("apply") (member ("theorem") (memberJson (theoremRefA) (theoremRefB)) (typesWhenAny ("type_arguments") (typeArgs) (member ("arguments") (LexLeanStd.Json.Json.JArr (termJson (arguments))) (LexLeanStd.Json.Json.JNil))))
  | SProof.QEnd => LexLeanStd.Json.Json.JNil
  | SProof.QItem head tail => LexLeanStd.Json.Json.JItem (proofJson (head)) (proofJson (tail))
  | SProof.QBranch built bound proved tail => LexLeanStd.Json.Json.JItem (LexLeanStd.Json.Json.JObj (member ("constructor") (LexLeanStd.Json.Json.JStr (built)) (member ("binders") (namesJson (bound)) (member ("proof") (proofJson (proved)) (LexLeanStd.Json.Json.JNil))))) (proofJson (tail))
  | SProof.QMember typed bound statement branches tail => LexLeanStd.Json.Json.JItem (LexLeanStd.Json.Json.JObj (member ("type") (LexLeanBoot.SemanticTypes.typeJson (typed)) (member ("binder") (LexLeanStd.Json.Json.JStr (bound)) (member ("statement") (termJson (statement)) (member ("branches") (LexLeanStd.Json.Json.JArr (proofJson (branches))) (LexLeanStd.Json.Json.JNil)))))) (proofJson (tail))

@[expose] public def semanticTermText (term : STerm) : ByteArray := LexLeanStd.Json.writeJson (termJson (term))

@[expose] public def semanticProofText (proof : SProof) : ByteArray := LexLeanStd.Json.writeJson (proofJson (proof))

end LexLeanBoot.SemanticTerms
