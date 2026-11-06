"""Finite FLUIDS-005 water-line cases; only existing horizon-ctl actions."""
from pathlib import Path
import argparse
import json
import re
import subprocess
import sys
import time
import traceback
import uuid

p = argparse.ArgumentParser()
p.add_argument('session', type=Path)
p.add_argument('phase', choices=['before-cold', 'after-cold'])
a = p.parse_args()
s = json.loads(a.session.read_text())
out = a.session.parent
steps = []

def call(*words, role='actor'):
    command = [sys.executable, str(Path(__file__).with_name('call.py')),
               str(a.session), role, *map(str, words)]
    result = subprocess.run(command, capture_output=True, text=True, timeout=210)
    if result.returncode:
        raise RuntimeError(f'{role} {words}: {result.stdout} {result.stderr}')
    value = json.loads(result.stdout)
    steps.append({'role': role, 'words': words, 'value': value})
    return value

def command(value):
    return call('command', value)

def fresh_chat(query, match):
    marker = 'FLUIDS005_' + uuid.uuid4().hex[:12]
    command('say ' + marker)
    command(query)
    for _ in range(20):
        lines = call('chat-log', 60).get('chat', [])
        markers = [i for i, line in enumerate(lines) if marker in line]
        if markers:
            fresh = lines[:markers[0]]  # horizon chat-log is newest first
            hits = [line for line in fresh if match(line)]
            if hits:
                return hits[0]
        call('wait', 1)
    raise AssertionError(f'No fresh reply to {query!r} after {marker}')

def data(x):
    prefix = f'{x}, 65, 20 has the following block data:'
    return fresh_chat(f'data get block {x} 65 20', lambda line: line.startswith(prefix))

def single_reply(query):
    # The pinned host executes commands in order. Read one fresh server reply
    # after our marker; a control dispatch ACK is never the queried value.
    marker = 'FLUIDS005_REPLY_' + uuid.uuid4().hex[:12]
    command('say ' + marker)
    command(query)
    for _ in range(20):
        lines = call('chat-log', 60).get('chat', [])
        markers = [i for i, line in enumerate(lines) if marker in line]
        if markers:
            fresh = lines[:markers[0]]
            if fresh:
                assert len(fresh) == 1, (marker, fresh)
                return fresh[0]
        call('wait', 1)
    raise AssertionError(f'No fresh reply to {query!r} after {marker}')

def game_time():
    # Prepared 26.3 en_us: commands.time.query.gametime =
    # "The game time is %s tick(s)". The held HOLD captured that exact shape.
    line = single_reply('time query gametime')
    match = re.fullmatch(r'The game time is (\d+) tick\(s\)', line)
    assert match, line
    return int(match.group(1))

def verify_spawn_mobs_false():
    # Prepared 26.3 en_us: "Game rule %s is currently set to %s".
    line = single_reply('gamerule spawn_mobs')
    assert line == 'Game rule spawn_mobs is currently set to false', line

def disable_mob_spawning():
    # Current 26.3 name; doMobSpawning is only a data-fixer legacy alias.
    line = single_reply('gamerule spawn_mobs false')
    assert line == 'Game rule spawn_mobs is now set to false', line
    verify_spawn_mobs_false()

def number(x, key):
    line = data(x)
    match = re.search(r'\b' + re.escape(key) + r':\s*(-?\d+(?:\.\d+)?)', line)
    if not match:
        raise AssertionError(f'No numeric {key}: {line}')
    return float(match.group(1))

def content_amount(content):
    if content == '{}':
        return 0
    match = re.fullmatch(r'\{Fluid: \{amount: (\d+), id: "minecraft:water"\}\}', content)
    assert match, content
    return int(match.group(1))

def contents(section, expected):
    values = re.findall(r'TankContent: (\{\}|\{Fluid: \{[^{}]+\}\})', section)
    assert len(values) == expected, (section, values)
    return [content_amount(value) for value in values]

def amounts(total):
    tank_nbt, basin_nbt = data(20), data(23)
    tank_values = contents(tank_nbt, 1)
    input_section = basin_nbt.split('InputTanks: [', 1)[1].split('], OutputTanks:', 1)[0]
    output_section = basin_nbt.split('OutputTanks: [', 1)[1].split('], Overflow:', 1)[0]
    input_values, output_values = contents(input_section, 2), contents(output_section, 2)
    assert input_values[1] == 0 and output_values == [0, 0], basin_nbt
    assert 'InputItems: {Size: 9, Items: []}' in basin_nbt, basin_nbt
    assert 'OutputItems: {Size: 9, Items: []}' in basin_nbt, basin_nbt
    tank, basin = tank_values[0], input_values[0]
    state = {'tank': tank, 'basin': basin, 'total': tank + basin,
             'tank_nbt': tank_nbt, 'basin_nbt': basin_nbt}
    steps.append({'amounts': state})
    assert state['total'] == total, state
    assert 0 <= tank <= 8000 and 0 <= basin <= 1000, state
    return state

def setblock(x, z, state):
    call('setblock', x, 65, z, state)

def tick(count):
    before = game_time()
    command(f'tick step {count}')
    deadline = time.monotonic() + 180
    while time.monotonic() < deadline:
        current = game_time()
        if current == before + count:
            return
        assert current < before + count, (before, count, current)
        call('wait', 1)
    raise TimeoutError(f'authoritative tick step {count}: {before} -> {current}')

def look_peer():
    if s['mode'] != 'dedicated':
        return
    command('tp TeamPeer 26.5 66 23.5')
    call('wait-ready', 120, role='peer')
    call('look-at', 23.5, 65.6, 20.5, role='peer')
    call('wait', 8, role='peer')

def empty_inventory(role='actor'):
    for _ in range(40):
        value = call('inventory', role=role)
        if value.get('counts') == {}:
            return
        call('wait', 1, role=role)
    raise AssertionError(f'{role} inventory is not empty: {value}')

def shot(label):
    if s['mode'] != 'dedicated':
        return
    call('wait', 12, role='peer')
    path = out / f'{a.phase}-{label}-peer-{s["index"]:02d}.png'
    call('screenshot', path, role='peer')
    assert path.is_file() and path.stat().st_size > 0, path

def same(before, after):
    assert (before['tank'], before['basin']) == (after['tank'], after['basin']), (before, after)

def moved(before, after, expected):
    assert before['tank'] - after['tank'] == expected, (before, after)
    assert after['basin'] - before['basin'] == expected, (before, after)

def close():
    assert call('save').get('ok') is True
    if s['mode'] == 'dedicated':
        call('quit', role='peer')
    call('quit')
    (out / f'stop-{s["index"]:02d}.request').write_text('normal FLUIDS-005 close\n')

def before_cold():
    assert s['index'] == 0
    call('wait-ready', 120)
    command('difficulty peaceful')
    disable_mob_spawning()
    command('gamemode creative @a')
    command('clear @a')
    empty_inventory()
    if s['mode'] == 'dedicated':
        empty_inventory('peer')
    command('tp @s 25.5 67 23.5')
    call('wait-ready', 120)
    if s['mode'] == 'dedicated':
        call('wait-ready', 120, role='peer')
    command('tick freeze')
    call('fill', 17, 65, 17, 29, 72, 25, 'minecraft:air')
    call('fill', 17, 64, 17, 29, 64, 25, 'minecraft:smooth_stone')
    command('kill @e[type=!minecraft:player]')
    # Fixture construction and water seed are setup, not placement/fill claims.
    setblock(20, 20, 'create:fluid_tank')
    setblock(21, 20, 'create:mechanical_pump[facing=east]')
    setblock(22, 20, 'create:fluid_pipe[axis=x]')
    setblock(23, 20, 'create:basin[facing=down]')
    setblock(21, 21, 'create:cogwheel[axis=x]')
    setblock(22, 21, 'create:creative_motor[facing=west]')
    command('data merge block 22 65 21 {ScrollValue:0}')
    command('data merge block 20 65 20 {TankContent:{Fluid:{id:"minecraft:water",amount:1000}}}')
    tick(2)
    for x, state in [(20, 'create:fluid_tank'), (21, 'create:mechanical_pump'),
                     (22, 'create:fluid_pipe'), (23, 'create:basin')]:
        assert call('block', x, 65, 20)['block'].startswith(state), (x, state)
    assert number(21, 'Speed') == 0
    initial = amounts(1000)
    assert (initial['tank'], initial['basin']) == (1000, 0), initial
    look_peer()
    shot('basin-empty')
    tick(16)
    same(initial, amounts(1000))

    command('data merge block 22 65 21 {ScrollValue:16}')
    tick(2)
    positive = number(21, 'Speed')
    assert abs(positive) == 16, positive
    start = amounts(1000)
    tick(10)
    after_rate = amounts(1000)
    moved(start, after_rate, 80)

    # One extra adjacent Pipe is a forbidden branch of this exact line.
    setblock(22, 19, 'create:fluid_pipe[axis=z]')
    tick(10)
    same(after_rate, amounts(1000))
    setblock(22, 19, 'minecraft:air')
    before_resume = amounts(1000)
    tick(10)
    after_resume = amounts(1000)
    moved(before_resume, after_resume, 80)

    command('data merge block 22 65 21 {ScrollValue:-16}')
    tick(2)
    reversed_speed = number(21, 'Speed')
    assert reversed_speed == -positive, (positive, reversed_speed)
    before_reverse = amounts(1000)
    tick(10)
    moved(before_reverse, amounts(1000), 80)

    command('data merge block 22 65 21 {ScrollValue:0}')
    tick(2)
    assert number(21, 'Speed') == 0
    partial = amounts(1000)
    assert 0 < partial['tank'] < 1000 and 0 < partial['basin'] < 1000, partial
    command('tick unfreeze')
    call('wait', 40)
    same(partial, amounts(1000))
    (out / 'pre-cold-amounts.json').write_text(json.dumps(partial, indent=2) + '\n')
    close()

def after_cold():
    assert s['index'] == 1
    assert (out / 'exit-00.json').is_file(), 'normal first exit receipt missing'
    exits = json.loads((out / 'exit-00.json').read_text())['codes']
    assert exits and all(code == 0 for code in exits.values()), exits
    call('wait-ready', 120)
    verify_spawn_mobs_false()
    look_peer()
    prior = json.loads((out / 'pre-cold-amounts.json').read_text())
    empty_inventory()
    if s['mode'] == 'dedicated':
        empty_inventory('peer')
    current = amounts(1000)
    same(prior, current)
    assert number(21, 'Speed') == 0
    command('tick freeze')
    command('data merge block 22 65 21 {ScrollValue:16}')
    tick(2)
    assert abs(number(21, 'Speed')) == 16
    tick(150)
    full = amounts(1000)
    assert (full['tank'], full['basin']) == (0, 1000), full
    shot('basin-filled')

    # Explicit second fixture: source refill makes a separate 2,000 mB census.
    command('data merge block 20 65 20 {TankContent:{Fluid:{id:"minecraft:water",amount:1000}}}')
    full_input = amounts(2000)
    assert (full_input['tank'], full_input['basin']) == (1000, 1000), full_input
    tick(20)
    same(full_input, amounts(2000))
    command('data merge block 22 65 21 {ScrollValue:0}')
    tick(2)
    assert number(21, 'Speed') == 0
    command('tick unfreeze')
    call('wait', 20)
    same(full_input, amounts(2000))

    if s['mode'] == 'dedicated':
        # One accepted Basin bucket action supplies the actual peer empty view.
        command('give TeamActor minecraft:bucket 1')
        call('select', 'minecraft:bucket')
        steps.append({'inventory_before_drain': call('inventory')})
        call('interact', 23, 65, 20, 'south')
        drained = amounts(1000)
        assert (drained['tank'], drained['basin']) == (1000, 0), drained
        steps.append({'inventory_after_drain': call('inventory')})
        call('wait', 12, role='peer')
        shot('basin-empty-after-bucket')
    close()

try:
    (before_cold if a.phase == 'before-cold' else after_cold)()
except Exception:
    (out / f'{a.phase}-result-{s["index"]:02d}.json').write_text(json.dumps(
        {'status': 'FAIL', 'traceback': traceback.format_exc(), 'steps': steps}, indent=2) + '\n')
    raise
else:
    (out / f'{a.phase}-result-{s["index"]:02d}.json').write_text(json.dumps(
        {'status': 'PASS', 'steps': steps, 'ended': time.time()}, indent=2) + '\n')
