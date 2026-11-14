#!/usr/bin/env python3
"""Construct, execute and join one run's Intel host tests (SPEC §8.3/§9.4).

Only the conformance harness crosses machines. All other workspace executables
stay beside their CARGO_BIN_EXE companions on the producer. This is execution
scheduling, not a cache or an alternative acceptance gate. Python 3.9+.
"""

import argparse
from collections import Counter
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import platform
import re
import shutil
import stat
import subprocess
import sys
import tarfile
import tempfile
import threading

HOST = "x86_64-apple-darwin"
SCHEMA = 1
BDD = ".github/bdd-parts.json"
LANES = ".github/intel-host-lanes.json"
DATA_ROOTS = ("registry/src", "registry/cache", "registry/index")
BUILD = ["cargo", "test", "--locked", "--workspace", "--all-features", "--no-run", "--no-fail-fast", "--message-format=json"]
DOCS = ["cargo", "test", "--locked", "--workspace", "--all-features", "--doc", "--no-fail-fast"]
NAME = re.compile(r"[a-z0-9]+(?:-[a-z0-9]+)*\Z")


class Invalid(RuntimeError):
    """The run cannot establish complete host execution."""


def require(condition, message):
    if not condition:
        raise Invalid(message)


def json_pairs(pairs):
    result = {}
    for key, value in pairs:
        require(key not in result, f"duplicate JSON key: {key}")
        result[key] = value
    return result


def read_json(path, bdd=False):
    # The historical BDD file repeats identical keys; match its Rust parser.
    path = Path(path)
    require(path.is_file() and not path.is_symlink(), f"not a regular JSON artifact: {path}")
    return json.loads(path.read_text(), object_pairs_hook=None if bdd else json_pairs)


def write_json(path, value):
    Path(path).write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


def digest(path):
    h = hashlib.sha256()
    with Path(path).open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            h.update(block)
    return h.hexdigest()


def safe_relative(name):
    require(isinstance(name, str) and name and "\\" not in name and "\x00" not in name,
            f"unsafe artifact path: {name!r}")
    path = PurePosixPath(name)
    require(not path.is_absolute() and all(p not in ("", ".", "..") for p in name.split("/")),
            f"unsafe artifact path: {name!r}")
    return path


def file_info(path, name):
    path = Path(path)
    require(path.is_file() and not path.is_symlink(), f"not a regular file: {path}")
    return {"path": str(safe_relative(name)), "size": path.stat().st_size,
            "sha256": digest(path), "mode": stat.S_IMODE(path.stat().st_mode)}


def check_file(root, info):
    name = str(safe_relative(info["path"]))
    path = Path(root) / name
    for parent in [path, *path.parents]:
        if parent == Path(root):
            break
        require(not parent.is_symlink(), f"symlink in artifact path: {path}")
    require(file_info(path, name) == info, f"file digest, size or mode differs: {name}")


def exact_files(root, names):
    expected = set(names)
    require(len(expected) == len(names), "duplicate artifact file")
    actual = set()
    for path in Path(root).rglob("*"):
        require(not path.is_symlink(), f"symlink artifact: {path}")
        require(path.is_dir() or path.is_file(), f"special artifact: {path}")
        if path.is_file():
            actual.add(path.relative_to(root).as_posix())
        else:
            prefix = path.relative_to(root).as_posix() + "/"
            require(any(name.startswith(prefix) for name in expected), f"unexpected artifact directory: {path}")
    require(actual == expected, f"artifact files differ: missing={sorted(expected-actual)}, unexpected={sorted(actual-expected)}")


def archive(output, files):
    """Write only explicit regular files; never a home/config tree or symlinks."""
    entries = []
    with tarfile.open(output, "w:gz", format=tarfile.PAX_FORMAT) as tar:
        for name, source in sorted(files.items()):
            info = file_info(source, name)
            require(info["mode"] & 0o7000 == 0, f"special file mode: {name}")
            entries.append(info)
            item = tarfile.TarInfo(name)
            item.size, item.mode, item.mtime = info["size"], info["mode"], 0
            with Path(source).open("rb") as stream:
                tar.addfile(item, stream)
    return entries


def inspect_archive(path, entries, destination=None):
    """Check all members before writing anything; do not call extractall."""
    names = [entry["path"] for entry in entries]
    require(len(names) == len(set(names)), "duplicate archive manifest member")
    expected = {str(safe_relative(entry["path"])): entry for entry in entries}
    require(not any(str(parent) in expected for name in expected for parent in PurePosixPath(name).parents),
            "archive file/directory collision")
    if destination is not None:
        destination = Path(destination)
        require(not destination.exists(), f"archive destination already exists: {destination}")
        require(not any(p.is_symlink() for p in [destination, *destination.parents]), "symlink extraction destination")
    with tarfile.open(path, "r:gz") as tar:
        members = tar.getmembers()
        require(len(members) == len(expected), "missing or extra archive members")
        seen = set()
        for member in members:
            name = str(safe_relative(member.name))
            require(name not in seen and name in expected, f"duplicate or unexpected archive member: {name}")
            seen.add(name)
            info = expected[name]
            require(member.isfile() and not member.issparse() and not member.linkname,
                    f"non-regular archive member: {name}")
            require(member.size == info["size"] and member.mode == info["mode"] and member.mode & 0o7000 == 0,
                    f"archive size or mode differs: {name}")
            stream = tar.extractfile(member)
            h = hashlib.sha256()
            for block in iter(lambda: stream.read(1024 * 1024), b""):
                h.update(block)
            require(h.hexdigest() == info["sha256"], f"archive digest differs: {name}")
        if destination is not None:
            destination.mkdir(parents=True)
            for member in members:
                target = destination / member.name
                target.parent.mkdir(parents=True, exist_ok=True)
                with tar.extractfile(member) as source, target.open("xb") as out:
                    shutil.copyfileobj(source, out)
                target.chmod(member.mode)


def plan(root):
    bdd, lanes = read_json(root / BDD, bdd=True), read_json(root / LANES)
    require(set(bdd) <= {"parts", "targets", "remainder"}, "unknown BDD plan field")
    require(set(lanes) == {"producer", "lanes"} and bool(lanes["lanes"]),
            "Intel plan requires a producer and nonempty execution lanes")
    parts, targets, remainder = bdd["parts"], bdd.get("targets", {}), bdd["remainder"]
    require(isinstance(parts, dict) and isinstance(targets, dict), "invalid BDD parts")
    names = [*parts, *targets, remainder]
    require(all(isinstance(n, str) and NAME.fullmatch(n) for n in names), "invalid BDD part name")
    require(len(names) == len(set(names)), "overlapping BDD parts")
    ids = []
    for part, values in parts.items():
        require(isinstance(values, list) and values, f"empty BDD part: {part}")
        require(all(isinstance(v, str) and re.fullmatch(r"[A-Z]{2}-[0-9]{2}", v) for v in values), "invalid conformance ID")
        ids.extend(values)
    require(len(ids) == len(set(ids)), "duplicate conformance ID")
    target_names = []
    for values in targets.values():
        require(isinstance(values, list) and values and all(NAME.fullmatch(v) for v in values), "invalid BDD target part")
        target_names.extend(values)
    require(len(target_names) == len(set(target_names)) and "conformance" not in target_names,
            "overlapping or wholly assigned conformance target")
    require(set(lanes["producer"]) == set(targets), "producer must own every complete BDD target part")
    mapped = list(lanes["producer"])
    for lane, values in lanes["lanes"].items():
        require(NAME.fullmatch(lane) and isinstance(values, list) and values, "invalid or empty Intel lane")
        require(not set(values) & set(targets), "target parts execute only on producer")
        mapped.extend(values)
    require(Counter(mapped) == Counter(names), "Intel lane plan drift: every BDD part must appear exactly once")
    return bdd, lanes


def test_name(identifier):
    return "conformance_" + identifier.lower().replace("-", "_")


def lane_filters(bdd, lanes):
    all_named = {test_name(i) for ids in bdd["parts"].values() for i in ids}
    result = {}
    for lane, parts in lanes["lanes"].items():
        names = {test_name(i) for part in parts if part in bdd["parts"] for i in bdd["parts"][part]}
        if bdd["remainder"] in parts:
            result[lane] = ["--exact", *[arg for name in sorted(all_named - names) for arg in ("--skip", name)]]
        else:
            result[lane] = ["--exact", *sorted(names)]
    return result


def parse_list(output):
    names = []
    count = None
    for line in output.splitlines():
        if line.endswith(": test"):
            names.append(line[:-6])
        elif line.endswith(": benchmark"):
            raise Invalid("benchmark in default test inventory is unsupported")
        elif re.fullmatch(r"\d+ tests?, \d+ benchmarks?", line):
            require(count is None, "multiple libtest listing summaries")
            count = int(line.split()[0])
            require(line.split(", ")[1].startswith("0 "), "benchmark inventory")
        elif line.strip():
            raise Invalid(f"unrecognized libtest listing: {line}")
    require(count == len(names) and len(names) == len(set(names)), "missing or duplicate libtest inventory")
    return sorted(names)


def partition(whole, listed, bdd):
    require(whole, "empty conformance inventory")
    require(any(name.startswith("conformance_") for name in whole), "no conformance cases in harness")
    require(all(values for values in listed.values()), "empty Intel lane")
    assigned = [name for values in listed.values() for name in values]
    require(Counter(assigned) == Counter(whole), "missing, duplicate or foreign conformance tests")
    named = {test_name(i) for ids in bdd["parts"].values() for i in ids}
    require(named <= set(whole), f"named BDD tests absent from actual harness: {sorted(named-set(whole))}")


def check_execution(stdout, expected, ignored):
    """A --list transcript cannot satisfy this check, including empty targets."""
    results = []
    summaries = []
    for line in stdout.splitlines():
        match = re.fullmatch(r"test (.+) \.\.\. (ok|FAILED|ignored)(?:, .*)?", line)
        if match:
            name, result = match.groups()
            results.append((name.removesuffix(" - should panic"), result))
        summary = re.fullmatch(r"test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out; finished in .+", line)
        if summary:
            summaries.append(summary.groups())
    require(Counter(name for name, _ in results) == Counter(expected), "execution did not report each selected test exactly once")
    require(len(summaries) == 1, "missing or repeated libtest execution summary")
    statuses = dict(results)
    require(set(ignored) <= set(expected), "ignored inventory outside selected tests")
    require(all(statuses[name] == ("ignored" if name in ignored else "ok") for name in expected), "failed test or changed ignored semantics")
    verdict, passed, failed, skips, measured, _ = summaries[0]
    require((verdict, int(passed), int(failed), int(skips), int(measured)) ==
            ("ok", len(expected)-len(ignored), 0, len(ignored), 0), "libtest summary does not match execution")


class Recorder:
    def __init__(self, output):
        self.output = Path(output)
        (self.output / "logs").mkdir(parents=True, exist_ok=True)
        self.records = []

    def run(self, command, cwd, env=None, kind="setup", expected=None, ignored=None):
        number = len(self.records)
        paths = [self.output / "logs" / f"{number:04d}.{channel}.log" for channel in ("stdout", "stderr")]
        record = {"command": [str(a) for a in command], "cwd": str(cwd), "env": env or {}, "kind": kind}
        if expected is not None:
            record.update(expected=expected, ignored=ignored or [])
        with paths[0].open("wb") as out, paths[1].open("wb") as err:
            try:
                process = subprocess.Popen(command, cwd=cwd, env={**os.environ, **(env or {})},
                                           stdin=subprocess.DEVNULL, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
                def drain(source, destination, live):
                    while block := source.read1(8192):
                        destination.write(block)
                        destination.flush()
                        live.buffer.write(block)
                        live.flush()
                threads = [threading.Thread(target=drain, args=(source, dest, live)) for source, dest, live in
                           ((process.stdout, out, sys.stdout), (process.stderr, err, sys.stderr))]
                for thread in threads:
                    thread.start()
                record["exit_code"] = process.wait()
                for thread in threads:
                    thread.join()
                process.stdout.close()
                process.stderr.close()
            except OSError as error:
                err.write(str(error).encode())
                record["exit_code"] = 127
        record["stdout"], record["stderr"] = [file_info(path, path.relative_to(self.output).as_posix()) for path in paths]
        self.records.append(record)
        return record

    def output_text(self, record, channel="stdout"):
        return (self.output / record[channel]["path"]).read_text(errors="replace")

    def checked(self, command, cwd, env=None):
        record = self.run(command, cwd, env)
        require(record["exit_code"] == 0, f"construction command failed: {command}")
        return self.output_text(record)


def git(root, *args):
    value = subprocess.run(["git", *args], cwd=root, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    return value.stdout.decode().strip()


def source_identity(root):
    require(not git(root, "status", "--porcelain", "--untracked-files=all"), "checkout is not clean")
    return {"commit": git(root, "rev-parse", "HEAD"), "tree": git(root, "rev-parse", "HEAD^{tree}"),
            "files": {name: digest(root / name) for name in (BDD, LANES, "Cargo.lock", "rust-toolchain.toml")}}


def run_identity():
    result = {name: os.environ.get(name, "") for name in ("GITHUB_REPOSITORY", "GITHUB_RUN_ID", "GITHUB_RUN_ATTEMPT")}
    require(all(result.values()), "GitHub repository, run ID and attempt are required")
    # GitHub names a repository case-insensitively, and the repository is now
    # spelled `UOR-Foundation/lexlean`; any other repository stays foreign.
    require(result["GITHUB_REPOSITORY"].lower() == "uor-foundation/lexlean", "foreign repository")
    require(all(result[n].isdigit() for n in ("GITHUB_RUN_ID", "GITHUB_RUN_ATTEMPT")), "invalid run ID/attempt")
    return result


def native_identity(root, recorder):
    require(platform.system() == "Darwin" and platform.machine() == "x86_64", "Intel macOS native execution required")
    dangerous = [name for name in os.environ if name.startswith(("RUSTFLAGS", "RUSTDOCFLAGS", "CARGO_ENCODED_", "CARGO_PROFILE_", "CARGO_BUILD_", "CARGO_TARGET_", "RUST_TEST_", "DYLD_"))
                 or name in {"RUSTC", "RUSTDOC", "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER", "CARGO_BUILD_TARGET", "CARGO_TARGET_DIR", "RUST_TEST_THREADS"}]
    require(not dangerous, f"unexpected compiler/loader overrides: {dangerous}")
    channels = re.findall(r'^channel\s*=\s*"([0-9]+\.[0-9]+\.[0-9]+)"\s*$',
                          (root / "rust-toolchain.toml").read_text(), re.M)
    require(len(channels) == 1, "expected one exact Rust version in rust-toolchain.toml")
    pin = channels[0]
    rustc = recorder.checked(["rustc", "-vV"], root)
    cargo = recorder.checked(["cargo", "-V"], root)
    sysroot = recorder.checked(["rustc", "--print", "sysroot"], root).strip()
    require(f"host: {HOST}\n" in rustc and f"release: {pin}\n" in rustc, "Rust version/host does not match pin")
    return {"host": HOST, "pin": pin, "rustc": rustc, "cargo": cargo, "sysroot": sysroot,
            "checkout": str(root), "profile": "test", "features": "--workspace --all-features", "locked": True}


def cargo_context(package):
    version = package["version"].split("-", 1)
    components = version[0].split(".")
    return {"CARGO_MANIFEST_DIR": str(Path(package["manifest_path"]).parent),
            "CARGO_MANIFEST_PATH": package["manifest_path"], "CARGO_PKG_NAME": package["name"],
            "CARGO_PKG_VERSION": package["version"], "CARGO_PKG_VERSION_MAJOR": components[0],
            "CARGO_PKG_VERSION_MINOR": components[1], "CARGO_PKG_VERSION_PATCH": components[2],
            "CARGO_PKG_VERSION_PRE": version[1] if len(version) == 2 else "",
            **{"CARGO_PKG_" + key.upper(): package.get(key) or "" for key in
               ("description", "homepage", "repository", "license", "license_file", "rust_version")},
            "CARGO_PKG_AUTHORS": ":".join(package.get("authors", [])), "CARGO": "cargo"}


def dependency_sources(metadata, root, check_paths=True):
    for package in metadata["packages"]:
        source = package.get("source")
        if source is None:
            manifest = Path(package["manifest_path"])
            if check_paths:
                manifest = manifest.resolve()
            require(manifest.is_relative_to(root) and not manifest.is_relative_to(root / "target"),
                    f"unsupported path dependency outside bound checkout: {package['name']}")
        else:
            require(source == "registry+https://github.com/rust-lang/crates.io-index",
                    f"unsupported dependency source: {package['name']}: {source}")


def built_targets(metadata, output, root, check_paths=True):
    packages = {p["id"]: p for p in metadata["packages"] if p["id"] in metadata["workspace_members"]}
    targets = {}
    for line in output.splitlines():
        value = json.loads(line)
        if value.get("reason") != "compiler-artifact" or not value.get("executable") or not value["profile"]["test"]:
            continue
        require(value["package_id"] in packages, "test executable outside workspace")
        package, target = packages[value["package_id"]], value["target"]
        key = f'{package["name"]}/{",".join(target["kind"])}/{target["name"]}'
        require(key not in targets, f"duplicate test executable: {key}")
        executable = Path(value["executable"])
        require(executable.is_relative_to(root / "target") and (not check_paths or executable.is_file()), "test executable outside target directory")
        targets[key] = {"id": key, "package": package["name"], "name": target["name"],
                        "kind": target["kind"], "executable": str(executable), "env": cargo_context(package),
                        "cwd": str(Path(package["manifest_path"]).parent)}
    # Cargo's default test selection includes libraries, binaries and integration
    # targets. A new target is either discovered and executed or fails construction.
    required = {f'{p["name"]}/{",".join(t["kind"])}/{t["name"]}' for p in packages.values() for t in p["targets"]
                if t.get("test") and set(t["kind"]) & {"lib", "rlib", "proc-macro", "bin", "test"}}
    require(required <= set(targets), f"workspace test targets not constructed: {sorted(required-set(targets))}")
    conformance = [t for t in targets.values() if t["package"] == "repo-conformance" and t["kind"] == ["test"] and t["name"] == "conformance"]
    require(len(conformance) == 1, "expected one conformance integration harness")
    docs = sorted(t["name"].replace("-", "_") for p in packages.values() for t in p["targets"] if t.get("doctest") and set(t["kind"]) & {"lib", "rlib", "proc-macro"})
    return sorted(targets.values(), key=lambda t: t["id"]), conformance[0]["id"], docs


def loader_environment(root, native):
    return {"DYLD_FALLBACK_LIBRARY_PATH": os.pathsep.join([str(root / "target/debug/deps"),
            str(root / "target/debug"), str(Path(native["sysroot"]) / "lib/rustlib" / HOST / "lib"),
            str(Path.home() / "lib"), "/usr/local/lib", "/usr/lib"])}


def dylib_closure(binary, root, native, recorder):
    """Resolve Mach-O load commands now; never guess a dependency is unnecessary."""
    known, todo, libraries = set(), [Path(binary)], {}
    search = [Path(binary).parent, root / "target/debug", Path(native["sysroot"]) / "lib/rustlib" / HOST / "lib"]
    while todo:
        path = todo.pop().resolve()
        if path in known:
            continue
        known.add(path)
        deps = recorder.checked(["otool", "-L", str(path)], root).splitlines()[1:]
        commands = recorder.checked(["otool", "-l", str(path)], root)
        rpaths = re.findall(r"cmd LC_RPATH\n\s+cmdsize \d+\n\s+path (.+) \(offset \d+\)", commands)
        for line in deps:
            dependency = line.strip().split(" (compatibility version", 1)[0]
            if dependency.startswith(("/usr/lib/", "/System/Library/")):
                continue
            if dependency.startswith("@rpath/"):
                candidates = [Path(p.replace("@loader_path", str(path.parent)).replace("@executable_path", str(Path(binary).parent))) / dependency[7:] for p in rpaths]
                candidates += [p / dependency[7:] for p in search]
            else:
                candidates = [Path(dependency.replace("@loader_path", str(path.parent)).replace("@executable_path", str(Path(binary).parent)))]
            found = next((p.resolve() for p in candidates if p.is_file()), None)
            require(found is not None, f"unresolved runtime dylib: {dependency}")
            if found == path:  # LC_ID_DYLIB is listed as well.
                continue
            require(found.is_relative_to(root / "target") or found.is_relative_to(Path(native["sysroot"])),
                    f"runtime dylib outside construction/toolchain: {found}")
            name = PurePosixPath(dependency).name
            require(name not in libraries or libraries[name] == found, "ambiguous dylib basename")
            libraries[name] = found
            todo.append(found)
    return {"dylibs/" + name: value for name, value in libraries.items()}


def dependency_files(cargo_home):
    # This factory currently supports this repository's crates.io lock only.
    # A fresh Cargo home, not the runner's home, is the source of these files.
    # Git/alternate-registry support must define its own credential-free format.
    require(not (cargo_home / "git").exists(), "unsupported Git dependency data")
    files = {}
    for relative in DATA_ROOTS:
        base = cargo_home / relative
        if not base.exists():
            continue
        require(not base.is_symlink(), f"symlink Cargo data root: {base}")
        require(all(p.is_dir() and p.name.startswith("index.crates.io-") for p in base.iterdir()),
                "unsupported dependency registry: only crates.io sparse data is transferable")
        for path in base.rglob("*"):
            require(not path.is_symlink(), f"symlink in dependency data: {path}")
            if path.is_file():
                name = path.relative_to(cargo_home).as_posix()
                if relative == "registry/index":
                    require(path.name not in ("config", "config.toml", "credentials", "credentials.toml") and
                            ".git" not in path.relative_to(base).parts, "configuration in registry index")
                files["cargo/" + name] = path
            else:
                require(path.is_dir(), f"special dependency data: {path}")
    return files


def check_records(directory, records, must_succeed=False):
    names = []
    for number, record in enumerate(records):
        for channel in ("stdout", "stderr"):
            info = record[channel]
            require(info["path"] == f"logs/{number:04d}.{channel}.log", "unexpected invocation log path")
            check_file(directory, info)
            names.append(info["path"])
        if must_succeed:
            require(record["exit_code"] == 0, f"failed construction invocation: {record['command']}")
    return names


def record_ok(directory, record):
    require(record["exit_code"] == 0, f"failed invocation: {record['command']}")
    text = (directory / record["stdout"]["path"]).read_text(errors="replace")
    if record["kind"] == "harness":
        check_execution(text, record["expected"], record["ignored"])
    elif record["kind"] == "doctests":
        stderr = (directory / record["stderr"]["path"]).read_text(errors="replace")
        listed = re.findall(r"^\s*Doc-tests ([A-Za-z0-9_]+)\s*$", stderr, re.M)
        require(Counter(listed) == Counter(record["expected"]), "doctest target coverage differs")
        require(len(re.findall(r"^test result: ok\.", text, re.M)) == len(listed), "missing doctest execution summaries")


def checked_record(directory, checks, command):
    matches = [r for r in checks if r["command"] == command]
    require(len(matches) == 1 and matches[0]["exit_code"] == 0, f"missing, duplicate or failed construction command: {command}")
    record = matches[0]
    return record, (directory / record["stdout"]["path"]).read_text(errors="replace")


def verify_native_checks(directory, checks, binding):
    native = binding["native"]
    for command, expected in ((["rustc", "-vV"], native["rustc"]),
                              (["cargo", "-V"], native["cargo"]),
                              (["rustc", "--print", "sysroot"], native["sysroot"] + "\n")):
        record, output = checked_record(directory, checks, command)
        require(record["cwd"] == native["checkout"] and record["env"] == {} and output == expected,
                "recorded Rust identity differs")


def verify_construction_checks(directory, construction):
    checks, binding = construction["checks"], construction["binding"]
    check_records(directory, checks, must_succeed=True)
    verify_native_checks(directory, checks, binding)
    root = Path(binding["native"]["checkout"])
    cargo_env = binding["cargo_environment"]
    require(set(cargo_env) == {"CARGO_HOME", "CARGO_REGISTRIES_CRATES_IO_PROTOCOL", "CARGO_TERM_COLOR", "CARGO_INCREMENTAL"} and
            cargo_env["CARGO_REGISTRIES_CRATES_IO_PROTOCOL"] == "sparse" and cargo_env["CARGO_TERM_COLOR"] == "never" and cargo_env["CARGO_INCREMENTAL"] == "0" and
            Path(cargo_env["CARGO_HOME"]).is_absolute(), "unexpected Cargo construction environment")
    metadata_command = ["cargo", "metadata", "--locked", "--all-features", "--format-version", "1"]
    metadata_record, metadata_output = checked_record(directory, checks, metadata_command)
    build_record, build_output = checked_record(directory, checks, BUILD)
    require(all(r["env"] == cargo_env and r["cwd"] == str(root) for r in (metadata_record, build_record)),
            "Cargo construction context differs")
    metadata = json.loads(metadata_output)
    dependency_sources(metadata, root, check_paths=False)
    targets, conformance, docs = built_targets(metadata, build_output, root, check_paths=False)
    require(len(targets) == len(construction["targets"]) and conformance == construction["conformance"] and docs == construction["doc_targets"],
            "actual Cargo construction target inventory differs")
    for actual, target in zip(targets, construction["targets"]):
        require(all(target[k] == v for k, v in actual.items()), "target context differs from Cargo metadata")
        require(target["file"]["path"] == Path(target["executable"]).relative_to(root).as_posix(), "target file path differs")
        for args, expected in ((["--list"], target["inventory"]), (["--list", "--ignored"], target["ignored"])):
            record, output = checked_record(directory, checks, [target["executable"], *args])
            require(record["env"] == {**target["env"], **binding["loader"], **cargo_env} and record["cwd"] == target["cwd"] and
                    parse_list(output) == expected, "actual binary inventory differs")
        require(set(target["ignored"]) <= set(target["inventory"]), "ignored tests outside inventory")
    target = next(t for t in construction["targets"] if t["id"] == conformance)
    for lane, filters in construction["filters"].items():
        record, output = checked_record(directory, checks, [target["executable"], "--list", *filters])
        require(record["env"] == {**target["env"], **binding["loader"], **cargo_env} and record["cwd"] == target["cwd"] and
                parse_list(output) == construction["lanes"][lane], "actual filtered binary inventory differs")
    expected_checks = 5 + 2 * len(targets) + len(construction["lanes"])
    otool = [r for r in checks if r["command"][0] == "otool"]
    require(otool and len(checks) == expected_checks + len(otool) and all(
        len(r["command"]) == 3 and r["command"][1] in ("-L", "-l") and r["cwd"] == str(root) and r["env"] == {} for r in otool),
        "unexpected construction invocation or missing loader inspection")
    inspected = {}
    for record in otool:
        path = record["command"][2]
        require(Path(path).is_relative_to(root / "target") or Path(path).is_relative_to(Path(binding["native"]["sysroot"])),
                "loader inspection outside construction/toolchain")
        inspected.setdefault(path, []).append(record["command"][1])
    require(target["executable"] in inspected and all(Counter(flags) == Counter(["-L", "-l"]) for flags in inspected.values()),
            "missing or duplicate Mach-O loader inspection")


def new_output(path):
    path = Path(path).resolve()
    require(not path.exists(), f"output already exists: {path}")
    path.mkdir(parents=True)
    return path


def build(root, output):
    bdd, lanes = plan(root)
    require(os.environ.get("GITHUB_JOB") == "intel-construction", "foreign construction job")
    sources = [line for line in (root / "Cargo.lock").read_text().splitlines() if re.match(r"^\s*source\s*=", line)]
    require(all(re.fullmatch(r'\s*source\s*=\s*"registry\+https://github\.com/rust-lang/crates\.io-index"\s*', line)
                for line in sources),
            "unsupported dependency source: Intel transfer currently supports crates.io and workspace paths only")
    binding = {"run": run_identity(), "source": source_identity(root), "producer_job": "intel-construction"}
    output = new_output(output)
    recorder = Recorder(output)
    native = native_identity(root, recorder)
    binding["native"] = native
    loader = loader_environment(root, native)
    binding["loader"] = loader
    cargo_home = Path(tempfile.mkdtemp(prefix="lexlean-intel-cargo-")).resolve()
    cargo_env = {"CARGO_HOME": str(cargo_home), "CARGO_REGISTRIES_CRATES_IO_PROTOCOL": "sparse", "CARGO_TERM_COLOR": "never", "CARGO_INCREMENTAL": "0"}
    binding["cargo_environment"] = cargo_env
    metadata = json.loads(recorder.checked(["cargo", "metadata", "--locked", "--all-features", "--format-version", "1"], root, cargo_env))
    dependency_sources(metadata, root)
    result = recorder.checked(BUILD, root, cargo_env)
    targets, conformance_id, docs = built_targets(metadata, result, root)
    conformance = next(t for t in targets if t["id"] == conformance_id)
    for target in targets:
        target["file"] = file_info(target["executable"], Path(target["executable"]).relative_to(root).as_posix())
        env = {**target["env"], **loader, **cargo_env}
        target["inventory"] = parse_list(recorder.checked([target["executable"], "--list"], Path(target["cwd"]), env))
        target["ignored"] = parse_list(recorder.checked([target["executable"], "--list", "--ignored"], Path(target["cwd"]), env))
        require(set(target["ignored"]) <= set(target["inventory"]), "ignored tests not in full inventory")
    complete_targets = {t["name"] for t in targets if t["package"] == "repo-conformance" and t["kind"] == ["test"]}
    require(all(name in complete_targets for values in bdd.get("targets", {}).values() for name in values), "BDD target absent from workspace")
    filters = lane_filters(bdd, lanes)
    listed = {lane: parse_list(recorder.checked([conformance["executable"], "--list", *args], Path(conformance["cwd"]), {**conformance["env"], **loader, **cargo_env})) for lane, args in filters.items()}
    partition(conformance["inventory"], listed, bdd)
    # Include only the actual harness and the load commands' non-system closure.
    payload = {"harness/" + Path(conformance["executable"]).name: Path(conformance["executable"])}
    payload.update(dylib_closure(conformance["executable"], root, native, recorder))
    transfers = {}
    for name, files in (("conformance.tar.gz", payload),):
        members = archive(output / name, files)
        transfers[name] = {"file": file_info(output / name, name), "members": members}
    construction = {"schema": SCHEMA, "binding": binding, "targets": targets, "conformance": conformance_id,
                    "doc_targets": docs, "filters": filters, "lanes": listed, "archives": transfers,
                    "checks": list(recorder.records)}
    receipt = {"schema": SCHEMA, "part": "producer", "job": "intel-construction", "binding": binding,
               "invocations": []}
    # Preserve setup logs in the same numbered stream. A failed test does not
    # stop later targets/doctests or prevent consumers from receiving the build.
    for target in targets:
        if target["id"] != conformance_id:
            record = recorder.run([target["executable"], "--format=pretty"], Path(target["cwd"]),
                                  {**target["env"], **loader, **cargo_env}, "harness", target["inventory"], target["ignored"])
            receipt["invocations"].append(record)
    receipt["invocations"].append(recorder.run(DOCS, root, cargo_env, kind="doctests", expected=docs))
    name = "dependencies.tar.gz"
    members = archive(output / name, dependency_files(cargo_home))
    transfers[name] = {"file": file_info(output / name, name), "members": members}
    write_json(output / "construction.json", construction)
    receipt["construction_sha256"] = digest(output / "construction.json")
    write_json(output / "producer.json", receipt)
    if os.environ.get("GITHUB_OUTPUT"):
        with open(os.environ["GITHUB_OUTPUT"], "a") as stream:
            stream.write("lanes=" + json.dumps(list(filters), separators=(",", ":")) + "\n")
    print("Construction complete. Producer test statuses will be judged by join.")
    return 0


def load_construction(root, directory):
    value = read_json(directory / "construction.json")
    require(value["schema"] == SCHEMA, "unknown construction schema")
    require(value["binding"]["run"] == run_identity() and value["binding"]["producer_job"] == "intel-construction",
            "foreign run, repository, attempt or construction job")
    require(value["binding"]["source"] == source_identity(root), "source commit, tree, lock or plan differs")
    bdd, lanes = plan(root)
    require(value["filters"] == lane_filters(bdd, lanes), "lane filters differ from committed plan")
    require(set(value["lanes"]) == set(lanes["lanes"]), "missing or foreign lane")
    native = value["binding"]["native"]
    require(native["host"] == HOST and native["profile"] == "test" and native["features"] == "--workspace --all-features" and native["locked"] is True,
            "foreign construction target/profile/features")
    conformance = [t for t in value["targets"] if t["id"] == value["conformance"]]
    require(len(conformance) == 1, "missing or duplicate constructed conformance target")
    partition(conformance[0]["inventory"], value["lanes"], bdd)
    require(set(value["archives"]) == {"conformance.tar.gz", "dependencies.tar.gz"}, "unexpected archive")
    for name, transfer in value["archives"].items():
        require(transfer["file"]["path"] == name, "archive path mismatch")
        check_file(directory, transfer["file"])
        inspect_archive(directory / name, transfer["members"])
        for member in transfer["members"]:
            path = member["path"]
            if name == "dependencies.tar.gz":
                require(any(path.startswith("cargo/" + prefix + "/") for prefix in DATA_ROOTS), "non-dependency Cargo home file")
                components = PurePosixPath(path).parts
                require(len(components) >= 5 and components[3].startswith("index.crates.io-"), "foreign dependency registry")
                if components[2] == "index":
                    require(components[-1] not in ("config", "config.toml", "credentials", "credentials.toml") and
                            ".git" not in components, "configuration in registry index")
            else:
                require(path.startswith("harness/") or path.startswith("dylibs/"), "unexpected executable archive file")
                require(len(PurePosixPath(path).parts) == 2, "nested executable archive member")
    harnesses = [e for e in value["archives"]["conformance.tar.gz"]["members"] if e["path"].startswith("harness/")]
    require(len(harnesses) == 1 and {k: v for k, v in harnesses[0].items() if k != "path"} ==
            {k: v for k, v in conformance[0]["file"].items() if k != "path"}, "harness digest/mode mismatch")
    verify_construction_checks(directory, value)
    return value, conformance[0], harnesses[0]["path"]


def execute(root, construction_dir, lane, output):
    require(os.environ.get("GITHUB_JOB") == "intel-part", "foreign consumer job")
    construction, target, harness = load_construction(root, construction_dir)
    require(lane in construction["lanes"], "unknown lane")
    output = new_output(output)
    recorder = Recorder(output)
    require(native_identity(root, recorder) == construction["binding"]["native"], "source path, pin or toolchain differs")
    require(loader_environment(root, construction["binding"]["native"]) == construction["binding"]["loader"], "loader environment differs")
    with tempfile.TemporaryDirectory(prefix="lexlean-intel-") as temporary:
        staging = Path(temporary).resolve()
        for name, destination in (("conformance.tar.gz", "runtime"), ("dependencies.tar.gz", "dependencies")):
            inspect_archive(construction_dir / name, construction["archives"][name]["members"], staging / destination)
        cargo_home = staging / "dependencies/cargo"
        cargo_home.mkdir(parents=True, exist_ok=True)
        binary = str(staging / "runtime" / harness)
        env = {**target["env"], **construction["binding"]["loader"], "CARGO_HOME": str(cargo_home),
               "CARGO_NET_OFFLINE": "true", "CARGO_REGISTRIES_CRATES_IO_PROTOCOL": "sparse", "CARGO_TERM_COLOR": "never", "CARGO_INCREMENTAL": "0",
               "DYLD_LIBRARY_PATH": str(staging / "runtime/dylibs")}
        filters = construction["filters"][lane]
        actual = parse_list(recorder.checked([binary, "--list", *filters], Path(target["cwd"]), env))
        ignored = parse_list(recorder.checked([binary, "--list", "--ignored", *filters], Path(target["cwd"]), env))
        require(actual == construction["lanes"][lane], "consumer inventory differs")
        require(ignored == sorted(set(actual) & set(target["ignored"])), "consumer ignored semantics differ")
        record = recorder.run([binary, "--format=pretty", *filters], Path(target["cwd"]), env, "harness", actual, ignored)
        receipt = {"schema": SCHEMA, "part": lane, "job": "intel-part", "binding": construction["binding"],
                   "construction_sha256": digest(construction_dir / "construction.json"),
                   "checks": recorder.records[:-1], "invocations": [record]}
        write_json(output / "receipt.json", receipt)
        try:
            record_ok(output, record)
        except Invalid as error:
            print(error, file=sys.stderr)
            return 1
    return 0


def join(root, artifacts):
    bdd, lanes = plan(root)
    expected = {"intel-construction", *("intel-" + lane for lane in lanes["lanes"])}
    require({p.name for p in artifacts.iterdir()} == expected and all(p.is_dir() and not p.is_symlink() for p in artifacts.iterdir()),
            "missing, duplicate or unexpected Intel artifact directories")
    directory = artifacts / "intel-construction"
    construction, conformance, harness = load_construction(root, directory)
    fingerprint = digest(directory / "construction.json")
    producer = read_json(directory / "producer.json")
    receipts = [("producer", directory, producer)] + [(lane, artifacts / ("intel-" + lane), read_json(artifacts / ("intel-" + lane) / "receipt.json")) for lane in lanes["lanes"]]
    failures = []
    for part, location, receipt in receipts:
        require(receipt["schema"] == SCHEMA and receipt["part"] == part and receipt["job"] == ("intel-construction" if part == "producer" else "intel-part") and receipt["binding"] == construction["binding"] and receipt["construction_sha256"] == fingerprint,
                f"foreign or mismatched receipt: {part}")
        checks = construction["checks"] if part == "producer" else receipt["checks"]
        records = checks + receipt["invocations"]
        files = check_records(location, records)
        require(all(r["exit_code"] == 0 for r in checks), f"failed construction/listing: {part}")
        expected_invocations = [t for t in construction["targets"] if t["id"] != construction["conformance"]] if part == "producer" else [conformance]
        require(len(receipt["invocations"]) == len(expected_invocations) + (1 if part == "producer" else 0), f"missing or duplicate invocations: {part}")
        for target, record in zip(expected_invocations, receipt["invocations"]):
            inventory = target["inventory"] if part == "producer" else construction["lanes"][part]
            ignored = sorted(set(inventory) & set(target["ignored"]))
            require(record["kind"] == "harness" and record["expected"] == inventory and record["ignored"] == ignored and record["cwd"] == target["cwd"], f"invocation inventory/context differs: {part}")
            if part == "producer":
                require(record["command"] == [target["executable"], "--format=pretty"] and record["env"] == {**target["env"], **construction["binding"]["loader"], **construction["binding"]["cargo_environment"]}, "producer command differs")
            else:
                require(record["command"][1:] == ["--format=pretty", *construction["filters"][part]] and record["command"][0].endswith("/runtime/" + harness), "consumer command differs")
                runtime = Path(record["command"][0]).parents[1]
                require(runtime.is_absolute() and runtime.name == "runtime", "consumer runtime path differs")
                environment = {**target["env"], **construction["binding"]["loader"],
                               "CARGO_NET_OFFLINE": "true", "CARGO_HOME": str(runtime.parent / "dependencies/cargo"),
                               "CARGO_REGISTRIES_CRATES_IO_PROTOCOL": "sparse", "CARGO_TERM_COLOR": "never", "CARGO_INCREMENTAL": "0",
                               "DYLD_LIBRARY_PATH": str(runtime / "dylibs")}
                require(record["env"] == environment, "consumer environment differs")
                require(len(checks) == 5, "missing or unexpected consumer checks")
                verify_native_checks(location, checks, construction["binding"])
                for arguments, expected in ((["--list"], inventory), (["--list", "--ignored"], ignored)):
                    listing, output = checked_record(location, checks, [record["command"][0], *arguments, *construction["filters"][part]])
                    require(listing["env"] == environment and listing["cwd"] == record["cwd"] and parse_list(output) == expected,
                            "consumer listing evidence differs")
            try:
                record_ok(location, record)
            except Invalid as error:
                failures.append(f"{part}: {error}")
        if part == "producer":
            docs = receipt["invocations"][-1]
            require(docs["kind"] == "doctests" and docs["command"] == DOCS and docs["cwd"] == construction["binding"]["native"]["checkout"] and docs["expected"] == construction["doc_targets"] and docs["env"] == construction["binding"]["cargo_environment"], "doctest invocation differs")
            try:
                record_ok(location, docs)
            except Invalid as error:
                failures.append(f"producer: {error}")
            files += ["construction.json", "producer.json", *construction["archives"]]
        else:
            files += ["receipt.json"]
        exact_files(location, files)
    require(not failures, "\n".join(failures))
    print(f"Intel host complete: producer and {len(lanes['lanes'])} lanes; {len(conformance['inventory'])} conformance cases inventoried, ignored semantics preserved.")
    return 0


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path.cwd())
    commands = parser.add_subparsers(dest="operation", required=True)
    build_parser = commands.add_parser("build")
    build_parser.add_argument("--output", type=Path, required=True)
    execute_parser = commands.add_parser("execute")
    execute_parser.add_argument("--construction", type=Path, required=True)
    execute_parser.add_argument("--lane", required=True)
    execute_parser.add_argument("--output", type=Path, required=True)
    join_parser = commands.add_parser("join")
    join_parser.add_argument("--artifacts", type=Path, required=True)
    args = parser.parse_args()
    try:
        root = args.root.resolve()
        if args.operation == "build":
            return build(root, args.output)
        if args.operation == "execute":
            return execute(root, args.construction.resolve(), args.lane, args.output)
        return join(root, args.artifacts.resolve())
    except (Invalid, OSError, ValueError, KeyError, TypeError, tarfile.TarError, subprocess.CalledProcessError) as error:
        print(f"Intel host tests: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
