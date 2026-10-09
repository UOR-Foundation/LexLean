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
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R13

def __prog : _root_.LexLeanTarget.TargetSyntax.Program :=
  { adts := [{ constructors := [[], [(.adt 0), .nat, (.adt 0)]] }], functions := [
    { parameters := [0], types := [.nat], result := .nat,
      body := (.call 1 [(.build (.adt 1) (.adt 0) [(.build (.adt 0) (.adt 0) []), (.var 0), (.build (.adt 1) (.adt 0) [(.build (.adt 0) (.adt 0) []), (.value .nat (.nat 5)), (.build (.adt 0) (.adt 0) [])])])]) },
    { parameters := [0], types := [(.adt 0)], result := .nat,
      body := (.«match» .nat (.var 0) [(.arm (.adt 0) [] (.value .nat (.nat 0))), (.arm (.adt 1) [1, 2, 3] (.prim .natAdd [(.prim .natAdd [(.call 1 [(.var 1)]), (.var 2)]), (.call 1 [(.var 3)])]))]) }] }

def __enc_0 : (Coverage.Types.Tree) -> _root_.LexLeanTarget.TargetSyntax.Value
  | Coverage.Types.Tree.leaf => _root_.LexLeanTarget.TargetSyntax.Value.adt 0 []
  | Coverage.Types.Tree.node __x0 __x1 __x2 => _root_.LexLeanTarget.TargetSyntax.Value.adt 1 [(__enc_0 __x0), (_root_.LexLeanTarget.TargetSyntax.Value.nat __x1), (__enc_0 __x2)]

def __fits_1 : ∀ (tree : (Coverage.Types.Tree)), Bool
  | (Coverage.Types.Tree.leaf) => (Bool.true && Bool.true)
  | (Coverage.Types.Tree.node l v r) => (Bool.true && ((((((Bool.true && Bool.true) && (__fits_1 (l))) && (Bool.true && Bool.true)) && (Nat.blt ((Coverage.Types.treeSum (l)) + v) 18446744073709551616)) && (((Bool.true && Bool.true) && (__fits_1 (r))) && Bool.true)) && (Nat.blt (((Coverage.Types.treeSum (l)) + v) + (Coverage.Types.treeSum (r))) 18446744073709551616)))

theorem __rel_1 : ∀ (tree : (Coverage.Types.Tree)), _root_.LexLeanPreservation.FunRel __prog 1 [(__enc_0 tree)] (_root_.LexLeanPreservation.Rel (__fits_1 tree) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.Types.treeSum tree)))
  | (Coverage.Types.Tree.leaf) => by rw [__fits_1.eq_def]; exact _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_match (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convA_hit rfl rfl _root_.LexLeanPreservation.conv_value))
  | (Coverage.Types.Tree.node l v r) => by rw [__fits_1.eq_def]; exact _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_match (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convA_miss rfl (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__rel_1 (l))) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natAdd (Coverage.Types.treeSum (l)) v)) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__rel_1 (r))) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natAdd ((Coverage.Types.treeSum (l)) + v) (Coverage.Types.treeSum (r)))))))

def __fits_0 (n : Nat) : Bool :=
  (((((Bool.true && Bool.true) && (Bool.true && ((((Bool.true && Bool.true) && (Bool.true && ((Bool.true && Bool.true) && Bool.true))) && Bool.true) && Bool.true))) && Bool.true) && Bool.true) && (__fits_1 ((Coverage.Types.Tree.node ((Coverage.Types.Tree.leaf : (Coverage.Types.Tree))) (n) ((Coverage.Types.Tree.node ((Coverage.Types.Tree.leaf : (Coverage.Types.Tree))) ((5 : Nat)) ((Coverage.Types.Tree.leaf : (Coverage.Types.Tree))) : (Coverage.Types.Tree))) : (Coverage.Types.Tree)))))

attribute [local irreducible] Coverage.Types.treeSum in
theorem __rel_0 (n : Nat) : _root_.LexLeanPreservation.FunRel __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat n)] (_root_.LexLeanPreservation.Rel (__fits_0 n) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.treeTotal n))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build _root_.LexLeanPreservation.convL_nil _root_.LexLeanPreservation.construct_adt) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build _root_.LexLeanPreservation.convL_nil _root_.LexLeanPreservation.construct_adt) (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build _root_.LexLeanPreservation.convL_nil _root_.LexLeanPreservation.construct_adt) _root_.LexLeanPreservation.convL_nil))) _root_.LexLeanPreservation.construct_adt) _root_.LexLeanPreservation.convL_nil))) _root_.LexLeanPreservation.construct_adt) _root_.LexLeanPreservation.convL_nil) (__rel_1 ((Coverage.Types.Tree.node ((Coverage.Types.Tree.leaf : (Coverage.Types.Tree))) (n) ((Coverage.Types.Tree.node ((Coverage.Types.Tree.leaf : (Coverage.Types.Tree))) ((5 : Nat)) ((Coverage.Types.Tree.leaf : (Coverage.Types.Tree))) : (Coverage.Types.Tree))) : (Coverage.Types.Tree)))))

def denote (n : Nat) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 n) (_root_.LexLeanPreservation.Obs.value ((_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.treeTotal n)))) _root_.LexLeanPreservation.Obs.overflow

theorem root (n : Nat) : _root_.LexLeanPreservation.RunConv __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat n)] (_root_.LexLeanPreservation.Rel (__fits_0 n) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.Main.treeTotal n))) :=
  _root_.LexLeanPreservation.run_of_funRel (__rel_0 n)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R13
