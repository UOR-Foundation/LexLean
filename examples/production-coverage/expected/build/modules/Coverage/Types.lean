module
public import Init
public import LexLeanRuntime.Portable
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Coverage.Types

public inductive Shape where
  | circle (_ : Nat)
  | rect (_ : Nat) (_ : Nat)
  | empty

public inductive Tree where
  | leaf
  | node (_ : Tree) (_ : Nat) (_ : Tree)

public structure Point where
  x : Nat
  y : Nat

public structure Box (Item : Type) where
  content : Item
  count : Nat

public class Weights where
  base : Nat
  scale : Nat

public instance (priority := 1000) defaultWeights : Weights where
  base := 2
  scale := 3

@[expose] public def area (shape : Shape) : Nat := (match shape with | Shape.circle r => (LexLeanRuntime.multiply ((LexLeanRuntime.multiply (r) (r) : Nat)) (3) : Nat) | Shape.rect w h => (LexLeanRuntime.multiply (w) (h) : Nat) | Shape.empty => 0)

@[expose] public def treeSum : (tree : Tree) -> Nat
  | Tree.leaf => 0
  | Tree.node l v r => ((treeSum (l) + v) + treeSum (r))

@[expose] public def unbox (Item : Type) (box : Box (Item)) : Item := (box).content

@[expose] public def twice (step : ((Nat) -> (Nat))) (value : Nat) : Nat := (step ((step (value))))

@[expose] public def mapList (A : Type) (B : Type) : (transform : ((A) -> (B))) -> (values : List (A)) -> List (B)
  | _transform, List.nil => ([] : List (B))
  | transform, List.cons head tail => ((transform (head)) :: mapList (A) (B) (transform) (tail))

mutual
@[expose] public def isEven : (n : Nat) -> Bool
  | Nat.zero => true
  | Nat.succ k => isOdd (k)
termination_by structural n => n

@[expose] public def isOdd : (n : Nat) -> Bool
  | Nat.zero => false
  | Nat.succ k => isEven (k)
termination_by structural n => n
end

end Coverage.Types
