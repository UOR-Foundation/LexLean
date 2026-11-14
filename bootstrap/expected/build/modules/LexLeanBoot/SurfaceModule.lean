module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanBoot.AxiomOutput
public import LexLeanStd.Text
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.SurfaceModule

public structure Located where
  kind : Nat
  bodyStart : Nat
  bodyStop : Nat
  data : Option ((Prod (Nat) (Nat)))

@[expose] public def lit (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def environment (kind : Nat) : ByteArray := (if (Nat.beq (kind) (0)) then lit ("semanticmodule") else (if (Nat.beq (kind) (1)) then lit ("toolingmodule") else lit ("coremodule")))

@[expose] public def control (kind : Nat) : ByteArray := (if (Nat.beq (kind) (0)) then lit ("\\semanticdata") else (if (Nat.beq (kind) (1)) then lit ("\\toolingdata") else lit ("\\coredata")))

@[expose] public def tail (subject : ByteArray) (offset : Nat) : ByteArray := LexLeanStd.Text.sliceOr (subject) (offset) ((LexLeanRuntime.subtract ((LexLeanRuntime.length (subject) : Nat)) (offset) : Nat))

@[expose] public def dataRange (text : ByteArray) (kind : Nat) (start : Nat) (stop : Nat) : Option ((Prod (Nat) (Nat))) := (let region : ByteArray := LexLeanStd.Text.sliceOr (text) (start) ((LexLeanRuntime.subtract (stop) (start) : Nat)); (let lead : Nat := LexLeanBoot.AxiomOutput.skipWhite (region); (let tag : ByteArray := control (kind); (if LexLeanStd.Text.startsWith (tail (region) (lead)) (tag) then (let afterTag : Nat := (lead + (LexLeanRuntime.length (tag) : Nat)); (let brace : Nat := (afterTag + LexLeanBoot.AxiomOutput.skipWhite (tail (region) (afterTag))); (if LexLeanStd.Text.startsWith (tail (region) (brace)) (lit ("{")) then (let inside : ByteArray := tail (region) ((brace + 1)); (let trimmedStop : Nat := LexLeanBoot.AxiomOutput.dropWhiteEnd (inside) (0); (if (if (Nat.blt (0) (trimmedStop)) then LexLeanStd.Text.matchAt (inside) (lit ("}")) ((LexLeanRuntime.subtract (trimmedStop) (1) : Nat)) else false) then Option.some ((((start + brace) + 1), (((start + brace) + 1) + (LexLeanRuntime.subtract (trimmedStop) (1) : Nat)))) else Option.none))) else Option.none))) else Option.none))))

@[expose] public def isLetter (text : ByteArray) (position : Nat) : Bool := (if LexLeanBoot.AxiomOutput.byteIn (text) (position) ((65 : UInt8)) ((90 : UInt8)) then true else LexLeanBoot.AxiomOutput.byteIn (text) (position) ((97 : UInt8)) ((122 : UInt8)))

@[expose] public def opensAtom (text : ByteArray) (position : Nat) : Bool := (if LexLeanBoot.AxiomOutput.byteIs (text) (position) ((92 : UInt8)) then LexLeanBoot.AxiomOutput.byteIn (text) ((position + 1)) ((0 : UInt8)) ((127 : UInt8)) else false)

@[expose] public def atomFrom (text : ByteArray) («at» : Nat) : Nat := ((LexLeanCollections.iterateUntil ((fun (position : Nat) => (if opensAtom (text) (position) then Option.none else (if (Nat.blt (position) ((LexLeanRuntime.length (text) : Nat))) then Option.some ((position + 1)) else Option.none)))) (((LexLeanRuntime.length (text) : Nat) + 1)) («at») : (Prod (Nat) (Bool)))).1

@[expose] public def lettersEnd (text : ByteArray) («at» : Nat) : Nat := ((LexLeanCollections.iterateUntil ((fun (position : Nat) => (if isLetter (text) (position) then Option.some ((position + 1)) else Option.none))) (((LexLeanRuntime.length (text) : Nat) + 1)) («at») : (Prod (Nat) (Bool)))).1

@[expose] public def nextAtom (text : ByteArray) («at» : Nat) : Option ((Prod (Nat) (Nat))) := (let start : Nat := atomFrom (text) («at»); (if opensAtom (text) (start) then (if isLetter (text) ((start + 1)) then Option.some ((start, lettersEnd (text) ((start + 1)))) else Option.some ((start, (start + 2)))) else Option.none))

@[expose] public def isControl (text : ByteArray) (atom : (Prod (Nat) (Nat))) (word : ByteArray) : Bool := (match (LexLeanRuntime.slice (text) ((atom).1) ((LexLeanRuntime.subtract ((atom).2) ((atom).1) : Nat)) : Option (ByteArray)) with | Option.some window => (LexLeanRuntime.equal (window) (word) : Bool) | Option.none => false)

@[expose] public def nameAt (text : ByteArray) («at» : Nat) : Option ((Prod (Nat) (Nat))) := (if LexLeanBoot.AxiomOutput.byteIs (text) («at») ((123 : UInt8)) then (match LexLeanStd.Text.find (tail (text) ((«at» + 1))) (lit ("}")) with | Option.some offset => Option.some (((«at» + 1), ((«at» + 1) + offset))) | Option.none => Option.none) else Option.none)

@[expose] public def nameBytes (text : ByteArray) (name : (Prod (Nat) (Nat))) : ByteArray := LexLeanStd.Text.sliceOr (text) ((name).1) ((LexLeanRuntime.subtract ((name).2) ((name).1) : Nat))

@[expose] public def kindOf (name : ByteArray) : Option (Nat) := (if (LexLeanRuntime.equal (name) (environment (0)) : Bool) then Option.some (0) else (if (LexLeanRuntime.equal (name) (environment (1)) : Bool) then Option.some (1) else (if (LexLeanRuntime.equal (name) (environment (2)) : Bool) then Option.some (2) else Option.none)))

@[expose] public def openStep (text : ByteArray) («at» : Nat) : Option (Nat) := (match nextAtom (text) («at») with | Option.none => Option.none | Option.some atom => (if isControl (text) (atom) (lit ("\\begin")) then (match nameAt (text) ((atom).2) with | Option.none => Option.none | Option.some name => (if (LexLeanRuntime.equal (nameBytes (text) (name)) (lit ("lexlean")) : Bool) then Option.some ((atom).2) else Option.none)) else Option.some ((atom).2)))

@[expose] public def opening (text : ByteArray) : Option ((Prod (Nat) ((Prod (Nat) (Nat))))) := (let «at» : Nat := ((LexLeanCollections.iterateUntil ((fun («at» : Nat) => openStep (text) («at»))) (((LexLeanRuntime.length (text) : Nat) + 1)) (0) : (Prod (Nat) (Bool)))).1; (match nextAtom (text) («at») with | Option.none => Option.none | Option.some atom => (if isControl (text) (atom) (lit ("\\begin")) then (match nameAt (text) ((atom).2) with | Option.none => Option.none | Option.some name => (match kindOf (nameBytes (text) (name)) with | Option.none => Option.none | Option.some kind => Option.some ((kind, (((name).2 + 1), (atom).2))))) else Option.none)))

@[expose] public def closeStep (text : ByteArray) («at» : Nat) : Option (Nat) := (match nextAtom (text) («at») with | Option.none => Option.none | Option.some atom => (if isControl (text) (atom) (lit ("\\end")) then Option.none else Option.some ((atom).2)))

@[expose] public def closing (text : ByteArray) (kind : Nat) («from» : Nat) : Option (Nat) := (let «at» : Nat := ((LexLeanCollections.iterateUntil ((fun («at» : Nat) => closeStep (text) («at»))) (((LexLeanRuntime.length (text) : Nat) + 1)) («from») : (Prod (Nat) (Bool)))).1; (match nextAtom (text) («at») with | Option.none => Option.none | Option.some atom => (if isControl (text) (atom) (lit ("\\end")) then (match nameAt (text) ((atom).2) with | Option.none => Option.none | Option.some name => (if (LexLeanRuntime.equal (nameBytes (text) (name)) (environment (kind)) : Bool) then Option.some ((atom).1) else Option.none)) else Option.none)))

@[expose] public def locateModule (text : ByteArray) : Option (Located) := (match opening (text) with | Option.none => Option.none | Option.some found => (let kind : Nat := (found).1; (let start : Nat := ((found).2).1; (match closing (text) (kind) (((found).2).2) with | Option.none => Option.none | Option.some stop => (if (Nat.blt (stop) (start)) then Option.none else Option.some (({ kind := kind, bodyStart := start, bodyStop := stop, data := dataRange (text) (kind) (start) (stop) } : Located)))))))

end LexLeanBoot.SurfaceModule
