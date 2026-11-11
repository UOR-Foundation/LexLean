# Decision-regions oracle

One-time, never a release input (SPEC.md §17.12, *Decision regions*). It
exports `Regions.lean`, the authored Lean of the decision-region checker and
its soundness theorem, into the native `lexlean/core-module/2` source
`language/runtime-1.2/library/src/Regions.lex.tex`, and checks that the Lean
the compiler generates from that source is the same environment. The commit
after the one that adds this directory removes it; the record of what it
established is `tests/runtime-library/REGIONS.md`.

Inputs: a checkout with `lexlean` built (`target/debug/lexlean`), the
`to-notation` helper of `notation/` built into the same target directory, the
pinned Lean 4.32.1 under `~/.elan`, and Python 3. The exporter, `flags.lean`,
`canon.py`, and `compare.py` are those of the fold-law oracle (checkpoint
`b58f1952`), with the module list narrowed to `Regions` and the files read as
UTF-8. Language 1.2 no longer admits the data form, so the notation is printed
by `lexlean::sdk::to_notation` (the helper) rather than by `lexlean fmt`.

`run.sh <repository root> <work dir>` runs the steps that `REGIONS.md`
describes and prints the comparison.
