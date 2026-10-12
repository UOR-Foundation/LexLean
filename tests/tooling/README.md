# Tooling fixtures

Committed compiler-tooling projects (SPEC.md §17.20). A tooling module is
accepted only in a project the compiler itself embeds, so nothing here is built
through the command line: `cargo xtask check-golden` and
`cargo xtask check-reproducibility` render each fixture with tooling enabled
(`repo_conformance::tooling`), and the `tooling` suite (`TL-01`..`TL-11`) plants
one defect at a time in a copy of each.

| Fixture | What it holds |
| --- | --- |
| `core` | The term language over `Nat`, `List`, `Option`, `Prod`, `Array`, `String`, and `Bool`: literals, calls with explicit type arguments, constructors, fields, `let`, lambda, application, `if`, flat matches (with a default branch), licensed `partial` recursion, and a mutual group. |
| `commands` | A command registration with its elaborator of the abbreviated type `Lean.Elab.Command.CommandElab`, an invocation and an evaluation as semantic data, calls with a trailing `optParam` default and with the empty structure instance, and name literals. |
| `traversal` | A mutual traversal of the real `Lean.Compiler.LCNF.Code`, `FunDecl`, `Cases`, and `Alt` family, whose `Code` is indexed by `Purity` (some of its constructors exist only at `Purity.impure`). |

Each fixture is a project (`lexlean.toml`, the Lake files every project has,
and `src/Main.lex.tex`) and commits no lock: the gates compute it in memory, so
no committed lock can drift from the compiler-semantics ID. `expected/build/` is
the published build tree; `cargo xtask check-golden --write` is its only writer.

The registry rows are the pinned Lean's own. Their constructor lists, binder
kinds, defaults, and index requirements were read from the pinned Lean's
environment, and each carries the toolchain source file that defines it and that
file's SHA-256 (`TL-03` compares every digest with the pinned toolchain's file).
The `plumbing` row of a fixture is the set of constants the pinned Lean's
elaboration of its generated module uses beyond the declared ones. What the
registry says is checked twice: by the compiler for consistency (`TL-03` to
`TL-09`), and by Lean, which elaborates every generated module without a message
and refuses a registry that misstates a signature or a constructor list that its
tool is consistent with (`TL-10`).
