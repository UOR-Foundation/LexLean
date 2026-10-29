Feature: lexicon

  Lexicon packages, entries, forms, denotations, LSE, LRE, and renderer tokens (§13, §16.11).

  @GL-01 @build
  Scenario: Lexicon packages obey the exact package layout, schema, ID-to-path rule, and exact imports.
    Given a test.pkg package whose entry probe lives in elsewhere.toml, and another test.pkg importing lexlean.core@2.0.0
    When lock runs on each
    Then the misplaced entry fails with LLR3004
    And the import at an unavailable exact version fails with LLR3001 or LLR3004

  @GL-02 @build
  Scenario: Entry files obey the exact entry schema and category-specific field rules.
    Given test.pkg entries adding `mystery = true`, setting category to `noun`, and setting category to `grammar`
    When lock runs on each
    Then the unknown field fails with LLR3004
    And the unknown category fails with LLR3004
    And the core-only grammar category fails with LLR3004

  @GL-03 @build
  Scenario: Forms obey channel, feature, canonical-source, safety, and explicit-inflection requirements.
    Given test.pkg entries with surface ` probe`, with features `["sideways"]`, and with `canonical_source = false` on the only form
    When lock runs on each
    Then the padded surface fails with LLR3004
    And the unknown feature fails with LLR3004
    And the entry without a canonical form fails with LLR3004

  @GL-04 @build
  Scenario: Every entry uses one fixed frame and packages cannot add grammar productions.
    Given test.pkg entries with `frame = "macro"` and with `frame = "call"` on a zero-arity constant
    When lock runs on each
    Then the unknown frame fails with LLR3004
    And the mismatched frame fails with LLR3004

  @GL-05 @build
  Scenario: Denotations are exactly core, Lean, document, or acyclic defined values.
    Given test.pkg entries whose denotation is `kind = "magic"` and `kind = "core"` with constructor logic.eq
    When lock runs on each
    Then the unknown denotation kind fails with LLR3004
    And the core denotation outside lexlean.core fails with LLR3004

  @GL-06 @build
  Scenario: Every semantic entry has a valid canonical LSE signature with scoped binders and universes.
    Given test.pkg entries with signature `(local ghost)` and with the signature row deleted
    When lock runs on each
    Then the unbound local fails with LLR3004
    And the missing signature fails with LLR3004

  @GL-07 @build
  Scenario: Every canonical render uses valid LRE with complete slot use and no raw TeX.
    Given a test.pkg entry whose math render is `(slot 0)` on a zero-arity entry
    When lock runs
    Then the out-of-range slot fails with LLR3004

  @GL-08 @build
  Scenario: Only the core renderer-token registry can authorize emitted LaTeX controls and glyphs.
    Given a test.pkg entry whose math render is `(token unregistered-token)` and the committed language/renderer-tokens.toml
    When the project is relocked and checked and the registry rows are counted
    Then check fails with LLR3004
    And the registry has at least 70 `[[token]]` rows

  @GL-09 @build
  Scenario: Package import cycles and excessive import depth are rejected.
    Given packages test.cyca and test.cycb importing each other, and a copy with `max_import_depth = 3` and a six-package chain test.chain1 through test.chain6
    When each project is relocked and checked
    Then the import cycle fails with LLR3003
    And the chain deeper than 3 fails with LLS8002 or LLR3003

  @GL-10 @build
  Scenario: Defined-denotation cycles and document-definition cycles are rejected.
    Given test.pkg entries cyca and cycb whose defined denotations reference each other as `(const test.pkg::...)`
    When the project is relocked and checked
    Then check fails with LLR3003

  @GL-11 @build
  Scenario: A document denotation must resolve to an available declaration with a matching signature.
    Given the defs fixture project and a copy whose double.toml component is renamed to `elsewhere`
    When both are checked after relocking the copy
    Then the intact defs project checks cleanly
    And the mismatched document denotation fails with LLF5001

  @GL-12 @build
  Scenario: Every used external Lean entry is checked by a generated interface probe during verification.
    Given the verified example fixture and its CheckedProject external_used map
    When the published .lean probe module under the outcome probe directory is read
    Then external_used is non-empty
    And the probe text contains the lean_name of every used external entry

  @GL-13 @build
  Scenario: Duplicate packages, entries, forms, and qualified IDs are rejected while same-surface overloads remain explicit candidates.
    Given a lexlean.toml listing test.pkg twice, a test.pkg entry with two forms both id probe, and two packages test.dupa and test.dupb both defining nzz
    When Engine::load, lock, and relock plus check run respectively
    Then the duplicate package row fails with LLR3002 or LLC0101
    And the duplicated form id fails lock with LLR3004
    And the same-surface overloads lock and check cleanly

  @GL-14 @build
  Scenario: Cases and induction are available only through a complete validated eliminator descriptor.
    Given language/std/nat/entries/nat.toml and a test.pkg copy of it renamed gadget with its succ constructor row pointed at lexlean.std.nat::zero
    When the nat entry is inspected and lock runs on the copy
    Then nat.toml carries `[eliminator]` naming Nat.rec and Nat.casesOn
    And the duplicated constructor descriptor fails with LLR3004

  @GL-15 @build
  Scenario: Glossary files reject free description, documentation, note, meaning, and unknown prose fields.
    Given four test.pkg entries each adding one of `description`, `documentation`, `note`, or `meaning` set to `free prose`
    When lock runs on each
    Then every prose field fails with LLR3004

  @GL-16 @build
  Scenario: Package and entry bytes participate in lock and semantic closure hashes exactly as specified.
    Given the example project using test.pkg@1.0.0 with entry probe.toml, relocked
    When a trailing newline is appended to probe.toml and the project relocks, then surface `probe` becomes `probed` and it relocks again
    Then the lexlean.lock bytes differ after the byte-only change
    And the semantic_id differs after the surface change

  @GL-17 @build
  Scenario: Language 1.2 resolves the exact 1.2 builtin package closure and enforces 1.2 lexicon semantics.
    Given a language 1.2 project initialized with builtin lexlean.std.nat, and the builtin packages of every language
    When lexlean.lock is generated, the 1.2 bootstrap is loaded, and every builtin manifest and entry file is read
    Then the lock contains `lexlean.core`, `lexlean.std.bool`, and `lexlean.std.nat` all at version 1.2.0 and the bootstrap is language 1.2
    And every builtin manifest validates against its language's lexicon schema with its own language, version, and import versions
    And every 1.2 package has the 1.1 package's file set with byte-identical entries, differing only in its manifest

  @GL-18 @build
  Scenario: Cross-version package, lexicon, and lock combinations fail closed before backend execution.
    Given a language 1.1 project importing a language 1.2 lexicon package, a language 1.2 project importing a 1.1 package, and a 1.0 lock tampered to `lexlean/lock/2` and language 1.2
    When lock, check, `lock --check`, and build run on each project
    Then each package mismatch fails lock with LLC0103 naming both languages
    And the tampered lock fails check and `lock --check` with LLC0102
    And build refuses every project and no build or verification root exists

  @GL-19 @build
  Scenario: A generated external-Lean glossary is a lexicon package of kind-lean entries that the package loader accepts: each signature is the LSE translation of the Lean type with implicit and instance binders kept as binders and left out of applications, each identifier, surface, category, and render follows the documented naming scheme, an inductive type carries an eliminator descriptor over its constructor entries, and two runs over one extraction produce byte-identical packages and reports.
    Given the committed extraction of a Lean module holding theorems, definitions, a class, a recursive inductive type, and predicates of each arity, equal to a fresh extraction when the pinned Lean is available
    When the glossary planner turns it into a package twice and the package loader reads the result
    Then the loader accepts the package and every entry denotes a Lean constant by module and name
    And the entry for the theorem about the class keeps its instance binder, applies the class method to its explicit arguments only, and has the identifier `fix.scale-zero`, the surface `fixScaleZero`, and the category its type's shape gives it
    And the entry for the recursive inductive type carries an eliminator whose constructors are the entries of its two constructors with their fields and induction hypotheses
    And both runs are byte-identical, package and report

  @GL-20 @build
  Scenario: A Lean declaration the translation cannot express is refused with the registered diagnostic code of its reason and never emitted with a weakened signature, a declaration that depends on a refused one is refused with the root reason, and the coverage report counts requested declarations as translated, refused by reason, and excluded by kind, with counts that sum to the request.
    Given the committed extraction, which holds a Sort-polymorphic declaration, a strict-implicit binder, a projection, a string literal, an equation at Sort 0, a name outside the conservative grammar, and a theorem about the Sort-polymorphic declaration, and an extraction whose entry fails the signature check
    When the glossary planner runs and the strict mode converts the refusals to an error
    Then each declaration is absent from the package and carries the registered code of its reason in the report
    And the dependent theorem is refused with the code of the root reason and names the declaration it depends on
    And the report counts sum to the number of requested declarations
    And the strict mode fails with the registered code and its class exit

  @GL-21 @build
  Scenario: A specialisation request names a class-polymorphic Lean declaration, its kind (`theorem` or `def`), a tag, and the arguments of its application at a concrete type, each a closed term of identifiers, numerals, holes, and applications; the wrapper emitter writes one `public` declaration for each under the reserved root `LexLeanSpec`, whose type is `type_of%` the application and whose value is the application, with every constant `_root_`-qualified and no tactic and no axiom, in sorted order and byte for byte the same on every run, and a request with a duplicate tag, a name outside the conservative grammar, or an argument outside the term grammar is refused with LLT4018.
    Given the specialisation request of tests/fixtures/specialisation for Std.Commutative.comm at Nat, Int, UInt32 and BitVec 32
    When the wrapper emitter runs twice on the request
    Then both runs write the committed host module byte for byte
    And the host module holds one public declaration per wrapper under LexLeanSpec and no tactic, by, or axiom command
    And a request with a duplicate tag, a tactic argument, or an invalid name is refused with LLT4018
