#!/usr/bin/env python3
"""Compare three declared event streams with nine settled arbitration projections."""
import argparse
import copy
import json
from pathlib import Path
import subprocess
from compare_reference import decode, finite_number, load_run

FIXTURES = ('arbitration_unequal_merge', 'arbitration_dominant_removal', 'arbitration_subordinate_reversal')
LABELS = ('initial', 'disconnected', 'merged', 'after')
TICKS = {'disconnected': 20, 'merged': 40, 'after': 80}
POSITIONS = ([2, 2, 2], [2, 3, 2], [2, 4, 2])
REVISION = '7ff2760e4924d35d4bc2e22c4b19923f0178cd16'
PRESENT_FIELDS = {'id', 'present', 'kind', 'generated_rpm', 'theoretical_rpm', 'effective_rpm',
                  'source_parent', 'network_present', 'network_members', 'capacity_su', 'stress_su', 'flicker'}


def case_input(fixture):
    if fixture not in FIXTURES:
        raise ValueError('Unknown declared arbitration selection')
    events = [{'op': 'place_generator', 'node': 0, 'generated': 16},
              {'op': 'place_generator', 'node': 2, 'generated': -16},
              {'op': 'settle', 'label': 'initial'},
              {'op': 'set_generated', 'node': 0, 'generated': 16},
              {'op': 'set_generated', 'node': 2, 'generated': 32},
              {'op': 'settle', 'label': 'disconnected'}, {'op': 'place_join'},
              {'op': 'settle', 'label': 'merged'}]
    if fixture == 'arbitration_dominant_removal':
        events.append({'op': 'destroy_generator', 'node': 2})
    elif fixture == 'arbitration_subordinate_reversal':
        events.append({'op': 'set_generated', 'node': 0, 'generated': -16})
    events.append({'op': 'settle', 'label': 'after'})
    return {'case': fixture, 'arbitration_trial': {'spec': {
        'lower': 0, 'join': 1, 'upper': 2, 'capacity_per_rpm': 16384,
        'join_neighbor_order': [0, 2], 'be_order': [0, 1, 2]}, 'events': events}}


def validate_reference(reference):
    for fixture in FIXTURES:
        for label, tick in TICKS.items():
            row = reference['traces'][f'{fixture}/{tick:03}']
            if set(row) != {'schema', 'fixture', 'tick', 'phase', 'dimension', 'world_seed', 'context', 'machines'}:
                raise ValueError('Extra/missing selected reference fields')
            for index, machine in enumerate(row['machines']):
                absent = ((label == 'disconnected' and index == 1)
                          or (label == 'after' and fixture == 'arbitration_dominant_removal' and index == 2)
                          or (label == 'after' and fixture == 'arbitration_subordinate_reversal' and index == 0))
                if machine['position'] != POSITIONS[index]:
                    raise ValueError('Wrong selected position')
                if absent:
                    if (set(machine) != {'block', 'position', 'properties'}
                            or machine['block'] != 'minecraft:air' or machine['properties'] != {}):
                        raise ValueError('Expected explicit selected air shape')
                else:
                    properties = {'axis': 'y', 'waterlogged': 'false'} if index == 1 else {'facing': 'up' if index == 0 else 'down'}
                    block = 'create:shaft' if index == 1 else 'create:creative_motor'
                    if machine['block'] != block or machine['properties'] != properties:
                        raise ValueError('Wrong selected dry block properties')
                    if index != 1:
                        generated = 16 if index == 0 else 32
                        if machine['generated_rpm'] != generated or machine['setting'] != (generated if index == 0 else -generated):
                            raise ValueError('Reference generator input contradicts authored schedule')


def validate_results(cases, results):
    if type(results) is not list or len(results) != len(cases):
        raise ValueError('Missing/extra trial cases')
    for case, result in zip(cases, results):
        if (type(result) is not dict or set(result) != {'case', 'checkpoints'}
                or type(result['case']) is not str or result['case'] != case['case']
                or type(result['checkpoints']) is not list or len(result['checkpoints']) != 4):
            raise ValueError('Wrong case/checkpoint identity or fields')
        prior_effects = []
        for label, checkpoint in zip(LABELS, result['checkpoints']):
            if (type(checkpoint) is not dict or set(checkpoint) != {'label', 'nodes', 'effects'}
                    or type(checkpoint['label']) is not str or checkpoint['label'] != label
                    or type(checkpoint['nodes']) is not list or len(checkpoint['nodes']) != 3):
                raise ValueError('Missing/duplicate trial checkpoint or node array')
            nodes = checkpoint['nodes']
            if any(type(n) is not dict or type(n.get('id')) is not int for n in nodes) or [n['id'] for n in nodes] != [0, 1, 2]:
                raise ValueError('Wrong/duplicate trial slot identity')
            for index, node in enumerate(nodes):
                if type(node.get('present')) is not bool:
                    raise ValueError('Missing typed presence')
                if not node['present']:
                    if set(node) != {'id', 'present'}:
                        raise ValueError('Absent node carries kinetic fields')
                    continue
                if (set(node) != PRESENT_FIELDS or node['kind'] != ('shaft' if index == 1 else 'generator')
                        or any(not finite_number(node[key]) for key in
                               ('generated_rpm', 'theoretical_rpm', 'effective_rpm', 'capacity_su', 'stress_su'))
                        or type(node['flicker']) is not int or node['flicker'] < 0
                        or type(node['network_present']) is not bool):
                    raise ValueError('Malformed present trial node')
                parent = node['source_parent']
                if (parent is not None and (type(parent) is not int or parent not in range(3)
                        or parent == index or (parent != 1 and index != 1) or not nodes[parent]['present'])):
                    raise ValueError('Unknown/nonadjacent/absent trial parent')
                members = node['network_members']
                if (type(members) is not list or any(type(n) is not int or n not in range(3) for n in members)
                        or members != sorted(set(members))
                        or (index not in members if node['network_present'] else members != [])
                        or any(not nodes[n]['present'] for n in members)):
                    raise ValueError('Malformed trial network membership')
            for index, node in enumerate(nodes):
                if not node['present']:
                    continue
                for member in node['network_members']:
                    peer = nodes[member]
                    if not peer['network_present'] or peer['network_members'] != node['network_members']:
                        raise ValueError('Broken reciprocal trial network group')
                cursor = index
                seen = set()
                while nodes[cursor]['source_parent'] is not None:
                    if cursor in seen:
                        raise ValueError('Cyclic trial parent forest')
                    seen.add(cursor)
                    parent = nodes[cursor]['source_parent']
                    if nodes[parent]['network_members'] != node['network_members']:
                        raise ValueError('Parent outside trial network')
                    cursor = parent
            effects = checkpoint['effects']
            if (type(effects) is not list or len(effects) > 16 or effects[:len(prior_effects)] != prior_effects):
                raise ValueError('Broken cumulative effect ledger')
            for effect in effects:
                if (type(effect) is not dict or set(effect) != {'node', 'kind', 'with_drops'}
                        or type(effect['node']) is not int or effect['node'] not in (0, 2)
                        or effect['kind'] != 'generator' or type(effect['with_drops']) is not bool
                        or effect['with_drops'] is not True):
                    raise ValueError('Malformed domain destruction effect')
            prior_effects = effects


def difference(cases, results, reference):
    for case, result in zip(cases, results):
        fixture = case['case']
        for checkpoint in result['checkpoints'][1:]:
            label = checkpoint['label']
            row = reference['traces'][f'{fixture}/{TICKS[label]:03}']
            for node, machine in zip(checkpoint['nodes'], row['machines']):
                present = machine['block'] != 'minecraft:air'
                if node['present'] != present:
                    return f'{fixture}/{label} node {node["id"]} presence: reference {present} != trial {node["present"]}'
                if not present:
                    continue
                expected = {key: machine[key] for key in ('generated_rpm', 'theoretical_rpm', 'effective_rpm', 'flicker')}
                expected.update({key: machine['network'][key] for key in ('capacity_su', 'stress_su')})
                expected['network_present'] = machine['network']['present']
                expected['source_parent'] = POSITIONS.index(machine['source_parent']) if machine['source_parent'] else None
                expected['network_members'] = sorted(POSITIONS.index(position) for position in machine['network_members'])
                for key, value in expected.items():
                    if node[key] != value:
                        return f'{fixture}/{label} node {node["id"]} {key}: reference {value!r} != trial {node[key]!r}'
            drops = {}
            for effect in checkpoint['effects']:
                drops['create:creative_motor'] = drops.get('create:creative_motor', 0) + 1
            if drops != row['context']['drops']:
                return f'{fixture}/{label} domain destruction projection {drops!r} != reference live drops {row["context"]["drops"]!r}'
    return None


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('reference', type=Path)
    parser.add_argument('--probe', type=Path, required=True)
    parser.add_argument('--output', type=Path)
    parser.add_argument('--control', choices=('same-sign', 'removal-target', 'omit-join', 'duplicate-node', 'repeated-labels'))
    args = parser.parse_args()
    try:
        reference = load_run(args.reference)
        if reference['environment']['fixture_set'] != 'p02-arbitration-1' or reference['environment']['reference_revision'] != REVISION:
            raise ValueError('Requires the complete pinned arbitration corpus')
        validate_reference(reference)
        cases = [case_input(fixture) for fixture in FIXTURES]
        submitted = copy.deepcopy(cases)
        if args.control == 'same-sign':
            submitted[2]['arbitration_trial']['events'][-2]['generated'] = 16
        elif args.control == 'removal-target':
            submitted[1]['arbitration_trial']['events'][-2]['node'] = 0
        elif args.control == 'omit-join':
            for case in submitted:
                case['arbitration_trial']['events'] = [event for event in case['arbitration_trial']['events'] if event['op'] != 'place_join']
        elif args.control == 'duplicate-node':
            submitted[0]['arbitration_trial']['spec']['upper'] = 0
        elif args.control == 'repeated-labels':
            for case in submitted:
                case['case'] = 'repeated metadata'
        process = subprocess.run([str(args.probe.resolve())], input=json.dumps(submitted), text=True, capture_output=True, timeout=30)
        if args.control == 'duplicate-node':
            if process.returncode == 2 and not process.stdout.strip() and 'DuplicateNode' in process.stderr:
                print('Actual probe refused duplicate trial node identity: exit2, no partial output.')
                return 2
            print(f'Required actual duplicate-node refusal missing: exit{process.returncode}, stdout={process.stdout!r}, stderr={process.stderr!r}')
            return 1
        if process.returncode:
            raise ValueError(f'Probe failed ({process.returncode}): {process.stderr.strip()}')
        results = decode(process.stdout)
        validate_results(submitted, results)
        if args.output:
            args.output.write_text(json.dumps(results, sort_keys=True, indent=2, allow_nan=False) + '\n')
        mismatch = difference(cases, results, reference)
        if mismatch:
            print(mismatch)
            return 1
        print('Equal declared-event settled projections: 3 streams, 9 rows, 22 PRESENT machines; '
              'domain effects only, not Java scheduling, native items or live arbitration.')
        return 0
    except (OSError, ValueError, KeyError, TypeError, OverflowError, subprocess.TimeoutExpired) as error:
        print(f'Invalid declared arbitration comparison: {error}')
        return 1 if args.control == 'duplicate-node' else 2


if __name__ == '__main__':
    raise SystemExit(main())
