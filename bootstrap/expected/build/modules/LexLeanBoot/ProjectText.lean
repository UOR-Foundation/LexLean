module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Numeral
public import LexLeanStd.Order
public import LexLeanStd.Sha256
public import LexLeanBoot.LockText
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.ProjectText

public inductive LexiconSource where
  | builtin (_ : ByteArray)
  | path (_ : ByteArray) (_ : ByteArray)
  | git (_ : ByteArray) (_ : ByteArray) (_ : ByteArray) (_ : ByteArray)

public structure ArtifactSource where
  digest : ByteArray
  location : ByteArray

public structure SourcePackage where
  packageName : ByteArray
  version : ByteArray
  location : ByteArray

public structure Limits where
  maxFileBytes : Nat
  maxTotalSourceBytes : Nat
  maxPrimitiveAtoms : Nat
  maxTokenLatticeEdges : Nat
  maxParseStates : Nat
  maxIrNodes : Nat
  maxScopeDepth : Nat
  maxImportDepth : Nat
  maxDiagnostics : Nat
  maxChildOutputBytes : Nat
  childTimeoutMs : Nat

public structure PdfProvider where
  program : ByteArray
  programSha256 : ByteArray
  versionArgv : List (ByteArray)
  versionStdoutSha256 : ByteArray
  compileArgv : List (ByteArray)
  output : ByteArray
  resources : List (ByteArray)

public structure ProjectConfig where
  projectName : ByteArray
  language : ByteArray
  modulePrefix : ByteArray
  sourceRoots : List (ByteArray)
  entrypoints : List (ByteArray)
  buildRoot : ByteArray
  lockfile : ByteArray
  leanWorkspace : ByteArray
  lexiconSources : List (LexiconSource)
  artifactSources : List (ArtifactSource)
  sourcePackages : List (SourcePackage)
  limits : Limits
  pdf : Option (PdfProvider)

@[expose] public def lit (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def cat (left : ByteArray) (right : ByteArray) : ByteArray := (LexLeanRuntime.append (left) (right) : ByteArray)

@[expose] public def stringLine (key : String) (value : ByteArray) : ByteArray := LexLeanBoot.LockText.stringLine (lit (key)) (value)

@[expose] public def arrayLine (key : String) (values : List (ByteArray)) : ByteArray := LexLeanBoot.LockText.line (lit (key)) (LexLeanBoot.LockText.tomlArray (values))

@[expose] public def digestLine (key : String) (digest : ByteArray) : ByteArray := LexLeanBoot.LockText.digestLine (lit (key)) (digest)

@[expose] public def numberLine (key : String) (value : Nat) : ByteArray := LexLeanBoot.LockText.line (lit (key)) (LexLeanStd.Numeral.decimal (value))

@[expose] public def header (table : String) : ByteArray := LexLeanBoot.LockText.header (lit (table))

@[expose] public def packageOf (source : LexiconSource) : ByteArray := (match source with | LexiconSource.builtin named => named | LexiconSource.path named _ => named | LexiconSource.git named _ _ _ => named)

@[expose] public def sourcesSorted (rows : List (LexiconSource)) : List (LexiconSource) := (LexLeanCollections.listFold ((fun (acc : List (LexiconSource)) (entry : (Prod (ByteArray) (LexiconSource))) => ((entry).2 :: acc))) (([] : List (LexiconSource))) (LexLeanStd.Order.reverseList ((Prod (ByteArray) (LexiconSource))) (LexLeanStd.Order.sortByKey (LexiconSource) (LexLeanStd.Order.reverseList ((Prod (ByteArray) (LexiconSource))) ((LexLeanCollections.listFold ((fun (acc : List ((Prod (ByteArray) (LexiconSource)))) (row : LexiconSource) => ((packageOf (row), row) :: acc))) (([] : List ((Prod (ByteArray) (LexiconSource))))) (rows) : List ((Prod (ByteArray) (LexiconSource)))))))) : List (LexiconSource))

@[expose] public def sourceText (source : LexiconSource) : ByteArray := cat (header ("[[lexicon_source]]")) ((match source with | LexiconSource.builtin named => cat (stringLine ("package") (named)) (stringLine ("kind") (lit ("builtin"))) | LexiconSource.path named location => cat (cat (stringLine ("package") (named)) (stringLine ("kind") (lit ("path")))) (stringLine ("path") (location)) | LexiconSource.git named url revision subdirectory => cat (cat (cat (cat (stringLine ("package") (named)) (stringLine ("kind") (lit ("git")))) (stringLine ("url") (url))) (stringLine ("revision") (revision))) (stringLine ("subdirectory") (subdirectory))))

@[expose] public def sourcesText (rows : List (LexiconSource)) : ByteArray := (match rows with | List.nil => cat (lit ("lexicon_source = []")) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 10))]) | List.cons _ _ => (LexLeanCollections.listFold ((fun (acc : ByteArray) (row : LexiconSource) => cat (acc) (sourceText (row)))) (_root_.ByteArray.mk #[]) (sourcesSorted (rows)) : ByteArray))

@[expose] public def artifactText (artifact : ArtifactSource) : ByteArray := cat (cat (header ("[[artifact_source]]")) (digestLine ("sha256") ((artifact).digest))) (stringLine ("path") ((artifact).location))

@[expose] public def packageText (member : SourcePackage) : ByteArray := cat (cat (cat (cat (header ("[[source_package]]")) (stringLine ("package") ((member).packageName))) (stringLine ("version") ((member).version))) (stringLine ("kind") (lit ("path")))) (stringLine ("path") ((member).location))

@[expose] public def limitsText (given : Limits) : ByteArray := cat (cat (cat (cat (cat (cat (cat (cat (cat (cat (cat (header ("[limits]")) (numberLine ("max_file_bytes") ((given).maxFileBytes))) (numberLine ("max_total_source_bytes") ((given).maxTotalSourceBytes))) (numberLine ("max_primitive_atoms") ((given).maxPrimitiveAtoms))) (numberLine ("max_token_lattice_edges") ((given).maxTokenLatticeEdges))) (numberLine ("max_parse_states") ((given).maxParseStates))) (numberLine ("max_ir_nodes") ((given).maxIrNodes))) (numberLine ("max_scope_depth") ((given).maxScopeDepth))) (numberLine ("max_import_depth") ((given).maxImportDepth))) (numberLine ("max_diagnostics") ((given).maxDiagnostics))) (numberLine ("max_child_output_bytes") ((given).maxChildOutputBytes))) (numberLine ("child_timeout_ms") ((given).childTimeoutMs))

@[expose] public def pdfText (provider : PdfProvider) : ByteArray := cat (cat (cat (cat (cat (cat (cat (cat (header ("[pdf]")) (stringLine ("mode") (lit ("external")))) (stringLine ("program") ((provider).program))) (digestLine ("program_sha256") ((provider).programSha256))) (arrayLine ("version_argv") ((provider).versionArgv))) (digestLine ("version_stdout_sha256") ((provider).versionStdoutSha256))) (arrayLine ("compile_argv") ((provider).compileArgv))) (stringLine ("output") ((provider).output))) (arrayLine ("resources") ((provider).resources))

@[expose] public def canonicalProject (config : ProjectConfig) : ByteArray := (let head : ByteArray := cat (cat (cat (cat (cat (cat (cat (cat (cat (stringLine ("spec") (lit ("lexlean/project/1"))) (stringLine ("name") ((config).projectName))) (stringLine ("language") ((config).language))) (stringLine ("module_prefix") ((config).modulePrefix))) (arrayLine ("source_roots") ((config).sourceRoots))) (arrayLine ("entrypoints") ((config).entrypoints))) (stringLine ("build_root") ((config).buildRoot))) (stringLine ("lockfile") ((config).lockfile))) (stringLine ("lean_workspace") ((config).leanWorkspace))) (stringLine ("lean_toolchain") (LexLeanBoot.LockText.leanToolchain)); (let artifacts : ByteArray := (LexLeanCollections.listFold ((fun (acc : ByteArray) (row : ArtifactSource) => cat (acc) (artifactText (row)))) (_root_.ByteArray.mk #[]) ((config).artifactSources) : ByteArray); (let packages : ByteArray := (LexLeanCollections.listFold ((fun (acc : ByteArray) (row : SourcePackage) => cat (acc) (packageText (row)))) (_root_.ByteArray.mk #[]) ((config).sourcePackages) : ByteArray); (let tail : ByteArray := (match (config).pdf with | Option.some provider => pdfText (provider) | Option.none => _root_.ByteArray.mk #[]); cat (cat (cat (cat (cat (head) (sourcesText ((config).lexiconSources))) (artifacts)) (packages)) (limitsText ((config).limits))) (tail)))))

@[expose] public def configSha256 (config : ProjectConfig) : ByteArray := LexLeanStd.Sha256.sha256Hex (canonicalProject (config))

end LexLeanBoot.ProjectText
