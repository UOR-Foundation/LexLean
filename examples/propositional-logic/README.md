# propositional-logic

Reasoning over `Prop`-typed locals: commutativity and associativity of conjunction and disjunction, disjunction elimination, double negation, De Morgan, explosion, biconditionals, and classical double-negation elimination under an exact axiom policy.

- **Language:** 1.0. **Source modules:** 1 under `src/`.
- **Register rows that name it** (`model/ids.toml`, level `build`): `EX-08` (every example is discovered and must pass the full example gate).
- **What is claimed.** `cargo xtask verify-examples` formats, locks, checks and builds the project, verifies it under the pinned `leanprover/lean4:v4.32.1` (elaboration, `leanchecker` replay, exact axiom audit), and compares its build outputs and normalized verification records with `expected/` byte for byte (SPEC.md §28.3, §29.5).
- **What is not claimed.** Nothing beyond what the rows above and the statements in `src/` say.
- **Run.** From this directory, with the `lexlean` binary and the pinned toolchain installed: `lexlean check` and `lexlean verify`. The comparison with `expected/` is the gate's: `cargo xtask verify-examples` from the repository root.
