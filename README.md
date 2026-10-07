# LexLean & UORC 

A closed-lexicon LaTeX-to-Lean 4 compiler and the canonical **Universal Object Reference Compression (UORC)** implementation framework.

## Project State: Phase Mirror Governed
This repository adheres strictly to the [Phase Mirror Methodology](AGENTS.md). 
There are no vacuous claims. Every statement is physically backed by tested code or a documented formal axiom.

### Ground-Truth Claim Table
The definitive status of all features and subsystems is maintained in the canonical claim matrix:
* **Table Location:** `docs/PIRTM-README-Claim-Table.md`
* **Release Artifact:** `artifacts/PIRTM-README-Claim-Table.md`
* **Current SHA-256:** `32fdac3b9cc89fd93fef6ddf041e2c61fc6da2a18ecee02c1cf1b062bbd65836`

### Major Milestones Completed
1. **UORC Core Crates (`uorc-core`)**: Fully implemented parsing, evaluating, deterministic checkpointing, and synthesis for universal object references. 58 passing tests with 0 `clippy` warnings.
2. **Formal Theorem Closure**: Integrated `TheoremRegister` structurally into the `uorc-core` crate mapping exact Lean 4 mathematical statements to executable logic gates.
3. **Lean 4 Proof Integration**: Implemented Phase Mirror governance theorems covering Bounded Iteration and Zeno Damping inside `lean/ADR`.
4. **Axiom Ledger**: All "Proof Debts" (unproven constraints) are explicitly tracked in `docs/PIRTM-axiom-ledger.md` instead of hidden within `sorry` blocks.
5. **Zero-Drift CI Validation**: `sedona_spine_ci.yml` strictly enforces toolchain pinning (`leanprover/lean4:v4.32.1`), prevents `sorry` leaks, and mandates reproducible builds via the `just vv` acceptance boundary.

## What it does

LexLean compiles `.lex.tex` modules written in a closed controlled language: every word, symbol, and control must be declared by a versioned lexicon package, every sentence parses under a fixed grammar, and one typed intermediate representation generates both the canonical LaTeX document and the prose-free Lean 4 program. 

UORC provides the structural compression and formal runtime mapping these objects precisely down to memory-bounded state machines and synthesis pipelines.

## Acceptance Gates

The unified CI acceptance gate encompasses the Rust compiler, Lean formal models, SMT oracles, and intentional mutation-drift detection:

```text
cd packages/compression-main
just vv
```

*If `just vv` is not fully green, release readiness cannot be asserted.*
