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
`src/SurfaceModule.lex.tex` (SPEC.md §17.23, `BS-28`) imports `AxiomOutput`, whose scanners of
white space it trims with, and the standard library's `Text`.
`src/Diagnostics.lex.tex` (SPEC.md §17.23, `BS-29`) imports the standard library's `Json`,
`Numeral`, `Order`, and `Text`: it writes, sorts, and caps the canonical diagnostic.

`src/CliOutput.lex.tex` (SPEC.md §17.23, `BS-30`) imports `Diagnostics` (`BS-29`), whose
diagnostic it writes for a terminal, and the standard library's `Numeral` and `Order`.
`src/LockText.lex.tex` (SPEC.md §17.23, `BS-31`) imports the standard library's `Numeral`,
`Order`, and `Text`, and writes the canonical lock text of SPEC.md §11.2.
`src/ContentId.lex.tex` (SPEC.md §17.23, `BS-32`) imports the standard library's `Numeral` and
`Sha256`, and computes the labeled hash recipes of content identity of SPEC.md §21.

`src/ProjectText.lex.tex` (SPEC.md §17.23, `BS-33`) imports `LockText` (`BS-31`), whose TOML
writers it uses, and the standard library's `Numeral`, `Order`, and `Sha256`, and writes the
canonical project manifest of SPEC.md §10.1 and its digest.
`src/ManifestText.lex.tex` (SPEC.md §17.23, `BS-34`) imports the standard library's `Json`,
`Numeral`, `Order`, and `Text`, and writes the build manifest of SPEC.md §21.6.
`src/GrammarProof.lex.tex` (SPEC.md §17.23, `BS-35`) imports `PrimitiveScan`, whose atoms, places,
and refusals it reads and returns, and the standard library's `Text`.
`src/SemanticPrint.lex.tex` (SPEC.md §17.23, `BS-36`) imports `SurfaceGeneric`, whose generic
notation it writes every value without a readable form in, `SurfaceDoc`, `SurfaceLex`, and the
standard library's `Json`, `Order`, and `Text`.
`src/SemanticRead.lex.tex` (SPEC.md §17.23, `BS-37`) imports `SurfaceLex`, whose lexer it reads
the text with, `SurfaceGeneric`, whose generic reader it reads every value without a readable form
with, and the standard library's `Json`, `Order`, and `Text`.
`src/CoreJson.lex.tex` (SPEC.md §17.23, `BS-38`) defines the kernel terms of a core module as
inductives and imports the standard library's `Json`, whose writer it writes each declaration
with, `Numeral`, `Order`, and `Text`, and `SurfaceLex`, whose `quoteText` escapes its strings.
`src/CorePrint.lex.tex` (SPEC.md §17.23, `BS-39`) imports `CoreJson`, whose module types it
prints, `AxiomOutput`, `SurfaceDoc`, `SurfaceGeneric`, `SurfaceLex`, and the standard library's
`Json`, `Numeral`, `Order`, and `Text`.
`src/CoreRead.lex.tex` (SPEC.md §17.23, `BS-40`) imports `CoreJson`, whose module types it
reads to, `SurfaceLex`, whose string reader it reads strings with, `SurfaceGeneric`, whose
`parseGeneric` reads a declaration's members, `AxiomOutput`, and the standard library's `Json`,
`Numeral`, `Order`, and `Text`.
`src/GrammarMath.lex.tex` (SPEC.md §17.23, `BS-42`) imports `PrimitiveScan`, whose atoms, places,
and refusals it reads and returns, and the standard library's `Numeral` and `Text`.
`src/PropositionTerms.lex.tex` (SPEC.md §17.23, `BS-43`) imports `GrammarProof`, whose text tokens
and keyword occurrences it reads and returns, `PrimitiveScan`, and the standard library's `Numeral`.
`src/LexiconSchema.lex.tex` (SPEC.md §17.23, `BS-50`) imports `PrimitiveScan` (`BS-03`), whose
atoms it reads, `LexiconLse` (`BS-25`), whose byte classes it uses, and the standard library's
`Text`: the tables and predicates of the lexicon entry schema of SPEC.md §13.
`src/LseScopes.lex.tex` (SPEC.md §17.23, `BS-51`) imports `LexiconLse` (`BS-25`), whose
expressions it walks, and the standard library's `Order`: the scopes, references, and outer
shape of a semantic expression of SPEC.md §13.8.
`src/LseTypes.lex.tex` (SPEC.md §17.23, `BS-52`) imports `LexiconLse` (`BS-25`), whose
expressions it checks and prints, and the standard library's `Numeral` and `Order`: the
conservative checker of semantic expressions of SPEC.md §13.7.
`src/IrTerm.lex.tex` (SPEC.md §17.23, `BS-65`) imports `LexiconLse` (`BS-25`), whose universes
and binder modes it uses, and the standard library's `Json`, `Numeral`, and `Order`, and writes
the canonical JSON of a linked term of SPEC.md §17.9.
`src/IrProof.lex.tex` (SPEC.md §17.23, `BS-66`) imports `IrTerm` (`BS-65`), whose term JSON and
renumbering it uses, `LexiconLse`, and the standard library's `Json` and `Order`, and writes the
canonical JSON of a linked proof and declaration and checks an axiom policy.
`src/IrDocument.lex.tex` (SPEC.md §17.23, `BS-67`) imports `IrTerm`, `IrProof`, and `ContentId`, and
the standard library's `Json`, `Order`, and `Text`, and writes the canonical JSON of a linked module
and of a linked project, and the project's semantic ID.
`src/SemanticTypes.lex.tex` (SPEC.md §17.23, `BS-68`) imports the standard library's `Json`, `Order`,
and `Text`, and writes the closed types of the semantic module language as the linked IR embeds
them and as diagnostics spell them, and the facts of its primitives and integer families.
`src/SemanticTerms.lex.tex` (SPEC.md §17.23, `BS-69`) imports `SemanticTypes` (`BS-68`) and the standard
library's `Json` and `Order`, and writes the closed terms and proofs of the semantic module language as
the linked IR embeds them.
`src/SemanticDecls.lex.tex` (SPEC.md §17.23, `BS-70`) imports `SemanticTerms` (`BS-69`), `SemanticTypes`
(`BS-68`), and the standard library's `Json` and `Order`, and writes the ordinary declarations of the
semantic module language and its modules as the linked IR embeds them.
`src/SemanticModels.lex.tex` (SPEC.md §17.23, `BS-71`) imports `SemanticTerms`, `SemanticTypes`, and the
standard library's `Json` and `Order`, and writes the declarations of artifacts, contracts,
realizations, evidence, and models as the linked IR embeds them.
`src/SemanticReasoning.lex.tex` (SPEC.md §17.23, `BS-72`) imports `SemanticDecls`, `SemanticModels`,
`SemanticTerms`, `SemanticTypes`, and the standard library's `Json` and `Order`, and writes the
declarations of reasoning and text components and semantic modules of every kind.
