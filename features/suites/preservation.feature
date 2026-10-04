Feature: preservation

  Semantic preservation from the source to the realization calculus: the lowering, certificate A, the differential evaluator, and the preservation library (§17.17).

  @SP-01 @build
  Scenario: Every production root of the committed examples lowers to a valid realization program in first-binding order, byte-identical across two lowerings, with an origin for every function and document type and exactly the root's eligibility closure; the lowering and certificate sources match no construct by default; a planted closure disagreement fails with LLI9001 and a planted default arm is refused.
    Given every production root of the committed examples
    When each is lowered twice and its program, layout, and closure are inspected, and the lowering and certificate sources are audited
    Then each program is valid, canonical, identical across the two lowerings, fully attributed, and exactly its eligibility closure
    And a report whose closure omits a member fails with LLI9001, and a default arm planted in a preservation source is refused

  @SP-02 @build
  Scenario: Every production root of examples/production, examples/production-coverage, and examples/models has a certificate whose root theorem, that the lowered program converges on the encoded arguments to the encoded source value or to overflow exactly where the width predicate fails, compiles under the pinned Lean, replays through leanchecker, and depends on exactly Classical.choice, Quot.sound, and propext; a certificate generated against a program with a planted branch, arithmetic, constructor, recursion, or literal mutation is rejected, and each such rejection lies in the declaration of the function mutated; and a root whose lowered program or certificates would exceed max_ir_nodes or max_file_bytes is refused with LLS8002 before the toolchain is touched, certificates A and E by an estimate that is never below their size on the corpora and on families of programs that grow one dimension at a time, and certificate B by a floor on the entries its field reads print, which the lowering refuses before the Rust crate is built, and by being generated under the limit, which stops it at the limit.
    Given every production root of examples/production and of examples/production-coverage
    When each certificate is compiled with the pinned Lean beside the library and the generated modules, replayed, and its axioms audited
    Then every certificate is accepted and its root theorem depends on exactly the three classical axioms
    And a certificate generated against each planted program mutation is rejected inside the function mutated
    And a program past a limit is refused with LLS8002 before the toolchain, and the estimate is never below certificates A and E on the corpora and the stress families

  @SP-03 @build
  Scenario: On seeded inputs to every production root of examples/production, examples/production-coverage, and examples/models, the calculus interpreter's outcome on the lowered program equals the certificate's observation evaluated by Lean, and a planted disagreement is detected.
    Given seeded inputs to every production root of examples/production and of examples/production-coverage
    When the interpreter runs the lowered program and Lean evaluates the certificate's observation on the same inputs
    Then every interpreter outcome equals the observation Lean prints
    And an interpreter outcome altered in one case is reported as a disagreement

  @SP-04 @build
  Scenario: Every declaration of the preservation library is registered in library.toml and depends on exactly the axioms it registers, the statement vocabulary SPEC.md quotes equals the library's declarations byte for byte, the shipped calculus modules are byte-equal to the compiler project's golden modules, and a library module or certificate with a forbidden token, a disallowed option, or a foreign import is refused.
    Given the preservation library, its registry, and the shipped calculus modules
    When the library is compiled and every declaration's axioms printed, and the shipped modules are compared with the compiler golden
    Then every declaration's axioms equal the registry and every shipped module equals its golden
    And a library module with a planted forbidden token, option, or import is refused

  @SP-05 @build
  Scenario: Verification checks certificate A for every production root after named-root extraction and publishes each certificate, its audit output and process records, and a preservation.json valid against its schema whose digest the attestation binds; the lowered program and each target's crate the certificates are about are published and bound by that record, which also states the function each crate is invoked through and agrees with certificate E and the crate; a certificate the pinned Lean rejects fails with LLV7013 and a drifted preservation environment with LLV7014, before publication; and a pinned Lean that gives up on a certificate by heartbeats or recursion depth is reported as that rejection unless the module is at least a quarter of max_file_bytes, in which case, as when it is killed or out of memory, it is an exhausted resource, LLS8002.
    Given examples/production, and the fixtures that plant a mutated certificate and a drifted library under the pinned toolchain
    When each is verified
    Then the production example publishes every certificate, the program and crates, the audit output, and a schema-valid preservation.json that binds them and names each crate's entry, bound by its attestation
    And the planted certificate fails with LLV7013 and the drifted library with LLV7014, each with nothing published
    And a heartbeat verdict on a small module is a rejection and on a module a quarter of the limit or more is LLS8002

  @SP-06 @build
  Scenario: The certified roots of examples/production, examples/production-coverage, and examples/models together exercise every runtime construct of the production registry, a type parameter through an instance of a generic definition, and a construct that no certified root exercises is reported.
    Given the eligibility reports of every certified root and the runtime rows of the production registry
    When the constructs the roots realize are collected, with a type parameter counted where a generic definition is instantiated
    Then every runtime row is exercised by some certified root
    And with the coverage example's collection roots withheld, their constructs are reported as unexercised

  @SP-07 @build
  Scenario: The declared Rust machine is generated LexLean: RustSyntax states every construct of the closed Rust AST and RustSemantics its evaluator over calculus values, a `?` on an error raising out of its function, and each runtime item as the calculus primitive it realizes at its width and in its profile; both are kernel-checked modules of the compiler project with exact axioms whose shipped copies equal the compiler golden, the runtime items' failure and heap classes equal the renderer's, and the term of every certified root's crate elaborates against RustSyntax.
    Given the generated RustSyntax and RustSemantics modules, the runtime item table, and the crate of every certified root
    When the compiler project is verified, the shipped copies are compared with its golden, the item classes with the renderer, and each crate term is elaborated
    Then the modules verify with exact axioms, the copies and classes agree, and every crate term elaborates

  @SP-08 @build
  Scenario: Certificate B relates every rendering to its program: for every production root in each of its targets and every renderer fixture in each profile that renders it, the aligner derives the shipped library's correspondence between the lowered program and its crate from a closed rule set, whose rules are exactly the correspondence's constructors, each a case of the library's soundness theorem and used by some rendering, and which names every calculus construct; the pinned Lean checks and replays every derivation, each simulation theorem depends on exactly Classical.choice, Quot.sound, and propext, and a crate mutated after rendering is refused.
    Given every production root's crate in each target and every renderer fixture's crate in each profile
    When the aligner derives each crate's correspondence and the pinned Lean checks, replays, and audits it
    Then every rule the aligner emits is a constructor of the correspondence and a case of its soundness theorem, every rule is used and every calculus construct named, every derivation checks, every simulation theorem has exactly the three axioms, and every mutated crate is refused

  @SP-09 @build
  Scenario: Certificate E composes certificates A and B: for every production root in each of its targets, a generated proof that every encoded argument is well typed for the root's parameters and the library's composition theorem establish that the rendered root, or its boundary entry when it has one, invoked on the encoded source arguments, realizes certificate A's observation of the source, as `some` when the arguments satisfy the invariants §17.12 states of them and as `none` otherwise; the pinned Lean checks and replays every composition, each end-to-end theorem depends on exactly Classical.choice, Quot.sound, and propext, a composition claiming the other result shape or another function is refused, and verification fails with LLV7016 on a rejected composition.
    Given every production root's certificates A and B in each of its targets
    When the composition is generated, checked, replayed, and audited by the pinned Lean, and restated with the other result shape and with another function
    Then every composition checks with exactly the three axioms, every restatement is refused, and verification reports a rejected composition as LLV7016

  @SP-10 @build
  Scenario: A production root whose parameters can hold a map, set, or graph, directly or inside options, results, products, lists, document types, and recursive groups of them, is lowered with a generated entry function that calls the root only when the generated validators of each such parameter accept it and returns none otherwise; certificate A proves each validator decides exactly the proposition §17.12 states of its type and the entry's two outcomes; a validator is called only by the entry and by validators, so no proof-only invariant becomes a runtime check, and a validator called from inside the program is refused by the lowering; and a validator dropped from the entry, one weakened to admit an equal key, and a rendering whose entry drops a validator are each refused by Lean at the relation they change; a lowering whose entry skips a validated parameter is refused by the generator, which judges the validated parameters from the source types alone, and the differentials break each validated parameter's invariant in turn, which the interpreter, Lean, the declared machine, and rustc each refuse with none.
    Given the production roots of the examples, a recursive tree whose nodes carry a set, a list of children, a map, and an option among them, and the lowering's boundary audit
    When each root is lowered, certified, and run through its entry on valid and invalid inputs, mutations of the entry and the validators are planted, and a validator call is planted inside the program
    Then every validator decides its proposition and no validator is called from inside the program, every invalid input is refused with none, and every planted defect is refused where it was planted
    And an entry that skips a validated parameter is refused by the generator, and each validated parameter is broken in turn on every differential

  @SP-11 @build
  Scenario: Every production root of examples/production, examples/production-coverage, and examples/models is rendered in each of its targets as a package that builds under the pinned Rust toolchain, and on the seeded inputs of the differential, through the root and through its boundary entry on valid and invalid inputs, the printed outcome of each package equals the interpreter's outcome, which the declared Rust machine reproduces; that the compiler agrees with the machine the certificates are about is build evidence, never a premise of a proof.
    Given every certified root's program, its seeded inputs, and the pinned Rust toolchain
    When each root is rendered in each target as a package, built, and run through its root and its entry
    Then every package builds and every printed outcome equals the interpreter's
