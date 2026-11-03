module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Buffer
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.GnafValidate

public inductive ActionKind where
  | Observation
  | Preprocessing
  | Advice
  | RetainedState
  | Dispatch
  | Fallback
  | Communication
  | Randomness
  | Scheduling
  | Execution

public inductive Charge where
  | ChSteps
  | ChConstant (_ : Nat)
  | ChFree
  | ChUndeclared

public structure Action where
  kind : ActionKind
  charge : Charge

public inductive Boundary where
  | BdComplete
  | BdPreparedState
  | BdPreparedPlan

public inductive OperandSize where
  | OsWeighted
  | OsUnit

public structure Capacity where
  fuel : Nat
  domain : Nat
  systems : Nat
  charge : Nat

public structure Machine where
  fuel : Nat
  capacity : Capacity
  operandSize : OperandSize
  actions : List (Action)
  boundary : Boundary
  prepared : List (ActionKind)

public inductive Carrier where
  | CaGrammar (_ : Nat) (_ : List (ActionKind)) (_ : Nat)
  | CaInternalPlans
  | CaOptimizerOutput
  | CaDiscovered
  | CaCached

public inductive Completeness where
  | CoGrammarEquality
  | CoMissing
  | CoCitesUniverseId
  | CoCitesOptimizer

public inductive Objective where
  | ObScalar
  | ObVector

public inductive ClaimClass where
  | ClExact
  | ClNormalForm
  | ClCanonical
  | ClRepresentationMinimal
  | ClComparisonTheorem
  | ClProfileDefinedComparison
  | ClInputTotal
  | ClGlobalOptimal
  | ClArgminComplete
  | ClParetoOptimal
  | ClFrontierComplete
  | ClPointwiseEnvelopeComplete
  | ClQueryFamilyAnswerComplete
  | ClUseCaseGlobalOptimal
  | ClWorkloadArgminComplete
  | ClWorkloadParetoOptimal
  | ClWorkloadFrontierComplete
  | ClFamilyOptimal
  | ClCompetitiveBound
  | ClCompetitiveOptimal
  | ClAsymptoticBound
  | ClAsymptoticOptimal
  | ClUseCaseClassComplete
  | ClUseCaseClassAnswerComplete
  | ClMaintainedUseCaseClass
  | ClRestrictedUniverseOptimal
  | ClRevisionPreserved
  | ClBestKnown
  | ClMeasuredBestAmongTested
  | ClHeuristicSelected
  | ClInstanceOptimal

public inductive Scope where
  | ScGrammarUniverse
  | ScCalculusPrograms
  | ScRustPrograms

public structure Request where
  domainLength : Nat
  machine : Machine
  carrier : Carrier
  completeness : Completeness
  objective : Objective
  claim : ClaimClass
  scope : Scope

public inductive Rejection where
  | RjEmptyDomain
  | RjInternalPlanUniverse
  | RjOptimizerDefinedUniverse
  | RjDiscoveredUniverse
  | RjCachedUniverse
  | RjMissingCompleteness
  | RjSelfReferentialCompleteness
  | RjOptimizerCompleteness
  | RjBeyondCapacity
  | RjUnitCostOperands
  | RjDuplicateAction (_ : ActionKind)
  | RjUnaccountedAction (_ : ActionKind)
  | RjHiddenCost (_ : ActionKind)
  | RjUnrealizableAction (_ : ActionKind)
  | RjUnboundPreparation (_ : ActionKind)
  | RjStrayPreparedArtifact (_ : ActionKind)
  | RjClaimAlias
  | RjScalarClaimOverPartialOrder
  | RjVectorClaimOverTotalOrder
  | RjUncoveredScope
  | RjUnsupportedClaim

@[expose] public def kindIndex (kind : ActionKind) : Nat := (match kind with | ActionKind.Observation => 0 | ActionKind.Preprocessing => 1 | ActionKind.Advice => 2 | ActionKind.RetainedState => 3 | ActionKind.Dispatch => 4 | ActionKind.Fallback => 5 | ActionKind.Communication => 6 | ActionKind.Randomness => 7 | ActionKind.Scheduling => 8 | ActionKind.Execution => 9)

@[expose] public def kindName (kind : ActionKind) : ByteArray := (match kind with | ActionKind.Observation => (LexLeanRuntime.utf8Encode ("observation") : ByteArray) | ActionKind.Preprocessing => (LexLeanRuntime.utf8Encode ("preprocessing") : ByteArray) | ActionKind.Advice => (LexLeanRuntime.utf8Encode ("advice") : ByteArray) | ActionKind.RetainedState => (LexLeanRuntime.utf8Encode ("retained_state") : ByteArray) | ActionKind.Dispatch => (LexLeanRuntime.utf8Encode ("dispatch") : ByteArray) | ActionKind.Fallback => (LexLeanRuntime.utf8Encode ("fallback") : ByteArray) | ActionKind.Communication => (LexLeanRuntime.utf8Encode ("communication") : ByteArray) | ActionKind.Randomness => (LexLeanRuntime.utf8Encode ("randomness") : ByteArray) | ActionKind.Scheduling => (LexLeanRuntime.utf8Encode ("scheduling") : ByteArray) | ActionKind.Execution => (LexLeanRuntime.utf8Encode ("execution") : ByteArray))

@[expose] public def performed (kind : ActionKind) : Bool := (match kind with | ActionKind.Observation => true | ActionKind.Preprocessing => false | ActionKind.Advice => false | ActionKind.RetainedState => false | ActionKind.Dispatch => true | ActionKind.Fallback => true | ActionKind.Communication => false | ActionKind.Randomness => false | ActionKind.Scheduling => false | ActionKind.Execution => true)

@[expose] public def preparation (kind : ActionKind) : Bool := (match kind with | ActionKind.Observation => false | ActionKind.Preprocessing => true | ActionKind.Advice => true | ActionKind.RetainedState => true | ActionKind.Dispatch => false | ActionKind.Fallback => false | ActionKind.Communication => false | ActionKind.Randomness => false | ActionKind.Scheduling => false | ActionKind.Execution => false)

@[expose] public def kindEq (left : ActionKind) (right : ActionKind) : Bool := (Nat.beq (kindIndex (left)) (kindIndex (right)))

@[expose] public def render (rejection : Rejection) : Except (ByteArray) (ByteArray) := (match rejection with | Rejection.RjEmptyDomain => LexLeanStd.Buffer.finish (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.new (256)) ((LexLeanRuntime.utf8Encode ("{\"kind\":\"empty_domain\"") : ByteArray))) ((LexLeanRuntime.utf8Encode ("}") : ByteArray))) | Rejection.RjInternalPlanUniverse => LexLeanStd.Buffer.finish (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.new (256)) ((LexLeanRuntime.utf8Encode ("{\"kind\":\"internal_plan_universe\"") : ByteArray))) ((LexLeanRuntime.utf8Encode ("}") : ByteArray))) | Rejection.RjOptimizerDefinedUniverse => LexLeanStd.Buffer.finish (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.new (256)) ((LexLeanRuntime.utf8Encode ("{\"kind\":\"optimizer_defined_universe\"") : ByteArray))) ((LexLeanRuntime.utf8Encode ("}") : ByteArray))) | Rejection.RjDiscoveredUniverse => LexLeanStd.Buffer.finish (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.new (256)) ((LexLeanRuntime.utf8Encode ("{\"kind\":\"discovered_universe\"") : ByteArray))) ((LexLeanRuntime.utf8Encode ("}") : ByteArray))) | Rejection.RjCachedUniverse => LexLeanStd.Buffer.finish (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.new (256)) ((LexLeanRuntime.utf8Encode ("{\"kind\":\"cached_universe\"") : ByteArray))) ((LexLeanRuntime.utf8Encode ("}") : ByteArray))) | Rejection.RjMissingCompleteness => LexLeanStd.Buffer.finish (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.new (256)) ((LexLeanRuntime.utf8Encode ("{\"kind\":\"missing_completeness\"") : ByteArray))) ((LexLeanRuntime.utf8Encode ("}") : ByteArray))) | Rejection.RjSelfReferentialCompleteness => LexLeanStd.Buffer.finish (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.new (256)) ((LexLeanRuntime.utf8Encode ("{\"kind\":\"self_referential_completeness\"") : ByteArray))) ((LexLeanRuntime.utf8Encode ("}") : ByteArray))) | Rejection.RjOptimizerCompleteness => LexLeanStd.Buffer.finish (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.new (256)) ((LexLeanRuntime.utf8Encode ("{\"kind\":\"optimizer_completeness\"") : ByteArray))) ((LexLeanRuntime.utf8Encode ("}") : ByteArray))) | Rejection.RjBeyondCapacity => LexLeanStd.Buffer.finish (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.new (256)) ((LexLeanRuntime.utf8Encode ("{\"kind\":\"beyond_capacity\"") : ByteArray))) ((LexLeanRuntime.utf8Encode ("}") : ByteArray))) | Rejection.RjUnitCostOperands => LexLeanStd.Buffer.finish (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.new (256)) ((LexLeanRuntime.utf8Encode ("{\"kind\":\"unit_cost_operands\"") : ByteArray))) ((LexLeanRuntime.utf8Encode ("}") : ByteArray))) | Rejection.RjDuplicateAction kind => LexLeanStd.Buffer.finish (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.new (256)) ((LexLeanRuntime.utf8Encode ("{\"kind\":\"duplicate_action\"") : ByteArray))) ((LexLeanRuntime.utf8Encode (",\"action\":\"") : ByteArray))) (kindName (kind))) ((LexLeanRuntime.utf8Encode ("\"}") : ByteArray))) | Rejection.RjUnaccountedAction kind => LexLeanStd.Buffer.finish (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.new (256)) ((LexLeanRuntime.utf8Encode ("{\"kind\":\"unaccounted_action\"") : ByteArray))) ((LexLeanRuntime.utf8Encode (",\"action\":\"") : ByteArray))) (kindName (kind))) ((LexLeanRuntime.utf8Encode ("\"}") : ByteArray))) | Rejection.RjHiddenCost kind => LexLeanStd.Buffer.finish (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.new (256)) ((LexLeanRuntime.utf8Encode ("{\"kind\":\"hidden_cost\"") : ByteArray))) ((LexLeanRuntime.utf8Encode (",\"action\":\"") : ByteArray))) (kindName (kind))) ((LexLeanRuntime.utf8Encode ("\"}") : ByteArray))) | Rejection.RjUnrealizableAction kind => LexLeanStd.Buffer.finish (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.new (256)) ((LexLeanRuntime.utf8Encode ("{\"kind\":\"unrealizable_action\"") : ByteArray))) ((LexLeanRuntime.utf8Encode (",\"action\":\"") : ByteArray))) (kindName (kind))) ((LexLeanRuntime.utf8Encode ("\"}") : ByteArray))) | Rejection.RjUnboundPreparation kind => LexLeanStd.Buffer.finish (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.new (256)) ((LexLeanRuntime.utf8Encode ("{\"kind\":\"unbound_preparation\"") : ByteArray))) ((LexLeanRuntime.utf8Encode (",\"action\":\"") : ByteArray))) (kindName (kind))) ((LexLeanRuntime.utf8Encode ("\"}") : ByteArray))) | Rejection.RjStrayPreparedArtifact kind => LexLeanStd.Buffer.finish (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.new (256)) ((LexLeanRuntime.utf8Encode ("{\"kind\":\"stray_prepared_artifact\"") : ByteArray))) ((LexLeanRuntime.utf8Encode (",\"action\":\"") : ByteArray))) (kindName (kind))) ((LexLeanRuntime.utf8Encode ("\"}") : ByteArray))) | Rejection.RjClaimAlias => LexLeanStd.Buffer.finish (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.new (256)) ((LexLeanRuntime.utf8Encode ("{\"kind\":\"claim_alias\"") : ByteArray))) ((LexLeanRuntime.utf8Encode ("}") : ByteArray))) | Rejection.RjScalarClaimOverPartialOrder => LexLeanStd.Buffer.finish (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.new (256)) ((LexLeanRuntime.utf8Encode ("{\"kind\":\"scalar_claim_over_partial_order\"") : ByteArray))) ((LexLeanRuntime.utf8Encode ("}") : ByteArray))) | Rejection.RjVectorClaimOverTotalOrder => LexLeanStd.Buffer.finish (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.new (256)) ((LexLeanRuntime.utf8Encode ("{\"kind\":\"vector_claim_over_total_order\"") : ByteArray))) ((LexLeanRuntime.utf8Encode ("}") : ByteArray))) | Rejection.RjUncoveredScope => LexLeanStd.Buffer.finish (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.new (256)) ((LexLeanRuntime.utf8Encode ("{\"kind\":\"uncovered_scope\"") : ByteArray))) ((LexLeanRuntime.utf8Encode ("}") : ByteArray))) | Rejection.RjUnsupportedClaim => LexLeanStd.Buffer.finish (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.writeBytes (LexLeanStd.Buffer.new (256)) ((LexLeanRuntime.utf8Encode ("{\"kind\":\"unsupported_claim\"") : ByteArray))) ((LexLeanRuntime.utf8Encode ("}") : ByteArray))))

@[expose] public def findCharge (actions : List (Action)) (wanted : ActionKind) : Option (Charge) := (LexLeanCollections.listFold ((fun (found : Option (Charge)) (action : Action) => (match found with | Option.some held => Option.some (held) | Option.none => (if kindEq ((action).kind) (wanted) then Option.some ((action).charge) else Option.none)))) (Option.none) (actions) : Option (Charge))

@[expose] public def countKind (actions : List (Action)) (wanted : ActionKind) : Nat := (LexLeanCollections.listFold ((fun (count : Nat) (action : Action) => (if kindEq ((action).kind) (wanted) then (count + 1) else count))) (0) (actions) : Nat)

@[expose] public def hasKind (kinds : List (ActionKind)) (wanted : ActionKind) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (kind : ActionKind) => (if found then true else kindEq (kind) (wanted)))) (false) (kinds) : Bool)

@[expose] public def checkDistinct (actions : List (Action)) : Option (Rejection) := (LexLeanCollections.listFold ((fun (found : Option (Rejection)) (action : Action) => (match found with | Option.some held => Option.some (held) | Option.none => (if (Nat.blt (1) (countKind (actions) ((action).kind))) then Option.some (Rejection.RjDuplicateAction ((action).kind)) else Option.none)))) (Option.none) (actions) : Option (Rejection))

@[expose] public def checkPerformedOne (machine : Machine) (wanted : ActionKind) : Option (Rejection) := (match findCharge ((machine).actions) (wanted) with | Option.none => Option.some (Rejection.RjUnaccountedAction (wanted)) | Option.some charge => (match charge with | Charge.ChSteps => Option.none | Charge.ChConstant _ => Option.some (Rejection.RjHiddenCost (wanted)) | Charge.ChFree => Option.some (Rejection.RjHiddenCost (wanted)) | Charge.ChUndeclared => Option.some (Rejection.RjHiddenCost (wanted))))

@[expose] public def checkPerformed (machine : Machine) : Option (Rejection) := (match checkPerformedOne (machine) (ActionKind.Observation) with | Option.some held => Option.some (held) | Option.none => (match checkPerformedOne (machine) (ActionKind.Dispatch) with | Option.some held => Option.some (held) | Option.none => (match checkPerformedOne (machine) (ActionKind.Fallback) with | Option.some held => Option.some (held) | Option.none => checkPerformedOne (machine) (ActionKind.Execution))))

@[expose] public def checkDeclared (machine : Machine) (action : Action) : Option (Rejection) := (let kind : ActionKind := (action).kind; (if performed (kind) then Option.none else (if preparation (kind) then (match (action).charge with | Charge.ChConstant cost => (if (Nat.blt (0) (cost)) then Option.none else Option.some (Rejection.RjHiddenCost (kind))) | Charge.ChFree => (match (machine).boundary with | Boundary.BdComplete => Option.some (Rejection.RjUnboundPreparation (kind)) | Boundary.BdPreparedState => (if hasKind ((machine).prepared) (kind) then Option.none else Option.some (Rejection.RjUnboundPreparation (kind))) | Boundary.BdPreparedPlan => (if hasKind ((machine).prepared) (kind) then Option.none else Option.some (Rejection.RjUnboundPreparation (kind)))) | Charge.ChSteps => Option.some (Rejection.RjHiddenCost (kind)) | Charge.ChUndeclared => Option.some (Rejection.RjHiddenCost (kind))) else Option.some (Rejection.RjUnrealizableAction (kind)))))

@[expose] public def checkDeclaredAll (machine : Machine) : Option (Rejection) := (LexLeanCollections.listFold ((fun (found : Option (Rejection)) (action : Action) => (match found with | Option.some held => Option.some (held) | Option.none => checkDeclared (machine) (action)))) (Option.none) ((machine).actions) : Option (Rejection))

@[expose] public def checkPrepared (machine : Machine) : Option (Rejection) := (LexLeanCollections.listFold ((fun (found : Option (Rejection)) (kind : ActionKind) => (match found with | Option.some held => Option.some (held) | Option.none => (match findCharge ((machine).actions) (kind) with | Option.none => Option.some (Rejection.RjStrayPreparedArtifact (kind)) | Option.some charge => (match charge with | Charge.ChFree => Option.none | Charge.ChSteps => Option.some (Rejection.RjStrayPreparedArtifact (kind)) | Charge.ChConstant _ => Option.some (Rejection.RjStrayPreparedArtifact (kind)) | Charge.ChUndeclared => Option.some (Rejection.RjStrayPreparedArtifact (kind))))))) (Option.none) ((machine).prepared) : Option (Rejection))

@[expose] public def checkPlanPreparation (request : Request) : Option (Rejection) := (match (request).carrier with | Carrier.CaGrammar _ preparations _ => (LexLeanCollections.listFold ((fun (found : Option (Rejection)) (kind : ActionKind) => (match found with | Option.some held => Option.some (held) | Option.none => (match findCharge (((request).machine).actions) (kind) with | Option.some _ => Option.none | Option.none => Option.some (Rejection.RjUnaccountedAction (kind)))))) (Option.none) (preparations) : Option (Rejection)) | Carrier.CaInternalPlans => Option.none | Carrier.CaOptimizerOutput => Option.none | Carrier.CaDiscovered => Option.none | Carrier.CaCached => Option.none)

@[expose] public def universeSize (plans : Nat) (thresholds : Nat) : Nat := (plans + (LexLeanRuntime.multiply (thresholds) ((LexLeanRuntime.multiply (plans) (plans) : Nat)) : Nat))

@[expose] public def chargeBeyond (machine : Machine) : Bool := (LexLeanCollections.listFold ((fun (over : Bool) (action : Action) => (if over then true else (match (action).charge with | Charge.ChConstant cost => (Nat.blt (((machine).capacity).charge) (cost)) | Charge.ChSteps => false | Charge.ChFree => false | Charge.ChUndeclared => false)))) (false) ((machine).actions) : Bool)

@[expose] public def checkCapacity (request : Request) : Option (Rejection) := (let machine : Machine := (request).machine; (let capacity : Capacity := (machine).capacity; (let systems : Bool := (match (request).carrier with | Carrier.CaGrammar plans _ thresholds => (Nat.blt ((capacity).systems) (universeSize (plans) (thresholds))) | Carrier.CaInternalPlans => false | Carrier.CaOptimizerOutput => false | Carrier.CaDiscovered => false | Carrier.CaCached => false); (if (if (Nat.blt ((capacity).fuel) ((machine).fuel)) then true else (if (Nat.blt ((capacity).domain) ((request).domainLength)) then true else (if systems then true else chargeBeyond (machine)))) then Option.some (Rejection.RjBeyondCapacity) else Option.none))))

@[expose] public def checkClaim (request : Request) : Option (Rejection) := (match (request).claim with | ClaimClass.ClRestrictedUniverseOptimal => Option.some (Rejection.RjClaimAlias) | ClaimClass.ClGlobalOptimal => (match (request).objective with | Objective.ObVector => Option.some (Rejection.RjScalarClaimOverPartialOrder) | Objective.ObScalar => Option.none) | ClaimClass.ClArgminComplete => (match (request).objective with | Objective.ObVector => Option.some (Rejection.RjScalarClaimOverPartialOrder) | Objective.ObScalar => Option.none) | ClaimClass.ClParetoOptimal => (match (request).objective with | Objective.ObScalar => Option.some (Rejection.RjVectorClaimOverTotalOrder) | Objective.ObVector => Option.none) | ClaimClass.ClFrontierComplete => (match (request).objective with | Objective.ObScalar => Option.some (Rejection.RjVectorClaimOverTotalOrder) | Objective.ObVector => Option.none) | ClaimClass.ClExact => Option.some (Rejection.RjUnsupportedClaim) | ClaimClass.ClNormalForm => Option.some (Rejection.RjUnsupportedClaim) | ClaimClass.ClCanonical => Option.some (Rejection.RjUnsupportedClaim) | ClaimClass.ClRepresentationMinimal => Option.some (Rejection.RjUnsupportedClaim) | ClaimClass.ClComparisonTheorem => Option.some (Rejection.RjUnsupportedClaim) | ClaimClass.ClProfileDefinedComparison => Option.some (Rejection.RjUnsupportedClaim) | ClaimClass.ClInputTotal => Option.some (Rejection.RjUnsupportedClaim) | ClaimClass.ClPointwiseEnvelopeComplete => Option.some (Rejection.RjUnsupportedClaim) | ClaimClass.ClQueryFamilyAnswerComplete => Option.some (Rejection.RjUnsupportedClaim) | ClaimClass.ClUseCaseGlobalOptimal => Option.some (Rejection.RjUnsupportedClaim) | ClaimClass.ClWorkloadArgminComplete => Option.some (Rejection.RjUnsupportedClaim) | ClaimClass.ClWorkloadParetoOptimal => Option.some (Rejection.RjUnsupportedClaim) | ClaimClass.ClWorkloadFrontierComplete => Option.some (Rejection.RjUnsupportedClaim) | ClaimClass.ClFamilyOptimal => Option.some (Rejection.RjUnsupportedClaim) | ClaimClass.ClCompetitiveBound => Option.some (Rejection.RjUnsupportedClaim) | ClaimClass.ClCompetitiveOptimal => Option.some (Rejection.RjUnsupportedClaim) | ClaimClass.ClAsymptoticBound => Option.some (Rejection.RjUnsupportedClaim) | ClaimClass.ClAsymptoticOptimal => Option.some (Rejection.RjUnsupportedClaim) | ClaimClass.ClUseCaseClassComplete => Option.some (Rejection.RjUnsupportedClaim) | ClaimClass.ClUseCaseClassAnswerComplete => Option.some (Rejection.RjUnsupportedClaim) | ClaimClass.ClMaintainedUseCaseClass => Option.some (Rejection.RjUnsupportedClaim) | ClaimClass.ClRevisionPreserved => Option.some (Rejection.RjUnsupportedClaim) | ClaimClass.ClBestKnown => Option.some (Rejection.RjUnsupportedClaim) | ClaimClass.ClMeasuredBestAmongTested => Option.some (Rejection.RjUnsupportedClaim) | ClaimClass.ClHeuristicSelected => Option.some (Rejection.RjUnsupportedClaim) | ClaimClass.ClInstanceOptimal => Option.some (Rejection.RjUnsupportedClaim))

@[expose] public def checkScope (request : Request) : Option (Rejection) := (match (request).scope with | Scope.ScGrammarUniverse => Option.none | Scope.ScCalculusPrograms => Option.some (Rejection.RjUncoveredScope) | Scope.ScRustPrograms => Option.some (Rejection.RjUncoveredScope))

@[expose] public def afterClaim (request : Request) : Option (Rejection) := (match checkClaim (request) with | Option.some held => Option.some (held) | Option.none => checkScope (request))

@[expose] public def afterPlans (request : Request) : Option (Rejection) := (match checkPlanPreparation (request) with | Option.some held => Option.some (held) | Option.none => afterClaim (request))

@[expose] public def afterPrepared (request : Request) : Option (Rejection) := (match checkPrepared ((request).machine) with | Option.some held => Option.some (held) | Option.none => afterPlans (request))

@[expose] public def afterDeclared (request : Request) : Option (Rejection) := (match checkDeclaredAll ((request).machine) with | Option.some held => Option.some (held) | Option.none => afterPrepared (request))

@[expose] public def afterPerformed (request : Request) : Option (Rejection) := (match checkPerformed ((request).machine) with | Option.some held => Option.some (held) | Option.none => afterDeclared (request))

@[expose] public def afterDistinct (request : Request) : Option (Rejection) := (match checkDistinct (((request).machine).actions) with | Option.some held => Option.some (held) | Option.none => afterPerformed (request))

@[expose] public def afterOperands (request : Request) : Option (Rejection) := (match ((request).machine).operandSize with | OperandSize.OsUnit => Option.some (Rejection.RjUnitCostOperands) | OperandSize.OsWeighted => afterDistinct (request))

@[expose] public def afterCapacity (request : Request) : Option (Rejection) := (match checkCapacity (request) with | Option.some held => Option.some (held) | Option.none => afterOperands (request))

@[expose] public def afterCompleteness (request : Request) : Option (Rejection) := (match (request).completeness with | Completeness.CoGrammarEquality => afterCapacity (request) | Completeness.CoMissing => Option.some (Rejection.RjMissingCompleteness) | Completeness.CoCitesUniverseId => Option.some (Rejection.RjSelfReferentialCompleteness) | Completeness.CoCitesOptimizer => Option.some (Rejection.RjOptimizerCompleteness))

@[expose] public def afterDomain (request : Request) : Option (Rejection) := (match (request).carrier with | Carrier.CaGrammar _ _ _ => afterCompleteness (request) | Carrier.CaInternalPlans => Option.some (Rejection.RjInternalPlanUniverse) | Carrier.CaOptimizerOutput => Option.some (Rejection.RjOptimizerDefinedUniverse) | Carrier.CaDiscovered => Option.some (Rejection.RjDiscoveredUniverse) | Carrier.CaCached => Option.some (Rejection.RjCachedUniverse))

@[expose] public def firstViolation (request : Request) : Option (Rejection) := (if (Nat.beq ((request).domainLength) (0)) then Option.some (Rejection.RjEmptyDomain) else afterDomain (request))

@[expose] public def validateRequest (request : Request) : Except (ByteArray) (Option (ByteArray)) := (match firstViolation (request) with | Option.none => Except.ok (Option.none) | Option.some rejection => (match render (rejection) with | Except.ok text => Except.ok (Option.some (text)) | Except.error problem => Except.error (problem)))

end LexLeanBoot.GnafValidate
