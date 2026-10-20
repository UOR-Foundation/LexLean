Feature: normative-source

  Normative text as LexLean source: the conformance register, the diagnostic registry, the ledger, and the authority register are modules of one project, and the model files and generated documents are projections of it.

  @NS-01 @build
  Scenario: The specification project is a language-1.2 workspace member whose every module is a semantic module in the authoring notation; it is canonical under fmt, locked, checked, and built, and under the pinned toolchain it verifies, which establishes that each record definition elaborates against its row structure and that the generated modules replay with the audited axioms, and does not establish that a row's text is the normative text, that identifiers are distinct, or that a projection equals its committed file, which NS-02 to NS-04 measure.
    Given the committed `spec` project beside `bootstrap` and `stdlib`
    When the project is formatted, locked, checked, and built, and verified under the pinned toolchain
    Then every module is a semantic module in the authoring notation, the project is canonical and its lock is current, and it builds and verifies

  @NS-02 @build
  Scenario: The conformance register, the diagnostic registry, the honesty ledger, and the authority register are modules of the specification project, and `model/ids.toml`, `model/errors.toml`, `model/ledger.toml`, `model/authorities.toml`, `CONFORMANCE.md`, and `ERRORS.md` regenerate from them byte-identically.
    Given the register, error, ledger, and authority modules of the committed `spec` project
    When the modules are projected to the model files and the two generated documents
    Then each projection is byte-identical to its committed file

  @NS-03 @build
  Scenario: A difference between a model file or SPEC.md and the projection of the specification project's modules, in either direction, fails the repository gate naming R1, a source edit that is not projected being such a difference, and the section references of the §31 table are checked against those the register modules carry.
    Given the committed sources and projections, and a plant of a hand edit of a model file or of SPEC.md, of a source edit that is not projected, and of a section reference that differs from the table
    When the repository gate runs on each plant
    Then each plant fails naming R1 or RP-07 and the committed tree passes

  @NS-04 @build
  Scenario: The projector fails closed on a module of the specification project that holds a declaration kind, a term kind, a field, a name, a file, or a family it does not read, and on a duplicate identifier or code, and a plant of each is refused naming the file and, for a declaration, its position.
    Given the projector and a plant of each foreign declaration kind, term kind, field, name, file, family, and duplicate
    When the projector reads each planted module
    Then each plant is refused naming the file and, for a declaration, its position, and the committed modules project

  @NS-05 @build
  Scenario: SPEC.md regenerates byte-identically from the chapter modules of the specification project, its conformance-ID table, its diagnostic-code table, and its stated total being derived from the register and registry modules and never typed, and the projector refuses a chapter item it does not read.
    Given the chapter, register, and registry modules of the committed `spec` project, and a plant of each foreign chapter item, a hand edit of a derived table, and a register edit that is not projected
    When the modules are projected to SPEC.md and the repository gate runs on each plant
    Then SPEC.md is byte-identical to the projection, each derived region equals what the register and registry modules give, and each plant is refused naming R1 and its position
