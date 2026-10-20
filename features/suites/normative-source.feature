Feature: normative-source

  Normative text as LexLean source: the specification, the conformance register, the diagnostic registry, the ledger, and the authority register are `.lex.tex` modules of one project, and SPEC.md and the model files are generated projections of it.

  @NS-01 @build
  Scenario: The specification project is a language-1.2 workspace member whose modules are semantic modules written in the authoring notation: it checks, builds, and verifies under the pinned toolchain, and the `.lex.tex` modules of the project are the only hand-edited normative inputs of the repository.
    Given the committed `spec` project beside `bootstrap` and `stdlib`
    When the project is formatted, locked, checked, built, and verified under the pinned toolchain, and the repository is audited for hand-edited normative inputs
    Then every module of the project is a semantic module in the authoring notation, the project verifies, and no normative input outside the project's `.lex.tex` modules is hand-edited

  @NS-02 @build
  Scenario: The conformance register, the diagnostic registry, the honesty ledger, and the authority register are LexLean modules of the specification project, and `model/ids.toml`, `model/errors.toml`, `model/ledger.toml`, `model/authorities.toml`, `CONFORMANCE.md`, and `ERRORS.md` regenerate from them byte-identically.
    Given the register, error, ledger, and authority modules of the committed `spec` project
    When the project's modules are projected to the model files and the two generated documents
    Then each projection is byte-identical to its committed file

  @NS-03 @build
  Scenario: SPEC.md regenerates byte-identically from the document modules of the specification project, and its conformance-ID table, its diagnostic-code table, its authority table, and its stated total are derived from the register modules and never typed.
    Given the document and register modules of the committed `spec` project
    When the document modules are projected to SPEC.md
    Then SPEC.md is byte-identical to the projection and its three derived tables and its total equal those derived from the register modules

  @NS-04 @build
  Scenario: A difference between a normative source and its projection, in either direction, fails the repository gate naming R1, and the specification-link gate checks the projection instead of a hand-kept bijection.
    Given the committed sources and projections, and a plant of a hand edit of a projection and of a source edit not projected
    When the repository gate runs on each plant
    Then each plant fails naming R1, the specification-link gate reads the projection, and the committed tree passes

  @NS-05 @build
  Scenario: The projector fails closed on a module of the specification project that holds a declaration kind, a term kind, a field, a name, or an ordering it does not read, and on a duplicate identifier or code, and a plant of each is refused with the position of the offending declaration.
    Given the projector and a plant of each foreign declaration kind, term kind, field, name, ordering, and duplicate
    When the projector reads each planted module
    Then each plant is refused with the position of the offending declaration and the committed modules project
