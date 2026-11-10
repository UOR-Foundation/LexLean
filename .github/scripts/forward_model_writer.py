#!/usr/bin/env python3
"""Bounded official model projection, never release or corpus qualification."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tarfile

CHECKPOINT = '2fd42657901de7f984a719a7d785917dd0854974'
CHECKPOINT_TREE = '59dc9586e8fcd53ceeb1525e66033bad0eb01f1d'
INFRASTRUCTURE = frozenset(('.github/scripts/forward_model_writer.py',
                            '.github/scripts/test_forward_model_writer.py',
                            '.github/workflows/forward-model-writer.yml'))
PRODUCTS = frozenset(('SPEC.md', 'CONFORMANCE.md', 'ERRORS.md',
                      'model/ids.toml', 'model/errors.toml',
                      'model/ledger.toml', 'model/authorities.toml'))


def git(root, *args):
    return subprocess.check_output(['git', '-C', str(root), *args])


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def inventory(root):
    paths = git(root, 'ls-files', '-z').decode().split('\0')
    return {p: {'sha256': digest(root / p), 'mode': (root / p).stat().st_mode & 0o777}
            if (root / p).is_file() and not (root / p).is_symlink()
            else {'symlink': os.readlink(root / p)} if (root / p).is_symlink()
            else {'missing': True} for p in paths if p}


def changes(before, after):
    return sorted(p for p in before.keys() | after.keys() if before.get(p) != after.get(p))


def check_scope(before, after):
    changed = changes(before, after)
    unexpected = set(changed) - PRODUCTS
    if unexpected:
        raise RuntimeError('writer changed protected inputs: ' + ', '.join(sorted(unexpected)))
    return changed


def check_untracked(root, out, stage):
    paths = [p for p in git(root, 'ls-files', '--others', '-z').decode().split('\0') if p]
    if paths:
        save(out / (stage + '-unexpected.json'), paths)
        with tarfile.open(out / (stage + '-unexpected.tar'), 'w') as archive:
            for path in paths:
                archive.add(root / path, arcname=path, recursive=False)
        raise RuntimeError('writer created unexpected untracked files at ' + stage)


def save(path, value):
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + '\n')


def execute(root, out, receipt, name, command, env):
    with (out / (name + '.log')).open('wb') as log:
        result = subprocess.run(command, cwd=root, env=env, stdout=log, stderr=subprocess.STDOUT)
    receipt['commands'].append({'name': name, 'argv': command, 'exit': result.returncode})
    save(out / 'receipt.json', receipt)
    if result.returncode:
        raise subprocess.CalledProcessError(result.returncode, command)


def run(root, out, expected_head):
    root, out = root.resolve(), out.resolve()
    if out == root or root in out.parents:
        raise RuntimeError('evidence directory must be outside checkout')
    out.mkdir(parents=True, exist_ok=True)
    if any(out.iterdir()):
        raise RuntimeError('evidence directory must be empty')
    receipt = {'qualified': False, 'phase': 'model-projection-only', 'commands': [],
               'run_id': os.environ.get('GITHUB_RUN_ID'),
               'run_attempt': os.environ.get('GITHUB_RUN_ATTEMPT'),
               'repository': os.environ.get('GITHUB_REPOSITORY'),
               'expected_head': expected_head, 'checkpoint': CHECKPOINT}
    before = None
    status = 1
    try:
        head = git(root, 'rev-parse', 'HEAD').decode().strip()
        receipt.update(head=head, tree=git(root, 'rev-parse', 'HEAD^{tree}').decode().strip())
        if head != expected_head:
            raise RuntimeError('checkout differs from admitted head')
        git(root, 'merge-base', '--is-ancestor', CHECKPOINT, 'HEAD')
        if git(root, 'rev-parse', CHECKPOINT + '^{tree}').decode().strip() != CHECKPOINT_TREE:
            raise RuntimeError('checkpoint tree differs from admitted source')
        overlay = set(git(root, 'diff', '--name-only', '-z', CHECKPOINT, 'HEAD').decode().split('\0')) - {''}
        if overlay - INFRASTRUCTURE:
            raise RuntimeError('source changed after checkpoint; explicit readmission required')
        if git(root, 'status', '--porcelain', '--untracked-files=all'):
            raise RuntimeError('checkout must be clean')
        check_untracked(root, out, 'initial')
        before = inventory(root)
        save(out / 'before.json', before)
        env = dict(os.environ)
        for key in tuple(env):
            if key.startswith(('LEXLEAN_', 'CARGO_', 'RUST')):
                del env[key]
        env.update(CARGO_TARGET_DIR=str(out / 'build'), CARGO_TERM_COLOR='never')
        import tomllib
        pin = tomllib.loads((root / 'rust-toolchain.toml').read_text())['toolchain']['channel']
        if pin != '1.97.1':
            raise RuntimeError('Rust pin changed; readmit this driver')
        execute(root, out, receipt, 'rustc-version', ['rustc', '--version', '--verbose'], env)
        if not (out / 'rustc-version.log').read_text().startswith('rustc ' + pin + ' '):
            raise RuntimeError('actual rustc differs from pin')
        execute(root, out, receipt, 'cargo-version', ['cargo', '--version', '--verbose'], env)
        if not (out / 'cargo-version.log').read_text().startswith('cargo ' + pin + ' '):
            raise RuntimeError('actual cargo differs from pin')
        execute(root, out, receipt, 'build-writer', ['cargo', 'build', '--locked', '-p', 'xtask'], env)
        check_untracked(root, out, 'build')
        binary = out / 'build/debug/xtask'
        receipt['writer_sha256'] = digest(binary)
        if before != inventory(root):
            raise RuntimeError('building writer changed tracked inputs')
        execute(root, out, receipt, 'model-write-1', [str(binary), 'validate-model', '--write'], env)
        check_untracked(root, out, 'first')
        first = inventory(root)
        save(out / 'after-first.json', first)
        check_scope(before, first)
        execute(root, out, receipt, 'model-write-2', [str(binary), 'validate-model', '--write'], env)
        check_untracked(root, out, 'second')
        second = inventory(root)
        save(out / 'after-second.json', second)
        check_scope(before, second)
        if first != second:
            raise RuntimeError('model projection did not reach a two-pass fixed point')
        if digest(binary) != receipt['writer_sha256']:
            raise RuntimeError('writer executable changed')
        receipt['model_projection_fixed_point'] = True
        status = 0
    except subprocess.CalledProcessError as error:
        status = error.returncode if error.returncode > 0 else 1
        receipt['error'] = str(error)
    except Exception as error:
        receipt['error'] = str(error)
    finally:
        receipt['exit'] = status
        if before is not None:
            try:
                check_untracked(root, out, 'final')
            except RuntimeError as error:
                receipt['unexpected_output_error'] = str(error)
            after = inventory(root)
            save(out / 'after.json', after)
            receipt['changed_paths'] = changes(before, after)
            receipt['unexpected_paths'] = sorted(set(receipt['changed_paths']) - PRODUCTS)
            (out / 'products.patch').write_bytes(git(root, 'diff', '--binary', '--full-index', 'HEAD'))
            (out / 'status.txt').write_bytes(git(root, 'status', '--porcelain', '--untracked-files=all'))
        save(out / 'receipt.json', receipt)
        files = sorted(p for p in out.iterdir() if p.is_file() and p.name != 'SHA256SUMS')
        (out / 'SHA256SUMS').write_text(''.join(f'{digest(p)}  {p.name}\n' for p in files))
    return status


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=Path.cwd())
    parser.add_argument('--out', required=True, type=Path)
    parser.add_argument('--expected-head', required=True)
    args = parser.parse_args()
    sys.exit(run(args.root, args.out, args.expected_head))
