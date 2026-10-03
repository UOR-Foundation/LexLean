import LexLeanPreservation.RustSound
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.Cba27a3dc52c63de8b08b0726788d5c73.R29.RustStd
open LexLeanTarget LexLeanPreservation.Rust

def program : TargetSyntax.Program :=
  { adts := [{ constructors := [[.nat, (.list (.adt 0))]] }], functions := [
    { parameters := [0], types := [.nat], result := .nat,
      body := (.call 1 [(.build (.adt 0) (.adt 0) [(.var 0), (.build .cons (.list (.adt 0)) [(.build (.adt 0) (.adt 0) [(.value .nat (.nat 1)), (.build .nil (.list (.adt 0)) [])]), (.build .nil (.list (.adt 0)) [])])])]) },
    { parameters := [0], types := [(.adt 0)], result := .nat,
      body := (.«match» .nat (.var 0) [(.arm (.adt 0) [1, 2] (.prim .natAdd [(.value .nat (.nat 1)), (.call 2 [(.var 2)])]))]) },
    { parameters := [0], types := [(.list (.adt 0))], result := .nat,
      body := (.«match» .nat (.var 0) [(.arm .nil [] (.value .nat (.nat 0))), (.arm .cons [1, 2] (.prim .natAdd [(.call 1 [(.var 1)]), (.call 2 [(.var 2)])]))]) }] }

def krate : RustSyntax.Crate :=
  ({ profile := .std, items := [
    (.enum (.adt 0) [{ number := 0, fields := [.nat, (.list (.adt 0))] }]),
    (.function (.generated .function 0) [{ pattern := (.bind (.generated .binding 0)), type := .nat }] (.fallible .nat) (.mk [] (.call (.function 1) [(.construct (.adt 0 0) [(.copy (.generated .binding 0)), (.construct .cons [(.construct (.adt 0 0) [(.lit (.nat 1)), (.construct (.nil (.adt 0)) [])]), (.construct (.nil (.adt 0)) [])])])] false))),
    (.function (.generated .function 1) [{ pattern := (.bind (.generated .binding 0)), type := (.adt 0) }] (.fallible .nat) (.mk [(.mk (.bind (.generated .holder 1)) none (.clone (.generated .binding 0)))] (.matchOn (.move (.generated .holder 1)) [(.mk (.adt 0 0 [.wild, (.bind (.generated .binding 2))]) (.mk [(.mk (.bind (.generated .operand 2)) none (.lit (.nat 1))), (.mk (.bind (.generated .operand 3)) none (.call (.function 2) [(.clone (.generated .binding 2))] true))] (.call (.runtime .natAdd) [(.move (.generated .operand 2)), (.move (.generated .operand 3))] false)))]))),
    (.function (.generated .function 2) [{ pattern := (.bind (.generated .binding 0)), type := (.list (.adt 0)) }] (.fallible .nat) (.mk [(.mk (.bind (.generated .holder 4)) none (.clone (.generated .binding 0)))] (.matchOn (.uncons (.generated .holder 4)) [(.mk .none (.mk [] (.succeed (.lit (.nat 0))))), (.mk (.some (.tuple [(.bind (.generated .binding 1)), (.bind (.generated .binding 2))])) (.mk [(.mk (.bind (.generated .operand 5)) none (.call (.function 1) [(.clone (.generated .binding 1))] true)), (.mk (.bind (.generated .operand 6)) none (.call (.function 2) [(.clone (.generated .binding 2))] true))] (.call (.runtime .natAdd) [(.move (.generated .operand 5)), (.move (.generated .operand 6))] false)))])))] } : LexLeanTarget.RustSyntax.Crate)

def flags : Flags := []

theorem fun0 : FunOK program krate flags 0 :=
  ⟨_, rfl, .inl ⟨_, _, [], (.call (.function 1) [(.construct (.adt 0 0) [(.copy (.generated .binding 0)), (.construct .cons [(.construct (.adt 0 0) [(.lit (.nat 1)), (.construct (.nil (.adt 0)) [])]), (.construct (.nil (.adt 0)) [])])])] false), [(0, .nat, (some 0))], rfl, rfl,
    (Corr.callF (ts := [(.adt 0)]) (Corr.opsOfL (Corr.lCons (Corr.eOfBNil (Corr.build (ts := [.nat, (.list (.adt 0))]) (mask := [false, false]) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lCons (Corr.eOfBNil (Corr.build (ts := [(.adt 0), (.list (.adt 0))]) (mask := [false, false]) (Corr.opsOfL (Corr.lCons (Corr.eOfBNil (Corr.build (ts := [.nat, (.list (.adt 0))]) (mask := [false, false]) (Corr.opsOfL (Corr.lCons (Corr.value rfl rfl) (Corr.lCons (Corr.eOfBNil (Corr.build (ts := []) (mask := []) (Corr.opsNil) rfl)) (Corr.lNil)))) rfl)) (Corr.lCons (Corr.eOfBNil (Corr.build (ts := []) (mask := []) (Corr.opsNil) rfl)) (Corr.lNil)))) rfl)) (Corr.lNil)))) rfl)) (Corr.lNil))) rfl rfl rfl rfl)⟩⟩

theorem fun1 : FunOK program krate flags 1 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .holder 1)) none (.clone (.generated .binding 0)))], (.matchOn (.move (.generated .holder 1)) [(.mk (.adt 0 0 [.wild, (.bind (.generated .binding 2))]) (.mk [(.mk (.bind (.generated .operand 2)) none (.lit (.nat 1))), (.mk (.bind (.generated .operand 3)) none (.call (.function 2) [(.clone (.generated .binding 2))] true))] (.call (.runtime .natAdd) [(.move (.generated .operand 2)), (.move (.generated .operand 3))] false)))]), [(0, (.adt 0), (some 0))], rfl, rfl,
    (Corr.matchArms (st := (.adt 0)) (U := []) (view := false) (idx := [0]) (Corr.var rfl rfl) (Corr.mCons (Γ' := [(2, (.list (.adt 0)), (some 2)), (1, .nat, none), (0, (.adt 0), (some 0))]) (j := 0) (loads := []) (lets := [(.mk (.bind (.generated .operand 2)) none (.lit (.nat 1))), (.mk (.bind (.generated .operand 3)) none (.call (.function 2) [(.clone (.generated .binding 2))] true))]) (tail := (.call (.runtime .natAdd) [(.move (.generated .operand 2)), (.move (.generated .operand 3))] false)) (Corr.mNil) rfl rfl rfl (Corr.primF (ts := [.nat, .nat]) (Corr.opsCons (Corr.value rfl rfl) (Corr.opsCons (Corr.eOfBNil (Corr.callOps (ts := [(.list (.adt 0))]) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lNil))) rfl rfl rfl rfl rfl)) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl)) rfl rfl rfl)⟩⟩

theorem fun2 : FunOK program krate flags 2 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .holder 4)) none (.clone (.generated .binding 0)))], (.matchOn (.uncons (.generated .holder 4)) [(.mk .none (.mk [] (.succeed (.lit (.nat 0))))), (.mk (.some (.tuple [(.bind (.generated .binding 1)), (.bind (.generated .binding 2))])) (.mk [(.mk (.bind (.generated .operand 5)) none (.call (.function 1) [(.clone (.generated .binding 1))] true)), (.mk (.bind (.generated .operand 6)) none (.call (.function 2) [(.clone (.generated .binding 2))] true))] (.call (.runtime .natAdd) [(.move (.generated .operand 5)), (.move (.generated .operand 6))] false)))]), [(0, (.list (.adt 0)), (some 0))], rfl, rfl,
    (Corr.matchArms (st := (.list (.adt 0))) (U := []) (view := true) (idx := [0, 1]) (Corr.var rfl rfl) (Corr.mCons (Γ' := [(0, (.list (.adt 0)), (some 0))]) (j := 0) (loads := []) (lets := []) (tail := (.succeed (.lit (.nat 0)))) (Corr.mCons (Γ' := [(2, (.list (.adt 0)), (some 2)), (1, (.adt 0), (some 1)), (0, (.list (.adt 0)), (some 0))]) (j := 1) (loads := []) (lets := [(.mk (.bind (.generated .operand 5)) none (.call (.function 1) [(.clone (.generated .binding 1))] true)), (.mk (.bind (.generated .operand 6)) none (.call (.function 2) [(.clone (.generated .binding 2))] true))]) (tail := (.call (.runtime .natAdd) [(.move (.generated .operand 5)), (.move (.generated .operand 6))] false)) (Corr.mNil) rfl rfl rfl (Corr.primF (ts := [.nat, .nat]) (Corr.opsCons (Corr.eOfBNil (Corr.callOps (ts := [(.adt 0)]) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lNil))) rfl rfl rfl rfl rfl)) (Corr.opsCons (Corr.eOfBNil (Corr.callOps (ts := [(.list (.adt 0))]) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lNil))) rfl rfl rfl rfl rfl)) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl)) rfl rfl rfl (Corr.fOfB (Corr.bOfE (Corr.value rfl rfl)))) rfl rfl rfl)⟩⟩

theorem crate_ok : CrateOK program krate flags := by
  intro f fn h
  match f with
  | 0 => exact fun0
  | 1 => exact fun1
  | 2 => exact fun2
  | _ + 3 => simp [program, index_eq] at h

/-- Every function of the program is simulated by its rendering. -/
theorem root : ∀ n f, FunSem program krate flags n f := fun n f => simulate crate_ok n f

end LexLeanPreserve.Cba27a3dc52c63de8b08b0726788d5c73.R29.RustStd
