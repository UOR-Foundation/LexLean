# reasoning

Language-1.2 reasoning machines across five modules: a clinical forward expert system with its invariant and termination proved, a terminating countdown, a breadth-first planner, depth-first and generate-and-verify reasoners, and production roots extracted through Lean. Their transcription to calculus programs, held against the reasoners themselves, is in the `compiler` project (`RS-12`), not in this example's sources.

- **Language:** 1.2. **Source modules:** 5 under `src/`.
- **Register rows whose gates run on it** (`model/ids.toml`, level `build`): `RS-03`, `RS-11`, `RS-12`, `RS-14`, `SP-02`, `OR-02`, and `EX-08` (every example is discovered and must pass the full example gate).
- **What is claimed.** `cargo xtask verify-examples` formats, locks, checks and builds the project, verifies it under the pinned `leanprover/lean4:v4.32.1` (elaboration, `leanchecker` replay, exact axiom audit), and compares its build outputs and normalized verification records with `expected/` byte for byte (SPEC.md §28.6, §28.3, §29.5).
- **What is not claimed.** A reasoner's claims are the theorems the example states and the checks it applies at run time. Its production certificates are `build`-level evidence and the differential rows (`OR-*`) are validation evidence only; no GNAF request or optimality certificate is attached.
- **Run.** From this directory, with the `lexlean` binary and the pinned toolchain installed: `lexlean check` and `lexlean verify`. The comparison with `expected/` is the gate's: `cargo xtask verify-examples` from the repository root.
