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
namespace LexLeanPreserve.C098c9d7898f8ebcc504db089f79c1a27.R15

def __prog : LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0], types := [.nat], result := (.pair (.option .nat) (.result .nat .unit)),
      body := (.build .pair (.pair (.option .nat) (.result .nat .unit)) [(.build .some (.option .nat) [(.build .succ .nat [(.var 0)])]), (.cond (.prim .natLt [(.var 0), (.value .nat (.nat 3))]) (.build .ok (.result .nat .unit) [(.build .zero .nat [])]) (.build .error (.result .nat .unit) [(.build .unit .unit [])]))]) }] }

def __fits_0 (n : Nat) : Bool :=
  ((((((true && true) && (Nat.blt (Nat.succ (n) : Nat) 18446744073709551616)) && true) && true) && ((((true && (true && true)) && true) && (if (Nat.blt n (3 : Nat)) then (((true && true) && true) && true) else (((true && true) && true) && true))) && true)) && true)

theorem __rel_0 (n : Nat) : LexLeanPreservation.FunRel __prog 0 [(LexLeanTarget.TargetSyntax.Value.nat n)] (LexLeanPreservation.Rel (__fits_0 n) ((LexLeanPreservation.encPair (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.nat) (LexLeanPreservation.encExcept LexLeanPreservation.encUnit LexLeanTarget.TargetSyntax.Value.nat)) (Coverage.Main.builders n))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) LexLeanPreservation.construct_succ) LexLeanPreservation.convL_nil) LexLeanPreservation.construct_some) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_cond (fun (__b : Bool) => (if __b then (((true && true) && true) && true) else (((true && true) && true) && true))) (fun (__b : Bool) => (LexLeanPreservation.encExcept LexLeanPreservation.encUnit LexLeanTarget.TargetSyntax.Value.nat) (if __b then (Except.ok ((Nat.zero : Nat)) : (Except Unit Nat)) else (Except.error (()) : (Except Unit Nat)))) (Nat.blt n (3 : Nat)) (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natLt n (3 : Nat))) (fun _ => (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_zero) LexLeanPreservation.convL_nil) LexLeanPreservation.construct_ok)) (fun _ => (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_unit) LexLeanPreservation.convL_nil) LexLeanPreservation.construct_error))) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair)

def denote (n : Nat) : LexLeanPreservation.Obs :=
  LexLeanPreservation.Rel (__fits_0 n) ((LexLeanPreservation.encPair (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.nat) (LexLeanPreservation.encExcept LexLeanPreservation.encUnit LexLeanTarget.TargetSyntax.Value.nat)) (Coverage.Main.builders n))

theorem root (n : Nat) : LexLeanPreservation.RunConv __prog 0 [(LexLeanTarget.TargetSyntax.Value.nat n)] (LexLeanPreservation.Rel (__fits_0 n) ((LexLeanPreservation.encPair (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.nat) (LexLeanPreservation.encExcept LexLeanPreservation.encUnit LexLeanTarget.TargetSyntax.Value.nat)) (Coverage.Main.builders n))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 n)

end LexLeanPreserve.C098c9d7898f8ebcc504db089f79c1a27.R15
