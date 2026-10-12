Feature: tooling

  Compiler-tooling modules: a closed, typed program over a registry of the external Lean interface (§17.20).

  @TL-01 @build
  Scenario: A tooling module is the one `toolingmodule` environment of its module, holding one canonical `\toolingdata` argument of the closed `lexlean/tooling-module/1` schema, and every committed fixture satisfies `tooling/schemas/tooling-module.schema.json`; a member, declaration, term, signature, or atom outside it, a member written with its default, a spec tag but the exact one, a value that is not its own canonical encoding, a repeated member, a value nested deeper than the decoder reads, and an unsorted import list fail with LLT4013, a module over `max_ir_nodes` fails with LLS8002, a second data argument or a second module form in the same module is a structural error that fails with LLP2003, and no change of any single node of a committed fixture panics the compiler, which accepts or refuses it with LLT4013, LLT4014, or LLT4015.
    Given the committed `core`, `commands`, and `traversal` tooling fixtures
    When each module's data is validated against the tooling-module schema and decoded by the compiler
    Then every fixture is schema-valid and decodes to exactly the canonical bytes it was written as
    And each planted schema defect (an unknown member, a written default, another spec tag) fails the schema and fails with LLT4013 alone, and a non-canonical encoding, a repeated member, a value nested 200 levels, and an unsorted import list fail with LLT4013 while a re-indented value is accepted
    And a module over `max_ir_nodes` fails with LLS8002, a second data argument and a second module form fail with LLP2003, and every single-node change of a fixture is accepted or refused by a registered code without a panic

  @TL-02 @build
  Scenario: A tooling module requires language 1.2 and a project the compiler itself embeds: a language-1.1 project, a user project, the check, build, and verify requests of the public API, and the command line refuse it with LLT4013 before either backend runs, no configuration member or option admits one, and a tooling module has no production root, no production eligibility report, and no semantic snapshot.
    Given the `core` fixture as a user project, as a language-1.1 project, and with a configuration member that would admit it
    When the library API and the command line check, build, and verify it
    Then each refuses it with LLT4013 before a backend runs and the configuration member is rejected
    And the embedded-project check of the fixture yields no production root, no eligibility report, and no semantic snapshot

  @TL-03 @build
  Scenario: A tooling module cites one authority, Lean at the compiler's pinned release and source commit, which equal the pins of `model/authorities.toml`, and its registry is closed: an authority that is not the pin, a name declared twice, a signature or field type naming a constant that no registry row declares, a variable a signature does not bind, an empty, unsorted, or repeated plumbing row, a plumbing constant that is also declared, and a source path outside the toolchain source or a digest that is not 64 lowercase hexadecimal digits fail with LLT4014, and every source digest equals the SHA-256 of that file in the pinned toolchain.
    Given the three fixtures and the authority pins of the model and of the extraction data
    When the authority, a name, a signature, a plumbing row, and a source pin are each planted with one defect
    Then each defect fails with LLT4014 naming it
    And the compiler's pin equals both authority pins, and every source digest of the fixtures equals the digest of that file in the pinned toolchain

  @TL-04 @build
  Scenario: A registered type states its parameters (a type, a type constructor, or an index drawn from a registered type whose constructors are all nullary), its complete constructor list in Lean's order with typed fields, and the one index a constructor exists at, an opaque type pins no constructors, and an abbreviation is the type it abbreviates wherever types are compared; an index of an unregistered or non-nullary type, a repeated constructor, a field naming an unregistered type, applying a type to the wrong number of arguments, or using a variable no parameter binds, a requirement that names no index parameter or a constructor of another type, an opaque type with constructors, and an abbreviation of itself or of a cycle fail with LLT4014.
    Given the fixtures' registered types, among them the `Purity` index, the index-constrained `Code` constructors, an opaque type, and the `CommandElab` abbreviation
    When each type rule is planted with one defect
    Then each defect fails with LLT4014 naming it
    And the unplanted fixtures check

  @TL-05 @build
  Scenario: A call's exact signature (constants, variables, applications, arrows, explicit, implicit, and instance binders, `Type` and `Type u`, `optParam` defaults, natural literals, and the empty structure instance) is its typing: the implicit binders of types and type constructors are the type arguments every use states, any other implicit binder and every instance binder is inferred by Lean and never stated, trailing binders with a default are optional, and types are compared exactly after the unfolding of abbreviations; a use with a wrong number of type arguments or arguments, an argument or result of another type, or an unregistered callee fails with LLT4015.
    Given the fixtures' calls, among them implicit and instance binders, a trailing `optParam` default, and the empty structure instance default
    When each call is used with one planted typing defect
    Then each use fails with LLT4015 naming it
    And the unplanted uses, with a default omitted and given, check

  @TL-06 @build
  Scenario: A tool is written in a closed typed term language with explicit binder types and type arguments, and its `match` is flat, one constructor per branch, and exhaustive at the scrutinee's index; a constructor available at the index without a branch, a branch for a constructor of another type or one that exists only at another index, a branch binding the wrong number of fields, a field of a type with several constructors, a condition that is not Bool, an unbound variable, a binder that shadows a variable in scope, and an ill-typed let, lambda, application, list, array, or constructor fail with LLT4015.
    Given the fixtures' terms, among them every term form and a match over an index-constrained family
    When each term rule is planted with one defect
    Then each defect fails with LLT4015 naming it
    And the unplanted terms check

  @TL-07 @build
  Scenario: A function calls only registered calls, functions defined before it, itself when it is partial, and the functions of its mutual group, a contiguous run of partial functions with one label; a self call of a function that is not partial, a forward call, a mutual group that is not one run, and a group with a member that is not partial fail with LLT4015.
    Given the `core` fixture's recursive function and mutual group
    When each recursion licence is planted with one defect
    Then each defect fails with LLT4015 naming it
    And the licensed recursion checks

  @TL-08 @build
  Scenario: A traversal is a partial function that takes its subject first and is exactly a match on it, its group handles every type of the subject's recursive family, and each branch uses every field whose type lies in the family through the traversal of that type applied to a value derived from the field, so that a skipped field, a handler applied to the subject instead of the field, a handler that is not a traversal, a default branch, and a constructor that a registry gains and no branch handles fail with LLT4015, while a traversal of the real Lean `Code` family elaborates under the pinned Lean.
    Given the `traversal` fixture's mutual traversal of the real LCNF `Code`, `FunDecl`, `Cases`, and `Alt` family
    When a field, a handler, a branch, the handler's argument, and a registry constructor or field are each planted with one defect
    Then each defect fails with LLT4015 naming it
    And the pinned Lean elaborates the unplanted traversal without a message

  @TL-09 @build
  Scenario: A command declares its syntax kind, a grammar of a leading token, identifiers, and signature types, and an elaborator with no parameters whose type is the abbreviation `Lean.Elab.Command.CommandElab`, an invocation supplies one identifier or signature tree per non-token atom, and an evaluation runs a function of the module on typed arguments, all as semantic data; an elaborator with a parameter or another type, a grammar without a leading token, an invocation of an unregistered command or with a wrong argument count or kind, and an evaluation of an unregistered function or with ill-typed arguments fail with LLT4015, and the pinned Lean runs the committed elaborator, which accepts a constant that exists and refuses one that does not.
    Given the `commands` fixture's registered command, its elaborator, its invocation, and its evaluation
    When each command rule is planted with one defect
    Then each defect fails with LLT4015 naming it
    And the pinned Lean runs the elaborator on an existing constant and refuses a missing one

  @TL-10 @build
  Scenario: Both backends traverse the one linked module: the Lean text begins with the fixed header, holds one rendering per function, command, invocation, and evaluation, and contains no comment, every declaration with output has one source-mapped piece, coverage is complete, the LaTeX document has one entry per declaration, and the pinned Lean elaborates every committed fixture without a message while refusing a registry that misstates an arity or a constructor list that the tool is consistent with.
    Given the rendered Lean and LaTeX of each fixture
    When their text, source maps, and coverage are read and the pinned Lean elaborates the Lean
    Then the text has the fixed header, one rendering per declaration, and no comment, and every mapped piece and the coverage are complete
    And Lean elaborates each fixture without a message and refuses a planted arity or constructor misstatement

  @TL-11 @build
  Scenario: Tooling modules are deterministic and content-addressed: each committed fixture's published build tree equals its golden byte for byte and across two clean builds in different directories, changing any declaration, registry row, source pin, or term changes the semantic ID, and the schema lies outside `schemas/` and so outside the compiler-semantics ID, which the committed locks of the three languages still record.
    Given the committed goldens of each fixture and two copies of it in different directories
    When each is built
    Then every published tree equals its golden and the other copy's byte for byte
    And changing a declaration, a registry row, a source pin, or a term changes the semantic ID, and the schema is outside the compiler-semantics ID that the locks of all three languages record
