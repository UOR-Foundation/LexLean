#!/usr/bin/env bash
# The release's CI evidence (SPEC.md §30.3): the run of `ci.yml` that was the
# whole gate on the tree of the tagged commit and passed every job, written
# with its jobs and the transcripts of its legs as `vv-evidence.txt`.
#
# The release workflow runs no recipe of the gate itself. Run whole on one
# hosted runner, `just vv` exceeds the six-hour job limit: the `bdd` recipe
# alone took over four hours in a rehearsal, and Atlas follows it. The gate
# of the line (§9.4) has already verified the tree as the parts of one run,
# and this script finds that run under the rule `.github/ci-plan.sh` applies
# to a push: a run on the commit itself, or, when the commit is a merge whose
# tree is the tree of its second parent, a run on that parent. A run counts
# when every job of it concluded `success` and it holds the join of Atlas,
# every host of the matrix, and every leg of `.github/vv-rows.json`; a
# draft run, which has only the `rest` leg, never does. A tag on a commit no
# such run verified is refused, naming what was looked for.
#
# Inputs, all from the environment: REPO (owner/name), SHA (the tagged
# commit), OUT (the evidence file to write), and the lines of the header:
# REF and RELEASE_RUN. `LEXLEAN_CI_ROWS` and `LEXLEAN_CI_WORKFLOW` name the
# legs file and the workflow when a test runs the script elsewhere. `gh`
# reads the runs, the jobs, and the artifacts (`actions: read`).
set -euo pipefail

rows_file=${LEXLEAN_CI_ROWS:-.github/vv-rows.json}
workflow=${LEXLEAN_CI_WORKFLOW:-.github/workflows/ci.yml}
repo=${REPO:?REPO names owner/name}
sha=${SHA:?SHA names the tagged commit}
out=${OUT:?OUT names the evidence file}
ref=${REF:-}
release_run=${RELEASE_RUN:-}

hosts=$(grep -c '^          - { os: ' "$workflow")
parts=$(jq -r '.[].part' "$rows_file")

# The jobs of a run as `name<TAB>conclusion<TAB>started<TAB>completed`, or
# nothing when the run cannot be read.
jobs_of() {
  gh api "repos/$repo/actions/runs/$1/jobs?per_page=100" --paginate \
    --jq '.jobs[] | "\(.name)\t\(.conclusion)\t\(.started_at)\t\(.completed_at)"' 2>/dev/null || true
}

# Whether a jobs listing is the whole gate, passed.
whole_gate_passed() {
  local jobs=$1 part
  [ -n "$jobs" ] || return 1
  printf '%s\n' "$jobs" | awk -F'\t' '$2 != "success" { bad = 1 } END { exit bad }' || return 1
  printf '%s\n' "$jobs" | grep -qxP 'atlas \(join\)\tsuccess\t.*' || return 1
  [ "$(printf '%s\n' "$jobs" | grep -c '^build and non-Lean tests (')" -eq "$hosts" ] || return 1
  for part in $parts; do
    printf '%s\n' "$jobs" | grep -qP "^just vv \(\Q$part\E\)\tsuccess\t" || return 1
  done
}

# The commits whose runs are evidence for this one: the commit, and its second
# parent when the commit is a merge that lands that parent's tree unchanged.
candidates="$sha"
if parent=$(git rev-parse --verify --quiet "$sha^2") \
  && [ "$(git rev-parse "$sha^{tree}")" = "$(git rev-parse "$parent^{tree}")" ]; then
  candidates="$sha $parent"
fi

found=
for commit in $candidates; do
  runs=$(gh api "repos/$repo/actions/workflows/ci.yml/runs?head_sha=$commit&status=success&per_page=30" \
    --jq ".workflow_runs[] | select(.head_repository.full_name == \"$repo\") | \"\(.id)\t\(.event)\t\(.created_at)\t\(.updated_at)\t\(.html_url)\"" 2>/dev/null) || runs=
  while IFS=$'\t' read -r id event created updated url; do
    [ -n "$id" ] || continue
    jobs=$(jobs_of "$id")
    if whole_gate_passed "$jobs"; then
      found=$id
      break 2
    fi
  done <<< "$runs"
done

if [ -z "$found" ]; then
  echo "release-gate: no run of ci.yml was the whole gate and passed every job on $candidates of $repo" >&2
  echo "release-gate: the gate runs on every push to the line (SPEC.md §9.4); a tag is made from a commit it has verified" >&2
  exit 1
fi

{
  echo "repository $repo"
  echo "commit $sha"
  [ -z "$ref" ] || echo "ref $ref"
  [ -z "$release_run" ] || echo "run $release_run"
  echo
  echo "gate run $url"
  echo "gate run id $found, event $event, created $created, updated $updated"
  if [ "$commit" = "$sha" ]; then
    echo "gate run commit $commit (the tagged commit)"
  else
    echo "gate run commit $commit (the second parent of the tagged commit, whose tree $(git rev-parse "$sha^{tree}") the tagged commit carries unchanged)"
  fi
  echo
  echo "jobs (name, conclusion, started, completed):"
  printf '%s\n' "$jobs"
  echo
} > "$out"

# The legs' transcripts, while GitHub retains the run's artifacts.
parts_dir=$(mktemp -d)
if gh run download "$found" -R "$repo" -p 'acceptance-gate-evidence-*' -D "$parts_dir" >/dev/null 2>&1 \
  && [ -n "$(find "$parts_dir" -name vv.txt -print -quit)" ]; then
  find "$parts_dir" -name vv.txt | sort | while read -r transcript; do
    echo "--- $(basename "$(dirname "$transcript")")/vv.txt"
    cat "$transcript"
    echo
  done >> "$out"
else
  echo "the run's artifacts are no longer retained; the job records above are the evidence" >> "$out"
fi
rm -rf "$parts_dir"

# The line `just vv` prints when the whole gate passes, which the release
# criterion reads (§30.3, `release-check`): here it summarises the run, every
# leg, every Atlas shard and the join, and every host of which passed.
{
  echo "gate: every job of run $found passed on $commit"
  echo "vv: the acceptance gate passed"
} >> "$out"

echo "release-gate: run $found ($url) was the whole gate on $commit and passed every job"
