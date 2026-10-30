# source me: the pinned toolchain first on PATH and LEAN_PATH over the prebuilt Mathlib
# and its packages (read only; this never builds or fetches anything).
MATHLIB=${MATHLIB:-$HOME/mathlib-pinned}
export PATH="$HOME/.elan/toolchains/leanprover--lean4---v4.32.1/bin:$PATH"
LP="$MATHLIB/.lake/build/lib/lean"
for p in "$MATHLIB"/.lake/packages/*/; do LP="$LP:${p}.lake/build/lib/lean"; done
export LEAN_PATH="$LP"
