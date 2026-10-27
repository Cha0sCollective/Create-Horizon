#!/usr/bin/env python3
"""Compare settled P01 checkpoints to the small native kernel; not full trace timing."""
import argparse
import json
from pathlib import Path
import subprocess
from compare_reference import decode, finite_number, load_run

CHECKPOINTS = {'direct_stop_reverse': (10, 30, 70), 'small_to_small': (20,),
               'large_to_small': (20,), 'small_to_large': (20,),
               'finite_stress': (20, 50, 70, 90, 120), 'motor_value_settings': (10, 20, 30, 40, 60)}


def case_input(fixture, tick, row):
    nodes = [{'id': index, 'impact_per_rpm': 4.0 if m['block'] == 'create:millstone' else 0.0,
              'source': None} for index, m in enumerate(row['machines']) if m['block'] != 'minecraft:air']
    if fixture == 'finite_stress':
        source = {'rpm': -32.0 if tick < 120 else 0.0, 'capacity_per_rpm': 8.0}
        links = [(0, 1, 'shaft'), (1, 2, 'shaft'), (2, 3, 'equal_cogs'), (2, 4, 'equal_cogs'),
                 (2, 5, 'shaft'), (3, 6, 'shaft'), (4, 7, 'shaft')]
    else:
        source = {'rpm': -16.0 if fixture == 'direct_stop_reverse' and tick >= 30 else 16.0,
                  'capacity_per_rpm': 16384.0}
        if fixture == 'motor_value_settings':
            row_number, magnitude = {10: (1, 16), 20: (1, 0), 30: (0, 0), 40: (0, 256), 60: (1, 256)}[tick]
            source = {'row': row_number, 'magnitude': magnitude, 'capacity_per_rpm': 16384.0}
        ratio = {'small_to_small': 'equal_cogs', 'large_to_small': 'large_to_small',
                 'small_to_large': 'small_to_large'}.get(fixture, 'shaft')
        links = [(0, 1, 'shaft'), (1, 2, ratio)]
    nodes[0]['source'] = source
    ids = {n['id'] for n in nodes}
    edges = [{'from': a, 'to': b, 'transmission': ratio} for a, b, ratio in links if a in ids and b in ids]
    return {'case': f'{fixture}/{tick:03}', 'nodes': nodes, 'edges': edges}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('reference', type=Path)
    parser.add_argument('--probe', type=Path, required=True)
    parser.add_argument('--negative-control', action='store_true', help='Deliberately map large-to-small as equal cogs')
    args = parser.parse_args()
    try:
        reference = load_run(args.reference)
        if reference['environment']['fixture_set'] != 's0-edges-1':
            raise ValueError('Requires the complete expanded reference corpus')
        cases = [case_input(f, t, reference['traces'][f'{f}/{t:03}']) for f, ticks in CHECKPOINTS.items() for t in ticks]
        if args.negative_control:
            for case in cases:
                for edge in case['edges']:
                    if edge['transmission'] == 'large_to_small':
                        edge['transmission'] = 'equal_cogs'
        process = subprocess.run([str(args.probe.resolve())], input=json.dumps(cases), text=True,
                                 capture_output=True, timeout=30)
        if process.returncode:
            raise ValueError(f'Probe failed ({process.returncode}): {process.stderr.strip()}')
        results = decode(process.stdout)
        if not isinstance(results, list) or len(results) != len(cases):
            raise ValueError('Missing probe results')
        observed_nodes = 0
        for case, result in zip(cases, results):
            if (type(result) is not dict or set(result) != {'case', 'rotations'}
                    or result['case'] != case['case'] or type(result['rotations']) is not list
                    or any(type(r) is not dict or type(r.get('id')) is not int for r in result['rotations'])
                    or [r['id'] for r in result['rotations']] != [n['id'] for n in case['nodes']]):
                raise ValueError('Wrong/duplicate probe case or node identity')
            machines = reference['traces'][case['case']]['machines']
            for rotation in result['rotations']:
                if (set(rotation) != {'id', 'theoretical_rpm', 'effective_rpm', 'stress_su', 'capacity_su', 'state'}
                        or any(not finite_number(rotation[k]) for k in ('theoretical_rpm', 'effective_rpm', 'stress_su', 'capacity_su'))
                        or type(rotation['state']) is not str):
                    raise ValueError('Malformed probe rotation')
                machine = machines[rotation['id']]
                expected = {key: machine[key] for key in ('theoretical_rpm', 'effective_rpm')}
                expected.update({key: machine['network'][key] for key in ('stress_su', 'capacity_su')})
                expected['state'] = ('unpowered' if machine['theoretical_rpm'] == 0 else
                                     'overstressed' if machine['effective_rpm'] == 0 else 'running')
                for key, value in expected.items():
                    if rotation[key] != value:
                        print(f'{case["case"]} node {rotation["id"]} {key}: reference {value!r} != native {rotation[key]!r}')
                        return 1
                observed_nodes += 1
        print(f'Equal settled states: {len(cases)} checkpoints, {observed_nodes} nodes; not propagation timing, world connectivity or live-host conformance.')
        return 0
    except (OSError, ValueError, KeyError, TypeError, subprocess.TimeoutExpired) as error:
        print(f'Invalid core comparison: {error}')
        return 2


if __name__ == '__main__':
    raise SystemExit(main())
