# Mathlib import: notes for #334 (pin, config, lock, fetch)

Working notes for the design document (#341). Not normative; SPEC.md is.

## The pin

Mathlib commit `520045ab14e26149ee970e2e617ca04b09bde5d6`, `lean-toolchain` exactly
`leanprover/lean4:v4.32.1` (checked by reading the file at that commit).

- It is the commit tagged `v4.32.1` in `leanprover-community/mathlib4`, committed 2026-07-23
  with the message "chore: bump toolchain to v4.32.1".
- It is not on `master`. Its parent is `81a5d257c8e4` (the `v4.32.0` bump, 2026-07-13). `master`
  went from `v4.32.0` straight to `v4.33.0-rc1` (`79d0395a1825`, 2026-07-16), so a search of
  `master`'s history for `lean-toolchain` finds no `v4.32.1`. The tags `v4.32.1` and `v4.32.2`
  (`905b95818eb3`, 2026-07-28) sit on a release line off `v4.32.0`.
- Consequence: the pin is a point release of Mathlib's `v4.32` line, not a `master` snapshot.
  Mathlib on that line is frozen at the July 2026 state plus whatever Mathlib backported.
  Neighbours: `v4.32.0` is `81a5d257c8e410db227a6665ed08f64fea08e997`; `v4.32.2` is
  `905b95818eb32af7874a58b427f50c1711a5e96c`, which needs LexLean's pin to move to `v4.32.2`.
- The pin is written as the commit, never the tag. The tag is a mutable reference.

## Configuration

`[[lean_library]]` in `lexlean.toml`, language 1.2 only, keys `name`, `url`, `revision`, sorted
and unique by `name`, omitted when absent. Refused with `LLC0109`: non-HTTPS URL, revision that
is not 40 lowercase hex, `branch`/`tag`/`rev`/`ref` (named as mutable references), any other
field, a missing field, a name that is not a package identifier, unsorted or repeated tables,
another language.

Why language 1.2 only: it follows `source_package` and `artifact_source`, so languages 1.0 and
1.1 keep their identities. The cost is that a language-1.0 document (for example
`examples/peano-arithmetic`, the §13.6 denotation example) cannot declare a library. Owner
decision.

Why `name` is a package identifier (`[a-z][a-z0-9-]{0,62}`): same rule as `source_package`.
It must equal the Lake package name, so a Lake package with an uppercase letter or an underscore
(for example `Qq`, `importGraph`) cannot be declared. `mathlib` and `batteries` are fine.

## Declared once, agreed with Lake

Lake needs its own `require` in the workspace's lakefile and an entry in
`lake-manifest.json`. To keep "declare once" honest, `lexlean.toml` is the pin and the Lake
workspace must agree: the manifest's `packages` entry of the same `name` must have the same
`url` and `rev`, else `LLC0110` names both. LexLean does not write the lakefile or the manifest.
Someone still has to add the `require` and run `lake update` once; scaffolding that is a
follow-up (`lexlean deps add`?), not part of this change.

## Lock rows

```toml
[[lean_library]]
name = "mathlib"
url = "https://github.com/leanprover-community/mathlib4"
revision = "520045ab14e26149ee970e2e617ca04b09bde5d6"
lake_manifest_sha256 = "..."
glossary_sha256 = "..."
```

- `lake_manifest_sha256`: SHA-256 of `<lean_workspace>/<packagesDir>/<name>/lake-manifest.json`,
  Mathlib's own manifest, which pins batteries, aesop, Qq, proofwidgets and the rest.
- `glossary_sha256`: tree digest (§11.5) of the project's lexicon package `lean.<name>`, or the
  digest of the empty tree (`SHA-256("lexlean-tree-v1\0")`) when the project declares none.
  This couples to #335: the generated glossary has to be written as a path lexicon package named
  `lean.mathlib`, or the row says "empty". If #335 chooses another place, change `resolve`.
- Not pinned: the bytes of the built `.olean` files and of the Mathlib source tree. Presence is
  checked from `.git/HEAD` (the declared commit), the library's manifest, and the existence of
  `.lake/build/lib/lean`. A tampered checkout with a matching `HEAD` is not detected. Adding a
  `tree_sha256` over the Mathlib source (about 100 MB, a second or so to hash) would close the
  source half; the `.olean` half needs Lean's own check (`leanchecker` replay or Lake's trace
  files). Owner decision. §5.1 already puts the imported workspace in the trusted computing base.

## Diagnostics

| Code | Class | Meaning |
| --- | --- | --- |
| `LLC0109` | cli-configuration, exit 2 | `[[lean_library]]` declaration invalid |
| `LLC0110` | cli-configuration, exit 2 | library is not the library the lock pins, or Lake does not require it as declared |
| `LLV7019` | environment, exit 3 | declared library not present (absent, wrong `HEAD`, no built artifacts, no manifest) |
| `LLV7020` | environment, exit 3 | `deps fetch` failed (no `lake`, `lake` failed, library absent afterwards) |

Register rows `ML-01..ML-04` (suite `mathlib`), scenarios in `features/suites/mathlib.feature`,
cases in `crates/conformance/src/cases/mathlib.rs`.

## Offline behaviour

`lock`, `check`, `build`, `fmt`, `verify` resolve the library through `compute_lock`, which every
one of them calls, and never start a process for it. Absent library: `LLV7019`, which names the
library and says `lexlean deps fetch`. Even `lock --allow-network` does not fetch a library.

## `lexlean deps fetch`

Runs `lake exe cache get` in the Lake workspace through the checked child runner (cleared
environment, `PATH` kept, `HOME` kept, `child_timeout_ms` and `max_child_output_bytes` apply),
then re-runs the presence check. The Lake workspace must already require the library; Lake
clones it at the manifest's revision as part of `lake exe`, and Mathlib's `cache` executable
downloads the `.olean` files.

Caveat: the default `child_timeout_ms` is 300000 (five minutes). A cold `cache get` can exceed it
on a slow link; the failure is `LLS8002`, not `LLV7020`, and the fix is to raise the limit in
`lexlean.toml`. Measured numbers are under "Measured fetch".

## Identity cost

Adding `lean_library` to `schemas/project-v2.schema.json` and `schemas/lock-v2.schema.json`
moves the language-1.2 compiler-semantics ID (those files are in the 1.2 digest,
`is_v1_2_file`). That makes every committed language-1.2 lock, golden file and expected output
stale, and AGENTS.md wants such a change to land alone with one regeneration and one full gate.
See the status section for what was and was not regenerated on this branch.

## Status of this branch

Done and checked:

- Register rows `ML-01..ML-04`, SPEC §31 rows and prose (§10.1, §10.4, §11.1, §11.2, §11.4,
  §23.4, the diagnostic tables), `model/errors.toml` (4 codes), generated `ERRORS.md` and
  `CONFORMANCE.md`, `features/suites/mathlib.feature`, the cases in
  `crates/conformance/src/cases/mathlib.rs` and `conformance_ml_01..04`.
- Config (`config.rs`), lock rows (`lock.rs`), `lean_library.rs`, `lexlean deps fetch`
  (`cli.rs`, `Engine::deps_fetch`, crate-private so the seven public entry points of §24.1 stay seven).
- Passing: `conformance_ml_01..04`, `conformance_cl_13`, `conformance_rp_07`, 58 of the 61
  `conformance_cf/cl/pk_01/pk_02/rp/er` cases I ran, `cargo xtask validate-model`,
  `validate-spec-links`, `cargo xtask check-test-inventory` (801 tests, none lost), `cargo test -p
  lexlean --all-features` (219 unit tests and the crate's integration tests, all ok), `cargo fmt --check`, `cargo clippy --workspace --all-targets
  --all-features -D warnings`.
- `conformance_rp_11` fails and also fails on `main` at `71defb4c6`: the README refresh (#331)
  removed the `| Capability | IDs | Level |` table that RP-11 reads. Not caused by this branch.

Not done:

- The schema change (`schemas/project-v2.schema.json`, `schemas/lock-v2.schema.json` gain
  `lean_library`). Applying it moves the language-1.2 compiler-semantics ID: 528 tracked files
  carry the current ID (locks, golden files, expected outputs), and regenerating them needs the
  full gate with Lean. It is held back as a patch, `334-schema-identity-move.patch` in the shared
  notes folder, so this branch does not change a language identity without the owner's say-so.
  Until it is applied, the JSON schemas do not describe `lean_library`.
- The full `just vv`, and any regeneration. Only the subset above was run.
- A real `lake exe cache get` run by this branch's code. The tool call was refused by the permission
  classifier (it runs Mathlib's own `cache` program). Ilya ran it outside the session; see
  "Measured fetch" below. The conformance cases use a stand-in `lake`, so `lexlean deps fetch`
  itself has not been run against real Mathlib.
- The cases were not run red before the implementation existed (see VERIFICATION.md); two defects
  were planted afterwards and both were caught.

## Measured fetch

Pinned Mathlib `520045ab14e2`, fetched outside this session by Ilya (not by `lexlean deps fetch`):

- Disk: `~/mathlib-pinned` 7.0 GB in total: `.lake/build` 6.4 GB (8275 `.olean` files under
  `.lake/build/lib/lean`), `.lake/packages` 387 MB (batteries, aesop, Qq, proofwidgets, plausible,
  importGraph, LeanSearchClient, Cli), the shallow source about 147 MB. The download cache
  `~/.cache/mathlib` is a further 436 MB (8639 `.ltar` files). Free space on `/` is 646 GB now;
  other sessions share the disk, so a before/after difference is not attributable.
- Time, approximate: file times in `.lake` and `~/.cache/mathlib` run from 11:03:47 to 11:09:09
  on 2026-10-29, so about 5.4 minutes cold, network and unpack together. No shell history records
  the command, so treat it as a bound from file times, not a stopwatch. A five-minute
  `child_timeout_ms` (the default) would have been borderline.
- Layout check (by file, `lake` not run): `.git/HEAD` is the pinned commit, `lake-manifest.json`
  is present, and `.lake/build/lib/lean/Mathlib.olean` exists, which are the three things
  `lean_library::present` requires.

## Open questions for the owner

1. Apply the identity-moving schema patch now, or batch it with the other Mathlib changes so the
   language-1.2 regeneration happens once?
2. Language 1.2 only, or all languages (§13.6 denotations exist in 1.0)?
3. Pin the Mathlib source tree digest in the lock as well (closes the "matching `HEAD`, tampered
   files" gap for the source, not for `.olean` files)?
4. Who writes the Lake `require` and `lake-manifest.json` entry: the user by hand (now), or a
   `lexlean deps add`?
5. The `v4.32.1` Mathlib commit is a release-line commit, not a `master` commit. Moving the pin to
   `v4.32.2` (`905b95818eb3`) would need a LexLean toolchain change; staying on `v4.32.1` gives
   a Mathlib frozen at July 2026.
6. `child_timeout_ms` defaults to 300000; a cold `cache get` may need more.
