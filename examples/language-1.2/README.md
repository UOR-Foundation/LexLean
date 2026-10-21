# language-1.2

The smallest language-1.2 project (SPEC.md §17.12): a `lexlean/semantic-module/2` module using the 1.2-only typed `let` term, verified with an empty observed axiom set.

- **Language:** 1.2. **Source modules:** 1 under `src/`.
- **Register rows that name it** (`model/ids.toml`, level `build`): `EX-08` (every example is discovered and must pass the full example gate).
- **What is claimed.** `cargo xtask verify-examples` formats, locks, checks and builds the project, verifies it under the pinned `leanprover/lean4:v4.32.1` (elaboration, `leanchecker` replay, exact axiom audit), and compares its build outputs and normalized verification records with `expected/` byte for byte (SPEC.md §28.3, §29.5).
- **What is not claimed.** Nothing beyond what the rows above and the statements in `src/` say.
- **Run.** From this directory, with the `lexlean` binary and the pinned toolchain installed: `lexlean check` and `lexlean verify`. The comparison with `expected/` is the gate's: `cargo xtask verify-examples` from the repository root.
