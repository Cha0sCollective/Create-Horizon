#!/usr/bin/env python3
"""Compare complete P01 captures for a declared, versioned fixture set."""
import argparse
import hashlib
import json
import math
from pathlib import Path
import re

FIXTURES = {'direct_stop_reverse': 70, 'small_to_small': 20, 'large_to_small': 20,
            'small_to_large': 20, 'milling_16': 300, 'milling_256': 300}
FIXTURE_SETS = {'s0-initial-1': (1, FIXTURES),
                's0-edges-1': (2, {**FIXTURES, 'finite_stress': 120,
                                 'milling_interrupt_reverse': 340, 'milling_full_output': 100,
                                 'milling_one_full_slot': 100,
                                 'milling_input_changed': 100, 'milling_input_removed': 100,
                                 'retrieval_empty_hand': 40, 'retrieval_overflow': 40,
                                 'motor_value_settings': 60})}
MARKER = 'CREATE_HORIZON_TRACE '
HEX256 = re.compile(r'[0-9a-f]{64}')
ADAPTER_FILES = {'tools/reference.py', 'tools/reference/init.gradle',
                 'tools/reference/java/dev/ch4os/reference/PortReferenceTests.java'}
GENERATED_PROPERTIES_DATE = re.compile(
    rb'^#(?:Mon|Tue|Wed|Thu|Fri|Sat|Sun) (?:Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec) '
    rb'\d{2} \d{2}:\d{2}:\d{2} \S+ \d{4}\r?\n?$')


def comparison_config_hash(path):
    data = path.read_bytes()
    if path.name == 'server.properties':
        # Java Properties.store writes this comment; preserve all actual settings.
        data = b''.join(line for line in data.splitlines(keepends=True)
                        if not GENERATED_PROPERTIES_DATE.fullmatch(line))
    return hashlib.sha256(data).hexdigest()


def expected_machines(fixture, tick):
    motor = ('create:creative_motor', [2, 2, 2])
    if fixture.startswith('retrieval_'):
        return [('create:millstone', [2, 4, 2])]
    if fixture == 'motor_value_settings':
        return [motor, ('create:shaft', [2, 3, 2])]
    if fixture == 'finite_stress':
        return [('create:hand_crank', [2, 2, 2]), ('create:shaft', [2, 3, 2]),
                ('create:cogwheel', [2, 4, 2]), ('create:cogwheel', [3, 4, 2]), ('create:cogwheel', [2, 4, 3]),
                ('create:millstone', [2, 5, 2]),
                ('create:millstone' if tick > 30 else 'minecraft:air', [3, 5, 2]),
                ('create:millstone' if 50 < tick <= 70 else 'minecraft:air', [2, 5, 3])]
    if fixture == 'direct_stop_reverse':
        return [motor, ('minecraft:air' if tick > 30 else 'create:shaft', [2, 3, 2]),
                ('create:shaft', [2, 4, 2])]
    if fixture == 'milling_interrupt_reverse':
        return [motor, ('minecraft:air' if 40 < tick <= 80 else 'create:shaft', [2, 3, 2]),
                ('create:millstone', [2, 4, 2])]
    if fixture.startswith('milling_'):
        return [motor, ('create:shaft', [2, 3, 2]), ('create:millstone', [2, 4, 2])]
    driver = 'create:large_cogwheel' if fixture == 'large_to_small' else 'create:cogwheel'
    load = 'create:large_cogwheel' if fixture == 'small_to_large' else 'create:cogwheel'
    return [motor, (driver, [2, 3, 2]), (load, [3, 3, 2 if fixture == 'small_to_small' else 3])]


def validate_context(fixture, tick, context):
    require(type(context) is dict, 'Missing fixture context')
    if fixture.startswith('retrieval_'):
        overflow = fixture == 'retrieval_overflow'
        expected = {'player_gravel': 5 if tick > 10 and not overflow else 0,
                    'player_cobblestone': 3 if tick > 20 and not overflow else 0,
                    'player_iron': 2304 if overflow else 0,
                    'ground_gravel': 5 if tick > 10 and overflow else 0,
                    'ground_cobblestone': 3 if tick > 20 and overflow else 0}
        require(context == expected and all(type(value) is int for value in context.values()),
                'Invalid retrieval inventory accounting')
    elif fixture == 'motor_value_settings':
        expected = 16 if tick <= 10 else 1 if tick <= 20 else -1 if tick <= 30 else -256 if tick <= 40 else 256
        require(set(context) == {'setting'} and type(context['setting']) is int and context['setting'] == expected,
                'Invalid motor setting context')
    elif fixture == 'finite_stress':
        require(set(context) == {'capacity_per_rpm', 'millstone_impact_per_rpm'}
                and all(finite_number(value) for value in context.values())
                and context['capacity_per_rpm'] == 8 and context['millstone_impact_per_rpm'] == 4,
                'Wrong finite-source configuration')
    elif fixture == 'milling_interrupt_reverse':
        require(set(context) == {'paused_work'} and type(context['paused_work']) is int
                and (context['paused_work'] == -1 if tick <= 40 else 0 < context['paused_work'] <= 250),
                'Invalid interruption context')
    elif fixture in ('milling_full_output', 'milling_one_full_slot', 'milling_input_changed', 'milling_input_removed'):
        expected = {'external_cobblestone': int(tick > 20 and fixture in ('milling_input_changed', 'milling_input_removed')),
                    'external_barrier': int(tick <= 20 and fixture == 'milling_input_changed'),
                    'extracted_gravel': 0}
        require(context == expected and all(type(value) is int for value in context.values()),
                'Invalid external inventory accounting')
    else:
        require(context == {}, 'Unexpected fixture context')


def unique_object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError(f'Duplicate JSON key: {key}')
        result[key] = value
    return result


def decode(text):
    return json.loads(text, object_pairs_hook=unique_object)


def finite_number(value):
    return type(value) in (int, float) and math.isfinite(value)


def require(condition, message):
    if not condition:
        raise ValueError(message)


def load_run(directory):
    directory = Path(directory)
    require((directory / 'exit-code.txt').read_text().strip() == '0', f'{directory}: run did not succeed')
    environment = decode((directory / 'environment.json').read_text())
    require(environment['mode'] == 'capture', 'Expected a capture run')
    fixture_set = environment.get('fixture_set', 's0-initial-1')
    require(fixture_set in FIXTURE_SETS, 'Unknown reference fixture set')
    schema, fixtures = FIXTURE_SETS[fixture_set]
    require(isinstance(environment['reference_revision'], str)
            and re.fullmatch(r'[0-9a-f]{40}', environment['reference_revision']), 'Invalid source revision')
    require(environment['minecraft'] == '1.21.1' and environment['neoforge'] == '21.1.219', 'Unexpected platform')
    require(isinstance(environment['java'], str) and environment['java'].startswith('openjdk version "21.')
            and 'OpenJDK 64-Bit Server VM' in environment['java'], 'Missing Java runtime identity')
    adapter = environment['adapter_sha256']
    require(type(adapter) is dict and set(adapter) == ADAPTER_FILES, 'Incomplete adapter identity')
    require(all(isinstance(value, str) and HEX256.fullmatch(value) for value in adapter.values()), 'Invalid adapter hash')
    log = (directory / 'console.log').read_text()
    rows = {}
    for line in log.splitlines():
        if MARKER not in line:
            continue
        row = decode(line.split(MARKER, 1)[1])
        fixture, tick = row['fixture'], row['tick']
        require(fixture in fixtures and type(tick) is int and 1 <= tick <= fixtures[fixture], 'Unknown fixture/tick')
        require((fixture, tick) not in rows, f'Duplicate sample {fixture} tick {tick}')
        require(type(row['schema']) is int and row['schema'] == schema
                and row['phase'] == 'gametest_scheduled_callback', 'Unexpected trace schema/phase')
        require(row['dimension'] == 'minecraft:overworld' and type(row['world_seed']) is int, 'Missing world context')
        if schema == 2:
            validate_context(fixture, tick, row['context'])
        require(len(row['machines']) == len(expected_machines(fixture, tick)), 'Wrong machine observation count')
        require([(m['block'], m['position']) for m in row['machines']] == expected_machines(fixture, tick),
                f'Wrong or duplicate machine identity: {fixture} tick {tick}')
        for machine in row['machines']:
            require(len(machine['position']) == 3 and all(type(n) is int for n in machine['position']), 'Invalid position')
            properties = machine['properties']
            require(type(properties) is dict and all(isinstance(k, str) and isinstance(v, str)
                    for k, v in properties.items()), 'Missing/invalid block properties')
            if machine['block'] == 'minecraft:air':
                continue
            require(machine['block'].startswith('create:'), 'Unexpected block namespace')
            if machine['block'] == 'create:creative_motor':
                require(properties.get('facing') == 'up', 'Wrong motor facing')
            elif machine['block'] == 'create:hand_crank':
                require(properties.get('facing') == 'down', 'Wrong crank facing')
            elif machine['block'] != 'create:millstone':
                require(properties.get('axis') == 'y', 'Wrong shaft/cog axis')
            for key in ('theoretical_rpm', 'effective_rpm'):
                require(finite_number(machine[key]), f'Invalid {key}')
            network = machine['network']
            require(type(network['present']) is bool, 'Invalid network presence')
            require(all(finite_number(network[k]) for k in ('stress_su', 'capacity_su')), 'Invalid stress/capacity')
            if machine['block'] == 'create:millstone':
                require(type(machine['remaining_work']) is int, 'Invalid recipe work')
                if schema == 2:
                    require(type(machine['output_slot_limit']) is int and 1 <= machine['output_slot_limit'] <= 99,
                            'Missing output slot limit')
                slots = machine['slots']
                expected = {('input', 0), *(('output', i) for i in range(9))}
                require(len(slots) == 10 and {(s['owner'], s['slot']) for s in slots} == expected, 'Missing/duplicate inventory slot')
                for slot in slots:
                    require(type(slot['slot']) is int and isinstance(slot['item'], str)
                            and slot['item'].startswith('minecraft:') and type(slot['count']) is int
                            and 0 <= slot['count'] <= 64, 'Invalid fixture stack')
        rows[(fixture, tick)] = row
    expected = {(name, tick) for name, count in fixtures.items() for tick in range(1, count + 1)}
    missing = sorted(expected - rows.keys())
    require(not missing, f'Missing samples: {missing[:5]} ({len(missing)} total)')
    # A full trace without successful terminal assertions is not a passed run.
    require(re.search(rf'All {len(fixtures)} required tests passed', log) is not None, 'Missing complete test success summary')
    artifacts = decode((directory / 'resolved-artifacts.json').read_text())
    configs = decode((directory / 'config-sha256.json').read_text())
    require(type(artifacts) is list and bool(artifacts), 'Missing dependency records')
    identities = set()
    configurations = set()
    for artifact in artifacts:
        require(type(artifact) is dict and set(artifact) == {'configuration', 'coordinate', 'file', 'classifier', 'sha256'},
                'Incomplete dependency record')
        require(artifact['configuration'] in {'compileClasspath', 'runtimeClasspath'}, 'Unexpected dependency configuration')
        require(isinstance(artifact['coordinate'], str) and re.fullmatch(r'[^:\s]+:[^:\s]+:[^:\s]+', artifact['coordinate']),
                'Invalid dependency coordinate')
        require(isinstance(artifact['file'], str) and bool(artifact['file'])
                and '/' not in artifact['file'] and '\\' not in artifact['file'], 'Invalid artifact filename')
        require(artifact['classifier'] is None or isinstance(artifact['classifier'], str), 'Invalid artifact classifier')
        require(isinstance(artifact['sha256'], str) and HEX256.fullmatch(artifact['sha256']), 'Invalid artifact hash')
        identity = tuple(artifact[k] for k in ('configuration', 'coordinate', 'file', 'classifier'))
        require(identity not in identities, 'Duplicate artifact identity')
        identities.add(identity)
        configurations.add(artifact['configuration'])
    require(configurations == {'compileClasspath', 'runtimeClasspath'}, 'Missing dependency configuration')
    require(type(configs) is dict and bool(configs), 'Missing configuration records')
    require(all(isinstance(key, str) and isinstance(value, str) and HEX256.fullmatch(value)
                for key, value in configs.items()), 'Invalid configuration identity')
    config_files = [p for p in directory.rglob('*') if p.is_file()
                    and (p.name == 'server.properties' or any(part in {'config', 'serverconfig', 'defaultconfigs'}
                         for part in p.relative_to(directory).parts[:-1]))]
    actual_configs = {p.relative_to(directory).as_posix(): hashlib.sha256(p.read_bytes()).hexdigest()
                      for p in config_files}
    require(configs == actual_configs and any(Path(key).name == 'create-server.toml' for key in configs),
            'Incomplete configuration manifest or changed configuration files')
    return {'environment': {'fixture_set': fixture_set, **{key: environment[key] for key in
                            ('reference_revision', 'minecraft', 'neoforge', 'java', 'adapter_sha256')}},
            'artifacts': artifacts,
            'configs': {p.relative_to(directory).as_posix(): comparison_config_hash(p) for p in config_files},
            'traces': {f'{name}/{tick:03}': rows[(name, tick)] for name, tick in sorted(rows)}}


def first_difference(left, right, path='$'):
    if type(left) is not type(right):
        return f'{path}: type {type(left).__name__} != {type(right).__name__}'
    if isinstance(left, dict):
        if left.keys() != right.keys():
            return f'{path}: keys differ: {sorted(left.keys() ^ right.keys())}'
        for key in sorted(left):
            difference = first_difference(left[key], right[key], f'{path}.{key}')
            if difference:
                return difference
    elif isinstance(left, list):
        if len(left) != len(right):
            return f'{path}: length {len(left)} != {len(right)}'
        for index, (a, b) in enumerate(zip(left, right)):
            difference = first_difference(a, b, f'{path}[{index}]')
            if difference:
                return difference
    elif left != right:
        return f'{path}: {left!r} != {right!r}'
    return None


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('left', type=Path)
    parser.add_argument('right', type=Path)
    args = parser.parse_args()
    try:
        require(args.left.resolve() != args.right.resolve(), 'Reproducibility requires two distinct runs')
        left, right = load_run(args.left), load_run(args.right)
        difference = first_difference(left, right)
        if difference:
            print(difference)
            return 1
        count = len(FIXTURE_SETS[left['environment']['fixture_set']][1])
        print(f'Equal: {count} fixtures, {len(left["traces"])} samples, source/JDK/adapter, dependencies and configurations (generated Properties date excluded).')
        return 0
    except (OSError, ValueError, KeyError, TypeError) as error:
        print(f'Invalid reference evidence: {error}')
        return 2


if __name__ == '__main__':
    raise SystemExit(main())
