
namespace Tre

theorem rtE_other (me ps S D) (x : Expr)
    (h1 : ∀ m args, x ≠ Expr.call (Callee.function m) args false)
    (h2 : ∀ c t e, x ≠ Expr.cond c t e)
    (h3 : ∀ s arms, x ≠ Expr.matchOn s arms)
    (h4 : ∀ b, x ≠ Expr.block b) :
    rtE me ps S D x = some (Expr.construct (Ctor.err S D) [x]) := by
  rw [rtE]
  · intro m a h; exact h1 m a h
  · intro c t e h; exact h2 c t e h
  · intro s a h; exact h3 s a h
  · intro b h; exact h4 b h

end Tre
