#!/bin/bash
# The decision-regions module's one-time export and its equivalence check
# (tests/runtime-library/REGIONS.md). Run from a checkout with the pinned Lean
# 4.32.1 under ~/.elan, a built `lexlean`, and the `to-notation` helper built
# into the same target directory:
#   CARGO_TARGET_DIR=<root>/target cargo build --manifest-path notation/Cargo.toml
#
# usage: run.sh <repository root> <work dir>
#   1. Regions.lean (this directory) is compiled; flags.lean and export.lean
#      export its environment; canon.py writes the data form; to-notation
#      prints it in the core notation, which is written to
#      language/runtime-1.2/library/src/Regions.lex.tex.
#   2. `lexlean fmt --check` holds the written source canonical, and
#      `lexlean build` generates its Lean.
#   3. That Lean is compiled, exported, and converted the same way; compare.py
#      compares the two declaration by declaration.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"; root="$1"; work="$2"
TC=$HOME/.elan/toolchains/leanprover--lean4---v4.32.1
export PATH=$TC/bin:$PATH
LEXLEAN="$root/target/debug/lexlean"; NOTATION="$root/target/debug/to-notation"
rm -rf "$work"; mkdir -p "$work"
export_env() { # <src dir> <out dir>
  local src="$1" out="$2"
  mkdir -p "$out/olean/LexLeanRuntime" "$out/flags" "$out/export"
  export LEAN_PATH="$out/olean"
  lean -R "$src" -o "$out/olean/LexLeanRuntime/Regions.olean" "$src/LexLeanRuntime/Regions.lean" > "$out/Regions.log" 2>&1 || { grep -A3 error "$out/Regions.log" | head -30; exit 1; }
  (cd "$out/flags" && lean --run "$here/flags.lean" > flags.log 2>&1)
  EXPORT_DIR="$out/export" EXPORT_MODS=Regions lean --run "$here/export.lean" > "$out/export.log" 2>&1
  FLAGS="$out/flags" EXPORT="$out/export" python3 "$here/canon.py" "$out/lextex" > "$out/canon.log" 2>&1
}
mkdir -p "$work/authored/LexLeanRuntime"; cp "$here/Regions.lean" "$work/authored/LexLeanRuntime/"
export_env "$work/authored" "$work/first"
lib="$root/language/runtime-1.2/library"
"$NOTATION" "$work/first/lextex/Regions.lex.tex" "$lib/src/Regions.lex.tex"
(cd "$lib" && "$LEXLEAN" lock > /dev/null && "$LEXLEAN" fmt --check && rm -rf .lexlean && "$LEXLEAN" build > /dev/null)
generated=$(ls -d "$lib"/.lexlean/build/*/modules)
export_env "$generated" "$work/second"
mkdir -p "$work/compare/original" "$work/compare/regenerated"
cp "$work/first/lextex/Regions.lex.tex" "$work/compare/original/"
cp "$work/second/lextex/Regions.lex.tex" "$work/compare/regenerated/"
python3 "$here/compare.py" "$work/compare/original" "$work/compare/regenerated"
