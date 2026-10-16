Feature: proofs

  The structured proof language and its conservative checking (§16).

  @PF-01 @build
  Scenario: Assume and exact-style simple proof sentences create scoped introductions and exact proof nodes.
    Given a `main-goal` theorem `if \(n = n\), then \(n + 0 = n\)` proved by `Assume \(h\).` then reflexivity
    When lexlean check runs and the Lean for `Main` is rendered
    Then the Lean contains `intro` for the Assume step
    And a `second` theorem closed with `\reference{Main::first}` renders an `exact` step

  @PF-02 @build
  Scenario: Simple Apply is accepted only when its declared signature yields exactly one residual premise.
    Given theorems `first`: `If \(0 + 0 = 0\), then \(0 * 0 = 0\)` and `second` proved by `Apply \(\reference{Main::first}\).` then reflexivity
    When lexlean check runs and the Lean for `Main` is rendered
    Then the Lean contains `apply` for the single residual premise
    And a `first` with two nested conditionals leaves two premises so the simple Apply fails with LLF5002 or LLF5003

  @PF-03 @build
  Scenario: Structured apply requires every numbered residual premise exactly once and in signature order.
    Given a doubly conditional `first` and a `second` proved by `\begin{apply}` with `\begin{premise}{1}` and `\begin{premise}{2}` each closed by reflexivity
    When lexlean check runs and the Lean for `Main` is rendered
    Then the structured apply lowers to `apply`
    And renumbering the first premise to `{premise}{2}` fails with LLF5003 or LLP2003

  @PF-04 @build
  Scenario: Reflexivity lowers only to pinned Lean rfl and closes the current goal.
    Given the example project whose proof is `Close the goal by reflexivity.`
    When the Lean for `Main` is rendered
    Then the proof body is exactly `by` followed by `rfl`
    And a `main-goal` theorem with two reflexivity sentences fails with LLF5002 because the goal is already closed

  @PF-05 @build
  Scenario: Witness steps supply the next existential witness with no implicit search.
    Given a `main-goal` theorem `There exists a natural number \(k\) such that \(k + 0 = k\)` proved by `Use \(0\) as the witness.` then reflexivity
    When lexlean check runs and the Lean for `Main` is rendered
    Then the witness lowers to `refine` with a `?_` placeholder
    And the same witness sentence on the non-existential goal `\(n + 0 = n\)` fails with LLF5002

  @PF-06 @build
  Scenario: Left and right alternative steps select only the corresponding disjunction constructor.
    Given a `main-goal` disjunction `\(n + 0 = n\) or \(n = 1\)` proved by `Select the left alternative.` then reflexivity
    When lexlean check runs and the Lean for `Main` is rendered
    Then the Lean contains `left`
    And the mirrored disjunction with `Select the right alternative.` renders `right`
    And selecting the left alternative on the plain goal `\(n + 0 = n\)` fails with LLF5002

  @PF-07 @build
  Scenario: Have establishes a nested proposition before introducing its fresh hypothesis into subsequent scope.
    Given a `main-goal` proof with `\begin{have}{h}` establishing `\(n + 0 = n\)` by reflexivity and then `Close the goal with \(h\).`
    When lexlean check runs and the Lean for `Main` is rendered
    Then the Lean contains `have` with `:= by`
    And a proof that closes with `\(h\)` without any preceding have fails with one of LLP2002, LLL1004, LLT4001, or LLF5002

  @PF-08 @build
  Scenario: Rewrite applies every explicitly directed rule strictly in source order at exactly one target.
    Given a `second` theorem rewriting its goal with `\forward{\reference{Main::first}}` then `\backward{\reference{Main::first}}`
    When lexlean check runs and the Lean for `Main` is rendered
    Then the reversed-arrow rule appears after `rw [` in source order
    And a rewrite whose rule `first` is a disjunction, not an equation, fails with LLF5002

  @PF-09 @build
  Scenario: Simplify lowers to simp only with exactly the listed rules and target.
    Given a `second` theorem with `\begin{simplify}{goal}` listing `\rule{\reference{Main::first}}`
    When lexlean check runs and the Lean for `Main` is rendered
    Then the Lean contains `simp only [`
    And a `\begin{simplify}{goal}` block with no rules fails with LLF5003

  @PF-10 @build
  Scenario: Constructor requires the exact ordered branch count and every branch closes.
    Given a `main-goal` conjunction `\(n + 0 = n\) and \(n * 1 = n\)` proved by `\begin{constructor}` with `\begin{branch}{1}` and `\begin{branch}{2}`
    When lexlean check runs and the Lean for `Main` is rendered
    Then the Lean contains `constructor`
    And deleting branch 2 fails with LLF5003 or LLF5004

  @PF-11 @build
  Scenario: Cases requires a validated descriptor, every constructor once, and exact branch binders.
    Given a `main-goal` proof `\begin{cases}{n}` with cases `lexlean.std.nat::zero` binding nothing and `lexlean.std.nat::succ` binding `\bind{m}`
    When lexlean check runs and the Lean for `Main` is rendered
    Then the Lean contains `cases` with a `zero` alternative and a `succ` alternative
    And omitting the `succ` case fails with LLF5003
    And changing the binder to `\bind{m;extra}` fails with LLF5003

  @PF-12 @build
  Scenario: Induction requires a validated descriptor, every constructor once, and exact field and induction-hypothesis binders.
    Given a `main-goal` proof `\begin{induction}{n}` whose `succ` case binds `\bind{m;ih}`
    When lexlean check runs and the Lean for `Main` is rendered
    Then the Lean contains `induction` with a `succ` alternative
    And shrinking the succ binder to `\bind{m}` fails with LLF5003

  @PF-13 @build
  Scenario: Calculation chains use one declared relation, at least one step, and exact endpoint proofs.
    Given a `second` theorem `\(0 + 0 = 0\)` proved by `\begin{calculate}` from `\start{0 + 0}` with one `\step{lexlean.core::eq}{0}{\reference{Main::first}}`
    When lexlean check runs and the Lean for `Main` is rendered
    Then the Lean contains `calc`
    And removing the only `\step` line fails with LLF5003

  @PF-14 @build
  Scenario: Proof locals, hypotheses, premise scopes, and case scopes cannot capture or leak.
    Given a `main-goal` proof that binds `m` inside the `succ` case of `\begin{cases}{n}` and then writes `Close the goal with \(m\).` after `\end{cases}`
    When lexlean check runs
    Then it fails with one of LLF5002, LLP2002, LLL1004, or LLT4001 because `m` does not leak out of its case scope

  @PF-15 @build
  Scenario: Every proof and nested branch must close all goals and rejects steps after closure.
    Given a `main-goal` conditional theorem whose proof is only `Assume \(h\).`
    When lexlean check runs
    Then it fails with LLF5004 for the unclosed goal
    And a proof with `Close the goal by reflexivity.` written twice fails with LLF5002 for the step after closure

  @PF-16 @build
  Scenario: Raw tactics, custom proof nodes, unrestricted automation, and proof holes are rejected.
    Given a `main-goal` theorem whose proof reads `By simp.`
    When lexlean check runs
    Then it fails with LLF5005 or LLL1004
    And a proof reading `The proof is omitted.` also fails with LLF5005 or LLL1004

  @PF-17 @build
  Scenario: native_decide is never accepted or generated.
    Given a `main-goal` theorem whose proof reads `Close the goal by native_decide.`
    When lexlean check runs
    Then it fails with LLF5005 or LLL1004
    And `language/semantics.toml` contains no constructor named `native_decide`
    And the rendered Lean for the example `Main` never contains `native_decide`

  @PF-18 @build
  Scenario: Lean proof failures remap to the smallest originating LexLean proof or statement span.
    Given the example project with the statement changed to the false `\(n + 0 = 0\)` while keeping the reflexivity proof
    When lexlean verify runs
    Then it fails with LLV7002
    And the diagnostic's primary span points into `Main.lex.tex`
    And the span covers nonempty originating source text of that module

  @PF-19 @build
  Scenario: The language-1.2 linear_arithmetic proof form names only the prior document definitions it unfolds, lowers to one fixed omega script, proves true linear obligations, and is refused by Lean on a false one.
    Given the committed recursion example whose decrease evidence is proved by linear_arithmetic
    When it is rendered and verified, copies add a tactic member to the form and to a unit-variant proof, a copy unfolds a later definition, a language-1.1 module uses the form, and a copy states a false linear claim
    Then the generated Lean is the fixed intros, Boolean-comparison rewriting, and omega script, with subst_vars and the unfolded weight measure where the form names it, and verifies
    And the extra members, the later definition, and the language-1.1 use fail with LLT4001 and the false claim checks but fails verification with LLV7002

  @PF-20 @build
  Scenario: Under language 1.2 the order propositions <= and < take two operands of one integer type, Nat, Int, or one fixed-width type, and lower to Lean's order on that type; language 1.1 keeps them over Nat only.
    Given the committed fold-laws example, whose integer-kernel theorems state order over Int and UInt8
    When it is rendered and verified, a copy orders an Int against a Nat, a copy orders two strings, and a language-1.1 module orders two Int values
    Then the generated Lean writes each order with Lean's `<=` and `<` on the operands' own type and verifies
    And the mixed operands, the strings, and the language-1.1 use fail with LLT4001

  @PF-21 @build
  Scenario: The language-1.2 linear_arithmetic proof form reads the runtime: it rewrites less_than, equal, negate, quotient, remainder, and fixed-width order and equality into linear arithmetic over mathematical values, splits every if and match left in the goal, proves integer kernels for all inputs, and is refused by Lean on a false one.
    Given the committed fold-laws example, whose clamped requantization step, truncating division, and fixed-width order theorems are proved by linear_arithmetic
    When it is rendered and verified, and copies state a clamp bound one too small, a quotient identity with the wrong remainder sign, and a fixed-width order one past its literal
    Then the generated Lean is the fixed script: the runtime bridges and the registered rewrites, the fixed-width bit reading, the split of every if and match, and omega, and it verifies for all inputs
    And each false copy checks but fails verification with LLV7002

  @PF-22 @build
  Scenario: The runtime module LexLeanRuntime.Laws states the fold laws as LexLean source with every row's axioms pinned, and linear_arithmetic rewrites additive list, set, and map folds into sums that are linear and invariant under append and reverse, so fold equalities verify for all inputs with decide only on closed instances.
    Given the shipped runtime and the committed fold-laws example
    When the runtime is built, a copy of the law registry names a declaration the module lacks or registers a law twice, the example is verified, and copies state a fold equality that is false
    Then the laws module is a native core module importing the portable and collection runtimes with every row's axioms pinned, and the registry's rewrites and laws are its declarations
    And the planted registries are refused, the example's fold equalities over lists, sets, and maps verify for all inputs, every decide proof is of a closed statement, and each false copy fails verification with LLV7002

  @PF-23 @build
  Scenario: The language-1.2 law proof form applies one registered fold law (fusion, the fold of a pair, or the fold of an append) to checked type arguments, arguments, and prior theorems discharging its premises, and is refused in linking for an unregistered law, a wrong argument count or type, or a missing premise, and by Lean for a statement the law does not prove.
    Given the committed fold-laws example, whose single-pass statistics and parity theorems are proved by law
    When it is rendered and verified, copies name an unregistered law, drop an argument, pass an argument of the wrong type, drop the premise, name a later theorem as the premise, a language-1.1 module uses the form, and a copy applies the pair law to a swapped statement
    Then the generated Lean applies the registered theorem with explicit type arguments, arguments, and premises, and verifies
    And each malformed copy and the language-1.1 use fail with LLT4001 and the swapped statement fails verification with LLV7002
