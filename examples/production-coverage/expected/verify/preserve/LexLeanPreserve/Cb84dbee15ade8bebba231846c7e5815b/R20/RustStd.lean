import LexLeanPreservation.RustSound
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R20.RustStd
open LexLeanTarget LexLeanPreservation.Rust

def program : TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0], types := [.nat], result := .bool,
      body := (.call 1 [(.var 0)]) },
    { parameters := [0], types := [.nat], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .zero [] (.build .false .bool [])), (.arm .succ [1] (.call 2 [(.var 1)]))]) },
    { parameters := [0], types := [.nat], result := .bool,
      body := (.«match» .bool (.var 0) [(.arm .zero [] (.build .true .bool [])), (.arm .succ [1] (.call 1 [(.var 1)]))]) }] }

def krate : RustSyntax.Crate :=
  ({ profile := .std, items := [
    (.function (.generated .function 0) [{ pattern := (.bind (.generated .binding 0)), type := .nat }] .bool (.mk [] (.call (.function 1) [(.copy (.generated .binding 0))] false))),
    (.function (.generated .function 1) [{ pattern := (.bind (.generated .binding 0)), type := .nat }] .bool (.mk [(.mk (.bind (.generated .holder 1)) none (.copy (.generated .binding 0)))] (.cond (.isZero (.generated .holder 1)) (.mk [] (.lit (.bool false))) (.mk [(.mk (.bind (.generated .binding 1)) none (.predecessor (.generated .holder 1)))] (.call (.function 2) [(.copy (.generated .binding 1))] false))))),
    (.function (.generated .function 2) [{ pattern := (.bind (.generated .binding 0)), type := .nat }] .bool (.mk [(.mk (.bind (.generated .holder 2)) none (.copy (.generated .binding 0)))] (.cond (.isZero (.generated .holder 2)) (.mk [] (.lit (.bool true))) (.mk [(.mk (.bind (.generated .binding 1)) none (.predecessor (.generated .holder 2)))] (.call (.function 1) [(.copy (.generated .binding 1))] false)))))] } : LexLeanTarget.RustSyntax.Crate)

def flags : Flags := []

theorem fun0 : FunOK program krate flags 0 :=
  ⟨_, rfl, .inl ⟨_, _, [], (.call (.function 1) [(.copy (.generated .binding 0))] false), [(0, .nat, (some 0))], rfl, rfl,
    (Corr.callOps (ts := [.nat]) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lNil))) rfl rfl rfl rfl rfl)⟩⟩

theorem fun1 : FunOK program krate flags 1 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .holder 1)) none (.copy (.generated .binding 0)))], (.cond (.isZero (.generated .holder 1)) (.mk [] (.lit (.bool false))) (.mk [(.mk (.bind (.generated .binding 1)) none (.predecessor (.generated .holder 1)))] (.call (.function 2) [(.copy (.generated .binding 1))] false))), [(0, .nat, (some 0))], rfl, rfl,
    (Corr.matchNat (sl := (some 1)) (Corr.var rfl rfl) rfl rfl rfl (Corr.kIf (la := []) (ta := (.lit (.bool false))) (lb := []) (tb := (.call (.function 2) [(.copy (.generated .binding 1))] false)) (Corr.bOfE (Corr.buildConst rfl)) (Corr.callOps (ts := [.nat]) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lNil))) rfl rfl rfl rfl rfl)))⟩⟩

theorem fun2 : FunOK program krate flags 2 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .holder 2)) none (.copy (.generated .binding 0)))], (.cond (.isZero (.generated .holder 2)) (.mk [] (.lit (.bool true))) (.mk [(.mk (.bind (.generated .binding 1)) none (.predecessor (.generated .holder 2)))] (.call (.function 1) [(.copy (.generated .binding 1))] false))), [(0, .nat, (some 0))], rfl, rfl,
    (Corr.matchNat (sl := (some 1)) (Corr.var rfl rfl) rfl rfl rfl (Corr.kIf (la := []) (ta := (.lit (.bool true))) (lb := []) (tb := (.call (.function 1) [(.copy (.generated .binding 1))] false)) (Corr.bOfE (Corr.buildConst rfl)) (Corr.callOps (ts := [.nat]) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lNil))) rfl rfl rfl rfl rfl)))⟩⟩

theorem crate_ok : CrateOK program krate flags := by
  intro f fn h
  match f with
  | 0 => exact fun0
  | 1 => exact fun1
  | 2 => exact fun2
  | _ + 3 => simp [program, index_eq] at h

/-- Every function of the program is simulated by its rendering. -/
theorem root : ∀ n f, FunSem program krate flags n f := fun n f => simulate crate_ok n f

end LexLeanPreserve.Cb84dbee15ade8bebba231846c7e5815b.R20.RustStd
