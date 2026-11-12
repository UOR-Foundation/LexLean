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

@[expose] public def beginMarker (kind : Nat) : ByteArray := (LexLeanRuntime.append ((LexLeanRuntime.append (lit ("\\begin{")) (environment (kind)) : ByteArray)) (lit ("}")) : ByteArray)

@[expose] public def endMarker (kind : Nat) : ByteArray := (LexLeanRuntime.append ((LexLeanRuntime.append (lit ("\\end{")) (environment (kind)) : ByteArray)) (lit ("}")) : ByteArray)

@[expose] public def lastAt (subject : ByteArray) (needle : ByteArray) : Option (Nat) := (let total : Nat := (LexLeanRuntime.length (subject) : Nat); (let size : Nat := (LexLeanRuntime.length (needle) : Nat); (if (Nat.blt (total) (size)) then Option.none else (let position : Nat := ((LexLeanCollections.iterateUntil ((fun (position : Nat) => LexLeanBoot.AxiomOutput.lastStep (subject) (needle) (position))) ((total + 1)) ((LexLeanRuntime.subtract (total) (size) : Nat)) : (Prod (Nat) (Bool)))).1; (if LexLeanStd.Text.matchAt (subject) (needle) (position) then Option.some (position) else Option.none)))))

@[expose] public def tail (subject : ByteArray) (offset : Nat) : ByteArray := LexLeanStd.Text.sliceOr (subject) (offset) ((LexLeanRuntime.subtract ((LexLeanRuntime.length (subject) : Nat)) (offset) : Nat))

@[expose] public def dataRange (text : ByteArray) (kind : Nat) (start : Nat) (stop : Nat) : Option ((Prod (Nat) (Nat))) := (let region : ByteArray := LexLeanStd.Text.sliceOr (text) (start) ((LexLeanRuntime.subtract (stop) (start) : Nat)); (let lead : Nat := LexLeanBoot.AxiomOutput.skipWhite (region); (let tag : ByteArray := control (kind); (if LexLeanStd.Text.startsWith (tail (region) (lead)) (tag) then (let afterTag : Nat := (lead + (LexLeanRuntime.length (tag) : Nat)); (let brace : Nat := (afterTag + LexLeanBoot.AxiomOutput.skipWhite (tail (region) (afterTag))); (if LexLeanStd.Text.startsWith (tail (region) (brace)) (lit ("{")) then (let inside : ByteArray := tail (region) ((brace + 1)); (let trimmedStop : Nat := LexLeanBoot.AxiomOutput.dropWhiteEnd (inside) (0); (if (if (Nat.blt (0) (trimmedStop)) then LexLeanStd.Text.matchAt (inside) (lit ("}")) ((LexLeanRuntime.subtract (trimmedStop) (1) : Nat)) else false) then Option.some ((((start + brace) + 1), (((start + brace) + 1) + (LexLeanRuntime.subtract (trimmedStop) (1) : Nat)))) else Option.none))) else Option.none))) else Option.none))))

@[expose] public def tryKind (text : ByteArray) (kind : Nat) : (Prod (Nat) (Option (Located))) := (let marker : ByteArray := beginMarker (kind); (match LexLeanStd.Text.find (text) (marker) with | Option.none => (0, Option.none) | Option.some found => (let start : Nat := (found + (LexLeanRuntime.length (marker) : Nat)); (match lastAt (text) (endMarker (kind)) with | Option.none => (1, Option.none) | Option.some stop => (if (Nat.blt (stop) (start)) then (1, Option.none) else (2, Option.some (({ kind := kind, bodyStart := start, bodyStop := stop, data := dataRange (text) (kind) (start) (stop) } : Located))))))))

@[expose] public def locateFrom (text : ByteArray) (kind : Nat) : Option (Located) := (let tried : (Prod (Nat) (Option (Located))) := tryKind (text) (kind); (if (Nat.beq ((tried).1) (0)) then Option.none else (tried).2))

@[expose] public def locateModule (text : ByteArray) : Option (Located) := (let semantic : (Prod (Nat) (Option (Located))) := tryKind (text) (0); (if (Nat.beq ((semantic).1) (0)) then (let tooling : (Prod (Nat) (Option (Located))) := tryKind (text) (1); (if (Nat.beq ((tooling).1) (0)) then locateFrom (text) (2) else (tooling).2)) else (semantic).2))

end LexLeanBoot.SurfaceModule
