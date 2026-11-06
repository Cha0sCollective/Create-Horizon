#!/usr/bin/env python3
"""Use Horizon Mixins on a pristine git export; never patch the upstream checkout."""
import argparse
import subprocess
import tarfile
import shutil
import hashlib
import tempfile
from pathlib import Path
from common import ROOT, HORIZON, NATIVE, PINS, cargo, check_horizon, environment
import setup_mixins


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("action", choices=["stage", "check", "build"])
    parser.add_argument("--work-dir", type=Path, default=ROOT / ".local",
                        help="Owned generated build directory (Linux storage is faster under WSL)")
    args = parser.parse_args()
    work = args.work_dir.resolve()
    if work in [ROOT.resolve(), HORIZON.resolve(), Path(work.anchor)]:
        raise SystemExit("Build directory must be a separate generated-output folder")
    ownership = work / ".create-horizon-stage-root"
    owner = str(ROOT.resolve()) + "\n"
    if ownership.is_symlink():
        raise SystemExit("Refusing redirected build ownership marker")
    if work.exists() and not ownership.exists() and work != (ROOT / ".local").resolve() and any(work.iterdir()):
        raise SystemExit("Refusing a nonempty external build directory without this project's ownership marker")
    work.mkdir(parents=True, exist_ok=True)
    if ownership.exists() and ownership.read_text() != owner:
        raise SystemExit("Build directory belongs to another source checkout")
    for name in ["horizon-source", "stage", "mixin-input", "horizon-source.tar", "stage-key.txt", "plan.json"]:
        path = work / name
        if path.is_symlink() or not path.resolve().is_relative_to(work):
            raise SystemExit("Refusing redirected generated path: " + str(path))
    ownership.write_text(owner)
    check_horizon()
    setup_mixins.main()
    source = work / "horizon-source"
    if not source.exists():
        source.mkdir(parents=True)
        archive = work / "horizon-source.tar"
        with archive.open("wb") as output:
            subprocess.run(["git", "archive", PINS["horizon_revision"]], cwd=HORIZON, stdout=output, check=True)
        with tarfile.open(archive) as package:
            package.extractall(source, members=[m for m in package.getmembers() if not m.issym() and not m.islnk()])
        archive.unlink()
        (source / ".horizon-revision").write_text(PINS["horizon_revision"])
    if (source / ".horizon-revision").read_text().strip() != PINS["horizon_revision"]:
        raise SystemExit("Generated host source has a different pin; use a fresh build directory")
    # The upstream CLAUDE.md is a documentation alias; Mixins rejects symlinks.
    for path in source.rglob("*"):
        if path.is_symlink():
            if path.name != "CLAUDE.md":
                raise SystemExit("Unexpected symlink in pristine export: " + str(path))
            path.unlink()
    mixins = ROOT / "target" / "debug" / "horizon-mixins"
    if not mixins.exists():
        cargo("build", "--locked", "--manifest-path", ROOT / ".tools/horizon-mixins/Cargo.toml", "-p", "horizon-mixins-cli")
    stage = work / "stage"
    digest = hashlib.sha256(str(PINS).encode())
    for path in sorted(NATIVE.rglob("*")):
        if path.is_file():
            digest.update(path.relative_to(NATIVE).as_posix().encode())
            digest.update(path.read_bytes())
    marker = work / "stage-key.txt"
    key = digest.hexdigest()
    if not marker.exists() or marker.read_text() != key or not stage.exists():
        mods = work / "mixin-input"
        if mods.exists():
            shutil.rmtree(mods)
        shutil.copytree(NATIVE, mods / "create")
        subprocess.run([str(mixins), "plan", "--workspace", str(source), "--mods", str(mods),
                        "--out", str(work / "plan.json")], env=environment(), check=True)
        # Mixins' atomic replacement works on Linux only. Like Horizon's launcher,
        # apply to a fresh output and synchronize changed files to a stable Cargo
        # workspace. Unchanged source paths/mtimes keep incremental builds useful.
        with tempfile.TemporaryDirectory(prefix="mixin-stage-", dir=work) as temporary:
            fresh = Path(temporary) / "game"
            subprocess.run([str(mixins), "apply", "--workspace", str(source), "--mods", str(mods),
                            "--out", str(fresh)], env=environment(), check=True)
            wanted = set()
            for path in fresh.rglob("*"):
                relative = path.relative_to(fresh)
                wanted.add(relative)
                target = stage / relative
                if path.is_dir(): target.mkdir(parents=True, exist_ok=True)
                elif not target.exists() or target.read_bytes() != path.read_bytes():
                    target.parent.mkdir(parents=True, exist_ok=True)
                    shutil.copy2(path, target)
            if stage.exists():
                for path in sorted(stage.rglob("*"), reverse=True):
                    if path.relative_to(stage) not in wanted:
                        if path.is_dir(): path.rmdir()
                        else: path.unlink()
        marker.write_text(key)
    if args.action != "stage":
        # Build the playable client with its existing sound-output feature.
        # Keep optional controllers disabled; a no-defaults server build cannot
        # establish playback even when the mod's client sound requests run.
        command = [args.action, "--locked", "--manifest-path", stage / "Cargo.toml", "-p", "horizon-client", "--bin", "horizon-client", "--no-default-features", "--features", "horizon-client/audio", "--jobs", "4"]
        if args.action == "build":
            command += ["-p", "horizon-client-lib", "--bin", "horizon-ctl", "--profile", "fast", "--config", "profile.fast.package.horizon-client.opt-level=0"]
        cargo(*command)


if __name__ == "__main__":
    main()
