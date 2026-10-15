"""One-time: lift the language-1.2 runtime texts out of `backend/semantic.rs`, byte for byte, into the Lean
sources the oracle compiles (scratch tool, never in the release tree).

Each module is what a generated language-1.2 module splices, with the generated module's own header and options
in place of its namespace: the text is the same Rust literal, so what it elaborates to is what every generated
module elaborated, now owned by one module instead of one copy per module."""
import os, re, sys

HEADER_OPTIONS = (
    "set_option autoImplicit false\n"
    "set_option maxRecDepth 100000\n"
    "set_option maxHeartbeats 1000000000\n"
    "set_option linter.constructorNameAsVariable false\n"
)

# module, the Rust item the text is read from, the runtime modules it imports
MODULES = [
    ("Portable", "portable_runtime_1_2", []),
    ("Collections", "COLLECTIONS_RUNTIME", []),
    ("Models", "MODELS_RUNTIME", []),
    ("Reasoning", "REASONING_RUNTIME", ["Portable", "Collections"]),
]


def literal(source, opener):
    start = source.index(opener) + len(opener)
    end = source.index('"#', start)
    return source[start:end]


def portable_1_2(source):
    """`portable_runtime_1_2` is not a literal: it is `portable_runtime()` with every definition exposed. This is
    its transformation, replayed exactly (`semantic.rs`, `fn portable_runtime_1_2`)."""
    frozen = literal(source, 'fn portable_runtime() -> &\'static str {\n    r#"')
    out = []
    for line in frozen.split("\n")[:-1] if frozen.endswith("\n") else frozen.split("\n"):
        if line.startswith("@[noinline] public def "):
            out.append("@[expose, noinline] public def " + line[len("@[noinline] public def "):] + "\n")
        elif line.startswith("public def "):
            out.append("@[expose] public def " + line[len("public def "):] + "\n")
        else:
            out.append(line + "\n")
    return "".join(out)


def text_of(source, item):
    if item == "portable_runtime_1_2":
        return portable_1_2(source)
    return literal(source, f'const {item}: &str = r#"')


def main():
    semantic_rs, out = sys.argv[1], sys.argv[2]
    source = open(semantic_rs, encoding="utf-8").read()
    os.makedirs(os.path.join(out, "LexLeanRuntime"), exist_ok=True)
    for module, item, imports in MODULES:
        text = "module\npublic import Init\n"
        for imported in imports:
            text += f"public import LexLeanRuntime.{imported}\n"
        text += HEADER_OPTIONS
        text += text_of(source, item)
        path = os.path.join(out, "LexLeanRuntime", f"{module}.lean")
        with open(path, "w", encoding="utf-8", newline="\n") as handle:
            handle.write(text)
        print(module, item, len(text_of(source, item).encode()), "runtime bytes")


if __name__ == "__main__":
    main()
