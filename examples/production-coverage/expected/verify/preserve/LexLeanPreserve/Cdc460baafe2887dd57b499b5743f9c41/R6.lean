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
namespace LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R6

def __prog : LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [(.list (.pair .nat (.list .nat))), .nat], result := (.pair (.list .nat) (.pair (.list .nat) (.option (.list .nat)))),
      body := (.build .pair (.pair (.list .nat) (.pair (.list .nat) (.option (.list .nat)))) [(.call 1 [(.var 0), (.var 1)]), (.build .pair (.pair (.list .nat) (.option (.list .nat))) [(.call 3 [(.var 0), (.var 1)]), (.call 16 [(.var 0)])])]) },
    { parameters := [0, 1], types := [(.list (.pair .nat (.list .nat))), .nat], result := (.list .nat),
      body := (.«match» (.list .nat) (.call 2 [(.var 0), (.var 1)]) [(.arm .none [] (.build .nil (.list .nat) [])), (.arm .some [2] (.var 2))]) },
    { parameters := [0, 1], types := [(.list (.pair .nat (.list .nat))), .nat], result := (.option (.list .nat)),
      body := (.«match» (.option (.list .nat)) (.var 0) [(.arm .nil [] (.build .none (.option (.list .nat)) [])), (.arm .cons [2, 3] (.«match» (.option (.list .nat)) (.prim .compare [(.var 1), (.first (.var 2))]) [(.arm .lt [] (.build .none (.option (.list .nat)) [])), (.arm .eq [] (.build .some (.option (.list .nat)) [(.second (.var 2))])), (.arm .gt [] (.call 2 [(.var 3), (.var 1)]))]))]) },
    { parameters := [0, 1], types := [(.list (.pair .nat (.list .nat))), .nat], result := (.list .nat),
      body := (.call 4 [(.var 0), (.prim .natAdd [(.prim .length [(.call 13 [(.var 0), (.build .nil (.list .nat) [])])]), (.value .nat (.nat 1))]), (.build .cons (.list .nat) [(.var 1), (.build .nil (.list .nat) [])]), (.build .cons (.list .nat) [(.var 1), (.build .nil (.list .nat) [])])]) },
    { parameters := [0, 1, 2, 3], types := [(.list (.pair .nat (.list .nat))), .nat, (.list .nat), (.list .nat)], result := (.list .nat),
      body := (.«match» (.list .nat) (.var 1) [(.arm .zero [] (.var 3)), (.arm .succ [4] (.«let» 5 (.list .nat) (.call 5 [(.var 0), (.var 3), (.var 2), (.build .nil (.list .nat) [])]) (.«match» (.list .nat) (.var 5) [(.arm .nil [] (.var 3)), (.arm .cons [6, 7] (.call 4 [(.var 0), (.var 4), (.var 5), (.call 11 [(.var 3), (.var 5)])]))])))]) },
    { parameters := [0, 1, 2, 3], types := [(.list (.pair .nat (.list .nat))), (.list .nat), (.list .nat), (.list .nat)], result := (.list .nat),
      body := (.«match» (.list .nat) (.var 2) [(.arm .nil [] (.var 3)), (.arm .cons [4, 5] (.call 5 [(.var 0), (.var 1), (.var 5), (.call 6 [(.var 1), (.call 7 [(.var 0), (.var 4)]), (.var 3)])]))]) },
    { parameters := [0, 1, 2], types := [(.list .nat), (.list .nat), (.list .nat)], result := (.list .nat),
      body := (.«match» (.list .nat) (.var 1) [(.arm .nil [] (.var 2)), (.arm .cons [3, 4] (.call 6 [(.var 0), (.var 4), (.cond (.call 9 [(.var 0), (.var 3)]) (.var 2) (.cond (.call 9 [(.var 2), (.var 3)]) (.var 2) (.call 10 [(.var 2), (.var 3)])))]))]) },
    { parameters := [0, 1], types := [(.list (.pair .nat (.list .nat))), .nat], result := (.list .nat),
      body := (.«match» (.list .nat) (.call 8 [(.var 0), (.var 1)]) [(.arm .none [] (.build .nil (.list .nat) [])), (.arm .some [2] (.var 2))]) },
    { parameters := [0, 1], types := [(.list (.pair .nat (.list .nat))), .nat], result := (.option (.list .nat)),
      body := (.«match» (.option (.list .nat)) (.var 0) [(.arm .nil [] (.build .none (.option (.list .nat)) [])), (.arm .cons [2, 3] (.«match» (.option (.list .nat)) (.prim .compare [(.var 1), (.first (.var 2))]) [(.arm .lt [] (.build .none (.option (.list .nat)) [])), (.arm .eq [] (.build .some (.option (.list .nat)) [(.second (.var 2))])), (.arm .gt [] (.call 8 [(.var 3), (.var 1)]))]))]) },
    { parameters := [0, 1], types := [(.list .nat), .nat], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .nil [] (.build .false .bool [])), (.arm .cons [2, 3] (.«match» .bool (.prim .compare [(.var 1), (.var 2)]) [(.arm .lt [] (.build .false .bool [])), (.arm .eq [] (.build .true .bool [])), (.arm .gt [] (.call 9 [(.var 3), (.var 1)]))]))]) },
    { parameters := [0, 1], types := [(.list .nat), .nat], result := (.list .nat),
      body := (.«match» (.list .nat) (.var 0) [(.arm .nil [] (.build .cons (.list .nat) [(.var 1), (.build .nil (.list .nat) [])])), (.arm .cons [2, 3] (.«match» (.list .nat) (.prim .compare [(.var 1), (.var 2)]) [(.arm .lt [] (.build .cons (.list .nat) [(.var 1), (.var 0)])), (.arm .eq [] (.var 0)), (.arm .gt [] (.build .cons (.list .nat) [(.var 2), (.call 10 [(.var 3), (.var 1)])]))]))]) },
    { parameters := [0, 1], types := [(.list .nat), (.list .nat)], result := (.list .nat),
      body := (.«match» (.list .nat) (.var 1) [(.arm .nil [] (.var 0)), (.arm .cons [2, 3] (.call 11 [(.call 12 [(.var 0), (.var 2)]), (.var 3)]))]) },
    { parameters := [0, 1], types := [(.list .nat), .nat], result := (.list .nat),
      body := (.«match» (.list .nat) (.var 0) [(.arm .nil [] (.build .cons (.list .nat) [(.var 1), (.build .nil (.list .nat) [])])), (.arm .cons [2, 3] (.«match» (.list .nat) (.prim .compare [(.var 1), (.var 2)]) [(.arm .lt [] (.build .cons (.list .nat) [(.var 1), (.var 0)])), (.arm .eq [] (.var 0)), (.arm .gt [] (.build .cons (.list .nat) [(.var 2), (.call 12 [(.var 3), (.var 1)])]))]))]) },
    { parameters := [0, 1], types := [(.list (.pair .nat (.list .nat))), (.list .nat)], result := (.list .nat),
      body := (.«match» (.list .nat) (.var 0) [(.arm .nil [] (.var 1)), (.arm .cons [2, 3] (.call 13 [(.var 3), (.call 14 [(.second (.var 2)), (.call 15 [(.var 1), (.first (.var 2))])])]))]) },
    { parameters := [0, 1], types := [(.list .nat), (.list .nat)], result := (.list .nat),
      body := (.«match» (.list .nat) (.var 0) [(.arm .nil [] (.var 1)), (.arm .cons [2, 3] (.call 14 [(.var 3), (.call 15 [(.var 1), (.var 2)])]))]) },
    { parameters := [0, 1], types := [(.list .nat), .nat], result := (.list .nat),
      body := (.«match» (.list .nat) (.var 0) [(.arm .nil [] (.build .cons (.list .nat) [(.var 1), (.build .nil (.list .nat) [])])), (.arm .cons [2, 3] (.«match» (.list .nat) (.prim .compare [(.var 1), (.var 2)]) [(.arm .lt [] (.build .cons (.list .nat) [(.var 1), (.var 0)])), (.arm .eq [] (.var 0)), (.arm .gt [] (.build .cons (.list .nat) [(.var 2), (.call 15 [(.var 3), (.var 1)])]))]))]) },
    { parameters := [0], types := [(.list (.pair .nat (.list .nat)))], result := (.option (.list .nat)),
      body := (.«let» 1 (.list .nat) (.call 21 [(.var 0), (.build .nil (.list .nat) [])]) (.call 17 [(.var 0), (.prim .natAdd [(.prim .length [(.var 1)]), (.value .nat (.nat 1))]), (.var 1), (.build .nil (.list .nat) [])])) },
    { parameters := [0, 1, 2, 3], types := [(.list (.pair .nat (.list .nat))), .nat, (.list .nat), (.list .nat)], result := (.option (.list .nat)),
      body := (.«match» (.option (.list .nat)) (.var 1) [(.arm .zero [] (.«match» (.option (.list .nat)) (.var 2) [(.arm .nil [] (.build .some (.option (.list .nat)) [(.call 20 [(.var 3), (.build .nil (.list .nat) [])])])), (.arm .cons [4, 5] (.build .none (.option (.list .nat)) []))])), (.arm .succ [6] (.«match» (.option (.list .nat)) (.call 18 [(.var 0), (.var 2), (.var 2)]) [(.arm .nil [] (.«match» (.option (.list .nat)) (.var 2) [(.arm .nil [] (.build .some (.option (.list .nat)) [(.call 20 [(.var 3), (.build .nil (.list .nat) [])])])), (.arm .cons [7, 8] (.build .none (.option (.list .nat)) []))])), (.arm .cons [9, 10] (.call 17 [(.var 0), (.var 6), (.call 27 [(.var 2), (.var 9)]), (.build .cons (.list .nat) [(.var 9), (.var 3)])]))]))]) },
    { parameters := [0, 1, 2], types := [(.list (.pair .nat (.list .nat))), (.list .nat), (.list .nat)], result := (.list .nat),
      body := (.«match» (.list .nat) (.var 2) [(.arm .nil [] (.build .nil (.list .nat) [])), (.arm .cons [3, 4] (.cond (.call 19 [(.var 0), (.var 3), (.var 1)]) (.build .cons (.list .nat) [(.var 3), (.call 18 [(.var 0), (.var 1), (.var 4)])]) (.call 18 [(.var 0), (.var 1), (.var 4)])))]) },
    { parameters := [0, 1, 2], types := [(.list (.pair .nat (.list .nat))), .nat, (.list .nat)], result := .bool,
      body := (.«match» .bool (.var 2) [(.arm .nil [] (.build .true .bool [])), (.arm .cons [3, 4] (.cond (.call 26 [(.call 24 [(.var 0), (.var 3)]), (.var 1)]) (.build .false .bool []) (.call 19 [(.var 0), (.var 1), (.var 4)])))]) },
    { parameters := [0, 1], types := [(.list .nat), (.list .nat)], result := (.list .nat),
      body := (.«match» (.list .nat) (.var 0) [(.arm .nil [] (.var 1)), (.arm .cons [2, 3] (.call 20 [(.var 3), (.build .cons (.list .nat) [(.var 2), (.var 1)])]))]) },
    { parameters := [0, 1], types := [(.list (.pair .nat (.list .nat))), (.list .nat)], result := (.list .nat),
      body := (.«match» (.list .nat) (.var 0) [(.arm .nil [] (.var 1)), (.arm .cons [2, 3] (.call 21 [(.var 3), (.call 22 [(.second (.var 2)), (.call 23 [(.var 1), (.first (.var 2))])])]))]) },
    { parameters := [0, 1], types := [(.list .nat), (.list .nat)], result := (.list .nat),
      body := (.«match» (.list .nat) (.var 0) [(.arm .nil [] (.var 1)), (.arm .cons [2, 3] (.call 22 [(.var 3), (.call 23 [(.var 1), (.var 2)])]))]) },
    { parameters := [0, 1], types := [(.list .nat), .nat], result := (.list .nat),
      body := (.«match» (.list .nat) (.var 0) [(.arm .nil [] (.build .cons (.list .nat) [(.var 1), (.build .nil (.list .nat) [])])), (.arm .cons [2, 3] (.«match» (.list .nat) (.prim .compare [(.var 1), (.var 2)]) [(.arm .lt [] (.build .cons (.list .nat) [(.var 1), (.var 0)])), (.arm .eq [] (.var 0)), (.arm .gt [] (.build .cons (.list .nat) [(.var 2), (.call 23 [(.var 3), (.var 1)])]))]))]) },
    { parameters := [0, 1], types := [(.list (.pair .nat (.list .nat))), .nat], result := (.list .nat),
      body := (.«match» (.list .nat) (.call 25 [(.var 0), (.var 1)]) [(.arm .none [] (.build .nil (.list .nat) [])), (.arm .some [2] (.var 2))]) },
    { parameters := [0, 1], types := [(.list (.pair .nat (.list .nat))), .nat], result := (.option (.list .nat)),
      body := (.«match» (.option (.list .nat)) (.var 0) [(.arm .nil [] (.build .none (.option (.list .nat)) [])), (.arm .cons [2, 3] (.«match» (.option (.list .nat)) (.prim .compare [(.var 1), (.first (.var 2))]) [(.arm .lt [] (.build .none (.option (.list .nat)) [])), (.arm .eq [] (.build .some (.option (.list .nat)) [(.second (.var 2))])), (.arm .gt [] (.call 25 [(.var 3), (.var 1)]))]))]) },
    { parameters := [0, 1], types := [(.list .nat), .nat], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .nil [] (.build .false .bool [])), (.arm .cons [2, 3] (.«match» .bool (.prim .compare [(.var 1), (.var 2)]) [(.arm .lt [] (.build .false .bool [])), (.arm .eq [] (.build .true .bool [])), (.arm .gt [] (.call 26 [(.var 3), (.var 1)]))]))]) },
    { parameters := [0, 1], types := [(.list .nat), .nat], result := (.list .nat),
      body := (.«match» (.list .nat) (.var 0) [(.arm .nil [] (.build .nil (.list .nat) [])), (.arm .cons [2, 3] (.«match» (.list .nat) (.prim .compare [(.var 1), (.var 2)]) [(.arm .lt [] (.var 0)), (.arm .eq [] (.var 3)), (.arm .gt [] (.build .cons (.list .nat) [(.var 2), (.call 27 [(.var 3), (.var 1)])]))]))]) },
    { parameters := [0], types := [(.list (.pair .nat (.list .nat)))], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .nil [] (.build .true .bool [])), (.arm .cons [1, 2] (.cond (.call 29 [(.second (.var 1))]) (.«match» .bool (.var 2) [(.arm .nil [] (.build .true .bool [])), (.arm .cons [3, 4] (.cond (.«match» .bool (.prim .compare [(.first (.var 1)), (.first (.var 3))]) [(.arm .lt [] (.build .true .bool [])), (.arm .eq [] (.build .false .bool [])), (.arm .gt [] (.build .false .bool []))]) (.call 28 [(.var 2)]) (.build .false .bool [])))]) (.build .false .bool [])))]) },
    { parameters := [0], types := [(.list .nat)], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .nil [] (.build .true .bool [])), (.arm .cons [1, 2] (.«match» .bool (.var 2) [(.arm .nil [] (.build .true .bool [])), (.arm .cons [3, 4] (.cond (.«match» .bool (.prim .compare [(.var 1), (.var 3)]) [(.arm .lt [] (.build .true .bool [])), (.arm .eq [] (.build .false .bool [])), (.arm .gt [] (.build .false .bool []))]) (.call 29 [(.var 2)]) (.build .false .bool [])))]))]) },
    { parameters := [0, 1], types := [(.list (.pair .nat (.list .nat))), .nat], result := (.option (.pair (.list .nat) (.pair (.list .nat) (.option (.list .nat))))),
      body := (.cond (.call 28 [(.var 0)]) (.build .some (.option (.pair (.list .nat) (.pair (.list .nat) (.option (.list .nat))))) [(.call 0 [(.var 0), (.var 1)])]) (.build .none (.option (.pair (.list .nat) (.pair (.list .nat) (.option (.list .nat))))) [])) }] }

def __fits_0 (g : (List (Prod Nat (List Nat)))) (start : Nat) : Bool :=
  ((((true && (true && true)) && true) && (((((true && (true && true)) && (LexLeanPreservation.graphFits Coverage.Colls.LexLeanCollections.insertElement (g))) && (((true && true) && (LexLeanPreservation.graphFits Coverage.Colls.LexLeanCollections.insertElement (g))) && true)) && true) && true)) && true)

theorem __rel_0 (g : (List (Prod Nat (List Nat)))) (start : Nat) : LexLeanPreservation.FunRel __prog 0 [((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat))) g), (LexLeanTarget.TargetSyntax.Value.nat start)] (LexLeanPreservation.Rel (__fits_0 g start) ((LexLeanPreservation.encPair (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) (LexLeanPreservation.encPair (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) (LexLeanPreservation.encOption (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat)))) (Coverage.Colls.graphs g start))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.tpl_graphSuccessors (LexLeanPreservation.keySpec_nat (Coverage.Colls.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) (fun _ _ => rfl)) (LexLeanPreservation.listEnc LexLeanTarget.TargetSyntax.Value.nat) (LexLeanPreservation.listEnc (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat))) (fun _ => rfl) Coverage.Colls.LexLeanCollections.lookupEntry (fun _ => rfl) (fun _ _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.lookupEntry, h]) (fun _ _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.lookupEntry, h]) (fun _ _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.lookupEntry, h]) (p := __prog) (fi := 1) (tk := .nat) rfl rfl (g) (start))) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.tpl_graphReachable (LexLeanPreservation.keySpec_nat (Coverage.Colls.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) (fun _ _ => rfl)) (LexLeanPreservation.listEnc LexLeanTarget.TargetSyntax.Value.nat) (fun _ => rfl) (LexLeanPreservation.listEnc (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat))) (fun _ => rfl) (⟨(fun _ => rfl), (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.insertElement, h]), (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.insertElement, h]), (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.insertElement, h]), (fun _ => rfl), (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.removeElement, h]), (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.removeElement, h]), (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.removeElement, h]), (fun _ => rfl), (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.containsElement, h]), (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.containsElement, h]), (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.containsElement, h])⟩ : LexLeanPreservation.SetOps (Coverage.Colls.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) Coverage.Colls.LexLeanCollections.insertElement Coverage.Colls.LexLeanCollections.removeElement Coverage.Colls.LexLeanCollections.containsElement) (⟨(fun _ => rfl), (fun _ _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.lookupEntry, h]), (fun _ _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.lookupEntry, h]), (fun _ _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.lookupEntry, h])⟩ : LexLeanPreservation.LookOps (Coverage.Colls.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) (Coverage.Colls.LexLeanCollections.lookupEntry : Nat -> List (Prod Nat (List Nat)) -> Option (List Nat))) (g) (Coverage.Colls.LexLeanCollections.graphSuccessors (g)) (fun _ => rfl) (Coverage.Colls.LexLeanCollections.reachableFrom (g)) (fun _ _ => rfl) (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.reachableFrom, h]) (fun _ _ _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.reachableFrom, h]) (p := __prog) (fi := 3) (tk := .nat) rfl rfl rfl rfl rfl rfl rfl rfl rfl rfl rfl rfl rfl (start))) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (LexLeanPreservation.tpl_graphTopological (LexLeanPreservation.keySpec_nat (Coverage.Colls.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) (fun _ _ => rfl)) (LexLeanPreservation.listEnc LexLeanTarget.TargetSyntax.Value.nat) (fun _ => rfl) (LexLeanPreservation.listEnc (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat))) (fun _ => rfl) (LexLeanPreservation.optEnc (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat)) (fun _ => rfl) (⟨(fun _ => rfl), (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.insertElement, h]), (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.insertElement, h]), (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.insertElement, h]), (fun _ => rfl), (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.removeElement, h]), (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.removeElement, h]), (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.removeElement, h]), (fun _ => rfl), (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.containsElement, h]), (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.containsElement, h]), (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.containsElement, h])⟩ : LexLeanPreservation.SetOps (Coverage.Colls.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) Coverage.Colls.LexLeanCollections.insertElement Coverage.Colls.LexLeanCollections.removeElement Coverage.Colls.LexLeanCollections.containsElement) (⟨(fun _ => rfl), (fun _ _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.lookupEntry, h]), (fun _ _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.lookupEntry, h]), (fun _ _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.lookupEntry, h])⟩ : LexLeanPreservation.LookOps (Coverage.Colls.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) (Coverage.Colls.LexLeanCollections.lookupEntry : Nat -> List (Prod Nat (List Nat)) -> Option (List Nat))) (g) (Coverage.Colls.LexLeanCollections.graphSuccessors (g)) (fun _ => rfl) (⟨(fun _ => rfl), (fun _ _ _ => rfl), (fun _ _ => rfl), (fun _ _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.topological, h] <;> rfl), (fun _ _ _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.topological, h])⟩ : LexLeanPreservation.TopoOps (Coverage.Colls.LexLeanCollections.graphSuccessors (g)) Coverage.Colls.LexLeanCollections.containsElement Coverage.Colls.LexLeanCollections.removeElement (Coverage.Colls.LexLeanCollections.topological (g))) (p := __prog) (fi := 16) (tk := .nat) rfl rfl rfl rfl rfl rfl rfl rfl rfl rfl rfl rfl)) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair)

def denote (g : (List (Prod Nat (List Nat)))) (start : Nat) : LexLeanPreservation.Obs :=
  cond (__fits_0 g start) (LexLeanPreservation.Obs.value (((LexLeanPreservation.encPair (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) (LexLeanPreservation.encPair (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) (LexLeanPreservation.encOption (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat)))) (Coverage.Colls.graphs g start)))) LexLeanPreservation.Obs.overflow

theorem root (g : (List (Prod Nat (List Nat)))) (start : Nat) : LexLeanPreservation.RunConv __prog 0 [((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat))) g), (LexLeanTarget.TargetSyntax.Value.nat start)] (LexLeanPreservation.Rel (__fits_0 g start) ((LexLeanPreservation.encPair (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) (LexLeanPreservation.encPair (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) (LexLeanPreservation.encOption (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat)))) (Coverage.Colls.graphs g start))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 g start)

def __valid_29 : (List Nat) -> Bool :=
  LexLeanPreservation.ascSet (Coverage.Colls.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering)

def __inv_29 : (List Nat) -> Prop :=
  LexLeanPreservation.InvSet (Coverage.Colls.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering)

theorem __viff_29 : ∀ (__v : (List Nat)), __valid_29 __v = true ↔ __inv_29 __v :=
  LexLeanPreservation.ascSet_inv (Coverage.Colls.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering)

theorem __vrel_29 : ∀ (__v : (List Nat)), LexLeanPreservation.FunRel __prog 29 [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) __v)] (LexLeanPreservation.Rel true (LexLeanTarget.TargetSyntax.Value.bool (__valid_29 __v))) :=
  LexLeanPreservation.tpl_validSet (LexLeanPreservation.keySpec_nat (Coverage.Colls.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) (fun _ _ => rfl)) (LexLeanPreservation.listEnc LexLeanTarget.TargetSyntax.Value.nat) (fun _ => rfl) rfl

def __valid_28 : (List (Prod Nat (List Nat))) -> Bool :=
  LexLeanPreservation.ascMap (Coverage.Colls.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) __valid_29

def __inv_28 : (List (Prod Nat (List Nat))) -> Prop :=
  LexLeanPreservation.InvMap (Coverage.Colls.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) __inv_29

theorem __viff_28 : ∀ (__v : (List (Prod Nat (List Nat)))), __valid_28 __v = true ↔ __inv_28 __v :=
  LexLeanPreservation.ascMap_inv (Coverage.Colls.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) __valid_29 __inv_29 __viff_29

theorem __vrel_28 : ∀ (__v : (List (Prod Nat (List Nat)))), LexLeanPreservation.FunRel __prog 28 [((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat))) __v)] (LexLeanPreservation.Rel true (LexLeanTarget.TargetSyntax.Value.bool (__valid_28 __v))) :=
  LexLeanPreservation.tpl_validMap (LexLeanPreservation.keySpec_nat (Coverage.Colls.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) (fun _ _ => rfl)) __valid_29 (LexLeanPreservation.listEnc (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat))) (fun _ => rfl) __vrel_29 rfl

def entryFits (g : (List (Prod Nat (List Nat)))) (start : Nat) : Bool :=
  (if __valid_28 g then __fits_0 g start else true)

def entryValue (g : (List (Prod Nat (List Nat)))) (start : Nat) : LexLeanTarget.TargetSyntax.Value :=
  (if __valid_28 g then (LexLeanTarget.TargetSyntax.Value.some ((LexLeanPreservation.encPair (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) (LexLeanPreservation.encPair (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) (LexLeanPreservation.encOption (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat)))) (Coverage.Colls.graphs g start))) else LexLeanTarget.TargetSyntax.Value.none)

/-- The entry's observation. -/
def denoteEntry (g : (List (Prod Nat (List Nat)))) (start : Nat) : LexLeanPreservation.Obs :=
  cond (entryFits g start) (LexLeanPreservation.Obs.value (entryValue g start)) LexLeanPreservation.Obs.overflow

/-- §17.12's invariants of the validated parameters. -/
def accepts (g : (List (Prod Nat (List Nat)))) (start : Nat) : Prop :=
  (__inv_28 g ∧ True)

theorem entry (g : (List (Prod Nat (List Nat)))) (start : Nat) : LexLeanPreservation.RunConv __prog 30 [((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat))) g), (LexLeanTarget.TargetSyntax.Value.nat start)] (denoteEntry g start) :=
  LexLeanPreservation.run_of_funRel (LexLeanPreservation.FunRel.fits_eq (LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_cond (fun __c => if __c then __fits_0 g start else true) (fun __c => if __c then (LexLeanTarget.TargetSyntax.Value.some ((LexLeanPreservation.encPair (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) (LexLeanPreservation.encPair (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) (LexLeanPreservation.encOption (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat)))) (Coverage.Colls.graphs g start))) else LexLeanTarget.TargetSyntax.Value.none) (__valid_28 g) (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__vrel_28 g)) (fun _ => LexLeanPreservation.Conv.fits_eq (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (__rel_0 g start)) LexLeanPreservation.convL_nil) LexLeanPreservation.construct_some) (by simp)) (fun _ => LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_none))) (by simp [entryFits]))

theorem entry_accepts (g : (List (Prod Nat (List Nat)))) (start : Nat) (__h : accepts g start) : denoteEntry g start = LexLeanPreservation.someObs (denote g start) := by
  unfold accepts at __h
  unfold denoteEntry entryFits entryValue denote
  simp only [if_true, (__viff_28 g).mpr __h.1]
  cases __fits_0 g start <;> rfl

theorem entry_refuses (g : (List (Prod Nat (List Nat)))) (start : Nat) (__h : ¬ accepts g start) : denoteEntry g start = LexLeanPreservation.Obs.value LexLeanTarget.TargetSyntax.Value.none := by
  unfold accepts at __h
  unfold denoteEntry entryFits entryValue
  cases __c0 : __valid_28 g
  · simp
  · exact absurd ⟨(__viff_28 g).mp __c0, trivial⟩ __h

end LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R6
