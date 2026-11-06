"""Release CLI cache checks using stub Cargo only; no compiler or network."""
import hashlib
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import setup_mixins


class ReleaseBuildTests(unittest.TestCase):
    def test_same_revision_foreign_binary_rebuilt_and_valid_cache_reused(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary).resolve()
            tools = root / ".tools"
            tools.mkdir()
            old_marker = tools / "mixins-built-revision"
            old_marker.write_text("retained debug marker\n")
            source = tools / "private-source"
            source.mkdir()
            binary = root / "target/release/horizon-mixins"
            revision = "c" * 40
            compiled = b"exact pinned release CLI"

            def stub_cargo(*args):
                self.assertEqual(args, ("build", "--release", "--locked", "--jobs", "4",
                                       "--manifest-path", source / "Cargo.toml",
                                       "-p", "horizon-mixins-cli"))
                # Existing foreign bytes must be absent even if Cargo's source
                # fingerprints would otherwise allow a Fresh no-op build.
                self.assertFalse(binary.exists())
                binary.parent.mkdir(parents=True, exist_ok=True)
                binary.write_bytes(compiled)

            with patch.object(setup_mixins, "ROOT", root), patch.object(setup_mixins, "cargo", side_effect=stub_cargo) as cargo:
                self.assertEqual(setup_mixins.build_release(source, revision), binary)
                self.assertEqual(cargo.call_count, 1)
                receipt = tools / "mixins-built-release.json"
                expected = {"source_revision": revision, "profile": "release",
                            "binary_sha256": hashlib.sha256(compiled).hexdigest()}
                self.assertEqual(json.loads(receipt.read_text()), expected)
                setup_mixins.build_release(source, revision)
                self.assertEqual(cargo.call_count, 1)
                # The source receipt still matches, but another CLI build has
                # overwritten this shared target with different bytes.
                binary.write_bytes(b"foreign CLI from another source")
                setup_mixins.build_release(source, revision)
                self.assertEqual(cargo.call_count, 2)
                quarantined = list(binary.parent.glob("horizon-mixins.unverified-*"))
                self.assertEqual(len(quarantined), 1)
                self.assertEqual(quarantined[0].parent, binary.parent)
                self.assertEqual(quarantined[0].read_bytes(), b"foreign CLI from another source")
                self.assertEqual(binary.read_bytes(), compiled)
                self.assertEqual(json.loads(receipt.read_text()), expected)
                setup_mixins.build_release(source, revision)
                self.assertEqual(cargo.call_count, 2)
            self.assertEqual(old_marker.read_text(), "retained debug marker\n")


if __name__ == "__main__":
    unittest.main()
