# Text-laws migration provenance

The tokenizer's universal laws (SPEC.md §17.12, `LM-01`, `LM-02`, `LM-13`) were
a hand-written Lean module. They are LexLean source now: one native core module
of the schema `lexlean/core-module/2` (SPEC.md §17.10), and the Lean the
compiler generates from it is what a module that declares a tokenizer imports.
This record is the evidence that the migration lost nothing, in the manner of
`tests/preservation-library/MIGRATION.md` and
`examples/uor-atlas/MIGRATION.md`.

## What was migrated

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
