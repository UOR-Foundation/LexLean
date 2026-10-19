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

## Derived rows (#91 stage 2)

The library stored, for every row the pinned Lean makes itself (the 6,890 rows
whose names it reserves, which are the equations, unfolding equations, and
splitters of matchers and definitions and the `else_eq` of sparse `casesOn`
constants, and the 313 sparse `casesOn` constants), the type and value Lean made,
and the decoder compared them with what its construction made. A copy of what
Lean makes goes stale when a type of the calculus changes and nothing outside Lean
can rewrite it, so these rows are now *derived* (SPEC.md §17.10, `SM-34`): a row
states its name, kind, and extension state, and for a sparse `casesOn` its record,
and no universe parameters, type, value, reducibility metadata, or group. The
decoder makes the constant at the row's place in the order.

**How.** The one-time converter is the oracle of checkpoint commit
`d87385f92` (`tests/preservation-library/oracle-derived/`, removed from the
release tree by the commit that follows it). It reads each of the 13 native
modules, checks that the module is the canonical notation of itself, marks as
derived every row whose name ends `eq_<n>`, `eq_def`, `splitter`, or `else_eq`
or that carries a sparse record, drops what the construction decides, removes the
nodes no remaining row reaches (renumbering the rest in order), checks that the
module validates and that its notation reads back to it, and writes the notation.
`lexlean fmt --check` holds the 13 modules canonical.

| module | rows derived | nodes before | nodes after | source bytes before | source bytes after |
| --- | ---: | ---: | ---: | ---: | ---: |
| `Core` | 121 | 37,821 | 31,011 | 1,218,748 | 968,302 |
| `Values` | 0 | 2,391 | 2,391 | 70,609 | 70,609 |
| `Primitives` | 15 | 5,614 | 5,358 | 153,500 | 146,765 |
| `Fixed` | 3 | 14,674 | 14,632 | 512,246 | 510,964 |
| `Keys` | 424 | 18,870 | 4,605 | 796,856 | 211,428 |
| `Templates` | 20 | 37,054 | 35,439 | 829,853 | 765,497 |
| `Validate` | 35 | 13,051 | 10,309 | 322,612 | 238,989 |
| `RustBase` | 220 | 48,164 | 32,780 | 1,595,627 | 1,074,826 |
| `RustCorr` | 184 | 86,704 | 45,828 | 2,880,189 | 1,250,898 |
| `RustLemmas` | 5,635 | 571,074 | 390,732 | 24,353,232 | 17,219,467 |
| `RustMatches` | 511 | 243,840 | 157,737 | 9,147,820 | 5,419,902 |
| `RustSound` | 14 | 12,277 | 9,815 | 418,463 | 338,719 |
| `Compose` | 21 | 4,580 | 3,813 | 162,717 | 140,680 |
| total | 7,203 | 1,096,114 | 744,450 | 42,462,472 | 28,357,046 |

The Lean the compiler generates from the library falls from 82,856,219 to
57,115,996 bytes.

**Equivalence.** The environment the derived rows make is the environment the
stored rows made. The Lean `lexlean build` generates from the library before and
after the change was compiled by the pinned Lean 4.32.1 and exported with the
exporter, `flags.lean`, `probe.lean`, `canon.py`, and `compare.py` of checkpoint
`d54dbf85` (above), unchanged. The comparison is of the two re-exports, not of the
new re-export with the committed sources: that checkpoint's exporter predates the
runtime library (#90) and the later moves of the library, and its re-export of
the unchanged library already differs from the committed data in all 13 modules
(the imports, the order of the rows of `Keys` and `RustLemmas`, rows that name
runtime constants, and the `else_eq` of sparse constants), so it can establish the
equality of two re-exports and not the equality of one with the committed
sources.

Result: the two re-exports have the same 9,247 declarations in the same order in
the same 13 modules, and every module's data, declaration by declaration, is equal
as trees of kernel terms: kind, universe parameters, type, value, reducibility
hints and transparency, tags, structure, class, instance, projection, matcher,
no-confusion, simplification, equation-record, inlining, sparse-`casesOn`, and
`grind` state, compiled flags, and exact axioms. 9,213 rows are equal in every
byte of their terms; 34 rows (10 in `RustLemmas`, 24 in `RustMatches`), all sparse
`casesOn` constants, are equal up to the hygienic names of bound variables, as
before. The modules' imports are equal beyond the two imports the decoder adds
(`Lean` and `Lean.Meta.Constructions.SparseCasesOn`) in both. The attribute probe
(inlining kind, sparse record with its type, major position, arity and
constructors, `grind` registration, matcher information, specialization,
noncomputability, of every constant) prints 1,088 identical lines for both
environments.

**Verification.** `lexlean lock && lexlean verify` in a copy of the project of
`tests/negative/preservation-drift` stages the library into the certificate
environment, compiles it, replays each module through `leanchecker`, and audits
the axioms of the 7,740 declarations audited by name against their policies, and
reports `verified 2 modules`. Derived rows keep their axiom policy, so the audit
reads the same rows as before. A certificate cites no derived row: the library
states no type of it, and a certificate that tried would be refused by the
builder naming the row.

**What this change did not do.** The language-1.2 compiler-semantics ID moves
with the library and the schema. Its locks, goldens, fixtures, calculus and
production records, and verified records were not regenerated here: they are
regenerated once, with the rest of the train, so every test that reads a committed
lock reports `LLC0102` until then.

## Authored proofs repaired for `Callee.machine` and `Lit.overflow` (#91 stage 2, `SP-18`)

`RustSyntax` gained a callee and a literal: `Callee.machine` over the eight
primitives of `Machine`, and `Lit.overflow`. Every row the pinned Lean makes
itself is derived (above), so the 7,203 of them are made for the new modules and
nothing stored goes stale. Five authored declarations are one case short, as hh
measured on #91:

| declaration | module | what the new constructor changes in its proof |
| --- | --- | --- |
| `stable` | `RustBase` | 20 applications of `eval.match_3` take a third alternative, and its `Callee.casesOn` a third arm |
| `rc_call_function` | `RustLemmas` | 2 applications of `eval.match_3` take a third alternative |
| `rc_call_runtime` | `RustLemmas` | 2 applications of `eval.match_3` take a third alternative |
| `rc_call_halt` | `RustLemmas` | 4 applications of `eval.match_3` take a third alternative |
| `rc_negate` | `RustLemmas` | its `Lit.casesOn` takes a ninth arm |

**How.** The one-time tool is the oracle of checkpoint commit `37468a723`
(`tests/preservation-library/oracle-machine/`, removed from the release tree by the
commit that follows it). It edits kernel terms; it adds no Lean. The new alternative
of the matcher is `fun primitive => runMachine primitive <operands>`, the operands
those of the alternative for `Callee.runtime`. The new arm of `Callee.casesOn` is the
arm for `runtime` over a primitive instead of an item, since the proof of that case
does not depend on what the callee runs: both sides of the goal are the same
`returned (…) prop`. The new arm of `Lit.casesOn` is the arm for `unit` with
`Lit.overflow` in its place: `negateR` writes a literal that is not a Boolean as
`Expr.not` of it, so the case closes with the hypothesis about `Expr.not` of the
literal. A
declaration's statement is not edited.

**What it established.**

- Each of the five is needed: with the other four repaired and any one left as it
  was, `RustBase` or `RustLemmas` is refused at that declaration (`stable`,
  `rc_call_function`, `rc_call_runtime`, `rc_call_halt`, and `rc_negate`, each in
  its own run). With all five repaired, the 13 modules compile over the new
  modules with no error, so these five are the whole set.
- Nothing else changed. Compared as trees of kernel terms, the old and new
  `RustBase` and `RustLemmas` (358 and 6,040 declarations, including their derived
  rows) differ in exactly these five declarations; the other 11 modules are
  byte-identical, and the statement of each of the five is the tree it was.
- The result verifies. `lexlean lock && lexlean verify` in a copy of the project of
  `tests/negative/preservation-drift` compiles the library over the regenerated
  modules, replays every module, and audits the exact axioms of the 7,740 declarations
  audited by name (`verified 2 modules`), and the compiler project verifies with the
  regenerated `RustSyntax` and `RustSemantics` (`verified 17 modules`).

`RustBase` has 33,911 nodes after (32,780) and `RustLemmas` 392,182 (390,732), the
edited values being copies. The shipped copies of `RustSyntax` and `RustSemantics`
in `language/preservation-1.2/modules/` are regenerated from `compiler/src`; their
golden under `compiler/expected/` is not, with the rest of the records bound to the
compiler-semantics ID.

## Authored proofs repaired for `Expr.loop` (#91 stage 3, `SP-19`)

`Expr` gained a constructor, `loop` over an `Ident` and a `Block`, and
`RustSemantics.eval` an arm that recurses through `eval` itself, so the mutual group
and the statement of `stable` are the ones they were. Every declaration that does
`Expr.casesOn` is one case short. The oracle is checkpoint commit `a549a3d48`
(`tests/preservation-library/oracle-loop/`, removed by the commit that follows it).

| module | declarations | repair |
| --- | --- | --- |
| `RustBase` | `stable` | an arm for `loop` made from the arm for `block` with its proof replaced by an application of `stable_loop`; `stable_loop` and the derived row `eval.eq_23` are added before it |
| `RustLemmas` | `rc_read`, `rcb_flat`, `foldStep_spec`, `rc_negate`, `rc_negate_halt` | an arm for `loop` made from a donor constructor's arm, whose proof uses its fields only through the constructor |
| `RustLemmas` | `rc_litValue.match_1` | a matcher alternative for `loop` (its type, value, arm of `Expr.casesOn`, and record), and the alternative passed at each application |
| `RustMatches` | `ctorOf_eq`, `loadOf_spec`, `rc_partRead`, `rebuildArg_rc`, `rebuild_value` | the same donor arm |

`stable_loop` is the one new proof. It is Lean, compiled over the new machine modules,
exported with the exporter of checkpoint `d54dbf85`, and grafted in; its text is:

```lean
theorem stable_loop (n : Nat)
    (ihE : ∀ (c : Crate) (env : List (Ident × Value)) (e : RustSyntax.Expr),
      eval n c env e ≠ ROutcome.exhausted → eval (n + 1) c env e = eval n c env e)
    (ihB : ∀ (c : Crate) (env : List (Ident × Value)) (b : Block),
      evalBlock n c env b ≠ ROutcome.exhausted → evalBlock (n + 1) c env b = evalBlock n c env b)
    (c : Crate) (env : List (Ident × Value)) (s : Ident) (b : Block)
    (h : eval (n + 1) c env (RustSyntax.Expr.loop s b) ≠ ROutcome.exhausted) :
    eval (n + 1 + 1) c env (RustSyntax.Expr.loop s b) = eval (n + 1) c env (RustSyntax.Expr.loop s b) := by
  rw [eval, eval] at *
  have hb : evalBlock n c env b ≠ ROutcome.exhausted := by
    intro hx; rw [hx] at h; exact h rfl
  rw [ihB c env b hb]
  generalize evalBlock n c env b = r at *
  cases r with
  | value step =>
    cases step <;> simp at h ⊢
    all_goals first | rfl | skip
    exact ihE c _ _ h
  | _ => rfl
```

A declaration's statement is not edited, and the 13 modules compile over the loop-extended
machine modules with no error. What was not run: the compiler project's `verify` needs the
records the maintainer regenerates, and reports that the compiler semantics changed since
locking.

## The loop form certified (#91 stage 4, `SP-20`)

The renderer writes a function whose self calls are all tail calls as a loop, so the crate no
longer holds the body the calculus function corresponds to. `FunOK` is now `FunOK0 ∨ FunLoop`:
`FunOK0` is the statement `FunOK` was, and `FunLoop` states that the crate's body is `loopOf` of
an ordinary recursive body, with the Boolean checks of `loopCheckOf` and the `Corr` derivation of
that body. `Corr`, its recursor, and `sound` are untouched; the soundness of the new disjunct is
`funSem_loop`, and `funSem_of` is `Or.casesOn` over the two. The oracle is checkpoint commit
`448aa180c` (the Lean sources, `tests/preservation-library/oracle-loopcorr/lean/`, 2,707 lines,
198 declarations, no `sorry`) and the commit that follows it (the graft and surgery tools,
`oracle-loopcorr/src/bin/`), both removed by the commit after them.

| module | rows | change |
| --- | --- | --- |
| `RustCorr` | 149, before `FunOK` | `Pos`, `carried`, `pack`, `rtE`/`rtB`/`rtArms`, `loopParams`, `loopParts`, `loopOf`, `loopCheck`, `loopCheckOf`, and `FunLoop`, exported from Lean over the committed library |
| `RustCorr` | `FunOK`, `CrateOK` | `FunOK` becomes `FunOK0` (its value unchanged) and a new `FunOK p c A f := Or (FunOK0 p c A f) (FunLoop p c A f)`; `CrateOK`'s height and axiom set follow |
| `RustSound` | 751, before `funSem_of._simp_1_14` | the relational layer for the loop, the relevance and glue lemmas, `funSem_loop` with its exact axioms (`Classical.choice`, `Quot.sound`, `propext`) |
| `RustSound` | `funSem_of` | the existential case analysis is the first arm of an `Or.casesOn` over the two disjuncts; the second is `funSem_loop` |

`funSem_loop`'s statement:

```lean
theorem funSem_loop {p : TargetSyntax.Program} {c : RCrate} {A : Flags} {n f : Nat}
    (hS : ∀ (Γ : Ctx) (fl : Bool) (j : Jd), Corr p c A Γ fl j → Sem p c A n Γ fl j)
    (h : FunLoop p c A f) : FunSem p c A n f
```

Found on the way, each fixed where it was found:

- The exporter names a private auxiliary declaration after the module it was elaborated in; a row
  grafted into another module is renamed to the host module (`_private.LexLeanPreservation.<host>.#0`),
  or the decoder cannot reproduce it.
- A name with an apostrophe (`All2.imp'`) cannot be parsed from the audit's output, which quotes
  names with it; the lemma is `All2.mono`.
- The decoder cannot realize a sparse `casesOn` over an inductive declared in the same module
  (`Pos`), because the realization base environment has no `Pos.casesOn`; `keptNames` is written
  with its constructors spelled out. A decoder that realizes the same-module recursor is the
  change that would remove the restriction; it is not made here.
- `compileRows` compiles a mutual group in the command that adds its last member, so a command
  boundary inside a group failed (`patBinders`/`patBindersList`). This change requires #212, which
  keeps a compiled mutual group in one command.

Run: the 13 modules compile with no error over the pinned Lean 4.32.1 (`RustCorr` 10 s,
`RustLemmas` 64 s, `RustMatches` 14 s, `RustSound` 13 s, `Compose` 4 s), the library audit is
satisfied for every row, and `conformance_sp_20` compiles certificate B of the fixtures rendered
as loops over it. What was not run: the compiler project's `verify` and every case that reads a
committed lock, golden, or export report that the compiler semantics changed since locking; the
maintainer regenerates those records.

