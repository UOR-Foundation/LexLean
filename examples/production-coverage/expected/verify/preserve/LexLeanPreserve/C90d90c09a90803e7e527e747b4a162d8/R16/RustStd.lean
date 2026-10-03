import LexLeanPreservation.RustSound
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C90d90c09a90803e7e527e747b4a162d8.R16.RustStd
open LexLeanTarget LexLeanPreservation.Rust

def program : TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0], types := [(.pair .nat .bool)], result := (.pair .bool .nat),
      body := (.build .pair (.pair .bool .nat) [(.second (.var 0)), (.first (.var 0))]) }] }

def krate : RustSyntax.Crate :=
  ({ profile := .std, items := [
    (.function (.generated .function 0) [{ pattern := (.bind (.generated .binding 0)), type := (.pair .nat .bool) }] (.pair .bool .nat) (.mk [] (.pair (.block (.mk [(.mk (.tuple [.wild, (.bind (.generated .part 1))]) none (.copy (.generated .binding 0)))] (.move (.generated .part 1)))) (.block (.mk [(.mk (.tuple [(.bind (.generated .part 2)), .wild]) none (.copy (.generated .binding 0)))] (.move (.generated .part 2)))))))] } : LexLeanTarget.RustSyntax.Crate)

def flags : Flags := []

theorem fun0 : FunOK program krate flags 0 :=
  ⟨_, rfl, .inl ⟨_, _, [], (.pair (.block (.mk [(.mk (.tuple [.wild, (.bind (.generated .part 1))]) none (.copy (.generated .binding 0)))] (.move (.generated .part 1)))) (.block (.mk [(.mk (.tuple [(.bind (.generated .part 2)), .wild]) none (.copy (.generated .binding 0)))] (.move (.generated .part 2))))), [(0, (.pair .nat .bool), (some 0))], rfl, rfl,
    (Corr.bOfE (Corr.buildPair (Corr.lCons (Corr.eOfB (lets := [(.mk (.tuple [.wild, (.bind (.generated .part 1))]) none (.copy (.generated .binding 0)))]) (tail := (.move (.generated .part 1))) (Corr.second (ta := .nat) (tb := .bool) (Corr.var rfl rfl) rfl)) (Corr.lCons (Corr.eOfB (lets := [(.mk (.tuple [(.bind (.generated .part 2)), .wild]) none (.copy (.generated .binding 0)))]) (tail := (.move (.generated .part 2))) (Corr.first (ta := .nat) (tb := .bool) (Corr.var rfl rfl) rfl)) (Corr.lNil)))))⟩⟩

theorem crate_ok : CrateOK program krate flags := by
  intro f fn h
  match f with
  | 0 => exact fun0
  | _ + 1 => simp [program, index_eq] at h

/-- Every function of the program is simulated by its rendering. -/
theorem root : ∀ n f, FunSem program krate flags n f := fun n f => simulate crate_ok n f

end LexLeanPreserve.C90d90c09a90803e7e527e747b4a162d8.R16.RustStd
