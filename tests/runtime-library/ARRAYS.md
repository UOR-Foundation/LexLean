# Array provenance

`LexLeanRuntime.Arrays` (SPEC.md §17.12) is the sixth native module of the
runtime project `language/runtime-1.2/library/`: the instances that let an
array answer the portable `length` and `index`, the three array primitives,
and the laws a `law` proof applies to them. Like the modules before it
(`tests/runtime-library/MIGRATION.md`, `LAWS.md`), it is LexLean source,
schema `lexlean/core-module/2` in the core notation (§17.21.8). This record is
the evidence that the source states what was proved, and that the Lean the
compiler generates from it is the same environment.

## What the module states

15 declarations in the namespace `LexLeanArrays`, over 604 nodes, in a source
of 15,451 bytes that imports `Init` and `LexLeanRuntime.Portable`:

- two instances: `arrayLengthable`, `LexLeanRuntime.Lengthable (Array α)` with
  `Array.size`, and `arrayIndexable`, `LexLeanRuntime.Indexable (Array α) α`
  with `value[offset]?`;
- three definitions: `fromList` (`List.toArray`), `toList` (`Array.toList`),
  and `set` (`Array.setIfInBounds`, which leaves the array as it is when the
  offset is not below its size);
- ten laws, which say that an array is the list of its elements:
  `to_list_from_list` and `from_list_to_list` (the two conversions are
  inverse), `length_to_list` (an array has the length of its list),
  `list_index` (the portable `index` of a list is `values[offset]?`),
  `index_to_list` (an array is indexed as its list is), `index_beyond` (no
  element at or past the length), `length_set` (`set` keeps the length),
  `index_set_same` (within the length, the offset written holds the element
  written), `index_set_other` (an offset that `Nat.beq` tells apart from the
  one written is unchanged), and `set_beyond` (`set` at or past the length is
  the identity).

There are no auxiliary rows: Lean realized no equation lemma, matcher,
splitter, or private proof term in the module. The code generator compiled
the two instances and the three definitions, which the data records.

Every row pins its exact axioms. The instances, the definitions, and the
three laws proved by `rfl` (`to_list_from_list`, `from_list_to_list`,
`length_to_list`) pin none. The other seven laws pin exactly `propext`. No
row depends on `Quot.sound` or `Classical.choice`. The one proof by cases,
`list_index`, is written with `induction … with` and `cases … with`, never by
recursive pattern matching in a theorem. The decoder of core module 2 rebuilds
a matcher's equation lemmas from the elaborator's construction and refuses one
it cannot rebuild; this module gives it no matcher to rebuild, and the
authored Lean was exported as written, with no change.

## How

The one-time oracle is in the checkpoint commit that adds this record
(`tests/runtime-library/arrays-oracle/`); the commit after it removes it. Its
exporter, `flags.lean`, `canon.py`, and `compare.py` are the fold-law oracle's
(`b58f1952`), narrowed to `Arrays` and its one runtime import, with the files
read and written as UTF-8. `run.sh <repository> <work>`, run on
`feat/verified-arrays` over `aaf84550` with the pinned Lean 4.32.1 on Windows
x86-64:

1. `lexlean build` of a copy of the runtime project, narrowed to `Portable`,
   generates the Lean of `Portable`. `Arrays.lean` is compiled against it with
   the pinned Lean, silently.
2. `flags.lean` and `export.lean` export the `Arrays` environment; `canon.py`
   writes its data form, and `lexlean::sdk::to_notation` prints it in the core
   notation, which is the committed
   `language/runtime-1.2/library/src/Arrays.lex.tex`.
3. `lexlean fmt --check` holds that source canonical ("6 modules are
   canonical"), and `lexlean build` of the runtime project generates its Lean
   ("built 6 modules"). The `Portable` generated beside it is the one
   `Arrays.lean` was compiled against, byte for byte.
4. That Lean is compiled, exported, and converted by the same programs, and
   `compare.py` compares the two declaration by declaration, as trees of
   kernel terms.

## Equivalence

Result: the same 15 declarations in the same order, every one equal in every
byte of its terms (`declarations=15 equal=15 equal-up-to-binder-names=0`, 604
nodes on both sides). No row is equal only up to the names of bound variables:
the module has no sparse `casesOn`.

The one difference is the import list, and `compare.py` reports it: the
regenerated module also privately imports `Lean`,
`Lean.Meta.Constructions.SparseCasesOn`, `Lean.Compiler.LCNF.Main`, and
`Lean.Compiler.LCNF.PhaseExt`, the imports of the decoder a regenerated module
carries, which SPEC.md §17.10 allows. `compare.py` excuses only the first of
the four, as the fold-law oracle's did, so its row for the module ends in
`DIFFERS` with the one reason `imports`, and it exits nonzero.

## C output

A build that emits C for every module compiles this one. The pinned `lean`,
given the generated `LexLeanRuntime/Arrays.lean` with both `-o` and `-c` and
the regenerated `Portable` on its path, ended successfully in about 16 s and
wrote 1,458,216 bytes of C. The generated `Portable.lean`, the control, took
about 15 s and wrote 1,641,701 bytes.

## Verification

- `RT-01` (`conformance_rt_01`) checks that every module of the registry
  `language/runtime-1.2/library.toml`, `Arrays` now among them, is one native
  core module of the runtime's own project, listed in dependency order, that
  every declaration pins its exact axioms, and that a source which is missing,
  weakened to an inexact policy, or imports anything but `Init` and an earlier
  runtime module is refused.
- `RT-03` (`conformance_rt_03`) checks that a verification compiles every
  runtime module a project imports before the project's own modules, replays
  it with `leanchecker`, and audits each of its declarations against the
  axioms its row pins, and that the runtime's own project, `Arrays` included,
  verifies under the pinned Lean.
- `SM-39` (`conformance_sm_39`) is the register row of the language-1.2
  arrays this module is the runtime of; its statement in `model/ids.toml`
  says what it checks.

This record states what the oracle observed. It does not state the result of
any of the three rows: `just vv` does.
