#!/usr/bin/env python3
"""Compare only the twelve accepted horizontal settled checkpoints to static geometry."""
import argparse
import copy
import json
from pathlib import Path
import subprocess
from compare_reference import decode, finite_number, load_run

FIXTURES = ('horizontal_x_connected', 'horizontal_x_mismatch',
            'horizontal_z_connected', 'horizontal_z_mismatch')
TICKS = (20, 50, 60)
REFERENCE_REVISION = '7ff2760e4924d35d4bc2e22c4b19923f0178cd16'
ROTATION_FIELDS = {'id', 'theoretical_rpm', 'effective_rpm', 'stress_su', 'capacity_su', 'state'}
TRANSMISSIONS = {'shaft', 'equal_cogs', 'large_to_small', 'small_to_large'}


def case_input(fixture, tick):
    """Author inputs from the frozen fixture schedule, never from measured outputs."""
    x_axis = fixture.startswith('horizontal_x_')
    connected = fixture.endswith('_connected')
    axis = 'x' if x_axis else 'z'
    generated = 16 if tick == 20 else -16
    positions = ([(2, 3, 3), (3, 3, 3), (4, 3, 3), (4, 4, 4)] if x_axis else
                 [(3, 3, 4), (3, 3, 3), (3, 3, 2), (4, 4, 2)])
    blocks = [{'id': 0, 'position': list(positions[0]), 'kind': 'motor',
               'facing': 'east' if x_axis else 'north',
               'setting': generated if x_axis else -generated, 'capacity_per_rpm': 16384.0}]
    for index, kind in ((1, 'shaft'), (2, 'large_cog'), (3, 'small_cog')):
        block_axis = axis if index != 3 or connected else ('z' if x_axis else 'x')
        blocks.append({'id': index, 'position': list(positions[index]), 'kind': kind, 'axis': block_axis})
    return {'case': f'{fixture}/{tick:03}', 'geometry': {'blocks': blocks}}


def expected_edges(case_name):
    edges = [{'from': 0, 'to': 1, 'transmission': 'shaft'},
             {'from': 1, 'to': 2, 'transmission': 'shaft'}]
    if case_name.split('/')[0].endswith('_connected'):
        edges.append({'from': 2, 'to': 3, 'transmission': 'large_to_small'})
    return edges


def validate_results(cases, results):
    """Malformed evidence refuses before any differing-state comparison."""
    if type(results) is not list or len(results) != len(cases):
        raise ValueError('Missing/extra probe cases')
    for case, result in zip(cases, results):
        if (type(result) is not dict or set(result) != {'case', 'edges', 'rotations'}
                or type(result['case']) is not str or result['case'] != case['case']):
            raise ValueError('Wrong/duplicate probe case identity or fields')
        edges = result['edges']
        if type(edges) is not list:
            raise ValueError('Missing edge array')
        pairs = []
        for edge in edges:
            if (type(edge) is not dict or set(edge) != {'from', 'to', 'transmission'}
                    or any(type(edge[k]) is not int or edge[k] not in range(4) for k in ('from', 'to'))
                    or edge['from'] == edge['to'] or type(edge['transmission']) is not str
                    or edge['transmission'] not in TRANSMISSIONS):
                raise ValueError('Malformed probe edge')
            pairs.append(tuple(sorted((edge['from'], edge['to']))))
        if pairs != sorted(set(pairs)):
            raise ValueError('Duplicate or noncanonical edge order')
        rotations = result['rotations']
        if (type(rotations) is not list
                or any(type(r) is not dict or set(r) != ROTATION_FIELDS for r in rotations)
                or any(type(r['id']) is not int for r in rotations)
                or [r['id'] for r in rotations] != [0, 1, 2, 3]):
            raise ValueError('Missing/duplicate/wrong rotation node identity or fields')
        for rotation in rotations:
            if (any(not finite_number(rotation[k]) for k in ROTATION_FIELDS - {'id', 'state'})
                    or type(rotation['state']) is not str
                    or rotation['state'] not in {'running', 'unpowered', 'overstressed'}):
                raise ValueError('Malformed probe rotation')


def difference(cases, results, reference):
    for case, result in zip(cases, results):
        name = case['case']
        expected = expected_edges(name)
        if result['edges'] != expected:
            return f'{name} edges: expected {expected!r} != probe {result["edges"]!r}'
        row = reference['traces'][name]
        machines = row['machines']
        blocks = case['geometry']['blocks']
        converted = blocks[0]['setting'] * (1 if blocks[0]['facing'] == 'east' else -1)
        if machines[0]['setting'] != blocks[0]['setting'] or machines[0]['generated_rpm'] != converted:
            raise ValueError(f'{name}: reference motor facts contradict authored schedule')
        if result['rotations'][0]['theoretical_rpm'] != machines[0]['generated_rpm']:
            return f'{name} motor generated RPM: reference {machines[0]["generated_rpm"]!r} != probe {result["rotations"][0]["theoretical_rpm"]!r}'
        if any(machine['generated_rpm'] != 0 for machine in machines[1:]):
            raise ValueError(f'{name}: passive generated RPM contradicts authored facts')
        connected = name.split('/')[0].endswith('_connected')
        # These are independent literal fixture groups, not Java IDs or a parent tree.
        groups = ([0, 1, 2, 3],) if connected else ([0, 1, 2], [3])
        for group in groups:
            powered = 0 in group
            members = [blocks[index]['position'] for index in group] if powered else []
            for index in group:
                if (sorted(machines[index]['network_members']) != sorted(members)
                        or machines[index]['network']['present'] is not powered):
                    raise ValueError(f'{name}: reference network membership contradicts authored layout')
        # Edge equality above establishes the native component relation for this exact fixture.
        for rotation, machine in zip(result['rotations'], machines):
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
    parser.add_argument('--output', type=Path, help='Save validated canonical probe output, including a differing result')
    parser.add_argument('--negative-control', choices=('axis', 'facing', 'edge-ratio', 'duplicate-case'),
                        help='Change geometry, wrap a reported ratio, or require actual duplicate-batch refusal')
    args = parser.parse_args()
    try:
        reference = load_run(args.reference)
        if reference['environment']['fixture_set'] != 'p02-horizontal-1':
            raise ValueError('Requires the complete accepted horizontal fixture set')
        if reference['environment']['reference_revision'] != REFERENCE_REVISION:
            raise ValueError('Requires the pinned Create reference revision')
        cases = [case_input(fixture, tick) for fixture in FIXTURES for tick in TICKS]
        submitted = copy.deepcopy(cases)
        if args.negative_control in ('axis', 'facing'):
            for case in submitted:
                if not case['case'].split('/')[0].endswith('_connected'):
                    continue
                blocks = case['geometry']['blocks']
                if args.negative_control == 'axis':
                    blocks[3]['axis'] = 'z' if blocks[3]['axis'] == 'x' else 'x'
                else:
                    blocks[0]['facing'] = 'north' if blocks[0]['facing'] == 'east' else 'east'
        if args.negative_control == 'duplicate-case':
            submitted.append(copy.deepcopy(submitted[0]))
        process = subprocess.run([str(args.probe.resolve())], input=json.dumps(submitted), text=True,
                                 capture_output=True, timeout=30)
        if args.negative_control == 'duplicate-case':
            if (process.returncode == 2 and not process.stdout.strip()
                    and 'Duplicate geometry case name' in process.stderr):
                print('Actual probe refused duplicate valid geometry cases: exit 2, no partial output.')
                return 2
            print(f'Duplicate geometry batch did not receive the required actual probe refusal: '
                  f'exit {process.returncode}, stdout {process.stdout!r}, stderr {process.stderr!r}')
            return 1
        if process.returncode:
            raise ValueError(f'Probe failed ({process.returncode}): {process.stderr.strip()}')
        results = decode(process.stdout)
        if args.negative_control == 'edge-ratio':
            # Software output wrapper only: the actual native/Wasm implementation is unchanged.
            validate_results(cases, results)
            next(edge for result in results for edge in result['edges']
                 if edge['transmission'] == 'large_to_small')['transmission'] = 'equal_cogs'
        validate_results(cases, results)
        if args.output:
            args.output.write_text(json.dumps(results, sort_keys=True, indent=2, allow_nan=False) + '\n')
        mismatch = difference(cases, results, reference)
        if mismatch:
            print(mismatch)
            return 1
        print('Equal static geometry and settled states: 12 checkpoints, 48 machines; '
              'only captured layouts, not arbitrary-axis or propagation/live-host conformance.')
        return 0
    except (OSError, ValueError, KeyError, TypeError, StopIteration, subprocess.TimeoutExpired) as error:
        print(f'Invalid geometry comparison: {error}')
        # A control setup/timeout/decoder error is not proof of actual probe refusal.
        return 1 if args.negative_control == 'duplicate-case' else 2


if __name__ == '__main__':
    raise SystemExit(main())
