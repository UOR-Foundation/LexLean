"""Behavioral plants for the transport driver, not official Rust writer evidence."""
import importlib.util
import json
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location('writer', Path(__file__).with_name('forward_model_writer.py'))
writer = importlib.util.module_from_spec(spec)
spec.loader.exec_module(writer)


class CustodyTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name) / 'repo'
        self.root.mkdir()
        self.out = Path(self.temp.name) / 'evidence'
        for command in [('init', '-q'), ('config', 'user.name', 'Fixture'),
                        ('config', 'user.email', 'fixture@example.invalid')]:
            writer.git(self.root, *command)
        (self.root / 'rust-toolchain.toml').write_text('[toolchain]\nchannel = "1.97.1"\n')
        (self.root / 'SPEC.md').write_text('old\n')
        (self.root / 'malformed-control.json').write_text('{intentionally malformed')
        writer.git(self.root, 'add', '.')
        writer.git(self.root, 'commit', '-qm', 'fixture')
        self.head = writer.git(self.root, 'rev-parse', 'HEAD').decode().strip()
        self.checkpoint = patch.object(writer, 'CHECKPOINT', self.head)
        self.checkpoint.start()
        self.addCleanup(self.checkpoint.stop)
        self.tree = patch.object(writer, 'CHECKPOINT_TREE',
                                 writer.git(self.root, 'rev-parse', 'HEAD^{tree}').decode().strip())
        self.tree.start()
        self.addCleanup(self.tree.stop)
        self.plant = None

    def fake_execute(self, root, out, receipt, name, command, env):
        self.assertFalse(any(k.startswith('LEXLEAN_') for k in env))
        (out / (name + '.log')).write_text('rustc 1.97.1 (fixture)\n' if name == 'rustc-version' else 'cargo 1.97.1 (fixture)\n' if name == 'cargo-version' else name + '\n')
        receipt['commands'].append({'name': name, 'argv': command, 'exit': 0})
        if name == 'build-writer':
            binary = out / 'build/debug/xtask'
            binary.parent.mkdir(parents=True)
            binary.write_bytes(b'fixture binary, not Rust evidence')
        if name.startswith('model-write'):
            (root / 'SPEC.md').write_text('new\n')
        if self.plant:
            self.plant(root, out, receipt, name)

    def run_writer(self, expected=None):
        with patch.object(writer, 'execute', self.fake_execute):
            status = writer.run(self.root, self.out, expected or self.head)
        receipt = json.loads((self.out / 'receipt.json').read_text())
        return status, receipt

    def test_success_is_unqualified_and_fixed_point_with_patch_and_hashes(self):
        status, receipt = self.run_writer()
        self.assertEqual(status, 0)
        self.assertFalse(receipt['qualified'])
        self.assertTrue(receipt['model_projection_fixed_point'])
        self.assertEqual(receipt['changed_paths'], ['SPEC.md'])
        self.assertIn('+new', (self.out / 'products.patch').read_text())
        self.assertEqual((self.root / 'malformed-control.json').read_text(), '{intentionally malformed')
        for line in (self.out / 'SHA256SUMS').read_text().splitlines():
            sha, name = line.split('  ')
            self.assertEqual(sha, writer.digest(self.out / name))

    def test_unadmitted_committed_source_refused(self):
        (self.root / 'SPEC.md').write_text('unadmitted source')
        writer.git(self.root, 'add', '.')
        writer.git(self.root, 'commit', '-qm', 'unadmitted source')
        head = writer.git(self.root, 'rev-parse', 'HEAD').decode().strip()
        status, receipt = self.run_writer(head)
        self.assertNotEqual(status, 0)
        self.assertEqual(receipt['commands'], [])
        self.assertIn('readmission', receipt['error'])

    def test_build_changes_product_refused(self):
        def plant(root, out, receipt, name):
            if name == 'build-writer':
                (root / 'SPEC.md').write_text('changed during build')
        self.plant = plant
        status, receipt = self.run_writer()
        self.assertNotEqual(status, 0)
        self.assertIn('building writer changed', receipt['error'])
        self.assertNotIn('model-write-1', [c['name'] for c in receipt['commands']])

    def test_wrong_head_refused_before_commands(self):
        status, receipt = self.run_writer('0' * 40)
        self.assertNotEqual(status, 0)
        self.assertEqual(receipt['commands'], [])

    def test_dirty_input_refused(self):
        (self.root / 'SPEC.md').write_text('dirty')
        status, receipt = self.run_writer()
        self.assertNotEqual(status, 0)
        self.assertEqual(receipt['commands'], [])

    def test_protected_malformed_control_change_fails_and_retains_patch(self):
        def plant(root, out, receipt, name):
            if name == 'model-write-1':
                (root / 'malformed-control.json').write_text('{}')
        self.plant = plant
        status, receipt = self.run_writer()
        self.assertNotEqual(status, 0)
        self.assertEqual(receipt['unexpected_paths'], ['malformed-control.json'])
        self.assertNotIn('model-write-2', [c['name'] for c in receipt['commands']])
        self.assertIn('malformed-control.json', (self.out / 'products.patch').read_text())

    def test_non_fixed_point_fails(self):
        def plant(root, out, receipt, name):
            if name == 'model-write-2':
                (root / 'SPEC.md').write_text('different second pass')
        self.plant = plant
        status, receipt = self.run_writer()
        self.assertNotEqual(status, 0)
        self.assertIn('fixed point', receipt['error'])

    def test_first_failure_exit_and_partial_products_preserved(self):
        def plant(root, out, receipt, name):
            if name == 'model-write-1':
                receipt['commands'][-1]['exit'] = 37
                raise subprocess.CalledProcessError(37, ['fixture-writer'])
        self.plant = plant
        status, receipt = self.run_writer()
        self.assertEqual(status, 37)
        self.assertEqual(receipt['exit'], 37)
        self.assertEqual(receipt['changed_paths'], ['SPEC.md'])
        self.assertTrue((self.out / 'model-write-1.log').is_file())
        self.assertFalse((self.out / 'model-write-2.log').exists())

    def test_untracked_output_refused(self):
        def plant(root, out, receipt, name):
            if name == 'model-write-1':
                (root / 'unexpected.json').write_text('{}')
        self.plant = plant
        status, receipt = self.run_writer()
        self.assertNotEqual(status, 0)
        self.assertIn('untracked', receipt['error'])
        self.assertTrue((self.out / 'first-unexpected.tar').is_file())
        self.assertFalse((self.out / 'model-write-2.log').exists())

    def test_ignored_output_is_captured_and_refused(self):
        info = self.root / '.git/info/exclude'
        info.write_text('ignored-output\n')
        def plant(root, out, receipt, name):
            if name == 'model-write-1':
                (root / 'ignored-output').write_text('unexpected bytes')
        self.plant = plant
        status, receipt = self.run_writer()
        self.assertNotEqual(status, 0)
        import tarfile
        with tarfile.open(self.out / 'first-unexpected.tar') as archive:
            self.assertEqual(archive.extractfile('ignored-output').read(), b'unexpected bytes')

    def test_output_inside_checkout_refused(self):
        with self.assertRaisesRegex(RuntimeError, 'outside checkout'):
            writer.run(self.root, self.root / 'evidence', self.head)

    def test_executable_drift_refused(self):
        def plant(root, out, receipt, name):
            if name == 'model-write-2':
                (out / 'build/debug/xtask').write_bytes(b'changed')
        self.plant = plant
        status, receipt = self.run_writer()
        self.assertNotEqual(status, 0)
        self.assertIn('executable changed', receipt['error'])


if __name__ == '__main__':
    unittest.main()
