import LexLeanPreservation.RustSound
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.C9b67141c19e48429ed1a6443d832fde7.R7.RustStd
open LexLeanTarget LexLeanPreservation.Rust

def program : TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0], types := [.nat], result := (.result .nat (.pair .bool .bool)),
      body := (.«let» 1 .nat (.var 0) (.cond (.call 1 [(.var 1)]) (.«let» 2 .nat (.call 2 [(.var 1)]) (.cond (.call 3 [(.var 1), (.var 2)]) (.build .ok (.result .nat (.pair .bool .bool)) [(.var 2)]) (.build .error (.result .nat (.pair .bool .bool)) [(.build .pair (.pair .bool .bool) [(.build .true .bool []), (.build .false .bool [])])]))) (.build .error (.result .nat (.pair .bool .bool)) [(.build .pair (.pair .bool .bool) [(.build .false .bool []), (.build .false .bool [])])]))) },
    { parameters := [0], types := [.nat], result := .bool,
      body := (.prim .natLe [(.var 0), (.value .nat (.nat 10))]) },
    { parameters := [0], types := [.nat], result := .nat,
      body := (.call 4 [(.var 0)]) },
    { parameters := [0, 1], types := [.nat, .nat], result := .bool,
      body := (.prim .natLe [(.var 1), (.var 0)]) },
    { parameters := [0], types := [.nat], result := .nat,
      body := (.prim .natSub [(.var 0), (.value .nat (.nat 1))]) }] }

def krate : RustSyntax.Crate :=
  ({ profile := .std, items := [
    (.function (.generated .function 0) [{ pattern := (.bind (.generated .binding 0)), type := .nat }] (.result .nat (.pair .bool .bool)) (.mk [(.mk (.bind (.generated .binding 1)) (some .nat) (.copy (.generated .binding 0)))] (.cond (.call (.function 1) [(.copy (.generated .binding 1))] false) (.mk [(.mk (.bind (.generated .binding 2)) (some .nat) (.call (.function 2) [(.copy (.generated .binding 1))] false))] (.cond (.call (.function 3) [(.copy (.generated .binding 1)), (.copy (.generated .binding 2))] false) (.mk [] (.construct (.ok .nat (.pair .bool .bool)) [(.copy (.generated .binding 2))])) (.mk [] (.construct (.err .nat (.pair .bool .bool)) [(.pair (.lit (.bool true)) (.lit (.bool false)))])))) (.mk [] (.construct (.err .nat (.pair .bool .bool)) [(.pair (.lit (.bool false)) (.lit (.bool false)))]))))),
    (.function (.generated .function 1) [{ pattern := (.bind (.generated .binding 0)), type := .nat }] .bool (.mk [(.mk (.bind (.generated .operand 1)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 2)) none (.lit (.nat 10)))] (.call (.runtime .natLe) [(.move (.generated .operand 1)), (.move (.generated .operand 2))] false))),
    (.function (.generated .function 2) [{ pattern := (.bind (.generated .binding 0)), type := .nat }] .nat (.mk [] (.call (.function 4) [(.copy (.generated .binding 0))] false))),
    (.function (.generated .function 3) [{ pattern := (.bind (.generated .binding 0)), type := .nat }, { pattern := (.bind (.generated .binding 1)), type := .nat }] .bool (.mk [(.mk (.bind (.generated .operand 3)) none (.copy (.generated .binding 1))), (.mk (.bind (.generated .operand 4)) none (.copy (.generated .binding 0)))] (.call (.runtime .natLe) [(.move (.generated .operand 3)), (.move (.generated .operand 4))] false))),
    (.function (.generated .function 4) [{ pattern := (.bind (.generated .binding 0)), type := .nat }] .nat (.mk [(.mk (.bind (.generated .operand 5)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 6)) none (.lit (.nat 1)))] (.call (.runtime .natSub) [(.move (.generated .operand 5)), (.move (.generated .operand 6))] false)))] } : LexLeanTarget.RustSyntax.Crate)

def flags : Flags := []

theorem fun0 : FunOK program krate flags 0 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .binding 1)) (some .nat) (.copy (.generated .binding 0)))], (.cond (.call (.function 1) [(.copy (.generated .binding 1))] false) (.mk [(.mk (.bind (.generated .binding 2)) (some .nat) (.call (.function 2) [(.copy (.generated .binding 1))] false))] (.cond (.call (.function 3) [(.copy (.generated .binding 1)), (.copy (.generated .binding 2))] false) (.mk [] (.construct (.ok .nat (.pair .bool .bool)) [(.copy (.generated .binding 2))])) (.mk [] (.construct (.err .nat (.pair .bool .bool)) [(.pair (.lit (.bool true)) (.lit (.bool false)))])))) (.mk [] (.construct (.err .nat (.pair .bool .bool)) [(.pair (.lit (.bool false)) (.lit (.bool false)))]))), [(0, .nat, (some 0))], rfl, rfl,
    (Corr.letBind (Corr.var rfl rfl) rfl (Corr.cond (lc := []) (ce := (.call (.function 1) [(.copy (.generated .binding 1))] false)) (lk := []) (Corr.callOps (ts := [.nat]) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lNil))) rfl rfl rfl rfl rfl) (Corr.kIf (la := [(.mk (.bind (.generated .binding 2)) (some .nat) (.call (.function 2) [(.copy (.generated .binding 1))] false))]) (ta := (.cond (.call (.function 3) [(.copy (.generated .binding 1)), (.copy (.generated .binding 2))] false) (.mk [] (.construct (.ok .nat (.pair .bool .bool)) [(.copy (.generated .binding 2))])) (.mk [] (.construct (.err .nat (.pair .bool .bool)) [(.pair (.lit (.bool true)) (.lit (.bool false)))])))) (lb := []) (tb := (.construct (.err .nat (.pair .bool .bool)) [(.pair (.lit (.bool false)) (.lit (.bool false)))])) (Corr.letBind (Corr.eOfBNil (Corr.callOps (ts := [.nat]) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lNil))) rfl rfl rfl rfl rfl)) rfl (Corr.cond (lc := []) (ce := (.call (.function 3) [(.copy (.generated .binding 1)), (.copy (.generated .binding 2))] false)) (lk := []) (Corr.callOps (ts := [.nat, .nat]) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lCons (Corr.var rfl rfl) (Corr.lNil)))) rfl rfl rfl rfl rfl) (Corr.kIf (la := []) (ta := (.construct (.ok .nat (.pair .bool .bool)) [(.copy (.generated .binding 2))])) (lb := []) (tb := (.construct (.err .nat (.pair .bool .bool)) [(.pair (.lit (.bool true)) (.lit (.bool false)))])) (Corr.build (ts := [.nat]) (mask := [false]) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lNil))) rfl) (Corr.build (ts := [(.pair .bool .bool)]) (mask := [false]) (Corr.opsOfL (Corr.lCons (Corr.buildPair (Corr.lCons (Corr.buildConst rfl) (Corr.lCons (Corr.buildConst rfl) (Corr.lNil)))) (Corr.lNil))) rfl)))) (Corr.build (ts := [(.pair .bool .bool)]) (mask := [false]) (Corr.opsOfL (Corr.lCons (Corr.buildPair (Corr.lCons (Corr.buildConst rfl) (Corr.lCons (Corr.buildConst rfl) (Corr.lNil)))) (Corr.lNil))) rfl))))⟩⟩

theorem fun1 : FunOK program krate flags 1 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .operand 1)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 2)) none (.lit (.nat 10)))], (.call (.runtime .natLe) [(.move (.generated .operand 1)), (.move (.generated .operand 2))] false), [(0, .nat, (some 0))], rfl, rfl,
    (Corr.prim (ts := [.nat, .nat]) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsCons (Corr.value rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl)⟩⟩

theorem fun2 : FunOK program krate flags 2 :=
  ⟨_, rfl, .inl ⟨_, _, [], (.call (.function 4) [(.copy (.generated .binding 0))] false), [(0, .nat, (some 0))], rfl, rfl,
    (Corr.callOps (ts := [.nat]) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lNil))) rfl rfl rfl rfl rfl)⟩⟩

theorem fun3 : FunOK program krate flags 3 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .operand 3)) none (.copy (.generated .binding 1))), (.mk (.bind (.generated .operand 4)) none (.copy (.generated .binding 0)))], (.call (.runtime .natLe) [(.move (.generated .operand 3)), (.move (.generated .operand 4))] false), [(1, .nat, (some 1)), (0, .nat, (some 0))], rfl, rfl,
    (Corr.prim (ts := [.nat, .nat]) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl)⟩⟩

theorem fun4 : FunOK program krate flags 4 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .operand 5)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 6)) none (.lit (.nat 1)))], (.call (.runtime .natSub) [(.move (.generated .operand 5)), (.move (.generated .operand 6))] false), [(0, .nat, (some 0))], rfl, rfl,
    (Corr.prim (ts := [.nat, .nat]) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsCons (Corr.value rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl)⟩⟩

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

end LexLeanPreserve.C9b67141c19e48429ed1a6443d832fde7.R7.RustStd
