import LexLeanPreservation.RustSound
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R9.RustStd
open LexLeanTarget LexLeanPreservation.Rust

def program : TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [.nat, .string], result := (.pair (.list (.pair .string .nat)) (.pair (.list .nat) (.list (.pair .nat (.list .nat))))),
      body := (.build .pair (.pair (.list (.pair .string .nat)) (.pair (.list .nat) (.list (.pair .nat (.list .nat))))) [(.build .cons (.list (.pair .string .nat)) [(.build .pair (.pair .string .nat) [(.value .string (.string "a")), (.var 0)]), (.build .cons (.list (.pair .string .nat)) [(.build .pair (.pair .string .nat) [(.value .string (.string "b")), (.value .nat (.nat 2))]), (.build .nil (.list (.pair .string .nat)) [])])]), (.build .pair (.pair (.list .nat) (.list (.pair .nat (.list .nat)))) [(.build .cons (.list .nat) [(.value .nat (.nat 1)), (.build .cons (.list .nat) [(.value .nat (.nat 3)), (.build .nil (.list .nat) [])])]), (.build .cons (.list (.pair .nat (.list .nat))) [(.build .pair (.pair .nat (.list .nat)) [(.value .nat (.nat 1)), (.build .cons (.list .nat) [(.value .nat (.nat 2)), (.build .nil (.list .nat) [])])]), (.build .cons (.list (.pair .nat (.list .nat))) [(.build .pair (.pair .nat (.list .nat)) [(.value .nat (.nat 2)), (.build .cons (.list .nat) [(.value .nat (.nat 1)), (.build .nil (.list .nat) [])])]), (.build .nil (.list (.pair .nat (.list .nat))) [])])])])]) }] }

def krate : RustSyntax.Crate :=
  ({ profile := .std, items := [
    (.function (.generated .function 0) [{ pattern := (.bind (.generated .binding 0)), type := .nat }, { pattern := .wild, type := .str }] (.pair (.list (.pair .str .nat)) (.pair (.list .nat) (.list (.pair .nat (.list .nat))))) (.mk [] (.pair (.construct .cons [(.pair (.lit (.str "a")) (.copy (.generated .binding 0))), (.construct .cons [(.pair (.lit (.str "b")) (.lit (.nat 2))), (.construct (.nil (.pair .str .nat)) [])])]) (.pair (.construct .cons [(.lit (.nat 1)), (.construct .cons [(.lit (.nat 3)), (.construct (.nil .nat) [])])]) (.construct .cons [(.pair (.lit (.nat 1)) (.construct .cons [(.lit (.nat 2)), (.construct (.nil .nat) [])])), (.construct .cons [(.pair (.lit (.nat 2)) (.construct .cons [(.lit (.nat 1)), (.construct (.nil .nat) [])])), (.construct (.nil (.pair .nat (.list .nat))) [])])])))))] } : LexLeanTarget.RustSyntax.Crate)

def flags : Flags := []

theorem fun0 : FunOK program krate flags 0 :=
  ⟨_, rfl, .inl ⟨_, _, [], (.pair (.construct .cons [(.pair (.lit (.str "a")) (.copy (.generated .binding 0))), (.construct .cons [(.pair (.lit (.str "b")) (.lit (.nat 2))), (.construct (.nil (.pair .str .nat)) [])])]) (.pair (.construct .cons [(.lit (.nat 1)), (.construct .cons [(.lit (.nat 3)), (.construct (.nil .nat) [])])]) (.construct .cons [(.pair (.lit (.nat 1)) (.construct .cons [(.lit (.nat 2)), (.construct (.nil .nat) [])])), (.construct .cons [(.pair (.lit (.nat 2)) (.construct .cons [(.lit (.nat 1)), (.construct (.nil .nat) [])])), (.construct (.nil (.pair .nat (.list .nat))) [])])]))), [(1, .string, none), (0, .nat, (some 0))], rfl, rfl,
    (Corr.bOfE (Corr.buildPair (Corr.lCons (Corr.eOfBNil (Corr.build (ts := [(.pair .string .nat), (.list (.pair .string .nat))]) (mask := [false, false]) (Corr.opsOfL (Corr.lCons (Corr.buildPair (Corr.lCons (Corr.value rfl rfl) (Corr.lCons (Corr.var rfl rfl) (Corr.lNil)))) (Corr.lCons (Corr.eOfBNil (Corr.build (ts := [(.pair .string .nat), (.list (.pair .string .nat))]) (mask := [false, false]) (Corr.opsOfL (Corr.lCons (Corr.buildPair (Corr.lCons (Corr.value rfl rfl) (Corr.lCons (Corr.value rfl rfl) (Corr.lNil)))) (Corr.lCons (Corr.eOfBNil (Corr.build (ts := []) (mask := []) (Corr.opsNil) rfl)) (Corr.lNil)))) rfl)) (Corr.lNil)))) rfl)) (Corr.lCons (Corr.buildPair (Corr.lCons (Corr.eOfBNil (Corr.build (ts := [.nat, (.list .nat)]) (mask := [false, false]) (Corr.opsOfL (Corr.lCons (Corr.value rfl rfl) (Corr.lCons (Corr.eOfBNil (Corr.build (ts := [.nat, (.list .nat)]) (mask := [false, false]) (Corr.opsOfL (Corr.lCons (Corr.value rfl rfl) (Corr.lCons (Corr.eOfBNil (Corr.build (ts := []) (mask := []) (Corr.opsNil) rfl)) (Corr.lNil)))) rfl)) (Corr.lNil)))) rfl)) (Corr.lCons (Corr.eOfBNil (Corr.build (ts := [(.pair .nat (.list .nat)), (.list (.pair .nat (.list .nat)))]) (mask := [false, false]) (Corr.opsOfL (Corr.lCons (Corr.buildPair (Corr.lCons (Corr.value rfl rfl) (Corr.lCons (Corr.eOfBNil (Corr.build (ts := [.nat, (.list .nat)]) (mask := [false, false]) (Corr.opsOfL (Corr.lCons (Corr.value rfl rfl) (Corr.lCons (Corr.eOfBNil (Corr.build (ts := []) (mask := []) (Corr.opsNil) rfl)) (Corr.lNil)))) rfl)) (Corr.lNil)))) (Corr.lCons (Corr.eOfBNil (Corr.build (ts := [(.pair .nat (.list .nat)), (.list (.pair .nat (.list .nat)))]) (mask := [false, false]) (Corr.opsOfL (Corr.lCons (Corr.buildPair (Corr.lCons (Corr.value rfl rfl) (Corr.lCons (Corr.eOfBNil (Corr.build (ts := [.nat, (.list .nat)]) (mask := [false, false]) (Corr.opsOfL (Corr.lCons (Corr.value rfl rfl) (Corr.lCons (Corr.eOfBNil (Corr.build (ts := []) (mask := []) (Corr.opsNil) rfl)) (Corr.lNil)))) rfl)) (Corr.lNil)))) (Corr.lCons (Corr.eOfBNil (Corr.build (ts := []) (mask := []) (Corr.opsNil) rfl)) (Corr.lNil)))) rfl)) (Corr.lNil)))) rfl)) (Corr.lNil)))) (Corr.lNil)))))⟩⟩

theorem crate_ok : CrateOK program krate flags := by
  intro f fn h
  match f with
  | 0 => exact fun0
  | _ + 1 => simp [program, index_eq] at h

/-- Every function of the program is simulated by its rendering. -/
theorem root : ∀ n f, FunSem program krate flags n f := fun n f => simulate crate_ok n f

end LexLeanPreserve.Caf8298921c44aaa9b843e59a43cd1585.R9.RustStd
