module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanBoot.Diagnostics
public import LexLeanStd.Numeral
public import LexLeanStd.Order
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.CliOutput

public structure Palette where
  boldRed : ByteArray
  bold : ByteArray
  cyan : ByteArray
  reset : ByteArray

@[expose] public def lit (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def cat (left : ByteArray) (right : ByteArray) : ByteArray := (LexLeanRuntime.append (left) (right) : ByteArray)

@[expose] public def paletteOf (color : Bool) : Palette := (if color then ({ boldRed := _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 27)), (_root_.UInt8.ofNat (nat_lit 91)), (_root_.UInt8.ofNat (nat_lit 49)), (_root_.UInt8.ofNat (nat_lit 59)), (_root_.UInt8.ofNat (nat_lit 51)), (_root_.UInt8.ofNat (nat_lit 49)), (_root_.UInt8.ofNat (nat_lit 109))], bold := _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 27)), (_root_.UInt8.ofNat (nat_lit 91)), (_root_.UInt8.ofNat (nat_lit 49)), (_root_.UInt8.ofNat (nat_lit 109))], cyan := _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 27)), (_root_.UInt8.ofNat (nat_lit 91)), (_root_.UInt8.ofNat (nat_lit 51)), (_root_.UInt8.ofNat (nat_lit 54)), (_root_.UInt8.ofNat (nat_lit 109))], reset := _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 27)), (_root_.UInt8.ofNat (nat_lit 91)), (_root_.UInt8.ofNat (nat_lit 48)), (_root_.UInt8.ofNat (nat_lit 109))] } : Palette) else ({ boldRed := _root_.ByteArray.mk #[], bold := _root_.ByteArray.mk #[], cyan := _root_.ByteArray.mk #[], reset := _root_.ByteArray.mk #[] } : Palette))

@[expose] public def place (span : LexLeanBoot.Diagnostics.Span) : ByteArray := cat (cat (cat (cat ((span).path) (lit (":"))) (LexLeanStd.Numeral.decimal ((span).lineStart))) (lit (":"))) (LexLeanStd.Numeral.decimal ((span).columnStart))

@[expose] public def lines («prefix» : ByteArray) (items : List (ByteArray)) : ByteArray := (LexLeanCollections.listFold ((fun (acc : ByteArray) (item : ByteArray) => cat (acc) (cat (cat («prefix») (item)) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 10))])))) (_root_.ByteArray.mk #[]) (items) : ByteArray)

@[expose] public def labelLines (items : List (LexLeanBoot.Diagnostics.Label)) : ByteArray := (LexLeanCollections.listFold ((fun (acc : ByteArray) (item : LexLeanBoot.Diagnostics.Label) => cat (acc) (cat (cat (cat (cat (lit ("  label: ")) ((item).message)) (lit (" ("))) (place ((item).span))) (cat (lit (")")) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 10))]))))) (_root_.ByteArray.mk #[]) (items) : ByteArray)

@[expose] public def noteMessages (items : List (LexLeanBoot.Diagnostics.Note)) : List (ByteArray) := LexLeanStd.Order.reverseList (ByteArray) ((LexLeanCollections.listFold ((fun (acc : List (ByteArray)) (item : LexLeanBoot.Diagnostics.Note) => ((item).message :: acc))) (([] : List (ByteArray))) (items) : List (ByteArray)))

@[expose] public def humanOne (colors : Palette) (given : LexLeanBoot.Diagnostics.Diagnostic) : ByteArray := (let head : ByteArray := cat (cat (cat (cat (cat (cat (cat ((colors).boldRed) (lit ("error"))) ((colors).reset)) (lit ("["))) ((given).code)) (lit ("]: "))) (cat ((colors).bold) ((given).message))) ((colors).reset); (let located : ByteArray := (match (given).primary with | Option.some span => cat (cat (cat (cat (cat (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 10))]) (lit ("  "))) ((colors).cyan)) (lit ("-->"))) ((colors).reset)) (cat (lit (" ")) (place (span))) | Option.none => _root_.ByteArray.mk #[]); cat (cat (cat (cat (cat (cat (head) (located)) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 10))])) (labelLines ((given).labels))) (lines (lit ("  note: ")) (noteMessages ((given).notes)))) (lines (lit ("  help: ")) ((given).help))) (lines (lit ("  cause: ")) ((given).causes))))

@[expose] public def humanDiagnostics (items : List (LexLeanBoot.Diagnostics.Diagnostic)) (color : Bool) : ByteArray := (let colors : Palette := paletteOf (color); (LexLeanCollections.listFold ((fun (acc : ByteArray) (item : LexLeanBoot.Diagnostics.Diagnostic) => cat (acc) (humanOne (colors) (item)))) (_root_.ByteArray.mk #[]) (items) : ByteArray))

@[expose] public def jsonStep (state : (Prod (Bool) (Option (ByteArray)))) (argument : ByteArray) : (Prod (Bool) (Option (ByteArray))) := (if (state).1 then state else (if (LexLeanRuntime.equal (argument) (lit ("--diagnostic-format=json")) : Bool) then (true, Option.some (argument)) else (match (state).2 with | Option.some previous => (if (if (LexLeanRuntime.equal (previous) (lit ("--diagnostic-format")) : Bool) then (LexLeanRuntime.equal (argument) (lit ("json")) : Bool) else false) then (true, Option.some (argument)) else (false, Option.some (argument))) | Option.none => (false, Option.some (argument)))))

@[expose] public def afterFirst (arguments : List (ByteArray)) : List (ByteArray) := (match arguments with | List.nil => ([] : List (ByteArray)) | List.cons _ rest => rest)

@[expose] public def wantsJson (arguments : List (ByteArray)) : Bool := ((LexLeanCollections.listFold ((fun (state : (Prod (Bool) (Option (ByteArray)))) (argument : ByteArray) => jsonStep (state) (argument))) ((false, Option.none)) (afterFirst (arguments)) : (Prod (Bool) (Option (ByteArray))))).1

@[expose] public def commandNames : List (ByteArray) := (lit ("init") :: (lit ("lock") :: (lit ("check") :: (lit ("build") :: (lit ("verify") :: (lit ("fmt") :: (lit ("migrate") :: (lit ("clean") :: (lit ("explain") :: (lit ("production") :: ([] : List (ByteArray))))))))))))

@[expose] public def isCommand (argument : ByteArray) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (known : ByteArray) => (if found then true else (LexLeanRuntime.equal (argument) (known) : Bool)))) (false) (commandNames) : Bool)

@[expose] public def namedCommand (arguments : List (ByteArray)) : ByteArray := (match (LexLeanCollections.listFold ((fun (found : Option (ByteArray)) (argument : ByteArray) => (match found with | Option.some _ => found | Option.none => (if isCommand (argument) then Option.some (argument) else Option.none)))) (Option.none) (afterFirst (arguments)) : Option (ByteArray)) with | Option.some known => known | Option.none => _root_.ByteArray.mk #[])

end LexLeanBoot.CliOutput
