import json
from pathlib import Path
import tempfile
import subprocess
import unittest
from unittest import mock

import forward_product_writer as w


class Custody(unittest.TestCase):
    def test_renderer_only_scalar_changes(self):
        with tempfile.TemporaryDirectory() as d:
            p = Path(d) / 's.toml'
            old, new = 'a' * 64, 'b' * 64
            source = f'# keep\nrust_renderer = "{old}"\nother = "{old}"\n'
            p.write_text(source)
            w.reconcile_renderer(p, {'renderer_observed': new, 'renderer_recorded': old})
            self.assertEqual(p.read_text(), source.replace(f'rust_renderer = "{old}"', f'rust_renderer = "{new}"'))

    def test_renderer_stale_observation_stops_without_mutation(self):
        with tempfile.TemporaryDirectory() as d:
            p = Path(d) / 's.toml'
            source = f'rust_renderer = "{"a" * 64}"\n'
            p.write_text(source)
            with self.assertRaises(RuntimeError):
                w.reconcile_renderer(p, {'renderer_observed': 'b'*64, 'renderer_recorded': 'c'*64})
            self.assertEqual(p.read_text(), source)

    def test_renderer_duplicate_or_bad_value_stops(self):
        with tempfile.TemporaryDirectory() as d:
            p = Path(d) / 's.toml'
            p.write_text(f'rust_renderer = "{"a" * 64}"\n' * 2)
            with self.assertRaises(RuntimeError):
                w.reconcile_renderer(p, {'renderer_observed': 'b'*64, 'renderer_recorded': 'a'*64})
            with self.assertRaises(RuntimeError):
                w.reconcile_renderer(p, {'renderer_observed': 'bad', 'renderer_recorded': 'a'*64})

    def test_fixture_changes_block_calculus(self):
        allowed = lambda p: p in w.CALCULUS_FILES or p.startswith(w.CALCULUS_PREFIXES)
        for p in ['tests/negative/stale-lock/lexlean.lock', 'model/errors.toml',
                  'compiler/src/TargetSyntax.lex.tex', 'tests/fixtures/cf-10/overlay/2/lexlean.toml']:
            with self.assertRaises(RuntimeError):
                w.require_scope({p: 'old'}, {p: 'new'}, allowed)
        w.require_scope({}, {'compiler/src/GnafFixtures.lex.tex': 'new'}, allowed)

    def test_deletions_and_new_files_are_changes(self):
        with self.assertRaises(RuntimeError):
            w.require_scope({'old': 1}, {'new': 2}, lambda p: p == 'new')

    def test_lock_scope_does_not_grant_neighbor_access(self):
        with self.assertRaises(RuntimeError):
            w.require_scope({'p/lexlean.toml': 1}, {'p/lexlean.toml': 2}, lambda p: p == 'p/lexlean.lock')

    def test_admission_has_nested_example_not_fixture_roots(self):
        data = json.loads((Path(__file__).parent / 'forward_product/admission.json').read_text())
        roots = {r['path'] for r in data['ordinary']}
        self.assertIn('examples/nested-induction/lexlean.toml', roots)
        self.assertFalse(any(p.startswith(('tests/negative/', 'tests/fixtures/')) for p in roots))
        self.assertEqual(len(roots), len(data['ordinary']))
        for row in data['ordinary']:
            self.assertEqual(row['lock'], str(Path(row['path']).with_name('lexlean.lock')))

    def test_wrong_head_fails_before_any_native_command(self):
        with tempfile.TemporaryDirectory() as d:
            root, out = Path(d) / 'repo', Path(d) / 'evidence'
            root.mkdir()
            subprocess.run(['git', 'init', '-q', str(root)], check=True)
            subprocess.run(['git', '-C', str(root), '-c', 'user.name=Test', '-c',
                            'user.email=test@example.invalid', 'commit', '-q', '--allow-empty', '-m', 'test'], check=True)
            self.assertEqual(w.run(root, out, '0'*40), 1)
            receipt = json.loads((out / 'receipt.json').read_text())
            self.assertEqual(receipt['commands'], [])
            self.assertIn('exact admitted clean head', receipt['error'])
            self.assertFalse((root / w.OVERLAY).exists())

    def test_existing_evidence_is_never_overwritten(self):
        with tempfile.TemporaryDirectory() as d:
            root, out = Path(d) / 'repo', Path(d) / 'evidence'
            root.mkdir()
            out.mkdir()
            (out / 'original.log').write_text('original failure')
            with self.assertRaises(RuntimeError):
                w.run(root, out, '0'*40)
            self.assertEqual((out / 'original.log').read_text(), 'original failure')

    def test_evidence_inside_checkout_is_refused(self):
        with tempfile.TemporaryDirectory() as d:
            with self.assertRaises(RuntimeError):
                w.run(Path(d), Path(d) / 'evidence', '0'*40)

    def test_deleted_overlay_cleanup_records_failure_without_raising(self):
        with tempfile.TemporaryDirectory() as d:
            receipt = {'observer_source_sha256': 'a'*64}
            self.assertFalse(w.cleanup_observer(Path(d), receipt))
            self.assertIn('missing', receipt['observer_cleanup_error'])

    def test_directory_overlay_cleanup_retains_unexpected_evidence(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            overlay = root / w.OVERLAY
            overlay.mkdir(parents=True)
            (overlay / 'evidence').write_text('unexpected output')
            receipt = {'observer_source_sha256': 'a'*64}
            self.assertFalse(w.cleanup_observer(root, receipt))
            self.assertEqual((overlay / 'evidence').read_text(), 'unexpected output')

    def test_changed_and_symlink_overlay_are_retained_not_followed(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            overlay = root / w.OVERLAY
            overlay.parent.mkdir(parents=True)
            overlay.write_text('original')
            expected = w.digest(overlay)
            overlay.write_text('changed')
            receipt = {'observer_source_sha256': expected}
            self.assertFalse(w.cleanup_observer(root, receipt))
            self.assertEqual(overlay.read_text(), 'changed')
            overlay.unlink()
            overlay.symlink_to(root / 'absent-target')
            self.assertFalse(w.cleanup_observer(root, receipt))
            self.assertTrue(overlay.is_symlink())

    def test_intact_overlay_is_removed(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            overlay = root / w.OVERLAY
            overlay.parent.mkdir(parents=True)
            overlay.write_text('original')
            receipt = {'observer_source_sha256': w.digest(overlay)}
            self.assertTrue(w.cleanup_observer(root, receipt))
            self.assertFalse(overlay.exists())

    def test_overlay_loss_still_finalizes_receipt_inventory_and_hashes(self):
        for replacement in ('missing', 'directory'):
            with self.subTest(replacement=replacement), tempfile.TemporaryDirectory() as d:
                root, out = Path(d) / 'repo', Path(d) / 'evidence'
                data = root / w.DATA
                data.mkdir(parents=True)
                (data / 'admission.json').write_text('{"model": {}, "ordinary": []}')
                (data / 'observe_forward_identity.rs').write_text('observer bytes')
                (root / 'rust-toolchain.toml').write_text('[toolchain]\nchannel = "1.97.1"\n')
                (root / 'lean-toolchain').write_text('leanprover/lean4:v4.32.1\n')
                subprocess.run(['git', 'init', '-q', str(root)], check=True)
                subprocess.run(['git', '-C', str(root), 'add', '.'], check=True)
                subprocess.run(['git', '-C', str(root), '-c', 'user.name=Test', '-c',
                                'user.email=test@example.invalid', 'commit', '-q', '-m', 'test'], check=True)
                head = w.git(root, 'rev-parse', 'HEAD').decode().strip()
                original = subprocess.check_output

                def fault(command, **kwargs):
                    if command[0] == 'rustup':
                        overlay = root / w.OVERLAY
                        overlay.unlink()
                        if replacement == 'directory':
                            overlay.mkdir()
                            (overlay / 'unexpected').write_text('retain me')
                        raise RuntimeError('planted command failure after overlay loss')
                    return original(command, **kwargs)

                with mock.patch.object(w, 'BASE', head), mock.patch.object(w.subprocess, 'check_output', side_effect=fault):
                    self.assertEqual(w.run(root, out, head), 1)
                receipt = json.loads((out / 'receipt.json').read_text())
                self.assertIn('planted command failure', receipt['error'])
                self.assertIn('observer_cleanup_error', receipt)
                for name in ('final-inventory.json', 'final.patch', 'final-changed.tar', 'SHA256SUMS'):
                    self.assertTrue((out / name).is_file(), name)
                self.assertIn('receipt.json', (out / 'SHA256SUMS').read_text())


if __name__ == '__main__':
    unittest.main()
