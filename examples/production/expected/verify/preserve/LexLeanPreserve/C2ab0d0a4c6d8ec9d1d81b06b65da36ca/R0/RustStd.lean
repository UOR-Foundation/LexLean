import LexLeanPreservation.RustSound
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C2ab0d0a4c6d8ec9d1d81b06b65da36ca.R0.RustStd
open LexLeanTarget LexLeanPreservation.Rust

def program : TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [(.fixed .u32), (.fixed .u32)], result := (.option (.fixed .u32)),
      body := (.prim .checkedAdd [(.var 0), (.var 1)]) }] }

def krate : RustSyntax.Crate :=
  ({ profile := .std, items := [
    (.function (.generated .function 0) [{ pattern := (.bind (.generated .binding 0)), type := (.fixed .u32) }, { pattern := (.bind (.generated .binding 1)), type := (.fixed .u32) }] (.option (.fixed .u32)) (.mk [(.mk (.bind (.generated .operand 1)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 2)) none (.copy (.generated .binding 1)))] (.call (.runtime (.checkedAdd .u32)) [(.move (.generated .operand 1)), (.move (.generated .operand 2))] false)))] } : LexLeanTarget.RustSyntax.Crate)

def flags : Flags := []

theorem fun0 : FunOK program krate flags 0 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .operand 1)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 2)) none (.copy (.generated .binding 1)))], (.call (.runtime (.checkedAdd .u32)) [(.move (.generated .operand 1)), (.move (.generated .operand 2))] false), [(1, (.fixed .u32), (some 1)), (0, (.fixed .u32), (some 0))], rfl, rfl,
    (Corr.prim (ts := [(.fixed .u32), (.fixed .u32)]) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl)⟩⟩

theorem crate_ok : CrateOK program krate flags := by
  intro f fn h
  match f with
  | 0 => exact fun0
  | _ + 1 => simp [program, index_eq] at h

/-- Every function of the program is simulated by its rendering. -/
theorem root : ∀ n f, FunSem program krate flags n f := fun n f => simulate crate_ok n f

end LexLeanPreserve.C2ab0d0a4c6d8ec9d1d81b06b65da36ca.R0.RustStd
