Feature: stdlib

  The standard library: language-1.2 source beside the compiler, each member a certified production root validated by a seeded differential of its evaluator and rustc realizations against the Rust standard library (§17.22).

  @SL-01 @build
  Scenario: The standard library's `Text` module is language-1.2 source whose seven members over `Bytes` (`startsWith`, `endsWith`, `trim`, `lines`, `splitOn`, `joinWith`, `find`) are production roots eligible for `rust-std` with exactly their declared effects; each member's lowered program under the calculus evaluator and its rendered `rust-std` package under the pinned rustc agree with Rust's `starts_with`, `ends_with`, `trim_ascii`, `lines`, `split`, `join`, and `find` on a seeded corpus; a planted defect in a member is detected; and the certified packages the production export publishes for it are the committed ones.
    Given the committed `stdlib` project and the Rust standard library as the oracle
    When the project is checked and built, each member is lowered and run by the evaluator and by its rendered rust-std package on a seeded corpus, a character class in `trim` is planted, and the project is exported under the pinned Lean
    Then the seven roots are eligible for rust-std with exactly their declared effects and no run disagrees with the oracle
    And the planted `trim` disagrees, and every exported package is byte-identical to its committed package and realizes the member's lowered program

  @SL-02 @build
  Scenario: The standard library's `Sha256` module is language-1.2 source stating FIPS 180-4 SHA-256 over `UInt32` bit operations and `iterate` (modular addition by widening to `UInt64`, rotation as two shifts, padding read by position, the message schedule as a sixteen-word window, sixty-four rounds over the round-constant list), whose roots `sha256 : Bytes -> Digest` and `sha256Hex : Bytes -> Bytes` are production roots eligible for `rust-std` with exactly their declared effects; the lowered program under the calculus evaluator and the rendered `rust-std` package under the pinned rustc compute the FIPS 180-4 example digests and every NIST CAVP SHA-256 short- and long-message vector, and agree with the `sha2` crate on a seeded corpus whose lengths cover every padding boundary, a disagreement failing closed and never resolved by adopting the oracle's digest; a planted one-bit error in the round function is detected; and the certified packages the production export publishes for the two roots are the committed ones.
    Given the committed `stdlib` project, the FIPS 180-4 example digests, the NIST CAVP `SHA256ShortMsg` and `SHA256LongMsg` vectors, and the `sha2` crate as the oracle
    When the project is checked and built, `sha256` and `sha256Hex` are lowered and run by the evaluator and by their rendered rust-std packages on the vectors and on a seeded corpus, one bit of the round function is planted, and the project is exported under the pinned Lean
    Then both roots are eligible for rust-std with exactly their declared effects, every vector is computed and no run disagrees with the oracle
    And the planted round function disagrees on the first vector, and every exported package is byte-identical to its committed package and realizes the root's lowered program
