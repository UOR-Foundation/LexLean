#!/bin/bash
# The array module's one-time export and its equivalence check
# (tests/runtime-library/ARRAYS.md). Run from a checkout with the pinned Lean
# 4.32.1 under ~/.elan, a built `lexlean`, and the `to-notation` helper built
# into the same target directory:
#   CARGO_TARGET_DIR=<root>/target cargo build --manifest-path notation/Cargo.toml
# LEXLEAN, NOTATION, and PYTHON name other builds of the three programs.
#
# usage: run.sh <repository root> <work dir>
#   1. `lexlean build` of a copy of the runtime project, narrowed to the one
#      module Arrays imports, gives the Lean of Portable; Arrays.lean (this
#      directory) is compiled against it.
#   2. flags.lean and export.lean export the Arrays environment; canon.py
#      writes the data form; to-notation prints it in the core notation, which
#      is written to language/runtime-1.2/library/src/Arrays.lex.tex.
#   3. `lexlean fmt --check` holds the written source canonical, and
#      `lexlean build` generates its Lean. The Portable it generates beside it
#      is the one Arrays.lean was compiled against, byte for byte.
#   4. That Lean is compiled, exported, and converted the same way; compare.py
#      compares the two declaration by declaration.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"; root="$1"; work="$2"
TC=$HOME/.elan/toolchains/leanprover--lean4---v4.32.1
export PATH=$TC/bin:$PATH
LEXLEAN="${LEXLEAN:-$root/target/debug/lexlean}"; NOTATION="${NOTATION:-$root/target/debug/to-notation}"
PYTHON="${PYTHON:-python3}"
rm -rf "$work"; mkdir -p "$work"
lib="$root/language/runtime-1.2/library"
export_env() { # <src dir with LexLeanRuntime/*.lean> <out dir>
  local src="$1" out="$2"
  mkdir -p "$out/olean/LexLeanRuntime" "$out/flags" "$out/export"
  export LEAN_PATH="$out/olean"
  for m in Portable Arrays; do
    lean -R "$src" -o "$out/olean/LexLeanRuntime/$m.olean" "$src/LexLeanRuntime/$m.lean" > "$out/$m.log" 2>&1 || { grep -A3 error "$out/$m.log" | head -30; exit 1; }
  done
  (cd "$out/flags" && lean --run "$here/flags.lean" > flags.log 2>&1)
  EXPORT_DIR="$out/export" EXPORT_MODS=Arrays lean --run "$here/export.lean" > "$out/export.log" 2>&1
  FLAGS="$out/flags" EXPORT="$out/export" "$PYTHON" "$here/canon.py" "$out/lextex" > "$out/canon.log" 2>&1
}
# 1: the Lean of Portable, from a project that does not name Arrays, so that
# the step is the same before the native source exists and after.
cp -r "$lib" "$work/runtime"
rm -rf "$work/runtime/.lexlean" "$work/runtime/src/Arrays.lex.tex"
sed -i 's|^entrypoints = .*|entrypoints = ["src/Portable.lex.tex"]|' "$work/runtime/lexlean.toml"
(cd "$work/runtime" && "$LEXLEAN" lock > /dev/null && "$LEXLEAN" build > /dev/null)
portable=$(ls -d "$work"/runtime/.lexlean/build/*/modules)
mkdir -p "$work/authored/LexLeanRuntime"
cp "$portable/LexLeanRuntime/Portable.lean" "$work/authored/LexLeanRuntime/"
cp "$here/Arrays.lean" "$work/authored/LexLeanRuntime/"
# 2: the export of the authored Lean, printed in the notation.
export_env "$work/authored" "$work/first"
"$NOTATION" "$work/first/lextex/Arrays.lex.tex" "$lib/src/Arrays.lex.tex"
# 3: the written source is canonical, and the project builds with it.
(cd "$lib" && "$LEXLEAN" lock > /dev/null && "$LEXLEAN" fmt --check && rm -rf .lexlean && "$LEXLEAN" build)
generated=$(ls -d "$lib"/.lexlean/build/*/modules)
cmp "$portable/LexLeanRuntime/Portable.lean" "$generated/LexLeanRuntime/Portable.lean"
echo "the Portable that Arrays.lean was compiled against is the one the project generates, byte for byte"
# 4: the regenerated environment of the native source.
export_env "$generated" "$work/second"
mkdir -p "$work/compare/original" "$work/compare/regenerated"
cp "$work/first/lextex/Arrays.lex.tex" "$work/compare/original/"
cp "$work/second/lextex/Arrays.lex.tex" "$work/compare/regenerated/"
"$PYTHON" "$here/compare.py" "$work/compare/original" "$work/compare/regenerated"
