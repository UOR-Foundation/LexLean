#!/usr/bin/env bash
# Regenerate tests/glossary/extraction.json from GlossaryFixtures.lean with the
# pinned Lean. The conformance cases GL-19, GL-20, and CL-25 compare a fresh
# extraction with this file when the pinned toolchain is available, so a change
# to the extractor or to the fixtures shows up as a diff here.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$here/../.." && pwd)"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
cd "$root"
cp "$root/lean-toolchain" "$work/lean-toolchain"
cp "$here/GlossaryFixtures.lean" "$work/GlossaryFixtures.lean"
cp "$root/crates/lexlean/lean/GlossaryExtract.lean" "$work/GlossaryExtract.lean"
(cd "$work" && lean -o GlossaryFixtures.olean GlossaryFixtures.lean)
(cd "$work" && LEAN_PATH="$work" lean --run GlossaryExtract.lean -- --modules GlossaryFixtures) \
  > "$here/extraction.json"
echo "wrote $here/extraction.json"
