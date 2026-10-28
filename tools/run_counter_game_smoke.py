"""Fresh-game native typed-counter proof, including all qualified parent probes.

Always runs extended mutation, dimension, residency and captured-admission checks.
Counter payload/identity are transient: no inventory, guest ABI or checkpoint proof.
"""
import argparse
import json
from pathlib import Path
import time
import traceback

import run_mutation_game_smoke as mutation

base = mutation.base
TYPE = BLOCK = 'paired_activation:counter'
POSITION = [133, 112, 3]


class CounterSession(mutation.MutationSession):
    def __init__(self, mode, game, resources):
        # Parent validates the exact beacons ZIP/component before creating a run.
        super().__init__(mode, game, resources, extended=True, dimensions=True, residency=True)
        self.counter_checkpoint = None
        self.dimension_counter_key = None
        self.counter_proved = False
        try:
            own = Path(__file__).resolve()
            parent = Path(mutation.__file__).resolve()
            original = Path(base.__file__).resolve()
            self.meta.update(
                driver=str(own), driver_sha256=base.sha(own),
                mutation_driver=str(parent), mutation_driver_sha256=base.sha(parent),
                base_driver=str(original), base_driver_sha256=base.sha(original),
                counter_type=TYPE, counter_position=POSITION, counter_probes=True,
                simulation_distance=5, counter_probe_render_distance=9,
                counter_scope='one native transient typed record; no inventory, guest ABI or coherent checkpoint',
            )
            for name, source in [('driver.py', own), ('mutation-driver.py', parent),
                                 ('base-driver.py', original)]:
                (self.out / name).write_bytes(source.read_bytes())
            authored = {
                'data/paired_activation/horizon/machine/counter.json':
                    {'format': 1, 'save-schema': 1},
                'data/paired_activation/horizon/block/counter.json':
                    {'template': 'minecraft:iron_block', 'machine-type': TYPE},
                'assets/paired_activation/blockstates/counter.json':
                    {'variants': {'': {'model': 'paired_activation:block/counter'}}},
                'assets/paired_activation/models/block/counter.json':
                    {'parent': 'minecraft:block/cube_all',
                     'textures': {'all': 'minecraft:block/yellow_concrete'}},
                'assets/paired_activation/items/counter.json':
                    {'model': {'type': 'minecraft:model', 'model': 'paired_activation:block/counter'}},
            }
            for name, value in authored.items():
                path = self.package / name
                path.parent.mkdir(parents=True, exist_ok=True)
                base.dump(path, value)
            language = self.package / 'assets/paired_activation/lang/en_us.json'
            labels = json.loads(language.read_text())
            labels['block.paired_activation.counter'] = 'Transient native counter'
            base.dump(language, labels)
            options = self.home / 'options.txt'
            settings = options.read_text()
            if settings.count('simulationDistance:5\n') != 1:
                raise AssertionError('parent fresh-home simulation distance changed; review counter boundary')
            self.original = {str(p.relative_to(self.package)): p.read_bytes()
                             for p in self.package.rglob('*') if p.is_file()}
            self.meta.update(package_files=base.tree(self.package))
            base.dump(self.out / 'environment.json', self.meta)
        except BaseException as error:
            self.meta.update(package_files=base.tree(self.package), counter_setup_error=str(error))
            base.dump(self.out / 'environment.json', self.meta)
            base.dump(self.out / 'result.json', {'passed': False, 'phase': 'setup',
                      'error': str(error), 'traceback': traceback.format_exc(), 'checks': self.checks,
                      'driver_sha256': self.meta['driver_sha256'],
                      'mutation_driver_sha256': self.meta.get('mutation_driver_sha256'),
                      'base_driver_sha256': self.meta['base_driver_sha256']})
            raise

    def start(self, phase, expect_refusal=False):
        super().start(phase, expect_refusal)
        if expect_refusal:
            return
        status = self.control('counter-trial', 'status')
        local = self.control('counter-trial', 'local-status')
        self.check(phase + ': authoritative counter trial starts disabled',
                   status.get('enabled') is False and status.get('trial') is None)
        self.check(phase + ': local counter trial starts disabled',
                   local.get('enabled') is False and local.get('trial') is None)
        if self.mode == 'integrated':
            self.check(phase + ': counter replica is never the authority',
                       local.get('remote') is True and local.get('execution_role') == 'Client')
        if phase == 'restart':
            original = self.control('state')['player']['position']
            self.move_owner([129, 100, 1])
            self.await_block(POSITION, BLOCK)
            self.check('restart loads the saved bound cube without restoring a counter payload',
                       self.control('counter-trial', 'status').get('enabled') is False)
            self.move_owner(original)

    def status(self):
        value = self.control('counter-trial', 'status')
        if value.get('enabled'):
            trial = value.get('trial', {})
            scope = trial.get('scope', {})
            if not (value.get('remote') is False and value.get('level') == 'minecraft:overworld'
                    and value.get('execution_role') == ('IntegratedServer' if self.mode == 'integrated' else 'Client')
                    and trial.get('type') == TYPE and trial.get('schema') == 1
                    and trial.get('position') == POSITION
                    and trial.get('state') == {'name': BLOCK, 'properties': {}}
                    and trial.get('persistence') == 'disabled-transient-trial'
                    and trial.get('fault') is None
                    and scope.get('dimension') == 'minecraft:overworld'
                    and type(scope.get('world_instance')) is int and scope['world_instance'] > 0):
                raise AssertionError('counter status lost authority, type, canonical state or fault invariant')
        return value

    def await_status(self, predicate, label, timeout=30):
        deadline = time.monotonic() + timeout
        while time.monotonic() < deadline:
            value = self.status()
            if predicate(value):
                self.check(label, True)
                return value
            time.sleep(0.1)
        raise AssertionError(label + ': last status ' + json.dumps(value))

    @staticmethod
    def key(status):
        trial = status['trial']
        return {'scope': trial['scope'], 'instance': trial['record']['instance']}

    def read_key(self, key, stale=False):
        value = self.control('counter-trial', 'read', key['scope']['world_instance'],
                             key['instance'], allow_error=stale)
        if stale:
            self.check('old exact counter key is explicitly refused',
                       isinstance(value.get('error'), str) and 'StaleKey' in value['error'])
        else:
            self.check('exact counter key reads the committed host record',
                       value.get('record', {}).get('instance') == key['instance'])
        return value

    def freeze(self):
        self.typed('/tick freeze')
        previous = self.status()
        deadline = time.monotonic() + 10
        while time.monotonic() < deadline:
            time.sleep(0.3)
            current = self.status()
            if current['game_time'] == previous['game_time']:
                self.check('actual tick freeze stops authoritative game time', True)
                return current
            previous = current
        raise AssertionError('authoritative game time did not freeze')

    def stable(self, before, label, clock_advances=False):
        if clock_advances:
            after = self.await_status(lambda s: s['game_time'] >= before['game_time'] + 3,
                                      label + ': world clock continues')
        else:
            time.sleep(0.35)
            after = self.status()
            self.check(label + ': clock is stopped', after['game_time'] == before['game_time'])
        self.check(label + ': committed payload and instance stay unchanged',
                   self.key(after) == self.key(before)
                   and after['trial']['record']['counter'] == before['trial']['record']['counter']
                   and after['trial']['record']['last_advanced'] == before['trial']['record']['last_advanced'])
        return after

    def step_once(self):
        before = self.status()
        count = before['trial']['record']['counter']
        self.typed('/tick step 1')
        after = self.await_status(lambda s: s['game_time'] >= before['game_time'] + 1,
                                  'one requested simulation step completed')
        self.check('one step advances exactly once with the current game time',
                   after['game_time'] == before['game_time'] + 1
                   and after['trial']['record']['counter'] == count + 1
                   and after['trial']['record']['last_advanced'] == after['game_time']
                   and self.key(after) == self.key(before))
        return self.stable(after, 'completed single step stays frozen')

    def counter_on(self):
        self.control('counter-trial', 'on', TYPE, *POSITION)
        value = self.status()
        self.check('counter configuration has captured immutable type and schema', value.get('enabled') is True)
        if self.mode == 'integrated':
            local = self.control('counter-trial', 'local-status')
            self.check('enabling a native record leaves the replica disabled',
                       local.get('enabled') is False and local.get('trial') is None)
        return value

    def counter_set(self, state):
        self.typed('/setblock ' + ' '.join(map(str, POSITION)) + ' ' + state)
        self.await_block(POSITION, state)

    def place_counter_item(self):
        # Freeze leaves player movement/gravity running. Give the owner a real
        # platform at placement height, then verify XYZ rather than accepting
        # the already-correct chunk from the earlier low-altitude teleport.
        feet = [131.5, 112.0, 1.5]
        self.set_block([131, 111, 1], 'minecraft:stone')
        for cell in [[131, 112, 1], [131, 113, 1]]:
            self.set_block(cell, 'minecraft:air')
        self.typed('/tp @s ' + ' '.join(map(str, feet)))
        deadline = time.monotonic() + 20
        while time.monotonic() < deadline:
            player = self.control('state').get('player') or {}
            position = player.get('position')
            if (isinstance(position, list) and len(position) == 3
                    and all(abs(actual - wanted) < 0.05 for actual, wanted in zip(position, feet))):
                break
            time.sleep(0.1)
        else:
            raise AssertionError('owner did not reach exact safe counter placement XYZ')
        self.check('actual owner reached safe counter placement XYZ and height', True)
        message = 'You are standing on minecraft:stone'
        old = self.messages().count(message)
        self.typed('/ground')
        self.await_message(message, old)
        self.control('give', BLOCK, 1)
        deadline = time.monotonic() + 10
        while time.monotonic() < deadline:
            inventory = self.control('inventory')
            if inventory['counts'].get(BLOCK, 0) > 0:
                break
            time.sleep(0.1)
        self.check('counter item is in actual player inventory', inventory['counts'].get(BLOCK, 0) > 0)
        if not any(slot.get('item') == BLOCK for slot in inventory['hotbar']):
            source = next(slot['slot'] for slot in inventory['main'] if slot.get('item') == BLOCK)
            self.control('open-inventory')
            if self.control('ui')['screen'] == 'creative':
                tabs = [w for w in self.control('pad', 'status')['menu']['targets']
                        if w['kind'].startswith('tab')]
                rect = max(tabs, key=lambda w: (w['rect'][1], w['rect'][0]))['rect']
                self.control('click-at', rect[0] + rect[2] / 2, rect[1] + rect[3] / 2)
            for slot in [source, 0, source]:
                self.control('click-slot', slot)
            moved = self.control('inventory')
            self.check('counter hotbar setup preserves player item counts',
                       moved['counts'] == inventory['counts'] and moved['carried'] is None)
            self.control('key', 'escape', 'tap')
        position = self.control('state')['player']['position']
        self.check('owner remains on exact platform before actual item placement',
                   all(abs(actual - wanted) < 0.05 for actual, wanted in zip(position, feet)))
        support_top = [POSITION[0] + 0.5, POSITION[1], POSITION[2] + 0.5]
        eye = [position[0], position[1] + 1.62, position[2]]
        self.check('counter support top is within conservative player interaction reach',
                   sum((a - b) ** 2 for a, b in zip(eye, support_top)) < 4.0 ** 2)
        self.control('place', *POSITION, '--item', BLOCK)
        self.await_block(POSITION, BLOCK)

    def eligibility_move(self, position, predicate, label):
        self.move_owner(position)
        return self.await_status(predicate, label)

    def counter_proof(self):
        original = self.control('state')['player']['position']
        self.control('render-distance', 9)
        self.move_owner([129, 100, 1])
        self.freeze()
        self.counter_set('minecraft:air')
        self.set_block([POSITION[0], POSITION[1] - 1, POSITION[2]], 'minecraft:stone')
        for type_id, position in [
            ('not_namespaced', POSITION), ('foreign:counter', POSITION),
            ('paired_activation:missing', POSITION), (mutation.MARKER, POSITION),
            (TYPE, [641, 112, 3]), (TYPE, [133, 50000, 3]),
        ]:
            refusal = self.control('counter-trial', 'on', type_id, *position, allow_error=True)
            self.check('invalid type or unloaded/invalid target explicitly refuses: ' + type_id + str(position),
                       isinstance(refusal.get('error'), str)
                       and self.status().get('enabled') is False)
        empty = self.counter_on()
        self.check('resident air config creates no record', empty['trial'].get('record') is None)
        self.place_counter_item()
        placed = self.await_status(lambda s: s['trial'].get('record') is not None,
                                  'actual player item placement admits a host record')
        self.check('frozen placement initializes zero committed payload',
                   placed['trial']['record']['counter'] == 0
                   and placed['trial']['record']['last_advanced'] is None)
        first = self.key(placed)
        self.read_key(first)
        duplicate = self.control('counter-trial', 'on', TYPE, *POSITION, allow_error=True)
        self.check('duplicate trial enable refuses without resetting', isinstance(duplicate.get('error'), str))
        self.counter_set(BLOCK)
        self.check('resident no-op preserves instance and frozen payload', self.status()['trial']['record'] == placed['trial']['record'])
        self.counter_set('minecraft:air')
        removed = self.await_status(lambda s: s['trial'].get('record') is None,
                                    'command removal retires the committed record')
        self.check('block removal retains a bounded retirement snapshot',
                   removed['trial']['last_retirement']['record']['instance'] == first['instance']
                   and removed['trial']['last_retirement']['reason'] == 'BlockChanged')
        self.counter_set('minecraft:stone')
        self.counter_set(BLOCK)
        replaced = self.status()
        self.check('replacement admits a fresh checked instance', self.key(replaced) != first
                   and replaced['trial']['record']['counter'] == 0)
        self.read_key(first, stale=True)
        prior = self.key(replaced)
        off = self.control('counter-trial', 'off')
        self.check('explicit take retires and disables the trial',
                   off['status'].get('enabled') is False
                   and off['retired']['last_retirement']['reason'] == 'Explicit'
                   and off['retired']['last_retirement']['record']['instance'] == prior['instance'])
        self.counter_on()
        self.read_key(prior, stale=True)
        self.await_status(lambda s: s.get('position_block_tick_eligible') is True,
                          'bound counter is storage-ready and block-tick eligible')
        self.step_once()
        self.step_once()
        before = self.status()
        self.typed('/tick unfreeze')
        self.await_status(lambda s: s['trial']['record']['counter'] >= before['trial']['record']['counter'] + 3,
                          'native counter advances during ordinary running ticks')
        self.control('key', 'escape', 'tap')
        self.check('actual pause screen opened', self.control('screen').get('screen') == 'paused')
        # Allow the client pause notification/in-flight server tick to settle.
        time.sleep(0.5)
        paused = self.stable(self.status(), 'actual pause suspends counter')
        self.control('resume')
        self.await_status(lambda s: s['trial']['record']['counter'] > paused['trial']['record']['counter'],
                          'resumed world advances the same record')
        frozen = self.freeze()
        self.stable(frozen, 'explicit freeze suspends counter')
        # The supported minimum simulation distance is five. At distance seven,
        # the simulation ticket is level 33 (26 + 7), outside block ticking.
        # Render distance nine retains the counter's full 3x3 neighbourhood.
        inactive = self.eligibility_move([241, 100, 1], lambda s:
            s['trial']['record']['resident'] is True
            and s.get('position_ticket_block_ticking') is False
            and s.get('position_storage_ticking') is True
            and s.get('position_block_tick_eligible') is False,
            'loaded counter is excluded by simulation tickets with its neighbours resident')
        self.check('ineligible movement while frozen preserves payload',
                   inactive['trial']['record']['counter'] == frozen['trial']['record']['counter'])
        self.typed('/tick unfreeze')
        self.stable(inactive, 'loaded but simulation-ineligible counter', clock_advances=True)
        dormant = self.eligibility_move([641, 100, 1], lambda s:
            s['trial']['record']['resident'] is False,
            'ordinary streaming marks the counter record dormant')
        self.stable(dormant, 'unloaded dormant counter', clock_advances=True)
        before_reload = self.freeze()
        reloaded = self.eligibility_move([129, 100, 1], lambda s:
            s['trial']['record']['resident'] is True and s.get('position_block_tick_eligible') is True,
            'matching ordinary reload resumes storage-ready record')
        self.await_block(POSITION, BLOCK)
        self.check('reload retains exact key and payload without elapsed-time catch-up',
                   self.key(reloaded) == self.key(before_reload)
                   and reloaded['trial']['record']['counter'] == before_reload['trial']['record']['counter'])
        self.step_once()
        parked = self.status()
        self.dimension_counter_key = self.key(parked)
        self.travel('minecraft:the_nether', [0, 100, 0])
        self.check('Nether has no automatically configured native counter',
                   self.control('counter-trial', 'status').get('enabled') is False)
        self.travel('minecraft:overworld', [129, 100, 1])
        self.freeze()
        if self.mode == 'integrated':
            returned = self.status()
            self.check('parked authoritative World retains exact frozen record',
                       self.key(returned) == self.dimension_counter_key
                       and returned['trial']['record']['counter'] == parked['trial']['record']['counter'])
        else:
            self.check('in-frame World replacement leaves trial disabled', self.status().get('enabled') is False)
            self.counter_on()
            returned = self.status()
            self.check('new frozen trial on saved cube restores no native payload or identity',
                       self.key(returned) != self.dimension_counter_key
                       and returned['trial']['record']['counter'] == 0
                       and returned['trial']['record']['last_advanced'] is None)
            self.read_key(self.dimension_counter_key, stale=True)
        self.read_key(self.key(returned))
        self.counter_checkpoint = {'key': self.key(returned),
                                   'counter': returned['trial']['record']['counter']}
        self.typed('/tick unfreeze')
        advanced = self.await_status(lambda s:
            s['trial']['record']['counter'] > self.counter_checkpoint['counter'],
            'final configured record advances after unfreeze')
        self.counter_checkpoint['counter'] = advanced['trial']['record']['counter']
        self.control('render-distance', 2)
        self.move_owner(original)
        self.counter_proved = True
        base.dump(self.out / 'counter-phase-proof.json', {
            'counter_position': POSITION, 'counter_type': TYPE,
            'dimension_departure': parked, 'checkpoint': self.counter_checkpoint,
            'limits': 'transient native payload; no inventory, guest ABI or coherent checkpoint',
        })

    def residency_round_trip(self):
        self.counter_proof()
        super().residency_round_trip()
        self.check_counter_checkpoint('later residency probes retain the configured counter')

    def check_counter_checkpoint(self, label):
        value = self.status()
        self.check(label, self.key(value) == self.counter_checkpoint['key']
                   and value['trial']['record']['counter'] >= self.counter_checkpoint['counter'])
        self.read_key(self.counter_checkpoint['key'])
        self.counter_checkpoint['counter'] = value['trial']['record']['counter']

    def stop(self, success=True):
        if (success and self.phase == 'initial' and self.process is not None
                and self.process.poll() is None and self.counter_proved):
            self.check_counter_checkpoint('final exact counter key survives all inherited controls')
        return super().stop(success)

    def verify_retirements(self):
        receipts = []
        for line in (self.out / 'initial-console.log').read_text().splitlines():
            if line.startswith('HORIZON_COUNTER_RETIRE '):
                receipts.append(json.loads(line.removeprefix('HORIZON_COUNTER_RETIRE ')))
        base.dump(self.out / 'counter-retirement-receipts.json', receipts)

        def matches(receipt, key, minimum):
            trial = receipt.get('trial', {})
            retirement = trial.get('last_retirement') or {}
            record = retirement.get('record') or {}
            return (trial.get('scope') == key['scope'] and trial.get('type') == TYPE
                    and trial.get('schema') == 1 and trial.get('position') == POSITION
                    and trial.get('record') is None and retirement.get('reason') == 'Explicit'
                    and record.get('instance') == key['instance']
                    and type(record.get('counter')) is int and record['counter'] >= minimum)

        boundaries = ['integrated-exit'] if self.mode == 'integrated' else ['world-close', 'application-exit']
        self.check('normal initial shutdown explicitly retires final committed counter', any(
            r.get('boundary') in boundaries
            and matches(r, self.counter_checkpoint['key'], self.counter_checkpoint['counter']) for r in receipts))
        if self.mode == 'in-frame':
            departure = json.loads((self.out / 'counter-phase-proof.json').read_text())['dimension_departure']
            self.check('in-frame dimension switch emits old-scope retirement receipt', any(
                r.get('boundary') == 'dimension-switch'
                and matches(r, self.dimension_counter_key, departure['trial']['record']['counter']) for r in receipts))

    def run(self):
        try:
            super().run(True)
            self.verify_retirements()
        except BaseException as error:
            path = self.out / 'result.json'
            result = json.loads(path.read_text()) if path.exists() else {}
            result.update(passed=False, error=str(error), traceback=traceback.format_exc(), checks=self.checks)
            base.dump(path, result)
            raise
        finally:
            path = self.out / 'result.json'
            if path.exists():
                result = json.loads(path.read_text())
                result.update(driver_sha256=self.meta['driver_sha256'],
                              checks=self.checks,
                              mutation_driver_sha256=self.meta['mutation_driver_sha256'],
                              base_driver_sha256=self.meta['base_driver_sha256'],
                              counter_probes=True, counter_checkpoint=self.counter_checkpoint,
                              counter_limitations=['native inventory-free diagnostic only',
                                                   'no guest callbacks or persistent generation',
                                                   'ordinary saved block is not saved counter payload',
                                                   'no coherent checkpoint or crash-time retirement delivery'])
                base.dump(path, result)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--mode', choices=['integrated', 'in-frame', 'both'], default='both')
    parser.add_argument('--game', type=Path, required=True)
    parser.add_argument('--resources', type=Path, default=base.ENGINE / '.local/horizon-client/26.3')
    args = parser.parse_args()
    modes = ['integrated', 'in-frame'] if args.mode == 'both' else [args.mode]
    for mode in modes:
        CounterSession(mode, args.game, args.resources).run()


if __name__ == '__main__':
    main()
