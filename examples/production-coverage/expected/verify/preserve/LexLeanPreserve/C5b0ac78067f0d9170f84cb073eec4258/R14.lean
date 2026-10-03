import LexLeanPreservation.Core
import LexLeanPreservation.Values
import LexLeanPreservation.Primitives
import LexLeanPreservation.Fixed
import LexLeanPreservation.Keys
import LexLeanPreservation.Templates
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
namespace LexLeanPreserve.C5b0ac78067f0d9170f84cb073eec4258.R14

def __prog : LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [(.option .nat), (.result .nat .bool)], result := .nat,
      body := (.prim .natAdd [(.«match» .nat (.var 0) [(.arm .none [] (.value .nat (.nat 0))), (.arm .some [2] (.var 2))]), (.«match» .nat (.var 1) [(.arm .ok [3] (.var 3)), (.arm .error [4] (.cond (.var 4) (.value .nat (.nat 1)) (.value .nat (.nat 2))))])]) }] }

def __fits_0 (v : (Option Nat)) (r : (Except Bool Nat)) : Bool :=
  (((true && (match v with | Option.none => true | Option.some x => true)) && ((true && (match r with | Except.ok y => true | Except.error flag => (true && (if flag then true else true)))) && true)) && (Nat.blt ((match v with | Option.none => (0 : Nat) | Option.some x => x) + (match r with | Except.ok y => y | Except.error flag => (if flag then (1 : Nat) else (2 : Nat)))) 18446744073709551616))

theorem __rel_0 (v : (Option Nat)) (r : (Except Bool Nat)) : LexLeanPreservation.FunRel __prog 0 [((LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.nat) v), ((LexLeanPreservation.encExcept LexLeanTarget.TargetSyntax.Value.bool LexLeanTarget.TargetSyntax.Value.nat) r)] (LexLeanPreservation.Rel (__fits_0 v r) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.options v r))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_matchV (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.nat) (fun (__a : (Option Nat)) => (match __a with | Option.none => true | Option.some x => true)) (fun (__a : (Option Nat)) => LexLeanTarget.TargetSyntax.Value.nat (match __a with | Option.none => (0 : Nat) | Option.some x => x)) v (LexLeanPreservation.conv_var rfl) (fun (__a : (Option Nat)) _ => match __a with | Option.none => (LexLeanPreservation.convA_hit rfl rfl LexLeanPreservation.conv_value) | Option.some x => (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_var rfl))))) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_matchV (LexLeanPreservation.encExcept LexLeanTarget.TargetSyntax.Value.bool LexLeanTarget.TargetSyntax.Value.nat) (fun (__a : (Except Bool Nat)) => (match __a with | Except.ok y => true | Except.error flag => (true && (if flag then true else true)))) (fun (__a : (Except Bool Nat)) => LexLeanTarget.TargetSyntax.Value.nat (match __a with | Except.ok y => y | Except.error flag => (if flag then (1 : Nat) else (2 : Nat)))) r (LexLeanPreservation.conv_var rfl) (fun (__a : (Except Bool Nat)) _ => match __a with | Except.ok y => (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_var rfl)) | Except.error flag => (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_cond (fun (__b : Bool) => (if __b then true else true)) (fun (__b : Bool) => LexLeanTarget.TargetSyntax.Value.nat (if __b then (1 : Nat) else (2 : Nat))) flag (LexLeanPreservation.conv_var rfl) (fun _ => LexLeanPreservation.conv_value) (fun _ => LexLeanPreservation.conv_value)))))) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd (match v with | Option.none => (0 : Nat) | Option.some x => x) (match r with | Except.ok y => y | Except.error flag => (if flag then (1 : Nat) else (2 : Nat)))))

def denote (v : (Option Nat)) (r : (Except Bool Nat)) : LexLeanPreservation.Obs :=
  LexLeanPreservation.Rel (__fits_0 v r) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.options v r))

theorem root (v : (Option Nat)) (r : (Except Bool Nat)) : LexLeanPreservation.RunConv __prog 0 [((LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.nat) v), ((LexLeanPreservation.encExcept LexLeanTarget.TargetSyntax.Value.bool LexLeanTarget.TargetSyntax.Value.nat) r)] (LexLeanPreservation.Rel (__fits_0 v r) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.options v r))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 v r)

end LexLeanPreserve.C5b0ac78067f0d9170f84cb073eec4258.R14
