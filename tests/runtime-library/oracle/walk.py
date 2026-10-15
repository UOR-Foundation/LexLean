"""One-time: how the constants of the calculus modules as they were relate to the constants of the calculus modules
as they are now generated (scratch tool, never in the release tree).

A generated language-1.2 module used to carry its own copy of the runtime; it now imports the runtime modules. The
source of every other declaration is the same text, but Lean shares a matcher (and other auxiliaries) only within a
module, never with an imported one: a definition that reused a matcher of its module's runtime copy now has a
matcher of its own, and the numbering of later matchers moves. So the same definition's kernel terms name some
different auxiliary constants.

The walk establishes that this, and nothing else, changed. Every declaration of the old modules whose name is not an
auxiliary's is paired with its new counterpart (a runtime copy's `LexLeanTarget.<M>.<ns>.x` with the runtime
module's `<ns>.x`), and the two kernel types and values are walked in parallel: they must be the same tree up to the
names of bound variables and of constants, and each pair of differing constant names is recorded. The auxiliaries so
paired are walked in turn, until nothing new is paired. An old name paired with two new names is reported; the
mapping is the result.

usage: walk.py <old export dir> <new export dir> <mapping out>"""
import glob, json, os, re, sys
from collections import defaultdict

COPY = re.compile(r"^LexLeanTarget\.(?:TargetSemantics|RustSemantics|Certification|Gnaf|TargetSyntax|RustSyntax)\."
                  r"(LexLean(?:Runtime|Collections|Models|Reasoning))\.")
AUXILIARY = re.compile(r"(^|\.)(match_\d+|_proof_\d+|proof_\d+|_sparseCasesOn_\d+|_private|_cstage\d*|_unsafe_rec|_sunfold|"
                       r"_unary|_binary|_mutual|eq_\d+|eq_def|splitter|_auxLemma\.\d+|_simp_\d+|_lambda_\d+)(\.|$)")


def load(directory):
    env = {}
    for path in glob.glob(os.path.join(directory, "*.json")):
        data = json.load(open(path))
        module = os.path.basename(path)[:-5]
        for d in data["declarations"]:
            env[d["name"]] = (module, d, data["nodes"])
    return env


def counterpart(name):
    return COPY.sub(r"\1.", name)


class Walk:
    def __init__(self, old, new):
        self.old, self.new = old, new
        self.pairs = defaultdict(set)
        self.problems = []

    def nodes(self, a, an, b, bn, where):
        stack, seen = [(a, b)], set()
        while stack:
            x, y = stack.pop()
            if (x, y) in seen:
                continue
            seen.add((x, y))
            p, q = an[x], bn[y]
            if p["k"] != q["k"]:
                self.problems.append(f"{where}: node kinds {p['k']} and {q['k']}")
                return
            k = p["k"]
            if k == "c":
                if p["u"] != q["u"]:
                    self.problems.append(f"{where}: universe levels of {p['n']} and {q['n']}")
                if p["n"] != q["n"]:
                    self.pairs[p["n"]].add(q["n"])
            elif k == "a":
                stack += [(p["f"], q["f"]), (p["x"], q["x"])]
            elif k in ("l", "p"):
                if p["b"] != q["b"]:
                    self.problems.append(f"{where}: binder kinds")
                stack += [(p["t"], q["t"]), (p["v"], q["v"])]
            elif k == "e":
                if p["d"] != q["d"]:
                    self.problems.append(f"{where}: let flags")
                stack += [(p["t"], q["t"]), (p["v"], q["v"]), (p["body"], q["body"])]
            elif k == "j":
                if p["i"] != q["i"]:
                    self.problems.append(f"{where}: projection indices")
                for key in ("n", "s"):
                    if p[key] != q[key]:
                        self.pairs[p[key]].add(q[key])
                stack.append((p["x"], q["x"]))
            elif k == "d":
                stack.append((p["x"], q["x"]))
            elif {key: v for key, v in p.items()} != {key: v for key, v in q.items()}:
                self.problems.append(f"{where}: leaves {p} and {q}")

    def declaration(self, old_name, new_name):
        _, a, an = self.old[old_name]
        _, b, bn = self.new[new_name]
        if a.get("kind") != b.get("kind"):
            self.problems.append(f"{old_name} -> {new_name}: kinds {a.get('kind')} and {b.get('kind')}")
            return
        self.nodes(a["type"], an, b["type"], bn, f"{old_name} (type)")
        if "value" in a or "value" in b:
            if "value" in a and "value" in b:
                self.nodes(a["value"], an, b["value"], bn, f"{old_name} (value)")
            else:
                self.problems.append(f"{old_name}: one side has a value")


def main():
    old, new = load(sys.argv[1]), load(sys.argv[2])
    walk = Walk(old, new)
    walked = set()
    queue = []
    for name in sorted(old):
        if AUXILIARY.search(name):
            continue
        target = counterpart(name)
        if target not in new:
            walk.problems.append(f"{name}: no counterpart `{target}`")
            continue
        queue.append((name, target))
    while queue:
        pair = queue.pop()
        if pair in walked:
            continue
        walked.add(pair)
        walk.declaration(*pair)
        for a, targets in list(walk.pairs.items()):
            for b in targets:
                if (a, b) not in walked and a in old and b in new:
                    queue.append((a, b))
    mapping = {}
    ambiguous = {}
    for a, targets in walk.pairs.items():
        if len(targets) == 1:
            mapping[a] = next(iter(targets))
        else:
            ambiguous[a] = sorted(targets)
    # the declarations of the old modules no walk reached keep their counterpart by name
    unreached = sorted(name for name in old if name not in mapping and counterpart(name) != name
                       and not any(name == a for a, _ in walked))
    json.dump({"mapping": dict(sorted(mapping.items())), "ambiguous": ambiguous}, open(sys.argv[3], "w"), indent=1)
    renamed = {a: b for a, b in mapping.items() if counterpart(a) != b}
    print(f"walked {len(walked)} declaration pairs; {len(mapping)} names paired, {len(renamed)} of them not by the "
          f"runtime rule; {len(ambiguous)} ambiguous; {len(walk.problems)} problems")
    for a, b in sorted(renamed.items())[:40]:
        print("  ", a, "->", b)
    for a, targets in sorted(ambiguous.items())[:20]:
        print("  AMBIGUOUS", a, targets)
    for problem in walk.problems[:30]:
        print("  PROBLEM", problem)


if __name__ == "__main__":
    main()
