"""Finite, disposable CONTRAPTIONS-004 owner foundation runtime check.

Attach to an already launched native-owner-foundation-005/session.py dedicated
session. This script starts no game or build and touches no world blocks.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import shutil
import subprocess
import time
import uuid
from pathlib import Path


SOURCE = "9651dba12d163896d45a8cc4ba1b0b3b0da1feff"
PACKET = Path("/mnt/g/Create Rusty/.local/native-owner-foundation-005")
HANDOFF = (
    Path(__file__).resolve().parents[2]
    / "docs/teams/contraptions/CONTRAPTIONS_004_PROBE_HANDOFF.md"
)
SELECTOR = "@e[tag=ch004_probe,limit=1]"
POSITION = [8.5, 64.0, 8.5]


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for part in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(part)
    return digest.hexdigest()


def save_json(path: Path, value: object) -> None:
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n")


def fixture_command() -> str:
    lines = [line for line in HANDOFF.read_text().splitlines()
             if line.startswith("chat /summon minecraft:block_display 8.5 64 8.5 ")]
    assert len(lines) == 1, "exact single-line fixture missing from reviewed handoff"
    return lines[0].removeprefix("chat ")


class Packet:
    def __init__(self, session_path: Path, phase: str):
        session_path = session_path.resolve()
        self.session = json.loads(session_path.read_text())
        session = self.session
        assert session["mode"] == "dedicated" and set(session["processes"]) == {"server", "peer"}
        assert session["source_head"] == SOURCE and session["source_revision"] == SOURCE
        self.index = session["index"]
        assert self.index == (0 if phase == "initial" else 1), (phase, self.index)
        self.phase = phase
        mirror = Path(session["mirror"]).resolve()
        assert mirror.parent == PACKET.resolve(), mirror
        assert session_path == mirror / f"session-{self.index:02d}.json"
        self.out = mirror / f"owner-check-{self.index:02d}"
        self.out.mkdir(exist_ok=False)
        self.steps: list[dict] = []
        self.checks: list[str] = []
        self.server_log = Path(session["processes"]["server"]["log"])
        self.peer_log = Path(session["processes"]["peer"]["log"])
        self.socket = Path(session["processes"]["peer"]["socket"])
        game = Path(session["processes"]["peer"]["game"])
        self.ctl = game.with_name("horizon-ctl")
        assert sha256(game) == session["binary_sha256"]["horizon-client"]
        assert sha256(self.ctl) == session["binary_sha256"]["horizon-ctl"]
        shutil.copy2(Path(__file__), self.out / "driver.py")
        shutil.copy2(HANDOFF, self.out / HANDOFF.name)
        save_json(self.out / "inputs.json", {
            "phase": phase,
            "session": session,
            "driver_sha256": sha256(Path(__file__)),
            "handoff_sha256": sha256(HANDOFF),
            "fixture_sha256": hashlib.sha256(fixture_command().encode()).hexdigest(),
        })

    def check(self, name: str, condition: bool) -> None:
        if not condition:
            raise AssertionError(name)
        self.checks.append(name)

    def call(self, *words: object, timeout: int = 60, allow_error: bool = False) -> dict:
        command = [str(self.ctl), "--socket", str(self.socket),
                   "--timeout", str(timeout), *map(str, words)]
        result = subprocess.run(command, capture_output=True, text=True, timeout=timeout + 10)
        receipt = {"words": list(map(str, words)), "exit_code": result.returncode,
                   "stdout": result.stdout, "stderr": result.stderr}
        self.steps.append(receipt)
        save_json(self.out / f"control-{len(self.steps):03d}.json", receipt)
        if result.returncode and not allow_error:
            raise RuntimeError(receipt)
        try:
            value = json.loads(result.stdout)
        except json.JSONDecodeError:
            if allow_error:
                return receipt
            raise RuntimeError(receipt) from None
        if isinstance(value, dict) and value.get("error") and not allow_error:
            raise RuntimeError(receipt)
        return value

    def ready_near_fixture(self) -> None:
        deadline = time.monotonic() + 120
        while not self.socket.exists() and time.monotonic() < deadline:
            time.sleep(0.25)
        self.check("peer socket appeared", self.socket.exists())
        self.call("wait-ready", 120, timeout=130)
        self.call("command", "tp @s 8.5 66 12.5")
        self.call("wait-ready", 120, timeout=130)
        deadline = time.monotonic() + 90
        while time.monotonic() < deadline:
            status = self.call("status")
            position = status.get("player", {}).get("position")
            block = self.call("block", 8, 64, 8)
            if (position and abs(position[0] - 8.5) < 0.3
                    and abs(position[2] - 12.5) < 0.3 and block.get("loaded")):
                self.check("peer at loaded fixture chunk", True)
                return
            time.sleep(0.5)
        raise AssertionError("peer did not reach loaded fixture chunk")

    def chat_reply(self, command: str, needle: str) -> str:
        marker = "OWNER004_" + uuid.uuid4().hex[:12]
        self.call("command", "say " + marker)
        self.call("command", command)
        for _ in range(30):
            lines = self.call("chat-log", 80).get("chat", [])
            markers = [i for i, line in enumerate(lines) if marker in line]
            if markers:
                fresh = [line for line in lines[:markers[0]] if marker not in line]
                joined = " ".join(fresh)
                if needle in joined:
                    return joined
            time.sleep(0.5)
        raise AssertionError(f"no fresh {needle!r} reply to {command!r}")

    def entity(self) -> str:
        return self.chat_reply(f"data get entity {SELECTOR}", "assembly_record_v1")

    def position(self) -> list[float]:
        line = self.chat_reply(f"data get entity {SELECTOR} Pos", "has the following entity data")
        numbers = re.findall(r"(?<![A-Za-z])[-+]?\d+(?:\.\d+)?", line.rsplit(":", 1)[-1])
        self.check("entity Pos has three values", len(numbers) == 3)
        return [float(number) for number in numbers]

    def trace(self, role: str, event: str) -> dict:
        path = self.server_log if role == "server" else self.peer_log
        deadline = time.monotonic() + 90
        while time.monotonic() < deadline:
            for line in path.read_text(errors="replace").splitlines():
                if "Create owner probe: " in line:
                    payload = json.loads(line.split("Create owner probe: ", 1)[1])
                    if payload.get("event") == event:
                        save_json(self.out / f"{role}-probe.json", payload)
                        return payload
            time.sleep(0.5)
        raise AssertionError(f"missing {role} {event} probe line: {path}")

    def inspect_probes(self) -> None:
        server = self.trace("server", "server_scan_and_guards")
        self.check("server exact IDs", server["uuid"] == [5, 6, 7, 8]
                   and server["assembly_id"] == [1, 2, 3, 4])
        self.check("server discovered only probe", server["discovered"] is True
                   and server["scan_refused"] == [] and server["scan_retired"] == []
                   and len(server["scan_active"]) == 1)
        self.check("native move and remove refused", server["move_return"] is False
                   and server["remove_return"] is False)
        self.check("carrier retained in place", server["retained"] is True
                   and server["before"] == POSITION and server["after"] == POSITION)
        peer = self.trace("peer", "peer_view_received")
        self.check("peer exact IDs and private record absent", peer["uuid"] == [5, 6, 7, 8]
                   and peer["assembly_id"] == [1, 2, 3, 4]
                   and peer["canonical_record_present"] is False)
        self.check("peer inert view payload", peer["phase"] == "preparing"
                   and peer["anchor"] == [8, 64, 8] and peer["axis"] == "y"
                   and peer["cells"][0] == {"local": [0, 0, 0], "state": "minecraft:slime_block"}
                   and peer["cells"][1] == {"local": [0, 0, 1], "state": "minecraft:oak_log", "axis": "z"})

    def inspect_saved_entity(self) -> None:
        raw = self.entity()
        self.check("exact marker retained", "create_horizon:assembly_owner_v1" in raw
                   and "create_horizon:assembly_owner:1" in raw)
        self.check("raw record and synced view retained", "assembly_record_v1" in raw
                   and "assembly_view_v1" in raw and "ch004_probe" in raw)
        self.check("carrier position unchanged", self.position() == POSITION)
        save_json(self.out / "entity-query.json", {"raw": raw, "position": POSITION})

    def world_cells(self) -> dict[str, str]:
        found = {}
        for x, y, z in [(8, 63, 8), (8, 64, 8), (8, 64, 9)]:
            block = self.call("block", x, y, z)
            self.check(f"loaded world cell {x},{y},{z}", block.get("loaded") is True)
            found[f"{x},{y},{z}"] = block["state"]
        return found

    def command_guards(self) -> None:
        self.call("command", f"kill {SELECTOR}")
        self.check("command kill retained carrier", self.position() == POSITION)
        self.call("command", f"tp {SELECTOR} 9 64 8")
        self.check("command teleport retained position", self.position() == POSITION)
        self.inspect_saved_entity()

    def finish(self) -> None:
        self.call("command", "save-all")
        self.call("wait", 3)
        self.call("quit", allow_error=True)
        stop = Path(self.session["mirror"]) / f"stop-{self.index:02d}.request"
        stop.write_text("owner foundation packet finished\n")
        receipt = Path(self.session["mirror"]) / f"exit-{self.index:02d}.json"
        deadline = time.monotonic() + 120
        while not receipt.exists() and time.monotonic() < deadline:
            time.sleep(0.5)
        self.check("normal session exit receipt", receipt.exists())
        exits = json.loads(receipt.read_text())
        self.check("server and peer exit zero", exits["codes"] == {"server": 0, "peer": 0})
        save_json(self.out / "result.json", {"phase": self.phase, "checks": self.checks,
                   "exit": exits, "source_head": SOURCE})

    def run(self) -> None:
        self.ready_near_fixture()
        before_blocks = self.world_cells()
        if self.phase == "cold":
            first = Path(self.session["mirror"]) / "owner-check-00" / "world-cells.json"
            self.check("cold world cells match initial run", before_blocks == json.loads(first.read_text()))
        if self.phase == "initial":
            self.call("command", fixture_command())
        self.inspect_probes()
        self.inspect_saved_entity()
        if self.phase == "cold":
            self.command_guards()
        self.check("fixture world cells unchanged", self.world_cells() == before_blocks)
        save_json(self.out / "world-cells.json", before_blocks)
        self.finish()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("session", type=Path, help="mirror session-00/01.json from session.py")
    parser.add_argument("--phase", choices=("initial", "cold"), required=True)
    args = parser.parse_args()
    packet = Packet(args.session, args.phase)
    try:
        packet.run()
    except BaseException as error:
        save_json(packet.out / "failure.json", {"error": repr(error), "checks": packet.checks})
        raise


if __name__ == "__main__":
    main()
