# Certificate B as data: the statements are the text's

Certificate B of every production root was Lean text written by
`production/rust_cert.rs` and `production/rust_term.rs`. It is now kernel-term
data: a native core module of schema `lexlean/core-module/2` the compiler builds
(`production/certificate/data_b`) and the one core backend renders (SPEC.md
§17.17, `SP-16`; stage 2 of #89). This record is the evidence that the change
kept every statement: for each row of each data certificate, the kernel term it
states is definitionally the kernel term the committed text certificate
elaborated to, under the pinned Lean 4.32.1.

## How it was checked

At the branch commit `63871961` plus the generator (before the verify records
were regenerated), the committed records of the four corpora still held the text
certificates B
(`examples/<corpus>/expected/verify/preserve/LexLeanPreserve/C<hex>/R<i>/Rust{Core,Std}.lean`).
For each corpus the oracle (`dev_oracle_b` in `crates/conformance/src/preservation.rs`):

1. built every root's certificate B in each target as data, staged it with the
   environment, and compiled and replayed it with the pinned Lean
   (`preservation::check`), and ran the audit module over it
   (`preserve::audit`): every row prints exactly the axioms its policy pins;
2. compiled the committed text certificate B of the same root and target beside
   it, its namespace renamed from `<module>` to `<module>.Old` (the `namespace`
   and `end` lines; every other name of the text is absolute or resolves inside
   the renamed namespace);
3. compiled an oracle module importing both which, for every row of the data
   certificate (`program`, `krate`, `flags`, `fun<n>`, `crate_ok`, `root`),
   compares the type of `<module>.<row>` (its own names read as the old
   module's) with the type of `<module>.Old.<row>` by the kernel's definitional
   equality (`Lean.Kernel.isDefEq`), and fails on the first pair that differs;
4. compared, and counted, the bodies of the definitions (`program`, `krate`,
   `flags`) by the same relation.

```lean
import <module>
import <module>.Old
import Lean
open Lean Meta
def swapRoot (new old : Name) (e : Expr) : Expr := …  -- reads <module>.x as <module>.Old.x
def kdef (a b : Expr) : MetaM Bool := do
  ofExceptKernelException (Kernel.isDefEq (← getEnv) {} a b)
run_cmd Lean.Elab.Command.liftTermElabM do
  let pairs : List (Name × Name) := [(`<module>.program, `<module>.Old.program), …, (`<module>.root, `<module>.Old.root)]
  for (new, old) in pairs do
    …
    unless (← kdef (swapRoot `<module> `<module>.Old n.type) o.type) do
      throwError "{new}: the statement differs from {old}"
  logInfo m!"SP-16 oracle: {types} statements equal; bodies {same} of {total}"
```

Command: `LEXLEAN_EXAMPLE=<corpus> cargo test -p repo-conformance --lib dev_oracle_b -- --ignored --nocapture`
on Linux x86-64 with the pinned toolchain. The oracle ran once, before the text
generator of certificate B was deleted, and was removed from the tree with it, as
the oracles of certificates E and A were.

## Result

Oracle run on 2026-10-17 at commit `b29f0b9e` (the branch with the generator, before the verify records were regenerated), Lean 4.32.1, Linux x86-64: 76 certificates B, 984 rows, every pair definitionally equal (`Kernel.isDefEq`); definition bodies (`program`, `krate`, `flags`) equal 228 of 228.

| corpus | certificates B | rows | definition bodies equal |
| --- | ---: | ---: | ---: |
| `production` | 10 | 70 | 30 of 30 |
| `production-coverage` | 49 | 441 | 147 of 147 |
| `models` | 9 | 162 | 27 of 27 |
| `reasoning` | 8 | 311 | 24 of 24 |

Every certificate, by root and target:

| corpus | root | target | rows equal | definition bodies equal |
| --- | --- | --- | ---: | ---: |
| `production` | `C3950237c975b30f5dd9298b8e09919c9.R0` | rust-core | 6 | 3 of 3 |
| `production` | `C3950237c975b30f5dd9298b8e09919c9.R0` | rust-std | 6 | 3 of 3 |
| `production` | `C3950237c975b30f5dd9298b8e09919c9.R1` | rust-core | 7 | 3 of 3 |
| `production` | `C3950237c975b30f5dd9298b8e09919c9.R1` | rust-std | 7 | 3 of 3 |
| `production` | `C3950237c975b30f5dd9298b8e09919c9.R2` | rust-core | 8 | 3 of 3 |
| `production` | `C3950237c975b30f5dd9298b8e09919c9.R2` | rust-std | 8 | 3 of 3 |
| `production` | `C3950237c975b30f5dd9298b8e09919c9.R3` | rust-core | 7 | 3 of 3 |
| `production` | `C3950237c975b30f5dd9298b8e09919c9.R3` | rust-std | 7 | 3 of 3 |
| `production` | `C3950237c975b30f5dd9298b8e09919c9.R4` | rust-std | 7 | 3 of 3 |
| `production` | `C3950237c975b30f5dd9298b8e09919c9.R5` | rust-std | 7 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R0` | rust-std | 24 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R1` | rust-std | 16 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R2` | rust-std | 11 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R3` | rust-std | 17 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R4` | rust-std | 12 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R5` | rust-std | 10 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R6` | rust-std | 36 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R7` | rust-std | 21 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R8` | rust-std | 19 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R9` | rust-std | 6 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R10` | rust-core | 6 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R10` | rust-std | 6 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R11` | rust-core | 6 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R11` | rust-std | 6 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R12` | rust-core | 7 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R12` | rust-std | 7 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R13` | rust-std | 7 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R14` | rust-core | 8 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R14` | rust-std | 8 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R15` | rust-core | 6 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R15` | rust-std | 6 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R16` | rust-core | 6 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R16` | rust-std | 6 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R17` | rust-core | 6 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R17` | rust-std | 6 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R18` | rust-core | 10 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R18` | rust-std | 10 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R19` | rust-std | 9 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R20` | rust-core | 8 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R20` | rust-std | 8 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R21` | rust-core | 6 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R21` | rust-std | 6 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R22` | rust-core | 6 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R22` | rust-std | 6 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R23` | rust-core | 6 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R23` | rust-std | 6 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R24` | rust-core | 6 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R24` | rust-std | 6 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R25` | rust-core | 6 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R25` | rust-std | 6 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R26` | rust-std | 6 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R27` | rust-std | 6 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R28` | rust-std | 6 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R29` | rust-std | 6 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R30` | rust-std | 8 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R31` | rust-std | 9 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R32` | rust-core | 9 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R32` | rust-std | 9 | 3 of 3 |
| `production-coverage` | `Cc2030093ffdb7a3e8068874ff75532ac.R33` | rust-std | 8 | 3 of 3 |
| `models` | `C6e8361ce3941a9dbed4a89b310886cf8.R0` | rust-std | 30 | 3 of 3 |
| `models` | `C6e8361ce3941a9dbed4a89b310886cf8.R1` | rust-std | 32 | 3 of 3 |
| `models` | `C6e8361ce3941a9dbed4a89b310886cf8.R2` | rust-std | 27 | 3 of 3 |
| `models` | `C6e8361ce3941a9dbed4a89b310886cf8.R3` | rust-std | 17 | 3 of 3 |
| `models` | `C6e8361ce3941a9dbed4a89b310886cf8.R4` | rust-std | 10 | 3 of 3 |
| `models` | `C6e8361ce3941a9dbed4a89b310886cf8.R5` | rust-std | 10 | 3 of 3 |
| `models` | `C6e8361ce3941a9dbed4a89b310886cf8.R6` | rust-std | 11 | 3 of 3 |
| `models` | `C6e8361ce3941a9dbed4a89b310886cf8.R7` | rust-std | 10 | 3 of 3 |
| `models` | `C6e8361ce3941a9dbed4a89b310886cf8.R8` | rust-std | 15 | 3 of 3 |
| `reasoning` | `Cd362aa768021aa2dd83c667dc02645c0.R0` | rust-core | 46 | 3 of 3 |
| `reasoning` | `Cd362aa768021aa2dd83c667dc02645c0.R1` | rust-core | 48 | 3 of 3 |
| `reasoning` | `Cd362aa768021aa2dd83c667dc02645c0.R2` | rust-std | 48 | 3 of 3 |
| `reasoning` | `Cd362aa768021aa2dd83c667dc02645c0.R3` | rust-std | 45 | 3 of 3 |
| `reasoning` | `Cd362aa768021aa2dd83c667dc02645c0.R4` | rust-std | 18 | 3 of 3 |
| `reasoning` | `Cd362aa768021aa2dd83c667dc02645c0.R5` | rust-core | 45 | 3 of 3 |
| `reasoning` | `Cd362aa768021aa2dd83c667dc02645c0.R6` | rust-std | 35 | 3 of 3 |
| `reasoning` | `Cd362aa768021aa2dd83c667dc02645c0.R7` | rust-std | 26 | 3 of 3 |

What changed is the proof term (now written with every argument, by the data generator), the form of the module (kernel data rendered by the decoder, in place of Lean source text), and `crate_ok` (a case analysis on the function index in place of a tactic block); what a verification establishes of a root did not.
