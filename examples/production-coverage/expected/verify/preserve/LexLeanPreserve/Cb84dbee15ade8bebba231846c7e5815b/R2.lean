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
namespace LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2

def __prog : _root_.LexLeanTarget.TargetSyntax.Program :=
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
  ((((Bool.true && (Bool.true && Bool.true)) && (Nat.blt (acc + key) 18446744073709551616)) && (((Bool.true && (Bool.true && Bool.true)) && (Nat.blt (value + bonus) 18446744073709551616)) && Bool.true)) && (Nat.blt ((acc + key) + (value + bonus)) 18446744073709551616))

theorem __rel_2 (bonus : Nat) (acc : Nat) (key : Nat) (value : Nat) : _root_.LexLeanPreservation.FunRel __prog 2 [(_root_.LexLeanTarget.TargetSyntax.Value.nat bonus), (_root_.LexLeanTarget.TargetSyntax.Value.nat acc), (_root_.LexLeanTarget.TargetSyntax.Value.nat key), (_root_.LexLeanTarget.TargetSyntax.Value.nat value)] (_root_.LexLeanPreservation.Rel (__fits_2 bonus acc key value) (_root_.LexLeanTarget.TargetSyntax.Value.nat (((acc + key) + (value + bonus))))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natAdd acc key)) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natAdd value bonus)) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natAdd (acc + key) (value + bonus)))

def __fits_0 (m : (List (Prod Nat Nat))) (bonus : Nat) : Bool :=
  (((Bool.true && Bool.true) && (Bool.true && (Bool.true && Bool.true))) && (_root_.LexLeanPreservation.foldFits (fun __st __e => (fun (__p0 : Nat) (__p1 : Nat) (__p2 : Nat) => __fits_2 (bonus) __p0 __p1 __p2) __st __e.1 __e.2) (fun __st __e => ((fun (acc : Nat) (key : Nat) (value : Nat) => ((acc + key) + (value + bonus)))) __st __e.1 __e.2) ((0 : Nat)) (m)))

theorem __rel_0 (m : (List (Prod Nat Nat))) (bonus : Nat) : _root_.LexLeanPreservation.FunRel __prog 0 [((_root_.LexLeanPreservation.encList (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.nat _root_.LexLeanTarget.TargetSyntax.Value.nat)) m), (_root_.LexLeanTarget.TargetSyntax.Value.nat bonus)] (_root_.LexLeanPreservation.Rel (__fits_0 m bonus) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.Colls.mapFolding m bonus))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_closure (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil))) (_root_.LexLeanPreservation.tpl_mapFold (ek := _root_.LexLeanTarget.TargetSyntax.Value.nat) (ev := _root_.LexLeanTarget.TargetSyntax.Value.nat) (es := _root_.LexLeanTarget.TargetSyntax.Value.nat) (_root_.LexLeanPreservation.listEnc (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.nat _root_.LexLeanTarget.TargetSyntax.Value.nat)) (fun _ => rfl) ((fun (acc : Nat) (key : Nat) (value : Nat) => ((acc + key) + (value + bonus)))) (fun (__p0 : Nat) (__p1 : Nat) (__p2 : Nat) => __fits_2 (bonus) __p0 __p1 __p2) 2 [(_root_.LexLeanTarget.TargetSyntax.Value.nat bonus)] (fun (__p0 : Nat) (__p1 : Nat) (__p2 : Nat) => __rel_2 (bonus) __p0 __p1 __p2) (p := __prog) (fi := 1) (tk := .nat) (tw := .nat) (ts := .nat) rfl ((0 : Nat)) (m)))

def denote (m : (List (Prod Nat Nat))) (bonus : Nat) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 m bonus) (_root_.LexLeanPreservation.Obs.value ((_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.Colls.mapFolding m bonus)))) _root_.LexLeanPreservation.Obs.overflow

theorem root (m : (List (Prod Nat Nat))) (bonus : Nat) : _root_.LexLeanPreservation.RunConv __prog 0 [((_root_.LexLeanPreservation.encList (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.nat _root_.LexLeanTarget.TargetSyntax.Value.nat)) m), (_root_.LexLeanTarget.TargetSyntax.Value.nat bonus)] (_root_.LexLeanPreservation.Rel (__fits_0 m bonus) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.Colls.mapFolding m bonus))) :=
  _root_.LexLeanPreservation.run_of_funRel (__rel_0 m bonus)

def __valid_4 : Nat -> Bool :=
  fun _ => Bool.true

def __inv_4 : Nat -> Prop :=
  fun _ => _root_.True

theorem __viff_4 : ∀ (__v : Nat), __valid_4 __v = Bool.true ↔ __inv_4 __v :=
  fun __v => _root_.LexLeanPreservation.validTrue_inv __v

theorem __vrel_4 : ∀ (__v : Nat), _root_.LexLeanPreservation.FunRel __prog 4 [(_root_.LexLeanTarget.TargetSyntax.Value.nat __v)] (_root_.LexLeanPreservation.Rel Bool.true (_root_.LexLeanTarget.TargetSyntax.Value.bool (__valid_4 __v))) :=
  _root_.LexLeanPreservation.tpl_validTrue _root_.LexLeanTarget.TargetSyntax.Value.nat rfl

def __valid_3 : (List (Prod Nat Nat)) -> Bool :=
  _root_.LexLeanPreservation.ascMap (Coverage.Colls.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) __valid_4

def __inv_3 : (List (Prod Nat Nat)) -> Prop :=
  _root_.LexLeanPreservation.InvMap (Coverage.Colls.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) __inv_4

theorem __viff_3 : ∀ (__v : (List (Prod Nat Nat))), __valid_3 __v = Bool.true ↔ __inv_3 __v :=
  _root_.LexLeanPreservation.ascMap_inv (Coverage.Colls.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) __valid_4 __inv_4 __viff_4

theorem __vrel_3 : ∀ (__v : (List (Prod Nat Nat))), _root_.LexLeanPreservation.FunRel __prog 3 [((_root_.LexLeanPreservation.encList (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.nat _root_.LexLeanTarget.TargetSyntax.Value.nat)) __v)] (_root_.LexLeanPreservation.Rel Bool.true (_root_.LexLeanTarget.TargetSyntax.Value.bool (__valid_3 __v))) :=
  _root_.LexLeanPreservation.tpl_validMap (_root_.LexLeanPreservation.keySpec_nat (Coverage.Colls.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) (fun _ _ => rfl)) __valid_4 (_root_.LexLeanPreservation.listEnc (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.nat _root_.LexLeanTarget.TargetSyntax.Value.nat)) (fun _ => rfl) __vrel_4 rfl

def entryFits (m : (List (Prod Nat Nat))) (bonus : Nat) : Bool :=
  (if __valid_3 m then __fits_0 m bonus else Bool.true)

def entryValue (m : (List (Prod Nat Nat))) (bonus : Nat) : _root_.LexLeanTarget.TargetSyntax.Value :=
  (if __valid_3 m then (_root_.LexLeanTarget.TargetSyntax.Value.some (_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.Colls.mapFolding m bonus))) else _root_.LexLeanTarget.TargetSyntax.Value.none)

/-- The entry's observation. -/
def denoteEntry (m : (List (Prod Nat Nat))) (bonus : Nat) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (_root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.entryFits m bonus) (_root_.LexLeanPreservation.Obs.value (_root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.entryValue m bonus)) _root_.LexLeanPreservation.Obs.overflow

/-- §17.12's invariants of the validated parameters. -/
def accepts (m : (List (Prod Nat Nat))) (bonus : Nat) : Prop :=
  (__inv_3 m ∧ _root_.True)

theorem entry (m : (List (Prod Nat Nat))) (bonus : Nat) : _root_.LexLeanPreservation.RunConv __prog 5 [((_root_.LexLeanPreservation.encList (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.nat _root_.LexLeanTarget.TargetSyntax.Value.nat)) m), (_root_.LexLeanTarget.TargetSyntax.Value.nat bonus)] (_root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.denoteEntry m bonus) :=
  _root_.LexLeanPreservation.run_of_funRel (_root_.LexLeanPreservation.FunRel.fits_eq (_root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_cond (fun __c => if __c then __fits_0 m bonus else Bool.true) (fun __c => if __c then (_root_.LexLeanTarget.TargetSyntax.Value.some (_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.Colls.mapFolding m bonus))) else _root_.LexLeanTarget.TargetSyntax.Value.none) (__valid_3 m) (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__vrel_3 m)) (fun _ => _root_.LexLeanPreservation.Conv.fits_eq (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (__rel_0 m bonus)) _root_.LexLeanPreservation.convL_nil) _root_.LexLeanPreservation.construct_some) (by simp)) (fun _ => _root_.LexLeanPreservation.conv_build _root_.LexLeanPreservation.convL_nil _root_.LexLeanPreservation.construct_none))) (by simp [_root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.entryFits]))

theorem entry_accepts (m : (List (Prod Nat Nat))) (bonus : Nat) (__h : _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.accepts m bonus) : _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.denoteEntry m bonus = _root_.LexLeanPreservation.someObs (_root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.denote m bonus) := by
  unfold _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.accepts at __h
  unfold _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.denoteEntry _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.entryFits _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.entryValue _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.denote
  simp only [_root_.if_true, (__viff_3 m).mpr __h.1]
  cases __fits_0 m bonus <;> rfl

theorem entry_refuses (m : (List (Prod Nat Nat))) (bonus : Nat) (__h : ¬ _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.accepts m bonus) : _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.denoteEntry m bonus = _root_.LexLeanPreservation.Obs.value _root_.LexLeanTarget.TargetSyntax.Value.none := by
  unfold _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.accepts at __h
  unfold _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.denoteEntry _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.entryFits _root_.LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2.entryValue
  cases __c0 : __valid_3 m
  · simp
  · exact _root_.absurd ⟨(__viff_3 m).mp __c0, _root_.trivial⟩ __h

end LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R2
