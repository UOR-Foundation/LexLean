module
public import Init
public import LexLeanTarget.BudgetOracle
public import LexLeanTarget.Gnaf
public import LexLeanTarget.GnafFixtures
public import LexLeanTarget.GradeOracle
public import LexLeanTarget.PlannerOracle
public import LexLeanTarget.ReasoningFixtures
public import LexLeanTarget.ReasoningOracle
public import LexLeanTarget.RustSemantics
public import LexLeanTarget.RustSyntax
public import LexLeanTarget.ScreeningOracle
public import LexLeanTarget.TargetFixtures
public import LexLeanTarget.TargetOracle
public import LexLeanTarget.TargetSemantics
public import LexLeanTarget.TargetSyntax
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanTarget.Main

@[expose] public def emptyProgram : LexLeanTarget.TargetSyntax.Program := ({ adts := ([] : List (LexLeanTarget.TargetSyntax.Adt)), functions := ([] : List (LexLeanTarget.TargetSyntax.Function)) } : LexLeanTarget.TargetSyntax.Program)

end LexLeanTarget.Main
