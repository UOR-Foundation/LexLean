# Array oracle

One-time, never a release input. It exports `Arrays.lean`, the authored Lean
of the array instances, primitives, and laws, into the native
`lexlean/core-module/2` source
`language/runtime-1.2/library/src/Arrays.lex.tex`, and checks that the Lean
the compiler generates from that source is the same environment. The commit
after the one that adds this directory removes it; the record of what it
established is `tests/runtime-library/ARRAYS.md`.

Inputs: a checkout with `lexlean` built (`target/debug/lexlean`), the
`to-notation` helper of `notation/` built into the same target directory, the
pinned Lean 4.32.1 under `~/.elan`, and Python 3. The environment variables
`LEXLEAN`, `NOTATION`, and `PYTHON` name other builds of the three programs.
The exporter, `flags.lean`, `canon.py`, and `compare.py` are those of the
fold-law oracle (checkpoint `b58f1952`), with the module list narrowed to
`Arrays` and its one runtime import, `Portable`, whose generated Lean is
compiled first, and with the files read and written as UTF-8. Language 1.2
does not admit the data form, so the notation is printed by
`lexlean::sdk::to_notation` (the helper) rather than by `lexlean fmt`.

`run.sh <repository root> <work dir>` runs the steps that `ARRAYS.md`
describes and prints the comparison.
