#!/usr/bin/env bash
# The fetcher of the run's bootstrap export (SPEC.md §9.4), started beside a
# leg's gate command in a run that is the whole gate. It waits until a case
# of the leg asks for the export (the harness writes `<dir>/wanted`), then
# waits for the artifact `bootstrap-export` of THIS run, unpacks it into
# `<dir>/artifact`, and writes `<dir>/ready`; the harness checks the custody
# record against its own run and checkout before any case reads the bundle.
# When the `bootstrap export` job ends without the artifact it writes the
# reason to `<dir>/failed`, and the case fails with it. It reads the run's
# artifacts and jobs through `gh` with the job's token (`actions: read`),
# and nothing of another run.
set -uo pipefail
dir=${1:?the directory the harness waits in}
repo=${GITHUB_REPOSITORY:?}
run=${GITHUB_RUN_ID:?}
mkdir -p "$dir"
fail() {
  printf '%s\n' "$1" > "$dir/failed.tmp" && mv "$dir/failed.tmp" "$dir/failed"
  echo "take-bootstrap-export: $1" >&2
  exit 1
}
# No API call until a case needs the export: most legs never ask.
until [ -e "$dir/wanted" ]; do sleep 5; done
echo "take-bootstrap-export: wanted at $(date -u +%H:%M:%SZ)"
polls=0
while :; do
  id=$(gh api "repos/$repo/actions/runs/$run/artifacts?name=bootstrap-export" \
    --jq '[.artifacts[] | select(.expired | not)] | max_by(.created_at) | .id // empty' 2>/dev/null) || id=
  if [ -n "$id" ]; then
    rm -rf "$dir/artifact" "$dir/artifact.zip"
    # An artifact is listed a few seconds before its upload is finalized, and
    # a download in that window fails: retry with backoff before giving up.
    for wait in 5 10 20 40 0; do
      gh api "repos/$repo/actions/artifacts/$id/zip" > "$dir/artifact.zip" && break
      [ "$wait" -gt 0 ] || fail "the artifact $id of run $run did not download"
      echo "take-bootstrap-export: artifact $id did not download; again in ${wait}s" >&2
      sleep "$wait"
    done
    mkdir -p "$dir/artifact" && unzip -q "$dir/artifact.zip" -d "$dir/artifact" \
      || fail "the artifact $id of run $run did not unpack"
    rm -f "$dir/artifact.zip"
    echo "take-bootstrap-export: artifact $id of run $run at $(date -u +%H:%M:%SZ)"
    touch "$dir/ready"
    exit 0
  fi
  # Every fourth poll, whether the export job ended without its artifact.
  if [ $((polls % 4)) -eq 0 ]; then
    state=$(gh api "repos/$repo/actions/runs/$run/jobs?filter=latest&per_page=100" --paginate \
      --jq '.jobs[] | select(.name == "bootstrap export") | "\(.status) \(.conclusion)"' 2>/dev/null) || state=
    case "$state" in
      "completed success") ;; # uploaded: the next listing shows it
      completed*) fail "the bootstrap export job of run $run ended: $state" ;;
    esac
  fi
  polls=$((polls + 1))
  sleep 30
done
