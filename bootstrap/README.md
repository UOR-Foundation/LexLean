# bootstrap

The compiler's own components as language-1.2 source (SPEC.md §17.23, issue 92).

`src/Text.lex.tex` is **temporary**. It is a copy of `stdlib/src/Text.lex.tex`
with the `production` declarations removed, kept here only because a project
reads its own source roots and cannot import another project's modules
(`config::is_project_relative` refuses `..`, and symbolic links are refused).
The `.lex.tex` notation has no comment form (§17.21.5), so this note is the
header the copy cannot carry. `BS-01` asserts on every run that the copy is the
standard library's module without its `production` declarations; delete the
copy, and that assertion, when a project can depend on another.
