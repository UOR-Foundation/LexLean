# bootstrap

The compiler's own components as language-1.2 source (SPEC.md §17.23, issue 92).

`src/AxiomOutput.lex.tex`, `src/CalculusCheck.lex.tex`, `src/PrimitiveScan.lex.tex`, `src/RuntimeValidate.lex.tex`,
`src/RustPrint.lex.tex`, `src/RustValidate.lex.tex`, and `src/TokenAudit.lex.tex` import the standard library's `Text`
(`stdlib/src/Text.lex.tex`) as a source package:
`lexlean.toml` declares `stdlib` beside this project in the workspace
(`lexlean-workspace.toml`), the lock pins it by digest, and its modules are
linked, verified, and replayed with this project under their own prefix
`LexLeanStd` (SPEC.md §10.5). Its definitions are ordinary definitions here, so
this project republishes none of the standard library's production roots. The
`.lex.tex` notation has no comment form (§17.21.5), so this note is the header
the sources cannot carry.

`src/RustAst.lex.tex` imports only `RustValidate` (the sorts it extends with the
payloads a printer reads); `src/RustPrint.lex.tex` also imports the standard
library's `Buffer` (`stdlib/src/Buffer.lex.tex`), which it writes through.
`src/GnafValidate.lex.tex` (SPEC.md §17.15, `BS-16`) imports `Buffer` alone and
writes the rejection of a GNAF request through it.

`src/LockText.lex.tex` (SPEC.md §17.23, `BS-31`) imports the standard library's `Numeral`,
`Order`, and `Text`, and writes the canonical lock text of SPEC.md §11.2.

`src/ProjectText.lex.tex` (SPEC.md §17.23, `BS-33`) imports `LockText` (`BS-31`), whose TOML
writers it uses, and the standard library's `Numeral`, `Order`, and `Sha256`, and writes the
canonical project manifest of SPEC.md §10.1 and its digest.
