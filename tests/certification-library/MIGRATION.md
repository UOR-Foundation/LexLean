# Certification library migration provenance

The GNAF certification proof library (SPEC.md §17.18) was a hand-written Lean
project of 13 modules. It is now LexLean source: 13 native core modules of the
schema `lexlean/core-module/2` (SPEC.md §17.10), and the Lean that
`conformance_gc_09` compiles and audits is generated from them by the compiler.
This record is the evidence that the migration lost nothing, in the manner of
`tests/preservation-library/MIGRATION.md` and `examples/uor-atlas/MIGRATION.md`;
it uses the same exporter, parametrised for this library.

## What was migrated

At repository commit `ba52c601` (the merge of #78) the library was
`compiler/proofs/LexLeanCertification/*.lean`: Git tree
`298c8f97742584f297546db8b972c9cb9cdbfda8`, 13 Lean files, 97,597 bytes, with
`compiler/proofs/library.toml` registering 107 declarations and their exact
axioms. The modules, in dependency order, with what the exporter found in each
of them:

| module | Lean bytes | declarations | DAG nodes | source bytes |
| --- | ---: | ---: | ---: | ---: |
| `Trials` | 8,191 | 103 | 9,954 | 418,614 |
| `Tail` | 5,486 | 112 | 12,803 | 600,382 |
| `Factor` | 8,462 | 3,631 | 58,049 | 3,593,577 |
| `Schemas` | 13,110 | 181 | 19,802 | 842,763 |
| `Facts` | 7,000 | 60 | 7,234 | 311,022 |
| `PremiseSound` | 3,363 | 6 | 3,865 | 140,311 |
| `Positions` | 4,054 | 45 | 3,471 | 155,232 |
| `Lower` | 9,552 | 50 | 12,434 | 478,781 |
| `Frontier` | 15,904 | 113 | 27,690 | 1,073,909 |
| `Representation` | 7,590 | 63 | 12,033 | 482,450 |
| `Normality` | 4,497 | 7 | 3,504 | 130,001 |
| `Check` | 4,998 | 20 | 12,231 | 441,980 |
| `Soundness` | 5,390 | 5 | 5,533 | 196,280 |
| total | 97,597 | 4,396 | 188,603 | 8,865,302 |

Most declarations are not written in the Lean files: the elaborator generated
them (equation lemmas, matchers, structure projections, proof auxiliaries), and
the data carries them with the declarations that cause them.

## How

The one-time conversion is the oracle of checkpoint commit `0d90ea93e81d6306bc83adead8697ce63343b514`
(`tests/certification-library/oracle/`, removed from the release tree by the
commit that follows it). It is the preservation library's exporter
(`tests/preservation-library/oracle/` of its own checkpoint) with this library's
module list, prefix, and titles: `export.lean` over the elaborated environment
of the 13 modules, `flags.lean` for the constants the code generator compiled
and the sparse `casesOn` records, `canon.py` for the canonical sources, and
`equivalence.sh`, `compare.py`, and `probe.lean` for the comparison below. The
environment it ran over was the library's 13 modules, compiled from the Lean of
`ba52c601`, over the compiler project's generated `Gnaf`, `GnafFixtures`,
`Certification`, and `GnafCertificates` (of that commit), the calculus modules,
and the preservation library as the compiler generates it from its native
modules (the library imports only `LexLeanPreservation.Core` of it, by the
names of its constants).

The sources carry titles from the project glossary
`compiler/proofs/glossary/certification.titles`, so the library project passes
the closed-lexicon checks like any project. Its entrypoint is `Soundness`,
which imports every other module.

## Equivalence

`equivalence.sh` of the oracle takes the Lean the compiler generated from the
committed native modules (`lexlean build` of the library project), compiles it
with the pinned Lean over the same environment, exports the new environment
with the same `export.lean` and `flags.lean`, and converts it with the same
`canon.py`.

Result: the regenerated environment has the same 4,396 declarations in the same
order, in the same 13 modules, and every module's data, declaration by
declaration, is equal, as trees of kernel terms: kind, universe parameters,
type, value, reducibility hints and transparency, tags, structure, class,
instance, projection, matcher, no-confusion, simplification, equation-record,
inlining, sparse-`casesOn`, and `grind` state, compiled flags, and exact axioms.
4,395 rows are equal in every byte of their terms. One row, a sparse `casesOn`
constant of `Schemas`, is equal up to the names of bound variables, which the
elaborator's construction makes hygienically and which no kernel judgment sees.
The one other difference is the private `Lean` import the generic runtime adds to
each regenerated module (SPEC.md §17.10 allows it). The attribute probe, which
asks the pinned Lean for the state of every constant of both environments,
prints two equal outputs (the difference file is empty).

Not carried, by design, because none of it is kernel or elaboration state a
proof reads: docstrings and module docstrings, declaration source ranges, the
completion blacklist, import-use tracking, and what the code generator derives.
The set of constants the code generator compiled is carried, and equal.

## Registry

`library.toml` no longer lists declarations (its tag is now
`lexlean/certification-library/2`): every declaration and its exact axioms are
read from the native modules. 107 declarations were registered at `ba52c601`;
the native modules state 4,396, of which 3,957 have plain identifier names and
are audited by name by `conformance_gc_09` (the rest are generated with an
inductive type or have a private or escaped name, and are covered by every
declaration that uses them). All 107 formerly registered declarations are among
the 3,957, in the same module and with the same exact axioms. The 3,957 pin
`Classical.choice`, `Quot.sound`, and `propext` (3,534), `Quot.sound` and
`propext` (57), `propext` (54), or none (312); the SHA-256 of the sorted derived
list of name, module, and axioms is
`5b14f2aaf05928280a9fd3734ba4a3f664aede66e1a347596dc92a2e3e6e1b9f`.

## Verification

- `conformance_gc_09` compiles the Lean generated from the native modules over
  the compiler project's generated modules and the shipped preservation
  library, prints the axioms of the 3,957 audited declarations and requires each
  to equal its row's pin, instantiates the soundness theorems at every certified
  fixture, replays the library and the instances through `leanchecker` (the same
  kernel, SPEC.md §22.4), and requires Lean to reject a weakened checker and a
  false lemma and the audit to see an added axiom.
- `conformance_gc_11` requires the library project to hold no Lean file and no
  Lean file in the release tree to declare the library's namespace (the
  permanent condition `VR-19` states of the Atlas), its sources to be the listed
  modules, the registry to list only the modules, the
  Lean `lexlean build` writes to equal the Lean `gc_09` compiles, a drifted
  source to generate different Lean, and a foreign import or an inexact policy
  to be refused.
- `cargo xtask check-reproducibility` builds the library project twice in two
  directories and requires the published artifacts to be byte-identical.
- The planted defects (a changed policy, a changed statement, a handwritten Lean
  file, a disabled import check, a stale generated module) are recorded in
  `VERIFICATION.md`, each with the failure it produces.

## Identities

The library is repository evidence: it is not shipped and no file of it lies in
the language data or the schemas the compiler-semantics IDs hash. Languages 1.0,
1.1, and 1.2 are byte-identical to the base of this change, and
`conformance_rp_10` recomputes all three identities from disk.
