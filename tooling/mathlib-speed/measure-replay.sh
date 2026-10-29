#!/usr/bin/env bash
# measure-replay.sh PROBE_MODULE IMPORTED_MODULE
# Replays the same dependency under each strategy and prints one row each (see measure.sh).
# PROBE_MODULE is compiled (its .olean is on LEAN_PATH); IMPORTED_MODULE is the one Mathlib
# module it imports. Strategies:
#   own            leanchecker PROBE            the project module alone (today, SPEC 22.4)
#   module         leanchecker IMPORTED         one whole Mathlib module (its own constants)
#   modules-all    leanchecker <every module IMPORTED loads>   whole-module over the import closure
#   fresh          leanchecker --fresh IMPORTED every constant IMPORTED loads, into a fresh env
#   closure        closure-replay --module PROBE  only the constants PROBE (transitively) mentions
set -u
here=$(cd "$(dirname "$0")" && pwd)
CR=$here/closure-replay/.lake/build/bin/closure-replay
probe=$1; imported=$2
export HEAVY_MEM=${HEAVY_MEM:-7000M}
mods=$("$CR" --import "$imported" --list)
echo "# $probe imports $imported: $(echo "$mods" | wc -l) modules in the import closure"
"$here/measure.sh" "$probe/own" leanchecker "$probe"
MEASURE_LOG=/dev/stderr "$here/measure.sh" "$probe/closure" "$CR" --module "$probe"
"$here/measure.sh" "$probe/module" leanchecker "$imported"
[ "${SKIP_SLOW:-0}" = 1 ] || {
  "$here/measure.sh" "$probe/modules-all" timeout "${LIMIT:-1500}" leanchecker $mods
  "$here/measure.sh" "$probe/fresh" timeout "${LIMIT:-1500}" leanchecker --fresh "$imported"
}
