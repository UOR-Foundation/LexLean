# nested-induction

Language-1.2 induction over a nested inductive type: a `Json` value that holds a `List` of itself and a `List` of pairs that end in itself, mutual structural `size` and `leaves` groups over the four members of its recursive family, and `leaves_le_size`, proved by `nested_induction` with `linear_arithmetic` in each of its nine branches.

- **Language:** 1.2. **Source modules:** 1 under `src/`.
- **Register rows whose gates run on it** (`model/ids.toml`, level `build`): `SM-37`, `DF-19`, and `EX-08` (every example is discovered and must pass the full example gate).
- **What is claimed.** `cargo xtask verify-examples` formats, locks, checks and builds the project, verifies it under the pinned `leanprover/lean4:v4.32.1` (elaboration, `leanchecker` replay, exact axiom audit), and compares its build outputs and normalized verification records with `expected/` byte for byte (SPEC.md §28.6, §28.3, §29.5). The theorem is lowered to `induction value using Json.rec` with the propositions of the three auxiliary members as its motives, and its nine goals are named by position.
- **What is not claimed.** Nothing beyond what the rows above and the statements in `src/` say.
- **Run.** From this directory, with the `lexlean` binary and the pinned toolchain installed: `lexlean check` and `lexlean verify`. The comparison with `expected/` is the gate's: `cargo xtask verify-examples` from the repository root.
- **Titles.** The module is titled “Natural number addition”, carried over from the literal example; #251 retitles the examples that carry it.
