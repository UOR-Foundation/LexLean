import LexLeanPreservation.RustSound
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R3.RustStd
open LexLeanTarget LexLeanPreservation.Rust

def program : TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0], types := [.nat], result := .nat,
      body := (.call 1 [(.var 0), (.value .nat (.nat 0))]) },
    { parameters := [0, 1], types := [.nat, .nat], result := .nat,
      body := (.cond (.prim .natLt [(.var 0), (.value .nat (.nat 2))]) (.var 1) (.call 1 [(.prim .natSub [(.var 0), (.value .nat (.nat 2))]), (.prim .natAdd [(.var 1), (.value .nat (.nat 1))])])) }] }

def krate : RustSyntax.Crate :=
  ({ profile := .std, items := [
    (.function (.generated .function 0) [{ pattern := (.bind (.generated .binding 0)), type := .nat }] (.fallible .nat) (.mk [] (.call (.function 1) [(.copy (.generated .binding 0)), (.lit (.nat 0))] false))),
    (.function (.generated .function 1) [{ pattern := (.bind (.generated .binding 0)), type := .nat }, { pattern := (.bind (.generated .binding 1)), type := .nat }] (.fallible .nat) (.mk [(.mk (.bind (.generated .operand 1)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 2)) none (.lit (.nat 2)))] (.cond (.call (.runtime .natLt) [(.move (.generated .operand 1)), (.move (.generated .operand 2))] false) (.mk [] (.succeed (.copy (.generated .binding 1)))) (.mk [] (.call (.function 1) [(.block (.mk [(.mk (.bind (.generated .operand 3)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 4)) none (.lit (.nat 2)))] (.call (.runtime .natSub) [(.move (.generated .operand 3)), (.move (.generated .operand 4))] false))), (.block (.mk [(.mk (.bind (.generated .operand 5)) none (.copy (.generated .binding 1))), (.mk (.bind (.generated .operand 6)) none (.lit (.nat 1)))] (.call (.runtime .natAdd) [(.move (.generated .operand 5)), (.move (.generated .operand 6))] true)))] false)))))] } : LexLeanTarget.RustSyntax.Crate)

def flags : Flags := []

theorem fun0 : FunOK program krate flags 0 :=
  ⟨_, rfl, .inl ⟨_, _, [], (.call (.function 1) [(.copy (.generated .binding 0)), (.lit (.nat 0))] false), [(0, .nat, (some 0))], rfl, rfl,
    (Corr.callF (ts := [.nat, .nat]) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lCons (Corr.value rfl rfl) (Corr.lNil)))) rfl rfl rfl rfl)⟩⟩

theorem fun1 : FunOK program krate flags 1 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .operand 1)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 2)) none (.lit (.nat 2)))], (.cond (.call (.runtime .natLt) [(.move (.generated .operand 1)), (.move (.generated .operand 2))] false) (.mk [] (.succeed (.copy (.generated .binding 1)))) (.mk [] (.call (.function 1) [(.block (.mk [(.mk (.bind (.generated .operand 3)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 4)) none (.lit (.nat 2)))] (.call (.runtime .natSub) [(.move (.generated .operand 3)), (.move (.generated .operand 4))] false))), (.block (.mk [(.mk (.bind (.generated .operand 5)) none (.copy (.generated .binding 1))), (.mk (.bind (.generated .operand 6)) none (.lit (.nat 1)))] (.call (.runtime .natAdd) [(.move (.generated .operand 5)), (.move (.generated .operand 6))] true)))] false))), [(1, .nat, (some 1)), (0, .nat, (some 0))], rfl, rfl,
    (Corr.cond (lc := [(.mk (.bind (.generated .operand 1)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 2)) none (.lit (.nat 2)))]) (ce := (.call (.runtime .natLt) [(.move (.generated .operand 1)), (.move (.generated .operand 2))] false)) (lk := []) (Corr.prim (ts := [.nat, .nat]) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsCons (Corr.value rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl) (Corr.kIf (la := []) (ta := (.succeed (.copy (.generated .binding 1)))) (lb := []) (tb := (.call (.function 1) [(.block (.mk [(.mk (.bind (.generated .operand 3)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 4)) none (.lit (.nat 2)))] (.call (.runtime .natSub) [(.move (.generated .operand 3)), (.move (.generated .operand 4))] false))), (.block (.mk [(.mk (.bind (.generated .operand 5)) none (.copy (.generated .binding 1))), (.mk (.bind (.generated .operand 6)) none (.lit (.nat 1)))] (.call (.runtime .natAdd) [(.move (.generated .operand 5)), (.move (.generated .operand 6))] true)))] false)) (Corr.fOfB (Corr.bOfE (Corr.var rfl rfl))) (Corr.callF (ts := [.nat, .nat]) (Corr.opsOfL (Corr.lCons (Corr.eOfB (lets := [(.mk (.bind (.generated .operand 3)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 4)) none (.lit (.nat 2)))]) (tail := (.call (.runtime .natSub) [(.move (.generated .operand 3)), (.move (.generated .operand 4))] false)) (Corr.prim (ts := [.nat, .nat]) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsCons (Corr.value rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl)) (Corr.lCons (Corr.eOfB (lets := [(.mk (.bind (.generated .operand 5)) none (.copy (.generated .binding 1))), (.mk (.bind (.generated .operand 6)) none (.lit (.nat 1)))]) (tail := (.call (.runtime .natAdd) [(.move (.generated .operand 5)), (.move (.generated .operand 6))] true)) (Corr.prim (ts := [.nat, .nat]) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsCons (Corr.value rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl)) (Corr.lNil)))) rfl rfl rfl rfl)))⟩⟩

theorem crate_ok : CrateOK program krate flags := by
  intro f fn h
  match f with
  | 0 => exact fun0
  | 1 => exact fun1
  | _ + 2 => simp [program, index_eq] at h

/-- Every function of the program is simulated by its rendering. -/
theorem root : ∀ n f, FunSem program krate flags n f := fun n f => simulate crate_ok n f

end LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R3.RustStd
