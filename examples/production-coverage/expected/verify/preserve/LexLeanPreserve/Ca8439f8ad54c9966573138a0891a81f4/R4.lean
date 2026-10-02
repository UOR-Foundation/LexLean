import LexLeanPreservation.Core
import LexLeanPreservation.Values
import LexLeanPreservation.Primitives
import LexLeanPreservation.Fixed
import LexLeanPreservation.Keys
import LexLeanPreservation.Templates
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
namespace LexLeanPreserve.Ca8439f8ad54c9966573138a0891a81f4.R4

def __prog : LexLeanTarget.TargetSyntax.Program :=
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
  ((true && (true && true)) && (Nat.blt (x + (1 : Nat)) 18446744073709551616))

theorem __rel_2 (x : Nat) : LexLeanPreservation.FunRel __prog 2 [(LexLeanTarget.TargetSyntax.Value.nat x)] (LexLeanPreservation.Rel (__fits_2 x) (LexLeanTarget.TargetSyntax.Value.nat ((x + (1 : Nat))))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd x (1 : Nat)))

def __fits_4 (x : Nat) : Bool :=
  (((true && (true && true)) && true) && (if (Nat.blt x (10 : Nat)) then ((((true && (true && true)) && (Nat.blt (x + (2 : Nat)) 18446744073709551616)) && true) && true) else (true && true)))

theorem __rel_4 (x : Nat) : LexLeanPreservation.FunRel __prog 4 [(LexLeanTarget.TargetSyntax.Value.nat x)] (LexLeanPreservation.Rel (__fits_4 x) ((LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.nat) ((if (Nat.blt x (10 : Nat)) then (Option.some ((x + (2 : Nat))) : (Option Nat)) else (Option.none : (Option Nat)))))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_cond (fun (__b : Bool) => (if __b then ((((true && (true && true)) && (Nat.blt (x + (2 : Nat)) 18446744073709551616)) && true) && true) else (true && true))) (fun (__b : Bool) => (LexLeanPreservation.encOption LexLeanTarget.TargetSyntax.Value.nat) (if __b then (Option.some ((x + (2 : Nat))) : (Option Nat)) else (Option.none : (Option Nat)))) (Nat.blt x (10 : Nat)) (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natLt x (10 : Nat))) (fun _ => (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd x (2 : Nat))) LexLeanPreservation.convL_nil) LexLeanPreservation.construct_some)) (fun _ => (LexLeanPreservation.conv_build LexLeanPreservation.convL_nil LexLeanPreservation.construct_none)))

def __fits_0 (n : Nat) (start : Nat) : Bool :=
  ((((true && (true && (true && true))) && (LexLeanPreservation.iterateFits (fun (__p0 : Nat) => __fits_2 __p0) ((fun (x : Nat) => (x + (1 : Nat)))) (n) (start))) && (((true && (true && (true && true))) && (LexLeanPreservation.untilFits (fun (__p0 : Nat) => __fits_4 __p0) ((fun (x : Nat) => (if (Nat.blt x (10 : Nat)) then (Option.some ((x + (2 : Nat))) : (Option Nat)) else (Option.none : (Option Nat))))) (n) (start))) && true)) && true)

theorem __rel_0 (n : Nat) (start : Nat) : LexLeanPreservation.FunRel __prog 0 [(LexLeanTarget.TargetSyntax.Value.nat n), (LexLeanTarget.TargetSyntax.Value.nat start)] (LexLeanPreservation.Rel (__fits_0 n start) ((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.bool)) (Coverage.Colls.iterations n start))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_closure LexLeanPreservation.convL_nil) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil))) (LexLeanPreservation.tpl_iterate (es := LexLeanTarget.TargetSyntax.Value.nat) ((fun (x : Nat) => (x + (1 : Nat)))) (fun (__p0 : Nat) => __fits_2 __p0) 2 [] (fun (__p0 : Nat) => __rel_2 __p0) (Coverage.Colls.LexLeanCollections.iterate ((fun (x : Nat) => (x + (1 : Nat))))) (fun _ => rfl) (fun _ _ => rfl) (p := __prog) (fi := 1) (ts := .nat) rfl (n) (start))) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_closure LexLeanPreservation.convL_nil) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil))) (LexLeanPreservation.tpl_iterateUntil (es := LexLeanTarget.TargetSyntax.Value.nat) (LexLeanPreservation.optEnc LexLeanTarget.TargetSyntax.Value.nat) (fun _ => rfl) (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.bool) (fun _ => rfl) ((fun (x : Nat) => (if (Nat.blt x (10 : Nat)) then (Option.some ((x + (2 : Nat))) : (Option Nat)) else (Option.none : (Option Nat))))) (fun (__p0 : Nat) => __fits_4 __p0) 4 [] (fun (__p0 : Nat) => __rel_4 __p0) (Coverage.Colls.LexLeanCollections.iterateUntil ((fun (x : Nat) => (if (Nat.blt x (10 : Nat)) then (Option.some ((x + (2 : Nat))) : (Option Nat)) else (Option.none : (Option Nat)))))) (fun _ => rfl) (fun _ _ h => by simp only [Coverage.Colls.LexLeanCollections.iterateUntil, h]) (fun _ _ _ h => by simp only [Coverage.Colls.LexLeanCollections.iterateUntil, h]) (p := __prog) (fi := 3) (ts := .nat) rfl (n) (start))) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair)

def denote (n : Nat) (start : Nat) : LexLeanPreservation.Obs :=
  LexLeanPreservation.Rel (__fits_0 n start) ((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.bool)) (Coverage.Colls.iterations n start))

theorem root (n : Nat) (start : Nat) : LexLeanPreservation.RunConv __prog 0 [(LexLeanTarget.TargetSyntax.Value.nat n), (LexLeanTarget.TargetSyntax.Value.nat start)] (LexLeanPreservation.Rel (__fits_0 n start) ((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat (LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.bool)) (Coverage.Colls.iterations n start))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 n start)

end LexLeanPreserve.Ca8439f8ad54c9966573138a0891a81f4.R4
