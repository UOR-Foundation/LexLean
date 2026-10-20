# AGENTS

The standing brief for anyone --- human or otherwise --- changing this
repository.

## What this repository is

LexLean: a closed-lexicon LaTeX-to-Lean 4 compiler whose canonical document and
prose-free Lean program are generated from one semantic representation.
`SPEC.md` is the complete implementation contract; nothing here is normative
beyond what it authorizes. It is itself LexLean source: the modules of `spec/`
are the only hand-edited normative inputs, and `SPEC.md`, `model/*.toml`,
`CONFORMANCE.md` and `ERRORS.md` are generated from them (`NS-02`, `NS-05`). Read `README.md` for the shape of the repository,
then `VERIFICATION.md` for which gate discharges what.

The repository was created from `UOR-Foundation/template` at commit
`0a1c799338d7db829aa23365e1acf4f9d01ff8b5` and retains its claim model,
conformance runner, falsifiable gate discipline, dual license, and `xtask`
pattern (SPEC.md §2.2). The template's domain-specific audit allow-list was
removed; LexLean's closed error model is enforced by `audit-errors`.

## The rules

Every gate names the rule it enforces when it fails, so a red gate says *which
promise* was broken rather than merely that something is wrong. These are those
rules, from SPEC.md §27.2; nothing else in the repository defines an
`R`-number.

| Rule | Requirement | Enforced by |
| --- | --- | --- |
| R1 | `model/*.toml` is the single source of every repository conformance claim and public diagnostic code; generated claim/error documentation must match. | `cargo xtask validate-model` |
| R2 | Honesty levels are load-bearing: `some-true`, `build`, and `open` are never blurred. | the honesty meta-gate (`just bdd`) and the vocabulary checks in `validate-model` |
| R3 | A capability begins with a register row, then one scenario, then one failing named test, then implementation. | `just bdd` |
| R4 | Nothing is deferred: no deferral marker, stub, placeholder component, empty branch, or feature switch that disables a normative capability. | `audit-deferral`, inside `validate-model` |
| R5 | Every public failure is registered and emitted through the closed error model; user input never panics. | `audit-errors`, inside `validate-model`, and the negative fixture suite |
| R6 | Shipped dependency boundaries, versions, licenses, and sources are explicit and audited. | `just deny` and `audit-shipped` |
| R7 | Every accepted LexLean source and canonical output satisfies complete lexical and symbolic closure. | the `lexical-closure` and `latex-pdf` conformance suites |
| R8 | Lean and LaTeX are both derived from one linked IR; neither backend may accept an opaque bypass. | the `lean-backend` and `latex-pdf` conformance suites and `audit-language-closure` |
| R9 | Verified status requires Lean elaboration, kernel replay, exact axiom audit, and policy success. | the `verification` conformance suite and `cargo xtask verify-examples` |
| R10 | Platform-independent artifacts are deterministic, source-mapped, and content-addressed. | `cargo xtask check-reproducibility` and `cargo xtask check-golden` |

Expanded, in the order they are most often broken:

1. **The model is the single source (R1).** The source is the `.lex.tex`
   modules of `spec/`. `SPEC.md`, `model/*.toml`, `CONFORMANCE.md` and
   `ERRORS.md` are *generated* from them; editing any is a mistake the gate
   catches, and so is a second document that restates a registered statement
   (`audit-source-authority`, `NS-06`). Run `just model-write` after changing
   a module. The §31 table of `SPEC.md` is derived from the register modules,
   and `cargo xtask validate-spec-links` still checks what a projection does
   not give by itself (RP-07).

2. **Nothing is deferred (R4).** If a change cannot be finished, it should not
   be started --- and `audit-deferral` will say so. It reads every crate,
   `xtask`, the language data, the schemas, the features, the examples, and
   the fixtures, which includes its own source.

3. **Levels are load-bearing (R2).** `some-true` is reproduced from an
   authority and is not established here. `build` is constructed here and
   validated against its oracle: evidence, not proof. `open` is measured and
   reported, never asserted, and no `open` row is acceptance evidence.

4. **A claim about a dependency belongs to that dependency.** Lean's
   guarantees are authority rows in `model/authorities.toml`, not LexLean
   claims. Citing Lean 4.32.1 is not proving it; `leanchecker` is a
   same-kernel replay, never described as an independent checker (§22.4).

## Adding a capability

In this order, because the order is the discipline (R3):

1. A row in the register module of its ID family (`spec/src/Ids<FAMILY>.lex.tex`,
   one `IdRow` record named for the ID, its `sections` cell naming the SPEC.md
   sections that state it), then `just model-write`, which writes `model/ids.toml`,
   the §31 table of `SPEC.md`, and the generated documents. A new family is
   added to `registerFamilies` in `spec/src/Order.lex.tex`.
2. A scenario in `features/suites/`, tagged `@<ID> @build`, whose statement
   equals the register statement after trimming (§27.7).
3. A failing test named exactly `conformance_<id>`, lowercased with
   underscores (§27.8).
4. The implementation.
5. `just vv`.

## Adding a crate

A crate is *shipped* when its manifest does not say `publish = false`, and the
gates read that rather than a list. LexLean 1.0 ships exactly one crate:
`lexlean`. A shipped crate may not depend on a `publish = false` repository
crate (§8.4), must forbid unsafe Rust (§8.1), and is subject to the closed
error model (R5).

## Writing a gate

A gate that cannot fail is worse than no gate, because it reads as evidence.
Before adding one, plant the defect it exists to catch and confirm it fires,
then record that in `VERIFICATION.md`'s falsifiability table (§27.9). A gate
that cannot be falsified because the relevant register is empty must report
that it is armed by the first row, not pass silently as evidence.

Two habits follow from that:

- **Arm an anti-vacuity check on the register, do not assert it outright.**
  "There must be feature files" is false on an empty repository and true on a
  populated one. "There are registered IDs and no feature files" is the defect
  in both. Write the second.
- **A gate that reads source must survive reading its own.** `audit-deferral`
  spells its markers in halves for exactly this reason: a list of forbidden
  tokens written out in full matches itself, and the alternative --- exempting
  the file --- puts a hole precisely where a deferral parked in a gate would
  sit.

## Release lines

A release X.Y lives on the branch `release/vX.Y` (SPEC.md §9.4): cut from
`main` when work on X.Y begins, the target of every pull request of the
milestone X.Y, the source of each `vX.Y.Z` tag, and merged forward by merge
commit into the next line after each landing. `main` is the newest released
line and receives a release line only at its tag. A pull request that moves a
language identity lands one at a time on its line, with one regeneration and
one full gate each; the forward merge carries the regeneration to the next
line. Two lines are open at once so that the next release's work does not wait
for the current release to close.

## Comments

Explain *why*, not *what*. The code says what it does. A comment earns its
place by recording the reason a decision went one way when it could plausibly
have gone another.

## Editing the specification

`SPEC.md` is written in the chapter modules `spec/src/Chapter*.lex.tex`: one
`DocRows::Item` record per heading, paragraph, table, fenced block, continuation
or derived block, in order. To change a sentence, change the string of its
item; to add one, add an item and renumber `i0001`, `i0002`, ... in the module;
keep an item to 60 lines (`cont` continues it). Write a `%` as U+FF05 (a source
cannot hold a raw one, and the projector writes it back), a backslash as `\\`
and a quote as `\"` inside a string. Then run `lexlean fmt` in `spec/` and
`just model-write`. The §31, §26.3 and §27.4 tables and the total under §31 are
derived blocks, not text: change the register, registry or authority module instead.
The projector is Rust and can falsify a projection, never establish one (C9).

A scanner over `.lex.tex` sources selects a module by its parsed `spec`, never by
a substring of its text: the chapters quote every marker and control in their
strings (`conformance_au_05` found one that did not).
