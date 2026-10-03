import LexLeanPreservation.RustSound
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R13.RustStd
open LexLeanTarget LexLeanPreservation.Rust

def program : TargetSyntax.Program :=
  { adts := [{ constructors := [[.nat, .nat]] },
    { constructors := [[.nat, .nat]] },
    { constructors := [[.nat, .nat]] }], functions := [
    { parameters := [0, 1], types := [.nat, .nat], result := (.pair .nat .nat),
      body := (.«let» 2 (.adt 0) (.build (.adt 0) (.adt 0) [(.var 0), (.var 1)]) (.build .pair (.pair .nat .nat) [(.prim .natAdd [(.field (.var 2) 0), (.field (.call 1 []) 1)]), (.call 2 [(.build (.adt 0) (.adt 1) [(.field (.var 2) 1), (.value .nat (.nat 1))])])])) },
    { parameters := [], types := [], result := (.adt 2),
      body := (.build (.adt 0) (.adt 2) [(.value .nat (.nat 2)), (.value .nat (.nat 3))]) },
    { parameters := [0], types := [(.adt 1)], result := .nat,
      body := (.field (.var 0) 0) }] }

def krate : RustSyntax.Crate :=
  ({ profile := .std, items := [
    (.enum (.adt 0) [{ number := 0, fields := [.nat, .nat] }]),
    (.enum (.adt 1) [{ number := 0, fields := [.nat, .nat] }]),
    (.enum (.adt 2) [{ number := 0, fields := [.nat, .nat] }]),
    (.function (.generated .function 0) [{ pattern := (.bind (.generated .binding 0)), type := .nat }, { pattern := (.bind (.generated .binding 1)), type := .nat }] (.fallible (.pair .nat .nat)) (.mk [(.mk (.bind (.generated .binding 2)) (some (.adt 0)) (.construct (.adt 0 0) [(.copy (.generated .binding 0)), (.copy (.generated .binding 1))]))] (.succeed (.pair (.block (.mk [(.mk (.bind (.generated .operand 4)) none (.matchOn (.clone (.generated .binding 2)) [(.mk (.adt 0 0 [(.bind (.generated .part 1)), .wild]) (.mk [] (.move (.generated .part 1))))])), (.mk (.bind (.generated .operand 5)) none (.block (.mk [(.mk (.bind (.generated .holder 3)) none (.call (.function 1) [] false))] (.matchOn (.move (.generated .holder 3)) [(.mk (.adt 2 0 [.wild, (.bind (.generated .part 2))]) (.mk [] (.move (.generated .part 2))))]))))] (.call (.runtime .natAdd) [(.move (.generated .operand 4)), (.move (.generated .operand 5))] true))) (.call (.function 2) [(.construct (.adt 1 0) [(.matchOn (.clone (.generated .binding 2)) [(.mk (.adt 0 0 [.wild, (.bind (.generated .part 6))]) (.mk [] (.move (.generated .part 6))))]), (.lit (.nat 1))])] false))))),
    (.function (.generated .function 1) [] (.adt 2) (.mk [] (.construct (.adt 2 0) [(.lit (.nat 2)), (.lit (.nat 3))]))),
    (.function (.generated .function 2) [{ pattern := (.bind (.generated .binding 0)), type := (.adt 1) }] .nat (.mk [] (.matchOn (.clone (.generated .binding 0)) [(.mk (.adt 1 0 [(.bind (.generated .part 7)), .wild]) (.mk [] (.move (.generated .part 7))))])))] } : LexLeanTarget.RustSyntax.Crate)

def flags : Flags := []

theorem fun0 : FunOK program krate flags 0 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .binding 2)) (some (.adt 0)) (.construct (.adt 0 0) [(.copy (.generated .binding 0)), (.copy (.generated .binding 1))]))], (.succeed (.pair (.block (.mk [(.mk (.bind (.generated .operand 4)) none (.matchOn (.clone (.generated .binding 2)) [(.mk (.adt 0 0 [(.bind (.generated .part 1)), .wild]) (.mk [] (.move (.generated .part 1))))])), (.mk (.bind (.generated .operand 5)) none (.block (.mk [(.mk (.bind (.generated .holder 3)) none (.call (.function 1) [] false))] (.matchOn (.move (.generated .holder 3)) [(.mk (.adt 2 0 [.wild, (.bind (.generated .part 2))]) (.mk [] (.move (.generated .part 2))))]))))] (.call (.runtime .natAdd) [(.move (.generated .operand 4)), (.move (.generated .operand 5))] true))) (.call (.function 2) [(.construct (.adt 1 0) [(.matchOn (.clone (.generated .binding 2)) [(.mk (.adt 0 0 [.wild, (.bind (.generated .part 6))]) (.mk [] (.move (.generated .part 6))))]), (.lit (.nat 1))])] false))), [(1, .nat, (some 1)), (0, .nat, (some 0))], rfl, rfl,
    (Corr.letBind (Corr.eOfBNil (Corr.build (ts := [.nat, .nat]) (mask := [false, false]) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lCons (Corr.var rfl rfl) (Corr.lNil)))) rfl)) rfl (Corr.fOfB (Corr.bOfE (Corr.buildPair (Corr.lCons (Corr.eOfB (lets := [(.mk (.bind (.generated .operand 4)) none (.matchOn (.clone (.generated .binding 2)) [(.mk (.adt 0 0 [(.bind (.generated .part 1)), .wild]) (.mk [] (.move (.generated .part 1))))])), (.mk (.bind (.generated .operand 5)) none (.block (.mk [(.mk (.bind (.generated .holder 3)) none (.call (.function 1) [] false))] (.matchOn (.move (.generated .holder 3)) [(.mk (.adt 2 0 [.wild, (.bind (.generated .part 2))]) (.mk [] (.move (.generated .part 2))))]))))]) (tail := (.call (.runtime .natAdd) [(.move (.generated .operand 4)), (.move (.generated .operand 5))] true)) (Corr.prim (ts := [.nat, .nat]) (Corr.opsCons (Corr.eOfBNil (Corr.fieldRead (ts := [.nat, .nat]) (rd := (.move (.generated .part 1))) (Corr.var rfl rfl) rfl rfl rfl rfl rfl rfl)) (Corr.opsCons (Corr.eOfB (lets := [(.mk (.bind (.generated .holder 3)) none (.call (.function 1) [] false))]) (tail := (.matchOn (.move (.generated .holder 3)) [(.mk (.adt 2 0 [.wild, (.bind (.generated .part 2))]) (.mk [] (.move (.generated .part 2))))])) (Corr.fieldHeld (ts := [.nat, .nat]) (rd := (.move (.generated .part 2))) (Corr.eOfBNil (Corr.callOps (ts := []) (Corr.opsNil) rfl rfl rfl rfl rfl)) rfl rfl rfl rfl rfl rfl rfl)) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl)) (Corr.lCons (Corr.eOfBNil (Corr.callOps (ts := [(.adt 1)]) (Corr.opsOfL (Corr.lCons (Corr.eOfBNil (Corr.build (ts := [.nat, .nat]) (mask := [false, false]) (Corr.opsOfL (Corr.lCons (Corr.eOfBNil (Corr.fieldRead (ts := [.nat, .nat]) (rd := (.move (.generated .part 6))) (Corr.var rfl rfl) rfl rfl rfl rfl rfl rfl)) (Corr.lCons (Corr.value rfl rfl) (Corr.lNil)))) rfl)) (Corr.lNil))) rfl rfl rfl rfl rfl)) (Corr.lNil)))))))⟩⟩

theorem fun1 : FunOK program krate flags 1 :=
  ⟨_, rfl, .inl ⟨_, _, [], (.construct (.adt 2 0) [(.lit (.nat 2)), (.lit (.nat 3))]), [], rfl, rfl,
    (Corr.build (ts := [.nat, .nat]) (mask := [false, false]) (Corr.opsOfL (Corr.lCons (Corr.value rfl rfl) (Corr.lCons (Corr.value rfl rfl) (Corr.lNil)))) rfl)⟩⟩

theorem fun2 : FunOK program krate flags 2 :=
  ⟨_, rfl, .inl ⟨_, _, [], (.matchOn (.clone (.generated .binding 0)) [(.mk (.adt 1 0 [(.bind (.generated .part 7)), .wild]) (.mk [] (.move (.generated .part 7))))]), [(0, (.adt 1), (some 0))], rfl, rfl,
    (Corr.fieldRead (ts := [.nat, .nat]) (rd := (.move (.generated .part 7))) (Corr.var rfl rfl) rfl rfl rfl rfl rfl rfl)⟩⟩

theorem crate_ok : CrateOK program krate flags := by
  intro f fn h
  match f with
  | 0 => exact fun0
  | 1 => exact fun1
  | 2 => exact fun2
  | _ + 3 => simp [program, index_eq] at h

/-- Every function of the program is simulated by its rendering. -/
theorem root : ∀ n f, FunSem program krate flags n f := fun n f => simulate crate_ok n f

end LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R13.RustStd
