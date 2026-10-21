# Text-laws migration provenance

The tokenizer's universal laws (SPEC.md §17.12, `LM-01`, `LM-02`, `LM-13`) were
a hand-written Lean module. They are LexLean source now: one native core module
of the schema `lexlean/core-module/2` (SPEC.md §17.10), and the Lean the
compiler generates from it is what a module that declares a tokenizer imports.
This record is the evidence that the migration lost nothing, in the manner of
`tests/preservation-library/MIGRATION.md` and
`examples/uor-atlas/MIGRATION.md`.

## What was migrated

This record describes the module as it was first migrated, at commit
`b4be399a6`: the counts below (17 declarations, 824 DAG nodes) are the first
oracle's own output, and running its `export.lean` and `flags.lean` from
commit `54594e1a0` under `leanprover/lean4:v4.32.1` prints `LexLeanText:
decls=17 nodes=824` and `17 7` again. They are not the committed module's
counts: slice 1 went on to extend the module (the vocabulary and piece
functions `idBound` and `pieceBytes`, `Good`, `allBelow`, and the private
names of #91), and the module on `main` holds 118 declarations, which is what
the pipeline reports for its oracle source in the second record below. The
equivalence of the committed module is the second record's, not this one's.

`LexLeanText` — the text laws — authored once at
`tests/text-laws/oracle/src/LexLeanText.lean` (recoverable from checkpoint
commit `54594e1a0`), proved under `leanprover/lean4:v4.32.1` with no `sorry`, no
`admit` and no new axiom:

```
'LexLeanText.roundtrip' depends on axioms: [propext, Quot.sound]
'LexLeanText.in_range' depends on axioms: [propext, Quot.sound]
'LexLeanText.utf8_roundtrip' depends on axioms: [propext, Quot.sound, Classical.choice]
'LexLeanText.text_roundtrip' depends on axioms: [propext, Quot.sound, Classical.choice]
```

(`propext`, `Quot.sound` and `Classical.choice` are Lean's own standard axioms,
not new ones, and the exporter records exactly these per declaration in
`policy`.)

The 17 declarations, in the exporter's order: `Bytes` (`ByteArray`), `Token`
(`UInt32`), `byteVocab` (256) with its equation lemma `byteVocab.eq_1`,
`InVocab`, `byte_lt_byteVocab`, `decode`, `utf8Decode`, `decodeText`, `encode`
with its equation lemma `encode.eq_1`, `utf8Encode`, `encodeText`, `in_range`,
`roundtrip`, `utf8_roundtrip`, `text_roundtrip`. The exported document holds
824 DAG nodes, 699 of them proof nodes, and its `imports` are `["Init"]`.

The laws are the three the register names: `roundtrip` (`decode (encode b) =
some b` for every byte string `b`), its `text_roundtrip` counterpart through
`utf8Encode`/`utf8Decode`, and `in_range` (every token a decode produces
indexes the vocabulary); `utf8_roundtrip` is the UTF-8 law the byte-level
scheme needs.

## How

The one-time conversion is the oracle of checkpoint commit `54594e1a0`
(`tests/text-laws/oracle/`, removed from the release tree by `55ad6ec95`):
`build.sh`, `export.lean`, `flags.lean`, `probe.lean`, `canon.py`. `export.lean`
walks every declaration the module owns and writes its kernel type and value as
a child-before-parent DAG, with its universe parameters, reducibility hints and
transparency, the structure, class, instance and projection registrations, the
no-confusion, matcher, simplification, equation-record, inlining,
sparse-`casesOn` and `grind` state, and the exact axioms `#print axioms`
reports. `flags.lean` reads the two facts the exporter cannot (which constants
the code generator compiled, and which `casesOn` constants are sparse).
`probe.lean` asks the pinned Lean for the attribute state of both environments.
`canon.py` writes the data as canonical JSON of `lexlean/core-module/2` and
wraps the module as a `.lex.tex` source.

The source carries a title from the project glossary
`language/text-1.2/library/glossary/text.titles`, so the library project passes
the closed-lexicon checks like any project. The module
(`language/text-1.2/library/src/LexLeanText.lex.tex`) is the exporter's output
re-pointed at that glossary, which differs from the canonical exporter output of
the same run only in that header line. It is committed in the authoring notation
(SPEC.md §17.21), written by `lexlean fmt` from that data: the notation prints
the same module, and `AU-04` requires it of every language-1.2 source.

## Equivalence

The Lean the compiler generated from the committed native data was compiled by
the pinned Lean 4.32.1, exported again with the same `export.lean` and
`flags.lean`, probed with the same `probe.lean`, and converted with the same
`canon.py`. The pipeline's stdout:

```
flags.lean  -> "81 35"
export.lean -> "LexLeanText: decls=17 nodes=824"
canon.py    -> "LexLeanText 32203"
```

Result of the structured comparison of the two native documents:

- the same 17 declarations, in the same order;
- 824 DAG nodes equal, and 699 proof nodes equal — every declaration's type and
  value equal as trees of kernel terms (rows are compared by the terms they
  root, the node tables being numbered in the order nodes were met);
- every declaration row equal in kind, universe parameters, reducibility hints
  and transparency, tags, inlining, compiled flag, and exact axioms;
- the attribute probe (inlining kind, specialize/nospecialize,
  noncomputability, matcher information, the sparse-`casesOn` record, `grind`
  registration), asked of the pinned Lean over both environments, prints two
  equal outputs (17 lines each, `diff` rc=0, zero bytes).

So every declaration is equal in types, values, reducibility and attributes.

### The one difference, stated plainly

Two differences are recorded, and neither is a lost declaration:

1. **The private `Lean` import.** The regenerated module's `imports` are
   `["Init", "Lean"]` where the original's are `["Init"]`. The extra import is
   the generic decoder runtime's, which the compiler adds to every generated
   core module, and SPEC.md §17.10 allows it — it is the same difference the
   preservation migration records.
2. **The raw probe lists `Init` twice.** The raw probe of the regenerated
   environment prints the import list with `Init` repeated, because the module
   both declares `public import Init` and inherits it. `canon.py` line 96
   dedupes the list with `dict.fromkeys` before it commits the artifact, so the
   committed data lists `["Init", "Lean"]`, each name once. The raw list is
   *not* what is committed and no declaration depends on the difference; it is
   recorded here because the raw probe output is evidence and a reader
   comparing the two probe files will see it.

## Registry

`language/text-1.2/library.toml` lists only the modules and the toolchain pin:
the compiler reads every declaration and its exact axioms from the native
module, never from a list. The list it reads is the one module,
`LexLeanText.LexLeanText`, and `LM-13` checks that the name the elaboration
imports is the name the registry lists and that no independently authored Lean
implementation of the laws exists in the release tree.

## Verification

- `elan run leanprover/lean4:v4.32.1 lean LexLeanText.lean` exits 0 with no
  message: no `sorry`, no `admit`, no new axiom.
- The regenerated environment equals the original, declaration by declaration,
  as recorded under **Equivalence** above.
- `conformance_lm_13` requires the module to be one native source of the
  embedded project, the registry to list only it, the source to be embedded in
  the compiler, and no Lean file of the laws to exist anywhere in the release
  tree.

## Identities

Every file this change adds lies in the language-1.2 partition
(`language/text-1.2/`), and `crates/lexlean/src/lib.rs` excludes that path from
the language 1.0 and 1.1 compiler-semantics IDs (`is_v1_2_file`), so those two
identities are byte-identical by construction and `conformance_rp_10`
recomputes them from disk. The language-1.2 compiler-semantics ID moves, so
every lock, golden, fixture expectation and verified record bound to it is
stale; that regeneration is the work of the slices that add the example, and
this slice leaves it to them so that one sweep pays for it once.

## Second record: context windows and generation (slice 2 of #30)

Slice 2 (`LM-04`..`LM-06`) adds the laws of context windows and text
generators to the same module, by the round trip of the first record. Nothing
in the release tree is hand-written Lean.

### What was added

The oracle source of the first record was extended outside the release tree
and proved under `leanprover/lean4:v4.32.1` with no `sorry`, no `admit`, and
no new axiom:

- **Windows:** `fits`, `keepLast`, `admitRefuse`, `admitKeepLast`,
  `pushRefuse`, `pushKeepLast`, `admit`, `push`, `fitsOption`, and
  `suffixOption`.
- **Window laws:** `admitRefuse_bounded`, `admitKeepLast_bounded`,
  `pushRefuse_bounded`, `pushKeepLast_bounded`, `admitRefuse_exact`,
  `admitKeepLast_exact`, `admitKeepLast_suffix`, and `admitRefuse_refuses`.
- **Generation:** `firstMax`, `xorshift32`, `weight`, `totalWeight`,
  `drawGo`, `proportional`, `chooseGreedy`, `chooseSample`, `guard`, `genGo`
  (structural in its fuel), `generateWith`, `generateGreedy`,
  `generateSample`, their `Guarded` forms, and `atMost`.
- **Generation laws:** `generateGreedy_bounded`, `generateSample_bounded`,
  `generateGreedy_guarded`, and `generateSample_guarded`.

The exact axioms `#print axioms` reports for the laws an elaboration names:

```
'LexLeanText.admitRefuse_bounded' does not depend on any axioms
'LexLeanText.admitKeepLast_bounded' depends on axioms: [propext, Quot.sound]
'LexLeanText.pushRefuse_bounded' does not depend on any axioms
'LexLeanText.pushKeepLast_bounded' depends on axioms: [propext, Quot.sound]
'LexLeanText.admitRefuse_exact' does not depend on any axioms
'LexLeanText.admitKeepLast_exact' depends on axioms: [propext, Quot.sound]
'LexLeanText.admitKeepLast_suffix' depends on axioms: [propext, Quot.sound]
'LexLeanText.admitRefuse_refuses' does not depend on any axioms
'LexLeanText.generateGreedy_bounded' depends on axioms: [propext, Quot.sound]
'LexLeanText.generateSample_bounded' depends on axioms: [propext, Quot.sound]
'LexLeanText.generateGreedy_guarded' depends on axioms: [propext, Quot.sound]
'LexLeanText.generateSample_guarded' depends on axioms: [propext, Quot.sound]
```

`xorshift32` evaluates to `270369`, `67634689`, `2647435461`, and `307599695`
from seed 1, which is Marsaglia's sequence for the 13, 17, 5 triple.

### The first record's pipeline, re-run

Before the extension, the first record's pipeline was run again on the
committed slice-1 oracle source:

```
export.lean -> "LexLeanText: decls=118 nodes=12570"
```

After `lexlean fmt`, its output is byte-identical to the module `main`
committed. The counts the first record prints (17 declarations, 824 nodes)
are not what this pipeline produces from that source today. The committed
module is what the pipeline produces.

### The extended export

```
flags.lean  -> "198 58"
export.lean -> "LexLeanText: decls=198 nodes=19448"
canon.py    -> "LexLeanText 799557"
```

The committed `language/text-1.2/library/src/LexLeanText.lex.tex` is that
output, written in the authoring notation by `lexlean fmt` in a locked scratch
copy of the library project.

### Equivalence

The Lean the compiler generates from the committed native data
(`text_laws::stage()`) was compiled by the pinned Lean as the module
`LexLeanText`. It was then exported again with the same `export.lean` and
`flags.lean`, converted with the same `canon.py`, and formatted with
`lexlean fmt`:

```
export.lean -> "LexLeanText: decls=198 nodes=19448"
```

Compared with the committed module, the regenerated module has:

- the same 198 declarations, in the same order;
- the same 19,448 DAG nodes;
- every row equal in kind, universe parameters, type, value, reducibility,
  transparency, tags, inlining, compiled flag, and exact axioms.

The attribute probe prints the same 198 lines for the library's constants in
both environments.

### Differences, stated plainly

1. **The private `Lean` import.** The regenerated module imports `["Init",
   "Lean"]`. This is the import of the generic decoder runtime, as in the
   first record. The decoder's private constants (`LexLeanCore.Runtime`) are
   also the 64 extra lines of the regenerated probe and the difference
   between the `flags.lean` counts (`262 86` against `198 58`). None of them
   is a declaration of the library.
2. **One hygienic binder name.** The `else` binder of one compiled matcher is
   `else._@.LexLeanText.#2792387140._hygCtx._hyg.#91` in the original and
   `else._@._hyg.#2` in the regenerated module. A binder name is not part of
   a kernel term's meaning (terms are compared up to renaming of bound
   variables).
3. **Layout.** Line breaks differ where the longer binder name was wrapped.

Apart from these, the two modules are equal.

## Third record: classifiers (slice 3 of #30)

Slice 3 (`LM-07`) adds the features of a text classifier and their laws to the
same module, by the round trip of the first record, in the way the second
record added windows and generation. The 198 declarations the module already
held are not re-exported: the additions are exported alone and joined to the
committed data, so the existing rows and nodes are the ones the earlier
records evidence. Nothing in the release tree is hand-written Lean.

### What was added

The additions were authored once as Lean outside the release tree, in a scratch
module `LexLeanText.Ext` that imports the library module the compiler generates
from the committed data, and proved under `leanprover/lean4:v4.32.1` with no
`sorry`, no `admit`, and no new axiom. Thirty-one declarations were exported,
among them eight the elaboration names:

- **Functions:** `features` (the count of each token id below a size, in
  order, over `countOf`), `fitWidth` (pad with zeros or truncate to a width),
  `total` (the sum of a list of integers), and `tokenCount`.
- **Laws:** `features_width`, `features_fit`, `features_total_text` (under
  `longest_match`), and `features_total_text_bytes` (under `byte_level`); both
  totals apply `features_total_of` to the tokenizer's `in_range`.
- **Support:** `countOf`, `hot`, `total_nil`, `total_cons`, `total_append`,
  `total_map_add`, `total_map_zero`, `total_one_hot`, `countOf_cons`,
  `features_total_of`, and the equation, `_proof_` and matcher declarations
  Lean generated for them.

The exact axioms `#print axioms` reports for the laws an elaboration names:

```
'LexLeanText.features_width' depends on axioms: [propext]
'LexLeanText.features_fit' depends on axioms: [propext, Quot.sound]
'LexLeanText.features_total_text' depends on axioms: [propext, Classical.choice, Quot.sound]
'LexLeanText.features_total_text_bytes' depends on axioms: [propext, Classical.choice, Quot.sound]
```

### How

```
lexlean build                       # in a locked scratch copy of the library project
lean -o LexLeanText/LexLeanText.olean   # the generated Lean of the committed module
lean -o LexLeanText/Ext.olean Ext.lean  # the additions, importing that module
lean --run flags.lean               # the second record's flags.lean, over LexLeanText.Ext
lean --run export.lean              # the second record's export.lean, libMods = [LexLeanText.Ext]
```

The exporter's output is converted with the same `canon.py` and joined to the
committed data by a scratch script that renumbers the additions' nodes into the
committed node table, reusing an existing node wherever a node is structurally
equal (so a shared `Nat` or `List` is one node), appends the thirty-one
declaration rows after the 198, and recomputes `proof_nodes` by the same
definition `canon.py` uses (the nodes the values of theorems reach and no type
reaches). `lexlean` then wrote the notation (`sdk::to_notation`), the form of
every language-1.2 source (`AU-04`).

### Equivalence

The committed module, read back by the compiler, has:

- 22,774 DAG nodes, of which the first 19,448 are equal, node for node, to the
  nodes of the committed module before this record;
- 229 declarations, of which the first 198 are equal, row for row (kind,
  universe parameters, type and value indices, reducibility, transparency,
  tags, inlining, compiled flag, and exact axioms), to the declarations before
  this record, in the same order;
- the same `imports`, `["Init"]`.

The Lean the compiler generates from the committed data was compiled by the
pinned Lean as `LexLeanText.LexLeanText` (the kernel replays every
declaration when the module is decoded), and `#print axioms` on the four laws
above printed the lines recorded above.

### Differences, stated plainly

1. **Twenty nodes are no longer proof data.** `proof_nodes` is derived: the
   nodes the values of theorems reach and no declaration's type reaches. The
   additions' types reach twenty nodes that only proofs reached before, so
   those twenty are written as ordinary nodes (without `proof(`) in the
   notation. No node, declaration, or value changed.
2. **The Lean source is not committed.** The additions' Lean is the scratch
   input of the round trip, as the first two records' was; the committed data
   is the module, and a change to it goes through the same round trip.

## Fourth record: the production bridge (slice 4 of #30)

Slice 4 (`LM-10`) realizes text components as ordinary executable language-1.2
definitions (`LexLeanStd.TextModels`, SPEC.md §17.12) and relates each to the
library function it replaces by a **uniqueness lemma** of the library: any
function that satisfies the helper's defining equations equals the helper. The
library names no generated constant, so the lemma takes the production form as
an argument, and an elaboration applies it to the production form with the
equations proved by unfolding. This record adds the first two, by the round
trip of the third record: authored once as Lean outside the release tree in the
scratch module `LexLeanText.Ext`, which imports the library module the compiler
generates from the committed data, proved under `leanprover/lean4:v4.32.1` with
no `sorry`, no `admit`, and no new axiom, exported alone with the first
record's `export.lean` and `flags.lean`, and joined to the committed data.

### What was added

- `withIndex_unique`: a function `f : List String -> Nat -> List (Nat x
  String)` with `f [] i = []` and `f (s :: ss) i = (i, s) :: f ss (i + 1)` is
  `withIndex`.
- `fitWidth_of`: for a `take` and a `zeros` that satisfy the equations of
  `List.take` and `List.replicate n 0`, `take n xs ++ zeros (n - xs.length)`
  is `fitWidth n xs`.

```
'LexLeanText.withIndex_unique' does not depend on any axioms
'LexLeanText.fitWidth_of' depends on axioms: [propext]
```

### Equivalence

The Lean the compiler generates from the committed data was compiled by the
pinned Lean as `LexLeanText.LexLeanText`, and `#print axioms` printed the lines
above. The export of the additions has 2 declarations and 656 nodes; joined by
the third record's method (nodes renumbered into the committed table, an
existing node reused wherever structurally equal, `proof_nodes` recomputed over
the whole table), the committed module holds 23,226 nodes and 231
declarations, of which the first 229 are equal to the declarations of the third
record. The same `imports`, `["Init"]`, are kept: the lemma statements name
only `Init` constants, so the library imports no runtime and a production form
is related to them by the elaboration, which unfolds the runtime's operations.

### Differences, stated plainly

1. **Proof data of existing nodes.** `proof_nodes` is derived, and the new
   statements reach nodes that only proofs reached before, so those nodes are
   written as ordinary nodes in the notation. No node, declaration, or value
   changed.
2. **The Lean source is not committed.** As in the earlier records, the
   additions' Lean is the scratch input of the round trip.
