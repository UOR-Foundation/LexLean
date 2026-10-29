# Mathlib import: specialisation wrappers and the signature probe (notes for #341)

Issue #336, tracking issue #333. Branch `feat/mathlib-wrappers`, local commits only.
Register rows `GL-19` and `VR-24`; diagnostics `LLT4017` and `LLT4018`;
SPEC.md §13.12 (new) and §18.8 (amended).

## The problem

A Mathlib theorem such as `add_comm` is stated over a class
(`{G} [AddCommMagma G] (a b : G)`). A glossary entry carries an LSE signature
(§13.8) with no instance binders, so the declaration cannot be denoted as it is.
The fix is a monomorphic constant: `@add_comm Nat _`, declared in the project's
host Lean library, which an entry then denotes (§13.6).

## What was built

**Emitter.** `lexlean::wrappers::{parse_request, emit}` (hidden module, like
the other internals) and `cargo xtask emit-wrappers <request.toml> <host.lean>`.

Input, `lexlean/wrappers/1` (TOML):

```toml
spec = "lexlean/wrappers/1"
imports = ["Mathlib.Data.BitVec"]     # sorted, unique Lean modules

[[wrapper]]
declaration = "add_comm"              # the class-polymorphic constant
kind = "theorem"                      # or "def"
tag = "BitVec32"                      # names the specialisation
arguments = ["(BitVec 32)", "_"]      # explicit arguments of @declaration
```

Arguments are a closed term grammar: `_`, a name, `@name`, a decimal, or a
parenthesised application. No tactic, `by`, `fun`, string, ascription or comment
can be written; Lean syntax words (`by`, `fun`, `sorry`, ...) are refused by
name, and every name is printed `_root_`-qualified so a wrapper cannot capture
another. Anything else is `LLT4018`.

Output, for each wrapper (sorted by Lean name, so the request's order does not
reach the bytes):

```lean
public theorem LexLeanSpec.BitVec32.add_comm : type_of% (@_root_.add_comm (_root_.BitVec 32) _) := @_root_.add_comm (_root_.BitVec 32) _
```

The module header is `module`, one `public import` per import, and
`set_option autoImplicit false`. The `xtask` also prints one tab-separated line
per wrapper (Lean name, declaration, tag, line) and the library gives
`EmittedWrapper::denotation(module)`, the `[denotation]` table of an entry that
denotes it.

The emitter never runs Lean and never writes a signature. The type of the wrapper
is whatever Lean elaborates for the application (instances come from instance
resolution at the `_`); it is a term, adds no axiom of its own, and shows no
tactic. For #335: produce the request, build the host library, read each
wrapper's type from Lean, write the entry's LSE signature from it. The probe is
the safety net whichever way the signature was produced.

**Probe.** An external entry whose Lean name is under the reserved root
`LexLeanSpec` is probed by

```lean
example : (<signature>) = (type_of% @<wrapper>) := rfl
```

instead of `example : <signature> := <wrapper>`. The first form compares the two
types by definitional equality at default transparency, binder for binder. The
second is weaker: Lean inserts implicit arguments and synthesises instances while
checking it, so a signature without the instance binder is accepted. Every other
entry keeps its old line, so no existing example's probe changes.

Diagnostics:

- `LLT4017`: the signature of a wrapper entry is not the type of the wrapper
  (names the entry and carries Lean's message).
- `LLT4003` (existing): the wrapper does not exist (Lean's `unknownIdentifier`).
- `LLT4018`: the specialisation request is refused.

## Evidence

Fixture `tests/fixtures/specialisation`: request for `Std.Commutative.comm` and
`Std.Associative.assoc` at `Nat`, `Int`, `UInt32`, `BitVec 32` (8 wrappers, all
elaborate under v4.32.1); a project whose entries denote the `Nat` and `Int`
commutativity wrappers.

- `conformance_gl_19`, `conformance_vr_24`: written failing first (`no lexicon
  case is wired for GL-19`), then pass.
- Positive: `verified 1 module`; the probe holds `example : ((x0 : Nat) -> (x1 :
  Nat) -> Eq (Nat.add x0 x1) (Nat.add x1 x0)) = (type_of%
  @LexLeanSpec.Nat.Std.Commutative.comm) := rfl`. The LSE writes `Nat.add`, the
  wrapper's type writes `a + b` through instances: they are definitionally equal.
- Planted wrong signature (`b + a` replaced by `a + b`):
  `LLT4017: the signature of test.spec::nat-comm is not definitionally the Lean
  type of the specialisation wrapper it denotes ... (∀ (x0 x1 : Nat), x0.add x1 =
  x0.add x1) = ∀ (a b : Nat), a + b = b + a`.
- Missing wrapper: `LLT4003 ... Unknown identifier LexLeanSpec.Nat.Std.Commutative.nope`.
- Seven defects planted in the source, each caught by the named test (table in
  VERIFICATION.md). The fixture also caught a real defect of mine: an
  unparenthesised signature (`a -> b = c` is `a -> (b = c)`).
- Mathlib at the pin (`520045ab14e2...`, `~/mathlib-pinned`, toolchain v4.32.1):
  `add_comm` at `Nat`, `Int`, `BitVec 32`, `add_assoc` at `BitVec 32` and
  `mul_comm` at `Int` build from `import Mathlib.Data.BitVec`; the strict probe
  line accepts the `Nat`, `Int` and `BitVec 32` signatures written with
  `Nat.add`, `Int.add`, `BitVec.add` and refuses the planted one. Axioms:
  `Nat.add_comm` wrapper `[propext]`, `BitVec 32` wrapper
  `[propext, Classical.choice, Quot.sound]`.
- `UInt32` has no `AddCommMagma` in Mathlib at this pin: that wrapper fails at the
  host build (`failed to synthesize AddCommMagma UInt32`). A specialisation exists
  only where Mathlib has the instance.

## How many class-polymorphic declarations become usable

Measured with a Lean script (not part of the repository; recipe below) over
`Init`, `Std` and `Lean`, theorems and definitions that are public, not internal,
and have an instance binder; a declaration counts at a type when the first
sort-typed binder takes the type, every instance binder is found by instance
resolution, and every binder up to the last instance binder ends up assigned
(that is, `@decl T _ ... _` elaborates).

| | declarations |
| --- | --- |
| class-polymorphic candidates | 30200 |
| specialisable at some of the four types | 4146 (2804 theorems) |
| at `Nat` / `Int` / `UInt32` / `BitVec 32` (all) | 3778 / 3959 / 3720 / 3428 |
| at `Nat` / `Int` / `UInt32` / `BitVec 32` (theorems) | 2486 / 2713 / 2535 / 2308 |

The rest are mostly underdetermined by the carrier alone: `Std.Commutative.comm`
needs `op` stated (the fixture gives it `(@HAdd.hAdd T T T _)`), so a generator must
choose arguments, not only a type.

The same count over the Mathlib modules that `Mathlib.Data.BitVec` imports at
the pin (the closure of that one module, not all of Mathlib; this includes
`Init`, `Std` and `Lean`):

| | declarations |
| --- | --- |
| class-polymorphic candidates | 38427 |
| specialisable at some of the four types | 8437 (6502 theorems) |
| at `Nat` / `Int` / `UInt32` / `BitVec 32` (all) | 6963 / 8140 / 4580 / 6229 |
| at `Nat` / `Int` / `UInt32` / `BitVec 32` (theorems) | 5217 / 6292 / 3309 / 4612 |

`UInt32` has far fewer because Mathlib gives it almost no algebraic instances. A
count over all of `Mathlib` was not run.

Recipe: for each constant, `forallMetaTelescopeReducing` its type, unify the first
sort binder with the type, `trySynthInstance` each instance binder, check the
prefix is assigned. Scripts are in the session scratch folder
(`MeasureStd.lean`, `MeasureMathlib.lean`, copied to the shared notes folder of the
session); say if they should be committed.

## Open owner decisions

1. **The peano-arithmetic example is not converted.** Its host library
   `PeanoArithmeticHost.lean` is an empty module and its entries denote
   `Init`. Making entries denote wrappers there changes its probe, closure and lock
   files (`expected/`), and requires the host library to be built before
   `verify-examples`. Nothing builds a host library today: `verify` runs
   `lake env lean` and a denoted module must already have an `.olean`. Proposal: a
   documented prerequisite (`lake build <HostLib>`), or a `verify` stage that builds
   it. The fixture does the former.
2. **Strict probe for all external entries?** The weakness above applies to every
   entry. Applying the strict line to all would change every committed probe and so
   every example's `expected/verify`, and needs a regeneration.
3. **Reserved root `LexLeanSpec` and the name `LexLeanSpec.<tag>.<declaration>`.**
   The probe keys on the root. A project that declares its own `LexLeanSpec.*` outside
   the emitter gets the strict line too (harmless but undocumented).
4. **CLI.** Only `xtask` and the library call the emitter. A `lexlean` subcommand
   changes §23.4 and its rows.
5. **Who writes the LSE signature of a wrapper.** The emitter does not. #335 needs a
   Lean-side exporter from the built wrapper's type to LSE.
6. **`def` wrappers.** Accepted by the grammar; only theorems were built and
   measured. A computable `def` may need `noncomputable`; not handled.
7. **Axioms.** A wrapper adds none, but the declaration it applies brings its own
   (the `Int` fixture theorem needs `\allowaxioms{propext}`). The document's axiom
   audit sees them; the host library itself is not audited.
8. **Register collisions.** Rows `GL-19`, `VR-24`, codes `LLT4017`, `LLT4018`, and the
   register total (459) in `SPEC.md`, `README.md`, `xtask/src/spec_links.rs`,
   `crates/conformance/src/cases/repository.rs` will conflict with the siblings'.
