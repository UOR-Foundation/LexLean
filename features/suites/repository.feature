Feature: repository

  Repository identity, layout, gates, and generated-document discipline (§2, §7, §9, §27, §30).

  @RP-01 @build
  Scenario: The repository, crate, executable, metadata, and licenses have the exact LexLean identity specified.
    Given the workspace Cargo.toml, the lexlean crate manifest, and the two license files
    When the section 2.3 metadata rows and the exact one-line crate description are looked up and `lexlean --version` runs
    Then Cargo.toml contains version 0.2.0, edition 2021, rust-version 1.97, and license MIT OR Apache-2.0
    And LICENSE-APACHE and LICENSE-MIT exist as files
    And the binary prints `lexlean 0.2.0` on its first stdout line with exit 0

  @RP-02 @build
  Scenario: The repository is derived from the pinned UOR template commit and contains no inherited domain-specific claim logic.
    Given AGENTS.md, SPEC.md, xtask/src/audit.rs, xtask/src/main.rs, and the Justfile
    When the pinned template commit 0a1c799338d7db829aa23365e1acf4f9d01ff8b5 is searched for
    Then AGENTS.md and SPEC.md both record that commit
    And none of the four tooling files still mentions the template's audit-limits remnant

  @RP-03 @build
  Scenario: The completed repository has the required file and crate layout, and the tracked tree holds no CACHEDIR.TAG or path component the repository walkers skip outside sanctioned expected/ oracle directories.
    Given the section 7 layout tree parsed out of the SPEC.md text fence and the tracked file list from git ls-files
    When every tree row is resolved to a path under the repository root and every tracked path is inspected for CACHEDIR.TAG and walker-skipped components
    Then each row ending in a slash exists as a directory and every other row exists as a file
    And more than 80 rows were checked
    And no tracked path is named CACHEDIR.TAG or has a walker-skipped component outside sanctioned expected/ oracle directories

  @RP-04 @build
  Scenario: Only the lexlean crate is shipped and no shipped crate depends on repository-only tooling.
    Given the Cargo.toml manifests of crates/model, crates/conformance, xtask, and crates/lexlean
    When each manifest is inspected for `publish = false` and dependency names
    Then the three tooling crates declare `publish = false` and the lexlean crate does not
    And the lexlean manifest names none of repo-model, repo-conformance, or xtask

  @RP-05 @build
  Scenario: The just vv recipe runs every normative acceptance gate in the specified order.
    Given the committed Justfile
    When the vv recipe line is read
    Then it reads `vv: fmt-check model spec-links lint test features bdd inventory examples golden repro deny`
    And the Justfile also defines the model-write and golden-write recipes plus each of the twelve gates

  @RP-06 @build
  Scenario: CONFORMANCE.md and ERRORS.md are exact generated views of model files.
    Given the model directory loaded through repo_model::Model::load
    When CONFORMANCE.md and ERRORS.md are re-rendered from the loaded model in memory
    Then the committed CONFORMANCE.md is byte-equal to the rendered conformance view
    And the committed ERRORS.md is byte-equal to the rendered errors view

  @RP-07 @build
  Scenario: The specification conformance table and model register are bijective and text-consistent.
    Given the section 31 table of SPEC.md parsed into id, suite, and statement rows
    When the table is zipped against the model ids register in order
    Then both hold exactly 211 rows
    And each row's ID, suite, and statement text is equal in the table and the register

  @RP-08 @build
  Scenario: Repository source contains no unsanctioned deferral marker, stub, placeholder, ignored capability, or disabling feature.
    Given every rs, toml, json, feature, tex, and lean file outside target, .git, .lexlean, and expected directories
    When each file is scanned for the sanctioned list of deferral marker spellings, ignore attributes, and deferral macro calls
    Then no scanned file contains any marker
    And more than 100 files were scanned

  @RP-09 @build
  Scenario: The shipped crate forbids unsafe Rust and the audit proves the prohibition is active.
    Given crates/lexlean/src/lib.rs and every file under crates/lexlean/src
    When lib.rs is checked for the forbid unsafe-code crate attribute and every source is scanned for an unsafe keyword
    Then the forbid attribute is present in lib.rs
    And no shipped source file contains the unsafe token

  @RP-10 @build
  Scenario: The embedded compiler-semantics ID equals a clean recomputation from normative language and schema inputs.
    Given every file under language, schemas, tests/golden/axiom-parser, and tests/golden/canonical-json sorted by path bytes
    When lexlean::artifact::content_id::tree_digest recomputes the tree digest from those files
    Then the recomputed digest equals lexlean::compiler_semantics_id()

  @RP-11 @build
  Scenario: Every public README capability claim is tied to a registered model ID and honesty level.
    Given README.md and the loaded model ids register
    When every README table row marked `build` has its five-character IDs extracted
    Then each extracted ID is registered in the model at level build
    And at least 10 such capability rows are found

  @RP-12 @build
  Scenario: A release is refused unless the complete release criterion and all required artifacts are satisfied.
    Given the repository tree at version 0.2.0 and the repo_model::release CRITERIA list
    When repo_model::release::check runs against the tree
    Then release is refused and one refusal reason names source-tag
    And the criteria include source-tag, checksums, host-binaries, crate-package, semantics-id, conformance-doc, errors-doc, spec, licenses, sbom, and ci-evidence
    And the Justfile release recipe depends on vv and runs `cargo xtask release-check`

  @RP-13 @build
  Scenario: The ordinary workspace test phase excludes repo-conformance, the bdd phase alone runs its complete suite, and a test inventory proves that the two phases together equal the whole workspace test set with no test lost and none run twice, and that the parts of the committed bdd plan together equal the bdd phase with every test in exactly one part.
    Given the Justfile test and bdd recipes, .github/bdd-parts.json, and the test list of cargo test --workspace --all-features
    When the test lists of the test recipe and of the bdd recipe are taken
    Then the two lists are disjoint and their union equals the workspace list
    And no test of repo-conformance is in the test recipe's list and every conformance_<id> test is in the bdd recipe's list exactly once
    And a test recipe that includes repo-conformance, or a bdd recipe that runs fewer targets, fails the inventory
    And the test lists of the parts of the bdd plan, each taken with the part's harness filters, are disjoint and their union equals the bdd recipe's list, so that a test two parts run or a test no part runs fails the inventory

  @RP-14 @build
  Scenario: The normative CI job runs every recipe of just vv in parallel legs that each start from a clean checkout on the standard ubuntu-24.04 runner, each recipe exactly once but bdd, which runs once per part of its committed plan on the row that names the part, the examples leg at the verification resource profile its matrix row names and without UOR Atlas, which the Atlas jobs of the same run verify as the shards of the committed plan and a join at that profile, every process once and nothing across runs, and the other legs at the default, with no cache, path filter, or skipped recipe, and each job uploads its CI evidence; a plan job decides what a run is from the event alone: a draft pull request runs the rest leg and no other job, a ready pull request, a request, and a push to main run every job, and a push to main that merges a head whose tree it equals runs none when the whole gate passed on that head, and runs the whole gate when any job of that run did not pass, was skipped, or is missing.
    Given .github/workflows/ci.yml, .github/vv-rows.json, .github/ci-plan.sh, .github/verify-shards.json, and .github/bdd-parts.json
    When the runner, the environment, the job steps, and the plan are read, and the plan script is run against planted events
    Then every gate job runs on ubuntu-24.04 with the 360-minute limit, and each verification profile variable is set once per job, from the leg's matrix row or as the examples profile on an Atlas job
    And the leg that runs examples names width 2, a 12 GiB budget, and the committed hints, which the verifier parses, verifies every example but uor-atlas, and every other leg names no profile and no example part, and the legs run the recipes of vv, each exactly once but bdd, which runs once per part of the bdd plan on the one row that names the part, in the order of vv within a leg, through the one gate command
    And the Atlas jobs run every shard of the plan exactly once, each after the jobs of the shards it needs, from the one executable the prefix job built, the join asserts that Atlas verified, and the plan elaborates and replays every Atlas module exactly once with every import elaborated by the shard or by one it needs
    And the vv job has no path filter, no cache action, and no condition on a recipe, the Atlas jobs and the host matrix run only when the plan names the whole gate, and each job uploads its ci-evidence under a name of its own, and the plan script names the rest leg alone for a draft pull request and the whole gate for a ready one, a request, and a push, and names no job for a merge whose tree is its second parent's only when a run of that parent passed every job, the Atlas join, every host, and every leg, and for a skipped or failed job, a missing join, host, or leg, a failed call, a changed tree, and a push that merges nothing it names the whole gate
