module

public import Init.Data.BitVec

/-! The host module of the BLAKE3 G acceptance demo. It holds the one lemma that
the imported libraries (Lean core, Std, Mathlib at the pin) do not state:
rotating left undoes rotating right by the same amount. -/

public section

theorem Blake3GHost.rotateLeft_rotateRight (x : BitVec 32) (n : Nat) :
    (x.rotateRight n).rotateLeft n = x := by
  apply BitVec.eq_of_getLsbD_eq
  intro i hi
  simp only [BitVec.getLsbD_rotateLeft, BitVec.getLsbD_rotateRight]
  have hn : n % 32 < 32 := Nat.mod_lt _ (by decide)
  generalize n % 32 = r at hn ⊢
  by_cases h1 : i < r
  · have h3 : 32 - r + i < 32 := by omega
    have h5 : ¬ (32 - r + i < 32 - r) := by omega
    simp [h1, h3, h5]
  · have h2 : i - r < 32 - r := by omega
    have h6 : r + (i - r) = i := by omega
    simp [h1, hi, h2, h6]

end
