# Fold-law oracle

One-time, never a release input (SPEC.md §17.12, *Fold laws*). It exports
`Laws.lean`, the authored Lean of the fold laws, into the native
`lexlean/core-module/2` source `language/runtime-1.2/library/src/Laws.lex.tex`,
and checks that the Lean the compiler generates from that source is the same
environment. The commit after the one that adds this directory removes it; the
record of what it established is `tests/runtime-library/LAWS.md`.

Inputs: a checkout with `lexlean` built (`target/debug/lexlean`), the pinned
Lean 4.32.1 under `~/.elan`, and Python 3. The exporter, `flags.lean`,
`canon.py`, and `compare.py` are those of the runtime migration's oracle
(checkpoint `34d940f8`), with the module list narrowed to `Laws`, and
`compare.py` naming each row that is equal only up to binder names.

`run.sh <repository root> <work dir>` runs the three steps that `LAWS.md`
describes and prints the comparison.
