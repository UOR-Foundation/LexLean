module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Json
public import LexLeanStd.Order
public import LexLeanStd.Text
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.SemanticTypes

public inductive SemanticType where
  | SType
  | SNat
  | SBool
  | SProp
  | SUnit
  | SInt
  | SInt8
  | SInt16
  | SInt32
  | SInt64
  | SUInt8
  | SUInt16
  | SUInt32
  | SUInt64
  | SString
  | SBytes
  | SOrdering
  | SContractViolation
  | SReasoningFailure
  | SParameter (_ : ByteArray)
  | SOption (_ : SemanticType)
  | SResult (_ : SemanticType) (_ : SemanticType)
  | SList (_ : SemanticType)
  | SNamed (_ : Option (ByteArray)) (_ : ByteArray) (_ : SemanticType)
  | SProduct (_ : SemanticType) (_ : SemanticType)
  | SFunction (_ : SemanticType) (_ : SemanticType)
  | SMap (_ : SemanticType) (_ : SemanticType)
  | SSet (_ : SemanticType)
  | SNil
  | SItem (_ : SemanticType) (_ : SemanticType)

public inductive SemanticPrimitive where
  | PSubtract
  | PMultiply
  | PQuotient
  | PRemainder
  | PNegate
  | PCheckedConvert
  | PCheckedAdd
  | PCheckedSubtract
  | PCheckedMultiply
  | PCheckedNegate
  | PCheckedQuotient
  | PBitAnd
  | PBitOr
  | PBitXor
  | PBitNot
  | PShiftLeft
  | PShiftRight
  | PAppend
  | PLength
  | PIndex
  | PSlice
  | PUtf8Encode
  | PUtf8Decode
  | PCompareBytes
  | PEqual
  | PSplitExact
  | PJoin
  | PParseDecimal
  | PFormatDecimal
  | PMapInsert
  | PMapRemove
  | PMapLookup
  | PMapContains
  | PMapSize
  | PMapKeys
  | PMapValues
  | PMapEntries
  | PMapFold
  | PSetInsert
  | PSetRemove
  | PSetContains
  | PSetSize
  | PSetElements
  | PSetUnion
  | PSetIntersection
  | PSetDifference
  | PSetFold
  | PListFold
  | PIterate
  | PIterateUntil
  | PGraphSuccessors
  | PGraphReachable
  | PGraphTopological
  | PLessThan
  | PToInt
  | PToNat

public inductive SemanticInteger where
  | IInt
  | IInt8
  | IInt16
  | IInt32
  | IInt64
  | IUInt8
  | IUInt16
  | IUInt32
  | IUInt64

public structure PrimitiveFacts where
  serdeName : ByteArray
  languageOneTwo : Bool
  widening : Bool
  functionArgument : Option (Nat)

@[expose] public def lit (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def cat (left : ByteArray) (right : ByteArray) : ByteArray := (LexLeanRuntime.append (left) (right) : ByteArray)

@[expose] public def member (key : String) (entry : LexLeanStd.Json.Json) (rest : LexLeanStd.Json.Json) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JMember (lit (key)) (entry) (rest)

@[expose] public def kinded (kind : String) (rest : LexLeanStd.Json.Json) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JObj (member ("kind") (LexLeanStd.Json.Json.JStr (lit (kind))) (rest))

@[expose] public def typeJson : (semantic : SemanticType) -> LexLeanStd.Json.Json
  | SemanticType.SType => kinded ("type") (LexLeanStd.Json.Json.JNil)
  | SemanticType.SNat => kinded ("nat") (LexLeanStd.Json.Json.JNil)
  | SemanticType.SBool => kinded ("bool") (LexLeanStd.Json.Json.JNil)
  | SemanticType.SProp => kinded ("prop") (LexLeanStd.Json.Json.JNil)
  | SemanticType.SUnit => kinded ("unit") (LexLeanStd.Json.Json.JNil)
  | SemanticType.SInt => kinded ("int") (LexLeanStd.Json.Json.JNil)
  | SemanticType.SInt8 => kinded ("int8") (LexLeanStd.Json.Json.JNil)
  | SemanticType.SInt16 => kinded ("int16") (LexLeanStd.Json.Json.JNil)
  | SemanticType.SInt32 => kinded ("int32") (LexLeanStd.Json.Json.JNil)
  | SemanticType.SInt64 => kinded ("int64") (LexLeanStd.Json.Json.JNil)
  | SemanticType.SUInt8 => kinded ("uint8") (LexLeanStd.Json.Json.JNil)
  | SemanticType.SUInt16 => kinded ("uint16") (LexLeanStd.Json.Json.JNil)
  | SemanticType.SUInt32 => kinded ("uint32") (LexLeanStd.Json.Json.JNil)
  | SemanticType.SUInt64 => kinded ("uint64") (LexLeanStd.Json.Json.JNil)
  | SemanticType.SString => kinded ("string") (LexLeanStd.Json.Json.JNil)
  | SemanticType.SBytes => kinded ("bytes") (LexLeanStd.Json.Json.JNil)
  | SemanticType.SOrdering => kinded ("ordering") (LexLeanStd.Json.Json.JNil)
  | SemanticType.SContractViolation => kinded ("contract_violation") (LexLeanStd.Json.Json.JNil)
  | SemanticType.SReasoningFailure => kinded ("reasoning_failure") (LexLeanStd.Json.Json.JNil)
  | SemanticType.SParameter named => kinded ("parameter") (member ("name") (LexLeanStd.Json.Json.JStr (named)) (LexLeanStd.Json.Json.JNil))
  | SemanticType.SOption inner => kinded ("option") (member ("value") (typeJson (inner)) (LexLeanStd.Json.Json.JNil))
  | SemanticType.SResult good bad => kinded ("result") (member ("ok") (typeJson (good)) (member ("error") (typeJson (bad)) (LexLeanStd.Json.Json.JNil)))
  | SemanticType.SList inner => kinded ("list") (member ("element") (typeJson (inner)) (LexLeanStd.Json.Json.JNil))
  | SemanticType.SNamed owner named arguments => kinded ("named") (member ("member") (LexLeanStd.Json.Json.JObj ((match owner with | Option.some source => member ("module") (LexLeanStd.Json.Json.JStr (source)) (member ("name") (LexLeanStd.Json.Json.JStr (named)) (LexLeanStd.Json.Json.JNil)) | Option.none => member ("name") (LexLeanStd.Json.Json.JStr (named)) (LexLeanStd.Json.Json.JNil)))) (member ("arguments") (LexLeanStd.Json.Json.JArr (typeJson (arguments))) (LexLeanStd.Json.Json.JNil)))
  | SemanticType.SProduct left right => kinded ("product") (member ("left") (typeJson (left)) (member ("right") (typeJson (right)) (LexLeanStd.Json.Json.JNil)))
  | SemanticType.SFunction parameters outcome => kinded ("function") (member ("parameters") (LexLeanStd.Json.Json.JArr (typeJson (parameters))) (member ("result") (typeJson (outcome)) (LexLeanStd.Json.Json.JNil)))
  | SemanticType.SMap key entry => kinded ("map") (member ("key") (typeJson (key)) (member ("value") (typeJson (entry)) (LexLeanStd.Json.Json.JNil)))
  | SemanticType.SSet inner => kinded ("set") (member ("element") (typeJson (inner)) (LexLeanStd.Json.Json.JNil))
  | SemanticType.SNil => LexLeanStd.Json.Json.JNil
  | SemanticType.SItem head tail => LexLeanStd.Json.Json.JItem (typeJson (head)) (typeJson (tail))

mutual
@[expose] public def typeDisplay : (semantic : SemanticType) -> ByteArray
  | SemanticType.SType => lit ("Type")
  | SemanticType.SNat => lit ("Nat")
  | SemanticType.SBool => lit ("Bool")
  | SemanticType.SProp => lit ("Prop")
  | SemanticType.SUnit => lit ("Unit")
  | SemanticType.SInt => lit ("Int")
  | SemanticType.SInt8 => lit ("Int8")
  | SemanticType.SInt16 => lit ("Int16")
  | SemanticType.SInt32 => lit ("Int32")
  | SemanticType.SInt64 => lit ("Int64")
  | SemanticType.SUInt8 => lit ("UInt8")
  | SemanticType.SUInt16 => lit ("UInt16")
  | SemanticType.SUInt32 => lit ("UInt32")
  | SemanticType.SUInt64 => lit ("UInt64")
  | SemanticType.SString => lit ("String")
  | SemanticType.SBytes => lit ("Bytes")
  | SemanticType.SOrdering => lit ("Ordering")
  | SemanticType.SContractViolation => lit ("ContractViolation")
  | SemanticType.SReasoningFailure => lit ("ReasoningFailure")
  | SemanticType.SParameter named => named
  | SemanticType.SOption inner => cat (lit ("Option (")) (cat (typeDisplay (inner)) (lit (")")))
  | SemanticType.SResult good bad => cat (cat (lit ("Result (")) (typeDisplay (good))) (cat (cat (lit (") (")) (typeDisplay (bad))) (lit (")")))
  | SemanticType.SList inner => cat (lit ("List (")) (cat (typeDisplay (inner)) (lit (")")))
  | SemanticType.SNamed owner named arguments => cat ((match owner with | Option.some source => cat (cat (source) (lit ("."))) (named) | Option.none => named)) ((LexLeanCollections.listFold ((fun (acc : ByteArray) (shown : ByteArray) => cat (acc) (cat (lit (" (")) (cat (shown) (lit (")")))))) (_root_.ByteArray.mk #[]) (displayItems (arguments)) : ByteArray))
  | SemanticType.SProduct left right => cat (cat (lit ("Prod (")) (typeDisplay (left))) (cat (cat (lit (") (")) (typeDisplay (right))) (lit (")")))
  | SemanticType.SFunction parameters outcome => cat (cat (lit ("(")) ((LexLeanCollections.listFold ((fun (acc : ByteArray) (shown : ByteArray) => cat (acc) (cat (lit ("(")) (cat (shown) (lit (") -> ")))))) (_root_.ByteArray.mk #[]) (displayItems (parameters)) : ByteArray))) (cat (lit ("(")) (cat (typeDisplay (outcome)) (lit ("))"))))
  | SemanticType.SMap key entry => cat (cat (lit ("Map (")) (typeDisplay (key))) (cat (cat (lit (") (")) (typeDisplay (entry))) (lit (")")))
  | SemanticType.SSet inner => cat (lit ("Set (")) (cat (typeDisplay (inner)) (lit (")")))
  | SemanticType.SNil => _root_.ByteArray.mk #[]
  | SemanticType.SItem _ _ => _root_.ByteArray.mk #[]
termination_by structural semantic => semantic

@[expose] public def displayItems : (semantic : SemanticType) -> List (ByteArray)
  | SemanticType.SType => ([] : List (ByteArray))
  | SemanticType.SNat => ([] : List (ByteArray))
  | SemanticType.SBool => ([] : List (ByteArray))
  | SemanticType.SProp => ([] : List (ByteArray))
  | SemanticType.SUnit => ([] : List (ByteArray))
  | SemanticType.SInt => ([] : List (ByteArray))
  | SemanticType.SInt8 => ([] : List (ByteArray))
  | SemanticType.SInt16 => ([] : List (ByteArray))
  | SemanticType.SInt32 => ([] : List (ByteArray))
  | SemanticType.SInt64 => ([] : List (ByteArray))
  | SemanticType.SUInt8 => ([] : List (ByteArray))
  | SemanticType.SUInt16 => ([] : List (ByteArray))
  | SemanticType.SUInt32 => ([] : List (ByteArray))
  | SemanticType.SUInt64 => ([] : List (ByteArray))
  | SemanticType.SString => ([] : List (ByteArray))
  | SemanticType.SBytes => ([] : List (ByteArray))
  | SemanticType.SOrdering => ([] : List (ByteArray))
  | SemanticType.SContractViolation => ([] : List (ByteArray))
  | SemanticType.SReasoningFailure => ([] : List (ByteArray))
  | SemanticType.SParameter _ => ([] : List (ByteArray))
  | SemanticType.SOption _ => ([] : List (ByteArray))
  | SemanticType.SResult _ _ => ([] : List (ByteArray))
  | SemanticType.SList _ => ([] : List (ByteArray))
  | SemanticType.SNamed _ _ _ => ([] : List (ByteArray))
  | SemanticType.SProduct _ _ => ([] : List (ByteArray))
  | SemanticType.SFunction _ _ => ([] : List (ByteArray))
  | SemanticType.SMap _ _ => ([] : List (ByteArray))
  | SemanticType.SSet _ => ([] : List (ByteArray))
  | SemanticType.SNil => ([] : List (ByteArray))
  | SemanticType.SItem head tail => (typeDisplay (head) :: displayItems (tail))
termination_by structural semantic => semantic
end

@[expose] public def primitiveFacts (primitive : SemanticPrimitive) : PrimitiveFacts := (match primitive with | SemanticPrimitive.PSubtract => ({ serdeName := lit ("subtract"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PMultiply => ({ serdeName := lit ("multiply"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PQuotient => ({ serdeName := lit ("quotient"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PRemainder => ({ serdeName := lit ("remainder"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PNegate => ({ serdeName := lit ("negate"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PCheckedConvert => ({ serdeName := lit ("checked_convert"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PCheckedAdd => ({ serdeName := lit ("checked_add"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PCheckedSubtract => ({ serdeName := lit ("checked_subtract"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PCheckedMultiply => ({ serdeName := lit ("checked_multiply"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PCheckedNegate => ({ serdeName := lit ("checked_negate"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PCheckedQuotient => ({ serdeName := lit ("checked_quotient"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PBitAnd => ({ serdeName := lit ("bit_and"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PBitOr => ({ serdeName := lit ("bit_or"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PBitXor => ({ serdeName := lit ("bit_xor"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PBitNot => ({ serdeName := lit ("bit_not"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PShiftLeft => ({ serdeName := lit ("shift_left"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PShiftRight => ({ serdeName := lit ("shift_right"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PAppend => ({ serdeName := lit ("append"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PLength => ({ serdeName := lit ("length"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PIndex => ({ serdeName := lit ("index"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PSlice => ({ serdeName := lit ("slice"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PUtf8Encode => ({ serdeName := lit ("utf8_encode"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PUtf8Decode => ({ serdeName := lit ("utf8_decode"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PCompareBytes => ({ serdeName := lit ("compare_bytes"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PEqual => ({ serdeName := lit ("equal"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PSplitExact => ({ serdeName := lit ("split_exact"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PJoin => ({ serdeName := lit ("join"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PParseDecimal => ({ serdeName := lit ("parse_decimal"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PFormatDecimal => ({ serdeName := lit ("format_decimal"), languageOneTwo := false, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PMapInsert => ({ serdeName := lit ("map_insert"), languageOneTwo := true, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PMapRemove => ({ serdeName := lit ("map_remove"), languageOneTwo := true, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PMapLookup => ({ serdeName := lit ("map_lookup"), languageOneTwo := true, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PMapContains => ({ serdeName := lit ("map_contains"), languageOneTwo := true, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PMapSize => ({ serdeName := lit ("map_size"), languageOneTwo := true, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PMapKeys => ({ serdeName := lit ("map_keys"), languageOneTwo := true, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PMapValues => ({ serdeName := lit ("map_values"), languageOneTwo := true, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PMapEntries => ({ serdeName := lit ("map_entries"), languageOneTwo := true, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PMapFold => ({ serdeName := lit ("map_fold"), languageOneTwo := true, widening := false, functionArgument := Option.some (0) } : PrimitiveFacts) | SemanticPrimitive.PSetInsert => ({ serdeName := lit ("set_insert"), languageOneTwo := true, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PSetRemove => ({ serdeName := lit ("set_remove"), languageOneTwo := true, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PSetContains => ({ serdeName := lit ("set_contains"), languageOneTwo := true, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PSetSize => ({ serdeName := lit ("set_size"), languageOneTwo := true, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PSetElements => ({ serdeName := lit ("set_elements"), languageOneTwo := true, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PSetUnion => ({ serdeName := lit ("set_union"), languageOneTwo := true, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PSetIntersection => ({ serdeName := lit ("set_intersection"), languageOneTwo := true, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PSetDifference => ({ serdeName := lit ("set_difference"), languageOneTwo := true, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PSetFold => ({ serdeName := lit ("set_fold"), languageOneTwo := true, widening := false, functionArgument := Option.some (0) } : PrimitiveFacts) | SemanticPrimitive.PListFold => ({ serdeName := lit ("list_fold"), languageOneTwo := true, widening := false, functionArgument := Option.some (0) } : PrimitiveFacts) | SemanticPrimitive.PIterate => ({ serdeName := lit ("iterate"), languageOneTwo := true, widening := false, functionArgument := Option.some (0) } : PrimitiveFacts) | SemanticPrimitive.PIterateUntil => ({ serdeName := lit ("iterate_until"), languageOneTwo := true, widening := false, functionArgument := Option.some (0) } : PrimitiveFacts) | SemanticPrimitive.PGraphSuccessors => ({ serdeName := lit ("graph_successors"), languageOneTwo := true, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PGraphReachable => ({ serdeName := lit ("graph_reachable"), languageOneTwo := true, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PGraphTopological => ({ serdeName := lit ("graph_topological"), languageOneTwo := true, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PLessThan => ({ serdeName := lit ("less_than"), languageOneTwo := true, widening := false, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PToInt => ({ serdeName := lit ("to_int"), languageOneTwo := true, widening := true, functionArgument := Option.none } : PrimitiveFacts) | SemanticPrimitive.PToNat => ({ serdeName := lit ("to_nat"), languageOneTwo := true, widening := true, functionArgument := Option.none } : PrimitiveFacts))

@[expose] public def integerName (integer : SemanticInteger) : ByteArray := (match integer with | SemanticInteger.IInt => lit ("int") | SemanticInteger.IInt8 => lit ("int8") | SemanticInteger.IInt16 => lit ("int16") | SemanticInteger.IInt32 => lit ("int32") | SemanticInteger.IInt64 => lit ("int64") | SemanticInteger.IUInt8 => lit ("uint8") | SemanticInteger.IUInt16 => lit ("uint16") | SemanticInteger.IUInt32 => lit ("uint32") | SemanticInteger.IUInt64 => lit ("uint64"))

@[expose] public def typeText (semantic : SemanticType) : ByteArray := LexLeanStd.Json.writeJson (typeJson (semantic))

@[expose] public def typeSpelling (semantic : SemanticType) : ByteArray := typeDisplay (semantic)

end LexLeanBoot.SemanticTypes
