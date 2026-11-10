#!/usr/bin/env python3
"""Temporary initial product wave. Stops before production/golden/fixture writers."""
import argparse
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tarfile
import tomllib

sys.dont_write_bytecode = True

from forward_model_writer import git, digest, save, changes

BASE = 'd1cc3c6d3dd11e30a1e4420fa81e7121f9640332'
DATA = '.github/scripts/forward_product'
OVERLAY = 'xtask/examples/observe_forward_identity.rs'
INFRA = {'.github/scripts/forward_product_writer.py',
         '.github/scripts/test_forward_product_writer.py',
         '.github/workflows/forward-product-writer.yml',
         DATA + '/admission.json', DATA + '/observe_forward_identity.rs',
         DATA + '/README.md'}
CALCULUS_PREFIXES = ('compiler/certificates/', 'compiler/fixtures/', 'compiler/gnaf/',
                     'compiler/rust/', 'language/lcnf-1.2/tooling/',
                     'language/preservation-1.2/modules/')
CALCULUS_FILES = {'tests/lcnf/registry.json', 'compiler/gnaf.manifest.json',
                  'compiler/src/TargetFixtures.lex.tex', 'compiler/src/ReasoningFixtures.lex.tex',
                  'compiler/src/GnafFixtures.lex.tex', 'compiler/src/GnafCertificates.lex.tex',
                  'schemas/gnaf-request.schema.json', 'schemas/gnaf-fixture.schema.json',
                  'schemas/gnaf-certificate.schema.json', 'schemas/gnaf-certificate-fixture.schema.json'}


def inventory(root):
    names = git(root, 'ls-files', '-z').decode().split('\0')
    names += git(root, 'ls-files', '--others', '--exclude-standard', '-z').decode().split('\0')
    result = {}
    for name in sorted(set(names) - {''}):
        p = root / name
        result[name] = ({'link': os.readlink(p)} if p.is_symlink() else
                        {'sha256': digest(p), 'mode': p.stat().st_mode & 0o777} if p.is_file()
                        else {'missing': True})
    return result


def require_scope(before, after, allowed):
    bad = [p for p in changes(before, after) if not allowed(p)]
    if bad:
        raise RuntimeError('protected input changed: ' + ', '.join(bad))


def reconcile_renderer(path, observation):
    old = path.read_bytes()
    value = observation['renderer_observed']
    if not re.fullmatch('[0-9a-f]{64}', value):
        raise RuntimeError('invalid renderer observation')
    pattern = rb'(?m)^(rust_renderer = ")[0-9a-f]{64}("\r?$)'
    if len(re.findall(pattern, old)) != 1:
        raise RuntimeError('renderer field is not uniquely replaceable')
    recorded = tomllib.loads(old.decode())['rust_renderer']
    if recorded != observation['renderer_recorded']:
        raise RuntimeError('observation is stale')
    path.write_bytes(re.sub(pattern, lambda m: m[1] + value.encode() + m[2], old))


def cleanup_observer(root, receipt):
    """Remove only our intact regular overlay; retain altered evidence for review."""
    overlay = root / OVERLAY
    try:
        if overlay.is_symlink() or not overlay.is_file():
            raise RuntimeError('observer overlay is missing or no longer a regular file')
        if digest(overlay) != receipt['observer_source_sha256']:
            raise RuntimeError('observer overlay bytes changed')
        overlay.unlink()
        return True
    except Exception as error:
        receipt['observer_cleanup_error'] = str(error)
        return False


def run(root, out, expected_head):
    root, out = root.resolve(), out.resolve()
    if out == root or root in out.parents:
        raise RuntimeError('evidence must be outside checkout')
    out.mkdir(parents=True, exist_ok=True)
    if any(out.iterdir()):
        raise RuntimeError('evidence must be empty')
    receipt = {'qualified': False, 'phase': 'initial-calculus-only', 'commands': []}
    before = None
    installed = False
    status = 1
    env = dict(os.environ)
    for key in tuple(env):
        if key.startswith(('LEXLEAN_', 'CARGO_', 'RUST')):
            del env[key]
    env.update(CARGO_TARGET_DIR=str(out / 'build'), CARGO_TERM_COLOR='never')

    def snapshot(name, products=False):
        current = inventory(root)
        save(out / (name + '-inventory.json'), current)
        if not products:
            return current
        (out / (name + '.patch')).write_bytes(git(root, 'diff', '--binary', '--full-index', 'HEAD'))
        with tarfile.open(out / (name + '-changed.tar'), 'w') as archive:
            for p in changes(before or {}, current):
                if (root / p).exists() or (root / p).is_symlink():
                    archive.add(root / p, arcname=p, recursive=False)
        return current

    def execute(name, command, allow_failure=False):
        inputs = snapshot(name + '-before')
        hashes = {}
        for tool in ('rustc', 'cargo', 'lean', 'lake', 'leanchecker'):
            # Resolve through the toolchain managers, not their shared proxy binary.
            manager = ['rustup', 'which', tool] if tool in ('rustc', 'cargo') else ['elan', 'which', tool]
            path = subprocess.check_output(manager, cwd=root, env=env, text=True).strip()
            hashes[tool] = {'path': path, 'sha256': digest(Path(path))}
        binary = Path(command[0])
        if binary.is_absolute():
            hashes['executable'] = {'path': str(binary), 'sha256': digest(binary)}
        with (out / (name + '.log')).open('wb') as log:
            result = subprocess.run(command, cwd=root, env=env, stdout=log, stderr=subprocess.STDOUT)
        after = snapshot(name + '-after')
        receipt['commands'].append({'name': name, 'argv': command, 'exit': result.returncode,
                                    'tools': hashes, 'changed': changes(inputs, after)})
        save(out / 'receipt.json', receipt)
        if result.returncode and not allow_failure:
            raise RuntimeError(f'{name} failed with exit {result.returncode}; inspect original log')
        return result.returncode

    def build(label):
        inputs = inventory(root)
        execute(label, ['cargo', 'build', '--locked', '-p', 'xtask', '-p', 'lexlean',
                        '--bins', '--example', 'observe_forward_identity'])
        require_scope(inputs, inventory(root), lambda p: False)

    def observe(label):
        inputs = inventory(root)
        execute(label, [str(out / 'build/debug/examples/observe_forward_identity'), str(root)])
        require_scope(inputs, inventory(root), lambda p: False)
        return json.loads((out / (label + '.log')).read_text())

    def locks(label, admission):
        for i, row in enumerate(admission['ordinary']):
            inputs = inventory(root)
            execute(f'{label}-{i:02}', [str(out / 'build/debug/lexlean'), '--project', row['path'], 'lock'])
            require_scope(inputs, inventory(root), lambda p: p == row['lock'])
        snapshot(label + '-products', products=True)

    try:
        head = git(root, 'rev-parse', 'HEAD').decode().strip()
        receipt.update(head=head, tree=git(root, 'rev-parse', 'HEAD^{tree}').decode().strip(),
                       run_id=os.environ.get('GITHUB_RUN_ID'), run_attempt=os.environ.get('GITHUB_RUN_ATTEMPT'))
        if head != expected_head or git(root, 'status', '--porcelain', '--untracked-files=all'):
            raise RuntimeError('exact admitted clean head required')
        git(root, 'merge-base', '--is-ancestor', BASE, 'HEAD')
        admission = json.loads((root / DATA / 'admission.json').read_text())
        paths = set(git(root, 'diff', '--name-only', '-z', BASE, 'HEAD').decode().split('\0')) - {''}
        if paths - INFRA - admission['model'].keys():
            raise RuntimeError('unadmitted source since base')
        for path, sha in admission['model'].items():
            if digest(root / path) != sha:
                raise RuntimeError('admitted official model product missing: ' + path)
        if tomllib.loads((root / 'rust-toolchain.toml').read_text())['toolchain']['channel'] != '1.97.1':
            raise RuntimeError('Rust pin changed')
        if (root / 'lean-toolchain').read_text().strip() != 'leanprover/lean4:v4.32.1':
            raise RuntimeError('Lean pin changed')
        before = snapshot('initial')
        if (root / OVERLAY).exists():
            raise RuntimeError('observer overlay already exists')
        (root / OVERLAY).parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(root / DATA / 'observe_forward_identity.rs', root / OVERLAY)
        installed = True
        receipt['observer_source_sha256'] = digest(root / OVERLAY)
        for name, cmd, expected in [('rust-version', ['rustc', '--version'], 'rustc 1.97.1 '),
                                     ('cargo-version', ['cargo', '--version'], 'cargo 1.97.1 '),
                                     ('lean-version', ['lean', '--version'], 'Lean (version 4.32.1,')]:
            execute(name, cmd)
            if not (out / (name + '.log')).read_text().startswith(expected):
                raise RuntimeError('actual tool differs from pin: ' + name)
        build('build-initial')
        observed = observe('identity-initial')
        reconcile_renderer(root / 'language/semantics-1.2.toml', observed)
        snapshot('renderer-pin', products=True)
        build('build-renderer')
        observed = observe('identity-renderer')
        if not observed['renderer_matches']:
            raise RuntimeError('renderer reconciliation did not match owning API')
        locks('locks-initial', admission)
        inputs = inventory(root)
        execute('calculus-write', [str(out / 'build/debug/xtask'), 'check-calculus', '--write'])
        require_scope(inputs, inventory(root), lambda p: p in CALCULUS_FILES or p.startswith(CALCULUS_PREFIXES))
        snapshot('calculus-products', products=True)
        build('build-calculus')
        final = observe('identity-calculus')
        if final['errors_sha256'] != observed['errors_sha256'] or not final['renderer_matches']:
            raise RuntimeError('error model or renderer changed unexpectedly')
        locks('locks-calculus', admission)
        inputs = inventory(root)
        code = execute('calculus-check', [str(out / 'build/debug/xtask'), 'check-calculus'], allow_failure=True)
        require_scope(inputs, inventory(root), lambda p: False)
        receipt['calculus_check_exit'] = code
        receipt['stop'] = 'Review initial products and rebuilt identity before production/golden cycle. No fixture inputs changed.'
        receipt['fixed_point_established'] = False
        status = 0 if code == 0 else 1
    except Exception as error:
        receipt['error'] = str(error)
    finally:
        if installed and not cleanup_observer(root, receipt):
            status = 1
        if before is not None:
            try:
                after = snapshot('final', products=True)
                receipt['changed'] = changes(before, after)
            except Exception as error:
                receipt['final_snapshot_error'] = str(error)
                status = 1
        receipt['exit'] = status
        save(out / 'receipt.json', receipt)
        (out / 'SHA256SUMS').write_text(''.join(f'{digest(p)}  {p.name}\n' for p in sorted(out.iterdir())
                                               if p.is_file() and p.name != 'SHA256SUMS'))
    return status


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=Path.cwd())
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--expected-head', required=True)
    args = parser.parse_args()
    sys.exit(run(args.root, args.out, args.expected_head))
