#!/usr/bin/env bash
# Make the pinned Mathlib and its packages available to this example's Lake
# workspace without a network: hard links to an already fetched checkout, so no
# extra disk. `lexlean deps fetch` is the way to acquire the library when there
# is no such checkout. Usage: MATHLIB=/path/to/pinned/checkout ./setup-offline.sh
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
mathlib="${MATHLIB:-$HOME/mathlib-pinned}"
revision="520045ab14e26149ee970e2e617ca04b09bde5d6"
[ "$(cat "$mathlib/.git/HEAD")" = "$revision" ] || { echo "$mathlib is not at $revision" >&2; exit 1; }
mkdir -p "$here/.lake/packages"
[ -e "$here/.lake/packages/mathlib" ] || cp -al "$mathlib" "$here/.lake/packages/mathlib"
for package in "$mathlib"/.lake/packages/*/; do
  name="$(basename "$package")"
  [ -e "$here/.lake/packages/$name" ] || cp -al "$package" "$here/.lake/packages/$name"
done
(cd "$here" && lake build Blake3GHost)
