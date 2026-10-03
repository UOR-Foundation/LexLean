module
public import Init
public import LexLeanTarget.Gnaf
public import LexLeanTarget.GnafFixtures
public import LexLeanTarget.RustSemantics
public import LexLeanTarget.RustSyntax
public import LexLeanTarget.TargetFixtures
public import LexLeanTarget.TargetOracle
public import LexLeanTarget.TargetSemantics
public import LexLeanTarget.TargetSyntax
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
namespace LexLeanTarget.Main

@[expose] public def emptyProgram : LexLeanTarget.TargetSyntax.Program := ({ adts := ([] : List (LexLeanTarget.TargetSyntax.Adt)), functions := ([] : List (LexLeanTarget.TargetSyntax.Function)) } : LexLeanTarget.TargetSyntax.Program)

end LexLeanTarget.Main
