#!/bin/bash
cd /tmp/claude-0/-home-user-LexLean/9ca8b44d-09dc-5ac5-bf84-20869a767d08/scratchpad/tre
export PATH=$HOME/.elan/toolchains/leanprover--lean4---v4.32.1/bin:$PATH
S=/tmp/claude-0/-home-user-LexLean/9ca8b44d-09dc-5ac5-bf84-20869a767d08/scratchpad
FILES="Base DefsBody PatLemmas Comp Comp2 Comp3 Comp4 Comp5 Comp6 Comp7 Comp8 Step1 Step2 Step3 Step4 Step5 Rel1 Rel2 Rel3 Env1 Env2 Loop Env3 Env4 Glue1 Glue2 Glue3 Glue4 Glue5 Glue6 Fun Fun2 Fun3 ${EXTRA:-}"
: > All.lean
for f in $FILES; do cat $f.lean >> All.lean; done
LEAN_PATH=$S/chkl/out lean All.lean 2>&1 | cut -c1-700 | grep -v "^$" | grep -v "linter\|Variable name\|binding can be\|simp argument\|Hint\|simp only\|simp_all\|changedVals" | head -${LINES_MAX:-60}
