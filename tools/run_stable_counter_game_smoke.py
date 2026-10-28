"""Run the qualified counter suite with one inert SDK-write fixture target.

The original drivers remain unchanged. Only the lower deferred-write target is
prepared as stone while the parent's exact event assertions run, then restored
by a checked command. Natural grass decay may resume after restoration.
"""
import argparse
import hashlib
import json
from pathlib import Path

PARENT_PINS = {
    'run_counter_game_smoke.py': 'd19c480572c3fbcb0ce01afeb868efb5dc276ad64606ab01f40edd9bf2d970e0',
    'run_mutation_game_smoke.py': 'cacda5cbd3a080e90bac62c99c8d336745031cdd135531b5ffa6b0728ac579f2',
    'run_paired_game_smoke.py': '9cb17515a22cdcb74a4e550370b2ecc51c94810b4aeb32094d31b34c837f00fd',
}


def verify_parent_pins():
    for name, expected in PARENT_PINS.items():
        path = Path(__file__).resolve().parent / name
        if hashlib.sha256(path.read_bytes()).hexdigest() != expected:
            raise RuntimeError(f'qualified parent changed: {name}')


# Check bytes before importing any qualified parent module.
verify_parent_pins()
import run_counter_game_smoke as counter


STONE = 'minecraft:stone'


def setblock_succeeded(response, position):
    # The fixed fresh-profile language is en_us. `ok` alone only acknowledges
    # dispatch; successful setblock emits this coordinate-specific feedback.
    expected = 'Changed the block at ' + ', '.join(map(str, position))
    chat = response.get('chat')
    return response.get('ok') is True and isinstance(chat, list) and bool(chat) and chat[0] == expected


class StableCounterSession(counter.CounterSession):
    def __init__(self, mode, game, resources):
        verify_parent_pins()
        super().__init__(mode, game, resources)
        self.env.pop('HORIZON_SESSION_FILE', None)
        self.stabilization = None
        own = Path(__file__).resolve()
        self.entrypoint_sha256 = counter.base.sha(own)
        (self.out / 'stable-counter-driver.py').write_bytes(own.read_bytes())
        self.meta.update(
            entrypoint_driver=str(own), entrypoint_driver_sha256=self.entrypoint_sha256,
            fixture_stabilization='lower SDK deferred-write target temporarily stone',
            session_environment_removed='HORIZON_SESSION_FILE' not in self.env,
        )
        counter.base.dump(self.out / 'environment.json', self.meta)

    def deferred_writes(self):
        player = self.control('state')['player']['block_pos']
        self.check('stable SDK fixture has an integer player block position',
                   isinstance(player, list) and len(player) == 3
                   and all(type(value) is int for value in player))
        lower = [player[0] + 2, player[1], player[2]]
        original = self.control('block', *lower)
        self.check('stable SDK lower target is already resident',
                   original.get('loaded') is True and isinstance(original.get('state'), str))
        self.check('stable SDK preparation starts with observation disabled',
                   self.control('mutations', 'status').get('enabled') is False)
        self.stabilization = {
            'position': lower, 'original': original['state'], 'prepared': STONE,
            'restore_attempted': False, 'restored_by_command': False,
            'restoration_noop_verified': False,
            'case_error': None, 'restore_error': None,
        }
        receipt = self.out / 'stable-sdk-target.json'
        counter.base.dump(receipt, self.stabilization)
        case_error = None
        try:
            if original['state'] != STONE:
                changed = self.control('command', 'setblock ' + ' '.join(map(str, lower)) + ' ' + STONE)
                self.check('stable SDK preparation command succeeds', setblock_succeeded(changed, lower))
            self.await_block(lower, STONE)
            self.check('stable SDK preparation keeps the actor in the same block',
                       self.control('state')['player']['block_pos'] == player)
            # Preserve every qualified parent's event, scope, sequence and count check.
            super().deferred_writes()
        except BaseException as error:
            case_error = error
            self.stabilization['case_error'] = str(error)
            raise
        finally:
            self.stabilization['restore_attempted'] = True
            try:
                # On successful parent completion the lower cell is stone again.
                # Do not require grass to remain grass after the command: it can
                # legitimately decay on a subsequent ordinary simulation tick.
                restored = self.control(
                    'command', 'setblock ' + ' '.join(map(str, lower)) + ' ' + original['state'])
                self.stabilization['restored_by_command'] = setblock_succeeded(restored, lower)
                same = None if self.stabilization['restored_by_command'] else self.control('block', *lower)
                self.stabilization['restoration_noop_verified'] = (
                    same is not None and same.get('loaded') is True
                    and same.get('state') == original['state'])
                ok = self.stabilization['restored_by_command'] or self.stabilization['restoration_noop_verified']
                if not ok:
                    raise AssertionError('stable SDK original lower target restoration failed')
                if case_error is None:
                    self.check('stable SDK original lower target restoration accepted', True)
            except BaseException as error:
                self.stabilization['restore_error'] = str(error)
                if case_error is None:
                    raise
            finally:
                counter.base.dump(receipt, self.stabilization)

    def run(self):
        try:
            super().run()
        finally:
            path = self.out / 'result.json'
            if path.exists():
                result = json.loads(path.read_text())
                result.update(entrypoint_driver_sha256=self.entrypoint_sha256,
                              fixture_stabilization=self.stabilization)
                counter.base.dump(path, result)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--mode', choices=['integrated', 'in-frame', 'both'], default='both')
    parser.add_argument('--game', type=Path, required=True)
    parser.add_argument('--resources', type=Path, required=True)
    args = parser.parse_args()
    for mode in (['integrated', 'in-frame'] if args.mode == 'both' else [args.mode]):
        StableCounterSession(mode, args.game, args.resources).run()


if __name__ == '__main__':
    main()
