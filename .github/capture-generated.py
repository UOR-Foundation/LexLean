#!/usr/bin/env python3
"""Capture normal writer results, including deletions already in the index.

The artifact is not qualification by itself. Two private indexes establish
that its patch reconstructs the actual result tree from the recorded HEAD;
the repository's real index is never rewritten by this capture.
"""

import argparse
import hashlib
import json
import os
from pathlib import Path
import resource
import stat
import subprocess
import tempfile


SCOPES = ("examples", "compiler", "language/preservation-1.2/modules",
          "tests/fixtures", "tests/negative")
CAP = 64 * 1024 * 1024
MAX_ENTRIES = 30000
MAX_TOTAL = 512 * 1024 * 1024


def require(condition, message):
    if not condition:
        raise ValueError(message)


def native(value):
    return (value.st_dev, value.st_ino, value.st_mode, value.st_nlink,
            value.st_uid, value.st_gid, value.st_size, value.st_mtime_ns,
            value.st_ctime_ns)


def fingerprint(path):
    before = path.lstat()
    require(stat.S_ISREG(before.st_mode) and before.st_nlink == 1,
            f"Generated entry is not a single-link regular file: {path}")
    require(before.st_size <= CAP, f"Generated entry exceeds capture bound: {path}")
    fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    try:
        require(native(os.fstat(fd)) == native(before), f"Open identity changed: {path}")
        digest = hashlib.sha256()
        blob = hashlib.sha1(b"blob " + str(before.st_size).encode() + b"\0")
        remaining = before.st_size
        while remaining:
            chunk = os.read(fd, min(65536, remaining))
            require(chunk, f"Generated entry truncated: {path}")
            remaining -= len(chunk)
            digest.update(chunk)
            blob.update(chunk)
        require(not os.read(fd, 1), f"Generated entry grew: {path}")
        require(native(os.fstat(fd)) == native(before) == native(path.lstat()),
                f"Read identity changed: {path}")
        return {"mode": "100755" if before.st_mode & 0o111 else "100644",
                "bytes": before.st_size, "sha256": digest.hexdigest(),
                "git_blob": blob.hexdigest()}, native(before)
    finally:
        os.close(fd)


def git(root, *args, index=None, objects=None):
    env = {**os.environ, "GIT_OPTIONAL_LOCKS": "0"}
    if index is not None:
        env["GIT_INDEX_FILE"] = str(index)
    if objects is not None:
        env["GIT_OBJECT_DIRECTORY"] = str(objects)
        env["GIT_ALTERNATE_OBJECT_DIRECTORIES"] = str(root / ".git/objects")

    def limits():
        resource.setrlimit(resource.RLIMIT_FSIZE, (CAP, CAP))

    with tempfile.TemporaryFile() as output, tempfile.TemporaryFile() as errors:
        result = subprocess.run(
            ["git", "-c", f"safe.directory={root}", *args], cwd=root,
            env=env, stdout=output, stderr=errors, timeout=120, preexec_fn=limits)
        require(output.tell() <= CAP and errors.tell() <= CAP, "Git capture exceeds bound")
        output.seek(0)
        errors.seek(0)
        raw, error = output.read(CAP + 1), errors.read(CAP + 1)
        require(result.returncode == 0,
                f"Git capture failed ({args[0]}): {result.returncode}: {error[:4096]!r}")
        return raw


def tree(root, revision, index=None, objects=None):
    rows = {}
    for line in git(root, "ls-tree", "-rz", revision, index=index, objects=objects).split(b"\0"):
        if not line:
            continue
        require(len(rows) < MAX_ENTRIES, "Git tree exceeds entry bound")
        meta, name = line.split(b"\t", 1)
        mode, kind, blob = meta.decode("ascii").split()
        path = name.decode("utf-8")
        require(path not in rows and all(p not in ("", ".", "..") for p in path.split("/")),
                "Invalid or duplicate Git path")
        rows[path] = {"mode": mode, "kind": kind, "git_blob": blob}
    return rows


def generated_roots(baseline):
    directories = {"compiler/expected", "compiler/rust", "language/preservation-1.2/modules"}
    files = {"compiler/lexlean.lock"}
    for path in baseline:
        parts = path.split("/")
        if len(parts) == 3 and parts[0] == "examples" and parts[-1] == "lexlean.toml":
            directories.add("/".join(parts[:2]) + "/expected")
            files.add("/".join(parts[:2]) + "/lexlean.lock")
        if path.startswith(("tests/fixtures/", "tests/negative/")) and parts[-1] == "case.toml":
            directories.add(path[:-len("case.toml")] + "expected")
        if path.startswith("tests/negative/") and path.endswith("/project/lexlean.toml"):
            files.add(path[:-len("lexlean.toml")] + "lexlean.lock")
    return sorted(directories), sorted(files)


def is_generated(path, directories, files):
    return path in files or any(path.startswith(d + "/") for d in directories)


def snapshot(root, directories, files):
    rows, identities = {}, {}
    total = 0

    def visit(relative):
        nonlocal total
        require(len(identities) < MAX_ENTRIES, "Generated inventory exceeds entry bound")
        require(len(relative.encode()) <= 4096 and len(Path(relative).parts) <= 64,
                "Generated path exceeds capture bound")
        path = root / relative
        info = path.lstat()
        identities[relative] = native(info)
        if stat.S_ISDIR(info.st_mode):
            with os.scandir(path) as entries:
                for entry in entries:
                    visit(relative + "/" + entry.name)
        else:
            require(total + info.st_size <= MAX_TOTAL, "Generated inventory exceeds byte bound")
            total += info.st_size
            row, identity = fingerprint(path)
            require(identity == identities[relative], f"Snapshot identity changed: {relative}")
            rows[relative] = row

    for relative in directories + files:
        # Parents are part of custody too; following a directory alias would
        # otherwise make a regular leaf appear to belong to the result tree.
        parent = root
        identities[""] = native(root.lstat())
        for part in Path(relative).parts[:-1]:
            parent /= part
            info = parent.lstat()
            require(stat.S_ISDIR(info.st_mode), f"Generated parent is not a directory: {parent}")
            identities[str(parent.relative_to(root))] = native(info)
        path = root / relative
        if path.exists() or path.is_symlink():
            visit(relative)
    return rows, identities


def capture(root, evidence, expected_head):
    root = root.resolve(strict=True)
    require(git(root, "rev-parse", "HEAD").decode().strip() == expected_head,
            "Source HEAD changed")
    require(git(root, "rev-parse", "--show-object-format").strip() == b"sha1", "Unsupported Git object format")
    require(not git(root, "ls-files", "--others", "--exclude-standard", "-z"),
            "Uncaptured untracked source remains")
    baseline = tree(root, "HEAD")
    directories, files = generated_roots(baseline)
    before, identities = snapshot(root, directories, files)
    patch_path = evidence / "generated.patch"
    with patch_path.open("xb") as output:
        output.write(git(root, "diff", "--binary", "--full-index", "--no-renames",
                         "--no-ext-diff", "--no-textconv", "HEAD", "--"))
    patch_row, patch_identity = fingerprint(patch_path)
    with (evidence / "generated-stat.txt").open("xb") as output:
        output.write(git(root, "diff", "--stat", "--no-renames", "HEAD", "--"))
    with tempfile.TemporaryDirectory(prefix="lexlean-capture-index-") as temporary:
        # The devcontainer owns its clone. Host capture must neither need
        # write access to that Git database nor alter its objects or index.
        objects = Path(temporary) / "objects"
        objects.mkdir()
        actual_index = Path(temporary) / "actual"
        patch_index = Path(temporary) / "patch"
        git(root, "read-tree", "HEAD", index=actual_index, objects=objects)
        git(root, "add", "--all", "--", *SCOPES, index=actual_index, objects=objects)
        actual_tree = git(root, "write-tree", index=actual_index, objects=objects).decode().strip()
        actual = tree(root, actual_tree, objects=objects)
        for path in baseline.keys() | actual.keys():
            if baseline.get(path) != actual.get(path):
                require(is_generated(path, directories, files), f"Non-generated source changed: {path}")
        expected = {p: r for p, r in actual.items() if is_generated(p, directories, files)}
        require(before.keys() == expected.keys(), "Physical generated inventory differs from captured Git inventory")
        for path, row in before.items():
            require(expected[path] == {"mode": row["mode"], "kind": "blob", "git_blob": row["git_blob"]},
                    f"Generated bytes or mode differ from captured Git tree: {path}")
        git(root, "read-tree", "HEAD", index=patch_index, objects=objects)
        if patch_row["bytes"]:
            git(root, "apply", "--cached", "--binary", "--whitespace=nowarn", str(patch_path), index=patch_index, objects=objects)
        reconstructed_tree = git(root, "write-tree", index=patch_index, objects=objects).decode().strip()
        require(reconstructed_tree == actual_tree, "Captured patch does not reconstruct the complete result tree")
    after, final_identities = snapshot(root, directories, files)
    require(after == before and final_identities == identities, "Generated inventory changed during capture")
    require(git(root, "rev-parse", "HEAD").decode().strip() == expected_head, "Source HEAD changed during capture")
    final_patch_row, final_patch_identity = fingerprint(patch_path)
    require((final_patch_row, final_patch_identity) == (patch_row, patch_identity),
            "Captured patch changed during reconstruction")
    patch_sha = patch_row["sha256"]
    final_diff = git(root, "diff", "--binary", "--full-index", "--no-renames",
                     "--no-ext-diff", "--no-textconv", "HEAD", "--")
    require(len(final_diff) == patch_row["bytes"] and hashlib.sha256(final_diff).hexdigest() == patch_sha,
            "Whole source diff changed during capture")
    require(not git(root, "ls-files", "--others", "--exclude-standard", "-z"),
            "Uncaptured untracked source appeared during capture")
    require(git(root, "rev-parse", "HEAD").decode().strip() == expected_head, "Source HEAD changed at completion")
    inventory = {"schema": 1, "complete": True, "source_commit": expected_head,
                 "source_tree": git(root, "rev-parse", "HEAD^{tree}").decode().strip(),
                 "result_tree": actual_tree, "reconstructed_tree": reconstructed_tree,
                 "patch_sha256": patch_sha, "generated_files": before,
                 "deleted_generated_paths": sorted(p for p in baseline if is_generated(p, directories, files) and p not in actual),
                 "scope": "Generated tracked products, not ignored build caches or whole qualification"}
    encoded = (json.dumps(inventory, sort_keys=True) + "\n").encode()
    require(len(encoded) <= CAP, "Generated inventory document exceeds bound")
    with (evidence / "generated-tree.json").open("xb") as output:
        output.write(encoded)
    with (evidence / "generated-patch.sha256").open("x") as output:
        output.write(f"{patch_sha}  generated.patch\n")
    print(f"Complete generated capture: {len(before)} files, {len(inventory['deleted_generated_paths'])} deletions, tree {actual_tree}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("source", type=Path)
    parser.add_argument("evidence", type=Path)
    parser.add_argument("expected_head")
    arguments = parser.parse_args()
    capture(arguments.source, arguments.evidence.resolve(strict=True), arguments.expected_head)
