#!/usr/bin/env python3
"""Small genuine Git regressions; no compiler or generated-oracle editing."""

import contextlib
import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch


SPEC = importlib.util.spec_from_file_location("capture", Path(__file__).with_name("capture-generated.py"))
CAPTURE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CAPTURE)


class CaptureTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="lexlean-capture-test-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.root = self.base / "source"
        self.root.mkdir()
        self.evidence = self.base / "evidence"
        self.evidence.mkdir()
        self.run_git("init", "--quiet")
        self.run_git("config", "user.name", "Capture fixture")
        self.run_git("config", "user.email", "capture@example.invalid")
        for name, data in {
            ".gitignore": "*.ignored\n",
            "examples/example/lexlean.toml": "original config\n",
            "examples/example/expected/deleted.txt": "retired by writer\n",
            "examples/example/expected/changed.txt": "before\n",
            "examples/example/expected/unchanged.txt": "unchanged\n",
            "compiler/lexlean.lock": "original lock\n",
            "compiler/expected/manifest.json": "{}\n",
            "compiler/rust/package.json": "{}\n",
            "language/preservation-1.2/modules/Test.lean": "original module\n",
            "tests/fixtures/suite/case/case.toml": "nested case\n",
            "tests/fixtures/suite/case/expected/result.json": "{}\n",
            "tests/negative/case/case.toml": "negative case\n",
            "tests/negative/case/project/lexlean.toml": "negative config\n",
            "tests/negative/case/project/lexlean.lock": "negative lock\n",
            "tests/negative/case/expected/result.json": "{}\n",
        }.items():
            destination = self.root / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_text(data)
        self.run_git("add", "--all")
        self.run_git("commit", "--quiet", "-m", "fixture: base")
        self.head = self.run_git("rev-parse", "HEAD").decode().strip()
        self.deleted = "examples/example/expected/deleted.txt"
        self.changed = "examples/example/expected/changed.txt"
        self.added = "examples/example/expected/added.txt"
        (self.root / self.deleted).unlink()
        (self.root / self.changed).write_text("after\n")
        (self.root / self.added).write_bytes(b"actual\0binary\xff\n")
        self.run_git("add", "--intent-to-add", "--", *CAPTURE.SCOPES)

    def run_git(self, *args):
        result = subprocess.run(["git", *args], cwd=self.root, capture_output=True, timeout=20)
        self.assertEqual(result.returncode, 0, result.stderr)
        return result.stdout

    def capture(self):
        with contextlib.redirect_stdout(io.StringIO()):
            CAPTURE.capture(self.root, self.evidence, self.head)
        return json.loads((self.evidence / "generated-tree.json").read_bytes())

    def git_inventory(self):
        return {str(p.relative_to(self.root)): hashlib.sha256(p.read_bytes()).hexdigest()
                for p in (self.root / ".git").rglob("*") if p.is_file()}

    def test_complete_binary_patch_and_inventory_preserve_actual_index(self):
        index_before = (self.root / ".git/index").read_bytes()
        legacy = self.run_git("diff", "--binary")
        self.assertNotIn(self.deleted.encode(), legacy)
        self.assertIn(self.deleted.encode(), self.run_git("diff", "--cached", "--name-only"))
        inventory = self.capture()
        self.assertTrue(inventory["complete"])
        self.assertEqual(inventory["deleted_generated_paths"], [self.deleted])
        self.assertNotIn(self.deleted, inventory["generated_files"])
        self.assertEqual(inventory["generated_files"][self.added]["sha256"],
                         hashlib.sha256((self.root / self.added).read_bytes()).hexdigest())
        self.assertIn("tests/fixtures/suite/case/expected/result.json", inventory["generated_files"])
        self.assertEqual(inventory["result_tree"], inventory["reconstructed_tree"])
        self.assertIn(self.deleted.encode(), (self.evidence / "generated.patch").read_bytes())
        self.assertIn(self.deleted.encode(), (self.evidence / "generated-stat.txt").read_bytes())
        self.assertEqual(index_before, (self.root / ".git/index").read_bytes())

    def test_clone_git_metadata_and_objects_are_not_written(self):
        before = self.git_inventory()
        self.capture()
        self.assertEqual(before, self.git_inventory())

    def test_clean_verify_capture_has_empty_patch_and_complete_inventory(self):
        self.run_git("add", "--all")
        self.run_git("commit", "--quiet", "-m", "fixture: current tree")
        self.head = self.run_git("rev-parse", "HEAD").decode().strip()
        inventory = self.capture()
        self.assertEqual((self.evidence / "generated.patch").read_bytes(), b"")
        self.assertEqual(inventory["deleted_generated_paths"], [])
        self.assertEqual(inventory["source_tree"], inventory["result_tree"])

    def test_executed_original_worktree_only_diff_mutant_refused(self):
        original = CAPTURE.git

        def legacy(root, *args, **kwargs):
            if args[:2] == ("diff", "--binary"):
                args = tuple(value for value in args if value != "HEAD")
            return original(root, *args, **kwargs)

        with patch.object(CAPTURE, "git", side_effect=legacy):
            with self.assertRaisesRegex(ValueError, "does not reconstruct the complete result tree"):
                self.capture()
        self.assertFalse((self.evidence / "generated-tree.json").exists())

    def test_missing_inventory_member_refused(self):
        original = CAPTURE.snapshot

        def omitted(*args):
            rows, identities = original(*args)
            rows.pop(self.added)
            return rows, identities

        with patch.object(CAPTURE, "snapshot", side_effect=omitted):
            with self.assertRaisesRegex(ValueError, "Physical generated inventory differs"):
                self.capture()

    def test_corrupt_inventory_digest_refused(self):
        original = CAPTURE.snapshot

        def changed(*args):
            rows, identities = original(*args)
            rows[self.changed]["git_blob"] = "0" * 40
            return rows, identities

        with patch.object(CAPTURE, "snapshot", side_effect=changed):
            with self.assertRaisesRegex(ValueError, "Generated bytes or mode differ"):
                self.capture()

    def test_ignored_extra_generated_member_refused(self):
        (self.root / "examples/example/expected/extra.ignored").write_text("not captured by Git\n")
        with self.assertRaisesRegex(ValueError, "Physical generated inventory differs"):
            self.capture()

    def test_authored_source_change_refused(self):
        (self.root / "examples/example/lexlean.toml").write_text("changed config\n")
        with self.assertRaisesRegex(ValueError, "Non-generated source changed"):
            self.capture()

    def test_untracked_outside_capture_scope_refused(self):
        (self.root / "unregistered.txt").write_text("not an owning output\n")
        with self.assertRaisesRegex(ValueError, "Uncaptured untracked source remains"):
            self.capture()

    def test_change_during_final_readback_refused(self):
        original = CAPTURE.snapshot
        calls = 0

        def changed(*args):
            nonlocal calls
            calls += 1
            if calls == 2:
                (self.root / self.changed).write_text("later bytes\n")
            return original(*args)

        with patch.object(CAPTURE, "snapshot", side_effect=changed):
            with self.assertRaisesRegex(ValueError, "Generated inventory changed during capture"):
                self.capture()

    def test_tracked_outside_scope_mutation_during_capture_refused(self):
        original = CAPTURE.snapshot
        calls = 0

        def changed(*args):
            nonlocal calls
            calls += 1
            if calls == 2:
                # Existing tracked file, outside every generated directory and
                # every scope staged by the capture's private actual index.
                (self.root / ".gitignore").write_text("*.ignored\nlate-source-drift\n")
            return original(*args)

        with patch.object(CAPTURE, "snapshot", side_effect=changed):
            with self.assertRaisesRegex(ValueError, "Whole source diff changed during capture"):
                self.capture()
        self.assertFalse((self.evidence / "generated-tree.json").exists())

    def test_file_and_aggregate_capture_bounds_refused(self):
        with patch.object(CAPTURE, "CAP", 1):
            with self.assertRaises(ValueError):
                CAPTURE.fingerprint(self.root / self.changed)
        with patch.object(CAPTURE, "MAX_TOTAL", 1):
            with self.assertRaisesRegex(ValueError, "exceeds byte bound"):
                self.capture()

    def test_read_growth_and_truncation_refused(self):
        original = CAPTURE.os.read
        target = self.root / self.changed
        touched = False

        def grew(fd, count):
            nonlocal touched
            if not touched:
                with target.open("ab") as output:
                    output.write(b"extra")
                touched = True
            return original(fd, count)

        with patch.object(CAPTURE.os, "read", side_effect=grew):
            with self.assertRaisesRegex(ValueError, "Generated entry grew"):
                CAPTURE.fingerprint(target)

        def truncated(fd, count):
            target.write_bytes(b"")
            return original(fd, count)

        with patch.object(CAPTURE.os, "read", side_effect=truncated):
            with self.assertRaisesRegex(ValueError, "Generated entry truncated"):
                CAPTURE.fingerprint(target)

    def test_symlink_and_hardlink_refused(self):
        target = self.root / self.added
        target.unlink()
        target.symlink_to("changed.txt")
        with self.assertRaisesRegex(ValueError, "single-link regular"):
            self.capture()
        target.unlink()
        os.link(self.root / self.changed, target)
        with self.assertRaisesRegex(ValueError, "single-link regular"):
            self.capture()

    def test_stale_head_refused(self):
        self.head = "0" * 40
        with self.assertRaisesRegex(ValueError, "Source HEAD changed"):
            self.capture()

    def test_existing_evidence_refused_without_overwrite(self):
        (self.evidence / "generated.patch").write_bytes(b"retained old evidence")
        with self.assertRaises(FileExistsError):
            self.capture()
        self.assertEqual((self.evidence / "generated.patch").read_bytes(), b"retained old evidence")


if __name__ == "__main__":
    unittest.main()
