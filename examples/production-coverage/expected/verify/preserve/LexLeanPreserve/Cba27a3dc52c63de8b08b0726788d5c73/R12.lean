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
namespace LexLeanPreserve.Cba27a3dc52c63de8b08b0726788d5c73.R12

def __prog : LexLeanTarget.TargetSyntax.Program :=
  { adts := [{ constructors := [[], [(.adt 0), .nat, (.adt 0)]] }], functions := [
    { parameters := [0], types := [.nat], result := .nat,
      body := (.call 1 [(.build (.adt 1) (.adt 0) [(.build (.adt 0) (.adt 0) []), (.var 0), (.build (.adt 1) (.adt 0) [(.build (.adt 0) (.adt 0) []), (.value .nat (.nat 5)), (.build (.adt 0) (.adt 0) [])])])]) },
    { parameters := [0], types := [(.adt 0)], result := .nat,
      body := (.«match» .nat (.var 0) [(.arm (.adt 0) [] (.value .nat (.nat 0))), (.arm (.adt 1) [1, 2, 3] (.prim .natAdd [(.prim .natAdd [(.call 1 [(.var 1)]), (.var 2)]), (.call 1 [(.var 3)])]))]) }] }

def __enc_0 : (Coverage.Types.Tree) -> LexLeanTarget.TargetSyntax.Value
  | Coverage.Types.Tree.leaf => LexLeanTarget.TargetSyntax.Value.adt 0 []
  | Coverage.Types.Tree.node __x0 __x1 __x2 => LexLeanTarget.TargetSyntax.Value.adt 1 [(__enc_0 __x0), (LexLeanTarget.TargetSyntax.Value.nat __x1), (__enc_0 __x2)]

def __fits_1 : ∀ (tree : (Coverage.Types.Tree)), Bool
  | (Coverage.Types.Tree.leaf) => (true && true)
  | (Coverage.Types.Tree.node l v r) => (true && ((((((true && true) && (__fits_1 (l))) && (true && true)) && (Nat.blt ((Coverage.Types.treeSum (l)) + v) 18446744073709551616)) && (((true && true) && (__fits_1 (r))) && true)) && (Nat.blt (((Coverage.Types.treeSum (l)) + v) + (Coverage.Types.treeSum (r))) 18446744073709551616)))

theorem __rel_1 : ∀ (tree : (Coverage.Types.Tree)), LexLeanPreservation.FunRel __prog 1 [(__enc_0 tree)] (LexLeanPreservation.Rel (__fits_1 tree) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Types.treeSum tree)))
  | (Coverage.Types.Tree.leaf) => by rw [__fits_1.eq_def]; exact LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_hit rfl rfl LexLeanPreservation.conv_value))
  | (Coverage.Types.Tree.node l v r) => by rw [__fits_1.eq_def]; exact LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_1 (l))) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd (Coverage.Types.treeSum (l)) v)) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_1 (r))) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd ((Coverage.Types.treeSum (l)) + v) (Coverage.Types.treeSum (r)))))))

def __fits_0 (n : Nat) : Bool :=
  (((((true && true) && (true && ((((true && true) && (true && ((true && true) && true))) && true) && true))) && true) && true) && (__fits_1 ((Coverage.Types.Tree.node ((Coverage.Types.Tree.leaf : (Coverage.Types.Tree))) (n) ((Coverage.Types.Tree.node ((Coverage.Types.Tree.leaf : (Coverage.Types.Tree))) ((5 : Nat)) ((Coverage.Types.Tree.leaf : (Coverage.Types.Tree))) : (Coverage.Types.Tree))) : (Coverage.Types.Tree)))))

attribute [local irreducible] Coverage.Types.treeSum in
theorem __rel_0 (n : Nat) : LexLeanPreservation.FunRel __prog 0 [(LexLeanTarget.TargetSyntax.Value.nat n)] (LexLeanPreservation.Rel (__fits_0 n) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.treeTotal n))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_adt) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_adt) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_adt) LexLeanPreservation.convL_nil))) LexLeanPreservation.construct_adt) LexLeanPreservation.convL_nil))) LexLeanPreservation.construct_adt) LexLeanPreservation.convL_nil) (__rel_1 ((Coverage.Types.Tree.node ((Coverage.Types.Tree.leaf : (Coverage.Types.Tree))) (n) ((Coverage.Types.Tree.node ((Coverage.Types.Tree.leaf : (Coverage.Types.Tree))) ((5 : Nat)) ((Coverage.Types.Tree.leaf : (Coverage.Types.Tree))) : (Coverage.Types.Tree))) : (Coverage.Types.Tree)))))

def denote (n : Nat) : LexLeanPreservation.Obs :=
  LexLeanPreservation.Rel (__fits_0 n) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.treeTotal n))

theorem root (n : Nat) : LexLeanPreservation.RunConv __prog 0 [(LexLeanTarget.TargetSyntax.Value.nat n)] (LexLeanPreservation.Rel (__fits_0 n) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.treeTotal n))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 n)

end LexLeanPreserve.Cba27a3dc52c63de8b08b0726788d5c73.R12
