import json
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest import mock

import forward_cycle_writer as w


class CycleTests(unittest.TestCase):
    def test_dependency_order_rebuilds_after_every_writer(self):
        events = []
        w.producer_round(lambda label, command: events.append(command),
                         lambda label: events.append('rebuild-observe-relock'), 1)
        self.assertEqual(events, ['check-calculus', 'rebuild-observe-relock',
                                 'check-production-export', 'rebuild-observe-relock',
                                 'check-golden', 'rebuild-observe-relock',
                                 'check-calculus', 'rebuild-observe-relock'])

    def test_stable_round_passes(self):
        records = []
        count = w.bounded_cycle(lambda n: None, lambda: {'x': 1}, lambda *x: records.append(x))
        self.assertEqual(count, 1)
        self.assertEqual(records, [(1, [], True)])

    def test_requires_full_product_state_not_only_identity(self):
        state = {'identity': 'same', 'product': 0}
        records = []
        def step(n):
            state['product'] = min(n, 2)
        count = w.bounded_cycle(step, lambda: dict(state), lambda *x: records.append(x))
        self.assertEqual(count, 3)
        self.assertEqual([r[2] for r in records], [False, False, True])

    def test_unbounded_drift_stops_at_three(self):
        state = {'product': 0}
        calls = []
        def step(n):
            calls.append(n)
            state['product'] = n
        with self.assertRaisesRegex(RuntimeError, 'within 3'):
            w.bounded_cycle(step, lambda: dict(state), lambda *x: None)
        self.assertEqual(calls, [1, 2, 3])

    def test_oscillation_stops_without_retry(self):
        state = {'product': 0}
        calls = []
        def step(n):
            calls.append(n)
            state['product'] = n % 2
        with self.assertRaisesRegex(RuntimeError, 'oscillated'):
            w.bounded_cycle(step, lambda: dict(state), lambda *x: None)
        self.assertEqual(calls, [1, 2])

    def test_writer_failure_stops_original_round(self):
        events = []
        def writer(label, command):
            events.append(command)
            if command == 'check-production-export':
                raise RuntimeError('original native failure')
        with self.assertRaisesRegex(RuntimeError, 'original native failure'):
            w.bounded_cycle(lambda n: w.producer_round(writer, lambda x: None, n),
                            lambda: {}, lambda *x: None)
        self.assertEqual(events, ['check-calculus', 'check-production-export'])

    def make_source(self, root):
        paths = ['examples/new', 'compiler', 'stdlib', 'std-models', 'std-reasoning', 'kappa', 'bootstrap', 'tests/tooling/core']
        for path in paths:
            p = root / path
            p.mkdir(parents=True)
            (p / 'lexlean.toml').write_text('project input')
        data = root / w.SEED_DATA
        data.mkdir(parents=True)
        (data / 'admission.json').write_text(json.dumps({'ordinary': [{'path': 'examples/new/lexlean.toml', 'lock': 'examples/new/lexlean.lock'}]}))
        (data / 'observe_forward_identity.rs').write_text('fake observer source')
        (root / 'rust-toolchain.toml').write_text('[toolchain]\nchannel = "1.97.1"\n')
        (root / 'lean-toolchain').write_text('leanprover/lean4:v4.32.1\n')
        subprocess.run(['git', 'init', '-q', str(root)], check=True)
        subprocess.run(['git', '-C', str(root), 'add', '.'], check=True)
        subprocess.run(['git', '-C', str(root), '-c', 'user.name=Test', '-c', 'user.email=test@example.invalid',
                        'commit', '-q', '-m', 'test'], check=True)
        return w.git(root, 'rev-parse', 'HEAD').decode().strip(), w.git(root, 'rev-parse', 'HEAD^{tree}').decode().strip()

    def test_scopes_preserve_authored_and_fixture_inputs(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            self.make_source(root)
            scopes = w.product_scopes(root)
            for protected in ['examples/new/src/Main.lex.tex', 'examples/new/expected/verify/a.json',
                              'tests/production-export/production/roots.json', 'tests/fixtures/cf-10/lexlean.lock',
                              'tests/negative/stale-lock/expected/diagnostics.json', 'compiler/src/TargetSyntax.lex.tex']:
                self.assertFalse(any(allowed(protected) for allowed in scopes.values()), protected)
            self.assertTrue(scopes['check-golden']('examples/new/expected/build/new.lean'))
            self.assertTrue(scopes['check-golden']('tests/tooling/core/expected/build/new.lean'))
            self.assertTrue(scopes['check-production-export']('tests/production-export/stdlib/packages/rust-std/sha256/src/lib.rs'))
            self.assertFalse(scopes['check-golden']('examples/new/expected/build-not/a'))

    def test_fake_run_success_and_original_failure_custody(self):
        for fail in (False, True):
            with self.subTest(fail=fail), tempfile.TemporaryDirectory() as d:
                root, out = Path(d) / 'repo', Path(d) / 'evidence'
                head, tree = self.make_source(root)
                original_run, original_output = subprocess.run, subprocess.check_output
                events = []
                def lookup(argv, **kwargs):
                    if argv[0] in ('rustup', 'elan'):
                        return '/bin/true\n'
                    return original_output(argv, **kwargs)
                def native(argv, **kwargs):
                    if argv[0] == 'git':
                        return original_run(argv, **kwargs)
                    name = Path(argv[0]).name
                    events.append(argv)
                    code, log = 0, ''
                    if argv[:2] == ['cargo', 'build']:
                        for p in ('xtask', 'lexlean', 'examples/observe_forward_identity'):
                            target = out / 'build/debug' / p
                            target.parent.mkdir(parents=True, exist_ok=True)
                            target.write_text('fake binary')
                    elif name in ('cargo', 'rustc', 'lean'):
                        log = {'cargo': 'cargo 1.97.1 fake', 'rustc': 'rustc 1.97.1 fake', 'lean': 'Lean (version 4.32.1, fake)'}[name]
                    elif name == 'observe_forward_identity':
                        log = json.dumps({'renderer_matches': True, 'renderer_observed': 'r', 'errors_sha256': 'e', 'compiler_semantics_1_2': 's'})
                    elif name == 'lexlean':
                        (root / 'examples/new/lexlean.lock').write_text('official fake lock')
                    elif name == 'xtask' and '--write' in argv:
                        if fail and argv[1] == 'check-production-export':
                            (root / w.OVERLAY).unlink()
                            code, log = 7, 'original production failure'
                        else:
                            path = {'check-calculus': 'compiler/src/TargetFixtures.lex.tex',
                                    'check-production-export': 'tests/production-export/production/packages/p/src/lib.rs',
                                    'check-golden': 'examples/new/expected/build/Main.lean'}[argv[1]]
                            target = root / path
                            target.parent.mkdir(parents=True, exist_ok=True)
                            target.write_text('official fake product')
                    kwargs['stdout'].write(log.encode())
                    return subprocess.CompletedProcess(argv, code)
                with mock.patch.object(w, 'BASE', head), mock.patch.object(w, 'BASE_TREE', tree), \
                        mock.patch.object(w.subprocess, 'check_output', side_effect=lookup), \
                        mock.patch.object(w.subprocess, 'run', side_effect=native):
                    self.assertEqual(w.run(root, out, head), int(fail))
                receipt = json.loads((out / 'receipt.json').read_text())
                self.assertFalse(receipt['qualified'])
                self.assertTrue((out / 'final-inventory.json').is_file())
                self.assertTrue((out / 'SHA256SUMS').is_file())
                if fail:
                    self.assertEqual(receipt['commands'][-1]['exit'], 7)
                    self.assertIn('observer_cleanup_error', receipt)
                    self.assertEqual((out / 'round-1-production.log').read_text(), 'original production failure')
                    self.assertFalse(any('check-golden' in x for x in events))
                    self.assertNotIn('producer_fixed_point', receipt)
                else:
                    self.assertEqual(receipt['producer_fixed_point']['rounds'], 2)
                    self.assertTrue(receipt['producer_fixed_point']['read_only_checks_passed'])
                    self.assertFalse((root / w.OVERLAY).exists())
                    self.assertEqual([r['inventory_equal'] for r in receipt['rounds']], [False, True])


if __name__ == '__main__':
    unittest.main()
