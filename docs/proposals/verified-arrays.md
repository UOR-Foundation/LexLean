# Arrays of fixed-width integers: design

Written 2026-11-12 for owner acceptance. Normative text is `SPEC.md` §17.12, *Arrays*; this document explains the design, records the evidence, and lists what is left to the owners. Register row: `SM-39`. Register total on this branch: 494.

## 1. Summary

Language 1.2 has lists, byte strings, maps and sets. A list is indexed in linear time and a byte string cannot be written. An inference engine needs a sequence of machine integers that is read and written by position, and proofs about loops over it. This change adds exactly that and nothing else:

1. One type, `array{element: T}`, where `T` is one of the eight fixed-width integer types. It lowers to the pinned Lean's `Array T`.
2. `length` and `index`, which exist, now also apply to an array. Three new primitives: `array_from_list`, `array_to_list`, `array_set`.
3. One new runtime module, `LexLeanRuntime.Arrays`, written as LexLean source like the other five. It holds two instances, three definitions and nine laws. Every law depends on `propext` at most.
4. The meaning of an array is the list of its elements. Two laws (`array_index`, `array_length`) turn a loop over an array into the same loop over a list; from there the proof is an ordinary induction in the existing proof forms. No proof form is added.
5. Loops are ordinary definitions: structural recursion on a fuel counter that carries the position. No iteration construct is added.

**Acceptance result.** `examples/arrays` defines an int8 dot product over two arrays by index and proves `dot left right = dotList (array_to_list left) (array_to_list right)` for all arrays, where `dotList` is the plain recursion on two lists. `lexlean check` and `lexlean verify` pass; the theorem's observed axioms are exactly `Quot.sound` and `propext` (section 6). The same example writes an array in place (a rectification loop) and proves that the write keeps the length.

Decisions that need the owners:

| # | Decision | Recommendation |
| --- | --- | --- |
| D1 | `array_set` total, or returning `Option` like `prim update` of #443 | Total. Every law is then an equation, and a write allocates nothing |
| D2 | Element types | The eight fixed-width integers only. Widening later is compatible; narrowing is not |
| D3 | Production (certified Rust) | Formal-only in this change, as text components are today. A Rust realization is a separate item that should follow #443's pattern |
| D4 | A readable notation `(Array T)` | Not in this change. It would change the canonical spelling of any document type named `Array` |
| D5 | The register ID | `SM-39` is free on `release/v0.4`. Issue #434 uses the same number on the `v0.5` line and would move to the next free one at the forward merge |
| D6 | Byte-packed storage | Not proposed. `Bytes` is the packed byte sequence; #443 gives it an in-place write |

## 2. What exists and what is in flight

- `Bytes` (Lean `ByteArray`) is indexed in constant time by `index` and has no write. PR #443 adds `prim update` on `Bytes` with a full Rust production path; it targets `main`, and its checks are red at the time of writing. This change does not touch `Bytes` and does not duplicate #443.
- Lists are indexed by `index` in linear time (`LexLeanRuntime.listIndex`, structural recursion).
- The models example decodes int8 weights from content-addressed bytes into `List (List Int)` (§17.12, models). It has no array to put them in.
- PR #474 (draft) adds a sixth runtime module on the same line by the same procedure used here. Both changes move the language-1.2 identity, so whichever lands second regenerates the language-1.2 records once more.

## 3. Constraints taken as given

- SPEC §4.2: no raw Lean, no author axioms, no proof holes. Nothing here relaxes them.
- §17.12: the runtime of language 1.2 is LexLean source, every row pins its exact axioms, and the facts a proof uses are stated in that source, never assumed.
- §17.13: every construct has exactly one production row.
- AGENTS.md R3: register row, scenario, failing named test, then implementation. R4: nothing deferred.
- A claim about Lean belongs to Lean (AGENTS.md, rule 4). Constant-time indexing and in-place writes are Lean's.

## 4. Design

### 4.1 The type

`{"kind":"array","element":T}` with `T` in `Int8`, `Int16`, `Int32`, `Int64`, `UInt8`, `UInt16`, `UInt32`, `UInt64`. Anything else is `LLT4001`. An array has no literal and cannot be matched on: it is made from a list and read by position.

Generated Lean writes `_root_.Array (T)` and `_root_.LexLeanArrays.f`. Rooting the names means no document declaration can capture them and no new name had to be reserved, so no document that was accepted before is refused now. (A tooling module of this repository already declares a type named `Array`.)

### 4.2 Operations

| Operation | Type | Meaning | Lean |
| --- | --- | --- | --- |
| `length a` | `Nat` | number of elements | `Array.size` through `Lengthable` |
| `index a i` | `Option T` | element at `i`, none at or past the length | `a[i]?` through `Indexable` |
| `array_from_list xs` | `array T` | the array of the list's elements | `List.toArray` |
| `array_to_list a` | `List T` | the list of the array's elements | `Array.toList` |
| `array_set a i v` | `array T` | `a` with element `i` replaced; `a` itself at or past the length | `Array.setIfInBounds` |

`length` and `index` are the existing primitives. They resolve on an array through two instances declared in the new module, exactly as they resolve on lists and byte strings through instances in `Portable`.

### 4.3 Laws

For an array `a`, a list `xs`, offsets `i`, `j`, an element `v`:

| Law | Statement | Axioms |
| --- | --- | --- |
| `array_to_list_from_list` | `array_to_list (array_from_list xs) = xs` | none |
| `array_from_list_to_list` | `array_from_list (array_to_list a) = a` | none |
| `array_length` | `length a = length (array_to_list a)` | none |
| `array_index` | `index a i = index (array_to_list a) i` | `propext` |
| `array_index_beyond` | `length a <= i -> index a i = none` | `propext` |
| `array_set_length` | `length (array_set a i v) = length a` | `propext` |
| `array_set_index_same` | `i < length a -> index (array_set a i v) i = some v` | `propext` |
| `array_set_index_other` | `(i =? j) = false -> index (array_set a i v) j = index a j` | `propext` |
| `array_set_beyond` | `length a <= i -> array_set a i v = a` | `propext` |

Each is a theorem of `LexLeanRuntime.Arrays`, proved from the pinned Lean's own `Array` lemmas and exported once into the core notation (`tests/runtime-library/ARRAYS.md`). The law registry (`language/runtime-1.2/laws.toml`) gains an optional `module` on a row, so that a law can live in a module other than `Laws`, and a `law` proof imports the module that states its law. This matters downstream: `LexLeanRuntime.Laws` does not terminate when Lean emits C for it (#454), so a module that only applies array laws must not import it. The new module compiles with C output in 15.7 s (section 6.4).

### 4.4 How a loop is proved

The pattern, as `examples/arrays` uses it:

1. Write the loop over arrays (`dotLoop`) and the identical loop over lists (`dotIndex`).
2. State `array_index` and `array_length` for the element type as document theorems, each by one `law` proof.
3. `dotLoop a b i fuel acc = dotIndex (array_to_list a) (array_to_list b) i fuel acc` by induction on the fuel, each branch one `simplify` over the two definitions, the hypothesis and `array_index`.
4. From there the arrays are gone. The three list-indexing equations (`index (cons x xs) (i + 1) = index xs i`, `index (cons x xs) 0 = some x`, `index nil i = none`) hold by `reflexivity`, and the rest is induction on the list.

The whole proof of the dot product is 15 small theorems using `law`, `reflexivity`, `simplify`, `induction`, `cases` and `linear_arithmetic`. It was first written and checked directly in Lean 4.32.1 in the exact shapes the backend emits, then written in LexLean, where it verified without change to the proof plan.

### 4.5 Execution and mutation

Nothing in the meaning of an array depends on storage. The compiled behaviour is Lean's:

- `Array.size` and `a[i]?` read in constant time (`lean_array_get_size`, `lean_array_fget`).
- `Array.setIfInBounds` writes through `Array.set`, implemented by `lean_array_fset`, which writes in place when the array object is referenced once (`lean_is_exclusive`) and copies it first otherwise (`src/include/lean/lean.h` at the pinned commit).

This is recorded as a `some-true` row, `LEAN-ARRAY-4-32-1`, and is not a LexLean claim. A document gets the in-place write by threading the array through the loop and not keeping the old one. One measurement, not a claim (pinned Lean, `lean --run`, a loaded Windows machine, one run): `n` writes to an array of `n` int8 values took 7, 9 and 21 ms for `n` = 20,000, 40,000 and 80,000 when the array was passed along, and 238, 1,323 and 3,614 ms when each old array was kept alive in a second array. The first grows linearly, the second quadratically.

Prism builds an application by compiling the generated Lean to C, so it gets this behaviour directly.

### 4.6 Production

`type.array`, `primitive.array_from_list`, `primitive.array_to_list` and `primitive.array_set` are `formal-only` in `language/production-1.2.toml`. A production root whose closure reaches one is refused with `LLT4005` and a message that names the array. `length` and `index` keep their `runtime` rows; on an array they are refused through the array's type. Every lowering and certificate path treats the array type as it treats a proposition: it cannot arrive there, and if it did the path returns an error, never a default.

This is the state text components (`declaration.tokenizer` and the three beside it) are in today. It is a disposition in the closed registry, hashed into the identity, and not a switch.

## 5. What was changed

| Area | Change |
| --- | --- |
| IR (`ir/semantic.rs`) | `SemanticType::Array`, three `SemanticPrimitive` variants, element check, typing rules, language-1.1 refusal |
| Lean backend | type and primitive spelling, import of `LexLeanRuntime.Arrays`, per-law module import |
| Runtime | `Arrays.lex.tex` (15 declarations), registry, title entry, nine law rows |
| Production | four registry rows; eligibility keys; an error arm in every lowering and certificate match |
| Schemas | `array` type and three operations in the module and snapshot schemas, version 2 |
| Identity | `semantic_ir` 16 to 17, `lean_backend` 22 to 23; the language-1.2 identity moves |
| Model | `SM-39`, authority `LEAN-ARRAY-4-32-1`, oracle overlap table version 5 |
| Tests | `conformance_sm_39`, five negative fixtures, `examples/arrays` |

Languages 1.0 and 1.1 are untouched. No existing language-1.2 document changes meaning or is refused: the type and the primitives are new names, and the only changed rule (`length` and `index` accept an array) applies to a type no existing document could write.

## 6. Evidence

### 6.1 The example verifies

`lexlean verify` in `examples/arrays`, pinned Lean 4.32.1, Windows x86-64, 2026-11-12:

```text
verified 1 module; attestation edd3c119ebfd7583b2d9a85e4015f0bae6d473929b5a57d11be800f4a9da116b
```

The attestation identity is that of this host's run; the committed records are the Linux x86-64 ones the gate compares.

### 6.2 Observed axioms

From the attestation of that run, declared and observed sets equal for every declaration:

| Declaration | Observed axioms |
| --- | --- |
| `dot_equals_list` (the acceptance theorem) | `Quot.sound`, `propext` |
| `loop_reads_list`, `shift`, `index_sums` | `Quot.sound`, `propext` |
| `array_index` | `propext` |
| `array_length`, `index_shift`, `index_head`, `index_empty`, `length_empty`, `length_step` | none |
| `rectify_keeps_length`, `loop_keeps_length`, `set_keeps_length` | `propext` |
| `dot_sample`, `rectify_sample` (closed instances by `decide`) | `Quot.sound`, `propext`; `propext` |

`Quot.sound` enters through `to_int` and the `linear_arithmetic` lemmas, as it does in every existing module that uses them, not through arrays. No declaration depends on `Classical.choice`.

### 6.3 The runtime module

15 declarations. The export of the authored Lean and the export of the Lean the compiler generates from the committed source are equal in all 15 rows, as trees of kernel terms (`tests/runtime-library/ARRAYS.md`).

### 6.4 C output

The generated `LexLeanRuntime/Arrays.lean` compiled with `-c` in 15.7 s (1.46 MB of C); `Portable.lean`, as a control, in 14.9 s. `Laws` is the module that does not terminate there (#454).

### 6.5 Refusals and planted defects

`conformance_sm_39` and five negative fixtures hold the refusals: an element that is not a fixed-width integer, a write of another element type, a conversion of the wrong container, a pattern match on an array, an unregistered or mistyped law, a language-1.1 use, and a production root over an array. Lean refuses, with `LLV7002`, a copy of the example whose loop advances by two and a law applied to a statement it does not prove. The planted defects that were run against the gate are in `VERIFICATION.md`.

## 7. Limits

- An array of `UInt64` or `Int64` is an array of boxed values in the pinned Lean's runtime; the smaller widths are stored in the element word. This is our reading of the runtime's object layout, not something measured here.
- `index` returns an `Option`, so a loop matches on it at each read. Whether the pinned Lean's compiler removes that allocation was not measured.
- Mathematical `Int` has no `+` in language 1.2 (`a + b` is written `subtract a (negate b)`). The example defines `plus` once. This is an existing gap, not introduced here.
- Artifacts still decode to lists. An engine converts with `array_from_list` after decoding.
- No claim is made about speed.

## 8. Not in this change

- A Rust realization of arrays (D3).
- A readable `(Array T)` notation (D4).
- Array-typed artifacts.
- Slicing, appending, folding or iteration primitives on arrays. Each is a definition in a document today.
- Anything on `Bytes` (#443).

## 9. Decisions that belong to the owners

### D1. Total write or `Option`

#443's `update` on `Bytes` returns `Option`: none past the end. An `Option` result makes every law about a write conditional and makes every write allocate a result to take apart. A total write that is the identity out of range has the precedent of `map_remove` and `set_remove`, and the checked form is one conditional in the document. Recommendation: keep `array_set` total. If the owners prefer one shape for both, the cheaper alignment is to make `Bytes` follow, since no released document uses either.

### D2. Element types

Lean's `Array` is generic, and so are the nine laws. Restricting the element to fixed-width integers keeps an array a flat sequence of machine integers, which is what a production realization would need, and it is the only restriction that can be lifted later without refusing an accepted document. Recommendation: keep the restriction.

### D3. Production

A `rust-std` realization would be a shared buffer with a uniqueness check on write, as #443 does for `Bytes`, with its preservation lemmas. That is a change of the size of #443. Recommendation: land arrays formal-only now, and track the Rust realization as its own item after #443 settles the pattern.

### D4. Notation

`(Array T)` would be the natural readable form. Adding it changes how a document type named `Array` must be written. Recommendation: decide it together with the next notation change; the generic form works today.

### D5. Register ID

`SM-39` is the next free number on this line. Recommendation: keep it and renumber the unlanded `v0.5` draft at the forward merge.

### D6. Packed storage

A byte-packed array of int8 is a `Bytes` read through a conversion. Once #443 lands, a document can build that itself. Recommendation: no second representation in the language.
