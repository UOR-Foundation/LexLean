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
namespace LexLeanPreserve.Cd0cbc34d95d16582f4cc8f373aaea24b.R7

def __prog : _root_.LexLeanTarget.TargetSyntax.Program :=
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
  ((Bool.true && (Bool.true && Bool.true)) && Bool.true)

theorem __rel_1 (x : Nat) : _root_.LexLeanPreservation.FunRel __prog 1 [(_root_.LexLeanTarget.TargetSyntax.Value.nat x)] (_root_.LexLeanPreservation.Rel (__fits_1 x) (_root_.LexLeanTarget.TargetSyntax.Value.bool (Models.Ledger.withinCheck x))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natLe x (10 : Nat)))

def __fits_4 (x : Nat) : Bool :=
  ((Bool.true && (Bool.true && Bool.true)) && Bool.true)

theorem __rel_4 (x : Nat) : _root_.LexLeanPreservation.FunRel __prog 4 [(_root_.LexLeanTarget.TargetSyntax.Value.nat x)] (_root_.LexLeanPreservation.Rel (__fits_4 x) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Models.Ledger.Guess x))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natSub x (1 : Nat)))

def __fits_2 (_yinput : Nat) : Bool :=
  ((Bool.true && Bool.true) && (__fits_4 (_yinput)))

attribute [local irreducible] Models.Ledger.Guess in
theorem __rel_2 (_yinput : Nat) : _root_.LexLeanPreservation.FunRel __prog 2 [(_root_.LexLeanTarget.TargetSyntax.Value.nat _yinput)] (_root_.LexLeanPreservation.Rel (__fits_2 _yinput) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Models.Ledger.GuessModel _yinput))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__rel_4 (_yinput)))

def __fits_3 (x : Nat) (y : Nat) : Bool :=
  ((Bool.true && (Bool.true && Bool.true)) && Bool.true)

theorem __rel_3 (x : Nat) (y : Nat) : _root_.LexLeanPreservation.FunRel __prog 3 [(_root_.LexLeanTarget.TargetSyntax.Value.nat x), (_root_.LexLeanTarget.TargetSyntax.Value.nat y)] (_root_.LexLeanPreservation.Rel (__fits_3 x y) (_root_.LexLeanTarget.TargetSyntax.Value.bool (Models.Ledger.belowCheck x y))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natLe y x))

def __fits_0 (x : Nat) : Bool :=
  (Bool.true && (let _ychecked1_input : Nat := x; (((Bool.true && Bool.true) && (__fits_1 (_ychecked1_input))) && (if (Models.Ledger.withinCheck (_ychecked1_input)) then (((Bool.true && Bool.true) && (__fits_2 (_ychecked1_input))) && (let _ychecked1_output : Nat := (Models.Ledger.GuessModel (_ychecked1_input)); (((Bool.true && (Bool.true && Bool.true)) && (__fits_3 (_ychecked1_input) (_ychecked1_output))) && (if (Models.Ledger.belowCheck (_ychecked1_input) (_ychecked1_output)) then ((Bool.true && Bool.true) && Bool.true) else (((((Bool.true && Bool.true) && ((Bool.true && Bool.true) && Bool.true)) && Bool.true) && Bool.true) && Bool.true))))) else (((((Bool.true && Bool.true) && ((Bool.true && Bool.true) && Bool.true)) && Bool.true) && Bool.true) && Bool.true)))))

attribute [local irreducible] Models.Ledger.GuessModel Models.Ledger.belowCheck Models.Ledger.withinCheck in
theorem __rel_0 (x : Nat) : _root_.LexLeanPreservation.FunRel __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat x)] (_root_.LexLeanPreservation.Rel (__fits_0 x) ((_root_.LexLeanPreservation.encExcept (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.bool _root_.LexLeanTarget.TargetSyntax.Value.bool) _root_.LexLeanTarget.TargetSyntax.Value.nat) (Models.Main.guessChecked x))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (let _ychecked1_input : Nat := x; _root_.LexLeanPreservation.conv_let (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.conv_cond (fun (__b : Bool) => (if __b then (((Bool.true && Bool.true) && (__fits_2 (_ychecked1_input))) && (let _ychecked1_output : Nat := (Models.Ledger.GuessModel (_ychecked1_input)); (((Bool.true && (Bool.true && Bool.true)) && (__fits_3 (_ychecked1_input) (_ychecked1_output))) && (if (Models.Ledger.belowCheck (_ychecked1_input) (_ychecked1_output)) then ((Bool.true && Bool.true) && Bool.true) else (((((Bool.true && Bool.true) && ((Bool.true && Bool.true) && Bool.true)) && Bool.true) && Bool.true) && Bool.true))))) else (((((Bool.true && Bool.true) && ((Bool.true && Bool.true) && Bool.true)) && Bool.true) && Bool.true) && Bool.true))) (fun (__b : Bool) => (_root_.LexLeanPreservation.encExcept (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.bool _root_.LexLeanTarget.TargetSyntax.Value.bool) _root_.LexLeanTarget.TargetSyntax.Value.nat) (if __b then (let _ychecked1_output : Nat := (Models.Ledger.GuessModel (_ychecked1_input)); (if (Models.Ledger.belowCheck (_ychecked1_input) (_ychecked1_output)) then (Except.ok (_ychecked1_output) : (Except (Prod Bool Bool) Nat)) else (Except.error (((Bool.true, Bool.false) : (Prod Bool Bool))) : (Except (Prod Bool Bool) Nat)))) else (Except.error (((Bool.false, Bool.false) : (Prod Bool Bool))) : (Except (Prod Bool Bool) Nat)))) (Models.Ledger.withinCheck (_ychecked1_input)) (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__rel_1 (_ychecked1_input))) (fun _ => (let _ychecked1_output : Nat := (Models.Ledger.GuessModel (_ychecked1_input)); _root_.LexLeanPreservation.conv_let (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (__rel_2 (_ychecked1_input))) (_root_.LexLeanPreservation.conv_cond (fun (__b : Bool) => (if __b then ((Bool.true && Bool.true) && Bool.true) else (((((Bool.true && Bool.true) && ((Bool.true && Bool.true) && Bool.true)) && Bool.true) && Bool.true) && Bool.true))) (fun (__b : Bool) => (_root_.LexLeanPreservation.encExcept (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.bool _root_.LexLeanTarget.TargetSyntax.Value.bool) _root_.LexLeanTarget.TargetSyntax.Value.nat) (if __b then (Except.ok (_ychecked1_output) : (Except (Prod Bool Bool) Nat)) else (Except.error (((Bool.true, Bool.false) : (Prod Bool Bool))) : (Except (Prod Bool Bool) Nat)))) (Models.Ledger.belowCheck (_ychecked1_input) (_ychecked1_output)) (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (__rel_3 (_ychecked1_input) (_ychecked1_output))) (fun _ => (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) _root_.LexLeanPreservation.construct_ok)) (fun _ => (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build _root_.LexLeanPreservation.convL_nil _root_.LexLeanPreservation.construct_true) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build _root_.LexLeanPreservation.convL_nil _root_.LexLeanPreservation.construct_false) _root_.LexLeanPreservation.convL_nil)) _root_.LexLeanPreservation.construct_pair) _root_.LexLeanPreservation.convL_nil) _root_.LexLeanPreservation.construct_error))))) (fun _ => (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build _root_.LexLeanPreservation.convL_nil _root_.LexLeanPreservation.construct_false) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build _root_.LexLeanPreservation.convL_nil _root_.LexLeanPreservation.construct_false) _root_.LexLeanPreservation.convL_nil)) _root_.LexLeanPreservation.construct_pair) _root_.LexLeanPreservation.convL_nil) _root_.LexLeanPreservation.construct_error))))

def denote (x : Nat) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 x) (_root_.LexLeanPreservation.Obs.value (((_root_.LexLeanPreservation.encExcept (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.bool _root_.LexLeanTarget.TargetSyntax.Value.bool) _root_.LexLeanTarget.TargetSyntax.Value.nat) (Models.Main.guessChecked x)))) _root_.LexLeanPreservation.Obs.overflow

theorem root (x : Nat) : _root_.LexLeanPreservation.RunConv __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat x)] (_root_.LexLeanPreservation.Rel (__fits_0 x) ((_root_.LexLeanPreservation.encExcept (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.bool _root_.LexLeanTarget.TargetSyntax.Value.bool) _root_.LexLeanTarget.TargetSyntax.Value.nat) (Models.Main.guessChecked x))) :=
  _root_.LexLeanPreservation.run_of_funRel (__rel_0 x)

end LexLeanPreserve.Cd0cbc34d95d16582f4cc8f373aaea24b.R7
