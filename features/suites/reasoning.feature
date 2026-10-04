Feature: reasoning

  Language-1.2 reasoning machines: logics, inference rules, verifiers, reasoners, bounded search, termination evidence, traces, and their production realization (§17.12).

  @RS-01 @build
  Scenario: Language 1.2 logic, inference_rule, verifier, and reasoner declarations, the forward, search, and generate_and_verify strategies, the initial_invariant, terminates, observation_invariant, and answer_correct claims, and the reasoning_failure type belong to the closed lexlean/semantic-module/2 schema and its snapshot schema, are rejected under language 1.1 before either backend runs, and admit no member, strategy, or claim outside the closed schema, such as an opaque oracle, a prompt, an unregistered strategy, or an unregistered claim, which fail with LLT4001.
    Given a language-1.2 module with every reasoning construct
    When it is linked under language 1.2 and again under language 1.1, and with an opaque member, an unregistered strategy, or an unregistered claim added
    Then language 1.2 links it and its snapshot validates against the closed schemas
    And language 1.1 rejects every reasoning construct and the closed schema rejects the extra member, strategy, and claim with LLT4001

  @RS-02 @build
  Scenario: A logic names a prior state relation and an optional invariant with a statement-exact preservation theorem and an optional ranking, each of exactly its state type, and an inference rule over a logic is a typed guard, conclusion, and optional binding with a candidate list whose soundness theorem states exactly that the guard implies the relation and whose progress theorem states exactly that the ranking decreases; logics, rules, verifiers, and reasoners may be generic, and a predicate of the wrong signature, an inexact theorem, a rule over another logic than its reasoner's, or a use with the wrong number of type arguments fails with LLT4010.
    Given logics and inference rules with soundness and progress theorems
    When they are linked and every theorem is compared with its generated statement
    Then each rule elaborates to its guard, conclusion, candidates, and guarded application with a generated soundness theorem
    And a predicate of the wrong signature, an inexact theorem, a rule over a foreign logic, or a use with the wrong number of type arguments fails with LLT4010

  @RS-03 @build
  Scenario: A verifier links an executable Boolean check over a subject and a candidate to a prior proposition-valued specification by statement-exact soundness and optional completeness theorems that Lean restates against the fixed reasoning semantics; an inexact statement or a verifier whose subject or candidate type differs from its reasoner's fails with LLT4010, and a false soundness theorem is refused by verification with LLV7002.
    Given verifiers with soundness and completeness theorems
    When they are linked, bound to reasoners, and verified
    Then Lean restates each theorem against the fixed reasoning semantics
    And an inexact statement or a type mismatch fails with LLT4010 and a false soundness theorem is refused with LLV7002

  @RS-04 @build
  Scenario: Every reasoner, generic or not, elaborates in linking to ordinary language-1.2 inductives, structures, definitions, and theorems over the existing match, if, list_fold, iterate_until, and set operations, with no reasoner primitive and no model-based semantic escape hatch, and the formal-only LexLeanReasoning runtime appears in no production closure.
    Given forward, search, generate-and-verify, and generic reasoners
    When they are linked and their elaborations are inspected
    Then each is exactly ordinary declarations over the existing primitives, checked again by the ordinary rules
    And no generated declaration names a reasoning primitive and no production closure reaches LexLeanReasoning

  @RS-05 @build
  Scenario: A forward reasoner fires, at each of at most its fuel iterations, the first applicable rule in declared priority order and candidate order, re-checking the rule's guard at every application, accounts its iterations, guard evaluations, firings, and its one verification in its ledger, and its verdict is the verifier-accepted answer of a saturated state or exactly one of the closed failures exhausted, unsolved, or rejected.
    Given a forward reasoner over binding-free and binding rules
    When its elaboration runs on inputs that saturate, exhaust their fuel, and yield rejected and missing answers
    Then each step fires the first applicable rule in priority and candidate order through its guarded application
    And the verdict is the verified answer or exactly exhausted, unsolved, or rejected, as the kernel decides

  @RS-06 @build
  Scenario: A search reasoner explores states breadth-first or depth-first under explicit natural-number fuel and frontier bounds, with optional deduplication over an ordered state type, and accepts only a state whose answer its verifier's check accepts, and a generate-and-verify reasoner checks at most its budget of its generator's candidates in order and answers the first its verifier accepts; a missing or non-natural bound or budget, a literal zero frontier, or deduplication over an unordered state fails with LLT4011, and a search or generation cut short by a bound yields exhausted and never unsolved.
    Given breadth-first and depth-first search reasoners, with and without deduplication, and a generate-and-verify reasoner
    When they run under their fuel, frontier, and budget bounds and are linked with missing or ill-typed bounds
    Then each accepts only a verified answer and reports a bound cut as exhausted, as the kernel decides
    And a missing or non-natural bound or budget, a literal zero frontier, or deduplication over an unordered state fails with LLT4011

  @RS-07 @build
  Scenario: A terminates claim requires a ranking, a statement-exact progress theorem on every rule, an initial-invariant claim when progress assumes the invariant, and a statement-exact fuel-bound theorem, and generates a kernel-checked theorem that the forward reasoner saturates within its fuel; a claim without that evidence fails with LLT4011, and a false progress theorem for a rule that undoes another is refused by verification with LLV7002.
    Given forward reasoners claiming termination
    When their claims are linked and verified
    Then Lean proves that each saturates within its fuel from the ranking and the progress and fuel-bound theorems
    And a claim without a ranking, progress, or fuel bound fails with LLT4011 and a false progress theorem is refused with LLV7002

  @RS-08 @build
  Scenario: Each logic, rule, verifier, and reasoner generates fixed-template theorems over its elaboration, among them guarded-application soundness, trace replay, derivation, invariant preservation, iteration, frontier, and verification bounds, saturation, and answer soundness, whose proofs apply only the emitted, axiom-free LexLeanReasoning runtime and the declaration's own theorems; every runtime lemma and every template is used by the committed example and accepted by pinned Lean, and a mutated runtime lemma statement is refused by Lean.
    Given the reasoning example's generated theorems
    When the example is verified and its generated declarations are compared with the emitted runtime
    Then every runtime lemma and every template is produced and accepted by pinned Lean under its owner's axiom policy
    And a mutated runtime lemma statement is refused by Lean

  @RS-09 @build
  Scenario: Executable code reaches a rule's conclusion only through the rule's guarded application and a state's answer only through its reasoner's verifier, unless an answer_correct claim's statement-exact theorem proves the answer correct on every state, which erases the check, and otherwise fails with LLT4012, an answer proved correct only under an invariant keeping its guard; a trace is evidence only by replay: a forged trace or an inapplicable rule application yields invalid_step at run time and the kernel decides that it does.
    Given executable code using rules and reasoners
    When it calls a conclusion or an unverified answer directly, with and without an answer correctness claim, and replays forged and genuine traces
    Then a direct conclusion or unverified answer fails with LLT4012 unless the answer is proved correct
    And a forged trace or an inapplicable application replays to invalid_step and a genuine trace replays to the reasoner's state, as the kernel decides

  @RS-10 @build
  Scenario: Reasoning declarations and their elaborations are part of the semantic ID; each declaration is charged to max_ir_nodes, before it is elaborated, by a bound its elaboration never exceeds, so a reasoner whose elaboration would exceed the limit fails with LLS8002 before any of it is built, and an elaboration nests with the logarithm of its number of rules rather than their number; a language-1.2 snapshot carries each elaboration with its generated theorems, and the canonical document renders a closed catalog of interfaces, rules, strategy, bounds, claims, and generated obligations that contains no trace value and never says verified.
    Given the reasoning example, and a reasoner over many rules
    When its snapshot, identities, and canonical document are generated, and a reasoner is linked under a limit its elaboration would exceed
    Then each elaboration and generated theorem is in the snapshot and the semantic ID, the many-rule elaboration is shallow, and the document lists the closed catalog with no trace value and no verification claim
    And the reasoner over the limit fails with LLS8002 before it is elaborated

  @RS-11 @build
  Scenario: Production eligibility realizes inference rules and reasoners through their elaborations and erases logics and verifiers, the realization table maps every reasoning row to calculus elements, the eligibility report records each reasoning root's strategy, bounds or budget, rule order, ledger counters, and the theorems bounding them, and the example's rust-core and rust-std reasoning roots extract through Lean.
    Given the reasoning example's production roots
    When eligibility, realization, and extraction run over them
    Then rules and reasoners are realized through their elaborations and logics and verifiers are erased
    And each root's report records its reasoning resources and extraction succeeds for rust-core and rust-std

  @RS-12 @build
  Scenario: The compiler project's reasoning oracles declare exactly the declarations of every reasoner the reasoning example's production roots run (the forward reasoners Triage and Grade, the generic Spend, the breadth-first search Plan with deduplication, the depth-first search Screen, and the generate-and-verify reasoner Dose) and of Review, which the example states by theorem, and a calculus transcription of each produces the oracle's verdict on every fixture argument as the kernel decides, the explained answer and the six-counter ledger of its run too for Triage, Plan, Screen, and Dose, compiles to committed rust-core and rust-std packages, and charges at least the guard evaluations and firings of the reasoner's ledger in calculus steps.
    Given the compiler project's reasoning oracles and transcriptions
    When the transcriptions are evaluated, compared with the oracle, and rendered to Rust
    Then every outcome equals the oracle's, with the run's ledger where the transcription accounts it, as the kernel decides and the packages equal their generator
    And every transcription charges at least the ledger's guard evaluations and firings in steps

  @RS-13 @build
  Scenario: GNAF requests over forward-chaining plans of the clinical rule base charge rule search as execution in calculus steps and are answered over their declared plans, each plan computes on every patient of the request's domain the level the oracle's Triage derives from the same findings as the kernel decides, and a request whose search is charged nothing or whose universe is the candidates a search discovered is refused.
    Given GNAF requests over reasoning plans
    When they are answered
    Then the answer charges each plan's search in steps over the declared plans
    And each plan computes the level the oracle's Triage derives from the same findings as the kernel decides
    And a plan whose search is charged nothing or a universe of discovered candidates is refused

  @RS-14 @build
  Scenario: The committed reasoning example verifies a multi-step clinical forward derivation with its invariant and termination, a generic terminating countdown, a deduplicating breadth-first planner, a depth-first search and a generate-and-verify reasoner over model-generated candidates, and an answer proved correct with its check erased, with every generated theorem kernel-checked, and planting a rule-threshold mutation in it is refused by verification.
    Given the committed reasoning example
    When it is verified, and verified again with a rule threshold mutated
    Then every generated theorem and every concrete derivation, trace, and verdict theorem is kernel-checked
    And the mutated rule is refused by verification with LLV7002
