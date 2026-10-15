#!/bin/bash
# The migration's equivalence check: the Lean the compiler generates from the native runtime modules is compiled
# by the pinned Lean, its environment is exported again by the same exporter, and the result, converted by the same
# conversion, must equal the committed native modules byte for byte; the attribute probe of both environments must
# agree.
# usage: equivalence.sh <generated modules dir, containing LexLeanRuntime/*.lean> <committed src dir> <work dir>
set -uo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
generated="$1"; committed="$2"; work="$3"
TC=$HOME/.elan/toolchains/leanprover--lean4---v4.32.1
export PATH=$TC/bin:$PATH
rm -rf "$work"; mkdir -p "$work/src" "$work/out/LexLeanRuntime" "$work/flags" "$work/export"
cp -r "$generated/LexLeanRuntime" "$work/src/"
export LEAN_PATH="$work/out"
: > "$work/summary.txt"
for m in Portable Collections Models Reasoning; do
  s=$(date +%s)
  lean -R "$work/src" -o "$work/out/LexLeanRuntime/$m.olean" "$work/src/LexLeanRuntime/$m.lean" > "$work/out/$m.log" 2>&1
  echo "$m rc=$? seconds=$(( $(date +%s) - s )) output_bytes=$(wc -c < "$work/out/$m.log")" >> "$work/summary.txt"
done
(cd "$work/flags" && lean --run "$here/flags.lean" > flags.log 2>&1)
EXPORT_DIR="$work/export" lean --run "$here/export.lean" > "$work/export.log" 2>&1
(cd "$here" && FLAGS="$work/flags" EXPORT="$work/export" python3 canon.py "$work/lextex" > "$work/canon.log" 2>&1)
for m in Portable Collections Models Reasoning; do
  if cmp -s "$work/lextex/$m.lex.tex" "$committed/$m.lex.tex"; then echo "$m byte-equal" >> "$work/summary.txt"; else echo "$m DIFFERS" >> "$work/summary.txt"; fi
done
# the attribute state of every constant, read by the pinned Lean's own queries from both environments
LEAN_PATH="$here/out" lean --run "$here/probe.lean" > "$work/probe-original.txt" 2>&1
LEAN_PATH="$work/out" lean --run "$here/probe.lean" 2>&1 | grep -v "LexLeanCore.Runtime" > "$work/probe-regenerated.txt"
if diff "$work/probe-original.txt" "$work/probe-regenerated.txt" > "$work/probe.diff"; then echo "probe equal ($(wc -l < "$work/probe-original.txt") lines)" >> "$work/summary.txt"; else echo "probe DIFFERS" >> "$work/summary.txt"; fi
echo finished >> "$work/summary.txt"
