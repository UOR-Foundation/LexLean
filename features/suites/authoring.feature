Feature: authoring

  @AU-01 @build
  Scenario: Every value of the closed semantic-module schema has one notation and the notation reads back to the value: every alternative of the declaration, term, type, and proof unions, with and without its optional members, is written and read back equal, a node that no readable form carries exactly is written generically, the notation is its own canonical form, and every committed semantic module is written and read back equal to its data.
    Given the semantic-module schema and every committed semantic module
    When every alternative of the declaration, term, type, and proof unions is sampled with and without its optional members, and each sample and each module is written in the notation and read back
    Then each is read back equal, the notation written from what is read is the notation written, and a node no readable form carries exactly is written generically

  @AU-02 @build
  Scenario: A language-1.2 semantic module written in the authoring notation links to exactly the module its data form links to, with the same semantic ID and the same generated Lean and LaTeX byte for byte and source maps that relate each declaration to its text in the notation; fmt writes a module written as data in the notation, to the committed bytes, and leaves the notation unchanged, and the notation under language 1.1 is refused with LLP2003.
    Given committed language-1.2 examples and their copies written as data
    When each pair is checked, built, and format-checked, and the notation is put under language 1.1
    Then the semantic IDs and the generated Lean and LaTeX are equal, the source maps name the notation, fmt writes the data form as the committed notation and leaves the notation unchanged, and language 1.1 refuses it

  @AU-03 @build
  Scenario: Malformed notation fails closed and never panics: an unclosed delimiter, a keyword used as a name, a literal that is not canonical, an unknown kind, and every single-token deletion, duplication, and replacement of a committed module fail with a registered code at a position in the notation or link to a module that decodes, and a numeral with a redundant leading zero is LLL1003 outside a string and is not one inside it.
    Given a committed language-1.2 module in the notation
    When it is malformed in each way named, and each of its tokens is deleted, duplicated, and replaced in turn, and numerals with a redundant leading zero are written outside and inside a string
    Then each is refused with a registered code at a position or links to a module that decodes, nothing panics, and only the numeral outside a string is LLL1003

  @AU-04 @build
  Scenario: No language-1.2 source of the product is written as data: every committed language-1.2 source with a semantic, tooling, or core module, in the compiler, the language libraries, the examples, and the positive fixtures, is written in the authoring notation and is canonical, the data form remains only under language 1.0 and 1.1 and in refused fixtures whose payload the notation cannot spell, and a refused fixture whose payload the notation can spell is a defect.
    Given every committed source of the repository that has a semantic, tooling, or core module
    When its language and its form are read, and each refused fixture written as data is written in the notation and read back
    Then no language-1.2 source of the product is written as data, each is canonical, and a refused fixture that the notation can spell is not written as data

  @AU-05 @build
  Scenario: A language-1.2 tooling module written in the authoring notation links to exactly the module its data form links to, with the same semantic ID and the same generated Lean and LaTeX byte for byte and source maps that relate each declaration to its text in the notation; the notation and the canonical JSON of every committed tooling module are in bijection and the notation is its own canonical form; fmt writes a tooling module written as data as exactly the committed notation and leaves the notation unchanged; and malformed notation, and every single-token deletion and duplication of a committed tooling module, fails with a position or decodes, and nothing panics.
    Given every committed tooling module and the committed tooling fixtures, and their copies written as data
    When each module is written in the notation and read back, each fixture and its data form is linked, formatted, and rendered, and every token of a committed module is deleted and duplicated in turn
    Then each is read back equal, fmt writes the data form as the committed notation, the semantic IDs, Lean and LaTeX are equal, the source maps name the notation, and each mutation is refused with a position or decodes

  @AU-06 @build
  Scenario: The compiler-semantics ID does not depend on the form a module of the embedded tree is written in: a semantic, tooling, or core module in the authoring notation contributes its data form, so the committed tree and the same tree with every such module written as data have one ID, a change to a module's meaning changes the ID in either form, the identity a preservation record states for the native library is over the same data forms, so it is the committed one, and the embedded tree holds at least one module written in the notation.
    Given the embedded tree, and the same tree with every module written in the notation written as data
    When the tree digest of each is taken, a declaration of a module is removed in either form, and the library identity of a preservation record is taken over each
    Then the two trees have the compiler-semantics ID, the changed tree has another in both forms, and the two library identities are equal and are the one the committed record states

  @AU-07 @build
  Scenario: No product source is the output of a generator: every `.lex.tex` source of `compiler/`, `language/`, and `examples/` is written and edited as source, and no writer of the repository (the generators `cargo xtask check-calculus` compares and rewrites) emits one of them except the four fixture modules of `compiler/` (`TargetFixtures`, `ReasoningFixtures`, `GnafFixtures`, `GnafCertificates`), generated from committed fixtures, requests, and certificates, and the extraction adapter's module, generated from the registry the pinned Lean answers; a source emitted by any other writer is a defect.
    Given every file the writers of the repository emit, and every committed source of compiler, language, and examples
    When the emitted sources of those trees are listed
    Then they are exactly the four fixture modules and the extraction adapter's module

  @AU-08 @build
  Scenario: The authoring API reads, writes, converts, and assembles the source of a module without a project: for every committed semantic and tooling source and every core source of at most twelve megabytes it reads the module it links to, writes it back in the form it has and in the other form, assembles the source from its header and data to the canonical notation, and is idempotent in each; and a source that is not a module, or whose notation is malformed, is an error with a position in the source and never a panic.
    Given every committed semantic and tooling source, every core source of at most twelve megabytes, and some that are not modules
    When each is read, written back in its form, converted to the other form and back, and assembled, and the malformed are read
    Then each is its module, the conversions are idempotent, the assembly is the canonical source, and the rest are errors with a position and no panic

  @AU-09 @build
  Scenario: Every core module of the closed schema has one notation and the notation reads back to the module and to its data form: every committed core module reads back equal from its notation, its canonical JSON is the canonical JSON the compiler accepts and is, for a module written as data, byte for byte the data it was read from, the notation is its own canonical form, a node is written inside the node that uses it exactly when it is used once by the node after it and every other node is a statement, and a module that carries every kind of node, level, annotation, and declaration member is written and read back equal.
    Given every committed core module, the Atlas modules read as modules of the schema the notation names, and a module that carries every kind of node, level, annotation, and declaration member
    When each module is written in the notation and read back, its data form is written, and the statements of each notation are compared with the nodes the rule writes inside their user
    Then each module is read back equal, the data form is the committed canonical JSON, the notation written from what is read is the notation written, and exactly the nodes the rule names are written inside their user

  @AU-10 @build
  Scenario: A language-1.2 core module written in the authoring notation links to exactly the module its data form links to, with the same semantic ID and the same generated Lean and LaTeX byte for byte and source maps that name the notation; fmt writes a core module written as data as the notation and leaves the notation unchanged; and the notation under language 1.0 and 1.1 is refused with LLP2003.
    Given a language-1.2 core module and its copy written as data, and the same module under language 1.0 and 1.1
    When each pair is checked, built, and format-checked, and the notation is put under language 1.0 and 1.1
    Then the semantic IDs and the generated Lean and LaTeX are equal, the source maps name the notation, fmt writes the data form as the notation and leaves the notation unchanged, and language 1.0 and 1.1 refuse it

  @AU-11 @build
  Scenario: Malformed core notation fails closed and never panics: an unclosed delimiter, a keyword used as a name, a label that is not the number of its node, a reference to a node not yet written, a numeral with a redundant leading zero, a statement that defines no node, a nesting beyond the bound, and every single-token deletion and duplication of a committed module fail with a registered code at a position in the notation or link to a module that decodes.
    Given a committed core module in the notation
    When it is malformed in each way named, and each of its tokens is deleted and duplicated in turn
    Then each is refused with a registered code at a position or links to a module that decodes, and nothing panics

  @AU-12 @build
  Scenario: A core module written with names for its statements and its variables reads to exactly the module its canonical notation reads to, and the canonical notation keeps the names of the binders and writes the numbers of the statements: a variable is written by the name of its binder exactly when no nearer binder has the name, a constant a binder in scope is named like is written as a string, a theorem added to the certification library by name reads to the committed module and is written as the committed bytes, and an unknown or repeated statement name and a variable given universe levels are refused with a position.
    Given every committed core module in the notation, the last module of the certification library without the theorem it ends with, and a project whose module is written with named statements
    When every number of a statement is replaced by a name and the text is read and written, the theorem is written by name and appended, the project is format-checked and formatted, and an unknown name, a repeated name, and a variable with levels are put in
    Then each reads to the committed module and is written as the committed bytes, fmt writes the numbers, and each defect is refused with a position
