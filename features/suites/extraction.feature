Feature: extraction

  Named-root extraction through Lean's compiler front end: the pinned authority interface, the closed compiler input, and fail-closed rejection (§22.10).

  @NE-01 @build
  Scenario: Verifying a project with production roots extracts every root through Lean's compiler front end into one canonical compiler input whose bytes and ID are schema-valid, recorded in the attestation, and identical from distinct project directories, and a project without a production root publishes none.
    Given the committed `production` example and its committed extraction record
    When the record is read into the canonical compiler input and the example is verified from two project roots under pinned Lean
    Then the input validates against `schemas/compiler-input.schema.json`, equals the committed input byte for byte, and survives renumbering of every LCNF variable
    And both roots publish the same `production/compiler-input.json`, the attestation records its SHA-256, and the `recursion` example publishes no input

  @NE-02 @build
  Scenario: Each root's extracted closure is exactly its computational dependencies, equals its production-eligibility closure, carries its runtime members and monomorphization instances, names every constant its code uses, marks recursion from the use graph, and records proof-only dependencies as erased and never as runtime members.
    Given the canonical compiler input of the `production` example
    When each root's closure is compared with the eligibility report computed from the semantic IR
    Then the two closures are equal for every root, the eleven definitions include no theorem, and `halvings` erases `countdown_decreases`
    And every external is a Lean core constant or a runtime member, and a disagreement on erased proofs is rejected

  @NE-03 @build
  Scenario: An unknown root, an opaque, axiomatic, unsafe, partial, or noncomputable dependency, an external implementation, an unresolved external, an unsupported compiler form, a kind that differs from the declared kind, and a malformed, noisy, or foreign extraction record fail closed with LLV7011, while a core definition its module exports as an axiom is admitted as that definition exactly when it is computable and Lean's compiler holds its code, and a borrowed type annotation canonicalizes to the type it annotates.
    Given the committed extraction record
    When one closure member is made opaque, an axiom, partial, unsafe, noncomputable, or external, a type unsupported, an external foreign or axiomatic, a root unknown, or the record noisy, extended, or answering other roots
    Then each fails with LLV7011 naming the rejection class
    And an external exported as an axiom is admitted only when declared a computable definition with compiled code, and a borrowed domain canonicalizes away
    And pinned Lean itself refuses an extraction module whose root does not exist

  @NE-04 @build
  Scenario: Every constant the extraction adapter uses is registered exactly once, as a call with its exact signature and pinned source identity, a type with its exact constructors, or plumbing; each extraction compares every signature structurally and the adapter's constants with the registry under pinned Lean, the adapter runs no LCNF pass, and drift of a signature, a constructor list, the adapter's constants, its output, or the Lean identity fails with LLV7012.
    Given the embedded registry `language/lcnf-1.2/authority.toml` and adapter `language/lcnf-1.2/extract.lean`
    When the registry is compared with the adapter, the pinned toolchain sources, and pinned Lean
    Then every call the adapter makes has a row whose source SHA-256 matches the toolchain, every probe elaborates, and the adapter has no comment and runs no LCNF pass
    And a changed signature fails on its probe line and a foreign Lean commit fails with LLV7012

  @NE-05 @build
  Scenario: A dependency dropped from Lean's extracted facts or from the production-eligibility closure fails extraction with LLV7011 before any compiler input is published.
    Given the committed extraction record and the production example's eligibility reports
    When `Production.Kernel.area` is removed from Lean's closure, from its declarations only, or from the eligibility closure
    Then each fails with LLV7011 naming the dropped dependency
    And no compiler input is produced

  @NE-06 @build
  Scenario: A proof-only dependency presented as a runtime closure member fails extraction with LLV7011 before any compiler input is published.
    Given the committed extraction record
    When the termination evidence `Production.Kernel.countdown_decreases` is presented as a runtime member of `halvings`, or `area` is both erased and realized
    Then each fails with LLV7011
    And no compiler input is produced
