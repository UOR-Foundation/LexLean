import LexLeanPreservation.Core
import LexLeanPreservation.Values
import LexLeanPreservation.Primitives
import LexLeanPreservation.Fixed
import LexLeanPreservation.Keys
import LexLeanPreservation.Templates
import LexLeanPreservation.Validate
import Models.Flows
import Models.Glyphs
import Models.Ledger
import Models.Main
import Models.Pipeline
import Models.Policy
import Models.Recognizer
import Models.Session
import Models.Triage
set_option autoImplicit false
set_option maxRecDepth 100000
set_option linter.unusedVariables false
namespace LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R7

def __prog : LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0], types := [.nat], result := (.result .nat (.pair .bool .bool)),
      body := (.«let» 1 .nat (.var 0) (.cond (.call 1 [(.var 1)]) (.«let» 2 .nat (.call 2 [(.var 1)]) (.cond (.call 3 [(.var 1), (.var 2)]) (.build .ok (.result .nat (.pair .bool .bool)) [(.var 2)]) (.build .error (.result .nat (.pair .bool .bool)) [(.build .pair (.pair .bool .bool) [(.build .true .bool []), (.build .false .bool [])])]))) (.build .error (.result .nat (.pair .bool .bool)) [(.build .pair (.pair .bool .bool) [(.build .false .bool []), (.build .false .bool [])])]))) },
    { parameters := [0], types := [.nat], result := .bool,
      body := (.prim .natLe [(.var 0), (.value .nat (.nat 10))]) },
    { parameters := [0], types := [.nat], result := .nat,
      body := (.call 4 [(.var 0)]) },
    { parameters := [0, 1], types := [.nat, .nat], result := .bool,
      body := (.prim .natLe [(.var 1), (.var 0)]) },
    { parameters := [0], types := [.nat], result := .nat,
      body := (.prim .natSub [(.var 0), (.value .nat (.nat 1))]) }] }

def __fits_1 (x : Nat) : Bool :=
  ((true && (true && true)) && true)

theorem __rel_1 (x : Nat) : LexLeanPreservation.FunRel __prog 1 [(LexLeanTarget.TargetSyntax.Value.nat x)] (LexLeanPreservation.Rel (__fits_1 x) (LexLeanTarget.TargetSyntax.Value.bool (Models.Ledger.withinCheck x))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natLe x (10 : Nat)))

def __fits_4 (x : Nat) : Bool :=
  ((true && (true && true)) && true)

theorem __rel_4 (x : Nat) : LexLeanPreservation.FunRel __prog 4 [(LexLeanTarget.TargetSyntax.Value.nat x)] (LexLeanPreservation.Rel (__fits_4 x) (LexLeanTarget.TargetSyntax.Value.nat (Models.Ledger.Guess x))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natSub x (1 : Nat)))

def __fits_2 (__input : Nat) : Bool :=
  ((true && true) && (__fits_4 (__input)))

attribute [local irreducible] Models.Ledger.Guess in
theorem __rel_2 (__input : Nat) : LexLeanPreservation.FunRel __prog 2 [(LexLeanTarget.TargetSyntax.Value.nat __input)] (LexLeanPreservation.Rel (__fits_2 __input) (LexLeanTarget.TargetSyntax.Value.nat (Models.Ledger.GuessModel __input))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_4 (__input)))

def __fits_3 (x : Nat) (y : Nat) : Bool :=
  ((true && (true && true)) && true)

theorem __rel_3 (x : Nat) (y : Nat) : LexLeanPreservation.FunRel __prog 3 [(LexLeanTarget.TargetSyntax.Value.nat x), (LexLeanTarget.TargetSyntax.Value.nat y)] (LexLeanPreservation.Rel (__fits_3 x y) (LexLeanTarget.TargetSyntax.Value.bool (Models.Ledger.belowCheck x y))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natLe y x))

def __fits_0 (x : Nat) : Bool :=
  (true && (let __checked1_input : Nat := x; (((true && true) && (__fits_1 (__checked1_input))) && (if (Models.Ledger.withinCheck (__checked1_input)) then (((true && true) && (__fits_2 (__checked1_input))) && (let __checked1_output : Nat := (Models.Ledger.GuessModel (__checked1_input)); (((true && (true && true)) && (__fits_3 (__checked1_input) (__checked1_output))) && (if (Models.Ledger.belowCheck (__checked1_input) (__checked1_output)) then ((true && true) && true) else (((((true && true) && ((true && true) && true)) && true) && true) && true))))) else (((((true && true) && ((true && true) && true)) && true) && true) && true)))))

attribute [local irreducible] Models.Ledger.GuessModel Models.Ledger.belowCheck Models.Ledger.withinCheck in
theorem __rel_0 (x : Nat) : LexLeanPreservation.FunRel __prog 0 [(LexLeanTarget.TargetSyntax.Value.nat x)] (LexLeanPreservation.Rel (__fits_0 x) ((LexLeanPreservation.encExcept (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.bool LexLeanTarget.TargetSyntax.Value.bool) LexLeanTarget.TargetSyntax.Value.nat) (Models.Main.guessChecked x))) :=
  LexLeanPreservation.funRel_intro rfl rfl (let __checked1_input : Nat := x; LexLeanPreservation.conv_let (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.conv_cond (fun (__b : Bool) => (if __b then (((true && true) && (__fits_2 (__checked1_input))) && (let __checked1_output : Nat := (Models.Ledger.GuessModel (__checked1_input)); (((true && (true && true)) && (__fits_3 (__checked1_input) (__checked1_output))) && (if (Models.Ledger.belowCheck (__checked1_input) (__checked1_output)) then ((true && true) && true) else (((((true && true) && ((true && true) && true)) && true) && true) && true))))) else (((((true && true) && ((true && true) && true)) && true) && true) && true))) (fun (__b : Bool) => (LexLeanPreservation.encExcept (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.bool LexLeanTarget.TargetSyntax.Value.bool) LexLeanTarget.TargetSyntax.Value.nat) (if __b then (let __checked1_output : Nat := (Models.Ledger.GuessModel (__checked1_input)); (if (Models.Ledger.belowCheck (__checked1_input) (__checked1_output)) then (Except.ok (__checked1_output) : (Except (Prod Bool Bool) Nat)) else (Except.error (((true, false) : (Prod Bool Bool))) : (Except (Prod Bool Bool) Nat)))) else (Except.error (((false, false) : (Prod Bool Bool))) : (Except (Prod Bool Bool) Nat)))) (Models.Ledger.withinCheck (__checked1_input)) (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_1 (__checked1_input))) (fun _ => (let __checked1_output : Nat := (Models.Ledger.GuessModel (__checked1_input)); LexLeanPreservation.conv_let (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_2 (__checked1_input))) (LexLeanPreservation.conv_cond (fun (__b : Bool) => (if __b then ((true && true) && true) else (((((true && true) && ((true && true) && true)) && true) && true) && true))) (fun (__b : Bool) => (LexLeanPreservation.encExcept (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.bool LexLeanTarget.TargetSyntax.Value.bool) LexLeanTarget.TargetSyntax.Value.nat) (if __b then (Except.ok (__checked1_output) : (Except (Prod Bool Bool) Nat)) else (Except.error (((true, false) : (Prod Bool Bool))) : (Except (Prod Bool Bool) Nat)))) (Models.Ledger.belowCheck (__checked1_input) (__checked1_output)) (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (__rel_3 (__checked1_input) (__checked1_output))) (fun _ => (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) LexLeanPreservation.construct_ok)) (fun _ => (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_true) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_false) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair) LexLeanPreservation.convL_nil) LexLeanPreservation.construct_error))))) (fun _ => (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_false) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_false) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair) LexLeanPreservation.convL_nil) LexLeanPreservation.construct_error))))

def denote (x : Nat) : LexLeanPreservation.Obs :=
  cond (__fits_0 x) (LexLeanPreservation.Obs.value (((LexLeanPreservation.encExcept (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.bool LexLeanTarget.TargetSyntax.Value.bool) LexLeanTarget.TargetSyntax.Value.nat) (Models.Main.guessChecked x)))) LexLeanPreservation.Obs.overflow

theorem root (x : Nat) : LexLeanPreservation.RunConv __prog 0 [(LexLeanTarget.TargetSyntax.Value.nat x)] (LexLeanPreservation.Rel (__fits_0 x) ((LexLeanPreservation.encExcept (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.bool LexLeanTarget.TargetSyntax.Value.bool) LexLeanTarget.TargetSyntax.Value.nat) (Models.Main.guessChecked x))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 x)

end LexLeanPreserve.C734e662d06f5339c113e042e805b8ef8.R7
