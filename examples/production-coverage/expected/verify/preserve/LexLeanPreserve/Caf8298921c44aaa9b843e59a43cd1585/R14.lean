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
namespace LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R14

def __prog : _root_.LexLeanTarget.TargetSyntax.Program :=
  { adts := [{ constructors := [[.nat, .nat]] },
    { constructors := [[.nat, .nat]] },
    { constructors := [[.nat, .nat]] }], functions := [
    { parameters := [0, 1], types := [.nat, .nat], result := (.pair .nat .nat),
      body := (.«let» 2 (.adt 0) (.build (.adt 0) (.adt 0) [(.var 0), (.var 1)]) (.build .pair (.pair .nat .nat) [(.prim .natAdd [(.field (.var 2) 0), (.field (.call 1 []) 1)]), (.call 2 [(.build (.adt 0) (.adt 1) [(.field (.var 2) 1), (.value .nat (.nat 1))])])])) },
    { parameters := [], types := [], result := (.adt 2),
      body := (.build (.adt 0) (.adt 2) [(.value .nat (.nat 2)), (.value .nat (.nat 3))]) },
    { parameters := [0], types := [(.adt 1)], result := .nat,
      body := (.field (.var 0) 0) }] }

def __enc_0 (__s : (Coverage.Types.Point)) : _root_.LexLeanTarget.TargetSyntax.Value :=
  _root_.LexLeanTarget.TargetSyntax.Value.adt 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat (__s).x), (_root_.LexLeanTarget.TargetSyntax.Value.nat (__s).y)]

def __enc_1 (__s : (Coverage.Types.Box Nat)) : _root_.LexLeanTarget.TargetSyntax.Value :=
  _root_.LexLeanTarget.TargetSyntax.Value.adt 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat (__s).content), (_root_.LexLeanTarget.TargetSyntax.Value.nat (__s).count)]

def __enc_2 (__s : (Coverage.Types.Weights)) : _root_.LexLeanTarget.TargetSyntax.Value :=
  _root_.LexLeanTarget.TargetSyntax.Value.adt 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat (__s).base), (_root_.LexLeanTarget.TargetSyntax.Value.nat (__s).scale)]

def __fits_1 : Bool :=
  ((Bool.true && (Bool.true && Bool.true)) && Bool.true)

theorem __rel_1 : _root_.LexLeanPreservation.FunRel __prog 1 [] (_root_.LexLeanPreservation.Rel __fits_1 (__enc_2 Coverage.Types.defaultWeights)) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value _root_.LexLeanPreservation.convL_nil)) _root_.LexLeanPreservation.construct_adt)

def __fits_2 (box : (Coverage.Types.Box Nat)) : Bool :=
  Bool.true

theorem __rel_2 (box : (Coverage.Types.Box Nat)) : _root_.LexLeanPreservation.FunRel __prog 2 [(__enc_1 box)] (_root_.LexLeanPreservation.Rel (__fits_2 box) (_root_.LexLeanTarget.TargetSyntax.Value.nat (Coverage.Types.unbox Nat box))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_field (_root_.LexLeanPreservation.conv_var rfl) rfl)

def __fits_0 (a : Nat) (b : Nat) : Bool :=
  (((Bool.true && (Bool.true && Bool.true)) && Bool.true) && (let p : (Coverage.Types.Point) := ({ x := a, y := b } : (Coverage.Types.Point)); ((((Bool.true && ((Bool.true && __fits_1) && Bool.true)) && (Nat.blt ((p).x + (Coverage.Types.defaultWeights).scale) 18446744073709551616)) && (((((Bool.true && (Bool.true && Bool.true)) && Bool.true) && Bool.true) && (__fits_2 (({ content := (p).y, count := (1 : Nat) } : (Coverage.Types.Box Nat))))) && Bool.true)) && Bool.true)))

attribute [local irreducible] Coverage.Types.unbox in
theorem __rel_0 (a : Nat) (b : Nat) : _root_.LexLeanPreservation.FunRel __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat a), (_root_.LexLeanTarget.TargetSyntax.Value.nat b)] (_root_.LexLeanPreservation.Rel (__fits_0 a b) ((_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.nat _root_.LexLeanTarget.TargetSyntax.Value.nat) (Coverage.Main.records a b))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (let p : (Coverage.Types.Point) := ({ x := a, y := b } : (Coverage.Types.Point)); _root_.LexLeanPreservation.conv_let (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) _root_.LexLeanPreservation.construct_adt) (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_field (_root_.LexLeanPreservation.conv_var rfl) rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_field (_root_.LexLeanPreservation.conv_call _root_.LexLeanPreservation.convL_nil __rel_1) rfl) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_natAdd (p).x (Coverage.Types.defaultWeights).scale)) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_call (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_build (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_field (_root_.LexLeanPreservation.conv_var rfl) rfl) (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value _root_.LexLeanPreservation.convL_nil)) _root_.LexLeanPreservation.construct_adt) _root_.LexLeanPreservation.convL_nil) (__rel_2 (({ content := (p).y, count := (1 : Nat) } : (Coverage.Types.Box Nat))))) _root_.LexLeanPreservation.convL_nil)) _root_.LexLeanPreservation.construct_pair))

def denote (a : Nat) (b : Nat) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 a b) (_root_.LexLeanPreservation.Obs.value (((_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.nat _root_.LexLeanTarget.TargetSyntax.Value.nat) (Coverage.Main.records a b)))) _root_.LexLeanPreservation.Obs.overflow

theorem root (a : Nat) (b : Nat) : _root_.LexLeanPreservation.RunConv __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.nat a), (_root_.LexLeanTarget.TargetSyntax.Value.nat b)] (_root_.LexLeanPreservation.Rel (__fits_0 a b) ((_root_.LexLeanPreservation.encPair _root_.LexLeanTarget.TargetSyntax.Value.nat _root_.LexLeanTarget.TargetSyntax.Value.nat) (Coverage.Main.records a b))) :=
  _root_.LexLeanPreservation.run_of_funRel (__rel_0 a b)

end LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R14
