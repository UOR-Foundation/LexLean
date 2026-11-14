# Semantic JSON depth fixtures

`SM-12` reads these authored language-1.2 modules in a temporary project with
`max_scope_depth = 1024`. A semantic module nests as deeply as that bound
allows, in its notation and in the JSON the notation reads to: a list of n
cells nests about n levels.

`list-1000.lex.tex` is a 1000-cell list, past every fixed 128-level JSON
reader: it checks, builds, and has a snapshot whose canonical bytes are written
on a caller's default 2 MiB stack. The refused sources live under `negative/`,
matching the authoring API corpus's classification of intentionally unreadable
modules. `negative/list-1100.lex.tex` is nested past the bound, and is refused
with `LLP2001` naming `max_scope_depth`. `negative/later-list-1100.lex.tex` puts
the same failure after another declaration; the diagnostic's source span
covers the module body and does not pretend to name the nested term.
`split-list-200.lex.tex` shows the workaround the diagnostic still names, two
100-cell definitions joined by `prim append`.

The case also lowers the bound to 150 to show that both readers take it from
the project (a 145-cell list checks, 146 cells are refused by the JSON reader
with `LLT4001`, 148 by the notation with `LLP2001`), calls the semantic JSON
reader directly, and checks that its canonical and closed-schema refusals
remain intact. No verification claim is made by these checking fixtures.
