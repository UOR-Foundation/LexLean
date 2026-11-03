# Use generated Lean from Lake

A language-1.2 project's generated Lean can be built in a separate Lake
workspace together with the runtime generated from a matching compiler
checkout. This is useful for Lean tests and tactics over the generated
modules. It requires Rust, Git, jq, tar, the pinned `leanprover/lean4:v4.32.1`
toolchain through elan, and the source checkout; an installed LexLean binary
alone does not export the runtime's Lean source.

## Build the project and locate its modules

Run the project's installed compiler from the project directory:

```sh
lexlean lock
lexlean build --diagnostic-format json > build-result.json
jq -r '.artifacts[0]' build-result.json
```

Check that the command succeeded. The reported path is relative to the
project directory, normally `.lexlean/build/<build-id>`. Its `modules/`
directory holds the generated Lean, and `manifest.json` records the language
and compiler-semantics identity. Keep the absolute path to this build
as `<project-build>` below. Rebuilding after a source or configuration change
can produce a new path.

## Obtain the matching runtime

Use a clean LexLean checkout at the compiler's source revision (a release tag
is a useful starting point for a released binary). Build a compiler from that
checkout, and generate its runtime in a separate scratch directory:

```sh
# In the LexLean checkout; make the scratch directory outside the checkout.
cargo build --locked -p lexlean
mkdir -p /absolute/path/to/runtime-copy
git archive HEAD language/runtime-1.2/library | tar -x -C /absolute/path/to/runtime-copy
cd /absolute/path/to/runtime-copy/language/runtime-1.2/library
/absolute/path/to/LexLean/target/debug/lexlean lock
/absolute/path/to/LexLean/target/debug/lexlean build --diagnostic-format json > build-result.json
jq -r '.artifacts[0]' build-result.json
```

Use the absolute path to this reported build as `<runtime-build>`. The runtime
project generates `LexLeanRuntime` modules from the native LexLean sources
that the compiler embeds and renders during verification (§17.12).

Before combining the builds, compare their manifests:

```sh
jq '{language, semantics_id: .compiler.semantics_id}' "<project-build>/manifest.json"
jq '{language, semantics_id: .compiler.semantics_id}' "<runtime-build>/manifest.json"
```

Both must report language `1.2` and the same `semantics_id`. That identity
covers the runtime registry and sources and the backend versions (§21.2).
A matching package version alone is insufficient. `lexlean --version` reports
the language-1.0 identity, so use these language-1.2 manifests for this check.
If the identities differ, obtain the matching checkout or rebuild the project
with the checkout's compiler before proceeding. Retain the source revision
and both manifests with the consumer's build instructions.

## Create the Lake consumer

Create a separate directory containing `lean-toolchain` with this one line:

```text
leanprover/lean4:v4.32.1
```

For a project with generated module `Probe.Main` using `Portable` and
`Collections`, write this `lakefile.toml`. Replace the two build paths with
absolute paths and replace `Probe.Main` with the generated module you need:

```toml
name = "consumer"
defaultTargets = ["Runtime", "Probe"]

[[lean_lib]]
name = "Runtime"
roots = ["LexLeanRuntime.Portable", "LexLeanRuntime.Collections"]
srcDir = "<runtime-build>/modules"

[[lean_lib]]
name = "Probe"
roots = ["Probe.Main"]
srcDir = "<project-build>/modules"
```

Run `lake build` in the consumer directory. Add the runtime roots your modules
import, such as `LexLeanRuntime.Models`, `LexLeanRuntime.Reasoning`, or
`LexLeanRuntime.Laws`, to `Runtime.roots` when needed. Add other generated
project roots or libraries for their generated source-package imports as
needed. Keep authored Lean tests in a separate library in this consumer
workspace; they can import `Probe.Main` normally.

Lake compiles these sources into its own build directory. It does not reuse
`.lexlean/verified/<attestation-id>/oleans`, whose retained files serve
LexLean's replay records and are not a Lake library distribution. Lake
compilation settings need not match LexLean's; identical object bytes are not
promised. Lake success is consumer elaboration evidence; LexLean's
verified claim still requires `lexlean verify`, including kernel replay,
exact axiom audit and policy checks (§22). Neither this workflow nor a
consumer's added theorems modifies an existing LexLean attestation.
