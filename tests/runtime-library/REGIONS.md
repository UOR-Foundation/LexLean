# Decision-region provenance

`LexLeanRuntime.Regions` (SPEC.md §17.12, *Decision regions*) is the sixth
native module of the runtime project `language/runtime-1.2/library/`: the
decision-region checker that a `region_check` proof applies
(UOR-Foundation/PrismPM#605). Like the modules before it
(`tests/runtime-library/MIGRATION.md`, `LAWS.md`), it is LexLean source,
schema `lexlean/core-module/2` in the core notation (§17.21.8). This record is
the evidence that the source states what was proved, and that the Lean the
compiler generates from it is the same environment.

## The probe

Verdant Resonance, a Prism decision model (7 fields, 35 first-match rules,
15 properties), on `release/v0.4` at `bc71b33f` with the pinned Lean 4.32.1,
before this change: `lexlean check` passes, and `lexlean verify` fails after
about five minutes. Each property's `linear_arithmetic` script splits the
35-rule judge into cases, and on one branch of the split `omega` no longer
holds the negations of the earlier rules' tests in a form it reads ("omega
could not prove the goal"). The properties are true: brute force over all
3,131,000 in-bounds requests finds no counterexample.

The missing piece is a proof whose size does not grow with the case split: for
one property, each rule whose answer breaks it contributes one obligation, the
part of its box inside the property's box minus the boxes of the rules before
it is empty, and box subtraction over a few intervals is something the kernel
decides quickly.

## What the module states

379 declarations in the namespace `LexLeanRegions`:

- the data `Cond`, `Rule`, `Property`, with what Lean generates for
  structures;
- the semantics in the backend's own shapes: `get`, `opHolds` over eight
  operator codes written with `Nat.blt`, `Nat.ble`, `Nat.beq`, and `!`,
  `condHolds`, `allHold` folding `&&` from the left (`allHoldFrom`),
  `judge` testing each rule with `if b then`, `bad`, and `breaks`;
- `Spec`, the same semantics written with `if` over six codes, with boxes
  (`inBox`, `sub`, `subAll`, `meet`, `condBox`, `boxOf`), the obligations
  (`oblRule`, `oblRules`, `oblOther`, `regionCheck`), the property hull
  (`upper`, `hull`), and their lemmas, ending in `Spec.regionCheck_sound`;
- the bridge `opHolds_spec`, `condHolds_spec`, `allHoldFrom_spec`,
  `allHold_spec`, `judge_spec`, `breaks_spec`, from the public shapes to
  `Spec`;
- `obligation`, and `holds`: if `obligation B R d P = true`, then for every
  request `x` of the box's width, `breaks R d P x = false`;
- the auxiliary rows Lean realized while proving them: equation lemmas,
  matchers, splitters, and private proof terms.

Every row pins its exact axioms. No row depends on `Classical.choice`: `holds`
and every row it uses are within `propext` and `Quot.sound`. The proofs are
written with `induction` and non-overlapping patterns, never by recursive
pattern matching in a theorem or a wildcard after literals: the decoder of
core module 2 rebuilds a matcher's equation lemmas from the elaborator's
construction, and the first export, which used both, did not decode (a
matcher's equation lemma for an overlapping wildcard was "not what the
elaborator's construction makes", and two theorems by structural recursion
named private matchers the decoder had not realized).

## How

The one-time oracle is in the checkpoint commit that adds this record
(`tests/runtime-library/regions-oracle/`); the commit after it removes it. Its
exporter, `flags.lean`, `canon.py`, and `compare.py` are the fold-law oracle's
(`b58f1952`), narrowed to `Regions`. `run.sh <repository> <work>`:

1. `Regions.lean` is compiled with the pinned Lean, silently, and
   `flags.lean` and `export.lean` export its environment; `canon.py` writes
   its data form, and `lexlean::sdk::to_notation` prints it in the core
   notation, which is the committed
   `language/runtime-1.2/library/src/Regions.lex.tex`.
2. `lexlean fmt --check` holds that source canonical, and `lexlean build` of
   the runtime project generates its Lean.
3. That Lean is compiled, exported, and converted by the same programs, and
   `compare.py` compares the two declaration by declaration, as trees of
   kernel terms.

## Equivalence

Result: the same 379 declarations in the same order, every one equal in every
byte of its terms. The only other difference is the private imports the
decoder adds to a regenerated module (`Lean`,
`Lean.Meta.Constructions.SparseCasesOn`, `Lean.Compiler.LCNF.Main`,
`Lean.Compiler.LCNF.PhaseExt`), which SPEC.md §17.10 allows.

## Verification

- Every verification of a project that imports `Regions` (every language-1.2
  module with a `region_check` proof) compiles it, replays it with
  `leanchecker`, and audits each declaration against its row's exact axioms.
  `conformance_rt_03` verifies the runtime project itself, `Regions` included.
- `conformance_pf_24` holds the module native, importing only `Init`, with
  `breaks`, `obligation`, and `holds` within `propext` and `Quot.sound`. It
  pins the proof the backend emits for a property of
  `examples/decision-regions`, refuses in linking a rule with `!=`, a property
  that leaves a field unbounded, a comparison of a sum, a `region_check` used
  as a branch, and a language-1.1 use, verifies the example, and has Lean
  refuse a copy whose heating rule drops its occupancy test at exactly the two
  properties that rule breaks.
- On Verdant Resonance's canonical module with every `linear_arithmetic`
  proof replaced by `region_check{}`, `lexlean verify` succeeds in about 73 s
  (all 15 properties), and a copy with rule 2's proximity limit raised from 20
  to 60 is refused at `inv004Holds` and `inv008Holds`, the two properties the
  change breaks.

## Identity

Languages 1.0 and 1.1 are byte-identical. Every file this change adds or
changes lies in the language-1.2 partition of the compiler-semantics ID
(`language/runtime-1.2/`, the semantic-module and snapshot schemas of version
2). The repository's writers regenerated, on Linux x86-64 with the pinned Lean
4.32.1, every language-1.2 lock, golden, fixture expectation, calculus and
production package, and the verified records of every example but
`examples/uor-atlas`. That example's verification was stopped by the host's
out-of-memory guard in `CensusCert11` (its hinted peak is about 12.1 GB), so
its records are not yet regenerated at this identity.
