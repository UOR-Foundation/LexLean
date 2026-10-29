# Mathlib import: glossary generation (#335), working notes

Input to the design document of #341. Status: prototype on branch
`feat/mathlib-glossary`, committed locally, not pushed.

## What exists

`lexlean deps glossary` turns Lean declarations into a lexicon package of
`kind = "lean"` entries plus a coverage report. It needs no project.

```text
lexlean deps glossary --package <ID> [--version V] [--language L]
    (--modules M... | --names N...) [--workspace DIR] --out DIR
    [--report FILE] [--strict]
```

- `--workspace` is a Lake workspace whose libraries are already built (the
  `~/mathlib-pinned` checkout from #334). The command runs
  `lake env lean --run` there; without it, plain `lean --run` over the
  toolchain's own libraries (`Init`, `Std`, `Lean`). Nothing is fetched or built.
- The Lean metaprogram is `crates/lexlean/lean/GlossaryExtract.lean`, compiled
  into the executable and written to a private temp file per run. It prints one
  JSON document (sorted keys, declarations sorted by name). Everything after
  that is Rust (`crates/lexlean/src/glossary/`), so package bytes, codes and the
  report are fixed by the release and testable without Lean.
- Normative text: SPEC §13.12 (translation, categories, naming, eliminators,
  output) and §23.4 (`deps glossary`). Register rows `CL-25`, `GL-19`, `GL-20`.
  Codes `LLT4101`–`LLT4107`.

## How a declaration is translated

The input is the kernel `Expr` of the declaration's type, not its syntax.

- `Prop` -> `(sort prop)`; `Type u` -> `(sort (type u))`; `Sort u` is refused.
- Binder infos map to `explicit`/`implicit`/`instance`. LSE already has an
  `instance` binder mode and universe variables, so instance arguments and
  universe polymorphism over `Type u` are **not** refusal reasons.
- An application keeps only the arguments of explicit binders (what §17.6 says
  an LSE application is). The dropped implicit and instance arguments are
  re-inferred by Lean when verification probes the entry (§13.7).
- `OfNat.ofNat _ n _` -> `(nat n)`. `outParam`, `semiOutParam`, `optParam`,
  `autoParam` are erased (identity on the type); the report counts entries
  where that happened.
- `Eq`, `And`, `Or`, `Not`, `Iff`, `Exists` map to the existing core entries.
  Those are Type-level, so `Eq` at `Sort 0` (proofs of a proposition) is refused.
- Every constant a signature mentions must have an entry, so the generator
  translates the closure of the request. The added types, classes, functions,
  constructors are *support entries*. A declaration that mentions a refused,
  excluded or absent one is refused, transitively, with the root reason.
- Before anything is written, each entry goes through `parse_entry` and the
  conservative `check_signature` of the package loader, and the whole package
  through `load_package`. An entry that fails is refused (`LLT4107`) and so are
  its dependents.

## Naming scheme

(Full rules in SPEC §13.12.)

| Lean | entry id | file | surface |
|---|---|---|---|
| `Nat.add_comm` | `nat.add-comm` | `entries/nat/add-comm.toml` | `natAddComm` |
| `HAdd.hAdd` | `h-add.h-add` | `entries/h-add/h-add.toml` | `hAddHAdd` |
| `Nat.pow_2` | `nat.pow-2` | | `natPow2` |

Words split at `_`, camel humps, capital runs; `'` is the word `prime`.
Colliding ids all take a `-<6 hex of sha256(lean name)>` suffix. Surfaces are
not unique (`Nat.add_comm` and `Nat.addComm` share one); that is by design and
the report counts it.

Categories follow the type: theorem -> `proof-constant`; type former ->
`type-noun` / `function`; proposition former -> `predicate-constant`,
`adjective-predicate` (1 argument), `transitive-predicate` (2), else `function`;
term -> `term-constant` / `function`. Predicates get a prose form from the last
name component (`Nat.Prime` -> "prime") only if it is letters only.

## Verification of the prototype

- `conformance_gl_19` (package + determinism), `conformance_gl_20` (refusals,
  coverage arithmetic, strict mode), `conformance_cl_25` (command surface and a
  real Lean run, twice, byte-identical).
- `tests/glossary/GlossaryFixtures.lean` exercises every refusal reason;
  `tests/glossary/extraction.json` is the committed extractor output and the
  Lean-backed cases re-extract and compare. Regenerate with
  `tests/glossary/regenerate.sh`.
- Not done: a real Lean probe over the generated entries (see decisions).

## Coverage

All numbers come from `lexlean deps glossary` (debug build) on a 16 GB WSL
host shared with other jobs, Mathlib at `520045ab` with its built oleans.
"Excluded" are requested declarations that are not candidates (internal names,
recursors, matchers and other auxiliaries, instances, deprecated, and the six
core constants); they are not refusals. `requested = translated + refused +
excluded`.

**Mathlib, eight modules** (`Algebra.Group.Defs`, `Algebra.Group.Basic`,
`Algebra.Ring.Defs`, `Algebra.Order.Group.Defs`, `Order.Basic`, `Logic.Basic`,
`Data.Nat.Basic`, `Data.Int.Basic`): 3,065 requested.

| Outcome | Count |
|---|---|
| translated | 1,146 (+236 support entries; 1,383 entry files in all) |
| refused: `Sort u` polymorphism (LLT4101) | 174 |
| refused: name outside grammar, mostly `'` (LLT4105) | 166 |
| refused: strict-implicit binder (LLT4102) | 13 |
| refused: core constant at wrong level (LLT4104) | 7 |
| refused: depends on a `Sort u` declaration (LLT4106, root LLT4101) | 215 |
| refused: depends on an excluded instance (LLT4106) | 23 |
| refused: depends on an excluded internal/auxiliary declaration | 10 |
| refused: depends on a non-grammar name | 6 |
| excluded: internal 671, auxiliary 303, instance 229, recursor 83, deprecated 19 | 1,305 |

So 65% of the 1,760 candidate declarations translate; typeclass-heavy lemmas
are among them. No declaration is refused for having an instance argument.

**Lean core, six `Init` modules** (`Prelude`, `Core`, `Data.Nat.Basic`,
`Data.List.Basic`, `Data.Int.Basic`, `Data.BitVec.Lemmas`): 6,892 requested,
2,445 translated, 1,050 refused (481 `Sort u`, 195 names, 76 signature-check,
4 level, 294 dependency), 3,397 excluded, 81 support entries.

**Time and peak memory** (whole command, including the Lean child; the Rust
side is a fraction of a second):

| Request | Wall | Peak RSS |
|---|---|---|
| 6 `Init` modules, 6,892 declarations | 7.6-11 s | 1.4-1.7 GB |
| 4 Mathlib modules, 1,777 declarations | 26 s (cold cache) | 2.9 GB |
| 8 Mathlib modules, 3,065 declarations | 7.5 s warm | 2.9 GB |
| first Mathlib run after the oleans were fetched | 2 min 50 s (cold page cache, machine busy) | 3.4 GB |

The memory is Lean importing Mathlib's environment; it does not grow with the
request. Scaling to all of Mathlib was not measured.

## Open owner decisions

1. **Apostrophe in names.** §13.6's conservative Lean-name grammar excludes
   `'`, so every `foo'` is refused (`LLT4105`): 195 of ~3,500 requested in six
   `Init` modules, and 74 of 1,777 in the first Mathlib sample. Widening the
   grammar (and the LSE/probe identifier quoting) would recover most of them.
2. **Strict-implicit binders** (`⦃x⦄`) have no LSE mode (`LLT4102`). Mathlib's
   set-theoretic lemmas use them heavily. Either add a mode to LSE or accept the loss.
3. **`Sort u` polymorphism** (`LLT4101`) is the largest refusal and cascades
   into everything that mentions `id`, `Function.comp`, `Subtype`, `PUnit`, and
   so on. LSE has `Prop` and `Type u` only. Options: leave as is; add a `Sort u`
   form to LSE; or hand-map the few Sort-polymorphic core constants to entries.
4. **Rust signature checker** rejects some dependent signatures that Lean
   accepts (`getElem`-style `valid xs i` binder types): 76 of ~2,500 translated
   `Init` declarations. They are refused (`LLT4107`), never emitted. Loosening
   the checker is a separate decision.
5. **Existing std entries.** `lexlean.std.nat::nat` and a generated `nat`
   denote the same Lean constant. Mapping Lean names to existing `kind = "lean"`
   entries of the std packages would avoid cross-package ambiguity; not done.
6. **Probe.** Dropping implicit arguments and erasing annotations is checked
   only when `verify` generates the §13.7 probe. A `--probe` pass inside the
   command (reuse `backend::lean::probe_module`, run Lean once over all entries,
   refuse mismatches with `LLT4003`) would make "never weakened" a property of
   the generator. Estimated as a day of work.
7. **Report-only refusals vs failure.** Refusals are reported with exit 0;
   `--strict` makes them errors. Confirm that is the wanted default.
8. **Code ranges.** `LLT4101`–`LLT4107` are mine; the sibling sessions need to
   avoid them. The register count (`SPEC.md` "Total required capability IDs"
   and `xtask/src/spec_links.rs`) is a literal that each branch bumps, so the
   branches will conflict there; resolve by recounting at merge.
9. **Eliminator names.** Default `T.casesOn` / `T.rec`; custom
   `@[cases_eliminator]` / `@[induction_eliminator]` are honoured. Check that
   the backend's `cases ... using` / `induction ... using` emission is happy
   with `T.rec` for indexed families (not exercised yet).

## Interfaces assumed from the siblings

- #334: only a built Lake workspace path (`--workspace`). No config, lock or
  fetch code is touched. The `deps` command group is mine for now
  (`CommandKind::Deps` / `DepsAction::Glossary` in `cli.rs`); #334 will want to
  add `fetch` beside `glossary`, which is a small merge in that enum and in
  `COMMAND_NAMES`.
