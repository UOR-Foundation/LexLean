Feature: gnaf

  GNAF requests over the production realization calculus: the complete-system universe, machine contract, accounting, objective order, and claim, fixed before any optimizer (§17.15).

  @GN-01 @build
  Scenario: Every committed GNAF request and fixture is canonical, validates against the GNAF schemas whose calculus definitions are the target program schema's, and states its components' universe identity; a malformed request, an invalid reference, plan, or realized program, a reference without a value on a domain argument, or a wrong universe identity fails closed with LLB6006, and a machine or request beyond the host's capacity fails closed with LLS8002.
    Given every committed request under `compiler/gnaf/` and a set of malformed, invalid, misidentified, and over-capacity requests
    When each is reread, schema-validated, loaded, and answered
    Then every committed fixture is canonical, schema-valid, states its own universe identity, loads, and has its committed answer
    And each malformed, invalid, or misidentified request fails with LLB6006 and each over-capacity one with LLS8002, while a request at full fuel capacity is answered

  @GN-02 @build
  Scenario: The GNAF model is a kernel-checked LexLean definition, Lean's kernel reduces the universe, the system statuses, and the answer of every committed request to what the host transcription computes, and a wrong answer, an answer computed from a universe with a system omitted, a tie with a member dropped, and a universe with a system omitted are rejected by Lean.
    Given the `compiler` project and its generated `GnafFixtures` module
    When the project is verified under pinned Lean, and a copy stating a wrong answer, answers from universes with systems omitted, ties with a member dropped, and a universe with a system omitted is verified
    Then the kernel proves every committed universe, status list, and answer, and the module equals its generator
    And the copy fails with LLV7002 on each planted statement

  @GN-03 @build
  Scenario: Candidate membership is the grammar's expansion, which a kernel-checked theorem proves equal to the grammar's well-formed selectors for every grammar, independent of evaluation and fixed by a universe identity before any optimizer, and optimizer-defined, discovered, cached, and internal-plan universes and missing, self-referential, or optimizer-citing completeness evidence are rejected.
    Given a grammar of two plans and one dispatch threshold
    When its universe is expanded, its completeness theorem is read, and requests carrying every other carrier and completeness evidence are answered
    Then the members are exactly the fixed and dispatching systems in expansion order, the kernel-checked theorem equates membership with well-formedness for every grammar, and neither depends on evaluation
    And every other carrier or completeness evidence is rejected, even a discovered carrier listing every member

  @GN-04 @build
  Scenario: Every action a system performs is charged by steps, every admitted preparation action by a positive constant for each plan that needs it or by a prepared artifact bound in the common initial state, operands are charged by weight, and a request fits the capacity its machine binds; hidden zero-cost, free, undeclared, duplicated, unaccounted, unbound, stray, unit-cost, over-capacity, and unrealizable actions are rejected, and a plan's preparation enters exactly the cost of the systems that can run it.
    Given a valid machine contract
    When each action is charged otherwise, preparation is declared per plan or bound as a prepared artifact, operands are charged by unit, or the capacity is exceeded
    Then each violating request is rejected naming the violated rule
    And a plan's preparation charge adds exactly to the systems that can run it, and a bound prepared artifact adds nothing

  @GN-05 @build
  Scenario: Scalar claims require the total step order and Pareto claims the componentwise steps-and-size order, a scalar claim over the partial order, a vector claim over the total order, the restricted-universe alias, an undecided claim class, and a scope beyond the grammar universe are rejected, equal-cost systems are all kept, a frontier answer has incomparable members, and GNAF-VEC-02 posed over the calculus has a frontier of exactly three of four systems for which no omission is certified and whose componentwise minima no system attains.
    Given the scalar and vector requests, a grammar with a duplicated plan, and GNAF-VEC-02 posed over the calculus
    When every claim class is posed under each objective and scope and each request is answered and certified
    Then only scalar claims under the total order and Pareto claims under the componentwise order are accepted within the grammar universe, and every equal-cost system is kept
    And the frontiers hold incomparable systems, the posed GNAF-VEC-02 frontier is three of four systems, no omission is certified, and no system attains the componentwise minima

  @GN-06 @build
  Scenario: Complete-system cost includes selection and counts only the code a system can run, the system argmin differs from the best internal plan and from the per-input plan envelope that no system attains, an inadmissible system is excluded, and an unresolved system makes the answer incomplete rather than being removed.
    Given short lists, long lists, and their union as domains
    When each is answered over the same universe of plans and dispatching systems
    Then the union's argmin is a dispatching system although the best single plan is cheaper on the long lists and recursion on the short ones, and each system's size is the code it can run
    And the sum of the per-domain optima is lower than any system's cost, an inadmissible plan is excluded, and a diverging plan makes the answer incomplete

  @GN-07 @build
  Scenario: The authority's GNAF-VEC-01, GNAF-VEC-02, GNAF-VEC-04, GNAF-VEC-17, GNAF-REJ-14, and GNAF-REJ-29 vectors are kernel-checked theorems over the argmin and frontier the answers are computed by, stated with the authority's numbers, and the authority is vendored with a recomputed SHA-256, cited by revision, and claimed some-true.
    Given the `Gnaf` model, the vendored authority, `model/authorities.toml`, and `model/ledger.toml`
    When the vector theorems' statements are decoded, the authority's bytes are hashed, and a copy stating a wrong frontier for GNAF-VEC-02 is verified
    Then every vector is stated with the authority's numbers over the argmin and frontier `evaluate` uses, the vendored bytes hash to the row's SHA-256, and the claim is some-true
    And the copy fails with LLV7002

  @GN-08 @build
  Scenario: The GNAF dependency manifest names the authority's revision and SHA-256, every kind, operation, machine, cost, proof, interchange, and address profile with its role, every restriction of the admitted universe, and exactly the claim classes the model answers, and equals its generator.
    Given the committed `compiler/gnaf.manifest.json` and the authority row
    When the manifest is reread with unknown fields refused and compared with the model
    Then it equals its generator and names the authority's revision, SHA-256, and vendored copy, and every required profile with its role
    And its restrictions state the host's capacity and its strongest claims are exactly the classes the model answers
