#!/usr/bin/env python3
"""Temporary bounded production/golden/calculus cycle; no fixture or example verification writer."""
import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import tomllib

sys.dont_write_bytecode = True

from forward_model_writer import git, digest, save, changes

BASE = '28fcce2e46538a05115dfa089aa4d764ecea4458'
BASE_TREE = 'e12baca5f6253a5f63341541f83655da7eed79ce'
DATA = '.github/scripts/forward_cycle'
SEED_DATA = '.github/scripts/forward_product'
OVERLAY = 'xtask/examples/observe_forward_identity.rs'
INFRA = {'.github/scripts/forward_cycle_writer.py',
         '.github/scripts/test_forward_cycle_writer.py',
         '.github/workflows/forward-cycle-writer.yml',
         DATA + '/README.md'}
from forward_product_writer import (CALCULUS_FILES, CALCULUS_PREFIXES,
                                    inventory, require_scope, cleanup_observer)


def product_scopes(root):
    # Match xtask::example_dirs and tooling::fixtures, including directories
    # without existing goldens. Source admission fixes the discovery inputs.
    examples = sorted(p for p in (root / 'examples').iterdir() if p.is_dir())
    tooling = sorted(p for p in (root / 'tests/tooling').iterdir() if p.is_dir())
    if not examples or not tooling:
        raise RuntimeError('empty official golden discovery')
    native = [root / p for p in ('compiler', 'stdlib', 'std-models', 'std-reasoning', 'kappa', 'bootstrap')]
    dirs = examples + native
    if any(not (p / 'lexlean.toml').is_file() for p in dirs):
        raise RuntimeError('missing golden project manifest')
    golden = tuple(str(p.relative_to(root)) + '/expected/build/' for p in dirs + tooling)
    # production_export::EXPORTED is fixed by the admitted Rust source.
    exports = tuple('tests/production-export/' + p + '/packages/' for p in
                    ('production', 'production-coverage', 'stdlib', 'bootstrap', 'helpdesk'))
    return {
        'check-calculus': lambda p: p in CALCULUS_FILES or p.startswith(CALCULUS_PREFIXES),
        'check-production-export': lambda p: p.startswith(exports),
        'check-golden': lambda p: p == 'tests/rust-runtime-golden/transcript.txt' or p.startswith(golden),
    }


def producer_round(writer, refresh, number):
    prefix = f'round-{number}'
    writer(prefix + '-calculus-before', 'check-calculus')
    refresh(prefix + '-before-export')
    writer(prefix + '-production', 'check-production-export')
    # The generated stdlib SHA-256 Rust package is compiled by build.rs,
    # separately from embedded::FILES: rebuild before using its new bytes.
    refresh(prefix + '-after-export')
    writer(prefix + '-golden', 'check-golden')
    refresh(prefix + '-after-golden-build')
    writer(prefix + '-calculus-after', 'check-calculus')
    refresh(prefix + '-after-golden')


def bounded_cycle(perform, measure, record, limit=3):
    previous = measure()
    seen = [previous]
    for number in range(1, limit + 1):
        perform(number)
        current = measure()
        same = current == previous
        record(number, changes(previous, current), same)
        if same:
            return number
        if current in seen:
            raise RuntimeError('producer cycle oscillated; original evidence retained')
        seen.append(current)
        previous = current
    raise RuntimeError(f'producer cycle did not converge within {limit} rounds')


def run(root, out, expected_head):
    root, out = root.resolve(), out.resolve()
    if out == root or root in out.parents:
        raise RuntimeError('evidence must be outside checkout')
    out.mkdir(parents=True, exist_ok=True)
    if any(out.iterdir()):
        raise RuntimeError('evidence must be empty')
    receipt = {'qualified': False, 'phase': 'production-golden-calculus-cycle', 'commands': [], 'rounds': [],
               'qualified_fixed_point': False, 'max_rounds': 3}
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
        if git(root, 'rev-parse', BASE + '^{tree}').decode().strip() != BASE_TREE:
            raise RuntimeError('source base tree changed')
        paths = set(git(root, 'diff', '--name-only', '-z', BASE, 'HEAD').decode().split('\0')) - {''}
        if paths - INFRA:
            raise RuntimeError('unadmitted source since base')
        locks_admission = json.loads((root / SEED_DATA / 'admission.json').read_text())
        scopes = product_scopes(root)
        if tomllib.loads((root / 'rust-toolchain.toml').read_text())['toolchain']['channel'] != '1.97.1':
            raise RuntimeError('Rust pin changed')
        if (root / 'lean-toolchain').read_text().strip() != 'leanprover/lean4:v4.32.1':
            raise RuntimeError('Lean pin changed')
        before = snapshot('initial')
        if (root / OVERLAY).exists():
            raise RuntimeError('observer overlay already exists')
        (root / OVERLAY).parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(root / SEED_DATA / 'observe_forward_identity.rs', root / OVERLAY)
        installed = True
        receipt['observer_source_sha256'] = digest(root / OVERLAY)
        for name, cmd, expected in [('rust-version', ['rustc', '--version'], 'rustc 1.97.1 '),
                                     ('cargo-version', ['cargo', '--version'], 'cargo 1.97.1 '),
                                     ('lean-version', ['lean', '--version'], 'Lean (version 4.32.1,')]:
            execute(name, cmd)
            if not (out / (name + '.log')).read_text().startswith(expected):
                raise RuntimeError('actual tool differs from pin: ' + name)
        build('build-initial')
        initial_identity = observe('identity-initial')
        latest_identity = initial_identity
        previous_round_identity = initial_identity
        if not initial_identity['renderer_matches']:
            raise RuntimeError('initial renderer pin is stale; no automatic reconciliation in this phase')

        def refresh(label):
            nonlocal latest_identity
            build(label + '-build')
            observed = observe(label + '-identity')
            if (not observed['renderer_matches'] or
                    observed['renderer_observed'] != initial_identity['renderer_observed'] or
                    observed['errors_sha256'] != initial_identity['errors_sha256']):
                raise RuntimeError('renderer/error identity changed outside admitted scope')
            latest_identity = observed
            locks(label + '-locks', locks_admission)

        def writer(label, command):
            inputs = inventory(root)
            execute(label, [str(out / 'build/debug/xtask'), command, '--write'])
            require_scope(inputs, inventory(root), scopes[command])
            snapshot(label + '-products', products=True)

        def record_round(number, changed, same):
            nonlocal previous_round_identity
            identity = latest_identity
            identity_same = identity == previous_round_identity
            previous_round_identity = identity
            receipt['rounds'].append({'round': number, 'changed': changed, 'inventory_equal': same, 'identity_equal': identity_same,
                                      'compiler_semantics_1_2': identity['compiler_semantics_1_2']})
            save(out / 'receipt.json', receipt)
            snapshot(f'round-{number}-end', products=True)
            if same and not identity_same:
                raise RuntimeError('unchanged source has inconsistent owning identity observation')

        count = bounded_cycle(lambda n: producer_round(writer, refresh, n),
                              lambda: inventory(root), record_round)
        converged = inventory(root)
        for command in ('check-calculus', 'check-production-export', 'check-golden'):
            execute('final-' + command, [str(out / 'build/debug/xtask'), command])
            require_scope(converged, inventory(root), lambda p: False)
        if observe('identity-final') != latest_identity:
            raise RuntimeError('final owning identity observation changed')
        require_scope(converged, inventory(root), lambda p: False)
        receipt['producer_fixed_point'] = {'rounds': count, 'scope': 'production-export, golden, calculus and ordinary locks',
                                            'whole_inventory_equal': True, 'read_only_checks_passed': True}
        receipt['stop'] = 'Independent product review required. No fixture writer, example verification writer, bootstrap counters or final qualification.'
        status = 0

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
