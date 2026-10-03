module
public import Init
public import LexLeanTarget.TargetSyntax
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
namespace LexLeanTarget.TargetOracle

@[expose] public def encodeNats : (items : List (Nat)) -> List (LexLeanTarget.TargetSyntax.Value)
  | List.nil => ([] : List (LexLeanTarget.TargetSyntax.Value))
  | List.cons item rest => (LexLeanTarget.TargetSyntax.Value.nat (item) :: encodeNats (rest))

@[expose] public def encodeInts : (items : List (Int)) -> List (LexLeanTarget.TargetSyntax.Value)
  | List.nil => ([] : List (LexLeanTarget.TargetSyntax.Value))
  | List.cons item rest => (LexLeanTarget.TargetSyntax.Value.int (item) :: encodeInts (rest))

@[expose] public def encodeI8s : (items : List (Int8)) -> List (LexLeanTarget.TargetSyntax.Value)
  | List.nil => ([] : List (LexLeanTarget.TargetSyntax.Value))
  | List.cons item rest => (LexLeanTarget.TargetSyntax.Value.i8 (item) :: encodeI8s (rest))

@[expose] public def encodeBools : (items : List (Bool)) -> List (LexLeanTarget.TargetSyntax.Value)
  | List.nil => ([] : List (LexLeanTarget.TargetSyntax.Value))
  | List.cons item rest => (LexLeanTarget.TargetSyntax.Value.bool (item) :: encodeBools (rest))

@[expose] public def encodeStrings : (items : List (String)) -> List (LexLeanTarget.TargetSyntax.Value)
  | List.nil => ([] : List (LexLeanTarget.TargetSyntax.Value))
  | List.cons item rest => (LexLeanTarget.TargetSyntax.Value.string (item) :: encodeStrings (rest))

@[expose] public def encodeNatStrings : (items : List ((Prod (Nat) (String)))) -> List (LexLeanTarget.TargetSyntax.Value)
  | List.nil => ([] : List (LexLeanTarget.TargetSyntax.Value))
  | List.cons item rest => (LexLeanTarget.TargetSyntax.Value.pair (LexLeanTarget.TargetSyntax.Value.nat ((item).1)) (LexLeanTarget.TargetSyntax.Value.string ((item).2)) :: encodeNatStrings (rest))

@[expose] public def encodeAdjacency : (items : List ((Prod (Nat) (List (Nat))))) -> List (LexLeanTarget.TargetSyntax.Value)
  | List.nil => ([] : List (LexLeanTarget.TargetSyntax.Value))
  | List.cons item rest => (LexLeanTarget.TargetSyntax.Value.pair (LexLeanTarget.TargetSyntax.Value.nat ((item).1)) (LexLeanTarget.TargetSyntax.Value.list (encodeNats ((item).2))) :: encodeAdjacency (rest))

end LexLeanTarget.TargetOracle
