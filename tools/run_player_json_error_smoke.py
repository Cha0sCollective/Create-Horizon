"""Focused integrated-owner JSON save/shutdown failures in disposable homes.

Checks specified on-disk mutations and natural shutdown/reopen, not atomic JSON,
guest retirement retention, a coherent checkpoint or complete vanilla parity.
"""
import argparse
import json
from pathlib import Path
import re
import sys
import traceback

import run_clock_save_error_smoke as clock_helper

base = clock_helper.base


class PlayerJsonSession(clock_helper.ClockSession):
    def __init__(self, game, resources, expected_revision, record):
        self.record = record
        self.json_blocker = None
        self.json_identity = None
        self.original_json = None
        super().__init__('integrated', game, resources, expected_revision)

    def preflight(self):
        # ClockSession already clears HORIZON_SESSION_FILE before this hook and
        # preserves the inherited display-service XDG_RUNTIME_DIR connection.
        own = Path(__file__).resolve()
        dependency = Path(clock_helper.__file__).resolve()
        (self.out / 'clock-driver.py').write_bytes(dependency.read_bytes())
        (self.out / 'driver.py').write_bytes(own.read_bytes())
        self.meta.update(driver=str(own), driver_sha256=base.sha(own), scope=__doc__,
                         clock_driver_sha256=base.sha(dependency), record=self.record,
                         dependency_files={'base-driver.py': self.meta['base_driver_sha256'],
                                           'anvil_dump.py': self.meta['nbt_driver_sha256'],
                                           'clock-driver.py': base.sha(dependency)})
        self.check('probe uses the actual Linux runtime and one literal JSON record kind',
                   sys.platform == 'linux' and self.record in ['advancements', 'stats'])
        super().preflight()
        self.check('integrated owner probe starts without an inherited session file',
                   'HORIZON_SESSION_FILE' not in self.env)
        self.check('prepared resource manifest has captured jar identities', all(
            re.fullmatch('[0-9a-f]{64}', json.loads((self.resources /
                'horizon-resources.json').read_text())[key])
            for key in ['client_sha256', 'server_sha256']))
        base.dump(self.out / 'environment.json', self.meta)

    def owned_path(self, segments, leaf_required=True):
        """Literal fresh-home ancestry; never follow a redirected fixture path."""
        home = self.home.resolve(strict=True)
        if self.home.is_symlink() or not home.is_relative_to(self.scratch.resolve(strict=True)):
            raise AssertionError('home is not this run\'s canonical scratch directory')
        current = self.home
        for segment in ['saves', 'PairedSmoke', *segments[:-1]]:
            current /= segment
            if current.is_symlink() or not current.is_dir():
                raise AssertionError('owned fixture parent missing or redirected: ' + str(current))
        expected = home.joinpath('saves', 'PairedSmoke', *segments[:-1])
        if current.resolve(strict=True) != expected:
            raise AssertionError('fixture parent escaped its literal fresh world')
        path = current / segments[-1]
        if path.is_symlink() or (leaf_required and not path.is_file()):
            raise AssertionError('fixture leaf is not the owned regular file: ' + str(path))
        return path

    def dat(self, label, segments):
        path = self.owned_path(segments)
        (self.out / (label + '-' + path.name)).write_bytes(path.read_bytes())
        value = self.nbt.plain(self.nbt.read_dat(path), full=True)
        base.dump(self.out / (label + '-' + path.name + '.json'), value)
        return value

    def saved_identity(self, label):
        level = self.dat(label, ['level.dat'])['Data']
        words = level['singleplayer_uuid']
        self.check(label + ': saved level identifies the actual owner',
                   isinstance(words, list) and len(words) == 4)
        text = ''.join(f'{word & 0xffffffff:08x}' for word in words)
        identity = '-'.join([text[:8], text[8:12], text[12:16], text[16:20], text[20:]])
        if hasattr(self, 'owner_uuid'):
            self.check(label + ': owner identity is stable', self.owner_uuid == identity)
        self.owner_uuid = identity
        return level

    def json_records(self, label):
        for kind in ['advancements', 'stats']:
            path = self.owned_path(['players', kind, self.owner_uuid + '.json'], leaf_required=False)
            if path == self.json_blocker:
                self.check(label + ': injected leaf remains an empty directory',
                           path.is_dir() and not any(path.iterdir()))
                continue
            self.check(label + ': independent ' + kind + ' record exists', path.is_file())
            raw = path.read_bytes()
            (self.out / (label + '-' + kind + '.json')).write_bytes(raw)
            value = json.loads(raw)
            self.check(label + ': ' + kind + ' retains the ordinary JSON projection',
                       isinstance(value, dict) and isinstance(value.get('DataVersion'), int)
                       and (kind != 'stats' or isinstance(value.get('stats'), dict)))

    def install_json_blocker(self):
        segments = ['players', self.record, self.owner_uuid + '.json']
        path = self.owned_path(segments)
        raw = path.read_bytes()
        original = path.lstat()
        self.check('injection starts from a parsed owned baseline JSON record',
                   isinstance(json.loads(raw), dict))
        (self.out / 'blocked-leaf-baseline.json').write_bytes(raw)
        # The server is held. Recheck the exact regular file before replacing it.
        again = path.lstat()
        if (again.st_dev, again.st_ino) != (original.st_dev, original.st_ino) or path.read_bytes() != raw:
            raise AssertionError('baseline JSON changed before injection')
        path.unlink()
        self.original_json = raw
        self.json_blocker = path
        path.mkdir()
        identity = path.lstat()
        self.json_identity = (identity.st_dev, identity.st_ino)
        base.dump(self.out / 'blocker.json', {'path': str(path), 'device': identity.st_dev,
                  'inode': identity.st_ino, 'baseline_sha256': base.sha(self.out / 'blocked-leaf-baseline.json'),
                  'kind': 'owned regular JSON replaced by owned empty directory'})

    def repair_json(self):
        if self.json_blocker is None:
            return
        path = self.owned_path(['players', self.record, self.owner_uuid + '.json'], leaf_required=False)
        try:
            identity = path.lstat()
        except FileNotFoundError:
            if self.json_identity is not None:
                raise AssertionError('owned JSON blocker disappeared; refusing repair')
            # mkdir failed after unlinking this run's archived baseline file.
            with path.open('xb') as stream:
                stream.write(self.original_json)
            self.json_blocker = None
            return
        if (path != self.json_blocker or not path.is_dir() or self.json_identity is None
                or (identity.st_dev, identity.st_ino) != self.json_identity):
            raise AssertionError('JSON blocker identity changed; refusing cleanup')
        path.rmdir()  # Only this exact owned empty directory, never recursive.
        with path.open('xb') as stream:
            stream.write(self.original_json)
        self.json_blocker = None
        self.json_identity = None

    def mutate(self, count, block, rate, spawn_x):
        p = ' '.join(map(str, self.position))
        for text in ['clear @s minecraft:stone', f'give @s minecraft:stone {count}',
                     f'setblock {p} minecraft:{block}', f'setworldspawn {spawn_x} 90 20',
                     f'time of minecraft:overworld rate {rate}',
                     'time of minecraft:overworld pause']:
            self.command(text)
        # Independent owner command view, not a possibly stale client inventory.
        value = self.control('command', 'execute if items entity @s container.* minecraft:stone'
                             ' run say OWNER_STONE_PRESENT')
        self.check('actual owner command sees the inventory mutation',
                   'OWNER_STONE_PRESENT' in json.dumps(value.get('chat', [])))

    def disk_block(self, label):
        x, y, z = self.position
        cx, cz = x // 16, z // 16
        path = self.owned_path(['dimensions', 'minecraft', 'overworld', 'region',
                               f'r.{cx // 32}.{cz // 32}.mca'])
        (self.out / (label + '-' + path.name)).write_bytes(path.read_bytes())
        chunks = self.nbt.region_chunks(path)
        root = self.nbt.plain(chunks[(cx & 31, cz & 31)], full=True)
        section = next(s for s in root['sections'] if s['Y'] == y // 16)
        container = section['block_states']
        palette = container['palette']
        index = ((y & 15) * 16 + (z & 15)) * 16 + (x & 15)
        bits = max(4, (len(palette) - 1).bit_length())
        per = 64 // bits
        entry = 0 if len(palette) == 1 else (
            (container['data'][index // per] & ((1 << 64) - 1)) >> ((index % per) * bits)
        ) & ((1 << bits) - 1)
        state = palette[entry]
        base.dump(self.out / (label + '-saved-block.json'), {'position': self.position, 'state': state})
        # The raw NBT helper retains 26.3 mixed-list wrappers: a default
        # string state is encoded as the sole empty-key compound entry.
        if isinstance(state, dict) and set(state) == {''}:
            state = state['']
        return state if isinstance(state, str) else state.get('Name', state.get('id'))

    def verify_saved(self, label, count, block, rate, spawn_x):
        level = self.saved_identity(label)
        player = self.dat(label, ['players', 'data', self.owner_uuid + '.dat'])
        controls = self.dat(label, ['data', 'minecraft', 'world_clocks.dat'])['data']['minecraft:overworld']
        self.check(label + ': Level world-spawn mutation reached disk', level['spawn']['pos'] == [spawn_x, 90, 20])
        self.check(label + ': actual owner inventory mutation reached disk',
                   sum(item.get('count', 0) for item in player['Inventory']
                       if item.get('id') == 'minecraft:stone') == count)
        self.check(label + ': world clock mutation reached disk',
                   controls.get('rate') == rate and controls.get('paused') == 1)
        self.check(label + ': chunk block mutation reached disk', self.disk_block(label) == 'minecraft:' + block)
        self.json_records(label)

    def finish(self, expected):
        process = self.process
        reply = self.control('quit', allow_error=True, timeout=30)
        base.dump(self.out / (self.phase + '-quit-reply.json'), reply)
        process.wait(timeout=90)  # Request ACK alone never proves saving succeeded.
        self.log.close()
        self.log = None
        report = json.loads(self.report.read_text()) if self.report.exists() else {}
        console = (self.out / (self.phase + '-console.log')).read_text()
        base.dump(self.out / (self.phase + '-natural-exit.json'),
                  {'pid': process.pid, 'exit_code': process.returncode, 'forced': False, 'report': report})
        self.process = None
        self.check(self.phase + ': natural expected exit code', process.returncode == expected)
        if expected:
            target = str(self.json_blocker)
            self.check('shutdown exposes the owner JSON failure beyond quit request acknowledgement',
                       target in str(report.get('error')) and target in console)

    def run(self):
        failure = None
        try:
            self.start('initial')
            self.check('actual game uses the integrated owner route',
                       self.control('status').get('server', {}).get('address') == 'local')
            x, y, z = self.control('state')['player']['block_pos']
            self.position = [x + 2, y + 1, z]
            self.control('server-hold', 'on')
            self.mutate(7, 'gold_block', 0.5, 20)
            self.control('save')
            self.verify_saved('baseline', 7, 'gold_block', 0.5, 20)
            self.install_json_blocker()
            self.mutate(13, 'diamond_block', 0.25, 21)
            reply = self.control('save', allow_error=True, timeout=90)
            base.dump(self.out / 'expected-save-error.json', reply)
            self.check('owner JSON save fails synchronously while the process remains alive',
                       reply.get('ok') is not True and str(self.json_blocker) in str(reply.get('error'))
                       and self.process.poll() is None)
            self.control('state')
            self.verify_saved('failed-save', 13, 'diamond_block', 0.25, 21)
            self.mutate(19, 'emerald_block', 0.75, 22)
            self.finish(1)
            self.verify_saved('failed-shutdown', 19, 'emerald_block', 0.75, 22)
            self.repair_json()
            self.start('repair-restart')
            self.control('server-hold', 'on')
            self.control('save')
            self.verify_saved('reopened', 19, 'emerald_block', 0.75, 22)
            self.finish(0)
        except BaseException as error:
            failure = {'error': str(error), 'traceback': traceback.format_exc()}
        finally:
            if self.process is not None:
                try:
                    self.stop(success=True)
                except Exception as error:
                    self.cleanup_errors.append('owned process cleanup: ' + str(error))
            elif self.log is not None:
                self.log.close()
            try:
                self.repair_json()
            except Exception as error:
                self.cleanup_errors.append('owned JSON repair: ' + str(error))
            base.dump(self.out / 'result.json', {'passed': failure is None and not self.cleanup_errors,
                'failure': failure, 'cleanup_errors': self.cleanup_errors, 'checks': self.checks,
                'driver_sha256': self.meta['driver_sha256'], 'dependencies': self.meta['dependency_files'],
                'game_sha256': self.meta['game_sha256'], 'record': self.record, 'scope': __doc__,
                'limitations': ['pre-open directory obstruction only; no arbitrary-write byte atomicity',
                    'natural failure/reopen proves termination and eventual lock release, not every cleanup call',
                    'no guest retirement ownership or coherent checkpoint claim']})
        if failure or self.cleanup_errors:
            raise RuntimeError('player JSON smoke failed; inspect ' + str(self.out))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--game', type=Path, required=True)
    parser.add_argument('--resources', type=Path, default=base.ENGINE / '.local/horizon-client/26.3')
    parser.add_argument('--expected-revision', required=True)
    parser.add_argument('--record', choices=['advancements', 'stats', 'both'], default='both')
    args = parser.parse_args()
    for record in ['advancements', 'stats'] if args.record == 'both' else [args.record]:
        PlayerJsonSession(args.game, args.resources, args.expected_revision, record).run()


if __name__ == '__main__':
    main()
