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
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4

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
      body := (.prim .intSub [(.var 0), (.var 1)]) },
    { parameters := [0], types := [(.list .int)], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .nil [] (.build .true .bool [])), (.arm .cons [1, 2] (.«match» .bool (.var 2) [(.arm .nil [] (.build .true .bool [])), (.arm .cons [3, 4] (.cond (.«match» .bool (.prim .compare [(.var 1), (.var 3)]) [(.arm .lt [] (.build .true .bool [])), (.arm .eq [] (.build .false .bool [])), (.arm .gt [] (.build .false .bool []))]) (.call 5 [(.var 2)]) (.build .false .bool [])))]))]) },
    { parameters := [0, 1], types := [(.list .nat), (.list .int)], result := (.option (.pair .nat .int)),
      body := (.cond (.call 5 [(.var 1)]) (.build .some (.option (.pair .nat .int)) [(.call 0 [(.var 0), (.var 1)])]) (.build .none (.option (.pair .nat .int)) [])) }] }

def __fits_2 (a : Nat) (x : Nat) : Bool :=
  ((Bool.true && (Bool.true && Bool.true)) && (Nat.blt (a + x) 18446744073709551616))

theorem __rel_2 (a : Nat) (x : Nat) : LexLeanPreservation.FunRel __prog 2 [(LexLeanTarget.TargetSyntax.Value.nat a), (LexLeanTarget.TargetSyntax.Value.nat x)] (LexLeanPreservation.Rel (__fits_2 a x) (LexLeanTarget.TargetSyntax.Value.nat ((a + x)))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd a x))

def __fits_4 (a : Int) (x : Int) : Bool :=
  ((Bool.true && (Bool.true && Bool.true)) && (LexLeanPreservation.intFits (Coverage.Colls.LexLeanRuntime.subtract (a) (x) : Int)))

theorem __rel_4 (a : Int) (x : Int) : LexLeanPreservation.FunRel __prog 4 [(LexLeanTarget.TargetSyntax.Value.int a), (LexLeanTarget.TargetSyntax.Value.int x)] (LexLeanPreservation.Rel (__fits_4 a x) (LexLeanTarget.TargetSyntax.Value.int ((Coverage.Colls.LexLeanRuntime.subtract (a) (x) : Int)))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_intSub a x))

def __fits_0 (xs : (List Nat)) (s : (List Int)) : Bool :=
  ((((Bool.true && (Bool.true && (Bool.true && Bool.true))) && (LexLeanPreservation.foldFits (fun (__p0 : Nat) (__p1 : Nat) => __fits_2 __p0 __p1) ((fun (a : Nat) (x : Nat) => (a + x))) ((0 : Nat)) (xs))) && (((Bool.true && (Bool.true && (Bool.true && Bool.true))) && (LexLeanPreservation.foldFits (fun (__p0 : Int) (__p1 : Int) => __fits_4 __p0 __p1) ((fun (a : Int) (x : Int) => (Coverage.Colls.LexLeanRuntime.subtract (a) (x) : Int))) ((5 : Int)) (s))) && Bool.true)) && Bool.true)

theorem __rel_0 (xs : (List Nat)) (s : (List Int)) : LexLeanPreservation.FunRel __prog 0 [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) xs), ((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.int) s)] (LexLeanPreservation.Rel (__fits_0 xs s) ((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.int) (Coverage.Colls.folds xs s))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_closure LexLeanPreservation.convL_nil) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil))) (LexLeanPreservation.tpl_listFold (ea := LexLeanTarget.TargetSyntax.Value.nat) (es := LexLeanTarget.TargetSyntax.Value.nat) (LexLeanPreservation.listEnc LexLeanTarget.TargetSyntax.Value.nat) (fun _ => rfl) ((fun (a : Nat) (x : Nat) => (a + x))) (fun (__p0 : Nat) (__p1 : Nat) => __fits_2 __p0 __p1) 2 [] (fun (__p0 : Nat) (__p1 : Nat) => __rel_2 __p0 __p1) (p := __prog) (fi := 1) (ta := .nat) (ts := .nat) rfl ((0 : Nat)) (xs))) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_closure LexLeanPreservation.convL_nil) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil))) (LexLeanPreservation.tpl_listFold (ea := LexLeanTarget.TargetSyntax.Value.int) (es := LexLeanTarget.TargetSyntax.Value.int) (LexLeanPreservation.listEnc LexLeanTarget.TargetSyntax.Value.int) (fun _ => rfl) ((fun (a : Int) (x : Int) => (Coverage.Colls.LexLeanRuntime.subtract (a) (x) : Int))) (fun (__p0 : Int) (__p1 : Int) => __fits_4 __p0 __p1) 4 [] (fun (__p0 : Int) (__p1 : Int) => __rel_4 __p0 __p1) (p := __prog) (fi := 3) (ta := .int) (ts := .int) rfl ((5 : Int)) (s))) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair)

def denote (xs : (List Nat)) (s : (List Int)) : LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 xs s) (LexLeanPreservation.Obs.value (((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.int) (Coverage.Colls.folds xs s)))) LexLeanPreservation.Obs.overflow

theorem root (xs : (List Nat)) (s : (List Int)) : LexLeanPreservation.RunConv __prog 0 [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) xs), ((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.int) s)] (LexLeanPreservation.Rel (__fits_0 xs s) ((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.int) (Coverage.Colls.folds xs s))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 xs s)

def __valid_5 : (List Int) -> Bool :=
  LexLeanPreservation.ascSet (Coverage.Colls.LexLeanCollections.Key.compare : Int -> Int -> Ordering)

def __inv_5 : (List Int) -> Prop :=
  LexLeanPreservation.InvSet (Coverage.Colls.LexLeanCollections.Key.compare : Int -> Int -> Ordering)

theorem __viff_5 : ∀ (__v : (List Int)), __valid_5 __v = Bool.true ↔ __inv_5 __v :=
  LexLeanPreservation.ascSet_inv (Coverage.Colls.LexLeanCollections.Key.compare : Int -> Int -> Ordering)

theorem __vrel_5 : ∀ (__v : (List Int)), LexLeanPreservation.FunRel __prog 5 [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.int) __v)] (LexLeanPreservation.Rel Bool.true (LexLeanTarget.TargetSyntax.Value.bool (__valid_5 __v))) :=
  LexLeanPreservation.tpl_validSet (LexLeanPreservation.keySpec_int (Coverage.Colls.LexLeanCollections.Key.compare : Int -> Int -> Ordering) (fun _ _ => rfl)) (LexLeanPreservation.listEnc LexLeanTarget.TargetSyntax.Value.int) (fun _ => rfl) rfl

def entryFits (xs : (List Nat)) (s : (List Int)) : Bool :=
  (if __valid_5 s then __fits_0 xs s else Bool.true)

def entryValue (xs : (List Nat)) (s : (List Int)) : LexLeanTarget.TargetSyntax.Value :=
  (if __valid_5 s then (LexLeanTarget.TargetSyntax.Value.some ((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.int) (Coverage.Colls.folds xs s))) else LexLeanTarget.TargetSyntax.Value.none)

/-- The entry's observation. -/
def denoteEntry (xs : (List Nat)) (s : (List Int)) : LexLeanPreservation.Obs :=
  _root_.cond (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.entryFits xs s) (LexLeanPreservation.Obs.value (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.entryValue xs s)) LexLeanPreservation.Obs.overflow

/-- §17.12's invariants of the validated parameters. -/
def accepts (xs : (List Nat)) (s : (List Int)) : Prop :=
  (__inv_5 s ∧ _root_.True)

theorem entry (xs : (List Nat)) (s : (List Int)) : LexLeanPreservation.RunConv __prog 6 [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) xs), ((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.int) s)] (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.denoteEntry xs s) :=
  LexLeanPreservation.run_of_funRel (LexLeanPreservation.FunRel.fits_eq (LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_cond (fun __c => if __c then __fits_0 xs s else Bool.true) (fun __c => if __c then (LexLeanTarget.TargetSyntax.Value.some ((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.int) (Coverage.Colls.folds xs s))) else LexLeanTarget.TargetSyntax.Value.none) (__valid_5 s) (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__vrel_5 s)) (fun _ => LexLeanPreservation.Conv.fits_eq (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (__rel_0 xs s)) LexLeanPreservation.convL_nil) LexLeanPreservation.construct_some) (by simp)) (fun _ => LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_none))) (by simp [LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.entryFits]))

theorem entry_accepts (xs : (List Nat)) (s : (List Int)) (__h : LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.accepts xs s) : LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.denoteEntry xs s = LexLeanPreservation.someObs (LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.denote xs s) := by
  unfold LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.accepts at __h
  unfold LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.denoteEntry LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.entryFits LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.entryValue LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.denote
  simp only [_root_.if_true, (__viff_5 s).mpr __h.1]
  cases __fits_0 xs s <;> rfl

theorem entry_refuses (xs : (List Nat)) (s : (List Int)) (__h : ¬ LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.accepts xs s) : LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.denoteEntry xs s = LexLeanPreservation.Obs.value LexLeanTarget.TargetSyntax.Value.none := by
  unfold LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.accepts at __h
  unfold LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.denoteEntry LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.entryFits LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4.entryValue
  cases __c0 : __valid_5 s
  · simp
  · exact _root_.absurd ⟨(__viff_5 s).mp __c0, _root_.trivial⟩ __h

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R4
