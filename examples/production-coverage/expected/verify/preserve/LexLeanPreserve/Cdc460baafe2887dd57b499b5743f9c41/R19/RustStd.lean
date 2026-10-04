import LexLeanPreservation.RustSound
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R19.RustStd
open LexLeanTarget LexLeanPreservation.Rust

def program : TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0], types := [(.list .nat)], result := (.list .bool),
      body := (.call 1 [(.closure 2 []), (.var 0)]) },
    { parameters := [0, 1], types := [(.fn [.nat] .bool), (.list .nat)], result := (.list .bool),
      body := (.«match» (.list .bool) (.var 1) [(.arm .nil [] (.build .nil (.list .bool) [])), (.arm .cons [2, 3] (.build .cons (.list .bool) [(.apply (.var 0) [(.var 2)]), (.call 1 [(.var 0), (.var 3)])]))]) },
    { parameters := [0], types := [.nat], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .zero [] (.build .true .bool [])), (.arm .succ [1] (.call 3 [(.var 1)]))]) },
    { parameters := [0], types := [.nat], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .zero [] (.build .false .bool [])), (.arm .succ [1] (.call 2 [(.var 1)]))]) }] }

def krate : RustSyntax.Crate :=
  ({ profile := .std, items := [
    (.enum (.fn 0) [{ number := 2, fields := [] }]),
    (.apply 0 [.nat] .bool false [{ function := 2, captures := [], functionFallible := false }]),
    (.function (.generated .function 0) [{ pattern := (.bind (.generated .binding 0)), type := (.list .nat) }] (.list .bool) (.mk [] (.call (.function 1) [(.construct (.closure 0 2) []), (.clone (.generated .binding 0))] false))),
    (.function (.generated .function 1) [{ pattern := (.bind (.generated .binding 0)), type := (.fn 0) }, { pattern := (.bind (.generated .binding 1)), type := (.list .nat) }] (.list .bool) (.mk [(.mk (.bind (.generated .holder 1)) none (.clone (.generated .binding 1)))] (.matchOn (.uncons (.generated .holder 1)) [(.mk .none (.mk [] (.construct (.nil .bool) []))), (.mk (.some (.tuple [(.bind (.generated .binding 2)), (.bind (.generated .binding 3))])) (.mk [] (.construct .cons [(.block (.mk [(.mk (.bind (.generated .callee 2)) (some (.fn 0)) (.clone (.generated .binding 0)))] (.apply (.generated .callee 2) [(.copy (.generated .binding 2))] false))), (.call (.function 1) [(.clone (.generated .binding 0)), (.clone (.generated .binding 3))] false)])))]))),
    (.function (.generated .function 2) [{ pattern := (.bind (.generated .binding 0)), type := .nat }] .bool (.mk [(.mk (.bind (.generated .holder 3)) none (.copy (.generated .binding 0)))] (.cond (.isZero (.generated .holder 3)) (.mk [] (.lit (.bool true))) (.mk [(.mk (.bind (.generated .binding 1)) none (.predecessor (.generated .holder 3)))] (.call (.function 3) [(.copy (.generated .binding 1))] false))))),
    (.function (.generated .function 3) [{ pattern := (.bind (.generated .binding 0)), type := .nat }] .bool (.mk [(.mk (.bind (.generated .holder 4)) none (.copy (.generated .binding 0)))] (.cond (.isZero (.generated .holder 4)) (.mk [] (.lit (.bool false))) (.mk [(.mk (.bind (.generated .binding 1)) none (.predecessor (.generated .holder 4)))] (.call (.function 2) [(.copy (.generated .binding 1))] false)))))] } : LexLeanTarget.RustSyntax.Crate)

def flags : Flags := [(([.nat], .bool), false)]

theorem fun0 : FunOK program krate flags 0 :=
  ⟨_, rfl, .inl ⟨_, _, [], (.call (.function 1) [(.construct (.closure 0 2) []), (.clone (.generated .binding 0))] false), [(0, (.list .nat), (some 0))], rfl, rfl,
    (Corr.callOps (ts := [(.fn [.nat] .bool), (.list .nat)]) (Corr.opsOfL (Corr.lCons (Corr.eOfBNil (Corr.closure (caps := []) (mask := []) (af := false) (ff := false) (Corr.opsNil) rfl rfl rfl rfl rfl rfl rfl)) (Corr.lCons (Corr.var rfl rfl) (Corr.lNil)))) rfl rfl rfl rfl rfl)⟩⟩

theorem fun1 : FunOK program krate flags 1 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .holder 1)) none (.clone (.generated .binding 1)))], (.matchOn (.uncons (.generated .holder 1)) [(.mk .none (.mk [] (.construct (.nil .bool) []))), (.mk (.some (.tuple [(.bind (.generated .binding 2)), (.bind (.generated .binding 3))])) (.mk [] (.construct .cons [(.block (.mk [(.mk (.bind (.generated .callee 2)) (some (.fn 0)) (.clone (.generated .binding 0)))] (.apply (.generated .callee 2) [(.copy (.generated .binding 2))] false))), (.call (.function 1) [(.clone (.generated .binding 0)), (.clone (.generated .binding 3))] false)])))]), [(1, (.list .nat), (some 1)), (0, (.fn [.nat] .bool), (some 0))], rfl, rfl,
    (Corr.matchArms (st := (.list .nat)) (U := []) (view := true) (idx := [0, 1]) (Corr.var rfl rfl) (Corr.mCons (Γ' := [(1, (.list .nat), (some 1)), (0, (.fn [.nat] .bool), (some 0))]) (j := 0) (loads := []) (lets := []) (tail := (.construct (.nil .bool) [])) (Corr.mCons (Γ' := [(3, (.list .nat), (some 3)), (2, .nat, (some 2)), (1, (.list .nat), (some 1)), (0, (.fn [.nat] .bool), (some 0))]) (j := 1) (loads := []) (lets := []) (tail := (.construct .cons [(.block (.mk [(.mk (.bind (.generated .callee 2)) (some (.fn 0)) (.clone (.generated .binding 0)))] (.apply (.generated .callee 2) [(.copy (.generated .binding 2))] false))), (.call (.function 1) [(.clone (.generated .binding 0)), (.clone (.generated .binding 3))] false)])) (Corr.mNil) rfl rfl rfl (Corr.build (ts := [.bool, (.list .bool)]) (mask := [false, false]) (Corr.opsOfL (Corr.lCons (Corr.eOfB (lets := [(.mk (.bind (.generated .callee 2)) (some (.fn 0)) (.clone (.generated .binding 0)))]) (tail := (.apply (.generated .callee 2) [(.copy (.generated .binding 2))] false)) (Corr.apply (ps := [.nat]) (af := false) (Corr.var rfl rfl) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lNil))) rfl rfl rfl rfl)) (Corr.lCons (Corr.eOfBNil (Corr.callOps (ts := [(.fn [.nat] .bool), (.list .nat)]) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lCons (Corr.var rfl rfl) (Corr.lNil)))) rfl rfl rfl rfl rfl)) (Corr.lNil)))) rfl)) rfl rfl rfl (Corr.build (ts := []) (mask := []) (Corr.opsNil) rfl)) rfl rfl rfl)⟩⟩

theorem fun2 : FunOK program krate flags 2 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .holder 3)) none (.copy (.generated .binding 0)))], (.cond (.isZero (.generated .holder 3)) (.mk [] (.lit (.bool true))) (.mk [(.mk (.bind (.generated .binding 1)) none (.predecessor (.generated .holder 3)))] (.call (.function 3) [(.copy (.generated .binding 1))] false))), [(0, .nat, (some 0))], rfl, rfl,
    (Corr.matchNat (sl := (some 1)) (Corr.var rfl rfl) rfl rfl rfl (Corr.kIf (la := []) (ta := (.lit (.bool true))) (lb := []) (tb := (.call (.function 3) [(.copy (.generated .binding 1))] false)) (Corr.bOfE (Corr.buildConst rfl)) (Corr.callOps (ts := [.nat]) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lNil))) rfl rfl rfl rfl rfl)))⟩⟩

theorem fun3 : FunOK program krate flags 3 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .holder 4)) none (.copy (.generated .binding 0)))], (.cond (.isZero (.generated .holder 4)) (.mk [] (.lit (.bool false))) (.mk [(.mk (.bind (.generated .binding 1)) none (.predecessor (.generated .holder 4)))] (.call (.function 2) [(.copy (.generated .binding 1))] false))), [(0, .nat, (some 0))], rfl, rfl,
    (Corr.matchNat (sl := (some 1)) (Corr.var rfl rfl) rfl rfl rfl (Corr.kIf (la := []) (ta := (.lit (.bool false))) (lb := []) (tb := (.call (.function 2) [(.copy (.generated .binding 1))] false)) (Corr.bOfE (Corr.buildConst rfl)) (Corr.callOps (ts := [.nat]) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lNil))) rfl rfl rfl rfl rfl)))⟩⟩

theorem crate_ok : CrateOK program krate flags := by
  intro f fn h
  match f with
  | 0 => exact fun0
  | 1 => exact fun1
  | 2 => exact fun2
  | 3 => exact fun3
  | _ + 4 => simp [program, index_eq] at h

/-- Every function of the program is simulated by its rendering. -/
theorem root : ∀ n f, FunSem program krate flags n f := fun n f => simulate crate_ok n f

end LexLeanPreserve.Cdc460baafe2887dd57b499b5743f9c41.R19.RustStd
