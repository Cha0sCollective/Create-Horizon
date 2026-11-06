"""Quill UI milestone against a root-frozen disposable Horizon session.

This script neither launches a process nor builds anything. Run `full`, unload
the world normally, relaunch the same disposable run, then run `reopen`.
Screenshots need human inspection for the expected HUD text in the receipt.
"""

from pathlib import Path
import argparse
import hashlib
import json
import subprocess
import time


def digest(path):
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


parser = argparse.ArgumentParser()
parser.add_argument("session", type=Path, help="root's frozen session JSON")
parser.add_argument("ctl", type=Path, help="matching absolute horizon-ctl path")
parser.add_argument("case", choices=("full", "reopen"))
parser.add_argument("--origin", nargs=3, type=int, required=True, metavar=("X", "Y", "Z"),
                    help="x/y/z of the first target in a disposable Creative world")
parser.add_argument("--out", type=Path, help="receipt directory; default: session mirror/quill")
args = parser.parse_args()

session_path = args.session.resolve(strict=True)
session = json.loads(session_path.read_text())
assert session["mode"] in ("integrated", "dedicated"), session
assert "actor" in session["processes"]
assert session["mode"] == "integrated" or "peer" in session["processes"]
assert session["source_revision"] and session["native_tree"]
ctl = args.ctl.resolve(strict=True)
assert ctl.is_absolute() and digest(ctl) == session["binary_sha256"]["horizon-ctl"]
game = Path(session["processes"]["actor"]["game"])
assert digest(game) == session["binary_sha256"]["horizon-client"]
out = (args.out or Path(session["mirror"]) / "quill").resolve()
out.mkdir(parents=True, exist_ok=True)
receipt_path = out / f"{session['mode']}-{session['index']:02d}-{args.case}.json"
assert not receipt_path.exists(), receipt_path
pre_unload_path = out / f"{session['mode']}-pre-unload.json"
x, y, z = args.origin
points = (0, 1, 15, 16)
quill = "create:schematic_and_quill"
steps = []
frames = []
result = {"case": args.case, "session": str(session_path), "mode": session["mode"],
          "index": session["index"], "source_revision": session["source_revision"],
          "native_tree": session["native_tree"], "origin": args.origin,
          "steps": steps, "hud_frames": frames,
          "status": "RUNNING"}


def call(role, *words):
    started = time.monotonic()
    reply = subprocess.run(
        [str(ctl), "--socket", session["processes"][role]["socket"], *map(str, words)],
        capture_output=True, text=True, timeout=150,
    )
    entry = {"role": role, "words": list(map(str, words)),
             "seconds": round(time.monotonic() - started, 3), "code": reply.returncode,
             "stdout": reply.stdout, "stderr": reply.stderr}
    steps.append(entry)
    if reply.returncode:
        raise RuntimeError(f"{role} {words}: {reply.stdout} {reply.stderr}")
    return json.loads(reply.stdout)


def actor(*words):
    return call("actor", *words)


def command(text, role="actor"):
    return call(role, "command", text)


def teleport(role, px, py, pz):
    command(f"tp @s {px} {py} {pz}", role)
    for _ in range(80):
        position = call(role, "status")["player"]["position"]
        if all(abs(position[i] - value) < .2 for i, value in enumerate((px, py, pz))):
            break
        time.sleep(.1)
    else:
        raise TimeoutError(f"{role} did not reach {(px, py, pz)}")
    call(role, "wait-ready", 120)


def aim(role, offset):
    assert offset in points
    teleport(role, x + offset + .5, y, z - 2.5)
    answer = call(role, "look-at", "block", x + offset, y, z, "north")
    assert answer["face"] == "north", answer
    hit = call(role, "looking-at")
    assert hit["kind"] == "block", hit
    assert hit["block"]["pos"] == [x + offset, y, z], hit
    assert hit["block"]["face"] == "north", hit


def selected(role, item):
    answer = call(role, "select", item)
    assert answer["held"]["item"] == item, answer
    return answer


def launch_arg(role, name):
    words = session["processes"][role]["command"]
    return words[words.index(name) + 1]


def shot(role, label, expected, style=None):
    path = out / f"{session['mode']}-{session['index']:02d}-{label}-{role}.png"
    assert not path.exists(), path
    call(role, "wait", 3)
    call(role, "screenshot", path)
    assert path.is_file() and path.stat().st_size > 1024, path
    frames.append({"role": role, "label": label, "path": str(path),
                   "expected_visible_text": expected, "expected_style": style,
                   "inspection": "PENDING: compare actual rendered pixels"})


def region():
    return actor("blocks", x, y, z, x + 16, y, z)


def counts(role="actor"):
    return call(role, "inventory")["counts"]


def await_value(label, read, accepted, seconds=20):
    deadline = time.monotonic() + seconds
    last = None
    while time.monotonic() < deadline:
        last = read()
        if accepted(last):
            return last
        time.sleep(.1)
    raise TimeoutError(f"{label} did not settle: {last!r}")


def assert_unchanged(before_region, before_counts):
    assert region() == before_region, "Quill changed fixture blocks"
    assert counts() == before_counts, "Quill changed actor inventory counts"


try:
    actor("wait-ready", 120)
    if args.case == "full":
        assert not pre_unload_path.exists(), pre_unload_path
        # This bounded fixture is only for root's disposable world.
        command(f"fill {x - 4} {y - 1} {z - 3} {x + 19} {y - 1} {z + 4} minecraft:stone")
        command(f"fill {x - 4} {y} {z - 3} {x + 19} {y + 3} {z + 4} minecraft:air")
        for offset in points:
            command(f"setblock {x + offset} {y} {z} minecraft:stone")
        command(f"setblock {x + 4} {y} {z + 2} "
                "minecraft:lever[face=floor,facing=north,powered=false]")
        expected_blocks = [[x + offset, y, z, "minecraft:stone"] for offset in points]
        await_value("four target blocks", region,
                    lambda data: data["blocks"] == expected_blocks)
        await_value("unpowered lever", lambda: actor("block", x + 4, y, z + 2),
                    lambda data: "powered=false" in data["state"])
        # Clearing the original terrain makes leaf litter item drops. Remove
        # them before the inventory baseline so later teleporting cannot pick
        # up fixture debris and masquerade as a Quill item-accounting failure.
        teleport("actor", x + 8.5, y, z - 2.5)
        command("kill @e[type=minecraft:item]")
        command("clear @s")
        await_value("empty actor inventory", counts, lambda data: not data)
        await_value("no fixture item drops",
                    lambda: actor("entities", 32, "minecraft:item"),
                    lambda data: not data["entities"])
        command("gamemode creative @s")
        await_value("actor Creative", lambda: actor("status"),
                    lambda data: data["creative"] is True)
        command(f"give @s {quill} 1")
        command("give @s minecraft:stick 1")
        await_value("actor Quill and stick", counts,
                    lambda data: data.get(quill) == 1 and data.get("minecraft:stick") == 1)
        selected("actor", quill)
        if "peer" in session["processes"]:
            call("peer", "wait-ready", 120)
            command("gamemode creative @s", "peer")
            await_value("peer Creative", lambda: call("peer", "status"),
                        lambda data: data["creative"] is True)
            command("clear @s", "peer")
            await_value("empty peer inventory", lambda: counts("peer"),
                        lambda data: not data)
            command(f"give @s {quill} 1", "peer")
            await_value("peer Quill", lambda: counts("peer"),
                        lambda data: data.get(quill) == 1)
            selected("peer", quill)
        before_region, before_counts = region(), counts()
        peer_counts = counts("peer") if "peer" in session["processes"] else None
        assert before_counts.get(quill) == 1, before_counts

        # A single held press may trigger repeated use callbacks, but only one corner.
        aim("actor", 16)
        actor("mouse", "right", "down")
        actor("wait", 18)
        shot("actor", "held-first", [f"First: {x + 16}, {y}, {z}",
                                      "Size: 1 x 1 x 1 (max 16 each)"])
        actor("mouse", "right", "up")
        aim("actor", 0)
        shot("actor", "preview-17", ["Size: 17 x 1 x 1 (max 16 each)"], "red")
        actor("mouse", "right", "click")
        shot("actor", "refused-17", [f"First: {x + 16}, {y}, {z}",
                                        "Selection exceeds 16 blocks on an axis"])
        aim("actor", 1)
        shot("actor", "preview-16", ["Size: 16 x 1 x 1 (max 16 each)"], "green")
        actor("mouse", "right", "click")
        shot("actor", "reversed-16", [f"First: {x + 16}, {y}, {z}",
                                        f"Second: {x + 1}, {y}, {z}", "Size: 16 x 1 x 1"])
        actor("mouse", "right", "click")
        shot("actor", "third-click", ["Selection ready; saving is not available yet"])
        assert_unchanged(before_region, before_counts)

        actor("key", "shift", "down")
        actor("mouse", "right", "click")
        actor("key", "shift", "up")
        shot("actor", "cancel", ["Selection canceled",
                                  "Right-click a block: first corner"])
        aim("actor", 16)
        actor("mouse", "right", "click")
        actor("mouse", "right", "click")
        shot("actor", "quick-edges", [f"First: {x + 16}, {y}, {z}",
                                       f"Second: {x + 16}, {y}, {z}", "Size: 1 x 1 x 1"])
        actor("key", "shift", "down")
        actor("mouse", "right", "click")
        actor("key", "shift", "up")

        aim("actor", 16)
        actor("mouse", "right", "click")
        selected("actor", "minecraft:stick")
        shot("actor", "item-away", [])
        selected("actor", quill)
        shot("actor", "item-return", ["Right-click a block: first corner"])
        actor("mouse", "right", "click")
        actor("open-inventory")
        assert actor("state")["screen"] == "creative"
        actor("wait", 3)
        actor("close")
        assert actor("state")["screen"] == "playing"
        shot("actor", "screen-return", ["Right-click a block: first corner"])
        assert_unchanged(before_region, before_counts)

        selected("actor", "minecraft:stick")
        teleport("actor", x + 4.5, y, z + 3.5)
        actor("look-at", "block", x + 4, y, z + 2, "up")
        lever_before = actor("block", x + 4, y, z + 2)
        assert "powered=false" in lever_before["state"], lever_before
        actor("mouse", "right", "click")
        await_value("lever powered by stick click",
                    lambda: actor("block", x + 4, y, z + 2),
                    lambda data: "powered=true" in data["state"])
        assert_unchanged(before_region, before_counts)

        # Peer-local selection: one client's first corner cannot change the other.
        selected("actor", quill)
        aim("actor", 16)
        actor("mouse", "right", "click")
        if "peer" in session["processes"]:
            shot("peer", "peer-before-own-click", ["Right-click a block: first corner"])
            aim("peer", 0)
            call("peer", "mouse", "right", "click")
            shot("peer", "peer-first", [f"First: {x}, {y}, {z}"])
            shot("actor", "actor-still-first", [f"First: {x + 16}, {y}, {z}"])
            actor("key", "shift", "down")
            actor("mouse", "right", "click")
            actor("key", "shift", "up")
            shot("peer", "peer-after-actor-cancel", [f"First: {x}, {y}, {z}"])
            aim("actor", 16)
            actor("mouse", "right", "click")
            assert counts("peer") == peer_counts, "Quill changed peer inventory counts"
        shot("actor", "before-same-process-unload", [f"First: {x + 16}, {y}, {z}"])
        if session["mode"] == "integrated":
            actor("key", "esc", "tap")
            await_value("paused menu", lambda: actor("state"),
                        lambda data: data["screen"] == "paused")
            pause_frame = out / f"{session['mode']}-{session['index']:02d}-pause.png"
            assert not pause_frame.exists(), pause_frame
            actor("screenshot", pause_frame)
            actor("click-button", "Save and Quit to Title")
            await_value("title after world leave", lambda: actor("state"),
                        lambda data: data["screen"] == "title", seconds=90)
            actor("open", launch_arg("actor", "--world"))
        else:
            actor("disconnect")
            await_value("title after disconnect", lambda: actor("state"),
                        lambda data: data["screen"] == "title", seconds=30)
            actor("connect", f"127.0.0.1:{session['port']}",
                  launch_arg("actor", "--name"))
        actor("wait-ready", 120)
        await_value("same-process Quill return", counts,
                    lambda data: data.get(quill) == 1)
        selected("actor", quill)
        shot("actor", "same-process-world-reset",
             ["Right-click a block: first corner"])
        assert_unchanged(before_region, before_counts)
        if "peer" in session["processes"]:
            shot("peer", "peer-after-actor-rejoin", [f"First: {x}, {y}, {z}"])
        aim("actor", 16)
        actor("mouse", "right", "click")
        shot("actor", "before-world-unload", [f"First: {x + 16}, {y}, {z}"])
        assert_unchanged(before_region, before_counts)
        pre_unload_path.write_text(json.dumps({"mode": session["mode"],
            "run": session["run"], "index": session["index"],
            "origin": args.origin, "region": before_region,
            "counts": before_counts, "peer_counts": peer_counts}, indent=2) + "\n")
    else:
        prior = json.loads(pre_unload_path.read_text())
        assert prior["mode"] == session["mode"] and prior["run"] == session["run"]
        assert prior["index"] != session["index"] and prior["origin"] == args.origin
        await_value("reopened actor Quill", counts,
                    lambda data: data.get(quill) == 1)
        selected("actor", quill)
        shot("actor", "after-world-reopen", ["Right-click a block: first corner"])
        assert region() == prior["region"], "world fixture changed across reopen"
        assert counts() == prior["counts"], "inventory changed across reopen"
        if "peer" in session["processes"]:
            await_value("reopened peer Quill", lambda: counts("peer"),
                        lambda data: data.get(quill) == 1)
            selected("peer", quill)
            shot("peer", "peer-after-world-reopen", ["Right-click a block: first corner"])
            assert counts("peer") == prior["peer_counts"], "peer inventory changed across reopen"
    result["status"] = "AUTOMATED_CHECKS_PASS_HUD_INSPECTION_PENDING"
except BaseException as exc:
    result["status"] = "FAIL"
    result["error"] = repr(exc)
    raise
finally:
    with receipt_path.open("x") as stream:
        json.dump(result, stream, indent=2)
        stream.write("\n")
    print(json.dumps({"status": result["status"], "receipt": str(receipt_path),
                      "hud_frames": len(frames)}), flush=True)
