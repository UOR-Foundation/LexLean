# Bootstrap performance

What each certified component costs against the Rust it replaces, and what that means for the fixed point (issue 273). This is a record of measurements, not a target: it is not normative and carries no register row (issue 106 C8). Update it as components land.

## Result

The fixed point is practical on the measured components. The one that could break it, the lexer, is fixed (issue 373). The compiler's Rust is about 230 s of a full gate of about 36,800 s, so stage 1's cost is set by that 230 s times the certified ratio, and Lean and rustc do not change.

- **Budget:** one more gate's worth of compiler time, 36,800 s, which allows the certified compiler to be about 158 times slower than the Rust overall. This number is a proposal for issue 273 to accept or change.
- **Projection:** at the measured median ratio of 25, stage 1 adds about 1.6 hours to the gate's 10.2 hours of runner time.
- **The one component that could break it is linear now:** the lexer, `lex`, was quadratic, at 680 s on a 343 KB file against 3.8 ms for Rust, and a port of the generated-source audit with that shape would have cost 11 to 22 gates. Moving its four strict scans into the branches that read them (issue 373) took it to 29 ms on that file, with the same lexemes.

## Measurements

One call to each `rust-std` package against its stage-0 oracle on the same input, at the largest input measured. `Growth` is the fitted exponent of time against size, certified then oracle: 1 is linear and 2 is quadratic. `Heap` is peak heap in KiB, certified then oracle. `Load` is the 1-minute load average when the certified runs started, with the memory available beside it.

| Row | Root | Oracle | Replaces | Inputs | Largest | Certified ms | Rust ms | Ratio | Growth | Heap | Load |
| --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| `BS-01` | `is_closed_lean_name` | `ir::core::is_closed_lean_name` | own Rust | 7 | 31,657 B | 0.045 | 0.075 | 0.6 | 1.02 / 0.98 | 0 / 0 | 1.29, 10 GiB |
| `BS-01` | `parse_audit_output` | `verify::axiom::parse_audit_output` | own Rust | 7 | 368,000 B | 3,906 | 5.5 | 714 | **1.77** / 1.08 | 4,129 / 2,024 | 1.25, 10 GiB |
| `BS-02` | `lex` | `production::preserve::lex` | own Rust | 10 | 343,104 B | 28.8 | 3.2 | 9 | 1.01 / 0.97 | 13,329 / 7,665 | 1.82, 10 GiB |
| `BS-02` | `audit_tokens` | `production::preserve::audit_tokens` | own Rust | 10 | 343,104 B | 25.8 | 3.1 | 8 | 0.94 / 0.97 | 13,329 / 7,665 | 1.84, 10 GiB |
| `BS-03` | `scan_source` | `source::scan::scan` | own Rust | 9 | 262,116 B | 29.9 | 4.7 | 6 | 1.06 / 1.01 | 13,890 / 10,495 | 1.59, 9 GiB |
| `BS-03` | `reject_atoms` | `source::scan::reject_forbidden_atoms` | own Rust | 7 | 21,383 atoms | 12.2 | 0.058 | 208 | 0.98 / 0.99 | 0 / 0 | 1.78, 10 GiB |
| `BS-04` | `check_program` | `calculus::check::check` | own Rust | 12 | 6,775 nodes | 88.6 | 1.1 | 79 | 1.19 / 0.91 | 7 / 6 | 1.12, 10 GiB |
| `BS-04` | `check_arguments` | `calculus::check::check_arguments` | own Rust | 103 | 635 nodes | 0.086 | 0.0001 | 1,223 | none | 0 / 0 | 1.34, 10 GiB |
| `BS-05` | `validate_crate` | `calculus::rust::validate::validate` | own Rust | 12 | 8,036 exprs | 38.4 | 1.4 | 28 | 1.37 / 1.30 | 71 / 213 | 1.45, 10 GiB |
| `BS-05` | `correspond_crate` | `calculus::rust::validate::correspond` | own Rust | 12 | 8,036 exprs | 19.8 | 1.8 | 11 | 0.99 / 1.07 | 60 / 1,189 | 1.46, 10 GiB |
| `BS-06` | `print_crate` | `calculus::rust::ast::print` | own Rust | 6 | 335,801 B out | 77.9 | 3.0 | 26 | **1.83** / 1.38 | 1,401 / 466 | 1.21, 10 GiB |
| `BS-06` | `print_expr` | `calculus::rust::ast::print_expr` | own Rust | 510 | 56,729 B out | 6.3 | 0.27 | 24 | 0.93 / 0.88 | 238 / 221 | 1.14, 10 GiB |
| `BS-09` | `validate_runtime` | `calculus::rust::rt_validate::validate` | own Rust | 117 | 1,562 nodes | 1.1 | 0.025 | 45 | none | 10 / 5 | 1.09, 10 GiB |
| `SL-01` | `find` | `str::find` | std | 5 | 262,144 B | 11.2 | 0.22 | 51 | 1.00 / 1.01 | 0 / 0 | 1.60, 10 GiB |
| `SL-01` | `starts_with` | `<[u8]>::starts_with` | std | 5 | 262,144 B | 0.003 | 0.002 | 2 | 0.82 / 0.87 | 128 / 0 | 1.55, 10 GiB |
| `SL-01` | `ends_with` | `<[u8]>::ends_with` | std | 5 | 262,144 B | 0.003 | 0.001 | 2 | 0.83 / 0.79 | 128 / 0 | 1.55, 10 GiB |
| `SL-01` | `trim` | `<[u8]>::trim_ascii` | std | 5 | 262,144 B | 0.18 | 0.073 | 2 | 0.96 / 1.00 | 128 / 0 | 1.55, 10 GiB |
| `SL-01` | `split_on` | `<[u8]>::split` | std | 5 | 262,144 B | 5.9 | 0.82 | 7 | 1.02 / 1.02 | 1,408 / 1,008 | 1.55, 10 GiB |
| `SL-01` | `lines` | `str::lines` | std | 5 | 262,144 B | 4.7 | 0.39 | 12 | 0.94 / 1.04 | 717 / 441 | 1.50, 10 GiB |
| `SL-01` | `join_with` | `<[Vec<u8>]>::join` | std | 5 | 262,144 B | 256 | 0.042 | 6,152 | **1.93** / 0.99 | 864 / 287 | 1.41, 10 GiB |
| `SL-02` | `sha256` | `sha2::Sha256` | `sha2` | 6 | 4 MiB | 30.0 | 2.0 | 15 | 0.99 / 0.89 | 0 / 0 | 1.60, 10 GiB |
| `SL-02` | `sha256_hex` | `sha2::Sha256` and hex | `sha2` | 6 | 4 MiB | 37.3 | 2.0 | 19 | 1.09 / 0.98 | 0 / 0 | 1.60, 10 GiB |
| `SL-05` | `write_all` | `Vec::extend_from_slice` | std | 5 | 262,144 B | 1.2 | 0.014 | 82 | 1.09 / 0.82 | 1,096 / 256 | 1.38, 10 GiB |
| `SL-05` | `write_each` | `Vec::push` | std | 5 | 262,144 B | 63.4 | 0.13 | 489 | 1.02 / 0.96 | 1,096 / 256 | 1.35, 10 GiB |
| `SL-05` | `write_numbers` | `write!` into a `Vec` | std | 4 | 334,175 B | 85.4 | 0.49 | 175 | 1.12 / 1.01 | 1,401 / 352 | 1.32, 10 GiB |

"Own Rust" means the root replaces code in `crates/lexlean`, not a crate. "None" under growth means the inputs sit in too narrow a band of sizes to fit one. Bold marks growth above 1.5.

The ratio grows with size for a superlinear root, so its ratio holds only at the size shown. The lexer before issue 373 shows it: its ratio was 180,745 at 343,104 B, and 64,757 B took 20 s, which confirmed the report of about 40 s on 87 KB.

## Superlinear components

**`lex` (and `audit_tokens`, which calls it), fixed by issue 373.** Cause: strict `let` bindings. The steps in `src/TokenAudit.lex.tex` bound every candidate scan before they branched on the byte, so the scans ran whether or not they were used. It was not list accumulation: lexemes are consed and reversed once, which is linear. Two kinds of scan repeated:

- **To the end of the text.** `stepLexAt` scanned for a block comment at every position, whitespace included. `stepLexToken` scanned for a string and `stepLexIdent` for a raw string, and each ran until its closing token.
- **To the end of the line.** `stepLexAt` scanned for a line comment at every position, and `scanSegmentsStep` looked for a closing guillemet on every segment of a dotted name. The test module has a line of 262,408 bytes.

Fix: each scan moved into the branch that reads it, in the source, where the proofs and BS-02 pass. It needed neither `Buffer` nor the loops of issue 105. Before the change, a copy of the generated Rust with those scans guarded by their first byte tested the diagnosis: it returned the same lexemes on all of `TargetSemantics.lean` (42,236 of them), and guarding only the first kind of scan left 105 s on the full file, so both kinds were needed. Through the harness the committed package now takes 28.8 ms at 343,104 B against 680,088 ms before, and grows at 1.01 against 2.04.

**`parse_audit_output`.** Cause: `insertEntry` in `src/AxiomOutput.lex.tex` inserts each record into a sorted list, walking and rebuilding it, where the oracle uses a map. Fix: collect the records, sort once, and find duplicates in the sorted pass. The last doubling of the measured ladder grows at 2.13, so the fit's 1.77 understates it.

**`join_with`.** Cause: `joinWith` in `stdlib/src/Text.lex.tex` appends each piece to the whole result so far, which copies it every time (the shape issue 92 reported). Fix: write through `Buffer`. Today's callers join short lists of names, so this is a trap for later ports rather than a present cost.

**`print_crate`.** Cause not isolated, and it may not be superlinear. Its source writes through `Buffer`, whose `finish` joins pairwise (n log n), and its inputs are different crates rather than prefixes of one, so the oracle also fits 1.38 on them. Over the top three inputs the certified fit is about 1.4 against 1.1. It costs 78 ms at the largest committed crate.

Later ports should avoid both shapes: a strict `let` around a scan that can reach the end of the input, and a repeated `prim append` onto a growing result. `write_each` shows a third cost: `Buffer` is 489 times `Vec::push` byte by byte and 82 times `extend_from_slice` by pieces, so write pieces.

## Stage 1 projection

Stage 1 recompiles every committed project once with the certified compiler.

**The gate.** A full gate (run 40387747440, `e0b74b81e`, Linux runners) is 36,775 s of runner time: 27,232 s of `just vv` shards and 9,543 s of Atlas shards. Its longest shards are `bdd-preservation` (3,996 s), `bdd-bootstrap-check` (3,755 s), and `examples` (3,381 s).

**The Rust compiler's share.** Stage-0 `lexlean build --all` before Lean:

| Project | Source | `check` | `build` |
| --- | ---: | ---: | ---: |
| `examples/production` | 3.8 KB | 0.6 s | 0.5 s |
| `bootstrap` | 557 KB | 1.1 s | 1.4 s |
| `examples/uor-atlas` | 24.8 MB | 11.1 s | 10.4 s |

These fit about 0.5 s per project plus 0.42 s per MB. The corpus is 348 projects and 141 MB of `.lex.tex`, which gives about 230 s, 0.6% of the gate. `language/preservation-1.2/library` (32 MB) failed at its `lexlean.lock` in the measuring copy, before any compile, so it was not timed; the fit stands in for it.

**Certified time, one pass over the corpus, as committed:**

| Component | Basis | Time |
| --- | --- | ---: |
| `audit_tokens`, `lex` | 253 verify modules, 1.3 MB, largest 85 KB, if every one is held as text, at 84 ns per byte | about 0.1 s |
| `reject_atoms` | 0.57 µs per atom, at most 2,000,000 atoms per project | at most 1.1 s per project |
| `check`, `validate`, `correspond`, `print` | 256 packages, each at the largest cost measured or next doubling | under 75 s |
| `scan_source` | 114 ns per byte over 141 MB | 16 s |
| `sha256_hex` | 8.9 ns per byte over 184 MB, per pass | 1.6 s |
| `parse_audit_output` | largest project about 1,800 declarations, under 200 KB of output | about 1 s per run |

Before issue 373 the lexer dominated what is ported, at up to 336 s. Only certificates held as text reach it, so about 0.1 s is a ceiling now.

**The phases still in Rust.** Most of the compiler is unmeasured. Applying the measured linear ratios to the 230 s:

| Ratio | Basis | Added to the gate |
| ---: | --- | ---: |
| 6 | `scan_source`, best walker | 1,400 s |
| 25 | median of the linear roots over text and trees | 5,800 s (1.6 h) |
| 80 | `check_program`, worst walker | 18,400 s (5.1 h) |
| 158 | the budget | 36,800 s (10.2 h) |

The breaker was growth, not the constant. `verify/source_audit.rs` reads all 43 MB of generated Lean with a separate lexer, still in Rust. At the quadratic lexer's rate on the test module, a port of it would have cost about 810,000 s, and 8.6 MB `Scales.lean` alone about 425,000 s (118 hours); about half of that rate came from the module's long lines, so the range was 400,000 to 810,000 s, 11 to 22 gates. At the fixed lexer's 84 ns per byte the same 43 MB takes about 4 s, if the port keeps each scan in its branch.

## Stage 2 projection

Stage 2 recompiles the compiler. `crates/lexlean/src` is 5.8 MB of Rust. The one direct pair, `verify/axiom.rs` (8,830 B) and `AxiomOutput.lex.tex` (27,557 B), suggests a factor of about 3, so about 18 MB of source. That estimate comes from a single pair and is the weakest number here.

- **Compile:** about 8 s in Rust, so 200 s at ratio 25 and 640 s at ratio 80.
- **Lean, if stage 2 also verifies:** dominates and no certified ratio touches it. At Atlas's rate (0.38 ms per byte) 18 MB is about 1.9 hours. At the rate of the last recorded bootstrap verify (`VERIFICATION.md`, 1,040 s, at least 1.9 ms per byte) it is at least 9.3 hours.
- **rustc on the generated Rust:** not measured separately. The 197 KB `check_program` library and its harness built and ran in under 40 s.

## Faster first

1. `parse_audit_output`: harmless at today's declaration counts, about 1 s per run. It reaches minutes only near 3 MB of output.
2. `join_with`: no present cost, but it should be fixed before a port joins long lists.

`lex` led this list until issue 373 made it linear. Constant factors come after these. Nothing else measured is near the budget.

## Crates

The self-hosted compiler uses no outside crates. `sha2` is the only one with a certified replacement, at 15 to 19 times its time at 4 MiB (issue 172 reported 24 to 35). Still to be replaced and unmeasured: `camino`, `clap`, `fs4`, `rustix`, `same-file`, `semver`, `serde`, `serde_json`, `signal-hook`, `tempfile`, `toml`, `unicode-normalization`, `walkdir`. Of these, `serde_json` and `toml` carry real work; their share of the 0.5 s per project was not separated.

## Method and limits

The harness is `crates/conformance/src/cases/bootstrap/perf.rs` with `crates/conformance/src/bin/bootstrap_perf.rs`. It renders a harness for each committed package, times `run` on the conformance corpus and on ladders of input sizes, times the oracle on the same inputs in its own process, and panics if the two ever disagree.

```bash
LEXLEAN_PERF_DIR=/path/on/a/fast/disk cargo run --release -p repo-conformance --bin bootstrap_perf -- lex
```

`LEXLEAN_PERF_MAX` raises the largest ladder input (400,000 B for `lex` and `audit_tokens` here), and `LEXLEAN_PERF_DROP` leaves out that many of a root's largest inputs.

The limits on these numbers:

- **Machine:** WSL2 Ubuntu on Windows 11, x86_64, 12 threads, rustc 1.97.1, CARGO_BUILD_JOBS=4, runs on 2026-11-01 at `ccbf03843`, and the `BS-02` rows on 2026-11-04 at `d34c1feb3`. CI runners differ, so the gate's seconds and these milliseconds are not on one clock; the projection's ratios assume they scale together.
- **Noise:** the VM is shared with other jobs, not all of which take the build lock, so some timings may carry their load. The load and available memory beside each row are what was recorded; 1.1 to 1.9 throughout. Treat ratios within a factor of two as equal.
- **Memory:** the lexer copy ran with memory tight, about 6 GB available because `/tmp` is memory-backed and was full; the rows had 9 to 10 GiB.
- **Peak heap excludes the stack.** Recursive roots run on a stack of 4 GiB, and it is not counted.
- **Thinned inputs:** `check_program`, `validate_crate`, `correspond_crate`, and `print_crate` time the largest input per doubling, because rendering every crate as a literal does not compile in 9 GiB. For the same reason `validate_crate` and `correspond_crate` leave out their largest input and `reject_atoms` its two largest. The projection allows for one more doubling.
- **Build profiles:** packages build at opt-level 3 and their harness crates at opt-level 0. The oracle runs in one process, with no process start in its time.
- **Builds differ:** the lexer before issue 373 took 205 s built into one binary with no counting allocator and 680 s through the harness, 3.3 times more, on the same input. The cause is not isolated. The harness is the form production links, a library crate, so the table keeps it, but compare times only within one build.
- **Repeats:** a call repeats until it has run 25 times or for 200 ms, and the median is reported, so slow ones run once. The 680 s figure from before issue 373 was one run; the fixed lexer ran 7 times at its largest input.
- **Refusals:** `audit_tokens` lexes the whole module and then refuses its largest input at a `set_option` that certificates may not hold, so that row times lexing plus the walk up to there. `validate_runtime`'s largest input is refused, and its inputs are one program with small changes, so it has no growth. `check_arguments` measures small inputs in a narrow band.
- **One pass:** stage 1 here is one compile of each project. If it instead reruns the whole gate on the certified compiler, each in-process compile in the suites counts as well and the projection is low. That count is not measured.
