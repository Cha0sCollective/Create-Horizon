"""Real-game passive mutation observer probes in fresh paired-package worlds.

Runs the original activation/static-state/admission checks unchanged, with actual
command, item placement, piston and SDK deferred-write observations layered in.
No observation here establishes machine lifecycle, causes, generations or saves.
"""
import argparse
import hashlib
import io
import json
from pathlib import Path
import zipfile

import run_paired_game_smoke as base


MARKER = 'paired_activation:marker'
DEFAULT = MARKER + '[axis=y,enabled=true]'
FIRST = MARKER + '[axis=x,enabled=false]'
OTHER = MARKER + '[axis=z,enabled=true]'


def descriptor(text):
    """Read only the canonical descriptors returned by the game's block probe."""
    if '[' not in text:
        return {'name': text, 'properties': {}}
    name, properties = text.split('[', 1)
    if not properties.endswith(']'):
        raise AssertionError('noncanonical block descriptor: ' + text)
    pairs = [part.split('=', 1) for part in properties[:-1].split(',')]
    if any(len(pair) != 2 for pair in pairs) or len({p[0] for p in pairs}) != len(pairs):
        raise AssertionError('noncanonical block properties: ' + text)
    return {'name': name, 'properties': dict(pairs)}


class MutationSession(base.Session):
    def __init__(self, mode, game, resources, extended=False):
        # Validate the optional local artifact before creating/printing any run.
        # Reuse these exact bytes so archive identity cannot change mid-setup.
        raw = module = None
        if extended:
            fixture = base.ENGINE / 'target/mods-test/mods/beacons.hmod'
            raw = fixture.read_bytes()
            with zipfile.ZipFile(io.BytesIO(raw)) as archive:
                module = archive.read('mod.wasm')
        super().__init__(mode, game, resources, static_states=True)
        self.extended = extended
        self.observer_instances = []
        own_path = Path(__file__).resolve()
        base_path = Path(base.__file__).resolve()
        # The base class archives its own source by module __file__. Replace that
        # ambiguous label explicitly and retain both exact drivers in the receipt.
        (self.out / 'driver.py').write_bytes(own_path.read_bytes())
        (self.out / 'base-driver.py').write_bytes(base_path.read_bytes())
        guests = {
            name: base.sha(self.package / name)
            for name in ['client.wasm', 'server.wasm']
        }
        if extended:
            installed = self.home / 'horizon-mods/beacons.hmod'
            installed.write_bytes(raw)
            (self.out / 'beacons.hmod').write_bytes(raw)
            guests['beacons.component'] = hashlib.sha256(module).hexdigest()
            guests['beacons.hmod'] = base.sha(installed)
            (self.home / 'horizon-mods.txt').write_text(
                'mod.paired_activation:true\nmod.beacons:true\n')
        self.meta.update(
            driver=str(own_path), driver_sha256=base.sha(own_path),
            base_driver=str(base_path), base_driver_sha256=base.sha(base_path),
            selected_guest_sha256=guests,
            observer_scope='explicit transient authoritative storage-World observer; no persistent identity',
            extra_code_fixture='existing SDK beacons sandbox artifact, identity recorded; build provenance not inferred'
                if extended else None,
            scope='local passive mutation observations plus original paired/static-state checks; no machine lifecycle, transactions or performance',
        )
        base.dump(self.out / 'environment.json', self.meta)

    def start(self, phase, expect_refusal=False):
        super().start(phase, expect_refusal)
        if expect_refusal:
            return
        status = self.control('mutations', 'status')
        local = self.control('mutations', 'local-status')
        self.check(phase + ': authoritative observer starts disabled', status.get('enabled') is False)
        self.check(phase + ': local observer starts disabled', local.get('enabled') is False)
        if self.mode == 'integrated':
            self.check(phase + ': replica is the remote client',
                       local.get('execution_role') == 'Client' and local.get('remote') is True)

    def observer_on(self, name=MARKER, capacity=64):
        self.control('mutations', 'on', name, capacity)
        status = self.control('mutations', 'status')
        scope = status.get('scope')
        self.check('observer enabled on authoritative current level',
                   status.get('enabled') is True and status.get('remote') is False
                   and status.get('execution_role') == ('IntegratedServer' if self.mode == 'integrated' else 'Client')
                   and isinstance(scope, dict) and scope.get('dimension') == 'minecraft:overworld'
                   and type(scope.get('world_instance')) is int and scope['world_instance'] > 0
                   and status.get('pending') == 0 and status.get('fault') is None)
        self.check('explicit enable never reuses a transient instance',
                   scope['world_instance'] not in self.observer_instances)
        self.observer_instances.append(scope['world_instance'])
        if self.mode == 'integrated':
            self.check('enabling server does not enable replica',
                       self.control('mutations', 'local-status').get('enabled') is False)
        return scope

    def observer_off(self):
        self.control('mutations', 'off')
        self.check('explicit observer teardown disables current storage world',
                   self.control('mutations', 'status').get('enabled') is False)

    def batch(self, label, scope, expected, first_sequence=1):
        result = self.control('mutations', 'drain')
        events = result.get('events')
        self.check(label + ': complete event count', isinstance(events, list) and len(events) == len(expected))
        for i, (event, (position, previous, current)) in enumerate(zip(events, expected)):
            self.check(label + f': exact transition {i + 1}',
                       event.get('position') == position
                       and event.get('previous') == descriptor(previous)
                       and event.get('current') == descriptor(current)
                       and event.get('scope') == scope
                       and event.get('sequence') == first_sequence + i
                       and event.get('cause') == 'unclassified'
                       and type(event.get('game_time')) is int and event['game_time'] >= 0)
        times = [event['game_time'] for event in events]
        self.check(label + ': captured game times are ordered', times == sorted(times))
        status = result.get('status', {})
        self.check(label + ': successful drain leaves enabled empty observer',
                   status.get('enabled') is True and status.get('pending') == 0 and status.get('fault') is None)
        return events

    def set_block(self, position, state):
        self.control('command', 'setblock ' + ' '.join(map(str, position)) + ' ' + state)

    def check_static_states(self, default_pos, restart=False):
        if restart:
            super().check_static_states(default_pos, restart=True)
            self.check('restart keeps observer disabled while static states load',
                       self.control('mutations', 'status').get('enabled') is False)
            return
        cases = []
        for i, (axis, enabled) in enumerate((a, e) for a in ['x', 'y', 'z'] for e in ['false', 'true']):
            position = [default_pos[0] + 2 + 2 * (i % 3), default_pos[1], default_pos[2] + 2 * (i // 3)]
            self.set_block(position, 'minecraft:air')
            cases.append((position, 'minecraft:air', f'{MARKER}[axis={axis},enabled={enabled}]'))
        placed = [default_pos[0] - 3, default_pos[1] - 1, default_pos[2]]
        self.set_block(placed, 'minecraft:air')
        self.await_block(placed, 'minecraft:air')
        scope = self.observer_on()
        # The base method drives six /setblock commands and a real item-use
        # placement through the game's existing mouse/window-event path.
        super().check_static_states(default_pos)
        self.batch('six commands and actual item placement', scope,
                   cases + [(placed, 'minecraft:air', DEFAULT)])
        duplicate = self.control('mutations', 'on', MARKER, 64, allow_error=True)
        self.check('duplicate enable explicitly refuses rather than resetting',
                   isinstance(duplicate, dict) and isinstance(duplicate.get('error'), str))
        self.batch('duplicate enable preserves empty observer', scope, [])
        self.observer_off()
        self.ordered_writes(default_pos)
        self.overflow(default_pos)
        if self.extended:
            self.piston(default_pos)
            self.deferred_writes()
        # All original marker cases and placement remain intact for the base
        # save/restart assertions. Deliberately leave this observer active so
        # restart-disabled is a meaningful nonpersistence check.
        self.observer_on()
        self.check('observer remains enabled before original save/reload/exit',
                   self.control('mutations', 'status').get('enabled') is True)

    def ordered_writes(self, origin):
        position = [origin[0], origin[1] + 2, origin[2] - 3]
        unwatched = [position[0] + 1, position[1], position[2]]
        for cell in [position, unwatched]:
            self.set_block(cell, 'minecraft:air')
            self.await_block(cell, 'minecraft:air')
        scope = self.observer_on()
        previous = 'minecraft:air'
        expected = []
        for current in [FIRST, OTHER, OTHER, 'minecraft:air', FIRST, 'minecraft:air']:
            self.set_block(position, current)
            self.await_block(position, current)
            if current != previous:
                expected.append((position, previous, current))
            previous = current
        self.set_block(unwatched, 'minecraft:stone')
        self.set_block(unwatched, 'minecraft:air')
        # This rejected command cannot reach a successful resident write.
        self.set_block([position[0], 50000, position[2]], FIRST)
        self.batch('ordered replacement removal no-op and filtering', scope, expected)
        self.batch('second drain contains no stale transitions', scope, [])
        self.observer_off()

    def overflow(self, origin):
        position = [origin[0], origin[1] + 2, origin[2] - 3]
        scope = self.observer_on(capacity=1)
        self.set_block(position, FIRST)
        self.set_block(position, OTHER)
        self.await_block(position, OTHER)
        status = self.control('mutations', 'status')
        self.check('overflow does not prevent the second committed world write',
                   status.get('pending') == 1 and status.get('fault') is not None)
        for _ in range(2):
            result = self.control('mutations', 'drain', allow_error=True)
            self.check('overflow drain refuses complete-batch success',
                       isinstance(result, dict) and isinstance(result.get('error'), str)
                       and 'exceeded' in result['error'] and 'events' not in result)
            again = self.control('mutations', 'status')
            self.check('overflow is sticky without sequence reset',
                       again.get('fault') == status['fault'] and again.get('pending') == 1
                       and again.get('scope') == scope)
        self.observer_off()
        self.set_block(position, 'minecraft:air')
        self.await_block(position, 'minecraft:air')

    def piston(self, origin):
        base_pos = [origin[0], origin[1] + 4, origin[2] + 5]
        power = [base_pos[0] - 1, base_pos[1], base_pos[2]]
        source = [base_pos[0] + 1, base_pos[1], base_pos[2]]
        target = [base_pos[0] + 2, base_pos[1], base_pos[2]]
        for position in [power, base_pos, source, target]:
            self.set_block(position, 'minecraft:air')
        self.set_block(base_pos, 'minecraft:piston[facing=east]')
        self.set_block(source, FIRST)
        self.await_block(source, FIRST)
        scope = self.observer_on()
        self.set_block(power, 'minecraft:redstone_block')
        self.await_block(target, FIRST)
        self.await_block(source, 'minecraft:piston_head[facing=east,short=false,type=normal]')
        moving = 'minecraft:moving_piston[facing=east,type=normal]'
        self.batch('actual piston displacement', scope,
                   [(source, FIRST, moving), (target, moving, FIRST)])
        self.observer_off()
        # Teardown first, then clean only the four explicitly prepared cells.
        for position in [power, base_pos, source, target]:
            self.set_block(position, 'minecraft:air')
            self.await_block(position, 'minecraft:air')

    def deferred_writes(self):
        state = self.control('state')
        x, y, z = state['player']['block_pos']
        cells = [[x + 2, y, z], [x + 2, y + 1, z]]
        before = []
        for position in cells:
            block = self.control('block', *position)
            self.check('deferred-write target is already loaded', block.get('loaded') is True)
            before.append(block['state'])
        self.check('deferred-write targets require real transitions',
                   all(previous != 'minecraft:glowstone' for previous in before))
        scope = self.observer_on('minecraft:glowstone')
        message = f'{self.player_label} lit a beacon at {x + 2} {y} {z}'
        old = self.messages().count(message)
        self.typed('/beacon 2')
        self.await_message(message, old)
        for position in cells:
            self.await_block(position, 'minecraft:glowstone')
        self.batch('actual SDK deferred world writes', scope,
                   [(position, previous, 'minecraft:glowstone') for position, previous in zip(cells, before)])
        old = self.messages().count(message)
        self.typed('/beacon 2')
        self.await_message(message, old)
        self.batch('repeated SDK writes are resident no-ops', scope, [])
        self.observer_off()
        # The upper cell can be the base smoke's original default marker.
        # Restore exact named properties before inherited save/restart checks.
        for position, previous in zip(cells, before):
            self.set_block(position, previous)
            self.await_block(position, previous)

    def run(self, extended):
        try:
            return super().run(extended)
        finally:
            path = self.out / 'result.json'
            if path.exists():
                result = json.loads(path.read_text())
                result.update(driver_sha256=self.meta['driver_sha256'],
                              base_driver_sha256=self.meta['base_driver_sha256'],
                              selected_guest_sha256=self.meta['selected_guest_sha256'],
                              transient_observer_instances=self.observer_instances,
                              mutation_extended=extended,
                              limitations=['passive committed-write observations only',
                                           'causes explicitly unclassified',
                                           'no persistent world identity or object generation',
                                           'no chunk lifecycle or transactional inventory proof',
                                           'dimension-change game probes not included'])
                base.dump(path, result)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--mode', choices=['integrated', 'in-frame', 'both'], default='both')
    parser.add_argument('--game', type=Path, required=True)
    parser.add_argument('--resources', type=Path, default=base.ENGINE / '.local/horizon-client/26.3')
    parser.add_argument('--extended', action='store_true',
                        help='also test piston/SDK writes and inherited captured-reload/admission controls')
    args = parser.parse_args()
    modes = ['integrated', 'in-frame'] if args.mode == 'both' else [args.mode]
    for mode in modes:
        MutationSession(mode, args.game, args.resources, args.extended).run(args.extended)


if __name__ == '__main__':
    main()
