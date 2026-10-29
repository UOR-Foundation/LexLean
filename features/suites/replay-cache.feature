Feature: replay-cache

  A replayed declaration is keyed by a content-addressed DigestSet over its canonical serialisation and its dependencies' keys, so a warm replay reuses what an unchanged key already holds (§22.13).

  @RC-01 @build
  Scenario: A replay key is a PrismPM ADR-015 DigestSet, a `sha256` and a `blake3` of the same bytes in lowercase hexadecimal, labelled `blake3:` and the 64 digits of the `blake3` value, over the one canonical serialisation of a declaration's name, universe parameters, kind attributes and type, the DigestSet of its value or proof term, the keys of the declarations it names, the Lean toolchain, and the Mathlib revision; equal inputs give equal keys, and a change to any one of those inputs gives a different key.
    Given declarations with a name, universe parameters, kind attributes, a type, a value, and dependencies, and a toolchain and a Mathlib revision
    When their keys are computed, and then computed again after each input is changed in turn
    Then the key is a DigestSet whose two digests cover the same bytes, its label is `blake3:` and the blake3 digits, equal inputs give equal keys, and each changed input gives a different key

  @RC-02 @build
  Scenario: A declaration changed in a dependency graph changes the key of every declaration above it and of none beside or below it, declarations that name one another, an inductive type with its constructors and recursor, are keyed as one group, and facts that repeat a declaration, name a declaration neither declared nor anchored by a given key, or do not parse are refused (`LLV7020`).
    Given a dependency graph of declarations with a shared base, two branches, and a mutual group
    When one declaration is changed and the keys are computed again, and facts that repeat, dangle, or do not parse are read
    Then exactly the declarations above the changed one have new keys, the members of the group have one group digest, and each malformed set of facts is refused with `LLV7020`

  @RC-03 @build
  Scenario: A warm replay reuses the recorded result of every unit whose key is unchanged and replays only the others, whether a unit is one declaration or one module; only a replay that succeeded is recorded, each entry names the digest of the executable that produced it, and an entry that an executable other than the current one produced is not reused.
    Given a cache directory and units keyed by declaration and by module
    When the units are replayed cold, again unchanged, again with one unit changed, with a failing replay, and with another executable
    Then the unchanged run replays nothing, the changed run replays only the changed unit, a failed replay leaves no entry, every entry names its executable, and the other executable replays every unit

  @RC-04 @build
  Scenario: A cache entry whose bytes changed while its key was kept, an entry stored under a key other than the one it records, and an entry that does not parse as an entry, are each rejected with `LLV7019`, never reused, and not silently replaced.
    Given a cache directory holding an entry for a key
    When the entry's result is edited without its key, the entry is copied under another key, and the entry is truncated
    Then each lookup is refused with `LLV7019`, the replayer is not called, and the entry file is left as found

  @RC-05 @build
  Scenario: A changed Lean toolchain or a changed Mathlib revision gives every unit a new key, so no result recorded under the previous environment is reused, and the entries of the previous environment are left unread and intact.
    Given a cache filled for one toolchain and one Mathlib revision
    When the same declarations are replayed under another toolchain, under another Mathlib revision, and under the first environment again
    Then the first two runs replay every unit and reuse none, and the third reuses every unit from the entries the first run wrote

  @RC-06 @build
  Scenario: Facts exported by the pinned Lean from compiled modules, keyed and replayed through the pinned `leanchecker` cold and then warm, reuse every module whose key is unchanged: after a one-line edit to one declaration, only the modules whose declarations reach it are replayed again, and the warm verdicts equal the cold ones.
    Given three compiled Lean modules in a chain, the last of which does not use the declaration the edit will change
    When they are exported, keyed, and replayed cold, and again after a one-line edit to one declaration of the middle module
    Then the warm run replays the edited module and no other, and the verdicts of the warm run equal those of a cold run of the edited modules
