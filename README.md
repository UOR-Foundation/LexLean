<div align="center">
<pre>
 _               _
| |             | |
| |     _____  _| |     ___  __ _ _ __
| |    / _ \ \/ / |    / _ \/ _` | '_ \
| |___|  __/>  <| |___|  __/ (_| | | | |
|______\___/_/\_\______\___|\__,_|_| |_|
</pre>

<strong>One controlled language document in. A canonical paper, a kernel checked Lean 4 program, and a certified Rust package out.</strong>
</div>

LexLean compiles `.lex.tex` modules written in a closed controlled language. Every word, symbol, and control must be declared by a versioned lexicon package. Every sentence parses under a fixed grammar. One typed intermediate representation generates both the canonical LaTeX document and the prose free Lean 4 program.

`lexlean verify` compiles the generated Lean under the pinned `leanprover/lean4:v4.32.1` toolchain. It replays every module with `leanchecker`, audits axioms by parsing `#print axioms` exactly, and publishes a content addressed attestation.

**Evidence, not belief.** `check` and `build` never claim verification. Only `verify` runs Lean. Whatever it cannot check is reported as not validated, never passed in silence.

The complete normative contract is [SPEC.md](SPEC.md). The generated register is [CONFORMANCE.md](CONFORMANCE.md). The closed diagnostic registry is [ERRORS.md](ERRORS.md). The falsifiability evidence is in [VERIFICATION.md](VERIFICATION.md). The changelog is [CHANGELOG.md](CHANGELOG.md). The MNIST model is at [UOR Foundation/MNIST](https://github.com/UOR-Foundation/MNIST).

## Quickstart

### Run in the container

`lexlean verify` runs the pinned toolchain, so the published image carries the compiler and `leanprover/lean4:v4.32.1` together. This is the shortest path to a run that can actually verify:

```text
docker pull ghcr.io/uor-foundation/lexlean:0.4.0
docker run --rm -v "$PWD:/work" ghcr.io/uor-foundation/lexlean:0.4.0 verify
```

The image is around 4 GB. Nearly all of it is the pinned toolchain's compiled library, the part `verify` needs. A smaller image would have to leave it out.

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

From that one source LexLean generates the prose free Lean module:

```lean
module
public import Init
set_option autoImplicit false
namespace LexLeanExample.Main

public theorem add_zero (llv0 : Nat) : Eq (Nat.add llv0 0) llv0 := by
  rfl

end LexLeanExample.Main
```

It also generates the canonical LaTeX document from the linked representation, plus source maps, coverage, and a manifest. All outputs are byte reproducible across directories. `verify` then compiles the module under the pinned toolchain, replays it with `leanchecker`, parses `#print axioms` exactly, and records the empty observed axiom set in a content addressed attestation.

## How LexLean works

```text
your document (.lex.tex, closed lexicon)
        |
        v
one typed semantic representation (the single source of truth)
        |
        +====> canonical LaTeX (paper or PDF)
        |
        +====> prose free Lean 4 (kernel checked, exact axiom audit)
                     |
                     v
              production compilation
                     |
                     v
              certified Rust package
              + certificates A / B / E + attestation
              (replays offline, without the source)
```

* `lexlean init` scaffolds a project. It claims nothing.
* `lexlean lock` pins lexicons and dependencies in a canonical lock file. It claims nothing.
* `lexlean check` checks lexical closure, grammar, typing, and diagnostics. It never claims verification.
* `lexlean build` generates Lean, LaTeX, source maps, and manifests. It never claims verification.
* `lexlean verify` compiles under the pinned toolchain, replays with `leanchecker`, audits axioms, and attests. It is the only verified claim.
* `lexlean production build` exports certified Rust packages with their certificates. It claims kernel checked preservation.
* `lexlean migrate --to 1.2` upgrades a language 1.1 project, proved in a scratch copy first. It writes nothing on refusal.

A language 1.2 project can import the modules of a locked, content addressed package of its workspace instead of copying them. `[[source_package]]` in `lexlean.toml` names a member directory listed by `lexlean-workspace.toml`. `lexlean.lock` pins the package by the digest of its manifest and source files. Every command recomputes the pin. Imports are closed to the project's own modules and the declared packages. `verify` replays the package's modules under its module prefix with the package as their origin (SPEC.md §10.5, `PK-01`..`PK-05`). The workspace the cases use is [tests/packages/workspace](tests/packages/workspace/).

Language 1.1 semantic modules use the closed core backend's fixed finite Lean budgets (`maxRecDepth = 100000`, `maxHeartbeats = 1000000000`) so complete nested byte data declarations are not restricted by interactive defaults. Project child timeout and output limits and full kernel and axiom verification remain mandatory. These are generated backend settings, not caller provided options.

## Why LexLean

The paper cannot drift from the program. Both are generated from one linked representation. Neither is copied, neither is hand maintained, and a change to one is a change to the other. Assumptions are audited, not assumed. An attestation records the toolchain hashes, the process records, and the exact observed axiom set of every module. The binary carries its guarantees. Production compilation to Rust emits certificates that a consumer replays offline, without the source, without Lean, and without trusting the producer.

## When to use LexLean

Safety critical code that must be proved, not sampled. Total programs with machine checked termination, exact failure typing, and a certified Rust output. Audited or regulated artifacts. Every claim is a content addressed record: what was checked, by which toolchain, under which assumptions. Documents whose formal content must match their prose. The canonical document is regenerated from the same representation that the kernel checks. Verified models and reasoners. Integer neural networks, rule based reasoning machines, and text classifiers as closed semantic objects. The [MNIST model](https://github.com/UOR-Foundation/MNIST) is one: an int8 digit recognizer whose weights are content addressed artifacts, whose forward pass is checked layer by layer, and whose contract for every input is a theorem.

## Capabilities

Every capability is a claim in [`model/ids.toml`](model/ids.toml) at honesty level `build`. It is constructed here and validated against its oracle by the test named `conformance_<id>`. Every row is validated by `just vv`, and its IDs link the claim to its register row, scenario, and conformance test.

The 457 registered conformance IDs cover repository identity, configuration, lexical closure, lexicons, grammar, typed IR, declarations, proofs, Lean generation, LaTeX regeneration, diagnostics, verification, CLI, security, production eligibility, named root extraction, realization calculus, Rust backend, GNAF, certification, semantic preservation, models, differential oracle, production export, reasoning machines, compiler tooling, authoring notation, runtime, standard library, bootstrap, source packages, examples, and text components. See [CONFORMANCE.md](CONFORMANCE.md) for the full register and [model/ids.toml](model/ids.toml) for every individual ID with its scenario and test.

## Examples that verify

Every directory under [examples/](examples/) is discovered by the example gate (`EX-08`). Each must format, lock, check, build, and verify with real Lean 4.32.1 (`cargo xtask verify-examples`). Platform independent build outputs and normalized verification records are committed under `expected/` and compared byte for byte by the golden gate (§28.3) and the example gate (§29.5). That those bytes are also independent of where the build ran is the separate claim of `AR-13` and `EX-06`.

The `nat-add-zero` example is the literal SPEC.md §29 document. Other examples cover Peano arithmetic, propositional logic, list induction, semantic 1.1, language 1.2, recursive data, higher order functions, recursion, collections, production, production coverage, models, reasoning, fold laws, helpdesk, and uor atlas. Three further projects are written in LexLean itself and pass the same gates: [compiler/](compiler/) defines the production realization calculus and GNAF certification; [stdlib/](stdlib/) is the standard library; [bootstrap/](bootstrap/) holds the compiler's own components beside the Rust they replace.

## Building and running the gate

Prerequisites: Rust 1.97.1 (`rust-toolchain.toml` pins it), [just](https://github.com/casey/just), `cargo-deny`, and the pinned Lean toolchain `leanprover/lean4:v4.32.1` installed through `elan`. The dev container provides all of them.

```text
just vv        # the complete normative acceptance gate (SPEC.md §9.2)
just release   # vv, then the §30 release criterion; refused until 1.0.0
```

All 457 registered conformance IDs are implemented and pass. `just vv` runs clean from a checkout with the pinned toolchain installed.

The differential against the independent generator `lean4-prod` (SPEC.md §17.19, `OR-01`..`OR-08`) is validation evidence only and no input to any identity. An unavailable oracle fails the gate on Linux x86 64 and prints `NOT VALIDATED` elsewhere. It is never reported as agreement. `just vv` is the Linux x86 64 gate. On the other four supported hosts (§8.3) the crate builds and every test runs. Platform dependent assertions are skipped and printed, detected at run time. On Linux x86 64 the toolchain gate is mandatory.

Releases live on release lines (SPEC.md §9.4). `release/vX.Y` is cut from `main`, targets its pull requests, and is merged forward into the next line. `main` is the newest released line. A push to a release line runs the same gate as a push to `main`. Versions follow SPEC.md §30.1 and are recorded in [CHANGELOG.md](CHANGELOG.md). §2.3 fixes `0.1.0` as the initial version and `1.0.0` as the first release satisfying the complete specification. No tag before `1.0.0` claims the §30 criterion. `cargo xtask release-check` reads that criterion and says what does not hold. The release evidence is the gate run of the tagged commit.

## Evidence, not belief

`check` and `build` never claim verification (`VR-18`). Only `verify` runs Lean. Its attestation records toolchain hashes, process records, and observed axiom sets (`VR-01`..`VR-14`). Facts about external tools (Lean 4.32.1, Lake, leanchecker, `#print axioms` output shapes) are level `some-true` rows in [`model/ledger.toml`](model/ledger.toml), reproduced from cited authorities and not established here. The acceptance gate is `just vv` (SPEC.md §9.2). A release is refused until the complete §30 criterion holds (`RP-12`).

## Documented deviations

Generated Lean declares `public` declarations and imports where SPEC.md §18.1 and §29.3 print bare forms. Under the Lean 4.32.1 module system a non public declaration is module private. A non public import may not contribute constants to a public declaration's signature. A definition body is hidden from importing modules unless exposed. The committed oracles carry `public`.

Artifact naming uses slash separated directories (`modules/LexLeanExample/Main.lean`) instead of the §21.5 module path. Unique existence lowers to its definitional expansion because Lean's `Init` has no `ExistsUnique` constant. The linked IR keeps `ExistsUnique`; only the printed Lean bytes expand it. A `Witness` step leaves the `And` goal for the remaining proof. The pinned `leanchecker` has no version flag, so its attestation uses a fixed identity probe.

Indentation uses two spaces per section depth. The §29.2 literal indents nothing inside `theorem` or `proof`, so only `\begin{section}` nesting adds a level. Redundant leading zeros are rejected as noncanonical decimal source (`LLL1003`). Generated numerals are ascribed except where the binder is a monomorphic constant type. Nested proof scopes use a `quote` environment. Identifier shaped names with `_` are emitted through the registered `\_` token. Non ASCII surfaces must name a renderer token. Unused binders get a `_` prefix to satisfy Lean's linter. Fixtures are discovered by their `case.toml`, not by directory name.

All deviations are enforced by the same golden and conformance gates as everything else.

## Layout

Language data lives in [language/](language/). Claim data lives in [model/](model/). Schemas are in [schemas/](schemas/). The compiler is in [crates/lexlean/](crates/lexlean/). Gates are in [xtask/](xtask/) and [crates/conformance/](crates/conformance/). Examples are in [examples/](examples/). Negative fixtures are in [tests/negative/](tests/negative/). The projects written in LexLean itself are [compiler/](compiler/) (the production calculus, GNAF, and certification), [stdlib/](stdlib/) (the standard library), and [bootstrap/](bootstrap/) (the compiler's own components beside the Rust they replace). The pinned differential oracle is described in [oracle/](oracle/) as validation evidence only. The schemas of the stable API and the tooling interface are under [api/](api/) and [tooling/](tooling/). CI is in [.github/](.github/).

## License

Dual licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.
