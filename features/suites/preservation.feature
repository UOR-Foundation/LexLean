Feature: preservation

  Semantic preservation from the source to the realization calculus: the lowering, certificate A, the differential evaluator, and the preservation library (§17.17).

  @SP-01 @build
  Scenario: Every production root of the committed examples lowers to a valid realization program in first-binding order, byte-identical across two lowerings, with an origin for every function and document type and exactly the root's eligibility closure; the lowering and certificate sources match no construct by default; a planted closure disagreement fails with LLI9001 and a planted default arm is refused.
    Given every production root of the committed examples
    When each is lowered twice and its program, layout, and closure are inspected, and the lowering and certificate sources are audited
    Then each program is valid, canonical, identical across the two lowerings, fully attributed, and exactly its eligibility closure
    And a report whose closure omits a member fails with LLI9001, and a default arm planted in a preservation source is refused

  @SP-02 @build
  Scenario: Every production root of examples/production, examples/production-coverage, examples/models, and examples/reasoning has a certificate whose root theorem, that the lowered program converges on the encoded arguments to the encoded source value or to overflow exactly where the width predicate fails, compiles under the pinned Lean, replays through leanchecker, and depends on exactly Classical.choice, Quot.sound, and propext; a certificate generated against a program with a planted branch, arithmetic, constructor, recursion, or literal mutation is rejected, and each such rejection lies in the declaration of the function mutated; and a root whose lowered program or certificates would exceed max_ir_nodes or max_file_bytes is refused with LLS8002 before the toolchain is touched: early, by a lower bound on each certificate from what the program states, the greatest of which is compared with the limit and none of which is above its certificate on the corpora, on families of programs that grow one dimension at a time, and on programs that grow several at once, so that a program whose certificates fit, a match on 240 constructors, a string of 450 000 characters, or both a wide match and a long string, is not refused by it (and a mixed shape that fits at its full scale of 256 is checked to fit and not edge-checked), and by a floor on certificate B from the entries its field reads print, which the lowering refuses before the Rust crate is built; during certificate B's derivation at 12 bytes a kernel node; and for every certificate by weighing the finished module in the authoring notation it is recorded in against max_file_bytes before any file is staged; the programs, crates, and certificates of all the roots of a project count together toward max_total_source_bytes, which a chain of roots calling one another passes before memory is exhausted; and a match on 180 constructors verifies through the whole pipeline.
    Given every production root of examples/production and of examples/production-coverage
    When each certificate is compiled with the pinned Lean beside the library and the generated modules, replayed, and its axioms audited
    Then every certificate is accepted and its root theorem depends on exactly the three classical axioms
    And a certificate generated against each planted program mutation is rejected inside the function mutated
    And a program past a limit is refused with LLS8002 before the toolchain, and the lower bound it refuses on is never above the largest certificate on the corpora and the stress families, so programs whose certificates fit are not refused

  @SP-03 @build
  Scenario: On seeded inputs to every production root of examples/production, examples/production-coverage, examples/models, and examples/reasoning, the calculus interpreter's outcome on the lowered program equals the certificate's observation evaluated by Lean, and a planted disagreement is detected.
    Given seeded inputs to every production root of examples/production and of examples/production-coverage
    When the interpreter runs the lowered program and Lean evaluates the certificate's observation on the same inputs
    Then every interpreter outcome equals the observation Lean prints
    And an interpreter outcome altered in one case is reported as a disagreement

  @SP-04 @build
  Scenario: Every declaration of the preservation library is a row of a native module that pins its axioms exactly and depends on exactly them under the pinned Lean, the statement vocabulary SPEC.md quotes equals what the pinned Lean prints of the generated library, the shipped calculus modules are byte-equal to the compiler project's golden modules, and a library module or certificate with a forbidden token, a disallowed option, or a foreign import is refused.
    Given the preservation library, its registry, the generated audit output, and the shipped calculus modules
    When the library is compiled and every declaration's axioms and the statement vocabulary printed, and the shipped modules are compared with the compiler golden
    Then every declaration's axioms equal its native row's policy, the quoted vocabulary equals the printed one, and every shipped module equals its golden
    And a library module with a planted forbidden token, option, or import is refused

  @SP-05 @build
  Scenario: Verification checks certificate A for every production root after named-root extraction and publishes each certificate, its audit output and process records, and a preservation.json valid against its schema whose digest the attestation binds; the lowered program and each target's crate the certificates are about are published and bound by that record, which also states the function each crate is invoked through and agrees with certificate E and the crate; a certificate the pinned Lean rejects fails with LLV7013 and a drifted preservation environment with LLV7014, before publication; and a pinned Lean that gives up on a certificate by heartbeats or recursion depth is reported as that rejection unless the module is at least a quarter of max_file_bytes, in which case, as when it is killed or out of memory, it is an exhausted resource, LLS8002.
    Given examples/production, and the fixtures that plant a mutated certificate and a drifted library under the pinned toolchain
    When each is verified
    Then the production example publishes every certificate, the program and crates, the audit output, and a schema-valid preservation.json that binds them and names each crate's entry, bound by its attestation
    And the planted certificate fails with LLV7013 and the drifted library with LLV7014, each with nothing published
    And a heartbeat verdict on a small module is a rejection and on a module a quarter of the limit or more is LLS8002

  @SP-06 @build
  Scenario: The certified roots of examples/production, examples/production-coverage, examples/models, and examples/reasoning together exercise every runtime construct of the production registry, a type parameter through an instance of a generic definition, and a construct that no certified root exercises is reported.
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
  Scenario: Certificate E composes certificates A and B: for every production root in each of its targets, a generated proof that every encoded argument is well typed for the root's parameters and the library's composition theorem establish that the rendered root, or its boundary entry when it has one, invoked on the encoded source arguments, realizes certificate A's observation of the source, as `some` when the arguments satisfy the invariants §17.12 states of them and as `none` otherwise; the pinned Lean checks and replays every composition, each end-to-end theorem depends on exactly Classical.choice, Quot.sound, and propext, a composition claiming the other result shape or another function is refused, verification fails with LLV7016 on a rejected composition, and a root verifies whatever its parameters are named, including the names the composition binds for itself, the words a certificate writes bare, the names of the namespaces the certificates begin a name with, and the tokens the certificate audit forbids, as does a declaration spelled like a word the audit of generated Lean forbids.
    Given every production root's certificates A and B in each of its targets
    When the composition is generated, checked, replayed, and audited by the pinned Lean, and restated with the other result shape and with another function
    Then every composition checks with exactly the three axioms, every restatement is refused, and verification reports a rejected composition as LLV7016
    And roots whose parameters are named like the composition's own binders or like forbidden tokens verify

  @SP-10 @build
  Scenario: A production root whose parameters can hold a map, set, or graph, directly or inside options, results, products, lists, document types, and recursive groups of them, is lowered with a generated entry function that calls the root only when the generated validators of each such parameter accept it and returns none otherwise; certificate A proves each validator decides exactly the proposition §17.12 states of its type and the entry's two outcomes; a validator is called only by the entry and by validators, so no proof-only invariant becomes a runtime check, and a validator called from inside the program is refused by the lowering; and a validator dropped from the entry, one weakened to admit an equal key, and a rendering whose entry drops a validator are each refused by Lean at the relation they change; a lowering whose entry skips a validated parameter is refused by the generator, which judges the validated parameters from the source types alone, and the differentials break each validated parameter's invariant in turn, which the interpreter, Lean, the declared machine, and rustc each refuse with none.
    Given the production roots of the examples, a recursive tree whose nodes carry a set, a list of children, a map, and an option among them, and the lowering's boundary audit
    When each root is lowered, certified, and run through its entry on valid and invalid inputs, mutations of the entry and the validators are planted, and a validator call is planted inside the program
    Then every validator decides its proposition and no validator is called from inside the program, every invalid input is refused with none, and every planted defect is refused where it was planted
    And an entry that skips a validated parameter is refused by the generator, and each validated parameter is broken in turn on every differential

  @SP-11 @build
  Scenario: Every production root of examples/production, examples/production-coverage, examples/models, and examples/reasoning is rendered in each of its targets as a package that builds under the pinned Rust toolchain, and on the seeded inputs of the differential, through the root and through its boundary entry on valid and invalid inputs, the printed outcome of each package equals the interpreter's outcome, which the declared Rust machine reproduces; that the compiler agrees with the machine the certificates are about is build evidence, never a premise of a proof.
    Given every certified root's program, its seeded inputs, and the pinned Rust toolchain
    When each root is rendered in each target as a package, built, and run through its root and its entry
    Then every package builds and every printed outcome equals the interpreter's

  @SP-12 @build
  Scenario: The preservation library is LexLean source: every module of its registry is one language-1.2 native core module, the library project holds no file of Lean source, and no Lean file anywhere in the release tree declares its namespace or lies under its module root; the registry lists only the modules, and the declarations, their exact axiom policies, the constructors of the correspondence, and the cases of its soundness theorem are read from the native data, which the compiler embeds, hashes into the language-1.2 semantics, and generates the Lean of every verification from. The one Lean text a generated library module carries beside its data is the decoder of core module 2, which loads native modules.
    Given the preservation library project, its registry, and the native modules the compiler embeds
    When the project's files are listed and the registry, the declarations, and the correspondence are read from the native data
    Then every listed module has exactly one native source and the project holds no Lean file, and no Lean file in the release tree declares the library's namespace
    And the registry names no declaration, and the constructors and soundness cases the audit uses are those of the native modules
    And a native source that is missing, extra, of the first schema, or weakened to an inexact policy is refused

  @SP-13 @build
  Scenario: Certificate text writes every list and array of any length as a term the pinned Lean elaborates without binders: one of at most 32 elements as a literal and a longer one as a chain of List.cons, so that no certificate holds a literal Lean splits under let binders; a root calling a function of 33 parameters with 33 operands, a root copying a record of 33 fields, and a root matching on 33 constructors of one field each verify through the whole pipeline, and none of their certificates writes a literal of more than 32 elements.
    Given a root calling a function of 33 parameters, a root copying a record of 33 fields, and a root matching on 33 constructors of one field
    When the certificates of each root are generated and the list literals they write are counted, and each root is verified with the pinned Lean
    Then no certificate writes a list literal of more than 32 elements
    And every root verifies, certificate B included

  @SP-14 @build
  Scenario: Certificate E of every production root is kernel-term data: a native core module of schema lexlean/core-module/2 that the compiler builds from the lowered program, the layout, and the recorded types of the preservation library's lemmas, every argument and universe level explicit, which the one core backend renders to the Lean that verification compiles and whose record is the authoring notation, reading back to the data byte for byte; its theorems state what §17.17 states for the composition, each row pins its axioms exactly and the audit module prints and checks them, a certificate held as data is audited row by row and never as text (no axiom row, no forbidden constant, imports within the environment), and a planted wrong lemma, a changed statement, a forged crate reference, or an added axiom is refused.
    Given every production root of the committed examples, the lowered programs, and the native preservation library
    When certificate E of each root is built as a core module, rendered by the core backend, recorded in the notation, and verified under the pinned Lean
    Then the record reads back to the module byte for byte, every row pins its axioms exactly, and the theorems elaborate, replay, and pass the audit
    And a wrong lemma application, a changed statement, a forged crate reference, and an added axiom are each refused

  @SP-15 @build
  Scenario: Certificate A of every production root is kernel-term data: a native core module of schema lexlean/core-module/2 that the compiler builds from the lowered program, the layout, the semantic source of the root's closure, and the recorded types of the preservation library's lemmas, every argument and universe level explicit, so that no Lean text template for certificate A remains in the production compiler; its program literal, encoders, width predicates, relation theorems, validators, and entry are rows of that module, which the one core backend renders to the Lean that verification compiles and whose record is the authoring notation, reading back to the data byte for byte; each theorem states, up to elaboration, the statement the text certificate stated, which an oracle over the committed corpora records, every row pins its axioms exactly, and a wrong lemma application, a changed statement, a forged program literal, or an added axiom is refused.
    Given every production root of the committed examples, the lowered programs, the semantic source of each root's closure, and the native preservation library
    When certificate A of each root is built as a core module, rendered by the core backend, recorded in the notation, and verified under the pinned Lean
    Then the record reads back to the module byte for byte, every row pins its axioms exactly, and the theorems elaborate, replay, and pass the audit
    And every theorem's statement is definitionally the one the committed text certificate elaborated to
    And a wrong lemma application, a changed statement, a forged program literal, and an added axiom are each refused

  @SP-16 @build
  Scenario: Certificate B of every production root is kernel-term data: a native core module of schema lexlean/core-module/2 that the compiler builds from the lowered program, the rendered crate, and the recorded types of the preservation library's correspondence rules, every argument and universe level explicit, so that no Lean text template for certificate B remains in the production compiler; its program and crate literals, flags, per-function derivations, and assembly are rows of that module, which the one core backend renders to the Lean that verification compiles and whose record is the authoring notation, reading back to the data byte for byte; each theorem states, up to elaboration, the statement the text certificate stated, which an oracle over the committed corpora records, every row pins its axioms exactly and the audit module prints and checks them, and a wrong lemma application, a changed statement, a forged program literal, an added axiom, or a row whose policy lies is refused.
    Given every production root of the committed examples, the lowered programs, the rendered crates, and the native preservation library
    When certificate B of each root is built as a core module, rendered by the core backend, recorded in the notation, and verified under the pinned Lean
    Then the record reads back to the module byte for byte, every row pins its axioms exactly, and the theorems elaborate, replay, and pass the audit
    And every theorem's statement is definitionally the one the committed text certificate elaborated to
    And a wrong lemma application, a changed statement, a forged program literal, an added axiom, and a row whose policy lies are each refused

  @SP-17 @build
  Scenario: No Lean source text template remains in the production compiler: a lexer-level scan of the Rust sources under crates/lexlean/src/production/ and of the probe and audit-module part of crates/lexlean/src/backend/lean.rs reads every string literal outside test code and finds none that is Lean syntax, which is a literal holding a Lean symbol (`:=`, `=>`, `→`, `∀`, `∃`, `⟨`, `⟩`, `(.`, `«`, `#print`, `#check`, `#eval`, `@[`, a `fun` binder), a Lean command line (`import`, `namespace`, `set_option`, `universe`, `open`), or a declaration (`theorem`, `def`, `example`, `instance`, `structure`, `inductive`, `abbrev`) with a type or a body, apart from a short allow-list of the printouts of kernel terms that developers read and the closed commands of the auxiliary module renderer; the staged audit module is an auxiliary module of typed commands that the backend renders; and a planted template in a production source, in the auxiliary module part of the backend, and in a module the scan sees as test code is each reported, the last not.
    Given the Rust sources of the production compiler and of the auxiliary module renderer of the Lean backend
    When every string literal outside test code is read by a lexer and tested for Lean syntax
    Then no literal is Lean syntax outside the allow-list, and the staged audit module is an auxiliary module of typed commands
    And a planted template in a production source and in the auxiliary module part of the backend is each reported, and a literal in test code is not
