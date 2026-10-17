# Runtime migration provenance

The Lean backend used to splice fixed runtime text into every generated
module that used it (SPEC.md §17.12): the portable primitives, the ordered
collections, the model predicates, and the reasoning lemmas, each a Rust string
constant of `crates/lexlean/src/backend/semantic.rs`, written inside the
generated module's namespace, so that every module carried its own copy. For
language 1.2 they are now LexLean source: four native core modules of the
schema `lexlean/core-module/2` (SPEC.md §17.10) in the embedded project
`language/runtime-1.2/library/`, which a generated module imports, and the
Lean of every verification is generated from them by the compiler. Languages
1.0 and 1.1 keep their released bytes, held as data. This record is the
evidence that the migration lost nothing, in the manner of
`tests/preservation-library/MIGRATION.md` (#79).

## What was migrated

At repository commit `d284b165` (`main`), the language-1.2 runtime texts were:

| module | from | text bytes | declarations | DAG nodes | source bytes |
| --- | --- | ---: | ---: | ---: | ---: |
| `LexLeanRuntime.Portable` | `portable_runtime_1_2()`, the frozen portable runtime with every definition exposed | 13,491 | 176 | 4,750 | 248,405 |
| `LexLeanRuntime.Collections` | `COLLECTIONS_RUNTIME` | 8,016 | 105 | 3,106 | 178,383 |
| `LexLeanRuntime.Models` | `MODELS_RUNTIME` | 5,101 | 27 | 901 | 38,526 |
| `LexLeanRuntime.Reasoning` | `REASONING_RUNTIME` | 12,532 | 63 | 5,381 | 219,698 |
| total | | 39,140 | 371 | 14,138 | 685,012 |

`Reasoning` states its lemmas over the portable and collection runtimes, so it
imports `Portable` and `Collections`; the other three import only `Init`. The
code generator compiled 115 of the 371 declarations, which the data records.

## How

The one-time conversion is the oracle of checkpoint commit `34d940f8a164cc7b488233292ae27bef749303db`
(`tests/runtime-library/oracle/`, removed from the release tree by the commit
that follows it). It reuses the exporter, `flags.lean`, `probe.lean`,
`canon.py` and `compare.py` of the preservation library's migration, with the
runtime's module list:

1. `extract.py` lifts the four texts byte for byte out of `semantic.rs` into
   four Lean modules, each with the header and options a generated
   language-1.2 module carries (`portable_runtime_1_2` is not a literal: the
   tool replays its exposure of the frozen text exactly), and `build.sh`
   compiles them with the pinned Lean 4.32.1, silently.
2. `export.lean` writes every declaration each module owns with its kernel
   data, extension state, and the exact axioms `#print axioms` reports;
   `flags.lean` reads the compiled constants and the sparse `casesOn` records;
   `canon.py` writes the canonical sources, which are
   `language/runtime-1.2/library/src/*.lex.tex`.

The sources carry titles from the project glossary
`language/runtime-1.2/library/glossary/runtime.titles`.

## Equivalence

`equivalence.sh` of the oracle takes the Lean the compiler generated from the
committed native modules (`lexlean build` of the runtime project), compiles it
with the pinned Lean, exports the new environment with the same programs, and
converts it with the same `canon.py`; `compare.py` compares the result with the
committed sources declaration by declaration, as trees of kernel terms.

Result: the same 371 declarations in the same order in the same four modules.
368 rows are equal in every byte of their terms; 3 rows, exactly the sparse
`casesOn` constants of `Collections`, are equal up to the names of bound
variables (the elaborator's construction names its binders hygienically,
`else._@.….#N`), which no kernel judgment sees. The one other difference is the
private `Lean` import the decoder adds to each regenerated module, which SPEC.md
§17.10 allows. The attribute probe prints equal outputs (47 lines) for both
environments.

## The frozen bytes of languages 1.0 and 1.1

The released language-1.1 output splices `portable_runtime()` (13,339 bytes)
into a semantic module's namespace, and the released language-1.0 and 1.1
output splices the decoder of core module 1, `LEAN_CORE_DECODER` of
`backend/core.rs` (12,833 bytes), into every core module. Both were lifted byte
for byte into `language/runtime-1.2/frozen/portable-runtime-1.1.txt` and
`language/runtime-1.2/frozen/core-decoder-1.txt`, which the language-1.0 and
1.1 backends alone read. After the change `cargo xtask check-golden --write`
rewrites no file of the language-1.0 and 1.1 examples and fixtures, and
`conformance_rt_02` holds both texts equal to the released output.

## The proof libraries

Each generated language-1.2 module that used the runtime held its own copy:
`LexLeanTarget.TargetSemantics.LexLeanRuntime.index` and
`LexLeanTarget.RustSemantics.LexLeanRuntime.index` were distinct constants with
the same definition. The native preservation and certification libraries are
kernel data that names those copies, so their references had to follow the
runtime into `LexLeanRuntime.index`.

Moving the runtime out of the calculus modules changed more than the copies'
names. Lean shares a structurally equal matcher within a module but never with
an imported one (tested with the pinned Lean): `TargetSemantics.primitive` and
`RustSemantics.literal` had reused the matcher of their module's copy of
`checkedMultiplyInt64`, and now each has a matcher of its own, so the numbering
of their later matchers moved by two. The libraries name those matchers, their
equation lemmas, and their splitters.

So the renaming was derived, not assumed. `walk.py` compiled the six calculus
modules the libraries import (`TargetSyntax`, `TargetSemantics`, `RustSyntax`,
`RustSemantics`, `Gnaf`, `Certification`) as `main` generated them and as they
are generated now, exported both environments, paired every declaration whose
name is not an auxiliary's with its counterpart (a copy's `LexLeanTarget.<M>.<ns>.x`
with the runtime module's `<ns>.x`), and walked the two kernel types and values
in parallel, pairing each pair of differing constant names and then walking the
auxiliaries so paired, until nothing new was paired. Result: 3,914 declaration
pairs walked, every pair the same tree up to the names of bound variables and of
constants; 480 names paired, 463 of them by the runtime rule and 17 the
renumbered matchers (`primitive.match_7` … `match_27` to `match_9` … `match_29`,
`literal.match_7` … `match_17` to `match_9` … `match_19`). Two old names were
paired with two new names each: the copies' `checkedMultiplyInt64.match_1`,
which is `LexLeanRuntime.checkedMultiplyInt64.match_1` where the runtime
definition uses it and `primitive.match_7` (`literal.match_7`) where the
calculus definition reused it. Every library use of them is in `primitive`'s or
`literal`'s equation lemmas or in theorems that unfold them, so they were
renamed to the calculus definition's own matcher; the two are the same
definition, so the kernel would accept either.

`rename.py` then rewrote the names, and nothing else: every name of a constant
or projection node, a declaration's name, and its mutual group that the pairing
pairs, or that is derived from a paired name (an equation lemma, a splitter, a
private realization of either), with the runtime rule for the rest. The node
tables keep their numbering, so every row's kernel terms are its old terms with
names substituted, and binder names are kept as they are. It renamed 551 name
occurrences in 12 modules of the preservation library and 280 in 11 modules of
the certification library; no native source of the repository names a runtime
copy any more (`conformance_rt_01`).

The libraries are written in the core notation (§17.21.8), so the rename ran on
their data form: each source was converted to `\coredata` and back with
`lexlean::sdk::to_data_form` and `to_notation`, a round trip shown first to
change no byte of any of the 26 sources, and the renamed data printed as the
canonical notation. The same conversion wrote the four runtime sources, which
the migration produced as data, in the notation.

Two rows declare a name another library module declares:

- `LexLeanRuntime.equal.eq_1`, realized by `Primitives` on the `TargetSemantics`
  copy and by `RustBase` on the `RustSemantics` copy, which do not import each
  other. The renamed rows are the same declaration up to the names of bound
  variables. Lean realizes an equation lemma in every module that asks for it
  and imports the copies of two modules as one (tested with the pinned Lean, by
  elaboration and by `leanchecker` replay), so both rows are kept, and the
  library's check accepts a name declared twice exactly when it is an equation
  lemma and the two rows are the same declaration up to binder names
  (`production/preserve.rs`), auditing it once.
- `LexLeanRuntime.length.eq_1`, declared by `Primitives` and by the
  certification library's `Factor`, which already declared one name before the
  change, and are never imported together.

## Verification

The renamed libraries, the runtime modules, and the regenerated calculus
modules are what every verification now compiles, so the evidence is the
verifications themselves:

- `lexlean verify` of every language-1.2 example and of the compiler project
  compiles the runtime modules the project imports, replays them through
  `leanchecker`, and audits each of their declarations against its row's exact
  axioms; for a project with production roots it compiles the calculus modules
  and the renamed preservation library and checks certificates A, B and E of
  every root with exactly the three axioms. Their records under
  `expected/verify/` were regenerated.
- `conformance_gc_09` compiles and audits the renamed certification library.
- `conformance_rt_03` verifies the runtime's own project and refuses a row that
  pins another axiom set (`LLV7005`).
- The planted defects are recorded in `VERIFICATION.md`, each with the gate it
  turns red.

The verification of `examples/uor-atlas` (language 1.0, whose output and
records this change does not touch) needs more memory than the development host
has and runs in CI.

## The differential oracle

The pinned oracle (`lean4-prod`, SPEC.md §17.19) exports one module's closure
and does not follow a call into another module. With the runtime spliced into
each module, a collection template or a reasoner's iteration was the module's
own definition and the oracle lowered it, refusing most of them as
`escaping-local-function` or `opaque-lowering`; imported, a template Lean's
compiler keeps a call is `unresolved-external-call`. The oracle's answers on
the corpus changed accordingly, and the tables were rewritten to them, each
as a new version with its digest in `oracle/table-versions.toml`:

| Table | Version | Change |
| --- | --- | --- |
| `known-differences.toml` | 2 → 3 | roots move from `escaping-local-function` (KD-01) and `opaque-lowering` (KD-02) to `unresolved-external-call` (KD-03); `mapEntriesRoot` and `setElementsRoot`, which the oracle now generates, join `borrowed-parameter` (KD-09) and the new `output-buffer` (KD-13) |
| `known-non-overlap.toml` | 1 → 2 | the 17 map, set, and graph constructs and the 3 reasoning constructs are refused as `unresolved-external-call`; `map_entries` and `set_elements` leave the table, realized by a compiled root |

The oracle returns a list into a buffer its caller passes and answers how many
elements it wrote. The harness passes a buffer larger than any case writes and
compares the elements written; the convention is a contract difference
(KD-13), not a behavior difference. The corpus is 102 roots: the oracle
generates 32 (30 before) and refuses 70 (72); 25 crates compile (23) and run
on 1446 inputs (1411).

## The decoder of core module 2

`LEAN_CORE_DECODER_V2` (`backend/core_v2.rs`) is the Lean text every core
module of language 1.2 carries: it loads native data into the environment, so it
cannot itself be native data that it loads. It is the one Lean text of the
language-1.2 compiler not generated from a native module, held to that by
`conformance_rt_04`; printing it from a native module needs the lossless
kernel-term printer of #93 (stage 3c).

## Identities

Languages 1.0 and 1.1 are byte-identical: every file this change adds or
changes lies in the language-1.2 partition of the compiler-semantics ID
(`language/runtime-1.2/`, `language/preservation-1.2/`), and
`conformance_rp_10` recomputes all three identities from disk.

| language | compiler-semantics ID | change |
| --- | --- | --- |
| 1.0 | `95deb33a8d416d7bf60f02a36e251a71c3ee6f046b7e475d7bae2fc5ddc3767d` | none |
| 1.1 | `c56e1a234cee7c12f00e4f2f53d6094e2ed51f5bbf5195b60f79d4e8aa5bce53` | none |
| 1.2 | `974c98b59fc09a23c87bba2bd495f516ffbb7ae78085ea62b7e2580f6d01a290` → `a41110055179678ed946fba6370dad2f60f559f7b1bf536ddc6418a68c52e54d` | regenerated |

Every language-1.2 lock (229, the compiler-semantics ID substituted), golden,
fixture expectation, production-export package, generated calculus output, and
verified record was regenerated; the goldens and locks of languages 1.0 and 1.1
are unchanged.
