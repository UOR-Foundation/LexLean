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
namespace LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R7

def __prog : LexLeanTarget.TargetSyntax.Program :=
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
  ((true && true) && (LexLeanPreservation.graphFits Coverage.Colls.LexLeanCollections.insertElement (g)))

theorem __rel_0 (g : (List (Prod String (List String)))) : LexLeanPreservation.FunRel __prog 0 [((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.string (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.string))) g)] (LexLeanPreservation.Rel (__fits_0 g) ((LexLeanPreservation.encOption (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.string)) (Coverage.Colls.stringGraph g))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (LexLeanPreservation.tpl_graphTopological (LexLeanPreservation.keySpec_string Coverage.Colls.LexLeanCollections.compareCodes rfl (fun _ _ => rfl) (fun _ _ => rfl) (fun _ _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.compareCodes, h]) (fun _ _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.compareCodes, h]) (fun _ _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.compareCodes, h]) (Coverage.Colls.LexLeanCollections.Key.compare : String -> String -> Ordering) (fun _ _ => rfl)) (LexLeanPreservation.listEnc LexLeanTarget.TargetSyntax.Value.string) (fun _ => rfl) (LexLeanPreservation.listEnc (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.string (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.string))) (fun _ => rfl) (LexLeanPreservation.optEnc (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.string)) (fun _ => rfl) (⟨(fun _ => rfl), (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.insertElement, h]), (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.insertElement, h]), (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.insertElement, h]), (fun _ => rfl), (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.removeElement, h]), (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.removeElement, h]), (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.removeElement, h]), (fun _ => rfl), (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.containsElement, h]), (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.containsElement, h]), (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.containsElement, h])⟩ : LexLeanPreservation.SetOps (Coverage.Colls.LexLeanCollections.Key.compare : String -> String -> Ordering) Coverage.Colls.LexLeanCollections.insertElement Coverage.Colls.LexLeanCollections.removeElement Coverage.Colls.LexLeanCollections.containsElement) (⟨(fun _ => rfl), (fun _ _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.lookupEntry, h]), (fun _ _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.lookupEntry, h]), (fun _ _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.lookupEntry, h])⟩ : LexLeanPreservation.LookOps (Coverage.Colls.LexLeanCollections.Key.compare : String -> String -> Ordering) (Coverage.Colls.LexLeanCollections.lookupEntry : String -> List (Prod String (List String)) -> Option (List String))) (g) (Coverage.Colls.LexLeanCollections.graphSuccessors (g)) (fun _ => rfl) (⟨(fun _ => rfl), (fun _ _ _ => rfl), (fun _ _ => rfl), (fun _ _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.topological, h] <;> rfl), (fun _ _ _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.topological, h])⟩ : LexLeanPreservation.TopoOps (Coverage.Colls.LexLeanCollections.graphSuccessors (g)) Coverage.Colls.LexLeanCollections.containsElement Coverage.Colls.LexLeanCollections.removeElement (Coverage.Colls.LexLeanCollections.topological (g))) (p := __prog) (fi := 1) (tk := .string) rfl rfl rfl rfl rfl rfl rfl rfl rfl rfl rfl rfl))

def denote (g : (List (Prod String (List String)))) : LexLeanPreservation.Obs :=
  cond (__fits_0 g) (LexLeanPreservation.Obs.value (((LexLeanPreservation.encOption (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.string)) (Coverage.Colls.stringGraph g)))) LexLeanPreservation.Obs.overflow

theorem root (g : (List (Prod String (List String)))) : LexLeanPreservation.RunConv __prog 0 [((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.string (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.string))) g)] (LexLeanPreservation.Rel (__fits_0 g) ((LexLeanPreservation.encOption (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.string)) (Coverage.Colls.stringGraph g))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 g)

def __valid_14 : (List String) -> Bool :=
  LexLeanPreservation.ascSet (Coverage.Colls.LexLeanCollections.Key.compare : String -> String -> Ordering)

def __inv_14 : (List String) -> Prop :=
  LexLeanPreservation.InvSet (Coverage.Colls.LexLeanCollections.Key.compare : String -> String -> Ordering)

theorem __viff_14 : ∀ (__v : (List String)), __valid_14 __v = true ↔ __inv_14 __v :=
  LexLeanPreservation.ascSet_inv (Coverage.Colls.LexLeanCollections.Key.compare : String -> String -> Ordering)

theorem __vrel_14 : ∀ (__v : (List String)), LexLeanPreservation.FunRel __prog 14 [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.string) __v)] (LexLeanPreservation.Rel true (LexLeanTarget.TargetSyntax.Value.bool (__valid_14 __v))) :=
  LexLeanPreservation.tpl_validSet (LexLeanPreservation.keySpec_string Coverage.Colls.LexLeanCollections.compareCodes rfl (fun _ _ => rfl) (fun _ _ => rfl) (fun _ _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.compareCodes, h]) (fun _ _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.compareCodes, h]) (fun _ _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.compareCodes, h]) (Coverage.Colls.LexLeanCollections.Key.compare : String -> String -> Ordering) (fun _ _ => rfl)) (LexLeanPreservation.listEnc LexLeanTarget.TargetSyntax.Value.string) (fun _ => rfl) rfl

def __valid_13 : (List (Prod String (List String))) -> Bool :=
  LexLeanPreservation.ascMap (Coverage.Colls.LexLeanCollections.Key.compare : String -> String -> Ordering) __valid_14

def __inv_13 : (List (Prod String (List String))) -> Prop :=
  LexLeanPreservation.InvMap (Coverage.Colls.LexLeanCollections.Key.compare : String -> String -> Ordering) __inv_14

theorem __viff_13 : ∀ (__v : (List (Prod String (List String)))), __valid_13 __v = true ↔ __inv_13 __v :=
  LexLeanPreservation.ascMap_inv (Coverage.Colls.LexLeanCollections.Key.compare : String -> String -> Ordering) __valid_14 __inv_14 __viff_14

theorem __vrel_13 : ∀ (__v : (List (Prod String (List String)))), LexLeanPreservation.FunRel __prog 13 [((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.string (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.string))) __v)] (LexLeanPreservation.Rel true (LexLeanTarget.TargetSyntax.Value.bool (__valid_13 __v))) :=
  LexLeanPreservation.tpl_validMap (LexLeanPreservation.keySpec_string Coverage.Colls.LexLeanCollections.compareCodes rfl (fun _ _ => rfl) (fun _ _ => rfl) (fun _ _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.compareCodes, h]) (fun _ _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.compareCodes, h]) (fun _ _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.compareCodes, h]) (Coverage.Colls.LexLeanCollections.Key.compare : String -> String -> Ordering) (fun _ _ => rfl)) __valid_14 (LexLeanPreservation.listEnc (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.string (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.string))) (fun _ => rfl) __vrel_14 rfl

def entryFits (g : (List (Prod String (List String)))) : Bool :=
  (if __valid_13 g then __fits_0 g else true)

def entryValue (g : (List (Prod String (List String)))) : LexLeanTarget.TargetSyntax.Value :=
  (if __valid_13 g then (LexLeanTarget.TargetSyntax.Value.some ((LexLeanPreservation.encOption (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.string)) (Coverage.Colls.stringGraph g))) else LexLeanTarget.TargetSyntax.Value.none)

/-- The entry's observation. -/
def denoteEntry (g : (List (Prod String (List String)))) : LexLeanPreservation.Obs :=
  cond (entryFits g) (LexLeanPreservation.Obs.value (entryValue g)) LexLeanPreservation.Obs.overflow

/-- §17.12's invariants of the validated parameters. -/
def accepts (g : (List (Prod String (List String)))) : Prop :=
  (__inv_13 g ∧ True)

theorem entry (g : (List (Prod String (List String)))) : LexLeanPreservation.RunConv __prog 15 [((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.string (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.string))) g)] (denoteEntry g) :=
  LexLeanPreservation.run_of_funRel (LexLeanPreservation.FunRel.fits_eq (LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_cond (fun __c => if __c then __fits_0 g else true) (fun __c => if __c then (LexLeanTarget.TargetSyntax.Value.some ((LexLeanPreservation.encOption (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.string)) (Coverage.Colls.stringGraph g))) else LexLeanTarget.TargetSyntax.Value.none) (__valid_13 g) (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__vrel_13 g)) (fun _ => LexLeanPreservation.Conv.fits_eq (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_0 g)) LexLeanPreservation.convL_nil) LexLeanPreservation.construct_some) (by simp)) (fun _ => LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_none))) (by simp [entryFits]))

theorem entry_accepts (g : (List (Prod String (List String)))) (__h : accepts g) : denoteEntry g = LexLeanPreservation.someObs (denote g) := by
  unfold accepts at __h
  unfold denoteEntry entryFits entryValue denote
  simp only [if_true, (__viff_13 g).mpr __h.1]
  cases __fits_0 g <;> rfl

theorem entry_refuses (g : (List (Prod String (List String)))) (__h : ¬ accepts g) : denoteEntry g = LexLeanPreservation.Obs.value LexLeanTarget.TargetSyntax.Value.none := by
  unfold accepts at __h
  unfold denoteEntry entryFits entryValue
  cases __c0 : __valid_13 g
  · simp
  · exact absurd ⟨(__viff_13 g).mp __c0, trivial⟩ __h

end LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R7
