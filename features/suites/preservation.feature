Feature: preservation

  Semantic preservation from the source to the realization calculus: the lowering, certificate A, the differential evaluator, and the preservation library (§17.17).

  @SP-01 @build
  Scenario: Every production root of the committed examples and the preservation corpus lowers to a valid realization program in first-binding order, byte-identical across two lowerings, with an origin for every function and document type and exactly the root's eligibility closure; the lowering and certificate sources match no construct by default; a planted closure disagreement fails with LLI9001 and a planted default arm is refused.
    Given every production root of the committed examples and of the preservation corpus
    When each is lowered twice and its program, layout, and closure are inspected, and the lowering and certificate sources are audited
    Then each program is valid, canonical, identical across the two lowerings, fully attributed, and exactly its eligibility closure
    And a report whose closure omits a member fails with LLI9001, and a default arm planted in a preservation source is refused

  @SP-02 @build
  Scenario: Every production root of examples/production and the preservation corpus has a certificate whose root theorem, that the lowered program converges on the encoded arguments to the encoded source value or to overflow exactly where the width predicate fails, compiles under the pinned Lean, replays through leanchecker, and depends on exactly Classical.choice, Quot.sound, and propext; a certificate generated against a program with a planted branch, arithmetic, constructor, recursion, or literal mutation is rejected.
    Given every production root of examples/production and of the preservation corpus
    When each certificate is compiled with the pinned Lean beside the library and the generated modules, replayed, and its axioms audited
    Then every certificate is accepted and its root theorem depends on exactly the three classical axioms
    And a certificate generated against each planted program mutation is rejected

  @SP-03 @build
  Scenario: On seeded inputs to every production root of examples/production and the preservation corpus, the calculus interpreter's outcome on the lowered program equals the certificate's observation evaluated by Lean, and a planted disagreement is detected.
    Given seeded inputs to every production root of examples/production and of the preservation corpus
    When the interpreter runs the lowered program and Lean evaluates the certificate's observation on the same inputs
    Then every interpreter outcome equals the observation Lean prints
    And an interpreter outcome altered in one case is reported as a disagreement

  @SP-04 @build
  Scenario: Every declaration of the preservation library depends on exactly the axioms library.toml registers, the shipped calculus modules are byte-equal to the compiler project's golden modules, and a library module or certificate with a forbidden token, a disallowed option, or a foreign import is refused.
    Given the preservation library, its registry, and the shipped calculus modules
    When the library is compiled and every declaration's axioms printed, and the shipped modules are compared with the compiler golden
    Then every declaration's axioms equal the registry and every shipped module equals its golden
    And a library module with a planted forbidden token, option, or import is refused
