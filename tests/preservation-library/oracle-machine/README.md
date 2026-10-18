# Machine-callee repair oracle

One-time, never a release input (SPEC.md §17.10). It repaired the five authored
declarations of the preservation library that adding `Callee.machine` and
`Lit.overflow` to `RustSyntax` (`SP-18`) leaves a case short, and it was used to
find them. The commit after the one that adds this directory removes it; the
record of what it established is `tests/preservation-library/MIGRATION.md`.

Inputs: the library sources of the commit before the change, the machine modules
generated from `compiler/src` (`lexlean build` in a copy of `compiler/`), the pinned Lean
`4.32.1`. It adds no Lean: the edits are to kernel terms.

1. `cargo run --bin sites -- <module> <declaration> <constant>` lists the
   applications of a constant in a declaration's type and value, by node and
   number of arguments, and `cargo run --bin show -- <module> <declaration> <depth>`
   (or `@node <n> <depth>`, `@arg <n> <k> <depth>`) prints a term.
2. `libcheck.sh` compiles the library over the new machine modules and reports
   which module and declaration fails.
3. `cargo run --bin patch -- <module> <declaration>…` edits the named declarations'
   values: every application of the matcher `eval.match_3` with its four arguments
   gets a fifth, `fun primitive => runMachine primitive <operands>`, the operands
   those of its alternative for `Callee.runtime`; every `Callee.casesOn` with two
   arms gets a third, the arm for `runtime` over a primitive in place of an item
   (the constructor `Callee.runtime` read as `Callee.machine`); every `Lit.casesOn`
   with eight arms gets a ninth, the arm for `unit` with `Lit.overflow` for
   `Lit.unit`. It copies the value, appends the edited nodes, removes what nothing
   reaches, renumbers, and writes the notation after checking that it reads back.
   A declaration's statement is never edited.
