"""Bounded real-game paired-folder smoke; all worlds and homes are disposable.

Run in WSL through tools/dev.sh after building the reviewed engine. This checks
activation and admission, not Create machinery or performance.
"""
import argparse
import datetime
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import time
import traceback
import uuid

WORKSPACE = Path(__file__).resolve().parents[2]
ENGINE = WORKSPACE / 'horizon'
CTL = WORKSPACE / '.local/bin/horizon-ctl'


def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def tree(root):
    return {str(p.relative_to(root)): sha(p) for p in sorted(root.rglob('*')) if p.is_file()}


def dump(path, value):
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + '\n')


class Session:
    def __init__(self, mode, game, resources):
        self.mode, self.game, self.resources = mode, game.resolve(), resources.resolve()
        self.player_label = 'SmokeTester' if mode == 'integrated' else 'Player'
        self.run_id = str(uuid.uuid4())
        self.out = WORKSPACE / '.local/h02d/game-runs' / self.run_id
        self.scratch = Path.home() / '.cache/create-horizon/runs' / self.run_id
        self.out.mkdir(parents=True)
        self.scratch.mkdir(parents=True)
        self.env = os.environ.copy()
        for key, name in {
            'HORIZON_HOME': 'game', 'HORIZON_LAUNCHER_HOME': 'launcher',
            'XDG_DATA_HOME': 'data', 'XDG_CONFIG_HOME': 'config',
            'HORIZON_LAUNCHER_TRASH': 'trash', 'HORIZON_LAUNCHER_CLIPBOARD': 'clipboard',
            'HORIZON_LAUNCHER_DESKTOP': 'desktop',
        }.items():
            folder = self.scratch / name
            folder.mkdir()
            self.env[key] = str(folder)
        self.env.update(HORIZON_LAUNCHER_NO_SELF_UPDATE='1', HORIZON_NO_AUDIO='1',
                        HORIZON_NO_DISCORD='1', HORIZON_TELEMETRY_URL='off')
        self.home = self.scratch / 'game'
        (self.home / 'options.txt').write_text(
            'graphicsPreset:"custom"\nrenderDistance:2\nsimulationDistance:5\n'
            'maxFps:60\nvsync:false\ninactivityFpsLimit:minimized\npauseOnLostFocus:false\n')
        (self.home / 'horizon-mods.txt').write_text('mod.paired_activation:true\n')
        self.package = self.home / 'horizon-mods/paired_activation'
        self.package.mkdir(parents=True)
        fixture = (ENGINE / 'crates/horizon-mods/tests/paired_activation.rs').read_text()
        descriptor = re.search(r'const DESCR: &str = r#"(.*?)"#;', fixture, re.S).group(1)
        (self.package / 'horizon-paired.toml').write_text(descriptor)
        components = Path.home() / '.cache/create-horizon/target/paired-guest/components'
        for name in ['client.wasm', 'server.wasm']:
            (self.package / name).write_bytes((components / name).read_bytes())
        authored = {
            'data/paired_activation/horizon/block/marker.json':
                {'template': 'minecraft:iron_block', 'properties': {'light': 7}},
            'assets/paired_activation/blockstates/marker.json':
                {'variants': {'': {'model': 'paired_activation:block/marker'}}},
            'assets/paired_activation/models/block/marker.json':
                {'parent': 'minecraft:block/cube_all', 'textures': {'all': 'minecraft:block/light_blue_concrete'}},
            'assets/paired_activation/items/marker.json':
                {'model': {'type': 'minecraft:model', 'model': 'paired_activation:block/marker'}},
            'assets/paired_activation/lang/en_us.json': {'block.paired_activation.marker': 'Paired smoke marker'},
        }
        for name, value in authored.items():
            path = self.package / name
            path.parent.mkdir(parents=True, exist_ok=True)
            dump(path, value)
        self.original = {str(p.relative_to(self.package)): p.read_bytes()
                         for p in self.package.rglob('*') if p.is_file()}
        self.world = self.home / 'saves/PairedSmoke'
        self.socket = self.scratch / 'ctl.sock'
        self.process, self.log, self.phase, self.sequence = None, None, '', 0
        self.checks = []
        build = json.loads(self.game.with_name('build.json').read_text())
        if build['game_sha256'] != sha(self.game):
            raise RuntimeError('game executable differs from its frozen build receipt')
        self.meta = {'run_id': self.run_id, 'mode': mode, 'scratch': str(self.scratch),
                     'driver_sha256': sha(Path(__file__)),
                     'engine_revision': build['engine_revision'], 'engine_tree': build['engine_tree'],
                     'game': str(self.game), 'game_sha256': build['game_sha256'],
                     'resources': str(self.resources),
                     'resources_manifest_sha256': sha(self.resources / 'horizon-resources.json'),
                     'package_files': tree(self.package), 'started_utc': datetime.datetime.now(datetime.UTC).isoformat(),
                     'scope': 'local paired activation/admission, no Create machinery or performance qualification'}
        dump(self.out / 'environment.json', self.meta)
        (self.out / 'driver.py').write_bytes(Path(__file__).read_bytes())
        print(f'{mode}: {self.out}', flush=True)

    def check(self, name, condition):
        if not condition:
            raise AssertionError(name)
        self.checks.append(name)
        print(f'  PASS {name}', flush=True)

    def control(self, *args, allow_error=False, timeout=45):
        self.sequence += 1
        result = subprocess.run([str(CTL), '--socket', str(self.socket), '--timeout', str(timeout),
                                 *map(str, args)], env=self.env, capture_output=True, text=True, timeout=timeout + 5)
        receipt = {'args': args, 'exit_code': result.returncode, 'stdout': result.stdout, 'stderr': result.stderr}
        dump(self.out / f'{self.phase}-{self.sequence:03d}-{args[0]}.json', receipt)
        try:
            value = json.loads(result.stdout)
        except ValueError:
            if allow_error:
                return receipt
            raise RuntimeError(receipt)
        if not allow_error and (result.returncode or isinstance(value, dict) and value.get('error')):
            raise RuntimeError(receipt)
        return value

    def start(self, phase, expect_refusal=False):
        self.phase = phase
        self.socket.unlink(missing_ok=True)
        args = [str(self.game), '--resources', str(self.resources),
                '--world', 'PairedSmoke', '--seed', '0', '--creative', '--test-terrain',
                '--render-distance', '2', '--offscreen', '--width', '960', '--height', '540',
                '--name', 'SmokeTester', '--control-socket', str(self.socket)]
        if self.mode == 'in-frame':
            args.append('--in-frame')
        dump(self.out / f'{phase}-command.json', args)
        self.log = (self.out / f'{phase}-console.log').open('w')
        self.process = subprocess.Popen(args, env=self.env, cwd=self.home, stdout=self.log, stderr=subprocess.STDOUT)
        deadline = time.monotonic() + 180
        while not self.socket.exists():
            if self.process.poll() is not None:
                if expect_refusal:
                    return
                raise RuntimeError(f'{phase}: game exited {self.process.returncode}')
            if time.monotonic() > deadline:
                raise TimeoutError(f'{phase}: no control socket')
            time.sleep(0.25)
        if not expect_refusal:
            self.control('wait-ready', 180, timeout=190)
            state = self.control('state')
            self.check(f'{phase}: loaded requested world', state.get('world') == 'PairedSmoke')

    def stop(self, success=True):
        if self.process is None:
            if self.log is not None:
                self.log.close()
            return
        cleanup_error = None
        try:
            if self.process.poll() is None:
                try:
                    self.control('quit', timeout=30)
                    self.process.wait(timeout=45)
                except Exception as error:
                    cleanup_error = error
                    self.process.terminate()
                    try:
                        self.process.wait(timeout=10)
                    except subprocess.TimeoutExpired:
                        self.process.kill()
                        self.process.wait(timeout=10)
            dump(self.out / f'{self.phase}-exit.json', {'exit_code': self.process.returncode,
                 'cleanup_error': str(cleanup_error) if cleanup_error else None})
            if success:
                if cleanup_error:
                    raise cleanup_error
                self.check(f'{self.phase}: clean exit', self.process.returncode == 0)
        finally:
            self.log.close()
            self.process = None

    def typed(self, text):
        self.control('key', 't', 'tap')
        self.control('type', text)
        screen = self.control('screen')
        self.check('typed chat has exact requested input', screen.get('chat_input') == text)
        self.control('key', 'enter', 'tap')
        screen = self.control('screen')
        self.check('typed chat submitted and closed', screen.get('screen') == 'playing')

    def return_to_play(self):
        # Esc leaves one Options subpage at a time before leaving Options itself.
        for _ in range(6):
            screen = self.control('screen').get('screen')
            if screen == 'playing':
                return
            if screen == 'paused':
                self.control('resume')
            elif screen == 'options':
                self.control('key', 'escape', 'tap')
            else:
                raise AssertionError(f'unexpected screen while returning to play: {screen}')
        raise AssertionError('Options did not return to the world')

    def messages(self):
        # `chat` and `chat-log` on a remote client report server lines only.
        # `styled` includes the actual client-local lines drawn on the screen.
        return json.dumps(self.control('messages', 100), ensure_ascii=False)

    def await_message(self, text, previous=0):
        deadline = time.monotonic() + 20
        while time.monotonic() < deadline:
            messages = self.messages()
            if messages.count(text) > previous:
                return messages
            time.sleep(0.25)
        raise AssertionError(f'missing drawn chat output {text!r}')

    def restore_package(self):
        for name, data in self.original.items():
            path = self.package / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)

    def run(self, extended):
        try:
            self.start('initial')
            status = self.control('status')
            remote = status.get('server')
            self.check('correct execution path', (isinstance(remote, dict) and remote.get('address') == 'local')
                       if self.mode == 'integrated' else remote is None)
            self.control('options', 'mods')
            widgets = self.control('ui')['widgets']
            rows = [w for w in widgets if (w.get('label') or '').startswith('Paired activation 0:')]
            self.check('one installed running package row', len(rows) == 1 and 'Stopped' not in rows[0]['label'])
            self.return_to_play()
            token = f'{self.mode}-{self.run_id[:8]}'
            self.typed('/hello ' + token)
            observed = self.await_message('hello ' + token + ' after ')
            match = re.search(r'hello ' + re.escape(token) + r' after (\d+) ticks', observed)
            self.check('client callback ticked and answered', match is not None and int(match.group(1)) > 0)
            reply = self.player_label + ' (overworld)'
            old = self.messages().count(reply)
            self.typed('/online')
            self.await_message(reply, old)
            self.check('server player command answered', True)
            self.typed('/save ' + token)
            self.await_message('saved')
            storage = self.home / 'horizon-mods/.horizon-mod-data/paired_activation'
            self.check('client settings use client side', (storage / 'client/note').read_text() == token)
            self.check('server settings use server side', (storage / 'server/seen').is_file()
                       and not (storage / 'client/seen').exists() and not (storage / 'server/note').exists())
            state = self.control('state')
            x, y, z = state['player']['block_pos']
            pos = [x + 2, y + 1, z]
            self.control('command', f'setblock {pos[0]} {pos[1]} {pos[2]} paired_activation:marker')
            time.sleep(1)
            block = self.control('block', *pos)
            self.check('registered custom block visible', 'paired_activation:marker' in json.dumps(block))
            self.control('save')
            pin_path = self.world / 'horizon-paired-packages.json'
            pins = json.loads(pin_path.read_text())
            self.check('one exact package and host pin', list(pins['packages']) == ['paired_activation']
                       and pins['build'] == 'sha256:' + self.meta['game_sha256'])
            dump(self.out / 'world-pins.json', pins)
            if extended:
                self.snapshot_reload(token)
            self.stop()
            self.restore_package()
            self.start('restart')
            self.typed('/load')
            self.await_message(token)
            self.check('same-build restart reads side settings', True)
            self.check('restart keeps exact pins', json.loads(pin_path.read_text()) == pins)
            self.check('restart preserves custom block', 'paired_activation:marker' in json.dumps(self.control('block', *pos)))
            self.stop()
            if extended:
                self.mismatch_controls()
            dump(self.out / 'result.json', {'passed': True, 'mode': self.mode, 'checks': self.checks,
                                          'extended': extended, 'game_sha256': self.meta['game_sha256']})
        except BaseException as error:
            dump(self.out / 'result.json', {'passed': False, 'checks': self.checks, 'error': str(error),
                                          'traceback': traceback.format_exc()})
            raise
        finally:
            self.stop(success=False)

    def snapshot_reload(self, token):
        (self.package / 'client.wasm').write_bytes(b'invalid replacement')
        (self.package / 'server.wasm').unlink()
        declaration = 'data/paired_activation/horizon/block/marker.json'
        (self.package / declaration).write_bytes(b'invalid replacement')
        asset = 'assets/paired_activation/lang/en_us.json'
        (self.package / asset).write_bytes(b'invalid replacement')
        overlay = self.home / '.horizon-cache/mod-content'
        (overlay / 'horizon-mod-content.stamp').unlink()
        self.control('key', 'f3', 'down')
        self.control('key', 't', 'tap')
        self.control('key', 'f3', 'up')
        self.control('wait-ready', 90, timeout=95)
        self.check('resource reload completed after invalidated stamp', (overlay / 'horizon-mod-content.stamp').is_file())
        # Declaration files are consumed into tables, not copied into the overlay.
        self.check('forced resource rebuild uses captured asset',
                   (overlay / asset).read_bytes() == self.original[asset])
        self.typed('/hello captured-' + token)
        self.await_message('hello captured-' + token + ' after ')
        reply = self.player_label + ' (overworld)'
        old = self.messages().count(reply)
        self.typed('/online')
        self.await_message(reply, old)
        self.check('source replacement cannot replace running components', True)

    def mismatch_controls(self):
        before = tree(self.world)
        descriptor = self.package / 'horizon-paired.toml'
        descriptor.write_text(descriptor.read_text().replace("version='0'", "version='1'"))
        self.refusal('changed-package', before)
        self.restore_package()
        (self.home / 'horizon-mods.txt').write_text('mod.paired_activation:false\n')
        self.refusal('missing-package', before)

    def refusal(self, phase, before):
        self.start(phase, expect_refusal=True)
        deadline = time.monotonic() + 90
        expected = 'paired world packages or exact host executable differ'
        while time.monotonic() < deadline:
            text = (self.out / f'{phase}-console.log').read_text()
            if expected in text or self.process.poll() is not None:
                break
            time.sleep(0.25)
        self.check(phase + ': explicit admission refusal', expected in text)
        self.stop(success=False)
        self.check(phase + ': saved world bytes unchanged', tree(self.world) == before)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--mode', choices=['integrated', 'in-frame', 'both'], default='both')
    parser.add_argument('--game', type=Path, required=True)
    parser.add_argument('--resources', type=Path, default=ENGINE / '.local/horizon-client/26.3')
    parser.add_argument('--extended', action='store_true')
    args = parser.parse_args()
    modes = ['integrated', 'in-frame'] if args.mode == 'both' else [args.mode]
    for mode in modes:
        Session(mode, args.game, args.resources).run(args.extended)


if __name__ == '__main__':
    main()
