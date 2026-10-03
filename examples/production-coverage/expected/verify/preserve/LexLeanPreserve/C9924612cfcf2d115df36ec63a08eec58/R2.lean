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
namespace LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R2

def __prog : LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [(.list (.pair .nat .nat)), .nat], result := .nat,
      body := (.call 1 [(.closure 2 [(.var 1)]), (.value .nat (.nat 0)), (.var 0)]) },
    { parameters := [0, 1, 2], types := [(.fn [.nat, .nat, .nat] .nat), .nat, (.list (.pair .nat .nat))], result := .nat,
      body := (.«match» .nat (.var 2) [(.arm .nil [] (.var 1)), (.arm .cons [3, 4] (.call 1 [(.var 0), (.apply (.var 0) [(.var 1), (.first (.var 3)), (.second (.var 3))]), (.var 4)]))]) },
    { parameters := [0, 1, 2, 3], types := [.nat, .nat, .nat, .nat], result := .nat,
      body := (.prim .natAdd [(.prim .natAdd [(.var 1), (.var 2)]), (.prim .natAdd [(.var 3), (.var 0)])]) },
    { parameters := [0], types := [(.list (.pair .nat .nat))], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .nil [] (.build .true .bool [])), (.arm .cons [1, 2] (.cond (.call 4 [(.second (.var 1))]) (.«match» .bool (.var 2) [(.arm .nil [] (.build .true .bool [])), (.arm .cons [3, 4] (.cond (.«match» .bool (.prim .compare [(.first (.var 1)), (.first (.var 3))]) [(.arm .lt [] (.build .true .bool [])), (.arm .eq [] (.build .false .bool [])), (.arm .gt [] (.build .false .bool []))]) (.call 3 [(.var 2)]) (.build .false .bool [])))]) (.build .false .bool [])))]) },
    { parameters := [0], types := [.nat], result := .bool,
      body := (.build .true .bool []) },
    { parameters := [0, 1], types := [(.list (.pair .nat .nat)), .nat], result := (.option .nat),
      body := (.cond (.call 3 [(.var 0)]) (.build .some (.option .nat) [(.call 0 [(.var 0), (.var 1)])]) (.build .none (.option .nat) [])) }] }

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

def __valid_4 : Nat -> Bool :=
  fun _ => true

def __inv_4 : Nat -> Prop :=
  fun _ => True

theorem __viff_4 : ∀ (__v : Nat), __valid_4 __v = true ↔ __inv_4 __v :=
  fun __v => LexLeanPreservation.validTrue_inv __v

theorem __vrel_4 : ∀ (__v : Nat), LexLeanPreservation.FunRel __prog 4 [(LexLeanTarget.TargetSyntax.Value.nat __v)] (LexLeanPreservation.Rel true (LexLeanTarget.TargetSyntax.Value.bool (__valid_4 __v))) :=
  LexLeanPreservation.tpl_validTrue LexLeanTarget.TargetSyntax.Value.nat rfl

def __valid_3 : (List (Prod Nat Nat)) -> Bool :=
  LexLeanPreservation.ascMap (Coverage.Colls.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) __valid_4

def __inv_3 : (List (Prod Nat Nat)) -> Prop :=
  LexLeanPreservation.InvMap (Coverage.Colls.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) __inv_4

theorem __viff_3 : ∀ (__v : (List (Prod Nat Nat))), __valid_3 __v = true ↔ __inv_3 __v :=
  LexLeanPreservation.ascMap_inv (Coverage.Colls.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) __valid_4 __inv_4 __viff_4

theorem __vrel_3 : ∀ (__v : (List (Prod Nat Nat))), LexLeanPreservation.FunRel __prog 3 [((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.nat)) __v)] (LexLeanPreservation.Rel true (LexLeanTarget.TargetSyntax.Value.bool (__valid_3 __v))) :=
  LexLeanPreservation.tpl_validMap (LexLeanPreservation.keySpec_nat (Coverage.Colls.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) (fun _ _ => rfl)) __valid_4 (LexLeanPreservation.listEnc (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.nat)) (fun _ => rfl) __vrel_4 rfl

def entryFits (m : (List (Prod Nat Nat))) (bonus : Nat) : Bool :=
  (if __valid_3 m then __fits_0 m bonus else true)

def entryValue (m : (List (Prod Nat Nat))) (bonus : Nat) : LexLeanTarget.TargetSyntax.Value :=
  (if __valid_3 m then (LexLeanTarget.TargetSyntax.Value.some (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Colls.mapFolding m bonus))) else LexLeanTarget.TargetSyntax.Value.none)

/-- The entry's observation. -/
def denoteEntry (m : (List (Prod Nat Nat))) (bonus : Nat) : LexLeanPreservation.Obs :=
  LexLeanPreservation.Rel (entryFits m bonus) (entryValue m bonus)

/-- §17.12's invariants of the validated parameters. -/
def accepts (m : (List (Prod Nat Nat))) (bonus : Nat) : Prop :=
  (__inv_3 m ∧ True)

theorem entry (m : (List (Prod Nat Nat))) (bonus : Nat) : LexLeanPreservation.RunConv __prog 5 [((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.nat)) m), (LexLeanTarget.TargetSyntax.Value.nat bonus)] (denoteEntry m bonus) :=
  LexLeanPreservation.run_of_funRel (LexLeanPreservation.FunRel.fits_eq (LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_cond (fun __c => if __c then __fits_0 m bonus else true) (fun __c => if __c then (LexLeanTarget.TargetSyntax.Value.some (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Colls.mapFolding m bonus))) else LexLeanTarget.TargetSyntax.Value.none) (__valid_3 m) (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__vrel_3 m)) (fun _ => LexLeanPreservation.Conv.fits_eq (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (__rel_0 m bonus)) LexLeanPreservation.convL_nil) LexLeanPreservation.construct_some) (by simp)) (fun _ => LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_none))) (by simp [entryFits]))

theorem entry_accepts (m : (List (Prod Nat Nat))) (bonus : Nat) (__h : accepts m bonus) : denoteEntry m bonus = LexLeanPreservation.someObs (denote m bonus) := by
  unfold accepts at __h
  unfold denoteEntry entryFits entryValue denote LexLeanPreservation.Rel
  simp only [if_true, (__viff_3 m).mpr __h.1]
  cases __fits_0 m bonus <;> rfl

theorem entry_refuses (m : (List (Prod Nat Nat))) (bonus : Nat) (__h : ¬ accepts m bonus) : denoteEntry m bonus = LexLeanPreservation.Obs.value LexLeanTarget.TargetSyntax.Value.none := by
  unfold accepts at __h
  unfold denoteEntry entryFits entryValue LexLeanPreservation.Rel
  cases __c0 : __valid_3 m
  · simp
  · exact absurd ⟨(__viff_3 m).mp __c0, trivial⟩ __h

end LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R2
