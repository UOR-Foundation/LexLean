Feature: runtime

  The runtime of generated Lean as native modules, and the frozen runtime text of languages 1.0 and 1.1 (§17.12).

  @RT-01 @build
  Scenario: The runtime of language 1.2 is LexLean source: every module of its registry is one native core module of the runtime's own project, which holds no file of Lean source, the registry lists only the modules in dependency order, every declaration pins its exact axioms, and a source that is missing, weakened to an inexact policy, or imports anything but `Init` and an earlier runtime module is refused; the one-time migration input is absent, no native source names a runtime copy inside a module, and no Lean file declares a runtime namespace but the released output of a language-1.0 or 1.1 project.
    Given the runtime project, its registry, and the native modules the compiler embeds
    When the project's files, the registry, and every native source of the repository are read
    Then every listed module has exactly one embedded native source, the project holds no Lean file, and the registry names no declaration
    And a native source that is missing, weakened to an inexact policy, or imports outside the runtime is refused
    And no native source names `<module>.LexLeanRuntime.x`, and only the released output of a frozen language declares a runtime namespace

  @RT-02 @build
  Scenario: The runtime text of languages 1.0 and 1.1 is the one declared exception: the portable runtime and the decoder of core module 1 are the released bytes, held as data under `language/runtime-1.2/frozen/`, read by the language-1.0 and 1.1 backends alone, excluded from the language-1.0 and 1.1 identities, and reproduced byte for byte by the released language-1.1 and 1.0 output; a one-byte change of either is not the released output.
    Given the frozen runtime bytes the compiler embeds and the committed language-1.0 and 1.1 goldens
    When each frozen text is compared with the released modules that splice it
    Then each is embedded as it is on disk and appears verbatim in the released output
    And a one-byte change of either does not, and the language-1.0 and 1.1 identities exclude them

  @RT-03 @build
  Scenario: A verification compiles every runtime module that a project's generated modules, or the calculus modules its certificates are checked against, import before the project's modules, replays it with `leanchecker`, and audits each of its declarations against the axioms its row pins; the committed verification of a project that uses the collections prints them, the runtime's own project verifies under the pinned Lean, and a row that pins another axiom set than its declaration observes fails with LLV7005.
    Given the committed verification of the collections example and the runtime's own project
    When the runtime project is verified, and verified again with one row pinning an axiom its declaration does not use
    Then the committed audit names the runtime's declarations and the runtime project verifies
    And the planted row fails with LLV7005

  @RT-04 @build
  Scenario: No Lean runtime text remains in the language-1.2 compiler: a generated language-1.2 module imports the runtime modules it uses and splices none, and the only string literal of the compiler's source that holds Lean declarations under a namespace is the decoder of core module 2, which loads native modules and so cannot be one; a planted runtime text constant, raw or escaped, is found.
    Given the source of the compiler
    When every string literal outside its unit tests is read for Lean declarations under a namespace
    Then the only one is the decoder of core module 2
    And a runtime text constant planted in the semantic backend, raw or escaped, is found
