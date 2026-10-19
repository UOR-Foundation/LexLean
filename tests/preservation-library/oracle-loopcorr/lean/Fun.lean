
namespace Tre

theorem aligned_rc_len : ∀ {ps rc lp fs os}, Aligned ps rc lp fs os → rc.length = ps.length := by
  intro ps rc lp fs os h
  induction h with
  | nil => rfl
  | changed _ ih => simp [ih]
  | kept _ ih => simp [ih]
  | dropped _ ih => simp [ih]

theorem lets_ev {c : Crate} {renv Y : Env} {lets : List Let} {tail : Expr} {ro : ROutcome}
    (h : LexLeanPreservation.Rust.LetsOK c renv lets Y) :
    LexLeanPreservation.Rust.RC c Y tail ro → EvB c renv (Block.mk lets tail) ro := by
  induction h with
  | nil => intro hr; exact evBlockNil.2 hr
  | cons hb hp _ ih =>
    intro hr; exact evBlockCons_val hb hp (ih hr)

theorem lets_raise {c : Crate} {renv : Env} {lets : List Let} {tail : Expr} {hh : ROutcome}
    (h : LexLeanPreservation.Rust.LetsRaise c renv lets hh) :
    Halt hh → EvB c renv (Block.mk lets tail) hh := by
  induction h with
  | here hb _ => intro hhalt; exact evBlockCons_halt hb hhalt
  | later hb hp _ ih => intro hhalt; exact evBlockCons_val hb hp (ih hhalt)

theorem realizes_stuck {fl : Bool} {o : LexLeanPreservation.Obs} {r : ROutcome}
    (h : LexLeanPreservation.Rust.RealizesM fl fl o r) : r ≠ ROutcome.stuck := by
  intro e; subst e
  cases fl <;> cases o <;> simp [LexLeanPreservation.Rust.RealizesM, LexLeanPreservation.Rust.Realizes,
    LexLeanPreservation.Rust.RealizesF] at h

theorem realizesFn_of {F : Bool} {o : LexLeanPreservation.Obs} {r : ROutcome}
    (h : LexLeanPreservation.Rust.RealizesM F F o r) :
    LexLeanPreservation.Rust.RealizesFn F o (finished r F) := by
  cases F <;> cases o <;> cases r <;> simp [LexLeanPreservation.Rust.RealizesM,
    LexLeanPreservation.Rust.Realizes, LexLeanPreservation.Rust.RealizesF,
    LexLeanPreservation.Rust.RealizesFn, finished] at h ⊢ <;> first | exact h | skip

end Tre
