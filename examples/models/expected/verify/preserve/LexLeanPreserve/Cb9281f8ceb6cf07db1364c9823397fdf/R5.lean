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
namespace LexLeanPreserve.Cb9281f8ceb6cf07db1364c9823397fdf.R5

def __prog : LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [.nat, .nat], result := (.result (.pair .nat .nat) (.pair .bool .bool)),
      body := (.«let» 2 .nat (.var 0) (.«let» 3 .nat (.var 1) (.cond (.call 1 [(.var 2)]) (.cond (.call 2 [(.var 2), (.var 3)]) (.«let» 4 (.pair .nat .nat) (.call 3 [(.var 2), (.var 3)]) (.cond (.call 1 [(.first (.var 4))]) (.build .ok (.result (.pair .nat .nat) (.pair .bool .bool)) [(.var 4)]) (.build .error (.result (.pair .nat .nat) (.pair .bool .bool)) [(.build .pair (.pair .bool .bool) [(.build .true .bool []), (.build .true .bool [])])]))) (.build .error (.result (.pair .nat .nat) (.pair .bool .bool)) [(.build .pair (.pair .bool .bool) [(.build .false .bool []), (.build .false .bool [])])])) (.build .error (.result (.pair .nat .nat) (.pair .bool .bool)) [(.build .pair (.pair .bool .bool) [(.build .false .bool []), (.build .true .bool [])])])))) },
    { parameters := [0], types := [.nat], result := .bool,
      body := (.prim .natLe [(.var 0), (.value .nat (.nat 100))]) },
    { parameters := [0, 1], types := [.nat, .nat], result := .bool,
      body := (.prim .natLe [(.var 1), (.value .nat (.nat 10))]) },
    { parameters := [0, 1], types := [.nat, .nat], result := (.pair .nat .nat),
      body := (.call 4 [(.var 0), (.var 1)]) },
    { parameters := [0, 1], types := [.nat, .nat], result := (.pair .nat .nat),
      body := (.build .pair (.pair .nat .nat) [(.prim .natAdd [(.var 0), (.var 1)]), (.var 1)]) }] }

def __fits_1 (s : Nat) : Bool :=
  ((true && (true && true)) && true)

theorem __rel_1 (s : Nat) : LexLeanPreservation.FunRel __prog 1 [(LexLeanTarget.TargetSyntax.Value.nat s)] (LexLeanPreservation.Rel (__fits_1 s) (LexLeanTarget.TargetSyntax.Value.bool (Models.Ledger.cappedCheck s))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natLe s (100 : Nat)))

def __fits_2 (s : Nat) (x : Nat) : Bool :=
  ((true && (true && true)) && true)

theorem __rel_2 (s : Nat) (x : Nat) : LexLeanPreservation.FunRel __prog 2 [(LexLeanTarget.TargetSyntax.Value.nat s), (LexLeanTarget.TargetSyntax.Value.nat x)] (LexLeanPreservation.Rel (__fits_2 s x) (LexLeanTarget.TargetSyntax.Value.bool (Models.Ledger.smallCheck s x))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natLe x (10 : Nat)))

def __fits_4 (s : Nat) (x : Nat) : Bool :=
  ((((true && (true && true)) && (Nat.blt (s + x) 18446744073709551616)) && (true && true)) && true)

theorem __rel_4 (s : Nat) (x : Nat) : LexLeanPreservation.FunRel __prog 4 [(LexLeanTarget.TargetSyntax.Value.nat s), (LexLeanTarget.TargetSyntax.Value.nat x)] (LexLeanPreservation.Rel (__fits_4 s x) ((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.nat) (Models.Ledger.Spill s x))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd s x)) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair)

def __fits_3 (__state : Nat) (__input : Nat) : Bool :=
  ((true && (true && true)) && (__fits_4 (__state) (__input)))

attribute [local irreducible] Models.Ledger.Spill in
theorem __rel_3 (__state : Nat) (__input : Nat) : LexLeanPreservation.FunRel __prog 3 [(LexLeanTarget.TargetSyntax.Value.nat __state), (LexLeanTarget.TargetSyntax.Value.nat __input)] (LexLeanPreservation.Rel (__fits_3 __state __input) ((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.nat) (Models.Ledger.SpillModel __state __input))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (__rel_4 (__state) (__input)))

def __fits_0 (s : Nat) (x : Nat) : Bool :=
  (true && (let __checked1_state : Nat := s; (true && (let __checked1_input : Nat := x; (((true && true) && (__fits_1 (__checked1_state))) && (if (Models.Ledger.cappedCheck (__checked1_state)) then (((true && (true && true)) && (__fits_2 (__checked1_state) (__checked1_input))) && (if (Models.Ledger.smallCheck (__checked1_state) (__checked1_input)) then (((true && (true && true)) && (__fits_3 (__checked1_state) (__checked1_input))) && (let __checked1_step : (Prod Nat Nat) := (Models.Ledger.SpillModel (__checked1_state) (__checked1_input)); (((true && true) && (__fits_1 ((__checked1_step).1))) && (if (Models.Ledger.cappedCheck ((__checked1_step).1)) then ((true && true) && true) else (((((true && true) && ((true && true) && true)) && true) && true) && true))))) else (((((true && true) && ((true && true) && true)) && true) && true) && true))) else (((((true && true) && ((true && true) && true)) && true) && true) && true)))))))

attribute [local irreducible] Models.Ledger.SpillModel Models.Ledger.cappedCheck Models.Ledger.smallCheck in
theorem __rel_0 (s : Nat) (x : Nat) : LexLeanPreservation.FunRel __prog 0 [(LexLeanTarget.TargetSyntax.Value.nat s), (LexLeanTarget.TargetSyntax.Value.nat x)] (LexLeanPreservation.Rel (__fits_0 s x) ((LexLeanPreservation.encExcept (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.bool LexLeanTarget.TargetSyntax.Value.bool) (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.nat)) (Models.Main.ledgerPost s x))) :=
  LexLeanPreservation.funRel_intro rfl rfl (let __checked1_state : Nat := s; LexLeanPreservation.conv_let (LexLeanPreservation.conv_var rfl) (let __checked1_input : Nat := x; LexLeanPreservation.conv_let (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.conv_cond (fun (__b : Bool) => (if __b then (((true && (true && true)) && (__fits_2 (__checked1_state) (__checked1_input))) && (if (Models.Ledger.smallCheck (__checked1_state) (__checked1_input)) then (((true && (true && true)) && (__fits_3 (__checked1_state) (__checked1_input))) && (let __checked1_step : (Prod Nat Nat) := (Models.Ledger.SpillModel (__checked1_state) (__checked1_input)); (((true && true) && (__fits_1 ((__checked1_step).1))) && (if (Models.Ledger.cappedCheck ((__checked1_step).1)) then ((true && true) && true) else (((((true && true) && ((true && true) && true)) && true) && true) && true))))) else (((((true && true) && ((true && true) && true)) && true) && true) && true))) else (((((true && true) && ((true && true) && true)) && true) && true) && true))) (fun (__b : Bool) => (LexLeanPreservation.encExcept (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.bool LexLeanTarget.TargetSyntax.Value.bool) (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.nat)) (if __b then (if (Models.Ledger.smallCheck (__checked1_state) (__checked1_input)) then (let __checked1_step : (Prod Nat Nat) := (Models.Ledger.SpillModel (__checked1_state) (__checked1_input)); (if (Models.Ledger.cappedCheck ((__checked1_step).1)) then (Except.ok (__checked1_step) : (Except (Prod Bool Bool) (Prod Nat Nat))) else (Except.error (((true, true) : (Prod Bool Bool))) : (Except (Prod Bool Bool) (Prod Nat Nat))))) else (Except.error (((false, false) : (Prod Bool Bool))) : (Except (Prod Bool Bool) (Prod Nat Nat)))) else (Except.error (((false, true) : (Prod Bool Bool))) : (Except (Prod Bool Bool) (Prod Nat Nat))))) (Models.Ledger.cappedCheck (__checked1_state)) (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_1 (__checked1_state))) (fun _ => (LexLeanPreservation.conv_cond (fun (__b : Bool) => (if __b then (((true && (true && true)) && (__fits_3 (__checked1_state) (__checked1_input))) && (let __checked1_step : (Prod Nat Nat) := (Models.Ledger.SpillModel (__checked1_state) (__checked1_input)); (((true && true) && (__fits_1 ((__checked1_step).1))) && (if (Models.Ledger.cappedCheck ((__checked1_step).1)) then ((true && true) && true) else (((((true && true) && ((true && true) && true)) && true) && true) && true))))) else (((((true && true) && ((true && true) && true)) && true) && true) && true))) (fun (__b : Bool) => (LexLeanPreservation.encExcept (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.bool LexLeanTarget.TargetSyntax.Value.bool) (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.nat)) (if __b then (let __checked1_step : (Prod Nat Nat) := (Models.Ledger.SpillModel (__checked1_state) (__checked1_input)); (if (Models.Ledger.cappedCheck ((__checked1_step).1)) then (Except.ok (__checked1_step) : (Except (Prod Bool Bool) (Prod Nat Nat))) else (Except.error (((true, true) : (Prod Bool Bool))) : (Except (Prod Bool Bool) (Prod Nat Nat))))) else (Except.error (((false, false) : (Prod Bool Bool))) : (Except (Prod Bool Bool) (Prod Nat Nat))))) (Models.Ledger.smallCheck (__checked1_state) (__checked1_input)) (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (__rel_2 (__checked1_state) (__checked1_input))) (fun _ => (let __checked1_step : (Prod Nat Nat) := (Models.Ledger.SpillModel (__checked1_state) (__checked1_input)); LexLeanPreservation.conv_let (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (__rel_3 (__checked1_state) (__checked1_input))) (LexLeanPreservation.conv_cond (fun (__b : Bool) => (if __b then ((true && true) && true) else (((((true && true) && ((true && true) && true)) && true) && true) && true))) (fun (__b : Bool) => (LexLeanPreservation.encExcept (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.bool LexLeanTarget.TargetSyntax.Value.bool) (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.nat)) (if __b then (Except.ok (__checked1_step) : (Except (Prod Bool Bool) (Prod Nat Nat))) else (Except.error (((true, true) : (Prod Bool Bool))) : (Except (Prod Bool Bool) (Prod Nat Nat))))) (Models.Ledger.cappedCheck ((__checked1_step).1)) (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_first (LexLeanPreservation.conv_var rfl)) LexLeanPreservation.convL_nil) (__rel_1 ((__checked1_step).1))) (fun _ => (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) LexLeanPreservation.construct_ok)) (fun _ => (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_true) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_true) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair) LexLeanPreservation.convL_nil) LexLeanPreservation.construct_error))))) (fun _ => (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_false) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_false) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair) LexLeanPreservation.convL_nil) LexLeanPreservation.construct_error)))) (fun _ => (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_false) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_true) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair) LexLeanPreservation.convL_nil) LexLeanPreservation.construct_error)))))

def denote (s : Nat) (x : Nat) : LexLeanPreservation.Obs :=
  LexLeanPreservation.Rel (__fits_0 s x) ((LexLeanPreservation.encExcept (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.bool LexLeanTarget.TargetSyntax.Value.bool) (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.nat)) (Models.Main.ledgerPost s x))

theorem root (s : Nat) (x : Nat) : LexLeanPreservation.RunConv __prog 0 [(LexLeanTarget.TargetSyntax.Value.nat s), (LexLeanTarget.TargetSyntax.Value.nat x)] (LexLeanPreservation.Rel (__fits_0 s x) ((LexLeanPreservation.encExcept (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.bool LexLeanTarget.TargetSyntax.Value.bool) (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.nat)) (Models.Main.ledgerPost s x))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 s x)

end LexLeanPreserve.Cb9281f8ceb6cf07db1364c9823397fdf.R5
