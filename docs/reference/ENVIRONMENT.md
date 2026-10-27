# P01 reference environment

This is the S0 development environment, not a supported player installation.
Versions and source revisions are recorded in `workspace.toml`. G0's experimental
acceptance and its remaining limits are in `../design/G0_DECISION.md`.

## Installed Linux tools

Installed October 27, 2026 in the existing Ubuntu 24.04 WSL2 distribution, x86_64.
The host CPU reports AMD Ryzen 9 7900X, 24 logical CPUs; WSL exposes approximately
15 GiB RAM and 4 GiB swap. These are development context, not ratified benchmark
hardware or a performance measurement. The Windows Java default was not changed.

| Package | Installed Ubuntu package version |
| --- | --- |
| build-essential | 12.10ubuntu1 |
| gcc / g++ metapackages | 4:13.2.0-7ubuntu1 |
| actual compiler | GCC 13.3.0, 13.3.0-6ubuntu2~24.04.1 |
| pkg-config | 1.8.1-2build1 |
| libasound2-dev | 1.2.11-1ubuntu0.3 |
| libudev-dev | 255.4-1ubuntu8.18 |
| libvulkan1 | 1.3.275.0-1build1 |
| mesa-vulkan-drivers | 25.2.8-0ubuntu0.24.04.4 |
| python3 | 3.12.3-0ubuntu2.1 |
| openjdk-21-jdk | 21.0.12.1+1-1~24.04.4 |
| openjdk-25-jdk | 25.0.4.1+1-1~24.04.4 |

JDK paths are `/usr/lib/jvm/java-21-openjdk-amd64` and
`/usr/lib/jvm/java-25-openjdk-amd64`. The reference runner chooses 21 explicitly;
Horizon exporters choose 25. Do not rely on the shell's default Java alternative.
Rust 1.99.1 remains project-local through `tools/dev.sh`. Installed package versions
are an environment record; future apt repositories may need archived packages to
reconstruct the exact installation.

## Create reference commands

From this repository inside Ubuntu:

```sh
python3 tools/reference.py tasks
python3 tools/reference.py compile
python3 tools/reference.py lock
python3 tools/reference.py capture
```

The runner refuses a different Create revision, tracked changes or untracked inputs,
wrong wrapper checksum or different pinned JDK. It invokes the Java wrapper main
directly because the Windows checkout's `gradlew` has CRLF endings; it does not
rewrite the source snapshot. Gradle 8.14.3 comes from that reference's wrapper.
Pass `--offline` only after all artifacts required by the selected task are cached.

Each invocation creates a fresh directory under
`../.local/s0/reference-runs/<uuid>`. Its GameTest world, configuration, resources,
environment metadata, console log, exit status and resolved artifact hashes stay
there. Gradle's shared dependency cache is `../.local/gradle`; it is not a world
directory. No command uses Prism's saves or launches the owner's playable instances.

The development adapter adds its own observer source and original empty structure
to the build and replaces upstream test discovery. Production Create mechanics,
recipes and source files remain unchanged. This is an instrumented reference build,
not the exact bytes of a published Create JAR. Record the adapter hashes and actual
runtime configuration with any accepted trace. Resolved compile/runtime artifacts
are listed by configuration, coordinate and SHA-256, including generated local
Minecraft/NeoForge classpath files as `local:classpath:<filename>`. Inspect a successful `lock`
run's `resolved-artifacts.json` before recording it as
`docs/reference/artifacts.lock.json`. Capture refuses a missing lock or any artifact
difference before starting the GameTest server. This checks upstream dynamic
dependency ranges instead of assuming they stayed fixed.

`capture` runs the real NeoForge GameTest server. A successful build or an empty
test selection is insufficient: require all declared fixtures, all sample ticks,
successful completion and repeated traces. No Rust behavior is compared yet.

## Horizon resource preparation

The player's Prism 26.3 client was verified against Horizon's pinned SHA-1
`e877b6a07acd633fb3bb475002175cec036e7b87`. The matching official server SHA-1 is
`33680f5f2ac32864d6d7cf5e56a705fdb3e05f4c`; the selected original Mojang version
manifest SHA-1 is `11f1aefe473eefba0af62e3286c33b692807b2eb`.

For this WSL environment, a local official-layout input directory was assembled at
`../.local/s0/minecraft-26.3`: client copied from Prism, 53 libraries copied from
Prism, 12 missing Linux libraries downloaded from their official manifest URLs and
verified by SHA-1. Its assets lookup points to Prism's asset store for reading.
The matching server was downloaded from the official manifest URL and verified.
Acquisition details are local in `../.local/s0/horizon-inputs.json`. All imported
and prepared files stay outside Git; no game files accompany this documentation.

The first build used temporary Linux storage. For retained development builds,
use the persistent user-scoped cache and keep executable copies in the ignored
workspace. From `../horizon`:

```sh
export CARGO_TARGET_DIR="$HOME/.cache/create-horizon/target"
../create-horizon/tools/dev.sh cargo build --locked --profile fast -j 3 \
  -p horizon-client --features mods --bin horizon-client --bin horizon-ctl \
  -p horizon-prepare --bin horizon-prepare
mkdir -p ../.local/bin
cp "$CARGO_TARGET_DIR/fast/horizon-client" "$CARGO_TARGET_DIR/fast/horizon-ctl" \
  "$CARGO_TARGET_DIR/fast/horizon-prepare" ../.local/bin/
```

The first preparation attempt on the Windows-mounted drive was interrupted: an
exporter made individual-byte NBT writes there. An exact `git archive` of the
pinned engine was extracted to `/tmp/create-horizon-s0-engine`, verified imported
jars/exporter caches were copied there, and preparation completed on the native
Linux filesystem. The compiled preparation binary above was used with `--root`
pointing to that snapshot. This changes the output location, not exporter logic.
The completed resource folder was then copied back into the ignored workspace.
Logs distinguish the interrupted attempt from the successful native-filesystem run.
Temporary build/staging directories were later cleared, so subsequent builds use
the persistent cache above. The retained resources and test evidence were preserved.
To regenerate resources, extract the exact pinned `git archive` into a fresh native
Linux staging directory, run the retained `horizon-prepare --prepare --root <stage>`
with explicit JDK 25 and the verified input/server paths above, then copy only the
completed resource folder back. A temporary staging directory is disposable; it
is not the location to retain executables or the only copy of prepared resources.

Prism's asset index 34 has SHA-1 `917ff6d74b9f635c46bac0d28857206bbcc1ffe4`, newer
than Horizon's accepted index list. The pinned tool explicitly allows such an
index with a diagnostic and verifies each referenced asset by its own SHA-1. JAR
and selected version-manifest checks remain unchanged. Record this index in any
visual/language baseline; do not claim all asset-index pins matched.

Prepared resources are retained in `../horizon/.local/horizon-client/26.3`; the
first smoke used the native-filesystem copy; a later smoke also passed with the
retained binaries and workspace resources. Optional sounds are now imported too.
Game launches
must additionally use fresh data/config/launcher directories and the isolation
variables required by Horizon's AGENTS.md. Building binaries and preparing resources
alone do not verify rendering, networking, machinery or game conformance.
