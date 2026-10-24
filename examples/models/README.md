# models

Language-1.2 models across nine modules: a stateful session, a rule policy, a statistical triage, an integer feed-forward digit recognizer whose weights are content-addressed artifacts, ledgers, and composite models, with claims discharged by statement-exact theorems and production roots that apply models through their required runtime checks.

- **Language:** 1.2. **Source modules:** 9 under `src/`.
- **Register rows whose gates run on it** (`model/ids.toml`, level `build`): `MD-08`, `MD-11`, `MD-12`, `MD-13`, `SP-02`, `OR-02`, and `EX-08` (every example is discovered and must pass the full example gate).
- **What is claimed.** `cargo xtask verify-examples` formats, locks, checks and builds the project, verifies it under the pinned `leanprover/lean4:v4.32.1` (elaboration, `leanchecker` replay, exact axiom audit), and compares its build outputs and normalized verification records with `expected/` byte for byte (SPEC.md §28.6, §28.3, §29.5).
- **What is not claimed.** A model's claims are the theorems the example states and the checks it applies at run time; no accuracy or generalization claim about a model is made. Its production certificates are `build`-level evidence and the differential rows (`OR-*`) are validation evidence only; no GNAF request or optimality certificate is attached.
- **Run.** From this directory, with the `lexlean` binary and the pinned toolchain installed: `lexlean check` and `lexlean verify`. The comparison with `expected/` is the gate's: `cargo xtask verify-examples` from the repository root.
