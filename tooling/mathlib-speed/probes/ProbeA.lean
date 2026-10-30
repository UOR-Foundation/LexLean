import Mathlib.Order.Defs.PartialOrder

theorem probeA {α : Type} [PartialOrder α] (a b : α) (h1 : a ≤ b) (h2 : b ≤ a) : a = b :=
  le_antisymm h1 h2
