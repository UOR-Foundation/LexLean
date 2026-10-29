# Mathlib import: speed notes (#337)

Input for the design document (#341). Measurements, a replay recommendation, open owner
decisions. Everything is reproducible with `tooling/mathlib-speed/`.

Setup: Lean 4.32.1 (the pin), Mathlib `520045ab14e2` (`~/mathlib-pinned`, prebuilt oleans from
the cache, never rebuilt), WSL2 with 16 GB shared with other sessions running builds, so wall
times are noisy. Peak memory is the `/usr/bin/time -v` maximum resident set. Each probe is a
one-theorem module importing one Mathlib module (`tooling/mathlib-speed/probes/`).

## 1. No rebuild

Nothing here builds Mathlib. Oleans come from the explicit fetch step; `LEAN_PATH` points at
them (`mathlib-env.sh`). Only the project module is compiled.

## 2. Targeted imports vs `import Mathlib`

Warm runs of `lean probe.lean` (probe = imports + `#check @decl`):

| Imports | Modules loaded | Wall | Peak memory |
| --- | --- | --- | --- |
| `Mathlib.Order.Defs.PartialOrder` (A) | 1189 | 0.6 s | 860 MiB |
| `Mathlib.Data.Nat.Prime.Defs` (B) | 1921 | 1.0 s | 1360 MiB |
| `Mathlib.Algebra.BigOperators.Group.Finset.Sigma` (D) | 2124 | 1.2 s | 1505 MiB |
| `Mathlib.Analysis.Real.Sqrt` (C) | 2917 | 1.6 s | 1943 MiB |
| `Mathlib` | whole library | 6.7 to 9.6 s warm, 40.5 s first run | 5.9 GB |

The cost is the transitive import closure of the module that defines the declaration, so
"minimal" means: import the module that *defines* each referenced declaration, not a
re-exporting parent. Module counts include Lean core, Std, Batteries and Aesop (about 1100
modules are the floor). The module is known exactly: the lexicon entry already names it
(`module` in a `lean` denotation, SPEC 13.6), and `Environment.getModuleIdxFor?` gives it to the
generator (#335). Dropping an import implied by another changes no cost; only choosing the
defining module over an umbrella does. Generated Lean must never write `import Mathlib`; a check
for that would be a diagnostic with a negative test, not yet added (section 6).

## 3. Replay

Findings about the pinned `leanchecker` (source: `src/lean/LeanChecker.lean` in the toolchain):

- It takes module names only, as prefixes. It has no declaration filter.
- `leanchecker M` replays M's own constants into the environment of its imports. The imports
  are trusted, not checked.
- `leanchecker --fresh M` (one module only) replays every constant M loads, imports included,
  into an empty environment: the whole import closure.
- Naming every module of the closure on one command line is whole-module replay of the closure.

A declaration subset is still possible, because `Lean.Environment.replay` is already
dependency-driven: given a map of constants it sends each to the kernel after the constants it
mentions. `tooling/mathlib-speed/closure-replay` (about 120 lines of Lean, built with the pinned
toolchain) reads the most private part of each `.olean` (the part that carries proof bodies, as
`leanchecker` does), starts from the constants of the project module, collects everything they
transitively mention (plus the members, constructors and recursors of any inductive family,
which `replay` reads), and replays that closure into an empty environment. A mention that is not
found in the loaded modules is an error, so an incomplete closure cannot pass. Same kernel as
`leanchecker`; not an independent checker.

Seconds / peak MiB, warm:

| Strategy | A | B | C | D |
| --- | --- | --- | --- | --- |
| `leanchecker Probe` (today, SPEC 22.4: project module only) | 2.6 / 758 | 2.9 / 1195 | 4.0 / 1726 | 2.2 / 1331 |
| `leanchecker <imported module>` (one Mathlib module, own constants) | 2.0 / 757 | 2.4 / 1197 | 2.6 / 1729 | 2.1 / 1331 |
| `closure-replay --module Probe` (project constants + closure) | 0.3 / 568 | 0.4 / 817 | 6.1 / 1284 | 2.2 / 935 |
| `leanchecker --fresh <imported module>` (whole import closure) | 114 / 906 | 214 / 1254 | 302 / 1836 | 207 / 1453 |
| `leanchecker` on every module of the closure | over 1500 (killed) / 3615 | not run | not run | not run |
| `closure-replay --all` (same tool, every loaded constant) | 105 / 698 | not run | not run | not run |

Closure size, constants replayed of constants loaded: A 356 of 125 042, B 572 of 186 514,
C 10 861 of 290 597, D 4 041 of 214 417. Kernel time in the closure: A 46 ms, B 78 ms, C 5.5 s,
D 1.6 s. For C the closure is dominated by real analysis proofs; the cost is the kernel
re-checking them, which is the point.

### What each option covers (planted defect)

`planted-defect.sh`: module `Evil` adds a false theorem with `debug.skipKernelTC`; `UsesEvil`
imports and uses it. `leanchecker UsesEvil` passes (exit 0), `leanchecker Evil` fails,
`closure-replay` fails for both. So replaying only the project's modules, which is what
SPEC 22.4 does, never checks any imported Mathlib declaration; those are trusted as part of the
workspace (SPEC 5.1 item 5). Closure replay turns that trust into a check for exactly the
declarations the project depends on.

### Recommendation

Use closure replay for the imported part, and keep stock `leanchecker` for the generated modules.

- Cheapest correct: closure replay is 50x to 700x cheaper than `--fresh`, the only stock option
  that checks imports, and checks the constants that matter. Whole-module replay of the closure
  does not finish (each module task rebuilds its own import environment, so the work grows
  roughly with the square of the module count).
- Roots are the constants the compiled project module mentions, not just the glossary names, so
  instances and lemmas found by elaboration are included.
- Keep `leanchecker M` per generated module as today. It costs 2 to 4 s per module, mostly
  importing, and its record and executable hash stay as SPEC 22.4 and 22.2 specify. The closure
  tool is additional, run once per project (or once per module if #338 caches the result per
  closure).
- `closure-replay` already includes the project constants, so it could later replace the
  per-module step; that changes an attestation tool and is the owner's call.

## 4. Byte-reproducibility

Method (`repro-check.sh`): build one Mathlib module twice, each time from an empty output
directory and a fresh copy of just that source file, with `lean -R .` (no absolute path) and the
prebuilt oleans of its imports on `LEAN_PATH`, then compare all artifacts. A scratch copy of the
dependency closure was not needed: the closure of imports is the cache itself.

Result: identical. For `Mathlib.Order.Defs.PartialOrder`, `Mathlib.Data.Nat.Prime.Defs` and
`Mathlib.Algebra.BigOperators.Group.Finset.Sigma`, `.olean`, `.olean.server`, `.olean.private`,
`.ir`, `.ilean` and `.c` are byte-identical between the two builds, and all but `.c` equal the
files in the Mathlib cache.

One input is not obvious: Lake passes the package name to the compiler in its `--setup` file
(`"package": "mathlib"`), and Lean records it in the `.olean` (the string `mathlib`, 88 bytes,
and a different header address). Without it the local build differs from the cache in all four
binary files; with it every file is identical. Other options: the `leanOptions` of Mathlib's
`lakefile.lean` (`maxSynthPendingDepth=3`, `autoImplicit=false`, `pp.unicode.fun`, the
`weak.linter.*` set).

Consequence: no canonical declaration serialisation is needed for the pin as it stands. The
bytes of the fetched `.olean` files are a stable identity for a (Mathlib commit, toolchain)
pair, and the cache's `.hash` files or a SHA-256 of the files can serve as content addresses.
Limits: three modules, one toolchain build, x86_64 Linux. The `.olean` header embeds the
toolchain commit hash, so bytes identify nothing across toolchains. If the pin moves, or a
module with nondeterministic behaviour shows up, re-run `repro-check.sh`; the fallback would be
a digest over the sorted `(name, kind, type, value)` of the constants, which the closure tool
already reads. It is not built, because nothing here needs it.

## 5. Not measured, caveats

- Timings come from a shared, loaded machine; the ratios are solid, the absolute seconds are
  not. The 40 s first run of `import Mathlib` is a cold page cache.
- `leanchecker` on every module of the closure was measured once (A) and killed at 1500 s; WSL
  was restarted several times during the session, which killed some runs, and those were rerun.
- `Environment.replay` skips unsafe and partial constants, as `leanchecker` does.
- Axioms such as `Lean.ofReduceBool` are replayed as axioms; the axiom audit (SPEC 22.5) is
  what rejects them.
- No LexLean Rust or SPEC change was made, so no register row, scenario or gate run applies yet.

## 6. Open owner decisions

1. Adopt `closure-replay` as a LexLean verification step? It is a new Lean program the
   attestation would hash (like `leanchecker`, SPEC 22.2), built by the pinned toolchain. It
   needs SPEC 5.1 and 22.4 text, a register row, scenario and failing test first (R3), a
   diagnostic code in ERRORS.md with a negative test (the planted-defect module above is that
   test), and a decision whether it also replaces `leanchecker` per generated module.
2. Reject `import Mathlib` (and any umbrella that re-exports far more than it names) in
   generated Lean: a diagnostic and negative test, or only a lint.
3. Run closure replay per module or once per project (interacts with the replay cache, #338).
4. Whether the content address of an imported declaration is the cache file hash or a
   canonical declaration digest. The first works today (section 4).
