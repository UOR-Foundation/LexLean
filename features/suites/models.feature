Feature: models

  Language-1.2 models: content-addressed artifacts, contracts, realizations, evidence, model bindings, composition, and the runtime boundary (§17.12).

  @MD-01 @build
  Scenario: Language 1.2 artifact, contract, realization, evidence, and model declarations, the checked_apply term, the contract_violation type, and the less_than primitive belong to the closed lexlean/semantic-module/2 schema and its snapshot schema, are rejected under language 1.1 before either backend runs, and admit no member outside the closed schema, such as prompt text, a free-form description, or raw model configuration.
    Given a language-1.2 semantic module with every model construct
    When it is linked under language 1.2 and again under language 1.1, and with a prompt member added
    Then language 1.2 links it and its snapshot validates against the closed schemas
    And language 1.1 rejects every model construct and the closed schema rejects the extra member with LLT4001

  @MD-02 @build
  Scenario: A model artifact is admitted only from a configured, content-addressed, confined project file whose SHA-256 and byte length equal its declaration and whose bytes decode under its declared closed schema to its declared type and role; a missing artifact or a digest or length mismatch fails with LLR3007, a schema, type, or role violation fails with LLR3008, and the generated Lean embeds the exact bytes with a kernel-checked theorem that the typed value is their decoding.
    Given the models example's configured artifacts
    When each is read, digested, decoded, and embedded
    Then the generated Lean states that each typed value is the decoding of the exact embedded bytes and Lean's kernel checks it
    And a missing file or a digest or length mismatch fails with LLR3007 and a schema, type, or role violation fails with LLR3008

  @MD-03 @build
  Scenario: A contract names typed input, output, and optional state interfaces and prior proposition-valued predicates of exactly those interface signatures, and each runtime validator is an executable Boolean definition of the same signature linked to its predicate by a statement-exact soundness theorem and an optional completeness theorem that Lean restates against the fixed model semantics; any mismatch fails with LLT4006.
    Given contracts with predicates and runtime validators
    When they are linked and their soundness and completeness theorems are compared with the generated statements
    Then each theorem states exactly its obligation and Lean restates it against the fixed model semantics
    And a predicate or validator of the wrong signature or an inexact theorem fails with LLT4006

  @MD-04 @build
  Scenario: Deterministic, rule, statistical, and neural realizations each elaborate to one ordinary language-1.2 denotation over their declared interface, with exact integer kernels over the less_than primitive, and their layer widths, artifact shapes and roles, labels, and statement-exact width evidence are checked in linking; a descriptor outside the closed set, such as a floating-point element, is rejected, and a width, shape, or role mismatch fails with LLT4006.
    Given deterministic, rule, statistical, and neural realizations
    When each descriptor is elaborated
    Then each is exactly one ordinary denotation with its companions, typed by the ordinary rules
    And a floating-point element is rejected and a width, shape, or role mismatch fails with LLT4006

  @MD-05 @build
  Scenario: A model binds one contract instantiation to a realization of the identical interface, with entry evidence for every effective precondition; an interface mismatch fails with LLT4006 before either backend runs, and a realization whose behavior violates a statement-exact contract claim is refused by Lean's kernel at verification.
    Given model bindings of contracts to realizations
    When their interfaces, evidence, and entry obligations are linked and verified
    Then an identical interface links and its contract claims are kernel-checked
    And an interface mismatch fails with LLT4006 and a realization violating its claim is refused at verification

  @MD-06 @build
  Scenario: Every evidence claim is a kind of the closed claim set whose statement LexLean generates from the contract and realization, is discharged by a prior theorem stating it exactly, and is restated against the fixed Lean model semantics; an unregistered, vacuous, inexact, or foreign claim fails with LLT4009, the canonical document says only that a claim is discharged, and only the verified attestation records the generated declarations as verified.
    Given evidence declarations with every claim kind
    When each claim is generated, compared, and restated in Lean
    Then only an exact prior theorem discharges a claim and the document never calls a claim verified
    And an unregistered, vacuous, inexact, or foreign claim fails with LLT4009

  @MD-07 @build
  Scenario: Every model application in executable code validates each contract predicate its evidence does not discharge, through sound validators, in the fixed order input invariant, precondition, output invariant, postcondition, returning the contract violation on refusal; an application missing a required check, naming a check without a sound validator, or applying a realization directly fails with LLT4008.
    Given executable code applying models
    When each application is compared with what its evidence discharges
    Then each required check runs in the fixed order and a refusal is returned
    And a missing check, a check without a sound validator, or a direct realization application fails with LLT4008

  @MD-08 @build
  Scenario: Sequence, fan-out, product, branch, and scan composites of stateless and stateful models are ordinary typed compositions whose stage interfaces and threaded states match exactly, whose every stage precondition and state invariant is discharged at the model's entry, by a statement-exact junction theorem, or by a run-time check, and whose every check a stage's evidence leaves open after it runs is made at run time; any other composition fails with LLT4007.
    Given sequence, fan-out, product, branch, and scan composites
    When their stages, junctions, and entries are linked
    Then stage interfaces and states match and every stage precondition is discharged
    And a mismatched, unjustified, or forged composition fails with LLT4007

  @MD-09 @build
  Scenario: The exact integer kernels of dense, rectified-linear, requantization, and first-maximum layers and the less_than primitive agree with an independent integer model on seeded random weights and inputs under Lean.
    Given seeded random integer networks and inputs
    When their kernels are evaluated by Lean and by an independent model
    Then every output agrees
    And a mutated kernel is refused

  @MD-10 @build
  Scenario: Model declarations are part of the semantic identity, so changing an artifact byte with its declared digest, a descriptor, a claim, or a check changes the semantic ID, and language-1.2 snapshots carry, schema-valid, every elaborated declaration, generated obligation, cross-check, and required check.
    Given the models example and its snapshot
    When an artifact byte, a descriptor, a claim, or a check changes
    Then the semantic ID changes
    And the snapshot carries every elaborated declaration, obligation, cross-check, and required check and validates

  @MD-11 @build
  Scenario: Model constructs have production dispositions under which artifacts, realizations, models, validators, and checked applications are realized through their elaborations while contracts and evidence are erased, the realization table covers every new runtime construct, and production roots applying an artifact-backed model, directly and through its checks, are eligible and extract the same closure through Lean.
    Given the models example's production roots
    When their eligibility is analysed and their closures extracted through Lean
    Then artifacts, realizations, models, and checked applications are realized through their elaborations and contracts and evidence are erased
    And Lean's closure equals the eligibility closure

  @MD-12 @build
  Scenario: The committed models example verifies nontrivial deterministic stateful, rule, statistical, artifact-backed neural, and composite models with every claim kernel-checked, and planting a contract and realization mismatch in it is refused by verification.
    Given the committed models example
    When it is verified and a contract and realization mismatch is planted
    Then every claim is kernel-checked
    And the planted mismatch is refused by verification
