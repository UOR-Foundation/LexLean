# recursion

Language-1.2 recursion: mutual structural groups over nested and mutual families, and well-founded definitions whose per-call decrease obligations are proved by `linear_arithmetic` and bound as explicit evidence.

- **Language:** 1.2. **Source modules:** 2 under `src/`.
- **Register rows whose gates run on it** (`model/ids.toml`, level `build`): `DF-16`, `DF-17`, `SM-27`, `PF-19`, and `EX-08` (every example is discovered and must pass the full example gate).
- **What is claimed.** `cargo xtask verify-examples` formats, locks, checks and builds the project, verifies it under the pinned `leanprover/lean4:v4.32.1` (elaboration, `leanchecker` replay, exact axiom audit), and compares its build outputs and normalized verification records with `expected/` byte for byte (SPEC.md §28.6, §28.3, §29.5).
- **What is not claimed.** Nothing beyond what the rows above and the statements in `src/` say.
- **Run.** From this directory, with the `lexlean` binary and the pinned toolchain installed: `lexlean check` and `lexlean verify`. The comparison with `expected/` is the gate's: `cargo xtask verify-examples` from the repository root.
- **Titles.** Every module of this example is titled “Natural number addition”, carried over from the literal example; #251 retitles the seven examples that carry it.
