"""One-off FLUIDS-005 world session from Operations' frozen 007 inputs."""
from pathlib import Path
import argparse
import hashlib
import json
import os
import re
import shutil
import socket
import subprocess
import tempfile
import time
import uuid

cache = Path('/home/arbiter/.cache/create-horizon')
runs = cache / 'team-fluids-005'
mirror_root = Path('/mnt/g/Create Rusty/.local/native-fluids-005-runtime')

def sha(path):
    h = hashlib.sha256()
    with path.open('rb') as f:
        for part in iter(lambda: f.read(1024 * 1024), b''):
            h.update(part)
    return h.hexdigest()

p = argparse.ArgumentParser()
p.add_argument('mode', choices=['integrated', 'dedicated'])
p.add_argument('--binaries', required=True, type=Path)
p.add_argument('--binaries-sha256', required=True)
p.add_argument('--source-map', required=True, type=Path)
p.add_argument('--source-map-sha256', required=True)
p.add_argument('--source-head', required=True)
p.add_argument('--jar', required=True, type=Path)
p.add_argument('--jar-sha256', required=True)
p.add_argument('--resources', required=True, type=Path)
p.add_argument('--prerequisites', required=True, type=Path)
p.add_argument('--prerequisites-sha256', required=True)
p.add_argument('--resource-sha256', action='append', required=True,
               help='name=sha256 for each of four prepared base manifests')
p.add_argument('--run', type=Path)
args = p.parse_args()
receipt = args.binaries.resolve(strict=True)
source_map = args.source_map.resolve(strict=True)
jar = args.jar.resolve(strict=True)
resources = args.resources.resolve(strict=True)
prerequisites = args.prerequisites.resolve(strict=True)
assert sha(receipt) == args.binaries_sha256, 'binary receipt changed'
assert sha(source_map) == args.source_map_sha256, 'source map changed'
assert sha(jar) == args.jar_sha256, 'Create JAR changed'
assert sha(prerequisites) == args.prerequisites_sha256, '007 prerequisite receipt changed'
built = json.loads(receipt.read_text())
inputs = json.loads(source_map.read_text())
prepared = json.loads(prerequisites.read_text())
assert built['source_map_sha256'] == args.source_map_sha256
assert Path(built['source_inputs']).resolve(strict=True) == source_map
assert inputs['git_head'] == args.source_head and len(args.source_head) == 40
assert inputs['source_root'] == '/mnt/g/Create Rusty/.local/integration/create-teams-001'
assert prepared['source_map_sha256'] == args.source_map_sha256
assert Path(prepared['base_resources']).resolve(strict=True) == resources
composed = Path(prepared['resources']).resolve(strict=True)
assert set(prepared['resource_files']) == {'facts.json', 'gameplay-facts.json', 'items.json'}
for name, digest in prepared['resource_files'].items():
    assert sha(composed / name) == digest, '007 composed table changed: ' + name
base_sha = {}
for item in args.resource_sha256:
    name, separator, digest = item.partition('=')
    assert separator and name not in base_sha and re.fullmatch('[0-9a-f]{64}', digest), item
    base_sha[name] = digest
assert set(base_sha) == {'horizon-resources.json', 'facts.json', 'gameplay-facts.json', 'items.json'}
for name, digest in base_sha.items():
    assert sha(resources / name) == digest, 'prepared base manifest changed: ' + name
held = Path(built['directory']).resolve(strict=True)
expected = built['sha256']
assert set(expected) == {'horizon-client', 'horizon-ctl'}
for name, digest in expected.items():
    assert sha(held / name) == digest, name
runs.mkdir(exist_ok=True)
mirror_root.mkdir(exist_ok=True)
if args.run:
    run = args.run.resolve()
    assert run.parent == runs and (run / 'mode').read_text() == args.mode
    assert (run / 'resources').resolve(strict=True) == resources
    previous = json.loads((run / 'session-00.json').read_text())
    assert previous['source_head'] == args.source_head
    assert previous['source_map_sha256'] == args.source_map_sha256
    assert previous['binary_sha256'] == expected
    assert previous['jar_sha256'] == args.jar_sha256
    assert previous['resources'] == str(resources)
    assert previous['prerequisites_sha256'] == args.prerequisites_sha256
    assert previous['base_resource_sha256'] == base_sha
else:
    run = Path(tempfile.mkdtemp(prefix=args.mode + '-', dir=runs))
    (run / 'mode').write_text(args.mode)
    (run / 'resources').symlink_to(resources, target_is_directory=True)
mirror = mirror_root / run.name
mirror.mkdir(exist_ok=True)
index = 0
while (run / f'session-{index:02d}.json').exists():
    index += 1

roles = {'integrated': ['actor'], 'dedicated': ['server', 'actor', 'peer']}[args.mode]
port = None
if args.mode == 'dedicated':
    with socket.socket() as probe:
        probe.bind(('127.0.0.1', 0))
        port = probe.getsockname()[1]

session = {'mode': args.mode, 'run': str(run), 'mirror': str(mirror), 'index': index,
           'port': port, 'source_head': inputs['git_head'], 'source_revision': inputs['git_head'], 'native_tree': inputs['native_tree'],
           'source_map_sha256': args.source_map_sha256, 'binary_receipt_sha256': args.binaries_sha256,
           'prerequisites_sha256': args.prerequisites_sha256, 'base_resource_sha256': base_sha,
           'files': {str(held/name):digest for name,digest in expected.items()},
           'binary_sha256': expected, 'jar_sha256': args.jar_sha256, 'resources': str(resources), 'processes': {}}
children = {}
handles = []
for role in roles:
    root = run / role
    root.mkdir(exist_ok=True)
    (root / 'horizon-mods.txt').write_text('mod.create:true\n')
    assets = root / 'horizon-mods/create-resources'
    assets.mkdir(parents=True, exist_ok=True)
    if not (assets / jar.name).exists():
        shutil.copy2(jar, assets / jar.name)
    assert sha(assets / jar.name) == args.jar_sha256
    env = os.environ.copy()
    for variable, leaf in [('HORIZON_HOME', 'home'), ('HORIZON_LAUNCHER_HOME', 'launcher'),
                           ('XDG_DATA_HOME', 'data'), ('XDG_CONFIG_HOME', 'config'),
                           ('HORIZON_LAUNCHER_TRASH', 'trash'), ('HORIZON_LAUNCHER_CLIPBOARD', 'clipboard'),
                           ('HORIZON_LAUNCHER_DESKTOP', 'desktop')]:
        env[variable] = str(root / leaf)
    env.update(HORIZON_LAUNCHER_NO_SELF_UPDATE='1', CREATE_HORIZON_TRACE='1',
               CREATE_HORIZON_TRACE_POSES='1', WGPU_BACKEND='vulkan',
               WINIT_UNIX_BACKEND='x11', DISPLAY=':0',
               VK_ICD_FILENAMES='/usr/share/vulkan/icd.d/lvp_icd.json')
    for variable in ['WAYLAND_DISPLAY', 'HORIZON_SESSION_FILE', 'HORIZON_NO_AUDIO', 'CREATE_HORIZON_OWNER_PROBE']:
        env.pop(variable, None)
    env['HORIZON_NO_AUDIO']='1'
    command = [str(held / 'horizon-client'), '--resources', str(run / 'resources')]
    control = Path('/tmp') / f'team-fluids-005-{run.name}-{role}-{index:02d}.sock'
    assert not control.exists()
    if role == 'server':
        (root / 'eula.txt').write_text('eula=true\n')
        identities = [{'name': name, 'uuid': str(uuid.UUID(
            bytes=hashlib.md5(('OfflinePlayer:' + name).encode()).digest(), version=3))}
            for name in ['TeamActor', 'TeamPeer']]
        (root / 'whitelist.json').write_text(json.dumps(identities))
        (root / 'ops.json').write_text(json.dumps([dict(item, level=4, bypassesPlayerLimit=False)
                                                  for item in identities]))
        (root / 'server.properties').write_text(
            f'online-mode=false\nserver-ip=127.0.0.1\nserver-port={port}\n'
            'level-name=TeamFluids005\nlevel-seed=5\nwhite-list=true\n'
            'gamemode=survival\ndifficulty=peaceful\nview-distance=2\n'
            'simulation-distance=2\nmax-players=2\nmax-tick-time=0\n'
            'allow-flight=false\npause-when-empty-seconds=-1\n')
        command += ['--dedicated', '--nogui', '--port', str(port), '--world', 'TeamFluids005']
    else:
        name = 'TeamActor' if role == 'actor' else 'TeamPeer'
        env['HORIZON_PLAYER_NAME'] = name
        command += ['--name', name, '--offscreen', '--render-distance', '2',
                    '--width', '960', '--height', '600', '--control-socket', str(control)]
        if args.mode == 'integrated':
            command += ['--world', 'TeamFluids005', '--seed', '5', '--creative']
        else:
            command += ['--server', f'127.0.0.1:{port}']
    logfile = run / f'{role}-{index:02d}.log'
    handle = logfile.open('x')
    handles.append(handle)
    child = subprocess.Popen(command, cwd=root, env=env,
                             stdin=subprocess.PIPE if role == 'server' else subprocess.DEVNULL,
                             stdout=handle, stderr=subprocess.STDOUT)
    children[role] = child
    session['processes'][role] = {'pid': child.pid, 'command': command, 'game': str(held/'horizon-client'),
                                  'socket': str(control), 'log': str(logfile)}
    if role == 'server':
        deadline = time.monotonic() + 180
        while time.monotonic() < deadline:
            assert child.poll() is None, (child.returncode, str(logfile))
            if 'Done (' in logfile.read_text(errors='replace'):
                break
            time.sleep(.25)
        else:
            raise TimeoutError('dedicated startup: ' + str(logfile))

(run / f'session-{index:02d}.json').write_text(json.dumps(session, indent=2) + '\n')
(mirror / f'session-{index:02d}.json').write_text(json.dumps(session, indent=2) + '\n')
print(json.dumps(session), flush=True)
stop = mirror / f'stop-{index:02d}.request'
while not stop.exists():
    if args.mode == 'integrated' and children['actor'].poll() is not None:
        break
    time.sleep(.25)
if args.mode == 'dedicated':
    for role in ['actor', 'peer']:
        children[role].wait(timeout=90)
    if children['server'].poll() is None:
        children['server'].stdin.write(b'stop\n')
        children['server'].stdin.flush()
receipt = {'index': index, 'codes': {role: child.wait(timeout=90) for role, child in children.items()}}
(run / f'exit-{index:02d}.json').write_text(json.dumps(receipt, indent=2) + '\n')
(mirror / f'exit-{index:02d}.json').write_text(json.dumps(receipt, indent=2) + '\n')
for role in roles:
    shutil.copy2(run / f'{role}-{index:02d}.log', mirror / f'{role}-{index:02d}.log')
for handle in handles:
    handle.close()
print(json.dumps(receipt), flush=True)
assert all(code == 0 for code in receipt['codes'].values()), receipt
