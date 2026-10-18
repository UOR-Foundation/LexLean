# bootstrap

The compiler's own components as language-1.2 source (SPEC.md §17.23, issue 92).

`src/AxiomOutput.lex.tex` and `src/TokenAudit.lex.tex` import the standard
library's `Text` (`stdlib/src/Text.lex.tex`) as a source package:
`lexlean.toml` declares `stdlib` beside this project in the workspace
(`lexlean-workspace.toml`), the lock pins it by digest, and its modules are
linked, verified, and replayed with this project under their own prefix
`LexLeanStd` (SPEC.md §10.5). Its definitions are ordinary definitions here, so
this project republishes none of the standard library's production roots. The
`.lex.tex` notation has no comment form (§17.21.5), so this note is the header
the sources cannot carry.
