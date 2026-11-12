"""Behavioral tests of the host lanes' join (SPEC §8.3), run in honesty."""

import copy
from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parent))
import host_lanes  # noqa: E402

BDD = {"parts": {"one": ["AA-01"], "two": ["AA-02"]}, "targets": {"honesty": ["bdd"]}, "remainder": "rest"}
PLAN = {"producer": ["honesty"], "lanes": {"a": ["one"], "b": ["two", "rest"]}}
NAMES = ["producer", "a", "b"]
HERE = {"commit": "c" * 40, "tree": "t" * 40, "run": "7", "attempt": "1"}
HARNESS = ["conformance_aa_01", "conformance_aa_02", "conformance_aa_03"]


def receipts():
    base = {"schema": 1, "target": "x86_64-pc-windows-msvc", "passed": True, "conformance": HARNESS,
            "whole": None, **HERE}
    return {
        "producer": {**base, "lane": "producer", "executed": ["meta", "unit"],
                     "whole": sorted(["meta", "unit", *HARNESS])},
        "a": {**base, "lane": "a", "executed": ["conformance_aa_01"]},
        "b": {**base, "lane": "b", "executed": ["conformance_aa_02", "conformance_aa_03"]},
    }


class Join(unittest.TestCase):
    def check(self, value):
        return host_lanes.check_join(value, NAMES, PLAN, BDD, HERE)

    def refuse(self, value):
        with self.assertRaises(host_lanes.Invalid):
            self.check(value)

    def test_complete_lanes_pass(self):
        self.assertEqual(self.check(receipts()), 5)

    def test_missing_lane(self):
        value = receipts()
        del value["b"]
        self.refuse(value)

    def test_test_run_twice(self):
        value = receipts()
        value["a"]["executed"].append("conformance_aa_02")
        self.refuse(value)

    def test_test_lost(self):
        value = receipts()
        value["b"]["executed"] = ["conformance_aa_02"]
        self.refuse(value)

    def test_failed_lane(self):
        value = receipts()
        value["a"]["passed"] = False
        self.refuse(value)

    def test_other_commit_or_run(self):
        for field in ("commit", "tree", "run"):
            value = receipts()
            value["a"][field] = "x"
            self.refuse(value)

    def test_different_harness(self):
        value = receipts()
        value["b"] = copy.deepcopy(value["b"])
        value["b"]["conformance"] = HARNESS[:2]
        self.refuse(value)

    def test_conformance_lane_outside_harness(self):
        value = receipts()
        value["a"]["executed"] = ["meta"]
        value["producer"]["executed"] = ["conformance_aa_01", "unit"]
        self.refuse(value)

    def test_named_test_absent(self):
        value = receipts()
        for receipt in value.values():
            receipt["conformance"] = ["conformance_aa_02", "conformance_aa_03"]
        value["producer"]["whole"] = sorted(["meta", "unit", "conformance_aa_02", "conformance_aa_03"])
        value["a"]["executed"] = []
        self.refuse(value)


if __name__ == "__main__":
    unittest.main()
