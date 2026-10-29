module
public import Init
set_option autoImplicit false

/-! The host library of the `VR-25` fixture: one clean theorem and four that
plant the axioms an import must not smuggle in. Each states the commutativity
of `Nat` addition, so one entry shape denotes all five. The planted ones prove
it from `Nat.add_comm` and merely carry a hypothesis supplied by the axiom, so
that their observed axioms are exactly those named below. -/

meta def alwaysTrue : Bool := true

/-- No axiom. -/
public theorem AxHost.clean (a b : Nat) : a + b = b + a := Nat.add_comm a b

/-- Observed axioms: `sorryAx`. -/
public theorem AxHost.viaSorry (a b : Nat) : a + b = b + a := sorry

/-- Observed axioms: `Lean.ofReduceBool` and `Lean.trustCompiler`, the latter
because `Lean.ofReduceBool` itself rests on it. -/
public theorem AxHost.viaReduce (a b : Nat) : a + b = b + a :=
  (fun (_ : alwaysTrue = true) => Nat.add_comm a b) (Lean.ofReduceBool alwaysTrue true rfl)

/-- Observed axioms: `Lean.trustCompiler` alone. -/
public theorem AxHost.viaTrust (a b : Nat) : a + b = b + a :=
  (fun (_ : True) => Nat.add_comm a b) Lean.trustCompiler

/-- Observed axioms: the axiom `native_decide` generates for this declaration
under Lean 4.32.1, `AxHost.viaNative._native.native_decide.ax_1_1`, which Lean
prints with a hygiene mark. -/
public theorem AxHost.viaNative (a b : Nat) : a + b = b + a :=
  (fun (_ : 2 + 2 = 4) => Nat.add_comm a b) (by native_decide)
