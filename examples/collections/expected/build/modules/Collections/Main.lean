module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import Collections.Measure
public import Collections.Tables
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace Collections.Main

@[expose] public def ancestors (parents : List ((Prod (Nat) (Nat)))) : (Prod (List ((Prod (Nat) (Nat)))) (Bool)) := (LexLeanCollections.iterateUntil ((fun (state : List ((Prod (Nat) (Nat)))) => (let derived : List ((Prod (Nat) (Nat))) := (LexLeanCollections.setFold ((fun (outer : List ((Prod (Nat) (Nat)))) (left : (Prod (Nat) (Nat))) => (LexLeanCollections.setFold ((fun (inner : List ((Prod (Nat) (Nat)))) (right : (Prod (Nat) (Nat))) => (if (Nat.beq ((left).2) ((right).1)) then (LexLeanCollections.setInsert (inner) (((left).1, (right).2)) : List ((Prod (Nat) (Nat)))) else inner))) (outer) (state) : List ((Prod (Nat) (Nat)))))) (state) (state) : List ((Prod (Nat) (Nat)))); (if (Nat.beq ((LexLeanCollections.setSize (derived) : Nat)) ((LexLeanCollections.setSize (state) : Nat))) then Option.none else Option.some (derived))))) (8) (parents) : (Prod (List ((Prod (Nat) (Nat)))) (Bool)))

@[expose] public def family : List ((Prod (Nat) (Nat))) := ([(1, 2), (2, 3), (3, 4)] : List ((Prod (Nat) (Nat))))

public theorem table_slots : (Collections.Tables.buildTable (("x" :: ("y" :: ("x2" :: ([] : List (String)))))) = ([("x", 0), ("x2", 2), ("y", 1)] : List (Prod (String) (Nat)))) := by
  decide

public theorem table_lookup : ((LexLeanCollections.mapLookup (Collections.Tables.buildTable (("a" :: ("b" :: ([] : List (String)))))) ("b") : Option (Nat)) = Option.some (1)) := by
  decide

public theorem call_order : (Collections.Tables.callOrder = Option.some (("main" :: ("emit" :: ("parse" :: ("report" :: ("lex" :: ([] : List (String))))))))) := by
  decide

public theorem parse_reaches_lex : (Collections.Tables.reachableFromParse = (["lex", "parse"] : List (String))) := by
  decide

public theorem cycle_has_no_order : ((LexLeanCollections.graphTopological (([(1, [2]), (2, [1])] : List (Prod (Nat) (List (Nat))))) : Option (List (Nat))) = Option.none) := by
  decide

public theorem ancestor_closure : (ancestors (family) = (([(1, 2), (1, 3), (1, 4), (2, 3), (2, 4), (3, 4)] : List ((Prod (Nat) (Nat)))), true)) := by
  decide

public theorem bounded_iteration : ((LexLeanCollections.iterate ((fun (value : Nat) => (value + 2))) (3) (1) : Nat) = 7) := by
  decide

public theorem reordered_literals_agree : (([(1, true), (2, false)] : List (Prod (Nat) (Bool))) = ([(1, true), (2, false)] : List (Prod (Nat) (Bool)))) := by
  rfl

public theorem op_map_insert : ((LexLeanCollections.mapInsert (([("a", 1), ("b", 2)] : List (Prod (String) (Nat)))) ("c") (3) : List (Prod (String) (Nat))) = ([("a", 1), ("b", 2), ("c", 3)] : List (Prod (String) (Nat)))) := by
  decide

public theorem op_map_insert_replaces : ((LexLeanCollections.mapInsert (([("a", 1), ("b", 2)] : List (Prod (String) (Nat)))) ("a") (9) : List (Prod (String) (Nat))) = ([("a", 9), ("b", 2)] : List (Prod (String) (Nat)))) := by
  decide

public theorem op_map_remove : ((LexLeanCollections.mapRemove (([("a", 1), ("b", 2)] : List (Prod (String) (Nat)))) ("a") : List (Prod (String) (Nat))) = ([("b", 2)] : List (Prod (String) (Nat)))) := by
  decide

public theorem op_map_remove_absent : ((LexLeanCollections.mapRemove (([("a", 1), ("b", 2)] : List (Prod (String) (Nat)))) ("z") : List (Prod (String) (Nat))) = ([("a", 1), ("b", 2)] : List (Prod (String) (Nat)))) := by
  decide

public theorem op_map_lookup : ((LexLeanCollections.mapLookup (([("a", 1), ("b", 2)] : List (Prod (String) (Nat)))) ("b") : Option (Nat)) = Option.some (2)) := by
  decide

public theorem op_map_lookup_absent : ((LexLeanCollections.mapLookup (([("a", 1), ("b", 2)] : List (Prod (String) (Nat)))) ("z") : Option (Nat)) = Option.none) := by
  decide

public theorem op_map_contains : ((LexLeanCollections.mapContains (([("a", 1), ("b", 2)] : List (Prod (String) (Nat)))) ("a") : Bool) = true) := by
  decide

public theorem op_map_size : ((LexLeanCollections.mapSize (([("a", 1), ("b", 2)] : List (Prod (String) (Nat)))) : Nat) = 2) := by
  decide

public theorem op_map_keys : ((LexLeanCollections.mapKeys (([("a", 1), ("b", 2)] : List (Prod (String) (Nat)))) : List (String)) = ("a" :: ("b" :: ([] : List (String))))) := by
  decide

public theorem op_map_values : ((LexLeanCollections.mapValues (([("a", 1), ("b", 2)] : List (Prod (String) (Nat)))) : List (Nat)) = (1 :: (2 :: ([] : List (Nat))))) := by
  decide

public theorem op_map_entries : ((LexLeanCollections.mapEntries (([("a", 1), ("b", 2)] : List (Prod (String) (Nat)))) : List ((Prod (String) (Nat)))) = (("a", 1) :: (("b", 2) :: ([] : List ((Prod (String) (Nat))))))) := by
  decide

public theorem op_map_fold : ((LexLeanCollections.mapFold ((fun (visited : List (String)) (key : String) (_value : Nat) => (key :: visited))) (([] : List (String))) (([("a", 1), ("b", 2)] : List (Prod (String) (Nat)))) : List (String)) = ("b" :: ("a" :: ([] : List (String))))) := by
  decide

public theorem op_set_insert : ((LexLeanCollections.setInsert (([1, 2, 3] : List (Nat))) (0) : List (Nat)) = ([0, 1, 2, 3] : List (Nat))) := by
  decide

public theorem op_set_remove : ((LexLeanCollections.setRemove (([1, 2, 3] : List (Nat))) (2) : List (Nat)) = ([1, 3] : List (Nat))) := by
  decide

public theorem op_set_contains : ((LexLeanCollections.setContains (([1, 2, 3] : List (Nat))) (4) : Bool) = false) := by
  decide

public theorem op_set_size : ((LexLeanCollections.setSize (([1, 2, 3] : List (Nat))) : Nat) = 3) := by
  decide

public theorem op_set_elements : ((LexLeanCollections.setElements (([1, 2, 3] : List (Nat))) : List (Nat)) = (1 :: (2 :: (3 :: ([] : List (Nat)))))) := by
  decide

public theorem op_set_union : ((LexLeanCollections.setUnion (([1, 2, 3] : List (Nat))) (([0, 5] : List (Nat))) : List (Nat)) = ([0, 1, 2, 3, 5] : List (Nat))) := by
  decide

public theorem op_set_intersection : ((LexLeanCollections.setIntersection (([1, 2, 3] : List (Nat))) (([2, 3, 9] : List (Nat))) : List (Nat)) = ([2, 3] : List (Nat))) := by
  decide

public theorem op_set_difference : ((LexLeanCollections.setDifference (([1, 2, 3] : List (Nat))) (([2] : List (Nat))) : List (Nat)) = ([1, 3] : List (Nat))) := by
  decide

public theorem op_set_fold : ((LexLeanCollections.setFold ((fun (visited : List (Nat)) (element : Nat) => (element :: visited))) (([] : List (Nat))) (([1, 2, 3] : List (Nat))) : List (Nat)) = (3 :: (2 :: (1 :: ([] : List (Nat)))))) := by
  decide

public theorem op_list_fold : ((LexLeanCollections.listFold ((fun (visited : List (Nat)) (element : Nat) => (element :: visited))) (([] : List (Nat))) ((5 :: (1 :: (4 :: ([] : List (Nat)))))) : List (Nat)) = (4 :: (1 :: (5 :: ([] : List (Nat)))))) := by
  decide

public theorem op_iterate_zero : ((LexLeanCollections.iterate ((fun (value : Nat) => (value + 2))) (0) (1) : Nat) = 1) := by
  decide

public theorem op_iterate_until_fixed_point : ((LexLeanCollections.iterateUntil ((fun (value : Nat) => (if (Nat.blt (value) (3)) then Option.some ((value + 1)) else Option.none))) (10) (0) : (Prod (Nat) (Bool))) = (3, true)) := by
  decide

public theorem op_iterate_until_exhausted : ((LexLeanCollections.iterateUntil ((fun (value : Nat) => (if (Nat.blt (value) (3)) then Option.some ((value + 1)) else Option.none))) (2) (0) : (Prod (Nat) (Bool))) = (2, false)) := by
  decide

public theorem op_graph_successors : ((LexLeanCollections.graphSuccessors (([(1, [2, 3]), (2, [4]), (3, [4]), (4, [])] : List (Prod (Nat) (List (Nat))))) (1) : List (Nat)) = ([2, 3] : List (Nat))) := by
  decide

public theorem op_graph_successors_absent : ((LexLeanCollections.graphSuccessors (([(1, [2, 3]), (2, [4]), (3, [4]), (4, [])] : List (Prod (Nat) (List (Nat))))) (9) : List (Nat)) = ([] : List (Nat))) := by
  decide

public theorem op_graph_reachable : ((LexLeanCollections.graphReachable (([(1, [2, 3]), (2, [4]), (3, [4]), (4, [])] : List (Prod (Nat) (List (Nat))))) (2) : List (Nat)) = ([2, 4] : List (Nat))) := by
  decide

public theorem op_graph_topological : ((LexLeanCollections.graphTopological (([(1, [2, 3]), (2, [4]), (3, [4]), (4, [])] : List (Prod (Nat) (List (Nat))))) : Option (List (Nat))) = Option.some ((1 :: (2 :: (3 :: (4 :: ([] : List (Nat)))))))) := by
  decide

public theorem key_order_nat : (([0, 9, 10, 11, 100] : List (Nat)) = (LexLeanCollections.listFold ((fun (built : List (Nat)) (element : Nat) => (LexLeanCollections.setInsert (built) (element) : List (Nat)))) (([] : List (Nat))) ((10 :: (9 :: (100 :: (0 :: (11 :: ([] : List (Nat)))))))) : List (Nat))) := by
  decide

public theorem key_order_int : (([(-100 : Int), (-10 : Int), (-2 : Int), (0 : Int), (3 : Int), (9 : Int), (100 : Int)] : List (Int)) = (LexLeanCollections.listFold ((fun (built : List (Int)) (element : Int) => (LexLeanCollections.setInsert (built) (element) : List (Int)))) (([] : List (Int))) (((3 : Int) :: ((-2 : Int) :: ((0 : Int) :: ((-10 : Int) :: ((100 : Int) :: ((-100 : Int) :: ((9 : Int) :: ([] : List (Int)))))))))) : List (Int))) := by
  decide

public theorem key_order_int8 : (([(-128 : Int8), (-9 : Int8), (-1 : Int8), (0 : Int8), (9 : Int8), (127 : Int8)] : List (Int8)) = (LexLeanCollections.listFold ((fun (built : List (Int8)) (element : Int8) => (LexLeanCollections.setInsert (built) (element) : List (Int8)))) (([] : List (Int8))) (((127 : Int8) :: ((-1 : Int8) :: ((0 : Int8) :: ((-128 : Int8) :: ((9 : Int8) :: ((-9 : Int8) :: ([] : List (Int8))))))))) : List (Int8))) := by
  decide

public theorem key_order_int16 : (([(-32768 : Int16), (-10 : Int16), (-1 : Int16), (0 : Int16), (10 : Int16), (32767 : Int16)] : List (Int16)) = (LexLeanCollections.listFold ((fun (built : List (Int16)) (element : Int16) => (LexLeanCollections.setInsert (built) (element) : List (Int16)))) (([] : List (Int16))) (((32767 : Int16) :: ((-1 : Int16) :: ((0 : Int16) :: ((-32768 : Int16) :: ((10 : Int16) :: ((-10 : Int16) :: ([] : List (Int16))))))))) : List (Int16))) := by
  decide

public theorem key_order_int32 : (([(-2147483648 : Int32), (-100 : Int32), (-1 : Int32), (0 : Int32), (100 : Int32), (2147483647 : Int32)] : List (Int32)) = (LexLeanCollections.listFold ((fun (built : List (Int32)) (element : Int32) => (LexLeanCollections.setInsert (built) (element) : List (Int32)))) (([] : List (Int32))) (((2147483647 : Int32) :: ((-1 : Int32) :: ((0 : Int32) :: ((-2147483648 : Int32) :: ((100 : Int32) :: ((-100 : Int32) :: ([] : List (Int32))))))))) : List (Int32))) := by
  decide

public theorem key_order_int64 : (([(-9223372036854775808 : Int64), (-99 : Int64), (-1 : Int64), (0 : Int64), (99 : Int64), (9223372036854775807 : Int64)] : List (Int64)) = (LexLeanCollections.listFold ((fun (built : List (Int64)) (element : Int64) => (LexLeanCollections.setInsert (built) (element) : List (Int64)))) (([] : List (Int64))) (((9223372036854775807 : Int64) :: ((-1 : Int64) :: ((0 : Int64) :: ((-9223372036854775808 : Int64) :: ((99 : Int64) :: ((-99 : Int64) :: ([] : List (Int64))))))))) : List (Int64))) := by
  decide

public theorem key_order_uint8 : (([(0 : UInt8), (7 : UInt8), (70 : UInt8), (255 : UInt8)] : List (UInt8)) = (LexLeanCollections.listFold ((fun (built : List (UInt8)) (element : UInt8) => (LexLeanCollections.setInsert (built) (element) : List (UInt8)))) (([] : List (UInt8))) (((255 : UInt8) :: ((0 : UInt8) :: ((7 : UInt8) :: ((70 : UInt8) :: ([] : List (UInt8))))))) : List (UInt8))) := by
  decide

public theorem key_order_uint16 : (([(0 : UInt16), (9 : UInt16), (10 : UInt16), (65535 : UInt16)] : List (UInt16)) = (LexLeanCollections.listFold ((fun (built : List (UInt16)) (element : UInt16) => (LexLeanCollections.setInsert (built) (element) : List (UInt16)))) (([] : List (UInt16))) (((65535 : UInt16) :: ((0 : UInt16) :: ((9 : UInt16) :: ((10 : UInt16) :: ([] : List (UInt16))))))) : List (UInt16))) := by
  decide

public theorem key_order_uint32 : (([(0 : UInt32), (99 : UInt32), (100 : UInt32), (4294967295 : UInt32)] : List (UInt32)) = (LexLeanCollections.listFold ((fun (built : List (UInt32)) (element : UInt32) => (LexLeanCollections.setInsert (built) (element) : List (UInt32)))) (([] : List (UInt32))) (((4294967295 : UInt32) :: ((0 : UInt32) :: ((99 : UInt32) :: ((100 : UInt32) :: ([] : List (UInt32))))))) : List (UInt32))) := by
  decide

public theorem key_order_uint64 : (([(0 : UInt64), (9 : UInt64), (10 : UInt64), (18446744073709551615 : UInt64)] : List (UInt64)) = (LexLeanCollections.listFold ((fun (built : List (UInt64)) (element : UInt64) => (LexLeanCollections.setInsert (built) (element) : List (UInt64)))) (([] : List (UInt64))) (((18446744073709551615 : UInt64) :: ((0 : UInt64) :: ((9 : UInt64) :: ((10 : UInt64) :: ([] : List (UInt64))))))) : List (UInt64))) := by
  decide

public theorem key_order_bool : (([false, true] : List (Bool)) = (LexLeanCollections.listFold ((fun (built : List (Bool)) (element : Bool) => (LexLeanCollections.setInsert (built) (element) : List (Bool)))) (([] : List (Bool))) ((true :: (false :: ([] : List (Bool))))) : List (Bool))) := by
  decide

public theorem key_order_string : ((["", "Z", "a", "a b", "ab", "z", "ß", "é", "日本", "😀"] : List (String)) = (LexLeanCollections.listFold ((fun (built : List (String)) (element : String) => (LexLeanCollections.setInsert (built) (element) : List (String)))) (([] : List (String))) (("z" :: ("é" :: ("a" :: ("Z" :: ("ß" :: ("日本" :: ("😀" :: ("ab" :: ("" :: ("a b" :: ([] : List (String))))))))))))) : List (String))) := by
  decide

public theorem key_order_product : (([(1, "a"), (1, "z"), (2, "B"), (2, "a")] : List ((Prod (Nat) (String)))) = (LexLeanCollections.listFold ((fun (built : List ((Prod (Nat) (String)))) (element : (Prod (Nat) (String))) => (LexLeanCollections.setInsert (built) (element) : List ((Prod (Nat) (String)))))) (([] : List ((Prod (Nat) (String))))) (((2, "a") :: ((1, "z") :: ((2, "B") :: ((1, "a") :: ([] : List ((Prod (Nat) (String))))))))) : List ((Prod (Nat) (String))))) := by
  decide

end Collections.Main
