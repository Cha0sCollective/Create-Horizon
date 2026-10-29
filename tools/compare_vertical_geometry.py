#!/usr/bin/env python3
"""Compare six authored Y/UP static snapshots to the retained complete E/F corpus."""
import argparse
import copy
import json
from pathlib import Path
import subprocess
from compare_reference import decode, finite_number, load_run

CHECKPOINTS = (('direct_stop_reverse', 10), ('direct_stop_reverse', 30), ('direct_stop_reverse', 70),
               ('small_to_small', 20), ('large_to_small', 20), ('small_to_large', 20))
REFERENCE_REVISION = '7ff2760e4924d35d4bc2e22c4b19923f0178cd16'
ROTATION_FIELDS = {'id', 'theoretical_rpm', 'effective_rpm', 'stress_su', 'capacity_su', 'state'}
TRANSMISSIONS = {'shaft', 'equal_cogs', 'large_to_small', 'small_to_large'}
BLOCK_IDS = {'motor': 'create:creative_motor', 'shaft': 'create:shaft',
             'small_cog': 'create:cogwheel', 'large_cog': 'create:large_cogwheel'}


def case_input(fixture, tick):
    """Inputs are authored from the sample-before-action schedule, never measured outputs."""
    if (fixture, tick) not in CHECKPOINTS:
        raise ValueError('Unknown selected vertical checkpoint')
    direct = fixture == 'direct_stop_reverse'
    setting = -16 if direct and tick in (30, 70) else 16
    blocks = [{'id': 0, 'position': [2, 2, 2], 'kind': 'motor',
               'facing': 'up', 'setting': setting, 'capacity_per_rpm': 16384.0}]
    if not direct or tick != 70:
        driver = 'shaft' if direct else 'large_cog' if fixture == 'large_to_small' else 'small_cog'
        blocks.append({'id': 1, 'position': [2, 3, 2], 'kind': driver, 'axis': 'y'})
    target = 'shaft' if direct else 'large_cog' if fixture == 'small_to_large' else 'small_cog'
    position = [2, 4, 2] if direct else [3, 3, 2] if fixture == 'small_to_small' else [3, 3, 3]
    blocks.append({'id': 2, 'position': position, 'kind': target, 'axis': 'y'})
    return {'case': f'{fixture}/{tick:03}', 'geometry': {'blocks': blocks}}


def expected_edges(name):
    if name == 'direct_stop_reverse/070':
        return []
    first = {'from': 0, 'to': 1, 'transmission': 'shaft'}
    if name.startswith('direct_stop_reverse/'):
        second = {'from': 1, 'to': 2, 'transmission': 'shaft'}
    elif name == 'small_to_small/020':
        second = {'from': 1, 'to': 2, 'transmission': 'equal_cogs'}
    elif name == 'large_to_small/020':
        second = {'from': 1, 'to': 2, 'transmission': 'large_to_small'}
    elif name == 'small_to_large/020':
        second = {'from': 2, 'to': 1, 'transmission': 'large_to_small'}
    else:
        raise ValueError('Unknown selected topology')
    return [first, second]


def validate_selected_reference(cases, reference):
    """Schema2 omits generated/setting/parent/member/basis facts; never fabricate them."""
    row_fields = {'schema', 'fixture', 'tick', 'phase', 'dimension', 'world_seed', 'context', 'machines'}
    machine_fields = {'position', 'block', 'properties', 'theoretical_rpm', 'effective_rpm', 'network'}
    for case in cases:
        name = case['case']
        row = reference['traces'][name]
        if (type(row) is not dict or set(row) != row_fields or type(row['schema']) is not int
                or row['schema'] != 2 or type(row['context']) is not dict or row['context'] != {}):
            raise ValueError(f'{name}: unexpected selected schema2 row fields')
        blocks = {block['id']: block for block in case['geometry']['blocks']}
        machines = row['machines']
        if type(machines) is not list or len(machines) != 3:
            raise ValueError(f'{name}: wrong selected observation slots')
        for index, machine in enumerate(machines):
            if index not in blocks:
                if (name != 'direct_stop_reverse/070' or index != 1 or type(machine) is not dict
                        or set(machine) != {'position', 'block', 'properties'}
                        or machine['position'] != [2, 3, 2] or machine['block'] != 'minecraft:air'
                        or type(machine['properties']) is not dict or machine['properties'] != {}):
                    raise ValueError(f'{name}: gap slot must be explicit air without kinetic fields')
                continue
            block = blocks[index]
            properties = {'facing': 'up'} if index == 0 else {'axis': 'y', 'waterlogged': 'false'}
            if (type(machine) is not dict or set(machine) != machine_fields
                    or machine['position'] != block['position'] or machine['block'] != BLOCK_IDS[block['kind']]
                    or type(machine['properties']) is not dict or machine['properties'] != properties
                    or any(not finite_number(machine[key]) for key in ('theoretical_rpm', 'effective_rpm'))):
                raise ValueError(f'{name}: wrong selected machine fields/properties at slot {index}')
            network = machine['network']
            powered = not (name == 'direct_stop_reverse/070' and index == 2)
            if (type(network) is not dict or set(network) != {'present', 'stress_su', 'capacity_su'}
                    or type(network['present']) is not bool or network['present'] is not powered
                    or any(not finite_number(network[key]) for key in ('stress_su', 'capacity_su'))):
                raise ValueError(f'{name}: wrong selected network fields/presence at slot {index}')


def validate_results(cases, results):
    if type(results) is not list or len(results) != len(cases):
        raise ValueError('Missing/extra vertical probe cases')
    for case, result in zip(cases, results):
        ids = [block['id'] for block in case['geometry']['blocks']]
        if (type(result) is not dict or set(result) != {'case', 'edges', 'rotations'}
                or type(result['case']) is not str or result['case'] != case['case']):
            raise ValueError('Wrong/duplicate vertical case identity or fields')
        if type(result['edges']) is not list:
            raise ValueError('Missing vertical edge array')
        pairs = []
        for edge in result['edges']:
            if (type(edge) is not dict or set(edge) != {'from', 'to', 'transmission'}
                    or any(type(edge[key]) is not int or edge[key] not in ids for key in ('from', 'to'))
                    or edge['from'] == edge['to'] or type(edge['transmission']) is not str
                    or edge['transmission'] not in TRANSMISSIONS):
                raise ValueError('Malformed vertical edge')
            pairs.append(tuple(sorted((edge['from'], edge['to']))))
        if pairs != sorted(set(pairs)):
            raise ValueError('Duplicate or noncanonical vertical edges')
        rotations = result['rotations']
        if (type(rotations) is not list
                or any(type(r) is not dict or set(r) != ROTATION_FIELDS for r in rotations)
                or any(type(r['id']) is not int for r in rotations)
                or [r['id'] for r in rotations] != ids):
            raise ValueError('Missing/duplicate/wrong PRESENT vertical node identity or fields')
        for rotation in rotations:
            if (any(not finite_number(rotation[key]) for key in ROTATION_FIELDS - {'id', 'state'})
                    or type(rotation['state']) is not str
                    or rotation['state'] not in {'running', 'unpowered', 'overstressed'}):
                raise ValueError('Malformed vertical rotation')


def difference(cases, results, reference):
    for case, result in zip(cases, results):
        name = case['case']
        expected = expected_edges(name)
        if result['edges'] != expected:
            return f'{name} edges: expected {expected!r} != probe {result["edges"]!r}'
        machines = reference['traces'][name]['machines']
        for rotation in result['rotations']:
            machine = machines[rotation['id']]
            expected_values = {key: machine[key] for key in ('theoretical_rpm', 'effective_rpm')}
            expected_values.update({key: machine['network'][key] for key in ('stress_su', 'capacity_su')})
            expected_values['state'] = ('unpowered' if machine['theoretical_rpm'] == 0 else
                                        'overstressed' if machine['effective_rpm'] == 0 else 'running')
            for key, value in expected_values.items():
                if rotation[key] != value:
                    return f'{name} node {rotation["id"]} {key}: reference {value!r} != probe {rotation[key]!r}'
    return None


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('reference', type=Path)
    parser.add_argument('--probe', type=Path, required=True)
    parser.add_argument('--output', type=Path, help='Save validated canonical output, including a differing result')
    parser.add_argument('--negative-control', choices=('axis', 'facing', 'edge-ratio', 'duplicate-case'),
                        help='Perturb the equal-cog fixture, reported equal ratio, or require actual duplicate-batch refusal')
    args = parser.parse_args()
    try:
        reference = load_run(args.reference)
        if (reference['environment']['fixture_set'] != 's0-edges-1'
                or reference['environment']['reference_revision'] != REFERENCE_REVISION):
            raise ValueError('Requires the complete expanded corpus at the pinned Create revision')
        cases = [case_input(fixture, tick) for fixture, tick in CHECKPOINTS]
        validate_selected_reference(cases, reference)
        submitted = copy.deepcopy(cases)
        if args.negative_control in ('axis', 'facing'):
            equal_case = next(case for case in submitted if case['case'] == 'small_to_small/020')
            blocks = equal_case['geometry']['blocks']
            if args.negative_control == 'axis':
                blocks[2]['axis'] = 'x'
            else:
                blocks[0]['facing'] = 'east'
        if args.negative_control == 'duplicate-case':
            submitted.append(copy.deepcopy(submitted[0]))
        process = subprocess.run([str(args.probe.resolve())], input=json.dumps(submitted), text=True,
                                 capture_output=True, timeout=30)
        if args.negative_control == 'duplicate-case':
            if (process.returncode == 2 and not process.stdout.strip()
                    and 'Duplicate geometry case name' in process.stderr):
                print('Actual probe refused duplicate valid vertical cases: exit 2, no partial output.')
                return 2
            print(f'Duplicate vertical batch did not receive required actual probe refusal: '
                  f'exit {process.returncode}, stdout {process.stdout!r}, stderr {process.stderr!r}')
            return 1
        if process.returncode:
            raise ValueError(f'Probe failed ({process.returncode}): {process.stderr.strip()}')
        results = decode(process.stdout)
        if args.negative_control == 'edge-ratio':
            # Software output wrapper: the actual native/Wasm edge implementation is unchanged.
            validate_results(cases, results)
            next(edge for result in results for edge in result['edges']
                 if edge['transmission'] == 'equal_cogs')['transmission'] = 'shaft'
        validate_results(cases, results)
        if args.output:
            args.output.write_text(json.dumps(results, sort_keys=True, indent=2, allow_nan=False) + '\n')
        mismatch = difference(cases, results, reference)
        if mismatch:
            print(mismatch)
            return 1
        print('Equal vertical static geometry and settled states: 6 checkpoints, 17 PRESENT machines; '
              'selected schema2 snapshots, not timing/parents/network-equivalence or live conformance.')
        return 0
    except (OSError, ValueError, KeyError, TypeError, StopIteration, subprocess.TimeoutExpired) as error:
        print(f'Invalid vertical geometry comparison: {error}')
        return 1 if args.negative_control == 'duplicate-case' else 2


if __name__ == '__main__':
    raise SystemExit(main())
