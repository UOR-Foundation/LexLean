# higher-order

Language-1.2 first-class functions: generic `mapList`, `foldList` and `compose`, lambdas with explicit captures, definition references across modules, and executable definitions that pass only non-escaping closures.

- **Language:** 1.2. **Source modules:** 2 under `src/`.
- **Register rows that name it** (`model/ids.toml`, level `build`): `DF-14`, `DF-15`, `SM-25`, `SM-26`, and `EX-08` (every example is discovered and must pass the full example gate).
- **What is claimed.** `cargo xtask verify-examples` formats, locks, checks and builds the project, verifies it under the pinned `leanprover/lean4:v4.32.1` (elaboration, `leanchecker` replay, exact axiom audit), and compares its build outputs and normalized verification records with `expected/` byte for byte (SPEC.md §28.3, §29.5).
- **What is not claimed.** Nothing beyond what the rows above and the statements in `src/` say.
- **Run.** From this directory, with the `lexlean` binary and the pinned toolchain installed: `lexlean check` and `lexlean verify`. The comparison with `expected/` is the gate's: `cargo xtask verify-examples` from the repository root.
