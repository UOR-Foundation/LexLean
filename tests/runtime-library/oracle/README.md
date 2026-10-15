# Runtime migration oracle

One-time, never a release input (SPEC.md §17.12, *Runtime modules*). It
converted the Lean runtime texts the backend spliced into every generated
language-1.2 module into native `lexlean/core-module/2` sources, checked that the
conversion lost nothing, and renamed the native proof libraries' references to
the per-module runtime copies. The commit after the one that adds this
directory removes it; the record of what it established is
`tests/runtime-library/MIGRATION.md`.

Inputs: `crates/lexlean/src/backend/semantic.rs` at `main` `d284b165`, the
calculus modules the compiler project generated before and after this change,
the pinned Lean `4.32.1`, Python 3. The exporter, `flags.lean`, `probe.lean`,
`canon.py` and `compare.py` are those of the preservation library's migration
(`tests/preservation-library/MIGRATION.md`, checkpoint `d54dbf85`), with the
runtime's module list.

1. `extract.py <semantic.rs> src`: lifts `portable_runtime_1_2` (the frozen
   portable runtime with every definition exposed, replayed exactly),
   `COLLECTIONS_RUNTIME`, `MODELS_RUNTIME` and `REASONING_RUNTIME` byte for byte
   into `src/LexLeanRuntime/{Portable,Collections,Models,Reasoning}.lean`, each
   with the header and options a generated language-1.2 module carries.
2. `build.sh`: compiles them into `out/`.
3. `flags.lean`, run in a scratch directory with `LEAN_PATH=out`: `sparse.json`
   and `compiled.txt`.
4. `export.lean`, run with `LEAN_PATH=out` and `EXPORT_DIR=<dir>`: one JSON file
   per module.
5. `canon.py <dir>` with `EXPORT=` and `FLAGS=`: the canonical sources as
   data (`\coredata`), which `lexlean::sdk::to_notation` writes in the core
   notation (§17.21.8): what `language/runtime-1.2/library/src/` holds.
6. `equivalence.sh <generated> <committed src> <work>`: the Lean `lexlean
   build` generates from those sources is compiled, exported and converted with
   the same programs; the result is compared with the committed sources, and
   `compare.py` reports, declaration by declaration, what differs.
7. `compile_tree.py`, `walk.py`: the calculus modules as they were (`main`) and
   as they are now generated are compiled and exported (`export-calculus.lean`:
   `export.lean` with the calculus modules' list and the check that refuses a
   nested inductive lifted, since only kernel terms are compared), and walked
   in parallel; the result is the pairing of their constants.
8. `rename.py --mapping <pairing>`: renames the preservation and the
   certification libraries' references by that pairing and the runtime rule.
   It reads and writes the data form: each library source is converted with
   `lexlean::sdk::to_data_form` before and `to_notation` after, a round trip
   that changes no byte of an unrenamed source.
