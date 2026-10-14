"""One-time conversion of the exporter's raw JSON into canonical `lexlean/core-module/2` data and the
`.lex.tex` source modules of the preservation-library project (scratch tool, never in the release tree)."""
import json, os, sys

base = os.path.dirname(os.path.abspath(__file__))
MODS = ["LexLeanText"]
PREFIX = "LexLeanText"


def canon(v):
    return json.dumps(v, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def meta_value(raw):
    (k, v), = raw.items()
    return {"t": k, "v": v}


def convert_node(n):
    if n["k"] == "d":
        n = dict(n)
        n["m"] = [{"n": key, "v": meta_value(val)} for key, val in n["m"]]
    return n


def opt(v):
    return [] if v is None else [v]


EXPORT_DIR = os.environ.get("EXPORT", "export")
flags = os.environ.get("FLAGS", base)
SPARSE = {x["name"]: x for x in json.load(open(os.path.join(flags, "sparse.json")))}
COMPILED = set(open(os.path.join(flags, "compiled.txt")).read().split("\n"))


def convert_decl(d):
    d = dict(d)
    tags = list(d.get("tags", []))
    if d["name"] in SPARSE:
        tags.append("sparse_cases_on")
    order = ["protected", "aux_recursor", "elab_as_elim", "defeq", "backward_defeq", "recursive", "sparse_cases_on"]
    tags = sorted(set(tags), key=order.index)
    if tags:
        d["tags"] = tags
    if d["kind"] == "inductive":
        d["compiled"] = True
    elif d["kind"] in ("abbrev", "definition"):
        if d["name"] in COMPILED:
            d["compiled"] = True
        if d["generated"]:
            # the compiler's partial implementation records are ordinary rows of their own safety
            d["generated"] = False
            d["safety"] = "partial"
    if "matcher" in d:
        m = dict(d["matcher"])
        if m.get("u_elim_pos") is None:
            m.pop("u_elim_pos", None)
        m["discrs"] = [opt(x) for x in m["discrs"]]
        d["matcher"] = m
    if "eqn_info" in d:
        e = dict(d["eqn_info"])
        e["perms"] = [[opt(c) for c in row] for row in e["perms"]]
        d["eqn_info"] = e
    return d


def reachable(nodes, roots):
    seen = set()
    stack = list(roots)
    while stack:
        i = stack.pop()
        if i in seen:
            continue
        seen.add(i)
        n = nodes[i]
        k = n["k"]
        if k == "a":
            stack += [n["f"], n["x"]]
        elif k in ("l", "p"):
            stack += [n["t"], n["v"]]
        elif k == "e":
            stack += [n["t"], n["v"], n["body"]]
        elif k in ("j", "d"):
            stack.append(n["x"])
    return seen


def convert(j):
    nodes = [convert_node(n) for n in j["nodes"]]
    decls = [convert_decl(d) for d in j["declarations"]]
    proofs = reachable(nodes, [d["value"] for d in decls if d["kind"] == "theorem"])
    # a node that a type also reaches is not closed proof data: keep only nodes no type reaches
    typed = reachable(nodes, [d["type"] for d in decls])
    return {
        "spec": "lexlean/core-module/2",
        "imports": list(dict.fromkeys(j["imports"])),
        "nodes": nodes,
        "proof_nodes": sorted(proofs - typed),
        "declarations": decls,
    }


TITLES = {
    "LexLeanText": "LexLean text laws",
}


def source(module, doc):
    name = module
    imports = [i[len(PREFIX) + 1:] for i in doc["imports"] if i.startswith(PREFIX + ".")]
    lines = ["\\begin{lexlean}{%s}" % name,
             "\\useglossary{lexlean.std.nat@1.2.0}",
             "\\useglossary{preservation.titles@1.0.0}"]
    for i in imports:
        lines.append("\\importmodule{%s}" % i)
    lines.append("\\title{%s}" % TITLES[name])
    lines.append("\\begin{coremodule}")
    lines.append("\\coredata{" + canon(doc) + "}")
    lines.append("\\end{coremodule}")
    lines.append("\\end{lexlean}")
    return "\n".join(lines) + "\n"


def main():
    out = sys.argv[1]
    os.makedirs(out, exist_ok=True)
    for m in MODS:
        j = json.load(open(os.path.join(base, EXPORT_DIR, f"{m}.json")))
        doc = convert(j)
        text = source(m, doc)
        open(f"{out}/{m}.lex.tex", "w").write(text)
        print(m, len(text))


if __name__ == "__main__":
    main()
