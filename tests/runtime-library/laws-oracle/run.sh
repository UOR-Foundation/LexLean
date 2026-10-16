#!/bin/bash
# The fold-law module's one-time export and its equivalence check
# (tests/runtime-library/LAWS.md). Run from a checkout with the pinned Lean
# 4.32.1 under ~/.elan and a built `lexlean`.
#
# usage: run.sh <repository root> <work dir>
#   1. `lexlean build` of the runtime project gives the Lean of Portable and
#      Collections; Laws.lean (this directory) is compiled against them.
#   2. flags.lean and export.lean export the Laws environment; canon.py writes
#      the data form; `lexlean fmt` prints it in the core notation: the
#      committed language/runtime-1.2/library/src/Laws.lex.tex.
#   3. The Lean `lexlean build` generates from that committed source is
#      compiled, exported, and converted the same way; compare.py compares the
#      two declaration by declaration.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
root="$1"; work="$2"
TC=$HOME/.elan/toolchains/leanprover--lean4---v4.32.1
export PATH=$TC/bin:$PATH
LEXLEAN="$root/target/debug/lexlean"
rm -rf "$work"; mkdir -p "$work"
cp -r "$root/language/runtime-1.2/library" "$work/runtime"
(cd "$work/runtime" && "$LEXLEAN" lock > /dev/null && "$LEXLEAN" build > /dev/null)
generated=$(ls -d "$work"/runtime/.lexlean/build/*/modules)
export_env() { # <src dir with LexLeanRuntime/*.lean> <out dir>
  local src="$1" out="$2"
  mkdir -p "$out/olean/LexLeanRuntime" "$out/flags" "$out/export"
  export LEAN_PATH="$out/olean"
  for m in Portable Collections Laws; do
    lean -R "$src" -o "$out/olean/LexLeanRuntime/$m.olean" "$src/LexLeanRuntime/$m.lean" > "$out/$m.log" 2>&1
  done
  (cd "$out/flags" && lean --run "$here/flags.lean" > flags.log 2>&1)
  EXPORT_DIR="$out/export" EXPORT_MODS=Laws lean --run "$here/export.lean" > "$out/export.log" 2>&1
  (FLAGS="$out/flags" EXPORT="$out/export" python3 "$here/canon.py" "$out/lextex" > "$out/canon.log" 2>&1)
}
# 1-2: the export of the authored Lean.
mkdir -p "$work/authored/LexLeanRuntime"
cp "$generated/LexLeanRuntime/Portable.lean" "$generated/LexLeanRuntime/Collections.lean" "$work/authored/LexLeanRuntime/"
cp "$here/Laws.lean" "$work/authored/LexLeanRuntime/Laws.lean"
export_env "$work/authored" "$work/first"
# 2: the data form printed in the notation equals the committed source.
cp -r "$root/language/runtime-1.2/library" "$work/notation"
cp "$work/first/lextex/Laws.lex.tex" "$work/notation/src/Laws.lex.tex"
(cd "$work/notation" && "$LEXLEAN" lock > /dev/null && "$LEXLEAN" fmt > /dev/null)
cmp "$work/notation/src/Laws.lex.tex" "$root/language/runtime-1.2/library/src/Laws.lex.tex"
echo "the export printed in the notation is the committed source, byte for byte"
# 3: the regenerated environment of the committed native source.
export_env "$generated" "$work/second"
mkdir -p "$work/compare/original" "$work/compare/regenerated"
cp "$work/first/lextex/Laws.lex.tex" "$work/compare/original/"
cp "$work/second/lextex/Laws.lex.tex" "$work/compare/regenerated/"
python3 "$here/compare.py" "$work/compare/original" "$work/compare/regenerated"
