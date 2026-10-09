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
namespace LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R17

def __prog : _root_.LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0], types := [(.pair .nat .bool)], result := (.pair .bool .nat),
      body := (.build .pair (.pair .bool .nat) [(.second (.var 0)), (.first (.var 0))]) }] }

def __fits_0 (p : (Prod Nat Bool)) : Bool :=
  ((Bool.true && (Bool.true && Bool.true)) && Bool.true)

theorem __rel_0 (p : (Prod Nat Bool)) : _root_.LexLeanPreservation.FunRel __prog 0 [((_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.nat _root_.LexLeanTarget.TargetSyntax.Value.bool) p)] (_root_.LexLeanPreservation.Rel (__fits_0 p) ((_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.bool _root_.LexLeanTarget.TargetSyntax.Value.nat) (Coverage.Main.pairs p))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_second (_root_.LexLeanPreservation.conv_var rfl)) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_first (_root_.LexLeanPreservation.conv_var rfl)) _root_.LexLeanPreservation.convL_nil)) _root_.LexLeanPreservation.construct_pair)

def denote (p : (Prod Nat Bool)) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 p) (_root_.LexLeanPreservation.Obs.value (((_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.bool _root_.LexLeanTarget.TargetSyntax.Value.nat) (Coverage.Main.pairs p)))) _root_.LexLeanPreservation.Obs.overflow

theorem root (p : (Prod Nat Bool)) : _root_.LexLeanPreservation.RunConv __prog 0 [((_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.nat _root_.LexLeanTarget.TargetSyntax.Value.bool) p)] (_root_.LexLeanPreservation.Rel (__fits_0 p) ((_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.bool _root_.LexLeanTarget.TargetSyntax.Value.nat) (Coverage.Main.pairs p))) :=
  _root_.LexLeanPreservation.run_of_funRel (__rel_0 p)

end LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R17
