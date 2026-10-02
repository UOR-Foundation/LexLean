module
public import Init
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
namespace Coverage.Syntax

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

public inductive Term where
  | literal (_ : Nat)
  | plus (_ : Term) (_ : Term)

end Coverage.Syntax
