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
  `max_ir_nodes`, or whose largest certificate is certain to exceed
  `max_file_bytes` (a lower bound from what the program states: 16 bytes for
  each type node, 20 for each expression node, 25 for each shape node, 11
  for each pair of arms of a match, 3 for each byte of a string, and 1000;
  calibrated at half of the most the corpora and stress families allow,
  never above their largest certificate in `conformance_sp_02`, so a program
  whose certificates fit is not refused by it), or whose field reads print so
  many record entries that certificate B would exceed it, is refused with
  `LLS8002` before the toolchain is touched; certificate B, whose derivations
  grow with the square of a record's arity and cannot be bounded from node
  counts, is
  generated under `max_file_bytes` and stops at it. Certificates are now
  generated right after lowering for that reason. Negative fixtures
  `lowering-size-limit`, `certificate-size-limit` (a record of 800 fields
  copied field by field), and `certificate-generation-limit` (a sum of the
  500 fields of a record) cover them. A pinned Lean that is killed, runs out
  of memory, or overflows its stack while checking a certificate is
  `LLS8002`; one that exhausts its heartbeat or recursion budget is `LLS8002`
  only when the module is at least a quarter of `max_file_bytes` and is
  otherwise Lean's verdict on the certificate, `LLV7013`, `LLV7015`, or
  `LLV7016` (fixtures `certificate-heartbeat-rejected` and
  `certificate-resource-exhausted`): a wrong certificate whose proof makes
  `isDefEq` loop is not a limit. Rejections of certificates A, B, and E name
  the declaration, the root, and the bounded first error.
- Every certificate is generated under `max_file_bytes` (the proof of each
  term, each match arm, the encoders, each function, the boundary), so no
  program makes the generator build more than the limit allows; the early
  refusal is a lower bound, not the guarantee. A
  match of thousands of arms cannot overflow the stack (the aligner derives
  arms in a loop and a derivation is written and dropped iteratively).
- A root verifies whatever its parameters are named: the names certificate E
  binds begin with two underscores, which no semantic name does, and the
  certificate token audit reads a quoted name as the name it is, forbidden
  constants and attributes included (the generator quotes a name spelled like
  a forbidden token, and the backend one spelled like a word the generated-Lean
  audit forbids, such as `native_decide`), while an unclosed quotation, string,
  or comment is refused; the words a certificate writes bare, which a parameter
  could shadow (`true`, `cond`, `absurd`, `denote`, `accepts`, ...), are
  written by their full names, and `conformance_sp_09` checks that no bare
  word of a certificate is anything but syntax.
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
