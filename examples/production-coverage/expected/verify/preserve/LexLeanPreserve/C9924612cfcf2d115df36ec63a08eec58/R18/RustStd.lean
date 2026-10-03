import LexLeanPreservation.RustSound
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R18.RustStd
open LexLeanTarget LexLeanPreservation.Rust

def program : TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [.nat, .nat], result := .nat,
      body := (.«let» 2 .nat (.prim .natAdd [(.var 0), (.value .nat (.nat 1))]) (.prim .natAdd [(.call 1 [(.closure 2 [(.var 2), (.var 1)]), (.var 0)]), (.prim .natAdd [(.apply (.closure 3 [(.var 2)]) [(.var 0)]), (.call 1 [(.closure 4 []), (.var 1)])])])) },
    { parameters := [0, 1], types := [(.fn [.nat] .nat), .nat], result := .nat,
      body := (.apply (.var 0) [(.apply (.var 0) [(.var 1)])]) },
    { parameters := [0, 1, 2], types := [.nat, .nat, .nat], result := .nat,
      body := (.prim .natAdd [(.prim .natMul [(.var 2), (.var 0)]), (.var 1)]) },
    { parameters := [0, 1], types := [.nat, .nat], result := .nat,
      body := (.prim .natSub [(.var 1), (.var 0)]) },
    { parameters := [0], types := [.nat], result := .nat,
      body := (.prim .natAdd [(.var 0), (.var 0)]) }] }

def krate : RustSyntax.Crate :=
  ({ profile := .std, items := [
    (.enum (.fn 0) [{ number := 2, fields := [.nat, .nat] }, { number := 3, fields := [.nat] }, { number := 4, fields := [] }]),
    (.apply 0 [.nat] .nat true [{ function := 2, captures := [.copy, .copy], functionFallible := true }, { function := 3, captures := [.copy], functionFallible := false }, { function := 4, captures := [], functionFallible := true }]),
    (.function (.generated .function 0) [{ pattern := (.bind (.generated .binding 0)), type := .nat }, { pattern := (.bind (.generated .binding 1)), type := .nat }] (.fallible .nat) (.mk [(.mk (.bind (.generated .binding 2)) (some .nat) (.block (.mk [(.mk (.bind (.generated .operand 1)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 2)) none (.lit (.nat 1)))] (.call (.runtime .natAdd) [(.move (.generated .operand 1)), (.move (.generated .operand 2))] true)))), (.mk (.bind (.generated .operand 6)) none (.call (.function 1) [(.construct (.closure 0 2) [(.copy (.generated .binding 2)), (.copy (.generated .binding 1))]), (.copy (.generated .binding 0))] true)), (.mk (.bind (.generated .operand 7)) none (.block (.mk [(.mk (.bind (.generated .operand 4)) none (.block (.mk [(.mk (.bind (.generated .callee 3)) (some (.fn 0)) (.construct (.closure 0 3) [(.copy (.generated .binding 2))]))] (.apply (.generated .callee 3) [(.copy (.generated .binding 0))] true)))), (.mk (.bind (.generated .operand 5)) none (.call (.function 1) [(.construct (.closure 0 4) []), (.copy (.generated .binding 1))] true))] (.call (.runtime .natAdd) [(.move (.generated .operand 4)), (.move (.generated .operand 5))] true))))] (.call (.runtime .natAdd) [(.move (.generated .operand 6)), (.move (.generated .operand 7))] false))),
    (.function (.generated .function 1) [{ pattern := (.bind (.generated .binding 0)), type := (.fn 0) }, { pattern := (.bind (.generated .binding 1)), type := .nat }] (.fallible .nat) (.mk [(.mk (.bind (.generated .callee 8)) (some (.fn 0)) (.clone (.generated .binding 0)))] (.apply (.generated .callee 8) [(.block (.mk [(.mk (.bind (.generated .callee 9)) (some (.fn 0)) (.clone (.generated .binding 0)))] (.apply (.generated .callee 9) [(.copy (.generated .binding 1))] true)))] false))),
    (.function (.generated .function 2) [{ pattern := (.bind (.generated .binding 0)), type := .nat }, { pattern := (.bind (.generated .binding 1)), type := .nat }, { pattern := (.bind (.generated .binding 2)), type := .nat }] (.fallible .nat) (.mk [(.mk (.bind (.generated .operand 12)) none (.block (.mk [(.mk (.bind (.generated .operand 10)) none (.copy (.generated .binding 2))), (.mk (.bind (.generated .operand 11)) none (.copy (.generated .binding 0)))] (.call (.runtime .natMul) [(.move (.generated .operand 10)), (.move (.generated .operand 11))] true)))), (.mk (.bind (.generated .operand 13)) none (.copy (.generated .binding 1)))] (.call (.runtime .natAdd) [(.move (.generated .operand 12)), (.move (.generated .operand 13))] false))),
    (.function (.generated .function 3) [{ pattern := (.bind (.generated .binding 0)), type := .nat }, { pattern := (.bind (.generated .binding 1)), type := .nat }] .nat (.mk [(.mk (.bind (.generated .operand 14)) none (.copy (.generated .binding 1))), (.mk (.bind (.generated .operand 15)) none (.copy (.generated .binding 0)))] (.call (.runtime .natSub) [(.move (.generated .operand 14)), (.move (.generated .operand 15))] false))),
    (.function (.generated .function 4) [{ pattern := (.bind (.generated .binding 0)), type := .nat }] (.fallible .nat) (.mk [(.mk (.bind (.generated .operand 16)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 17)) none (.copy (.generated .binding 0)))] (.call (.runtime .natAdd) [(.move (.generated .operand 16)), (.move (.generated .operand 17))] false)))] } : LexLeanTarget.RustSyntax.Crate)

def flags : Flags := [(([.nat], .nat), true)]

theorem fun0 : FunOK program krate flags 0 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .binding 2)) (some .nat) (.block (.mk [(.mk (.bind (.generated .operand 1)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 2)) none (.lit (.nat 1)))] (.call (.runtime .natAdd) [(.move (.generated .operand 1)), (.move (.generated .operand 2))] true)))), (.mk (.bind (.generated .operand 6)) none (.call (.function 1) [(.construct (.closure 0 2) [(.copy (.generated .binding 2)), (.copy (.generated .binding 1))]), (.copy (.generated .binding 0))] true)), (.mk (.bind (.generated .operand 7)) none (.block (.mk [(.mk (.bind (.generated .operand 4)) none (.block (.mk [(.mk (.bind (.generated .callee 3)) (some (.fn 0)) (.construct (.closure 0 3) [(.copy (.generated .binding 2))]))] (.apply (.generated .callee 3) [(.copy (.generated .binding 0))] true)))), (.mk (.bind (.generated .operand 5)) none (.call (.function 1) [(.construct (.closure 0 4) []), (.copy (.generated .binding 1))] true))] (.call (.runtime .natAdd) [(.move (.generated .operand 4)), (.move (.generated .operand 5))] true))))], (.call (.runtime .natAdd) [(.move (.generated .operand 6)), (.move (.generated .operand 7))] false), [(1, .nat, (some 1)), (0, .nat, (some 0))], rfl, rfl,
    (Corr.letBind (Corr.eOfB (lets := [(.mk (.bind (.generated .operand 1)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 2)) none (.lit (.nat 1)))]) (tail := (.call (.runtime .natAdd) [(.move (.generated .operand 1)), (.move (.generated .operand 2))] true)) (Corr.prim (ts := [.nat, .nat]) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsCons (Corr.value rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl)) rfl (Corr.primF (ts := [.nat, .nat]) (Corr.opsCons (Corr.eOfBNil (Corr.callOps (ts := [(.fn [.nat] .nat), .nat]) (Corr.opsOfL (Corr.lCons (Corr.eOfBNil (Corr.closure (caps := [.nat, .nat]) (mask := [false, false]) (af := true) (ff := true) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lCons (Corr.var rfl rfl) (Corr.lNil)))) rfl rfl rfl rfl rfl rfl rfl)) (Corr.lCons (Corr.var rfl rfl) (Corr.lNil)))) rfl rfl rfl rfl rfl)) (Corr.opsCons (Corr.eOfB (lets := [(.mk (.bind (.generated .operand 4)) none (.block (.mk [(.mk (.bind (.generated .callee 3)) (some (.fn 0)) (.construct (.closure 0 3) [(.copy (.generated .binding 2))]))] (.apply (.generated .callee 3) [(.copy (.generated .binding 0))] true)))), (.mk (.bind (.generated .operand 5)) none (.call (.function 1) [(.construct (.closure 0 4) []), (.copy (.generated .binding 1))] true))]) (tail := (.call (.runtime .natAdd) [(.move (.generated .operand 4)), (.move (.generated .operand 5))] true)) (Corr.prim (ts := [.nat, .nat]) (Corr.opsCons (Corr.eOfB (lets := [(.mk (.bind (.generated .callee 3)) (some (.fn 0)) (.construct (.closure 0 3) [(.copy (.generated .binding 2))]))]) (tail := (.apply (.generated .callee 3) [(.copy (.generated .binding 0))] true)) (Corr.apply (ps := [.nat]) (af := true) (Corr.eOfBNil (Corr.closure (caps := [.nat]) (mask := [false]) (af := true) (ff := false) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lNil))) rfl rfl rfl rfl rfl rfl rfl)) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lNil))) rfl rfl rfl rfl)) (Corr.opsCons (Corr.eOfBNil (Corr.callOps (ts := [(.fn [.nat] .nat), .nat]) (Corr.opsOfL (Corr.lCons (Corr.eOfBNil (Corr.closure (caps := []) (mask := []) (af := true) (ff := true) (Corr.opsNil) rfl rfl rfl rfl rfl rfl rfl)) (Corr.lCons (Corr.var rfl rfl) (Corr.lNil)))) rfl rfl rfl rfl rfl)) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl)) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl))⟩⟩

theorem fun1 : FunOK program krate flags 1 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .callee 8)) (some (.fn 0)) (.clone (.generated .binding 0)))], (.apply (.generated .callee 8) [(.block (.mk [(.mk (.bind (.generated .callee 9)) (some (.fn 0)) (.clone (.generated .binding 0)))] (.apply (.generated .callee 9) [(.copy (.generated .binding 1))] true)))] false), [(1, .nat, (some 1)), (0, (.fn [.nat] .nat), (some 0))], rfl, rfl,
    (Corr.applyF (ps := [.nat]) (Corr.var rfl rfl) (Corr.opsOfL (Corr.lCons (Corr.eOfB (lets := [(.mk (.bind (.generated .callee 9)) (some (.fn 0)) (.clone (.generated .binding 0)))]) (tail := (.apply (.generated .callee 9) [(.copy (.generated .binding 1))] true)) (Corr.apply (ps := [.nat]) (af := true) (Corr.var rfl rfl) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lNil))) rfl rfl rfl rfl)) (Corr.lNil))) rfl rfl rfl)⟩⟩

theorem fun2 : FunOK program krate flags 2 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .operand 12)) none (.block (.mk [(.mk (.bind (.generated .operand 10)) none (.copy (.generated .binding 2))), (.mk (.bind (.generated .operand 11)) none (.copy (.generated .binding 0)))] (.call (.runtime .natMul) [(.move (.generated .operand 10)), (.move (.generated .operand 11))] true)))), (.mk (.bind (.generated .operand 13)) none (.copy (.generated .binding 1)))], (.call (.runtime .natAdd) [(.move (.generated .operand 12)), (.move (.generated .operand 13))] false), [(2, .nat, (some 2)), (1, .nat, (some 1)), (0, .nat, (some 0))], rfl, rfl,
    (Corr.primF (ts := [.nat, .nat]) (Corr.opsCons (Corr.eOfB (lets := [(.mk (.bind (.generated .operand 10)) none (.copy (.generated .binding 2))), (.mk (.bind (.generated .operand 11)) none (.copy (.generated .binding 0)))]) (tail := (.call (.runtime .natMul) [(.move (.generated .operand 10)), (.move (.generated .operand 11))] true)) (Corr.prim (ts := [.nat, .nat]) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl)) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl)⟩⟩

theorem fun3 : FunOK program krate flags 3 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .operand 14)) none (.copy (.generated .binding 1))), (.mk (.bind (.generated .operand 15)) none (.copy (.generated .binding 0)))], (.call (.runtime .natSub) [(.move (.generated .operand 14)), (.move (.generated .operand 15))] false), [(1, .nat, (some 1)), (0, .nat, (some 0))], rfl, rfl,
    (Corr.prim (ts := [.nat, .nat]) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl)⟩⟩

theorem fun4 : FunOK program krate flags 4 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .operand 16)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 17)) none (.copy (.generated .binding 0)))], (.call (.runtime .natAdd) [(.move (.generated .operand 16)), (.move (.generated .operand 17))] false), [(0, .nat, (some 0))], rfl, rfl,
    (Corr.primF (ts := [.nat, .nat]) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl)⟩⟩

theorem crate_ok : CrateOK program krate flags := by
  intro f fn h
  match f with
  | 0 => exact fun0
  | 1 => exact fun1
  | 2 => exact fun2
  | 3 => exact fun3
  | 4 => exact fun4
  | _ + 5 => simp [program, index_eq] at h

/-- Every function of the program is simulated by its rendering. -/
theorem root : ∀ n f, FunSem program krate flags n f := fun n f => simulate crate_ok n f

end LexLeanPreserve.C9924612cfcf2d115df36ec63a08eec58.R18.RustStd
