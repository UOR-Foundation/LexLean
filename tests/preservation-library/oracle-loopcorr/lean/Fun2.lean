
namespace LexLeanPreservation.Rust

/-- A function whose crate body is the loop form of an ordinary recursive-form body. -/
def FunLoop (p : LexLeanTarget.TargetSyntax.Program) (c : RCrate) (A : Flags) (f : Nat) : Prop :=
  ∃ (fn : LexLeanTarget.TargetSyntax.Function) (rc : List LexLeanTarget.RustSyntax.Parameter)
    (rty : LexLeanTarget.RustSyntax.RustType) (lets0 : List LexLeanTarget.RustSyntax.Let)
    (tail0 : LexLeanTarget.RustSyntax.Expr) (Γ : Ctx) (ps : List Tre.Pos)
    (S D : LexLeanTarget.RustSyntax.RustType) (hn : Nat) (pns : List Nat)
    (pl : List LexLeanTarget.RustSyntax.Parameter) (bodyL : LexLeanTarget.RustSyntax.Block),
    LexLeanRuntime.index p.functions f = some fn ∧
    LexLeanTarget.RustSemantics.findFunction c.items (fnIdent f) = some (pl, rty, bodyL) ∧
    Tre.loopOf f ps S D hn pns rc (LexLeanTarget.RustSyntax.Block.mk lets0 tail0) = some (pl, bodyL) ∧
    Tre.loopCheckOf f ps S D hn pns rc (LexLeanTarget.RustSyntax.Block.mk lets0 tail0) = true ∧
    paramCtx [] fn.parameters fn.types (LexLeanTarget.RustSemantics.patternsOf rc) = some Γ ∧
    Corr p c A Γ (LexLeanTarget.RustSemantics.fallibleType rty)
      (Jd.b (LexLeanTarget.RustSemantics.fallibleType rty) fn.body fn.result lets0 tail0)

end LexLeanPreservation.Rust
