# production-coverage

Language-1.2 semantic-preservation coverage: production roots across eight modules that together exercise every runtime row of the production registry, each checked by certificate A during verification.

- **Language:** 1.2. **Source modules:** 8 under `src/`.
- **Register rows that name it** (`model/ids.toml`, level `build`): `PX-01`, `SP-02`, `SP-05`, `SP-08`, `SP-10`, `OR-02`, and `EX-08` (every example is discovered and must pass the full example gate).
- **What is claimed.** `cargo xtask verify-examples` formats, locks, checks and builds the project, verifies it under the pinned `leanprover/lean4:v4.32.1` (elaboration, `leanchecker` replay, exact axiom audit), and compares its build outputs and normalized verification records with `expected/` byte for byte (SPEC.md §28.3, §29.5).
- **What is not claimed.** Its production certificates (`SP-*`) are `build`-level evidence: a kernel-checked relation between each lowered program and its source denotation, and a rustc differential on the declared Rust machine. The differential rows (`OR-*`) are validation evidence only, never a proof. No GNAF request or optimality certificate is attached to this example, so it makes no optimality claim.
- **Run.** From this directory, with the `lexlean` binary and the pinned toolchain installed: `lexlean check` and `lexlean verify`. The comparison with `expected/` is the gate's: `cargo xtask verify-examples` from the repository root.
