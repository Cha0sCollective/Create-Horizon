"""One-off FLUIDS-004A checks against an already running frozen .11 session.

The project lead owns launch, lease, process exits and raw evidence preservation.
Run each case once at a distinct pad in a new disposable world.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import subprocess
import time
import traceback
import uuid
from pathlib import Path


SOURCE = "55339ce3347b2ce667281e8a3e6318ec856144e4"
SOURCE_MAP = "b6b913501fdfc3fe4326dcafb53fc8c47a8eb7dccc697615ba0a5d984e18aa4b"
GAME_SHA = "edba263e154f198b4a62033d3c468ffa6567c371c5201eb5ccb8577cc0ba807b"
CTL_SHA = "170e866057d97e73b9231c8084547d4a4a02346238db35901ef64bf3d0d6d449"


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("session", type=Path, help="already launched session JSON")
    parser.add_argument("--ctl", type=Path, required=True, help="frozen horizon-ctl binary")
    parser.add_argument("--out", type=Path, required=True, help="new receipt directory")
    parser.add_argument("--case", choices=("normal", "sneak"), required=True)
    parser.add_argument("--origin", nargs=3, type=int, required=True, metavar=("X", "Y", "Z"))
    return parser.parse_args()


class Check:
    def __init__(self, args: argparse.Namespace, session: dict):
        self.args = args
        self.session = session
        self.steps: list[dict] = []
        self.observations: dict = {}
        self.x, self.y, self.z = args.origin
        self.peer = "peer" if session["mode"] == "dedicated" else "actor"
        self.peer_counts: dict[str, int] | None = None

    def call(self, *words: object, role: str = "actor") -> dict:
        command = [
            str(self.args.ctl),
            "--socket",
            self.session["processes"][role]["socket"],
            *map(str, words),
        ]
        result = subprocess.run(command, capture_output=True, text=True, timeout=180)
        record = {
            "role": role,
            "words": list(map(str, words)),
            "code": result.returncode,
            "stdout": result.stdout,
            "stderr": result.stderr,
        }
        self.steps.append(record)
        if result.returncode:
            raise RuntimeError(f"control call failed: {record}")
        try:
            return json.loads(result.stdout)
        except json.JSONDecodeError as error:
            raise RuntimeError(f"control call returned non-JSON: {record}") from error

    def wait_chat(self, command: str, needle: str) -> str:
        """Bound a query to a unique chat marker, never an older NBT reply."""
        marker = "FLUIDS004_" + uuid.uuid4().hex[:12]
        self.call("command", "say " + marker)
        self.call("command", command)
        for _ in range(20):
            lines = self.call("chat-log", 60).get("chat", [])
            markers = [i for i, line in enumerate(lines) if marker in line]
            if markers:
                # Both local and dedicated chat-log return newest lines first.
                fresh = [line for line in lines[: markers[0]] if marker not in line]
                hits = [line for line in fresh if needle in line]
                if hits:
                    return hits[0]
            self.call("wait", 1)
        raise AssertionError(f"no fresh {needle!r} reply to {command!r}")

    def nbt(self, pos: tuple[int, int, int]) -> str:
        x, y, z = pos
        return self.wait_chat(
            f"data get block {x} {y} {z}",
            f"{x}, {y}, {z} has the following block data:",
        )

    def content(self, pos: tuple[int, int, int], expected: int) -> str:
        data = self.nbt(pos)
        actual = re.search(r"\bamount:\s*(\d+)", data)
        amount = int(actual.group(1)) if actual else 0
        assert amount == expected, (pos, expected, amount, data)
        assert "Size: 1" in data and "Height: 1" in data, data
        if expected == 0:
            assert "TankContent: {}" in data, data
        else:
            assert "minecraft:water" in data, data
        return data

    def absent_block_entity(self, pos: tuple[int, int, int]) -> str:
        x, y, z = pos
        marker = "FLUIDS004_ABSENT_" + uuid.uuid4().hex[:12]
        self.call("command", "say " + marker)
        self.call("command", f"data get block {x} {y} {z}")
        for _ in range(20):
            lines = self.call("chat-log", 60).get("chat", [])
            markers = [i for i, line in enumerate(lines) if marker in line]
            if markers:
                fresh = [line for line in lines[: markers[0]] if marker not in line]
                errors = [
                    line for line in fresh
                    if "block entity" in line.lower() or "commands.data.block.invalid" in line
                ]
                if errors:
                    assert all("TankContent" not in line for line in fresh), fresh
                    return errors[0]
            self.call("wait", 1)
        raise AssertionError(f"missing old-block-entity absence reply at {pos}")

    def teleport(self, x: float, y: float, z: float, *, role: str = "actor") -> None:
        self.call("command", f"tp @s {x} {y} {z}", role=role)
        self.call("wait-ready", 120, role=role)

        for _ in range(40):
            pos = self.call("status", role=role)["player"]["position"]
            if abs(pos[0] - x) < 0.3 and abs(pos[2] - z) < 0.3 and abs(pos[1] - y) < 1.1:
                return
            self.call("wait", 1, role=role)
        raise AssertionError(f"{role} teleport did not reach {(x, y, z)}")

    def at(self, x: int, z: int, *, role: str = "actor") -> None:
        self.teleport(x + 0.5, self.y + 1, z + 3.5, role=role)

    def give(self, item: str) -> None:
        self.call("command", f"give @s {item} 1")
        self.call("wait", 3)
        self.call("select", item)

    def shot(self, name: str) -> None:
        self.call("screenshot", self.args.out / f"{name}-{self.peer}.png", role=self.peer)

    def peer_block(self, pos: tuple[int, int, int], expected: str) -> dict:
        self.call("wait", 3, role=self.peer)
        observed = self.call("block", *pos, role=self.peer)
        assert observed["block"] == expected, observed
        return observed

    def grid(self) -> dict[str, str]:
        """Every block in the air-filled 7x7 horizontal fixture at Tank height."""
        found = {}
        for x in range(self.x - 3, self.x + 4):
            for z in range(self.z - 3, self.z + 4):
                block = self.call("block", x, self.y, z)
                assert block.get("loaded"), (x, self.y, z, block)
                found[f"{x},{self.y},{z}"] = block["block"]
        return found

    @staticmethod
    def no_world_water(grid: dict[str, str]) -> None:
        wet = {pos: block for pos, block in grid.items() if "water" in block}
        assert not wet, wet

    @staticmethod
    def tank_stack(inventory: dict) -> dict:
        def visit(value: object):
            if isinstance(value, dict):
                if value.get("item") == "create:fluid_tank":
                    yield value
                for child in value.values():
                    yield from visit(child)
            elif isinstance(value, list):
                for child in value:
                    yield from visit(child)

        matches = list(visit({key: inventory.get(key) for key in ("hotbar", "main", "offhand", "held")}))
        assert matches, inventory
        for stack in matches:
            assert stack.get("count") == 1, stack
            assert not stack.get("components"), stack
        return matches[0]

    def assert_inventory(self, tank_count: int, tool: str) -> dict:
        inventory = self.call("inventory")
        counts = {name: amount for name, amount in inventory["counts"].items() if amount}
        expected = {tool: 1}
        if tank_count:
            expected["create:fluid_tank"] = tank_count
        assert counts == expected, (counts, expected, inventory)
        if tank_count:
            self.tank_stack(inventory)
        return inventory

    def assert_peer_inventory(self) -> dict | None:
        if self.peer == "actor":
            return None
        inventory = self.call("inventory", role=self.peer)
        counts = {name: amount for name, amount in inventory["counts"].items() if amount}
        assert counts == self.peer_counts, (counts, self.peer_counts, inventory)
        return inventory

    def setup(self) -> tuple[int, int, int]:
        x, y, z = self.x, self.y, self.z
        target = (x, y, z)
        self.call("wait-ready", 120)
        if self.peer != "actor":
            self.call("wait-ready", 120, role=self.peer)
        self.call("command", "gamemode creative")
        self.call("wait", 3)
        assert self.call("status")["creative"] is True
        self.at(x, z)
        self.call("fill", x - 3, y, z - 3, x + 3, y + 3, z + 3, "minecraft:air")
        self.call("fill", x - 3, y - 1, z - 3, x + 3, y - 1, z + 3, "minecraft:smooth_stone")
        self.call("command", "clear @s")
        self.no_item_entity()
        self.give("create:fluid_tank")
        self.call("place", x, y, z)
        state = self.call("block", *target)
        assert state["block"] == "create:fluid_tank"
        assert all(part in state["state"] for part in ("bottom=true", "top=true", "shape=window")), state
        assert "Window: 1b" in self.content(target, 0)
        self.give("minecraft:water_bucket")
        assert self.call("inventory")["counts"].get("minecraft:water_bucket", 0) == 1
        self.call("interact", x, y, z, "south")
        self.observations["filled"] = self.content(target, 1000)
        assert self.call("inventory")["counts"].get("minecraft:water_bucket", 0) == 1
        if self.peer != "actor":
            self.at(x + 4, z, role=self.peer)
            self.call("look-at", x + 0.5, y + 0.5, z + 0.5, role=self.peer)
            peer_inventory = self.call("inventory", role=self.peer)
            self.peer_counts = {
                name: amount for name, amount in peer_inventory["counts"].items() if amount
            }
            assert not self.peer_counts, peer_inventory
            self.observations["peer_inventory_baseline"] = peer_inventory
        self.observations["peer_filled_block"] = self.peer_block(target, "create:fluid_tank")
        self.shot("filled-window")
        return target

    def no_item_entity(self) -> bool:
        """Query a new disposable world's item-entity set, without removing it."""
        marker = "FLUIDS004_ITEM_" + uuid.uuid4().hex[:12]
        self.call("command", f"execute unless entity @e[type=minecraft:item] run say {marker}")
        for _ in range(20):
            lines = self.call("chat-log", 60).get("chat", [])
            if any(marker in line for line in lines):
                return True
            self.call("wait", 1)
        raise AssertionError("an item entity remains in the disposable world")

    def replace_from_inventory(self, target: tuple[int, int, int], tool: str) -> None:
        x, y, z = target
        replacement = (x + 2, y, z)
        self.call("select", "create:fluid_tank")
        self.call("place", *replacement)
        assert self.call("block", *replacement)["block"] == "create:fluid_tank"
        self.observations["replacement_nbt"] = self.content(replacement, 0)
        assert self.call("block", *target)["block"] == "minecraft:air"
        self.observations["after_replacement_inventory"] = self.assert_inventory(0, tool)
        self.observations["peer_after_replacement_inventory"] = self.assert_peer_inventory()
        self.no_world_water(self.grid())
        self.no_item_entity()
        self.observations["peer_replacement_block"] = self.peer_block(replacement, "create:fluid_tank")
        self.shot("replacement")

    def normal(self) -> None:
        target = self.setup()
        x, y, z = target
        self.give("create:wrench")
        old_state = self.call("block", *target)["state"]
        self.call("interact", x, y, z, "south")
        new_state = self.call("block", *target)["state"]
        assert old_state != new_state and "shape=plain" in new_state, (old_state, new_state)
        self.observations["window_nbt"] = self.content(target, 1000)
        assert "Window: 0b" in self.observations["window_nbt"]
        self.observations["peer_after_window_inventory"] = self.assert_peer_inventory()
        self.shot("plain-window")

        self.call("command", "gamemode survival")
        self.call("wait", 3)
        assert self.call("status")["creative"] is False
        self.call("command", "clear @s")
        self.give("minecraft:diamond_pickaxe")
        self.observations["before_break_inventory"] = self.assert_inventory(0, "minecraft:diamond_pickaxe")
        broken = self.call("break", x, y, z)
        assert broken.get("block") == "minecraft:air", broken
        assert broken.get("drops", {}).get("create:fluid_tank") == 1, broken
        assert broken.get("items_left_on_ground") == 1, broken
        self.observations["break"] = broken
        self.observations["after_break_inventory"] = self.assert_inventory(0, "minecraft:diamond_pickaxe")
        self.observations["drop_item"] = self.wait_chat(
            "data get entity @e[type=minecraft:item,limit=1] Item", "create:fluid_tank"
        )
        assert "TankContent" not in self.observations["drop_item"]
        assert "block_entity_data" not in self.observations["drop_item"]
        self.observations["after_break_grid"] = self.grid()
        self.no_world_water(self.observations["after_break_grid"])
        assert self.call("block", *target)["block"] == "minecraft:air"
        self.observations["old_block_entity_absent"] = self.absent_block_entity(target)
        self.observations["peer_after_break_block"] = self.peer_block(target, "minecraft:air")
        self.observations["peer_after_break_inventory"] = self.assert_peer_inventory()
        self.shot("after-break")
        self.teleport(x + 0.5, y + 1, z + 0.5)
        self.call("wait", 5)
        self.observations["after_pickup_inventory"] = self.assert_inventory(1, "minecraft:diamond_pickaxe")
        self.observations["peer_after_pickup_inventory"] = self.assert_peer_inventory()
        self.no_item_entity()
        self.replace_from_inventory(target, "minecraft:diamond_pickaxe")

    def sneak(self) -> None:
        target = self.setup()
        x, y, z = target
        self.call("command", "gamemode survival")
        self.call("wait", 3)
        assert self.call("status")["creative"] is False
        self.call("command", "clear @s")
        self.give("create:wrench")
        self.observations["before_wrench_inventory"] = self.assert_inventory(0, "create:wrench")
        self.call("key", "shift", "down")
        try:
            self.call("interact", x, y, z, "south")
            self.call("wait", 3)
        finally:
            self.call("release")
        assert self.call("block", *target)["block"] == "minecraft:air"
        self.observations["old_block_entity_absent"] = self.absent_block_entity(target)
        self.observations["after_wrench_inventory"] = self.assert_inventory(1, "create:wrench")
        self.observations["peer_after_wrench_inventory"] = self.assert_peer_inventory()
        self.observations["after_wrench_grid"] = self.grid()
        self.no_world_water(self.observations["after_wrench_grid"])
        self.no_item_entity()
        self.observations["peer_after_wrench_block"] = self.peer_block(target, "minecraft:air")
        self.shot("after-wrench")
        self.replace_from_inventory(target, "create:wrench")


def main() -> int:
    args = parse_args()
    session = json.loads(args.session.read_text())
    assert session["source_head"] == SOURCE, session.get("source_head")
    assert session["source_map_sha256"] == SOURCE_MAP, session.get("source_map_sha256")
    assert session["binary_sha256"]["horizon-client"] == GAME_SHA
    assert session["binary_sha256"]["horizon-ctl"] == CTL_SHA
    assert sha256(args.ctl) == CTL_SHA
    assert sha256(Path(session["processes"]["actor"]["command"][0])) == GAME_SHA
    assert session["mode"] in ("integrated", "dedicated")
    assert args.session.resolve() != args.out.resolve()
    args.out.mkdir(parents=True, exist_ok=False)
    check = Check(args, session)
    receipt = {
        "case": args.case,
        "origin": args.origin,
        "session": str(args.session),
        "source_head": SOURCE,
        "source_map_sha256": SOURCE_MAP,
        "binary_sha256": session["binary_sha256"],
        "started_unix": time.time(),
    }
    try:
        getattr(check, args.case)()
        receipt["passed"] = True
    except Exception:
        receipt["passed"] = False
        receipt["error"] = traceback.format_exc()
    receipt["ended_unix"] = time.time()
    receipt["observations"] = check.observations
    receipt["steps"] = check.steps
    (args.out / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print(json.dumps({key: receipt.get(key) for key in ("case", "passed", "error", "session")}))
    return 0 if receipt["passed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
