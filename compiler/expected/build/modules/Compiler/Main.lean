module
public import Init
public import Compiler.Gnaf
public import Compiler.GnafFixtures
public import Compiler.TargetFixtures
public import Compiler.TargetOracle
public import Compiler.TargetSemantics
public import Compiler.TargetSyntax
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
namespace Compiler.Main

@[expose] public def emptyProgram : Compiler.TargetSyntax.Program := ({ adts := ([] : List (Compiler.TargetSyntax.Adt)), functions := ([] : List (Compiler.TargetSyntax.Function)) } : Compiler.TargetSyntax.Program)

end Compiler.Main
