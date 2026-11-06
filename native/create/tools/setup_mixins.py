#!/usr/bin/env python3
"""Build the exact published or privately pinned Mixins source."""
import io
import tarfile
from urllib.request import urlopen
from common import ROOT, PINS, cargo
from mixins_source import private_manifest, materialize_private, verify_private
from mixins_source import confined_file


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
    built = ROOT / ".tools/mixins-built-revision"
    if not built.exists() or built.read_text().strip() != revision or not (ROOT / "target/debug/horizon-mixins").exists():
        cargo("build", "--locked", "--manifest-path", root / "Cargo.toml", "-p", "horizon-mixins-cli")
        built.write_text(revision + "\n")
    return root


if __name__ == "__main__":
    main()
