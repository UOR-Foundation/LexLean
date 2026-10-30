import Mathlib.Algebra.BigOperators.Group.Finset.Sigma

theorem probeD (s t : Finset Nat) (f : Nat → Nat → Nat) :
    (∑ x ∈ s, ∑ y ∈ t, f x y) = ∑ y ∈ t, ∑ x ∈ s, f x y := Finset.sum_comm
