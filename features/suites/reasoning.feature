Feature: reasoning

  Language-1.2 reasoning machines: logics, inference rules, verifiers, reasoners, bounded search, termination evidence, traces, and their production realization (§17.12).

  @RS-01 @build
  Scenario: Language 1.2 logic, inference_rule, verifier, and reasoner declarations and the reasoning_failure type belong to the closed lexlean/semantic-module/2 schema and its snapshot schema, are rejected under language 1.1 before either backend runs, and admit no member or strategy outside the closed schema, such as an opaque reasoner, a prompt, or a randomized or unbounded search, which fail with LLT4001.
    Given a language-1.2 module with every reasoning construct
    When it is linked under language 1.2 and again under language 1.1, and with an opaque member or an unregistered strategy added
    Then language 1.2 links it and its snapshot validates against the closed schemas
    And language 1.1 rejects every reasoning construct and the closed schema rejects the extra member and the strategy with LLT4001

  @RS-02 @build
  Scenario: A logic names a prior state relation and an optional invariant with a statement-exact preservation theorem and an optional ranking, each of exactly its state type, and an inference rule over a logic is a typed guard, conclusion, and optional binding with a candidate list whose soundness theorem states exactly that the guard implies the relation and whose progress theorem states exactly that the ranking decreases; a predicate of the wrong signature, an inexact theorem, or a rule over another logic than its reasoner's fails with LLT4010.
    Given logics and inference rules with soundness and progress theorems
    When they are linked and every theorem is compared with its generated statement
    Then each rule elaborates to its guard, conclusion, candidates, and guarded application with a generated soundness theorem
    And a predicate of the wrong signature, an inexact theorem, or a rule over a foreign logic fails with LLT4010

  @RS-03 @build
  Scenario: A verifier links an executable Boolean check over a subject and a candidate to a prior proposition-valued specification by statement-exact soundness and optional completeness theorems that Lean restates against the fixed reasoning semantics; an inexact statement or a verifier whose subject or candidate type differs from its reasoner's fails with LLT4010, and a false soundness theorem is refused by verification with LLV7002.
    Given verifiers with soundness and completeness theorems
    When they are linked, bound to reasoners, and verified
    Then Lean restates each theorem against the fixed reasoning semantics
    And an inexact statement or a type mismatch fails with LLT4010 and a false soundness theorem is refused with LLV7002

  @RS-04 @build
  Scenario: Every reasoner elaborates in linking to ordinary language-1.2 inductives, structures, definitions, and theorems over the existing match, if, list_fold, iterate_until, and set operations, with no reasoner primitive and no model-based semantic escape hatch, and the formal-only LexLeanReasoning runtime appears in no production closure.
    Given forward and search reasoners
    When they are linked and their elaborations are inspected
    Then each is exactly ordinary declarations over the existing primitives, checked again by the ordinary rules
    And no generated declaration names a reasoning primitive and no production closure reaches LexLeanReasoning

  @RS-05 @build
  Scenario: A forward reasoner fires, at each of at most its fuel iterations, the first applicable rule in declared priority order and candidate order, re-checking the rule's guard at every application, and its verdict is the verifier-accepted answer of the final state or exactly one of the closed failures exhausted, unsolved, or rejected.
    Given a forward reasoner over binding-free and binding rules
    When its elaboration runs on inputs that saturate, exhaust their fuel, and yield rejected and missing answers
    Then each step fires the first applicable rule in priority and candidate order through its guarded application
    And the verdict is the verified answer or exactly exhausted, unsolved, or rejected, as the kernel decides

  @RS-06 @build
  Scenario: A search reasoner explores states breadth-first or depth-first under explicit natural-number fuel and frontier bounds, with optional deduplication over an ordered state type, and accepts only a state whose answer its verifier's check accepts; a missing or non-natural bound, a literal zero frontier, or deduplication over an unordered state fails with LLT4011, and a search cut short by a bound yields exhausted and never unsolved.
    Given breadth-first and depth-first search reasoners with and without deduplication
    When they run under their fuel and frontier bounds and are linked with missing or ill-typed bounds
    Then each accepts only a verified answer and reports a bound cut as exhausted, as the kernel decides
    And a missing or non-natural bound, a literal zero frontier, or deduplication over an unordered state fails with LLT4011

  @RS-07 @build
  Scenario: A terminates claim requires a ranking, a statement-exact progress theorem on every rule, an initial-invariant claim when progress assumes the invariant, and a statement-exact fuel-bound theorem, and generates a kernel-checked theorem that the forward reasoner saturates within its fuel; a claim without that evidence fails with LLT4011, and a false progress theorem for a rule that undoes another is refused by verification with LLV7002.
    Given forward reasoners claiming termination
    When their claims are linked and verified
    Then Lean proves that each saturates within its fuel from the ranking and the progress and fuel-bound theorems
    And a claim without a ranking, progress, or fuel bound fails with LLT4011 and a false progress theorem is refused with LLV7002

  @RS-08 @build
  Scenario: Each logic, rule, verifier, and reasoner generates fixed-template theorems over its elaboration, among them guarded-application soundness, trace replay, derivation, invariant preservation, iteration and frontier bounds, saturation, and answer soundness, whose proofs apply only the emitted, axiom-free LexLeanReasoning runtime and the declaration's own theorems; every runtime lemma and every template is used by the committed example and accepted by pinned Lean.
    Given the reasoning example's generated theorems
    When the example is verified and its generated declarations are compared with the emitted runtime
    Then every runtime lemma and every template is produced and accepted by pinned Lean under its owner's axiom policy
    And a mutated runtime lemma statement is refused by verification

  @RS-09 @build
  Scenario: Executable code reaches a rule's conclusion only through the rule's guarded application and a state's answer only through its reasoner's verifier, otherwise failing with LLT4012, and a trace is evidence only by replay: a forged trace or an inapplicable rule application yields invalid_step at run time and the kernel decides that it does.
    Given executable code using rules and reasoners
    When it calls a conclusion or an unverified answer directly, and replays forged and genuine traces
    Then a direct conclusion or unverified answer fails with LLT4012
    And a forged trace or an inapplicable application replays to invalid_step and a genuine trace replays to the reasoner's state, as the kernel decides

  @RS-10 @build
  Scenario: Reasoning declarations and their elaborations are part of the semantic ID and charged to max_ir_nodes; a language-1.2 snapshot carries each elaboration with its generated theorems, and the canonical document renders a closed catalog of interfaces, rules, strategy, bounds, claims, and generated obligations that contains no trace value and never says verified.
    Given the reasoning example
    When its snapshot, identities, and canonical document are generated
    Then each elaboration and generated theorem is in the snapshot and the semantic ID, and its nodes are charged
    And the document lists the closed catalog with no trace value and no verification claim

  @RS-11 @build
  Scenario: Production eligibility realizes inference rules and reasoners through their elaborations and erases logics and verifiers, the realization table maps every reasoning row to calculus elements, the eligibility report records each reasoning root's strategy, bounds, rule order, and ledger, and the example's rust-core and rust-std reasoning roots extract through Lean.
    Given the reasoning example's production roots
    When eligibility, realization, and extraction run over them
    Then rules and reasoners are realized through their elaborations and logics and verifiers are erased
    And each root's report records its reasoning resources and extraction succeeds for rust-core and rust-std

  @RS-12 @build
  Scenario: The compiler project's reasoning oracle declares exactly the clinical declarations of the reasoning example, and calculus transcriptions of its forward engine produce the oracle's verdicts on every fixture argument as the kernel decides, compile to committed rust-core and rust-std packages, and charge at least the iterations and firings of the reasoner's ledger in calculus steps.
    Given the compiler project's reasoning oracle and transcriptions
    When the transcriptions are evaluated, compared with the oracle, and rendered to Rust
    Then every outcome equals the oracle's as the kernel decides and the packages equal their generator
    And every transcription charges at least the ledger's iterations and firings in steps

  @RS-13 @build
  Scenario: GNAF requests over the forward-engine transcriptions charge rule search as execution in calculus steps and are answered over their declared plans, and a request whose search plan is charged nothing for its iterations or derives its universe from discovered candidates is refused.
    Given GNAF requests over reasoning plans
    When they are answered
    Then the answer charges each plan's search in steps over the declared plans
    And a plan whose search is charged nothing or a universe of discovered candidates is refused

  @RS-14 @build
  Scenario: The committed reasoning example verifies a multi-step clinical forward derivation with its invariant and termination, a deduplicating breadth-first planner, and a depth-first generate-and-verify screening over model-generated candidates, with every generated theorem kernel-checked, and planting a rule-threshold mutation in it is refused by verification.
    Given the committed reasoning example
    When it is verified, and verified again with a rule threshold mutated
    Then every generated theorem and every concrete derivation, trace, and verdict theorem is kernel-checked
    And the mutated rule is refused by verification with LLV7002
