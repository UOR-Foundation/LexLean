#!/usr/bin/env bash
# The SIMD probe of decision #109 (D4) on the pinned Rust, recorded in
# VERIFICATION.md ("The SIMD probe of #109 on the pinned Rust"). It builds
# `probe.rs` for each of the four targets #109 lists, each case with and
# without `-C target-feature=+<feature>`, and holds every outcome to the record:
#
#   inside   the target's integer SIMD intrinsics inside a `#[target_feature]`
#            function of a crate that forbids unsafe code;
#   outside  the same crate with `--cfg probe_outside`, which adds a function
#            without the attribute that calls `inside`.
#
# Each case is a full library build (`--emit=link`), so a case that compiles
# has passed code generation as well as the safety check. Run from the
# repository root, so that `rustup` selects the toolchain `rust-toolchain.toml`
# pins and the source path in a diagnostic is the same on every host. The
# targets other than the host's come from `rustup target add`.
set -euo pipefail

cd "$(dirname "$0")/../.."
source=.github/simd-probe/probe.rs
channel=$(sed -n 's/^channel = "\(.*\)"$/\1/p' rust-toolchain.toml)
version=$(rustc --version)
echo "$version"
case "$version" in
  "rustc $channel "*) ;;
  *) echo "::error::the probe ran on '$version', not the pinned $channel"; exit 1 ;;
esac

out=$(mktemp -d)
trap 'rm -rf "$out"' EXIT
failed=0
summary=""

# probe TARGET FEATURE CASE FLAG EXPECT [NEEDLE...]
# EXPECT is `compiles` or an error code; each NEEDLE must appear in the output.
probe() {
  local target=$1 feature=$2 case=$3 flag=$4 expect=$5
  shift 5
  local args=(--crate-type=lib --crate-name=simd_probe --edition=2021 --emit=link
    --diagnostic-width=100 --target "$target")
  if [ "$case" = outside ]; then args+=(--cfg probe_outside); fi
  if [ "$flag" = yes ]; then args+=(-C "target-feature=+$feature"); fi
  local log="$out/$target-$case-$flag.log" status=0
  echo
  echo "::group::$target $case, -C target-feature=+$feature: $flag"
  echo "rustc ${args[*]} -o $out/libsimd_probe.rlib $source"
  rustc "${args[@]}" -o "$out/libsimd_probe.rlib" "$source" >"$log" 2>&1 || status=$?
  cat "$log"
  echo "exit $status"
  echo "::endgroup::"
  local observed=compiles ok=yes
  if [ "$status" -ne 0 ]; then
    observed=$(grep -o -m1 '^error\[E[0-9]*\]' "$log" | sed 's/^error\[\(.*\)\]$/\1/' || true)
    observed=${observed:-"exit $status"}
  fi
  if [ "$observed" != "$expect" ]; then ok=no; fi
  local needle
  for needle in "$@"; do
    if ! grep -qF -- "$needle" "$log"; then
      ok=no
      echo "::error::$target $case (+$feature: $flag): the output lacks: $needle"
    fi
  done
  if [ "$ok" = no ]; then
    failed=1
    echo "::error::$target $case (+$feature: $flag): expected $expect, observed $observed"
  fi
  summary+="$target | $case | $flag | $observed | $ok"$'\n'
}

e0133='error[E0133]: call to function `inside` with `#[target_feature]` is unsafe'
for flag in no yes; do
  probe x86_64-unknown-linux-gnu avx2 inside "$flag" compiles
  probe x86_64-unknown-linux-gnu avx2 outside "$flag" E0133 "$e0133"
  probe aarch64-unknown-linux-gnu neon inside "$flag" compiles
  probe aarch64-unknown-linux-gnu neon outside "$flag" E0133 "$e0133"
  probe wasm32-unknown-unknown simd128 inside "$flag" compiles
  probe wasm32-unknown-unknown simd128 outside "$flag" compiles
  probe thumbv7em-none-eabihf dsp inside "$flag" E0658 \
    'error[E0658]: the target feature `dsp` is currently unstable' \
    'error[E0658]: use of unstable library feature `stdarch_arm_dsp`'
  probe thumbv7em-none-eabihf dsp outside "$flag" E0658 \
    'error[E0658]: the target feature `dsp` is currently unstable' \
    'error[E0658]: use of unstable library feature `stdarch_arm_dsp`'
done

echo
echo "target | case | -C target-feature | observed | as recorded"
printf '%s' "$summary" | sort
exit "$failed"
