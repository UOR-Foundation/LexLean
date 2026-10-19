
namespace Tre

theorem rtE_lit (me ps S D l) : rtE me ps S D (Expr.lit l) = some (Expr.construct (Ctor.err S D) [Expr.lit l]) := by
  rw [rtE] <;> intros <;> simp_all
theorem rtE_call_other (me ps S D m args) (h : (m == me) = false) :
    rtE me ps S D (Expr.call (Callee.function m) args false) =
      some (Expr.construct (Ctor.err S D) [Expr.call (Callee.function m) args false]) := by
  rw [rtE]; simp [h]
theorem rtE_call_prop (me ps S D cal args) :
    rtE me ps S D (Expr.call cal args true) =
      some (Expr.construct (Ctor.err S D) [Expr.call cal args true]) := by
  rw [rtE] <;> intros <;> simp_all
theorem rtE_cond (me ps S D cd t e) : rtE me ps S D (Expr.cond cd t e) =
    (match rtB me ps S D t, rtB me ps S D e with
      | some t', some e' => some (Expr.cond cd t' e')
      | _, _ => none) := by
  rw [rtE]
theorem rtE_block (me ps S D b) : rtE me ps S D (Expr.block b) =
    (match rtB me ps S D b with | some b' => some (Expr.block b') | none => none) := by
  rw [rtE]
theorem rtE_match (me ps S D s arms) : rtE me ps S D (Expr.matchOn s arms) =
    (match rtArms me ps S D arms with | some arms' => some (Expr.matchOn s arms') | none => none) := by
  rw [rtE]
theorem rtE_self (me ps S D args) :
    rtE me ps S D (Expr.call (Callee.function me) args false) =
      (match carried ps args with
        | none => none
        | some xs => match pack xs with
          | none => none
          | some st => some (Expr.construct (Ctor.ok S D) [st])) := by
  rw [rtE]; simp

end Tre
