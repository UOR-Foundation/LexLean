module
public meta import Lean
import all Reasoning.Budget
import all Reasoning.Clinic
import all Reasoning.Main
import all Reasoning.Planner
import all Reasoning.Screening
set_option autoImplicit false
set_option linter.unusedVariables false
universe u u_1 u_2 v w

meta def LexLeanExtract.escape (text : String) : String :=
  (String.foldl (α := String) (fun (out : String) (c : Char) => (if (BEq.beq (α := Char) c '"') then (String.append out "\\\"") else (if (BEq.beq (α := Char) c '\\') then (String.append out "\\\\") else (if (Nat.blt (Char.toNat c) 32) then (String.append out (String.append "\\u" (String.ofList (List.drop (α := Char) 1 (Nat.toDigits 16 (Nat.add (Char.toNat c) 65536)))))) else (String.push out c))))) "" text)

meta def LexLeanExtract.str (text : String) : String :=
  (String.append "\"" (String.append (LexLeanExtract.escape text) "\""))

meta def LexLeanExtract.name (n : Lean.Name) : String :=
  (LexLeanExtract.str (Lean.Name.toString n))

meta def LexLeanExtract.arr (items : (List String)) : String :=
  (String.append "[" (String.append (String.intercalate "," items) "]"))

meta def LexLeanExtract.obj (fields : (List (Prod String String))) : String :=
  (String.append "{" (String.append (String.intercalate "," (List.map (α := (Prod String String)) (β := String) (fun (kv : (Prod String String)) => (String.append (LexLeanExtract.str (kv).fst) (String.append ":" (kv).snd))) fields)) "}"))

meta def LexLeanExtract.bool (value : Bool) : String :=
  (if value then "true" else "false")

meta def LexLeanExtract.fvar (id : Lean.FVarId) : String :=
  (LexLeanExtract.name (id).name)

meta partial def LexLeanExtract.mergeNames (a : (List Lean.Name)) (b : (List Lean.Name)) : (List Lean.Name) :=
  (match a with | .nil => b | .cons x xs => (match b with | .nil => a | .cons y ys => (if (Lean.Name.lt y x) then (List.cons (α := Lean.Name) y (LexLeanExtract.mergeNames a ys)) else (List.cons (α := Lean.Name) x (LexLeanExtract.mergeNames xs b)))))

meta partial def LexLeanExtract.splitNames (l : (List Lean.Name)) : (Prod (List Lean.Name) (List Lean.Name)) :=
  (match l with | .nil => (Prod.mk (α := (List Lean.Name)) (β := (List Lean.Name)) (List.nil (α := Lean.Name)) (List.nil (α := Lean.Name))) | .cons x xs => (let rest : (Prod (List Lean.Name) (List Lean.Name)) := (LexLeanExtract.splitNames xs); (Prod.mk (α := (List Lean.Name)) (β := (List Lean.Name)) (List.cons (α := Lean.Name) x (rest).snd) (rest).fst)))

meta partial def LexLeanExtract.sortNames (l : (List Lean.Name)) : (List Lean.Name) :=
  (match l with | .nil => (List.nil (α := Lean.Name)) | .cons x xs => (match xs with | .nil => l | .cons y ys => (let halves : (Prod (List Lean.Name) (List Lean.Name)) := (LexLeanExtract.splitNames l); (LexLeanExtract.mergeNames (LexLeanExtract.sortNames (halves).fst) (LexLeanExtract.sortNames (halves).snd)))))

meta def LexLeanExtract.sorted (names : (Std.HashSet Lean.Name)) : (List Lean.Name) :=
  (LexLeanExtract.sortNames (Std.HashSet.toList (α := Lean.Name) names))

meta def LexLeanExtract.insert (names : (Std.HashSet Lean.Name)) (n : Lean.Name) : (Std.HashSet Lean.Name) :=
  (Std.HashSet.insert (α := Lean.Name) names n)

meta partial def LexLeanExtract.type (e : Lean.Expr) : String :=
  (match e with | .const n us => (if (BEq.beq (α := Lean.Name) n ``lcErased) then (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "erased"))] : List (Prod String String)))) else (if (BEq.beq (α := Lean.Name) n ``lcAny) then (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "any"))] : List (Prod String String)))) else (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "const")), (Prod.mk (α := String) (β := String) "name" (LexLeanExtract.name n)), (Prod.mk (α := String) (β := String) "levels" (LexLeanExtract.arr (List.map (α := Lean.Level) (β := String) (fun (u : Lean.Level) => (LexLeanExtract.str (ToString.toString (α := Lean.Level) u))) us)))] : List (Prod String String)))))) | .app fn arg => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "app")), (Prod.mk (α := String) (β := String) "head" (LexLeanExtract.type (Lean.Expr.getAppFn e))), (Prod.mk (α := String) (β := String) "arguments" (LexLeanExtract.arr (List.map (α := Lean.Expr) (β := String) (fun (a : Lean.Expr) => (LexLeanExtract.type a)) (Array.toList (α := Lean.Expr) (Lean.Expr.getAppArgs e)))))] : List (Prod String String)))) | .forallE binderName domain body binderInfo => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "arrow")), (Prod.mk (α := String) (β := String) "domain" (LexLeanExtract.type domain)), (Prod.mk (α := String) (β := String) "codomain" (LexLeanExtract.type body))] : List (Prod String String)))) | .fvar id => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "fvar")), (Prod.mk (α := String) (β := String) "id" (LexLeanExtract.fvar id))] : List (Prod String String)))) | .bvar position => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "bvar")), (Prod.mk (α := String) (β := String) "index" (ToString.toString (α := Nat) position))] : List (Prod String String)))) | .sort u => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "sort")), (Prod.mk (α := String) (β := String) "level" (LexLeanExtract.str (ToString.toString (α := Lean.Level) u)))] : List (Prod String String)))) | .mvar id => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "unsupported")), (Prod.mk (α := String) (β := String) "expression" (LexLeanExtract.str "metavariable"))] : List (Prod String String)))) | .lam binderName domain body binderInfo => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "unsupported")), (Prod.mk (α := String) (β := String) "expression" (LexLeanExtract.str "lambda"))] : List (Prod String String)))) | .letE declName t value body nonDep => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "unsupported")), (Prod.mk (α := String) (β := String) "expression" (LexLeanExtract.str "let"))] : List (Prod String String)))) | .lit literal => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "unsupported")), (Prod.mk (α := String) (β := String) "expression" (LexLeanExtract.str "literal"))] : List (Prod String String)))) | .mdata data body => (match (Lean.annotation? (Lean.Name.mkSimple "borrowed") e) with | .some annotated => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "borrowed")), (Prod.mk (α := String) (β := String) "type" (LexLeanExtract.type body))] : List (Prod String String)))) | .none => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "unsupported")), (Prod.mk (α := String) (β := String) "expression" (LexLeanExtract.str "metadata"))] : List (Prod String String))))) | .proj typeName idx struct => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "unsupported")), (Prod.mk (α := String) (β := String) "expression" (LexLeanExtract.str "projection"))] : List (Prod String String)))))

meta def LexLeanExtract.arg (a : (Lean.Compiler.LCNF.Arg Lean.Compiler.LCNF.Purity.pure)) : String :=
  (match a with | .erased => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "erased"))] : List (Prod String String)))) | .fvar id => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "fvar")), (Prod.mk (α := String) (β := String) "id" (LexLeanExtract.fvar id))] : List (Prod String String)))) | .type e => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "type")), (Prod.mk (α := String) (β := String) "type" (LexLeanExtract.type e))] : List (Prod String String)))))

meta def LexLeanExtract.literal (value : Lean.Compiler.LCNF.LitValue) : String :=
  (match value with | .nat v => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "nat")), (Prod.mk (α := String) (β := String) "value" (LexLeanExtract.str (ToString.toString (α := Nat) v)))] : List (Prod String String)))) | .str v => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "string")), (Prod.mk (α := String) (β := String) "value" (LexLeanExtract.str v))] : List (Prod String String)))) | .uint8 v => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "uint8")), (Prod.mk (α := String) (β := String) "value" (LexLeanExtract.str (ToString.toString (α := Nat) (UInt8.toNat v))))] : List (Prod String String)))) | .uint16 v => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "uint16")), (Prod.mk (α := String) (β := String) "value" (LexLeanExtract.str (ToString.toString (α := Nat) (UInt16.toNat v))))] : List (Prod String String)))) | .uint32 v => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "uint32")), (Prod.mk (α := String) (β := String) "value" (LexLeanExtract.str (ToString.toString (α := Nat) (UInt32.toNat v))))] : List (Prod String String)))) | .uint64 v => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "uint64")), (Prod.mk (α := String) (β := String) "value" (LexLeanExtract.str (ToString.toString (α := Nat) (UInt64.toNat v))))] : List (Prod String String)))) | .usize v => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "usize")), (Prod.mk (α := String) (β := String) "value" (LexLeanExtract.str (ToString.toString (α := Nat) (UInt64.toNat v))))] : List (Prod String String)))))

meta def LexLeanExtract.letValue (v : (Lean.Compiler.LCNF.LetValue Lean.Compiler.LCNF.Purity.pure)) : String :=
  (match v with | .lit value => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "literal")), (Prod.mk (α := String) (β := String) "literal" (LexLeanExtract.literal value))] : List (Prod String String)))) | .erased => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "erased"))] : List (Prod String String)))) | .proj typeName position struct => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "projection")), (Prod.mk (α := String) (β := String) "type_name" (LexLeanExtract.name typeName)), (Prod.mk (α := String) (β := String) "index" (ToString.toString (α := Nat) position)), (Prod.mk (α := String) (β := String) "value" (LexLeanExtract.fvar struct))] : List (Prod String String)))) | .const declName us args => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "const")), (Prod.mk (α := String) (β := String) "name" (LexLeanExtract.name declName)), (Prod.mk (α := String) (β := String) "levels" (LexLeanExtract.arr (List.map (α := Lean.Level) (β := String) (fun (u : Lean.Level) => (LexLeanExtract.str (ToString.toString (α := Lean.Level) u))) us))), (Prod.mk (α := String) (β := String) "arguments" (LexLeanExtract.arr (List.map (α := (Lean.Compiler.LCNF.Arg Lean.Compiler.LCNF.Purity.pure)) (β := String) (fun (a : (Lean.Compiler.LCNF.Arg Lean.Compiler.LCNF.Purity.pure)) => (LexLeanExtract.arg a)) (Array.toList (α := (Lean.Compiler.LCNF.Arg Lean.Compiler.LCNF.Purity.pure)) args))))] : List (Prod String String)))) | .fvar id args => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "apply")), (Prod.mk (α := String) (β := String) "function" (LexLeanExtract.fvar id)), (Prod.mk (α := String) (β := String) "arguments" (LexLeanExtract.arr (List.map (α := (Lean.Compiler.LCNF.Arg Lean.Compiler.LCNF.Purity.pure)) (β := String) (fun (a : (Lean.Compiler.LCNF.Arg Lean.Compiler.LCNF.Purity.pure)) => (LexLeanExtract.arg a)) (Array.toList (α := (Lean.Compiler.LCNF.Arg Lean.Compiler.LCNF.Purity.pure)) args))))] : List (Prod String String)))))

meta def LexLeanExtract.param (p : (Lean.Compiler.LCNF.Param Lean.Compiler.LCNF.Purity.pure)) : String :=
  (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "id" (LexLeanExtract.fvar (p).fvarId)), (Prod.mk (α := String) (β := String) "type" (LexLeanExtract.type (p).type)), (Prod.mk (α := String) (β := String) "borrow" (LexLeanExtract.bool (p).borrow))] : List (Prod String String))))

mutual
meta partial def LexLeanExtract.code (c : (Lean.Compiler.LCNF.Code Lean.Compiler.LCNF.Purity.pure)) : String :=
  (match c with | .«let» decl k => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "let")), (Prod.mk (α := String) (β := String) "id" (LexLeanExtract.fvar (decl).fvarId)), (Prod.mk (α := String) (β := String) "type" (LexLeanExtract.type (decl).type)), (Prod.mk (α := String) (β := String) "value" (LexLeanExtract.letValue (decl).value)), (Prod.mk (α := String) (β := String) "body" (LexLeanExtract.code k))] : List (Prod String String)))) | .«fun» decl k => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "fun")), (Prod.mk (α := String) (β := String) "declaration" (LexLeanExtract.funDecl decl)), (Prod.mk (α := String) (β := String) "body" (LexLeanExtract.code k))] : List (Prod String String)))) | .jp decl k => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "join")), (Prod.mk (α := String) (β := String) "declaration" (LexLeanExtract.funDecl decl)), (Prod.mk (α := String) (β := String) "body" (LexLeanExtract.code k))] : List (Prod String String)))) | .jmp id args => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "jump")), (Prod.mk (α := String) (β := String) "target" (LexLeanExtract.fvar id)), (Prod.mk (α := String) (β := String) "arguments" (LexLeanExtract.arr (List.map (α := (Lean.Compiler.LCNF.Arg Lean.Compiler.LCNF.Purity.pure)) (β := String) (fun (a : (Lean.Compiler.LCNF.Arg Lean.Compiler.LCNF.Purity.pure)) => (LexLeanExtract.arg a)) (Array.toList (α := (Lean.Compiler.LCNF.Arg Lean.Compiler.LCNF.Purity.pure)) args))))] : List (Prod String String)))) | .cases cs => (LexLeanExtract.cases cs) | .«return» id => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "return")), (Prod.mk (α := String) (β := String) "id" (LexLeanExtract.fvar id))] : List (Prod String String)))) | .unreach t => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "unreachable")), (Prod.mk (α := String) (β := String) "type" (LexLeanExtract.type t))] : List (Prod String String)))))

meta partial def LexLeanExtract.funDecl (d : (Lean.Compiler.LCNF.FunDecl Lean.Compiler.LCNF.Purity.pure)) : String :=
  (match d with | .mk fvarId binderName params type value => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "id" (LexLeanExtract.fvar fvarId)), (Prod.mk (α := String) (β := String) "parameters" (LexLeanExtract.arr (List.map (α := (Lean.Compiler.LCNF.Param Lean.Compiler.LCNF.Purity.pure)) (β := String) (fun (p : (Lean.Compiler.LCNF.Param Lean.Compiler.LCNF.Purity.pure)) => (LexLeanExtract.param p)) (Array.toList (α := (Lean.Compiler.LCNF.Param Lean.Compiler.LCNF.Purity.pure)) params)))), (Prod.mk (α := String) (β := String) "type" (LexLeanExtract.type type)), (Prod.mk (α := String) (β := String) "value" (LexLeanExtract.code value))] : List (Prod String String)))))

meta partial def LexLeanExtract.cases (cs : (Lean.Compiler.LCNF.Cases Lean.Compiler.LCNF.Purity.pure)) : String :=
  (match cs with | .mk typeName resultType discr alts => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "cases")), (Prod.mk (α := String) (β := String) "type_name" (LexLeanExtract.name typeName)), (Prod.mk (α := String) (β := String) "result_type" (LexLeanExtract.type resultType)), (Prod.mk (α := String) (β := String) "discriminant" (LexLeanExtract.fvar discr)), (Prod.mk (α := String) (β := String) "alternatives" (LexLeanExtract.arr (List.map (α := (Lean.Compiler.LCNF.Alt Lean.Compiler.LCNF.Purity.pure)) (β := String) (fun (a : (Lean.Compiler.LCNF.Alt Lean.Compiler.LCNF.Purity.pure)) => (LexLeanExtract.alt a)) (Array.toList (α := (Lean.Compiler.LCNF.Alt Lean.Compiler.LCNF.Purity.pure)) alts))))] : List (Prod String String)))))

meta partial def LexLeanExtract.alt (a : (Lean.Compiler.LCNF.Alt Lean.Compiler.LCNF.Purity.pure)) : String :=
  (match a with | .alt ctorName params k => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "constructor")), (Prod.mk (α := String) (β := String) "constructor" (LexLeanExtract.name ctorName)), (Prod.mk (α := String) (β := String) "parameters" (LexLeanExtract.arr (List.map (α := (Lean.Compiler.LCNF.Param Lean.Compiler.LCNF.Purity.pure)) (β := String) (fun (p : (Lean.Compiler.LCNF.Param Lean.Compiler.LCNF.Purity.pure)) => (LexLeanExtract.param p)) (Array.toList (α := (Lean.Compiler.LCNF.Param Lean.Compiler.LCNF.Purity.pure)) params)))), (Prod.mk (α := String) (β := String) "code" (LexLeanExtract.code k))] : List (Prod String String)))) | .default k => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "default")), (Prod.mk (α := String) (β := String) "code" (LexLeanExtract.code k))] : List (Prod String String)))))
end

meta partial def LexLeanExtract.typeConstants (e : Lean.Expr) (out : (Std.HashSet Lean.Name)) : (Std.HashSet Lean.Name) :=
  (match e with | .const n us => (if (Bool.or (BEq.beq (α := Lean.Name) n ``lcErased) (BEq.beq (α := Lean.Name) n ``lcAny)) then out else (LexLeanExtract.insert out n)) | .app f a => (LexLeanExtract.typeConstants a (LexLeanExtract.typeConstants f out)) | .forallE binderName d b binderInfo => (LexLeanExtract.typeConstants b (LexLeanExtract.typeConstants d out)) | .bvar position => out | .fvar id => out | .mvar id => out | .sort u => out | .lit literal => out | .lam binderName d b binderInfo => (LexLeanExtract.typeConstants b (LexLeanExtract.typeConstants d out)) | .letE declName t v b nonDep => (LexLeanExtract.typeConstants b (LexLeanExtract.typeConstants v (LexLeanExtract.typeConstants t out))) | .mdata data b => (LexLeanExtract.typeConstants b out) | .proj typeName idx b => (LexLeanExtract.typeConstants b (LexLeanExtract.insert out typeName)))

meta def LexLeanExtract.argConstants (a : (Lean.Compiler.LCNF.Arg Lean.Compiler.LCNF.Purity.pure)) (out : (Std.HashSet Lean.Name)) : (Std.HashSet Lean.Name) :=
  (match a with | .type e => (LexLeanExtract.typeConstants e out) | .erased => out | .fvar id => out)

meta def LexLeanExtract.paramConstants (ps : (Array (Lean.Compiler.LCNF.Param Lean.Compiler.LCNF.Purity.pure))) (out : (Std.HashSet Lean.Name)) : (Std.HashSet Lean.Name) :=
  (Array.foldl (α := (Lean.Compiler.LCNF.Param Lean.Compiler.LCNF.Purity.pure)) (β := (Std.HashSet Lean.Name)) (fun (acc : (Std.HashSet Lean.Name)) (p : (Lean.Compiler.LCNF.Param Lean.Compiler.LCNF.Purity.pure)) => (LexLeanExtract.typeConstants (p).type acc)) out ps)

meta def LexLeanExtract.letValueConstants (v : (Lean.Compiler.LCNF.LetValue Lean.Compiler.LCNF.Purity.pure)) (out : (Std.HashSet Lean.Name)) : (Std.HashSet Lean.Name) :=
  (match v with | .const n us args => (Array.foldl (α := (Lean.Compiler.LCNF.Arg Lean.Compiler.LCNF.Purity.pure)) (β := (Std.HashSet Lean.Name)) (fun (acc : (Std.HashSet Lean.Name)) (a : (Lean.Compiler.LCNF.Arg Lean.Compiler.LCNF.Purity.pure)) => (LexLeanExtract.argConstants a acc)) (LexLeanExtract.insert out n) args) | .fvar id args => (Array.foldl (α := (Lean.Compiler.LCNF.Arg Lean.Compiler.LCNF.Purity.pure)) (β := (Std.HashSet Lean.Name)) (fun (acc : (Std.HashSet Lean.Name)) (a : (Lean.Compiler.LCNF.Arg Lean.Compiler.LCNF.Purity.pure)) => (LexLeanExtract.argConstants a acc)) out args) | .proj typeName position struct => (LexLeanExtract.insert out typeName) | .lit value => out | .erased => out)

mutual
meta partial def LexLeanExtract.codeConstants (c : (Lean.Compiler.LCNF.Code Lean.Compiler.LCNF.Purity.pure)) (out : (Std.HashSet Lean.Name)) : (Std.HashSet Lean.Name) :=
  (match c with | .«let» decl k => (LexLeanExtract.codeConstants k (LexLeanExtract.letValueConstants (decl).value (LexLeanExtract.typeConstants (decl).type out))) | .«fun» decl k => (LexLeanExtract.codeConstants k (LexLeanExtract.funConstants decl out)) | .jp decl k => (LexLeanExtract.codeConstants k (LexLeanExtract.funConstants decl out)) | .jmp id args => (Array.foldl (α := (Lean.Compiler.LCNF.Arg Lean.Compiler.LCNF.Purity.pure)) (β := (Std.HashSet Lean.Name)) (fun (acc : (Std.HashSet Lean.Name)) (a : (Lean.Compiler.LCNF.Arg Lean.Compiler.LCNF.Purity.pure)) => (LexLeanExtract.argConstants a acc)) out args) | .cases cs => (LexLeanExtract.casesConstants cs out) | .«return» id => out | .unreach t => (LexLeanExtract.typeConstants t out))

meta partial def LexLeanExtract.funConstants (d : (Lean.Compiler.LCNF.FunDecl Lean.Compiler.LCNF.Purity.pure)) (out : (Std.HashSet Lean.Name)) : (Std.HashSet Lean.Name) :=
  (match d with | .mk fvarId binderName params type value => (LexLeanExtract.codeConstants value (LexLeanExtract.typeConstants type (LexLeanExtract.paramConstants params out))))

meta partial def LexLeanExtract.casesConstants (cs : (Lean.Compiler.LCNF.Cases Lean.Compiler.LCNF.Purity.pure)) (out : (Std.HashSet Lean.Name)) : (Std.HashSet Lean.Name) :=
  (match cs with | .mk typeName resultType discr alts => (Array.foldl (α := (Lean.Compiler.LCNF.Alt Lean.Compiler.LCNF.Purity.pure)) (β := (Std.HashSet Lean.Name)) (fun (acc : (Std.HashSet Lean.Name)) (a : (Lean.Compiler.LCNF.Alt Lean.Compiler.LCNF.Purity.pure)) => (LexLeanExtract.altConstants a acc)) (LexLeanExtract.typeConstants resultType (LexLeanExtract.insert out typeName)) alts))

meta partial def LexLeanExtract.altConstants (a : (Lean.Compiler.LCNF.Alt Lean.Compiler.LCNF.Purity.pure)) (out : (Std.HashSet Lean.Name)) : (Std.HashSet Lean.Name) :=
  (match a with | .alt ctorName ps k => (LexLeanExtract.codeConstants k (LexLeanExtract.paramConstants ps (LexLeanExtract.insert out ctorName))) | .default k => (LexLeanExtract.codeConstants k out))
end

meta def LexLeanExtract.kind (info : Lean.ConstantInfo) : String :=
  (match info with | .defnInfo value => (match (value).safety with | .safe => "definition" | .«unsafe» => "unsafe-definition" | .«partial» => "partial-definition") | .opaqueInfo opaqueValue => "opaque" | .thmInfo theoremValue => "theorem" | .axiomInfo axiomValue => "axiom" | .inductInfo inductiveValue => "inductive" | .ctorInfo constructorValue => "constructor" | .recInfo recursorValue => "recursor" | .quotInfo quotientValue => "quotient")

meta def LexLeanExtract.originalKind (env : Lean.Environment) (n : Lean.Name) : String :=
  (match (Lean.getOriginalConstKind? env n) with | .some original => (match original with | .defn => "definition" | .thm => "theorem" | .«axiom» => "axiom" | .«opaque» => "opaque" | .quot => "quotient" | .induct => "inductive" | .ctor => "constructor" | .recursor => "recursor") | .none => "unknown")

meta def LexLeanExtract.moduleOf (env : Lean.Environment) (n : Lean.Name) : Lean.Name :=
  (match (Lean.Environment.getModuleIdxFor? env n) with | .some index => (Array.getD (α := Lean.Name) (Lean.EnvironmentHeader.moduleNames (Lean.Environment.header env)) index (Lean.Name.anonymous)) | .none => (Lean.Name.anonymous))

meta def LexLeanExtract.inProject (env : Lean.Environment) (modules : (Array Lean.Name)) (n : Lean.Name) : Bool :=
  (Array.contains (α := Lean.Name) modules (LexLeanExtract.moduleOf env n))

meta def LexLeanExtract.facts (env : Lean.Environment) (n : Lean.Name) (info : Lean.ConstantInfo) : (Lean.Core.CoreM (List (Prod String String))) :=
  (Bind.bind (m := Lean.Core.CoreM) (α := Bool) (β := (List (Prod String String))) (Lean.Compiler.LCNF.shouldGenerateCode n) (fun (generates : Bool) => (Pure.pure (f := Lean.Core.CoreM) (α := (List (Prod String String))) (([(Prod.mk (α := String) (β := String) "name" (LexLeanExtract.name n)), (Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str (LexLeanExtract.kind info))), (Prod.mk (α := String) (β := String) "module" (LexLeanExtract.name (LexLeanExtract.moduleOf env n))), (Prod.mk (α := String) (β := String) "computable" (LexLeanExtract.bool (Bool.not (Lean.isNoncomputable env n)))), (Prod.mk (α := String) (β := String) "generates_code" (LexLeanExtract.bool generates)), (Prod.mk (α := String) (β := String) "original_kind" (LexLeanExtract.str (LexLeanExtract.originalKind env n))), (Prod.mk (α := String) (β := String) "internal" (LexLeanExtract.bool (Lean.Name.isInternal n)))] : List (Prod String String))))))

mutual
meta partial def LexLeanExtract.sameSignature (a : Lean.Expr) (b : Lean.Expr) : Bool :=
  (match a with | .mdata data1 inner1 => (LexLeanExtract.sameSignature inner1 b) | _ => (match b with | .mdata data2 inner2 => (LexLeanExtract.sameSignature a inner2) | _ => (LexLeanExtract.sameStructure a b)))

meta partial def LexLeanExtract.sameStructure (a : Lean.Expr) (b : Lean.Expr) : Bool :=
  (match a with | .forallE name1 domain1 body1 info1 => (match b with | .forallE name2 domain2 body2 info2 => (Bool.and (BEq.beq (α := Lean.BinderInfo) info1 info2) (Bool.and (LexLeanExtract.sameSignature domain1 domain2) (LexLeanExtract.sameSignature body1 body2))) | _ => (Bool.false)) | .lam name1 domain1 body1 info1 => (match b with | .lam name2 domain2 body2 info2 => (Bool.and (BEq.beq (α := Lean.BinderInfo) info1 info2) (Bool.and (LexLeanExtract.sameSignature domain1 domain2) (LexLeanExtract.sameSignature body1 body2))) | _ => (Bool.false)) | .app function1 argument1 => (match b with | .app function2 argument2 => (Bool.and (LexLeanExtract.sameSignature function1 function2) (LexLeanExtract.sameSignature argument1 argument2)) | _ => (Bool.false)) | .const constant1 levels1 => (match b with | .const constant2 levels2 => (Bool.and (BEq.beq (α := Lean.Name) constant1 constant2) (BEq.beq (α := (List Lean.Level)) levels1 levels2)) | _ => (Bool.false)) | .sort level1 => (match b with | .sort level2 => (BEq.beq (α := Lean.Level) level1 level2) | _ => (Bool.false)) | .bvar position1 => (match b with | .bvar position2 => (BEq.beq (α := Nat) position1 position2) | _ => (Bool.false)) | .lit literal1 => (match b with | .lit literal2 => (BEq.beq (α := Lean.Literal) literal1 literal2) | _ => (Bool.false)) | .fvar free1 => (match b with | .fvar free2 => (BEq.beq (α := Lean.FVarId) free1 free2) | _ => (Bool.false)) | .mvar meta1 => (match b with | .mvar meta2 => (BEq.beq (α := Lean.MVarId) meta1 meta2) | _ => (Bool.false)) | .letE local1 type1 value1 scope1 flag1 => (match b with | .letE local2 type2 value2 scope2 flag2 => (Bool.and (LexLeanExtract.sameSignature type1 type2) (Bool.and (LexLeanExtract.sameSignature value1 value2) (LexLeanExtract.sameSignature scope1 scope2))) | _ => (Bool.false)) | .proj structure1 field1 target1 => (match b with | .proj structure2 field2 target2 => (Bool.and (BEq.beq (α := Lean.Name) structure1 structure2) (Bool.and (BEq.beq (α := Nat) field1 field2) (LexLeanExtract.sameSignature target1 target2))) | _ => (Bool.false)) | _ => (Bool.false))
end

public meta def LexLeanExtract.elabSignature : Lean.Elab.Command.CommandElab :=
  (fun (stx : Lean.Syntax) => (Lean.Elab.Command.liftTermElabM (α := Unit) (let declName : Lean.Name := (Lean.Syntax.getId (Lean.Syntax.getArg stx 1)); (Bind.bind (m := Lean.Elab.Term.TermElabM) (α := Lean.ConstantInfo) (β := Unit) (Lean.getConstInfo (m := Lean.Elab.Term.TermElabM) declName) (fun (info : Lean.ConstantInfo) => (Bind.bind (m := Lean.Elab.Term.TermElabM) (α := Lean.Expr) (β := Unit) (Lean.Elab.Term.elabType (Lean.Syntax.getArg stx 3)) (fun (elaborated : Lean.Expr) => (Bind.bind (m := Lean.Elab.Term.TermElabM) (α := Lean.Expr) (β := Unit) (Lean.instantiateMVars (m := Lean.Elab.Term.TermElabM) elaborated) (fun (registered : Lean.Expr) => (Bind.bind (m := Lean.Elab.Term.TermElabM) (α := Lean.Expr) (β := Unit) (Lean.instantiateMVars (m := Lean.Elab.Term.TermElabM) (Lean.ConstantInfo.type info)) (fun (actual : Lean.Expr) => (if (LexLeanExtract.sameSignature actual registered) then (Pure.pure (f := Lean.Elab.Term.TermElabM) (α := Unit) (Unit.unit)) else (Lean.«throwError» (m := Lean.Elab.Term.TermElabM) (α := Unit) (Lean.MessageData.compose (Lean.stringToMessageData "lexlean-extract-drift: the signature of `") (Lean.MessageData.compose (Lean.MessageData.ofName declName) (Lean.MessageData.compose (Lean.stringToMessageData "` is ") (Lean.MessageData.ofExpr (Lean.ConstantInfo.type info))))))))))))))))))

meta def LexLeanExtract.classify (env : Lean.Environment) (c : Lean.Name) (info : Lean.ConstantInfo) : String :=
  (let structural : Bool := (Bool.or (Lean.Environment.isProjectionFn env c) (Bool.or (Lean.isCasesOnRecursor env c) (Bool.or (Lean.isAuxRecursor env c) (Bool.or (Lean.isNoConfusion env c) (Bool.or (Lean.Name.isInternal c) (Bool.or (Lean.Meta.isInstanceCore env c) (Bool.or (Lean.Meta.isMatcherCore env c) (BEq.beq (α := String) (Lean.Name.getString! c) "ctorIdx")))))))); (let former : Bool := (Lean.Expr.isSort (Lean.Expr.getForallBody (Lean.ConstantInfo.type info))); (match info with | .inductInfo inductiveValue => "type" | .defnInfo definitionValue => (if former then "type" else (if (Bool.and (Lean.Name.isPrefixOf (Lean.Name.mkSimple "Lean") c) (Bool.not structural)) then "call" else "plumbing")) | .opaqueInfo opaqueValue => (if (Bool.and (Lean.Name.isPrefixOf (Lean.Name.mkSimple "Lean") c) (Bool.not structural)) then "call" else "plumbing") | .axiomInfo axiomValue => (if (Bool.and (Bool.and (Lean.Name.isPrefixOf (Lean.Name.mkSimple "Lean") c) (Bool.not structural)) (Bool.not former)) then "call" else "plumbing") | .thmInfo theoremValue => "plumbing" | .quotInfo quotientValue => "plumbing" | .ctorInfo constructorValue => "plumbing" | .recInfo recursorValue => "plumbing")))

meta def LexLeanExtract.ownNames (env : Lean.Environment) : (Std.HashSet Lean.Name) :=
  (Lean.SMap.foldStage2 (α := Lean.Name) (β := Lean.ConstantInfo) (σ := (Std.HashSet Lean.Name)) (fun (acc : (Std.HashSet Lean.Name)) (n : Lean.Name) (info : Lean.ConstantInfo) => (LexLeanExtract.insert acc n)) (Std.HashSet.emptyWithCapacity (α := Lean.Name)) (Lean.Environment.constants env))

meta def LexLeanExtract.ownUses (env : Lean.Environment) : (Std.HashSet Lean.Name) :=
  (Lean.SMap.foldStage2 (α := Lean.Name) (β := Lean.ConstantInfo) (σ := (Std.HashSet Lean.Name)) (fun (acc : (Std.HashSet Lean.Name)) (n : Lean.Name) (info : Lean.ConstantInfo) => (if (List.elem (α := Lean.Name) (Lean.Name.mkSimple "LexLeanExtract") (Lean.Name.components (Lean.Name.eraseMacroScopes n))) then (Array.foldl (α := Lean.Name) (β := (Std.HashSet Lean.Name)) (fun (seen : (Std.HashSet Lean.Name)) (used : Lean.Name) => (LexLeanExtract.insert seen used)) (Array.foldl (α := Lean.Name) (β := (Std.HashSet Lean.Name)) (fun (seen : (Std.HashSet Lean.Name)) (used : Lean.Name) => (LexLeanExtract.insert seen used)) acc (Lean.Expr.getUsedConstants (Lean.ConstantInfo.type info))) (match (Lean.ConstantInfo.value? info (Bool.true)) with | .some value => (Lean.Expr.getUsedConstants value) | .none => ((#[] : Array Lean.Name)))) else acc)) (Std.HashSet.emptyWithCapacity (α := Lean.Name)) (Lean.Environment.constants env))

meta partial def LexLeanExtract.classifyAll (env : Lean.Environment) (own : (Std.HashSet Lean.Name)) (names : (List Lean.Name)) : (Lean.Elab.Command.CommandElabM (List (Prod Lean.Name String))) :=
  (match names with | .nil => (Pure.pure (f := Lean.Elab.Command.CommandElabM) (α := (List (Prod Lean.Name String))) (List.nil (α := (Prod Lean.Name String)))) | .cons c rest => (if (Std.HashSet.contains (α := Lean.Name) own c) then (LexLeanExtract.classifyAll env own rest) else (match (Lean.Environment.find? env c) with | .some info => (Bind.bind (m := Lean.Elab.Command.CommandElabM) (α := (List (Prod Lean.Name String))) (β := (List (Prod Lean.Name String))) (LexLeanExtract.classifyAll env own rest) (fun (tail : (List (Prod Lean.Name String))) => (Pure.pure (f := Lean.Elab.Command.CommandElabM) (α := (List (Prod Lean.Name String))) (List.cons (α := (Prod Lean.Name String)) (Prod.mk (α := Lean.Name) (β := String) c (LexLeanExtract.classify env c info)) tail)))) | .none => (Lean.«throwError» (m := Lean.Elab.Command.CommandElabM) (α := (List (Prod Lean.Name String))) (Lean.stringToMessageData (String.append "lexlean-extract-drift: the adapter uses the unknown constant `" (String.append (Lean.Name.toString c) "`")))))))

meta def LexLeanExtract.adapterConstants : (Lean.Elab.Command.CommandElabM (List (Prod Lean.Name String))) :=
  (Bind.bind (m := Lean.Elab.Command.CommandElabM) (α := Lean.Environment) (β := (List (Prod Lean.Name String))) (Lean.MonadEnv.getEnv (m := Lean.Elab.Command.CommandElabM)) (fun (env : Lean.Environment) => (LexLeanExtract.classifyAll env (LexLeanExtract.ownNames env) (LexLeanExtract.sorted (LexLeanExtract.ownUses env)))))

meta partial def LexLeanExtract.listLines (used : (List (Prod Lean.Name String))) : (Lean.Elab.Command.CommandElabM Unit) :=
  (match used with | .nil => (Pure.pure (f := Lean.Elab.Command.CommandElabM) (α := Unit) (Unit.unit)) | .cons entry rest => (Bind.bind (m := Lean.Elab.Command.CommandElabM) (α := Unit) (β := Unit) (MonadLiftT.monadLift (m := IO) (n := Lean.Elab.Command.CommandElabM) (α := Unit) (IO.println (α := String) (String.append (entry).snd (String.append " " (Lean.Name.toString (entry).fst))))) (fun (printed : Unit) => (LexLeanExtract.listLines rest))))

meta def LexLeanExtract.listAdapterConstants : (Lean.Elab.Command.CommandElabM Unit) :=
  (Bind.bind (m := Lean.Elab.Command.CommandElabM) (α := (List (Prod Lean.Name String))) (β := Unit) (LexLeanExtract.adapterConstants) (fun (used : (List (Prod Lean.Name String))) => (LexLeanExtract.listLines used)))

meta def LexLeanExtract.registeredClass (calls : (Array Lean.Name)) (types : (Array (Prod Lean.Name (Array String)))) (opaques : (Array Lean.Name)) (plumbing : (Array Lean.Name)) (c : Lean.Name) : String :=
  (let typeNames : (List Lean.Name) := (List.map (α := (Prod Lean.Name (Array String))) (β := Lean.Name) (fun (row : (Prod Lean.Name (Array String))) => (row).fst) (Array.toList (α := (Prod Lean.Name (Array String))) types)); (if (Array.contains (α := Lean.Name) calls c) then "call" else (if (Bool.or (List.contains (α := Lean.Name) typeNames c) (Array.contains (α := Lean.Name) opaques c)) then "type" else (if (Array.contains (α := Lean.Name) plumbing c) then "plumbing" else (if (List.any (α := (Prod Lean.Name (Array String))) (Array.toList (α := (Prod Lean.Name (Array String))) types) (fun (candidate : (Prod Lean.Name (Array String))) => (Bool.and (BEq.beq (α := Lean.Name) (Lean.Name.getPrefix c) (candidate).fst) (Array.contains (α := String) (candidate).snd (Lean.Name.getString! c))))) then "plumbing" else "unregistered")))))

meta partial def LexLeanExtract.usedProblems (calls : (Array Lean.Name)) (types : (Array (Prod Lean.Name (Array String)))) (opaques : (Array Lean.Name)) (plumbing : (Array Lean.Name)) (used : (List (Prod Lean.Name String))) : (List String) :=
  (match used with | .nil => (List.nil (α := String)) | .cons entry rest => (let c : Lean.Name := (entry).fst; (let foreignType : Bool := (Bool.and (BEq.beq (α := String) (entry).snd "type") (Bool.not (Lean.Name.isPrefixOf (Lean.Name.mkSimple "Lean") c))); (let expected : String := (if foreignType then "a type or plumbing" else (entry).snd); (let registered : String := (LexLeanExtract.registeredClass calls types opaques plumbing c); (if (if (BEq.beq (α := String) (entry).snd "plumbing") then (Bool.not (BEq.beq (α := String) registered "unregistered")) else (if foreignType then (Bool.or (BEq.beq (α := String) registered "type") (BEq.beq (α := String) registered "plumbing")) else (BEq.beq (α := String) registered expected))) then (LexLeanExtract.usedProblems calls types opaques plumbing rest) else (List.cons (α := String) (String.append "`" (String.append (Lean.Name.toString c) (String.append "` is used as " (String.append expected (String.append " but registered as " registered))))) (LexLeanExtract.usedProblems calls types opaques plumbing rest))))))))

meta partial def LexLeanExtract.unusedProblems (used : (List (Prod Lean.Name String))) (names : (List Lean.Name)) : (List String) :=
  (match names with | .nil => (List.nil (α := String)) | .cons c rest => (if (List.any (α := (Prod Lean.Name String)) used (fun (entry : (Prod Lean.Name String)) => (BEq.beq (α := Lean.Name) (entry).fst c))) then (LexLeanExtract.unusedProblems used rest) else (List.cons (α := String) (String.append "`" (String.append (Lean.Name.toString c) "` is plumbing but unused")) (LexLeanExtract.unusedProblems used rest))))

meta partial def LexLeanExtract.typeProblems (env : Lean.Environment) (types : (List (Prod Lean.Name (Array String)))) : (List String) :=
  (match types with | .nil => (List.nil (α := String)) | .cons row rest => (let n : Lean.Name := (row).fst; (match (Lean.Environment.find? env n) with | .some info => (match info with | .inductInfo «inductive» => (if (BEq.beq (α := (List String)) (List.map (α := Lean.Name) (β := String) (fun (constructorName : Lean.Name) => (Lean.Name.getString! constructorName)) («inductive»).ctors) (Array.toList (α := String) (row).snd)) then (LexLeanExtract.typeProblems env rest) else (List.cons (α := String) (String.append "`" (String.append (Lean.Name.toString n) (String.append "` has constructors " (String.append (ToString.toString (α := (List String)) (List.map (α := Lean.Name) (β := String) (fun (listed : Lean.Name) => (Lean.Name.getString! listed)) («inductive»).ctors)) (String.append ", registered " (ToString.toString (α := (List String)) (Array.toList (α := String) (row).snd))))))) (LexLeanExtract.typeProblems env rest))) | _ => (if (Array.isEmpty (α := String) (row).snd) then (LexLeanExtract.typeProblems env rest) else (List.cons (α := String) (String.append "`" (String.append (Lean.Name.toString n) (String.append "` is not an inductive type, registered with constructors " (ToString.toString (α := (List String)) (Array.toList (α := String) (row).snd))))) (LexLeanExtract.typeProblems env rest)))) | .none => (List.cons (α := String) (String.append "`" (String.append (Lean.Name.toString n) "` is not a constant")) (LexLeanExtract.typeProblems env rest)))))

meta def LexLeanExtract.checkNames (calls : (Array Lean.Name)) (types : (Array (Prod Lean.Name (Array String)))) (opaques : (Array Lean.Name)) (plumbing : (Array Lean.Name)) : (Lean.Elab.Command.CommandElabM Unit) :=
  (Bind.bind (m := Lean.Elab.Command.CommandElabM) (α := Lean.Environment) (β := Unit) (Lean.MonadEnv.getEnv (m := Lean.Elab.Command.CommandElabM)) (fun (env : Lean.Environment) => (Bind.bind (m := Lean.Elab.Command.CommandElabM) (α := (List (Prod Lean.Name String))) (β := Unit) (LexLeanExtract.adapterConstants) (fun (used : (List (Prod Lean.Name String))) => (let names : (List Lean.Name) := (Array.toList (α := Lean.Name) plumbing); (let differences : (List String) := (List.append (α := String) (LexLeanExtract.usedProblems calls types opaques plumbing used) (List.append (α := String) (LexLeanExtract.unusedProblems used names) (LexLeanExtract.typeProblems env (Array.toList (α := (Prod Lean.Name (Array String))) types)))); (if (List.isEmpty (α := String) differences) then (Pure.pure (f := Lean.Elab.Command.CommandElabM) (α := Unit) (Unit.unit)) else (Lean.«throwError» (m := Lean.Elab.Command.CommandElabM) (α := Unit) (Lean.stringToMessageData (String.append "lexlean-extract-drift: the adapter and its registry differ: " (String.intercalate "; " differences)))))))))))

meta def LexLeanExtract.declUses (decl : (Lean.Compiler.LCNF.Decl Lean.Compiler.LCNF.Purity.pure)) : (Std.HashSet Lean.Name) :=
  (match (decl).value with | .code body => (LexLeanExtract.codeConstants body (LexLeanExtract.typeConstants ((decl).toSignature).type (LexLeanExtract.paramConstants ((decl).toSignature).params (Std.HashSet.emptyWithCapacity (α := Lean.Name))))) | .extern attributes => (LexLeanExtract.typeConstants ((decl).toSignature).type (LexLeanExtract.paramConstants ((decl).toSignature).params (Std.HashSet.emptyWithCapacity (α := Lean.Name)))))

meta def LexLeanExtract.declText (decl : (Lean.Compiler.LCNF.Decl Lean.Compiler.LCNF.Purity.pure)) : String :=
  (let signatureUses : (Std.HashSet Lean.Name) := (LexLeanExtract.typeConstants ((decl).toSignature).type (LexLeanExtract.paramConstants ((decl).toSignature).params (Std.HashSet.emptyWithCapacity (α := Lean.Name)))); (match (decl).value with | .code body => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "safe" (LexLeanExtract.bool ((decl).toSignature).safe)), (Prod.mk (α := String) (β := String) "level_parameters" (LexLeanExtract.arr (List.map (α := Lean.Name) (β := String) (fun (levelName : Lean.Name) => (LexLeanExtract.name levelName)) ((decl).toSignature).levelParams))), (Prod.mk (α := String) (β := String) "type" (LexLeanExtract.type ((decl).toSignature).type)), (Prod.mk (α := String) (β := String) "parameters" (LexLeanExtract.arr (List.map (α := (Lean.Compiler.LCNF.Param Lean.Compiler.LCNF.Purity.pure)) (β := String) (fun (parameter : (Lean.Compiler.LCNF.Param Lean.Compiler.LCNF.Purity.pure)) => (LexLeanExtract.param parameter)) (Array.toList (α := (Lean.Compiler.LCNF.Param Lean.Compiler.LCNF.Purity.pure)) ((decl).toSignature).params)))), (Prod.mk (α := String) (β := String) "value" (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "code")), (Prod.mk (α := String) (β := String) "code" (LexLeanExtract.code body))] : List (Prod String String))))), (Prod.mk (α := String) (β := String) "uses" (LexLeanExtract.arr (List.map (α := Lean.Name) (β := String) (fun (usedName : Lean.Name) => (LexLeanExtract.name usedName)) (LexLeanExtract.sorted (LexLeanExtract.codeConstants body signatureUses)))))] : List (Prod String String)))) | .extern attributes => (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "safe" (LexLeanExtract.bool ((decl).toSignature).safe)), (Prod.mk (α := String) (β := String) "level_parameters" (LexLeanExtract.arr (List.map (α := Lean.Name) (β := String) (fun (levelName : Lean.Name) => (LexLeanExtract.name levelName)) ((decl).toSignature).levelParams))), (Prod.mk (α := String) (β := String) "type" (LexLeanExtract.type ((decl).toSignature).type)), (Prod.mk (α := String) (β := String) "parameters" (LexLeanExtract.arr (List.map (α := (Lean.Compiler.LCNF.Param Lean.Compiler.LCNF.Purity.pure)) (β := String) (fun (parameter : (Lean.Compiler.LCNF.Param Lean.Compiler.LCNF.Purity.pure)) => (LexLeanExtract.param parameter)) (Array.toList (α := (Lean.Compiler.LCNF.Param Lean.Compiler.LCNF.Purity.pure)) ((decl).toSignature).params)))), (Prod.mk (α := String) (β := String) "value" (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "kind" (LexLeanExtract.str "extern"))] : List (Prod String String))))), (Prod.mk (α := String) (β := String) "uses" (LexLeanExtract.arr (List.map (α := Lean.Name) (β := String) (fun (usedName : Lean.Name) => (LexLeanExtract.name usedName)) (LexLeanExtract.sorted signatureUses))))] : List (Prod String String))))))

meta partial def LexLeanExtract.codeLoop (env : Lean.Environment) (modules : (Array Lean.Name)) (queue : (List Lean.Name)) (visited : (Std.HashSet Lean.Name)) (decls : (Std.HashMap Lean.Name String)) (referenced : (Std.HashSet Lean.Name)) : (Lean.Core.CoreM (Prod (Std.HashSet Lean.Name) (Prod (Std.HashMap Lean.Name String) (Std.HashSet Lean.Name)))) :=
  (match queue with | .nil => (Pure.pure (f := Lean.Core.CoreM) (α := (Prod (Std.HashSet Lean.Name) (Prod (Std.HashMap Lean.Name String) (Std.HashSet Lean.Name)))) (Prod.mk (α := (Std.HashSet Lean.Name)) (β := (Prod (Std.HashMap Lean.Name String) (Std.HashSet Lean.Name))) visited (Prod.mk (α := (Std.HashMap Lean.Name String)) (β := (Std.HashSet Lean.Name)) decls referenced))) | .cons n rest => (if (Std.HashSet.contains (α := Lean.Name) visited n) then (LexLeanExtract.codeLoop env modules rest visited decls referenced) else (match (Lean.Environment.find? env n) with | .some info => (let visitedNow : (Std.HashSet Lean.Name) := (LexLeanExtract.insert visited n); (if (Bool.and (BEq.beq (α := String) (LexLeanExtract.kind info) "definition") (Bool.not (Lean.isNoncomputable env n))) then (Bind.bind (m := Lean.Core.CoreM) (α := Bool) (β := (Prod (Std.HashSet Lean.Name) (Prod (Std.HashMap Lean.Name String) (Std.HashSet Lean.Name)))) (Lean.Compiler.LCNF.shouldGenerateCode n) (fun (generates : Bool) => (if generates then (Bind.bind (m := Lean.Core.CoreM) (α := Unit) (β := (Prod (Std.HashSet Lean.Name) (Prod (Std.HashMap Lean.Name String) (Std.HashSet Lean.Name)))) (Lean.MonadNameGenerator.setNGen (m := Lean.Core.CoreM) (Lean.NameGenerator.mk (Lean.Name.mkSimple "_uniq") 1)) (fun (restarted : Unit) => (Bind.bind (m := Lean.Core.CoreM) (α := (Lean.Compiler.LCNF.Decl Lean.Compiler.LCNF.Purity.pure)) (β := (Prod (Std.HashSet Lean.Name) (Prod (Std.HashMap Lean.Name String) (Std.HashSet Lean.Name)))) (Lean.Compiler.LCNF.CompilerM.run (α := (Lean.Compiler.LCNF.Decl Lean.Compiler.LCNF.Purity.pure)) (Lean.Compiler.LCNF.toDecl n)) (fun (decl : (Lean.Compiler.LCNF.Decl Lean.Compiler.LCNF.Purity.pure)) => (let uses : (List Lean.Name) := (LexLeanExtract.sorted (LexLeanExtract.declUses decl)); (LexLeanExtract.codeLoop env modules (List.append (α := Lean.Name) rest (List.filter (α := Lean.Name) (fun (candidate : Lean.Name) => (LexLeanExtract.inProject env modules candidate)) uses)) visitedNow (Std.HashMap.insert (α := Lean.Name) (β := String) decls ((decl).toSignature).name (LexLeanExtract.declText decl)) (List.foldl (α := (Std.HashSet Lean.Name)) (β := Lean.Name) (fun (seen : (Std.HashSet Lean.Name)) (outside : Lean.Name) => (LexLeanExtract.insert seen outside)) referenced (List.filter (α := Lean.Name) (fun (outside : Lean.Name) => (Bool.not (LexLeanExtract.inProject env modules outside))) uses)))))))) else (LexLeanExtract.codeLoop env modules rest visitedNow decls referenced)))) else (LexLeanExtract.codeLoop env modules rest visitedNow decls referenced))) | .none => (Lean.«throwError» (m := Lean.Core.CoreM) (α := (Prod (Std.HashSet Lean.Name) (Prod (Std.HashMap Lean.Name String) (Std.HashSet Lean.Name)))) (Lean.stringToMessageData (String.append "lexlean-extract: unresolved constant `" (String.append (Lean.Name.toString n) "`")))))))

meta partial def LexLeanExtract.kernelLoop (env : Lean.Environment) (modules : (Array Lean.Name)) (queue : (List Lean.Name)) (seen : (Std.HashSet Lean.Name)) : (Lean.Core.CoreM (Std.HashSet Lean.Name)) :=
  (match queue with | .nil => (Pure.pure (f := Lean.Core.CoreM) (α := (Std.HashSet Lean.Name)) seen) | .cons n rest => (if (Std.HashSet.contains (α := Lean.Name) seen n) then (LexLeanExtract.kernelLoop env modules rest seen) else (match (Lean.Environment.find? env n) with | .some info => (LexLeanExtract.kernelLoop env modules (List.append (α := Lean.Name) rest (List.filter (α := Lean.Name) (fun (candidate : Lean.Name) => (LexLeanExtract.inProject env modules candidate)) (LexLeanExtract.sorted (Array.foldl (α := Lean.Name) (β := (Std.HashSet Lean.Name)) (fun (acc : (Std.HashSet Lean.Name)) (used : Lean.Name) => (LexLeanExtract.insert acc used)) (Std.HashSet.emptyWithCapacity (α := Lean.Name)) (match (Lean.ConstantInfo.value? info (Bool.true)) with | .some value => (Lean.Expr.getUsedConstants value) | .none => ((#[] : Array Lean.Name))))))) (LexLeanExtract.insert seen n)) | .none => (Lean.«throwError» (m := Lean.Core.CoreM) (α := (Std.HashSet Lean.Name)) (Lean.stringToMessageData (String.append "lexlean-extract: unresolved constant `" (String.append (Lean.Name.toString n) "`")))))))

meta partial def LexLeanExtract.ctorRows (env : Lean.Environment) (names : (List Lean.Name)) : (Lean.Core.CoreM (List String)) :=
  (match names with | .nil => (Pure.pure (f := Lean.Core.CoreM) (α := (List String)) (List.nil (α := String))) | .cons c rest => (match (Lean.Environment.find? env c) with | .some info => (match info with | .ctorInfo constructor => (Bind.bind (m := Lean.Core.CoreM) (α := Lean.Expr) (β := (List String)) (Lean.Meta.MetaM.run' (α := Lean.Expr) (Lean.Compiler.LCNF.toLCNFType ((constructor).toConstantVal).type)) (fun (constructorType : Lean.Expr) => (Bind.bind (m := Lean.Core.CoreM) (α := (List String)) (β := (List String)) (LexLeanExtract.ctorRows env rest) (fun (tail : (List String)) => (Pure.pure (f := Lean.Core.CoreM) (α := (List String)) (List.cons (α := String) (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "name" (LexLeanExtract.name c)), (Prod.mk (α := String) (β := String) "parameters" (ToString.toString (α := Nat) (constructor).numParams)), (Prod.mk (α := String) (β := String) "fields" (ToString.toString (α := Nat) (constructor).numFields)), (Prod.mk (α := String) (β := String) "type" (LexLeanExtract.type constructorType))] : List (Prod String String)))) tail)))))) | _ => (Lean.«throwError» (m := Lean.Core.CoreM) (α := (List String)) (Lean.stringToMessageData (String.append "lexlean-extract: `" (String.append (Lean.Name.toString c) "` is not a constructor"))))) | .none => (Lean.«throwError» (m := Lean.Core.CoreM) (α := (List String)) (Lean.stringToMessageData (String.append "lexlean-extract: `" (String.append (Lean.Name.toString c) "` is not a constructor"))))))

meta def LexLeanExtract.inductiveRow (env : Lean.Environment) (n : Lean.Name) : (Lean.Core.CoreM String) :=
  (match (Lean.Environment.find? env n) with | .some info => (match info with | .inductInfo «inductive» => (Bind.bind (m := Lean.Core.CoreM) (α := (List String)) (β := String) (LexLeanExtract.ctorRows env («inductive»).ctors) (fun (constructors : (List String)) => (Pure.pure (f := Lean.Core.CoreM) (α := String) (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "parameters" (ToString.toString (α := Nat) («inductive»).numParams)), (Prod.mk (α := String) (β := String) "indices" (ToString.toString (α := Nat) («inductive»).numIndices)), (Prod.mk (α := String) (β := String) "recursive" (LexLeanExtract.bool («inductive»).isRec)), (Prod.mk (α := String) (β := String) "constructors" (LexLeanExtract.arr constructors))] : List (Prod String String))))))) | _ => (Pure.pure (f := Lean.Core.CoreM) (α := String) "null")) | .none => (Pure.pure (f := Lean.Core.CoreM) (α := String) "null"))

meta partial def LexLeanExtract.constantRows (env : Lean.Environment) (decls : (Std.HashMap Lean.Name String)) (names : (List Lean.Name)) : (Lean.Core.CoreM (List String)) :=
  (match names with | .nil => (Pure.pure (f := Lean.Core.CoreM) (α := (List String)) (List.nil (α := String))) | .cons n rest => (match (Lean.Environment.find? env n) with | .some info => (Bind.bind (m := Lean.Core.CoreM) (α := (List (Prod String String))) (β := (List String)) (LexLeanExtract.facts env n info) (fun (row : (List (Prod String String))) => (Bind.bind (m := Lean.Core.CoreM) (α := String) (β := (List String)) (LexLeanExtract.inductiveRow env n) (fun (inductiveText : String) => (Bind.bind (m := Lean.Core.CoreM) (α := (List String)) (β := (List String)) (LexLeanExtract.constantRows env decls rest) (fun (tail : (List String)) => (Pure.pure (f := Lean.Core.CoreM) (α := (List String)) (List.cons (α := String) (LexLeanExtract.obj (List.append (α := (Prod String String)) row (([(Prod.mk (α := String) (β := String) "kernel_uses" (LexLeanExtract.arr (List.map (α := Lean.Name) (β := String) (fun (usedName : Lean.Name) => (LexLeanExtract.name usedName)) (LexLeanExtract.sorted (Array.foldl (α := Lean.Name) (β := (Std.HashSet Lean.Name)) (fun (acc : (Std.HashSet Lean.Name)) (used : Lean.Name) => (LexLeanExtract.insert acc used)) (Std.HashSet.emptyWithCapacity (α := Lean.Name)) (match (Lean.ConstantInfo.value? info (Bool.true)) with | .some value => (Lean.Expr.getUsedConstants value) | .none => ((#[] : Array Lean.Name)))))))), (Prod.mk (α := String) (β := String) "declaration" (Option.getD (α := String) (Std.HashMap.get? (α := Lean.Name) (β := String) decls n) "null")), (Prod.mk (α := String) (β := String) "inductive" inductiveText)] : List (Prod String String))))) tail)))))))) | .none => (Lean.«throwError» (m := Lean.Core.CoreM) (α := (List String)) (Lean.stringToMessageData (String.append "lexlean-extract: unresolved constant `" (String.append (Lean.Name.toString n) "`"))))))

meta partial def LexLeanExtract.externalRows (env : Lean.Environment) (names : (List Lean.Name)) : (Lean.Core.CoreM (List String)) :=
  (match names with | .nil => (Pure.pure (f := Lean.Core.CoreM) (α := (List String)) (List.nil (α := String))) | .cons r rest => (match (Lean.Environment.find? env r) with | .some info => (Bind.bind (m := Lean.Core.CoreM) (α := (List (Prod String String))) (β := (List String)) (LexLeanExtract.facts env r info) (fun (row : (List (Prod String String))) => (Bind.bind (m := Lean.Core.CoreM) (α := (List String)) (β := (List String)) (LexLeanExtract.externalRows env rest) (fun (tail : (List String)) => (let compiled : (Prod String String) := (Prod.mk (α := String) (β := String) "compiled" (LexLeanExtract.bool (Option.isSome (α := Lean.IR.Decl) (Lean.IR.findEnvDecl env r)))); (Pure.pure (f := Lean.Core.CoreM) (α := (List String)) (List.cons (α := String) (LexLeanExtract.obj (List.append (α := (Prod String String)) row (match info with | .ctorInfo constructor => (([compiled, (Prod.mk (α := String) (β := String) "inductive_type" (LexLeanExtract.name (constructor).induct))] : List (Prod String String))) | _ => (([compiled] : List (Prod String String)))))) tail))))))) | .none => (Lean.«throwError» (m := Lean.Core.CoreM) (α := (List String)) (Lean.stringToMessageData (String.append "lexlean-extract: unresolved constant `" (String.append (Lean.Name.toString r) "`"))))))

meta partial def LexLeanExtract.checkRoots (env : Lean.Environment) (modules : (Array Lean.Name)) (roots : (List Lean.Name)) : (Lean.Core.CoreM Unit) :=
  (match roots with | .nil => (Pure.pure (f := Lean.Core.CoreM) (α := Unit) (Unit.unit)) | .cons root rest => (if (Bool.and (LexLeanExtract.inProject env modules root) (Option.isSome (α := Lean.ConstantInfo) (Lean.Environment.find? env root))) then (LexLeanExtract.checkRoots env modules rest) else (Lean.«throwError» (m := Lean.Core.CoreM) (α := Unit) (Lean.stringToMessageData (String.append "lexlean-extract: unknown root `" (String.append (Lean.Name.toString root) "`"))))))

meta def LexLeanExtract.run (roots : (Array Lean.Name)) (modules : (Array Lean.Name)) : (Lean.Core.CoreM String) :=
  (Bind.bind (m := Lean.Core.CoreM) (α := Lean.Environment) (β := String) (Lean.MonadEnv.getEnv (m := Lean.Core.CoreM)) (fun (env : Lean.Environment) => (Bind.bind (m := Lean.Core.CoreM) (α := Unit) (β := String) (LexLeanExtract.checkRoots env modules (Array.toList (α := Lean.Name) roots)) (fun (checked : Unit) => (Bind.bind (m := Lean.Core.CoreM) (α := (Prod (Std.HashSet Lean.Name) (Prod (Std.HashMap Lean.Name String) (Std.HashSet Lean.Name)))) (β := String) (LexLeanExtract.codeLoop env modules (Array.toList (α := Lean.Name) roots) (Std.HashSet.emptyWithCapacity (α := Lean.Name)) (Std.HashMap.emptyWithCapacity (α := Lean.Name) (β := String)) (Std.HashSet.emptyWithCapacity (α := Lean.Name))) (fun (visit : (Prod (Std.HashSet Lean.Name) (Prod (Std.HashMap Lean.Name String) (Std.HashSet Lean.Name)))) => (Bind.bind (m := Lean.Core.CoreM) (α := (Std.HashSet Lean.Name)) (β := String) (LexLeanExtract.kernelLoop env modules (Std.HashSet.toList (α := Lean.Name) (visit).fst) (Std.HashSet.emptyWithCapacity (α := Lean.Name))) (fun (recorded : (Std.HashSet Lean.Name)) => (Bind.bind (m := Lean.Core.CoreM) (α := (List String)) (β := String) (LexLeanExtract.constantRows env ((visit).snd).fst (LexLeanExtract.sorted recorded)) (fun (constants : (List String)) => (Bind.bind (m := Lean.Core.CoreM) (α := (List String)) (β := String) (LexLeanExtract.externalRows env (LexLeanExtract.sorted ((visit).snd).snd)) (fun (externals : (List String)) => (Pure.pure (f := Lean.Core.CoreM) (α := String) (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "spec" (LexLeanExtract.str "lexlean/lcnf-extraction/2")), (Prod.mk (α := String) (β := String) "lean" (LexLeanExtract.obj (([(Prod.mk (α := String) (β := String) "version" (LexLeanExtract.str (Lean.versionString))), (Prod.mk (α := String) (β := String) "githash" (LexLeanExtract.str (Lean.githash)))] : List (Prod String String))))), (Prod.mk (α := String) (β := String) "roots" (LexLeanExtract.arr (List.map (α := Lean.Name) (β := String) (fun (listedName : Lean.Name) => (LexLeanExtract.name listedName)) (Array.toList (α := Lean.Name) roots)))), (Prod.mk (α := String) (β := String) "constants" (LexLeanExtract.arr constants)), (Prod.mk (α := String) (β := String) "externals" (LexLeanExtract.arr externals))] : List (Prod String String)))))))))))))))))

meta def LexLeanExtract.mainNames (roots : (Array Lean.Name)) (modules : (Array Lean.Name)) : (Lean.Elab.Command.CommandElabM Unit) :=
  (Lean.Elab.Command.liftCoreM (α := Unit) (Bind.bind (m := Lean.Core.CoreM) (α := String) (β := Unit) (LexLeanExtract.run roots modules) (fun (text : String) => (MonadLiftT.monadLift (m := IO) (n := Lean.Core.CoreM) (α := Unit) (IO.println (α := String) text)))))

meta def LexLeanExtract.toNames (texts : (Array String)) : (Array Lean.Name) :=
  (Array.map (α := String) (β := Lean.Name) (fun (text : String) => (String.toName text)) texts)

meta def LexLeanExtract.toTypeRows (rows : (Array (Prod String (Array String)))) : (Array (Prod Lean.Name (Array String))) :=
  (Array.map (α := (Prod String (Array String))) (β := (Prod Lean.Name (Array String))) (fun (row : (Prod String (Array String))) => (Prod.mk (α := Lean.Name) (β := (Array String)) (String.toName (row).fst) (row).snd)) rows)

meta def LexLeanExtract.checkAuthority (calls : (Array String)) (types : (Array (Prod String (Array String)))) (opaques : (Array String)) (plumbing : (Array String)) : (Lean.Elab.Command.CommandElabM Unit) :=
  (LexLeanExtract.checkNames (LexLeanExtract.toNames calls) (LexLeanExtract.toTypeRows types) (LexLeanExtract.toNames opaques) (LexLeanExtract.toNames plumbing))

meta def LexLeanExtract.main (roots : (Array String)) (modules : (Array String)) : (Lean.Elab.Command.CommandElabM Unit) :=
  (LexLeanExtract.mainNames (LexLeanExtract.toNames roots) (LexLeanExtract.toNames modules))

syntax (name := LexLeanExtract.signatureCommand) "lexlean_signature " ident " : " term : command
attribute [command_elab LexLeanExtract.signatureCommand] LexLeanExtract.elabSignature

lexlean_signature Array.contains : ({α : (Type u)} → ([h1 : (BEq α)] → ((Array α) → (α → Bool))))

lexlean_signature Array.foldl : ({α : (Type u)} → ({β : (Type v)} → ((β → (α → β)) → (β → ((as : (Array α)) → ((optParam Nat 0) → ((optParam Nat (@Array.size α as)) → β)))))))

lexlean_signature Array.getD : ({α : (Type u_1)} → ((Array α) → (Nat → (α → α))))

lexlean_signature Array.isEmpty : ({α : (Type u)} → ((Array α) → Bool))

lexlean_signature Array.map : ({α : (Type u)} → ({β : (Type v)} → ((α → β) → ((Array α) → (Array β)))))

lexlean_signature Array.size : ({α : (Type u)} → ((Array α) → Nat))

lexlean_signature Array.toList : ({α : (Type u)} → ((Array α) → (List α)))

lexlean_signature BEq.beq : ({α : (Type u)} → ([self : (BEq α)] → (α → (α → Bool))))

lexlean_signature Bind.bind : ({m : ((Type u) → (Type v))} → ([self : (Bind m)] → ({α : (Type u)} → ({β : (Type u)} → ((m α) → ((α → (m β)) → (m β)))))))

lexlean_signature Bool.and : (Bool → (Bool → Bool))

lexlean_signature Bool.not : (Bool → Bool)

lexlean_signature Bool.or : (Bool → (Bool → Bool))

lexlean_signature Char.toNat : (Char → Nat)

lexlean_signature IO.println : ({α : (Type u_1)} → ([h1 : (ToString α)] → (α → (IO Unit))))

lexlean_signature Lean.Compiler.LCNF.CompilerM.run : ({α : Type} → ((Lean.Compiler.LCNF.CompilerM α) → ((optParam Lean.Compiler.LCNF.CompilerM.State { }) → ((optParam Lean.Compiler.LCNF.Phase Lean.Compiler.LCNF.Phase.base) → (Lean.Core.CoreM α)))))

lexlean_signature Lean.Compiler.LCNF.shouldGenerateCode : (Lean.Name → (Lean.Core.CoreM Bool))

lexlean_signature Lean.Compiler.LCNF.toDecl : (Lean.Name → (Lean.Compiler.LCNF.CompilerM (Lean.Compiler.LCNF.Decl Lean.Compiler.LCNF.Purity.pure)))

lexlean_signature Lean.Compiler.LCNF.toLCNFType : (Lean.Expr → (Lean.Meta.MetaM Lean.Expr))

lexlean_signature Lean.ConstantInfo.type : (Lean.ConstantInfo → Lean.Expr)

lexlean_signature Lean.ConstantInfo.value? : (Lean.ConstantInfo → ((optParam Bool Bool.false) → (Option Lean.Expr)))

lexlean_signature Lean.Elab.Command.liftCoreM : ({α : Type} → ((Lean.Core.CoreM α) → (Lean.Elab.Command.CommandElabM α)))

lexlean_signature Lean.Elab.Command.liftTermElabM : ({α : Type} → ((Lean.Elab.Term.TermElabM α) → (Lean.Elab.Command.CommandElabM α)))

lexlean_signature Lean.Elab.Term.elabType : (Lean.Syntax → (Lean.Elab.Term.TermElabM Lean.Expr))

lexlean_signature Lean.EnvExtension.asyncMode : ({σ : Type} → ((Lean.EnvExtension σ) → Lean.EnvExtension.AsyncMode))

lexlean_signature Lean.Environment.constants : (Lean.Environment → Lean.ConstMap)

lexlean_signature Lean.Environment.find? : (Lean.Environment → (Lean.Name → ((optParam Bool Bool.false) → (Option Lean.ConstantInfo))))

lexlean_signature Lean.Environment.getModuleIdxFor? : (Lean.Environment → (Lean.Name → (Option Lean.ModuleIdx)))

lexlean_signature Lean.Environment.header : (Lean.Environment → Lean.EnvironmentHeader)

lexlean_signature Lean.Environment.isProjectionFn : (Lean.Environment → (Lean.Name → Bool))

lexlean_signature Lean.EnvironmentHeader.moduleNames : (Lean.EnvironmentHeader → (Array Lean.Name))

lexlean_signature Lean.Expr.getAppArgs : (Lean.Expr → (Array Lean.Expr))

lexlean_signature Lean.Expr.getAppFn : (Lean.Expr → Lean.Expr)

lexlean_signature Lean.Expr.getForallBody : (Lean.Expr → Lean.Expr)

lexlean_signature Lean.Expr.getUsedConstants : (Lean.Expr → (Array Lean.Name))

lexlean_signature Lean.Expr.isSort : (Lean.Expr → Bool)

lexlean_signature Lean.IR.findEnvDecl : (Lean.Environment → (Lean.Name → (Option Lean.IR.Decl)))

lexlean_signature Lean.MessageData.compose : (Lean.MessageData → (Lean.MessageData → Lean.MessageData))

lexlean_signature Lean.MessageData.ofExpr : (Lean.Expr → Lean.MessageData)

lexlean_signature Lean.MessageData.ofName : (Lean.Name → Lean.MessageData)

lexlean_signature Lean.Meta.MetaM.run' : ({α : Type} → ((Lean.Meta.MetaM α) → ((optParam Lean.Meta.Context { }) → ((optParam Lean.Meta.State { }) → (Lean.Core.CoreM α)))))

lexlean_signature Lean.Meta.isInstanceCore : (Lean.Environment → (Lean.Name → Bool))

lexlean_signature Lean.Meta.isMatcherCore : (Lean.Environment → (Lean.Name → Bool))

lexlean_signature Lean.MonadEnv.getEnv : ({m : (Type → Type)} → ([self : (Lean.MonadEnv m)] → (m Lean.Environment)))

lexlean_signature Lean.MonadNameGenerator.setNGen : ({m : (Type → Type)} → ([self : (Lean.MonadNameGenerator m)] → (Lean.NameGenerator → (m Unit))))

lexlean_signature Lean.Name.components : (Lean.Name → (List Lean.Name))

lexlean_signature Lean.Name.eraseMacroScopes : (Lean.Name → Lean.Name)

lexlean_signature Lean.Name.getPrefix : (Lean.Name → Lean.Name)

lexlean_signature Lean.Name.getString! : (Lean.Name → String)

lexlean_signature Lean.Name.isInternal : (Lean.Name → Bool)

lexlean_signature Lean.Name.isPrefixOf : (Lean.Name → (Lean.Name → Bool))

lexlean_signature Lean.Name.lt : (Lean.Name → (Lean.Name → Bool))

lexlean_signature Lean.Name.mkSimple : (String → Lean.Name)

lexlean_signature Lean.Name.mkStr1 : (String → Lean.Name)

lexlean_signature Lean.Name.mkStr2 : (String → (String → Lean.Name))

lexlean_signature Lean.Name.toString : (Lean.Name → ((optParam Bool Bool.true) → String))

lexlean_signature Lean.PersistentArray.branching : USize

lexlean_signature Lean.PersistentArray.initShift : USize

lexlean_signature Lean.PersistentEnvExtension.toEnvExtension : ({α : Type} → ({β : Type} → ({σ : Type} → ((Lean.PersistentEnvExtension α β σ) → (Lean.EnvExtension (Lean.PersistentEnvExtensionState α σ))))))

lexlean_signature Lean.PersistentHashMap.mkEmptyEntriesArray : ({α : (Type u_1)} → ({β : (Type u_2)} → (Array (Lean.PersistentHashMap.Entry α β (Lean.PersistentHashMap.Node α β)))))

lexlean_signature Lean.SMap.foldStage2 : ({α : (Type u)} → ({β : (Type v)} → ([h2 : (BEq α)] → ([h3 : (Hashable α)] → ({σ : (Type w)} → ((σ → (α → (β → σ))) → (σ → ((@Lean.SMap α β h2 h3) → σ))))))))

lexlean_signature Lean.Syntax.getArg : (Lean.Syntax → (Nat → Lean.Syntax))

lexlean_signature Lean.Syntax.getId : (Lean.Syntax → Lean.Name)

lexlean_signature Lean.annotation? : (Lean.Name → (Lean.Expr → (Option Lean.Expr)))

lexlean_signature Lean.getConstInfo : ({m : (Type → Type)} → ([h1 : (Monad m)] → ([h2 : (Lean.MonadEnv m)] → ([h3 : (Lean.MonadError m)] → (Lean.Name → (m Lean.ConstantInfo))))))

lexlean_signature Lean.getOriginalConstKind? : (Lean.Environment → (Lean.Name → (Option Lean.ConstantKind)))

lexlean_signature Lean.githash : String

lexlean_signature Lean.instantiateMVars : ({m : (Type → Type)} → ([h1 : (Monad m)] → ([h2 : (Lean.MonadMCtx m)] → (Lean.Expr → (m Lean.Expr)))))

lexlean_signature Lean.isAuxRecursor : (Lean.Environment → (Lean.Name → Bool))

lexlean_signature Lean.isCasesOnRecursor : (Lean.Environment → (Lean.Name → Bool))

lexlean_signature Lean.isNoConfusion : (Lean.Environment → (Lean.Name → Bool))

lexlean_signature Lean.isNoncomputable : (Lean.Environment → (Lean.Name → ((optParam Lean.EnvExtension.AsyncMode (@Lean.EnvExtension.asyncMode (Lean.PersistentEnvExtensionState Lean.Name (Prod (List Lean.Name) Lean.NameSet)) (@Lean.PersistentEnvExtension.toEnvExtension Lean.Name Lean.Name (Prod (List Lean.Name) Lean.NameSet) Lean.noncomputableExt))) → Bool)))

lexlean_signature Lean.noncomputableExt : Lean.TagDeclarationExtension

lexlean_signature Lean.stringToMessageData : (String → Lean.MessageData)

lexlean_signature Lean.«throwError» : ({m : (Type → Type)} → ({α : Type} → ([h2 : (Monad m)] → ([h3 : (Lean.MonadError m)] → (Lean.MessageData → (m α))))))

lexlean_signature Lean.versionString : String

lexlean_signature List.any : ({α : (Type u)} → ((List α) → ((α → Bool) → Bool)))

lexlean_signature List.append : ({α : (Type u_1)} → ((List α) → ((List α) → (List α))))

lexlean_signature List.contains : ({α : (Type u)} → ([h1 : (BEq α)] → ((List α) → (α → Bool))))

lexlean_signature List.drop : ({α : (Type u)} → (Nat → ((List α) → (List α))))

lexlean_signature List.elem : ({α : (Type u)} → ([h1 : (BEq α)] → (α → ((List α) → Bool))))

lexlean_signature List.filter : ({α : (Type u)} → ((α → Bool) → ((List α) → (List α))))

lexlean_signature List.foldl : ({α : (Type u)} → ({β : (Type v)} → ((α → (β → α)) → (α → ((List β) → α)))))

lexlean_signature List.isEmpty : ({α : (Type u)} → ((List α) → Bool))

lexlean_signature List.map : ({α : (Type u_1)} → ({β : (Type u_2)} → ((α → β) → ((List α) → (List β)))))

lexlean_signature MonadLiftT.monadLift : ({m : ((Type u) → (Type v))} → ({n : ((Type u) → (Type w))} → ([self : (MonadLiftT m n)] → ({α : (Type u)} → ((m α) → (n α))))))

lexlean_signature Nat.add : (Nat → (Nat → Nat))

lexlean_signature Nat.blt : (Nat → (Nat → Bool))

lexlean_signature Nat.toDigits : (Nat → (Nat → (List Char)))

lexlean_signature Option.getD : ({α : (Type u_1)} → ((Option α) → (α → α)))

lexlean_signature Option.isSome : ({α : (Type u_1)} → ((Option α) → Bool))

lexlean_signature Pure.pure : ({f : ((Type u) → (Type v))} → ([self : (Pure f)] → ({α : (Type u)} → (α → (f α)))))

lexlean_signature Std.HashMap.emptyWithCapacity : ({α : (Type u)} → ({β : (Type v)} → ([h2 : (BEq α)] → ([h3 : (Hashable α)] → ((optParam Nat 8) → (@Std.HashMap α β h2 h3))))))

lexlean_signature Std.HashMap.get? : ({α : (Type u)} → ({β : (Type v)} → ({h2 : (BEq α)} → ({h3 : (Hashable α)} → ((@Std.HashMap α β h2 h3) → (α → (Option β)))))))

lexlean_signature Std.HashMap.insert : ({α : (Type u)} → ({β : (Type v)} → ({h2 : (BEq α)} → ({h3 : (Hashable α)} → ((@Std.HashMap α β h2 h3) → (α → (β → (@Std.HashMap α β h2 h3))))))))

lexlean_signature Std.HashSet.contains : ({α : (Type u)} → ({h1 : (BEq α)} → ({h2 : (Hashable α)} → ((@Std.HashSet α h1 h2) → (α → Bool)))))

lexlean_signature Std.HashSet.emptyWithCapacity : ({α : (Type u)} → ([h1 : (BEq α)] → ([h2 : (Hashable α)] → ((optParam Nat 8) → (@Std.HashSet α h1 h2)))))

lexlean_signature Std.HashSet.insert : ({α : (Type u)} → ({h1 : (BEq α)} → ({h2 : (Hashable α)} → ((@Std.HashSet α h1 h2) → (α → (@Std.HashSet α h1 h2))))))

lexlean_signature Std.HashSet.toList : ({α : (Type u)} → ({h1 : (BEq α)} → ({h2 : (Hashable α)} → ((@Std.HashSet α h1 h2) → (List α)))))

lexlean_signature String.append : (String → (String → String))

lexlean_signature String.foldl : ({α : (Type u)} → ((α → (Char → α)) → (α → (String → α))))

lexlean_signature String.intercalate : (String → ((List String) → String))

lexlean_signature String.ofList : ((List Char) → String)

lexlean_signature String.push : (String → (Char → String))

lexlean_signature String.toName : (String → Lean.Name)

lexlean_signature ToString.toString : ({α : (Type u)} → ([self : (ToString α)] → (α → String)))

lexlean_signature UInt16.toNat : (UInt16 → Nat)

lexlean_signature UInt32.toNat : (UInt32 → Nat)

lexlean_signature UInt64.toNat : (UInt64 → Nat)

lexlean_signature UInt8.toNat : (UInt8 → Nat)

lexlean_signature Unit.unit : Unit

#eval LexLeanExtract.checkAuthority ((#["Array.contains", "Array.foldl", "Array.getD", "Array.isEmpty", "Array.map", "Array.size", "Array.toList", "BEq.beq", "Bind.bind", "Bool.and", "Bool.not", "Bool.or", "Char.toNat", "IO.println", "Lean.Compiler.LCNF.CompilerM.run", "Lean.Compiler.LCNF.shouldGenerateCode", "Lean.Compiler.LCNF.toDecl", "Lean.Compiler.LCNF.toLCNFType", "Lean.ConstantInfo.type", "Lean.ConstantInfo.value?", "Lean.Elab.Command.liftCoreM", "Lean.Elab.Command.liftTermElabM", "Lean.Elab.Term.elabType", "Lean.EnvExtension.asyncMode", "Lean.Environment.constants", "Lean.Environment.find?", "Lean.Environment.getModuleIdxFor?", "Lean.Environment.header", "Lean.Environment.isProjectionFn", "Lean.EnvironmentHeader.moduleNames", "Lean.Expr.getAppArgs", "Lean.Expr.getAppFn", "Lean.Expr.getForallBody", "Lean.Expr.getUsedConstants", "Lean.Expr.isSort", "Lean.IR.findEnvDecl", "Lean.MessageData.compose", "Lean.MessageData.ofExpr", "Lean.MessageData.ofName", "Lean.Meta.MetaM.run'", "Lean.Meta.isInstanceCore", "Lean.Meta.isMatcherCore", "Lean.MonadEnv.getEnv", "Lean.MonadNameGenerator.setNGen", "Lean.Name.components", "Lean.Name.eraseMacroScopes", "Lean.Name.getPrefix", "Lean.Name.getString!", "Lean.Name.isInternal", "Lean.Name.isPrefixOf", "Lean.Name.lt", "Lean.Name.mkSimple", "Lean.Name.mkStr1", "Lean.Name.mkStr2", "Lean.Name.toString", "Lean.PersistentArray.branching", "Lean.PersistentArray.initShift", "Lean.PersistentEnvExtension.toEnvExtension", "Lean.PersistentHashMap.mkEmptyEntriesArray", "Lean.SMap.foldStage2", "Lean.Syntax.getArg", "Lean.Syntax.getId", "Lean.annotation?", "Lean.getConstInfo", "Lean.getOriginalConstKind?", "Lean.githash", "Lean.instantiateMVars", "Lean.isAuxRecursor", "Lean.isCasesOnRecursor", "Lean.isNoConfusion", "Lean.isNoncomputable", "Lean.noncomputableExt", "Lean.stringToMessageData", "Lean.throwError", "Lean.versionString", "List.any", "List.append", "List.contains", "List.drop", "List.elem", "List.filter", "List.foldl", "List.isEmpty", "List.map", "MonadLiftT.monadLift", "Nat.add", "Nat.blt", "Nat.toDigits", "Option.getD", "Option.isSome", "Pure.pure", "Std.HashMap.emptyWithCapacity", "Std.HashMap.get?", "Std.HashMap.insert", "Std.HashSet.contains", "Std.HashSet.emptyWithCapacity", "Std.HashSet.insert", "Std.HashSet.toList", "String.append", "String.foldl", "String.intercalate", "String.ofList", "String.push", "String.toName", "ToString.toString", "UInt16.toNat", "UInt32.toNat", "UInt64.toNat", "UInt8.toNat", "Unit.unit"] : Array String)) ((#[(Prod.mk (α := String) (β := (Array String)) "Bool" ((#["false", "true"] : Array String))), (Prod.mk (α := String) (β := (Array String)) "Lean.Compiler.LCNF.Alt" ((#["alt", "ctorAlt", "default"] : Array String))), (Prod.mk (α := String) (β := (Array String)) "Lean.Compiler.LCNF.Arg" ((#["erased", "fvar", "type"] : Array String))), (Prod.mk (α := String) (β := (Array String)) "Lean.Compiler.LCNF.Cases" ((#["mk"] : Array String))), (Prod.mk (α := String) (β := (Array String)) "Lean.Compiler.LCNF.Code" ((#["let", "fun", "jp", "jmp", "cases", "return", "unreach", "oset", "uset", "sset", "setTag", "inc", "dec", "del"] : Array String))), (Prod.mk (α := String) (β := (Array String)) "Lean.Compiler.LCNF.Decl" ((#["mk"] : Array String))), (Prod.mk (α := String) (β := (Array String)) "Lean.Compiler.LCNF.DeclValue" ((#["code", "extern"] : Array String))), (Prod.mk (α := String) (β := (Array String)) "Lean.Compiler.LCNF.FunDecl" ((#["mk"] : Array String))), (Prod.mk (α := String) (β := (Array String)) "Lean.Compiler.LCNF.LetDecl" ((#["mk"] : Array String))), (Prod.mk (α := String) (β := (Array String)) "Lean.Compiler.LCNF.LetValue" ((#["lit", "erased", "proj", "const", "fvar", "ctor", "oproj", "uproj", "sproj", "fap", "pap", "reset", "reuse", "box", "unbox", "isShared"] : Array String))), (Prod.mk (α := String) (β := (Array String)) "Lean.Compiler.LCNF.LitValue" ((#["nat", "str", "uint8", "uint16", "uint32", "uint64", "usize"] : Array String))), (Prod.mk (α := String) (β := (Array String)) "Lean.Compiler.LCNF.Param" ((#["mk"] : Array String))), (Prod.mk (α := String) (β := (Array String)) "Lean.Compiler.LCNF.Phase" ((#["base", "mono", "impure"] : Array String))), (Prod.mk (α := String) (β := (Array String)) "Lean.Compiler.LCNF.Purity" ((#["pure", "impure"] : Array String))), (Prod.mk (α := String) (β := (Array String)) "Lean.Compiler.LCNF.Signature" ((#["mk"] : Array String))), (Prod.mk (α := String) (β := (Array String)) "Lean.ConstantInfo" ((#["axiomInfo", "defnInfo", "thmInfo", "opaqueInfo", "quotInfo", "inductInfo", "ctorInfo", "recInfo"] : Array String))), (Prod.mk (α := String) (β := (Array String)) "Lean.ConstantKind" ((#["defn", "thm", "axiom", "opaque", "quot", "induct", "ctor", "recursor"] : Array String))), (Prod.mk (α := String) (β := (Array String)) "Lean.ConstantVal" ((#["mk"] : Array String))), (Prod.mk (α := String) (β := (Array String)) "Lean.ConstructorVal" ((#["mk"] : Array String))), (Prod.mk (α := String) (β := (Array String)) "Lean.DefinitionSafety" ((#["unsafe", "safe", "partial"] : Array String))), (Prod.mk (α := String) (β := (Array String)) "Lean.DefinitionVal" ((#["mk"] : Array String))), (Prod.mk (α := String) (β := (Array String)) "Lean.Expr" ((#["bvar", "fvar", "mvar", "sort", "const", "app", "lam", "forallE", "letE", "lit", "mdata", "proj"] : Array String))), (Prod.mk (α := String) (β := (Array String)) "Lean.FVarId" ((#["mk"] : Array String))), (Prod.mk (α := String) (β := (Array String)) "Lean.InductiveVal" ((#["mk"] : Array String))), (Prod.mk (α := String) (β := (Array String)) "Lean.Name" ((#["anonymous", "str", "num"] : Array String))), (Prod.mk (α := String) (β := (Array String)) "Lean.NameGenerator" ((#["mk"] : Array String))), (Prod.mk (α := String) (β := (Array String)) "List" ((#["nil", "cons"] : Array String))), (Prod.mk (α := String) (β := (Array String)) "Option" ((#["none", "some"] : Array String))), (Prod.mk (α := String) (β := (Array String)) "Prod" ((#["mk"] : Array String)))] : Array (Prod String (Array String)))) ((#["Array", "BEq", "Bind", "Char", "Hashable", "IO", "Lean.AxiomVal", "Lean.BinderInfo", "Lean.Compiler.InlineAttributeKind", "Lean.Compiler.LCNF.CompilerM", "Lean.Compiler.LCNF.CompilerM.State", "Lean.Compiler.LCNF.CtorInfo", "Lean.ConstMap", "Lean.Core.Context", "Lean.Core.CoreM", "Lean.Core.State", "Lean.DelayedMetavarAssignment", "Lean.Elab.Command.CommandElab", "Lean.Elab.Command.CommandElabM", "Lean.Elab.Command.Context", "Lean.Elab.Command.State", "Lean.Elab.Term.Context", "Lean.Elab.Term.State", "Lean.Elab.Term.TermElabM", "Lean.EnvExtension", "Lean.EnvExtension.AsyncMode", "Lean.Environment", "Lean.EnvironmentHeader", "Lean.Exception", "Lean.ExternAttrData", "Lean.FVarIdMap", "Lean.FVarIdSet", "Lean.IR.Decl", "Lean.LMVarId", "Lean.Level", "Lean.LevelMetavarDecl", "Lean.Literal", "Lean.LocalDecl", "Lean.LocalInstance", "Lean.MData", "Lean.MVarId", "Lean.MessageData", "Lean.Meta.AbstractMVarsResult", "Lean.Meta.Config", "Lean.Meta.ConfigWithKey", "Lean.Meta.Context", "Lean.Meta.DefEqCacheKey", "Lean.Meta.DefEqContext", "Lean.Meta.ExprConfigCacheKey", "Lean.Meta.FunInfo", "Lean.Meta.InfoCacheKey", "Lean.Meta.MetaM", "Lean.Meta.PostponedEntry", "Lean.Meta.State", "Lean.Meta.SynthInstanceCacheKey", "Lean.MetavarDecl", "Lean.ModuleIdx", "Lean.MonadEnv", "Lean.MonadError", "Lean.MonadMCtx", "Lean.MonadNameGenerator", "Lean.NameSet", "Lean.OpaqueVal", "Lean.ParserDescr", "Lean.PersistentArrayNode", "Lean.PersistentEnvExtension", "Lean.PersistentEnvExtensionState", "Lean.PersistentHashMap.Entry", "Lean.PersistentHashMap.Node", "Lean.QuotVal", "Lean.RecursorVal", "Lean.ReducibilityHints", "Lean.SMap", "Lean.Syntax", "Lean.TagDeclarationExtension", "Lean.TheoremVal", "Monad", "MonadLiftT", "Nat", "Pure", "Std.HashMap", "Std.HashSet", "String", "ToString", "UInt16", "UInt32", "UInt64", "UInt8", "USize", "Unit"] : Array String)) ((#["Applicative.toPure", "Array.casesOn", "Array.mkEmpty", "Bool.casesOn", "Char.ofNat", "EIO", "EmptyCollection.emptyCollection", "Eq", "Eq.casesOn", "Eq.ndrec", "Eq.refl", "Eq.symm", "False", "False.elim", "HEq", "HEq.refl", "IO.RealWorld", "Inhabited", "Inhabited.default", "Inhabited.mk", "Lean.Compiler.LCNF.Alt.casesOn", "Lean.Compiler.LCNF.Arg.casesOn", "Lean.Compiler.LCNF.Cases.casesOn", "Lean.Compiler.LCNF.Code.casesOn", "Lean.Compiler.LCNF.CompilerM.State.mk", "Lean.Compiler.LCNF.CtorInfo.casesOn", "Lean.Compiler.LCNF.Decl.toSignature", "Lean.Compiler.LCNF.Decl.value", "Lean.Compiler.LCNF.DeclValue.casesOn", "Lean.Compiler.LCNF.FunDecl.casesOn", "Lean.Compiler.LCNF.LCtx.mk", "Lean.Compiler.LCNF.LetDecl.fvarId", "Lean.Compiler.LCNF.LetDecl.type", "Lean.Compiler.LCNF.LetDecl.value", "Lean.Compiler.LCNF.LetValue.casesOn", "Lean.Compiler.LCNF.LitValue.casesOn", "Lean.Compiler.LCNF.Param.borrow", "Lean.Compiler.LCNF.Param.fvarId", "Lean.Compiler.LCNF.Param.type", "Lean.Compiler.LCNF.Purity.ctorIdx", "Lean.Compiler.LCNF.Signature.levelParams", "Lean.Compiler.LCNF.Signature.name", "Lean.Compiler.LCNF.Signature.params", "Lean.Compiler.LCNF.Signature.safe", "Lean.Compiler.LCNF.Signature.type", "Lean.ConstantInfo.casesOn", "Lean.ConstantInfo.ctorIdx", "Lean.ConstantInfo.rec", "Lean.ConstantKind.casesOn", "Lean.ConstantVal.type", "Lean.ConstructorVal.induct", "Lean.ConstructorVal.numFields", "Lean.ConstructorVal.numParams", "Lean.ConstructorVal.toConstantVal", "Lean.Core.instAddMessageContextCoreM", "Lean.Core.instInhabitedCoreM", "Lean.Core.instMonadCoreM", "Lean.Core.instMonadEnvCoreM", "Lean.Core.instMonadLiftIOCoreM", "Lean.Core.instMonadNameGeneratorCoreM", "Lean.Core.instMonadRefCoreM", "Lean.DefinitionSafety.casesOn", "Lean.DefinitionVal.safety", "Lean.Elab.Command.instAddErrorMessageContextCommandElabM", "Lean.Elab.Command.instInhabitedCommandElabM", "Lean.Elab.Command.instMonadCommandElabM", "Lean.Elab.Command.instMonadEnvCommandElabM", "Lean.Elab.Command.instMonadExceptOfExceptionCommandElabM", "Lean.Elab.Command.instMonadLiftTIOCommandElabM", "Lean.Elab.Command.instMonadRefCommandElabM", "Lean.Elab.MonadMacroAdapter.toMonadQuotation", "Lean.Elab.Term.instAddErrorMessageContextTermElabM", "Lean.Elab.Term.instMonadMacroAdapterTermElabM", "Lean.Elab.Term.instMonadTermElabM", "Lean.Expr.casesOn", "Lean.Expr.ctorIdx", "Lean.Expr.instBEq", "Lean.Expr.instHashable", "Lean.Expr.rec", "Lean.FVarId.casesOn", "Lean.FVarId.name", "Lean.InductiveVal.ctors", "Lean.InductiveVal.isRec", "Lean.InductiveVal.numIndices", "Lean.InductiveVal.numParams", "Lean.Level.instBEq", "Lean.Level.instToString", "Lean.LocalContext.mk", "Lean.Meta.Cache.mk", "Lean.Meta.Context.mk", "Lean.Meta.Diagnostics.mk", "Lean.Meta.State.mk", "Lean.Meta.instBEqDefEqCacheKey", "Lean.Meta.instBEqExprConfigCacheKey", "Lean.Meta.instBEqInfoCacheKey", "Lean.Meta.instBEqSynthInstanceCacheKey", "Lean.Meta.instHashableDefEqCacheKey", "Lean.Meta.instHashableExprConfigCacheKey", "Lean.Meta.instHashableInfoCacheKey", "Lean.Meta.instHashableSynthInstanceCacheKey", "Lean.Meta.instInhabitedConfigWithKey", "Lean.Meta.instMonadEnvMetaM", "Lean.Meta.instMonadMCtxMetaM", "Lean.Meta.instMonadMetaM", "Lean.MetavarContext.mk", "Lean.MonadError.mk", "Lean.MonadQuotation.toMonadRef", "Lean.Name.instBEq", "Lean.ParserDescr.binary", "Lean.ParserDescr.cat", "Lean.ParserDescr.const", "Lean.ParserDescr.node", "Lean.ParserDescr.symbol", "Lean.PersistentArray.mk", "Lean.PersistentArrayNode.node", "Lean.PersistentHashMap.Node.entries", "Lean.PersistentHashMap.mk", "Lean.instAddErrorMessageContextOfAddMessageContextOfMonad", "Lean.instBEqBinderInfo", "Lean.instBEqFVarId", "Lean.instBEqLevelMVarId", "Lean.instBEqLiteral", "Lean.instBEqMVarId", "Lean.instEmptyCollectionFVarIdMap", "Lean.instEmptyCollectionFVarIdSet", "Lean.instHashableFVarId", "Lean.instHashableLevelMVarId", "Lean.instHashableMVarId", "Lean.instHashableName", "Lean.instMonadEnvOfMonadLift", "Lean.instMonadExceptOfExceptionCoreM", "Lean.instMonadMCtxOfMonadLift", "List.casesOn", "List.instBEq", "List.toArray", "Monad.toBind", "Nat.hasNotBit", "Nat.land", "Nat.ne_of_beq_eq_false", "Nat.shiftRight", "OfNat.ofNat", "Option.casesOn", "Prod.fst", "Prod.snd", "ReaderT.instApplicativeOfMonad", "ReaderT.instMonadExceptOf", "ReaderT.instMonadLift", "StateRefT'", "StateRefT'.instMonad", "StateRefT'.instMonadExceptOf", "StateRefT'.instMonadLift", "Std.HashMap.instEmptyCollection", "String.instInhabited", "USize.toNat", "eq_of_heq", "instBEqOfDecidableEq", "instDecidableEqBool", "instDecidableEqChar", "instDecidableEqNat", "instDecidableEqString", "instInhabitedBool", "instInhabitedList", "instInhabitedProd", "instMonadEIO", "instMonadLiftT", "instMonadLiftTOfMonadLift", "instOfNatNat", "instToStringList", "instToStringNat", "instToStringString", "ite", "noConfusion_of_Nat"] : Array String))

#eval LexLeanExtract.main ((#["Reasoning.Main.triageLevel", "Reasoning.Main.gatewayLevel", "Reasoning.Main.triageSteps", "Reasoning.Main.planLeft", "Reasoning.Main.doseLevel", "Reasoning.Main.gradeLevel", "Reasoning.Main.screenDose", "Reasoning.Main.spendLeft"] : Array String)) ((#["Reasoning.Budget", "Reasoning.Clinic", "Reasoning.Main", "Reasoning.Planner", "Reasoning.Screening"] : Array String))
