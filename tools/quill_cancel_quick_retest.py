"""Focused Quill cancel then quick-edge retest in the saved integrated run.

Root launches the normal session.py `integrated --run <saved-run>` first.
This adapter neither builds nor launches a game. PNG contents need review.
"""

from pathlib import Path
import argparse
import hashlib
import json
import subprocess
import time


EXPECTED_RUN = "/home/arbiter/.cache/create-horizon/team-runtime-005/integrated-9vkqdlrp"
EXPECTED_SOURCE = "9651dba12d163896d45a8cc4ba1b0b3b0da1feff"
EXPECTED_NATIVE_TREE = "90c2156ed5ffb38131950581f05cbdc76beba9ba"
EXPECTED_MAP = "c912836f0ec11311340a4dc0e7f8748abf25ee1a46fba17df4c2c5a33c790647"
EXPECTED_GAME = "b862982154b5ad4cde22e95618920fbec3f6d61f11ed8984357254576c139e6e"
EXPECTED_CTL = "170e866057d97e73b9231c8084547d4a4a02346238db35901ef64bf3d0d6d449"
EXPECTED_JAR = "ef87fe5709f1ba1f5b8bb20a2925bafb4669e178fd6d8bf10c167759eefe37a"


def sha(path):
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


parser = argparse.ArgumentParser()
parser.add_argument("session", type=Path)
parser.add_argument("ctl", type=Path)
parser.add_argument("source_map", type=Path)
parser.add_argument("jar", type=Path)
parser.add_argument("--origin", nargs=3, type=int, required=True,
                    metavar=("X", "Y", "Z"))
args = parser.parse_args()
session_path = args.session.resolve(strict=True)
session = json.loads(session_path.read_text())
assert session["mode"] == "integrated" and session["index"] >= 2
assert session["run"] == EXPECTED_RUN
assert session["source_revision"] == EXPECTED_SOURCE
assert session["native_tree"] == EXPECTED_NATIVE_TREE
assert session["source_map_sha256"] == EXPECTED_MAP
assert session["binary_sha256"] == {"horizon-client": EXPECTED_GAME,
                                    "horizon-ctl": EXPECTED_CTL}
assert session["jar_sha256"] == EXPECTED_JAR
assert list(session["processes"]) == ["actor"]
ctl = args.ctl.resolve(strict=True)
source_map = args.source_map.resolve(strict=True)
jar = args.jar.resolve(strict=True)
out = (Path(session["mirror"]) / "quill").resolve()
assert session_path.parent == out.parent
out.mkdir(parents=True, exist_ok=True)
prior = json.loads((out / "integrated-pre-unload.json").read_text())
assert prior["mode"] == "integrated" and prior["run"] == session["run"]
assert prior["origin"] == args.origin
assert prior["index"] < session["index"]
receipt_path = out / f"integrated-{session['index']:02d}-cancel-quick.json"
assert not receipt_path.exists(), receipt_path
x, y, z = args.origin
assert prior["region"] == {
    "blocks": [[x + offset, y, z, "minecraft:stone"]
               for offset in (0, 1, 15, 16)],
    "counts": {"minecraft:stone": 4},
    "from": [x, y, z],
    "to": [x + 16, y, z],
}, "saved fixture receipt is not the fixed four-stone line"
assert prior["counts"] == {"create:schematic_and_quill": 1,
                           "minecraft:stick": 1}, "saved actor counts differ"
steps = []
frames = []
receipt = {"session": str(session_path), "run": session["run"],
           "index": session["index"], "origin": args.origin,
           "source_revision": session["source_revision"],
           "native_tree": session["native_tree"], "steps": steps,
           "hud_frames": frames, "status": "RUNNING"}


def verify_inputs():
    assert sha(source_map) == session["source_map_sha256"]
    frozen = json.loads(source_map.read_text())
    assert frozen["git_head"] == session["source_revision"]
    assert frozen["native_tree"] == session["native_tree"]
    assert len(frozen["files"]) == 711
    root = Path(frozen["source_root"])
    bad = [name for name, digest in frozen["files"].items()
           if sha(root / name) != digest]
    assert not bad, bad[:5]
    assert sha(Path(session["processes"]["actor"]["game"])) == session["binary_sha256"]["horizon-client"]
    assert sha(ctl) == session["binary_sha256"]["horizon-ctl"]
    assert sha(jar) == session["jar_sha256"]
    return {"source_files": len(frozen["files"]), "mismatches": 0,
            "game": session["binary_sha256"]["horizon-client"],
            "ctl": session["binary_sha256"]["horizon-ctl"],
            "jar": session["jar_sha256"]}


def call(*words):
    started = time.monotonic()
    process = subprocess.run([str(ctl), "--socket",
        session["processes"]["actor"]["socket"], *map(str, words)],
        capture_output=True, text=True, timeout=150)
    steps.append({"words": list(map(str, words)), "code": process.returncode,
                  "stdout": process.stdout, "stderr": process.stderr,
                  "seconds": round(time.monotonic() - started, 3)})
    if process.returncode:
        raise RuntimeError(f"{words}: {process.stdout} {process.stderr}")
    return json.loads(process.stdout)


def aim(offset):
    px, py, pz = x + offset + .5, y, z - 2.5
    call("command", f"tp @s {px} {py} {pz}")
    deadline = time.monotonic() + 20
    while time.monotonic() < deadline:
        position = call("status")["player"]["position"]
        if all(abs(position[i] - value) < .2
               for i, value in enumerate((px, py, pz))):
            break
        time.sleep(.1)
    else:
        raise TimeoutError(f"teleport did not arrive: {(px, py, pz)}")
    call("wait-ready", 120)
    assert call("look-at", "block", x + offset, y, z, "north")["face"] == "north"
    hit = call("looking-at")
    assert hit["kind"] == "block" and hit["block"]["pos"] == [x + offset, y, z]
    assert hit["block"]["face"] == "north"


def shot(label, expected):
    path = out / f"integrated-{session['index']:02d}-{label}-actor.png"
    assert not path.exists(), path
    call("wait", 3)
    call("screenshot", path)
    assert path.is_file() and path.stat().st_size > 1024, path
    frames.append({"label": label, "path": str(path),
                   "expected_visible_text": expected,
                   "inspection": "PENDING: compare rendered pixels"})


def region():
    return call("blocks", x, y, z, x + 16, y, z)


def counts():
    return call("inventory")["counts"]


try:
    receipt["integrity_before"] = verify_inputs()
    call("wait-ready", 120)
    assert region() == prior["region"]
    assert counts() == prior["counts"]
    assert call("select", "create:schematic_and_quill")["held"]["item"] == "create:schematic_and_quill"

    # Establish the completed pair explicitly; no prior client UI state is assumed.
    aim(16)
    call("mouse", "right", "click")
    call("wait", 3)
    aim(1)
    call("mouse", "right", "click")
    shot("before-cancel", [f"First: {x + 16}, {y}, {z}",
                           f"Second: {x + 1}, {y}, {z}", "Size: 16 x 1 x 1"])

    call("key", "shift", "down")
    try:
        assert call("state")["player"]["sneaking"] is True
        call("mouse", "right", "click")
        call("wait", 3)  # consume queued use while Shift is still down
    finally:
        call("key", "shift", "up")
    assert call("state")["player"]["sneaking"] is False
    shot("after-cancel", ["Selection canceled",
                          "Right-click a block: first corner"])

    aim(16)
    call("mouse", "right", "click")
    call("mouse", "right", "click")
    shot("quick-edges", [f"First: {x + 16}, {y}, {z}",
                         f"Second: {x + 16}, {y}, {z}", "Size: 1 x 1 x 1"])
    assert region() == prior["region"], "selected-line blocks changed"
    assert counts() == prior["counts"], "inventory counts changed"
    receipt["integrity_after"] = verify_inputs()
    receipt["status"] = "AUTOMATED_CHECKS_PASS_HUD_INSPECTION_PENDING"
except BaseException as error:
    receipt["status"] = "FAIL"
    receipt["error"] = repr(error)
    raise
finally:
    with receipt_path.open("x") as stream:
        json.dump(receipt, stream, indent=2)
        stream.write("\n")
    print(json.dumps({"status": receipt["status"],
                      "receipt": str(receipt_path),
                      "hud_frames": len(frames)}), flush=True)
