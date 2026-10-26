# uor-atlas

The complete native Atlas declaration and proof graph (86 modules: 80 Atlas modules and 6 under `Prelude/`), including the census/group chain, `S37`, `S38` and the integer-uniqueness statement `S43`; no handwritten Atlas module is a generated dependency. CI verifies it as sharded parts of one run.

- **Language:** 1.0. **Source modules:** 86 under `src/`.
- **Register rows whose gates run on it** (`model/ids.toml`, level `build`): `VR-19`, `VR-23`, and `EX-08` (every example is discovered and must pass the full example gate).
- **What is claimed.** `cargo xtask verify-examples` formats, locks, checks and builds the project, verifies it under the pinned `leanprover/lean4:v4.32.1` (elaboration, `leanchecker` replay, exact axiom audit), and compares its build outputs and normalized verification records with `expected/` byte for byte (SPEC.md §28.6, §28.3, §29.5).
- **What is not claimed.** Nothing beyond what the rows above and the statements in `src/` say.
- **Run.** From this directory, with the `lexlean` binary and the pinned toolchain installed: `lexlean check` and `lexlean verify`. The comparison with `expected/` is the gate's: `cargo xtask verify-examples` from the repository root.
