# Loop repair oracle

One-time, never a release input (SPEC.md §17.10). It repaired the authored
declarations of the preservation library that adding `Expr.loop` to `RustSyntax`
(`SP-19`) leaves a case short, and it was used to find them. The commit after the
one that adds this directory removes it; the record of what it established is
`tests/preservation-library/MIGRATION.md`. It adds no Lean file: the one lemma
that needed a new proof, `stable_loop`, was written in Lean in a scratch directory,
compiled over the new machine modules, exported with the exporter of checkpoint
`d54dbf85` unchanged, and its text is recorded in MIGRATION.md.

1. `measure.py <generated Lean dir>` replaces the decoder's refusal of a declaration
   with a warning and an axiom of the declaration's stated type, so that `libcheck.sh`
   reports every declaration that breaks in one pass and not the first of each module.
2. `cargo run --bin arm -- <module> <donor>:<declaration>…`: every `Expr.casesOn` of
   the declaration gets an arm for `loop`, the arm for the donor constructor with its
   field replaced by `loop`'s two; it refuses a donor whose proof uses the field
   other than through the constructor.
3. `cargo run --bin matcher -- <module> <donor>`: the matcher `rc_litValue.match_1`
   gets an alternative for `loop` (its type, its value, its arm of `Expr.casesOn`, its
   matcher record) and each application of it the alternative to pass.
4. `cargo run --bin loopstable -- <RustBase> <exported stable_loop module> <ihE>`:
   grafts `stable_loop` and the derived row of `eval`'s equation for `loop` before
   `stable`, and gives `stable`'s `Expr.casesOn` the arm for `loop`, made from the
   arm for `block` with its proof replaced by an application of the lemma.
5. `sites`, `show`, `users` inspect terms.
