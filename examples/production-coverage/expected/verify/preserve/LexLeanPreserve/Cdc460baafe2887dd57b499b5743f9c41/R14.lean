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
namespace LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R14

def __prog : LexLeanTarget.TargetSyntax.Program :=
  { adts := [{ constructors := [[.nat, .nat]] },
    { constructors := [[.nat, .nat]] },
    { constructors := [[.nat, .nat]] }], functions := [
    { parameters := [0, 1], types := [.nat, .nat], result := (.pair .nat .nat),
      body := (.«let» 2 (.adt 0) (.build (.adt 0) (.adt 0) [(.var 0), (.var 1)]) (.build .pair (.pair .nat .nat) [(.prim .natAdd [(.field (.var 2) 0), (.field (.call 1 []) 1)]), (.call 2 [(.build (.adt 0) (.adt 1) [(.field (.var 2) 1), (.value .nat (.nat 1))])])])) },
    { parameters := [], types := [], result := (.adt 2),
      body := (.build (.adt 0) (.adt 2) [(.value .nat (.nat 2)), (.value .nat (.nat 3))]) },
    { parameters := [0], types := [(.adt 1)], result := .nat,
      body := (.field (.var 0) 0) }] }

def __enc_0 (__s : (Coverage.Types.Point)) : LexLeanTarget.TargetSyntax.Value :=
  LexLeanTarget.TargetSyntax.Value.adt 0 [(LexLeanTarget.TargetSyntax.Value.nat (__s).x), (LexLeanTarget.TargetSyntax.Value.nat (__s).y)]

def __enc_1 (__s : (Coverage.Types.Box Nat)) : LexLeanTarget.TargetSyntax.Value :=
  LexLeanTarget.TargetSyntax.Value.adt 0 [(LexLeanTarget.TargetSyntax.Value.nat (__s).content), (LexLeanTarget.TargetSyntax.Value.nat (__s).count)]

def __enc_2 (__s : (Coverage.Types.Weights)) : LexLeanTarget.TargetSyntax.Value :=
  LexLeanTarget.TargetSyntax.Value.adt 0 [(LexLeanTarget.TargetSyntax.Value.nat (__s).base), (LexLeanTarget.TargetSyntax.Value.nat (__s).scale)]

def __fits_1 : Bool :=
  ((true && (true && true)) && true)

theorem __rel_1 : LexLeanPreservation.FunRel __prog 1 [] (LexLeanPreservation.Rel __fits_1 (__enc_2 Coverage.Types.defaultWeights)) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_adt)

def __fits_2 (box : (Coverage.Types.Box Nat)) : Bool :=
  true

theorem __rel_2 (box : (Coverage.Types.Box Nat)) : LexLeanPreservation.FunRel __prog 2 [(__enc_1 box)] (LexLeanPreservation.Rel (__fits_2 box) (LexLeanTarget.TargetSyntax.Value.nat (Coverage.Types.unbox Nat box))) :=
  LexLeanPreservation.funRel_intro rfl rfl (LexLeanPreservation.conv_field (LexLeanPreservation.conv_var rfl) rfl)

def __fits_0 (a : Nat) (b : Nat) : Bool :=
  (((true && (true && true)) && true) && (let p : (Coverage.Types.Point) := ({ x := a, y := b } : (Coverage.Types.Point)); ((((true && ((true && __fits_1) && true)) && (Nat.blt ((p).x + (Coverage.Types.defaultWeights).scale) 18446744073709551616)) && (((((true && (true && true)) && true) && true) && (__fits_2 (({ content := (p).y, count := (1 : Nat) } : (Coverage.Types.Box Nat))))) && true)) && true)))

attribute [local irreducible] Coverage.Types.unbox in
theorem __rel_0 (a : Nat) (b : Nat) : LexLeanPreservation.FunRel __prog 0 [(LexLeanTarget.TargetSyntax.Value.nat a), (LexLeanTarget.TargetSyntax.Value.nat b)] (LexLeanPreservation.Rel (__fits_0 a b) ((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.nat) (Coverage.Main.records a b))) :=
  LexLeanPreservation.funRel_intro rfl rfl (let p : (Coverage.Types.Point) := ({ x := a, y := b } : (Coverage.Types.Point)); LexLeanPreservation.conv_let (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_var rfl) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_adt) (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_prim (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_field (LexLeanPreservation.conv_var rfl) rfl) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_field (LexLeanPreservation.conv_call LexLeanPreservation.convL_nil __rel_1) rfl) LexLeanPreservation.convL_nil)) (LexLeanPreservation.prim_natAdd (p).x (Coverage.Types.defaultWeights).scale)) (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_call (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons (LexLeanPreservation.conv_field (LexLeanPreservation.conv_var rfl) rfl) (LexLeanPreservation.convL_cons LexLeanPreservation.conv_value LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_adt) LexLeanPreservation.convL_nil) (__rel_2 (({ content := (p).y, count := (1 : Nat) } : (Coverage.Types.Box Nat))))) LexLeanPreservation.convL_nil)) LexLeanPreservation.construct_pair))

def denote (a : Nat) (b : Nat) : LexLeanPreservation.Obs :=
  cond (__fits_0 a b) (LexLeanPreservation.Obs.value (((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.nat) (Coverage.Main.records a b)))) LexLeanPreservation.Obs.overflow

theorem root (a : Nat) (b : Nat) : LexLeanPreservation.RunConv __prog 0 [(LexLeanTarget.TargetSyntax.Value.nat a), (LexLeanTarget.TargetSyntax.Value.nat b)] (LexLeanPreservation.Rel (__fits_0 a b) ((LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.nat) (Coverage.Main.records a b))) :=
  LexLeanPreservation.run_of_funRel (__rel_0 a b)

end LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R14
