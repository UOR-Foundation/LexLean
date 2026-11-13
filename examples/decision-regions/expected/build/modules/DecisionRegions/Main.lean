module
public import Init
public import LexLeanRuntime.Regions
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace DecisionRegions.Main

@[expose] public def answerIdle : Nat := 0

@[expose] public def answerHeat : Nat := 1

@[expose] public def answerCool : Nat := 2

@[expose] public def answerDry : Nat := 3

public structure Request where
  temperature : Nat
  humidity : Nat
  occupied : Nat

@[expose] public def cooling (request : Request) : Bool := ((Nat.beq ((request).occupied) (1)) && (Nat.blt (29) ((request).temperature)))

@[expose] public def heating (request : Request) : Bool := ((Nat.beq ((request).occupied) (1)) && (Nat.ble ((request).temperature) (15)))

@[expose] public def drying (request : Request) : Bool := (Nat.blt (70) ((request).humidity))

@[expose] public def judge (request : Request) : Nat := (if cooling (request) then answerCool else (if heating (request) then answerHeat else (if drying (request) then answerDry else answerIdle)))

@[expose] public def breaksCoolWhenHot (request : Request) : Bool := ((((((Nat.ble ((request).temperature) (50)) && (Nat.ble ((request).humidity) (100))) && (Nat.blt ((request).occupied) (2))) && (Nat.beq ((request).occupied) (1))) && (Nat.blt (29) ((request).temperature))) && (!(Nat.beq (judge (request)) (answerCool))))

public theorem coolWhenHot (request : Request) : (breaksCoolWhenHot (request) = false) := by
  exact LexLeanRegions.holds [(LexLeanRegions.Rule.mk [(LexLeanRegions.Cond.mk 2 4 1), (LexLeanRegions.Cond.mk 0 2 29)] 2), (LexLeanRegions.Rule.mk [(LexLeanRegions.Cond.mk 2 4 1), (LexLeanRegions.Cond.mk 0 1 15)] 1), (LexLeanRegions.Rule.mk [(LexLeanRegions.Cond.mk 1 2 70)] 3)] 0 (LexLeanRegions.Property.mk true [(LexLeanRegions.Cond.mk 0 1 50), (LexLeanRegions.Cond.mk 1 1 100), (LexLeanRegions.Cond.mk 2 0 2), (LexLeanRegions.Cond.mk 2 4 1), (LexLeanRegions.Cond.mk 0 2 29)] 2) [(0, 50), (0, 100), (0, 1)] (by decide +kernel) [(request).temperature, (request).humidity, (request).occupied] rfl

@[expose] public def breaksNoHeatWhenEmpty (request : Request) : Bool := ((((Nat.ble ((request).temperature) (50)) && (Nat.ble ((request).humidity) (100))) && (Nat.beq ((request).occupied) (0))) && (Nat.beq (judge (request)) (answerHeat)))

public theorem noHeatWhenEmpty (request : Request) : (breaksNoHeatWhenEmpty (request) = false) := by
  exact LexLeanRegions.holds [(LexLeanRegions.Rule.mk [(LexLeanRegions.Cond.mk 2 4 1), (LexLeanRegions.Cond.mk 0 2 29)] 2), (LexLeanRegions.Rule.mk [(LexLeanRegions.Cond.mk 2 4 1), (LexLeanRegions.Cond.mk 0 1 15)] 1), (LexLeanRegions.Rule.mk [(LexLeanRegions.Cond.mk 1 2 70)] 3)] 0 (LexLeanRegions.Property.mk false [(LexLeanRegions.Cond.mk 0 1 50), (LexLeanRegions.Cond.mk 1 1 100), (LexLeanRegions.Cond.mk 2 4 0)] 1) [(0, 50), (0, 100), (0, 0)] (by decide +kernel) [(request).temperature, (request).humidity, (request).occupied] rfl

@[expose] public def breaksNoCoolWhenEmpty (request : Request) : Bool := ((((Nat.ble ((request).temperature) (50)) && (Nat.ble ((request).humidity) (100))) && (Nat.beq ((request).occupied) (0))) && (Nat.beq (judge (request)) (answerCool)))

public theorem noCoolWhenEmpty (request : Request) : (breaksNoCoolWhenEmpty (request) = false) := by
  exact LexLeanRegions.holds [(LexLeanRegions.Rule.mk [(LexLeanRegions.Cond.mk 2 4 1), (LexLeanRegions.Cond.mk 0 2 29)] 2), (LexLeanRegions.Rule.mk [(LexLeanRegions.Cond.mk 2 4 1), (LexLeanRegions.Cond.mk 0 1 15)] 1), (LexLeanRegions.Rule.mk [(LexLeanRegions.Cond.mk 1 2 70)] 3)] 0 (LexLeanRegions.Property.mk false [(LexLeanRegions.Cond.mk 0 1 50), (LexLeanRegions.Cond.mk 1 1 100), (LexLeanRegions.Cond.mk 2 4 0)] 2) [(0, 50), (0, 100), (0, 0)] (by decide +kernel) [(request).temperature, (request).humidity, (request).occupied] rfl

@[expose] public def breaksIdleWhenEmptyAndDry (request : Request) : Bool := (((((Nat.ble ((request).temperature) (50)) && (Nat.ble ((request).humidity) (70))) && (Nat.beq ((request).occupied) (0))) && (!(Nat.beq ((request).temperature) (99)))) && (!(Nat.beq (judge (request)) (answerIdle))))

public theorem idleWhenEmptyAndDry (request : Request) : (breaksIdleWhenEmptyAndDry (request) = false) := by
  exact LexLeanRegions.holds [(LexLeanRegions.Rule.mk [(LexLeanRegions.Cond.mk 2 4 1), (LexLeanRegions.Cond.mk 0 2 29)] 2), (LexLeanRegions.Rule.mk [(LexLeanRegions.Cond.mk 2 4 1), (LexLeanRegions.Cond.mk 0 1 15)] 1), (LexLeanRegions.Rule.mk [(LexLeanRegions.Cond.mk 1 2 70)] 3)] 0 (LexLeanRegions.Property.mk true [(LexLeanRegions.Cond.mk 0 1 50), (LexLeanRegions.Cond.mk 1 1 70), (LexLeanRegions.Cond.mk 2 4 0), (LexLeanRegions.Cond.mk 0 6 99)] 0) [(0, 50), (0, 70), (0, 0)] (by decide +kernel) [(request).temperature, (request).humidity, (request).occupied] rfl

end DecisionRegions.Main
