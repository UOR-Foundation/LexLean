#!/usr/bin/env bash
# The wait of `cut.yml` for the gate of a cut (SPEC.md §9.4): polls the runs
# of `ci.yml` on HEAD_SHA, the regenerated head `regenerate.yml` delivered and
# dispatched the gate on, until the newest pull request or dispatched run of
# it completes or WAIT_MINUTES pass. A run that completed `success` counts
# only when `.github/release-gate.sh` accepts it as the whole gate with every
# job passed, the evidence `release.yml` will read for the tag; so a cut lands
# on exactly what would release.
#
# Writes `state` to GITHUB_OUTPUT: `passed`, `failed` (the newest run failed,
# was cancelled, or passed without being the whole gate; exit 1), or
# `pending` (no verdict in time; exit 0, or exit 1 when FINAL is `true`, the
# last wait of the cut). A run re-run after a failure is the newest run again,
# so re-running the cut's failed jobs waits for it afresh.
#
# Inputs, from the environment: REPO, HEAD_SHA, WAIT_MINUTES, FINAL. `gh`
# reads the runs and jobs (`actions: read`); the checkout is HEAD_SHA.
set -euo pipefail

repo=${REPO:?REPO names owner/name}
head=${HEAD_SHA:?HEAD_SHA names the commit the gate runs on}
wait=${WAIT_MINUTES:-340}
final=${FINAL:-false}
out=${GITHUB_OUTPUT:-/dev/null}
summary=${GITHUB_STEP_SUMMARY:-/dev/null}
say() { printf '%s\n' "$1" | tee -a "$summary"; }

deadline=$((SECONDS + wait * 60))
while :; do
  state=$(gh api "repos/$repo/actions/workflows/ci.yml/runs?head_sha=$head&per_page=30" \
    --jq "[.workflow_runs[] | select(.head_repository.full_name == \"$repo\") | select(.event == \"pull_request\" or .event == \"workflow_dispatch\")][0] // empty | \"\(.status) \(.conclusion) \(.html_url)\"" \
    2>/dev/null || true)
  case "$state" in
    "completed success "*)
      if SHA=$head OUT="${RUNNER_TEMP:-/tmp}/gate-evidence.txt" .github/release-gate.sh; then
        say "The gate passed on \`$head\`: ${state##* }"
        echo "state=passed" >> "$out"
        exit 0
      fi
      say "The newest run of \`ci.yml\` on \`$head\` passed without being the whole gate: ${state##* }"
      echo "state=failed" >> "$out"
      exit 1
      ;;
    completed*)
      echo "::error::the gate of $head did not pass: $state"
      say "The gate of \`$head\` did not pass (${state##* }). Re-run its failed jobs, then re-run this run's failed jobs to land the cut; or fix the cut branch and merge its pull request by hand."
      echo "state=failed" >> "$out"
      exit 1
      ;;
  esac
  if [ "$SECONDS" -ge "$deadline" ]; then
    break
  fi
  sleep 60
done

echo "state=pending" >> "$out"
if [ "$final" = true ]; then
  echo "::error::the gate of $head did not finish in time: ${state:-no run yet}"
  say "The gate of \`$head\` gave no verdict in time (${state:-no run yet}). Re-run this run's failed jobs to wait again, or merge the pull request by hand once it passes."
  exit 1
fi
say "The gate of \`$head\` gave no verdict in $wait minutes (${state:-no run yet}); the next job waits on."
