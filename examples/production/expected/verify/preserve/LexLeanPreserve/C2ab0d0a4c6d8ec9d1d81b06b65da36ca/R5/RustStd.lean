import LexLeanPreservation.RustSound
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C2ab0d0a4c6d8ec9d1d81b06b65da36ca.R5.RustStd
open LexLeanTarget LexLeanPreservation.Rust

def program : TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0], types := [(.list .nat)], result := .nat,
      body := (.call 1 [(.var 0), (.value .nat (.nat 0))]) },
    { parameters := [0, 1], types := [(.list .nat), .nat], result := .nat,
      body := (.«match» .nat (.var 0) [(.arm .nil [] (.var 1)), (.arm .cons [2, 3] (.var 2))]) }] }

def krate : RustSyntax.Crate :=
  ({ profile := .std, items := [
    (.function (.generated .function 0) [{ pattern := (.bind (.generated .binding 0)), type := (.list .nat) }] .nat (.mk [] (.call (.function 1) [(.clone (.generated .binding 0)), (.lit (.nat 0))] false))),
    (.function (.generated .function 1) [{ pattern := (.bind (.generated .binding 0)), type := (.list .nat) }, { pattern := (.bind (.generated .binding 1)), type := .nat }] .nat (.mk [(.mk (.bind (.generated .holder 1)) none (.clone (.generated .binding 0)))] (.matchOn (.uncons (.generated .holder 1)) [(.mk .none (.mk [] (.copy (.generated .binding 1)))), (.mk (.some (.tuple [(.bind (.generated .binding 2)), .wild])) (.mk [] (.copy (.generated .binding 2))))])))] } : LexLeanTarget.RustSyntax.Crate)

def flags : Flags := []

theorem fun0 : FunOK program krate flags 0 :=
  ⟨_, rfl, .inl ⟨_, _, [], (.call (.function 1) [(.clone (.generated .binding 0)), (.lit (.nat 0))] false), [(0, (.list .nat), (some 0))], rfl, rfl,
    (Corr.callOps (ts := [(.list .nat), .nat]) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lCons (Corr.value rfl rfl) (Corr.lNil)))) rfl rfl rfl rfl rfl)⟩⟩

theorem fun1 : FunOK program krate flags 1 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .holder 1)) none (.clone (.generated .binding 0)))], (.matchOn (.uncons (.generated .holder 1)) [(.mk .none (.mk [] (.copy (.generated .binding 1)))), (.mk (.some (.tuple [(.bind (.generated .binding 2)), .wild])) (.mk [] (.copy (.generated .binding 2))))]), [(1, .nat, (some 1)), (0, (.list .nat), (some 0))], rfl, rfl,
    (Corr.matchArms (st := (.list .nat)) (U := []) (view := true) (idx := [0, 1]) (Corr.var rfl rfl) (Corr.mCons (Γ' := [(1, .nat, (some 1)), (0, (.list .nat), (some 0))]) (j := 0) (loads := []) (lets := []) (tail := (.copy (.generated .binding 1))) (Corr.mCons (Γ' := [(3, (.list .nat), none), (2, .nat, (some 2)), (1, .nat, (some 1)), (0, (.list .nat), (some 0))]) (j := 1) (loads := []) (lets := []) (tail := (.copy (.generated .binding 2))) (Corr.mNil) rfl rfl rfl (Corr.bOfE (Corr.var rfl rfl))) rfl rfl rfl (Corr.bOfE (Corr.var rfl rfl))) rfl rfl rfl)⟩⟩

theorem crate_ok : CrateOK program krate flags := by
  intro f fn h
  match f with
  | 0 => exact fun0
  | 1 => exact fun1
  | _ + 2 => simp [program, index_eq] at h

/-- Every function of the program is simulated by its rendering. -/
theorem root : ∀ n f, FunSem program krate flags n f := fun n f => simulate crate_ok n f

end LexLeanPreserve.C2ab0d0a4c6d8ec9d1d81b06b65da36ca.R5.RustStd
