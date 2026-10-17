import LexLeanPreservation.Core
import LexLeanPreservation.Values
import LexLeanPreservation.Primitives
import LexLeanPreservation.Fixed
import LexLeanPreservation.Keys
import LexLeanPreservation.Templates
import LexLeanPreservation.Validate
import LexLeanStd.Text
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 0
set_option synthInstance.maxHeartbeats 0
set_option linter.unusedVariables false
namespace LexLeanPreserve.C773f2bfdb607c032ddcf4a67bed19d11.R0

def __prog : _root_.LexLeanTarget.TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [.bytes, .bytes], result := .bool,
      body := (.«match» .bool (.prim .slice [(.var 0), (.value .nat (.nat 0)), (.prim .length [(.var 1)])]) [(.arm .some [2] (.prim .equal [(.var 2), (.var 1)])), (.arm .none [] (.build .false .bool []))]) }] }

def __fits_0 (subject : ByteArray) («prefix» : ByteArray) : Bool :=
  (((Bool.true && (Bool.true && (((Bool.true && Bool.true) && (Nat.blt (_root_.LexLeanRuntime.length («prefix») : Nat) 18446744073709551616)) && Bool.true))) && Bool.true) && (match (_root_.LexLeanRuntime.slice (subject) ((0 : Nat)) ((_root_.LexLeanRuntime.length («prefix») : Nat)) : (Option ByteArray)) with | Option.some head => ((Bool.true && (Bool.true && Bool.true)) && Bool.true) | Option.none => (Bool.true && Bool.true)))

theorem __rel_0 (subject : ByteArray) («prefix» : ByteArray) : _root_.LexLeanPreservation.FunRel __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.bytes subject), (_root_.LexLeanTarget.TargetSyntax.Value.bytes «prefix»)] (_root_.LexLeanPreservation.Rel (__fits_0 subject «prefix») (_root_.LexLeanTarget.TargetSyntax.Value.bool (LexLeanStd.Text.startsWith subject «prefix»))) :=
  _root_.LexLeanPreservation.funRel_intro rfl rfl (_root_.LexLeanPreservation.conv_matchV (_root_.LexLeanPreservation.encOption _root_.LexLeanTarget.TargetSyntax.Value.bytes) (fun (__a : (Option ByteArray)) => (match __a with | Option.some head => ((Bool.true && (Bool.true && Bool.true)) && Bool.true) | Option.none => (Bool.true && Bool.true))) (fun (__a : (Option ByteArray)) => _root_.LexLeanTarget.TargetSyntax.Value.bool (match __a with | Option.some head => (_root_.LexLeanRuntime.equal (head) («prefix») : Bool) | Option.none => Bool.false)) (_root_.LexLeanRuntime.slice (subject) ((0 : Nat)) ((_root_.LexLeanRuntime.length («prefix») : Nat)) : (Option ByteArray)) (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons _root_.LexLeanPreservation.conv_value (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil) (_root_.LexLeanPreservation.prim_length_bytes «prefix»)) _root_.LexLeanPreservation.convL_nil))) (_root_.LexLeanPreservation.prim_slice_bytes subject (0 : Nat) (_root_.LexLeanRuntime.length («prefix») : Nat))) (fun (__a : (Option ByteArray)) _ => match __a with | Option.some head => (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_prim (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) (_root_.LexLeanPreservation.convL_cons (_root_.LexLeanPreservation.conv_var rfl) _root_.LexLeanPreservation.convL_nil)) (_root_.LexLeanPreservation.prim_equal_bytes head «prefix»))) | Option.none => (_root_.LexLeanPreservation.convA_miss rfl (_root_.LexLeanPreservation.convA_hit rfl rfl (_root_.LexLeanPreservation.conv_build _root_.LexLeanPreservation.convL_nil _root_.LexLeanPreservation.construct_false)))))

def denote (subject : ByteArray) («prefix» : ByteArray) : _root_.LexLeanPreservation.Obs :=
  _root_.cond (__fits_0 subject «prefix») (_root_.LexLeanPreservation.Obs.value ((_root_.LexLeanTarget.TargetSyntax.Value.bool (LexLeanStd.Text.startsWith subject «prefix»)))) _root_.LexLeanPreservation.Obs.overflow

theorem root (subject : ByteArray) («prefix» : ByteArray) : _root_.LexLeanPreservation.RunConv __prog 0 [(_root_.LexLeanTarget.TargetSyntax.Value.bytes subject), (_root_.LexLeanTarget.TargetSyntax.Value.bytes «prefix»)] (_root_.LexLeanPreservation.Rel (__fits_0 subject «prefix») (_root_.LexLeanTarget.TargetSyntax.Value.bool (LexLeanStd.Text.startsWith subject «prefix»))) :=
  _root_.LexLeanPreservation.run_of_funRel (__rel_0 subject «prefix»)

end LexLeanPreserve.C773f2bfdb607c032ddcf4a67bed19d11.R0
