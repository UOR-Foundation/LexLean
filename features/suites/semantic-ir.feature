Feature: semantic-ir

  Compiler phases, closed IR variants, conservative elaboration, and content identity (§17, §21).

  @SM-01 @build
  Scenario: Compiler phases execute in the required order and no backend receives an unlinked or ambiguous program.
    Given the example project with a tab and the unknown word `banana` both inserted into the theorem statement
    When lexlean check runs
    Then the first diagnostic is the normalization error LLL1002 rather than a lexical error
    And building a project with only `banana` inserted fails without creating `.lexlean/build`

  @SM-02 @build
  Scenario: Every global and local reference has one closed typed reference variant and one stable identity.
    Given the `test.defs` project with a type, term, and predicate definition plus the `add-zero` theorem
    When the linked IR is serialized to canonical JSON and every `kind` tag is collected
    Then the tags are drawn only from `core`, `external`, `document`, `defined`, the definition and theorem kinds, and the policy kinds
    And a second check of the same project reproduces the same semantic ID and identical canonical JSON bytes

  @SM-03 @build
  Scenario: Term IR contains only the specified closed variants and represents every accepted semantic term.
    Given the example project and the `test.defs` project
    When each linked IR is serialized to canonical JSON and every `k` tag is collected
    Then no term tag falls outside `sort`, `local`, `global`, `app`, `pi`, `lam`, `let`, and `nat`
    And the corpus exercises `app`, `pi`, and `global`

  @SM-04 @build
  Scenario: Proof IR contains only the specified closed variants and represents every accepted proof form.
    Given the example project and the `test.defs` project whose proofs close goals by reflexivity
    When each linked IR is serialized to canonical JSON and every proof `k` tag is collected
    Then no proof tag falls outside the closed set from `seq` and `intro` through `induction` and `calc`
    And the corpus exercises the `rfl` proof variant

  @SM-05 @build
  Scenario: Conservative signature elaboration checks arity, binders, categories, and expected types without claiming Lean kernel equivalence.
    Given the example project with the statement `\(n + 0 = n\) and \(n + 0\)` whose second conjunct is a term, not a proposition
    When lexlean check runs
    Then it fails with LLT4001 or LLP2001 from conservative elaboration
    And no Lean kernel is consulted to reach that verdict

  @SM-06 @build
  Scenario: Omitted implicit binders are recorded as controlled application metadata and user holes are rejected.
    Given the example project whose statement applies `Eq` with an omitted implicit type argument
    When the linked IR is serialized to canonical JSON
    Then the application carries an `"i":[` implicit-binder record
    And the statement rewritten as `\(n + _ = n\)` fails check with LLL1004 for the user hole

  @SM-07 @build
  Scenario: Document-entry signatures and generated declaration signatures are compared canonically before rendering.
    Given the `test.defs` project with the binder in `entries/double.toml` renamed from `n` to `renamed`
    When the project is relocked and lexlean check runs
    Then the alpha-renamed entry signature still matches the generated `double` declaration and the check succeeds

  @SM-08 @build
  Scenario: Module, component, local, and hypothesis name generation is deterministic and collision checked.
    Given the example project rendered twice
    When the Lean text for `Main` from both renders is compared
    Then the generated names are byte-identical
    And a copy with a second theorem also named `add-zero` fails check with LLP2003 or LLR3002 for the collision

  @SM-09 @build
  Scenario: Linked IR has stable schema-tagged canonical JSON with alpha-safe binder serialization.
    Given the example project and a copy with the bound variable `n` renamed to `m` throughout the statement
    When both projects are checked and the canonical key of the `add-zero` statement is computed
    Then the two alpha-renamed statements share one canonical key

  @SM-10 @build
  Scenario: Source ID and semantic ID use exactly the specified framed hash inputs.
    Given the example project checked through the internal pipeline
    When SHA-256 over `lexlean-source-v1\0` and the length-framed `project`, `lock`, `path`, and `source` inputs is recomputed by hand
    Then it equals the reported source ID
    And the semantic ID equals `semantic_id` of the compiler semantics ID, the canonical linked IR, and the closure JSON

  @SM-11 @build
  Scenario: Linked project result sets contain every selected module and imported module exactly once in stable order.
    Given the example project with a `Helper` copy of `Main` and `\importmodule{Helper}` added to `Main`
    When lexlean check runs with the entrypoint selection
    Then the result units are exactly `Helper` then `Main` in sorted order
    And the `All` selection returns the same complete sorted set

  @SM-12 @build
  Scenario: No semantic IR node can contain opaque prose, raw backend text, or an unknown extension.
    Given the example project checked through the internal pipeline
    When the linked IR is serialized to canonical JSON
    Then the JSON contains none of `natural number`, `For every`, `reflexivity`, or `Close the goal`

  @SM-13 @build
  Scenario: Inherited section parameters are represented explicitly and emitted only on declarations that use them.
    Given a `Main` module whose `basics` section has `\parameters{natural number \(p\)}` and theorems `uses-param` and `ignores-param`
    When the project is checked and each declaration's parameter list is inspected
    Then `uses-param` inherits exactly one binder
    And `ignores-param` inherits none

  @SM-14 @build
  Scenario: A numeral without a unique expected type is rejected rather than defaulted.
    Given the example project with the theorem statement `\(1 = 1\)`
    When lexlean check runs
    Then it fails with LLT4001 or LLP2002 because the numeral has no unique expected type
    And no default type is chosen for `1`

  @SM-15 @build
  Scenario: A native core module is closed typed DAG data shared by both backends, carries explicit declaration policies, and accepts no backend source text.
    Given a native core module containing a checked theorem declaration
    When lexlean check and build run
    Then the linked IR carries its typed DAG and explicit axiom policy into generated Lean and LaTeX
    And adding a raw Lean field fails closed-schema validation

  @SM-16 @build
  Scenario: The language-1.1 semantic snapshot contains every closed declaration, term, recursion, match, instance, proof variant, and exact theorem axiom policy without paths or backend text.
    Given the source-free language-1.1 semantic-module fixture
    When the public Engine snapshots it twice before and after a build
    Then the snapshot schema validates and contains structure, class, instance, inductive, recursive definition, match, cases, induction, simplify, Boolean term data, and the exact nonempty theorem axiom policy
    And all three snapshot byte strings and IDs are identical and contain no absolute path or generated Lean text

  @SM-17 @build
  Scenario: Language 1.1 has distinct mathematical Int, fixed-width signed and unsigned integer, UTF-8 string, byte-sequence, Option, and Result semantic types with canonical literals.
    Given the committed portable semantic-module fixture
    When its canonical snapshot is inspected through the public owned DTO
    Then every portable scalar and container type is present as a distinct closed value
    And every integer and byte literal has its canonical representation

  @SM-18 @build
  Scenario: Portable arithmetic, conversion, bitwise, bounded-shift, collection, UTF-8, byte-order, split/join, and decimal operations form one closed typed primitive vocabulary.
    Given the committed portable semantic-module fixture
    When LexLean checks every registered portable primitive
    Then each primitive has exactly its declared argument and result types
    And Boolean equality is available for each closed decidable portable scalar without coercion
    And no backend-specific operation or open extension is admitted

  @SM-19 @build
  Scenario: Portable semantic operations generate deterministic Lean 4.32.1 definitions that elaborate and replay with exact declared computational axiom policies.
    Given the committed portable semantic-module fixture
    When LexLean builds and verifies it on the pinned toolchain
    Then the generated runtime and module elaborate and replay
    And each computational definition reports exactly its source-declared axiom policy
    And nested records retain every large arbitrary-byte literal during elaboration and replay
    And byte literals use closed explicit octets with deterministic literal-local sharing while canonical document numbers and every byte remain unchanged

  @SM-20 @build
  Scenario: Noncanonical, out-of-range, invalid-byte, ill-typed, and unbounded fixed-width values fail before either backend runs.
    Given portable semantic modules containing one malformed value at a time
    When LexLean check runs
    Then the input fails with LLT4001 before Lean or LaTeX rendering
    And the diagnostic distinguishes syntax, range, byte spelling, and type mismatch

  @SM-21 @build
  Scenario: Structural recursion admits byte/list values and closed Option and Result inductives while preserving termination and exhaustiveness checks.
    Given a portable module with list recursion and document-defined Option and Result inductives
    When LexLean checks the module
    Then recursive calls are accepted only on a structurally smaller value
    And missing branches or a non-decreasing recursive call are rejected

  @SM-22 @build
  Scenario: The public owned snapshot DTO and schemas cover every portable type, literal, primitive, and explicit definition axiom policy without backend text.
    Given the committed portable semantic-module fixture
    When it is snapshotted in two absolute roots
    Then both schemas validate every portable variant and their bytes are identical
    And the snapshot contains no generated Lean, Rust, or absolute path

  @SM-23 @build
  Scenario: Language 1.2 semantic modules use the versioned module and snapshot schemas and accept the typed nonrecursive let term, which language 1.1 rejects before either backend runs.
    Given the committed language-1.2 example whose definition and theorem statement use the let term
    When it is checked, snapshotted, rendered, and verified, and copies are changed to language 1.1 or to the other module discriminator
    Then the 1.2 snapshot uses lexlean/semantic-snapshot/2 and validates against the v2 module and snapshot schemas
    And the generated Lean contains the fixed let lowering and verifies with real Lean
    And the language-1.1 copy, a /1 module under 1.2, and a /2 module under 1.1 fail with LLT4001 before any build root exists
    And a mistyped, shadowing, or self-referential let binder is rejected and the 1.1 snapshot keeps lexlean/semantic-snapshot/1

  @SM-24 @build
  Scenario: Language 1.2 product types, pairs, projections, and product matches are typed, snapshotted under the v2 schemas, and give identical semantic IDs from distinct roots.
    Given two copies of the committed recursive-data example in distinct absolute roots
    When both are snapshotted and checked, and copies mistype a projection, a product match, or use a product in language 1.1
    Then both snapshots and semantic IDs are identical and validate against the v2 snapshot and module schemas
    And the generated Lean uses Prod, pair syntax, projections, and a Prod.mk match
    And each mistyped form and the language-1.1 product fail with LLT4001

  @SM-25 @build
  Scenario: Language 1.2 function types, lambdas with exact explicit captures, full applications, and definition references are typed, lowered to fixed Lean, and verified.
    Given the committed higher-order example across two modules
    When it is rendered and verified, and copies change a capture list, a lambda parameter, an application, or a function reference
    Then the generated Lean uses fun, arrow types, and parenthesized applications and verifies with real Lean
    And a missing, foreign, or unsorted capture, a shadowing parameter, a wrong arity, a non-function application, a constant reference, and a language-1.1 lambda fail with LLT4001

  @SM-26 @build
  Scenario: Language 1.2 snapshots carry a deterministic alpha identity per definition that alpha-equivalent definitions share and any other change alters.
    Given two roots of the committed higher-order example and copies that rename binders or swap operands
    When each is snapshotted
    Then both roots give identical snapshots that validate against the v2 schema with one alpha identity per definition
    And renaming binders keeps the alpha identity while changing the semantic identity, swapping operands changes it, and language 1.1 snapshots carry none

  @SM-27 @build
  Scenario: Language 1.2 snapshots carry complete recursion evidence (mutual labels, decreasing arguments, measures, and evidence bindings), and changing an evidence binding changes the semantic and alpha identities.
    Given two roots of the committed recursion example and a copy that binds a different but equivalent evidence theorem
    When each is snapshotted
    Then both roots give identical v2-valid snapshots recording every mutual member, measure, and evidence binding
    And the rebinding changes the definition's alpha and semantic identities and leaves the other definitions' alpha identities unchanged

  @SM-28 @build
  Scenario: Language 1.2 finite maps and sets over closed ordered key types, their literals, and their primitive operations are typed, lowered to the fixed ordered-collection runtime, and verified.
    Given the committed collections example with a String-keyed symbol table built by a fold across two modules
    When it is rendered and verified, and copies mistype an operation result or apply a map operation to a non-map
    Then the generated Lean represents maps and sets as canonical lists, calls the fixed LexLeanCollections runtime, and verifies with real Lean
    And each mistyped use and a language-1.1 set fail with LLT4001 before any build output

  @SM-29 @build
  Scenario: Reordered equivalent map, set, and graph literals link to byte-identical semantic data and generated Lean, LaTeX, and lexicon-closure artifacts, while duplicate keys, non-literal literal keys, and key types without a canonical order are rejected.
    Given two copies of the committed collections example, one with graph nodes and edges reordered
    When both are checked and rendered, and copies duplicate a key, use a computed literal key, or key a map by Unit
    Then the source identities differ while the semantic identities, generated Lean, and canonical LaTeX are identical
    And each rejected literal fails with LLT4001 before any build output

  @SM-30 @build
  Scenario: Language 1.2 graph literals reference only declared nodes, a graph's nodes are its keys and every successor, and successor, reachability, and topological-order queries are deterministic, bounded by the node count, and report a cycle as none.
    Given the committed collections example with a call graph and a cyclic graph
    When it is rendered and verified, and copies reference an undeclared node, duplicate an edge or a node, or mistype a query result
    Then the generated graph lists every node in canonical order with sorted successors and the kernel computes the reachable set, the least-first topological order, and none for the cycle
    And each invalid graph fails with LLT4001 before any build output

  @SM-31 @build
  Scenario: Language 1.2 accepts a second native core-module schema, and only language 1.2 does, whose annotation nodes, extended names, definition safety and groups, compiled and reducibility state, and matcher, projection, no-confusion, simplification, equation-record, inlining, sparse-casesOn, and grind extension state are closed data that the linker validates against the kind of the declaration carrying it and bounds in depth by max_scope_depth, and that both backends render, and that the Lean backend's one fixed decoder submits through Lean's checked declaration API, so that the generated module elaborates under the pinned Lean with exactly its pinned axioms.
    Given a language-1.2 native core module using every second-schema construct, and copies that plant each refusable defect
    When it is linked in languages 1.0, 1.1, and 1.2 and rendered by both backends
    Then only language 1.2 accepts it, the LaTeX document and the Lean text carry every construct, and the pinned Lean elaborates the Lean with exactly the pinned axioms
    And each planted defect is refused before any build output

  @SM-32 @build
  Scenario: Language 1.2 widening conversions, to_int from every fixed-width integer type to Int and to_nat from every unsigned fixed-width type to Nat, are total, are rejected under language 1.1, and elaborate in linking to ordinary terms over bit_and, equal, conditionals, and exact integer and natural sums, so that a production root decodes a byte string to integers with no decimal text in its lowered program and is certified like any other root.
    Given the committed production-coverage example, whose roots widen a value of every fixed-width type and decode a byte string, and copies that widen a non-integer, a signed value to Nat, or state a mistyped result
    When it is linked under languages 1.1 and 1.2, rendered, lowered, and verified
    Then the generated Lean and the lowered programs contain only the elaborated terms, no decimal primitive, and the kernel decides the widening of every width at its bounds
    And each misuse fails with LLT4001 before any build output

  @SM-33 @build
  Scenario: A version 2 core-module row whose name the pinned Lean reserves is not submitted as data: the decoder runs the pinned Lean's own action for the name and refuses the module when the action does not make the name or makes a constant whose kind, levels, type, or value differ from the row's.
    Given a language-1.2 native core module with a definition and the equation theorem the pinned Lean reserves for it
    When it is verified, and copies give the theorem another proof of the same statement or a reserved name the construction does not make
    Then the pinned Lean verifies the module whose theorem is the one its own action makes
    And each copy fails with LLV7002, naming the row the construction does not make as stated
