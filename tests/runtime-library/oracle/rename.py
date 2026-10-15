"""One-time: the native proof libraries name the runtime as it was, a copy inside each generated calculus module
(`LexLeanTarget.TargetSemantics.LexLeanRuntime.index`). The runtime is now one set of native modules
(`LexLeanRuntime.index`), so the libraries' kernel data is renamed (scratch tool, never in the release tree).

What changes, and nothing else: every name that names a runtime copy, `LexLeanTarget.<M>.<runtime namespace>.x`,
becomes `<runtime namespace>.x` wherever a name is written (a constant or projection node, a declaration's name,
its mutual group). Node tables keep their numbering, so a row's kernel terms are its old terms with the names
substituted.

Two copies of one runtime definition can have had the same equation lemma realized twice, once in each of two
library modules. Identified, the two rows declare one name; the tool requires them to be the same row (kernel type
and value as trees up to the names of bound variables, which differ only by the hygiene of the module each copy was
elaborated in; every other field equal) and reports them. Nothing is dropped: the pinned Lean imports a reserved
equation-lemma name declared by two modules (tested by elaboration and `leanchecker` replay). Binder names are
data the kernel never reads and are kept as they are.

usage: rename.py <library dir> <module prefix> [<earlier library dir> <prefix>]...
Exits non-zero on anything it cannot account for."""
import hashlib, json, os, re, sys

COPY = re.compile(r"LexLeanTarget\.(?:TargetSemantics|RustSemantics|Certification|Gnaf|TargetSyntax|RustSyntax)\."
                  r"(LexLean(?:Runtime|Collections|Models|Reasoning))\.")
OPEN, CLOSE = "\\coredata{", "}\n\\end{coremodule}"
NAME_FIELDS = {("n", "c"), ("n", "j"), ("s", "j"), ("name", None), ("all", None)}


def canon(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def read(path):
    text = open(path, encoding="utf-8").read()
    start, end = text.index(OPEN) + len(OPEN), text.rindex(CLOSE)
    payload = text[start:end]
    data = json.loads(payload)
    if canon(data) != payload:
        sys.exit(f"{path}: the payload is not in the canonical form this tool writes")
    return text[:start], data, text[end:]


def modules_of(library):
    """The registry's module list: `library.toml` inside the project, or beside it (the preservation library)."""
    inside = os.path.join(library, "library.toml")
    path = inside if os.path.exists(inside) else os.path.join(os.path.dirname(library), "library.toml")
    registry = open(path, encoding="utf-8").read()
    return re.findall(r'"([A-Za-z0-9_.]+)"', registry.split("modules", 1)[1].split("\n", 1)[0])


def source(library, prefix, module):
    return os.path.join(library, "src", module[len(prefix) + 1:].replace(".", "/") + ".lex.tex")


# The walk's pairing of the calculus modules' constants as they were with the constants as they are generated now
# (`walk.py`): a name it pairs, or a name derived from one (an equation lemma, a splitter, a private realization of
# either), is renamed by it; any other name of a runtime copy by the runtime rule.
MAPPING = {}
PRIVATE = re.compile(r"^(_private\.[A-Za-z0-9_.]+?\.#\d+\.)(.*)$")


def renamed_name(name):
    match = PRIVATE.match(name)
    prefix, rest = (match.group(1), match.group(2)) if match else ("", name)
    parts = rest.split(".")
    for cut in range(len(parts), 0, -1):
        head = ".".join(parts[:cut])
        if head in MAPPING:
            return prefix + ".".join([MAPPING[head]] + parts[cut:])
    return prefix + COPY.sub(r"\1.", rest) if COPY.search(rest) else name


def rename(value, touched, key=None, node=None):
    if isinstance(value, dict):
        kind = value["k"] if "k" in value else node
        return {k: rename(v, touched, k, kind) for k, v in value.items()}
    if isinstance(value, list):
        return [rename(v, touched, key, node) for v in value]
    if isinstance(value, str) and (key, node) in NAME_FIELDS:
        renamed = renamed_name(value)
        if renamed != value:
            touched.append((key, node, value, renamed))
        return renamed
    if isinstance(value, str) and COPY.search(value):
        touched.append((key, node, value, value))
    return value


def node_children(n):
    k = n["k"]
    if k == "a":
        return [n["f"], n["x"]]
    if k in ("l", "p"):
        return [n["t"], n["v"]]
    if k == "e":
        return [n["t"], n["v"], n["body"]]
    if k in ("j", "d"):
        return [n["x"]]
    return []


def tree_hashes(nodes):
    """The hash of every node as a tree, without the names of bound variables (a binder's `n`), which no kernel
    judgment sees: the copies were elaborated in different modules, whose hygienic binder names differ."""
    out = []
    for n in nodes:
        rest = {k: v for k, v in n.items()
                if (k not in ("f", "x", "t", "v", "body") or n["k"] in ("n", "t"))
                and not (k == "n" and n["k"] in ("l", "p", "e"))}
        parts = [[out[c] for c in node_children(n)], canon(rest)]
        out.append(hashlib.sha256(canon(parts).encode()).hexdigest())
    return out


def comparable(declaration, hashes):
    """A row with every node reference replaced by the tree it roots."""
    row = json.loads(canon(declaration))
    row["type"] = hashes[row["type"]]
    if "value" in row:
        row["value"] = hashes[row["value"]]
    if "eqn_info" in row:
        row["eqn_info"]["type"] = hashes[row["eqn_info"]["type"]]
        row["eqn_info"]["value"] = hashes[row["eqn_info"]["value"]]
    for field in (row.get("inductive") or {}).get("structure", {}).get("fields", []):
        if "auto_param" in field:
            field["auto_param"] = hashes[field["auto_param"]]
    return row


def main():
    args = sys.argv[1:]
    if args and args[0] == "--mapping":
        walked = json.load(open(args[1]))
        MAPPING.update(walked["mapping"])
        # An old name the walk paired with two new names: a runtime copy's matcher that a calculus definition
        # reused. Every library use of it is in that definition's equations or in theorems that unfold the
        # definition, so it is renamed to the matcher the definition now has of its own.
        MAPPING.update(walked["chosen"])
        unresolved = sorted(set(walked["ambiguous"]) - set(walked["chosen"]))
        if unresolved:
            sys.exit(f"ambiguous pairings with no choice: {unresolved}")
        args = args[2:]
    libraries = [(args[0], args[1])] + list(zip(args[2::2], args[3::2]))
    # earlier libraries first, so that a row of this library is compared with every earlier declaration
    order = libraries[1:] + libraries[:1]
    declared = {}
    for index, (library, prefix) in enumerate(order):
        write = index == len(order) - 1
        for module in modules_of(library):
            path = source(library, prefix, module)
            head, data, tail = read(path)
            touched = []
            data = rename(data, touched)
            for key, node, old, _ in touched:
                if (key, node) not in NAME_FIELDS:
                    sys.exit(f"{module}: a runtime name is written where this tool renames none: {key} {node} {old}")
            touched = [entry for entry in touched if entry[2] != entry[3]]
            hashes = tree_hashes(data["nodes"])
            shared = []
            originals = {new: old for _, _, old, new in touched}
            for d in data["declarations"]:
                row = comparable(d, hashes)
                original = originals.get(d["name"], d["name"])
                # Only a collision the rename makes is this tool's: two libraries that already declared one name
                # (they are never imported together) are as they were.
                if d["name"] in declared and declared[d["name"]][2] != original:
                    owner, earlier, _ = declared[d["name"]]
                    if not re.search(r"\.eq_[0-9]+$", d["name"]):
                        sys.exit(f"{module}: `{d['name']}` is also declared by `{owner}` and is not an equation lemma")
                    if row != earlier:
                        sys.exit(f"{module}: `{d['name']}` is declared by `{owner}` as another row")
                    shared.append(f"{d['name']} (also {owner})")
                else:
                    declared.setdefault(d["name"], (module, row, original))
            if write:
                open(path, "w", encoding="utf-8", newline="\n").write(head + canon(data) + tail)
                names = sorted({new for _, _, _, new in touched})
                print(f"{module}: renamed {len(touched)} name occurrences ({len(names)} distinct)"
                      + (f"; the same row as an earlier module declares: {shared}" if shared else ""))


if __name__ == "__main__":
    main()
