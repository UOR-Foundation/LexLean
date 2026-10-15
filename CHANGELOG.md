# Changelog

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and
the version axes are the ones SPEC.md §30.1 separates: the compiler crate and
binary carry the SemVer below, each project selects the supported language
identifier (`1.0`, `1.1`, or `1.2`), and each language's compiler-semantics ID is a
digest over its normative language data, schemas, and pinned golden fixtures.

SPEC.md §2.3 fixes `0.1.0` as the initial implementation version and `1.0.0` as
the first release satisfying the complete specification. No tag before `1.0.0`
is therefore a §30 release: `cargo xtask release-check` reads the complete
§30.3 artifact set and §30.4 completion criterion and refuses, naming every
criterion that does not hold. That refusal is the accurate answer at these
versions, and the entries below say what each tag does and does not claim.

## Unreleased

- Certificates write a list of any length as a term Lean elaborates without
  binders (SPEC.md §17.17, `SP-13`, #97; no new diagnostic code). Lean's
  `[…]` macro splits a literal of more than 32 elements in halves under
  `let` binders, and certificate B's witnesses, whose elements Lean
  postpones to metavariables, did not unify through them: a valid
  certificate B of a root calling a function of 33 or more parameters was
  rejected with `LLV7015`. One helper (`production::lean_list`) now writes
  every list and array of the three certificate writers: a literal up to 32
  elements, byte-identical to before, and the `List.cons` chain the macro
  writes for a short one beyond that. The 100-parameter project of
  `tests/negative/certificate-heartbeat-rejected` verifies without its
  shim. The certificates of `examples/models` and `examples/reasoning` that
  hold such a list change, and their verification records are regenerated,
  as are the sizes `certificate-resource-exhausted` and
  `certificate-generation-limit` quote.
  `SP-13` brings the register to 380 IDs.
- The language-1.2 authoring notation (SPEC.md §17.21, `AU-01`..`AU-03`; stage 3
- Core modules in the authoring notation (SPEC.md §17.21.8, `AU-09`..`AU-12`;
  stages 3c and 3d of #93). A native `coremodule` of schema `lexlean/core-module/2`
  can be written as text instead of one `\coredata{…}` argument of canonical
  JSON: an `imports` line, the declarations (`theorem Name.{u} : @41 := @77 {…}`,
  the other members in the generic notation), and the table of shared
  expressions, one statement per node that is not written inside the node that
  uses it (`@14 := fun (n : @12) => Nat.succ n`), in a Lean-like syntax of
  explicit arguments, universe levels, and binders. The numbering of the nodes
  is the order of the text, so the notation reads back to the module, and the
  canonical JSON it reads to is byte for byte the data the module had: the
  committed libraries (`compiler/proofs`, 13 modules, and
  `language/preservation-1.2/library`, 13 modules, 48 MB of data) are written in
  the notation at about three quarters of the size, and the compiler-semantics
  ID of language 1.2 (`464235ad…`) is unchanged, because a source of the
  embedded tree written in the notation contributes its data form (§21.2). The
  Atlas (language 1.0, schema 1) stays as `\coredata`: languages 1.0 and 1.1
  accept the data form only, and the notation under either is `LLP2003`. `fmt`
  writes the notation, `lexlean::sdk` reads, writes, and assembles it
  (`ModuleKind::Core`), and the core backend's coverage names
  `lexlean.core::coresource`. No diagnostic code is new. A bound variable is
  written by the name of its binder when no nearer binder has the name, a
  constant a binder in scope is named like is written `const "…"`, and a
  statement may be written `@name := …` and named `@name`, which `fmt` numbers
  (`AU-12`; stage 3d of #93): a person adds a theorem to a library by writing
  its declaration and the statements of its type and value, with every
  argument and level explicit, as a kernel term has them, and counting nothing.
  The certification library gains the first one written that way,
  `LexLeanCertification.authored_eq_symm` (symmetry of `Eq` by `Eq.rec`), which
  the pinned Lean's kernel checks with the rest of the library. Reading the identity of
  language 1.2 now reads the embedded libraries from the notation, on as many
  threads as there are cores: 0.8 s on four cores in a release build, and
  nothing for language 1.0 and 1.1.
- The authoring API (`lexlean::sdk`, SPEC.md §24.8, `AU-08`; stage 3e of #93):
  `read_module`, `write_module`, `to_data_form`, `to_notation`,
  `declaration_spans`, and `assemble` read, rewrite, convert, and assemble the
  `.lex.tex` source of a semantic or tooling module without a project, with the
  data as canonical JSON and the offset of an error in the source. The
  formatter, the identity of the embedded tree, and the conformance suite's own
  editing of committed sources are written over the same table of module kinds.
  `AU-07` is the permanent gate that no writer of the repository emits a
  product source but five named records (the four fixture modules of
  `compiler/` and the extraction adapter's module, each generated from an input
  that is not a source); it is shown to fail on a planted writer output.
- The calculus and the Rust machine are source (SPEC.md §17.14; stage 3e of #93).
  `compiler/src/{TargetSyntax,TargetSemantics,TargetOracle,RustSyntax,RustSemantics,Main}`,
  the reasoning oracles, and the project configuration are written and edited as
  `.lex.tex`: the Rust builders that rendered them (`calculus_source`,
  `rust_source`, 5,300 lines) and the check that compared them with their own
  output are removed; so are the builders of the GNAF model and the
  certification model (`gnaf_model`, `certification_model`, 7,000 lines), whose
  committed `Gnaf` and `Certification` are source too. `check-calculus` still compares the generated files (the
  fixtures and the modules that state them, the Rust packages, the shipped
  copies of the calculus modules), and `conformance_sp_07` now reads the
  machine's runtime items and their failure and heap classes from the committed
  `RustSyntax` and `RustSemantics`, and the compiler project's configuration
  from the production example. No file of the repository changes by this: it
  is a change of who owns them.
- The language-1.2 authoring notation (SPEC.md §17.21, `AU-01`..`AU-08`; stage 3
- Certificate budgets (SPEC.md §22.6, `SP-02`, no new diagnostic code).
  Every certificate now lifts both of the pinned Lean's heartbeat budgets, as
  only the certificates of a program with a match of more than 100 arms did,
  and certificate B declares its `program` and `krate` terms `noncomputable`.
  A root whose program carries its parameters as data (`UOR-Foundation/MNIST`
  at its model root: 243 KB of crate, int_tensor artifacts decoded into
  literals) was refused twice by a valid certificate: the code generator
  exhausted its 200 000 heartbeats compiling the crate term, which nothing
  runs, and then `isDefEq` exhausted the same budget on the function holding
  the literal. The bound that remains is the wall clock, `child_timeout_ms`,
  as the specification already said. Every certificate text changes by its
  header, so the committed certificate records of the examples and the
  fixture expectations that quote certificate sizes are regenerated. The
  fixtures `certificate-heartbeat-rejected` and
  `certificate-resource-exhausted` model an exhausted Lean by a substituted
  `lake` that set a budget of one heartbeat after `autoImplicit`; it now
  overrides the lifted budget instead. With the budget lifted, their
  100-parameter project reaches certificate B for the first time, which
  refuses it for a reason of its own (#97): Lean elaborates a list literal
  of more than 32 elements under `let` binders, and a certificate witness
  with postponed holes in such a list does not unify.

- The language-1.2 authoring notation (SPEC.md §17.21, `AU-01`..`AU-03`; stage 3
  of #27, #93). A `semanticmodule` can be written as readable text instead of
  `\semanticdata{…}` JSON: declarations in a fixed grammar, read from the
  schema, with readable forms for types (`(List Nat)`, `(A * B)`), terms
  (`(let x : Nat := 1; (x + x))`, `(match s | C x => …)`) and members
  (`Module::name`), and a generic form `kind{…}` for every other value. The
  notation and the canonical JSON are in bijection, so a module in the notation
  has the semantic ID and the generated Lean and LaTeX of its data form, and
  `fmt` writes the notation canonically and writes a module out of its data
  form (`fmt --check` reports a language-1.2 module still written as data as
  `LLL1003`). Every language-1.2 source of the repository is written in the
  notation: the compiler project, the language libraries, the examples, the
  fixtures, and every refused fixture the notation can spell (287 sources;
  `AU-04` keeps the data form to language 1.0 and 1.1 and to refused fixtures
  that test the data form's own refusal, and the conformance helpers that edit
  a module's data read and write it in whichever form the source has). No
  diagnostic code is new: a malformed text is `LLP2001`. Compiler-tooling
  modules (`toolingmodule`, §17.20) are written in the generic notation of the
  same family (one object without its `spec`, declarations as `type{…}`,
  `call{…}`, `function{…}`), `fmt` writes them out of `\toolingdata`, their
  four committed sources (the embedded extraction adapter and three fixtures)
  are migrated, and a source map names each declaration's text (`AU-05`). A
  source of the embedded tree written in the notation contributes its data form
  to the compiler-semantics ID (§21.2, `AU-06`), so moving the adapter's source
  to the notation changes no identity. Nothing a build or a
  verification derives from a module changes, so no compiler-semantics ID, no
  generated Lean or LaTeX byte, and no verified record changes; the source IDs,
  the source maps, the coverage records, and the spans of the fixtures'
  diagnostics, which name the source's own bytes, are regenerated.
- The verifier's resource profile gains scheduling hints and a memory budget,
  and the gate's `examples` leg uses it (SPEC.md §9.4, §22.11, `RP-14`, no new
  ID or code; LexLean #83). `LEXLEAN_VERIFY_HINTS` names a JSON object of
  per-module `wall_ms` and `peak_rss_kib`, which start the heaviest processes
  of a stage first; `LEXLEAN_VERIFY_MEMORY_KIB` stops a worker from starting a
  process whose expected peak would push the running total past the budget, and
  a process whose expected peak alone passes it runs alone. Neither enters an
  artifact or an identity; the published directory is byte for byte that of
  width 1 (`VR-21`). `cargo xtask ci-hints` derives the hints from a run's
  telemetry. In `ci.yml`, the `examples` leg verifies at width 2 with
  `.github/verify-hints.json` and a 12 GiB budget, read from its matrix row,
  and the other leg names no profile; `release.yml` still runs `just vv` at
  width 1. On the standard runner's 16 GB, 22 of Atlas's 66 modules peak above
  8 GiB, so the modelled gain is small: about 13 min of Atlas's 169, from main's
  telemetry. The CI run of this change is the measurement.
  `fmt` writes the notation canonically (a module written as data stays as it
  is). No diagnostic code is new: a malformed text is `LLP2001`, and nothing
  embedded in the compiler-semantics ID changes.
- Rust runtime library boundary (SPEC.md §17.16, `RB-09`, no new
  diagnostic code), stage 1 of #91. `rust_library::PRIMITIVES` states the
  machine primitives the runtime may use, each by its meaning, and
  `rust_library::LIBRARY` lists every name the runtime text takes from
  Rust's library with its role: part of a primitive, stated in the closed AST
  over primitives, or a construct of the closed AST. The audit reads the
  text and refuses a name outside the table, a row the text does not use,
  a row citing a non-primitive, and a primitive no row cites. The runtime
  text is unchanged, so no runtime digest, package, or semantic ID changes.
- `rust-std` rendering: ownership and lists (SPEC.md §17.14, §17.16, `RB-08`,
  no new diagnostic code). A read of a binding that nothing after it reads
  is a move and a projection of a pair component that nothing after it
  reads moves the component out of the pair (`rust::moves`, run after
  lowering and before validation); every other read clones, as before. The
  ownership check refuses a component moved out twice and a binding read or
  moved whole after a component left it. A list is a run of contiguous
  cells, extended in place when no other list holds its buffer and
  fronted by a single cell otherwise, so `cons` and `uncons` stay constant
  work on every list, no buffer is written while another list can see it,
  and a list built by one consumer is one buffer; every runtime loop still counts what it
  walks and the counted work of every fixture is unchanged. `RB-08` shows
  every clone a rendering keeps is one the ownership check requires (each
  one, made a move, is refused), that the pass makes a last read a move
  again when it is put back, the three refusals, and the runtime on two
  hundred thousand cells and two lists on one tail. The renderer and
  `rust-std` runtime records of `language/semantics-1.2.toml` change
  (`rust_backend` 3), so the language-1.2 compiler-semantics ID changes,
  every committed package is re-rendered, and every verified example's
  records are regenerated. Measured on `UOR-Foundation/MNIST` (LexLean #81),
  the certified `rust-std` root goes from 28.7 ms to 7.7 ms per image
  with the same predictions and the same counted work.
- The certification proof library is LexLean source (SPEC.md §17.18, `GC-09`,
  `GC-11`; stage 2b of #27, #79). Its 13 hand-written Lean modules (Git tree
  `298c8f97` at `ba52c601`: 97,597 bytes, 107 registered declarations) are now 13
  native core modules of `lexlean/core-module/2` in an ordinary closed project,
  `compiler/proofs/`, which holds no file of Lean source. `library.toml` lists
  only the modules; the declarations (3,957 audited by name of 4,396) and their
  exact axioms are read from the native data, and `conformance_gc_09` compiles
  and audits the Lean generated from them. The migration is lossless
  (`tests/certification-library/MIGRATION.md`); the library is repository
  evidence, so no compiler-semantics ID changes. `GC-11` brings the register to 366 IDs.
- The preservation proof library is LexLean source (SPEC.md §17.10, §17.17,
  `SM-31`, `SP-12`; stage 2a of #27, #79). Its 13 hand-written Lean modules
  (Git tree `50f70bb5` at `ba52c601`: 525,026 bytes, 934 registered
  declarations) are now 13 native core modules of a second schema,
  `lexlean/core-module/2` (`schemas/core-module-v2.schema.json`, language 1.2
  only): annotation nodes, extended names (numeric components and escapes),
  definition safety and groups, compiled and reducibility state, and the
  matcher, projection, no-confusion, simplification, equation-record,
  compiler-inlining, sparse-`casesOn`, and `grind` state, decoded by one fixed
  generic runtime through Lean's checked declaration API (a sparse `casesOn` is
  remade by the elaborator's own construction in a realization of its type, since
  its record is private to that construction). A native core module is bounded in
  expression depth by `max_scope_depth` (`LLS8002`), is rendered on a stack sized
  for it, and is exempt from the module-name capture rule, since it writes no name
  relative to the project; the LaTeX document states each row's elaborator state.
  The library is an ordinary closed project
  (`language/preservation-1.2/library/`, with a title glossary): it locks,
  checks, and builds, and `LexLeanPreservation`, like `LexLeanTarget`, may be a
  project's module prefix when the project has no production root.
  `library.toml` lists the modules only: the 9,247 declarations carry their exact
  axioms (what `#print axioms` prints, which for the members of a reference cycle
  is the pinned Lean's order-dependent answer), 7,740 of them are audited by name
  where 934 were, all 934 with unchanged axioms. The statement vocabulary SPEC.md
  quotes is what Lean prints of the generated library in the committed audit
  output; the certificate-B rule set is read from the native data;
  `check-reproducibility` builds the library project twice. The exporter is the
  oracle of a checkpoint commit (`d54dbf85`) and is absent from the release
  tree; `tests/preservation-library/MIGRATION.md` records the equivalence
  evidence: re-exporting the regenerated environment gives the same 9,247
  declarations, equal as kernel terms (313 sparse `casesOn` rows up to the names
  of bound variables), and an attribute probe over both environments prints equal
  outputs. Languages 1.0 and 1.1 are byte-identical; every language-1.2 identity
  changed.
- Compiler-tooling modules (SPEC.md §17.20, `TL-01`..`TL-11`, `LLT4013`,
  `LLT4014`, `LLT4015`; stage 1a of #27): a third module form beside the
  ordinary declaration environments and the native `coremodule` and
  `semanticmodule`, `\begin{toolingmodule}\toolingdata{...}\end{toolingmodule}`,
  canonical JSON of the closed schema `lexlean/tooling-module/1`
  (`tooling/schemas/tooling-module.schema.json`), language 1.2 only, accepted only in a
  project the compiler embeds (`Project::with_tooling`) and refused with
  `LLT4013` in every user project, request of the public API, and command.
  It is a program over a typed registry of the external Lean interface (types
  with exact constructor lists, index requirements such as `LetValue .pure`,
  abbreviations such as `CommandElab`; calls with exact structured signatures
  including binder kinds, `optParam` defaults, and the empty structure
  instance; plumbing; each pinned to the toolchain source file and its
  SHA-256), a closed term language with explicit binder types and type
  arguments (literals, calls, constructors, fields, `let`, lambda, application,
  `if`, and a flat `match` that is exhaustive at the scrutinee's index),
  licensed `partial` and mutual recursion, a *complete traversal* check (every
  field of a recursive family is recursed into through a handler applied to a
  value derived from the field, so a registry that gains a constructor or a
  field makes the tool fail to link and never silently omit it), and command
  registration with invocation and evaluation as semantic data. Both backends
  derive from the one linked value, with no member that holds Lean text; the
  generated module carries one source-mapped piece per declaration and
  complete output coverage. Three committed fixtures under `tests/tooling/`
  (the term language, a command with its elaborator, invocation and
  evaluation, and a mutual traversal of the real `Lean.Compiler.LCNF.Code`
  family) are elaborated by the pinned Lean without a message; their published
  build trees are goldens compared by `cargo xtask check-golden` and rebuilt in
  two directories by `check-reproducibility`. 106 planted defects, each changing
  one member of a fixture, are refused with the registered code and reason, and
  two registries that misstate an arity or a constructor list consistently with
  their tool, and an invocation of a constant that does not exist, are refused
  by Lean, not by the compiler. Every single-node change
  of a fixture is accepted or refused by a registered code without a panic. The
  schema lies in `tooling/schemas/`, outside `schemas/`, so that no change of
  the tooling language alone changes an identity. The register has 350 IDs and
  the model 74 codes.
- The named-root extraction adapter is the first program written in the tooling
  language (SPEC.md §22.10, `NE-04`; stage 1 of #27, with the entry above).
  `language/lcnf-1.2/extract.lean`, handwritten Lean, and
  `language/lcnf-1.2/authority.toml`, its handwritten registry, are replaced by
  the project `language/lcnf-1.2/tooling/`: one tooling module whose registry is
  Lean's own answer, derived by `tests/lcnf/export.lean` from the pinned
  toolchain (117 types, 109 calls with exact signatures and source SHA-256, 171
  plumbing constants), and whose 65 functions are terms of the closed language.
  The module is generated by `cargo xtask check-calculus` from its authoring
  form, as the calculus is, so an edit of it is refused. The driver of verification no
  longer splices Lean text: it appends the signature probes, the registry
  check, and the evaluation of the roots to the module as typed declarations,
  has the compiler check them against the module's registry, renders the
  module, and reads Lean's messages by where the compiler placed each
  declaration, not by line numbers. The handwritten adapter is kept as the
  *oracle* `tests/lcnf/oracle/extract.lean`, outside the language data:
  `conformance_ne_04` requires its record to equal the module's on every kind
  of constant the extraction classifies. Lean orders the join points of a
  function by the numbering of its fresh variables, which depends on what the
  process elaborated before, so the adapter starts every translation from the
  name generator's initial state and its record no longer depends on the
  adapter's own text (`conformance_ne_04` elaborates up to thirteen
  definitions before the extraction and requires the same bytes); the oracle,
  which does not, is compared on functions with at most one join point.
  The registry's rows are a set (a signature may name a constant a later row
  declares), implicit binders that are not types are inferred, signatures carry
  natural literals, and the driver reads roots as text, so no Lean syntax is
  written for a name. Every identity of language 1.2 changes with the
  compiler-semantics ID: the committed locks, goldens, and verified records of
  its projects are regenerated. The canonical compiler inputs of `production`
  and `models` are byte-identical to those the handwritten adapter produced;
  those of `reasoning` and `production-coverage` differ in the order of join
  points in 6 of 166 and 7 of 161 declarations, which Lean chooses by its
  variable numbering, with the same nodes, closures, externals, and
  inductives. The register is unchanged
  in size (350 IDs, 74 codes); `TL-03`, `TL-05`, and `NE-04` are reworded.
- The normative CI gate runs in two legs at once (SPEC.md §9.4, `RP-14`, no new
  ID or code): `examples`, which is about three quarters of the gate's time on
  the runner, and the other eleven recipes of `vv` in their order, each leg a
  clean checkout on the standard runner at width 1. Together they run every
  recipe of `vv` exactly once, which `conformance_rp_14` now checks on
  `ci.yml`; `just vv` itself is unchanged and `release.yml` still runs it
  whole. The gate's wall time becomes the longer leg, about 2 h 40 min where the
  last full run took 3 h 16 min (an estimate from that run's timings). Each leg
  uploads its own `acceptance-gate-evidence-<leg>`.
- Production export contract (SPEC.md §24.7, `PX-01`..`PX-12`, `CL-23`,
  `LLC0201`..`LLC0204`, `LLV7101`, `LLV7102`): a stable, versioned façade,
  `lexlean::api::production`, through which a consumer such as PrismPM
  compiles production roots to certified Rust packages without the hidden
  compiler modules, the build or verification directories, generated Lean, or
  a backend. A closed `lexlean/production-request/1` names roots, targets by
  registered ID and descriptor digest (`production::targets`), the source,
  semantic, and lock identities pinned at setup, and GNAF evidence to be
  re-checked; it has no field for a certification, an optimality claim, Lean,
  a program, or Rust. Every request, root, target, pin, evidence, path, and
  package defect is refused before Lean runs; the project is then verified by
  the unchanged §22 pipeline, and each root's package is rendered from the
  verified lowered program exporting only its recorded entry as `run`, its
  library beginning with the crate certificates B and E are about. The bundle
  is published at `<build_root>/production/<result-id>/` with the attestation,
  compiler input, preservation record, programs, crates, and certificates, and
  a `lexlean/production-result/1` binding every output to the source,
  semantic, lock, compiler-semantics, target, compiler-input, preservation,
  and certificate identities with the closed status `kernel-checked`
  (preservation, never optimality) and optimality `not-claimed` or the GNAF
  verdict the host checker recomputes. `production::verify_artifact` replays a
  bundle without the source, Lean, or the network and reports
  `bindings-verified`, naming the four stages it does not re-run. The CLI
  mirrors it as `lexlean production targets | build --request | verify-artifact`.
  The five envelope schemas live under `api/schemas/`, outside the
  compiler-semantics ID, each recorded under its spec version in
  `api/schemas/versions.toml`. A public-surface consumer
  (`crates/conformance/src/production_consumer.rs`) exports
  `examples/production` and `examples/production-coverage`, and its 59
  packages are committed under `tests/production-export/`
  (`just production-export`, `just production-export-write`). No 1.0, 1.1, or
  1.2 output, semantic ID, attestation, or golden changes.

- Differential validation oracle (SPEC.md §17.19, `OR-01`..`OR-08`, no new
  diagnostic code): the independent Lean 4 to Rust generator `lean4-prod`
  (`https://github.com/auser/lean4-prod`) is pinned in
  `oracle/lean4-prod.toml` by remote, commit `991956fe427bb646498fd0912c89b71be71e7beb`,
  and the SHA-256 of the canonical frame of the 165 files the integration
  uses, with the role `validation-evidence`. Nothing of it is vendored: its
  repository ships no licence text for a copy to carry, so the suite
  acquires it (`LEXLEAN_ORACLE_DIR`, or a fetch of the commit), refuses a
  checkout that does not hash to the pin, and builds it with the pinned
  toolchains. The fetch is atomic (made beside the cache, verified, then
  renamed), a cache that is not the pin is discarded and fetched once more, a
  transient network failure is tried again twice, and the oracle's crates.io
  dependencies are fetched `--locked` into a Cargo home of their own and built
  offline; the pin's toolchains must be the repository's, and every build and
  run is bounded in time. The corpus is every production root of the four certified
  examples, one probe root for each runtime construct they do not realize in
  an accepted root, and the production-refusal fixtures with their marking
  removed: 102 roots, of which the oracle generates Rust for 30 and refuses
    72 (66 of roots LexLean takes, 6 of roots it rejects). The comparison is of
  acceptance, contract, behavior, and resources and never of bytes; on the 23
  compiled crates it agrees with the interpreter and LexLean's own rendering on
  1411 inputs, the seeded ones of the existing differentials and boundary
  inputs derived from each root's types and literals. Every difference is in
  `oracle/known-differences.toml` or is a disagreement, and every construct of
  the production registry is realized in the closure of a compiled root or is a
  row of `oracle/known-non-overlap.toml`; both are re-asserted on every run and
  versioned by the digest of their rows (`oracle/table-versions.toml`). Planted
  arithmetic, control, resource, and ABI discrepancies, each judged on its own,
  are detected, a
  better equivalent rendering is accepted despite its bytes, an oracle that
  fails is unavailable and never validated, and the new gate
  `audit-oracle-isolation` fails on any mention of the oracle in the inputs of
  an identity. Findings against the oracle itself: it renders `Prod.fst` and
  `Prod.snd` as field accesses rustc rejects, and exports a natural literal
  beyond 64 bits that rustc rejects. The register has 338 conformance IDs.
- GNAF certification (SPEC.md §17.18, `GC-01`..`GC-10`, new `LLB6007`): the
  module `Certification` of `compiler/` checks a certificate against a GNAF
  request. A global optimum or argmin is certified by a universal lower bound
  with its attainers, a complete Pareto frontier by a frontier cover, and
  each over a premise for every system of the independent universe; a
  premise is an exclusion, a lower bound from a prefix of the domain, a
  measurement, or a reduction by one of the closed schemas `collapse`,
  `specialize_small`, and `specialize_large`, so a universe with systems that
  exhaust the request's fuel can be certified where enumeration has no answer.
  GNAF-normality is a status of its own: no theorem makes it optimality, the
  normality route never certifies, and a certified optimum is GNAF-normal while
  the converse is false. A representation metric supports an execution claim
  only through a stated bridge. The verdict is `refused`, `certified`, or
  `incomplete` with the first open obligation and the strongest status
  established (`correct_only`, `locally_normalized`, `representation_minimal`,
  or `not_certified`). The hand-written library
  `compiler/proofs/LexLeanCertification/` proves the checker sound over
  `Gnaf.expand` at the request's fuel and every larger one, and proves for
  every improving schema application the module's own closed relation
  `Certification.preserves` (the rewriting keeps the system's observations
  over the request's domain at strictly lower cost, defined in the module by
  `sameObservations` and `strictlyCheaper` and decided by the kernel for each
  committed reduction); its declarations'
  axioms are pinned in `compiler/proofs/library.toml`. Seventy-three certificate
  fixtures under `compiler/certificates/` are reduced to the host's verdict by
  the kernel; the host transcription refuses a certificate beyond 65,536
  premises, 1,024 positions in a list, or numbers of 2^32 with `LLS8002`. The
  register has 325 conformance IDs.
- CI execution on the standard runner (#48, SPEC.md §9.2, §9.4, §22.11,
  §22.12; VERIFICATION.md, "Execution of `just vv` (Issue #48)"): `test` is
  `cargo test --workspace --exclude repo-conformance --all-features` and `bdd`
  is `cargo test -p repo-conformance --all-features`, the sole owner of the
  complete conformance suite, with a new `inventory` recipe
  (`cargo xtask check-test-inventory`) that lists the tests of both and requires
  them to partition `cargo test --workspace --all-features` (RP-13); the
  verifier's process width is an operational profile read from
  `LEXLEAN_VERIFY_WIDTH` (default 1, `LLV7017` when malformed), recorded in no
  artifact, scheduled by an ordered pool whose results never depend on completion
  order, and shown invariant on a multi-module project (VR-20, VR-21); performance
  telemetry (`LEXLEAN_CI_TELEMETRY`) goes to CI evidence only and a measurement in
  a verified artifact fails the gate (VR-22); `ci.yml` stamps each recipe,
  samples the host, and uploads a per-recipe, per-stage, per-module summary
  (RP-14). Every verification no longer hashes the running executable (4.7 s in a
  debug build), and the dev profile keeps line tables only. Register: 330 IDs,
  70 codes (`LLV7017`). The runner's numbers from the first full run (CI run 38201023383: `vv`
  3 h 16 min, `uor-atlas` 8277 s of the 11711 s of recipes) are recorded in
  VERIFICATION.md, "Runner numbers (first full run, PR #72)", as noncanonical
  evidence of cost.
- CI evidence is readable from the job log (#48, SPEC.md §9.4,
  VERIFICATION.md, "Execution of `just vv` (Issue #48)"): `cargo xtask
  ci-summary` prints the summary to the log (also when `just vv` failed),
  appends it to `$GITHUB_STEP_SUMMARY` cut at 512 KiB, and lists at most the
  fifteen slowest rows of each table, besides keeping `summary.md`; the
  supported-host job drops the `cargo build` its `cargo test` repeated (SPEC.md
  §8.3). Pinned by `conformance_rp_14` and the unit tests of
  `repo_conformance::evidence`; no register ID or code is added.
- A host that embeds the library can end the children the library started
  (SPEC.md §17.17, §24.6, `CL-22`): `lexlean::verify::end_live_children` ends
  every child process group the library started in this process and has not
  seen exit, reporting each as `Ended`, `AlreadyGone`, or `Failed` with the OS
  error, and `lexlean::verify::allow_children` re-enables starting children
  afterwards. The shutdown state is set under the same lock that registers a
  child, so a child started concurrently with the call is ended or refused
  (`LLV7018`), never left running; the library installs no handler, changes no
  signal disposition, and never signals the host's own process or group. The
  executable's interrupt thread and a run's own timeout now end groups through
  that one function (`verify::lifeline::end_leader`), and `conformance_cl_22`
  counts the signalling sites across every source file of the library. On
  Windows `taskkill` exits 128 alike for a tree that has exited, a process that
  has exited with its handle still open, and access denied, so a `taskkill`
  that fails is classified by whether `tasklist` still lists the root process
  (`AlreadyGone` when it does not, `Failed` when it does or cannot say). The
  tests run a host process of its own that registers its own handling of
  `SIGINT` and `SIGTERM`, deliver each from outside, and compare its signal
  dispositions before and after the library starts and ends children (where
  the host can report them; the `ps` of macOS cannot, so a second host that
  handles nothing is sent `SIGQUIT` and must die of it with its child still
  running, on every host); the macOS and Windows host legs of the CI now run
  `cargo test --no-fail-fast`, so that a host reports every failing test and
  not the first failing binary's; the
  concurrent-start case ends the children sixty times and catches a planted
  removal of the registration lock in 100 of 100 runs. `CL-22` and `LLV7018`
  are registered (`LLV7017` is the verification width of §22.11), and the
  falsifiability record is in `VERIFICATION.md`. The register has 339
  conformance IDs.
- The acceptance gate fits its job again (SPEC.md §9.2, VERIFICATION.md,
  "Runtime of `just vv`"): the run of this branch was cancelled at the
  360-minute limit. `bdd` is `cargo test -p repo-conformance --test bdd`, the
  meta-gate, because `test` already runs every conformance test (the second
  run was 79 minutes); the conformance cases' environment lock is shared by
  verifications and exclusive only for the code that changes the process
  environment; the preservation environment (the calculus modules and the library) is compiled once per toolchain and library text and shared by the workspaces of a run;
  the four corpora, the stress families, the rejection fixtures, and the
  fixture suite run side by side; and `cargo xtask verify-examples` prints
  the time of each example. The conformance suite takes 50 minutes of the
  reference container where it took 68, and the gate is projected at
  276 minutes (4 h 36 min) on the runner, `uor-atlas` (three hours of it)
  unchanged.
- The certified projects are the four examples `production`,
  `production-coverage`, `models`, and `reasoning`. The five reasoning oracle
  modules of the `compiler` project declare no `production` root, so they are
  never analysed (SPEC.md §17.13) and have no certificate; the `compiler`
  project is verified, not certified.
- Semantic preservation from the source to the realization calculus
  (SPEC.md §17.17, `SP-01`..`SP-06`): every production root is lowered to a
  target program and certified by a kernel-checked Lean theorem (certificate
  A) that the lowered program converges on the encoded arguments to the
  encoded source value, or to overflow exactly where the width predicate
  fails. The hand-written proof library and the calculus modules ship as
  language data under `language/preservation-1.2/`, with every library
  declaration's axioms pinned in `library.toml`. `lexlean verify` checks the
  certificates after named-root extraction, publishes them with
  `preserve/preservation.json` (`schemas/preservation.schema.json`), binds the
  record in the attestation, and fails closed with `LLV7013` (a rejected
  certificate) or `LLV7014` (a drifted preservation environment). The
  compiler project's module prefix is now `LexLeanTarget`.
- The graph templates of the calculus library now count and order every
  node, successors without their own entry included, as the Lean rendering
  does; `graph_topological`'s instance is in first-binding order.
- Fixed a pre-existing defect: `LLV7012` (Lean compiler-front-end authority
  drift) is registered in the environment class with exit code 3, but the
  compiler mapped it to the language class and exited 1. The code now exits
  3, `conformance_cl_11` asserts every registered code's class and exit code
  against `model/errors.toml`, and the `extraction-authority-drift` fixture
  expects exit 3.
- The declared Rust machine (SPEC.md §17.17, `SP-07`): the meaning of a
  rendered crate is stated by the generated, kernel-checked LexLean modules
  `RustSyntax` (the closed Rust AST) and `RustSemantics` (its evaluator over
  calculus values, `?` as a raise out of the function, each runtime item as
  the calculus primitive it realizes, and the machine's abort on an
  unrealizable sequence length), shipped beside the calculus modules. Every
  certified root's crate prints to a Lean term (`production::rust_term`) that
  the machine evaluates in agreement with the calculus interpreter.
- Certificate B (SPEC.md §17.17, `SP-08`): each certified root's crate in
  each of its targets is related to its lowered program by a kernel-checked
  simulation theorem, `LexLeanPreserve.C<hex>.R<i>.RustCore` or `.RustStd`.
  The library proves once that every derivation of its closed
  correspondence `Corr` is sound for the machine (`sound`, `simulate`); the
  aligner (`production::rust_cert`) writes each function's derivation, whose
  side conditions the kernel decides. `audit-production` holds the aligner's
  rules, `Corr`'s constructors, and the soundness theorem's cases to one
  another and requires the aligner to name every calculus construct with no
  default arm. `lexlean verify` derives, checks, replays, and audits
  certificate B beside certificate A, records it under `renderings` in
  `preservation.json`, and refuses a rendering it cannot certify with
  `LLV7015` (negative fixture `certificate-b-rejected`).
- Machine items are indexed by width and sequence kind (SPEC.md §17.17,
  `SP-08`): `fixed_u8::checked_add` on two `u16` values, or `length_bytes` on
  text, is stuck as in Rust rather than the calculus primitive's value, and
  value typing carries a fixed-width value's width and a sequence's kind, so
  certificate B refuses a rendering that changes either. This closed a gap the
  first width mutation exposed.
- Certificate E (SPEC.md §17.17, `SP-09`): certificates A and B compose into
  `LexLeanPreserve.C<hex>.R<i>.Compose.RustCore` or `.RustStd`, whose theorem
  states that the rendered root, invoked on the encoded arguments, realizes
  the encoded source result. The proof is the library's `compose` over a
  generated proof that every encoded argument is well typed. `lexlean verify`
  checks and records it beside A and B and refuses a rejected composition with
  the new `LLV7016` (negative fixture `certificate-e-rejected`).
- Boundary validators (SPEC.md §17.12, §17.17, `SP-10`): a root whose
  parameters can hold a map, set, or graph, including inside options,
  results, products, lists, document types, and recursive groups of them, is
  lowered with generated validators and an entry that returns `none` for
  arguments that break the strictly-ascending invariant §17.12 states and
  the root's result as `some` otherwise. Certificate A proves each validator
  decides exactly the library's proposition of its type (the recursive
  group's by the same structural recursion as its encoders) and the entry's
  two outcomes; certificate E states the rendered entry's. A validator is
  called only by the entry and by validators, which a boundary audit in the
  lowering enforces, so no proof-only invariant becomes a runtime check.
  `examples/production-coverage` gains a recursive tree root (`Boundary`) with
  such a boundary.
- The conformance suite plants defects (SPEC.md §17.17) of nine kinds in
  lowered programs, each refused by Lean at the relation of the function it
  changed, and of ten kinds in rendered crates, including an entry that omits
  its first validator; and it builds every certified root as a Rust
  package under the pinned toolchain and compares its printed outcomes,
  through the root and through the entry, with the interpreter's
  (`SP-11`), as build evidence that rustc agrees with the declared machine.
- Review of the semantic-preservation work (SPEC.md §17.17): the certificate
  generator now judges which parameters the entry validates, and which
  components each validator checks, from the source types by its own
  recursion, and refuses a lowering that differs (a skipped parameter, a
  validator that checks nothing); the validation mutation is planted at every
  validated parameter, every component check, and every order comparison, and
  the differential breaks each validated parameter's invariant in turn. Every
  mutation, in the lowered program, in the crate, and in certificate E, is
  planted at its first, a middle, and its last place and must fail in the
  declaration of the function it changed. Certificate E is stated for the
  arguments a Rust caller can pass (`RepresentableL`); the machine's abort
  is proved to arise only in an item that cannot fail
  (`runItem_abort_infallible`) and the suite checks that no infallible item
  other than a length overflows. The differential reaches the overflow arm
  (top and bottom of every scalar type, larger and longer inputs, a search),
  and the rustc differential runs a root with an entry through the entry
  alone. `preservation.json` binds the lowered program and each rendered
  crate, which are published beside the certificates. The statement
  vocabulary SPEC.md quotes is held byte-equal to the library's by
  `validate-spec-links`, and every library declaration must be registered.
- Limits (SPEC.md §17.17 *Limits*). A root whose lowered program exceeds
  `max_ir_nodes`, or for which a lower bound on one of its certificates
  exceeds `max_file_bytes`, or whose field reads print so many record entries
  that certificate B would exceed it, is refused with `LLS8002` before the
  toolchain is touched. The bound is per certificate (A from types,
  expressions, and the pairs of arms of its matches; B from expressions,
  shapes, and literals; E from shapes and pairs), the greatest of the three is
  compared with the limit and not their sum, and each cost is half of the
  greatest that stays below its certificate over the corpora, the stress
  families, and programs that grow several dimensions at once; `conformance_sp_02`
  checks it against the corpora, the families, random combinations none of
  which it was fitted to, and the largest scaling of nine mixed shapes that
  fits the default limit (not refused). Certificate B, whose derivations grow
  with the square of a record's arity and cannot be bounded from node counts,
  is generated under `max_file_bytes` and stops at it. Certificates are
  generated right after lowering for that reason. Negative fixtures
  `lowering-size-limit`, `certificate-size-limit` (a record of 800 fields
  copied field by field), `certificate-generation-limit` (a sum of the 500
  fields of a record), and `certificates-total-limit` cover them. The
  programs, crates, and certificates of all the roots of a project count
  toward `max_total_source_bytes` as they are generated, and so does the
  eligibility report (§17.13), whose closure members each record a shortest
  call path: a chain of 3000 calls took 638 MB, a chain of 1000 roots 9.5 GB
  and 90 seconds, and 2000 exhausted a 16 GB host in `lexlean check`. A path is
  now shared, and a chain of 2359 functions or 243 roots is the most a project
  may have at the default limit (`LLS8002` from `check`, fixture
  `eligibility-report-limit`). A pinned Lean that is killed, runs out of memory, or
  overflows its stack while checking a certificate is `LLS8002`; one that
  exhausts its heartbeat or recursion budget is `LLS8002` only when the module
  is at least a quarter of `max_file_bytes` and is otherwise Lean's verdict
  on the certificate, `LLV7013`, `LLV7015`, or `LLV7016` (fixtures
  `certificate-heartbeat-rejected` and `certificate-resource-exhausted`): a
  wrong certificate whose proof makes `isDefEq` loop is not a limit.
  Rejections of certificates A, B, and E name the declaration, the root, and
  the bounded first error.
- Every certificate is generated under `max_file_bytes` (the proof of each
  term, each match arm, the encoders, each function, the boundary), so no
  program makes the generator build more than the limit allows; the early
  refusal is a lower bound, not the guarantee. A
  match of thousands of arms cannot overflow the stack (the aligner derives
  arms in a loop and a derivation is written and dropped iteratively).
- A match on many constructors verifies through `lexlean verify`. The
  extraction record of a named root nests two levels of JSON for each arm, and
  its reader refused it from about 60 arms (`LLV7011`, `recursion limit
  exceeded`); it is now read without the limit, on a thread with a large
  stack, as is the compiler input. The pinned Lean needs more than its default
  heartbeats for a certificate B of a match on 160 or more arms (a type
  mismatch that blamed a valid certificate), so a program whose widest match
  has more than 100 arms sets both budgets to 0 and the wall clock,
  `child_timeout_ms`, is the limit. A match on 180 constructors is verified by
  `conformance_sp_02`, and one on 300 by hand; SPEC.md says where the machine
  reaches the timeout. The earlier statement that a match on 440 constructors
  verifies was false: its certificates are generated, and Lean does not check
  certificate B within 15 minutes.
- A timeout ends the work. `child_timeout_ms` killed `lake` and left the
  `lean` it had started running at full speed with a growing resident set,
  which with the budgets lifted for a wide match nothing ended; every child of
  `verify` now leads a process group of its own, killed and awaited as a whole
  on a timeout (a test starts a grandchild and asserts none is left). The
  group is signalled directly (`rustix`, no `kill` executable on `PATH`, which
  the shipped image does not have), a group that cannot be ended is said so in
  the diagnostic, and an interrupt (`SIGINT`, `SIGTERM`, `SIGHUP`, `SIGQUIT`, watched with
  `signal-hook`) ends the live groups before it ends the `lexlean` executable,
  because a child in a group of its own no longer receives the terminal's
  Ctrl-C; a test sends each to the real binary. A signal the process was
  started ignoring (`nohup`'s `SIGHUP`) stays ignored, the library installs no
  handler (a host that embeds it keeps its own signals), a zombie in a killed
  group counts as ended (the shipped image runs `lexlean` as PID 1 and reaps
  what it can), a `taskkill` that fails is reported, and `verify` removes the
  staging directories that interrupted runs left. `SIGKILL` cannot be caught,
  and leaves the children. SPEC.md §8.5 names the role of every shipped
  dependency, including `rustix` and `signal-hook`, and `audit-shipped`
  requires it.
- The lower bound on certificate A counts a type as the certificate prints
  it: a pair nested 120 deep was `120 * 121 / 2` type nodes and a bound 1.5
  times the certificate, and the programs that are refused when they fit are
  now sought among nested pairs, options, lists, results, and records, one
  function and twenty, and random combinations of seven dimensions.
- Names of a project that Lean resolves to something else, refused at `check`:
  a module, or any segment of a dotted module name, named like a namespace the
  generated code writes qualified (`Nat` with a function `blt`; `Sub.Nat` in
  `Sub.Other`), a pattern binder named `none`, `some`, `isTrue`, or `isFalse`
  (a bare name in a pattern is the constructor), a constructor or field named like a member Lean
  declares for the type (`rec`, `casesOn`, `noConfusion`, `ctorIdx`, ... read
  off the pinned Lean), a `module_prefix` that begins with a root this compiler
  ships (`LexLeanPreserve`, `LexLeanPreservation`, `LexLeanRuntime`, ...), and a
  module with a production root under the prefix `LexLeanTarget`; a parameter
  named like a nullary constructor of its own type (`zero`, `red`, and `none`
  or `some` of a parameter) is valid, and the module silences the linter that
  reported it.
- The token audit reads a number as the pinned Lean's `numberFnAux` does
  (`1e10axiom`, `1_0axiom`, `0b1_0axiom`, `1.e5axiom` are a number and a
  keyword; `1_`, `1.foo`, and a field index followed by `_` or an exponent are
  refused where Lean refuses them or reads them by the parse), checked against
  the pinned Lean on a generated set of prefixes by `conformance_sp_09`, and
  refuses every `#` command, since Lean reads the longest command token it
  knows and the rest as a name (`#evalIO`).
- The eligibility reports of all the modules of a project count together
  toward `max_total_source_bytes` (it was per module), and `check` and `build`
  measure them with one function (the length of the written file), so they
  pass and refuse the same limits (they differed by about 128 bytes a module).
- A root verifies whatever its names are, with no name a user can write
  capturing a name a generated file uses. Every reference a certificate makes to
  a global is written from the root (`_root_.LexLeanPreservation.conv_var`),
  so that a parameter, however spelled, cannot turn the first segment of a
  global into a field access of itself (a parameter named `LexLeanPreservation`,
  `LexLeanTarget`, or `LexLeanPreserve` broke certificates A and E); the names
  certificate E binds begin with two underscores, which no semantic name does;
  the certificate token audit follows Lean's tokenizer (character literals,
  raw strings, dotted and unicode names, `sorry.1`, attributes) and reads a
  quoted name as the name it is, forbidden constants and attributes included
  (the generator quotes a name spelled like a forbidden token, the backend and
  the audit module one spelled like a word the generated-Lean audit forbids,
  such as `native_decide` and `IO`, so a function, structure, inductive,
  field, constructor, or parameter of that name verifies), and fails closed
  on anything it cannot classify; `conformance_sp_09` verifies a project whose
  parameters are spelled like every namespace the certificates of the corpora
  begin a name with, and its gate reads the first component of every longer
  name.
- Language-1.2 declaration names (§17.12 rule 10). No declaration may be spelled
  like a name the backend writes unqualified (`Int`, `String`, `Unit`,
  `Except`, `ByteArray`, `Ordering`, `UInt8`..`UInt64`, `Int8`..`Int64`,
  `true`, `false`, ...), and no binder may be named `true` or `false`: a
  declaration `Int` was the `Int` of every later signature of its module and a
  parameter `true` the `true` of every Boolean literal, and Lean reported
  `LLV7002` for a program `check` had accepted. Linking now refuses them
  (`LLT4001`, fixtures `declaration-lean-name` and `binder-bool-literal-name`).
  A `module_prefix` whose first segment is a module root of the pinned
  toolchain (`Init`, `Std`, `Lean`, `Lake`, `LakeMain`, `LeanChecker`,
  `LeanIR`) or `IO` is refused by configuration (`LLC0101`, fixture
  `module-prefix-reserved`) where it was `LLV7003` or `LLI9001` in the middle
  of verification.
- Certificate E is stated, in full, for representable arguments, and Lean
  applies it to arguments of the differential, some at the bounds of `u64`
  and `i64`, so a hypothesis that cannot be met or used fails; the planted
  `RepresentableL .. ∧ False` is refused. `preservation.json` records the
  function each crate is invoked through, per root and target, which E states
  and the crate defines; SPEC.md says what is and is not a package and which
  certified roots rest on the kernel proof alone for the overflow arm.
- Fixed a pre-existing defect: named-root extraction (SPEC.md §22.10)
  refused every root that reached an `Init` function whose module does not
  expose its body (`String.toInt?`, `String.toUTF8`, `String.splitOn`,
  `String.intercalate`, `List.takeTR`), because the module system exports
  such a definition as an axiom, and every root whose LCNF types carry the
  `borrowed` annotation (an instance over `Nat.div` or `Int.div`), because
  the adapter recorded all metadata as unsupported. The adapter now records
  each constant's declared kind (`Lean.getOriginalConstKind?`), whether
  Lean's compiler holds an external's code (`Lean.IR.findEnvDecl`), and the
  `borrowed` annotation (`Lean.annotation?`), each registered in
  `language/lcnf-1.2/authority.toml`; the host admits an exported axiom
  exactly when it was declared a computable definition with compiled code,
  as Lean's own code generator does, and reads a borrowed type as the type
  it annotates. Declared axioms, opaques, unsafe, partial, and noncomputable
  dependencies, other metadata, and a kind that differs from the declared
  kind still fail with `LLV7011` (`NE-03`). The coverage corpus that these
  defects had kept out of verification is now the verified example
  `examples/production-coverage`, whose certificates `lexlean verify` checks.
- Language 1.2 (SPEC.md §17.12): a strict extension of language 1.1 selected
  by `language = "1.2"`, with builtin packages at `1.2.0`, the lock schema
  `lexlean/lock/2`, the semantic-module schema `lexlean/semantic-module/2`,
  and the snapshot envelope `lexlean/semantic-snapshot/2`. Routing is fixed
  by the project language: a `/1` module under 1.2, a `/2` module under 1.1,
  a mismatched lock schema, and a cross-version package or glossary are
  rejected before either backend runs.
- The first 1.2-only construct is the typed, nonrecursive `let` term. It
  lowers to a Lean `let`, and language 1.1 rejects it in linking (`SM-23`).
- The compiler-semantics IDs of languages 1.0 and 1.1 are unchanged: each
  language digests a fixed nested partition of the embedded tree, and every
  committed 1.0 and 1.1 artifact keeps its bytes. Migration from 1.1 to 1.2 is
  explicit and never implicit.
- Language-1.2 recursive data (§17.12): inductives may recurse uniformly and
  strictly positively, directly or nested under `List`, `Option`, `Result`,
  and products; `mutual` labels form contiguous mutual groups; every member
  must have a buildable base case; structures stay nonrecursive. Structural
  recursion uses exactly the direct recursive fields, and induction binds one
  hypothesis per recursive field. Products, pairs, `first`, `second`, and
  `Prod.mk` matches are new (`DF-12`, `DF-13`, `SM-24`, `examples/recursive-data`).
  A standalone recursive definition decreases only over a self-recursive
  inductive, and no 1.2 declaration may take the name of a built-in
  constructor owner (`Bool`, `List`, `Nat`, `Option`, `Prod`, `Result`).
- Language-1.2 binder hygiene (§17.12 rule 10). No binder may be spelled like:
  - a declaration of its module;
  - a built-in name the backend emits;
  - the root of the module prefix.

  The rule covers type parameters, value parameters, and pattern, `let`,
  quantifier, and proof binders. Such a binder would capture the
  unqualified reference in generated Lean after linking had accepted it;
  linking now refuses it. Also new in language 1.2:
  - the canonical LaTeX states the type parameters of parameterized data;
  - every declaration is its own source-map node in both artifacts;
  - type parameters, constructors, and mutual labels are charged to
    `max_ir_nodes`.

  Link-time type diagnostics now spell types as the document does, not as
  internal structures.
  Eight new negative fixtures cover positivity, non-uniformity, an
  uninhabited cycle, a recursive structure, a non-contiguous group, bad type
  arguments, a constructor mismatch, and a forward reference.
- Language-1.2 higher-order code (§17.12): function types, lambdas with exact
  explicit captures, full applications, definition references, and generic
  definitions and theorems with explicit type arguments and no polymorphic
  recursion. `executable` definitions are admitted only with
  non-escaping closures and executable callees: no type they state or write
  may hold a function, directly or through a document type's fields, and a
  function parameter is used only where a closure may be. Type parameters
  never capture a backend type, a document type, or a value binder, and the
  universe is never a type argument. Snapshots record a per-definition alpha
  identity (`DF-14`, `DF-15`, `SM-25`, `SM-26`, `examples/higher-order`),
  exposed as `SnapshotSemanticDeclaration::alpha_identity` and
  `SemanticSnapshot::alpha_ids`. Fifteen new negative fixtures cover
  capture, arity, typing, escaping closures (returned, stored in an
  `Option`, and passed in or out of a structure), polymorphic ambiguity and
  recursion, recursion under a lambda, formal callees, a captured type name,
  and a universe type argument.
  Changes after review:
  - Binder hygiene (§17.12 rule 10) covers generic type parameters and
    lambda parameters, so neither the declaration's own name, a sibling
    declaration, nor the module prefix can be captured.
  - A type parameter its declaration never mentions lowers as `(_T : Type)`.
  - The alpha identity numbers binders in evaluation order rather than in
    serialized member order.
  - The 1.2 LaTeX states every parameter with its type and every closure
    with what it binds and captures.
- Language-1.2 recursion (§17.12): mutual definition groups recurse
  structurally over one recursive family (naturals, lists, or an inductive
  group with its containers) and lower with `termination_by structural`;
  well-founded definitions carry a measure and one statement-exact evidence
  theorem per call site, checked in linking and proved under Lean. A
  well-founded call may sit under `if` and `match`: its obligation is
  quantified over the enclosing match binders and hypotheses, and lowers
  through `match (generalizing := false) __decreaseN : s` with the evidence
  applied to `_` for each enclosing binder and substituted by `subst_vars`
  (`examples/recursion` reassociates and prunes a syntax tree by a weight
  measure, the pruning under binders its branches ignore). The new `linear_arithmetic` proof form
  discharges linear obligations and may unfold named prior definitions.
  Recursion that escapes its check (a group member called inside a lambda or
  referenced as a value, a mutual label shared by an inductive and a
  definition group) is rejected; eleven new negative fixtures cover these
  and the decrease and evidence rules (`DF-16`, `DF-17`, `PF-19`, `SM-27`).
  Changes after review:
  - Mutual groups may be well-founded: every member has a measure, and each
    call's obligation compares the callee's measure at the arguments with
    the caller's. `ping` and `pong` in `examples/recursion` call each other
    on the same argument and terminate by their two measures.
  - Every mutual group's call graph must be strongly connected, with every
    member calling into the group.
  - The example adds a structural mutual constant-folding rewriter over
    `Expr` and `Stmt` that returns the rewritten tree.
  - A false evidence theorem stating the exact obligation is refused by
    verification (`recursion-false-evidence`), and SPEC states what linking
    and verification each establish.
  - The negative fixture suite requires every diagnostic of a class to
    carry its prescribed code, and the absolute-path guard no longer
    mistakes an escaped newline after a colon for a Windows drive.
- Language-1.2 ordered collections (§17.12): `map` and `set` types over
  closed ordered key types, canonical map/set/graph literals (reordered
  source links to identical semantic data), insertion/lookup/set-algebra
  operations, ordered folds, bounded iteration, and graph successors,
  reachability, and topological order, all over the fixed
  `LexLeanCollections` runtime, which every module applying a collection
  primitive emits (`SM-28`, `SM-29`, `SM-30`, `DF-18`). Literal keys are
  checked as terms before they are ordered. `examples/collections` proves
  each of the 24 operations under Lean and, for every key type (negative and
  fixed-width integers, Booleans, non-ASCII strings, pairs), that linking's
  canonical order equals Lean's own insertion order. Ten new negative
  fixtures cover duplicate, unordered, non-literal, noncanonical, and
  out-of-range keys, graph references and duplicate edges, fold typing,
  unbounded iteration, and collections under language 1.1.
  Changes after review:
  - A graph's nodes are its keys and every successor, so a successor
    inserted without an entry of its own is a node with no successors;
    reachability and topological order now agree on it.
  - `SM-28` and `SM-30` check every operation, under Lean, against an
    independent `BTreeMap`/`BTreeSet` model on seeded operation sequences,
    including a chain as deep as its node count allows.
  - `SM-29` compares the snapshot and every published artifact that does
    not record source positions.
  - Operation costs are stated, `iterate_until`'s bound is named as fuel,
    and an oversized collection literal is `LLS8002`
    (`collection-literal-limit`).
  - Literal normalization walks the typed module instead of a JSON round
    trip, and the canonical document names map and set types.
- Language-1.2 production eligibility (§17.13): executable status is a
  declared, checked property. A definition becomes a production root only by
  declaring `production` with registered targets (`rust-core`, `rust-std`)
  and admitted effects (`allocation`, `overflow`, `recursion`). Its runtime
  closure across modules, monomorphized at type arguments and with
  termination evidence erased, is classified construct by construct against
  the closed registry `language/production-1.2.toml`; a boundary that is not
  first-order data, a formal-only or unresolved dependency, an out-of-width
  literal, unavailable allocation, or an unadmitted effect fails with the new
  `LLT4005` before any backend runs. Each module's report is published as
  `production/<module>.eligibility.json`, and audit-production rejects any
  default branch in the analysis (`PD-01`..`PD-07`, `examples/production`).
  Changes after review:
  - A root that declares type parameters is refused even when its signature
    never mentions them, and a type parameter that survives instantiation in
    the closure is refused (`production-phantom-type-parameter`).
  - audit-production also refuses binding catch-alls, tuple defaults, and
    equality tests on the IR, counts a variant only where a match arm names
    it, and reads character literals; clippy's wildcard lints back it up.
  - Each report records, per target, every natural-number and integer
    representation crossing the root boundary with its width.
  - `executable` is renamed in the document and §17.12 as the closure rule
    it is, distinct from production eligibility.
  - New fixtures for a universe and a proposition inside a named type at the
    boundary; PD-05 states only what linked source can reach, and the
    analysis's other refusals have unit tests on unlinked IR.
  - PD-01 pins every column of every registry row, and a registry failure is
    an internal error (`LLI9001`), not `LLT4005`.
- Named-root extraction (§22.10): verifying a project with production roots
  now runs a new stage 12. Lean's compiler front end translates every
  definition each root reaches to base-phase LCNF through the pinned adapter
  `language/lcnf-1.2/extract.lean`, which runs no LCNF pass after the
  translation. Every Lean operation the adapter uses is pinned in
  `language/lcnf-1.2/authority.toml` by exact signature and source SHA-256,
  and is probed on each run; drift fails with the new `LLV7012`.
  Extraction fails closed with the new `LLV7011` on:
  - an unknown root, an opaque, axiomatic, unsafe, partial, or noncomputable
    dependency, or an unresolved or unsupported constant or form;
  - a dropped dependency;
  - a proof presented as a runtime member;
  - any disagreement with the production-eligibility closure.

  Each root's Lean closure must equal the eligibility closure computed from
  the semantic IR, so two independent implementations check each other.
  Verification publishes the canonical, root-independent
  `production/compiler-input.json` (`lexlean/compiler-input/1`, schema
  `compiler-input`) and records it in the attestation (`NE-01`..`NE-06`).
  Changes after review:
  - The adapter reports facts only. For everything reachable from the roots
    it reports each constant's kind, module, computability, and kernel uses,
    and the base-phase LCNF of every code-generating project definition,
    runtime members included. The host decides each closure, its runtime
    members, its erased proofs, recursion (cycles of the use graph), and
    admissibility, and carries the eligibility analysis's monomorphization
    plan into the compiler input (`lexlean/compiler-input/2`).
  - The authority registry (`lexlean/lcnf-authority/2`) is closed. Every
    constant the adapter uses is one call, type, or plumbing row. Every
    extraction checks the adapter's constants against it, compares each
    signature structurally (default values and binder kinds included), and
    compares each type's constructors with Lean's.
  - A warning beside the record is drift. Universe levels are recorded.
    Variable scope is lexical.
  - Language-1.2 attestations are `lexlean/attestation/2`
    (`attestation-v2` schema), which records the compiler input. The
    verified layout lists the eligibility reports, and VR-13 checks it on a
    project with production roots.
  - New fixtures `extraction-rejected` (`LLV7011`) and
    `extraction-authority-drift` (`LLV7012`) run end to end. The adapter's
    classification is checked on real Lean over a hand-written module, and
    the authority statement is a `some-true` ledger claim.
- The production realization calculus (§17.14), the closed target of
  production compilation. It is defined as LexLean in the new `compiler/`
  project: syntax, a kernel-checked denotation with exact step accounting
  and explicit overflow, and generated fixture theorems. The host side,
  `lexlean::calculus`, provides:
  - the closed JSON form `lexlean/target-program/1`, with canonical bytes
    and content identity under alpha-renaming;
  - the static rules, failing closed with the new `LLB6005`;
  - a reference interpreter;
  - a realization library that transcribes every collection primitive;
  - a realization table covering every runtime construct of the production
    registry;
  - the reference Rust rendering under `#![forbid(unsafe_code)]`.

  The hand-constructed fixtures are confirmed three ways: Lean's kernel (or,
  for the few primitives the kernel cannot reduce, Lean's evaluator),
  LexLean's own collection primitives, and `rustc`
  (`TC-01`..`TC-07`, `cargo xtask check-calculus`). The calculus modules
  and the project configuration are generated from their definition in
  `repo-conformance`, and `cargo xtask check-calculus` compares them byte
  for byte.

  Changes after review:
  - The step count charges every operation the evaluator performs: the
    bindings a lookup examines, each arm a match tries, the captures an
    application passes, the fields a projection skips, and a primitive's
    operand and result weights (one per node, plus characters and bytes).
    The kernel confirms every kernel-reducible fixture's count.
  - Two Rust profiles. `rust-core` is `#![no_std]`, allocates nothing, and
    renders exactly the programs that need no heap. `rust-std` shares strings,
    byte strings, and persistent list cells behind `Rc`, so `cons` and its
    match are constant work and a use clones a handle. Both are library
    crates, committed as packages under `compiler/rust/` with the lint gate
    of §17.16. A work
    counter in the runtime never exceeds the denotation's steps on any
    fixture, and a quadratic append is detected.
  - Every runtime row's `allocation` in `language/production-1.2.toml` now
    equals its realization's. Eleven rows that only read, measure, or fold
    existing storage (`term.nil`, the map and set lookups, sizes, folds, and
    representations, and `graph_successors`) no longer require allocation;
    their operand types still do.
  - Every fixed-width primitive is exercised at every width it admits
    (sixteen new fixtures). Byte equality is decided by the kernel; the new
    `byte-compare` fixture shows `compare_bytes` is not.
  - `rustc` 1.97.1 is a cited authority (`RUSTC-1-97-1`), and `TC-07`
    refuses any other.
  - `TC-04`'s plant is a real one-step mischarge. `TC-03` samples the
    progress property on random well-typed arguments, which found that the
    interpreter's checked fixed-width multiplication overflowed `i128` and
    panicked on two large 64-bit operands; it is now checked.
  - An ill-typed program has no canonical form and no identity.
- The canonical Rust backend (§17.16). A target program is lowered to a
  closed Rust AST (`lexlean::calculus::rust::ast`) and checked before it is
  printed into canonical bytes:
  - identifiers are generated from closed kinds, and each is bound once per
    function;
  - every value is moved at most once;
  - `rust-core` names no heap type, runtime function, or construct;
  - failure is typed exactly: a function that can overflow returns `R<T>`
    and every call to it propagates, and any other returns its value;
  - every construct carries the calculus element it was lowered from, and
    is refused unless its correspondence row names that element, at that
    element's width, and the program uses it.

  Strings are escaped by LexLean itself, independently of the toolchain's
  Unicode tables. A type no value inhabits is rendered as an empty match
  where it is a parameter and omitted where it is an arm, and any other
  computation of one is refused. Units, literal and equal branches, matches
  that rebuild their scrutinee, and the other shapes Rust's lint gate
  constrains are rendered as it admits (§17.16, Lowering).

  Packages (`lexlean::calculus::package`, manifests `lexlean/rust-package/1`,
  provenance `lexlean/rust-provenance/1`) add exported functions with
  checked names, passing modes (`own`, `borrow`, `copy`), and declared
  failure modes; a boundary holding a function value at any depth, or a
  version part that is not a canonical `u64`, is refused. Each package also
  carries a Cargo manifest whose lint table is the package's gate (ten
  documented exceptions), and provenance binding every file's SHA-256, the
  program identity, the runtime, the sources, and the language-1.2
  compiler-semantics ID. `language/semantics-1.2.toml` gains `rust_backend`
  and the SHA-256 of each profile's runtime. Every fixture whose entry takes
  and returns first-order data is committed as a package in each profile
  that admits it, bound by its `sources` to the semantic ID of the verified
  `compiler` build that states its program, along with negative manifests
  for identifier collisions, ownership mismatch, unsupported boundary and
  uninhabited types, hidden allocation, arithmetic mismatch, and
  noncanonical versions and sources (`RB-01`..`RB-07`). The harnesses that
  run packages live in the conformance crate. `language/semantics-1.2.toml`
  also records the digest of the renderer's sources (`rust_renderer`), so
  no rendering can change without changing LexLean's identity.
  - A harness runs its calls on a thread with a fixed 64 MiB stack, one Rust
    function per called export: the primitive differential's single `main`
    overflowed the 1 MiB main-thread stack of the Windows runner.
- GNAF requests over the calculus (§17.15), after UOR-GNAF
  `uor-gnaf/1-draft.2`, cited by revision and SHA-256 as the authority
  `UOR-GNAF-1-DRAFT-2`. The module `Gnaf` of `compiler/` fixes, before any
  optimizer runs:
  - the problem, a reference program and a non-empty domain;
  - the machine contract, with the closed action kinds (observation,
    preprocessing, advice, retained state, dispatch, fallback, communication,
    randomness, scheduling, execution) and their charges;
  - the comparison boundary;
  - the universe, a grammar of complete systems (fixed plans and dispatching
    selectors over shared plans) with definitional completeness evidence;
  - the scalar or componentwise (steps, size) order;
  - the claim class and its scope.

  Optimizer-defined, discovered, cached, and internal-plan universes are
  rejected, as are hidden costs, unrealizable actions, and claims over the
  wrong order or beyond the grammar. An unknown cost leaves the answer
  incomplete. Every committed request's answer is a kernel-checked theorem,
  and the authority's GNAF-VEC-01/02/04/17 and GNAF-REJ-14/29 vectors are
  theorems of the model. The host side `lexlean::gnaf` loads requests
  (`lexlean/gnaf-request/1`, new `LLB6006`), bounds fuel and universe size
  (`LLS8002`), and transcribes the answer (`GN-01`..`GN-08`). The `Gnaf`
  module is generated from its definition in `repo-conformance`, and
  `cargo xtask check-calculus` compares it byte for byte.

  Changes after review:
  - One generic order, `Gnaf.argmin` and `Gnaf.frontier` over rows of an
    identity and a cost, computes every answer and every authority vector.
    Equal costs dominate neither way, and new fixtures with a duplicated
    plan (`argmin-tie`, `frontier-tie`) keep every equal-cost system;
    dropping one is refused by the kernel (GNAF-REJ-21).
  - `Gnaf.expandComplete` is a kernel-checked theorem that, for every grammar
    and selector, membership in the expansion is exactly well-formedness, and
    `GnafFixtures` pins every request's universe and every answered request's
    system statuses as well as its answer. A grammar now dispatches between
    every ordered pair of plans, a plan and itself included.
  - A system's size counts only the functions reachable from its entry, so
    shared plans it cannot run no longer flatten the frontier, and
    GNAF-VEC-02 is posed as a request (`vec02-pareto-envelope`) whose
    frontier is three of its four systems; `Gnaf.certifies` refuses that
    frontier with a member omitted (GNAF-REJ-29) and `Gnaf.minimaAttained`
    shows no system attains its componentwise minima (GNAF-REJ-14).
  - Preparation is declared per plan and charged to exactly the systems that
    can run the plan, which can change the optimum; a `free` preparation is
    admitted only through a prepared artifact bound in the machine by action
    and SHA-256 (`unbound_preparation`, `stray_prepared_artifact`).
  - The machine binds its capacity and its operand-size treatment; a request
    beyond its capacity or charging operands by unit is refused. The host
    refuses a request beyond its own capacity with `LLS8002` instead of
    reporting a cost the model would not, and an evaluation panic is
    `LLI9001`, not a platform failure.
  - Every request states its `SystemUniverseId`, the framed SHA-256 of its
    problem, machine, and carrier, which `load` recomputes; `load` also
    refuses a domain argument on which the reference returns no value,
    repeated thresholds, and misdeclared plan preparation (`LLB6006`).
  - `restricted_universe_optimal` is refused as the §12.4 alias it is, and
    `profile_defined_comparison` carries its profile, class, and shape.
  - The authority vectors are stated over the order the answers use, with
    path identities and certificate invalidation for GNAF-VEC-01, the
    randomized expectation for GNAF-VEC-17, and the rejections of
    GNAF-VEC-04, GNAF-REJ-14, and GNAF-REJ-29; `GN-07` decodes each
    statement against the authority's numbers.
  - The authority is vendored at `model/authorities/UOR-GNAF-v1-draft.2.md`,
    and `validate-model` recomputes the SHA-256 of any vendored authority.
  - `compiler/gnaf.manifest.json` is the UOR-GNAF §20 dependency manifest
    (`GN-08`), and the GNAF schemas are generated with their calculus
    definitions taken from `schemas/target-program.schema.json`.
- The language-1.2 portable runtime exposes every definition, so a
  definition imported from another module reduces in the kernel through the
  primitives it applies. The 1.2 `lean_backend` version is bumped, and the
  frozen language-1.1 runtime is unchanged.
- Every semantic-module member must survive into the typed value: an extra
  member of a unit variant (for example `{"kind":"reflexivity","tactic":...}`)
  was silently ignored and is now rejected in every language.
- Language 1.1 definitions now reject a type parameter written inside their
  body: the scope is empty, so such a definition could never elaborate.
- A semantic-module definition parameter, quantifier, `let`, or lambda binder
  that its scope never mentions now lowers as `_name`. Such a module type
  checked and built but failed verification with `LLV7006` on pinned Lean's
  unused-variable warning; only Lean text that never verified changes
  (`SM-08`).
- The public snapshot DTO grows with language 1.2. `SnapshotTerm` gains
  `Let`, `Pair`, `First`, `Second`, `Lambda`, `Apply`, `FunctionRef`,
  `MapLiteral`, `SetLiteral`, and `GraphLiteral` (with `SnapshotMapEntry` and
  `SnapshotEdge`) and call type arguments; `SnapshotType` gains `Product`,
  `Function`, `Map`, and `Set`; `SnapshotPrimitive` gains the collection
  operations; inductives gain `mutual`; definitions gain `type_parameters`,
  `executable`, `mutual`, `termination` (`SnapshotTermination`), and
  `production` (`SnapshotProduction`);
  theorems gain `type_parameters`; `SnapshotProof` gains `Apply` type
  arguments and `LinearArithmetic`. Downstream exhaustive matches must add
  them.
- New conformance IDs `CF-17`, `CF-18`, `GL-17`, `GL-18`, and `SM-23`; new
  example `examples/language-1.2`, verified with real Lean; ten new negative
  fixtures for version mismatches in both directions, a package of another
  language, and malformed versions.
- Schemas for the later languages, in the 1.2-only partition so the frozen
  1.0 and 1.1 identities are untouched: `project-v2`, `lexicon-v2`, and
  `build-manifest-v2` admit languages 1.1 and 1.2, and `lock-1.1` describes
  the language-1.1 lock. The v1 project, lexicon, manifest, and lock schemas
  had pinned `language` to `1.0`, so every committed 1.1 document violated
  them; every example's documents are now validated against its language's
  schemas.
- `SnapshotSemanticModule::parse` takes the project language, which routes
  the `semanticdata` discriminator; a downstream caller must pass it.
- Language-1.2 models (§17.12, issue #29): `artifact`, `contract`,
  `realization`, `evidence`, and `model` declarations, the `checked_apply`
  term, the `contract_violation` type, and the `less_than` primitive, all in
  the closed `lexlean/semantic-module/2` schema. Every model declaration
  elaborates in linking to ordinary definitions and theorems, which the Lean
  backend, production eligibility, extraction, and the axiom audit consume;
  the document and the snapshot (new `elaboration` member) show the source
  and its elaboration.
  - Artifacts are content-addressed project files configured as
    `[[artifact_source]]` (language 1.2 only), read confined and counted
    toward `max_total_source_bytes`, decoded under the closed `bytes`,
    `int_tensor`, and `utf8_lines` schemas, recorded as `model-artifact`
    build-manifest inputs, and embedded in Lean with a kernel-checked
    decoding theorem.
  - Realizations are deterministic, rule, statistical (linear scoring),
    integer feed-forward neural (dense, ReLU, truncating requantization,
    first-maximum or function decoder), or composite (sequence, fan-out,
    product, branch, scan with unconditional, proved, or checked
    junctions; every form but a scan may compose stateful stages, threading
    the product of their states, proved junctions join stateful stages, and
    a stage whose evidence leaves its postcondition or output invariant open
    is checked right after it runs); widths, shapes, roles, labels, and descriptor slot
    types are checked in linking. A tensor declares at most 16 dimensions,
    checked with its declared type before any byte is decoded, and every
    artifact declaration is charged before it decodes: its length once more
    toward `max_total_source_bytes` and the exact node count of its decoded
    value toward `max_ir_nodes`, running across the link, so naming one
    digest many times cannot amplify memory.
  - Evidence claims (`satisfies_contract`, `preserves_invariant`,
    `initial_invariant`, `equivalent_to`, `dataset_agreement`) are generated
    statements discharged by statement-exact theorems and restated in Lean
    against the fixed `LexLeanModels` semantics; executable code must apply
    a model through `checked_apply` with every check its evidence does not
    discharge.
  - New diagnostics `LLR3007`, `LLR3008`, `LLT4006`, `LLT4007`, `LLT4008`,
    and `LLT4009`; new conformance IDs `MD-01` to `MD-12`; new example
    `examples/models` (nine modules and a title glossary), which produces and
    verifies with real Lean, including extraction, every claim form of the
    fixed model semantics and every runtime check and refusal; 33 new
    negative fixtures. The canonical document never prints an artifact's
    bytes or decoded value.
  - The language-1.2 `semantic_ir`, `lean_backend`, and `latex_backend`
    versions are bumped. Language-1.2 proposition-valued definitions lower
    `@[expose, reducible]`, `linear_arithmetic` closes with `all_goals
    omega`, and the extraction adapter reads LCNF types through metadata
    annotations. `fmt` stages a project's artifacts with its sources.
  - The public snapshot DTO grows: `SnapshotDeclaration::elaboration`
    (`SnapshotElaboration`, with the alpha identity of each elaborated
    definition), the model declaration variants, `CheckedApply`,
    `ContractViolation`, and `LessThan`. Downstream exhaustive matches must
    add them.
- Semantic preservation certifies the roots of `examples/reasoning`
  (`ReasoningFailure` is lowered as the pair of Booleans `ContractViolation` is,
  and the reasoning declarations are read through their elaborations). Two
  defects of the certificate generator that the older corpora did not reach are
  fixed: a source local that linking generated (`__b`, `__acc`) is spelled
  `_y...` in the certificates, because the certificates' own binders begin with
  two underscores and captured it, and the `match` of a generic template carries
  its result type, because Lean otherwise postpones it and a fold's step relation
  fails on the pending metavariable. `LexLeanReasoning` is a reserved module
  root, as `LexLeanModels` is.
- Language-1.2 reasoning machines (§17.12, issue #31): `logic`,
  `inference_rule`, `verifier`, and `reasoner` declarations and the
  `reasoning_failure` type, in the closed `lexlean/semantic-module/2`
  schema, with no oracle, prompt, or reasoner primitive. Each elaborates in
  linking to ordinary inductives, structures, definitions, and theorems over
  `match`, `if`, `list_fold`, `iterate_until`, and set primitives.
  - Rules are guarded transitions of a logic's state with statement-exact
    soundness and progress theorems; verifiers are executable checks with
    exact soundness (and completeness) theorems; reasoners are `forward`
    (first applicable rule in priority and candidate order, under fuel),
    `search` (breadth- or depth-first under fuel and a frontier bound,
    optionally deduplicating over an ordered state type), or
    `generate_and_verify` (a generator's candidates checked in order within a
    mandatory budget). Logics, rules, verifiers, and reasoners may be
    generic. Claims `initial_invariant`, `terminates` (forward only, from a
    ranking, progress on every rule, and an exact fuel bound),
    `observation_invariant` (a predicate of the observation and a state kept
    by every step), and `answer_correct` (the answer term correct on every
    state, or on every state satisfying the invariant or the observation
    invariant, which erases the check).
  - Verdicts are the verifier-accepted (or proved-correct) answer or one of
    `exhausted`, `unsolved`, `rejected`, `invalid_step`; a forward reasoner
    answers only from a saturated state. Traces are `E.Step` data, evidence
    only by replay. A six-counter ledger (iterations, guard evaluations,
    firings, expansions, verifications, frontier peak) accounts every run.
  - Generated theorems are fixed-template applications of the formal-only,
    axiom-free `LexLeanReasoning` runtime and are restated by pinned Lean;
    executable code reaching a rule's conclusion or an unverified answer is
    refused (`LLT4012`).
  - New diagnostics `LLT4010`, `LLT4011`, and `LLT4012`; new conformance IDs
    `RS-01` to `RS-14`; new example `examples/reasoning` (five modules and a
    title glossary) with the six-counter ledger of each run decided by the
    kernel; 85 new negative fixtures; the `compiler` project's five oracle
    modules (`ReasoningOracle`, `GradeOracle`, `BudgetOracle`,
    `PlannerOracle`, `ScreeningOracle`) stating every reasoner the example's
    production roots run, with calculus transcriptions of each strategy
    (the forward engines `Triage`, `Review`, `Grade`, and the generic
    `Spend`; the breadth-first search with deduplication `Plan`; the
    depth-first search `Screen`; the generate-and-verify `Dose`) and their
    rust-core and rust-std packages, in the new `ReasoningFixtures` module;
    GNAF requests over forward-chaining plans, each plan tied to the
    oracle's `Triage` on the request's domain.
  - `answer_correct` names a theorem of one of three exact statements: the
    answer correct on every state (`forall x s v, a = some v -> Spec x v`),
    on every state satisfying the logic's invariant (`forall x s v, J s -> a
    = some v -> Spec x v`, which needs the `initial_invariant` claim and
    whose premise the generated run theorem `E.run_invariant` discharges),
    or on every state a new `observation_invariant` claim holds of
    (`forall x s v, Rl x s -> a = some v -> Spec x v`, where `Rl : (I, S) ->
    prop` holds of the observed state and is kept by the logic's relation,
    so `Spec` may depend on the observation: `Grade` answers a chart's level,
    at most 3 and 3 only for a hypotensive patient). The check of a reasoner
    that claims any form is erased in its own verdict, but only an answer
    correct on every state may be read by executable code without its
    verifier: under an invariant, `E.extract`, `E.accept`, and `E.conclude`
    (forward) or `E.searchStep` (search) are `LLT4012`, because each reads
    an answer from a state or search the caller supplies.
  - A declaration named like, or below, any namespace the generated Lean
    refers to (`LexLeanReasoning.Star`, but also `Nat.x` or `List.y`) is
    refused in language 1.2 (`LLT4001`): the reservation is not limited to
    reasoning modules.
  - The elaboration charge of a reasoner covers the type arguments of every
    use of its logic, rules, and verifier; it remains a fitted bound checked
    after elaboration, not a proof.
  - Generated bounds also state a forward run's firings and a search's
    verifications and expansions at most the fuel (`E.firings_bounded`,
    `E.verifications_bounded`, `E.expansions_bounded`), recorded in the
    eligibility report; a run's guard evaluations and a search's firings are
    accounted, not bounded.
  - Every logic, rule, verifier, and reasoner is charged to `max_ir_nodes`
    for the most its elaboration can be before it is elaborated
    (`LLS8002`), and the rules of a reasoner are combined as balanced
    trees, so an elaboration nests with the logarithm of its rule count.
  - The language-1.2 `semantic_ir`, `lean_backend`, and `latex_backend`
    versions are bumped; `linear_arithmetic` also rewrites with
    `Bool.and_eq_true`, `Bool.or_eq_true`, `Bool.not_eq_true'`, `and_true`,
    `true_and`, and `Option.some.injEq`. The eligibility report gains
    per-root `reasoning` rows, and its schema now admits the model and
    reasoning construct keys. The snapshot elaboration gains `theorems`.
- A production root that applies a collection, fold, iteration, or graph
  primitive now admits `recursion` (SPEC.md §17.13, `TC-06`; #105). Those
  primitives are realized by library templates that call themselves, so the
  stack depth of the rendered Rust is bounded only by the input (a scan of an
  80,000-byte name aborted on an 8 MiB stack), yet their registry rows said
  `recursion = false` and no root declared the effect. The rows are corrected,
  and `conformance_tc_06` derives each runtime row's `recursion` from its
  realization (the call graph of a template's instance, or the structural
  `recursion` reference) instead of trusting the row. The language-1.2
  compiler-semantics ID changes with the registry; `examples/reasoning` and
  `examples/production-coverage` declare the effect on the roots that use these
  primitives.
- The host legs of CI may run for 120 minutes, where they were cut off at 60:
  the Intel macOS leg takes 51 to 61 minutes, and a leg cancelled at its limit
  is not a result about the code.

## 0.3.0

- Support exhaustive Boolean matches and keep imported list construction
  definitions exposed for kernel reduction across generated modules.
- Preserve digit runs inside `semanticdata` JSON string values as exact string
  content. Exact content identities beginning with zero no longer trigger the
  standalone LexLean numeral canonicalization diagnostic; JSON numeric tokens
  and ordinary source numerals retain their existing leading-zero rejection.
  Payload tracking is linear in source size and respects whitespace, escaped
  quotes, and control-shaped bytes inside strings.
- Treat Lean comment delimiters inside generated string literals as data while
  retaining the backend invariant that generated Lean syntax contains no line
  or block comments.
- Generic language support for production modeling: generic semantic identifier
  quotation for reserved Lean keywords/segments, full Lean string escape sequence
  support without parser divergence, and bounded semantic elaboration for large byte
  constants.
- Immutable typed authority binding and oracle evidence: Lean 4.32.1, Lake,
  leanchecker, and `#print axioms` bound to upstream Lean FRO release 4.32.1, source commit
  `f054605aea4b840552cca2e725580bffd1e1b704`, and archive SHA-256
  `6dec8667fbf57ba480a18a8b0c353b2ee157346b2630b211ccbefeedf20545f8`, with positive
  and negative oracle execution records.
- First-party package identity and publishing bootstrap: owner-controlled initial token
  upload support in release workflow before OIDC trusted publishing transition, and
  standalone offline package verification (`cargo xtask check-package`).
- Release identity provenance and tree manifest package: `release/release-identity.json`
  and `release/MANIFEST.sha256` binding all 712 packaged files, consumed by PrismPM
  without source-only assumptions.

## 0.2.0

Language 1.1 gains the closed portable data and operation vocabulary required
by generated application runtimes. This is the immutable PrismPM integration
line; it does not weaken the separate LexLean 1.0.0 full-spec release gate.

### Implemented

- Distinct mathematical `Int` and fixed-width signed/unsigned integer types,
  UTF-8 strings, byte sequences, ordering, option, and result values, with
  canonical checked literals and no implicit host-width conversion.
- Twenty-eight generic typed operations covering checked arithmetic and
  conversion, quotient/remainder zero cases, bit operations and bounded
  shifts, collections and byte ranges, UTF-8, byte ordering, bounded exact
  split/join, and canonical decimal parse/format.
- One fixed Lean 4.32.1 runtime lowering and canonical LaTeX rendering for the
  portable vocabulary. Definitions and theorems both carry exact observed
  axiom policies; verification still elaborates, replays with `leanchecker`,
  and audits every policy.
- `SM-17` through `SM-22`, an all-operation semantic fixture, exhaustive
  fixed-width bound/signature tests, schema validation, malformed-input cases,
  and cross-root snapshot identity checks. The register now contains 222
  implemented capability IDs.

### Compatibility

- Language 1.0 syntax and meaning are unchanged. Language 1.1 and
  `lexlean/semantic-snapshot/1` are extended in place under the ecosystem's
  pre-freeze policy; their compiler-semantics digest and affected expected
  artifacts are regenerated.
- No Prism-, Holo-, or Calculator-specific node, raw Lean/Rust field, macro,
  tactic, or backend escape hatch is added.

### Not claimed

- This remains an integration release rather than the complete LexLean §30
  release, which is intentionally reserved for version 1.0.0.

## 0.1.1

The UOR Atlas becomes the foundation model of LexLean: every accepted document
carries the Atlas-derived header, and the Atlas formalization is closed inside
the repository.

### Implemented

- The one-time Atlas conversion is closed and recorded in
  `examples/uor-atlas/MIGRATION.md`; its independently authored Lean source and
  exporter are absent from the release tree. `VR-19` now permanently audits
  the native source graph itself: every source module has one generated Lean
  module, public imports stay within `Init` and the generated graph, the only
  backend-support import is `Lean`, and no second Atlas implementation exists.
  `just vv` verifies the generated native Atlas
  under `leanprover/lean4:v4.32.1`, replays it through `leanchecker` — a
  same-kernel replay, not an independent checker (§22.4) — and runs the
  standing exact axiom gate.
- Two new built-in lexicon packages, `lexlean.std.int@1.0.0` and
  `lexlean.uor.atlas@1.0.0`. The Atlas package is registered under
  `[[builtin_package]]`, locked into every project, and unconditionally
  visible in every document, so the header is carried whether or not a
  document names an Atlas entry; its visibility closes transitively over
  `lexlean.core`, `lexlean.std.nat`, and `lexlean.std.int`.
- The frozen Atlas pack is complete against the native source register: every
  label carries exactly one disposition, the registers key on exact
  identifiers, and every frozen entry refers to a declaration owned by the
  native source rather than importing an independently authored module.
  Coverage begins at `Atlas.lex.tex`; exercise, denotation,
  surface-disjointness, and authority-scope audits run as gates,
  each with a planted-defect record in VERIFICATION.md.
- `examples/uor-atlas/` verifies under the pinned toolchain with its
  committed verification records, and the negative fixture suite grows to 28
  classes with `atlas-level-conflation`, rejected by `LLR3005`: a native Atlas
  document declaration cannot be consumed as an external glossary atom.
- `examples/uor-atlas/src/Atlas.lex.tex` is the single native semantic and
  proof source for 5,519 environment declarations; 58 private source-compiler
  implementation records remain hashed provenance and are emitted by neither
  backend. Both backends traverse that closed DAG, generated Lean publicly
  imports only `Init` and generated Atlas modules, privately imports only the
  generic `Lean` support module, and contains no independently authored Atlas
  implementation.
  `S43` is authoritatively the proved integer-uniqueness statement.
  `SM-15` and `VR-19` bring the register to 211 IDs, all implemented at level
  `build`.
- Language `1.1` adds a closed generic semantic declaration, term, and proof
  language for structures, classes, instances, finite inductives, total
  structural recursion, exhaustive matches, Boolean validators, exact theorem
  application, and axiom-free Boolean reflection. Its seven-module
  `semantic-1.1` fixture contains no handwritten Lean and exercises every
  closed variant through elaboration, `leanchecker`, and exact axiom audit.
- The stable seventh `Engine` operation returns an owned, read-only,
  path-independent `lexlean/semantic-snapshot/1` DTO. The public DTOs and
  complete closed JSON Schema expose every legal semantic module variant while
  keeping mutable compiler internals and both fixed backends private. `SM-16`,
  `DF-11`, `CF-16`, `CL-19`, and `CL-20` bring the register to 216 IDs, all
  implemented at level `build`.

### Changed

- Generated-module verification passes Lean an explicit package root with
  `-R`. This removes random staging paths from `.olean` serialization, so a
  same-platform verification has byte-identical oleans and an identical
  attestation across absolute project roots.
- The compiler-semantics ID moves from
  `fa171c7a2d78cf17e6cb49bbec5c1eed8bee20033472b1953211104068589ba7` to
  `95deb33a8d416d7bf60f02a36e251a71c3ee6f046b7e475d7bae2fc5ddc3767d`:
  the accepted language changed, so §30.1 requires a new ID. Every committed
  lock and verification record is regenerated against it.
- Language `1.1` has its independent compiler-semantics ID
  `0accaf7b80d572e21451d5fa650d92a1a28dae799823931b2f756e448fe89996`;
  language-1.0 locks and generated artifacts remain byte-identical.
- The four 0.1.0 examples still format byte-identically and generate
  byte-identical Lean and LaTeX modules; their source maps and manifests
  differ only in the source and semantic digests those artifacts embed. No
  previously-accepted run changed its generated bytes (§30.2).

### Not claimed

- This is not a §30 release. `cargo xtask release-check` refuses at `0.1.1`,
  and the release criterion is met only at `1.0.0`.
- The native Atlas graph's verification status is a `build` claim: it elaborates,
  replays, and reports no axiom outside Lean's own. The mathematical content
  is the specification's, cited at `some-true`, and the Lean kernel and
  elaborator beneath it are cited, not verified; the honest claim is that the
  Atlas is as sound as Lean 4.32.1, not that it is sound absolutely.
- Verified status is claimed by `verify` alone. `check` and `build` never
  claim it (`VR-18`), and `leanchecker` is a same-kernel replay, never
  described as an independent checker (§22.4).

## 0.1.0

The initial implementation of `LEXLEAN-SPEC-1`.

### Implemented

All 216 conformance IDs of SPEC.md §31 are implemented at honesty level
`build`: constructed in this repository and validated against an oracle by the
test named `conformance_<id>`. [CONFORMANCE.md](CONFORMANCE.md) is the
generated register, [ERRORS.md](ERRORS.md) the closed diagnostic registry, and
[VERIFICATION.md](VERIFICATION.md) the falsifiability record for every gate.

- Closed project configuration, canonical lock file, and offline dependency
  policy (`CF-01`..`CF-16`).
- Total lexical closure over every accepted atom (`LX-01`..`LX-14`) and
  versioned lexicon packages with closed schemas, denotations, and renderer
  tokens (`GL-01`..`GL-16`).
- The fixed structural, mathematical, and proposition grammar with closed
  ambiguity handling (`GR-01`..`GR-16`), the typed closed IR with canonical
  serialization, native modules, and language-1.1 snapshots (`SM-01`..`SM-16`),
  and generic semantic declarations plus document definitions with exact
  self-application and acyclicity rules (`DF-01`..`DF-11`).
- The structured proof language with pinned Lean lowerings (`PF-01`..`PF-18`),
  prose-free deterministic generated Lean with complete token traceability
  (`LN-01`..`LN-12`), and canonical LaTeX regeneration with the optional
  hash-checked PDF provider (`TX-01`..`TX-12`).
- Canonical diagnostics, source maps, coverage, manifests, and reproducible
  builds (`AR-01`..`AR-14`); fifteen-stage verification with `leanchecker`
  replay and exact axiom audit (`VR-01`..`VR-19`); the exact CLI contract and
  the stable seven-method Rust `Engine` API (`CL-01`..`CL-20`); filesystem
  confinement, no shell, no hidden network, and the closed failure model
  (`SE-01`..`SE-12`).
- Six example projects that verify under the pinned `leanprover/lean4:v4.32.1`
  toolchain, and the complete negative fixture suite (`EX-01`..`EX-08`).

### Not claimed

- This is not a §30 release. `cargo xtask release-check` refuses at `0.1.0`,
  and the release criterion is met only at `1.0.0`.
- Verified status is claimed by `verify` alone. `check` and `build` never claim
  it (`VR-18`), and `leanchecker` is a same-kernel replay, never described as
  an independent checker (§22.4).
- Facts about external tools are level `some-true` rows in
  [`model/ledger.toml`](model/ledger.toml): reproduced from cited authorities,
  not established here.
- The normative verification and reproducibility gate runs on Linux x86-64
  (§8.3). The other four supported hosts build the crate and run every test
  that does not need the pinned toolchain or a POSIX shell; each such case
  reports which assertions it did not run.

### Known deviations

The README's "Documented deviations" section lists every place the generated
bytes differ from a literal reading of SPEC.md, with the reason. Each is
enforced by the same golden and conformance gates as everything else.
