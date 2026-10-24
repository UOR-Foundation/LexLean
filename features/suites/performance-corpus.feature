Feature: performance-corpus

  The general performance corpus: the one registered object over which every acceptance criterion about the performance of compiled programs quantifies, its closed kinds, its members as production roots, and the check that no member is specific to a consumer (§28.7).

  @PC-01 @build
  Scenario: The performance corpus is the language-1.2 project examples/performance-corpus whose manifest corpus.toml (lexlean/performance-corpus/1) fixes the closed kinds integer-kernels, byte-parsing, long-folds, maps-and-sets, recursion, and compiler and one row per member with its identifier <kind>/<name>, its production root, the targets and effects the root declares, and its contract; every member is exactly one production root of the project declared as its row states, every kind but compiler has a member, and compiler states its precondition, that the compiler is itself a production root, and has no member while it does not hold; a manifest with an unknown field, an unregistered kind, a member that is not such a root or is declared otherwise, a kind without a precondition and without a member, or a kind with both fails the check naming the defect.
    Given the committed corpus project, its manifest, and the production roots the project declares
    When the manifest is read against those roots, and a manifest with an unknown field, an unregistered kind, a member whose root is not a production root, a member declared with other targets, a kind without a member, and a member of the compiler kind are each read in turn
    Then every member is one production root declared as its row states, every kind but compiler has a member, and compiler states its precondition with no member
    And each planted manifest fails the check naming its defect

  @PC-02 @build
  Scenario: No member of the performance corpus is specific to a consumer: the project declares no source package and no lexicon source that is not builtin or a path inside the project, no file of the corpus holds a network address or names a repository, and a planted package, lexicon source, address, or repository name fails the check; and every member exports through lexlean::api::production in every target it declares with status kernel-checked, one output per member and target, and the published bundle replays with status bindings-verified.
    Given the committed corpus project and every file of it
    When its configuration and files are checked for a source package, a lexicon source outside the project, an address, and a repository name, each is planted in turn, and every member is exported in every target it declares through the production façade
    Then the committed corpus passes and each planted defect fails the check naming it
    And every member exports with status kernel-checked, one output per member and target, and the bundle replays with status bindings-verified
