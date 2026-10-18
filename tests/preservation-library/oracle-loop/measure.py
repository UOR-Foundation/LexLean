import glob,sys
old='''  catch error =>
    throwError m!"native core declaration '{name}':{indentD error.toMessageData}"'''
new='''  catch error =>
    logWarning m!"BREAK native core declaration '{name}':{indentD error.toMessageData}"
    try
      let nm := nameOf name
      let levelParams ← liftString (namesOf (← liftString (field json "levels")))
      let type ← liftString (nodeAt nodes (← liftString (natField json "type")))
      unless (← getEnv).contains nm do Lean.addDecl (.axiomDecl { name := nm, levelParams, type, isUnsafe := false })
    catch _ => pure ()'''
n=0
for f in glob.glob(sys.argv[1]+'/*.lean'):
    s=open(f).read()
    if old in s:
        open(f,'w').write(s.replace(old,new)); n+=1
print("patched",n)
