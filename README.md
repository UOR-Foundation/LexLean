<div align="center">
  <img src="assets/lexlean-logo.png" alt="LexLean" width="1200">

  <p><strong>One controlled language in. A canonical paper, a kernel checked Lean 4 program, and a certified Rust package out.</strong></p>
</div>

LexLean compiles `.lex.tex` modules written in a closed controlled language. Every word, symbol, and control comes from a versioned lexicon package. Every sentence follows a fixed grammar. One typed representation produces both the canonical LaTeX document and a Lean 4 program with no prose.

`lexlean verify` compiles the generated Lean with `leanprover/lean4:v4.32.1`, replays every module with `leanchecker`, audits axioms by parsing `#print axioms` exactly, and publishes a content addressed attestation. `check` and `build` never claim verification.

[Spec](SPEC.md) · [Conformance](CONFORMANCE.md) · [Diagnostics](ERRORS.md) · [Evidence](VERIFICATION.md) · [Changelog](CHANGELOG.md) · [MNIST model](https://github.com/UOR-Foundation/MNIST)

## Quickstart

```text
docker pull ghcr.io/uor-foundation/lexlean:0.4.0
docker run --rm -v "$PWD:/work" ghcr.io/uor-foundation/lexlean:0.4.0 verify
```

The image is 4 GB because it carries the pinned toolchain. Each [release](https://github.com/UOR-Foundation/LexLean/releases) attaches one binary per supported host, the packaged crate, a CycloneDX bill of materials, checksums for every asset, and the gate evidence for the tagged commit.

```text
tar -xzf lexlean-0.4.0-binaries.tar.gz
sha256sum -c checksums.txt
```

A binary runs `init`, `lock`, `fmt`, `check`, and `build`. `verify` also needs the pinned toolchain through [elan](https://github.com/leanprover/elan). From source:

```text
cargo install --locked --path crates/lexlean
```

Then:

```text
lexlean init --name my-doc --module-prefix MyDoc
lexlean lock
lexlean check
lexlean build
lexlean verify
```

This is the complete SPEC.md §29 example, committed as [examples/nat-add-zero/src/Main.lex.tex](examples/nat-add-zero/src/Main.lex.tex):

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

It generates a Lean module with no prose:

```lean
module
public import Init
set_option autoImplicit false
namespace LexLeanExample.Main

public theorem add_zero (llv0 : Nat) : Eq (Nat.add llv0 0) llv0 := by
  rfl

end LexLeanExample.Main
```

The same representation yields canonical LaTeX, source maps, coverage, and a manifest. Outputs are byte reproducible. `verify` checks the module, replays it with `leanchecker`, parses `#print axioms` exactly, and records the observed axiom set in a content addressed attestation.

## How it works

```text
document -> typed representation -> LaTeX + Lean 4 -> certified Rust
```

`lexlean init` scaffolds a project and claims nothing. `lock` pins lexicons and dependencies. `check` checks closure, grammar, typing, and diagnostics, never verification. `build` generates Lean, LaTeX, source maps, and manifests, never verification. `verify` compiles under the pinned toolchain, replays with `leanchecker`, audits axioms, and attests; it is the only verified claim. `production build` exports certified Rust packages. `migrate --to 1.2` upgrades a language 1.1 project in a scratch copy first and writes nothing on refusal.

A language 1.2 project can import a locked, content addressed package of its workspace. `[[source_package]]` in `lexlean.toml` names a member directory listed by `lexlean-workspace.toml`; `lexlean.lock` pins it by the digest of its manifest and source files. Every command recomputes the pin, imports stay closed, and `verify` replays the package's modules with the package as their origin (SPEC.md §10.5, `PK-01`..`PK-05`).

## Why LexLean

One linked representation feeds every output, so the paper and the program cannot drift. Every claim is a content addressed record: what was checked, with which toolchain, under which assumptions. Production compilation emits certificates that a consumer replays offline, without the source or a trusted producer.

Use it for safety critical code, audited or regulated artifacts, documents whose formal content must match their prose, and models or reasoners. The [MNIST model](https://github.com/UOR-Foundation/MNIST) is one: an int8 digit recognizer whose weights are content addressed artifacts and whose contract for every input is a theorem.

## Capabilities

Every capability is a claim in [`model/ids.toml`](model/ids.toml) at honesty level `build`. `just vv` validates every row by the test named `conformance_<id>`. See [CONFORMANCE.md](CONFORMANCE.md) for the full register.

Every directory under [examples/](examples/) must format, lock, check, build, and verify with Lean 4.32.1. Other suites cover arithmetic, logic, induction, recursion, collections, production, models, reasoning, fold laws, helpdesk, and uor atlas. [compiler/](compiler/), [stdlib/](stdlib/), and [bootstrap/](bootstrap/) are written in LexLean itself.

## Gate

Prerequisites: Rust 1.97.1, [just](https://github.com/casey/just), `cargo-deny`, and Lean 4.32.1 through `elan`. The dev container provides all of them.

```text
just vv
just release
```

All 462 registered conformance IDs are implemented and pass. `just vv` runs the complete normative acceptance gate (SPEC.md §9.2). `just release` runs `vv` and the §30 criterion; it is refused until 1.0.0.

The [`lean4-prod`](https://github.com/UOR-Foundation/lean4-prod) differential is validation evidence only. A missing oracle fails on Linux x86 64 and prints `NOT VALIDATED` elsewhere. Releases live on `release/vX.Y`, and a tag before 1.0.0 does not claim the §30 criterion.

## Evidence

`check` and `build` never claim verification. Only `verify` runs Lean. Its attestation records toolchain hashes, process records, and observed axiom sets. External facts are `some-true` rows in [`model/ledger.toml`](model/ledger.toml). The gate is `just vv`; release waits for §30.

Generated Lean declares `public` declarations and imports where Lean 4.32.1 requires them. Unique existence lowers to its definitional expansion. Leading zeros are rejected, numerals are ascribed except at monomorphic constant binders, nested proof scopes use `quote`, names with `_` escape as `\_`, non ASCII surfaces need a renderer token, and unused binders get a `_` prefix. Fixtures are found by `case.toml`. Every deviation is enforced by the same gates.

## Layout

Language data is in [language/](language/), claims in [model/](model/), schemas in [schemas/](schemas/), the compiler in [crates/lexlean/](crates/lexlean/), and gates in [xtask/](xtask/) and [crates/conformance/](crates/conformance/). Examples are in [examples/](examples/) and negative fixtures in [tests/negative/](tests/negative/). CI is in [.github/](.github/).

## License

Dual licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.
