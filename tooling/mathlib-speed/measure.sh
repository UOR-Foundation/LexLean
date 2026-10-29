#!/usr/bin/env bash
# measure.sh LABEL CMD...  -- run CMD under /usr/bin/time -v (through $RUNNER, default
# ~/bin/heavy, the memory-capped lane) and print one tab-separated row:
#   LABEL  wall_seconds  peak_rss_MiB  exit_code
# Peak RSS is the largest resident set of the process tree, as /usr/bin/time reports it.
# stdout/stderr of CMD go to $MEASURE_LOG (default: /dev/null).
set -u
label=$1; shift
runner=${RUNNER:-$HOME/bin/heavy}
tf=$(mktemp)
"$runner" /usr/bin/time -v -o "$tf" "$@" >"${MEASURE_LOG:-/dev/null}" 2>&1
rc=$?
wall=$(sed -n "s/.*Elapsed (wall clock) time (h:mm:ss or m:ss): //p" "$tf" | awk -F: "{ if (NF==3) print \$1*3600+\$2*60+\$3; else print \$1*60+\$2 }")
kb=$(sed -n "s/.*Maximum resident set size (kbytes): //p" "$tf")
rm -f "$tf"
printf "%s\t%s\t%s\t%s\n" "$label" "$wall" "$((kb / 1024))" "$rc"
