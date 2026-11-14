"""Behavioral tests for one-run Intel construction, inventory and execution."""

import contextlib
import copy
import importlib.util
import io
import json
import os
from pathlib import Path
import shutil
import stat
import sys
import tarfile
import tempfile
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location("intel_host_tests", Path(__file__).with_name("intel_host_tests.py"))
factory = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(factory)


def listing(names):
    return "".join(name + ": test\n" for name in sorted(names)) + f"\n{len(names)} tests, 0 benchmarks\n"


def execution(names, ignored=(), failed=()):
    lines = [f"test {name} ... " + ("FAILED" if name in failed else "ignored" if name in ignored else "ok") for name in names]
    lines.append(f"test result: {'FAILED' if failed else 'ok'}. {len(names)-len(ignored)-len(failed)} passed; {len(failed)} failed; {len(ignored)} ignored; 0 measured; 0 filtered out; finished in 0.01s")
    return "\n".join(lines) + "\n"


class Fixture:
    """Small, fully logged Cargo construction and execution, with no toolchain."""
    def __init__(self, directory):
        self.root = directory / "repo"
        self.root.mkdir()
        (self.root / ".github").mkdir()
        self.bdd = {"parts": {"a": ["BS-07"], "b": ["BS-06"], "c": ["BS-05"]}, "targets": {"honesty": ["bdd"]}, "remainder": "rest"}
        self.lanes = {"producer": ["honesty"], "lanes": {"lower": ["a"], "print": ["b"], "validate": ["c"], "moves": ["rest"]}}
        factory.write_json(self.root / factory.BDD, self.bdd)
        factory.write_json(self.root / factory.LANES, self.lanes)
        (self.root / "Cargo.lock").write_text('version = 4\n')
        (self.root / "rust-toolchain.toml").write_text('[toolchain]\nchannel = "1.97.1"\n')
        self.artifacts = directory / "artifacts"
        self.artifacts.mkdir()
        self.producer = self.artifacts / "intel-construction"
        self.producer.mkdir()
        self.native = {"checkout": str(self.root), "host": factory.HOST, "pin": "1.97.1", "rustc": f"rustc 1.97.1\nhost: {factory.HOST}\nrelease: 1.97.1\n", "cargo": "cargo 1.97.1\n", "sysroot": "/toolchain", "profile": "test", "features": "--workspace --all-features", "locked": True}
        self.binding = {"run": {"GITHUB_REPOSITORY": "UOR-Foundation/LexLean", "GITHUB_RUN_ID": "100", "GITHUB_RUN_ATTEMPT": "1"}, "source": {"commit": "a"*40, "tree": "b"*40, "files": {}}, "producer_job": "intel-construction", "native": self.native, "loader": {"DYLD_FALLBACK_LIBRARY_PATH": "/toolchain/lib"}, "cargo_environment": {"CARGO_HOME": "/temp/producer-cargo", "CARGO_REGISTRIES_CRATES_IO_PROTOCOL": "sparse", "CARGO_TERM_COLOR": "never", "CARGO_INCREMENTAL": "0"}}
        self.metadata = {"workspace_members": ["repo-conformance", "ordinary"], "packages": []}
        self.compiler = []
        self.inventories = {}
        for package_name, target_data in (("repo-conformance", [("lib", "repo_conformance", ["library_test"]), ("test", "conformance", ["conformance_bs_05", "conformance_bs_06", "conformance_bs_07", "future_case", "ignored_future"]), ("test", "bdd", ["honesty_test"])]), ("ordinary", [("lib", "ordinary", ["ordinary_test"]), ("test", "future_integration", [])])):
            package_root = self.root / package_name
            package_root.mkdir()
            package = {"id": package_name, "name": package_name, "version": "0.4.0", "manifest_path": str(package_root / "Cargo.toml"), "targets": [], "authors": ["Someone"]}
            for kind, name, names in target_data:
                target = {"name": name, "kind": [kind], "test": True, "doctest": kind == "lib"}
                package["targets"].append(target)
                binary = self.root / "target/debug/deps" / (name + "-abc123")
                binary.parent.mkdir(parents=True, exist_ok=True)
                binary.write_text("binary " + name)
                binary.chmod(0o755)
                self.compiler.append({"reason": "compiler-artifact", "package_id": package_name, "profile": {"test": True}, "target": target, "executable": str(binary)})
                self.inventories[str(binary)] = names
            self.metadata["packages"].append(package)
        targets, conformance, docs = factory.built_targets(self.metadata, self.build_output(), self.root)
        self.target = next(t for t in targets if t["id"] == conformance)
        self.records = []
        self.identity_checks(self.producer, self.records)
        self.add(self.producer, self.records, ["cargo", "metadata", "--locked", "--all-features", "--format-version", "1"], json.dumps(self.metadata), env=self.binding["cargo_environment"])
        self.add(self.producer, self.records, factory.BUILD, self.build_output(), env=self.binding["cargo_environment"])
        for target in targets:
            binary = target["executable"]
            target["file"] = factory.file_info(binary, Path(binary).relative_to(self.root).as_posix())
            target["inventory"] = sorted(self.inventories[binary])
            target["ignored"] = ["ignored_future"] if target["name"] == "conformance" else []
            env = {**target["env"], **self.binding["loader"], **self.binding["cargo_environment"]}
            self.add(self.producer, self.records, [binary, "--list"], listing(target["inventory"]), env=env, cwd=target["cwd"])
            self.add(self.producer, self.records, [binary, "--list", "--ignored"], listing(target["ignored"]), env=env, cwd=target["cwd"])
        filters = factory.lane_filters(self.bdd, self.lanes)
        inventories = {"lower": ["conformance_bs_07"], "print": ["conformance_bs_06"], "validate": ["conformance_bs_05"], "moves": ["future_case", "ignored_future"]}
        for lane, names in inventories.items():
            self.add(self.producer, self.records, [self.target["executable"], "--list", *filters[lane]], listing(names), env={**self.target["env"], **self.binding["loader"], **self.binding["cargo_environment"]}, cwd=self.target["cwd"])
        for flag in ("-L", "-l"):
            self.add(self.producer, self.records, ["otool", flag, self.target["executable"]], "binary:\n\t/usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1.0.0)\n")
        archives = {}
        for name, files in (("conformance.tar.gz", {"harness/" + Path(self.target["executable"]).name: self.target["executable"]}), ("dependencies.tar.gz", {})):
            members = factory.archive(self.producer / name, files)
            archives[name] = {"file": factory.file_info(self.producer / name, name), "members": members}
        self.construction = {"schema": 1, "binding": self.binding, "targets": targets, "conformance": conformance, "doc_targets": docs, "filters": filters, "lanes": inventories, "archives": archives, "checks": copy.deepcopy(self.records)}
        factory.write_json(self.producer / "construction.json", self.construction)
        self.receipts = {}
        invocations = []
        for target in targets:
            if target["id"] != conformance:
                invocations.append(self.add(self.producer, self.records, [target["executable"], "--format=pretty"], execution(target["inventory"], target["ignored"]), kind="harness", env={**target["env"], **self.binding["loader"], **self.binding["cargo_environment"]}, cwd=target["cwd"], expected=target["inventory"], ignored=target["ignored"]))
        invocations.append(self.add(self.producer, self.records, factory.DOCS, execution([])*len(docs), stderr="".join(f"   Doc-tests {doc}\n" for doc in docs), kind="doctests", env=self.binding["cargo_environment"], expected=docs))
        self.receipts["producer"] = {"schema": 1, "part": "producer", "job": "intel-construction", "binding": self.binding, "construction_sha256": factory.digest(self.producer / "construction.json"), "invocations": invocations}
        factory.write_json(self.producer / "producer.json", self.receipts["producer"])
        for lane, names in inventories.items():
            directory = self.artifacts / ("intel-" + lane)
            directory.mkdir()
            records = []
            self.identity_checks(directory, records)
            binary = "/temp/consumer/runtime/harness/" + Path(self.target["executable"]).name
            env = {**self.target["env"], **self.binding["loader"], "CARGO_HOME": "/temp/consumer/dependencies/cargo", "CARGO_NET_OFFLINE": "true", "CARGO_REGISTRIES_CRATES_IO_PROTOCOL": "sparse", "CARGO_TERM_COLOR": "never", "CARGO_INCREMENTAL": "0", "DYLD_LIBRARY_PATH": "/temp/consumer/runtime/dylibs"}
            ignored = sorted(set(names) & set(self.target["ignored"]))
            self.add(directory, records, [binary, "--list", *filters[lane]], listing(names), cwd=self.target["cwd"], env=env)
            self.add(directory, records, [binary, "--list", "--ignored", *filters[lane]], listing(ignored), cwd=self.target["cwd"], env=env)
            run = self.add(directory, records, [binary, "--format=pretty", *filters[lane]], execution(names, ignored), kind="harness", cwd=self.target["cwd"], env=env, expected=names, ignored=ignored)
            self.receipts[lane] = {"schema": 1, "part": lane, "job": "intel-part", "binding": self.binding, "construction_sha256": factory.digest(self.producer / "construction.json"), "checks": records[:-1], "invocations": [run]}
            factory.write_json(directory / "receipt.json", self.receipts[lane])

    def build_output(self):
        return "".join(json.dumps(record) + "\n" for record in self.compiler)

    def identity_checks(self, directory, records):
        for command, output in ((["rustc", "-vV"], self.native["rustc"]), (["cargo", "-V"], self.native["cargo"]), (["rustc", "--print", "sysroot"], self.native["sysroot"] + "\n")):
            self.add(directory, records, command, output)

    def add(self, directory, records, command, stdout, stderr="", kind="setup", env=None, cwd=None, expected=None, ignored=None, exit_code=0):
        record = {"command": command, "cwd": cwd or str(self.root), "env": env or {}, "kind": kind, "exit_code": exit_code}
        if expected is not None:
            record.update(expected=expected, ignored=ignored or [])
        for channel, data in (("stdout", stdout), ("stderr", stderr)):
            path = f"logs/{len(records):04d}.{channel}.log"
            (directory / path).parent.mkdir(exist_ok=True)
            (directory / path).write_text(data)
            record[channel] = factory.file_info(directory / path, path)
        records.append(record)
        return record

    def save(self, part):
        path = self.producer / "producer.json" if part == "producer" else self.artifacts / ("intel-" + part) / "receipt.json"
        factory.write_json(path, self.receipts[part])

    def join(self):
        with patch.object(factory, "run_identity", return_value=self.binding["run"]), patch.object(factory, "source_identity", return_value=self.binding["source"]), contextlib.redirect_stdout(io.StringIO()):
            return factory.join(self.root, self.artifacts)


class FactoryTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.directory = Path(self.temp.name)

    def test_committed_lane_plan_is_complete_and_preserves_existing_parts(self):
        root = Path(__file__).resolve().parents[2]
        bdd, lanes = factory.plan(root)
        self.assertGreater(len(lanes["lanes"]), 0)
        self.assertEqual(set(lanes["producer"]), set(bdd["targets"]))

    def test_run_identity_names_this_repository_in_any_case(self):
        for name in ("UOR-Foundation/LexLean", "UOR-Foundation/lexlean"):
            with patch.dict(os.environ, {"GITHUB_REPOSITORY": name, "GITHUB_RUN_ID": "1", "GITHUB_RUN_ATTEMPT": "1"}):
                self.assertEqual(factory.run_identity()["GITHUB_REPOSITORY"], name)
        for name in ("someone/lexlean", "UOR-Foundation/lexlean-fork"):
            with patch.dict(os.environ, {"GITHUB_REPOSITORY": name, "GITHUB_RUN_ID": "1", "GITHUB_RUN_ATTEMPT": "1"}):
                with self.assertRaises(BaseException):
                    factory.run_identity()

    def test_plan_missing_duplicate_foreign_and_added_part_are_refused(self):
        fixture = Fixture(self.directory)
        for change in (lambda p: p["lanes"]["lower"].clear(), lambda p: p["lanes"]["print"].append("a"), lambda p: p["lanes"]["moves"].append("unknown")):
            with self.subTest(change=change):
                value = copy.deepcopy(fixture.lanes)
                change(value)
                factory.write_json(fixture.root / factory.LANES, value)
                with self.assertRaises(factory.Invalid):
                    factory.plan(fixture.root)
        factory.write_json(fixture.root / factory.LANES, fixture.lanes)
        fixture.bdd["parts"]["new-part"] = ["ZZ-01"]
        factory.write_json(fixture.root / factory.BDD, fixture.bdd)
        with self.assertRaisesRegex(factory.Invalid, "plan drift"):
            factory.plan(fixture.root)

    def test_list_parser_empty_duplicate_and_benchmark_semantics(self):
        self.assertEqual(factory.parse_list(listing([])), [])
        self.assertEqual(factory.parse_list("one: test\n\n1 test, 0 benchmarks\n"), ["one"])
        for text in ("", "x: test\nx: test\n2 tests, 0 benchmarks\n", "x: benchmark\n0 tests, 1 benchmark\n", "x: test\n0 tests, 0 benchmarks\n"):
            with self.subTest(text=text), self.assertRaises(factory.Invalid):
                factory.parse_list(text)

    def test_partition_rejects_empty_missing_duplicate_foreign_and_named_drift(self):
        bdd = {"parts": {"p": ["AA-01"]}}
        whole = ["conformance_aa_01", "future"]
        factory.partition(whole, {"p": ["conformance_aa_01"], "rest": ["future"]}, bdd)
        for original, selected in (([], {}), (whole, {"p": whole, "rest": []}), (whole, {"p": [whole[0]]}), (whole, {"p": whole, "rest": ["future"]}), (whole, {"p": whole, "rest": ["foreign"]}), (["future"], {"p": ["future"]})):
            with self.subTest(selected=selected), self.assertRaises(factory.Invalid):
                factory.partition(original, selected, bdd)

    def test_listing_is_not_execution_even_for_empty_target(self):
        factory.check_execution(execution([]), [], [])
        for names in ([], ["a"]):
            with self.assertRaises(factory.Invalid):
                factory.check_execution(listing(names), names, [])

    def test_ignored_cases_preserve_default_cargo_semantics(self):
        factory.check_execution(execution(["a", "b"], ["b"]), ["a", "b"], ["b"])
        for output in (execution(["a", "b"]), execution(["a", "b"], ["a", "b"]), execution(["a", "b"], ["b"], ["a"]), execution(["a"], [])):
            with self.subTest(output=output), self.assertRaises(factory.Invalid):
                factory.check_execution(output, ["a", "b"], ["b"])

    def test_archive_roundtrip_preserves_executable_mode(self):
        binary = self.directory / "binary"
        binary.write_bytes(b"binary")
        binary.chmod(0o755)
        path = self.directory / "payload.tar.gz"
        entries = factory.archive(path, {"harness/binary": binary})
        factory.inspect_archive(path, entries, self.directory / "extracted")
        self.assertEqual((self.directory / "extracted/harness/binary").read_bytes(), b"binary")
        self.assertEqual(stat.S_IMODE((self.directory / "extracted/harness/binary").stat().st_mode), 0o755)
        with self.assertRaises(factory.Invalid):
            factory.inspect_archive(path, entries, self.directory / "extracted")

    def test_archive_rejects_paths_links_devices_duplicates_and_extra_members(self):
        for name, kind in (("../escape", tarfile.REGTYPE), ("/absolute", tarfile.REGTYPE), ("harness/../escape", tarfile.REGTYPE), ("harness\\escape", tarfile.REGTYPE), ("harness/link", tarfile.SYMTYPE), ("harness/link", tarfile.LNKTYPE), ("harness/device", tarfile.CHRTYPE)):
            with self.subTest(name=name, kind=kind):
                path = self.directory / "bad.tar.gz"
                with tarfile.open(path, "w:gz") as tar:
                    member = tarfile.TarInfo(name)
                    member.type = kind
                    member.linkname = "/outside" if kind in (tarfile.SYMTYPE, tarfile.LNKTYPE) else ""
                    tar.addfile(member, io.BytesIO())
                entry = {"path": name, "size": 0, "mode": 0o644, "sha256": factory.hashlib.sha256(b"").hexdigest()}
                with self.assertRaises(factory.Invalid):
                    factory.inspect_archive(path, [entry])
        path = self.directory / "duplicate.tar.gz"
        with tarfile.open(path, "w:gz") as tar:
            for _ in range(2):
                tar.addfile(tarfile.TarInfo("same"), io.BytesIO())
        with self.assertRaises(factory.Invalid):
            factory.inspect_archive(path, [{"path": "same", "size": 0, "mode": 0o644, "sha256": "x"}])

    def test_archive_rejects_symlink_destination_and_digest_or_mode_changes(self):
        source = self.directory / "source"
        source.write_bytes(b"x")
        path = self.directory / "good.tar.gz"
        entries = factory.archive(path, {"harness/x": source})
        for key, value in (("sha256", "0"*64), ("mode", 0o777), ("size", 2)):
            altered = copy.deepcopy(entries)
            altered[0][key] = value
            with self.subTest(key=key), self.assertRaises(factory.Invalid):
                factory.inspect_archive(path, altered)
        (self.directory / "link").symlink_to(self.directory, target_is_directory=True)
        with self.assertRaises(factory.Invalid):
            factory.inspect_archive(path, entries, self.directory / "link/extract")

    def test_dependency_archive_never_includes_home_credentials_config_or_git(self):
        home = self.directory / "cargo-home"
        (home / "registry/src/index.crates.io-1234/package").mkdir(parents=True)
        (home / "registry/src/index.crates.io-1234/package/Cargo.toml").write_text("public package")
        (home / "credentials.toml").write_text("private")
        (home / "config.toml").write_text("private")
        files = factory.dependency_files(home)
        self.assertEqual(list(files), ["cargo/registry/src/index.crates.io-1234/package/Cargo.toml"])
        (home / "git/db/private").mkdir(parents=True)
        with self.assertRaisesRegex(factory.Invalid, "Git"):
            factory.dependency_files(home)

    def test_resolved_metadata_rejects_external_path_and_non_crates_io_sources(self):
        fixture = Fixture(self.directory)
        factory.dependency_sources(fixture.metadata, fixture.root)
        for source, manifest in ((None, "/outside/Cargo.toml"), (None, str(fixture.root / "target/generated/Cargo.toml")),
                                 ("git+https://example.com/repo", str(fixture.root / "git/Cargo.toml")),
                                 ("registry+https://private.example", str(fixture.root / "registry/Cargo.toml"))):
            metadata = copy.deepcopy(fixture.metadata)
            metadata["packages"].append({"name": "unsupported", "source": source, "manifest_path": manifest})
            with self.subTest(source=source, manifest=manifest), self.assertRaises(factory.Invalid):
                factory.dependency_sources(metadata, fixture.root)
        metadata = copy.deepcopy(fixture.metadata)
        metadata["packages"].append({"name": "public", "source": "registry+https://github.com/rust-lang/crates.io-index", "manifest_path": "/cargo-home/registry/src/public/Cargo.toml"})
        factory.dependency_sources(metadata, fixture.root)

    def test_dependency_symlink_alternate_registry_and_index_config_refused(self):
        home = self.directory / "home"
        (home / "registry/index/index.crates.io-1234").mkdir(parents=True)
        evil = home / "registry/index/index.crates.io-1234/config"
        evil.write_text("extraheader = private")
        with self.assertRaises(factory.Invalid):
            factory.dependency_files(home)
        evil.unlink()
        evil.symlink_to(self.directory)
        with self.assertRaises(factory.Invalid):
            factory.dependency_files(home)
        evil.unlink()
        (home / "registry/index/private.example-1234").mkdir()
        with self.assertRaises(factory.Invalid):
            factory.dependency_files(home)

    def test_complete_join_accepts_future_targets_and_ignored_tests_on_non_intel(self):
        fixture = Fixture(self.directory)
        self.assertEqual(fixture.join(), 0)
        self.assertIn("ordinary/test/future_integration", [t["id"] for t in fixture.construction["targets"]])

    def test_join_current_source_run_and_attempt_must_match_construction(self):
        fixture = Fixture(self.directory)
        for field in ("commit", "tree", "run", "attempt"):
            source, run = copy.deepcopy(fixture.binding["source"]), copy.deepcopy(fixture.binding["run"])
            if field in ("commit", "tree"):
                source[field] = "f" * 40
            else:
                run["GITHUB_RUN_ID" if field == "run" else "GITHUB_RUN_ATTEMPT"] = "999"
            with patch.object(factory, "source_identity", return_value=source), patch.object(factory, "run_identity", return_value=run):
                with self.subTest(field=field), self.assertRaises(factory.Invalid):
                    factory.join(fixture.root, fixture.artifacts)

    def test_join_missing_duplicate_and_unexpected_artifact_refused(self):
        for mode in ("missing", "duplicate", "extra"):
            base = self.directory / mode
            base.mkdir()
            fixture = Fixture(base)
            if mode == "missing":
                shutil.rmtree(fixture.artifacts / "intel-lower")
            elif mode == "duplicate":
                shutil.copytree(fixture.artifacts / "intel-lower", fixture.artifacts / "intel-lower-copy")
            else:
                (fixture.artifacts / "unrelated").write_text("unexpected")
            with self.subTest(mode=mode), self.assertRaises(factory.Invalid):
                fixture.join()

    def test_join_foreign_failed_cancelled_and_mismatched_receipts_refused(self):
        for mode in ("foreign", "failed", "cancelled", "digest", "job", "part"):
            base = self.directory / mode
            base.mkdir()
            fixture = Fixture(base)
            receipt = fixture.receipts["lower"]
            if mode == "foreign":
                receipt["binding"] = copy.deepcopy(receipt["binding"])
                receipt["binding"]["run"]["GITHUB_RUN_ATTEMPT"] = "2"
            elif mode in ("failed", "cancelled"):
                receipt["invocations"][0]["exit_code"] = 101 if mode == "failed" else -15
            elif mode == "digest":
                receipt["construction_sha256"] = "0"*64
            elif mode == "job":
                receipt["job"] = "someone-else"
            else:
                receipt["part"] = "moves"
            fixture.save("lower")
            with self.subTest(mode=mode), self.assertRaises(factory.Invalid):
                fixture.join()

    def test_join_changed_log_extra_file_missing_and_duplicate_invocations_refused(self):
        for mode in ("log", "file", "missing", "duplicate"):
            base = self.directory / mode
            base.mkdir()
            fixture = Fixture(base)
            record = fixture.receipts["lower"]["invocations"][0]
            if mode == "log":
                (fixture.artifacts / "intel-lower" / record["stdout"]["path"]).write_text("forged")
            elif mode == "file":
                (fixture.artifacts / "intel-lower/other.txt").write_text("unexpected")
            elif mode == "missing":
                fixture.receipts["lower"]["invocations"].clear()
            else:
                fixture.receipts["lower"]["invocations"].append(record)
            fixture.save("lower")
            with self.subTest(mode=mode), self.assertRaises(factory.Invalid):
                fixture.join()

    def test_join_rejects_resealed_listing_as_execution_and_changed_environment(self):
        for mode in ("listing", "RUSTFLAGS", "RUSTC_WRAPPER", "sparse", "CARGO_INCREMENTAL"):
            base = self.directory / mode
            base.mkdir()
            fixture = Fixture(base)
            record = fixture.receipts["lower"]["invocations"][0]
            if mode == "listing":
                path = fixture.artifacts / "intel-lower" / record["stdout"]["path"]
                path.write_text(listing(record["expected"]))
                record["stdout"] = factory.file_info(path, record["stdout"]["path"])
            elif mode == "sparse":
                record["env"]["CARGO_REGISTRIES_CRATES_IO_PROTOCOL"] = "git"
            else:
                record["env"][mode] = "other"
            fixture.save("lower")
            with self.subTest(mode=mode), self.assertRaises(factory.Invalid):
                fixture.join()

    def test_actual_cargo_metadata_discovers_new_target_and_missing_binary_fails(self):
        fixture = Fixture(self.directory)
        target = {"name": "another_future", "kind": ["test"], "test": True, "doctest": False}
        fixture.metadata["packages"][0]["targets"].append(target)
        with self.assertRaisesRegex(factory.Invalid, "not constructed"):
            factory.built_targets(fixture.metadata, fixture.build_output(), fixture.root)

    def test_ordinary_failure_does_not_hide_later_target_and_doctest_records(self):
        fixture = Fixture(self.directory)
        receipt = fixture.receipts["producer"]
        receipt["invocations"][0]["exit_code"] = 101
        fixture.save("producer")
        with self.assertRaisesRegex(factory.Invalid, "producer: failed invocation"):
            fixture.join()
        self.assertGreater(len(receipt["invocations"]), 2)
        self.assertEqual(receipt["invocations"][-1]["kind"], "doctests")

    def test_should_panic_annotation_is_execution_of_original_name(self):
        factory.check_execution("test expects_panic - should panic ... ok\n" +
                                "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n",
                                ["expects_panic"], [])

    def test_tampered_construction_list_logs_and_missing_checks_are_refused(self):
        for mode in ("missing", "list"):
            base = self.directory / mode
            base.mkdir()
            fixture = Fixture(base)
            if mode == "missing":
                fixture.construction["checks"].pop()
            else:
                record = next(r for r in fixture.construction["checks"] if r["command"] == [fixture.target["executable"], "--list"])
                path = fixture.producer / record["stdout"]["path"]
                path.write_text(listing(["foreign_case"]))
                record["stdout"] = factory.file_info(path, record["stdout"]["path"])
            factory.write_json(fixture.producer / "construction.json", fixture.construction)
            for part, receipt in fixture.receipts.items():
                receipt["construction_sha256"] = factory.digest(fixture.producer / "construction.json")
                fixture.save(part)
            with self.subTest(mode=mode), self.assertRaises(factory.Invalid):
                fixture.join()

    def test_native_identity_rejects_target_specific_compiler_overrides(self):
        fixture = Fixture(self.directory)
        for name in ("RUSTFLAGS", "CARGO_TARGET_X86_64_APPLE_DARWIN_RUSTFLAGS", "CARGO_BUILD_RUSTFLAGS", "RUSTC_WORKSPACE_WRAPPER"):
            with patch.dict(os.environ, {name: "unexpected"}), patch.object(factory.platform, "system", return_value="Darwin"), patch.object(factory.platform, "machine", return_value="x86_64"):
                with self.subTest(name=name), self.assertRaisesRegex(factory.Invalid, "overrides"):
                    factory.native_identity(fixture.root, factory.Recorder(self.directory / name))

    def test_real_fake_executables_construct_execute_and_join_with_producer_failure(self):
        # Exercise the actual orchestration, subprocess capture, tar transport,
        # consumer extraction and join. Only Rust/macOS tools and git identity
        # are substituted; no manually authored successful execution receipts.
        for fail in (False, True):
            base = self.directory / ("failure" if fail else "success")
            base.mkdir()
            fixture = Fixture(base)
            shutil.rmtree(fixture.artifacts)
            fixture.artifacts.mkdir()
            tools = base / "tools"
            tools.mkdir()
            def program(name, code):
                path = tools / name
                path.write_text("#!" + sys.executable + "\n" + code)
                path.chmod(0o755)
                return path
            cargo = "import json,sys\nargs=sys.argv[1:]\n"
            cargo += "if args[0] == '-V': print('cargo 1.97.1')\n"
            cargo += "elif args[0] == 'metadata': print(" + repr(json.dumps(fixture.metadata)) + ")\n"
            cargo += "elif '--no-run' in args: print(" + repr(fixture.build_output()) + ", end='')\n"
            docs = ["ordinary", "repo_conformance"]
            cargo += "elif '--doc' in args:\n"
            cargo += " for target in " + repr(docs) + ":\n  print('   Doc-tests ' + target, file=sys.stderr)\n  print(" + repr(execution([])) + ", end='')\n"
            cargo += "else: sys.exit(77)\n"
            program("cargo", cargo)
            program("rustc", "import sys\nprint(" + repr(fixture.native["rustc"].strip()) + " if '-vV' in sys.argv else '/toolchain')\n")
            program("otool", "import sys\nprint(sys.argv[-1] + ':\\n\\t/usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1.0.0)')\n")
            for binary, names in fixture.inventories.items():
                ignored = ["ignored_future"] if "ignored_future" in names else []
                failed = ["ordinary_test"] if fail and "ordinary_test" in names else []
                code = "import sys\nnames=" + repr(names) + "\nignored=" + repr(ignored) + "\nfailed=" + repr(failed) + "\n"
                code += "args=iter(sys.argv[1:]); positive=[]; skips=[]; only_ignored=False; is_list=False\n"
                code += "for arg in args:\n if arg == '--skip': skips.append(next(args))\n elif arg == '--ignored': only_ignored=True\n elif arg == '--list': is_list=True\n elif not arg.startswith('--'): positive.append(arg)\n"
                code += "selected=sorted(n for n in names if (not positive or n in positive) and n not in skips and (not only_ignored or n in ignored))\n"
                code += "if is_list:\n for name in selected: print(name + ': test')\n print(str(len(selected)) + ' tests, 0 benchmarks')\n"
                code += "else:\n for name in selected: print('test ' + name + ' ... ' + ('FAILED' if name in failed else 'ignored' if name in ignored else 'ok'))\n nfailed=len(set(selected)&set(failed)); nignored=len(set(selected)&set(ignored))\n print('test result: ' + ('FAILED' if nfailed else 'ok') + '. ' + str(len(selected)-nfailed-nignored) + ' passed; ' + str(nfailed) + ' failed; ' + str(nignored) + ' ignored; 0 measured; 0 filtered out; finished in 0.01s')\n sys.exit(1 if nfailed else 0)\n"
                Path(binary).write_text("#!" + sys.executable + "\n" + code)
                Path(binary).chmod(0o755)
            output_file = base / "outputs"
            env = {**fixture.binding["run"], "GITHUB_JOB": "intel-construction", "GITHUB_OUTPUT": str(output_file), "PATH": str(tools) + os.pathsep + os.environ["PATH"]}
            with patch.dict(os.environ, env), patch.object(factory, "source_identity", return_value=fixture.binding["source"]), patch.object(factory.platform, "system", return_value="Darwin"), patch.object(factory.platform, "machine", return_value="x86_64"):
                self.assertEqual(factory.build(fixture.root, fixture.producer), 0)
                lanes = json.loads(output_file.read_text().split("=", 1)[1])
                self.assertEqual(set(lanes), set(fixture.lanes["lanes"]))
                with patch.dict(os.environ, {"GITHUB_JOB": "intel-part"}):
                    for lane in lanes:
                        self.assertEqual(factory.execute(fixture.root, fixture.producer, lane, fixture.artifacts / ("intel-" + lane)), 0)
                with patch.dict(os.environ, {"GITHUB_JOB": "intel-join"}):
                    if fail:
                        with self.assertRaisesRegex(factory.Invalid, "producer: failed invocation"):
                            factory.join(fixture.root, fixture.artifacts)
                    else:
                        self.assertEqual(factory.join(fixture.root, fixture.artifacts), 0)

    def test_recorder_captures_real_process_failures_and_later_success(self):
        recorder = factory.Recorder(self.directory / "recorded")
        failure = recorder.run([sys.executable, "-c", "import sys; print('failed stdout'); print('failed stderr', file=sys.stderr); sys.exit(3)"], self.directory)
        success = recorder.run([sys.executable, "-c", "print('later success')"], self.directory)
        self.assertEqual((failure["exit_code"], success["exit_code"]), (3, 0))
        self.assertIn("failed stderr", recorder.output_text(failure, "stderr"))
        factory.check_records(recorder.output, recorder.records)


if __name__ == "__main__":
    unittest.main()
