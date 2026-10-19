Feature: production

  Language-1.2 production eligibility: targets, effects, construct dispositions, runtime closures, and the root analysis (§17.13).

  @PD-01 @build
  Scenario: The closed production registry fixes the language-1.2 targets, effects, and the disposition of every semantic construct kind, and every construct the eligibility analysis can classify has exactly one registry row.
    Given the embedded registry `language/production-1.2.toml`
    When its construct rows are compared with every key the eligibility analysis can produce and every construct kind of `schemas/semantic-module-v2.schema.json`
    Then each key and each schema kind has exactly one row and no row names an unknown construct
    And the targets are `rust-core` without allocation and `rust-std` with allocation at 64-bit widths, the effects are `allocation`, `overflow`, and `recursion`, and propositions, universes, and theorems are formal-only or erased

  @PD-02 @build
  Scenario: Formal-only theorems, propositions, and non-executable definitions coexist with eligible executable production roots, and a module that declares no production root is never analysed for production.
    Given the committed `production` example whose `Kernel` module holds a theorem and a proposition-valued definition and no root
    When the project is checked and built
    Then `Kernel` has no eligibility report while `Main` reports all six roots eligible beside its own theorems
    And the `recursion` example, which declares no root, publishes no production artifact

  @PD-03 @build
  Scenario: A production root's runtime closure contains exactly its transitive computational dependencies across modules at their type instantiations, and its termination evidence is recorded as erased and never realized.
    Given the eligibility report of the `production` example
    When each root's runtime closure is read
    Then it lists exactly the root and its transitive dependencies, `firstOr` instantiated at `Nat` with its call path, and `countdown_decreases` only as an erased dependency
    And removing the call to `countdown` from the source removes it and its evidence from the closure

  @PD-04 @build
  Scenario: Production eligibility depends on the declared target: a construct that requires heap allocation is an admitted effect on a target with allocation and makes the root ineligible on a target without it.
    Given the `sumAll` root declared for `rust-std` only
    When its report is read and the root is redeclared for `rust-core` as well
    Then `rust-std` realizes `allocation`, `overflow`, and `recursion`, the fixed-width `checkedSum` realizes no effect on either target
    And the redeclared root fails with LLT4005 naming `rust-core` and heap allocation and leaves no backend output

  @PD-05 @build
  Scenario: A production root fails with LLT4005 before any backend runs when it declares type parameters or is not declared executable, when its boundary holds a universe, proposition, type parameter, or function, directly or in a named type's fields, or when its closure reaches a formal-only construct, a literal outside the target width, unavailable allocation, or an effect the root does not admit.
    Given the nine `production-*` negative fixtures and a copy of the example with `shapeArea`'s admitted `overflow` removed
    When each is checked and built
    Then each fails with LLT4005 naming its violation, its target, and the call path to it, and no build or verification root exists
    And a root naming the unregistered target `rust-wasm` fails with LLT4001

  @PD-06 @build
  Scenario: Every production root's eligibility report is a deterministic, schema-valid build artifact recording its runtime closure, realized types, erased dependencies, constructs, and per-target effects with their sources.
    Given two independent copies of the `production` example
    When both are built
    Then `production/Production/Main.eligibility.json` is byte-identical, canonical, schema-valid, free of absolute paths, and recorded in the manifest with kind `production-eligibility` and its digest
    And every reported effect is admitted and names its sources, every reported construct is a runtime row, and the snapshot validates against `semantic-snapshot-v2`

  @PD-07 @build
  Scenario: The eligibility analysis classifies every semantic construct by an explicit exhaustive match, and the exhaustiveness audit rejects a planted wildcard arm, rest pattern, implicit-default binding form, or unnamed IR variant.
    Given the committed eligibility analysis source
    When the exhaustiveness audit reads it against `crates/lexlean/src/ir/semantic.rs`
    Then it passes and every IR variant is named
    And a planted wildcard arm, rest pattern, `if let`, or unnamed `SemanticInteger::UInt64` each fails the audit

  @PD-08 @build
  Scenario: Every root that a conformance harness runs on a corpus and that admits `recursion` has a measured stack bound in `tests/stack-bounds.json`, whose rows are exactly the recursive packages of the committed `bootstrap` and `stdlib` exports, and the stack of the harness thread (`rust_harness::HARNESS_STACK`) and of the evaluator thread is at least the largest recorded bound times the stated headroom of 4, so that a harness stack below that bound is refused and a harness binary run on a quarter of its bound aborts instead of finishing; the bounds are measurements of the cases' own inputs on one host, evidence and not a guarantee for another host or input.
    Given the committed `bootstrap` and `stdlib` production exports, the stack bounds recorded in `tests/stack-bounds.json`, and the harness thread stacks of the conformance suite
    When every recursive package of those exports is matched to its row, each harness stack is compared with the largest bound times the headroom, and the rendered harness of `isClosedLeanName` is run on a stack at that product and on a quarter of its own bound
    Then every recursive package has a row, no row is stale, and every harness stack covers its bound with the headroom
    And a harness stack below a bound is refused, and the harness on a quarter of its bound aborts
