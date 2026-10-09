import LexLeanPreservation.Core
import LexLeanPreservation.Values
import LexLeanPreservation.Primitives
import LexLeanPreservation.Fixed
import LexLeanPreservation.Keys
import LexLeanPreservation.Templates
import LexLeanPreservation.Validate
import Production.Kernel
import Production.Main
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.C55a45db3449637cd1dd5bfdb5faa7bb4.R5

def __prog : _root_.LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0], types := [(.list .nat)], result := .nat,
      body := (.call 1 [(.var 0), (.value .nat (.nat 0))]) },
    { parameters := [0, 1], types := [(.list .nat), .nat], result := .nat,
      body := (.«match» .nat (.var 0) [(.arm .nil [] (.var 1)), (.arm .cons [2, 3] (.var 2))]) }] }

def __fits_1 (values : (List Nat)) (fallback : Nat) : Bool :=
  (Bool.true && ((fun (Item : Type) (__a : (List Item)) (__k0 : Bool) (__k1 : ∀ (head : Item) (tail : (List Item)), Bool) => match __a with | List.nil => __k0 | List.cons head tail => __k1 head tail) Nat values (Bool.true) (fun (head : Nat) (tail : (List Nat)) => Bool.true)))

theorem __rel_1 (values : (List Nat)) (fallback : Nat) : _root_.LexLeanPreservation.FunRel __prog 1 [((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.nat) values), (_root_.LexLeanTarget.TargetSyntax.Value.nat fallback)] (_root_.LexLeanPreservation.Rel (__fits_1 values fallback) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Production.Kernel.firstOr Nat values fallback))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_matchV (_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.nat) (fun (__a : (List Nat)) => ((fun (Item : Type) (__a : (List Item)) (__k0 : Bool) (__k1 : ∀ (head : Item) (tail : (List Item)), Bool) => match __a with | List.nil => __k0 | List.cons head tail => __k1 head tail) Nat __a (Bool.true) (fun (head : Nat) (tail : (List Nat)) => Bool.true))) (fun (__a : (List Nat)) => _root_.LexLeanTarget.TargetSyntax.Value.nat ((fun (Item : Type) (__a : (List Item)) (__k0 : Nat) (__k1 : ∀ (head : Item) (tail : (List Item)), Nat) => match __a with | List.nil => __k0 | List.cons head tail => __k1 head tail) Nat __a (fallback) (fun (head : Nat) (tail : (List Nat)) => head))) values (_root_.LexLeanPreservation.conv_var rfl) (fun (__a : (List Nat)) _ => match __a with | List.nil => (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_var rfl)) | List.cons head tail => (_root_.LexLeanPreservation.convA_miss rfl (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_var rfl)))))

def __fits_0 (values : (List Nat)) : Bool :=
  ((Bool.true && (Bool.true && Bool.true)) && (__fits_1 (values) ((0 : Nat))))

attribute [local irreducible] Production.Kernel.firstOr in
theorem __rel_0 (values : (List Nat)) : _root_.LexLeanPreservation.FunRel __prog 0 [((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.nat) values)] (_root_.LexLeanPreservation.Rel (__fits_0 values) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Production.Main.headOr values))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value _root_.LexLeanPreservation.convL_nil)) (__rel_1 (values) ((0 : Nat))))

def denote (values : (List Nat)) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 values) (_root_.LexLeanPreservation.Obs.value ((_root_.LexLeanTarget.TargetSyntax.Value.nat (Production.Main.headOr values)))) _root_.LexLeanPreservation.Obs.overflow

theorem root (values : (List Nat)) : _root_.LexLeanPreservation.RunConv __prog 0 [((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.nat) values)] (_root_.LexLeanPreservation.Rel (__fits_0 values) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Production.Main.headOr values))) :=
  _root_.LexLeanPreservation.run_of_funRel (__rel_0 values)

end LexLeanPreserve.C55a45db3449637cd1dd5bfdb5faa7bb4.R5
