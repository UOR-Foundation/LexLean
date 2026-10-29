import Mathlib.Analysis.Real.Sqrt

theorem probeC (x y : Real) (hx : 0 ≤ x) (h : x < y) : Real.sqrt x < Real.sqrt y :=
  Real.sqrt_lt_sqrt hx h
