import Mathlib.Data.Nat.Prime.Defs

theorem probeB (p : Nat) (h : Nat.Prime p) : 2 ≤ p := h.two_le
