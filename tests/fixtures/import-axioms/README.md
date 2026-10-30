# import-axioms

The fixture of `VR-25` and `VR-26` (SPEC.md §5.3, §18.8, §22.6).

- `project/AxHost.lean`: a host library of five theorems that state the
  commutativity of `Nat` addition. `clean` carries no axiom; `viaSorry`,
  `viaReduce`, `viaTrust` and `viaNative` plant `sorryAx`, `Lean.ofReduceBool`
  (which rests on `Lean.trustCompiler`), `Lean.trustCompiler`, and the axiom
  `native_decide` generates.
- `project/`: a project whose five entries (`test.ax::clean-comm` and the four
  others) denote them. The cases copy it, run `lake build AxHost`, write the
  document each case needs, lock, and verify; they carry no lock and no build
  output.
