# VERIFICATION

How this repository's claims are checked, which recipe enforces which rule, and the evidence that each gate can actually fail. A gate nobody has seen fail is indistinguishable from a gate that cannot (SPEC.md §27.9).

## The acceptance gate

`just vv` runs, in the normative order (SPEC.md §9.2):

| Recipe | Command | Rules it enforces |
| --- | --- | --- |
| `fmt-check` | `cargo fmt --all -- --check` | one canonical source formatting |
| `model` | `cargo xtask validate-model` | R1 (model is the single source; every model file parsed with unknown-field rejection), R2 (honesty levels and vocabulary, via the meta-gate), R3 (register/scenario/test bijection, Gherkin subset), R4 (`audit-deferral`), R5 (`audit-errors`), R6 (`audit-shipped`, including the shipped crate's normative links, and every vendored authority's SHA-256 recomputed from its copy), R8 (`audit-generated`, `audit-language-closure`), RP-09 (`audit-no-unsafe`), PD-07 (`audit-production`), §17.19 (`audit-oracle-isolation`), §27.5 (CONFORMANCE.md and ERRORS.md equal regeneration) |
| `spec-links` | `cargo xtask validate-spec-links` | RP-07, §27.6: the §31 table and `model/ids.toml` are bijective and byte-consistent |
| `lint` | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | no tolerated warnings |
| `test` | `cargo test --workspace --exclude repo-conformance --all-features` | §28.1 classes 1–2 and 4–5 (unit, property, integration, CLI) of every crate but `repo-conformance`, and the model crate's own tests. `repo-conformance` is excluded, not skipped: `bdd` owns it (RP-13) |
| `features` | `cargo check --workspace --all-features --all-targets` | every target compiles |
| `bdd` | `cargo xtask bdd` | `cargo test -p repo-conformance --all-features`, or in CI the part of `.github/bdd-parts.json` that `LEXLEAN_BDD_PART` names (§9.4). R3, §27.7, §27.8: the sole owner of the complete `repo-conformance` suite — its library tests, the `bdd` meta-gate target (register ↔ scenario ↔ test bijection and its own falsifiability test), and the `conformance` target with every registered conformance test, which include the §28.2 fixture suite (`conformance_ex_07`) and the crate-packaging round trip (`conformance_rp_12`) |
| `inventory` | `cargo xtask check-test-inventory` | RP-13: the tests listed by `test` and by `bdd` (`cargo test … -- --list`) are disjoint and their union is the list of `cargo test --workspace --all-features`: no test lost, none run twice; the lists of the parts of `.github/bdd-parts.json`, each with its harness filters, are disjoint and their union is `bdd`'s list |
| `examples` | `cargo xtask verify-examples` | §28.6, EX-01: every example directory and the `compiler` project (§17.14, §17.15) format, lock, check, build, and verify with real Lean 4.32.1; when an example commits `expected/verify/`, its normalized verification records must equal it (§29.5) |
| `golden` | `cargo xtask check-golden` | R10, §28.3: the *published* build tree of a real `build` in a fresh directory equals the committed oracles byte for byte |
| `repro` | `cargo xtask check-reproducibility` | AR-13, §28.4: two clean `build`s in distinct absolute directories publish byte-identical trees with no absolute path inside |
| `deny` | `cargo deny --all-features check` | advisories, bans, licenses, sources |

Outside `vv`:

- `just fixtures` (`cargo xtask check-fixtures`) runs every §28.2 fixture under `tests/fixtures/` and `tests/negative/` through the CLI entry point and compares exit code, canonical command result, diagnostics, artifact list, and platform-independent hashes with `expected/`. `just fixtures-write` is the only rewrite path.
- `just calculus` (`cargo xtask check-calculus`) compares every committed target fixture under `compiler/fixtures/`, every GNAF request under `compiler/gnaf/`, the GNAF dependency manifest `compiler/gnaf.manifest.json` and schemas `schemas/gnaf-request.schema.json` and `schemas/gnaf-fixture.schema.json`, every GNAF certificate fixture under `compiler/certificates/` and the schemas `schemas/gnaf-certificate.schema.json` and `schemas/gnaf-certificate-fixture.schema.json`, the generated `TargetFixtures`, `GnafFixtures`, and `GnafCertificates` modules, the calculus modules `TargetSyntax`, `TargetSemantics`, `TargetOracle`, and `Main`, the `Gnaf` and `Certification` models, the project configuration, and every fixture's Rust package under `compiler/rust/rust-core/` and `compiler/rust/rust-std/` and every negative package manifest under `compiler/rust/negative/` with what the hand-written fixture sets, the calculus's definition in `crates/conformance/src/calculus_source.rs`, the models' definitions in `crates/conformance/src/gnaf_model.rs` and `crates/conformance/src/certification_model.rs`, and the renderer produce (§17.14, §17.15), and the calculus modules shipped for certificates under `language/preservation-1.2/modules/` with the compiler project's golden modules (§17.17), and the extraction adapter's tooling project `language/lcnf-1.2/tooling/` with what `crates/conformance/src/lcnf_adapter.rs`, `lcnf_extract.rs`, and `lcnf_registry.rs` render over the registry `tests/lcnf/registry.json` (§22.10); `just test` enforces the same comparison through `conformance_tc_03` and `conformance_gn_01`. `just calculus-write` is the only rewrite path, and it first regenerates the registry from the pinned Lean (`tests/lcnf/export.lean`), so it needs the pinned toolchain.
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

### production bundle inventory can fail

Before the correction for #133, `cargo test --offline --locked -p lexlean
--lib api::production::tests -- --nocapture` ran four tests and all four
failed. Collection reported both `evidence/attestation.json` and the literal
filename `evidence\attestation.json` under the slash-separated key; moving the
canonical file to the literal filename still reported the canonical key. A
non-UTF-8 filename was accepted with a replacement character, and duplicate
inventory keys reached the missing-`result.json` check instead of being
refused as duplicates.

Collection now encodes actual path components without changing their text
and refuses a component outside UTF-8 with `LLV7102`; replay refuses duplicate
keys before reading the result. The existing `PX-07` case also copies and
moves a real attestation to the literal-backslash filename and adds a
non-UTF-8 filename on a filesystem that accepts it, reusing its existing
export without another Lean invocation. Valid bundle bytes and identities
are unchanged.

### production compiler identity can fail

Before the correction for #136, `cargo test --offline --locked -p lexlean
--lib api::production::tests::attestation_facts -- --nocapture` ran four
Lean-free tests: the historical producing version passed, and all three
malformed-version cases failed. Hash-correct attestation bodies were accepted
with no compiler version, an empty version, or a numeric version, contrary to
the tests' explicit version requirements.

Replay now reads a non-empty string from the hash-checked attestation's
`lexlean.version` and requires the result's `compiler.version` to equal it.
The comparison uses the producing attestation, so it does not restrict replay
to this running crate's version. `PX-07` changes the result's version and
recomputes its identity within the existing tamper matrix, reusing the same
export without another verification run.

After both corrections, all eight Lean-free replay unit cases pass, and
`cargo test -p repo-conformance --test conformance conformance_px_07 -- --exact --test-threads=1`
passes in 195.15 seconds, including the untouched bundle and every existing
tamper. Formatting, workspace/all-target Clippy with warnings denied, and
the model/spec-link audits pass (380 IDs and 80 registered codes).

Before the correction for #142, `cargo test -p lexlean --lib
api::production::tests::attestation_facts` ran eight filtered tests in 0.00
seconds: the four existing tests passed and all four new tests failed. The
new tests require the producing compiler-semantics digest, a verified
`lexlean/attestation/2` envelope, and exact pinned Lean-version fields in
both the Lean and Lake records, including refusals of missing or malformed
facts, wrong patches, prereleases, and unrelated occurrences of the pin.
Compilation took 41.22 seconds; these are helper tests without Lean work.

Replay now checks those facts after recomputing the attestation identity
and binds its compiler semantics to the result. The compiler version stays
a comparison with the producing attestation, preserving historical versions;
the existing requirement that replay use the producing semantics and pinned
Lean toolchain is unchanged.

The expanded `PX-07` matrix changes the attestation's compiler semantics,
Lean and Lake version facts, schema tag, and verification status. It
refreshes the attestation identity, its file length and SHA-256, and both
enclosing result bindings, so the assertions target consistency rather than
an outdated digest. It shares its existing export and retains a
self-consistent historical compiler-version case. The Lean-backed matrix
belongs to `just bdd` and uses that existing export, without a second
verification for these regressions.

The bundled compiler input also states a Lean identity: `lean.version` and
`lean.githash`, whose file digests replay checked without reading those
fields. Replay now reads only that header
through the existing `LeanIdentity` type and compares its two strings with
the pin after checking the file's result and attestation bindings. Unknown
top-level payloads are skipped by the JSON reader rather than read as LCNF;
no extraction or full compiler-input schema validation is performed, and
`not_rechecked` is unchanged.

Focused cases cover missing, malformed, and foreign identity fields, and
skipped payloads nested 129, 257, and 2048 levels before or after the identity.
`PX-07` changes each scalar pin while refreshing the input's file digest,
the attestation's matching input digest, and both enclosing content
identities.

The filtered
`compiler_input_identity_refuses_missing_malformed_or_foreign_facts` run
then failed in 0.00 seconds on a top-level array containing the pinned
identity object: derived struct readers also accept sequence forms, although
the compiler-input identity requires named objects. Compilation took 1 minute
21 seconds, including Cargo lock contention. The matrix also rejects an array
in `lean`. Replay now requires an object at both levels through map readers
that preserve duplicate-field rejection and iterative ignored-payload skips;
the existing test also covers trailing JSON and duplicate `lean`, `version`,
and `githash` fields.

After all corrections, `cargo test -p lexlean --lib
api::production::tests` passes all fourteen helper and inventory tests in
0.01 seconds. Formatting, workspace/all-target Clippy with warnings denied,
and the model/spec-link audits pass (380 IDs and 80 registered codes).

The expanded `cargo test -p repo-conformance --all-features --test
conformance conformance_px_07 -- --exact --test-threads=1 --nocapture`
passes in 180.42 seconds with `LEXLEAN_VERIFY_WIDTH=1`. It retains every
previous path, byte, and result-field tamper, adds 24 fully rehashed
attestation/compiler-input fact tampers, and accepts the untouched bundle
and the self-consistent historical-version pair. All share the same real
production export; none of the added tampers invokes another verification.

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

### configured paths reject Windows drive prefixes (Issue #138)

The project-relative path validator previously accepted `C:`, `C:/outside`,
`c:../outside`, and a drive prefix inside a slash-separated path. On Windows,
such a segment can replace the project root when pushed onto a host path
([`PathBuf::push`](https://doc.rust-lang.org/std/path/struct.PathBuf.html#method.push)).
Every segment now rejects an ASCII letter followed by a colon before any
path join, on every host. Other colon names keep their existing policy.

Before the fix, `cargo test --offline --locked -p lexlean --test unit
path_rules_are_strict -- --exact --nocapture` ran one test and failed in
0.00 seconds at `crates/lexlean/tests/unit.rs:146`:

```text
"C:" escapes the project
```

The unit regression covers bare, absolute, drive-relative, lower-case and
nested prefixes, and retains ordinary relative paths and non-prefix colon
names. The existing `SE-01` case also checks configuration rejection for
`build_root`, `lockfile`, and `lean_workspace` by calling `parse_project`
alone. These checks exercise the shared policy before host path handling
and never access an outside path or run cleanup on one. No Windows execution
is established by the Linux failure above.

After the fix, all thirteen unit tests pass in 0.80 seconds, including the
expanded path matrix; `conformance_se_01` passes in 0.40 seconds. Formatting,
workspace/all-target Clippy with warnings denied, and the model/spec-link
audits pass (380 IDs and 80 registered codes).

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

### explicit byte-literal lowering can fail

A modeled byte literal used to be written `ByteArray.mk #[n, ...]`. It is now
written `_root_.ByteArray.mk #[...]` with every octet an explicit
`_root_.UInt8.ofNat (nat_lit n)`; an octet occurring four or more times in one
literal is bound once by a `let __byteN` local to that literal, so no generated
local outside the literal takes part in its meaning. The canonical document
keeps the original decimal numbers and order. SM-19, for languages 1.1 and 1.2,
reads every generated literal back with an independent closed-grammar reader
(it neither evaluates Lean nor calls the renderer), checks the document
display, verifies, and evaluates the generated definitions against the
original bytes. MD-02 applies the same reader to every embedded artifact byte.
The reader's own controls refuse foreign, noncanonical and unused syntax, and
a binding name that disagrees with its value.

Observed before the change: with the lowerer of `main` `243f0c734` unchanged,
`conformance_sm_19` fails at its first fixture, the empty array, with
`closed explicit-octet lowering: "expected closed byte syntax \"_root_.ByteArray.mk #[\""`.
That is the reader rejecting the old spelling. It is not a Lean failure: the
largest SM-19 corpus (1024 bytes, all 256 values) elaborates under `main`'s
rendering at the default `maxRecDepth` (1.1 s; 0.8 s explicit).

Measured, not claimed as an improvement beyond these numbers: Lean 4.32.1
(x86-64 Linux, 4 cores, one run each) on a file holding one `public def` of a
seeded-random byte literal in which all 256 values occur.

| bytes | `maxRecDepth` | `main` rendering | explicit rendering |
| --- | --- | --- | --- |
| 4096 | 100000 | 3.0 s, ok | 2.1 s, ok |
| 4096 | default | 2.6 s, ok | 2.3 s, ok |
| 16384 | 100000 | 12.7 s, ok | 11.3 s, ok |
| 16384 | default | 10.8 s, recursion depth | 8.9 s, recursion depth |
| 65536 | 100000 | 37.1 s, heartbeats (`isDefEq`) | 49.2 s, heartbeats (`LCNF compiler`) |
| 65536 | default | 33.3 s, heartbeats (`isDefEq`) | 42.7 s, recursion depth |

Bisection on a standalone scratch Lean file at Lean's default `maxRecDepth` (no
`set_option`; generated semantic modules instead emit `set_option maxRecDepth
100000`, `backend/semantic.rs`, which corresponds to the `100000` rows in the
table above), by 16-byte steps between 1024 and 16384 bytes, one run per size,
seeded-random bytes with all 256 values present: the largest literal that
elaborates at the default depth is 6394 bytes under both renderings, and 6409
bytes fails under both (`maximum recursion depth has been reached`). Elaboration
time at 4096 bytes is 2.0 s under both; at 6394 bytes it is 4.4 s for the `main`
rendering and 3.3 s for the explicit one. The lowering does not move the largest
elaborable literal.

No size succeeds under one rendering and fails under the other. Both fail at
16384 bytes at the default depth and at 65536 bytes at either depth; the
explicit form is faster at 4096 and 16384 bytes and slower at 65536. The
timing script is not committed; it writes the two spellings above for the
same seeded bytes and runs `lean` on the file.

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
thread 'conformance_pd_05' panicked at crates/conformance/src/cases/production.rs:662:17:
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
thread 'conformance_pd_05' panicked at crates/conformance/src/cases/production.rs:662:17:
production-unbounded-type: expected "construct `type.list` requires heap allocation, which target `rust-core` does not provide", got LLT4005: phase production: production root `LanguageTwelve.Main.first` is not eligible for target `rust-core`: effect mismatch: construct `type.list` realizes effect `allocation`, which the root does not admit (in `LanguageTwelve.Main.first`, reached by LanguageTwelve.Main.first; 1 violation(s) in total)
thread 'conformance_pd_04' panicked at crates/conformance/src/support.rs:432:14:
check fails
```

### production effect admission can fail

Planted: the admitted-effect test disabled (`if false &&
!production.effects.contains(effect) {`), so a root realized effects it did
not declare. Command: `cargo test -p repo-conformance --test conformance --
conformance_pd_05`. The first fixture whose only violation is an unadmitted
effect now checks.

```text
thread 'conformance_pd_05' panicked at crates/conformance/src/support.rs:432:14:
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
  thread 'conformance_ne_02' panicked at crates/conformance/src/cases/extraction.rs:434:13:
  assertion `left == right` failed
  ```

- the monomorphization plan dropped (`instances: Vec::new()`),
  `conformance_ne_02`:

  ```text
  thread 'conformance_ne_02' panicked at crates/conformance/src/cases/extraction.rs:416:18:
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
  thread 'conformance_ne_04' panicked at crates/conformance/src/cases/extraction.rs:977:34:
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
thread 'conformance_sm_23' panicked at crates/conformance/src/support.rs:465:14:
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
thread 'conformance_rp_10' panicked at crates/conformance/src/cases/repository.rs:630:13:
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
thread 'conformance_df_13' panicked at crates/conformance/src/support.rs:432:14:
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
thread 'conformance_df_12' panicked at crates/conformance/src/support.rs:432:14:
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
thread 'conformance_df_14' panicked at crates/conformance/src/cases/declarations.rs:1042:13:
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
thread 'conformance_df_15' panicked at crates/conformance/src/cases/declarations.rs:1156:17:
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
thread 'conformance_df_14' panicked at crates/conformance/src/cases/declarations.rs:1069:17:
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
thread 'conformance_df_17' panicked at crates/conformance/src/support.rs:432:14:
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
thread 'conformance_df_16' panicked at crates/conformance/src/support.rs:432:14:
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
thread 'conformance_df_12' panicked at crates/conformance/src/support.rs:432:14:
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
thread 'conformance_sm_29' panicked at crates/conformance/src/support.rs:432:14:
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

### the calculus and the machine are sources (SPEC.md §17.14)

`compiler/src/{TargetSyntax,TargetSemantics,TargetOracle,RustSyntax,RustSemantics,Main}.lex.tex`,
the reasoning oracles, and the project configuration were rendered from Rust
builders (`calculus_source.rs`, `rust_source.rs`) and compared with them by
`check-calculus`; the builders are removed, so the committed files are written
and edited as `.lex.tex`, and the plant that used to be here (an edit of
`TargetSemantics` that no longer equalled its builder) has no builder to be
unequal to. What guards them now is what was always independent of the
builder: the shipped copies of the calculus modules equal the compiler
project's golden (next section), `compiler/` is elaborated, replayed, and
audited by `just examples`, and `conformance_sp_07` reads the machine from the
committed modules and equates it with the renderer. Three plants, each an edit
of a committed file, show the last:

- **a class the machine states wrongly.** `itemFallible` in
  `compiler/src/RustSemantics.lex.tex` says `natAdd` cannot fail.
  `-- conformance_sp_07`:

  ```text
  panicked at crates/conformance/src/cases/preservation.rs:2026:17:
  assertion `left == right` failed: natAdd: the machine's classes equal the renderer's
    left: Some((false, false))
   right: Some((true, false))
  ```

- **an item the machine does not declare.** The `natSub` constructor is
  removed from `Item` in `compiler/src/RustSyntax.lex.tex`.
  `-- conformance_sp_07`:

  ```text
  panicked at crates/conformance/src/cases/preservation.rs:2024:40:
  NatSub (`.natSub`) is declared
  ```

- **a configuration that is not the production example's.** The name in
  `compiler/lexlean.toml` is changed. `-- conformance_sp_07`:

  ```text
  panicked at crates/conformance/src/cases/preservation.rs:1935:13:
  assertion `left == right` failed: compiler/lexlean.toml is the production example's under its own name
  ```

Removed: each file was restored; `conformance_sp_07` passes.

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

### certificate E can fail

Planted: certificate E of the recursive-tree root (`examples/production-coverage`,
`Coverage.Boundary.groveRoot`, `rust-std`) restated twice, once with the
first `RealizesFn false` read as `RealizesFn true` (the other result shape),
and once with the rendered function's identifier `fnIdent 18` read as
`fnIdent 19` (another function). Command: `lean` on each restated module
beside the environment (`conformance_sp_09` plants both in the first root of
each project). Expected: no composition of certificates A and B proves the
restatement.

```text
E_fall.lean:45:130: error: Type mismatch
  hr
has type
  LexLeanPreservation.Rust.RealizesFn
E_func.lean:45:25: error: Application type mismatch: The argument
  hc
has type
  LexLeanPreservation.Rust.RCI RustStd.krate (LexLeanPreservation.Rust.fnIdent 18)
```

The negative fixture `certificate-e-rejected` (a lake overlay swaps the
result shape of every certificate E) fails `lexlean verify` with `LLV7016`
and nothing published. Removed: the restatements were deleted; every
certificate E compiles, replays, and depends on exactly the three axioms.

### boundary validators can fail

Planted, in the lowered program of the recursive-tree root and with the
certificate still derived from the source (`conformance_sp_10`): the entry
calls the root without its first validator, and the last order-checking
validator's `eq` arm answers true, admitting an equal key. Expected: Lean
refuses each at the relation of the function it changed.

```text
Planted/Validation.lean:381:1236: error: Application type mismatch: The argument
  __vrel_2 g
Planted/Weakening.lean:352:333: error: Application type mismatch: The argument
  rfl
```

The first lies in the declaration `entry` (function 18), the second in
`__vrel_16`; `conformance_sp_10` requires the declaration to be the changed
function's own. Planted, in a rendered crate (`conformance_sp_08`): the same
entry's crate without its first validator's `if`; the aligner derives
nothing (`function 18: no conditional form fits: a call is not rendered as a
call; …`) and Lean rejects the unmutated derivation restated over the
mutated crate. Planted, in the lowered program: a call of a validator from
the root, and the entry removed while the validators remain. Expected: the
lowering's boundary audit (`LLI9001`) refuses both.

```text
function 0 references function 2: a boundary validator runs only at the entry, because inside the program §17.12's invariants hold by construction and are proof-only
0 entries for 16 boundary validators
```

Removed: every plant was removed; the entry and its 16 validators for the
recursive-tree root certify in A, B, and E, and the differential's inputs
that break an invariant are refused with `none` by the interpreter, by
Lean's `denoteEntry`, by the declared machine, and by rustc.

### the lowered-program mutations are targeted

Planted (`conformance_sp_02`): a checked addition bounded as a subtraction
(`Coverage.Prims.fixedSmall`) and the bounds of a slice exchanged
(`Coverage.Prims.sequences`), beside the five earlier mutations. Expected:
Lean rejects each inside `__rel_0`, the relation of the function changed, and
`conformance_sp_02` requires the declaration of the first error to be exactly
that relation.

```text
Planted/Overflow.lean:30:304: error: Application type mismatch: The argument
  LexLeanPreservation.prim_checkedAdd_i8 a b
Planted/Buffer.lean:30:3029: error: Application type mismatch: The argument
  LexLeanPreservation.conv_value
```

Removed: the mutations were removed; all nine kinds (seven of the closure,
two of the boundary) are planted in the coverage example in each run.

### the rustc differential of the declared machine can fail

Planted: `nat_lt` of the Rust runtime written as `a <= b` in
`crates/lexlean/src/calculus/rust/runtime.rs`. Command: `cargo test -p
repo-conformance --test conformance -- conformance_sp_11`. Expected: a root's
package prints another outcome than the interpreter.

```text
thread 'conformance_sp_11' (17125) panicked at crates/conformance/src/preservation.rs:2630:5:
root_3_core run 0: rustc {"kind":"nat","value":"1"} != interpreter {"kind":"nat","value":"2"}
root_3_std run 0: rustc {"kind":"nat","value":"1"} != interpreter {"kind":"nat","value":"2"}
```

Removed: the runtime was restored; the 68 packages build and 504 runs agree,
96 of them through a boundary entry.

### the boundary is judged from the source types

At `e43d31a` the validated set was whatever the lowering wrote into the
entry's layout, and certificate A then proved the weaker statement it
implied: a lowering that skipped the validator of a parameter was certified
(`conformance_sp_10` and `lexlean verify` passed). Planted in
`crates/lexlean/src/production/lower.rs`, first the entry omitting the
validator of the parameter named `spare : list (map nat bool)`, then
`carries` answering false for a list. Command: `cargo test -p repo-conformance
--test conformance -- conformance_sp_10`; and `lexlean verify` on a project
whose root takes `spare : list (map nat bool)` with the second plant.
Expected: the certificate generator, which decides which parameters carry
§17.12's invariant from the source types by its own recursion, refuses the
lowering before any theorem is written.

```text
thread 'conformance_sp_10' (25266) panicked at crates/conformance/src/cases/preservation.rs:2143:40:
the lowering's own boundary is accepted: Diagnostic { code: DiagnosticCode("LLI9001"), message: "phase preservation: parameter 3: `List (Map (Nat) (Bool))` carries §17.12's invariant but the entry does not validate it", …
thread 'conformance_sp_10' (30777) panicked at crates/conformance/src/cases/preservation.rs:2143:40:
the lowering's own boundary is accepted: Diagnostic { code: DiagnosticCode("LLI9001"), message: "phase preservation: parameter 3: `List (Map (Nat) (Bool))` carries §17.12's invariant but the entry does not validate it", …
error[LLI9001]: phase preservation: a parameter carries §17.12's invariant but the root has no entry
```

Removed: both plants were removed. `conformance_sp_10` also plants the same
defects itself, in the lowered program (a skipped parameter validator for
every validated parameter of every entry, a validator replaced by the
trivial one for every non-trivial validator), requires the generator to
refuse each naming the parameter or the validator, and requires the entry to
exist exactly for the roots whose parameters hold a map or a set by its own
oracle (`differential::carriers`), and an input breaking each validated
parameter's invariant in turn to be refused with `none` by the interpreter,
Lean, the machine, and rustc.

### the statement vocabulary quoted in SPEC.md can fail

Planted, first in `SPEC.md`, `Rel`'s quoted definition ending in `.stuck`
instead of `.overflow`, then in the committed audit output
`examples/production/expected/verify/preserve/audit.txt`, which is what the
pinned Lean prints of the generated library, the same word changed on `Rel`'s
line. Command: `cargo xtask validate-spec-links`. Expected: a statement changed
on one side only is refused.

```text
gate failed: §17.17 (SP-04): SPEC.md does not quote the statement of `LexLeanPreservation.Rel` as Lean prints it
```

Both plants print this line. Removed: both edits were reverted; the gate
reports 352 table rows bijective with the register. The unit tests of
`repo_model::vocabulary` plant a changed statement, a missing quotation, a
quotation out of order, no block, no vocabulary section, and a statement the
audit module no longer prints, and require each to be named;
`conformance_sp_04` requires every declaration the library states, by name, to
be printed by the audit module under its native row's exact policy.

### the preservation library is LexLean source and a defect in it is refused

The library (`language/preservation-1.2/library/src/`, 13 native
`lexlean/core-module/2` modules) is the only source of the Lean every
verification compiles, so a defect is planted in the source and must be seen
where the Lean the compiler generates from it is checked. The plants below
that change a native module were run with `lexlean verify` against the pinned
Lean 4.32.1 in the project of `tests/negative/preservation-drift` (two modules,
one production root); the rest were run through their conformance tests. After
each, the file was restored with `git checkout` and the compiler rebuilt.

Planted (a): a hand-written Lean file `LexLeanPreservation/Core.lean` placed in
`language/preservation-1.2/library/`. Command: `cargo test -p repo-conformance
--test conformance -- conformance_sp_12`. Expected: the library project holds no
Lean file of its own.

```text
thread 'conformance_sp_12' panicked at crates/conformance/src/cases/preservation.rs:
<root>/language/preservation-1.2/library/LexLeanPreservation/Core.lean: an independently authored Lean file lies in the library project
```

Planted (b): the policy of `LexLeanPreservation.ListEnc.items_cons` in
`src/Values.lex.tex` changed from `{"axioms":[],"kind":"exact"}` to
`{"axioms":["propext"],"kind":"exact"}`. Command: `lexlean lock && lexlean
verify`. Expected: the audit module prints what Lean computes, which is no
axiom, and the registered set is refused. (The first attempt changed the policy
of a generated constructor row, `ListEnc.mk`, and verified: a row the
elaborator generates with its inductive is covered by the inductive's own
audit and is not printed by name. The plant was moved to an audited row.)

```text
error[LLV7014]: preservation environment: the library declaration `LexLeanPreservation.ListEnc.items_cons` depends on [], registered ["propext"]
```

Planted (c): the first numeral node of `src/Values.lex.tex`
(`{"k":"n","v":"1"}`) changed to `8`. Command: `lexlean lock && lexlean
verify`. Expected: the kernel refuses the first declaration whose term uses it
when the decoder submits it.

```text
error[LLV7014]: preservation environment: `LexLeanPreservation.Values` does not compile silently under the pinned Lean: error: native core declaration 'LexLeanPreservation.construct_succ': (kernel) application type mismatch
```

Planted (d): the `arity` of a sparse `casesOn` record in `src/Keys.lex.tex`
changed from 4 to 5. Command: `lexlean lock && lexlean verify`. Expected: the
record the decoder gets from the elaborator's own construction differs from the
one the row states, and the module is refused.

```text
error[LLV7014]: preservation environment: `LexLeanPreservation.Keys` does not compile silently under the pinned Lean: error: native core declaration '_private.LexLeanPreservation.Keys.#0.LexLeanTarget.TargetSemantics.LexLeanCollections.compareCodes.match_1.splitter._sparseCasesOn_2':
  core declaration '_private.LexLeanPreservation.Keys.0.LexLeanTarget.TargetSemantics.LexLeanCollections.compareCodes.match_1.splitter._sparseCasesOn_2' states another sparse casesOn record than the elaborator keeps
```

Planted (e): the check that a `grind` attribute belongs to a theorem removed
from `validate_version_2_row` (`crates/lexlean/src/ir/core.rs`) by prefixing its
condition with `false &&`. Command: `cargo test -p repo-conformance --test
conformance -- conformance_sm_31`. Expected: the fixture whose definition row
carries a `grind` registration is no longer refused.

```text
thread 'conformance_sm_31' panicked at crates/conformance/src/support.rs:514:14:
check fails
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 351 filtered out
```

Planted (f): the depth bound of a native module in `crates/lexlean/src/link.rs`
replaced by `0` (`core.expression_depth()` no longer measured). Same command.
Expected: the module nested beyond `max_scope_depth` is accepted instead of
refused with `LLS8002`.

```text
thread 'conformance_sm_31' panicked at crates/conformance/src/support.rs:514:14:
check fails
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 351 filtered out
```

Planted (g): the check in `build_native`
(`crates/lexlean/src/production/preserve.rs`) that a row pins its axioms
exactly disabled. Command: `cargo test -p repo-conformance --test conformance
-- conformance_sp_12`. Expected: the source weakened to an inexact policy is
no longer refused.

```text
thread 'conformance_sp_12' panicked at crates/conformance/src/cases/preservation.rs:1692:22:
an inexact policy is refused: 7740
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 351 filtered out
```

Planted (h): a Lean file `examples/Planted.lean` that declares `namespace
LexLeanPreservation`, outside the library's project; then the directory
`tests/preservation-library/oracle`, the completed one-time migration input,
recreated. Command: `cargo test -p repo-conformance --test conformance --
conformance_sp_12`. Expected: the permanent condition `VR-19` states of the
Atlas holds of this library too, over the whole release tree.

```text
thread 'conformance_sp_12' panicked at crates/conformance/src/support.rs:63:9:
<root>/examples/Planted.lean: an independently authored Lean file declares `namespace LexLeanPreservation`
thread 'conformance_sp_12' panicked at crates/conformance/src/support.rs:40:5:
tests/preservation-library/oracle: the completed one-time migration input is absent from the release tree
```

Removed: every plant was reverted; `conformance_sm_31`, `conformance_sp_04`, and
`conformance_sp_12` pass. The negative fixture `tests/negative/preservation-drift`
pins the other direction: a pinned Lean whose generated `Core` no longer
elaborates is `LLV7014`, its message naming the generated module and line.

What was not run on the development host: the verification of
`examples/uor-atlas` (about 11.5 GB of memory, beyond the host; language 1.0, so
its records do not depend on this change and CI reverifies them) and
`conformance_vr_19`. The CI legs run both.

### a derived row is what the pinned Lean derives

A version 2 row whose name the pinned Lean reserves is made by Lean's own
reserved-name action and refused unless it equals the row (`SM-33`, §17.10,
#91). Run with the pinned Lean 4.32.1 on WSL, on `main` `c183f250` with the
change. The plants that change the library or the decoder were run with
`lexlean lock && lexlean verify` in a copy of the project of
`tests/negative/preservation-drift`, the compiler rebuilt for each, and the
file restored with `git checkout`. Unplanted, the same command verifies
(`verified 2 modules`): all 7,071 derived rows of the library are remade
exactly.

Planted (a): the value of the derived row
`entriesOf.match_3.eq_1` in `src/RustMatches.lex.tex` replaced by its own
type node (`@234896` to `@234887`). Expected: the theorem Lean's matcher
equations make differs from the row, and the module is refused.

```text
error[LLV7014]: preservation environment: `LexLeanPreservation.RustMatches` does not compile silently under the pinned Lean: error: native core declaration '_private.LexLeanPreservation.RustMatches.#0.LexLeanPreservation.Rust.entriesOf.match_3.eq_1':
  core declaration '_private.LexLeanPreservation.RustMatches.0.LexLeanPreservation.Rust.entriesOf.match_3.eq_1' is not what the elaborator's construction makes
```

Planted (b): a remade sparse `casesOn` no longer entered in Lean's sparse
`casesOn` cache (`crates/lexlean/src/backend/core_v2.rs`, the loop over the
two keys made empty). Expected: the splitter proof of a matcher finds no
cached constant under the private key, makes its own, and differs from the
row.

```text
error[LLV7014]: preservation environment: `LexLeanPreservation.RustLemmas` does not compile silently under the pinned Lean: error: native core declaration '_private.LexLeanPreservation.RustLemmas.#0.LexLeanPreservation.Rust.sem_part.match_1_1.eq_2':
  core declaration '_private.LexLeanPreservation.RustLemmas.0.LexLeanPreservation.Rust.sem_part.match_1_1.eq_2' is not what the elaborator's construction makes
```

Planted (c): the decoder's own constants kept in the cache (the filter at the
start of `decodeAndAdd` removed). Expected: the splitter of
`entriesOf.match_3` takes the decoder's `addOneUnchecked._sparseCasesOn_1`,
cached under the same key, instead of the module's constant.

```text
error[LLV7014]: preservation environment: `LexLeanPreservation.RustMatches` does not compile silently under the pinned Lean: error: native core declaration '_private.LexLeanPreservation.RustMatches.#0.LexLeanPreservation.Rust.entriesOf.match_3.eq_2':
  core declaration '_private.LexLeanPreservation.RustMatches.0.LexLeanPreservation.Rust.entriesOf.match_3.eq_2' is not what the elaborator's construction makes
```

Planted (d): the comparison in `sameAsRow` disabled (its condition prefixed
with `true ||`). Command: `cargo test -p repo-conformance --test conformance
-- conformance_sm_33`. Expected: the fixture whose `eq_1` has another proof of
the same statement is no longer refused.

```text
thread 'conformance_sm_33' panicked at crates/conformance/src/support.rs:641:14:
verify fails
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 386 filtered out
```

Removed: every plant was reverted; `conformance_sm_33`, `conformance_sm_31`,
`conformance_sp_12`, and `conformance_rp_07` pass, and `cargo xtask check-fixtures` holds 269 fixtures equal (on `main` `32afe5da`).
### the fold laws can fail

Integer order, `linear_arithmetic` over the runtime, the fold laws, and the
`law` form (#117, `PF-20`..`PF-23`). Each defect was planted in the working
tree, the case named was run with `cargo test -p repo-conformance
--all-features --test conformance -- <case>` (Linux x86-64, pinned Lean
4.32.1), and the plant was reverted. Each case passes on the reverted tree.

Planted (a): order typed over `Nat` only under language 1.2 (the Int and
fixed-width arm of `le`/`lt` disabled). `conformance_pf_20`: the committed
corpus no longer links.

```text
check succeeds: LexLeanError { ... code: DiagnosticCode("LLT4001"), message: "phase link: order left operand has type Int, expected Nat", primary: Some(Span { path: "src/Kernels.lex.tex", ... }) ... }
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 389 filtered out
```

Planted (b): the script without `all_goals (repeat' split)`.
`conformance_pf_21` refuses the rendered script (`the fixed script with the
unfolded definition`). The same compiler run by `lexlean verify` on a copy of
`examples/fold-laws` shows what the split is for: Lean refuses the first
kernel whose zero-divisor `if` is left unsplit, with
`error[LLV7002]: Lean rejected FoldLaws.Kernels (error): omega could not
prove the goal` at `src/Kernels.lex.tex:79` (`halving_shrinks`).

Planted (c): the backend leaves out one registered rewrite
(`LexLeanLaws.listFold_add_nat_self`) from the script. `conformance_pf_22`,
whose expected script is read from the committed registry rather than from
the compiler, refuses the rendered Lean of `doubled_total`
(`proofs.rs:1087`).

Planted (d): the rewrite removed from `language/runtime-1.2/laws.toml`
itself. The registry is language data hashed into the language-1.2
compiler-semantics ID (§21.2), so `conformance_pf_22` fails before any
backend runs, on the identity: `LLC0102: lexlean.lock is stale or
noncanonical` (`the compiler semantics changed since locking`). A registry
that names a declaration the module lacks, or registers a law twice, and a
laws row that only bounds its axioms are refused by the runtime build. Those
three are plants the case makes itself, each asserted by its message.

Planted (e): the argument types of a `law` application unchecked.
`conformance_pf_23`: the copy that passes `true` for a `Nat` argument links
(`check fails` is not met).

```text
thread 'conformance_pf_23' panicked at crates/conformance/src/support.rs:619:14:
check fails
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 389 filtered out
```

The false statements are refused by Lean in every run of the cases, not only
when planted. `conformance_pf_21` refuses a clamp bound one too small, a
quotient identity without the remainder's sign, and a fixed-width order one
past its literal. `conformance_pf_22` refuses a doubled fold equated to a sum
and a count, the reverse's sum equated to a count, and a map's entries summed
as its keys twice. `conformance_pf_23` refuses the pair law applied to a
swapped statement. Each assertion names exactly the refused theorems.

### a derived row is made by the pinned Lean and never stored (SM-34)

A version 2 row may be derived: it states no universe parameters, type, or value,
and the decoder makes the constant with the pinned Lean's own construction (`SM-34`,
§17.10, #91 stage 2). Run with the pinned Lean 4.32.1 in a Linux container, on
`main` `efea57d8` with the change. The plants that change the library or the
decoder were run with `lexlean lock && lexlean verify` in a copy of the project of
`tests/negative/preservation-drift`, the compiler rebuilt for each, and the file
restored with `git checkout`. Unplanted, the same command verifies (`verified 2
modules`): all 7,203 derived rows of the library are made by Lean.

Planted (a): the name of the derived row `LexLeanPreservation.Rel.eq_1` in
`src/Core.lex.tex` replaced by `LexLeanPreservation.Rel.eq_x`, a name the pinned Lean
does not reserve. Expected: the module is refused at the row.

```text
error[LLV7014]: preservation environment: `LexLeanPreservation.Core` does not compile silently under the pinned Lean: error: native core declaration 'LexLeanPreservation.Rel.eq_x':
  derived core declaration 'LexLeanPreservation.Rel.eq_x' bears a name the elaborator does not reserve and states no sparse casesOn record
```

Planted (b): in `src/RustBase.lex.tex`, the record of the derived sparse `casesOn`
`LexLeanPreservation.Rust.WT._sparseCasesOn_1` with its last constructor
(`Value.bytes`) removed and its arity left at 14. Expected: the record the
construction keeps is not the row's.

```text
error[LLV7014]: preservation environment: `LexLeanPreservation.RustBase` does not compile silently under the pinned Lean: error: native core declaration 'LexLeanPreservation.Rust.WT._sparseCasesOn_1':
  core declaration 'LexLeanPreservation.Rust.WT._sparseCasesOn_1' states another sparse casesOn record than the elaborator keeps
```

Planted (c): the same record with `Value.bytes` replaced by `Value.nat`, so that its
size, arity, and major premise are the row's own. Expected, and found: the
construction makes another constant under the name, and the stored matcher
`WT.match_1`, whose value applies it, is refused by the kernel.

```text
error[LLV7014]: preservation environment: `LexLeanPreservation.RustBase` does not compile silently under the pinned Lean: error: native core declaration 'LexLeanPreservation.Rust.WT.match_1':
  (kernel) application type mismatch
    @LexLeanPreservation.Rust.WT._sparseCasesOn_1 (fun x => motive x x✝) x✝¹
```

What this does not catch, found by running it: the constructors of a sparse
record are the input of the construction, so the same plant on a sparse `casesOn` no
stored row applies (the `Ordering.eq` record of `Keys`, with `Ordering.lt` in its place)
is made, differently, and the library verifies. The recipe is checked against its
arity and major premise, and against every stored row that uses the constant, and
not against the constructors the original elaboration chose.

Planted (e): the record of `LexLeanPreservation.Rust.WT._sparseCasesOn_1` naming
`Value.clos`, which is no constructor of `Value`. Expected: refused.

```text
error[LLV7014]: preservation environment: `LexLeanPreservation.RustBase` does not compile silently under the pinned Lean: error: native core declaration 'LexLeanPreservation.Rust.WT._sparseCasesOn_1':
  core declaration 'LexLeanPreservation.Rust.WT._sparseCasesOn_1' names 'LexLeanTarget.TargetSyntax.Value.clos', which is not a constructor of 'LexLeanTarget.TargetSyntax.Value'
```

Planted (f): the same record naming `Value.adt` twice. Expected: refused.

```text
  core declaration 'LexLeanPreservation.Rust.WT._sparseCasesOn_1' names the constructor 'LexLeanTarget.TargetSyntax.Value.adt' twice
```

Planted (g): the row `splitter._sparseCasesOn_2` of `src/Keys.lex.tex` removed, so
that the construction of the splitter makes the constant itself. Expected: the module
does not state what the construction made.

```text
error[LLV7014]: preservation environment: `LexLeanPreservation.Keys` does not compile silently under the pinned Lean: error: the construction made the sparse casesOn '_private.LexLeanPreservation.Keys.0.LexLeanCollections.compareCodes.match_1.splitter._sparseCasesOn_2', which the module does not state
```

Not distinguished, and recorded as a non-evidence note: the constructors of the
`Ordering.eq` record of `Keys` replaced by `Ordering.lt` (command as in (h), run in the
same way) still verifies with the use-scan of (h) in place. The splitter that
refers to the constant is made by the pinned Lean from the constant it finds, so the
use is of the swapped constant, and nothing in the module can tell. The content of a
derived row is checked by its uses alone, and SM-34 says so; no further machinery is
added.

Planted (h): a derived sparse `casesOn` row nothing refers to, added to
`src/Keys.lex.tex` (a copy of the `splitter._sparseCasesOn_2` row named
`_sparseCasesOn_3`, with `Ordering.gt` for `Ordering.eq`), built and verified in a
copy of the project of `tests/negative/preservation-drift` with `lexlean lock &&
lexlean verify`. Expected: refused, because the use-scan finds no row that refers to
it. The unplanted library verifies (`verified 2 modules`) with the scan, so every one
of its 313 derived sparse rows is referred to.

```text
error[LLV7014]: preservation environment: `LexLeanPreservation.Keys` does not compile silently under the pinned Lean: error: the derived sparse casesOn '_private.LexLeanPreservation.Keys.0.LexLeanCollections.compareCodes.match_1.splitter._sparseCasesOn_3' is referred to by no row of the module
```

Planted (d): the name of `LexLeanPreservation.Rel.eq_1` replaced by `Rel.eq_2`,
which is reserved but which the construction does not make. Expected: refused.

```text
error[LLV7014]: preservation environment: `LexLeanPreservation.Core` does not compile silently under the pinned Lean: error: native core declaration 'LexLeanPreservation.Rel.eq_2':
  derived core declaration 'LexLeanPreservation.Rel.eq_2' bears a name the elaborator reserves, and its construction does not make it
```

Planted (e): a stored row back in the library, `src/Compose.lex.tex` replaced by its
version at `efea57d8`. Command: `cargo test -p repo-conformance --test conformance --
conformance_sm_34`. Expected: the gate over the library names the row.

```text
thread 'conformance_sm_34' panicked at crates/conformance/src/cases/semantic_ir.rs:2719:21:
assertion `left == right` failed: .../language/preservation-1.2/library/src/Compose.lex.tex: `LexLeanPreservation.Rust.Representable._sparseCasesOn_1` is stored and must be derived
```

Planted (f): the check that the constant the construction made is the row's kind
removed from `addDerived` (`crates/lexlean/src/backend/core_v2.rs`, the condition
prefixed with `true ||`). Command: `conformance_sm_34`. Expected: the copy whose
theorem is a `definition` is no longer refused.

```text
thread 'conformance_sm_34' panicked at crates/conformance/src/support.rs:757:14:
verify fails
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 414 filtered out
```

Planted (g): the check that a derived row states none of what the construction
decides removed from the linker (`validate_derived_row`'s result discarded in
`crates/lexlean/src/ir/core.rs`). Command: `conformance_sm_34`. Expected: the copies
that state a type, a value, or universe parameters are no longer refused.

```text
thread 'conformance_sm_34' panicked at crates/conformance/src/support.rs:730:14:
check fails
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 414 filtered out
```

Planted (h): `derived` allowed with a `type` in the schema's `derivedDeclaration`.
Command: `conformance_sm_34`. Expected: the schema accepts the derived row with a
type and the case says so.

```text
thread 'conformance_sm_34' panicked at crates/conformance/src/cases/semantic_ir.rs:2629:17:
the schema accepts a derived row with a type
```

Planted (i): the branch of the decoder that handles a derived row disabled
(`if (optionalField json "derived").isSome` replaced by `if false`). Command:
`conformance_sm_34`. Expected: the derived module is no longer verified, so the
case fails before it reaches any refusal.

```text
thread 'conformance_sm_34' panicked at crates/conformance/src/support.rs:2286:10:
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 414 filtered out
```

Removed: every plant was reverted. `conformance_sm_31`, `conformance_sm_33`, and
`conformance_sm_34` pass with the change, and `cargo test -p lexlean --lib` passes
(200 tests). What was not run is listed in the pull request of the change: no record
bound to the language-1.2 compiler-semantics ID was regenerated, so every case that
reads a committed lock (for example `conformance_au_10`, `conformance_au_11`, and
`conformance_au_12`) reports `LLC0102` until the train regenerates.

### the machine's primitives and the overflow literal can fail (SP-18)

`RustSyntax` declares eight machine primitives, `Callee.machine`, and `Lit.overflow`,
and `RustSemantics` evaluates them (`SP-18`, §17.14, §17.16, #91 stage 2). The case
reads the constructors from the committed `compiler/src` modules, and has the pinned
Lean 4.32.1 evaluate 19 claims over the shipped copies of the modules (the boundary
values of `u8` and `i8`, a shift by the width and by the width less one, an
operand of another width, `try_from` out of and into range, the overflow literal,
and the same calls through `eval`). Each plant was run on `main` `efea57d8` with the
change in a Linux container, with `cargo test -p repo-conformance --test conformance --
conformance_sp_18`; the two that change the semantics were regenerated into the
shipped copies (`lexlean build` in a copy of `compiler/`) before the run, and every
file was restored with `git checkout`.

Planted (a): `machineAccepts` made `true`, so a shift of a `u16` is run as a shift of
a `u8`. Expected: the claim that it is stuck fails.

```text
the machine does not evaluate as stated:
/tmp/.tmpF9nFg5/Machine.lean:16:100: error: Type mismatch
  rfl
```

Planted (b): the literal `Overflow` evaluated to `Value.bool false` in `literal`.
Expected: the claims about the literal, and about `Err(Overflow)` built by `eval`,
fail.

```text
the machine does not evaluate as stated:
/tmp/.tmpdTa9It/Machine.lean:19:52: error: Type mismatch
```

Planted (c): the `tryFrom` primitive removed from `Machine` in
`compiler/src/RustSyntax.lex.tex`. Expected: the declared primitives are not the eight.

```text
assertion `left == right` failed: the machine's primitives, each at a width
  left: [("checkedAdd", 1), …, ("checkedShr", 1)]
 right: [("checkedAdd", 1), …, ("checkedShr", 1), ("tryFrom", 1)]
```

Planted (d): `overflow` removed from `Lit`. Expected: the literal is not declared.

```text
the literal Overflow is declared
```

Not planted: the arm of `eval` that runs a machine callee. A module without it does
not link (the match is not total), which is the semantic modules' own check, not this
case's. The five authored declarations of the library that the constructors leave a
case short are in `tests/preservation-library/MIGRATION.md`, with the run that shows
each is needed.

### the loop over a state can fail (SP-19)

`RustSyntax` declares `Expr.loop` over a state binding and a body block, and
`RustSemantics` evaluates it by recursing through `eval` (`SP-19`, §17.14, §17.16,
#91 stage 3). The case reads the constructor from the committed `compiler/src`
module, and has the pinned Lean 4.32.1 evaluate four claims over the shipped copies
of the modules (a countdown from three ends at zero, a countdown from zero ends at
once, the same countdown at fuel two is exhausted, and a body whose value is neither
`Ok` nor `Err` is stuck), and refuse three wrong statements of the same loops. The
Rust side is covered by unit tests in `calculus::rust::ast` (the printed loop rebinds
the state and breaks with the payload of `Err`; the validator accepts a body that
moves only the state and refuses one that moves another binding), by the exhaustive
matches of the moves pass, the lowering, the validator, and the certificate-B term
builder, and by `audit-production`, which needs a disposition for the new
construct. Each plant was run in a Linux container with `cargo test -p
repo-conformance --test conformance -- conformance_sp_19`; the files were restored
afterwards and compared byte for byte.

Planted (a): the shipped `RustSemantics` evaluating the loop again without rebinding
the state. Expected: the countdown claim fails. Observed: caught earlier, because
the library's proof `stable_loop` states the equation of `eval` for `loop`; the
shipped module is rejected.

```text
the shipped module `LexLeanPreservation.RustBase` was rejected:
… error: native core declaration 'LexLeanPreservation.Rust.stable_loop':
  (kernel) application type mismatch
```

Planted (b): the shipped `RustSemantics` ending the loop with the unit value
instead of the payload of `Err`. Expected: the countdown claim fails. Observed: the
same rejection of `stable_loop`.

Planted (c): the constructor `loop` removed from `Expr` in
`compiler/src/RustSyntax.lex.tex`. Expected: the declared expressions lack the loop.

```text
thread 'conformance_sp_19' panicked at crates/conformance/src/cases/preservation.rs:4795:18:
the expression Loop is declared
```

Plants (a) and (b) are caught by the proof library before the claims run, which is
the stronger gate and not the one this case adds; the claims themselves are shown to
be able to fail by the three statements of a wrong value, each of which the pinned
Lean refuses (the loop ending at one, the exhausted loop stated as a value, and the
stuck loop stated as the unit value).

### a function rendered as a loop is certified, and a loop outside the rendering is refused (SP-20)

`FunOK` is `FunOK0` or `FunLoop` (`SP-20`, §17.16, §17.17, #91 stage 4). The case
reads the library's modules for the declarations (`FunOK0`, `FunOK`, `FunLoop`,
`loopOf`, `loopCheckOf`, `funSem_loop`, `funSem_of`, `sound`), lowers the renderer
fixtures in each profile and the production roots of `examples/production`,
`examples/production-coverage`, `examples/models`, and `examples/reasoning` in each
of their targets under a lock computed in the case, reads every loop of every
rendering back into its recursive body with `loopform::recover` (which also
requires the library's form, transliterated, to be the crate's function), asserts
that there is a loop that changes a parameter and one that keeps another (the
case is armed by the first loop), plants six breaks of each loop in every loop
(a state of another type, no destructuring of the state, an end that is not an
`Err`, a renamed parameter the body reads, a pattern that rebinds a kept parameter,
a state held by a parameter the function does not have) and requires each refused,
and has the pinned Lean 4.32.1 compile certificate B of the fixtures rendered as
loops over the shipped library with the library audit and the exact axioms of the
root theorems. The unit tests of `data_b::loopform` restore a countdown loop to its
recursive body and refuse three breaks. Each plant was run in a Linux container with
`cargo test -p repo-conformance --test conformance -- conformance_sp_20`; the file
was restored with `git checkout` afterwards.

Planted (a): the form building an end as `Ok` (`Retarget::end`). Expected: the
rendering of the first loop is refused. Observed:

```text
thread 'conformance_sp_20' panicked at crates/conformance/src/preservation.rs:2530:21:
graph-reachable (rust-std): certificate B: function 2: the loop is not the loop form of the recursive body it restores to
```

Planted (b): `recover` returning the inverted body without requiring the library's
form to be the crate's function. Expected: a broken loop is accepted. Observed:

```text
thread 'conformance_sp_20' panicked at crates/conformance/src/cases/preservation.rs:5078:21:
graph-reachable (rust-std): function 2: the mutation 4 of a loop is accepted
```

(Mutations 0 to 3 are refused by the inversion itself; the pattern that rebinds a
kept parameter is the first the library's checks alone refuse.)

Planted (c): the holder number of the `FunLoop` witnesses one too large. Expected:
the kernel refuses the equation that `loopOf` holds. Observed:

```text
renderings: `LexLeanPreserve.Fixture.F44.RustStd` was rejected:
… error: native core declaration '…RustStd.fun11':
  (kernel) application type mismatch
    And.intro
      (Eq.refl (LexLeanPreservation.Rust.Loop.loopOf 11 [Pos.changed, Pos.changed] …
```

What was not run in this change: the cases that read a committed lock, golden, or
production export (`conformance_sp_08` among them) report that the compiler
semantics changed since locking, as they do on the branch before this change; the
maintainer regenerates those records. `examples/production-coverage`, whose crates
hold loops (`loop {` appears in 37 of its 52 rendered crates),
was verified end to end in a copy: `lexlean lock && lexlean verify` printed
`verified 8 modules; attestation 16b5350dcfb7abac988cce6d7c8aedb381b77af343de4ce68a4dbf12970626f8`
after 20 min 17 s, with certificates A, B, and E of every root replayed and audited.

The decoder fix #212 makes (a compiled mutual group is never split across commands) is needed by the new library and is not part of this change: with the #212 branch merged
in, all 13 modules compile; without it `patBinders`/`patBindersList` are compiled in two commands and Lean rejects them.

### the machine's bitwise, order, and remainder primitives can fail (SP-21)

`RustSyntax.Machine` gains nine constructors (`bitAnd`, `bitOr`, `bitXor`, `bitNot`
at a width; `natEq`, `natLe`, `natLt`, `natRem`, `intRem`), each evaluated by
`RustSemantics` as the calculus primitive of the same name (`SP-21`, §17.14, §17.16,
#91 stage 4, PR 1). The case reads the constructors from the committed
`compiler/src` module (the first eight of `SP-18`, then these nine, seventeen in all)
and has the pinned Lean 4.32.1 evaluate fifteen claims over the shipped copies
(bitwise results at `u8` and `i8`, an operand of another width, the three orders
at equal operands, the remainders including a zero divisor, and one call through
`eval`) and refuse four wrong statements of the same calls. The shipped
`RustSyntax.lean` and `RustSemantics.lean` were regenerated from `compiler/src` with
`lexlean build`; the other two machine modules are unchanged. Growing `Machine`
cost no library repair: `conformance_sp_13` (every library module, compiled by the
pinned Lean over the regenerated modules) passes unchanged, 390 s on this host.
Plants were run on the branch head with `cargo test -p repo-conformance --test
conformance -- conformance_sp_21`; files restored with `git checkout` after each.

Planted (a): `natLt` evaluated as the primitive `natLe` in `machinePrimitive` (the
module regenerated into the shipped copy). Expected: the claim that `4 < 4` is
false fails.

```text
the machine does not evaluate as stated:
…/Machine.lean:14:101: error: Type mismatch
  rfl
```

Planted (b): `intRem` removed from `Machine` in `compiler/src/RustSyntax.lex.tex`.
Expected: the nine declared constructors differ.

```text
assertion `left == right` failed: the bitwise primitives at a width, the order of naturals, and the remainders
  left: [("bitAnd", 1), …, ("natRem", 0)]
 right: [("bitAnd", 1), …, ("natRem", 0), ("intRem", 0)]
```

Not planted: the arms of `machineWidth`, `machineShifts` and `machinePrimitive` for
`bitXor`, `bitOr` and `natEq`, `natLe`, `natRem`, `intRem` individually; each
claim exercises its constructor once, so a swapped arm is caught only where a
claim distinguishes the two results. `SP-18` now checks that the first eight
constructors are its eight; its register statement no longer says "eight".

### a theorem proved by a split on a document term can fail (SM-35)

`term_cases` is a language-1.2 proof form: a document term and a nonempty sorted unique
list of prior document definitions, lowered to `set_option linter.unusedSimpArgs false in cases llCase : <term> <;> simp (config :=
{decide := true}) only [<definitions>, llCase, ite_true, ite_false]` (`SM-35`, §17.10, #91 stage 4). It exists
because the unbounded refinement of a runtime item, `runExport … = runItem …`, is true and
provable only by that script (measured on the `nat_add` slice: six seconds, the axioms
`propext`, `Classical.choice`, `Quot.sound`), and `simplify` (plain `simp only`), `cases` on
a local, and `reflexivity` cannot state it (the two sides are a case-of-case). The case
builds a language-1.2 project whose theorem equates a direct match on `flagOf n` with the same
match wrapped in a second match, proves it by `term_cases{scrutinee: (flagOf n), definitions:
[direct, wrapped]}`, checks that the module satisfies `semantic-module-v2`, that the
formatter keeps the proof and its fixed point links to the same module, that the notation
reads back to the same module, that the generated Lean has the stated lowering, that the
pinned Lean verifies the module with exactly `[propext]` (where the toolchain is installed),
and that the LaTeX catalog carries the theorem's statement and policy. The first lowering named the definitions and `llCase` alone; the runtime's `nat_add` refinement, tried against it, ran out of heartbeats at `isDefEq` because the unfolded `itemFallible item` leaves a conditional on a decided condition that `simp only` without `ite_true` and `ite_false` does not reduce, so the lowering names those two lemmas and, because they are unused in a proof with no conditional, turns the unused-argument linter off for its one script; with them the same statement is proved in six seconds. Seven misuses are
refused as `LLT4001` naming the misuse (the planted projects of the case): language 1.1 (data form, schema 1: "a language-1.2
construct"), an empty list, definitions out of order, a repeated definition, a definition that
is absent, a term of type `Nat`, and a theorem parameter named `llCase`. Each of the seven misuses is also a committed fixture under `tests/negative/`
(`term-cases-language-11`, `-empty-definitions`, `-definitions-unsorted`,
`-definition-repeated`, `-definition-absent`, `-scrutinee-not-inductive`,
`-reserved-name`: seven, each `LLT4001` with the message of the case), listed in `EX-07`'s prescribed
classes (285 to 292). Their locks are current for the head they were written at and are
relocked, with every other fixture's, by the regeneration wave; on this head `EX-07` fails
for the 220 existing fixtures whose lock is stale (`LLC0102`) and for none of the seven. The neighbouring
suites (`semantic-ir`, `proofs`, `declarations`, 76 cases): 54 pass and 22 fail with
`LLC0102`, the lock bound to the compiler-semantics ID, as every case that copies a
committed project does on this stack since `SP-18`; none fails on the schema, the
proof forms, or the snapshot.

Each plant was run with `cargo test -p repo-conformance --test conformance -- conformance_sm_35`
and restored with `git checkout`.

Planted (a): the hypothesis left out of the `simp only` list in the lowering. Expected: the
lowering assertion of the case fails. Observed: it does (the generated module shows
`simp … only [direct, wrapped]`). Not shown: that the pinned Lean then also rejects this
module; the toy theorem may not need the hypothesis, and the `nat_add` slice did not
test it.

Planted (b): the language gate removed. Observed: the language-1.1 project is accepted and
the case fails at the expected refusal (`check fails`).

Planted (c): the sortedness and uniqueness check replaced by acceptance. Observed: the same.

Not planted: the check that the term's type is an inductive of the document, the empty list,
the absent definition, and the reserved name, each of which the case exercises with its own
misuse; their checks were not removed one by one.

### the machine's natural and integer arithmetic can fail (SP-23)

`RustSyntax.Machine` gains nine constructors (`checkedNatAdd`, `natSub`, `checkedNatMul`,
`natQuot`, `checkedIntAdd`, `checkedIntSub`, `checkedIntMul`, `checkedIntNeg`,
`checkedIntQuot`) and `RustSemantics` a definition `machineOptional` that `runMachine`
reads: the seven checked ones return `Some` of the primitive's value, or `None` where the
primitive overflows (the runtime's `nat_add` and the others return `Err(Overflow)` there, and
a body cannot observe `abort`); `natSub` and `natQuot` cannot overflow and are the value
(`SP-23`, §17.14, §17.16, #91 stage 4, PR 2). Found while setting up the `nat_add` slice:
`Machine.checkedAdd u64` accepts only `Value.u64`, and the runtime's `nat_*` and `int_*`
compute on `Value.nat` and `Value.int`, so those nine functions were not expressible over
the first seventeen primitives (corrected on #91). The case reads the constructors from
`compiler/src` and has the pinned Lean 4.32.1 evaluate eighteen claims over the shipped
copies (sums and products at the width of a natural, differences below zero, quotients by
zero, `MIN / -1`, `-MIN`, and an operand of another width, which is stuck) and refuse four
wrong statements. The library compiles over the regenerated modules, with the new body of
`runMachine`, unchanged: `conformance_sp_13` passes in 400 s, so 0 library repairs. The
RB-15 case covers the nine in the same harness (106 functions).

Planted (a): `machineOptional` of `checkedIntNeg` made `false` (regenerated into the shipped
copy). Expected: the claim that `-MIN` is `None` fails.

```text
the machine does not evaluate as stated:
…/Machine.lean:18:109: error: Type mismatch
  rfl
```

Planted (b): `natSub` removed from `Machine` in `compiler/src/RustSyntax.lex.tex`. Expected:
the nine declared constructors differ.

```text
  left: [("checkedNatAdd", 0), ("checkedNatMul", 0), ("natQuot", 0), …]
 right: [("checkedNatAdd", 0), ("natSub", 0), ("checkedNatMul", 0), …]
```

A first run of plant (b) removed the `natSub` of `Item` instead and the case passed, which
is the correct answer to that edit and not a plant of this case.

### the width families of the core runtime are source (RB-16, SP-22, widths branch)

`claude/issue-91-stage4-widths-2` writes out the 92 functions of the eight widths in `RustRuntime`:
`fixed_<w>::checked_add`, `checked_sub`, `checked_mul`, `checked_quot`, `bit_and`, `bit_or`, `bit_xor`,
`bit_not`, `shift_left`, `shift_right`, and `convert` (88), and `checked_neg_<w>` for the four signed
widths (4), each a call of a machine primitive, with a `refines_<path with _>` theorem each. The
runtime is then 110 functions and 110 theorems. `RustType.i128` is a node across the machine for
`convert`'s operand (the `Type::I128` of the AST and the decoder, the Lean constructor, the certificate
B encoder, the leaf `LkI128` of `RustValidate`, one more hand-built crate: BS-05 counts 443/195/306/82,
749 in all). The printer groups the consecutive functions of one module by the path in the exported
name. `rb_16` compiles the oracle and the generated runtime together and runs every one of the 110
functions on 9 boundary values of its width (all pairs, six shift amounts, nine conversion inputs)
and 200 seeded calls: 27 s; three plants of width bodies (`checked_add` of `u8` for `checked_sub`,
`shift_left` of `i32` for `shift_right`, `bit_xor` of `u16` for `bit_or`) fail at their function.
`sp_22` verifies the module of 110 theorems in 8 min 1 s (13 min 51 s of CPU) and its plant on one
theorem in about a minute; the case is 566 s with `rb_16` and `rb_09`, runs on a stack of 64 MiB
(`lexlean::READER_STACK`) and allows its project an hour (`child_timeout_ms`), so `SP-22` is the one case of the part `preservation-runtime` of `bdd`
(`bdd-preservation-runtime` in `.github/vv-rows.json`). The proofs of the width theorems need
`allWidth` and `hasWidth` in their definitions: a head without them did not terminate.

The reader stack is a measured need, not a workaround. The runtime module reads (`sdk::read_module`)
to 1.14 MB of data nested 119 levels, because a list of n elements nests n levels, and the reader
recurses once per level in frames of about 17 KB each in a debug build, which the tests and `vv` are:
run on threads of 2, 4, 8, 16, 32, and 64 MiB, the read aborts with a stack overflow at 2 MiB, which
is what a default or test thread has, and succeeds at 4 MiB. Three places read it on a thread the
caller does not size: `sp_22` before it had its own (`has overflowed its stack`), `rp_10`, which
recomputes the compiler-semantics ID through `identity_bytes` on the test thread (the same), and
the decoder in `runtime::core`. Each now spawns one of `READER_STACK`, 64 MiB, sixteen times the
measured need, so the module can grow with the runtime (each function adds about one level; the
stack is virtual memory the thread touches only as deep as the recursion goes). A release build
needs far less; the 64 MiB is for the debug builds that run the gate.

The identity-bound records (`rust_renderer`, `rust_runtime_core`, `rust_runtime_std` in
`language/semantics-1.2.toml`, the example and fixture locks, the compiler goldens, the packages of
the production export) are regenerated once, by the maintainer at the end of the stack when it is
green on the line, not by each PR: this branch moves the compiler-semantics ID again, on top of
#243's move (`RustSyntax` gains `RustType.i128`), and leaves every such record as it found it.

### a theorem proved by a split on a Boolean local can fail (SM-36)

A language-1.2 `cases` proof may name a local of type `Bool` (`SM-36`, §17.10, #91 stage 4). The
case (`cases/semantic_ir/boolean.rs`) builds a project whose theorem `pickSame` states
`pick b = b` for `pick b := if b then true else false`, proved by `cases{scrutinee: b, branches:
[false, true]}` with `reflexivity` in each; it requires the module to satisfy `semantic-module-v2`,
the formatter to keep the proof and its fixed point to link to the same module, the notation to read
back to the same module, the generated Lean to hold `cases b with`, `| false =>`, and `| true =>`, and
the pinned Lean to verify the module with an empty axiom policy. Five misuses are refused: language
1.1, a missing branch, a repeated branch, a branch with a binder, and `induction` on a Boolean.

Plants, each restored: the `Bool` arm of `elimination_binder_types` disabled (`proof scrutinee `b` is not
Nat, List, or a document inductive` at the accepted project); the language gate removed (the
language-1.1 project is accepted and the case fails at `check fails`); the induction refusal removed
(the same).

### the machine callee and the overflow value are printed and computed (RB-15)

`rust::ast::Callee::Machine` and `Lit::Overflow` mirror the Lean constructors
(`RB-15`, §17.14, §17.16, #91 stage 4, PR 1). The case (`cases/rust_machine.rs`)
builds one crate with a function per machine primitive and width (`Machine::all()`,
97 functions: 5 without a width, 11 at each of 8 widths, and the negation at the 4
signed widths), validates and prints it, requires the printed text to contain eight
named forms, and compiles it with the pinned rustc 1.97.1 under `warnings = "deny"`;
the harness calls every function on the boundary values of its width (minimum,
minimum plus one, minus one, zero, one, maximum minus one, maximum) and compares with
an `i128` computation, including 17 conversion inputs per width, 6 shift amounts per
width, and the `MIN % -1` case of `int_rem`. Two crates are refused (a call with one
operand of two; a stray overflow value), and an `Err(Overflow)` is accepted. The
lowering emits neither construct, so no rendering changes: `rb_07` and the
`lexlean` unit test of the renderer digest fail on this branch only on the records
bound to the compiler-semantics ID (`rust_renderer` in `language/semantics-1.2.toml`
and the `compiler_semantics` field of `provenance.json`), which are left to the
regeneration wave; with the `rust_renderer` digest set locally, all 208 `lexlean`
unit tests pass.

Plants were run with `cargo test -p repo-conformance --test conformance --
conformance_rb_15`, each restored with `git checkout`.

Planted (a): `natLt` printed as `PartialOrd::le`. Expected: the comparison of equal
operands differs. Observed: `assertion left == right failed, left: true, right: false`
at the harness.

Planted (b): `bitXor` printed as `BitOr::bitor`. Expected: `xor u8 1 1` differs.
Observed: `assertion left == right failed: xor u8 1 1, left: 1, right: 0`.

Planted (c): the arity check of the validator removed. Expected: the crate with one
operand of two is accepted, and the case fails because the plant is not refused.
Observed: `the planted crate is refused` panicked.

Planted (d): the stray-overflow refusal removed. Observed: the same panic.

Planted (e): a machine call in a lowered crate (the `sum-to` fixture, the callee of its
first runtime call replaced by `Machine::NatEq`). Expected: `correspond` refuses it as a
construct with no row. Observed with the construct recorded as `call:machine`:
`the construct `call:machine` has no target-semantics correspondence`. With the gate
removed (the construct recorded as nothing), the plant is accepted and the case fails:
`a machine call has no row: ()` at the `expect_err`.

An earlier head printed the operators infix (`(a & b)`); rustc with warnings denied
refused it for redundant parentheses as a block value, which is why the primitives
print as fully qualified trait calls.

### the core runtime is the definitions of RustRuntime (RB-16)

`rust::source` decodes the shipped `RustRuntime` module and `runtime::core()` prints its
functions after the base text (`RB-16`, §17.16, #91 stage 4, PR 2). The module defines
eighteen of the 144 functions: the natural functions `nat_add`, `nat_sub`, `nat_mul`, `nat_quot`,
`nat_rem`, `nat_eq`, `nat_le`, `nat_lt`, and `nat_succ`, and the integer functions `int_add`,
`int_sub`, `int_mul`, `int_neg`, `int_quot`, and `int_rem`, each a call of a machine primitive, and the Boolean functions `bool_not`, `bool_and`, and `bool_or`, a negation and two conditionals (`if p0 { p1 } else { false }`, `if p0 { true } else { p1 }`)
(the fallible ones return `R<T>`; `nat_add` is `match u64::checked_add(p0, p1) { Some(v0) => Ok(v0), None => Err(Overflow) }`, and
the other fallible ones have its shape; the plain ones are the call alone); the handwritten text of the whole runtime is kept at
`tests/rust-runtime-oracle/{core,std}.rs.txt` (its SHA-256 is the `rust_runtime_core` the
language data recorded before this change). The case (`cases/rust_runtime.rs`) requires the shipped
copy to be the compiler project's module byte for byte; requires the module to define at
least one function and the differential to cover exactly the functions it defines; requires
the embedded module to print as `assemble` of the decoded items; compiles the oracle as
module `old` and the generated runtime as module `new` of one crate under the pinned rustc with
warnings denied; and runs every defined function on all pairs (triples with a zero-divisor value
for the quotients and remainders) of 11 boundary values of its type and on 1,000 to 2,000 seeded
calls each, comparing the debug form of the result and the counted work (`work()`, the same for
both: zero). Four bodies changed to another primitive (the checked product for the checked sum of
naturals, the checked difference for the checked sum of integers, the strict order for the loose
one, the remainder for the quotient) are each caught at the function's own `result` assertion.
Six decoder refusals are asserted by the message they give (no `coreCrate`, a definition
with a parameter, a reference to a definition the module lacks, a machine primitive and a
runtime function `RustSyntax` does not declare, a constructor with an extra argument).

The tree-wide gates that read the runtime were run: `rb_01`, `rb_04`, `rb_05`, `rb_06`,
`rb_08`, `rb_09`, `tc_07` pass. `rb_09` needed the library table to list the path forms the generated text uses where
the handwritten text used methods (`<int>::checked_add`, `checked_sub`, `checked_mul`, `checked_div`,
`checked_neg`, `checked_rem`, `saturating_sub`, and `core::cmp::PartialEq::eq`, `PartialOrd::le`,
`PartialOrd::lt`) and to drop the three method rows nothing uses any more (`checked_rem`, `ok_or`,
`saturating_sub`); its plant of a role citing a name that is not a primitive now names the path row
`<int>::saturating_sub`; `tc_07`'s plant of a wrapping addition named the handwritten text and now names the
generated one (`u64::checked_add(p0, p1)` for `Some(u64::wrapping_add(p0, p1))`) and is still
detected. `rb_07` and the `lexlean` unit tests of the renderer and runtime digests fail only on
the records bound to the compiler-semantics ID (`rust_renderer`, `rust_runtime_core`,
`rust_runtime_std` in `language/semantics-1.2.toml`), which are left to the regeneration wave.

`rb_06` found one more break of the generated text: the machine primitives `int_rem` and `int_quot` print as block expressions, and Clippy's default lints refuse a block as a `match` scrutinee (`blocks_in_conditions`) in the package that carries `int_quot`. The printer binds a scrutinee that begins with a block, `{ let value = <block>; match value { … } }`, as it binds a loop's step; with it `rb_06` passes on every committed package.

The runtime crate is checked by `validate` (`runtime::core` runs it on the decoded crate), which
refused the first decoded `nat_add`: `arithmetic: the value a fallible function returns,
construct:err, is not a fallible value`. The arithmetic rule now reads an `Err` whose error type is
`RustType.overflow` and whose one payload is the overflow value as a fallible value, the dual of
`RB-15`'s overflow literal, and nothing else the runtime builds as one. The error type, which the
first head stood in for with `unit` and printed without type arguments, is the node
`RustType.overflow` (`Type::Overflow`, the Lean constructor `RustType.overflow`, the leaf
`LkOverflow` of `RustValidate`'s `Ty`), and the printer states `Err::<u64, Overflow>(Overflow)`.
Three plants of the module are refused by `validate` in the case: an `Err` of the natural zero,
an `Err` whose error type is `nat`, and a parameter of type `str` in the core runtime
(`hidden allocation`). The case also requires `tests/rust-runtime-oracle/std.rs.txt` to be the
text `runtime::STD` holds.

The case routes its plants through `runtime::core_from`, the function `core()` is `core_from` of the
embedded module, so they are refused where `core()` refuses them; the first head validated
`decoded.core` in the case itself, and with the `validate` call removed from `core()` the case
still passed (review of `b4b1eaf4c`). With the call removed from `core_from`, `conformance_rb_16`
fails at the first plant: `an Err of a value that is not the overflow value` is accepted and printed.
A fourth module plant is a bare `Ok` of an arithmetic result (the `succeed` of the `Some` arm that every checked function shares, `someArm`, replaced by
`Expr.construct (Ctor.ok …)`, so the module is refused wherever any function uses it): `construct:ok, is not a fallible value`.

The module's `\title{Natural number addition}` stays: a title is a phrase concept of the module's
glossary (`lexlean.std.nat`), and all eighteen modules of the compiler project carry that one; a title
of the module's own (tried: `The core runtime profile`) is refused as `LLL1004`, ``The` is not a phrase
concept`, by `sp_22` when it builds the project.

The normative validator moved with the Rust one (`BS-05`): `returnsResult` and the tail rule of
`EConstruct` in `RustValidate.lex.tex` read an `Err` whose error type is the `LkOverflow` leaf and
whose one payload is `LitOverflow` as a fallible value (`isOverflowTy`, `isOverflowArgs`,
`isOverflowErr`). Before it, `validateCrate` refused `Function{params: [(p0, Nat)], result:
Fallible(Nat), body: Construct{Err(Nat, Overflow), [Lit::Overflow]}}` that `validate` accepted, and
the corpus held no crate of that shape. It holds three now, in `machine_crates()`: that crate
(accepted by both), the same with the payload `Nat 0`, and the same with the error type `nat` (refused
by both as `arithmetic: … is not a fallible value`); the counts are 442 `validateCrate` vectors (195
refused) and 305 `correspondCrate` vectors (81 refused). Planted: the tail rule of `EConstruct`
refusing every `Err` at a fallible tail again. Observed: `run_boot_validate_crate_std_14: the
rendering and the Rust oracle disagree on 1 of 126 runs: run 123 … arithmetic: the value a fallible
function returns, construct:err, is not a fallible value != ok`, the first of the three crates. The
same reading in `returnsResult`, which only the ends of a loop use, is not observable by a crate of
this shape and was not planted. `conformance_bs_05` passes every leg through the certified export
(2,127 s) and fails only at the comparison with the committed packages, the record bound to the
compiler-semantics ID.

Plants were run with `cargo test -p repo-conformance --test conformance conformance_rb_16`,
each restored with `git checkout` or the saved file.

Planted (a): one byte added to the shipped copy. Observed: `language/rust-runtime-1.2/RustRuntime.lex.tex
differs from compiler/src/RustRuntime.lex.tex`.

Planted (b): `Machine.checkedNatAdd` replaced by `Machine.checkedNatMul` in the module (part of
the case). Observed: the harness fails at `nat_add result`.

Planted (c): the arity check of the decoder removed. Observed: the extra-argument module is
accepted and the case fails at `a constructor with another number of arguments`.

Planted (d): `runtime::core()` assembling the base alone. Observed: `the embedded module is the
committed one`.

Planted (e): the differential table emptied. Observed: `the module defines {"nat_add"}, the
differential covers {}`.

### each core runtime function has its refinement theorem (SP-22)

`refines_nat_add` states, for all `x y : Nat`, `runExport 12 coreCrate "nat_add" [nat x, nat y]
= runItem core natAdd [nat x, nat y]` and is proved by `term_cases` over the primitive's outcome
with the definitions the unfolding draws on (`SP-22`, §17.16, #91 stage 4, PR 2); the other thirteen
natural and integer functions have a theorem of the same form over their primitive, the module's own data definitions
(parameters, argument reads, the two arms of a checked call) listed with the `RustSemantics`
ones. the three Boolean functions are proved by `cases` on each Boolean parameter (`SM-36`) with `reflexivity` in each of the two or four branches, because a Boolean has no document inductive for `term_cases` to split on and the two sides agree only on the values. `nat_succ`'s run adds the operand one to the item's list, and `itemOperands` stated that with
`prim append`, which the proof cannot unfold; its arm now matches the list of one operand and
states the list of two directly, keeping `prim append` for any other length, so the item's meaning
is unchanged and the shipped `RustSemantics.lean` is regenerated. The case (`cases/rust_refinement.rs`)
lists the functions with the decoder and the theorems from the module's data, requires one
theorem per function with a statement that runs the function it is named for, builds the
project of the five modules the runtime module imports, and has the pinned Lean verify it
(about 100 s: 60 for the project, 40 for the planted one). The proof reached the
conditional of `itemFallible` only through `ite_true` and `ite_false`, which the lowering of
`term_cases` therefore names (see `SM-35`).

Command and host: `cargo test -p repo-conformance --test conformance -- conformance_sp_22`, in the
`preservation` part of `bdd` (`.github/bdd-parts.json`). The theorem-list assertions and their
plants run on every host; the elaboration of the module and its plants run where
`leanprover/lean4:v4.32.1` is installed (`lean_backed("SP-22")`), and on the normative host
(Linux x86-64, §8.3) the case panics rather than skip. Cost on a 4-core container: 142 s for the
case, of which about 60 s is the project of the five modules and about 70 s the planted one; the
18 theorems add under ten seconds to the module (71 s with them, 80 s with none, on the same
machine, so the difference is inside the noise): under half a second a theorem. The width
families (92 more functions of the same form, on branch `claude/issue-91-stage4-widths-2`) put 110
theorems in one module and one crate of 110 functions: it verifies in 8 min 1 s on this host
(13 min 51 s of CPU), about 4 s a theorem, 11 theorems (one width) in 39 s over the base. An earlier
record here said that verification did not end in an hour and blamed `findFunction` unfolding
through the crate; that was wrong. That run's proofs lacked `allWidth` and `hasWidth` in their
definition lists, so `simp` could not close the goals and kept searching, and a later run was killed
by rebuilding the binary under it. The width families need `SP-22` in a part of `bdd` of their own
for the time, not for the crate's length.

Memory (the review of #248): the pinned Lean's peak resident set while `conformance_sp_22` runs the 110-theorem module, sampled every three seconds (`ps -C lean -o rss=`, the largest), is 10,389,084 KiB (10.39 GB) over a case of 442 s (7 min 22 s; 13 min 33 s of CPU) on this 16 GB host, one `lean` process at a time. The harness (cargo, rustc, the lexlean process) adds about 1 GB, so a 16 GB runner has more than 4 GB beyond it, past the two-gigabyte bar the review set for splitting the module, and the module is not split. The leg is not given a memory budget: `RP-14` (§9.4) requires a leg that does not run examples to name no profile, so a `memory_kib` on the `bdd-preservation-runtime` row is refused (`a leg that does not run examples names no profile and no part`, observed with 12582912), and the kernel's OOM kill (this host's cgroup killed a `lean` at 13.6 GB when two large elaborations ran together; a run of one is not near it) stays the failure mode. Giving a `bdd` leg a profile is a change of §9.4 and of `RP-14`, which is the maintainer's.

Plants, each restored: a theorem renamed to `refines_nat_other` (`the module defines` the eighteen names,
`its refinement theorems are for` the seventeen and `nat_other`); the theorem removed (the same message); the theorem
run on `"nat_sub"` (`refines_nat_add states the run of nat_sub`); the body changed to the checked
product (`LLV7002` with `unsolved goals`); the axiom policy changed to `[propext]` (`LLV7005`,
`violates its exact axiom policy: observed [Classical.choice, Quot.sound, propext]`).

### a list of any length is a term Lean elaborates without binders

Lean's `[…]` macro writes a literal of more than 32 elements under `let`
binders, and a certificate B witness does not unify through them (#97). Run
with the pinned Lean 4.32.1 on WSL, and without Lean on Windows.

Observed before the change: the project of
`tests/negative/certificate-heartbeat-rejected` (the `total → h1 → h0` chain
of 100 parameters), copied out and verified without its shim on `main`
`d284b165`. Command: `lexlean verify --all`. The valid certificate B is
refused.

```text
error[LLV7015]: certificate B: `LexLeanPreserve.C6963c57fd1f319f49d7646a253d9f310.R0.RustCore` was rejected in `fun0` of root `Production.Main.total`: error: Application type mismatch: The argument
  rfl
has type
  ?m.5380 = ?m.5380
```

With the change, the same command verifies it (`verified 1 module; attestation
32499d57…`, 1 min 56 s), and `conformance_sp_13` verifies the call of 33
operands to a function of 33 parameters, the copy of a 33-field record, and
the match on 33 constructors of one field (132 s).

Planted (a): `lean_list::list_by` made to write every list as a literal
(`if true || items.len() <= LITERAL_MAX`). Command: `cargo test -p
repo-conformance --all-features --test conformance -- conformance_sp_13`.
Expected: the Lean-free half finds the literal Lean would split, before any
toolchain runs.

```text
thread 'conformance_sp_13' panicked at crates/conformance/src/cases/preservation.rs:1772:25:
a call of 33 operands to a function of 33 parameters: `LexLeanPreserve.Cb589ceb26d4475920a8dc7d11fcc9b64.R0` writes a list literal of 33 elements, which Lean splits under `let` binders
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 370 filtered out
```

Removed: the plant was reverted and `conformance_sp_13` passes. A list of at
most 32 elements is written byte for byte as before, so only certificates that
hold a longer list changed. That is 13 certificates of `examples/models` and
`examples/reasoning`, whose records `cargo xtask verify-examples --write`
regenerated and a second, checking run confirmed. The others are two
fixtures whose diagnostics quote a size the writers produce: the certificate
A of `tests/negative/certificate-resource-exhausted` (63 552 bytes before, 66 000
after, still at least a quarter of `max_file_bytes`; main's figure already
carries the header of the lifted budgets), and the derivation bound of
`tests/negative/certificate-generation-limit` (5 285 358 bytes before, 4 259 854
after: the chain is shorter than the literal Lean splits under binders, and the
root still exceeds `max_file_bytes` in phase certificates, `LLS8002`).

### certificate A is kernel-term data and a defect in it is refused

Planted, first (`SP-15`, #157): the seven LOWERING mutations of
`conformance_sp_02` (Branches, Arithmetic, Overflow, Constructor, Buffer,
Recursion, Literal) at the first, a middle, and the last site of
`examples/production-coverage`, run by hand at commit `7653ed1e` with the
developer lens `dev_plant`, since removed from the tree (the repository carries
no ignored test; the record is the evidence): 20 plants. All 20 are caught,
none accepted. 14 are rejected by the kernel inside the `__rel_<n>` row of the
mutated function, and 6 are refused by the generator while it writes the
certificate, because certificate A is derived from the source and the lowering
in lockstep, so a lowering that no longer follows the source has no certificate
to give to Lean (`Planted.refused`). The first line of each class:

```text
.../Planted/P0.lean:531:0: error: native core declaration 'LexLeanPreserve.Planted.P0.__rel_0':
phase preservation: the lowering of || is not a cond
phase preservation: the lowering of `low` is not a variable
phase preservation: the lowering of `i` is not a variable
phase preservation: the lowering of a literal is not a value
phase preservation: a call of function 1 lowered to 2
phase preservation: a call of function 2 lowered to 1
```

(The kernel line is the same shape for the other 13: Branches fn 6, Arithmetic
×3, Overflow ×3, Constructor ×3, Recursion fn 21, Literal ×3, each naming
`__rel_<function>`. The refused six are: Branches fn 0 and fn 3, Buffer ×2,
Recursion fn 0 and fn 2.)

Planted, second: five plants in the data of one certificate A of
`examples/production` (`conformance_sp_15`, which prints them with `eprintln!`
as "SP-15 plants refused"): a wrong lemma (`prim_natAdd` renamed
`prim_natMul`), a changed statement (`Bool.true` renamed `Bool.false`), a
forged program literal (one literal of the program row raised by one, on a
fresh path), an added axiom (`sorryAx` as the root's proof), and a row whose
policy lies about its axioms.

```text
wrong lemma: .../R2PlantWrongLemma.lean:525:0: error: native core declaration 'LexLeanPreserve.Cf5a58e441acc8b93ded11826c51cc7c8.R2.__rel_2':
      (kernel) application type mismatch
        LexLeanPreservation.conv_prim
changed statement: .../R2PlantChangedStatement.lean:525:0: error: native core declaration 'LexLeanPreserve.Cf5a58e441acc8b93ded11826c51cc7c8.R2.__rel_1':
      (kernel) application type mismatch
        LexLeanPreservation.conv_apply
forged program: .../R2PlantForgedProgram.lean:525:0: error: native core declaration 'LexLeanPreserve.Cf5a58e441acc8b93ded11826c51cc7c8.R2.__prog':
      (kernel) application type mismatch
        1
added axiom: the forbidden constant `sorryAx`
lying policy: the certificate row `LexLeanPreserve.Cf5a58e441acc8b93ded11826c51cc7c8.R2.__rel_1` depends on ["Classical.choice", "Quot.sound", "propext"], its policy pins []
```

The added axiom is refused by the data audit before any Lean runs. The lying
policy compiles (that is not Lean's to refuse) and is refused by the audit
module.

Removed: the plants were not committed; every plant is applied to a copy in
the test. With none planted, `conformance_sp_15` passes on the committed
records (run on regenerated records: "test result: ok. 1 passed; 0 failed;
... finished in 128.47s"; the certificate hex ID above is the one before the
train's regeneration), and `conformance_sp_02` passes ("test result: ok. 1
passed; 0 failed; ... finished in 7951.32s", on `7653ed1e2`). SP-15 also holds
the source scan: it scans the sources that build certificate A
(`production/certificate.rs` and `certificate/data_a*`) for Lean-syntax string
literals and finds a planted template `format!("theorem {}0 : …")`, so the scan
is armed.

Behind "the largest scaling of each mixed shape fits": SP-02 passes, so at the
largest scaling every shape's lower bound is at or below its certificate A. The
tight one, measured by hand, is mixed shape 7 at 256: 5,879 expression nodes,
certificate A 120,894 bytes in the notation, `COSTS_A` bound about 115.6k
(5400 + 5879 × 300 / 16), 4.4% under. The other shapes' figures were not
recorded separately.

### certificate A certifies lists of recursive types and closures of any size (#208)

Three limits of the certificate-A builders, each reproduced on `origin/main`
(`724a55590`) with `lexlean verify` under the pinned Lean 4.32.1 and accepted
with the fixes. The reproducers are one-module projects with a
`production: {effects: [allocation], targets: [rust-std]}` root over
`Ty = TUnit | TOption Ty | TFn (List Ty) Ty`; they are committed as
`tests/preservation-certificates/lists`, which `conformance_sp_15` certifies
under the pinned Lean beside the `production` example and which its Lean-free
part checks for the chunking below.

| limit | reproducer | refused on `main` | accepted with the fix |
| --- | --- | --- | --- |
| (a) a record or single-constructor inductive holding `List Ty` as a root parameter | `structure S2 {ts : List Ty}`, root `probe (x : S2) : Nat = 0` | `LLV7013 … rejected in decodeAndAdd … compiler IR check failed at __enc_0. Error: unknown join point 'block_0'` | verified |
| (b) `cons` onto a list of a recursive type | root `consTy (h : Ty) (r : List Ty) = cons h r` | `LLV7013 … rejected in __rel_0 … (kernel) application type mismatch LexLeanPreservation.conv_build (LexLeanPreservation.convL_cons …` | verified |
| (c) a closure whose boundary encoders are split across 64-declaration commands | 70 functions `f_i`, ten `g_j` summing seven of them each, root `closure (t : Ty)` summing the `g_j` (81 functions; a chain of 50 does not reach the boundary, a chain of 70 does) | `LLV7013 … rejected in decodeAndAdd of root … f8: Unknown constant …R80.__items_0 … failed to compile definition … depends on '__enc_0', which is 'noncomputable'` | verified |

The causes, from the generated terms:

- (a) The encoder of a document type outside a recursive group (`S2`, compiled)
  applies `ListEnc.enc Ty __L_0`, and the bundle `__L_0` is a row with
  `compiled: false`. The code generator has no code for the bundle and reads the
  application as a closed term whose join point it cannot find. A bundle that a
  compiled row applies is now compiled (`Gen::finish`, `compile_applied_bundles`);
  a bundle only the proofs name keeps `compiled: false`, so no committed record
  moves.
- (b) `construct_cons` states the payload of the consed list as `List.map enc
  tail`. For a list nested in a recursive group the encoding is `Value.list
  (__items_k l)`, and `__items_k tail` is not `List.map enc tail` for a variable
  tail (that is an induction, and the bundle gives `items_nil` and `items_cons`
  only), so the equation `conv_build` needs does not hold by `Eq.refl`. For a
  nested list the lemma now takes `__items_k tail`, which holds by one unfolding
  of `__items_k` on a `cons`, unless the tail is a literal list (a `nil`, or a
  `cons` onto one), where `List.map` of the element encoder and the items row
  reduce to the same elements and the committed certificates state the map. After this, the same root's `denote` is refused
  with (a)'s message: the same bundle rule applies.
- (c) `group_representatives` of `backend/core.rs` grouped only the partial
  implementation records. The safe mutual group (`__enc_k`, `__items_k`; `all`
  names the group) was scheduled as independent nodes, and the runtime compiles
  a group from the members present in the command, so a command boundary
  between them compiled `__enc_0` apart from `__items_0`. A safe compiled group
  of two or more rows is now one scheduling node whose members are emitted
  contiguously, and a command boundary never falls inside a run of one group
  (the 64-declaration boundary and the node-count halving both).

No library lemma, no axiom, no `sorry`; nothing under `language/` is edited
except `lean_backend` in `semantics-1.2.toml`, which moves from `20` to `21`
because the emitted Lean text of a module with a safe mutual group differs
(order and chunking): every 1.2 compiler-semantics ID, lock, source, semantic,
and build ID moves with it, and the maintainer regenerates the records in the
identity wave. The record gates are therefore red on this branch by design.

Evidence that no existing certificate moves, with `lean_backend` held at `20`
(the bump is the only difference after it): a temporary test, not kept in the
tree, generated every certificate A of the four corpora (59 roots) and compared
its notation with the committed `expected/verify/preserve/…/R<i>.lex.tex`: 59 of 59
byte-identical. Its first run found one that was not, `roseTotal` of
`production-coverage`: the committed certificate states `List.map` of the
element encoder for a literal tail (`cons … (nil …)`), where it and the items
row reduce to the same elements, so the items payload is used only for a tail
that is not a literal list (`is_literal_list`), and the second run is the 59.
`cargo xtask check-golden` and `cargo xtask check-production-export` pass, and
`cargo xtask verify-examples` reached `production` (140 normalized records
equal) before the process was lost; it was not completed, the remaining
examples (including `production-coverage`) are not re-verified here, and the
byte comparison above is the evidence for their certificates A.

Planted:

- the pre-fix defect of (a), a bundle row of the list-project certificate of
  `probeStructure` marked `compiled: false` (`conformance_sp_15`): refused by
  Lean, the output names the join point;
- a defect in the new payload of (b), the `__items_0` row of the `consTy`
  certificate renamed to another encoder (`conformance_sp_15`): refused by the
  kernel;
- for (c), the group armed in the Lean-free part of `conformance_sp_15` and the
  unit tests `a_safe_mutual_group_is_one_command` and
  `the_group_test_is_armed_by_an_ungrouped_pair` of `backend/core.rs`: the same
  rows with `all` reduced to themselves straddle a command, so the check can
  fail; and the closure root is refused on `main`, above.

Cost: the closure root dominates `conformance_sp_15`. The 81-function closure
verifies in about 21 minutes on the machine of this record under load from other
jobs (user time 17.5 minutes); on `main` a chain of 50 functions took 16. So
`SP-15` is its own part of `bdd` (`preservation-lists`, the row
`bdd-preservation-lists`) and no other leg grows.

Found, not fixed here: a left-nested sum of 70 calls makes
`Program::to_file_bytes` panic (`recursion limit exceeded`), #215.
### a lowered program nested beyond the limit is refused, not a panic (#215, R5)

`Program::to_file_bytes` serialized the lowered program and read the text back
through the canonical JSON reader, whose limit on nesting is `serde_json`'s 128,
and `expect`ed that to succeed. A root whose lowered program nests deeper than
127 arrays and objects therefore panicked in `verify` instead of failing with a
registered diagnostic. On `main` (`39edf5ac2`), with the committed negative
fixture `lowering-depth-limit` (a root `deep` calling a chain of 509 generic
definitions `g_k<T> x = g_{k+1}<(T, Nat)> (x, 0)`, whose instances nest a pair
one level deeper per definition; its project lifts `max_total_source_bytes`,
because the eligibility report of a chain takes the square of its length):

```text
thread 'main' panicked at crates/lexlean/src/calculus/mod.rs:496:14:
program is JSON: "recursion limit exceeded at line 1 column 705999"
```

Now, from the same project:

```text
error[LLS8002]: root `Production.Main.deep`: program_json_depth exceeded in phase lowering: at most 512, the lowered program nests 515 deep
```

The reader on this path is the unbounded one that `production::lcnf` already
uses, so the program nests as deeply as its expressions do, and the depth is
bounded where the program is lowered (`production::lower::PROGRAM_JSON_DEPTH`,
512, measured on the serialized text by `json_depth`, which skips strings).
The number is a stack bound, stated and tested, not the reader's: a debug
build round-trips a chain of 400 nested expressions on a 1 MiB thread and 800 on
2 MiB; the limit, about 250 nested expressions, takes about a quarter of 2 MiB,
and the unit test `a_program_at_the_limit_round_trips_on_a_two_mebibyte_stack`
builds the deepest chain within the limit and round-trips it on a 2 MiB thread.
It over-refuses nothing that was accepted: a survey of every lowered program of
the four corpora (deepest: `models`, 50) and of the stress families (deepest:
`nested pairs 120`, 244) found none above the limit, where the reader's 127
would have refused two of the stress families if they had been serialized.

Planted: the check absent is `main`'s behavior, the panic above on the same
fixture (`EX-07` runs the fixture through the CLI and requires exit 4 and the
one prescribed code, so a panic or a pass fails it). The limit and the
round-trip test move together: a limit raised past what a 2 MiB stack hosts
fails `a_program_at_the_limit_round_trips_on_a_two_mebibyte_stack`.

Not covered: a program nested far beyond the limit can overflow the stack in
`serde_json::to_string` before the check measures it; the lowering and
certificate generation recurse over the same nesting on their own stacks.

### certificate B is kernel-term data and a defect in it is refused

Certificate B of a root is a native core module the compiler builds
(`production/certificate/data_b` over `production/kernel.rs`), so a defect is
planted in the data and must be seen where the kernel checks it, or before,
where the data is audited. `conformance_sp_16` builds every certificate B of the
four corpora as data (76 certificates, 984 rows, 6,378,738 bytes of notation),
reads each record back from its notation, checks that its rows are the ones the
text certificate declared, verifies the production example under the pinned
Lean, and then plants five defects in the data of one certificate B of that
example (the second root's, which has several functions and adds), each refused.
The outputs were recorded on Linux x86-64 with Lean 4.32.1. Command: `cargo test
-p repo-conformance --test conformance -- conformance_sp_16 --exact --nocapture`.

Planted (a), a wrong lemma: the first two rules of the correspondence the
certificate uses, in the order of the data, with the first renamed to the second
in every constant node that names it. Expected: the kernel refuses the
derivation, since the rule's premises are not the judgment's.

```text
<staging>/src/LexLeanPreserve/C3950237c975b30f5dd9298b8e09919c9/R2/RustCorePlantWrongLemma.lean:517:0: error: native core declaration 'LexLeanPreserve.C3950237c975b30f5dd9298b8e09919c9.R2.RustCore.fun1':
  (kernel) application type mismatch
    @LexLeanPreservation.Rust.Corr.applyF RustCore.program RustCore.krate RustCore.flags
```

Planted (b), a changed statement: the statement of `fun0` replaced by the statement
of `fun1`. Expected: the theorem proves `FunOK … 0` and claims `FunOK … 1`.

```text
<staging>/src/LexLeanPreserve/C3950237c975b30f5dd9298b8e09919c9/R2/RustCorePlantChangedStatement.lean:517:0: error: native core declaration 'LexLeanPreserve.C3950237c975b30f5dd9298b8e09919c9.R2.RustCore.fun0':
  (kernel) declaration type mismatch, 'LexLeanPreserve.C3950237c975b30f5dd9298b8e09919c9.R2.RustCore.fun0' has type
```

Planted (c), a forged program literal: the runtime item `Item.natAdd`, shared by
the crate literal and every derivation that names it, renamed to `Item.natSub`,
so the crate's addition is a subtraction the program does not state. Expected:
no derivation relates them, and the kernel refuses the first rule whose side
condition names the item.

```text
<staging>/src/LexLeanPreserve/C3950237c975b30f5dd9298b8e09919c9/R2/RustCorePlantForgedProgram.lean:517:0: error: native core declaration 'LexLeanPreserve.C3950237c975b30f5dd9298b8e09919c9.R2.RustCore.fun2':
  (kernel) application type mismatch
    LexLeanPreservation.Rust.Corr.primF
```

Planted (d), an added axiom: a `sorryAx` constant node as the proof of the root
theorem. Expected: the data audit refuses it before any Lean runs.

```text
the forbidden constant `sorryAx`
```

Planted (e), a lying policy: the root's policy set to pin no axiom. It compiles
(Lean does not read the policy), and the audit module, which prints the axioms
of every row, refuses it.

```text
the certificate row `LexLeanPreserve.C3950237c975b30f5dd9298b8e09919c9.R2.RustCore.root` depends on ["Classical.choice", "Quot.sound", "propext"], its policy pins []
```

Removed: the plants were not committed to any generator; they are mutations the
case applies to a copy of the data. The negative fixture `certificate-b-rejected`
(its overlay turns the constant node of `Item.natAdd` into `Item.natSub` in the
rendered data of every certificate B) fails `lexlean verify` with `LLV7015` and
nothing published, as it did when the overlay edited the text. Before
`conformance_sp_16` had an arm, the same command failed with `no preservation
case is wired for SP-16`, the red step of R3.

### no Lean source text template remains in the production compiler

`conformance_sp_17` reads the Rust sources with a lexer
(`crates/conformance/src/lean_literals.rs`): comments, plain, raw and byte
strings, character literals and lifetimes the way the Rust lexer reads them,
every item under `#[cfg(test)]` dropped, and each remaining string literal
tested for Lean syntax. It covers every source under
`crates/lexlean/src/production/` and the probe and audit module part of
`crates/lexlean/src/backend/lean.rs` (from the section "Probe and audit
modules"; the rest of the file renders the lexicon terms of the program a user
wrote, which is what the backend is for).

A literal is Lean syntax when it holds one of: `:=`, `=>`, `↦`, `→`, `∀`, `∃`,
`⟨`, `⟩`, `(.`, `«`, `@[`; a `fun` binder; a line of `#print`, `#check`,
`#eval`, `import`, `namespace`, `set_option`, `universe`, or `open` with more
after it; a parenthesized application of a capitalized name (`(Option {})`); or,
when the literal holds a `:`, a line of `theorem`, `def`, `example`,
`instance`, `structure`, `inductive`, or `abbrev` with more after it. A bare
word (`"inductive"`) is a name, and an English diagnostic carries none of the
above. The allow-list is 11 entries, each with its reason in the source: the
printouts of kernel terms that developers read (`kernel/ext.rs`, the
`LEXLEAN_ROWS` and `LEXLEAN_SHOW` printouts), the quote of one segment of a Lean
name (`certificate.rs`), and the closed commands the auxiliary module renderer
writes (`backend/lean.rs`). The case also requires every allow-list entry to
match a literal that exists, so an entry cannot outlive its reason.

Planted templates, each reported by the scan on the real source of the file
with the template appended, and nothing else reported:

```text
SP-17 planted in production/preserve.rs: line 2032 `theorem t (n : Nat) : n = {x} := by decide`
SP-17 planted in production/certificate.rs: line 3139 `theorem t (n : Nat) : n = {x} := by decide`
SP-17 planted in production/lower.rs: line 3729 `theorem t (n : Nat) : n = {x} := by decide`
SP-17 planted in backend/lean.rs: line 2714 `namespace Planted\nend Planted\n`
```

The same template under `#[cfg(test)]` is not reported, and the definition
accepts seven Lean literals and refuses five plain ones (the case lists them).

Before it ran, the scan found the templates the survey of #89 named, not only
the audit module: the `render_ty`, `render_value`, `render_prim`, `render_expr`
and `render_program` builders of the calculus as Lean text (certificate B's old
path; the program is a kernel-term row now), `lean_type_text` (the runtime
spelling of a type, now the structure `RuntimeType` that the differential
harness spells in the conformance crate), the harness's value printer and list
literals (moved to the conformance crate, which writes the Lean of its own
`#eval` inputs), and the unused `budget_options`. The staged audit module is an
`AuxiliaryModule` of typed commands (`PrintAxioms`, `PrintMarker`, `PrintName`,
`CheckName`), rendered without the `module` header: Lean refuses a `module`
file that imports a file that is not one
(`cannot import non-`module` A from `module``), and the certificates are
rendered outside the module system by `lean_text_legacy`, because the user
modules they import are not exposed to a `module` certificate. Retiring that
rendering changes every certificate's text and so every record, which waits
for the regeneration; the audit module keeps the unheaded form only because it
imports them. Its text is the text of before, except that four library
declarations whose last segment is the Lean token `at` (`Rust.RC.at`,
`RCB.at`, `RCI.at`, `RCL.at`) are now quoted as every auxiliary module quotes a
reserved segment (`RC.«at»`), the same name to Lean.

### certificate E is kernel-term data and a defect in it is refused

Certificate E of a root is a native core module the compiler builds
(`production/certificate/data.rs` over `production/kernel.rs`), so a defect is
planted in the data and must be seen where the kernel checks it, or before,
where the data is audited. `conformance_sp_14` builds every certificate E of the
four corpora as data, reads each record back from its notation, verifies the
production example under the pinned Lean, and then plants five defects in the
data of one certificate E of that example, each refused. The plants below are
what the case runs; their outputs were recorded on Linux x86-64 with Lean
4.32.1. Command: `cargo test -p repo-conformance --test conformance --
conformance_sp_14 --nocapture`.

Planted (a), a wrong lemma: the first scalar well-typedness lemma the
certificate applies (`LexLeanPreservation.Rust.wt_u32`, the proof that the
encoded first argument `left : UInt32` is well typed) renamed to `LexLeanPreservation.Rust.wt_bool` in every constant node
that names it. Expected: the kernel refuses the row, since the lemma's type is
not the argument's.

```text
<staging>/src/LexLeanPreserve/C7b8829e2cc52192864282f3d66903769/R0/Compose/RustCorePlantWrongLemma.lean:517:0: error: native core declaration 'LexLeanPreserve.C7b8829e2cc52192864282f3d66903769.R0.Compose.RustCore.root':
  (kernel) application type mismatch
    LexLeanPreservation.Rust.wt_bool left
```

Planted (b), a changed statement: the constant node of the fallibility flag
(`Bool.true`, shared by the statement and the proof) renamed to `Bool.false`.
Expected: the restated theorem claims the other result shape of the crate's
root, and `compose`'s equation on `fnFallible` no longer holds by `Eq.refl`.

```text
<staging>/src/LexLeanPreserve/C7b8829e2cc52192864282f3d66903769/R0/Compose/RustCorePlantChangedStatement.lean:517:0: error: native core declaration 'LexLeanPreserve.C7b8829e2cc52192864282f3d66903769.R0.Compose.RustCore.root':
  (kernel) application type mismatch
    LexLeanPreservation.Rust.compose (fun __e_n => RustCore.root __e_n 0)
```

Planted (c), a forged crate reference: the constant node `<module_b>.krate`
renamed to `<module_b>.flags`. Expected: the kernel refuses the row, since
certificate B's root theorem is about the crate, not the flags.

```text
<staging>/src/LexLeanPreserve/C7b8829e2cc52192864282f3d66903769/R0/Compose/RustCorePlantForgedCrate.lean:517:0: error: native core declaration 'LexLeanPreserve.C7b8829e2cc52192864282f3d66903769.R0.Compose.RustCore.root':
  (kernel) application type mismatch
    LexLeanPreservation.Rust.RCI RustCore.flags
```

Planted (d), an added axiom: a constant node `sorryAx` appended to the DAG and
made the `root` row's proof. Expected: the data audit refuses the module
before any Lean runs, naming the forbidden constant.

```text
the forbidden constant `sorryAx`
```

Planted (e), a lying policy: the `root` row's policy changed from the three
certificate axioms to `exact{axioms: []}`. Expected: Lean accepts the module
(the proof is unchanged), and the audit module, which prints the axioms of
every row of a data certificate, refuses the row whose policy does not state
what Lean prints.

```text
the certificate row `LexLeanPreserve.C7b8829e2cc52192864282f3d66903769.R0.Compose.RustCore.root` depends on ["Classical.choice", "Quot.sound", "propext"], its policy pins []
```

Removed: the plants are mutations of a clone of the generated data inside the
case; nothing is restored. The case passes on the unplanted data, and the
committed records of the four corpora are its notation, read back to the data
byte for byte.

### certificate generation is bounded

`lexlean verify` of a project whose root instantiates the generic chain
`g_k<T> = g_{k+1}<(T, T)>` (§17.17 *Limits*) before the limits existed used
gigabytes and failed with a rejected certificate A (`LLV7013`, a heartbeat
timeout) at 17 deep. Now the lowering counts the types, expressions, shapes,
literals, and pairs of arms the certificates repeat and refuses, before the
toolchain is touched, only a root for which a lower bound on one of its
certificates is certain to exceed the limit (a bound for each of A, B, and E,
from costs per unit that SPEC.md §17.17 *Limits* tabulates; the greatest of
the three is compared with the limit), and every certificate is generated under
the limit:

```text
chain 13 deep: verified 1 module; attestation 43e4a0e1bf30e7be8d987013ff62ada3f5a7e4b2b09f2929f53aa9810887d849
chain 14 deep: error[LLS8002]: max_file_bytes exceeded in phase certificates: a part of the certificate is 4601936 bytes, beyond the 4194304 bytes of max_file_bytes
chain 15 deep: error[LLS8002]: root `Production.Main.chain`: max_file_bytes exceeded in phase lowering: configured 4194304, its certificate A is at least 4457541 bytes (262189 program nodes)
chain 16 deep (negative fixture lowering-size-limit): error[LLS8002]: root `Production.Main.chain`: max_file_bytes exceeded in phase lowering: configured 4194304, its certificate A is at least 8913991 bytes (524336 program nodes)
```

Superseded: the figures above are the text certificates'. With certificates A
and B as kernel data (#157, #163) a repeated type is one node, the chain
verifies 16 deep, and the fixture is a 400-constructor match; see "The
size-limit fixture can fail when the certificates stop costing what it
assumes" below.

(The chain verified 14 deep before certificates wrote every reference to a
global from the root: `_root_.` adds about 10% to certificate A, which at 14
deep was 4.2 MB, and the generation, which sees the whole, now stops it.)

The first estimate (400 bytes a node) refused the chain from 11 deep though
its certificates were 0.5 MB, the second (an upper-bound calibration, 70 bytes
a type node) from 13 although certificate A of 13 deep is 2.1 MB and the
chain verifies: both refused programs whose certificates fit. A bound that
only refuses what cannot fit refuses nothing that does. The third, one bound
on the sum of what the certificates repeat, was calibrated on programs that
grow one dimension at a time and was above the largest certificate of programs
that grow two: `S` with 100 `nat` fields and a string of 5000 characters (bound
17 668, A 12 225, B 17 456, E 9 781), and a match on 200 constructors
returned in a pair with a string of 330 000 characters (bound 1 447 939,
largest certificate 1 135 681), so that 350 constructors with 940 000
characters, whose certificates are at 80% of the limit, were refused. The
bound is now one for each certificate, the greatest is compared with the limit,
and `conformance_sp_02` certifies under the default limits a match on 420
constructors (certificate A of 4.0 MB), a string literal of 450 000 characters
(certificate B of 1.4 MB), and, for each of nine mixed shapes, the largest
scaling whose certificates are all under the limit, found by search. The
string literal verifies under the pinned Lean:

```text
string literal of 450000 characters: verified 1 module; attestation b8722f14b820547f7dbdfa61293f4847d18a50f4756db154abfe3b445f4dd637
```

Certificate B cannot be bounded from the counts: the
rendering of a field read prints the whole pattern of its record, so B grows
with the square of the arity, and with the depth of a nesting. Two negative
fixtures reach it before the toolchain:

```text
certificate-size-limit (a record of 800 fields copied field by field): error[LLS8002]: root `Production.Main.copy`: max_file_bytes exceeded in phase lowering: configured 4194304, its field reads print 640000 record entries, which take its certificate B at least 10240000 bytes
certificate-generation-limit (the sum of the 500 fields of a record): error[LLS8002]: root `Production.Main.total`: max_file_bytes exceeded in phase certificates: certificate B of its rust-core rendering: the derivation of a function is at least 5285359 bytes, beyond the 4194304 bytes of max_file_bytes
```

At 800 fields the generator used to build a 13 MB certificate B from 2400
program nodes, 3 times `max_file_bytes`, before the check after generation
refused it; the estimate gave 0.97 MB.

A pinned Lean that dies, or runs out of memory, while checking a certificate
is `LLS8002`; one that exhausts a heartbeat or recursion budget is `LLS8002`
only when the module is at least a quarter of `max_file_bytes`, and is
Lean's verdict on the certificate otherwise. The two fixtures are one project
(a function of 100 parameters called by the root, with a lake overlay that
gives certificate A a heartbeat budget of one): with the default limit the
module is 56 680 bytes and the outcome is a rejected certificate; with
`max_file_bytes` at 131 072, above its largest certificate (56 680 bytes, so
that generation passes) and below four times its module (226 720), the module
is more than a quarter of it and the outcome is a limit; `conformance_sp_02`
asserts the two inequalities, so a change of a certificate reports that the
fixture no longer lies between them, not a changed hash.

```text
certificate-heartbeat-rejected: error[LLV7013]: certificate A: `LexLeanPreserve.C8fe….R0` was rejected in `__prog` of root `Production.Main.total`: error: (deterministic) timeout at `isDefEq`, maximum number of heartbeats (1) has been reached
certificate-resource-exhausted: error[LLS8002]: the pinned Lean exhausted a resource checking `LexLeanPreserve.C8fe….R0` (maximum number of heartbeats): the generated module is 56680 bytes, at least a quarter of max_file_bytes 131072, and is beyond what the machine checks
```

Rejections of certificates A, B, and E carry the declaration, the root, and
the bounded first error, as the fixtures' expected output records:

```text
error[LLV7015]: certificate B: `LexLeanPreserve.C….R2.RustCore` was rejected in `fun2` of root `Production.Main.quadruple`: error: Application type mismatch: The argument
  rfl
has type
  ?m.157 = ?m.157
but is expected to have type
  RustSemantics.itemFallible RustSyntax.Item.natSub = true
```

### a wrong certificate is not reported as a limit

At `eea3db0` any heartbeat or recursion message ended in `LLS8002`. The
overflow plant on `Coverage.Prims.fixedMiddle` (a lowering defect: the
certificate is generated from the source and the program is mutated after)
makes Lean give up on its heartbeats, and `lexlean verify` would have blamed
the project's types. Planted: the quarter-of-the-limit test of
`resource_death` removed (`HEAVY_FRACTION` made so large that every module is
heavy, which is the old behaviour). Command: `cargo test -p repo-conformance
--test conformance -- conformance_sp_02`. Expected: the suite, which asks the
product's classifier about every rejection it plants in A and E, refuses.

```text
thread 'conformance_sp_02' (25505) panicked at crates/conformance/src/cases/preservation.rs:76:5:
Coverage.Prims.fixedMiddle Overflow: a rejected certificate of 4776 bytes is reported as an exhausted resource:
/tmp/lexlean-plant-mx1DoX/src/LexLeanPreserve/Planted/P7.lean:29:0: error: (deterministic) timeout at `whnf`, maximum number of heartbeats (200000) has been reached
```

Removed: the constant was restored. The unit test of `lexlean::verify` for a
heartbeat verdict on a small and on a heavy module fails under each plant of
the constant, a fraction of 0 (nothing is heavy) and a huge one (everything
is):

```text
thread 'verify::resource_tests::a_heartbeat_verdict_is_a_limit_only_for_a_heavy_module' (25315) panicked at crates/lexlean/src/verify/mod.rs:2613:73:
a limit
thread 'verify::resource_tests::a_heartbeat_verdict_is_a_limit_only_for_a_heavy_module' (25377) panicked at crates/lexlean/src/verify/mod.rs:2610:9:
assertion failed: resource_death("M.R0", 1, HEARTBEATS, 5_000, &limits()).is_none()
```

### the lower bound and the floor are checked against what is generated

Planted: the cost of a type node of certificate A in `lower.rs` raised from
272 to 2000 sixteenths of a byte. Command: `cargo test -p repo-conformance
--test conformance -- conformance_sp_02`. Expected: the bound passes the size
of a certificate of the corpora or of the stress families, so that it would
refuse a program that fits.

```text
production-coverage: Coverage.Colls.mapOps: the lower bounds Bounds { a: 51284, b: 13958, e: 839 } are above certificate A (29765), B (59984), or E (2963)
```

Planted: the cost of a pair of arms of certificate A raised from 180 to 500.
Expected: the check that valid programs whose certificates fit are not refused
(a match on 240 constructors, a string of 450 000 characters, at the default
limits; certificate A is recorded in the notation, which is three times the Lean text of a wide match, so the match is 240 where it was 420) refuses the match.

Not re-run since certificate A became kernel data (#157): the cost of a pair of
arms is now 1018 sixteenths, so the figures of this plant and its quoted
`LLS8002` line are the text certificate's. The current evidence that the bound
refuses, and does not over-refuse, is the fixture `lowering-size-limit` (a
match on 400 constructors, refused in lowering) and `conformance_sp_02`'s pass
(the match on 240 constructors fits and is not refused).

```text
Production.Main.r: lowering failed: Diagnostic { code: DiagnosticCode("LLS8002"), message: "root `Production.Main.r`: max_file_bytes exceeded in phase lowering: configured 4194304, its certificate A is at least 5524790 bytes (1265 program nodes)", primary: None, labels: [], notes: [], help: [], causes: [], detail: None }
```

Planted: the reviewer's defect, the bounds at twice their cost (the greatest
for which each stays below its certificate) and compared with the limit as
a sum. Expected: the same check refuses a program whose certificates fit.

```text
Production.Main.r: lowering failed: Diagnostic { code: DiagnosticCode("LLS8002"), message: "root `Production.Main.r`: max_file_bytes exceeded in phase lowering: configured 4194304, its certificate the sum of A, B, and E is at least 7111500 bytes (1142 program nodes)", primary: None, labels: [], notes: [], help: [], causes: [], detail: None }
```

(At half the cost the sum would still be below the largest certificate of
those programs, which is why the check plants the cost too: the margin and the
aggregator are separate protections.)

Planted: the floor under certificate B raised from 16 to 100 bytes a record
entry. Expected: a certificate B smaller than its floor.

```text
record copy 25 Production.Main.r (rust-core): the floor 62500 is above certificate B, 24733 bytes
```

Planted: the check of a derivation against the limit removed from
`within_budget` in `rust_cert.rs`. The final check of the finished
certificate still refuses it, at its whole size, and the suite, which asks
that certificate B stop while it is derived, refuses.

```text
record copy 100: certificate B was refused when finished, not while derived: limit: the certificate is 245088 bytes, beyond the 122544 bytes of max_file_bytes
```

Removed: all were restored. The size a rule records is the length of
the text it prints (a unit test of `rust_cert`), and `conformance_sp_02`
requires that each bound is never above its certificate for any root of
the corpora, of the stress families, or of 24 random combinations of five
dimensions (none of which the costs were fitted to), that the largest scaling
of each mixed shape that fits is not refused, that the record copies have a B
far beyond A and E, and that the floor is never above any B.

### the unreached-overflow list is checked both ways

Planted: the exemption of `Coverage.Main.shapes` kept in `UNREACHED_OVERFLOW`
although a sampled input of that root overflows. Command: `cargo test -p
repo-conformance --test conformance -- conformance_sp_03`. Expected: a root
listed as unreached that is reached is refused, as is a root absent from the
list that never overflows.

```text
Coverage.Main.shapes overflows on a sampled input but is listed as unreached (exempt: true)
```

Removed: the list was restored.

### the interpreter, Lean, the machine, and rustc agree where arithmetic overflows

Planted: the skip of fallible operations removed from the check of the
declared machine's infallible items. Command: `cargo test -p repo-conformance
--test conformance -- conformance_sp_07`. Expected: an infallible item
overflows on an argument it was never meant to receive, which the check names.

```text
rust-core: the infallible item op_nat_add overflows on [Nat 1, Nat 18446744073709551615]
```

Removed: the skip was restored; the machine's `abort` allowance is pinned to
the fallible items only.

### certificate E is stated in full and cannot be vacuous

Planted: `∧ False` added to the `hrep` hypothesis of both statements of
certificate E in `crates/lexlean/src/production/certificate.rs` (with and
without an entry). The proof never uses `hrep`, so Lean accepts the module,
and at `eea3db0` `conformance_sp_09` passed. Command: `cargo test -p
repo-conformance --test conformance -- conformance_sp_09`. Expected: the suite
asserts the whole binder list, the one hypothesis, and the conclusion by
equality, and refuses.

```text
production: `LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R0.Compose.RustCore`: hypothesis (hrep : LexLeanPreservation.Rust.RepresentableL [(LexLeanTarget.TargetSyntax.Value.u32 left), (LexLeanTarget.TargetSyntax.Value.u32 right)] ∧ False)
```

Removed: the generator was restored. Independently of that equality, Lean
applies every certificate E to arguments of the differential, one that breaks
an invariant and ones at `u64::MAX` and the bounds of `i64`, proving the
hypothesis by unfolding it, and `conformance_sp_09` plants the same
`RepresentableL .. ∧ False` in every root's E and requires Lean to refuse the
planted module at a witness, not at `root`. An argument that is not
representable cannot be witnessed, as the check on a hand-written module
shows (`root 18446744073709551616 (by simp [RepresentableL, Representable])`
leaves `⊢ False`). `conformance_sp_09` also requires, as before, that each of
the other two compositions (the other result shape, another function) is
refused at `root`.

### the mutation checks name the declaration

Planted: the declaration expected for a mutation of an entry in
`expected_declarations` (`crates/conformance/src/preservation.rs`) renamed
from `entry` to `entry_wrong`. Command: `cargo test -p repo-conformance
--test conformance -- conformance_sp_10`. Expected: Lean rejects the mutated
entry in `entry`, the check compares that name with the expected one and
refuses. (`conformance_sp_02` passes under the same plant: it plants the
lowered-program mutations of the closure, not the entry.)

```text
Coverage.Boundary.groveRoot Validation in function 18 (Entry { validators: [Some(2), Some(13), Some(14), Some(15)] }): the error lies in `entry`, not in ["entry_wrong"]:
Planted/P0.lean:381:1236: error: Application type mismatch: The argument
  __vrel_2 g
```

Removed: the name was restored. The same check holds for B and E (each
rejected rendering must fail inside the declaration of the function mutated,
or of its callers for a member of a template instance) and for the first,
middle, and last site of every kind.

### the published program and crate are bound by the record

Planted: the digest of a rendering's crate in `preservation.json` computed
over the text without its trailing newline
(`crates/lexlean/src/production/preserve.rs`). Command: `cargo test -p
repo-conformance --test conformance -- conformance_sp_05`. Expected: the
digest the record carries differs from the digest of the file published
under `preserve/crate`.

```text
LexLeanPreserve.C2b83c7159500bd653f3a83abd73760f3.R0: the record binds the published crate in rust-core
  left: Some("62fd4c69178f72589d4336ed4f5695f4515fd7b322669e892acddf48ba5b1dcb")
 right: Some("a9ae0afab91c841fada5f7d20ccfe8a41dfc2ca90b5b46dab280d5cd17d1385f")
```

Removed: the digest was restored; the program digest is bound the same way.

### the record names the function each crate is invoked through

Planted: the entry of the root `Coverage.Boundary.groveRoot` in the committed
record `examples/production-coverage/expected/verify/preserve/preservation.json`
changed from function 18 to 0 (and its symbol from `f18` to `f0`). Command:
`cargo test -p repo-conformance --test conformance -- conformance_sp_05`.
Expected: the record disagrees with certificate E, which states function 18.

```text
thread 'conformance_sp_05' (25243) panicked at crates/conformance/src/cases/preservation.rs:1569:25:
assertion `left == right` failed: production-coverage R0 rust-std: E states the function the record names
  left: {18}
 right: {0}
```

Removed: the record was restored. `conformance_sp_11` reads the entry of each
root and target from the same committed records and asserts it equals the
lowering's before it builds the package that exports it.

### the register, the vocabulary, and the exempt roots are held to the text

Planted: a word added to `SP-02`'s statement in `model/ids.toml` only; a
character changed in `SPEC.md`'s quotation of `RCI`; a root removed from the
specification's list of roots whose overflow no input reaches. Commands:
`cargo xtask validate-spec-links` for the first two, `conformance_sp_03` for
the third.

```text
gate failed: RP-07: `SP-02`'s statement differs between the table and the register:
gate failed: §17.17 (SP-04): SPEC.md quotes a declaration of `LexLeanPreservation/RustBase.lean` that the file does not state byte for byte: `/-- Rust convergence: some fuel gives the outcome, which is never`
thread 'conformance_sp_03' (16266) panicked at crates/conformance/src/cases/preservation.rs:1387:13:
assertion `left == right` failed: SPEC.md lists exactly the roots the suite exempts
```

Removed: all three were restored. The specification states that of 49
certified roots 34 declare an overflow, 15 reach it, and 19 rest on the
kernel proof alone for the overflow arm, and names the 19; the test computes
the counts and compares the list.

### a validated parameter that cannot be broken fails the sampling

Planted: `invalidate` in `crates/conformance/src/differential.rs` made to find
nothing. At `eea3db0` the parameter was left out of the differential without a
word. Command: `conformance_sp_03`. Expected: the sampling fails.

```text
Coverage.Boundary.groveRoot: no sampled `g` could be made to break its invariant in 64 tries
```

Removed: the function was restored.

### no name of the source can capture a name the generator binds, or fail the audit

At `96088be` a root `r(hrep: nat)` was rejected as a certificate E
(`LLV7016`: the parameter was captured by the hypothesis), as were `ro`, and,
for a root with a set parameter (a boundary entry), `h`, `hr`, `hc`; and a
parameter named `kernel`, `prefix`, `notation`, `infix`, `elab`, `macro`,
`syntax`, or `extern` failed the token audit of its certificate with
`LLI9001`, an internal error. Every name the composition binds now begins
with two underscores (`__e_hrep`, `__e_ro`, `__e_hr`, `__e_hc`, `__e_h`,
`__e_n`), which no semantic name does; the audit reads a quoted name as data,
and the generator quotes a name spelled like a forbidden token. The suite's
`names` project has five roots: parameters named `hrep ro h hr hc n f x P`,
a root with set parameters named `h hr` and others named `hc ro hrep`, the
eight words as parameters, and a structure with fields `macro` and `syntax`
and a local `elab`; each certificate E is asserted in full, every binder of
it is checked, the staged workspace passes the audit, and the project
verifies under the pinned Lean.

Planted: the proof lambda of E named `h` as before. Command: `cargo test -p
repo-conformance --test conformance -- conformance_sp_09`. Expected: the
binder check refuses.

```text
thread 'conformance_sp_09' (5944) panicked at crates/conformance/src/cases/preservation.rs:955:9:
names: `LexLeanPreserve.Cf98777d97dc3c4f7e91e905af21e1e07.R1.Compose.RustStd` binds `h`, which a parameter of that name would capture
```

Planted: the audit's reading of a quoted name removed (`code_only` no longer
skips `«…»`). Expected: the audit refuses the user's `kernel` as a forbidden
token, as before.

```text
the workspace stages: Diagnostic { code: DiagnosticCode("LLI9001"), message: "phase preservation: the certificate `LexLeanPreserve.Cf98777d97dc3c4f7e91e905af21e1e07.R2`: the forbidden token `kernel`", primary: None, labels: [], notes: [], help: [], causes: [], detail: None }
```

Removed: both were restored. The unit test of `production::preserve` shows
the other direction: the audit still refuses a bare `prefix` binder, `macro`
as a command, `implemented_by`, `sorry`, and `Lean.ofReduceBool`, and takes
the same words quoted.

### a match of thousands of arms cannot overflow the stack

`Aligner::m` recursed once per arm, and a derivation nested as deeply as the
match, written and dropped recursively: with `max_file_bytes = 33554432` a
9000-arm match made `lexlean verify` die with `fatal runtime error: stack
overflow` (the debug build at 2000). The arms are now derived by a loop, and
a derivation is written and dropped with a stack of its own (a unit test
writes and drops one 300 000 levels deep on a 256 KiB stack). `conformance_sp_08`
aligns a match of 9000 arms, and then under a limit of 1 MiB, on the 2 MiB
stack of a thread. Planted: the previous `rust_cert.rs`.

```text
thread '<unknown>' (28415) has overflowed its stack
fatal runtime error: stack overflow, aborting
```

Removed: the file was restored.

### a name of the source cannot shadow a word a certificate writes

At `6992f33` a root `r(true: nat)` was rejected as a certificate A
(`LLV7013`, in `__fits_0`: the bare `true` of `(true && …)` was the
parameter). Real-Lean verifies of one root with a set parameter (a boundary
entry) and one named parameter showed the same capture for six more names,
which the first review's list had not tried:

```text
true: error[LLV7013]: certificate A: `…R0` was rejected in `__fits_0` of root `Production.Main.total`: error: Application type mismatch: The argument true has type Nat but is expected to have type Bool
cond: … was rejected in `denote` … Function expected at cond but this term has type Nat
denote, accepts: … was rejected in `entry_accepts` … Function expected at denote / accepts
entryFits, entryValue: … was rejected in `denoteEntry` … Function expected at entryFits / entryValue
absurd: … was rejected in `entry_refuses` … Function expected at absurd
if_true: … was rejected in `entry_accepts` … Variable `if_true` is not a proposition or let-declaration
```

The certificates now write `Bool.true` and `Bool.false` for the Booleans they
mention where a parameter is in scope, `_root_.cond`, `_root_.absurd`,
`_root_.trivial`, `_root_.if_true`, and `_root_.True` for the globals, and the
definitions they state (`denote`, `accepts`, `entryFits`, `entryValue`,
`denoteEntry`) by their full names, and their own lambdas use names beginning
with two underscores. `conformance_sp_09` checks the whole text of every
certificate A and E of the three corpora: every bare word that is not Lean
syntax or a tactic, a backend type name (which linking refuses as a binder), a
named argument, a declared name, a pattern head, or one of the source's own
names fails. The suite's `names` project has roots whose parameters are named
`cond denote accepts entry entryFits entryValue denoteEntry root fits absurd
if_true trivial native_decide` (also behind a set parameter, for the entry),
besides the names of the earlier rounds; it is asserted in full and verifies
under the pinned Lean. (`true` and `false` were parameters of it until the
backend's own module was found to capture the Boolean literals of a function
with a parameter of that name; linking now refuses them, and the certificates
still write `Bool.true`, as the reasoning above holds for them.)

Planted: `true` written bare again. Expected: the check of bare words refuses.

```text
thread 'conformance_sp_09' (22628) panicked at crates/conformance/src/cases/preservation.rs:689:5:
LexLeanPreserve.Cb181ebce5bf85940bfaf72afe84d15a7.R0: bare words a parameter could shadow: {"true (in ` : UInt32) : Bool :=   ((true && (true && tr`)", "true (in ` :=   ((true && (true && true)) && true)  th`)", "true (in `) : Bool :=   ((true && (true && true)) && t`)", "true (in `ue && (true && tr
```

Removed: the constant was restored.

### a quoted name is the name it is

The token audit read `«…»` as data at `6992f33`, which let `Lean.«ofReduceBool»`,
`«sorryAx» False true`, `@[«implemented_by» g]`, `@[«extern» "c"]`,
`«Lean».«ofReduceNat»`, `@[«macro» foo]`, and an unclosed `«x` before
`:= sorry` through; the pinned Lean elaborates each as the thing itself. The
audit now keeps the content of a quoted name: forbidden constants are refused
quoted or not, a quoted name inside an attribute list is read plain, and a
quoted name outside one is exempt only from the keyword spellings (a quoted
`kernel` or `prefix` is an identifier). An unclosed quotation, string, or
comment, an empty quotation, and one that is not made of letters, digits, `_`,
`.`, and `'` are refused. The unit test of `production::preserve` holds the
reviewer's seven probes, with the other attribute forms, as refusals, and the
positive cases (`«kernel»`, `«sorry»` as a name) as acceptances.

Planted: the quoted content skipped again. Command: `cargo test -p lexlean
--lib production::preserve`. Expected: the first probe passes the audit.

```text
test production::preserve::tests::a_quoted_forbidden_name_is_still_forbidden ... FAILED

```

Removed: the audit was restored.

### a name spelled like a word the generated-Lean audit forbids

A parameter `native_decide` ended in `error[LLI9001]: phase verify:
`Production.Main`: forbidden token `native_decide` in generated Lean`, an
internal error for a valid program. The backend now quotes a segment spelled
like a word that audit forbids (`native_decide`, `admit`, `IO`), as it quotes
a reserved token, and the audit reads a quoted segment as a different token.
The `names` project has a parameter of that name and verifies.

Planted: the quoting removed from the backend's `identifier`. Command:
`lexlean verify` on a project with a parameter `native_decide`. Expected: the
old internal error.

```text
error[LLI9001]: phase verify: `Production.Main`: forbidden token `native_decide` in generated Lean
```

Removed: the quoting was restored.

### the roots of a project are bounded together

The call path of a closure member is the work of this section. A member of the
eligibility report (§17.13, a build artifact) records its shortest call path
from the root, as a list of strings, so the closure of a chain of `D` calls
holds `D^2 / 2` steps and the closures of `N` roots that call one another
`N^3 / 6`. Measured with the previous code (`lexlean check`; the roots are
chained, each calling the one before):

```text
chain of 3000 functions: 638 MB (7000: about 3 GB, the 2.9 GB an earlier text recorded)
chain of 250 roots: 214 MB, 1.9 s; 500: 1.3 GB, 15 s; 1000: 9.5 GB, 90 s; 2000: exhausted the host
```

Independent functions are linear, as the first reviewer of the earlier text
found: 1500, 3000, 7000, and 14000 trivial functions take 56, 101, 223, and
435 MB. The certificates of the roots repeat their closures too, and `verify`
held all of them: the texts of `N` chained roots grow with `N` squared, each
under `max_file_bytes` (in-process, 25 roots take 1.3 MB, 50 4.8 MB, 100
18.4 MB of certificates, crates excluded). A path is now shared with the paths
that extend it, and the report counts toward `max_total_source_bytes`, once more
beyond the sources (the rule for artifact declarations): the analysis totals the
bytes the report takes root by root and stops a closure whose members alone pass
the limit; the certificates, crates, and programs of all the roots count toward
`max_total_source_bytes` as they are generated. (A first version bounded the
report by `max_file_bytes` and refused the generic chain 14 deep, whose
report is 4 266 130 bytes and which verifies; the report is an artifact, not a
file the compiler reads.) Now (`lexlean check` and `build`):

```text
chain of 250 roots (default limits): error[LLS8002]: phase production: the production-eligibility report of module `Main` is beyond the 67108864 bytes of max_total_source_bytes at root `Production.Main.r243`: a member of a closure records its shortest call path, so a chain of calls takes the square of its length and roots that call one another the cube of their number
accepted and refused: a chain of 243 roots (check 4.5 s, 75 MB; build 14 s, 675 MB, a report of 66 508 069 bytes) and 244; a chain of 2359 functions and 2360
```

The negative fixture `eligibility-report-limit` is a chain of 100 roots under
a `max_total_source_bytes` of 4 194 304 (`check`), and `certificates-total-limit`
a chain of 40 roots under one of 2 000 000 (`verify`), which passes the report
and is refused by the count of generated bytes:

```text
error[LLS8002]: max_total_source_bytes exceeded in phase certificates: configured 2000000, the programs, crates, and certificates generated so far are 2010779 bytes once root `Production.Main.r25` has generated its certificates B and E and its crates
```

Planted: the report's limit removed (`ReportBudget::exceeded` never reached:
the stop of a closure and the sum over roots both disabled). Command: `lexlean
check` in the fixture `eligibility-report-limit`. Expected: the project is
accepted, as it was (its report is 5.5 MB).

```text
checked 1 module (source ec9e53086cd452500ccaf1b1d524bd2619b6e6e58198e4b2925a4cf1ee97bf69, semantic cce26a8cdd58e11dbb290260bdd03259cc463a76359f0b81cdbcb5ce146129e5)
exit 0
```

Planted: the count removed from `GeneratedBytes::charge`. Command: `lexlean
verify` in the fixture `certificates-total-limit`. Expected: the project is not
refused; Lean is started on a project of 40 roots.

```text
(no output after 200 seconds; the process was stopped by the timeout, exit 124)
```

Removed: both were restored.

### a parameter named like a namespace of the library

A root with a parameter named `LexLeanPreservation`, `LexLeanTarget`, or
`LexLeanPreserve` failed certificate A or E: Lean reads the first component
of a dotted name as a local before it reads a namespace, so every
`LexLeanPreservation.conv_var` of the certificate was a field of the
parameter. Every reference a certificate makes to a global is now written from
the root (`_root_.LexLeanPreservation.conv_var`), by a pass over the text
that reads it with the lexer of the token audit (`from_the_root`), and
`conformance_sp_09` has two oracles that do not depend on it. Its gate reads
the first component of every longer name, the parameter's name included (the
earlier gate skipped any name beginning with a name of the source, which is
what hid the three); and the `names` project has a parameter for each
namespace the certificates of the three corpora begin a name with, which the
suite reads off those certificates (`LexLeanPreservation`, `LexLeanTarget`,
`LexLeanPreserve`, `Corr`, `RustSyntax`, `TargetSyntax`, ...), for the
namespaces of the shipped libraries and the toolchain besides (`Compose`,
`Rust`, `RustStd`, `RustCore`, `LexLeanAudit`, `LexLeanCore`, `LexLeanModels`,
`LexLeanError`, `IO`, `Init`, `Lean`, `Lake`, `Std`; `LexLeanRuntime` and
`LexLeanCollections` are names the backend writes bare and linking refuses), and
verifies under the pinned Lean. A project of three roots, one parameter named
for each of the first three, verifies through `lexlean verify`.

Planted: the pass removed (`from_the_root` returns its text). Command: `cargo
test -p repo-conformance --test conformance -- conformance_sp_09`. Expected:
the gate names the unrooted references.

```text
LexLeanPreserve.C26947099fef0ea02e807c837bb4923e0.R0: names a parameter could shadow: {"LexLeanPreservation.FunRel (line 21)", "LexLeanPreservation.Obs (line 24)", "LexLeanPreservation.Obs.overflow (line 25)", "LexLeanPreservation.Obs.value (line 25)", "LexLeanPreservation.Rel (line 21)", ...
```

The same plant through `lexlean verify` of the project of three roots is
refused by Lean itself, which is the oracle that does not read the text:

```text
error[LLV7013]: certificate A: `LexLeanPreserve.C5fb782e9353b19232660321887217eca.R0` was rejected in `__rel_0` of root `Production.Main.rPreservation`: error(lean.invalidField): Invalid field `FunRel`: The environment does not contain `Nat.FunRel`, so it is not possible to project the field `FunRel` from an expression
  LexLeanPreservation
of type `Nat`
```

Planted: `LexLeanPreserve` removed from the roots the pass writes (the root in
which a certificate E names its A and B). Expected: the gate refuses E.

```text
LexLeanPreserve.C26947099fef0ea02e807c837bb4923e0.R0.Compose.RustCore: names a parameter could shadow: {"LexLeanPreserve.C26947099fef0ea02e807c837bb4923e0.R0.RustCore.krate (line 10)", "LexLeanPreserve.C26947099fef0ea02e807c837bb4923e0.R0.RustCore.root (line 11)", "LexLeanPreserve.C26947099fef0ea02e807c837bb4923e0.R0.denote (line 10)", "LexLeanPreserve.C26947099fef0ea02e807c837bb4923e0.R0.root (line 11)"}
```

Removed: both were restored.

### a match of many arms verifies through `lexlean verify`

SPEC.md and this file said that a match on 440 constructors verifies. It was
certified in-process and never checked by Lean: through `lexlean verify` a
match of about 60 arms or more ended in `error[LLV7011]: named-root
extraction: the extraction record is malformed: recursion limit exceeded`
(Lean's code for a match nests two levels of JSON for each arm, and the reader
stops at 128 levels), and past that three more things stood in the way. The
compiler input holding the record was serialized and re-read with the same
limit (a panic); the pinned Lean's default `synthInstance.maxHeartbeats`
(20 000) is exhausted by a certificate B of a match on 160 arms, and
`maxHeartbeats` (200 000) by one on 200, each ending as `Application type
mismatch` in a valid certificate; and Lean's time for certificate B grows
faster than the fourth power of the arms. The record is read without the
limit on a stack of its own, the compiler input likewise, and the certificates
of a program whose widest match has more than 100 arms lift both budgets
(SPEC.md §17.17 *Limits*). Measured, with the pinned Lean 4.32.1, the
certificate B of a match on 100 constructors took 11 seconds, 130 took 17, 160
26, 200 42, 300 201, and 440 more than 15 minutes; certificate A of 200
constructors (949 KB) took 59 seconds. Through `lexlean verify`, a match on
180 constructors verifies (`conformance_sp_02` runs it), and so does one on
300 (about 13 minutes with the library compiled first), while one on 400 ends
in the registered limit:

```text
300 constructors: verified 1 module; attestation e4e97a92752af7d2dca23dcec4c4998cd8751f9f070e8d7323f568fa36585cb0
400 constructors: error[LLS8002]: child_timeout_ms exceeded by `lean` in phase LexLeanPreserve.C439e5df7108688ab7813661fe3502b62.R0: configured 300000, observed 300006 ms
```

Planted: the recursion limit of the extraction record not disabled
(`reader.disable_recursion_limit()` removed in `lcnf.rs`). Command: `lexlean
verify` of a match on 100 constructors. Expected: the old error.

```text
error[LLV7011]: named-root extraction: the extraction record is malformed: recursion limit exceeded at line 1 column 83365
```

Planted: the budgets not lifted (`budget_options` returns nothing). Command: the
same on 180 constructors. Expected: a valid certificate is rejected.

```text
error[LLV7013]: certificate A: `LexLeanPreserve.Cfb71e290f81fe25a5c5414c91c004e69.R0` was rejected in `__rel_0` of root `Production.Main.r`: error: (deterministic) timeout at `whnf`, maximum number of heartbeats (200000) has been reached
```

(With the budgets scaled by a formula instead, 300 constructors ended in
`error[LLV7015]: certificate B ... was rejected in `fun0` ...: Application
type mismatch`: the cost of B grows faster than any formula that could be
stated, so the budgets are lifted and the wall clock is the limit.) The quick
check of `conformance_sp_02`, that the certificates of 101 arms carry the two
options and those of 100 do not, refuses the plant before Lean starts:

```text
assertion `left == right` failed: 101 arms: `LexLeanPreserve.C16c73cd9be17a8a630fd2b154636a079.R0` and `set_option maxHeartbeats 0
```

Removed: both were restored. A semantic module is JSON and the front end
reads it with the reader's own recursion limit of 128 levels, which
SPEC.md §17.17 now records: a chain of 130 nested `let`s is `LLT4001`
`recursion limit exceeded` (`conformance_sp_02` checks it).

### no declaration can be named for a name the backend writes bare

A function, structure, or inductive named `Int`, `String`, `Unit`, `Except`,
`ByteArray`, `Ordering`, `UInt8`, `UInt64`, `Int8`, `Int64` (and the other
widths), `true`, or `false` was accepted by `check` and ended in
`error[LLV7002]: Lean rejected Coverage.Main: type expected, got (Int : Nat ->
Nat)`, because the module's namespace holds the declaration and Lean resolves
the name there before the root. A probe of every identifier the examples'
generated Lean writes (1218), each as a function in a module that uses the
built-in types, found exactly these, besides the seven refused before. A
parameter named `true` was the same defect for Boolean literals (`Application
type mismatch: The argument true has type Nat`). Quoting cannot prevent it,
since Lean resolves names and not spellings, so linking refuses every name of
`BACKEND_BARE_NAMES` (33, `true` and `false` added) for a declaration of any
kind and for a binder: `LLT4001`, the fixtures `declaration-lean-name` and
`binder-bool-literal-name`, and `conformance_df_12` for each name and each of
the three kinds.

Planted: the declaration check removed (`check_declaration_name`). Command:
`cargo test -p repo-conformance --test conformance -- conformance_df_12`.
Expected: a declaration named `Int` is accepted.

```text
thread 'conformance_df_12' (9037) panicked at crates/conformance/src/support.rs:475:14:
check fails
```

Planted: `true` and `false` removed from the names. Command: `conformance_ex_07`.
Expected: the fixture of a parameter `true` is accepted.

```text
/home/user/wt-25/tests/negative/binder-bool-literal-name: step 1 `check ` exited 0, case.toml expects 1
```

A function, structure, inductive, field, or constructor named `native_decide`
or `IO` is valid: the module quotes it, and so now does the audit module,
which printed `#print axioms Coverage.Main.native_decide` unquoted and ended
in `error[LLI9001]: phase verify: audit module LexLeanAudit...: forbidden
token Coverage.Main.native_decide in generated Lean`. A unit test of the audit
module holds the quoted text, and `conformance_sp_09` verifies, under the
pinned Lean, a project with a function `native_decide`, a structure `IO`
with fields `native_decide` and `kernel`, and an inductive `extern` with
constructors of those names.

Planted: the quoting removed from `audit_modules`. Command: `cargo test -p
lexlean --lib audit_module_tests`. Expected: the old text.

```text
thread 'backend::lean::audit_module_tests::the_audit_module_names_a_forbidden_looking_declaration_quoted' (3453) panicked at crates/lexlean/src/backend/lean.rs:2587:9:
module
import Coverage.Main
#print axioms Coverage.Main.IO
```

A `module_prefix` of `IO` gave the same internal error and `Lean`, `Init`, or
`Std` gave `LLV7003` in the middle of verification (the project's `Lean.Main`
beside the toolchain's `Lean`); a first segment of one of the toolchain's
roots (`Init`, `Std`, `Lean`, `Lake`, `LakeMain`, `LeanChecker`, `LeanIR`) or
`IO` is now a configuration error before anything runs: `LLC0101`, the fixture
`module-prefix-reserved`. The prefixes the repository uses (`LexLeanExample`, `Production`, ...) are free;
so is `LexLeanTarget` without a production root (see the module and prefix
record below).

Planted: `is_reserved_module_root` made false. Command: `cargo test -p lexlean
--lib reserved_root_tests`. Expected: a toolchain root is accepted.

```text
thread 'config::reserved_root_tests::a_toolchain_root_is_reserved_and_a_project_root_is_not' (3500) panicked at crates/lexlean/src/config.rs:1083:13:
Init
```

Removed: all four were restored.

### the token audit reads text as Lean does

The audit of certificates and library modules read characters differently from
Lean in ways that hid text from it: a character literal `'"'` opened a string
that swallowed the following `theorem t : False := sorry`; a raw string
`r"\"` ended later than Lean ends it; `sorry.1`, `0sorry`, and `sorry` followed
by a Greek letter or an accented one were read as another word than Lean
reads; and `attribute [«implemented_by» g] h` was not read as an attribute. The
scanner is now a lexer of Lean's tokens (comments with nesting, strings with
escapes and gaps, raw strings with any number of `#`, character literals with
`\x` and `\u` escapes, numbers, dotted and unicode and quoted names, commands),
fails closed on anything it cannot classify (an unclosed literal, comment, or
quotation, a control character, code in an interpolated string, a character
literal of two characters), and takes every use of a forbidden name as the name,
quoted or not. The unit test `the_text_is_read_as_lean_reads_it` holds each
probe as a refusal and the legitimate forms (`'"'`, `r#"a"b"#`, `x''`, `α₁`,
`0x1F`, `«sorry»` as a name, doc comments containing the word) as acceptances,
and `the_shipped_library_audits` reads every module of the library with it.

Planted: the character literal branch of `lex` disabled. Command: `cargo test
-p lexlean --lib production::preserve`. Expected: the first probe passes.

```text
thread 'production::preserve::tests::the_text_is_read_as_lean_reads_it' (17823) panicked at crates/lexlean/src/production/preserve.rs:1436:13:
"def c1 := '\"'\ntheorem t3 : False := sorry\ndef c2 := '\"'\n"
```

Removed: the branch was restored.

### a timeout ends the work, not only the process that was waited for

`child_timeout_ms` killed `lake`, and the `lean` it had started was left
running with its parent gone: at 100% of a core, with a resident set that grew
without bound (3.3 GB after 70 seconds, 4.5 GB after two minutes). Before the
budgets of a wide match were lifted, Lean's heartbeats ended such a process by
themselves; with them at 0 nothing did, and SPEC.md §17.17 said the timeout
bounds every Lean process. Every child of `verify` now leads a process group of
its own (`CommandExt::process_group`, safe Rust), and on a timeout (or any
other failure of waiting) the whole group is killed and awaited (first with the
platform's `kill`, which a host may not have: the section on interrupts below
replaces it with a direct signal; `taskkill /T` on windows). The unit test starts a script that
starts a long-running grandchild and outlives the timeout, then reads the
process table:

```text
test verify::child::timeout_tests::a_timeout_ends_what_the_child_started ... ok
```

Planted: the signal to the group removed, so that only the child is killed.
Command: `cargo test -p lexlean --lib timeout_tests`. Expected: the
grandchild is still running.

```text
thread 'verify::child::timeout_tests::a_timeout_ends_what_the_child_started' (13145) panicked at crates/lexlean/src/verify/child.rs:804:9:
a timeout left running: ["13149: sleep 1000.13144 "]
```

Through the CLI, `lexlean verify` of a match on 200 constructors under a
`child_timeout_ms` of 30 000 ends `error[LLS8002]: child_timeout_ms exceeded by
`lean` in phase LexLeanPreservation.RustLemmas` and, three seconds later, no
`lean` and no `lake` is running. The other places that start `lake`, `lean`,
`rustc`, or `cargo` are the test harness (no timeout) and `lock` and the PDF
provider, which go through the same function.

Removed: the signal was restored.

### a bound on certificate A that counts a type as it is printed

The early bound counted the type of a `Build`, `Value`, `Let`, or `Match` as
a tree at every node, so a pair nested `d` deep cost `d(d + 1) / 2` type
nodes at 17 bytes, and the bound exceeded the certificate from about 40 deep
(1.55 times at 120; with twenty functions of that depth, a bound of 5.08 MB on
certificates of 2.9 MB, refused at the default limit). A type node is now
counted as the certificate prints it: those of every parameter, result, and
constructor field, and the largest type written at an expression of a function
once for the function (`Measure::printed_types`); `types`, which charges
`max_ir_nodes`, is unchanged. The costs were fitted again on 166 programs
(the corpora, 32 families, the mixed shapes at five scalings, and 24 random
shapes held out of the fit): the largest ratio of a bound to its certificate
at the fitted costs is 1.00 in the fit and 0.99 in the held-out programs, and
0.50 at half. The families added are pairs nested 20, 60, and 120 deep, the same
in twenty functions, options, lists, and results nested 15 to 50 deep, and
records nested 10 and 30 deep, and two of the mixed shapes (and every random one)
have a nested pair of up to 120 deep in up to 24 functions.

Planted: the type of an expression counted at every node again
(`note_type` keeps `printed_types`). Command: `cargo test -p repo-conformance
--test conformance -- conformance_sp_02`. Expected: a program whose
certificates fit is refused.

```text
Production.Main.r: lowering failed: Diagnostic { code: DiagnosticCode("LLS8002"), message: "root `Production.Main.r`: max_file_bytes exceeded in phase lowering: configured 4194304, its certificate A is at least 6508182 bytes (363049 program nodes)", primary: None, labels: [], notes: [], help: [], causes: [], detail: None }
```

Removed: the count was restored. Certificate E's bound no longer has a cost
for shapes (it was above E for 24 functions of nested pairs, 3.7 times): E
states the entry, whatever else the program holds.

### a module is a namespace, and a prefix is not the library's

A module of a project is a namespace Lean searches before the root from every
module of it, so a module `Nat` with a function `blt` captured `Nat.blt` in
itself and in the modules that import it (`error[LLV7002]: Function expected
at Nat.blt`). Language 1.2 refuses a module named like a namespace the
generated code writes qualified: the names of `BACKEND_BARE_NAMES`, the
runtime classes and Lean namespaces of `QUALIFIED_NAMESPACES`, and the roots
this compiler ships; `conformance_sp_09` reads every uppercase namespace the
corpora's generated modules write qualified off them and requires that a module
of each name is refused (the first run named `Appendable`, `Decimal`, `Fixed`,
`Indexable`, `Key`, `Lengthable`, `Sliceable`, `ToMathInt`, besides `Ord` and
`Quotient`, which the list now holds). Fixture `module-name-lean-namespace`.

A `module_prefix` that begins with `LexLeanPreserve`, `LexLeanTarget`, or
`LexLeanPreservation` passed `check` and failed every production `verify`
(`LLV7013`, `object file … of module LexLeanTarget.Main does not exist`). The
roots this compiler ships or generates (`SHIPPED_ROOTS`, eleven, checked against
the roots of the shipped library by `every_shipped_root_is_reserved`) are
`LLC0101` at configuration. `LexLeanTarget` is the root of the calculus modules
that the library imports and the prefix of the repository's `compiler` project,
which has no production root: it is accepted, and a module with a production
root under it is `LLT4005`. (The previous record called `LexLeanTarget` free,
which is true only without a production root.) Fixtures
`module-prefix-shipped-root` and `module-prefix-target-production`.

Planted: `is_reserved_module_name` made false. Command: `conformance_sp_09`.
Expected: the namespaces are named.

```text
generated modules write namespaces qualified that a module name is not refused for: {"Appendable", "Decimal", "Fixed", "Indexable", "Key", "Lengthable", "Ord", "Quotient", "Sliceable", "ToMathInt"}
```

Planted: the shipped roots removed from `is_reserved_module_root`. Command:
`cargo test -p lexlean --lib reserved_root`. Expected: the prefixes are free.

```text
thread 'config::reserved_root_tests::every_shipped_root_is_reserved' (32745) panicked at crates/lexlean/src/config.rs:1117:13:
```

Removed: both were restored.

### a name Lean declares itself, and a parameter named like a constructor

A constructor named `rec`, `recOn`, `casesOn`, `noConfusion`,
`noConfusionType`, `ctorElim`, `ctorElimType`, `ctorIdx`, or `rec_1`, or a field
named `rec`, `recOn`, `casesOn`, `noConfusion`, `noConfusionType`, or `ctorIdx`,
was accepted by `check` and ended in `error[LLV7002]: … (kernel) constant has
already been declared 'Coverage.Main.I_rec.rec'`. `LEAN_GENERATED_MEMBERS` is
read off the pinned Lean: `conformance_df_12` declares ten samples (a
recursive inductive, a nested one, mutual ones, a higher-order one, an
enumeration, structures with a proposition field) and lists the names Lean holds
under each besides the user's own and the `_sizeOf_` ones, which no source name
can spell: `casesOn`, `ctorElim`, `ctorElimType`, `ctorIdx`, `noConfusion`,
`noConfusionType`, `rec`, `recOn`, `below`, `brecOn`, `toCtorIdx` (enumerations),
`mk` (structures), and the numbered `rec_1`, `below_1`, `brecOn_1`. They are
`LLT4001` for a constructor or a field (`mk` only for a field); the near names
(`rec_`, `recur`, `ctorIdx0`) are free. Fixture `member-generated-name`.

A used parameter named like a nullary constructor of its own type (`zero :
nat`, `none : option nat`, `nil : list nat`, `red : Color`) is valid, and ended
in `error[LLV7006]: Lean produced unexpected output … Local variable 'red'
resembles constructor 'Production.Main.Color.red'`. The constructors of a module
are never written bare, so its header now silences
`linter.constructorNameAsVariable`; a project of the four verifies.

Planted: the check of the generated members removed. Command: `conformance_df_12`.
Expected: a constructor `rec` is accepted.

```text
thread 'conformance_df_12' (358) panicked at crates/conformance/src/support.rs:475:14:
check fails
```

Planted: the option removed from the header. Command: `lexlean verify` of the
project of the four. Expected: the old error.

```text
error[LLV7006]: Lean produced unexpected output for `Production.Main` (warning): Local variable 'red' resembles constructor 'Production.Main.Color.red' - write '.red' (with a dot) or 'Production.Main.Color.red' to use the constructor.
Note: This linter can be disabled with `set_option linter.constructorNameAsVariable false`
```

Removed: both were restored.

### the lexer takes the longest token

`def x : Float := 1e10axiom bad : False` was read as `1`, the name `e10axiom`,
and so on: the exponent of a number was not scanned, and Lean reads `1e10`
and the keyword `axiom` (the same for `1.5e3unsafe` and `1e10#evalIO.println
"pwn2"`, which printed `pwn2`); and `#evalIO.println "pwned"` was one command
`#evalIO` where Lean takes the longest token it knows, `#eval`, and a name. A
number now takes `[eE][+-]?[0-9]+` after its digits and fraction (an `e` with
no digits is refused, as Lean refuses it), and `0b`, `0o`, `0x` the digits of
their radix (none is refused); every `#` command is refused, since neither a
library module nor a certificate holds one and the word after the `#` is not
read. The unit test holds the reviewer's probes (and `#evalpwned`,
`#printaxiom`, `0b1sorry`) as refusals and `1e10`, `1E-3`, `1.5e+3`, `0xFF`,
`0b101e` as acceptances.

Planted: the exponent not scanned. Command: `cargo test -p lexlean --lib
production::preserve`. Expected: the first probe passes the audit.

```text
thread 'production::preserve::tests::the_text_is_read_as_lean_reads_it' (32386) panicked at crates/lexlean/src/production/preserve.rs:1436:13:
"def x : Float := 1e10axiom bad : False\ntheorem t : False := bad\n"
```

Planted: commands refused only when they are exactly `#evalnever` (the old
exact match, in effect). Expected: `1e10#evalIO.println "pwn2"` and the rest
pass.

```text
thread 'production::preserve::tests::the_text_is_read_as_lean_reads_it' (32444) panicked at crates/lexlean/src/production/preserve.rs:1436:13:
"def x := 1e10#evalIO.println \"pwn2\"\n"
```

Removed: both were restored.

### the eligibility reports of a project are bounded together

The budget was per module: six modules of a chain of 240 roots wrote six
reports of 58 984 126 bytes (354 MB) against a limit of 67 108 864, and `build`
held 1.18 GB. The analysis now carries the bytes of the reports of the modules
before it (`check` and `build` and `verify` all go through it), and `build` sums
the files it writes. The fixture `eligibility-reports-total-limit` is three
modules of 60 roots, each of 1.6 MB, under a limit of 4 194 304: the third
module is refused.

```text
error[LLS8002]: phase production: the production-eligibility report of module `Main` is beyond the 4194304 bytes of max_total_source_bytes at root `Production.Main.r56`: a member of a closure records its shortest call path, so a chain of calls takes the square of its length and roots that call one another the cube of their number
```

Planted: each module starts from 0 again (the old behaviour). Command:
`lexlean check` in the fixture. Expected: the project is accepted.

```text
checked 3 modules (source 20f1bcc232bf1186b1118a118beb8947341bfcf4c524d2ef57af5f19a2cd782b, semantic 8077be4ab922d16dc2f9611b1b915d5562759428c237a5d16f3f24cdb71b8e03)
exit 0
```

Removed: the carry was restored.

### certificates A and E are generated under the limit

At `96088be` the estimate (nodes only) was below certificate A for a string
literal of 100 000 characters (0.01), an enumeration of 2000 constructors
(0.05), and a match nested 25 deep (0.26), and `lexlean verify` generated a
certificate A of 297 MB for 4000 constructors before refusing it. Every
certificate is now generated under `max_file_bytes`: the proof of each term,
each match arm, the encoders, each function, and the boundary are checked as
they are built. A match arm was also written by wrapping the whole arm again
for each preceding arm, which is cubic in the arms in time; it is written
once. `conformance_sp_02` certifies the stress families (literals,
enumerations of many constructors, nested matches, wide instances, long
names) and regenerates A and E of every family under half their size,
requiring `LLS8002`.

Planted: the byte budget removed from certificate A (`within` never refuses).
Expected: A is regenerated under half its size and accepted.

```text
thread '<unnamed>' (28544) panicked at crates/conformance/src/cases/preservation.rs:384:17:
assertion `left == right` failed: doubling let chain 4: Production.Main.r: certificate A under half its size
  left: "accepted"
```

Removed: the budget was restored. The early refusal is a lower bound and the
budget is the guarantee; the SPEC says so.

### the cost of certification is fixed, not proportional to the roots

`lexlean verify` of a project with a production root compiles the whole
preservation library (13 modules) before the first certificate, whether the
project has one root or many; a reviewer measured about 2.5 minutes for it,
209 s for a six-root project and 144 to 157 s for a three-root one. The
preservation suite reflects it: on the host that ran the plants above,
`conformance_sp_10` took 236 s, `conformance_sp_05` 131 s, and
`conformance_sp_02` 727 s alone; the whole suite is about 24 minutes at three
test threads. A CI budget for the Lean-backed jobs must count that, not the
number of roots.

Scope note: the relaxation of named-root extraction that admits `Init`
definitions exported as axioms (NE-03) shares this change because the
certificates need it; it is recorded in `CHANGELOG.md` and is not part of the
semantic-preservation claim.

### an interrupt ends the children, and a group is ended without `kill`

A child leads a process group of its own so that a timeout can end what it
started. That took it out of the group the terminal signals: a Ctrl-C (or the
`SIGTERM` of `timeout 600 lexlean verify`) ended `lexlean` and left the `lake`
and the `lean` it had started running at full speed. `child::run` now starts a
thread, once, that waits for `SIGINT`, `SIGTERM`, and `SIGHUP` (the next section
says what changed since: `SIGQUIT`, ignored signals, who arms it) (`signal-hook`,
Apache-2.0 OR MIT, with `libc` and `signal-hook-registry`, which `cargo deny`
accepts under the licences already allowed); the thread kills every live child
group, and lets the signal end the process, so the exit status is the
signal's. A child is registered under the lock the thread takes, so a signal
between the spawn and the registration still ends it. `SIGKILL` cannot be
caught and leaves the children; SPEC.md says so.
The code is `crates/lexlean/src/verify/lifeline.rs`, a module of its own with
item-level `cfg`: an inline `#[cfg(unix)] mod` makes `conformance_rp_12` treat every
test after it in the file as cfg-hidden (it did, on the first full run).

The group was signalled by a `kill` executable found on `PATH`, and where there
was none (the shipped `debian:bookworm-slim` image has no `procps`) only `lake`
was killed and the failure dropped. It is now `rustix::process::kill_process_group`
(a safe wrapper; the crate still forbids unsafe code), and a group that could
not be signalled, or that is still there two seconds after `SIGKILL`, is said in
the diagnostic: `the processes it started may still be running: …`.

`crates/lexlean/tests/interrupt.rs` runs the real `lexlean lock --allow-network`
with a `git` on `PATH` that starts a `sleep` and never finishes, waits until
the `sleep` runs, sends `lexlean` `SIGINT` and, in a second run, `SIGTERM`,
and requires that `lexlean` ended by the signal itself and that no `sleep` is
left. `crates/lexlean/tests/stop_without_kill.rs` sets `PATH` to a directory
holding only `sleep` (and requires that `kill` cannot be resolved there) and
requires that a timeout ends the grandchild.

Planted: the handler does not signal the live groups. Command: `cargo test -p
lexlean --test interrupt`. Expected: the `sleep` is left running after
`SIGINT`.

```text
thread 'an_interrupt_leaves_no_child_running' (28288) panicked at crates/lexlean/tests/interrupt.rs:134:5:
SIGINT left running: [(28294, "sleep 1000.28287 ")]
```

Planted: `stop` does not signal the group (only the child is killed and awaited).
Command: `cargo test -p lexlean --test stop_without_kill`. Expected: the
grandchild survives and the diagnostic says so, rather than saying nothing.

```text
thread 'a_timeout_ends_the_group_on_a_host_without_kill' (28375) panicked at crates/lexlean/tests/stop_without_kill.rs:59:5:
the group was ended, so the report does not claim otherwise: Diagnostic { code: DiagnosticCode("LLS8002"), message: "child_timeout_ms exceeded by `lake` in phase Probe: configured 700, observed 702 ms; the processes it started may still be running: processes of group 28377 still existed 2000 ms after SIGKILL"
```

Removed: both were restored.

### the interrupt handler leaves the process alone, and a zombie is ended

Review r49h found that the thread above did more than it should. It is now
armed by `cli::main_entry`, which only the `lexlean` executable calls, and the
first child installs it only then: `lexlean::cli::run`, called by a host that
embeds the library, installs no handler, so the host's `SIGTERM` is the
host's. The thread watches `SIGINT`, `SIGTERM`, `SIGHUP`, and `SIGQUIT`
(Ctrl-\ used to leave the child running), less the signals the process was
started ignoring: the mask is read before anything is installed, from
`SigIgn` of `/proc/self/status` or from `ps -o ignored`, so a `nohup lexlean
verify` is not ended by the `SIGHUP` it was told to ignore; where the host can
report neither, `SIGHUP` is not watched. `stop` counts a zombie of the group
as ended, and reaps the ones that are children of this process (the shipped
image runs `lexlean` as PID 1, which never reaps the grandchildren it killed,
so the group seemed to outlive `SIGKILL` and every timeout waited two seconds
and claimed that processes might still be running); the process table is
`/proc` or `ps -axo pid=,ppid=,pgid=,stat=`. A `taskkill` that exits non-zero
is reported (`taskkill_outcome`), and `verify` removes the
`.lexlean/verified/.staging-*` directories that interrupted or killed runs left
(`remove_stale_staging`, under the project's mutation lock; a file, a link, and
a directory of another name are left alone). SPEC.md §8.5 gives `rustix` and
`signal-hook` (and every other direct dependency) a role, and `audit-shipped`
requires the table and the declared dependencies of `lexlean` to be the same
set.

Tests: `tests/interrupt.rs` sends `SIGINT`, `SIGTERM`, `SIGQUIT` and (where
`/proc` says which signals were ignored) `SIGHUP` to the real binary and requires
that nothing is left; `an_ignored_signal_stays_ignored` starts it through `sh -c
'trap "" HUP; exec lexlean ...'` (and `INT`, `QUIT` where there is a `/proc`),
signals it, requires that it and its child are still there 1.5 s later, and
then that `SIGTERM` still ends both. A case skips a signal that the test process
itself was started ignoring (a test run in the background of a
non-interactive shell), and says so, because `lexlean` rightly keeps it ignored.
`tests/embedded.rs` registers a `SIGTERM` flag as a host would, calls
`lexlean::cli::run` on a project whose `git` starts and fails, and then signals
its own process: the host's handler must see it and the process must live.
The unit tests are `a_zombie_in_the_group_counts_as_ended` (a zombie child of
the test process in the group of a killed `sleep`; the stop returns `Ok` within
1.5 s), `a_failing_taskkill_is_reported`, `an_ignored_signal_is_not_watched`,
`the_mask_of_this_process_is_read`, and
`stale_staging_directories_are_removed_and_nothing_else`; `verify_ok` of the
conformance suite plants a stale staging directory before every verification and
requires that it is gone afterwards. The windows build of the tests and the
library is checked with `cargo clippy -p lexlean --all-targets --target
x86_64-pc-windows-gnu -- -D warnings`.

The binders of the declarations that Lean elaborates inside a namespace
(`def X.initial`, `X.guard`, `X.apply` of models and reasoners, where Lean opens
`X`) were probed with the real `lexlean verify`: a model whose state is a
`match` with the binder `initial`, the very name of the declaration being
defined, verifies; the binder `none` is `LLT4001`, as in every declaration. A
data type cannot share the name of a model or a rule, since that is `duplicate
generated name` at link (probed with a type `Fever` beside the rule `Fever`),
so the namespace of `X` holds no constructor that a pattern binder could be
read as; a definition of the namespace (`X.guard`) in a pattern is a variable,
as the pinned Lean elaborates it (`some guard => guard + 1` over
`def A2.guard`).

Planted: the signals the process was started ignoring are watched like the
others. Command: `cargo test -p lexlean --test interrupt -- an_ignored_signal_stays_ignored`.
Expected: the process started with `SIGHUP` ignored is ended by it.

```text
thread 'an_ignored_signal_stays_ignored' (30305) panicked at crates/lexlean/tests/interrupt.rs:198:13:
SIGHUP was ignored at start, yet lexlean ended (Some(ExitStatus(unix_wait_status(1)))) or lost its child ([])
```

Planted: the handler is installed whether or not the executable armed it.
Command: `cargo test -p lexlean --test embedded`. Expected: the embedding host is
ended by its own `SIGTERM`.

```text
error: test failed, to rerun pass `-p lexlean --test embedded`

Caused by:
  process didn't exit successfully: `/home/user/wt-24/target/debug/deps/embedded-d0e54ad484b58cfc` (signal: 15, SIGTERM: termination signal)
```

Planted: `SIGQUIT` is not among the signals watched. Command: `cargo test -p
lexlean --test interrupt -- an_interrupt_leaves_no_child_running`. Expected: the
child survives Ctrl-\.

```text
thread 'an_interrupt_leaves_no_child_running' (29414) panicked at crates/lexlean/tests/interrupt.rs:229:5:
SIGQUIT left running: [(29432, "sleep 3000.29413 ")]
```

Planted: `stop` reads no process table (every member of the group is taken to be
running). Command: `cargo test -p lexlean --lib -- a_zombie_in_the_group`.
Expected: the stop waits two seconds and reports the zombie.

```text
thread 'verify::child::timeout_tests::a_zombie_in_the_group_counts_as_ended' (30432) panicked at crates/lexlean/src/verify/child.rs:880:9:
assertion `left == right` failed: a zombie is not running
  left: Err("processes of group 30433 still existed 2000 ms after SIGKILL")
 right: Ok(())
```

Planted: `taskkill_outcome` takes any exit status for success. Command: `cargo test
-p lexlean --lib -- a_failing_taskkill`. Expected: an exit status 128 is not
reported.

```text
thread 'verify::child::timeout_tests::a_failing_taskkill_is_reported' (30484) panicked at crates/lexlean/src/verify/child.rs:826:68:
exit 128: ()
```

Planted: `verify` does not call `remove_stale_staging`. Command: `cargo test -p
repo-conformance --test conformance -- conformance_df_12`. Expected: the
staging directory of an interrupted run is still there after the verification.

```text
thread 'conformance_df_12' (29640) panicked at crates/conformance/src/support.rs:2032:5:
/tmp/lexlean-example-case-0Rm1sS/.lexlean/verified/.staging-interrupted-0: the staging directory of an interrupted run is removed by the next verify
```

Planted: the role of `signal-hook` removed from the table of SPEC.md §8.5 (the
name spelled differently). Command: `cargo xtask validate-model`. Expected:
`audit-shipped` fails naming the dependency.

```text
gate failed: R6: the shipped crate depends on `signal-hook`, which SPEC.md §8.5 gives no role
```

Removed: all seven were restored (the unit test of the audit,
`every_shipped_dependency_has_a_role_and_every_role_a_dependency`, plants a
dependency with no role, a role with no dependency, and a SPEC without the
table).

### inherited output remains subject to the child deadline (Issue #132)

A leader can exit successfully while a descendant still holds stdout or
stderr. The runner now keeps its existing timeout and process-group guard
active until the leader has exited and both output readers have finished;
joining the readers cannot bypass the deadline. Timeout and host shutdown
still use the existing termination path. `SE-06` also exercises a finite
descendant holding stdout after its leader exits; the crate's unit tests
cover each stream separately, group cleanup, and a descendant that finishes
within its deadline with the successful status and output retained.

Before the fix, `cargo test -p lexlean --lib a_timeout_bounds_` ran both
inherited-pipe regressions and both failed in 3.00 seconds: `expect_err`
received a successful `ChildRecord` with `exit_code: 0` after the finite
descendant completed, despite `child_timeout_ms = 200`.

The separate serialized host regression, `cargo test -p lexlean --test
end_children an_embedding_host_ends_a_group_after_its_leader_exits`, failed
in 0.02 seconds before the fix:

```text
the group stays registered while its output is being read
  left: []
 right: [ChildGroupEnd { group: 102, outcome: Ended }]
```

The case waits until the leader is actually reaped before asking the host
to end the remaining group, and its descendant is finite even when the
assertion fails. These process-group assertions run on Unix; the existing
Windows tree-termination implementation is unchanged.

After the fix, all seven timeout unit cases pass in 1.21 seconds, the
hosting regression passes in 0.03 seconds, and `conformance_se_06` passes in
1.79 seconds. Formatting, workspace/all-target Clippy with warnings denied,
and the model/spec-link audits pass. The existing hosting suite passed nine
cases in the sandbox; its real-signal case passes outside that sandbox in
0.04 seconds, where SIGINT and SIGTERM reach its child host.

### a host ends the children the library started

A host that embeds the library (`lexlean::cli::run`, a service, a test
harness) owns its own signals and its own termination, and the library
installs no handler for it; until this change it had no way to end the
children the library had started, so an interrupted host left its `lake` and
`lean` running. The library now exposes `lexlean::verify::end_live_children`,
which ends every child process group it started in this process and has not
seen exit and reports each group as `Ended`, `AlreadyGone`, or `Failed` with
the OS error, and `lexlean::verify::allow_children`, which re-enables starting
children after the call. The shutdown state is set and the live list is taken
under the one lock that registers a child, so a child started concurrently
with the call is either ended by it or refused (`LLV7018`), never left
running; a zombie in a group counts as ended, and the host's own process and
process group are never signalled. The executable's interrupt thread and a
run's own timeout end groups through the same function
(`verify::lifeline::end_leader`), so exactly one implementation of "end the
live groups" exists, the executable is its first user, and a second place
that signals a group is a failure of `conformance_cl_22`, which counts the
sites across every source file of the library, not a review comment.

`crates/lexlean/tests/end_children.rs` runs an embedding host that calls
`lexlean::cli::run` against a fake `git` that starts a long `sleep`, ends the
live children while it runs, and requires that no process of the group
remains, that the call reports every group ended, and that the run returns a
diagnostic rather than hanging. It runs a host **process** of its own (the
test binary, run again for one case, as the leader of its own process group
the way a service manager starts one) that registers its own handling of
`SIGINT` and `SIGTERM`, delivers each to it from outside while a child is
live, and requires that the host ends the children from its handling's thread,
survives to run its own cleanup, exits by its own status and not by a signal,
that its pid and pgid are untouched, and that its signal dispositions (`SigCgt`
and `SigIgn` of `/proc/self/status`, or the `caught` and `ignored` columns of
`ps`) are equal before the library starts a child, after it has started
one, and after it has ended it; a host with `/proc` must be able to report
them, a host with neither `/proc` nor `ps` is an error and not a pass, and
the `ps` of macOS has neither column, which the first run of this branch on
the two macOS legs showed (`ps cannot report the caught signals`). What shows
the same thing on every host is a second host process that handles no signal:
it is sent `SIGQUIT` while a child the library started is running, and must
die of that signal by its default action with the child still running, which
a handler the library had installed would have ended first
(`a_signal_the_host_did_not_handle_ends_it_by_default`). A fresh process is
each case's own, because a disposition that an earlier case of the same
process had already changed would hide the library's doing. It covers the no-op, the idempotent second call, a
child that already exited (`AlreadyGone`), the refusal of a child started
after the call, and a concurrent-starts stress that ends the live children
sixty times while four threads keep starting them and reads the process table
after every call. The correct function passes every cycle: it failed in 0 of
100 runs of `cargo test -p lexlean --test end_children
concurrent_starts_are_ended_or_refused`. A simulated `EPERM` and a failing
`taskkill` are unit tests of the outcome handling
(`verify::lifeline::tests::a_refused_group_is_reported`,
`verify::child::timeout_tests::a_failing_taskkill_is_reported`). On Windows
`taskkill /PID <pid> /T /F` exits 128 alike for a process that does not exist,
a process that has exited while its handle is still open (the library's `Child`
holds it), and access denied (measured on Windows 11 by the author of #74), so
a tree that has already exited could not be told from one that could not be
ended, and the acceptance criterion "a call after the children have already
exited reports them as gone, not as a failure" would have failed there. A
`taskkill` that fails is therefore classified by asking `tasklist` whether the
root process is still listed: not listed is `AlreadyGone`; listed, or
`tasklist` unable to say, is `Failed`; the classification is the pure function
`verify::lifeline::tree_outcome`, tested on every host with the five cases
(success, exit 128 not listed, exit 128 still listed, exit 128 unexplained,
`taskkill` not runnable). The unix
cases are compiled out on Windows, and `cargo clippy --workspace --all-targets
--all-features -- -D warnings` was run for `x86_64-unknown-linux-gnu`,
`x86_64-pc-windows-msvc`, and `aarch64-apple-darwin`, which is how a Windows
`spawn` that used a `Child` after moving it was found (the first run of this
branch had not reached the host matrix). `conformance_cl_22` holds the
register, the public functions, the single kill path, and the registered
`LLV7018` to the source.

Each of the ten plants below was applied alone to the finished sources,
produced the failure quoted, and was removed.

Planted: `end_leader` ends only the direct child and not its group
(`kill_process` for `kill_process_group`). Command: `cargo test -p lexlean
--test end_children an_embedding_host_ends_every_live_group`. Expected: the
grandchild survives and its group is reported failing, not ended.

```text
thread 'an_embedding_host_ends_every_live_group' (32748) panicked at crates/lexlean/tests/end_children.rs:223:5:
every live group is reported ended: [ChildGroupEnd { group: 32750, outcome: Failed("processes of group 32750 still existed 2000 ms after SIGKILL") }]
```

Planted: `end_leader` signals the host's own process group
(`kill_process_group(getpgid(None))` for the child's group). Command: `cargo
test -p lexlean --test end_children the_host_ends_children_from_its_own_signal_thread`.
Expected: the host process, which leads its own group, is killed by its own
`SIGKILL` before it can finish, and the case says so.

```text
thread 'the_host_ends_children_from_its_own_signal_thread' (1162) panicked at crates/lexlean/tests/end_children.rs:403:5:
the host must survive both signals, run its own cleanup, and exit by its own status, not by a signal (it said "the host said nothing more" and ended with ExitStatus(unix_wait_status(9)))
```

Planted: `signal_outcome` takes any refusal for success (`Err(_errno) =>
Some(GroupEnd::Ended)`). Command: `cargo test -p lexlean --lib
verify::lifeline::tests::a_refused_group_is_reported`. Expected: a simulated
`EPERM` is reported ended, not failed.

```text
thread 'verify::lifeline::tests::a_refused_group_is_reported' (1582) panicked at crates/lexlean/src/verify/lifeline.rs:616:17:
EPERM is reported, not taken for success: Some(Ended)
```

Planted: `tree_outcome` takes every failed `taskkill` for a tree that was
already gone (`_ => GroupEnd::AlreadyGone`). Command: `cargo test -p lexlean
--lib a_failing_taskkill`. Expected: an exit status 128 for a process that
`tasklist` still lists is reported, not called gone.

```text
thread 'verify::child::timeout_tests::a_failing_taskkill_is_reported' (4104) panicked at crates/lexlean/src/verify/child.rs:704:9:
exit 128 with the process still listed is reported: AlreadyGone
```

Planted: the executable's interrupt thread signals the live groups itself
instead of calling the public function. Command: `cargo test -p repo-conformance
--test conformance -- conformance_cl_22`. Expected: the "exactly one kill
path" check finds a second signal site in the module.

```text
thread 'conformance_cl_22' (2241) panicked at crates/conformance/src/cases/cli_api.rs:1101:13:
assertion `left == right` failed: §24.6: a process group is signalled in exactly one place
  left: ["verify/lifeline.rs", "verify/lifeline.rs"]
 right: ["verify/lifeline.rs"]
```

Planted: the timeout path of a run signals the group itself (a
`kill_process_group` call in `verify::child::stop`, as it did before the two
shared one function). Same command. Expected: the same check finds the second
site in another file, which a check of `lifeline.rs` alone could not.

```text
thread 'conformance_cl_22' (8184) panicked at crates/conformance/src/cases/cli_api.rs:1101:13:
assertion `left == right` failed: §24.6: a process group is signalled in exactly one place
  left: ["verify/child.rs", "verify/lifeline.rs"]
 right: ["verify/lifeline.rs"]
```

Planted: `stop` (the timeout path) no longer ends the group, only the direct
child. Command: `cargo test -p lexlean --lib
verify::child::timeout_tests::a_timeout_ends_what_the_child_started`.
Expected: the grandchild outlives the timeout and the case fails.

```text
thread 'verify::child::timeout_tests::a_timeout_ends_what_the_child_started' (8473) panicked at crates/lexlean/src/verify/child.rs:670:9:
```

Planted: the shutdown state is dropped from `end_live_children` (the
`SHUTDOWN.store(true, Ordering::SeqCst)` removed), so a child started after
the call is no longer refused. Command: `cargo test -p lexlean --test
end_children a_child_started_after_the_end_is_refused`. Expected: the child
starts, and the case fails at its own refusal assertion.

```text
thread 'a_child_started_after_the_end_is_refused' (4360) panicked at crates/lexlean/tests/end_children.rs:646:18:
a child started after the end must be refused
```

Planted: the registration lock is released between reading the shutdown state
and registering the child (`drop(groups)` before `command.spawn()`, the lock
taken again to push). Command: `cargo test -p lexlean --test end_children
concurrent_starts_are_ended_or_refused`. Expected: a child that has started and
is not registered is alive when a call returns. The first version of this
case ended the children once and caught the plant in 59 of 60 runs, which is a
timing assumption; the case now ends them sixty times and the plant was caught
in 100 of 100 runs, at cycle 0 in the one quoted.

```text
thread 'concurrent_starts_are_ended_or_refused' (4888) panicked at crates/lexlean/tests/end_children.rs:779:9:
cycle 0: a child started concurrently with the call survived it: [(4893, "sleep 300.4887 "), (4894, "sleep 300.4887 "), (4896, "sleep 300.4887 ")]
```

Planted: the library installs its interrupt handler without the executable
having armed it (the `ARMED` test removed from `install`), which is what a
host that embeds the library must never see. Command: `cargo test -p lexlean
--test end_children the_host_ends_children_from_its_own_signal_thread`.
Expected on a host that can report its masks (Linux): the host's caught-signal
mask changes (the bit of `SIGQUIT`, which the host did not register) when it
starts its first child, and the case fails there; and on every host the
unhandled `SIGQUIT` finds the child already ended, which is the second case
quoted below. The first version of this assertion ran in the process of the other
cases, where an earlier case had already installed the handler and the
comparison passed under the plant; that is why the host is a process of its
own.

```text
thread 'the_host_ends_children_from_its_own_signal_thread' (5490) panicked at crates/lexlean/tests/end_children.rs:403:5:
the host must survive both signals, run its own cleanup, and exit by its own status, not by a signal (it said "the host said nothing more" and ended with ExitStatus(unix_wait_status(25856)))
thread 'the_host_process' (5494) panicked at crates/lexlean/tests/end_children.rs:134:9:
assertion `left == right` failed: starting a child changed a signal disposition of the host
  left: Some((4294984770, 4096))
 right: Some((4294984774, 4096))
```

and, with no mask read, `cargo test -p lexlean --test end_children
a_signal_the_host_did_not_handle_ends_it_by_default`:

```text
thread 'a_signal_the_host_did_not_handle_ends_it_by_default' (5955) panicked at crates/lexlean/tests/end_children.rs:450:5:
assertion `left == right` failed: the library installed no handler, so it did not end the child when the host died of the signal: []
  left: 0
 right: 1
```

Removed: every plant was restored; the tests pass.

### a module is a namespace at every segment

Module `Sub.Nat` with a function `blt`, imported by `Sub.Other`, was accepted by
`check` and refused by `verify`: `Nat.blt` in `Sub.Other` resolved to
`Prefix.Sub.Nat.blt` (`Application type mismatch`). Lean searches the enclosing
namespaces of `Prefix.Sub.Other`, which include `Prefix.Sub`, and every segment
of a module name is a namespace of the project. Linking now tests every segment
of a dotted name against the reserved set (`reserved_module_segment`); the
fixtures `module-name-dotted-last` (`Sub.Nat`) and `module-name-dotted-first`
(`Nat.Sub`) are `LLT4001`, and the existing `module-name-lean-namespace` message
names the segment.

Planted: `reserved_module_segment` tests the whole name only. Command: `lexlean
check` in each of the fixtures `module-name-dotted-last` and
`module-name-dotted-first`. Expected: both are accepted.

```text
checked 2 modules (source 4bd59b5867512b66e98b35dd74d27eef50e6bc850c897d5af4d530678c0d3cc4, semantic e904a4a2e026042bc708e7b8129258d69ae2763c697b71f3d11cebfbb1a76c05)
checked 2 modules (source c584273f917668a46189fa9c29cbfbe55135d7951b24fd93bacee8b9622ea2fc, semantic 24d36bc5a7c86fae8260eb6c958736b683fda227531b8ae77e5fd43ef62cedde)
```

Removed: the segment check was restored; both are `LLT4001`.

### a pattern binder is read as the constructor it is spelled like

A binder `none` in the branch `Option.some` of an `option (option nat)` was
generated as `Option.some none`, which Lean reads as the constructor, so the
match was not exhaustive (`Missing cases`) after `check` had accepted it. Read off
the pinned Lean by `conformance_df_12` (every constructor of `Init` whose last
component, written bare, resolves to a constructor), the names are `false`,
`true`, `isFalse`, `isTrue`, `none`, `some`: the first two were refused as
built-in names already, and the rest are `PATTERN_CONSTRUCTOR_NAMES`, refused as
pattern binders only (`LLT4001`, fixture `binder-pattern-constructor-name`). A
parameter of these names is a variable and stays valid (the backend writes the
constructors qualified); a pattern binder `zero`, `nil`, or `cons` is a
variable, and the case verifies a module of them with the real Lean.
SPEC.md and CHANGELOG.md said that a parameter named like a nullary
constructor of its own type is valid, which is true of parameters; they now say
what holds of pattern binders.

Planted: the refusal of pattern binders removed. Command: `lexlean check`, then
`lexlean verify`, in the fixture `binder-pattern-constructor-name`. Expected:
`check` accepts it and Lean rejects it, the state before.

```text
checked 1 module (source 69a1b8dbb8e3e72bcaf6ce3e8c77a5363bb269a8dfca623c8b95786311caf795, semantic 71742264c7601d60fb55f1c9008080c221571a48031f15e3847dee19c77b7977)
error[LLV7002]: Lean rejected `LanguageTwelve.Main` (error): Missing cases:
(some (some _))
```

Removed: the refusal was restored (`LLT4001`, `pattern binder `none` is spelled
like a constructor Lean resolves without a namespace`).

### the lexer reads a number as Lean's `numberFnAux` does

Lean 4.32.1 reads `_` between and after digits (`takeDigitsFn`), a bare dot
followed by an exponent (`1.e5`), and nothing but digits after a `.` that follows
a term (`p.1.2`, `fieldIdxFn`); the lexer read `1_0axiom` as `1` and the name
`_0axiom`, so the audit accepted `def x := 1_0axiom bad : False` and Lean
declared the axiom. The number branch follows the pinned source line for line
(`Lean/Parser/Basic.lean`: `decimalNumberFn`, `parseOptDot`, `parseOptExp`,
`binNumberFn`, `octalNumberFn`, `hexNumberFn`; there are no hexadecimal floats
in 4.32.1, `0x1.8` is `0x1`, `.`, `8`). Where Lean refuses the text (`1_` and a
non-digit, `1.foo`, `1e`) the audit refuses it, and a field index followed by
`_` or an exponent, which the token reads differently by the parse, is refused
rather than guessed. `conformance_sp_09` writes 611 prefixes (radix
prefixes, digit runs with separators, fractions, bare dots, exponents, signs,
ranges, field indices) followed by `axiom bad : False`, has the pinned Lean say
which declare the axiom, and requires the audit to agree on every text it reads.

The first run of the differential failed on fourteen prefixes (`1e5.`,
`0x1.`, `p.1.`, ...) followed by `axiom`: Lean reads the number and the field
`axiom` of it, a name, where the audit reads the keyword. That is a text the
audit refuses and Lean accepts, which misses nothing, and the test now counts
those prefixes (a word right after a `.`) apart; every other prefix agrees.

Planted: a `_` ends the number. Command: `cargo test -p lexlean --lib
production::preserve`. Expected: `1_0axiom` passes the audit.

```text
thread 'production::preserve::tests::the_text_is_read_as_lean_reads_it' (27826) panicked at crates/lexlean/src/production/preserve.rs:1436:13:
"def x := 1_0axiom bad : False\n"
```

The same plant, against the pinned Lean. Command: `cargo test -p repo-conformance
--test conformance -- conformance_sp_09`. Expected: the audit misses the axiom
that Lean declares.

```text
thread 'conformance_sp_09' (27961) panicked at crates/conformance/src/cases/preservation.rs:277:5:
the audit reads numbers differently from the pinned Lean:
`0.5_`: the audit reads the keyword false, Lean declares it true
`0.5_0`: the audit reads the keyword false, Lean declares it true
`00_`: the audit reads the keyword false, Lean declares it true
`00__0`: the audit reads the keyword false, Lean declares it true
```

Removed: the lexer was restored; `conformance_sp_09` passes (840 s).

### the eligibility reports are measured by one function

`check` passed the fixture `eligibility-reports-total-limit` from a limit of 4 422 588
and `build` from 4 422 972: the analysis counted the roots of a module and not
the module around them (128 bytes a module). The analysis now measures the
report as `to_file_bytes`, the one function the written file, `build`, and
`check` answer to. `conformance_sp_02` builds the fixture, sums the lengths of
the report files written (the limit at which both pass), and requires `check`
and `build` to refuse 1024, 384, 128, and 1 byte below it and to accept it and
one byte above.

Planted: the report's bytes are the roots' (`budget.used - written_before`, the
old accounting). Command: `cargo test -p repo-conformance --test conformance --
conformance_sp_02`. Expected: the two commands disagree at the limits the
reviewer found.

```text
thread 'conformance_sp_02' (27712) panicked at crates/conformance/src/cases/preservation.rs:183:13:
assertion `left == right` failed: check at max_total_source_bytes 4422588 (4422972 written)
  left: true
 right: false
```

Removed: the measure was restored. The same test leaves `crates/conformance/tests`
without `zz_scratch.rs`, the scratch probe that `cee5c34` committed by mistake: it
asserted nothing and ran every stress family on every `cargo test`. The two
functions it called, `measure_program` and `record_slots`, were public before it
(`assert_bounds` uses them) and stay so.

### the reasoning example is certified like any other

Merging #31 (reasoning machines) made the eight roots of `examples/reasoning`
production-eligible: they return `ReasoningFailure`, which is a pair of Booleans
like `ContractViolation` (`answered`, `replay`), and they call reasoners and
rules that linking elaborates to ordinary definitions. The lowering, the
certificate generator, and the source reader take `ReasoningFailure` with the
same representation (`exhausted`, `unsolved`, `rejected`, `invalid_step` are
`(false, false)`, `(false, true)`, `(true, false)`, `(true, true)`), and read the
four reasoning declarations (`logic`, `inference_rule`, `verifier`, `reasoner`)
only through their elaborations, as they do the model declarations.
`lexlean verify` of the example failed on its first certificate A, twice, for two
defects the older corpora did not reach.

The elaboration of a reasoner names its own locals `__acc`, `__b`, `__found`,
`__s`, `__step`, and the certificates bind names of their own that begin with two
underscores (`__b` for the condition of a conditional, `__a`, `__x0`): under the
condition `__b`, the source local `__b` of `Step.Burn __b` was the Boolean.
A generated source local is now spelled `_y` and the rest in every certificate
(`identifier`); no source name begins with an underscore, so the other spelling
is no one's.

Planted: `identifier` without the `_y` spelling. Command: `lexlean verify` in a
copy of `examples/reasoning`. Expected: certificate A of `spendLeft` is rejected.

```text
error[LLV7013]: certificate A: `LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R7` was rejected in `__rel_14` of root `Reasoning.Main.spendLeft`: error: Application type mismatch: The argument
  __b
has type
  Bool
but is expected to have type
  Nat
in the application
  Reasoning.Budget.Spend.Step.Burn __b
```

The second defect was in a template the older corpora also use: a source `match`
in a generic definition is written as a function over the type parameters whose
body is the `match`, applied to the type arguments. That function is elaborated
before its arguments, so its `match` has no expected type, Lean postpones it, and
the check of a fold's step relation against the step function failed on the
pending metavariable (`(fun T __a __k0 __k1 => ?m.369) Nat __p0 ...`). The
`match` now carries its result type.

Planted: the result type removed from the template's `match`. Command: the same.
Expected: the fold of `spendLeft` is rejected.

```text
error[LLV7013]: certificate A: `LexLeanPreserve.C18ad574fdbd90d6971bb010291acf197.R7` was rejected in `__rel_9` of root `Reasoning.Main.spendLeft`: error: Type mismatch
  __rel_14 _ys __p0 __p1
has type
  LexLeanPreservation.FunRel __prog 14
    [LexLeanPreservation.encPair LexLeanTarget.TargetSyntax.Value.nat LexLeanTarget.TargetSyntax.Value.nat _ys,
      LexLeanPreservation.encOption __enc_0 __p0, LexLeanTarget.TargetSyntax.Value.nat __p1]
```

Removed: both were restored. `examples/reasoning` verifies with its certificates
A, B, and E, in the example's `verify` records, and the conformance suite
certifies its roots with the rest (`conformance_sp_02`, `conformance_sp_03`, the
differential of the four examples); four of its roots declare an overflow that no
input reaches, which SPEC.md lists.

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
(1) thread 'conformance_ex_07' panicked at crates/conformance/src/cases/examples.rs:550:13:
/home/user/wt-25/tests/negative/extraction-uncompiled-external: step 1 `verify ` exited 0, case.toml expects 1
(2) thread 'conformance_ne_03' panicked at crates/conformance/src/cases/extraction.rs:668:18:
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
thread 'conformance_tc_07' panicked at crates/conformance/src/cases/calculus.rs:1379:17:
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
thread 'conformance_tc_07' panicked at crates/conformance/src/cases/calculus.rs:1384:17:
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
thread 'conformance_tc_07' panicked at crates/conformance/src/cases/calculus.rs:1299:29:
binding-and-shapes: rust-core renders a program that needs the heap
```

Removed: the check was restored; `rust-core` renders exactly the fixtures
`realization::program_allocates` says need no heap.

### calculus realization coverage can fail

Planted: the realization row of `primitive.map_size` was deleted from
`crates/lexlean/src/calculus/realization.rs`. Command: `cargo test -p
repo-conformance --test conformance -- conformance_tc_06`.

```text
thread 'conformance_tc_06' panicked at crates/conformance/src/cases/calculus.rs:1177:48:
the realization table: "runtime construct `primitive.map_size` has no realization row"
```

Removed: the row was restored; `conformance_tc_06` passes.

### registry allocation agreement can fail

Planted: `term.nil` in `language/production-1.2.toml` was marked
`allocation = true`, although its realization builds no heap storage.
Command: `cargo test -p repo-conformance --test conformance --
conformance_tc_06`.

```text
thread 'conformance_tc_06' panicked at crates/conformance/src/cases/calculus.rs:1193:13:
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
thread 'conformance_tc_06' panicked at crates/conformance/src/cases/calculus.rs:1247:13:
(primitive, width) pairs no fixture exercises: [("bit_and", "u16"), ("bit_not", "u16"), ("bit_or", "u16"), ("bit_xor", "u16"), ("checked_mul", "u16"), ("checked_quot", "u16"), ("compare", "u16"), ("format_decimal", "u16"), ("parse_decimal", "u16"), ("shift_left", "u16"), ("shift_right", "u16")]
```

Removed: the generator was restored; every admitted pair is exercised.

### calculus progress sampling can fail

Planted: the interpreter took the `succ` shape only for naturals below
`2^63`, so larger naturals matched no arm. No fixture's stated arguments
reach such a value. Command: `cargo test -p repo-conformance --test
conformance -- conformance_tc_03`. Expected: a seeded random argument does.

```text
thread '<unnamed>' (8081) panicked at crates/conformance/src/cases/calculus.rs:43:17:
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
thread 'conformance_rb_01' (22507) panicked at crates/conformance/src/cases/rust_backend.rs:454:40:
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
thread 'conformance_rb_01' (11292) panicked at crates/conformance/src/cases/rust_backend.rs:454:40:
adt-evaluation (rust-std): the construct `call:runtime:nat_mul` does not realize `prim:nat_add`, the element it was lowered from
```

A second plant chose the `u16` checked addition for every `u8` one:

```text
thread 'conformance_rb_01' (12208) panicked at crates/conformance/src/cases/rust_backend.rs:454:40:
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
thread 'conformance_rb_01' (29092) panicked at crates/conformance/src/cases/rust_backend.rs:454:40:
closure-captures (rust-core): the construct `enum:closures` does not realize `expr:closure`, the element it was lowered from
```

Removed: the row was restored; `conformance_rb_01` passes, and
`closure-inline` renders and runs.

### rust identifier collision can fail

Planted: the package check admitted Rust keywords as exported names
(`KEYWORDS.contains(&name) && name.is_empty()`). Command: `cargo test -p repo-conformance --test conformance -- conformance_rb_02`. Expected: the committed negative
manifest that the check exists for is packaged.

```text
thread 'conformance_rb_02' (29709) panicked at crates/conformance/src/cases/rust_backend.rs:109:13:
identifier-keyword: the negative manifest packages
```

Removed: the check was restored; `conformance_rb_02` passes.

### rust ownership mismatch can fail

Planted: the package check let an export copy a parameter of any type
(`Passing::Copy => ...`). Command: `cargo test -p repo-conformance --test conformance -- conformance_rb_03`. Expected: the committed negative
manifest that the check exists for is packaged.

```text
thread 'conformance_rb_03' (30347) panicked at crates/conformance/src/cases/rust_backend.rs:109:13:
ownership-copy-list: the negative manifest packages
```

Removed: the check was restored; `conformance_rb_03` passes.

### rust boundary type check can fail

Planted: the package check never found a function value at an export's
boundary (`Ty::Fn { .. } => false`). Command: `cargo test -p repo-conformance --test conformance -- conformance_rb_04`. Expected: the committed negative
manifest that the check exists for is packaged.

```text
thread 'conformance_rb_04' (31065) panicked at crates/conformance/src/cases/rust_backend.rs:109:13:
unsupported-function-boundary: the negative manifest packages
```

Removed: the check was restored; `conformance_rb_04` passes.

### rust boundary check reaches into records

Planted: the boundary check stopped at a named type
(`false && seen.insert(*index)` in `holds_function`). Command: `cargo test -p repo-conformance --test conformance -- conformance_rb_04`. Expected: the committed negative
manifest that the check exists for is packaged.

```text
thread 'conformance_rb_04' (31880) panicked at crates/conformance/src/cases/rust_backend.rs:109:13:
unsupported-function-in-record: the negative manifest packages
```

Removed: the check was restored; `conformance_rb_04` passes.

### rust uninhabited value refusal can fail

Planted: lowering no longer refused a call whose result type no value
inhabits (the `self.inhabited(&result)?` of `Term::Call` was deleted). Command: `cargo test -p repo-conformance --test conformance -- conformance_rb_04`. Expected: the committed negative
manifest that the check exists for is packaged.

```text
thread 'conformance_rb_04' (32579) panicked at crates/conformance/src/cases/rust_backend.rs:109:13:
unsupported-uninhabited-value: the negative manifest packages
```

Removed: the check was restored; `conformance_rb_04` passes.

### rust declared failure check can fail

Planted: the package check admitted an export declaring no errors for a
function that can overflow (`(Errors::None, true) if false`). Command: `cargo test -p repo-conformance --test conformance -- conformance_rb_05`. Expected: the committed negative
manifest that the check exists for is packaged.

```text
thread 'conformance_rb_05' (759) panicked at crates/conformance/src/cases/rust_backend.rs:109:13:
arithmetic-undeclared-overflow: the negative manifest packages
```

Removed: the check was restored; `conformance_rb_05` passes.

### rust version check can fail

Planted: the version check read each part as a `u128`, so a part above
`u64::MAX`, which Cargo refuses, passed. Command: `cargo test -p repo-conformance --test conformance -- conformance_rb_07`. Expected: the committed negative
manifest that the check exists for is packaged.

```text
thread 'conformance_rb_07' (1325) panicked at crates/conformance/src/cases/rust_backend.rs:109:13:
version-overflow: the negative manifest packages
```

Removed: the check was restored; `conformance_rb_07` passes.

### rust package lint gate can fail

Planted: the generated `Cargo.toml` set Clippy's default lints to `allow`,
and the packages were regenerated. Command: `cargo test -p repo-conformance --test conformance -- conformance_rb_06`. Expected: the package
with a planted clone of a `Copy` value passes Clippy.

```text
thread 'conformance_rb_06' (10515) panicked at crates/conformance/src/cases/rust_backend.rs:975:13:
the planted lint is refused
```

Removed: the generator was restored and the packages regenerated;
`conformance_rb_06` passes.

### rust lint exceptions and lowering rules are load-bearing

Each of the ten exceptions of the package scaffold (the rows of `allowedLints` in
`language/package-1.2/library/src/RustPackage.lex.tex`, which replaced `package::ALLOWED_LINTS`)
was removed in turn by dropping its row from the module, and each rendering rule
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
thread 'conformance_rb_06' (18145) panicked at crates/conformance/src/cases/rust_backend.rs:845:13:
a package fails its lint gate:
error: very complex type used. Consider factoring parts into `type` definitions
```

Removed: every exception and rule was restored and the packages
regenerated; `conformance_rb_06` passes.


### rust package scaffold can fail

The scaffold (`RB-10`..`RB-13`) was made to fail by planting a defect in the
repository itself, running the case, and restoring the file. Each case is
`cargo test -p repo-conformance --test conformance -- conformance_rb_NN`.

| Plant | Case | First refusal |
| --- | --- | --- |
| a manifest constant (`[lints.clippy]` and an `allow`) appended to `package.rs` | `RB-10` | ``the renderer holds the package scaffolding as text beside its module: package.rs: the literal "[lints.clippy] single_match = \"allow\" " states `single_match` `` |
| an allowed lint of the module moved to `correctness` | `RB-11` | ``allows a lint in Clippy's `correctness` group (§17.16)`` |
| the `single_match` row dropped from the module | `RB-11` | ``the specification's `single_match` is not the module's`` |
| the reader's refusal of `correctness` disabled | `RB-11` | the module with a `correctness` row decodes, and the case's `expect_err` fires |
| the recursion statement of `language/production-1.2.toml` set back to "explicit failure" | `RB-13` | the statement is printed, and the case demands `aborts the process` |
| `language/package-1.2/` removed from the files introduced for 1.2 | `RB-12` | `assertion left == right failed` on the language-1.0 identity: the scaffold is hashed into it |

`RB-10` also plants `publish = false`, a key `all = ...`, and `priority = -1`
as string literals of three renderer files (the audit reads the Boolean and
the priority as the text a printer would write), and `RB-11` checks the
module's project (`language/package-1.2/library`) by locking and checking it,
and refuses a copy whose theorem equates the predicate with `3`. `RB-13` no
longer claims that production states the root's effects or that replay binds
them: `PX-01` compares each bundle's `provenance.json` effects with the
result's, a check that needs a verified export and has no recorded plant.

The same defects are planted inside the cases for what a plant in the
repository cannot reach: a module whose declaration, constructor, field, or
body shape differs (`RB-10`), a table row changed, dropped, or added on either
side and a theorem removed or weakened (`RB-11`), the edition, a lint row, and
a gate changed in a copy of the embedded tree, which must move the language-1.2
ID alone (`RB-12`), and effects that are unregistered, unordered,
or state allocation on `rust-core` (`RB-13`). The ten exceptions are each
removed by dropping their row of the module in the table above.

A first version of `RB-13` also made the renderer refuse a list that omitted
an effect the package's program shows (a function that reaches itself, an
export that can overflow). Run over every committed export with the effects of
the root's eligibility report, it refused `Coverage.Boundary.groveRoot`
(`recursion`) and `Coverage.Colls.graphs` and `stringGraph` (`overflow`): the
entry of a root carries a generated check of its arguments, which recurses over
a recursive type such as `Grove`, and the graph templates' `nat_add` fuel
bound can overflow, neither of
which the root's eligibility reports. The renderer cannot decide what the
registry means, so the check was removed and the list stays the root's, as
eligibility decides; `shown_effects` remains only to state the fixtures'.
That eligibility omits the recursion of the argument check (a stack demand of
the input's depth) is a finding for #105 and not changed here.

Cases that compare to regenerated records (`RB-02`..`RB-05`, `RB-07`, and the
cases that read a lock, such as `AU-02`, `AU-03`, and `AU-10`..`AU-12`) fail on
the hand-written tree alone, because the language-1.2 ID and every package's
`package.json` and `provenance.json` move. `RB-02`..`RB-05` and `RB-07` passed
on a scratch copy whose Rust packages were rewritten by the writer of
`check-calculus` (`rust_packages::files`): every `Cargo.toml` and `src/lib.rs`
was byte-identical to the committed one, and only `package.json`,
`provenance.json`, and the negative manifests changed. The regeneration of the
locks and the remaining records is the train's.

### rust runtime mutation is detected by the differential

Planted: `parse_int` returned `Ok(None)` for a decimal outside `i64`
(`Some(None) => Ok(None)` in the runtime). Command: `cargo test -p repo-conformance --test conformance -- conformance_rb_06`. Expected: the
primitive differential, which runs every primitive instance on its
boundary and seeded inputs, finds the inputs whose denotation overflows.

```text
thread 'conformance_rb_06' (2037) panicked at crates/conformance/src/cases/rust_backend.rs:902:17:
primitives_std: the rendering and the denotation disagree on 3 of 51441 runs: ["run 38634: {\"kind\":\"none\"} != {\"kind\":\"overflow\"}", "run 38650: {\"kind\":\"none\"} != {\"kind\":\"overflow\"}", "run 38656: {\"kind\":\"none\"} != {\"kind\":\"overflow\"}"]
```

A second plant computed `int_rem` as `a.checked_rem(b).unwrap_or(z)`, so
`i64::MIN rem -1` returned the default:

```text
thread 'conformance_rb_06' (17475) panicked at crates/conformance/src/cases/rust_backend.rs:902:17:
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
thread 'conformance_rb_06' (3563) panicked at crates/conformance/src/cases/rust_backend.rs:829:13:
a package does not build:
error[E0308]: arguments to this function are incorrect
```

and for the projection:

```text
thread 'conformance_rb_06' (6453) panicked at crates/conformance/src/cases/rust_backend.rs:829:13:
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
thread 'conformance_rb_07' (28593) panicked at crates/conformance/src/cases/rust_backend.rs:1119:17:
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
thread 'conformance_rb_07' (13568) panicked at crates/conformance/src/cases/rust_backend.rs:1069:17:
assertion `left == right` failed: rust-core/boolean-shapes/provenance.json: the renderers of two roots disagree
```

Removed: the provenance was restored and the packages regenerated;
`conformance_rb_07` passes.

### rust source binding can fail

Planted: the package generator wrote the zero identity as every package's
`sources`, and the packages were regenerated. Command: `cargo test -p repo-conformance --test conformance -- conformance_rb_07`. Expected: no
package binds the verified build that states its program.

```text
thread 'conformance_rb_07' (30490) panicked at crates/conformance/src/cases/rust_backend.rs:1126:46:
compiler/rust/rust-std/adt-evaluation: sources ["0000000000000000000000000000000000000000000000000000000000000000"] are not the semantic ID cd690f0a5afba87d8ddeebfb51773d2d833d2d9b0f897cc9c5702224d8b1994d of the verified compiler build
```

A second plant made the binding check accept any single source
(`sources.len() != 1`); the forged-identity check refused it:

```text
thread 'conformance_rb_07' (19913) panicked at crates/conformance/src/cases/rust_backend.rs:1139:69:
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
thread 'conformance_rb_01' (10406) panicked at crates/conformance/src/cases/rust_backend.rs:454:40:
boolean-shapes (rust-core): a negated zero test or predecessor: a zero test is negated as its complement, and a predecessor is not a Boolean
```

Removed: `negate` was restored; both tests pass.

### rust last-use moves can fail

Planted: the lowering no longer ran `rust::moves::apply` on a function body
(the call in `crates/lexlean/src/calculus/rust/lower.rs` commented out), so
every read was a clone again. Command: `cargo test -p repo-conformance --test
conformance -- conformance_rb_08`. Expected: a clone the rendering keeps is
one the ownership check does not require, so making it a move is accepted.

```text
thread 'conformance_rb_08' (13002) panicked at crates/conformance/src/cases/rust_backend.rs:1269:29:
adt-evaluation/rust-std: clone 0 is kept but a move of it is accepted
```

Removed: the call was restored; `conformance_rb_08` passes, with more moves
than clones over every fixture in every profile.

### rust component moves are tracked

Planted: `consume_parts` in `crates/lexlean/src/calculus/rust/validate.rs`
recorded the moved components in a set of its own instead of the checker's
`parted`, so a binding stayed readable whole after a component left it.
Command: `cargo test -p repo-conformance --test conformance --
conformance_rb_08`. Expected: a projection clone the rendering keeps because
the pair is read afterwards is, made a move, no longer refused.

```text
thread 'conformance_rb_08' (13579) panicked at crates/conformance/src/cases/rust_backend.rs:1269:29:
binding-and-shapes/rust-std: clone 0 is kept but a move of it is accepted
```

Removed: the record was restored; `conformance_rb_08` passes, and the three
planted crates of the case (a read, a move, and a second component move after
a component left the pair) are refused naming ownership.

### rust list runs respect sharing

Planted: `cons` in the `rust-std` runtime extended a run's buffer at its
end rather than at the run's own length (`cells.truncate(len)` removed), so
a list whose only holder had taken cells off it was built on top of the
cells it no longer had; the packages were regenerated. Command: `cargo test
-p repo-conformance --test conformance -- conformance_rb_08`. Expected: the
list harness sees the stale cell instead of the one it built.

```text
thread 'conformance_rb_08' (21836) panicked at crates/conformance/src/cases/rust_backend.rs:1490:13:
the list runtime:
thread 'main' (21955) panicked at h/run_lists_std/src/main.rs:23:5:
assertion `left == right` failed: a run extended by its only holder keeps only its own cells
  left: [3, 2, 1]
 right: [99, 2, 1]
```

A first version of the harness built on a tail two other lists still held,
which is the cell path and not the in-place one, and did not reach the
plant; the harness now takes a cell off a list it holds alone. Removed: the
truncation was restored and the packages regenerated; `conformance_rb_08`
passes, and `conformance_tc_07` holds every fixture's counted work within
the denotation's steps with the new runtime.

### certificate budgets cover a large program

Planted: nothing in this repository; the defect was observed on
`UOR-Foundation/MNIST` at the LexLean of `d284b165`, whose model root
`Mnist.Model.classify` (the network under its contract, its parameters ten
`int_tensor` artifacts) renders to a 242,624-byte crate. Command:
`lexlean verify --all` in that project. Expected before the change: the
certificate is valid and the pinned Lean accepts it. Observed:

```text
error[LLV7015]: certificate B: `LexLeanPreserve.Cfee62143c6d631437aa7081b9c317efa.R0.RustStd` was rejected in `krate` of root `Mnist.Model.classify`: error: (deterministic) timeout at `«LCNF compiler»`, maximum number of heartbeats (200000) has been reached
```

With `program` and `krate` declared `noncomputable` (nothing compiles the
data of the statement) the same project failed one function later:

```text
error[LLV7015]: certificate B: `LexLeanPreserve.Cfee62143c6d631437aa7081b9c317efa.R0.RustStd` was rejected in `fun37` of root `Mnist.Model.classify`: error: (deterministic) timeout at `isDefEq`, maximum number of heartbeats (200000) has been reached
```

With both budgets lifted for every certificate, as they were for a match of
more than 100 arms, the project verifies (attestation `10a88470…`, the
certificates A, B and E of the root replayed and audited). `conformance_sp_02`
now asserts that the certificates of a 2-arm, a 100-arm and a 101-arm program
all lift both budgets and that certificate B's two terms are `noncomputable`;
reverting either half of the change fails it:

```text
thread 'conformance_sp_02' panicked at crates/conformance/src/cases/preservation.rs:
2 arms: `LexLeanPreserve.C….R0` lacks `set_option maxHeartbeats 0
`
```

### rust runtime library boundary can fail

Planted: the audit read its own table instead of the runtime text (`let
used = rows.keys()...` in `crates/lexlean/src/calculus/rust_library.rs`).
Command: `cargo test -p repo-conformance --all-features --test conformance
conformance_rb_09`. Expected: a runtime with a library function the table
does not list is accepted.

```text
thread 'conformance_rb_09' (28380) panicked at crates\conformance\src\cases\rust_backend.rs:1518:62:
the runtime uses the method `wrapping_add`: ()
```

A second plant deleted the row of `saturating_sub` from `LIBRARY`:

```text
thread 'conformance_rb_09' (31736) panicked at crates\conformance\src\cases\rust_backend.rs:1504:29:
the runtime keeps to its library table: "the runtime uses the method `saturating_sub`, which the library table does not list"
```

Removed: the scan and the row were restored; `conformance_rb_09` passes.
`RB-09` also plants a path, a macro, and a type outside the table, a removed
row, an unused row, a row citing a non-primitive, and an uncited primitive.


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
thread 'conformance_gn_04' panicked at crates/conformance/src/cases/gnaf.rs:1062:9:
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
thread 'conformance_gn_04' panicked at crates/conformance/src/cases/gnaf.rs:1113:9:
assertion `left == right` failed: Fixed { plan: 0 }
```

Removed: the reachable plans were restored; `conformance_gn_04` passes.

### GNAF prepared-artifact binding can fail

Planted: the host's `check_declared` admitted a `free` preparation on a
prepared boundary whether or not an artifact it prepared is bound
(`(!bound).then_some(..)` replaced by `None`). Command: `cargo test -p
repo-conformance --test conformance -- conformance_gn_04`.

```text
thread 'conformance_gn_04' panicked at crates/conformance/src/cases/gnaf.rs:1062:9:
assertion `left == right` failed: reject-unbound-prepared-state
```

Removed: the binding rule was restored; `conformance_gn_04` passes.

### GNAF operand-size declaration can fail

Planted: the host's `validate` skipped the operand-size rule
(`if false && request.machine.operand_size == OperandSize::Unit`). Command:
`cargo test -p repo-conformance --test conformance -- conformance_gn_04`.

```text
thread 'conformance_gn_04' panicked at crates/conformance/src/cases/gnaf.rs:1062:9:
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
thread 'conformance_gn_04' panicked at crates/conformance/src/cases/gnaf.rs:1062:9:
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
thread 'conformance_gn_05' panicked at crates/conformance/src/cases/gnaf.rs:1199:9:
assertion `left == right` failed: reject-restricted-universe-alias
```

Removed: the alias rule was restored; `conformance_gn_05` passes.

### GNAF evaluation panic can fail

Planted: a panic of the evaluation thread was reported as the platform's
`LLV7010`. Command: `cargo test -p lexlean --lib gnaf`. Expected: the
planted panic surfaces as a platform failure.

```text
thread 'gnaf::tests::a_panicking_evaluation_is_an_internal_failure' (28684) panicked at crates/lexlean/src/gnaf.rs:1778:9:
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
thread 'conformance_gn_07' panicked at crates/conformance/src/cases/gnaf.rs:1542:5:
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
thread 'conformance_gn_08' panicked at crates/conformance/src/cases/gnaf.rs:1831:5:
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
thread 'conformance_sm_30' panicked at crates/conformance/src/support.rs:2031:10:
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
thread 'conformance_sm_28' panicked at crates/conformance/src/support.rs:2031:10:
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
thread 'conformance_sm_28' panicked at crates/conformance/src/support.rs:2031:10:
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

### audit-deferral reads the whole repository and the prose markers

Found by reading the gate's roots, not by a failure: `audit-deferral` did not read the root documents (`README.md`, `SPEC.md`, `CHANGELOG.md`, `VERIFICATION.md`, `AGENTS.md`), `.github`, `.devcontainer`, `api/`, `oracle/`, or `tooling/`, and its markers were only the code-style ones, so prose that put work in the future passed. It now reads all of them, and the phrases `not yet`, `later release`, `in a later`, `eventually`, and `follow in` (spelled in halves in the gate) are markers. A vendored authority copy is the one file not read: a text pinned by digest (R6) is the authority's words, not ours, and the path is taken from the row that vendors it. Widening it found 25 occurrences, each reworded to a present fact; `SPEC.md` §10.5 lost its promise of a release to come, and the `AU-11` statement changed `a reference to a node not yet written` to `a forward reference to a node` in the register, SPEC.md §31, and the scenario.

Planted, one at a time, each spelled here in halves: `not` + ` yet` in a sentence appended to `README.md`, and `event` + `ually` in a comment appended to `.github/ci-plan.sh`. Command: `cargo xtask validate-model`.

```text
gate failed: R4: nothing is deferred. None of TODO, FIXME, XXX, unimplemented!, todo!, for now, later version, not yet, later release, in a later, eventually, follow in may appear outside a code span.

README.md:227: The export is not yet supported.
```

```text
.github/ci-plan.sh:92: # a fix eventually
```

Removed: both files were restored; `audit-deferral` reports nothing deferred.

### the devcontainer pin row (RP-16) can fail

Found by the #34 audit: the devcontainer hard-codes its toolchain versions and no gate compared them with `rust-toolchain.toml` or `lean-toolchain`. `conformance_rp_16` reads the Dockerfile's installs and also runs the same checker on six planted Dockerfiles in memory (another Rust version, another Lean version, a second Lean toolchain, a second Rust toolchain, an unpinned Lean default, an unpinned Rust default), each of which must be refused.

Planted, on the committed file one at a time: `--default-toolchain 1.97.1` changed to `1.97.0` in `.devcontainer/Dockerfile`, then `leanprover/lean4:v4.32.1` changed to `leanprover/lean4:v4.32.0`. Command: `cargo test -p repo-conformance --test conformance conformance_rp_16 -- --exact`.

```text
the devcontainer installs exactly the pinned toolchains: "Rust toolchains installed [\"1.97.0\"], pinned 1.97.1"
the devcontainer installs exactly the pinned toolchains: "Lean toolchains installed [\"leanprover/lean4:v4.32.0\"], pinned leanprover/lean4:v4.32.1"
```

Removed: the Dockerfile was restored and the case passes. SPEC.md §9.3 now says what the container does (the image installs the pins at build) instead of a post-create command it never had.

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
thread 'release::tests::the_local_sha256_agrees_with_the_test_vectors' panicked at crates/model/src/release.rs:562:9:
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

Planted: the `EX-08` scenario removed from `features/suites/examples.feature`. Command: `cargo test -p repo-conformance --all-features` (the meta-gate target alone, which the honesty workflow runs, is `--test bdd`). Expected: R3 reports the registered ID with no scenario.

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

### release-check's schema criterion can fail, and no longer fails falsely

Found by `cargo xtask release-check` on `main` at `724a55590`: it reported eight schemas (`target-fixture`, `target-program`, `gnaf-request`, `gnaf-fixture`, `gnaf-certificate`, `gnaf-certificate-fixture`, `rust-package`, `rust-provenance`) as validated by no conformance case, although the calculus, GNAF, and certification cases read and validate them. The criterion searched the conformance sources for the bare name in quotes; those cases spell the file name (`"target-fixture.schema.json"`). It now accepts either spelling.

Planted: `schema("target-fixture.schema.json")` in `crates/conformance/src/cases/calculus.rs` changed to `schema("target-fixtureX.schema.json")`. Command: `cargo xtask release-check`.

```text
  schemas-exercised: schemas/target-fixture.schema.json is validated by no conformance case
```

Removed: the file was restored; with the criterion fixed, `release-check` reports no `schemas-exercised` line, and the remaining refusals are the `1.0.0` and `release/` ones above.

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
thread 'conformance_md_06' (23724) panicked at crates/conformance/src/support.rs:583:5:
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
thread 'conformance_md_07' (16949) panicked at crates/conformance/src/support.rs:432:14:
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
thread 'conformance_md_02' (13418) panicked at crates/conformance/src/support.rs:583:5:
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
thread 'conformance_md_08' (14466) panicked at crates/conformance/src/support.rs:432:14:
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
thread 'conformance_md_04' (15034) panicked at crates/conformance/src/support.rs:583:5:
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
thread 'conformance_md_09' (15625) panicked at crates/conformance/src/support.rs:2031:10:
the module verifies with real Lean: LexLeanError { class: Language, diagnostics: [Diagnostic { code: DiagnosticCode("LLV7002"), message: "Lean rejected `Models.Main` (error): Tactic `decide` proved that the proposition\n  Tie Sample.s0 = 1\nis false", ...
thread 'conformance_md_09' (18279) panicked at crates/conformance/src/support.rs:2031:10:
the module verifies with real Lean: LexLeanError { class: Language, diagnostics: [Diagnostic { code: DiagnosticCode("LLV7002"), message: "Lean rejected `Models.Main` (error): Tactic `decide` proved that the proposition\n  Logits Sample.s0 = [50075, 63645, 87054, -4700, 27212]\nis false", ...
thread 'conformance_md_09' (18907) panicked at crates/conformance/src/support.rs:2031:10:
the module verifies with real Lean: LexLeanError { class: Language, diagnostics: [Diagnostic { code: DiagnosticCode("LLV7002"), message: "Lean rejected `Models.Main` (error): Tactic `decide` proved that the proposition\n  Logits Sample.s0 = [50075, 63645, 87054, -4700, 27212]\nis false", ...
thread 'conformance_md_09' (19553) panicked at crates/conformance/src/support.rs:2031:10:
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
(c) thread 'conformance_md_02' (29254) panicked at crates/conformance/src/support.rs:432:14:
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
thread 'conformance_md_06' (21466) panicked at crates/conformance/src/support.rs:502:14:
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

### IEEE-754 dyadic decoding and scale-ratio derivation can fail (Issue #118)

`MD-13` was registered before its named test failed on the missing corpus
declarations:

```text
thread 'conformance_md_13' (1417488) panicked at crates/conformance/src/cases/models.rs:3284:9:
Models.Recognizer declares `Dyadic`
test conformance_md_13 ... FAILED
```

Planted, one at a time in a focused copy of `examples/models/src/Recognizer.lex.tex`:

- the normal `binary32` exponent offset `150` (`biased - 127 - 23`) replaced by
  `149` in `decodeBinary32`, which still links cleanly (`lexlean check` exits
  `0`) because the result remains an `Option Dyadic`:

  ```text
  error[LLV7002]: Lean rejected `Models.Recognizer` (error): Tactic `decide` proved that the proposition
    DyadicMatches (decodeBinary32 8388608) 0 8388608 (-149)
  is false
  ```

- the tie-to-even rounding branch
  `(prim subtract q (prim negate (prim remainder q (2 : Int) (0 : Int) : Int) : Int) : Int)`
  in `dyadicRatioRound` replaced by unconditional round-up
  `(prim subtract q (-1 : Int) : Int)`, which still links cleanly and agrees on
  every non-tie and odd-tie case, but rounds an exact tie with even quotient up
  instead of keeping it even:

  ```text
  error[LLV7002]: Lean rejected `Models.Recognizer` (error): Tactic `decide` proved that the proposition
    scaleRatioRound32 1065353219 1069547520 1140850688 = Option.some 12582916
  is false
  ```

- a seeded scale-ratio expectation skewed by `+1` against the independent
  `u128` rational oracle in `conformance_md_13`:

  ```text
  error[LLV7002]: Lean rejected `Models.Recognizer` (error): Tactic `decide` proved that the proposition
    scaleRatioRound32 1047092574 969894013 1000834620 = some 75739599
  is false
  ```

`conformance_md_13` plants all three mutations in `Recognizer.lex.tex`,
asserting that `check` succeeds and `verify` fails with `LLV7002` on each, and
verifies 40 seeded `binary32`, `binary64`, and `round(a * b / c * 2^32)`
theorems against the independent exact rational oracle in addition to the
committed `examples/models` corpus.

Removed: the committed `examples/models` corpus is unchanged; `conformance_md_01`..`conformance_md_13` pass.

### reasoning rule priority order can fail

Planted: the leaves of `E.select` were combined in reverse of the declared order (`leaves.into_iter().rev()` before `balanced` in `elaborate_forward`), so the last rule is tried first. Command: `cargo test -p repo-conformance --test conformance -- conformance_rs_05`. Expected: a forward reasoner that fires rules out of their declared priority order. The case reads the order the select term evaluates its guards in (a `match` scrutinee before its branches, a `let` value before its body), not document order, because canonical JSON orders object keys alphabetically.

```text
thread 'conformance_rs_05' (4138) panicked at crates/conformance/src/cases/reasoning.rs:892:5:
assertion `left == right` failed: select tries the rules in declared priority order
  left: ["Escalate.guard", "Shock.guard", "Sepsis.guard", "Sirs.guard", "Leukocytosis.guard", "Tachypnea.guard", "Tachycardia.guard", "Fever.guard"]
 right: ["Fever.guard", "Tachycardia.guard", "Tachypnea.guard", "Leukocytosis.guard", "Sirs.guard", "Sepsis.guard", "Shock.guard", "Escalate.guard"]
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 303 filtered out; finished in 0.81s
```

Removed: the reversal was restored; `conformance_rs_05` passes, and its Lean-backed half also requires the reversed order to be refused by verification.

### reasoning firing without the guard can fail

Planted: each arm of `E.fire` applied the rule's conclusion directly (`some (N.conclusion s)`) instead of its guarded application. Command: `cargo test -p repo-conformance --test conformance -- conformance_rs_05`. Expected: a reasoner step that applies a rule whose guard does not hold.

```text
thread 'conformance_rs_05' (5662) panicked at crates/conformance/src/cases/reasoning.rs:903:5:
assertion `left == right` failed: every step fires through its rule's guarded application
  left: ["Fever.conclusion", "Tachycardia.conclusion", "Tachypnea.conclusion", "Leukocytosis.conclusion", "Sirs.conclusion", "Sepsis.conclusion", "Shock.conclusion", "Escalate.conclusion"]
 right: ["Fever.apply", "Tachycardia.apply", "Tachypnea.apply", "Leukocytosis.apply", "Sirs.apply", "Sepsis.apply", "Shock.apply", "Escalate.apply"]
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 303 filtered out; finished in 0.95s
```

Removed: the guarded application was restored; `conformance_rs_05` passes.

### reasoning guard evaluations can go unaccounted

Planted: a forward step's ledger kept its `attempts` counter instead of adding `E.attempts` of the state it scanned. Command: `cargo test -p repo-conformance --test conformance -- conformance_rs_05`. Expected: a ledger that charges firings but not the rules tried.

```text
thread 'conformance_rs_05' (7028) panicked at crates/conformance/src/cases/reasoning.rs:908:5:
assertion failed: calls(&derived(triage,
                    "Triage.step")["body"]).contains(&"Triage.attempts".to_owned())
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 303 filtered out; finished in 0.84s
```

Removed: the count was restored; `conformance_rs_05` passes.

### reasoning runtime boundary can fail

Planted: `check_boundary` no longer refused a guarded member (`env.reasoning.guarded(&key).filter(|_| false)` in `model.rs`). Command: `cargo test -p repo-conformance --test conformance -- conformance_rs_09`. Expected: executable code reaching a rule's conclusion or an unverified answer links.

```text
thread 'conformance_rs_09' (8223) panicked at crates/conformance/src/support.rs:465:14:
check fails
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 303 filtered out; finished in 0.26s
```

Removed: the refusal was restored; `conformance_rs_09` passes.

### reasoning answer correctness erasure can fail

Planted: `E.accept` always ran the verifier's check (`match &None::<MemberRef>` for the claim in `elaborate_common`), so an `answer_correct` claim erased nothing. Command: `cargo test -p repo-conformance --test conformance -- conformance_rs_09`. Expected: a check proved unnecessary still runs, so the claim states a runtime effect it does not have.

```text
thread 'conformance_rs_09' (16575) panicked at crates/conformance/src/cases/reasoning.rs:1591:5:
assertion `left == right` failed
  left: ["gradedCheck", "Grade.extract"]
 right: ["Grade.extract"]
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 303 filtered out; finished in 2.60s
```

Removed: the erasure was restored; `conformance_rs_09` passes.

### generate-and-verify budget is checked by Lean

Planted: `E.try` checked a candidate while fewer than the budget plus one had been checked, so a generate-and-verify reasoner could exceed its budget. Command: `cargo test -p repo-conformance --test conformance -- conformance_rs_06`. Expected: the generated `E.try_count` and `E.verifications_bounded` theorems are refused by pinned Lean.

```text
thread 'conformance_rs_06' (10047) panicked at crates/conformance/src/cases/reasoning.rs:284:14:
the reasoning example verifies under pinned Lean: LexLeanError { class: Language, diagnostics: [Diagnostic { code: DiagnosticCode("LLV7002"), message: "Lean rejected `Reasoning.Screening` (error): Application type mismatch: The argument\n  llB\nhas type\n  __t.ledger.verifications.blt (3 + 1) = true\nbut is expected to have type\n  __t.ledger.verifications.blt 3 = true ...
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 303 filtered out; finished in 20.39s
```

Removed: the bound was restored; `conformance_rs_06` passes.

### generic reasoner elaboration can fail

Planted: a rule-based reasoner's elaboration was no longer generalized (the `generalize` call in `check_reasoner` was removed), so a generic reasoner's declarations would mention its type parameters unbound. Command: `cargo test -p repo-conformance --test conformance -- conformance_rs_04`. Expected: the generic `Spend` of the example is refused in linking.

```text
thread 'conformance_rs_04' (7644) panicked at crates/conformance/src/cases/reasoning.rs:149:10:
snapshot: LexLeanError { class: Language, diagnostics: [Diagnostic { code: DiagnosticCode("LLT4001"), message: "phase link: reasoner declaration `Spend` elaborates to `Spend.observe`, which is ill-formed: unbound type parameter `T`", ...
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 303 filtered out; finished in 0.31s
```

Removed: the call was restored; `conformance_rs_04` passes.

### reasoning eligibility rows can be dropped

Planted: the eligibility report omitted every root's `reasoning` rows (`if false && !root.reasoning.is_empty()` in `production/mod.rs`). Command: `cargo test -p repo-conformance --test conformance -- conformance_rs_11`. Expected: a production root whose search resources are unreported.

```text
thread 'conformance_rs_11' (8812) panicked at crates/conformance/src/cases/reasoning.rs:2456:32:
Reasoning.Main.triageLevel records its reasoning
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 303 filtered out; finished in 0.73s
```

Removed: the rows were restored; `conformance_rs_11` passes.

### a calculus transcription that skips guard accounting can fail

Planted: the traced transcription of `Triage` counted no guard evaluation per step (`nat(0)` for the step's attempts in `calculus/reasoning.rs`). Command: `cargo test -p repo-conformance --test conformance -- conformance_rs_12`. Expected: a transcription whose account charges firings it never searched for.

```text
thread 'conformance_rs_12' (9480) panicked at crates/conformance/src/cases/reasoning.rs:3095:5:
every firing follows a guard evaluation
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 303 filtered out; finished in 0.17s
```

Removed: the count was restored; `conformance_rs_12` passes. The case also plants, in itself, a transcription answering without searching, and requires its steps to fall below the account.

### a reasoning capability without its scenario can fail

Planted: `features/suites/reasoning.feature` was moved away. Command: `cargo test -p repo-conformance --test bdd`. Expected: R3 names every RS row that has no scenario.

```text
thread 'every_id_has_a_scenario_and_a_test' (10919) panicked at crates/conformance/tests/bdd.rs:31:5:
R3: RS-01 is registered but has no scenario in features/suites/.
R3: RS-02 is registered but has no scenario in features/suites/.
```

Removed: the file was restored; the BDD gate passes.

### a mutated reasoning runtime lemma and a planted rule threshold are refused by Lean

`conformance_rs_08` plants, in itself, a mutated `LexLeanReasoning.guarded` statement (`r t s` for `r s t`) in a standalone copy of the emitted runtime and requires pinned Lean to refuse it, after requiring the unmutated runtime to be accepted with every name reported axiom-free by `#print axioms`. `conformance_rs_14` plants, in a copy of `examples/reasoning`, the tachycardia threshold 80 for 90: linking accepts it, and verification refuses it with `LLV7002`, because the `Tachycardia` rule would then conclude a finding its relation does not justify. Both plants are inside the tests, so there is no failing test run to record; the refusals they require are these, produced by running the same two mutations by hand:

```text
Mut.lean:475:30: error: Type mismatch
  h rfl
has type
  r s c
but is expected to have type
  r c s
error[LLV7002]: Lean rejected `Reasoning.Clinic` (error): omega could not prove the goal:
a possible counterexample may satisfy the constraints
  81 ≤ a ≤ 90
where
 a := ↑s.vitals.heartRate
  --> src/Clinic.lex.tex:6:22144
```

### reasoning elaboration nesting can fail

Planted: `balanced` combined its items as a right-nested chain (`items.split_at(1)` for `items.split_at(items.len() / 2)` in `reasoning.rs`), so the elaboration of a reasoner nests as deeply as it has rules. Command: `cargo test -p repo-conformance --test conformance -- conformance_rs_10`. Expected: a reasoner of 602 rules nests as many levels, and every pass that recurses on a term does too.

```text
thread 'conformance_rs_10' (1093) has overflowed its stack
fatal runtime error: stack overflow, aborting
error: test failed, to rerun pass `-p repo-conformance --test conformance`
  process didn't exit successfully: `target/debug/deps/conformance-ca277230fee88893 conformance_rs_10 --quiet` (signal: 6, SIGABRT: process abort signal)
```

Removed: the balanced combination was restored; `conformance_rs_10` passes (the same case asserts the depth of `E.select`, `E.attempts`, and `E.successors` is at most 80).

### reasoning elaboration charge can fail

Planted: the charge of a reasoner (`charge(artifacts, ..., bound)?` in `check_reasoner`) was removed, so a reasoner's elaboration is built before any limit is consulted. Command: `cargo test -p repo-conformance --test conformance -- conformance_rs_10`. Expected: `tests/negative/reasoning-elaboration-over-limit`, whose limit its elaboration would exceed, links.

```text
thread 'conformance_rs_10' (31910) panicked at crates/conformance/src/support.rs:475:14:
check fails
```

Removed: the charge was restored; `conformance_rs_10` and EX-07 pass.

### reasoning elaboration bounds can fail

Planted, one at a time, in `reasoning.rs`: the binding term of a reasoner's bound multiplied by 0, its weight multiplier lowered from 40 to 1, and a rule's weight multiplier lowered from 12 to 1. Command: `cargo test -p repo-conformance --test conformance -- conformance_rs_10`. Expected: an elaboration larger than what it was charged is an internal error, which the wide-state, wide-binding, and many-rule projects of the case provoke.

```text
thread 'conformance_rs_10' (11690) panicked at crates/conformance/src/support.rs:465:14:
check succeeds: ... "phase link: internal: reasoner `E` elaborated to 5414 nodes, beyond the 5340 it was charged" ...
thread 'conformance_rs_10' (7117) panicked at crates/conformance/src/support.rs:465:14:
check succeeds: ... "phase link: internal: reasoner `E` elaborated to 12395 nodes, beyond the 10859 it was charged" ...
thread 'conformance_rs_10' (12573) panicked at crates/conformance/src/support.rs:465:14:
check succeeds: ... "phase link: internal: inference rule `R0` elaborated to 1302 nodes, beyond the 683 it was charged" ...
```

Removed: the constants were restored; `conformance_rs_10` passes.

### reasoning link-time checks can fail

Planted, one at a time, in `reasoning.rs`, then `cargo xtask check-fixtures` (the negative fixtures are the gate): exactness of every generated obligation (`require_obligation` returned `Ok`); a rule over another logic than its reasoner's; two rules of one name; a literal zero frontier; deduplication over an unordered state; a termination claim by a rule without progress, without the initial invariant it assumes, and by a search; a non-executable verifier check; a non-executable rule in an executable reasoner; claims out of order. Expected: the fixture naming each rule stops being refused.

```text
gate failed: tests/negative/reasoning-answer-correct-inexact: step 1 `check ` exited 0, case.toml expects 1
gate failed: tests/negative/reasoning-rule-foreign-logic: step 1 `check ` exited 0, case.toml expects 1
gate failed: tests/negative/reasoning-duplicate-rule/expected/command.json differs from the observed run
gate failed: tests/negative/reasoning-zero-frontier: step 1 `check ` exited 0, case.toml expects 1
gate failed: tests/negative/reasoning-deduplicate-unordered-state/expected/command.json differs from the observed run
gate failed: tests/negative/reasoning-terminates-without-progress: step 1 `check ` exited 0, case.toml expects 1
gate failed: tests/negative/reasoning-terminates-without-initial: step 1 `check ` exited 0, case.toml expects 1
gate failed: tests/negative/reasoning-terminating-search: step 1 `check ` exited 0, case.toml expects 1
gate failed: tests/negative/reasoning-verifier-formal-check/expected/command.json differs from the observed run
gate failed: tests/negative/reasoning-formal-rule/expected/command.json differs from the observed run
gate failed: tests/negative/reasoning-answer-correct-unsorted: step 1 `check ` exited 0, case.toml expects 1
```

Removed: every check was restored; the fixtures pass.

### reasoning boundary can fail through a function value

Planted: `check_boundary` refused a guarded member only when it was the callee of a `call` (`env.reasoning.guarded(&key).filter(|_| matches!(node, SemanticTerm::Call { .. }))` in `model.rs`), so a function value naming `N.conclusion` or `E.extract` was not refused. Command: `cargo test -p repo-conformance --test conformance -- conformance_rs_09`. Expected: `tests/negative/reasoning-rule-bypass-ref` (a `list_fold` whose step is `function_ref Pick.conclusion`), `-alias` (a name bound to one), `-iterate`, and `reasoning-extract-bypass-ref` and `-alias` link. The other positions the fixtures cover (a lambda body, a branch, a rule guard, a reasoner's observation or generator, another reasoner's answer) reach the guarded member through a `call`, so the plant leaves them refused.

```text
thread 'conformance_rs_09' (24786) panicked at crates/conformance/src/support.rs:475:14:
check fails
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 303 filtered out; finished in 0.35s
```

Removed: the filter was removed; `conformance_rs_09` passes.

### reasoning claims, type arguments, verifier types, and reserved names can fail

Planted, one at a time in `reasoning.rs` and `semantic.rs`: duplicate claim kinds accepted (`prior >= rank` for `prior > rank` in `sorted_claims`); a rule used at other type arguments than its reasoner's logic (the comparison of `rule_at.logic.type_arguments` replaced by `false`); the verifier's subject type, and then its candidate type, no longer compared with the reasoner's observation and answer; the initial invariant no longer required of an answer claimed correct under the invariant (`if initial.is_none()` for `if false`); and a declaration named below `LexLeanReasoning` no longer refused (`if env.language_1_2 && BACKEND_BARE_NAMES` for `if false && ...`). Commands: `conformance_rs_07` (claims), `conformance_rs_02` (type arguments), `conformance_rs_03` (verifier types), `conformance_rs_09` (the initial invariant), `conformance_rs_01` (names). Expected: each is caught by the fixture that names it; for the type arguments and the verifier types, a later pass would still refuse the project, with another code (`LLT4001`, "ill-formed elaboration"), and the fixtures pin `LLT4010` and the message.

```text
conformance_rs_07: thread 'conformance_rs_07' (25125) panicked at crates/conformance/src/support.rs:475:14: check fails
conformance_rs_02: expected LLT4010, found ["LLT4001"] (LLT4001: phase link: reasoner declaration `Spend` elaborates to `Spend.fire`, which is ill-formed: function `Tick.apply` argument has type Prod (T) (Nat), expected Prod (Bool) (Nat))
conformance_rs_03 (subject): expected LLT4010, found ["LLT4001"] (LLT4001: phase link: reasoner declaration `Clinic` elaborates to `Clinic.accept`, which is ill-formed: function `probedCheck` argument has type Nat, expected Bool)
conformance_rs_03 (candidate): expected LLT4010, found ["LLT4001"] (LLT4001: phase link: reasoner declaration `Clinic` elaborates to `Clinic.accept`, which is ill-formed: function `boundedCheck` argument has type Bool, expected Nat)
conformance_rs_09: thread 'conformance_rs_09' (28049) panicked at crates/conformance/src/support.rs:475:14: check fails
conformance_rs_01: thread 'conformance_rs_01' (27729) panicked at crates/conformance/src/support.rs:475:14: check fails
```

Removed: every check was restored; the cases pass.

### reasoning ledger counters can go uncounted

Planted, one at a time in `reasoning.rs`, then `lexlean verify examples/reasoning`: a search no longer counted its expansions (`expansions: succ(counter("expansions"))` kept as `counter("expansions")`), no longer counted the successors it fired (`firings: add(counter("firings"), length(var("__successors")))` as `counter("firings")`), and a forward step no longer counted its firing. Expected: pinned Lean refuses the example, because the committed ledger theorems (`plan_ledger`, `plan_unsolved_ledger`, `spend_ledger`, and the others) state the six counters of each run and the generated bounds (`E.expansions_bounded`, `E.firings_bounded`) are proved from the step that counts. Before the ledger theorems existed, the first two plants verified.

```text
E2 (expansions): error[LLV7002]: Lean rejected `Reasoning.Planner` (error): Type mismatch
  Nat.le_refl ?m.252
E3 (search firings): error[LLV7002]: Lean rejected `Reasoning.Main` (error): Tactic `rfl` failed: The left-hand side
  ((Planner.Plan.run 2).fst.ledger.iterations, (Planner.Plan.run 2).fst.ledger.attempts,
    (Planner.Plan.run 2).fst.ledger.firings, (Planner.Plan.run 2).fst.ledger.expansions,
    (Planner.Plan.run 2).fst.ledger.verifications, (Planner.Plan.run 2).fst.ledger.frontier)
is not definitionally equal to the right-hand side
  (4, 15, 8, 3, 4, 4)
E5 (forward firings): error[LLV7002]: Lean rejected `Reasoning.Budget` (error): Tactic `rfl` failed: The left-hand side
  { state := __t✝, trace := LexLeanRuntime.append __r.trace [__step✝],
        ... firings := __r.ledger.firings, ...}.ledger.firings
is not definitionally equal to the right-hand side
  __r.ledger.firings + 1
```

Removed: the counters were restored; the example verifies.

### reasoning answer correctness under the invariant can fail

Planted: the generated verdict theorem of a reasoner claiming an answer correct under the invariant no longer passed the run invariant to the correctness theorem (`pass = ""` for the `llJ` argument in the `VerdictForward` template). Command: `lexlean verify examples/reasoning`. Expected: `Grade`, whose answer is on the 0..3 scale only because its logic's invariant says so, is refused.

```text
error[LLV7002]: Lean rejected `Reasoning.Main` (error): Application type mismatch: The argument
  llH
has type
  conclude v llR.fst.state = Except.ok llV
but is expected to have type
  Clinic.Consistent ?m.180
in the application
  conclude_sound ?m.179 ?m.180 ?m.181 llH
```

Removed: the argument was restored; the example verifies. `tests/negative/reasoning-false-invariant-answer-correct` states the invariant form exactly and falsely, and verification refuses it.

### reasoning elaboration charge covers large type arguments

Planted: the type arguments of a reasoner's rule uses (copied into every declaration that mentions the rule) no longer counted in its charge (`.saturating_add(0u64.saturating_mul(rule_type_arguments))`). Command: `cargo test -p repo-conformance --test conformance -- conformance_rs_10`. Expected: a reasoner of 40 rules, each used at a type of 100 nested products, elaborates beyond what it was charged, an internal error for an admitted input. Before the charge covered it, the same input failed so, with 41688 nodes against 28100.

```text
thread 'conformance_rs_10' (2608) panicked at crates/conformance/src/support.rs:465:14:
check succeeds: LexLeanError { class: Language, diagnostics: [Diagnostic { code: DiagnosticCode("LLT4001"), message: "phase link: internal: reasoner `Wide` elaborated to 41704 nodes, beyond the 28340 it was charged", ...
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 303 filtered out; finished in 32.85s
```

Removed: the term was restored; `conformance_rs_10` passes.

### reasoning verification counts hold with the check erased

Planted: the generated step theorem of a search's verification count stated an equality (`verifications q = verifications r + 1`), as it did before search reasoners with `answer_correct` existed. Command: `lexlean verify examples/reasoning`. Expected: `PlanFit`, `PlanFitR`, and `PlanFitU` (search reasoners claiming an answer correct, whose generated `check` is erased where the answer is proved) are refused, because a step that does not call the check does not add one; the theorem is now `<=`, proved by the same step theorem for every search.

```text
error[LLV7002]: Lean rejected `Reasoning.Planner` (error): Application type mismatch: The argument
error[LLV7002]: Lean rejected `Reasoning.Planner` (error): Type mismatch
  ... .ledger.verifications =
    __r.ledger.verifications + 1
  --> src/Planner.lex.tex:6:19745
```

Removed: the inequality was restored; the example verifies.

### reasoning E.extract stays guarded wherever the answer is conditional

Planted: `E.extract` was left unguarded for an answer correct under an invariant too (the guard condition `answer_correct.is_none() || answer_premise.is_some()` reduced to `answer_correct.is_none()`). Command: `cargo test -p repo-conformance --test conformance -- conformance_rs_09`. Expected: `tests/negative/reasoning-extract-bypass-invariant`, which calls `Grade.extract` from a definition and so reads an answer outside the invariant, is refused with `LLT4012`; with the plant it links.

```text
thread 'conformance_rs_09' (21748) panicked at crates/conformance/src/support.rs:475:14:
check fails
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 303 filtered out; finished in 1.24s
```

Removed: the guard was restored; `conformance_rs_09` passes.

### reasoning members that read an answer from the caller's state stay guarded

Planted, one at a time in `reasoning.rs`: only `E.extract` guarded where an answer is proved correct under an invariant (the acceptance `E.accept`, which is then the extraction, left reachable: `if erased_conditionally {` for `if false {`), and then the search step `E.searchStep` and the forward `E.conclude` left reachable (the member pushed in their place replaced by one nothing names). Command: `cargo test -p repo-conformance --test conformance -- conformance_rs_09`. Expected: `tests/negative/reasoning-accept-bypass-invariant`, `-accept-bypass-ref`, `-accept-bypass-alias`, `-accept-bypass-observation`, `-search-accept-bypass-invariant` (the first plant), and `reasoning-conclude-bypass-invariant`, `-conclude-bypass-observation`, `-search-step-bypass-invariant` (the second), each an executable definition reading an answer from a state or search its caller supplies, link, and so do the `forgedRead` definitions over the example's `Grade`, `PlanFit`, and `PlanFitR`. Each unplanted fixture is refused with `LLT4012`; `Cap`, `PlanFitU`, and `Plan` (whose check runs) stay reachable, which the case also requires.

```text
BG (accept left reachable): thread 'conformance_rs_09' (5900) panicked at crates/conformance/src/support.rs:475:14:
check fails
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 303 filtered out; finished in 1.27s
BS (searchStep and conclude left reachable): thread 'conformance_rs_09' (6203) panicked at crates/conformance/src/support.rs:475:14:
check fails
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 303 filtered out; finished in 1.52s
```

Removed: the members were guarded again; `conformance_rs_09` passes. Before this record the example's `Grade.accept` and `Grade.conclude` linked from executable code and returned the chart's level, 99 for a chart no run reaches, with the check erased.

### reasoning observation invariants are held to their exact obligations

Planted: the initial obligation of an `observation_invariant` claim (`Rl x (observe x)`) was no longer required to match the theorem the claim names. Command: `cargo test -p repo-conformance --test conformance -- conformance_rs_07`. Expected: `tests/negative/reasoning-observation-initial-inexact` is refused with `LLT4010`; with the plant it links. The preservation obligation, the predicate's signature, and the claim order have their own negatives in the same case (`reasoning-observation-preserved-inexact`, `-invariant-signature`, `-unsorted`), and `reasoning-false-observation-answer-correct` states the conditional answer falsely and is refused by pinned Lean (`LLV7002`).

```text
thread 'conformance_rs_07' (22650) panicked at crates/conformance/src/support.rs:475:14:
check fails
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 303 filtered out; finished in 1.82s
```

Removed: the requirement was restored; `conformance_rs_07` passes.

### reasoning elaboration charge covers a verifier's own type arguments

Planted: the type arguments of a reasoner's own uses of its logic and its verifier no longer counted in its charge (the two `.chain(...)` terms removed from the type-argument sum). Command: `cargo test -p repo-conformance --test conformance -- conformance_rs_10`. Expected: a reasoner whose verifier is a copy of the example's with a parameter nothing mentions, used at a type of 1200 leaves while every rule is used at a small one, elaborates beyond its charge. With the plant that is the internal error below; without it the input links.

```text
thread '<unnamed>' (31526) panicked at crates/conformance/src/support.rs:465:14:
check succeeds: LexLeanError { class: Language, diagnostics: [Diagnostic { code: DiagnosticCode("LLT4001"), message: "phase link: internal: reasoner `Wide` elaborated to 13105 nodes, beyond the 3416 it was charged", ...
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 303 filtered out; finished in 35.87s
```

An earlier form of the shape, whose rules shared the large type, passed with the plant (the rules' own charge covered it) and so could not catch it; the shape now keeps rules, logic, and state small. Removed: the terms were restored; `conformance_rs_10` passes.

### reasoning elaboration charge covers a rule's logic uses and a generator's verifier

Planted, one at a time in `reasoning.rs`: the type arguments of an inference rule's own use of its logic no longer counted in the rule's charge (`.chain(&at.type_arguments)` removed from `rule_types`), and the type arguments of a generate-and-verify reasoner's verifier no longer counted in its charge (the term `16u64.saturating_mul(verifier_at.type_arguments ...)` multiplied by zero). Command: `cargo test -p repo-conformance --test conformance -- conformance_rs_10`. Expected: a copy of `Budget` whose declarations gain a parameter nothing mentions and whose rules and reasoner use the rest at a type of 1200 leaves elaborates a rule beyond its charge, and a copy of `Screening` whose `Dose` has a verifier over such a parameter at 1200 leaves elaborates the reasoner beyond its charge; both link unplanted.

```text
PE: check succeeds: LexLeanError { class: Language, diagnostics: [Diagnostic { code: DiagnosticCode("LLT4001"), message: "phase link: internal: inference rule `TickQ` elaborated to 14531 nodes, beyond the 604 it was charged", ...
PF: check succeeds: LexLeanError { class: Language, diagnostics: [Diagnostic { code: DiagnosticCode("LLT4001"), message: "phase link: internal: reasoner `DoseQ` elaborated to 17185 nodes, beyond the 2740 it was charged", ...
```

Removed: the terms were restored and the planted build was cleaned (`cargo clean -p lexlean`); `conformance_rs_10` passes.

### reasoning transcriptions keep the order a search expands and tries in

Planted, inside the Lean-backed half of `conformance_rs_12` (there is no failing run to record; the refusal it requires is this one): the `Plan` transcription for argument 2 with the candidates of every pour tried in reverse order, stated against its oracle in a copy of the compiler project. The verdict is unchanged (2 and 2 are found either way), so a comparison of verdicts alone accepts it; the fixture states the explained answer, with the steps that reach it, and the six counters of the oracle's run, so pinned Lean refuses it.

```text
S4TRANSCRIPT Lean rejected `Compiler.ReasoningFixtures` (error): Tactic `rfl` failed: The left-hand side
  reasoningPlan2Run
is not definitionally equal to the right-hand side
  TargetSemantics.Outcome.value
    ((match PlannerOracle.Plan 2 with
        | Except.ok found =>
          (((TargetSyntax.Value.nat found.fst.fst).pair (TargetSyntax.Value.nat found.fst.snd)).pair
              (TargetSyntax.Value.list (encodePlanSteps found.snd))).ok
        | Except.error failure => (encodeFailure failure).error).pair
      ((TargetSyntax.Value.nat (PlannerOracle.Plan.run 2).fst.ledger.iterations).pair ...
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 303 filtered out; finished in 385.54s
```

The transcript was captured by printing the message the case requires; the print was removed afterward.

### reasoning transcriptions can lose their search, their verdict, or their plan

Planted, one at a time, then `cargo test -p repo-conformance --test conformance -- conformance_rs_12` (and `conformance_rs_13`): the `Spend` transcription's entry replaced by the constant it answers (same value, no search); then, inside the Lean-backed half of `conformance_rs_12`, a `Plan` transcription whose visited-state check is removed (it never saturates, so it answers exhausted where the oracle is unsolved) and a `Dose` transcription that checks four candidates for a budget of three (it rejects where the oracle exhausts), each stated against its oracle in a copy of the compiler project; and in `gnaf/reasoning.rs` the goal-directed plan answering level 2 for two findings without an infection. Expected: the steps check refuses the first (a transcription charges at least the ledger's guard evaluations and firings), pinned Lean refuses the next two, and the plan fixture's levels refuse the last.

```text
FL: thread 'conformance_rs_12' (3262) panicked at crates/conformance/src/cases/reasoning.rs:3018:13:
reasoning-spend-2 charges its search: 2 steps for 12 guard evaluations and firings
Lean (Plan without the visited check): Lean rejected `Compiler.ReasoningFixtures` (error): Tactic `rfl` failed: The left-hand side
  reasoningPlan5Run
is not definitionally equal to the right-hand side
  TargetSemantics.Outcome.value
    (match PlannerOracle.Plan.verdict 5 with
    | Except.ok found => ((TargetSyntax.Value.nat found.fst).pair (TargetSyntax.Value.nat found.snd)).ok
    | Except.error failure => (encodeFailure failure).error)
    193370
Lean (Dose with a budget of four): Lean rejected `Compiler.ReasoningFixtures` (error): Tactic `rfl` failed: The left-hand side
  reasoningDose120Run
is not definitionally equal to the right-hand side
  TargetSemantics.Outcome.value
    (match ScreeningOracle.Dose.verdict 120 with
    | Except.ok found => (TargetSyntax.Value.nat found).ok
    | Except.error failure => (encodeFailure failure).error)
    401
GN: thread 'conformance_rs_13' (3483) panicked at crates/conformance/src/cases/reasoning.rs:3283:9:
assertion `left == right` failed: reasoning-gnaf-goal computes Triage's levels
  left: Pair { left: Nat { value: "3" }, right: Pair { left: Nat { value: "0" }, right: Pair { left: Nat { value: "2" }, right: Pair { left: Nat { value: "2" }, right: Nat { value: "0" } } } } }
 right: Pair { left: Nat { value: "3" }, right: Pair { left: Nat { value: "0" }, right: Pair { left: Nat { value: "2" }, right: Pair { left: Nat { value: "1" }, right: Nat { value: "0" } } } } }
```

Removed: the constant, the mutations, and the plan were restored; `conformance_rs_12` and `conformance_rs_13` pass.

### compiler-tooling modules can fail (stage 1a of #27)

A tooling module (SPEC.md §17.20, `TL-01`..`TL-11`) is a program over a typed
registry of the external Lean interface, written as canonical JSON, whose Lean
and LaTeX are both generated from the one linked value, so every claim that it
is closed is a claim about the compiler's own checks and the evidence for it is
what those checks refuse. The `tooling` suite plants one defect at a time in a
copy of each of the three committed fixtures (`tests/tooling/core`, `commands`,
`traversal`) and requires the registered code and the reason it names: 18 planted
defects of the schema and the encoding (`TL-01`), 1 of the context and the
refusals of the public API, the command line, language 1.1, and a configuration
member (`TL-02`), 16 of the authority and the registry (`TL-03`), 13 of the type
rules (`TL-04`), 11 of the typing of a call (`TL-05`), 20 of the term language
(`TL-06`), 6 of recursion (`TL-07`), 9 of complete traversal (`TL-08`), and 12
of commands, invocations, and evaluations (`TL-09`): 106 in all. The plants of
`TL-04` to `TL-09` were first run with a reason that no message contains, so
that the failure printed what the compiler said and the text each plant must
name could be taken from it; none of the 106 was accepted, and each is refused
with its own code and the text it names. The pinned Lean is the second check: it elaborates every
committed module without a message (`TL-10`), refuses a registry that gives
`Nat.pred` another arity while the tool uses it consistently with the
misstatement (`Function expected`), refuses one that omits `Option.none` while
the match is exhaustive over what it lists (`Missing cases`), and runs the
committed command's elaborator, which refuses an invocation of a constant that
does not exist (`TL-09`): none of the three can be seen by the compiler alone.
`TL-01` also changes every single node of every fixture (the member removed, or
replaced by `null`, `0`, `""`, `{}`, or `[]`, or the element repeated): 19761
changes, 890 accepted and 18871 refused by `LLT4013`, `LLT4014`, or `LLT4015`,
none panicking (R5), in 13 s.

Two checks were found by reading the finished sources against the statements,
and each has a plant of its own below. A traversal that mentioned a field but
called its handler on the subject was accepted, because the check asked only
that the branch call a handler of the right type somewhere; it now follows the
value from the field through `let`, `match`, and the lambdas handed to the same
call (`P04`, and the plant "a handler applied to the subject, not the field" of
`TL-08`). An abbreviation of itself unfolds to itself, which the test for a
fixed point did not tell from a type with nothing left to unfold (`P07`).
A shadowing binder is refused so that what a name stands for is read from its
one binding, which the traversal check depends on (`P03`).

The first version of this change moved `schemas/tooling-module.schema.json` into
the compiler-semantics ID like every schema of `schemas/`. Regenerating the
goldens (`cargo xtask check-golden --write`) then showed that the same commit
had changed the generated Lean of three existing examples (`models`,
`reasoning`, and the `compiler` project): the shared declaration emitter had
started to drop the blank rendering of a semantic declaration, which is the
separator its neighbours carry. The emitter now skips a blank rendering only for
a tooling module (`DeclarationSource::skip_blank`), the schema lies in
`tooling/schemas/` where no identity reads it, and `check-golden --write` leaves
every existing golden, lock, and verification record unchanged.

Each of the 24 plants below was applied alone to the finished sources (to the
schema file for `P24` and to a golden for `P23`), produced the failure quoted,
and was removed. Command for each: `cargo test -p repo-conformance --test
conformance -- <case> --exact`.

Planted: a match that need not be exhaustive (`crates/lexlean/src/ir/tooling_check.rs`). Cases: `conformance_tl_06`, `conformance_tl_08`. Expected: a match that omits a constructor is accepted, and so is a registry that gains a constructor no branch handles.

```text
thread 'conformance_tl_06' (N) panicked at crates/conformance/src/cases/tooling.rs:192:9:
TL-06: 1 plants were not refused as named:
a match that omits a constructor: the module was accepted
```

Planted: a match blind to the scrutinee's index (`crates/lexlean/src/ir/tooling_check.rs`). Cases: `conformance_tl_06`. Expected: the constructors that exist only at another index count as available, so a traversal of `Code .pure` no longer has a match it can satisfy (the fixture itself is refused).

```text
thread 'conformance_tl_06' (N) panicked at crates/conformance/src/cases/tooling.rs:126:44:
traversal: LexLeanError { class: Language, diagnostics: [Diagnostic { code: DiagnosticCode("LLT4015"), message: "phase link: the match on `Lean.Compiler.LCNF.Code` is not exhaustive: it omits oset, uset, sset, setTag, inc, dec, del", primary: Some(Span { path: "src/Main.lex.tex", byte_start: 125, byte_end: 19403, line_start: 5, column_start: 14, line_end: 5, column_end: 19274 }), labels: [], notes
```

Planted: shadowing allowed (`crates/lexlean/src/ir/tooling_check.rs`). Cases: `conformance_tl_06`. Expected: a binder that shadows a variable in scope is accepted.

```text
thread 'conformance_tl_06' (N) panicked at crates/conformance/src/cases/tooling.rs:192:9:
TL-06: 1 plants were not refused as named:
a binder that shadows another: expected LLT4015 naming "`xs` shadows a variable in scope", found [("LLT4015", "phase link: the variable `h` is not in scope")]
```

Planted: a handler on any value counts as recursing into the field (`crates/lexlean/src/ir/tooling_check.rs`). Cases: `conformance_tl_08`. Expected: a handler applied to the subject, not to a value derived from the field, is taken for recursing into the field.

```text
thread 'conformance_tl_08' (N) panicked at crates/conformance/src/cases/tooling.rs:192:9:
TL-08: 1 plants were not refused as named:
a handler applied to the subject, not the field: the module was accepted
```

Planted: a traversal with a default branch (`crates/lexlean/src/ir/tooling_check.rs`). Cases: `conformance_tl_08`. Expected: a traversal with a default branch is accepted.

```text
thread 'conformance_tl_08' (N) panicked at crates/conformance/src/cases/tooling.rs:192:9:
TL-08: 1 plants were not refused as named:
a default branch: the module was accepted
```

Planted: a traversal that may skip a field (`crates/lexlean/src/ir/tooling_check.rs`). Cases: `conformance_tl_08`. Expected: a traversal that never mentions a field of its recursive family is accepted.

```text
thread 'conformance_tl_08' (N) panicked at crates/conformance/src/cases/tooling.rs:192:9:
TL-08: 1 plants were not refused as named:
a skipped field: expected LLT4015 naming "the traversal `Traversal.code` skips the field `k` of `Lean.Compiler.LCNF.Code.fun`", found [("LLT4015", "phase link: the traversal `Traversal.code` does not recurse into `Lean.Compiler.LCNF.Code` for the field `k` of `Lean.Compiler.LCNF.Code.fun`: its branch calls no traversal of `Lean.Compiler.LCNF.Code` in the group on a value derived from the field")]
```

Planted: an abbreviation of itself (`crates/lexlean/src/ir/tooling_check.rs`). Cases: `conformance_tl_04`. Expected: an abbreviation of itself is accepted.

```text
thread 'conformance_tl_04' (N) panicked at crates/conformance/src/cases/tooling.rs:192:9:
TL-04: 1 plants were not refused as named:
an abbreviation of itself: expected LLT4014 naming "abbreviates itself", found [("LLT4015", "phase link: `Commands.elab` is declared to return Lean.Elab.Command.CommandElab, its body has type (Lean.Syntax → (Lean.Elab.Command.CommandElabM Unit))")]
```

Planted: a self call without `partial` (`crates/lexlean/src/ir/tooling_check.rs`). Cases: `conformance_tl_07`. Expected: a self call of a function that is not partial is accepted.

```text
thread 'conformance_tl_07' (N) panicked at crates/conformance/src/cases/tooling.rs:192:9:
TL-07: 1 plants were not refused as named:
a self call of a function that is not partial: the module was accepted
```

Planted: a mutual group that need not be one run (`crates/lexlean/src/ir/tooling_check.rs`). Cases: `conformance_tl_07`. Expected: a mutual group that is not one contiguous run is accepted.

```text
thread 'conformance_tl_07' (N) panicked at crates/conformance/src/cases/tooling.rs:192:9:
TL-07: 1 plants were not refused as named:
a mutual group that is not one run: the module was accepted
```

Planted: the authority's commit is not compared (`crates/lexlean/src/ir/tooling.rs`). Cases: `conformance_tl_03`. Expected: an authority at another Lean commit is accepted.

```text
thread 'conformance_tl_03' (N) panicked at crates/conformance/src/cases/tooling.rs:192:9:
TL-03: 1 plants were not refused as named:
an authority at another commit: the module was accepted
```

Planted: a digest of any length (`crates/lexlean/src/ir/tooling_check.rs`). Cases: `conformance_tl_03`. Expected: a source digest of any length is accepted.

```text
thread 'conformance_tl_03' (N) panicked at crates/conformance/src/cases/tooling.rs:192:9:
TL-03: 1 plants were not refused as named:
a short digest: the module was accepted
```

Planted: a value that need not be canonical (`crates/lexlean/src/ir/tooling.rs`). Cases: `conformance_tl_01`. Expected: a value that is not its own canonical encoding is accepted.

```text
thread 'conformance_tl_01' (N) panicked at crates/conformance/src/cases/tooling.rs:192:9:
TL-01: 2 plants were not refused as named:
members out of order: the module was accepted
an escaped scalar: the module was accepted
```

Planted: every loaded project admits tooling (`crates/lexlean/src/project.rs`). Cases: `conformance_tl_02`. Expected: a tooling module is accepted in a user project.

```text
thread 'conformance_tl_02' (N) panicked at crates/conformance/src/support.rs:495:14:
check fails
```

Planted: a tooling module not charged to max_ir_nodes (`crates/lexlean/src/link.rs`). Cases: `conformance_tl_01`. Expected: a tooling module is not charged to `max_ir_nodes`.

```text
thread 'conformance_tl_01' (N) panicked at crates/conformance/src/cases/tooling.rs:387:40:
over the node limit
```

Planted: language 1.1 admitted (`crates/lexlean/src/ir/tooling.rs`). Cases: `conformance_tl_02`. Expected: a tooling module is accepted under language 1.1.

```text
thread 'conformance_tl_02' (N) panicked at crates/conformance/src/cases/tooling.rs:192:9:
TL-02: 1 plants were not refused as named:
language 1.1: the module was accepted
```

Planted: an elaborator of the unfolded type accepted (`crates/lexlean/src/ir/tooling_check.rs`). Cases: `conformance_tl_09`. Expected: an elaborator of the unfolded type, which Lean's `command_elab` attribute would refuse, is accepted.

```text
thread 'conformance_tl_09' (N) panicked at crates/conformance/src/cases/tooling.rs:192:9:
TL-09: 1 plants were not refused as named:
an elaborator of the unfolded type: the module was accepted
```

Planted: a missing argument accepted (`crates/lexlean/src/ir/tooling_check.rs`). Cases: `conformance_tl_05`. Expected: a call with a missing argument is accepted.

```text
thread 'conformance_tl_05' (N) panicked at crates/conformance/src/cases/tooling.rs:192:9:
TL-05: 2 plants were not refused as named:
a call with a missing argument: the module was accepted
a required argument omitted before the defaults: the module was accepted
```

Planted: a comment in the generated Lean (`crates/lexlean/src/backend/tooling.rs`). Cases: `conformance_tl_10`. Expected: the generated Lean holds a comment.

```text
thread 'conformance_tl_10' (N) panicked at crates/conformance/src/cases/tooling.rs:1736:35:
the project renders: render: LLI9001: phase lean-backend: a tooling rendering contains a comment token
```

Planted: `partial` not emitted (`crates/lexlean/src/backend/tooling.rs`). Cases: `conformance_tl_10`. Expected: the generated Lean omits `partial`, so Lean cannot show termination of the traversal.

```text
thread 'conformance_tl_10' (N) panicked at crates/conformance/src/cases/tooling.rs:1789:5:
traversal: Lean exited 1 and said: $STAGING/Tool.lean:8:16: error: fail to show termination for
  Traversal.code
  Traversal.cases
  Traversal.alt
  Traversal.funDecl
```

Planted: a declaration without its own piece (`crates/lexlean/src/backend/semantic.rs`). Cases: `conformance_tl_10`. Expected: a declaration has no piece of its own (the LaTeX document, which shares the emitter, loses its entry too).

```text
thread 'conformance_tl_10' (N) panicked at crates/conformance/src/cases/tooling.rs:1932:9:
assertion `left == right` failed: commands: one LaTeX entry per declaration
  left: 35
 right: 36
```

Planted: an entry that does not say its kind (`crates/lexlean/src/backend/tooling.rs`). Cases: `conformance_tl_10`. Expected: a LaTeX entry does not state its kind.

```text
thread 'conformance_tl_10' (N) panicked at crates/conformance/src/cases/tooling.rs:1947:9:
assertion `left == right` failed: commands: each entry states its kind, in order
  left: []
 right: ["type", "type", "type", "type", "type", "type", "type", "type", "call", "call", "call", "call", "call", "call", "call", "call", "call", "type", "type", "type", "type", "type", "type", "type", "type", "type", "type", "plumbing", "function", "command", "function", "function", "function", "function", "invoke", "evaluate"]
```

Planted: output that depends on the process (`crates/lexlean/src/backend/tooling.rs`). Cases: `conformance_tl_11`. Expected: the LaTeX output depends on the process (its identifier is written into it).

```text
thread 'conformance_tl_11' (N) panicked at crates/conformance/src/cases/tooling.rs:2013:35:
R10: commands: coverage/ToolingCommands/Main.coverage.json differs from its golden; run `cargo xtask check-golden --write`
```

Planted: a golden that differs by one byte (`tests/tooling/core/expected/build/modules/ToolingCore/Main.lean`). Cases: `conformance_tl_11`. Expected: a committed golden differs from the published build tree by one byte.

```text
thread 'conformance_tl_11' (N) panicked at crates/conformance/src/cases/tooling.rs:2013:35:
R10: core: modules/ToolingCore/Main.lean differs from its golden; run `cargo xtask check-golden --write`
```

Planted: a schema that allows a default written out (`tooling/schemas/tooling-module.schema.json`). Cases: `conformance_tl_01`. Expected: the committed schema allows a member written with its default.

```text
thread 'conformance_tl_01' (N) panicked at crates/conformance/src/cases/tooling.rs:297:9:
an opaque flag written with its default: the committed schema must refuse it
```

### the extraction adapter can fail (stage 1 of #27)

The adapter of named-root extraction (SPEC.md §22.10) was handwritten Lean with a
handwritten registry. It is now a tooling module, `language/lcnf-1.2/tooling/`,
over a registry the pinned Lean gives, so that what the compiler claims of it is
what the gates below refuse. Three of them check the files: `cargo xtask
check-calculus` requires the committed project to equal what its generator
(`crates/conformance/src/lcnf_adapter.rs`, `lcnf_extract.rs`, `lcnf_registry.rs`)
renders over `tests/lcnf/registry.json`; `conformance_ne_04` requires that
registry to equal a fresh derivation from the pinned toolchain (the exporter
`tests/lcnf/export.lean` and the elaborated adapter's own constants) and every
source digest to equal the file in the toolchain; and `conformance_tl_03`
requires the module's authority to equal the compiler's pin. Three check the
program: `conformance_ne_04` runs the handwritten adapter, kept as the oracle
`tests/lcnf/oracle/extract.lean`, beside the module on every kind of constant
the extraction classifies (12 roots, 23 constants, every base-phase form) and
requires the same record up to the numbering of the variables of each constant;
it requires the module's record to be the same bytes whatever number of
definitions Lean elaborated before it (0, 1, 2, 3, 5, 7, 11, and 13); and it
plants six drifts in a copy of the registry, each of which the pinned Lean
refuses and the driver reads as `LLV7012` naming what drifted: a default value
(`Lean.Environment.find?`), a binder kind (`Lean.throwError`, an instance binder
stated as an implicit one, which types the same for the tool), a constructor
order (`Lean.Compiler.LCNF.Alt`, consistent with every match of the tool), a
plumbing constant removed, one that is never used, and a warning inside the
adapter, which Lean prints on standard output with exit 0.

Lean is not deterministic in what the oracle compared. Lean orders the join
points of a function by the numbering of its fresh variables, and that numbering
depends on everything the process elaborated before the translation: the oracle
run with 0, 1, 5, 11, 100, or 301 definitions before it, and with 2, 3, 7, 50, or
299, gives two programs for `match b with | true => n + 1 | false => n + 2`
followed by `x * x`. The handwritten adapter therefore gave the compiler input
its own Lean text happened to give. The module starts every translation from the
name generator's initial state, so that its record does not depend on its own
text (plant `N4`), and the probe module has a function with two join points
(`twoJoins`) for that check and none in the comparison with the oracle. The
canonical compiler inputs of the four examples with production roots, from the
verification of each by the module and from the committed record of the oracle:
`production` and `models` are byte-identical; `reasoning` differs in 6 of 166
declarations (`Reasoning.Clinic.Triage.select`, `recommendationCheck`,
`Grade.select`, `Plan.failure`, `Plan.searchStep`, `Screen.searchStep`) and
`production-coverage` in 7 of 161 (`reachableFrom`, `natOps`, `options`,
`checkedAddInt64`, `checkedQuotientInt64`, `checkedSubtractInt64`,
`multiplyMagnitudeInt64`), each with the same nodes, in the order of two join
points; their closures, externals, inductives, erased proofs, and roots are
equal. Every project of language 1.2 was verified again with real Lean and
`leanchecker`, and its `expected/verify` regenerated; the 1.0 and 1.1 projects
were verified again and are byte-identical.

Each plant is applied alone to the finished sources, the gate is run, and the
tree is restored.

Planted: a byte appended to the committed project (`language/lcnf-1.2/tooling/src/Extract.lex.tex`). Command: `cargo xtask check-calculus`. Cases: none (the gate). Expected: the committed project differs from its generator

```text
gate failed: language/lcnf-1.2/tooling/src/Extract.lex.tex differs from its generator; run `cargo xtask check-calculus --write`
```

Planted: a source digest of the registry changed by one hex digit (`tests/lcnf/registry.json`). Command: `cargo xtask check-calculus`. Cases: none (the gate). Expected: the project no longer equals what the registry renders

```text
gate failed: language/lcnf-1.2/tooling/src/Extract.lex.tex differs from its generator; run `cargo xtask check-calculus --write`
```

Planted: the same digest forged *consistently*, the project rewritten from the forged registry (`tests/lcnf/registry.json` and `language/lcnf-1.2/tooling/`). `cargo xtask check-calculus` passes (994 generated files equal their generator), because the project is what the forged registry renders; `conformance_ne_04` is the gate that sees it. Expected: the committed rows are not the exporter's.

```text
panicked at crates/conformance/src/cases/extraction.rs:1317:5:
assertion `left == right` failed: the committed rows are the exporter's
```

Planted: an adapter that reports an opaque constant as a definition, the project regenerated (`crates/conformance/src/lcnf_extract.rs`). Command: the comparison of the two records, as `conformance_ne_04` makes it. Cases: `conformance_ne_04`. Expected: the record is not the oracle's

```text
the tooling module's record is not the handwritten oracle's: the first difference is at byte 4159:
adapter: ...,"name":"Probe.Main.halver","original_kind":"definition"},{"computable":true,"declaration":{"level_parameters":[],"parameters":[],"safe":true,"type":{"kind":"const","levels":[],"name":"Nat"},"uses":["Inhabited","Nat","instInhabitedNat"],...
oracle:  ...,"name":"Probe.Main.halver","original_kind":"definition"},{"computable":true,"declaration":null,"generates_code":true,"inductive":null,"internal":false,"kernel_uses":["Nat","instInhabitedNat","Inhabited.default"],"kind":"opaque",...
```

Planted: an adapter that does not restart the name generator, the project regenerated (`crates/conformance/src/lcnf_extract.rs`). Command: the counter check of `conformance_ne_04`. Cases: `conformance_ne_04`. Expected: the record changes with what Lean elaborated before it

```text
the record changes when 1 definitions come before the extraction
```

Planted: a registry check that finds every unregistered constant plumbing, the project regenerated (`crates/conformance/src/lcnf_extract.rs`). Command: the Lean half of `conformance_ne_04`. Cases: `conformance_ne_04`. Expected: a plumbing constant removed from the registry is drift

```text
thread 'main' (8351) panicked at crates/conformance/src/cases/extraction.rs:1344:18:
a drifted run is refused: CompilerInput { spec: "lexlean/compiler-input/2", lean: LeanIdentity { version: "4.32.1", githash: "f054605aea4b840552cca2e725580bffd1e1b704" }, roots: [], closures: [], declarations: [], inductives: [], externals: [], erased: [] }
```

Planted: a driver that cannot tell where in its text Lean's message is (`crates/lexlean/src/production/lcnf.rs`). Command: `cargo test -p repo-conformance --test conformance -- conformance_ne_04 --exact`. Cases: `conformance_ne_04`. Expected: a warning inside the adapter is drift

```text
thread 'conformance_ne_04' (10086) panicked at crates/conformance/src/cases/extraction.rs:1352:22:
expected drift naming "the pinned extraction adapter no longer elaborates cleanly", got Rejected("the extraction printed more than its single JSON record")
```

Planted: a driver that does not evaluate the registry check (`crates/lexlean/src/production/lcnf.rs`). Command: `cargo test -p repo-conformance --test conformance -- conformance_ne_04 --exact`. Cases: `conformance_ne_04`. Expected: a registry that lists its constructors in another order is drift; without the check the driver's text does not change with the registry at all, so the plant of a constructor order produces the same text as the unplanted run and the case fails on that.

```text
panicked at crates/conformance/src/cases/extraction.rs:1340:9:
assertion `left != right` failed: `Lean.Compiler.LCNF.Alt` has constructors
```

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
- `LEAN-LCNF-4-32-1`: Lean's compiler front end (base-phase LCNF), bound by source revision and by the SHA-256 of the defining source file of every registered call and type of the tooling project `language/lcnf-1.2/tooling/`, re-checked against the pinned toolchain by `conformance_ne_04`.
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
every library declaration's printed axioms with its native row's exact policy
(7,740 declarations audited by name; `library.toml` lists only the modules),
the statement vocabulary SPEC.md quotes with what the pinned Lean prints of the
generated library, the shipped calculus modules with the compiler golden, and
refuses planted `sorry`,
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
`conformance_sp_09` certifies certificate E for every root in each target
and restates the first root's two ways (the other result shape, another
function); `conformance_sp_10` checks the boundary: every root with a map or
set parameter has an entry, a validator is called only from the entry or from
validators, every invalid input is refused with `none`, two mutations of the
boundary and the planted calls above are refused; `conformance_sp_11` builds
every certified root as a package under the pinned Rust toolchain and compares
504 runs, 96 of them through an entry, with the interpreter's outcomes.
`conformance_sp_05` verifies
`examples/production` and requires every certificate, the audit output, and a
schema-valid `preservation.json` bound by the attestation in the published
set; the negative fixtures `certificate-rejected` (a lake overlay turns the
certificates' natural additions into subtractions) and `preservation-drift`
(it appends a false theorem to the shipped library) must fail with `LLV7013`
and `LLV7014` with nothing published, and `certificate-b-rejected` (the
overlay turns every `nat_add` call of each certificate B's crate into
`nat_sub`, leaving certificate A intact) with `LLV7015`.

## GNAF certification (Issue #23)

`conformance_gc_01` reads every certificate fixture under
`compiler/certificates/` (73) as published, requires canonical bytes, the
framed SHA-256 identity of the certificate, the schemas, and the host's
verdict, and refuses malformed certificates with `LLB6007` and certificates
beyond 65,536 premises, 1,024 positions in a list, or numbers of 2^32 with
`LLS8002`. `conformance_gc_02` requires the committed `Certification`,
`GnafCertificates`, and model sources to equal their generators, runs the
compiler project's `verify` (the kernel reduces `Certification.check` on each
of the 73 certificates to the verdict the host gives it, for each
certificate with a `reduced` premise decides `certificateReductionsPreserve`,
and decides both observation relations on pairs of systems that are no schema's
rewriting, one pair of each class per request),
and compiles a module stating five false claims (a certified optimum for a
certificate missing a competitor's premise, an unattained bound, a locally
normal system, a premise-free bound over unresolved systems, and the
preservation of an ineligible reduction), each of which Lean rejects as "Not a
definitional equality". `conformance_gc_03`..`conformance_gc_08` assert the
verdict, obligation, and established status of every fixture, sweep mutations
of the certified lower bounds (every premise dropped or weakened, the bound
moved, an attainer dropped or added), sweep every affine bridge below the
committed one (including a system bounded exactly on the bridge's line), pose
the unresolved requests as frontiers and as bridge claims, compare the
certificate with enumeration where enumeration answers, compare the host's
`observations` with the trials of every pair of systems of four requests,
check `preserves` against `improves` for every schema application of six
requests, and require that `normalityVerdict` is the only reader of
`isNormal`. `conformance_gc_09` compiles the Lean the compiler generates from the 13 native
modules of `compiler/proofs/` over the compiler project's generated modules and
the shipped preservation library, compares the exact axioms of the 3,957
declarations it audits by name (what their rows pin) with Lean's `#print
axioms`, instantiates the
soundness theorems at every certified fixture, elaborates the preservation
theorems at ascriptions stated in the module's own relations (so the library
cannot be proving a second copy), replays the library and the instances
through `leanchecker` (the same kernel, §22.4, not an independent checker), and
requires Lean to reject the library when the checker's strict test is weakened
and when a false lemma (a theorem's statement exchanged for another's, its
proof kept) is planted in a staged copy of `Schemas`, and the exact axiom audit
to see an axiom added to a theorem (its proof replaced by `sorryAx`).
`conformance_gc_11` requires the library to be LexLean source: no Lean file in
its project or declaring its namespace anywhere in the release tree, sources
exactly the registered modules, a registry that lists only
the modules, a project that builds with Lean byte for byte what the kernel
check compiles, a drifted source that generates different Lean, and a foreign
import, an inexact policy, or a first-schema module carrying second-schema data
to be refused. `conformance_gc_10` requires the generated files to
be deterministic and every identity to be recomputed from content. Each
theorem is a kernel-checked proof about the evaluator and the checker as
generated; the host transcription and the fixtures are `build` evidence
validated against that kernel.

### GNAF certification can fail (host transcription)

Each defect below was planted one at a time in
`crates/lexlean/src/gnaf/certification.rs`, with `cargo test -p
repo-conformance --test conformance -- <tests> conformance_gc_01`, and
restored. Every one was refused by the named case, with the panic line of
`verdict` (`certification.rs:102`) or of the sweep in `conformance_gc_01`
(`certification.rs:255`) and the fixture it names:

| Planted | Refused by | Fixture that fails |
| --- | --- | --- |
| the premise-count check skipped for an extra premise | `gc_03`, `gc_08` | `lb-extra-premise` |
| the premise-count check skipped for a missing premise | `gc_03`, `gc_08` | `lb-premise-omitted` |
| a strict lower bound read as `bound <= cost` | `gc_03`, `gc_01` | `lb-tie-bounded` |
| a non-strict bound read as `bound < cost` | `gc_03`, `gc_01` | `lb-undercut`, `lb-no-attainer` |
| an omitted tie not reported | `gc_03` | `lb-tie-dropped` |
| the eligibility test (no plan calls the entry) dropped | `gc_05` | a plan calling the entry blocks every schema |
| the universe identity not compared | `gc_08` | `reused-enlarged` |
| an absent bridge read as slope 1, offset 0 | `gc_07` | `bridge-missing` |
| a bridge violation read as `cost <= limit` | `gc_07` | `bridge-attained` |
| a bound at the line read as unproved (`cost <= limit`) | `gc_07` | the tight-line sweep |
| preparation dropped from a bound | `gc_03`, `gc_01` | `lb-preparation-charged` |
| a premise `excluded` read as proving exclusion of an admitted system | `gc_03`, `gc_01` | `lb-excluded-admitted` |
| an unresolved measurement read as inadmissible | `gc_03`, `gc_01` | `lb-unresolved-measured` |
| an attainer allowed to be bounded | `gc_03`, `gc_01` | `lb-attainer-bounded` |
| attainers not required to be in order | `gc_03`, `gc_01` | `lb-attainers-unordered` |
| a normality certificate returning the optimum | `gc_06` | `normal-not-global` |
| a system always GNAF-normal | `gc_06` | `normality-improvable` |
| a frontier non-member not required to be dominated | `gc_04` | `frontier-tie-dropped` |
| a dominated member accepted | `gc_04` | `frontier-dominated-member` |
| a frontier bound not required to be dominated | `gc_04` | `frontier-premise-weakened` |
| a reduction accepting a bound as complete | `gc_05` | `reduction-incomplete` |
| the small specialization allowed at the threshold | `gc_05` | `reduction-small-at-threshold` |
| the large specialization refused at the threshold | `gc_05` | `reduction-threshold-edge` |

The first run of these plants left thirteen unrefused. Nine are now refused
by a fixture or a sweep added for them (a bound at a tie, preparation in a
bound, a premise excluding an admitted system, an unresolved system offered as
measured, a bounded attainer, attainers out of order, both thresholds' edges,
and a bridge bound exactly at its line). Four cannot be refused: each removes
a conjunct that another conjunct makes redundant, so the mutant computes the
same function. Dropping `ascending` from `pays_at_least` leaves `pickFrom`'s
own length check, which already refuses a position listed twice or out of
order (the fixture `lb-positions-repeated` states that refusal); dropping
`small == large` from the collapse eligibility leaves `rewrite`, which is
`none` for a dispatch of two plans (`reduction-collapse-distinct`); and
dropping either the size or the preparation comparison from `improves` changes
nothing, because the rewriting of a dispatch to one plan's fixed system reaches
a subset of the dispatch's code. They are kept as the statements of UOR-GNAF
§6.3 and are equivalent mutants, not evidence.

A one-line mutation sweep of the transcription then found seventeen more
survivors (adversarial review `r23a`: every `open(Obligation::..)` replaced by
`None` and every `<`, `<=`, `==`, `&&`, `||` flipped in the lines of
`certification.rs` from `eligible` to `check`, 60 mutants, each run against
`conformance_gc_01` and `gc_03`..`gc_08`, `gc_10`). Each real one is now
refused by a named case, and the sweep was run again on the final tree:

| Mutant | Refused by | Fixture or sweep that fails |
| --- | --- | --- |
| lower-bound attainer index `< premises` as `<=` | `gc_03` | `lb-attainer-at-count` |
| frontier member index `< premises` as `<=` | `gc_04` | `frontier-member-at-count` |
| frontier members "in order and in range" as "or" | `gc_04` | `frontier-members-unordered`, `frontier-member-at-count` |
| a frontier member with an unproven fact accepted | `gc_04` | `frontier-member-unproven` |
| a frontier non-member with an unproven fact accepted | `gc_04` | `frontier-nonmember-unproven` |
| a frontier member or non-member the fuel leaves unresolved ignored | `gc_04` | the unresolved request posed as a frontier |
| a reduction over an unresolved system ignored | `gc_05` | `reduction-over-unresolved` |
| a bridge accepting a reduced or unproven system | `gc_07` | `bridge-reduced`, `bridge-unproven` |
| a bridge accepting an unresolved system | `gc_07` | the unresolved request posed as a bridge claim |
| an out-of-domain position in `pays_at_least` accepted (host and kernel would disagree) | `gc_03` | `lb-position-out-of-domain` |
| a Pareto claim routed as a scalar reduction or lower bound | `gc_04`, `gc_03` | `reduction-pareto`, `route-lower-bound-pareto` |
| a measured system above an argmin's bound, or a measured tie under an optimum, read as an omission | `gc_03` | `lb-argmin-measured`, `lb-optimum-tie-measured` |
| `after < steps` as `<=` in the observations of a rewriting | `gc_05` | `observations` against the trials of every pair of systems |

Five mutants of the final sweep are not refused, and each is equivalent: the
mutant computes the function the original does on every input that reaches it.
`small == large && domain-of-lists` in the collapse eligibility as `||`:
`rewrite` is `none` for a dispatch of two plans, so the conjunct is never the
deciding one. `size(target) <= size(selector)` in `improves` as `<`: a
dispatch carries its dispatching function and both plans, so it is strictly
larger than the fixed system of one plan (and dropping the size or the
preparation comparison altogether changes nothing, because the rewriting
reaches a subset of the dispatch's code). `keeps && cheaper` in `preserves` as
`||`: it is evaluated only after `improves` holds, and `improves_preserves`
proves both sides then. The position of the rewriting in the expansion, which
a `reduces` fact carries and no verdict reads. `cost < bound` after
`cost == bound` was excluded, which `<=` leaves unchanged. Two further
mutants (`||` read as `&&` in a closure's head) do not compile.

A second review (`r23b`) found three more mutants of `premise_fact` that every
non-Lean case let through (an `excluded` position one past the domain, an
`excluded` argument on which the system exhausts the fuel, and a
`pays_at_least` over an inadmissible prefix, each read as an exclusion where
the model says the premise proves nothing; the first two are unsound), and
that nothing pinned the observation relations themselves. Each is refused:

| Planted | Refused by | Fixture or pair that fails |
| --- | --- | --- |
| `excluded` with a position outside the domain read as `Inadmissible` | `gc_03`, `gc_01` | `lb-excluded-at-length` |
| `excluded` on an argument that exhausts the fuel read as `Inadmissible` | `gc_03`, `gc_01` | `lb-excluded-unresolved` |
| `pays_at_least` over an inadmissible prefix read as `Inadmissible` | `gc_03`, `gc_01` | `lb-pays-inadmissible-prefix` |
| `certificateReductionsPreserve` true for a request that is no grammar's | `gc_05`, `gc_01` | `request-rejected-cached` |

The unresolved frontier and bridge cases are now committed fixtures
(`frontier-unresolved-member`, `frontier-unresolved-nonmember`,
`bridge-unresolved`, over the requests `frontier-over-unresolved` and
`optimum-over-unresolved`), so the kernel decides them. The refusal
`NoGrammar` is gone: `validate` rejects every carrier but a grammar before a
certificate is read, so the model's total match over carriers names those
rejections (`request-rejected-cached`, and `gc_08` for the other three). The
final sweep, 60 mutants of the lines from `eligible` to `check` and the 31
plants above, was run on this tree: 53 refused, 2 do not compile, and the
same 5 equivalent mutants survive (the justifications above stand), plus the
4 equivalent plants.

### GNAF certification can fail (model and library)

The observation relations of the closed module are pinned by kernel-decided
instances on pairs of systems that are no schema's rewriting (`<request>Pair<i>x<j>Keeps`
and `...Cheaper`, one pair of each class `(keeps, cheaper)` of
`(false, false)`, `(true, false)`, and `(true, true)` per request, read by the
host from the trials, independently of `improves`). Before they existed, a
`keepsObservation` and `cheaperObservation` neutered to tautologies passed
every gate, because each `<id>Preserves` instance is decided by `improves`,
which implies both relations. Each plant below regenerates the module with
`cargo xtask check-calculus --write`, runs the named case, and is restored.

| Planted in `certification_model.rs` | Command | Result |
| --- | --- | --- |
| `keepsObservation` and `cheaperObservation` return true wherever the system is admitted (`no(), no()` as `yes(), yes()` in `status_pair`) | `conformance_gc_02` | the compiler project does not verify: the pair instances are false |
| `cheaperObservation` with `ble` for `blt` | `conformance_gc_02` | the compiler project does not verify |
| `preserves` that no longer reads `sameObservations` (the first relation read twice) | `conformance_gc_05` | `preserves reads sameObservations: {"Option.none", "Option.some", "improves", "rewrite", "strictlyCheaper"}` |

`preserves` defined as `improves` alone is extensionally the same function,
because `improves_preserves` proves `improves` implies the two relations, so
no evaluation can tell them apart; `gc_05` reads the references of `preserves`,
`reductionsPreserve`, and the relations instead, as it does for `isNormal`.

Planted in `crates/conformance/src/certification_model.rs`: the strict
lower-bound test `blt(bound, cost)` as `ble(bound, cost)`; `cargo xtask
check-calculus --write` regenerated `compiler/src/Certification.lex.tex`.
Command: `cargo test -p repo-conformance --test conformance --
conformance_gc_02`. Expected: the kernel disagrees with the host on a
committed verdict.

```text
thread 'conformance_gc_02' panicked at crates/conformance/src/support.rs:1850:17:
the compiler project verifies under pinned Lean 4.32.1: LexLeanError {
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 324 filtered out; finished in 196.38s
```

Removed: the generator was restored and the module regenerated.

Planted in the native library `compiler/proofs/src/` (the sources the
hand-written library's Lean was converted into, `tests/certification-library/
MIGRATION.md`): (a) the pinned axioms of `LexLeanCertification.schema_cheaper`
in `Schemas.lex.tex` changed from the three to `propext` alone; (b) the
statement of `normality_never_certifies` in `Soundness.lex.tex` exchanged for
the statement of another theorem of the module, its proof kept. Command:
`cargo test -p repo-conformance --test conformance -- conformance_gc_09`.

```text
thread 'conformance_gc_09' panicked at crates/conformance/src/cases/certification.rs:2353:9:
assertion `left == right` failed: LexLeanCertification.schema_cheaper: exactly the registered axioms
  left: Some(["Classical.choice", "Quot.sound", "propext"])
 right: Some(["propext"])
thread 'conformance_gc_09' panicked at crates/conformance/src/certification_library.rs:317:9:
the module `LexLeanCertification.Soundness` was rejected by Lean:
<staging>/src/LexLeanCertification/Soundness.lean:516:0: error: native core declaration 'LexLeanCertification.normality_never_certifies':
```

Removed: both edits were reverted. Then, for `conformance_gc_11`: (c) a Lean file
`compiler/proofs/Soundness.lean` added; (d) the check in `check_sources`
(`crates/conformance/src/certification_library.rs`) that a module imports only
what the library is stated against disabled by prefixing its condition with
`false &&`; (e) `library_text` made to return the generated text with one
character appended, a generated module that is not what the project builds.

```text
thread 'conformance_gc_11' panicked at crates/conformance/src/cases/certification.rs:2653:13:
<root>/compiler/proofs/Soundness.lean: an independently authored Lean file lies in the library project
thread 'conformance_gc_11' panicked at crates/conformance/src/cases/certification.rs:2761:6:
a foreign import is refused: ()
thread 'conformance_gc_11' panicked at crates/conformance/src/cases/certification.rs:2705:9:
assertion `left == right` failed: Trials: the project's generated Lean is the Lean the kernel check compiles
```

A Lean file `examples/Planted.lean` declaring `namespace LexLeanCertification`,
outside the library's project, is refused by the same case, the permanent
condition `VR-19` states of the Atlas held over the whole release tree:

```text
thread 'conformance_gc_11' panicked at crates/conformance/src/support.rs:63:9:
<root>/examples/Planted.lean: an independently authored Lean file declares `namespace LexLeanCertification`
```

Removed: every edit was reverted and the cases pass. Three more defects are
planted by `gc_09` itself, each against a staged copy and each required to be
seen: a copy of the generated `Certification` whose strict test is weakened
(`Nat.ble` for `Nat.blt`) is valid Lean, and the library's proofs of the
lower-bound route are rejected over it; a false lemma, `schema_cheaper` given
the statement of `improves_preserves` and keeping its own proof, is planted in
a staged copy of `Schemas` beside the library's compiled modules, and Lean's
kernel refuses it; and `schema_cheaper`'s proof replaced by `sorryAx` is
accepted by the kernel and is seen in what Lean prints of its axioms. The
GC-02 module of false claims gained a fifth, the preservation of an ineligible
reduction (`Certification.certificateReductionsPreserve` is `false` for it).

### Runtime added by GNAF certification

Measured in the reference container (4 vCPUs) on the final tree, alone unless
stated. `conformance_gc_01` 16 s, `conformance_gc_02` 414 s, `conformance_gc_09`
about 115 s (the library compiled from its generated Lean, its instances,
`leanchecker`, the weakened checker, the false lemma, and the planted axiom; the
generated environment is compiled once per machine and shared; the library
migration of #79 moved this from 80 s), `conformance_gc_11` 55 s (two builds of
the library project), and every other `GC` case under 3 s. Most of `gc_02` is not added work: it
awaits the one shared verification of the `compiler` project that every case
shares, and the adversarial review's accounting of the marginal cost, against
the same suite at the base, is that shared verification (+37 s),
`gc_02`'s own 19 s of planted verdicts, `gc_09`, and `gc_01`, with
`conformance_gn_02` within the noise of the box: about two to four minutes of
`cargo test`, and about 40 s more for
the `compiler` project in `verify-examples`. The record of the change that stops a planted copy reaching the certificate
modules (a measurement, not a saving to quote: repeated alone, `conformance_gn_02`
takes 424 s to 451 s with and without it, which is the noise of the box):

| Case | Without | With | Change |
| --- | --- | --- | --- |
| `conformance_gn_02` (planted `GnafFixtures` in a copy of the project) | fails: the certificates name requests the planted module does not state | 430 s | needed for correctness, no saving claimed |
| `conformance_tc_03` (planted `TargetFixtures`) | 616 s | 624 s | none: the copy fails before the certificate modules are reached |
| `conformance_rs_12` (planted `ReasoningFixtures`) | 441 s | 442 s | none, same reason |

so the change is kept for `gn_02` only, where the copy would not verify without it. Replacing the project-copy plants by
`library::compile` against the built modules, as `gc_02` does for its planted
verdicts, would save about 140 s of CPU per copy, but the copies are asserted
to fail with `LLV7002` and to name the planted theorem, which a raw Lean
compile does not produce; it is not done here. The full
`cargo test --workspace --all-features --no-fail-fast` took 4272 s on the final
tree, with the 325 conformance tests in 3847 s, and `cargo xtask
verify-examples` took 371 s for the `compiler` project and 1333 s for the nine
language-1.2 examples and the `compiler` project in all, without `uor-atlas`.

## Runtime of `just vv` (Issue #25, merged with #31)

The acceptance gate is one job with a limit of 360 minutes (`ci.yml`), and the
job of this branch at `03a58a6` (run 37946936724) was cancelled at that limit
while `repro` ran, with `deny` still to come. The runner log gives where the
time went (UTC, 2026-10-09; the runner has 4 vCPUs and 16 GB):

| Stage | Start | Took |
| --- | --- | --- |
| `fmt-check`, `model`, `spec-links`, `lint` | 14:49:04 | 1.6 min |
| `test` (`cargo test --workspace --all-features`) | 14:50:40 | 82.3 min: `property` 5.7, `conformance` 70.0 (4199 s), `zz_scratch` 6.0 |
| `features` | 16:12:58 | 0.3 min |
| `bdd` (`cargo test -p repo-conformance`) | 16:13:13 | 78.9 min: `conformance` 71.5 (4288 s), `zz_scratch` 6.7 |
| `examples`, the eleven examples without `uor-atlas` | 17:32:04 | 12.0 min |
| `examples`, `uor-atlas` | 17:44:03 | 182.5 min |
| `examples`, `compiler` | 20:46:34 | 1.9 min |
| `golden` | 20:48:26 | 0.1 min |
| `repro` | 20:48:30 | cancelled at 20:49:15 |

PR #50's complete run took 4 h 18 min and the run of the tree before it
(`6b198ee`) 3 h 48 min, of which `uor-atlas` is the same 3 hours. Of the 360
minutes, 79 were the second run of the conformance suite by `bdd`, 12 were the
scratch probe `zz_scratch` (deleted in the commit before this one), and the
suite itself took 70 minutes of a machine that was a little more than half
busy.

What was found and changed, in the order of what it saved (measured on the
reference container of this repository, 4 vCPUs and 16 GB):

- **`bdd` ran the whole suite again.** `test` is `cargo test --workspace`,
  which runs every target of `repo-conformance`, and `bdd` was `cargo test -p
  repo-conformance`, the same targets. `bdd` is now `cargo test -p
  repo-conformance --test bdd`, the meta-gate (R2, R3, §27.7, §27.8) that it
  exists for; every conformance test still runs once, in `test`. SPEC.md §9.2,
  the recipe, `RP-05`'s table of recipe lines, and the table at the top of this
  file say so. Saved: 79 minutes of CI.
- **Every verification in the test process took a turn.** `support::env_lock()`
  was one mutex held for the whole of every `verify`, so the Lean runs of the
  suite's cases ran one at a time on four cores. It is now a read-write lock:
  the only code that changes the process environment, `with_env`, holds it
  exclusively, and every test that merely resolves a toolchain holds it
  shared (`env_shared`), so verifications run side by side. Nothing that
  changes the environment runs beside anything that reads it, as before.
- **The preservation environment was compiled for every workspace.** The
  calculus modules and the library of `language/preservation-1.2/` take 86
  seconds of the pinned Lean, and the harness staged and compiled them again in
  every workspace it checked (the four corpora, the plants, the renderings).
  `preservation::shipped` compiles them once per toolchain and library text,
  into `target/debug/lexlean-preserve-shipped/<hash>`, published by renaming a
  finished directory, and the workspaces' `LEAN_PATH` ends in it; each
  workspace still compiles and replays every module that depends on a project,
  and `leanchecker` replays what a certificate imports. `conformance_sp_03`
  went from 776 s to 401 s (cold) and 315 s (warm).
- **The corpora were certified one after another, and the cases waited for
  them.** `reports()` runs the four projects side by side (each holds about two
  gigabytes and waits on one Lean process at a time), and three of the suite's
  four test threads no longer wait on it for fifteen minutes. `conformance_sp_02`
  starts its independent parts together (the stress families, the verification
  of the 180-arm match, the corpora) and certifies the stress families on four
  workers: 1156 s to 593 s alone. `conformance_sp_05` verifies the example while
  two of the rejected fixtures run; `conformance_ex_07` runs its fixtures four at
  a time (the ones with a substituted toolchain take the lock exclusively, as
  they must); and the six rejection fixtures that `EX-07`, `SP-02`, and `SP-05`
  all read are run once (`fixtures::observe_shared`).
- **`uor-atlas` is unchanged.** It is 182.5 minutes of the 360, every module
  of it elaborated, replayed, and audited one process at a time
  (`PROCESS_WIDTH = 1`) because one of its environments approaches the
  memory of the runner; overlapping two made the hosted runner lose its
  control-plane heartbeat in the past. Nothing here touches it.

Before and after, on the reference container, `cargo test --workspace
--all-features` (315 conformance tests, all passing):

| | before | after |
| --- | --- | --- |
| `property` target | 322 s | 298 s |
| `conformance` target | 4085 s | 3013 s |
| whole `test` stage | 4483 s | 3373 s |
| `bdd` | 4085 s again | 1 s |
| mean CPU use of the four cores | n/a | 69 % |

The other stages of `vv` on the reference container: `fmt-check` 2 s, `model`
57 s (a build), `spec-links` under 1 s, `lint` 28 s, `features` 19 s, `golden`
61 s, `repro` 121 s, `deny` 6 s; `cargo xtask check-fixtures` (run by
`conformance_ex_07` as well) 573 s.

`cargo xtask verify-examples` on the reference container, after the last edit
(every example verified, and its normalized records equal `expected/verify`):

| Example | Took |
| --- | --- |
| `collections`, `higher-order`, `language-1.2`, `list-induction`, `nat-add-zero`, `propositional-logic`, `recursion`, `recursive-data` | 7 to 12 s each (71 s together) |
| `peano-arithmetic`, `semantic-1.1` | 13 s, 17 s |
| `production` | 124 s |
| `models` | 158 s |
| `reasoning` | 201 s |
| `production-coverage` | 245 s |
| the fourteen without `uor-atlas` | 825 s (13.8 min) |
| `compiler` | 250 s (4.2 min) |
| `uor-atlas` | 14876 s (248 min), at a peak of 14.4 GB with 8 GB of swap added to the 16 GB container; the runner took 182.5 min |

`uor-atlas` depends on nothing this change touches (it has no preservation
certificates and its sources, lock, and goldens are byte-identical to
`origin/main`), so its runner time is carried over as measured.

Projected `just vv` on the runner, from the measured stages of the runner log
and the ratio of the runner to the reference container in the one stage both
ran, the conformance suite (4199 s there for 301 tests, 4085 s here for 315
before the changes above):

| Stage | Runner, measured at `03a58a6` | Projected, merged tree |
| --- | --- | --- |
| `fmt-check` .. `lint` | 1.6 min | 2 min |
| `test` | 82.3 min | 58 min (3373 s x 1.03, the ratio of the one stage both ran) |
| `features` | 0.3 min | 0.3 min |
| `bdd` | 78.9 min | 1 min |
| `examples` without `uor-atlas`, with `compiler` | 13.9 min | 27 min (13.8 + 4.2 min, taken at 1.5 times for the runner) |
| `uor-atlas` | 182.5 min | 183 min |
| `golden`, `repro`, `deny` | not reached | 5 min (0.1 + 2 + 0.1 min at 1.5 times, rounded up) |
| Total | over 360 min | 276 min (4 h 36 min); at 2 times on the examples, 288 min |

The projection assumes the runner's `uor-atlas` time as it was and a runner
as fast as the reference container on everything else, which the one stage
that both ran supports; it is evidence for the margin, and the first complete
run on the runner is the measurement.

### the lock, the shared environment, and the parallel cases can fail

Each of the changes above leaves a gate in place that it could have made
vacuous, so each was made to fail again after the change.

- `bdd`'s recipe is pinned by `RP-05`. Planted: the recipe restored to `cargo
  test -p repo-conformance`. Command: `cargo test -p repo-conformance --test
  conformance -- conformance_rp_05`. Observed, then removed:

  ```text
  thread 'conformance_rp_05' panicked at crates/conformance/src/cases/repository.rs:325:17:
  assertion `left == right` failed: §9.2: `bdd` runs exactly the specified command
    left: ["cargo test -p repo-conformance"]
   right: ["cargo test -p repo-conformance --test bdd"]
  ```

  (Superseded by #48: `bdd` is now the whole suite, `cargo test -p repo-conformance
  --all-features`, and `test` excludes `repo-conformance`; the records of the
  current recipes are in "Execution of `just vv` (Issue #48)".)

- The shipped environment is still compiled by the pinned Lean, and a rejected
  module stops the harness. Planted: the text of every shipped module written
  with `def plantedBroken : Nat := "x"` appended (and the cache key changed, so
  that a cached build was not found). Command: `cargo test -p repo-conformance
  --test conformance -- conformance_sp_03`. Observed, then removed:

  ```text
  the shipped module `LexLeanTarget.TargetSyntax` was rejected:
  .../lexlean-preserve-shipped/3dd836fd....17031/src/LexLeanTarget/TargetSyntax.lean:162:27: error: Type mismatch
    "x"
  has type
  ```

  (A change to the library text itself changes the semantic ID, so every locked
  project is `LLC0102` stale before the harness is reached: the key of the
  cache is the hash of the text for the case that nothing else notices.)
- A rejected fixture is still reported when `conformance_ex_07` runs its
  fixtures on four threads. Planted: `LLL1004` in
  `tests/negative/unknown-word/expected/diagnostics.json` changed to `LLL1005`.
  Command: `cargo test -p repo-conformance --test conformance --
  conformance_ex_07`. Observed, then restored:

  ```text
  thread 'conformance_ex_07' panicked at crates/conformance/src/cases/examples.rs:550:13:
  fixture failures:
  .../tests/negative/unknown-word/expected/diagnostics.json differs from the observed run (§28.3: golden output changes only through `just fixtures-write`)
  ```

- A rejection fixture is still checked when `conformance_sp_05` runs two at a
  time. Planted: `("preservation-drift", "LLV7014")` expected as `LLV7013`.
  Observed, then restored: `assertion `left == right` failed: preservation-drift
  left: ["LLV7014"] right: ["LLV7013"]`, raised in the worker and carried to the
  case.
- A failing stress family still fails `conformance_sp_02` when four workers
  certify them. Planted: the first assertion of the fitting families inverted
  (`assert!(certified.is_empty(), ..)`). Observed, then restored: `enumeration
  of 420 constructors certifies`, raised in a worker.
- The gate that no parameter can capture a name a certificate writes still
  reads the locals that linking generates. `assert_no_bare_globals` passes
  names that begin `_y`, the spelling of a local that linking generated (`__acc`
  of a model or a reasoner), because no source name begins with an underscore.
  Planted: `identifier` spelling such a local bare (`acc`). Command:
  `cargo test -p repo-conformance --test conformance -- conformance_sp_09`.
  Observed, then restored: `names a parameter could shadow: {"acc (line 101)",
  "acc (line 102)", ... "best (line 199)", ... "bias (line 101)", ...}`.
  Without the exemption the gate reports the `_yacc`, `_ybest`, and `_ybias`
  that the models and reasoning examples introduce, which is what it was added for.
- `names()`, the project whose parameters are spelled like every namespace
  a certificate writes, no longer tries the namespaces the backend writes bare
  (`LexLeanModels`, `LexLeanReasoning`): linking refuses those as binders, and
  the fixtures `binder-bool-literal-name` and `declaration-lean-name` hold
  that.

## Differential validation oracle (Issue #26)

SPEC.md §17.19 and `OR-01`..`OR-08`. The independent Lean 4 to Rust generator
`lean4-prod` (`https://github.com/auser/lean4-prod`, commit
`991956fe427bb646498fd0912c89b71be71e7beb`) is pinned in
`oracle/lean4-prod.toml` and run against the Lean LexLean generates. It is
validation evidence only: no authority row or ledger claim cites it, and no
identity, attestation, or certificate is computed from it or names it.

**What was observed** (reference container, 4 vCPUs, the pinned toolchains,
the oracle fetched by `git` at its pinned commit into an empty cache and built
by the pinned Lake and Cargo, its crates.io dependencies fetched `--locked`
into a Cargo home of the cache's own; the same files from the checkout the pin
names hash to the same digest, c0d76452...a4a93a7, 165 files):

| | |
| --- | --- |
| roots in the corpus | 102: 6 of `production`, 34 of `production-coverage`, 9 of `models`, 8 of `reasoning`, 38 probes, 7 rejected fixtures |
| the oracle's answer | generates Rust for 30, refuses 72: 66 of roots LexLean takes (42 escaping local function, 13 opaque lowering, 4 unresolved external call, 3 polymorphic type, 2 unbounded `Int`, 2 `Nat.succ`) and 6 of roots LexLean rejects (4 opaque lowering, 1 `export-refused`, 1 `codegen-OpaqueType`); `opaque-lowering` is 17 over the corpus. The seventh rejected root is generated (a literal beyond 64 bits) |
| crates that compile and run | 23, on 1411 inputs: the 337 seeded inputs of the RB-06/SP-11 differentials and 1074 boundary inputs (MIN and -1 of each signed width, every constructor of every parameter type, the program's own literals and their neighbours, delimiters); each prints the interpreter's outcome and LexLean's own rendering's, overflow included, with the same contract (modulo a borrowed list parameter) and no allocation in a `rust-core` root |
| runtime constructs of the registry | 128; 89 are realized in the closure of a compiled root (and exercised by the seeds as far as they reach: that is not a claim that every path through the construct ran) and 39 are rows of `oracle/known-non-overlap.toml` with 14 formal-only or erased constructs, 53 rows in all |
| known differences | 12 rows in `oracle/known-differences.toml` (version 2), each exact on its roots; the six rejected-and-refused roots are rows (`both-refuse:<kind>`) so that a refusal for another reason is a disagreement |

**Findings about the oracle** (recorded as known differences, not hidden): it
renders a pair projection as `.fst` or `.snd` on a Rust tuple, which rustc
rejects (six roots, including the probes for `term.first` and `term.second`);
it exports a natural literal beyond 64 bits that rustc then rejects, where
LexLean refuses the root first (`LLT4005`); and its domain, by its own
documentation, excludes the mathematical integer, monomorphization,
and escaping local functions, which is why 72 of the 102 roots are refused.
Nothing found disagrees with LexLean on a program both take, on the seeded and
the boundary inputs.

**Cost.** `conformance_or_01`..`08` run in one process with one shared corpus
run (and one statement of the eligible roots of each corpus, which the survey
and the audit of the boundary inputs share). Warm cache: 145 s wall for the
group, 7.3 CPU-minutes on the four cores (OR-02, OR-03, OR-05 wait for the
one run, about 100 s; OR-04 plants and builds 21 plants, 21 rewrites that mean
the same and 2 equivalent mutants in one workspace, 127 s; OR-06 builds nine
candidate crates, 44 s; OR-01, OR-07, OR-08 take 1.3 s, 5.3 s, 7.4 s). Cold
cache (a fetch of the pinned commit from GitHub, the Lake build, the Cargo
fetch and build of the oracle): 234 s wall before the roots were shared, 11.4
CPU-minutes. Phase one of this issue measured 150 s and 6.2 CPU-minutes warm;
the boundary inputs add about one CPU-minute, since the interpreter runs each
of them once (about a second over the corpus; stating the roots is the same
cost as before) and the generated harnesses have four times the inputs. Added
to `just vv`: at most a few minutes of wall time, since the group runs beside
the longest tests of the suite, and no verification of an example.

**Limits.** The overlap is a measurement of the pinned oracle on this corpus:
a construct the table lists as outside it is outside it for the stated reason
and the stated probe, and a probe of scope `closure` is a root of a certified
example whose closure holds the construct among others (artifact-backed
models, reasoners, the recursive-data helper, `less_than`), so its refusal is
not attributable to that construct alone. A construct counted inside the overlap
is realized in the closure of a root the oracle generated and rustc compiled;
whether the inputs reach it is the seeds' reach: derived from the types and the
literals of the root, audited for constructor coverage, and planted against for
the arithmetic edges the reviewer found, but a bounded set. The oracle is not
run on inputs it would need a different ABI for (a list result by output
buffer does not occur in the corpus). Recursion depth and the runtime's counted
work are not comparable and are not compared; a candidate rendering's *work* is
the rendering's own count of itself (`tick`), which a rendering that does not
count reports as none, and the allocations an allocator out of its reach counts
are the measure that does not depend on it (a rendering that does more and
neither counts nor allocates passes both: stated, not hidden). The pin makes the
oracle verifiable, not available: the third-party repository and the crates.io
dependencies are outside this repository's audits, and the residual risk and the
repin procedure are in SPEC.md §17.19. The oracle is fetched from the network
in CI, with three tries over about a minute for a transient failure; if it
cannot be, the gate fails on the normative host.

### audit-oracle-isolation can fail

Planted: the oracle's name in a comment on the first line of
`language/semantics-1.2.toml` (an input of the compiler-semantics ID), and
separately its pinned commit in a `$comment` of
`schemas/attestation-v2.schema.json`. Command: `cargo xtask validate-model`.
Expected: R2 names the file and the identifier.

```text
gate failed: R2: `crates/lexlean/language/semantics-1.2.toml` mentions the differential oracle (`lean4-prod`); the oracle is validation evidence and no input to a semantic ID, build ID, verified artifact, attestation, or optimality certificate (§17.19)
gate failed: R2: `crates/lexlean/schemas/attestation-v2.schema.json` mentions the differential oracle (`991956fe427bb646498fd0912c89b71be71e7beb`); the oracle is validation evidence and no input to a semantic ID, build ID, verified artifact, attestation, or optimality certificate (§17.19)
```

Removed: both restored; the audit reports 5251 files in 7 roots naming no part
of the oracle. `conformance_or_08` plants seven identifiers of the oracle (its
name, its Lake target, its commit, its tree digest, its git tree, its pin's
path, its pin's schema tag) in each of the seven places the identities and
records are read from, and observes the audit name the file each time; it also
removes a scanned file and observes the refusal to pass over an empty root, and
plants an `oracle` field in a build manifest, a preservation record, and a GNAF
certificate (valid as committed) and in the closed schemas of the attestation,
the semantic snapshot, and the compiler input.

### the differential can fail

Each of these was planted, the named test run with the oracle acquired,
observed, and restored. Command:
`cargo test -p repo-conformance --test conformance -- <test>`.

- The comparison cannot ignore a changed outcome. Planted: the outcome
  comparison of `differences` made unreachable (`if false && normalized(got) !=
  expected`). Observed, then restored: `conformance_or_04` failed with `a
  checked multiplication made wrapping in Production.Main.shapeArea is
  detected as Behavior ["outcome:overflow/value"]: found []`, and
  `conformance_or_03` with `assertion `left == right` failed: []` on the
  stand-in whose outcome differs.
- An oracle that is silent is not agreement. Planted: the vacuity test of
  `validate` removed. Observed, then restored: `conformance_or_07` failed with
  `a run over no root is unavailable, not validated`. Planted: the arm that
  reads an exporter's exit without a refusal marker made a `Refused` verdict.
  Observed, then restored: `an exporter that crashes must be unavailable, not
  an answer: Ok(Refused { stage: Export, message: "" })`. Planted: the gate's
  `Fail` arm taken on a host that is not the normative one. Observed, then
  restored: `the normative host fails on an unavailable oracle:
  NotValidated("the oracle was not acquired: planted")`.
- A checkout that is not the pin is refused. Planted: the last digit of
  `tree_sha256` in the pin. Observed, then restored: `OR-03: the oracle is
  unavailable (the checkout is not the pinned oracle: 165 files hash to
  c0d76452...a4a93a7; the pin names 165 files hashing to
  c0d76452...a4a93a8); that is not a validation, and on the normative host it
  fails the gate (§17.19)`.
- A difference no row names is a disagreement with both sides' evidence.
  Planted: `Coverage.Prims.sequences` removed from the roots of the
  known difference `borrowed-parameter`. Observed, then restored:
  `OR-03: LexLean and the oracle disagree.` followed by `Coverage.Prims.sequences
  Contract `borrowed-parameter``, LexLean's contract and the oracle's, its
  signature `["&[u64]", "alloc::vec::Vec<u8>", "u64"]`, and the path of the
  record (`oracle-evidence.json`) it was written to.
- A construct in neither the accepted roots nor the table is refused. Planted:
  the row of `primitive.map_insert` renamed. Observed, then restored:
  `conformance_or_02`: `primitive.map_inserted is a construct of the
  registry`; the other ways (a row for a construct a compared root realizes, a
  row whose probe is refused for another reason or accepted, a not-runtime row
  for a runtime construct and the reverse) are planted by the case itself.
- A rendering wrong on one input, and one that does more work, are refused.
  Planted: the outcome comparison of `accept_rendering` removed. Observed, then
  restored: `a rendering wrong on one input is refused: ()`. Planted: the work
  comparison removed. Observed, then restored: `a rendering that does more work
  is refused: ()`.
- Planted discrepancies in the oracle's crates (`conformance_or_04` plants 21
  in copies of the generated Rust of 13 roots, and each is judged under a crate,
  a harness, and a result of its own): a checked multiplication made wrapping,
  an addition replaced by a multiplication, a literal off by one, swapped
  branches, an inverted comparison, a dropped recursive arm, swapped parameters,
  a parameter widened from `u32` to `u64`, a dropped failure typing, and the
  edges: a checked division made wrapping (`MIN / -1`), a negation and an
  addition made wrapping at the bound, a truncating division made Euclidean, a
  shift made to wrap, a match arm's constant changed, a split limit one too
  small, characters counted as bytes, a saturating subtraction made wrapping, a
  checked addition made a panic, an append in the other order, and an
  allocation in a `rust-core` root. Each is detected with the kind and the name
  the case states. Beside each is its *null* rewrite, the same site written to
  mean the same (`+ 0`, `* 1`, `.map(|x| x)`, `2 + 2` for `4`), and two
  *equivalent* mutants (the quotient by zero, which the product that follows
  ignores, and an equality made an inequality under a conjunction with false):
  none of them is a discrepancy. Three plants are also run alone and found as
  they are in company. The unplanted crates of the same roots, built and run the
  same way, agree.


### the follow-up review of the differential: what was planted

The review of the first version found that two of the nine plants shared a root
with another and were judged by the later one's result, that the comparison was
bound to its seeds, that acquisition could be poisoned and had no retry, and a
number of smaller things. Each was fixed and each fix was planted against.
Command: `cargo test -p repo-conformance --test conformance -- <test>` with the
oracle acquired; every plant was restored.

- **Plants are independent.** The executor keys each run's crate, harness, and
  result by a key of its own (`Slot`), and refuses two runs under one key before
  it builds. Planted: `evaluate` keyed by the root (as it was). Observed:
  `conformance_or_04` failed with `the runs to execute have distinct keys; a
  shared key would let one run's result stand for another's`. The reviewer's
  confound is reproduced as a control: the null rewrite of a plant is the same
  site written to mean the same, and it must not be detected. Planted: the null
  rewrite of `a literal off by one` made a real change (`2 + 2` for `3`).
  Observed, then restored: `a literal off by one: its null edit (the same site,
  the same meaning) is not a discrepancy: [(Behavior, "outcome:value/value")]`.
  Three plants are run alone and found as they are in company, so what is found
  does not depend on what is built beside it.
- **The comparison is not bound to the production differentials' seeds.**
  Planted: the boundary inputs left out of `eligible_roots`. Observed, then
  restored: `conformance_or_04` listed, among the 21 plants, the four the seeds
  alone miss: `a checked signed division made wrapping (MIN / -1) in
  Coverage.Prims.fixedSmall`, `a match arm's constant changed in
  Coverage.Main.options`, `a split limit one too small in Coverage.Prims.text`,
  and `a saturating subtraction made wrapping in Coverage.Main.natOps`, each
  `found {}`. Planted: the pairwise cover limited to the singles. Observed,
  then restored: `a checked signed division made wrapping (MIN / -1) in
  Coverage.Prims.fixedSmall ... found {}` (the pair `-128, -1` is a pair).
  Planted: `Value::None` left out of the pool of an option. Observed, then
  restored: `conformance_or_03` failed with `none and error false occur
  together`. The audit of constructor coverage can fail too: the case removes
  the inputs that build `none` and observes the audit name `: none`.
- **Acquisition is atomic, self-healing, and bounded.** Against a local git
  remote that is the case's own. Planted: the removal of the staging directory
  on failure left out. Observed, then restored: `["source.fetching-15662"]`
  left in the cache. Planted: a cache that is not the pin kept. Observed, then
  restored: `a cache that heals: NotAcquired("... source is not the pinned
  oracle and cannot be removed")`. Planted: a permanent failure (a remote that
  does not exist) treated as transient. Observed, then restored: `the repin
  procedure is named: git fetch exited Some(128): fatal: '...' does not appear
  to be a git repository`. The unreachable remote is tried three times (`after 3
  tries`), the wrong content is a pin mismatch once, and `retry` is tested for
  two blips survived, the delays waited, and a permanent failure ended at once.
  The cold path was run in this container: an empty cache and a local checkout
  (the Lake build, `cargo fetch --locked` into the cache's own Cargo home, and
  the offline build) took 38 s, crates.io reachable; the whole group from an
  empty cache with the commit fetched from GitHub took 234 s of wall time.
- **The pin's toolchains are the repository's.** Planted: the comparison with
  `rust-toolchain.toml` skipped, then the one with `lean-toolchain`. Observed,
  then restored: `conformance_or_01` failed with `another Rust is a drift:
  Ok(())` and `another Lean is a drift: Ok(())`. The failure a drift gives
  names both values (`oracle/lean4-prod.toml states rust_toolchain `...` and
  rust-toolchain.toml pins `...``).
- **The tables are versioned by content.** `oracle/table-versions.toml` records
  the digest of the rows at each version. Planted: a word of a row of
  `known-differences.toml` changed without a new version. Observed, then
  restored: `the rows changed (their digest is `3588e785...`) but the table is
  still at version 2, which oracle/table-versions.toml records as
  `2b09c7bb...`; raise `version` and record the new rows_sha256 there`. The case
  also raises the version without recording it (`records it up to version`),
  records a version that changed nothing (`changed nothing`), and a layout that
  is no row (a comment) changes nothing.
- **A rejected root's refusal is compared.** The six rejected roots the oracle
  refuses are rows (`both-refuse:<kind>`). Planted: the key of the row for
  `LanguageTwelve.Main.positive` renamed. Observed, then restored:
  `conformance_or_03` failed on the table (the rows no longer match their
  version); planted in the code instead, the key `both-refuse-x:<kind>`,
  `conformance_or_03` failed with `assertion `left == right` failed` on the
  stand-in whose key is `both-refuse:opaque-lowering`.
- **Tools that overrun are ended with what they started.** Planted: the process
  group of a tool left out. Observed, then restored: `conformance_or_07` failed
  with `the child 27650 of the overrunning tool is ended with it`. The tools of
  that case that are not meant to overrun run under 120 s, the one that is
  under 1 s.
- **A rendering that does more work and counts none of it is refused.**
  Planted: the comparison of allocations removed. Observed, then restored:
  `conformance_or_06` failed with `a rendering that does more and counts none of
  it is refused: ()`.

## Execution of `just vv` (Issue #48)

Issue #48 changes how the gate is run on the standard GitHub-hosted runner
(`ubuntu-24.04`, 4 vCPU, 16 GB) and nothing it accepts. Every obligation of
`just vv` is still run from a clean checkout of the exact pinned toolchains,
with no cache of a build directory, object, verification tree, attestation, or
proof result, no path filter, and no recipe skipped; the verifier runs at its
default width of 1 on UOR Atlas; `honesty` and `reproducibility` stay separate
workflows; the four non-normative hosts stay in the matrix. What changed is who
owns which tests, how the verifier's process width is chosen and kept out of
every identity, what the run reports about itself, and two measured costs that
every build and every verification paid for nothing.

### Ownership of the conformance suite

Before this change `test` was `cargo test --workspace --all-features`, which
runs every target of `repo-conformance`, and (until #49) `bdd` ran them again;
after #49 `bdd` ran the `bdd` target alone and `test` owned the suite. The
ownership is now exact, as the recipes read:

| Recipe | Command | Runs |
| --- | --- | --- |
| `test` | `cargo test --workspace --exclude repo-conformance --all-features` | the tests of `lexlean`, `repo-model`, and `xtask`: libraries, integration targets, doctests |
| `bdd` | `cargo test -p repo-conformance --all-features` | the whole of `repo-conformance`: its library tests, the `bdd` target, the `conformance` target |
| `inventory` | `cargo xtask check-test-inventory` | nothing; it lists the tests of both recipes and of the whole workspace |

`inventory` is what keeps "excluded" from meaning "lost". It runs `cargo test
--workspace --all-features -- --list` (the reference: the one command `vv` ran
before the split) and the `-- --list` of the two recipes' own commands, read
from the Justfile, and requires (`repo_conformance::inventory::compare`) that
the lists of `test` and `bdd` are disjoint, that their union is the reference's
list, that no target of `repo-conformance` is in `test`'s list and every one is
in `bdd`'s (the binaries are those `cargo metadata` names for the package), and
that `bdd` lists `conformance_<id>` tests at all. On the tree of this change
(local run): 525 tests, 169 in `test` and 356 in `bdd`, disjoint, none lost.

It is a recipe of `vv`, and not a test, so that it is one gate with one line in
the log that runs once, in the normative order after both phases, and is not
repeated by the host matrix's whole-workspace run on the other hosts.
`conformance_rp_13` checks everything else about it: the recipe lines, that the
inventory follows both phases in `vv`, that this file and SPEC.md name the
commands that run, and the comparison itself on synthetic lists (a partition is
accepted; a test run twice, a test lost, and a misplaced target are refused).
The honesty workflow runs `cargo test -p repo-conformance --test bdd`, the
meta-gate target alone, because it installs no Lean; that is a different job,
not a second run inside `vv`.

### The verifier's process width

`PROCESS_WIDTH = 1` is gone. The number of `lean`, `leanchecker`, and audit
processes that run at once is the *verification process width*, an operational
resource profile (SPEC.md §22.11): read from `LEXLEAN_VERIFY_WIDTH` (unset or
empty: 1; a decimal integer from 1 to 64; anything else is `LLV7017` before any
work, `invalid-verify-width`), recorded nowhere, and part of no identity. It is
not a field of `lexlean.toml`, the lock, a request, or the command line
(`VerifyRequest` still has exactly its one field, `conformance_cl_15`). The
normative job sets nothing, so it runs at width 1; a width above 1 is for a host
whose memory is known, and is exercised in this repository only by the
conformance cases, on small projects. No width above 1 has been run on UOR Atlas
and none is claimed.

`verify::pool::run_ordered` runs the independent processes of a stage, at most
`width` at a time, and hands each result to the caller one at a time in item
order, whatever order the processes finish in. The stages and their independence
are the verifier's own, unchanged:

- elaboration proceeds by topological frontier; a module starts only after
  everything it imports has produced its `.olean`; the frontier is the barrier;
- replay starts after every elaboration, audit after every replay, so replay and
  audit do not overlap each other or elaboration; within each, the processes read
  staged `.olean` files and write none (replay) or write only their own record
  (audit member);
- results, records, staged files, the merge of observed axioms, the policy rows
  and the attestation body are produced in the canonical order, so none of them
  can depend on completion order; the failure reported is the first in that
  order, and no item later than a known failure is started (items are claimed in
  order, so a later failure position can never reopen a stage; the
  `stop_after` minimum is therefore not distinguishable from a plain store by
  any test, and `a_later_failure_does_not_reopen_the_stage` fixes the behaviour);
- width 1 is the previous behaviour literally: the stage runs on the calling
  thread and a process starts only after the result of the one before has been
  consumed, so a failure only the consumer sees (a missing `.olean`) stops the
  stage before another process starts (`width_one_alternates_job_and_consume`).

`conformance_vr_21` verifies the committed `peano-arithmetic` project (`Addition`
is imported by `Divisibility`, `Doubling`, and `Order`, which `Main` imports:
frontiers of one, three, and one modules) at width 1 and at width 3 and requires
the two published directories to be byte-identical, with equal attestation IDs,
observed axiom sets and policy rows, and process records in order. It reads the
telemetry of both runs for what the bytes cannot show: at width 1 no two child
processes overlap; at width 3 the three members of the middle frontier overlap by
more than 50 ms (a width that is applied does this with half-second processes
started microseconds apart; up to three attempts are made, each also compared
byte for byte with width 1, so that a host stalled for a moment is not a failure
and a width that is not applied, which never overlaps, is), the `Addition`
frontier ends before the three start and they end before `Main` starts, and the
elaboration, replay, and audit stages do not overlap one another (all within 5 ms
of clock rounding). It then verifies the committed `production` example (two
modules, six production roots) at width 1 and at width 2, the two runs together,
and requires the whole published tree, the attestation, the canonical compiler
input, the preservation record, the observed axioms and policy rows, and the
process records to be equal, and the compiler input to be the committed
`expected/verify` one; that jobs which finish in reverse order are consumed in
item order; and that a tampered published directory is refused at width 3 (the
reuse rule). The production pair costs two production verifications (about
2 minutes each, run side by side); it is what shows that no pooled stage reaches
extraction or the certificates. Alone, `conformance_vr_21` takes 176 s on the
reference container.

### Telemetry

With `LEXLEAN_CI_TELEMETRY=<dir>` the verifier appends JSON lines to
`<dir>/verify-telemetry.jsonl`: `verify_begin` (width, processors, memory),
`stage` (wall time, host memory and swap, the verifier's own peak resident set),
`frontier` (index, members, wall time), `process` (each Lean, `leanchecker`, and
audit process: stage, module or audit member, wall time, exit code, and, from
`/proc`, the peak resident set and swap of the process and what it started),
`completion` (the order in which the processes of a stage finished: the one place
completion order exists), and `verify_end`. `cargo xtask verify-examples` adds
`examples-telemetry.jsonl` (wall time per example, and of its `fmt`, `lock`,
`check`, `build`, and `verify` steps: the verifier's own clock starts after the
project has been read, linked, and rendered, which each of `check`, `build`, and
`verify` does again, so that time is the step less the verification it contains)
and prints, for each example, the digest of its published verified directory; the
conformance runner adds `cases-telemetry.jsonl` (wall time per `conformance_<id>`
case, passed or not).

Stages: `certificate-generation`, `toolchain-preflight`, `workspace-preflight`,
`staging`, `probe`, `elaboration`, `replay`, `audit`, `policy`, `extraction`,
`certificates`, `pdf`, `attestation`, `scratch-cleanup`, `olean-hash`,
`executable-hash`, `attestation-body`, `publication`.

Telemetry is write-only and noncanonical. Every measurement key is in a closed
vocabulary (`wall_ms`, `started_unix_ms`, `peak_rss_kib`, `peak_swap_kib`,
`mem_total_kib`, `mem_available_kib`, `swap_total_kib`, `swap_used_kib`,
`host_cpus`, `process_width`, `completion_seq`) that SPEC.md §22.12 states and
`conformance_vr_22` compares with the code. A JSON key of that vocabulary in any
file of a verified directory, or a file named as telemetry, fails `cargo xtask
verify-examples` for every example and `conformance_vr_22`
(`repo_conformance::evidence::telemetry_in`). A verification with telemetry on
publishes the bytes one with it off does, and a telemetry directory that cannot
be written changes no result.

`ci.yml` runs `just --timestamp --timestamp-format '%s%.3f' vv`, which stamps
each recipe's first command (that is how a recipe is timed without changing it),
samples the host every five seconds into `resources.tsv` (free memory, swap in
use, the largest resident set), points `LEXLEAN_CI_TELEMETRY` at `ci-evidence/`,
and, after the gate, runs `cargo xtask ci-summary`, which writes
`ci-evidence/summary.md`, prints it to the job log (an uploaded artifact cannot
be read from the log) and appends it to `$GITHUB_STEP_SUMMARY` when GitHub sets
it, cut there at 512 KiB, half of the 1 MiB GitHub accepts, and bounded by
construction because every table lists at most fifteen rows. The step carries
`if: ${{ !cancelled() }}`, so a failed `vv` still reports the recipes it reached
(`repo_conformance::evidence::publish`, which has the tests). The summary holds the wall time of
each `vv` recipe with the lowest free memory, highest swap, and largest process
while it ran; the fifteen slowest projects to verify; the time of each verification
stage; the fifteen slowest verifier processes with their peak resident set and
swap; the fifteen slowest examples and their steps; the fifteen slowest conformance
cases; the stages of the twelve slowest projects; and for
the longest single verification (which on the runner is `uor-atlas`) its stages
with the lowest free memory, highest swap and the verifier's own peak, its slowest
and largest processes by module, the processes that swapped, and its slowest
frontiers. The per-module rows of every verification are in
`verify-telemetry.jsonl` itself.
`ci-evidence/` is uploaded with the gate log.

### Waste found and removed

Found by reading the job and by the telemetry above; measured on the reference
container (4 vCPU, 15 GB), which is not the runner.

- **Every verification hashed the running executable.** The attestation records
  the SHA-256 of the running `lexlean`, and `verify` read and hashed it every
  time: 4.7 seconds in the debug builds that the test suite and `cargo xtask
  verify-examples` use. The new `executable-hash` stage showed it: a five-module
  project verified in 11.5 s, 4.7 s of it in that stage. The digest is now computed
  once per process, and `sha2` is compiled at `opt-level = 3` in the dev profile
  (`[profile.dev.package.sha2]`); the stage takes 0.3 s and the same project
  verifies in 7.4 s. The conformance suite verifies hundreds of projects in one
  process, and `verify-examples` sixteen. No value `sha2` computes changes.
- **Debug information that no gate reads.** `[profile.dev] debug =
  "line-tables-only"` (the test profile inherits it): backtraces keep file and
  line. Cold build of `cargo test --workspace --all-features --no-run`, 86 s
  against 61 s; the target directory 2.7 GB against 1.3 GB; the conformance test
  binary 279 MB against 136 MB. The runner's disk also holds the proof evidence.
- **A build step before the host matrix's tests.** Each leg ran `cargo build
  --locked --workspace --all-features` and then `cargo test` with the same
  arguments. `cargo test` builds, in the same profile (the test profile inherits
  `dev`; there is no `[profile.test]`), the library and the binary of `lexlean`
  (its integration tests run the binary: after `cargo test --workspace
  --all-features --no-run` on a clean target `target/debug/lexlean` exists) and
  every dependency, so the shipped crate that does not build fails the leg at the
  test step. What the build step added was the non-test compilation of `xtask`
  (no `cfg(not(test))` code exists in the workspace, and no host in the job runs
  `xtask`): 1.7 s on the reference container with the dependencies built, and one
  step of the four legs' set-up. That is small, and it is the only measurement
  claimed; the runners' figure is whatever the next run's step durations show.
  The step is removed and `conformance_rp_14` refuses a `cargo build` in
  `ci.yml`. SPEC.md §8.3 says that the host job builds through `cargo test`.
- **Looked at and left**: `features` (`cargo check --all-targets`) re-checks what
  `lint` (clippy) checked, 26 s cold, but is a recipe SPEC.md §9.2 names; the
  feature unification of `-p repo-conformance` against `--workspace` recompiles
  nothing (the inventory runs the three commands back to back without a build);
  `rustup toolchain install`, `just`, `cargo-deny`, and elan are each installed
  once per job; the shipped preservation environment (the calculus modules and the
  library, about 100 s of the pinned Lean) is compiled by every verification of a
  project with a production root, five of the examples, and is not shared between
  them: sharing it would be a proof result standing in for work the current clean
  checkout owes, which the acceptance rule forbids (the conformance harness shares
  it between test workspaces, as #49 describes, and each workspace still replays
  what it imports). No swap or memory setting of the job was changed: the 4 GB
  overflow file at `vm.swappiness=10` exists for Atlas's 14.4 GB peak at width 1
  on a 16 GB runner, and nothing measured here can show a smaller one is safe.

### Numbers

Local, on the reference container (4 vCPU, 15 GB, shared with other jobs, so the
long figures move by several percent between runs), recipes run in the order of
`vv` on the tree of this change merged with #69:

| Recipe | Took |
| --- | --- |
| `model` | 33 s (warm build) |
| `spec-links` | under 1 s |
| `test` | 387 s |
| `features` | 25 s (warm build) |
| `bdd` | 3541 s (330 conformance tests, the library tests, the meta-gate) |
| `inventory` | 7 s: 525 tests, 169 in `test`, 356 in `bdd` |
| `examples` without `uor-atlas` | 1218 s: the eight small ones 2 to 8 s each, `peano-arithmetic` 7 s, `semantic-1.1` 13 s, `production` 132 s, `models` 178 s, `reasoning` 234 s, `production-coverage` 281 s, `compiler` 342 s (12 s of it each in `check` and in `build`, which read, link and render the project) |
| `golden` | 61 s |
| `repro` | 162 s |
| `deny` | 1 s |

(`conformance_vr_21` now takes 176 s by itself: the peano pair at widths 1 and 3,
and the production pair at widths 1 and 2 run together. The longest cases of the
suite, from `cases-telemetry.jsonl`: `EX-07` 1383 s, `DF-12` 1083 s, `SP-02` 1027 s,
`GN-02` 943 s, `GC-02` 891 s; 333 cases, 13815 s summed over the four test
threads.)

`uor-atlas` was not run: its sources, lock, goldens, and width are those of
`3f9ce42`, and it takes four hours here. Every other example's normalized
records equalled its committed `expected/verify`, and the scan for telemetry in
the verified directory passed for each.

With `LEXLEAN_CI_TELEMETRY` set, the `bdd` and `examples` runs recorded their own
verifications: 142 in all (76 reached an attestation; the others are the negative
cases), 5716 s of verifier wall time summed over the test threads, of which the
`certificates` stage is 3550 s (the shipped preservation environment, about
100 s per verification of a project with a production root), `elaboration` 1464 s,
`replay` 350 s, `audit` 112 s, `extraction` 89 s, and `executable-hash` 0.43 s in
all, where 76 attestations at 4.6 s each would have been about 350 s. Paired runs
in one session, same machine: a five-module project, 11.5 s before and 7.4 s
after; the smallest production-rooted project (one root), 124.8 s before and
114.2 s after (114.0 s with telemetry on: the sampling costs nothing
measurable). The first complete telemetry of the recipes and of `uor-atlas` is the
first CI run's.

All of the figures above are local: this change was written on the reference
container, not on the runner. The runner's own are in the next subsection. The first complete run of
`ci.yml` produces the runner's numbers in its `ci-evidence/` (`summary.md`,
`vv.txt`, `resources.tsv`, `verify-telemetry.jsonl`, `examples-telemetry.jsonl`,
`cases-telemetry.jsonl`); the before figures for comparison are those of the last
green runs on main (3 h 25 min on `3f9ce42`, of which `uor-atlas` is 182.5 minutes
and is untouched by this change: same sources, same width; the later runs on
main were reported at 4 h 18 min and 5 h 02 min, and the first run of this branch
says where the time goes). The
projection is the `vv` of `9d32a95` less the hashing, less the build and link
time of the debug builds, plus the inventory's seven seconds and about three
minutes of CPU for the production pair of `conformance_vr_21`; the runner's run
decides, and a timing is never evidence of a semantic claim.

### Runner numbers (first full run, PR #72)

These come from the first complete run of `ci.yml` on the standard runner, the
run the paragraph above waited for: CI run 38201023383, job 114654244617
(`just vv`), `ubuntu-24.04` (4 vCPU, 16 GB), 2026-10-12, head `19e6d30` of
PR #72 (main at `9abdd71` and its follow-up). All seven jobs were green. They are
noncanonical evidence of cost, taken from the job's `ci-evidence/`; they prove
no semantic claim, and nothing in the repository's identities or gates depends
on them.

The job took 3 h 16 min (03:45:41 to 07:01:37 UTC); the recipes sum to 11711 s
(195.2 min).

| Recipe | Took (s) |
| --- | --- |
| `fmt-check` | 1 |
| `model` | 42 |
| `spec-links` | 0 |
| `lint` | 23 |
| `test` | 192 |
| `features` | 12 |
| `bdd` | 2357 |
| `inventory` | 4 |
| `examples` | 8985 (lowest free memory 37 MiB, highest swap 4774 MiB, largest process 15249 MiB) |
| `golden` | 32 |
| `repro` | 63 |
| `deny` | 1 |

Verification time by project, in seconds (the number of verifications): `uor-atlas`
8276.8 (1), `production` 1879.0 (23), `compiler` 489.7 (6), `reasoning` 328.0 (4),
`models` 240.6 (6), `nat-add-zero` 237.0 (51), `production-coverage` 155.4,
`semantic-fixture` 141.3 (6). Over all verifications the stages took
`elaboration` 5002.3, `replay` 4338.2, `certificates` 2267.8, `audit` 97.8,
`staging` 68.2, `extraction` 56.4, `probe` 40.9.

UOR Atlas is 8277 s of the `examples` recipe: `elaboration` 4193.2, `replay`
4070.5, `audit` 11.0. Its slowest modules: `CensusCert4` elaboration 1154.6 s (peak
RSS 15338 MiB, swap 2672 MiB) and replay 1121.4 s (15329 MiB, swap 2568 MiB);
`Census` 845.6 s and 818.3 s; `Group` 172.5 s and 166.2 s; `Scales` 160.4 s and
151.2 s; `Closure` 144 s and 144 s (about 15 GB resident). The slowest frontiers
by elaboration: #10 (22 members) 1628.8 s, #15 (16) 1045.6 s, #8 (3) 862.4 s. The
slowest conformance cases, of 333 and 9196 s summed thread time: `SP-02` 795.9 s,
`EX-07` 754.0 s, `DF-12` 532.5 s, `GC-02` 500.1 s, `SP-09` 407.3 s, `SP-03`
405.8 s, `SP-04` 403.0 s, `GN-02` 379.0 s.

The wall time of `vv` varies much more between runs of near-identical code than
any change of this issue moves it: 3 h 25 min on #49, 4 h 18 min on #50, 5 h
02 min on #69 (without telemetry), 4 h 16 min on #70, and 3 h 16 min on #72.
A single before and after is therefore not a meaningful comparison, and none is
claimed; what the run does show is where the time goes (`uor-atlas`, at 8277 s of
the 11711 s, with memory at the limit of the runner) and that the recipes other
than `examples` and `bdd` are a few minutes together.

### inventory can fail

Each record of this and the next two headings is a defect planted, the command
run, and the failure observed (verbatim, paths shortened), after which the defect
was removed.

- **inventory.** `test` restored to `cargo test --workspace --all-features` (the
  conformance suite runs in both phases):
  `gate failed: RP-13: 353 test(s) run in both phases, e.g. `a_hidden_conformance_test_is_flagged` of tests/bdd.rs [bdd]`
  (`cargo xtask check-test-inventory`). `bdd` restored to `--test bdd` (what #49 left):
  `gate failed: RP-13: 348 test(s) of the workspace run in neither phase, e.g. `conformance_ar_01` of tests/conformance.rs [conformance]`.
  `bdd` as `--all-features --test conformance`:
  `gate failed: RP-13: 32 test(s) of the workspace run in neither phase, e.g. `a_hidden_conformance_test_is_flagged` of tests/bdd.rs [bdd]`.
- **the recipes.** `bdd` restored to `cargo test -p repo-conformance --test bdd`,
  `cargo test -p repo-conformance --test conformance -- conformance_rp_05 conformance_rp_13`:

  ```text
  thread 'conformance_rp_05' panicked at crates/conformance/src/cases/repository.rs:329:17:
  assertion `left == right` failed: §9.2: `bdd` runs exactly the specified command
    left: ["cargo test -p repo-conformance --test bdd"]
   right: ["cargo test -p repo-conformance --all-features"]
  ```

  `test` restored to the whole workspace: `conformance_rp_05` reports `§9.2: `test`
  runs exactly the specified command` and `conformance_rp_13` the same pair; `inventory`
  dropped from `vv`: `§9.2: `just vv` runs the twelve gates in the normative order`.
### width-invariance can fail

- **completion order.** In `verify::pool::run_ordered` every result consumed as it
  arrives instead of in item order. `cargo test -p repo-conformance --test
  conformance -- conformance_vr_21`:

  ```text
  thread 'conformance_vr_21' panicked at crates/conformance/src/cases/operational.rs:312:5:
  assertion `left == right` failed: and are consumed in item order
    left: [3, 2, 1, 0]
   right: [0, 1, 2, 3]
  ```

  (`cargo test -p lexlean --lib verify::pool` fails two of its tests the same way.)
  End to end, with that defect built into `lexlean`, the committed peano project at
  `LEXLEAN_VERIFY_WIDTH=3` published attestation `09b0d13a…`, then `3a92ebff…`, then
  `2bf41a33…` on three runs, against `cadc86cb…` at width 1: the order of the three
  members of the middle frontier decided the bytes.
- **the import barrier.** Every module placed in the first frontier:
  `conformance_vr_21` fails at `Lean rejected `PeanoArithmetic.Main` (error):
  object file '$STAGING/oleans/PeanoArithmetic/Order.olean' of module
  PeanoArithmetic.Order does not exist` (`LLV7002`).
- **the width in an identity.** `("verifier_width", width)` added to the
  attestation body: `conformance_vr_20` and `conformance_vr_21` fail on the
  attestation ID of the same project at width 2 (and 3) against the default.
- **reuse.** The comparison with a published directory made to succeed whatever
  it finds: `conformance_vr_21` fails at `a tampered published directory is not
  reused`.
- **a width that is not applied.** `let width = operational.width().min(1);` in
  `verify::run` (the verifier ignores `LEXLEAN_VERIFY_WIDTH`; before the
  overlap assertions this passed every case, since two runs at width 1 are equal):
  `conformance_vr_21`:

  ```text
  thread 'conformance_vr_21' panicked at crates/conformance/src/cases/operational.rs:500:5:
  three attempts at width 3 never overlapped two members of a three-wide frontier: the width is not applied
  ```

### telemetry-scan can fail

- **telemetry in an artifact.** `("wall_ms", 1)` added to every process record
  `ChildRecord::to_json` writes: `conformance_vr_22` fails at
  `assertion failed: crate::evidence::telemetry_in(with.root.as_std_path()).expect("scans").is_empty()`;
  the same scan runs over every example in `cargo xtask verify-examples`.
### The CI job and the width variable can fail

- **a skipped pull-request gate.** The summary step given `if: ${{ !cancelled() &&
  github.event_name != 'pull_request' }}` (which a test accepting any condition
  containing `!cancelled()` let through): `ci.yml:116: the only condition allowed is a
  step's `if: ${{ !cancelled() }}``; a job-level `if: ${{ !cancelled() }}`:
  `ci.yml:49: the only condition allowed is ...`; the gate step given
  `if: ${{ !cancelled() }}`: `the summary step and the evidence upload carry it`
  (left: 3) (all `conformance_rp_14`).
- **a summary that is not in the log.** Each planted in
  `repo_conformance::evidence::publish` and `xtask/src/ci.rs`, and the unit tests
  (`cargo test -p repo-conformance --lib evidence`) and `conformance_rp_14` run:
  - the `writeln!(stdout, "{summary}")` removed: `the_summary_is_printed_to_the_log_even_when_the_gate_failed`
    and `the_step_summary_is_appended_to_when_the_variable_names_one_and_only_then`
    fail (the latter at `and still printed`);
  - the step summary read from another variable than `GITHUB_STEP_SUMMARY`:
    `the_step_summary_is_appended_to_when_the_variable_names_one_and_only_then`
    and `a_step_summary_that_cannot_be_written_is_a_warning_and_not_a_failure`
    fail;
  - `bounded` returning its input whole: `what_goes_into_the_step_summary_is_bounded_and_cut_at_a_line`
    fails;
  - the table of projects listed without its fifteen-row limit:
    `a_run_with_thousands_of_projects_and_examples_lists_only_the_slowest` fails
    (`the summary does not grow with the run`);
  - the task passing `&|_| None` as the environment, or `&mut std::io::sink()` as
    the output: `ci-summary prints to stdout and reads GITHUB_STEP_SUMMARY from the
    environment` (`conformance_rp_14`).
- **a build step restored to the host matrix.** `- run: cargo build --locked
  --workspace --all-features` before the matrix's `cargo test`:
  `ci.yml has no `cargo build` step: the host matrix's `cargo test` builds the crate`
  (`conformance_rp_14`). The summary step given no `if: ${{ !cancelled() }}` is
  refused by the condition count of the same case (the record above).
- **a host leg that stops at its first failing binary.** `--no-fail-fast`
  removed from the matrix's `cargo test`: `the host matrix runs the workspace's
  tests once, and reports every failing test of the host, not the first binary's`
  (`conformance_rp_14`). The defect is real, not hypothetical: the first run of
  the change that added `end_children` failed both macOS legs in about three
  minutes at that one binary, and nothing was known of any other test of those
  hosts until it was fixed and run again.
- **a width that is not Unicode.** `from_environment` reading the variable with
  `std::env::var(..).ok()` (a non-Unicode value then selects the default width, silently):
  `verify::operational::tests::the_environment_is_read_as_bytes` fails with
  `not Unicode: Operational { width: 1, telemetry: None }`.
- **the CI job.** `paths-ignore: ['**.md']` under `pull_request:`:
  `ci.yml has no `paths-ignore:``; the runner replaced by `${{ vars.BIG_RUNNER ||
  'ubuntu-24.04' }}`: `the normative job runs on the standard runner`; the gate step
  given `if: github.event_name != 'pull_request'`:
  `ci.yml:82: a condition other than !cancelled() on a step:         if: github.event_name != 'pull_request'`
  (all `conformance_rp_14`).
- **a malformed width.** `invalid-verify-width` (`LEXLEAN_VERIFY_WIDTH=0`) is a
  negative fixture of §28.5: `verify` exits 3 with `LLV7017` before the build is
  published, on every host.

## The CI gate in two legs

`just vv` runs its twelve recipes one after another, and `examples` is most of
that time: 8985 s of the 11711 s on the runner (CI run 38201023383, "Runner
numbers" above), 8277 s of it UOR Atlas, which runs one Lean process at a time
because its largest modules peak at about 15 GB on a 16 GB runner. Nothing the
other recipes check waits on it, yet the job made them wait.

`ci.yml` now runs the gate as one matrix job of two legs at once, each its own
clean checkout on `ubuntu-24.04` at width 1 with the same set-up: `examples`
alone, and the other eleven recipes in the order of `vv`, each leg through
`just --timestamp … ${{ matrix.recipes }}`, which stops at its first failing
recipe as `vv` does. `just vv`, its recipes, and its order are unchanged
(`RP-05`), and `release.yml` still runs it whole. The leg that runs `examples`
is the one that asserts that Atlas verified, chosen by its recipes and not by
its log. A leg ends its log with a stamped `vv part: every recipe passed`, so
that `cargo xtask ci-summary` times its last recipe to the end of the leg rather
than to nothing (`evidence::PART_PASSED_LINE`), and each leg uploads
`acceptance-gate-evidence-<leg>`. The wall time of the gate becomes the longer
leg: from the runner numbers, about 8985 s of recipes plus set-up against the
earlier 11711 s, so roughly 2 h 40 min instead of 3 h 16 min. That is an
estimate from one run's timings, not a measurement, and the variation between
runs recorded above is larger than either leg's set-up.

`RP-14` (SPEC.md §9.4) now reads the legs out of the matrix and requires that
together they run every recipe of `vv` exactly once, each leg in the order of
`vv`, through the one gate command, with the end line and the per-leg artifact
name; its other checks (the runner, the width, the time limit, no cache, no
path filter, no condition but `!cancelled()` on the two evidence steps) are
unchanged.

### the legs can fail

Each record is a defect planted in `ci.yml`, then `cargo test -p
repo-conformance --test conformance -- --exact conformance_rp_14`, and the
failure, after which the file was restored.

- **a recipe dropped from the legs** (`golden` removed from the second leg):
  `§9.4: the legs run every recipe of vv exactly once`.
- **a recipe run by both legs** (`examples` added to the second leg): the same.
- **a leg out of the order of vv** (`bdd` before `test`): `§9.4: the leg
  ["fmt-check", "model", "spec-links", "lint", "bdd", "test", "features",
  "inventory", "golden", "repro", "deny"] runs its recipes in the order of vv`.
- **the Atlas assertion keyed to the log** (`case "$(cat ci-evidence/vv.txt)"`):
  `the leg that runs examples asserts that Atlas verified`.
- **the end line of a leg dropped**: `a leg marks the end of its last recipe as
  the summary reads it`.
- **each leg running all of vv** (`vv` in place of `${{ matrix.recipes }}`):
  `every leg runs its recipes through the one gate command`.

The summary reads a leg's end line as it reads any last stamped line, so no
branch of `recipe_spans` is added for it and none could be planted;
`a_leg_of_the_gate_ends_its_last_recipe_at_the_leg_s_own_line` fixes how a leg's
log is timed.

## The examples leg at a resource profile (Issue #83)

Issue #83 asks for a shorter gate without weaker proof obligations. On the
runner the gate's wall time is the `examples` leg: 3 h 14 min on `main` at
`d284b165` (CI run 38520744224), 10,188 s of it the verification of UOR Atlas
at width 1 (elaboration 5,102 s, replay 5,029 s, audit 15 s).

### What the runner allows

From that run's telemetry (`verify-telemetry.jsonl` of
`acceptance-gate-evidence-examples`) and Atlas's import graph, a schedule
model gives Atlas's verification time at each profile. Its inputs are
measurements; its outputs are estimates.

| profile | Atlas, minutes |
| --- | --- |
| width 1 (the run) | 169 |
| width 2, hints, 12 GiB budget, frontier barriers (§22.11 as written) | 156 |
| width 3, the same | 154 |
| width 2, no budget (a host with memory to spare) | 104 |
| width 4, no budget, no frontier barriers, replay of a module right after its elaboration | 66 |

Memory, not processors, bounds the standard runner. 22 of the 66 modules peak
above 8 GiB of resident set and 4 above 12 GiB (`CensusCert4` 14.9 GiB and 1,308
s of elaboration, `ClosureOrbit` 14.9 GiB, `CensusCert10` 14.3 GiB, `Closure`
14.3 GiB), and three of them went into swap at width 1. On 16 GB no two of
those can run together, so a profile on this runner buys about 8 % of Atlas.
The larger gains need a host with more memory (§9.4 rules a larger runner out)
or lighter Census proofs: `CensusCert4` is one `of_decide_eq_true` over
`gramRange 4000 1000`, native core data whose authoring surface is #93. The
conformance suite is already run about four wide by the test harness (on the
same run, 370 cases took 12,536 s in a `bdd` of 3,200 s), and it is on the
shorter leg.

### What changed

`ci.yml`'s matrix row for `examples` names `width: "2"`,
`memory_kib: "12582912"`, and `hints: ".github/verify-hints.json"`, and the
job reads `LEXLEAN_VERIFY_WIDTH`, `LEXLEAN_VERIFY_MEMORY_KIB`, and
`LEXLEAN_VERIFY_HINTS` from the row; the other row names three empty values,
the default. The hints are those of run 38520744224, derived by `cargo xtask
ci-hints --from verify-telemetry.jsonl --out .github/verify-hints.json`: 136
modules of the sixteen examples, each with the largest `wall_ms` and
`peak_rss_kib` observed for its name. A module two examples share is scheduled
as the heavier one, and a module the hints do not name is expected at no
memory, which is a schedule, never a verdict.

Nothing the leg proves moves. Every elaboration, replay, and audit member runs
as before; the leg still requires each example's normalized verification
records to equal `expected/verify`, which width 1 wrote, so a profile that
changed one byte would fail the gate. `release.yml` runs `just vv` with no
profile, which verifies Atlas at width 1. The sentence of the #48 section above,
that no width above 1 has been run on UOR Atlas, describes the tree before this
change.

`RP-14` reads the profile out of the matrix. The examples leg must name
exactly that width, budget, and hints file; the width and the hints must parse
with the verifier's own functions (`Operational::parse_width`,
`Operational::parse_hints`), and the hints must name `Census`; any other
leg must name none; each variable is set once, from the row; and no profile may
come from `vars.` or `inputs.`.

### the profile pins can fail

Each record is a defect planted in `ci.yml` or the hints, then
`cargo test -p repo-conformance --test conformance -- --exact
conformance_rp_14`, and the failure, after which the file was restored.

- **the examples leg at width 3**: `§9.4: the examples leg's profile`.
- **the rest leg at width 2**: `§9.4: a leg that does not run examples names no
  profile`.
- **the budget dropped from the examples row**: `§9.4: the row … names
  `memory_kib``.
- **the width written on the job** (`LEXLEAN_VERIFY_WIDTH: "2"`): `§9.4:
  `LEXLEAN_VERIFY_WIDTH` is read from the matrix row`.
- **the width set again on the gate step**: `§9.4: `LEXLEAN_VERIFY_WIDTH` is set
  once`.
- **the width from a repository variable** (`${{ vars.W }}`): `the gate selects
  no runner and no profile from outside the file: `vars.``.
- **the hints file cut in half**: `the committed hints are a hints file the
  verifier reads` (`LLV7017`, EOF while parsing).
- **the hints file empty** (`{}`): `the hints cover the Atlas module that
  dominates the leg`.

`ci::tests::hints_keep_the_heaviest_observation_of_each_module` fixes the
derivation: only elaboration and replay processes, the largest of each field
per name, and the same bytes for the same telemetry.

## Shared production verification (Issue #141)

`SP-05` reads the verified directory retained by the production-export
harness's `exported("production")` fixture, which `PX-01` already uses.
That fixture owns the project copy and lazily exports it through the consumer
and `production::build`, whose unchanged verification pipeline elaborates,
replays, and audits the source and its certificates. A crate-private harness
helper locates that directory from the bundle's build root and attestation
identity. The stable consumer still uses only the public production API;
the conformance harness owns this access to the retained verification files.
The shared setup also plants an interrupted staging directory containing a
leftover Lean file and asserts that the real export removes it, preserving
the cleanup probe previously supplied by `support::verify_ok`.

Every positive artifact, schema, digest, and attestation assertion of
`SP-05`, its six rejection observations, its host gate, and its parallel
setup are retained. When `SP-05` runs alone it initializes the same real
export; together with `PX-01` the fixture's existing `OnceLock` supplies one
verified run. The independent second runs for verification determinism and
offline export are unchanged, as are all acceptance recipes, resource
profiles, source artifacts, and semantic identities. The saved verification
belongs to `bdd`; this change does not shorten Atlas's proof work.

With one Cargo worker and `LEXLEAN_VERIFY_WIDTH=1`, `cargo test --offline
--locked -p repo-conformance --all-features --test conformance --
conformance_sp_05 conformance_px_01 --exact --test-threads=1 --nocapture`
passes both cases before and after the change: 1225.31 and 1060.20 seconds
of test execution, respectively. The changed case also passes alone in a
fresh process, with `conformance_sp_05` as the sole filter, in 749.99 seconds.
These are observations on a shared development host with warmed Cargo state,
not a controlled benchmark or a measurement of the whole CI gate. The saved
pipeline invocation follows from the shared fixture, independently of those
timings; no timing threshold is an acceptance condition.

Formatting, workspace/all-target Clippy with warnings denied, the model and
spec-link audits (380 IDs, 80 codes), and all five `bdd` meta-tests pass.
`cargo xtask check-test-inventory` confirms the same 632 workspace tests:
220 in `test` and 412 in `bdd`, disjoint with none lost.

## Production export contract (Issue #28)

SPEC.md §24.7; `PX-01`..`PX-12` and `CL-23`. The Lean-backed cases share three exports in one process (the consumer's exports of `examples/production` and `examples/production-coverage`, and a second, offline CLI export of `examples/production` from another directory): 4 min 08 s and 2.2 GB peak RSS for all thirteen cases together on the development host (`cargo test -p repo-conformance --test conformance -- conformance_px_ conformance_cl_23`). The committed packages under `tests/production-export/` are regenerated only by `just production-export-write`.

### production export can fail

The production export contract (§24.7, `PX-01`..`PX-13`, `CL-23`). Each record
below is a defect planted in a clean tree at the commit that introduced the
contract, the command run, and the failure observed (verbatim, paths
shortened), after which the defect was removed.

- **a forged certification field accepted.** `#[serde(deny_unknown_fields)]`
  removed from `ProductionRequest` in `crates/lexlean/src/api/production.rs`,
  so a request carrying `certification` deserializes.
  `cargo test -p repo-conformance --test conformance -- conformance_px_06`:

  ```text
  panicked at crates/conformance/src/cases/production_export.rs:890:44:
  a request with `certification` deserialized
  ```

- **a stale digest accepted.** The comparison of each pinned identity with the
  project's made vacuous (`pinned != actual && pinned == Sha256Digest([0; 32])`).
  `-- conformance_px_04` runs Lean on the stale request and then fails:

  ```text
  panicked at crates/conformance/src/cases/production_export.rs:126:9:
  LLC0204: `.lexlean/build` was published
  ```

  The run after verification still refuses a changed source (the defence in
  depth of `build_inner`), so what the plant lets through is the stale request
  reaching Lean and publishing; the case requires that nothing is.

- **a path escape accepted.** The evidence reader's `project.confined_file(relative)`
  replaced by `project.root.join(relative)`. `-- conformance_px_09`: the
  absolute evidence path outside the project is read, and the request fails
  later, on the certificate, instead of at the escape:

  ```text
  panicked at crates/conformance/src/support.rs:583:5:
  expected LLS8001, found ["LLV7010"] (LLV7010: evidence `evidence/sum.certificate.json`: No such file or directory (os error 2))
  ```

  A first plant that removed only an explicit `is_project_relative` check in
  front of `confined_file` was *not* detected (`conformance_px_09` passed):
  `confined_file` already refuses an absolute or parent-relative path, so the
  check was redundant and was deleted rather than kept untested.

- **a schema drift without a version bump.** The `title` of
  `api/schemas/production-result.schema.json` changed, `versions.toml` and the
  spec string unchanged. `-- conformance_px_12`:

  ```text
  panicked at crates/conformance/src/cases/production_export.rs:1320:79:
  production-result: the schema changed under `lexlean/production-result/1` (recorded 85dbe9bde4f190a693f20a53d3e7397bba5d449033f6630fb9753cf938c7334d, now 3272fe0d0c8545d69f6d8d08b20ca8ae07777dc554480be004f07f57bf9c7538); a changed schema needs a new spec version
  ```

- **a nondeterministic field in the result.** `compiler.version` set to the
  crate version followed by the absolute project root. `-- conformance_px_08`
  (two exports, from two absolute directories and two working directories):

  ```text
  panicked at crates/conformance/src/cases/production_export.rs:1094:13:
  the two bundles are byte-identical; these differ: ["result.json", "result.json"]
  ```

  (The list was a chain of both sides' keys; it is a set since.)

- **a reach into an internal module.** Armed in the case itself:
  `conformance_px_01` appends each of `use lexlean::production::lower;`,
  `lexlean::verify::run(`, `use lexlean::{Engine, calculus};`, and
  `lexlean::api::render_build(` to the consumer's source and requires the
  stable-surface gate to report it; the hidden-module list is read from
  `lib.rs`, so a new internal module is covered without editing the gate.

- **replay.** `conformance_px_07` is its own plant set: eleven single-byte
  tampers (package library and manifest, crate, program, certificates A, B,
  and E, attestation, compiler input, preservation record, result), fifteen
  field tampers with the result identity recomputed (semantic, source,
  attestation, and build IDs, compiler semantics, target digest, entry, root
  index, package name, export name, a certificate theorem, the certificate
  order, the axioms, an optimality claimed without evidence, the GNAF verdict),
  an added file, and a removed file are each `LLV7102`, and a symlink is
  `LLS8001`.

- **an output's effects not checked against eligibility** (`PX-13`). The
  comparison of a result's per-output effects with the root's eligibility was
  planted to ignore `recursion` (`kept(effects) != kept(required)` in
  `effects_agree`, `crates/conformance/src/cases/production_export.rs`).
  Command: `cargo test -p repo-conformance --all-features --test conformance
  conformance_px_13` (Windows, toolchain hidden, so the platform-independent
  assertions ran). Expected: an output with `recursion` dropped is accepted.

  ```text
  thread 'conformance_px_13' (22224) panicked at crates\conformance\src\cases\production_export.rs:1501:60:
  dropped: ()
  ```

  Removed: the comparison was restored; `conformance_px_13` passes. On a host
  with the pinned toolchain the case also exports `production-coverage` and
  requires each of its 49 outputs to state exactly its eligibility's effects.

## Template recursion (Issue #105)

SPEC.md §17.13, §17.14; `TC-06`, `PD-05`. The collection, fold, iteration, and
graph primitives are realized by library templates that call themselves, so the
Rust a root renders recurses on the stack once per element, and a root that
scanned an 80,000-byte name aborted on an 8 MiB stack while its registry rows
said `recursion = false` and its declaration admitted no `recursion`. The rows
are corrected, `conformance_tc_06` derives each runtime row's `recursion` from
its realization instead of trusting the row (the structural `recursion`
reference, or a library template whose instance reaches itself through calls
and closures), and `PD-05` refuses a root that iterates and does not admit the
effect. Each record below is a defect planted in a clean tree, the command run,
and the failure observed (paths shortened), after which the defect was removed.

- **a template-realized row that says `recursion = false`.** The registry row of
  `primitive.list_fold` set back to `false`.
  `cargo test -p repo-conformance --test conformance -- conformance_tc_06`:

  ```text
  panicked at crates/conformance/src/cases/calculus.rs:1224:13:
  recursion disagreements: [
      "primitive.list_fold: registry false, realization true",
  ```

- **a runtime-item row that says `recursion = true`.** The row of
  `primitive.map_size` (realized by the `length` item, a loop) set to `true`,
  so the claim cannot only be raised. `conformance_tc_06`:

  ```text
  panicked at crates/conformance/src/cases/calculus.rs:1224:13:
  recursion disagreements: [
      "primitive.map_size: registry true, realization false",
  ```

- **a derivation that finds no recursion.** `calls_itself` made to return
  `false` before it reads a function, so the check agrees with whatever the
  registry says about a template. `conformance_tc_06`:

  ```text
  panicked at crates/conformance/src/cases/calculus.rs:1224:13:
      "primitive.map_insert: registry true, realization false",
      "primitive.map_remove: registry true, realization false",
      "primitive.map_lookup: registry true, realization false",
  ```

- **an eligibility analysis that ignores the recursion a primitive row implies.**
  The check in `eligibility.rs` that turns a registry row into the realized
  effect `recursion` skips every `primitive.*` key, so a root that iterates and
  does not admit the effect is eligible. `conformance_pd_05` (the committed
  `iterations` root of `examples/production-coverage` with `recursion` removed
  from its declaration must be `LLT4005`):

  ```text
  panicked at crates/conformance/src/support.rs:595:14:
  check fails
  ```

Removed: each edit was restored; `conformance_tc_06` and `conformance_pd_05`
pass. The registry is part of the language-1.2 compiler-semantics ID, so the
record of this change is the regeneration it forces: locks, goldens, fixtures,
the calculus's packages, the production export, and the verification records of
the examples (`cargo xtask check-golden`, `check-fixtures`, `check-calculus`,
`check-production-export`, `verify-examples --write`); only the language-1.2
projects change, and `examples/uor-atlas` (language 1.0) is untouched.

### a tail-recursive template rendered as a call can fail its row (`RB-14`, SPEC.md §17.13, §17.16, #105 step 2)

A function every self call of which is a tail call, and some of which changes a
parameter the body reads, is rendered as a loop (`calculus/rust/tail.rs`); the
calculus decides it (`rust::loops`, which `realization::recurses` reads, so the
derivation of `recursion` in `TC-06` follows the rendering) and the lowering
fails when its rewriting disagrees. Nine primitives go back to
`recursion = false` (`list_fold`, `iterate`, `iterate_until`, `map_fold`,
`set_fold`, `map_lookup`, `map_contains`, `set_contains`, `graph_successors`);
the others keep it, since each has a call whose result is used. `RB-14` renders
every template in the fixture that instantiates it, compares the rendered call
graph to the registry row of every template-realized primitive, and states the
fourteen templates whose instance holds a loop. The case passes on the tree it
was written in (0.5 s). It makes each of these plants on every run's input
(`cargo test -p repo-conformance --test conformance -- conformance_rb_14`;
a defect planted in a clean tree, the failure observed, paths shortened, and
the defect removed):

- **the rewriting never makes a loop** (`looped` returns `None` at its first
  test):

  ```text
  panicked at crates/conformance/src/cases/rust_loops.rs:197:32:
  graph-reachable: function 2: the lowering and the calculus disagree on whether its self calls are a loop
  ```

- **neither the rewriting nor the calculus calls anything a loop** (both
  predicates return false, so a template is rendered recursively again and its
  row now says `false`):

  ```text
  assertion `left == right` failed: the templates rendered as loops
    left: []
   right: ["graph_reachable", "graph_successors", "graph_topological", "iterate", ...]
  ```

- **a row that says `true` for a loop.** The registry row of `primitive.list_fold`
  set back to `true`:

  ```text
  panicked at crates/conformance/src/cases/rust_loops.rs:314:5:
  recursion disagreements: [
      "primitive.list_fold: registry true, rendering false",
  ]
  ```

`conformance_pd_05` follows the rows: its second half removes `recursion` from
`stringGraph` (a graph search, whose template still recurses) and expects
`LLT4005` naming the effect, and checks that `iterations`, which iterates and
declares none, passes; it passes on the relocked scratch copy. The nineteen
declarations of the roots that no longer require `recursion` (`isClosedLeanName`,
`lex`, `auditTokens`, `mapFolding`, `folds`, `iterations`, `keyTypes`, the seven
reasoning roots that iterate, and the five `Text` members) lose it, read from the
eligibility reports of that copy, and the constants of `BS-01`, `BS-02` and
`SL-01` that state them follow.

A definition of the project follows the same rule (the owner's decision: the
effect says what the renderer does for every function, not only templates).
Eligibility (`production::eligibility::analyse_root`) lowers the closure of a
root whose walk found `recursion`, with the report the walk produced, and
requires the effect only if the lowered program recurses
(`realization::program_recurses`, which reads `rust::loops`); a root that does
not lower keeps the registry's answer. On the relocked scratch copy,
`examples/production-coverage`'s `wellFounded` and `examples/production`'s
`halvings` (tail-recursive definitions) require no `recursion` and lose it from
their declarations, while `sumAll`, `treeTotal`, `parity` (mutual) and the
others keep it; `conformance_pd_05` checks `halvings`, and removes the effect
from `sumAll` and expects `LLT4005`. This plant is made on the `rust::loops`
decision (`conformance_rb_14`): it reports a loop for every function that
mentions itself, so a function with a call that is not a tail call is a loop
for the calculus and not for the lowering:

```text
panicked at crates/conformance/src/cases/rust_loops.rs:197:32:
graph-reachable: function 8: the lowering and the calculus disagree on whether its self calls are a loop
```

Cost of the lowering at `check`: `lexlean check` of `bootstrap/` (three
modules, the largest committed roots; a debug build) took 2.1 s to 2.4 s without
the refinement and 2.4 s to 2.8 s with it (three runs each, scratch copy).

Removed: each edit was restored and `conformance_rb_14` passes. The converse
direction (a row `false` for a template that is rendered as a call) is
`conformance_tc_06`'s, which derives the row from the calculus.

What was checked and what was not. On a scratch copy of the tree, the
renderer's record regenerated by the repository's writers
(`check-calculus --write`) and every lock relocked, `conformance_tc_07`
(every fixture renders, builds under the pinned rustc with warnings denied,
and prints the denotation's value), `conformance_rb_06` (the lint gate over
every committed package) and `conformance_rb_07` pass. The lint gate caught a
defect of the first printing of a loop: a body with bindings as the scrutinee
of the `match` is `clippy::blocks_in_conditions`, so the printer binds the
body's value first (`let step = { ... }; match step { ... }`). Nothing in the
tree was regenerated by this change, and the certificate of a root with a loop
was the dependency it left: `cargo xtask check-production-export` on that
scratch copy stopped with `LLV7015: certificate B: no derivation relates
Production.Main.halvings to its rust-core rendering: function 1: binder
pattern Bind(Param(8)) binds no local`, because the `Corr` correspondence had
no rule that relates a calculus self tail call to `Expr.loop`. `SP-20` (#214,
in the same wave) adds that rule and its soundness case, and the `Production`,
`Coverage`, `Reasoning`, `bootstrap` and `stdlib` roots that use a loop are
certified with it.

## The authoring notation (Issue #93, stage 3 of #27)

`AU-01`..`AU-12` (SPEC.md §17.21, §24.8) state that the notation and the canonical
JSON of the closed schema are in bijection, that a module in the notation is
the module its data form is, that malformed notation fails closed, that
every language-1.2 source of the repository is written in it, that the same
holds for tooling modules, that no identity depends on the form, that no
writer owns a product source but five named records, and that programs read
and write a source through the authoring API. The
corpus of `AU-01` is every committed semantic module of language 1.2 (290 modules
with 4,949 declarations at this writing), and its samples are read from the schema, one for each
alternative of the declaration, term, type, and proof unions with and without
its optional members. Each plant below is one edit of the compiler, started from
the passing cases, and must turn the named case red for the reason it names.

- **a readable form that is not what the parser reads.** The printer of a
  `match` writes its branch marker without the space (`|C`), which the parser
  reads as a word. `-- conformance_au_01`:

  ```text
  panicked at crates/conformance/src/cases/authoring.rs:115:65:
  term match: a match has at least one `| …` branch (at byte 9)
  (match 0 |x => 0)
  ```

- **a readable form that drops what it does not spell.** The guard that a
  binary form has exactly its two members is removed, so a node with an extra
  member is written as `(1 + 2)`. `-- conformance_au_01`:

  ```text
  panicked at crates/conformance/src/cases/authoring.rs:116:5:
  assertion `left == right` failed: add{left: nat{value: "1"}, right: nat{value: "2"}, extra: true}: read back differently
  (1 + 2)
  ```

- **numerals in a string read as numerals.** The scanner no longer tracks the
  strings of a notation body, so `"007"` is a numeral with a redundant leading
  zero. `-- conformance_au_03`:

  ```text
  panicked at crates/conformance/src/support.rs:569:14:
  check succeeds: LexLeanError { class: Language, diagnostics: [Diagnostic { code: DiagnosticCode("LLL1003"), message: "numeral `007` has a redundant leading zero; the canonical decimal is `7`", …
  ```

- **a source map off by one.** The spans of the declarations of a notation
  module are shifted one byte. `-- conformance_au_02`:

  ```text
  panicked at crates/conformance/src/cases/authoring.rs:421:17:
  language-1.2: `efinition{
    name: doubledSuccessor, …` is not a declaration of the notation
  ```

- **the notation under language 1.1.** The language check of the link stage is
  disabled, so the notation is accepted where only the data form exists.
  `-- conformance_au_02`:

  ```text
  panicked at crates/conformance/src/support.rs:687:5:
  expected LLP2003, found ["LLT4001"] (LLT4001: phase link: theorem `t` statement has type Bool, expected Prop)
  ```

- **a formatter that does not format.** `fmt` returns the notation as it is, so
  a module in another layout is canonical. `-- conformance_au_02`:

  ```text
  panicked at crates/conformance/src/cases/authoring.rs:355:18:
  another layout is not canonical
  ```

Moving the committed sources to the notation (`AU-04`, 287 sources) adds the
plants of the gate that keeps them there; each is a change to a committed
source or to one line of the compiler or the gate, and each turns the named
case red for the reason it names.

- **a product source written as data.** `examples/language-1.2/src/Main.lex.tex`
  is written as `\semanticdata{…}`. `-- conformance_au_04`:

  ```text
  panicked at crates/conformance/src/cases/authoring.rs:647:5:
  1 source(s) are not written in the authoring notation:
  examples/language-1.2/src/Main.lex.tex: written as data, which the notation can say
  ```

- **a refused fixture that the notation can spell, written as data.**
  `tests/negative/application-arity/project/src/Main.lex.tex` is written as
  `\semanticdata{…}` with a module of the schema the notation names.
  `-- conformance_au_04`:

  ```text
  panicked at crates/conformance/src/cases/authoring.rs:647:5:
  1 source(s) are not written in the authoring notation:
  tests/negative/application-arity/project/src/Main.lex.tex: written as data, which the notation can say (a refused fixture)
  ```

- **a source that is not its own canonical source.** One empty line is
  removed from `compiler/src/Gnaf.lex.tex`. `-- conformance_au_04`:

  ```text
  panicked at crates/conformance/src/cases/authoring.rs:647:5:
  1 source(s) are not written in the authoring notation:
  compiler/src/Gnaf.lex.tex: not its own canonical source
  ```

- **a gate that reads no source.** The language of every project is read as
  1.1, so every source is exempt as one that predates the notation.
  `-- conformance_au_04`:

  ```text
  panicked at crates/conformance/src/cases/authoring.rs:656:5:
  0 sources in the notation, 0 canonical: the walk did not read the product
  ```

- **a formatter that leaves the data form alone.** `canonical_source` no longer
  writes a language-1.2 module out of its data form, so a module written as
  data is canonical and `fmt` never migrates it. `-- conformance_au_02`:

  ```text
  panicked at crates/conformance/src/cases/authoring.rs:276:18:
  a module written as data is not canonical
  ```

The migration itself is checked by what it must not change. The calculus
generator, which writes the compiler project's generated modules with
`lx::module_tex` in the notation, writes exactly the migrated bytes of those
modules (`check-calculus`: 994 generated files equal their generator). The
exported production packages are equal to the committed ones
(`check-production-export`). The regenerated goldens (112 files) are source
IDs, source maps, and coverage records only, and the regenerated fixture
expectations (392 files) differ from the committed ones only in the byte,
line, and column of a diagnostic, never in its code or message. The
verified records are not regenerated, because nothing they bind changes: the
semantic ID of `examples/language-1.2` before and after the migration is
`e8cdfbdf…`.

The one macOS failure of the first run of this stage (`end_children`, the
concurrent-start case, `EAGAIN` from `fork` and `ps`) was the case's own: the
four spawner threads held every child they started, so the children the host
ended stayed in the process table as zombies until the runner's limit was
reached. They now reap the children that have exited. The planted removal of
the registration lock, which the case exists to catch, still fails it in cycle
0: `a child started concurrently with the call survived it`.

Tooling modules in the notation (`AU-05`) and the identity that does not depend
on the form (`AU-06`) add three more, each one edit of the compiler:

- **a declaration span one byte late.** The span of each declaration of a
  tooling module is shifted one byte, so the source map names text that is
  not the declaration. `-- conformance_au_05`:

  ```text
  panicked at crates/conformance/src/cases/authoring.rs:793:13:
  assertion `left == right` failed: …/tests/tooling/traversal/src/Main.lex.tex: a span is not its own declaration
    left: Obj({"constructors": …, "kind": Str("ype"), "name": Str("Lean.Compiler.LCNF.Purity"), …
  ```

- **a formatter that leaves a tooling module alone.** `canonical_source` no
  longer rewrites a tooling module, so one written as `\toolingdata` is
  canonical. `-- conformance_au_05`:

  ```text
  panicked at crates/conformance/src/cases/authoring.rs:817:9:
  assertion `left == right` failed: commands: fmt did not write the data form as the committed notation
  ```

- **an identity of the bytes of a file.** The compiler-semantics ID hashes the
  file as it is, so the form the adapter's source is written in is part of
  what the compiler means. `-- conformance_au_06`:

  ```text
  panicked at crates/conformance/src/cases/authoring.rs:960:17:
  assertion `left == right` failed: language/lcnf-1.2/tooling/src/Extract.lex.tex: a notation module contributes its data form
  ```

The four committed tooling sources are migrated with the same printer `fmt`
uses, and that is checked by what must not change: `check-calculus` finds the
994 generated files, among them the adapter's tooling module and the three
fixtures, equal to their generator, which now writes the notation; the locks
of every example are current under the unchanged language-1.2 ID, which
`AU-06` recomputes from the tree with the adapter's source in the notation and
as data; the regenerated goldens of the three fixtures are source IDs, source
maps, and coverage records only.

The gate that no writer owns a product source (`AU-07`) and the authoring API
(`AU-08`) add three:

- **a writer that emits a product source.** `calculus::files()`, the generator
  `cargo xtask check-calculus` compares and rewrites, is made to emit
  `compiler/src/Main.lex.tex`. `-- conformance_au_07`:

  ```text
  panicked at crates/conformance/src/cases/authoring.rs:1044:5:
  assertion `left == right` failed: the writers of the repository emit exactly the named records and no other product source
    left: {"compiler/src/GnafCertificates.lex.tex", …, "compiler/src/Main.lex.tex", …}
   right: {"compiler/src/GnafCertificates.lex.tex", …, "compiler/src/TargetFixtures.lex.tex", "language/lcnf-1.2/tooling/src/Extract.lex.tex"}
  ```

- **an assembly that drops what the source says.** `sdk::assemble` writes no
  `\importmodule` line. `-- conformance_au_08`:

  ```text
  panicked at crates/conformance/src/cases/authoring.rs:1249:17:
  assertion `left == right` failed: …/examples/reasoning/src/Main.lex.tex: assembled from its header and data
  ```

- **a rewrite that changes the form of a source.** `sdk::write_module` writes a
  source in the notation back as data. `-- conformance_au_08`:

  ```text
  panicked at crates/conformance/src/cases/authoring.rs:1181:9:
  assertion `left == right` failed: …/examples/reasoning/src/Main.lex.tex
  ```

The generated records of `AU-07` are named, not exempted by a pattern: the four
fixture modules of `compiler/` are written from the committed fixtures,
requests, and certificates (which are data, not sources), and the extraction
adapter's module from the registry the pinned Lean answers (`conformance_ne_04`
equates the registry with a fresh derivation). The writers' own files
(`check-calculus`: 977 of them, where it was 994 before the builders of the
calculus, the machine, and the two models were removed) are compared by
`cargo xtask check-calculus` as before.

Core modules in the notation (`AU-09`, `AU-10`, `AU-11`; SPEC.md §17.21.8) add
nine plants, each one edit of the compiler, started from the passing cases:

- **a rule for what is written inside its user that is not the specified
  one.** The printer writes a node inside its user only when the nodes under it
  are fewer than two deep, not 48. `-- conformance_au_09`:

  ```text
  panicked at crates/conformance/src/cases/authoring_core.rs:322:5:
  assertion `left == right` failed: the module that carries every kind: the statements are not the nodes the rule gives one
    left: {0, 1, 2, …, 10, 12, 13, …, 31, 33, 34, 35}
   right: {0, 1, 2, …, 10, 13, 14, …, 31, 33, 34, 35}
  ```

- **a canonical writer that is not the canonical JSON.** The data form of an
  application writes its keys as `k`, `f`, `x`. `-- conformance_au_09`:

  ```text
  panicked at crates/conformance/src/cases/authoring_core.rs:420:46:
  compiler/proofs/src/Check.lex.tex: the compiler refuses it: core-module JSON is not in canonical byte form: object key `f` is not strictly ordered
  ```

- **a level written as another.** A universe parameter is written `u`
  whatever its name. `-- conformance_au_09`:

  ```text
  panicked at crates/conformance/src/cases/authoring_core.rs:313:5:
  the module that carries every kind: the notation reads to another module
  ```

- **a source map that names the data.** The core backend's coverage names
  `lexlean.core::coredata` for a module written in the notation.
  `-- conformance_au_10`:

  ```text
  panicked at crates/conformance/src/cases/authoring_core.rs:650:5:
  the small module: the coverage of the notation does not name the notation
  ```

- **a formatter that leaves a core module alone.** `canonical_source` no longer
  rewrites a core module, so one written as `\coredata` is canonical.
  `-- conformance_au_10`:

  ```text
  panicked at crates/conformance/src/cases/authoring_core.rs:575:14:
  a module written as data is not canonical
  ```

- **a statement that need not be its node.** The reader no longer checks that
  a label is the number of the node its statement defines, so a module whose
  labels are wrong links. `-- conformance_au_11`:

  ```text
  panicked at crates/conformance/src/support.rs:595:14:
  check fails
  ```

- **a nesting without a bound.** The reader's bound on nesting is raised past
  what the case writes, so 300 parentheses read. `-- conformance_au_11`: the
  same `check fails` from `support.rs:595:14`.

- **an identity that does not read a core module's meaning.** The data form of
  the embedded tree leaves a core module as its bytes. `-- conformance_au_06`:

  ```text
  panicked at crates/conformance/src/cases/authoring.rs:934:17:
  assertion `left == right` failed: language/preservation-1.2/library/src/Compose.lex.tex: a notation module contributes its data form
  ```

- **a library written as data again.** One module of the preservation library
  is restored to its `\coredata` form. `-- conformance_au_04`:

  ```text
  panicked at crates/conformance/src/cases/authoring.rs:661:5:
  1 source(s) are not written in the authoring notation:
  language/preservation-1.2/library/src/Values.lex.tex: written as data, which the notation can say
  ```

- **an API that takes a core module for another kind.** `sdk::module_kind`
  names a core module a tooling module. `-- conformance_au_08`:

  ```text
  panicked at crates/conformance/src/cases/authoring.rs:1242:71:
  …/compiler/proofs/src/Frontier.lex.tex: the module is not a `lexlean/tooling-module/1` module, the schema its notation names
  ```

Names and statement names (`AU-12`; stage 3d of #93) add four more:

- **a variable named when a nearer binder has the name.** The printer writes a
  variable by its binder's name whether or not a nearer binder has it, so the
  text reads to another variable. `-- conformance_au_09`:

  ```text
  panicked at crates/conformance/src/cases/authoring_core.rs:340:5:
  the module that carries every kind: the notation reads to another module
  ```

- **a constant that a binder shadows.** The printer writes a constant bare
  even when a binder in scope is named like it, so the text reads to the
  variable. The same failure from `conformance_au_09`.

- **an arrow whose body is not under a binder.** The reader does not put the
  anonymous binder of `A -> B` in scope, so every name in the body is one index
  off. `-- conformance_au_12`:

  ```text
  panicked at crates/conformance/src/cases/authoring_core.rs:963:9:
  assertion `left == right` failed: compiler/proofs/src/Soundness.lex.tex: fmt does not write the numbers
  ```

- **a repeated statement name.** The reader takes the later of two statements
  of one name. `-- conformance_au_12`:

  ```text
  panicked at crates/conformance/src/support.rs:595:14:
  check fails
  ```

The first theorem written that way is in the certification library:
`LexLeanCertification.authored_eq_symm` (symmetry of `Eq` by `Eq.rec`, with every
argument and level explicit), a declaration and two statements appended to
`Soundness`. `AU-12` writes it from its authored text on the library without
it and finds the committed module and the committed bytes; the family `GC`
compiles the library with the pinned Lean (the kernel checks the theorem with
the rest), replays it with `leanchecker`, and audits its axioms (it pins none,
and Lean prints none). The committed libraries are otherwise unchanged by names:
a shared body is a node of its own, so a variable in it is free in its
statement and stays `#i` (twenty-five of the twenty-six modules are the same
bytes; one differs by two).

A first plant found a defect in the rule and not in the code: the printer and
the case both excluded the nodes a declaration states from being written
inside their user, which counting a declaration's use of its roots already
does, so that condition could not be exercised and was removed from both.

The 26 committed language-1.2 core modules are migrated with the same printer
`fmt` uses, after the reading of each was checked against the data it came from:
the canonical JSON of the notation of every one of them is the committed JSON
byte for byte, the compiler's own decoder (`CoreModule::parse`, with its check
of the canonical byte form and its validation) accepts it, and the 66 Atlas
modules (language 1.0, written as data) are written back byte for byte by the
same canonical writer and read back equal as modules of the schema the notation
names. What must not change did not: the compiler-semantics ID of language 1.2
is still `464235ad…`, the locks of every example are current, `check-golden`
finds every published artifact equal to its oracle, and `check-production-export`
finds every exported package equal to its committed package (16 minutes). The
migrated sources are 0.61 to 0.85 of the size of the data they were (24.4 MB
against 28.8 MB for the largest module), and reading the identity of language 1.2
takes 0.8 s on four cores in a release build.

A second defect was found by CI and not by any case: `library_identity` (the
`library.registry_sha256` of every `preservation.json`) hashed the embedded
native library as text, so writing the 26 modules in the notation changed the
record of `examples/models` (`verify-examples`, R10) although no module's
meaning changed, which is what `AU-06` says cannot happen. The identity is now
over each source's data form, `AU-06` states it and compares it with the
committed record, and the record of `models` verified again is byte for byte the
committed one.

- **a library identity over the text of a source.** `library_identity_of` hashes
  `source.as_bytes()` and not its `identity_bytes`. `-- conformance_au_06`:

  ```text
  panicked at crates/conformance/src/cases/authoring.rs:1018:5:
  assertion `left == right` failed: the library identity of a record depends on the form its sources are written in
  ```

Removed: each edit was restored; `conformance_au_01`..`conformance_au_12` pass. The generic notation has its own property test over
arbitrary canonical JSON values (`surface::generic`), and every alternative of
the schema its own sample (`surface::semantic`), as unit tests of the
`lexlean` crate.

## UOR Atlas as the parts of one run (Issue #131)

Issue #131 asked why every pull request's gate takes three hours and what
would shorten it without a weaker proof obligation. On `main` at `514e95ad`
(CI run 38716449420) the `examples` leg took 178 minutes, 156 of them the
verification of UOR Atlas (elaboration 5,047 s, replay 4,286 s); the `rest`
leg took 76 minutes. Atlas's 66 modules are not independent: 63 of its sources
declare `\importmodule`, the generated Lean imports them as `public import`,
and the chain `Roots → Blocks → Places → Group → Census → CensusCert4`, then
`CensusComplete → Closure → ClosurePairs → ClosurePairCert* → … → Atlas`, is 20
deep. From the committed hints and the measured replays, the elaboration of
that chain up to `CensusCert4` (2,687 s) and the replay of `CensusCert4`
(1,229 s) bound any schedule of today's modules at about 65 minutes, however
many runners it has; a single 16 GB runner at width 2 cannot overlap the four
modules that peak near 15 GiB with anything, which is where the other 90
minutes went. The maintainer's reading on the issue, that a content-addressed
certificate of main's proof would skip Atlas on five of the 34 merged pull
requests once its key covered the compiler sources, stands; `§9.4`'s sentence
on caches is unchanged.

What changed is where the processes run, not which run. Atlas's verification
is partitioned into the shards of `.github/verify-shards.json`, each a job of
its own clean checkout on the standard runner: `prefix` elaborates the import
closure of `CensusCert4` (15 modules, 1,399 s of elaboration); `cert4` that
module alone; `census-a` to `census-d` the other `CensusCert*` modules, the
replays of the prefix, and the modules that import only the prefix; `closure`
the three modules every later one imports; `orbit` and `pairs-a` to `pairs-d`
the rest; `cert4-replay` the one replay that takes 20 minutes; and the `join`
the last four modules, then every shard's records, the attestation, and the
comparison with `expected/verify`. The critical path is `prefix` (23 minutes
of elaboration at width 2), `cert4` (22), `closure` (3), the `pairs` shards (11
each), the join (under 2), plus each job's set-up: the Lean toolchain and,
once, the Rust build of the gate's executable, which the `prefix` job uploads
and the other jobs take from the run. That is an estimate of 80 to 90 minutes
for the Atlas side against 178; the first CI run of the change is the
measurement, and the `rest` leg (76 minutes, 64 of them `bdd`) is then the
longer one.

The verifier runs as one part through `lexlean::verify_part`
(`verify::partition`, SPEC.md §9.4, §22.11). A shard runs the prelude whole
(certificate generation, the preflights, staging, the generated-source audit,
the probe), seeds the placed set with the modules whose `.olean` arrived from
the shards it needs, elaborates the modules of its plan in the same
topological frontiers, replays and audits the modules its plan names, and
moves its `.olean` tree (intermediates included, a later shard's elaboration
reads them) and its process records to its output directory; it publishes
nothing. The join does the same for its own plan, reads every other part's
records (`ChildRecord::from_json`, which checks each stream against its
digest), judges an imported elaboration as the elaboration stage judges its
own (exit 0, silent), an imported replay as the replay stage does (exit 0,
else `LLV7003`), and an imported audit member as the audit stage does (exit 0,
silent, its printed set parsed against the member's declarations, else
`LLV7004`), and assembles the attestation's process list in the canonical
order of §22.11 (`partition::elaboration_order`: the frontiers in module
order, then replays in sorted name, then the family) from whichever part
produced each record. A module without a record from any part, a record of a
process no module owns, two parts that wrote the same `.olean` or record, and
a shard whose plan elaborates a module before its imports (`module order did
not converge: … imports …`) are refused. Before it judges any imported
record, the join requires its `executable_sha256` to be the join's own digest
of the tool that produced it (`partition::check_toolchain`): `lean` for an
elaboration or an audit member, `leanchecker` for a replay, else `LLI9001`
naming the record and both digests. In a whole verification that holds by
construction; across the jobs of a run it held because every job installs the
same pinned toolchain, which was a practice and not a check (the maintainer's
review of #152). A part is defined for a project without production roots and
without PDF rendering; Atlas is such a project.

`VR-23` verifies `peano-arithmetic` whole and as two shards and a join over
its committed import graph (the first frontier; the second, replaying both;
the rest), and requires the join's verified directory to equal the whole's
byte for byte with the same attestation ID, then refuses the three plans
above and a join handed the second shard's replay record of a module with its
`executable_sha256` rewritten to zeros (`` the imported replay record of
`PeanoArithmetic.…` ``, with both digests). `RP-14` reads `ci.yml` against the plan: every shard run by exactly one
job, each job waiting (through the job graph) for the jobs of the shards its
shard needs and never for a row of its own matrix, the examples profile on
every Atlas job, the one `cargo build` in the `prefix` job and every other job
taking the executable and the shards' output from the run (`pattern:
atlas-*`, never a `run-id`), the shard output uploaded under the shard's name,
the join asserting Atlas's two lines, the draft condition on every Atlas job
and no other, and the `ready_for_review` type that reruns them. The plan
itself is checked by `cargo xtask ci-shards` and inside `RP-14` by
`repo_conformance::shards::check`.

### the partition can fail

Each record is a defect planted in `.github/verify-shards.json` or `ci.yml`,
then `cargo test -p repo-conformance --test conformance -- --exact
conformance_rp_14` (the plan's defects also fail `cargo xtask ci-shards` with
the same line), and the failure, after which the file was restored.

- **a module replayed by two shards** (`CensusCert4` added to `census-a`'s
  replay): `` `LexLeanExample.CensusCert4` is replayed by `census-a` and by
  `cert4-replay` ``.
- **a shard elaborating a module before its imports** (`cert4` with no needs):
  `` shard `cert4` elaborates `LexLeanExample.CensusCert4`, whose import
  `LexLeanExample.CensusClosure` is elaborated by no shard it needs ``.
- **a module elaborated by no shard** (`Atlas` dropped from the join):
  `` `LexLeanExample.Atlas` is elaborated by no shard ``.
- **a job that no longer waits for the shards it needs** (`atlas-closure` with
  `needs: [atlas-prefix]`): `` §9.4: `closure` runs in `atlas-closure`, which
  does not wait for `atlas-l1`, the job of `cert4` it needs ``.
- **an Atlas job without the draft condition** (`atlas-l3`): `` §9.4:
  `atlas-l3` waits only for a pull request to leave draft ``.
- **a second `cargo build`** (in `atlas-closure`): `ci.yml builds the gate's
  executable once` (`left: 2, right: 1`).
- **a shard run by two jobs** (`pairs-a` added to `atlas-l1`'s matrix):
  `` §9.4: the shard `pairs-a` is run by two jobs ``.
- **the examples leg verifying every example** (`examples_part` emptied):
  `§9.4: the examples leg's profile and part` (`left: [.., ""]`, `right: [..,
  "without:uor-atlas"]`).
- **a cache action in a shard job**: `` ci.yml has no `actions/cache` ``.
- **the join reading another run's artifacts** (`run-id` on its download):
  `` §9.4: `atlas-join` reads the artifacts of this run only ``.
- **the join no longer checking the toolchain of what it imports** (the
  `partition::check_toolchain` call removed from `verify::run_part`), then
  `cargo test -p repo-conformance --all-features --test conformance -- --exact
  conformance_vr_23`: the join accepted the replay record of another
  `leanchecker` and published, and the case failed at its refusal, `a record
  of another leanchecker: ()`. The unit test
  `verify::partition::tests::a_part_output_round_trips_its_oleans_and_records`
  holds the same refusal on the function itself.

Two defects were found by the cases and not by review: a shard exported the
`.olean` files it had imported along with its own, so the join refused the
duplicate (`` `oleans/PeanoArithmetic/Addition.ir` was written by two parts of
the run ``; `export` now moves only the modules of the shard's plan), and the
join assembled the imported elaboration and replay records into the
attestation without publishing them, so the directory it published had no
`process/lean/PeanoArithmetic.Addition.json` and `verify-examples` refused it
against `expected/verify` (R10). Both are what `VR-23` and the end-to-end run
of `verify-examples` as two shards and a join of `peano-arithmetic` now check.
### runtime modules can fail

Five plants, each on the committed tree and each restored after its run.
Commands: `cargo test -p repo-conformance --all-features --test conformance
-- <case> --exact` for the first four; for the fifth, `lexlean lock && lexlean
verify` in a copy of `examples/collections`, where pinned Lean is the oracle.

1. A hand-written runtime text put back in the semantic backend (a
   `COLLECTIONS_RUNTIME` string constant appended to `backend/semantic.rs`).
2. The frozen language-1.1 portable runtime changed in one name
   (`index` to `indez` in `language/runtime-1.2/frozen/portable-runtime-1.1.txt`).
3. A generated Lean copy of a runtime module committed beside its source
   (`language/runtime-1.2/library/LexLeanRuntime/Portable.lean`).
4. A preservation-library reference to a per-module runtime copy put back
   (`@244 := LexLeanRuntime.index` to
   `@244 := LexLeanTarget.TargetSemantics.LexLeanRuntime.index` in the notation of
   `Core.lex.tex`).
5. A runtime row pinning an axiom its declaration does not use
   (`LexLeanCollections.insertEntry` pinned to `exact{axioms: [propext]}` in
   `Collections.lex.tex`).

```text
thread 'conformance_rt_04' panicked at crates/conformance/src/cases/runtime.rs:494:9:
assertion `left == right` failed: the semantic backend carries no runtime text
  left: [2966]
 right: []
thread 'conformance_rt_02' panicked at crates/conformance/src/cases/runtime.rs:260:5:
the frozen portable runtime is the released language-1.1 output
thread 'conformance_rt_01' panicked at crates/conformance/src/cases/runtime.rs:88:9:
.../language/runtime-1.2/library/LexLeanRuntime/Portable.lean: a Lean file lies in the runtime project
thread 'conformance_rt_01' panicked at crates/conformance/src/cases/runtime.rs:199:21:
.../language/preservation-1.2/library/src/Core.lex.tex: a runtime copy `…TargetSemantics.LexLeanRuntime.index
@247 := @2` is named inside a module
error[LLV7005]: `LexLeanCollections.insertEntry` of the runtime module `LexLeanRuntime.Collections` violates its exact axiom policy: observed []
  --> lexlean.toml:1:1
```

Removed: each file was restored, and `RT-01`..`RT-04` pass on the restored
tree.

### the runtime gates, read again (Issue #154)

`RT-04` had cut each compiler source at its first `#[cfg(test)]`, asked a
literal for a `namespace` line and a declaration, and read one literal at a
time; #154 reproduced a text that passed. Each shape below is now planted on
the committed tree and restored after its run
(`cargo test -p repo-conformance --all-features --test conformance --
conformance_rt_04 --exact`); the same shapes, and the ones that must not be
found, are asserted by the case itself.

1. Production code after the first test module of the verifier: a
   `const PLANTED: &str = "public def LexLeanRuntime.listFold : Nat := 0";`
   after `HEAVY_FRACTION` in `verify/mod.rs`, which follows the module that
   ends at line 2694.
2. A declaration under a runtime name and no `namespace` line, as a string
   constant appended to `backend/semantic.rs`.
3. The same text written with `concat!` (`namespace LexLeanCollections`,
   `public def listFold : Nat := 0`, `end LexLeanCollections` as three
   literals).
4. The same under a `format!` template
   (`public def LexLeanReasoning.{name} : Nat := 0`).
5. Production code after a non-item `#[cfg(test)]` (#169): a struct with a
   `#[cfg(test)]` field followed by
   `const PLANTED_AFTER_CFG_TEST_FIELD: &str = "public def LexLeanRuntime.x : Nat := 0";`
   appended to `backend/semantic.rs` (`without_test_items` blanks only items
   under `#[cfg(test)]` and file-leading `#![cfg(test)]`, and the case itself
   also plants the five shapes of #169 — a struct field, an enum variant, a
   function parameter, a match arm, and an inline module with `#![cfg(test)]` —
   as well as `+` concatenation).

```text
thread 'conformance_rt_04' panicked at crates/conformance/src/cases/runtime.rs:765:5:
assertion `left == right` failed: the only Lean runtime text in the compiler is the decoder of core module 2
  left: [("backend/core_v2.rs", 34), ("verify/mod.rs", 2731)]
 right: [("backend/core_v2.rs", 34)]
  left: [("backend/core_v2.rs", 34), ("backend/semantic.rs", 2965)]     (2 and 3)
  left: [("backend/core_v2.rs", 34), ("backend/semantic.rs", 2966)]     (4)
  left: [("backend/core_v2.rs", 83), ("backend/semantic.rs", 3191)]     (5)
```

What the gate cannot see is said here and not left to a reader: a text built
at run time from parts declared apart (a name in one constant, a template in
another) is not a literal or a run of literals, and `RT-04` does not read it.
What bounds that is the other half of its statement, which is checked over
the output: a generated language-1.2 module that declares under a runtime
namespace, or names a runtime constant without importing the module that
declares it, is refused (planted in the case: the committed module of the
collections example with its runtime imports dropped, and with a runtime
namespace appended).

`RT-03`'s axiom check compares the observed and the pinned sets in both
directions (`LLV7005`); the fifth plant of the section above pinned an axiom a
declaration does not use, and the case now also plants the other:
`LexLeanCollections.compareCodes._sparseCasesOn_1` pinned to
`exact{axioms: []}` in `Collections.lex.tex`, which uses `propext`:

```text
error[LLV7005]: `LexLeanCollections.compareCodes._sparseCasesOn_1` violates its exact axiom policy: observed [propext]
  --> lexlean.toml:1:1
```

A runtime declaration whose body changes and still type-checks
(`@3159 := 64` to `65` in `Portable.lex.tex`, a width constant of
`checkedMultiplyInt64`) is not caught by `RT-01`..`RT-04`: all four pass with
it planted, because it is a well-typed change that pins the same axioms. It is
caught by the identity discipline, which is the gate for any normative byte:
the language-1.2 compiler-semantics ID changes, so `lexlean lock --check` in
each language-1.2 example refuses its lock,

```text
error[LLC0102]: lexlean.lock is stale or noncanonical; run `lexlean lock`
  --> lexlean.lock:1:1
```

and a verification after relocking differs from the committed record of the
example (`examples/collections/expected/verify/audit/LexLeanAudit.Aea7d3fe….*`
against `LexLeanAudit.A65452ac….*`; the audit prefix is a function of the
identity); `cargo xtask verify-examples` compares those records, so the change
stays red until every record is regenerated, which is a diff a review reads. No
runtime-specific oracle of a declaration's body exists beyond those pins, and
none is claimed.

`RT-02`'s statement that the frozen files belong to no older identity was a
grep of `SPEC.md` for the path; it now recomputes the three IDs from disk
(`RP-10`'s recomputation, shared) and finds, with either frozen file changed
by one byte, the language-1.0 and 1.1 IDs unchanged and the 1.2 ID changed.
The repository walkers skip any directory that holds a `CACHEDIR.TAG` (including
`xtask/src/audit.rs`, `crates/conformance/src/lib.rs`, and
`crates/conformance/src/cases/oracle.rs`): with a Cargo target directory named
`target-check` in the checkout, `RT-01` and `SP-12` failed on its Lean files
before and pass now. Conversely (`RP-03`, #170), the tracked tree (`git ls-files`)
holds no `CACHEDIR.TAG` file and no path component the walkers skip (`target`,
`.git`, `.lexlean`, `.lake`, `node_modules`, or `expected` outside the
sanctioned `expected/` oracle directories). Planted in the git index and
restored after each run (`cargo test -p repo-conformance --test conformance
conformance_rp_03 -- --exact`):

1. A tracked `examples/collections/CACHEDIR.TAG`:
   `left: ["examples/collections/CACHEDIR.TAG: tracked `CACHEDIR.TAG` would cause repository walkers to skip `examples/collections`"]`
2. A tracked `.lexlean` directory outside the ignored roots
   (`crates/lexlean/.lexlean/planted.lean`):
   `left: ["crates/lexlean/.lexlean/planted.lean: tracked path has walker-skipped component `.lexlean`"]`

The fifth runtime module, `LexLeanRuntime.Laws` (#117), declares under the
namespace `LexLeanLaws`, which the runtime cases' list of namespaces did not
hold: a copy of a laws declaration in a generated module, or a literal of the
compiler's source under that namespace, passed `RT-01` and `RT-04`. The list is
now asserted equal to the first segments of the declarations of every shipped
runtime module (`rt_04`), so a sixth module cannot be missed the same way. Both
halves are planted by removing `"LexLeanLaws"` from the list and running
`cargo test -p repo-conformance --all-features --test conformance --
conformance_rt_04 --exact`, each caught:

```text
assertion `left == right` failed: a planted runtime text is found (a declaration under the laws namespace): const PLANTED: &str = "public def LexLeanLaws.sumNat (xs : List Nat) : Nat := xs.length\n";
  left: 0
 right: 1
```

and, with that plant also removed from the case,

```text
assertion `left == right` failed: the runtime namespaces the gates know are the namespaces the shipped runtime declares
  left: {"LexLeanCollections", "LexLeanLaws", "LexLeanModels", "LexLeanReasoning", "LexLeanRuntime"}
 right: {"LexLeanCollections", "LexLeanModels", "LexLeanReasoning", "LexLeanRuntime"}
```

The identity wave (#219) regenerated the Atlas records from these shards with
no run of the whole Atlas on one host: twelve parts (`prefix`, `cert4`,
`census-a` to `census-d`, `cert4-replay`, `closure`, `pairs-a` to `pairs-d`)
were produced in the maintainer's 15 GB container at `LEXLEAN_VERIFY_WIDTH=2`
and `LEXLEAN_VERIFY_MEMORY_KIB=12582912`, and `orbit`, whose `ClosureOrbit`
elaboration peaks above that container's cgroup, was the `atlas-part-orbit`
artifact of the wave's first gate run (39243140976 on 621573a43, whose join
was red for want of exactly these records); `verify-examples` in
`join:uor-atlas` mode wrote the records from the thirteen parts, and the gate
run of the head that carries them is their comparison.

## The planted verification of the compiler project (Issue #145)

`conformance_rs_12` asks which diagnostic pinned Lean's refusal of the planted
reasoning transcriptions carries: a copy of the `compiler` project whose
`ReasoningFixtures` states them must end as `LLV7002`. How long Lean takes is
not part of that claim, but the copy inherited the 300 s `child_timeout_ms` of
the committed project, and a Lean child that outran it ended the case as
`LLS8002`, a different claim.

It did so on four runs: the push run of #99's merge (`c257a657`) and the PR
runs of #135, #137 and #143. The `ci-evidence` telemetry of the three PR runs
and of the passing push run of #102's merge (`70de63d3`) says what happened:

| run | `RS-12` | the five verifications of `compiler` (s) | `TargetSemantics` elaboration in each (s) | `EX-07` |
| --- | ---: | --- | --- | ---: |
| #102 merge, passed | 95 s | 154, 76, 36, 68, 64 | 10, 10, 10, 10, 11 | 1086 s |
| #135, failed | 596 s | 231, 112, 55, **543**, 98 | 17, 18, 17, **160**, 17 | 1690 s |
| #137, failed | 590 s | 237, 115, 55, **537**, 97 | 18, 18, 17, **150**, 17 | 1630 s |
| #143, failed | 595 s | 234, 115, 54, **542**, 94 | 17, 18, 17, **159**, 17 | 1624 s |

- **The module is far from the limit.** `ReasoningFixtures` elaborates in 25 s
  to 31 s in the passing run, in 36 s to 38 s in the unaffected verifications of
  the failing runs, and in 73 s to 105 s on an idle four-core development host
  (`conformance_rs_12` alone, passing: 737 s in all before the change, 691 s
  after).
- **Everything was slow, not the module.** In the fourth verification of each
  failing run the Lean children of every project running at the time took
  about 5 to 20 times their usual time: `TargetSemantics` 150 s to 160 s (17 s
  in the other verifications), the same module in `production`'s certificates
  172 s (13.5 s on the examples leg), `RustCorr` 47 s (4.3 s), the trivial
  `LexLeanExample.Main` of `nat-add-zero` 101 s (1 s to 5 s), a `leanchecker`
  replay 4 s (0.8 s). Memory was not short: 6.6 GiB or more available and no
  swap throughout. A slowdown of 5 to 20 times on 26 s to 38 s is 130 s to
  760 s, which is over 300 s.
- **It was not a single event.** The three runs have the same shape, in the same
  verification and to within 7 % of each other, and all three were slow runs
  throughout (`EX-07` 1.5 times the passing run's, `TargetSemantics` 17 s
  against 10 s even where nothing stalled). The cause of the slowdown, whether
  contention among the concurrent cases of the `bdd` recipe on the slower
  runners or something of the host's, is not established here.

The scratch copy now allows 900 s: three times the default, which keeps a
slowdown of more than 12 times (against the 73 s of the idle host) or of more
than 24 times (against the 37 s of CI) harmless, while a child that does hang
is still ended, in 15 minutes. The committed `compiler` project, its limits, `verified_compiler()`,
and every other case are unchanged. `verified_compiler()` has the same
exposure in principle; it ran in the quiet first minutes of all three failing
runs (231 s to 237 s) and is left alone until it fails.

### the planted verification can fail

- **a limit the case does not obey.** The scratch copy's `child_timeout_ms` set
  to 5000 in the same edit. `-- conformance_rs_12`:

  ```text
  panicked at crates/conformance/src/support.rs:703:5:
  expected LLV7002, found ["LLS8002"] (LLS8002: child_timeout_ms exceeded by `lean` in phase LexLeanTarget.ReasoningOracle: configured 5000, observed 5007 ms)
  ```

  The edit reaches the verification it is made for: the limit in force is the
  one the case sets, and a child that exceeds it is still `LLS8002`.

Removed: the edit was restored; `conformance_rs_12` passes in 691 s on the
idle host (737 s before the change).

### The start-order case of the verifier pool (Issue #148)

`verify::pool::tests::hints_order_the_start_and_never_the_consumption` failed
once in CI (Intel macOS host leg of the PR run of #147). It ran four items on
four workers and recorded the start order by pushing from inside each job,
after its claim, so what it recorded was the order in which the scheduler ran
the workers: `[3, 0, 1]` against `[3, 1, 0]`. Run 600 times beside eight busy
loops on an idle four-core host, the unchanged case failed 82 times, with the
same assertion and other permutations (`[3, 1, 2]`). It now uses two workers,
item 3 holding one for the whole run, so the first two claims are 3 and 1 (as
a set) and the next two, one thread's, are 0 then 2.

- **a claim order the case does not observe.** The claim of `run_ordered_with`
  taken from the items in their own order instead of the hint order
  (`items[sequence]` for `effective[sequence]`). `-- verify::pool`:

  ```text
  panicked at crates/lexlean/src/verify/pool.rs:672:9:
  assertion `left == right` failed: the hinted heaviest were started first
    left: [0, 1]
   right: [1, 3]
  ```

Removed: the edit was restored; the ten `verify::pool` cases pass, and the
changed case passes 400 of 400 runs beside the same eight busy loops.
## Exact toolchain version fields (Issue #134)

The existing `VR-02` owner (SPEC.md §8.2, §22.2) also refuses patch-prefix
collisions and prerelease suffixes in both tool banners, and a banner whose
unrelated version equals the pin. These use the owner's existing verified
fixture; every wrong banner fails in preflight before module compilation.

Planted: retain the old substring comparison in the behavior-preserving
`reports_pinned_version` extraction. Command:
`cargo test -p lexlean --lib verify::toolchain::tests::version_banners_require_the_exact_lean_version -- --exact --nocapture`.
Observed:

```text
thread 'verify::toolchain::tests::version_banners_require_the_exact_lean_version' panicked:
lean: Lean (version 4.32.10, release)
test result: FAILED. 0 passed; 1 failed
```

The fixed comparison reads the complete Lean version field of each banner.
The unit case covers real pinned Lean/Lake formats, other patches,
prereleases, unrelated occurrences of the pin, and malformed banners.

Validated after the fix: the unit case and `conformance_vr_02` pass (the
latter in 4.76 s), as do formatting, workspace/all-target Clippy with warnings
denied, model/spec-link audits, and all five BDD meta-gate tests.

### a standard-library member can fail its oracle (SPEC.md §17.22)

`SL-01` holds every member of `LexLeanStd.Text` to the function of Rust's
standard library it corresponds to: the member's lowered program under the
calculus evaluator and its rendered `rust-std` package under the pinned rustc
are run on a seeded corpus (edge cases first, then random inputs over the
bytes the members distinguish), and one disagreement fails the case. The first
run of the case found a defect in the source and not in the differential: three
declarations admitted `overflow` their closures never realize, and the case
refused them before any run (`startsWith: the declared effects`, declared
`["allocation", "overflow"]`, realized `["allocation"]`); the declarations now
admit exactly what they realize.

- **a character class off by one.** `isSpace` takes the vertical tab (`11`)
  for white space in place of the form feed (`12`), so `trim` keeps a form
  feed and strips a vertical tab, which `<[u8]>::trim_ascii` never does.
  `-- conformance_sl_01`:

  ```text
  panicked at crates/conformance/src/cases/stdlib.rs:483:17:
  trim: the evaluator and Rust's standard library disagree on 431 of 600 runs: ["run 4 on [Bytes { hex: \"0b2061200b\" }]: {\"hex\":\"61\",\"kind\":\"bytes\"} != {\"hex\":\"0b2061200b\",\"kind\":\"bytes\"}", …]
  ```

  The same plant is made by the case itself on a copy of the project on every
  run, so the gate stays armed: a corpus that no longer reaches the vertical
  tab or the form feed fails the case.

Removed: the source was restored; `conformance_sl_01` passes, in 156 s with
the production export of the seven members under the pinned Lean.

`SL-02` holds `LexLeanStd.Sha256` to FIPS 180-4 (#126, stage 1): both roots'
lowered programs under the calculus evaluator and their rendered `rust-std`
packages under the pinned rustc compute the three FIPS 180-4 example digests,
every vector of the NIST CAVP `SHA256ShortMsg.rsp` and `SHA256LongMsg.rsp`
files (`tests/stdlib/sha256/`), and the digest of the `sha2` crate on a seeded
corpus (every length 0..200, then random lengths to 4 KiB, and the million-`a`
example under rustc). The oracles are checked against each other first, and one
disagreement with them fails the case.

- **one bit of the round function.** `a := (add t1 t2)` becomes
  `a := (prim bit_xor (add t1 t2) (1 : UInt32) : UInt32)`, so the new `a` of
  every round is off in its low bit. `-- conformance_sl_02`, without its Lean
  half:

  ```text
  panicked at crates/conformance/src/cases/stdlib_sha256.rs:305:9:
  sha256: the evaluator and the oracles disagree on 372 of 372 runs: ["run 0 on [Bytes { hex: \"616263\" }]: {"constructor":0,"fields":[{"kind":"u32","value":"3804428771"}, …],"kind":"adt"} != {"constructor":0,"fields":[{"kind":"u32","value":"3128432319"}, …],"kind":"adt"}", …]
  ```

  `3128432319` is `0xba7816bf`, the first word of the FIPS 180-4 digest of
  `abc`. The same plant is made by the case itself on a copy of the project on
  every run, so the gate stays armed.

Removed: the source was restored; `conformance_sl_02` passes, in 305 s with
the production export of both roots under the pinned Lean (`conformance_sl_01`
in 206 s on the same tree).

`SL-05` holds `LexLeanStd.Buffer` (#92) to Rust's concatenation: the three roots
`writeAll`, `writeEach`, and `writeNumbers` under the evaluator and under the
pinned rustc agree with the concatenation of the pieces, the bytes, and the
decimals on a seeded corpus whose sizes cross the chunk boundary, and with the
refusal at one byte beyond the capacity. Why the member is chunked: a chain of
`append_bytes` of one byte copies both operands at each call, so the work the
rendered runtime counts quadruples as n doubles. Measured with the generated
runtime of `is_closed_lean_name`, release build, n one-byte appends:

| n | work | time |
| --- | --- | --- |
| 10,000 | 50,005,000 | 2 ms |
| 20,000 | 200,010,000 | 6 ms |
| 40,000 | 800,020,000 | 37 ms |
| 80,000 | 3,200,040,000 | 184 ms |
| 160,000 | 12,800,080,000 | 963 ms |

At the size of the largest committed library (256,713 bytes) the chain costs
about 3.3e10 units (n squared over two). The chunk size is the measurement of
the same bytes written as pieces of twelve bytes (`writeAll`, what a printer
writes) and one by one (`writeEach`), in work units, the time being flat at
about 50 ms and 350 ms at every size:

| chunk | `writeAll`, 12-byte pieces | `writeEach`, 1-byte writes |
| --- | --- | --- |
| 128 | 4,359,112 | 19,634,868 |
| 256 | 5,512,876 | 35,803,264 |
| 512 | 7,953,561 | 68,379,787 |
| 1,024 | 13,210,575 | 133,761,735 |
| 2,048 | 23,855,901 | 264,577,610 |
| 4,096 | 45,391,365 | 525,826,906 |

A write copies at most the pending chunk, so its work is proportional to the
chunk; the join at the end (pairwise merging) adds n log k. The module seals
at 256 bytes. A chunk of 128 would save 21 percent of 256's work on writes of a
dozen bytes and 45 percent on one-byte writes, and costs twice the chunks to
join; the time is flat across the sizes, and the work at 256 is a thousandth
of the chain's at this size, so the choice between 128 and 256 is not
material. One-byte writes then cost 551,000 units for 4,096
bytes, 2,236,000 for 16,384 and 9,077,000 for 65,536 (about 135 units a
byte at the larger sizes, linear). The contiguous realization the next rendering work can give
the interface (a registry primitive that writes in place) replaces the chunks
without touching a root. The case holds the proportion of those three work
counts within a factor of two of linear.

- **a boundary that refuses at the capacity itself.** `(grown <=? (project
  buffer capacity))` becomes `(grown <? (project buffer capacity))`: a buffer
  of exactly its capacity is refused. Run 5 of `writeEach`: the program gives
  the refusal, the oracle the byte `00`.
- **a buffer that never seals.** `(256 <=? (prim length joined : Nat))`
  becomes `(256 <=? 0)`: the bytes are the same, the pending chunk grows by
  one append per write, which is the chain this member replaces. Under rustc
  the work of 4,096 and 16,384 one-byte writes is 8,394,753 and 134,242,305, a
  ratio of 16 for a ratio of 4 in size, and the case fails its proportion.

Certification: `LexLeanStd.Buffer` verifies under the pinned Lean with exact
axiom policies; only `writeByte`, `byteStep`, and `writeEach`, which reach
`to_nat`, state `[Quot.sound, propext]`, and no other definition states an
axiom. The three roots are eligible for `rust-std` with `allocation` and
`overflow`. `conformance_sl_05` passes up to the comparison with the committed
packages (the records are regenerated at landing).

`BS-06` holds the two production roots of `LexLeanBoot.RustPrint`
(`bootstrap/src/RustPrint.lex.tex`, over `RustAst.lex.tex`) to the oracle,
`calculus::rust::ast::{print, print_expr}`. The case runs 615 vectors of
`printCrate` (472 the distinct committed crates: 236 lowered by the renderer
from 177 programs, and the crates of 260 packages; the hand-built crates; the
real header of each profile; 57 refused for a capacity one byte short) and
1,955 of `printExpr` (1,404 hand-built: every literal at the edge of each
width and sign, the 128 ASCII scalars and the edges of UTF-8 in strings, byte
strings, every runtime row and machine primitive with its operands and with
one too many, every constructor, pattern, block, conditional, match with a
scrutinee that begins with a brace, and loop) under the pinned rustc on
every host (42 harness binaries, 504 s: the part the host matrix runs), and on
the normative host the evaluator on 544 + 1,955 of them (the crates of at most
6,000 nodes of data), 400 + 399 seeded mutants of the crates of at most 80
expressions, 2,984 more mutants under rustc (67 s), twelve planted defects, and
the production export of the project (the whole case: 1,658 s on this host,
4 cores). The two roots certify in 15 min 57 s (`production build` of the
two roots), and the packages the export publishes are the files the
regeneration commits. Each root's rendered package recurses with the depth of
its input; the stack the harness needs, measured by bisecting the stack of the
built binary on the case's own corpus in a debug build, is 7,077,888 bytes for
`print_crate` and 4,259,840 for `print_expr` (the rows of `tests/stack-bounds.json`
that the landing regeneration records). The committed libraries pin the printed
items; the header before them is the runtime of the committed export, which
moves with the runtime (#248), so the case holds the header to the vectors that
give the roots the oracle's own header and holds the committed libraries to the
items after it.

Each of these twelve plants is made on a copy of
`bootstrap/src/RustPrint.lex.tex` on every run (the first disagreement, from
the case; run numbers index the vectors of the root, the hand-built ones
before the committed):

- **the quote is not escaped**, `printExpr`, run 80: the program gives
  `Str::lit("""`, the oracle `Str::lit("\"")`.
- **the hexadecimal of a scalar is in upper case**, `printExpr`, run 20: the
  program gives `Bytes::lit(&[0xFF, 0x00, 0x10])`, the oracle the lower case.
- **the delete character is printed as it is**, `printExpr`, run 83, the
  string of the 128 ASCII scalars.
- **the least i64 is spelled as a negative literal**, `printExpr`, run 12: the
  program gives the refusal of the buffer (the text is one byte longer), the
  oracle `i64::MIN`.
- **a width's least value is spelled as a literal**, `printExpr`, run 54:
  `-128i8` for `i8::MIN`.
- **the byte string separator has no space**, `printExpr`, run 20:
  `[0xff,0x00,0x10]`.
- **a unit result is printed as an arrow**, `printCrate`, run 58.
- **the indentation is three spaces**, `printExpr`, run 1336.
- **a comparison under a negation is not parenthesized**, `printExpr`, run
  232: `!v0 - 1` for `!(v0 - 1)`.
- **an empty else is written out**, `printExpr`, run 1339.
- **the blank line between items is dropped**, `printCrate`, run 0.
- **the empty dispatch is matched with arms**, `printCrate`, run 16.

The case's first failure, before the module existed (R3), was `no bootstrap
case is wired for BS-06`, and `no stdlib case is wired for SL-05` for the
member. Removed: the source was not changed (the plants are made on copies).

Findings while writing the module, for the next components. Certificate A
cannot name the variable a `match` is on inside one of its branches (`local
`node` is unbound`): the module passes the fields of a pattern instead. A
`bit_and` on a `Nat` is refused (it needs fixed-width operands): the
module takes a remainder. The names `id` and `first` are reserved by the
language and by Lean, and the captures of a function are sorted. The
exact axiom policy of a definition is read from the construct it reaches
(`to_nat` states both axioms; the other constructs of this module state
none), and the verifier reports one definition per run, so the first
declaration of an axiom set is found by reading the construct, not by running.

`BS-07` holds the three roots of the failure typing of the lowering to the
oracle: `siteTrees` of `LexLeanBoot.CalculusCheck` (the tree of typing sites,
`calculus::sites::trees`), and `deadFunctions` and `fallibleFunctions` of
`LexLeanBoot.RustLower` (`bootstrap/src/RustLower.lex.tex`; the dead functions
are the functions the crate of `rust::lower` renders as an empty match, and the
failures are `calculus::rust::fallible_functions`). No file of the renderer
(`RENDERER_FILES`) is changed, so the renderer's digest does not move. The case runs 309
vectors per root (the 22 programs of the rules of the analysis, the programs of
the corpora of `BS-04` that the checker accepts, and the committed target
programs) under the pinned rustc on every host (38 harness binaries built in
531 s, 927 runs in 17 s more), and on the normative host the evaluator on the
272 of them of at most 300 nodes with 150 seeded mutants (37 s for
`siteTrees`, 12 s for `deadFunctions`, 23 s for `fallibleFunctions`), 4,500
more runs for the 1,500 seeded mutants under rustc (25 binaries, 189 s),
thirteen planted defects, and the production export. The case up to the
comparison with the committed packages takes 1,990 s on this host (4 cores);
the packages are the maintainer's regeneration at landing.

Measured certificates (the largest part of each root, in bytes; the file limit
`max_file_bytes` is 4,194,304 and three quarters of it, which the case asserts
for each root, is 3,145,728):

| Build | Largest part |
| --- | --- |
| one root, the whole analysis (collect, edges, liveness, reach, failures) | 4,465,840 (refused, `LLS8002`) |
| the same in two roots, `deadFunctions` and `fallibleFunctions`, each calling `walk` | 3,987,444 and 3,841,069 |
| `walk` alone, as the root `inferTy` (a probe, not committed) | 2,977,029 |
| `siteTrees` with its body chain as a second recursion over `Expr` | 3,157,718 (11,990 over the budget) |
| `siteTrees` with the chain folded into `siteTree` | **3,144,673** (1,055 under) |
| `deadFunctions` and `fallibleFunctions` reading the tree | **1,250,877** and **1,003,841** |

The margin of `siteTrees` is therefore 1,055 bytes, and the second `match` over
`Expr` that the fold removed (16 arms, 120 pairs of arms) bought the 12,000
bytes. The rule for the later slices of `BS-07`: nothing is added to the
closure of `siteTrees`; a slice that needs more from the typing root splits it,
by item or by kind of site, and measures, rather than growing it. The consumers
sit at 30 and 24 percent of the limit, and that is the room the remaining
slices live in. The typing root is `walk` and `inferTy` of `CalculusCheck`
(one copy, the one `checkProgram` runs on), and `RustLower` holds no second
copy: its own `inferTy` and `armFields` were moved into the module and removed
from it. The roots recurse with the depth of their input; the stack bounds that
the case measured in a debug build, which the landing regeneration records, are
5,242,880 bytes for `site_trees`, 4,063,232 for `dead_functions`, and 4,128,768
for `fallible_functions`.

The evaluator share is bounded by a measurement, not an estimate. The first
design (one root holding the analysis, with `walk` in it) took 1,789 s on the
295 vectors of the corpus then. With the typing root the 272 vectors within 300
nodes take 37 s for `siteTrees` (the root that carries `walk`), and all 309
vectors take 874 s for it, so the 37 largest programs cost 835 s on one root;
the bound of 300 nodes keeps the three roots at 72 s with the mutants. A
program above the bound is still a vector under rustc on every host.

Each of these thirteen plants is made on a copy of `bootstrap/src/RustLower.lex.tex`
(the first ten) or `bootstrap/src/CalculusCheck.lex.tex` (the last three) on
every run, and is caught on the leg of its root; the run is the index among the
vectors of the root (the programs of the rules first):

- **an addition of naturals does not fail**, `fallibleFunctions`, run 0.
- **a successor does not fail**, `fallibleFunctions`, run 6.
- **a parse of an integer does not fail**, `fallibleFunctions`, run 90: the
  program gives `[false]`, the oracle `[true]`.
- **a call is not an edge**, `fallibleFunctions`, run 0.
- **an application is not an edge**, `fallibleFunctions`, run 7.
- **the closures of a dead function count**, `deadFunctions`, run 9: the
  function whose only argument type is inhabited by the closure of a dead
  function is dead in the second round, not in the first.
- **the failures are followed one round**, `fallibleFunctions`, run 0.
- **a closure of any function type makes an application fail**,
  `fallibleFunctions`, run 7.
- **no type holds itself**, `deadFunctions`, run 12.
- **every function type is inhabited**, `deadFunctions`, run 9.
- **an applied target reads as no type**, `siteTrees`, run 7.
- **the body of an arm is typed without its binders**, `siteTrees`, run 131.
- **the body of a let is typed without its binding**, `siteTrees`, run 21.

The case's first failure, before the module existed (R3), was `no bootstrap
case is wired for BS-07`. Removed: the source was not changed (the plants are
made on copies).

Findings while building the roots. (1) A gate that reads its own vectors can
be vacuous without failing: eight of the programs of the rules were refused by
the checker (a quotient and a remainder with two operands, and types with no
constructor), and `vectors` drops a program the checker refuses, so the plant
of "the closures of a dead function count" was undetected until the case found
that the program built to catch it had never been a vector. The programs are
now valid (an argument no value inhabits is a function type no closure
inhabits), and the case fails if a program of the rules is refused or if none
of them has a dead function. (2) The outcome of the typing root nests with the
length of the chains of a program, past the 128 levels that the JSON reader
allows by default; the harness reads an outcome line without the limit, as the
other readers of a deep value do. (3) Certificate A cannot name the matched
variable in a branch, and a function that returned the matched chain in its
leaves rebuilds it from the fields of the pattern. (4) The exact axiom policy
of a definition follows the construct it reaches, so a definition that reached
`walk` states `[Quot.sound, propext]` and states none once it no longer does
(`LLV7005` says so, one definition per run). (5) The committed-package
comparison of the case ends it on a tree whose packages were not regenerated:
on a clean checkout at the assertion that the exported package files are the
committed ones (the three package directories are absent), and on a tree with
the three packages copied in at `audit_tokens/package.json`, which differs
because every package carries the identity of the project, as for `BS-06`.

`BS-07`, second slice: `lowerTypes` of `LexLeanBoot.RustLower` returns the
items of the crate that declare its types (the `Enum` of each ADT and of each
function type, and the `Apply` of each function type) as a `RustAst.Def`
chain, held to the items of the crate that `rust::lower` builds before its
first function. Its input is the program, the tree of `siteTrees`, the dead
functions, and the failing functions, so the case feeds it the legs of the
vector before it. The case runs 310 vectors per root (one more than the first
slice: the package of `lowerTypes` is a committed program; the measurement is on
the tree of that slice, and with the package of `applyMoves` of `BS-08` the
vectors are 311 per root and the runs 1,244) under the pinned
rustc on every host (58 harness binaries built in 875 s, 1,240 runs in 23 s
more), and on the normative host the evaluator on the 272 of them of at most
300 nodes with 150 seeded mutants (53 s for `siteTrees`, 14 s for
`deadFunctions`, 24 s for `fallibleFunctions`, 26 s for `lowerTypes`), 6,000
seeded runs under rustc (35 binaries, 278 s), twenty planted defects, and the
production export. The case up to the comparison with the committed packages
takes 2,589 s on this host (4 cores), and stops there on this tree with the
packages and the records left to the maintainer's regeneration: the packages
`lower_types`, `dead_functions` and `fallible_functions`, `bootstrap/expected/`,
the locks, and the PD-08 rows. The rows measured in a debug build with the four
packages of `BS-07` are 4,063,232 bytes for `lower_types`, 4,063,232 for
`dead_functions` (as in the first slice), and 4,063,232 for `fallible_functions`
(4,128,768 in the first slice: one step of the bisection less, with the corpus of
the second slice).

Measured certificates after the slice (the largest part of each root, in
bytes; the budget is 3,145,728, three quarters of `max_file_bytes`):

| Root | Largest part | Share of the limit |
| --- | --- | --- |
| `siteTrees` | 3,144,673 | 75.0% (1,055 under the budget; unchanged) |
| `deadFunctions` | 1,273,118 | 30.4% (was 1,250,877) |
| `fallibleFunctions` | 1,058,398 | 25.2% (was 1,003,841) |
| `lowerTypes` | 1,535,849 | 36.6% |

Nothing was added to the closure of `siteTrees`, as the rule for the slices
requires: the typing root is unchanged, and the two consumers grew by 22,000
and 55,000 bytes because the collector they share now numbers the function
types as the oracle does. Before the slice was built, a probe root returning a
`Def` chain of `n` empty enums certified under the pinned Lean with parts of
48,495 and 40,258 bytes, so certificate B takes a value of a recursive sort as
an output and no chain encoding is needed; the `Def` of the output is the one
`RustPrint` prints.

The numbering of the function types is part of the output, and it is the
oracle's: the types of each function and its result before its body, the type
of a closure and its record before the closures in its captures, and the
fields of the ADTs after all the functions. Two differences between the first
slice's collector and the oracle did not matter for the flags and matter for
the numbers: the closure was recorded after its captures, and the chain of
known types was built newest first. Both are fixed in the collector, and the
plant "function types are numbered newest first" (run 7) is the second.

Each of these seven plants is made on a copy of `bootstrap/src/RustLower.lex.tex`
on every run and is caught on the leg of `lowerTypes`; the run is the index
among the vectors of the root:

- **a field that holds its owner is not boxed**, run 11.
- **a boxed capture is cloned**, run 130.
- **an application of a function type is never fallible**, run 7.
- **the closures of a function type are listed newest first**, run 7.
- **a function type with closures is written as the stated type**, run 7.
- **function types are numbered newest first**, run 7.
- **a string capture is copied**, run 128.

The counts of `BS-05` and `BS-06` are restated on the tree with the four
packages of `BS-07` (259 manifests and packages, 176 programs, 235 crates; 455
and 318 vectors of `BS-05`; 613 vectors of `printCrate` and 470 committed
distinct crates of `BS-06`, 545 of the evaluator's), computed by running their
cases on that tree.

`BS-08` holds the production root of `LexLeanBoot.RustMoves`
(`bootstrap/src/RustMoves.lex.tex`), the last-use moves of the lowering, to the
public oracle `calculus::rust::moves::apply`, the first pass of the function
bodies and a case of its own (the first draft of it was a leg of the case of
`BS-07`, whose run, with the mutants drawn from the
compiler's own crates, reached 14 GB of memory and was killed after 44 minutes; the case is split by pass
and the mutation bases are the crates of at most 80 expressions, as `BS-05`
draws them: 4.7 GB resident). The case runs 352 crates (the hand-built
rule crates, the hand-built crates of `BS-05`, and the crates the renderer
lowers from the 260 committed manifests) under the pinned rustc on every host
(21 harness binaries built in 263 s, 352 runs in 24 s more), and on the
normative host the evaluator on the 317 of them of at most 400 expressions with
150 seeded mutants (47 s), 1,500 seeded mutants under rustc (9 binaries, 54
s), eight planted defects, and the production export. The case up to the
comparison with the committed packages takes 1,734 s on this host (4 cores),
and stops there on this tree with the packages and the records left to the
maintainer's regeneration: the package `apply_moves`, `bootstrap/expected/`,
the locks, and the PD-08 row `bootstrap/apply_moves`, which the case measured in
a debug build as 6,029,312 bytes.

Measured certificate of `applyMoves` (bytes; the limit `max_file_bytes` is
4,194,304 and the budget the case asserts is 3,145,728): part A 551,083, part B
565,927 (the largest, 13.5% of the limit), part E 164,725; the export took 19
min 46 s. The exact axiom policy observed is `Classical.choice`, `Quot.sound`,
and `propext`. The pass is 218 lines of oracle source, about 2,600 bytes of
certificate per line. The typing root `siteTrees` is untouched, as the rule for
the slices requires.

The pass is a backwards liveness pass over the 17 constructors of `Expr` with
the live set an association list of counts. It is written as two recursions
beside each other, the reads of an expression given what is read after it, and
its rewrite given the same, because the structure that holds a recursive sort
beside the set was not needed: the rewrite never returns the set. Each of
these eight plants is made on a copy of `bootstrap/src/RustMoves.lex.tex` on
every run and is caught by the oracle; the run is the index among the crates
(the rule crates first):

- **a last read never moves**, run 0.
- **a component moves though another is read twice**, run 8.
- **the reads of a component on two alternatives are added**, run 9.
- **the holder of an application is not read before its operands**, run 21.
- **the left of a pair is read before its right**, run 23.
- **a pattern does not unbind its names**, run 3.
- **a pattern that binds both components is a projection**, run 11.
- **the state of a loop is not read**, run 24.

Findings. (1) A plant is only as strong as the crates that reach it: five of
the first drafts of these were not detected, and each turned out to be
equivalent or too weak on the crates then in the corpus: replacing one arm of
`mayMove` left the other answering `true`; the order of two reads in the
function that only counts them does not change the counts, so the plant went to
the function that rewrites; the sum of the reads of two alternatives is
non-zero where their maximum is, so it went to the counts of the components. A
rule crate was added for each that needs one (a component projected while
another is read twice after it, a component read once on each of two
alternatives, an application whose holder is also an operand, a pattern that
binds both components after a projection, a read of the state before a loop).
(2) The whole case takes 29 minutes (1,734 s) on this host, and `BS-07` keeps its own. (3) The counts of `BS-05`,
`BS-06`, and `BS-07` move with the package of this root (260 manifests and
packages, 177 programs, 236 crates; `BS-05` 456 and 319 vectors; `BS-06` 615
vectors of `printCrate` and 472 committed distinct crates; `BS-07` 311
vectors) and are recounted by running their cases on this tree.

## The `bdd` recipe as the parts of one run (Issue #131)

With UOR Atlas verified as the parts of one run, the `rest` leg became the
gate's longest. On the run that merged #152 (CI run 38905653756, head
`55583ba7`) it took 111.5 minutes, of which `bdd` took 5,741 s and the other
ten recipes 903 s. `bdd` is one `cargo test` process, and its harness runs four
test threads on the four-CPU runner. The cases' own wall times
(`cases-telemetry.jsonl`) sum to 22,708 s over those threads, so the threads
were busy for the whole recipe and its time is its work divided by four.

The cases are not independent. A case that needs a verified project, an
exported package set, or a compiled environment reads it from a `OnceLock`
that the first case to ask builds, once per test process. Splitting such a
group across processes builds the fixture once per process. Which fixtures each
case reaches was read from the crate's code, following calls from each case's
arm of its suite's `run` to the functions holding a `OnceLock`, and checked by
hand for the heavy ones. The groups, with the case time of their members on
that run:

| Group | Cases | Case seconds |
| --- | --- | ---: |
| preservation: `reports` (four projects certified side by side), `shipped`, the certification library | `SP-*` but `SP-05`, `GC-09` | 6,890 |
| `verified_compiler`, `verified_reasoning`, the planted calculus verification | `GC-02`, `GN-02/03/07`, `TC-03..05`, `RS-*` | 5,558 |
| the production export, `verified_models`, and `EX-07` | the `CL-*` cases of the export, `PX-*`, `MD-*`, `SP-05`, `EX-07` | 4,595 |
| `verified`, `broken_proof`, the oracle | `VR-*`, `AR-*`, `AU-*`, `LN-*`, `NE-*`, `OR-*`, `SE-*`, `EX-01/02`, `GL-12`, `PF-18` | 4,089 |
| everything else | 223 cases | 1,563 |

These are the parts of `.github/bdd-parts.json`: `preservation`, `compiler`,
`production`, `verification`, and the remainder `rest`. Some fixtures are
still built in more than one part. The shipped preservation environment and the
certification environment are built in `preservation` and `compiler`, because
`GC-02` needs them beside `verified_compiler`, and moving it would verify the
`compiler` project twice instead. The GNAF requests and pairs, the committed
certificate fixtures, and the native certification registry are built in two
or three parts; the host computes them without Lean.
The SPEC text and `axioms_insufficient` are read or verified in two. The
estimate of each part, its case
seconds over four threads plus about seven minutes of checkout, toolchains, and
build, is 36 minutes for `preservation`, 30 for `compiler`, 36 for
`production` (bound by `EX-07`, 29 minutes alone), 24 for `verification`, and
22 for the `rest` leg (15 minutes of its other recipes and 7 of the
remainder). These are estimates; the measurement is the first green run of the
change.

`cargo xtask bdd` is the recipe (`repo_conformance::bdd_parts`, SPEC.md §9.2,
§9.4). With `LEXLEAN_BDD_PART` unset or empty it runs `cargo test -p
repo-conformance --all-features`, the command the recipe always was. With a
part named, it adds that part's harness filters: `--exact` and the part's test
names, or for the remainder `--exact` and a `--skip` of every named test. On
this toolchain `--exact` makes `--skip` exact too: `--exact --skip
conformance_rp_1` still lists `conformance_rp_10` to `conformance_rp_14`, and
without `--exact` lists none of them. The remainder therefore runs the
library tests, the doctests, the meta-gate target, and every conformance test
no part names, an ID added later included. The harness selects the tests; no
case decides whether it runs.

`cargo xtask check-test-inventory` proves the partition from the tests
themselves. It lists `bdd` whole and each part with its filters (`cargo test …
-- --list <filters>`) and requires the parts to be disjoint, their union to be
`bdd`'s list, and every test a part names to be in that part's list
(`inventory::compare_parts`). On this branch, with main through #146 merged
(register 392), it reports the 430 tests of `bdd` as the parts' 21, 14, 39, 91,
and 265 (the remainder's 265 are 227 conformance tests and the 38 tests of the
library and meta-gate targets). The IDs main added after the plan was written
went to the remainder by default (`PF-20` to `PF-23`), except `SP-14`, which
shares the shipped preservation environment and is named in `preservation`.
`RP-14` reads
`ci.yml`: a row names a part exactly when it runs `bdd`, every part of the
plan, the remainder included, is run by exactly one row, a row that runs `bdd`
alone is named `bdd-<part>`, `LEXLEAN_BDD_PART` is set once on the gate job and
from the row, and every other recipe of `vv` still runs exactly once. `RP-13`
holds the recipe line, the arguments the task and the inventory share
(`bdd_parts::CARGO_ARGUMENTS`), that every ID the plan names has its test, and
the comparison's refusals.

What does not change: `just vv`, which runs `bdd` whole; `release.yml`; the
suite and every case in it; the host matrix, which runs the whole workspace on
every host.

### the bdd partition can fail

Each record is a defect planted on this branch, the command that ran, and its
failure, after which the file was restored.

- **an ID named by two parts** (`SP-02` added to `compiler`), `cargo xtask
  check-test-inventory`: `` .github/bdd-parts.json: `SP-02` is named by the
  parts `compiler` and `preservation` ``.
- **an ID with no test** (`SP-99` added to `preservation`), the same command:
  `` RP-13: the part `preservation` of .github/bdd-parts.json names `SP-99`,
  and runs no test `conformance_sp_99` ``.
- **the remainder skipping nothing** (the `--skip` loop removed from
  `Plan::harness_filters`), the same command: `` RP-13: `conformance_ar_01` of
  tests/conformance.rs [conformance] is run by the parts `verification` and
  `rest` of .github/bdd-parts.json ``.
- **a part run by no row** (the `bdd-verification` row removed from `ci.yml`),
  `cargo test -p repo-conformance --all-features --test conformance -- --exact
  conformance_rp_14`: `` §9.4: the part `verification` of bdd is run by no row
  ``.
- **a part run by two rows** (the `bdd-compiler` row naming `production`), the
  same command: `` §9.4: a row that runs bdd alone is named `bdd-production` ``.
- **the `rest` leg running `bdd` with no part** (its `bdd_part` emptied), the
  same command: `§9.4: a row names a part of bdd exactly when it runs bdd`.
- **the part chosen outside the matrix row** (`LEXLEAN_BDD_PART: compiler` on
  the gate job), the same command: `` §9.4: `LEXLEAN_BDD_PART` is read from the
  matrix row ``.
- **the recipe back to the bare `cargo test`** (`bdd:` running `cargo test -p
  repo-conformance --all-features` again), `cargo test -p repo-conformance
  --all-features --test conformance -- --exact conformance_rp_13`: the
  recipe-line assertion (`left: "cargo test -p repo-conformance
  --all-features"`, `right: "cargo xtask bdd"`). `cargo xtask
  check-test-inventory` refuses it as well: `` RP-13: the `bdd` recipe is
  `cargo test -p repo-conformance --all-features`, not `cargo xtask bdd`, the
  command whose arguments the inventory lists ``.

### the bootstrap axiom-output parser can fail its oracle (SPEC.md §17.23)

`BS-01` holds the two production roots of `LexLeanBoot.AxiomOutput` to the
Rust they replace, `verify::axiom::parse_audit_output` and
`ir::core::is_closed_lean_name`. The committed vectors of
`tests/golden/axiom-parser/` (28 runs: 13 accepted, 15 rejected, every line,
every wrapped record, and each file whole) run under the calculus evaluator;
both roots' rendered `rust-std` packages, compiled by the pinned rustc, run
12,000 generated audit outputs and 40,000 names. A run compares the result,
the rejection's message, and the declaration it is attributed to. The case
passed in 413 s locally, with the production export under the pinned Lean, and
makes each of these plants on a copy of the source on every run:

- **the continuation loses its space.** `-- conformance_bs_01`:

  ```text
  plant detected: the continuation joins without the space the unwrapped form has: run 48 on "'A.b' depends on axioms: [propext,\n  'C.d' does not depend on any axioms" ["A.b"]: the program gives error {"`A.b' depends on axioms: [propext,'C.d` is not a Lean name", some "A.b"}, the oracle error {"`A.b' depends on axioms: [propext, 'C.d` is not a Lean name", some "A.b"}
  ```

- **U+0085 is not white space** (the parser trims with `char::is_whitespace`,
  not ASCII white space): run 34, the program rejects
  `"\u{85}'A.b' does not depend on any axioms\u{a0}\n"` where the oracle
  accepts it.
- **a location number may exceed `usize`**: run 150, `A.lean:18446744073709551616:0: info: ...`
  is accepted by the program and rejected by the oracle.
- **axioms ordered descending**: run 1, the program yields
  `["propext", "Quot.sound", "Classical.choice"]` where the oracle yields
  `["Classical.choice", "Quot.sound", "propext"]`.
- **a rejected line loses its declaration**: run 11, the program gives
  `none` where the oracle attributes `unrecognized audit line: ...` to
  `Demo.M.unknown`.
- **a name segment may begin with a digit**: `isClosedLeanName` of `"9"`,
  the program `true`, the oracle `false`.
- **the duplicate record check is dropped**: run 76, two records for `A.b`
  are accepted where the oracle says ``duplicate record for `A.b` ``.

Removed: the source was not changed (the plants are made on copies).

Authoring found three shapes the certificate generator does not accept, each
reduced in a scratch project before being avoided in the source (reported on
#92): a user call that is the whole `then` or `else` of an `if` (or the right
operand of `&&`/`||`), and an `iterate_until` whose initial state holds a
parameter, are each rejected by certificate A with an application type
mismatch (`LLV7013`); the module binds such a call to a `let` first, and
spells a decimal without `iterate_until`. Lean's exact axiom policy was
aligned declaration by declaration (`Quot.sound`, `propext` for the `UInt8`
range checks and byte comparison, as the spike on #92 measured).
### the bootstrap preservation lexer and token audit can fail its oracle (SPEC.md §17.23, §17.17)

`BS-02` holds the two production roots of `LexLeanBoot.TokenAudit` (`bootstrap/src/TokenAudit.lex.tex`) to the stage-0 Rust oracle they replace, `production::preserve::lex` and `production::preserve::audit_tokens`. The corpus of 1,731 deterministic Lean 4 source vectors (accepted modules, rejected control characters, unclosed comments/strings/raw strings/char literals/quoted names, malformed radix/exponent/field-index numbers, interpolated strings, forbidden commands, forbidden tokens inside and outside `@[...]`/`attribute [...]`, qualified forbidden constants, disallowed options, and unlisted imports) and 300 seeded combinations per root run under the calculus evaluator; both roots' rendered `rust-std` packages, compiled by the pinned `rustc`, run 5,731 inputs each. The case passed locally with the production export under the pinned Lean (`audit_tokens` and `lex` committed under `tests/production-export/bootstrap/packages/rust-std/`), and makes each of these seven plants on a copy of `bootstrap/src/TokenAudit.lex.tex` on every run:

- **a nested block comment does not increment depth** (`depth := (dep + 1)` -> `depth := dep`). `-- conformance_bs_02`:

  ```text
  BS-02: plant detected: a nested block comment does not increment depth: run 48 on "/- /- nested -/ theorem t : False := sorry\n": the program gives ok [...], the oracle error "a comment is not closed"
  ```

- **a hexadecimal literal rejects uppercase `A..F` digits** (`(inRange byte (65 : UInt8) (70 : UInt8))` -> `(inRange byte (65 : UInt8) (65 : UInt8))`): run 60 on `"def x := 0x1_Fsorry\n"`, the program gives `error "a number has a digit separator or a radix prefix with no digit after it"` where the oracle lexes `0x1_F` followed by `sorry`.
- **an interpolated string is not refused** (`(if (if (not (project firstSeg quoted)) then (if isBanged then isQuoteAfter else false) else false)` -> `(if false`): run 44 on `"def s := s!\"{sorry}\"\n"`, the program lexes `s!` and `"{sorry}"` where the oracle refuses with `"an interpolated string holds code"`.
- **a C1 control character (`U+0080..U+009F`) is not refused** (`(byteIn subject (position + 1) (128 : UInt8) (159 : UInt8))` -> `(byteIn subject (position + 1) (128 : UInt8) (128 : UInt8))`): run 97 on `"def x := 1\u{85}\n"`, the program lexes `\u{85}` as a symbol where the oracle refuses with `"a control character U+0085 in the text"`.
- **a quoted forbidden attribute inside `@[...]` is not refused** (`(let activeTok : Bool := (if (not (project onlySeg quoted)) then true else (0 <? attributeDepth));` -> `(let activeTok : Bool := (not (project onlySeg quoted));`): run 23 on `"@[«implemented_by» g] def h := 1\n"`, the program gives `ok ()` where the oracle refuses with `"the forbidden token `implemented_by`"`.
- **a qualified forbidden constant is not checked** (`(let bad : Bool := (isForbiddenConstant (project seg text));` -> `(let bad : Bool := false;`): run 20 on `"def x := Lean.ofReduceBool\n"`, the program gives `ok ()` where the oracle refuses with `"the forbidden constant `ofReduceBool` (in `Lean.ofReduceBool`)"`.
- **an import of an unlisted module is accepted** (`(let okMod : Bool := (bytesInList modName imports);` -> `(let okMod : Bool := true;`): run 94 on `"import Mathlib\n"`, the program gives `ok ()` where the oracle refuses with `"the import of `Mathlib` is not allowed"`.

Removed: the source was not changed (the plants are made on copies).

### the bootstrap primitive scanner can fail its oracle (SPEC.md §17.23, §12.2)

`BS-03` holds the two production roots of `LexLeanBoot.PrimitiveScan`
(`bootstrap/src/PrimitiveScan.lex.tex`) to the stage-0 Rust oracle they replace,
`source::scan::scan` and `source::scan::reject_forbidden_atoms`. On every host
the rendered `rust-std` packages, compiled by that host's pinned rustc, run
the deterministic vectors, a 128-byte window and a 2,048-byte prefix of every
committed `.lex.tex` source, and the whole sources up to 8,192 bytes (up to
1,024 bytes and one source in four for `rejectAtoms`); on the normative host
the lowered programs run under the calculus evaluator on the vectors, the
windows, and 80 seeded texts, the packages run 2,500 seeded texts, and ten
planted defects are each detected. A run compares every atom (class, text,
byte, line, and column spans) or the rejection (code, message, span, help).
Measured locally on the normative host (4 cores): 4,565 `scanSource` and 815
`rejectAtoms` rustc runs agree, 907 evaluator runs per root agree, the corpora
take 66 s to build, rustc 76 s to build the two packages, and the case took
about 380 s before the Lean-backed export (shared by the cases of the bootstrap
project in one process) and 877 s with it. The ten plants, each made on a copy
of the source on every run (`-- conformance_bs_03`):

- **a run of white space stops at a line feed** (`(if space then true else feed)`
  -> `space`): run 1, `\begin{x} ab 12 + ℕ\n`.
- **a backslash before any scalar below U+0100 is a control** (`nbAscii` over
  `0..255`): run 5, `\ℕ`, which the oracle refuses with `LLL1004`.
- **a column counts the bytes of a scalar**: run 1, the column after `ℕ`.
- **the atom limit admits one atom more**: run 37, `a b c` with a limit of 4.
- **a lone zero has a redundant leading zero**: run 13.
- **a numeral after identifier material need not touch it**: run 147, an atom
  list with a gap, which the scanner never produces and the case therefore adds.
- **a leading zero inside a payload string is rejected**: run 106.
- **a coremodule environment is not a payload**: run 130, a quoted `007` in a
  `coremodule` body.
- **an escaped backslash escapes the next scalar**: run 110.
- **a brace opens no level of a payload**: run 118, a payload with an inner
  brace.

Removed: the source was not changed (the plants are made on copies).

Authoring found no shape the certificate generator refused: `lexlean verify`
passed on the first attempt apart from the exact axiom policy, which was
aligned declaration by declaration (six declarations observe no axioms, the
others `Quot.sound` and `propext`).

### the stacks the harnesses give a recursive root can fail their record (`PD-08`, SPEC.md §17.13, §17.23, #105 step 3)

A root that admits `recursion` is correct only under a stack its depth fits,
and nothing in a package bounds that depth (§17.16). The conformance harnesses
are the host of the roots they run, and gave them 64 MiB (`rust_harness`) and
1 GiB (the `BS` evaluator thread) by habit. `tests/stack-bounds.json` records
what the cases' own inputs need, written by the cases themselves with
`LEXLEAN_MEASURE_STACK=1` (`all`) or `=scaling` on Linux x86-64 with rustc
1.97.1; `PD-08` reads it. It is evidence at level `build` from one host and
these inputs, not a stack bound for any program or host.

Method. A harness binary reads `LEXLEAN_HARNESS_STACK` for the stack of its
thread; `stack_bounds::least` doubles it from 64 KiB until the binary
finishes (exit status 0 and the `work` line) and bisects the last doubling to
64 KiB. This is the issue's method, applied to the debug build, which is the
build the cases run. The evaluator is measured by the resident size of its
thread's stack after the `BS-01` and `BS-02` workload, because a bisection
reruns a workload of about 40 minutes per probe. A debug harness holds the
corpus as an array literal in its frame, so the bound of a root is the frame
of its corpus plus the depth of its recursion. A case that runs one root's
harness on several corpora (`BS-04`) records the largest need among them.

Since `RB-14` renders a function whose self calls are all tail calls as a
loop, the rows are the five packages whose provenance still states
`recursion`: the audit-output parser, whose sorted insertion of each record
among the ones before it is not a tail call, the two calculus checkers, which
recurse with the depth of the program they check, and the two Rust crate
validators (`BS-05`, `validate_crate` and `correspond_crate`), which recurse
with the depth of the crate they check. The validators were measured with
`LEXLEAN_MEASURE_STACK=all` on `conformance_bs_05` alone, over the 675 vectors
and the 4,000 seeded mutants of its rustc differentials, the largest need of
all of them being the row. The scans, the token
roots and the standard library's text and SHA-256 roots are loops and need no
row (`observe` returns at once for a package that does not recurse, and
stays in their cases so that a root that recurses again is held to a row).

| subject | least stack (bytes) | with headroom 4 |
| --- | ---: | ---: |
| `bootstrap/evaluator` | 21,037,056 | 84,148,224 |
| `bootstrap/parse_audit_output` | 6,291,456 | 25,165,824 |
| `bootstrap/check_program` | 13,697,024 | 54,788,096 |
| `bootstrap/check_arguments` | 4,915,200 | 19,660,800 |
| `bootstrap/validate_crate` | 5,046,272 | 20,185,088 |
| `bootstrap/correspond_crate` | 4,915,200 | 19,660,800 |

Both stacks in use cover the largest product: 64 MiB (67,108,864) for the
harness against 54,788,096 (the two new rows have products of 20,185,088 and
19,660,800), and 1 GiB for the evaluator against 84,148,224.

The growth of one root with its input is measured on `parseAuditOutput`, the
one root of the first component that still recurses: one audit output of `n`
records, each the axiom line of its own declaration (46 bytes a record) with
names that ascend, so that each insertion walks the whole sorted list before
it (a single call, its argument passed through `black_box` so that a release
build cannot fold the parse). The issue's table was of `isClosedLeanName`,
which is a loop since `RB-14` and needs one 64 KiB step at every length.

| `n` | bytes | debug | release |
| ---: | ---: | ---: | ---: |
| 250 | 11,500 | 327,680 | 131,072 |
| 1,000 | 46,000 | 1,048,576 | 327,680 |
| 2,000 | 92,000 | 2,097,152 | 655,360 |
| 4,000 | 184,000 | 4,128,768 | 1,245,184 |

Debug costs about 1,048 bytes a record, so the 64 MiB of the harness covers
an output of 2,000 records with headroom 4 (8,388,608), which is
the planted size; `PD-08` asserts that every row up to the planted size fits
the harness stack with headroom and that the table grows. The harness holds
its input as one literal, a chain of `cons` as deep as the record count, and
a release build of a chain past a few thousand records takes rustc the better
part of an hour, so the table stops at 4,000. Release grows too (the
insertion is not a shape LLVM turns into a loop), at about a third of the
debug cost; the debug column is the one the suite's stacks are held to.

`PD-08` makes each of these plants on every run's input (the record, a stack
constant, or a case input is changed in a clean tree, the case run, the failure
observed, paths shortened, and the change removed):

- **the harness stack lowered below a bound.** `rust_harness::HARNESS_STACK`
  set to `4 << 20`. `cargo test -p repo-conformance --test conformance --
  conformance_pd_08 --exact`:

  ```text
  panicked at crates/conformance/src/cases/bootstrap.rs:1620:75:
  bootstrap/check_arguments: a stack of 4194304 bytes is below its recorded bound with headroom (19660800 bytes)
  ```

- **the evaluator stack lowered below its bound.** `EVALUATOR_STACK` set to
  `64 << 20`:

  ```text
  panicked at crates/conformance/src/cases/bootstrap.rs:1620:75:
  bootstrap/evaluator: a stack of 67108864 bytes is below its recorded bound with headroom (84148224 bytes)
  ```

- **a recursive package with no row.** The row of `bootstrap/check_arguments`
  deleted from `tests/stack-bounds.json`:

  ```text
  assertion `left == right` failed: the record has a row for each recursive package of the bootstrap and stdlib exports and for no other
    left: {"bootstrap/check_program", "bootstrap/parse_audit_output"}
   right: {"bootstrap/check_arguments", "bootstrap/check_program", "bootstrap/parse_audit_output"}
  ```

- **a row for no recursive package.** A row for `stdlib/trim` (a loop since
  `RB-14`) added: the same assertion, with `stdlib/trim` on the left only.

- **a recorded need that is too large.** The debug rows of the 2,000- and
  4,000-record outputs multiplied by 16:

  ```text
  panicked at crates/conformance/src/cases/bootstrap.rs:1635:9:
  an audit output of 92000 bytes needs 134217728 bytes with headroom, and the harness stack is 67108864
  ```

- **a recorded need that is stale.** Every debug row of the growth table
  divided by 8, as after a change that makes a frame eight times larger; the
  harness of `parseAuditOutput` on an output of 2,000 records is built and run
  on the stack the record now allows:

  ```text
  panicked at crates/conformance/src/cases/bootstrap.rs:1673:5:
  an audit output of 2000 records: the harness finishes on 1048576 bytes, its need with headroom
  ```

Removed: each edit was restored and `conformance_pd_08` passes. One
direction is not caught, and the headroom is why: a recorded need smaller than
the true one by less than a factor of 4 still finishes, which is what the
headroom of 4 is for. A recursive root that a harness runs without a row fails
the case that runs it (`stack_bounds::observe` in `BS-01`, `BS-02`, `BS-03`,
`BS-04`, `SL-01`, `SL-02`); that was not planted separately, because the set
equality above is the same fact read from the export.
### the bootstrap calculus checker can fail its oracle (SPEC.md §17.23, §17.14)

`BS-04` holds the two production roots of `LexLeanBoot.CalculusCheck` (`bootstrap/src/CalculusCheck.lex.tex`) to the stage-0 Rust oracle they replace (the counts below are the case's report at b5871e8b9 on its base; the committed corpus has grown since), `calculus::check::check` and `calculus::check::check_arguments`, for a program given as calculus data. The case decodes the arguments of every vector back to the program the oracle read (each list of a recursive type is a chain of its type's own constructors, a faithful image of the list that certificate A can take, #208), runs the 175 committed target programs, the refusals of `TC-02`, one program for each rule a planted defect removes, and the literal, primitive, and shape corpora under the rendered `rust-std` packages on every host, and on the normative host under the calculus evaluator, with 4,722 seeded programs and 403 seeded argument runs and 9,296 more under `rustc`. A run compares the result and the reason to the byte; the runs whose reason spells a scalar beyond U+009F as `\u{…}` are outside the claim and left out (the reasons of Rust's `Debug` for a string past the tables of the module). The case makes each of these twelve plants on a copy of `bootstrap/src/CalculusCheck.lex.tex` on every normative run:

- **the exhaustiveness of a match is not checked** (`(firstMissing required seen)` -> `(ctor Option.none [Shape])`): run 3, the program gives `ok ()` where the oracle says `function 0: the match is not exhaustive: Adt { constructor: 2 } is not covered`.
- **a second arm for one shape is accepted** (`(if (shapeIn covered shape) then` -> `(if false then`): run 4, `ok ()` where the oracle says `the arm for Adt { constructor: 0 } is unreachable`.
- **a closure may capture every parameter** (`(total <=? given)` -> `(total <? given)`): run 19, the program's `closure of function 2 operand 1 has type List { element: Nat }, expected Nat` where the oracle says `captures 2 of its 2 parameters; at least one must remain`.
- **a nat literal may be one past the largest u64** (`18446744073709551615` -> `…616` in the literal check): run 14, `ok ()` where the oracle says `nat literal 18446744073709551616 is outside the 64-bit realization`.
- **`-0` is a canonical decimal**: run 13, `ok ()` where the oracle says `` `-0` is not a canonical decimal ``.
- **an arm may bind one name twice** (`(hasRepeat binders)` -> `false`): run 23, `ok ()` where the oracle says `shape Pair binds the same name twice`.
- **`field` is accepted on a type with several constructors**: run 10, `ok ()` where the oracle says `` `field` requires an ADT with exactly one constructor; ADT 0 has 3 ``.
- **a byte literal may have an odd length**: run 15, `ok ()` where the oracle says ``byte literal `abc` is not even-length lowercase hexadecimal``.
- **a shift takes a u64 count**: run 27, the program refuses `[U8, U32]` with `primitive ShiftLeft takes [Fixed { width: U8 }, Fixed { width: U64 }]` where the oracle accepts.
- **the tail of a list is typed as its element**: run 29, `primitive Length does not apply to [Nat]` where the oracle accepts.
- **DEL (U+007F) is not escaped in a spelled string**: run 30, the program spells `\u{7f}` as the raw control character, the oracle as `\\u{7f}`.
- **the argument count of an entry is not checked** (`checkArguments`): run 103, `ok ()` where the oracle says `function 0 takes 2 argument(s), received 1`.

Removed: the source was not changed (the plants are made on copies).

Authoring found four shapes certificate A and certificate B do not accept, each reduced on a scratch project before being avoided in the source (reported as issue 208): a structure or inductive holding a list of a recursive type as a root parameter, a `cons` of a recursive type's value in a definition, a mutual group or nested group whose rows the 64-declaration commands separate, and (certificate B) a match on a recursive type one of whose arms returns a bare binder of that type, which the module returns through an `Option`.

### the bootstrap Rust crate validator can fail its oracle (SPEC.md §17.23, §17.16)

`BS-05` holds the two production roots of `LexLeanBoot.RustValidate` (`bootstrap/src/RustValidate.lex.tex`) to the stage-0 oracle they replace, `calculus::rust::validate::{validate, correspond}`. The 177 distinct programs among the 260 committed package manifests and 122 fixtures lower to 236 distinct crates over both profiles; with 132 `validateCrate` and 12 `correspondCrate` rule runs (each of the 48 reachable refusals three times, found by a seeded mutation search at a fixed seed, which asserts that it reached the target and does not stop short of it), 17 hand-built crates of the merge of moved sets, 34 hand-built crates of the loop rules, and 37 hand-built crates of the machine callee and the overflow value (#91 stage 4, `SP-18`, `RB-15`), that is 456 `validateCrate` vectors (195 refused) and 319 `correspondCrate` vectors (82 refused), 775 in all. On every host both roots' rendered `rust-std` packages, compiled by the pinned rustc, agree with the oracle on all 775. On the normative host the evaluator agrees on the 201 crates of at most 400 expressions, the rule runs, and the hand-built crates (421 + 284 vectors), and on 500 seeded mutants (271 and 244 refused), and the rendered packages agree on 2,000 further seeded mutants per root. The case passed on the branch merged with `main` after wave 6 (#220), with the production export under the pinned Lean 4.32.1. Every one of these counts is an assertion of the case (`COMMITTED_*`, `RULE_*`, `VECTORS_*`, `EVALUATOR_*`, `SEEDED_*`), so a tree that no longer gives them fails the case instead of leaving this paragraph stale. The figures of this paragraph were measured on the head with `bootstrap/lexlean.lock` relocked locally, because every committed lock is stale under the head's compiler (the compiler-semantics ID has moved); the lock is not committed on a branch, and the regeneration at landing commits the relock.

Measured on the merge of `release/v0.4` (b8ff7bef6) into the stage 4 stack, #248's head `63bd949a3` (this host, `bootstrap/lexlean.lock` and `compiler/lexlean.lock` relocked locally with a freshly built `lexlean` and restored): the counts the case asserts held without a second edit (226 distinct crates from 167 programs among 250 committed manifests; 446 `validateCrate` vectors, 195 refused; 309 `correspondCrate` vectors, 82 refused), every plant was detected, the production export of the module certified under certificate A with `close_forward_references` removed (release/v0.4's union pin, #221, replaces it), and the case failed only at the comparison with the committed packages (`packages/rust-std/audit_tokens/package.json differs from the committed package`), the record bound to the compiler-semantics ID. 1,995 s for the case, 33 min 16 s wall.

The loop rules (the oracle's `Expr::Loop`, SP-19 and RB-14) are `ELoop [Id, Expr]` of the module: ownership of the state across a body that runs again (two refusals, for a name and for a component, naming the least offender in the oracle's order of names), the fallible-tail rule, `apply_types` with `tail = false` for the body, and the correspondence traversal treating the body as a block. The correspondence table was re-diffed against the oracle's `CORRESPONDENCE` after SP-19: 97 rows, and the six that gained `expr:call` (`destructure:pair`, `type:result`, `type:pair`, `construct:ok`, `construct:err`, `construct:pair`) are the only difference from the table first transcribed; the module's table and the oracle's are equal row for row and element for element.

The refusal that a construct has no row of the correspondence table cannot be reached by any crate of the closed AST (every construct it emits has a row), so no vector reaches it and the module's transcription of it is held by reading, not by a run. A crate whose exported name or origin element holds a non-ASCII or non-printable scalar is outside the claim.

**Certificate A bounds the shape of the module.** The certificate states the pairs of arms of every match (SPEC.md §17.17), so it grows with the square of the constructors of the sort matched; the first draft, with `Expr` of 27 constructors, `Ty` of 19, `Pat` of 12, and `Rt` of 50, had a certificate part of 5,040,689 bytes against `max_file_bytes` of 4,194,304. The module therefore gathers the variants a function does not tell apart into small kinds (`Expr` 17 constructors with `ELoop`, `Ty` 7, `Pat` 7: reads, wrappers, and probes carry a kind; the leaf, unary, binary, and named types carry theirs), keeps the 50 runtime functions of `runtime::Item` as a table of rows built in chunks (`family`, stem, form, failure, heap) instead of a match, and builds the 97-row correspondence table and the item table in chunks of `let`s, because a canonical program may nest 128 levels (`Program::to_file_bytes`). The 17th constructor of `Expr` still fits (`lexlean verify` of the project, then the export). One limit was met on the way and is a finding about the compiler, not about the module: with the six extra `expr:call` elements the table, built inside `correspondCrate` as before, made the certificate row `__fits_76._unsafe_rec` of that root depend on `propext` while its policy pins none (`LLV7013`, from `lexlean verify` itself); the same module with the six elements removed verified, and so did the module with the table as a definition of its own (`correspondTable`, taking `()`), while moving the item rows out as well did not. The module therefore builds the table in its own definition; why the row moves with the size of the root is for the certificate generator to say, and the finding is filed as #222 with the reproducer and the exact message. Other limits met earlier, each worked around in the module and none in the compiler: a recursive call as the argument of another is not structural for certificate A (each is bound by a `let`); a mutual group is entered through its first-declared member and must be strongly connected (`endsArgs` is therefore a function of its own beside the group `LoopEnds`); a `match` that returns a field of a recursive sort beside constant arms of it is rejected by certificate B (`LLV7015`, the pair-projection helper is inlined); each definition declares its exact axioms (`propext` where `bytesLess` is reached).

Each of these sixteen plants is made on a copy of `bootstrap/src/RustValidate.lex.tex` on every run (the first disagreement, from the case). Run numbers index the vectors of the root as `detection` holds them, which puts the rule runs first: for `validateCrate` runs 0..131 are the generator's, 132..148 the merge crates by hand, 149..182 the loop crates by hand, then the seeded mutants; for `correspondCrate` runs 0..11 are the generator's, 12..45 the loop crates by hand, then the seeded mutants. The oracle's message is the one of the vector named.

- **a heap type admitted under `rust-core`** (`(let blocked : Bool := (if (project cx core) then heapy else false);` -> `... then false else false);`), `validateCrate`, run 0: the oracle `hidden allocation: rust-core renders the heap type List(Nat), which rust-core does not provide`.
- **a name bound twice accepted** (`(match (idHas (project st bound) who)` -> `(match (idHas (nil Id) who)`), run 7: the oracle `hygiene: `a3` is bound twice in one function`.
- **the moves of earlier match arms forgotten** (`moved := (unionIds (project acc moved) (project bArm3 moved))` -> `moved := (project bArm3 moved)`), run 74, a generator run: the oracle `ownership: `v0` is moved in the body of a loop that runs again`; the merge crates by hand (runs 132..148) are the vectors that exist for this plant when the seeded corpus does not reach it.
- **a component moved after its whole accepted** (the test `(idHas (project st moved) who)` of `consumeParts` -> `false`), run 80: the oracle `ownership: a component of `v3` is moved after it was moved`.
- **an undeclared ADT type accepted** (`(hasVariants (project (project cx decl) adts) n)` -> `true`), run 47: the oracle `type Adt0 is not declared`.
- **propagation in an infallible function accepted** (`(if (project s2 propagates) then (not fallible) else false)` -> `false`), run 3: the oracle `arithmetic: `f0` propagates a failure its result type does not carry`.
- **a fallible call without propagation accepted** (`(let excused : Bool := (if tail then (project cx fallible) else false);` -> `(let excused : Bool := true;`), run 27: the oracle `arithmetic: the fallible call of `nat_mul` does not propagate its overflow`.
- **a dispatch arm's failure not compared with its function's** (`(let differs : Bool := (if isFunction then (if functionFallible then false else true) else functionFallible);` -> `(let differs : Bool := false;`), run 39: the oracle `arithmetic: Fn0::apply misstates the failure of f1`.
- **a correspondence row narrowed** (the `read` row's `expr:var` -> `expr:match`), `correspondCrate`, run 0: the oracle `the construct `call:runtime:nat_le` realizes `prim:nat_le`, which the program does not use`.
- **the width of a runtime call not compared** (`(LexLeanStd.Text::startsWith construct (text "call:runtime:"))` -> `false`), `correspondCrate`, run 10: the oracle `the construct `call:runtime:nat_mul` works at width None, but `prim:nat_mul` is at Some(U8)`.
- **a name moved in a loop body accepted** (`(let offends : Bool := (if same then false else (if kept then false else seen));` -> `(let offends : Bool := false;`), `validateCrate`, run 61: the oracle `ownership: `v0` is moved in the body of a loop that runs again`.
- **a component moved in a loop body accepted** (`(let offends : Bool := (if seen then (if same then false else true) else false);` -> `(let offends : Bool := false;`), run 90: the oracle `ownership: a component of `v3` is moved in the body of a loop that runs again`.
- **the end of a fallible loop not checked** (`| Bool.true => (endsBlock (project cx items) body)` -> `| Bool.true => (ctor Result.ok [Unit, Bytes] ())`), run 88: the oracle `arithmetic: the value a fallible loop ends with, lit:nat, is not a fallible value`.
- **a loop's state not moved into the loop** (`(match (consume st state)` -> `(match (ctor Result.ok [St, Bytes] st)`), run 18: the oracle `ownership: `m3` is moved twice`.
- **the name a loop's refusal reports not the least** (`(match (idLess who seen)` -> `(match true`), run 164, a hand-built loop crate that moves several outer names: the oracle `ownership: `v1` is moved in the body of a loop that runs again`.
- **a loop body checked as a tail** (`(applyBlock types decl body false)` -> `(applyBlock types decl body tail)`), run 70: the oracle `arithmetic: an application of Fn0 drops a failure its apply can have`.

The machine callee and the overflow value (#91 stage 4, PR 1b, on `RB-15`): the module has a node for each (`Callee.CalMachine` over a family index, `Lit.LitOverflow`), a table of the 26 families of `RustSyntax.Machine` (name and arity, in the module, as the 50 runtime items are), and the three rules of `validate.rs` that follow from them: the number of operands of a machine call against its primitive's (`machine: `p` receives n operands of a`), the overflow value outside the payload of an `Err` (`overflow: …`), and the construct `call:machine`, which no row of the correspondence has, so `correspondCrate` refuses a crate that holds one as a construct with no row (the refusal that was unreachable from the closed AST before). The encoder encodes both nodes instead of refusing them. `machine_crates()` is the corpus: a call of each of the 26 families with one operand too many (the whole table: a wrong name or arity in it is a different message), a call of three families with the right count (accepted by `validateCrate`), a propagating machine call, the overflow value alone (refused by `validateCrate`, accepted by `correspondCrate`, which has no instance for it), and `Err(Overflow)` (accepted by both): 23 crates, 19 refused by `validateCrate` and 22 by `correspondCrate`. Their three refusals (`machine-arity`, `overflow-stray`, `no-row`) are listed beside the 48 reachable ones as `HAND_BUILT`, because no mutant of a committed crate holds a machine call. A first export of the extended module failed at certificate A, `the certificate row … R6.__fits_78._unsafe_rec depends on ["propext"], its policy pins []`: the scrutinee of the new `argsC` arm was an `if` and its recursive call sat under it; binding the test with `let`, as the module's other recursive arms do, made the export succeed (929 s). Four plants, each detected: the arity not compared (`machine arity not checked`), the stray overflow value accepted, the overflow value in an `Err` refused, and a machine call not an instance of correspondence (the last is detected by `correspondCrate`, the others by `validateCrate`). The case on this head passes through the 20 plants and the certified export of the module and fails at its last leg, the comparison of the exported packages with `tests/production-export/bootstrap/packages` (`audit_tokens/package.json` differs: the committed records are bound to the compiler-semantics ID, and the module's own packages changed), which the maintainer's regeneration wave rewrites. This tree's `bootstrap/lexlean.lock` was relocked locally to run it and is not committed.

Certificate A closes the pinned axioms over the rows it names (found on the nine natural and integer families): a row pinned the axioms of the rows pushed before it, so `__fits_78._unsafe_rec`, which calls `__fits_111._unsafe_rec` of its mutual group, pinned `[]` where Lean prints `propext`, and which of the two came first was an accident of the group's order that the shipped `Machine` change moved. `close_forward_references` takes the pins to a fixpoint over the references among the rows at `finish`. Without it the export of `bootstrap` on this head fails with `LLV7013` at that row; with it the export passes (1,223 s).

The case's first failure, before the module existed (R3), was `no bootstrap case is wired for BS-05`; its first failure on the loop work was the conformance crate not compiling against `Expr::Loop`. Removed: the source was not changed (the plants are made on copies). The case no longer has switches that skip a leg (`BS05_QUICK`, `BS05_PLANTS`, `BS05_STOP`): every leg runs on every run.

### source packages can fail

The cases of the `packages` suite (`PK-01`..`PK-04`, SPEC.md §10.5) were written
before the implementation (R3). `conformance_pk_01` first failed, with the
configuration, the workspace, and the pins in place and the pin comparison not
yet in `lock --check`, at its first refusal:

```text
expected LLC0105, found ["LLC0102"] (LLC0102: lexlean.lock is stale or noncanonical; run `lexlean lock`)
```

Each defect below was planted, the named case run with
`cargo test -p repo-conformance --test conformance <case> -- --exact --test-threads=1`,
and the defect removed. The observed lines are the cases' own.

| Planted defect | Case | Observed |
| --- | --- | --- |
| the tree digest covers the manifest alone (a source file of the package is not pinned) | `conformance_pk_01` | `the project is refused`: a changed byte of the package is not refused |
| `check_pins` is not called when linking (a lock read before the package changed is trusted) | `conformance_pk_01` | `linking reads the package again` |
| a project the workspace file does not list is accepted | `conformance_pk_01` | `expected LLC0108, found ["LLC0102"]` |
| a symlink in the path of a package is not refused as such | `conformance_pk_01` | `expected LLS8001, found ["LLC0108"]` |
| two sources may claim one module prefix | `conformance_pk_01` | `two sources claim PkgLib` |
| an import may name a module of any package of the closure | `conformance_pk_02` | `check fails` (the undeclared import is accepted) |
| the production roots of a package are analysed | `conformance_pk_04` | `` `PkgBase.Core`: only the importing project's own roots are analysed `` (left: true) |
| `fmt` visits the modules of a package | `conformance_pk_03` | `` `base:src`: No such file or directory `` (it reads the package as if it were the project) |
| the build manifest names no origin | `conformance_pk_03` | `the manifest names the origin of a package module` |
| the attestation names no origin | `conformance_pk_03` | `the modules of a package carry the package as their origin` (left: `Null`) |

One plant was not caught where it was first made: removing `check_pins` from
`load_locked` alone left `conformance_pk_03` green, because `read_current_lock`
makes the same comparison first. The comparison at link is the one that reads
the package after the lock was read, so `conformance_pk_01` now links with a
lock read before a byte of the package changed and requires `LLC0105`; the
plant above is the one it caught.

Measured on Linux x86-64 with Lean 4.32.1: `conformance_pk_01` and
`conformance_pk_02` run in under 3 s, `conformance_pk_03` (the importing
project formatted, built, and verified, with every module replayed and audited)
in 185 s, and `conformance_pk_04` (a root whose closure reaches a definition of
`lib` and of `base` certified and exported) in 188 s.

Languages 1.0 and 1.1 are unchanged. After the schema change, in a scratch
checkout whose language-1.2 locks were regenerated, `cargo xtask check-golden
--write` left every file of `nat-add-zero`, `peano-arithmetic`,
`propositional-logic`, `list-induction`, `semantic-1.1`, and `uor-atlas`
untouched. The language-1.2 example `language-1.2`, which declares no source
package, differs from its committed oracles in identities and the digests
derived from them alone (the compiler-semantics, source, semantic, and build
IDs, and the digests of its lock and its source map); the committed
language-1.2 records are regenerated in the wave the maintainer names, not by
this change.

### the standard-library source packages and the helpdesk can fail (`SL-03`, `SL-04`, `EX-09`, #32)

The three cases were written before the packages and the example existed (R3).
With the sources moved out of the tree, `conformance_sl_03`, `conformance_sl_04`,
and `conformance_ex_09` each failed at their first read:

```text
SL-03: std-models/lexlean.toml: No such file or directory (os error 2)
SL-04: std-reasoning/lexlean.toml: No such file or directory (os error 2)
EX-09: examples/helpdesk/lexlean.toml: No such file or directory (os error 2)
```

Each defect below was planted in the committed source, the named case run with
`cargo test -p repo-conformance --test conformance <case> -- --exact --test-threads=1`,
and the defect removed. The observed lines are the cases' own, cut for width.

| Planted defect | Case | Observed |
| --- | --- | --- |
| `Scores` declares a `composite{}` | `conformance_sl_03` | `no composite takes an importer's models` |
| a file `std-models/src/Hand.lean` | `conformance_sl_03` | `std-models/src/Hand.lean is Lean source` |
| the manifest declares an `[[artifact_source]]` | `conformance_sl_03` | `the package declares no source package and no artifact` |
| the law `margin_tie` is renamed away | `conformance_sl_03` | `the module states `margin_tie`` |
| `leaderStep` keeps the last maximal score | `conformance_sl_03` | `LLV7002`: `Tactic `decide` proved that the proposition best [3, 9, 9, 4] = 1 is false` |
| `drain_bound` states `Remaining x < second x`, a bound the ranking does not meet | `conformance_sl_04` | `LLT4010`: `reasoner `Drain` fuel bound: `drain_bound` does not state exactly the generated obligation` |
| the manifest of `std-reasoning` declares a source package | `conformance_sl_04` | `the package declares no source package and no artifact` |
| the abstention threshold is 1, not 2 | `conformance_ex_09` | `classify disagrees with the oracle: on "invoice down": 0 != 8` |
| a file `examples/helpdesk/src/Glue.rs` | `conformance_ex_09` | `Glue.rs is handwritten Lean or Rust` |
| the keyword `refund` is misspelled in `Router.features` | `conformance_ex_09` | `classify disagrees with the oracle: on "help refund is": 8 != 1` |
| `Reply.workFor` gives an outage four items of work, not three | `conformance_ex_09` | `dispatch disagrees with the oracle: on "my site is down": outage:3 != outage:2` |
| `plan` loses its `production:` clause | `conformance_ex_09` | `the packages' own roots are not the example's` (`left == right`: two roots) |
| `classify` applies the model with no checks | `conformance_ex_09` | `LLT4008`: `executable `classify` applies model `Router::ScoringModel` without the runtime checks [precondition]` |

The threshold plant was not caught where it was first made. With the topics
weighted alike a request mentioning one topic scored a margin of 3 and every
other request 0, so a threshold of 1, 2, or 3 gave the same answers and the
differential passed; only the comparison with the committed export caught it
(`packages/rust-std/classify/package.json differs from the committed package`).
The weights are now 4, 3, and 2, so the margins 1, 2, 3, and 4 all occur and
the differential itself catches the plant, as the row above records. The
plants of `std-models` and `std-reasoning` that need only the manifest or the
source text fail before any Lean runs; the `leaderStep` plant fails in the
kernel, and the `drain_bound` plant at link.

Three more refusals are the case's own planted copies, each required to be
refused with exactly its code before any backend output: a changed byte of
`std-models` and a changed version of it in the example's manifest (`LLC0105`),
an import of `LexLeanStdModels.Scores` by an example that does not declare the
package while `std-reasoning` does (`LLR3009`), a flipped bit of the weights
artifact (`LLR3007`), and a `checked` application without its precondition
(`LLT4008`). The same pin check refused the example when the packages were
edited under a lock that predated them, before any case was run:

```text
error[LLC0105]: source package `std-models` is not the package the lock pins: tree_sha256: locked dddefafb…, found 50c65a4e…
```

The proof forms bound what the combinators claim. `linear_arithmetic` could
not prove that `decideOr` abstains below the threshold for every list (Lean
reported `omega could not prove the goal` with the counterexample
`c ≥ 2, a - b ≥ 1`, where `a` is the threshold and `b` the margin), and a theorem about `classify` over a byte literal is not reducible by the
kernel (`Tactic `decide` failed` and `Tactic `rfl` failed` on `classify`
applied to `"hello"`), so the laws of the score combinators are stated at
closed instances and the behavior of the roots is held by the differential
against an oracle written in the case, the way `SL-01` holds `Text`. The
gate law of `GateModel` is general over the type parameter.

Measured on Linux x86-64 with Lean 4.32.1: `conformance_sl_03` and
`conformance_sl_04` together in 92 s (each verifies its package, replays it
with `leanchecker`, and audits its axioms); `conformance_ex_09` in 821 s (the
example formatted, built, and verified with its three packages, the evaluator
differential, and the production export compared with the committed packages).
No language data, schema, or fixture moved for these rows: after the merge of
the wave-6 identity `cargo xtask check-golden --write` changed only the files
of `std-models`, `std-reasoning`, and `examples/helpdesk`, and
`cargo xtask check-production-export --write` only
`tests/production-export/helpdesk/`.

### byte-to-fixed-width widening conversions in language 1.2 (`SM-32`, #114)

Language 1.2 adds the total widening primitives `to_int` (any fixed-width
integer to `Int`) and `to_nat` (any unsigned fixed-width integer to `Nat`),
elaborated in linking to bit tests (`bit_and`, `equal`) and balanced sums over
`Int` or `Nat` under a generated `__widen<n>` binding so that Lean
verification, production eligibility, production lowering, and certificates A,
B, and E operate on the ordinary elaborated terms without decimal formatting or
parsing (`Coverage.Prims.widenings` and `Coverage.Prims.pixels`).

- **sign bit of a signed width added instead of subtracted.** `bit_contribution`
  in `ir::semantic::widen` weights the top bit of a signed integer by $+2^{w-1}$
  instead of $-2^{w-1}$. `-- conformance_sm_32`:

  ```text
  panicked at crates/conformance/src/support.rs:2151:10:
  the module verifies with real Lean: LexLeanError { class: Language, diagnostics: [Diagnostic { code: DiagnosticCode("LLV7002"), message: "lean failed in phase `LanguageTwelve.Main` (exitSome(1)): ..." }] }
  ```

- **signed operand accepted by `to_nat`.** `infer_primitive` admits `ToNat` on
  any `is_fixed_integer()` operand instead of `is_unsigned_fixed_integer()`.
  `-- conformance_sm_32`:

  ```text
  panicked at crates/conformance/src/support.rs:607:14:
  check fails with LLT4001: ()
  ```

Removed: both edits were restored; `conformance_sm_32` and the unit tests of
`ir::semantic::widen` pass.

## The plan of a CI run can fail (RP-14, #131)

`.github/ci-plan.sh` decides what a run of `ci.yml` is: the `rest` leg for a
draft pull request, the whole gate for a ready pull request, a request, and a
push, and nothing for a push that merges a head whose tree it equals when the
whole gate passed on that head. A decision that skips jobs is a gate that can
pass without evidence, so `RP-14` runs the script in a scratch repository with a
stand-in for `gh` against planted events (a draft, a ready pull request, a
request, a push, a merge that lands its head's tree with a run that passed
every job, and each way that evidence can fall short) and requires the whole
gate everywhere the evidence is not complete. Each defect below was planted,
`cargo test -p repo-conformance --test conformance conformance_rp_14 -- --exact`
run, and the defect removed. The observed lines are the case's own.

| Planted defect | Observed |
| --- | --- |
| the tree of the merge is not compared with the tree of its second parent | `ci-plan.sh` returns no job for a merge that changed the tree (`repository.rs:381`, left: nothing, right: the whole gate) |
| a job that did not conclude `success` is accepted | `§9.4: a skipped job is not the whole gate passing` |
| the join of Atlas is not required of the run | `§9.4: no Atlas join is not the whole gate passing` |
| the number of hosts is not compared with the host matrix | `§9.4: no host is not the whole gate passing` |
| a draft pull request runs the whole gate | `repository.rs:322`, the draft is not the `rest` leg alone |
| a failed query of the runs is read as a pass | `repository.rs:336`, a failed call is not the whole gate |
| the `rest` row of `.github/vv-rows.json` is renamed | `§9.4: the leg a draft pull request runs is the one named `rest`` |
| the host matrix does not wait for the plan | `repository.rs:1792`, `§8.3: the hosts are checked when the plan says the run is the whole gate` |
| an Atlas job decides by the event itself | ``§9.4: `atlas-prefix` runs only when the plan says the run is the whole gate`` |
| `.github/ci-plan.sh` is not executable (the mode it first had in git: the plan failed on a runner with `Permission denied` and skipped every job) | `§9.4: .github/ci-plan.sh is executable` |
| `release/**` removed from the push trigger of `ci.yml` (a release line's pushes would run no gate) | `repository.rs:1839`, `§9.4: ci.yml runs on a push to `main` and to a release line` |

The script's decisions are run on Linux, where the gate runs; on another host of the matrix the case reads the files and skips them. (The first full run of this change failed on Windows for that reason: the stand-in for `gh` was put on a `PATH` written with `:`, so the script found no run and, failing closed, chose the whole gate, which the case refused.)

Against the previous `ci.yml` the case fails at the missing plan, as R3 requires
(the case was changed before the workflow). The two defects that a stand-in `gh`
cannot show, a run of another repository's pull request and a run that did not
pass, are pinned by the script's own query (`event=pull_request&status=success`,
`head_repository.full_name`), which the case reads.

## The size-limit fixture can fail when the certificates stop costing what it assumes (SP-02, integration of #157 and #163)

`tests/negative/lowering-size-limit` asserts that a root whose certificates
would exceed `max_file_bytes` is refused with `LLS8002` in lowering, before the
toolchain starts. It was the generic chain `g_k<T> = g_{k+1}<(T, T)>` 16 deep,
whose type has 2^16 nodes. With certificates A and B as kernel data a repeated
type is one node (`COSTS_A.type_node` is 0), and the fixtures writer on the
integrated tree failed on it:

```text
gate failed: tests/negative/lowering-size-limit: step 1 `verify ` exited 0, case.toml expects 4
stdout: {"artifacts":[".lexlean/verified/d01384bec176b7fde4ad0e805d8f49b8056ffbd7c82ad66d9e6e7c792047f74b"],"attestation_id":"d01384bec176b7fde4ad0e805d8f49b8056ffbd7c82ad66d9e6e7c792047f74b","build_id":"c4921fdd0c20f1915f1097c2c3834edf8e454917df05f1ffad701d5006267205","command":"verify","diagnostics":[],"exit_code":0,…,"success":true}
```

The chain verifies, which is correct: its certificates are small. What a
small source still makes large is the pairs of arms of a match, so the fixture
is a match on 400 constructors (20 KB of source). Its observed output, in two
seconds and without starting Lean:

```text
error[LLS8002]: root `Production.Main.wide`: max_file_bytes exceeded in phase lowering: configured 4194304, its certificate A is at least 10200437 bytes (1205 program nodes)
```

This is a finding about the fixture's premise, not a planted defect: the
planted-defect evidence of the early refusal's bound is `conformance_sp_02`'s,
which passed on the train's CI run and records both the early bound and the
post-generation notation check (`SPEC.md` §17.17, #187):

- **a root whose certificate passes lowering's lower bound and certificate B's
  12-byte-per-node derivation floor (`NODE_BYTES` in `data_b/align.rs`) is
  refused by the finished-certificate notation check (`record_source` against
  `max_file_bytes`).** `tests/negative/certificate-generation-limit` sums 500
  fields of a record: its 500 field reads take 250,000 pattern entries (`2,000,000`
  bytes at lowering's 8-byte-per-slot floor, under the default `4,194,304`), and
  its derivation's kernel nodes at `12` bytes each also stay under `4,194,304`
  while record-read nodes average about `35` bytes in the recorded notation.
  When `data_b::render` weighs the finished notation (`8,513,069` bytes, `2.03`
  times `max_file_bytes`), it refuses with `LLS8002` before any file is staged
  or Lean is started:

  ```text
  error[LLS8002]: root `Production.Main.sum500` (target `rust-core`): max_file_bytes exceeded in phase certificate-b: the certificate is 8513069 bytes, beyond the 4194304 bytes of max_file_bytes
  ```

  In `conformance_sp_02`, `record copy 100` regenerated under a quarter of its
  recorded size is refused *during* derivation (`the derivation of a function is
  at least … bytes, beyond the … bytes of max_file_bytes`), and certificates A
  and E of every stress family regenerated under half their recorded size are
  refused with `LLS8002` by the finished-certificate weight check.

### undeclared environment variables in the compiler crate can fail (`CL-17`, #180 / #187)

`conformance_cl_17` scans every `.rs` source under `crates/lexlean/src` (with
comments and `#[cfg(test)]` items blanked) for `env::var`, `env::var_os`,
`env::vars`, and `env::vars_os` and refuses any read outside §23.1's four
operational variables (`WIDTH_VARIABLE`, `TELEMETRY_VARIABLE`, `HINTS_VARIABLE`,
and `MEMORY_VARIABLE` in `verify/operational.rs`), `cli.rs`'s `"NO_COLOR"`
presentation check, and the toolchain/process locators `"HOME"`, `"PATH"`, and
`"ELAN_HOME"` in `verify/child.rs` and `verify/toolchain.rs`. On every run it
plants `let _ = std::env::var("LEXLEAN_AXDBG");` on a copy of
`production/certificate/data_a.rs` and requires the scan to report
`production/certificate/data_a.rs`.

## Census windows 4 and 10 as width-100 certificates (Issue #168)

`CensusCert4` and `CensusCert10` each proved one window of 1,000 Gram types,
`gramRange 4000 1000` and `gramRange 10000 1000`, by a single
`of_decide_eq_true`, and their memory grew with the work of the whole window:
on main run 39054719580
`CensusCert4` took 1,419.3 s to elaborate and 2,248.5 s to replay at about
15 GiB with 3.5 GiB of swap, and `CensusCert10` 114.6 s and 120.8 s at
14.3 and 14.7 GiB with swap. Each window is now ten width-100 certificates,
`CensusCert40` to `CensusCert49` and `CensusCert100` to `CensusCert109`, joined
by `gramRange_append` as `CensusCert7` already was. The statements, the kernel
`decide` proofs of each piece, and every importer are unchanged.

Measured on run 39086315289 (the Atlas jobs of this tree):

| Window | Elaboration (s) | Replay (s) | Largest piece (GiB) | Swap |
| --- | ---: | ---: | ---: | --- |
| 4 | 111.5 | 117.6 | 4.5 | none |
| 10 | 118.1 | 115.4 | 5.6 | none |

No `Census*` process swapped on that run; `Closure` and `ClosureOrbit` still
did. The join wrote 347 normalized verification records, four for each of the
twenty new modules beside the 267 before; they are this commit's
`expected/verify`, and the hints of the 22 changed modules are that run's.

### the census windows can fail

A piece whose window is wrong, or a join that does not sum to the width of its
window, is refused by the kernel when `CensusCert4` or `CensusCert10`
elaborates, because each keeps the statement it had; a piece missing from the
Atlas root is refused by `audit-atlas-exercise` (R4); a piece missing from the
shard plan is refused by `cargo xtask ci-shards` and by the plan check of
RP-14.

## Planted defects for merged IDs that had no record (Issue #33)

Each ID below had a case that passes but no record that the case can fail
(§27.9). Each defect was planted alone in the named source, the named test run
once with `cargo test -p repo-conformance --test conformance <test> --
--exact`, the failure observed, and the plant restored with `git checkout`.
The first seventeen rows were run on a Linux x86-64 host without the pinned Lean
toolchain, so each defect is one that the case's platform-independent assertions
reach before any Lean step. The last eight rows (`DF-18` to `LM-02`) were run
with `leanprover/lean4:v4.32.1` installed through `elan`. The signatures are
the first panic message, trimmed.

| ID | Planted defect | Expected failure signature |
| --- | --- | --- |
| `GL-18` | `lexicon/package.rs`: the `manifest.language != ctx.language` test made `false &&` | `expected LLC0103, found ["LLR3004"]` (`support.rs`, `expect_code`) |
| `GL-17` | `lexicon/mod.rs`: `load_bootstrap_for("1.2")` reads `bootstrap-1.1.toml` | `init 1.2: error[LLI9001]: phase language-load: bootstrap schema drift` |
| `SM-24` | `ir/semantic.rs`: a pair projection of a non-product type returns `Ok(None)` | `check fails` (the mutated project must be refused) |
| `SM-26` | `ir/semantic.rs`: `alpha_identity` keeps a parameter's own name instead of its alpha-renamed one | `assertion left == right failed` on the two alpha identities |
| `SM-27` | `ir/semantic.rs`: `alpha_identity` drops the termination evidence (`Vec::new()`) | `assertion left != right failed` on the unchanged digest after rebinding the evidence |
| `PD-01` | `production/eligibility.rs`: `STRUCTURAL_KEYS` lists `type.nat` where `type.bool` was | `structural keys are unique` (`left: 91`, `right: 92`) |
| `PD-02` | `production/eligibility.rs`: `analyse_module` no longer returns `Ok(None)` for a module without roots | `a module with formal content and no root is never analysed` |
| `PD-03` | `production/eligibility.rs`: termination evidence is no longer inserted into `erased` | `termination evidence is erased, never realized` (`left: []`, `right: ["Production.Kernel.countdown_decreases"]`) |
| `PD-06` | `production/eligibility.rs`: every `EffectRow` is built with no sources | `array shorter than minItems 1` at `/roots/1/targets/0/effects/0/sources` of `production-eligibility.schema.json` |
| `TC-01` | `calculus/mod.rs`: `canonical` binds parameters but keeps their spelling | `assertion left == right failed: adt-evaluation` (alpha variant bytes differ) |
| `TC-02` | `calculus/check.rs`: the match exhaustiveness test made `false &&` | `an invalid program loads: expected `the match is not exhaustive: Adt { constructor: 2 } is not covered`` |
| `MD-01` | `ir/semantic.rs`: the language-1.1 gate `first_language_1_2_construct` made `&& false` | `tests/negative/model-under-1.1: expected "`contract declaration` is a language-1.2 construct"` |
| `MD-10` | `artifact/content_id.rs`: `semantic_id_for` stops framing the linked IR | `every change changes the semantic ID` (`left: 1`, `right: 6`) |
| `LM-01` | `ir/semantic/language_model.rs`: the generated law `in_range` renamed `in_bounds` | `Digits generates its three laws` |
| `LM-03` | `ir/semantic/model.rs`: both `utf8_lines` carriage-return refusals made `false &&` | `check fails` (an artifact with a CR must be refused) |
| `LM-13` | `text_laws.rs`: `LAWS_MODULE` names `LexLeanText.Laws` | `the shipped registry: ... LLI9001 ... the registry does not list `LexLeanText.Laws`` |
| `PK-05` | `link.rs`: a linked module's `origin` is `None` | `the standard library's Text is a module of the project, from the package` (`left: None`, `right: Some("std")`) |
| `DF-18` | `ir/semantic.rs`: the three-argument test of `Iterate`/`IterateUntil` made `false &&` | `expected "primitive IterateUntil expects 3 argument(s) (step, natural-number bound, initial state), received 2", got LLT4001: ... requires a typed initial state` |
| `PF-19` | `ir/semantic.rs`: the `linear_arithmetic` prior-definition test made `false &&` | `check fails` (an unfolding of a non-prior definition must be refused) |
| `TC-05` | `calculus/library.rs`: the `MapLookup` template builds the `map_contains` function | `fixture map-lookup: function 0: the body has type Bool, expected Option { value: String }` |
| `NE-01` | `production/lcnf.rs`: `Renamer::bind` keeps Lean's name instead of the fresh one | `the production compiler input violates schemas/compiler-input.schema.json: ... `_uniq.19` does not match pattern `^v[0-9]+$`` |
| `MD-03` | `ir/semantic/model.rs`: the validator order test uses `>` for `>=` | `expected LLT4006, found ["LLT4001"] (LLT4001: phase link: duplicate generated name `DigitContract.postcondition_sound`)` |
| `MD-05` | `ir/semantic/model.rs`: a model declaration's interface mismatch is filtered out | `expected LLT4006, found ["LLT4009"] (... evidence `ReportEvidence` is about another contract or realization)` |
| `MD-11` | `calculus/realization.rs`: the `term.checked_apply` table key renamed | `the realization table covers term.checked_apply` |
| `LM-02` | `ir/semantic/language_model.rs`: the `in_range` law states `false` for `true` | `the module verifies with real Lean: ... LLV7002 ... Lean rejected `Models.Recognizer` (error): Type mismatch LexLeanText.in_range pieces __bytes` |

Two first attempts were not caught and are not evidence: making
`require_language_1_2` always succeed leaves `MD-01` green, because the
declaration-level gate above is the one it exercises; and serializing the
evidence `claims` away fails `MD-10` on the closed schema, not on the identity,
so the identity plant above is the one recorded.

## Negative fixtures for older diagnostic codes (Issue #33)

Fourteen of the 26 codes registered before v0.3 that had no fixture now have one
under `tests/negative/`. Each is a refusal the CLI emits on that input, its
`expected/` written by the fixture writer for that directory alone, and the whole
suite (`conformance_ex_07`) passes with the pinned toolchain installed. Each was
planted against: the refusal's message reworded at the emitting source with a
leading `~`, the fixture's comparison observed to fail on `expected/command.json`,
and the edit restored.

| Fixture | Code | Reworded in |
| --- | --- | --- |
| `source-invalid-utf8` | `LLL1001` | `source/normalize.rs` |
| `numeral-leading-zero` | `LLL1003` | `source/scan.rs` |
| `math-island-nested` | `LLL1006` | `grammar/chart.rs` |
| `math-dollar-delimiters` | `LLP2001` | `grammar/chart.rs` |
| `math-nonassociative-chain` | `LLP2004` | `grammar/math.rs` |
| `proof-step-after-closure` | `LLF5002` | `elaborate/proofs.rs` |
| `proof-leaves-goal-open` | `LLF5004` | `elaborate/proofs.rs` |
| `check-duplicate-selection` | `LLC0002` | `project.rs` |
| `git-lexicon-offline` | `LLS8003` | `lock.rs` |
| `lexicon-unsafe-surface` | `LLR3006` | `lexicon/entry.rs` |
| `lexicon-reserved-alias` | `LLR3002` | `lexicon/resolve.rs` |
| `lexicon-raw-unicode-render` | `LLB6002` | `backend/latex.rs` |
| `definition-binder-mismatch` | `LLT4004` | `elaborate/definitions.rs` |
| `lake-dependency-absent` | `LLV7007` | `verify/workspace.rs` |

`lake-dependency-absent` runs `verify`, so it needs the pinned toolchain, as
`vr-11-exact-mismatch` does.

The other eleven have no fixture (`LLC0001` gained one, `check-unknown-flag`,
once the schema stopped requiring `primary` of the positionless codes; see the
section on the diagnostic schema below):

- `LLC0104`: a case-folded collision needs two paths differing only by case, which
  a repository cannot hold on a case-insensitive checkout, and a duplicate
  logical module is refused earlier as `LLP2003`.
- `LLT4002`: no input was found that reaches it; the one emission site needs
  several overloaded candidates none of which fits.
- `LLL1005`, `LLB6001`, `LLB6003`, `LLI9001`: internal invariants and publication
  failures no input of a fixture reaches.
- `LLB6004`, `LLV7006`: need an external PDF provider, or a Lean warning from
  generated code.
- `LLV7008`, `LLV7009`, `LLV7010`: depend on the host (a non-UTF-8 path, git and
  the network, a filesystem failure).

### the refusal of the data form can fail (`AU-13`, #149)

`AU-13` states that a language-1.2 module written with `\semanticdata`,
`\toolingdata` or `\coredata` is refused with `LLP2003` at its data argument,
naming the control and the authoring notation, that `fmt` leaves the bytes,
that the SDK's two conversions are inverses, and that language 1.1 and the
Atlas keep the data form. The case was written before the refusal; it was not
run red on its own before the implementation, and the first plant below
restores exactly the condition it would have run in. Four defects were planted
in the refusal, one at a time, each undone before the next; each turns
`conformance_au_13` red:

- **a language-1.2 source in the data form is accepted again** (the semantic
  branch of the refusal removed from `link.rs`). The case finds the example
  written as data links:

  ```text
  panicked at crates/conformance/src/support.rs:758:14:
  check fails
  ```

- **a tooling module written as data is accepted again.**

  ```text
  panicked at crates/conformance/src/cases/authoring.rs:745:28:
  commands: a tooling module written as data linked
  ```

- **a core module written as data is accepted again.** The same
  `check fails` from `support.rs:758:14`, reached in the core part of the case
  after the semantic and tooling parts pass.

- **the refusal at the wrong position** (the span of the first atom of the
  file instead of the data argument):

  ```text
  panicked at crates/conformance/src/cases/authoring.rs:676:28:
  language-1.2: the position is not after \semanticdata
  ```

The cases that compared a language-1.2 build of the data form with the build of
the notation (`AU-02`, `AU-05`, `AU-10`) can no longer do so, because that form
is not a source; their statements and scenarios were restated to what they
still check (builds with source maps that name the notation, `fmt` the identity
on it), and the equivalence of the two forms is carried by the SDK's
conversions (`AU-08`, `AU-13`). The plants of other cases that edited the data
form of a language-1.2 module (`DF-15`, `SP-02`, `SP-12`, `GC-11`, `RT-01`,
`TL-01`, `CF-18`, and `SM-34` from the identity wave that joined the two) now
hand the compiler the notation of the edit, or the decoder the data directly
where the notation cannot say it (a spec tag of another schema, the canonical
bytes of a tooling module, an empty arity, a derived core row written with a
type, a value, or universe parameters: the notation states a row's universe
parameters with its type, so the decoder names a derived row's type before its
parameters); the fixture `tests/negative/semantic-module-v1-under-1.2` is a fixture of this
refusal and expects `LLP2003`.

Not run for this record: the records the change regenerates (the maintainer's
wave) and every Lean-backed case, which needs the pinned toolchain.

## Classification matches are total (RP-15, Issue #33)

`cargo xtask validate-model` runs `audit-classifications` (R8). It reads every
Rust source of `crates/lexlean/src` and finds each `match` whose arms name five
or more variants of one closed enum: the five IR enums everywhere, and the
calculus and Rust AST enums beneath `calculus/`. Such a `match` is a
classification, and it must name every variant: a `_` arm, or a bare binding
arm (`other =>`), fails the audit, because either would give a variant added
later a meaning nobody chose and switch off the compiler's own exhaustiveness
check. Smaller matches are lookups and may default. A guarded catch-all is not
one: the compiler ignores guards, so it still refuses an unnamed variant.

It reads direct arm patterns only (`SemanticType::Nat`, or `Self::Nat` inside
an `impl` of the enum). A `match` on a tuple of enums is a pairing and is not
audited; serialization and the snapshot are `serde`-derived, so no
hand-written `match` of theirs exists. `if let` and `matches!` over these enums
are not read.

It also fails when a consumer it must read has no classification left, so a
refactor cannot move a consumer out from under it: the Lean and LaTeX backends
(`backend/semantic.rs`), validation and charge (`ir/semantic.rs`), production
eligibility, the calculus checker and realization table, and the Rust AST, its
lowering, and its validator.

At the time of writing it reads 137 sources and finds 244 classifications, all
total. There is no exemption: every source is read alike.

**What the 27 catch-alls it found became.** Each plain `_` arm lists the
variants it took by default, and each binding arm is `name @ (A | B | ..)`, so
the arm body is unchanged; rustc confirms the list is exactly the remaining
variants. The Lean backend's `primitive` became one exhaustive `spelling` table
of the 56 primitives. For the 25 in sources the compiler-semantics ID does not
read, generated output is unchanged: `cargo xtask check-golden` and `cargo xtask
check-calculus` pass, the `lexlean` unit tests pass, and `cargo clippy
--workspace --all-targets --all-features -- -D warnings` is clean.

**The renderer.** The other two, in `calculus/rust/lower.rs` and `validate.rs`,
are sources the compiler-semantics ID reads: their bytes are recorded as
`rust_renderer` in `language/semantics-1.2.toml` and checked by
`the_compiler_semantics_records_the_renderer`. Editing them changed that digest,
so the record moved (`6d1e0639...` to `c7fe1d8d...`) and the compiler-semantics
ID moves with it. No record was regenerated: the committed locks and the
expected outputs that carry the ID are stale until the maintainer's identity
wave regenerates them once, and until then a committed lock fails `LLC0102`.

**Planted.** Each was made alone in the named source, `cargo xtask
validate-model` run, and the edit restored.

```text
backend/semantic.rs, `spelling`: the `GraphTopological` arm made `_ =>`
gate failed: R8: crates/lexlean/src/backend/semantic.rs:355: a `match` naming 55 variants of `SemanticPrimitive` has a wildcard arm at line 497; ...

backend/semantic.rs, `spelling`: the `LessThan` arm made `other =>`
gate failed: R8: crates/lexlean/src/backend/semantic.rs:355: a `match` naming 55 variants of `SemanticPrimitive` has a binding catch-all arm at line 498; ...

ir/semantic.rs, `type_node_count` (charge): `_ => 0,` added after the last arm
gate failed: R8: crates/lexlean/src/ir/semantic.rs:2048: a `match` naming 28 variants of `SemanticType` has a wildcard arm at line 2079; ...

calculus/rust/ast.rs, `lit`: the `Lit::Unit` arm made `_ =>`
gate failed: R8: crates/lexlean/src/calculus/rust/ast.rs:458: a `match` naming 7 variants of `Lit` has a wildcard arm at line 459; ...
```

`conformance_rp_15` plants a wildcard, and then a binding arm, in a
classification of each of ten consumer functions (`spelling` and
`render_latex` of the backend, `infer_term` and `type_node_count` of
validation and charge, `type_key` of eligibility, `prim_type` of the calculus
checker, `in_place` of the realization table, `expr` of the Rust AST, `item` of
its lowering, and `expr` of its validator) and observes the audit name the
file. It empties and then removes each consumer and observes `would pass over`.
`cargo test -p repo-model classification` covers the scanner
itself: a total match, a wildcard, a binding, a guarded wildcard, a lookup,
`Self::` paths, prose and literals, a tuple pairing, block bodies, and raw
strings.

## The diagnostic schema requires a position except where none exists (Issue #33)

`schemas/diagnostic.schema.json` required `primary` of every diagnostic, but
the compiler issues eight codes where no source position exists, and a fixture of
one of them (a command-line usage error) could not pass `conformance_ex_07`'s
schema check. Inventing a span would assert a location the failure does not
have, so the schema now has two alternatives: a diagnostic with `primary`, or one
without it whose code is one of the eight. `SPEC.md` 20.1 names the same
codes and the situation of each.

The eight were read from the emitter, as the paths that run outside any loaded
project, since `Engine::bound` anchors every other project diagnostic at its
file: `LLC0001` (clap, `explain`, `init`, an unreadable request file), `LLC0101`
and `LLS8001` and `LLV7008` and `LLC0108` (`project::discover` and
`Project::load`, which run before an engine exists), `LLC0201` (the production
request is parsed before the engine anchors anything), and `LLV7010`, `LLS8001`
and `LLV7102` (`verify_artifact`, which has no project). A run of the CLI in
JSON mode confirmed `LLC0001`, `LLC0101` and `LLV7010` carry no `primary`, and
that a configuration that parses, a stale lock, and an unsupported language do.

`conformance_ex_07` checks, before any fixture runs, that the schema's list and
the `SPEC.md` table are the same codes, that each listed code is valid with and
without `primary`, that `LLL1001`, `LLP2001`, `LLT4001`, `LLV7002` and
`LLC0102` are valid with it and refused without it, and that every committed
`expected/diagnostics.json` validates. Planted, each alone, observed, restored:

```text
LLL1001 added to both the schema's list and the SPEC table
LLL1001 has a source, so omitting primary is refused

LLC0001 removed from both
tests/negative/check-unknown-flag/expected/diagnostics.json: ... oneOf: 0 of 2 alternatives match ... required property `primary` is missing
```

The schema is hashed into the compiler-semantics ID, so the ID moves; no record
was regenerated, and until the maintainer's wave the committed locks fail
`LLC0102`, which fails the fixtures that run a project.

## Negative fixtures for codes only conformance cases reached (Issue #33)

Six of the 18 codes added since v0.3 had no fixture under `tests/negative/`.
Each fixture below is a refusal the compiler emits on that input, written by
the fixture writer (`repo_conformance::fixtures::write`, the function behind
`cargo xtask check-fixtures --write`) for that directory alone, and compared
byte for byte by `cargo xtask check-fixtures` thereafter. Each was planted
against: the named edit was made alone in the named source, the fixture's
comparison observed to fail on `expected/command.json`, and the edit restored.

| Fixture | Code | Planted defect |
| --- | --- | --- |
| `production-request-malformed` | `LLC0201` | `api/production.rs`: `request_error` emits `LLC0202` |
| `production-root-unknown` | `LLC0202` | `api/production.rs`: the unknown-root refusal emits `LLC0201` |
| `production-target-unregistered` | `LLC0203` | `api/production.rs`: `target_error` emits `LLC0204` |
| `production-expectation-stale` | `LLC0204` | `api/production.rs`: `stale` emits `LLC0203` |
| `source-package-outside-workspace` | `LLC0108` | `source_package.rs`: the refusal reads "may not declare" for "cannot declare" |
| `tooling-module-in-user-project` | `LLT4013` | `link.rs`: the user-project refusal emits `LLT4014` |

The twelve other codes of the list have no fixture, for a reason the runner
imposes and not a gap in the case that reaches them:

- `LLC0105`, `LLC0106`, `LLC0107`, `LLR3009` need a `lexlean-workspace.toml` in
  a strict ancestor of the project root (`Workspace::of_project` starts at the
  parent), and a fixture runs a temporary copy of `project/` alone. A
  fixture can express only that no workspace exists, which is `LLC0108`.
  `--project` is read against the process directory, not the copy.
- `LLT4014`, `LLT4015` are refusals inside a tooling module, and a user project
  is refused at `LLT4013` before the module is decoded, as the fixture above
  shows; they are reached only in the tooling projects the compiler embeds.
- `LLV7018` follows `lexlean::verify::end_live_children`, a host call of the
  library API; no command line reaches it.
- `LLV7102` replays a published bundle, which only a verified export produces,
  and that needs the pinned toolchain; `LLV7101` needs certificate evidence
  built for the root's lowered program.
- `LLB6005`, `LLB6006`, `LLB6007` are target programs, GNAF requests, and
  certificates that the library and `cargo xtask check-calculus` read; no
  command takes one, and the one production path to `LLB6005` is a package the
  Rust profile cannot render, which no committed project builds.

## Planted defects for the context windows and text generators (slice 2 of #30)

Each defect below was planted alone in `ir/semantic/language_model.rs`, the named
test run once with `cargo test -p repo-conformance --test conformance <test> --
--exact`, the failure observed, and the plant restored with `git checkout`
(§27.9). The runs were made on a Linux x86-64 host with `leanprover/lean4:v4.32.1`
installed through `elan`, with `schemas/semantic-module-v2.schema.json` at its
content before the `minimum: 0` change for `capacity` and `max_tokens`, because
that change moves the compiler-semantics ID and the committed example locks
were not regenerated in the same change; the planted source is not an input of
that ID. The signatures are the first panic message, trimmed.

| ID | Planted defect | Expected failure signature |
| --- | --- | --- |
| `LM-04` | `check_context_window`: the `keep_last` arm names `ADMIT_REFUSE` where it named `ADMIT_KEEP_LAST` | `the module verifies with real Lean: ... LLV7002 ... Lean rejected `Models.Recognizer` (error): Type mismatch LexLeanText.admitKeepLast_bounded capacity __tokens has type ... admitKeepLast ... but is expected to have type ... admitRefuse ...` |
| `LLT4016` (window) | `check_context_window`: the capacity-0 test made `false &&` | `check fails` (a window of capacity 0 must be refused) |
| `LLT4016` (generator) | `check_text_generator`: the `max_tokens`-0 test made `false &&` | `check fails` (a generator of `max_tokens` 0 must be refused) |
| `LM-05` | `check_text_generator`: the stateful-scorer refusal made `false &&` | `expected "is stateful" under LLT4006, got LLT4006: ... scorer `CountingModel` needs runtime checks its evidence does not discharge` |
| `LM-05` | `check_text_generator`: the undischarged-runtime-check refusal made `false &&` | `check fails` (a scorer whose postcondition is a runtime check must be refused) |
| `LM-05` | `check_text_generator`: the `Option.none` arm of `generate` returns `unknown_token` where it returned `invalid_utf8` | `the module verifies with real Lean: ... LLV7002 ... Tactic `rfl` failed: The left-hand side Garble.generate "a" is not definitionally equal to the right-hand side Except.error Garble.Failure.invalid_utf8` |
| `LM-06` | `check_text_generator`: `sampled` made `false &&` | `assertion `left == right` failed: Draw.run takes the seed as its last input` (`left: Some(String("list"))`, `right: Some(String("uint32"))`) |

The wrong-shape scorer refusals (a contract that maps to `nat`, a contract over
`list int`) share the stateful and undischarged refusals' branch structure in
`check_text_generator` but were not planted separately; the case asserts each
by its reason (`does not map `list uint32` to `list int``).
## Text classifiers, composition, and the no-prompt audit (LM-07, LM-08, LM-09, slice 3 of #30)

`LM-07` (classifiers), `LM-08` (composition and adapters through the model
constructs alone), and `LM-09` (no prompt is code) are `build` rows, validated
against Lean's kernel and the compiler's own registries and not proved here.
Lean's guarantees are the authority rows' (`model/authorities.toml`); the
library's lemmas are checked by the pinned Lean when the generated module is
decoded, and a classifier's, a window model's, and an adapter's generated
theorems are checked by it again in each module that declares them.

### What each case establishes

- **`conformance_lm_07`** links classifiers `Words` (over the `longest_match`
  tokenizer) and `Letters` (over the `byte_level` one) over a stateless
  `list int -> nat` model, reads their elaboration (`features`,
  `classifyTokens`, `classify`, and the laws `features_width`,
  `features_total`, `features_fit`), checks the generated Lean applies the
  library's `features` and proves by its four lemmas without splicing any,
  refuses a non-tokenizer (`LLT4006`), an unknown model (`LLT4006`), an unsorted
  policy (`LLT4006`), a model from `list uint32` (`LLT4006`), a model whose
  postcondition is checked at run time (`LLT4006`), a stateful model
  (`LLT4006`), and an unknown member (`LLT4001`); verifies the laws with the
  pinned Lean; evaluates four classifications in the kernel; refuses a wrong
  claim (`LLV7002`); and fails a classifier whose policy omits an axiom its
  laws observe (`LLV7005`).
- **`conformance_lm_08`** links a stateful model over the window `Last2` whose
  step is `Last2.push` and whose contract's postcondition is
  `fitsOption Last2.capacity pushed`, so that `Last2.push_bounded` is the whole
  proof of its `satisfies_contract` evidence; composes it under `scan`; links
  adapters over `string` that apply `Words.classify` and `Halt.generate`, whose
  evidence is `Words.features_width` and `Digits.text_roundtrip`; reads the
  schema's declaration kinds and asserts they are the nineteen of the language
  (none for composition or adapters); refuses a model whose realization is a
  classifier (`LLT4006`); verifies all of it with the pinned Lean, including the
  kernel evaluation of the scan's stream; and fails an adapter whose
  realization is not the contract's label (`LLV7002`).
- **`conformance_lm_09`** reads `schemas/semantic-module-v2.schema.json` and
  `language/production-1.2.toml`: the 56 operations of the schema's primitive
  node are the 56 `primitive.*` rows of the registry, each is a variant of the
  compiler's `SemanticPrimitive`, each is classified by the audit (18 may take
  a `string` or `bytes` operand, and every one of those returns data), and none
  of the four text declarations states a member that is a free `string` (a
  string type no pattern, enumeration, or constant closes, found through
  references). The audit is armed by those 18 operations; a planted
  `eval_string` row in either registry, an unclassified operation, a missing
  classification, a text member `prompt` of type string on each of the four
  declarations, and one reached through a reference are each caught, with their
  names, inside the case.

### Where it was run

Linux x86-64, `leanprover/lean4:v4.32.1` installed through `elan`,
`LEXLEAN_VERIFY_WIDTH=2` and `LEXLEAN_VERIFY_MEMORY_KIB=12582912`. The change
moves the language-1.2 compiler-semantics ID (the text library, the schema, and
the `semantic_ir` and `latex_backend` versions), so the committed lock of
`examples/models` is stale (`LLC0102`); the cases were run with that one lock
rewritten by `lexlean lock` in the working tree, and the rewrite is not part of
this change, because the maintainer regenerates every record in the identity
wave. With it, on the tree merged with the head of #176 (`8153ee5f7`):

```
cargo test -p repo-conformance --test conformance conformance_lm_
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 419 filtered out; finished in 555.69s
```

The ten are `LM-01`..`LM-09` and `LM-13`. `LM-07` and `LM-08` narrow the
example's entrypoint from `Main` to `Recognizer` and lock again in their own
copy of the project, as the helper `narrowed` says, so that verifying one of
them does not verify the rest of the example once more.

### The cases can fail

Each defect was planted alone in the named file, the named test run once with
`cargo test -p repo-conformance --test conformance <test> -- --exact`, the
failure observed, and the plant restored with `git checkout`. The signatures
are the first panic message, trimmed.

| ID | Planted defect | Expected failure signature |
| --- | --- | --- |
| `LM-07` | `ir/semantic/language_model.rs`: the stateful-model test of `check_text_classifier` made `false &&` | `expected "is stateful" under LLT4006, got LLT4006: phase link: text classifier `Remembering` model `RecentModel` does not take `list int`` |
| `LM-07` | the same file: the open-runtime-checks test made `false &&` | `expected "needs runtime checks" under LLT4006, got LLT4006: phase link: text classifier `Words` model `RawDigitModel` does not take `list int`` |
| `LM-07` | the same file: `features_width` states a width of `0` for `size` | `the module verifies with real Lean: ... LLV7002 ... Lean rejected `Models.Recognizer` (error): Type mismatch LexLeanText.features_width Digits.size __tokens ... but is expected to have type LexLeanRuntime.length (LexLeanText.features Digits.size __tokens) = 0` |
| `LM-07` | the same file: `features_fit` proved by the `features_width` lemma | `the generated Lean proves by LexLeanText.features_fit` |
| `LM-07` | the same file: the `byte_level` total law proved by the `longest_match` lemma | `the generated Lean proves by LexLeanText.features_total_text_bytes` |
| `LM-08` | `ir/semantic/language_model.rs`: the window's generated law renamed `push_fits` | `check succeeds: ... LLT4001 ... proof application names a forward or missing theorem `Last2.push_bounded`` |
| `LM-08` | the same file: `features_width` states a width of `0` for `size` | `the module verifies with real Lean: ... LLV7002 ... Lean rejected `Models.Recognizer` (error): Type mismatch LexLeanText.features_width Digits.size __tokens` |
| `LM-09` | `language/production-1.2.toml`: a `primitive.eval_string` row | `the audit holds: primitive `eval_string` is in the production registry and not in the schema enumeration` |
| `LM-09` | `schemas/semantic-module-v2.schema.json`: a `prompt` member of type string on `text_classifier` | `the audit holds: text declaration `text_classifier` member `prompt` is a free string` |

Two attempts at planting a new declaration kind (`text_adapter`) in the schema
were not evidence for `LM-08`: the schema is an input of the compiler-semantics
ID, so the lock is stale (`LLC0102`) before the case reads the kinds. The
kinds assertion is reached only in a tree whose records are current.

### Gates run

```
cargo xtask validate-model         validate-model: documents current, 429 ids, 86 codes, meta-gate and audits clean (R1)
cargo xtask validate-spec-links    validate-spec-links: 429 table rows bijective with the register (RP-07)
cargo fmt --all -- --check         (no output)
cargo clippy --workspace --all-targets -- -D warnings   (no output)
cargo test --workspace --exclude repo-conformance --all-features
```

The last runs 204 unit tests of `lexlean` and the rest of the workspace; one
test of `crates/lexlean/tests/property.rs`,
`formatter_is_idempotent_on_every_example`, fails with `LLC0102` ("the compiler
semantics changed since locking") on the committed lock of `examples/collections`,
which is the stale record described above and is regenerated in the wave.

## A mutual group of implementation records is pinned and audited by its union (LLV7013, found by the slice 3 identity, #221)

### The defect

On the tree of #221 merged with `main` 17f7e462c, `lexlean verify` of the
bootstrap project stopped with

```
error[LLV7013]: certificate A: the certificate row
`LexLeanPreserve.C3df453b80ed6295dbadd323b69d0f1c6.R6.__fits_76._unsafe_rec`
depends on ["propext"], its policy pins []
```

and did so again on a second identity (`Caf08…`, the same tree with
`latex_backend` bumped), while `main` verifies. The staged Lean sources of the
preserve stage (the environment modules, every certificate, `Compose`, the audit
module) and the generated `LexLeanBoot` modules are byte-identical between
`main` and the failing tree once the identity hash is substituted
(`diff -rq`: no difference), so the verdict differs only through the hash in the
certificate module names.

The row is the first of a mutual group of three implementation records
(`__fits_76`, `__fits_106`, `__fits_147` of root R6, `all` naming the three),
pinned `[]`, `[propext]`, `[propext]`: the generator pinned each row from what
it referenced when it was pushed. In the certificate's data the terms of the
three do not refer to one another (`__fits_76` and `__fits_147` refer to no
other row; `__fits_106` to `__enc_2` and `__enc_8._unsafe_rec`), and the
constants `__fits_76` does refer to (`LexLeanBoot.RustValidate.Cap`,
`Cap.casesOn`, `Cap.rec`, `LexLeanTarget.TargetSyntax.Value`, `Value.adt`)
print as axiom-free on both identities, so the difference is not in what the
row's term names. Lean 4.32.1 stores axiom data for a module's constants
(`Lean/Util/CollectAxioms.lean`, `exportedAxiomsExt`) by one shared traversal,
over the module's constants in the order of a hash map keyed by names, with an
empty-set sentinel for a constant in progress; the audit printed, on `main`'s
identity, `[]`, `[propext]`, `[propext]` (the pins), and on the failing
identities `[propext]` for all three. The exact path by which the traversal
gives `__fits_76` the set of its group on those identities is not established
here; what is established is that identical sources print differently and that
the union of the group's prints is `[propext]` on every identity tried. The
second occurrence is #227's core-library case, where `__fits_78._unsafe_rec`
calls the later `__fits_111._unsafe_rec` and is pinned `[]` where Lean prints
`propext`: a forward reference inside a mutual group.

### The fix

`group_rows` in `production/certificate/data_a.rs` gives every member of a
group of `RowKind::Partial` rows the union of the members' axioms once the group
is complete; `audit` in `production/preserve.rs` checks such a group once, at
its first member: the union of the sets printed for its members must equal the
pin of each member, and the message names the group. Every other row is checked
row by row, as before. SPEC.md §17.17 states it.
`cargo test -p lexlean --lib group_tests` passes: the union is the same whichever
member Lean reaches first, and a member the audit did not print is refused.

### The check can fail

The per-row pin was restored (the union block of `group_rows` skipped with
`if true ||`), and `lexlean verify` run in `bootstrap/` on the identity of this
tree (`C3df…`, about 17 minutes under `LEXLEAN_VERIFY_WIDTH=2` and
`LEXLEAN_VERIFY_MEMORY_KIB=12582912`, with the pinned Lean installed):

```
error[LLV7013]: certificate A: the certificate rows of the mutual group
["LexLeanPreserve.C3df453b80ed6295dbadd323b69d0f1c6.R6.__fits_76._unsafe_rec",
 "LexLeanPreserve.C3df453b80ed6295dbadd323b69d0f1c6.R6.__fits_106._unsafe_rec",
 "LexLeanPreserve.C3df453b80ed6295dbadd323b69d0f1c6.R6.__fits_147._unsafe_rec"]
print the union ["propext"], but the policy of
`LexLeanPreserve.C3df453b80ed6295dbadd323b69d0f1c6.R6.__fits_76._unsafe_rec`
pins []
```

The plant was restored with `git checkout`. The same verification with the fix in
place is the regeneration of the bootstrap records in this change.
