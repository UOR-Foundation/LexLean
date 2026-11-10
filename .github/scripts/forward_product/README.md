# Temporary initial calculus producer for #405

This is preparation infrastructure, not a shipped capability or a release gate.
It has not been compiled with Rust or run with Lean in the preparation executor.
No workflow has been published or executed by this preparation.

## Admission and scope

The base is d1cc3c6d3dd11e30a1e4420fa81e7121f9640332. The only source
changes admitted above it are the three independently reviewed model products
from run 42748207774 (exact SHA256 values in admission.json) and the six files
of this infrastructure. The applied local model patch is
SHA256 de6896ef9484494c537092760af532937cbd8ceb76b4d6f4bf20f66fbd3cc953.
Its publication belongs to the earlier model owner; this does not authorize or
retry that operation. The execution head must independently be reviewed and
supplied exactly; a clean checkout and unchanged ancestry/source are required.

The 28 ordinary lock roots are the reviewed admission inventory. This includes
examples/nested-induction even though it has no committed lock. Source admission
prevents newly added roots being silently missed. No fixture, overlay, malformed
lock, request descriptor, scratch-owned project or lexical entry is relocked.
The full fixture admission helper is intentionally unnecessary for this narrower
phase: every fixture byte remains protected by whole-checkout inventories.

## Execution order and exact stop

1. Record actual Rust/Cargo 1.97.1 and Lean 4.32.1 versions and real tool binary
   hashes resolved through rustup/elan. Use the repository's install/retry route.
2. Copy the exact read-only observer seed into a temporary xtask example.
   Build the official xtask/lexlean binaries and observer with --locked.
3. Observe renderer_digest(RENDERER_FILES), compiler_semantics_id_for(1.2),
   every embedded input and errors table. Refuse a binary whose embedded bytes
   differ from disk. Replace exactly the existing rust_renderer scalar using
   this observation, rebuild, and require equality with the owning API.
4. Invoke official lexlean --project MANIFEST lock on each ordinary root.
   Only that project's lockfile may change during its invocation.
5. Run official check-calculus --write once, preserving the original exit and
   log even if it fails. Its output scope comes from calculus::check/files,
   gnaf::files, certification::files, lcnf_registry::generated/regenerate and
   calculus::shipped_modules. In particular authored TargetSyntax, Gnaf,
   Certification and Main sources are protected, not mislabeled generated.
6. Rebuild after generated embedded inputs change, observe the identity again,
   and relock ordinary roots with the rebuilt binary. Run a read-only
   check-calculus observation. A nonzero exit remains nonzero and is not repaired
   by looping the writer or weakening assertions.
7. Remove the temporary example in the finalizer, preserve snapshots, source
   patches, changed-file archives (including new files), command logs/exits and
   exact binary hashes, and STOP for independent output review.

A zero exit means this bounded initial phase ran successfully, not that a final
fixed point or conformance was established. check-calculus copies compiler golden
modules into shipped preservation data. This phase deliberately does not change
that golden and cannot establish closure of the later production-export/golden/
post-golden-calculus/rebuild cycle. A later reviewed producer must implement and
measure that bounded cycle, then handle source-format/lock-current examples and
fixture controls with the independently admitted helper. No fixture or example
writer runs here. #442 remains sole owner of the all-nine-case counts observer.

The seed remains byte-identical to the reviewed input. Its Rust compilation is
still a pending native check, not a claimed result. Its executable SHA256 is
recorded when invoked. The seed/infrastructure must be removed before final
candidate acceptance unless separately justified; the temporary Rust example is
removed at the ordinary finalizer when intact. A missing, modified or type-changed
overlay instead fails cleanup and is recorded without interrupting final evidence;
unexpected remaining bytes are retained for review, never accepted as products.
An abruptly cancelled job's partial checkout is never a candidate.

The workflow is read-only and branch/path scoped. Publishing it would trigger a
hosted run, so publication requires separate coordination and review. The old
model workflow paths are not touched. No full qualifying CI reruns are added.
Generated .lexlean scratch directories are owned by the official commands and
ignored by Git; they are not admitted source or returned products. Build outputs
are isolated outside the checkout, never restored from a cache.

## Preparation checks

Run python3 -B -m unittest discover -s .github/scripts
-p test_forward_product_writer.py. Tests plant protected-input changes, deletions,
stale/duplicate renderer pins and invalid observations, and check the admitted
lock scope. These are Python custody tests only, not Rust compilation, native
writer success, fixed-point evidence or final acceptance.
