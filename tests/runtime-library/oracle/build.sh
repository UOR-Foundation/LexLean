#!/bin/bash
# Compile the runtime sources lifted from `backend/semantic.rs` into `out/`, in dependency order.
export PATH=$HOME/.elan/bin:$PATH
cd "$(dirname "$0")"
LEAN=$HOME/.elan/toolchains/leanprover--lean4---v4.32.1/bin/lean
export LEAN_PATH=$PWD/out
rm -rf out; mkdir -p out/LexLeanRuntime
for m in Portable Collections Models Reasoning; do
  s=$(date +%s)
  $LEAN -R src -o out/LexLeanRuntime/$m.olean src/LexLeanRuntime/$m.lean > out/$m.log 2>&1
  echo "LexLeanRuntime.$m rc=$? seconds=$(( $(date +%s) - s )) output_bytes=$(wc -c < out/$m.log)" >> out/summary.txt
done
echo done >> out/summary.txt
