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
namespace LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R0

def __prog : LexLeanTarget.TargetSyntax.Program :=
  { adts := [{ constructors := [[(.list .nat), (.list (.adt 0)), (.list (.pair .string .int)), (.option (.adt 0)), (.list (.pair .nat (.adt 0)))], [(.list (.pair .nat .int))], [(.result (.adt 0) .nat)]] },
    { constructors := [[(.list (.pair .string .int)), .string]] }], functions := [
    { parameters := [0, 1, 2, 3], types := [(.adt 0), (.adt 1), (.option (.list .nat)), (.list (.list (.pair .nat .bool)))], result := (.pair .nat (.pair (.adt 1) (.pair (.option (.list .nat)) (.list (.list (.pair .nat .bool)))))),
      body := (.build .pair (.pair .nat (.pair (.adt 1) (.pair (.option (.list .nat)) (.list (.list (.pair .nat .bool)))))) [(.call 1 [(.var 0)]), (.build .pair (.pair (.adt 1) (.pair (.option (.list .nat)) (.list (.list (.pair .nat .bool))))) [(.var 1), (.build .pair (.pair (.option (.list .nat)) (.list (.list (.pair .nat .bool)))) [(.var 2), (.var 3)])])]) },
    { parameters := [0], types := [(.adt 0)], result := .nat,
      body := (.«match» .nat (.var 0) [(.arm (.adt 0) [1, 2, 3, 4, 5] (.prim .length [(.var 1)])), (.arm (.adt 1) [6] (.prim .length [(.var 6)])), (.arm (.adt 2) [7] (.value .nat (.nat 0)))]) },
    { parameters := [0], types := [(.adt 0)], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm (.adt 0) [1, 2, 3, 4, 5] (.cond (.call 3 [(.var 1)]) (.cond (.call 4 [(.var 2)]) (.cond (.call 5 [(.var 3)]) (.cond (.call 7 [(.var 4)]) (.cond (.call 8 [(.var 5)]) (.build .true .bool []) (.build .false .bool [])) (.build .false .bool [])) (.build .false .bool [])) (.build .false .bool [])) (.build .false .bool []))), (.arm (.adt 1) [6] (.cond (.call 11 [(.var 6)]) (.build .true .bool []) (.build .false .bool []))), (.arm (.adt 2) [7] (.cond (.call 12 [(.var 7)]) (.build .true .bool []) (.build .false .bool [])))]) },
    { parameters := [0], types := [(.list .nat)], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .nil [] (.build .true .bool [])), (.arm .cons [1, 2] (.«match» .bool (.var 2) [(.arm .nil [] (.build .true .bool [])), (.arm .cons [3, 4] (.cond (.«match» .bool (.prim .compare [(.var 1), (.var 3)]) [(.arm .lt [] (.build .true .bool [])), (.arm .eq [] (.build .false .bool [])), (.arm .gt [] (.build .false .bool []))]) (.call 3 [(.var 2)]) (.build .false .bool [])))]))]) },
    { parameters := [0], types := [(.list (.adt 0))], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .nil [] (.build .true .bool [])), (.arm .cons [1, 2] (.cond (.call 2 [(.var 1)]) (.call 4 [(.var 2)]) (.build .false .bool [])))]) },
    { parameters := [0], types := [(.list (.pair .string .int))], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .nil [] (.build .true .bool [])), (.arm .cons [1, 2] (.cond (.call 6 [(.second (.var 1))]) (.«match» .bool (.var 2) [(.arm .nil [] (.build .true .bool [])), (.arm .cons [3, 4] (.cond (.«match» .bool (.prim .compare [(.first (.var 1)), (.first (.var 3))]) [(.arm .lt [] (.build .true .bool [])), (.arm .eq [] (.build .false .bool [])), (.arm .gt [] (.build .false .bool []))]) (.call 5 [(.var 2)]) (.build .false .bool [])))]) (.build .false .bool [])))]) },
    { parameters := [0], types := [.int], result := .bool,
      body := (.build .true .bool []) },
    { parameters := [0], types := [(.option (.adt 0))], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .none [] (.build .true .bool [])), (.arm .some [1] (.call 2 [(.var 1)]))]) },
    { parameters := [0], types := [(.list (.pair .nat (.adt 0)))], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .nil [] (.build .true .bool [])), (.arm .cons [1, 2] (.cond (.call 9 [(.var 1)]) (.call 8 [(.var 2)]) (.build .false .bool [])))]) },
    { parameters := [0], types := [(.pair .nat (.adt 0))], result := .bool,
      body := (.cond (.call 10 [(.first (.var 0))]) (.call 2 [(.second (.var 0))]) (.build .false .bool [])) },
    { parameters := [0], types := [.nat], result := .bool,
      body := (.build .true .bool []) },
    { parameters := [0], types := [(.list (.pair .nat .int))], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .nil [] (.build .true .bool [])), (.arm .cons [1, 2] (.cond (.call 6 [(.second (.var 1))]) (.«match» .bool (.var 2) [(.arm .nil [] (.build .true .bool [])), (.arm .cons [3, 4] (.cond (.«match» .bool (.prim .compare [(.first (.var 1)), (.first (.var 3))]) [(.arm .lt [] (.build .true .bool [])), (.arm .eq [] (.build .false .bool [])), (.arm .gt [] (.build .false .bool []))]) (.call 11 [(.var 2)]) (.build .false .bool [])))]) (.build .false .bool [])))]) },
    { parameters := [0], types := [(.result (.adt 0) .nat)], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .ok [1] (.call 2 [(.var 1)])), (.arm .error [2] (.call 10 [(.var 2)]))]) },
    { parameters := [0], types := [(.adt 1)], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm (.adt 0) [1, 2] (.cond (.call 5 [(.var 1)]) (.build .true .bool []) (.build .false .bool [])))]) },
    { parameters := [0], types := [(.option (.list .nat))], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .none [] (.build .true .bool [])), (.arm .some [1] (.call 3 [(.var 1)]))]) },
    { parameters := [0], types := [(.list (.list (.pair .nat .bool)))], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .nil [] (.build .true .bool [])), (.arm .cons [1, 2] (.cond (.call 16 [(.var 1)]) (.call 15 [(.var 2)]) (.build .false .bool [])))]) },
    { parameters := [0], types := [(.list (.pair .nat .bool))], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .nil [] (.build .true .bool [])), (.arm .cons [1, 2] (.cond (.call 17 [(.second (.var 1))]) (.«match» .bool (.var 2) [(.arm .nil [] (.build .true .bool [])), (.arm .cons [3, 4] (.cond (.«match» .bool (.prim .compare [(.first (.var 1)), (.first (.var 3))]) [(.arm .lt [] (.build .true .bool [])), (.arm .eq [] (.build .false .bool [])), (.arm .gt [] (.build .false .bool []))]) (.call 16 [(.var 2)]) (.build .false .bool [])))]) (.build .false .bool [])))]) },
    { parameters := [0], types := [.bool], result := .bool,
      body := (.build .true .bool []) },
    { parameters := [0, 1, 2, 3], types := [(.adt 0), (.adt 1), (.option (.list .nat)), (.list (.list (.pair .nat .bool)))], result := (.option (.pair .nat (.pair (.adt 1) (.pair (.option (.list .nat)) (.list (.list (.pair .nat .bool))))))),
      body := (.cond (.call 2 [(.var 0)]) (.cond (.call 13 [(.var 1)]) (.cond (.call 14 [(.var 2)]) (.cond (.call 15 [(.var 3)]) (.build .some (.option (.pair .nat (.pair (.adt 1) (.pair (.option (.list .nat)) (.list (.list (.pair .nat .bool))))))) [(.call 0 [(.var 0), (.var 1), (.var 2), (.var 3)])]) (.build .none (.option (.pair .nat (.pair (.adt 1) (.pair (.option (.list .nat)) (.list (.list (.pair .nat .bool))))))) [])) (.build .none (.option (.pair .nat (.pair (.adt 1) (.pair (.option (.list .nat)) (.list (.list (.pair .nat .bool))))))) [])) (.build .none (.option (.pair .nat (.pair (.adt 1) (.pair (.option (.list .nat)) (.list (.list (.pair .nat .bool))))))) [])) (.build .none (.option (.pair .nat (.pair (.adt 1) (.pair (.option (.list .nat)) (.list (.list (.pair .nat .bool))))))) [])) }] }

mutual
def __enc_0 : (Coverage.Boundary.Grove) -> LexLeanTarget.TargetSyntax.Value
  | Coverage.Boundary.Grove.node __x0 __x1 __x2 __x3 __x4 => LexLeanTarget.TargetSyntax.Value.adt 0 [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) __x0), (LexLeanTarget.TargetSyntax.Value.list (__items_0 __x1)), ((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.string LexLeanTarget.TargetSyntax.Value.int)) __x2), (__aux_1 __x3), (LexLeanTarget.TargetSyntax.Value.list (__items_3 __x4))]
  | Coverage.Boundary.Grove.leaf __x0 => LexLeanTarget.TargetSyntax.Value.adt 1 [((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.int)) __x0)]
  | Coverage.Boundary.Grove.settled __x0 => LexLeanTarget.TargetSyntax.Value.adt 2 [(__aux_4 __x0)]

def __items_0 : (List (Coverage.Boundary.Grove)) -> List LexLeanTarget.TargetSyntax.Value
  | [] => []
  | __x0 :: __x1 => (__enc_0 __x0) :: __items_0 __x1

def __aux_1 : (Option (Coverage.Boundary.Grove)) -> LexLeanTarget.TargetSyntax.Value
  | none => LexLeanTarget.TargetSyntax.Value.none
  | some __x0 => LexLeanTarget.TargetSyntax.Value.some (__enc_0 __x0)

def __aux_2 : (Prod Nat (Coverage.Boundary.Grove)) -> LexLeanTarget.TargetSyntax.Value
  | (__x0, __x1) => LexLeanTarget.TargetSyntax.Value.pair (LexLeanTarget.TargetSyntax.Value.nat __x0) (__enc_0 __x1)

def __items_3 : (List (Prod Nat (Coverage.Boundary.Grove))) -> List LexLeanTarget.TargetSyntax.Value
  | [] => []
  | __x0 :: __x1 => (__aux_2 __x0) :: __items_3 __x1

def __aux_4 : (Except Nat (Coverage.Boundary.Grove)) -> LexLeanTarget.TargetSyntax.Value
  | Except.error __x0 => LexLeanTarget.TargetSyntax.Value.error (LexLeanTarget.TargetSyntax.Value.nat __x0)
  | Except.ok __x0 => LexLeanTarget.TargetSyntax.Value.ok (__enc_0 __x0)

end

def __L_0 : LexLeanPreservation.ListEnc (Coverage.Boundary.Grove) :=
  ⟨__enc_0, __items_0, __items_0.eq_1, __items_0.eq_2⟩

def __O_1 : LexLeanPreservation.OptEnc (Coverage.Boundary.Grove) :=
  ⟨__enc_0, __aux_1, __aux_1.eq_1, __aux_1.eq_2⟩

def __L_3 : LexLeanPreservation.ListEnc (Prod Nat (Coverage.Boundary.Grove)) :=
  ⟨__aux_2, __items_3, __items_3.eq_1, __items_3.eq_2⟩

def __enc_1 (__s : (Coverage.Boundary.Ledger)) : LexLeanTarget.TargetSyntax.Value :=
  LexLeanTarget.TargetSyntax.Value.adt 0 [((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.string LexLeanTarget.TargetSyntax.Value.int)) (__s).entries), (LexLeanTarget.TargetSyntax.Value.string (__s).owner)]

def __fits_1 (g : (Coverage.Boundary.Grove)) : Bool :=
  (true && (match g with | Coverage.Boundary.Grove.node tags children labels spare paired => ((true && true) && (Nat.blt (Coverage.Boundary.LexLeanCollections.setSize (tags) : Nat) 18446744073709551616)) | Coverage.Boundary.Grove.leaf weights => ((true && true) && (Nat.blt (Coverage.Boundary.LexLeanCollections.mapSize (weights) : Nat) 18446744073709551616)) | Coverage.Boundary.Grove.settled outcome => true))

theorem __rel_1 (g : (Coverage.Boundary.Grove)) : LexLeanPreservation.FunRel __prog 1 [(__enc_0 g)] (LexLeanPreservation.Rel (__fits_1 g) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Boundary.groveTags g))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_matchV __enc_0 (fun (__a : (Coverage.Boundary.Grove)) => (match __a with | Coverage.Boundary.Grove.node tags children labels spare paired => ((true && true) && (Nat.blt (Coverage.Boundary.LexLeanCollections.setSize (tags) : Nat) 18446744073709551616)) | Coverage.Boundary.Grove.leaf weights => ((true && true) && (Nat.blt (Coverage.Boundary.LexLeanCollections.mapSize (weights) : Nat) 18446744073709551616)) | Coverage.Boundary.Grove.settled outcome => true)) (fun (__a : (Coverage.Boundary.Grove)) => LexLeanTarget.TargetSyntax.Value.nat (match __a with | Coverage.Boundary.Grove.node tags children labels spare paired => (Coverage.Boundary.LexLeanCollections.setSize (tags) : Nat) | Coverage.Boundary.Grove.leaf weights => (Coverage.Boundary.LexLeanCollections.mapSize (weights) : Nat) | Coverage.Boundary.Grove.settled outcome => (0 : Nat))) g (LexLeanPreservation.conv_var rfl) (fun (__a : (Coverage.Boundary.Grove)) _ => match __a with | Coverage.Boundary.Grove.node tags children labels spare paired => (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (LexLeanPreservation.prim_length_list (LexLeanPreservation.listEnc LexLeanTarget.TargetSyntax.Value.nat) tags))) | Coverage.Boundary.Grove.leaf weights => (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (LexLeanPreservation.prim_length_list (LexLeanPreservation.listEnc (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.int)) weights)))) | Coverage.Boundary.Grove.settled outcome => (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_hit rfl rfl LexLeanPreservation.conv_value)))))

def __fits_0 (g : (Coverage.Boundary.Grove)) (ledger : (Coverage.Boundary.Ledger)) (picks : (Option (List Nat))) (spare : (List (List (Prod Nat Bool)))) : Bool :=
  ((((true && true) && (__fits_1 (g))) && (((true && (((true && (true && true)) && true) && true)) && true) && true)) && true)

attribute [local irreducible] Coverage.Boundary.groveTags in
theorem __rel_0 (g : (Coverage.Boundary.Grove)) (ledger : (Coverage.Boundary.Ledger)) (picks : (Option (List Nat))) (spare : (List (List (Prod Nat Bool)))) : LexLeanPreservation.FunRel __prog 0 [(__enc_0 g), (__enc_1 ledger), ((LexLeanPreservation.encOption (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat)) picks), ((LexLeanPreservation.encList (LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.bool))) spare)] (LexLeanPreservation.Rel (__fits_0 g ledger picks spare) ((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat (LexLeanPreservation.encPair __enc_1 (LexLeanPreservation.encPair (LexLeanPreservation.encOption (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat)) (LexLeanPreservation.encList (LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.bool)))))) (Coverage.Boundary.groveRoot g ledger picks spare))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_1 (g))) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair)

def denote (g : (Coverage.Boundary.Grove)) (ledger : (Coverage.Boundary.Ledger)) (picks : (Option (List Nat))) (spare : (List (List (Prod Nat Bool)))) : LexLeanPreservation.Obs :=
  cond (__fits_0 g ledger picks spare) (LexLeanPreservation.Obs.value (((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat (LexLeanPreservation.encPair __enc_1 (LexLeanPreservation.encPair (LexLeanPreservation.encOption (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat)) (LexLeanPreservation.encList (LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.bool)))))) (Coverage.Boundary.groveRoot g ledger picks spare)))) LexLeanPreservation.Obs.overflow

theorem root (g : (Coverage.Boundary.Grove)) (ledger : (Coverage.Boundary.Ledger)) (picks : (Option (List Nat))) (spare : (List (List (Prod Nat Bool)))) : LexLeanPreservation.RunConv __prog 0 [(__enc_0 g), (__enc_1 ledger), ((LexLeanPreservation.encOption (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat)) picks), ((LexLeanPreservation.encList (LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.bool))) spare)] (LexLeanPreservation.Rel (__fits_0 g ledger picks spare) ((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat (LexLeanPreservation.encPair __enc_1 (LexLeanPreservation.encPair (LexLeanPreservation.encOption (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat)) (LexLeanPreservation.encList (LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.bool)))))) (Coverage.Boundary.groveRoot g ledger picks spare))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 g ledger picks spare)

def __valid_3 : (List Nat) -> Bool :=
  LexLeanPreservation.ascSet (Coverage.Boundary.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering)

def __inv_3 : (List Nat) -> Prop :=
  LexLeanPreservation.InvSet (Coverage.Boundary.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering)

theorem __viff_3 : ∀ (__v : (List Nat)), __valid_3 __v = true ↔ __inv_3 __v :=
  LexLeanPreservation.ascSet_inv (Coverage.Boundary.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering)

theorem __vrel_3 : ∀ (__v : (List Nat)), LexLeanPreservation.FunRel __prog 3 [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) __v)] (LexLeanPreservation.Rel true (LexLeanTarget.TargetSyntax.Value.bool (__valid_3 __v))) :=
  LexLeanPreservation.tpl_validSet (LexLeanPreservation.keySpec_nat (Coverage.Boundary.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) (fun _ _ => rfl)) (LexLeanPreservation.listEnc LexLeanTarget.TargetSyntax.Value.nat) (fun _ => rfl) rfl

def __valid_6 : Int -> Bool :=
  fun _ => true

def __inv_6 : Int -> Prop :=
  fun _ => True

theorem __viff_6 : ∀ (__v : Int), __valid_6 __v = true ↔ __inv_6 __v :=
  fun __v => LexLeanPreservation.validTrue_inv __v

theorem __vrel_6 : ∀ (__v : Int), LexLeanPreservation.FunRel __prog 6 [(LexLeanTarget.TargetSyntax.Value.int __v)] (LexLeanPreservation.Rel true (LexLeanTarget.TargetSyntax.Value.bool (__valid_6 __v))) :=
  LexLeanPreservation.tpl_validTrue LexLeanTarget.TargetSyntax.Value.int rfl

def __valid_5 : (List (Prod String Int)) -> Bool :=
  LexLeanPreservation.ascMap (Coverage.Boundary.LexLeanCollections.Key.compare : String -> String -> Ordering) __valid_6

def __inv_5 : (List (Prod String Int)) -> Prop :=
  LexLeanPreservation.InvMap (Coverage.Boundary.LexLeanCollections.Key.compare : String -> String -> Ordering) __inv_6

theorem __viff_5 : ∀ (__v : (List (Prod String Int))), __valid_5 __v = true ↔ __inv_5 __v :=
  LexLeanPreservation.ascMap_inv (Coverage.Boundary.LexLeanCollections.Key.compare : String -> String -> Ordering) __valid_6 __inv_6 __viff_6

theorem __vrel_5 : ∀ (__v : (List (Prod String Int))), LexLeanPreservation.FunRel __prog 5 [((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.string LexLeanTarget.TargetSyntax.Value.int)) __v)] (LexLeanPreservation.Rel true (LexLeanTarget.TargetSyntax.Value.bool (__valid_5 __v))) :=
  LexLeanPreservation.tpl_validMap (LexLeanPreservation.keySpec_string Coverage.Boundary.LexLeanCollections.compareCodes rfl (fun _ _ => rfl) (fun _ _ => rfl) (fun _ _ _ _ h => by simp only [Coverage.Boundary.LexLeanCollections.compareCodes, h]) (fun _ _ _ _ h => by simp only [Coverage.Boundary.LexLeanCollections.compareCodes, h]) (fun _ _ _ _ h => by simp only [Coverage.Boundary.LexLeanCollections.compareCodes, h]) (Coverage.Boundary.LexLeanCollections.Key.compare : String -> String -> Ordering) (fun _ _ => rfl)) __valid_6 (LexLeanPreservation.listEnc (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.string LexLeanTarget.TargetSyntax.Value.int)) (fun _ => rfl) __vrel_6 rfl

def __valid_10 : Nat -> Bool :=
  fun _ => true

def __inv_10 : Nat -> Prop :=
  fun _ => True

theorem __viff_10 : ∀ (__v : Nat), __valid_10 __v = true ↔ __inv_10 __v :=
  fun __v => LexLeanPreservation.validTrue_inv __v

theorem __vrel_10 : ∀ (__v : Nat), LexLeanPreservation.FunRel __prog 10 [(LexLeanTarget.TargetSyntax.Value.nat __v)] (LexLeanPreservation.Rel true (LexLeanTarget.TargetSyntax.Value.bool (__valid_10 __v))) :=
  LexLeanPreservation.tpl_validTrue LexLeanTarget.TargetSyntax.Value.nat rfl

def __valid_11 : (List (Prod Nat Int)) -> Bool :=
  LexLeanPreservation.ascMap (Coverage.Boundary.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) __valid_6

def __inv_11 : (List (Prod Nat Int)) -> Prop :=
  LexLeanPreservation.InvMap (Coverage.Boundary.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) __inv_6

theorem __viff_11 : ∀ (__v : (List (Prod Nat Int))), __valid_11 __v = true ↔ __inv_11 __v :=
  LexLeanPreservation.ascMap_inv (Coverage.Boundary.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) __valid_6 __inv_6 __viff_6

theorem __vrel_11 : ∀ (__v : (List (Prod Nat Int))), LexLeanPreservation.FunRel __prog 11 [((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.int)) __v)] (LexLeanPreservation.Rel true (LexLeanTarget.TargetSyntax.Value.bool (__valid_11 __v))) :=
  LexLeanPreservation.tpl_validMap (LexLeanPreservation.keySpec_nat (Coverage.Boundary.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) (fun _ _ => rfl)) __valid_6 (LexLeanPreservation.listEnc (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.int)) (fun _ => rfl) __vrel_6 rfl

mutual
def __valid_2 : (Coverage.Boundary.Grove) -> Bool
  | Coverage.Boundary.Grove.node __x0 __x1 __x2 __x3 __x4 => (if __valid_3 __x0 then (if __valid_4 __x1 then (if __valid_5 __x2 then (if __valid_7 __x3 then (if __valid_8 __x4 then true else false) else false) else false) else false) else false)
  | Coverage.Boundary.Grove.leaf __x0 => (if __valid_11 __x0 then true else false)
  | Coverage.Boundary.Grove.settled __x0 => (if __valid_12 __x0 then true else false)

def __valid_4 : (List (Coverage.Boundary.Grove)) -> Bool
  | [] => true
  | __x0 :: __x1 => if __valid_2 __x0 then __valid_4 __x1 else false

def __valid_7 : (Option (Coverage.Boundary.Grove)) -> Bool
  | none => true
  | some __x0 => __valid_2 __x0

def __valid_8 : (List (Prod Nat (Coverage.Boundary.Grove))) -> Bool
  | [] => true
  | __x0 :: __x1 => if __valid_9 __x0 then __valid_8 __x1 else false

def __valid_9 : (Prod Nat (Coverage.Boundary.Grove)) -> Bool
  | (__x0, __x1) => if __valid_10 __x0 then __valid_2 __x1 else false

def __valid_12 : (Except Nat (Coverage.Boundary.Grove)) -> Bool
  | Except.error __x0 => __valid_10 __x0
  | Except.ok __x0 => __valid_2 __x0

end

mutual
def __inv_2 : (Coverage.Boundary.Grove) -> Prop
  | Coverage.Boundary.Grove.node __x0 __x1 __x2 __x3 __x4 => (__inv_3 __x0 ∧ (__inv_4 __x1 ∧ (__inv_5 __x2 ∧ (__inv_7 __x3 ∧ (__inv_8 __x4 ∧ True)))))
  | Coverage.Boundary.Grove.leaf __x0 => (__inv_11 __x0 ∧ True)
  | Coverage.Boundary.Grove.settled __x0 => (__inv_12 __x0 ∧ True)

def __inv_4 : (List (Coverage.Boundary.Grove)) -> Prop
  | [] => True
  | __x0 :: __x1 => __inv_2 __x0 ∧ __inv_4 __x1

def __inv_7 : (Option (Coverage.Boundary.Grove)) -> Prop
  | none => True
  | some __x0 => __inv_2 __x0

def __inv_8 : (List (Prod Nat (Coverage.Boundary.Grove))) -> Prop
  | [] => True
  | __x0 :: __x1 => __inv_9 __x0 ∧ __inv_8 __x1

def __inv_9 : (Prod Nat (Coverage.Boundary.Grove)) -> Prop
  | (__x0, __x1) => __inv_10 __x0 ∧ __inv_2 __x1

def __inv_12 : (Except Nat (Coverage.Boundary.Grove)) -> Prop
  | Except.error __x0 => __inv_10 __x0
  | Except.ok __x0 => __inv_2 __x0

end

mutual
theorem __viff_2 : ∀ (__v : (Coverage.Boundary.Grove)), __valid_2 __v = true ↔ __inv_2 __v
  | Coverage.Boundary.Grove.node __x0 __x1 __x2 __x3 __x4 => by rw [__valid_2.eq_def, __inv_2.eq_def]; exact (LexLeanPreservation.condAnd_iff (__viff_3 __x0) (LexLeanPreservation.condAnd_iff (__viff_4 __x1) (LexLeanPreservation.condAnd_iff (__viff_5 __x2) (LexLeanPreservation.condAnd_iff (__viff_7 __x3) (LexLeanPreservation.condAnd_iff (__viff_8 __x4) LexLeanPreservation.trueIff)))))
  | Coverage.Boundary.Grove.leaf __x0 => by rw [__valid_2.eq_def, __inv_2.eq_def]; exact (LexLeanPreservation.condAnd_iff (__viff_11 __x0) LexLeanPreservation.trueIff)
  | Coverage.Boundary.Grove.settled __x0 => by rw [__valid_2.eq_def, __inv_2.eq_def]; exact (LexLeanPreservation.condAnd_iff (__viff_12 __x0) LexLeanPreservation.trueIff)

theorem __viff_4 : ∀ (__v : (List (Coverage.Boundary.Grove))), __valid_4 __v = true ↔ __inv_4 __v
  | [] => by rw [__valid_4.eq_def, __inv_4.eq_def]; exact LexLeanPreservation.trueIff
  | __x0 :: __x1 => by rw [__valid_4.eq_def, __inv_4.eq_def]; exact LexLeanPreservation.condAnd_iff (__viff_2 __x0) (__viff_4 __x1)

theorem __viff_7 : ∀ (__v : (Option (Coverage.Boundary.Grove))), __valid_7 __v = true ↔ __inv_7 __v
  | none => by rw [__valid_7.eq_def, __inv_7.eq_def]; exact LexLeanPreservation.trueIff
  | some __x0 => by rw [__valid_7.eq_def, __inv_7.eq_def]; exact __viff_2 __x0

theorem __viff_8 : ∀ (__v : (List (Prod Nat (Coverage.Boundary.Grove)))), __valid_8 __v = true ↔ __inv_8 __v
  | [] => by rw [__valid_8.eq_def, __inv_8.eq_def]; exact LexLeanPreservation.trueIff
  | __x0 :: __x1 => by rw [__valid_8.eq_def, __inv_8.eq_def]; exact LexLeanPreservation.condAnd_iff (__viff_9 __x0) (__viff_8 __x1)

theorem __viff_9 : ∀ (__v : (Prod Nat (Coverage.Boundary.Grove))), __valid_9 __v = true ↔ __inv_9 __v
  | (__x0, __x1) => by rw [__valid_9.eq_def, __inv_9.eq_def]; exact LexLeanPreservation.condAnd_iff (__viff_10 __x0) (__viff_2 __x1)

theorem __viff_12 : ∀ (__v : (Except Nat (Coverage.Boundary.Grove))), __valid_12 __v = true ↔ __inv_12 __v
  | Except.error __x0 => by rw [__valid_12.eq_def, __inv_12.eq_def]; exact __viff_10 __x0
  | Except.ok __x0 => by rw [__valid_12.eq_def, __inv_12.eq_def]; exact __viff_2 __x0

end

mutual
theorem __vrel_2 : ∀ (__v : (Coverage.Boundary.Grove)), LexLeanPreservation.FunRel __prog 2 [(__enc_0 __v)] (LexLeanPreservation.Rel true (LexLeanTarget.TargetSyntax.Value.bool (__valid_2 __v)))
  | Coverage.Boundary.Grove.node __x0 __x1 __x2 __x3 __x4 => by rw [__valid_2.eq_def]; exact LexLeanPreservation.FunRel.fits_eq (LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_cond (fun _ => true) (fun __c => LexLeanTarget.TargetSyntax.Value.bool (if __c then (if __valid_4 __x1 then (if __valid_5 __x2 then (if __valid_7 __x3 then (if __valid_8 __x4 then true else false) else false) else false) else false) else false)) (__valid_3 __x0) (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__vrel_3 __x0)) (fun _ => (LexLeanPreservation.conv_cond (fun _ => true) (fun __c => LexLeanTarget.TargetSyntax.Value.bool (if __c then (if __valid_5 __x2 then (if __valid_7 __x3 then (if __valid_8 __x4 then true else false) else false) else false) else false)) (__valid_4 __x1) (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__vrel_4 __x1)) (fun _ => (LexLeanPreservation.conv_cond (fun _ => true) (fun __c => LexLeanTarget.TargetSyntax.Value.bool (if __c then (if __valid_7 __x3 then (if __valid_8 __x4 then true else false) else false) else false)) (__valid_5 __x2) (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__vrel_5 __x2)) (fun _ => (LexLeanPreservation.conv_cond (fun _ => true) (fun __c => LexLeanTarget.TargetSyntax.Value.bool (if __c then (if __valid_8 __x4 then true else false) else false)) (__valid_7 __x3) (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__vrel_7 __x3)) (fun _ => (LexLeanPreservation.conv_cond (fun _ => true) (fun __c => LexLeanTarget.TargetSyntax.Value.bool (if __c then true else false)) (__valid_8 __x4) (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__vrel_8 __x4)) (fun _ => LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_true) (fun _ => LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_false))) (fun _ => LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_false))) (fun _ => LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_false))) (fun _ => LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_false))) (fun _ => LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_false))))) rfl
  | Coverage.Boundary.Grove.leaf __x0 => by rw [__valid_2.eq_def]; exact LexLeanPreservation.FunRel.fits_eq (LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_cond (fun _ => true) (fun __c => LexLeanTarget.TargetSyntax.Value.bool (if __c then true else false)) (__valid_11 __x0) (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__vrel_11 __x0)) (fun _ => LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_true) (fun _ => LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_false)))))) rfl
  | Coverage.Boundary.Grove.settled __x0 => by rw [__valid_2.eq_def]; exact LexLeanPreservation.FunRel.fits_eq (LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_cond (fun _ => true) (fun __c => LexLeanTarget.TargetSyntax.Value.bool (if __c then true else false)) (__valid_12 __x0) (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__vrel_12 __x0)) (fun _ => LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_true) (fun _ => LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_false))))))) rfl

theorem __vrel_4 : ∀ (__v : (List (Coverage.Boundary.Grove))), LexLeanPreservation.FunRel __prog 4 [((LexLeanPreservation.ListEnc.enc __L_0) __v)] (LexLeanPreservation.Rel true (LexLeanTarget.TargetSyntax.Value.bool (__valid_4 __v)))
  | [] => by rw [__valid_4.eq_def]; exact LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_true)))
  | __x0 :: __x1 => by rw [__valid_4.eq_def]; exact LexLeanPreservation.FunRel.fits_eq (LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_cond (fun _ => true) (fun __c => LexLeanTarget.TargetSyntax.Value.bool (if __c then __valid_4 __x1 else false)) (__valid_2 __x0) (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__vrel_2 __x0)) (fun _ => LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__vrel_4 __x1)) (fun _ => LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_false)))))) rfl

theorem __vrel_7 : ∀ (__v : (Option (Coverage.Boundary.Grove))), LexLeanPreservation.FunRel __prog 7 [((LexLeanPreservation.OptEnc.enc __O_1) __v)] (LexLeanPreservation.Rel true (LexLeanTarget.TargetSyntax.Value.bool (__valid_7 __v)))
  | none => by rw [__valid_7.eq_def]; exact LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_true)))
  | some __x0 => by rw [__valid_7.eq_def]; exact LexLeanPreservation.FunRel.fits_eq (LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__vrel_2 __x0)))))) rfl

theorem __vrel_8 : ∀ (__v : (List (Prod Nat (Coverage.Boundary.Grove)))), LexLeanPreservation.FunRel __prog 8 [((LexLeanPreservation.ListEnc.enc __L_3) __v)] (LexLeanPreservation.Rel true (LexLeanTarget.TargetSyntax.Value.bool (__valid_8 __v)))
  | [] => by rw [__valid_8.eq_def]; exact LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_true)))
  | __x0 :: __x1 => by rw [__valid_8.eq_def]; exact LexLeanPreservation.FunRel.fits_eq (LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_cond (fun _ => true) (fun __c => LexLeanTarget.TargetSyntax.Value.bool (if __c then __valid_8 __x1 else false)) (__valid_9 __x0) (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__vrel_9 __x0)) (fun _ => LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__vrel_8 __x1)) (fun _ => LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_false)))))) rfl

theorem __vrel_9 : ∀ (__v : (Prod Nat (Coverage.Boundary.Grove))), LexLeanPreservation.FunRel __prog 9 [(__aux_2 __v)] (LexLeanPreservation.Rel true (LexLeanTarget.TargetSyntax.Value.bool (__valid_9 __v)))
  | (__x0, __x1) => by rw [__valid_9.eq_def]; exact LexLeanPreservation.FunRel.fits_eq (LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_cond (fun _ => true) (fun __c => LexLeanTarget.TargetSyntax.Value.bool (if __c then __valid_2 __x1 else false)) (__valid_10 __x0) (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_first (LexLeanPreservation.conv_var rfl)) LexLeanPreservation.convL_nil) (__vrel_10 __x0)) (fun _ => LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_second (LexLeanPreservation.conv_var rfl)) LexLeanPreservation.convL_nil) (__vrel_2 __x1)) (fun _ => LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_false))) rfl

theorem __vrel_12 : ∀ (__v : (Except Nat (Coverage.Boundary.Grove))), LexLeanPreservation.FunRel __prog 12 [(__aux_4 __v)] (LexLeanPreservation.Rel true (LexLeanTarget.TargetSyntax.Value.bool (__valid_12 __v)))
  | Except.error __x0 => by rw [__valid_12.eq_def]; exact LexLeanPreservation.FunRel.fits_eq (LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__vrel_10 __x0)))))) rfl
  | Except.ok __x0 => by rw [__valid_12.eq_def]; exact LexLeanPreservation.FunRel.fits_eq (LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__vrel_2 __x0))))) rfl

end

theorem __vinv_4 : ∀ (__v : (List (Coverage.Boundary.Grove))), __inv_4 __v ↔ LexLeanPreservation.InvList __inv_2 __v :=
  LexLeanPreservation.invList_eqs __inv_2 __inv_4 (by rw [__inv_4.eq_def]) (fun _ _ => by rw [__inv_4.eq_def])

theorem __vinv_7 : ∀ (__v : (Option (Coverage.Boundary.Grove))), __inv_7 __v ↔ LexLeanPreservation.InvOption __inv_2 __v
  | none => by rw [__inv_7.eq_def]; exact Iff.rfl
  | some _ => by rw [__inv_7.eq_def]; exact Iff.rfl

theorem __vinv_8 : ∀ (__v : (List (Prod Nat (Coverage.Boundary.Grove)))), __inv_8 __v ↔ LexLeanPreservation.InvList __inv_9 __v :=
  LexLeanPreservation.invList_eqs __inv_9 __inv_8 (by rw [__inv_8.eq_def]) (fun _ _ => by rw [__inv_8.eq_def])

theorem __vinv_9 : ∀ (__v : (Prod Nat (Coverage.Boundary.Grove))), __inv_9 __v ↔ LexLeanPreservation.InvPair __inv_10 __inv_2 __v
  | (_, _) => by rw [__inv_9.eq_def]; exact Iff.rfl

theorem __vinv_12 : ∀ (__v : (Except Nat (Coverage.Boundary.Grove))), __inv_12 __v ↔ LexLeanPreservation.InvExcept __inv_10 __inv_2 __v
  | Except.error _ => by rw [__inv_12.eq_def]; exact Iff.rfl
  | Except.ok _ => by rw [__inv_12.eq_def]; exact Iff.rfl

def __valid_13 (__s : (Coverage.Boundary.Ledger)) : Bool :=
  (if __valid_5 (__s).entries then true else false)

def __inv_13 (__s : (Coverage.Boundary.Ledger)) : Prop :=
  (__inv_5 (__s).entries ∧ True)

theorem __viff_13 : ∀ (__s : (Coverage.Boundary.Ledger)), __valid_13 __s = true ↔ __inv_13 __s :=
  fun __s => (LexLeanPreservation.condAnd_iff (__viff_5 (__s).entries) LexLeanPreservation.trueIff)

theorem __vrel_13 : ∀ (__s : (Coverage.Boundary.Ledger)), LexLeanPreservation.FunRel __prog 13 [(__enc_1 __s)] (LexLeanPreservation.Rel true (LexLeanTarget.TargetSyntax.Value.bool (__valid_13 __s))) :=
  fun __s => LexLeanPreservation.FunRel.fits_eq (LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_cond (fun _ => true) (fun __c => LexLeanTarget.TargetSyntax.Value.bool (if __c then true else false)) (__valid_5 (__s).entries) (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__vrel_5 (__s).entries)) (fun _ => LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_true) (fun _ => LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_false))))) rfl

def __valid_14 : (Option (List Nat)) -> Bool :=
  LexLeanPreservation.validOption __valid_3

def __inv_14 : (Option (List Nat)) -> Prop :=
  LexLeanPreservation.InvOption __inv_3

theorem __viff_14 : ∀ (__v : (Option (List Nat))), __valid_14 __v = true ↔ __inv_14 __v :=
  LexLeanPreservation.validOption_inv __valid_3 __inv_3 __viff_3

theorem __vrel_14 : ∀ (__v : (Option (List Nat))), LexLeanPreservation.FunRel __prog 14 [((LexLeanPreservation.encOption (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat)) __v)] (LexLeanPreservation.Rel true (LexLeanTarget.TargetSyntax.Value.bool (__valid_14 __v))) :=
  LexLeanPreservation.tpl_validOption __valid_3 __vrel_3 rfl

def __valid_17 : Bool -> Bool :=
  fun _ => true

def __inv_17 : Bool -> Prop :=
  fun _ => True

theorem __viff_17 : ∀ (__v : Bool), __valid_17 __v = true ↔ __inv_17 __v :=
  fun __v => LexLeanPreservation.validTrue_inv __v

theorem __vrel_17 : ∀ (__v : Bool), LexLeanPreservation.FunRel __prog 17 [(LexLeanTarget.TargetSyntax.Value.bool __v)] (LexLeanPreservation.Rel true (LexLeanTarget.TargetSyntax.Value.bool (__valid_17 __v))) :=
  LexLeanPreservation.tpl_validTrue LexLeanTarget.TargetSyntax.Value.bool rfl

def __valid_16 : (List (Prod Nat Bool)) -> Bool :=
  LexLeanPreservation.ascMap (Coverage.Boundary.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) __valid_17

def __inv_16 : (List (Prod Nat Bool)) -> Prop :=
  LexLeanPreservation.InvMap (Coverage.Boundary.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) __inv_17

theorem __viff_16 : ∀ (__v : (List (Prod Nat Bool))), __valid_16 __v = true ↔ __inv_16 __v :=
  LexLeanPreservation.ascMap_inv (Coverage.Boundary.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) __valid_17 __inv_17 __viff_17

theorem __vrel_16 : ∀ (__v : (List (Prod Nat Bool))), LexLeanPreservation.FunRel __prog 16 [((LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.bool)) __v)] (LexLeanPreservation.Rel true (LexLeanTarget.TargetSyntax.Value.bool (__valid_16 __v))) :=
  LexLeanPreservation.tpl_validMap (LexLeanPreservation.keySpec_nat (Coverage.Boundary.LexLeanCollections.Key.compare : Nat -> Nat -> Ordering) (fun _ _ => rfl)) __valid_17 (LexLeanPreservation.listEnc (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.bool)) (fun _ => rfl) __vrel_17 rfl

def __valid_15 : (List (List (Prod Nat Bool))) -> Bool :=
  LexLeanPreservation.validList __valid_16

def __inv_15 : (List (List (Prod Nat Bool))) -> Prop :=
  LexLeanPreservation.InvList __inv_16

theorem __viff_15 : ∀ (__v : (List (List (Prod Nat Bool)))), __valid_15 __v = true ↔ __inv_15 __v :=
  LexLeanPreservation.validList_inv __valid_16 __inv_16 __viff_16

theorem __vrel_15 : ∀ (__v : (List (List (Prod Nat Bool)))), LexLeanPreservation.FunRel __prog 15 [((LexLeanPreservation.encList (LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.bool))) __v)] (LexLeanPreservation.Rel true (LexLeanTarget.TargetSyntax.Value.bool (__valid_15 __v))) :=
  LexLeanPreservation.tpl_validList __valid_16 (LexLeanPreservation.listEnc (LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.bool))) (fun _ => rfl) __vrel_16 rfl

def entryFits (g : (Coverage.Boundary.Grove)) (ledger : (Coverage.Boundary.Ledger)) (picks : (Option (List Nat))) (spare : (List (List (Prod Nat Bool)))) : Bool :=
  (if __valid_2 g then (if __valid_13 ledger then (if __valid_14 picks then (if __valid_15 spare then __fits_0 g ledger picks spare else true) else true) else true) else true)

def entryValue (g : (Coverage.Boundary.Grove)) (ledger : (Coverage.Boundary.Ledger)) (picks : (Option (List Nat))) (spare : (List (List (Prod Nat Bool)))) : LexLeanTarget.TargetSyntax.Value :=
  (if __valid_2 g then (if __valid_13 ledger then (if __valid_14 picks then (if __valid_15 spare then (LexLeanTarget.TargetSyntax.Value.some ((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat (LexLeanPreservation.encPair __enc_1 (LexLeanPreservation.encPair (LexLeanPreservation.encOption (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat)) (LexLeanPreservation.encList (LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.bool)))))) (Coverage.Boundary.groveRoot g ledger picks spare))) else LexLeanTarget.TargetSyntax.Value.none) else LexLeanTarget.TargetSyntax.Value.none) else LexLeanTarget.TargetSyntax.Value.none) else LexLeanTarget.TargetSyntax.Value.none)

/-- The entry's observation. -/
def denoteEntry (g : (Coverage.Boundary.Grove)) (ledger : (Coverage.Boundary.Ledger)) (picks : (Option (List Nat))) (spare : (List (List (Prod Nat Bool)))) : LexLeanPreservation.Obs :=
  cond (entryFits g ledger picks spare) (LexLeanPreservation.Obs.value (entryValue g ledger picks spare)) LexLeanPreservation.Obs.overflow

/-- §17.12's invariants of the validated parameters. -/
def accepts (g : (Coverage.Boundary.Grove)) (ledger : (Coverage.Boundary.Ledger)) (picks : (Option (List Nat))) (spare : (List (List (Prod Nat Bool)))) : Prop :=
  (__inv_2 g ∧ (__inv_13 ledger ∧ (__inv_14 picks ∧ (__inv_15 spare ∧ True))))

theorem entry (g : (Coverage.Boundary.Grove)) (ledger : (Coverage.Boundary.Ledger)) (picks : (Option (List Nat))) (spare : (List (List (Prod Nat Bool)))) : LexLeanPreservation.RunConv __prog 18 [(__enc_0 g), (__enc_1 ledger), ((LexLeanPreservation.encOption (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat)) picks), ((LexLeanPreservation.encList (LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.bool))) spare)] (denoteEntry g ledger picks spare) :=
  LexLeanPreservation.run_of_funRel (LexLeanPreservation.FunRel.fits_eq (LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_cond (fun __c => if __c then (if __valid_13 ledger then (if __valid_14 picks then (if __valid_15 spare then __fits_0 g ledger picks spare else true) else true) else true) else true) (fun __c => if __c then (if __valid_13 ledger then (if __valid_14 picks then (if __valid_15 spare then (LexLeanTarget.TargetSyntax.Value.some ((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat (LexLeanPreservation.encPair __enc_1 (LexLeanPreservation.encPair (LexLeanPreservation.encOption (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat)) (LexLeanPreservation.encList (LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.bool)))))) (Coverage.Boundary.groveRoot g ledger picks spare))) else LexLeanTarget.TargetSyntax.Value.none) else LexLeanTarget.TargetSyntax.Value.none) else LexLeanTarget.TargetSyntax.Value.none) else LexLeanTarget.TargetSyntax.Value.none) (__valid_2 g) (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__vrel_2 g)) (fun _ => LexLeanPreservation.Conv.fits_eq (LexLeanPreservation.conv_cond (fun __c => if __c then (if __valid_14 picks then (if __valid_15 spare then __fits_0 g ledger picks spare else true) else true) else true) (fun __c => if __c then (if __valid_14 picks then (if __valid_15 spare then (LexLeanTarget.TargetSyntax.Value.some ((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat (LexLeanPreservation.encPair __enc_1 (LexLeanPreservation.encPair (LexLeanPreservation.encOption (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat)) (LexLeanPreservation.encList (LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.bool)))))) (Coverage.Boundary.groveRoot g ledger picks spare))) else LexLeanTarget.TargetSyntax.Value.none) else LexLeanTarget.TargetSyntax.Value.none) else LexLeanTarget.TargetSyntax.Value.none) (__valid_13 ledger) (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__vrel_13 ledger)) (fun _ => LexLeanPreservation.Conv.fits_eq (LexLeanPreservation.conv_cond (fun __c => if __c then (if __valid_15 spare then __fits_0 g ledger picks spare else true) else true) (fun __c => if __c then (if __valid_15 spare then (LexLeanTarget.TargetSyntax.Value.some ((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat (LexLeanPreservation.encPair __enc_1 (LexLeanPreservation.encPair (LexLeanPreservation.encOption (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat)) (LexLeanPreservation.encList (LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.bool)))))) (Coverage.Boundary.groveRoot g ledger picks spare))) else LexLeanTarget.TargetSyntax.Value.none) else LexLeanTarget.TargetSyntax.Value.none) (__valid_14 picks) (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__vrel_14 picks)) (fun _ => LexLeanPreservation.Conv.fits_eq (LexLeanPreservation.conv_cond (fun __c => if __c then __fits_0 g ledger picks spare else true) (fun __c => if __c then (LexLeanTarget.TargetSyntax.Value.some ((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat (LexLeanPreservation.encPair __enc_1 (LexLeanPreservation.encPair (LexLeanPreservation.encOption (LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat)) (LexLeanPreservation.encList (LexLeanPreservation.encList (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.bool)))))) (Coverage.Boundary.groveRoot g ledger picks spare))) else LexLeanTarget.TargetSyntax.Value.none) (__valid_15 spare) (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__vrel_15 spare)) (fun _ => LexLeanPreservation.Conv.fits_eq (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)))) (__rel_0 g ledger picks spare)) LexLeanPreservation.convL_nil) LexLeanPreservation.construct_some) (by simp)) (fun _ => LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_none)) (by simp)) (fun _ => LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_none)) (by simp)) (fun _ => LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_none)) (by simp)) (fun _ => LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_none))) (by simp [entryFits]))

theorem entry_accepts (g : (Coverage.Boundary.Grove)) (ledger : (Coverage.Boundary.Ledger)) (picks : (Option (List Nat))) (spare : (List (List (Prod Nat Bool)))) (__h : accepts g ledger picks spare) : denoteEntry g ledger picks spare = LexLeanPreservation.someObs (denote g ledger picks spare) := by
  unfold accepts at __h
  unfold denoteEntry entryFits entryValue denote
  simp only [if_true, (__viff_2 g).mpr __h.1, (__viff_13 ledger).mpr __h.2.1, (__viff_14 picks).mpr __h.2.2.1, (__viff_15 spare).mpr __h.2.2.2.1]
  cases __fits_0 g ledger picks spare <;> rfl

theorem entry_refuses (g : (Coverage.Boundary.Grove)) (ledger : (Coverage.Boundary.Ledger)) (picks : (Option (List Nat))) (spare : (List (List (Prod Nat Bool)))) (__h : ¬ accepts g ledger picks spare) : denoteEntry g ledger picks spare = LexLeanPreservation.Obs.value LexLeanTarget.TargetSyntax.Value.none := by
  unfold accepts at __h
  unfold denoteEntry entryFits entryValue
  cases __c0 : __valid_2 g
  · simp
  · cases __c1 : __valid_13 ledger
    · simp
    · cases __c2 : __valid_14 picks
      · simp
      · cases __c3 : __valid_15 spare
        · simp
        · exact absurd ⟨(__viff_2 g).mp __c0, (__viff_13 ledger).mp __c1, (__viff_14 picks).mp __c2, (__viff_15 spare).mp __c3, trivial⟩ __h

end LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R0
