# Semantic JSON depth fixtures

`SM-12` reads these authored language-1.2 modules in a temporary project with
`max_scope_depth = 1024`. The semantic JSON reader still has its documented
128-level nesting limit: 122 `cons` cells check, while 123 fail with `LLT4001`
and advice to use smaller definitions joined by `prim append`.

`later-list-123.lex.tex` puts the same failure after another declaration; the
diagnostic's source span covers the module body and does not pretend to name
the nested term. `split-list-200.lex.tex` demonstrates the advised workaround
with two 100-cell definitions. The case also calls the semantic JSON reader
directly and checks that its existing canonical and closed-schema refusals
remain intact. No verification claim is made by these checking fixtures.
