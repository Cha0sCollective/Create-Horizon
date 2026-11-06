"""Focused checks for private tool pinning; no compiler, network or game needed."""
import hashlib
import json
from pathlib import Path
import tempfile
import unittest

from mixins_source import materialize_private, private_manifest, tree_sha256, verify_private


class PrivateSourceTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.base = self.root / "base"
        self.base.mkdir()
        (self.base / "Cargo.toml").write_bytes(b"baseline\n")
        (self.base / ".revision").write_text("a" * 40)
        self.overlay = self.root / "tools/mixins-private"
        self.overlay.mkdir(parents=True)
        (self.overlay / "Cargo.toml").write_bytes(b"private\n")
        expected = self.root / "expected"
        expected.mkdir()
        (expected / "Cargo.toml").write_bytes(b"private\n")
        self.revision = "c" * 40
        self.expected_sha256 = tree_sha256(expected)
        self.manifest = {
            "base_revision": "a" * 40,
            "base_tree_sha256": tree_sha256(self.base),
            "source_revision": self.revision,
            "source_tree_sha256": self.expected_sha256,
            "files": [{"path": "Cargo.toml", "replacement": "Cargo.toml",
                       "base_sha256": hashlib.sha256(b"baseline\n").hexdigest()}],
        }

    def test_materialization_preserves_base_and_pins_complete_result(self):
        destination = self.root / "private"
        materialize_private(self.base, destination, self.overlay, self.manifest)
        self.assertEqual((self.base / "Cargo.toml").read_bytes(), b"baseline\n")
        self.assertEqual((destination / ".revision").read_text().strip(), self.revision)
        verify_private(destination, self.expected_sha256)
        (destination / "extra.rs").write_text("unexpected")
        with self.assertRaises(SystemExit):
            verify_private(destination, self.expected_sha256)

    def test_changed_base_refused_before_copy(self):
        (self.base / "Cargo.toml").write_bytes(b"other\n")
        with self.assertRaises(SystemExit):
            materialize_private(self.base, self.root / "private", self.overlay, self.manifest)
        self.assertFalse((self.root / "private").exists())

    def test_tree_hash_frames_arbitrary_file_contents(self):
        single = self.root / "single"
        multiple = self.root / "multiple"
        single.mkdir()
        multiple.mkdir()
        (single / "a").write_bytes(b"b\0c\0d")
        (multiple / "a").write_bytes(b"b")
        (multiple / "c").write_bytes(b"d")
        self.assertNotEqual(tree_sha256(single), tree_sha256(multiple))

    def test_overlay_paths_confined_and_base_hash_checked(self):
        for field, value in [("path", "../escape"), ("replacement", "C:/escape"),
                             ("base_sha256", "0" * 64)]:
            entry = self.manifest["files"][0]
            previous = entry[field]
            entry[field] = value
            with self.assertRaises(SystemExit):
                materialize_private(self.base, self.root / "private", self.overlay, self.manifest)
            entry[field] = previous
            self.assertFalse((self.root / "private").exists())

    def test_overlay_manifest_and_provenance_are_pinned(self):
        raw = json.dumps(self.manifest).encode()
        (self.overlay / "source.json").write_bytes(raw)
        pins = {"horizon_mixins_revision": self.revision,
                "horizon_mixins_overlay_sha256": hashlib.sha256(raw).hexdigest(),
                "horizon_mixins_upstream_revision": "a" * 40}
        self.assertEqual(private_manifest(self.root, pins)[1], self.manifest)
        pins["horizon_mixins_upstream_revision"] = "b" * 40
        with self.assertRaises(SystemExit):
            private_manifest(self.root, pins)
        pins["horizon_mixins_upstream_revision"] = "a" * 40
        (self.overlay / "source.json").write_bytes(raw + b"\n")
        with self.assertRaises(SystemExit):
            private_manifest(self.root, pins)


if __name__ == "__main__":
    unittest.main()
