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
namespace LexLeanPreserve.Cba27a3dc52c63de8b08b0726788d5c73.R3

def __prog : LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [(.list .nat), (.list .int)], result := (.pair .nat .int),
      body := (.build .pair (.pair .nat .int) [(.call 1 [(.closure 2 []), (.value .nat (.nat 0)), (.var 0)]), (.call 3 [(.closure 4 []), (.value .int (.int 5)), (.var 1)])]) },
    { parameters := [0, 1, 2], types := [(.fn [.nat, .nat] .nat), .nat, (.list .nat)], result := .nat,
      body := (.«match» .nat (.var 2) [(.arm .nil [] (.var 1)), (.arm .cons [3, 4] (.call 1 [(.var 0), (.apply (.var 0) [(.var 1), (.var 3)]), (.var 4)]))]) },
    { parameters := [0, 1], types := [.nat, .nat], result := .nat,
      body := (.prim .natAdd [(.var 0), (.var 1)]) },
    { parameters := [0, 1, 2], types := [(.fn [.int, .int] .int), .int, (.list .int)], result := .int,
      body := (.«match» .int (.var 2) [(.arm .nil [] (.var 1)), (.arm .cons [3, 4] (.call 3 [(.var 0), (.apply (.var 0) [(.var 1), (.var 3)]), (.var 4)]))]) },
    { parameters := [0, 1], types := [.int, .int], result := .int,
      body := (.prim .intSub [(.var 0), (.var 1)]) }] }

def __fits_2 (a : Nat) (x : Nat) : Bool :=
  ((true && (true && true)) && (Nat.blt (a + x) 18446744073709551616))

theorem __rel_2 (a : Nat) (x : Nat) : LexLeanPreservation.FunRel __prog 2 [(LexLeanTarget.TargetSyntax.Value.nat a), (LexLeanTarget.TargetSyntax.Value.nat x)] (LexLeanPreservation.Rel (__fits_2 a x) (LexLeanTarget.TargetSyntax.Value.nat ((a + x)))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd a x))

def __fits_4 (a : Int) (x : Int) : Bool :=
  ((true && (true && true)) && (LexLeanPreservation.intFits (Coverage.Colls.LexLeanRuntime.subtract (a) (x) : Int)))

theorem __rel_4 (a : Int) (x : Int) : LexLeanPreservation.FunRel __prog 4 [(LexLeanTarget.TargetSyntax.Value.int a), (LexLeanTarget.TargetSyntax.Value.int x)] (LexLeanPreservation.Rel (__fits_4 a x) (LexLeanTarget.TargetSyntax.Value.int ((Coverage.Colls.LexLeanRuntime.subtract (a) (x) : Int)))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_intSub a x))

def __fits_0 (xs : (List Nat)) (s : (List Int)) : Bool :=
  ((((true && (true && (true && true))) && (LexLeanPreservation.foldFits (fun (__p0 : Nat) (__p1 : Nat) => __fits_2 __p0 __p1) ((fun (a : Nat) (x : Nat) => (a + x))) ((0 : Nat)) (xs))) && (((true && (true && (true && true))) && (LexLeanPreservation.foldFits (fun (__p0 : Int) (__p1 : Int) => __fits_4 __p0 __p1) ((fun (a : Int) (x : Int) => (Coverage.Colls.LexLeanRuntime.subtract (a) (x) : Int))) ((5 : Int)) (s))) && true)) && true)

theorem __rel_0 (xs : (List Nat)) (s : (List Int)) : LexLeanPreservation.FunRel __prog 0 [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) xs), ((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.int) s)] (LexLeanPreservation.Rel (__fits_0 xs s) ((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.int) (Coverage.Colls.folds xs s))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_closure LexLeanPreservation.convL_nil) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil))) (LexLeanPreservation.tpl_listFold (ea := LexLeanTarget.TargetSyntax.Value.nat) (es := LexLeanTarget.TargetSyntax.Value.nat) (LexLeanPreservation.listEnc LexLeanTarget.TargetSyntax.Value.nat) (fun _ => rfl) ((fun (a : Nat) (x : Nat) => (a + x))) (fun (__p0 : Nat) (__p1 : Nat) => __fits_2 __p0 __p1) 2 [] (fun (__p0 : Nat) (__p1 : Nat) => __rel_2 __p0 __p1) (p := __prog) (fi := 1) (ta := .nat) (ts := .nat) rfl ((0 : Nat)) (xs))) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_closure LexLeanPreservation.convL_nil) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil))) (LexLeanPreservation.tpl_listFold (ea := LexLeanTarget.TargetSyntax.Value.int) (es := LexLeanTarget.TargetSyntax.Value.int) (LexLeanPreservation.listEnc LexLeanTarget.TargetSyntax.Value.int) (fun _ => rfl) ((fun (a : Int) (x : Int) => (Coverage.Colls.LexLeanRuntime.subtract (a) (x) : Int))) (fun (__p0 : Int) (__p1 : Int) => __fits_4 __p0 __p1) 4 [] (fun (__p0 : Int) (__p1 : Int) => __rel_4 __p0 __p1) (p := __prog) (fi := 3) (ta := .int) (ts := .int) rfl ((5 : Int)) (s))) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair)

def denote (xs : (List Nat)) (s : (List Int)) : LexLeanPreservation.Obs :=
  LexLeanPreservation.Rel (__fits_0 xs s) ((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.int) (Coverage.Colls.folds xs s))

theorem root (xs : (List Nat)) (s : (List Int)) : LexLeanPreservation.RunConv __prog 0 [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) xs), ((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.int) s)] (LexLeanPreservation.Rel (__fits_0 xs s) ((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.int) (Coverage.Colls.folds xs s))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 xs s)

end LexLeanPreserve.Cba27a3dc52c63de8b08b0726788d5c73.R3
