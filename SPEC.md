# LexLean Repository and Project Specification

**Repository:** `https://github.com/afflom/lexlean`  
**Project name:** LexLean  
**Rust crate and executable:** `lexlean`  
**Specification identifier:** `LEXLEAN-SPEC-1`  
**Language identifiers:** `lexlean-language/1.0`, `lexlean-language/1.1`
**Project-schema identifier:** `lexlean/project/1`  
**Status:** Normative implementation specification  
**Target initial release:** `1.0.0`

---

## 1. Normative force

This document is the complete implementation contract for `github.com/afflom/lexlean`.

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, and **MAY** are normative. A behavior not authorized by this specification is not part of LexLean language 1.0, 1.1, or 1.2.

A LexLean implementation conforms to this specification only when:

1. every conformance ID in §31 is present in `model/ids.toml` at honesty level `build`;
2. every ID has exactly one scenario in its named `features/suites/*.feature` file;
3. every ID has exactly one Rust conformance test named `conformance_<id>`, where hyphens are changed to underscores and letters are lower-case;
4. every required positive and negative fixture passes;
5. `just vv` succeeds from a clean checkout;
6. all generated platform-independent artifacts match their committed oracles;
7. every verification fixture passes with the pinned Lean toolchain;
8. no public capability is represented as complete while any required branch, error path, artifact, or audit remains absent.

The repository MUST NOT ship a partial implementation under the LexLean 1.0 language identifier. Work may occur on branches with failing tests, but the default branch and every release tag MUST satisfy the entire acceptance gate.

---

## 2. Repository identity and initialization

### 2.1 Name

The repository name is **LexLean** and its canonical GitHub location is:

```text
https://github.com/afflom/lexlean
```

The Cargo package, library crate, and executable are all named `lexlean`.

The one-line description is:

> A closed-lexicon LaTeX-to-Lean 4 compiler whose canonical document and prose-free Lean program are generated from one semantic representation.

### 2.2 Template origin

The repository MUST be created from `UOR-Foundation/template`, using template commit:

```text
0a1c799338d7db829aa23365e1acf4f9d01ff8b5
```

The inherited claim model, conformance runner, falsifiable gate discipline, dual license, and `xtask` pattern MUST be retained. Domain-specific remnants from the repository from which the template was cut MUST be removed or rewritten; in particular, the inherited `audit-limits` allow-list is not applicable to LexLean.

### 2.3 Repository metadata

The root workspace metadata MUST be:

```toml
[workspace.package]
version = "0.3.0"
edition = "2021"
rust-version = "1.97"
license = "MIT OR Apache-2.0"
repository = "https://github.com/afflom/lexlean"
homepage = "https://github.com/afflom/lexlean"
authors = ["Alex Flom"]
keywords = ["lean", "latex", "formal-methods", "compiler", "proof"]
categories = ["compilers", "development-tools"]
```

The initial implementation version is `0.1.0`. The first release satisfying this complete specification is `1.0.0`.

### 2.4 License

The repository MUST remain dual-licensed under:

- Apache License 2.0; or
- MIT;

at the recipient's option. `LICENSE-APACHE` and `LICENSE-MIT` MUST remain at the repository root.

---

## 3. Purpose

LexLean compiles a closed, glossary-defined mathematical document language with a LaTeX surface into:

1. a typed semantic intermediate representation;
2. canonical human-readable LaTeX;
3. prose-free Lean 4 source;
4. source maps and complete lexical-coverage records;
5. deterministic build manifests; and
6. verification attestations produced only after Lean elaboration, kernel replay, and axiom-policy checking succeed.

The central property is:

> No accepted LexLean document contains an uninterpreted word, symbol, punctuation mark, control sequence, reference, or semantically relevant structural token.

The document is a program. Human-visible mathematical text and kernel-visible Lean declarations are two renderings of one linked semantic object.

---

## 4. Scope and non-goals

### 4.1 LexLean 1.0 supports

LexLean 1.0 MUST support:

- closed lexical resolution for all source tokens;
- a fixed controlled grammar for propositions and definitional statements;
- symbolic mathematical expressions with glossary-defined operators;
- document modules and explicit module imports;
- glossary packages and exact package locking;
- scoped section parameters, quantified binders, proof locals, and branch locals;
- nonrecursive type aliases, term definitions, and predicate definitions;
- theorem, lemma, and corollary declarations;
- structured proofs using the proof forms in §14;
- external Lean constants with declared LexLean signatures;
- canonical LaTeX rendering from the semantic IR;
- Lean 4 generation with no comments, strings, documentation text, or proof holes;
- verification in a pinned Lake workspace;
- per-declaration axiom-dependency policies;
- optional isolated PDF rendering through the external-provider protocol in §19;
- human and canonical-JSON diagnostics;
- a Rust library API and command-line interface;
- deterministic, content-addressed build artifacts.

### 4.2 LexLean 1.0 deliberately does not support

The following are outside the LexLean 1.0 language and MUST be rejected rather than approximated:

- arbitrary natural language;
- unrestricted LaTeX;
- TeX macro definition or macro expansion;
- raw Lean declarations, terms, tactics, commands, or code blocks;
- source comments of any kind;
- free expository paragraphs;
- author-defined axioms;
- `sorry`, `admit`, synthetic proof placeholders, or deferred proof nodes;
- recursive or mutually recursive document definitions;
- document-defined inductive types, structures, classes, or instances;
- string literals, file inclusion, shell escape, network access, or foreign code;
- heuristic ambiguity resolution;
- probabilistic or model-based translation;
- a claim that `leanchecker` is an independent proof checker;
- a claim that successful Rust generation alone establishes a theorem;
- a claim that imported libraries are proved by LexLean.

Complex Lean objects that LexLean 1.0 cannot define MAY be imported as closed external glossary entries. Their guarantees belong to their upstream repositories. LexLean verifies only that the generated declarations elaborate and check in the pinned environment and that their observed axiom dependencies satisfy their explicit policies.

---

## 5. Trust and evidence model

### 5.1 Trusted computing base

A verification attestation depends on:

1. the exact LexLean compiler semantics identified in the build manifest;
2. the pinned Lean 4 toolchain;
3. Lean's elaborator and kernel;
4. the `leanchecker` executable from that same toolchain, used as a separate-process replay check;
5. the imported Lean workspace and its locked dependencies;
6. the operating system, filesystem, and process implementation;
7. any external PDF engine only for PDF bytes, never for theorem verification.

### 5.2 Untrusted inputs

LexLean MUST treat all of the following as untrusted:

- `.lex.tex` source;
- project configuration;
- lock files;
- glossary packages;
- imported path-package contents;
- external tool output;
- existing build directories;
- generated files from previous runs.

Every untrusted input MUST be parsed, validated, bounded by the project's explicit resource policy, and either accepted under a specified schema or rejected.

### 5.3 Evidence states

LexLean distinguishes:

- **checked**: lexical, grammatical, resolution, linking, and LexLean semantic checks succeeded;
- **built**: checked, and canonical artifacts were emitted;
- **verified**: built, Lean elaboration succeeded, `leanchecker` replay succeeded, the axiom audit succeeded, and every declaration satisfied its policy.

The words “proved”, “verified”, “kernel-checked”, and equivalent status markers MUST NOT be emitted for a merely checked or built result.

### 5.4 Axiom audit scope

LexLean's axiom audit is specifically an **axiom-dependency** audit. It reports the transitive axioms returned by Lean's `#print axioms`. It is not a general dependency graph, a source provenance proof, or an independent semantic interpretation of imported code.

---

## 6. Global invariants

Every implementation path MUST preserve all of the following.

### I1. Lexical closure

Every accepted non-whitespace source atom is covered exactly once by a selected glossary entry, a core structural entry, a numeric constructor, or a scoped declaration.

### I2. Symbol closure

Mathematical symbols, punctuation, braces, delimiters, control sequences, and reference syntax are subject to the same closure requirement as prose words.

### I3. Grammatical closure

A sequence of known lexical items is not sufficient. The complete component MUST parse under the versioned controlled grammar.

### I4. Semantic closure

Every accepted parse constructs a defined semantic IR node. There is no opaque “text” node.

### I5. Unique interpretation

After lexical alternatives, parses, name resolution, and type-directed filtering are considered, exactly one distinct linked IR is required. Zero interpretations is an error. More than one distinct interpretation is an ambiguity error. LexLean MUST NOT guess.

### I6. No bypass

There is no raw-LaTeX, raw-Lean, raw-tactic, raw-HTML, raw-string, or untyped extension escape.

### I7. Single semantic source

Canonical LaTeX and Lean source are generated from the same linked IR. Neither generated artifact is parsed to create the other.

### I8. Canonical publication

The publishable mathematical document is the canonical LaTeX renderer's output, not the author's unchecked source bytes.

### I9. Scoped declarations

Every variable or hypothesis occurrence resolves to a unique scoped declaration. Capture is prevented by internal IDs, not spelling conventions.

### I10. Determinism

Given identical normalized source, project configuration, lock file, glossary closure, compiler-semantics identifier, and Lean toolchain identifier, platform-independent outputs are byte-identical.

### I11. Verification honesty

No verified attestation exists unless every required verification stage succeeds. A failed stage removes the staging directory and produces no verified directory.

### I12. No hidden proof assumptions

Every emitted declaration has an explicit axiom policy, and the observed set is recorded.

### I13. Complete traceability

Every semantic token in generated Lean and every visible token or control sequence in canonical LaTeX traces to an IR node and ultimately to source, glossary, or compiler-prelude origin.

### I14. Closed failure model

Every user-visible failure has a registered diagnostic code and a sanctioned exit-code class. User-controlled input MUST NOT cause a panic.

### I15. Offline verification

`check`, `build`, `fmt`, and `verify` perform no network operation. Package acquisition is confined to an explicit `lock --allow-network` invocation.

---

## 7. Repository layout

The completed repository MUST have this layout. Additional files are allowed only when they have a defined role and are included by the repository audits.

```text
.
├── .cargo/
│   └── config.toml
├── .devcontainer/
│   └── devcontainer.json
├── .github/
│   └── workflows/
│       ├── ci.yml
│       ├── honesty.yml
│       └── reproducibility.yml
├── AGENTS.md
├── CONFORMANCE.md
├── ERRORS.md
├── README.md
├── SPEC.md
├── VERIFICATION.md
├── Cargo.lock
├── Cargo.toml
├── Justfile
├── LICENSE-APACHE
├── LICENSE-MIT
├── clippy.toml
├── deny.toml
├── lean-toolchain
├── rust-toolchain.toml
├── rustfmt.toml
├── compiler/
│   ├── expected/
│   ├── fixtures/
│   ├── gnaf/
│   ├── gnaf.manifest.json
│   ├── lake-manifest.json
│   ├── lakefile.toml
│   ├── lean-toolchain
│   ├── lexlean.lock
│   ├── lexlean.toml
│   ├── rust/
│   └── src/
├── crates/
│   ├── lexlean/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── api.rs
│   │       ├── artifact/
│   │       │   ├── canonical_json.rs
│   │       │   ├── content_id.rs
│   │       │   ├── manifest.rs
│   │       │   ├── mod.rs
│   │       │   └── source_map.rs
│   │       ├── backend/
│   │       │   ├── latex.rs
│   │       │   ├── lean.rs
│   │       │   ├── mod.rs
│   │       │   └── pdf.rs
│   │       ├── calculus/
│   │       │   ├── check.rs
│   │       │   ├── interp.rs
│   │       │   ├── library.rs
│   │       │   ├── mod.rs
│   │       │   ├── realization.rs
│   │       │   ├── rust/
│   │       │   │   ├── ast.rs
│   │       │   │   ├── lower.rs
│   │       │   │   ├── mod.rs
│   │       │   │   ├── package.rs
│   │       │   │   ├── runtime.rs
│   │       │   │   └── validate.rs
│   │       │   └── term.rs
│   │       ├── cli.rs
│   │       ├── config.rs
│   │       ├── diagnostic.rs
│   │       ├── elaborate/
│   │       │   ├── definitions.rs
│   │       │   ├── expressions.rs
│   │       │   ├── mod.rs
│   │       │   ├── proofs.rs
│   │       │   └── resolve.rs
│   │       ├── error.rs
│   │       ├── fmt.rs
│   │       ├── gnaf.rs
│   │       ├── grammar/
│   │       │   ├── chart.rs
│   │       │   ├── math.rs
│   │       │   ├── mod.rs
│   │       │   ├── proposition.rs
│   │       │   ├── proof.rs
│   │       │   └── structural.rs
│   │       ├── ir/
│   │       │   ├── declaration.rs
│   │       │   ├── document.rs
│   │       │   ├── mod.rs
│   │       │   ├── proof.rs
│   │       │   └── term.rs
│   │       ├── lexicon/
│   │       │   ├── entry.rs
│   │       │   ├── lre.rs
│   │       │   ├── lse.rs
│   │       │   ├── mod.rs
│   │       │   ├── package.rs
│   │       │   └── resolve.rs
│   │       ├── lib.rs
│   │       ├── link.rs
│   │       ├── lock.rs
│   │       ├── main.rs
│   │       ├── project.rs
│   │       ├── source/
│   │       │   ├── atom.rs
│   │       │   ├── coverage.rs
│   │       │   ├── mod.rs
│   │       │   ├── normalize.rs
│   │       │   └── scan.rs
│   │       └── verify/
│   │           ├── axiom.rs
│   │           ├── child.rs
│   │           ├── leanchecker.rs
│   │           ├── mod.rs
│   │           ├── toolchain.rs
│   │           └── workspace.rs
│   ├── model/
│   └── conformance/
├── examples/
│   └── nat-add-zero/
├── features/
│   └── suites/
├── language/
│   ├── bootstrap.toml
│   ├── bootstrap-1.1.toml
│   ├── bootstrap-1.2.toml
│   ├── semantics.toml
│   ├── semantics-1.1.toml
│   ├── semantics-1.2.toml
│   ├── production-1.2.toml
│   ├── renderer-tokens.toml
│   ├── lcnf-1.2/
│   │   ├── authority.toml
│   │   └── extract.lean
│   ├── core/
│   │   ├── lexicon.toml
│   │   └── entries/
│   ├── core-1.1/
│   │   ├── lexicon.toml
│   │   └── entries/
│   ├── core-1.2/
│   │   ├── lexicon.toml
│   │   └── entries/
│   └── std/
│       ├── bool-1.1/
│       ├── bool-1.2/
│       ├── int/
│       ├── int-1.1/
│       ├── int-1.2/
│       ├── nat/
│       ├── nat-1.1/
│       └── nat-1.2/
├── model/
│   ├── authorities/
│   ├── authorities.toml
│   ├── errors.toml
│   ├── ids.toml
│   └── ledger.toml
├── schemas/
│   ├── attestation.schema.json
│   ├── attestation-v2.schema.json
│   ├── build-manifest.schema.json
│   ├── build-manifest-v2.schema.json
│   ├── compiler-input.schema.json
│   ├── core-module.schema.json
│   ├── coverage.schema.json
│   ├── diagnostic.schema.json
│   ├── entry.schema.json
│   ├── gnaf-fixture.schema.json
│   ├── gnaf-request.schema.json
│   ├── lexicon.schema.json
│   ├── lexicon-v2.schema.json
│   ├── lock.schema.json
│   ├── lock-1.1.schema.json
│   ├── lock-v2.schema.json
│   ├── production-eligibility.schema.json
│   ├── project.schema.json
│   ├── project-v2.schema.json
│   ├── rust-package.schema.json
│   ├── rust-provenance.schema.json
│   ├── semantic-snapshot.schema.json
│   ├── semantic-snapshot-v2.schema.json
│   ├── semantic-module.schema.json
│   ├── semantic-module-v2.schema.json
│   ├── source-map.schema.json
│   ├── target-fixture.schema.json
│   └── target-program.schema.json
├── tests/
│   ├── fixtures/
│   ├── golden/
│   └── negative/
└── xtask/
    ├── Cargo.toml
    └── src/
        ├── audit.rs
        ├── codegen.rs
        ├── main.rs
        └── spec_links.rs
```

`CONFORMANCE.md` and `ERRORS.md` are generated and committed. Their committed bytes MUST equal regeneration from `model/*.toml`.

`language/` is the compiler's versioned built-in language data. It is distinct from `model/`, which records claims about the repository.

---

## 8. Toolchains, platforms, and dependency policy

### 8.1 Rust

The exact repository Rust toolchain is:

```toml
[toolchain]
channel = "1.97.1"
components = ["rustfmt", "clippy"]
profile = "minimal"
```

The workspace uses Rust edition 2021 and MSRV 1.97. The shipped crate MUST compile with `#![forbid(unsafe_code)]`.

### 8.2 Lean

LexLean language 1.0 is pinned to:

```text
leanprover/lean4:v4.32.1
```

The repository-root `lean-toolchain` file MUST contain exactly that line followed by LF.

The corresponding Lean tag resolves to source commit:

```text
f054605aea4b840552cca2e725580bffd1e1b704
```

LexLean MUST reject a verification environment whose reported Lean version is not 4.32.1. The verification attestation records the SHA-256 digest of the actual `lean`, `lake`, and `leanchecker` executables used, so platform-specific executable bytes do not need to be assumed identical.

### 8.3 Supported hosts

The Rust crate MUST build and its non-Lean tests MUST pass on:

- Linux x86-64;
- Linux AArch64;
- macOS x86-64;
- macOS AArch64;
- Windows x86-64.

The normative full verification and reproducibility gate runs on Linux x86-64. Other hosts produce platform-bound attestations and MUST obey the same source, manifest, and policy formats.

Non-UTF-8 project paths are outside language 1.0 and produce a registered environment diagnostic.

### 8.4 Cargo dependency rules

- Every dependency requirement MUST be explicit; wildcard requirements are forbidden.
- `Cargo.lock` is committed.
- `cargo deny --all-features check` is part of `just vv`.
- Unknown registries and unknown Git sources are denied.
- `repo-model`, `repo-conformance`, and `xtask` are `publish = false`.
- No shipped crate may depend on a `publish = false` repository crate.
- Cargo features MUST NOT disable a normative capability. LexLean 1.0 defines no optional capability feature.
- Dependency additions require a license decision and an update to `deny.toml` only when the actual graph requires it.

### 8.5 Required implementation dependencies

The implementation MAY choose exact compatible releases through `Cargo.lock`, but the dependency roles are fixed:

- CLI parsing;
- TOML parsing;
- JSON parsing and serialization;
- SHA-256;
- semantic-version parsing;
- UTF-8 path handling;
- Unicode NFC normalization;
- structured errors;
- temporary directories;
- directory walking;
- file locking.

No dependency may provide arbitrary TeX execution, Lean parsing by string substitution, probabilistic language interpretation, or a second proof authority.

---

## 9. Root tooling

### 9.1 `.cargo/config.toml`

The file MUST contain only repository aliases; target-specific remnants from the template MUST be removed.

```toml
[alias]
xtask = "run --package xtask --"
vv = "run --package xtask -- validate"
```

### 9.2 `Justfile`

The normative acceptance recipe is `just vv`. It MUST run, in this order:

1. `fmt-check`;
2. `model`;
3. `spec-links`;
4. `lint`;
5. `test`;
6. `features`;
7. `bdd`;
8. `examples`;
9. `golden`;
10. `repro`;
11. `deny`.

The recipes have these meanings:

```text
fmt-check   cargo fmt --all -- --check
model       cargo xtask validate-model
spec-links  cargo xtask validate-spec-links
lint        cargo clippy --workspace --all-targets --all-features -- -D warnings
test        cargo test --workspace --all-features
features    cargo check --workspace --all-features --all-targets
bdd         cargo test -p repo-conformance
examples    cargo xtask verify-examples
golden      cargo xtask check-golden
repro       cargo xtask check-reproducibility
deny        cargo deny --all-features check
vv          all recipes above, in order
```

`just model-write` regenerates `CONFORMANCE.md` and `ERRORS.md`. Golden files are rewritten only by the explicit `just golden-write` recipe. No acceptance recipe rewrites source or expected output.

### 9.3 Development container

The development container MUST install:

- Rust 1.97.1 with `rustfmt` and `clippy`;
- `just`;
- `cargo-deny`;
- `elan`;
- `leanprover/lean4:v4.32.1`;
- the Rust Analyzer and Lean 4 editor extensions.

Its post-create command MUST install the pinned Lean toolchain explicitly and MUST NOT select an unpinned default. The container configuration is development support; verification still performs its own version and digest preflight.

### 9.4 Continuous integration

`ci.yml` MUST run `just vv` from a clean checkout on Linux x86-64. It MUST install the exact Rust toolchain, Lean 4.32.1 through `elan`, `just`, and `cargo-deny`.

`honesty.yml` MUST run the model/honesty gate separately so claim drift is distinguishable from code failure.

`reproducibility.yml` MUST perform two builds in distinct absolute directories and compare every platform-independent artifact byte-for-byte after path normalization.

---

## 10. Project files

A LexLean project contains:

- one `lexlean.toml`;
- one generated `lexlean.lock`;
- one `lean-toolchain`;
- a valid Lake workspace;
- one or more `.lex.tex` modules under configured source roots;
- zero or more local lexicon packages;
- `.lexlean/`, reserved for generated and verified artifacts.

No project file may override the language grammar, backend, verifier sequence, axiom-audit parser, source-map format, or verified artifact set.

### 10.1 `lexlean.toml`

`lexlean.toml` is UTF-8, NFC, LF-terminated TOML. Comments are forbidden. Unknown fields are errors. Every field below is required except `[pdf]`.

```toml
spec = "lexlean/project/1"
name = "example-project"
language = "1.0"
module_prefix = "ExampleProject"
source_roots = ["src"]
entrypoints = ["src/Main.lex.tex"]
build_root = ".lexlean"
lockfile = "lexlean.lock"
lean_workspace = "."
lean_toolchain = "leanprover/lean4:v4.32.1"

[[lexicon_source]]
package = "lexlean.std.nat"
kind = "builtin"

[limits]
max_file_bytes = 4194304
max_total_source_bytes = 67108864
max_primitive_atoms = 2000000
max_token_lattice_edges = 4000000
max_parse_states = 4000000
max_ir_nodes = 2000000
max_scope_depth = 1024
max_import_depth = 128
max_diagnostics = 256
max_child_output_bytes = 16777216
child_timeout_ms = 300000
```

#### Canonical project serialization

The parsed project has one canonical TOML serialization used by `project_config_sha256` and `source_id`. It writes:

1. scalar top-level keys in the order shown by the schema example;
2. `lexicon_source` tables sorted by `package`, with keys in the order `package`, `kind`, then kind-specific fields;
3. language 1.2 only: `artifact_source` tables sorted by `sha256`, with keys in the order `sha256`, `path`, omitted entirely when there are none, so a project without artifacts keeps its bytes;
4. `[limits]` with keys in the order shown;
5. optional `[pdf]` last, with keys in the order shown;
6. basic quoted strings, decimal integers, arrays with comma-space separators, LF line endings, and one final LF;
7. no comments, blank leading lines, or trailing spaces.

Input whitespace does not affect this canonical serialization. A project may be semantically valid before formatting, but `lexlean lock --check` and repository examples require canonical bytes.

#### Project-field rules

| Field | Rule |
|---|---|
| `spec` | Exactly `lexlean/project/1`. |
| `name` | Lower-case ASCII package identifier: `[a-z][a-z0-9-]{0,62}`. |
| `language` | Language version: `1.0`, `1.1`, or `1.2`. |
| `module_prefix` | One or more dot-separated ASCII Lean-name segments matching `[A-Z][A-Za-z0-9_]*`. |
| `source_roots` | Nonempty, unique, sorted project-relative directories. |
| `entrypoints` | Nonempty, unique, sorted project-relative `.lex.tex` files beneath a source root. |
| `build_root` | Project-relative directory; must resolve within the project and must not be a symlink. |
| `lockfile` | Project-relative regular-file path; exactly one lock file. |
| `lean_workspace` | Project-relative directory containing `lean-toolchain` and a Lake configuration. |
| `lean_toolchain` | Exactly the language-1.0 toolchain string. |
| `lexicon_source` | Unique by package and sorted by package identifier. |
| `artifact_source` | Language 1.2 only (another language rejects the field, `LLC0101`): optional `[[artifact_source]]` tables `sha256 = "<64 lowercase hex>"`, `path = "<project-relative file>"`, sorted by `sha256`, unique by digest and by path, never beneath the build root. Each names the content-addressed bytes of one model artifact (§17.12); `check`, `build`, `fmt`, and `verify` read the file, confined (§25.1), and require its SHA-256 to equal the configured digest (`LLR3007`). Artifacts are never fetched. |
| `limits` | Every listed positive integer is required. There are no hidden compiler defaults. |

#### Lexicon-source forms

A source is exactly one of:

```toml
[[lexicon_source]]
package = "lexlean.std.nat"
kind = "builtin"
```

```toml
[[lexicon_source]]
package = "example.algebra"
kind = "path"
path = "glossary/example.algebra"
```

```toml
[[lexicon_source]]
package = "example.algebra"
kind = "git"
url = "https://github.com/example/algebra-lexicon.git"
revision = "0123456789abcdef0123456789abcdef01234567"
subdirectory = "lexicon"
```

Rules:

- `builtin` accepts no additional fields.
- `path` requires `path` and accepts no URL or revision.
- `git` requires HTTPS URL, exact 40-lowercase-hex commit, and a relative `subdirectory`; it accepts no branch, tag, or mutable reference.
- package IDs match `[a-z][a-z0-9]*(\.[a-z][a-z0-9-]*)*`.
- source order does not create shadowing.
- path packages and cached Git packages are hashed as described in §21.4.
- `check`, `build`, `fmt`, and `verify` never fetch a missing Git source.

### 10.2 Explicit resource policy

The values under `[limits]` are semantic inputs to acceptance. `max_total_source_bytes` counts normalized selected modules, loaded lexicon manifests and entries, configuration, lock bytes, and, in language 1.2, the bytes of every configured model artifact and, once more, the length of every artifact declaration, which materializes its bytes again (§17.12 rule 2); a single artifact is also bounded by `max_file_bytes`. Exceeding one produces `LLS8002` and identifies:

- the limit name;
- configured value;
- observed value, when safely known;
- the source or phase involved.

The compiler MUST stream or incrementally inspect configuration and lock data so that it does not require a hidden pre-configuration size limit. Platform allocation failure is reported as an environment failure, not misrepresented as a language error.

### 10.3 Optional PDF provider

When `[pdf]` is absent, no PDF is requested or emitted. When present, all fields are required:

```toml
[pdf]
mode = "external"
program = "tools/tectonic"
program_sha256 = "64-lowercase-hex-digits"
version_argv = ["--version"]
version_stdout_sha256 = "64-lowercase-hex-digits"
compile_argv = ["--outdir", "{out_dir}", "{input}"]
output = "{stem}.pdf"
resources = []
```

Only `mode = "external"` exists in language 1.0. The protocol is specified in §19.

### 10.4 Lake workspace pin

The project `lean-toolchain` MUST contain the exact toolchain string. `lexlean lock` records hashes of:

- `lean-toolchain`;
- the one Lake configuration, either `lakefile.toml` or `lakefile.lean`;
- `lake-manifest.json`, when present.

If both Lake configuration forms exist, locking fails. Verification requires the recorded files to match and requires all manifest dependencies to be locally available.

---

## 11. Lock file

### 11.1 General

`lexlean.lock` is generated, canonical TOML with no comments and final LF. Users MUST NOT hand-edit it. `lexlean lock --check` regenerates it in memory and compares exact bytes. Language 1.0 and 1.1 use `spec = "lexlean/lock/1"`. Language 1.2 uses `spec = "lexlean/lock/2"`. `lexlean lock` migrates locks to the appropriate schema for the project language.

The top-level schema is:

```toml
spec = "lexlean/lock/1"
language = "1.0"
compiler_semantics = "64-lowercase-hex-digits"
project_config_sha256 = "64-lowercase-hex-digits"
lean_toolchain = "leanprover/lean4:v4.32.1"

[[workspace_file]]
path = "lean-toolchain"
sha256 = "..."

[[package]]
id = "lexlean.core"
version = "1.0.0"
kind = "builtin"
source = "embedded"
revision = "compiler-semantics"
tree_sha256 = "..."
manifest_sha256 = "..."
imports = []

[[package]]
id = "example.algebra"
version = "1.2.3"
kind = "path"
source = "glossary/example.algebra"
revision = "none"
tree_sha256 = "..."
manifest_sha256 = "..."
imports = ["lexlean.core@1.0.0"]
```

An optional `[pdf]` lock record mirrors the configured provider and records the hashes of every declared resource.

Language-1.2 model artifacts (§10.1, §17.12) have no lock rows: each is pinned
by its `sha256` in `lexlean.toml`, whose canonical bytes the lock's project
digest and the source ID cover, and every `check`, `build`, `fmt`, and
`verify` re-reads and re-digests the bytes.

### 11.2 Canonical ordering

- `workspace_file` rows sort by `path`.
- `package` rows sort by `(id, version)`.
- package `imports` sort lexicographically and contain no duplicates.
- all keys use the order shown by the schema formatter.
- lowercase hexadecimal is mandatory.

### 11.3 Package closure

Every package imported by a source module, and every transitive lexicon import, appears exactly once in the lock. The lock includes `lexlean.core@1.0.0` even though it is automatically loaded.

A source import names an exact package and exact version. Version ranges are not part of language 1.0.

### 11.4 Offline behavior

- `lexlean lock` resolves local and already-cached sources without networking.
- `lexlean lock --allow-network` may acquire missing exact Git commits.
- no other command accepts `--allow-network`;
- verification fails if a locked package is absent or its digest differs;
- an acquired Git package is stored at `<build_root>/cache/git/<revision>/<tree_sha256>/` and is revalidated before use;
- Git submodules, Git LFS indirection, and nested repositories are rejected.

### 11.5 Tree digest

A package tree digest is SHA-256 over:

```text
"lexlean-tree-v1\0"
+ for each regular file in bytewise-sorted project-relative path order:
    u32be(path_byte_length)
    + path_utf8_bytes
    + u64be(file_byte_length)
    + file_bytes
```

Only `lexicon.toml` and files under `entries/` participate. Symlinks, device files, FIFOs, sockets, and non-UTF-8 paths are rejected.

---

## 12. Source normalization and primitive atoms

### 12.1 File encoding

Every `.lex.tex` file MUST be valid UTF-8.

Compilation normalizes CRLF and lone CR to LF, then requires Unicode NFC. A non-NFC file is rejected by `check` with a fix-it; `fmt` rewrites it canonically.

A source file MUST end in one LF and MUST NOT contain:

- NUL;
- tab;
- trailing spaces;
- Unicode line or paragraph separators;
- non-ASCII whitespace;
- a raw percent character;
- a TeX comment;
- a byte-order mark.

A percent sign may appear only through a glossary-defined control sequence such as `\percent`.

### 12.2 Primitive scanner

The primitive scanner recognizes only these atom classes:

1. **control**: backslash followed by one or more ASCII letters, or backslash followed by one ASCII nonletter;
2. **ASCII word**: one or more ASCII letters;
3. **metadata/math identifier**: an ASCII letter followed by zero or more ASCII letters, digits, `_`, or `'`, when the structural or math grammar requests an identifier;
4. **numeral**: one or more ASCII digits;
5. **delimiter**: `{`, `}`, `(`, `)`, `[`, `]`;
6. **ASCII symbol**: one printable ASCII scalar not classified above;
7. **Unicode symbol**: one non-ASCII Unicode scalar;
8. **whitespace**: one or more U+0020 or LF scalars.

The scanner assigns byte and line/column spans. It does not assign mathematical meaning.

The scanner has no dependency on host Unicode character classes. Non-ASCII prose is possible only when its exact scalar sequence is declared as a glossary form.

### 12.3 Core syntax also belongs to the glossary

The compiler embeds the exact bytes and digest of `lexlean.core@1.0.0` and loads it before structural parsing. The lock must identify the same digest. The embedded package is not an implicit prose exception; it is the bootstrap glossary.

Braces, environment delimiters, punctuation, math delimiters, structural controls, and grammar words are entries in `lexlean.core@1.0.0`. They are not accepted merely because TeX recognizes them.

The scanner is the only bootstrapping layer. Every accepted primitive atom after scanning is accounted for by lexical coverage.

### 12.4 No TeX expansion

LexLean does not execute TeX's mouth, macro expander, conditionals, catcodes, package loader, or file inclusion. The following controls are always forbidden in source, whether or not a glossary attempts to declare them:

```text
\def
\gdef
\edef
\xdef
\let
\futurelet
\newcommand
\renewcommand
\providecommand
\input
\include
\usepackage
\documentclass
\csname
\catcode
\write
\read
\openin
\openout
\special
\immediate
\verbatim
\verb
```

The canonical LaTeX backend may emit trusted controls from the renderer-token registry; this does not make those controls legal source escape hatches.

---

## 13. Lexicon packages

### 13.1 Package layout

A lexicon package is:

```text
<package-root>/
├── lexicon.toml
└── entries/
    └── <local-entry-id path>.toml
```

`lexicon.toml` is:

```toml
spec = "lexlean/lexicon/1"
package = "example.algebra"
version = "1.2.3"
language = "1.0"
imports = ["lexlean.core@1.0.0"]
```

Unknown fields and comments are errors. `imports` are exact, sorted, and unique.

A local entry ID is dot-separated lower-case ASCII:

```text
[a-z][a-z0-9-]*(\.[a-z][a-z0-9-]*)*
```

The entry file path is `entries/` plus each dot-separated segment as a directory and `.toml` on the final segment. For example, `nat.add.zero` is stored at `entries/nat/add/zero.toml`.

The globally qualified ID is:

```text
<package-id>::<local-entry-id>
```

### 13.2 Entry schema

Every entry file has:

```toml
spec = "lexlean/entry/1"
id = "nat.add"
category = "infix-function"
signature = "(pi ((explicit a (const lexlean.std.nat::nat)) (explicit b (const lexlean.std.nat::nat))) (const lexlean.std.nat::nat))"
surface_arity = 2
frame = "infix"
precedence = 65
associativity = "left"

[denotation]
kind = "lean"
module = "Init.Prelude"
name = "HAdd.hAdd"

[[form]]
id = "plus"
channel = "math"
surface = "+"
canonical_source = true
features = []

[render]
math = "(seq (slot 0) (space) (token plus) (space) (slot 1))"
```

Fields that do not apply to the selected category or denotation are forbidden rather than ignored.

### 13.3 Categories

The exact language-1.0 categories are:

| Category | Semantic role |
|---|---|
| `structural` | Core-only source structure. |
| `grammar` | Core-only determiner, connective, copula, or proof keyword. |
| `label-word` | A concept token allowed in titles and headings. |
| `type-noun` | A type-valued atom or phrase. |
| `term-constant` | A term-valued atom. |
| `function` | A function referenced by explicit call syntax. |
| `prefix-function` | One explicit surface argument. |
| `postfix-function` | One explicit surface argument. |
| `infix-function` | Two explicit surface arguments. |
| `noun-function` | Canonical “the SELF of ARG” function phrase. |
| `binary-noun-function` | Canonical “the SELF of ARG and ARG” phrase. |
| `predicate-constant` | A proposition-valued atom. |
| `adjective-predicate` | Canonical “ARG is SELF” predicate. |
| `intransitive-predicate` | Canonical “ARG SELF” predicate. |
| `transitive-predicate` | Canonical “ARG SELF ARG” predicate. |
| `infix-predicate` | Two-argument mathematical relation. |
| `proof-constant` | A proof term or theorem reference. |

A non-core entry cannot use `structural` or `grammar`.

Category/frame compatibility is exact:

| Category | Permitted frame |
|---|---|
| `structural`, `grammar`, `label-word`, `type-noun`, `term-constant`, `predicate-constant` | `atom` |
| `function` | `call` |
| `prefix-function` | `prefix` |
| `postfix-function` | `postfix` |
| `infix-function`, `infix-predicate` | `infix` |
| `noun-function` | `noun-of` |
| `binary-noun-function` | `binary-noun-of` |
| `adjective-predicate` | `adjective` |
| `intransitive-predicate` | `intransitive` |
| `transitive-predicate` | `transitive` |
| `proof-constant` | `atom` when `surface_arity = 0`, otherwise `call` |

### 13.4 Frames

The exact frame values and surface slot order are:

| Frame | Surface pattern |
|---|---|
| `atom` | `SELF` |
| `call` | `SELF ( ARG_0, ..., ARG_n )` |
| `prefix` | `SELF ARG_0` |
| `postfix` | `ARG_0 SELF` |
| `infix` | `ARG_0 SELF ARG_1` |
| `noun-of` | `the SELF of ARG_0` |
| `binary-noun-of` | `the SELF of ARG_0 and ARG_1` |
| `adjective` | `ARG_0 is SELF` |
| `intransitive` | `ARG_0 SELF` |
| `transitive` | `ARG_0 SELF ARG_1` |

Category/frame compatibility is fixed by the category names above. A package cannot introduce arbitrary grammar productions.

### 13.5 Forms

Each form declares:

- `id`: unique local form ID;
- `channel`: `text`, `math`, or `both`;
- `surface`: exact normalized source spelling;
- `canonical_source`: Boolean;
- `features`: sorted unique values from:
  - `singular`;
  - `plural`;
  - `sentence-case`;
  - `lower-case`;
  - `article-a`;
  - `article-an`.

Rules:

1. leading, trailing, or repeated whitespace in `surface` is forbidden;
2. a surface is parsed into primitive atoms at package-load time;
3. control-sequence forms are aliases only and cannot be canonical source forms for non-core entries;
4. a non-core canonical text form contains only ASCII words and renderer-safe punctuation;
5. a non-core canonical math form contains only renderer-safe ASCII or Unicode symbols;
6. if no safe canonical surface exists, `fmt` emits `\lexeme{qualified-id}`;
7. exactly one canonical source form exists for each channel required by the category;
8. forms are case-sensitive; capitalization variants are separate forms;
9. inflections are explicit; LexLean does not infer plurals or articles;
10. two entries may share a surface, but acceptance then requires unique grammar/type resolution.

### 13.6 Denotations

A denotation is exactly one of:

#### Core

```toml
[denotation]
kind = "core"
constructor = "logic.forall"
```

Only `lexlean.core` may use it.

#### External Lean

```toml
[denotation]
kind = "lean"
module = "Init.Data.Nat.Basic"
name = "Nat.add_zero"
```

The module and name must use the conservative ASCII Lean-name grammar. The entry signature is the complete LexLean interface.

#### Document declaration

```toml
[denotation]
kind = "document"
module = "Main"
component = "double"
```

The declaration must exist in the named LexLean module and match the entry signature.

#### Defined lexicon value

```toml
[denotation]
kind = "defined"
value = "(lam ((explicit n (const lexlean.std.nat::nat))) (app (const lexlean.std.nat::succ) (local n)))"
```

Defined values are nonrecursive, acyclic, and may reference only earlier entries in the package dependency order.

No denotation contains an English description field. Fields named `description`, `documentation`, `note`, `meaning`, or any unknown name are rejected.

### 13.7 Signature requirement

Every semantic entry has a complete signature in the LexLean Semantic Expression encoding in §13.8. The compiler hashes the canonical signature and records the hash in the linked glossary closure.

The signature is an interface declaration checked twice:

1. LexLean uses it for controlled resolution and conservative elaboration.
2. Verification generates an external-interface probe whose Lean elaboration must succeed.

LexLean's Rust checker does not claim to replace Lean's dependent type checker.

### 13.8 LexLean Semantic Expressions (LSE)

LSE is an S-expression language. Its grammar is:

```ebnf
expr        = sort | const | local | app | pi | lam | let | nat ;
sort        = "(" "sort" "prop" ")"
            | "(" "sort" "(" "type" universe ")" ")" ;
const       = "(" "const" qualified-id [universe-args] ")" ;
local       = "(" "local" local-id ")" ;
app         = "(" "app" expr expr {expr} ")" ;
pi          = "(" "pi" "(" binder {binder} ")" expr ")" ;
lam         = "(" "lam" "(" binder {binder} ")" expr ")" ;
let         = "(" "let" local-id expr expr expr ")" ;
nat         = "(" "nat" decimal ")" ;
binder      = "(" binder-mode local-id expr ")" ;
binder-mode = "explicit" | "implicit" | "instance" ;
universe-args = "(" universe {universe} ")" ;
universe    = decimal
            | universe-id
            | "(" "succ" universe ")"
            | "(" "max" universe universe {universe} ")"
            | "(" "imax" universe universe ")" ;
```

Whitespace is one or more ASCII spaces or LF. Comments and quoted strings do not exist.

Rules:

- identifiers are ASCII and validated;
- each local resolves by lexical scope;
- binders are capture-free internal IDs after parsing;
- `app` has at least a function and one argument;
- `pi` and `lam` have at least one binder;
- universe variables are declared by the entry's optional sorted `universes` array;
- all semantic conveniences such as equality, conjunction, existence, and implication are applications of core constants after desugaring;
- canonical LSE prints one ASCII space between atoms, no redundant grouping, and final LF only when stored as a file field;
- alpha-equivalent LSE values are canonicalized by binder order before hashing.

### 13.9 Rendering expressions (LRE)

LRE is the only way a glossary entry influences canonical visible output.

```ebnf
render      = form | self-form | slot | seq | space | token
            | group | paren | bracket | sub | sup | frac
            | operator-name ;
form        = "(" "form" qualified-id form-id ")" ;
self-form   = "(" "self-form" form-id ")" ;
slot        = "(" "slot" decimal ")" ;
seq         = "(" "seq" render {render} ")" ;
space       = "(" "space" ")" ;
token       = "(" "token" renderer-token-id ")" ;
group       = "(" "group" render ")" ;
paren       = "(" "paren" render ")" ;
bracket     = "(" "bracket" render ")" ;
sub         = "(" "sub" render render ")" ;
sup         = "(" "sup" render render ")" ;
frac        = "(" "frac" render render ")" ;
operator-name = "(" "operator-name" ascii-identifier ")" ;
```

Rules:

- raw TeX strings do not exist;
- every `form` references an existing form;
- every `token` references `language/renderer-tokens.toml`;
- every slot index is less than `surface_arity`;
- every explicit surface argument appears exactly once in the canonical render template;
- implicit semantic parameters are not surface slots;
- `operator-name` accepts only `[A-Za-z][A-Za-z0-9_]*`;
- non-core packages cannot define renderer tokens;
- renderer expressions are acyclic.

### 13.10 Renderer-token registry

`language/renderer-tokens.toml` is the single source for trusted LaTeX controls and mathematical glyphs. Each row declares:

- token ID;
- exact emitted UTF-8 bytes;
- channel;
- arity;
- whether grouping is required;
- source package authority, always core.

The language-1.0 registry MUST be the minimal transitive closure of tokens referenced by the exact preamble, core LRE, and shipped standard-package LRE. Unused rows fail the language audit. It MUST contain the following semantic token IDs; document/package options and environment names in the fixed preamble are also individual core rows rather than unclassified text:

```text
documentclass
usepackage
newtheorem
theoremstyle
begin
end
center
large
section
subsection
label
texttt
operatorname
mathbb
mathrm
proof
definition
theorem
lemma
corollary
plus
minus
times
cdot
slash
equals
not-equals
less
less-equal
greater
greater-equal
member
not-member
subset
subset-equal
union
intersection
forall
exists
exists-unique
logical-and
logical-or
logical-not
implies
iff
mapsto
arrow
left-arrow
comma
period
colon
semicolon
left-paren
right-paren
left-bracket
right-bracket
```

Canonical math bytes are fixed: logical conjunction is `\land`, disjunction `\lor`, negation `\lnot`, implication `\to`, equivalence `\leftrightarrow`, universal quantification `\forall`, existence `\exists`, unique existence `\exists!`, membership `\in`, and set inclusion `\subseteq` where the corresponding token is used. Preamble option/package/environment atoms, braces, separators, and dynamically generated validated labels are all covered by core structural rows or source-derived metadata rows.

A renderer token is not a source macro facility. It is a closed compiler datum included in the compiler-semantics hash.

### 13.11 Package validation

Loading a package MUST reject:

- schema mismatch;
- package/version mismatch with the lock;
- unknown fields;
- comments;
- duplicate package or entry IDs;
- invalid entry paths;
- invalid forms;
- missing canonical forms;
- form/category incompatibility;
- invalid LSE or LRE;
- unresolved references;
- import cycles;
- defined-denotation cycles;
- invalid renderer slots;
- raw control output;
- a document denotation whose declaration is unavailable;
- an eliminator descriptor that references absent constructors;
- a package whose `language` differs from the project's (`LLC0103`).

---

## 14. Lexical resolution

### 14.1 Token lattice

LexLean builds a token lattice rather than committing greedily to one segmentation.

For each source position, it records every glossary form whose primitive-atom sequence begins there. Whitespace between atoms in a multiword form is equivalent to one separator. The parser may select any path that:

- begins at the component's first non-whitespace atom;
- ends at its last non-whitespace atom;
- covers every non-whitespace atom exactly once;
- respects structural channel boundaries;
- stays below `max_token_lattice_edges`.

Import order gives no priority.

### 14.2 Scoped identifiers

A local identifier is accepted before global glossary resolution only in an explicit binder position. The binder creates a scoped lexical declaration. Every later occurrence must resolve to that declaration or a different explicit global selection.

Local scope order is:

1. inherited section parameters;
2. declaration binders;
3. proof locals and hypotheses;
4. branch binders.

Inner locals shadow an identically spelled outer local. Local identifiers do not shadow text glossary forms.

### 14.3 Explicit qualification

`\lexeme{package::entry}` selects one glossary entry. The control and every delimiter are core entries; the qualified ID is structural metadata.

`\reference{Module::component}` selects one document declaration as a proof term. It is valid only when the module is imported or current and the declaration precedes the reference.

Qualified selection is canonicalized back to the entry's safe canonical surface when possible. It never injects arbitrary output.

### 14.4 Ambiguity

The parser and elaborator retain alternatives until:

- grammar-category constraints;
- scope;
- arity;
- expected semantic category;
- declared signatures; and
- conservative type unification

remove them.

Distinct alternatives that canonicalize to the same linked IR are collapsed. If more than one distinct linked IR remains, compilation fails with `LLP2002` and presents the minimal differentiating spans and qualified candidate IDs.

No package order, frequency, capitalization heuristic, or external language model may select a meaning.



## 15. LexLean document grammar

### 15.1 Structural grammar

The structural grammar is fixed. Environment and control names are literal core entries.

```ebnf
module          = "\begin{lexlean}" "{" module-name "}"
                  header
                  block*
                  "\end{lexlean}" ;

header          = use-glossary*
                  import-module*
                  title ;

use-glossary    = "\useglossary" "{" package-id "@" semver "}" ;
import-module   = "\importmodule" "{" module-name "}" ;
title           = "\title" "{" phrase "}" ;

block           = section | declaration ;

section         = "\begin{section}" "{" component-id "}"
                  "\heading" "{" phrase "}"
                  [parameters]
                  block*
                  "\end{section}" ;

parameters      = "\parameters" "{" binder-list "}" ;

declaration     = type-definition
                | term-definition
                | predicate-definition
                | theorem
                | lemma
                | corollary ;

type-definition = "\begin{typedefinition}"
                  "{" component-id "}"
                  "{" qualified-entry-id "}"
                  axiom-policy
                  type-definition-sentence
                  "\end{typedefinition}" ;

term-definition = "\begin{termdefinition}"
                  "{" component-id "}"
                  "{" qualified-entry-id "}"
                  axiom-policy
                  term-definition-sentence
                  "\end{termdefinition}" ;

predicate-definition
                = "\begin{predicatedefinition}"
                  "{" component-id "}"
                  "{" qualified-entry-id "}"
                  axiom-policy
                  predicate-definition-sentence
                  "\end{predicatedefinition}" ;

theorem         = "\begin{theorem}" "{" component-id "}"
                  axiom-policy proposition-sentence proof
                  "\end{theorem}" ;

lemma           = "\begin{lemma}" "{" component-id "}"
                  axiom-policy proposition-sentence proof
                  "\end{lemma}" ;

corollary       = "\begin{corollary}" "{" component-id "}"
                  axiom-policy proposition-sentence proof
                  "\end{corollary}" ;

proof           = "\begin{proof}" proof-step+ "\end{proof}" ;

axiom-policy    = "\noaxioms"
                | "\allowaxioms" "{" lean-name-list "}"
                | "\exactaxioms" "{" lean-name-list "}" ;
```

Rules:

- imports occur only in the header;
- `\useglossary` rows sort by package reference under `fmt`;
- `\importmodule` rows sort by module name under `fmt`;
- each module has exactly one title;
- section nesting is allowed up to `max_scope_depth`;
- section component IDs and declaration component IDs share one module-wide namespace;
- component IDs match `[a-z][a-z0-9-]*`;
- module names are relative to `module_prefix`;
- the full generated Lean module is `<module_prefix>.<module-name>`;
- an imported module must be a project module and must not create an import cycle;
- a selected build includes the transitive import closure;
- a source module may import only declarations exported by an explicitly imported module;
- no declaration may reference a later declaration in the same module;
- an empty module is valid, but a theorem-like declaration without a proof is not.

### 15.2 Core structural controls

The complete structural control set for language 1.0 is:

```text
\begin
\end
\useglossary
\importmodule
\title
\heading
\parameters
\noaxioms
\allowaxioms
\exactaxioms
\lexeme
\reference
\forward
\backward
\rule
\start
\step
\bind
\(
\)
\[
\]
```

The complete environment-name set is:

```text
lexlean
section
typedefinition
termdefinition
predicatedefinition
theorem
lemma
corollary
proof
have
rewrite
simplify
apply
constructor
branch
premise
cases
case
induction
calculate
```

Any other control or environment name is unknown unless it is a nonstructural glossary form accepted in the current expression channel. A glossary cannot add an environment.

### 15.3 Phrases

A `type-phrase` is either one linked `type-noun` frame, including its explicit surface arguments, or one math island whose result is a sort.

A `phrase` is a nonempty sequence of:

- `label-word`;
- `type-noun`;
- the canonical nominal form of a term or function entry;
- a math island containing a term whose result is not `Prop`;
- core punctuation `:`, `-`, `(`, or `)`.

A phrase cannot contain:

- a proposition;
- a quantifier;
- a predicate frame;
- a proof instruction;
- an unbound pronoun;
- an adverbial assertion.

Titles and headings therefore identify concepts but do not make unproved claims.

### 15.4 Binder lists

A text binder has:

```ebnf
binder      = type-phrase math-new-local ;
binder-list = binder { ";" binder } ;
```

Example:

```latex
\parameters{natural number \(n\); natural number \(m\)}
```

A `math-new-local` is a math island containing exactly one fresh ASCII identifier. The identifier is a display spelling; the IR assigns a unique `LocalId`.

All source binders are explicit. Implicit and instance binders may exist in external signatures, but language 1.0 does not let document prose silently introduce them.

### 15.5 Mathematical islands

Only `\(...\)` and `\[...\]` delimit mathematical islands. Dollar delimiters are forbidden.

Inside a math island:

- spaces and LF are insignificant separators;
- parentheses group;
- comma separates explicit call arguments;
- a local identifier resolves through scope;
- a numeral uses the core numeral constructor;
- `\lexeme{qualified-id}` explicitly selects an entry;
- `\reference{Module::component}` selects an earlier or imported document declaration;
- glossary frames supply prefix, postfix, and infix syntax;
- function application is `f(arg_0, ..., arg_n)`;
- juxtaposition is never implicit multiplication or application;
- braces do not group mathematical terms;
- `_` has no built-in subscript meaning;
- every operator has declared precedence and associativity.

The Pratt precedence scale is `0..255`; higher values bind more tightly. An infix operator declares `left`, `right`, or `none`. A nonassociative chain without parentheses is an ambiguity error.

Numerals have no default type. A numeral must receive an expected type from an operator, relation, binder, definition signature, or declaration statement. Otherwise elaboration fails.

### 15.6 Proposition grammar

The controlled proposition grammar is:

```ebnf
proposition-sentence = proposition "." ;

proposition    = conditional ;

conditional    = "if" proposition "," "then" proposition
               | equivalence ;

equivalence    = implication
                 [ "if" "and" "only" "if" implication ] ;

implication    = disjunction
                 [ "implies" implication ] ;

disjunction    = conjunction { "or" conjunction } ;

conjunction    = negation { "and" negation } ;

negation       = "not" negation
               | quantified
               | atomic-proposition ;

quantified     = "For" "every" binder
                 { "and" binder } ","
                 proposition
               | "for" "every" binder
                 { "and" binder } ","
                 proposition
               | "there" "exists" article binder
                 "such" "that" proposition
               | "there" "exists" "exactly" "one" binder
                 "such" "that" proposition ;

article        = "a" | "an" ;

atomic-proposition
              = math-proposition
              | predicate-frame ;
```

The lowercase `for every` form is accepted after a grammar boundary; sentence-initial canonical output uses `For`.

A `math-proposition` is a math island whose linked result is `Prop`.

A `predicate-frame` is one of the fixed predicate frames declared in §13.4. Its semantic denotation is application of that entry to its surface arguments.

Compositional semantics are:

| Surface | IR |
|---|---|
| `For every x, P` | dependent `Pi` into `Prop` |
| `there exists a x such that P` | `Exists (fun x => P)` |
| `there exists exactly one x such that P` | `ExistsUnique (fun x => P)` |
| `if P, then Q` or `P implies Q` | implication |
| `P if and only if Q` | `Iff P Q` |
| `P and Q` | `And P Q` |
| `P or Q` | `Or P Q` |
| `not P` | `Not P` |

Core logical words have no user-overridable denotation.

### 15.7 Definition sentences

A type definition has exactly one of these forms:

```text
A SELF-TYPE is defined as TYPE.
An SELF-TYPE is defined as TYPE.
For every BINDER-LIST, a SELF-TYPE is defined as TYPE.
For every BINDER-LIST, an SELF-TYPE is defined as TYPE.
```

A term definition has exactly one of:

```text
SELF-TERM is defined as TERM.
For every BINDER-LIST, SELF-APPLICATION is defined as TERM.
```

A predicate definition has exactly one of:

```text
SELF-PREDICATE holds exactly when PROPOSITION.
For every BINDER-LIST, SELF-PREDICATE holds exactly when PROPOSITION.
```

Validation requirements:

1. `SELF` is the entry named in the declaration header.
2. Its denotation is `document` and names the current module and component.
3. Its signature agrees with the declaration kind.
4. Each explicit signature binder occurs exactly once in the self application and in signature order.
5. The right-hand side has the declared result category.
6. The right-hand side does not reference the declaration being defined.
7. The declaration references no later local declaration.
8. The definition graph is acyclic.
9. A type definition emits a nonrecursive Lean `def` whose result is a sort.
10. A predicate definition emits a nonrecursive Lean `def` returning `Prop`.
11. No definitional component accepts a proof environment.

### 15.8 Theorem-like components

A theorem, lemma, or corollary contains:

1. exactly one axiom policy;
2. exactly one proposition sentence;
3. exactly one proof;
4. no other text.

Quantified locals in the proposition are in scope throughout the proof. The declaration's Lean type is generated from the proposition IR, not from a separate signature field.

The three kinds all emit Lean `theorem`; their distinction affects canonical document labeling and IR metadata.

### 15.9 Axiom-policy syntax

`\noaxioms` means the observed set must be empty.

`\allowaxioms{A;B;C}` means the observed set must be a subset of the listed set.

`\exactaxioms{A;B;C}` means the observed set must equal the listed set.

A Lean-name list:

- uses semicolon separators;
- contains fully qualified conservative ASCII Lean names;
- is nonempty for `allow` and `exact`;
- contains no duplicates;
- is sorted by `fmt`;
- is recorded exactly in IR and manifests.

Every definition and theorem-like declaration requires a policy. There is no inherited module policy and no implicit default.

---

## 16. Structured proof language

### 16.1 General semantics

A proof is a structured program over one current goal and an ordered local context. It is not English passed to an interpreter and not Lean tactic text.

Each simple proof sentence or structured proof environment constructs one `Proof` IR node. The Lean backend lowers that node to a fixed tactic pattern.

A proof block MUST leave no goals. LexLean performs conservative structural checking; Lean elaboration is final. A proof that Rust accepts but Lean rejects is a verification failure, never a verified result.

### 16.2 Simple proof sentences

The exact simple sentences are:

```text
Assume \(x\).
Assume \(x\), \(y\).
Apply TERM.
Close the goal with TERM.
Close the goal by reflexivity.
Use TERM as the witness.
Select the left alternative.
Select the right alternative.
```

Semantics:

| Sentence | IR / Lean lowering |
|---|---|
| `Assume ...` | introduce the next leading goal binders; lower to `intro` |
| `Apply TERM.` | apply a proof term; valid only when exactly one residual goal remains |
| `Close the goal with TERM.` | `exact TERM` |
| `Close the goal by reflexivity.` | `rfl` |
| `Use TERM as the witness.` | provide the next existential witness |
| `Select the left alternative.` | choose `Or.inl` |
| `Select the right alternative.` | choose `Or.inr` |

A local named by `Assume` must be fresh in the current proof scope. The displayed spelling is mapped to a deterministic generated Lean local name.

### 16.3 Have

Syntax:

```latex
\begin{have}{h}
PROPOSITION.
\begin{proof}
PROOF-STEPS
\end{proof}
\end{have}
```

The nested proof establishes the proposition and introduces `h` for all subsequent steps in the containing proof block. Lowering is:

```lean
have <generated-h> : <proposition> := by
  <nested-proof>
```

The nested proof cannot see `h` before it is established.

### 16.4 Rewrite

Syntax:

```latex
\begin{rewrite}{goal}
\forward{PROOF-TERM}
\backward{PROOF-TERM}
\forward{PROOF-TERM}
\end{rewrite}
```

or with a hypothesis target:

```latex
\begin{rewrite}{h}
...
\end{rewrite}
```

Rules:

- the target is exactly `goal` or an in-scope proof-local spelling;
- at least one rule is required;
- rules are applied strictly in source order;
- `\forward` uses the equality or equivalence left-to-right;
- `\backward` uses it right-to-left;
- each rule contains exactly one proof term;
- no global rewrite set is consulted.

Lean lowering is a single `rw` command with ordered rules and explicit reverse markers, optionally `at <target>`.

This ordered, per-rule direction model is the complete multi-rule rewrite semantics.

### 16.5 Simplify

Syntax:

```latex
\begin{simplify}{goal}
\rule{PROOF-TERM}
\rule{PROOF-TERM}
\end{simplify}
```

The target rule is the same as rewrite. At least one rule is required.

Lowering uses `simp only [rules...]`, optionally at one named hypothesis. The ambient simp theorem set is not used. Kernel reduction and the fixed behavior of pinned Lean's `simp only` remain part of the Lean toolchain.

### 16.6 Structured apply

When application yields zero or more than one residual premise, the structured form is required:

```latex
\begin{apply}{PROOF-TERM}
\begin{premise}{1}
PROOF-STEPS
\end{premise}
\begin{premise}{2}
PROOF-STEPS
\end{premise}
\end{apply}
```

Rules:

- premise labels are consecutive decimal integers beginning with 1;
- every residual premise occurs exactly once;
- each premise is a nested proof scope;
- branch order follows the explicit premise order in the selected proof constant's declared signature;
- a mismatch is a proof-shape diagnostic before Lean invocation when the signature suffices, otherwise a remapped Lean diagnostic.

### 16.7 Constructor

Syntax:

```latex
\begin{constructor}
\begin{branch}{1}
PROOF-STEPS
\end{branch}
\begin{branch}{2}
PROOF-STEPS
\end{branch}
\end{constructor}
```

The branch count and order must match the target constructor's explicit proof fields. This form covers conjunction, biconditional constructors, and explicitly declared structures available through glossary metadata.

### 16.8 Cases

Syntax:

```latex
\begin{cases}{TERM}
\begin{case}{qualified-constructor-entry}
\bind{x;y}
PROOF-STEPS
\end{case}
\end{cases}
```

Requirements:

- the scrutinee type has a glossary eliminator descriptor;
- every constructor appears exactly once;
- case order is canonicalized to descriptor order;
- `\bind{}` contains a semicolon-separated list of fresh local spellings;
- binder count matches the descriptor's constructor fields;
- branch locals are scoped to the case;
- the backend emits pinned Lean `cases` syntax with named alternatives.

### 16.9 Induction

Syntax:

```latex
\begin{induction}{TERM}
\begin{case}{qualified-constructor-entry}
\bind{x;ih}
PROOF-STEPS
\end{case}
\end{induction}
```

Requirements mirror cases. The eliminator descriptor distinguishes constructor fields from induction hypotheses and fixes their binding order. Every case occurs exactly once.

The backend emits pinned Lean `induction` syntax. LexLean does not synthesize an induction principle or infer missing cases.

### 16.10 Calculation chains

Syntax:

```latex
\begin{calculate}
\start{TERM}
\step{qualified-relation-entry}{TERM}{PROOF-TERM}
\step{qualified-relation-entry}{TERM}{PROOF-TERM}
\end{calculate}
```

Rules:

- at least one step is required;
- every relation entry is identical;
- the relation has a glossary calculation descriptor;
- each proof term establishes the relation from the previous term to the new term;
- the first and last terms match the current goal's endpoints;
- language 1.0 ships an equality calculation descriptor;
- mixed-relation transitivity is not accepted.

The Lean backend emits a `calc` block.

### 16.11 Eliminator descriptors

A type entry may contain:

```toml
[eliminator]
cases_lean_name = "Nat.casesAuxOn"
induction_lean_name = "Nat.recAux"

[[eliminator.constructor]]
entry = "lexlean.std.nat::zero"
lean_name = "Nat.zero"
fields = []
induction_hypotheses = []

[[eliminator.constructor]]
entry = "lexlean.std.nat::succ"
lean_name = "Nat.succ"
fields = ["n"]
induction_hypotheses = ["ih"]
```

The descriptor is an interface, not trusted proof. External-interface probes and actual generated proofs must elaborate under Lean.

### 16.12 Forbidden proof forms

The proof parser MUST reject:

- arbitrary imperative sentences;
- unregistered synonyms;
- raw tactic blocks;
- tactic quotations;
- semicolon tactic combinators;
- tactic repetition;
- unrestricted simplification;
- unrestricted search;
- `native_decide`;
- generated or authored proof holes;
- an empty proof;
- a proof step after the current branch has closed;
- a branch that does not close.

---

## 17. Semantic elaboration and IR

### 17.1 Compiler phases

The required phase order is:

1. locate project root;
2. parse and validate project configuration;
3. read and validate lock;
4. normalize and scan source;
5. load and validate lexicon closure;
6. build primitive-atom coverage and token lattice;
7. parse structural components;
8. parse terms, propositions, and proofs;
9. resolve globals and locals;
10. conservatively elaborate candidates;
11. reject zero or multiple distinct interpretations;
12. link modules and document declarations;
13. construct linked IR;
14. validate every declaration and proof shape;
15. compute source and semantic content IDs;
16. render artifacts;
17. optionally verify.

No backend is invoked before linked IR is complete.

### 17.2 Core reference types

A global reference is exactly:

```rust
pub enum GlobalRef {
    Core(CoreRef),
    External(ExternalConstRef),
    Document(DocumentDeclRef),
    DefinedLexicon(DefinedLexiconRef),
}
```

`ExternalConstRef` contains:

```rust
pub struct ExternalConstRef {
    pub package: QualifiedPackageId,
    pub entry: QualifiedEntryId,
    pub lean_module: LeanModuleName,
    pub lean_name: LeanName,
    pub signature_hash: Sha256Digest,
}
```

A document reference contains logical module and component IDs plus the linked generated Lean name.

A local reference is `LocalId`, an opaque monotonically assigned integer unique within one linked project. Display spellings are metadata, not identity.

### 17.3 Term IR

The linked term IR MUST be capable of representing:

```rust
pub enum Term {
    Sort(Universe),
    Local(LocalId),
    Global(GlobalRef, Vec<Universe>),
    App {
        function: Box<Term>,
        explicit_args: Vec<Term>,
        omitted_implicit_binders: Vec<ImplicitBinderId>,
    },
    Pi {
        binders: Vec<Binder>,
        body: Box<Term>,
    },
    Lambda {
        binders: Vec<Binder>,
        body: Box<Term>,
    },
    Let {
        binder: Binder,
        value: Box<Term>,
        body: Box<Term>,
    },
    NatLiteral {
        decimal: String,
        expected_type: Box<Term>,
    },
}
```

Logical surface constructs are desugared to applications of core globals. There is no opaque prose term.

### 17.4 Proof IR

The proof IR contains exactly:

```rust
pub enum Proof {
    Sequence(Vec<ProofStep>),
    Intro(Vec<LocalId>),
    Exact(Term),
    ApplyOne(Term),
    Apply {
        function: Term,
        premises: Vec<Proof>,
    },
    Reflexivity,
    Witness(Term),
    SelectLeft,
    SelectRight,
    Have {
        local: LocalId,
        proposition: Term,
        proof: Box<Proof>,
    },
    Rewrite {
        target: RewriteTarget,
        rules: Vec<RewriteRule>,
    },
    SimplifyOnly {
        target: RewriteTarget,
        rules: Vec<Term>,
    },
    Constructor(Vec<Proof>),
    Cases {
        scrutinee: Term,
        cases: Vec<CaseProof>,
    },
    Induction {
        scrutinee: Term,
        cases: Vec<CaseProof>,
    },
    Calculate {
        relation: GlobalRef,
        start: Term,
        steps: Vec<CalculationStep>,
    },
}
```

No “custom”, “raw”, “plugin”, or “unknown” variant exists.

### 17.5 Document IR

A linked project contains a sorted map of `ModuleName` to `DocumentModule`. A module contains:

- normalized source identity;
- explicit glossary closure;
- explicit module imports;
- title phrase IR;
- ordered sections;
- ordered declarations;
- complete origin tables.

A declaration contains:

- source component ID;
- generated Lean name;
- declaration kind;
- inherited section parameters;
- statement or definition term;
- optional proof, required exactly when theorem-like;
- explicit axiom policy;
- origin node ID.

### 17.6 Conservative type-directed resolution

LexLean uses glossary signatures to:

- check arity;
- instantiate explicit binders;
- propagate expected categories and types;
- resolve overloaded lexical candidates;
- reject syntactically impossible applications;
- record omitted implicit binders.

LexLean MUST NOT claim complete Lean definitional equality. If conservative elaboration cannot choose uniquely, it rejects rather than delegating candidate choice to Lean. Lean may still reject a unique LexLean IR, and such rejection is a verification failure.

### 17.7 Definition linking

Document-entry signatures are compared against declaration statements using canonical LSE after local-binder substitution. A mismatch is detected before rendering.

Document declarations are available only after their source position. Module imports expose all exported declarations of the imported module.

### 17.8 Name generation

- Full module: `<module_prefix>.<source-module>`.
- Component Lean name: component ID with `-` changed to `_`.
- A component ID whose conversion collides with another component or a Lean keyword is rejected.
- Generated local names are `llv0`, `llv1`, ... in binder-introduction order.
- Generated proof hypothesis names are `llh0`, `llh1`, ... .
- Source spellings are retained for canonical LaTeX and diagnostics.
- `set_option autoImplicit false` is always emitted.

### 17.9 Canonical IR serialization

The semantic ID uses a canonical JSON serialization of linked IR:

- schema-tagged;
- no floating-point values;
- object keys in ASCII byte order;
- arrays in semantic order;
- lower-case hexadecimal;
- minimal JSON escaping;
- no insignificant whitespace;
- no trailing LF in the hashed payload.

Opaque Rust debug output is never hashed or exposed as a stable format.

### 17.10 Native closed core modules

A source module may use the ordinary declaration environments above or one
native `coremodule`, never both. The native form occurs after the module
header and title and contains exactly one `\coredata{...}` argument whose
contents are canonical JSON with schema `lexlean/core-module/1`.

The schema is generic. It contains a child-before-parent DAG of closed kernel
expressions (universe levels, de Bruijn variables, sorts, constants,
applications, lambdas, dependent function types, lets, literals, and
structure projections), plus declarations for definitions, theorems,
inductives, structures, classes, instances, constructors, and recursors.
Every declaration carries its universe parameters, exact type, applicable
value or proof, kernel reducibility hints, effective elaborator transparency,
complete structure field and parent registration, class status, exact
instance priority, attribute kind and synthesis order, and an explicit none,
allow-subset, or exact axiom policy. There is no Atlas-specific node and no
backend-source field. A declaration's global name is also its component
identity for a document denotation owned by that native module.

Linking rejects a noncanonical schema discriminator, a forward or out-of-range
DAG edge, a malformed or repeated global name, inconsistent declaration
metadata, an absent constructor, a missing explicit axiom policy, and any
resource-limit overrun. A native module contributes all expression and
declaration nodes to `max_ir_nodes` and contributes its normalized source,
decoded semantic value, and policy data to the project identities.

Both backends traverse this same linked value. The LaTeX backend renders the
native-environment declaration names, exact types, and definition or proof
terms as a canonical human-readable core document. The Lean backend
reconstructs `Lean.Expr` and `Lean.Declaration` values and submits every native
environment declaration through Lean's checked declaration API; generated
inductive auxiliaries may be submitted only once but remain named rows in the
IR and in the verification audit.

A lossless environment import may retain a source compiler's private
self-recursive implementation record with `generated = true`. Such a record
is not a safe kernel declaration that the native backend can recreate. It is
accepted only when no native-environment declaration refers to it; it remains
hashed import provenance and appears in neither generated backend nor the
axiom audit. Any other generated row without a native owner is rejected. This
classification is generic and structural: it is not selected by an Atlas
name, module, or feature. Generated Lean may import only the foundational
modules listed in the core data and other modules in the generated graph. The
generic environment-reconstruction runtime may privately import `Lean`; no
generated module may import an independently authored implementation of the
declarations being generated.

This form exists for lossless migration of already elaborated formal
libraries. A one-time conversion implementation can establish byte-exact
migration evidence, but it is not an input to either backend and is absent from
the release tree. Once committed, the native `.lex.tex` module is the source of
coverage and both generated artifacts.



### 17.11 Language-1.1 semantic modules

Language 1.1 adds the high-level `semanticmodule` alternative. It occurs
after the ordinary module header and title, is exclusive with prose blocks
and `coremodule`, and contains exactly one `\semanticdata{...}` canonical JSON
value with schema `lexlean/semantic-module/1`. It is semantic declaration
data, never Lean or LaTeX source. Language 1.0 rejects the environment.

Accepted semantic identifiers retain their exact names in generated Lean.
Each segment matching the pinned toolchain's reserved-token table is quoted
as an identifier in every declaration, reference, field, binder and proof
position. This does not admit names otherwise rejected by semantic validation
or weaken generated-source auditing; ordinary names keep their existing bytes.

The closed declaration variants are structure, class, explicit instance,
finite nonrecursive inductive, definition, and theorem. Structures, classes,
and inductives carry ordered explicit type parameters, fields, and positional
constructors; their schema-level `parameters` array is required to be empty so
no value index can be accepted without a corresponding closed type/application
IR. Definitions and theorems carry ordered explicit value parameters.
Instances have exactly priority 1000,
target a prior document class, carry the exact ordered field set, and are
unique for one class/type argument tuple. Definitions are nonrecursive unless
they name exactly one `Nat`, `List`, or document-inductive decreasing argument
and their body is a top-level exhaustive match on that argument. A recursive
self-call is legal only inside a match branch and passes a pattern-bound
structural subvalue in that argument position. Mutual recursion, forward references, missing or mixed
cases, recursive inductive payloads, and ambiguous instances are rejected in
linking. An `instance_value` term names its required class/type tuple and the
resolved prior instance. Linking recomputes that unique resolution and rejects
a missing, different, forward, self-referential, or cyclic instance instead of
delegating synthesis to Lean.

The closed type variants include `Type`, type parameters, `Nat`, `Bool`,
`Prop`, `Unit`, mathematical unbounded `Int`, the distinct fixed-width types
`Int8`, `Int16`, `Int32`, `Int64`, `UInt8`, `UInt16`, `UInt32`, and `UInt64`,
UTF-8 `String`, immutable `Bytes`, three-way `Ordering`, `Option`, `Result`,
`List`, and document-named types. `Option` and `Result` carry their complete
closed value/error types. They are not aliases for a sentinel, exception, or
host-width integer. JSON decoding guarantees that a `String` literal is valid
UTF-8. Generated Lean strings escape quotes, backslashes, newline, carriage
return and tab; other control scalars use Lean's four-digit Unicode escapes.
Other Unicode scalars remain literal UTF-8, without normalization or changes
to literal escape-shaped data. Source-document restrictions remain unchanged.
Digit runs within `semanticdata` JSON string values are preserved as
string bytes and are never reclassified as source numerals; this permits exact
content identities whose first hexadecimal digit is zero. JSON number tokens
remain subject to the JSON decoder's grammar. A byte literal is an even-length
lower-case hexadecimal string.
Generated Lean forbids line and block comment tokens in syntax, while the same
byte sequences inside a generated Lean string literal remain exact string
data.
Integer literals use one canonical base-ten spelling: zero is `0`; any other
value has no leading zero; only signed representations admit a leading minus;
and every fixed-width literal is rejected unless it is within the exact range
of its named type. `Int` is not range-limited or lowered through a host-sized
integer.

The closed term variants are locals, canonical natural, integer, string,
byte, and Boolean literals, unit, list nil/cons, record construction,
positional construction, field
projection, document calls, conditional, exhaustive match, equality and
natural comparison, arithmetic, Boolean connectives, propositional
conjunction, implication,
bi-implication, universal binding, and deterministically resolved instance
values. They additionally contain one typed `primitive` node whose
`operation`, ordered `arguments`, and explicit `result` select exactly one of:
`subtract`, `multiply`, `quotient`, `remainder`, `negate`, `checked_convert`,
`checked_add`, `checked_subtract`, `checked_multiply`, `checked_negate`,
`checked_quotient`, `bit_and`, `bit_or`, `bit_xor`, `bit_not`, `shift_left`,
`shift_right`, `append`, `length`, `index`, `slice`, `utf8_encode`,
`utf8_decode`, `compare_bytes`, `equal`, `split_exact`, `join`,
`parse_decimal`, or `format_decimal`. This list is closed and
backend-independent.

Unbounded subtraction and multiplication accept only equal `Nat` or `Int`
operands; unbounded negation accepts `Int`. Quotient and remainder accept two
equal `Nat` or `Int` operands and an explicit same-typed zero-divisor result.
Fixed-width conversion produces `Option` of the explicit target type. Fixed
addition, subtraction, multiplication, signed negation, and quotient produce
`Option` and return none for overflow or, for quotient, a zero divisor.
Bitwise operations preserve one fixed-width type. A shift amount is `UInt32`,
and the result is none unless it is less than the width. Append preserves an
equal list or byte type. Length returns `Nat`; index and slice return `Option`.
UTF-8 decode returns `Option String`; byte comparison is unsigned
lexicographic `Ordering`. `equal` accepts two operands of exactly the same
`Nat`, `Bool`, fixed-width integer, `String`, `Bytes`, or `Ordering` type and
uses that type's fixed decidable equality without coercion; mathematical
`Int`, containers, and document types are rejected. Exact split rejects an empty delimiter or more than
the explicit `UInt32` maximum number of fields. Join is its deterministic
delimiter intercalation. Decimal parsing accepts a value exactly when parsing,
range checking, and formatting reproduce the input byte-for-byte; formatting
is canonical base ten. No operation wraps, saturates, defaults a numeric type,
uses a host exception, or delegates its meaning to a backend-specific payload.

Members are qualified logical
module/name pairs. Arity, owner, ordered fields, local scope, match binder
count, primitive signature, result type, integer range, byte spelling, and
exhaustiveness are checked before rendering. There is no source
variant for a raw command, expression, tactic, macro, unsafe/partial/
noncomputable definition, termination annotation, or backend extension.

Structural recursion admits `Nat`, `List`, and a document-defined finite
inductive such as the closed stdlib `Option` and `Result` declarations. For
lists of bytes and byte sequences represented by such lists, the same
top-level-match, complete-constructor, and structurally-smaller recursive-call
rules apply. Native `ByteArray` is intentionally accessed through the total
index and slice primitives rather than through an open backend pattern-match.
Every semantic declaration, nested type, term, primitive argument/result,
match branch, and proof is charged recursively to `max_ir_nodes`; counting
only the top-level declaration array is forbidden.

A semantic binder its scope never mentions (a definition parameter, a
quantifier, `let`, or lambda binder, or a match pattern binder) lowers as
`_name` (a match pattern binder as `_`). Pinned Lean's unused-variable linter
warns on such a binder under its own name, verification admits no
unexpected output (§20.2), and semantic names begin with an ASCII letter, so
`_name` captures nothing. The binder's type, position, and meaning are
unchanged.

The closed proof variants are reflexivity, decidable Boolean bridging,
`simp only` over a nonempty sorted unique set of document definitions,
induction hypotheses, and the fixed `Bool.and_eq_true`, `Nat.beq_eq`, or
`Nat.blt_eq` Boolean-to-proposition bridge lemmas; constructor
congruence, cases, and structural induction. Each has one fixed proof-term or
tactic lowering and one canonical LaTeX description; users cannot supply a
tactic name or argument string. Cases and induction require a `Nat`, `List`,
or document-inductive local; their constructor set and pattern-binder arities
are checked exactly before rendering. Induction may carry a strictly sorted,
unique list of other in-scope theorem parameters to generalize; the scrutinee
cannot be generalized. Every simplification name resolves
to one of those closed choices. An axiom-free `boolean_reflection` proof is
available only in two validated
generic shapes: structural `List Nat` recursion that reflects `Nat.beq` or
`Nat.blt` into an independently recursive proposition, and a finite structure
whose complete ordered `Nat` field set is reflected from right-associated
Boolean conjunction into propositional conjunction. The proof payload names
the prior Boolean and proposition definitions and the in-scope locals; record
expected values are canonical Nat literals. The backend emits fixed natural
deduction using primitive Nat direction lemmas and locally proved Boolean
conjunction/reflexivity bridges. It does not invoke `propext` or a general
tactic.
An `apply` proof names one prior theorem and supplies exactly its ordered,
type-checked semantic arguments; it lowers to one `exact` application and
cannot carry a proof-term or tactic string.

Every definition and theorem has an exact axiom policy. An omitted `axioms`
member means the exact empty set; a present member is a strictly sorted,
duplicate-free list of Lean names and means exact equality with that set.
Structures, classes, instances, and inductives always use the exact empty set.
Computational definitions may declare a nonempty exact policy only to account
for trusted dependencies observed in their fixed Lean implementation; this
does not authorize source axioms, proof holes, raw Lean, or an unchecked
backend. Verification rejects a missing, additional, or unlisted observed
axiom for every declaration and records the declared and observed sets in the
attestation. A theorem that unfolds or otherwise depends on a computational
definition inherits every dependency Lean observes; separating declaration
policies never erases or masks that dependency.

Both fixed backends consume the same owned `SemanticModule`. Lean emits public
structures, classes, fixed-priority instances, inductives, exposed total
definitions, and theorems inside the generated logical namespace. LaTeX emits
the corresponding ordered declaration catalog. Full output coverage and maps
bind each emitted byte to the semantic payload. The snapshot embeds the
decoded module and each declaration as nested JSON rather than an encoded
string. The snapshot schema embeds the complete closed language-1.1 semantic
module definitions; it does not validate that field as an unconstrained JSON
value. Verification elaborates the generated module, replays it with
`leanchecker`, and audits every theorem's observed axioms.

The language-1.1 semantic Lean backend uses the same fixed finite resource
budgets as the native-core backend: `maxRecDepth` is `100000` and
`maxHeartbeats` is `1000000000`. Lean measures the latter per command, in
thousands of small allocations. Its interactive defaults are not adequate
for complete nested byte-data declarations. These are fixed generated
backend options, not source-supplied commands or request overrides; neither
may be zero (unlimited). Configured child timeout and output-size limits,
source/IR limits, kernel replay, and exact axiom policies remain unchanged.
Exhausting a finite Lean budget still fails verification; the budgets do not
assert that every resource-bounded source can be verified on every host.

### 17.12 Language 1.2 and the compatibility/migration contract

Language 1.2 is selected by `language = "1.2"` in `lexlean.toml`. It is a
strict extension of language 1.1: every language-1.1 structural form,
lexicon form, declaration, type, term, primitive, and proof variant is
inherited with unchanged meaning, typing, Lean lowering, and LaTeX
rendering. The constructs that exist only in language 1.2 are listed in the
closed table below; each is rejected under language 1.1 before either
backend runs.

| 1.2-only construct | Schema | Meaning |
|---|---|---|
| `let` term | `lexlean/semantic-module/2` | A typed, nonrecursive local definition `{"binder":{"name":...,"type":...},"kind":"let","value":...,"body":...}`. The value is checked in the enclosing scope and must have exactly the binder type; the binder is in scope only in `body`, may not shadow any local, and is never treated as a structurally smaller recursive argument. It lowers to the Lean term `(let x : T := v; b)` and contributes its binder type, value, and body to `max_ir_nodes`. |
| `product` type | `lexlean/semantic-module/2` | `{"kind":"product","left":T,"right":U}`, the binary product of two closed types, lowered to `(Prod (T) (U))`. |
| `pair`, `first`, `second` terms | `lexlean/semantic-module/2` | `{"kind":"pair","left":a,"right":b}` has the product of its component types and lowers to `(a, b)`; `first` and `second` require a product operand and lower to `(v).1` and `(v).2`. A `match` on a product has exactly one `Prod.mk` branch with two binders. |
| recursive inductive | `lexlean/semantic-module/2` | A constructor field may mention the inductive being declared, under the recursive-data rules below. |
| `mutual` inductive label | `lexlean/semantic-module/2` | `"mutual":"Label"` on an inductive places it in a mutual group under the recursive-data rules below. |
| `function` type | `lexlean/semantic-module/2` | `{"kind":"function","parameters":[T...],"result":U}` with at least one parameter, lowered to `((T1) -> ... -> (U))`. |
| `lambda`, `apply`, `function_ref` terms | `lexlean/semantic-module/2` | Anonymous functions with explicit captures, full application of a function value, and a document definition as a function value, under the higher-order rules below. |
| generic definitions and theorems | `lexlean/semantic-module/2` | `type_parameters` on a definition or theorem and `type_arguments` on a `call`, a `function_ref`, or an `apply` proof, under the higher-order rules below. |
| `executable` definitions | `lexlean/semantic-module/2` | `"executable":true` asserts production eligibility under the higher-order rules below. |
| `mutual` definition label | `lexlean/semantic-module/2` | `"mutual":"Label"` on a structurally recursive definition places it in a mutual definition group under the recursion rules below. |
| `termination` evidence | `lexlean/semantic-module/2` | `{"measure":m,"evidence":[theorem...]}` on a definition declares well-founded recursion under the recursion rules below. |
| `linear_arithmetic` proof | `lexlean/semantic-module/2` | `{"kind":"linear_arithmetic"}`, optionally with `definitions`, a strictly sorted list of prior document definitions and no other member. Without definitions it lowers to the fixed script `intros`, `try set_option linter.unusedSimpArgs false in simp only [← Bool.not_eq_true, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq] at *` (adding `LexLeanRuntime.subtract, LexLeanRuntime.multiply` when the module emits the portable runtime), and `all_goals omega`, so a goal the simplifier already closed leaves nothing for `omega`; with definitions, `subst_vars` follows `intros` and the definitions lead the `simp only` list, so hypotheses equating a local with a constructor value are substituted and a measure over them unfolds to linear arithmetic. |
| `map`, `set` types | `lexlean/semantic-module/2` | `{"kind":"map","key":K,"value":V}` and `{"kind":"set","element":K}` over an ordered key type, under the collection rules below. |
| `map_literal`, `set_literal`, `graph_literal` terms | `lexlean/semantic-module/2` | Literal collections of literal keys, canonicalized in linking under the collection rules below. |
| collection primitives | `lexlean/semantic-module/2` | The `primitive` operations `map_insert`, `map_remove`, `map_lookup`, `map_contains`, `map_size`, `map_keys`, `map_values`, `map_entries`, `map_fold`, `set_insert`, `set_remove`, `set_contains`, `set_size`, `set_elements`, `set_union`, `set_intersection`, `set_difference`, `set_fold`, `list_fold`, `iterate`, `iterate_until`, `graph_successors`, `graph_reachable`, and `graph_topological`, under the collection rules below. |
| `less_than` primitive | `lexlean/semantic-module/2` | `{"kind":"primitive","operation":"less_than","arguments":[a,b],"result":{"kind":"bool"}}` over two values of one ordered key type (rule 1 under *Collections and state threading*): `true` exactly when `a` precedes `b` in that type's fixed order. It lowers to `LexLeanCollections.lessThan`. |
| `artifact` declaration | `lexlean/semantic-module/2` | A typed constant decoded from content-addressed project bytes, under the model rules below. |
| `contract` declaration | `lexlean/semantic-module/2` | Typed input, output, and optional state interfaces, proposition-valued predicates, and their runtime validators, under the model rules below. |
| `realization` declaration | `lexlean/semantic-module/2` | A deterministic, rule, statistical, neural, or composite denotation of an interface, under the model rules below. |
| `evidence` declaration | `lexlean/semantic-module/2` | Closed claims about a realization against a contract, each discharged by a theorem, under the model rules below. |
| `model` declaration | `lexlean/semantic-module/2` | A contract bound to a realization of its interface, with evidence and entry theorems, under the model rules below. |
| `checked_apply` term | `lexlean/semantic-module/2` | `{"kind":"checked_apply","model":M,"type_arguments":[T...],"arguments":[...],"checks":[...]}`, a model application that runs the named validators, under the model rules below. |
| `contract_violation` type | `lexlean/semantic-module/2` | `{"kind":"contract_violation"}`, the closed type of a validator refusal, with the four nullary constructors `ContractViolation.precondition`, `.input_invariant`, `.postcondition`, and `.output_invariant`, under the model rules below. |

#### Recursive data (language 1.2)

These rules are LexLean's own and are checked in linking (`LLT4001`) before
either backend runs; they do not defer to what Lean would accept.

1. **Groups.** A standalone inductive is a group of one. Inductives carrying
   the same `mutual` label form one group: the members are contiguous in
   declaration order, there are at least two, they declare identical ordered
   type parameters, and each has at least one constructor and no value
   parameters. A member may mention any member of its group; every other
   reference is to an earlier declaration. No language-1.2 declaration is named `Bool`, `List`, `Nat`, `Option`,
   `Prod`, or `Result`: a local reference to a built-in constructor carries no
   module, so such a declaration would make it ambiguous.
2. **Uniformity.** Every mention of a group member inside a constructor field
   applies it to exactly the declared type parameters, in order.
3. **Strict positivity.** A group member occurs in a field only as the whole
   field type (a *direct* occurrence) or nested under `List`, `Option`,
   `Result`, or a product. A group member inside the type arguments of
   another document type is rejected, because the positivity of that type's
   parameters is not part of its closed contract.
4. **Inhabitation.** The least set of inhabited members is computed by
   fixed point: a member is inhabited when one of its constructors has only
   constructible fields. `List` and `Option` are always constructible, a
   `Result` when either side is, a product when both are, a group member
   when it is already inhabited, and every other accepted type always. A
   member outside the fixed point is an uninhabited recursive cycle.
5. **Structures** and classes remain nonrecursive: a field that mentions its
   own structure or class is rejected; recursive data is an inductive.
6. **Structural recursion.** When a recursive definition matches its
   decreasing argument, the structurally smaller pattern binders are the
   `List.cons` tail, the `Nat.succ` predecessor, and every field of a
   document constructor that is a direct occurrence of the matched
   inductive itself. A nested occurrence, or a member of another type in a
   mutual group, is not smaller under this rule. Imported recursive
   inductives keep the same smaller positions. A standalone recursive
   definition decreases only on `Nat`, `List`, or a self-recursive inductive
   with no nested occurrence and no mutual group, exactly the types
   `induction` accepts: a nested or mutual type has no single structural
   eliminator for one function.
7. **Induction.** `induction` applies to a self-recursive inductive with no
   nested occurrence and no mutual group, and binds the constructor fields
   followed by one hypothesis per direct recursive field, in field order.
   `cases` applies to every inductive.
8. **Lowering.** Each inductive lowers to one `public inductive` with
   positional `(_ : T)` fields; each mutual group lowers to one
   `mutual ... end` block. A constructor's identity is its owner and name,
   `Owner.ctor`, qualified by its module when imported; constructor names are
   unique within their owner and lowered in declaration order. The canonical
   LaTeX of a `lexlean/semantic-module/2` module additionally lists the type
   parameters of every parameterized inductive, structure, or class, every
   constructor with its field types, the mutual group label, and every
   structure or class field; language-1.1 documents keep their bytes.
9. **Accounting.** Every constructor field type is charged recursively to
   `max_ir_nodes`, and a `lexlean/semantic-module/2` module is also charged
   one node per type parameter, per constructor, and per mutual group label,
   so no part of a data declaration is free; language-1.1 modules keep their
   historical count.
10. **Binder hygiene.** The backend refers to the module's own declarations,
    to the built-in names it emits (`And`, `Bool`, `ByteArray`, `Except`,
    `Iff`, `Int`, `Int8`..`Int64`, `LexLeanCollections`, `LexLeanRuntime`,
    `List`, `Nat`, `Option`, `Ordering`, `Prod`, `Prop`, `Result`, `String`,
    `Type`, `UInt8`..`UInt64`, `Unit`, `and_congr`, `congr`, `decide`, `id`,
    `rfl`), and to every imported declaration through the project's module
    prefix, all without qualification. In a `lexlean/semantic-module/2`
    module no binder (a type parameter, a value parameter, or a pattern,
    `let`, quantifier, lambda, or proof binder) is spelled like a declaration of the
    module, one of those built-in names, or the first segment of the module
    prefix, because Lean would resolve the reference to the binder after
    linking has accepted the module. The language-1.1 contract is frozen
    and does not carry this rule.
11. **Source maps.** In a `lexlean/semantic-module/2` module every
    declaration is its own mapping node in both artifacts, relating its
    generated Lean and LaTeX to exactly its object in the source; the
    preamble and closing map to the whole module. Language-1.1 maps keep
    their module granularity and bytes.

#### Higher-order code (language 1.2)

1. **Generic declarations.** A definition or theorem may declare ordered,
   unique `type_parameters`; they lower to explicit `(T : Type)` binders
   before the value parameters. Every `call`, `function_ref`, and theorem
   `apply` supplies exactly the callee's type arguments, explicitly: there is
   no inference, so no use is ambiguous. Every type written in a
   definition's parameters, result, or body, or in a theorem's statement or
   proof arguments, mentions only declared type parameters; this also holds
   for language-1.1 definitions, whose scope is empty. A language-1.2 type
   parameter obeys binder hygiene (rule 10 of the recursive-data rules: no
   declaration of the module, built-in name, or module prefix root), and is
   moreover not spelled like any value binder of the same declaration. An
   imported document type is referred to through the module prefix, so a
   type parameter of its spelling captures nothing. A type parameter that its declaration
   mentions nowhere (no parameter, result, body, statement, or proof type)
   lowers as `(_T : Type)`, so Lean's unused-variable linter stays silent. An
   explicit type argument instantiates a `(T : Type)` binder and so never
   mentions the universe `Type`.
2. **No polymorphic recursion.** A recursive call passes exactly the
   definition's own type parameters, in order (the `polymorphic-recursion`
   negative fixture). Consequently a closed instantiation of a definition
   reaches only finitely many instantiations: a recursive call repeats the
   current one, and every other call is to an earlier definition at type
   arguments built from the caller's. This is the monomorphization rule.
   LexLean emits the generic definitions with explicit `(T : Type)` binders
   and the explicit type arguments of every use; it does not emit
   specialized copies, and no claim here depends on one.
3. **Lambdas.** A `lambda` binds at least one typed parameter and lists its
   `captures`: strictly sorted, unique enclosing locals that are exactly the
   free locals of its body. Only the captures and the parameters are visible
   in the body, and a parameter may not shadow any enclosing local. A
   recursive definition may not call or reference itself inside a lambda.
   A lambda lowers to `(fun (x : T) ... => body)`.
4. **Application.** `apply` fully applies a function-typed value: the
   argument count equals the function type's parameter count and each
   argument has the parameter's type; a function value with no statically
   known function type is not applied. It lowers to `(f (a) ... )`.
   `function_ref` names a prior definition with at least one value
   parameter at explicit type arguments and has its function type.
5. **Identity.** Binder names are part of the semantic value and therefore
   of every source and semantic identity. A language-1.2 snapshot also
   records, for each source definition (and, in an elaboration, each
   elaborated definition), `alpha_id`: the SHA-256 of the canonical
   JSON of the definition after renaming its type parameters to `T0, T1, ...`
   and its value parameters and every term binder to `_0, _1, ...` in
   evaluation order (the parameters, then the body's subterms as they are
   evaluated: an application's function before its arguments, a
   conditional's condition, then branch, and else branch, operands left to
   right), each binder scoped to its own subterm, with every lambda's
   renamed captures sorted again; the order does not depend on how the
   definition serializes. Alpha-equivalent definitions share it; any
   other change alters it.
6. **Executable closures.** Higher-order values are formal by default. A
   definition marked `"executable":true` is admitted only when every
   closure it forms is non-escaping and second-class. `executable` is a
   precondition of production eligibility, never production eligibility
   itself, which is decided per root and target by §17.13. A type *holds a
   function* when it is a function type, or a container, product, or
   document type that holds one through its type arguments or, transitively,
   through the field types of its declaration. Its result type and every
   non-function parameter type hold no function; no type written in its body
   (a type argument, a `let`, `nil`, or lambda parameter type, a primitive
   result type) holds one, so no data value in it carries a closure; a
   function-typed parameter takes and returns no function and occurs only
   where a closure may; a lambda or
   function reference occurs only as a direct argument to a function-typed
   parameter of an executable callee, or as the function of an `apply`; the
   function of an `apply` is a parameter, a lambda, or a function reference,
   never a projection or other computed value; no `let` binds a function;
   and every called or referenced definition is itself executable (or the
   definition itself, recursively). Any violation fails closed in linking;
   nothing is silently lowered.
7. **Accounting.** Function types, lambda parameter types, type arguments,
   and every subterm are charged to `max_ir_nodes`. Recursive occurrences of
   an inductive group under a function type are rejected by positivity.

#### Recursion (language 1.2)

Termination evidence is semantic data, never tactic or backend text; no
`partial`, `unsafe`, `sorry`, or source `termination_by` exists.

1. **Structural recursion** of a single definition follows the
   recursive-data rule 6 above.
2. **Mutual groups.** Definitions carrying the same `mutual` label form one
   group: contiguous, at least two members, identical type parameters, and
   either every member structurally recursive (`recursive_argument`, a
   top-level match on it, no `termination`) or every member well-founded
   (rule 3); a group mixing the two is rejected. The *recursion call graph*
   of a group has an edge from each member to every member its body calls
   or references; every member calls into the group and the graph is
   strongly connected, so no member is outside the mutual recursion. For a
   structural group: Every member's decreasing type belongs to one *recursive
   family*: the naturals; the lists of one element type; or one inductive
   group at one instantiation of its type parameters, together with `List`
   or `Option` of its members exactly when the group's constructors nest a
   member under that container. In a member's
   top-level match, every pattern binder whose type belongs to the family is
   structurally smaller, and every call to a group member passes a smaller
   binder in that member's decreasing position. Members see each other; the
   recursion call graph is exactly these calls. A group lowers to one
   `mutual ... end` block whose members each declare
   `termination_by structural x1 ... xn => d`, so Lean must confirm the same
   structural recursion and no other; every `xi` other than the decreasing `d`
   is written `_`, because a named binder the measure never mentions is an
   unused variable that verification refuses (§17.11). A label names one group
   of one kind: a label carried by both an inductive group and a definition
   group is rejected. No member of the group being checked is called or
   referenced under a lambda, and none is referenced as a value.
3. **Well-founded recursion.** A definition with `termination` has no
   `recursive_argument`; it is standalone, a group of one, or a member of a
   well-founded mutual group. Its `measure` is a binder-free term of type
   `Nat` over its parameters that mentions no member of its group. Its
   recursive calls (calls to any member of its group) pass the group's own type parameters, do not
   occur in another recursive call's argument or under a `lambda` or
   quantifier, and the definition is never referenced as a value. A call's
   arguments, and every enclosing `if` condition and `match` scrutinee,
   mention only parameters and the binders of enclosing matches, and those
   binders repeat no parameter or other enclosing binder. Every `if` and
   `match` of the body is numbered in one fixed pre-order; each call site,
   in that order, has the *decrease obligation*
   `h1 -> ... -> measure[args] < measure` over its enclosing nodes, outermost
   first, where the conclusion compares the *callee's* measure at the call's
   arguments with the caller's own measure, and an `if` contributes `c = b` for its condition `c` and branch
   polarity `b`, and a `match` contributes `s = v` for its scrutinee `s` and
   the value `v` the branch's pattern denotes over its binders (a Boolean
   literal, a pair for `Prod.mk`, and otherwise the constructor applied to
   the binders at the scrutinee type's arguments). `evidence` lists one
   prior theorem of the same module per call site, in order, whose type
   parameters equal the definition's, whose parameters are the definition's
   followed by every enclosing match binder with its type, outermost first,
   and whose statement equals the obligation exactly. Linking rejects any
   other evidence, so no recursive call enters the linked IR without its
   exact, separately named decrease obligation: unguarded recursion cannot
   enter it. Whether an obligation is *true* is a proof, and LexLean, as for
   every proof, leaves it to Lean's kernel at verification (§22): a false
   evidence theorem with the exact statement links, and verification refuses
   both the theorem and the definition (Lean independently refuses a
   nonterminating well-founded definition), so the project is never
   verified (`recursion-false-evidence`). An `executable` well-founded
   definition may be reached by a production root (§17.13), and no
   production claim holds for a project that is not verified.
4. **Well-founded lowering.** The definition lowers to
   `@[expose, semireducible] public def`, each `if` to
   `match (generalizing := false) __decreaseN : c with | true => ... | false => ...`,
   each `match` to `match (generalizing := false) __decreaseN : s with ...`,
   then `termination_by measure` and `decreasing_by all_goals first` with one
   `| (have __evidence := evidence T... p... _... __decreaseI...; subst_vars; exact __evidence)`
   per call site, passing the parameters, one `_` per enclosing match binder,
   and the hypotheses of its enclosing nodes. `generalizing := false` keeps
   every earlier hypothesis stated over the parameters, exactly as the
   evidence states it; each binder appears in its match's hypothesis, so Lean
   solves the `_` by unification and a binder the branch ignores stays
   `_`; Lean still refines the decreasing goal by each match on a variable,
   and `subst_vars` applies the same substitutions to the evidence. Each
   goal is closed by the first evidence theorem whose statement it is, after
   those substitutions; since every evidence theorem states exactly one call
   site's obligation, which theorem closes a goal is not observable, and
   Lean presents a cross-member goal of a mutual group as the callee's
   measure at the arguments against the caller's measure, the obligation's
   own conclusion. A well-founded mutual group lowers to one `mutual ... end`
   block of such definitions. A parameter
   neither the body nor the measure mentions lowers as `_name` and is passed
   as `_name`; generated names begin with two underscores, which no lowered
   source binder does. Semireducibility lets closed instances reduce in
   proofs; the evidence's observed axioms flow into the definition's exact
   policy.
5. **Theorem binders** that neither the statement nor the proof mentions
   are bound as `_name`, so an evidence theorem may take every parameter of
   its definition without a Lean linter warning.
6. **Closed members.** Every member of a semantic-module value must survive
   into the typed value: a member the closed schema does not define, such as
   an extra member of a unit variant, is rejected in linking in every
   language.

#### Collections and state threading (language 1.2)

1. **Ordered keys.** A map key, set element, or graph node type is one of
   `Nat`, `Int`, the fixed-width integers, `Bool`, `String`, or a product of
   such types. Each has one fixed total order: integers numerically,
   `false` before `true`, strings by Unicode scalar sequence (equivalently
   UTF-8 byte order), and products lexicographically. Any other key type is
   rejected, because its iteration order would be unspecified. No host hash
   or pointer order exists anywhere in the semantics. The generated Lean
   `Key` instances realize the same orders, and `examples/collections`
   states for every key type that a linked literal equals Lean's own
   insertion of the source order, so pinned Lean checks linking's order.
2. **Representation.** A map value is its strictly ascending entry list and
   a set its strictly ascending element list, so equal maps and sets are
   structurally equal and iterate identically. Generated Lean represents
   `map K V` as `List (Prod K V)` and `set K` as `List K`, and the
   canonical document names them `Map (K) (V)` and `Set (K)`. Literals and
   the closed operations are the only producers, and each operation maps
   strictly ascending lists to strictly ascending lists; `SM-28` and `SM-30`
   check this differentially, under Lean, against an independent ordered-map
   model on seeded random operation sequences. Memory is one entry or
   element per member. Every literal entry, element, node, and edge is
   charged to `max_ir_nodes` in linking, so an oversized literal is
   `LLS8002` before either backend runs; a computed collection is a runtime
   value whose size is bounded only by the operations that build it, and
   the cost of each operation is stated in item 4.
3. **Literals.** A `map_literal`, `set_literal`, or `graph_literal` is keyed
   only by literal values (natural, integer, string, and Boolean literals
   and pairs of them); other keys are inserted with `map_insert` or
   `set_insert`. Each literal key is first checked as a term, so its
   spelling is canonical and a fixed-width key is in range; only then does
   it have a position in the order. Linking rejects a duplicate key,
   element, node, or edge, and
   a graph edge whose source or target is not a declared node, then sorts
   every literal into canonical order. Reordered equivalent source therefore
   links to identical semantic data and semantic identities, and generates
   byte-identical Lean modules, LaTeX modules, and lexicon closures; only
   the source identity and the artifacts that record source positions (the
   build manifest, source maps, and coverage) record the textual order.
4. **Operations.** Each collection primitive has one explicit result type:
   insertion replaces an existing key, removal of an absent key is the
   identity, lookup returns `Option`, keys/values/entries/elements are
   ascending lists, union inserts the right operand's elements into the
   left, intersection and difference keep the left operand's order, and
   `map_fold`, `set_fold`, and `list_fold` visit members in ascending (or
   list) order with the state as the step's first argument. A graph is
   `map node (set node)`. Its *nodes* are its keys together with every
   successor, so a successor inserted without an entry of its own is a node
   with no successors; `graph_successors` of an absent node is empty;
   `graph_reachable` is the breadth-first closure including the start node,
   computed in at most node-count-plus-one rounds, each of which either adds
   a node or ends the search, so the bound never truncates the closure;
   `graph_topological` is Kahn's order over every node, taking the least
   ready node first, removing one node per round within the same bound, or
   none when a cycle remains. Costs, in key comparisons, for collections of
   `n` and `m` members, a graph of `V` nodes and `E` edges, and maximum
   out-degree `d`: insertion, removal, lookup, and membership `O(n)`;
   size, keys, values, entries, elements, and folds `O(n)` steps; union
   `O(m(n + m))`; intersection and difference `O(nm)`; successors `O(V)`;
   reachability `O(V^2(V + E))`; topological order `O(V^3(V + d))`. Every
   intermediate list holds at most the members of its result or of the
   graph's node set.
5. **State threading.** State is explicit and pure: `list_fold`, `map_fold`,
   `set_fold`, `iterate` (exactly `n` steps), and `iterate_until` (until the
   step returns none or its natural-number fuel is exhausted, returning the
   final state and whether a fixed point was reached) take the step as a
   function argument. The count and the fuel are required arguments, so
   every iteration performs at most that many steps; an iteration without
   one is rejected in linking. No ambient mutable state exists. A lambda or function reference passed as a step is in a closure
   position, so executable definitions may use these combinators.
6. **Runtime.** In language 1.2 every definition of the emitted portable
   runtime is exposed (`@[expose, noinline]` where language 1.1 has
   `@[noinline]`), so a definition imported from another module reduces in
   the kernel through the primitives it applies; `noinline` is kept, and the
   language-1.1 runtime text is frozen. A module that writes a
   collection type or literal, or
   applies any collection primitive (including one applied only to a
   collection imported from another module), emits the fixed
   `LexLeanCollections` runtime in its own namespace:
   the `Key` order instances and the operations above as exposed, total
   Lean definitions over lists, with no comment tokens.

#### Models, contracts, realizations, and evidence (language 1.2)

A model is a contract bound to a realization of the same interface, with
evidence about the realization. These rules are LexLean's own and are checked
in linking, before either backend runs; the diagnostic each failure raises is
named with the rule. Every model declaration *elaborates*, in linking, to
ordinary language-1.2 definitions and theorems. The Lean backend, production
eligibility, extraction, and the axiom audit read only the elaboration; the
canonical document and the snapshot read the source declaration together with
its elaboration (rule 13). No prompt text, free-form description, raw JSON,
unchecked code, or untyped blob defines any meaning: every member of every
model declaration belongs to the closed `lexlean/semantic-module/2` schema,
every closed set below is a closed enumeration of that schema, and an unknown
member or value is an ordinary schema failure (`LLT4001`).

1. **References and order.** Every contract, realization, evidence, model,
   predicate, validator, artifact, and theorem a model declaration names is
   a prior declaration of its module or an imported one. Every theorem named
   by a validator, a width obligation, a claim, a junction, or an entry is a
   prior theorem *of the same module* as the declaration naming it, as for
   termination evidence. A use of a generic contract, realization, or model
   is `{"member":M,"type_arguments":[T...]}` with exactly its type
   parameters; nothing is inferred. A model declaration's `axioms` row is
   the exact axiom policy of every Lean declaration it generates (rule 12).
2. **Artifacts** (`LLR3007`, `LLR3008`). `{"kind":"artifact","name":W,
   "role":R,"sha256":H,"length":N,"schema":S,"type":T}` names the project
   bytes configured as `[[artifact_source]]` with digest `H` (§10.1). Linking
   reads each configured artifact once, confined (§25.1), within
   `max_file_bytes`, and counted toward `max_total_source_bytes` (§10.2), and
   requires its SHA-256 to equal the configured digest. A missing file, a
   digest or length differing from the configuration or the declaration, or
   a declaration whose digest no configured source has is `LLR3007`. The
   closed schemas are `bytes` (decoding to `bytes`); `int_tensor` with an
   `element` among `int8`, `int16`, `int32`, `int64`, `uint8`, `uint16`,
   `uint32`, `uint64` and a nonempty `shape` of at most 16 positive extents
   (little-endian, two's complement for the signed elements, row-major,
   exactly width times the product of the extents bytes, decoding to the
   `shape`-deep nested `list int` of the exact element values); and
   `utf8_lines` (valid UTF-8 without CR, every line LF-terminated, decoding
   to `list string`). The closed roles constrain the schema: `parameters`
   and `table` take `int_tensor`, `vocabulary` takes `utf8_lines`, `binary`
   takes `bytes`, and `dataset` takes any. Bytes that do not decode under
   the schema, a declared type other than the schema's decoded type, or a
   role its schema does not admit is `LLR3008`. The rank bound, the role,
   and the declared type are checked, and the bytes checked against the
   schema, before any byte is decoded. Each declaration is then charged
   before it materializes anything, because one digest may be named by any
   number of declarations and each holds its own copy: its length counts
   once more toward `max_total_source_bytes`, and the exact node count of
   its decoded value (computed from the length, the shape, and the line
   count, without building it) is charged to `max_ir_nodes` on top of every
   node linked and decoded before it in the link; exceeding either is
   `LLS8002` before decoding allocates. Decoding a project's artifacts
   therefore allocates at most a constant multiple of
   `max_total_source_bytes` bytes and `max_ir_nodes` nodes. There is no
   floating-point
   element, no JSON, ONNX, or other configuration schema, and no
   unspecified rounding. An artifact elaborates to `W.bytes : bytes`, its
   exact bytes, and `W : T`, the decoded value; generated Lean adds the
   kernel-checked theorem `W.decoded` that `W` is the decoding of
   `W.bytes` (by `decide` for tensors and lines, by `rfl` for bytes), so the
   typed value used everywhere is tied to the exact content-addressed bytes.
   `W.bytes` is emitted from the bytes linking read and digest-checked
   against both the configuration and the declaration, in the same process
   that renders the module; verification does not re-digest the emitted
   literal, so that link rests on construction, while the link from the
   bytes to the typed value is the kernel's.
3. **Contracts** (`LLT4006`). `{"kind":"contract","name":C,
   "type_parameters":[...],"input":{"name":x,"type":I},
   "output":{"name":y,"type":O},"state":{"name":s,"next":t,"type":S}?,
   "precondition":P?,"postcondition":Q?,"invariant":J?,"validators":[...]}`.
   The binder names are pairwise distinct legal names; `invariant` requires
   `state`; at least one predicate is present. Each predicate is a prior
   definition with the contract's type parameters, result `prop`, and the
   parameter types of the table below; anything else is `LLT4006`.

   | Contract | Precondition | Postcondition | Invariant |
   |---|---|---|---|
   | stateless | `(I)` | `(I, O)` | — |
   | stateful | `(S, I)` | `(S, I, S, O)` | `(S)` |

   `validators` are strictly sorted by predicate, at most one per present
   predicate: `{"predicate":p,"validator":V,"sound":T,"complete":U?}`. `V`
   is a prior `executable` definition of the predicate's parameter types
   with result `bool`; `T` states exactly `V(args) = true -> p(args)` and
   `U`, when present, exactly `p(args) -> V(args) = true`, over the
   predicate's parameters named by the contract binders (statements are
   compared up to the names of bound variables). A missing, mistyped, or
   non-executable validator, an absent predicate, or an inexact theorem is
   `LLT4006`. A contract elaborates to no runtime declaration; generated
   Lean restates each link as `C.<predicate>_sound` and
   `C.<predicate>_complete` against the fixed `LexLeanModels.SoundN` and
   `CompleteN` (rule 12).
4. **Realizations** (`LLT4006`). `{"kind":"realization","name":R,
   "type_parameters":[...],"input":{"name":x,"type":I},"output":O,
   "state":{"name":s,"type":S,"initial":v}?,"descriptor":D,
   "executable":true?}` elaborates to the ordinary definition
   `R : (I) -> O`, or `R : (S, I) -> product S O` with `R.initial : S` when
   stateful, plus the companion definitions named below; companions are
   ordinary definitions under the realization's name and occupy their names
   (§17.8). Each descriptor term has the type its slot fixes (a guard
   `bool`, an action, default, or label `O`, an encoder or feature map
   `list int`, a denotation `O` or `product S O`); a term of another type is
   an interface mismatch, `LLT4006`. The closed descriptors:
   - `deterministic` `{"body":b}`: the denotation is `b`.
   - `rule` `{"rules":[{"name":n,"guard":g,"action":a}...],"default":d}`:
     at least one rule, names unique; the denotation is the first rule, in
     declared order, whose guard holds, else `d`; the companion
     `R.selected : (I) -> nat` is the index of the firing rule, or the rule
     count for the default.
   - `statistical` `{"scheme":"linear_scoring","features":f,"width":w,
     "width_evidence":T,"weights":Wt,"bias":B,"labels":[...]}`, stateless:
     the companions `R.features` (`f`) and `R.scores` (`Wt` times the
     features plus `B`, over mathematical integers) and the denotation, the
     label of the first maximal score.
   - `neural` `{"architecture":"integer_feedforward","encoder":e,"width":w,
     "width_evidence":T,"layers":[...],"decoder":...}`, stateless: the
     companions `R.encoded` (`e`), `R.layer1` ... `R.layerK` (one per layer,
     each `list int -> list int`), and `R.logits`, and the denotation
     decoding the final values. The closed layers are `dense`
     `{"inputs":n,"outputs":m,"weights":Wt,"bias":B}` (`Wt v + B` over
     mathematical integers), `relu` (negative values become zero), and
     `requantize` `{"shift":k,"minimum":lo,"maximum":hi}` (division by `2^k`
     truncating toward zero, then clamping to `[lo, hi]`; `lo <= hi` and
     `k <= 4096`). The closed decoders are `argmax` `{"labels":[...]}` (the
     label of the first maximal value) and `function` `{"binder":v,
     "body":b}` (`b` over the final values bound to `v`).
   - `composite` `{"form":F}`: rule 7.

   Widths are checked statically: the first dense layer takes `width`
   inputs, each dense layer takes the current width, a weights slot is a
   `parameters` artifact of shape `[outputs, inputs]` (statistical:
   `[labels, width]`) and a bias slot one of shape `[outputs]`, and an
   argmax decoder has one label per final value; a label list is nonempty
   and each label is a closed term, mentioning no local. `width_evidence`
   states exactly
   `length(e) = w` over the input binder. Any other width, shape, role, or
   label count, or an inexact width theorem, is `LLT4006`. The kernels are
   ordinary terms over `list_fold`, `match`, the integer primitives, and
   `less_than`, so they mean exactly what Lean computes and nothing is
   approximated. A realization marked `executable` is checked as executable
   code, including its companions.
5. **Evidence** (`LLT4009`). `{"kind":"evidence","name":E,
   "type_parameters":[...],"contract":Cuse,"realization":Ruse,
   "claims":[...]}` about a contract and a realization of the same interface
   (otherwise `LLT4006`). `claims` are strictly sorted by kind and theorem.
   The closed claim kinds, their generated statements (premises absent from
   the contract are dropped; `R` is the realization at its type arguments),
   and the fixed Lean helper each is restated against:

   | Claim | Requires | Generated statement | Helper |
   |---|---|---|---|
   | `satisfies_contract` | postcondition | stateless: `P x -> Q x (R x)`; stateful: `J s -> P s x -> Q s x (first(R s x)) (second(R s x))` | `Satisfies`, `SatisfiesTotal`, `SatisfiesStep...` |
   | `preserves_invariant` | invariant | `J s -> P s x -> J (first(R s x))` | `Preserves`, `PreservesTotal` |
   | `initial_invariant` | invariant | `J R.initial` | `Initial` |
   | `equivalent_to` (`"reference":F`, a definition or model of the realization's signature) | — | `P x -> R x = F x` (stateful: over `s` and `x` with `J s`) | `Equivalent...` |
   | `dataset_agreement` (`"dataset":D,"comparison":c,"comparison_sound":T',"examples":n,"agreements":k`), stateless | — | `length(D) = n` and the count of examples `(a, b)` of `D` with `c (R a) b = true` is `k` | `Agreement` |

   For `dataset_agreement`, `D : list (product I O)` is a prior nullary
   definition, `c : (O, O) -> bool`, `k <= n`, and `T'` states exactly
   `c e o = true -> e = o`, so an agreement is an equality. Each claim's
   theorem states exactly its generated statement. An unknown claim kind is
   a schema failure (`LLT4001`); a claim whose contract lacks what it
   requires (a vacuous claim), an inexact, later, foreign, or non-theorem
   reference, or unsorted or duplicate claims is `LLT4009`. Evidence
   elaborates to no runtime declaration. Nothing is ever asserted without a
   theorem: a claim is *discharged* in linking and *verified* only by a
   verified attestation (rule 13).
6. **Models** (`LLT4006`, `LLT4007`, `LLT4009`). `{"kind":"model","name":M,
   "type_parameters":[...],"contract":Cuse,"realization":Ruse,
   "evidence":[Euse...]?,"entry":[T...]?}`. The realization's input,
   output, and state types equal the contract's at their type arguments
   (`LLT4006`); each listed evidence, listed once, is about exactly this
   contract and realization use (`LLT4009`). `entry` lists one theorem per
   *effective precondition* `e` of the realization (none for the
   noncomposite kinds; rule 7 for composites), in order, each stating
   exactly `P x -> e` (or `e` when the contract has no precondition), and
   for a stateful realization `J s -> P s x -> e` over its state and input
   (absent premises dropped); an omission or an inexact theorem is
   `LLT4007`. A claim discharges only the check it states: a dataset
   agreement or an equivalence discharges none, and satisfying the contract
   does not preserve its invariant. A model elaborates to the
   ordinary definition `M`, applying the realization, and `M.initial` when
   stateful. Its *required checks* are: `input_invariant` when the contract
   has an invariant; `output_invariant` when it has an invariant and no
   evidence discharges `preserves_invariant`; `postcondition` when it has a
   postcondition and no evidence discharges `satisfies_contract`; and
   `precondition` when it has a precondition.
7. **Composition** (`LLT4007`). A composite stage is a prior model, stateless
   or stateful. Types match exactly. A stage's *checks after it runs* are the
   output invariant and the postcondition among its required checks (rule 6);
   each needs a sound validator, and they run right after the stage in that
   order, a refusal returning `ContractViolation.output_invariant` or
   `.postcondition`. So a stage whose only evidence is, for example, a dataset
   agreement composes, its postcondition checked at run time. A composite with
   any check at run time (a `checked` junction or a stage's checks after it
   runs) has the output `result O contract_violation`, `O` its output without
   checks.

   *State.* A sequence, fan-out, product, or branch with stateful stages
   declares a state whose type is the right-nested product of those stages'
   states in stage order (`left` before `right`, `then` before `else`), with
   any initial value of that type; a composite without stateful stages
   declares none (`LLT4007` otherwise). Each stateful stage reads its own
   component and its step replaces exactly that component: a sequence and a
   fan-out or product replace the component of every stateful stage they run,
   and a branch replaces only the component of the arm taken, carrying the
   other unchanged. A refusal returns the state the composite was given, with
   the violation. A stage's state component is the one the composite was
   given, so its invariant is established at the composite's entry unless a
   checked junction validates it at run time.

   The closed forms:
   - `sequence` `{"stages":[...],"junctions":[...]}`: at least two stages,
     one junction per stage after the first, each stage consuming the
     previous stage's output. A junction before stage `i+1` is
     `unconditional` (the stage has no precondition), `proved`
     `{"evidence":T}` (the stage has a precondition), or `checked` (the
     stage has a precondition or is stateful with an invariant, and has a
     validator for each; the invariant validator, on its component, and the
     precondition validator run before it, a refusal returning
     `.input_invariant` or `.precondition`); a `proved` junction before a
     stage without a precondition, or a `checked` one before a stage with
     nothing to check, is `LLT4007`. `T` is over the
     composite's state `s` (when stateful) and the earlier stage's input `y`,
     and states exactly that the earlier stage's invariant of its component
     and its precondition imply the later stage's precondition of its
     component and the earlier stage's output: `J_i(s_i) -> P_i(s_i, y) ->
     P_{i+1}(s_{i+1}, out)`, each absent premise dropped and each stateless
     stage taking no component, where `out` is `M_i(y)`, or
     `second(M_i(s_i, y))` for a stateful stage. The effective preconditions
     are the first stage's invariant and precondition and the invariant of
     every later stateful stage whose junction is not checked.
   - `fanout` `{"left":Ml,"right":Mr}`: one input, output
     `product O_l O_r`; effective preconditions each stage's invariant and
     precondition (`P_l x` and `P_r x` for stateless stages; `J_l(s_l)`,
     `P_l(s_l, x)`, `J_r(s_r)`, and `P_r(s_r, x)` for stateful ones, each
     absent predicate dropped).
   - `product` `{"left":Ml,"right":Mr}`: input `product I_l I_r`, output
     `product O_l O_r`; effective preconditions each stage's invariant and
     precondition (`P_l (first x)` and `P_r (second x)` for stateless
     stages; `J_l(s_l)`, `P_l(s_l, first x)`, `J_r(s_r)`, and
     `P_r(s_r, second x)` for stateful ones, each absent predicate dropped).
   - `branch` `{"guard":g,"then":Mt,"else":Me}`: `g : bool` (another type is
     `LLT4007`), both arms of the composite's interface; effective
     preconditions `g = true -> R` for each invariant and precondition `R`
     of the then stage and `g = false -> R` for the else stage's.
   - `scan` `{"stage":Ms,"junction":j}` over one stateful stage: input
     `list I`, threading the stage's state from `Ms.initial` and collecting
     outputs in order; it declares no state of its own. The junction is
     `unconditional` (no precondition, no checks after the stage runs, and
     any invariant carried by `initial_invariant` and `preserves_invariant`
     evidence), `proved` (that evidence, no checks after the stage, and `T`
     stating exactly `J s -> P s x`; the stage has a precondition), or
     `checked` (the stage has an invariant, a precondition, or a check after
     it runs, with a validator for each; the invariant and precondition
     validators run before each step and the checks after it after each
     step; the first refusal is returned; the output is
     `result (list O) contract_violation`).

   Any other composition, junction, or stage is `LLT4007`. A composite's
   denotation is the ordinary typed composition of its stages' model
   functions and their validators.
8. **Runtime boundary** (`LLT4008`). `checked_apply` names a model, its type
   arguments, its arguments (`x`, or `s, x` when stateful), and `checks`, a
   strictly sorted subset of `input_invariant`, `output_invariant`,
   `postcondition`, and `precondition`, each of which names a predicate the
   contract has and validates with a sound validator. Its type is
   `result O contract_violation`, or `result (product S O)
   contract_violation` when stateful. It elaborates to ordinary code that
   binds the arguments once and runs the listed checks in the fixed order
   input invariant, precondition, the model, output invariant,
   postcondition, returning `Result.error` of the violated predicate's
   constructor at the first refusal and `Result.ok` of the model's result
   otherwise; generated binders are spelled `__checked<n>_...`, a spelling
   no source name can take. In executable code (an executable definition
   or instance field, or a descriptor term of an executable realization), a
   `checked_apply` lists every required check of its model; a plain `call`
   or `function_ref` of a model is allowed only when the model has no
   required checks; and a realization function is never applied directly.
   Anything else is `LLT4008`. Formal code may apply any model unchecked. A
   check is therefore erased exactly when a theorem discharges it, and a
   proof-only predicate is never presented as runtime enforcement.
9. **The `contract_violation` type.** Its four nullary constructors are
   ordinary data, matched by `match`; they lower to the pairs of `Prod Bool
   Bool` `(false, false)` (precondition), `(false, true)` (input invariant),
   `(true, false)` (postcondition), and `(true, true)` (output invariant),
   so values cross modules unchanged, and the canonical document names the
   type `ContractViolation`.
10. **Identity and accounting.** Model declarations are linked IR, so every
    digest, length, schema, role, type, descriptor, claim, junction, entry,
    and check is part of the semantic ID (§21.4); artifact bytes are covered
    through their digests, and their configured digests through the source
    ID. Every source node and every elaborated declaration and obligation
    is charged to `max_ir_nodes`.
11. **Elaboration in Lean.** Each elaborated definition and theorem lowers as
    any language-1.2 declaration does; a proposition-valued language-1.2
    definition is `@[expose, reducible]`, so a predicate over a finite
    domain is decidable exactly when its body is. A long list literal in an
    elaboration is a balanced `append` of literals of at most 32 elements.
    Declarations a model declaration generates refer to their own module's
    members by qualified name.
12. **Cross-checks and the fixed model semantics.** A module with a model
    declaration emits the fixed `LexLeanModels` runtime in its namespace:
    `Sound1`, `Sound2`, `Sound4`, `Complete1`, `Complete2`, `Complete4`, the
    `Satisfies`, `Preserves`, `Initial`, `Equivalent`, and `Agreement`
    helpers, and the decoders `inRange`, `encodeInt`, `tensorMatches`,
    `utf8Char`, and `linesMatch`, each an exposed total definition with no
    comment tokens. Every validator link and claim is restated against this
    fixed text by a generated theorem (`C.<predicate>_sound`,
    `C.<predicate>_complete`, `E.<theorem>`) whose proof is exactly the
    user's theorem, or `decide` for `dataset_agreement`; every artifact has
    its `W.decoded` theorem. LexLean's statement generator is thus checked
    by pinned Lean: had it generated a statement not definitionally equal to
    the fixed helper's, Lean would refuse the restatement. `examples/models`
    produces, and verifies, every helper and every check branch, and
    `MD-12` fails if a helper of the emitted runtime is produced by none of
    its declarations. The axiom audit covers every generated declaration: each
    is allowed its owner's axioms, and their union must equal the owner's
    exact policy (§22.6).
13. **Document, snapshot, and attestation.** The canonical document renders
    each model declaration as a closed catalog of facts (interface,
    descriptor, schema and digest but never the bytes, predicates,
    validators, claims, junctions, entry theorems, required checks), each
    generated obligation with the theorem that states it, and each
    elaborated definition, an artifact's only by name and type (its bytes
    and decoded value are never document text, whatever their size); a
    claim reads "discharged by" its theorem and no
    document text says "verified". A language-1.2 snapshot declaration
    carries its `elaboration` (declarations, the alpha identity of each
    elaborated definition, obligations, cross-checks, and required checks) when it is a model declaration or applies a model
    through `checked_apply`. Only a verified attestation records the
    generated declarations, each with its policy and result, as verified.
14. **Production.** Artifacts, realizations, models, validators, and
    checked applications are realized through their elaborations; contracts
    and evidence are erased (§17.13). Lean extraction of a root sees only
    elaborated definitions.

`examples/models` is the committed instance: a stateful deterministic
session, a rule policy, a statistical triage and an artifact-backed neural
recognizer, every composite form, checked boundaries at production roots,
and every claim kind, all verified by pinned Lean.

Routing is fixed by the project language and is never inferred from module
content, so no byte sequence has two meanings:

| Project language | `semanticdata` schema | Snapshot envelope | Lock schema | Builtin packages |
|---|---|---|---|---|
| `1.0` | rejected (`LLP2003`) | `lexlean/semantic-snapshot/1` | `lexlean/lock/1` | `1.0.0` |
| `1.1` | exactly `lexlean/semantic-module/1` | `lexlean/semantic-snapshot/1` | `lexlean/lock/1` | `1.1.0` |
| `1.2` | exactly `lexlean/semantic-module/2` | `lexlean/semantic-snapshot/2` | `lexlean/lock/2` | `1.2.0` |

`schemas/semantic-module-v2.schema.json` is the language-1.1 module schema
with the `/2` discriminator and the 1.2-only constructs added;
`schemas/semantic-snapshot-v2.schema.json` is the snapshot schema with the
`/2` envelope, `language` fixed to `1.2`, the same embedded module
definitions, and the closed `elaboration` member of a declaration (rule 13
under *Models, contracts, realizations, and evidence*). An elaboration is
described by its own `elaboration_*` definitions, identical to the source
definitions except that their names also admit the reserved spelling with two
leading underscores that only generated binders use; the source definitions,
and so every `linked_ir`, keep rejecting it, as linking does.
`schemas/lock-v2.schema.json` has the lock shape of
`schemas/lock.schema.json` with `spec` fixed to `lexlean/lock/2` and
`language` fixed to `1.2`.

The project, lock, lexicon-manifest, and build-manifest schemas of §7 pin
`language` to `1.0`, and they are hashed into the frozen 1.0 and 1.1
identities, so the later languages have their own schemas in the 1.2-only
partition rather than edits to those files. A language-`1.1` or `1.2`
`lexlean.toml` validates against `schemas/project-v2.schema.json`, its lexicon
manifests against `schemas/lexicon-v2.schema.json`, and its build manifest
against `schemas/build-manifest-v2.schema.json`; a language-1.1 lock validates
against `schemas/lock-1.1.schema.json` (`lexlean/lock/1`, `language` fixed to
`1.1`) and a language-1.2 lock against `schemas/lock-v2.schema.json`.

Compatibility rules:

- A language-1.1 semantic module containing any 1.2-only construct is
  rejected in linking (`LLT4001`) and names the construct.
- A semantic module whose discriminator differs from the one its project
  language selects is rejected in linking (`LLT4001`); a `/1` module in a 1.2
  project and a `/2` module in a 1.1 project are both rejected.
- A lock whose `spec` does not match its `language` row is rejected
  (`LLC0103`); a lock whose language, compiler-semantics ID, or package
  closure differs from the project is stale (`LLC0102`).
- A lexicon package whose `language` differs from the project language is
  rejected at lock time (`LLC0103`); a builtin glossary reference with the
  version of another language is rejected against the lock (`LLR3001`).
- An unsupported or malformed project language (for example `1.3`, `1.2.0`,
  or `01.2`) is rejected at configuration time (`LLC0103`).

All of these failures occur before Lean or LaTeX generation and before any
child process starts.

Migration is explicit and versioned. A language-1.1 project becomes a
language-1.2 project only by changing its `language` row, rewriting each
`semanticdata` discriminator from `/1` to `/2`, changing builtin glossary
references from `1.1.0` to `1.2.0`, and regenerating the lock with
`lexlean lock`. Every language-1.1 value is a valid language-1.2 value with
the same meaning, so the migration changes identities (the compiler-semantics
ID, the lock, and every source and semantic ID) but not generated Lean
declarations; the generated text differs only in the exposure attributes of
the portable runtime (rule 6 under *Collections and state threading*) and the
`reducible` attribute of proposition-valued definitions (rule 11 under
*Models, contracts, realizations, and evidence*), which change no
definition, type, or compiled code. No command migrates a project
implicitly: building an
unmigrated project under a newer compiler keeps its declared language and its
historical identities and artifact bytes.

### 17.13 Production eligibility

Being valid LexLean never implies being executable. A language-1.2 definition
becomes a *production root* only by declaring it:

```json
"production": {"effects": ["overflow"], "targets": ["rust-core", "rust-std"]}
```

`targets` is non-empty and `targets` and `effects` are each strictly ascending
and drawn from the closed registry `language/production-1.2.toml`
(`lexlean/production/1`). A malformed declaration is `LLT4001`. A module that
declares no production root is never analysed for production, so theorems,
propositions, and non-executable definitions remain ordinary formal content
and coexist with executable roots in the same module.

**Registry.** The registry fixes three closed sets:

- *targets*, each a machine profile with `allocation` (whether heap
  allocation exists) and the bit widths of the natural-number and
  mathematical-integer representations. Language 1.2 registers `rust-core`
  (no allocation, 64-bit widths) and `rust-std` (allocation, 64-bit widths);
- *effects*: `allocation` (heap storage whose exhaustion stops the
  realization with an explicit failure), `overflow` (a `Nat` or `Int` value
  outside the target width stops the realization with an explicit failure,
  never wrapping or truncating), and `recursion` (a stack depth bounded only
  by the input);
- *constructs*: one row per semantic construct kind (`type.<kind>`,
  `term.<kind>`, `primitive.<operation>`, `declaration.<kind>`, the
  refinements `declaration.inductive.recursive` and
  `declaration.definition.recursive`, and `constructor.nat_succ`) with its
  `disposition` (`runtime`, `formal-only`, or `erased`), whether it requires
  `allocation`, the representations (`nat`, `int`) for which its result may
  `overflow`, and whether it implies `recursion`. A construct requires
  allocation when its values live on the heap (strings, byte strings, lists,
  maps, sets, a type that contains itself) or when it builds new heap
  storage; reading, comparing, measuring, or folding storage that already
  exists requires none, since the type of that storage already does. Each
  runtime row's `allocation` equals its realization's (§17.14).

The registry is language data: it is embedded and hashed into the
language-1.2 compiler-semantics ID, so changing a disposition changes
LexLean's identity. Every construct kind of `lexlean/semantic-module/2` has
exactly one row; a construct with no row is rejected rather than defaulted.

**Closure.** Eligibility is decided after linking and before any backend runs,
over the root's *runtime closure*: the root and, transitively across modules,
every definition it calls or references and every instance it resolves, each
instantiated at its type arguments (monomorphization). Every type the closure
realizes is visited through the declarations it names, under their
instantiation. Termination evidence theorems are *erased* dependencies:
recorded, never realized, and never walked. Proofs and theorem statements are
never part of a runtime closure.

Language-1.2 model declarations enter a closure only through their
elaborations (§17.12): a closure member generated by an artifact, a
realization (including its companions), or a model is reported with that
construct (`declaration.artifact`, `declaration.realization`, or
`declaration.model`, each `runtime`) and walked as the ordinary definition it
is; a definition whose source applies a model through `checked_apply` is
walked as its elaboration and also reports `term.checked_apply`; contracts
and evidence (`erased`) elaborate to no runtime definition, so no closure
reaches them, while a contract's validators are ordinary executable
definitions. The `contract_violation` type is `runtime` plain data, and
`less_than` a `runtime` key comparison.

**Rules.** A root is eligible for a target exactly when none of the following
holds; otherwise `check` fails with `LLT4005`, naming the root, the target, the
violation nearest the root, the closure member it occurs in, and the call path
that reaches it:

1. a root parameter or result holds, directly or inside the fields of a named
   type, a universe, a proposition, an uninstantiated type parameter, or a
   function — a root takes and returns first-order data only, so a closure can
   never escape it;
2. the closure reaches a construct whose disposition is `formal-only` or
   `erased`, a declaration that is not a definition, a definition that is not
   declared `executable`, or a dependency that does not resolve. Linking
   already refuses a call to a theorem, an unresolved reference, and a
   non-executable callee of an executable definition (§17.12), so from linked
   source only the root itself can be non-executable; the analysis checks
   every case regardless, and its unit tests exercise the others on IR that
   bypasses linking;
3. a natural-number or mathematical-integer literal in the closure lies
   outside the target width;
4. the closure requires allocation and the target provides none;
5. the closure realizes an effect the root does not admit. The realized
   effects are `allocation` when the closure requires allocation on a target
   that provides it, `overflow` for every construct whose registry row names
   the representation of its result, and `recursion` for every recursive
   definition.

Two realization obligations follow from these rules and bind every later
production stage. A `Nat` or `Int` value crossing a root boundary is
represented in the target width, so a caller cannot supply a value outside it,
and every value the realization computes outside it is the `overflow`
failure, never a wrapped or truncated value. Primitive operations are
realized without recursion; only recursive definitions of the closure carry
the `recursion` effect.

The analysis is a conservative over-approximation of effects: it may require a
root to admit an effect its realization never exhibits, and it never admits a
root whose realization could exhibit an effect, a construct, or a value
representation that the declaration does not cover. Production compilation
never approximates or erases computational meaning to admit a root.

**Report.** Every module with a production root has the build artifact
`production/<full-module-path>.eligibility.json`
(`lexlean/production-eligibility/1`, `schemas/production-eligibility.schema.json`),
recorded in the manifest with kind `production-eligibility`. Per root it
records the declared effects, the runtime closure in discovery order with each
member's construct, type arguments, and shortest call path, every realized type
with its construct, the erased dependencies, every construct with the closure
members using it, and, per target, each realized effect with every
`(construct, instance)` source and every natural-number or integer
representation crossing the root boundary (`parameter <name>` or `result`,
directly or inside a container or a named type's fields) with the width in
which the target realizes it, so the first realization obligation above is
stated per root. The report is a deterministic function of the
linked semantic modules and the registry. Verification checks every root's
runtime closure and erased dependencies against the closure Lean's own
compiler front end extracts (§22.10); any difference fails verification.

**Exhaustiveness.** The analysis maps every IR variant to its registry key by
an exhaustive match that names every variant and every field. Its source admits
no wildcard arm, rest pattern, `if let`, `while let`, `let`-`else`, or
`matches!`, and names every variant of `SemanticType`, `SemanticTerm`,
`SemanticPrimitive`, `SemanticDeclaration`, and `SemanticInteger`;
`cargo xtask validate-model` (audit-production) rejects any such default and
any unnamed variant, so a new construct cannot reach production without its
disposition.

### 17.14 Production realization calculus

Production compilation targets one closed calculus, defined before any
optimizer and independently of every renderer. The calculus is LexLean: the
language-1.2 project `compiler/` defines its syntax (`TargetSyntax`), its
denotation (`TargetSemantics`), encoders of LexLean collection values
(`TargetOracle`), and the statements about its fixtures (`TargetFixtures`).
Lean elaborates, replays, and axiom-audits these modules like any other
LexLean program, and `compiler/` passes the verify, golden, and
reproducibility gates of §28.6. No meaning is read from rendered Rust text.

**Programs.** A program (`lexlean/target-program/1`,
`schemas/target-program.schema.json`) is a list of algebraic data types, each
a non-empty list of constructors given by their field types, and a list of
monomorphic functions, each with parameter names, parameter types, a result
type, and a body. Types are `unit`, `bool`, `nat`, `int`, the fixed widths
`u8`..`u64` and `i8`..`i64`, `string`, `bytes`, `ordering`, `option`,
`result`, `list`, `pair`, an indexed ADT, and `fn` with at least one
parameter. Expressions are a literal, a local, `let`, `cond`, an exhaustive
`match` over constructor shapes, `build` of a shape, a direct `call`, a
`closure` of a function over a proper prefix of its parameters, `apply`, a
primitive `prim`, `first`, `second`, and `field` of a single-constructor ADT.
The primitives are natural and integer arithmetic and comparison, checked
fixed-width arithmetic, bitwise operations and shifts, scalar equality,
Boolean connectives, append, length, index, and slice, UTF-8 encoding and
decoding, byte comparison, exact split and join, decimal formatting and
parsing, fixed-width conversion, and `compare`, the total key order of
§17.12 (numeric, `false` before `true`, strings by scalar sequence, pairs
lexicographically) returning an `ordering`. Locals are natural numbers
scoped lexically; there is no mutable state, no reference, and no aliasing:
every value is an immutable tree, and a byte string is an immutable buffer.

**Static rules.** A program is valid exactly when every type names declared
ADTs, every local is bound, every `call`, `closure`, `apply`, `build`, and
primitive receives exactly the operand types of its signature, every match is
exhaustive and has no unreachable arm, every arm binds exactly its shape's
fields once, every function's body has its result type, and every literal has
its type and lies in its realization: `nat` in `[0, 2^64)`, `int` and the
fixed widths in their two's complement ranges, all spelled as canonical
decimals, bytes as lowercase hexadecimal. An ordering literal is admitted; a
closure literal is not. `lexlean::calculus::load` reads a program, checks
every rule, and returns its canonical form; any malformed member or violated
rule is `LLB6005`, and nothing is repaired or guessed.

**Canonical form and identity.** The canonical form of a valid program renames
every local of a function to its first-binding order (parameters, then
binders in evaluation order), so alpha-equivalent programs have
byte-identical canonical JSON (§21.7). A program's identity is the SHA-256 of
its canonical bytes. An invalid program has neither.

**Denotation.** `TargetSemantics.run fuel program entry arguments` binds the
entry's parameters and evaluates its body by `eval`, structural recursion on
`fuel`. Operands evaluate left to right; a call evaluates its operands, then
the callee's body in a fresh environment of its parameters; `apply` passes a
closure's captures before its operands. Every primitive is defined by
reference to the language-1.2 primitive of the same meaning, so a value of
the calculus means what LexLean says it means, and `compare` is LexLean's own
map key order. The result is an `Outcome`: a value with its exact step count,
`overflow` with its step count, `stuck`, or `exhausted`.

**Cost.** The step count charges every operation the evaluator performs, so
no runtime operation is free:

- each evaluated node is one step, on top of the steps of its operands;
- reading a local is one step more per binding the lookup examines before
  it finds the local's;
- a `match` is one step more per arm it tries, the taken arm included;
- `apply` is one step more per capture it passes;
- `field` is one step more per field it skips;
- a primitive is one step more per unit of *weight* of its operands and of
  its result. A value's weight is one per node, plus a string's characters
  and a byte string's bytes, so every primitive whose work grows with its
  operands (append, length, index, slice, encoding, decoding, comparison,
  split, join, formatting, parsing) is charged in proportion.

The charge is exact for the denotation: the reference interpreter charges
the same, and the kernel confirms each kernel-reducible fixture's step count.
It bounds the work of the Rust renderings below, each of which realizes every
step with work at most its charge. A `nat` or `int` result outside its
realization is `overflow`, never a wrapped or truncated value. The static
rules are designed so that a valid program is never `stuck`; the conformance
suite checks this on every fixture program at its stated arguments and at
seeded random well-typed arguments under sampled fuels, which is build
evidence, not a proof. `exhausted` reports only that the fuel bounded the
evaluation and is not an observation of the program.

**Realizations.** `lexlean::calculus::realization::TABLE` maps every construct
whose production disposition (§17.13) is `runtime` to the calculus elements
realizing it, and the conformance suite checks that the table and the
registry's runtime rows are in bijection and that every reference exists. A
map is its ascending entry list `List (Pair K V)`, a set its ascending element
list, and a graph its map from node to successor set, exactly the
representation of §17.12; every collection, fold, iteration, and graph
primitive is a library template (`lexlean::calculus::library`) written as a
transcription of the `LexLeanCollections` definition over `compare`,
instantiated monomorphically. Agreement with LexLean is checked pointwise:
each template's fixtures state, and the kernel decides, that its outcome
equals LexLean's own primitive on that input, which is evidence on those
inputs, not a proof for all inputs. The model constructs are realized by the
elements of their elaborations (§17.12): an artifact, a realization, and a
model are functions (a model's body a call), a checked application is a
sequence of `let`s, validator calls, and conditionals building a `Result`, the
`contract_violation` type is a pair of Booleans, and `less_than` is `compare`
matched to a Boolean. A construct requires allocation exactly
when its realization does: its values are strings, byte strings, lists, or a
type that holds itself behind a heap handle (`indirection`), or it builds
such a value; reading, comparing, measuring, or folding existing storage
allocates nothing. The conformance suite checks every runtime row's
`allocation` against its realization. Lean's `&&` and `||` do not evaluate
a decided right operand, so they are realized by `cond`. A type parameter is
realized by its closed type argument (§17.13).

**Fixtures.** The hand-constructed programs under `compiler/fixtures/`
(`lexlean/target-fixture/1`, `schemas/target-fixture.schema.json`) each name a
program, an entry, arguments, fuel, and the expected outcome computed by the
reference interpreter, a step-for-step transcription of `eval`. Together they
use every type, literal, expression, shape, primitive, and template, and
every fixed-width primitive at every width it admits. The generated module `TargetFixtures` (`cargo xtask check-calculus`
compares it and every fixture with its generator byte for byte) defines each
fixture's evaluation and states:

1. for every fixture whose primitives a `reflexivity` proof can reduce, that
   the evaluation reduces to the expected outcome, steps included, which
   Lean checks by `rfl`. Lean's elaborator checks such a proof by
   definitional unfolding before the kernel sees it, and from a module file
   under pinned Lean it does not unfold what `split_exact`, `parse_decimal`
   (`String.splitOn`, `String.toInt?`), `compare_bytes`
   (`ByteArray.toList`), `utf8_encode`, `join`, and `format_decimal` reach,
   which recurse well-foundedly or are not exposed. LexLean's proof
   language has no kernel-only decision, so fixtures applying them are
   decided by Lean's evaluator instead; byte equality is decided by the
   kernel;
2. for every library fixture, that the realization's outcome equals the
   value LexLean's own collection primitive computes on the same input,
   decided by the kernel.

Lean's evaluator, running the published generated sources compiled again by
the same pinned Lean (the published oleans carry no compiled code),
reproduces every fixture's outcome, steps included. A wrong expected outcome
or a mutated template is rejected by Lean, and an evaluator-only fixture
stated one step off is refused.

**Rust profiles.** The reference renderings (`lexlean::calculus::rust`)
realize the two machine profiles of §17.13, each as one safe Rust 2021
library crate under `#![forbid(unsafe_code)]` that declares no lint
exception:

- `rust-core` is `#![no_std]` and declares no `extern crate`, so neither
  `alloc` nor `std` is linked and nothing is allocated. It renders exactly
  the programs that need no heap: a program whose realization requires
  allocation (a string, a byte string, a list, or a type that holds itself)
  is refused with the reason, never approximated.
- `rust-std` uses the standard library. A string and a byte string are
  immutable shared buffers (`Rc<str>`, `Rc<[u8]>`), a list is a persistent
  list of shared cells, so building and taking apart a cell is constant
  work, and an ADT field or a closure capture whose type holds its owner is
  boxed behind `Rc`, and nothing else is.

In both, `nat` is `u64` and `int` is `i64`, each operation checked and every
out-of-range result the explicit `Err(Overflow)` propagated by `?`; the
fixed widths are the native integer types with `checked_*` operations;
orderings are `core::cmp::Ordering`; ADTs are enums; function values are
defunctionalized into one enum per function type; and using a local clones a
handle or a value of fixed size, never a structure, so ownership never
changes an observable value and no step does hidden work. Every loop of the
runtime counts its iterations and every bulk copy counts its length in a
work counter, and on every fixture with an observable outcome the count
never exceeds the denotation's steps; the release of a list, whose cells were each counted when built, is
not counted again. Calls are Rust calls with no tail-call or stack-depth
guarantee, evaluation is single-threaded and sequential, the functions form
no stable ABI, and heap exhaustion aborts the process. Observable behavior is
the printed value or `overflow`; steps, fuel, and the work count are not
observable. The claim covers exactly programs of this calculus rendered this
way: no other Rust construct (unsafe code, foreign functions, threads,
asynchronous code, floating point, interior mutability, input and output
beyond the harness) is within it. Every fixture whose entry takes and
returns first-order data is committed as a package in each profile that
admits it, under its lint gate (§17.16), and compared with the renderer by
`cargo xtask check-calculus`. Every fixture with an
observable outcome is rendered, compiled by the pinned `rustc` 1.97.1
(`RUSTC-1-97-1`, an authority this repository cites) with warnings denied,
linked with a harness, and run; its output must equal the denotation's, its
work count must not exceed the steps, and planted value and work
discrepancies must be detected. This is `build` evidence for the renderings,
not a proof of them; preservation of meaning from LexLean through the
calculus to Rust is a separate obligation.

### 17.15 GNAF requests over the calculus

A claim that a realization is optimal is meaningful only against a candidate
universe, a machine, and an order fixed before any optimizer runs. LexLean
fixes them by the request components of UOR-GNAF (`uor-gnaf/1-draft.2`,
authority `UOR-GNAF-1-DRAFT-2`, §27.4): the module `Gnaf` of `compiler/` is
the normative model, a LexLean definition Lean elaborates, replays, and
axiom-audits. It defines the request, its fail-closed validation, the
universe and the theorem that it is complete, and the answer the request's
universe has under the calculus denotation of §17.14. It imports only
`TargetSyntax` and `TargetSemantics`, and nothing in it calls, cites, or is
computed from an optimizer.

**Request.** A request (`lexlean/gnaf-request/1`,
`schemas/gnaf-request.schema.json`) names:

1. the problem: a reference program and a non-empty domain of arguments
   (UOR-GNAF §8.1), so that an empty domain never makes a claim vacuously
   true;
2. the machine contract X and accounting model M (§8.2, §9.1): fuel; the
   machine's hard capacity, the largest fuel, domain, universe, and declared
   charge it admits; the operand-size treatment, `weighted` (every primitive
   charged its operands' and result's weight, as §17.14 charges it) or
   `unit`; and for each admitted kind of action its charge. The kinds are
   observation, preprocessing, advice, retained state, dispatch, fallback,
   communication, randomness, scheduling, and execution, and a charge is
   `steps`, a constant per invocation, `free`, or `undeclared`. The
   comparison boundary (§10.8) is `complete`, `prepared_state`, or
   `prepared_plan`, and the prepared artifacts bound in the common initial
   state of every competitor each name the preparation action that produced
   it and the SHA-256 of its bytes;
3. the universe U_sys (§8.3): a carrier, its completeness evidence, and its
   `SystemUniverseId`. The only admitted carrier is a grammar of complete
   systems: an argument type, a result type, plans shared by every system,
   each with the preparation actions it needs before an invocation, and
   strictly increasing dispatch thresholds. Its members are exactly the fixed
   systems, one per plan, then for every threshold every dispatch from a
   small plan to a large plan, both ranging over every plan, in that order.
   A system is the complete selector-plus-executor (§8.5): it realizes as one
   program whose entry function (index 0) is its selector and whose plans
   sit at indices 1 onward. The only admitted completeness evidence is
   `grammar_equality`, which cites the kernel theorem `Gnaf.expandComplete`.
   The `SystemUniverseId` is the framed SHA-256 `lexlean-gnaf-universe-v1`
   of the problem, the machine, and the carrier in canonical JSON, computed
   from the request before any evaluation; a stated identity is only an
   equality check against it;
4. the objective and order (§9.2, §9.3): `scalar` is total cost over the
   domain, totally ordered, lower is better; `vector` is the pair of total
   cost and size, ordered componentwise with no weighting;
5. the claim class (§12.4), every class of the authority, with
   `profile_defined_comparison` carrying its profile, class, and result shape
   and `instance_optimal` its two constants, and the scope the claim ranges
   over: the grammar universe, all calculus programs, or all Rust programs.

**Validation.** `Gnaf.validate` returns the first violated rule, in this
order, and `Gnaf.answer` of a refused request is `rejected` with it:

1. an empty domain;
2. a carrier that is not a grammar: one system's internal plans taken as
   the universe (`internal_plan_universe`), an optimizer's output, the
   candidates a search discovered, or a cache;
3. completeness evidence that is missing, cites the universe's own
   identity, or cites an optimizer;
4. a request beyond the capacity its machine binds: more fuel, domain
   arguments, or systems than the capacity admits, or a declared charge
   above it (`beyond_capacity`);
5. the `unit` operand-size treatment, under which a primitive whose work
   grows with its operands is an unbounded action at unit cost
   (`unit_cost_operands`, §8.2);
6. an action kind accounted twice (`duplicate_action`);
7. an action some system performs (observation, dispatch, fallback,
   execution) that is unaccounted, or charged other than by `steps`: the
   calculus counts that work as steps, so a zero, constant, free, or
   undeclared charge hides or replaces it (`hidden_cost`);
8. an admitted preparation action (preprocessing, advice, retained state)
   charged by `steps` (the calculus has no preparation phase whose steps it
   counts), by zero, or `undeclared` (`hidden_cost`), or `free` other than
   on a prepared boundary whose common initial state binds an artifact it
   prepared (`unbound_preparation`, §10.8); and an admitted communication,
   randomness, or scheduling action, which the sequential deterministic
   calculus machine does not have, so no system of the grammar realizes it
   (`unrealizable_action`);
9. a prepared artifact of an action that is not admitted `free`, which
   would exclude work that is charged (`stray_prepared_artifact`);
10. a plan needing a preparation action the machine does not account
    (`unaccounted_action`);
11. `restricted_universe_optimal`, which §12.4 makes a legacy alias that a
    conforming answer never emits as a base class (`claim_alias`); a scalar
    claim (`global_optimal`, `argmin_complete`) over the vector order, a
    vector claim (`pareto_optimal`, `frontier_complete`) over the scalar
    order, or any other claim class, which this model does not decide
    (`unsupported_claim`);
12. a scope beyond the grammar universe. No coverage bridge from the grammar
    to all calculus programs or to all Rust programs is proved, so such a
    claim is refused (`uncovered_scope`) rather than inferred from the
    grammar's answer.

**Completeness.** `Gnaf.wellFormed` states the grammar's membership
semantics without the expansion: a fixed system runs a plan below the plan
count, and a dispatch runs two such plans at a declared threshold.
`Gnaf.expandComplete` proves, for every grammar and every selector, that the
selector is a member of `Gnaf.expand` exactly when it is well formed; Lean's
kernel checks the proof by induction over the expansion's construction. The
expansion and the membership semantics depend on nothing but the grammar.

**Answer.** For a valid request, every system is evaluated on every domain
argument with the request's fuel. The first argument on which the system
does not return the reference's value decides: overflow, a stuck
evaluation, or a different value makes the system inadmissible, and an
exhausted evaluation leaves it unresolved. A system that agrees everywhere
is admitted with its cost, its total steps plus, on every invocation, the
declared constant of each preparation each plan it can run needs, and its
size, one per expression node and match arm of every function reachable
from its entry. A plan's preparation is therefore charged exactly to the
systems that can run it, and code a system cannot run is not its size. An
unresolved system is never removed and never given an infinite cost
(§8.4): any unresolved system makes the answer `incomplete`. Otherwise the
answer is `infeasible` when nothing is admitted; for `scalar`, `argmin`, every
admitted system attaining the minimum cost and that minimum; for `vector`,
`frontier`, every admitted system no other strictly dominates. Both are
computed by the model's one generic order, `Gnaf.argmin` and
`Gnaf.frontier`, over rows of an identity and a cost; equal costs never
dominate each other, so every equal-cost system is kept (GNAF-REJ-21).
Because a dispatching system pays for its observation and selection, the
best internal plan and the per-input envelope of the plans are not complete
systems' costs, and the model never answers with them. `Gnaf.certifies`
accepts a claimed argmin or frontier exactly when it is the request's own,
as an identity set, and `Gnaf.minimaAttained` says whether some admitted
system attains the componentwise minima of the admitted costs.

**Host side.** `lexlean::gnaf` transcribes the model step for step: `load`
reads a request and checks what the model presupposes, reporting `LLB6006`
for a malformed request; a reference program, plan, or realized system that
is not valid under §17.14; a domain argument outside the reference entry's
parameter type or on which the reference returns no value under the
request's fuel; grammar argument and result types other than the entry's;
repeated thresholds; a plan preparing by an action that is not preparation,
or naming one twice; and a stated universe identity other than the one the
request's components determine. `answer` evaluates it; and request, status,
and answer terms are emitted so the kernel confirms each committed request.
The host evaluator's capacity is part of the machine it offers: a machine
asking for, or a request using, more than fuel 4096, 1024 domain
arguments, 65,536 systems, or charges of 2^32 is `LLS8002`. Within that
capacity every declared charge fits the host's integers, so a step total
the host cannot represent is `LLS8002` as well, never a cost the model would
not report. Evaluation runs on a stack sized for the request's fuel, and a
panic of the evaluation thread is `LLI9001`.

**Fixtures.** The requests under `compiler/gnaf/` (`lexlean/gnaf-fixture/1`,
`schemas/gnaf-fixture.schema.json`) each name a request and its expected
answer. They include a Pareto frontier of incomparable systems; ties from a
duplicated plan, in an argmin and in a frontier; GNAF-VEC-02 posed over the
calculus, four systems of which exactly three form the frontier; a
dispatching system that is the argmin although the best single plan is not
and although the per-input envelope of the plans is lower than any system
attains; an inadmissible plan excluded; an unknown cost making the answer
incomplete; a plan's preparation moving the argmin; prepared artifacts
bound in the common initial state; and a refused request for every rule
above. The generated module `GnafFixtures` (compared with its generator by
`cargo xtask check-calculus`) states for each request that `Gnaf.universeOf`
reduces to the host's expansion and `Gnaf.answer` to the expected answer,
for each answered request that `Gnaf.statusesOf` reduces to the host's
statuses, and for the posed GNAF-VEC-02 that no frontier with a member
omitted is certified while the frontier in another order is, and that no
system attains the componentwise minima; each is decided by Lean's kernel.
A wrong answer, an answer computed from a universe with a system omitted, a
tie with a member dropped, and a universe with a system omitted are
rejected by Lean. The GNAF schemas are generated with the fixtures, and
their calculus definitions are `schemas/target-program.schema.json`'s own.

**Authority vectors, dependency manifest, and honesty.** The authority's
normative fixtures GNAF-VEC-01 (the compositions of its operations, with
their path identities, and the S0 certificate invalid for S1), GNAF-VEC-02,
GNAF-VEC-04, GNAF-VEC-17 (the pointwise envelope, the uniform worst case,
and the fair randomized expectation), GNAF-REJ-14, and GNAF-REJ-29 (§16) are
theorems of `Gnaf` over the same `argmin`, `frontier`, and certification the
answers are computed by, decided by the kernel. `compiler/gnaf.manifest.json`
is the dependency manifest §20 requires: the draft's revision and SHA-256,
every kind, operation, machine, cost, proof, interchange, and address profile
with its role, every restriction of the admitted universe, and the claim
classes the model actually answers. The authority's requirements are
`some-true` here: reproduced from the cited document, a copy of which is
vendored at `model/authorities/UOR-GNAF-v1-draft.2.md` and whose SHA-256
`cargo xtask validate-model` recomputes. The model and its fixtures are
`build` evidence that LexLean realizes them, not a proof of the authority.
UOR-NAF is informative to UOR-GNAF (§20) and is neither cited as evidence
nor relied upon. A request's answer is a statement about the grammar
universe under the calculus's step accounting; it is not a claim about
machine time, about calculus programs outside the grammar, or about Rust
programs.

### 17.16 Rust backend

The Rust backend renders target programs (§17.14) to Rust without making Rust
text the source of any meaning: a program is lowered to a closed Rust AST,
the AST is checked, and only then printed. `lexlean::calculus::rust` is the
backend; `lexlean::calculus::package` is its entry point for packages.

**AST.** The AST (`rust::ast`) has no node for arbitrary text. Every
identifier is generated from a closed kind and an index (`v<n>` for a local
of the program, `m<n>` for a match scrutinee or a bound condition, `a<n>` for
an operand, `c<n>` for an applied closure, `r<n>` and `h<n>` for a value
taken out of a box or a pair, `k<n>` and `p<n>` for a closure's captures and
parameters, `f<n>` for a program function) or is an exported name the package
manifest declares. Every called function is a program function or a runtime
item of the closed table `rust::runtime::Item`; every type is a calculus type
or one of the representation types `Rc<T>` (a field or capture whose type
holds its owner), `R<T>` (a fallible result), and `&T` (a borrowed export
parameter). Every construct carries its *origin*: the calculus element
(§17.14) of the term, shape, or literal it was lowered from, or the
structural realization it is, and the width of a fixed-width element. The
constructs are every item, every binding, and every expression other than a
block, a list's `uncons`, and a predecessor, which are parts of the
construct around them; a pattern is checked through the match that carries
it.
The printer lays out every item, block, and arm in one fixed way, so the
rendering's bytes are canonical without a formatter. A string literal is
written by LexLean's own escaper: printable ASCII other than `"` and `\` as
itself, those two escaped, and every other scalar as `\u{..}`; a byte string
is an array of `0x..` bytes. No byte of a rendering depends on the Unicode
tables of the toolchain that built LexLean.

**Lowering.** Each calculus construct has one rendering, and the shapes Rust
writes differently are fixed:

- a unit carries nothing: a unit binder is `_`, a unit read is the literal
  `()`, a unit result type and a unit block value are left implicit, an
  operand that computes a unit is bound (`let () = e;`) in operand order
  before the call that receives `()`, and a fallible function or dispatch
  returning a computed unit binds it before `Ok(())`;
- a conditional or match whose branches are the Boolean literals is its
  condition or that condition's negation (a zero test negated is the
  nonzero test); a conditional whose branches are the same code up to the
  names they bind is its condition, evaluated and discarded, then that code;
  and a match each of whose arms rebuilds exactly what it matched is its
  scrutinee;
- a conditional on a Boolean literal and an `if` without `else` holding only
  another bind their (inner) condition first; an empty unit `else` is left
  implicit; a block that binds a value and returns that binding is the
  value; a field of a computed record binds the record first;
- a type no value inhabits (a function type no closure of a live function
  inhabits, or a named type, pair, or result built only from such types):
  a function with a parameter of such a type is never called and its body is
  the empty match on that parameter; a match arm whose shape holds such a
  value is never taken and is omitted, as Rust's exhaustiveness admits; and
  any other computation of such a value is refused, as `LLB6005`
  (*unsupported type*), because Rust would make the code after it
  unreachable.

Each of these renders the same denotation with no more work than the plain
rendering.

**Failure typing.** A function can overflow exactly when its body reaches a
primitive or successor that can (`nat_add`, `nat_mul`, `int_add`, `int_sub`,
`int_mul`, `int_neg`, `int_quot`, `parse_decimal` at `int`, `succ`), a call to
a function that can, or an application of a function type one of whose
closures can: the least fixed point of these rules, computed by a worklist
over the reverse call and application edges. A function that can overflow
returns `R<T>` and every call to it ends in `?`, except in tail position,
where its result is returned as it is; any other function returns its value
and no call to it propagates. The length of a sequence cannot exceed
`2^64 - 1` on any machine that holds it, so `length` cannot overflow.

**Checks.** Before printing, `rust::validate` refuses, as `LLB6005`:

1. *hygiene*: a name used but not bound, or bound twice in one function,
   and an item declared twice;
2. *ownership*: a value moved twice, or read after it was moved, on any
   path; branches are alternatives;
3. *hidden allocation*: in `rust-core`, any heap type, heap runtime
   function, string or byte string literal, list construction or match, or
   box;
4. *arithmetic*: a fallible call that does not propagate outside tail
   position, an infallible call that propagates, a fallible function's tail
   that is not a fallible value, and a function that propagates a failure
   its result type does not carry;
5. *correspondence*, instance by instance: an emitted construct with no row
   in `validate::CORRESPONDENCE`, whose row does not name the element of its
   own origin, whose width (a fixed-width runtime function's or literal's)
   differs from its origin's, or whose origin the program does not use; and
   a type the crate names whose row names no element the program uses. The
   origin is derived from the term being lowered, independently of the
   construct chosen for it, so a call of `nat_mul` lowered from a `nat_add`
   term, or a `u16` addition lowered from a `u8` one, is refused even in a
   program that uses both. Types are checked against the program's elements
   as a whole. That the parts of a construct are composed faithfully (which
   arm is which, which function a call names, the order of captures and
   fields) is not a check of the AST: it is established by the conformance
   differential below, against which planted lowering mutations are
   recorded. A negated zero test or predecessor is refused as well: a zero
   test is negated as its complement, and the printer parenthesizes any
   comparison under `!`.

**Packages.** A package manifest (`lexlean/rust-package/1`,
`schemas/rust-package.schema.json`) names a crate and its version (three
canonical decimals, each at most `2^64 - 1`, as Cargo reads them; the schema
pattern and the renderer agree), a profile, a target program, the exported
functions, and its `sources`: the semantic IDs (§21.4) of the LexLean builds
whose linked IR states the program, as lowercase SHA-256 hex, strictly
ascending, and empty for a program no LexLean build states. The renderer
checks their form and records them; that they name the build stating the
program is established by the conformance suite against the verified build
(below). That the program faithfully realizes the semantic objects of that
build (realization-to-source preservation) is the obligation of the
production pipeline that derives target programs from LexLean roots, not of
this backend: `sources` binds a package to the build that states its program,
and the backend never derives a program from a build. Each export names a
program function, its Rust name, how it takes each parameter (`own`;
`borrow`, by shared reference; or `copy`, only for a `Copy` type: a scalar,
or an option, result, or pair of `Copy` types), and whether it can fail
(`none` or `overflow`). A package is refused, as `LLB6005`, when:

- an exported name is not a lowercase snake-case identifier of at most 64
  characters, is a Rust keyword, has the shape of a generated name, or is
  declared by the runtime; a name is exported twice; or the crate name is a
  sysroot crate or `harness` (*identifier collision*);
- an export copies a parameter whose type is not `Copy` (*ownership
  mismatch*);
- an export's boundary holds a function value, directly or inside the fields
  of a named type at any depth: the boundary is first-order (§17.13)
  (*unsupported type*);
- an export's declared errors differ from its function's (*arithmetic
  mismatch*);
- the profile is `rust-core` and the program needs heap allocation (*hidden
  allocation*); or
- the manifest is malformed, its version not canonical, its sources not
  canonical, its program invalid or without a faithful rendering, or an
  export's arity wrong.

A package is three files. `src/lib.rs` is the rendering with one public
wrapper per export. `Cargo.toml` declares the crate, edition 2021, no
dependencies, and its lint gate: `unsafe_code` forbidden, rustc warnings
denied, and every Clippy lint of the default set denied except the ten of
`package::ALLOWED_LINTS`, each because its advice would make the rendering
depart from the calculus:

| Lint | Clippy group | Its advice, and why the rendering does not follow it |
| --- | --- | --- |
| `type_complexity` | complexity | a type alias: a generated type is exactly its calculus type, and an alias is a name with no calculus counterpart |
| `too_many_arguments` | complexity | a struct of parameters: a function takes exactly its calculus parameters, and the struct is a type with no calculus counterpart |
| `large_enum_variant` | perf | a boxed variant: boxing changes the representation and allocates, which `rust-core` cannot and the calculus does not |
| `result_large_err` | perf | a boxed error: likewise |
| `result_unit_err` | style | an error type in place of unit: a function returns exactly its calculus result type |
| `single_match` | style | `if let` or `if` in place of a match with an empty arm: every calculus match states an arm for every shape |
| `manual_map` | style | `Option::map` and a Rust closure: a calculus closure is defunctionalized, never a Rust closure |
| `manual_unwrap_or`, `manual_ok_err` | complexity | a library combinator in place of the program's own match: the combinator is outside the closed runtime |
| `manual_unwrap_or_default` | suspicious | likewise; its advice is a rewrite that preserves the match's meaning, so allowing it masks no defect |

No allowed lint is in Clippy's `correctness` group.

Every committed package passes this gate, and the fixtures exercise every
shape of **Lowering** and every exception above: removing any one rule or
exception makes a committed package fail the gate. `provenance.json` (`lexlean/rust-provenance/1`,
`schemas/rust-provenance.schema.json`) binds the crate's name and version, the
profile, the program's identity, the language-1.2 compiler-semantics ID
(§21.2), the SHA-256 of the runtime the profile carries, the sources, the
exports, and the SHA-256 of `Cargo.toml` and `src/lib.rs`.
`language/semantics-1.2.toml` records the SHA-256 of each profile's runtime
(`rust_runtime_core`, `rust_runtime_std`) and the digest of the renderer's
sources (`rust_renderer`: one §21.1 frame per file of
`rust::RENDERER_FILES`, the six files of `calculus/rust/`, labeled by its
name, under the domain `lexlean-rust-renderer-v1`). Unit tests of the runtime
and the renderer and the conformance suite check both records, so neither
the runtime nor any rendering can change without changing LexLean's
compiler-semantics ID, and two different renderings never claim one
compiler identity. `rust_backend` is a version label for humans that no gate
checks.

**Harnesses.** The harnesses that call a package's exports and print their
outcomes are test text, not renderings: they live in the conformance crate
(`repo_conformance::rust_harness`), outside the shipped crate, and reach the
renderer only through `rust::Literals`, which builds each argument literal
through the closed AST. They write only library and function names that are
plain Rust identifiers.

**Evidence.** Every fixture whose entry takes and returns first-order data is
committed as a package under `compiler/rust/<target>/<fixture>/` in each
profile that admits it, exporting its entry as `run`, with two more that pass
parameters by reference and by copy; every committed package's `sources` is
the semantic ID of the `compiler` project's build, whose `TargetFixtures`
module states each fixture's program. The fixtures include a dispatch of
several closures of mixed failure, captures read by copy, clone, box, and as
a unit, a recursive closure, a closure applied where it is built, a function
type no closure inhabits, a boxed field read, a string that needs escaping,
and each shape of **Lowering**. The negative manifests and their stated
errors are committed under `compiler/rust/negative/`. `cargo xtask
check-calculus` compares all of them with the generator. The conformance
suite builds every committed package offline in one workspace under its lint
gate; calls each export whose fixture has an observable outcome from a
separate crate and compares the printed outcome with the denotation's; runs
a differential of every primitive instance each profile admits on its
boundary values and seeded inputs; checks that the packages it runs emit
every correspondence row and every node of the AST; renders every
package in two separate processes with different working directories and
environments and compares the bytes with each other and with the committed
packages; validates every manifest and provenance against its schema; and
checks every package's `sources` against the published, verified build: the
semantic ID of `compiler/expected/build/manifest.json`, named by the
committed verification records, whose `TargetFixtures` source map binds the
committed module source, which states the package's program as the
declaration `<fixture>Program`. The binding is to the whole build, the
smallest object LexLean gives a semantic ID; the declaration is found by the
fixture's name, which provenance does not record. That the build verifies is
established by `cargo xtask verify-examples` and `cargo xtask check-golden`,
which reverify and compare those committed records; the suite checks only
that a record of the build is committed under its name.

## 18. Lean backend

### 18.1 Output contract

For each LexLean module, the backend emits one `.lean` file under a path matching the full generated module name.

The file structure is exactly:

```lean
module
import <sorted external and generated modules>
set_option autoImplicit false
namespace <full generated module>

<declarations in source order>

end <full generated module>
```

There are no comments or blank documentation blocks. The file ends in one LF.

For a language-1.1 `semanticmodule`, the fixed `set_option maxRecDepth 100000`
and `set_option maxHeartbeats 1000000000` lines from §17.11 follow
`set_option autoImplicit false`, before the namespace. This does not change
the language-1.0 prose-module layout.

### 18.2 Prose-free rule

Generated Lean MUST contain no:

- line comment;
- block comment;
- documentation comment;
- string literal, except canonical native-core data and the fixed generic
  decoder's keys and diagnostics in a module that linked a `coremodule`;
- character literal used as documentation;
- command whose purpose is textual output, except the separate axiom-audit modules;
- source-text copy;
- glossary description;
- `sorry`;
- `admit`;
- `axiom`;
- `opaque`;
- `unsafe`;
- `native_decide`;
- placeholder declaration.

A generated-source audit lexes every `.lean` file before verification. It
checks tokens, not substring guesses, so legal identifier fragments are not
misclassified. The native-core string exception is selected from the linked
module kind, never from source text, and the core schema forbids a backend
source field.

### 18.3 Names and imports

- all imports are explicit, deduplicated, and bytewise sorted;
- every external global is emitted by fully qualified Lean name;
- generated-module imports use their full generated names;
- no `open`, `open scoped`, namespace alias, or imported notation is emitted;
- imported notation is not relied upon for semantic lowering;
- source order is retained for declarations;
- inherited section parameters are emitted explicitly on each declaration that uses them;
- unused section parameters are not added to a declaration.

### 18.4 Term lowering

The backend favors explicit core applications over presentation notation:

- equality lowers to `Eq`;
- conjunction to `And`;
- disjunction to `Or`;
- negation to `Not`;
- existence to `Exists`;
- unique existence to `ExistsUnique`;
- equivalence to `Iff`;
- external functions use fully qualified constants;
- binders preserve explicit, implicit, and instance modes from linked signatures;
- numerals use Lean numeral syntax with an expected type.

The renderer MAY use parentheses to make every application and binder unambiguous. Output formatting is fixed at two spaces per tactic or branch indentation and never depends on a pretty-printer version.

### 18.5 Leading universal binders

Leading universal binders in a theorem statement are lambda-lifted into Lean declaration parameters in source order. They are already in scope when the proof begins. A proof-level `Assume` therefore introduces only binders remaining in the current goal after declaration parameters.

The IR retains the original proposition, and the source map relates both the parameter and body ranges to the quantified source clause.

### 18.6 Definitions

Type, term, and predicate definitions emit `def`, never `abbrev`, `opaque`, or `theorem`.

A document definition is nonrecursive. Its generated type is explicit. The generated value contains no compiler-invented assumption.

### 18.7 Proof lowering

Proof IR lowers only to these pinned Lean forms:

```text
intro
exact
apply
rfl
refine
constructor
left
right
have
rw
simp only
cases ... with
induction ... with
calc
```

The backend does not accept backend-specific user text. Tactic forms are emitted solely from structured proof IR.

### 18.8 External-interface probe module

Verification generates a probe module named:

```text
LexLeanProbe.P<first-32-hex-of-semantic-id>
```

For every used external Lean entry, sorted by qualified entry ID, it emits an `example` whose declared type is the entry's linked signature and whose value is the external Lean constant. Universe variables are alpha-renamed with an entry-index prefix.

The probe establishes only that the external constant can inhabit the declared interface in the pinned environment. It is not included in the canonical document, module API, or axiom-policy results.

A preexisting module with the reserved probe name is an environment conflict and causes verification to fail. Before any compilation, LexLean also rejects a preexisting workspace/search-path module whose full name equals any generated document module.

### 18.9 Axiom-audit module family

Verification reserves an audit module root named:

```text
LexLeanAudit.A<first-32-hex-of-semantic-id>
```

For every generated module `<generated-module>`, it generates one audit member
named:

```text
LexLeanAudit.A<first-32-hex-of-semantic-id>.<generated-module>
```

Members are ordered by generated module name. Each member imports exactly its
generated module and emits one:

```lean
#print axioms <fully-qualified-declaration-name>
```

for every declaration installed in that generated module's native Lean
environment, in sorted fully qualified name order. Thus each declaration
present in the generated environment is audited exactly once, while no audit
process reconstructs the entire generated environment as new declarations.

An audit member contains no other command and no comments. Every member source
and normalized process record, plus the concatenated normalized output in
member order, are verification artifacts.

A preexisting module with the reserved audit root or any member name is an
environment conflict and causes verification to fail.

---

## 19. Canonical LaTeX and PDF

### 19.1 Canonical LaTeX is generated, not copied

The LaTeX backend consumes linked IR. It MUST NOT copy an author's sentence, alias spelling, whitespace, TeX control, or punctuation directly into output.

Every visible output word is produced from a canonical glossary form. Every mathematical construct is produced through LRE and the renderer-token registry. Every structural control is produced by the fixed backend. Every non-whitespace canonical-TeX atom, including fixed preamble options, package names, environment names, braces, labels, and separators, receives an output-coverage origin.

### 19.2 Exact preamble

Each module emits a standalone `.tex` file with this logical preamble and order:

```latex
\documentclass[11pt]{article}
\usepackage[T1]{fontenc}
\usepackage{amsmath}
\usepackage{amssymb}
\usepackage{amsthm}
\usepackage[hidelinks]{hyperref}
\newtheorem{theorem}{Theorem}[section]
\newtheorem{lemma}[theorem]{Lemma}
\newtheorem{corollary}[theorem]{Corollary}
\theoremstyle{definition}
\newtheorem{definition}[theorem]{Definition}
\begin{document}
```

The file ends with:

```latex
\end{document}
```

No comment, author, date, timestamp, generator banner, path, or host metadata is emitted.

### 19.3 Document rendering

- The title is rendered in a `center` environment with `\LARGE`.
- Source sections render as `\section{...}` and nested sections as `\subsection{...}`. Nesting beyond two levels continues with a deterministic bold heading construct registered in the renderer.
- Section parameters render immediately below the heading as a display labeled by the core glossary concept `Parameters`.
- Type, term, and predicate definitions render in `definition`.
- Theorem-like kinds use their corresponding environments.
- Every component receives:
  ```latex
  \label{ll:<module-slug>:<component-id>}
  ```
- Statements render from proposition or definition IR.
- Proofs render from proof IR using the canonical proof phrases in `lexlean.core`.
- Imported-module lists are not human-visible claims and are omitted.
- The file uses LF and one final LF.

### 19.4 Canonical proposition rendering

Canonical prose uses the controlled grammar:

- leading `For every`;
- `there exists`;
- `there exists exactly one`;
- `if ..., then ...`;
- `if and only if`;
- `and`, `or`, and `not`;
- canonical predicate frames.

Parentheses are inserted when grammar precedence would otherwise change the IR. Mathematical terms use canonical LRE.

### 19.5 Canonical proof rendering

Proof IR renders to fixed formal prose. Representative mappings are:

| Proof IR | Canonical text |
|---|---|
| `Intro` | `Assume ... .` |
| `Exact` | `The goal follows from ... .` |
| `ApplyOne` | `Apply ... .` |
| `Reflexivity` | `The goal follows by reflexivity.` |
| `Have` | `We first establish ... .` followed by its nested proof |
| `Rewrite` | `Rewrite ... using ... .` with every direction stated |
| `SimplifyOnly` | `Simplify ... using only ... .` |
| `Cases` | `Consider the cases of ... .` |
| `Induction` | `Proceed by induction on ... .` |
| `Calculate` | a displayed aligned chain |

These words are core glossary entries. The renderer never invents synonyms.

### 19.6 Output lexical coverage

For every canonical `.tex`, LexLean emits a coverage record proving mechanically that:

- every visible word maps to a glossary entry;
- every mathematical token maps to a glossary entry, local, numeral, or core constructor;
- every control sequence maps to a renderer token;
- every punctuation mark maps to core grammar or renderer data;
- no raw output segment is unclassified.

Failure to produce complete output coverage is a backend error and prevents artifact publication.

### 19.7 External PDF provider

PDF output is optional and has no proof authority.

When configured, `lexlean verify`:

1. verifies the provider executable SHA-256;
2. invokes `version_argv` with no shell;
3. normalizes stdout and verifies `version_stdout_sha256`;
4. creates an isolated temporary working directory;
5. copies only the canonical `.tex` and declared regular resource files into it;
6. expands only the whole-argument placeholders:
   - `{input}`;
   - `{out_dir}`;
   - `{stem}`;
7. invokes `compile_argv` directly, with no shell parsing;
8. enforces child timeout and output-size limits;
9. requires exactly the configured output regular file;
10. requires the bytes to begin with `%PDF-`;
11. copies the PDF atomically into the platform-bound verification artifact set;
12. records process, recipe, and output hashes.

PDF bytes and process records are not placed in the platform-independent build directory.

A placeholder embedded in a larger argument is forbidden. Each required placeholder must occur exactly once where the schema requires it. The provider receives no project directory as its working directory and no undeclared resource.

### 19.8 PDF recipe content address

The PDF recipe ID is:

```text
SHA256(
  "lexlean-pdf-recipe-v1\0"
  || frame("tex", canonical_tex_sha256_bytes)
  || frame("program", program_sha256_bytes)
  || frame("version-output", version_stdout_sha256_bytes)
  || frame("argv", canonical_json(compile_argv))
  || frame("resources", canonical_json(sorted resource path/hash rows))
)
```

The actual PDF SHA-256 is recorded separately. The recipe and PDF are platform-bound evidence and do not affect `semantic_id`.

---

## 20. Diagnostics, source maps, and coverage

### 20.1 Diagnostic object

Every diagnostic serializes as:

```json
{
  "spec": "lexlean/diagnostic/1",
  "code": "LLL1004",
  "severity": "error",
  "message": "unknown source atom",
  "primary": {
    "path": "src/Main.lex.tex",
    "byte_start": 120,
    "byte_end": 129,
    "line_start": 7,
    "column_start": 5,
    "line_end": 7,
    "column_end": 14
  },
  "labels": [],
  "notes": [],
  "help": [],
  "causes": []
}
```

Offsets are zero-based half-open UTF-8 byte offsets. Lines and columns are one-based; columns count Unicode scalar values after normalization.

Object keys are canonicalized. Diagnostics sort by:

1. project-relative path;
2. byte start;
3. severity;
4. code;
5. message.

### 20.2 Severity

Language 1.0 compiler diagnostics are either `error` or attached `note`/`help`. There is no recoverable compiler warning category.

Any warning emitted by Lean, Lake, `leanchecker`, or a PDF provider is a verification failure unless the output is the expected informational result of the generated axiom-audit commands.

### 20.3 Source-map schema

Each module map contains:

```json
{
  "spec": "lexlean/source-map/1",
  "source_id": "...",
  "semantic_id": "...",
  "module": "Main",
  "sources": [],
  "artifacts": [],
  "nodes": [],
  "mappings": []
}
```

A mapping identifies:

- artifact kind and relative path;
- generated half-open byte range;
- source file and half-open byte range, or a synthetic core/glossary origin;
- IR node ID;
- role: `declaration`, `binder`, `term`, `proof`, `structure`, `renderer`, or `synthetic`.

Every non-whitespace generated Lean token and every canonical LaTeX token/control has at least one mapping. Boilerplate maps to explicit synthetic origins such as `core:lean-preamble/1`, never to a fabricated source span.

### 20.4 Lean diagnostic remapping

Verification parses Lean locations against generated files. It selects the smallest generated mapping that encloses the reported byte position. Ties are resolved by:

1. shortest generated range;
2. non-synthetic before synthetic;
3. lowest stable IR node ID.

The remapped diagnostic contains both:

- the primary LexLean source span;
- the generated Lean location as a note.

If an external Lean diagnostic has no generated mapping, the primary location is the declaration component and the unmapped generated path/range is retained as a note.

### 20.5 Coverage schema

Each module coverage file contains sorted rows for:

- source primitive atoms;
- selected lexical forms;
- scoped declarations;
- canonical LaTeX visible tokens;
- canonical LaTeX controls;
- generated Lean semantic tokens.

Every source atom row records exactly one selected binding. Whitespace rows are optional; non-whitespace rows are mandatory. Overlap or a gap is an internal invariant failure.

### 20.6 Diagnostic output modes

The CLI supports:

- `human`: deterministic text to stderr, optional color;
- `json`: one canonical JSON command-result object to stdout and no human diagnostic text.

JSON output has no timestamps, absolute paths, or terminal escape sequences.

The one command-result object is:

```json
{
  "spec": "lexlean/command-result/1",
  "command": "check",
  "success": false,
  "exit_code": 1,
  "modules": [],
  "artifacts": [],
  "diagnostics": []
}
```

Absent IDs are omitted rather than encoded as JSON `null`. Module and artifact rows are sorted. `success` is true exactly when `exit_code` is zero.

---

## 21. Content identity and manifests

### 21.1 Hash framing

All compound SHA-256 inputs use:

```text
frame(label, bytes) =
  u32be(length(label_utf8))
  || label_utf8
  || u64be(length(bytes))
  || bytes
```

Labels are ASCII and unique within each hash recipe.

### 21.2 Compiler-semantics ID

`language/semantics.toml` freezes language 1.0 exactly:

```toml
spec = "lexlean/compiler-semantics/1"
language = "1.0"
project_schema = "lexlean/project/1"
lock_schema = "lexlean/lock/1"
lexicon_schema = "lexlean/lexicon/1"
entry_schema = "lexlean/entry/1"
lean_backend = "3"
latex_backend = "2"
proof_lowering = "1"
axiom_parser = "lean-4.32.1/2"
canonical_json = "1"
```

`language/semantics-1.1.toml` independently versions the language-1.1
semantic IR, snapshot, proof forms, and fixed backends. A 1.1-only change
updates that file and therefore the 1.1 compiler-semantics ID without changing
the historical 1.0 ID. `language/semantics-1.2.toml` versions language 1.2 in
the same way.

Each language's compiler-semantics ID is the §11.5 tree digest of a fixed,
nested partition of the embedded tree. The language-1.2 ID covers the whole
tree. The language-1.1 ID excludes the files introduced solely for 1.2:
`language/bootstrap-1.2.toml`, `language/semantics-1.2.toml`,
`language/production-1.2.toml`, `language/lcnf-1.2/`, `language/core-1.2/`,
`language/std/{bool,int,nat}-1.2/`, `schemas/attestation-v2.schema.json`,
`schemas/build-manifest-v2.schema.json`,
`schemas/compiler-input.schema.json`, `schemas/gnaf-fixture.schema.json`,
`schemas/gnaf-request.schema.json`,
`schemas/lexicon-v2.schema.json`, `schemas/lock-1.1.schema.json`,
`schemas/lock-v2.schema.json`, `schemas/production-eligibility.schema.json`,
`schemas/project-v2.schema.json`, `schemas/rust-package.schema.json`,
`schemas/rust-provenance.schema.json`,
`schemas/semantic-module-v2.schema.json`,
`schemas/semantic-snapshot-v2.schema.json`,
`schemas/target-fixture.schema.json`, and
`schemas/target-program.schema.json`. The language-1.0 ID additionally
excludes the files introduced solely for 1.1: `language/bootstrap-1.1.toml`,
`language/semantics-1.1.toml`, `language/core-1.1/`,
`language/std/{bool,int,nat}-1.1/`, `schemas/semantic-module.schema.json`,
and `schemas/semantic-snapshot.schema.json`. Adding a file for a newer
language therefore never changes an older language's ID.

The full (language-1.2) tree is the §11.5 tree digest of:

- every regular file under `language/`;
- every regular file under `schemas/`;
- committed axiom-output parser fixtures under `tests/golden/axiom-parser/`;
- committed canonical-JSON fixtures under `tests/golden/canonical-json/`.

The specification-link gate ensures these version declarations agree with this document. The digest excludes README prose, CI YAML, host binaries, timestamps, and generated build output.

The released binary embeds all three closed language IDs. Repository tests
recompute the complete current input tree and, from the exclusion lists of
this section, the compatibility-filtered trees independently, compare each with
the embedded ID, and pin the 1.0 and 1.1 IDs to the committed example locks.

### 21.3 Source ID

The source ID is:

```text
SHA256(
  "lexlean-source-v1\0"
  || frame("project", canonical_project_toml_bytes)
  || frame("lock", canonical_lock_bytes)
  || for each selected module/import-closure source in sorted path order:
       frame("path", project_relative_path_utf8)
       || frame("source", normalized_source_bytes)
)
```

`build_root` is retained as its project-relative configured spelling. Absolute project location is excluded.

### 21.4 Semantic ID

The semantic ID is:

```text
SHA256(
  "lexlean-semantic-v1\0"
  || frame("compiler-semantics", compiler_semantics_digest_bytes)
  || frame("language", "1.0")
  || frame("toolchain", "leanprover/lean4:v4.32.1")
  || frame("linked-ir", canonical_linked_ir_json)
  || frame("lexicon-closure", canonical_linked_lexicon_json)
)
```

It is platform-independent. Display spellings, title/heading phrase IR, and proof IR are included because they affect canonical document output.

### 21.5 Build ID and layout

The build ID distinguishes different source/configuration identities that link to the same semantic object:

```text
SHA256(
  "lexlean-build-v1\0"
  || frame("source-id", source_id_bytes)
  || frame("semantic-id", semantic_id_bytes)
)
```

A successful build is published atomically at:

```text
.lexlean/build/<build-id>/
```

with:

```text
manifest.json
modules/<full-module-path>.lean
modules/<full-module-path>.tex
maps/<full-module-path>.map.json
coverage/<full-module-path>.coverage.json
lexicons/<source-module>.closure.json
production/<full-module-path>.eligibility.json
```

The `production/` report exists only for a language-1.2 module that declares a
production root (§17.13). Paths in manifests use `/` regardless of host OS.

### 21.6 Build manifest

`manifest.json` has:

```json
{
  "spec": "lexlean/build-manifest/1",
  "compiler": {
    "version": "1.0.0",
    "semantics_id": "..."
  },
  "language": "1.0",
  "project": "example-project",
  "source_id": "...",
  "semantic_id": "...",
  "build_id": "...",
  "lean_toolchain": "leanprover/lean4:v4.32.1",
  "selection": [],
  "modules": [],
  "inputs": [],
  "outputs": []
}
```

Each output row contains kind, project/build-relative path, byte length, and SHA-256. Rows sort by `(kind, path)`.

In language 1.2, every configured model artifact (§10.1, §17.12) is an
`inputs` row of kind `model-artifact` with its project-relative path, byte
length, and SHA-256, so the bytes a build's typed artifact values were
decoded from are named by the manifest.

The manifest does not contain its own hash. A verification attestation records the exact manifest-file hash.

### 21.7 Canonical JSON

All normative JSON uses this restricted canonical form:

- UTF-8;
- no BOM;
- no floating-point numbers;
- no JSON `null`; optional fields are omitted;
- no duplicate keys;
- object keys sorted by UTF-8 bytes;
- arrays in specified semantic order;
- integers in shortest decimal form;
- strings use required JSON escapes and otherwise raw UTF-8;
- Unicode scalar sequences in strings and keys are preserved exactly as provided; no normalization is applied;
- no insignificant whitespace;
- one final LF in the file;
- hash recipes over canonical payload omit the final file LF unless they explicitly hash file bytes.

### 21.8 Atomicity and concurrency

Mutating commands acquire an exclusive lock at:

```text
.lexlean/.lock
```

The lock file itself is regular and contains no semantic data.

Artifacts are written to a same-filesystem staging directory, fsynced where supported, and renamed only after all required files and hashes are complete. Existing content-addressed directories are reused only after every file validates against the new manifest; otherwise the command fails rather than overwriting unexplained bytes.

A failed command removes its staging tree and leaves no verified artifact.

---

## 22. Verification

### 22.1 Verification stages

`lexlean verify` performs exactly:

1. project/config/lock validation;
2. check and linked-IR construction;
3. deterministic build rendering;
4. Lean/Lake/leanchecker toolchain preflight;
5. Lake-workspace lock preflight;
6. external-interface probe generation and elaboration;
7. generated-module elaboration and `.olean` production;
8. separate-process `leanchecker` replay for every generated module;
9. process-sized axiom-audit module-family generation and execution;
10. exact axiom-output parsing;
11. per-declaration policy enforcement;
12. named-root extraction, when the project has a production root (§22.10);
13. optional configured PDF rendering;
14. process-output normalization;
15. verification-attestation construction;
16. atomic publication.

No stage is optional. PDF is absent only when the project configuration has no PDF provider, and named-root extraction only when no module declares a production root.

### 22.2 Lake-resolved execution

All Lean processes run with working directory equal to the configured Lake workspace and through the environment produced by:

```text
lake env <tool> <arguments...>
```

LexLean locates `lake` from the pinned toolchain, uses an absolute executable path, and verifies its version and digest.

Generated source and `.olean` roots are prepended to `LEAN_PATH` for the invocation. Every generated-module compilation also passes `-R <generated-source-root>` so Lean derives the module name from a normalized package root rather than embedding a random staging path in `.olean` bytes. Source and output paths mirror module names, so Lean module discovery and same-platform `.olean` bytes are deterministic across absolute project and staging roots.

LexLean does not run `lake update`, fetch dependencies, or modify the user's Lake files.

### 22.3 Module compilation

Modules compile in topological import order using the pinned `lean` executable. Each compilation emits:

- one `.olean`;
- captured normalized stdout;
- captured normalized stderr;
- exit status;
- process argv with normalized paths;
- executable digest.

LexLean does not request or include `.ilean`. Editor-information artifacts are not part of the normative proof artifact set.

Any nonzero exit, warning, unknown informational message, missing `.olean`, or output overflow fails verification.

### 22.4 `leanchecker`

For each generated module, sorted by full module name, LexLean invokes the pinned `leanchecker` in a separate process with that module prefix.

A zero exit is required. Normalized output is recorded.

This replay checks the newly loaded environment through Lean's kernel and is intended to detect environment manipulation. It is explicitly not described as an independent verifier.

### 22.5 Axiom output

With Lean 4.32.1, the accepted normalized payloads for one declaration are exactly:

```text
'<name>' does not depend on any axioms
```

or:

```text
'<name>' depends on axioms: [<comma-separated Lean names>]
```

Each audit member's output is parsed against that member's declarations before
the observed maps are merged. The parser:

- accepts an optional Lean location/information envelope;
- requires the quoted declaration name to equal the expected full name;
- parses zero or one record per generated command and requires exactly one;
- trims only envelope whitespace;
- sorts and deduplicates the parsed axiom set, rejecting duplicate textual names;
- rejects any unrecognized payload;
- rejects missing or extra records.

Golden fixtures are taken from the pinned toolchain's `#print axioms` behavior.

### 22.6 Policy enforcement

For observed set `O` and configured set `A`:

- `none` succeeds iff `O = ∅`;
- `allow` succeeds iff `O ⊆ A`;
- `exact` succeeds iff `O = A`.

The attestation records policy, allowed set, observed set, and result for every declaration.

An imported theorem's axioms are not exempt because the theorem is upstream. If they flow into a generated declaration, the generated declaration's policy must permit them.

A language-1.2 model declaration (§17.12) is audited through the Lean
declarations it generates: its elaborated definitions and theorems, its
cross-checks, and its artifacts' `.bytes` and `.decoded` declarations. Each
is recorded with policy `allow` over the owner's axiom set, and the union of
their observed sets must equal the owner's set exactly (`LLV7005`
otherwise), so the owner's `exact` policy is met by its elaboration as a
whole and no generated declaration may use an axiom the owner does not state.

### 22.7 Process normalization

Before process output is hashed:

1. CRLF and CR become LF;
2. ANSI escape sequences are removed;
3. the longest matching absolute prefixes are replaced, in this order:
   - staging root → `$STAGING`;
   - project root → `$PROJECT`;
   - Lake workspace → `$LAKE_WORKSPACE`;
   - toolchain root → `$TOOLCHAIN`;
   - user home → `$HOME`;
4. trailing spaces are removed;
5. blank final lines collapse to one final LF.

Unexpected absolute paths remaining in successful normalized output fail attestation construction.

### 22.8 Verification layout

After success, the staging directory is renamed to:

```text
.lexlean/verified/<attestation-id>/
```

It contains:

```text
attestation.json
build-manifest.json
modules/*.lean
modules/*.tex
maps/*.map.json
coverage/*.coverage.json
lexicons/*.closure.json
oleans/*.olean
probe/<probe-module>.lean
probe/process.json
audit/<audit-module-member>.lean
audit/output.txt
audit/<audit-module-member>.process.json
process/lean/*.json
process/leanchecker/*.json
production/*.eligibility.json              # when a production root exists
extract/<extraction-module>.lean           # when a production root exists
extract/process.json                       # when a production root exists
production/compiler-input.json             # when a production root exists
pdf/*                                      # when configured
```

The artifact set is fixed. `verify` has no output-directory option and no option to omit maps, coverage, source, audit records, or process records.

### 22.9 Attestation ID

The attestation object contains `attestation_id`, but the ID is computed over the canonical object with that field removed:

```text
SHA256(
  "lexlean-attestation-v1\0"
  || frame("attestation-body", canonical_json(body_without_attestation_id))
)
```

The body records:

- build manifest bytes and SHA-256;
- semantic ID;
- host OS and architecture;
- LexLean version and executable hash;
- Lean, Lake, and leanchecker version output and executable hashes;
- Lake workspace lock hashes;
- every process record;
- generated `.olean` hashes;
- axiom policies and observed sets;
- the canonical compiler input's byte length and SHA-256, when a production root exists;
- optional PDF process and bytes;
- overall status exactly `verified`.

A language-1.0 or 1.1 project's attestation is `lexlean/attestation/1`
(`schemas/attestation.schema.json`). A language-1.2 project's is
`lexlean/attestation/2` (`schemas/attestation-v2.schema.json`, in the 1.2-only
partition): the same body plus `compiler_input`, present exactly when a
production root exists. Routing is fixed by the project language.

There is no timestamp in the hashed attestation. Digital signing is outside language 1.0; release automation may sign the completed file without changing its contents.

### 22.10 Named-root extraction

A LexLean-authored compiler consumes verified generated Lean only through
Lean's own compiler front end, at one explicit authority boundary. Lean's
behavior there is an external authority (`LEAN-LCNF-4-32-1`), cited and pinned,
never a LexLean-proved fact.

**Authority interface.** `language/lcnf-1.2/authority.toml`
(`lexlean/lcnf-authority/2`) is the closed registry of every constant the
adapter's own definitions use, each in exactly one class:

- a *call* (a definition of Lean's `Lean` namespace other than a projection,
  recursor, instance, or matcher) names its exact signature at Lean 4.32.1
  (`f054605aea4b840552cca2e725580bffd1e1b704`), the defining source file at
  that revision with its SHA-256, and its role;
- a *type* of the `Lean` namespace names its complete constructor list in
  Lean's order;
- *plumbing* lists every other used constant by name: constructors,
  projections, recursors, instances, matchers, and core-library data, whose
  meaning the calls and types fix.

The registry and the adapter are in the language-1.2 partition (§21.2), so
both are part of the compiler-semantics ID. No user source can supply LCNF, a
Lean expression, or a root name: roots are the linked production roots
(§17.13).

**Adapter.** `language/lcnf-1.2/extract.lean` is the only Lean the extraction
runs, and it reports facts, not decisions. Its code contains no comment and
no optimization or application policy. From the roots it follows the code of
every translated definition: a constant of the project's generated modules
(the fixed runtime namespaces `<module>.LexLeanRuntime` and
`<module>.LexLeanCollections` included) that is a computable definition Lean
generates code for is translated with `Lean.Compiler.LCNF.toDecl` in the base
phase, and no LCNF pass runs afterwards. It records, for every project
constant reached through code or named by a reached kernel value, its kind,
defining module, computability, whether code is generated for it, whether it
is a compiler-generated helper, and the constants its kernel value names; for
a translated definition, its safety, universe parameters, LCNF type,
parameters, code, and the constants that code names; for a project inductive,
its constructors with their LCNF types; and, for every other constant
translated code names, the same facts. Universe levels are recorded wherever
a constant is instantiated. An LCNF type is recorded through its metadata
annotations, which carry no type (Lean attaches them, for example, to the
types of instances such as the portable runtime's integer quotient). It
prints one `lexlean/lcnf-extraction/2` record.

**Driver.** Verification generates the module `LexLeanExtract.X<hex32>` (the
first 32 hex digits of the semantic ID; a reserved module name) with exactly:
`module`, `public meta import Lean`, one `import all` per generated module, the
fixed option `set_option linter.unusedVariables false`, the adapter, a
`universe` declaration of the universe variables the registered signatures
name, one `lexlean_signature <name> : <signature>` per registry call in
registry order, one `#eval` of the registry check, and one `#eval` naming the
roots and the modules. It is elaborated with `lake env lean` against the
staged module outputs, after policy enforcement (stage 12, §22.1).

- `lexlean_signature` fails unless the constant's type equals the registered
  signature structurally up to binder names: a changed result, argument,
  binder kind, universe, or default value is drift, even where definitional
  equality would hide it.
- The registry check fails unless the adapter's own definitions use exactly
  the registered constants, each in its registered class, and every registered
  type has exactly its registered constructors.
- Any Lean message on a probe line, on the registry check, or inside the
  adapter, and any message the adapter raises as drift, is authority drift
  (`LLV7012`), whether or not Lean exits successfully; Lean reports messages
  on standard output, so a warning is never mistaken for part of the record.
- An error the adapter raises by name (an unknown root, an unresolved
  constant) is a rejection (`LLV7011`).

**Decisions.** The host decides everything the facts admit:

- each root's *closure* is the translated project definitions reached from
  it through translated code; a reached project constant that Lean does not
  translate is refused by its facts;
- the closure's *runtime members* are those of the runtime namespaces; its
  *source members* must equal the root's production-eligibility closure;
- its *erased proofs* are the project theorems its members' kernel values
  name, directly or through compiler-generated helpers, and the non-helper
  ones must equal the eligibility analysis's erased dependencies;
- a definition is *recursive* exactly when it lies on a cycle of the
  translations' use graph;
- each closure carries the eligibility analysis's monomorphization plan: every
  instance with its declaration and type arguments.

**Fail-closed rejection (`LLV7011`).** The host reads the record with a closed
schema and rejects:

- a record that is malformed, reports a constant twice, carries extra output,
  or answers other roots;
- a Lean version or source commit other than the pin (`LLV7012`);
- a translation reported for a constant Lean does not compile, or whose
  listed uses differ from what its code names;
- a closure member that is a theorem (a proof-as-runtime dependency), an
  axiom, an opaque (Lean's form of a partial definition), unsafe,
  noncomputable, without generated code, or implemented externally;
- an unsupported compiler form: an LCNF type with no closed representation, or
  a variable used outside the lexical scope that binds it;
- an external that is not a constant of Lean's `Init` library, or that is an
  axiom, opaque, or noncomputable (an unresolved dependency);
- a constant translated code names that is neither reported as a project
  constant nor recorded as an external (a dropped dependency), and a project
  constant a kernel value names that is not reported (an unresolved
  dependency);
- a root whose source members differ from its production-eligibility closure:
  a member only Lean reaches is a dependency the eligibility analysis dropped,
  and a member only the analysis reaches is one Lean does not compile;
- a root whose erased proofs differ from the eligibility analysis's.

Base-phase translation names fixed-width literals through their `OfNat`
instances, so the record's fixed-width literal forms, which the schema admits,
are exercised only by the host's record tests, not by pinned Lean.

**Compiler input.** On success verification publishes
`production/compiler-input.json` (`lexlean/compiler-input/2`,
`schemas/compiler-input.schema.json`): the Lean identity, the roots, each
root's closure with its source members, runtime members, instances, and erased
proofs, every closure definition with its recursion, universe parameters,
type, parameters, and LCNF code, the project inductives, the externals, and
the erased proofs. Every LCNF variable is renamed `v<n>` in binding order
within its declaration and every list is sorted, so the bytes and their
SHA-256 (the compiler-input ID) depend on neither Lean's unique-name counter,
the source path, nor the host. The attestation records the byte length and
SHA-256, and the normalized verification records include the input, the
driver, and its process record.

---

## 23. Command-line interface

### 23.1 General form

```text
lexlean [GLOBAL-OPTIONS] <COMMAND> [COMMAND-OPTIONS]
```

Global options:

```text
--project <path>                  default: lexlean.toml discovered upward
--diagnostic-format human|json    default: human
--color auto|always|never         default: auto; ignored for json
--version
--help
```

No environment variable changes semantic configuration.

### 23.2 Project discovery

Without `--project`, LexLean searches the current directory and parents for the first regular `lexlean.toml`. It stops at the filesystem root. Symlinked candidates are rejected.

The directory containing the selected config is the project root.

### 23.3 Selection

Commands accepting a selection support:

```text
--all
[INPUT...]
```

The modes are mutually exclusive:

- `--all`: every `.lex.tex` beneath every source root;
- one or more input paths: those modules and their transitive imports;
- neither: configured entrypoints and their transitive imports.

Inputs are project-relative or absolute paths that resolve beneath a configured source root. Duplicate logical modules, case-fold collisions, and two paths declaring the same module are errors.

Selections canonicalize to a sorted set. APIs and CLI results are always project result sets, even for one module.

### 23.4 Commands

#### `init`

```text
lexlean init [PATH] --name <name> --module-prefix <prefix>
```

Creates a new project only in an absent or empty directory. It writes canonical config, a canonical initial lock for builtin packages and workspace pins, `lean-toolchain`, a minimal Lake workspace, `src/Main.lex.tex`, and `.gitignore`. It never overwrites.

#### `lock`

```text
lexlean lock [--check] [--allow-network]
```

`--check` and `--allow-network` are mutually exclusive. Without either, it updates from local and cached exact sources. It writes atomically.

#### `check`

```text
lexlean check [--all | INPUT...]
```

Runs through linked IR and emits no build artifacts.

#### `build`

```text
lexlean build [--all | INPUT...]
```

Emits the fixed build layout at the build-ID path. It does not run Lean, does not invoke the PDF provider, and does not claim verification.

#### `verify`

```text
lexlean verify [--all | INPUT...]
```

Runs §22. It accepts no output or stage-suppression option.

#### `fmt`

```text
lexlean fmt [--check] [--all | INPUT...]
```

Parses and uniquely resolves source, rewrites canonical source, or exact-byte compares under `--check`.

#### `clean`

```text
lexlean clean
```

Removes only the configured `.lexlean` build root after verifying it is a nonsymlink directory inside the project. It does not remove package caches outside that root.

#### `explain`

```text
lexlean explain <DIAGNOSTIC-CODE>
```

Prints the generated `ERRORS.md` entry for one registered code. Unknown codes exit as CLI misuse.

### 23.5 Formatting

Canonical source formatting:

- NFC and LF;
- two spaces per environment nesting level;
- imports sorted;
- safe canonical lexical forms selected;
- explicit qualified selectors retained only when required for disambiguation;
- one structural control per line;
- one proposition or definition sentence per logical line;
- one proof sentence per line;
- axiom names sorted;
- no trailing whitespace;
- one final LF.

Formatting MUST preserve linked IR. The formatter compares pre- and post-render canonical IR and fails if they differ.

### 23.6 Exit codes

| Code | Meaning |
|---:|---|
| `0` | Command succeeded. |
| `1` | Source, glossary, grammar, semantic, Lean, proof, or axiom-policy failure. |
| `2` | CLI misuse, project-config error, lock-schema error, or invalid selection. |
| `3` | Missing or mismatched toolchain, Lake workspace, executable, or environment. |
| `4` | Security-policy or explicit resource-limit violation. |
| `70` | Internal invariant or software failure. |

A process terminated by the OS may additionally expose the platform's signal/exception status.

### 23.7 Output streams

Human mode:

- successful summaries to stdout;
- diagnostics to stderr;
- no progress spinner in noninteractive mode;
- paths project-relative where possible.

JSON mode:

- exactly one canonical JSON command-result object to stdout;
- stderr empty unless the process cannot construct JSON because of an internal failure;
- no color or progress text.

---

## 24. Public Rust API

### 24.1 Stable entry point

`crates/lexlean/src/lib.rs` exports:

```rust
pub struct Engine;
```

with:

```rust
impl Engine {
    pub fn load(project_file: &Utf8Path) -> Result<Self, LexLeanError>;
    pub fn lock(&self, request: LockRequest) -> Result<LockResult, LexLeanError>;
    pub fn check(&self, request: CheckRequest) -> Result<ProjectResultSet<CheckedUnit>, LexLeanError>;
    pub fn snapshot(&self, request: CheckRequest) -> Result<SemanticSnapshot, LexLeanError>;
    pub fn build(&self, request: BuildRequest) -> Result<ProjectResultSet<BuiltUnit>, LexLeanError>;
    pub fn verify(&self, request: VerifyRequest) -> Result<VerifiedProject, LexLeanError>;
    pub fn format(&self, request: FormatRequest) -> Result<FormatResultSet, LexLeanError>;
}
```

`snapshot` executes the same normalization, lexical closure, parsing,
elaboration, linking, resource-limit, and diagnostic pipeline as `check`. It
does not invoke either backend, start a child process, or write a project
artifact. Its owned, read-only DTO is serialized as
`lexlean/semantic-snapshot/1`, or `lexlean/semantic-snapshot/2` for a
language-1.2 project (§17.12); object keys use canonical ASCII order, arrays
use their specified semantic order, and the file spelling has exactly one
final LF. The snapshot ID is SHA-256 of those exact canonical bytes.

The envelope records source, semantic, compiler-semantics, and selected
language identities; normalized relative source identities; sorted module,
import, glossary, and visible-package closures; declarations, generated Lean
names, axiom policies, and origins; canonical linked terms and proofs; and the
complete native semantic module value, including expression DAG, binders,
structures, classes, instances, inductives, constructors, recursion and proof
metadata. Absolute paths, environment data, Rust debug text, mutable compiler
references, and backend output are not representable. The published JSON
Schema and derived Serde representation are the same shape.

### 24.2 Selection type

```rust
pub enum Selection {
    Entrypoints,
    All,
    Files(BTreeSet<Utf8PathBuf>),
}
```

An empty `Files` set is invalid. No API treats a multi-module operation as singular.

### 24.3 Requests

```rust
pub struct CheckRequest {
    pub selection: Selection,
}

pub struct BuildRequest {
    pub selection: Selection,
}

pub struct VerifyRequest {
    pub selection: Selection,
}

pub struct FormatRequest {
    pub selection: Selection,
    pub check_only: bool,
}

pub struct LockRequest {
    pub check_only: bool,
    pub allow_network: bool,
}
```

Build and verify requests cannot alter artifact selection, backend, axiom audit, toolchain, limits, or PDF policy.

### 24.4 Result sets

```rust
pub struct ProjectResultSet<U> {
    pub source_id: Sha256Digest,
    pub semantic_id: Sha256Digest,
    pub build_id: Option<Sha256Digest>,
    pub units: BTreeMap<ModuleName, U>,
}

pub struct CheckedUnit {
    pub module: ModuleName,
    pub summary: CheckedUnitSummary,
}

pub struct BuiltUnit {
    pub module: ModuleName,
    pub artifacts: ModuleArtifacts,
}

pub struct VerifiedProject {
    pub source_id: Sha256Digest,
    pub semantic_id: Sha256Digest,
    pub build_id: Sha256Digest,
    pub attestation_id: Sha256Digest,
    pub root: Utf8PathBuf,
    pub units: BTreeMap<ModuleName, VerifiedUnit>,
}
```

The full mutable compiler IR remains internal. Stable serializable summaries and artifact schema types are public.

### 24.5 Errors

Every public function returns only `LexLeanError`:

```rust
pub struct LexLeanError {
    pub class: ErrorClass,
    pub diagnostics: Vec<Diagnostic>,
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}
```

`ErrorClass` is:

```rust
pub enum ErrorClass {
    Language,
    CliOrConfiguration,
    Environment,
    SecurityOrLimit,
    Internal,
}
```

It maps exactly to CLI exit codes 1, 2, 3, 4, and 70.

The native API exposes optional, producer-set `DiagnosticDetail` through
`Diagnostic::detail()`. `PackageImportCycle { packages, importers }` records the actual
closed import walk, including its repeated final package, only for package
self-import or package-import graph cycles. Module and defined-denotation
cycles do not carry that detail. `importers` contains sorted distinct exact
`package@version` references whose validated import edges reach the cycle,
including its members. Package-local self-import rejection keeps its existing
timing; importers are extended only through successfully resolved, uniquely
identified packages and exact-version edges. Invalid identities, mismatched
versions, and unvalidated edges cannot establish reachability.
`UnqualifiedCrossPackageTermAmbiguity { candidates, packages, span }` records
sorted distinct qualified entry IDs from at least
two packages competing at the same unqualified lexical-term source range
among distinct surviving linked interpretations. Operator notation, binder
structure, and lexical-segmentation ambiguities alone do not carry that detail.
When multiple ranges qualify, the earliest range is reported. Explicitly
qualified selection is not an unqualified candidate. Its `packages` records
the sorted exact package references owning those candidates.

This metadata is native-only: canonical diagnostic JSON, human rendering,
diagnostic sorting, registered codes, and accepted-program artifact identities
are unchanged. The detail accessor is read-only; compiler producers alone set
the detail. `Diagnostic::new` creates a diagnostic without detail. Adding its
private storage in the unreleased 0.3.0 API is a Rust struct-literal source
compatibility change: downstream callers construct diagnostics with
`Diagnostic::new`, not struct literals. It is not a diagnostic wire-version
change or a claim of unchanged struct-literal source compatibility.

No public function panics for malformed user input, filesystem races, child failure, invalid UTF-8 input, invalid TOML/JSON, or unexpected external output.

---

## 25. Security and operational behavior

### 25.1 Filesystem confinement

LexLean:

- canonicalizes the project root once;
- rejects non-UTF-8 logical paths;
- rejects symlinks in source roots, lexicon packages, lock inputs, build roots, and declared PDF resources;
- rejects any normalized `..` escape;
- rejects duplicate files by filesystem identity;
- detects case-fold collisions before output;
- never follows a generated-output symlink;
- uses create-new semantics for staging files;
- validates existing content-addressed output before reuse.

### 25.2 No shell

Every child process is launched by executable path and argument vector. No command is passed through `sh`, `bash`, `cmd.exe`, PowerShell, or an equivalent shell.

### 25.3 No network

`check`, `build`, `fmt`, `verify`, `clean`, and `explain` do not invoke a network client. They do not update Lake or Git state.

Only `lock --allow-network` may acquire an exact configured Git commit over HTTPS. Prompts are disabled and mutable references are not accepted.

### 25.4 Child environment

Child environments begin from an allow-list required to locate the pinned toolchain and Lake workspace. LexLean explicitly sets:

```text
NO_COLOR=1
LANG=C.UTF-8
LC_ALL=C.UTF-8
GIT_TERMINAL_PROMPT=0
```

Platform-required path/home variables may be retained, normalized in records, and included in the environment preflight. Semantic configuration cannot be supplied through environment variables.

The PDF provider receives an isolated temporary home and no inherited proxy variables.

### 25.5 Limits

Every parser, token-lattice, graph, IR, diagnostic, child-output, and child-time limit is enforced with checked arithmetic. A limit failure is `LLS8002`, not an allocation panic.

### 25.6 Temporary data

Temporary directories are created beneath the configured build root with owner-only permissions where supported. They are removed on success and failure after atomic publication. LexLean does not print secret environment values or arbitrary file contents in diagnostics.

### 25.7 Internal invariants

An internal invariant failure:

- uses exit 70;
- emits `LLI9001`;
- identifies the phase and stable invariant name;
- does not claim a user source error;
- leaves no verified artifact.



## 26. Diagnostic registry and closed error model

### 26.1 `model/errors.toml`

Every public diagnostic code is registered once:

```toml
spec = "lexlean/errors/1"

[[error]]
code = "LLL1004"
class = "language"
exit = 1
title = "Unknown source atom"
statement = "A non-whitespace primitive atom has no covering lexical or scoped declaration."
```

Rows sort by code. Unknown fields, duplicate codes, invalid class/exit combinations, and an empty statement fail the model gate.

`ERRORS.md` is generated from this file. Source uses diagnostic codes only through a checked constructor or macro whose code argument is a compile-time string literal. `xtask` rejects a code used in Rust, tests, fixtures, or documentation that is absent from the registry, and rejects an unused registered code.

### 26.2 Code ranges

| Range | Class |
|---|---|
| `LLC0001`–`LLC0999` | CLI, configuration, selection, and lock structure |
| `LLL1001`–`LLL1999` | normalization, scanning, lexical closure, and token coverage |
| `LLP2001`–`LLP2999` | structural, phrase, mathematical, and proposition parsing |
| `LLR3001`–`LLR3999` | lexicon packages, references, imports, and resolution |
| `LLT4001`–`LLT4999` | conservative elaboration, signatures, and linking |
| `LLF5001`–`LLF5999` | definitions and structured proofs |
| `LLB6001`–`LLB6999` | Lean/LaTeX/PDF lowering, target realization, and artifact construction |
| `LLV7001`–`LLV7999` | toolchain, Lean, replay, audit, and verification |
| `LLS8001`–`LLS8999` | filesystem security, networking, child policy, and limits |
| `LLI9001`–`LLI9999` | internal invariants |

### 26.3 Required diagnostic codes

The initial registry MUST include at least these exact codes and meanings:

| Code | Meaning |
|---|---|
| `LLC0001` | Invalid command-line usage. |
| `LLC0002` | Mutually exclusive or empty selection. |
| `LLC0101` | Invalid or unknown project configuration field. |
| `LLC0102` | Missing, stale, or noncanonical lock file. |
| `LLC0103` | Unsupported schema or language version. |
| `LLC0104` | Duplicate module or case-folded path collision. |
| `LLL1001` | Invalid UTF-8, forbidden scalar, or line-ending form. |
| `LLL1002` | Forbidden comment, raw percent, tab, or TeX escape. |
| `LLL1003` | Non-NFC or noncanonical source requiring formatting. |
| `LLL1004` | Unknown non-whitespace primitive atom. |
| `LLL1005` | Source token-coverage gap or overlap. |
| `LLL1006` | Token-lattice construction failure. |
| `LLP2001` | No grammar parse. |
| `LLP2002` | More than one distinct linked interpretation. |
| `LLP2003` | Invalid structural order or component cardinality. |
| `LLP2004` | Invalid math precedence, associativity, or grouping. |
| `LLR3001` | Missing package, version, source, or digest mismatch. |
| `LLR3002` | Duplicate package, entry, form, or qualified ID. |
| `LLR3003` | Import, denotation, module, or definition cycle. |
| `LLR3004` | Invalid LSE, LRE, frame, form, or renderer token. |
| `LLR3005` | Unavailable or invalid document/external reference. |
| `LLR3006` | Unsafe canonical form or raw renderer output. |
| `LLR3007` | Model artifact unavailable or its digest or length mismatched (§10.1, §17.12). |
| `LLR3008` | Model artifact bytes violate the declared schema, type, or role (§17.12). |
| `LLT4001` | Arity, binder, category, or conservative type mismatch. |
| `LLT4002` | Unresolved overloaded entry. |
| `LLT4003` | External-interface probe mismatch. |
| `LLT4004` | Document-entry signature mismatch. |
| `LLT4005` | Production root not eligible for a declared target. |
| `LLT4006` | Model contract, realization, or artifact interface mismatch (§17.12). |
| `LLT4007` | Invalid model composition or entry obligation (§17.12). |
| `LLT4008` | Unvalidated model runtime boundary (§17.12). |
| `LLT4009` | Model evidence not established (§17.12). |
| `LLF5001` | Invalid definition form, self head, or recursion. |
| `LLF5002` | Invalid proof step for the current goal. |
| `LLF5003` | Missing, duplicate, or malformed proof branch. |
| `LLF5004` | Proof does not close all goals. |
| `LLF5005` | Forbidden proof form or proof hole. |
| `LLB6001` | Lean lowering has no defined form. |
| `LLB6002` | LaTeX lowering or renderer-token coverage failure. |
| `LLB6003` | Artifact hash, schema, or atomic publication failure. |
| `LLB6004` | PDF-provider protocol failure. |
| `LLB6005` | Target realization program or Rust package invalid or unrenderable (§17.14, §17.16). |
| `LLB6006` | GNAF request malformed (§17.15). |
| `LLV7001` | Lean/Lake/leanchecker version or executable mismatch. |
| `LLV7002` | Generated Lean elaboration or compilation failure. |
| `LLV7003` | `leanchecker` replay failure. |
| `LLV7004` | Missing, malformed, or unexpected axiom-audit output. |
| `LLV7005` | Axiom policy violation. |
| `LLV7006` | Lean warning or unexpected successful-process output. |
| `LLV7007` | Lake workspace lock or dependency availability mismatch. |
| `LLV7011` | Named-root extraction rejected (§22.10). |
| `LLV7012` | Lean compiler-front-end authority drift (§22.10). |
| `LLS8001` | Path escape, symlink, special file, or filesystem identity conflict. |
| `LLS8002` | Explicit resource limit exceeded: a project limit or an evaluator's declared capacity (§17.15). |
| `LLS8003` | Network operation attempted outside permitted lock acquisition. |
| `LLS8004` | Child environment, executable, hash, or shell-policy violation. |
| `LLI9001` | Internal invariant failure. |

An implementation may add a more specific code only by adding the model row, scenario, test, generated documentation, and behavior in the same change.

---

## 27. Repository claim and gate model

### 27.1 Separation of models

`model/*.toml` records claims about the LexLean repository and implementation.

`language/` and lexicon packages define the language accepted by LexLean.

A user's mathematical declarations never become repository conformance IDs merely by being compiled. This separation is strict.

### 27.2 Repository rules

`AGENTS.md` MUST define these rules and remain the only repository file that assigns `R` numbers:

| Rule | Requirement |
|---|---|
| R1 | `model/*.toml` is the single source of every repository conformance claim and public diagnostic code; generated claim/error documentation must match. |
| R2 | Honesty levels are load-bearing: `some-true`, `build`, and `open` are never blurred. |
| R3 | A capability begins with a register row, then one scenario, then one failing named test, then implementation. |
| R4 | Nothing is deferred: no deferral marker, stub, placeholder component, empty branch, or feature switch that disables a normative capability. |
| R5 | Every public failure is registered and emitted through the closed error model; user input never panics. |
| R6 | Shipped dependency boundaries, versions, licenses, and sources are explicit and audited. |
| R7 | Every accepted LexLean source and canonical output satisfies complete lexical and symbolic closure. |
| R8 | Lean and LaTeX are both derived from one linked IR; neither backend may accept an opaque bypass. |
| R9 | Verified status requires Lean elaboration, kernel replay, exact axiom audit, and policy success. |
| R10 | Platform-independent artifacts are deterministic, source-mapped, and content-addressed. |

### 27.3 Honesty levels

The template levels retain their meanings:

- `some-true`: a fact reproduced from an authority and not established by this repository;
- `build`: behavior constructed here and validated against its oracle;
- `open`: measured and reported, never asserted as established.

Every LexLean capability ID in §31 is `build`.

The initial repository has no `open` implementation claim. An open research or performance measurement may be added only as a ledger row and must not be used as acceptance evidence.

### 27.4 Authorities

`model/authorities.toml` MUST initially identify:

| Authority ID | Subject |
|---|---|
| `LEAN-REL-4-32-1` | Lean 4.32.1 release/tag and source commit |
| `LAKE-4-32-1` | Lake distributed with Lean 4.32.1 |
| `LEANCHECKER-4-32-1` | `leanchecker` replay behavior and its stated non-independent status |
| `PRINT-AXIOMS-4-32-1` | Lean 4.32.1 `#print axioms` output behavior |

Corresponding ledger claims are `some-true`. LexLean tests realize compatibility with those authorities but do not re-register Lean's guarantees as LexLean proofs.

The production profile adds `UOR-GNAF-1-DRAFT-2`, the UOR-GNAF normative draft
`uor-gnaf/1-draft.2` that §17.15 models, identified by its source revision and
the SHA-256 of the document; a copy is vendored at
`model/authorities/UOR-GNAF-v1-draft.2.md`, and an authority row that vendors
its source names the copy in `vendored` with `checksum = "sha256:<hex>"`,
which `cargo xtask validate-model` recomputes from the copy's bytes. Its ledger
claim is `some-true`, and the GN conformance IDs are `build` evidence that
LexLean realizes it.

### 27.5 Generated documents

`cargo xtask validate-model` MUST:

1. parse all model files with unknown-field rejection;
2. validate IDs, levels, authorities, errors, and cross-references;
3. generate `CONFORMANCE.md` and `ERRORS.md` in memory;
4. compare exact committed bytes;
5. scan feature scenarios and Rust test names;
6. reject missing, duplicate, or extra links;
7. run the honesty vocabulary checks;
8. run repository-specific audits.

`model-write` is the only normal path that rewrites generated documentation.

### 27.6 Specification links

`cargo xtask validate-spec-links` parses the §31 table and requires:

- every table ID appears exactly once;
- every `model/ids.toml` ID appears exactly once in the table;
- suite and statement match the model row;
- all IDs are `build`;
- every source section referenced by an ID exists.

The conformance table is therefore a normative requirement index while the model remains the source of the repository's implemented-claim register.

### 27.7 Gherkin subset

The inherited deliberately small parser is retained. Each suite file contains:

- a `Feature:` heading for humans;
- one tag line `@<ID> @build`;
- one `Scenario:` line;
- one or more `Given`, `When`, `Then`, `And`, or `But` steps.

No background, outline, examples table, pending step, or alternate tag order is accepted.

Each ID has exactly one scenario. The scenario's statement must equal its model statement after trimming.

### 27.8 Test naming

For ID `LX-01`, the exact Rust test function name is:

```rust
fn conformance_lx_01()
```

This formula applies to every ID. The conformance meta-gate rejects:

- a registered ID with no such test;
- more than one test claiming the same ID;
- a scenario with no ID;
- a test naming an unregistered ID;
- a test ignored by `#[ignore]`;
- a test hidden behind a disabled feature.

### 27.9 Falsifiable gates

Every new gate must have a committed falsifiability record in `VERIFICATION.md` describing:

- planted defect;
- command run;
- expected failure;
- observed diagnostic;
- removal of defect.

A gate that cannot be falsified because the relevant register is empty must report that it is armed by the first row, not pass silently as evidence.

### 27.10 Repository audits

The template audits are adapted as follows:

- `audit-deferral` scans all Rust, TOML, JSON, Markdown, Gherkin, `.lex.tex`, language, schema, example, and xtask source outside generated output and dependency caches;
- marker strings are assembled in pieces in the gate so it can scan its own source;
- mentions inside Markdown code spans or fenced code are allowed;
- `audit-errors` checks every public diagnostic against `model/errors.toml`;
- `audit-shipped` derives shipped crates from `publish = false`;
- `audit-generated` proves generated documents and schemas are current;
- `audit-language-closure` checks built-in lexicons and renderer tokens;
- `audit-no-unsafe` verifies the shipped crate forbids unsafe code;
- `audit-production` verifies that the production-eligibility analysis classifies every IR variant explicitly, with no default branch (§17.13).

---

## 28. Testing strategy

### 28.1 Test classes

The repository MUST contain:

1. unit tests for scanners, parsers, LSE/LRE, IR, hashing, path rules, and output parsers;
2. property tests for normalization, canonical serialization, alpha-safe scope handling, and formatter idempotence;
3. golden tests for canonical `.lex.tex`, `.lean`, `.tex`, maps, coverage, manifests, diagnostics, and normalized process output;
4. integration tests invoking the public API;
5. CLI tests for every command and exit code;
6. conformance tests for every §31 ID;
7. positive end-to-end Lean verification fixtures;
8. negative fixtures for every rejection class;
9. two-directory reproducibility tests;
10. gate-falsifiability tests.

### 28.2 Fixture layout

```text
tests/fixtures/<suite>/<id>/
├── project/
├── expected/
│   ├── command.json
│   ├── diagnostics.json
│   ├── artifacts.json
│   └── hashes.toml
└── case.toml
```

`case.toml` contains:

```toml
spec = "lexlean/test-case/1"
command = "check"
args = []
expected_exit = 0
expect_artifacts = true
```

A fixture may contain multiple invocations only when its conformance statement explicitly concerns sequence behavior, such as stale-lock detection. Such a fixture uses sorted `[[invocation]]` rows.

Expected process-dependent hashes are stored separately from platform-independent golden hashes.

### 28.3 Golden updates

Golden output changes require:

- an explicit `just golden-write`;
- a reviewed diff;
- a compiler-semantics ID change when language or backend semantics changed;
- updated conformance evidence when behavior changed.

Tests MUST NOT automatically accept new output.

### 28.4 Reproducibility test

The reproducibility gate copies one project into two distinct absolute directories, runs a clean `build` in each, and compares:

- generated Lean;
- canonical LaTeX;
- source maps after required normalization;
- coverage;
- lexicon closure;
- build manifest.

Verified attestations and `.olean` files are excluded because they are platform/process-bound. Their own internal hashes and normalization are tested separately.

### 28.5 Negative requirements

Tests MUST establish that LexLean rejects, at minimum:

- one unknown word;
- one unknown symbol;
- one unknown control;
- raw percent comments;
- raw Lean;
- a TeX macro;
- ambiguous lexical segmentation;
- ambiguous typed resolution;
- missing glossary entry;
- lexicon cycle;
- unsafe renderer control;
- forward document reference;
- recursive definition;
- missing proof;
- extra proof branch;
- unrestricted simplify;
- `native_decide`;
- a Lean elaboration failure;
- a `leanchecker` failure fixture;
- malformed axiom output;
- an axiom-policy excess;
- a path symlink;
- a stale lock;
- a toolchain mismatch;
- a configured limit overrun;
- a PDF executable hash mismatch;
- a language-1.2 construct under language 1.1;
- a `lexlean/semantic-module/1` module under language 1.2;
- a `lexlean/semantic-module/2` module under language 1.1;
- an unsupported language version;
- a malformed language version;
- a language-1.1 lock with the language-1.2 lock schema, and a language-1.2
  lock with the language-1.1 lock schema;
- a builtin glossary reference with another language's version, from 1.2 to
  1.1 and from 1.1 to 1.2;
- a lexicon package whose language differs from the project's;
- a recursive occurrence inside another document type (positivity);
- a non-uniform recursive occurrence;
- an uninhabited recursive cycle;
- a self-referential structure;
- a noncontiguous mutual group;
- a recursive occurrence with the wrong number of type arguments;
- a constructor applied to the wrong number of arguments;
- a forward reference outside a mutual group;
- an uninhabited cycle through a mutual group;
- a match on a value of one type with the constructors of another;
- a binder spelled like a name the generated Lean refers to;
- a lambda that omits a used capture;
- a lambda that declares an unused capture;
- an application with the wrong number of arguments;
- an application of a non-function value;
- a lambda whose type differs from the expected function type;
- an executable definition that returns a closure;
- a generic call without its type arguments;
- a polymorphically recursive call;
- a recursive call inside a lambda;
- an executable definition that calls a non-executable definition;
- an executable definition that stores a closure in an `Option`;
- an executable definition that returns a structure of closures;
- an executable definition that receives a structure of closures;
- a type parameter spelled like a built-in Lean type;
- the universe `Type` as an explicit type argument;
- a non-decreasing call between mutual definitions;
- a mutual definition with a missing case;
- a mutual call on a value that is not a smaller family binder;
- mutual definitions decreasing on different families;
- a cyclic well-founded measure;
- forged well-founded evidence;
- missing well-founded evidence;
- well-founded evidence that omits the match enclosing its call;
- a semantic-module member outside the closed schema;
- a mutual label shared by an inductive group and a definition group;
- a call to another member of a mutual group under a lambda;
- a well-founded definition referring to itself as a value;
- a mutual member that calls no member of its group;
- a mutual group whose call graph is not strongly connected;
- a mutual group mixing structural and well-founded members;
- a well-founded measure that mentions a member of its group;
- false well-founded evidence stating the exact obligation, refused by
  verification (`LLV7002`).
- a duplicate key in a map literal;
- a map keyed by a type without a canonical order;
- a map literal keyed by a non-literal value;
- a map literal key with a noncanonical spelling;
- a set literal element outside its fixed-width range;
- a graph edge to an undeclared node;
- a duplicate graph edge;
- a fold step whose type does not thread the state;
- an iteration without its fuel;
- a collection literal beyond `max_ir_nodes` (`LLS8002`);
- a collection under language 1.1.
- a production root on a target without allocation whose closure reaches allocation through two imported modules;
- a production root on a target without allocation that takes an unbounded list;
- a production root whose result is a proposition;
- a production root that takes a type universe;
- a production root that takes a named type holding a proposition in a field;
- a production root whose closure has an effect the root does not admit;
- a production root that takes a function;
- a production root whose closure binds a proposition-valued dependency;
- a production root with a literal outside the target width;
- a production root that is not declared executable;
- a production root with type parameters, whether or not its signature
  mentions them;
- a production root naming an unregistered target.
- a contract declaration under language 1.1;
- an `artifact_source` under language 1.1 (`LLC0101`);
- a model declaration with a prompt member outside the closed schema;
- a floating-point artifact element;
- an unregistered evidence claim kind;
- a missing model artifact, an artifact whose bytes differ from the configured
  digest, a declared artifact with no configured source, and an artifact
  whose declared length differs from its bytes (`LLR3007`);
- an artifact that is a symlink (`LLS8001`) and an artifact beyond
  `max_file_bytes` (`LLS8002`);
- artifact bytes too short for the declared tensor shape, a `utf8_lines`
  artifact that is not UTF-8, a role its schema does not admit, and a
  one-byte tensor with a 40000-deep shape, refused before any byte is
  decoded (`LLR3008`);
- a layer whose artifact has the wrong shape, a width theorem stating
  another width, a model binding a contract to a realization of another
  interface, a predicate with another signature, and a validator soundness
  theorem stating another obligation (`LLT4006`);
- a sequence junction left unconditional before a stage with a precondition,
  a junction theorem stating another obligation, a composite whose output
  type differs from its stages', and a model without its entry theorems
  (`LLT4007`);
- an executable application of a model without the check its precondition
  needs, without the postcondition check its evidence does not discharge, a
  check of a predicate without a validator, and a realization applied
  directly (`LLT4008`);
- a claim discharged by a theorem stating another obligation, an invariant
  claim about a stateless contract, and evidence about another realization
  (`LLT4009`);
- false evidence stating the exact claim, refused by verification
  (`LLV7002`).

### 28.6 Example verification

Every directory under `examples/` is discovered rather than listed. Each example MUST include its lock and expected platform-independent build outputs. `cargo xtask verify-examples` runs `fmt --check`, `lock --check`, `check`, `build`, and `verify`. The `compiler/` project (§17.14, §17.15) passes the same gates as an example.

---

## 29. Literal minimal example

### 29.1 Required standard Nat lexicon

`lexlean.std.nat@1.0.0` MUST provide at least:

| Entry | Category | Lean denotation | Canonical source |
|---|---|---|---|
| `nat` | `type-noun` | `Nat` | `natural number` / `natural numbers` |
| `zero` | `term-constant` | `Nat.zero` | `zero` and math `0` through numeral semantics |
| `succ` | `function` | `Nat.succ` | qualified/call form |
| `add` | `infix-function` | `Nat.add` | `+` |
| `addition` | `label-word` | a defined concept reference to `add` | `addition` |

It MUST include the Nat cases/induction descriptor.

### 29.2 Example files

`examples/nat-add-zero/lexlean.toml`:

```toml
spec = "lexlean/project/1"
name = "nat-add-zero"
language = "1.0"
module_prefix = "LexLeanExample"
source_roots = ["src"]
entrypoints = ["src/Main.lex.tex"]
build_root = ".lexlean"
lockfile = "lexlean.lock"
lean_workspace = "."
lean_toolchain = "leanprover/lean4:v4.32.1"

[[lexicon_source]]
package = "lexlean.std.nat"
kind = "builtin"

[limits]
max_file_bytes = 4194304
max_total_source_bytes = 67108864
max_primitive_atoms = 2000000
max_token_lattice_edges = 4000000
max_parse_states = 4000000
max_ir_nodes = 2000000
max_scope_depth = 1024
max_import_depth = 128
max_diagnostics = 256
max_child_output_bytes = 16777216
child_timeout_ms = 300000
```

`examples/nat-add-zero/lean-toolchain`:

```text
leanprover/lean4:v4.32.1
```

`examples/nat-add-zero/lakefile.toml`:

```toml
name = "nat_add_zero_host"
version = "0.1.0"
defaultTargets = ["NatAddZeroHost"]

[[lean_lib]]
name = "NatAddZeroHost"
```

`examples/nat-add-zero/NatAddZeroHost.lean`:

```lean
module
import Init
```

`examples/nat-add-zero/src/Main.lex.tex`:

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

`lexlean lock` generates and the repository commits the exact `lexlean.lock` for the built-in package and workspace files.

### 29.3 Expected generated Lean

The canonical generated module is exactly:

```lean
module
import Init
set_option autoImplicit false
namespace LexLeanExample.Main

theorem add_zero (llv0 : Nat) : Eq (Nat.add llv0 0) llv0 := by
  rfl

end LexLeanExample.Main
```

No prose or comments occur in the file.

### 29.4 Expected canonical LaTeX

The canonical `.tex` bytes are:

```latex
\documentclass[11pt]{article}
\usepackage[T1]{fontenc}
\usepackage{amsmath}
\usepackage{amssymb}
\usepackage{amsthm}
\usepackage[hidelinks]{hyperref}
\newtheorem{theorem}{Theorem}[section]
\newtheorem{lemma}[theorem]{Lemma}
\newtheorem{corollary}[theorem]{Corollary}
\theoremstyle{definition}
\newtheorem{definition}[theorem]{Definition}
\begin{document}
\begin{center}
{\LARGE Natural number addition}
\end{center}
\begin{theorem}
\label{ll:main:add-zero}
For every natural number \(n\), \(n + 0 = n\).
\end{theorem}
\begin{proof}
The goal follows by reflexivity.
\end{proof}
\end{document}
```

Every word, symbol, and control in this file has a coverage origin.

### 29.5 Expected verification

Verification MUST observe an empty axiom set for:

```text
LexLeanExample.Main.add_zero
```

The example's verified artifact is platform-bound. The committed example oracle contains platform-independent build artifacts and expected normalized verification records, not a cross-platform `.olean` oracle.

### 29.6 Required example mutations

The example suite MUST mechanically demonstrate:

1. changing `=` to a false proposition causes the old proof to fail in Lean;
2. replacing `addition` in the title with an undeclared word fails lexical closure;
3. adding a second same-surface proof entry with an indistinguishable signature causes ambiguity;
4. changing `\noaxioms` to an insufficient allow-list fails policy checking when a fixture introduces an axiom-dependent proof;
5. two clean builds in distinct paths are byte-identical for platform-independent artifacts.

---

## 30. Versioning, release, and completion

### 30.1 Version axes

LexLean has separate versions for:

- compiler crate/binary SemVer;
- language identifier `1.0`;
- project schema;
- lock schema;
- lexicon schema;
- entry schema;
- artifact schemas;
- compiler-semantics ID.

A compiler patch release may retain the semantics ID only when canonical accepted language, IR, diagnostics required for success/failure, and generated artifacts are unchanged.

### 30.2 Compatibility

Language 1.0 accepts only exact schema tags specified here. It does not silently interpret an unknown major or minor schema. A future compiler may support multiple versions through explicit dispatch, but a language-1.0 run remains byte-compatible.

No automatic in-place migration command is part of 1.0. Unsupported input fails with `LLC0103`.

### 30.3 Release artifacts

A `1.0.0` release MUST publish:

- source tag;
- checksums;
- supported-host binaries;
- crate package;
- compiler-semantics ID;
- generated `CONFORMANCE.md`;
- generated `ERRORS.md`;
- `SPEC.md`;
- license files;
- a software-bill-of-materials artifact;
- CI evidence that `just vv` passed on the tagged commit.

Release binaries MUST report:

```text
lexlean 1.0.0
language 1.0
compiler-semantics <digest>
lean-toolchain leanprover/lean4:v4.32.1
```

### 30.4 Completion criterion

The project is complete for LexLean 1.0 only when:

- every §31 ID is implemented at level `build`;
- no ID is represented by a stub or ignored test;
- all schemas are committed and exercised;
- built-in core, Nat, Int, and UOR Atlas lexicons validate;
- the literal example verifies;
- every negative fixture fails for the prescribed reason;
- every gate has falsifiability evidence;
- `just vv` passes from a clean checkout;
- the default branch has no `open` row used as a substitute for a capability;
- README claims are generated from or explicitly tied to model IDs;
- no normative behavior remains dependent on unspecified implementation choice.

### 30.5 Required implementation order

Each capability change follows R3. At repository scale, the dependency order is:

1. adapt template identity, rules, errors, and gates;
2. commit normative schemas and built-in language data;
3. implement normalization, scanning, package loading, and locking;
4. implement token lattice, structural grammar, expressions, and proposition grammar;
5. implement scope, resolution, conservative elaboration, linking, and IR;
6. implement definitions and proof IR;
7. implement canonical Lean, LaTeX, maps, coverage, and manifests;
8. implement toolchain preflight, external probes, Lean compilation, replay, and axiom audit;
9. implement CLI and stable Rust API;
10. complete examples, negative fixtures, reproducibility, and release gates.

The default branch must remain passing at each merge; a capability row is merged only with its complete scenario, test, implementation, and evidence.


## 31. Complete conformance-ID registry

Every row below is normative, has honesty level `build`, and MUST be copied byte-for-byte into `model/ids.toml` as its `statement`. The `suite` column names `features/suites/<suite>.feature`. The required Rust test name is `conformance_<id>` after lower-casing and replacing `-` with `_`.

| ID | Suite | Normative statement | Primary specification |
|---|---|---|---|
| `RP-01` | `repository` | The repository, crate, executable, metadata, and licenses have the exact LexLean identity specified. | §2 |
| `RP-02` | `repository` | The repository is derived from the pinned UOR template commit and contains no inherited domain-specific claim logic. | §2.2 |
| `RP-03` | `repository` | The completed repository has the required file and crate layout. | §7 |
| `RP-04` | `repository` | Only the lexlean crate is shipped and no shipped crate depends on repository-only tooling. | §7, §8.4 |
| `RP-05` | `repository` | The just vv recipe runs every normative acceptance gate in the specified order. | §9.2 |
| `RP-06` | `repository` | CONFORMANCE.md and ERRORS.md are exact generated views of model files. | §27.5 |
| `RP-07` | `repository` | The specification conformance table and model register are bijective and text-consistent. | §27.6 |
| `RP-08` | `repository` | Repository source contains no unsanctioned deferral marker, stub, placeholder, ignored capability, or disabling feature. | §27.10 |
| `RP-09` | `repository` | The shipped crate forbids unsafe Rust and the audit proves the prohibition is active. | §8.1, §27.10 |
| `RP-10` | `repository` | The embedded compiler-semantics ID equals a clean recomputation from normative language and schema inputs. | §21.2 |
| `RP-11` | `repository` | Every public README capability claim is tied to a registered model ID and honesty level. | §27 |
| `RP-12` | `repository` | A release is refused unless the complete release criterion and all required artifacts are satisfied. | §30 |
| `CF-01` | `configuration-lock` | Project configuration accepts exactly the project/1 schema and rejects unknown or missing fields. | §10.1 |
| `CF-02` | `configuration-lock` | Every operational limit is explicit, positive, parsed with checked arithmetic, and has no hidden default. | §10.2 |
| `CF-03` | `configuration-lock` | Configured paths resolve within the project under the specified UTF-8 and nonsymlink rules. | §10.1, §25.1 |
| `CF-04` | `configuration-lock` | Project discovery selects the nearest valid regular lexlean.toml and stops at the filesystem root. | §23.2 |
| `CF-05` | `configuration-lock` | Entrypoint, explicit-file, and all-module selections are mutually exclusive and canonicalized as specified. | §23.3 |
| `CF-06` | `configuration-lock` | Builtin, path, and exact-commit HTTPS Git lexicon sources obey their disjoint schemas. | §10.1 |
| `CF-07` | `configuration-lock` | The lock file is canonical, comment-free, sorted, generated, and exact-byte checkable. | §11 |
| `CF-08` | `configuration-lock` | The lock contains the complete exact transitive lexicon package closure including lexlean.core. | §11.3 |
| `CF-09` | `configuration-lock` | Package tree digests use the specified length-framed sorted-file algorithm and reject special files. | §11.5 |
| `CF-10` | `configuration-lock` | A changed config, package, workspace pin, or digest makes lock checking fail rather than silently refresh. | §11 |
| `CF-11` | `configuration-lock` | Check, build, format, and verify resolve only locked locally available dependencies. | §11.4 |
| `CF-12` | `configuration-lock` | Network package acquisition occurs only through lock --allow-network and only for an exact configured commit. | §11.4, §25.3 |
| `CF-13` | `configuration-lock` | The Lake workspace contains exactly one supported Lake configuration and the recorded workspace files match. | §10.4 |
| `CF-14` | `configuration-lock` | Language 1.0 accepts only leanprover/lean4:v4.32.1 for verification. | §8.2, §10.1 |
| `CF-15` | `configuration-lock` | Duplicate logical modules and case-folded path or module collisions are rejected. | §23.3 |
| `CF-16` | `configuration-lock` | Language 1.1 has a parallel exact builtin closure and rejects a language-1.0 lock or package without altering language-1.0 identities. | §10.1 |
| `CF-17` | `configuration-lock` | Language 1.2 declaration support accepts language 1.2 and rejects unsupported or malformed language versions. | §10.1 |
| `CF-18` | `configuration-lock` | Lockfile v2 schema migration validates 1.2 locks while preserving exact byte-stability for language 1.0 and 1.1 projects. | §11.1, §11.2 |
| `LX-01` | `lexical-closure` | Source decoding and line normalization enforce valid UTF-8, LF, final LF, and forbidden-scalar rules. | §12.1 |
| `LX-02` | `lexical-closure` | Non-NFC source is diagnosed and canonical formatting rewrites it without semantic change. | §12.1, §23.5 |
| `LX-03` | `lexical-closure` | Raw percent, comments, tabs, trailing spaces, and non-ASCII whitespace are rejected. | §12.1 |
| `LX-04` | `lexical-closure` | The primitive scanner recognizes exactly the specified atom classes and records exact spans. | §12.2 |
| `LX-05` | `lexical-closure` | Core braces, controls, punctuation, and grammar tokens receive glossary coverage rather than TeX trust. | §12.3 |
| `LX-06` | `lexical-closure` | An undeclared prose word is rejected with an exact unknown-atom diagnostic. | §14 |
| `LX-07` | `lexical-closure` | An undeclared symbol or control sequence is rejected with an exact unknown-atom diagnostic. | §14 |
| `LX-08` | `lexical-closure` | Lexical analysis builds all valid form edges without greedy import-order selection. | §14.1 |
| `LX-09` | `lexical-closure` | Every accepted non-whitespace source atom is covered exactly once in the selected path. | §6 I1, §14.1 |
| `LX-10` | `lexical-closure` | A local identifier is accepted only when introduced by a binder and every later occurrence resolves by scope. | §14.2 |
| `LX-11` | `lexical-closure` | TeX macro definition, expansion, file access, and execution controls are rejected even if a package declares them. | §12.4 |
| `LX-12` | `lexical-closure` | Qualified lexeme and document-reference controls select only existing closed entries or declarations. | §14.3 |
| `LX-13` | `lexical-closure` | Lexical or semantic ambiguity is rejected and no priority or heuristic chooses a candidate. | §14.4 |
| `LX-14` | `lexical-closure` | Canonical formatting chooses safe canonical forms and proves linked-IR preservation. | §23.5 |
| `GL-01` | `lexicon` | Lexicon packages obey the exact package layout, schema, ID-to-path rule, and exact imports. | §13.1 |
| `GL-02` | `lexicon` | Entry files obey the exact entry schema and category-specific field rules. | §13.2, §13.3 |
| `GL-03` | `lexicon` | Forms obey channel, feature, canonical-source, safety, and explicit-inflection requirements. | §13.5 |
| `GL-04` | `lexicon` | Every entry uses one fixed frame and packages cannot add grammar productions. | §13.4 |
| `GL-05` | `lexicon` | Denotations are exactly core, Lean, document, or acyclic defined values. | §13.6 |
| `GL-06` | `lexicon` | Every semantic entry has a valid canonical LSE signature with scoped binders and universes. | §13.7, §13.8 |
| `GL-07` | `lexicon` | Every canonical render uses valid LRE with complete slot use and no raw TeX. | §13.9 |
| `GL-08` | `lexicon` | Only the core renderer-token registry can authorize emitted LaTeX controls and glyphs. | §13.10 |
| `GL-09` | `lexicon` | Package import cycles and excessive import depth are rejected. | §13.11 |
| `GL-10` | `lexicon` | Defined-denotation cycles and document-definition cycles are rejected. | §13.6, §15.7 |
| `GL-11` | `lexicon` | A document denotation must resolve to an available declaration with a matching signature. | §13.6, §17.7 |
| `GL-12` | `lexicon` | Every used external Lean entry is checked by a generated interface probe during verification. | §18.8 |
| `GL-13` | `lexicon` | Duplicate packages, entries, forms, and qualified IDs are rejected while same-surface overloads remain explicit candidates. | §13.11, §14 |
| `GL-14` | `lexicon` | Cases and induction are available only through a complete validated eliminator descriptor. | §16.11 |
| `GL-15` | `lexicon` | Glossary files reject free description, documentation, note, meaning, and unknown prose fields. | §13.6 |
| `GL-16` | `lexicon` | Package and entry bytes participate in lock and semantic closure hashes exactly as specified. | §11, §21 |
| `GL-17` | `lexicon` | Language 1.2 resolves the exact 1.2 builtin package closure and enforces 1.2 lexicon semantics. | §13.1, §13.11 |
| `GL-18` | `lexicon` | Cross-version package, lexicon, and lock combinations fail closed before backend execution. | §10.1, §13.11 |
| `GR-01` | `grammar` | A source module parses only under the exact structural grammar and environment set. | §15.1, §15.2 |
| `GR-02` | `grammar` | Glossary imports, module imports, title, and blocks obey exact header order and cardinality. | §15.1 |
| `GR-03` | `grammar` | Sections nest within the configured scope limit and section parameters introduce explicit inherited context. | §15.1, §15.4 |
| `GR-04` | `grammar` | Titles and headings accept only bounded concept phrases and cannot encode an unproved proposition. | §15.3 |
| `GR-05` | `grammar` | Only parenthesized and display control delimiters create math islands; dollar math is rejected. | §15.5 |
| `GR-06` | `grammar` | Dynamic mathematical operators obey declared precedence, associativity, and explicit grouping. | §15.5 |
| `GR-07` | `grammar` | Mathematical juxtaposition is never interpreted as implicit multiplication or application. | §15.5 |
| `GR-08` | `grammar` | Universal, existential, unique-existential, conditional, and connective proposition forms have the specified compositional semantics. | §15.6 |
| `GR-09` | `grammar` | Proposition precedence and associativity produce the specified parse or an ambiguity error. | §15.6 |
| `GR-10` | `grammar` | Articles, plural forms, capitalization, and inflections are explicit lexicon data rather than inferred language rules. | §13.5, §15.6 |
| `GR-11` | `grammar` | A component with no complete grammar parse fails with a bounded structured diagnostic. | §6 I3, §26 |
| `GR-12` | `grammar` | Distinct surviving parses fail as ambiguity while semantically identical canonical IR alternatives collapse. | §14.4 |
| `GR-13` | `grammar` | Free expository paragraphs and opaque text nodes are rejected. | §4.2, §15 |
| `GR-14` | `grammar` | Definition and theorem-like components enforce exact sentence, policy, and proof cardinalities. | §15.7, §15.8 |
| `GR-15` | `grammar` | Explicit module imports form an acyclic graph and selected builds include their transitive closure. | §15.1 |
| `GR-16` | `grammar` | A same-module declaration cannot reference a later declaration. | §15.1, §17.7 |
| `SM-01` | `semantic-ir` | Compiler phases execute in the required order and no backend receives an unlinked or ambiguous program. | §17.1 |
| `SM-02` | `semantic-ir` | Every global and local reference has one closed typed reference variant and one stable identity. | §17.2 |
| `SM-03` | `semantic-ir` | Term IR contains only the specified closed variants and represents every accepted semantic term. | §17.3 |
| `SM-04` | `semantic-ir` | Proof IR contains only the specified closed variants and represents every accepted proof form. | §17.4 |
| `SM-05` | `semantic-ir` | Conservative signature elaboration checks arity, binders, categories, and expected types without claiming Lean kernel equivalence. | §17.6 |
| `SM-06` | `semantic-ir` | Omitted implicit binders are recorded as controlled application metadata and user holes are rejected. | §17.3, §17.6 |
| `SM-07` | `semantic-ir` | Document-entry signatures and generated declaration signatures are compared canonically before rendering. | §17.7 |
| `SM-08` | `semantic-ir` | Module, component, local, and hypothesis name generation is deterministic and collision checked. | §17.8 |
| `SM-09` | `semantic-ir` | Linked IR has stable schema-tagged canonical JSON with alpha-safe binder serialization. | §17.9 |
| `SM-10` | `semantic-ir` | Source ID and semantic ID use exactly the specified framed hash inputs. | §21.3, §21.4 |
| `SM-11` | `semantic-ir` | Linked project result sets contain every selected module and imported module exactly once in stable order. | §17.5, §23.3 |
| `SM-12` | `semantic-ir` | No semantic IR node can contain opaque prose, raw backend text, or an unknown extension. | §6 I4, §17 |
| `SM-13` | `semantic-ir` | Inherited section parameters are represented explicitly and emitted only on declarations that use them. | §17.5, §18.3 |
| `SM-14` | `semantic-ir` | A numeral without a unique expected type is rejected rather than defaulted. | §15.5 |
| `SM-15` | `semantic-ir` | A native core module is closed typed DAG data shared by both backends, carries explicit declaration policies, and accepts no backend source text. | §17.10 |
| `SM-16` | `semantic-ir` | The language-1.1 semantic snapshot contains every closed declaration, term, recursion, match, instance, proof variant, and exact theorem axiom policy without paths or backend text. | §17.11 |
| `SM-17` | `semantic-ir` | Language 1.1 has distinct mathematical Int, fixed-width signed and unsigned integer, UTF-8 string, byte-sequence, Option, and Result semantic types with canonical literals. | §17.11 |
| `SM-18` | `semantic-ir` | Portable arithmetic, conversion, bitwise, bounded-shift, collection, UTF-8, byte-order, split/join, and decimal operations form one closed typed primitive vocabulary. | §17.11 |
| `SM-19` | `semantic-ir` | Portable semantic operations generate deterministic Lean 4.32.1 definitions that elaborate and replay with exact declared computational axiom policies. | §17.11, §18, §22.6 |
| `SM-20` | `semantic-ir` | Noncanonical, out-of-range, invalid-byte, ill-typed, and unbounded fixed-width values fail before either backend runs. | §17.11 |
| `SM-21` | `semantic-ir` | Structural recursion admits byte/list values and closed Option and Result inductives while preserving termination and exhaustiveness checks. | §17.11 |
| `SM-22` | `semantic-ir` | The public owned snapshot DTO and schemas cover every portable type, literal, primitive, and explicit definition axiom policy without backend text. | §17.11, §21 |
| `SM-23` | `semantic-ir` | Language 1.2 semantic modules use the versioned module and snapshot schemas and accept the typed nonrecursive let term, which language 1.1 rejects before either backend runs. | §17.12 |
| `SM-24` | `semantic-ir` | Language 1.2 product types, pairs, projections, and product matches are typed, snapshotted under the v2 schemas, and give identical semantic IDs from distinct roots. | §17.12, §21 |
| `SM-25` | `semantic-ir` | Language 1.2 function types, lambdas with exact explicit captures, full applications, and definition references are typed, lowered to fixed Lean, and verified. | §17.12, §18 |
| `SM-26` | `semantic-ir` | Language 1.2 snapshots carry a deterministic alpha identity per definition that alpha-equivalent definitions share and any other change alters. | §17.12, §21 |
| `SM-27` | `semantic-ir` | Language 1.2 snapshots carry complete recursion evidence (mutual labels, decreasing arguments, measures, and evidence bindings), and changing an evidence binding changes the semantic and alpha identities. | §17.12, §21 |
| `SM-28` | `semantic-ir` | Language 1.2 finite maps and sets over closed ordered key types, their literals, and their primitive operations are typed, lowered to the fixed ordered-collection runtime, and verified. | §17.12 |
| `SM-29` | `semantic-ir` | Reordered equivalent map, set, and graph literals link to byte-identical semantic data and generated Lean, LaTeX, and lexicon-closure artifacts, while duplicate keys, non-literal literal keys, and key types without a canonical order are rejected. | §17.12, §21 |
| `SM-30` | `semantic-ir` | Language 1.2 graph literals reference only declared nodes, a graph's nodes are its keys and every successor, and successor, reachability, and topological-order queries are deterministic, bounded by the node count, and report a cycle as none. | §17.12 |
| `DF-01` | `declarations` | A valid type-definition sentence emits one nonrecursive sort-valued Lean def linked to its document entry. | §15.7, §18.6 |
| `DF-02` | `declarations` | A valid term-definition sentence emits one nonrecursive explicitly typed Lean def. | §15.7, §18.6 |
| `DF-03` | `declarations` | A valid predicate-definition sentence emits one nonrecursive Prop-valued Lean def. | §15.7, §18.6 |
| `DF-04` | `declarations` | Self recursion, mutual recursion, and later-declaration references are rejected. | §15.7 |
| `DF-05` | `declarations` | A definition's self head, explicit arguments, and signature order are checked exactly. | §15.7 |
| `DF-06` | `declarations` | Every generated definition and theorem-like declaration carries one explicit axiom policy. | §15.9, §22.6 |
| `DF-07` | `declarations` | Theorem, lemma, and corollary each emit Lean theorem declarations while retaining distinct document metadata. | §15.8, §18.6 |
| `DF-08` | `declarations` | Author-defined axioms, opaque declarations, and proofless theorem-like components are rejected. | §4.2, §15.8 |
| `DF-09` | `declarations` | Every theorem-like component contains exactly one nonempty structured proof. | §15.8, §16 |
| `DF-10` | `declarations` | Generated declarations preserve source order and every document reference respects that order. | §18.3 |
| `DF-11` | `declarations` | Language 1.1 checks and lowers generic structures, classes, instances, inductives, definitions, structural recursion, matches, Boolean validators, and closed proofs from semantic source data. | §17.11 |
| `DF-12` | `declarations` | Language 1.2 inductives admit uniform, strictly positive self, nested, and mutual recursion with a buildable base case, all checked before either backend runs. | §17.12 |
| `DF-13` | `declarations` | Language 1.2 structural recursion and induction over a recursive inductive use exactly its direct recursive fields, across modules, with one induction hypothesis per recursive field. | §17.12 |
| `DF-14` | `declarations` | Language 1.2 generic definitions and theorems take explicit type parameters, every use supplies exactly their type arguments, recursion is never polymorphic, and every written type mentions only declared parameters. | §17.12 |
| `DF-15` | `declarations` | An executable language-1.2 definition forms only non-escaping closures and calls only executable definitions; every violation fails before either backend runs. | §17.12 |
| `DF-16` | `declarations` | Language 1.2 mutual definition groups recurse structurally over one recursive family, including nested and mutual inductives, and every call between members passes a structurally smaller family binder. | §17.12 |
| `DF-17` | `declarations` | Language 1.2 well-founded definitions carry a binder-free natural-number measure and, per recursive call site, a prior theorem stating exactly that call's decrease obligation; linking checks the statements and Lean checks the proofs. | §17.12 |
| `DF-18` | `declarations` | Language 1.2 state threading is explicit: ordered folds and bounded iteration pass the state through a direct closure, executable definitions may use them, and every iteration carries a natural-number bound. | §17.12 |
| `PF-01` | `proofs` | Assume and exact-style simple proof sentences create scoped introductions and exact proof nodes. | §16.2 |
| `PF-02` | `proofs` | Simple Apply is accepted only when its declared signature yields exactly one residual premise. | §16.2 |
| `PF-03` | `proofs` | Structured apply requires every numbered residual premise exactly once and in signature order. | §16.6 |
| `PF-04` | `proofs` | Reflexivity lowers only to pinned Lean rfl and closes the current goal. | §16.2 |
| `PF-05` | `proofs` | Witness steps supply the next existential witness with no implicit search. | §16.2 |
| `PF-06` | `proofs` | Left and right alternative steps select only the corresponding disjunction constructor. | §16.2 |
| `PF-07` | `proofs` | Have establishes a nested proposition before introducing its fresh hypothesis into subsequent scope. | §16.3 |
| `PF-08` | `proofs` | Rewrite applies every explicitly directed rule strictly in source order at exactly one target. | §16.4 |
| `PF-09` | `proofs` | Simplify lowers to simp only with exactly the listed rules and target. | §16.5 |
| `PF-10` | `proofs` | Constructor requires the exact ordered branch count and every branch closes. | §16.7 |
| `PF-11` | `proofs` | Cases requires a validated descriptor, every constructor once, and exact branch binders. | §16.8 |
| `PF-12` | `proofs` | Induction requires a validated descriptor, every constructor once, and exact field and induction-hypothesis binders. | §16.9 |
| `PF-13` | `proofs` | Calculation chains use one declared relation, at least one step, and exact endpoint proofs. | §16.10 |
| `PF-14` | `proofs` | Proof locals, hypotheses, premise scopes, and case scopes cannot capture or leak. | §14.2, §16 |
| `PF-15` | `proofs` | Every proof and nested branch must close all goals and rejects steps after closure. | §16.1 |
| `PF-16` | `proofs` | Raw tactics, custom proof nodes, unrestricted automation, and proof holes are rejected. | §16.12 |
| `PF-17` | `proofs` | native_decide is never accepted or generated. | §16.12, §18.2 |
| `PF-18` | `proofs` | Lean proof failures remap to the smallest originating LexLean proof or statement span. | §20.4 |
| `PF-19` | `proofs` | The language-1.2 linear_arithmetic proof form names only the prior document definitions it unfolds, lowers to one fixed omega script, proves true linear obligations, and is refused by Lean on a false one. | §17.12, §16 |
| `LN-01` | `lean-backend` | Each generated Lean file has the exact module, import, option, namespace, declaration, and end structure. | §18.1 |
| `LN-02` | `lean-backend` | Imports are explicit, deduplicated, sorted, and every external global is fully qualified. | §18.3 |
| `LN-03` | `lean-backend` | Generated Lean contains no comments, documentation, prose-bearing strings, or copied source prose. | §18.2 |
| `LN-04` | `lean-backend` | Generated Lean contains no sorry, admit, axiom, opaque, unsafe, native_decide, or placeholder declaration. | §18.2 |
| `LN-05` | `lean-backend` | Every linked term and proof variant has one defined Lean lowering and missing lowering is a hard error. | §18.4, §18.7 |
| `LN-06` | `lean-backend` | Leading universal binders become deterministic declaration parameters with complete source mapping. | §18.5 |
| `LN-07` | `lean-backend` | All document definitions emit def and never alternate declaration forms. | §18.6 |
| `LN-08` | `lean-backend` | Proof lowering uses only the fixed pinned Lean forms enumerated by the specification. | §18.7 |
| `LN-09` | `lean-backend` | Lean formatting is byte-deterministic with fixed indentation, LF, and final LF. | §18 |
| `LN-10` | `lean-backend` | Every non-whitespace generated Lean token has a source, glossary, IR, or synthetic-core mapping. | §20.3 |
| `LN-11` | `lean-backend` | The generated-source audit tokenizes and rejects prose-bearing or forbidden Lean tokens before verification. | §18.2 |
| `LN-12` | `lean-backend` | Generated file paths and module names exactly mirror the configured module prefix and source module. | §15.1, §18.1 |
| `TX-01` | `latex-pdf` | Canonical LaTeX is rendered solely from linked IR and never copies source text or controls. | §19.1 |
| `TX-02` | `latex-pdf` | Every module uses the exact canonical LaTeX preamble and no host or timestamp metadata. | §19.2 |
| `TX-03` | `latex-pdf` | Statements use only canonical controlled proposition and definition renderings. | §19.3, §19.4 |
| `TX-04` | `latex-pdf` | Proof prose is generated from proof IR using fixed core lexical forms. | §19.5 |
| `TX-05` | `latex-pdf` | Titles, sections, parameters, environments, numbering, and labels follow the exact document rules. | §19.3 |
| `TX-06` | `latex-pdf` | Every visible LaTeX word, symbol, punctuation mark, and control has complete coverage. | §19.6 |
| `TX-07` | `latex-pdf` | Non-core lexicons cannot inject a raw TeX control or unclassified output segment. | §13.9, §19.6 |
| `TX-08` | `latex-pdf` | Canonical LaTeX bytes are deterministic, LF-normalized, and final-LF terminated. | §19 |
| `TX-09` | `latex-pdf` | An enabled external PDF provider runs without a shell in an isolated directory with exact hashes and resources. | §19.7 |
| `TX-10` | `latex-pdf` | The PDF recipe ID and actual PDF hash use the specified independent content records. | §19.8 |
| `TX-11` | `latex-pdf` | PDF success or failure never changes mathematical verification authority. | §19.7 |
| `TX-12` | `latex-pdf` | The publishable document is the canonical renderer output rather than unchecked source bytes. | §6 I8, §19 |
| `AR-01` | `artifacts` | Diagnostics use the canonical schema, exact spans, stable sorting, and registered codes. | §20.1, §26 |
| `AR-02` | `artifacts` | Source maps contain complete module, source, artifact, node, and range records. | §20.3 |
| `AR-03` | `artifacts` | Lean diagnostics remap by the specified smallest-enclosing mapping algorithm. | §20.4 |
| `AR-04` | `artifacts` | Coverage files record every required source and output token with no gap or overlap. | §20.5 |
| `AR-05` | `artifacts` | All compound hashes use the specified length-prefixed frame function. | §21.1 |
| `AR-06` | `artifacts` | Compiler-semantics identity is recomputed from the exact normative language, schema, backend, and parser inputs. | §21.2 |
| `AR-07` | `artifacts` | Source IDs are independent of absolute checkout location and include exact normalized inputs. | §21.3 |
| `AR-08` | `artifacts` | Semantic IDs are platform independent and include linked IR, lexicon closure, language semantics, and toolchain ID. | §21.4 |
| `AR-09` | `artifacts` | Successful builds publish only the fixed content-addressed build-ID layout. | §21.5 |
| `AR-10` | `artifacts` | Build manifests enumerate every input and output with stable paths, sizes, and hashes. | §21.6 |
| `AR-11` | `artifacts` | Normative JSON obeys the restricted canonical JSON format and hash/file newline distinction. | §21.7 |
| `AR-12` | `artifacts` | Concurrent and failed builds preserve atomic content-addressed artifacts and never overwrite unexplained bytes. | §21.8 |
| `AR-13` | `artifacts` | Two clean builds in different absolute directories produce byte-identical platform-independent artifacts. | §28.4 |
| `AR-14` | `artifacts` | Platform-independent build evidence is distinguished from platform-bound oleans, process records, PDF bytes, and attestations. | §21, §22 |
| `VR-01` | `verification` | Verification runs every specified stage in order and exposes no stage-suppression option. | §22.1 |
| `VR-02` | `verification` | Lean, Lake, and leanchecker versions and executable hashes are checked and recorded before use. | §22.2 |
| `VR-03` | `verification` | Lean execution uses the pinned Lake environment and never updates or fetches workspace dependencies. | §22.2 |
| `VR-04` | `verification` | Every used external interface is elaborated in the unique reserved probe module. | §18.8, §22.1 |
| `VR-05` | `verification` | Generated modules compile in topological order and produce one required olean each. | §22.3 |
| `VR-06` | `verification` | Verification neither requests nor includes ilean artifacts. | §22.3 |
| `VR-07` | `verification` | A Lean warning, unknown informational message, overflow, or missing output fails verification. | §20.2, §22.3 |
| `VR-08` | `verification` | Every generated module is replayed by a separate leanchecker process and every replay must succeed. | §22.4 |
| `VR-09` | `verification` | The reserved audit module family audits one generated module per process and prints axioms for every declaration in its native Lean environment exactly once. | §18.9 |
| `VR-10` | `verification` | The axiom parser accepts only the pinned exact output forms and rejects missing, duplicate, extra, or malformed records. | §22.5 |
| `VR-11` | `verification` | None, allow-subset, and exact axiom policies are enforced exactly and recorded per declaration. | §22.6 |
| `VR-12` | `verification` | Child process output is normalized with the exact path and line rules before hashing. | §22.7 |
| `VR-13` | `verification` | A verified directory contains the complete fixed source, map, coverage, olean, probe, audit, and process artifact set, and, exactly when a production root exists, the eligibility reports, the extraction module, its process record, and the compiler input. | §22.8 |
| `VR-14` | `verification` | The attestation ID is computed over the canonical body with its ID field removed. | §22.9 |
| `VR-15` | `verification` | Any failed verification stage removes staging and produces no verified artifact or verified status. | §6 I11, §22 |
| `VR-16` | `verification` | Axioms flowing from imported theorems remain subject to the generated declaration's policy. | §22.6 |
| `VR-17` | `verification` | Lean workspace configuration and manifest hashes must match the lock and all dependencies must be locally available. | §10.4, §22.2 |
| `VR-18` | `verification` | Check and build results never claim verified or kernel-checked status. | §5.3 |
| `VR-19` | `verification` | The native Atlas source graph is self-contained: every generated Atlas module publicly depends only on Init and the generated graph, its only backend-support import is Lean, and no independently authored Atlas implementation exists. | §10.4, §17.10, §22.2 |
| `CL-01` | `cli-api` | Global options and upward project discovery obey the exact CLI contract. | §23.1, §23.2 |
| `CL-02` | `cli-api` | Init creates the complete canonical skeleton only in an absent or empty destination and never overwrites. | §23.4 |
| `CL-03` | `cli-api` | Lock check, local update, and explicit network acquisition obey their exact mutually exclusive behavior. | §23.4 |
| `CL-04` | `cli-api` | Check runs through linked IR and emits no build artifacts. | §23.4 |
| `CL-05` | `cli-api` | Build emits the fixed build-ID artifact set without running Lean or claiming verification. | §23.4 |
| `CL-06` | `cli-api` | Verify runs the complete fixed verification pipeline and accepts no output or suppression option. | §23.4 |
| `CL-07` | `cli-api` | Format and format-check are idempotent and preserve linked IR. | §23.4, §23.5 |
| `CL-08` | `cli-api` | Clean removes only the validated configured build root and no source or external cache. | §23.4 |
| `CL-09` | `cli-api` | Explain prints exactly one registered diagnostic entry and rejects unknown codes. | §23.4 |
| `CL-10` | `cli-api` | All, explicit-files, and entrypoint selections return sorted project result sets including import closure. | §23.3 |
| `CL-11` | `cli-api` | Every command maps failures to the exact documented exit code. | §23.6 |
| `CL-12` | `cli-api` | Human and canonical-JSON output modes obey exact stream, color, and cardinality rules. | §23.7 |
| `CL-13` | `cli-api` | The public Engine exposes exactly the stable load, lock, check, snapshot, build, verify, and format entry points. | §24.1 |
| `CL-14` | `cli-api` | Every public multi-module operation returns a ProjectResultSet or VerifiedProject rather than a singular unit. | §24.2, §24.4 |
| `CL-15` | `cli-api` | Public requests cannot override backends, toolchain, verification stages, limits, policies, or fixed artifact sets. | §24.3 |
| `CL-16` | `cli-api` | Every public failure is a LexLeanError and malformed user input cannot panic. | §24.5 |
| `CL-17` | `cli-api` | Environment variables cannot alter semantic project configuration. | §23.1, §25.4 |
| `CL-18` | `cli-api` | Version output reports compiler, language, semantics ID, and Lean toolchain exactly. | §30.3 |
| `CL-19` | `cli-api` | Snapshot returns a stable owned canonical semantic envelope without writing artifacts or invoking a backend. | §24.1 |
| `CL-20` | `cli-api` | Language-1.1 init creates and verifies a declarative Lake workspace containing no source Lean module. | §23.4 |
| `CL-21` | `cli-api` | Native diagnostics distinguish package-import cycles and unqualified cross-package term ambiguities without changing diagnostic wire bytes. | §24.5 |
| `SE-01` | `security` | Source, package, workspace, resource, and output paths are confined and symlinks are rejected. | §25.1 |
| `SE-02` | `security` | Special files, duplicate filesystem identities, and case-fold collisions are rejected before processing. | §25.1 |
| `SE-03` | `security` | All child processes use direct executable and argv invocation with no shell. | §25.2 |
| `SE-04` | `security` | No command except lock --allow-network may perform package network acquisition. | §25.3 |
| `SE-05` | `security` | Child environments use the specified deterministic allow-list and recorded normalization. | §25.4 |
| `SE-06` | `security` | Every configured parser, graph, IR, diagnostic, child-output, and timeout limit is enforced with checked arithmetic. | §25.5 |
| `SE-07` | `security` | Temporary data uses confined owner-only staging and is removed after atomic publication or failure. | §25.6 |
| `SE-08` | `security` | External executables and PDF resources are hash-checked before use. | §19.7, §22.2 |
| `SE-09` | `security` | PDF execution receives only canonical TeX and declared resources in an isolated working directory. | §19.7 |
| `SE-10` | `security` | Internal invariant failures use LLI9001 and exit 70 without misclassifying user input. | §25.7 |
| `SE-11` | `security` | Diagnostics and process records do not expose secret environment values or arbitrary unrelated file contents. | §25.6 |
| `SE-12` | `security` | Git lexicon acquisition accepts only an exact 40-hex commit over HTTPS and rejects submodules and LFS indirection. | §10.1, §11.4 |
| `EX-01` | `examples` | The committed nat-add-zero example formats, locks, checks, builds, and verifies with an empty axiom set. | §29 |
| `EX-02` | `examples` | Changing the example proposition while retaining the old proof causes remapped Lean verification failure. | §29.6 |
| `EX-03` | `examples` | Replacing a title concept with an undeclared word causes lexical-closure failure. | §29.6 |
| `EX-04` | `examples` | Adding an indistinguishable same-surface entry causes ambiguity rather than priority selection. | §29.6 |
| `EX-05` | `examples` | An axiom-dependent fixture fails an insufficient declaration policy and records the observed excess. | §29.6 |
| `EX-06` | `examples` | Two clean example builds in distinct paths have byte-identical platform-independent artifacts. | §29.6 |
| `EX-07` | `examples` | The negative fixture suite covers every required rejection class and prescribed diagnostic family. | §28.5 |
| `EX-08` | `examples` | Every example directory is discovered automatically and must satisfy the full example gate. | §28.6 |
| `PD-01` | `production` | The closed production registry fixes the language-1.2 targets, effects, and the disposition of every semantic construct kind, and every construct the eligibility analysis can classify has exactly one registry row. | §17.13 |
| `PD-02` | `production` | Formal-only theorems, propositions, and non-executable definitions coexist with eligible executable production roots, and a module that declares no production root is never analysed for production. | §17.13 |
| `PD-03` | `production` | A production root's runtime closure contains exactly its transitive computational dependencies across modules at their type instantiations, and its termination evidence is recorded as erased and never realized. | §17.13 |
| `PD-04` | `production` | Production eligibility depends on the declared target: a construct that requires heap allocation is an admitted effect on a target with allocation and makes the root ineligible on a target without it. | §17.13 |
| `PD-05` | `production` | A production root fails with LLT4005 before any backend runs when it declares type parameters or is not declared executable, when its boundary holds a universe, proposition, type parameter, or function, directly or in a named type's fields, or when its closure reaches a formal-only construct, a literal outside the target width, unavailable allocation, or an effect the root does not admit. | §17.13, §26.3 |
| `PD-06` | `production` | Every production root's eligibility report is a deterministic, schema-valid build artifact recording its runtime closure, realized types, erased dependencies, constructs, and per-target effects with their sources. | §17.13, §21.5 |
| `PD-07` | `production` | The eligibility analysis classifies every semantic construct by an explicit exhaustive match, and the exhaustiveness audit rejects a planted wildcard arm, rest pattern, implicit-default binding form, or unnamed IR variant. | §17.13, §27.10 |
| `NE-01` | `extraction` | Verifying a project with production roots extracts every root through Lean's compiler front end into one canonical compiler input whose bytes and ID are schema-valid, recorded in the attestation, and identical from distinct project directories, and a project without a production root publishes none. | §22.10, §26.3 |
| `NE-02` | `extraction` | Each root's extracted closure is exactly its computational dependencies, equals its production-eligibility closure, carries its runtime members and monomorphization instances, names every constant its code uses, marks recursion from the use graph, and records proof-only dependencies as erased and never as runtime members. | §22.10, §26.3 |
| `NE-03` | `extraction` | An unknown root, an opaque, axiomatic, unsafe, partial, or noncomputable dependency, an external implementation, an unresolved external, an unsupported compiler form, and a malformed, noisy, or foreign extraction record fail closed with LLV7011. | §22.10, §26.3 |
| `NE-04` | `extraction` | Every constant the extraction adapter uses is registered exactly once, as a call with its exact signature and pinned source identity, a type with its exact constructors, or plumbing; each extraction compares every signature structurally and the adapter's constants with the registry under pinned Lean, the adapter runs no LCNF pass, and drift of a signature, a constructor list, the adapter's constants, its output, or the Lean identity fails with LLV7012. | §22.10, §26.3 |
| `NE-05` | `extraction` | A dependency dropped from Lean's extracted facts or from the production-eligibility closure fails extraction with LLV7011 before any compiler input is published. | §22.10, §26.3 |
| `NE-06` | `extraction` | A proof-only dependency presented as a runtime closure member fails extraction with LLV7011 before any compiler input is published. | §22.10, §26.3 |
| `TC-01` | `calculus` | Hand-constructed target programs have canonical bytes and a SHA-256 content identity, alpha-equivalent programs canonicalize to identical bytes and identity, an ill-typed program has neither, and every committed fixture validates against the target schemas. | §17.14, §21.7 |
| `TC-02` | `calculus` | Every malformed target program or violated static rule fails closed with LLB6005, and an invalid program has neither a canonical form nor a rendering. | §17.14, §26.3 |
| `TC-03` | `calculus` | The calculus denotation is a kernel-checked LexLean definition that charges every evaluator operation, Lean's kernel reduces every kernel-reducible fixture to its expected outcome with its exact step count, no fixture program is stuck on its stated arguments or on seeded random well-typed arguments at any sampled fuel, and a wrong expected outcome is rejected by Lean. | §17.14 |
| `TC-04` | `calculus` | Lean's evaluator, running the published calculus sources compiled again by pinned Lean, reproduces every fixture's expected outcome and step count, including fixtures a reflexivity proof cannot decide, and the comparison refuses a fixture stated one step off. | §17.14 |
| `TC-05` | `calculus` | Every realization library template has a fixture whose outcome the kernel proves equal to the value LexLean's own collection primitive computes, committed instances equal their templates, and a mutated template is rejected by Lean. | §17.12, §17.14 |
| `TC-06` | `calculus` | Every runtime construct of the production registry has exactly one realization row naming existing calculus elements and requires allocation exactly when its realization does, and the fixtures exercise every calculus type, literal, expression, shape, primitive, and template, and every fixed-width primitive at every width it admits. | §17.13, §17.14 |
| `TC-07` | `calculus` | Every fixture with an observable outcome renders to a safe Rust library crate in rust-std, and in rust-core exactly when it needs no heap, that the pinned rustc compiles with warnings denied, that prints exactly the denotation's value or overflow, and whose counted work never exceeds the denotation's steps; planted value and work discrepancies are detected. | §17.14 |
| `RB-01` | `rust-backend` | Every Rust rendering is built as a closed AST each of whose constructs carries the calculus element it realizes, every correspondence row and every AST node is exercised by a run package, and a construct whose row, width, or program does not justify its element is refused. | §17.16 |
| `RB-02` | `rust-backend` | An exported name that is not a lowercase snake-case identifier, is a Rust keyword, imitates a generated name, or is declared by the runtime, a name exported twice, and an unavailable crate name each fail with LLB6005, and a crate binding one name twice in a function is refused. | §17.16, §26.3 |
| `RB-03` | `rust-backend` | An export that copies a parameter whose type is not Copy fails with LLB6005, and a crate that moves a value twice or reads it after moving it is refused. | §17.16, §26.3 |
| `RB-04` | `rust-backend` | A package whose boundary holds a function value at any depth, whose program computes a value of a type no value inhabits, or whose rust-core program needs the heap fails with LLB6005, and a rust-core crate naming any heap type, runtime function, or construct is refused. | §17.13, §17.16 |
| `RB-05` | `rust-backend` | A function that can overflow returns R<T> and every call to it propagates, any other returns its value, an export whose declared errors differ from its function's fails with LLB6005, and a crate that drops, invents, or misreturns a failure is refused. | §17.16 |
| `RB-06` | `rust-backend` | Every committed package builds offline under its declared gates, rustc warnings and Clippy's default lints denied with ten documented exceptions, the export of each one with an observable outcome, called from a separate crate, prints exactly the denotation's observable outcome, and so does every primitive instance on its boundary and seeded inputs; a planted lint and a planted semantic mutation are detected. | §17.16 |
| `RB-07` | `rust-backend` | Packages are deterministic and content-addressed: renderings by two separate processes under different roots and environments are byte-identical to each other and to the committed package, every manifest and provenance validates against its schema, the provenance binds the SHA-256 of each file, the program identity, the runtime LexLean's semantics records, the sources, and the language-1.2 compiler-semantics ID, and every committed package's sources are the semantic ID of the verified build stating its program. | §17.16, §21.7 |
| `GN-01` | `gnaf` | Every committed GNAF request and fixture is canonical, validates against the GNAF schemas whose calculus definitions are the target program schema's, and states its components' universe identity; a malformed request, an invalid reference, plan, or realized program, a reference without a value on a domain argument, or a wrong universe identity fails closed with LLB6006, and a machine or request beyond the host's capacity fails closed with LLS8002. | §17.15, §26.3 |
| `GN-02` | `gnaf` | The GNAF model is a kernel-checked LexLean definition, Lean's kernel reduces the universe, the system statuses, and the answer of every committed request to what the host transcription computes, and a wrong answer, an answer computed from a universe with a system omitted, a tie with a member dropped, and a universe with a system omitted are rejected by Lean. | §17.15 |
| `GN-03` | `gnaf` | Candidate membership is the grammar's expansion, which a kernel-checked theorem proves equal to the grammar's well-formed selectors for every grammar, independent of evaluation and fixed by a universe identity before any optimizer, and optimizer-defined, discovered, cached, and internal-plan universes and missing, self-referential, or optimizer-citing completeness evidence are rejected. | §17.15 |
| `GN-04` | `gnaf` | Every action a system performs is charged by steps, every admitted preparation action by a positive constant for each plan that needs it or by a prepared artifact bound in the common initial state, operands are charged by weight, and a request fits the capacity its machine binds; hidden zero-cost, free, undeclared, duplicated, unaccounted, unbound, stray, unit-cost, over-capacity, and unrealizable actions are rejected, and a plan's preparation enters exactly the cost of the systems that can run it. | §17.15 |
| `GN-05` | `gnaf` | Scalar claims require the total step order and Pareto claims the componentwise steps-and-size order, a scalar claim over the partial order, a vector claim over the total order, the restricted-universe alias, an undecided claim class, and a scope beyond the grammar universe are rejected, equal-cost systems are all kept, a frontier answer has incomparable members, and GNAF-VEC-02 posed over the calculus has a frontier of exactly three of four systems for which no omission is certified and whose componentwise minima no system attains. | §17.15 |
| `GN-06` | `gnaf` | Complete-system cost includes selection and counts only the code a system can run, the system argmin differs from the best internal plan and from the per-input plan envelope that no system attains, an inadmissible system is excluded, and an unresolved system makes the answer incomplete rather than being removed. | §17.15 |
| `GN-07` | `gnaf` | The authority's GNAF-VEC-01, GNAF-VEC-02, GNAF-VEC-04, GNAF-VEC-17, GNAF-REJ-14, and GNAF-REJ-29 vectors are kernel-checked theorems over the argmin and frontier the answers are computed by, stated with the authority's numbers, and the authority is vendored with a recomputed SHA-256, cited by revision, and claimed some-true. | §17.15, §27.4 |
| `GN-08` | `gnaf` | The GNAF dependency manifest names the authority's revision and SHA-256, every kind, operation, machine, cost, proof, interchange, and address profile with its role, every restriction of the admitted universe, and exactly the claim classes the model answers, and equals its generator. | §17.15, §27.4 |
| `MD-01` | `models` | Language 1.2 artifact, contract, realization, evidence, and model declarations, the checked_apply term, the contract_violation type, and the less_than primitive belong to the closed lexlean/semantic-module/2 schema and its snapshot schema, are rejected under language 1.1 before either backend runs, and admit no member outside the closed schema, such as prompt text, a free-form description, or raw model configuration. | §17.12 |
| `MD-02` | `models` | A model artifact is admitted only from a configured, content-addressed, confined project file whose SHA-256 and byte length equal its declaration and whose bytes decode under its declared closed schema to its declared type and role; a missing artifact or a digest or length mismatch fails with LLR3007, a schema, type, or role violation fails with LLR3008, and the generated Lean embeds the exact bytes with a kernel-checked theorem that the typed value is their decoding. | §17.12, §10.1, §26.3 |
| `MD-03` | `models` | A contract names typed input, output, and optional state interfaces and prior proposition-valued predicates of exactly those interface signatures, and each runtime validator is an executable Boolean definition of the same signature linked to its predicate by a statement-exact soundness theorem and an optional completeness theorem that Lean restates against the fixed model semantics; any mismatch fails with LLT4006. | §17.12 |
| `MD-04` | `models` | Deterministic, rule, statistical, and neural realizations each elaborate to one ordinary language-1.2 denotation over their declared interface, with exact integer kernels over the less_than primitive, and their layer widths, artifact shapes and roles, labels, and statement-exact width evidence are checked in linking; a descriptor outside the closed set, such as a floating-point element, is rejected, and a width, shape, or role mismatch fails with LLT4006. | §17.12 |
| `MD-05` | `models` | A model binds one contract instantiation to a realization of the identical interface, with entry evidence for every effective precondition; an interface mismatch fails with LLT4006 before either backend runs, and a realization whose behavior violates a statement-exact contract claim is refused by Lean's kernel at verification. | §17.12, §22.6 |
| `MD-06` | `models` | Every evidence claim is a kind of the closed claim set whose statement LexLean generates from the contract and realization, is discharged by a prior theorem stating it exactly, and is restated against the fixed Lean model semantics; an unregistered, vacuous, inexact, or foreign claim fails with LLT4009, the canonical document says only that a claim is discharged, and only the verified attestation records the generated declarations as verified. | §17.12, §22.9 |
| `MD-07` | `models` | Every model application in executable code validates each contract predicate its evidence does not discharge, through sound validators, in the fixed order input invariant, precondition, output invariant, postcondition, returning the contract violation on refusal; an application missing a required check, naming a check without a sound validator, or applying a realization directly fails with LLT4008. | §17.12 |
| `MD-08` | `models` | Sequence, fan-out, product, branch, and scan composites of stateless and stateful models are ordinary typed compositions whose stage interfaces and threaded states match exactly, whose every stage precondition and state invariant is discharged at the model's entry, by a statement-exact junction theorem, or by a run-time check, and whose every check a stage's evidence leaves open after it runs is made at run time; any other composition fails with LLT4007. | §17.12 |
| `MD-09` | `models` | The exact integer kernels of dense, rectified-linear, requantization, and first-maximum layers and the less_than primitive agree with an independent integer model on seeded random weights and inputs under Lean. | §17.12 |
| `MD-10` | `models` | Model declarations are part of the semantic identity, so changing an artifact byte with its declared digest, a descriptor, a claim, or a check changes the semantic ID, and language-1.2 snapshots carry, schema-valid, every elaborated declaration, generated obligation, cross-check, and required check. | §17.12, §21.4 |
| `MD-11` | `models` | Model constructs have production dispositions under which artifacts, realizations, models, validators, and checked applications are realized through their elaborations while contracts and evidence are erased, the realization table covers every new runtime construct, and production roots applying an artifact-backed model, directly and through its checks, are eligible and extract the same closure through Lean. | §17.13, §17.14, §22.10 |
| `MD-12` | `models` | The committed models example verifies nontrivial deterministic stateful, rule, statistical, artifact-backed neural, and composite models with every claim kernel-checked, and planting a contract and realization mismatch in it is refused by verification. | §17.12, §28.6 |

**Total required capability IDs:** 290.

No row may be downgraded to `some-true` or `open`. Upstream Lean facts are ledger/authority rows, not substitutions for these build behaviors.

---

## 32. Final acceptance statement

A repository conforming to this specification has no semantically untracked prose path in a LexLean document:

- source tokens are closed;
- grammar is closed;
- denotations are closed;
- proof forms are closed;
- generated Lean is prose-free;
- generated LaTeX is coverage-complete;
- generation is deterministic;
- verification is explicit and kernel-backed;
- axiom dependencies are per-declaration and machine-audited;
- every public capability and failure is registered and falsifiable.

Accordingly, a change to the mathematical meaning changes the linked semantic object, the generated Lean statement, the canonical human document, or all three. A proof that no longer establishes the changed statement fails verification. There is no independent prose channel that can silently remain stale.
