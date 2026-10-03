import LexLeanPreservation.RustSound
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.Cba27a3dc52c63de8b08b0726788d5c73.R26.RustStd
open LexLeanTarget LexLeanPreservation.Rust

def program : TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0], types := [.string], result := (.pair (.option .int) (.option (.fixed .u32))),
      body := (.build .pair (.pair (.option .int) (.option (.fixed .u32))) [(.prim (.parseDecimal .int) [(.var 0)]), (.prim (.parseDecimal (.fixed .u32)) [(.var 0)])]) }] }

def krate : RustSyntax.Crate :=
  ({ profile := .std, items := [
    (.function (.generated .function 0) [{ pattern := (.bind (.generated .binding 0)), type := .str }] (.fallible (.pair (.option .int) (.option (.fixed .u32)))) (.mk [] (.succeed (.pair (.block (.mk [(.mk (.bind (.generated .operand 1)) none (.clone (.generated .binding 0)))] (.call (.runtime .parseInt) [(.move (.generated .operand 1))] true))) (.block (.mk [(.mk (.bind (.generated .operand 2)) none (.clone (.generated .binding 0)))] (.call (.runtime (.parseFixed .u32)) [(.move (.generated .operand 2))] false)))))))] } : LexLeanTarget.RustSyntax.Crate)

def flags : Flags := []

theorem fun0 : FunOK program krate flags 0 :=
  ⟨_, rfl, .inl ⟨_, _, [], (.succeed (.pair (.block (.mk [(.mk (.bind (.generated .operand 1)) none (.clone (.generated .binding 0)))] (.call (.runtime .parseInt) [(.move (.generated .operand 1))] true))) (.block (.mk [(.mk (.bind (.generated .operand 2)) none (.clone (.generated .binding 0)))] (.call (.runtime (.parseFixed .u32)) [(.move (.generated .operand 2))] false))))), [(0, .string, (some 0))], rfl, rfl,
    (Corr.fOfB (Corr.bOfE (Corr.buildPair (Corr.lCons (Corr.eOfB (lets := [(.mk (.bind (.generated .operand 1)) none (.clone (.generated .binding 0)))]) (tail := (.call (.runtime .parseInt) [(.move (.generated .operand 1))] true)) (Corr.prim (ts := [.string]) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl rfl rfl rfl rfl)) (Corr.lCons (Corr.eOfB (lets := [(.mk (.bind (.generated .operand 2)) none (.clone (.generated .binding 0)))]) (tail := (.call (.runtime (.parseFixed .u32)) [(.move (.generated .operand 2))] false)) (Corr.prim (ts := [.string]) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl rfl rfl rfl rfl)) (Corr.lNil))))))⟩⟩

theorem crate_ok : CrateOK program krate flags := by
  intro f fn h
  match f with
  | 0 => exact fun0
  | _ + 1 => simp [program, index_eq] at h

/-- Every function of the program is simulated by its rendering. -/
theorem root : ∀ n f, FunSem program krate flags n f := fun n f => simulate crate_ok n f

end LexLeanPreserve.Cba27a3dc52c63de8b08b0726788d5c73.R26.RustStd
