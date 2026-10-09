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
namespace LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R5

def __prog : _root_.LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [.nat, .nat], result := (.pair .nat (.pair .nat .bool)),
      body := (.build .pair (.pair .nat (.pair .nat .bool)) [(.call 1 [(.closure 2 []), (.var 0), (.var 1)]), (.call 3 [(.closure 4 []), (.var 0), (.var 1)])]) },
    { parameters := [0, 1, 2], types := [(.fn [.nat] .nat), .nat, .nat], result := .nat,
      body := (.«match» .nat (.var 1) [(.arm .zero [] (.var 2)), (.arm .succ [3] (.call 1 [(.var 0), (.var 3), (.apply (.var 0) [(.var 2)])]))]) },
    { parameters := [0], types := [.nat], result := .nat,
      body := (.prim .natAdd [(.var 0), (.value .nat (.nat 1))]) },
    { parameters := [0, 1, 2], types := [(.fn [.nat] (.option .nat)), .nat, .nat], result := (.pair .nat .bool),
      body := (.«match» (.pair .nat .bool) (.var 1) [(.arm .zero [] (.build .pair (.pair .nat .bool) [(.var 2), (.build .false .bool [])])), (.arm .succ [3] (.«match» (.pair .nat .bool) (.apply (.var 0) [(.var 2)]) [(.arm .none [] (.build .pair (.pair .nat .bool) [(.var 2), (.build .true .bool [])])), (.arm .some [4] (.call 3 [(.var 0), (.var 3), (.var 4)]))]))]) },
    { parameters := [0], types := [.nat], result := (.option .nat),
      body := (.cond (.prim .natLt [(.var 0), (.value .nat (.nat 10))]) (.build .some (.option .nat) [(.prim .natAdd [(.var 0), (.value .nat (.nat 2))])]) (.build .none (.option .nat) [])) }] }

def __fits_2 (x : Nat) : Bool :=
  ((Bool.true && (Bool.true && Bool.true)) && (Nat.blt (x + (1 : Nat)) 18446744073709551616))

theorem __rel_2 (x : Nat) : _root_.LexLeanPreservation.FunRel __prog 2 [(_root_.LexLeanTarget.TargetSyntax.Value.nat x)] (_root_.LexLeanPreservation.Rel (__fits_2 x) (_root_.LexLeanTarget.TargetSyntax.Value.nat ((x + (1 : Nat))))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natAdd x (1 : Nat)))

def __fits_4 (x : Nat) : Bool :=
  (((Bool.true && (Bool.true && Bool.true)) && Bool.true) && (if (Nat.blt x (10 : Nat)) then ((((Bool.true && (Bool.true && Bool.true)) && (Nat.blt (x + (2 : Nat)) 18446744073709551616)) && Bool.true) && Bool.true) else (Bool.true && Bool.true)))

theorem __rel_4 (x : Nat) : _root_.LexLeanPreservation.FunRel __prog 4 [(_root_.LexLeanTarget.TargetSyntax.Value.nat x)] (_root_.LexLeanPreservation.Rel (__fits_4 x) ((_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.nat) ((if (Nat.blt x (10 : Nat)) then (Option.some ((x + (2 : Nat))) : (Option Nat)) else (Option.none : (Option Nat)))))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_cond (fun (__b : Bool) => (if __b then ((((Bool.true && (Bool.true && Bool.true)) && (Nat.blt (x + (2 : Nat)) 18446744073709551616)) && Bool.true) && Bool.true) else (Bool.true && Bool.true))) (fun (__b : Bool) => (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.nat) (if __b then (Option.some ((x + (2 : Nat))) : (Option Nat)) else (Option.none : (Option Nat)))) (Nat.blt x (10 : Nat)) (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natLt x (10 : Nat))) (fun _ => (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natAdd x (2 : Nat))) _root_.LexLeanPreservation.convL_nil) _root_.LexLeanPreservation.construct_some)) (fun _ => (_root_.LexLeanPreservation.conv_build _root_.LexLeanPreservation.convL_nil _root_.LexLeanPreservation.construct_none)))

def __fits_0 (n : Nat) (start : Nat) : Bool :=
  ((((Bool.true && (Bool.true && (Bool.true && Bool.true))) && (_root_.LexLeanPreservation.iterateFits (fun (__p0 : Nat) => __fits_2 __p0) ((fun (x : Nat) => (x + (1 : Nat)))) (n) (start))) && (((Bool.true && (Bool.true && (Bool.true && Bool.true))) && (_root_.LexLeanPreservation.untilFits (fun (__p0 : Nat) => __fits_4 __p0) ((fun (x : Nat) => (if (Nat.blt x (10 : Nat)) then (Option.some ((x + (2 : Nat))) : (Option Nat)) else (Option.none : (Option Nat))))) (n) (start))) && Bool.true)) && Bool.true)

theorem __rel_0 (n : Nat) (start : Nat) : _root_.LexLeanPreservation.FunRel __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat n), (_root_.LexLeanTarget.TargetSyntax.Value.nat start)] (_root_.LexLeanPreservation.Rel (__fits_0 n start) ((_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.nat (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.nat _root_.LexLeanTarget.TargetSyntax.Value.bool)) (Coverage.Colls.iterations n start))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_closure _root_.LexLeanPreservation.convL_nil) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil))) (_root_.LexLeanPreservation.tpl_iterate (es := _root_.LexLeanTarget.TargetSyntax.Value.nat) ((fun (x : Nat) => (x + (1 : Nat)))) (fun (__p0 : Nat) => __fits_2 __p0) 2 [] (fun (__p0 : Nat) => __rel_2 __p0) (Coverage.Colls.LexLeanCollections.iterate ((fun (x : Nat) => (x + (1 : Nat))))) (fun _ => rfl) (fun _ _ => rfl) (p := __prog) (fi := 1) (ts := .nat) rfl (n) (start))) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_closure _root_.LexLeanPreservation.convL_nil) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil))) (_root_.LexLeanPreservation.tpl_iterateUntil (es := _root_.LexLeanTarget.TargetSyntax.Value.nat) (_root_.LexLeanPreservation.optEnc _root_.LexLeanTarget.TargetSyntax.Value.nat) (fun _ => rfl) (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.nat _root_.LexLeanTarget.TargetSyntax.Value.bool) (fun _ => rfl) ((fun (x : Nat) => (if (Nat.blt x (10 : Nat)) then (Option.some ((x + (2 : Nat))) : (Option Nat)) else (Option.none : (Option Nat))))) (fun (__p0 : Nat) => __fits_4 __p0) 4 [] (fun (__p0 : Nat) => __rel_4 __p0) (Coverage.Colls.LexLeanCollections.iterateUntil ((fun (x : Nat) => (if (Nat.blt x (10 : Nat)) then (Option.some ((x + (2 : Nat))) : (Option Nat)) else (Option.none : (Option Nat)))))) (fun _ => rfl) (fun _ _ __h => by simp only [Coverage.Colls.LexLeanCollections.iterateUntil, __h]) (fun _ _ _ __h => by simp only [Coverage.Colls.LexLeanCollections.iterateUntil, __h]) (p := __prog) (fi := 3) (ts := .nat) rfl (n) (start))) _root_.LexLeanPreservation.convL_nil)) _root_.LexLeanPreservation.construct_pair)

def denote (n : Nat) (start : Nat) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 n start) (_root_.LexLeanPreservation.Obs.value (((_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.nat (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.nat _root_.LexLeanTarget.TargetSyntax.Value.bool)) (Coverage.Colls.iterations n start)))) _root_.LexLeanPreservation.Obs.overflow

theorem root (n : Nat) (start : Nat) : _root_.LexLeanPreservation.RunConv __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat n), (_root_.LexLeanTarget.TargetSyntax.Value.nat start)] (_root_.LexLeanPreservation.Rel (__fits_0 n start) ((_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.nat (_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.nat _root_.LexLeanTarget.TargetSyntax.Value.bool)) (Coverage.Colls.iterations n start))) :=
  _root_.LexLeanPreservation.run_of_funRel (__rel_0 n start)

end LexLeanPreserve.Cceddcb09a1ecf4852fbc50de6912225b.R5
