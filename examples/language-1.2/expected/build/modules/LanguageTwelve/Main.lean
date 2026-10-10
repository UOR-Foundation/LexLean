module
public import Init
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LanguageTwelve.Main

@[expose] public def doubledSuccessor (n : Nat) : Nat := (let doubled : Nat := (n + n); (doubled + 1))

public theorem doubled_successor_two : (doubledSuccessor (2) = 5) := by
  rfl

public theorem let_statement_unfolds : (let three : Nat := 3; ((three + three) = 6)) := by
  rfl

end LanguageTwelve.Main
