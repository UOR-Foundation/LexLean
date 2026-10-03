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
namespace LexLeanPreserve.C90d90c09a90803e7e527e747b4a162d8.R1

def __prog : LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [(.list (.pair .nat .nat)), .nat], result := .nat,
      body := (.call 1 [(.closure 2 [(.var 1)]), (.value .nat (.nat 0)), (.var 0)]) },
    { parameters := [0, 1, 2], types := [(.fn [.nat, .nat, .nat] .nat), .nat, (.list (.pair .nat .nat))], result := .nat,
      body := (.«match» .nat (.var 2) [(.arm .nil [] (.var 1)), (.arm .cons [3, 4] (.call 1 [(.var 0), (.apply (.var 0) [(.var 1), (.first (.var 3)), (.second (.var 3))]), (.var 4)]))]) },
    { parameters := [0, 1, 2, 3], types := [.nat, .nat, .nat, .nat], result := .nat,
      body := (.prim .natAdd [(.prim .natAdd [(.var 1), (.var 2)]), (.prim .natAdd [(.var 3), (.var 0)])]) }] }

def __fits_2 (bonus : Nat) (acc : Nat) (key : Nat) (value : Nat) : Bool :=
  ((((true && (true && true)) && (Nat.blt (acc + key) 18446744073709551616)) && (((true && (true && true)) && (Nat.blt (value + bonus) 18446744073709551616)) && true)) && (Nat.blt ((acc + key) + (value + bonus)) 18446744073709551616))

theorem __rel_2 (bonus : Nat) (acc : Nat) (key : Nat) (value : Nat) : LexLeanPreservation.FunRel __prog 2 [(LexLeanTarget.TargetSyntax.Value.nat bonus), (LexLeanTarget.TargetSyntax.Value.nat acc), (LexLeanTarget.TargetSyntax.Value.nat key), (LexLeanTarget.TargetSyntax.Value.nat value)] (LexLeanPreservation.Rel (__fits_2 bonus acc key value) (LexLeanTarget.TargetSyntax.Value.nat (((acc + key) + (value + bonus))))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd acc key)) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd value bonus)) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd (acc + key) (value + bonus)))

def __fits_0 (m : (List (Prod Nat Nat))) (bonus : Nat) : Bool :=
  (((true && true) && (true && (true && true))) && (LexLeanPreservation.foldFits (fun __st __e => (fun (__p0 : Nat) (__p1 : Nat) (__p2 : Nat) => __fits_2 (bonus) __p0 __p1 __p2) __st __e.1 __e.2) (fun __st __e => ((fun (acc : Nat) (key : Nat) (value : Nat) => ((acc + key) + (value + bonus)))) __st __e.1 __e.2) ((0 : Nat)) (m)))

theorem __rel_0 (m : (List (Prod Nat Nat))) (bonus : Nat) : LexLeanPreservation.FunRel __prog 0 [((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.nat)) m), (LexLeanTarget.TargetSyntax.Value.nat bonus)] (LexLeanPreservation.Rel (__fits_0 m bonus) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Colls.mapFolding m bonus))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_closure (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil))) (LexLeanPreservation.tpl_mapFold (ek := LexLeanTarget.TargetSyntax.Value.nat) (ev := LexLeanTarget.TargetSyntax.Value.nat) (es := LexLeanTarget.TargetSyntax.Value.nat) (LexLeanPreservation.listEnc (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.nat)) (fun _ => rfl) ((fun (acc : Nat) (key : Nat) (value : Nat) => ((acc + key) + (value + bonus)))) (fun (__p0 : Nat) (__p1 : Nat) (__p2 : Nat) => __fits_2 (bonus) __p0 __p1 __p2) 2 [(LexLeanTarget.TargetSyntax.Value.nat bonus)] (fun (__p0 : Nat) (__p1 : Nat) (__p2 : Nat) => __rel_2 (bonus) __p0 __p1 __p2) (p := __prog) (fi := 1) (tk := .nat) (tw := .nat) (ts := .nat) rfl ((0 : Nat)) (m)))

def denote (m : (List (Prod Nat Nat))) (bonus : Nat) : LexLeanPreservation.Obs :=
  LexLeanPreservation.Rel (__fits_0 m bonus) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Colls.mapFolding m bonus))

theorem root (m : (List (Prod Nat Nat))) (bonus : Nat) : LexLeanPreservation.RunConv __prog 0 [((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.nat)) m), (LexLeanTarget.TargetSyntax.Value.nat bonus)] (LexLeanPreservation.Rel (__fits_0 m bonus) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Colls.mapFolding m bonus))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 m bonus)

end LexLeanPreserve.C90d90c09a90803e7e527e747b4a162d8.R1
