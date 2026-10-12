module
public meta import Lean
set_option autoImplicit false
set_option linter.unusedVariables false
universe u v

mutual
public meta partial def Traversal.code (c : (Lean.Compiler.LCNF.Code Lean.Compiler.LCNF.Purity.pure)) : Nat :=
  (match c with | .«let» decl k => (Nat.add 1 (Traversal.code k)) | .«fun» decl k => (Nat.add (Traversal.funDecl decl) (Traversal.code k)) | .jp decl k => (Nat.add (Traversal.funDecl decl) (Traversal.code k)) | .jmp fvarId args => 0 | .cases cases => (Traversal.cases cases) | .«return» fvarId => 0 | .unreach type => 0)

public meta partial def Traversal.funDecl (d : (Lean.Compiler.LCNF.FunDecl Lean.Compiler.LCNF.Purity.pure)) : Nat :=
  (match d with | .mk fvarId binderName params type value => (Traversal.code value))

public meta partial def Traversal.cases (cs : (Lean.Compiler.LCNF.Cases Lean.Compiler.LCNF.Purity.pure)) : Nat :=
  (match cs with | .mk typeName resultType discr alts => (List.foldl (α := Nat) (β := (Lean.Compiler.LCNF.Alt Lean.Compiler.LCNF.Purity.pure)) (fun (acc : Nat) (x : (Lean.Compiler.LCNF.Alt Lean.Compiler.LCNF.Purity.pure)) => (Nat.add acc (Traversal.alt x))) 0 (Array.toList (α := (Lean.Compiler.LCNF.Alt Lean.Compiler.LCNF.Purity.pure)) alts)))

public meta partial def Traversal.alt (a : (Lean.Compiler.LCNF.Alt Lean.Compiler.LCNF.Purity.pure)) : Nat :=
  (match a with | .alt ctorName params code => (Traversal.code code) | .default code => (Traversal.code code))
end
