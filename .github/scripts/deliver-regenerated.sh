#!/usr/bin/env bash
# The delivery of `regenerate.yml` (SPEC.md §9.4): the patch its `finish` job
# returned, committed on the commit it was made from and pushed to the branch
# that commit heads, then the gate dispatched on that branch.
#
# The branch is the one a caller names (BRANCH, `cut.yml`), or else the head
# branch of the one open pull request of this repository whose head is
# COMMIT. With none, or several, nothing is pushed and the patch stays the
# run's artifact. The push is a fast-forward of COMMIT: a branch pushed again
# while the run worked rejects it, and the newer head is left alone. A push
# with the workflow token starts no `pull_request` run, so `ci.yml` is
# dispatched on the branch, where its plan treats the run as the pull
# request's (`.github/ci-plan.sh`). The `regenerate/` branch that started the
# run is deleted once its tree is delivered, if it still names COMMIT.
#
# Inputs, from the environment: REPO, COMMIT, BRANCH (may be empty), RUN_URL,
# REGENERATED (the downloaded `regenerated-<commit>` artifact), GITHUB_REF,
# GITHUB_STEP_SUMMARY. The checkout is COMMIT with the workflow token. Output,
# to GITHUB_OUTPUT: `head`, the commit the gate was dispatched on, when it was
# (`cut.yml` waits for that run and lands the cut on it); nothing otherwise.
set -euo pipefail

repo=${REPO:?REPO names owner/name}
commit=${COMMIT:?COMMIT names the regenerated commit}
branch=${BRANCH:-}
dir=${REGENERATED:?REGENERATED names the artifact directory}
summary=${GITHUB_STEP_SUMMARY:-/dev/null}
output=${GITHUB_OUTPUT:-/dev/null}
say() { printf '%s\n' "$1" | tee -a "$summary"; }

(cd "$dir" && sha256sum --check --quiet SHA256SUMS)
if [ "$(cat "$dir/base-commit.txt")" != "$commit" ]; then
  echo "::error::the artifact was made from $(cat "$dir/base-commit.txt"), not $commit"
  exit 1
fi

if [ -z "$branch" ]; then
  heads=$(gh api "repos/$repo/pulls?state=open&per_page=100" --paginate \
    --jq ".[] | select(.head.repo.full_name == \"$repo\" and .head.sha == \"$commit\") | .head.ref" | sort -u)
  count=$(printf '%s' "$heads" | grep -c . || true)
  if [ "$count" -ne 1 ]; then
    say "No single open pull request of $repo has head \`$commit\` ($count found): nothing is pushed; the patch is this run's artifact \`regenerated-$commit\`."
    exit 0
  fi
  branch=$heads
fi

remote=$(git ls-remote origin "refs/heads/$branch" | cut -f1)
if [ "$remote" != "$commit" ]; then
  echo "::warning::$branch moved from $commit to ${remote:-nothing}; nothing is pushed"
  say "\`$branch\` is at \`${remote:-nothing}\`, no longer \`$commit\`: nothing is pushed; the patch is this run's artifact \`regenerated-$commit\`."
  exit 0
fi

if [ -s "$dir/regenerated.patch" ]; then
  git apply --index "$dir/regenerated.patch"
fi
if git diff --cached --quiet; then
  say "The regenerated tree of \`$commit\` is the committed tree: nothing to deliver."
  # A caller's branch was pushed with the workflow token, which started no
  # run: the gate is dispatched for it all the same.
  if [ -n "${BRANCH:-}" ]; then
    gh workflow run ci.yml --repo "$repo" --ref "$branch"
    echo "head=$commit" >> "$output"
  fi
else
  paths=$(git diff --cached --name-only | wc -l)
  {
    printf 'Regenerate the written outputs of %s\n\n' "${commit:0:10}"
    printf 'The write recipes of SPEC.md §9.2, run by regenerate.yml (%s),\n' "$RUN_URL"
    printf 'rewrote %s paths.\n' "$paths"
    if [ -s "$dir/counts.txt" ]; then
      printf '\nThe counts writer reported:\n\n'
      sed 's/^/    /' "$dir/counts.txt"
    fi
  } > "$RUNNER_TEMP/message.txt"
  git -c user.name='github-actions[bot]' \
    -c user.email='41898282+github-actions[bot]@users.noreply.github.com' \
    commit --quiet --file "$RUNNER_TEMP/message.txt"
  # A plain push is a fast-forward or nothing.
  git push origin "HEAD:refs/heads/$branch"
  gh workflow run ci.yml --repo "$repo" --ref "$branch"
  echo "head=$(git rev-parse HEAD)" >> "$output"
  say "Delivered $paths regenerated paths to \`$branch\` as \`$(git rev-parse HEAD)\` and dispatched \`ci.yml\` there."
fi

case "${GITHUB_REF:-}" in
  refs/heads/regenerate/*)
    started=${GITHUB_REF#refs/heads/}
    if git push --force-with-lease="refs/heads/$started:$commit" origin ":refs/heads/$started"; then
      say "Deleted \`$started\`."
    fi
    ;;
esac
