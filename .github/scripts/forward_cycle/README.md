# Temporary production, golden and calculus producer cycle

This implements one bounded hosted wave, not a release gate. Publication and
execution require separate review. The source anchor is admitted initial-product
commit 28fcce2e46538a05115dfa089aa4d764ecea4458, tree
e12baca5f6253a5f63341541f83655da7eed79ce. Only these four infrastructure files may
differ above that anchor. Prior product admission came from run 42754689172,
whose independently reviewed 189-file manifest has SHA256
f3e53cb08fae8be01761fcb06cea757c4da98251bfdf356a7045f077ffc24678.

## Actual dependency trace

- crates/lexlean/build.rs embeds every regular file from language/, schemas/,
  tests/golden/axiom-parser/ and tests/golden/canonical-json/. Its rerun rules
  include both directories and individual files. compiler_semantics_id_for(1.2)
  in lib.rs selects all embedded::FILES through embedded_digest(|_| true).
  identity_bytes uses surface::identity::data_form to normalize eligible
  semantic/tooling .lex.tex modules, and otherwise hashes their raw bytes.
  Thus all 905 currently observed files participate for 1.2, but their raw
  SHA256 inventory is custody evidence, not a replacement semantics hash.
- build.rs separately compiles the generated stdlib SHA-256 implementation via
  crates/lexlean/sha256_package.rs, linked to
  tests/production-export/stdlib/packages/rust-std/sha256/src/lib.rs. That source
  is NOT an embedded::FILES row. It can change in the production writer and must
  be rebuilt before the next command. model/errors.toml is also separately
  compiled; the entire error source remains frozen here.
- calculus::check calls the pinned Lean registry writer, regenerates its owned
  products, then shipped_modules copies exactly the four TARGET_MODULES in
  production::preserve from compiler/expected/build/modules/ into
  language/preservation-1.2/modules/. These .lean files use raw identity bytes.
  Their copy changes both the compiled embedded tree and potentially semantics.
- check-golden owns the runtime transcript, every discovered example plus
  compiler/stdlib/std-models/std-reasoning/kappa/bootstrap expected/build tree,
  and tooling::fixtures expected/build trees. Its published_build uses current
  locks (relock=false). It does not own expected/verify or negative fixture data.
- production_export::check owns packages/ only for its EXPORTED five projects.
  Roots descriptors and evidence inputs are protected. The command uses the
  actual production consumer, including its native verification and cleanup
  assertion. This is not a fixture request-reconciliation route.
- The potential identity edge through compiler golden modules is not a hash
  equation over a manifest containing itself. The four preserved modules are
  semantic modules, rendered from checked semantic declarations by
  backend::semantic::render_lean (via backend::lean::render_module). Their Lean
  text does not include compiler_semantics/source_id/build_id. Those identities
  belong to separate build metadata. Similarly calculus::rust::package::package
  prints the lowered Rust AST before placing compiler_semantics in the separate
  provenance.json; the compiled SHA-256 library source does not embed that field.
  This source trace supports measuring convergence; it does not prove that a
  future writer cannot expose another dependency or native failure.

## One bounded wave

Build and observe the exact initial source first. Each round then runs:

1. Official check-calculus --write, rebuild, owning identity observation, and
   official lock writer for the 28 previously admitted ordinary projects.
2. Official check-production-export --write, rebuild, observe and relock.
   This rebuild covers the generated SHA-256 implementation, not just semantics.
3. Official check-golden --write, rebuild, observe and relock.
4. Official check-calculus --write again to copy the new compiler golden into
   shipped preservation data, then rebuild, observe and relock.

There are at most three rounds. This is an execution budget, not an assertion
that three rounds suffice: one round updates downstream outputs, another can
propagate newly shipped Lean identities, and a no-change round measures closure.
A round must leave the ENTIRE tracked plus non-ignored new-file inventory
byte/mode/path equal to its start, and its owning identity observation must also
match the prior boundary. Equal semantics alone is insufficient. Nonconsecutive
repeated states fail as oscillation. Continued change after round three fails.
Any original command failure stops immediately and is preserved, never silently
retried or repaired. A later phase is not run merely to hide an earlier failure.

Once a round is unchanged, the rebuilt binary runs read-only check-calculus,
check-production-export and check-golden. These must all exit zero without any
source/product change. A final owning identity observation must be identical.
Only then is the bounded producer fixed point recorded. qualified remains false.
The final candidate still needs fixture/example products, corpus measurements,
actual accepted-parent ancestry, independent product review and qualification.

## Custody and boundaries

Each command has original argv/exit/log, source inventory before and after,
actual Rust/Cargo/Lean versions, resolved tool hashes and invoked binary SHA256.
Each writer and lock-group boundary additionally captures the cumulative patch
and full changed-file tar; new files live in the tar even when absent from Git's
tracked-file patch. Build snapshots include the generated SHA-256 source and all
other tracked inputs. An empty external output/build directory prevents reuse
of cross-run caches. Unexpected changed inputs stop the run.

The observer is the exact previously compiled seed copied temporarily into an
xtask example. It calls owning APIs and checks every embedded byte against disk.
It is removed on normal completion; altered or missing observer state fails but
does not suppress final evidence. The existing audited cleanup helper is reused.
No observer or new conformance assertion is added to shipped capabilities.

No fixture writer, example verification writer, count reporter, source formatter,
or malformed-control normalization runs here. No #442 or #316 code is imported.
Existing preparation workflows and their watched files are unchanged, preventing
a second initial/model producer from firing with this new infrastructure.
The new workflow uses the same pinned checkout/upload actions and repository
retry/toolchain route, contents:read only, no build cache and no Git writes. Its
six-hour wall cap bounds the complete native export cycle; no new host settings
or permissions are introduced.

Python tests use explicit fake native processes for orchestration/failure plants;
they are not claims that this new wave has run with Rust/Lean. Before publication,
run python3 -B -m unittest discover -s .github/scripts -p test_forward_cycle_writer.py
and independently review the exact source and trigger scope.
