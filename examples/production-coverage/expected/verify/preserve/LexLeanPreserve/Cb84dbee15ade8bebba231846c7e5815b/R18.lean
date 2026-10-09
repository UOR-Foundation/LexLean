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
namespace LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R18

def __prog : _root_.LexLeanTarget.TargetSyntax.Program :=
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
  (Bool.true && (((Bool.true && ((Bool.true && Bool.true) && (__ff_0 (value)))) && Bool.true) && (__ff_0 ((step (value))))))

theorem __rel_1 (step : (Nat -> Nat)) (__ff_0 : Nat -> Bool) (__fi_0 : Nat) (__fc_0 : List _root_.LexLeanTarget.TargetSyntax.Value) (__fh_0 : ∀ (__p0 : Nat), _root_.LexLeanPreservation.FunRel __prog __fi_0 (_root_.LexLeanTarget.TargetSemantics.LexLeanRuntime.append __fc_0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat __p0)]) (_root_.LexLeanPreservation.Rel (__ff_0 __p0) (_root_.LexLeanTarget.TargetSyntax.Value.nat (step __p0)))) (value : Nat) : _root_.LexLeanPreservation.FunRel __prog 1 [(_root_.LexLeanTarget.TargetSyntax.Value.closure __fi_0 __fc_0), (_root_.LexLeanTarget.TargetSyntax.Value.nat value)] (_root_.LexLeanPreservation.Rel (__fits_1 step __ff_0 value) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.Types.twice step value))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_apply (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_apply (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__fh_0 (value))) _root_.LexLeanPreservation.convL_nil) (__fh_0 ((step (value)))))

def __fits_2 (k : Nat) (offset : Nat) (x : Nat) : Bool :=
  ((((Bool.true && (Bool.true && Bool.true)) && (Nat.blt (Coverage.Main.LexLeanRuntime.multiply (x) (k) : Nat) 18446744073709551616)) && (Bool.true && Bool.true)) && (Nat.blt ((Coverage.Main.LexLeanRuntime.multiply (x) (k) : Nat) + offset) 18446744073709551616))

theorem __rel_2 (k : Nat) (offset : Nat) (x : Nat) : _root_.LexLeanPreservation.FunRel __prog 2 [(_root_.LexLeanTarget.TargetSyntax.Value.nat k), (_root_.LexLeanTarget.TargetSyntax.Value.nat offset), (_root_.LexLeanTarget.TargetSyntax.Value.nat x)] (_root_.LexLeanPreservation.Rel (__fits_2 k offset x) (_root_.LexLeanTarget.TargetSyntax.Value.nat (((Coverage.Main.LexLeanRuntime.multiply (x) (k) : Nat) + offset)))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natMul x k)) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natAdd (Coverage.Main.LexLeanRuntime.multiply (x) (k) : Nat) offset))

def __fits_3 (k : Nat) (y : Nat) : Bool :=
  ((Bool.true && (Bool.true && Bool.true)) && Bool.true)

theorem __rel_3 (k : Nat) (y : Nat) : _root_.LexLeanPreservation.FunRel __prog 3 [(_root_.LexLeanTarget.TargetSyntax.Value.nat k), (_root_.LexLeanTarget.TargetSyntax.Value.nat y)] (_root_.LexLeanPreservation.Rel (__fits_3 k y) (_root_.LexLeanTarget.TargetSyntax.Value.nat ((Coverage.Main.LexLeanRuntime.subtract (y) (k) : Nat)))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natSub y k))

def __fits_4 (n : Nat) : Bool :=
  ((Bool.true && (Bool.true && Bool.true)) && (Nat.blt (n + n) 18446744073709551616))

theorem __rel_4 (n : Nat) : _root_.LexLeanPreservation.FunRel __prog 4 [(_root_.LexLeanTarget.TargetSyntax.Value.nat n)] (_root_.LexLeanPreservation.Rel (__fits_4 n) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.double n))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natAdd n n))

def __fits_0 (base : Nat) (offset : Nat) : Bool :=
  (((Bool.true && (Bool.true && Bool.true)) && (Nat.blt (base + (1 : Nat)) 18446744073709551616)) && (let k : Nat := (base + (1 : Nat)); (((((Bool.true && (Bool.true && Bool.true)) && (Bool.true && Bool.true)) && (__fits_1 ((fun (x : Nat) => ((Coverage.Main.LexLeanRuntime.multiply (x) (k) : Nat) + offset))) (fun (__p0 : Nat) => __fits_2 (k) (offset) __p0) (base))) && (((((Bool.true && Bool.true) && ((Bool.true && Bool.true) && ((__fits_3 (k)) (base)))) && (((Bool.true && (Bool.true && Bool.true)) && (__fits_1 ((Coverage.Main.double)) (fun (__p0 : Nat) => __fits_4 __p0) (offset))) && Bool.true)) && (Nat.blt (((fun (y : Nat) => (Coverage.Main.LexLeanRuntime.subtract (y) (k) : Nat)) (base)) + (Coverage.Types.twice ((Coverage.Main.double)) (offset))) 18446744073709551616)) && Bool.true)) && (Nat.blt ((Coverage.Types.twice ((fun (x : Nat) => ((Coverage.Main.LexLeanRuntime.multiply (x) (k) : Nat) + offset))) (base)) + (((fun (y : Nat) => (Coverage.Main.LexLeanRuntime.subtract (y) (k) : Nat)) (base)) + (Coverage.Types.twice ((Coverage.Main.double)) (offset)))) 18446744073709551616))))

attribute [local irreducible] Coverage.Types.twice in
theorem __rel_0 (base : Nat) (offset : Nat) : _root_.LexLeanPreservation.FunRel __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat base), (_root_.LexLeanTarget.TargetSyntax.Value.nat offset)] (_root_.LexLeanPreservation.Rel (__fits_0 base offset) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.higher base offset))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (let k : Nat := (base + (1 : Nat)); _root_.LexLeanPreservation.conv_let (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natAdd base (1 : Nat))) (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_closure (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil))) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (__rel_1 ((fun (x : Nat) => ((Coverage.Main.LexLeanRuntime.multiply (x) (k) : Nat) + offset))) (fun (__p0 : Nat) => __fits_2 (k) (offset) __p0) (2) ([(_root_.LexLeanTarget.TargetSyntax.Value.nat k), (_root_.LexLeanTarget.TargetSyntax.Value.nat offset)]) (fun (__p0 : Nat) => __rel_2 (k) (offset) __p0) (base))) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_apply (_root_.LexLeanPreservation.conv_closure (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) ((__rel_3 (k)) (base))) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_closure _root_.LexLeanPreservation.convL_nil) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (__rel_1 ((Coverage.Main.double)) (fun (__p0 : Nat) => __fits_4 __p0) (4) ([]) (fun (__p0 : Nat) => __rel_4 __p0) (offset))) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natAdd ((fun (y : Nat) => (Coverage.Main.LexLeanRuntime.subtract (y) (k) : Nat)) (base)) (Coverage.Types.twice ((Coverage.Main.double)) (offset)))) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natAdd (Coverage.Types.twice ((fun (x : Nat) => ((Coverage.Main.LexLeanRuntime.multiply (x) (k) : Nat) + offset))) (base)) (((fun (y : Nat) => (Coverage.Main.LexLeanRuntime.subtract (y) (k) : Nat)) (base)) + (Coverage.Types.twice ((Coverage.Main.double)) (offset))))))

def denote (base : Nat) (offset : Nat) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 base offset) (_root_.LexLeanPreservation.Obs.value ((_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.higher base offset)))) _root_.LexLeanPreservation.Obs.overflow

theorem root (base : Nat) (offset : Nat) : _root_.LexLeanPreservation.RunConv __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat base), (_root_.LexLeanTarget.TargetSyntax.Value.nat offset)] (_root_.LexLeanPreservation.Rel (__fits_0 base offset) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.higher base offset))) :=
  _root_.LexLeanPreservation.run_of_funRel (__rel_0 base offset)

end LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R18
