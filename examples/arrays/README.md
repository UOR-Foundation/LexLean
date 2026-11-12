# arrays

Language-1.2 arrays of fixed-width integers (SPEC.md §17.12, *Arrays*): an int8 dot product computed by index over two arrays, proved equal for all arrays to its definition by recursion on two lists, and a rectification loop that writes an array in place and is proved to keep its length.

- **Language:** 1.2. **Source modules:** 1 under `src/`.
- **Register rows whose gates run on it** (`model/ids.toml`, level `build`): `SM-39`, and `EX-08` (every example is discovered and must pass the full example gate).
- **What is claimed.** `cargo xtask verify-examples` formats, locks, checks and builds the project, verifies it under the pinned `leanprover/lean4:v4.32.1` (elaboration, `leanchecker` replay, exact axiom audit), and compares its build outputs and normalized verification records with `expected/` byte for byte (SPEC.md §28.6, §28.3, §29.5). The theorem `dot_equals_list` holds for every pair of arrays, with observed axioms `Quot.sound` and `propext`; `rectify_keeps_length` holds for every array, with `propext`.
- **What is not claimed.** Nothing about speed. That the pinned Lean indexes an array in constant time and writes in place when the array is referenced once is a property of Lean, cited as `LEAN-ARRAY-4-32-1` in `model/authorities.toml` and not established here. An array is formal-only for production (SPEC.md §17.13): this example declares no production root and has no certified Rust package.
- **Run.** From this directory, with the `lexlean` binary and the pinned toolchain installed: `lexlean check` and `lexlean verify`. The comparison with `expected/` is the gate's: `cargo xtask verify-examples` from the repository root.
