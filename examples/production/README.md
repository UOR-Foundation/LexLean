# production

Language-1.2 production eligibility: declared production roots across two modules beside formal-only theorems, with each root's closure, effects and dispositions published as an eligibility report, and verification extracting the roots through Lean's compiler front end.

- **Language:** 1.2. **Source modules:** 2 under `src/`.
- **Register rows that name it** (`model/ids.toml`, level `build`): `PD-01`..`PD-06`, `NE-01`, `PX-01`, `PX-02`, `SP-02`, `SP-05`, `SP-08`, `VR-13`, `OR-02`, and `EX-08` (every example is discovered and must pass the full example gate).
- **What is claimed.** `cargo xtask verify-examples` formats, locks, checks and builds the project, verifies it under the pinned `leanprover/lean4:v4.32.1` (elaboration, `leanchecker` replay, exact axiom audit), and compares its build outputs and normalized verification records with `expected/` byte for byte (SPEC.md §28.3, §29.5).
- **What is not claimed.** Its production certificates (`SP-*`) are `build`-level evidence: a kernel-checked relation between each lowered program and its source denotation, and a rustc differential on the declared Rust machine. The differential against `lean4-prod` (`OR-*`) is validation evidence only. No GNAF request or optimality certificate is attached to this example, so it makes no optimality claim.
- **Run.** From this directory, with the `lexlean` binary and the pinned toolchain installed: `lexlean check` and `lexlean verify`. The comparison with `expected/` is the gate's: `cargo xtask verify-examples` from the repository root.
