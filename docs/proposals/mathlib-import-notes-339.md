# Mathlib import: axiom audit and status wording (notes for #341)

Issue #339, tracking issue #333. Branch `feat/mathlib-axioms`, local commit only, from `feat/mathlib-import-integration` (`9d2af0fc7`). Register rows `VR-25` and `VR-26`; diagnostics `LLV7023`, `LLV7024`, `LLV7025`; SPEC.md §5.3, §5.4, §18.8, §22.6, §22.9 and §26.3 amended.

## What exists

An *imported declaration* is the Lean constant a used external entry denotes when its Lean module is outside `Init`, `Std`, `Lean`, `Lake` and the generated runtime root `LexLeanRuntime`. For each one:

1. The probe module (§18.8) prints `#print axioms <name>` after its `example` lines, once per distinct Lean name. This is what §5.4 defines the audit to be, and it runs in the probe process that already imports the constant.
2. `verify::axiom::parse_import_audit_output` reads the output (the §22.5 grammar, plus the spellings below).
3. `verify::imports::audit` refuses, before any elaboration, an import whose observed set holds `sorryAx` (`LLV7023`), `Lean.ofReduceBool` (`LLV7024`), or `Lean.trustCompiler` or a `native_decide` axiom (`LLV7025`), unless an `allow` or `exact` policy of some declaration of the document names that axiom.
4. The attestation gets an `imports` member (entry, module, name, observed, status), present only when there is an import, so a project that imports nothing outside the toolchain keeps its attestation and the committed `expected/verify/probe/process.json` files stay valid. The status is `replayed in the pinned environment`.

Every other axiom of an import (`Classical.choice`, `Quot.sound`, `propext`, or anything else) is judged only by the generated declarations that depend on it, as before (`LLV7005`).

## Why the replay facts are not used

The facts of #338 (`replay_facts.lean`) could carry the axiom set: each constant line has its kind (`axiom ...`) and its `deps`, so the transitive axioms are the axiom nodes reachable from a constant. They are not used because (a) §5.4 defines the audit as the set Lean's `#print axioms` returns, and the probe process already loads the imports, so one command per import costs nothing; (b) the facts export walks the whole closure of a module (28 s and 2.6 GB for `Mathlib.Order.Basic`, per the #338 notes) to answer a question about a handful of constants. A cross-check of the two over a small module would be cheap to add if the owner wants the cache to be able to report axioms for reused units.

## Findings that change the issue's premise

- **`native_decide` is not `Lean.trustCompiler` at 4.32.1.** `theorem t : 2 + 2 = 4 := by native_decide` depends on `[t._native.native_decide.ax_1_1]`, an axiom generated per use. `Lean.ofReduceBool` and `Lean.trustCompiler` appear only when written out, and `Lean.ofReduceBool` itself rests on `Lean.trustCompiler` (`#print axioms Lean.ofReduceBool` is `[Lean.ofReduceBool, Lean.trustCompiler]`). Both are deprecated in 4.32.1. The generated axiom is classified with `Lean.trustCompiler` (`LLV7025`).
- **That axiom cannot be named by a policy.** Lean prints it with a hygiene mark (U+271D) and, from a private declaration, under `_private.<Module>.0`; policies take the closed ASCII name grammar. So an import that carries it is always refused. Owner decision: leave it so (refused whatever a policy says), or let a policy name it with a prefix pattern.
- **A policy that accepts `Lean.ofReduceBool` must name `Lean.trustCompiler` too.** The case asserts the half-named policy is `LLV7025`.

## Granularity

The import check asks only that some policy of the document names the axiom. The declaration that actually uses the import is then held to its own policy by the existing check (`LLV7005`, after elaboration). So a document with one declaration `\allowaxioms{sorryAx}` and another that uses the `sorry` import under `\noaxioms` passes the early check and fails with `LLV7005` later. Per-use attribution needs the IR's use map per declaration (the checked project has `external_used` for the whole project only); it was not built. Soundness is unchanged, only the code and timing of the refusal.

## Status wording, and what it does not say

§5.3 now states `replayed in the pinned environment` and forbids `proved`, `verified` and `kernel-checked` for imports. Owner decision: **the word "replayed" is only true in the sense that the pinned Lean loaded the compiled module of the locked workspace.** §22.4 runs `leanchecker` over the generated modules only, and the #337 notes found that `leanchecker M` replays only M's own constants, not its imports. §5.3 says so ("does not assert that `leanchecker` re-checked the declaration in this run"). If the owner adopts #337's closure replay, the word becomes literal; if not, a more modest status ("loaded from the pinned environment") would be more accurate. The constant is `verify::imports::IMPORTED_STATUS`.

## Schema and identity (held back)

`schemas/attestation.schema.json` and `schemas/attestation-v2.schema.json` have `additionalProperties: false`, so an attestation with `imports` does not validate against the committed schemas. Editing either moves the compiler-semantics ID (`lib.rs`: the schema files are embedded, and `attestation-v2` is a 1.2 file), which the repository's rules say lands one at a time with one regeneration. As #334 did for its schema move, the change is held back: `339-schema-imports-member.patch` is in the shared notes folder (it adds `imports` to both schemas and nothing else). The conformance cases do not validate the attestation of the fixture against the schema for this reason. Owner decision: take the identity move with the patch, or move the `imports` rows to a staged file (`probe/imports.json`) that the attestation does not carry.

## Evidence

- Red first: `conformance_vr_25` failed with `expected LLV7023, found ["LLV7005"] (... use_sorry_comm violates its none axiom policy: observed [sorryAx])`; `conformance_vr_26` failed with `§5.3 states the wording`.
- Green, in WSL under `~/bin/heavy`, one job at a time: `conformance_vr_25` (24 s), `conformance_vr_26`, `conformance_vr_20` to `vr_24` (they verify real projects, `peano-arithmetic` among them, whose host modules `Doubling`, `Order` and `Divisibility` are now audited imports, and `specialisation`), `cargo test -p lexlean --lib -- verify::imports verify::axiom` (10 tests), `cargo test -p repo-conformance --test bdd` (5), `conformance_rp_07`, `cargo xtask validate-model`, `cargo xtask validate-spec-links` (474 rows), `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Planted defects (VERIFICATION.md): removing the `imports::audit` call fails `conformance_vr_25`; a wrong `IMPORTED_STATUS` fails `conformance_vr_26`.
- Not run: the full `just vv`, `check-golden` and `verify-examples` (hours of Lean on a loaded machine), `conformance_rp_11` (known failure on main), and `cargo deny` (no dependency changed). The examples with host modules outside the toolchain (`peano-arithmetic`, the Atlas entries in `language/uor`) now print axioms in their probes; only `peano-arithmetic` was verified here, through the operational cases.

## Register count

Total capability IDs is now 474 (README.md, SPEC.md, `xtask/src/spec_links.rs`, `crates/conformance/src/cases/repository.rs`). Another branch adding rows must recount at merge. Codes `LLV7023` to `LLV7025` are the next free after `LLV7022`.
