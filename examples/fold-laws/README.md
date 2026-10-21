# fold-laws

Language-1.2 program equalities over all inputs: clamped requantization, truncating division, fixed-width order, and additive folds over lists, sets and maps proved by `linear_arithmetic` and the `law` proof form.

- **Language:** 1.2. **Source modules:** 3 under `src/`.
- **Register rows whose gates run on it** (`model/ids.toml`, level `build`): `PF-20`, `PF-21`, `PF-22`, `PF-23`, and `EX-08` (every example is discovered and must pass the full example gate).
- **What is claimed.** `cargo xtask verify-examples` formats, locks, checks and builds the project, verifies it under the pinned `leanprover/lean4:v4.32.1` (elaboration, `leanchecker` replay, exact axiom audit), and compares its build outputs and normalized verification records with `expected/` byte for byte (SPEC.md §28.6, §28.3, §29.5).
- **What is not claimed.** Nothing beyond what the rows above and the statements in `src/` say.
- **Run.** From this directory, with the `lexlean` binary and the pinned toolchain installed: `lexlean check` and `lexlean verify`. The comparison with `expected/` is the gate's: `cargo xtask verify-examples` from the repository root.
