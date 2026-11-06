"""Materialize an exact private Mixins overlay without changing its upstream base."""
import hashlib
import json
from pathlib import Path, PurePosixPath
import re
import shutil


def redirected(path):
    return path.is_symlink() or getattr(path, "is_junction", lambda: False)()


def tree_sha256(root):
    """Hash sorted relative paths and bytes; only our root revision marker is omitted."""
    if redirected(root):
        raise SystemExit("Mixins source root must not be redirected")
    digest = hashlib.sha256()
    for path in sorted(root.rglob("*"), key=lambda p: p.relative_to(root).as_posix()):
        if redirected(path):
            raise SystemExit("Mixins source must not contain links")
        relative = path.relative_to(root).as_posix()
        if path.is_file() and relative != ".revision":
            digest.update(relative.encode("utf-8") + b"\0")
            digest.update(hashlib.sha256(path.read_bytes()).digest())
    return digest.hexdigest()


def confined_file(root, name):
    relative = PurePosixPath(name)
    if (not name or relative.is_absolute() or ".." in relative.parts
            or "\\" in name or ":" in name or relative.as_posix() != name):
        raise SystemExit("Invalid private Mixins source path")
    target = root.joinpath(*relative.parts)
    chain = [target]
    chain.extend(parent for parent in target.parents if parent.is_relative_to(root))
    if any(redirected(path) for path in chain) or not target.resolve().is_relative_to(root.resolve()):
        raise SystemExit("Redirected private Mixins source path")
    return target


def private_manifest(project, pins):
    revision = pins["horizon_mixins_revision"]
    if not re.fullmatch(r"[0-9a-f]{40}", revision):
        raise SystemExit("Invalid private Mixins source pin")
    folder = project / "tools/mixins-private"
    if redirected(folder) or not folder.resolve().is_relative_to(project.resolve()):
        raise SystemExit("Redirected private Mixins overlay directory")
    manifest_path = confined_file(folder, "source.json")
    raw = manifest_path.read_bytes()
    if hashlib.sha256(raw).hexdigest() != pins["horizon_mixins_overlay_sha256"]:
        raise SystemExit("Private Mixins overlay manifest differs from its pin")
    manifest = json.loads(raw)
    if (manifest["base_revision"] != pins["horizon_mixins_upstream_revision"]
            or not re.fullmatch(r"[0-9a-f]{40}", manifest["base_revision"])
            or manifest["source_revision"] != revision):
        raise SystemExit("Private Mixins source provenance differs from its pin")
    return folder, manifest


def materialize_private(base, destination, overlay, manifest):
    if tree_sha256(base) != manifest["base_tree_sha256"]:
        raise SystemExit("Upstream Mixins source differs from the private overlay base")
    changes = []
    seen = set()
    for entry in manifest["files"]:
        name = entry["path"]
        if name.casefold() in seen or name.casefold() == ".revision":
            raise SystemExit("Duplicate or reserved Mixins overlay path")
        seen.add(name.casefold())
        original = confined_file(base, name)
        replacement = confined_file(overlay, entry["replacement"])
        before = hashlib.sha256(original.read_bytes()).hexdigest() if original.is_file() else None
        if before != entry["base_sha256"]:
            raise SystemExit("Mixins overlay base file differs: " + name)
        changes.append((name, replacement.read_bytes()))
    # Destination is new, under the project's ignored tool directory. Never
    # alter the official source snapshot or repair an existing private snapshot.
    shutil.copytree(base, destination)
    for name, data in changes:
        target = confined_file(destination, name)
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(data)
    verify_private(destination, manifest["source_tree_sha256"])
    (destination / ".revision").write_text(manifest["source_revision"] + "\n")


def verify_private(root, expected_sha256):
    if tree_sha256(root) != expected_sha256:
        raise SystemExit("Private Mixins source tree differs from its pin")
