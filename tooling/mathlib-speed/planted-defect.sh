#!/usr/bin/env bash
# planted-defect.sh -- shows what closure replay covers that `leanchecker MODULE` does not.
# Evil adds a false theorem with kernel checking switched off; UsesEvil imports it and uses it.
# Expected: leanchecker UsesEvil exits 0 (an import is trusted), leanchecker Evil exits nonzero,
# closure-replay exits nonzero for both. Exits nonzero if any of that does not hold.
set -u
here=$(cd "$(dirname "$0")" && pwd)
export PATH="$HOME/.elan/toolchains/leanprover--lean4---v4.32.1/bin:$PATH"
CR=$here/closure-replay/.lake/build/bin/closure-replay
work=$(mktemp -d)
cp "$here/probes/Evil.lean" "$here/probes/UsesEvil.lean" "$work/"
cd "$work" && export LEAN_PATH=$work
lean -R . -o Evil.olean Evil.lean && lean -R . -o UsesEvil.olean UsesEvil.lean || exit 2
ok=0
leanchecker UsesEvil >/dev/null 2>&1;   [ $? -eq 0 ] || { echo "UNEXPECTED: leanchecker UsesEvil failed"; ok=1; }
leanchecker Evil >/dev/null 2>&1;       [ $? -ne 0 ] || { echo "UNEXPECTED: leanchecker Evil passed"; ok=1; }
"$CR" --module UsesEvil >/dev/null 2>&1; [ $? -ne 0 ] || { echo "UNEXPECTED: closure-replay UsesEvil passed"; ok=1; }
"$CR" --module Evil >/dev/null 2>&1;     [ $? -ne 0 ] || { echo "UNEXPECTED: closure-replay Evil passed"; ok=1; }
[ $ok -eq 0 ] && echo "planted defect: leanchecker UsesEvil passes, closure-replay catches it"
rm -rf "$work"
exit $ok
