
namespace Tre

mutual
def occE : Expr → List Ident
  | Expr.lit _ => []
  | Expr.move i => [i]
  | Expr.clone i => [i]
  | Expr.copy i => [i]
  | Expr.deref i => [i]
  | Expr.not e => occE e
  | Expr.unbox i => [i]
  | Expr.box e => occE e
  | Expr.call _ args _ => occL args
  | Expr.apply h args _ => h :: occL args
  | Expr.construct _ args => occL args
  | Expr.pair a b => occE a ++ occE b
  | Expr.cond c t e => occE c ++ occB t ++ occB e
  | Expr.matchOn s arms => occE s ++ occArms arms
  | Expr.block b => occB b
  | Expr.uncons i => [i]
  | Expr.isZero i => [i]
  | Expr.nonZero i => [i]
  | Expr.predecessor i => [i]
  | Expr.widen e => occE e
  | Expr.succeed e => occE e
  | Expr.loop s b => s :: occB b
def occL : List Expr → List Ident
  | [] => []
  | e :: es => occE e ++ occL es
def occB : Block → List Ident
  | Block.mk lets tail => occLets lets ++ occE tail
def occLets : List Let → List Ident
  | [] => []
  | Let.mk _ _ e :: rest => occE e ++ occLets rest
def occArms : List Arm → List Ident
  | [] => []
  | Arm.mk _ b :: rest => occB b ++ occArms rest
end

def Agree (S : List Ident) (E E' : Env) : Prop := ∀ i ∈ S, lookup E i = lookup E' i

theorem Agree.mono {S T E E'} (h : Agree S E E') (hs : ∀ i ∈ T, i ∈ S) : Agree T E E' :=
  fun i hi => h i (hs i hi)

theorem Agree.ext {S E E'} (ext : Env) (h : Agree S E E') : Agree S (ext ++ E) (ext ++ E') := by
  intro i hi
  induction ext with
  | nil => exact h i hi
  | cons e rest ih =>
    obtain ⟨a, v⟩ := e
    simp only [List.cons_append, lookup, ih]

theorem Agree.cons {S E E'} (a : Ident) (v : V) (h : Agree S E E') :
    Agree S ((a, v) :: E) ((a, v) :: E') := by
  intro i hi
  simp only [lookup, h i hi]

theorem Agree.single {i E E'} (h : lookup E i = lookup E' i) : Agree [i] E E' := by
  intro j hj
  simp at hj
  subst hj
  exact h

end Tre
