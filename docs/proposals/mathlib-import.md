# Mathlib import: design

Issue #341, tracking issue #333. Draft for owner acceptance, written 2026-10-29. Normative text is `SPEC.md`; this document explains and asks. The prototype is the integration branch `feat/mathlib-import-integration`, with issues #334 to #340 merged locally (nothing pushed). The acceptance demo (#340) is `examples/mathlib-blake3-g`; its results are in section 10.7. Register total on this branch: 476.

## 1. Summary

A LexLean document can use declarations of an external Lean library, Mathlib first, without LexLean ever building or vouching for it:

1. The project declares the library once, with an exact commit, in `lexlean.toml`. The Lake workspace must require the same commit.
2. `lexlean deps fetch` downloads Mathlib's prebuilt `.olean` files. No other command touches the network or builds the library.
3. `lexlean deps glossary` turns chosen Lean modules into an ordinary lexicon package of `kind = "lean"` entries. No entry is written by hand.
4. The lock pins the library's commit, its own Lake manifest and the glossary.
5. `verify` probes every used entry against the real declaration (existing §13.7), imports only the Lean module that defines each used declaration, and audits axioms.
6. Imported declarations are described as "replayed in the pinned environment". LexLean never says it proved them (§4.2, §5.3).

**Acceptance result.** The demo document (4 modules, language 1.2) verifies end to end, offline, against the pinned Mathlib: `lexlean verify` prints `verified 4 modules`, with Lean elaboration, `leanchecker` replay and the exact axiom audit passing (section 10.7). Verification takes 41 to 74 s and 1.3 GB peak in a debug build on a loaded machine. Three defects of the merged branch were found by running it and are fixed (section 12). What it does not do: it does not check Mathlib's own proofs, it needs one hand-written 13-line Lean lemma in a host library for the rotation fact, and a semantic module cannot cite an external lemma at all.

What is built and checked is listed in section 12. What is not decided is in section 14, with a recommendation on each. The decisions that need the owner first:

| # | Decision | Recommendation |
| --- | --- | --- |
| D1 | Toolchain pin | Keep `v4.32.1`. A Mathlib commit matches it, on a release line, not `master` |
| D2 | Command naming | Keep the generic `lexlean deps <verb>` group |
| D3 | Which proof forms accept external lemmas in language 1.2 | Prose proofs already do, in 1.2 (the demo). Add one form to semantic modules: `apply` taking an external entry |
| D4 | `[[lean_library]]` in language 1.2 only, or in every language | Keep 1.2 only. The conflict with D3 seen before the demo is gone |
| D5 | Replace the stock `leanchecker` step for imports with closure replay | Adopt it, as an additional stage |
| D6 | Wire the replay cache into `verify` | Not yet. It did not pay off on the two Mathlib modules measured |
| D7 | Apply the schema changes that move the language-1.2 identity | Apply them once, with the other Mathlib changes, in one regeneration |
| D16 | How CI verifies an example that needs a 7 GB library | `lexlean deps fetch` before the examples leg, or a separate leg for this example |

## 2. What the goal asks and where it stands

| Goal in #333 | Mechanism | State |
| --- | --- | --- |
| Seamless: a short config entry plus one command | `[[lean_library]]`, `deps fetch`, `deps glossary` | Partly. Fetch is one command. A Lake `require` is still written by hand, and glossary generation needs a module list. Not one command end to end |
| Fast: Mathlib is never rebuilt | Fetch of prebuilt `.olean` files, targeted imports | Imports measured (section 10.2). A full `verify` of the demo takes 40 to 53 s warm and 74 s the first time, with no reuse between runs: an edit anywhere costs a full verification (section 10.7). Seconds, not minutes; "a warm build costs seconds" is not met for a document of this size |
| Precise: pinned, signature-checked, axiom-audited, content-addressed | Lock rows, §13.7 probe, strict wrapper probe, axiom audit, replay keys | Pin, lock, probe, keys and the axiom audit of imports built (section 13), and exercised by the demo. Content addresses of `.olean` bytes are not in the lock (section 6) |

## 3. Constraints taken as given

- SPEC §4.2: imported libraries are closed external glossary entries, and their guarantees belong upstream.
- SPEC §10.4 and §5.1: the Lake workspace and its locked dependencies belong to the trusted computing base.
- I15, SPEC §11.4: `check`, `build`, `fmt` and `verify` never fetch.
- SPEC §5.4: the axiom audit reports what `#print axioms` returns, nothing more.
- Out of scope for this work: changing LexLean's toolchain pin; any claim that LexLean proved an imported declaration.
- AGENTS.md (R1 to R6) and VERIFICATION.md: every capability has a register row, a scenario and a failing test first; every diagnostic has a code in ERRORS.md and a negative test.

## 4. The pin

Mathlib commit `520045ab14e26149ee970e2e617ca04b09bde5d6` has `lean-toolchain` exactly `leanprover/lean4:v4.32.1` (read at that commit, note 334). So no toolchain change is needed.

Facts the owner should know:

- The commit is tagged `v4.32.1` in `leanprover-community/mathlib4`, dated 2026-07-23, message "chore: bump toolchain to v4.32.1".
- It is not on `master`. Its parent is `81a5d257c8e4` (the `v4.32.0` bump). `master` went from `v4.32.0` straight to `v4.33.0-rc1`. A search of `master` for `v4.32.1` finds nothing. The tags `v4.32.1` and `v4.32.2` (`905b95818eb3`, 2026-07-28) sit on a release line off `v4.32.0`.
- The pin is therefore a point release of Mathlib's `v4.32` line, frozen at July 2026 plus whatever Mathlib backported.
- Moving to `v4.32.2` would need LexLean's own pin to move to `v4.32.2`. That is outside this work.
- The config states the commit, never the tag. A tag is a mutable reference.

## 5. Config surface

### 5.1 `lexlean.toml`

```toml
language = "1.2"

[[lean_library]]
name = "mathlib"
url = "https://github.com/leanprover-community/mathlib4"
revision = "520045ab14e26149ee970e2e617ca04b09bde5d6"
```

Rules (SPEC §10.1, §10.4; register `ML-01`):

- Language 1.2 only. It follows `source_package` and `artifact_source`, so languages 1.0 and 1.1 keep their identities.
- Keys `name`, `url`, `revision`, in that order, tables sorted and unique by `name`, omitted entirely when absent (a project without a library keeps its bytes).
- `url` is HTTPS. `revision` is 40 lowercase hex. No `branch`, `tag`, `rev`, `ref`, or any other field.
- `name` is a package identifier, `[a-z][a-z0-9-]{0,62}`, and must equal the Lake package name. A Lake package with an uppercase letter or an underscore (`Qq`, `importGraph`) therefore cannot be declared. `mathlib` and `batteries` can.
- Any violation is `LLC0109` (class `cli-configuration`, exit 2).

### 5.2 Declared once, agreed with Lake

Lake needs its own `require` in the workspace lakefile and an entry in `lake-manifest.json`. `lexlean.toml` is the pin; the workspace must agree. The manifest's `packages` entry of the same `name` must have the same `url` and `rev`, else `LLC0110` names both values. LexLean writes neither the lakefile nor the manifest. A person adds the `require` and runs `lake update` once. Scaffolding that (`lexlean deps add`) is a follow-up, not built.

### 5.3 Where the library must be

The library is the Lake package at `<lean_workspace>/<packagesDir>/<name>`, `packagesDir` defaulting to `.lake/packages`. It is *present* when all of these hold:

1. the directory exists and has `.git/HEAD`;
2. `.git/HEAD` holds the declared revision;
3. it has a `lake-manifest.json` (Mathlib's own);
4. its built artifacts directory `.lake/build/lib/lean` exists.

This is the whole check. It does not read the `.olean` files or the source tree.

### 5.4 Resource limits that matter

`child_timeout_ms` (default 300000, five minutes) bounds `lake exe cache get`. A timeout is `LLS8002`, not `LLV7020`. `max_child_output_bytes` (default 16 MiB) bounds each child's output. See section 11.

## 6. Lock rows

SPEC §11.1, §11.2; register `ML-02`. Language 1.2 only, after the `[[source_package]]` rows, sorted by `name`:

```toml
[[lean_library]]
name = "mathlib"
url = "https://github.com/leanprover-community/mathlib4"
revision = "520045ab14e26149ee970e2e617ca04b09bde5d6"
lake_manifest_sha256 = "..."
glossary_sha256 = "..."
```

| Field | Value |
| --- | --- |
| `lake_manifest_sha256` | SHA-256 of `<packagesDir>/<name>/lake-manifest.json`, Mathlib's own manifest. It pins batteries, aesop, Qq, proofwidgets and the rest |
| `glossary_sha256` | Tree digest (§11.5) of the project's lexicon package `lean.<name>`, or the digest of the empty tree (`SHA-256("lexlean-tree-v1\0")`) when the project declares none |

A lock that differs from the library as it now is gives `LLC0110`, naming both values of the first field that differs.

Couplings:

- The glossary row couples to the glossary generator only by a naming convention. The generator takes any `--package <ID>`. The lock looks for the package named `lean.<name>`. Nothing forces `--package lean.mathlib`. A mismatch silently locks the empty-tree digest. See section 15.
- The lock's `lake-manifest.json` row already exists for the project's own workspace (§10.4); this row is Mathlib's, one level down.

Not pinned: the bytes of the built `.olean` files, and the bytes of the Mathlib source tree. A tampered checkout with a matching `HEAD` is not detected by the lock. Two ways to close this are measured or sketched:

- A `tree_sha256` over the Mathlib source (about 100 MB, a second or so to hash, note 334) closes the source half.
- The `.olean` half needs Lean's own check (section 8). Separately, note 337 found that `.olean` bytes are stable for a (Mathlib commit, toolchain) pair, so a digest of the fetched files is a possible lock row. This was shown on three modules only.

Identity cost: adding `lean_library` to `schemas/project-v2.schema.json` and `schemas/lock-v2.schema.json` moves the language-1.2 compiler-semantics ID. 528 tracked files carry the current ID (locks, golden files, expected outputs). It is held back as a patch (`334-schema-identity-move.patch`). Until applied, the JSON schemas do not describe `lean_library`. See D7.

## 7. Commands

### 7.1 Existing commands

| Command | Effect on a declared library |
| --- | --- |
| `lock`, `check`, `build`, `fmt`, `verify` | Resolve the library offline through `compute_lock`. Absent library: `LLV7019`, naming the library and `lexlean deps fetch`, before any Lean process runs. Even `lock --allow-network` never fetches a library (§11.4). Register `ML-04` |

`fmt` fails when the library is absent even though it does not need Mathlib. That follows from every command computing the lock.

### 7.2 `lexlean deps fetch`

SPEC §23.4, register `ML-03`. Acquires the declared libraries and nothing else.

1. Checks each declaration against the Lake workspace (`LLC0110` on disagreement).
2. Runs `lake exe cache get` in the Lake workspace through the checked child runner: cleared environment, `PATH` and `HOME` kept, `child_timeout_ms` and `max_child_output_bytes` apply.
3. Confirms the library is then present (section 5.3).

With no library declared it succeeds and fetches nothing. It writes no file outside the Lake workspace. A missing or failing `lake`, and a library not present afterwards, are `LLV7020`. Lake clones the library at the manifest's revision as part of running `lake exe`; Mathlib's `cache` executable downloads the `.olean` files.

### 7.3 `lexlean deps glossary`

SPEC §13.12, §23.4; register rows `CL-25`, `GL-19`, `GL-20`. Not a project command: like `init` it needs no project, and it never fetches or builds.

```text
lexlean deps glossary --package <ID> [--version V] [--language L]
    (--modules M... | --names N...) [--workspace DIR] --out DIR
    [--report FILE] [--strict]
```

It runs a Lean metaprogram (`crates/lexlean/lean/GlossaryExtract.lean`, compiled into the executable) as `lake env lean --run` in `--workspace`, or as plain `lean --run` over the toolchain's own libraries without one. The metaprogram prints one JSON document. Everything after that is Rust, so package bytes, codes and the report are fixed by the release and testable without Lean.

Translation, in short (full rules in §13.12):

- The input is the kernel type of the declaration, not syntax.
- `Prop` and `Type u` are expressible. Instance arguments and universe polymorphism over `Type u` are not refusal reasons.
- An application keeps only explicit-binder arguments; Lean re-infers the rest when the entry is probed.
- Every constant a signature mentions needs an entry, so the generator translates the closure of the request. The added entries are *support entries*.
- Before anything is written, each entry passes the package loader's entry and signature checks. The same Lean environment and request give byte-identical files and report.
- Each entry's `module` is the module that defines the declaration (`Environment.getModuleIdxFor?`), which is what lets `verify` import the defining module and not an umbrella.

Refusals are reported with exit 0 by default. With `--strict` any refusal fails the command, one diagnostic per refusal, and nothing is written.

Gap found by the demo (#340): the extractor imports the modules named by `--modules` and, with none, only `Init`. So `--names` cannot reach a Mathlib declaration without also requesting its whole module (thousands of entries). The demo works around it by wrapping every cited lemma in two small host modules and requesting those (15 declarations). A flag naming the modules to load, or loading the defining module of each name, is needed (D2).

### 7.4 Specialisation wrappers

SPEC §13.13, §18.8; register rows `GL-21`, `VR-24`. For a class-polymorphic declaration such as `add_comm`, a *wrapper* is a monomorphic constant in the project's host Lean library, `@add_comm Nat _`, that an entry denotes. Today the only caller is `cargo xtask emit-wrappers <request.toml> <host.lean>` and the library function `lexlean::wrappers::emit`. There is no `lexlean` subcommand (decision D2). The emitter:

- takes a closed request (`lexlean/wrappers/1`) with arguments in a closed term grammar: `_`, a name, `@name`, a decimal, parenthesised application. No tactic, `by`, `fun`, string, ascription or comment. Names are written `_root_`-qualified so a wrapper cannot capture another;
- writes `public theorem LexLeanSpec.<tag>.<declaration> : type_of% (@<declaration> <arguments>) := @<declaration> <arguments>` and nothing else, sorted, byte-identical across runs;
- never runs Lean and never writes a signature. Someone (or a generator) states the entry's LSE signature, and verification decides whether it is the wrapper's type.

The probe of a wrapper entry is stronger than that of any other external entry: `example : (<signature>) = (type_of% @<constant>) := rfl`. The ordinary probe `example : <signature> := <constant>` is weaker, because Lean inserts implicit arguments and synthesises instances while checking it, so a signature that omits an instance binder is accepted. A wrong wrapper signature is `LLT4017`; a missing wrapper is `LLT4003`.

### 7.5 What a user does today

| Step | Who | Tool |
| --- | --- | --- |
| 1. Add `require mathlib from git ... @ "<commit>"` to the lakefile, run `lake update` once | Person | Lake |
| 2. Add `[[lean_library]]` to `lexlean.toml` | Person | editor |
| 3. `lexlean deps fetch` | Person | LexLean |
| 4. `lexlean deps glossary --package lean.mathlib --workspace <ws> --modules ... --out <dir>`, then declare the output as a path lexicon package | Person | LexLean |
| 5. `lexlean lock`, then `check`, `build`, `verify` | Person | LexLean |

Not built: `deps add` (step 1), a command that writes wrappers (section 7.4), a command that picks the module list from the document's needs.

How the demo does it: `examples/mathlib-blake3-g/setup-offline.sh` hard-links an already fetched pinned checkout and its packages into the example's `.lake/packages` (no extra disk; a symlink out of the project is refused by path confinement) and runs `lake build Blake3GHost`. The wrappers are emitted by `cargo xtask emit-wrappers` from `wrappers.toml` into `Blake3GSpecs.lean`. The glossary `glossary/lean.mathlib` (38 entries) is generated by `lexlean deps glossary` from the two host modules and committed; `glossary/blake3.g` holds 8 label words and no entry denotes an imported declaration. The lock's `glossary_sha256` equals the tree digest of `lean.mathlib`, so the `lean.<name>` coupling of section 6 works when the package is named that way.

## 8. Trust statement

This is the text the design asks the owner to accept as the statement of what an imported declaration is.

1. **What is imported.** The Lean constant that a used external entry denotes (§13.6), when the entry's Lean module is outside the pinned toolchain's own libraries (`Init`, `Std`, `Lean`, `Lake`). Its guarantees belong to the upstream repository (§4.2). LexLean never claims it proved an imported declaration. No glossary entry, generated or hand-written, is evidence that anyone proved anything; it is an interface.
2. **What is checked, today.** For each used entry, the §13.7 probe elaborates in the pinned environment: the constant can inhabit the entry's signature. For a wrapper entry, the signature is definitionally the wrapper's Lean type. The generated modules are replayed by the pinned `leanchecker` (§22.4), and their axioms are audited (§22.5, §22.6).
3. **What is not checked, today.** The imported declarations themselves are not replayed. `import` loads Mathlib's `.olean` files into the environment and the generated modules' `leanchecker` run trusts the imports: it replays the module's own constants into the environment of its imports. Note 337 demonstrates this with a planted defect: module `Evil` adds a false theorem with `debug.skipKernelTC`, module `UsesEvil` imports and uses it. `leanchecker UsesEvil` exits 0. `leanchecker Evil` and closure replay (below) fail. So the imported library is trusted as part of the workspace (§5.1, item 5).
4. **Environment.** The pinned toolchain `leanprover/lean4:v4.32.1`, the Mathlib commit `520045ab14e2...`, and Mathlib's own locked dependencies (pinned by the SHA-256 of its `lake-manifest.json`). All Lean processes run through `lake env` in the Lake workspace (§22.2). Measurements were taken on x86_64 Linux (WSL2) only.
5. **Replay proposed (D5).** Closure replay (`tooling/mathlib-speed/closure-replay`, about 120 lines of Lean built with the pinned toolchain) starts from the constants the project's compiled module mentions, collects everything they transitively mention, and replays that set into an empty environment through `Lean.Environment.replay`. A mention that is not found is an error, so an incomplete closure cannot pass. It is the same kernel as `leanchecker`, not an independent checker (§4.2 already forbids calling `leanchecker` independent). If adopted, the status word is still "replayed in the pinned environment".
6. **Axioms.** The audit is an axiom-dependency audit only (§5.4). Mathlib's standard axioms are `Classical.choice`, `Quot.sound`, `propext`, the same three LexLean certificates depend on. An imported theorem's axioms are not exempt: if they flow into a generated declaration, that declaration's policy must permit them (§22.6). Measured examples from note 336: the `Nat.add_comm` wrapper depends on `[propext]`; the `BitVec 32` wrapper on `[propext, Classical.choice, Quot.sound]`. The imports themselves are audited by `#print axioms` in the probe process, and an import that depends on `sorryAx`, `Lean.ofReduceBool`, `Lean.trustCompiler` or a `native_decide` axiom is refused unless a policy of the document names it (section 13).
7. **Wording.** An import's status is "replayed in the pinned environment". The words "proved", "verified" and "kernel-checked" are reserved for the document's generated declarations (§5.3, as amended by #339). The word "replayed" is only literally true in the sense that the pinned Lean loaded the compiled module of the locked workspace: §22.4 replays the generated modules only (item 3). It becomes literal if closure replay (D5, item 5) is adopted. Without it, "loaded from the pinned environment" is the more accurate status (D15).
8. **Not covered.** A checkout whose `HEAD` matches the declared commit but whose files or `.olean` files were altered (section 6). That a glossary entry's statement matches the informal mathematics a reader expects. That a wrapper's host library is itself sound: it is a Lean library of the project, in the trusted computing base, and a wrapper adds no axiom of its own.
9. **Replay cache.** If used, the cache is an untrusted input (§5.2). `entry_digest` detects changed bytes and a mismatch with the key; it is not authentication, since whoever can write the store recomputes it. A reused result is evidence that the unit was replayed once, by the executable the entry names, in the environment its key covers. It is not evidence of a replay by this run.

## 9. Cache keys and content addresses

| Object | Identified by | Where used |
| --- | --- | --- |
| Library source | 40-hex commit | `lexlean.toml`, lock, Lake manifest |
| Library's dependencies | SHA-256 of its `lake-manifest.json` | lock |
| Generated glossary | Tree digest (§11.5) of the package `lean.<name>` | lock |
| Generated glossary bytes | Deterministic: same environment and request, same bytes | tests `GL-19`, `CL-25` |
| Wrapper module | Deterministic: same request, same bytes | test `GL-21` |
| Built `.olean` | Not pinned. Byte-stable for a (commit, toolchain) pair on three modules | note 337 |
| A declaration, for replay | DigestSet key (below) | replay cache |
| A unit of replay (declaration or module) | DigestSet of the unit frame | replay cache |

### 9.1 Replay-cache keys

SPEC §22.13; register rows `RC-01` to `RC-06`. A *DigestSet* is the PrismPM ADR-015 object `{"blake3": H, "sha256": S}`, both lowercase hex of 64 characters over the same bytes, label `blake3:<H>`. A reader refuses a missing, extra, uppercase or differently spelled member.

- **Canonical declaration.** Fields `tag SP decimal-length ":" payload LF`: format `lexlean/replay-declaration/1`, name, universe parameters, kind attributes, type, then the DigestSet of the value or `none`. A term is an alpha-equivalence class (no binder names, annotations or metadata), a shared subterm is one node. The declaring module is excluded, so moving a declaration moves no key.
- **Declaration key.** DigestSet of the frame `lexlean/replay-key/1`, the toolchain string, the Mathlib revision, the canonical declaration, the group digest, and the sorted, de-duplicated keys of what it names outside its group. A change to a declaration changes the key of everything above it and of nothing else (a Merkle DAG).
- **Group.** Declarations that name one another (an inductive type, its constructors and recursor) form a strongly connected component, found with an iterative Tarjan (a 50,000-lemma chain is a unit test). The group digest is the DigestSet of the members' canonical bytes in name order.
- **Unit key.** DigestSet of the frame `lexlean/replay-unit/1`, the granularity (`declaration` or `module`), and the sorted member keys.
- **Entry.** One canonical JSON file per unit at `entries/<first two hex of blake3>/<blake3 hex>.json`, written to a temporary file and renamed: `spec`, `key`, `unit`, `environment` (toolchain, Mathlib revision), `executable` (tool and DigestSet of the executable), `result` (`exit_code` 0, stdout, stderr), `entry_digest`. Only a replay that exited zero is recorded.
- **Facts.** `crates/lexlean/src/verify/replay_facts.lean`, run by `lean --run` through `lake env`, prints one JSON line per constant a module declares and per constant those reach.

Reuse and refusal: no file is a miss. A file that does not parse, whose `entry_digest` is wrong, whose recorded key is not the key looked up, or whose environment differs, is `LLV7021` and is neither reused nor replaced. An entry produced by another executable is a miss and the replay overwrites it. A changed toolchain or Mathlib revision changes every key, and the old entries are left intact. Facts that repeat a declaration, name one neither declared nor anchored, or do not parse are `LLV7022`.

Dependency: `blake3 =1.5.5` with `pure` (as PrismPM pins), a new member of the shipped crate and of the §8.4 table; `deny.toml` gains `BSD-2-Clause` for `arrayref`, a transitive dependency. A new licence in the allow list is a reviewer's call by the file's own words.

### 9.2 Does the cache pay off?

Not on what was measured (section 10.4). It is a library a caller drives with a store directory. `lexlean verify` does not use it, and `verify`'s gates never read a cache (§22.13, §9.4, §10.5). This keeps §10.5's "no cache of a built package" true.

## 10. Measurements

Environment for all rows unless stated: Lean 4.32.1, Mathlib `520045ab14e2` at `~/mathlib-pinned` with prebuilt `.olean` files, WSL2 with 16 GB shared with other sessions running builds. Wall times are noisy. Ratios are solid, absolute seconds are not.

### 10.1 Fetch (#334)

Mathlib was fetched outside the session by the owner, not by `lexlean deps fetch`. There is **no stopwatch time for a cold fetch**.

| Item | Value |
| --- | --- |
| `~/mathlib-pinned` total | 7.0 GB |
| `.lake/build` | 6.4 GB, 8275 `.olean` files under `.lake/build/lib/lean` |
| `.lake/packages` | 387 MB (batteries, aesop, Qq, proofwidgets, plausible, importGraph, LeanSearchClient, Cli) |
| Shallow source | about 147 MB |
| Download cache `~/.cache/mathlib` | 436 MB, 8639 `.ltar` files |
| Time, from file times (network and unpack together) | about 5.4 minutes (11:03:47 to 11:09:09) |

The 5.4 minutes is a bound from file times. A five-minute `child_timeout_ms` (the default) would have been borderline; a timeout is `LLS8002`. Free disk space cannot be attributed to the fetch because other sessions share the disk.

### 10.2 Import cost (#337)

Warm runs of `lean probe.lean` (imports plus `#check @decl`):

| Imports | Modules loaded | Wall | Peak memory |
| --- | --- | --- | --- |
| `Mathlib.Order.Defs.PartialOrder` | 1189 | 0.6 s | 860 MiB |
| `Mathlib.Data.Nat.Prime.Defs` | 1921 | 1.0 s | 1360 MiB |
| `Mathlib.Algebra.BigOperators.Group.Finset.Sigma` | 2124 | 1.2 s | 1505 MiB |
| `Mathlib.Analysis.Real.Sqrt` | 2917 | 1.6 s | 1943 MiB |
| `Mathlib` | whole library | 6.7 to 9.6 s warm, 40.5 s first run (cold page cache) | 5.9 GB |

The cost is the import closure of the module that defines the declaration. About 1100 modules (Lean core, Std, Batteries, Aesop) are the floor. "Minimal" means the defining module, not a re-exporting parent. Generated Lean must never write `import Mathlib`; a check for that is not built (D8).

### 10.3 Replay (#337)

Seconds / peak MiB, warm. A, B, C, D are the four rows above in the same order.

| Strategy | A | B | C | D |
| --- | --- | --- | --- | --- |
| `leanchecker Probe` (today, §22.4: project module only) | 2.6 / 758 | 2.9 / 1195 | 4.0 / 1726 | 2.2 / 1331 |
| `leanchecker <imported module>` | 2.0 / 757 | 2.4 / 1197 | 2.6 / 1729 | 2.1 / 1331 |
| `closure-replay --module Probe` | 0.3 / 568 | 0.4 / 817 | 6.1 / 1284 | 2.2 / 935 |
| `leanchecker --fresh <imported module>` (whole import closure) | 114 / 906 | 214 / 1254 | 302 / 1836 | 207 / 1453 |
| `leanchecker` on every module of the closure | over 1500 (killed) / 3615 | not run | not run | not run |

Constants replayed of constants loaded: A 356 of 125 042, B 572 of 186 514, C 10 861 of 290 597, D 4 041 of 214 417. Kernel time in the closure: A 46 ms, B 78 ms, C 5.5 s, D 1.6 s.

Closure replay is about 50 times (C) to 540 times (B) cheaper than `--fresh`, by the table above. (Note 337 says "50x to 700x"; the table gives at most 535.) Whole-module replay of the closure does not finish.

Byte reproducibility (note 337, `repro-check.sh`): `Mathlib.Order.Defs.PartialOrder`, `Mathlib.Data.Nat.Prime.Defs` and `Mathlib.Algebra.BigOperators.Group.Finset.Sigma`, built twice from empty output directories, give byte-identical `.olean`, `.olean.server`, `.olean.private`, `.ir`, `.ilean` and `.c`, and all but `.c` equal the files in the Mathlib cache. One non-obvious input: Lake passes the package name in its `--setup` file and Lean records it in the `.olean`. Limits: three modules, one toolchain build, x86_64 Linux. The `.olean` header embeds the toolchain commit hash, so bytes identify nothing across toolchains.

### 10.4 Replay cache (#338)

RC-06: three tiny modules `A <- B <- C`, one-line edit to `B`. A and C reused, only B replayed; warm verdicts equal a cold run. Two runs, machine at load average 8 to 11:

| Phase | Facts and keys | `leanchecker` replays |
| --- | --- | --- |
| cold, 3 modules | 11.2 to 13.8 s | 1.2 to 1.4 s |
| warm, nothing changed | 6.6 to 7.9 s | 0 s |
| warm, one-line edit | 8.0 to 15.9 s | 0.5 to 0.6 s |
| cold again on the edited modules | 6.8 to 12.5 s | 1.3 to 1.4 s |

Mathlib modules (one run each, exporter and `leanchecker` run by hand under a 5 GB cap, not through the cache driver):

| Module | Constants exported (own) | Facts size | Export wall / peak RSS | `leanchecker` wall / peak RSS |
| --- | --- | --- | --- | --- |
| `Mathlib.Order.Basic` | 7,911 (427) | 25 MB | 28.0 s / 2.6 GB | 2.0 s / 0.87 GB |
| `Mathlib.Algebra.Group.Basic` | 1,993 (548) | 3.3 MB | 11.1 s / 2.7 GB | 4.7 s / 0.94 GB |

Exporting the closure of a Mathlib module is 2.4 to 14 times slower than replaying it. For small modules the facts export costs more than the replay it saves. A third module, `Mathlib.Data.Nat.Defs`, exported no constants and is not counted. The cache cannot pay off until the exporter changes (one process for many modules sharing a closure walk, a compiled exporter, anchors so a module exports only its own declarations); none of these was measured. The facts of `Std.Data.HashMap.Lemmas` are 24 MB, over the default `max_child_output_bytes`; RC-06 raises it to 64 MiB for its own run.

### 10.5 Glossary coverage (#335)

All numbers from `lexlean deps glossary`, debug build. "Excluded" are requested declarations that are not candidates (internal names, recursors, matchers and other auxiliaries, instances, deprecated, and the six core constants). `requested = translated + refused + excluded`.

**Mathlib, eight modules** (`Algebra.Group.Defs`, `Algebra.Group.Basic`, `Algebra.Ring.Defs`, `Algebra.Order.Group.Defs`, `Order.Basic`, `Logic.Basic`, `Data.Nat.Basic`, `Data.Int.Basic`): 3,065 requested.

| Outcome | Count |
| --- | --- |
| translated | 1,146 (plus 236 support entries) |
| refused: `Sort u` polymorphism (`LLT4101`) | 174 |
| refused: name outside grammar, mostly `'` (`LLT4105`) | 166 |
| refused: strict-implicit binder (`LLT4102`) | 13 |
| refused: core constant at wrong level (`LLT4104`) | 7 |
| refused: depends on a `Sort u` declaration (`LLT4106`, root `LLT4101`) | 215 |
| refused: depends on an excluded instance (`LLT4106`) | 23 |
| refused: depends on an excluded internal or auxiliary declaration (`LLT4106`) | 10 |
| refused: depends on a non-grammar name (`LLT4106`) | 6 |
| excluded: internal 671, auxiliary 303, instance 229, recursor 83, deprecated 19 | 1,305 |

The columns add up: 1,146 + 614 refused + 1,305 excluded = 3,065. Of the 1,760 candidates (3,065 minus 1,305), 65% translate. No declaration is refused for having an instance argument.

**Lean core, six `Init` modules** (`Prelude`, `Core`, `Data.Nat.Basic`, `Data.List.Basic`, `Data.Int.Basic`, `Data.BitVec.Lemmas`): 6,892 requested, 2,445 translated, 1,050 refused (481 `Sort u`, 195 names, 76 signature check, 4 level, 294 dependency), 3,397 excluded, 81 support entries.

| Request | Wall | Peak RSS |
| --- | --- | --- |
| 6 `Init` modules, 6,892 declarations | 7.6 to 11 s | 1.4 to 1.7 GB |
| 4 Mathlib modules, 1,777 declarations | 26 s (cold cache) | 2.9 GB |
| 8 Mathlib modules, 3,065 declarations | 7.5 s warm | 2.9 GB |
| first Mathlib run after the `.olean` files were fetched | 2 min 50 s (cold page cache, machine busy) | 3.4 GB |

Memory is Lean importing Mathlib's environment and does not grow with the request. Scaling to all of Mathlib was not measured. The cost of loading a package of 1,383 entry files in `check` and `verify` was not measured either.

### 10.6 Wrapper coverage (#336)

Counts from a Lean script (not in the repository; recipe in note 336, scripts in the shared notes folder): public, non-internal theorems and definitions with an instance binder, counted at a type when the first sort-typed binder takes the type, every instance binder is found by instance resolution, and every binder up to the last instance binder is assigned.

| Scope | Class-polymorphic candidates | Specialisable at some of `Nat`, `Int`, `UInt32`, `BitVec 32` | At `Nat` / `Int` / `UInt32` / `BitVec 32` (all) | Same, theorems only |
| --- | --- | --- | --- | --- |
| `Init`, `Std`, `Lean` | 30,200 | 4,146 (2,804 theorems) | 3,778 / 3,959 / 3,720 / 3,428 | 2,486 / 2,713 / 2,535 / 2,308 |
| Closure of `Mathlib.Data.BitVec` | 38,427 | 8,437 (6,502 theorems) | 6,963 / 8,140 / 4,580 / 6,229 | 5,217 / 6,292 / 3,309 / 4,612 |

Most of the rest are underdetermined by the carrier alone: `Std.Commutative.comm` needs `op` stated, so a generator must choose arguments, not only a type. `UInt32` has no `AddCommMagma` in Mathlib at this pin, so that wrapper fails at the host build. A count over all of Mathlib was not run.

Against real Mathlib at the pin: `add_comm` at `Nat`, `Int`, `BitVec 32`, `add_assoc` at `BitVec 32` and `mul_comm` at `Int` build from `import Mathlib.Data.BitVec`. The strict probe line accepts the signatures written with `Nat.add`, `Int.add`, `BitVec.add` and refuses a planted wrong one.

### 10.7 Acceptance demo and measurement table

Source: note 340 and `examples/mathlib-blake3-g` (README, `lexlean.toml`, `lexlean.lock`, `glossary-report.json`, `expected/verify/`). I cross-checked the theorem names and their `\exactaxioms` against the sources and `expected/verify/audit/output.txt`, the coverage sums against each other, and the register rows against `SPEC.md`. I did not repeat the timings. Environment as in section 10: Lean 4.32.1, Mathlib `520045ab14e2`, WSL2 with 16 GB shared and loaded (load average 5 to 25), debug build of `lexlean`.

**The document.** Four modules (`Main`, `Cancellation`, `Rotation`, `Mixing`), language 1.2, `[[lean_library]] mathlib` pinned. It proves for every 32-bit word what the BLAKE3 work could only test, citing lemmas of Lean core and Mathlib through the generated glossary. `lexlean verify` prints `verified 4 modules`: Lean elaboration, `leanchecker` replay of the four modules, and the exact axiom audit all pass, offline.

| Statement (`BitVec 32`; the first two also `UInt32`) | Theorem | Imported lemmas cited | Axioms (exact, audited) |
| --- | --- | --- | --- |
| `(a + b) - b = a` | `Cancellation::add-sub-cancel`, `-u32` | `BitVec.add_sub_cancel`, `UInt32.add_sub_cancel` (core) | `propext` |
| the same through Mathlib | `add-sub-cancel-group` | `add_sub_cancel_right` at `AddGroup (BitVec 32)` (Mathlib, via a wrapper) | `Classical.choice`, `Quot.sound`, `propext` |
| `(a ^^^ b) ^^^ b = a` | `xor-cancel`, `-u32` | `xor_assoc`, `xor_self`, `xor_zero` (core), `rewrite` | `Quot.sound`, `propext` |
| `rotl (rotr a n) n = a`, n in {7, 8, 12, 16} | `Rotation::rotate-left-right` | `Blake3GHost.rotateLeft_rotateRight` (host library, below) | `Quot.sound`, `propext` |
| the inverse of BLAKE3 G recovers `a b c d` | `Mixing::mixing-inverse` | the four above, `add_right_comm` (Mathlib) | all three |

The Mathlib route costs `Classical.choice` that the core route does not. The probe prints `#print axioms` for 11 imported or host constants (`expected/verify/probe`).

**What is not imported.** `rotateLeft (rotateRight x n) n = x` is in neither Lean core nor Mathlib at this pin (searched `Init`, `Std`, Mathlib and batteries). Its proof needs `BitVec.eq_of_getLsbD_eq`, two `getLsbD_rotate*` lemmas and arithmetic on bit indices (`omega`). No prose or semantic proof form can do index arithmetic, so it is 13 lines of ordinary Lean in the host library, `Blake3GHost.rotateLeft_rotateRight`. It is a lemma of this project's host library, in the trusted base beside the imported libraries, not a lemma of Mathlib or Lean. The audit checks its axioms (`propext`, `Quot.sound`) like any other. It is a candidate to propose upstream to Lean core (D17).

G is not a named definition in the document. A prose definition would need a hand-written glossary entry, and a semantic module cannot call an external constant. The statement is relational: for all 22 words, if the eight forward equations (the BLAKE3 `gcore` as the step machine runs it) and the eight inverse equations hold, with fresh names for the recovered intermediates, the four recovered words are `a`, `b`, `c`, `d`. The equations determine every value, so this is equivalent to `inverse (G ...) = ...`. `BitVec.rotateRight v n` with `0 < n < 32` is the BLAKE3 `rotr`; the document uses the BitVec functions and does not re-prove that identification.

**Hand-written versus generated.** Generated by `lexlean deps glossary`, committed in `glossary/lean.mathlib`: 38 entries, 11 translated declarations and 27 support entries (`HAdd`, `HAdd.hAdd`, `HSub`, `HXor`, `BitVec`, `Nat`, `UInt32` and so on). Hand-written, `glossary/blake3.g`, 8 files: the label words that titles and headings require (`word`, `arithmetic`, `cancellation`, `xor`, `rotation`, `mixing`, `inverse`, `function`; §13.3). They assert nothing and none denotes an imported declaration.

**Proof forms with an external lemma** (the question D3 asks):

| Form | Used with an external lemma? | Result |
| --- | --- | --- |
| `Close the goal with \(lemma(args)\)` | yes, in every theorem | works for core, Mathlib wrapper and host lemmas alike |
| `rewrite` (`\forward`) | yes | works; a lemma with implicit arguments (`xor_self`) is used bare |
| `have` | yes | works |
| `constructor` | yes | works; `P and Q and R` is left-folded, `And (And P Q) R`, so branches nest on the left |
| `Assume` (several names) | yes | works for 16 hypotheses |
| `simplify` | yes (scratch module, not committed) | `\begin{simplify}{goal}` with three `\rule` terms proves `(a ^^^ b) ^^^ b = a`; verified |
| `calculate` | yes (scratch module, not committed) | a one-step chain justified by the core lemma verified; steps are whole-term equalities, so a lemma inside a larger term needs `rewrite` |
| `Apply`, structured `apply`, `cases`, `induction`, `Use ... as the witness` | no | not exercised: the demo has no lemma with premises or an inductive scrutinee. Nothing in §16 treats a proof term differently by origin |
| language-1.2 semantic-module forms (`reflexivity`, `linear_arithmetic`, `congruence`, `law`, `apply`, `term_cases`, `cases`, `induction`, `simplify`) | no | cannot, by construction: `apply` names a prior document theorem, `law` a registered fold law, `linear_arithmetic` and `simplify` a fixed rewrite set plus prior document definitions (§17.11, §17.12). None names an external glossary entry, and a semantic module has no way to call an external constant |

Consequences, from the note: an external lemma can only be cited from prose; a statement about `BitVec` or `UInt32` bit operations cannot be a semantic-module theorem; issue #316 (document denotations cannot name semantic-module declarations) is not the binding constraint, the missing external-constant access in semantic modules is; and #317 / PR #332 (Nat bitwise primitives) does not help, because the demo is at fixed width where the primitives already exist but the semantic module still cannot prove anything about them without external lemmas. The scratch modules for `simplify` and `calculate` are not committed, so those two rows have no committed evidence.

**Measurements.** Peak memory is `/usr/bin/time -v` maximum resident set.

| Step | Wall | Peak RSS | Note |
| --- | --- | --- | --- |
| Fetch (`lake exe cache get`), from the #334 notes | about 5.4 min (file-time bound) | not measured | 7.0 GB on disk, 436 MB download cache. Not run by this demo: the checkout was already there |
| Lake workspace for the example (`cp -al` of the pinned checkout and 8 packages) | 8.5 s | n/a | hard links, no extra disk |
| Host library build (`lake build`) | 1 to 6 s | n/a | 2 modules |
| `deps glossary`, 2 host modules, 15 declarations | 95 s cold, 26 s warm | 3.08 GB, 3.25 GB | 11 translated, 4 excluded (internal proof auxiliaries), 0 refused, 27 support entries |
| `check` | under 5 s | under a 2 GB cap | 4 modules |
| `verify`, first run | 74 s | 1.33 GB | before the merge with #339 |
| `verify`, merged branch | 41 s | 1.36 GB | probe also prints `#print axioms` of imports |
| `verify`, `.lexlean` removed | 51 s | 1.33 GB | |
| `verify`, unchanged | 53 s | 1.32 GB | no per-module reuse: `verify` reads no replay cache (§22.13) |
| `verify`, after a one-line edit (title of the leaf module) | 40 s | 1.31 GB | |

Not measured: `lock` and `build` by themselves (the note gives no time), a cold `deps fetch` run by LexLean, and any per-stage split of `verify`.

Cost structure of `verify`, from the note: each of the four modules is a separate Lean process that imports the closure of `Blake3GSpecs` (Mathlib `Algebra.Group.Basic` and `Data.BitVec`), plus the probe, the axiom-audit module and `leanchecker` per module. There is no incremental behaviour: an edit anywhere costs a full verification. The imported Mathlib declarations are not replayed by `verify` (§22.4 checks only the project's modules). The closure replay of #337 is not wired in. So a verified document says nothing about Mathlib's own proofs, which is what §5.1 and §5.3 say.

**Translation coverage.** The demo's request (the two host modules): 15 requested, 11 translated, 0 refused, 4 excluded, 27 support entries. Every cited declaration translated. The modules the cited lemmas come from, requested whole into an uncommitted scratch directory (`Init.Data.BitVec.Lemmas`, `Init.Data.UInt.Lemmas`, `Init.Data.UInt.Bitwise`, `Mathlib.Algebra.Group.Defs`, `Mathlib.Algebra.Group.Basic`), warm, 15.5 s, 2.82 GB:

| Outcome | Count |
| --- | --- |
| requested | 7,222 |
| translated | 3,689 (plus 370 support entries) |
| refused | 841: apostrophe names 210, entry check 84, `Sort u` 46, strict-implicit binder 8, depends on a `Sort u` declaration 294, on an internal one 141, on an instance 20, on an auxiliary one 24, on a non-grammar name 14 |
| excluded | 2,692: internal 2,067, instance 239, auxiliary 215, deprecated 111, recursor 60 |

The rows add up (3,689 + 841 + 2,692 = 7,222; the refusal causes sum to 841; the exclusions to 2,692). 81% of the 4,530 candidates (7,222 minus 2,692) translate. The core lemmas the demo cites are monomorphic and translate directly (`bitVecAddSubCancel`). The two Mathlib lemmas (`add_sub_cancel_right`, `add_right_comm`) translate as class-polymorphic entries that cannot be used until a specialisation wrapper exists, which is why the demo wraps them. It wraps the core ones too only to keep the committed glossary at 38 entries instead of about 4,000.

**Defects the demo found in the merged branch.** Each was fixed, given a register row, a scenario and a case, and shown red by planting the defect and green with the fix. The order of R3 was not followed for the code: the demo found them, the fix came first, then the rows and tests, then the planted defect. That is a process deviation, recorded in the note.

| # | Defect | Effect | Covered by |
| --- | --- | --- | --- |
| 1 | The probe named every binder, so an instance binder `[x3 : HAdd ...]` drew the `unusedVariables` linter and the probe failed with `LLV7006` | Any generated entry for a class operator was unverifiable | `VR-27`, `conformance_vr_27`, fixture `tests/fixtures/probe-binders` (`HXor.hXor`) |
| 2 | The probe substituted the caller's arguments into the callee's binder types by name, so in `Eq (x ^^^ x) 0` inside an entry whose binder is named `a`, the `0` was ascribed `(0 : x0)` and Lean said `type expected` | Entries with a numeral argument and a clashing binder name failed | `VR-27` (same fixture, `UInt32.xor_self`); real Lean elaborates the probe with no output |
| 3 | `fmt` in write mode checked the project in a scratch copy without the library, so it failed with `LLV7019` for any project that declares one | `fmt` unusable with a library | `ML-05`, `conformance_ml_05` (a CRLF entrypoint of a project with a present library is rewritten) |

Defects 1 and 2 matter most to the design: the §13.7 probe, which the whole approach rests on, could not verify the most common kind of generated entry until the demo ran. Row `VR-25` of the demo branch collided with the #339 row of that number and is `VR-27` after the merge.

Also changed by #340: `xtask` links the example's `.lake` into the clean copy of `verify-examples` (hard links, a copy across devices, symlinks recreated) instead of copying 7 GB; `audit-deferral` skips `.lake` (Mathlib's own sources contain `TODO`); `.gitignore` lists `examples/**/.lake/`.

**What verified and what did not.**

| Verified | Not verified or not run |
| --- | --- |
| `verify` of the demo, 4 modules, offline, against the pinned Mathlib, nested layout `<workspace>/.lake/packages/mathlib` (hard-linked) | `lexlean deps fetch` against real Mathlib: the demo hard-links an existing checkout |
| Exact axiom sets of all nine theorems and lemmas, matching `\exactaxioms` and `expected/verify/audit/output.txt` | `cargo xtask verify-examples` on this example and the full `just vv`: the note records neither |
| `conformance_vr_27`, `conformance_ml_05` red and green | Mathlib's own proofs: not replayed (§22.4) |
| The prose forms marked "yes" in the table above, with an external lemma | `Apply`, structured `apply`, `cases`, `induction`, `Use` with an external lemma; any semantic module citing an external lemma |
| Register total 476 on the merged branch | Another platform, toolchain or Mathlib commit |

Draft issues from the note, not filed: (1) semantic modules cannot cite external lemmas; (2) a prose arithmetic form (`Close the goal by linear arithmetic.`) would let the rotation inverse be a document proof; (3) `deps glossary --names` imports only `Init`; (4) generated surfaces for class operators are unreadable (`HAdd.hAdd` is `hAddHAdd(a, b)`, a wrapper is `lexLeanSpecBitVec32BitVecAddSubCancel(a, b)`), and mapping the core operator classes to infix frames would make statements read as the issue states them; (5) the example needs the library fetched, so CI needs fetch or its own part (D16). They are folded into D2, D3, D16 and D17.

## 11. Limits and refusal reasons

### 11.1 Resource limits

| Limit | Default | What it bounds here | Observed |
| --- | --- | --- | --- |
| `child_timeout_ms` | 300000 | each child: `lake exe cache get`, `lean --run` for the glossary, `leanchecker`, probes | cold `cache get` about 5.4 min (bound), glossary first run 2 min 50 s |
| `max_child_output_bytes` | 16 MiB | each child's output | facts of `Std.Data.HashMap.Lemmas` are 24 MB |
| memory (not a project limit) | none | Lean import | 0.86 to 1.9 GB targeted, 5.9 GB for `import Mathlib`, 2.9 to 3.4 GB glossary, 2.6 to 2.7 GB facts export |
| disk (not a project limit) | none | fetch | 7.0 GB library plus 436 MB download cache |

A timeout is `LLS8002` (class `security-limit`, exit 4), even for the fetch. The demo's `verify` stayed at 1.3 GB peak and the glossary run at 3.1 to 3.3 GB; neither came near the defaults (section 10.7).

### 11.2 Refusal reasons

| Code | Raised by | Reason |
| --- | --- | --- |
| `LLC0109` | config | URL not HTTPS; revision not 40 lowercase hex; `branch`, `tag`, `rev`, `ref` or any other field; missing field; name not a package identifier; tables unsorted or repeated; language other than 1.2 |
| `LLC0110` | lock, `deps fetch` | Lake manifest missing, not JSON, `packagesDir` not relative, no `packages` entry of that name, or `url`/`rev` differ; or the lock's URL, revision, manifest digest or glossary digest differ from the library now; or the lock pins a library the project does not declare |
| `LLV7019` | `lock`, `check`, `build`, `fmt`, `verify` | directory missing; not a Git checkout; `HEAD` is not the declared revision; no `.lake/build/lib/lean`; no `lake-manifest.json` |
| `LLV7020` | `deps fetch` | no `lake` on `PATH`; `lake exe cache get` exited nonzero; library not present afterwards |
| `LLT4101` | glossary | `Sort u` (a sort that is neither `Prop` nor `Type u`) |
| `LLT4102` | glossary | strict-implicit binder |
| `LLT4103` | glossary | primitive projection, string literal or other form outside LSE |
| `LLT4104` | glossary | `Eq`, `Exists` and the like at a level the core entry does not cover |
| `LLT4105` | glossary | name outside the conservative grammar (the apostrophe is the common case) |
| `LLT4106` | glossary | mentions a constant that is refused, excluded or absent; the report names the root |
| `LLT4107` | glossary | the generated entry fails the loader's entry or signature check |
| `LLT4018` | wrappers | request has another field, a repeated declaration and tag, a name outside the grammar, an argument outside the term grammar, or no wrapper |
| `LLT4017` | `verify` | wrapper entry's signature is not definitionally the wrapper's type |
| `LLT4003` | `verify` | external probe fails, including a wrapper that does not exist |
| `LLV7023` | `verify` | an imported declaration depends on `sorryAx` and no axiom policy of the document names it |
| `LLV7024` | `verify` | an imported declaration depends on `Lean.ofReduceBool` and no policy names it |
| `LLV7025` | `verify` | an imported declaration depends on `Lean.trustCompiler` or on an axiom `native_decide` generated, and no policy names it |
| `LLV7021` | replay cache | entry does not parse, wrong digest, stored under another key, other environment, not a regular file, or records nonzero exit |
| `LLV7022` | replay cache | facts repeat a declaration, name one neither declared nor anchored, or do not parse; exporter failed |

A refusal never weakens a signature to make an entry fit (§13.12). Dependent declarations are refused with the root reason, so the report shows how many refusals one cause explains: 215 of 614 Mathlib refusals in the eight-module sample depend on a `Sort u` declaration.

### 11.3 Limits of the design itself

- One toolchain, one Mathlib commit, a release line. No evidence for any other.
- Measurements are from x86_64 Linux under WSL2. Other platforms are untested.
- Glossary generation is per module or name list. A whole-Mathlib glossary was not generated.
- Report-only refusals exit 0 (D13).
- The rust signature checker rejects some dependent signatures Lean accepts (76 of about 2,500 translated `Init` declarations).

## 12. What is built and with what evidence

| Issue | Rows | Codes | Evidence |
| --- | --- | --- | --- |
| #334 pin, config, lock, fetch | `ML-01` to `ML-04` | `LLC0109`, `LLC0110`, `LLV7019`, `LLV7020` | `conformance_ml_01..04` pass; the cases use a stand-in `lake` |
| #335 glossary | `CL-25`, `GL-19`, `GL-20` | `LLT4101` to `LLT4107` | `conformance_gl_19`, `_gl_20`, `_cl_25` (real Lean run twice, byte-identical); `tests/glossary/` fixtures exercise every refusal reason |
| #336 wrappers | `GL-21`, `VR-24` | `LLT4017`, `LLT4018` | `conformance_gl_21`, `_vr_24`, written failing first; seven planted defects each caught |
| #337 speed | none (no Rust or SPEC change) | none | scripts in `tooling/mathlib-speed/`; planted-defect script |
| #338 replay cache | `RC-01` to `RC-06` | `LLV7021`, `LLV7022` | 28 unit tests, `conformance_rc_01..06` pass (RC-06 on real Lean); two planted defects caught |
| #339 axiom audit | `VR-25`, `VR-26` | `LLV7023`, `LLV7024`, `LLV7025` | `conformance_vr_25`, `_vr_26` written failing first, then pass; `conformance_vr_20..24`, 10 unit tests, bdd suite; two planted defects caught |
| #340 acceptance demo | `VR-27`, `ML-05` (fixes); example `mathlib-blake3-g` (`EX-08`, `VR-23` gates) | none new | `verified 4 modules`; `conformance_vr_27`, `conformance_ml_05`, shown red by planting, written after the fix (R3 order not followed) |

Gaps that the evidence does not cover:

- The full `just vv` of the merged integration branch is not recorded. Each sibling ran a subset (fmt, clippy, `validate-model`, `validate-spec-links`, the named conformance cases, the bdd suite for its own scenarios). #338 and #339 state plainly that the examples, golden and repro gates were not run, and #340's note records no `verify-examples` run.
- `conformance_rp_11` fails on `main` at `71defb4c6` (the README refresh removed the table it reads). Not caused by this work.
- #334's cases were not run red before the implementation existed; two defects were planted afterwards and both were caught. #337 made no SPEC change, so no register row applies yet. #340's fixes were written after the code.
- `lexlean deps fetch` itself has not been run against real Mathlib. The demo exercises the nested `.lake/packages/mathlib` layout, but with a hard-linked checkout (`setup-offline.sh`), not one Lake cloned.
- The schema files are not updated for either `lean_library` or `imports` (D7).

## 13. Axiom audit of imports

Source: note 339, merged into the integration branch: rows `VR-25` and `VR-26`, codes `LLV7023` to `LLV7025`, SPEC §5.3, §5.4, §18.8, §22.6, §22.9 and §26.3 amended. The axiom sets the demo observed for its imports are in section 10.7: `propext` for the core `add_sub_cancel` lemmas; `propext`, `Quot.sound` for the xor lemmas and the host rotation lemma; all three standard axioms through the Mathlib route. No import in the demo carries `sorryAx` or a compiler-trust axiom, so no refusal of `LLV7023` to `LLV7025` was exercised by the demo (those have their own fixture, `tests/fixtures/import-axioms`).

An *imported declaration* is the Lean constant a used external entry denotes when its Lean module is outside `Init`, `Std`, `Lean`, `Lake` and the generated runtime root `LexLeanRuntime`. For each one:

1. The probe module (§18.8) prints `#print axioms <name>` after its `example` lines, once per distinct Lean name. This is what §5.4 defines the audit to be, and it runs in the probe process that already imports the constant.
2. `verify::axiom::parse_import_audit_output` reads the output (the §22.5 grammar plus the spellings below).
3. `verify::imports::audit` refuses, before any elaboration, an import whose observed set holds `sorryAx` (`LLV7023`), `Lean.ofReduceBool` (`LLV7024`), or `Lean.trustCompiler` or a `native_decide` axiom (`LLV7025`), unless an `allow` or `exact` policy of some declaration of the document names that axiom.
4. The attestation gets an `imports` member (entry, module, name, observed set, status), present only when there is an import, so a project that imports nothing outside the toolchain keeps its attestation. The status is `replayed in the pinned environment`.

Every other axiom of an import (`Classical.choice`, `Quot.sound`, `propext`, anything else) is judged only by the generated declarations that depend on it, as before (`LLV7005`).

Findings from note 339 that change the premise of the issue:

- **`native_decide` is not `Lean.trustCompiler` at 4.32.1.** `theorem t : 2 + 2 = 4 := by native_decide` depends on `[t._native.native_decide.ax_1_1]`, an axiom generated per use. `Lean.ofReduceBool` and `Lean.trustCompiler` appear only when written out, and `Lean.ofReduceBool` rests on `Lean.trustCompiler`. Both are deprecated in 4.32.1. The generated axiom is classified with `Lean.trustCompiler` (`LLV7025`).
- **That axiom cannot be named by a policy.** Lean prints it with a hygiene mark (U+271D), and from a private declaration under `_private.<Module>.0`; policies take the closed ASCII name grammar. An import that carries it is always refused (D15).
- **A policy that accepts `Lean.ofReduceBool` must name `Lean.trustCompiler` too.** A half-named policy is `LLV7025`.
- **Granularity.** The import check asks only that some policy of the document names the axiom. The declaration that uses the import is then held to its own policy by the existing check (`LLV7005`, after elaboration). A document with one declaration `\allowaxioms{sorryAx}` and another using the `sorry` import under `\noaxioms` passes the early check and fails with `LLV7005` later. Soundness is unchanged; only the code and timing of the refusal. Per-use attribution needs a per-declaration use map that the checked project does not have.
- **Replay facts are not used for axioms.** The #338 facts could carry the axiom set, but §5.4 defines the audit as what `#print axioms` returns, and the facts export walks a whole closure (28 s and 2.6 GB for `Mathlib.Order.Basic`) to answer a question about a few constants.
- **Existing examples change.** The host modules of `peano-arithmetic` (`Doubling`, `Order`, `Divisibility`) and the Atlas entries in `language/uor` are outside the toolchain, so their probes now print axioms and they are audited imports. Only `peano-arithmetic` was verified there; `check-golden` and `verify-examples` were not run.

Evidence: `conformance_vr_25` and `_vr_26` failed first and then pass; `conformance_vr_20` to `_vr_24` (real projects), 10 unit tests, the bdd suite, `validate-model`, `validate-spec-links` (474 rows), fmt and clippy pass. Two planted defects were caught. Not run: the full `just vv`, `check-golden`, `verify-examples`, `conformance_rp_11` (known failure on `main`).

Held back, as in #334: `schemas/attestation.schema.json` and `schemas/attestation-v2.schema.json` have `additionalProperties: false`, so an attestation with `imports` does not validate against them. Editing them moves the compiler-semantics ID. The change is a patch, `339-schema-imports-member.patch`, in the shared notes folder (D7, D15).

## 14. Decisions that belong to the owner

Each has a recommendation. Evidence for each is in the sections cited.

### D1. Toolchain pin (section 4)

A Mathlib commit matches `v4.32.1`, so no choice is forced. Accept the release-line commit `520045ab14e26149ee970e2e617ca04b09bde5d6`, or move LexLean's toolchain to `v4.32.2` (`905b95818eb3`) to follow Mathlib's later point release. **Recommendation:** accept the commit; moving the toolchain is outside #333.

### D2. Command naming (section 7)

Built names: `lexlean deps fetch`, `lexlean deps glossary`, `cargo xtask emit-wrappers`, config table `[[lean_library]]`, glossary package `lean.<name>`, reserved Lean root `LexLeanSpec`. Choices:

- keep the generic `deps` group (the table is not Mathlib-specific; Batteries or any Lake library fits);
- a library-specific group such as `lexlean mathlib ...`;
- a shorter `lexlean import`.

Open within the group: whether wrappers become `lexlean deps wrappers`, whether a `deps add` writes the lakefile `require`, and whether the module list of `deps glossary` can be derived from the documents' needs (so the fetch-and-glossary steps become one). **Recommendation:** keep `deps`; add `deps wrappers` when D3 is settled; defer `deps add`.

### D3. Which proof forms accept external lemmas in language 1.2

Facts, from the code, SPEC and the demo (section 10.7):

- In prose proofs (§16), every form that takes a `TERM` can contain an external `proof-constant` entry. The demo, a language-1.2 project with `[[lean_library]]`, does this with `Close the goal with`, `rewrite`, `have`, `constructor`, `Assume`, and in scratch modules `simplify` and `calculate`. So the question the issue asks, "which prose proof forms should accept external lemmas in language 1.2", has a first answer: all of those already do, with no change. Not exercised: `Apply`, structured `apply`, `cases`, `induction`, `Use ... as the witness` (the demo has no lemma with premises or an inductive scrutinee). Nothing in §16 treats a term differently by origin.
- A language-1.2 `semanticmodule` (§17.21), another kind of module in the same project, has closed proof forms (`SemanticProof` in `crates/lexlean/src/ir/semantic.rs`): `reflexivity`, `decide`, `linear_arithmetic`, `simplify`, `constructor`, `cases`, `term_cases`, `induction`, `congruence`, `boolean_reflection`, `law`, `apply`. None takes a lexicon entry: `apply` names a prior document theorem (`MemberRef`), `law` a registered fold law, `simplify` document definitions. A semantic module has no way to call an external constant at all.
- Consequence: an external lemma can be cited from prose and not from a semantic module. A fixed-width statement that needs Lean or Mathlib lemmas cannot be a semantic-module theorem.
- Limit of prose: bit-level facts reduce to arithmetic on indices, and no prose form does that. The demo needed 13 lines of Lean in a host library for the rotation inverse (draft issue 2 of note 340).

Options for semantic modules:

| Option | Effect | Cost |
| --- | --- | --- |
| A. `apply` accepts an external entry | Smallest change. Lowers to `exact @L args` like `law`, validated against the entry's signature by the same probe (§13.7); Lean checks the statement | New schema member; SPEC §17.12 text; row, scenario, `LLT` code, negative test; moves the language-1.2 identity |
| B. A new `external` proof form (note 340: `external{entry, arguments}`) | Same, with its own name | Same cost, more surface |
| C. `simplify` and rewrite-like forms also accept entries | Needed for equational Mathlib use | Larger: simp sets of external lemmas are the unrestricted search risk (§16.12 forbids unrestricted simplification) |
| D. None: Mathlib only from prose modules | No semantic-module change | Fixed-width semantic definitions and Mathlib stay apart |

For prose, the second option is a prose arithmetic form, `Close the goal by linear arithmetic.`, lowering to `omega` or to the `linear_arithmetic` of §17.12.

**Recommendation:** A, with explicit arguments only and no search. C only on demonstrated need. The prose arithmetic form is a separate, smaller decision; take it if more bit-level documents are expected.

### D4. Language scope of `[[lean_library]]` (section 5.1)

It is language 1.2 only, to keep languages 1.0 and 1.1 identities. Before the demo this looked like a conflict, because the only proofs that could cite external lemmas were thought to be language-1.0 prose. The demo shows prose modules work in a language-1.2 project that declares a library, so a 1.2 project has both the config and the proof forms (D3). The remaining cost: a language-1.0 or 1.1 project (the §13.6 example, `examples/peano-arithmetic`) cannot declare a library. It can still depend on Mathlib through Lake alone, since §10.4 already hashes `lake-manifest.json` for every language, but it has no presence check, no `deps fetch` support and no glossary row. Whether a prose-only language-1.0 project can reach 1.2 was not tested: `lexlean migrate --to 1.2` accepts language 1.1 only (§23.4). **Recommendation:** keep 1.2 only.

### D5. Closure replay (section 8, item 5; section 10.3)

Adopt `closure-replay` as an additional `verify` stage for the imported part, and keep stock `leanchecker` per generated module (2 to 4 s per module, mostly importing; its record and executable hash stay as §22.4 and §22.2). It is the only measured option that checks imports (50 to 540 times cheaper than `--fresh`) and its closure is rooted at the constants the compiled project module mentions, so instances and lemmas found by elaboration are included. Required: SPEC §5.1 and §22.4 text, a register row and scenario and failing test first (the planted-defect module is that test), a diagnostic and negative test, and a decision whether it later replaces `leanchecker` per module. It is a new Lean program the attestation would hash. **Recommendation:** adopt as an additional stage; decide replacement later.

### D6. Replay cache in `verify` (sections 9.2, 10.4)

Open points: whether `verify` may ever consult a store (§10.5 and §9.4 stay true while it does not); fail closed on a rejected entry (built) or evict and replay; whether an attestation must mark reused units; whether signing is wanted; the `blake3` dependency and `BSD-2-Clause` licence; hashing the executable once more (the existing preflight hashes only SHA-256); alpha-equivalent keys ignore binder names (a more conservative key would include them); facts size for Mathlib (anchors or a per-module store). **Recommendation:** do not wire it into `verify`. On the two Mathlib modules the export costs 2.4 to 14 times what it saves. Keep the library. Revisit after the exporter changes and a Mathlib-sized measurement exists.

### D7. Identity-moving schema change (section 6)

Two patches are held back for the same reason: `334-schema-identity-move.patch` (`lean_library` in the project and lock schemas) and `339-schema-imports-member.patch` (`imports` in the two attestation schemas). Apply them now, or batch them with the other Mathlib changes so the language-1.2 regeneration (528 files, the full gate with Lean) happens once. AGENTS.md wants such a change to land alone. **Recommendation:** one change, one regeneration, one full gate, after D3 is settled, because D3 = A also moves the identity. Note that the demo's committed lock and `expected/` carry the current language-1.2 ID too, so they are among the files that regeneration touches.

### D8. Reject `import Mathlib` (section 10.2)

Reject an umbrella import in generated Lean with a diagnostic and negative test, or only lint. It costs 5.9 GB and 7 to 40 s against 0.9 to 1.9 GB and 0.6 to 1.6 s. The generated probe already imports only the entry's defining module. **Recommendation:** diagnostic.

### D9. Lock more than the commit (section 6)

Options: `tree_sha256` over the Mathlib source (closes the source half); a digest of the fetched `.olean` files (closes the other half, but only if byte stability holds beyond the three modules measured); or neither, with replay (D5) as the check for what a verified document uses. **Recommendation:** replay (D5), plus the source digest if cheap; defer the `.olean` digest.

### D10. Lake `require` (section 5.2)

By hand now, or `lexlean deps add`. **Recommendation:** by hand for the first release; document it.

### D11. Fetch timeout (sections 5.4, 10.1)

A cold `cache get` took about 5.4 minutes. Options: document that a project declaring Mathlib raises `child_timeout_ms`; give `deps fetch` its own limit. **Recommendation:** document it (for example 900000). The demo did not give a measured cold fetch (it hard-links an existing checkout), so deciding on a separate limit for `deps fetch` still waits for one.

### D12. Glossary generator (section 10.5)

From note 335:

| # | Decision | Effect |
| --- | --- | --- |
| a | Apostrophe in names (`LLT4105`) | 166 of 3,065 requested in the Mathlib sample, 210 of 7,222 in the demo's modules; widening the §13.6 grammar and the LSE and probe identifier quoting would recover most |
| b | Strict-implicit binders (`LLT4102`) | 13 directly in the sample, add a mode to LSE or accept the loss |
| c | `Sort u` polymorphism (`LLT4101`) | Largest cause: 174 directly, 215 more by dependency. Add a `Sort u` form to LSE, hand-map the few core constants, or accept |
| d | Rust signature checker rejects some dependent signatures Lean accepts | 76 of about 2,500 translated `Init` declarations; separate decision |
| e | Existing std entries and a generated `nat` denote the same Lean constant | Map to existing entries or accept cross-package ambiguity |
| f | A probe pass inside the generator (`backend::lean::probe_module` once over all entries, refusing mismatches with `LLT4003`) | Makes "never weakened" a property of the generator; about a day |
| g | Eliminator names default to `T.casesOn` and `T.rec`; the backend's emission for indexed families is untested | Check before relying on generated descriptors |
| h | `--names` loads only `Init` (section 7.3) | Add a flag naming the modules to load, or load the defining module of each name. Recommendation: do it; without it a single Mathlib lemma costs a whole-module request or a host wrapper |
| i | Generated surfaces for class operators are unreadable: `hAddHAdd(a, b)`, `lexLeanSpecBitVec32BitVecAddSubCancel(a, b)` | Map the core operator classes to infix frames (`+`, `-`, `^^^`) in the generator, so statements read as written |

### D13. Refusals: report or fail

Refusals are reported with exit 0; `--strict` fails. **Recommendation:** keep the default (a large request always has refusals; the report is the output).

### D14. Wrappers (section 7.4)

| # | Decision |
| --- | --- |
| a | The `peano-arithmetic` example is not converted. Its host library is empty; converting it changes its probe, closure and lock files, and the host library must be built (`lake build <HostLib>`) before `verify-examples`. Nothing builds a host library today. The Mathlib demo takes the documented-prerequisite route (`setup-offline.sh` runs `lake build Blake3GHost`). Document the prerequisite, or add a build stage |
| b | Strict probe for all external entries, not only wrappers. Changes every committed probe and `expected/verify`; needs a regeneration |
| c | The reserved root `LexLeanSpec` and its name scheme. Any project declaring its own `LexLeanSpec.*` outside the emitter gets the strict line too |
| d | A `lexlean` subcommand for wrappers (changes §23.4 and its rows) |
| e | Who writes the LSE signature of a wrapper. #335 would need a Lean-side exporter from the built wrapper's type |
| f | `def` wrappers are accepted by the grammar; only theorems were built and measured. A computable `def` may need `noncomputable` |
| g | Axioms: a wrapper adds none, but the declaration it applies brings its own; the host library is not audited |

### D15. Axioms and wording

From note 339 (section 13):

| # | Decision | Recommendation |
| --- | --- | --- |
| a | Status word. "Replayed in the pinned environment" is literal only if closure replay (D5) is adopted. Otherwise "loaded from the pinned environment" is more accurate | Adopt D5 and keep "replayed"; if D5 is declined, change the word |
| b | The `native_decide` axiom cannot be named by a policy, so an import carrying it is always refused. Leave it, or let a policy name it with a prefix pattern | Leave it refused |
| c | Identity move for the `imports` attestation member: take the patch, or put the rows in a staged file (`probe/imports.json`) that the attestation does not carry | Take the patch, in the single regeneration of D7 |
| d | Per-use attribution of an import's axioms to the declaration that uses it (built: any policy of the document naming the axiom passes the early check) | Accept the current behaviour; soundness is unchanged |
| e | A cross-check of replay facts against `#print axioms`, if the cache is ever to report axioms for reused units | Only with D6 |

### D16. Examples that need a library, and the gate (section 10.7)

EX-08 discovers every `examples/*` directory and requires the full example gate. The Mathlib demo fails with `LLV7019` unless `.lake/packages/mathlib` is the pinned checkout with built artifacts. Where Mathlib lives for an example is therefore a decision: the demo hard-links an existing checkout into the example's `.lake/packages` (a symlink out of the project is refused by path confinement, and hard links need one filesystem). CI needs `lexlean deps fetch` (7.0 GB, about 5 minutes) before the examples leg, or this example needs its own CI part. `xtask` now links the example's `.lake` into the clean copy of `verify-examples` instead of copying 7 GB, and `audit-deferral` skips `.lake`. **Recommendation:** a separate CI part for this example that runs `deps fetch` first, so the ordinary examples leg stays offline and small.

### D17. The demo's own choices (section 10.7)

| # | Decision | Recommendation |
| --- | --- | --- |
| a | Propose `BitVec.rotateLeft_rotateRight` upstream to Lean core. It is the one hand-written Lean lemma in the demo | Yes |
| b | Whether the relational encoding of G is acceptable until a semantic module can name external constants | Yes, with D3 = A as the way out |
| c | Whether `verify` should also replay the imported declarations the document depends on. As it stands a verified document says nothing about Mathlib's own proofs | Same as D5 |

## 15. Inconsistencies and gaps found

Found while writing; none fixed (the brief limits this change to the document). The first two are in normative text.

1. **Instance binders in entries.** SPEC §13.13 and note 336 say a class-polymorphic declaration cannot be denoted directly because "its signature has instance binders that LSE entries do not state". But §13.8's grammar has the binder mode `instance`, and §13.12 and register row `GL-19` keep instance binders in generated signatures (note 335: "No declaration is refused for having an instance argument"). So the stated reason for wrappers does not hold as written: LSE can express an instance binder. The Rust elaborator also skips implicit and instance binders into metas (`crates/lexlean/src/elaborate/expressions.rs`, around line 1263), so the reason is not obviously that either. The fixture does show that wrappers are useful (a monomorphic constant at `Nat`, with a strict probe). I did not find the real motivation in the code. The author of §13.13 should confirm and restate the premise.
2. **Status wording.** #333 says the status wording is "replayed in the pinned environment" (§5.3). The text was missing until #339 added it to §5.3. The word is only literally true if closure replay is adopted, because §22.4 replays the generated modules and `leanchecker M` does not re-check M's imports (note 339 says so itself; D15a).
3. **Glossary package name.** The lock reads the glossary digest from the package named `lean.<name>`, but `deps glossary --package` accepts any ID. A glossary generated under another ID locks the empty-tree digest with no diagnostic.
4. **Note 337 ratio.** "50x to 700x" cheaper than `--fresh`; the table in the same note gives 50 to 535 times (114/0.3 = 380, 214/0.4 = 535, 302/6.1 = 49.5, 207/2.2 = 94).
5. **Note 335 file count.** "1,383 entry files in all" for 1,146 translated plus 236 support entries, which is 1,382. The difference of one is probably `lexicon.toml`; not checked.
6. **Workspace layout.** The #334 to #338 measurements ran with the Mathlib checkout as the workspace root, while the library path LexLean reads is `<workspace>/.lake/packages/mathlib/...`. The demo (#340) exercises the nested layout, with hard links rather than a Lake clone. Closed except for the clone case.
7. **Classes of the cache codes.** `LLV7021` and `LLV7022` are class `language`, exit 1, like `LLV7003`; `LLV7019` and `LLV7020` are `environment`, exit 3. A rejected cache entry is arguably an environment problem. Owner's call; no test depends on it beyond ERRORS.md.
8. **Register count.** `SPEC.md` says 476 total required IDs and `model/ids.toml` has 476, so the collisions noted in 335 and 336 (460, 459) and the row collision between #339 and #340 (both `VR-25`; the demo's is now `VR-27`) were resolved at merge. The example's README still lists `VR-25` and `VR-26` and `ML-01` to `ML-04` as the rows whose gates run on it and omits `VR-27` and `ML-05`.
9. **`fmt` needs the library.** `fmt` fails with `LLV7019` for an absent library although it does not use Mathlib. Follows from every command computing the lock. Recorded, not asked. (The demo's defect 3 was a different bug: `fmt` failed even with the library present.)
10. **Probe bugs found only by a real document.** Defects 1 and 2 of section 10.7 (instance-binder linter, numeral capture) made the §13.7 probe fail for the most common kind of generated entry, a class operator, and none of the tests of #334 to #339 caught them. The note does not say why; the wrapper fixture of #336 used `Nat` and `Int` commutativity and did not hit them.
11. **Prose and semantic modules in one project.** Before the demo I read the issue's question (which prose proof forms should accept external lemmas in language 1.2) as possibly meaning that 1.2 had no prose. In a 1.2 project, prose modules and semantic modules coexist; §17.11's "exclusive with prose blocks" applies inside one module. The real gap is semantic modules (D3).
12. **README wording.** The example's README says the imported proofs are "replayed in the pinned environment". By §22.4 only the project's four modules are replayed (item 2).

## 16. Review against SPEC.md

Each row says what the design does with that section, and whether the review found a conflict.

| Section | Design | Finding |
| --- | --- | --- |
| §4.2 | Imports are closed external glossary entries. Generated Lean for wrappers is written by a closed emitter into the project's host library, not into documents. No claim of proof by LexLean. | No conflict. The host library is raw Lean the project owns; it is in the trusted computing base |
| §5.1 | Item 5 (imported workspace and locked dependencies) already covers Mathlib. Closure replay (D5) would add an executable to hash | No conflict. D5 needs text |
| §5.3 | Imports are described as "replayed in the pinned environment", not "proved", "verified", "kernel-checked" | Wording added by #339. The word "replayed" is literal only with closure replay (section 15, item 2; D15a) |
| §5.4 | Audit stays an axiom-dependency audit. Each import's set is read from `#print axioms` in the probe | No conflict with #339's text. Attestation schema not updated (D15c) |
| §22.6 | An import's axioms are not exempt: policies of generated declarations still apply | Consistent. Early refusal of `sorryAx` and compiler-trust axioms is #339's addition |
| §10.4 | `[[lean_library]]`, presence rule, lock rows, `LLC0110`, `LLV7019` | Implemented and in SPEC. Schema JSON not updated (D7) |
| §11.4 | A library is never acquired by `lock`, even with `--allow-network` | Implemented, `ML-04` |
| §13.6 | Generated entries denote `kind = "lean"` with the defining module and the conservative name | No conflict. The apostrophe is outside the grammar (D12a) |
| §13.7 | Each used entry probed by Lean. Wrappers get the stronger probe | No conflict in the text. The ordinary probe is weaker than the wrapper probe (D14b), and the demo found two implementation defects in it (section 10.7, fixed by `VR-27`) |
| §13.8 | LSE signatures generated from kernel types, `Sort u` and strict-implicit refused | Conflict with the premise of §13.13 (section 15, item 1) |
| §16 | Prose forms accept external `proof-constant` entries through `TERM`, also in a language-1.2 project (demo); language-1.2 semantic-module forms do not | D3 |
| §16.11 | Generated eliminator descriptors use `T.casesOn`/`T.rec` or custom eliminators | Backend emission for indexed families untested (D12g) |
| §16.12 | No unrestricted simplification: rules out wide `simplify` over external lemmas | Reason for option A over C in D3 |
| §22.4 | `leanchecker` replays generated modules only | Basis of the trust statement item 3 |
| §22.13 | Replay cache is opt-in and unused by `verify` | Consistent with §9.4, §10.5 |

This review is a reading of the sections against the design and the code, plus the mechanical checks in section 17. It has not been done by anyone but the author of this document.

## 17. Checks run on this document

- Every `LL...` code named in this document has an entry in `ERRORS.md` of the integration branch (script: a loop over the codes found by `grep -oE 'LL[A-Z][0-9]{4}'`, each looked up as a `## ` heading), and every `AA-NN` register ID named has a row in `model/ids.toml` (second loop, same method). The count of register IDs is 476 in `SPEC.md` and in `model/ids.toml`.
- Numbers were taken from the seven notes files and cross-checked where they add up (sections 10.5 and 10.7 sums, section 10.3 ratios), and the demo's axiom sets and theorem names against its sources and `expected/verify/audit/output.txt`. Measurements were not repeated.
- SPEC section numbers were checked against `SPEC.md` of the integration branch: §13.12 glossary, §13.13 wrappers, §22.13 replay cache, §22.9, §26.3.
