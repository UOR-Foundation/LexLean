# Verification results reused across runs: design

Draft for owner acceptance, written 2026-11-13, against `release/v0.4` at `26e5e0ace`. Normative text is `SPEC.md`; this document explains and asks. It is a proposal only: this pull request adds this one file and changes no specification text, register row, workflow, compiler code, or generated document. It builds on two draft pull requests: #498 (every verification part names its toolchain by address) and #506 (one address scheme, `sha256:` over canonical bytes, `AR-15`).

## 1. Summary

Today every CI run proves everything again. SPEC.md §9.4 says the gate has "no cache of a ... verification tree, attestation, or proof result" and that "every process of the verification runs exactly once per run". A slice pull request that changes one bootstrap module therefore re-elaborates, replays, and audits every module of every project, UOR Atlas included.

The proposal is a content-addressed store of verifier process results:

- A stored result is reused only when its key matches exactly. The key is the address of everything the process reads and runs: its source bytes, the keys of its imports, the executables and toolchain library, its arguments, its environment, its limits, and the compiler-semantics ID.
- The stored result's own address must also check out.
- Only pushes to `release/*` write the store.
- Every push to `main` or `release/*`, and every release, re-proves everything with the store off.

**What it saves.** On a typical slice pull request, about 158 runner-minutes of Atlas and about 330 minutes of bdd and examples verification would reuse earlier results.

**What it changes.** On a pull request, a reused process was proved by an earlier release-line run on byte-identical inputs, not by that pull request's run. That is a change to what the gate verifies, so it is the owner's decision.

| # | Decision | Recommendation |
| --- | --- | --- |
| D1 | Adopt reuse by exact address on pull-request runs (the §9.4 wording in section 6) | Yes |
| D2 | The unit of reuse | One verifier process (an elaboration, a replay, an audit member), not one LexLean module |
| D3 | What a stored entry holds | The process record and its output files, never a verdict; every reused record is re-judged by this run's code |
| D4 | Who writes the store | Only pushes to `release/*`, which also re-prove everything; pull-request runs only read it |
| D5 | Whether the toolchain library tree is in the key | Yes: `lean` loads `Init`, `Std` and `Lean` from it, and the executable's digest does not fix them |
| D6 | The `lexlean` executable in the key | No: it only judges records, and judging is redone every run. The attestation still names it |
| D7 | Spot check | Each run re-runs one reused process, chosen from the run ID, and requires byte equality |
| D8 | RP-14 | It changes with D1: no cache is still forbidden, except the custody-checked store |

## 2. The unit, and why

A LexLean module becomes three processes of the verifier:

- its elaboration (`lake env lean -o <olean>`);
- its replay (`lake env leanchecker`);
- its audit member.

The partitioned Atlas verification already moves exactly these between the jobs of one run (`verify/partition.rs`). A shard exports its `.olean` files and process records, and the join re-judges every record as the stage that produced it would. `VR-23` requires the join to publish, byte for byte, what the whole verification publishes. The store is the same thing across runs: a part an earlier run produced, found by address instead of by job name.

## 3. The key, exactly

`key(p)` is the address (#506) of a framed recipe with its own domain, `lexlean-verify-step-v1`, whose frames are:

| Frame | Content |
| --- | --- |
| `step` | `elaborate`, `replay`, or `audit` |
| `module` | the full Lean module name |
| `source` | the address of the exact generated `.lean` bytes the process reads; for a replay, the key of the module's elaboration |
| `imports` | the keys of the elaborations of the module's direct imports, sorted by module. Each fixes its `.olean` and, recursively, everything below it |
| `argv`, `env` | the normalized argument vector and allow-listed environment, as the process record writes them |
| `toolchain` | the addresses of `lean`, `lake`, `leanchecker` (#498), and of the toolchain's `lib/` tree |
| `lake-workspace` | the lock hashes the attestation lists |
| `compiler-semantics` | the project language's compiler-semantics ID |
| `limits` | `child_timeout_ms` and `max_child_output_bytes` |
| `host` | operating system and architecture |

**Left out of the key, on purpose:**
- **Width, hints and the memory budget.** §22.11 already requires that they never change a byte.
- **The `lexlean` executable.** It decides how records are judged, and judging is redone on every run.
- **Timings and the run.**

**A stored entry** is `{key, record, outputs}`: the normalized process record plus the address of every output file. The output files are the `.olean`, `.ilean` and the intermediates the next elaboration reads. The entry's own address is the address of its canonical bytes. The store is `index/<key>`, which holds the entry's address, plus `objects/<address>`, which holds entries and output files.

## 4. Why reuse is sound

A process is a function of the executable and every file it loads, its arguments, its environment, the bytes it reads, and its limits. The key names all of them, directly or through import keys.

LexLean already relies on that function being deterministic, in three places:
- `VR-23` requires the partitioned Atlas to publish what the whole verification publishes;
- §22.11 requires width 1 and width N to agree;
- every gate compares normalized records against `expected/verify` (§29.5).

So equal keys give equal records and equal `.olean` bytes. The attestation built over reused records is byte-identical to a fresh one, and it does not mention reuse; telemetry and the job summary do.

**The one input that is not a function of bytes is wall time.** A process that passed in 290 s may time out at 300 s on a slower host. A timeout is an environment failure, never a verdict, so reusing the pass changes no proof claim.

## 5. What it refuses

- **A stale store.** Any changed input changes the key, so the lookup misses and the process runs. No entry is ever matched by name or path.
- **A changed toolchain.** The toolchain addresses are in the key, so it misses. On a hit, the stored record must still name this run's executables (the join's existing `LLI9001` check, extended by #498 to `lake`, `lexlean` and `rustc`).
- **A corrupted entry.** On every hit the verifier recomputes the entry's address and every output file's address, and requires them to equal the index. It also requires the entry's key to equal the key it asked for. A mismatch fails the verification; it is evidence of tampering, not a miss.
- **A consistent forgery**, by someone who can write the store and recompute the hashes. Addresses give integrity, not authority. Authority comes from custody:
  - Only a push to `release/*` writes the store.
  - Pull-request runs read it from that push run's artifact, through a custody record checked the way the bootstrap export's record is (#476, #498). The record names the repository, workflow, run, commit, and executable addresses.
  - The spot check (D7) re-runs one reused process in every run and requires byte equality.
- **Release evidence that rests on the store.** Every push to `main` or `release/*`, every release, and every `workflow_dispatch` marked `full` verifies with the store off.

## 6. The SPEC.md §9.4 wording (D1)

Replace "The gate has no path filter, no cache of a build directory, object, verification tree, attestation, or proof result, and no condition that skips a recipe of `just vv` but the plan's" with:

> The gate has no path filter, no cache of a build directory or object, and no condition that skips a recipe of `just vv` but the plan's. A verification process (§22.11) may be reused from an earlier run only by exact address: its key is the address of every input it reads, the executables and toolchain library it runs, its argument vector, environment, and limits, and the compiler-semantics ID; the reused record and output files must reproduce their recorded addresses, and the record must name this run's executables. Only a push to `release/*` writes such results, carried to later runs as an artifact with a custody record (`lexlean/ci-verify-store/1`) that `RP-14` checks; a pull request run reads them and writes none. Every push to `main` or to a release line, every release, and every `workflow_dispatch` with `full` verifies with no reuse. The records and attestation a run publishes are byte for byte those of a run with no reuse.

Two further sentences change:
- "Every process of the verification runs exactly once per run" becomes "Every process of the verification runs at most once per run and is reused only as above".
- "it, the output, and the records cross no run" (Atlas) becomes "they cross runs only through the store above".

**This changes what the gate verifies.** On a pull request, a reused process was proved by an earlier release-line run on byte-identical inputs, not by that pull request's run.

## 7. What it saves

From full gate 43046966603, using the verification seconds each job reports (gate-speed notes of 2026-11-12):

| Where | Seconds | Reusable on a slice PR |
| --- | ---: | --- |
| Atlas shards | ~9,500 | all |
| `examples` leg | 4,866 | all but the bootstrap module that changed and its importers |
| `rest` | 4,378 | models ×18 = 2,132 of it |
| `preservation` | 4,396 | production ×10 collapse to one |
| `production` | 3,385 | production ×17 collapse to one |

**Wall clock.**
- `preservation` (66 min) and `examples` (53–85 min) would drop to roughly 10–35 min.
- The Atlas chain would leave pull-request runs.
- The critical path becomes the BS-15 evaluator differential (~70 min) and the macOS AArch64 job. Differentials are not verification and do not speed up.

**Locally.**
- A second `lexlean verify` of an unchanged project would run no Lean process.
- With one module changed, it would run:
  - that module's elaboration and replay;
  - the elaborations of the modules that import it whose `.olean` changed;
  - every audit member.

  Audit members rerun on any change because the audit module's name embeds the semantic ID (`LexLeanAudit.A<semantic_hex32>`, `backend/lean.rs`), which moves with every source change.

**These numbers are estimates: the store does not exist to measure.** The pull request that implements it reports measured before and after times for a second verify of an unchanged project, for one changed module, and for a CI run of an unchanged tree.

## 8. Cost

- **Hashing the toolchain `lib/` tree.** It is 2.8 GB and took 24 s warm and 36 s cold in a shared 16 GB WSL VM. It is computed once per process, and only when the store is on.
- **Storage.** About as large as the `.olean` files of every verified project, carried as one artifact per release-line push.
- **A compiler-semantics or Lean-pin change invalidates every key,** so such pull requests pay the full price, as today.
