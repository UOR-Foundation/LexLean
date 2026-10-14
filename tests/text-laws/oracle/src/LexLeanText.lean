/-
  LexLean text laws --- the tokenizer's universal laws (issue #30, slice B).

  Authored once here, in a scratch directory OUTSIDE the release tree. This
  file is the hand-written Lean that the one-time exporter converts to native
  `lexlean/core-module/2` data; after that conversion no independently
  authored Lean implementation of the text laws exists in the release tree
  (the permanent R3 condition this slice adds).

  Proved properly: no `sorry`, no `admit`, no new axiom. Checked by the pinned
  Lean, `leanprover/lean4:v4.32.1`.
-/
namespace LexLeanText

/-- A token identifier is a `uint32` (SPEC LM-01): `tokens are uint32`. -/
abbrev Token := UInt32

/-- A byte string, as the closed schemes carry it. -/
abbrev Bytes := ByteArray

/-- The byte-level vocabulary size: `the single bytes are the ids 0..255,
present always as the byte fallback` (SPEC LM-01). A vocabulary piece takes
ids from `256` in artifact order; the laws below use only this bound. -/
def byteVocab : Nat := 256

/-- Every byte's id is below the vocabulary size. -/
theorem byte_lt_byteVocab (x : UInt8) : x.toNat < byteVocab := by
  simpa [byteVocab] using UInt8.toNat_lt x

/-- **`encode`** (SPEC LM-01): each byte is its own token id. -/
def encode (b : Bytes) : List Token := b.data.toList.map (fun x => x.toUInt32)

/-- The closed test the scheme's `in_range` law states: a token indexes the
vocabulary exactly when its id is below the vocabulary size. -/
def InVocab (t : Token) : Prop := t.toNat < byteVocab

/-- **`decode`** (SPEC LM-01): the ids `0..255` come back as their bytes; a
token outside the vocabulary is refused (SPEC LM-03 keeps invalid input out). -/
def decode (t : List Token) : Option Bytes :=
  if ∀ x ∈ t, x.toNat < byteVocab then
    some (ByteArray.mk (t.map (fun x => UInt8.ofNat x.toNat)).toArray)
  else none

/-- **`roundtrip`** (SPEC LM-02): for every byte string `b`, `decode (encode b)`
is `some b`. -/
theorem roundtrip (b : Bytes) : decode (encode b) = some b := by
  unfold decode encode
  rw [if_pos]
  · have hmap :
        (b.data.toList.map (fun x => x.toUInt32)).map (fun x => UInt8.ofNat x.toNat)
          = b.data.toList := by
      rw [List.map_map]
      have hfun : ((fun x => UInt8.ofNat x.toNat) ∘ fun x => x.toUInt32) = fun x => x := by
        funext x
        simp only [Function.comp_apply, UInt8.toNat_toUInt32, UInt8.ofNat_toNat]
      rw [hfun]
      simp
    rw [hmap]
  · intro x hx
    obtain ⟨y, _, hy⟩ := List.mem_map.mp hx
    rw [← hy, UInt8.toNat_toUInt32]
    exact byte_lt_byteVocab y

/-- **`in_range`** (SPEC LM-02): every token an `encode` produces indexes the
vocabulary, so the round trip also fixes the token boundaries. -/
theorem in_range (b : Bytes) (t : Token) (h : t ∈ encode b) : InVocab t := by
  unfold InVocab byteVocab
  simp only [encode] at h
  obtain ⟨x, _, hx⟩ := List.mem_map.mp h
  rw [← hx, UInt8.toNat_toUInt32]
  exact byte_lt_byteVocab x

/-- **`utf8_encode`** (SPEC LM-01): the bytes of a string. -/
def utf8Encode (s : String) : Bytes := s.toUTF8

/-- **`utf8_decode`** (SPEC LM-01): the string of valid UTF-8 bytes, or none. -/
def utf8Decode (b : Bytes) : Option String := String.fromUTF8? b

/-- The UTF-8 law the byte-level scheme needs: `utf8_decode (utf8_encode s)` is
`some s` for every string. -/
theorem utf8_roundtrip (s : String) : utf8Decode (utf8Encode s) = some s := by
  unfold utf8Decode utf8Encode String.fromUTF8? String.fromUTF8
  rw [String.toUTF8_eq_toByteArray]
  rw [dif_pos s.isValidUTF8]

/-- **`encodeText`** (SPEC LM-01): a string to tokens through `utf8_encode`. -/
def encodeText (s : String) : List Token := encode (utf8Encode s)

/-- **`decodeText`** (SPEC LM-01): tokens to a string through `utf8_decode`. -/
def decodeText (t : List Token) : Option String := (decode t).bind utf8Decode

/-- **`text_roundtrip`** (SPEC LM-02): the roundtrip over `string`. -/
theorem text_roundtrip (s : String) : decodeText (encodeText s) = some s := by
  unfold decodeText encodeText
  simp only [roundtrip, Option.bind_some, utf8_roundtrip]

end LexLeanText
