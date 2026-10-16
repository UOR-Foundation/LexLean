import LexLeanPreservation.RustSound
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 0
set_option synthInstance.maxHeartbeats 0
namespace LexLeanPreserve.Cc8b321d929900ecdc951104ec4ac0984.R0.RustStd
open LexLeanTarget LexLeanPreservation.Rust

noncomputable def program : TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [.bytes, .bytes], result := .bool,
      body := (.«match» .bool (.prim .slice [(.var 0), (.value .nat (.nat 0)), (.prim .length [(.var 1)])]) [(.arm .some [2] (.prim .equal [(.var 2), (.var 1)])), (.arm .none [] (.build .false .bool []))]) }] }

noncomputable def krate : RustSyntax.Crate :=
  ({ profile := .std, items := [
    (.function (.generated .function 0) [{ pattern := (.bind (.generated .binding 0)), type := .bytes }, { pattern := (.bind (.generated .binding 1)), type := .bytes }] .bool (.mk [(.mk (.bind (.generated .holder 5)) none (.block (.mk [(.mk (.bind (.generated .operand 2)) none (.move (.generated .binding 0))), (.mk (.bind (.generated .operand 3)) none (.lit (.nat 0))), (.mk (.bind (.generated .operand 4)) none (.block (.mk [(.mk (.bind (.generated .operand 1)) none (.clone (.generated .binding 1)))] (.call (.runtime .lengthBytes) [(.move (.generated .operand 1))] false))))] (.call (.runtime .sliceBytes) [(.move (.generated .operand 2)), (.move (.generated .operand 3)), (.move (.generated .operand 4))] false))))] (.matchOn (.move (.generated .holder 5)) [(.mk .none (.mk [] (.lit (.bool false)))), (.mk (.some (.bind (.generated .binding 2))) (.mk [(.mk (.bind (.generated .operand 6)) none (.move (.generated .binding 2))), (.mk (.bind (.generated .operand 7)) none (.move (.generated .binding 1)))] (.call (.runtime .equal) [(.move (.generated .operand 6)), (.move (.generated .operand 7))] false)))])))] } : LexLeanTarget.RustSyntax.Crate)

def flags : Flags := []

theorem fun0 : FunOK program krate flags 0 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .holder 5)) none (.block (.mk [(.mk (.bind (.generated .operand 2)) none (.move (.generated .binding 0))), (.mk (.bind (.generated .operand 3)) none (.lit (.nat 0))), (.mk (.bind (.generated .operand 4)) none (.block (.mk [(.mk (.bind (.generated .operand 1)) none (.clone (.generated .binding 1)))] (.call (.runtime .lengthBytes) [(.move (.generated .operand 1))] false))))] (.call (.runtime .sliceBytes) [(.move (.generated .operand 2)), (.move (.generated .operand 3)), (.move (.generated .operand 4))] false))))], (.matchOn (.move (.generated .holder 5)) [(.mk .none (.mk [] (.lit (.bool false)))), (.mk (.some (.bind (.generated .binding 2))) (.mk [(.mk (.bind (.generated .operand 6)) none (.move (.generated .binding 2))), (.mk (.bind (.generated .operand 7)) none (.move (.generated .binding 1)))] (.call (.runtime .equal) [(.move (.generated .operand 6)), (.move (.generated .operand 7))] false)))]), [(1, .bytes, (some 1)), (0, .bytes, (some 0))], rfl, rfl,
    (Corr.matchArms (st := (.option .bytes)) (U := []) (view := false) (idx := [1, 0]) (Corr.eOfB (lets := [(.mk (.bind (.generated .operand 2)) none (.move (.generated .binding 0))), (.mk (.bind (.generated .operand 3)) none (.lit (.nat 0))), (.mk (.bind (.generated .operand 4)) none (.block (.mk [(.mk (.bind (.generated .operand 1)) none (.clone (.generated .binding 1)))] (.call (.runtime .lengthBytes) [(.move (.generated .operand 1))] false))))]) (tail := (.call (.runtime .sliceBytes) [(.move (.generated .operand 2)), (.move (.generated .operand 3)), (.move (.generated .operand 4))] false)) (Corr.prim (ts := [.bytes, .nat, .nat]) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsCons (Corr.value rfl rfl) (Corr.opsCons (Corr.eOfB (lets := [(.mk (.bind (.generated .operand 1)) none (.clone (.generated .binding 1)))]) (tail := (.call (.runtime .lengthBytes) [(.move (.generated .operand 1))] false)) (Corr.prim (ts := [.bytes]) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl rfl rfl rfl rfl)) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl)) (Corr.mCons (Γ' := [(1, .bytes, (some 1)), (0, .bytes, (some 0))]) (j := 1) (loads := []) (lets := []) (tail := (.lit (.bool false))) (Corr.mCons (Γ' := [(2, .bytes, (some 2)), (1, .bytes, (some 1)), (0, .bytes, (some 0))]) (j := 0) (loads := []) (lets := [(.mk (.bind (.generated .operand 6)) none (.move (.generated .binding 2))), (.mk (.bind (.generated .operand 7)) none (.move (.generated .binding 1)))]) (tail := (.call (.runtime .equal) [(.move (.generated .operand 6)), (.move (.generated .operand 7))] false)) (Corr.mNil) rfl rfl rfl (Corr.prim (ts := [.bytes, .bytes]) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl)) rfl rfl rfl (Corr.bOfE (Corr.buildConst rfl))) rfl rfl rfl)⟩⟩

theorem crate_ok : CrateOK program krate flags := by
  intro f fn h
  match f with
  | 0 => exact fun0
  | _ + 1 => simp [program, index_eq] at h

/-- Every function of the program is simulated by its rendering. -/
theorem root : ∀ n f, FunSem program krate flags n f := fun n f => simulate crate_ok n f

end LexLeanPreserve.Cc8b321d929900ecdc951104ec4ac0984.R0.RustStd
