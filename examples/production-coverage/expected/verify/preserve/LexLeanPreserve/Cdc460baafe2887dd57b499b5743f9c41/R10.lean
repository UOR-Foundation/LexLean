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
namespace LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R10

def __prog : LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [.nat, .nat], result := .nat,
      body := (.«let» 2 .nat (.prim .natQuot [(.var 0), (.var 1), (.value .nat (.nat 7))]) (.«let» 3 .nat (.prim .natRem [(.var 0), (.var 1), (.value .nat (.nat 0))]) (.cond (.cond (.prim .natEq [(.var 2), (.value .nat (.nat 0))]) (.cond (.prim .natLe [(.var 3), (.value .nat (.nat 3))]) (.build .true .bool []) (.prim .boolNot [(.prim .natLt [(.var 0), (.var 1)])])) (.build .false .bool [])) (.prim .natAdd [(.prim .natSub [(.var 0), (.var 1)]), (.var 3)]) (.prim .natMul [(.var 2), (.var 3)])))) }] }

def __fits_0 (a : Nat) (b : Nat) : Bool :=
  (((true && (true && (true && true))) && true) && (let q : Nat := (Coverage.Main.LexLeanRuntime.quotient (a) (b) ((7 : Nat)) : Nat); (((true && (true && (true && true))) && true) && (let r : Nat := (Coverage.Main.LexLeanRuntime.remainder (a) (b) ((0 : Nat)) : Nat); ((((true && (true && true)) && true) && (if (Nat.beq q (0 : Nat)) then (((true && (true && true)) && true) && (if (Nat.ble r (3 : Nat)) then (true && true) else ((((true && (true && true)) && true) && true) && true))) else (true && true))) && (if ((Nat.beq q (0 : Nat)) && ((Nat.ble r (3 : Nat)) || (!(Nat.blt a b)))) then ((((true && (true && true)) && true) && (true && true)) && (Nat.blt ((Coverage.Main.LexLeanRuntime.subtract (a) (b) : Nat) + r) 18446744073709551616)) else ((true && (true && true)) && (Nat.blt (Coverage.Main.LexLeanRuntime.multiply (q) (r) : Nat) 18446744073709551616))))))))

theorem __rel_0 (a : Nat) (b : Nat) : LexLeanPreservation.FunRel __prog 0 [(LexLeanTarget.TargetSyntax.Value.nat a), (LexLeanTarget.TargetSyntax.Value.nat b)] (LexLeanPreservation.Rel (__fits_0 a b) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.natOps a b))) :=
  LexLeanPreservation.funRel_intro rfl rfl (let q : Nat := (Coverage.Main.LexLeanRuntime.quotient (a) (b) ((7 : Nat)) : Nat); LexLeanPreservation.conv_let (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil))) (LexLeanPreservation.prim_natQuot a b (7 : Nat))) (let r : Nat := (Coverage.Main.LexLeanRuntime.remainder (a) (b) ((0 : Nat)) : Nat); LexLeanPreservation.conv_let (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil))) (LexLeanPreservation.prim_natRem a b (0 : Nat))) (LexLeanPreservation.conv_cond (fun (__b : Bool) => (if __b then ((((true && (true && true)) && true) && (true && true)) && (Nat.blt ((Coverage.Main.LexLeanRuntime.subtract (a) (b) : Nat) + r) 18446744073709551616)) else ((true && (true && true)) && (Nat.blt (Coverage.Main.LexLeanRuntime.multiply (q) (r) : Nat) 18446744073709551616)))) (fun (__b : Bool) => LexLeanTarget.TargetSyntax.Value.nat (if __b then ((Coverage.Main.LexLeanRuntime.subtract (a) (b) : Nat) + r) else (Coverage.Main.LexLeanRuntime.multiply (q) (r) : Nat))) ((Nat.beq q (0 : Nat)) && ((Nat.ble r (3 : Nat)) || (!(Nat.blt a b)))) (LexLeanPreservation.conv_cond (fun (__b : Bool) => if __b then (((true && (true && true)) && true) && (if (Nat.ble r (3 : Nat)) then (true && true) else ((((true && (true && true)) && true) && true) && true))) else (true && true)) (fun (__b : Bool) => LexLeanTarget.TargetSyntax.Value.bool (__b && ((Nat.ble r (3 : Nat)) || (!(Nat.blt a b))))) (Nat.beq q (0 : Nat)) (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natEq q (0 : Nat))) (fun _ => (LexLeanPreservation.conv_cond (fun (__b : Bool) => if __b then (true && true) else ((((true && (true && true)) && true) && true) && true)) (fun (__b : Bool) => LexLeanTarget.TargetSyntax.Value.bool (__b || (!(Nat.blt a b)))) (Nat.ble r (3 : Nat)) (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natLe r (3 : Nat))) (fun _ => (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_true)) (fun _ => (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natLt a b)) LexLeanPreservation.convL_nil) (LexLeanPreservation.prim_boolNot (Nat.blt a b)))))) (fun _ => (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_false))) (fun _ => (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natSub a b)) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd (Coverage.Main.LexLeanRuntime.subtract (a) (b) : Nat) r))) (fun _ => (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natMul q r))))))

def denote (a : Nat) (b : Nat) : LexLeanPreservation.Obs :=
  cond (__fits_0 a b) (LexLeanPreservation.Obs.value ((LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.natOps a b)))) LexLeanPreservation.Obs.overflow

theorem root (a : Nat) (b : Nat) : LexLeanPreservation.RunConv __prog 0 [(LexLeanTarget.TargetSyntax.Value.nat a), (LexLeanTarget.TargetSyntax.Value.nat b)] (LexLeanPreservation.Rel (__fits_0 a b) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.natOps a b))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 a b)

end LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R10
