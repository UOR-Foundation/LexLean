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

  @RB-08 @build
  Scenario: A read of a binding that nothing after it reads is a move and a projection of a pair component that nothing after it reads moves the component out of the pair, every other read clones, and every clone a rendering keeps is one the ownership check requires; a list is a run of contiguous cells, and building or taking apart a cell is constant work on any list; a crate that reads or moves a binding after a component was moved out of it, or moves a component twice, is refused.
    Given every fixture program in each profile that renders it, and the rust-std lists package
    When each is lowered, every clone it keeps is made a move one at a time, a move is put back to a clone and the pass is run again, a component is moved out of a pair and the pair is then read, moved, or has the component moved again, and a harness builds and takes apart two hundred thousand cells and two lists on one tail
    Then every rendering is accepted, every clone made a move is refused by the ownership check, the pass makes the read a move again, each planted crate is refused naming ownership, and the cells count no work, overflow no stack, and both lists see their own items

  @RB-09 @build
  Scenario: Every name the Rust runtime takes from Rust's library is a row of its library table, each a machine primitive with a stated meaning, stated in the closed AST over primitives, or a construct of the closed AST, and every primitive is cited by a row; a runtime that names anything else, a row it does not use, a row citing a name that is not a primitive, and a primitive no row cites are refused.
    Given the runtime text of both profiles, its library table, and its primitives
    When the text is scanned for every name it imports, calls, or writes as a path, a macro, or a type, and the scan is repeated with a library function added to the text, a row removed, a row the text does not use added, a row citing a name that is not a primitive, and a primitive no row cites
    Then every name the runtime uses is a row and every row is used, and each planted text or table is refused naming what it broke

  @RB-10 @build
  Scenario: The scaffolding of a Rust package is data and not renderer text: the edition, whether the crate is published, the lint gates, and every allowed lint with its Clippy group and reason are read from the language-1.2 semantic module `RustPackage`, `#![no_std]` is the production target row's `allocation = false`, and `#![forbid(lint)]` restates each gate on rustc's lints at `forbid`; a renderer source holding a string of the module, a module that is not the shape the reader accepts, and a `Cargo.toml` or crate header that differs from the module are refused.
    Given the scaffold module, the production registry, and the sources of the renderer
    When the module is decoded, every package of both profiles is rendered, the renderer's files are scanned for the module's strings, and the scan and the decoder are repeated with a manifest constant, a lint table, and a header constant added to a renderer file, and with the module's declarations, vocabulary, and bodies altered
    Then the module decodes, `Cargo.toml` and the crate header are what the module and the registry give, no renderer file holds a string of the module, and each planted source or module is refused naming what it broke

  @RB-11 @build
  Scenario: Every lint a package allows states its Clippy group, none is in the `correctness` group, the module states the same as a theorem over the same list, which the reader checks by its shape and the module's project type-checks (`verify` replays its proof, this row does not), and the table of SPEC.md §17.16 equals the module's rows; an allowed lint in `correctness`, a specified row the module lacks, a module row the specification lacks, and a module without the theorem are refused.
    Given the scaffold module and the lint table of the specification
    When the module's rows are compared with the table, and the comparison and the decoder are repeated with a lint moved to `correctness`, a row dropped from each side, a row's group or reason changed, and the theorem removed or weakened, and the module's project is locked and checked
    Then the rows equal the table in order, no row is in `correctness`, the project checks, and each planted module or table is refused naming the row or the theorem

  @RB-12 @build
  Scenario: The language-1.2 compiler-semantics ID covers the package scaffold: changing a lint row, the edition, or a gate of a copy of the embedded tree changes it, writing the module in its data form or reformatting its notation does not, and the module is outside the identities of languages 1.0 and 1.1.
    Given the embedded tree and the files that §21.2 lists as introduced for language 1.2
    When the identities of the three languages are recomputed over the tree and over copies with a lint row, the edition, and a gate of the scaffold module changed, with the module written as data, and with its notation reformatted
    Then the recomputation of the unchanged tree equals each embedded identity, the language-1.2 identity differs for each changed copy and is equal for the rewritten ones, and the identities of languages 1.0 and 1.1 are equal for every copy

  @RB-13 @build
  Scenario: A package states the effects of its realization: `package.json` and `provenance.json` record them, ascending and registered, the production registry says that exhausting the stack aborts the process, and an unregistered or unordered list, or one that states allocation on `rust-core`, is refused.
    Given a recursive program, an overflowing program, and a plain program, each in both profiles
    When each is rendered with the effects its program shows, its provenance is read against the schema, and the render is repeated with an unregistered effect, an unordered list, and allocation on `rust-core`
    Then each rendering's provenance states exactly the manifest's effects, the registry's recursion statement says the process aborts, and each planted manifest is refused as LLB6005 naming the effect

  @RB-14 @build
  Scenario: A library template whose self calls are all tail calls is rendered as a loop over its accumulator that breaks with its result, so that the production rows of the primitives it realizes carry no `recursion`, while a template with a call that is not a tail call is rendered as a call and keeps the effect; a rendering that states a self call where a loop is possible, and a row whose `recursion` differs from its rendering, are refused.
    Given the library templates of the realization, each instantiated for the types the fixtures use
    When each is rendered in both profiles and its self calls are classified as tail calls or not
    Then every template whose self calls are all tail calls renders as a loop that breaks with its result, and its primitives' registry rows say `recursion = false`
    And every other template renders as a call and keeps `recursion = true`, and a template rendered with a self call it could have as a loop, or a row that disagrees with its rendering, is refused

  @RB-15 @build
  Scenario: The Rust AST mirrors the machine callee and the overflow literal of RustSyntax: a crate whose functions call each machine primitive at each width is checked, printed, and compiled by the pinned rustc with warnings denied, each call prints as the Rust operation its primitive names, and each function computes on the boundary values of its width exactly what an independent computation over a wider integer states; a machine call with another number of operands than its primitive takes, and an overflow value outside the payload of an `Err`, are refused as LLB6005, and a lowered crate that holds a machine call is refused by the correspondence between a rendering and its program as a construct with no row, because machine calls appear only in the runtime profile.
    Given a crate whose functions call each machine primitive at each width, and the planted crates of a call with another number of operands and of an overflow value outside an `Err`
    When the crate is checked, printed, and compiled by the pinned rustc with warnings denied, and its functions are run on the boundary values of the widths
    Then each call prints as the Rust operation its primitive names and each function computes what the independent computation states
    And each planted crate is refused as LLB6005 naming the break, and a lowered crate with a machine call put into it is refused by the correspondence as a construct with no row

  @RB-16 @build
  Scenario: The core runtime every Rust package carries is the text of a base and of the functions the language-1.2 module `RustRuntime` defines: the compiler project's module, shipped byte for byte at `language/rust-runtime-1.2/RustRuntime.lex.tex`, defines `coreCrate`, a crate of `RustSyntax` that the Rust crate decodes, never evaluates, and prints in place of the handwritten function of the same name, and each printed function agrees with that handwritten function in result and in counted work on the boundary and seeded values of its parameters when both are compiled by the pinned rustc with warnings denied, and the decoded crate is checked by the validator that checks every rendering (`validate` and the normative `validateCrate`), with the runtime's reading of its arithmetic rule, so that a function of the core runtime fails only with an `Err` of the overflow value whose error type is the node `RustType.overflow` and allocates nothing; a module that names a constructor `RustSyntax` does not declare, gives a constructor a wrong number of arguments, refers to a definition it lacks, defines a datum with parameters, or lacks `coreCrate`, a function that returns an `Err` of another value or another error type, a heap type in the core runtime, a shipped copy that differs from the compiler project's module, a defined function that no differential covers, and a function whose body is changed to compute another result are each refused.
    Given the committed module RustRuntime of the compiler project, its shipped copy, the handwritten core runtime kept as the oracle, and the planted modules
    When the module is decoded and printed into the core runtime, and the printed functions and the oracle's are compiled together by the pinned rustc with warnings denied and run on the boundary and seeded values of their parameters
    Then each defined function returns the oracle's result and counts the oracle's work, and the shipped copy is the compiler project's module byte for byte
    And each planted module and each function with another body is refused

  @RB-17 @build
  Scenario: The core runtime's support items are the definitions of a second closed module: the language-1.2 module `RuntimeSupport` of the compiler project, shipped byte for byte at `language/rust-runtime-1.2/RuntimeSupport.lex.tex`, defines `supportCrate`, a crate of `RuntimeSyntax`, whose items are of a closed set (a `use`, a unit struct with derives, a type alias with parameters, a static atomic counter, a trait of method signatures, an impl of a trait or of a type with bounded parameters, a generic function with bounded parameters), whose expressions are of a closed set (a variable, a literal, a path, a call of a path, a method call, a field of a pair, a borrow, an equality, a match), and whose types, paths, and methods name a declared item or one entry of a closed table of the standard library; the Rust crate decodes it, never evaluates it, checks it with a validator of its own, and prints it where the handwritten text stood, so that the core runtime holds no handwritten Rust text, and `equal`, `compare`, `tick`, and `work` agree with the handwritten ones in result and in counted work on the boundary and seeded values of the core profile when both are compiled by the pinned rustc with warnings denied; an item or expression outside the set, a name that is neither declared nor in the table, an impl that lacks a method of its trait or states another signature, a bound on a trait that is not declared, a heap type in the core runtime, a shipped copy that differs from the compiler project's module, and a changed body are each refused.
    Given the committed module RuntimeSupport of the compiler project, its shipped copy, the handwritten core runtime kept as the oracle, and the planted modules
    When the module is decoded, validated, and printed into the core runtime, and the printed and the handwritten runtimes are compiled together by the pinned rustc with warnings denied and run on the boundary and seeded values of the core profile
    Then equal, compare, tick, and work return the oracle's results and count the oracle's work, and the shipped copy is the compiler project's module byte for byte
    And each planted module is refused naming the break
