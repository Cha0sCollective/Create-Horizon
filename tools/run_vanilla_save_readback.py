"""Focused Horizon -> vanilla 26.3 chest/entity readback in disposable homes.

Uses shared save encoders, not a proof of out-of-range callback execution, full
vanilla parity, coherent checkpoints, fault recovery or performance.
"""
import argparse
import datetime
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import threading
import time
import traceback
import uuid
import zipfile

WORKSPACE = Path(__file__).resolve().parents[2]
ENGINE = WORKSPACE / 'horizon'
REVISION = 'a75184e1c0e0d34bcb99414f8f0b1550cc57650a'
GAME_SHA = '325e1f53f0860e3614229f514b4cfd1fd8f8254a8d30a49b567a52fc6a7bc25b'
JAVA = Path('/usr/lib/jvm/java-25-openjdk-amd64/bin/java')
CTL = WORKSPACE / '.local/bin/horizon-ctl'


def digest(path, algorithm='sha256'):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, algorithm).hexdigest()


def dump(path, value):
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + '\n')


def tree(path):
    return {str(p.relative_to(path)): digest(p) for p in sorted(path.rglob('*')) if p.is_file()}


class Readback:
    def __init__(self, game, resources):
        self.id = str(uuid.uuid4())
        self.out = WORKSPACE / '.local/h08a3/vanilla-readback' / self.id
        self.scratch = Path.home() / '.cache/create-horizon/vanilla-readback' / self.id
        self.out.mkdir(parents=True)
        self.game, self.resources = game.resolve(), resources.resolve()
        self.rust = self.log = self.vanilla = None
        self.sequence, self.checks, self.errors = 0, [], []
        self.meta = {'run_id': self.id, 'scratch': str(self.scratch),
                     'started_utc': datetime.datetime.now(datetime.UTC).isoformat(),
                     'scope': __doc__, 'driver_sha256': digest(Path(__file__))}
        (self.out / 'driver.py').write_bytes(Path(__file__).read_bytes())
        print(self.out, flush=True)

    def check(self, label, condition):
        if not condition:
            raise AssertionError(label)
        self.checks.append(label)
        print('  PASS ' + label, flush=True)

    def preflight(self):
        self.scratch.mkdir(parents=True)
        if sys.platform != 'linux':
            raise RuntimeError('this focused driver requires the actual Linux runtime')
        sys.path.insert(0, str(ENGINE / 'tools'))
        import test_save_roundtrip_parity as harness
        self.harness = harness
        for name in ['test_save_roundtrip_parity.py', 'lab.py', 'anvil_dump.py', 'save_roundtrip_scene.py']:
            source = ENGINE / 'tools' / name
            (self.out / name).write_bytes(source.read_bytes())
        lock_path = ENGINE / 'research-lock.json'
        lock = json.loads(lock_path.read_text())
        pin = lock['minecraft']
        self.check('research lock pins Minecraft 26.3 and Java 25', pin['version'] == '26.3' and pin['java_major'] == 25)
        imported = ENGINE / '.local/minecraft/26.3'
        manifest_path = imported / 'manifest.json'
        manifest = json.loads(manifest_path.read_text())
        self.check('cached launcher manifest is accepted and names pinned jars',
                   harness.lab.manifest_accepted(lock, digest(manifest_path, 'sha1'))
                   and manifest['id'] == '26.3' and manifest['javaVersion']['majorVersion'] == 25
                   and all(manifest['downloads'][kind]['sha1'] == pin[kind + '_sha1'] for kind in ['client', 'server']))
        jars = {kind: imported / 'jars' / (kind + '.jar') for kind in ['client', 'server', 'server-game']}
        self.check('owner-imported client/server jars match official lock', all(
            digest(jars[kind], 'sha1') == pin[kind + '_sha1'] for kind in ['client', 'server']))
        with zipfile.ZipFile(jars['server']) as bundle:
            versions = [line.split('\t') for line in bundle.read('META-INF/versions.list').decode().splitlines()]
            row = next(row for row in versions if len(row) == 3 and row[1] == '26.3')
            self.check('prepared server-game jar matches the official bundled version', digest(jars['server-game']) == row[0])
        resource_path = self.resources / 'horizon-resources.json'
        prepared = json.loads(resource_path.read_text())
        self.check('prepared resources match version and imported game jars',
                   prepared['minecraft'] == '26.3' and prepared['schema'] == 7
                   and prepared['client_sha256'] == digest(jars['client'])
                   and prepared['server_sha256'] == digest(jars['server-game']))
        receipt = self.game.with_name('build.json')
        build = json.loads(receipt.read_text())
        self.check('executable matches frozen H08a3 build', digest(self.game) == GAME_SHA
                   and build['game_sha256'] == GAME_SHA and build['engine_revision'] == REVISION)
        version = subprocess.run([str(JAVA), '-version'], capture_output=True, text=True, check=False)
        dump(self.out / 'java-version.json', {'args': [str(JAVA), '-version'], 'exit_code': version.returncode,
                                             'stdout': version.stdout, 'stderr': version.stderr})
        self.check('explicit Java executable is version 25', version.returncode == 0
                   and re.search(r'version "25(?:\.|\")', version.stdout + version.stderr) is not None)
        harness.SERVER = jars['server']
        self.home = self.scratch / 'rust-home'
        self.home.mkdir()
        homes = {'HORIZON_HOME': self.home, 'HORIZON_LAUNCHER_HOME': self.scratch / 'launcher',
                 'XDG_DATA_HOME': self.scratch / 'xdg-data', 'XDG_CONFIG_HOME': self.scratch / 'xdg-config'}
        for path in homes.values():
            path.mkdir(exist_ok=True)
        self.env = harness.lab.game_env(base=os.environ, **homes)
        # Horizon consumes and deletes launcher session files even on refusal.
        # Disposable game homes do not constrain an inherited absolute path.
        self.env.pop('HORIZON_SESSION_FILE', None)
        self.check('Rust launch cannot inherit a launcher authentication session',
                   'HORIZON_SESSION_FILE' not in self.env)
        self.env['HORIZON_TELEMETRY_URL'] = 'off'
        (self.home / 'horizon-mods').mkdir()
        (self.home / 'horizon-mods.txt').write_text('')
        (self.home / 'options.txt').write_text('renderDistance:2\nsimulationDistance:5\nmaxFps:60\nvsync:false\npauseOnLostFocus:false\n')
        self.socket = self.scratch / 'rust.sock'
        self.world = self.home / 'saves/Readback'
        self.java_home = self.scratch / 'vanilla-home'
        self.java_home.mkdir()
        self.meta.update(game=str(self.game), game_sha256=GAME_SHA, build=build,
                         resources=str(self.resources), resources_manifest_sha256=digest(resource_path),
                         jars={kind: {'path': str(path), 'sha256': digest(path)} for kind, path in jars.items()},
                         manifest_sha1=digest(manifest_path, 'sha1'), java=str(JAVA), java_sha256=digest(JAVA),
                         java_release_sha256=digest(JAVA.parent.parent / 'release'),
                         control_sha256=digest(CTL), rust_homes={k: str(v) for k, v in homes.items()},
                         helper_sha256={n: digest(self.out / n) for n in
                             ['test_save_roundtrip_parity.py', 'lab.py', 'anvil_dump.py', 'save_roundtrip_scene.py']})
        for name, path in [('research-lock.json', lock_path), ('minecraft-manifest.json', manifest_path),
                           ('resources-manifest.json', resource_path), ('build.json', receipt),
                           ('java-release.txt', JAVA.parent.parent / 'release'),
                           ('rust-options.txt', self.home / 'options.txt')]:
            (self.out / name).write_bytes(path.read_bytes())
        dump(self.out / 'environment.json', self.meta)

    def control(self, *args):
        self.sequence += 1
        result = subprocess.run([str(CTL), '--socket', str(self.socket), '--timeout', '180', *map(str, args)],
                                env=self.env, capture_output=True, text=True, timeout=190)
        dump(self.out / f'{self.sequence:03d}-rust-{args[0]}.json',
             {'args': args, 'exit_code': result.returncode, 'stdout': result.stdout, 'stderr': result.stderr})
        value = json.loads(result.stdout)
        if result.returncode or value.get('error'):
            raise RuntimeError(str(value))
        return value

    def rust_command(self, text):
        value = self.control('command', text)
        self.check('Rust command accepted: ' + text,
                   not self.harness.command_failed([json.dumps(value.get('chat', []))]))
        return json.dumps(value.get('chat', []))

    def write_world(self):
        command = [str(self.game), '--resources', str(self.resources), '--world', 'Readback', '--seed', '0',
                   '--creative', '--test-terrain', '--in-frame', '--offscreen', '--render-distance', '2',
                   '--width', '960', '--height', '540', '--control-socket', str(self.socket)]
        dump(self.out / 'rust-command.json', command)
        self.log = (self.out / 'rust-console.log').open('w')
        self.rust = subprocess.Popen(command, cwd=self.home, env=self.env, stdout=self.log, stderr=subprocess.STDOUT)
        deadline = time.monotonic() + 180
        while not self.socket.exists():
            if self.rust.poll() is not None or time.monotonic() > deadline:
                raise RuntimeError('Rust exited or never created its control socket')
            time.sleep(0.2)
        self.control('wait-ready', 180)
        self.check('Rust starts without selected mods', not (self.home / 'horizon-mods.txt').read_text().strip()
                   and not list((self.home / 'horizon-mods').iterdir()))
        x, y, z = self.control('state')['player']['block_pos']
        self.chest = [x + 2, y + 1, z]
        self.pig = [x + 4, y + 2, z + 2]
        self.tag = 'readback_' + self.id[:8]
        for rule in ['spawn_mobs', 'spawn_monsters', 'spawn_wandering_traders', 'spawn_patrols', 'spawn_phantoms', 'spawn_wardens', 'raids']:
            self.rust_command('gamerule ' + rule + ' false')
        self.rust_command('gamerule random_tick_speed 0')
        p = ' '.join(map(str, self.chest))
        self.rust_command('setblock ' + p + ' minecraft:chest{Items:[{Slot:0b,id:"minecraft:stone",count:23}]}')
        slots = self.control('container', *self.chest)['slots']
        self.check('Rust chest contains exactly 23 stone in slot zero',
                   slots[0] == {'item': 'minecraft:stone', 'count': 23} and all(s is None for s in slots[1:]))
        self.rust_command('summon minecraft:pig ' + ' '.join(map(str, self.pig))
                          + ' {UUID:[I;28672,0,0,99],NoAI:1b,NoGravity:1b,Silent:1b,Invulnerable:1b,Tags:["' + self.tag + '"]}')
        selector = '@e[type=minecraft:pig,tag=' + self.tag + ',limit=1]'
        self.rust_command('data get entity ' + selector)
        output = self.rust_command('execute if data entity ' + selector
            + ' {NoAI:1b,NoGravity:1b,Tags:["' + self.tag + '"]} run say RUST_PIG_VERIFIED')
        self.check('Rust actual entity data satisfies authored flags and tag', 'RUST_PIG_VERIFIED' in output)
        self.control('save-all')
        self.stop_rust()
        self.meta.update(chest=self.chest, pig=self.pig, tag=self.tag,
                         completed_rust_world=tree(self.world))
        shutil.copytree(self.world, self.java_home / 'world')
        self.check('completed world copied byte-for-byte into fresh vanilla home',
                   tree(self.java_home / 'world') == self.meta['completed_rust_world'])
        dump(self.out / 'environment.json', self.meta)

    def java_command(self, command):
        self.sequence += 1
        lines = self.vanilla.run(command)
        dump(self.out / f'{self.sequence:03d}-java-command.json', {'command': command, 'lines': lines})
        self.check('Java command accepted: ' + command, not self.harness.command_failed(lines))
        return lines

    def read_world(self):
        class VanillaReader(self.harness.Vanilla):
            def _pump(reader):
                try:
                    super()._pump()
                finally:
                    reader.drained.set()
        self.vanilla = VanillaReader.__new__(VanillaReader)
        self.vanilla.drained = threading.Event()
        previous = os.environ.copy()
        try:
            java_user = self.scratch / 'java-user'
            java_user.mkdir()
            java_homes = {key: self.scratch / ('java-' + name) for key, name in {
                'HORIZON_HOME': 'game', 'HORIZON_LAUNCHER_HOME': 'launcher',
                'XDG_DATA_HOME': 'xdg-data', 'XDG_CONFIG_HOME': 'xdg-config'}.items()}
            for path in java_homes.values():
                path.mkdir()
            os.environ.update(self.env)
            os.environ.pop('HORIZON_SESSION_FILE', None)
            os.environ.update({key: str(path) for key, path in java_homes.items()})
            os.environ.update(PATH=str(JAVA.parent) + os.pathsep + os.environ.get('PATH', ''),
                              JAVA_HOME=str(JAVA.parent.parent), HOME=str(java_user),
                              JAVA_TOOL_OPTIONS='-Duser.home=' + str(java_user))
            VanillaReader.__init__(self.vanilla, self.java_home, extra={'server-ip': '127.0.0.1',
                'view-distance': '2', 'simulation-distance': '5',
                'initial-enabled-packs': 'vanilla',
                'initial-disabled-packs': '', 'spawn-monsters': 'false', 'spawn-animals': 'false', 'spawn-npcs': 'false'})
        finally:
            os.environ.clear()
            os.environ.update(previous)
        shutil.copyfile(self.java_home / 'server.properties', self.out / 'server.properties')
        self.meta.update(vanilla_port=self.vanilla.port,
                         java_argv=self.vanilla.process.args, java_user_home=str(java_user),
                         java_homes={key: str(path) for key, path in java_homes.items()},
                         java_environment={'JAVA_HOME': str(JAVA.parent.parent), 'HOME': str(java_user),
                                           'JAVA_TOOL_OPTIONS': '-Duser.home=' + str(java_user),
                                           'PATH_prefix': str(JAVA.parent)})
        self.java_command('forceload add ' + ' '.join(map(str,
            [self.chest[0], self.chest[2], self.pig[0], self.pig[2]])))
        time.sleep(3)  # Same bounded loading allowance as the reused reader harness.
        for rule in ['spawn_mobs', 'spawn_monsters', 'spawn_wandering_traders', 'spawn_patrols', 'spawn_phantoms', 'spawn_wardens', 'raids']:
            self.java_command('gamerule ' + rule + ' false')
        self.java_command('scoreboard objectives add readback dummy')
        p = ' '.join(map(str, self.chest))
        self.java_command('data get block ' + p + ' Items')
        self.java_command('execute store result score #slots readback run data get block ' + p + ' Items')
        chest = self.java_command('execute if score #slots readback matches 1 if data block ' + p
            + ' {Items:[{Slot:0b,id:"minecraft:stone",count:23}]} run say JAVA_CHEST_VERIFIED')
        self.check('independent Java reads exact one-slot stone inventory', any('JAVA_CHEST_VERIFIED' in l for l in chest))
        selector = '@e[type=minecraft:pig,tag=' + self.tag + ',limit=1]'
        count = self.java_command('execute as @e[type=minecraft:pig,tag=' + self.tag + '] run say JAVA_PIG_INSTANCE')
        self.check('independent Java finds exactly one tagged pig', sum('JAVA_PIG_INSTANCE' in l for l in count) == 1)
        self.java_command('data get entity ' + selector)
        self.java_command('execute store result score #tags readback run data get entity ' + selector + ' Tags')
        pig = self.java_command('execute if score #tags readback matches 1 if data entity ' + selector
            + ' {UUID:[I;28672,0,0,99],NoAI:1b,NoGravity:1b,Tags:["' + self.tag + '"]} run say JAVA_PIG_VERIFIED')
        self.check('independent Java reads exact UUID flags and tag list', any('JAVA_PIG_VERIFIED' in l for l in pig))

    def stop_rust(self):
        if self.rust is not None:
            try:
                if self.rust.poll() is None:
                    self.control('quit')
                self.rust.wait(timeout=60)
                self.check('Rust completed a clean exit', self.rust.returncode == 0)
            finally:
                if self.rust.poll() is None:
                    self.rust.kill()
                    self.rust.wait(timeout=10)
                    self.errors.append('Rust required forced cleanup')
                dump(self.out / 'rust-exit.json', {'pid': self.rust.pid, 'exit_code': self.rust.returncode})
                self.rust = None
                if self.log:
                    self.log.close()

    def run(self):
        failure = None
        try:
            self.preflight()
            self.write_world()
            self.read_world()
        except BaseException as error:
            failure = {'error': str(error), 'traceback': traceback.format_exc()}
        finally:
            try:
                self.stop_rust()
            except Exception as error:
                self.errors.append('Rust cleanup: ' + str(error))
            if self.vanilla is not None and hasattr(self.vanilla, 'process'):
                process = self.vanilla.process
                self.meta['java_argv'] = process.args
                try:
                    if process.poll() is None:
                        self.vanilla.stop()
                    process.wait(timeout=10)
                    if process.returncode != 0 or not self.vanilla.drained.wait(5):
                        self.errors.append('Java exit or log drain was not clean')
                except Exception as error:
                    self.errors.append('Java cleanup: ' + str(error))
                    if process.poll() is None:
                        process.kill()
                        process.wait(timeout=10)
                (self.out / 'java-console.log').write_text('\n'.join(self.vanilla.lines) + '\n')
                dump(self.out / 'java-exit.json', {'pid': process.pid, 'exit_code': process.returncode})
                problems = self.vanilla.problems()
                dump(self.out / 'java-problems.json', problems)
                if problems:
                    self.errors.append('Java log contains problems under the original helper filter')
            try:
                if hasattr(self, 'java_home') and (self.java_home / 'server.properties').is_file():
                    shutil.copyfile(self.java_home / 'server.properties', self.out / 'server.properties')
                if hasattr(self, 'world') and 'completed_rust_world' in self.meta:
                    if tree(self.world) != self.meta['completed_rust_world']:
                        self.errors.append('Java readback changed the original completed Rust world')
            except Exception as error:
                self.errors.append('Final artifact/source audit: ' + str(error))
            dump(self.out / 'environment.json', self.meta)
            dump(self.out / 'result.json', {'passed': failure is None and not self.errors,
                 'failure': failure, 'cleanup_or_log_errors': self.errors, 'checks': self.checks,
                 'driver_sha256': self.meta['driver_sha256'], 'scope': __doc__})
        if failure or self.errors:
            raise RuntimeError(f'readback failed; see {self.out}')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--game', type=Path, required=True)
    parser.add_argument('--resources', type=Path, default=ENGINE / '.local/horizon-client/26.3')
    args = parser.parse_args()
    Readback(args.game, args.resources).run()


if __name__ == '__main__':
    main()
