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
namespace LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R19

def __prog : _root_.LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0], types := [(.list .nat)], result := (.list .bool),
      body := (.call 1 [(.closure 2 []), (.var 0)]) },
    { parameters := [0, 1], types := [(.fn [.nat] .bool), (.list .nat)], result := (.list .bool),
      body := (.«match» (.list .bool) (.var 1) [(.arm .nil [] (.build .nil (.list .bool) [])), (.arm .cons [2, 3] (.build .cons (.list .bool) [(.apply (.var 0) [(.var 2)]), (.call 1 [(.var 0), (.var 3)])]))]) },
    { parameters := [0], types := [.nat], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .zero [] (.build .true .bool [])), (.arm .succ [1] (.call 3 [(.var 1)]))]) },
    { parameters := [0], types := [.nat], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .zero [] (.build .false .bool [])), (.arm .succ [1] (.call 2 [(.var 1)]))]) }] }

def __fits_1 : ∀ (transform : (Nat -> Bool)) (__ff_0 : Nat -> Bool) (values : (List Nat)), Bool
  | transform, __ff_0, (List.nil) => (Bool.true && (Bool.true && Bool.true))
  | transform, __ff_0, (List.cons head tail) => (Bool.true && (((Bool.true && ((Bool.true && Bool.true) && (__ff_0 (head)))) && (((Bool.true && (Bool.true && Bool.true)) && (__fits_1 (transform) (__ff_0) (tail))) && Bool.true)) && Bool.true))

theorem __rel_1 : ∀ (transform : (Nat -> Bool)) (__ff_0 : Nat -> Bool) (__fi_0 : Nat) (__fc_0 : List _root_.LexLeanTarget.TargetSyntax.Value) (__fh_0 : ∀ (__p0 : Nat), _root_.LexLeanPreservation.FunRel __prog __fi_0 (_root_.LexLeanTarget.TargetSemantics.LexLeanRuntime.append __fc_0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat __p0)]) (_root_.LexLeanPreservation.Rel (__ff_0 __p0) (_root_.LexLeanTarget.TargetSyntax.Value.bool (transform __p0)))) (values : (List Nat)), _root_.LexLeanPreservation.FunRel __prog 1 [(_root_.LexLeanTarget.TargetSyntax.Value.closure __fi_0 __fc_0), ((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.nat) values)] (_root_.LexLeanPreservation.Rel (__fits_1 transform __ff_0 values) ((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.bool) (Coverage.Types.mapList Nat Bool transform values)))
  | transform, __ff_0, __fi_0, __fc_0, __fh_0, (List.nil) => by rw [__fits_1.eq_def]; exact _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_match (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_build _root_.LexLeanPreservation.convL_nil _root_.LexLeanPreservation.construct_nil)))
  | transform, __ff_0, __fi_0, __fc_0, __fh_0, (List.cons head tail) => by rw [__fits_1.eq_def]; exact _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_match (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convA_miss rfl (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_apply (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__fh_0 (head))) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (__rel_1 (transform) (__ff_0) (__fi_0) (__fc_0) (__fh_0) (tail))) _root_.LexLeanPreservation.convL_nil)) _root_.LexLeanPreservation.construct_cons))))

mutual
def __fits_2 : ∀ (n : Nat), Bool
  | (Nat.zero) => (Bool.true && (Bool.true && Bool.true))
  | (Nat.succ k) => (Bool.true && ((Bool.true && Bool.true) && (__fits_3 (k))))
termination_by structural n => n

def __fits_3 : ∀ (n : Nat), Bool
  | (Nat.zero) => (Bool.true && (Bool.true && Bool.true))
  | (Nat.succ k) => (Bool.true && ((Bool.true && Bool.true) && (__fits_2 (k))))
termination_by structural n => n

end

mutual
theorem __rel_2 : ∀ (n : Nat), _root_.LexLeanPreservation.FunRel __prog 2 [(_root_.LexLeanTarget.TargetSyntax.Value.nat n)] (_root_.LexLeanPreservation.Rel (__fits_2 n) (_root_.LexLeanTarget.TargetSyntax.Value.bool (Coverage.Types.isEven n)))
  | (Nat.zero) => by rw [__fits_2.eq_def]; exact _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_match (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_build _root_.LexLeanPreservation.convL_nil _root_.LexLeanPreservation.construct_true)))
  | (Nat.succ k) => by rw [__fits_2.eq_def]; exact _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_match (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convA_miss rfl (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__rel_3 (k))))))
termination_by structural n => n

theorem __rel_3 : ∀ (n : Nat), _root_.LexLeanPreservation.FunRel __prog 3 [(_root_.LexLeanTarget.TargetSyntax.Value.nat n)] (_root_.LexLeanPreservation.Rel (__fits_3 n) (_root_.LexLeanTarget.TargetSyntax.Value.bool (Coverage.Types.isOdd n)))
  | (Nat.zero) => by rw [__fits_3.eq_def]; exact _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_match (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_build _root_.LexLeanPreservation.convL_nil _root_.LexLeanPreservation.construct_false)))
  | (Nat.succ k) => by rw [__fits_3.eq_def]; exact _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_match (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convA_miss rfl (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__rel_2 (k))))))
termination_by structural n => n

end

def __fits_0 (values : (List Nat)) : Bool :=
  ((Bool.true && (Bool.true && Bool.true)) && (__fits_1 ((Coverage.Types.isEven)) (fun (__p0 : Nat) => __fits_2 __p0) (values)))

attribute [local irreducible] Coverage.Types.mapList in
theorem __rel_0 (values : (List Nat)) : _root_.LexLeanPreservation.FunRel __prog 0 [((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.nat) values)] (_root_.LexLeanPreservation.Rel (__fits_0 values) ((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.bool) (Coverage.Main.mapped values))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_closure _root_.LexLeanPreservation.convL_nil) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (__rel_1 ((Coverage.Types.isEven)) (fun (__p0 : Nat) => __fits_2 __p0) (2) ([]) (fun (__p0 : Nat) => __rel_2 __p0) (values)))

def denote (values : (List Nat)) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 values) (_root_.LexLeanPreservation.Obs.value (((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.bool) (Coverage.Main.mapped values)))) _root_.LexLeanPreservation.Obs.overflow

theorem root (values : (List Nat)) : _root_.LexLeanPreservation.RunConv __prog 0 [((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.nat) values)] (_root_.LexLeanPreservation.Rel (__fits_0 values) ((_root_.LexLeanPreservation.encList _root_.LexLeanTarget.TargetSyntax.Value.bool) (Coverage.Main.mapped values))) :=
  _root_.LexLeanPreservation.run_of_funRel (__rel_0 values)

end LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R19
