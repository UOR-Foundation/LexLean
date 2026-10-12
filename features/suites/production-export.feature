Feature: production-export

  The production export contract: the stable façade through which a consumer compiles production roots to certified Rust packages, its fail-closed request checks, unforgeable certification, determinism, confinement, offline operation, schemas, and source-free replay (§24.7).

  @PX-01 @build
  Scenario: A consumer that names only lexlean::api::production and the stable crate-root types exports every production root of examples/production and examples/production-coverage through production::build into a bundle whose every Rust package is byte-identical to its committed expected package and renders the certified crate followed by one export, run, of the root's recorded entry, and whose result binds every output to the project's source, semantic, and lock identities, the compiler-semantics ID, its target ID and descriptor digest, the compiler input and preservation record the attestation binds, certificates A, B, and E, and the certification status kernel-checked, with optimality not-claimed.
    Given the committed production examples and a consumer module that names only the stable surface
    When the consumer exports every production root of each example in every target it declares
    Then every package is byte-identical to its committed expected package and begins with the certified crate
    And the result binds each output to the source, semantic, lock, compiler-semantics, target, compiler-input, preservation, and certificate identities with status kernel-checked and optimality not-claimed

  @PX-02 @build
  Scenario: A production request naming an unknown root, a declaration that is not a production root, or a formal-only theorem fails with LLC0202 before any Lean process runs and publishes no build, verification, or production directory.
    Given the production example
    When a request names an unknown root, an executable declaration that is not a production root, and a theorem
    Then each fails with LLC0202 before any Lean process runs
    And no build, verification, or production directory is published

  @PX-03 @build
  Scenario: A production request naming an unregistered target, a registered target its root does not declare, or a registered target with any other descriptor digest fails with LLC0203 before any Lean process runs and publishes nothing, and production::targets returns each registered target's descriptor with a digest that changes with any field of it.
    Given the production example and the registered target descriptors
    When a request names an unregistered target, an undeclared target, and a target with a changed digest, and each descriptor field is changed in turn
    Then each request fails with LLC0203 before any Lean process runs and nothing is published
    And every changed descriptor field changes the digest

  @PX-04 @build
  Scenario: A production request pinning a source ID, semantic ID, or lock digest that a later edit of a source module or of the lock no longer matches fails with LLC0204 before any Lean process runs and publishes nothing, and a lock made stale by an edit of the project configuration fails with LLC0102.
    Given the production example's source, semantic, and lock identities pinned in a request
    When a source module, the lock, and the configuration are edited after pinning
    Then each stale pin fails with LLC0204 before any Lean process runs and nothing is published
    And the stale lock fails with LLC0102

  @PX-05 @build
  Scenario: Every committed production-refusal fixture of tests/negative fails through production::build with its committed diagnostic code before any Lean process runs and publishes no production directory.
    Given every committed production-refusal fixture and its committed diagnostics
    When each fixture is exported through production::build
    Then each fails with its committed code before any Lean process runs
    And no production directory is published

  @PX-06 @build
  Scenario: GNAF evidence whose certificate has another identity than the request pins, earns a refused verdict, or is for a request whose reference program is not the root's lowered program fails with LLV7101, a malformed request or certificate fails with LLB6006 or LLB6007, each before any Lean process runs, a request carrying a certification, optimality, preservation, certificate, Lean, program, or crate field fails with LLC0201, and evidence that binds the root is recorded with the identity, universe, and verdict the host checker recomputes.
    Given a production root, a GNAF request whose reference program is its lowered program, and certificates for it
    When evidence with a forged identity, a refused verdict, another reference program, and malformed documents is offered, and requests carrying claim fields are read
    Then each forged input fails with its registered code before any Lean process runs and no claim field deserializes
    And binding evidence is recorded with the verdict the host checker recomputes

  @PX-07 @build
  Scenario: verify_artifact accepts an untouched bundle with status bindings-verified and names the four stages it does not re-run, and rejects with LLV7102 a bundle in which one byte of a package, crate, program, certificate, attestation, compiler input, or preservation record, one binding or certification field of the result with its identity recomputed, or one added or removed file differs, and with LLS8001 a bundle holding a symlink.
    Given a published production bundle
    When the bundle is replayed untouched and then with each single tamper
    Then the untouched bundle replays with status bindings-verified and the stages it does not re-run
    And each tamper fails with LLV7102 and a symlink with LLS8001

  @PX-08 @build
  Scenario: Exporting one request from two copies of a project in distinct absolute directories and from distinct working directories yields byte-identical bundles under one result ID, and the result holds no absolute path, environment value, or time.
    Given two copies of the production example in distinct absolute directories
    When each copy exports the same request from a different working directory
    Then the bundles are byte-identical under one result ID
    And the result holds no absolute path, environment value, or time

  @PX-09 @build
  Scenario: Evidence paths that are absolute, contain .., or pass through a symlink leaving the project fail with LLS8001 before any Lean process runs, and a symlinked production area under the build root is refused with LLS8001 rather than followed.
    Given the production example with evidence paths and a production area outside the project
    When a request names absolute, parent-relative, and symlinked evidence, and the production area is a symlink
    Then each evidence path fails with LLS8001 before any Lean process runs
    And the symlinked production area is refused with LLS8001 and nothing outside the project is written

  @PX-10 @build
  Scenario: After setup, production::build takes no network option and starts no network or git operation: with a git sentinel first on PATH and an unreachable proxy in the environment it exports the project and the sentinel never runs, and a project whose lock needs an unacquired Git package fails closed with a registered code without acquiring it.
    Given a set-up project, a git sentinel first on PATH, and an unreachable proxy
    When the project is exported, and a project whose lock needs an unacquired Git package is exported
    Then the export succeeds and the sentinel never runs
    And the unacquired package fails closed with a registered code and nothing is fetched

  @PX-11 @build
  Scenario: No production request type can carry Lean, LCNF, a target program, or Rust: every field is a root name, a crate name and version, a registered target ID with a digest, a pinned digest, or a project-relative evidence path, as an exhaustive destructuring of each request type shows, and a root name, crate name, or version carrying Lean or Rust text fails with LLC0201, while evidence naming a Lean file fails as a malformed GNAF document.
    Given every production request type
    When each type is destructured exhaustively and Lean and Rust text is placed in each string field
    Then every field is of a closed form
    And each injection fails with LLC0201 or as a malformed GNAF document

  @PX-12 @build
  Scenario: Each production export schema under api/schemas validates the values the façade produces and refuses an unknown field, and its SHA-256 is recorded under its exact spec version in api/schemas/versions.toml, so that a schema changed without a new version fails the gate.
    Given the production export schemas, their recorded versions, and the values the façade produces
    When each value is validated, an unknown field is added, and a schema is changed under its recorded version
    Then every value validates and every unknown field is refused
    And a schema changed without a new version fails the gate
