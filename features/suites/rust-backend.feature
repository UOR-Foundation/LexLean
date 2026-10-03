Feature: rust-backend

  The canonical Rust backend: the closed Rust AST, its checks, packages, and provenance (§17.16).

  @RB-01 @build
  Scenario: Every Rust rendering is built as a closed AST each of whose constructs carries the calculus element it realizes, every correspondence row and every AST node is exercised by a run package, and a construct whose row, width, or program does not justify its element is refused.
    Given every fixture program in each profile that renders it, and every committed package
    When each is lowered to the closed Rust AST and each construct is compared with the row of the element it carries
    Then every construct realizes its element at its width, every table row is exercised, and a run package emits every AST node
    And a construct whose element the program lacks, a nat_mul lowered from nat_add, and a u16 addition lowered from a u8 one are refused

  @RB-02 @build
  Scenario: An exported name that is not a lowercase snake-case identifier, is a Rust keyword, imitates a generated name, or is declared by the runtime, a name exported twice, and an unavailable crate name each fail with LLB6005, and a crate binding one name twice in a function is refused.
    Given the committed negative manifests whose exported names or crate name collide
    When each is packaged, and a lowered crate is planted with one binding repeated
    Then each fails with LLB6005 and its stated identifier collision
    And the planted crate is refused for binding a name twice

  @RB-03 @build
  Scenario: An export that copies a parameter whose type is not Copy fails with LLB6005, and a crate that moves a value twice or reads it after moving it is refused.
    Given the negative manifest copying a list parameter, and lowered crates
    When the manifest is packaged and each crate is planted with an extra move of a match scrutinee
    Then the manifest fails with LLB6005 and its stated ownership mismatch
    And the planted crates are refused for moving a value twice and for reading it after the move

  @RB-04 @build
  Scenario: A package whose boundary holds a function value at any depth, whose program computes a value of a type no value inhabits, or whose rust-core program needs the heap fails with LLB6005, and a rust-core crate naming any heap type, runtime function, or construct is refused.
    Given the negative manifests with a function value at the boundary or in a record there, a value of an uninhabited type, or a rust-core program that needs the heap
    When each is packaged, and rust-core crates are planted with a string type and a heap runtime function
    Then each fails with LLB6005 and its stated reason
    And the planted crates are refused as hidden allocation

  @RB-05 @build
  Scenario: A function that can overflow returns R<T> and every call to it propagates, any other returns its value, an export whose declared errors differ from its function's fails with LLB6005, and a crate that drops, invents, or misreturns a failure is refused.
    Given every fixture program and the negative manifests misstating errors
    When fallibility is analysed, the manifests are packaged, and crates are planted with a dropped, an invented, and a misreturned failure
    Then every entry whose outcome is overflow is typed fallible and each manifest fails with LLB6005 and its stated arithmetic mismatch
    And every planted crate is refused

  @RB-06 @build
  Scenario: Every committed package builds offline under its declared gates, rustc warnings and Clippy's default lints denied with ten documented exceptions, the export of each one with an observable outcome, called from a separate crate, prints exactly the denotation's observable outcome, and so does every primitive instance on its boundary and seeded inputs; a planted lint and a planted semantic mutation are detected.
    Given every committed package, a harness crate calling its export, and the primitive differential of each profile
    When the packages and harnesses are built offline in one workspace, the packages are linted, and each harness of an observable outcome runs
    Then every package passes its gates and every harness prints the denotation's observable outcome on every input
    And a package with a clone of a Copy value fails its lint gate and a wrapping subtraction changes the printed outcome

  @RB-07 @build
  Scenario: Packages are deterministic and content-addressed: renderings by two separate processes under different roots and environments are byte-identical to each other and to the committed package, every manifest and provenance validates against its schema, the provenance binds the SHA-256 of each file, the program identity, the runtime LexLean's semantics records, the sources, and the language-1.2 compiler-semantics ID, and every committed package's sources are the semantic ID of the verified build stating its program.
    Given every committed package and its manifest, and the published verified build of the compiler project
    When two separate processes with different working directories and environments render every package, and each manifest is validated
    Then the renderings are identical to each other and to the committed bytes, and every manifest and provenance is schema-valid
    And the provenance binds each file's SHA-256, the program identity, the recorded runtime, the sources, and the compiler-semantics ID, and the sources are the verified build's semantic ID while a forged one is refused
