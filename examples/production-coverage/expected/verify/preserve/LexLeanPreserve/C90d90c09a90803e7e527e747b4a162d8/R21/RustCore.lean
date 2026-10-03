import LexLeanPreservation.RustSound
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C90d90c09a90803e7e527e747b4a162d8.R21.RustCore
open LexLeanTarget LexLeanPreservation.Rust

def program : TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [(.fixed .i16), (.fixed .i32)], result := (.pair (.option (.fixed .i16)) (.pair (.option (.fixed .i32)) (.pair (.option (.fixed .i32)) .bool))),
      body := (.build .pair (.pair (.option (.fixed .i16)) (.pair (.option (.fixed .i32)) (.pair (.option (.fixed .i32)) .bool))) [(.prim .checkedMul [(.var 0), (.var 0)]), (.build .pair (.pair (.option (.fixed .i32)) (.pair (.option (.fixed .i32)) .bool)) [(.prim .checkedAdd [(.var 1), (.value (.fixed .i32) (.i32 (-7)))]), (.build .pair (.pair (.option (.fixed .i32)) .bool) [(.prim (.convert .i32) [(.var 0)]), (.prim .equal [(.var 1), (.value (.fixed .i32) (.i32 3))])])])]) }] }

def krate : RustSyntax.Crate :=
  ({ profile := .core, items := [
    (.function (.generated .function 0) [{ pattern := (.bind (.generated .binding 0)), type := (.fixed .i16) }, { pattern := (.bind (.generated .binding 1)), type := (.fixed .i32) }] (.pair (.option (.fixed .i16)) (.pair (.option (.fixed .i32)) (.pair (.option (.fixed .i32)) .bool))) (.mk [] (.pair (.block (.mk [(.mk (.bind (.generated .operand 1)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 2)) none (.copy (.generated .binding 0)))] (.call (.runtime (.checkedMul .i16)) [(.move (.generated .operand 1)), (.move (.generated .operand 2))] false))) (.pair (.block (.mk [(.mk (.bind (.generated .operand 3)) none (.copy (.generated .binding 1))), (.mk (.bind (.generated .operand 4)) none (.lit (.fixed .i32 (-7))))] (.call (.runtime (.checkedAdd .i32)) [(.move (.generated .operand 3)), (.move (.generated .operand 4))] false))) (.pair (.block (.mk [(.mk (.bind (.generated .operand 5)) none (.copy (.generated .binding 0)))] (.call (.runtime (.convert .i32)) [(.widen (.move (.generated .operand 5)))] false))) (.block (.mk [(.mk (.bind (.generated .operand 6)) none (.copy (.generated .binding 1))), (.mk (.bind (.generated .operand 7)) none (.lit (.fixed .i32 3)))] (.call (.runtime .equal) [(.move (.generated .operand 6)), (.move (.generated .operand 7))] false))))))))] } : LexLeanTarget.RustSyntax.Crate)

def flags : Flags := []

theorem fun0 : FunOK program krate flags 0 :=
  ⟨_, rfl, .inl ⟨_, _, [], (.pair (.block (.mk [(.mk (.bind (.generated .operand 1)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 2)) none (.copy (.generated .binding 0)))] (.call (.runtime (.checkedMul .i16)) [(.move (.generated .operand 1)), (.move (.generated .operand 2))] false))) (.pair (.block (.mk [(.mk (.bind (.generated .operand 3)) none (.copy (.generated .binding 1))), (.mk (.bind (.generated .operand 4)) none (.lit (.fixed .i32 (-7))))] (.call (.runtime (.checkedAdd .i32)) [(.move (.generated .operand 3)), (.move (.generated .operand 4))] false))) (.pair (.block (.mk [(.mk (.bind (.generated .operand 5)) none (.copy (.generated .binding 0)))] (.call (.runtime (.convert .i32)) [(.widen (.move (.generated .operand 5)))] false))) (.block (.mk [(.mk (.bind (.generated .operand 6)) none (.copy (.generated .binding 1))), (.mk (.bind (.generated .operand 7)) none (.lit (.fixed .i32 3)))] (.call (.runtime .equal) [(.move (.generated .operand 6)), (.move (.generated .operand 7))] false)))))), [(1, (.fixed .i32), (some 1)), (0, (.fixed .i16), (some 0))], rfl, rfl,
    (Corr.bOfE (Corr.buildPair (Corr.lCons (Corr.eOfB (lets := [(.mk (.bind (.generated .operand 1)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 2)) none (.copy (.generated .binding 0)))]) (tail := (.call (.runtime (.checkedMul .i16)) [(.move (.generated .operand 1)), (.move (.generated .operand 2))] false)) (Corr.prim (ts := [(.fixed .i16), (.fixed .i16)]) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl)) (Corr.lCons (Corr.buildPair (Corr.lCons (Corr.eOfB (lets := [(.mk (.bind (.generated .operand 3)) none (.copy (.generated .binding 1))), (.mk (.bind (.generated .operand 4)) none (.lit (.fixed .i32 (-7))))]) (tail := (.call (.runtime (.checkedAdd .i32)) [(.move (.generated .operand 3)), (.move (.generated .operand 4))] false)) (Corr.prim (ts := [(.fixed .i32), (.fixed .i32)]) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsCons (Corr.value rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl)) (Corr.lCons (Corr.buildPair (Corr.lCons (Corr.eOfB (lets := [(.mk (.bind (.generated .operand 5)) none (.copy (.generated .binding 0)))]) (tail := (.call (.runtime (.convert .i32)) [(.widen (.move (.generated .operand 5)))] false)) (Corr.primWiden (t0 := (.fixed .i16)) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl)) (Corr.lCons (Corr.eOfB (lets := [(.mk (.bind (.generated .operand 6)) none (.copy (.generated .binding 1))), (.mk (.bind (.generated .operand 7)) none (.lit (.fixed .i32 3)))]) (tail := (.call (.runtime .equal) [(.move (.generated .operand 6)), (.move (.generated .operand 7))] false)) (Corr.prim (ts := [(.fixed .i32), (.fixed .i32)]) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsCons (Corr.value rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl)) (Corr.lNil)))) (Corr.lNil)))) (Corr.lNil)))))⟩⟩

theorem crate_ok : CrateOK program krate flags := by
  intro f fn h
  match f with
  | 0 => exact fun0
  | _ + 1 => simp [program, index_eq] at h

/-- Every function of the program is simulated by its rendering. -/
theorem root : ∀ n f, FunSem program krate flags n f := fun n f => simulate crate_ok n f

end LexLeanPreserve.C90d90c09a90803e7e527e747b4a162d8.R21.RustCore
