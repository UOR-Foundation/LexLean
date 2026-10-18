# Derived-row migration oracle

One-time, never a release input (SPEC.md §17.10). It converted the preservation
library's reserved-name and sparse `casesOn` rows to derived rows (`SM-34`) and
checked that the conversion lost nothing. The commit after the one that adds this
directory removes it; the record of what it established is
`tests/preservation-library/MIGRATION.md`.

It reuses the exporter, `flags.lean`, `probe.lean`, `canon.py`, and `compare.py`
of the checkpoint `d54dbf8551190e4ae77fbf69267fd4328e19db3f`
(`tests/preservation-library/oracle/`), unchanged: it adds no Lean.

1. `cargo run --bin convert -- <library src dir>` rewrites each module in place:
   a row whose name ends `eq_<n>`, `eq_def`, `splitter`, or `else_eq`, or that
   carries a sparse record, becomes a derived row (its universe parameters,
   type, value, reducibility metadata, and group are dropped), and the nodes
   nothing reaches are removed, the rest renumbered in order. It checks that
   every module was canonical first, that the converted module validates, and
   that its notation reads back to it.
2. `cargo run --bin todata -- <src dir> <out dir>` writes a source directory
   in the data form (`\coredata{…}`) that `compare.py` reads.
3. `equivalence.sh` compiles the Lean `lexlean build` generated from a library
   tree, exports its environment, and converts the export.
4. `compare.py <original data dir> <re-exported dir>` of the checkpoint compares
   the two declaration by declaration, as trees.
