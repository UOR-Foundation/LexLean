module
public import Init
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace HigherOrder.Combinators

@[expose] public def mapList (Input : Type) (Output : Type) : (transform : ((Input) -> (Output))) -> (values : List (Input)) -> List (Output)
  | _transform, List.nil => ([] : List (Output))
  | transform, List.cons head tail => ((transform (head)) :: mapList (Input) (Output) (transform) (tail))

@[expose] public def foldList (Input : Type) (Output : Type) : (step : ((Output) -> (Input) -> (Output))) -> (initial : Output) -> (values : List (Input)) -> Output
  | _step, initial, List.nil => initial
  | step, initial, List.cons head tail => foldList (Input) (Output) (step) ((step (initial) (head))) (tail)

@[expose] public def compose (Input : Type) (Output : Type) (Final : Type) (outer : ((Output) -> (Final))) (inner : ((Input) -> (Output))) : ((Input) -> (Final)) := (fun (value : Input) => (outer ((inner (value)))))

@[expose] public def increment (value : Nat) : Nat := (value + 1)

public inductive Arith where
  | literal (_ : Nat)
  | plus (_ : Arith) (_ : Arith)
  | twice (_ : Arith)

public structure Visitor where
  onLiteral : ((Nat) -> (Nat))
  onPlus : ((Nat) -> (Nat) -> (Nat))
  onTwice : ((Nat) -> (Nat))

@[expose] public def visit : (visitor : Visitor) -> (expression : Arith) -> Nat
  | visitor, Arith.literal value => ((visitor).onLiteral (value))
  | visitor, Arith.plus left right => ((visitor).onPlus (visit (visitor) (left)) (visit (visitor) (right)))
  | visitor, Arith.twice inner => ((visitor).onTwice (visit (visitor) (inner)))

@[expose] public def evaluate : (expression : Arith) -> Nat
  | Arith.literal value => value
  | Arith.plus left right => (evaluate (left) + evaluate (right))
  | Arith.twice inner => (let half : Nat := evaluate (inner); (half + half))

end HigherOrder.Combinators
