#!/usr/bin/env python3
"""Build the dev-only kinetic server component and compare it inside Horizon's sandbox."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import uuid
from compare_reference import load_run


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('references', type=Path, nargs='+')
    modes = parser.add_mutually_exclusive_group()
    modes.add_argument('--geometry', action='store_true', help='Compare the bounded horizontal geometry set in native and sandbox probes')
    modes.add_argument('--vertical-equal', action='store_true', help='Compare six Y/UP/equal-cog static snapshots in native and sandbox probes')
    args = parser.parse_args()
    if sys.platform != 'linux':
        parser.error('Run in the pinned Ubuntu development environment')
    repo = Path(__file__).resolve().parents[1]
    workspace = repo.parent
    manifest = repo / 'tools/wasm-probe/Cargo.toml'
    lock = manifest.with_name('Cargo.lock')
    if not lock.is_file():
        parser.error('Resolve and review the separate dev-tool Cargo.lock first')
    reference_hashes = []
    required_set = 'p02-horizontal-1' if args.geometry else 's0-edges-1'
    for reference in args.references:
        loaded = load_run(reference)
        if loaded['environment']['fixture_set'] != required_set:
            parser.error(f'This mode requires fixture set {required_set}')
        files = {'exit-code.txt', 'environment.json', 'console.log', 'resolved-artifacts.json',
                 'config-sha256.json', *loaded['configs']}
        if args.geometry:
            files.add('resources/data/create_horizon_horizontal/structure/empty.nbt')
        reference_hashes.append({name: hashlib.sha256((reference / name).read_bytes()).hexdigest()
                                 for name in sorted(files)})
    run = workspace / '.local/p02/wasm-runs' / str(uuid.uuid4())
    run.mkdir(parents=True)
    scratch = run / 'scratch'
    scratch.mkdir()
    env = os.environ.copy()
    env.pop('HORIZON_SESSION_FILE', None)
    env.update(TMPDIR=str(scratch), HORIZON_HOME=str(scratch / 'game'),
               HORIZON_LAUNCHER_HOME=str(scratch / 'launcher'), XDG_DATA_HOME=str(scratch / 'data'),
               XDG_CONFIG_HOME=str(scratch / 'config'), HORIZON_LAUNCHER_NO_SELF_UPDATE='1',
               HORIZON_NO_AUDIO='1', HORIZON_NO_DISCORD='1', HORIZON_TELEMETRY_URL='off',
               CARGO_BUILD_JOBS='1', CARGO_NET_OFFLINE='true')
    target = Path.home() / '.cache/create-horizon/kinetic-probe-target'
    native = Path.home() / '.cache/create-horizon/target'
    helper = repo / 'tools/dev.sh'
    source_files = [repo / 'Cargo.toml', repo / 'Cargo.lock', repo / 'rust-toolchain.toml',
                    repo / 'crates/create-core/Cargo.toml', repo / 'tools/dev.sh',
                    repo / 'crates/create-core/examples/kinetic_probe.rs',
                    repo / 'crates/create-core/tests/geometry.rs',
                    repo / 'tools/compare_core.py', repo / 'tools/compare_geometry.py', repo / 'tools/compare_reference.py',
                    repo / 'tools/compare_vertical_geometry.py',
                    repo / 'tools/kinetic_probe_bridge.rs',
                    *sorted((repo / 'crates/create-core/src').rglob('*.rs')),
                    *sorted((repo / 'tools/wasm-probe').rglob('*.rs')),
                    *sorted((repo / 'tools/wasm-probe').rglob('Cargo.toml')), lock, Path(__file__).resolve()]
    hashes = {str(p.relative_to(repo)): hashlib.sha256(p.read_bytes()).hexdigest() for p in source_files}
    engine = workspace / 'horizon'
    def engine_state():
        git = ['git', '-C', str(engine)]
        return (subprocess.check_output(git + ['rev-parse', 'HEAD'], text=True, env=env).strip(),
                subprocess.check_output(git + ['diff', '--binary', 'HEAD'], env=env),
                subprocess.check_output(git + ['status', '--porcelain=v1', '--untracked-files=all'], env=env))
    engine_revision, engine_diff, engine_status = engine_state()
    if any(line.startswith(b'?? ') for line in engine_status.splitlines()):
        raise RuntimeError('Engine has untracked inputs; stage or isolate them before a receipt run')
    (run / 'engine.patch').write_bytes(engine_diff)
    (run / 'engine-status.txt').write_bytes(engine_status)
    port_diff = subprocess.check_output(['git', '-C', str(repo), 'diff', '--binary', 'HEAD'], env=env)
    (run / 'port.patch').write_bytes(port_diff)
    meta = {'scope': 'development server-component sandbox; no live world or machinery',
            'mode': ('vertical-equal-static-geometry' if args.vertical_equal else
                     'horizontal-static-geometry' if args.geometry else 'declared-topology'),
            'engine_revision': engine_revision, 'engine_diff_sha256': hashlib.sha256(engine_diff).hexdigest(),
            'port_revision': subprocess.check_output(['git', '-C', str(repo), 'rev-parse', 'HEAD'], text=True, env=env).strip(),
            'port_diff_sha256': hashlib.sha256(port_diff).hexdigest(),
            'rustc': subprocess.check_output([str(helper), 'rustc', '--version'], text=True, env=env).strip(),
            'cargo': subprocess.check_output([str(helper), 'cargo', '--version'], text=True, env=env).strip(),
            'source_sha256': hashes, 'references': [str(p.resolve()) for p in args.references],
            'reference_sha256': reference_hashes}
    (run / 'environment.json').write_text(json.dumps(meta, indent=2) + '\n')
    print(f'Wasm probe run: {run}', flush=True)
    def execute(name, command, extra=None, expected=0):
        command_env = {**env, **(extra or {})}
        (run / f'{name}-command.json').write_text(json.dumps(command) + '\n')
        with (run / f'{name}.log').open('w') as log:
            result = subprocess.run(command, cwd=repo, env=command_env, stdout=log, stderr=subprocess.STDOUT)
        (run / f'{name}-exit.txt').write_text(str(result.returncode) + '\n')
        print(f'{name}: exit {result.returncode}', flush=True)
        if result.returncode != expected:
            raise RuntimeError(f'{name} failed; inspect {run / (name + ".log")}')
    cargo = [str(helper), 'cargo']
    execute('guest-build', cargo + ['build', '--manifest-path', str(manifest), '--locked', '--offline',
            '--release', '--target', 'wasm32-unknown-unknown', '-p', 'kinetic-wasm-probe-guest'], {'CARGO_TARGET_DIR': str(target)})
    execute('host-build', cargo + ['build', '--manifest-path', str(manifest), '--locked', '--offline',
            '-p', 'kinetic-wasm-probe-host'], {'CARGO_TARGET_DIR': str(native)})
    host = native / 'debug/kinetic-wasm-probe-host'
    core = target / 'wasm32-unknown-unknown/release/kinetic_wasm_probe_guest.wasm'
    artifact = run / 'kinetic-probe.hmod'
    execute('package', [str(host), 'package', str(core), str(artifact)])
    meta['core_wasm_sha256'] = hashlib.sha256(core.read_bytes()).hexdigest()
    meta['component_package_sha256'] = hashlib.sha256(artifact.read_bytes()).hexdigest()
    (run / 'environment.json').write_text(json.dumps(meta, indent=2) + '\n')
    component_env = {'CREATE_KINETIC_COMPONENT': str(artifact)}
    if args.geometry or args.vertical_equal:
        execute('native-build', cargo + ['build', '--example', 'kinetic_probe', '--locked', '--offline'],
                {'CARGO_TARGET_DIR': str(native)})
        native_probe = native / 'debug/examples/kinetic_probe'
        meta['native_probe_sha256'] = hashlib.sha256(native_probe.read_bytes()).hexdigest()
        meta['comparison_sha256'] = {}
        (run / 'environment.json').write_text(json.dumps(meta, indent=2) + '\n')
        comparator = repo / ('tools/compare_vertical_geometry.py' if args.vertical_equal else 'tools/compare_geometry.py')
        for index, reference in enumerate(args.references, 1):
            outputs = []
            for mode, probe, extra in [('native', native_probe, None), ('sandbox', host, component_env)]:
                name = f'{mode}-comparison-{index}'
                output = run / f'{name}.json'
                execute(name, ['python3', str(comparator), str(reference), '--probe', str(probe), '--output', str(output)], extra)
                outputs.append(output.read_bytes())
                meta['comparison_sha256'][output.name] = hashlib.sha256(outputs[-1]).hexdigest()
            if outputs[0] != outputs[1]:
                raise RuntimeError(f'Native and sandbox canonical outputs differ for reference {index}')
        for mode, probe, extra in [('native', native_probe, None), ('sandbox', host, component_env)]:
            for control in ('axis', 'facing', 'edge-ratio'):
                execute(f'{mode}-{control}-control', ['python3', str(comparator), str(args.references[0]),
                        '--probe', str(probe), '--negative-control', control], extra, expected=1)
            execute(f'{mode}-duplicate-case-control', ['python3', str(comparator), str(args.references[0]),
                    '--probe', str(probe), '--negative-control', 'duplicate-case'], extra, expected=2)
    else:
        for index, reference in enumerate(args.references, 1):
            execute(f'comparison-{index}', ['python3', str(repo / 'tools/compare_core.py'), str(reference), '--probe', str(host)], component_env)
        execute('wrong-ratio-control', ['python3', str(repo / 'tools/compare_core.py'), str(args.references[0]),
                '--probe', str(host), '--negative-control'], component_env, expected=1)
    if any(hashlib.sha256((repo / key).read_bytes()).hexdigest() != value for key, value in hashes.items()):
        raise RuntimeError('Probe sources changed during the run')
    if engine_state() != (engine_revision, engine_diff, engine_status):
        raise RuntimeError('Engine source identity changed during the run')
    for reference, expected in zip(args.references, reference_hashes):
        load_run(reference)
        if any(hashlib.sha256((reference / name).read_bytes()).hexdigest() != value for name, value in expected.items()):
            raise RuntimeError('Reference evidence changed during the run')
    if (hashlib.sha256(core.read_bytes()).hexdigest() != meta['core_wasm_sha256']
            or hashlib.sha256(artifact.read_bytes()).hexdigest() != meta['component_package_sha256']):
        raise RuntimeError('Probe artifacts changed during the run')
    if args.geometry or args.vertical_equal:
        if hashlib.sha256(native_probe.read_bytes()).hexdigest() != meta['native_probe_sha256']:
            raise RuntimeError('Native probe artifact changed during the run')
        if any(hashlib.sha256((run / name).read_bytes()).hexdigest() != value
               for name, value in meta['comparison_sha256'].items()):
            raise RuntimeError('Canonical comparison output changed during the run')
    (run / 'environment.json').write_text(json.dumps(meta, indent=2) + '\n')
    (run / 'complete.txt').write_text('All selected sandbox comparisons and controls passed; '
                                       'static geometry mode also matches native canonical output\n')


if __name__ == '__main__':
    main()
