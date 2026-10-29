#!/usr/bin/env bash
# repro-check.sh SRC_ROOT MODULE DEPS_LEAN_PATH
# Builds MODULE (a dotted name; its source is SRC_ROOT/<path>.lean) twice, each time from an
# empty output directory and a fresh copy of that one source file, with `lean -R` so no
# absolute path reaches the compiler, then compares every artifact byte for byte. LEAN_OPTS
# (e.g. "-DmaxSynthPendingDepth=3") is passed to `lean`. PACKAGE, when set, is the Lake package
# name Lake hands the compiler in its `--setup` file; it is recorded in the .olean, so a build
# meant to match a Lake build needs it. REFERENCE_DIR, when set, is a build root (a Lake build,
# a fetched cache) whose artifacts for MODULE are compared too. DEPS_LEAN_PATH holds the
# prebuilt oleans of the module's imports. Prints one line per artifact.
set -eu
src=$1; mod=$2; deps=$3
rel=$(echo "$mod" | tr . /)
work=$(mktemp -d)
for run in a b; do
  mkdir -p "$work/$run/src/$(dirname "$rel")" "$work/$run/out/$(dirname "$rel")"
  cp "$src/$rel.lean" "$work/$run/src/$rel.lean"
  setup=""
  if [ -n "${PACKAGE:-}" ]; then
    setup="--setup=$work/$run/setup.json"
    printf '{"name":"%s","package":"%s","isModule":true,"importArts":{},"dynlibs":[],"plugins":[],"options":{}}' "$mod" "$PACKAGE" >"$work/$run/setup.json"
  fi
  (cd "$work/$run/src" && LEAN_PATH="$deps" lean ${LEAN_OPTS:-} $setup -R . -o "$work/$run/out/$rel.olean" \
      -i "$work/$run/out/$rel.ilean" -c "$work/$run/out/$rel.c" "$rel.lean")
done
rc=0
for f in $(cd "$work/a/out" && find . -type f | sort); do
  if cmp -s "$work/a/out/$f" "$work/b/out/$f"; then echo "identical  $f  $(sha256sum < "$work/a/out/$f" | cut -c1-16)"
  else echo "DIFFERENT  $f"; rc=1; fi
done
if [ -n "${REFERENCE_DIR:-}" ]; then
  for f in $(cd "$work/a/out" && find . -type f -not -name "*.c" | sort); do
    if cmp -s "$work/a/out/$f" "$REFERENCE_DIR/$f"; then echo "matches-reference  $f"
    else echo "DIFFERS-FROM-REFERENCE  $f"; rc=1; fi
  done
fi
[ "${KEEP:-0}" = 1 ] && echo "kept $work" || rm -rf "$work"
exit $rc
