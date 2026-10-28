"""Focused authoritative import-free guest counter in disposable game homes.

Receipts prove evaluator invocation, not necessarily reaching the Wasm step body.
No inventory, persistent payload, coherent checkpoint, animation or performance claim.
"""
import argparse
import json
from pathlib import Path
import re
import sys
import time
import traceback

import run_clock_save_error_smoke as clock_helper

base = clock_helper.base
OWNER = 'guest_counter_fixture'
TYPE = BLOCK = OWNER + ':counter'
POSITION = [133, 112, 3]
PREFIX = 'HORIZON_COUNTER_GUEST '


class GuestSession(clock_helper.ClockSession):
    def __init__(self, mode, game, resources, revision, server, client, variant):
        self.components = {'server': server, 'client': client}
        self.variant = variant
        self.known_keys = set()
        try:
            super().__init__(mode, game, resources, revision)
        except BaseException as error:
            if hasattr(self, 'out'):
                base.dump(self.out / 'result.json', {'passed': False, 'phase': 'setup',
                    'error': str(error), 'traceback': traceback.format_exc(), 'scope': __doc__})
            raise

    def preflight(self):
        own, parent = Path(__file__).resolve(), Path(clock_helper.__file__).resolve()
        (self.out / 'clock-driver.py').write_bytes(parent.read_bytes())
        (self.out / 'driver.py').write_bytes(own.read_bytes())
        self.meta.update(driver=str(own), driver_sha256=base.sha(own), scope=__doc__,
                         clock_driver_sha256=base.sha(parent), variant=self.variant)
        self.check('actual Linux runtime uses the qualified immutable launch parents',
                   sys.platform == 'linux'
                   and base.sha(parent) == 'e29e1b9d823a077be0172cac4b4a296e33c3643059f54778227f3c80125f5f30'
                   and base.sha(Path(base.__file__)) == '9cb17515a22cdcb74a4e550370b2ecc51c94810b4aeb32094d31b34c837f00fd')
        temporary = self.scratch / 'guest-temp'
        temporary.mkdir()
        for key in ['TMPDIR', 'TMP', 'TEMP']:
            self.env[key] = str(temporary)
            self.meta['isolated_environment'][key] = str(temporary)
        self.check('all temporary paths belong to this fresh canonical scratch directory',
                   not temporary.is_symlink()
                   and temporary.resolve().parent == self.scratch.resolve())
        super().preflight()
        manifest = json.loads((self.resources / 'horizon-resources.json').read_text())
        pinned = base.ENGINE / '.local/horizon-client/26.3/horizon-resources.json'
        reference = json.loads(pinned.read_text())
        self.check('prepared Minecraft/schema/client/server identities match the owned pinned resources',
                   all(manifest[key] == reference[key]
                       for key in ['minecraft', 'schema', 'client_sha256', 'server_sha256']))
        (self.out / 'pinned-resource-manifest.json').write_bytes(pinned.read_bytes())
        self.env['HORIZON_COUNTER_GUEST_RECEIPTS'] = '1'
        package = self.home / 'horizon-mods' / OWNER
        package.mkdir()
        self.meta['input_components'] = {}
        for side, source in self.components.items():
            source = source.resolve(strict=True)
            raw = source.read_bytes()
            self.check(side + ': supplied input is a bounded component binary',
                       source.is_file() and len(raw) <= 32 * 1024 * 1024
                       and raw[:8] == b'\x00asm\x0d\x00\x01\x00')
            (self.out / (side + '.wasm')).write_bytes(raw)
            (package / (side + '.wasm')).write_bytes(raw)
            self.meta['input_components'][side] = {'path': str(source), 'sha256': base.sha(source)}
            self.check(side + ': archive exactly matches the supplied component',
                       base.sha(self.out / (side + '.wasm')) == base.sha(source))
        descriptor = ("format=1\nhost-contract='paired-preview-1'\n[content]\nid='" + OWNER
            + "'\nname='Guest counter fixture'\nversion='0.0.0'\napi='0.2'\nside='both'\ncapabilities=['registries']\n"
              "[client]\nentry='client.wasm'\napi='0.1'\ncapabilities=[]\n"
              "[server]\nentry='server.wasm'\napi='0.3'\ncapabilities=[]\n")
        (package / 'horizon-paired.toml').write_text(descriptor)
        files = {
            f'data/{OWNER}/horizon/machine/counter.json': {'format': 1, 'save-schema': 1},
            f'data/{OWNER}/horizon/block/counter.json': {'template': 'minecraft:iron_block', 'machine-type': TYPE},
            f'assets/{OWNER}/blockstates/counter.json': {'variants': {'': {'model': OWNER + ':block/counter'}}},
            f'assets/{OWNER}/models/block/counter.json': {'parent': 'minecraft:block/cube_all',
                'textures': {'all': 'minecraft:block/yellow_concrete'}},
            f'assets/{OWNER}/items/counter.json': {'model': {'type': 'minecraft:model', 'model': OWNER + ':block/counter'}},
            f'assets/{OWNER}/lang/en_us.json': {'block.' + OWNER + '.counter': 'Guest counter fixture'},
        }
        for name, value in files.items():
            target = package / name
            target.parent.mkdir(parents=True, exist_ok=True)
            base.dump(target, value)
        with (self.home / 'horizon-mods.txt').open('a') as settings:
            settings.write('mod.' + OWNER + ':true\n')
        self.meta.update(guest_package_files=base.tree(package), receipts_enabled=True,
                         selected_mod_settings=(self.home / 'horizon-mods.txt').read_text(),
                         simulation_distance=5, normal_native_regression='separate immutable counter driver')
        for name in ['horizon-paired.toml', *files]:
            archive = self.out / 'guest-package' / name
            archive.parent.mkdir(parents=True, exist_ok=True)
            archive.write_bytes((package / name).read_bytes())
        base.dump(self.out / 'environment.json', self.meta)

    def start(self, phase, expect_refusal=False):
        super().start(phase, expect_refusal)
        self.check(phase + ': authoritative and local trials default disabled',
                   self.status().get('enabled') is False
                   and self.control('counter-trial', 'local-status').get('enabled') is False)
        self.control('save')
        pins = json.loads((self.world / 'horizon-paired-packages.json').read_text())
        self.package_identity = pins['packages'][OWNER]
        self.check(phase + ': both explicitly selected captured packages are pinned to this build',
                   set(pins['packages']) == {'paired_activation', OWNER}
                   and re.fullmatch('[0-9a-f]{64}', self.package_identity) is not None
                   and pins['build'] == 'sha256:' + self.meta['game_sha256'])
        base.dump(self.out / (phase + '-world-packages.json'), pins)

    def status(self):
        return self.control('counter-trial', 'status')

    def wait(self, predicate, label, timeout=90):
        deadline, value = time.monotonic() + timeout, None
        while time.monotonic() < deadline:
            value = self.status()
            if predicate(value):
                self.check(label, True)
                return value
            time.sleep(0.1)
        raise AssertionError(label + ': ' + json.dumps(value))

    @staticmethod
    def key(value):
        trial = value['trial']
        return (trial['scope']['world_instance'], trial['scope']['dimension'], trial['record']['instance'])

    def receipts(self):
        result = []
        text = (self.out / (self.phase + '-console.log')).read_text()
        for line in text.splitlines(keepends=True):
            if line.startswith(PREFIX) and not line.endswith('\n') and self.process is None:
                raise AssertionError('completed game left an incomplete guest receipt')
            if line.startswith(PREFIX) and line.endswith('\n'):
                if len(line.encode()) > 8192:
                    raise AssertionError('oversized guest receipt')
                result.append(json.loads(line[len(PREFIX):]))
        seen, counters, faulted = set(), {}, set()
        for row in result:
            scope, request = row['scope'], row['request']
            key = (scope['world_instance'], scope['dimension'], row['instance'])
            invocation = (*key, row['game_time'])
            if any(type(request[field]) is not int for field in
                   ['world_instance', 'schema', 'x', 'y', 'z', 'instance', 'counter', 'game_time']):
                raise AssertionError('guest request contains noninteger scalar fields')
            expected = {'world_instance': key[0], 'dimension': key[1], 'type_id': TYPE,
                'schema': 1, 'x': POSITION[0], 'y': POSITION[1], 'z': POSITION[2],
                'instance': key[2], 'counter': counters.get(key, 0), 'game_time': row['game_time']}
            if (row['package_identity'] != self.package_identity or row['type'] != TYPE
                    or request != expected or key not in self.known_keys or invocation in seen or key in faulted
                    or row['outcome'] not in ['committed', 'skipped', 'stale', 'fault']):
                raise AssertionError('guest receipt lost captured identity, ordering or request projection')
            seen.add(invocation)
            if row['outcome'] == 'committed':
                proposal = {'world_instance': key[0], 'instance': key[2], 'game_time': row['game_time'],
                            'expected_counter': request['counter'], 'next_counter': request['counter'] + 1}
                if (row['proposal'] != proposal or row['error'] is not None
                        or any(type(value) is not int for value in row['proposal'].values())):
                    raise AssertionError('committed receipt has an invalid scalar result')
                counters[key] = request['counter'] + 1
            elif row['outcome'] == 'fault':
                if row['proposal'] is not None or not isinstance(row['error'], str) or not row['error'] or len(row['error']) > 512:
                    raise AssertionError('fault receipt lacks bounded evaluator error evidence')
                faulted.add(key)
        base.dump(self.out / (self.phase + '-guest-receipts.json'), result)
        return result

    def move(self, position, dimension='minecraft:overworld'):
        self.command('execute in ' + dimension + ' run tp @s ' + ' '.join(map(str, position)))
        deadline = time.monotonic() + 90
        while time.monotonic() < deadline:
            state = self.control('state')
            player = state.get('player') or {}
            actual = player.get('position')
            if (state.get('dimension') == dimension and actual
                    and all(abs(actual[i] - position[i]) < 0.75 for i in [0, 2])):
                self.control('wait-ready', 90, timeout=95)
                self.check('actual owner position and ready terrain reached ' + dimension + str(position), True)
                return
            time.sleep(0.1)
        raise AssertionError('owner never reached actual requested destination')

    def set_block(self, state):
        self.command('setblock ' + ' '.join(map(str, POSITION)) + ' ' + state)
        self.await_block(POSITION, state)

    def freeze(self):
        self.command('tick freeze')
        before = self.status()
        for _ in range(30):
            time.sleep(0.2)
            after = self.status()
            if after['game_time'] == before['game_time']:
                return after
            before = after
        raise AssertionError('actual authoritative game time did not freeze')

    def quiet(self, label, advances=False, reference=None):
        before, calls = reference if reference is not None else (self.status(), self.receipts())
        if advances:
            after = self.wait(lambda s: s['game_time'] >= before['game_time'] + 3, label + ': clock advances')
        else:
            time.sleep(0.4)
            after = self.status()
            self.check(label + ': clock stays stopped', before['game_time'] == after['game_time'])
        self.check(label + ': no evaluator calls or committed payload changes',
                   after['trial']['record'] == before['trial']['record'] and self.receipts() == calls)
        return after

    def enable(self, guest=True):
        self.control('counter-trial', 'guest-on' if guest else 'on', TYPE, *POSITION)
        value = self.status()
        trial = value['trial']
        backend = {'kind': 'guest', 'owner': OWNER, 'package_identity': self.package_identity}
        self.check('trial selection has exact authoritative captured type and backend',
                   value['remote'] is False and value['level'] == 'minecraft:overworld'
                   and value['execution_role'] == ('IntegratedServer' if self.mode == 'integrated' else 'Client')
                   and trial['type'] == TYPE and trial['schema'] == 1 and trial['position'] == POSITION
                   and trial['state'] == {'name': BLOCK, 'properties': {}}
                   and trial['persistence'] == 'disabled-transient-trial'
                   and (trial.get('backend') == backend if guest else 'backend' not in trial)
                   and trial['record']['counter'] == 0 and trial['record']['last_advanced'] is None)
        if guest:
            self.known_keys.add(self.key(value))
        local = self.control('counter-trial', 'local-status')
        if self.mode == 'integrated':
            self.check('integrated replica owns no record or payload', local['remote'] is True
                       and local['enabled'] is False and local.get('trial') is None)
        self.wait(lambda s: s.get('position_block_tick_eligible') is True, 'actual trial position is eligible')
        return value

    def stale(self, key):
        reply = self.control('counter-trial', 'read', key[0], key[2], allow_error=True)
        self.check('old exact scope/instance key is refused', 'StaleKey' in str(reply.get('error')))

    def step(self, guest=True, fault=False):
        before, calls = self.status(), self.receipts()
        self.command('tick step 1')
        after = self.wait(lambda s: s['game_time'] >= before['game_time'] + 1, 'one simulation step completed')
        rows = self.receipts()[len(calls):]
        previous = before['trial']['record']
        if guest:
            self.check('one receipt captures the actual host key, input and stepped time',
                       len(rows) == 1 and (rows[0]['scope']['world_instance'],
                       rows[0]['scope']['dimension'], rows[0]['instance']) == self.key(before)
                       and rows[0]['game_time'] == after['game_time']
                       and rows[0]['request']['counter'] == previous['counter'])
        self.check('one step keeps exact identity and advances exactly one world phase',
                   self.key(after) == self.key(before) and after['game_time'] == before['game_time'] + 1)
        if fault:
            self.check('evaluator failure retains last committed counter and phase',
                       after['trial']['record'] == previous
                       and 'GuestFailure(Evaluation)' in str(after['trial']['fault'])
                       and len(rows) == 1 and rows[0]['outcome'] == 'fault')
        else:
            self.check('one step commits exactly +1 matching the native comparison',
                       after['trial']['record']['counter'] == previous['counter'] + 1
                       and after['trial']['record']['last_advanced'] == after['game_time']
                       and after['trial']['fault'] is None
                       and (len(rows) == 1 and rows[0]['outcome'] == 'committed' if guest else not rows))
        if self.mode == 'integrated':
            local = self.control('counter-trial', 'local-status')
            self.check('replica remains disabled after the actual authoritative phase',
                       local.get('enabled') is False and local.get('trial') is None)
        return self.quiet('single stepped phase remains frozen')

    def normal(self):
        for _ in range(3):
            self.step()
        old = self.key(self.status())
        self.set_block('minecraft:air')
        self.set_block(BLOCK)
        replaced = self.status()
        self.check('physical replacement creates a fresh zero host instance',
                   self.key(replaced) != old and replaced['trial']['record']['counter'] == 0)
        self.known_keys.add(self.key(replaced))
        self.stale(old)
        self.step()
        self.command('tick unfreeze')
        self.wait(lambda s: s['trial']['record']['counter'] >= 4, 'ordinary running guest calls commit')
        self.control('key', 'escape', 'tap')
        self.check('actual pause screen opened', self.control('screen')['screen'] == 'paused')
        time.sleep(0.5)
        paused = self.quiet('pause stops guest evaluation')
        self.control('resume')
        self.wait(lambda s: s['trial']['record']['counter'] > paused['trial']['record']['counter'], 'resume advances')
        self.freeze()
        self.quiet('explicit freeze stops guest evaluation')
        self.move([241, 100, 1])
        inactive = self.wait(lambda s: s['trial']['record']['resident'] is True
                  and s['position_ticket_block_ticking'] is False and s['position_storage_ticking'] is True
                  and s['position_block_tick_eligible'] is False, 'loaded 3x3 storage is outside simulation distance five')
        calls = self.receipts()
        self.command('tick unfreeze')
        stable = self.quiet('loaded but ticket-ineligible record', advances=True, reference=(inactive, calls))
        calls = self.receipts()
        self.move([641, 100, 1])
        dormant = self.wait(lambda s: s['trial']['record']['resident'] is False, 'ordinary chunk unload is dormant')
        self.check('ineligible travel and unload make no calls or payload progress', self.receipts() == calls
                   and all(dormant['trial']['record'][field] == stable['trial']['record'][field]
                           for field in ['instance', 'counter', 'last_advanced']))
        self.quiet('dormant record', advances=True)
        self.freeze()
        reload_calls = self.receipts()
        self.move([129, 100, 1])
        loaded = self.wait(lambda s: s['trial']['record']['resident'] is True
                           and s['position_block_tick_eligible'] is True, 'matching chunk reload is eligible')
        self.check('reload keeps exact key/counter/last phase without catch-up',
                   self.key(loaded) == self.key(dormant)
                   and loaded['trial']['record']['counter'] == dormant['trial']['record']['counter']
                   and loaded['trial']['record']['last_advanced'] == dormant['trial']['record']['last_advanced'])
        self.check('frozen reload produces no evaluator invocation', self.receipts() == reload_calls)
        self.quiet('frozen eligible reload stays quiet', reference=(loaded, reload_calls))
        parked = self.step()
        calls = self.receipts()
        self.move([0, 100, 0], 'minecraft:the_nether')
        self.check('Nether has no automatically installed backend', self.status()['enabled'] is False)
        if self.mode == 'in-frame':
            self.retirement(self.key(parked), parked['trial']['record']['counter'], ['dimension-switch'])
        time.sleep(0.4)
        self.check('dimension without a trial has no guest invocations', self.receipts() == calls)
        self.move([129, 100, 1])
        self.freeze()
        self.await_block(POSITION, BLOCK)
        if self.mode == 'integrated':
            returned = self.status()
            self.check('parked World retains its exact frozen guest configuration/record',
                       returned['trial'] == parked['trial'])
        else:
            self.check('in-frame World destruction disables its trial', self.status()['enabled'] is False)
            self.enable()
            self.stale(self.key(parked))
        self.step()

    def faulty(self):
        for _ in range(3):
            self.step()
        self.check('fault variant commits the authored initial zero-to-three sequence',
                   self.status()['trial']['record']['counter'] == 3)
        self.step(fault=True)
        before = self.status()['trial']['record']
        self.command('tick step 2')
        self.wait(lambda s: s['game_time'] >= before['last_advanced'] + 3, 'faulted world still processes simulation phases')
        self.quiet('sticky evaluator fault')
        self.set_block('minecraft:air')
        self.check('physical removal works while guest is faulted', self.status()['trial']['record'] is None)
        self.set_block(BLOCK)
        self.check('sticky fault does not readmit a payload', self.status()['trial']['record'] is None)
        self.quiet('faulted removal and replacement cannot invoke the evaluator')

    def retirement(self, key, counter, boundaries):
        rows = []
        for line in (self.out / (self.phase + '-console.log')).read_text().splitlines():
            if line.startswith('HORIZON_COUNTER_RETIRE '):
                rows.append(json.loads(line.removeprefix('HORIZON_COUNTER_RETIRE ')))
        base.dump(self.out / (self.phase + '-retirement-receipts.json'), rows)
        def matches(row):
            trial = row.get('trial', {})
            retired = trial.get('last_retirement') or {}
            record = retired.get('record') or {}
            return (row.get('boundary') in boundaries and trial.get('type') == TYPE
                and trial.get('scope') == {'world_instance': key[0], 'dimension': key[1]}
                and trial.get('record') is None and retired.get('reason') == 'Explicit'
                and record.get('instance') == key[2] and record.get('counter') == counter
                and trial.get('backend') == {'kind': 'guest', 'owner': OWNER,
                                             'package_identity': self.package_identity})
        self.check('exact guest record is explicitly retired at ' + str(boundaries), any(map(matches, rows)))

    def finish(self):
        process = self.process
        self.control('quit', allow_error=True, timeout=30)
        process.wait(timeout=90)
        self.log.close()
        self.log = None
        report = json.loads(self.report.read_text())
        self.process = None
        base.dump(self.out / (self.phase + '-natural-exit.json'),
                  {'pid': process.pid, 'exit_code': process.returncode, 'forced': False, 'report': report})
        self.check(self.phase + ': natural clean exit with no Game error',
                   process.returncode == 0 and report.get('error') is None)
        self.receipts()

    def run(self):
        failure = None
        try:
            self.start('initial')
            self.control('render-distance', 9)
            self.move([129, 100, 1])
            self.freeze()
            self.set_block(BLOCK)
            self.enable(guest=False)
            native = self.step(guest=False)
            self.control('counter-trial', 'off')
            self.enable()
            self.stale(self.key(native))
            self.quiet('new frozen guest record starts at zero')
            self.normal() if self.variant == 'normal' else self.faulty()
            self.control('save')
            final = self.status()
            self.finish()
            if self.variant == 'normal':
                self.retirement(self.key(final), final['trial']['record']['counter'],
                    ['integrated-exit'] if self.mode == 'integrated' else ['world-close', 'application-exit'])
            self.start('restart')
            self.move([129, 100, 1])
            self.await_block(POSITION, BLOCK)
            self.check('saved bound cube restores no backend or payload', self.status()['enabled'] is False)
            self.check('restart has no automatic evaluator calls', self.receipts() == [])
            self.finish()
        except BaseException as error:
            failure = {'error': str(error), 'traceback': traceback.format_exc()}
        finally:
            if self.process is not None:
                try:
                    self.stop(success=True)
                except Exception as error:
                    self.cleanup_errors.append(str(error))
            elif self.log is not None:
                self.log.close()
            base.dump(self.out / 'result.json', {'passed': failure is None and not self.cleanup_errors,
                'failure': failure, 'cleanup_errors': self.cleanup_errors, 'checks': self.checks,
                'mode': self.mode, 'variant': self.variant, 'driver_sha256': self.meta['driver_sha256'],
                'game_sha256': self.meta['game_sha256'], 'scope': __doc__, 'environment': self.meta})
        if failure or self.cleanup_errors:
            raise RuntimeError('guest counter smoke failed; see ' + str(self.out))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--game', type=Path, required=True)
    parser.add_argument('--resources', type=Path, required=True)
    parser.add_argument('--expected-revision', required=True)
    parser.add_argument('--server-component', type=Path, required=True)
    parser.add_argument('--client-component', type=Path, required=True)
    parser.add_argument('--mode', choices=['integrated', 'in-frame', 'both'], default='both')
    parser.add_argument('--variant', choices=['normal', 'trap', 'reject', 'invalid'], default='normal')
    args = parser.parse_args()
    for mode in ['integrated', 'in-frame'] if args.mode == 'both' else [args.mode]:
        GuestSession(mode, args.game, args.resources, args.expected_revision,
                     args.server_component, args.client_component, args.variant).run()


if __name__ == '__main__':
    main()
