# Mathlib import: replay cache notes (issue #338)

Working notes for the design document (#341). Branch `feat/mathlib-replay-cache`, from main `71defb4`. Nothing here is pushed.

## What exists

A content-addressed replay cache in the `lexlean` crate: `verify::replay_cache` (keys, entries, facts, driver) and `artifact::digest_set` (the DigestSet). `lexlean verify` does not use it yet; it is a library a caller drives with a store directory. SPEC.md §22.13 states it, with rows `RC-01` to `RC-06` in `model/ids.toml`, scenarios in `features/suites/replay-cache.feature`, and cases in `crates/conformance/src/cases/replay_cache.rs`.

## Reuse of PrismPM

LexLean had no BLAKE3 and no DigestSet; it had `sha2` only (`Sha256Digest`). PrismPM ADR-015 (`docs/adr/015-dual-digests.md` in the PrismPM repository) fixes the format, and PrismPM's own code (`crates/prismpm/src/input_lock.rs`, `digest_set`) uses the `blake3` crate pinned `=1.5.5`. `DigestSet` here follows the ADR: members `sha256` and `blake3` only, lowercase hex of 64 characters, both over the same bytes, label `blake3:<hex>`, reader refuses a missing, extra, uppercase or differently spelled member. Known answers for the empty string and `abc` are unit tests.

The Windows-side PrismPM folder named in the brief holds only `.claude`; ADR-015 and the Rust usage were read from an existing WSL clone of PrismPM (read-only).

## Key format

All frames are fields `tag SP decimal-length ":" payload LF`, so no payload can run into its neighbour.

Canonical declaration (`canonical_declaration`, the one function): `format` (`lexlean/replay-declaration/1`), `name`, `levels` count and each `level`, `attrs`, `type`, then the value as `value-sha256` and `value-blake3` of its text, or `value none`. The declaring module is excluded, so moving a declaration moves no key.

Declaration key: DigestSet of a frame of `format` (`lexlean/replay-key/1`), `toolchain`, `mathlib` (revision), `declaration` (the canonical bytes), the group digest (both digests), and the sorted, de-duplicated keys of what it names outside its group (both digests each).

Group: declarations that name one another (an inductive type, its constructors and recursor) are a strongly connected component; the group digest is the DigestSet of the canonical bytes of the members in name order. Members keep distinct keys. Components are found with an iterative Tarjan, because a chain of 50,000 lemmas is a unit test and must not use the process stack.

Unit key: DigestSet of a frame of `lexlean/replay-unit/1`, the granularity word (`declaration` or `module`), and the sorted keys of the members. A unit is just a set of declaration keys, so the cache works for either granularity sibling #337 picks.

## Facts from Lean

`crates/lexlean/src/verify/replay_facts.lean` runs under the pinned `lean --run` through `lake env`. For a module it writes one JSON line per constant the module declares and per constant those reach (transitive closure), so each module's facts are self-contained and need no anchors. A term is an alpha-equivalence class (no binder names, binder annotations or metadata) written as a node table where a shared subterm is one node. Inductive types, constructors and recursors carry their kernel attributes and recursor rules, so a changed constructor list changes the key. Output is byte-identical across two runs.

`ReplayGraph::build` also takes `anchors` (name to key) for declarations a caller keyed in an earlier run, which is the route for keeping per-module fact files small for Mathlib.

## Cache record layout

`<store>/entries/<first two hex of blake3>/<blake3 hex>.json`, canonical JSON (§21.7), written to a temporary file and renamed:

```
entry_digest  DigestSet of the canonical string of the other members
environment   {mathlib_revision, toolchain}
executable    {digest: DigestSet, tool: "leanchecker"}   the executable that produced the result
key           DigestSet of the unit
result        {exit_code: 0, stderr, stdout}
spec          "lexlean/replay-cache-entry/1"
unit          {granularity, label}
```

Only a replay that exited zero is recorded. An entry produced by another executable is a miss and the replay overwrites it.

## Diagnostics

`LLV7019` replay cache entry rejected: does not parse or has the wrong members, `entry_digest` mismatch (bytes changed), recorded key is not the key looked up (stored under another key), environment differs from the caller's, not a regular file, or records a nonzero exit. The run stops; the file is left as found and the replayer is not called. `LLV7020` replay facts refused: a line does not parse, a declaration repeats, a name is neither declared nor anchored, or the exporter failed. Both are in `model/errors.toml`, SPEC §26.3, and have negative tests. `LLV7010`, `LLV7003` and `LLS8002` are reused.

## Test evidence

Red first, as the repository's R3 prescribes: with the register rows, scenarios and the six tests added and no case wired, `cargo test -p repo-conformance --test conformance conformance_rc_0 -- --test-threads=1` failed all six with `no conformance case is wired for RC-0N` (3:32 wall, peak 2.6 GB RSS).

After implementation, in WSL under `~/bin/heavy`:

| Run | Result | Wall | Peak RSS |
| --- | --- | --- | --- |
| `cargo test -p lexlean --lib -- replay_cache digest_set` | 28 passed | 20 s (incremental) | 1.74 GB |
| `cargo test -p repo-conformance --test conformance conformance_rc_0` | 6 passed (RC-01 to RC-06, RC-06 on real Lean) | 1:11 incl. build | 1.92 GB |
| `cargo fmt --all -- --check`, `cargo xtask validate-model`, `validate-spec-links`, `cargo clippy --workspace --all-targets --all-features -D warnings`, `cargo test -p repo-conformance --test bdd`, `conformance_rp_07` | all pass | 3 s to 98 s each | at most 1.76 GB |
| `cargo deny check licenses bans sources` | pass (advisories not run: offline) | 0.6 s | 70 MB |

The only gate not run is the full `just vv` (examples, golden, repro, the whole bdd suite): each takes hours of Lean and the change touches none of their inputs, but it was not run here.

Required tests, by row: tamper `RC-04` (result edited with digest kept, executable digest edited, copied under another key, truncated, emptied, non-regular file, forged environment with a recomputed digest, forged nonzero exit); changed dependency `RC-02` (`base` edit changes `base`, `left`, `right`, `top` and not `lone`; `left` edit changes `left`, `top`; mutual group); changed toolchain or Mathlib revision `RC-05` (every unit replays, the three environments' 15 entries coexist unchanged, returning to the first environment reuses all).

Planted defects (to show the tests can fail), recorded in VERIFICATION.md: with the `entry_digest` check disabled, the bytes-changed and the executable-edited tamper tests fail (2 of 8); with the toolchain and Mathlib revision removed from the key frame, the two environment tests fail (2 of 25). Both plants were removed afterwards.

## Warm versus cold, and the measurement that matters

RC-06 compiles three tiny modules `A <- B <- C` (C imports B but uses only A), exports facts through the pinned Lean, keys them at module granularity, and replays with `lake env leanchecker`. A one-line edit to a declaration of B (`b := a + 1` to `a + 3`) rebuilds B and C. Result: A and C reused, only B replayed. The warm verdicts equal a cold run on the edited modules.

Timings, two runs on a machine that other sessions were loading (load average 8 to 11), so the spread is real and the figures indicative only. Ranges are the two runs:

| Phase | Facts and keys | `leanchecker` replays |
| --- | --- | --- |
| cold, 3 modules | 11.2 to 13.8 s | 1.2 to 1.4 s |
| warm, nothing changed | 6.6 to 7.9 s | 0 s |
| warm, one-line edit (B edited, B and C rebuilt) | 8.0 to 15.9 s | 0.5 to 0.6 s |
| cold again on the edited modules | 6.8 to 12.5 s | 1.3 to 1.4 s |

Replay of these modules costs about 0.4 s each; exporting and keying the facts costs about 2 to 5 s per module in this setup (three exports per phase). **For small modules the facts export costs more than the replay it saves.** Core modules measured alone (same loaded machine): `Init.Data.Nat.Basic` closure 878 constants, 0.9 MB, 7 s wall (1.9 s on an idle run); `Init.Data.List.Lemmas` 2,736 constants, 5.6 MB, 3.3 s; `Std.Data.HashMap.Lemmas` 7,246 constants, 24 MB, 8 s. Peak RSS of the exporter is 1.4 to 1.7 GB, mostly Lean loading its own environment. So the cache saves time only where `leanchecker` on a unit costs more than exporting and keying it. That has to be measured on Mathlib modules with #337's numbers before the design commits; the two obvious improvements are running one Lean process that exports many modules (the load cost is paid once), and compiling the exporter instead of interpreting it with `lean --run`.

Peak memory of the cache itself is small: keys are 64 bytes per declaration plus the facts in memory (24 MB for the largest module here).

## Open owner decisions

1. SPEC amendment. §22.13 is new. §10.5 says "no cache of a built package" and §9.4 says the gate has no cache; both stay true because the cache is opt-in and the gate never uses it, but the owner should confirm that reading, and decide whether `lexlean verify` may ever consult a store.
2. Dependency. `blake3 =1.5.5` (as PrismPM pins) with `pure` (no C or SIMD build) joins the shipped crate, SPEC §8.4 table, and `deny.toml` gains `BSD-2-Clause` for `arrayref`, a transitive dependency of `blake3`. A new licence in the allow list is a reviewer's call by the file's own words.
3. Trust. `entry_digest` detects changed bytes and a mismatch with the key; it is not authentication, since whoever can write the store recomputes it (the forged-environment test shows a self-consistent forgery is refused only because the environment differs). A reused result is evidence of one earlier replay by the named executable, not of a replay by this run. Whether an attestation must mark reused units, and whether signing is wanted, is open. Status wording must stay "replayed in the pinned environment".
4. A rejected entry stops the run (fail closed, file untouched). Evict-and-replay is a policy alternative the owner may prefer.
5. Executable identity: the record stores a DigestSet of `leanchecker`; the existing preflight hashes only SHA-256 (and reads the whole file into memory). The cache hashes the executable once more, in 64 KiB pieces. Reusing one pass is possible.
6. Facts size: the facts of `Std.Data.HashMap.Lemmas` are 24 MB, over the default `max_child_output_bytes` (16 MiB), so RC-06 raises the limit to 64 MiB for its own run. For Mathlib, anchors or a per-module facts store are needed.
7. Alpha-equivalence: keys ignore binder names and annotations, so a rename of a bound variable is not a change. A more conservative key would include them.
8. Scope not done: no CLI command, no `lexlean verify` wiring, no Mathlib run (the pinned checkout `~/mathlib-pinned` did not exist when this work ran), no per-declaration `leanchecker` mode (#337).
