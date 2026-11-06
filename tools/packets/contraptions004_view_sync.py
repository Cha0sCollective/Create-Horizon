"""Finite, disposable CONTRAPTIONS-004 owner VIEW sync runtime check.

Attach to an already launched, newly pinned dedicated server/peer session.
This script starts no game or build and touches no world blocks.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import shutil
import signal
import subprocess
import time
import uuid
from pathlib import Path


HELD_SOURCE = "9651dba12d163896d45a8cc4ba1b0b3b0da1feff"
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
    def __init__(self, session_path: Path, phase: str, source_head: str, packet_root: Path):
        session_path = session_path.resolve()
        self.session = json.loads(session_path.read_text())
        session = self.session
        assert session["mode"] == "dedicated" and set(session["processes"]) == {"server", "peer"}
        assert session["source_head"] == source_head and session["source_revision"] == source_head
        self.source_head = source_head
        self.index = session["index"]
        assert self.index == (0 if phase == "initial" else 1), (phase, self.index)
        self.phase = phase
        mirror = Path(session["mirror"]).resolve()
        assert mirror.parent == packet_root.resolve(), mirror
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
            "expected_source_head": source_head,
            "packet_root": str(packet_root.resolve()),
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

    def chat_reply(self, command: str, needle: str | tuple[str, ...]) -> str:
        marker = "OWNER004_" + uuid.uuid4().hex[:12]
        self.call("command", "say " + marker)
        self.call("command", command)
        for _ in range(30):
            lines = self.call("chat-log", 80).get("chat", [])
            markers = [i for i, line in enumerate(lines) if marker in line]
            if markers:
                fresh = [line for line in lines[:markers[0]] if marker not in line]
                joined = " ".join(fresh)
                expected = (needle,) if isinstance(needle, str) else needle
                if any(item in joined for item in expected):
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

    def capture_peer_gates(self) -> list[dict]:
        gates = []
        if self.peer_log.is_file():
            for line in self.peer_log.read_text(errors="replace").splitlines():
                if "Create owner probe: " not in line:
                    continue
                try:
                    payload = json.loads(line.split("Create owner probe: ", 1)[1])
                except json.JSONDecodeError:
                    continue
                if payload.get("event") == "peer_view_gate":
                    gates.append(payload)
        save_json(self.out / "peer-gates.json", gates)
        return gates

    def trace(self, role: str, event: str) -> dict:
        path = self.server_log if role == "server" else self.peer_log
        deadline = time.monotonic() + 90
        while time.monotonic() < deadline:
            for line in path.read_text(errors="replace").splitlines():
                if "Create owner probe: " in line:
                    payload = json.loads(line.split("Create owner probe: ", 1)[1])
                    if payload.get("event") == event:
                        if role == "peer":
                            self.capture_peer_gates()
                        save_json(self.out / f"{role}-{event}.json", payload)
                        return payload
            time.sleep(0.5)
        gates = self.capture_peer_gates()
        last_gate = gates[-1] if gates else None
        raise AssertionError(f"missing {role} {event} probe line: {path}; last gate={last_gate}")

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
        queued = self.trace("server", "server_view_sync_queued")
        self.check("saved VIEW queued for tracker", queued["uuid"] == [5, 6, 7, 8]
                   and queued["assembly_id"] == [1, 2, 3, 4])
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
        kill = self.chat_reply(f"kill {SELECTOR}",
                               ("Killed ", "commands.kill.success.single"))
        self.check("command kill retained carrier", self.position() == POSITION)
        teleport = self.chat_reply(f"tp {SELECTOR} 9 64 8",
                                   ("Teleported ", "commands.teleport.success.location.single"))
        self.check("command teleport retained position", self.position() == POSITION)
        save_json(self.out / "command-guard-feedback.json", {"kill": kill, "teleport": teleport})
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
                   "exit": exits, "source_head": self.source_head})

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


def peer_alive(pid: int) -> bool:
    try:
        os.kill(pid, 0)
        return True
    except OSError:
        return False


def verified_peer(session: dict) -> bool:
    """Only the exact disposable child recorded by session.py may be signalled."""
    peer = session["processes"]["peer"]
    pid = int(peer["pid"])
    try:
        words = [item.decode() for item in Path(f"/proc/{pid}/cmdline").read_bytes().split(b"\0") if item]
        cwd = Path(f"/proc/{pid}/cwd").resolve()
        return (bool(words) and Path(words[0]).resolve() == Path(peer["game"]).resolve()
                and peer["socket"] in words
                and cwd.name == "contraptions-004-probe-client")
    except (OSError, ValueError):
        return False


def cleanup_failed_session(session_path: Path, error: BaseException, out: Path | None,
                           packet_root: Path) -> None:
    """Preserve the original failure and release a verified disposable session."""
    receipt: dict = {"original_error": repr(error), "actions": []}
    target = out / "failure.json" if out and out.is_dir() else None
    try:
        path = session_path.resolve()
        session = json.loads(path.read_text())
        mirror = Path(session["mirror"]).resolve()
        index = int(session["index"])
        assert (session["mode"] == "dedicated" and mirror.parent == packet_root.resolve()
                and path == mirror / f"session-{index:02d}.json")
        if target is None:
            target = mirror / f"owner-check-{index:02d}-failure.json"
        try:
            peer = session["processes"]["peer"]
            ctl = Path(peer["game"]).with_name("horizon-ctl")
            if ctl.is_file() and sha256(ctl) == session["binary_sha256"]["horizon-ctl"]:
                try:
                    result = subprocess.run(
                        [str(ctl), "--socket", peer["socket"], "--timeout", "10", "quit"],
                        capture_output=True, text=True, timeout=20)
                    receipt["actions"].append({"peer_quit_code": result.returncode,
                                               "stdout": result.stdout, "stderr": result.stderr})
                except (OSError, subprocess.TimeoutExpired) as quit_error:
                    receipt["actions"].append({"peer_quit_error": repr(quit_error)})
            else:
                receipt["actions"].append({"peer_quit_skipped": "frozen control binary unavailable"})
            pid = int(peer["pid"])
            deadline = time.monotonic() + 10
            while peer_alive(pid) and time.monotonic() < deadline:
                time.sleep(0.25)
            if peer_alive(pid):
                if verified_peer(session):
                    try:
                        os.kill(pid, signal.SIGTERM)
                        receipt["actions"].append({"peer_sigterm": pid})
                    except OSError as signal_error:
                        receipt["actions"].append({"peer_sigterm_error": repr(signal_error)})
                else:
                    receipt["actions"].append({"peer_sigterm_skipped": "PID identity mismatch"})
        except BaseException as peer_cleanup_error:
            receipt["actions"].append({"peer_cleanup_error": repr(peer_cleanup_error)})
        finally:
            stop = mirror / f"stop-{index:02d}.request"
            stop.write_text("owner foundation packet failed; bounded stop requested\n")
            receipt["actions"].append({"stop_request": str(stop)})
        exit_path = mirror / f"exit-{index:02d}.json"
        deadline = time.monotonic() + 120
        while not exit_path.exists() and time.monotonic() < deadline:
            time.sleep(0.5)
        receipt["exit"] = json.loads(exit_path.read_text()) if exit_path.exists() else "missing after 120s"
        save_json(target, receipt)
    except BaseException as cleanup_error:
        receipt["cleanup_error"] = repr(cleanup_error)
        if target is not None:
            try:
                save_json(target, receipt)
            except OSError:
                pass


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("session", type=Path, help="mirror session-00/01.json from session.py")
    parser.add_argument("--phase", choices=("initial", "cold"), required=True)
    parser.add_argument("--source-head", required=True, help="new frozen integration Git head")
    parser.add_argument("--packet-root", required=True, type=Path,
                        help="prepared disposable mirror parent from Operations")
    args = parser.parse_args()
    packet = None
    try:
        assert re.fullmatch(r"[0-9a-f]{40}", args.source_head) and args.source_head != HELD_SOURCE
        assert args.packet_root.is_dir()
        assert args.packet_root.resolve().parent == Path("/mnt/g/Create Rusty/.local").resolve()
        packet = Packet(args.session, args.phase, args.source_head, args.packet_root)
        packet.run()
    except BaseException as error:
        if packet is not None:
            try:
                packet.capture_peer_gates()
            except BaseException:
                pass
        cleanup_failed_session(args.session, error, packet.out if packet else None,
                               args.packet_root)
        raise


if __name__ == "__main__":
    main()
