# Certificate E as data: the statements are the text's

Certificate E of every production root was Lean text written by
`production/certificate.rs`. It is now kernel-term data: a native core module
of schema `lexlean/core-module/2` the compiler builds and the one core backend
renders (SPEC.md §17.17, `SP-14`; stage 1 of #89). This record is the evidence
that the change kept every statement: for each theorem of each data
certificate, the kernel term it states is definitionally the kernel term the
committed text certificate elaborated to, under the pinned Lean 4.32.1.

## How it was checked

At the branch's commit `1314d976`, before the verify records were regenerated,
the committed records of the four corpora still held the text certificates E
(`examples/<corpus>/expected/verify/preserve/LexLeanPreserve/C<hex>/R<i>/Compose/Rust*.lean`).
For each corpus the oracle:

1. built every root's certificates (A and B as text, E as data) with
   `repo_conformance::preservation::certificates`, staged them with the
   environment and the project's generated modules, and compiled and replayed
   every module with the pinned Lean (`preservation::check`), so that every
   data certificate E was in the environment under its own name;
2. compiled the committed text certificate E of the same root and target
   beside it, its namespace renamed from `<module>` to `<module>.Old` (the
   two lines `namespace <module>` and `end <module>`; every other name of the
   text is absolute or resolves inside the renamed namespace);
3. compiled an oracle module importing both, which, for every row of the data
   certificate (`__wt_k`, `__wtItems_n`, `__wtAux_n`, `root`), compares the
   type of `<module>.<row>` with the type of `<module>.Old.<row>` by
   `Lean.Meta.isDefEq` and fails on the first pair that differs:

```lean
import <module>
import <module>.Old
import Lean
open Lean Meta
run_cmd Lean.Elab.Command.liftTermElabM do
  let pairs : List (Name × Name) := [(`<module>.__wt_0, `<module>.Old.__wt_0), …, (`<module>.root, `<module>.Old.root)]
  for (new, old) in pairs do
    let n ← getConstInfo new
    let o ← getConstInfo old
    unless (← isDefEq n.type o.type) do
      throwError "{new}: the statement differs from {old}"
  logInfo m!"SP-14 oracle: {pairs.length} statements equal"
```

The oracle was the test `oracle_e_statements` in
`crates/conformance/src/preservation.rs` at that commit, run as
`cargo test -p repo-conformance --lib oracle_e_statements -- --ignored --nocapture`
on Linux x86-64 with the pinned toolchain; it was removed once this record was
written, as the library exporter was after `tests/preservation-library/MIGRATION.md`.

## Result

Oracle run on 2026-10-15 at commit `1314d976` (the branch before the records were regenerated), Lean 4.32.1, Linux x86-64: 76 certificates E, 131 theorem rows, every pair definitionally equal (`isDefEq`), 655 s.

| corpus | certificates E |
| --- | ---: |
| `production` | 10 |
| `production-coverage` | 49 |
| `models` | 9 |
| `reasoning` | 8 |

Every certificate, by module (the rows of a certificate are its `__wt_k`, `__wtItems_n`, `__wtAux_n`, and `root` theorems):

| corpus | certificate E | rows | oracle |
| --- | --- | ---: | --- |
| `production` | `C7b8829e2cc52192864282f3d66903769.R0.Compose.RustCore` | 1 | SP-14 oracle: 1 statements equal |
| `production` | `C7b8829e2cc52192864282f3d66903769.R0.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production` | `C7b8829e2cc52192864282f3d66903769.R1.Compose.RustCore` | 2 | SP-14 oracle: 2 statements equal |
| `production` | `C7b8829e2cc52192864282f3d66903769.R1.Compose.RustStd` | 2 | SP-14 oracle: 2 statements equal |
| `production` | `C7b8829e2cc52192864282f3d66903769.R2.Compose.RustCore` | 1 | SP-14 oracle: 1 statements equal |
| `production` | `C7b8829e2cc52192864282f3d66903769.R2.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production` | `C7b8829e2cc52192864282f3d66903769.R3.Compose.RustCore` | 1 | SP-14 oracle: 1 statements equal |
| `production` | `C7b8829e2cc52192864282f3d66903769.R3.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production` | `C7b8829e2cc52192864282f3d66903769.R4.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production` | `C7b8829e2cc52192864282f3d66903769.R5.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R0.Compose.RustStd` | 8 | SP-14 oracle: 8 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R1.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R2.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R3.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R4.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R5.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R6.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R7.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R8.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R9.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R10.Compose.RustCore` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R10.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R11.Compose.RustCore` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R11.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R12.Compose.RustCore` | 2 | SP-14 oracle: 2 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R12.Compose.RustStd` | 2 | SP-14 oracle: 2 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R13.Compose.RustStd` | 2 | SP-14 oracle: 2 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R14.Compose.RustCore` | 4 | SP-14 oracle: 4 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R14.Compose.RustStd` | 4 | SP-14 oracle: 4 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R15.Compose.RustCore` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R15.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R16.Compose.RustCore` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R16.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R17.Compose.RustCore` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R17.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R18.Compose.RustCore` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R18.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R19.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R20.Compose.RustCore` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R20.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R21.Compose.RustCore` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R21.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R22.Compose.RustCore` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R22.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R23.Compose.RustCore` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R23.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R24.Compose.RustCore` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R24.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R25.Compose.RustCore` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R25.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R26.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R27.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R28.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R29.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R30.Compose.RustStd` | 3 | SP-14 oracle: 3 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R31.Compose.RustStd` | 4 | SP-14 oracle: 4 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R32.Compose.RustCore` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R32.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R33.Compose.RustStd` | 2 | SP-14 oracle: 2 statements equal |
| `models` | `Ccca462bfe8cabc3365338af96b591362.R0.Compose.RustStd` | 2 | SP-14 oracle: 2 statements equal |
| `models` | `Ccca462bfe8cabc3365338af96b591362.R1.Compose.RustStd` | 2 | SP-14 oracle: 2 statements equal |
| `models` | `Ccca462bfe8cabc3365338af96b591362.R2.Compose.RustStd` | 3 | SP-14 oracle: 3 statements equal |
| `models` | `Ccca462bfe8cabc3365338af96b591362.R3.Compose.RustStd` | 2 | SP-14 oracle: 2 statements equal |
| `models` | `Ccca462bfe8cabc3365338af96b591362.R4.Compose.RustStd` | 2 | SP-14 oracle: 2 statements equal |
| `models` | `Ccca462bfe8cabc3365338af96b591362.R5.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `models` | `Ccca462bfe8cabc3365338af96b591362.R6.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `models` | `Ccca462bfe8cabc3365338af96b591362.R7.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `models` | `Ccca462bfe8cabc3365338af96b591362.R8.Compose.RustStd` | 1 | SP-14 oracle: 1 statements equal |
| `reasoning` | `C6d3e375f76cbc573b09fef98086539e0.R0.Compose.RustCore` | 4 | SP-14 oracle: 4 statements equal |
| `reasoning` | `C6d3e375f76cbc573b09fef98086539e0.R1.Compose.RustCore` | 4 | SP-14 oracle: 4 statements equal |
| `reasoning` | `C6d3e375f76cbc573b09fef98086539e0.R2.Compose.RustStd` | 6 | SP-14 oracle: 6 statements equal |
| `reasoning` | `C6d3e375f76cbc573b09fef98086539e0.R3.Compose.RustStd` | 5 | SP-14 oracle: 5 statements equal |
| `reasoning` | `C6d3e375f76cbc573b09fef98086539e0.R4.Compose.RustStd` | 3 | SP-14 oracle: 3 statements equal |
| `reasoning` | `C6d3e375f76cbc573b09fef98086539e0.R5.Compose.RustCore` | 4 | SP-14 oracle: 4 statements equal |
| `reasoning` | `C6d3e375f76cbc573b09fef98086539e0.R6.Compose.RustStd` | 5 | SP-14 oracle: 5 statements equal |
| `reasoning` | `C6d3e375f76cbc573b09fef98086539e0.R7.Compose.RustStd` | 2 | SP-14 oracle: 2 statements equal |

Every statement of every certificate E is the statement the text stated. What
changed is the proof term (now written with every argument, by the data
generator) and the form of the module (kernel data rendered by the decoder, in
place of Lean source text); what a verification establishes of a root did not.
