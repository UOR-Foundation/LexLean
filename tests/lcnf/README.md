# The extraction adapter's registry and oracle

Named-root extraction (SPEC.md §22.10) reads Lean's compiler front end through
an adapter. The adapter is a closed tooling module (§17.20), the project
`language/lcnf-1.2/tooling/`, and nothing in this directory is part of the
language data or of the compiler-semantics ID.

- `export.lean` is the *exporter*, run by the pinned Lean. Given requests
  (`call:`, `inductive:`, `opaque:`, `alias:` and a constant) it states each as
  a registry row of the tooling schema (`tooling/schemas/tooling-module.schema.json`):
  the exact signature of a call, the constructors and typed fields of a type,
  and the type a constant abbreviates, and it closes the set over every constant
  those signatures and fields name.
- `registry.json` is the exporter's output for the requests the adapter's terms
  make (`crates/conformance/src/lcnf_registry.rs`), with the toolchain source
  file that defines each row and its SHA-256, and the plumbing: the constants
  the elaborated adapter uses that no row declares. Nobody edits it. `cargo xtask
  check-calculus --write` regenerates it from the pinned Lean, and
  `conformance_ne_04` requires it to equal a fresh derivation.
- `oracle/extract.lean` is the handwritten adapter the tooling module replaced.
  It is *validation evidence* and takes no part in any build: `conformance_ne_04`
  runs it beside the module on every kind of constant the extraction classifies
  and requires the same record.

The adapter's functions are written, as the calculus is, as the terms that build
them (`crates/conformance/src/lcnf_adapter.rs` and `lcnf_extract.rs`); the
committed project is exactly what they and `registry.json` render, which
`cargo xtask check-calculus` enforces.
