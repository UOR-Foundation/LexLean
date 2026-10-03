import LexLeanPreservation.RustSound
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R4.RustStd
open LexLeanTarget LexLeanPreservation.Rust

def program : TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0], types := [(.list .nat)], result := .nat,
      body := (.call 1 [(.var 0)]) },
    { parameters := [0], types := [(.list .nat)], result := .nat,
      body := (.«match» .nat (.var 0) [(.arm .nil [] (.value .nat (.nat 0))), (.arm .cons [1, 2] (.prim .natAdd [(.var 1), (.call 1 [(.var 2)])]))]) }] }

def krate : RustSyntax.Crate :=
  ({ profile := .std, items := [
    (.function (.generated .function 0) [{ pattern := (.bind (.generated .binding 0)), type := (.list .nat) }] (.fallible .nat) (.mk [] (.call (.function 1) [(.clone (.generated .binding 0))] false))),
    (.function (.generated .function 1) [{ pattern := (.bind (.generated .binding 0)), type := (.list .nat) }] (.fallible .nat) (.mk [(.mk (.bind (.generated .holder 1)) none (.clone (.generated .binding 0)))] (.matchOn (.uncons (.generated .holder 1)) [(.mk .none (.mk [] (.succeed (.lit (.nat 0))))), (.mk (.some (.tuple [(.bind (.generated .binding 1)), (.bind (.generated .binding 2))])) (.mk [(.mk (.bind (.generated .operand 2)) none (.copy (.generated .binding 1))), (.mk (.bind (.generated .operand 3)) none (.call (.function 1) [(.clone (.generated .binding 2))] true))] (.call (.runtime .natAdd) [(.move (.generated .operand 2)), (.move (.generated .operand 3))] false)))])))] } : LexLeanTarget.RustSyntax.Crate)

def flags : Flags := []

theorem fun0 : FunOK program krate flags 0 :=
  ⟨_, rfl, .inl ⟨_, _, [], (.call (.function 1) [(.clone (.generated .binding 0))] false), [(0, (.list .nat), (some 0))], rfl, rfl,
    (Corr.callF (ts := [(.list .nat)]) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lNil))) rfl rfl rfl rfl)⟩⟩

theorem fun1 : FunOK program krate flags 1 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .holder 1)) none (.clone (.generated .binding 0)))], (.matchOn (.uncons (.generated .holder 1)) [(.mk .none (.mk [] (.succeed (.lit (.nat 0))))), (.mk (.some (.tuple [(.bind (.generated .binding 1)), (.bind (.generated .binding 2))])) (.mk [(.mk (.bind (.generated .operand 2)) none (.copy (.generated .binding 1))), (.mk (.bind (.generated .operand 3)) none (.call (.function 1) [(.clone (.generated .binding 2))] true))] (.call (.runtime .natAdd) [(.move (.generated .operand 2)), (.move (.generated .operand 3))] false)))]), [(0, (.list .nat), (some 0))], rfl, rfl,
    (Corr.matchArms (st := (.list .nat)) (U := []) (view := true) (idx := [0, 1]) (Corr.var rfl rfl) (Corr.mCons (Γ' := [(0, (.list .nat), (some 0))]) (j := 0) (loads := []) (lets := []) (tail := (.succeed (.lit (.nat 0)))) (Corr.mCons (Γ' := [(2, (.list .nat), (some 2)), (1, .nat, (some 1)), (0, (.list .nat), (some 0))]) (j := 1) (loads := []) (lets := [(.mk (.bind (.generated .operand 2)) none (.copy (.generated .binding 1))), (.mk (.bind (.generated .operand 3)) none (.call (.function 1) [(.clone (.generated .binding 2))] true))]) (tail := (.call (.runtime .natAdd) [(.move (.generated .operand 2)), (.move (.generated .operand 3))] false)) (Corr.mNil) rfl rfl rfl (Corr.primF (ts := [.nat, .nat]) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsCons (Corr.eOfBNil (Corr.callOps (ts := [(.list .nat)]) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lNil))) rfl rfl rfl rfl rfl)) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl)) rfl rfl rfl (Corr.fOfB (Corr.bOfE (Corr.value rfl rfl)))) rfl rfl rfl)⟩⟩

theorem crate_ok : CrateOK program krate flags := by
  intro f fn h
  match f with
  | 0 => exact fun0
  | 1 => exact fun1
  | _ + 2 => simp [program, index_eq] at h

/-- Every function of the program is simulated by its rendering. -/
theorem root : ∀ n f, FunSem program krate flags n f := fun n f => simulate crate_ok n f

end LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R4.RustStd
