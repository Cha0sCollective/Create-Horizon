#!/usr/bin/env python3
"""Run the pinned Create reference with an isolated development-only observer."""
import argparse
import gzip
import hashlib
import json
import os
from pathlib import Path
import struct
import subprocess
import sys
import uuid
import tomllib


def empty_template(path):
    """One original empty 16x8x16 structure; no game resources are copied."""
    def name(value):
        data = value.encode()
        return struct.pack('>H', len(data)) + data

    def integer(key, value):
        return b'\x03' + name(key) + struct.pack('>i', value)

    body = b'\x0a\x00\x00' + integer('DataVersion', 3955)
    body += b'\x09' + name('size') + b'\x03' + struct.pack('>i3i', 3, 16, 8, 16)
    body += b'\x09' + name('palette') + b'\x0a' + struct.pack('>i', 1)
    body += b'\x08' + name('Name') + name('minecraft:air') + b'\x00'
    for key in ('blocks', 'entities'):
        body += b'\x09' + name(key) + b'\x0a' + struct.pack('>i', 0)
    path.parent.mkdir(parents=True, exist_ok=True)
    data = gzip.compress(body + b'\x00', mtime=0)
    if not path.exists() or path.read_bytes() != data:
        path.write_bytes(data)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('mode', choices=('tasks', 'compile', 'capture', 'lock'))
    parser.add_argument('--offline', action='store_true')
    parser.add_argument('--workspace', type=Path, help='Workspace containing references/ and shared .local/ (default: repository parent)')
    parser.add_argument('--fixture-set', choices=('s0-edges-1', 'p02-arbitration-1'), default='s0-edges-1')
    args = parser.parse_args()
    if sys.platform != 'linux':
        parser.error('Run this tool inside the Ubuntu WSL development environment.')
    repo = Path(__file__).resolve().parents[1]
    workspace = args.workspace.resolve() if args.workspace else repo.parent
    reference = workspace / 'references/create'
    pins = tomllib.loads((repo / 'workspace.toml').read_text())
    accepted_artifacts = repo / pins['reference_environment']['artifact_lock']
    if args.mode == 'capture' and not accepted_artifacts.is_file():
        parser.error('Resolve and inspect dependencies with mode lock before accepting an artifact lock for capture')
    revision = subprocess.check_output(['git', '-C', str(reference), 'rev-parse', 'HEAD'], text=True).strip()
    if revision != pins['create_reference']['revision']:
        parser.error(f'Reference revision mismatch: {revision}')
    status = subprocess.check_output(['git', '-C', str(reference), 'status',
                                     '--porcelain=v1', '--untracked-files=all'], text=True)
    if status.strip():
        parser.error('Reference contains tracked changes or untracked inputs; restore the pinned snapshot before capture')
    wrapper = reference / 'gradle/wrapper/gradle-wrapper.jar'
    if hashlib.sha256(wrapper.read_bytes()).hexdigest() != pins['reference_environment']['wrapper_jar_sha256']:
        parser.error('Reference Gradle wrapper checksum mismatch')
    java_home = Path('/usr/lib/jvm/java-21-openjdk-amd64')
    java = java_home / 'bin/java'
    version = subprocess.run([str(java), '-version'], text=True, capture_output=True, check=True).stderr
    if not version.startswith('openjdk version "21.'):
        parser.error(f'Expected Java 21, got {version.splitlines()[0]}')
    if pins['reference_environment']['java_runtime'] not in version:
        parser.error('Java runtime differs from the pinned reference environment')
    run = workspace / '.local/s0/reference-runs' / str(uuid.uuid4())
    run.mkdir(parents=True)
    namespace = 'create_horizon_arbitration' if args.fixture_set == 'p02-arbitration-1' else 'create_horizon_reference'
    template_name = f'data/{namespace}/structure/empty.nbt'
    template_archive = run / 'resources' / template_name
    empty_template(template_archive)
    # A stable, verified original template input lets Gradle reuse its resource
    # output across fresh worlds instead of copying all Create resources again.
    template_root = workspace / '.local/s0/reference-templates' / namespace
    empty_template(template_root / template_name)
    if (template_root / template_name).read_bytes() != template_archive.read_bytes():
        parser.error('Generated template source differs from its per-run archive')
    adapter_files = [Path(__file__).resolve(), *sorted((repo / 'tools/reference').rglob('*'))]
    adapter_hashes = {str(p.relative_to(repo)): hashlib.sha256(p.read_bytes()).hexdigest()
                      for p in adapter_files if p.is_file() and
                      (args.fixture_set == 'p02-arbitration-1' or p.name != 'ArbitrationReferenceTests.java')}
    metadata = {'reference_revision': revision, 'minecraft': pins['create_reference']['minecraft'],
                'neoforge': pins['create_reference']['neoforge'], 'java': version.strip(),
                'mode': args.mode, 'adapter': str(repo / 'tools/reference'),
                'fixture_set': args.fixture_set,
                'adapter_sha256': adapter_hashes,
                'template_sha256': hashlib.sha256(template_archive.read_bytes()).hexdigest(),
                'observation_phase': 'GameTest scheduled callback; not a native-core comparison'}
    (run / 'environment.json').write_text(json.dumps(metadata, indent=2) + '\n')
    env = os.environ.copy()
    env.update(JAVA_HOME=str(java_home), GRADLE_USER_HOME=str(workspace / '.local/gradle'),
               CREATE_REFERENCE_ADAPTER=str(repo / 'tools/reference'), CREATE_REFERENCE_RUN=str(run),
               CREATE_REFERENCE_ARTIFACT_LOCK=str(accepted_artifacts),
               CREATE_REFERENCE_NAMESPACE=namespace,
               CREATE_REFERENCE_TEMPLATE_SOURCE=str(template_root),
               CREATE_REFERENCE_REQUIRE_LOCK='1' if args.mode == 'capture' else '0')
    env['PATH'] = str(java_home / 'bin') + os.pathsep + env['PATH']
    tasks = {'tasks': ['tasks', '--all'], 'compile': ['classes', 'createHorizonReferenceLock'],
             'capture': ['createHorizonReferenceLock', 'runGameTestServer'],
             'lock': ['createHorizonReferenceLock']}
    command = [str(java), '-Dorg.gradle.appname=gradlew', '-classpath',
               'gradle/wrapper/gradle-wrapper.jar', 'org.gradle.wrapper.GradleWrapperMain',
               '--no-daemon', '--max-workers=2', '-Dorg.gradle.jvmargs=-Xmx2G', '--console=plain',
               '--init-script', str(repo / 'tools/reference/init.gradle')]
    if args.offline:
        command.append('--offline')
    print(f'Reference run: {run}', flush=True)
    with (run / 'console.log').open('w') as log:
        process = subprocess.Popen(command + tasks[args.mode], cwd=reference, env=env,
                                   stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
        for line in process.stdout:
            log.write(line)
            print(line, end='', flush=True)
        code = process.wait()
    (run / 'exit-code.txt').write_text(str(code) + '\n')
    config_hashes = {str(p.relative_to(run)): hashlib.sha256(p.read_bytes()).hexdigest()
                     for p in run.rglob('*') if p.is_file()
                     and (p.name == 'server.properties' or any(part in {'config', 'serverconfig', 'defaultconfigs'}
                          for part in p.relative_to(run).parts[:-1]))}
    (run / 'config-sha256.json').write_text(json.dumps(config_hashes, indent=2) + '\n')
    return code


if __name__ == '__main__':
    raise SystemExit(main())
