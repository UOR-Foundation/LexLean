#!/usr/bin/env python3
"""Run one supported host's tests as lanes of one run, and join them (SPEC §8.3/§9.4).

A host whose one `cargo test --workspace` was the longest job of the gate runs
it as lanes instead, each a job of its own clean checkout and build: the
conformance harness's cases, partitioned exactly as the Intel lanes partition
them (`.github/intel-host-lanes.json`, whose completeness honesty checks), and
the `producer` lane, every other test of the workspace and every doctest. Each
lane records what the host listed and what it executed; the join requires a
receipt from every lane of the same run and commit, lanes that are disjoint,
and an execution whose union is the host's whole listing, test for test,
before the host's context passes. Nothing crosses runs. Python 3.9+.
"""

import argparse
from collections import Counter
import json
import os
from pathlib import Path
import re
import subprocess
import sys

sys.path.insert(0, str(Path(__file__).resolve().parent))
import intel_host_tests as factory  # noqa: E402  (the shared partition)

SCHEMA = 1
PRODUCER = "producer"
CARGO = ["cargo", "test", "--locked", "--all-features", "--no-fail-fast"]
HARNESS = [*CARGO, "-p", "repo-conformance", "--test", "conformance"]
RESULT = re.compile(r"test (.+?) \.\.\. (ok|ignored|FAILED)(?:, .*)?")
SUFFIXES = (" - should panic", " - compile fail", " - no_run")

require = factory.require
Invalid = factory.Invalid


def lanes(root):
    """The lanes of a host: the producer, then the conformance lanes."""
    bdd, plan = factory.plan(root)
    return bdd, plan, [PRODUCER, *plan["lanes"]]


def listing(output):
    """Every `name: test` line of a (multi-binary) `--list` transcript."""
    return [line[: -len(": test")] for line in output.splitlines() if line.endswith(": test")]


def executed(output):
    names = []
    for line in output.splitlines():
        match = RESULT.fullmatch(line.strip())
        if match:
            name = match.group(1)
            for suffix in SUFFIXES:
                name = name.removesuffix(suffix)
            names.append(name)
    return names


def run(command, root, capture=True):
    print("+ " + " ".join(command), flush=True)
    process = subprocess.Popen(command, cwd=root, stdout=subprocess.PIPE, text=True,
                               encoding="utf-8", errors="replace")
    lines = []
    for line in process.stdout:
        sys.stdout.write(line)
        lines.append(line)
    sys.stdout.flush()
    return process.wait(), "".join(lines)


def metadata_targets(root):
    """The test-carrying targets of repo-conformance other than the harness."""
    output = subprocess.run(["cargo", "metadata", "--locked", "--format-version", "1", "--no-deps"],
                            cwd=root, check=True, stdout=subprocess.PIPE, text=True).stdout
    package = next(p for p in json.loads(output)["packages"] if p["name"] == "repo-conformance")
    selectors = []
    for target in package["targets"]:
        kinds = set(target["kind"])
        if "test" in kinds and target["name"] != "conformance":
            selectors += ["--test", target["name"]]
        elif kinds & {"lib", "rlib"}:
            selectors.append("--lib")
        elif "bin" in kinds:
            selectors += ["--bin", target["name"]]
    return selectors


def identity(root):
    git = lambda *args: subprocess.run(["git", *args], cwd=root, check=True,  # noqa: E731
                                       stdout=subprocess.PIPE, text=True).stdout.strip()
    return {"commit": git("rev-parse", "HEAD"), "tree": git("rev-parse", "HEAD^{tree}"),
            "run": os.environ.get("GITHUB_RUN_ID", ""), "attempt": os.environ.get("GITHUB_RUN_ATTEMPT", "")}


def execute(root, target, lane, output):
    host = subprocess.run(["rustc", "-vV"], cwd=root, check=True, stdout=subprocess.PIPE,
                          text=True).stdout
    require(f"host: {target}\n" in host, f"this runner is not {target}")
    bdd, plan, names = lanes(root)
    require(lane in names, f"`{lane}` is no lane of {factory.LANES}")
    status, out = run([*HARNESS, "--", "--list"], root)
    require(status == 0, "the conformance harness does not list")
    conformance = factory.parse_list(out)
    receipt = {"schema": SCHEMA, "target": target, "lane": lane, **identity(root),
               "conformance": conformance, "whole": None, "executed": []}
    failed = False
    if lane == PRODUCER:
        status, out = run(["cargo", "test", "--locked", "--workspace", "--all-features", "--", "--list"], root)
        require(status == 0, "the workspace does not list")
        receipt["whole"] = sorted(listing(out))
        commands = [
            [*CARGO, "--workspace", "--exclude", "repo-conformance"],
            [*CARGO, "-p", "repo-conformance", *metadata_targets(root)],
            [*CARGO, "-p", "repo-conformance", "--doc"],
        ]
    else:
        commands = [[*HARNESS, "--", *factory.lane_filters(bdd, plan)[lane]]]
    for command in commands:
        status, out = run(command, root)
        failed |= status != 0
        receipt["executed"] += executed(out)
    receipt["executed"].sort()
    receipt["passed"] = not failed
    output.mkdir(parents=True, exist_ok=True)
    (output / "receipt.json").write_text(json.dumps(receipt, indent=1, sort_keys=True) + "\n")
    require(not failed, f"lane `{lane}` has a failing test")
    print(f"host-lanes: {target} lane {lane}: {len(receipt['executed'])} tests passed")


def join(root, target, artifacts):
    bdd, plan, names = lanes(root)
    receipts = {}
    for path in sorted(Path(artifacts).glob("*/receipt.json")):
        receipt = json.loads(path.read_text())
        require(receipt.get("schema") == SCHEMA and receipt.get("target") == target,
                f"{path}: a receipt of another host or format")
        require(receipt["lane"] not in receipts, f"two receipts of lane `{receipt['lane']}`")
        receipts[receipt["lane"]] = receipt
    return check_join(receipts, names, plan, bdd, identity(root))


def check_join(receipts, names, plan, bdd, here):
    """The lanes are this run's and commit's, complete, disjoint, and passed."""
    require(sorted(receipts) == sorted(names), f"lanes {sorted(names)}, receipts {sorted(receipts)}")
    for lane, receipt in receipts.items():
        for field in ("commit", "tree", "run", "attempt"):
            if field == "attempt":
                require(str(receipt[field]).isdigit() and int(receipt[field]) <= int(here[field] or 0),
                        f"lane `{lane}` ran in another attempt")
            else:
                require(receipt[field] == here[field], f"lane `{lane}` ran on another {field}")
        require(receipt.get("passed") is True, f"lane `{lane}` did not pass")
    conformance = receipts[PRODUCER]["conformance"]
    require(conformance and all(r["conformance"] == conformance for r in receipts.values()),
            "the lanes listed different conformance harnesses")
    named = {factory.test_name(i) for ids in bdd["parts"].values() for i in ids}
    require(named <= set(conformance), f"named tests absent: {sorted(named - set(conformance))}")
    whole = Counter(receipts[PRODUCER]["whole"] or [])
    ran = Counter(name for r in receipts.values() for name in r["executed"])
    require(whole, "the producer listed no workspace")
    missing, extra = whole - ran, ran - whole
    require(not missing and not extra,
            f"executed is not the host's listing: missing {sorted(missing)[:20]}, extra {sorted(extra)[:20]}")
    for lane in names:
        if lane == PRODUCER:
            continue
        require(set(receipts[lane]["executed"]) <= set(conformance),
                f"lane `{lane}` ran a test outside the conformance harness")
    print(f"host-lanes: {len(ran)} tests of {len(names)} lanes equal the host's listing, each once")
    return sum(ran.values())


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)
    one = sub.add_parser("execute")
    one.add_argument("--target", required=True)
    one.add_argument("--lane", required=True)
    one.add_argument("--output", required=True, type=Path)
    both = sub.add_parser("join")
    both.add_argument("--target", required=True)
    both.add_argument("--artifacts", required=True)
    arguments = parser.parse_args()
    root = Path(__file__).resolve().parents[2]
    try:
        if arguments.command == "execute":
            execute(root, arguments.target, arguments.lane, arguments.output)
        else:
            join(root, arguments.target, arguments.artifacts)
    except Invalid as error:
        print(f"::error::{error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
