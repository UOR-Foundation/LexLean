"""Compare two directories of native library sources declaration by declaration.

A module's data is compared as the environment it states: the imports (the regenerated one also privately
imports `Lean`, the generic runtime's import, which SPEC.md §17.10 allows), and every declaration in order,
with the kernel type and value compared as trees, not as rows of a shared node table, whose numbering depends on
the order the nodes were met. Two things may differ and are reported: nothing else may.

  * A sparse `casesOn` is made by the elaborator's construction in the regenerated environment, and the
    construction names its binders with fresh hygienic names, so those rows are equal up to the names of bound
    variables (alpha-equivalence), which no kernel judgment sees.

usage: compare.py <original dir> <regenerated dir>
"""
import glob, hashlib, json, os, re, sys


def load(path):
    text = open(path).read()
    return json.loads(re.search(r"\\coredata\{(.*)\}\n\\end\{coremodule\}", text, re.S).group(1))


def tree_hashes(nodes, names):
    """The hash of every node as a tree; with `names` the binder names are part of it, without they are not."""
    out = []
    for node in nodes:
        k = node["k"]
        parts = [k]
        if k == "a":
            parts += [out[node["f"]], out[node["x"]]]
        elif k in ("l", "p"):
            parts += [node["b"], out[node["t"]], out[node["v"]]] + ([node["n"]] if names else [])
        elif k == "e":
            parts += [out[node["t"]], out[node["v"]], out[node["body"]], node["d"]] + ([node["n"]] if names else [])
        elif k == "d":
            parts += [json.dumps(node["m"], sort_keys=True), out[node["x"]]]
        elif k == "j":
            parts += [node["n"], node["s"], node["i"], out[node["x"]]]
        else:
            parts += [json.dumps({key: value for key, value in node.items() if key != "k"}, sort_keys=True)]
        out.append(hashlib.sha256(json.dumps(parts).encode()).hexdigest())
    return out


def main():
    original, regenerated = sys.argv[1], sys.argv[2]
    failures = 0
    totals = {"declarations": 0, "nodes": 0, "exact": 0, "alpha": 0}
    for path in sorted(glob.glob(os.path.join(original, "*.lex.tex"))):
        name = os.path.basename(path)
        a = load(path)
        b = load(os.path.join(regenerated, name))
        b["imports"] = [i for i in b["imports"] if i != "Lean"]
        problems = []
        if a["imports"] != b["imports"]:
            problems.append("imports")
        if a["spec"] != b["spec"]:
            problems.append("spec")
        if [r["name"] for r in a["declarations"]] != [r["name"] for r in b["declarations"]]:
            problems.append("declaration names or order")
        named_a, named_b = tree_hashes(a["nodes"], True), tree_hashes(b["nodes"], True)
        plain_a, plain_b = tree_hashes(a["nodes"], False), tree_hashes(b["nodes"], False)
        exact = alpha = 0
        if "declaration names or order" not in problems:
            for ra, rb in zip(a["declarations"], b["declarations"]):
                rest_a = {k: v for k, v in ra.items() if k not in ("type", "value")}
                rest_b = {k: v for k, v in rb.items() if k not in ("type", "value")}
                # the equation data states two roots of the node table, which are trees too
                for rest, table in ((rest_a, named_a), (rest_b, named_b)):
                    if "eqn_info" in rest:
                        info = dict(rest["eqn_info"])
                        info["type"], info["value"] = table[info["type"]], table[info["value"]]
                        rest["eqn_info"] = info
                if rest_a != rest_b:
                    problems.append(f"{ra['name']}: {sorted(k for k in set(rest_a) | set(rest_b) if rest_a.get(k) != rest_b.get(k))}")
                    continue
                roots = [("type", ra["type"], rb["type"])] + ([("value", ra["value"], rb["value"])] if "value" in ra else [])
                if all(named_a[x] == named_b[y] for _, x, y in roots):
                    exact += 1
                elif all(plain_a[x] == plain_b[y] for _, x, y in roots) and "sparse" in ra:
                    alpha += 1
                else:
                    problems.append(f"{ra['name']}: kernel data differs")
        totals["declarations"] += len(a["declarations"])
        totals["nodes"] += len(a["nodes"])
        totals["exact"] += exact
        totals["alpha"] += alpha
        print(f"{name[:-8]:<11} declarations={len(a['declarations']):<5} equal={exact:<5} equal-up-to-binder-names={alpha:<4} {'OK' if not problems else 'DIFFERS'}")
        for problem in problems[:5]:
            print("  ", problem)
        failures += bool(problems)
    print(f"total declarations={totals['declarations']} nodes={totals['nodes']} equal={totals['exact']} equal-up-to-binder-names={totals['alpha']} differing modules={failures}")
    sys.exit(1 if failures else 0)


main()
