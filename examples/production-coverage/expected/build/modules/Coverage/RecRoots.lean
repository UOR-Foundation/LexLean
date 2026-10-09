module
public import Init
public import Coverage.Recur
public import Coverage.Syntax
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Coverage.RecRoots

@[expose] public def roseTotal (n : Nat) : Nat := Coverage.Recur.roseSize (Nat) (Coverage.Syntax.Rose.node (n) ((Coverage.Syntax.Rose.node (1) (([] : List (Coverage.Syntax.Rose (Nat)))) :: ([] : List (Coverage.Syntax.Rose (Nat))))))

@[expose] public def syntaxTotal (e : Coverage.Syntax.Expr) : Nat := Coverage.Recur.exprSize (e)

@[expose] public def wellFounded (a : Nat) (b : Nat) : (Prod (Nat) ((Prod (Nat) (Nat)))) := (Coverage.Recur.countdown (a) (0), (Coverage.Recur.reduce (a) (b), Coverage.Recur.search (a) (0) (b)))

@[expose] public def rewrite (t : Coverage.Syntax.Term) : (Prod (Coverage.Syntax.Term) (Coverage.Syntax.Term)) := (Coverage.Recur.reassociate (t), Coverage.Recur.prune (t))

end Coverage.RecRoots
