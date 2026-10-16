module
public import Init
public import LexLeanRuntime.Portable
public import Coverage.Types
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Coverage.Main

@[expose] public def double (n : Nat) : Nat := (n + n)

@[expose] public def natOps (a : Nat) (b : Nat) : Nat := (let q : Nat := (LexLeanRuntime.quotient (a) (b) (7) : Nat); (let r : Nat := (LexLeanRuntime.remainder (a) (b) (0) : Nat); (if ((Nat.beq (q) (0)) && ((Nat.ble (r) (3)) || (!(Nat.blt (a) (b))))) then ((LexLeanRuntime.subtract (a) (b) : Nat) + r) else (LexLeanRuntime.multiply (q) (r) : Nat))))

@[expose] public def intOps (a : Int) (b : Int) : Int := (LexLeanRuntime.subtract ((LexLeanRuntime.multiply (a) ((-3 : Int)) : Int)) ((LexLeanRuntime.negate ((LexLeanRuntime.remainder ((LexLeanRuntime.quotient (a) (b) ((1 : Int)) : Int)) (b) ((0 : Int)) : Int)) : Int)) : Int)

@[expose] public def shapes (w : Nat) (h : Nat) : Nat := (Coverage.Types.area (Coverage.Types.Shape.rect (w) (h)) + (Coverage.Types.area (Coverage.Types.Shape.circle (w)) + Coverage.Types.area (Coverage.Types.Shape.empty)))

@[expose] public def treeTotal (n : Nat) : Nat := Coverage.Types.treeSum (Coverage.Types.Tree.node (Coverage.Types.Tree.leaf) (n) (Coverage.Types.Tree.node (Coverage.Types.Tree.leaf) (5) (Coverage.Types.Tree.leaf)))

@[expose] public def records (a : Nat) (b : Nat) : (Prod (Nat) (Nat)) := (let p : Coverage.Types.Point := ({ x := a, y := b } : Coverage.Types.Point); (((p).x + (Coverage.Types.defaultWeights).scale), Coverage.Types.unbox (Nat) (({ content := (p).y, count := 1 } : Coverage.Types.Box (Nat)))))

@[expose] public def options (v : Option (Nat)) (r : Except (Bool) (Nat)) : Nat := ((match v with | Option.none => 0 | Option.some x => x) + (match r with | Except.ok y => y | Except.error flag => (if flag then 1 else 2)))

@[expose] public def builders (n : Nat) : (Prod (Option (Nat)) (Except (Unit) (Nat))) := (Option.some (Nat.succ (n)), (if (Nat.blt (n) (3)) then Except.ok (Nat.zero) else Except.error (())))

@[expose] public def pairs (p : (Prod (Nat) (Bool))) : (Prod (Bool) (Nat)) := ((p).2, (p).1)

@[expose] public def higher (base : Nat) (offset : Nat) : Nat := (let k : Nat := (base + 1); (Coverage.Types.twice ((fun (x : Nat) => ((LexLeanRuntime.multiply (x) (k) : Nat) + offset))) (base) + (((fun (y : Nat) => (LexLeanRuntime.subtract (y) (k) : Nat)) (base)) + Coverage.Types.twice ((double)) (offset))))

@[expose] public def mapped (values : List (Nat)) : List (Bool) := Coverage.Types.mapList (Nat) (Bool) ((Coverage.Types.isEven)) (values)

@[expose] public def parity (n : Nat) : Bool := Coverage.Types.isOdd (n)

end Coverage.Main
