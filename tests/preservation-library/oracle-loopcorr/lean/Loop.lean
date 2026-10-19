
namespace Tre

def tuplePat : List Ident → Option Pat
  | [] => none
  | [a] => some (Pat.bind a)
  | a :: rest => (tuplePat rest).map fun t => Pat.tuple [Pat.bind a, t]

def packReads : List Ident → Option Expr
  | [] => none
  | [a] => some (Expr.move a)
  | a :: rest => (packReads rest).map (Expr.pair (Expr.move a))

def loopParams : List Pos → List Parameter → List Ident → Option (List Parameter × List Ident)
  | [], [], [] => some ([], [])
  | Pos.changed :: ps, Parameter.mk (Pat.bind b) ty :: rest, f :: fs =>
      (loopParams ps rest fs).map fun (pl, os) => (Parameter.mk (Pat.bind f) ty :: pl, b :: os)
  | Pos.kept n :: ps, Parameter.mk (Pat.bind b) ty :: rest, fs =>
      if sameIdent b n then
        (loopParams ps rest fs).map fun (pl, os) => (Parameter.mk (Pat.bind b) ty :: pl, os)
      else none
  | Pos.dropped :: ps, Parameter.mk Pat.wild ty :: rest, fs =>
      (loopParams ps rest fs).map fun (pl, os) => (Parameter.mk Pat.wild ty :: pl, os)
  | _, _, _ => none

def holderId (hn : Nat) : Ident := Ident.generated IdentKind.holder hn
def paramId (k : Nat) : Ident := Ident.generated IdentKind.param k

/-- The pieces of the loop form of a function: its new parameters, the original binders of the
changed parameters, the entry value, and the loop body. -/
def loopParts (me : Nat) (ps : List Pos) (S D : RustType) (hn : Nat) (pns : List Nat)
    (params : List Parameter) (body : Block) :
    Option (List Parameter × List Ident × Expr × Block) :=
  match body with
  | Block.mk lets0 _ =>
    match loopParams ps params (pns.map paramId), rtB me ps S D body with
    | some (pl, os), some (Block.mk _ tail') =>
      match packReads (pns.map paramId), tuplePat os with
      | some pe, some tp =>
        some (pl, os, pe, Block.mk (Let.mk tp none (Expr.move (holderId hn)) :: lets0) tail')
      | _, _ => none
    | _, _ => none

/-- The loop form of a function: its new parameters and body. -/
def loopOf (me : Nat) (ps : List Pos) (S D : RustType) (hn : Nat) (pns : List Nat)
    (params : List Parameter) (body : Block) : Option (List Parameter × Block) :=
  (loopParts me ps S D hn pns params body).map fun (pl, _, pe, B1) =>
    (pl, Block.mk [Let.mk (Pat.bind (holderId hn)) none pe] (Expr.loop (holderId hn) B1))

end Tre
