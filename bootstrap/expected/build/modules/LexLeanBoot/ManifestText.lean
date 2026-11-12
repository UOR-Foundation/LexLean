module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Json
public import LexLeanStd.Numeral
public import LexLeanStd.Order
public import LexLeanStd.Text
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.ManifestText

public structure FileRow where
  kind : ByteArray
  location : ByteArray
  byteLength : Nat
  digest : ByteArray

public structure ModuleRow where
  moduleName : ByteArray
  leanModule : ByteArray
  sourcePath : ByteArray
  origin : Option (ByteArray)

public structure BuildManifest where
  language : ByteArray
  compilerVersion : ByteArray
  semanticsId : ByteArray
  projectName : ByteArray
  sourceId : ByteArray
  semanticId : ByteArray
  buildId : ByteArray
  selection : List (ByteArray)
  modules : List (ModuleRow)
  inputs : List (FileRow)
  outputs : List (FileRow)

@[expose] public def lit (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def cat (left : ByteArray) (right : ByteArray) : ByteArray := (LexLeanRuntime.append (left) (right) : ByteArray)

@[expose] public def jsonText (value : ByteArray) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JStr (value)

@[expose] public def jsonDigest (digest : ByteArray) : LexLeanStd.Json.Json := jsonText (LexLeanStd.Numeral.hex (digest))

@[expose] public def member (key : String) (value : LexLeanStd.Json.Json) (rest : LexLeanStd.Json.Json) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JMember (lit (key)) (value) (rest)

@[expose] public def textArray (values : List (ByteArray)) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JArr ((LexLeanCollections.listFold ((fun (acc : LexLeanStd.Json.Json) (item : ByteArray) => LexLeanStd.Json.Json.JItem (jsonText (item)) (acc))) (LexLeanStd.Json.Json.JNil) (LexLeanStd.Order.reverseList (ByteArray) (values)) : LexLeanStd.Json.Json))

@[expose] public def bigEndian (value : Nat) (width : Nat) : ByteArray := ((LexLeanCollections.iterate ((fun (state : (Prod (Nat) (ByteArray))) => ((LexLeanRuntime.quotient ((state).1) (256) (0) : Nat), cat (LexLeanStd.Numeral.pick (LexLeanStd.Numeral.byteValues) ((LexLeanRuntime.remainder ((state).1) (256) (0) : Nat))) ((state).2)))) (width) ((value, _root_.ByteArray.mk #[])) : (Prod (Nat) (ByteArray)))).2

@[expose] public def escapedKey (text : ByteArray) : ByteArray := cat (LexLeanStd.Text.joinWith (LexLeanStd.Text.splitOn (text) ((0 : UInt8))) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 0)), (_root_.UInt8.ofNat (nat_lit 1))])) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 0)), (_root_.UInt8.ofNat (nat_lit 0))])

@[expose] public def rowKey (row : FileRow) : ByteArray := cat (cat (cat (escapedKey ((row).kind)) (escapedKey ((row).location))) (bigEndian ((row).byteLength) (8))) ((row).digest)

@[expose] public def rowsSorted (rows : List (FileRow)) : List (FileRow) := (LexLeanCollections.listFold ((fun (acc : List (FileRow)) (entry : (Prod (ByteArray) (FileRow))) => ((entry).2 :: acc))) (([] : List (FileRow))) (LexLeanStd.Order.reverseList ((Prod (ByteArray) (FileRow))) (LexLeanStd.Order.sortByKey (FileRow) (LexLeanStd.Order.reverseList ((Prod (ByteArray) (FileRow))) ((LexLeanCollections.listFold ((fun (acc : List ((Prod (ByteArray) (FileRow)))) (row : FileRow) => ((rowKey (row), row) :: acc))) (([] : List ((Prod (ByteArray) (FileRow))))) (rows) : List ((Prod (ByteArray) (FileRow)))))))) : List (FileRow))

@[expose] public def rowJson (row : FileRow) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JObj (member ("kind") (jsonText ((row).kind)) (member ("path") (jsonText ((row).location)) (member ("byte_length") (LexLeanStd.Json.Json.JInt (false) ((row).byteLength)) (member ("sha256") (jsonDigest ((row).digest)) (LexLeanStd.Json.Json.JNil)))))

@[expose] public def rowsJson (rows : List (FileRow)) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JArr ((LexLeanCollections.listFold ((fun (acc : LexLeanStd.Json.Json) (row : FileRow) => LexLeanStd.Json.Json.JItem (rowJson (row)) (acc))) (LexLeanStd.Json.Json.JNil) (LexLeanStd.Order.reverseList (FileRow) (rowsSorted (rows))) : LexLeanStd.Json.Json))

@[expose] public def moduleJson (entry : ModuleRow) : LexLeanStd.Json.Json := (let tail : LexLeanStd.Json.Json := (match (entry).origin with | Option.some named => member ("origin") (jsonText (named)) (LexLeanStd.Json.Json.JNil) | Option.none => LexLeanStd.Json.Json.JNil); LexLeanStd.Json.Json.JObj (member ("module") (jsonText ((entry).moduleName)) (member ("lean_module") (jsonText ((entry).leanModule)) (member ("source_path") (jsonText ((entry).sourcePath)) (tail)))))

@[expose] public def modulesJson (rows : List (ModuleRow)) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JArr ((LexLeanCollections.listFold ((fun (acc : LexLeanStd.Json.Json) (entry : ModuleRow) => LexLeanStd.Json.Json.JItem (moduleJson (entry)) (acc))) (LexLeanStd.Json.Json.JNil) (LexLeanStd.Order.reverseList (ModuleRow) (rows)) : LexLeanStd.Json.Json))

@[expose] public def manifestJson (manifest : BuildManifest) : LexLeanStd.Json.Json := LexLeanStd.Json.Json.JObj (member ("spec") (jsonText (lit ("lexlean/build-manifest/1"))) (member ("compiler") (LexLeanStd.Json.Json.JObj (member ("version") (jsonText ((manifest).compilerVersion)) (member ("semantics_id") (jsonDigest ((manifest).semanticsId)) (LexLeanStd.Json.Json.JNil)))) (member ("language") (jsonText ((manifest).language)) (member ("project") (jsonText ((manifest).projectName)) (member ("source_id") (jsonDigest ((manifest).sourceId)) (member ("semantic_id") (jsonDigest ((manifest).semanticId)) (member ("build_id") (jsonDigest ((manifest).buildId)) (member ("lean_toolchain") (jsonText (lit ("leanprover/lean4:v4.32.1"))) (member ("selection") (textArray ((manifest).selection)) (member ("modules") (modulesJson ((manifest).modules)) (member ("inputs") (rowsJson ((manifest).inputs)) (member ("outputs") (rowsJson ((manifest).outputs)) (LexLeanStd.Json.Json.JNil)))))))))))))

@[expose] public def buildManifest (manifest : BuildManifest) : ByteArray := LexLeanStd.Json.writeJsonFile (manifestJson (manifest))

end LexLeanBoot.ManifestText
