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

  @LM-13 @build
  Scenario: The text laws a tokenizer elaborates through are LexLean source and not authored text: the module that states them is one language-1.2 native core module of the embedded `language/text-1.2` project, the project holds no file of Lean source, no independently authored Lean file anywhere in the release tree declares the `LexLeanText` namespace or lies under its module root, and the one-time migration input is absent; the registry lists only that module, the declarations and their exact axiom policies are read from its native data, which the compiler embeds and hashes into the language-1.2 semantics, and a module that declares a text component imports the generated module rather than splicing any Lean text.
    Given the embedded text library that states a tokenizer's laws, and the register
    When the release tree is searched for an independently authored implementation of those laws
    Then the native module is the only source of the laws and the library project holds no Lean file
    And a module that declares a text component imports the generated module rather than splicing any Lean text
