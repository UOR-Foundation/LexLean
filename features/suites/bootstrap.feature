Feature: bootstrap

  The compiler's own components as language-1.2 source: each is a certified production root held to the Rust it replaces by a differential, and the Rust stays in the tree as the stage-0 oracle until the fixed point of issue 92 (§17.23).

  @BS-01 @build
  Scenario: The axiom-output parser of SPEC.md §22.5 is language-1.2 source in the `bootstrap/` project: `parseAuditOutput` and the closed Lean-name check `isClosedLeanName` are production roots eligible for `rust-std` with exactly their declared effects; the parser's lowered program under the calculus evaluator agrees with the Rust parser `verify::axiom` on the result, the rejection message, and the attributed declaration for every committed golden vector (accepted, accepted-wrapped, and rejected); both roots' rendered `rust-std` packages under the pinned rustc agree with the Rust oracle on a large seeded corpus of generated, wrapped, and mutated audit outputs with no disagreement; a planted defect in the source is detected; and the certified packages the production export publishes for the project are the committed ones.
    Given the committed `bootstrap` project, the committed golden vectors of the axiom parser, and the Rust parser `verify::axiom` as the oracle
    When the project is checked and built, the parser's lowered program is run by the evaluator on every golden vector, both roots' rendered rust-std packages are run on a seeded corpus, a defect is planted in the source, and the project is exported under the pinned Lean
    Then both roots are eligible for rust-std with exactly their declared effects and no run disagrees with the oracle
    And every planted defect is detected, and every exported package is byte-identical to its committed package and realizes the lowered program
