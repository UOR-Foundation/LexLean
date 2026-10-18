#!/bin/bash
# The migration's equivalence check (see README.md): the Lean `lexlean build` generates from a library tree is
# compiled by the pinned Lean, its environment is exported by the exporter of checkpoint d54dbf85, and the export is
# converted by that checkpoint's canon.py.
# usage: equivalence.sh <oracle dir of d54dbf85> <base> <generated LexLeanPreservation dir> <work dir>
#   <base> holds src/{LexLeanRuntime,LexLeanTarget} (generated runtime modules and the shipped calculus modules) and
#   out/ (their oleans, compiled in the order Portable, Collections, TargetSyntax, TargetSemantics, RustSyntax,
#   RustSemantics).
set -uo pipefail
oracle="$1"; base="$2"; generated="$3"; work="$4"
TC=$HOME/.elan/toolchains/leanprover--lean4---v4.32.1
export PATH=$TC/bin:$PATH
rm -rf "$work"; mkdir -p "$work/src" "$work/flags" "$work/export"
cp -r "$generated" "$work/src/LexLeanPreservation"
cp -r "$base/src/LexLeanRuntime" "$base/src/LexLeanTarget" "$work/src/"
cp -r "$base/out" "$work/out"
export LEAN_PATH="$work/out"
mkdir -p "$work/out/LexLeanPreservation"
: > "$work/summary.txt"
for m in Core Values Primitives Fixed Keys Templates Validate RustBase RustCorr RustLemmas RustMatches RustSound Compose; do
  s=$(date +%s)
  lean -R "$work/src" -o "$work/out/LexLeanPreservation/$m.olean" "$work/src/LexLeanPreservation/$m.lean" > "$work/out/$m.log" 2>&1
  echo "$m rc=$? seconds=$(( $(date +%s) - s ))" >> "$work/summary.txt"
done
(cd "$work/flags" && lean --run "$oracle/flags.lean" > flags.log 2>&1)
EXPORT_DIR="$work/export" lean --run "$oracle/export.lean" > "$work/export.log" 2>&1
FLAGS="$work/flags" EXPORT="$work/export" python3 "$oracle/canon.py" "$work/lextex" > "$work/canon.log" 2>&1
lean --run "$oracle/probe.lean" 2>&1 | grep -v "LexLeanCore.Runtime" > "$work/probe.txt"
echo finished >> "$work/summary.txt"
