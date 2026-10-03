import LexLeanPreservation.RustSound
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.Cba27a3dc52c63de8b08b0726788d5c73.R14.RustStd
open LexLeanTarget LexLeanPreservation.Rust

def program : TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [(.option .nat), (.result .nat .bool)], result := .nat,
      body := (.prim .natAdd [(.«match» .nat (.var 0) [(.arm .none [] (.value .nat (.nat 0))), (.arm .some [2] (.var 2))]), (.«match» .nat (.var 1) [(.arm .ok [3] (.var 3)), (.arm .error [4] (.cond (.var 4) (.value .nat (.nat 1)) (.value .nat (.nat 2))))])]) }] }

def krate : RustSyntax.Crate :=
  ({ profile := .std, items := [
    (.function (.generated .function 0) [{ pattern := (.bind (.generated .binding 0)), type := (.option .nat) }, { pattern := (.bind (.generated .binding 1)), type := (.result .nat .bool) }] (.fallible .nat) (.mk [(.mk (.bind (.generated .operand 3)) none (.block (.mk [(.mk (.bind (.generated .holder 1)) none (.copy (.generated .binding 0)))] (.matchOn (.move (.generated .holder 1)) [(.mk .none (.mk [] (.lit (.nat 0)))), (.mk (.some (.bind (.generated .binding 2))) (.mk [] (.copy (.generated .binding 2))))])))), (.mk (.bind (.generated .operand 4)) none (.block (.mk [(.mk (.bind (.generated .holder 2)) none (.copy (.generated .binding 1)))] (.matchOn (.move (.generated .holder 2)) [(.mk (.ok (.bind (.generated .binding 3))) (.mk [] (.copy (.generated .binding 3)))), (.mk (.err (.bind (.generated .binding 4))) (.mk [] (.cond (.copy (.generated .binding 4)) (.mk [] (.lit (.nat 1))) (.mk [] (.lit (.nat 2))))))]))))] (.call (.runtime .natAdd) [(.move (.generated .operand 3)), (.move (.generated .operand 4))] false)))] } : LexLeanTarget.RustSyntax.Crate)

def flags : Flags := []

theorem fun0 : FunOK program krate flags 0 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .operand 3)) none (.block (.mk [(.mk (.bind (.generated .holder 1)) none (.copy (.generated .binding 0)))] (.matchOn (.move (.generated .holder 1)) [(.mk .none (.mk [] (.lit (.nat 0)))), (.mk (.some (.bind (.generated .binding 2))) (.mk [] (.copy (.generated .binding 2))))])))), (.mk (.bind (.generated .operand 4)) none (.block (.mk [(.mk (.bind (.generated .holder 2)) none (.copy (.generated .binding 1)))] (.matchOn (.move (.generated .holder 2)) [(.mk (.ok (.bind (.generated .binding 3))) (.mk [] (.copy (.generated .binding 3)))), (.mk (.err (.bind (.generated .binding 4))) (.mk [] (.cond (.copy (.generated .binding 4)) (.mk [] (.lit (.nat 1))) (.mk [] (.lit (.nat 2))))))]))))], (.call (.runtime .natAdd) [(.move (.generated .operand 3)), (.move (.generated .operand 4))] false), [(1, (.result .nat .bool), (some 1)), (0, (.option .nat), (some 0))], rfl, rfl,
    (Corr.primF (ts := [.nat, .nat]) (Corr.opsCons (Corr.eOfB (lets := [(.mk (.bind (.generated .holder 1)) none (.copy (.generated .binding 0)))]) (tail := (.matchOn (.move (.generated .holder 1)) [(.mk .none (.mk [] (.lit (.nat 0)))), (.mk (.some (.bind (.generated .binding 2))) (.mk [] (.copy (.generated .binding 2))))])) (Corr.matchArms (st := (.option .nat)) (U := []) (view := false) (idx := [0, 1]) (Corr.var rfl rfl) (Corr.mCons (Γ' := [(1, (.result .nat .bool), (some 1)), (0, (.option .nat), (some 0))]) (j := 0) (loads := []) (lets := []) (tail := (.lit (.nat 0))) (Corr.mCons (Γ' := [(2, .nat, (some 2)), (1, (.result .nat .bool), (some 1)), (0, (.option .nat), (some 0))]) (j := 1) (loads := []) (lets := []) (tail := (.copy (.generated .binding 2))) (Corr.mNil) rfl rfl rfl (Corr.bOfE (Corr.var rfl rfl))) rfl rfl rfl (Corr.bOfE (Corr.value rfl rfl))) rfl rfl rfl)) (Corr.opsCons (Corr.eOfB (lets := [(.mk (.bind (.generated .holder 2)) none (.copy (.generated .binding 1)))]) (tail := (.matchOn (.move (.generated .holder 2)) [(.mk (.ok (.bind (.generated .binding 3))) (.mk [] (.copy (.generated .binding 3)))), (.mk (.err (.bind (.generated .binding 4))) (.mk [] (.cond (.copy (.generated .binding 4)) (.mk [] (.lit (.nat 1))) (.mk [] (.lit (.nat 2))))))])) (Corr.matchArms (st := (.result .nat .bool)) (U := []) (view := false) (idx := [0, 1]) (Corr.var rfl rfl) (Corr.mCons (Γ' := [(3, .nat, (some 3)), (1, (.result .nat .bool), (some 1)), (0, (.option .nat), (some 0))]) (j := 0) (loads := []) (lets := []) (tail := (.copy (.generated .binding 3))) (Corr.mCons (Γ' := [(4, .bool, (some 4)), (1, (.result .nat .bool), (some 1)), (0, (.option .nat), (some 0))]) (j := 1) (loads := []) (lets := []) (tail := (.cond (.copy (.generated .binding 4)) (.mk [] (.lit (.nat 1))) (.mk [] (.lit (.nat 2))))) (Corr.mNil) rfl rfl rfl (Corr.cond (lc := []) (ce := (.copy (.generated .binding 4))) (lk := []) (Corr.bOfE (Corr.var rfl rfl)) (Corr.kIf (la := []) (ta := (.lit (.nat 1))) (lb := []) (tb := (.lit (.nat 2))) (Corr.bOfE (Corr.value rfl rfl)) (Corr.bOfE (Corr.value rfl rfl))))) rfl rfl rfl (Corr.bOfE (Corr.var rfl rfl))) rfl rfl rfl)) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl)⟩⟩

theorem crate_ok : CrateOK program krate flags := by
  intro f fn h
  match f with
  | 0 => exact fun0
  | _ + 1 => simp [program, index_eq] at h

/-- Every function of the program is simulated by its rendering. -/
theorem root : ∀ n f, FunSem program krate flags n f := fun n f => simulate crate_ok n f

end LexLeanPreserve.Cba27a3dc52c63de8b08b0726788d5c73.R14.RustStd
