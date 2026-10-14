# Text-laws oracle (one-time; outside the release tree)

This directory is the one-time exporter that converts the elaborated environment
of the hand-written text laws into native `lexlean/core-module/2` data, exactly
as `tests/preservation-library/oracle/` did for the preservation library (PR #80)
and `examples/uor-atlas/` before it. It is **not** a release input: it is
recoverable from this commit and removed by the commit that follows.

## Inputs

- `src/LexLeanText.lean` — the text laws, authored once, proved under
  `leanprover/lean4:v4.32.1` (no `sorry`, no `admit`, no new axiom): `encode`,
  `decode`, `roundtrip`, `in_range`, `utf8Encode`, `utf8Decode`,
  `utf8_roundtrip`, `encodeText`, `decodeText`, `text_roundtrip` (LM-01, LM-02).

## Programs

- `build.sh` — compiles `src/LexLeanText.lean` to `out/LexLeanText.olean`.
- `export.lean` — walks the elaborated environment and writes every declaration
  of the module as a child-before-parent DAG: kind, universe parameters,
  reducibility hints and transparency, tags, matcher, projection, no-confusion,
  simplification, equation-record, inlining, sparse-`casesOn`, `grind`, and the
  exact axioms `#print axioms` prints.
- `flags.lean` — the two facts `export.lean` cannot read: which constants the
  code generator compiled, and which `casesOn` constants are sparse.
- `canon.py` — writes the data as canonical JSON of `lexlean/core-module/2` and
  wraps each module as a `.lex.tex` source.
- `probe.lean` — asks the pinned Lean for the attribute state of every constant
  (inlining, specialize/nospecialize, noncomputable, matcher, sparse
  `casesOn`, `grind`), so the state can be compared across environments.

## Pipeline

```
sh build.sh
(cd flags && LEAN_PATH=$PWD/../out lean --run ../flags.lean)
EXPORT_DIR=export LEAN_PATH=$PWD/out lean --run export.lean
FLAGS=$PWD/flags EXPORT=$PWD/export python3 canon.py lextex
```

## Equivalence (the migration evidence)

The Lean the compiler generates from the committed native data is compiled by
the pinned Lean, exported again with the same `export.lean` and `flags.lean`,
and converted with the same `canon.py`. Result: the same 17 declarations in the
same order; 824 DAG nodes and 699 proof nodes equal; every declaration row equal
in kind, universe parameters, type, value, reducibility hints and transparency,
tags, inlining, compiled flag, and exact axioms. The single difference is the
private `Lean` import the generic decoder runtime adds to the regenerated module
(SPEC.md §17.10 allows it). The attribute probe prints 17 identical lines for
both environments.
