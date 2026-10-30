# Mathlib import: acceptance demo (#340), notes for the design document (#341)

Branch `feat/mathlib-demo` in `~/lexlean-mathlib-340`, cut from the merged `feat/mathlib-import-integration`
(`9d2af0fc7`). Local commits only, nothing pushed, nothing posted to GitHub.

## Result

The document in `demos/mathlib-blake3-g` verifies end to end, offline, against the pinned Mathlib
(`520045ab14e2`, toolchain `v4.32.1`): `lexlean verify` prints `verified 4 modules`, with the Lean
elaboration, `leanchecker` replay and the exact axiom audit of SPEC section 22 all passing. Statements, all for
`BitVec 32` (and the first two also for `UInt32`):

| Statement | Module / theorem | Imported lemmas cited | Axioms (exact) |
| --- | --- | --- | --- |
| `(a + b) - b = a` | `Cancellation::add-sub-cancel`, `-u32` | `BitVec.add_sub_cancel`, `UInt32.add_sub_cancel` (core) | `propext` |
| the same through Mathlib | `Cancellation::add-sub-cancel-group` | `add_sub_cancel_right` at `AddGroup (BitVec 32)` (Mathlib, wrapper) | `Classical.choice`, `Quot.sound`, `propext` |
| `(a ^^^ b) ^^^ b = a` | `Cancellation::xor-cancel`, `-u32` | `xor_assoc`, `xor_self`, `xor_zero` (core), `rewrite` | `Quot.sound`, `propext` |
| `rotl (rotr a n) n = a`, n in {7, 8, 12, 16} | `Rotation::rotate-left-right` | `Blake3GHost.rotateLeft_rotateRight` (host library, see below) | `Quot.sound`, `propext` |
| G inverse undoes G | `Mixing::mixing-inverse` | the four above, `add_right_comm` (Mathlib) | all three |

Every theorem states its axiom set with `\exactaxioms` and the audit confirms it. The Mathlib route costs
`Classical.choice` that the core route does not.

## What is NOT imported (read this first)

`rotateLeft (rotateRight x n) n = x` is in neither Lean core nor Mathlib at this pin (searched `Init`, `Std`,
Mathlib, batteries for any lemma relating `rotateLeft` and `rotateRight`: there is none). Proving it needs
`BitVec.eq_of_getLsbD_eq`, the two `getLsbD_rotate*` lemmas, and arithmetic on bit indices (`omega`). No prose or
semantic-module proof form can do index arithmetic, so the proof is ordinary Lean (13 lines) in the host library
`demos/mathlib-blake3-g/Blake3GHost.lean`, as `Blake3GHost.rotateLeft_rotateRight`. It is a lemma of this
project's host library, in the trusted base next to the imported libraries, not a lemma of Mathlib or Lean. The
LexLean audit checks its axioms (`propext`, `Quot.sound`) like any other. It is the one hand-written Lean in the
demo. It is a good candidate for upstreaming to Lean core.

G itself is not a named definition in the document. A prose definition needs a hand-written glossary entry, and a
semantic module cannot call an external constant (below). The statement is therefore relational: for all 22 words,
if the 8 forward equations (the BLAKE3 `gcore` as the step machine runs it: `a1 = a + b + x`, `d1 = rotr(d ^ a1, 16)`,
`c1 = c + d1`, `b1 = rotr(b ^ c1, 12)`, `a2 = a1 + b1 + y`, `d2 = rotr(d1 ^ a2, 8)`, `c2 = c1 + d2`,
`b2 = rotr(b1 ^ c2, 7)`) and the 8 inverse equations from the issue hold, with fresh names for the recovered
intermediates, then the four recovered words are `a`, `b`, `c`, `d`. The equations determine every value, so this
is equivalent to `inverse (G ...) = ...`. `BitVec.rotateRight v n` with `0 < n < 32` is the BLAKE3 `rotr`
(`(v >> n) | (v << (32 - n))`); `BitVec.rotateLeft` is its `rotl` up to the order of the operands of `|||`. The
document uses the BitVec functions and does not re-prove that identification.

## Hand-written entries

Generated (by `lexlean deps glossary`, 38 entries, committed in `glossary/lean.mathlib`): every entry for an
imported or host declaration, and the 27 support entries (`HAdd`, `HAdd.hAdd`, `HSub`, `HXor`, `BitVec`, `Nat`,
`UInt32`, ...). Hand-written (`glossary/blake3.g`, 8 files): label words only (`word`, `arithmetic`,
`cancellation`, `xor`, `rotation`, `mixing`, `inverse`, `function`), which titles and headings require (SPEC 13.3).
They denote generated constants and assert nothing. No hand-written entry denotes an imported declaration.

## Proof forms with external lemmas (the question asked first)

| Form | Used with an external lemma here? | Result |
| --- | --- | --- |
| `Close the goal with \(lemma(args)\)` | yes, in every theorem | works, core, Mathlib wrapper and host lemma alike |
| `rewrite` (`\forward`) | yes | works; a lemma with implicit arguments (`xor_self`) is used bare |
| `have` | yes | works |
| `constructor` | yes | works; note `P and Q and R` is left-folded, `And (And P Q) R` (`elaborate/mod.rs`), so branches nest on the left |
| `Assume` (several names) | yes | works for 16 hypotheses |
| `simplify` | yes (scratch module, not committed) | `\begin{simplify}{goal}` with three `\rule` lemma terms proves `(a ^^^ b) ^^^ b = a`; verified, `\allowaxioms{Quot.sound;propext}` |
| `calculate` | yes (scratch module, not committed) | a one-step chain justified by the core lemma verified; chain steps are whole-term equalities, so a lemma used inside a larger term needs `rewrite` instead |
| `Apply`, structured `apply`, `cases`, `induction`, `Use ... as the witness` | no | not exercised with an external lemma: the demo has no lemma with premises or an inductive scrutinee. Nothing in SPEC 16 treats a proof term differently by origin |
| language 1.2 semantic module: `reflexivity`, `linear_arithmetic`, `congruence`, `law`, `apply`, `term_cases`, `cases`, `induction`, `simplify` | no | cannot, by construction: `apply` names a prior theorem of the document, `law` a lemma of the registered fold-law table, `linear_arithmetic` and `simplify` a fixed rewrite set plus prior document definitions (SPEC 17.11, 17.12). None can name an external glossary entry, and a semantic module has no way to call an external constant at all |

Consequences: (a) an external lemma can only be cited from prose; (b) a statement about `BitVec`/`UInt32` bit
operations therefore cannot be a semantic-module theorem; (c) #316 (document denotations cannot name
semantic-module declarations) is not the binding constraint, the missing external-constant access in semantic
modules is; (d) #317 / PR #332 (Nat bitwise primitives) does not help, the demo is at fixed width where the
primitives already exist, but the semantic module still cannot prove anything about them without external lemmas.
`UInt32.add_sub_cancel` was proved in the document at `UInt32` with the core lemma; the semantic module has
`uint32` and `bit_xor` but no proof form that reaches that lemma.

## Draft issues (not filed)

1. **Semantic modules cannot cite external lemmas.** Language 1.2 semantic-module proofs name prior document
   theorems, registered fold laws or fixed rewrite sets. A proof form `external{entry: ..., arguments: [...]}`
   lowering to `exact @<Lean constant> ...`, validated against the entry's signature (the same probe as 13.7),
   would let a semantic module prove facts about its own fixed-width definitions from Lean core and Mathlib.
   Without it the semantic fixed-width world and the Mathlib world do not meet. Needs: register row, scenario,
   LLT code, SPEC 17.12 text. Related to #316 and #317.
2. **A prose arithmetic form.** Bit-level facts reduce to linear arithmetic on indices. A prose proof form
   `Close the goal by linear arithmetic.` (lowering to `omega`, or the `linear_arithmetic` of 17.12) after
   `Apply \(...ext lemma...\)` and `Assume` would let the rotation inverse be a document proof instead of a host
   lemma. Without it every such fact needs a host-library lemma written in Lean.
3. **`deps glossary --names` imports only `Init`.** `GlossaryExtract.lean` imports the `--modules` and, with
   none, `Init`. A Mathlib declaration cannot be requested by name without requesting its whole module (thousands
   of entries). Wanted: `--import` for the modules to load, or load the defining module of each name. The demo
   works around it by requesting the two host modules (15 declarations) and wrapping every cited lemma there.
4. **Generated surfaces for class operators are unreadable.** `HAdd.hAdd` is `hAddHAdd(a, b)`, a wrapper is
   `lexLeanSpecBitVec32BitVecAddSubCancel(a, b)`. Mapping the core operator classes to infix frames (`+`, `-`,
   `^^^`) in the generator would make the statements read as the issue states them.
5. **Examples that declare a library and the gate.** EX-08 auto-discovers `examples/*`. This example needs the
   library fetched (LLV7019 otherwise), so `cargo xtask verify-examples` needs it fetched in CI, or the example
   needs its own CI part. Owner decision (see Open decisions).

## Defects the demo found in the merged branch (fixed here, rows VR-27 and ML-05)

1. **Probe, instance binder.** `print_lse_type` named every binder, so `[x3 : HAdd ...]` drew the
   `unusedVariables` linter and the probe failed with `LLV7006`. Any generated entry for a class operator was
   unverifiable. Now an instance binder nothing later mentions is anonymous.
2. **Probe, numeral capture.** `parameter_types` substituted the caller's arguments into the callee's binder types
   by name, so in `Eq (x ^^^ x) 0` inside an entry whose binder is named `a` (as the type parameter of `Eq` is)
   the `0` was ascribed `(0 : x0)`, the type of a variable, and Lean said `type expected`. The callee's binders are
   now renamed apart.
3. **`fmt` with a library.** `fmt` (write mode) checks the project in a scratch copy that did not contain the
   library, so it failed with `LLV7019` on any project that declares one. The copy now carries the library's
   presence markers (`.git/HEAD`, manifest, built-artifacts directory), not the library.

Defects 1 and 2 are covered by `conformance_vr_27` (fixture `tests/fixtures/probe-binders`: `HXor.hXor`, which has an
instance binder, and `UInt32.xor_self`, whose binder is named `a`; real Lean elaborates the probe with no output). Defect 3 is
covered by `conformance_ml_05` (a CRLF entrypoint of a project that declares a present library is rewritten by `fmt`).
The row VR-25 of this branch collided with the #339 row of that number and is VR-27 after the merge. Each has a register
row, a scenario, a case, and was shown red by
planting the defect and green with the fix. The order of R3 was not followed for the code: the defects were found
by running the demo and fixed first, the rows and tests written after, then red demonstrated by planting. Total
register count after merging the integration branch: 476.

Also changed: `xtask` links the example's `.lake` into the clean copy of `verify-examples` (hard links, a copy
across devices, symlinks recreated) instead of copying 7 GB; `audit-deferral` skips `.lake` (Mathlib's own sources
contain `TODO`); `.gitignore` lists `examples/**/.lake/`.

## Measurements

Machine: WSL2, 16 GB shared with other sessions running builds (load average 5 to 25), so seconds are noisy.
Peak memory is `/usr/bin/time -v` maximum resident set. Debug build of `lexlean`. Mathlib at `520045ab14e2`.

| Step | Wall | Peak RSS | Note |
| --- | --- | --- | --- |
| Fetch (`lake exe cache get`), from the #334 notes | about 5.4 min (file-time bound) | not measured | 7.0 GB on disk (6.4 GB `.lake/build`, 387 MB packages, 147 MB source), 436 MB download cache |
| Lake workspace for the example (`cp -al` of the pinned checkout and 8 packages) | 8.5 s | n/a | hard links, no extra disk |
| Host library build (`lake build`) | 1 to 6 s | n/a | 2 modules |
| Glossary generation, 2 host modules, 15 declarations | 95 s cold, 26 s warm | 3.08 GB, 3.25 GB | 11 translated, 4 excluded (internal proof auxiliaries), 0 refused, 27 support entries |
| `check` | under 5 s | under 2 GB cap | 4 modules |
| `verify`, cold (first run) | 74 s | 1.33 GB | before the merge with #339 |
| `verify`, on the merged branch | 41 s | 1.36 GB | probe now also prints `#print axioms` of imports |
| `verify`, `.lexlean` removed | 51 s | 1.33 GB | |
| `verify`, unchanged | 53 s | 1.32 GB | no per-module reuse: `verify` reads no replay cache (SPEC 22.13) |
| `verify`, after a one-line edit (title of the leaf module) | 40 s | 1.31 GB | |

Cost structure of `verify`: each of the 4 modules is a separate Lean process that imports the closure of
`Blake3GSpecs` (Mathlib `Algebra.Group.Basic` and `Data.BitVec`), plus the probe, the axiom audit module and
`leanchecker` per module. There is no incremental behaviour: an edit anywhere costs a full verification. The
imported Mathlib declarations are not replayed by `verify` (SPEC 22.4 checks only the project's modules); the
closure replay of #337 is not wired into it.


## Coverage

Demo request (the two host modules, what the document cites): 15 requested, 11 translated, 0 refused, 4 excluded
(internal proof auxiliaries), 27 support entries. Every cited declaration translated.

Modules the cited lemmas come from, requested whole into a scratch directory (not committed), warm, 15.5 s, 2.82 GB
(`Init.Data.BitVec.Lemmas`, `Init.Data.UInt.Lemmas`, `Init.Data.UInt.Bitwise`, `Mathlib.Algebra.Group.Defs`,
`Mathlib.Algebra.Group.Basic`):

| Outcome | Count |
| --- | --- |
| requested | 7,222 |
| translated | 3,689 (+370 support entries) |
| refused | 841: names with an apostrophe 210, entry check 84, `Sort u` 46, strict-implicit binder 8, depends on a `Sort u` declaration 294, on an internal one 141, on an instance 20, on an auxiliary one 24, on a non-grammar name 14 |
| excluded | 2,692: internal 2,067, instance 239, auxiliary 215, deprecated 111, recursor 60 |

So 81% of the 4,530 candidate declarations (requested minus excluded) translate. The core lemmas the demo cites
(`BitVec.add_sub_cancel`, `xor_assoc`, `xor_self`, `xor_zero`) are monomorphic and translate directly (`bitVecAddSubCancel`);
the Mathlib ones (`add_sub_cancel_right`, `add_right_comm`) translate as class-polymorphic entries that cannot be
used until a specialisation wrapper exists, which is why the demo wraps them. The demo wraps the core ones too only to
keep the committed glossary at 38 entries instead of about 4,000.

## Open decisions for the owner

1. Where Mathlib lives for an example: the pinned checkout is hard-linked into `.lake/packages` of the example
   (`setup-offline.sh` in the example). Confinement (`confined_dir`) refuses a symlink out of the project, so a link is not an option;
   hard links need one filesystem. CI would need `lexlean deps fetch` (7 GB, about 5 minutes) before the examples
   leg, or this example needs its own part.
2. Whether `verify` should also replay the imported declarations the document depends on (closure replay, #337).
   As it stands a verified document says nothing about Mathlib's own proofs, which is what SPEC 5.1 and 5.3 say.
3. Whether an upstream lemma `BitVec.rotateLeft_rotateRight` should be proposed to Lean core.
4. Whether the relational encoding of G is acceptable until a semantic module can name external constants.
