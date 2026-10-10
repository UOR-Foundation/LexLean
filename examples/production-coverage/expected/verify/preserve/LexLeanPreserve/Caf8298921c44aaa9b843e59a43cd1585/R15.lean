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
namespace LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R15

def __prog : _root_.LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [(.option .nat), (.result .nat .bool)], result := .nat,
      body := (.prim .natAdd [(.«match» .nat (.var 0) [(.arm .none [] (.value .nat (.nat 0))), (.arm .some [2] (.var 2))]), (.«match» .nat (.var 1) [(.arm .ok [3] (.var 3)), (.arm .error [4] (.cond (.var 4) (.value .nat (.nat 1)) (.value .nat (.nat 2))))])]) }] }

def __fits_0 (v : (Option Nat)) (r : (Except Bool Nat)) : Bool :=
  (((Bool.true && (match v with | Option.none => Bool.true | Option.some x => Bool.true)) && ((Bool.true && (match r with | Except.ok y => Bool.true | Except.error flag => (Bool.true && (if flag then Bool.true else Bool.true)))) && Bool.true)) && (Nat.blt ((match v with | Option.none => (0 : Nat) | Option.some x => x) + (match r with | Except.ok y => y | Except.error flag => (if flag then (1 : Nat) else (2 : Nat)))) 18446744073709551616))

theorem __rel_0 (v : (Option Nat)) (r : (Except Bool Nat)) : _root_.LexLeanPreservation.FunRel __prog 0 [((_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.nat) v), ((_root_.LexLeanPreservation.encExcept _root_.LexLeanTarget.TargetSyntax.Value.bool _root_.LexLeanTarget.TargetSyntax.Value.nat) r)] (_root_.LexLeanPreservation.Rel (__fits_0 v r) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.options v r))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_matchV (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.nat) (fun (__a : (Option Nat)) => (match __a with | Option.none => Bool.true | Option.some x => Bool.true)) (fun (__a : (Option Nat)) => _root_.LexLeanTarget.TargetSyntax.Value.nat (match __a with | Option.none => (0 : Nat) | Option.some x => x)) v (_root_.LexLeanPreservation.conv_var rfl) (fun (__a : (Option Nat)) _ => match __a with | Option.none => (_root_.LexLeanPreservation.convA_hit rfl rfl _root_.LexLeanPreservation.conv_value) | Option.some x => (_root_.LexLeanPreservation.convA_miss rfl (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_var rfl))))) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_matchV (_root_.LexLeanPreservation.encExcept _root_.LexLeanTarget.TargetSyntax.Value.bool _root_.LexLeanTarget.TargetSyntax.Value.nat) (fun (__a : (Except Bool Nat)) => (match __a with | Except.ok y => Bool.true | Except.error flag => (Bool.true && (if flag then Bool.true else Bool.true)))) (fun (__a : (Except Bool Nat)) => _root_.LexLeanTarget.TargetSyntax.Value.nat (match __a with | Except.ok y => y | Except.error flag => (if flag then (1 : Nat) else (2 : Nat)))) r (_root_.LexLeanPreservation.conv_var rfl) (fun (__a : (Except Bool Nat)) _ => match __a with | Except.ok y => (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_var rfl)) | Except.error flag => (_root_.LexLeanPreservation.convA_miss rfl (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_cond (fun (__b : Bool) => (if __b then Bool.true else Bool.true)) (fun (__b : Bool) => _root_.LexLeanTarget.TargetSyntax.Value.nat (if __b then (1 : Nat) else (2 : Nat))) flag (_root_.LexLeanPreservation.conv_var rfl) (fun _ => _root_.LexLeanPreservation.conv_value) (fun _ => _root_.LexLeanPreservation.conv_value)))))) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natAdd (match v with | Option.none => (0 : Nat) | Option.some x => x) (match r with | Except.ok y => y | Except.error flag => (if flag then (1 : Nat) else (2 : Nat)))))

def denote (v : (Option Nat)) (r : (Except Bool Nat)) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 v r) (_root_.LexLeanPreservation.Obs.value ((_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.options v r)))) _root_.LexLeanPreservation.Obs.overflow

theorem root (v : (Option Nat)) (r : (Except Bool Nat)) : _root_.LexLeanPreservation.RunConv __prog 0 [((_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.nat) v), ((_root_.LexLeanPreservation.encExcept _root_.LexLeanTarget.TargetSyntax.Value.bool _root_.LexLeanTarget.TargetSyntax.Value.nat) r)] (_root_.LexLeanPreservation.Rel (__fits_0 v r) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.options v r))) :=
  _root_.LexLeanPreservation.run_of_funRel (__rel_0 v r)

end LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R15
