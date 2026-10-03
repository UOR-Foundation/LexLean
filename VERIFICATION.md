# VERIFICATION

How this repository's claims are checked, which recipe enforces which rule, and the evidence that each gate can actually fail. A gate nobody has seen fail is indistinguishable from a gate that cannot (SPEC.md §27.9).

## The acceptance gate

`just vv` runs, in the normative order (SPEC.md §9.2):

| Recipe | Command | Rules it enforces |
| --- | --- | --- |
| `fmt-check` | `cargo fmt --all -- --check` | one canonical source formatting |
| `model` | `cargo xtask validate-model` | R1 (model is the single source; every model file parsed with unknown-field rejection), R2 (honesty levels and vocabulary, via the meta-gate), R3 (register/scenario/test bijection, Gherkin subset), R4 (`audit-deferral`), R5 (`audit-errors`), R6 (`audit-shipped`, including the shipped crate's normative links, and every vendored authority's SHA-256 recomputed from its copy), R8 (`audit-generated`, `audit-language-closure`), RP-09 (`audit-no-unsafe`), PD-07 (`audit-production`), §27.5 (CONFORMANCE.md and ERRORS.md equal regeneration) |
| `spec-links` | `cargo xtask validate-spec-links` | RP-07, §27.6: the §31 table and `model/ids.toml` are bijective and byte-consistent |
| `lint` | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | no tolerated warnings |
| `test` | `cargo test --workspace --all-features` | §28.1 classes 1–2 and 4–5 (unit, property, integration, CLI), the model crate's own tests, and all 299 conformance tests, which include the §28.2 fixture suite (`conformance_ex_07`) and the crate-packaging round trip (`conformance_rp_12`) |
| `features` | `cargo check --workspace --all-features --all-targets` | every target compiles |
| `bdd` | `cargo test -p repo-conformance` | R3, §27.7, §27.8: register ↔ scenario ↔ test bijection, the meta-gate, and its own falsifiability test |
| `examples` | `cargo xtask verify-examples` | §28.6, EX-01: every example directory and the `compiler` project (§17.14, §17.15) format, lock, check, build, and verify with real Lean 4.32.1; when an example commits `expected/verify/`, its normalized verification records must equal it (§29.5) |
| `golden` | `cargo xtask check-golden` | R10, §28.3: the *published* build tree of a real `build` in a fresh directory equals the committed oracles byte for byte |
| `repro` | `cargo xtask check-reproducibility` | AR-13, §28.4: two clean `build`s in distinct absolute directories publish byte-identical trees with no absolute path inside |
| `deny` | `cargo deny --all-features check` | advisories, bans, licenses, sources |

Outside `vv`:

- `just fixtures` (`cargo xtask check-fixtures`) runs every §28.2 fixture under `tests/fixtures/` and `tests/negative/` through the CLI entry point and compares exit code, canonical command result, diagnostics, artifact list, and platform-independent hashes with `expected/`. `just fixtures-write` is the only rewrite path.
- `just calculus` (`cargo xtask check-calculus`) compares every committed target fixture under `compiler/fixtures/`, every GNAF request under `compiler/gnaf/`, the GNAF dependency manifest `compiler/gnaf.manifest.json` and schemas `schemas/gnaf-request.schema.json` and `schemas/gnaf-fixture.schema.json`, the generated `TargetFixtures` and `GnafFixtures` modules, the calculus modules `TargetSyntax`, `TargetSemantics`, `TargetOracle`, and `Main`, the `Gnaf` model, the project configuration, and every fixture's Rust package under `compiler/rust/rust-core/` and `compiler/rust/rust-std/` and every negative package manifest under `compiler/rust/negative/` with what the hand-written fixture sets, the calculus's definition in `crates/conformance/src/calculus_source.rs`, the model's definition in `crates/conformance/src/gnaf_model.rs`, and the renderer produce (§17.14, §17.15), and the calculus modules shipped for certificates under `language/preservation-1.2/modules/` with the compiler project's golden modules (§17.17); `just test` enforces the same comparison through `conformance_tc_03` and `conformance_gn_01`. `just calculus-write` is the only rewrite path.
- `just verify-write` (`cargo xtask verify-examples --write`) is the only path that rewrites `examples/*/expected/verify/`.
- `just release` runs `vv` and then `cargo xtask release-check` (RP-12): every §30.3 artifact by content, the §30.4 completion criteria, and the crate-packaging round trip (`cargo package`, extract, offline build, `--version` equal to the in-repository binary). It is refused until 1.0.0.

## What the claims mean

- Every §31 conformance ID is level `build` (§27.3): constructed here and validated against its oracle by the test `conformance_<id>`. Evidence, not a proof.
- Facts about Lean 4.32.1, Lake, `leanchecker`, and `#print axioms` output are `some-true` rows in `model/ledger.toml`: reproduced from the cited authority rows in `model/authorities.toml`, not established here.
- `lexlean verify` is the only command that runs Lean; its attestation records toolchain hashes, per-process records, and per-declaration observed axiom sets, and its ID is recomputed by `conformance_vr_14`.
- Lean-backed conformance cases detect the host at run time (`repo_conformance::support::lean_backed`): on Linux x86-64 the pinned toolchain is mandatory and a missing toolchain fails the case; on another supported host without the toolchain the case runs only its platform-independent assertions and prints that it did (§8.3). A verified fixture is reached only through `example_backed`, `corpus_backed`, `f1_backed`, or `verify_ok_backed`, so a case that forgets the gate fails on every non-normative host rather than claiming a verification it never ran. The generated Lean and LaTeX of those same fixtures are `build` products and are asserted byte for byte on every supported host.
- The cases whose subject is an external program written as `#!/bin/sh` take the same shape through `posix_shell_backed`, and a unix-only half of any other case reports itself through `unix_only` rather than dropping its assertions in silence.
- Two host properties are measured rather than inferred from the target triple, because they vary by filesystem and not by operating system: `case_sensitive_backed` writes two names differing only in case and reads back whether both survived (§23.3's collision cannot be built where they do not), and `non_utf8_names_backed` writes a name carrying a byte that is not valid UTF-8 (§8.3's environment diagnostic cannot be provoked where the filesystem refuses it). Each reports the host it declined on.

## Falsifiability records

Each gate below was made to fail by planting a defect, running the gate's command, recording the failure, and removing the defect. The observed lines are verbatim gate output (paths abbreviated to the repository root). `cargo xtask release-check` requires a `### <gate> can fail` record for every gate and audit named in `repo_model::release::GATES`.

### release inventory can fail

`release-artifacts` derives the tree manifest directly from the captured crate
produced in a fresh Cargo target. An existing unpacked verification cache is
not an input. Archive tests reject wrong package roots, duplicate/aliased
members, links, special files, truncation, corruption and trailing archives.
Release-tree traversal propagates errors and refuses aliases before writes.
Canonical path spelling is normalized after ancestor checks; each derived
asset replaces its directory entry atomically without changing external hard
links. This is not whole-release or power-loss atomicity.

In the non-root devcontainer, replacing the traversal error propagation with
the previous error-discarding behavior made
`release_manifest::tests::tree_inventory_rejects_symlinks_hardlinks_and_unreadable_directories`
fail at `assertion failed: result.is_err()` on an actual unreadable directory.
Restoring propagation passed the same test. The nine complete xtask tests,
model audits, scoped Clippy and dependency policy checks passed; these checks
alone are not complete release or publication acceptance.

From clean revision `9426d49`, the non-root devcontainer also ran
`cargo xtask release-artifacts` with a deliberately stale unpacked Cargo
manifest and extra cache file. Cargo packaged and verified 712 files in a fresh
target. Independent GNU tar extraction and Node SHA-256 inventory matched
every manifest row and all ten release-asset checksums; neither stale cache
input appeared. Crate SHA-256:
`b33f02625b7792e214379635b812e7eaa3f9282cad050d2606b412b546f28c7c`;
manifest SHA-256:
`5cff3e392b68ab1df90b117d400d7957cdf970fb0d387c5409956275fbb729c8`.
This checks package assembly, not the separately required build fleet, SBOM,
complete V&V evidence or publication.

CI run `36343661044` caught a hidden-test registration: an outer Unix `cfg`
hid the path-refusal test on other hosts. The test now always runs its portable
inventory assertions; its Unix-specific assertions use the repository's
explicit host-reporting convention. No test or release-gate exemption was added.

### release workflow validation can fail

GitHub run `35777451352` rejected the release workflow before starting any job:
`Unrecognized named-value: 'secrets'` in its token-selection condition.
Actionlint 1.7.8 reproduced the context error in the pinned container
`rhysd/actionlint@sha256:96d4a8c87dbbfb3bdd324f8fdc285fc3df5261e2decc619a4dd7e8ee52bbfd46`.
After moving only secret presence into the publish job's environment Boolean,
the same validator accepted every workflow with its default checks enabled.
The owner-token and trusted-publisher paths are preserved; no upload, release,
or full acceptance run is established by this syntax check.

### semantic string lowering can fail

Accepted NUL and C1-control literals produced Rust debug escapes rejected by
the pinned Lean parser (`LLV7002`). SM-19 now verifies eight modeled literal
probes, then executes those generated definitions against independently
encoded UTF-8 bytes. Coverage includes admitted controls, literal escape-shaped
text, quotes, backslashes, combining and supplementary scalars, empty text and
comment-shaped data. Source-document restrictions are unchanged; UTF-8
execution is not claimed as a reflexivity proof. The complete SM-19 run passed
with log SHA-256
`a6978a3aaffa815b5cc26e2f457169a04a2a6e8cd971186adf0a7c8d1aa9ee63`.
Restoring Rust debug spelling in the actual lowerer again produced Lean's
`invalid escape sequence` and `invalid hexadecimal numeral`; the same test
does not accept a Rust compilation failure. Logs are retained under ignored
`target/semantic-string-literals-{verified-evaluations,mutant}.log`.

### semantic identifier lowering can fail

Accepted field `namespace` and parameter `prefix` initially generated Lean
parser failures (`LLV7002`). `SM-19` now also renames only typed identifier
positions in the complete multi-module semantic example, exercising reserved
declaration, type-parameter, field, pattern, recursion, instance and proof/
reflection names through elaboration, kernel replay and exact axiom audit.
All eight expected identifier-bearing modules are changed; literals, term
kinds and semantic policies are preserved. The complete case passed with
log SHA-256 `7f1ab43c4e6685367494382c77ad9d485a58beb387b4251920a911aa2d03ad59`.
Removing reserved-segment quotation reproduced the actual `namespace`/
`prefix` parser failures, not a compile or fixture failure. Source restoration
retains the original accepted identifier language and all source-audit
prohibitions. Raw logs are retained under ignored
`target/semantic-reserved-identifiers-{lean-red,complete-variants,mutant}.log`.

### native diagnostic detail can fail

`CL-21` was registered before its named test failed on the unwired case.
Its actual `Engine::check` and `snapshot` cases cover package graph/self-import
cycles, cross-package lexical terms, qualified repairs, import-order and
prefix-package ordering, and nonmatching module/denotation cycles, same-package
terms, binder, segmentation, and both parsed and linked operator ambiguities.
Exact package versions and reverse import reachability additionally cover
connected/disconnected packages, every rotation of the actual loaded graph,
early self-import failures, and mismatched-version exclusions. Suppressing
reverse-edge traversal failed on the missing real parent package, recorded in
`target/typed-diagnostics-importers-mutant.log`. The earlier full-gate run was
interrupted for this addition and is retained as incomplete evidence in
`target/typed-diagnostics-full-vv-interrupted-importers.log`.
Existing committed diagnostic JSON and actual CLI JSON remain byte-equivalent;
these are native API checks, not Lean proof verification of the negative fixtures.

In the devcontainer, independently suppressing each native detail in its producer
setter made `cargo test -p repo-conformance --test conformance conformance_cl_21
-- --exact --nocapture` fail: the cycle mutation returned `None` instead of
`PackageImportCycle`, and the lexical mutation failed `actual term collision must
carry native detail`. Exact source restoration reproduced the passing case.
A real prefix-package regression first failed qualified-ID sorting, then passed
after sorting the joined IDs. Raw evidence remains in ignored
`target/typed-diagnostics-{red,cycle-mutant,term-mutant,prefix-red,focused}.log`.
The private detail field is an explicitly documented unreleased native
struct-literal source change; it does not alter the diagnostic wire schema.

### semantic elaboration budgets can fail

The `SM-19` regression generates one nested record/list declaration containing
twenty byte literals, including 4,096-, 4,097-, and 8,192-byte values. It keeps
the complete consumer corpus shape, checks every ordered literal against both
the owned snapshot and the actual generated array arguments, and verifies
repeated-byte and varied non-UTF-8 data through normal Lean elaboration,
kernel replay, and exact axiom audit. No Lean test implementation replaces the
modeled declaration. `LN-01` separately requires the exact finite option set
and preserves the historical language-1.0 preamble.

Before the correction, repeated data failed `LLV7002` with the pinned Lean
diagnostic `(deterministic) timeout at transform, maximum number of heartbeats
(200000) has been reached`; varied data failed the separate default recursion
limit. After correction, the two complete large-data fixtures verified.
Removing both generated resource-option lines again made
`cargo test --offline -p repo-conformance --test conformance conformance_sm_19
-- --exact --nocapture` fail on the actual `transform` heartbeat diagnostic,
with the current language lock intact. Restoring the fixed finite settings
does not change any declaration, byte, source limit, child timeout/output
bound, proof, or axiom policy. Raw runs are retained in ignored
`target/semantic-large-repeated-bytes-red.log`,
`target/semantic-large-varied-bytes-recursion-red.log`, and
`target/semantic-budget-removed-red.log`.

### semantic string context can fail

Observed: the initial payload detector searched for the literal adjacent bytes
`\semanticdata{`. A legal space or newline before the payload brace caused
`"007"` inside JSON string data to fail as a source numeral. The targeted test
`cargo test -p lexlean --lib semantic_string_context_tracks_controls_braces_and_whitespace -- --nocapture`
failed on `\semanticdata` followed by a newline and `{{"value":"007"}}`.

Corrected: payload scope follows the primitive control and delimiter atoms in
one linear pass. The same test passes with whitespace, braces inside strings,
escaped control-shaped string data, and successive payloads, and rejects
leading-zero numerals outside the payload, prefixed control names, and escaped
backslashes that do not introduce a control. JSON numeric tokens retain their
existing rejection. The Lean comment detector separately tests literal
delimiters and escaped quotes while rejecting actual line and block comments.

### language-1.1 semantic conformance can fail

Planted: the fixed Lean lowering of the generic `prop_and` term was changed
from propositional conjunction (`/\\`) to propositional disjunction (`\\/`).
Command: `cargo test -p repo-conformance --test conformance
conformance_sm_16 -- --exact --nocapture`. Expected: real Lean rejects the
Boolean-reflection proof because the corrupted proposition is not
definitionally equal to the independently defined validator proposition.

```text
the module verifies with real Lean: LexLeanError { class: Language,
diagnostics: [Diagnostic { code: DiagnosticCode("LLV7002"),
message: "Lean rejected `SemanticFixture.VariantProofs` (error): 'change' tactic failed, pattern
  (pair.left.beq 0 && pair.right.beq 1) = true ↔ pair.left = 0 ∧ pair.right = 1
is not definitionally equal to target
  validatePair pair = true ↔ pairValid pair" ... }] }
test conformance_sm_16 ... FAILED
test result: FAILED. 0 passed; 1 failed
```

Removed: the lowering was restored to conjunction from immutable implementation
commit `b52d47148fd30bb667daab244233851ca5029215`; `conformance_sm_16`
passes and its generated theorems have the exact empty observed axiom set.

### specification total can fail

Planted: the §31 total left at its pre-1.2 value (`**Total required
capability IDs:** 223.`) while the table carries 228 rows --- the state this
branch was first pushed in. Command: `cargo xtask validate-spec-links`.

```text
gate failed: RP-07: §31 states 223 required capability IDs but its table has 228 rows
```

### audit-production can fail

Planted: the explicit arm `SemanticPrimitive::GraphTopological =>
"primitive.graph_topological",` in
`crates/lexlean/src/production/eligibility.rs` replaced by the wildcard
`_ => "primitive.graph_topological",`, and separately by the binding
catch-all `_other => "primitive.graph_topological",`. Commands: `cargo xtask
validate-model` and `cargo clippy -p lexlean -- -D warnings`. Expected: both
the audit and the compiler refuse each default, so neither gate alone is
load-bearing.

```text
gate failed: §17.13: crates/lexlean/src/production/eligibility.rs:152: a wildcard arm would classify constructs by default
gate failed: §17.13: crates/lexlean/src/production/eligibility.rs:152: a binding or wildcard catch-all arm would classify constructs by default
error: wildcard matches only a single variant and will also match any future added variants
   --> crates/lexlean/src/production/eligibility.rs:152:9
```

The compiler sees only enum matches, so the audit alone covers a tuple
default and an equality chain over the IR. `conformance_pd_07` plants, against
the committed source, the wildcard, the binding catch-all, a rest pattern, an
`if let`, a tuple pattern `(_, _)`, an `==` test on `SemanticType`, an unnamed
`SemanticInteger::UInt64`, and a `SemanticPrimitive::GraphTopological` that
is listed but matched nowhere, and asserts that the audit reports each one.
Removed: every arm restored; both gates pass.

### production root monomorphism can fail

Planted: the check in `analyse_root` that a root declares no type parameters
was disabled (`if false && !type_parameters.is_empty()`). Command: `cargo test
-p repo-conformance --test conformance -- conformance_pd_05`. Expected: the
`production-phantom-type-parameter` root, whose signature never mentions its
type parameter, is no longer refused for declaring it; only the walk's second
check, which refuses a type parameter that survives instantiation, still
stops it, and the case observes the changed reason.

```text
thread 'conformance_pd_05' panicked at crates/conformance/src/cases/production.rs:656:17:
production-phantom-type-parameter: expected "production root declares the type parameters (Item)", got LLT4005: phase production: production root `LanguageTwelve.Main.count` is not eligible for target `rust-std`: the type parameter `Item` is never instantiated in the closure (in `LanguageTwelve.Main.count`, reached by LanguageTwelve.Main.count; 1 violation(s) in total)
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 249 filtered out
```

Removed: the check was restored; `conformance_pd_05` passes and the fixture
fails with `LLT4005` naming both violations.

### production target dependence can fail

Planted: the allocation test in `analyse_root` replaced by one that holds for
every target (`if target.allocation || !target.allocation {`), so a
no-allocation target admitted allocation as an effect. Command: `cargo test -p
repo-conformance --test conformance -- conformance_pd_04 conformance_pd_05`.

```text
thread 'conformance_pd_05' panicked at crates/conformance/src/cases/production.rs:656:17:
production-unbounded-type: expected "construct `type.list` requires heap allocation, which target `rust-core` does not provide", got LLT4005: phase production: production root `LanguageTwelve.Main.first` is not eligible for target `rust-core`: effect mismatch: construct `type.list` realizes effect `allocation`, which the root does not admit (in `LanguageTwelve.Main.first`, reached by LanguageTwelve.Main.first; 1 violation(s) in total)
thread 'conformance_pd_04' panicked at crates/conformance/src/support.rs:418:14:
check fails
```

### production effect admission can fail

Planted: the admitted-effect test disabled (`if false &&
!production.effects.contains(effect) {`), so a root realized effects it did
not declare. Command: `cargo test -p repo-conformance --test conformance --
conformance_pd_05`. The first fixture whose only violation is an unadmitted
effect now checks.

```text
thread 'conformance_pd_05' panicked at crates/conformance/src/support.rs:418:14:
check fails
```

### named-root dropped dependency can fail

Planted: the comparison of Lean's closure with the eligibility closure ignored
members only Lean reaches
(`source.difference(&eligibility.declarations).next().filter(|_| false)` in
`crates/lexlean/src/production/lcnf.rs`). Command: `cargo test -p
repo-conformance --test conformance -- conformance_ne_05`. Expected: an
eligibility closure that drops `Production.Kernel.area` is accepted.

```text
thread 'conformance_ne_05' panicked at crates/conformance/src/cases/extraction.rs:73:18:
expected an LLV7011 rejection containing "dropped dependency: Lean's compiler reaches `Production.Kernel.area` from `Production.Main.shapeArea`", got Ok(CompilerInput { spec: "lexlean/compiler-input/2", ...
```

Planted: the pinned adapter stopped following project callees (`if inProject r
then codeQueue := codeQueue`), so no dependency of a root was translated. The
oracle is Lean itself. Command: `lexlean lock && lexlean verify` in a copy of
`examples/production`.

```text
error[LLV7011]: named-root extraction: the extraction record of `Production.Main.LexLeanRuntime.instFixedUInt32` lacks its code
```

The message now names the class: "dropped dependency: … is a definition Lean
compiles, but the extraction reports no translation of it". Removed: both
were restored; `conformance_ne_05` passes and the production example verifies
with its committed compiler input.

### named-root proof-as-runtime and erasure can fail

Planted: the theorem check on closure members was bypassed (`"theorem" if
false =>` in `unrealizable`). Command: `cargo test -p repo-conformance --test
conformance -- conformance_ne_06`. Expected: the proof-as-runtime class is no
longer named.

```text
thread 'conformance_ne_06' panicked at crates/conformance/src/cases/extraction.rs:68:13:
expected "proof-as-runtime dependency: the theorem `Production.Kernel.area` is in the runtime closure", got `Production.Kernel.area` is a theorem, not a definition the compiler can realize
```

Planted: the pinned adapter reported theorems as definitions (`| .thmInfo _ =>
"definition"`), so the host could no longer tell which kernel-reached
constants are proofs. The oracle is Lean itself. Command: `lexlean lock &&
lexlean verify` in a copy of `examples/production`.

```text
error[LLV7011]: named-root extraction: the proof-only dependencies of `Production.Main.halvings` differ: Lean erases {}, the eligibility analysis {"Production.Kernel.countdown_decreases"}
```

Removed: both were restored; `conformance_ne_06` passes and the production
example verifies.

### named-root authority closure can fail

Planted: the pinned adapter ran an LCNF pass after translation
(`CompilerM.run (do let d ← toDecl n; d.etaExpand)`). The oracle is Lean
itself. Command: `lexlean lock && lexlean verify` in a copy of
`examples/production`. Expected: the registry check refuses every constant
the pass introduces.

```text
error[LLV7012]: named-root extraction: the adapter's constants no longer match the pinned registry: lexlean-extract-drift: the adapter and its registry differ: `Lean.Compiler.LCNF.CompilerM` is used as type but registered as unregistered; `Lean.Compiler.LCNF.instMonadCompilerM` is used as plumbing but registered as unregistered; `Lean.Compiler.LCNF.Decl.etaExpand` is used as call but registered as …
```

`conformance_ne_04` plants, against pinned Lean, a changed result type, a
changed default value (`optParam Bool Bool.true`), a changed binder kind
(`(α : Type)` for `{α : Type}`), a constructor list missing `ctorAlt`, a call
missing from the registry, and a deprecation warning inside the adapter on a
run Lean completes successfully; each is drift naming its cause. The
structural signature comparison is what refuses the default value and the
binder kind, which the definitional-equality probes it replaced accepted.
Removed: the adapter was restored; it verifies.

### named-root host decisions can fail

Planted, one at a time in `crates/lexlean/src/production/lcnf.rs`, each with
`cargo test -p repo-conformance --test conformance -- <case>`:

- recursion never marked (`if next == *name && false`), `conformance_ne_02`:

  ```text
  thread 'conformance_ne_02' panicked at crates/conformance/src/cases/extraction.rs:428:13:
  assertion `left == right` failed
  ```

- the monomorphization plan dropped (`instances: Vec::new()`),
  `conformance_ne_02`:

  ```text
  thread 'conformance_ne_02' panicked at crates/conformance/src/cases/extraction.rs:410:18:
  the firstOr instance
  ```

- lexical scope not restored after an alternative (`let _ = saved;`),
  `conformance_ne_03`, whose sibling-alternative reference then passes the
  renamer:

  ```text
  thread 'conformance_ne_03' panicked at crates/conformance/src/cases/extraction.rs:68:13:
  expected "is used before it is bound", got the extraction record of `Production.Kernel.total` lists uses that differ from its code
  ```

- output beside the record ignored (`lines.len() <= 1 || lines.len() > 1`),
  `conformance_ne_04`, whose warning plant is then misread as a malformed
  record:

  ```text
  thread 'conformance_ne_04' panicked at crates/conformance/src/cases/extraction.rs:834:34:
  expected drift naming "the pinned extraction adapter no longer elaborates cleanly", got Rejected("the extraction record is malformed: expected value at line 1 column 1")
  ```

Removed: each was restored; the cases pass.

### language-1.2 version routing can fail

Planted: the language-1.2 construct gate in `SemanticModule::validate` was
routed to the wrong language (`language == crate::LANGUAGE_1_2` instead of
`crate::LANGUAGE_1_1`), so language 1.1 admitted the 1.2-only `let` term and
language 1.2 refused it. Commands: `cargo test -p repo-conformance --test
conformance -- conformance_sm_23` and `cargo xtask check-fixtures`.
Expected: the committed language-1.2 example no longer checks, and the
negative fixture of a `let` under language 1.1 is accepted.

```text
thread 'conformance_sm_23' panicked at crates/conformance/src/support.rs:408:14:
check succeeds: LexLeanError { class: Language, diagnostics: [Diagnostic { code: DiagnosticCode("LLT4001"), message: "phase link: `let` is a language-1.2 construct; language 1.1 rejects it", ... }] }
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 227 filtered out
gate failed: tests/negative/language-1.2-construct-under-1.1: step 1 `check ` exited 0, case.toml expects 1
```

Planted: `"1.3"` appended to `LANGUAGE_VERSIONS`. Command: `cargo test -p
repo-conformance --test conformance -- conformance_cf_17`.

```text
thread 'conformance_cf_17' panicked at crates/conformance/src/cases/configuration_lock.rs:1097:40:
`1.3` is not a supported language
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 227 filtered out
```

Planted: `Lock::canonical_bytes` emitted `lexlean/lock/1` for language 1.2.
Command: `cargo test -p repo-conformance --test conformance --
conformance_cf_18`. The migrated copy of the 1.1 example is the first lock
written for language 1.2.

```text
thread 'conformance_cf_18' panicked at crates/conformance/src/cases/configuration_lock.rs:1221:13:
assertion failed: migrated_lock.starts_with("spec = \"lexlean/lock/2\"\nlanguage = \"1.2\"\n")
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 227 filtered out
```

Planted: `schemas/project-v2.schema.json` dropped from the 1.2-only partition
in `is_v1_2_file`, so the frozen 1.1 identity silently absorbed a 1.2 file.
Command: `cargo test -p repo-conformance --test conformance --
conformance_rp_10`. The test recomputes the 1.1 ID from the exclusion list in
§21.2, not from the compiler.

```text
thread 'conformance_rp_10' panicked at crates/conformance/src/cases/repository.rs:626:13:
assertion `left == right` failed: RP-10: the 1.1 ID
```

Planted: `"opt-11pt"` removed from the backend tokens of
`language/bootstrap-1.2.toml` only. Command: `cargo xtask validate-model`.
`audit-language-closure` had compared only the 1.0 and 1.1 bootstraps; it now
discovers every bootstrap.

```text
gate failed: R8: language/bootstrap-1.2.toml declares a different fixed backend token set
```

Removed: each mutation was reverted; `conformance_sm_23`,
`conformance_cf_17`, `conformance_cf_18`, `conformance_rp_10`,
`cargo xtask validate-model`, and `cargo xtask check-fixtures` pass.

### standalone structural recursion can fail

Planted: the rejection of a standalone `recursive_argument` over a nested or
mutual inductive disabled (`info.nested_or_mutual && false`), so a single
definition could claim structural recursion over `Rose`, which has no single
structural eliminator. Command: `cargo test -p repo-conformance --test
conformance -- conformance_df_13`.

```text
thread 'conformance_df_13' panicked at crates/conformance/src/support.rs:418:14:
check fails
```

### recursive data positivity can fail

Planted: the positivity rule in `classify_occurrence` was bypassed (`if false
&& ...`), so a group member inside another document type's arguments was
admitted. Command: `cargo test -p repo-conformance --test conformance --
conformance_df_12`. Expected: the mutation that adds an otherwise unused,
well-formed `Wrap` with a constructor field `Tree (Wrap)` is no longer
rejected at all; only the positivity rule refused it.

```text
thread 'conformance_df_12' panicked at crates/conformance/src/support.rs:418:14:
check fails
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 230 filtered out
```

Removed: the rule was restored; `conformance_df_12` passes and the
`recursive-type-positivity` negative fixture fails with `LLT4001`.

### lambda capture closure can fail

Planted: the exact-capture rule for `lambda` was bypassed (`if false && used
!= declared`), so a lambda could declare captures its body does not use.
Command: `cargo test -p repo-conformance --test conformance --
conformance_sm_25`. Expected: the capture mutation no longer fails with the
exact-capture diagnostic (an undeclared use still fails later as an unbound
local, and an unused declared capture is accepted).

```text
thread 'conformance_sm_25' panicked at crates/conformance/src/cases/semantic_ir.rs:1696:17:
expected "declared (), used (offset)", got LLT4001: phase link: unbound local `offset`
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 234 filtered out
```

Removed: the rule was restored; `conformance_sm_25` passes and the
`lambda-capture-extra` negative fixture fails with `LLT4001`.

### higher-order lowering can fail

Planted: the Lean lowering of `call` dropped its explicit type arguments
(`for argument in type_arguments.iter().take(0)` in
`crates/lexlean/src/backend/semantic.rs`), so a generic call lowered to a
call of the wrong arity. Command: `cargo test -p repo-conformance --test
conformance -- conformance_sm_25 conformance_df_14`.

```text
thread 'conformance_df_14' panicked at crates/conformance/src/cases/declarations.rs:926:13:
assertion failed: combinators.contains("mapList (Input) (Output) (transform) (tail)")
thread 'conformance_sm_25' panicked at crates/conformance/src/cases/semantic_ir.rs:1684:17:
missing "HigherOrder.Combinators.mapList (Nat) (Nat) ((fun (value : Nat) => (value + offset))) (values)" in:
```

Removed: the type arguments were restored; `conformance_sm_25` and
`conformance_df_14` pass.

### closure escape through data can fail

Planted: `holds_function` stopped looking through a document type's field
types (`false && type_info(member, env)...`), so a record of closures was data
without functions. Command: `cargo test -p repo-conformance --test
conformance -- conformance_df_15`. The executable `evaluator` that returns a
`Visitor` of closures is no longer refused for returning a function.

```text
thread 'conformance_df_15' panicked at crates/conformance/src/cases/declarations.rs:1040:17:
expected "escaping closure: executable definition `evaluator` returns a value of type Combinators.Visitor, which holds a function", got LLT4001: phase link: escaping closure in executable definition `evaluator`: a lambda may only be passed directly to an executable function parameter or applied
```

Removed: the field types were restored; `conformance_df_15` passes and the
`escaping-closure-structure` negative fixture fails with `LLT4001`.

### unused semantic binder lowering can fail

Planted: `bound_name` kept every binder's own name (`if used || true`), so a
definition parameter, quantifier, `let`, or lambda binder its scope never
mentions reached Lean unprefixed, where the unused-variable linter warns and
verification fails with `LLV7006`; the same holds for a type parameter its
declaration never mentions. Command: `cargo test -p repo-conformance --test
conformance -- conformance_sm_08 conformance_df_14`.

```text
thread 'conformance_df_14' panicked at crates/conformance/src/cases/declarations.rs:953:17:
missing "public def keepNat (_Phantom : Type) (value : Nat) : Nat := value\n" in:
thread 'conformance_sm_08' panicked at crates/conformance/src/cases/semantic_ir.rs:572:17:
missing "public def constantTrue (_ignored : Nat) : Bool := true" in:
test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 233 filtered out
```

Removed: the prefix was restored; `conformance_sm_08` passes and pinned Lean
verifies both the language-1.1 and the language-1.2 projects it builds.

### well-founded decrease check can fail

Planted: the statement comparison of termination evidence was bypassed
(`(false && stated.2 != obligation)`), so a theorem with the right binders
but a different statement was accepted as a call's decrease evidence.
Command: `cargo test -p repo-conformance --test conformance --
conformance_df_17`. Expected: the forged-evidence mutation links.

```text
thread 'conformance_df_17' panicked at crates/conformance/src/support.rs:418:14:
check fails
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 238 filtered out
```

Removed: the comparison was restored; `conformance_df_17` passes and the
`recursion-forged-evidence` negative fixture fails with `LLT4001`.

### structural decrease check can fail

Planted: the structural decrease test of a recursive call admitted any
variable (`SemanticTerm::Var { name } if true || smaller.contains(name)`),
so a member could call its group on its own argument. Command: `cargo test
-p repo-conformance --test conformance -- conformance_df_16`. Expected: the
`isOdd (number)` mutation links.

```text
thread 'conformance_df_16' panicked at crates/conformance/src/support.rs:418:14:
check fails
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 238 filtered out
```

Removed: the test was restored; `conformance_df_16` passes and the
`recursion-wrong-argument` negative fixture fails with `LLT4001`.

### recursion call graph check can fail

Planted: `check_group_call_graph` returned `Ok(())` at once, so a mutual
group's members were never required to call into the group or to be
strongly connected. Commands: `lexlean check` then `lexlean verify` on a copy
of the `recursion-mutual-member-without-call` negative project, whose
structural group `Walk` has a member `settle` that calls no member. Expected:
linking admits the group and Lean, not linking, is the first to refuse it.

```text
checked 1 module (source 9fb919ed41971b7cf0d6ccb531f9b67442d85c865de79e61a0015a80bcc848bd,
error[LLV7006]: Lean produced unexpected output for `LanguageTwelve.Main` (warning): unused `termination_by`, function is not recursive
  --> src/Main.lex.tex:5:32
```

In a well-founded group the same plant is still refused, by the separate rule
that a definition with termination evidence makes a recursive call:
`conformance_df_17` fails with `definition `pong` declares termination
evidence but makes no recursive call` where it expects the call-graph
message.

Removed: the check was restored; `recursion-mutual-member-without-call` and
`recursion-mutual-disconnected` fail in linking with `LLT4001`, and
`conformance_df_17` passes.

### false termination evidence is refused by verification

Not a plant but the boundary the recursion rules state: linking checks that
each obligation is stated exactly, and Lean's kernel decides whether it is
true. The `recursion-false-evidence` negative project states the exact
obligation of a call `stall (number)` on its own argument, with a
`linear_arithmetic` proof that cannot hold. Command: `lexlean verify`.

```text
error[LLV7002]: Lean rejected `LanguageTwelve.Main` (error): omega could not prove the goal:
error[LLV7002]: Lean rejected `LanguageTwelve.Main` (error): well-founded recursion cannot be used, `LanguageTwelve.Main.stall` does not take any (non-fixed) arguments
```

Each diagnostic points at its declaration in the source (the evidence
theorem, then `stall`), through the per-declaration source maps.

### well-founded lowering under match can fail

Planted: a numbered `match` lowered without `(generalizing := false)`, so
Lean refined the earlier hypotheses and the evidence of `reassociate` no
longer applied to them (one rejection per evidence application of
`reassociate` and `prune`). The oracle is Lean itself. Command: `lexlean build &&
lexlean verify` in `examples/recursion`.

```text
error[LLV7002]: Lean rejected `Recursion.Main` (error): Application type mismatch: The argument
error[LLV7002]: Lean rejected `Recursion.Main` (error): Application type mismatch: The argument
error[LLV7002]: Lean rejected `Recursion.Main` (error): Application type mismatch: The argument
error[LLV7002]: Lean rejected `Recursion.Main` (error): Application type mismatch: The argument
```

Removed: the lowering was restored; the recursion example verifies and its
normalized verification records match `cargo xtask verify-examples`.

### binder hygiene can fail

Planted: the language-1.2 call to `check_binder_hygiene` in
`SemanticModule::validate` was disabled (`if false && ...`). Commands: `cargo
test -p repo-conformance --test conformance -- conformance_df_12`, and
`lexlean check` then `lexlean verify` on a copy of the `binder-capture`
negative project, whose inductive `Box` has a type parameter `Prod` and a
product field. Expected: the renamed type parameter is admitted, and Lean,
not linking, is the first to refuse the capture.

```text
thread 'conformance_df_12' panicked at crates/conformance/src/support.rs:418:14:
check fails

checked 1 module (source 0eec48d541180c1a6dbf34539884c1063f8a6637964a1f7667a5b6c0f73a4d15, semantic 1fe3fc4d4fa60d781de6ea42be16516d7b6b860a2918f6ed351b40ad6e1e0d2c)
error[LLV7002]: Lean rejected `LanguageTwelve.Main` (error): Function expected at
  Prod
but this term has type
  Type
```

Removed: the check was restored; `conformance_df_12` passes and
`binder-capture` fails in linking with `LLT4001`, before any backend runs.

### collection ordering determinism can fail

Planted: the canonical normalization of map, set, and graph literals was
skipped (`if false && language == crate::LANGUAGE_1_2`), so a literal kept its
source order. Command: `cargo test -p repo-conformance --test conformance --
conformance_sm_29`. Expected: reordered equivalent graph literals no longer
link to one semantic identity.

```text
thread 'conformance_sm_29' panicked at crates/conformance/src/cases/semantic_ir.rs:2114:13:
assertion `left == right` failed: the semantics do not
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 242 filtered out
```

Removed: normalization was restored; `conformance_sm_29` passes.

### collection literal key checking can fail

Planted: a map literal's keys were ordered without first being checked as
terms (the `check_term(&entry.key, ...)` call removed), so a noncanonical
key such as `02` took a position in the canonical order. Command: `cargo
test -p repo-conformance --test conformance -- conformance_sm_29`.
Expected: the `02` mutation links.

```text
thread 'conformance_sm_29' panicked at crates/conformance/src/support.rs:418:14:
check fails
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 242 filtered out
```

Removed: the check was restored; `conformance_sm_29` passes and the
`collection-noncanonical-key` and `collection-out-of-range-key` fixtures fail
with `LLT4001`.

### collection runtime emission can fail

Planted: a collection primitive no longer required the runtime
(`Some("primitive") => false && ...` in `uses_collections`), so a module
that only measures an imported table emitted none. Commands: `cargo test -p
repo-conformance --test conformance -- conformance_sm_28`, then `lexlean
build && lexlean verify` in `examples/collections`, where pinned Lean is the
oracle.

```text
thread 'conformance_sm_28' panicked at crates/conformance/src/cases/semantic_ir.rs:2018:17:
missing "namespace LexLeanCollections" in:
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 242 filtered out
error[LLV7002]: Lean rejected `Collections.Measure` (error lean.unknownIdentifier): Unknown identifier `LexLeanCollections.mapSize`
error[LLV7002]: Lean rejected `Collections.Measure` (error): Tactic `decide` failed for proposition
```

Removed: the primitive case was restored; `conformance_sm_28` passes and the
collections example verifies.

### check-calculus can fail

Planted: the committed expected outcome of `compiler/fixtures/sum-to.json`
was edited from 55 to 56. Command: `cargo xtask check-calculus`. Expected:
the committed fixture no longer equals what the hand-written fixture set
renders.

```text
gate failed: compiler/fixtures/sum-to.json differs from its generator; run `cargo xtask check-calculus --write`
```

Removed: the fixture was restored; `cargo xtask check-calculus` reports 208
generated files equal to their generator.

### check-calculus covers the calculus modules

Planted: in the committed `compiler/src/TargetSemantics.lex.tex`, the bound
test of `natResult` was edited from `blt` to `ble`, admitting `2^64` as a
`nat` result. Command: `cargo xtask check-calculus`. Expected: the committed
module no longer equals what `crates/conformance/src/calculus_source.rs`
renders.

```text
gate failed: compiler/src/TargetSemantics.lex.tex differs from its generator; run `cargo xtask check-calculus --write`
```

Removed: the committed module was restored; the gate reports 208 generated
files equal to their generator.

### check-calculus covers the shipped calculus modules

Planted: one comment line appended to
`language/preservation-1.2/modules/LexLeanTarget/TargetSyntax.lean`, the copy
certificates import. Command: `cargo xtask check-calculus`. Expected: the
shipped module no longer equals the compiler project's golden module (§17.17),
reported before any file whose provenance binds the compiler-semantics ID.

```text
gate failed: <root>/language/preservation-1.2/modules/LexLeanTarget/TargetSyntax.lean differs from the compiler golden <root>/compiler/expected/build/modules/LexLeanTarget/TargetSyntax.lean; run `cargo xtask check-calculus --write`
```

Removed: the line was deleted; `cargo xtask check-calculus` reports 581
generated files equal to their generator.

### audit-production covers the preservation sources

Planted: `fn planted(t: Option<u8>) { if let Some(_) = t {} }` appended to
`crates/lexlean/src/production/certificate.rs`. Command: `cargo xtask
validate-model`. Expected: the certificate generator matches a construct by
default (§17.17 extends §17.13's audit to the lowering, the generator, and
the source reader).

```text
gate failed: §17.17: crates/lexlean/src/production/certificate.rs:6381: an `if let` with an implicit default would classify constructs by default
```

Removed: the function was deleted; `audit-production` reports that the three
preservation sources match no construct by default. `conformance_sp_01` plants
an `if let`, a `matches!`, and a wildcard arm in each source and requires
the audit to refuse every one.

### audit-production closes certificate B's rule set

Planted: in `crates/lexlean/src/production/rust_cert.rs`, the block
judgment's arm `Term::Apply { .. } => self.apply(g, fl, false, term, lets,
tail),` rewritten as `_ => self.apply(g, fl, false, term, lets, tail),`.
Command: `cargo xtask validate-model`. Expected: the aligner meets a
calculus construct by default (§17.17, SP-08).

```text
gate failed: §17.17 (SP-08): crates/lexlean/src/production/rust_cert.rs: the block judgment does not name the construct `Term::Apply`
crates/lexlean/src/production/rust_cert.rs: the block judgment has a wildcard arm, which would meet a construct by default
```

Removed: the arm was restored; `audit-production` reports that certificate
B's 49 rules are exactly the correspondence's constructors, each a case of
its soundness theorem, with every calculus construct named.
`conformance_sp_08` plants, in the repository's own sources, a rule dropped
from the aligner's `RULES`, a case renamed in `sound`, and this wildcard arm,
and requires each to be reported; the unit tests of
`repo_model::correspondence` plant a duplicate rule, an extra constructor, a
stray soundness case, a rule never written, a binding arm, and a calculus
construct the aligner does not name.

### CL-11 covers every registered code's class

Planted: `LLV7012` absent from the environment arm of
`DiagnosticCode::class` (`crates/lexlean/src/diagnostic.rs`), the state the
repository was in before the fix. Command: `cargo test -p repo-conformance
--test conformance conformance_cl_11`. Expected: the code's mapped class and
exit code differ from its `model/errors.toml` row (§23.6, R1).

```text
thread 'conformance_cl_11' panicked at crates/conformance/src/cases/cli_api.rs:539:13:
§23.6: codes whose mapped class differs from model/errors.toml: ["LLV7012: registered environment (exit 3), mapped language (exit 1)"]
```

Removed: `LLV7012` was added to the environment arm (with `LLV7014`); the case
passes, and the `extraction-authority-drift` fixture expects exit 3.

### NE-03 admits an exported axiom only as a compiled definition

Planted, one at a time, in `admissible_external` and the record reader
(`crates/lexlean/src/production/lcnf.rs`): (1) the `compiled` requirement
dropped; (2) the `borrowed` annotation read as an unsupported form, the state
before the fix; (3) every external reported as an axiom treated as an
exported definition, whatever kind it was declared with. Command: `cargo test
-p repo-conformance --all-features --test conformance conformance_ne_03
conformance_ex_07`. Expected: the planted rule admits what §22.10 refuses, or
refuses what it admits.

```text
(1) thread 'conformance_ne_03' panicked at crates/conformance/src/cases/extraction.rs:73:18:
expected an LLV7011 rejection containing "`instMulNat` is noncomputable or has no compiled code", got Ok(CompilerInput { …
(1) thread 'conformance_ex_07' panicked at crates/conformance/src/cases/examples.rs:382:13:
/home/user/wt-25/tests/negative/extraction-uncompiled-external: step 1 `verify ` exited 0, case.toml expects 1
(2) thread 'conformance_ne_03' panicked at crates/conformance/src/cases/extraction.rs:666:18:
a borrowed domain: Rejected("`Production.Kernel.area`: unsupported compiler form: the LCNF type `metadata` has no closed representation")
(3) thread 'conformance_ne_03' panicked at crates/conformance/src/cases/extraction.rs:73:18:
expected an LLV7011 rejection containing "`instMulNat` is an axiom", got Ok(CompilerInput { …
```

Removed: each plant restored; both cases pass. Before the fix, verifying
`examples/production-coverage` failed with `LLV7011` at `String.toInt?`,
`String.toUTF8`, and `List.takeTR` (axioms in the exported view) and at the
`metadata` type of `instQuotientNat`; it now verifies, its 33 certificates
included.

### calculus kernel oracle can fail

Planted: the reference interpreter computed `int_rem` as a Euclidean
remainder (`rem_euclid`) instead of Lean's truncating `Int.tmod`, and
`cargo xtask check-calculus --write` regenerated the fixtures from it, so
the stated expectation of `int-arithmetic` changed from -1 to 1. The oracle
is Lean's kernel. Command: `lexlean verify` in `compiler/`.

```text
error[LLV7002]: Lean rejected `LexLeanTarget.TargetFixtures` (error): Tactic `rfl` failed: The left-hand side
  intArithmeticRun
is not definitionally equal to the right-hand side
```

Removed: the interpreter was restored and the fixtures regenerated;
`compiler/` verifies and `conformance_tc_03` and `conformance_tc_04` pass.

### calculus rust differential can fail

Planted: the runtime computed natural subtraction by wrapping
(`a.wrapping_sub(b)`) instead of truncating at zero. Command: `cargo test -p
repo-conformance --test conformance -- conformance_tc_07`. Expected: the
compiled rendering of `nat-arithmetic` prints a different value.

```text
thread 'conformance_tc_07' panicked at crates/conformance/src/cases/calculus.rs:1351:17:
assertion `left == right` failed: natArithmetic_rust_std: the Rust rendering and the denotation disagree
```

Removed: the runtime was restored; `conformance_tc_07` passes.

### calculus rust work bound can fail

Planted: the `rust-std` runtime's `append_list` rebuilt its accumulated
result once per element of the left operand, so its work grew quadratically
while its value stayed correct. Command: `cargo test -p repo-conformance
--test conformance -- conformance_tc_07`. Expected: the rendering of
`list-append-long` counts more work than the denotation charges steps.

```text
thread 'conformance_tc_07' panicked at crates/conformance/src/cases/calculus.rs:1356:17:
listAppendLong_rust_std: the rendering worked 1220 units for 87 steps
```

Removed: the runtime was restored; `conformance_tc_07` passes, and every
rendering's work is within its steps.

### rust-core allocation refusal can fail

Planted: the renderer's heap check refused a heap type in `rust-core` only
when its description was empty, so `rust-core` admitted strings, byte
strings, and lists. Command: `cargo test -p repo-conformance --test
conformance -- conformance_tc_07`. Expected: `rust-core` renders a program
whose realization requires allocation.

```text
thread 'conformance_tc_07' panicked at crates/conformance/src/cases/calculus.rs:1271:29:
binding-and-shapes: rust-core renders a program that needs the heap
```

Removed: the check was restored; `rust-core` renders exactly the fixtures
`realization::program_allocates` says need no heap.

### calculus realization coverage can fail

Planted: the realization row of `primitive.map_size` was deleted from
`crates/lexlean/src/calculus/realization.rs`. Command: `cargo test -p
repo-conformance --test conformance -- conformance_tc_06`.

```text
thread 'conformance_tc_06' panicked at crates/conformance/src/cases/calculus.rs:1149:48:
the realization table: "runtime construct `primitive.map_size` has no realization row"
```

Removed: the row was restored; `conformance_tc_06` passes.

### registry allocation agreement can fail

Planted: `term.nil` in `language/production-1.2.toml` was marked
`allocation = true`, although its realization builds no heap storage.
Command: `cargo test -p repo-conformance --test conformance --
conformance_tc_06`.

```text
thread 'conformance_tc_06' panicked at crates/conformance/src/cases/calculus.rs:1165:13:
allocation disagreements: [
    "term.nil: registry true, realization false",
]
```

Removed: the row was restored; every runtime row's `allocation` equals its
realization's.

### fixed-width coverage can fail

Planted: the fixture generator skipped the `u16` instances of
`fixed-width-*` and `fixed-decimal-*`. Command: `cargo test -p
repo-conformance --test conformance -- conformance_tc_06`. Expected: the
typed (primitive, width) pairs only those fixtures exercise are reported.

```text
thread 'conformance_tc_06' panicked at crates/conformance/src/cases/calculus.rs:1219:13:
(primitive, width) pairs no fixture exercises: [("bit_and", "u16"), ("bit_not", "u16"), ("bit_or", "u16"), ("bit_xor", "u16"), ("checked_mul", "u16"), ("checked_quot", "u16"), ("compare", "u16"), ("format_decimal", "u16"), ("parse_decimal", "u16"), ("shift_left", "u16"), ("shift_right", "u16")]
```

Removed: the generator was restored; every admitted pair is exercised.

### calculus progress sampling can fail

Planted: the interpreter took the `succ` shape only for naturals below
`2^63`, so larger naturals matched no arm. No fixture's stated arguments
reach such a value. Command: `cargo test -p repo-conformance --test
conformance -- conformance_tc_03`. Expected: a seeded random argument does.

```text
thread '<unnamed>' (8081) panicked at crates/conformance/src/cases/calculus.rs:39:17:
assertion `left != right` failed: iterate: stuck on [Nat { value: "18446744073709551615" }] with fuel 7
```

Removed: the interpreter was restored. The sampling also found a real defect:
the interpreter's checked fixed-width multiplication multiplied in `i128`,
which two 64-bit operands near `u64::MAX` overflow, so it panicked instead of
returning `none`. The product is now checked.

### calculus cost accounting can fail

Planted: the reference interpreter did not charge the step for the arm a
`match` takes, and `cargo xtask check-calculus --write` regenerated the
fixtures from it. The oracle is Lean's kernel over the LexLean denotation.
Command: `lexlean verify` in `compiler/`.

```text
error[LLV7002]: Lean rejected `LexLeanTarget.TargetFixtures` (error): Tactic `rfl` failed: The left-hand side
  adtEvaluationRun
is not definitionally equal to the right-hand side
  TargetSemantics.Outcome.value (TargetSyntax.Value.nat 37) 49
```

A second plant dropped the result weight from a primitive's charge; the
kernel refused the same theorem, stated with 50 steps. Removed: the
interpreter was restored and the fixtures regenerated; `compiler/` verifies.

### kernel-opaque classification is observed

Probe: `compare_bytes` and byte equality were both treated as reducible, and
the new fixture `byte-compare` applies `compare_bytes` alone. Command:
`lexlean verify` in `compiler/`. Lean accepted the `rfl` theorem of the
`equality` fixture, so byte equality is now decided by the kernel. It refused
`byte-compare`:

```text
error[LLV7002]: Lean rejected `LexLeanTarget.TargetFixtures` (error): Tactic `rfl` failed: The left-hand side
  byteCompareRun
is not definitionally equal to the right-hand side
```

`compare_bytes` stays evaluator-only.

### rust construct correspondence can fail

Planted: the correspondence row of `call:runtime:nat_add` was deleted from
`crates/lexlean/src/calculus/rust/validate.rs`. Command: `cargo test -p repo-conformance --test conformance -- conformance_rb_01`. Expected:
every rendering that adds naturals emits a construct with no
target-semantics correspondence.

```text
thread 'conformance_rb_01' (22507) panicked at crates/conformance/src/cases/rust_backend.rs:514:40:
adt-evaluation (rust-std): the construct `call:runtime:nat_add` has no target-semantics correspondence
```

Removed: the row was restored; `conformance_rb_01` passes.

### rust correspondence is checked per construct instance

Planted: lowering chose `nat_mul` for every `nat_add` term
(`Prim::NatAdd => Item::NatMul` in `lower.rs`). Many fixtures use both
primitives, so a check of the program's element set as a whole would admit
it. Command: `cargo test -p repo-conformance --test conformance -- conformance_rb_01`. Expected: the call is refused because its own origin
is `prim:nat_add`.

```text
thread 'conformance_rb_01' (11292) panicked at crates/conformance/src/cases/rust_backend.rs:514:40:
adt-evaluation (rust-std): the construct `call:runtime:nat_mul` does not realize `prim:nat_add`, the element it was lowered from
```

A second plant chose the `u16` checked addition for every `u8` one:

```text
thread 'conformance_rb_01' (12208) panicked at crates/conformance/src/cases/rust_backend.rs:514:40:
fixed-checked-narrow (rust-std): the construct `call:runtime:checked_add` works at width Some(U16), but `prim:checked_add` is at Some(U8)
```

Removed: `item` was restored; `conformance_rb_01` passes. `RB-01` also
plants both mutations on a lowered crate directly.

### rust closure correspondence admits an inline closure

Planted: the row of `enum:closures` named only `type:fn`, so a function
type realized by a closure the program never states a type for is
unjustified. Command: `cargo test -p repo-conformance --test conformance -- conformance_rb_01`. Expected: the fixtures whose closure types
are stated nowhere are refused.

```text
thread 'conformance_rb_01' (29092) panicked at crates/conformance/src/cases/rust_backend.rs:514:40:
closure-captures (rust-core): the construct `enum:closures` does not realize `expr:closure`, the element it was lowered from
```

Removed: the row was restored; `conformance_rb_01` passes, and
`closure-inline` renders and runs.

### rust identifier collision can fail

Planted: the package check admitted Rust keywords as exported names
(`KEYWORDS.contains(&name) && name.is_empty()`). Command: `cargo test -p repo-conformance --test conformance -- conformance_rb_02`. Expected: the committed negative
manifest that the check exists for is packaged.

```text
thread 'conformance_rb_02' (29709) panicked at crates/conformance/src/cases/rust_backend.rs:108:13:
identifier-keyword: the negative manifest packages
```

Removed: the check was restored; `conformance_rb_02` passes.

### rust ownership mismatch can fail

Planted: the package check let an export copy a parameter of any type
(`Passing::Copy => ...`). Command: `cargo test -p repo-conformance --test conformance -- conformance_rb_03`. Expected: the committed negative
manifest that the check exists for is packaged.

```text
thread 'conformance_rb_03' (30347) panicked at crates/conformance/src/cases/rust_backend.rs:108:13:
ownership-copy-list: the negative manifest packages
```

Removed: the check was restored; `conformance_rb_03` passes.

### rust boundary type check can fail

Planted: the package check never found a function value at an export's
boundary (`Ty::Fn { .. } => false`). Command: `cargo test -p repo-conformance --test conformance -- conformance_rb_04`. Expected: the committed negative
manifest that the check exists for is packaged.

```text
thread 'conformance_rb_04' (31065) panicked at crates/conformance/src/cases/rust_backend.rs:108:13:
unsupported-function-boundary: the negative manifest packages
```

Removed: the check was restored; `conformance_rb_04` passes.

### rust boundary check reaches into records

Planted: the boundary check stopped at a named type
(`false && seen.insert(*index)` in `holds_function`). Command: `cargo test -p repo-conformance --test conformance -- conformance_rb_04`. Expected: the committed negative
manifest that the check exists for is packaged.

```text
thread 'conformance_rb_04' (31880) panicked at crates/conformance/src/cases/rust_backend.rs:108:13:
unsupported-function-in-record: the negative manifest packages
```

Removed: the check was restored; `conformance_rb_04` passes.

### rust uninhabited value refusal can fail

Planted: lowering no longer refused a call whose result type no value
inhabits (the `self.inhabited(&result)?` of `Term::Call` was deleted). Command: `cargo test -p repo-conformance --test conformance -- conformance_rb_04`. Expected: the committed negative
manifest that the check exists for is packaged.

```text
thread 'conformance_rb_04' (32579) panicked at crates/conformance/src/cases/rust_backend.rs:108:13:
unsupported-uninhabited-value: the negative manifest packages
```

Removed: the check was restored; `conformance_rb_04` passes.

### rust declared failure check can fail

Planted: the package check admitted an export declaring no errors for a
function that can overflow (`(Errors::None, true) if false`). Command: `cargo test -p repo-conformance --test conformance -- conformance_rb_05`. Expected: the committed negative
manifest that the check exists for is packaged.

```text
thread 'conformance_rb_05' (759) panicked at crates/conformance/src/cases/rust_backend.rs:108:13:
arithmetic-undeclared-overflow: the negative manifest packages
```

Removed: the check was restored; `conformance_rb_05` passes.

### rust version check can fail

Planted: the version check read each part as a `u128`, so a part above
`u64::MAX`, which Cargo refuses, passed. Command: `cargo test -p repo-conformance --test conformance -- conformance_rb_07`. Expected: the committed negative
manifest that the check exists for is packaged.

```text
thread 'conformance_rb_07' (1325) panicked at crates/conformance/src/cases/rust_backend.rs:108:13:
version-overflow: the negative manifest packages
```

Removed: the check was restored; `conformance_rb_07` passes.

### rust package lint gate can fail

Planted: the generated `Cargo.toml` set Clippy's default lints to `allow`,
and the packages were regenerated. Command: `cargo test -p repo-conformance --test conformance -- conformance_rb_06`. Expected: the package
with a planted clone of a `Copy` value passes Clippy.

```text
thread 'conformance_rb_06' (10515) panicked at crates/conformance/src/cases/rust_backend.rs:1035:13:
the planted lint is refused
```

Removed: the generator was restored and the packages regenerated;
`conformance_rb_06` passes.

### rust lint exceptions and lowering rules are load-bearing

Each of the ten exceptions of `package::ALLOWED_LINTS` was removed in turn
(its entry renamed to the pedantic `too_many_lines`), and each rendering rule
of §17.16 **Lowering** was disabled in turn; after each plant the packages
were regenerated and `conformance_rb_06` (with `conformance_rb_01`) ran.
Expected: a committed package fails its gate, so no exception or rule is
admitted without a fixture that needs it. Every plant was refused; the
first lint or build error of each:

| Exception removed, or rule disabled | First refusal |
| --- | --- |
| `type_complexity` | `very complex type used` |
| `too_many_arguments` | `this function has too many arguments (8/7)` |
| `large_enum_variant` | `large size difference between variants` |
| `result_large_err` | `the Err-variant returned from this function is very large` |
| `result_unit_err` | ``this returns a `Result<_, ()>` `` |
| `single_match` | ``you seem to be trying to use `match` for an equality check`` |
| `manual_unwrap_or` | ``this pattern reimplements `Option::unwrap_or` `` |
| `manual_unwrap_or_default` | ``match can be simplified with `.unwrap_or_default()` `` |
| `manual_map` | ``manual implementation of `Option::map` `` |
| `manual_ok_err` | ``manual implementation of `ok` `` |
| unit result written `-> ()` | `unneeded unit return type` |
| unit block value written `()` | `unneeded unit expression` |
| unit binder bound by name | ``unused variable: `v0` `` |
| unit capture read through its reference | `passing a unit value to a function` |
| computed unit operand passed directly | `passing a unit value to a function` |
| computed unit returned as `Ok(e)` | `passing a unit value to a function` |
| unit dispatch result wrapped as `Ok(f(..))` | `passing a unit value to a function` |
| empty unit `else` written | ``this `else` branch is empty`` |
| inner `if` of an `if` without `else` not bound | ``this `if` statement can be collapsed`` |
| literal Boolean branches kept | `this if-then-else expression returns a bool literal` |
| equal branches kept | ``this `if` has identical blocks`` |
| literal condition not bound | ``this `if` has identical blocks`` |
| rebuilding match kept | `this match expression is unnecessary` |
| binding returned by its block kept | ``returning the result of a `let` binding from a block`` |
| computed record matched in place | ``in a `match` scrutinee, avoid complex blocks`` |
| zero test negated as `Not(m == 0)` | `RB-01`: `a negated zero test or predecessor` (before that refusal: no run package emitted a nonzero test, and an export and the denotation disagreed on `!m == 0`) |
| uninhabited parameter's body rendered | `unreachable definition` |
| uninhabited arm rendered | `unreachable definition` |

The transcripts are of the form:

```text
thread 'conformance_rb_06' (18145) panicked at crates/conformance/src/cases/rust_backend.rs:905:13:
a package fails its lint gate:
error: very complex type used. Consider factoring parts into `type` definitions
```

Removed: every exception and rule was restored and the packages
regenerated; `conformance_rb_06` passes.

### rust runtime mutation is detected by the differential

Planted: `parse_int` returned `Ok(None)` for a decimal outside `i64`
(`Some(None) => Ok(None)` in the runtime). Command: `cargo test -p repo-conformance --test conformance -- conformance_rb_06`. Expected: the
primitive differential, which runs every primitive instance on its
boundary and seeded inputs, finds the inputs whose denotation overflows.

```text
thread 'conformance_rb_06' (2037) panicked at crates/conformance/src/cases/rust_backend.rs:962:17:
primitives_std: the rendering and the denotation disagree on 3 of 51441 runs: ["run 38634: {\"kind\":\"none\"} != {\"kind\":\"overflow\"}", "run 38650: {\"kind\":\"none\"} != {\"kind\":\"overflow\"}", "run 38656: {\"kind\":\"none\"} != {\"kind\":\"overflow\"}"]
```

A second plant computed `int_rem` as `a.checked_rem(b).unwrap_or(z)`, so
`i64::MIN rem -1` returned the default:

```text
thread 'conformance_rb_06' (17475) panicked at crates/conformance/src/cases/rust_backend.rs:962:17:
primitives_core: the rendering and the denotation disagree on 1 of 39092 runs: ["run 8393: {\"kind\":\"int\",\"value\":\"-9223372036854775807\"} != {\"kind\":\"int\",\"value\":\"0\"}"]
```

Removed: the runtime was restored; `conformance_rb_06` passes.

### rust lowering mutation is detected

Planted: a closure's dispatch passed its captures in reverse order
(`.rev()` in the printer of `Fn<n>::apply`), and, separately, a field
projection bound the mirrored field (`if field == types.len() - 1 -
selected`); the packages were regenerated each time. Command: `cargo test -p repo-conformance --test conformance -- conformance_rb_06`.
Expected: a run package computes another value. At `20b4ed6`, whose
fixtures captured and projected only naturals, both were caught as the
export and the denotation disagreeing (`closure_captures_core` and
`record_fields_core`). The fixtures now also capture a number with a string
and project from records of mixed types, so the same mutations no longer
type check and the build refuses them first:

```text
thread 'conformance_rb_06' (3563) panicked at crates/conformance/src/cases/rust_backend.rs:889:13:
a package does not build:
error[E0308]: arguments to this function are incorrect
```

and for the projection:

```text
thread 'conformance_rb_06' (6453) panicked at crates/conformance/src/cases/rust_backend.rs:889:13:
a package does not build:
error[E0614]: type `u64` cannot be dereferenced
```

Removed: the lowering was restored and the packages regenerated;
`conformance_rb_06` passes.

### rust provenance binding can fail

Planted: the provenance hashed only the core runtime for every profile,
and the packages were regenerated. Command: `cargo test -p repo-conformance --test conformance -- conformance_rb_07`. Expected: a `rust-std`
package's provenance no longer records the runtime LexLean's semantics
records.

```text
thread 'conformance_rb_07' (28593) panicked at crates/conformance/src/cases/rust_backend.rs:1179:17:
assertion `left == right` failed: compiler/rust/rust-std/adt-evaluation: the runtime is the one LexLean's semantics records
```

Removed: the hash was restored; `conformance_rb_07` passes.

### rust cross-root determinism can fail

Planted: the provenance's version carried the renderer's working
directory (`format!("{}+{}", manifest.version, current_dir)`), and the
packages were regenerated. Command: `cargo test -p repo-conformance --test conformance -- conformance_rb_07`. Expected: the two renderer
processes, each run with its own working directory and environment, write
different bytes.

```text
thread 'conformance_rb_07' (13568) panicked at crates/conformance/src/cases/rust_backend.rs:1129:17:
assertion `left == right` failed: rust-core/boolean-shapes/provenance.json: the renderers of two roots disagree
```

Removed: the provenance was restored and the packages regenerated;
`conformance_rb_07` passes.

### rust source binding can fail

Planted: the package generator wrote the zero identity as every package's
`sources`, and the packages were regenerated. Command: `cargo test -p repo-conformance --test conformance -- conformance_rb_07`. Expected: no
package binds the verified build that states its program.

```text
thread 'conformance_rb_07' (30490) panicked at crates/conformance/src/cases/rust_backend.rs:1186:46:
compiler/rust/rust-std/adt-evaluation: sources ["0000000000000000000000000000000000000000000000000000000000000000"] are not the semantic ID cd690f0a5afba87d8ddeebfb51773d2d833d2d9b0f897cc9c5702224d8b1994d of the verified compiler build
```

A second plant made the binding check accept any single source
(`sources.len() != 1`); the forged-identity check refused it:

```text
thread 'conformance_rb_07' (19913) panicked at crates/conformance/src/cases/rust_backend.rs:1199:69:
forged: ()
```

Removed: the generator and the check were restored and the packages
regenerated; `conformance_rb_07` passes.

### rust runtime identity record can fail

Planted: `nat_sub` wrapped (`a.wrapping_sub(b)`) without updating
`language/semantics-1.2.toml`. Command: `cargo test -p lexlean --lib
the_compiler_semantics_records_the_runtime`. Expected: the runtime no longer
has the digest LexLean's compiler semantics records.

```text
thread 'calculus::rust::runtime::tests::the_compiler_semantics_records_the_runtime' (16500) panicked at crates/lexlean/src/calculus/rust/runtime.rs:603:13:
assertion `left == right` failed: the runtime changed without its record `rust_runtime_core` in language/semantics-1.2.toml
  left: Some("7f18f40838cbf5b74494efecbd3553c16cd6b0fb42445bdff4c8036b5f0e2a95")
 right: Some("c87e438f590444f4c7689a09cca2110d37fea6c32564a767037c9d8647a0fe04")
```

Removed: the runtime was restored; the test passes.

### rust renderer identity record can fail

Planted: `negate` lost its zero-test arm in
`crates/lexlean/src/calculus/rust/lower.rs`, without updating
`rust_renderer` in `language/semantics-1.2.toml`. Command: `cargo test -p
lexlean --lib the_compiler_semantics_records_the_renderer`. Expected: the
renderer's sources no longer have the digest LexLean's compiler semantics
records, so the change cannot keep LexLean's identity.

```text
thread 'calculus::rust::tests::the_compiler_semantics_records_the_renderer' (9715) panicked at crates/lexlean/src/calculus/rust/mod.rs:244:9:
assertion `left == right` failed: the renderer changed without its record `rust_renderer` in language/semantics-1.2.toml
```

The same plant shows the negated zero test is refused before any package is
built (command `cargo test -p repo-conformance --test conformance --
conformance_rb_01`):

```text
thread 'conformance_rb_01' (10406) panicked at crates/conformance/src/cases/rust_backend.rs:514:40:
boolean-shapes (rust-core): a negated zero test or predecessor: a zero test is negated as its complement, and a predecessor is not a Boolean
```

Removed: `negate` was restored; both tests pass.

### GNAF kernel oracle can fail

Planted: the host's `lexlean::gnaf::expand` dispatched only from a plan to
itself or a later one (`for large in small..count`), dropping
`dispatch 3 1 0`, and `cargo xtask check-calculus --write` regenerated the
GNAF fixtures from it. No answer of the two-plan requests changes, because
the dropped system is never optimal; the oracle is Lean's kernel reducing the
`Gnaf` model, whose expansion still has it. Command: `lexlean verify` in
`compiler/`. Expected: every request's stated universe and every answered
request's stated statuses are refused (the first two of fifty-four errors):

```text
error[LLV7002]: Lean rejected `Compiler.GnafFixtures` (error): Tactic `rfl` failed: The left-hand side
  Gnaf.universeOf argminCompleteRequest
is not definitionally equal to the right-hand side
  [Gnaf.Selector.fixed 0, Gnaf.Selector.fixed 1, Gnaf.Selector.dispatch 3 0 0, Gnaf.Selector.dispatch 3 0 1,
    Gnaf.Selector.dispatch 3 1 1]
error[LLV7002]: Lean rejected `Compiler.GnafFixtures` (error): Tactic `rfl` failed: The left-hand side
  Gnaf.statusesOf argminCompleteRequest
is not definitionally equal to the right-hand side
  [(0, Gnaf.Status.admitted 180 15), (1, Gnaf.Status.admitted 205 26), (2, Gnaf.Status.admitted 235 22),
    (3, Gnaf.Status.admitted 167 46), (4, Gnaf.Status.admitted 260 33)]
```

Removed: `expand` was restored and the fixtures regenerated; `compiler/`
verifies and `conformance_gn_02` passes.

### GNAF completeness theorem can fail

Planted: the model's `dispatchSmalls` paired each small plan only with the
plans after it (`dispatchOver threshold small rest`), so the expansion lost
every dispatch to an earlier plan or to itself, and
`cargo xtask check-calculus --write` regenerated `Gnaf`. Command:
`lexlean verify` in `compiler/`. Expected: the lemma `dispatchInSmalls`,
on which `expandComplete` rests, no longer closes:

```text
error[LLV7002]: Lean rejected `Compiler.Gnaf` (error): unsolved goals
case cons
threshold small large level : Nat
larges : List Nat
head : Nat
```

Removed: `dispatchSmalls` was restored and `Gnaf` regenerated; `compiler/`
verifies and `conformance_gn_03` passes.

### GNAF ties can fail

Planted, separately: (1) the model's `attaining` kept only the first row at
the least cost; (2) the host's `argmin` kept only the first identity at the
least cost (`.take(1)`); (3) the model's `frontierFrom` dropped a row whose
cost a later row equals. Each was regenerated with
`cargo xtask check-calculus --write`. Command: `lexlean verify` in
`compiler/`. Expected: the kernel refuses each, through the authority's own
tie in GNAF-VEC-17 for (1) and through the duplicated-plan fixtures
(GNAF-REJ-21) for (2) and (3):

```text
error[LLV7002]: Lean rejected `Compiler.Gnaf` (error): Tactic `rfl` failed: The left-hand side
  argmin Nat uniformRows
is not definitionally equal to the right-hand side
  some ([0, 1], 10)
```

```text
error[LLV7002]: Lean rejected `Compiler.GnafFixtures` (error): Tactic `rfl` failed: The left-hand side
  Gnaf.answer argminTieRequest
is not definitionally equal to the right-hand side
  Gnaf.Answer.argmin [4] 167
```

```text
error[LLV7002]: Lean rejected `Compiler.GnafFixtures` (error): Tactic `rfl` failed: The left-hand side
  Gnaf.answer frontierTieRequest
is not definitionally equal to the right-hand side
  Gnaf.Answer.frontier [0, 4, 5]
```

Removed: each definition was restored and the project regenerated;
`compiler/` verifies and `conformance_gn_02` passes.

### GNAF reachable size can fail

Planted: the model's `status` sized a system by every function of its
realization (`reachableSize` over `range (length functions)`), the shared
plans it cannot run included, and `cargo xtask check-calculus --write`
regenerated `Gnaf`. Command: `lexlean verify` in `compiler/`. Expected:
every stated status list is refused, and the posed GNAF-VEC-02 frontier
collapses (seventeen errors, three of them):

```text
error[LLV7002]: Lean rejected `Compiler.GnafFixtures` (error): Tactic `rfl` failed: The left-hand side
  Gnaf.answer vec02ParetoEnvelopeRequest
is not definitionally equal to the right-hand side
  Gnaf.Answer.frontier [0, 1, 2]
error[LLV7002]: Lean rejected `Compiler.GnafFixtures` (error): Tactic `rfl` failed: The left-hand side
  Gnaf.certifies vec02ParetoEnvelopeRequest (Gnaf.Answer.frontier [2, 1, 0])
is not definitionally equal to the right-hand side
  true
error[LLV7002]: Lean rejected `Compiler.GnafFixtures` (error): Tactic `rfl` failed: The left-hand side
  Gnaf.minimaAttained vec02ParetoEnvelopeRequest
is not definitionally equal to the right-hand side
  false
```

Removed: `status` was restored and `Gnaf` regenerated; `compiler/`
verifies and `conformance_gn_06` passes.

### GNAF hidden-cost rejection can fail

Planted: the host's `check_performed` accepted a `free` charge for an action
systems perform. Command: `cargo test -p repo-conformance --test
conformance -- conformance_gn_04`. Expected: a request that declares
observation free is answered instead of refused.

```text
thread 'conformance_gn_04' panicked at crates/conformance/src/cases/gnaf.rs:1060:9:
assertion `left == right` failed: reject-free-observation
```

Removed: the charge rule was restored; `conformance_gn_04` passes.

### GNAF per-plan preparation can fail

Planted: the host's `system_charge` charged every plan's preparation to
every system (`(0..=plans.len())` in place of the reachable functions), the
uniform shift the review found. Command: `cargo test -p repo-conformance
--test conformance -- conformance_gn_04`. Expected: the fixed recursion
system, which cannot run the slicing plan, is charged its preprocessing.

```text
thread 'conformance_gn_04' panicked at crates/conformance/src/cases/gnaf.rs:1111:9:
assertion `left == right` failed: Fixed { plan: 0 }
```

Removed: the reachable plans were restored; `conformance_gn_04` passes.

### GNAF prepared-artifact binding can fail

Planted: the host's `check_declared` admitted a `free` preparation on a
prepared boundary whether or not an artifact it prepared is bound
(`(!bound).then_some(..)` replaced by `None`). Command: `cargo test -p
repo-conformance --test conformance -- conformance_gn_04`.

```text
thread 'conformance_gn_04' panicked at crates/conformance/src/cases/gnaf.rs:1060:9:
assertion `left == right` failed: reject-unbound-prepared-state
```

Removed: the binding rule was restored; `conformance_gn_04` passes.

### GNAF operand-size declaration can fail

Planted: the host's `validate` skipped the operand-size rule
(`if false && request.machine.operand_size == OperandSize::Unit`). Command:
`cargo test -p repo-conformance --test conformance -- conformance_gn_04`.

```text
thread 'conformance_gn_04' panicked at crates/conformance/src/cases/gnaf.rs:1060:9:
assertion `left == right` failed: reject-unit-cost-operands
```

Removed: the rule was restored; `conformance_gn_04` passes.

### GNAF machine capacity can fail

Planted, separately: (1) the host's `check_request_capacity` never refused
(`.filter(|_| false)`); (2) the host's `check_capacity` admitted any declared
charge (`if false && cost > HOST_CAPACITY.charge`). Command: `cargo test -p
repo-conformance --test conformance -- conformance_gn_04` and
`-- conformance_gn_01`. Expected: (1) a request using more fuel than its
machine's capacity is answered; (2) a request charging 2^40 per invocation
loads, so its cost could leave the host's integers.

```text
thread 'conformance_gn_04' panicked at crates/conformance/src/cases/gnaf.rs:1060:9:
assertion `left == right` failed: reject-beyond-capacity
```

```text
thread 'conformance_gn_01' panicked at crates/conformance/src/cases/gnaf.rs:90:28:
an invalid request loads: expected `charge`
```

Removed: both checks were restored; `conformance_gn_01` and
`conformance_gn_04` pass.

### GNAF universe identity can fail

Planted: the host's `load` skipped the comparison of the stated universe
identity with the recomputed one (`if false && request.universe !=
identity`). Command: `cargo test -p repo-conformance --test conformance --
conformance_gn_01`. Expected: a request stating another universe's identity
loads.

```text
thread 'conformance_gn_01' panicked at crates/conformance/src/cases/gnaf.rs:90:28:
an invalid request loads: expected `universe identity`
```

Removed: the comparison was restored; `conformance_gn_01` passes.

### GNAF reference definition can fail

Planted: the host's `load` skipped running the reference on the domain
(`.filter(|_| false)` before the search for an argument without a value).
Command: `cargo test -p repo-conformance --test conformance --
conformance_gn_01`. Expected: a request whose fuel leaves the reference
exhausted loads, its expected values undefined.

```text
thread 'conformance_gn_01' panicked at crates/conformance/src/cases/gnaf.rs:90:28:
an invalid request loads: expected `the reference returns no value (exhausted)`
```

Removed: the check was restored; `conformance_gn_01` passes.

### GNAF claim alias can fail

Planted: the host's `check_claim` accepted `restricted_universe_optimal`
(`=> None`). Command: `cargo test -p repo-conformance --test conformance --
conformance_gn_05`.

```text
thread 'conformance_gn_05' panicked at crates/conformance/src/cases/gnaf.rs:1197:9:
assertion `left == right` failed: reject-restricted-universe-alias
```

Removed: the alias rule was restored; `conformance_gn_05` passes.

### GNAF evaluation panic can fail

Planted: a panic of the evaluation thread was reported as the platform's
`LLV7010`. Command: `cargo test -p lexlean --lib gnaf`. Expected: the
planted panic surfaces as a platform failure.

```text
thread 'gnaf::tests::a_panicking_evaluation_is_an_internal_failure' (28684) panicked at crates/lexlean/src/gnaf.rs:1769:9:
assertion `left == right` failed
  left: ["LLV7010"]
 right: ["LLI9001"]
```

Removed: the panic is `LLI9001` again; the test passes.

### GNAF authority vector statements can fail

Planted: GNAF-VEC-02's `rD` was transcribed as `(3, 4)` instead of `(3, 3)`
in `crates/conformance/src/gnaf_model.rs`, and
`cargo xtask check-calculus --write` regenerated `Gnaf`. The frontier is
still `{rA, rB, rC}`, so Lean accepts the theorem; the check is the
statement's own numbers. Command: `cargo test -p repo-conformance --test
conformance -- conformance_gn_07`.

```text
thread 'conformance_gn_07' panicked at crates/conformance/src/cases/gnaf.rs:1540:5:
assertion `left == right` failed
  left: [(0, [1, 3]), (1, [2, 2]), (2, [3, 1]), (3, [3, 4])]
 right: [(0, [1, 3]), (1, [2, 2]), (2, [3, 1]), (3, [3, 3])]
```

Removed: the transcription was restored and `Gnaf` regenerated;
`conformance_gn_07` passes.

### GNAF dependency manifest can fail

Planted: the manifest's generator listed `restricted_universe_optimal` among
the strongest claims, and `cargo xtask check-calculus --write` regenerated
`compiler/gnaf.manifest.json`. Command: `cargo test -p repo-conformance
--test conformance -- conformance_gn_08`. Expected: the manifest claims a
class the model refuses.

```text
thread 'conformance_gn_08' panicked at crates/conformance/src/cases/gnaf.rs:1829:5:
assertion `left == right` failed
  left: [("global_optimal", "scalar"), ("argmin_complete", "scalar"), ("restricted_universe_optimal", "scalar"), ("pareto_optimal", "vector"), ("frontier_complete", "vector")]
 right: [("global_optimal", "scalar"), ("argmin_complete", "scalar"), ("pareto_optimal", "vector"), ("frontier_complete", "vector")]
```

Removed: the generator was restored and the manifest regenerated;
`conformance_gn_08` passes.

### vendored authority checksum can fail

Planted: the vendored `model/authorities/UOR-GNAF-v1-draft.2.md` was edited
from `Normative Draft 0.2` to `Normative Draft 0.3`. Command:
`cargo xtask validate-model`.

```text
gate failed: model is inconsistent: UOR-GNAF-1-DRAFT-2: model/authorities/UOR-GNAF-v1-draft.2.md hashes to 98d97c4b3ec55573fefb9986a1b821abad032fbd27e716cad26735f8ade999e1, not its checksum 5c342373b2ff809bfd607c413cafd0582d32bb097544c6597ff7d674fe99200a (R6)
```

Removed: the vendored bytes were restored; `validate-model` is clean.

### check-calculus covers the GNAF fixtures

Planted: the committed answer of `compiler/gnaf/argmin-over-systems.json`
was edited from 167 to 166 steps. Command: `cargo xtask check-calculus`.

```text
gate failed: compiler/gnaf/argmin-over-systems.json differs from its generator; run `cargo xtask check-calculus --write`
```

Removed: the committed bytes were restored; the gate reports 262 generated
files equal to their generator.

### check-calculus covers the Gnaf model

Planted: the theorem `expandComplete` of the committed
`compiler/src/Gnaf.lex.tex` was renamed `expandComplett`. Command:
`cargo xtask check-calculus`. Expected: the committed model no longer equals
what `crates/conformance/src/gnaf_model.rs` renders.

```text
gate failed: compiler/src/Gnaf.lex.tex differs from its generator; run `cargo xtask check-calculus --write`
```

Removed: the committed module was restored; the gate reports 262 generated
files equal to their generator.

### check-calculus covers the GNAF schemas

Planted: the committed `schemas/gnaf-request.schema.json` admitted a third
operand-size treatment, `free`. Command: `cargo xtask check-calculus`.
Expected: the schema no longer equals what its generator renders from the
request types and `schemas/target-program.schema.json`.

```text
gate failed: schemas/gnaf-request.schema.json differs from its generator; run `cargo xtask check-calculus --write`
```

Removed: the committed schema was restored; the gate reports 262 generated
files equal to their generator.

### language-1.2 imported runtime reduction can fail

Planted: language-1.2 modules emitted the frozen language-1.1 runtime, whose
`@[noinline]` primitives are not exposed to other modules. The oracle is
Lean's kernel. Command: `lexlean verify` in `compiler/`. Expected: a fixture
module can no longer reduce `run`, which `TargetSemantics` defines through
`LexLeanRuntime.index`.

```text
error[LLV7002]: Lean rejected `LexLeanTarget.TargetFixtures` (error): Tactic `rfl` failed: The left-hand side
  adtEvaluationRun
is not definitionally equal to the right-hand side
```

Removed: the exposed runtime was restored; `compiler/` verifies.

### graph node set can fail

Planted: `graphTopological` ordered only the graph's keys
(`let nodes := graph.map Prod.fst`), so a successor inserted without an entry
of its own was silently dropped from the order. Command: `cargo test -p
repo-conformance --test conformance -- conformance_sm_30`. Expected: the
seeded graphs and the 24-node chain, whose last node is such a successor,
disagree with the independent model.

```text
thread 'conformance_sm_30' panicked at crates/conformance/src/support.rs:1903:10:
the module verifies with real Lean: LexLeanError { class: Language, diagnostics: [Diagnostic { code: DiagnosticCode("LLV7002"), message: "Lean rejected `Collections.Main` (error): Tactic `decide` proved that the proposition\n  LexLeanCollections.graphTopological
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 242 filtered out
```

Lean refused twelve topological-order theorems. Removed: the node set was
restored to keys and successors; `conformance_sm_30` passes.

### key order agreement can fail

Planted: linking's `KeyOrder` compared negative integers by magnitude
(`(true, true) => magnitude` instead of `magnitude.reverse()`), so a literal's
canonical order disagreed with Lean's `Key Int` instance. Command: `cargo test
-p repo-conformance --test conformance -- conformance_sm_28`. Expected: the
`key_order_*` theorems, which state that a linked literal equals Lean's own
insertion of the source order, fail under verification.

```text
thread 'conformance_sm_28' panicked at crates/conformance/src/support.rs:1903:10:
the module verifies with real Lean: LexLeanError { class: Language, diagnostics: [Diagnostic { code: DiagnosticCode("LLV7002"), message: "Lean rejected `Collections.Main` (error): Tactic `decide` proved that the proposition\n  [-2, -10, -100, 0, 3, 9, 100] =\n    LexLeanCollections.listFold (fun built element => LexLeanCollections.setInsert built element) []\n      [3, -2, 0, -10, 100, -100, 9]\nis false"
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 242 filtered out
```

Lean refused five theorems, one per signed key type. Removed: the order was
restored; `conformance_sm_28` passes.

### collection canonical invariant can fail

Planted: the runtime's `setUnion` appended each new element
(`if containsElement key acc then acc else acc ++ [key]`), so a union was no
longer strictly ascending. Command: `cargo test -p repo-conformance --test
conformance -- conformance_sm_28`. Expected: the seeded union theorems,
whose right-hand sides come from `BTreeSet`, fail under verification.

```text
thread 'conformance_sm_28' panicked at crates/conformance/src/support.rs:1903:10:
the module verifies with real Lean: LexLeanError { class: Language, diagnostics: [Diagnostic { code: DiagnosticCode("LLV7002"), message: "Lean rejected `Collections.Main` (error): Tactic `decide` proved that the proposition\n  LexLeanCollections.setUnion [1, 2, 3] [0, 5] = [0, 1, 2, 3, 5]\nis false"
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 242 filtered out
```

Lean refused eleven theorems: `op_set_union` and ten of the twelve seeded
`model_set_union_*` theorems. Removed: the union was restored;
`conformance_sm_28` passes.

### fmt-check can fail

Planted: `fn   badly_formatted( ) {}` appended to `crates/model/src/release.rs`. Command: `cargo fmt --all -- --check`. Expected: a formatting diff and a nonzero exit.

```text
Diff in crates/model/src/release.rs:569:
         );
     }
 }
-fn   badly_formatted( ) {}
+fn badly_formatted() {}

exit=1
```

Removed: the line was deleted; `cargo fmt --all -- --check` is silent.

### validate-model can fail

Planted: the generated `CONFORMANCE.md` edited so that `RP-01`'s level cell reads `open`. Command: `cargo xtask validate-model`. Expected: the committed document no longer equals regeneration (R1).

```text
gate failed: CONFORMANCE.md is stale: it disagrees with model/*.toml (R1). Run `just model-write`.
```

Removed: the document was restored; the gate reports documents current.

### audit-deferral can fail

Planted: a deferral marker (spelled here in halves: `TO` + `DO`) appended as a comment to the `Justfile`, which is inside the audit's root-tooling scope. Command: `cargo xtask validate-model`. Expected: R4 names the file and line.

```text
gate failed: R4: nothing is deferred. None of TODO, FIXME, XXX, unimplemented!, todo!, for now, later version may appear outside a code span.

Justfile:86: # TODO: tighten this later
```

Removed: the comment was deleted; `audit-deferral` reports nothing deferred.

### audit-errors can fail

Planted: the literal `"LLL1004"` in `crates/conformance/src/cases/lexical_closure.rs` changed so that its third digit is `9` (a well-shaped but unregistered code) outside any `code!(` invocation. Command: `cargo xtask validate-model`. Expected: R5 rejects the unregistered literal at its line.

```text
gate failed: R5: crates/conformance/src/cases/lexical_closure.rs:225: `LLL1904` is not a registered diagnostic code (§26.1)
```

Removed: the literal was restored; `audit-errors` reports every registered code as constructed by the shipped crate, and no unregistered literal anywhere.

### audit-shipped can fail

Planted: the in-crate link `crates/lexlean/schemas` repointed at `../../tests`. Command: `cargo xtask validate-model`. Expected: R6 reports the link resolves elsewhere than the repository's `schemas/`.

```text
gate failed: R6: crates/lexlean/schemas resolves to <root>/tests rather than schemas; the crate must embed the repository's own normative data
```

Removed: the link was restored to `../../schemas`; `audit-shipped` reports the links resolve.

### audit-generated can fail

Planted: `schemas/coverage.schema.json` re-serialized with indentation (no longer canonical JSON). Command: `cargo xtask validate-model`. Expected: the schema is rejected as non-canonical.

```text
gate failed: R10: <root>/schemas/coverage.schema.json is not canonical JSON; regenerate the schema
```

Removed: the schema bytes were restored; `audit-generated` reports 12 schemas canonical.

### audit-language-closure can fail

Planted: a `[[token]]` row `orphan-token` appended to `language/renderer-tokens.toml` that no preamble construct or LRE references. Command: `cargo xtask validate-model`. Expected: R8 rejects the unused registry row (§13.10).

```text
gate failed: R8: unused registry rows fail the language audit (§13.10): ["orphan-token"]
```

Removed: the row was deleted; the audit reports the registry equal to the referenced closure, with no unused row.

### audit-no-unsafe can fail

Planted: the `#![forbid(unsafe_code)]` line removed from `crates/lexlean/src/lib.rs`. Command: `cargo xtask validate-model`. Expected: the audit reports the crate-level prohibition missing (RP-09).

```text
gate failed: R6: crates/lexlean/src/lib.rs must carry the crate-level prohibition
```

An `unsafe` block planted in `crates/lexlean/src/error.rs` instead does not even reach the audit: `rustc` refuses the crate under the prohibition (`error: usage of an `unsafe` block`, exit 101). Removed: the attribute was restored; the audit reports the prohibition active.

### audit-surface-disjointness can fail

Planted: `language/std/int/entries/add.toml`'s math surface changed from `intplus` to `+`, which `lexlean.std.nat::add` already owns. Command: `cargo xtask validate-model`. Expected: R7 names both owners and why a second one matters.

```text
gate failed: R7: the surface `+` is owned by 2 entries in the Math channel (lexlean.std.int::add, lexlean.std.nat::add); `fmt` spells a surface bare only when one visible entry owns it, so a second owner changes canonical output and breaks §30.2 byte-compatibility
```

Removed: the surface was restored; the audit reports 351 spellable surfaces with no two entries sharing one in a channel. The three parser-layer overlaps it counts separately (`-` between `lexlean.core::hyphen` and `lexlean.std.nat::sub`, and `cases`/`induction` between a grammar and a structural entry) are reported rather than hidden, because `structural` and `grammar` entries are never resolved as term atoms.

### audit-atlas-registers can fail

Planted: the native Atlas source declaration `UorAtlas.Scales.S37` renamed to `UorAtlas.Scales.MissingS37`. Command: `cargo xtask validate-model`. Expected: R4 names the live source obligation.

```text
gate failed: R4: the Atlas register has 1 live label(s) absent from the native Atlas source: S37; a live source row is an obligation, not an optional description of what was migrated
```

Renaming a source support declaration to `T48` (retracted), `T10` (superseded), or `L1` (non-denotable) fails on that exact disposition. The register keys on exact identifiers, so `T57a`, `T10a`, and `F12` remain live even though their prefixes are withheld.

The same gate refuses two native declarations whose final segment is the same live label. Planted: a second fully qualified declaration ending in `T5`.

```text
gate failed: R4: `T5` is declared as both `UorAtlas.Roots.T5` and `UorAtlas.Planted.T5` in the native Atlas source; one label has one declaration
```

Removed: the source bytes were restored; the audit reports every live label rooted in the native Atlas source, including `S37`, `S38`, and the corrected integer statement `S43`.

### audit-authority-scope can fail

Planted: an authority row citing the native Atlas source, which is repository content. Command: `cargo xtask validate-model`. Expected: R2 refuses the row.

```text
gate failed: R2: authority `ATLAS-LIBRARY` cites repository content; what this repository builds is a `build` claim with a conformance ID, never a `some-true` citation (§27.4)
```

A citation naming an existing repository path fails the same way, naming the path. The check reads each citation up to its first semicolon, because a legitimate row may go on to name repository fixtures as the evidence a third party compares against — `PRINT-AXIOMS-4-32-1` does exactly that with `tests/golden/axiom-parser/`, and must not be refused for it. Removed: the row was deleted; the audit reports four rows, none citing repository content.

### audit-atlas-denotations can fail

Planted, in the first direction: `atlas-t5.toml`'s component renamed to `UorAtlas.Roots.T5x9`, a declaration the native source does not make. Command: `cargo xtask validate-model`. Expected: R2 names the entry file and the dangling name.

```text
gate failed: R2: `atlas-t5.toml` refers to `UorAtlas.Roots.T5x9` in `Atlas`, which the native Atlas source does not declare
```

Planted, in the second direction: `atlas-a1.toml` deleted, so the native source owns `A1` and no frozen entry denotes it. Command: `cargo xtask validate-model`. Expected: R4 names the label.

```text
gate failed: R4: the native Atlas source declares live label `A1` and no frozen entry refers to it
```

Planted, on disposition: an entry `atlas-t48` for `T48`, which section 20.1 retracted, and an entry `atlas-l1` for `L1`, which the document declares non-denotable. Command: `cargo xtask validate-model`. Expected: R2 names the label and the disposition the register records.

```text
gate failed: R2: `atlas-t48.toml` is an entry for `T48`, which the register records as retracted; a document could cite it as though it stood
gate failed: R2: `atlas-l1.toml` is an entry for `L1`, which the register records as non-denotable; a document could cite it as though it stood
```

Removed: all restored; the audit reports every frozen declaration reference owned by the native source and every live label referred to once. The gate is two-directional on purpose: a reference with no source declaration advertises a result the corpus does not own, while a live declaration with no entry withholds it. Replacing a document reference with a `lean` denotation also fails, so the package cannot reverse the authority direction by importing an independently authored Atlas module.

The disposition arm reads the label out of the entry's own id rather than out of its denotation, so an entry for a withdrawn label is rejected even though no native declaration carries that name.

The audit also refuses to pass on an empty scan. Its first draft handed `gather` a list of extensions where that helper takes subdirectories *of* its root, so both scans found nothing and the gate reported success over zero files; it now fails when either scan is empty, because a gate that inspects nothing has not passed, it has failed to look.

### audit-atlas-exercise can fail

Planted, on prefix-freeness: `atlas-t59p`'s closing word removed, making its surface a prefix of `atlas-t59p0`'s. Command: `cargo xtask validate-model`. Expected: R7 names both entries and why a prefix is not merely untidy.

```text
gate failed: R7: `atlas-t59p`'s surface is a prefix of `atlas-t59p0`'s; a following word from another package would make a second complete parse
```

Planted, on source ownership: a second entrypoint added beside `src/Atlas.lex.tex`. Command: `cargo xtask validate-model`. Expected: R4 reports that the native source is not the sole coverage root.

```text
gate failed: R4: the Atlas example entrypoints are ["src/Atlas.lex.tex", "src/Labels.lex.tex"]; the native Atlas source must be the sole coverage root
```

Removing a theorem's value root from `proof_nodes` fails the same audit and names the theorem. Removed: both defects were restored; the audit reports the source declaration count and confirms every theorem proof root.

The prefix rule is not hygiene. `T59p`'s surface followed by `lexlean.std.nat`'s `zero` reads as `T59p0`, and the exhaustive module was rejected with `LLP2002`, two distinct linked interpretations, until every Atlas surface was given a closing word. Pairwise distinctness --- which `audit-surface-disjointness` already checks --- does not give the disjointness §2.4 makes the compatibility condition, because a surface can be extended into another by a word the closure supplies.

### honesty-vocabulary can fail

Planted: the sentence "The authority `LEAN-REL-4-32-1` proves every generated theorem." appended to `README.md`. Command: `cargo xtask validate-model`. Expected: R2 rejects assertive vocabulary about a cited authority, naming the appended line (its number is the file's last line, so it moves as the README grows).

```text
gate failed: the honesty meta-gate failed inside validate-model:

R2: README.md:134: `LEAN-REL-4-32-1` is cited, not established here, but this line says `proves`.
```

Removed: the sentence was deleted; the vocabulary check is clean.

### meta-gate can fail

Planted: the tag line of `LX-06` in `features/suites/lexical-closure.feature` changed to `@LX-06 @open`. Command: `cargo xtask validate-model`. Expected: R2 rejects the level drift between register and scenario.

```text
gate failed: the honesty meta-gate failed inside validate-model:

R2: LX-06's tag line must be exactly `@LX-06 @build`, found `@LX-06 @open` (§27.7).
```

Removed: the tag was restored. `crates/conformance/tests/bdd.rs::the_meta_gate_is_falsifiable` re-establishes this record on every run: it plants an empty test list, a reordered tag line, a drifted statement, and a pending step, and asserts each is reported; `a_hidden_conformance_test_is_flagged` plants a `cfg_attr(..., ignore)` attribute and asserts the attribute-block scanner flags it.

### model-unknown-field can fail

Planted: `priority = "high"` appended to `model/ids.toml`. Command: `cargo xtask validate-model`. Expected: the model parser rejects the unknown field (§27.5 step 1).

```text
gate failed: parsing <root>/model/ids.toml: TOML parse error at line 1264, column 1
     |
1264 | priority = "high"
     | ^^^^^^^^
unknown field `priority`, expected one of `id`, `level`, `suite`, `statement`
```

Removed: the row was deleted. `crates/model/src/lib.rs::unknown_fields_are_rejected_in_every_model_file` re-plants an unknown field at the top level and inside a row of each of the four model files on every `cargo test`.

### spec-links can fail

Planted: one word removed from the `RP-03` statement in `model/ids.toml`. Command: `cargo xtask validate-spec-links`. Expected: RP-07 reports the table/register disagreement.

```text
gate failed: RP-07: `RP-03`'s statement differs between the table and the register:
  table:    The completed repository has the required file and crate layout.
  register: The completed repository has the required layout.
```

Removed: the register row was restored; the gate reports 216 bijective rows.

### lint can fail

Planted: `fn planted_unused() { let unused_value = 1; }` appended to `crates/model/src/release.rs`. Command: `cargo clippy --workspace --all-targets --all-features -- -D warnings`. Expected: warnings are errors.

```text
error: unused variable: `unused_value`
error: function `planted_unused` is never used
error: could not compile `repo-model` (lib) due to 2 previous errors
```

Removed: the function was deleted; clippy is clean.

### test can fail

Planted: the last hex digit of the empty-input SHA-256 vector in `crates/model/src/release.rs` changed from `5` to `6`. Command: `cargo test -p repo-model --all-features`. Expected: the unit test fails on the digest.

```text
thread 'release::tests::the_local_sha256_agrees_with_the_test_vectors' panicked at crates/model/src/release.rs:558:9:
assertion `left == right` failed
  left: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
 right: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b856"
test result: FAILED. 3 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

Removed: the vector was restored. A conformance test fails the same way: `Json::to_file_bytes` changed to omit its final LF fails `conformance_ar_11` (`file form adds exactly one final LF; the hash form has none`).

### features can fail

Planted: `fn planted() -> u32 { "not a number" }` appended to `crates/conformance/tests/bdd.rs`, a test target. Command: `cargo check --workspace --all-features --all-targets`. Expected: the test target does not compile.

```text
error[E0308]: mismatched types
error: could not compile `repo-conformance` (test "bdd") due to 1 previous error
```

Removed: the function was deleted; every target checks.

### bdd can fail

Planted: the `EX-08` scenario removed from `features/suites/examples.feature`. Command: `cargo test -p repo-conformance --test bdd`. Expected: R3 reports the registered ID with no scenario.

```text
test every_id_has_a_scenario_and_a_test ... FAILED
the honesty meta-gate failed:

R3: EX-08 is registered but has no scenario in features/suites/.
test result: FAILED. 3 passed; 2 failed
```

Removed: the scenario was restored; five meta-gate tests pass.

### verify-examples can fail

Planted, first: the proof sentence of `examples/nat-add-zero/src/Main.lex.tex` changed to `Close the goal by reflexivity twice.`. Command: `cargo xtask verify-examples`. Expected: the example no longer passes the gate's first step.

```text
gate failed: nat-add-zero: fmt --check: LLF5005: not a registered proof sentence; the exact simple sentences are fixed
```

Planted, second (the record comparison itself, §29.5): the committed normalized verification record `examples/nat-add-zero/expected/verify/audit/output.txt` changed from `'LexLeanExample.Main.add_zero' does not depend on any axioms` to `'LexLeanExample.Main.add_zero' depends on axioms: [propext]`, so the source still verifies with real Lean 4.32.1 but its observed axiom audit no longer equals the committed record. Command: `cargo xtask verify-examples`. Expected: the example verifies, then the gate reports the drifted record after the examples that precede it alphabetically pass.

```text
verify-examples: list-induction verified (attestation 399964d863f7657b253d65d76eab32b26db86b7a7978f7f97a849ff8e66093ab)
verify-examples: list-induction: 7 normalized verification records equal expected/verify (§29.5)
verify-examples: nat-add-zero verified (attestation d614c14ebb6f5db3272cb6a1469309d046bf96931775e623c823c3c8ebf5a99a)
gate failed: R10: nat-add-zero: expected/verify/audit/output.txt differs from its committed oracle; a semantic change needs an explicit rewrite and review (§28.3)
```

Removed: the sentence and the record were restored; every example verifies with real Lean 4.32.1, prints its attestation ID, and its normalized records equal `expected/verify`.

### check-golden can fail

Planted: one byte appended to the committed oracle `examples/nat-add-zero/expected/build/modules/LexLeanExample/Main.lean`. Command: `cargo xtask check-golden`. Expected: the published build tree differs from the oracle (R10).

```text
gate failed: R10: nat-add-zero: expected/build/modules/LexLeanExample/Main.lean differs from its committed oracle; a semantic change needs an explicit rewrite and review (§28.3)
```

Removed: the oracle byte was restored; `check-golden` reports 6 published artifacts equal.

### check-reproducibility can fail

Planted: `publish_build` in `crates/lexlean/src/api.rs` made to write a `where.txt` containing the absolute project root into the staged build. Command: `cargo xtask check-reproducibility`. Expected: the two clean builds differ, or an absolute path is detected inside a published artifact (AR-07, AR-13).

```text
gate failed: AR-07: where.txt embeds the absolute checkout path /tmp/lexlean-build-VqKaW9
```

Removed: the write was deleted; the gate reports 6 published artifacts byte-identical across two directories.

### deny can fail

Planted: `walkdir = "2"` in the root `Cargo.toml` changed to `walkdir = "*"`. Command: `cargo deny --all-features check`. Expected: the wildcard ban fires (R6).

```text
error[wildcard]: found 2 wildcard dependencies for crate 'lexlean'
error[wildcard]: found 2 wildcard dependencies for crate 'repo-conformance'
error[wildcard]: found 1 wildcard dependency for crate 'xtask'
advisories ok, bans FAILED, licenses ok, sources ok
```

Removed: the requirement was restored; `cargo deny` reports every check ok.

### release-check can fail

Planted: a `release/` directory holding an SBOM `{"bomFormat":"CycloneDX","components":[]}` and a `checksums.txt` whose single line names `sbom.json` with an all-`f` digest, on the 0.1.0 tree. Command: `cargo xtask release-check`. Expected: the packaging round trip succeeds (it does: the packaged crate reports the same four-line identity), and every unmet §30.3/§30.4 criterion is listed by content, including the hash mismatch and the SBOM without the `lexlean` package.

```text
release-check: the packaged crate builds standalone with the same identity:
lexlean 0.1.0
language 1.0
compiler-semantics fa171c7a2d78cf17e6cb49bbec5c1eed8bee20033472b1953211104068589ba7
lean-toolchain leanprover/lean4:v4.32.1

gate failed: RP-12: the release is refused; unmet criteria:
  source-tag: the workspace version is not 1.0.0; §2.3 fixes the first complete release at 1.0.0
  source-tag: CHANGELOG.md has no `## 1.0.0` entry
  checksums: sbom.json: hash mismatch
  host-binaries: release/bin/x86_64-unknown-linux-gnu/lexlean is missing or empty
  host-binaries: release/bin/aarch64-unknown-linux-gnu/lexlean is missing or empty
  host-binaries: release/bin/x86_64-apple-darwin/lexlean is missing or empty
  host-binaries: release/bin/aarch64-apple-darwin/lexlean is missing or empty
  host-binaries: release/bin/x86_64-pc-windows-msvc/lexlean.exe is missing or empty
  crate-package: release/lexlean.crate: No such file or directory (os error 2)
  semantics-id: release/compiler-semantics-id.txt is not one 64-hex-digit line
  version-output: release/version-output.txt: No such file or directory (os error 2)
  sbom: release/sbom.json does not list the lexlean package
  ci-evidence: release/vv-evidence.txt: No such file or directory (os error 2)
```

Removed: the planted `release/` directory was deleted. Refusal remains the honest state until 1.0.0; `conformance_rp_12` additionally builds a synthetic tree without release artifacts and asserts `checksums`, `sbom`, `crate-package`, and `version-output` are each reported unmet.

### check-fixtures can fail

Planted: the mutated word `banana` in `tests/negative/unknown-word/project/src/Main.lex.tex` changed to `cherry`, so the fixture's diagnostic message no longer equals its committed expectation. Command: `cargo xtask check-fixtures`. Expected: the fixture's `expected/command.json` differs from the observed run (§28.3).

```text
gate failed: <root>/tests/negative/unknown-word/expected/command.json differs from the observed run (§28.3: golden output changes only through `just fixtures-write`)
--- expected
{"artifacts":[],"command":"check","diagnostics":[{"causes":[],"code":"LLL1004","help":[],"labels":[],"message":"`banana` is not a declared atom in any visible glossary", ...
--- observed
{"artifacts":[],"command":"check","diagnostics":[{"causes":[],"code":"LLL1004","help":[],"labels":[],"message":"`cherry` is not a declared atom in any visible glossary", ...
```

Removed: the project file was restored; `check-fixtures` reports 34 fixtures equal to their expected files. `conformance_ex_07` runs the same comparison and additionally pins exactly one prescribed diagnostic code per §28.5 rejection class.

### verification path determinism can fail

Observed in the PrismPM integration: the same generated module and pinned Lean
toolchain were verified from two fresh absolute project roots. Without an
explicit Lean package root, the two `.olean` files had the same size but
different SHA-256 digests, and their verification attestation IDs differed.
The random `.lexlean/verified/.staging-*` source path had been serialized into
the object file.

Corrected: generated-module compilation now runs `lake env lean -R
<generated-source-root> -o <olean> <source>`. `conformance_vr_05` verifies a
second fresh copy of its project and requires the attestation ID and every
generated `.olean` byte to equal the first run; it also requires each
normalized Lean process record to contain `-R $STAGING/lean-src`. Removing the
`-R` pair makes the attestation comparison fail. The restoring commit is
`d5a3403df3028bb5bd5af06ab725dbb0b7429581`; the targeted regression passed
after restoration.

### model evidence statement check can fail

Planted: the statement-exactness check of an evidence claim was skipped
(`if false { require_statement(env, theorem, &obligation, code!("LLT4009"))?; }`
at the end of `claim_obligation`). Command: `cargo test -p repo-conformance
--test conformance -- conformance_md_06`, and `cargo xtask check-fixtures`.
Expected: claims discharged by theorems that state something else link. The
first mutation `conformance_md_06` reaches is a duplicated claim, which now
fails later, on its duplicate generated name, instead of as unestablished
evidence; `model-forged-evidence` links (`check-fixtures` and
`conformance_ex_07` both fail on it).

```text
thread 'conformance_md_06' (23724) panicked at crates/conformance/src/support.rs:526:5:
expected LLT4009, found ["LLT4001"] (LLT4001: phase link: duplicate generated name `DigitEvidence.digit_net_exact`)
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 282 filtered out
```

Removed: the check was restored; `conformance_md_06` passes and
`model-forged-evidence` fails with `LLT4009`.

### model runtime boundary can fail

Planted: a contract's precondition was no longer a required runtime check
(`if false && contract.precondition.is_some()` in `required_checks`).
Command: `cargo test -p repo-conformance --test conformance --
conformance_md_07`. Expected: an executable application of a model with an
unchecked precondition links.

```text
thread 'conformance_md_07' (7264) panicked at crates/conformance/src/cases/models.rs:1546:9:
assertion `left == right` failed: SessionModel
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 282 filtered out
```

Removed: the requirement was restored; `conformance_md_07` passes and
`model-unvalidated-boundary` fails with `LLT4008`.

### model realization bypass check can fail

Planted: executable code could apply a realization function directly
(`if false && env.models.realization_functions.contains(&key)` in
`check_boundary`). Command: `cargo test -p repo-conformance --test
conformance -- conformance_md_07`. Expected: `classify` calling `DigitNet`
links.

```text
thread 'conformance_md_07' (16949) panicked at crates/conformance/src/support.rs:418:14:
check fails
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 282 filtered out
```

Removed: the check was restored; `conformance_md_07` passes and
`model-realization-bypass` fails with `LLT4008`.

### model artifact digest check can fail

Planted: linking no longer compared a configured artifact's bytes with its
configured digest (`if false && observed != source.sha256` in
`load_artifacts`). Command: `cargo test -p repo-conformance --test
conformance -- conformance_md_02`. Expected: tampered bytes are no longer
refused as a configuration mismatch. The declaration's own digest check still
refuses them, with another message, so the tampering never reaches a backend.

```text
thread 'conformance_md_02' (6697) panicked at crates/conformance/src/cases/models.rs:115:5:
expected "not its configured" under LLR3007, got LLR3007: phase link: artifact `banner` bytes have SHA-256 3733cd977ff8eb18b987357e22ced99f46097f31ecb239e878ae63760e83e4d5, not the declared 3369421cb6a657bcbdbec197a0c3b2a8e208dd7746853513f9d77331d0fa1397
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 282 filtered out
```

Removed: the check was restored; `conformance_md_02` passes and
`model-artifact-digest-mismatch` fails with `LLR3007`.

### model artifact schema decoding can fail

Planted: an integer tensor's byte length was no longer checked against its
shape (`if false && bytes.len() != expected` in `decode`). Command: `cargo
test -p repo-conformance --test conformance -- conformance_md_02`.
Expected: a shape the bytes cannot hold is no longer a schema violation.

```text
thread 'conformance_md_02' (13418) panicked at crates/conformance/src/support.rs:526:5:
expected LLR3008, found ["LLT4006"] (LLT4006: phase link: neural realization `DigitNet` layer 1 weights `hiddenWeights` is a int_tensor of shape [12, 8]; the slot needs an integer tensor of shape [12, 7])
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 282 filtered out
```

Removed: the check was restored; `conformance_md_02` passes and
`model-artifact-schema-violation` fails with `LLR3008`.

### model composition junction check can fail

Planted: a sequence junction was no longer checked against its stage
(`let _ = junction_check(...)` in `check_composite`). Command: `cargo test
-p repo-conformance --test conformance -- conformance_md_08`. Expected: an
unconditional junction before a stage with a precondition links.

```text
thread 'conformance_md_08' (14466) panicked at crates/conformance/src/support.rs:418:14:
check fails
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 282 filtered out
```

Removed: the check was restored; `conformance_md_08` passes and
`model-composition-missing-junction` fails with `LLT4007`.

### model descriptor slot typing can fail

Planted: descriptor terms were no longer checked against the types their
slots fix (`let _ = check_slots(...)` in `check_realization`). Command:
`cargo test -p repo-conformance --test conformance -- conformance_md_04`.
Expected: a Boolean label of a natural-number output is no longer an
interface mismatch.

```text
thread 'conformance_md_04' (15034) panicked at crates/conformance/src/support.rs:526:5:
expected LLT4006, found ["LLT4001"] (LLT4001: phase link: realization declaration `DigitNet` elaborates to `DigitNet`, which is ill-formed: list head has type Bool, expected Nat)
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 282 filtered out
```

Removed: the check was restored; `conformance_md_04` passes.

### model claim generator is checked by Lean

Planted: the `dataset_agreement` statement generator compared each expected
output with itself instead of with the realization's output (an over-claim
every example would satisfy), and a copy of `examples/models` was edited to
state exactly the planted statement (`labeled_agreement` over the planted
fold, 40 of 40, and the claim's `agreements` to 40). Commands: `lexlean
check`, then `lexlean verify`, in that copy. Expected: linking accepts the
theorem, because it states the generator's output exactly; Lean refuses the
restatement against the fixed `LexLeanModels.Agreement`.

```text
check 0
verify 1
error[LLV7002]: Lean rejected `Models.Recognizer` (error): Tactic `decide` proved that the proposition
  LexLeanModels.Agreement DigitNet sameDigit labeledGlyphs 40 40
is false
```

Removed: the generator was restored; the committed example verifies.

This restatement is proved by `decide` against the fixed `Agreement`, so it
does not use the generated statement; *model contract-claim generator is
checked by Lean* covers the claims whose restatement is the user's theorem.

### model kernel differential can fail

Planted, one at a time, against `conformance_md_09` (`cargo test -p
repo-conformance --test conformance -- conformance_md_09`), whose
expectations come from an independent integer model: the first-maximum
decoder keeping a later equal value (`less_than(leader, score + 1)`), the
requantization dividing by `2^(shift+1)`, the rectifier passing negative
values, and the portable runtime's integer quotient flooring (`Int.ediv`
for `Int.tdiv`). Expected: Lean refuses the seeded expectations.

```text
thread 'conformance_md_09' (15625) panicked at crates/conformance/src/support.rs:1903:10:
the module verifies with real Lean: LexLeanError { class: Language, diagnostics: [Diagnostic { code: DiagnosticCode("LLV7002"), message: "Lean rejected `Models.Main` (error): Tactic `decide` proved that the proposition\n  Tie Sample.s0 = 1\nis false", ...
thread 'conformance_md_09' (18279) panicked at crates/conformance/src/support.rs:1903:10:
the module verifies with real Lean: LexLeanError { class: Language, diagnostics: [Diagnostic { code: DiagnosticCode("LLV7002"), message: "Lean rejected `Models.Main` (error): Tactic `decide` proved that the proposition\n  Logits Sample.s0 = [50075, 63645, 87054, -4700, 27212]\nis false", ...
thread 'conformance_md_09' (18907) panicked at crates/conformance/src/support.rs:1903:10:
the module verifies with real Lean: LexLeanError { class: Language, diagnostics: [Diagnostic { code: DiagnosticCode("LLV7002"), message: "Lean rejected `Models.Main` (error): Tactic `decide` proved that the proposition\n  Logits Sample.s0 = [50075, 63645, 87054, -4700, 27212]\nis false", ...
thread 'conformance_md_09' (19553) panicked at crates/conformance/src/support.rs:1903:10:
the module verifies with real Lean: LexLeanError { class: Language, diagnostics: [Diagnostic { code: DiagnosticCode("LLV7002"), message: "Lean rejected `Models.Main` (error): Tactic `decide` proved that the proposition\n  Quantized Sample.s0 = [0, -14, -37, -33, -40, 10, -32, 46]\nis false", ...
```

A first seed, with large biases, saturated every requantized value, so the
divisor plant passed unnoticed; the seed now poses, and the case asserts it
poses, a tie, a negative value that truncation and flooring requantize
differently, clamping at both bounds, and a rectified value inside the
range.

Removed: each plant was restored; `conformance_md_09` passes.

### model evidence discharge rules can fail

Planted, one at a time (review r47, plants C and E): any claim at all
discharged the postcondition check (`discharged.is_empty()` for
`!discharged.contains(&ClaimKind::SatisfiesContract)` in `required_checks`),
and the output-invariant check was never required (`if false &&
!discharged.contains(&ClaimKind::PreservesInvariant)`). Commands: `cargo test
-p repo-conformance --test conformance -- conformance_md_07
conformance_md_08`. Expected: a model whose evidence is only a dataset
agreement or an equivalence, or satisfies its contract without preserving
its invariant, would run unchecked. `examples/models` now holds such models
(`Ledger.GuessModel`, `GuessExactModel`, `SpillModel`, `SpillRawModel`), and
roots that list the checks they need, whose refusals the kernel decides.

```text
thread 'conformance_md_07' (7826) panicked at crates/conformance/src/cases/models.rs:204:10:
snapshot: LexLeanError { class: Language, diagnostics: [Diagnostic { code: DiagnosticCode("LLT4007"), message: "phase link: composite `GuessTwice`: the output is Result (Prod (Nat) (Nat)) (ContractViolation), expected Prod (Nat) (Nat)", ...
thread 'conformance_md_07' (8388) panicked at crates/conformance/src/cases/models.rs:1546:9:
assertion `left == right` failed: SpillModel
thread 'conformance_md_08' (8389) panicked at crates/conformance/src/cases/models.rs:1880:5:
```

Removed: each rule was restored; `conformance_md_07` and `conformance_md_08`
pass.

### model artifact rank bound can fail

Planted: the tensor rank bound was skipped (`if false && shape.len() >
MAX_TENSOR_RANK` in `check_artifact`). Command: `cargo test -p
repo-conformance --test conformance -- conformance_md_02`. Expected: the
one-byte artifact with a 40000-deep shape of `model-artifact-rank-overflow`
is no longer refused by the bound. With the declared type still checked
before decoding it fails on the type, printing a 40000-deep type; before
review r47 the same input decoded first and exhausted 14 GB.

```text
thread 'conformance_md_02' (8977) panicked at crates/conformance/src/cases/models.rs:126:5:
tests/negative/model-artifact-rank-overflow: expected "an integer tensor has at most 16 dimensions, not 40000", got LLR3008: phase link: artifact `weights`: the declared type List (Int) is not the schema's value type List (List (List (List (List ...
```

Removed: the bound was restored; `conformance_md_02` passes, and the fixture
fails with `LLR3008` in about 0.2 s.

### model composition checks after a stage can fail

Planted: a composite stage's checks after it runs were dropped (`let checks:
Vec<ModelCheck> = Vec::new();` in `guard_after`). Commands: `cargo test -p
repo-conformance --test conformance -- conformance_md_08`, then `lexlean
verify` in a copy of `examples/models`. Expected: a stage whose evidence
leaves its postcondition or output invariant open runs unchecked inside a
composite; pinned Lean refuses the kernel-decided refusals.

```text
thread 'conformance_md_08' (9543) panicked at crates/conformance/src/cases/models.rs:1870:9:
ClampGuess elaborates Ledger::belowCheck: ...
error[LLV7002]: Lean rejected `Models.Main` (error): Tactic `decide` proved that the proposition
  twiceCode (guessOvershoot 4) = 13
is false
error[LLV7002]: Lean rejected `Models.Main` (error): Tactic `decide` proved that the proposition
  streamCode (spillStream [10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10]) = 14
is false
```

Removed: the checks were restored; `conformance_md_08` passes and the example
verifies.

### model branch state threading can fail

Planted: a branch over stateful stages no longer replaced the state
component of the arm taken (`.filter(|stateful| **stateful == index &&
false)` in `check_composite`), so both components were carried unchanged.
Command: `cargo test -p repo-conformance --test conformance --
conformance_md_08`. Expected: the kernel-decided outcomes of
`Flows.ClampOrTally` no longer hold.

```text
thread 'conformance_md_08' (6664) panicked at crates/conformance/src/cases/models.rs:245:14:
the models example verifies under pinned Lean: LexLeanError { class: Language, diagnostics: [Diagnostic { code: DiagnosticCode("LLV7002"), message: "Lean rejected `Models.Main` (error): Tactic `decide` proved that the proposition\n  branchCode (branchStep (5, 7) 3) = 8007008\nis false", ...
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 282 filtered out
```

Removed: the update was restored; `conformance_md_08` passes and the example
verifies.

### model artifact decoding charge can fail

Planted, separately: (a) the charge was dropped (`let _ = nodes;` in place
of `artifacts.charge(name, *length, nodes)?;` in `check_artifact`), so
decoded values were charged only when the module finished linking; (b) the
charge was made after decoding instead of before it; (c) the per-declaration
byte charge was dropped (`saturating_add(length - length)` in
`ArtifactStore::charge`). Command: `cargo test -p repo-conformance --test
conformance -- conformance_md_02`. Expected: (a) `model-artifact-decode-budget`,
one digest declared three times under `max_ir_nodes = 3000`, is refused only
after all three decode; (b) the 4 MiB artifact of empty lines, declared three
times, is decoded before it is refused, which needs about 1.5 GB, so the
re-run of the test in a process limited to 1 GiB of address space (on
Linux, the normative host; macOS's shell cannot set the limit) aborts;
(c) three declarations of a 1200-byte artifact fit a source budget that
holds only two more copies.

```text
(a) thread 'conformance_md_02' (28498) panicked at crates/conformance/src/cases/models.rs:126:5:
tests/negative/model-artifact-decode-budget: expected "max_ir_nodes exceeded: configured 3000, observed 3876 IR nodes once artifact `vocab2` decodes to 1292, before decoding", got LLS8002: max_ir_nodes exceeded in phase link: configured 3000, observed 3909 linked IR nodes through module `Main`
(b) thread 'conformance_md_02' (27434) panicked at crates/conformance/src/cases/models.rs:900:5:
the wide artifact is refused within 1 GiB: ExitStatus(unix_wait_status(6))
(c) thread 'conformance_md_02' (29254) panicked at crates/conformance/src/support.rs:418:14:
check fails
```

Removed: each plant was restored; `conformance_md_02` passes. The reviewer's
probe (three declarations of the 4 MiB artifact, default limits) now fails
with `LLS8002` at the first declaration in 0.3 s with a peak RSS of 25 MB,
where it took 27 s and 4.46 GB before; `decoded_nodes_is_exact` checks that
the charge equals the node count of the decoded value at every chunk
boundary.

### model checked junction before a stateful stage can fail

Planted: a checked junction no longer validated a later stateful stage's
invariant (`guard_before` received a clone of the stage with
`validators.remove(&ContractPredicate::Invariant)` in `check_composite`),
while the invariant stayed out of the entry requirements. Command: `cargo
test -p repo-conformance --test conformance -- conformance_md_07
conformance_md_08 conformance_md_12`. Expected: `Flows.TallySpillRaw` runs
`Ledger.SpillRawModel` from a state component its invariant does not hold
for, so the kernel-decided input-invariant refusal of `Main.raw_chain_refuses_input`
becomes an output-invariant one.

```text
thread 'conformance_md_08' (3882) panicked at crates/conformance/src/cases/models.rs:1991:9:
TallySpillRaw elaborates ContractViolation.input_invariant: ...
thread 'conformance_md_12' (3883) panicked at crates/conformance/src/cases/models.rs:245:14:
the models example verifies under pinned Lean: ... "Lean rejected `Models.Main` (error): Tactic `decide` proved that the proposition\n  chainCode (rawChainStep (4, 150) 3) = 4150002\nis false" ...
test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 287 filtered out
```

Removed: the validator was restored; the three tests pass, and
`RawChainModel` has no entry obligation.

### model checks after a stateful stage can fail

Planted: the postcondition validator run after a stateful stage received its
states swapped (`vec![after.clone(), input.clone(), before.clone(),
output.clone()]` in `guard_after`), which still type-checks because a
postcondition takes `S, I, S, O`. Command: `cargo test -p repo-conformance
--test conformance -- conformance_md_07 conformance_md_08
conformance_md_12`. Expected: `Ledger.Grows` is not symmetric in the two
states, so `Flows.TallySpillRaw` refuses the accepted step of
`Main.raw_chain_accepts`, and `Flows.TallyDrain` accepts the step that
`Main.drain_chain_refuses_postcondition` refuses (`Flows.Drain` violates
`Ledger.FreeContract`'s postcondition). `lexlean verify` in a copy of
`examples/models` under the plant rejects both.

```text
thread 'conformance_md_08' (5141) panicked at crates/conformance/src/cases/models.rs:245:14:
the models example verifies under pinned Lean: ... "Lean rejected `Models.Main` (error): Tactic `decide` proved that the proposition\n  chainCode (rawChainStep (4, 20) 3) = 7024104\nis false" ...
test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 287 filtered out
error[LLV7002]: Lean rejected `Models.Main` (error): Tactic `decide` proved that the proposition
  chainCode (drainChainStep (4, 20) 3) = 4020003
is false
```

Removed: the order was restored; the three tests pass and the example
verifies.

### model contract-claim generator is checked by Lean

Planted (review r47, S2): the stateless `satisfies_contract` generator stated
the vacuous `P x -> P x` for a contract with a precondition, and a copy of
`examples/models` was edited so that each such claim's theorem
(`triage_safe`, `policy_escalates`, `pipeline_responds`, `checked_answers`)
states exactly that. Commands: `lexlean check`, then `lexlean verify`, in that
copy. Expected: linking accepts the theorems, which state the generator's
output; Lean refuses the restatement against the fixed
`LexLeanModels.Satisfies`, which is not definitionally that statement.

```text
check 0
verify 1
error[LLV7002]: Lean rejected `Models.Triage` (error): Type mismatch
  Triage.triage_safe
has type
  ∀ (presentation : Presentation), Symptomatic presentation → Symptomatic presentation
but is expected to have type
  LexLeanModels.Satisfies Symptomatic Safe TriageScore
```

Removed: the generator was restored; the committed example verifies.

### model axiom audit union can fail

Planted: verification no longer required the generated declarations of a
model declaration to observe exactly its axiom set together (`if false &&
union... != declaration.axioms()` in `verify/mod.rs`). Command: `cargo test
-p repo-conformance --test conformance -- conformance_md_06`. Expected:
evidence stating an axiom none of its declarations observes verifies.

```text
thread 'conformance_md_06' (21466) panicked at crates/conformance/src/support.rs:445:14:
verify fails
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 282 filtered out
```

Removed: the check was restored; `conformance_md_06` passes.

### a planted contract and realization mismatch is refused by verification

Planted: in a copy of `examples/models`, the output bias of digit 8 in
`artifacts/output-bias.bin` was negated (from -7 to 7), with its digest
restated in `lexlean.toml` and the `outputBias` declaration and the lock
regenerated, so linking sees only consistent, content-addressed bytes.
Commands: `lexlean check`, then `lexlean verify`. Expected: the network no
longer computes the contract's digit, and only Lean's kernel can see it.
`conformance_md_12` plants the same mismatch and requires `LLV7002`.

```text
check 0
verify 1
error[LLV7002]: Lean rejected `Models.Recognizer` (error): Tactic `decide` proved that the proposition
  DigitNet Glyphs.Glyph.g0 = Glyphs.digitOf Glyphs.Glyph.g0
is false
  --> src/Recognizer.lex.tex:6:5824
```

Removed: the copy was discarded; the committed example verifies.

## End-to-end Lean evidence

The literal §29 example verifies against real `leanprover/lean4:v4.32.1`: probe elaboration, module compilation, separate-process `leanchecker` replay, exact `#print axioms` parsing, and the `\noaxioms` policy over an empty observed set (`conformance_ex_01`). The required §29.6 mutations are mechanized: a false proposition fails inside Lean and remaps to the source proof sentence (`conformance_ex_02`, `conformance_pf_18`); an undeclared title word fails lexical closure (`conformance_ex_03`); an indistinguishable same-surface entry is ambiguity, never priority (`conformance_ex_04`); an insufficient axiom allow-list fails policy checking with the observed excess recorded (`conformance_ex_05`, `conformance_vr_16`); and two clean builds in distinct paths publish byte-identical trees (`conformance_ex_06`, plus `just repro`).

The negative fixture suite (`tests/negative/<class>/`, §28.5) runs every rejection class through the CLI, including the Lean-backed ones: a Lean elaboration failure (`LLV7002`), a failing `leanchecker` (`LLV7003`, through a fixture toolchain overlay), malformed axiom output (`LLV7004`, through a `lake` overlay that corrupts only the audit run), an axiom-policy excess (`LLV7005`), a toolchain version mismatch (`LLV7001`), and a PDF executable hash mismatch (`LLS8004`). `conformance_vr_15` asserts that each failing stage (probe, module, replay, audit, policy) leaves no staging or verified directory behind, and `conformance_vr_07` plants a warning on a successful module compilation and asserts `LLV7006` with nothing published. `conformance_sp_06` collects the constructs every
certified root realizes and requires every runtime row of
`language/production-1.2.toml` among them; with the coverage example's collection roots
withheld, `primitive.map_insert` and the other collection rows are reported. `conformance_vr_10` runs the pinned `lean` on a module with three `#print axioms` commands and asserts the parser accepts the live output in the toolchain's own order and rejects an unknown-constant error line.

## Generic language delta verification (Issue #4)

All generic language extensions admitted for production-system modeling (including reserved-segment identifier quotation, Lean escape grammar for string literals, bounded semantic elaboration, and typed ambiguity/cycle diagnostic detail) are generic compiler features modeled under Language 1.1 (§17.11).

In accordance with gate discipline (R4, R8):
1. No Prism-specific parser branch, vocabulary, or handwritten Lean escape hatch is introduced;
2. Conformance cases `SM-19` (`strings::verify()`, `names::verify()`, and large byte declarations) and `CL-21` execute live Lean verification and negative falsifiability assertions;
3. Falsifiability is demonstrated by `semantic string lowering can fail` and `semantic identifier lowering can fail` above;
4. The full normative acceptance gate passes cleanly without deferral or narrowing.

## Authority and oracle evidence closure (Issue #6)

Every upstream authority cited by LexLean (`model/authorities.toml`) is bound to immutable, content-addressed references:
- `LEAN-REL-4-32-1`: Lean 4.32.1 release (source revision `f054605aea4b840552cca2e725580bffd1e1b704`, archive SHA-256 `6dec8667fbf57ba480a18a8b0c353b2ee157346b2630b211ccbefeedf20545f8`).
- `LAKE-4-32-1`: Lake toolchain component (same source commit and acquired archive SHA-256).
- `LEANCHECKER-4-32-1`: leanchecker kernel replay utility (same source commit and acquired archive SHA-256).
- `PRINT-AXIOMS-4-32-1`: Lean `#print axioms` output behavior bound by toolchain revision and committed test vectors (`tests/golden/axiom-parser/`).
- `LEAN-LCNF-4-32-1`: Lean's compiler front end (base-phase LCNF), bound by source revision and by the SHA-256 of the defining source file of every registered call and type in `language/lcnf-1.2/authority.toml`, re-checked against the pinned toolchain by `conformance_ne_04`.
- `RUSTC-1-97-1`: the Rust 1.97.1 compiler that `rust-toolchain.toml` pins (source revision `8bab26f4f68e0e26f0bb7960be334d5b520ea452`, source archive SHA-256 `622c2b429c53cbfdc0dd3a51d03554e91cd63ebec1912c1f5709640cdfef1a9d`), the oracle of the calculus's Rust renderings; `conformance_tc_07` refuses any other `rustc`.

Oracle execution evidence binds positive and negative paths:
- Positive execution: End-to-end elaboration, kernel replay, and axiom auditing across all examples (`list-induction`, `nat-add-zero`, `peano-arithmetic`, `propositional-logic`, `semantic-1.1`, `uor-atlas`).
- Negative execution: Non-vacuous rejection of planted mutations in `tests/negative/` across toolchain mismatch (`LLV7001`), elaboration failure (`LLV7002`), kernel replay rejection (`LLV7003`), axiom corruption (`LLV7004`), axiom policy excess (`LLV7005`), compilation warning (`LLV7006`), named-root extraction rejection (`LLV7011`, `extraction-rejected`, `extraction-uncompiled-external`), extraction authority drift (`LLV7012`, `extraction-authority-drift`), and PDF mismatch (`LLS8004`).
- Non-executable boundaries: Lean's mathematical correctness is an external authority guarantee (`some-true`), not proven by LexLean; `leanchecker` is a same-kernel replay mechanism rather than an independent verifier.

## First-party package identity and publishing bootstrap closure (Issue #5)

LexLean's first-party package identity is verified and bound to reproducible source and release evidence:
- Package identity: `lexlean` version `0.3.0`, Rust 2021 edition, license `MIT OR Apache-2.0`.
- Offline standalone verification: `cargo xtask check-package` executes `repo_conformance::support::packaged_crate_version`, extracting `lexlean-0.3.0.crate` into an isolated temporary workspace and building it with `--offline --locked`. It verifies that the standalone binary reports the exact four-line identity:
  ```text
  lexlean 0.3.0
  language 1.0
  compiler-semantics 95deb33a8d416d7bf60f02a36e251a71c3ee6f046b7e475d7bae2fc5ddc3767d
  lean-toolchain leanprover/lean4:v4.32.1
  ```
- Dependency closure: Package contains 0 repository-only dependencies (`repo-model`, `repo-conformance`, `xtask`), strictly enforced by `audit-shipped` (Rule R6).
- Embedded normative data: All normative files (`language/`, `schemas/`, `model/errors.toml`, `tests/golden/`, `LICENSE-APACHE`, `LICENSE-MIT`) are embedded without drift via canonical in-crate symlinks dereferenced by `cargo package`.
- Owner-controlled bootstrap protocol: Initial publication of first-party crates to crates.io requires owner-level credentials before trusted publishing OIDC can be configured. `.github/workflows/release.yml` accepts `secrets.CARGO_REGISTRY_TOKEN` for owner-controlled first upload and bootstrap checks, and transitions to short-lived GitHub Actions OIDC trusted publishing (`rust-lang/crates-io-auth-action`) once registered.
- Verification receipt:
  - Crate archive: `lexlean-0.3.0.crate`
  - SHA-256 digest: `9fd5c7a10e3904df8c465dea15f880e12f0d88361c1d4e948bbc7cd190e42d3e`
  - Package closure: 712 packaged files, 2.0 MiB uncompressed, 411 KiB compressed.

## Release identity and downstream provenance closure (Issue #3)

LexLean delivers an immutable, replayable release identity package for downstream consumers (including PrismPM 0.3.0 acceptance flows):
- Machine-readable release identity: `release/release-identity.json` specifies:
  - `spec`: `lexlean/release-identity/1`
  - `package`: `lexlean`
  - `version`: `0.3.0`
  - `compiler_semantics_id`: `95deb33a8d416d7bf60f02a36e251a71c3ee6f046b7e475d7bae2fc5ddc3767d`
  - `lean_toolchain`: `leanprover/lean4:v4.32.1`
  - `crate_sha256`: `e4f014979938722aadf17a73f97a4f839938e322fb32d3ce3d0abf760995726f`
  - `manifest_sha256`: `c3e5ca77f22c32290c73b4a10b6a256e6d179d7a95dbd2c45405dc8b4bde53b7`
  - `host_targets`: `["x86_64-unknown-linux-gnu", "aarch64-unknown-linux-gnu", "x86_64-apple-darwin", "aarch64-apple-darwin", "x86_64-pc-windows-msvc"]`
- Tree manifest binding: `release/MANIFEST.sha256` enumerates the exact SHA-256 for each of the 712 files inside the packaged crate, enabling downstream consumers to verify vendored trees via `audit_tree_manifest` without source-only assumptions.
- Complete release checksum manifest: `release/checksums.txt` covers all release assets, ensuring immutable binding across source, toolchain, and package artifacts.
- Downstream independence: PrismPM step-1 dependency checks verify the published artifacts and tree manifests completely offline using canonical cryptographic digests.

## Production release closure and §30 evidence receipts (Issue #2)

LexLean synthesizes complete release artifacts, evidence receipts, and authoritative oracle bindings required for production SDK consumption and PrismPM 0.3.0 acceptance:
- Deterministic release assembly:
  - `cargo xtask release-artifacts` derives all 10 release artifacts deterministically from clean git trees.
  - `release/version-output.txt` reports the exact four-line identity matching the compiler binary:
    ```text
    lexlean 0.3.0
    language 1.0
    compiler-semantics 95deb33a8d416d7bf60f02a36e251a71c3ee6f046b7e475d7bae2fc5ddc3767d
    lean-toolchain leanprover/lean4:v4.32.1
    ```
  - `release/compiler-semantics-id.txt`: `95deb33a8d416d7bf60f02a36e251a71c3ee6f046b7e475d7bae2fc5ddc3767d`
  - `release/lexlean.crate`: exact SHA-256 `73494e4d958fc7b67555df7a4bfb242589fe22f6928aae84ec5589fc5006e020`
  - `release/MANIFEST.sha256`: binding all 712 package files
  - `release/release-identity.json`: machine-readable provenance manifest
  - `release/checksums.txt`: SHA-256 manifest covering all release assets
- Normative release gate evaluation (`cargo xtask release-check`):
  - Standalone packaging verification: the packaged crate extracts and builds offline in complete isolation, printing the identical four-line identity and compiler-semantics ID.
  - Version-bound criteria distinction: in accordance with SPEC.md §2.3, tags prior to 1.0.0 are production integration milestones rather than full-spec completion releases. `release-check` verifies that all content and integrity criteria hold (`crate-package`, `semantics-id`, `checksums`, `conformance-doc`, `errors-doc`, `spec`, `licenses`), while honestly reporting `source-tag` and `version-output` as the only criteria reserved for 1.0.0.
- Cross-root reproducibility:
  - `cargo xtask check-reproducibility` proves two clean builds in distinct temporary paths are byte-identical across all platform-independent artifacts.
- Authoritative oracle coverage:
  - Both positive execution (elaboration, kernel replay, axiom auditing across all examples) and negative execution (non-vacuous mutation rejection in tests/negative/) are validated and bound to upstream commit digests.
- Downstream integration:
  - PrismPM dependency/identity checks referencing LexLean pass without manual exceptions or source assumptions.

## Semantic preservation (Issue #25)

`conformance_sp_01` lowers every production root of the language-1.2 examples
(`examples/production-coverage` among them) twice and
requires one valid program in first-binding order, an origin for every
function and ADT, and exactly the root's eligibility closure; a report with a
member removed must fail with `LLI9001`, and `validate-model` audits the
lowering, the certificate generator, and the source reader for default
matches. `conformance_sp_02` generates certificate A for every production root
of `examples/production` (6 roots) and `examples/production-coverage` (33 roots), compiles each
with the pinned Lean beside the shipped library, the shipped calculus modules,
and the generated modules, replays each through `leanchecker` (the same
kernel, §22.4), and requires every root theorem to depend on exactly
`Classical.choice`, `Quot.sound`, and `propext`; it then regenerates
certificates against programs with a planted branch swap, an addition that
subtracts, a wrong constructor, a wrong callee, and a wrong literal, and
requires Lean to reject all five. `conformance_sp_03` runs six seeded inputs
per root through the calculus interpreter and through Lean's evaluation of
each certificate's `denote`, requires every pair to agree (234 cases), and
requires one altered outcome to be reported. `conformance_sp_04` compares
every library declaration's printed axioms with
`language/preservation-1.2/library.toml` (406 declarations), the shipped
calculus modules with the compiler golden, and refuses planted `sorry`,
`axiom`, `native_decide`, `ofReduceBool`, a disallowed option, and a foreign
import. Each theorem is a kernel-checked proof about the root it names; the
generator, the library's coverage of constructs, and the coverage example are `build`
evidence for roots not certified. `conformance_sp_07` checks the declared Rust machine: the generated
`RustSyntax` and `RustSemantics` sources equal their generator
(`crates/conformance/src/rust_source.rs`), their shipped copies equal the
compiler golden (which `cargo xtask verify-examples` reverifies with exact
axioms), the machine declares exactly the renderer's runtime items with the
renderer's failure and heap classes, and every certified root's crate, in
every target it is eligible for, prints to a `RustSyntax` term that the
pinned Lean elaborates and evaluates on the differential's seeded inputs,
each outcome equal to the calculus interpreter's on the lowered program.
`conformance_sp_08` checks certificate B: the closed-rule-set audit
(`repo_model::correspondence`, also run by `cargo xtask validate-model`)
holds on the repository's aligner, correspondence, soundness theorem, and
calculus, the aligner's `RULES` equal `Corr`'s constructors in order, and a
rule dropped from `RULES`, a case renamed in `sound`, and a wildcard arm in
the block judgment are each reported; every renderer fixture in every profile
that renders it (138 renderings) has a certificate B whose
derivations use only declared rules, and with the certified roots'
renderings every one of the 49 rules is used; every certified root carries a
certificate B in each of its targets, audited in the certification run; the
fixtures' certificates compile, replay through `leanchecker`, and their
`root` theorems depend on exactly the three axioms. Seven crate mutations
(an `if`'s branches swapped, `nat_add` as `nat_sub`, a checked addition
as a checked subtraction, a sibling enum variant, a byte literal one byte
longer, a neighbouring callee, a natural literal plus one), each planted in
the first rendering that admits it, are refused twice: the aligner derives
nothing for the mutated crate or Lean rejects the derivation it writes, and
Lean rejects the unmutated derivation restated over the mutated crate. An
eighth mutation moves the first checked operation to another width
(`fixed_u8::checked_add` as `fixed_u16::checked_add`). The first run of
this case planted it and Lean accepted the realigned derivation: the machine
then read every width's item as the calculus's width-polymorphic primitive.
The machine now indexes such items by width (`itemAccepts`), the
correspondence's primitive rules require the operand types to have the
item's width (`itemTyped`), and value typing carries a fixed-width value's
width; the case requires certificate B to refuse the width change like the
other seven, and the renderer's correspondence check to refuse it too. A
ninth calls a list, bytes, or text item's sibling for another sequence
(`length_string` as `length_bytes`); the machine's sequence guard
(`itemTakes`) and value typing of lists, bytes, and text make certificate B
refuse it as well.
`conformance_sp_05` verifies
`examples/production` and requires every certificate, the audit output, and a
schema-valid `preservation.json` bound by the attestation in the published
set; the negative fixtures `certificate-rejected` (a lake overlay turns the
certificates' natural additions into subtractions) and `preservation-drift`
(it appends a false theorem to the shipped library) must fail with `LLV7013`
and `LLV7014` with nothing published, and `certificate-b-rejected` (the
overlay turns every `nat_add` call of each certificate B's crate into
`nat_sub`, leaving certificate A intact) with `LLV7015`.

