/-
  The Lean module behind `tests/glossary/extraction.json` (GL-19, GL-20). Each
  declaration exists to exercise one rule of SPEC.md §13.12:

    Fix.polyId          Sort-polymorphic type            LLT4101
    Fix.withStrict      strict-implicit binder           LLT4102
    Fix.projAxiom       primitive projection             LLT4103
    Fix.strLit          string literal                   LLT4103
    Fix.proofEq         Eq at Sort 0                     LLT4104
    Fix.add_comm'       name outside the grammar         LLT4105
    Fix.usesPoly        mentions Fix.polyId              LLT4106 (root LLT4101)

  and the rest exercise what is translated: a class with an instance argument,
  theorems over it, a recursive inductive type with an eliminator, a
  Prop-valued predicate of each arity, and numerals.

  Regenerate the extraction with `tests/glossary/regenerate.sh` after editing.
-/
import Lean
open Lean

universe u

namespace Fix

def polyId {α : Sort u} (a : α) : α := a

def withStrict (_f : ⦃n : Nat⦄ → n = n → Prop) : Prop := True

run_cmd Elab.Command.liftCoreM do
  let prod := mkApp2 (mkConst ``Prod [0, 0]) (mkConst ``Nat) (mkConst ``Nat)
  let body := mkApp3 (mkConst ``Eq [1]) (mkConst ``Nat) (mkProj ``Prod 0 (mkBVar 0)) (mkRawNatLit 0)
  addDecl (.axiomDecl { name := `Fix.projAxiom, levelParams := [], type := mkForall `x .default prod body, isUnsafe := false })

axiom strLit : "abc".length = 3

theorem proofEq (p : Prop) (h₁ h₂ : p) : h₁ = h₂ := rfl

theorem add_comm' (a b : Nat) : a + b = b + a := Nat.add_comm a b

theorem usesPoly (a : Nat) : polyId a = a := rfl

class Scale (α : Type u) where
  scale : Nat → α → α

def twice {α : Type u} [Scale α] (a : α) : α := Scale.scale 2 (Scale.scale 1 a)

theorem scale_zero {α : Type u} [Scale α] (a : α) (h : ∀ x : α, Scale.scale 0 x = x) :
    Scale.scale 0 a = a := h a

theorem twice_def {α : Type u} [Scale α] (a : α) :
    twice a = Scale.scale 2 (Scale.scale 1 a) := rfl

inductive Tree where
  | leaf : Tree
  | node (left right : Tree) (label : Nat) : Tree

def Tree.size : Tree → Nat
  | .leaf => 0
  | .node l r _ => l.size + r.size + 1

theorem Tree.size_leaf : Tree.leaf.size = 0 := rfl

def Small (n : Nat) : Prop := n < 10

def Between (a b : Nat) : Prop := a < b

def Always : Prop := True

theorem small_two : Small 2 := by unfold Small; decide

theorem small_two_between : Between 2 3 := by unfold Between; decide

end Fix
