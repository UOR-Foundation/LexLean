#!/bin/bash
# Build the hand-written text laws to an olean the exporter reads. One-time tool.
export PATH=$HOME/.elan/bin:$PATH
cd "$(dirname "$0")"
LEAN=$(ls -d $HOME/.elan/toolchains/leanprover--lean4---v4.32.1)/bin/lean
export LEAN_PATH=$PWD/out
mkdir -p out
$LEAN -R src -o out/LexLeanText.olean src/LexLeanText.lean
