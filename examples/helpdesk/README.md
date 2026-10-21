# helpdesk

A text application written entirely from language-1.2 source and the source packages `std`, `std-models` and `std-reasoning`: `dispatch : Bytes -> Bytes` scores a request with an integer feed-forward model, abstains below a margin, commits through the generic `GateModel`, and plans the reply with the `Drain` reasoner; `classify` uses the model alone and `plan` the reasoner alone.

- **Language:** 1.2. **Source modules:** 3 under `src/`.
- **Register rows that name it** (`model/ids.toml`, level `build`): `EX-09`, `SL-03`, `SL-04`, and `EX-08` (every example is discovered and must pass the full example gate).
- **What is claimed.** `cargo xtask verify-examples` formats, locks, checks and builds the project, verifies it under the pinned `leanprover/lean4:v4.32.1` (elaboration, `leanchecker` replay, exact axiom audit), and compares its build outputs and normalized verification records with `expected/` byte for byte (SPEC.md §28.3, §29.5).
- **What is not claimed.** The claims are those of the rows above: the roots verify under the pinned Lean and export through `lexlean production build` with the packages committed under `tests/production-export/helpdesk/`. Nothing is claimed about the quality of a reply, and no GNAF request or optimality certificate is attached.
- **Run.** From this directory, with the `lexlean` binary and the pinned toolchain installed: `lexlean check` and `lexlean verify`. The comparison with `expected/` is the gate's: `cargo xtask verify-examples` from the repository root.
