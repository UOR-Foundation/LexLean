Feature: kappa-addressing

  kappa-addressing in LexLean: the canonical forms and kappa-label of uor-addr written as a source package beside the standard library and validated against uor-addr as an oracle (decision D7 of issue 110, issue 127).

  @KA-01 @build
  Scenario: κ-addressing is the language-1.2 project `kappa` of the workspace, beside `stdlib` and outside `language/` and `schemas/`, under the module prefix `LexLeanKappa`, whose manifest declares the standard library as its one source package (`std`, the path `stdlib`) and no artifact or glossary, and whose two modules `Label` and `Json` verify under the pinned Lean with exact axiom policies; its roots `kappaLabel`, `isLabel`, and `handleWire` of `Label` and `canonicalJson` and `jsonLabel` of `Json` are production roots eligible for `rust-std` with exactly their declared effects; and adding the package moves no compiler-semantics identity: the identities of languages 1.0, 1.1, and 1.2 equal the values pinned in the committed locks, and no file of `language/` or `schemas/` names the package.
    Given the committed kappa project, the workspace file, the standard library it imports, and the committed locks
    When the project is checked and verified, its production roots are read from the eligibility report, and the compiler-semantics identities of the three languages are recomputed with the package present
    Then every module verifies with exact axiom policies and every root is eligible for rust-std with exactly its declared effects
    And the identities equal the pinned values and no file of language/ or schemas/ names the package

  @KA-02 @build
  Scenario: The κ-label of the package is the one of uor-addr: `kappaLabel` of canonical bytes is `sha256:` followed by the sixty-four lowercase hexadecimal digits of their SHA-256, seventy-one bytes, and `isLabel` accepts exactly that form, which it accepts on every committed `.kappa-label` file of uor-addr; the data handle of decision D3 is a kind and that label, `handleWire` writing it as the kind, `@`, and the label for a kind of lowercase letters, digits, and hyphens and refusing every other kind; and for each of the twelve JSON fixtures of uor-addr's byte-identity test, the label `jsonLabel` gives from the raw input equals the committed address and the address `uor_addr::json::address` gives, a disagreement failing closed and never resolved by adopting the oracle's label.
    Given the uor-addr crate as the oracle, its twelve committed JSON fixtures, and its committed kappa-label files
    When each fixture is labelled by the package and by the oracle, each committed label file is read by isLabel, and handles are written for valid and invalid kinds
    Then every label equals the committed address and the oracle's address
    And isLabel accepts exactly the committed form and handleWire writes kind@label only for a valid kind

  @KA-03 @build
  Scenario: The canonical JSON form of the package is that of uor-addr (RFC 8785 with Unicode NFC: members in the byte order of their NFC UTF-8 keys, no insignificant white space): `canonicalJson` is a production root whose lowered program under the calculus evaluator and whose rendered `rust-std` package under the pinned rustc give exactly the bytes of `uor_addr::json::canonicalize` on a seeded corpus of nested objects, arrays, strings with escapes and control characters and the Latin-1 letters in composed and decomposed form, integers, booleans, and null; it refuses, with no output, malformed JSON, a duplicate member name, a number that is not an integer (a fraction, an exponent, or minus zero), and a string outside the subset of Unicode whose NFC form it computes (ASCII, Latin-1, and the combining marks U+0300 to U+0303, U+0308, U+030A, and U+0327 after a letter that composes with them), and it never guesses an NFC form; and a planted member order, a planted escape, and a planted composition are each detected by the differential.
    Given the uor-addr crate as the oracle and a seeded corpus of JSON documents
    When canonicalJson runs under the calculus evaluator and as a rustc-compiled rust-std package, the inputs outside its subset are read, and a planted member order, a planted escape, and a planted composition are each run in turn
    Then both realizations give exactly the oracle's canonical bytes and refuse what the statement lists with no output
    And each planted defect is detected by the differential
