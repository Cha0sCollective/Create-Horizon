#!/usr/bin/env python3
"""Drive the bounded Goggles craft/equip and selected HUD observations.

Attach to root-owned, already running isolated sessions. This does not launch,
build, restart, or prepare resources. Screenshots always need human review.
"""
import argparse
import json
import math
import re
import subprocess
import time
from pathlib import Path


ITEMS = {"minecraft:string": 1, "minecraft:glass": 2, "create:golden_sheet": 1}
FIELD = "create_horizon:solver_theoretical_speed"
NUMBER = r"(-?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][+-]?\d+)?)"


def require(ok, message):
    if not ok:
        raise AssertionError(message)


def position(value):
    require(isinstance(value, list) and len(value) == 3, f"bad position: {value!r}")
    require(all(isinstance(n, (int, float)) for n in value), f"bad position: {value!r}")
    return value


def item_count(inv, item):
    return inv["counts"].get(item, 0)


def head(inv):
    return inv["armor"][0]


def assert_stock(inv, stock, goggles):
    for item, count in stock.items():
        require(item_count(inv, item) == count, f"{item}: {inv['counts']}")
    require(item_count(inv, "create:goggles") == goggles, f"goggles: {inv['counts']}")


class Controls:
    def __init__(self, name, ctl, socket, transcript):
        self.name, self.ctl, self.socket, self.transcript = name, str(ctl), str(socket), transcript

    def call(self, *words):
        command = [self.ctl, "--socket", self.socket, *map(str, words)]
        reply = subprocess.run(command, capture_output=True, text=True, timeout=150)
        self.transcript.append({"role": self.name, "words": list(map(str, words)),
                                "code": reply.returncode, "stdout": reply.stdout,
                                "stderr": reply.stderr})
        require(reply.returncode == 0, f"{self.name} {words}: {reply.stdout} {reply.stderr}")
        return json.loads(reply.stdout)

    def query(self, command, marker):
        begin, end = f"begin:{marker}", f"end:{marker}"
        self.call("command", "tellraw @s " + json.dumps({"text": begin}))
        self.call("command", command)
        self.call("command", "tellraw @s " + json.dumps({"text": end}))
        deadline = time.monotonic() + 20
        while time.monotonic() < deadline:
            lines = self.call("chat-log", 100)["chat"]
            if begin in lines and end in lines:
                first, last = lines.index(end), lines.index(begin)
                require(first < last, f"query marker order: {lines}")
                return "\n".join(reversed(lines[first + 1:last]))
            time.sleep(.1)
        raise TimeoutError(command)


def capture(client, path):
    require(not path.exists(), f"image already exists: {path}")
    client.call("screenshot", path)
    require(path.is_file() and path.stat().st_size > 1024, f"missing image: {path}")
    return str(path)


def teleport(client, stand):
    stand = position(stand)
    client.call("command", "tp @s " + " ".join(map(str, stand)))
    deadline = time.monotonic() + 10
    while time.monotonic() < deadline:
        current = client.call("state")["player"]["position"]
        if (abs(current[0] - stand[0]) < .2 and
                stand[1] - 3.1 <= current[1] <= stand[1] + .5 and
                abs(current[2] - stand[2]) < .2):
            return
        time.sleep(.1)
    raise TimeoutError(f"{client.name} teleport not observed at {stand}")


def change_gamemode(client, mode):
    client.call("command", "gamemode " + mode)
    client.call("wait", 2)
    deadline = time.monotonic() + 10
    while time.monotonic() < deadline:
        if client.call("state")["gamemode"] == mode:
            return
        time.sleep(.1)
    raise TimeoutError(f"{client.name} gamemode {mode} not observed")


def ensure_hotbar(actor, item):
    inv = actor.call("inventory")
    if any(s.get("item") == item for s in inv["hotbar"]):
        return
    source = next((s["slot"] for s in inv["main"] if s.get("item") == item), None)
    target = next((s["slot"] for s in inv["hotbar"] if not s.get("item")), None)
    require(source is not None and target is not None, f"no hotbar transfer: {inv}")
    actor.call("open-inventory")
    actor.call("click-slot", source)
    actor.call("click-slot", target)
    actor.call("close")
    require(actor.call("inventory")["carried"] is None, "cursor not empty")


def equip(actor):
    ensure_hotbar(actor, "create:goggles")
    actor.call("select", "create:goggles")
    actor.call("look", 0, -80)
    actor.call("mouse", "right", "click")
    actor.call("wait", 4)
    inv = actor.call("inventory")
    require(head(inv).get("item") == "create:goggles" and head(inv).get("count") == 1,
            f"head equip failed: {inv}")
    require(inv["held"] is None and item_count(inv, "create:goggles") == 1,
            f"equip accounting failed: {inv}")
    return inv


def server_head(actor, marker):
    value = actor.query("data get entity @s equipment.head", marker)
    item = re.search(r"(?<![A-Za-z0-9_])id:\s*['\"]?create:goggles['\"]?", value)
    require(item is not None and field_number(value, "count") == 1,
            f"server head slot is not one Goggles: {value}")
    return value


def verify_world(client, meta, role, marker):
    state = client.call("state")
    if meta["mode"] == "integrated":
        require(state["world"] == meta["world_id"], f"{role} in wrong integrated world")
        return
    # Remote clients expose an empty local level name. Bind the observation
    # to the frozen server endpoint/role instead of treating that blank as a
    # world-name match; the server --world identity is in the launcher receipt.
    require(not state.get("world") or state["world"] == meta["world_id"],
            f"{role} reports a different world: {state.get('world')}")
    ready = client.call("wait-ready", 120)
    server = ready["server"]
    require(server["address"] == meta["server_address"] and
            server["name"] == meta[role + "_name"] and
            server["joined"] and server["loaded"] and
            server["dimension"] == "minecraft:overworld",
            f"{role} in wrong dedicated session: {server}")
    response = client.query("time query gametime", marker + "-" + role + "-identity")
    require(re.fullmatch(r"The game time is \d+ tick\(s\)", response.strip()) is not None,
            f"{role} has no fresh server response: {response}")


def unequip(actor):
    actor.call("open-inventory")
    actor.call("click-slot", 500, "shift")
    actor.call("close")
    inv = actor.call("inventory")
    require(not head(inv).get("item") and item_count(inv, "create:goggles") == 1,
            f"unequip accounting failed: {inv}")
    return inv


def views(actor, peer, scene, output, prefix):
    images = []
    teleport(actor, scene["actor_view_stand"])
    actor.call("wait", 20)
    actor.call("look-at", *position(scene["peer_head_target"]))
    for camera in (1, 2, 3):
        actor.call("key", "f5", "tap")
        actor.call("wait", 2)
        images.append(capture(actor, output / f"{prefix}-actor-camera{camera}.png"))
    if peer:
        teleport(peer, scene["peer_stand"])
        peer.call("wait", 40)
        peer.call("look-at", *position(scene["actor_head_target"]))
        peer.call("wait", 20)
        images.append(capture(peer, output / f"{prefix}-peer.png"))
    return images


def craft(actor, peer, scene, output, marker):
    require(actor.call("state")["gamemode"] == "survival", "actor is not in Survival")
    before = actor.call("inventory")
    require(not head(before).get("item"), "actor head must start empty")
    require(before["counts"] == {}, f"actor inventory must start empty: {before['counts']}")
    assert_stock(before, {item: 0 for item in ITEMS}, 0)
    if peer:
        require(item_count(peer.call("inventory"), "create:goggles") == 0,
                "peer already has Goggles")
    for item, count in ITEMS.items():
        actor.call("command", f"give @s {item} {count}")
    actor.call("wait", 3)
    supplied = actor.call("inventory")
    assert_stock(supplied, ITEMS, 0)
    table = position(scene["table"])
    require(actor.call("block", *table)["block"] == "minecraft:crafting_table",
            f"crafting table missing at {table}")
    teleport(actor, scene["craft_stand"])
    actor.call("look-at", "block", *table, "up")
    actor.call("interact", *table, "up")
    screen = actor.call("screen")
    require(screen["screen"] == "inventory" and screen["craft_size"] == 3,
            f"ordinary crafting table not open: {screen}")
    source = {s["item"]: s["slot"] for s in supplied["hotbar"] + supplied["main"]
              if s.get("item") in ITEMS}
    require(set(source) == set(ITEMS), f"fixture slots missing: {supplied}")
    for item, slots in [("minecraft:string", [101]),
                        ("minecraft:glass", [103, 105]),
                        ("create:golden_sheet", [104])]:
        actor.call("click-slot", source[item])
        for slot in slots:
            actor.call("click-slot", slot, "right")
    screen = actor.call("screen")
    grid = {s["slot"]: s for s in screen["slots"]}
    require(grid[200].get("item") == "create:goggles" and grid[200].get("count") == 1,
            f"wrong recipe result: {grid[200]}")
    require(screen["carried"] is None, f"fixture left on cursor: {screen['carried']}")
    actor.call("click-slot", 200, "shift")
    actor.call("close")
    actor.call("wait", 3)
    made = actor.call("inventory")
    assert_stock(made, {item: 0 for item in ITEMS}, 1)
    require(made["carried"] is None and not head(made).get("item"),
            f"craft accounting: {made}")
    actor.call("interact", *table, "up")
    empty = {s["slot"]: s for s in actor.call("screen")["slots"]}
    require(all(not empty[slot].get("item") for slot in (101, 103, 104, 105, 200)),
            f"crafting grid retained stock: {empty}")
    actor.call("close")
    worn = equip(actor)
    server_equipment = server_head(actor, marker + "-equip")
    images = views(actor, peer, scene, output, "equipped")
    removed = unequip(actor)
    images += views(actor, peer, scene, output, "unequipped")
    reworn = equip(actor)
    images += views(actor, peer, scene, output, "reequipped")
    if peer:
        require(item_count(peer.call("inventory"), "create:goggles") == 0,
                "peer acquired fixture Goggles")
    return {"fixture": supplied, "craft_result": grid[200], "crafted": made,
            "equipped": worn, "server_equipment": server_equipment,
            "unequipped": removed, "reequipped": reworn, "images": images,
            "visual_review_pending": images}


def equip_fixture(actor, marker):
    require(actor.call("state")["gamemode"] == "survival", "fixture actor is not in Survival")
    before = actor.call("inventory")
    require(not head(before).get("item") and item_count(before, "create:goggles") == 0,
            f"fixture actor already has Goggles: {before}")
    actor.call("command", "give @s create:goggles 1")
    actor.call("wait", 3)
    supplied = actor.call("inventory")
    require(item_count(supplied, "create:goggles") == 1, supplied)
    worn = equip(actor)
    server_equipment = server_head(actor, marker + "-fixture-equip")
    return {"fixture_only": True, "supplied": supplied, "equipped": worn,
            "server_equipment": server_equipment}


def field_number(text, key):
    found = re.search(r"(?<![A-Za-z0-9_])['\"]?" + re.escape(key) + r"['\"]?:\s*" + NUMBER, text)
    require(found is not None, f"missing {key}: {text}")
    return float(found.group(1))


def block_fields(text):
    status = re.search(r"NetworkStatus:\s*['\"]?([A-Za-z]+)['\"]?", text)
    require(status is not None, f"missing NetworkStatus: {text}")
    float_field = re.search(r"['\"]?" + re.escape(FIELD) + r"['\"]?:\s*" + NUMBER + r"f\b", text)
    require(float_field is not None, f"solver speed is not a Float: {text}")
    return {"status": status.group(1), "theoretical": field_number(text, FIELD),
            "speed": field_number(text, "Speed"),
            "flow": field_number(text, "FlowScore") if "FlowScore" in text else None}


def new_trace(log, offset, pos, role):
    lines = log.read_bytes()[offset:].decode("utf-8", errors="replace").splitlines()
    events = []
    for line in lines:
        if "Create visual: " not in line:
            continue
        try:
            event = json.loads(line.split("Create visual: ", 1)[1])
        except json.JSONDecodeError:
            continue
        if event.get("event") == "goggles_hud" and event.get("pos") == pos:
            events.append(event)
    require(events, f"no fresh {role} goggles_hud trace at {pos}")
    return events[-1]


def traces_since(log, offset, pos):
    lines = log.read_bytes()[offset:].decode("utf-8", errors="replace").splitlines()
    events = []
    for line in lines:
        if "Create visual: " not in line:
            continue
        try:
            event = json.loads(line.split("Create visual: ", 1)[1])
        except json.JSONDecodeError:
            continue
        if event.get("event") == "goggles_hud" and event.get("pos") == pos:
            events.append(event)
    return events


def absent(actor, log, output, name, pos):
    actor.call("wait", 2)
    offset = log.stat().st_size
    actor.call("wait", 5)
    image = capture(actor, output / f"hud-absent-{name}.png")
    require(not traces_since(log, offset, pos), f"unexpected goggles HUD while {name}")
    return image


def hud(actor, scene, output, marker, target, expected_status, log):
    require(head(actor.call("inventory")).get("item") == "create:goggles", "Goggles not worn")
    require(target in ("millstone", "wheel"), "selected target only")
    pos = position(scene[target])
    block = "create:millstone" if target == "millstone" else "create:water_wheel"
    require(actor.call("block", *pos)["block"] == block, f"wrong HUD target at {pos}")
    teleport(actor, scene[target + "_stand"])
    actor.call("look-at", "block", *pos)
    actor.call("wait", 5)
    server = actor.query("data get block " + " ".join(map(str, pos)), marker + "-" + target)
    fields = block_fields(server)
    require(fields["status"] == expected_status, f"status: {fields}")
    require(expected_status in ("Running", "Idle", "Overstressed"), expected_status)
    require(math.isfinite(fields["theoretical"]), f"nonfinite server speed: {fields}")
    if expected_status == "Idle":
        require(fields["theoretical"] == 0, f"Idle speed not zero: {fields}")
    if expected_status == "Overstressed":
        require(fields["speed"] == 0, f"effective RPM not zero: {fields}")
        require(fields["theoretical"] != 0, f"overstress lost theoretical RPM: {fields}")
    if target == "millstone":
        role, expected = "stress_impact", 4 * abs(fields["theoretical"])
    else:
        require(fields["flow"] is not None, f"wheel FlowScore missing: {server}")
        require(fields["flow"].is_integer(), f"noninteger FlowScore: {server}")
        generated = max(-1, min(1, int(fields["flow"]))) * 8
        per_rpm = 32.0
        if fields["theoretical"] != generated and fields["theoretical"] != 0:
            per_rpm *= generated / fields["theoretical"]
        role, expected = "capacity_provided", abs(per_rpm * fields["theoretical"])
    offset = log.stat().st_size
    actor.call("wait", 5)
    image = capture(actor, output / f"hud-{target}-{expected_status.lower()}.png")
    trace = new_trace(log, offset, pos, role)
    details = trace["details"]
    require(details["provider"] == role and
            math.isclose(details["value_su"], expected, abs_tol=0.01) and
            math.isclose(details["theoretical_rpm"], fields["theoretical"], abs_tol=0.01),
            f"server/client trace mismatch: {fields}, {trace}")
    return {"server": server, "fields": fields, "expected_value_su": expected,
            "client_trace": trace, "image": image, "visual_review_pending": [image]}


def negatives(actor, scene, output, marker, log):
    require(actor.call("state")["gamemode"] == "survival", "actor is not in Survival")
    require(head(actor.call("inventory")).get("item") == "create:goggles", "Goggles not worn")
    millstone = position(scene["millstone"])
    teleport(actor, scene["millstone_stand"])
    actor.call("look-at", "block", *millstone)
    baseline_offset = log.stat().st_size
    actor.call("wait", 5)
    require(traces_since(log, baseline_offset, millstone),
            "no selected baseline HUD before negative checks")
    images = []
    unequip(actor)
    actor.call("look-at", "block", *millstone)
    images.append(absent(actor, log, output, "no-goggles", millstone))
    equip(actor)
    actor.call("look-at", "block", *millstone)
    actor.call("key", "f1", "tap")
    try:
        require(head(actor.call("inventory")).get("item") == "create:goggles",
                "F1 check lost worn Goggles")
        images.append(absent(actor, log, output, "f1", millstone))
    finally:
        actor.call("key", "f1", "tap")
    try:
        change_gamemode(actor, "spectator")
        spectator_inventory = actor.call("inventory")
        require(head(spectator_inventory).get("item") == "create:goggles" and
                item_count(spectator_inventory, "create:goggles") == 1,
                "spectator lost worn Goggles")
        server_head(actor, marker + "-spectator")
        actor.call("look-at", "block", *millstone)
        images.append(absent(actor, log, output, "spectator", millstone))
    finally:
        change_gamemode(actor, "survival")
    shaft = position(scene["shaft"])
    require(actor.call("block", *shaft)["block"] == "create:shaft", "plain shaft missing")
    teleport(actor, scene["shaft_stand"])
    actor.call("look-at", "block", *shaft)
    images.append(absent(actor, log, output, "plain-shaft", shaft))
    gauge = position(scene["speedometer"])
    require(actor.call("block", *gauge)["block"] == "create:speedometer", "speedometer missing")
    teleport(actor, scene["speedometer_stand"])
    actor.call("look-at", "block", *gauge)
    actor.call("wait", 3)
    offset = log.stat().st_size
    actor.call("wait", 5)
    images.append(capture(actor, output / "hud-speedometer.png"))
    events = traces_since(log, offset, gauge)
    require(events, "retained speedometer HUD trace missing")
    gauge_trace = events[-1]
    require(gauge_trace["details"]["provider"] == "legacy_network", gauge_trace)
    server_gauge = actor.query("data get block " + " ".join(map(str, gauge)), marker + "-gauge")
    gauge_speed = field_number(server_gauge, "Speed")
    require(math.isclose(gauge_trace["details"]["rpm"], gauge_speed, abs_tol=0.01),
            f"gauge RPM mismatch: {gauge_trace}, {server_gauge}")
    return {"images": images, "gauge_server": server_gauge, "gauge_trace": gauge_trace,
            "visual_review_pending": images}


def reopen(actor, peer, scene, output, marker):
    require(actor.call("state")["gamemode"] == "survival", "actor reopened outside Survival")
    inv = actor.call("inventory")
    assert_stock(inv, {item: 0 for item in ITEMS}, 1)
    require(head(inv).get("item") == "create:goggles", f"head not retained: {inv}")
    server_equipment = server_head(actor, marker + "-reopen")
    images = views(actor, peer, scene, output, "reopened")
    return {"inventory": inv, "server_equipment": server_equipment,
            "images": images, "visual_review_pending": images}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--metadata", type=Path, required=True, help="frozen root-supplied session/build JSON")
    parser.add_argument("--output", type=Path, required=True, help="packet-owned evidence directory outside Git")
    parser.add_argument("--step", choices=("craft", "equip-fixture", "hud", "negatives", "reopen"), required=True)
    parser.add_argument("--target", choices=("millstone", "wheel"))
    parser.add_argument("--status", choices=("Running", "Idle", "Overstressed"))
    args = parser.parse_args()
    meta = json.loads(args.metadata.read_text(encoding="utf-8"))
    for key in ("source_commit", "binary_sha256", "resource_key", "world_id", "mode", "ctl",
                "actor_socket", "actor_log", "scene"):
        require(meta.get(key), f"missing frozen metadata {key}")
    require(re.fullmatch(r"[0-9a-f]{40}", meta["source_commit"]) is not None,
            "source_commit must be a full Git revision")
    require(isinstance(meta["binary_sha256"], dict) and
            all(isinstance(value, str) and re.fullmatch(r"[0-9a-f]{64}", value)
                for value in meta["binary_sha256"].values()),
            "binary_sha256 must map roles to full hashes")
    require(meta["mode"] in ("integrated", "dedicated"), "bad mode")
    require(meta["mode"] != "dedicated" or meta.get("peer_socket"), "dedicated peer missing")
    require(args.step != "hud" or (args.target and args.status), "HUD needs target and status")
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    name = args.step + (f"-{args.target}-{args.status.lower()}" if args.step == "hud" else "")
    report_path, transcript_path = output / f"{name}.json", output / f"{name}-transcript.json"
    require(not report_path.exists() and not transcript_path.exists(), f"step already recorded: {name}")
    transcript = []
    actor = Controls("actor", meta["ctl"], meta["actor_socket"], transcript)
    peer = Controls("peer", meta["ctl"], meta["peer_socket"], transcript) if meta.get("peer_socket") else None
    marker = f"experience003-{name}-{int(time.time())}"
    report = {"step": name, "metadata": meta, "passed_assertions": False}
    try:
        verify_world(actor, meta, "actor", marker)
        if peer:
            verify_world(peer, meta, "peer", marker)
        if args.step == "craft":
            report["result"] = craft(actor, peer, meta["scene"], output, marker)
        elif args.step == "equip-fixture":
            report["result"] = equip_fixture(actor, marker)
        elif args.step == "hud":
            log = Path(meta["actor_log"])
            require(log.is_file(), f"actor trace log missing: {log}")
            report["result"] = hud(actor, meta["scene"], output, marker, args.target, args.status, log)
        elif args.step == "negatives":
            log = Path(meta["actor_log"])
            require(log.is_file(), f"actor trace log missing: {log}")
            report["result"] = negatives(actor, meta["scene"], output, marker, log)
        else:
            report["result"] = reopen(actor, peer, meta["scene"], output, marker)
        report["passed_assertions"] = True
    except Exception as error:
        report["error"] = repr(error)
        raise
    finally:
        transcript_path.write_text(json.dumps(transcript, indent=2) + "\n", encoding="utf-8")
        report_path.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
