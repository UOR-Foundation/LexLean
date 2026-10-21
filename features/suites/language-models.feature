Feature: language-models

  Language-1.2 text components: tokenizers and their round-trip laws, context windows, text
  generators and their explicit randomness contract, text classifiers, adapters and composition
  (§17.12).

  @LM-01 @build
  Scenario: A `tokenizer` is a closed language-1.2 declaration whose scheme is `byte_level` or `longest_match` and whose vocabulary is a #29 artifact with role `vocabulary` and schema `utf8_lines`; tokens are `uint32`, the single bytes are the ids 0..255 present always as the byte fallback, the vocabulary pieces take the ids from 256 in artifact order, and the declaration elaborates to `encode` and `decode` over `bytes` together with `encodeText` and `decodeText` over `string` through `utf8_encode` and `utf8_decode`, so a scheme outside the closed enumeration, a vocabulary whose role or schema is not the one stated, or a token outside `uint32` is refused as a schema violation (`LLT4001`) or by the artifact rules (`LLR3007`).
    Given a language-1.2 tokenizer declaration over a `utf8_lines` vocabulary artifact
    When it is linked under language 1.2, and again with an unknown scheme
    Then the ids 0..255 are the byte fallback and the pieces are in artifact order
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
  Scenario: A `context_window` is a closed language-1.2 declaration of a capacity of at least one token and an overflow disposition `refuse` or `keep_last` that elaborates to `capacity`, `fit` over `list uint32`, and `push` of one `uint32` token through the text-laws library, with its laws generated and kernel-checked: `bounded` and `push_bounded` state that every admitted or pushed context fits the capacity, `exact` that a context that fits is admitted unchanged, `suffix` under `keep_last` that what is kept is the end of the context, and `refuses` under `refuse` that a context that does not fit is not admitted; a capacity of 0 is refused (`LLT4016`), a disposition or capacity outside the closed schema is a schema violation (`LLT4001`), and a window whose laws observe an axiom its policy omits fails the audit.
    Given context windows of capacity 2 under `keep_last` and under `refuse`
    When they are linked and their laws are checked by Lean, and again with capacity 0
    Then every admitted or pushed context fits and a context that fits is admitted unchanged
    And capacity 0 is refused as an unstated bound

  @LM-05 @build
  Scenario: A `text_generator` under the `greedy` policy names a prior tokenizer, a prior context window, a prior stateless model from `list uint32` to `list int` whose every runtime check its evidence discharges, a prior `uint32` stop token, and a `max_tokens` of at least one, and elaborates to the closed outcome types `End` (`stop`, `exhausted`) and `Failure` (`unknown_token`, `context_overflow`, `invalid_utf8`) and to a loop of at most `max_tokens` steps that takes the first maximum and no seed; its laws `bounded` (at most `max_tokens` tokens are generated) and `context_bounded` (the scorer is consulted only on contexts that fit the window) are generated and kernel-checked, the loop reaches each end and each failure in the kernel, a wrong claim about its output fails verification, a `max_tokens` of 0 is refused (`LLT4016`), and a member of another kind is refused (`LLT4006`).
    Given greedy generators over a tokenizer, a window, a scorer model and a stop token
    When they are linked, their laws are checked by Lean, and their runs are evaluated by the kernel
    Then each run ends at its stop token or after `max_tokens` tokens, or with its closed failure
    And no elaborated definition takes a seed, and `max_tokens` 0 is refused

  @LM-06 @build
  Scenario: Nondeterminism is an explicit contract: a `text_generator` under the `sample` policy draws through the closed `xorshift32` generator and `proportional` distribution only, takes its seed as an explicit `uint32` input and returns the next seed as an explicit output, so the same prompt and seed give the same output, which the kernel computes; it has the same generated and kernel-checked laws as a greedy generator, `xorshift32` agrees with the reference sequence, and a generator or distribution outside the closed members, a sample policy without one, or a seed stated on a policy is a schema violation (`LLT4001`).
    Given a sampling generator under `xorshift32` and `proportional`
    When it runs from a prompt and a seed in the kernel
    Then the same prompt and seed give the same tokens and the next seed is returned
    And a generator or distribution outside the closed members is a schema violation

  @LM-07 @build
  Scenario: A `text_classifier` is a closed language-1.2 declaration that names a prior tokenizer and a prior stateless model from `list int` to a label type whose every runtime check its evidence discharges, and elaborates to `features`, the count of each token id below the tokenizer's `size`, to `classifyTokens`, the model applied to the features, and to `classify` over `string` through `encodeText`, with its laws generated and kernel-checked: `features_width` states that the features have exactly `size` entries, `features_total` that the features of an encoded text sum to its token count so that no token is dropped, and `features_fit` that the fixed-width map `fitWidth` leaves the features unchanged; a classifier whose tokenizer is not a prior tokenizer, or whose model is not a stateless model from `list int` with every runtime check discharged, is refused (`LLT4006`), a member outside the closed schema is a schema violation (`LLT4001`), and a classifier whose laws observe an axiom its policy omits fails the audit.
    Given classifiers over a tokenizer and a stateless model from `list int` to a label
    When they are linked, their laws are checked by Lean, and their classifications are evaluated by the kernel
    Then the features have exactly the tokenizer's size entries and sum to the token count of the text
    And a classifier over a non-tokenizer or a stateful model is refused

  @LM-08 @build
  Scenario: Text components compose and adapt through the model constructs alone, and no new declaration kind exists for it: a stateful model whose state is a context window's `list uint32`, whose step is the window's `push`, and whose contract's postcondition states that the pushed context fits composes under `scan` with its `satisfies_contract` evidence proved by the window's generated `push_bounded`, and an adapter is a model over `string` whose realization applies the elaborated `classify` of a classifier or `generate` of a generator, with `satisfies_contract` evidence proved from the generated laws of that classifier or generator; the closed list of language-1.2 declaration kinds is unchanged by it, an adapter whose realization does not meet its contract fails verification, and an adapter that applies a declaration of another kind is refused (`LLT4006`).
    Given a stateful window model stepping by `push`, and adapters over a classifier and a generator
    When they are composed under `scan` and verified by Lean, and again with a false contract
    Then the window model's evidence is proved by `push_bounded` and the adapters' evidence by the generated laws
    And the declaration kinds are unchanged and a false contract fails verification

  @LM-09 @build
  Scenario: No prompt is code: no member of the closed language maps `string` or `bytes` to a function, a term, or source — every operation of the primitive registry that takes a `string` or `bytes` operand is classified by the audit with a data result, the registry's schema enumeration and the production registry's primitive rows equal one another and equal that classified set, and the text declarations of `lexlean/semantic-module/2` state no `string`-typed member, so a prompt reaches a model only as the tokens `encodeText` makes of it; a planted operation or text member of that shape is caught by the audit with its name.
    Given the primitive registry, the production registry, and the text declarations of the closed schema
    When the audit classifies every operation that takes a `string` or `bytes` operand
    Then every one has a data result and the two registries agree
    And a planted operation or text member that maps a string to code is caught by name

  @LM-13 @build
  Scenario: The text laws a tokenizer elaborates through are LexLean source and not authored text: the module that states them is one language-1.2 native core module of the embedded `language/text-1.2` project, the project holds no file of Lean source, no independently authored Lean file anywhere in the release tree declares the `LexLeanText` namespace or lies under its module root, and the one-time migration input is absent; the registry lists only that module, the declarations and their exact axiom policies are read from its native data, which the compiler embeds and hashes into the language-1.2 semantics, and a module that declares a text component imports the generated module rather than splicing any Lean text.
    Given the embedded text library that states a tokenizer's laws, and the register
    When the release tree is searched for an independently authored implementation of those laws
    Then the native module is the only source of the laws and the library project holds no Lean file
    And a module that declares a text component imports the generated module rather than splicing any Lean text
