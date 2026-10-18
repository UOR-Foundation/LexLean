#!/bin/bash
# Compile the 13 library modules, generated from a library tree by `lexlean build`, over a base holding the generated
# runtime modules and the machine modules under test, and report the errors of each (a module with an error writes no
# .olean, so the modules after it report one import error).
# usage: libcheck.sh <base> <generated LexLeanPreservation dir> <work>
#   <base> holds src/{LexLeanRuntime,LexLeanTarget} and out/ (their oleans, compiled in the order Portable, Collections,
#   TargetSyntax, TargetSemantics, RustSyntax, RustSemantics).
set -u
export PATH=$HOME/.elan/toolchains/leanprover--lean4---v4.32.1/bin:$PATH
base="$1"; generated="$2"; work="$3"
rm -rf "$work"; mkdir -p "$work/src"
cp -r "$generated" "$work/src/LexLeanPreservation"
cp -r "$base/src/LexLeanRuntime" "$base/src/LexLeanTarget" "$work/src/"
cp -r "$base/out" "$work/out"
export LEAN_PATH="$work/out"
mkdir -p "$work/out/LexLeanPreservation"
: > "$work/summary.txt"
for m in Core Values Primitives Fixed Keys Templates Validate RustBase RustCorr RustLemmas RustMatches RustSound Compose; do
  s=$(date +%s)
  lean -R "$work/src" -o "$work/out/LexLeanPreservation/$m.olean" "$work/src/LexLeanPreservation/$m.lean" > "$work/out/$m.log" 2>&1
  echo "$m rc=$? errors=$(grep -c 'error' $work/out/$m.log) seconds=$(( $(date +%s) - s ))" >> "$work/summary.txt"
done
echo finished >> "$work/summary.txt"
