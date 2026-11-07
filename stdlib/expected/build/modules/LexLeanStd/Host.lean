module
public import Init
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanStd.Host

public inductive FailureKind where
  | notFound
  | alreadyExists
  | denied
  | other

public structure Failure where
  kind : FailureKind
  message : ByteArray

public inductive EntryKind where
  | file
  | directory
  | symlink
  | other

public structure Entry where
  kind : EntryKind
  size : Nat
  identity : Option ((Prod (Nat) (Nat)))

public structure Child where
  program : String
  arguments : List (String)
  directory : String
  environment : List ((Prod (String) (String)))
  output : String
  errors : String

public structure Waited where
  finished : List ((Prod (Nat) (Option (Int32))))
  clock : Nat

public structure Start where
  arguments : List (ByteArray)
  directory : Except (Failure) (ByteArray)
  executable : Except (Failure) (ByteArray)
  environment : List ((Prod (ByteArray) (ByteArray)))
  terminal : Bool

public inductive Request where
  | read (_ : String)
  | write (_ : String) (_ : ByteArray)
  | list (_ : String)
  | stat (_ : String)
  | createDir (_ : String)
  | removeFile (_ : String)
  | removeTree (_ : String)
  | rename (_ : String) (_ : String)
  | canonical (_ : String)
  | lock (_ : String)
  | spawn (_ : Child)
  | wait (_ : List (Nat)) (_ : Nat)
  | stop (_ : Nat)
  | exit (_ : UInt8) (_ : ByteArray) (_ : ByteArray)

public inductive Event where
  | start (_ : Start)
  | read (_ : Except (Failure) (ByteArray))
  | finished (_ : Except (Failure) (Unit))
  | listed (_ : Except (Failure) (List (ByteArray)))
  | stat (_ : Except (Failure) (Entry))
  | canonical (_ : Except (Failure) (ByteArray))
  | spawned (_ : Except (Failure) ((Prod (Nat) (Nat))))
  | waited (_ : Except (Failure) (Waited))

end LexLeanStd.Host
