module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanKappa.Blake3
public import LexLeanKappa.Json
public import LexLeanKappa.Keccak
public import LexLeanKappa.Label
public import LexLeanStd.Sha256
public import LexLeanStd.Text
public import LexLeanKappa.Sha512
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanKappa.Axis

@[expose] public def named (axis : ByteArray) (name : String) : Bool := (LexLeanRuntime.equal (axis) (LexLeanKappa.Label.text (name)) : Bool)

@[expose] public def width (axis : ByteArray) : Nat := (if named (axis) ("sha256") then 64 else (if named (axis) ("sha3-256") then 64 else (if named (axis) ("keccak256") then 64 else (if named (axis) ("sha512") then 128 else (if named (axis) ("blake3") then 64 else 0)))))

@[expose] public def digestHex (axis : ByteArray) (message : ByteArray) : Option (ByteArray) := (if named (axis) ("sha256") then Option.some (LexLeanStd.Sha256.sha256Hex (message)) else (if named (axis) ("sha3-256") then Option.some (LexLeanKappa.Keccak.sha3Hex (message)) else (if named (axis) ("keccak256") then Option.some (LexLeanKappa.Keccak.keccakHex (message)) else (if named (axis) ("sha512") then Option.some (LexLeanKappa.Sha512.sha512Hex (message)) else (if named (axis) ("blake3") then Option.some (LexLeanKappa.Blake3.blake3Hex (message)) else Option.none)))))

@[expose] public def prefixOf (axis : ByteArray) : ByteArray := (LexLeanRuntime.append (axis) (LexLeanKappa.Label.text (":")) : ByteArray)

@[expose] public def labelOn (axis : ByteArray) (canonical : ByteArray) : Option (ByteArray) := (match digestHex (axis) (canonical) with | Option.some digest => Option.some ((LexLeanRuntime.append (prefixOf (axis)) (digest) : ByteArray)) | Option.none => Option.none)

@[expose] public def isLabelOn (axis : ByteArray) (candidate : ByteArray) : Bool := (if (Nat.blt (0) (width (axis))) then (if (Nat.beq ((LexLeanRuntime.length (candidate) : Nat)) (((LexLeanRuntime.length (prefixOf (axis)) : Nat) + width (axis)))) then (if LexLeanStd.Text.startsWith (candidate) (prefixOf (axis)) then (Nat.beq (LexLeanKappa.Label.lowerHexRun (candidate) ((LexLeanRuntime.length (prefixOf (axis)) : Nat))) ((LexLeanRuntime.length (candidate) : Nat))) else false) else false) else false)

@[expose] public def jsonLabelOn (axis : ByteArray) (input : ByteArray) : Option (ByteArray) := (match LexLeanKappa.Json.canonicalResult (input) with | Except.ok canonical => labelOn (axis) (canonical) | Except.error _ => Option.none)

end LexLeanKappa.Axis
