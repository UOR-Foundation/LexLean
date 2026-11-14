module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Order
public import LexLeanBoot.LexiconLse
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.LseScopes

public inductive SWork where
  | SExpr (_ : LexLeanBoot.LexiconLse.Lse)
  | SBind (_ : ByteArray)
  | STrunc (_ : Nat)
  | SUni (_ : LexLeanBoot.LexiconLse.Universe)

public structure Sc where
  work : List (SWork)
  scope : List (ByteArray)
  depth : Nat
  error : Option (ByteArray)

public structure Rc where
  stack : List (LexLeanBoot.LexiconLse.Lse)
  found : List ((Prod (ByteArray) (ByteArray)))

@[expose] public def lit (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def cat (left : ByteArray) (right : ByteArray) : ByteArray := (LexLeanRuntime.append (left) (right) : ByteArray)

@[expose] public def member (items : List (ByteArray)) (wanted : ByteArray) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (item : ByteArray) => (if found then true else (LexLeanRuntime.equal (item) (wanted) : Bool)))) (false) (items) : Bool)

@[expose] public def dropFront (items : List (ByteArray)) (count : Nat) : List (ByteArray) := ((LexLeanCollections.iterateUntil ((fun (rest : List (ByteArray)) => (match rest with | List.nil => Option.none | List.cons _ tail => Option.some (tail)))) (count) (items) : (Prod (List (ByteArray)) (Bool)))).1

@[expose] public def scGo (st : Sc) (work : List (SWork)) : Option (Sc) := Option.some (({ work := work, scope := (st).scope, depth := (st).depth, error := Option.none } : Sc))

@[expose] public def scFail (st : Sc) (message : ByteArray) : Option (Sc) := Option.some (({ work := (st).work, scope := (st).scope, depth := (st).depth, error := Option.some (message) } : Sc))

@[expose] public def uniStep (st : Sc) (rest : List (SWork)) (universes : List (ByteArray)) (u : LexLeanBoot.LexiconLse.Universe) : Option (Sc) := (match u with | LexLeanBoot.LexiconLse.Universe.UNum _ => scGo (st) (rest) | LexLeanBoot.LexiconLse.Universe.UVar name => (if member (universes) (name) then scGo (st) (rest) else scFail (st) (cat (cat (lit ("universe variable `")) (name)) (lit ("` is not declared")))) | LexLeanBoot.LexiconLse.Universe.USucc inner => scGo (st) ((SWork.SUni (inner) :: rest)) | LexLeanBoot.LexiconLse.Universe.UMax items => scGo (st) ((SWork.SUni (items) :: rest)) | LexLeanBoot.LexiconLse.Universe.UIMax a b => scGo (st) ((SWork.SUni (a) :: (SWork.SUni (b) :: rest))) | LexLeanBoot.LexiconLse.Universe.UNil => scGo (st) (rest) | LexLeanBoot.LexiconLse.Universe.UItem head tail => scGo (st) ((SWork.SUni (head) :: (SWork.SUni (tail) :: rest))))

@[expose] public def exprStep (st : Sc) (rest : List (SWork)) (expr : LexLeanBoot.LexiconLse.Lse) : Option (Sc) := (match expr with | LexLeanBoot.LexiconLse.Lse.LSortProp => scGo (st) (rest) | LexLeanBoot.LexiconLse.Lse.LSortType u => scGo (st) ((SWork.SUni (u) :: rest)) | LexLeanBoot.LexiconLse.Lse.LConst _ _ us => scGo (st) ((SWork.SUni (us) :: rest)) | LexLeanBoot.LexiconLse.Lse.LLocal name => (if member ((st).scope) (name) then scGo (st) (rest) else scFail (st) (cat (cat (lit ("local `")) (name)) (lit ("` is not in scope")))) | LexLeanBoot.LexiconLse.Lse.LApp f args => scGo (st) ((SWork.SExpr (f) :: (SWork.SExpr (args) :: rest))) | LexLeanBoot.LexiconLse.Lse.LPi binders body => scGo (st) ((SWork.SExpr (binders) :: (SWork.SExpr (body) :: (SWork.STrunc ((st).depth) :: rest)))) | LexLeanBoot.LexiconLse.Lse.LLam binders body => scGo (st) ((SWork.SExpr (binders) :: (SWork.SExpr (body) :: (SWork.STrunc ((st).depth) :: rest)))) | LexLeanBoot.LexiconLse.Lse.LLet name ty value body => scGo (st) ((SWork.SExpr (ty) :: (SWork.SExpr (value) :: (SWork.SBind (name) :: (SWork.SExpr (body) :: (SWork.STrunc ((st).depth) :: rest)))))) | LexLeanBoot.LexiconLse.Lse.LNat _ => scGo (st) (rest) | LexLeanBoot.LexiconLse.Lse.LNil => scGo (st) (rest) | LexLeanBoot.LexiconLse.Lse.LArg head tail => scGo (st) ((SWork.SExpr (head) :: (SWork.SExpr (tail) :: rest))) | LexLeanBoot.LexiconLse.Lse.LBinder _ name ty more => scGo (st) ((SWork.SExpr (ty) :: (SWork.SBind (name) :: (SWork.SExpr (more) :: rest)))))

@[expose] public def scStep (universes : List (ByteArray)) (st : Sc) : Option (Sc) := (match (st).error with | Option.some _ => Option.none | Option.none => (match (st).work with | List.nil => Option.none | List.cons item rest => (match item with | SWork.SExpr expr => exprStep (st) (rest) (expr) | SWork.SBind name => Option.some (({ work := rest, scope := (name :: (st).scope), depth := ((st).depth + 1), error := Option.none } : Sc)) | SWork.STrunc wanted => Option.some (({ work := rest, scope := dropFront ((st).scope) ((LexLeanRuntime.subtract ((st).depth) (wanted) : Nat)), depth := wanted, error := Option.none } : Sc)) | SWork.SUni u => uniStep (st) (rest) (universes) (u))))

@[expose] public def checkScopes (expr : LexLeanBoot.LexiconLse.Lse) (universes : List (ByteArray)) (bound : Nat) : Option (ByteArray) := (((LexLeanCollections.iterateUntil ((fun (st : Sc) => scStep (universes) (st))) (((LexLeanRuntime.multiply (bound) (4) : Nat) + 4)) (({ work := (SWork.SExpr (expr) :: ([] : List (SWork))), scope := ([] : List (ByteArray)), depth := 0, error := Option.none } : Sc)) : (Prod (Sc) (Bool)))).1).error

@[expose] public def rcGo (st : Rc) (stack : List (LexLeanBoot.LexiconLse.Lse)) : Option (Rc) := Option.some (({ stack := stack, found := (st).found } : Rc))

@[expose] public def rcStep (st : Rc) : Option (Rc) := (match (st).stack with | List.nil => Option.none | List.cons expr rest => (match expr with | LexLeanBoot.LexiconLse.Lse.LSortProp => rcGo (st) (rest) | LexLeanBoot.LexiconLse.Lse.LSortType _ => rcGo (st) (rest) | LexLeanBoot.LexiconLse.Lse.LConst p e _ => Option.some (({ stack := rest, found := ((p, e) :: (st).found) } : Rc)) | LexLeanBoot.LexiconLse.Lse.LLocal _ => rcGo (st) (rest) | LexLeanBoot.LexiconLse.Lse.LApp f args => rcGo (st) ((f :: (args :: rest))) | LexLeanBoot.LexiconLse.Lse.LPi binders body => rcGo (st) ((binders :: (body :: rest))) | LexLeanBoot.LexiconLse.Lse.LLam binders body => rcGo (st) ((binders :: (body :: rest))) | LexLeanBoot.LexiconLse.Lse.LLet _ ty value body => rcGo (st) ((ty :: (value :: (body :: rest)))) | LexLeanBoot.LexiconLse.Lse.LNat _ => rcGo (st) (rest) | LexLeanBoot.LexiconLse.Lse.LNil => rcGo (st) (rest) | LexLeanBoot.LexiconLse.Lse.LArg head tail => rcGo (st) ((head :: (tail :: rest))) | LexLeanBoot.LexiconLse.Lse.LBinder _ _ ty more => rcGo (st) ((ty :: (more :: rest)))))

@[expose] public def referencedConsts (expr : LexLeanBoot.LexiconLse.Lse) (bound : Nat) : List ((Prod (ByteArray) (ByteArray))) := LexLeanStd.Order.reverseList ((Prod (ByteArray) (ByteArray))) ((((LexLeanCollections.iterateUntil ((fun (st : Rc) => rcStep (st))) (((LexLeanRuntime.multiply (bound) (4) : Nat) + 4)) (({ stack := (expr :: ([] : List (LexLeanBoot.LexiconLse.Lse))), found := ([] : List ((Prod (ByteArray) (ByteArray)))) } : Rc)) : (Prod (Rc) (Bool)))).1).found)

@[expose] public def isSort (expr : LexLeanBoot.LexiconLse.Lse) : Bool := (match expr with | LexLeanBoot.LexiconLse.Lse.LSortProp => true | LexLeanBoot.LexiconLse.Lse.LSortType _ => true | LexLeanBoot.LexiconLse.Lse.LConst _ _ _ => false | LexLeanBoot.LexiconLse.Lse.LLocal _ => false | LexLeanBoot.LexiconLse.Lse.LApp _ _ => false | LexLeanBoot.LexiconLse.Lse.LPi _ _ => false | LexLeanBoot.LexiconLse.Lse.LLam _ _ => false | LexLeanBoot.LexiconLse.Lse.LLet _ _ _ _ => false | LexLeanBoot.LexiconLse.Lse.LNat _ => false | LexLeanBoot.LexiconLse.Lse.LNil => false | LexLeanBoot.LexiconLse.Lse.LArg _ _ => false | LexLeanBoot.LexiconLse.Lse.LBinder _ _ _ _ => false)

@[expose] public def explicitStep (state : (Prod (LexLeanBoot.LexiconLse.Lse) (Nat))) : Option ((Prod (LexLeanBoot.LexiconLse.Lse) (Nat))) := (match (state).1 with | LexLeanBoot.LexiconLse.Lse.LBinder mode _ _ more => Option.some ((more, (if (Nat.beq (mode) (0)) then ((state).2 + 1) else (state).2))) | LexLeanBoot.LexiconLse.Lse.LSortProp => Option.none | LexLeanBoot.LexiconLse.Lse.LSortType _ => Option.none | LexLeanBoot.LexiconLse.Lse.LConst _ _ _ => Option.none | LexLeanBoot.LexiconLse.Lse.LLocal _ => Option.none | LexLeanBoot.LexiconLse.Lse.LApp _ _ => Option.none | LexLeanBoot.LexiconLse.Lse.LPi _ _ => Option.none | LexLeanBoot.LexiconLse.Lse.LLam _ _ => Option.none | LexLeanBoot.LexiconLse.Lse.LLet _ _ _ _ => Option.none | LexLeanBoot.LexiconLse.Lse.LNat _ => Option.none | LexLeanBoot.LexiconLse.Lse.LNil => Option.none | LexLeanBoot.LexiconLse.Lse.LArg _ _ => Option.none)

@[expose] public def outerShape (expr : LexLeanBoot.LexiconLse.Lse) (bound : Nat) : (Prod (Nat) (Bool)) := (match expr with | LexLeanBoot.LexiconLse.Lse.LPi binders body => ((((LexLeanCollections.iterateUntil ((fun (state : (Prod (LexLeanBoot.LexiconLse.Lse) (Nat))) => explicitStep (state))) ((bound + 1)) ((binders, 0)) : (Prod ((Prod (LexLeanBoot.LexiconLse.Lse) (Nat))) (Bool)))).1).2, isSort (body)) | LexLeanBoot.LexiconLse.Lse.LSortProp => (0, true) | LexLeanBoot.LexiconLse.Lse.LSortType _ => (0, true) | LexLeanBoot.LexiconLse.Lse.LConst _ _ _ => (0, false) | LexLeanBoot.LexiconLse.Lse.LLocal _ => (0, false) | LexLeanBoot.LexiconLse.Lse.LApp _ _ => (0, false) | LexLeanBoot.LexiconLse.Lse.LLam _ _ => (0, false) | LexLeanBoot.LexiconLse.Lse.LLet _ _ _ _ => (0, false) | LexLeanBoot.LexiconLse.Lse.LNat _ => (0, false) | LexLeanBoot.LexiconLse.Lse.LNil => (0, false) | LexLeanBoot.LexiconLse.Lse.LArg _ _ => (0, false) | LexLeanBoot.LexiconLse.Lse.LBinder _ _ _ _ => (0, false))

end LexLeanBoot.LseScopes
