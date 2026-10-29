Feature: mathlib

  An external Lean library, Mathlib in particular, is pinned in the configuration and the lock and acquired only by an explicit command (§10.4, §11.4, §23.4).

  @ML-01 @build
  Scenario: A language-1.2 project declares an external Lean library once, as a `[[lean_library]]` table of its `lexlean.toml` with a `name`, an HTTPS `url`, and an exact 40-lowercase-hex `revision`, sorted and unique by `name`, and the configuration reads back to the bytes written; a project without one keeps its bytes, and a URL that is not HTTPS, a revision that is not a 40-lowercase-hex commit, a branch, a tag, or any other field, a name that is not a package identifier, a repeated or unsorted table, and a declaration in another language are each refused with LLC0109.
    Given a language-1.2 project that declares a Lean library, and the same project edited in the thirteen ways the statement names
    When the configuration is parsed and written back, and a project without a library is parsed and written back
    Then the declaration reads back to the bytes written and a project without a library keeps its bytes
    And each edit is refused with LLC0109

  @ML-02 @build
  Scenario: The lock pins each declared Lean library by a `[[lean_library]]` row of its name, URL, exact revision, the SHA-256 of the library's own `lake-manifest.json`, and the SHA-256 of its generated glossary, the tree digest of the lexicon package `lean.<name>` of the project or of the empty tree when the project has none; the Lake workspace's `lake-manifest.json` must require the library at the declared URL and revision, rows sort by name, and the lock reads back to the bytes written; a project without a library keeps its lock bytes, and a library, a manifest digest, a glossary, or a workspace requirement that is not the one the lock pins is refused with LLC0110 naming both values.
    Given a language-1.2 project that declares two Lean libraries and a Lake workspace that requires them, both present as `lake exe cache get` leaves them
    When the project is locked, locked again, and checked, and the library, its glossary, and the Lake requirement are each changed
    Then the lock holds one sorted row per library with the revision and both digests and reads back to the bytes written
    And each change is refused with LLC0110 naming both values

  @ML-03 @build
  Scenario: `lexlean deps fetch` is the only command that acquires a declared Lean library: it checks the declaration against the Lake workspace, runs `lake exe cache get` in the Lake workspace under the pinned toolchain through the checked child runner, and confirms the library is present at the declared revision with its built artifacts; with no library declared it fetches nothing and succeeds, a missing or failing `lake` and a library that is not at the declared revision afterward are LLV7020, and the command writes nothing of the project outside the Lake workspace.
    Given a language-1.2 project that declares a Lean library its Lake workspace requires and that is absent, and a `lake` that behaves as `lake exe cache get` does
    When `lexlean deps fetch` runs, then with a failing, a missing, and an unproductive `lake`, then in a project that declares no library
    Then the library is present at the declared revision, the project locks and checks offline, and nothing outside the Lake workspace was written
    And each failure is LLV7020 with exit 3, and a project without a library fetches nothing

  @ML-04 @build
  Scenario: `lock`, `check`, `build`, `fmt`, and `verify` never fetch a declared Lean library: when the library is not present in the Lake workspace at the declared revision with its built artifacts, each fails offline with LLV7019 naming the library and `lexlean deps fetch`, before any Lean process runs and without a network connection.
    Given a language-1.2 project that declares a Lean library that is absent, at another revision, or without its built artifacts
    When `lock`, `check`, `build`, `fmt`, and `verify` each run with a `lake` on the path that would fetch
    Then each fails with LLV7019 naming the library and `lexlean deps fetch`
    And no backend output is published and the `lake` was never run

  @ML-05 @build
  Scenario: `fmt` rewrites the sources of a project that declares a Lean library: the scratch copy in which the formatter proves preservation of the linked IR holds the library's presence markers (the checkout's `HEAD`, its manifest, and the directory of built artifacts) and not the library, so a present library is recognised without being copied and an absent one is still LLV7019.
    Given a language-1.2 project that declares a Lean library, present in its Lake workspace, whose entrypoint has CRLF line ends
    When fmt runs
    Then it rewrites the entrypoint to its canonical LF form and fails nothing
    And the project directory gains no library
