import LexLeanPreservation.RustSound
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R13.RustStd
open LexLeanTarget LexLeanPreservation.Rust

def program : TargetSyntax.Program :=
  { adts := [{ constructors := [[], [(.adt 0), .nat, (.adt 0)]] }], functions := [
    { parameters := [0], types := [.nat], result := .nat,
      body := (.call 1 [(.build (.adt 1) (.adt 0) [(.build (.adt 0) (.adt 0) []), (.var 0), (.build (.adt 1) (.adt 0) [(.build (.adt 0) (.adt 0) []), (.value .nat (.nat 5)), (.build (.adt 0) (.adt 0) [])])])]) },
    { parameters := [0], types := [(.adt 0)], result := .nat,
      body := (.«match» .nat (.var 0) [(.arm (.adt 0) [] (.value .nat (.nat 0))), (.arm (.adt 1) [1, 2, 3] (.prim .natAdd [(.prim .natAdd [(.call 1 [(.var 1)]), (.var 2)]), (.call 1 [(.var 3)])]))]) }] }

def krate : RustSyntax.Crate :=
  ({ profile := .std, items := [
    (.enum (.adt 0) [{ number := 0, fields := [] }, { number := 1, fields := [(.rc (.adt 0)), .nat, (.rc (.adt 0))] }]),
    (.function (.generated .function 0) [{ pattern := (.bind (.generated .binding 0)), type := .nat }] (.fallible .nat) (.mk [] (.call (.function 1) [(.construct (.adt 0 1) [(.box (.construct (.adt 0 0) [])), (.copy (.generated .binding 0)), (.box (.construct (.adt 0 1) [(.box (.construct (.adt 0 0) [])), (.lit (.nat 5)), (.box (.construct (.adt 0 0) []))]))])] false))),
    (.function (.generated .function 1) [{ pattern := (.bind (.generated .binding 0)), type := (.adt 0) }] (.fallible .nat) (.mk [(.mk (.bind (.generated .holder 1)) none (.clone (.generated .binding 0)))] (.matchOn (.move (.generated .holder 1)) [(.mk (.adt 0 0 []) (.mk [] (.succeed (.lit (.nat 0))))), (.mk (.adt 0 1 [(.bind (.generated .boxed 6)), (.bind (.generated .binding 2)), (.bind (.generated .boxed 7))]) (.mk [(.mk (.bind (.generated .binding 1)) none (.unbox (.generated .boxed 6))), (.mk (.bind (.generated .binding 3)) none (.unbox (.generated .boxed 7))), (.mk (.bind (.generated .operand 4)) none (.block (.mk [(.mk (.bind (.generated .operand 2)) none (.call (.function 1) [(.clone (.generated .binding 1))] true)), (.mk (.bind (.generated .operand 3)) none (.copy (.generated .binding 2)))] (.call (.runtime .natAdd) [(.move (.generated .operand 2)), (.move (.generated .operand 3))] true)))), (.mk (.bind (.generated .operand 5)) none (.call (.function 1) [(.clone (.generated .binding 3))] true))] (.call (.runtime .natAdd) [(.move (.generated .operand 4)), (.move (.generated .operand 5))] false)))])))] } : LexLeanTarget.RustSyntax.Crate)

def flags : Flags := []

theorem fun0 : FunOK program krate flags 0 :=
  ⟨_, rfl, .inl ⟨_, _, [], (.call (.function 1) [(.construct (.adt 0 1) [(.box (.construct (.adt 0 0) [])), (.copy (.generated .binding 0)), (.box (.construct (.adt 0 1) [(.box (.construct (.adt 0 0) [])), (.lit (.nat 5)), (.box (.construct (.adt 0 0) []))]))])] false), [(0, .nat, (some 0))], rfl, rfl,
    (Corr.callF (ts := [(.adt 0)]) (Corr.opsOfL (Corr.lCons (Corr.eOfBNil (Corr.build (ts := [(.adt 0), .nat, (.adt 0)]) (mask := [true, false, true]) (Corr.opsOfL (Corr.lCons (Corr.eOfBNil (Corr.build (ts := []) (mask := []) (Corr.opsNil) rfl)) (Corr.lCons (Corr.var rfl rfl) (Corr.lCons (Corr.eOfBNil (Corr.build (ts := [(.adt 0), .nat, (.adt 0)]) (mask := [true, false, true]) (Corr.opsOfL (Corr.lCons (Corr.eOfBNil (Corr.build (ts := []) (mask := []) (Corr.opsNil) rfl)) (Corr.lCons (Corr.value rfl rfl) (Corr.lCons (Corr.eOfBNil (Corr.build (ts := []) (mask := []) (Corr.opsNil) rfl)) (Corr.lNil))))) rfl)) (Corr.lNil))))) rfl)) (Corr.lNil))) rfl rfl rfl rfl)⟩⟩

theorem fun1 : FunOK program krate flags 1 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .holder 1)) none (.clone (.generated .binding 0)))], (.matchOn (.move (.generated .holder 1)) [(.mk (.adt 0 0 []) (.mk [] (.succeed (.lit (.nat 0))))), (.mk (.adt 0 1 [(.bind (.generated .boxed 6)), (.bind (.generated .binding 2)), (.bind (.generated .boxed 7))]) (.mk [(.mk (.bind (.generated .binding 1)) none (.unbox (.generated .boxed 6))), (.mk (.bind (.generated .binding 3)) none (.unbox (.generated .boxed 7))), (.mk (.bind (.generated .operand 4)) none (.block (.mk [(.mk (.bind (.generated .operand 2)) none (.call (.function 1) [(.clone (.generated .binding 1))] true)), (.mk (.bind (.generated .operand 3)) none (.copy (.generated .binding 2)))] (.call (.runtime .natAdd) [(.move (.generated .operand 2)), (.move (.generated .operand 3))] true)))), (.mk (.bind (.generated .operand 5)) none (.call (.function 1) [(.clone (.generated .binding 3))] true))] (.call (.runtime .natAdd) [(.move (.generated .operand 4)), (.move (.generated .operand 5))] false)))]), [(0, (.adt 0), (some 0))], rfl, rfl,
    (Corr.matchArms (st := (.adt 0)) (U := []) (view := false) (idx := [0, 1]) (Corr.var rfl rfl) (Corr.mCons (Γ' := [(0, (.adt 0), (some 0))]) (j := 0) (loads := []) (lets := []) (tail := (.succeed (.lit (.nat 0)))) (Corr.mCons (Γ' := [(3, (.adt 0), (some 3)), (2, .nat, (some 2)), (1, (.adt 0), (some 1)), (0, (.adt 0), (some 0))]) (j := 1) (loads := [(.mk (.bind (.generated .binding 1)) none (.unbox (.generated .boxed 6))), (.mk (.bind (.generated .binding 3)) none (.unbox (.generated .boxed 7)))]) (lets := [(.mk (.bind (.generated .operand 4)) none (.block (.mk [(.mk (.bind (.generated .operand 2)) none (.call (.function 1) [(.clone (.generated .binding 1))] true)), (.mk (.bind (.generated .operand 3)) none (.copy (.generated .binding 2)))] (.call (.runtime .natAdd) [(.move (.generated .operand 2)), (.move (.generated .operand 3))] true)))), (.mk (.bind (.generated .operand 5)) none (.call (.function 1) [(.clone (.generated .binding 3))] true))]) (tail := (.call (.runtime .natAdd) [(.move (.generated .operand 4)), (.move (.generated .operand 5))] false)) (Corr.mNil) rfl rfl rfl (Corr.primF (ts := [.nat, .nat]) (Corr.opsCons (Corr.eOfB (lets := [(.mk (.bind (.generated .operand 2)) none (.call (.function 1) [(.clone (.generated .binding 1))] true)), (.mk (.bind (.generated .operand 3)) none (.copy (.generated .binding 2)))]) (tail := (.call (.runtime .natAdd) [(.move (.generated .operand 2)), (.move (.generated .operand 3))] true)) (Corr.prim (ts := [.nat, .nat]) (Corr.opsCons (Corr.eOfBNil (Corr.callOps (ts := [(.adt 0)]) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lNil))) rfl rfl rfl rfl rfl)) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl)) (Corr.opsCons (Corr.eOfBNil (Corr.callOps (ts := [(.adt 0)]) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lNil))) rfl rfl rfl rfl rfl)) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl)) rfl rfl rfl (Corr.fOfB (Corr.bOfE (Corr.value rfl rfl)))) rfl rfl rfl)⟩⟩

theorem crate_ok : CrateOK program krate flags := by
  intro f fn h
  match f with
  | 0 => exact fun0
  | 1 => exact fun1
  | _ + 2 => simp [program, index_eq] at h

/-- Every function of the program is simulated by its rendering. -/
theorem root : ∀ n f, FunSem program krate flags n f := fun n f => simulate crate_ok n f

end LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R13.RustStd
