import LexLeanPreservation.RustSound
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R3.RustStd
open LexLeanTarget LexLeanPreservation.Rust

def program : TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0, 1], types := [(.list .nat), (.list .int)], result := (.pair .nat .int),
      body := (.build .pair (.pair .nat .int) [(.call 1 [(.closure 2 []), (.value .nat (.nat 0)), (.var 0)]), (.call 3 [(.closure 4 []), (.value .int (.int 5)), (.var 1)])]) },
    { parameters := [0, 1, 2], types := [(.fn [.nat, .nat] .nat), .nat, (.list .nat)], result := .nat,
      body := (.«match» .nat (.var 2) [(.arm .nil [] (.var 1)), (.arm .cons [3, 4] (.call 1 [(.var 0), (.apply (.var 0) [(.var 1), (.var 3)]), (.var 4)]))]) },
    { parameters := [0, 1], types := [.nat, .nat], result := .nat,
      body := (.prim .natAdd [(.var 0), (.var 1)]) },
    { parameters := [0, 1, 2], types := [(.fn [.int, .int] .int), .int, (.list .int)], result := .int,
      body := (.«match» .int (.var 2) [(.arm .nil [] (.var 1)), (.arm .cons [3, 4] (.call 3 [(.var 0), (.apply (.var 0) [(.var 1), (.var 3)]), (.var 4)]))]) },
    { parameters := [0, 1], types := [.int, .int], result := .int,
      body := (.prim .intSub [(.var 0), (.var 1)]) }] }

def krate : RustSyntax.Crate :=
  ({ profile := .std, items := [
    (.enum (.fn 0) [{ number := 2, fields := [] }]),
    (.apply 0 [.nat, .nat] .nat true [{ function := 2, captures := [], functionFallible := true }]),
    (.enum (.fn 1) [{ number := 4, fields := [] }]),
    (.apply 1 [.int, .int] .int true [{ function := 4, captures := [], functionFallible := true }]),
    (.function (.generated .function 0) [{ pattern := (.bind (.generated .binding 0)), type := (.list .nat) }, { pattern := (.bind (.generated .binding 1)), type := (.list .int) }] (.fallible (.pair .nat .int)) (.mk [] (.succeed (.pair (.call (.function 1) [(.construct (.closure 0 2) []), (.lit (.nat 0)), (.clone (.generated .binding 0))] true) (.call (.function 3) [(.construct (.closure 1 4) []), (.lit (.int 5)), (.clone (.generated .binding 1))] true))))),
    (.function (.generated .function 1) [{ pattern := (.bind (.generated .binding 0)), type := (.fn 0) }, { pattern := (.bind (.generated .binding 1)), type := .nat }, { pattern := (.bind (.generated .binding 2)), type := (.list .nat) }] (.fallible .nat) (.mk [(.mk (.bind (.generated .holder 1)) none (.clone (.generated .binding 2)))] (.matchOn (.uncons (.generated .holder 1)) [(.mk .none (.mk [] (.succeed (.copy (.generated .binding 1))))), (.mk (.some (.tuple [(.bind (.generated .binding 3)), (.bind (.generated .binding 4))])) (.mk [] (.call (.function 1) [(.clone (.generated .binding 0)), (.block (.mk [(.mk (.bind (.generated .callee 2)) (some (.fn 0)) (.clone (.generated .binding 0)))] (.apply (.generated .callee 2) [(.copy (.generated .binding 1)), (.copy (.generated .binding 3))] true))), (.clone (.generated .binding 4))] false)))]))),
    (.function (.generated .function 2) [{ pattern := (.bind (.generated .binding 0)), type := .nat }, { pattern := (.bind (.generated .binding 1)), type := .nat }] (.fallible .nat) (.mk [(.mk (.bind (.generated .operand 3)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 4)) none (.copy (.generated .binding 1)))] (.call (.runtime .natAdd) [(.move (.generated .operand 3)), (.move (.generated .operand 4))] false))),
    (.function (.generated .function 3) [{ pattern := (.bind (.generated .binding 0)), type := (.fn 1) }, { pattern := (.bind (.generated .binding 1)), type := .int }, { pattern := (.bind (.generated .binding 2)), type := (.list .int) }] (.fallible .int) (.mk [(.mk (.bind (.generated .holder 5)) none (.clone (.generated .binding 2)))] (.matchOn (.uncons (.generated .holder 5)) [(.mk .none (.mk [] (.succeed (.copy (.generated .binding 1))))), (.mk (.some (.tuple [(.bind (.generated .binding 3)), (.bind (.generated .binding 4))])) (.mk [] (.call (.function 3) [(.clone (.generated .binding 0)), (.block (.mk [(.mk (.bind (.generated .callee 6)) (some (.fn 1)) (.clone (.generated .binding 0)))] (.apply (.generated .callee 6) [(.copy (.generated .binding 1)), (.copy (.generated .binding 3))] true))), (.clone (.generated .binding 4))] false)))]))),
    (.function (.generated .function 4) [{ pattern := (.bind (.generated .binding 0)), type := .int }, { pattern := (.bind (.generated .binding 1)), type := .int }] (.fallible .int) (.mk [(.mk (.bind (.generated .operand 7)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 8)) none (.copy (.generated .binding 1)))] (.call (.runtime .intSub) [(.move (.generated .operand 7)), (.move (.generated .operand 8))] false)))] } : LexLeanTarget.RustSyntax.Crate)

def flags : Flags := [(([.nat, .nat], .nat), true), (([.int, .int], .int), true)]

theorem fun0 : FunOK program krate flags 0 :=
  ⟨_, rfl, .inl ⟨_, _, [], (.succeed (.pair (.call (.function 1) [(.construct (.closure 0 2) []), (.lit (.nat 0)), (.clone (.generated .binding 0))] true) (.call (.function 3) [(.construct (.closure 1 4) []), (.lit (.int 5)), (.clone (.generated .binding 1))] true))), [(1, (.list .int), (some 1)), (0, (.list .nat), (some 0))], rfl, rfl,
    (Corr.fOfB (Corr.bOfE (Corr.buildPair (Corr.lCons (Corr.eOfBNil (Corr.callOps (ts := [(.fn [.nat, .nat] .nat), .nat, (.list .nat)]) (Corr.opsOfL (Corr.lCons (Corr.eOfBNil (Corr.closure (caps := []) (mask := []) (af := true) (ff := true) (Corr.opsNil) rfl rfl rfl rfl rfl rfl rfl)) (Corr.lCons (Corr.value rfl rfl) (Corr.lCons (Corr.var rfl rfl) (Corr.lNil))))) rfl rfl rfl rfl rfl)) (Corr.lCons (Corr.eOfBNil (Corr.callOps (ts := [(.fn [.int, .int] .int), .int, (.list .int)]) (Corr.opsOfL (Corr.lCons (Corr.eOfBNil (Corr.closure (caps := []) (mask := []) (af := true) (ff := true) (Corr.opsNil) rfl rfl rfl rfl rfl rfl rfl)) (Corr.lCons (Corr.value rfl rfl) (Corr.lCons (Corr.var rfl rfl) (Corr.lNil))))) rfl rfl rfl rfl rfl)) (Corr.lNil))))))⟩⟩

theorem fun1 : FunOK program krate flags 1 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .holder 1)) none (.clone (.generated .binding 2)))], (.matchOn (.uncons (.generated .holder 1)) [(.mk .none (.mk [] (.succeed (.copy (.generated .binding 1))))), (.mk (.some (.tuple [(.bind (.generated .binding 3)), (.bind (.generated .binding 4))])) (.mk [] (.call (.function 1) [(.clone (.generated .binding 0)), (.block (.mk [(.mk (.bind (.generated .callee 2)) (some (.fn 0)) (.clone (.generated .binding 0)))] (.apply (.generated .callee 2) [(.copy (.generated .binding 1)), (.copy (.generated .binding 3))] true))), (.clone (.generated .binding 4))] false)))]), [(2, (.list .nat), (some 2)), (1, .nat, (some 1)), (0, (.fn [.nat, .nat] .nat), (some 0))], rfl, rfl,
    (Corr.matchArms (st := (.list .nat)) (U := []) (view := true) (idx := [0, 1]) (Corr.var rfl rfl) (Corr.mCons (Γ' := [(2, (.list .nat), (some 2)), (1, .nat, (some 1)), (0, (.fn [.nat, .nat] .nat), (some 0))]) (j := 0) (loads := []) (lets := []) (tail := (.succeed (.copy (.generated .binding 1)))) (Corr.mCons (Γ' := [(4, (.list .nat), (some 4)), (3, .nat, (some 3)), (2, (.list .nat), (some 2)), (1, .nat, (some 1)), (0, (.fn [.nat, .nat] .nat), (some 0))]) (j := 1) (loads := []) (lets := []) (tail := (.call (.function 1) [(.clone (.generated .binding 0)), (.block (.mk [(.mk (.bind (.generated .callee 2)) (some (.fn 0)) (.clone (.generated .binding 0)))] (.apply (.generated .callee 2) [(.copy (.generated .binding 1)), (.copy (.generated .binding 3))] true))), (.clone (.generated .binding 4))] false)) (Corr.mNil) rfl rfl rfl (Corr.callF (ts := [(.fn [.nat, .nat] .nat), .nat, (.list .nat)]) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lCons (Corr.eOfB (lets := [(.mk (.bind (.generated .callee 2)) (some (.fn 0)) (.clone (.generated .binding 0)))]) (tail := (.apply (.generated .callee 2) [(.copy (.generated .binding 1)), (.copy (.generated .binding 3))] true)) (Corr.apply (ps := [.nat, .nat]) (af := true) (Corr.var rfl rfl) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lCons (Corr.var rfl rfl) (Corr.lNil)))) rfl rfl rfl rfl)) (Corr.lCons (Corr.var rfl rfl) (Corr.lNil))))) rfl rfl rfl rfl)) rfl rfl rfl (Corr.fOfB (Corr.bOfE (Corr.var rfl rfl)))) rfl rfl rfl)⟩⟩

theorem fun2 : FunOK program krate flags 2 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .operand 3)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 4)) none (.copy (.generated .binding 1)))], (.call (.runtime .natAdd) [(.move (.generated .operand 3)), (.move (.generated .operand 4))] false), [(1, .nat, (some 1)), (0, .nat, (some 0))], rfl, rfl,
    (Corr.primF (ts := [.nat, .nat]) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl)⟩⟩

theorem fun3 : FunOK program krate flags 3 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .holder 5)) none (.clone (.generated .binding 2)))], (.matchOn (.uncons (.generated .holder 5)) [(.mk .none (.mk [] (.succeed (.copy (.generated .binding 1))))), (.mk (.some (.tuple [(.bind (.generated .binding 3)), (.bind (.generated .binding 4))])) (.mk [] (.call (.function 3) [(.clone (.generated .binding 0)), (.block (.mk [(.mk (.bind (.generated .callee 6)) (some (.fn 1)) (.clone (.generated .binding 0)))] (.apply (.generated .callee 6) [(.copy (.generated .binding 1)), (.copy (.generated .binding 3))] true))), (.clone (.generated .binding 4))] false)))]), [(2, (.list .int), (some 2)), (1, .int, (some 1)), (0, (.fn [.int, .int] .int), (some 0))], rfl, rfl,
    (Corr.matchArms (st := (.list .int)) (U := []) (view := true) (idx := [0, 1]) (Corr.var rfl rfl) (Corr.mCons (Γ' := [(2, (.list .int), (some 2)), (1, .int, (some 1)), (0, (.fn [.int, .int] .int), (some 0))]) (j := 0) (loads := []) (lets := []) (tail := (.succeed (.copy (.generated .binding 1)))) (Corr.mCons (Γ' := [(4, (.list .int), (some 4)), (3, .int, (some 3)), (2, (.list .int), (some 2)), (1, .int, (some 1)), (0, (.fn [.int, .int] .int), (some 0))]) (j := 1) (loads := []) (lets := []) (tail := (.call (.function 3) [(.clone (.generated .binding 0)), (.block (.mk [(.mk (.bind (.generated .callee 6)) (some (.fn 1)) (.clone (.generated .binding 0)))] (.apply (.generated .callee 6) [(.copy (.generated .binding 1)), (.copy (.generated .binding 3))] true))), (.clone (.generated .binding 4))] false)) (Corr.mNil) rfl rfl rfl (Corr.callF (ts := [(.fn [.int, .int] .int), .int, (.list .int)]) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lCons (Corr.eOfB (lets := [(.mk (.bind (.generated .callee 6)) (some (.fn 1)) (.clone (.generated .binding 0)))]) (tail := (.apply (.generated .callee 6) [(.copy (.generated .binding 1)), (.copy (.generated .binding 3))] true)) (Corr.apply (ps := [.int, .int]) (af := true) (Corr.var rfl rfl) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lCons (Corr.var rfl rfl) (Corr.lNil)))) rfl rfl rfl rfl)) (Corr.lCons (Corr.var rfl rfl) (Corr.lNil))))) rfl rfl rfl rfl)) rfl rfl rfl (Corr.fOfB (Corr.bOfE (Corr.var rfl rfl)))) rfl rfl rfl)⟩⟩

theorem fun4 : FunOK program krate flags 4 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .operand 7)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 8)) none (.copy (.generated .binding 1)))], (.call (.runtime .intSub) [(.move (.generated .operand 7)), (.move (.generated .operand 8))] false), [(1, .int, (some 1)), (0, .int, (some 0))], rfl, rfl,
    (Corr.primF (ts := [.int, .int]) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl)⟩⟩

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

end LexLeanPreserve.C2f1f97944953d8dec983ce8fe0b8fbfc.R3.RustStd
