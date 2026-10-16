# Fold-law provenance

`LexLeanRuntime.Laws` (SPEC.md §17.12, *Fold laws*) is the fifth native module
of the runtime project `language/runtime-1.2/library/`: the sums, bridges, and
fold laws that `linear_arithmetic` rewrites with and a `law` proof applies
(#117). Like the four modules migrated before it
(`tests/runtime-library/MIGRATION.md`), it is LexLean source, schema
`lexlean/core-module/2` in the core notation (§17.21.8). This record is the
evidence that the source states what was proved, and that the Lean the
compiler generates from it is the same environment.

## The probe

Run on `main` at `32afe5da` with the pinned Lean 4.32.1, before this change.

- **An integer kernel.** `clampStep x = clamp(((x + 20) · 9365133 + 2³¹) quot 2³²,
  0, 255)` and its monotonicity, written with `less_than`, proved by
  `linear_arithmetic{definitions: [clampStep]}`: `omega could not prove the
  goal: No usable constraints found`. The script did not unfold `less_than`,
  `quotient`, or `negate`, and nothing split the clamp's `if`s.
- **Two folds.** `doubledTotal xs = total xs + total xs`, where `total` folds
  `acc + v` and `doubledTotal` folds `(acc + v) + v`: `omega` reads the two
  folds as unrelated atoms. The missing lemma is a normal form for additive
  folds: a fold whose step adds what each element contributes is its
  initial value plus the sum of the contributions, and the sum is linear in
  the contribution. Once both folds are in that form, the equality is linear
  over one atom, the sum of the identity.

## What the module states

119 declarations: the definitions `sumNat` and `sumInt` with what Lean
generates for them, the theorems the registry names
(`language/runtime-1.2/laws.toml`: 55 rewrites and 3 applied laws), the general
`sumNat_perm` and `sumInt_perm` that the reverse laws are proved from, and the
auxiliary rows Lean realized while proving them: equation lemmas, matchers,
splitters, and private proof terms. That includes equation lemmas of
`LexLeanCollections.lessThan`, `listFold`, and
`LexLeanRuntime.quotient`/`remainder`, which Lean realizes in the module that
asks for them. Every row pins its exact axioms. No row depends on
`Classical.choice`: the bridges are proved by case analysis on `compare` and
by rewriting `isZero`, never by `by_cases` or `simp_all`, so a theorem
proved by `linear_arithmetic` inherits at most `propext` and `Quot.sound` from
the laws it uses.

## How

The one-time oracle is in the checkpoint commit that adds this record
(`b58f1952`, `tests/runtime-library/laws-oracle/`); the commit after it
removes it. Its
exporter, `flags.lean`, `canon.py`, and `compare.py` are those of the runtime
migration (`MIGRATION.md`), with the module list narrowed to `Laws`.
`run.sh <repository> <work>`:

1. `lexlean build` of the runtime project generates the Lean of `Portable` and
   `Collections`. `Laws.lean` is compiled against them with the pinned Lean,
   silently.
2. `flags.lean` and `export.lean` export the `Laws` environment, and `canon.py`
   writes its data form. `lexlean fmt` prints that in the notation, and the
   result equals the committed `language/runtime-1.2/library/src/Laws.lex.tex`
   byte for byte.
3. The Lean `lexlean build` generates from the committed source is compiled,
   exported, and converted by the same programs, and `compare.py` compares the
   two declaration by declaration, as trees of kernel terms.

## Equivalence

Result: the same 119 declarations in the same order. 118 rows are equal in
every byte of their terms. One row, `lessThan.match_1.splitter._sparseCasesOn_2`
(a sparse `casesOn`), is equal up to the names of bound variables, which the
elaborator's construction makes hygienically and no kernel judgment sees. This
is the same difference the migration found for the sparse `casesOn` rows of
`Collections`. The only other difference is the private `Lean` import the
decoder adds to a regenerated module, which SPEC.md §17.10 allows.

## Verification

- Every verification of a project that imports `Laws` (every language-1.2
  module with a `linear_arithmetic` or `law` proof) compiles it, replays it
  with `leanchecker`, and audits each declaration against its row's exact
  axioms. `conformance_rt_03` verifies the runtime project itself, `Laws`
  included.
- `conformance_pf_22` holds the module native with its imports, the
  registry's names declared in it, and every row within `propext` and
  `Quot.sound`. It refuses a registry that names a missing declaration or a
  law twice, and a laws row that only bounds its axioms. It also verifies the
  fold equalities of `examples/fold-laws` for all inputs and has Lean refuse
  three false ones.
- The planted defects are recorded in `VERIFICATION.md` ("the fold laws can
  fail").

## Identity

Languages 1.0 and 1.1 are byte-identical. Every file this change adds or
changes lies in the language-1.2 partition of the compiler-semantics ID
(`language/runtime-1.2/`, `language/semantics-1.2.toml`, the semantic-module
and snapshot schemas of version 2). Every language-1.2 lock (the
compiler-semantics ID substituted), golden, fixture expectation, calculus
package, production export, and verified record was regenerated by the
repository's writers.
