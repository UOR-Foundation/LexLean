#!/usr/bin/env bash
# A bounded retry for the steps of the workflows that fetch a pinned toolchain
# (#242):
#
#   bash .github/retry.sh <command> [<argument>...]
#
# runs the command at most three times, waiting 10 and then 20 seconds between
# attempts, and exits with the status of the third attempt when every attempt
# fails, after that attempt's own error. The command and its arguments are the
# same on every attempt, so what is fetched and what is verified (the pinned
# version, the installer's own checks) are the same: only the number of
# attempts changes. Every failing exit is retried. An HTTP 5xx from the release
# host and a resolver failure (curl's exit 6) are both transient on the
# runner's side and are why this exists; a wrong URL, which is not transient,
# still fails, after the third attempt and never later.
set -uo pipefail

if [ "$#" -eq 0 ]; then
  echo "retry: no command given" >&2
  exit 2
fi

attempts=3
status=0
for ((attempt = 1; attempt <= attempts; attempt++)); do
  if "$@"; then
    exit 0
  else
    status=$?
  fi
  echo "retry: attempt $attempt of $attempts failed with exit $status: $*" >&2
  if [ "$attempt" -lt "$attempts" ]; then
    sleep $((attempt * 10))
  fi
done
echo "::error::retry: $* failed after $attempts attempts (exit $status)" >&2
exit "$status"
