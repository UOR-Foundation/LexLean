module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Numeral
public import LexLeanStd.Order
public import LexLeanStd.Text
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.LockText

public structure LockPackage where
  ident : ByteArray
  version : ByteArray
  kind : ByteArray
  origin : ByteArray
  revision : ByteArray
  treeSha256 : ByteArray
  manifestSha256 : ByteArray
  imports : List (ByteArray)

public structure LockSourcePackage where
  ident : ByteArray
  version : ByteArray
  kind : ByteArray
  origin : ByteArray
  modulePrefix : ByteArray
  treeSha256 : ByteArray
  manifestSha256 : ByteArray
  imports : List (ByteArray)

public structure LockPdf where
  program : ByteArray
  programSha256 : ByteArray
  versionArgv : List (ByteArray)
  versionStdoutSha256 : ByteArray
  compileArgv : List (ByteArray)
  output : ByteArray
  resources : List ((Prod (ByteArray) (ByteArray)))

public structure Lock where
  language : ByteArray
  compilerSemantics : ByteArray
  projectConfigSha256 : ByteArray
  workspaceFiles : List ((Prod (ByteArray) (ByteArray)))
  packages : List (LockPackage)
  sourcePackages : List (LockSourcePackage)
  pdf : Option (LockPdf)

@[expose] public def lit (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def cat (left : ByteArray) (right : ByteArray) : ByteArray := (LexLeanRuntime.append (left) (right) : ByteArray)

@[expose] public def leanToolchain : ByteArray := lit ("leanprover/lean4:v4.32.1")

@[expose] public def upperHex (value : Nat) : ByteArray := LexLeanStd.Numeral.pick (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 48)), (_root_.UInt8.ofNat (nat_lit 49)), (_root_.UInt8.ofNat (nat_lit 50)), (_root_.UInt8.ofNat (nat_lit 51)), (_root_.UInt8.ofNat (nat_lit 52)), (_root_.UInt8.ofNat (nat_lit 53)), (_root_.UInt8.ofNat (nat_lit 54)), (_root_.UInt8.ofNat (nat_lit 55)), (_root_.UInt8.ofNat (nat_lit 56)), (_root_.UInt8.ofNat (nat_lit 57)), (_root_.UInt8.ofNat (nat_lit 65)), (_root_.UInt8.ofNat (nat_lit 66)), (_root_.UInt8.ofNat (nat_lit 67)), (_root_.UInt8.ofNat (nat_lit 68)), (_root_.UInt8.ofNat (nat_lit 69)), (_root_.UInt8.ofNat (nat_lit 70))]) (value)

@[expose] public def escapeByte (value : Nat) : ByteArray := (if (Nat.beq (value) (34)) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 92)), (_root_.UInt8.ofNat (nat_lit 34))] else (if (Nat.beq (value) (92)) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 92)), (_root_.UInt8.ofNat (nat_lit 92))] else (if (if (Nat.blt (value) (32)) then true else (Nat.beq (value) (127))) then cat (cat (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 92)), (_root_.UInt8.ofNat (nat_lit 117)), (_root_.UInt8.ofNat (nat_lit 48)), (_root_.UInt8.ofNat (nat_lit 48))]) (upperHex ((LexLeanRuntime.quotient (value) (16) (0) : Nat)))) (upperHex ((LexLeanRuntime.remainder (value) (16) (0) : Nat))) else LexLeanStd.Numeral.pick (LexLeanStd.Numeral.byteValues) (value))))

@[expose] public def escapeStep (subject : ByteArray) (state : (Prod (Nat) (ByteArray))) : Option ((Prod (Nat) (ByteArray))) := (match LexLeanStd.Text.byteAt (subject) ((state).1) with | Option.some byte => Option.some ((((state).1 + 1), cat ((state).2) (escapeByte ((let __widen0 : UInt8 := byte; ((((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((128 : UInt8)) : UInt8)) ((128 : UInt8)) : Bool) then 128 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((64 : UInt8)) : UInt8)) ((64 : UInt8)) : Bool) then 64 else 0)) + ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((32 : UInt8)) : UInt8)) ((32 : UInt8)) : Bool) then 32 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((16 : UInt8)) : UInt8)) ((16 : UInt8)) : Bool) then 16 else 0))) + (((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((8 : UInt8)) : UInt8)) ((8 : UInt8)) : Bool) then 8 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((4 : UInt8)) : UInt8)) ((4 : UInt8)) : Bool) then 4 else 0)) + ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((2 : UInt8)) : UInt8)) ((2 : UInt8)) : Bool) then 2 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((1 : UInt8)) : UInt8)) ((1 : UInt8)) : Bool) then 1 else 0))))))))) | Option.none => Option.none)

@[expose] public def tomlString (value : ByteArray) : ByteArray := cat (cat (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 34))]) ((((LexLeanCollections.iterateUntil ((fun (state : (Prod (Nat) (ByteArray))) => escapeStep (value) (state))) (((LexLeanRuntime.length (value) : Nat) + 1)) ((0, _root_.ByteArray.mk #[])) : (Prod ((Prod (Nat) (ByteArray))) (Bool)))).1).2)) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 34))])

@[expose] public def tomlArray (values : List (ByteArray)) : ByteArray := cat (cat (lit ("[")) (LexLeanStd.Text.joinWith (LexLeanStd.Order.reverseList (ByteArray) ((LexLeanCollections.listFold ((fun (acc : List (ByteArray)) (item : ByteArray) => (tomlString (item) :: acc))) (([] : List (ByteArray))) (values) : List (ByteArray)))) (lit (", ")))) (lit ("]"))

@[expose] public def line (key : ByteArray) (shown : ByteArray) : ByteArray := cat (cat (cat (key) (lit (" = "))) (shown)) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 10))])

@[expose] public def stringLine (key : ByteArray) (value : ByteArray) : ByteArray := line (key) (tomlString (value))

@[expose] public def digestLine (key : ByteArray) (digest : ByteArray) : ByteArray := line (key) (tomlString (LexLeanStd.Numeral.hex (digest)))

@[expose] public def header (name : ByteArray) : ByteArray := cat (cat (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 10))]) (name)) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 10))])

@[expose] public def escapedKey (text : ByteArray) : ByteArray := cat (LexLeanStd.Text.joinWith (LexLeanStd.Text.splitOn (text) ((0 : UInt8))) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 0)), (_root_.UInt8.ofNat (nat_lit 1))])) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 0)), (_root_.UInt8.ofNat (nat_lit 0))])

@[expose] public def pairsByPath (rows : List ((Prod (ByteArray) (ByteArray)))) : List ((Prod (ByteArray) (ByteArray))) := (LexLeanCollections.listFold ((fun (acc : List ((Prod (ByteArray) (ByteArray)))) (entry : (Prod (ByteArray) ((Prod (ByteArray) (ByteArray))))) => ((entry).2 :: acc))) (([] : List ((Prod (ByteArray) (ByteArray))))) (LexLeanStd.Order.reverseList ((Prod (ByteArray) ((Prod (ByteArray) (ByteArray))))) (LexLeanStd.Order.sortByKey ((Prod (ByteArray) (ByteArray))) (LexLeanStd.Order.reverseList ((Prod (ByteArray) ((Prod (ByteArray) (ByteArray))))) ((LexLeanCollections.listFold ((fun (acc : List ((Prod (ByteArray) ((Prod (ByteArray) (ByteArray)))))) (row : (Prod (ByteArray) (ByteArray))) => (((row).1, row) :: acc))) (([] : List ((Prod (ByteArray) ((Prod (ByteArray) (ByteArray))))))) (rows) : List ((Prod (ByteArray) ((Prod (ByteArray) (ByteArray)))))))))) : List ((Prod (ByteArray) (ByteArray))))

@[expose] public def packageKey (row : LockPackage) : ByteArray := cat (escapedKey ((row).ident)) (escapedKey ((row).version))

@[expose] public def packagesSorted (rows : List (LockPackage)) : List (LockPackage) := (LexLeanCollections.listFold ((fun (acc : List (LockPackage)) (entry : (Prod (ByteArray) (LockPackage))) => ((entry).2 :: acc))) (([] : List (LockPackage))) (LexLeanStd.Order.reverseList ((Prod (ByteArray) (LockPackage))) (LexLeanStd.Order.sortByKey (LockPackage) (LexLeanStd.Order.reverseList ((Prod (ByteArray) (LockPackage))) ((LexLeanCollections.listFold ((fun (acc : List ((Prod (ByteArray) (LockPackage)))) (row : LockPackage) => ((packageKey (row), row) :: acc))) (([] : List ((Prod (ByteArray) (LockPackage))))) (rows) : List ((Prod (ByteArray) (LockPackage)))))))) : List (LockPackage))

@[expose] public def sourcesSorted (rows : List (LockSourcePackage)) : List (LockSourcePackage) := (LexLeanCollections.listFold ((fun (acc : List (LockSourcePackage)) (entry : (Prod (ByteArray) (LockSourcePackage))) => ((entry).2 :: acc))) (([] : List (LockSourcePackage))) (LexLeanStd.Order.reverseList ((Prod (ByteArray) (LockSourcePackage))) (LexLeanStd.Order.sortByKey (LockSourcePackage) (LexLeanStd.Order.reverseList ((Prod (ByteArray) (LockSourcePackage))) ((LexLeanCollections.listFold ((fun (acc : List ((Prod (ByteArray) (LockSourcePackage)))) (row : LockSourcePackage) => (((row).ident, row) :: acc))) (([] : List ((Prod (ByteArray) (LockSourcePackage))))) (rows) : List ((Prod (ByteArray) (LockSourcePackage)))))))) : List (LockSourcePackage))

@[expose] public def pinText (table : ByteArray) (rows : List ((Prod (ByteArray) (ByteArray)))) : ByteArray := (LexLeanCollections.listFold ((fun (acc : ByteArray) (row : (Prod (ByteArray) (ByteArray))) => cat (acc) (cat (cat (header (table)) (stringLine (lit ("path")) ((row).1))) (digestLine (lit ("sha256")) ((row).2))))) (_root_.ByteArray.mk #[]) (pairsByPath (rows)) : ByteArray)

@[expose] public def packageText (row : LockPackage) : ByteArray := cat (cat (cat (cat (cat (cat (cat (cat (header (lit ("[[package]]"))) (stringLine (lit ("id")) ((row).ident))) (stringLine (lit ("version")) ((row).version))) (stringLine (lit ("kind")) ((row).kind))) (stringLine (lit ("source")) ((row).origin))) (stringLine (lit ("revision")) ((row).revision))) (digestLine (lit ("tree_sha256")) ((row).treeSha256))) (digestLine (lit ("manifest_sha256")) ((row).manifestSha256))) (line (lit ("imports")) (tomlArray ((row).imports)))

@[expose] public def sourceText (row : LockSourcePackage) : ByteArray := cat (cat (cat (cat (cat (cat (cat (cat (header (lit ("[[source_package]]"))) (stringLine (lit ("id")) ((row).ident))) (stringLine (lit ("version")) ((row).version))) (stringLine (lit ("kind")) ((row).kind))) (stringLine (lit ("source")) ((row).origin))) (stringLine (lit ("module_prefix")) ((row).modulePrefix))) (digestLine (lit ("tree_sha256")) ((row).treeSha256))) (digestLine (lit ("manifest_sha256")) ((row).manifestSha256))) (line (lit ("imports")) (tomlArray ((row).imports)))

@[expose] public def pdfText (entry : LockPdf) : ByteArray := cat (cat (cat (cat (cat (cat (cat (header (lit ("[pdf]"))) (stringLine (lit ("program")) ((entry).program))) (digestLine (lit ("program_sha256")) ((entry).programSha256))) (line (lit ("version_argv")) (tomlArray ((entry).versionArgv)))) (digestLine (lit ("version_stdout_sha256")) ((entry).versionStdoutSha256))) (line (lit ("compile_argv")) (tomlArray ((entry).compileArgv)))) (stringLine (lit ("output")) ((entry).output))) (pinText (lit ("[[pdf.resource]]")) ((entry).resources))

@[expose] public def canonicalLock (given : Lock) : ByteArray := (let spec : ByteArray := (if (LexLeanRuntime.equal ((given).language) (lit ("1.2")) : Bool) then lit ("lexlean/lock/2") else lit ("lexlean/lock/1")); (let head : ByteArray := cat (cat (cat (cat (stringLine (lit ("spec")) (spec)) (stringLine (lit ("language")) ((given).language))) (digestLine (lit ("compiler_semantics")) ((given).compilerSemantics))) (digestLine (lit ("project_config_sha256")) ((given).projectConfigSha256))) (stringLine (lit ("lean_toolchain")) (leanToolchain)); (let pins : ByteArray := pinText (lit ("[[workspace_file]]")) ((given).workspaceFiles); (let rows : ByteArray := (LexLeanCollections.listFold ((fun (acc : ByteArray) (row : LockPackage) => cat (acc) (packageText (row)))) (_root_.ByteArray.mk #[]) (packagesSorted ((given).packages)) : ByteArray); (let sources : ByteArray := (LexLeanCollections.listFold ((fun (acc : ByteArray) (row : LockSourcePackage) => cat (acc) (sourceText (row)))) (_root_.ByteArray.mk #[]) (sourcesSorted ((given).sourcePackages)) : ByteArray); (let tail : ByteArray := (match (given).pdf with | Option.some entry => pdfText (entry) | Option.none => _root_.ByteArray.mk #[]); cat (cat (cat (cat (head) (pins)) (rows)) (sources)) (tail)))))))

end LexLeanBoot.LockText
