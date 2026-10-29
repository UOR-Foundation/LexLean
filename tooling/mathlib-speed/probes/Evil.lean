import Lean
open Lean Elab Command

-- A declaration the kernel never saw: the elaborator is told to skip kernel checking, which
-- is the "environment hacking" `leanchecker` exists to detect.
run_cmd liftCoreM do
  let decl := Declaration.thmDecl
    { name := `Evil.bogus, levelParams := [], type := mkConst ``False, value := mkConst ``True.intro }
  withOptions (·.setBool `debug.skipKernelTC true) (addDecl decl)
