Feature: stdlib

  The standard library: language-1.2 source beside the compiler, each member a certified production root validated by a seeded differential of its evaluator and rustc realizations against the Rust standard library (§17.22).

  @SL-01 @build
  Scenario: The standard library's `Text` module is language-1.2 source whose seven members over `Bytes` (`startsWith`, `endsWith`, `trim`, `lines`, `splitOn`, `joinWith`, `find`) are production roots eligible for `rust-std` with exactly their declared effects; each member's lowered program under the calculus evaluator and its rendered `rust-std` package under the pinned rustc agree with Rust's `starts_with`, `ends_with`, `trim_ascii`, `lines`, `split`, `join`, and `find` on a seeded corpus; a planted defect in a member is detected; and the certified packages the production export publishes for it are the committed ones.
    Given the committed `stdlib` project and the Rust standard library as the oracle
    When the project is checked and built, each member is lowered and run by the evaluator and by its rendered rust-std package on a seeded corpus, a character class in `trim` is planted, and the project is exported under the pinned Lean
    Then the seven roots are eligible for rust-std with exactly their declared effects and no run disagrees with the oracle
    And the planted `trim` disagrees, and every exported package is byte-identical to its committed package and realizes the member's lowered program
