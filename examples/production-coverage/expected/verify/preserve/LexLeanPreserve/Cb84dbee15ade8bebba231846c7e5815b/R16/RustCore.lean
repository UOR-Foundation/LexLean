import LexLeanPreservation.RustSound
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R16.RustCore
open LexLeanTarget LexLeanPreservation.Rust

def program : TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0], types := [.nat], result := (.pair (.option .nat) (.result .nat .unit)),
      body := (.build .pair (.pair (.option .nat) (.result .nat .unit)) [(.build .some (.option .nat) [(.build .succ .nat [(.var 0)])]), (.cond (.prim .natLt [(.var 0), (.value .nat (.nat 3))]) (.build .ok (.result .nat .unit) [(.build .zero .nat [])]) (.build .error (.result .nat .unit) [(.build .unit .unit [])]))]) }] }

def krate : RustSyntax.Crate :=
  ({ profile := .core, items := [
    (.function (.generated .function 0) [{ pattern := (.bind (.generated .binding 0)), type := .nat }] (.fallible (.pair (.option .nat) (.result .nat .unit))) (.mk [] (.succeed (.pair (.construct .some [(.call (.runtime .natSucc) [(.copy (.generated .binding 0))] true)]) (.block (.mk [(.mk (.bind (.generated .operand 1)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 2)) none (.lit (.nat 3)))] (.cond (.call (.runtime .natLt) [(.move (.generated .operand 1)), (.move (.generated .operand 2))] false) (.mk [] (.construct (.ok .nat .unit) [(.lit (.nat 0))])) (.mk [] (.construct (.err .nat .unit) [(.lit .unit)])))))))))] } : LexLeanTarget.RustSyntax.Crate)

def flags : Flags := []

theorem fun0 : FunOK program krate flags 0 :=
  ⟨_, rfl, .inl ⟨_, _, [], (.succeed (.pair (.construct .some [(.call (.runtime .natSucc) [(.copy (.generated .binding 0))] true)]) (.block (.mk [(.mk (.bind (.generated .operand 1)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 2)) none (.lit (.nat 3)))] (.cond (.call (.runtime .natLt) [(.move (.generated .operand 1)), (.move (.generated .operand 2))] false) (.mk [] (.construct (.ok .nat .unit) [(.lit (.nat 0))])) (.mk [] (.construct (.err .nat .unit) [(.lit .unit)]))))))), [(0, .nat, (some 0))], rfl, rfl,
    (Corr.fOfB (Corr.bOfE (Corr.buildPair (Corr.lCons (Corr.eOfBNil (Corr.build (ts := [.nat]) (mask := [false]) (Corr.opsOfL (Corr.lCons (Corr.buildSucc (Corr.lCons (Corr.var rfl rfl) (Corr.lNil))) (Corr.lNil))) rfl)) (Corr.lCons (Corr.eOfB (lets := [(.mk (.bind (.generated .operand 1)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 2)) none (.lit (.nat 3)))]) (tail := (.cond (.call (.runtime .natLt) [(.move (.generated .operand 1)), (.move (.generated .operand 2))] false) (.mk [] (.construct (.ok .nat .unit) [(.lit (.nat 0))])) (.mk [] (.construct (.err .nat .unit) [(.lit .unit)])))) (Corr.cond (lc := [(.mk (.bind (.generated .operand 1)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 2)) none (.lit (.nat 3)))]) (ce := (.call (.runtime .natLt) [(.move (.generated .operand 1)), (.move (.generated .operand 2))] false)) (lk := []) (Corr.prim (ts := [.nat, .nat]) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsCons (Corr.value rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl) (Corr.kIf (la := []) (ta := (.construct (.ok .nat .unit) [(.lit (.nat 0))])) (lb := []) (tb := (.construct (.err .nat .unit) [(.lit .unit)])) (Corr.build (ts := [.nat]) (mask := [false]) (Corr.opsOfL (Corr.lCons (Corr.buildConst rfl) (Corr.lNil))) rfl) (Corr.build (ts := [.unit]) (mask := [false]) (Corr.opsOfL (Corr.lCons (Corr.buildConst rfl) (Corr.lNil))) rfl)))) (Corr.lNil))))))⟩⟩

theorem crate_ok : CrateOK program krate flags := by
  intro f fn h
  match f with
  | 0 => exact fun0
  | _ + 1 => simp [program, index_eq] at h

/-- Every function of the program is simulated by its rendering. -/
theorem root : ∀ n f, FunSem program krate flags n f := fun n f => simulate crate_ok n f

end LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R16.RustCore
