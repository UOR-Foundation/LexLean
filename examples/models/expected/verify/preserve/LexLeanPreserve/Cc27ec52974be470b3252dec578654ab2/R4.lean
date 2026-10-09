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
namespace LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R4

def __prog : LexLeanTarget.TargetSyntax.Program :=
  { adts := [{ constructors := [[.nat, .nat]] }], functions := [
    { parameters := [0, 1], types := [(.adt 0), .nat], result := (.result (.pair (.adt 0) .nat) (.pair .bool .bool)),
      body := (.«let» 2 (.adt 0) (.var 0) (.«let» 3 .nat (.var 1) (.cond (.call 1 [(.var 2)]) (.cond (.call 2 [(.var 2), (.var 3)]) (.«let» 4 (.pair (.adt 0) .nat) (.call 3 [(.var 2), (.var 3)]) (.build .ok (.result (.pair (.adt 0) .nat) (.pair .bool .bool)) [(.var 4)])) (.build .error (.result (.pair (.adt 0) .nat) (.pair .bool .bool)) [(.build .pair (.pair .bool .bool) [(.build .false .bool []), (.build .false .bool [])])])) (.build .error (.result (.pair (.adt 0) .nat) (.pair .bool .bool)) [(.build .pair (.pair .bool .bool) [(.build .false .bool []), (.build .true .bool [])])])))) },
    { parameters := [0], types := [(.adt 0)], result := .bool,
      body := (.prim .natLe [(.field (.var 0) 0), (.value .nat (.nat 64))]) },
    { parameters := [0, 1], types := [(.adt 0), .nat], result := .bool,
      body := (.prim .natLe [(.var 1), (.value .nat (.nat 16))]) },
    { parameters := [0, 1], types := [(.adt 0), .nat], result := (.pair (.adt 0) .nat),
      body := (.call 4 [(.var 0), (.var 1)]) },
    { parameters := [0, 1], types := [(.adt 0), .nat], result := (.pair (.adt 0) .nat),
      body := (.«let» 2 .nat (.prim .natAdd [(.field (.var 0) 0), (.var 1)]) (.build .pair (.pair (.adt 0) .nat) [(.build (.adt 0) (.adt 0) [(.prim .natSub [(.var 2), (.prim .natSub [(.var 2), (.value .nat (.nat 64))])]), (.prim .natAdd [(.field (.var 0) 1), (.value .nat (.nat 1))])]), (.prim .natSub [(.var 2), (.value .nat (.nat 64))])])) }] }

def __enc_0 (__s : (Models.Session.Window)) : LexLeanTarget.TargetSyntax.Value :=
  LexLeanTarget.TargetSyntax.Value.adt 0 [(LexLeanTarget.TargetSyntax.Value.nat (__s).used), (LexLeanTarget.TargetSyntax.Value.nat (__s).items)]

def __fits_1 (window : (Models.Session.Window)) : Bool :=
  ((Bool.true && (Bool.true && Bool.true)) && Bool.true)

theorem __rel_1 (window : (Models.Session.Window)) : LexLeanPreservation.FunRel __prog 1 [(__enc_0 window)] (LexLeanPreservation.Rel (__fits_1 window) (LexLeanTarget.TargetSyntax.Value.bool (Models.Session.boundedCheck window))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_field (LexLeanPreservation.conv_var rfl) rfl) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natLe (window).used (64 : Nat)))

def __fits_2 (window : (Models.Session.Window)) (cost : Nat) : Bool :=
  ((Bool.true && (Bool.true && Bool.true)) && Bool.true)

theorem __rel_2 (window : (Models.Session.Window)) (cost : Nat) : LexLeanPreservation.FunRel __prog 2 [(__enc_0 window), (LexLeanTarget.TargetSyntax.Value.nat cost)] (LexLeanPreservation.Rel (__fits_2 window cost) (LexLeanTarget.TargetSyntax.Value.bool (Models.Session.affordableCheck window cost))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natLe cost (16 : Nat)))

def __fits_4 (window : (Models.Session.Window)) (cost : Nat) : Bool :=
  (((Bool.true && (Bool.true && Bool.true)) && (Nat.blt ((window).used + cost) 18446744073709551616)) && (let total : Nat := ((window).used + cost); ((((((Bool.true && (((Bool.true && (Bool.true && Bool.true)) && Bool.true) && Bool.true)) && Bool.true) && (((Bool.true && (Bool.true && Bool.true)) && (Nat.blt ((window).items + (1 : Nat)) 18446744073709551616)) && Bool.true)) && Bool.true) && (((Bool.true && (Bool.true && Bool.true)) && Bool.true) && Bool.true)) && Bool.true)))

theorem __rel_4 (window : (Models.Session.Window)) (cost : Nat) : LexLeanPreservation.FunRel __prog 4 [(__enc_0 window), (LexLeanTarget.TargetSyntax.Value.nat cost)] (LexLeanPreservation.Rel (__fits_4 window cost) ((LexLeanPreservation.encPair __enc_0 LexLeanTarget.TargetSyntax.Value.nat) (Models.Session.SessionStep window cost))) :=
  LexLeanPreservation.funRel_intro rfl rfl (let total : Nat := ((window).used + cost); LexLeanPreservation.conv_let (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_field (LexLeanPreservation.conv_var rfl) rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd (window).used cost)) (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natSub total (64 : Nat))) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natSub total (Models.Session.LexLeanRuntime.subtract (total) ((64 : Nat)) : Nat))) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_field (LexLeanPreservation.conv_var rfl) rfl) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd (window).items (1 : Nat))) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_adt) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natSub total (64 : Nat))) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair))

def __fits_3 (__state : (Models.Session.Window)) (__input : Nat) : Bool :=
  ((Bool.true && (Bool.true && Bool.true)) && (__fits_4 (__state) (__input)))

attribute [local irreducible] Models.Session.SessionStep in
theorem __rel_3 (__state : (Models.Session.Window)) (__input : Nat) : LexLeanPreservation.FunRel __prog 3 [(__enc_0 __state), (LexLeanTarget.TargetSyntax.Value.nat __input)] (LexLeanPreservation.Rel (__fits_3 __state __input) ((LexLeanPreservation.encPair __enc_0 LexLeanTarget.TargetSyntax.Value.nat) (Models.Session.SessionModel __state __input))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (__rel_4 (__state) (__input)))

def __fits_0 (window : (Models.Session.Window)) (cost : Nat) : Bool :=
  (Bool.true && (let __checked1_state : (Models.Session.Window) := window; (Bool.true && (let __checked1_input : Nat := cost; (((Bool.true && Bool.true) && (__fits_1 (__checked1_state))) && (if (Models.Session.boundedCheck (__checked1_state)) then (((Bool.true && (Bool.true && Bool.true)) && (__fits_2 (__checked1_state) (__checked1_input))) && (if (Models.Session.affordableCheck (__checked1_state) (__checked1_input)) then (((Bool.true && (Bool.true && Bool.true)) && (__fits_3 (__checked1_state) (__checked1_input))) && (let __checked1_step : (Prod (Models.Session.Window) Nat) := (Models.Session.SessionModel (__checked1_state) (__checked1_input)); ((Bool.true && Bool.true) && Bool.true))) else (((((Bool.true && Bool.true) && ((Bool.true && Bool.true) && Bool.true)) && Bool.true) && Bool.true) && Bool.true))) else (((((Bool.true && Bool.true) && ((Bool.true && Bool.true) && Bool.true)) && Bool.true) && Bool.true) && Bool.true)))))))

attribute [local irreducible] Models.Session.SessionModel Models.Session.affordableCheck Models.Session.boundedCheck in
theorem __rel_0 (window : (Models.Session.Window)) (cost : Nat) : LexLeanPreservation.FunRel __prog 0 [(__enc_0 window), (LexLeanTarget.TargetSyntax.Value.nat cost)] (LexLeanPreservation.Rel (__fits_0 window cost) ((LexLeanPreservation.encExcept (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.bool LexLeanTarget.TargetSyntax.Value.bool) (LexLeanPreservation.encPair __enc_0 LexLeanTarget.TargetSyntax.Value.nat)) (Models.Main.step window cost))) :=
  LexLeanPreservation.funRel_intro rfl rfl (let __checked1_state : (Models.Session.Window) := window; LexLeanPreservation.conv_let (LexLeanPreservation.conv_var rfl) (let __checked1_input : Nat := cost; LexLeanPreservation.conv_let (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.conv_cond (fun (__b : Bool) => (if __b then (((Bool.true && (Bool.true && Bool.true)) && (__fits_2 (__checked1_state) (__checked1_input))) && (if (Models.Session.affordableCheck (__checked1_state) (__checked1_input)) then (((Bool.true && (Bool.true && Bool.true)) && (__fits_3 (__checked1_state) (__checked1_input))) && (let __checked1_step : (Prod (Models.Session.Window) Nat) := (Models.Session.SessionModel (__checked1_state) (__checked1_input)); ((Bool.true && Bool.true) && Bool.true))) else (((((Bool.true && Bool.true) && ((Bool.true && Bool.true) && Bool.true)) && Bool.true) && Bool.true) && Bool.true))) else (((((Bool.true && Bool.true) && ((Bool.true && Bool.true) && Bool.true)) && Bool.true) && Bool.true) && Bool.true))) (fun (__b : Bool) => (LexLeanPreservation.encExcept (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.bool LexLeanTarget.TargetSyntax.Value.bool) (LexLeanPreservation.encPair __enc_0 LexLeanTarget.TargetSyntax.Value.nat)) (if __b then (if (Models.Session.affordableCheck (__checked1_state) (__checked1_input)) then (let __checked1_step : (Prod (Models.Session.Window) Nat) := (Models.Session.SessionModel (__checked1_state) (__checked1_input)); (Except.ok (__checked1_step) : (Except (Prod Bool Bool) (Prod (Models.Session.Window) Nat)))) else (Except.error (((Bool.false, Bool.false) : (Prod Bool Bool))) : (Except (Prod Bool Bool) (Prod (Models.Session.Window) Nat)))) else (Except.error (((Bool.false, Bool.true) : (Prod Bool Bool))) : (Except (Prod Bool Bool) (Prod (Models.Session.Window) Nat))))) (Models.Session.boundedCheck (__checked1_state)) (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) (__rel_1 (__checked1_state))) (fun _ => (LexLeanPreservation.conv_cond (fun (__b : Bool) => (if __b then (((Bool.true && (Bool.true && Bool.true)) && (__fits_3 (__checked1_state) (__checked1_input))) && (let __checked1_step : (Prod (Models.Session.Window) Nat) := (Models.Session.SessionModel (__checked1_state) (__checked1_input)); ((Bool.true && Bool.true) && Bool.true))) else (((((Bool.true && Bool.true) && ((Bool.true && Bool.true) && Bool.true)) && Bool.true) && Bool.true) && Bool.true))) (fun (__b : Bool) => (LexLeanPreservation.encExcept (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.bool LexLeanTarget.TargetSyntax.Value.bool) (LexLeanPreservation.encPair __enc_0 LexLeanTarget.TargetSyntax.Value.nat)) (if __b then (let __checked1_step : (Prod (Models.Session.Window) Nat) := (Models.Session.SessionModel (__checked1_state) (__checked1_input)); (Except.ok (__checked1_step) : (Except (Prod Bool Bool) (Prod (Models.Session.Window) Nat)))) else (Except.error (((Bool.false, Bool.false) : (Prod Bool Bool))) : (Except (Prod Bool Bool) (Prod (Models.Session.Window) Nat))))) (Models.Session.affordableCheck (__checked1_state) (__checked1_input)) (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (__rel_2 (__checked1_state) (__checked1_input))) (fun _ => (let __checked1_step : (Prod (Models.Session.Window) Nat) := (Models.Session.SessionModel (__checked1_state) (__checked1_input)); LexLeanPreservation.conv_let (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) (__rel_3 (__checked1_state) (__checked1_input))) (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil) LexLeanPreservation.construct_ok))) (fun _ => (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_false) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_false) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair) LexLeanPreservation.convL_nil) LexLeanPreservation.construct_error)))) (fun _ => (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_false) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_true) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair) LexLeanPreservation.convL_nil) LexLeanPreservation.construct_error)))))

def denote (window : (Models.Session.Window)) (cost : Nat) : LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 window cost) (LexLeanPreservation.Obs.value (((LexLeanPreservation.encExcept (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.bool LexLeanTarget.TargetSyntax.Value.bool) (LexLeanPreservation.encPair __enc_0 LexLeanTarget.TargetSyntax.Value.nat)) (Models.Main.step window cost)))) LexLeanPreservation.Obs.overflow

theorem root (window : (Models.Session.Window)) (cost : Nat) : LexLeanPreservation.RunConv __prog 0 [(__enc_0 window), (LexLeanTarget.TargetSyntax.Value.nat cost)] (LexLeanPreservation.Rel (__fits_0 window cost) ((LexLeanPreservation.encExcept (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.bool LexLeanTarget.TargetSyntax.Value.bool) (LexLeanPreservation.encPair __enc_0 LexLeanTarget.TargetSyntax.Value.nat)) (Models.Main.step window cost))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 window cost)

end LexLeanPreserve.Cc27ec52974be470b3252dec578654ab2.R4
