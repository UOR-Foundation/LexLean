# VERIFICATION

How this repository's claims are checked, which recipe enforces which rule, and the evidence that each gate can actually fail. A gate nobody has seen fail is indistinguishable from a gate that cannot (SPEC.md §27.9).

## The acceptance gate

`just vv` runs, in the normative order (SPEC.md §9.2):

| Recipe | Command | Rules it enforces |
| --- | --- | --- |
| `fmt-check` | `cargo fmt --all -- --check` | one canonical source formatting |
| `model` | `cargo xtask validate-model` | R1 (model is the single source; every model file parsed with unknown-field rejection), R2 (honesty levels and vocabulary, via the meta-gate), R3 (register/scenario/test bijection, Gherkin subset), R4 (`audit-deferral`), R5 (`audit-errors`), R6 (`audit-shipped`, including the shipped crate's normative links), R8 (`audit-generated`, `audit-language-closure`), RP-09 (`audit-no-unsafe`), PD-07 (`audit-production`), §27.5 (CONFORMANCE.md and ERRORS.md equal regeneration) |
| `spec-links` | `cargo xtask validate-spec-links` | RP-07, §27.6: the §31 table and `model/ids.toml` are bijective and byte-consistent |
| `lint` | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | no tolerated warnings |
| `test` | `cargo test --workspace --all-features` | §28.1 classes 1–2 and 4–5 (unit, property, integration, CLI), the model crate's own tests, and all 263 conformance tests, which include the §28.2 fixture suite (`conformance_ex_07`) and the crate-packaging round trip (`conformance_rp_12`) |
| `features` | `cargo check --workspace --all-features --all-targets` | every target compiles |
| `bdd` | `cargo test -p repo-conformance` | R3, §27.7, §27.8: register ↔ scenario ↔ test bijection, the meta-gate, and its own falsifiability test |
| `examples` | `cargo xtask verify-examples` | §28.6, EX-01: every example directory and the `compiler` project (§17.14) format, lock, check, build, and verify with real Lean 4.32.1; when an example commits `expected/verify/`, its normalized verification records must equal it (§29.5) |
| `golden` | `cargo xtask check-golden` | R10, §28.3: the *published* build tree of a real `build` in a fresh directory equals the committed oracles byte for byte |
| `repro` | `cargo xtask check-reproducibility` | AR-13, §28.4: two clean `build`s in distinct absolute directories publish byte-identical trees with no absolute path inside |
| `deny` | `cargo deny --all-features check` | advisories, bans, licenses, sources |

Outside `vv`:

- `just fixtures` (`cargo xtask check-fixtures`) runs every §28.2 fixture under `tests/fixtures/` and `tests/negative/` through the CLI entry point and compares exit code, canonical command result, diagnostics, artifact list, and platform-independent hashes with `expected/`. `just fixtures-write` is the only rewrite path.
- `just calculus` (`cargo xtask check-calculus`) compares every committed target fixture under `compiler/fixtures/`, the generated `TargetFixtures` module, the calculus modules `TargetSyntax`, `TargetSemantics`, `TargetOracle`, and `Main`, the project configuration, and every fixture's Rust package under `compiler/rust/rust-core/` and `compiler/rust/rust-std/` and every negative package manifest under `compiler/rust/negative/` with what the hand-written fixture set, the calculus's definition in `crates/conformance/src/calculus_source.rs`, and the renderer produce (§17.14), and the calculus modules shipped for certificates under `language/preservation-1.2/modules/` with the compiler project's golden modules (§17.17); `just test` enforces the same comparison through `conformance_tc_03`. `just calculus-write` is the only rewrite path.
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
thread 'conformance_pd_05' panicked at crates/conformance/src/cases/production.rs:646:17:
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
thread 'conformance_pd_05' panicked at crates/conformance/src/cases/production.rs:646:17:
production-unbounded-type: expected "construct `type.list` requires heap allocation, which target `rust-core` does not provide", got LLT4005: phase production: production root `LanguageTwelve.Main.first` is not eligible for target `rust-core`: effect mismatch: construct `type.list` realizes effect `allocation`, which the root does not admit (in `LanguageTwelve.Main.first`, reached by LanguageTwelve.Main.first; 1 violation(s) in total)
thread 'conformance_pd_04' panicked at crates/conformance/src/support.rs:390:14:
check fails
```

### production effect admission can fail

Planted: the admitted-effect test disabled (`if false &&
!production.effects.contains(effect) {`), so a root realized effects it did
not declare. Command: `cargo test -p repo-conformance --test conformance --
conformance_pd_05`. The first fixture whose only violation is an unadmitted
effect now checks.

```text
thread 'conformance_pd_05' panicked at crates/conformance/src/support.rs:390:14:
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
thread 'conformance_sm_23' panicked at crates/conformance/src/support.rs:380:14:
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
thread 'conformance_df_13' panicked at crates/conformance/src/support.rs:390:14:
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
thread 'conformance_df_12' panicked at crates/conformance/src/support.rs:390:14:
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
thread 'conformance_df_17' panicked at crates/conformance/src/support.rs:390:14:
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
thread 'conformance_df_16' panicked at crates/conformance/src/support.rs:390:14:
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
thread 'conformance_df_12' panicked at crates/conformance/src/support.rs:390:14:
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
thread 'conformance_sm_29' panicked at crates/conformance/src/support.rs:390:14:
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
`crates/lexlean/src/calculus/rust/validate.rs`. Command: `cargo test -p
repo-conformance --test conformance -- conformance_rb_01`. Expected: every
rendering that adds naturals emits a construct with no target-semantics
correspondence.

```text
thread 'conformance_rb_01' (18572) panicked at crates/conformance/src/cases/rust_backend.rs:242:40:
adt-evaluation (rust-std): the construct `call:runtime:nat_add` has no target-semantics correspondence
```

Removed: the row was restored; `conformance_rb_01` passes.

### rust identifier collision can fail

Planted: the package check admitted Rust keywords as exported names
(`KEYWORDS.contains(&name) && name.is_empty()`). Command: `cargo test -p
repo-conformance --test conformance -- conformance_rb_02`. Expected: the
committed negative manifest exporting `match` is packaged.

```text
thread 'conformance_rb_02' (2095) panicked at crates/conformance/src/cases/rust_backend.rs:101:51:
the manifest is refused: Package { files: {"Cargo.toml": [91, 112, 97, 99, 107, 97, 103, 101, 93, ...
```

Removed: the check was restored; `conformance_rb_02` passes.

### rust ownership mismatch can fail

Planted: the package check let an export copy a parameter of any type
(`Passing::Copy => ...`). Command: `cargo test -p repo-conformance --test
conformance -- conformance_rb_03`. Expected: the negative manifest copying
a list parameter is packaged.

```text
thread 'conformance_rb_03' (2716) panicked at crates/conformance/src/cases/rust_backend.rs:101:51:
the manifest is refused: Package { files: {"Cargo.toml": [91, 112, 97, 99, 107, 97, 103, 101, 93, ...
```

Removed: the check was restored; `conformance_rb_03` passes.

### rust boundary type check can fail

Planted: the package check never found a function value at an export's
boundary. Command: `cargo test -p repo-conformance --test conformance --
conformance_rb_04`. Expected: the negative manifest exporting a function
that takes a closure is packaged.

```text
thread 'conformance_rb_04' (3358) panicked at crates/conformance/src/cases/rust_backend.rs:101:51:
the manifest is refused: Package { files: {"Cargo.toml": [91, 112, 97, 99, 107, 97, 103, 101, 93, ...
```

Removed: the check was restored; `conformance_rb_04` passes.

### rust declared failure check can fail

Planted: the package check admitted an export declaring no errors for a
function that can overflow. Command: `cargo test -p repo-conformance --test
conformance -- conformance_rb_05`. Expected: the negative manifest
`arithmetic-undeclared-overflow` is packaged.

```text
thread 'conformance_rb_05' (3997) panicked at crates/conformance/src/cases/rust_backend.rs:101:51:
the manifest is refused: Package { files: {"Cargo.toml": [91, 112, 97, 99, 107, 97, 103, 101, 93, ...
```

Removed: the check was restored; `conformance_rb_05` passes.

### rust package lint gate can fail

Planted: the committed `compiler/rust/rust-std/nat-arithmetic/Cargo.toml`
set Clippy's default lints to `allow`. Command: `cargo test -p
repo-conformance --test conformance -- conformance_rb_06`. Expected: the
package with a planted clone of a `Copy` value passes Clippy. Setting the
group to `warn` instead is not a weakening: the package's
`[lints.rust] warnings = "deny"` makes every Clippy warning an error, and
that plant is refused like the original.

```text
thread 'conformance_rb_06' (19149) panicked at crates/conformance/src/cases/rust_backend.rs:584:13:
the planted lint is refused
```

Removed: the manifest was restored; `conformance_rb_06` passes.

### rust provenance binding can fail

Planted: the provenance hashed only the core runtime for every profile.
Command: `cargo test -p repo-conformance --test conformance --
conformance_rb_07`. Expected: a `rust-std` package's provenance no longer
equals the committed one.

```text
thread 'conformance_rb_07' (17867) panicked at crates/conformance/src/cases/rust_backend.rs:678:21:
assertion `left == right` failed: /home/user/wt-24/compiler/rust/rust-std/adt-evaluation/provenance.json
```

Removed: the hash was restored; `conformance_rb_07` passes.

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
thread 'conformance_sm_30' panicked at crates/conformance/src/support.rs:1875:10:
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
thread 'conformance_sm_28' panicked at crates/conformance/src/support.rs:1875:10:
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
thread 'conformance_sm_28' panicked at crates/conformance/src/support.rs:1875:10:
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
evidence for roots not certified. `conformance_sp_05` verifies
`examples/production` and requires every certificate, the audit output, and a
schema-valid `preservation.json` bound by the attestation in the published
set; the negative fixtures `certificate-rejected` (a lake overlay turns the
certificates' natural additions into subtractions) and `preservation-drift`
(it appends a false theorem to the shipped library) must fail with `LLV7013`
and `LLV7014` with nothing published.

