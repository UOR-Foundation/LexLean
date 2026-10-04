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
namespace LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R19

def __prog : LexLeanTarget.TargetSyntax.Program :=
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
  | transform, __ff_0, (List.nil) => (true && (true && true))
  | transform, __ff_0, (List.cons head tail) => (true && (((true && ((true && true) && (__ff_0 (head)))) && (((true && (true && true)) && (__fits_1 (transform) (__ff_0) (tail))) && true)) && true))

theorem __rel_1 : ∀ (transform : (Nat -> Bool)) (__ff_0 : Nat -> Bool) (__fi_0 : Nat) (__fc_0 : List LexLeanTarget.TargetSyntax.Value) (__fh_0 : ∀ (__p0 : Nat), LexLeanPreservation.FunRel __prog __fi_0 (LexLeanTarget.TargetSemantics.LexLeanRuntime.append __fc_0 [(LexLeanTarget.TargetSyntax.Value.nat __p0)]) (LexLeanPreservation.Rel (__ff_0 __p0) (LexLeanTarget.TargetSyntax.Value.bool (transform __p0)))) (values : (List Nat)), LexLeanPreservation.FunRel __prog 1 [(LexLeanTarget.TargetSyntax.Value.closure __fi_0 __fc_0), ((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) values)] (LexLeanPreservation.Rel (__fits_1 transform __ff_0 values) ((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.bool) (Coverage.Types.mapList Nat Bool transform values)))
  | transform, __ff_0, __fi_0, __fc_0, __fh_0, (List.nil) => by rw [__fits_1.eq_def]; exact LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_nil)))
  | transform, __ff_0, __fi_0, __fc_0, __fh_0, (List.cons head tail) => by rw [__fits_1.eq_def]; exact LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_apply (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__fh_0 (head))) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (__rel_1 (transform) (__ff_0) (__fi_0) (__fc_0) (__fh_0) (tail))) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_cons))))

mutual
def __fits_2 : ∀ (n : Nat), Bool
  | (Nat.zero) => (true && (true && true))
  | (Nat.succ k) => (true && ((true && true) && (__fits_3 (k))))
termination_by structural n => n

def __fits_3 : ∀ (n : Nat), Bool
  | (Nat.zero) => (true && (true && true))
  | (Nat.succ k) => (true && ((true && true) && (__fits_2 (k))))
termination_by structural n => n

end

mutual
theorem __rel_2 : ∀ (n : Nat), LexLeanPreservation.FunRel __prog 2 [(LexLeanTarget.TargetSyntax.Value.nat n)] (LexLeanPreservation.Rel (__fits_2 n) (LexLeanTarget.TargetSyntax.Value.bool (Coverage.Types.isEven n)))
  | (Nat.zero) => by rw [__fits_2.eq_def]; exact LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_true)))
  | (Nat.succ k) => by rw [__fits_2.eq_def]; exact LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_3 (k))))))
termination_by structural n => n

theorem __rel_3 : ∀ (n : Nat), LexLeanPreservation.FunRel __prog 3 [(LexLeanTarget.TargetSyntax.Value.nat n)] (LexLeanPreservation.Rel (__fits_3 n) (LexLeanTarget.TargetSyntax.Value.bool (Coverage.Types.isOdd n)))
  | (Nat.zero) => by rw [__fits_3.eq_def]; exact LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_false)))
  | (Nat.succ k) => by rw [__fits_3.eq_def]; exact LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_match (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convA_miss rfl (LexLeanPreservation.convA_hit rfl rfl (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_2 (k))))))
termination_by structural n => n

end

def __fits_0 (values : (List Nat)) : Bool :=
  ((true && (true && true)) && (__fits_1 ((Coverage.Types.isEven)) (fun (__p0 : Nat) => __fits_2 __p0) (values)))

attribute [local irreducible] Coverage.Types.mapList in
theorem __rel_0 (values : (List Nat)) : LexLeanPreservation.FunRel __prog 0 [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) values)] (LexLeanPreservation.Rel (__fits_0 values) ((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.bool) (Coverage.Main.mapped values))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_closure LexLeanPreservation.convL_nil) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (__rel_1 ((Coverage.Types.isEven)) (fun (__p0 : Nat) => __fits_2 __p0) (2) ([]) (fun (__p0 : Nat) => __rel_2 __p0) (values)))

def denote (values : (List Nat)) : LexLeanPreservation.Obs :=
  cond (__fits_0 values) (LexLeanPreservation.Obs.value (((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.bool) (Coverage.Main.mapped values)))) LexLeanPreservation.Obs.overflow

theorem root (values : (List Nat)) : LexLeanPreservation.RunConv __prog 0 [((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.nat) values)] (LexLeanPreservation.Rel (__fits_0 values) ((LexLeanPreservation.encList LexLeanTarget.TargetSyntax.Value.bool) (Coverage.Main.mapped values))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 values)

end LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R19
