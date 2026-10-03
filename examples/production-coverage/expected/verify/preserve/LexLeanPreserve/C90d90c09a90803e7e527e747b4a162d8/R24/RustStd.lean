import LexLeanPreservation.RustSound
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C90d90c09a90803e7e527e747b4a162d8.R24.RustStd
open LexLeanTarget LexLeanPreservation.Rust

def program : TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [.int, (.fixed .i8)], result := (.pair (.option (.fixed .u16)) (.option (.fixed .i64))),
      body := (.build .pair (.pair (.option (.fixed .u16)) (.option (.fixed .i64))) [(.prim (.convert .u16) [(.var 0)]), (.prim (.convert .i64) [(.var 1)])]) }] }

def krate : RustSyntax.Crate :=
  ({ profile := .std, items := [
    (.function (.generated .function 0) [{ pattern := (.bind (.generated .binding 0)), type := .int }, { pattern := (.bind (.generated .binding 1)), type := (.fixed .i8) }] (.pair (.option (.fixed .u16)) (.option (.fixed .i64))) (.mk [] (.pair (.block (.mk [(.mk (.bind (.generated .operand 1)) none (.copy (.generated .binding 0)))] (.call (.runtime (.convert .u16)) [(.widen (.move (.generated .operand 1)))] false))) (.block (.mk [(.mk (.bind (.generated .operand 2)) none (.copy (.generated .binding 1)))] (.call (.runtime (.convert .i64)) [(.widen (.move (.generated .operand 2)))] false))))))] } : LexLeanTarget.RustSyntax.Crate)

def flags : Flags := []

theorem fun0 : FunOK program krate flags 0 :=
  ⟨_, rfl, .inl ⟨_, _, [], (.pair (.block (.mk [(.mk (.bind (.generated .operand 1)) none (.copy (.generated .binding 0)))] (.call (.runtime (.convert .u16)) [(.widen (.move (.generated .operand 1)))] false))) (.block (.mk [(.mk (.bind (.generated .operand 2)) none (.copy (.generated .binding 1)))] (.call (.runtime (.convert .i64)) [(.widen (.move (.generated .operand 2)))] false)))), [(1, (.fixed .i8), (some 1)), (0, .int, (some 0))], rfl, rfl,
    (Corr.bOfE (Corr.buildPair (Corr.lCons (Corr.eOfB (lets := [(.mk (.bind (.generated .operand 1)) none (.copy (.generated .binding 0)))]) (tail := (.call (.runtime (.convert .u16)) [(.widen (.move (.generated .operand 1)))] false)) (Corr.primWiden (t0 := .int) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl)) (Corr.lCons (Corr.eOfB (lets := [(.mk (.bind (.generated .operand 2)) none (.copy (.generated .binding 1)))]) (tail := (.call (.runtime (.convert .i64)) [(.widen (.move (.generated .operand 2)))] false)) (Corr.primWiden (t0 := (.fixed .i8)) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl)) (Corr.lNil)))))⟩⟩

theorem crate_ok : CrateOK program krate flags := by
  intro f fn h
  match f with
  | 0 => exact fun0
  | _ + 1 => simp [program, index_eq] at h

/-- Every function of the program is simulated by its rendering. -/
theorem root : ∀ n f, FunSem program krate flags n f := fun n f => simulate crate_ok n f

end LexLeanPreserve.C90d90c09a90803e7e527e747b4a162d8.R24.RustStd
