# Certificate A as data: the statements are the text's

Certificate A of every production root was Lean text written by
`production/certificate.rs`. It is now kernel-term data: a native core module of
schema `lexlean/core-module/2` the compiler builds and the one core backend
renders (SPEC.md §17.17, `SP-15`; stage 3 of #89). This record is the evidence
that the change kept every statement: for each row of each data certificate, the
kernel term it states is definitionally the kernel term the committed text
certificate elaborated to, under the pinned Lean 4.32.1.

## How it was checked

At the branch commit `d9c6655e`, before the verify records were regenerated, the
committed records of the four corpora still held the text certificates A
(`examples/<corpus>/expected/verify/preserve/LexLeanPreserve/C<hex>/R<i>.lean`).
For each corpus the oracle (`dev_oracle_a` in `crates/conformance/src/preservation.rs`):

1. built every root's certificate A as data, staged it with the environment and
   the project's generated modules, and compiled and replayed it with the pinned
   Lean (`preservation::check`);
2. compiled the committed text certificate A of the same root beside it, its
   namespace renamed from `<module>` to `<module>.Old` (the `namespace` and `end`
   lines, and the absolute `_root_.<module>.` references);
3. compiled an oracle module importing both which, for every row of the data
   certificate, compares the type of `<module>.<row>` (its own names read as the
   old module's) with the type of `<module>.Old.<row>` by the kernel's
   definitional equality (`Lean.Kernel.isDefEq`), and fails on the first pair
   that differs. The kernel's equality is used because it is the one verification
   relies on: `Lean.Meta.isDefEq` does not unfold a `match` to the eliminator the
   data writes, so it refuses `match l with | [] => a | x :: _ => b` against
   `List.casesOn l a fun x _ => b`, which the kernel identifies.
4. compared, and counted without requiring, the bodies of the definitions: a
   recursive width predicate is a different, separately proved definition.

Command: `LEXLEAN_EXAMPLE=<corpus> cargo test -p repo-conformance --lib dev_oracle_a -- --ignored --nocapture`
on Linux x86-64 with the pinned toolchain. The oracle ran once, while the
committed records still held the text certificates, and was removed from the
tree with the developer lenses beside it (its source is at commit `7653ed1e`),
as the oracles of certificates E and B were: the repository carries no ignored
test (R4, `RP-12`), and this record is the evidence.

## Result

Every statement of every certificate A is the statement the text stated:
57 certificates A, 1326 rows, every pair definitionally equal; definition
bodies equal 696 of 739.

| corpus | certificates A | rows | definition bodies equal |
| --- | ---: | ---: | ---: |
| `production` | 6 | 45 | 25 of 27 |
| `production-coverage` | 34 | 479 | 246 of 287 |
| `models` | 9 | 243 | 129 of 129 |
| `reasoning` | 8 | 559 | 296 of 296 |

Every certificate, by root:

| corpus | root | rows equal | definition bodies equal |
| --- | --- | ---: | ---: |
| `production` | `C7b8829e2cc52192864282f3d66903769.R0` | 5 | 3 of 3 |
| `production` | `C7b8829e2cc52192864282f3d66903769.R1` | 8 | 5 of 5 |
| `production` | `C7b8829e2cc52192864282f3d66903769.R2` | 9 | 5 of 5 |
| `production` | `C7b8829e2cc52192864282f3d66903769.R3` | 8 | 4 of 5 |
| `production` | `C7b8829e2cc52192864282f3d66903769.R4` | 8 | 4 of 5 |
| `production` | `C7b8829e2cc52192864282f3d66903769.R5` | 7 | 4 of 4 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R0` | 105 | 44 of 62 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R1` | 20 | 11 of 11 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R2` | 22 | 12 of 12 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R3` | 16 | 9 of 9 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R4` | 20 | 11 of 11 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R5` | 9 | 5 of 5 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R6` | 20 | 11 of 11 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R7` | 20 | 11 of 11 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R8` | 40 | 21 of 21 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R9` | 5 | 3 of 3 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R10` | 5 | 3 of 3 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R11` | 5 | 3 of 3 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R12` | 8 | 5 of 5 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R13` | 10 | 5 of 7 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R14` | 12 | 8 of 8 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R15` | 5 | 3 of 3 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R16` | 5 | 3 of 3 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R17` | 5 | 3 of 3 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R18` | 13 | 7 of 7 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R19` | 14 | 6 of 9 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R20` | 11 | 5 of 7 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R21` | 5 | 3 of 3 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R22` | 5 | 3 of 3 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R23` | 5 | 3 of 3 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R24` | 5 | 3 of 3 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R25` | 5 | 3 of 3 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R26` | 5 | 3 of 3 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R27` | 5 | 3 of 3 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R28` | 5 | 3 of 3 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R29` | 5 | 3 of 3 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R30` | 16 | 8 of 12 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R31` | 21 | 10 of 16 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R32` | 14 | 6 of 9 |
| `production-coverage` | `C6cddd56cf041af764eeefdf9e95c897e.R33` | 13 | 6 of 9 |
| `models` | `Ccca462bfe8cabc3365338af96b591362.R0` | 46 | 24 of 24 |
| `models` | `Ccca462bfe8cabc3365338af96b591362.R1` | 50 | 26 of 26 |
| `models` | `Ccca462bfe8cabc3365338af96b591362.R2` | 43 | 23 of 23 |
| `models` | `Ccca462bfe8cabc3365338af96b591362.R3` | 26 | 14 of 14 |
| `models` | `Ccca462bfe8cabc3365338af96b591362.R4` | 14 | 8 of 8 |
| `models` | `Ccca462bfe8cabc3365338af96b591362.R5` | 13 | 7 of 7 |
| `models` | `Ccca462bfe8cabc3365338af96b591362.R6` | 15 | 8 of 8 |
| `models` | `Ccca462bfe8cabc3365338af96b591362.R7` | 13 | 7 of 7 |
| `models` | `Ccca462bfe8cabc3365338af96b591362.R8` | 23 | 12 of 12 |
| `reasoning` | `C6d3e375f76cbc573b09fef98086539e0.R0` | 86 | 45 of 45 |
| `reasoning` | `C6d3e375f76cbc573b09fef98086539e0.R1` | 90 | 47 of 47 |
| `reasoning` | `C6d3e375f76cbc573b09fef98086539e0.R2` | 92 | 49 of 49 |
| `reasoning` | `C6d3e375f76cbc573b09fef98086539e0.R3` | 75 | 40 of 40 |
| `reasoning` | `C6d3e375f76cbc573b09fef98086539e0.R4` | 29 | 16 of 16 |
| `reasoning` | `C6d3e375f76cbc573b09fef98086539e0.R5` | 84 | 44 of 44 |
| `reasoning` | `C6d3e375f76cbc573b09fef98086539e0.R6` | 61 | 33 of 33 |
| `reasoning` | `C6d3e375f76cbc573b09fef98086539e0.R7` | 42 | 22 of 22 |

What changed is the proof term (now written with every argument, by the data
generator), the form of the module (kernel data rendered by the decoder, in place
of Lean source text), and the form of a recursive width predicate and relation
theorem (the recursor, or `WellFounded.fix`, in place of a compiled recursive
definition); what a verification establishes of a root did not.
