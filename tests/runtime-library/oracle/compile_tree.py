"""Compile every `.lean` under `<root>/src` into `<root>/out`, in import order, with the pinned Lean (scratch tool).
usage: compile_tree.py <root>"""
import os, re, subprocess, sys, time

root = sys.argv[1]
lean = os.path.expanduser("~/.elan/toolchains/leanprover--lean4---v4.32.1/bin/lean")
src, out = os.path.join(root, "src"), os.path.join(root, "out")
modules = {}
for directory, _, files in os.walk(src):
    for name in files:
        if name.endswith(".lean"):
            path = os.path.join(directory, name)
            module = os.path.relpath(path, src)[:-5].replace("/", ".")
            imports = re.findall(r"^(?:public )?import (\S+)", open(path).read(), re.M)
            modules[module] = (path, [i for i in imports if i != "Init" and i != "Lean"])
done, order = set(), []
while len(done) < len(modules):
    ready = sorted(m for m, (_, imports) in modules.items() if m not in done and all(i in done or i not in modules for i in imports))
    if not ready:
        sys.exit(f"import cycle among {sorted(set(modules) - done)}")
    for m in ready:
        order.append(m)
        done.add(m)
env = dict(os.environ, LEAN_PATH=out)
for m in order:
    olean = os.path.join(out, m.replace(".", "/") + ".olean")
    os.makedirs(os.path.dirname(olean), exist_ok=True)
    started = time.time()
    result = subprocess.run([lean, "-R", src, "-o", olean, modules[m][0]], env=env, capture_output=True, text=True)
    print(f"{m} rc={result.returncode} seconds={time.time() - started:.0f} output={len(result.stdout) + len(result.stderr)}")
    if result.returncode != 0 or result.stdout.strip() or result.stderr.strip():
        print((result.stdout + result.stderr)[:2000])
        sys.exit(1)
