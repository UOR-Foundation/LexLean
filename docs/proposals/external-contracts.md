# Contracts for code LexLean did not generate: design

Draft for owner acceptance, written 2026-11-12, against `release/v0.4`. Normative text is `SPEC.md`; this document explains and asks. It is a proposal only: this pull request adds this one file. The mechanism it recommends changes the trust model of SPEC §4.2 and §5, which is the owners' decision, so nothing is implemented until they choose. The mechanism was prototyped in the pinned Lean 4.32.1 with scratch files outside the repository; the source and what Lean printed are in section 7.

This is gap B of three on the way to a verified inference engine written as a LexLean application. Gap A (arrays of fixed-width integers, `SM-39`) is implemented in its own pull request. Gap C (floating point) is a proposal in its own pull request and relies on this one for native execution.

## 1. Summary

A fast engine is hand-written code LexLean did not generate: SIMD, WebAssembly, a C kernel. Today a LexLean document cannot mention it. The only choices are to leave it out of the model, so nothing is proved about the system that runs, or to rewrite it in LexLean, so it is no longer fast.

The proposal is one new declaration, `external`, that names a foreign function, states its contract, and makes the assumption that the function meets the contract a visible, named, audited fact:

1. The declaration states a signature, a contract (a proposition over the inputs and the output), a **witness** (a LexLean definition with a theorem that it meets the contract), and **evidence** (what discharges the assumption for the real code).
2. Generated Lean declares an opaque constant whose hidden value is the witness, and one axiom, `<Module>.<name>.contract`, that states the contract of that constant.
3. A caller is proved against the contract and nothing else: the kernel cannot see through the constant.
4. Every theorem that uses the contract has that one axiom in its observed set. LexLean's policy is exact, so the theorem must list it, and `verify` reports it with its evidence. A theorem that does not use the external function does not carry it.
5. No other axiom can be written. The ban on author axioms stands for everything else, and the one axiom this form generates cannot be inconsistent, because the witness proves a function with that contract exists.

**Prototype result.** In pinned Lean, a caller `score` of an external int8 dot product `dotFast` was proved correct against the contract. `#print axioms` on the caller's theorem printed `[propext, Engine.Main.dotFast.contract]`; on a theorem about the reference alone it printed that it depends on no axioms; and `rfl` could not prove the external function equal to its witness (section 7).

Decisions that need the owners:

| # | Decision | Recommendation |
| --- | --- | --- |
| D1 | Whether a document may state an assumption about foreign code at all | Yes, in this one closed form |
| D2 | Mechanism | An opaque constant plus one generated contract axiom (option A of section 5). It is the only option where Lean itself reports every use |
| D3 | Whether a witness is mandatory | Yes. Without it the form is an author axiom under another name |
| D4 | Evidence kinds | `attested` (a content-addressed conformance report bound to the contract's statement ID) first; `proved` (a kernel-checked theorem about a Lean model of the code) when Lean-library imports land (#362) |
| D5 | Project-level opt-in | Yes: `[[external_source]]` in `lexlean.toml`, pinned in the lock, so a reviewer sees every external in the configuration |
| D6 | Emitting `@[extern "symbol"]` so that Lean-compiled C calls the foreign symbol | Not in the first change. Decide the symbol and calling convention with PrismPM first; until then the compiled constant runs the witness |
| D7 | Production (certified Rust) | Formal-only at first, like arrays and text components |
| D8 | An Aeneas bridge | Not now. It is sound in principle and not small (section 8) |

## 2. What the goal asks and where it stands

| Asked | Mechanism here | State |
| --- | --- | --- |
| Declare an external function with a stated contract | `external` declaration | Designed; prototyped in Lean; not implemented |
| A model can call it and proofs can use the contract | Ordinary call; `apply` or `simplify` naming `<name>.contract` | Prototyped |
| No hole | Mandatory witness theorem; closed form; no free axiom text | Argued in section 6 |
| Every use visible in the observed axiom or trust set | The exact axiom audit, unchanged | Prototyped: `#print axioms` shows the contract axiom |
| Reported by `verify` | Attestation gains trust records; summary line | Designed |
| Names the evidence that discharges it | `evidence: attested{...}` or `proved{...}` | Designed |
| Does not weaken the ban for everything else | Only this declaration can generate an axiom, of one fixed shape | By construction |
| Example: external int8 dot product, caller proved, trust entry shown | `examples/external-contract` | Prototype in Lean only (section 7) |

## 3. What exists

- **Contracts and models** (§17.12). A `contract` has typed input and output and proposition-valued predicates with validators; a `model` binds a contract to a `realization`, which is always a LexLean denotation. `checked_apply` runs validators at run time and returns a `ContractViolation` on refusal. This covers code LexLean generates, and results that are cheap to check. It does not cover a result that is as expensive to check as to compute, which is the case for a dot product.
- **External Lean constants** (§4.1, §13.7). A lexicon entry of kind `lean` names a constant of the Lean workspace with a declared signature, probed at verification. This imports Lean, not foreign code, and the workspace is in the trusted base (§5.1).
- **Lean-library imports** (#362, draft on `release/v0.5`), with a feasibility probe for Aeneas or Hax under #356.
- **The ban** (§4.2): no author-defined axioms, no raw Lean. Every declaration has an exact axiom policy; `verify` refuses a missing, additional or unlisted observed axiom (§17.11, §22).

## 4. Constraints taken as given

- §5.4: the axiom audit reports what `#print axioms` returns, nothing more. A mechanism Lean does not report would need a second audit that LexLean alone vouches for.
- §5.3: "verified" is only said of what Lean elaborated, `leanchecker` replayed and the audit accepted.
- AGENTS.md rule 4: a claim about a dependency belongs to that dependency. A claim about hand-written code belongs to whoever attests it, and must be named as such.
- R4: nothing deferred; R5: every failure registered.

## 5. Options

**A. Opaque constant and one generated contract axiom (recommended).** Described in section 6. Lean reports every use. One new kind of axiom exists, generated only by this form.

**B. Parameterised theorems, no axiom.** Every definition that reaches the external function takes the implementation and a proof of its contract as parameters; theorems are quantified over them. Nothing is assumed in Lean at all. The cost: every caller, transitively, changes its signature, a production root cannot be closed, and the "which implementation is running" fact moves entirely outside Lean, where only LexLean's own bookkeeping reports it.

**C. Check every result at run time.** This is `checked_apply` today. It needs no trust. It is the right answer whenever checking is cheaper than computing, and it is useless for an inference kernel, where the check is the computation.

**D. Reference body with an unaudited binding.** The external function is defined as the reference, and a compiler attribute (`@[extern]`, `@[implemented_by]`) substitutes foreign code. This is how Lean's own core binds native code. No axiom appears, so `#print axioms` shows nothing: every theorem looks unconditional while the running code is trusted silently. This is the hole the request rules out.

A is recommended because it is the only one in which the trusted assumption is an object of the logic that the pinned Lean itself tracks through every dependency, with no new audit to trust.

## 6. Design of option A

### 6.1 The declaration

In the authoring notation:

```text
external{
  name: dotFast,
  parameters: [left : array{element: Int8}, right : array{element: Int8}],
  result: Int,
  contract: DotContract,
  witness: {definition: dotReference, theorem: dot_reference_meets},
  evidence: attested{report: dotFastConformance},
  symbol: "engine_dot_i8"
}
```

- `contract` names a prior proposition-valued definition whose parameters are the external function's parameters followed by its result.
- `witness.definition` names a prior LexLean definition of exactly the external function's signature. `witness.theorem` names a prior theorem whose statement is exactly the contract applied to the parameters and to the witness applied to them. Linking checks both shapes; Lean checks the theorem.
- `evidence` is one of the closed forms of section 6.4.
- `symbol` is the foreign name, recorded in the build outputs for a host to bind (D6).

### 6.2 Lowering

```lean
public opaque dotFast (left : Array Int8) (right : Array Int8) : Int := dotReference left right
public axiom dotFast.contract (left : Array Int8) (right : Array Int8) :
    DotContract left right (dotFast left right)
```

The hidden value of the constant is the witness. The kernel never unfolds an opaque constant, so a theorem cannot unfold `dotFast` to its witness, and cannot derive the intended computational contract from that body; the contract is available only through the axiom. Reflexivity (`dotFast l r = dotFast l r`) and hypotheses about `dotFast` remain available and carry no axiom. A document refers to the axiom as the member `dotFast.contract`, in `apply` and `simplify`, exactly as it refers to a theorem.

The canonical document states the declaration, the contract, the witness and the evidence. The generated-source audit admits `opaque` and `axiom` tokens only at the positions an `external` declaration of the IR accounts for.

### 6.3 Why this is not a hole

The axiom says: the constant `dotFast` satisfies `DotContract`. The constant's hidden value is `dotReference`, and `dot_reference_meets` is a kernel-checked proof that `dotReference` satisfies `DotContract`. So the axiom is a true statement about the constant; it is unprovable inside Lean only because the constant is opaque. Replacing `opaque` by a definition turns the axiom into a theorem with the witness theorem as its proof. Any proof that uses the axiom therefore stays valid with the axiom replaced by that theorem: the axiom adds no logical strength. In particular it cannot make `False` provable, whatever contract an author writes, because an unsatisfiable contract has no witness theorem and is refused at verification.

What remains assumed is not logical. It is the statement that the foreign code, where it runs in place of the constant, computes a function with that contract, and that it is a function: no dependence on anything but its arguments, no effect, termination on every input the contract admits. That statement is what the evidence is for, and it is named in every theorem that needs it.

### 6.4 Evidence

- `attested{report: A}`: `A` is an `artifact` declaration of role `conformance`, content-addressed like every artifact. Its bytes are a canonical JSON report that states the statement ID of the external declaration (`SM-38`), the digest of the foreign code tested, the reference it was compared with, the number of cases and the result. Linking refuses a report that names another statement ID, and the project row must pin the report's digest. The report's bytes and the contract's identity are thereby authenticated; that is all the hash and the statement ID do. Binding the report to what actually runs is a separate check, which `verify` makes and which refuses (`LLV7007`-class refusal, code to be registered with the implementation) unless all of these hold: the report's code digest equals the digest of the artifact the build selects for the declaration; the report's symbol, ABI and target equal the declaration's `symbol` and the build's target; the report's attester is one the `[[external_source]]` row names as trusted; and the report's result is `pass` with at least the case count the row requires. With D6 undecided there is no selected artifact, so until then `verify` reports the evidence as `unbound` and says so in the trust line. So a change of the contract, the signature or the witness makes old evidence stale by construction. LexLean does not run the foreign code and does not vouch for the report: the attestation records who is trusted for what. The witness (D3) establishes only that the logical contract is satisfiable; a finite conformance report does not establish universal native correctness, purity or termination, and no wording of this document claims it.
- `proved{theorem: T}`: `T` is a theorem, checked by the pinned Lean, that a Lean model of the foreign code meets the contract. This needs the model to be importable (#362) and belongs to a later change. It does not remove trust; it moves it to whatever produced the model (section 8).

### 6.5 Policy and reporting

- A project lists the externals it admits: `[[external_source]]` in `lexlean.toml` with the module, the name and the report digest, pinned in `lexlean.lock`. An `external` declaration without its row is refused. A project with no row accepts no external declaration, so the default is the ban as it is today.
- **Proof dependencies and executable uses are different facts.** The axiom audit (§5.4) reports what `#print axioms` returns: a theorem that uses `dotFast.contract` lists it. It is not a call graph. A definition can call the opaque `dotFast` without mentioning the axiom, and a reflexive or conditional theorem can mention the call without it. So the axiom set alone does not account for every runtime use of foreign code. The design therefore adds a second, fail-closed inventory, the **external-call inventory**: the linker computes, from the IR, every definition, directly or transitively, that calls an `external` function, and the attestation records it (`external_uses`) beside `trust`. A production or `verify` root that reaches an external call not covered by an admitted `[[external_source]]` row is refused. The inventory is computed by LexLean, not by Lean, and the trust lines say so; the axiom audit is not claimed to be it.
- Each theorem and definition states its exact axioms as now. A theorem that uses `dotFast.contract` lists `Project.Module.dotFast.contract` beside `propext`; a missing or extra one is `LLV7005`, unchanged.
- The attestation gains `trust`: for each contract axiom, its evidence, the declarations whose observed sets contain it (proof dependencies), and, from the external-call inventory, the declarations that call the function (executable uses). `verify` prints one line per contract axiom after its summary, for example:

```text
verified 1 module; attestation <id>
trust: Engine.Main.dotFast.contract, attested by sha256:<report>, proof-used by 1 theorem; called by 1 definition (call inventory: LexLean)
```

- A status line never says "verified" without the trust lines when any exist.

### 6.6 Diagnostics

Malformed declarations are `LLT4001` (contract or witness of the wrong shape, a witness theorem that states something else, a report bound to another statement ID, an external without its configuration row). A witness theorem Lean refuses is `LLV7002`. An unlisted contract axiom is `LLV7005`. No new code is needed for these; whether the missing-row refusal deserves its own code is a detail for the implementation.

## 7. Evidence: the prototype

Pinned Lean 4.32.1 (`f054605aea4b840552cca2e725580bffd1e1b704`), Windows x86-64, 2026-11-12, one scratch file outside the repository, written in the shapes the backend emits:

```lean
@[expose] public def DotContract (left : Array Int8) (right : Array Int8) (result : Int) : Prop :=
  result = dotList left.toList right.toList

@[expose] public def dotReference (left : Array Int8) (right : Array Int8) : Int :=
  dotList left.toList right.toList

public theorem dot_reference_meets (left : Array Int8) (right : Array Int8) :
    DotContract left right (dotReference left right) := rfl

public opaque dotFast (left : Array Int8) (right : Array Int8) : Int := dotReference left right

public axiom dotFast.contract (left : Array Int8) (right : Array Int8) :
    DotContract left right (dotFast left right)

@[expose] public def score (left : Array Int8) (right : Array Int8) (bias : Int) : Int :=
  dotFast left right + bias

public theorem score_correct (left : Array Int8) (right : Array Int8) (bias : Int) :
    score left right bias = dotList left.toList right.toList + bias := by
  have h := dotFast.contract left right
  simp only [DotContract] at h
  simp only [score, h]
```

What Lean printed:

```text
'Engine.Main.score_correct' depends on axioms: [propext, Engine.Main.dotFast.contract]
'Engine.Main.reference_only' does not depend on any axioms
'Engine.Main.dot_reference_meets' does not depend on any axioms
```

and, for `example (l r : Array Int8) : dotFast l r = dotReference l r := by rfl`:

```text
error: Tactic `rfl` failed: The left-hand side
  Engine.Main.dotFast l r
is not definitionally equal to the right-hand side
  Engine.Main.dotReference l r
```

So the caller is proved against the contract, the trust entry is exactly one named axiom in the caller's observed set, a theorem that does not use the external function does not carry it, and the kernel cannot reach the witness through the constant.

This is a Lean prototype of the lowering. It is not `lexlean verify` output: no `external` declaration exists in the compiler.

## 8. Could Rust translated to Lean by Aeneas discharge a contract?

Aeneas translates a subset of safe Rust to pure Lean definitions; a contract would then be a theorem about the translated function, and the evidence would be `proved`.

What was checked (2026-11-12, the project's repository at `main`):

- Its Lean backend requires Mathlib and pins `leanprover/lean4:v4.36.0-rc2`. LexLean pins 4.32.1, and the draft that moves the pin (#433) moves it to 4.33.1.
- Generated files import the `Aeneas` Lean package, so its declarations and Mathlib would have to be imported, pinned and audited (#362, with the 7 GB library cost noted there).
- It supports a subset of safe Rust. Unsafe code and concurrency are stated as unsupported; some loop forms are unsupported.
- The translation is described in two papers; the tool that performs it is not itself verified.

Assessment:

- **Sound?** As far as Lean is concerned, yes: the theorem is kernel-checked and the axiom audit reports whatever axioms the library uses. But the theorem is about the Lean model, not about the Rust. That the model is faithful is the translator's claim. So an Aeneas bridge does not remove the trust entry; it replaces "this code was tested against the reference" with "this translator is faithful and the compiled Rust behaves as its source", which must be named the same way.
- **Small?** No. It needs Lean-library imports to land, a toolchain where LexLean's pin and Aeneas's meet, Mathlib in the verification of every project that uses it, and a new evidence kind.
- **Useful for this engine?** Only for a scalar, safe-Rust kernel. The fast paths that motivate an external function use SIMD intrinsics, which are outside the supported subset.

Recommendation: do not build the bridge now. Keep `proved` in the design so it can be added without changing the declaration, and revisit under #356 when #362 has landed.

## 9. What it costs

Estimated from the arrays change, which touched the same layers:

- IR, linker rules, schema (module and snapshot), Lean backend, canonical LaTeX, generated-source audit: one new declaration kind.
- Axiom handling: admit a generated name in exact policies; the attestation's `trust` records; the summary lines.
- Configuration and lock: `[[external_source]]`.
- One artifact role and report schema.
- Production registry row (formal-only), oracle overlap row.
- Register rows: one for the declaration and its refusals, one for the trust report. An example, negative fixtures for each refusal, and planted defects: a witness theorem about another function, a report bound to another statement ID, a theorem that omits the contract axiom from its policy, an external without its configuration row.
- It moves the language-1.2 identity, so it needs one regeneration, and lands one at a time with the other identity changes on the line (arrays, #474).

## 10. Limits of the design itself

- The contract is about a mathematical function. Foreign code that reads a clock, keeps state, or does not terminate is outside what the axiom says, and evidence by testing cannot establish totality.
- Attested evidence is as good as the test that produced it. LexLean binds the report to the contract and to the code digest; it does not judge coverage.
- A witness is required. A contract for which no LexLean function can be written (for example one that only bounds an error) needs the reference written first. For an inference kernel the witness is the integer reference engine, which has to exist anyway.
- Evidence is witness-only until D6: the report is bound to the contract, not to a running artifact, and `verify` says `unbound`.
- Until D6 is decided the compiled Lean runs the witness, so the trust entry describes a binding a host makes, not one LexLean emits.

## 11. Not implemented by this PR

Everything. The file is the proposal. What would have to be true before work starts: the owners accept D1 to D3; arrays (`SM-39`) are on the line, because the example is over arrays; and the identity change is sequenced with #474.

## 12. Decisions that belong to the owners

### D1. An assumption about foreign code in a document

§4.2 bans author axioms so that a verified status means what Lean checked. This proposal keeps that sentence true by putting the assumption in the theorem's own axiom set, where the status already shows it. The alternative is that systems with a hand-written kernel are never modelled.

### D2. Mechanism

Section 5. Option A if the trust must be reported by Lean; option B if the owners prefer that no axiom of any kind is ever generated, at the cost of threading parameters through every caller.

### D3. Mandatory witness

Without a witness an unsatisfiable contract proves anything. With it, the form adds no logical strength (section 6.3).

### D4. Evidence kinds

`attested` first. The report format and the artifact role are small. `proved` waits for #362.

### D5. Project opt-in

A configuration row makes every external visible in a diff of `lexlean.toml` and the lock, and keeps the default at "none".

### D6. `@[extern]`

Lean's `@[extern "symbol"]` on the opaque constant makes Lean-compiled C call the symbol. Prism builds applications that way, so this is how the fast engine would actually run. It fixes a calling convention for arrays and integers that PrismPM has to agree to, and the generated-source audit forbids the token today.

### D7. Production

Formal-only at first. A Rust realization would be an `extern` item or a trait parameter and is a separate design.

### D8. Aeneas

Section 8.
