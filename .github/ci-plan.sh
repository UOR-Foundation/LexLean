#!/usr/bin/env bash
# What a run of ci.yml is (SPEC.md §9.4), decided from the event and the
# repository alone, before any job that costs a runner starts.
#
#   pull request, draft       the `rest` leg only: a development signal
#   pull request, ready       the whole gate
#   workflow_dispatch         the whole gate
#   push to main              the whole gate, unless the push is a merge whose
#                             tree is the tree of its second parent and the
#                             whole gate passed on that parent as a pull
#                             request run: then nothing, and the run says
#                             which run verified the tree
#
# Why: a pull request is pushed to several times before its head is final, a
# merge of a head that already contains main lands that head's tree
# unchanged, and each of those runs started the whole gate (about 500
# runner-minutes on a pool of 20 shared jobs) for a tree whose verdict was
# known or was not going to be read.
#
# Inputs, all from the environment: EVENT, DRAFT, REPO. The jobs read `full`
# and `rows` from GITHUB_OUTPUT. `LEXLEAN_CI_ROWS` and `LEXLEAN_CI_WORKFLOW`
# name the legs file and the workflow when a test runs the script elsewhere.
set -euo pipefail

rows_file=${LEXLEAN_CI_ROWS:-.github/vv-rows.json}
workflow=${LEXLEAN_CI_WORKFLOW:-.github/workflows/ci.yml}
event=${EVENT:?EVENT names the workflow event}
draft=${DRAFT:-}
repo=${REPO:?REPO names owner/name}
out=${GITHUB_OUTPUT:-/dev/stdout}
summary=${GITHUB_STEP_SUMMARY:-/dev/null}

emit() {
  printf 'full=%s\nrows=%s\n' "$1" "$2" >> "$out"
}

whole_gate() {
  emit true "$(jq -c . "$rows_file")"
  printf '%s\n' "$1" >> "$summary"
}

# The id of a pull request run of this workflow on the second parent of HEAD
# that ran every job of the gate and passed every one, when HEAD is a merge of
# that parent with the same tree. A draft run has only the `rest` leg, so it is
# never one: the run must hold the join of Atlas, every host of the matrix, and
# every leg of the rows file, and no job that did not conclude `success`.
verified_parent_run() {
  local parent head_tree hosts runs id names part
  parent=$(git rev-parse --verify --quiet 'HEAD^2') || return 1
  head_tree=$(git rev-parse 'HEAD^{tree}')
  [ "$head_tree" = "$(git rev-parse "$parent^{tree}")" ] || return 1
  hosts=$(grep -c '^          - { os: ' "$workflow")
  runs=$(gh api "repos/$repo/actions/workflows/ci.yml/runs?head_sha=$parent&event=pull_request&status=success&per_page=30" \
    --jq ".workflow_runs[] | select(.head_repository.full_name == \"$repo\") | .id") || return 1
  for id in $runs; do
    names=$(gh api "repos/$repo/actions/runs/$id/jobs?per_page=100" --paginate \
      --jq '.jobs[] | "\(.name)\t\(.conclusion)"') || continue
    if printf '%s\n' "$names" | awk -F'\t' '$2 != "success" { bad = 1 } END { exit bad }' \
      && printf '%s\n' "$names" | grep -qxF $'atlas (join)\tsuccess' \
      && [ "$(printf '%s\n' "$names" | grep -c '^build and non-Lean tests (')" -eq "$hosts" ]; then
      for part in $(jq -r '.[].part' "$rows_file"); do
        printf '%s\n' "$names" | grep -qxF "just vv ($part)"$'\tsuccess' || continue 2
      done
      printf '%s\n' "$id"
      return 0
    fi
  done
  return 1
}

case "$event" in
  pull_request)
    if [ "$draft" = true ]; then
      emit false "$(jq -c '[.[] | select(.part == "rest")]' "$rows_file")"
      printf '%s\n' 'Draft pull request: the `rest` leg only. This run is not the gate; marking the pull request ready runs the whole gate.' >> "$summary"
    else
      whole_gate 'Pull request ready for review: the whole gate.'
    fi
    ;;
  push)
    if run=$(verified_parent_run); then
      emit false '[]'
      printf 'This push merges a head whose tree it equals, and the whole gate passed on that head: run %s of %s. Nothing is run again.\n' "$run" "$repo" >> "$summary"
    else
      whole_gate 'Push: the whole gate.'
    fi
    ;;
  *)
    whole_gate 'Run on request: the whole gate.'
    ;;
esac
