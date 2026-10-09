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
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R30

def __prog : _root_.LexLeanTarget.TargetSyntax.Program :=
  { adts := [{ constructors := [[.nat, (.list (.adt 0))]] }], functions := [
    { parameters := [0], types := [.nat], result := .nat,
      body := (.call 1 [(.build (.adt 0) (.adt 0) [(.var 0), (.build .cons (.list (.adt 0)) [(.build (.adt 0) (.adt 0) [(.value .nat (.nat 1)), (.build .nil (.list (.adt 0)) [])]), (.build .nil (.list (.adt 0)) [])])])]) },
    { parameters := [0], types := [(.adt 0)], result := .nat,
      body := (.«match» .nat (.var 0) [(.arm (.adt 0) [1, 2] (.prim .natAdd [(.value .nat (.nat 1)), (.call 2 [(.var 2)])]))]) },
    { parameters := [0], types := [(.list (.adt 0))], result := .nat,
      body := (.«match» .nat (.var 0) [(.arm .nil [] (.value .nat (.nat 0))), (.arm .cons [1, 2] (.prim .natAdd [(.call 1 [(.var 1)]), (.call 2 [(.var 2)])]))]) }] }

mutual
def __enc_0 : (Coverage.Syntax.Rose Nat) -> _root_.LexLeanTarget.TargetSyntax.Value
  | Coverage.Syntax.Rose.node __x0 __x1 => _root_.LexLeanTarget.TargetSyntax.Value.adt 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat __x0), (_root_.LexLeanTarget.TargetSyntax.Value.list (__items_0 __x1))]

def __items_0 : (List (Coverage.Syntax.Rose Nat)) -> List _root_.LexLeanTarget.TargetSyntax.Value
  | [] => []
  | __x0 :: __x1 => (__enc_0 __x0) :: __items_0 __x1

end

def __L_0 : _root_.LexLeanPreservation.ListEnc (Coverage.Syntax.Rose Nat) :=
  ⟨__enc_0, __items_0, __items_0.eq_1, __items_0.eq_2⟩

mutual
def __fits_1 : ∀ (rose : (Coverage.Syntax.Rose Nat)), Bool
  | (Coverage.Syntax.Rose.node label children) => (Bool.true && ((Bool.true && (((Bool.true && Bool.true) && (__fits_2 (children))) && Bool.true)) && (Nat.blt ((1 : Nat) + (Coverage.Recur.forestSize Nat (children))) 18446744073709551616)))
termination_by structural rose => rose

def __fits_2 : ∀ (forest : (List (Coverage.Syntax.Rose Nat))), Bool
  | (List.nil) => (Bool.true && Bool.true)
  | (List.cons head tail) => (Bool.true && ((((Bool.true && Bool.true) && (__fits_1 (head))) && (((Bool.true && Bool.true) && (__fits_2 (tail))) && Bool.true)) && (Nat.blt ((Coverage.Recur.roseSize Nat (head)) + (Coverage.Recur.forestSize Nat (tail))) 18446744073709551616)))
termination_by structural forest => forest

end

mutual
theorem __rel_1 : ∀ (rose : (Coverage.Syntax.Rose Nat)), _root_.LexLeanPreservation.FunRel __prog 1 [(__enc_0 rose)] (_root_.LexLeanPreservation.Rel (__fits_1 rose) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.Recur.roseSize Nat rose)))
  | (Coverage.Syntax.Rose.node label children) => by rw [__fits_1.eq_def]; exact _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_match (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__rel_2 (children))) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natAdd (1 : Nat) (Coverage.Recur.forestSize Nat (children))))))
termination_by structural rose => rose

theorem __rel_2 : ∀ (forest : (List (Coverage.Syntax.Rose Nat))), _root_.LexLeanPreservation.FunRel __prog 2 [((_root_.LexLeanPreservation.ListEnc.enc __L_0) forest)] (_root_.LexLeanPreservation.Rel (__fits_2 forest) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.Recur.forestSize Nat forest)))
  | (List.nil) => by rw [__fits_2.eq_def]; exact _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_match (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convA_hit rfl rfl _root_.LexLeanPreservation.conv_value))
  | (List.cons head tail) => by rw [__fits_2.eq_def]; exact _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_match (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convA_miss rfl (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__rel_1 (head))) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__rel_2 (tail))) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natAdd (Coverage.Recur.roseSize Nat (head)) (Coverage.Recur.forestSize Nat (tail)))))))
termination_by structural forest => forest

end

def __fits_0 (n : Nat) : Bool :=
  ((((Bool.true && (((((Bool.true && ((Bool.true && Bool.true) && Bool.true)) && Bool.true) && ((Bool.true && Bool.true) && Bool.true)) && Bool.true) && Bool.true)) && Bool.true) && Bool.true) && (__fits_1 ((Coverage.Syntax.Rose.node (n) (((Coverage.Syntax.Rose.node ((1 : Nat)) (([] : List (Coverage.Syntax.Rose Nat))) : (Coverage.Syntax.Rose Nat)) :: ([] : List (Coverage.Syntax.Rose Nat)))) : (Coverage.Syntax.Rose Nat)))))

attribute [local irreducible] Coverage.Recur.roseSize in
theorem __rel_0 (n : Nat) : _root_.LexLeanPreservation.FunRel __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat n)] (_root_.LexLeanPreservation.Rel (__fits_0 n) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.RecRoots.roseTotal n))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build _root_.LexLeanPreservation.convL_nil _root_.LexLeanPreservation.construct_nil) _root_.LexLeanPreservation.convL_nil)) _root_.LexLeanPreservation.construct_adt) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build _root_.LexLeanPreservation.convL_nil _root_.LexLeanPreservation.construct_nil) _root_.LexLeanPreservation.convL_nil)) _root_.LexLeanPreservation.construct_cons) _root_.LexLeanPreservation.convL_nil)) _root_.LexLeanPreservation.construct_adt) _root_.LexLeanPreservation.convL_nil) (__rel_1 ((Coverage.Syntax.Rose.node (n) (((Coverage.Syntax.Rose.node ((1 : Nat)) (([] : List (Coverage.Syntax.Rose Nat))) : (Coverage.Syntax.Rose Nat)) :: ([] : List (Coverage.Syntax.Rose Nat)))) : (Coverage.Syntax.Rose Nat)))))

def denote (n : Nat) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 n) (_root_.LexLeanPreservation.Obs.value ((_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.RecRoots.roseTotal n)))) _root_.LexLeanPreservation.Obs.overflow

theorem root (n : Nat) : _root_.LexLeanPreservation.RunConv __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat n)] (_root_.LexLeanPreservation.Rel (__fits_0 n) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.RecRoots.roseTotal n))) :=
  _root_.LexLeanPreservation.run_of_funRel (__rel_0 n)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R30
