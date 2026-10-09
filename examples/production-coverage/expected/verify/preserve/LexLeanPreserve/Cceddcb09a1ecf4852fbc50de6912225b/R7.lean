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
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7

def __prog : _root_.LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0], types := [(.list (.pair .string (.list .string)))], result := (.option (.list .string)),
      body := (.call 1 [(.var 0)]) },
    { parameters := [0], types := [(.list (.pair .string (.list .string)))], result := (.option (.list .string)),
      body := (.«let» 1 (.list .string) (.call 6 [(.var 0), (.build .nil (.list .string) [])]) (.call 2 [(.var 0), (.prim .natAdd [(.prim .length [(.var 1)]), (.value .nat (.nat 1))]), (.var 1), (.build .nil (.list .string) [])])) },
    { parameters := [0, 1, 2, 3], types := [(.list (.pair .string (.list .string))), .nat, (.list .string), (.list .string)], result := (.option (.list .string)),
      body := (.«match» (.option (.list .string)) (.var 1) [(.arm .zero [] (.«match» (.option (.list .string)) (.var 2) [(.arm .nil [] (.build .some (.option (.list .string)) [(.call 5 [(.var 3), (.build .nil (.list .string) [])])])), (.arm .cons [4, 5] (.build .none (.option (.list .string)) []))])), (.arm .succ [6] (.«match» (.option (.list .string)) (.call 3 [(.var 0), (.var 2), (.var 2)]) [(.arm .nil [] (.«match» (.option (.list .string)) (.var 2) [(.arm .nil [] (.build .some (.option (.list .string)) [(.call 5 [(.var 3), (.build .nil (.list .string) [])])])), (.arm .cons [7, 8] (.build .none (.option (.list .string)) []))])), (.arm .cons [9, 10] (.call 2 [(.var 0), (.var 6), (.call 12 [(.var 2), (.var 9)]), (.build .cons (.list .string) [(.var 9), (.var 3)])]))]))]) },
    { parameters := [0, 1, 2], types := [(.list (.pair .string (.list .string))), (.list .string), (.list .string)], result := (.list .string),
      body := (.«match» (.list .string) (.var 2) [(.arm .nil [] (.build .nil (.list .string) [])), (.arm .cons [3, 4] (.cond (.call 4 [(.var 0), (.var 3), (.var 1)]) (.build .cons (.list .string) [(.var 3), (.call 3 [(.var 0), (.var 1), (.var 4)])]) (.call 3 [(.var 0), (.var 1), (.var 4)])))]) },
    { parameters := [0, 1, 2], types := [(.list (.pair .string (.list .string))), .string, (.list .string)], result := .bool,
      body := (.«match» .bool (.var 2) [(.arm .nil [] (.build .true .bool [])), (.arm .cons [3, 4] (.cond (.call 11 [(.call 9 [(.var 0), (.var 3)]), (.var 1)]) (.build .false .bool []) (.call 4 [(.var 0), (.var 1), (.var 4)])))]) },
    { parameters := [0, 1], types := [(.list .string), (.list .string)], result := (.list .string),
      body := (.«match» (.list .string) (.var 0) [(.arm .nil [] (.var 1)), (.arm .cons [2, 3] (.call 5 [(.var 3), (.build .cons (.list .string) [(.var 2), (.var 1)])]))]) },
    { parameters := [0, 1], types := [(.list (.pair .string (.list .string))), (.list .string)], result := (.list .string),
      body := (.«match» (.list .string) (.var 0) [(.arm .nil [] (.var 1)), (.arm .cons [2, 3] (.call 6 [(.var 3), (.call 7 [(.second (.var 2)), (.call 8 [(.var 1), (.first (.var 2))])])]))]) },
    { parameters := [0, 1], types := [(.list .string), (.list .string)], result := (.list .string),
      body := (.«match» (.list .string) (.var 0) [(.arm .nil [] (.var 1)), (.arm .cons [2, 3] (.call 7 [(.var 3), (.call 8 [(.var 1), (.var 2)])]))]) },
    { parameters := [0, 1], types := [(.list .string), .string], result := (.list .string),
      body := (.«match» (.list .string) (.var 0) [(.arm .nil [] (.build .cons (.list .string) [(.var 1), (.build .nil (.list .string) [])])), (.arm .cons [2, 3] (.«match» (.list .string) (.prim .compare [(.var 1), (.var 2)]) [(.arm .lt [] (.build .cons (.list .string) [(.var 1), (.var 0)])), (.arm .eq [] (.var 0)), (.arm .gt [] (.build .cons (.list .string) [(.var 2), (.call 8 [(.var 3), (.var 1)])]))]))]) },
    { parameters := [0, 1], types := [(.list (.pair .string (.list .string))), .string], result := (.list .string),
      body := (.«match» (.list .string) (.call 10 [(.var 0), (.var 1)]) [(.arm .none [] (.build .nil (.list .string) [])), (.arm .some [2] (.var 2))]) },
    { parameters := [0, 1], types := [(.list (.pair .string (.list .string))), .string], result := (.option (.list .string)),
      body := (.«match» (.option (.list .string)) (.var 0) [(.arm .nil [] (.build .none (.option (.list .string)) [])), (.arm .cons [2, 3] (.«match» (.option (.list .string)) (.prim .compare [(.var 1), (.first (.var 2))]) [(.arm .lt [] (.build .none (.option (.list .string)) [])), (.arm .eq [] (.build .some (.option (.list .string)) [(.second (.var 2))])), (.arm .gt [] (.call 10 [(.var 3), (.var 1)]))]))]) },
    { parameters := [0, 1], types := [(.list .string), .string], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .nil [] (.build .false .bool [])), (.arm .cons [2, 3] (.«match» .bool (.prim .compare [(.var 1), (.var 2)]) [(.arm .lt [] (.build .false .bool [])), (.arm .eq [] (.build .true .bool [])), (.arm .gt [] (.call 11 [(.var 3), (.var 1)]))]))]) },
    { parameters := [0, 1], types := [(.list .string), .string], result := (.list .string),
      body := (.«match» (.list .string) (.var 0) [(.arm .nil [] (.build .nil (.list .string) [])), (.arm .cons [2, 3] (.«match» (.list .string) (.prim .compare [(.var 1), (.var 2)]) [(.arm .lt [] (.var 0)), (.arm .eq [] (.var 3)), (.arm .gt [] (.build .cons (.list .string) [(.var 2), (.call 12 [(.var 3), (.var 1)])]))]))]) },
    { parameters := [0], types := [(.list (.pair .string (.list .string)))], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .nil [] (.build .true .bool [])), (.arm .cons [1, 2] (.cond (.call 14 [(.second (.var 1))]) (.«match» .bool (.var 2) [(.arm .nil [] (.build .true .bool [])), (.arm .cons [3, 4] (.cond (.«match» .bool (.prim .compare [(.first (.var 1)), (.first (.var 3))]) [(.arm .lt [] (.build .true .bool [])), (.arm .eq [] (.build .false .bool [])), (.arm .gt [] (.build .false .bool []))]) (.call 13 [(.var 2)]) (.build .false .bool [])))]) (.build .false .bool [])))]) },
    { parameters := [0], types := [(.list .string)], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .nil [] (.build .true .bool [])), (.arm .cons [1, 2] (.«match» .bool (.var 2) [(.arm .nil [] (.build .true .bool [])), (.arm .cons [3, 4] (.cond (.«match» .bool (.prim .compare [(.var 1), (.var 3)]) [(.arm .lt [] (.build .true .bool [])), (.arm .eq [] (.build .false .bool [])), (.arm .gt [] (.build .false .bool []))]) (.call 14 [(.var 2)]) (.build .false .bool [])))]))]) },
    { parameters := [0], types := [(.list (.pair .string (.list .string)))], result := (.option (.option (.list .string))),
      body := (.cond (.call 13 [(.var 0)]) (.build .some (.option (.option (.list .string))) [(.call 0 [(.var 0)])]) (.build .none (.option (.option (.list .string))) [])) }] }

def __fits_0 (g : (List (Prod String (List String)))) : Bool :=
  ((Bool.true && Bool.true) && (_root_.LexLeanPreservation.graphFits Coverage.Colls.LexLeanCollections.insertElement (g)))

theorem __rel_0 (g : (List (Prod String (List String)))) : _root_.LexLeanPreservation.FunRel __prog 0 [((_root_.LexLeanPreservation.encList (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.string (_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.string))) g)] (_root_.LexLeanPreservation.Rel (__fits_0 g) ((_root_.LexLeanPreservation.encOption (_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.string)) (Coverage.Colls.stringGraph g))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (_root_.LexLeanPreservation.tpl_graphTopological (_root_.LexLeanPreservation.keySpec_string Coverage.Colls.LexLeanCollections.compareCodes rfl (fun _ _ => rfl) (fun _ _ => rfl) (fun _ _ _ _ __h => by simp only [Coverage.Colls.LexLeanCollections.compareCodes, __h]) (fun _ _ _ _ __h => by simp only [Coverage.Colls.LexLeanCollections.compareCodes, __h]) (fun _ _ _ _ __h => by simp only [Coverage.Colls.LexLeanCollections.compareCodes, __h]) (Coverage.Colls.LexLeanCollections.Key.compare : String -> String -> Ordering) (fun _ _ => rfl)) (_root_.LexLeanPreservation.listEnc _root_.LexLeanTarget.TargetSyntax.Value.string) (fun _ => rfl) (_root_.LexLeanPreservation.listEnc (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.string (_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.string))) (fun _ => rfl) (_root_.LexLeanPreservation.optEnc (_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.string)) (fun _ => rfl) (⟨(fun _ => rfl), (fun _ _ _ __h => by simp only [Coverage.Colls.LexLeanCollections.insertElement, __h]), (fun _ _ _ __h => by simp only [Coverage.Colls.LexLeanCollections.insertElement, __h]), (fun _ _ _ __h => by simp only [Coverage.Colls.LexLeanCollections.insertElement, __h]), (fun _ => rfl), (fun _ _ _ __h => by simp only [Coverage.Colls.LexLeanCollections.removeElement, __h]), (fun _ _ _ __h => by simp only [Coverage.Colls.LexLeanCollections.removeElement, __h]), (fun _ _ _ __h => by simp only [Coverage.Colls.LexLeanCollections.removeElement, __h]), (fun _ => rfl), (fun _ _ _ __h => by simp only [Coverage.Colls.LexLeanCollections.containsElement, __h]), (fun _ _ _ __h => by simp only [Coverage.Colls.LexLeanCollections.containsElement, __h]), (fun _ _ _ __h => by simp only [Coverage.Colls.LexLeanCollections.containsElement, __h])⟩ : _root_.LexLeanPreservation.SetOps (Coverage.Colls.LexLeanCollections.Key.compare : String -> String -> Ordering) Coverage.Colls.LexLeanCollections.insertElement Coverage.Colls.LexLeanCollections.removeElement Coverage.Colls.LexLeanCollections.containsElement) (⟨(fun _ => rfl), (fun _ _ _ _ __h => by simp only [Coverage.Colls.LexLeanCollections.lookupEntry, __h]), (fun _ _ _ _ __h => by simp only [Coverage.Colls.LexLeanCollections.lookupEntry, __h]), (fun _ _ _ _ __h => by simp only [Coverage.Colls.LexLeanCollections.lookupEntry, __h])⟩ : _root_.LexLeanPreservation.LookOps (Coverage.Colls.LexLeanCollections.Key.compare : String -> String -> Ordering) (Coverage.Colls.LexLeanCollections.lookupEntry : String -> List (Prod String (List String)) -> Option (List String))) (g) (Coverage.Colls.LexLeanCollections.graphSuccessors (g)) (fun _ => rfl) (⟨(fun _ => rfl), (fun _ _ _ => rfl), (fun _ _ => rfl), (fun _ _ _ _ __h => by simp only [Coverage.Colls.LexLeanCollections.topological, __h] <;> rfl), (fun _ _ _ _ _ __h => by simp only [Coverage.Colls.LexLeanCollections.topological, __h])⟩ : _root_.LexLeanPreservation.TopoOps (Coverage.Colls.LexLeanCollections.graphSuccessors (g)) Coverage.Colls.LexLeanCollections.containsElement Coverage.Colls.LexLeanCollections.removeElement (Coverage.Colls.LexLeanCollections.topological (g))) (p := __prog) (fi := 1) (tk := .string) rfl rfl rfl rfl rfl rfl rfl rfl rfl rfl rfl rfl))

def denote (g : (List (Prod String (List String)))) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 g) (_root_.LexLeanPreservation.Obs.value (((_root_.LexLeanPreservation.encOption (_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.string)) (Coverage.Colls.stringGraph g)))) _root_.LexLeanPreservation.Obs.overflow

theorem root (g : (List (Prod String (List String)))) : _root_.LexLeanPreservation.RunConv __prog 0 [((_root_.LexLeanPreservation.encList (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.string (_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.string))) g)] (_root_.LexLeanPreservation.Rel (__fits_0 g) ((_root_.LexLeanPreservation.encOption (_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.string)) (Coverage.Colls.stringGraph g))) :=
  _root_.LexLeanPreservation.run_of_funRel (__rel_0 g)

def __valid_14 : (List String) -> Bool :=
  _root_.LexLeanPreservation.ascSet (Coverage.Colls.LexLeanCollections.Key.compare : String -> String -> Ordering)

def __inv_14 : (List String) -> Prop :=
  _root_.LexLeanPreservation.InvSet (Coverage.Colls.LexLeanCollections.Key.compare : String -> String -> Ordering)

theorem __viff_14 : ∀ (__v : (List String)), __valid_14 __v = Bool.true ↔ __inv_14 __v :=
  _root_.LexLeanPreservation.ascSet_inv (Coverage.Colls.LexLeanCollections.Key.compare : String -> String -> Ordering)

theorem __vrel_14 : ∀ (__v : (List String)), _root_.LexLeanPreservation.FunRel __prog 14 [((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.string) __v)] (_root_.LexLeanPreservation.Rel Bool.true (_root_.LexLeanTarget.TargetSyntax.Value.bool (__valid_14 __v))) :=
  _root_.LexLeanPreservation.tpl_validSet (_root_.LexLeanPreservation.keySpec_string Coverage.Colls.LexLeanCollections.compareCodes rfl (fun _ _ => rfl) (fun _ _ => rfl) (fun _ _ _ _ __h => by simp only [Coverage.Colls.LexLeanCollections.compareCodes, __h]) (fun _ _ _ _ __h => by simp only [Coverage.Colls.LexLeanCollections.compareCodes, __h]) (fun _ _ _ _ __h => by simp only [Coverage.Colls.LexLeanCollections.compareCodes, __h]) (Coverage.Colls.LexLeanCollections.Key.compare : String -> String -> Ordering) (fun _ _ => rfl)) (_root_.LexLeanPreservation.listEnc _root_.LexLeanTarget.TargetSyntax.Value.string) (fun _ => rfl) rfl

def __valid_13 : (List (Prod String (List String))) -> Bool :=
  _root_.LexLeanPreservation.ascMap (Coverage.Colls.LexLeanCollections.Key.compare : String -> String -> Ordering) __valid_14

def __inv_13 : (List (Prod String (List String))) -> Prop :=
  _root_.LexLeanPreservation.InvMap (Coverage.Colls.LexLeanCollections.Key.compare : String -> String -> Ordering) __inv_14

theorem __viff_13 : ∀ (__v : (List (Prod String (List String)))), __valid_13 __v = Bool.true ↔ __inv_13 __v :=
  _root_.LexLeanPreservation.ascMap_inv (Coverage.Colls.LexLeanCollections.Key.compare : String -> String -> Ordering) __valid_14 __inv_14 __viff_14

theorem __vrel_13 : ∀ (__v : (List (Prod String (List String)))), _root_.LexLeanPreservation.FunRel __prog 13 [((_root_.LexLeanPreservation.encList (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.string (_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.string))) __v)] (_root_.LexLeanPreservation.Rel Bool.true (_root_.LexLeanTarget.TargetSyntax.Value.bool (__valid_13 __v))) :=
  _root_.LexLeanPreservation.tpl_validMap (_root_.LexLeanPreservation.keySpec_string Coverage.Colls.LexLeanCollections.compareCodes rfl (fun _ _ => rfl) (fun _ _ => rfl) (fun _ _ _ _ __h => by simp only [Coverage.Colls.LexLeanCollections.compareCodes, __h]) (fun _ _ _ _ __h => by simp only [Coverage.Colls.LexLeanCollections.compareCodes, __h]) (fun _ _ _ _ __h => by simp only [Coverage.Colls.LexLeanCollections.compareCodes, __h]) (Coverage.Colls.LexLeanCollections.Key.compare : String -> String -> Ordering) (fun _ _ => rfl)) __valid_14 (_root_.LexLeanPreservation.listEnc (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.string (_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.string))) (fun _ => rfl) __vrel_14 rfl

def entryFits (g : (List (Prod String (List String)))) : Bool :=
  (if __valid_13 g then __fits_0 g else Bool.true)

def entryValue (g : (List (Prod String (List String)))) : _root_.LexLeanTarget.TargetSyntax.Value :=
  (if __valid_13 g then (_root_.LexLeanTarget.TargetSyntax.Value.some ((_root_.LexLeanPreservation.encOption (_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.string)) (Coverage.Colls.stringGraph g))) else _root_.LexLeanTarget.TargetSyntax.Value.none)

/-- The entry's observation. -/
def denoteEntry (g : (List (Prod String (List String)))) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (_root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.entryFits g) (_root_.LexLeanPreservation.Obs.value (_root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.entryValue g)) _root_.LexLeanPreservation.Obs.overflow

/-- §17.12's invariants of the validated parameters. -/
def accepts (g : (List (Prod String (List String)))) : Prop :=
  (__inv_13 g ∧ _root_.True)

theorem entry (g : (List (Prod String (List String)))) : _root_.LexLeanPreservation.RunConv __prog 15 [((_root_.LexLeanPreservation.encList (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.string (_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.string))) g)] (_root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.denoteEntry g) :=
  _root_.LexLeanPreservation.run_of_funRel (_root_.LexLeanPreservation.FunRel.fits_eq (_root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_cond (fun __c => if __c then __fits_0 g else Bool.true) (fun __c => if __c then (_root_.LexLeanTarget.TargetSyntax.Value.some ((_root_.LexLeanPreservation.encOption (_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.string)) (Coverage.Colls.stringGraph g))) else _root_.LexLeanTarget.TargetSyntax.Value.none) (__valid_13 g) (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__vrel_13 g)) (fun _ => _root_.LexLeanPreservation.Conv.fits_eq (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__rel_0 g)) _root_.LexLeanPreservation.convL_nil) _root_.LexLeanPreservation.construct_some) (by simp)) (fun _ => _root_.LexLeanPreservation.conv_build _root_.LexLeanPreservation.convL_nil _root_.LexLeanPreservation.construct_none))) (by simp [_root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.entryFits]))

theorem entry_accepts (g : (List (Prod String (List String)))) (__h : _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.accepts g) : _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.denoteEntry g = _root_.LexLeanPreservation.someObs (_root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.denote g) := by
  unfold _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.accepts at __h
  unfold _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.denoteEntry _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.entryFits _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.entryValue _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.denote
  simp only [_root_.if_true, (__viff_13 g).mpr __h.1]
  cases __fits_0 g <;> rfl

theorem entry_refuses (g : (List (Prod String (List String)))) (__h : ¬ _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.accepts g) : _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.denoteEntry g = _root_.LexLeanPreservation.Obs.value _root_.LexLeanTarget.TargetSyntax.Value.none := by
  unfold _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.accepts at __h
  unfold _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.denoteEntry _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.entryFits _root_.LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7.entryValue
  cases __c0 : __valid_13 g
  · simp
  · exact _root_.absurd ⟨(__viff_13 g).mp __c0, _root_.trivial⟩ __h

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R7
