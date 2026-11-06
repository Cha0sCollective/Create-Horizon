#!/usr/bin/env python3
"""Build the exact published or privately pinned Mixins CLI in release mode."""
import hashlib
import io
import json
import stat
import tarfile
import uuid
from urllib.request import urlopen
from common import ROOT, PINS, cargo
from mixins_source import private_manifest, materialize_private, verify_private
from mixins_source import confined_file

PROFILE = "release"


def _unredirected_stat(path):
    """Check leaves below a configured cache root, including Windows junctions."""
    try:
        info = path.lstat()
    except FileNotFoundError:
        return None
    if stat.S_ISLNK(info.st_mode) or getattr(info, "st_file_attributes", 0) & stat.FILE_ATTRIBUTE_REPARSE_POINT:
        raise SystemExit("Refusing redirected Mixins cache path: " + str(path))
    return info


def _binary_path(target):
    # ROOT/target itself may be the configured shared-cache symlink. Its resolved
    # destination is the boundary; no redirected descendants may escape it.
    if (ROOT / "target").resolve(strict=True) != target or target.resolve(strict=True) != target:
        raise SystemExit("Mixins target directory changed during setup")
    directory = target / PROFILE
    info = _unredirected_stat(directory)
    if info is not None and not stat.S_ISDIR(info.st_mode):
        raise SystemExit("Mixins profile directory is not a directory")
    if directory.resolve() != directory:
        raise SystemExit("Refusing redirected Mixins profile directory")
    binary = directory / "horizon-mixins"
    info = _unredirected_stat(binary)
    if info is not None and not stat.S_ISREG(info.st_mode):
        raise SystemExit("Mixins CLI is not a regular file")
    if binary.resolve() != binary or not binary.is_relative_to(target):
        raise SystemExit("Mixins CLI escapes the resolved target directory")
    return binary


def _sha256(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def build_release(source, revision):
    configured = ROOT / "target"
    configured.mkdir(parents=True, exist_ok=True)
    target = configured.resolve(strict=True)
    if target.parent == target:
        raise SystemExit("Refusing the filesystem root as the Mixins target directory")
    binary = _binary_path(target)
    tools = (ROOT / ".tools").resolve(strict=True)
    receipt = tools / "mixins-built-release.json"
    info = _unredirected_stat(receipt)
    if info is not None and not stat.S_ISREG(info.st_mode):
        raise SystemExit("Mixins build receipt is not a regular file")
    recorded = None
    if info is not None:
        try:
            recorded = json.loads(receipt.read_text())
        except (ValueError, UnicodeError):
            pass
    expected = {"source_revision": revision, "profile": PROFILE}
    if binary.exists() and recorded == {**expected, "binary_sha256": _sha256(binary)}:
        return binary
    if binary.exists():
        # A shared Cargo target can contain foreign bytes while its fingerprints
        # still say Fresh. Make the expected output absent before asking Cargo to
        # rebuild, preserving only that old file beside the intended binary.
        binary = _binary_path(target)
        quarantine = binary.with_name(binary.name + ".unverified-" + uuid.uuid4().hex)
        if _unredirected_stat(quarantine) is not None or quarantine.resolve() != quarantine or not quarantine.is_relative_to(target):
            raise SystemExit("Unsafe Mixins binary quarantine path")
        binary.rename(quarantine)
        print("Preserved unverified Mixins CLI at " + str(quarantine))
    cargo("build", "--release", "--locked", "--jobs", "4",
          "--manifest-path", source / "Cargo.toml", "-p", "horizon-mixins-cli")
    binary = _binary_path(target)
    if not binary.is_file():
        raise SystemExit("Mixins release build did not produce the expected CLI")
    receipt.write_text(json.dumps({**expected, "binary_sha256": _sha256(binary)}, sort_keys=True) + "\n")
    return binary


def upstream_source(revision):
    root = ROOT / ".tools" / ("horizon-mixins-" + revision[:12])
    if not root.exists():
        with urlopen("https://codeload.github.com/horizon-rust/horizon-mixins/tar.gz/" + revision, timeout=60) as response:
            data = response.read()
        root.mkdir(parents=True)
        with tarfile.open(fileobj=io.BytesIO(data)) as package:
            for member in package.getmembers():
                parts = member.name.split("/")[1:]
                if not parts or parts == [""]: continue
                if member.issym() or member.islnk() or ".." in parts:
                    raise SystemExit("Unexpected Mixins archive entry")
                target = confined_file(root, "/".join(parts))
                if member.isdir(): target.mkdir(parents=True, exist_ok=True)
                elif member.isfile():
                    target.parent.mkdir(parents=True, exist_ok=True)
                    target.write_bytes(package.extractfile(member).read())
        (root / ".revision").write_text(revision)
    if (root / ".revision").read_text().strip() != revision:
        raise SystemExit("Mixins source revision differs from compatibility.json")
    return root


def main():
    revision = PINS["horizon_mixins_revision"]
    if "horizon_mixins_overlay_sha256" in PINS:
        overlay, manifest = private_manifest(ROOT, PINS)
        root = ROOT / ".tools" / ("horizon-mixins-private-" + revision[:12])
        if not root.exists():
            base = upstream_source(manifest["base_revision"])
            materialize_private(base, root, overlay, manifest)
        verify_private(root, manifest["source_tree_sha256"])
        if (root / ".revision").read_text().strip() != revision:
            raise SystemExit("Private Mixins source revision marker differs")
    else:
        root = upstream_source(revision)
    return build_release(root, revision)


if __name__ == "__main__":
    main()
