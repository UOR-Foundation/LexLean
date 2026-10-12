module
public meta import Lean
set_option autoImplicit false
set_option linter.unusedVariables false
universe u v

public meta def Commands.«elab» : Lean.Elab.Command.CommandElab :=
  (fun (stx : Lean.Syntax) => (Lean.Elab.Command.liftCoreM (α := Unit) (Bind.bind (m := Lean.Core.CoreM) (α := Lean.ConstantInfo) (β := Unit) (Lean.getConstInfo (m := Lean.Core.CoreM) (Lean.Syntax.getId (Lean.Syntax.getArg stx 1))) (fun (info : Lean.ConstantInfo) => (Pure.pure (f := Lean.Core.CoreM) (α := Unit) (Unit.unit))))))

syntax (name := Commands.require) "lexlean_require " ident " : " term : command
attribute [command_elab Commands.require] Commands.«elab»

public meta def Commands.run : (Lean.Elab.Command.CommandElabM Unit) :=
  (Lean.Elab.Command.liftCoreM (α := Unit) (Bind.bind (m := Lean.Core.CoreM) (α := Lean.ConstantInfo) (β := Unit) (Lean.getConstInfo (m := Lean.Core.CoreM) ``Nat.add) (fun (info : Lean.ConstantInfo) => (Pure.pure (f := Lean.Core.CoreM) (α := Unit) (Unit.unit)))))

public meta def Commands.«show» (n : Lean.Name) : String :=
  (Lean.Name.toString n)

public meta def Commands.showPlain (n : Lean.Name) : String :=
  (Lean.Name.toString n (Bool.false))

public meta def Commands.settle : (Lean.Core.CoreM Unit) :=
  (Lean.Meta.MetaM.run' (α := Unit) (Pure.pure (f := Lean.Meta.MetaM) (α := Unit) (Unit.unit)))

lexlean_require Nat.add : (Nat → (Nat → Nat))

#eval Commands.run
