#!/usr/bin/env bash
# measure-import.sh RUNS LABEL DECL MODULE...
# Elaborates a probe that imports MODULE... and mentions DECL, RUNS times, and prints one
# row per run (see measure.sh): label, wall seconds, peak RSS MiB, exit code. The first run
# is cold for the page cache; report runs 2.. as warm. `lean` and LEAN_PATH come from the
# caller, which pins them (the toolchain bin first on PATH, the Mathlib build on LEAN_PATH).
set -u
here=$(cd "$(dirname "$0")" && pwd)
runs=$1; label=$2; decl=$3; shift 3
probe=$(mktemp --suffix=.lean)
{ for m in "$@"; do echo "import $m"; done; echo "#check @$decl"; } >"$probe"
for i in $(seq 1 "$runs"); do
  MEASURE_LOG=/dev/null "$here/measure.sh" "$label#$i" lean "$probe"
done
rm -f "$probe"
