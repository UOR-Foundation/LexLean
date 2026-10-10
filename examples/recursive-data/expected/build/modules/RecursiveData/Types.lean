module
public import Init
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace RecursiveData.Types

public inductive Tree (Item : Type) where
  | leaf
  | node (_ : Tree (Item)) (_ : Item) (_ : Tree (Item))

public inductive Rose (Item : Type) where
  | node (_ : Item) (_ : List (Rose (Item)))

mutual
public inductive Expr where
  | literal (_ : Nat)
  | plus (_ : Expr) (_ : Expr)
  | block (_ : List (Stmt)) (_ : Expr)

public inductive Stmt where
  | assign (_ : String) (_ : Expr)
  | sequence (_ : Stmt) (_ : Stmt)
end

mutual
public inductive Node (Item : Type) where
  | node (_ : Item) (_ : Branches (Item))

public inductive Branches (Item : Type) where
  | none
  | more (_ : Node (Item)) (_ : Branches (Item))
end

public inductive Outcome (Problem : Type) (Item : Type) where
  | failure (_ : Problem)
  | success (_ : Item)

public structure Located (Item : Type) where
  value : Item
  position : (Prod (Nat) (Nat))

end RecursiveData.Types
