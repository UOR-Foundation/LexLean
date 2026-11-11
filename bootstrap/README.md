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

`src/StructuralGrammar.lex.tex` imports `PrimitiveScan` for the atoms it reads
and the refusal it returns, and `Buffer` and `Numeral` from the standard library.

`src/SurfaceLex.lex.tex` imports `AxiomOutput`, whose `whiteLength` holds the list of
white space the oracle's `char::is_whitespace` has, and the standard library's `Buffer` and `Text`.

`src/SurfaceDoc.lex.tex` imports only the standard library's `Buffer` and `Text`.

`src/SurfaceGeneric.lex.tex` imports `SurfaceLex` and `SurfaceDoc`, and the standard library's
`Json`, `Numeral`, `Order`, and `Text`: its values are the standard library's `Json`.

`src/SurfaceTooling.lex.tex` (SPEC.md §17.23, `BS-27`) imports `SurfaceGeneric` and `SurfaceLex`,
`AxiomOutput` for the white space of a scalar, and the standard library's `Json`, `Order`, and `Text`.
`src/LexiconLse.lex.tex` (SPEC.md §17.23, `BS-25`) imports the standard library's `Numeral`,
`Order`, and `Text`, and nothing of the bootstrap project: its expressions, universes, and
S-expression trees are sorts of its own, each list of a sort's values a chain in the sort.

`src/LexiconLre.lex.tex` (SPEC.md §17.23, `BS-26`) imports `LexiconLse`, whose tokenizer and
tree reader it reads with, and the standard library's `Order`.
