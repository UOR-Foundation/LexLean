Feature: declarations

  Definition sentences, theorem-like components, and axiom policies (§15.7-§15.9, §18.6).

  @DF-01 @build
  Scenario: A valid type-definition sentence emits one nonrecursive sort-valued Lean def linked to its document entry.
    Given the `test.defs` project whose typedefinition `count` says `A count is defined as \(N\)`
    When lexlean check runs and the Lean for `Main` is rendered
    Then the Lean contains `def count : Type :=` followed by `Nat`
    And the linked `count` declaration is a Definition whose entry is `test.defs::count`

  @DF-02 @build
  Scenario: A valid term-definition sentence emits one nonrecursive explicitly typed Lean def.
    Given the `test.defs` project whose termdefinition says `\(double(n)\) is defined as \(n + n\)`
    When the Lean for `Main` is rendered
    Then it contains a `def double` line
    And that line carries the explicit result type `: Nat :=`

  @DF-03 @build
  Scenario: A valid predicate-definition sentence emits one nonrecursive Prop-valued Lean def.
    Given the `test.defs` project whose predicatedefinition `good` holds exactly when some `\(k = k\)`
    When the Lean for `Main` is rendered
    Then the `def good` line ends its signature with `: Prop :=`

  @DF-04 @build
  Scenario: Self recursion, mutual recursion, and later-declaration references are rejected.
    Given the `test.defs` project with `double(n)` redefined as `\(double(n) + n\)`
    When lexlean check runs
    Then it fails with one of LLF5001, LLR3003, or LLR3005 for the self recursion
    And moving the `good` predicate before `count` and making it use the not-yet-declared `double(k)` fails with LLR3005 or LLF5001

  @DF-05 @build
  Scenario: A definition's self head, explicit arguments, and signature order are checked exactly.
    Given the `test.defs` project with the self head written as `\(double(n, n)\)`
    When lexlean check runs
    Then it fails with LLF5001 for the wrong explicit argument count
    And a copy whose self head is `\(double(m)\)` under the binder `n` also fails with LLF5001

  @DF-06 @build
  Scenario: Every generated definition and theorem-like declaration carries one explicit axiom policy.
    Given the `test.defs` project with the `\noaxioms` line removed from the `count` typedefinition
    When lexlean check runs
    Then it fails with LLP2003 for the missing axiom policy
    And the unmodified project's canonical linked JSON records a `"policy"` on every declaration

  @DF-07 @build
  Scenario: Theorem, lemma, and corollary each emit Lean theorem declarations while retaining distinct document metadata.
    Given the example project with `theorem` `add-zero` rewritten as a `lemma`
    When lexlean check runs and the build for `Main` is rendered
    Then the Lean contains `theorem add_zero`
    And the canonical LaTeX keeps `\begin{lemma}`
    And the linked declaration kind is `Lemma`

  @DF-08 @build
  Scenario: Author-defined axioms, opaque declarations, and proofless theorem-like components are rejected.
    Given the example project with the theorem environment renamed to `axiom`
    When lexlean check runs
    Then it fails with LLL1004 because `axiom` is not an accepted environment
    And a copy with the whole `\begin{proof}...\end{proof}` block deleted fails with LLF5005

  @DF-09 @build
  Scenario: Every theorem-like component contains exactly one nonempty structured proof.
    Given the example project with an empty `\begin{proof}\end{proof}` body
    When lexlean check runs
    Then it fails with one of LLF5004, LLF5003, or LLF5005
    And a copy with two reflexivity proofs in one theorem fails with LLP2003

  @DF-10 @build
  Scenario: Generated declarations preserve source order and every document reference respects that order.
    Given the `test.defs` project declaring `count`, `double`, `good`, then `add-zero`
    When the Lean for `Main` is rendered
    Then `def count`, `def double`, `def good`, and `theorem add_zero` appear in that source order

  @DF-11 @build
  Scenario: Language 1.1 checks and lowers generic structures, classes, instances, inductives, definitions, structural recursion, matches, Boolean validators, and closed proofs from semantic source data.
    Given the source-free language-1.1 semantic-module fixture
    When it is checked, built, and verified through the fixed Lean and LaTeX backends
    Then every generic declaration and proof form is present in both linked semantic data and deterministic generated artifacts with an empty axiom policy
    And forward references, duplicate names, bad instance priority, nonstructural recursion, nonexhaustive matches, and raw backend fields each fail before a backend runs

  @DF-12 @build
  Scenario: Language 1.2 inductives admit uniform, strictly positive self, nested, and mutual recursion with a buildable base case, all checked before either backend runs.
    Given the committed recursive-data example with self-recursive, nested, mutual, and parameterized types across modules
    When it is checked, rendered, and verified, and copies change one recursive occurrence, base case, group, or structure
    Then the generated Lean declares the recursive inductives and one mutual block and verifies with real Lean
    And non-uniform, mis-applied, non-positive, uninhabited, mis-grouped, and self-referential structure data each fail with LLT4001 before any build output
    And language 1.1 still rejects recursive payloads and one node fewer than the observed charge overruns max_ir_nodes

  @DF-13 @build
  Scenario: Language 1.2 structural recursion and induction over a recursive inductive use exactly its direct recursive fields, across modules, with one induction hypothesis per recursive field.
    Given the committed recursive-data example whose main module recurses and inducts over an imported tree type
    When it is rendered and verified, and copies change a recursive call, a binder list, or an induction scrutinee
    Then the generated Lean uses pattern equations over the direct recursive fields and binds one hypothesis per field
    And a call on the matched value or a non-recursive field, a missing hypothesis, and induction over a nested inductive fail with LLT4001 before any build output

  @DF-14 @build
  Scenario: Language 1.2 generic definitions and theorems take explicit type parameters, every use supplies exactly their type arguments, recursion is never polymorphic, and every written type mentions only declared parameters.
    Given the committed higher-order example with generic map, fold, composition, and a generic theorem applied at Nat
    When it is rendered and verified, and copies drop a call's type arguments, swap a recursive call's, write an undeclared parameter, or omit a theorem's
    Then the generated Lean binds explicit type parameters and passes explicit type arguments and verifies with real Lean
    And each mutation fails with LLT4001 before any build output and language 1.1 rejects definition type parameters

  @DF-15 @build
  Scenario: An executable language-1.2 definition forms only non-escaping closures and calls only executable definitions; every violation fails before either backend runs.
    Given the committed higher-order example whose executable definitions pass lambdas and function references directly to executable functions
    When copies mark a closure-returning, closure-storing, or data-applying definition executable, or unmark an executable callee
    Then the canonical LaTeX records production eligibility of the accepted definitions
    And every violation fails with LLT4001 naming the escaping closure or the non-executable callee before any build output

  @DF-16 @build
  Scenario: Language 1.2 mutual definition groups recurse structurally over one recursive family, including nested and mutual inductives, and every call between members passes a structurally smaller family binder.
    Given the committed recursion example with mutual groups over a nested rose tree, the mutual Expr and Stmt syntax, and the naturals
    When it is rendered and verified, and copies change a call, a case, a decreasing argument, or a group label
    Then each group lowers to one Lean mutual block whose members declare structural termination and verify with real Lean
    And a non-decreasing or out-of-family call, a missing case, a member without a decreasing argument, a one-member group, and mixed families fail with LLT4001 before any build output

  @DF-17 @build
  Scenario: Language 1.2 well-founded definitions carry a binder-free natural-number measure and, per recursive call site, a prior theorem stating exactly that call's decrease obligation; linking checks the statements and Lean checks the proofs.
    Given the committed recursion example with three well-founded definitions and their evidence theorems
    When it is rendered and verified, and copies forge a statement, make the measure cyclic, add evidence, or also declare structural recursion
    Then the generated Lean binds each condition as a numbered hypothesis and discharges each decreasing goal with one explicit evidence application, and verifies with real Lean
    And forged, cyclic, miscounted, mixed, and language-1.1 evidence fail with LLT4001 before any build output

  @DF-18 @build
  Scenario: Language 1.2 state threading is explicit: ordered folds and bounded iteration pass the state through a direct closure, executable definitions may use them, and every iteration carries a natural-number bound.
    Given the committed collections example whose executable ancestor closure iterates a set of facts to a fixed point within a bound
    When it is rendered and verified, and copies drop the bound or mistype a fold step
    Then the generated Lean passes the step as a lambda to the fixed iteration runtime and the kernel computes the closure and its convergence flag
    And the unbounded iteration and the mistyped step fail with LLT4001 before any build output

  @DF-19 @build
  Scenario: Language 1.2 mutual definition groups recurse structurally over the whole recursive family of a nested inductive, the types the pinned Lean takes as the motives of its recursor: the type itself, then each distinct application of `List`, `Option`, `Result`, or a product that holds the type in a constructor field of an earlier member, in the order of discovery, so that a group may decrease on the type, on a list of its values, on a list of pairs whose second component is one, and on such a pair, and in a member's top-level match every pattern binder whose type is a member of the family is structurally smaller; a group that decreases on a type outside the family and a call that passes anything but a smaller binder of the family are each refused as LLT4001, and each group lowers to one Lean mutual block of structural definitions that the pinned Lean accepts.
    Given the committed nested-induction example with mutual definition groups over a type that holds a `List` of itself and a `List` of pairs that end in itself
    When it is rendered and verified, and copies change a decreasing type, a call, or a pattern binder
    Then each group lowers to one Lean mutual block whose members declare structural termination and verify with real Lean
    And a group that decreases on a type outside the family and a call that passes a binder outside it fail with LLT4001 before any build output
