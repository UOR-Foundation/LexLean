Feature: language-models

  Language-1.2 text components: tokenizers and their round-trip laws, context windows, text
  generators and their explicit randomness contract, text classifiers, adapters and composition
  (§17.12).

  @LM-01 @build
  Scenario: A `tokenizer` is a closed language-1.2 declaration whose scheme is `byte_level` or `longest_match` and whose vocabulary is a #29 artifact with role `vocabulary` and schema `utf8_lines`; tokens are `uint32`, the single bytes are the ids 0..255 present always as the byte fallback, the vocabulary pieces take the ids from 256 in artifact order, and the declaration elaborates to `encode` and `decode` over `bytes` together with `encodeText` and `decodeText` over `string` through `utf8_encode` and `utf8_decode`, so a scheme outside the closed enumeration, a vocabulary whose role or schema is not the one stated, or a token outside `uint32` is refused as a schema violation (`LLT4001`) or by the artifact rules (`LLR3007`).
    Given a language-1.2 tokenizer declaration over a `utf8_lines` vocabulary artifact
    When it is linked under language 1.2, and again with an unknown scheme
    Then the ids 0..255 are the byte fallback and the pieces follow in artifact order
    And the unknown scheme is refused as a schema violation

  @LM-02 @build
  Scenario: A tokenizer's laws are generated and kernel-checked: `roundtrip` states that for every byte string `b`, `decode (encode b)` is `some b`; `text_roundtrip` states the same over `string`; and `in_range` states that every token a decode produces indexes the vocabulary, so the round trip also fixes the token boundaries, and a tokenizer whose elaboration does not close these laws fails verification.
    Given a linked tokenizer with its generated laws
    When Lean checks `roundtrip`, `text_roundtrip` and `in_range` for it
    Then every byte string decodes back to itself
    And every decoded token indexes the vocabulary, so the boundaries are fixed

  @LM-03 @build
  Scenario: Text that is not valid UTF-8 is refused and never tokenized: an artifact whose bytes do not decode under the `utf8_lines` schema — invalid UTF-8, a CR, or a missing final LF — fails by its artifact rules before any byte is decoded, and a text offered to `encodeText` that is not valid UTF-8 is refused rather than transcoded.
    Given text artifacts and texts, valid and invalid, in UTF-8
    When each is read and tokenized
    Then the valid ones are tokenized
    And the invalid one is refused rather than transcoded

  @LM-04 @build
  Scenario: A `context_window` has a capacity of at least one and an overflow disposition of `refuse` or `keep_last`, elaborating to a stateful #29 contract and realization over `list uint32` whose invariant is that the length does not exceed the capacity, with a preservation theorem, a suffix theorem and an exactness theorem; the window composes with the other stateful stages, `scan` included, a capacity below one is refused, and a context that overflows a `refuse` window is refused before any state is built.
    Given a `context_window` with a capacity and an overflow disposition
    When it composes with the stateful stages and a context overflows a `refuse` window
    Then its invariant, preservation, suffix and exactness theorems hold
    And the overflowing context is refused before any state is built

  @LM-05 @build
  Scenario: A `text_generator` takes a tokenizer, a window and a scorer, which is a #29 model from `list uint32` to `list int` whose width is the vocabulary size, and it requires `max_tokens` and `stop`; its loop is `iterate_until`, running out of fuel is `exhausted`, and its policy `greedy` takes the first maximum and has no seed parameter, so it is deterministic by type.
    Given a `text_generator` with a tokenizer, a window and a scorer, and its `max_tokens` and `stop`
    When it runs under the `greedy` policy to its stop or to exhaustion
    Then it takes the first maximum and has no seed parameter, so it is deterministic by type
    And running out of fuel reports `exhausted`

  @LM-06 @build
  Scenario: The policy `sample` samples only through closed members: the generator is `xorshift32` and the distribution `proportional`, the seed is an explicit input and the next seed an explicit output, the arithmetic is checked fixed-width with no wrapping primitive added, and a generation request that samples without that explicit contract is refused; no claim is made about the quality of the randomness.
    Given a generator under the `sample` policy
    When it samples, and again without the explicit contract
    Then the closed `xorshift32` generator and `proportional` distribution take the explicit seed and return the next seed
    And the run without that contract is refused

  @LM-07 @build
  Scenario: A `text_classifier` takes token-count features from the tokenizer and feeds them to a #29 model that produces the label, so classification is deterministic for a deterministic model and a label outside the model's closed label enumeration is refused.
    Given a `text_classifier` over token-count features and a #29 model with a closed label enumeration
    When it classifies, and again with a label outside the enumeration
    Then it produces the label of its model deterministically
    And the outside label is refused

  @LM-08 @build
  Scenario: Text components compose like any other language-1.2 declarations, and adapters add no new kind: an adapter is a #29 model whose realization is one of these elaborations, with a contract over the text types and `satisfies_contract` evidence, and a composition or an adapter whose interface does not match is refused rather than coerced.
    Given text components composed in a pipeline and an adapter realised by one of them
    When the composition is linked, and again with a mismatched interface
    Then the adapter's contract over the text types is satisfied with `satisfies_contract` evidence
    And the mismatched interface is refused rather than coerced

  @LM-09 @build
  Scenario: No free-form prompt string is semantic code, by construction: `string` is data, the primitive set has no string-to-code member, and a prompt reaches a model only as `encodeText`; a conformance case audits the registries for a string-to-code row and plants one, and a declaration offering prompt text as its semantic content is refused as a schema violation (`LLT4001`).
    Given the registries the text components are built from
    When a string-to-code row is planted in them
    Then the audit finds no string-to-code member and the primitive set has none
    And the planted row is detected

  @LM-10 @build
  Scenario: Text components are production-eligible in `rust-std` only and refused in `rust-core` with `LLT4005`, because strings and bytes allocate; the production evidence for a text root carries the differential comparison and certificates A, B and E, and the eligibility report gains `text_rows`, which record the nondeterminism contract of each production root as the reasoning rows do.
    Given a production root whose closure contains text components
    When its eligibility is analysed and its production evidence is built
    Then it is eligible in `rust-std` and refused in `rust-core` with LLT4005
    And its `text_rows` record the nondeterminism contract, with certificates A, B and E and the differential

  @LM-11 @build
  Scenario: The committed `examples/text-models` example builds its pipeline from the closed components alone — tokenizer, window, scorer, and a generator or classifier — verifies with every claim kernel-checked, and detects a planted mutation.
    Given the committed `examples/text-models` example
    When it is verified and a mutation is planted
    Then its pipeline of closed components verifies with every claim kernel-checked
    And the planted mutation is detected

  @LM-12 @build
  Scenario: Every fixed helper the text components are elaborated through is covered by a conformance case, so that a helper added without a case is a failure of the register rather than an untested path.
    Given the fixed helpers the text components elaborate through
    When the conformance cases are enumerated
    Then every helper has a case of its own
    And a helper added without a case fails the register
