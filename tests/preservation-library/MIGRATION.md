# Preservation library migration provenance

The preservation proof library (SPEC.md §17.17) was a hand-written Lean project
of 13 modules. It is now LexLean source: 13 native core modules of the schema
`lexlean/core-module/2` (SPEC.md §17.10), and the Lean that every verification
compiles is generated from them by the compiler. This record is the evidence
that the migration lost nothing, in the manner of
`examples/uor-atlas/MIGRATION.md`.

## What was migrated

At repository commit `ba52c601` (the merge of #78) the library was
`language/preservation-1.2/library/LexLeanPreservation/*.lean`: Git tree
`50f70bb5ffed11244522abd197d4608d27386302`, 13 Lean files, 525,026 bytes, with
`language/preservation-1.2/library.toml` registering 934 declarations and their
exact axioms. The modules, in dependency order, with what the exporter found in
each of them:

| module | declarations | DAG nodes | source bytes |
| --- | ---: | ---: | ---: |
| `Core` | 230 | 37,821 | 1,623,803 |
| `Values` | 71 | 2,391 | 112,170 |
| `Primitives` | 75 | 5,601 | 231,790 |
| `Fixed` | 331 | 14,674 | 740,120 |
| `Keys` | 457 | 18,870 | 971,298 |
| `Templates` | 201 | 37,054 | 1,371,305 |
| `Validate` | 118 | 13,051 | 512,629 |
| `RustBase` | 358 | 48,164 | 2,150,996 |
| `RustCorr` | 575 | 86,704 | 4,164,434 |
| `RustLemmas` | 6,040 | 571,074 | 28,743,718 |
| `RustMatches` | 686 | 243,840 | 11,604,196 |
| `RustSound` | 29 | 12,277 | 513,399 |
| `Compose` | 76 | 4,580 | 219,505 |
| total | 9,247 | 1,096,101 | 52,959,363 |


## How

The one-time conversion is the oracle of checkpoint commit `d54dbf8551190e4ae77fbf69267fd4328e19db3f`
(`tests/preservation-library/oracle/`, removed from the release tree by the
commit that follows it). It is two programs and a script:

1. `export.lean`, run by the pinned Lean 4.32.1 over the elaborated environment
   of the 13 modules (built with `build.sh` from the Lean sources at
   `ba52c601`). It walks every declaration the library's modules own and writes
   its kernel type and value as a child-before-parent DAG, with its universe
   parameters, reducibility hints and transparency, the structure, class,
   instance and projection registrations, the simplification-set and
   no-confusion records, the matcher and equation-lemma data, the reducibility
   tags, the compiler inlining attributes, the sparse `casesOn` records, the
   `grind` registrations, and the exact axioms `#print axioms` reports for it, which are what
   `Lean.collectAxioms` answers, which is what the audit module prints. For an
   imported constant that is the data stored in its module's `.olean`, computed
   when the module was written with a cache shared across the module's
   constants, so for the members of a reference cycle (a recursive unsafe
   definition; the constructors of an inductive family) it depends on the order
   of the module's constants. A row therefore pins what the pinned Lean prints,
   not a set derived from the term; an earlier version of the exporter derived
   the sets itself and was wrong for 51 rows, which the audit of the
   regenerated library refused.
2. `flags.lean` reads from the same environment the two facts the exporter
   cannot, which constants the code generator compiled and which `casesOn`
   constants are sparse.
3. `canon.py` writes the data as canonical JSON of `lexlean/core-module/2`
   and wraps each module as a `.lex.tex` source.

The sources carry titles from the project glossary
`language/preservation-1.2/library/glossary/preservation.titles`, so the
library project passes the closed-lexicon checks like any project.

## Equivalence

`equivalence.sh` of the oracle takes the Lean the compiler generated from the
committed native modules (`lexlean build` of the library project), compiles it
with the pinned Lean, exports the new environment with the same `export.lean`
and `flags.lean`, and converts it with the same `canon.py`.

Result: the regenerated environment has the same 9,247 declarations in the same
order, in the same 13 modules, and every module's data, declaration by
declaration, is equal, as trees of kernel terms (the node tables are numbered in
the order nodes were met, so rows are compared by the terms they root): kind,
universe parameters, type, value, reducibility hints and transparency, tags,
structure, class, instance, projection, matcher, no-confusion, simplification,
equation-record, inlining, sparse-`casesOn`, and `grind` state, compiled flags,
and exact axioms. 8,934 rows are equal in every byte of their terms. 313 rows,
exactly the sparse `casesOn` constants, are equal up to the names of bound
variables: the elaborator's construction names its binders with fresh hygienic
names (`else._@...._hyg.#N`), which depend on the session that makes them and
which no kernel judgment sees. The one other difference is the private `Lean`
import the generic runtime adds to each regenerated module, which SPEC.md §17.10
allows. The attribute probe, which asks the pinned Lean for the state of every
constant of both environments (inlining kind, sparse record with its type, major
position, arity and constructors, `grind` registration, matcher information,
specialization, noncomputability), prints two equal outputs.

What the first regenerations got wrong, each found by this comparison or by a
conformance test, and fixed:

- constructor eliminators the elaborator compiled before it marked them as
  sparse `casesOn` were left without code, because the runtime marked them first
  (14 rows);
- the sparse `casesOn` records that equation-lemma generation reads (313), the
  compiler inlining attributes (759), and the `grind` registrations (16) were not
  in the data at all, and `simp [Representable]` failed on the regenerated
  library (`conformance_sp_08`); they are now data, and the decoder makes a
  sparse `casesOn` with the elaborator's own construction in a realization of its
  type, because the record is private to that construction and two constants of
  one module (51 rows) may be the same construction, which the elaborator's
  cache would return once;
- a character literal, a comment, and the word `unsafe` in the runtime's text
  were refused by the audit of generated Lean;
- the exporter derived the exact axioms itself and was wrong for 51 rows: the
  pinned Lean answers from data stored in each `.olean`, so a row pins what Lean
  prints.

Not carried, by design, because none of it is kernel or elaboration state a
proof reads: docstrings and module docstrings, declaration source ranges, the
completion blacklist, import-use tracking, and what the code generator derives
(its intermediate representations, specialization caches, and auxiliary
functions). The set of constants the code generator compiled is carried, and
equal.

The exporter's output over the original environment is itself reproduced byte
for byte by the final exporter (the sources committed here are its output, and
a second run over the original oleans produced the same 13 files).

## Registry

`library.toml` no longer lists declarations: the compiler reads every
declaration and its exact axioms from the native modules. The list it reads
equals the list the hand-written registry held:

934 declarations were registered at `ba52c601`; the native modules state 9,247,
of which 7,740 have plain identifier names, are audited by name, and are
printed by the audit module (`library.toml` no longer lists them; the
specification's quoted statement vocabulary is taken from that output). All 934
formerly registered declarations are among the 7,740, in the same module and
with the same exact axioms (the SHA-256 of the sorted derived list of name,
module, axioms is `b8744d3dd836ee35f12dd2a14b2409c4d097b87fc4cef39efb229c5fbb96afcf`).
Four of them, the projections `Rust.RC.at`, `RCB.at`, `RCI.at`, and `RCL.at`,
are named by a segment that is also a Lean token; Lean prints them as written,
so they are audited like any other.

## Verification

The regenerated library is what every verification now compiles, so the
evidence is the verifications themselves:

- `lexlean verify` of every language-1.2 example and of the compiler project
  stages the generated library into the certificate environment, compiles it
  with the pinned Lean, replays each module through `leanchecker` (the same
  kernel, SPEC.md §22.4), prints the axioms of the 7,740 declarations audited by
  name and requires each to equal its row's exact policy, and requires the
  statement vocabulary SPEC.md quotes to equal what Lean prints. Their records
  under `expected/verify/` were regenerated and `cargo xtask verify-examples`
  reproduces them; the certificates of the production examples (certificate A,
  B, and E for every root) are accepted by the pinned Lean over the regenerated
  library, with exactly the three axioms.
- `cargo xtask check-reproducibility` builds the library project twice in two
  directories and requires its 66 published artifacts to be byte-identical.
- `conformance_sp_12` requires the Lean generated from the committed sources to
  equal, byte for byte, the Lean a verification stages, and that the library
  project holds no Lean file of its own.
- The planted defects (a hand-written Lean file, a changed axiom policy, a
  changed term, a changed sparse record, and weakened checks of the format and
  the registry) are recorded in `VERIFICATION.md`, each with the diagnostic it
  produces.

The verification of `examples/uor-atlas` (language 1.0, so its records are not
touched by this change) needs more memory than the development host has and
runs in CI.

## Identities

Languages 1.0 and 1.1 are byte-identical: every file this change adds or
changes lies in the language-1.2 partition of the compiler-semantics ID
(`language/preservation-1.2/`, `schemas/core-module-v2.schema.json`), and
`conformance_rp_10` recomputes all three identities from disk. The
compiler-semantics ID of language 1.2 and everything bound to it were
regenerated:

| language | compiler-semantics ID | change |
| --- | --- | --- |
| 1.0 | `95deb33a8d416d7bf60f02a36e251a71c3ee6f046b7e475d7bae2fc5ddc3767d` | none |
| 1.1 | `c56e1a234cee7c12f00e4f2f53d6094e2ed51f5bbf5195b60f79d4e8aa5bce53` | none |
| 1.2 | `a67e431f04b2517000d2599ccf66fbf03e4315347acabb647973e0d0145cf266` → `961282f94afc09dbbe84cdae395d5b36b6d57c559e4e5ef73d6a1935cb104e90` | regenerated |

Every language-1.2 lock, golden, fixture expectation, and verified record was
regenerated; the goldens and locks of languages 1.0 and 1.1 are unchanged.
