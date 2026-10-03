import LexLeanPreservation.RustSound
set_option autoImplicit false
set_option maxRecDepth 100000
namespace LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R2.RustStd
open LexLeanTarget LexLeanPreservation.Rust

def program : TargetSyntax.Program :=
  { adts := [], functions := [
    { parameters := [0], types := [.nat], result := .nat,
      body := (.call 1 [(.closure 2 []), (.var 0)]) },
    { parameters := [0, 1], types := [(.fn [.nat] .nat), .nat], result := .nat,
      body := (.apply (.var 0) [(.apply (.var 0) [(.var 1)])]) },
    { parameters := [0], types := [.nat], result := .nat,
      body := (.prim .natAdd [(.var 0), (.var 0)]) }] }

def krate : RustSyntax.Crate :=
  ({ profile := .std, items := [
    (.enum (.fn 0) [{ number := 2, fields := [] }]),
    (.apply 0 [.nat] .nat true [{ function := 2, captures := [], functionFallible := true }]),
    (.function (.generated .function 0) [{ pattern := (.bind (.generated .binding 0)), type := .nat }] (.fallible .nat) (.mk [] (.call (.function 1) [(.construct (.closure 0 2) []), (.copy (.generated .binding 0))] false))),
    (.function (.generated .function 1) [{ pattern := (.bind (.generated .binding 0)), type := (.fn 0) }, { pattern := (.bind (.generated .binding 1)), type := .nat }] (.fallible .nat) (.mk [(.mk (.bind (.generated .callee 1)) (some (.fn 0)) (.clone (.generated .binding 0)))] (.apply (.generated .callee 1) [(.block (.mk [(.mk (.bind (.generated .callee 2)) (some (.fn 0)) (.clone (.generated .binding 0)))] (.apply (.generated .callee 2) [(.copy (.generated .binding 1))] true)))] false))),
    (.function (.generated .function 2) [{ pattern := (.bind (.generated .binding 0)), type := .nat }] (.fallible .nat) (.mk [(.mk (.bind (.generated .operand 3)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 4)) none (.copy (.generated .binding 0)))] (.call (.runtime .natAdd) [(.move (.generated .operand 3)), (.move (.generated .operand 4))] false)))] } : LexLeanTarget.RustSyntax.Crate)

def flags : Flags := [(([.nat], .nat), true)]

theorem fun0 : FunOK program krate flags 0 :=
  ⟨_, rfl, .inl ⟨_, _, [], (.call (.function 1) [(.construct (.closure 0 2) []), (.copy (.generated .binding 0))] false), [(0, .nat, (some 0))], rfl, rfl,
    (Corr.callF (ts := [(.fn [.nat] .nat), .nat]) (Corr.opsOfL (Corr.lCons (Corr.eOfBNil (Corr.closure (caps := []) (mask := []) (af := true) (ff := true) (Corr.opsNil) rfl rfl rfl rfl rfl rfl rfl)) (Corr.lCons (Corr.var rfl rfl) (Corr.lNil)))) rfl rfl rfl rfl)⟩⟩

theorem fun1 : FunOK program krate flags 1 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .callee 1)) (some (.fn 0)) (.clone (.generated .binding 0)))], (.apply (.generated .callee 1) [(.block (.mk [(.mk (.bind (.generated .callee 2)) (some (.fn 0)) (.clone (.generated .binding 0)))] (.apply (.generated .callee 2) [(.copy (.generated .binding 1))] true)))] false), [(1, .nat, (some 1)), (0, (.fn [.nat] .nat), (some 0))], rfl, rfl,
    (Corr.applyF (ps := [.nat]) (Corr.var rfl rfl) (Corr.opsOfL (Corr.lCons (Corr.eOfB (lets := [(.mk (.bind (.generated .callee 2)) (some (.fn 0)) (.clone (.generated .binding 0)))]) (tail := (.apply (.generated .callee 2) [(.copy (.generated .binding 1))] true)) (Corr.apply (ps := [.nat]) (af := true) (Corr.var rfl rfl) (Corr.opsOfL (Corr.lCons (Corr.var rfl rfl) (Corr.lNil))) rfl rfl rfl rfl)) (Corr.lNil))) rfl rfl rfl)⟩⟩

theorem fun2 : FunOK program krate flags 2 :=
  ⟨_, rfl, .inl ⟨_, _, [(.mk (.bind (.generated .operand 3)) none (.copy (.generated .binding 0))), (.mk (.bind (.generated .operand 4)) none (.copy (.generated .binding 0)))], (.call (.runtime .natAdd) [(.move (.generated .operand 3)), (.move (.generated .operand 4))] false), [(0, .nat, (some 0))], rfl, rfl,
    (Corr.primF (ts := [.nat, .nat]) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsCons (Corr.var rfl rfl) (Corr.opsNil) rfl rfl) rfl rfl) rfl rfl rfl rfl rfl rfl)⟩⟩

theorem crate_ok : CrateOK program krate flags := by
  intro f fn h
  match f with
  | 0 => exact fun0
  | 1 => exact fun1
  | 2 => exact fun2
  | _ + 3 => simp [program, index_eq] at h

/-- Every function of the program is simulated by its rendering. -/
theorem root : ∀ n f, FunSem program krate flags n f := fun n f => simulate crate_ok n f

end LexLeanPreserve.Cc9e25e0343c548a31aabc5f0ee73295b.R2.RustStd
