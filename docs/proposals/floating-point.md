# Floating point: design

Draft for owner acceptance, written 2026-11-12, against `release/v0.4` at `aaf845507`. Normative text is `SPEC.md`; this document explains and asks. It is a proposal only: this pull request adds this one file and changes no language data, schema, register row, compiler code, example, or generated document. Every experiment behind it was run with scratch files outside the repository, on the pinned Lean 4.32.1; the commands and what they printed are in section 9, and the model that was measured is reproduced in appendix A so the numbers can be re-run.

This is gap C of three on the way to a verified machine-learning inference engine written as a LexLean application. Gap A (typed arrays with in-place update) and gap B (contracts for external code) are separate pull requests in progress. Where this document relies on them it says so, and it describes them only as far as they were described to this work.

## 1. Summary

The question is what LexLean can honestly say about floating-point arithmetic. The short answer:

1. **A float type in the language is out of reach for now.** Lean's `Float` and `Float32` are opaque to the kernel: every operation is a constant with no definition. The kernel cannot prove `1.0 + 1.0 = 2.0` on them (section 9.3). The only tactic that can, `native_decide`, adds one new axiom per use (section 9.4), and LexLean forbids it (`PF-17`, `LN-04`). A type lowered to `Float32` would be a type about whose values no theorem can be stated.
2. **Exact bit-level float semantics are in reach without any language change.** IEEE 754 binary32 arithmetic can be written as ordinary integer code: a *software model*, a function from two 32-bit patterns to one. A 98-line model of addition and multiplication, using only natural-number and integer arithmetic, was written for this proposal. The pinned kernel decides concrete instances of it by `decide` with no axioms (1.0 + 1.0, 0.1 + 0.2, subnormals, overflow, NaN, signed zero; section 9.5), and it agrees with the host's native `Float32` on all 3,001,764 operand pairs tried, for both operations (section 9.7).
3. **It is slow where it matters.** The kernel needs about 8 to 12 ms per float operation of this model; an integer multiply-add costs it about 0.3 ms. Compiled, the model runs about 70 times slower than Lean's own scalar `Float32` loop (section 7).
4. **Native floats can be connected without a hole, through gap B, not through the kernel.** The software model is the proved reference. A native float routine is an external function whose contract says "returns the same bits as the reference", whose evidence is an attested differential run (both run on the same inputs, outputs compared), and which appears in every dependent theorem as one named trust entry. Nothing else is assumed (section 6).
5. **Error bounds for whole networks are not proposed.** One universal rounding lemma was proved in about 15 lines to see what the work looks like (section 9.9). Bounds for a whole network need rational arithmetic and nonlinear reasoning the language does not have.

**Recommendation.** Build the inference engine on integers only, as the proved reference: the language already has the integer feed-forward network, integer tensors, and the exact derivation of integer scale multipliers from binary32 parameters (`MD-04`, `MD-13`). Track a binary32 software model for the standard library as a separate, optional item, to be started after the integer engine. Do not add a float type. Do not start error-bound proofs. The evidence gathered here did not contradict this expected answer; it made two parts of it sharper: the software model is cheaper to start than assumed, and too slow in the kernel to carry whole-network test vectors.

The decisions that need the owner, with a recommendation on each (reasons in section 12):

| # | Decision | Recommendation |
| --- | --- | --- |
| D1 | A floating-point type in the language | No, not in this release line or the next. Revisit only if the pinned Lean gives float operations a kernel-reducible definition |
| D2 | Which format a software model covers first | binary32 only. binary64 is the same construction and can follow |
| D3 | NaN results | One canonical quiet NaN, `0x7FC00000`, for every NaN result. Payloads are accepted on input and never propagated |
| D4 | Signed zero and equality | Keep signed zero exactly as IEEE 754 defines it. Equality in theorems is equality of bit patterns; IEEE comparison is a separate function |
| D5 | Operations of a first model | Add, subtract, multiply, compare, integer conversions. Divide, square root and fused multiply-add second. No `exp`, `tanh`, or other library functions, ever, in the exact model |
| D6 | Where the model lives | A standard-library module under the rules of SPEC §17.22, with a differential against a native float as its oracle |
| D7 | Representation | Bit patterns as `UInt32` at the interface, unbounded `Nat`/`Int` inside. No new type |
| D8 | New primitives to make the model faster in the kernel (shift and bit length on unbounded naturals) | No. They halve kernel time but move the language identity; the fuel loops the language already has are enough |
| D9 | How a native float routine is bound under gap B | At the level of a whole routine (dot product, matrix product) with its evaluation order written into the reference, not per operation |
| D10 | Error-bound proofs | Not proposed. Revisit with a concrete network and a concrete bound someone needs |
| D11 | Tracking | One issue for the optional binary32 module. No register row until work starts (R3, R4) |

## 2. What the goal asks and where it stands

A neural-network inference engine uses floating point in four places. They are different problems.

| Where floats appear | What is needed | State |
| --- | --- | --- |
| Trained weights | Nothing at inference time if the network is quantized (weights stored as small integers with a scale). The integer tensors are artifacts (`int_tensor`, SPEC §17.12 rule 2) | Supported today. Training and quantization happen outside LexLean and are not claimed |
| Scale factors of a quantized network | Turning three binary32 scales into one integer multiplier, exactly | Supported today: `MD-13`, rule 15, `round(a * b / c * 2^32)` by exact integer arithmetic |
| The arithmetic of the layers | Integer multiply-add, clamp, shift | Supported today on lists: `integer_feedforward` with `dense`, `relu`, `requantize` (rule 4, `MD-04`). Gap A makes it run in place on arrays. Gap B lets a native integer routine stand in for it |
| The arithmetic of the layers, for a network that is not quantized | Float multiply-add with a stated meaning | Not supported. This document |

So floating point is needed only for the last row. An engine that takes quantized networks never executes a float operation. That is the main reason for the recommendation.

## 3. Constraints taken as given

- SPEC §4.2: no raw Lean, no author axioms, no foreign code in the language. Generated Lean contains no `opaque`, `unsafe`, or `native_decide` (`LN-04`); `native_decide` is never accepted (`PF-17`, §16.12).
- SPEC §5.3: "verified" means Lean elaboration, `leanchecker` replay, the exact axiom audit, and policy success. Nothing in this document is verified in that sense. The scratch theorems were checked by the pinned Lean elaborator and kernel; one module was also replayed by `leanchecker` (section 9.6). I say "kernel-checked in a scratch file" for those.
- SPEC §17.12 rule 2 and rule 15: there is no floating-point artifact element and no floating-point type; binary32 and binary64 bit patterns enter integer derivations by decoding to exact dyadic rationals (an integer times a power of two), and non-finite patterns are rejected. The canonical IR and canonical JSON carry no floating-point values (§17.9, §21.7). The claim about rendered Rust programs covers no floating point (§17.14).
- SPEC §17.22: a standard-library member is validated by a differential against an oracle before it is a member.
- AGENTS.md rule 4: a claim about a dependency belongs to that dependency. Statements below about Lean are Lean's; statements about IEEE 754, WebAssembly, or a C compiler are theirs. Section 13 lists which is which and what I did and did not re-read.
- AGENTS.md R3 and R4: a capability starts with a register row and is finished in the change that starts it. A proposal document is not a capability; `docs/` is outside `audit-deferral`.
- The downstream SDK (Prism) runs LexLean applications by compiling the generated Lean to C with Lake. Taken from the task statement, not checked in this repository. It means anything LexLean lowers to must be Lean that the kernel can reason about and the compiler can run.
- The Lean pin is `leanprover/lean4:v4.32.1`. Pull request #433 proposes moving `release/v0.5` to v4.33.1. Every Lean fact in section 9 is a fact about 4.32.1 and must be re-run when the pin moves.

## 4. Semantics

This section says what a float operation would mean, wherever it is implemented. It is the specification a software model implements and a native routine is tested against.

### 4.1 Format, rounding, and order

- **Format.** IEEE 754-2019 binary32: 1 sign bit, 8 exponent bits with bias 127, 23 fraction bits. This is the layout rule 15 already fixes. A value is a 32-bit pattern; there is no separate float type (D7).
- **Rounding.** Every operation returns the exact mathematical result rounded once, to the nearest representable value, ties to the one with an even last bit (IEEE 754-2019 §4.3.1, `roundTiesToEven`). No other rounding mode exists. There are no exception flags and no traps: a flag is state that no LexLean value can observe.
- **Evaluation order.** The order written is the order evaluated. `add (add a b) c` and `add a (add b c)` are different terms with different values, and nothing may rewrite one into the other. A model is a library of functions, so the order is the structure of the term; there is no expression syntax for a compiler to reassociate.
- **No fused multiply-add unless written.** A fused multiply-add computes `a * b + c` with one rounding instead of two and gives different bits. `add (mul a b) c` always rounds twice. If a fused operation is wanted it is its own function, called by name.
- **Subnormals** (the values below the smallest normal number, which have fewer significant bits) are kept, with gradual underflow. There is no flush-to-zero mode.

The scratch model demonstrates that order matters, by two kernel-checked theorems: `(1e8 + -1e8) + 1 = 1` and `1e8 + (-1e8 + 1) = 0` in binary32 (section 9.5, `assoc_left` and `assoc_right`).

### 4.2 NaN

A NaN ("not a number") is any pattern with exponent 255 and a nonzero fraction; the fraction bits are its *payload*. IEEE 754-2019 §6.2 says what operations produce a NaN and recommends, but does not require, how payloads propagate. Real hardware differs here more than anywhere else (section 4.4).

Recommendation (D3): every operation that returns a NaN returns the one pattern `0x7FC00000` (sign 0, quiet bit set, rest zero). Any NaN is accepted as an operand. Payload and sign of a NaN operand never reach the result.

Reasons:

- It makes every operation a function of its operands with one answer, which is what a theorem needs.
- It is what can be tested. Lean's own `Float32.toBits` returns `0x7FC00000` for every NaN: `(0.0 / 0.0).toBits` and `(Float32.ofBits 0x7FA00001).toBits` both print `2143289344` (section 9.2). A differential through `toBits` cannot see a payload, so a model that specified payloads could not be checked against a native float through Lean.
- It equals the rule of WebAssembly's deterministic profile, which says "only positive canonical NaN outputs are produced" (WebAssembly core specification, numerics, fetched 2026-11-12).

Cost: sign-bit operations that IEEE 754 defines as pure bit manipulation (`negate`, `abs`, `copySign`) would, in the model, also return the canonical NaN for a NaN operand. That deviates from IEEE 754 for those operations on NaN operands only, and must be stated wherever the model is described.

The alternative is what `MD-13` does today: reject non-finite patterns, so every operation returns an optional value. It is simpler to reason about and it is right for parameters. It is wrong for arithmetic: overflow to infinity is ordinary behaviour of a float program and a model that refuses it is not a model of the hardware.

### 4.3 Signed zero and equality

IEEE 754 has two zeros. The model keeps both and follows IEEE 754-2019 §6.3: a product has the exclusive-or of the signs; a sum of two zeros of the same sign has that sign; an exact sum of zero from operands of opposite sign, or from `x + (-x)`, is `+0` under round-to-nearest. Four kernel-checked cases in section 9.5 cover these.

There are then two equalities, and they must not be confused (D4):

- *Bit equality*, the `equal` primitive on the pattern. `+0` and `-0` differ. This is what theorems state.
- *IEEE comparison*, a function in the library. `+0` equals `-0`, and NaN equals nothing, itself included.

### 4.4 Where real platforms differ

The model's semantics is one function. Native execution matches it only when all of the following are controlled. This list is what a conformance report under gap B has to pin (section 6).

| Source of difference | What happens | Authority and status |
| --- | --- | --- |
| NaN bit patterns | WebAssembly's `f32` arithmetic is deterministic except for NaN results: the sign is non-deterministic and, unless all NaN inputs are canonical, the payload is chosen among "arithmetic NaNs". x86, ARM and RISC-V each produce and propagate NaN payloads differently | WebAssembly core specification, fetched 2026-11-12. The hardware statement is from memory, not re-checked |
| x87 extended precision | 32-bit x86 code using the x87 unit keeps intermediates in 80-bit registers; a result is rounded twice and can differ in the last bit | From memory, not re-checked. Not an issue on x86-64, where compilers use SSE |
| Contraction to fused multiply-add | A C compiler may fuse `a * b + c`. Clang's default is `-ffp-contract=on`, fusion within one statement | Clang user manual, fetched 2026-11-12. Observed: Lake in the pinned toolchain compiles with `-ffp-contract=off`, and `lean.h` defines `lean_float32_add` as `a + b` (section 9.10) |
| Reduction order | A vectorized or threaded sum adds in several lanes and combines them. That is a different order and gives different bits than a left-to-right sum | Follows from non-associativity, shown in section 9.5. Which libraries do it is not surveyed here |
| Fast-math modes | Flags that permit reassociation, or processor modes that flush subnormals to zero | From memory, not re-checked |
| Library functions | `exp`, `tanh`, `log` and similar are not required by IEEE 754 to be correctly rounded, and math libraries differ in the last bits | From memory of IEEE 754-2019 §9.2, not re-read. This is why D5 excludes them |

One consequence deserves its own line. A native routine that sums in eight lanes is not a faster implementation of the left-to-right reference. It is a different function. To bind it, the reference must be written with that same lane structure, and then tested against it.

## 5. What can be proved, in three tiers

### 5.1 Tier (a): exact bit-level results

**What it is.** A software model of binary32 written in LexLean over integers, as a library. A float operation is an ordinary total function on bit patterns. No new language construct.

**What it lets you state and prove.**

- *Closed instances*: "this operation, or this whole computation, on these inputs returns these bits". Proved by `decide`, which evaluates a closed statement in the kernel (SPEC §17.12, fold laws, rule 6). This is a test vector with the kernel as the judge.
- *Equivalence of two programs over the model*, for all inputs: for example that an in-place array routine returns the same bits as the list-based definition. When both perform the same operations in the same order, this is ordinary induction over the data and needs no fact about floats. This is where gap A's laws relating arrays and lists apply.
- *Universal facts about one operation*: the result is always a valid pattern, addition is commutative, the rounding step is within half a quantum. These need real proofs by case analysis and integer arithmetic. One was done as a sample (section 9.9).

**What it does not give.** Anything about a real-number meaning (that is tier (b)), and anything about hardware (that is section 6).

**Is it in reach now?** Yes, with two honest qualifications.

- The model measured here is plain Lean, not LexLean source. It was written to use only what language 1.2 has: addition, `subtract`, `multiply`, `quotient`, `remainder`, comparison, conditionals, and a loop under a fuel in the shape of `iterate_until`. The repository already contains the same kind of code, verified: `MD-13` decodes binary32 and rounds a quotient to nearest-even "by exact integer Euclidean division and bit-stepping over `iterate_until`, kernel-checked by `decide`" (rule 15), and `Sha256` in the standard library proves its FIPS example digests by reflexivity. The port to LexLean was not done and its verification time was not measured.
- The model covers addition and multiplication only.

**Cost.** Section 7.

### 5.2 Tier (b): error bounds

**What it is.** Theorems that relate a float computation to the exact computation on the numbers the patterns denote: "the computed dot product differs from the exact one by at most this much".

**Per operation**, the fact is standard: a correctly rounded result is within half a *unit in the last place* (the gap between adjacent floats at that magnitude) of the exact result. For the model this is a theorem about integer division and is provable. The core step was proved for this proposal in 15 lines (section 9.9).

**Per computation**, bounds compose badly. The usual bound for an `n`-term dot product grows with `n` and with the sum of the magnitudes of the products. Through layers it grows multiplicatively, and a statement worth having ("the predicted class is the same as with exact arithmetic") needs a margin between the top two outputs, which is a property of one input, not of the network.

**What it would need that the language does not have.**

- Exact rational or dyadic arithmetic with an order, as a library, with its laws.
- Reasoning about products of variables. `linear_arithmetic` lowers to a fixed script ending in `omega`, which decides linear integer arithmetic only.
- Or the real numbers of an imported library (the Mathlib import proposal), and then a way to cite its lemmas from semantic modules, which that proposal lists as undecided.

**Is it in reach now?** No. A cheaper substitute exists inside tier (a): for one given input, compute an exact enclosing interval of the real-valued result by integer arithmetic and let the kernel decide that the class does not change. That is a certificate per input, not a theorem about the network. It is noted, not proposed.

Other proof assistants have large libraries for this (Flocq for Coq, Harrison's work in HOL Light). I state that from memory and did not check their size or whether a comparable Lean library exists.

### 5.3 Tier (c): native floats inside the kernel

**What can be proved: nothing about values.** In Lean 4.32.1, `Float32` is a structure around an opaque type, and `Float32.add`, `Float32.mul`, `Float32.toBits`, `Float32.ofBits` are opaque constants (section 9.2). The kernel has no definition to unfold. Observed (section 9.3):

- `(1.0 : Float32) + 1.0 = 2.0` is not provable by `rfl` ("not definitionally equal") nor by `decide` (no `Decidable` instance; `Float32` has no decidable equality).
- Comparing bits does not help: `((1.0 : Float32) + 1.0).toBits = 0x40000000` is not provable by `rfl` or `decide`; reduction "got stuck".
- Even `(Float32.ofBits 0x3F800000).toBits = 0x3F800000` is not provable.

`native_decide` proves all of them by running compiled code and adding an axiom that says the answer was `true` (section 9.4). That is exactly the kind of assumption LexLean exists to exclude.

Two smaller findings:

- `#print axioms Float32.add` reports `Classical.choice`. Any declaration that mentions a native float operation would carry that axiom in its exact set.
- The kernel can still treat native float operations as unknown functions. Program-equivalence theorems of the second kind in 5.1 would go through on an opaque type. That is a real but narrow capability, and it is not enough to justify a type whose values admit no test vector.

**Is it in reach now?** No, and not by LexLean's effort. It changes only if Lean gives float operations a kernel-reducible definition. I know of no such plan and did not look for one.

## 6. Connecting native execution without a hole

This section depends on gap B as described to this work: an `external` declaration binds a foreign function to a stated contract; it requires a LexLean witness function proved to satisfy the contract, so the assumption cannot be inconsistent; it appears in every dependent theorem's exact observed axiom set as one named contract axiom; and it names its evidence, either a proof or a content-addressed attested conformance report. If gap B lands in a different shape, this section must be re-read against it.

The construction:

1. **Reference.** A LexLean function over the software model, for example `dot (xs ys : list uint32) : uint32`, the left-to-right sum of products from `+0`. All theorems are about this function.
2. **Contract.** "For all inputs of equal length, the result equals `dot xs ys`", as bit equality.
3. **Witness.** The reference itself. The contract is satisfiable because the function that defines it satisfies it. Gap B's consistency requirement is met trivially.
4. **External.** The native routine, taking and returning bit patterns. Its interface has no float type. On the Lean side the conversion is `Float32.ofBits` and `Float32.toBits`; because `toBits` canonicalizes NaN (section 9.2), the native result already has the model's NaN rule.
5. **Evidence.** It cannot be a proof: the kernel cannot see native arithmetic (tier (c)). It is an attested differential conformance report: the native routine and the compiled reference run on the same inputs and every output is compared bit for bit.
6. **What a dependent theorem shows.** One named contract axiom. A theorem that uses only the reference shows none. Nothing is assumed beyond that entry.

What the report has to record to mean anything, from section 4.4: the toolchain and its digest, the target and its instruction-set features, the C compiler flags (contraction off, no fast-math), the rounding and subnormal modes of the processor at run time, the input classes and their sizes, the seed, and the number of mismatches, which must be zero. A report is evidence for one platform. A second platform needs a second report.

Granularity (D9). Binding single operations gives a small contract, but a call across the boundary per multiply defeats the purpose. Binding whole routines gives speed, and the reduction order becomes part of the reference (section 4.4, last paragraph). Recommendation: whole routines, with the single-operation differential included in the same report, because it is the stronger test of the semantics. A binary operation has 2^64 input pairs and cannot be exhausted; a unary one has 2^32 and can.

The prototype of step 5 was run for this proposal: section 9.7. It shows the method works on one host. It is not a conformance report in gap B's sense: nothing was attested or content-addressed.

The same pattern is already written down for integer targets: the acceptance line of open issue #119 reads "Each modeled instruction's semantics is a kernel-checked definition with a differential test against the target". A float routine is that pattern with a larger model.

The honesty level of this evidence (section 13): a differential is `build` evidence, constructed here and validated against an oracle. It is never proof, and no document may call a natively executed float result "verified".

## 7. What it costs

Measured on one machine (section 9.1) that was running other work at the same time. Repeated runs differed by a factor of two to three, so every number is a range and none is a claim.

### 7.1 Lines

| Item | Size | Status |
| --- | --- | --- |
| Scratch model: add and multiply with all special cases | 98 lines of Lean, 82 non-blank | Measured (appendix A) |
| The same as LexLean source | 150 to 300 lines | Estimate. For scale, the binary32/binary64 decoding and scale-ratio code of `MD-13` is about 150 lines of `Recognizer.lex.tex` |
| A first full module (D5: add, subtract, multiply, compare, integer conversions, then divide, square root, fused multiply-add) with its test-vector theorems | 600 to 1,200 lines | Estimate, low confidence. For scale, `Sha256.lex.tex` is 344 lines |
| Language, schema, compiler, backend changes | None | By construction: the model is ordinary integer code |

### 7.2 Proof effort

| Item | Effort | Status |
| --- | --- | --- |
| A closed instance (one test vector) | One line: `decide` | Measured: 18 cases by `decide`, two of them again by `rfl`; none depends on an axiom |
| The half-quantum bound of the rounding step, for all inputs | About 15 lines of Lean with `omega`; a second lemma (ties go to even) about the same. Both accepted at first attempt | Measured (section 9.9). Whether LexLean's `linear_arithmetic` script closes the same goals was not tried |
| "The model's addition is the correctly rounded exact sum", for all inputs | Not attempted | Estimate: days to weeks per operation in LexLean's proof language. Low confidence |
| An error bound for a dot product, for all inputs | Not attempted | Needs what section 5.2 lists first |

### 7.3 Kernel evaluation time

Time the kernel spends per float operation of the model, from Lean's profiler ("type checking", cumulative, divided by the number of operations). "Fuel loops" is the model of appendix A. "Shift and log2" replaces its two loops by `1 <<< n` and `Nat.log2`, which language 1.2 does not have on unbounded naturals.

| Workload | Fuel loops | Shift and log2 |
| --- | --- | --- |
| Add, 1,000 theorems, operands in (-1, 1) | 7.9 to 8.6 ms | 4.8 to 4.9 ms |
| Multiply, 1,000 theorems | 8.4 to 10.4 ms | 4.3 to 4.6 ms |
| Add with the widest alignment (largest finite plus a subnormal), 100 theorems | 47 to 62 ms | 9.5 ms |
| Dot product of 256, one theorem, 512 operations | 8.4 to 11.8 ms | 2.6 to 4.7 ms |
| Dot product of 1,024, one theorem, 2,048 operations | 7.7 to 11.7 ms | not run |

Two further costs sit on top in `lexlean verify`:

- The proof form `decide` lowers to Lean's plain `decide`, which evaluates the statement in the elaborator before the kernel does. Measured in the dot-product theorems: 15 to 23 ms per operation for the fuel-loop model (single multiply theorems were higher, about 30 ms), 7 to 10 ms for the other.
- `leanchecker` replays the module through the kernel a second time. Measured once: 17.5 s of wall time for the 256-element dot-product module, exit 0. That is more than the profiler's kernel line for the same module (4.3 to 6.0 s); the machine was busy and I did not separate load time from checking time.
- So one float operation in a dot product costs roughly 31 to 47 ms of verification: the elaborator once and the kernel twice, by addition of the measured parts. Given the replay figure, read this as a lower estimate.

For comparison, the integer engine's inner loop: a dot product of 1,024 small integers, one theorem, took the kernel 0.27 to 0.40 s, about 0.3 ms per multiply-add. A float multiply-add is two model operations, 15 to 24 ms. The kernel is 40 to 90 times slower on the float model.

What that means for a whole network, by multiplication (the largest single theorem actually run had 2,048 operations): a small classifier with 784 inputs, 128 hidden units and 10 outputs has 101,632 multiply-adds. One test vector for it would take the kernel 26 to 40 minutes per pass on the float model, and at least two to three hours of verification in all. On integers the same network is 26 to 40 seconds per kernel pass. Whether a single `decide` of 200,000 float operations works at all (memory, recursion depth) was not tested.

### 7.4 Run-time cost

Compiled by Lake with the pinned toolchain (`-O3`), nanoseconds per multiply-add in a dot product of one million elements, five runs each (three for the last row):

| Loop | ns per multiply-add |
| --- | --- |
| Native `Float32`, operands held as bit patterns | 4.8, 5.5, 6.7, 7.1, 16.2 |
| Native `Float` (binary64) on `FloatArray` | 1.6, 2.3, 2.5, 2.7, 3.5 |
| Integer, `Int8` operands, `Int64` accumulator | 2.0, 2.2, 3.7, 4.6, 5.5 |
| Integer, `Int8` operands, unbounded `Int` accumulator | 2.7, 3.7, 5.1, 5.9, 12.1 |
| Software model, fuel loops | 271, 429, 483, 515, 667 |
| Software model, shift and log2 | 905, 1607, 1629 |

- Running the model instead of hardware floats costs about a factor of 70 against Lean's own scalar `Float32` loop (medians: 483 ns against 6.7 ns; the five runs gave ratios of 17, 68, 78, 100 and 108), and about 130 against the integer loop, comparing medians.
- That baseline is itself slow. A vectorized native library is faster again by a factor I did not measure.
- Operands with far-apart exponents are much worse. The differential of section 9.7, a third of whose pairs have uniformly random exponents, averaged 8 to 18 microseconds per model operation over the whole run, against about 0.25 in the dot product, because the model aligns exactly on unbounded naturals. A production model would bound the alignment and keep a sticky bit (one bit recording that something nonzero was shifted out). That was not written.
- The variant that is faster in the kernel is slower when compiled. Kernel speed and run-time speed are separate optimization targets.

### 7.5 What an integer-only engine avoids

| Concern | Float engine | Integer engine |
| --- | --- | --- |
| Rounding, NaN, signed zero, subnormals | Must be specified (section 4) and modeled | Do not exist |
| Evaluation order | Part of the meaning. A vectorized sum is a different function | Irrelevant while nothing overflows: integer addition is associative, so any lane order gives the same value |
| Platform differences | Section 4.4 | None for exact integer arithmetic |
| Kernel evaluation | 15 to 24 ms per multiply-add | About 0.3 ms |
| Universal proofs | Case analysis on formats, nonlinear bounds | `linear_arithmetic` and the fold laws the runtime already has (`sumInt_perm` and the additive-fold laws) |
| Native speed | Baseline | Same order (table 7.4) |
| What it cannot say | | That its outputs equal those of the float network it was quantized from. Accuracy is measured on a dataset; the `dataset_agreement` claim states such a count exactly |

## 8. Recommendation

1. **No float type** (D1). Tier (c) is closed by Lean, not by effort.
2. **The integer-only engine is the proved reference.** The model declarations already express it (`integer_feedforward`, `int_tensor`, `requantize`, `MD-13`). Gap A makes it fast in Lean; gap B lets a native integer routine replace it under a contract, with the same differential method and none of section 4.4.
3. **Optionally, and tracked separately: a binary32 software model in the standard library** (tier (a)), for exact bit-level reference semantics. It needs no language change. It should start after the integer engine exists, because nothing in the integer engine depends on it.
4. **Native float routines only through gap B, and only after 3**, since the reference must exist before anything can be tested against it.
5. **No error-bound proofs for networks** (D10).

Did the evidence contradict the expected answer? No. It changed two things I would otherwise have guessed:

- Tier (a) is nearer than expected. The model for two operations is under a hundred lines, the kernel decides it with no axioms, and it matched the hardware on three million pairs without a single correction after it first compiled.
- Tier (a) is not a way to carry whole-network test vectors through the kernel. At about ten milliseconds per operation it suits vectors of thousands of operations, not millions. That limits what the optional module is for: reference semantics and small vectors, with larger runs resting on the compiled model and a differential.

## 9. Evidence

Everything here was run on 2026-11-12. Scratch files lived in a temporary directory outside the repository and are not committed. "Observed" means printed by the command shown.

### 9.1 Environment

| Item | Value |
| --- | --- |
| Lean | `~/.elan/toolchains/leanprover--lean4---v4.32.1/bin/lean --version` printed `Lean (version 4.32.1, x86_64-w64-windows-gnu, commit f054605aea4b840552cca2e725580bffd1e1b704, Release)`. The commit is the one `model/authorities.toml` names |
| Host | AMD Ryzen AI Max 390, 12 cores, 28 GB, Windows 11. Other work was running; timings are noisy |
| Invocation | `lean <file>.lean` for single files. For the model: a Lake project with `lean-toolchain` `leanprover/lean4:v4.32.1`, `lake build`, and `lean -Dprofiler=true -Dprofiler.threshold=100000 <file>.lean` with `LEAN_PATH` at the project's build directory |

### 9.2 What `Float32` is in Lean 4.32.1

File `F1.lean`, run as `lean F1.lean`.

| Command in the file | Observed |
| --- | --- |
| `#check Float32` | `Float32 : Type` |
| `#print Float32` | `structure Float32 : Type`, one field `Float32.val : float32Spec.float` |
| `#print float32Spec` | `opaque float32Spec : FloatSpec` |
| `#print Float32.add` | `opaque Float32.add : Float32 → Float32 → Float32` |
| `#print Float32.mul` | `opaque Float32.mul : Float32 → Float32 → Float32` |
| `#print Float32.toBits` | `opaque Float32.toBits : Float32 → UInt32` |
| `#print Float32.ofBits` | `opaque Float32.ofBits : UInt32 → Float32` |
| `#print Float.add` | `opaque Float.add : Float → Float → Float` |
| `#check @Float.toBits`, `#check @Float.ofBits` | `Float → UInt64`, `UInt64 → Float` |
| `#print axioms Float32.add` | `'Float32.add' depends on axioms: [Classical.choice]` |
| `#eval ((1.0 : Float32) + 1.0).toBits` | `1073741824` (`0x40000000`) |
| `#eval (Float32.ofBits 0x3DCCCCCD + Float32.ofBits 0x3E4CCCCD).toBits` | `1050253722` (`0x3E99999A`), equal to `(0.3 : Float32).toBits` |
| `#eval ((0.0 : Float32) / 0.0).toBits` | `2143289344` (`0x7FC00000`) |
| `#eval (Float32.ofBits 0x7FA00001).toBits` | `2143289344`: the payload is gone |
| `#eval (Float32.ofBits 0x7F800000 - Float32.ofBits 0x7F800000).toBits` | `2143289344` |

So `Float32` exists in 4.32.1, `toBits` and `ofBits` exist, all of them are opaque, and `toBits` returns one pattern for every NaN on this host. File `F4.lean` adds: `#print Float32.beq` gives `opaque Float32.beq : Float32 → Float32 → Bool`, and `example : DecidableEq Float32 := inferInstance` fails with "failed to synthesize instance of type class `DecidableEq Float32`".

### 9.3 The kernel cannot evaluate native floats

File `F2.lean`, run as `lean F2.lean`. Every line is an error; the messages are the evidence.

| Statement and proof | Observed error |
| --- | --- |
| `(1.0 : Float32) + 1.0 = 2.0 := by rfl` | "Tactic `rfl` failed: The left-hand side `1.0 + 1.0` is not definitionally equal to the right-hand side `2.0`" |
| the same `:= by decide` | "failed to synthesize `Decidable (1.0 + 1.0 = 2.0)`" |
| `((1.0 : Float32) + 1.0).toBits = (2.0 : Float32).toBits := by rfl` | "not definitionally equal" |
| the same `:= by decide` | "its `Decidable` instance ... did not reduce to `isTrue` or `isFalse`", "reduction got stuck" |
| `((1.0 : Float32) + 1.0).toBits = 0x40000000 := by decide` | the same, stuck |
| `(Float32.ofBits 0x3F800000).toBits = 0x3F800000 := by decide` | the same, stuck |
| `((1.0 : Float) + 1.0).toBits = (2.0 : Float).toBits := by decide` | the same, stuck |
| `((1.0 : Float32) + 1.0 == 2.0) = true := by decide` and `by rfl` | stuck; not definitionally equal |

### 9.4 What `native_decide` adds

File `F3.lean`, run as `lean F3.lean`.

| Theorem | `#print axioms` observed |
| --- | --- |
| `nd_bits : ((1.0 : Float32) + 1.0).toBits = 0x40000000 := by native_decide` | `[Classical.choice, nd_bits._native.native_decide.ax_1_1]` |
| `nd_beq : ((1.0 : Float32) + 1.0 == 2.0) = true := by native_decide` | `[Classical.choice, nd_beq._native.native_decide.ax_1_1]` |
| `nd_sum : (Float32.ofBits 0x3DCCCCCD + Float32.ofBits 0x3E4CCCCD).toBits = 0x3E99999A := by native_decide` | `[Classical.choice, nd_sum._native.native_decide.ax_1_1]` |
| `k_nat : (2 : Nat) ^ 31 + 2 ^ 31 = 4294967296 := by decide` (contrast) | "does not depend on any axioms" |

`#print nd_bits._native.native_decide.ax_1_1` shows `axiom nd_bits._native.native_decide.ax_1_1 : decide ((1.0 + 1.0).toBits = 1073741824) = true`. In this toolchain each use of `native_decide` declares its own axiom stating that the compiled evaluation returned `true`.

### 9.5 A software model the kernel can decide

The model is appendix A: `SF.add` and `SF.mul` on bit patterns held as natural numbers, using `+`, `-`, `*`, `/`, `%`, comparison, `if`, and two structurally recursive loops under a fuel. No `Float`, no bitwise operation, no shift, no `Nat.log2`, no well-founded recursion.

File `Cases.lean`, under `set_option maxRecDepth 100000` and `set_option maxHeartbeats 1000000000` (the options generated modules carry). The file has 20 theorems: the 18 below, each `:= by decide`, and two repeats by `rfl`. All were accepted the first time the file compiled.

| Theorem | Statement | Meaning |
| --- | --- | --- |
| `one_plus_one` | `add 0x3F800000 0x3F800000 = 0x40000000` | 1.0 + 1.0 = 2.0 |
| `tenth_plus_fifth` | `add 0x3DCCCCCD 0x3E4CCCCD = 0x3E99999A` | 0.1 + 0.2, rounded |
| `sub_plus_sub` | `add 0x00000001 0x00000001 = 0x00000002` | smallest subnormal doubled, exact |
| `minnormal_half` | `mul 0x00800000 0x3F000000 = 0x00400000` | smallest normal times 0.5 is a subnormal |
| `sub_tie_even` | `mul 0x00000003 0x3F000000 = 0x00000002` | a tie in the subnormal range goes to even |
| `sub_tie_zero` | `mul 0x00000001 0x3F000000 = 0x00000000` | underflow of a tie to +0 |
| `max_plus_max` | `add 0x7F7FFFFF 0x7F7FFFFF = 0x7F800000` | overflow to +infinity |
| `max_times_two` | `mul 0x7F7FFFFF 0x40000000 = 0x7F800000` | overflow to +infinity |
| `inf_minus_inf` | `add 0x7F800000 0xFF800000 = 0x7FC00000` | canonical NaN |
| `inf_times_zero` | `mul 0x7F800000 0x00000000 = 0x7FC00000` | canonical NaN |
| `nan_payload` | `add 0x7FA00001 0x3F800000 = 0x7FC00000` | a payload in, the canonical NaN out |
| `negzero_plus_zero` | `add 0x80000000 0x00000000 = 0x00000000` | -0 + +0 = +0 |
| `negzero_plus_negzero` | `add 0x80000000 0x80000000 = 0x80000000` | -0 + -0 = -0 |
| `cancel_is_poszero` | `add 0x3F800000 0xBF800000 = 0x00000000` | 1 + -1 = +0 |
| `negone_times_zero` | `mul 0xBF800000 0x00000000 = 0x80000000` | -1 * +0 = -0 |
| `assoc_left` | `add (add 0x4CBEBC20 0xCCBEBC20) 0x3F800000 = 0x3F800000` | (1e8 + -1e8) + 1 = 1 |
| `assoc_right` | `add 0x4CBEBC20 (add 0xCCBEBC20 0x3F800000) = 0x00000000` | 1e8 + (-1e8 + 1) = 0 |
| `max_plus_tiny` | `add 0x7F7FFFFF 0x00000001 = 0x7F7FFFFF` | widest alignment the format has |

`one_plus_one_rfl` and `tenth_plus_fifth_rfl` state the first two again with `:= rfl`; both accepted.

`#print axioms` on each of the 20 theorems printed "does not depend on any axioms".

The expected patterns in this table were written by hand before the file was run, not copied from the model's output.

### 9.6 Kernel time

Theorem files were generated by a small compiled program that computes each expected pattern with the compiled model, then timed with `lean -Dprofiler=true -Dprofiler.threshold=100000 <file>`. The profiler's cumulative "type checking" line is kernel time; "tactic execution" is the elaborator running `decide`. Two runs each; both shown.

| File | Proof | Kernel | Elaborator |
| --- | --- | --- | --- |
| Fuel loops, 1,000 add theorems | `by decide` | 7.88 s, 8.63 s | 18.3 s, 23 s |
| Fuel loops, 1,000 add theorems | `rfl` | 5.86 s, 6.64 s | not reported |
| Fuel loops, 1,000 mul theorems | `by decide` | 10.4 s, 8.44 s | 29.5 s, 32.5 s |
| Fuel loops, 100 widest-alignment add theorems | `by decide` | 6.17 s, 4.67 s | 14.8 s, 12.1 s |
| Fuel loops, dot of 64 | `by decide` | 1.42 s, 3.53 s | 2.52 s, 3.07 s |
| Fuel loops, dot of 256 | `by decide` | 5.69 s, 4.31 s | 11.6 s, 7.57 s |
| Fuel loops, dot of 256 | `by decide +kernel` | 4.74 s, 6.03 s | under 0.3 s |
| Fuel loops, dot of 1,024 | `by decide +kernel` | 23.9 s, 15.7 s | under 0.1 s |
| Shift and log2, 1,000 add theorems | `by decide` | 4.84 s, 4.94 s | 10.7 s, 10.4 s |
| Shift and log2, 1,000 mul theorems | `by decide` | 4.31 s, 4.57 s | 9.34 s, 12 s |
| Shift and log2, 100 widest-alignment add theorems | `by decide` | 0.95 s, 0.95 s | 1.85 s, 1.74 s |
| Shift and log2, dot of 64 | `by decide` | 0.62 s, 0.59 s | 1.13 s, 1.04 s |
| Shift and log2, dot of 256 | `by decide` | 1.32 s, 2.33 s | 3.73 s, 5.15 s |
| Shift and log2, dot of 256 | `by decide +kernel` | 2.42 s, 2.41 s | under 0.1 s |
| Integer dot of 1,024 (`acc + x * y` on `Int`) | `by decide +kernel`, `by decide` | 0.40 s, 0.27 s | 0.04 s, 0.60 s |

All runs exited 0 with no error. Scaling from 256 to 1,024 elements is linear within the noise. In the two fuel-loop `decide +kernel` rows for 256 elements the wall time exceeded the sum of the profiler's lines by 7 to 10 s; I did not find out where that time is counted.

`leanchecker`: the fuel-loop dot-of-256 module was compiled with `lean -o` (32.7 s wall in that run) and replayed with `leanchecker Dot256` (17.5 s wall, exit 0). One run.

A side observation, not about floats: the integer file took 70 to 120 s of wall time, of which the profiler attributed 54.6 s to typeclass inference on the 2,048 unascribed integer literals. Generated LexLean modules ascribe every literal and split long lists (rule 11), so this does not transfer.

### 9.7 Differential against the host's `Float32`

A compiled program (`lake build`, then run) compared, for each operand pair, `SF.add a b` with `(Float32.ofBits a + Float32.ofBits b).toBits`, and the same for multiplication. Inputs: all 1,764 ordered pairs of 42 hand-picked edge patterns (zeros, subnormals, the normal boundaries, powers of two near 2^24, the largest finite values, infinities, NaNs with and without payload), then 3,000,000 pseudo-random pairs in three equal classes: uniform bit patterns; exponents within 31 of each other, so that sums round and cancel; exponents chosen so that products land near the subnormal range.

Observed, first run with both model variants:

```text
differential: 3001764 pairs, add mismatches 0, mul mismatches 0, shift/log2 variant mismatching pairs 0
  results that were NaN 46704, infinite 440247, subnormal 259164 (of 6003528)
  wall 94241 ms
dot of 1000000: model 43ccdc85 native 43ccdc85 equal true
```

An earlier run of the fuel-loop model alone printed the same counts with zero mismatches, in 106,707 ms. The last line is a left-to-right dot product of one million pseudo-random values in (-1, 1): the model and the native loop end on the same 32 bits.

This is agreement with one implementation on one host: x86-64, Windows, the C compiler shipped with Lean 4.32.1. It is not a proof that the model is IEEE 754, and it says nothing about ARM or WebAssembly.

### 9.8 Run time

The same program timed five loops over one million elements; the numbers are table 7.4. Each loop was marked `@[noinline]` and given a start index that varies per repetition, after a first version was found to have been evaluated once and reused by the compiler (it reported 0.0005 ns per operation). That first version's timings are discarded.

### 9.9 One universal lemma

File `Bound.lean`, plain Lean, no imports beyond the prelude. With

```lean
def rne (m d : Nat) : Nat :=
  if d / 2 < m % d || (m % d == d / 2 && (m / d) % 2 == 1) then m / d + 1 else m / d
```

(divide by `d`, round to nearest, ties to even: the rounding step of the model), two theorems were accepted:

```lean
theorem rne_within_half (m d : Nat) (hd : 0 < d) (he : d % 2 = 0) :
    2 * (rne m d * d) ≤ 2 * m + d ∧ 2 * m ≤ 2 * (rne m d * d) + d

theorem rne_tie_even (t d : Nat) (hd : 0 < d) (he : d % 2 = 0) :
    rne (t * d + d / 2) d % 2 = 0
```

The first says the rounded value times the quantum is within half a quantum of `m`, for every `m` and every even positive `d`. Its proof is about 15 lines: `Nat.div_add_mod`, `Nat.mod_lt`, two rewrites to put the product in one shape, a case split on the condition, and `omega`, which here has to treat the product `d * (m / d)` as one unknown. The second proof is about the same length. `#print axioms` printed `[propext, Classical.choice, Quot.sound]` for both.

This is one step of one operation. It was chosen because it is the part every operation shares.

### 9.10 How the pinned toolchain compiles float code

Observed in the Lake build of the scratch project:

- `include/lean/lean.h` in the toolchain has `static inline float lean_float32_add(float a, float b) { return a + b; }` and the same shape for `lean_float32_mul`.
- The C that Lean emitted for the native dot-product loop calls `lean_float32_mul` then `lean_float32_add`, in that order, once per element.
- Lake's recorded compiler arguments for every module contain `-O3` and `-ffp-contract=off`.

So in this toolchain a `Float32` multiply followed by an add is compiled as two operations and is not fused. This is an observation of one toolchain on one host, not a guarantee by Lean.

## 10. Limits of this evidence

- The model has addition and multiplication. Subtraction is addition with a flipped sign and was not written. Division, square root, fused multiply-add, comparison, and conversions were not written.
- The model is Lean, not LexLean. No `.lex.tex` was written, `lexlean verify` was not run on any of it, and no axiom policy was audited by LexLean.
- There is no proof that the model equals IEEE 754. The evidence is 18 hand-written cases, one differential on one host, and the fact that no correction was needed.
- The differential draws three million pairs out of 2^64.
- Timings come from a busy machine and vary by a factor of two to three.
- The whole-network figures in 7.3 are multiplications, not measurements.
- The model was not optimized for either the kernel or the compiler. Both could improve; I do not know by how much.
- binary64 was not modeled. The construction is the same with wider products.
- Statements marked "from memory" in sections 4.4 and 5.2 were not re-checked. The two specifications that were consulted were read through a summarizing tool (section 13).
- Gap B is described second-hand. Section 6 is conditional on it.

## 11. Not implemented by this PR

Nothing is implemented by this pull request. Specifically not:

| Not implemented | What would have to be true before it is started |
| --- | --- |
| A floating-point type, literal, or artifact element in any language version | Lean gives `Float32` operations a definition the kernel can reduce, or the owner reverses `PF-17`/`LN-04`. Neither is proposed. Re-run sections 9.2 to 9.4 at every pin move |
| Tier (a): a binary32 model as LexLean source | The owner accepts D2 to D7. The integer engine exists, so that this is not on its path. A register row, a scenario, and a failing test named for it come first (R3). The oracle of its differential is chosen (§17.22). The whole first operation set can be finished in the change that starts it (R4) |
| Universal correctness theorems for the model ("add is the correctly rounded sum") | Tier (a) exists. A trial shows whether `linear_arithmetic` closes goals of the kind in section 9.9, or which proof form is missing |
| Tier (b): error bounds | The universal correctness theorems exist. A dyadic-rational library with order laws exists. A proof route for products of variables exists (a new proof form, or citing an imported library's lemmas from semantic modules). Someone names a network and a bound they need |
| Tier (c): proofs about native floats | Not startable. See the first row |
| A native float routine bound to the model | Gap B is merged with an attested conformance report as an evidence kind. Tier (a) exists for the operations the routine uses, with the routine's evaluation order written into the reference. A harness produces the report, content-addressed, with the fields of section 6 |
| A differential harness in this repository | Same as the previous row |
| binary64 | D2 revisited after binary32 exists |
| Any register row, scenario, authority row, SPEC text, or schema | The owner's decisions in section 12 |
| The scratch model, tests, and benchmark | They are evidence for this document, kept out of the repository on purpose. Appendix A is enough to re-run them |

## 12. Decisions that belong to the owner

### D1. A floating-point type in the language

No. Section 5.3: no theorem about a value is possible, and the only tactic that would give one is forbidden. Beyond Lean, the linked IR has no floating-point values and the claim about rendered Rust excludes floating point, so a float type would also need work in the IR, the production calculus and the Rust backend. A type that can be computed with but not reasoned about is the opposite of what the language is for.

### D2. Which format first

binary32. It is what inference engines use, rule 15 already fixes its layout, and products fit in 48 bits. binary64 is the same construction and adds nothing new to decide.

### D3. NaN

One canonical quiet NaN, `0x7FC00000`. Section 4.2. The alternative, rejecting NaN and infinity as `MD-13` does, is right for parameters and wrong for arithmetic. The cost is a stated deviation from IEEE 754 for sign-bit operations on NaN operands.

### D4. Signed zero and equality

Keep signed zero. State theorems with bit equality. Provide IEEE comparison as a library function with a different name. Section 4.3.

### D5. Operations

First: add, subtract, multiply, compare, conversion from and to integers. These are enough for a dense layer and a ReLU. Second: divide, square root, fused multiply-add, which IEEE 754 also requires to be correctly rounded. Never in the exact model: `exp`, `tanh`, `log`, and the like. A network that needs them gets an integer table or a stated integer approximation, which is what quantized engines do.

### D6. Where the model lives

The standard library, under §17.22, because that section already demands what this needs: language-1.2 source, production roots, and a differential against an oracle before a member is committed. The natural oracle is the native float of the host the differential runs on, through its bit conversions. Which host language supplies it (Rust's `f32` under the pinned rustc, as other members use, or Lean's `Float32`) is part of this decision; the experiment here used Lean's.

### D7. Representation

`UInt32` at the interface, because a pattern is 32 bits and the fixed-width type says so. `Nat` and `Int` inside, because the kernel evaluates them directly; the measured model does this. Whether fixed-width arithmetic inside would be faster compiled and slower in the kernel was not measured. No new type either way.

### D8. New primitives

The shift-and-log2 variant is about twice as fast in the kernel and five times on the worst case (table 7.3), and slower when compiled (table 7.4). Adding shift and bit length on unbounded naturals is a language change that moves the language identity and every file carrying it. Not worth it for an optional module. If such primitives arrive for another reason, the model should use them only if the compiled slowdown is fixed.

### D9. Granularity of a native binding

Whole routines, with the order in the reference. Section 6. This decision waits on gap B.

### D10. Error bounds

Not proposed. Section 5.2. The per-input interval certificate mentioned there is a possible later item if someone needs a robustness statement for specific inputs.

### D11. Tracking

Open one issue for the optional binary32 module, pointing at this document. Add no register row now: a row is a claim that must be discharged in the change that adds it.

## 13. Honesty levels and authorities

Whose claim each statement is (AGENTS.md rule 4, SPEC §27.3):

| Statement | Belongs to | What this document did |
| --- | --- | --- |
| What binary32 arithmetic is, how rounding and signed zero behave | IEEE 754-2019 | Cited by section number from memory. The standard was not re-read. If a register row ever cites it, it needs an authority row first, at level `some-true` |
| That `Float32` is opaque, what `native_decide` adds, that `toBits` canonicalizes NaN | Lean 4.32.1 | Observed on the pinned toolchain (sections 9.2 to 9.4). These are observations of one release on one host, not guarantees by Lean |
| That the kernel decides the model's instances with no axioms | Lean 4.32.1's kernel, about a scratch file | Observed (section 9.5). Not a LexLean claim until a LexLean module states it and `lexlean verify` passes |
| WebAssembly's NaN rule; Clang's contraction default | Their specifications | Fetched on 2026-11-12 through a tool that summarizes pages; the quotations are as that tool returned them and were not compared with the pages character by character |
| x87 behaviour, hardware NaN propagation, fast-math modes, library-function accuracy, the size of other provers' float libraries | Their authorities | From memory. Not verified |
| That the model agrees with native floats | Nobody yet | Measured once on one host. If registered, a differential is `build`: constructed here, validated against an oracle, evidence and not proof |
| Every timing | Nobody | Measured and reported. If ever registered, `open`: never asserted, never acceptance evidence |

The word "verified" does not apply to anything in this document.

## 14. Checks run on this document

- Every Lean statement in section 9 was run on the pinned toolchain on the day of writing, and the outputs are copied from the terminal, not reconstructed.
- Register IDs cited (`MD-04`, `MD-13`, `PF-17`, `LN-04`) and SPEC sections cited (§4.2, §5.3, §16.12, §17.9, §17.12, §17.14, §17.22, §21.7, §27.3) were opened on `release/v0.4` at `aaf845507`.
- A search of `language/`, `stdlib/src`, and `std-models/src` for "float" found nothing.
- The file uses LF line endings. Its only non-ASCII characters are the section sign and Lean's own symbols in quoted Lean.
- Not run: the repository gate. This change adds one file under `docs/`, which no gate reads; CI is the authority.

## Appendix A. The model that was measured

Plain Lean 4.32.1, no imports beyond the prelude. Bit patterns are natural numbers below 2^32. This is evidence, not proposed source: a LexLean module would be written in the authoring notation and would take `UInt32`.

```lean
namespace SF

def p23 : Nat := 8388608
def p24 : Nat := 16777216
def p31 : Nat := 2147483648

def sign (b : Nat) : Nat := b / p31
def expo (b : Nat) : Nat := (b / p23) % 256
def frac (b : Nat) : Nat := b % p23

def isNaN (b : Nat) : Bool := expo b == 255 && frac b != 0
def isInf (b : Nat) : Bool := expo b == 255 && frac b == 0
def isZero (b : Nat) : Bool := expo b == 0 && frac b == 0

def qNaN : Nat := 2143289344
def inf (s : Nat) : Nat := s * p31 + 2139095040

/-- A finite pattern is `mant * 2^(ex - 150)`. -/
def mant (b : Nat) : Nat := if expo b == 0 then frac b else p23 + frac b
def ex (b : Nat) : Nat := if expo b == 0 then 1 else expo b

/-- 2^n by a fuel loop: no power primitive is assumed. -/
def pow2 : Nat → Nat
  | 0 => 1
  | n + 1 => 2 * pow2 n

/-- Number of bits of `m`, by halving under a fuel. -/
def bitLenAux : Nat → Nat → Nat → Nat
  | 0, _, acc => acc
  | fuel + 1, m, acc => if m == 0 then acc else bitLenAux fuel (m / 2) (acc + 1)

def bitLen (m : Nat) : Nat := bitLenAux 320 m 0

/-- Pack a rounded significand `t <= 2^24` with quantum exponent `q >= -149`. -/
def pack (s t : Nat) (q : Int) : Nat :=
  let t1 : Nat := if t == p24 then p23 else t
  let q1 : Int := if t == p24 then q + 1 else q
  if t1 < p23 then s * p31 + t1
  else
    let biased : Int := q1 + 150
    if biased ≥ 255 then inf s else s * p31 + biased.toNat * p23 + (t1 - p23)

/-- Round the exact magnitude `m * 2^e` to binary32, ties to even. -/
def roundPack (s m : Nat) (e : Int) : Nat :=
  if m == 0 then s * p31
  else
    let top : Int := e + (bitLen m : Int) - 1
    let q : Int := if top - 23 < -149 then -149 else top - 23
    if q ≤ e then pack s (m * pow2 (e - q).toNat) q
    else
      let d : Nat := pow2 (q - e).toNat
      let t : Nat := m / d
      let r : Nat := m % d
      let h : Nat := d / 2
      let up : Bool := h < r || (r == h && t % 2 == 1)
      pack s (if up then t + 1 else t) q

def add (a b : Nat) : Nat :=
  if isNaN a || isNaN b then qNaN
  else if isInf a then (if isInf b && sign a != sign b then qNaN else a)
  else if isInf b then b
  else
    let ea := ex a
    let eb := ex b
    let e := if ea < eb then ea else eb
    let ma := mant a * pow2 (ea - e)
    let mb := mant b * pow2 (eb - e)
    let x : Int := (e : Int) - 150
    if sign a == sign b then roundPack (sign a) (ma + mb) x
    else if ma == mb then 0
    else if mb < ma then roundPack (sign a) (ma - mb) x
    else roundPack (sign b) (mb - ma) x

def mul (a b : Nat) : Nat :=
  if isNaN a || isNaN b then qNaN
  else
    let s : Nat := if sign a == sign b then 0 else 1
    if isInf a then (if isZero b then qNaN else inf s)
    else if isInf b then (if isZero a then qNaN else inf s)
    else roundPack s (mant a * mant b) ((ex a : Int) + (ex b : Int) - 300)

/-- Left-to-right dot product: `acc := add acc (mul x y)`. -/
def dot : List Nat → List Nat → Nat → Nat
  | x :: xs, y :: ys, acc => dot xs ys (add acc (mul x y))
  | _, _, acc => acc

end SF
```

The "shift and log2" variant replaces `pow2 n` by `1 <<< n` and `bitLen m` by `if m == 0 then 0 else Nat.log2 m + 1`, and is otherwise identical.

How it works, in one paragraph: a finite pattern is an integer significand times a power of two. A product is the product of the significands and the sum of the exponents; a sum is taken after scaling both significands to the smaller exponent. Both are exact on unbounded naturals. `roundPack` then finds the result's leading bit, chooses the exponent of the last kept bit (never below -149, which is what makes subnormals), divides, rounds to nearest with ties to even, and `pack` writes the fields, carrying into the exponent when rounding reached 2^24 and returning infinity when the exponent passes the top.
