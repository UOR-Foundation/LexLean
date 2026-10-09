import LexLeanPreservation.Core
import LexLeanPreservation.Values
import LexLeanPreservation.Primitives
import LexLeanPreservation.Fixed
import LexLeanPreservation.Keys
import LexLeanPreservation.Templates
import LexLeanPreservation.Validate
import Coverage.Boundary
import Coverage.Colls
import Coverage.Main
import Coverage.Prims
import Coverage.RecRoots
import Coverage.Recur
import Coverage.Syntax
import Coverage.Types
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R17

def __prog : LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0], types := [(.pair .nat .bool)], result := (.pair .bool .nat),
      body := (.build .pair (.pair .bool .nat) [(.second (.var 0)), (.first (.var 0))]) }] }

def __fits_0 (p : (Prod Nat Bool)) : Bool :=
  ((Bool.true && (Bool.true && Bool.true)) && Bool.true)

theorem __rel_0 (p : (Prod Nat Bool)) : LexLeanPreservation.FunRel __prog 0 [((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.bool) p)] (LexLeanPreservation.Rel (__fits_0 p) ((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.bool LexLeanTarget.TargetSyntax.Value.nat) (Coverage.Main.pairs p))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_second (LexLeanPreservation.conv_var rfl)) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_first (LexLeanPreservation.conv_var rfl)) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair)

def denote (p : (Prod Nat Bool)) : LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 p) (LexLeanPreservation.Obs.value (((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.bool LexLeanTarget.TargetSyntax.Value.nat) (Coverage.Main.pairs p)))) LexLeanPreservation.Obs.overflow

theorem root (p : (Prod Nat Bool)) : LexLeanPreservation.RunConv __prog 0 [((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.bool) p)] (LexLeanPreservation.Rel (__fits_0 p) ((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.bool LexLeanTarget.TargetSyntax.Value.nat) (Coverage.Main.pairs p))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 p)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R17
