<div align="center">
  <a href="https://github.com/UOR-Foundation/LexLean">
    <img src="assets/lexlean-logo.svg" alt="LexLean" width="320">
  </a>

  <br />

  <p><strong>One controlled-language document in. The canonical paper, a kernel-checked Lean 4 program, and a certified Rust package out.</strong></p>

  <p align="center">
  <a href="SPEC.md">Spec</a>
  ·
  <a href="CONFORMANCE.md">Conformance</a>
  ·
  <a href="ERRORS.md">Diagnostics</a>
  ·
  <a href="VERIFICATION.md">Evidence</a>
  ·
  <a href="CHANGELOG.md">Changelog</a>
  ·
  <a href="https://github.com/UOR-Foundation/MNIST">MNIST model</a>
  </p>

  <p>
  <a href="https://github.com/UOR-Foundation/LexLean/releases/latest"><img src="https://img.shields.io/github/v/release/UOR-Foundation/LexLean" alt="Latest release"></a>
  <a href="https://github.com/UOR-Foundation/LexLean/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/UOR-Foundation/LexLean/ci.yml?label=gate" alt="Acceptance gate"></a>
  <img src="https://img.shields.io/badge/Lean-4.32.1%20pinned-8b5cf6" alt="Pinned Lean toolchain">
  <img src="https://img.shields.io/badge/register-457%20conformance%20IDs-6d28d9" alt="Conformance register size">
  <img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2ea043" alt="License">
  </p>

  <p>LexLean compiles <code>.lex.tex</code> modules written in a closed controlled language: every word, symbol, and control must be declared by a versioned lexicon package, every sentence parses under a fixed grammar, and one typed intermediate representation generates both the canonical LaTeX document and the prose-free Lean 4 program.</p>

  <p><code>lexlean verify</code> compiles the generated Lean under the pinned <code>leanprover/lean4:v4.32.1</code> toolchain, replays every module with <code>leanchecker</code>, audits axioms by parsing <code>#print axioms</code> exactly, and publishes a content-addressed attestation.</p>

  <p><strong>Evidence, not belief.</strong> <code>check</code> and <code>build</code> never claim verification. Only <code>verify</code> runs Lean, and whatever it cannot check is reported as not validated, never passed in silence.</p>
</div>

## Quickstart

### Run in the container

`lexlean verify` runs the pinned toolchain, so the published image carries the compiler and `leanprover/lean4:v4.32.1` together. This is the shortest path to a run that can actually verify:

```text
docker pull ghcr.io/uor-foundation/lexlean:0.4.0
docker run --rm -v "$PWD:/work" ghcr.io/uor-foundation/lexlean:0.4.0 verify
```

The image is around 4 GB, nearly all of it the pinned toolchain's compiled library: the part `verify` needs, and the part a smaller image would have to leave out.

### Or install a binary

Each [release](https://github.com/UOR-Foundation/LexLean/releases) attaches one binary per supported host (SPEC.md §8.3), the packaged crate, a CycloneDX bill of materials, `checksums.txt` over every other asset, and the `just vv` evidence for the tagged commit.

```text
tar -xzf lexlean-0.4.0-binaries.tar.gz
sha256sum -c checksums.txt
```

A binary alone can `init`, `lock`, `fmt`, `check`, and `build`. `verify` additionally needs the pinned toolchain installed through [elan](https://github.com/leanprover/elan). From source, with the prerequisites in [Building and running the gate](#building-and-running-the-gate):

```text
cargo install --locked --path crates/lexlean
```

The complete normative contract is [SPEC.md](SPEC.md) (`LEXLEAN-SPEC-1`). The generated register is [CONFORMANCE.md](CONFORMANCE.md), the closed diagnostic registry is [ERRORS.md](ERRORS.md), and the falsifiability evidence is in [VERIFICATION.md](VERIFICATION.md).

### Write, build, verify

```text
lexlean init --name my-doc --module-prefix MyDoc
lexlean lock
lexlean check
lexlean build
lexlean verify
```

The document is plain controlled language. This is the complete SPEC.md §29 example, committed as [examples/nat-add-zero/src/Main.lex.tex](examples/nat-add-zero/src/Main.lex.tex):

```latex
\begin{lexlean}{Main}
\useglossary{lexlean.std.nat@1.0.0}
\title{Natural number addition}

\begin{theorem}{add-zero}
\noaxioms
For every natural number \(n\), \(n + 0 = n\).
\begin{proof}
Close the goal by reflexivity.
\end{proof}
\end{theorem}
\end{lexlean}
```

From that one source LexLean generates the prose-free Lean module

```lean
module
public import Init
set_option autoImplicit false
namespace LexLeanExample.Main

public theorem add_zero (llv0 : Nat) : Eq (Nat.add llv0 0) llv0 := by
  rfl

end LexLeanExample.Main
```

and the canonical LaTeX document, regenerated from the linked representation rather than copied (the prose reads "The goal follows by reflexivity."), plus source maps, coverage, and a manifest, all byte-reproducible across directories. `verify` then compiles the module under the pinned toolchain, replays it with `leanchecker`, parses `#print axioms` exactly, and records the empty observed axiom set in a content-addressed attestation.

## How LexLean works

```text
your document (.lex.tex, closed lexicon)
        |
        v
one typed semantic representation         <- the single source of truth
        |
   +----+----------------+
   v                     v
canonical LaTeX       prose-free Lean 4
(paper or PDF)        (kernel-checked, exact axiom audit)
                          |
                          v
                  production compilation
                          |
                          v
                  certified Rust package
                  + certificates A / B / E + attestation
                  (replays offline, without the source)
```

| Command | What it does | What it may claim |
| --- | --- | --- |
| `lexlean init` | Scaffolds a project | nothing |
| `lexlean lock` | Pins lexicons and dependencies in a canonical lock file | nothing |
| `lexlean check` | Lexical closure, grammar, typing, diagnostics | never verification |
| `lexlean build` | Generates Lean, LaTeX, source maps, manifests | never verification |
| `lexlean verify` | Compiles under the pinned toolchain, replays with `leanchecker`, audits axioms, attests | the only verified claim |
| `lexlean production build` | Exports certified Rust packages with their certificates | kernel-checked preservation |
| `lexlean migrate --to 1.2` | Upgrades a language-1.1 project, proved in a scratch copy first | writes nothing on refusal |

A language-1.2 project can also import the modules of a locked, content-addressed package of its workspace instead of copying them: `[[source_package]]` in `lexlean.toml` names a member directory listed by `lexlean-workspace.toml`, `lexlean.lock` pins the package by the digest of its manifest and source files, every command recomputes the pin, imports are closed to the project's own modules and the declared packages, and `verify` replays the package's modules under its module prefix with the package as their origin (SPEC.md §10.5, `PK-01`..`PK-05`; the workspace the cases use is [tests/packages/workspace](tests/packages/workspace/)).

Language-1.1 semantic modules use the closed-core backend's fixed finite Lean budgets (`maxRecDepth = 100000`, `maxHeartbeats = 1000000000`) so complete nested byte-data declarations are not restricted by interactive defaults. Project child timeout and output limits and full kernel and axiom verification remain mandatory. These are generated backend settings, not caller-provided options.

## Why LexLean

* **The paper cannot drift from the program.** Both are generated from one linked representation. Neither is copied, neither is hand-maintained, and a change to one is a change to the other.
* **Assumptions are audited, not assumed.** An attestation records the toolchain hashes, the process records, and the exact observed axiom set of every module.
* **The binary carries its guarantees.** Production compilation to Rust emits certificates that a consumer replays offline, without the source, without Lean, and without trusting the producer.

## When to use LexLean

* **Safety-critical code that must be proved, not sampled.** Total programs with machine-checked termination, exact failure typing, and a certified Rust output.
* **Audited or regulated artifacts.** Every claim is a content-addressed record: what was checked, by which toolchain, under which assumptions.
* **Documents whose formal content must match their prose.** The canonical document is regenerated from the same representation that the kernel checks.
* **Verified models and reasoners.** Integer neural networks, rule-based reasoning machines, and text classifiers as closed semantic objects. The [MNIST model](https://github.com/UOR-Foundation/MNIST) is one: an int8 digit recognizer whose weights are content-addressed artifacts, whose forward pass is checked layer by layer, and whose contract for every input is a theorem.

## Capabilities

Every capability below is a claim in [`model/ids.toml`](model/ids.toml), carried at honesty level `build`: constructed here and validated against its oracle by the test named `conformance_<id>`. Every row is validated by `just vv`, and its IDs link the claim to its register row, scenario, and conformance test.

| Capability | IDs | Level |
| --- | --- | --- |
| Exact repository identity, layout, generated documents, release gate, total classification of every closed enum by every consumer, and the devcontainer pinned to the toolchains | `RP-01`..`RP-18` | `build` |
| Closed project configuration, canonical lock file, and offline dependency policy | `CF-01`..`CF-18` | `build` |
| Total lexical closure: every accepted atom is covered by exactly one declared origin | `LX-01`..`LX-14` | `build` |
| Versioned lexicon packages with closed schemas, denotations, and renderer tokens | `GL-01`..`GL-18` | `build` |
| Fixed structural, mathematical, and proposition grammar with closed ambiguity handling | `GR-01`..`GR-16` | `build` |
| Typed closed IR with canonical serialization, native core modules (and the second schema of them that lossless migration of the proof library needs), portable language-1.1 application data and operations, versioned language-1.2 semantic modules, products, first-class functions, and widening conversions, ordered collections and graphs, semantic snapshots, alpha identities, recursion evidence, a split on a document term or a Boolean in a proof, and content identities | `SM-01`..`SM-36` | `build` |
| Document and generic semantic declarations with exact self-application, type checking, structural recursion, recursive and mutual language-1.2 data, generic and executable higher-order definitions, mutual and well-founded recursion with semantic evidence, explicit state threading, and acyclicity rules | `DF-01`..`DF-18` | `build` |
| The structured proof language with pinned Lean lowerings, including closed linear arithmetic, integer order, and the fold laws | `PF-01`..`PF-23` | `build` |
| Prose-free deterministic generated Lean with complete token traceability | `LN-01`..`LN-12` | `build` |
| Canonical LaTeX regeneration and the optional hash-checked PDF provider | `TX-01`..`TX-12` | `build` |
| Canonical diagnostics, source maps, coverage, manifests, and reproducible builds | `AR-01`..`AR-14` | `build` |
| Sixteen-stage verification with leanchecker replay and exact axiom audit | `VR-01`..`VR-23` | `build` |
| The exact CLI contract, the stable seven-method Rust `Engine` API, the host-driven end of the children the library started, the `production` subcommands, and the `migrate` command | `CL-01`..`CL-24` | `build` |
| Filesystem confinement, no shell, no hidden network, closed failure model | `SE-01`..`SE-12` | `build` |
| Language-1.2 production eligibility: closed targets, effects, and construct dispositions; runtime closures; target-dependent, fail-closed root analysis; deterministic eligibility reports; an exhaustiveness audit; measured stack bounds for every recursive root a harness runs | `PD-01`..`PD-08` | `build` |
| Named-root extraction through Lean's compiler front end: an adapter that is a closed tooling module over a registry derived from the pinned Lean and probed on every extraction, checked against a handwritten oracle; a closed, canonical, root-independent compiler input; fail-closed rejection; and an exact closure cross-check against production eligibility | `NE-01`..`NE-06` | `build` |
| The production realization calculus: a closed target syntax with canonical identity, a kernel-checked denotation, a realization library agreeing with LexLean's own collection primitives, complete realization coverage of the production registry, and a differentially tested Rust profile | `TC-01`..`TC-07` | `build` |
| The canonical Rust backend: a closed, checked Rust AST whose every construct corresponds to the calculus; hygienic identifiers, single ownership with last-use and component moves, exact failure typing, and no hidden allocation; contiguous lists with constant-work cells; a closed runtime library boundary of stated machine primitives; deterministic packages with checked exports, a lint gate and crate attributes read from one embedded language module, and provenance that states the realization's effects | `RB-01`..`RB-21` | `build` |
| GNAF requests over the calculus: an optimizer-independent complete-system universe, a machine contract that accounts every action, scalar and Pareto orders without weighting, fail-closed validation, and kernel-checked answers and authority vectors | `GN-01`..`GN-08` | `build` |
| GNAF certification: certificates that prove a global optimum, an argmin, or a complete Pareto frontier over the request's declared universe (a grammar of systems, never all calculus or Rust programs) from a premise for every system, a library proving the checker sound, closed transformation schemas, local normality kept distinct from optimality, a bridge from a representation metric to execution cost, and honest non-global statuses | `GC-01`..`GC-11` | `build` |
| Semantic preservation from the source to the calculus: a validated lowering of every production root, a kernel-checked certificate per root relating the lowered program to the source denotation with exact width predicates, a differential evaluator, a proof library that is LexLean source (native core modules migrated once from the Lean it replaces, its Lean generated by the compiler) pinned by exact axioms, certificate checking in verification, a declared Rust machine on which every rendering is evaluated, certificate B relating every rendering to its program, certificate E composing them end to end on the declared Rust machine (the printer, export wrappers, and `rustc` are outside it and are measured by the rustc differential), boundary validators, a rustc differential of the declared machine, and certificates A and B as kernel-term data built by the production compiler, which carries no Lean text template | `SP-01`..`SP-29` | `build` |
| Language-1.2 models: content-addressed typed artifacts, contracts with sound validators, exact deterministic, rule, statistical, neural (integer kernels), and composite realizations, generated evidence obligations restated in Lean, checked runtime boundaries, and their identity, production, and verification | `MD-01`..`MD-13` | `build` |
| The differential validation oracle: `lean4-prod` pinned by commit and content hash as validation evidence only, a corpus of every production root and one probe per runtime construct, a comparison of acceptance, contract, behavior, and resources that never compares bytes, versioned tables of known differences and of constructs outside the overlap, planted discrepancies detected, failures that are unavailable rather than validated, and its isolation from every identity | `OR-01`..`OR-08` | `build` |
| The production export contract: a stable, versioned `lexlean::api::production` façade and its CLI through which a consumer compiles production roots to certified Rust packages; closed requests refused before Lean on every root, target, pin, evidence, path, and package defect; results binding every output to the source, semantic, compiler-semantics, target, compiler-input, preservation, and certificate identities with unforgeable certification; deterministic, confined, offline bundles; source-free replay; versioned schemas; each output's effects and what `recursion` asks of the host | `PX-01`..`PX-13` | `build` |
| Language-1.2 reasoning machines: logics, guarded inference rules, verifiers, and forward, search, and generate-and-verify reasoners elaborated to ordinary declarations, with exact obligations, fixed-template theorems over the axiom-free reasoning runtime, a closed runtime boundary, resource ledgers, and their production and calculus realization, with GNAF requests over the clinical rule base's forward-chaining plans | `RS-01`..`RS-14` | `build` |
| Language-1.2 compiler-tooling modules: a closed, typed program over a registry of the external Lean interface (types with exact constructors and index requirements, calls with exact signatures), with flat index-exhaustive matching, licensed partial and mutual recursion, complete recursive traversal, command registration, invocation and evaluation as semantic data, both backends from one linked value, and no member that holds Lean text | `TL-01`..`TL-11` | `build` |
| The language-1.2 authoring notation: a readable `.lex.tex` text for a semantic module, in bijection with its canonical JSON, whose every position and variant is read from the schema, with readable forms for types and terms and a generic form for every other value, formatted canonically by `fmt` failing closed, the only form in which a language-1.2 source of the product is written (the data form is refused under language 1.2, and `lexlean::sdk` is the explicit conversion), the same for the compiler-tooling modules in the generic notation, with no compiler-semantics ID that depends on which form an embedded module is written in, with no product source owned by a generator but five named records, read and written by programs only through the authoring API (`lexlean::sdk`), and, for a native core module, a readable program over the shared table of kernel terms whose numbering is the order of its text | `AU-01`..`AU-13` | `build` |
| The runtime of generated Lean as native modules: the portable, collection, model, and reasoning runtimes of language 1.2 are LexLean source that every generated module imports, migrated losslessly and audited against their exact axioms, the released runtime text of languages 1.0 and 1.1 is frozen data equal to their released output, and no other Lean runtime text remains in the compiler but the decoder of core module 2 | `RT-01`..`RT-04` | `build` |
| The standard library: language-1.2 source beside the compiler whose members are certified production roots, each validated against its oracle by a seeded differential of its evaluator and rustc realizations; bytes and text over `Bytes`, an output buffer of a given capacity written in order; SHA-256 against FIPS 180-4, the NIST CAVP vectors, and the `sha2` crate; and the `std-models` and `std-reasoning` source packages, generic over types, whose laws are kernel-checked theorems and whose application is the `helpdesk` example | `SL-01`..`SL-05` | `build` |
| The bootstrap project: the compiler's own components as language-1.2 source compiled by LexLean's production pipeline to certified Rust packages, the exact axiom-output parser and its closed Lean-name check, the primitive scanner of §12.2, the preservation lexer and token audit, the target calculus's checker, the Rust crate validator, and the Rust printer, each held to the Rust it replaces (the stage-0 oracle) by the committed vectors and a seeded differential of its evaluator and rustc realizations, with planted defects detected and the production export committed | `BS-01`..`BS-09` | `build` |
| Source packages: a language-1.2 project imports the modules of a locked, content-addressed package of its workspace: `[[source_package]]` and `lexlean-workspace.toml`, the pins in `lexlean.lock` recomputed by every command, imports closed at link, no re-export of a package's production roots, and the package's modules replayed by the importer's `verify` with their origin in the records | `PK-01`..`PK-05` | `build` |
| The literal `nat-add-zero` example, the Lean-verified feature examples, and the complete negative fixture suite | `EX-01`..`EX-09` | `build` |
| Language-1.2 text components, first slice: tokenizers and their round-trip laws, with the laws carried as one native core module the generated modules import rather than authored Lean text | `LM-01`, `LM-02`, `LM-03`, `LM-13` | `build` |
| Language-1.2 text components, second slice: context windows with their bound, exactness, and suffix laws, and greedy and seeded text generators bounded by `max_tokens` whose scorer sees only contexts that fit, over the same native laws module | `LM-04`, `LM-05`, `LM-06` | `build` |
| Language-1.2 text components, third slice: text classifiers over token-count features with their width, total, and fit laws, composition of a window model under `scan` and adapters over `string` through the model constructs alone with no new declaration kind, and the audit that no prompt is code | `LM-07`, `LM-08`, `LM-09` | `build` |

Range rows abbreviate consecutive registered IDs; every individual ID in each range is registered in [`model/ids.toml`](model/ids.toml) at the stated level with its own scenario and test.

## Examples that verify under the pinned toolchain

Every directory under [examples/](examples/) is discovered by the example gate (`EX-08`) and must format, lock, check, build, and verify with real Lean 4.32.1 (`cargo xtask verify-examples`). Its platform-independent build outputs and its normalized verification records are committed under `expected/` and compared byte for byte by the golden gate (§28.3) and the example gate (§29.5); that those bytes are also independent of where the build ran is the separate claim of `AR-13` and `EX-06`.

| Example | What it exercises |
| --- | --- |
| [nat-add-zero](examples/nat-add-zero/) | The literal SPEC.md §29 document (`EX-01`..`EX-06`). |
| [peano-arithmetic](examples/peano-arithmetic/) | Five modules with explicit imports, a local path glossary of proof constants from `Init` (`Nat.add_comm`, `Nat.le_trans`, `Eq.symm`, universe-polymorphic `rfl`, ...), label words, document-denoted definitions of all three kinds (`count`, `double`, `even`, `positive`, `divides`), noun-of and binary-noun-of frames (`the successor of`, `the sum of ... and ...`), sections with inherited parameters and references to parameterized declarations, and every §16 proof form: `Assume`, `Apply`, `Close the goal with`/`by reflexivity`, witnesses, left/right, multi-rule `rewrite` at the goal, `constructor`, `cases` on naturals and on hypotheses (`And`, `Exists`), `induction`, and `calculate`; theorems under `\noaxioms`, `\allowaxioms{propext}`, and `\exactaxioms{Classical.choice;Quot.sound;propext}` whose observed axiom sets are audited exactly. |
| [propositional-logic](examples/propositional-logic/) | Reasoning over `Prop`-typed locals with a `proposition` type noun defined as the sort: commutativity and associativity of conjunction and disjunction, disjunction elimination, double negation, De Morgan, explosion, biconditionals, and classical double-negation elimination under an exact axiom policy, through cases on `And`/`Or`/`Iff` hypotheses, `constructor`, and `Apply`. |
| [list-induction](examples/list-induction/) | Universe-polymorphic `List` with an eliminator descriptor: type-valued section parameters, `List.nil`/`List.cons`, an infix `⧺` for `List.append`, structural induction over lists using earlier document lemmas as rewrite rules, and nested noun phrases (`the length of ... equals the sum of the length of ... and the length of ...`). |
| [semantic-1.1](examples/semantic-1.1/) | A source-Lean-free, multi-module language-1.1 project covering every closed type, declaration, term, instance, structural-recursion, match, Boolean/Nat validation, proof variant, portable integer width, byte/string form, primitive operation, and cross-module kernel reduction. |
| [language-1.2](examples/language-1.2/) | The language-1.2 contract (SPEC.md §17.12): a `lexlean/semantic-module/2` module whose definition and theorem statement use the 1.2-only typed `let` term, lowered to Lean `let` and verified with an empty observed axiom set. |
| [recursive-data](examples/recursive-data/) | Language-1.2 recursive data (SPEC.md §17.12) across two modules: a self-recursive parameterized `Tree`, a nested `Rose` through `List`, the mutual `Expr`/`Stmt` group, a parameterized result-like `Outcome`, products and pairs in a structure; structural recursion and induction with one hypothesis per recursive field, under exact axiom policies. |
| [higher-order](examples/higher-order/) | Language-1.2 first-class functions (SPEC.md §17.12): generic `mapList`, `foldList`, and `compose`; lambdas with explicit captures; definition references across modules; a visitor record of functions over an arithmetic AST agreeing with direct evaluation; a generic induction theorem applied at `Nat`; executable definitions passing only non-escaping closures. |
| [recursion](examples/recursion/) | Language-1.2 recursion (SPEC.md §17.12): mutual structural groups over a nested rose tree, the mutual `Expr`/`Stmt` syntax, and the naturals, each lowered with `termination_by structural`; three well-founded normalizations (`countdown`, `reduce`, a bounded `search`) whose per-call decrease obligations are proved by `linear_arithmetic` and bound as explicit evidence; closed values computed by kernel reduction under exact axiom policies. |
| [collections](examples/collections/) | Language-1.2 ordered collections (SPEC.md §17.12): a String-keyed compiler symbol table built by an executable fold, a call graph whose reachability and least-first topological order the kernel computes, a cyclic graph with no order, a reasoning-state fixed point (the ancestor closure of a parent relation) reached by bounded iteration, and reordered literals that are equal by construction. |
| [production](examples/production/) | Language-1.2 production eligibility (SPEC.md §17.13): six declared roots across two modules beside formal-only theorems and a proposition-valued definition; an effect-free fixed-width root eligible on `rust-core` and `rust-std`, overflow-admitting roots over a non-recursive inductive and a non-escaping closure, a well-founded root whose termination evidence is erased, and allocation-admitting list roots eligible on `rust-std` only, including a generic dependency instantiated at `Nat`; each root's closure, effects, and dispositions are published as its eligibility report, and verification extracts the roots through Lean's compiler front end into the canonical compiler input. |
| [production-coverage](examples/production-coverage/) | Language-1.2 semantic preservation coverage (SPEC.md §17.17): 33 production roots across four modules that together exercise every runtime row of the production registry (§17.13), covering fixed-width arithmetic at every width, text, bytes, and decimal parsing and formatting, records, options, generic and nested inductive types, higher-order functions with captures, structural, mutual, and well-founded recursion, and every collection template and key order; verification extracts every root and checks its certificate A. |
| [models](examples/models/) | Language-1.2 models (SPEC.md §17.12) across nine modules: a stateful deterministic session with an invariant, a rule policy, a statistical triage and an integer feed-forward digit recognizer whose weights are content-addressed artifacts embedded with kernel-checked decodings, stateful ledgers in every contract shape whose evidence leaves runtime checks open, every composite form over stateless and stateful stages (stateful sequences with proved and checked junctions, stateful fan-out, product, and branch, and stages checked after they run), every claim kind discharged by statement-exact theorems and restated against the fixed model semantics, every runtime refusal decided by the kernel, and production roots that apply models through their required runtime checks, extracted through Lean. |
| [reasoning](examples/reasoning/) | Language-1.2 reasoning machines (SPEC.md §17.12) across five modules: a clinical forward expert system whose eight guarded rules derive a multi-step triage with its invariant and termination proved, a generic terminating countdown, a deduplicating breadth-first jug planner, a depth-first search and a generate-and-verify reasoner over a model's proposals, an answer proved correct under the logic's invariant with its check erased, forged traces and inapplicable steps refused at run time, the six-counter ledger of each run and every generated theorem checked by the kernel, and production roots extracted through Lean and transcribed to calculus programs the compiler project checks against the reasoners themselves. |
| [fold-laws](examples/fold-laws/) | Language-1.2 program equalities over all inputs (SPEC.md §17.12, *Fold laws*) across three modules: a clamped requantization step monotone and bounded for every `Int`, truncating division by positive and negative literals, order and equality over `UInt8` and `Int8`, additive folds over lists, sets, and maps equal as sums (doubling, counting, weighting, append in either order, reverse, debits against credits, a map's keys and values), a single-pass statistics fold and a fused parity fold proved by the `law` form, and `decide` only on closed instances. |
| [helpdesk](examples/helpdesk/) | A text application written entirely from language-1.2 source and the source packages `std`, `std-models`, and `std-reasoning` (SPEC.md §17.22, `EX-09`): `dispatch : Bytes -> Bytes` trims the request with `Text`, scores it with an integer feed-forward model whose weights are content-addressed artifacts applied through its required precondition check, abstains with `decideOr` below a margin, commits through the generic `GateModel`, and plans the reply with the `Drain` reasoner; `classify` uses the model alone and `plan` the reasoner alone; the roots are exported with the packages committed under `tests/production-export/helpdesk/`. |
| [uor-atlas](examples/uor-atlas/) | The complete native Atlas declaration and proof graph, including the census/group chain, `S37`, `S38`, and the authoritative integer-uniqueness statement `S43`; no handwritten Atlas module is a generated dependency. |

Three further projects are written in LexLean itself and pass the same gates as an example. [compiler/](compiler/) defines the production realization calculus (SPEC.md §17.14), its `Gnaf` module states GNAF requests over that calculus (§17.15), with the complete-system universe, machine contract, and objective order all fixed before any optimizer and a theorem checked by the kernel that the universe is complete, and its `Certification` module checks GNAF certificates (§17.18) under [compiler/certificates/](compiler/certificates/): a global optimum, an argmin, or a complete Pareto frontier is certified from evidence for every system of the universe rather than by enumerating it, with the soundness of the checker proved in [compiler/proofs/](compiler/proofs/); local normality, correctness, and representation-minimality are separate, weaker statuses, and an incomplete certificate names its first open obligation. The requests live under [compiler/gnaf/](compiler/gnaf/), [compiler/gnaf.manifest.json](compiler/gnaf.manifest.json) is the dependency manifest UOR-GNAF §20 requires, and the target fixtures' Rust packages (§17.16) live under [compiler/rust/](compiler/rust/). [stdlib/](stdlib/) is the standard library, and [bootstrap/](bootstrap/) holds the compiler's own components beside the Rust they replace, the stage-0 oracle.

The larger [native Atlas source graph](examples/uor-atlas/src/Atlas.lex.tex) is rooted by `Atlas.lex.tex` and uses the generic closed-core form for lossless formal-library migration. Its module-local typed expression DAGs, declarations, definition bodies, proofs, inductive metadata, instances, and explicit axiom policies are linked semantic data. The same values generate readable canonical LaTeX and reconstruct kernel-checked Lean declarations. The one-time conversion and exact comparison are recorded in [MIGRATION.md](examples/uor-atlas/MIGRATION.md); the independently authored implementation is absent from the release tree. `VR-19` permanently requires one generated Lean module per native source module, public imports confined to `Init` and the generated graph, only the generic `Lean` backend-support import, and no second Atlas implementation.

## Building and running the gate

Prerequisites: Rust 1.97.1 (`rust-toolchain.toml` pins it), [just](https://github.com/casey/just), `cargo-deny`, and the pinned Lean toolchain `leanprover/lean4:v4.32.1` installed through `elan` (the dev container in [.devcontainer/](.devcontainer/) provides all of them).

```text
just vv        # the complete normative acceptance gate (SPEC.md §9.2)
just release   # vv, then the §30 release criterion; refused until 1.0.0
```

All 457 registered conformance IDs are implemented and pass; `just vv` runs clean from a checkout with the pinned toolchain installed.

The differential against the independent generator `lean4-prod` (SPEC.md §17.19, `OR-01`..`OR-08`) acquires it at its pinned commit: from the checkout `LEXLEAN_ORACLE_DIR` names, or by fetching the commit with `git` into the Cargo target directory (`LEXLEAN_ORACLE_CACHE` overrides), and builds it with the pinned Lean and Rust toolchains. It is validation evidence only and no input to any identity. An oracle that cannot be acquired, is not the pinned files, or fails is unavailable: that fails the gate on Linux x86-64 and is printed as `NOT VALIDATED` elsewhere, and is never reported as agreement.

`just vv` is the Linux x86-64 gate. On the other four supported hosts (§8.3) the crate builds and every test runs. A case whose assertions need something the host does not have runs its platform-independent assertions and prints which ones it skipped: the pinned toolchain, a `#!/bin/sh` program for the external-provider cases, a filesystem that distinguishes two names differing only in case, or one that accepts a name that is not valid UTF-8. Each is detected at run time rather than assumed from the target triple, and on Linux x86-64 the toolchain gate is mandatory, so nothing there passes vacuously.

Releases live on release lines (SPEC.md §9.4): `release/vX.Y` is the branch of the release X.Y, cut from `main` when its work begins and the target of its pull requests; tags are made from it, it is merged forward into the next line after each landing, and `main` is the newest released line. A push to a release line runs the same gate as a push to `main`.

Versions follow SPEC.md §30.1 and are recorded in [CHANGELOG.md](CHANGELOG.md). §2.3 fixes `0.1.0` as the initial implementation version and `1.0.0` as the first release satisfying the complete specification, so no tag before `1.0.0` claims the §30 release criterion: `cargo xtask release-check` reads that criterion and says exactly which parts of it do not hold. The release's evidence is the gate run of the tagged commit (SPEC.md §9.4, §30.3), so the publish job cannot refuse what the rehearsals accepted.

## Evidence, not belief

* `check` and `build` never claim verification (`VR-18`); only `verify` runs Lean, and its attestation records toolchain hashes, process records, and observed axiom sets (`VR-01`..`VR-14`).
* Facts about external tools (Lean 4.32.1, Lake, leanchecker, `#print axioms` output shapes) are level `some-true` rows in [`model/ledger.toml`](model/ledger.toml): reproduced from cited authorities, not established here.
* The acceptance gate is `just vv` (SPEC.md §9.2); a release is refused until the complete §30 criterion holds (`RP-12`).

## Documented deviations

* Generated Lean declares `public theorem` / `@[expose] public def` and `public import` where SPEC.md §18.1/§29.3 print bare `theorem` and `import`: under the Lean 4.32.1 module system a non-`public` declaration is module-private (the §18.9 axiom-audit module could not name it), a non-`public` import may not contribute constants to a public declaration's signature, and a definition body is hidden from importing modules unless exposed (a theorem in another module could not unfold or eliminate a document definition). The committed oracles carry `public`.
* The §21.5 `modules/<full-module-path>` artifact naming is realized as slash-separated directories (`modules/LexLeanExample/Main.lean`).
* Unique existence (§18.4 names `ExistsUnique`) lowers to its definitional expansion `Exists (fun (x : T) => And (P) ((y : T) → P[x:=y] → Eq y x))`: Lean 4.32.1's `Init` has no `ExistsUnique` constant. The linked IR keeps `ExistsUnique`; only the printed Lean bytes expand it, and a `Witness` step leaves the `And` goal for the remaining proof.
* The §18.8 probe module declares alpha-renamed universe variables with one `universe p0u ...` command before its `example` lines: Lean 4 has no `example.{u}` form.
* The pinned `leanchecker` has no version flag; its attestation `version_output` is the normalized answer to the fixed identity probe `lake env <leanchecker> LexLeanIdentityProbe` (the preflighted executable by absolute path), checked against the pinned toolchain's exact response.
* §23.5's "two spaces per environment nesting level" is realized as two spaces per *section* depth: the §29.2 literal indents nothing inside `theorem` or `proof`, so declaration and proof environments do not add a level; only `\begin{section}` nesting does (`LX-14`, and every example's committed source is `fmt --check` clean).
* A numeral with a redundant leading zero (`007`) is rejected as noncanonical decimal source (`LLL1003`, with the canonical spelling as help) although §12's lexical class admits any digit run: canonical source has one spelling per value, and the formatter cannot choose between two.
* §18.4 says generated numerals carry an expected type, and the §29.3 literal prints `Nat.add llv0 0` bare. Generated Lean prints a numeral bare exactly where the applied signature binder is a monomorphic constant type and ascribes it (`(0 : Nat)`) everywhere else, so the literal and the rule agree. The ascription is what a document type definition is *defined as*, in whichever module of the project declares it (§17.7), never the definition's own name: Lean's `OfNat` instances live on the underlying type.
* §20.4's fallback for a Lean diagnostic that no generated mapping encloses ("the declaration component") is realized as the nearest preceding declaration-role mapping of the generated module: a location after the last declaration (the closing `end`) remaps to that last declaration, with the unmapped generated range kept as a note.
* Fixture directories are named `tests/fixtures/<suite>/<id>-<slug>/` and `tests/negative/<class>/` (`cl-04-check-no-artifacts`, `vr-11-exact-mismatch`) where §28.2 writes `tests/fixtures/<suite>/<id>/`: one ID may own several fixtures, and the slug names which; the runner discovers fixtures by their `case.toml`, never by directory name, and every fixture runs from a temporary copy of its committed `project/`.
* A nested proof scope (a `constructor` branch, an `apply` premise, a `cases` or `induction` case) is set inside a `quote` environment. §19.5 fixes the phrases, not the layout, and a flat rendering makes two proofs that differ only in how their branches nest render to the same bytes; the indentation says where each scope closes, so the document presents the proof IR faithfully (§6 I8).
* Canonical LaTeX renders a quantified proposition as prose only in trailing position (the source formatter's §15.6 rule); a quantified operand that must be an island states its binder types (`\exists m \in \mathbb{N}, ...`), which the source math grammar has no spelling for. Document references no visible entry names render as `\texttt{Module::component}`, the escape form of qualified selectors, under their own coverage origin.
* Every identifier-shaped name canonical LaTeX emits (a display spelling or math identifier, a module segment, an LRE operator name, a glossary surface) is emitted through the registered `\_` token wherever it carries `_`, which TeX would otherwise read as a subscript. §12.2 admits `_` and `'`, §15.1 admits `_` in a module segment, and §13.9 admits `_` in an LRE operator name. The escape carries its own renderer-token coverage origin and the runs around it keep the name's, so §19.6 output coverage stays exact.
* §13.5 rule 5 admits a non-ASCII scalar in a canonical *source* form, but the canonical document emits glyphs only through the renderer-token registry (§19.1, §13.10), which the fixed §19.2 preamble can typeset. An entry whose LRE renders such a surface directly instead of naming a token is refused with `LLB6002` naming the entry, the form, and the scalar; every shipped core and standard entry with a Unicode surface already names one.
* §17.8 fixes generated local names as `llv<n>` / `llh<n>` in introduction order. A binder that the generated declaration never references keeps its index and carries a `_` prefix (`_llv1`, `_llh0`): pinned Lean's `unusedVariables` linter warns about an unreferenced binding, §20.2 makes any Lean warning a verification failure, and §18.1 fixes the file structure so no `set_option` may silence it. Without the prefix an ordinary proposition with an unused quantified binder (`For every natural number \(n\) and natural number \(m\), \(n + 0 = n\)`) could never verify. The prefix marks the binding as deliberate; nothing else changes.

All are enforced by the same golden and conformance gates as everything else.

## Layout

Language data lives in [language/](language/), claim data in [model/](model/), schemas in [schemas/](schemas/), the compiler in [crates/lexlean/](crates/lexlean/), gates in [xtask/](xtask/) and [crates/conformance/](crates/conformance/), the examples in [examples/](examples/), and the negative fixtures in [tests/negative/](tests/negative/). The projects written in LexLean itself are [compiler/](compiler/) (the production calculus, GNAF, and certification), [stdlib/](stdlib/) (the standard library), and [bootstrap/](bootstrap/) (the compiler's own components, beside the Rust they replace, the stage-0 oracle). The pinned differential oracle is described in [oracle/](oracle/) as validation evidence only, the schemas of the stable API and the tooling interface are under [api/](api/) and [tooling/](tooling/), and the README's own logo is [assets/lexlean-logo.svg](assets/lexlean-logo.svg). CI is in [.github/](.github/).

## License

Dual-licensed under [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.
