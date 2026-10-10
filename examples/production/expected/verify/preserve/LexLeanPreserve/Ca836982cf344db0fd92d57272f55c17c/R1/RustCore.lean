import LexLeanPreservation.RustSound
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.Ca836982cf344db0fd92d57272f55c17c.R1.RustCore
open LexLeanTarget LexLeanPreservation.Rust

def program : TargetSyntax.Program :=
  { adts := [{ constructors := [[.nat], [.nat, .nat]] }], functions := [
    { parameters := [0, 1], types := [.nat, .nat], result := .nat,
      body := (.call 1 [(.build (.adt 1) (.adt 0) [(.var 0), (.var 1)])]) },
    { parameters := [0], types := [(.adt 0)], result := .nat,
      body := (.«match» .nat (.var 0) [(.arm (.adt 0) [1] (.prim .natMul [(.prim .natMul [(.var 1), (.var 1)]), (.value .nat (.nat 3))])), (.arm (.adt 1) [2, 3] (.prim .natMul [(.var 2), (.var 3)]))]) }] }

def krate : RustSyntax.Crate :=
  ({ profile := .core, items := [
    (.enum (.adt 0) [{ number := 0, fields := [.nat] }, { number := 1, fields := [.nat, .nat] }]),
    (.function (.generated .function 0) [{ pattern := (.bind (.generated .binding 0)), type := .nat }, { pattern := (.bind (.generated .binding 1)), type := .nat }] (.fallible .nat) (.mk [] (.call (.function 1) [(.construct (.adt 0 1) [(.copy (.generated .binding 0)), (.copy (.generated .binding 1))])] false))),
    (.function (.generated .function 1) [{ pattern := (.bind (.generated .binding 0)), type := (.adt 0) }] (.fallible .nat) (.mk [(.mk (.bind (.generated .holder 1)) none (.clone (.generated .binding 0)))] (.matchOn (.move (.generated .holder 1)) [(.mk (.adt 0 0 [(.bind (.generated .binding 1))]) (.mk [(.mk (.bind (.generated .operand 4)) none (.block (.mk [(.mk (.bind (.generated .operand 2)) none (.copy (.generated .binding 1))), (.mk (.bind (.generated .operand 3)) none (.copy (.generated .binding 1)))] (.call (.runtime .natMul) [(.move (.generated .operand 2)), (.move (.generated .operand 3))] true)))), (.mk (.bind (.generated .operand 5)) none (.lit (.nat 3)))] (.call (.runtime .natMul) [(.move (.generated .operand 4)), (.move (.generated .operand 5))] false))), (.mk (.adt 0 1 [(.bind (.generated .binding 2)), (.bind (.generated .binding 3))]) (.mk [(.mk (.bind (.generated .operand 6)) none (.copy (.generated .binding 2))), (.mk (.bind (.generated .operand 7)) none (.copy (.generated .binding 3)))] (.call (.runtime .natMul) [(.move (.generated .operand 6)), (.move (.generated .operand 7))] false)))])))] } : LexLeanTarget.RustSyntax.Crate)

def flags : Flags := []

theorem fun0 : FunOK program krate flags 0 :=
  ⟨_, rfl, .inl ⟨_, _, [], (.call (.function 1) [(.construct (.adt 0 1) [(.copy (.generated .binding 0)), (.copy (.generated .binding 1))])] false), [(1, .nat, (some 1)), (0, .nat, (some 0))], rfl, rfl,
    (Corr.callF (ts := [(.adt 0)]) (Corr.opsOfL (Corr.lCons (Corr.eOfBNil (Corr.build (ts := [.nat, .nat]) (mask := [false, false]) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lCons (Corr.var rfl rfl) (Corr.lNil)))) rfl)) (Corr.lNil))) rfl rfl rfl rfl)⟩⟩

theorem fun1 : FunOK program krate flags 1 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .holder 1)) none (.clone (.generated .binding 0)))], (.matchOn (.move (.generated .holder 1)) [(.mk (.adt 0 0 [(.bind (.generated .binding 1))]) (.mk [(.mk (.bind (.generated .operand 4)) none (.block (.mk [(.mk (.bind (.generated .operand 2)) none (.copy (.generated .binding 1))), (.mk (.bind (.generated .operand 3)) none (.copy (.generated .binding 1)))] (.call (.runtime .natMul) [(.move (.generated .operand 2)), (.move (.generated .operand 3))] true)))), (.mk (.bind (.generated .operand 5)) none (.lit (.nat 3)))] (.call (.runtime .natMul) [(.move (.generated .operand 4)), (.move (.generated .operand 5))] false))), (.mk (.adt 0 1 [(.bind (.generated .binding 2)), (.bind (.generated .binding 3))]) (.mk [(.mk (.bind (.generated .operand 6)) none (.copy (.generated .binding 2))), (.mk (.bind (.generated .operand 7)) none (.copy (.generated .binding 3)))] (.call (.runtime .natMul) [(.move (.generated .operand 6)), (.move (.generated .operand 7))] false)))]), [(0, (.adt 0), (some 0))], rfl, rfl,
    (Corr.matchArms (st := (.adt 0)) (U := []) (view := false) (idx := [0, 1]) (Corr.var rfl rfl) (Corr.mCons (Γ' := [(1, .nat, (some 1)), (0, (.adt 0), (some 0))]) (j := 0) (loads := []) (lets := [(.mk (.bind (.generated .operand 4)) none (.block (.mk [(.mk (.bind (.generated .operand 2)) none (.copy (.generated .binding 1))), (.mk (.bind (.generated .operand 3)) none (.copy (.generated .binding 1)))] (.call (.runtime .natMul) [(.move (.generated .operand 2)), (.move (.generated .operand 3))] true)))), (.mk (.bind (.generated .operand 5)) none (.lit (.nat 3)))]) (tail := (.call (.runtime .natMul) [(.move (.generated .operand 4)), (.move (.generated .operand 5))] false)) (Corr.mCons (Γ' := [(3, .nat, (some 3)), (2, .nat, (some 2)), (0, (.adt 0), (some 0))]) (j := 1) (loads := []) (lets := [(.mk (.bind (.generated .operand 6)) none (.copy (.generated .binding 2))), (.mk (.bind (.generated .operand 7)) none (.copy (.generated .binding 3)))]) (tail := (.call (.runtime .natMul) [(.move (.generated .operand 6)), (.move (.generated .operand 7))] false)) (Corr.mNil) rfl rfl rfl (Corr.primF (ts := [.nat, .nat]) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl)) rfl rfl rfl (Corr.primF (ts := [.nat, .nat]) (Corr.opsCons (Corr.eOfB (lets := [(.mk (.bind (.generated .operand 2)) none (.copy (.generated .binding 1))), (.mk (.bind (.generated .operand 3)) none (.copy (.generated .binding 1)))]) (tail := (.call (.runtime .natMul) [(.move (.generated .operand 2)), (.move (.generated .operand 3))] true)) (Corr.prim (ts := [.nat, .nat]) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl)) (Corr.opsCons (Corr.value rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl)) rfl rfl rfl)⟩⟩

theorem crate_ok : CrateOK program krate flags := by
  intro f fn h
  match f with
  | 0 => exact fun0
  | 1 => exact fun1
  | _ + 2 => simp [program, index_eq] at h

/-- Every function of the program is simulated by its rendering. -/
theorem root : ∀ n f, FunSem program krate flags n f := fun n f => simulate crate_ok n f

end LexLeanPreserve.Ca836982cf344db0fd92d57272f55c17c.R1.RustCore
