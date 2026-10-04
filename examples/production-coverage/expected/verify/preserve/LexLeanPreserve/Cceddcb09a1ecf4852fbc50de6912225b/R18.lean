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
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R18

def __prog : LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [.nat, .nat], result := .nat,
      body := (.«let» 2 .nat (.prim .natAdd [(.var 0), (.value .nat (.nat 1))]) (.prim .natAdd [(.call 1 [(.closure 2 [(.var 2), (.var 1)]), (.var 0)]), (.prim .natAdd [(.apply (.closure 3 [(.var 2)]) [(.var 0)]), (.call 1 [(.closure 4 []), (.var 1)])])])) },
    { parameters := [0, 1], types := [(.fn [.nat] .nat), .nat], result := .nat,
      body := (.apply (.var 0) [(.apply (.var 0) [(.var 1)])]) },
    { parameters := [0, 1, 2], types := [.nat, .nat, .nat], result := .nat,
      body := (.prim .natAdd [(.prim .natMul [(.var 2), (.var 0)]), (.var 1)]) },
    { parameters := [0, 1], types := [.nat, .nat], result := .nat,
      body := (.prim .natSub [(.var 1), (.var 0)]) },
    { parameters := [0], types := [.nat], result := .nat,
      body := (.prim .natAdd [(.var 0), (.var 0)]) }] }

def __fits_1 (step : (Nat -> Nat)) (__ff_0 : Nat -> Bool) (value : Nat) : Bool :=
  (true && (((true && ((true && true) && (__ff_0 (value)))) && true) && (__ff_0 ((step (value))))))

theorem __rel_1 (step : (Nat -> Nat)) (__ff_0 : Nat -> Bool) (__fi_0 : Nat) (__fc_0 : List LexLeanTarget.TargetSyntax.Value) (__fh_0 : ∀ (__p0 : Nat), LexLeanPreservation.FunRel __prog __fi_0 (LexLeanTarget.TargetSemantics.LexLeanRuntime.append __fc_0 [(LexLeanTarget.TargetSyntax.Value.nat __p0)]) (LexLeanPreservation.Rel (__ff_0 __p0) (LexLeanTarget.TargetSyntax.Value.nat (step __p0)))) (value : Nat) : LexLeanPreservation.FunRel __prog 1 [(LexLeanTarget.TargetSyntax.Value.closure __fi_0 __fc_0), (LexLeanTarget.TargetSyntax.Value.nat value)] (LexLeanPreservation.Rel (__fits_1 step __ff_0 value) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Types.twice step value))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_apply (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_apply (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__fh_0 (value))) LexLeanPreservation.convL_nil) (__fh_0 ((step (value)))))

def __fits_2 (k : Nat) (offset : Nat) (x : Nat) : Bool :=
  ((((true && (true && true)) && (Nat.blt (Coverage.Main.LexLeanRuntime.multiply (x) (k) : Nat) 18446744073709551616)) && (true && true)) && (Nat.blt ((Coverage.Main.LexLeanRuntime.multiply (x) (k) : Nat) + offset) 18446744073709551616))

theorem __rel_2 (k : Nat) (offset : Nat) (x : Nat) : LexLeanPreservation.FunRel __prog 2 [(LexLeanTarget.TargetSyntax.Value.nat k), (LexLeanTarget.TargetSyntax.Value.nat offset), (LexLeanTarget.TargetSyntax.Value.nat x)] (LexLeanPreservation.Rel (__fits_2 k offset x) (LexLeanTarget.TargetSyntax.Value.nat (((Coverage.Main.LexLeanRuntime.multiply (x) (k) : Nat) + offset)))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natMul x k)) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd (Coverage.Main.LexLeanRuntime.multiply (x) (k) : Nat) offset))

def __fits_3 (k : Nat) (y : Nat) : Bool :=
  ((true && (true && true)) && true)

theorem __rel_3 (k : Nat) (y : Nat) : LexLeanPreservation.FunRel __prog 3 [(LexLeanTarget.TargetSyntax.Value.nat k), (LexLeanTarget.TargetSyntax.Value.nat y)] (LexLeanPreservation.Rel (__fits_3 k y) (LexLeanTarget.TargetSyntax.Value.nat ((Coverage.Main.LexLeanRuntime.subtract (y) (k) : Nat)))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natSub y k))

def __fits_4 (n : Nat) : Bool :=
  ((true && (true && true)) && (Nat.blt (n + n) 18446744073709551616))

theorem __rel_4 (n : Nat) : LexLeanPreservation.FunRel __prog 4 [(LexLeanTarget.TargetSyntax.Value.nat n)] (LexLeanPreservation.Rel (__fits_4 n) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.double n))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd n n))

def __fits_0 (base : Nat) (offset : Nat) : Bool :=
  (((true && (true && true)) && (Nat.blt (base + (1 : Nat)) 18446744073709551616)) && (let k : Nat := (base + (1 : Nat)); (((((true && (true && true)) && (true && true)) && (__fits_1 ((fun (x : Nat) => ((Coverage.Main.LexLeanRuntime.multiply (x) (k) : Nat) + offset))) (fun (__p0 : Nat) => __fits_2 (k) (offset) __p0) (base))) && (((((true && true) && ((true && true) && ((__fits_3 (k)) (base)))) && (((true && (true && true)) && (__fits_1 ((Coverage.Main.double)) (fun (__p0 : Nat) => __fits_4 __p0) (offset))) && true)) && (Nat.blt (((fun (y : Nat) => (Coverage.Main.LexLeanRuntime.subtract (y) (k) : Nat)) (base)) + (Coverage.Types.twice ((Coverage.Main.double)) (offset))) 18446744073709551616)) && true)) && (Nat.blt ((Coverage.Types.twice ((fun (x : Nat) => ((Coverage.Main.LexLeanRuntime.multiply (x) (k) : Nat) + offset))) (base)) + (((fun (y : Nat) => (Coverage.Main.LexLeanRuntime.subtract (y) (k) : Nat)) (base)) + (Coverage.Types.twice ((Coverage.Main.double)) (offset)))) 18446744073709551616))))

attribute [local irreducible] Coverage.Types.twice in
theorem __rel_0 (base : Nat) (offset : Nat) : LexLeanPreservation.FunRel __prog 0 [(LexLeanTarget.TargetSyntax.Value.nat base), (LexLeanTarget.TargetSyntax.Value.nat offset)] (LexLeanPreservation.Rel (__fits_0 base offset) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.higher base offset))) :=
  LexLeanPreservation.funRel_intro rfl rfl (let k : Nat := (base + (1 : Nat)); LexLeanPreservation.conv_let (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd base (1 : Nat))) (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_closure (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil))) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (__rel_1 ((fun (x : Nat) => ((Coverage.Main.LexLeanRuntime.multiply (x) (k) : Nat) + offset))) (fun (__p0 : Nat) => __fits_2 (k) (offset) __p0) (2) ([(LexLeanTarget.TargetSyntax.Value.nat k), (LexLeanTarget.TargetSyntax.Value.nat offset)]) (fun (__p0 : Nat) => __rel_2 (k) (offset) __p0) (base))) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_apply (LexLeanPreservation.conv_closure (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) ((__rel_3 (k)) (base))) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_closure LexLeanPreservation.convL_nil) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (__rel_1 ((Coverage.Main.double)) (fun (__p0 : Nat) => __fits_4 __p0) (4) ([]) (fun (__p0 : Nat) => __rel_4 __p0) (offset))) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd ((fun (y : Nat) => (Coverage.Main.LexLeanRuntime.subtract (y) (k) : Nat)) (base)) (Coverage.Types.twice ((Coverage.Main.double)) (offset)))) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd (Coverage.Types.twice ((fun (x : Nat) => ((Coverage.Main.LexLeanRuntime.multiply (x) (k) : Nat) + offset))) (base)) (((fun (y : Nat) => (Coverage.Main.LexLeanRuntime.subtract (y) (k) : Nat)) (base)) + (Coverage.Types.twice ((Coverage.Main.double)) (offset))))))

def denote (base : Nat) (offset : Nat) : LexLeanPreservation.Obs :=
  cond (__fits_0 base offset) (LexLeanPreservation.Obs.value ((LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.higher base offset)))) LexLeanPreservation.Obs.overflow

theorem root (base : Nat) (offset : Nat) : LexLeanPreservation.RunConv __prog 0 [(LexLeanTarget.TargetSyntax.Value.nat base), (LexLeanTarget.TargetSyntax.Value.nat offset)] (LexLeanPreservation.Rel (__fits_0 base offset) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.higher base offset))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 base offset)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R18
