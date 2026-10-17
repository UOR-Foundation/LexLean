Feature: packages

  Source packages: a project imports the modules of a locked, content-addressed package of its workspace (§10.5).

  @PK-01 @build
  Scenario: A language-1.2 project that declares a source package by its identifier, version, kind `path`, and the member directory a `lexlean-workspace.toml` lists locks it by the tree digest of its manifest and source files, its manifest digest, its version, its module prefix, and its own packages, and the configuration and the lock read back to the bytes written; a project without a source package keeps its bytes, and a package of another kind, a source package in another language, a changed byte or version (LLC0105), an undeclared package (LLC0106), a module prefix claimed twice (LLC0107), a project or a package outside the workspace (LLC0108), a path that escapes it, and a symlink (LLS8001) are each refused with the named code.
    Given a workspace of an importing project and a package, and the lock that pins the package
    When the configuration and the lock are written, read back, and locked again, and the package, the workspace, and the configuration are each edited in the six ways the statement names
    Then the bytes read back equal the bytes written, and a project without a source package locks to the bytes it locked to before
    And each edit is refused with the code the statement names, naming the package and both digests for a changed byte or version

  @PK-02 @build
  Scenario: A module of a project imports only modules of its own project and modules of the packages the project declares, and a module of a package only modules of itself and of the packages its own manifest declares: a package module is a module of the importing project under its own module prefix, a module of a package the project does not declare fails with LLR3009, and an import cycle through a package fails with LLR3003.
    Given an importing project, a package, and a package that imports a second package
    When the project is checked, then with the import of the second package moved to the project, then with a module importing a package it does not declare, then with a cycle through the package
    Then the closure links, and each package module is a module of the project under the package's module prefix
    And the undeclared import fails with LLR3009 and the cycle with LLR3003

  @PK-03 @build
  Scenario: An importing project builds and verifies under the pinned Lean offline, and its package modules are generated, elaborated, replayed with `leanchecker`, and audited for their exact axioms under the package's module prefix with the package as their origin in the records; the source ID of the project changes with the pins, and no file of the package is written by `fmt`, `build`, or `verify`.
    Given an importing project and a package beside it in a workspace
    When the project is formatted, built, and verified with the pinned Lean, and again with one byte of the package changed
    Then the verification records name the package modules under its prefix with the package as their origin, the exact axioms of each declaration are audited, and no file of the package changed
    And the changed byte is refused before any Lean process runs

  @PK-04 @build
  Scenario: A production root of the importing project whose closure reaches a definition of a package certifies and exports over that closure as it does for a definition of its own module, the `production:` roots of a package are not roots of the importing project, and the importer publishes no certificate of the package's own.
    Given an importing project with a production root that applies a definition of a package whose own modules declare production roots
    When the project is checked, built, and its root is certified and exported with the pinned Lean
    Then the root's closure includes the package definition and the export is the package of that closure
    And no root of the package is a root of the importing project and the export holds no certificate of the package's own roots

  @PK-05 @build
  Scenario: The `bootstrap/` pilot imports the standard library's `Text` module through a source package of the workspace instead of holding a copy: its configuration declares `stdlib` as a source package, `bootstrap/src/` holds no `Text` module, `check` links `LexLeanStd.Text` as a module of the project whose definitions are ordinary ones, so that the project's production roots remain `parseAuditOutput` and `isClosedLeanName`, and neither the case of `BS-01`, SPEC.md §17.23, nor `bootstrap/README.md` keeps the copy or its equivalence assertion.
    Given the bootstrap project, the standard library, and the workspace file of the repository
    When the bootstrap project is checked and the case of BS-01, SPEC.md §17.23, and the project's README are read
    Then the configuration declares the standard library as a source package and the project holds no Text module of its own
    And the standard library's Text module is linked as a module of the project, the project's production roots are its own two, and no copy or equivalence assertion remains
