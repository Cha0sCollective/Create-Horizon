# S0 P01/H01 evidence record

Date: October 27, 2026. This is an environment/reference packet, not a port release.
Exact source pins are in `workspace.toml`; environment details and commands are in
[ENVIRONMENT.md](ENVIRONMENT.md). Raw game files, worlds, logs and screenshots stay
under the ignored workspace `../.local/s0/` or the documented WSL scratch paths.

## Confirmed baseline results

| Check | Outcome and limit |
| --- | --- |
| Linux dependencies and JDKs | Installed and verified in Ubuntu 24.04; JDK 21 and 25 invoked explicitly |
| Private Rust scaffold | Formatting, linked build/test command and Clippy pass; zero gameplay tests exist |
| Horizon preparation tool | Built from the exact pinned source with locked Cargo dependencies |
| Horizon client + control binary | Fast-profile build with `mods` succeeds; same game binary includes dedicated mode |
| Horizon resource preparation | Succeeds on native WSL filesystem from verified owned 26.3 client/server; completed resources copied into ignored workspace |
| Horizon mod-runtime unit tests | 18 pass; this is not the full real-Wasm sandbox suite |
| Default integrated-server smoke | Fresh creative test-terrain world, seed 0, 640x360 offscreen, distance 2: 60 rendered world frames, exit 0, report error null |
| Existing two-client chest check | Pass, no skip, server-thread host; guest/host open and closed state agree |
| Existing two-client machine inventory check | Pass, no skip, hopper/dispenser/dropper slot and cobblestone interactions; not a milling/smelting test |
| Retained executables + workspace resources | Second fresh integrated-server smoke: 30 world frames, exit 0/error null; OpenGL software fallback |
| Optional sounds | 4,962 files, 377.3 MB copied from Prism and individually verified; playback not tested because test audio is disabled |

The graphics smoke uses llvmpipe software Vulkan (Mesa 25.2.8). It verifies ordinary
startup/rendering through the integrated server, not hardware GPU performance.
The smoke report records 49 generated chunks and 600 meshed sections. No provisional
product CPU/GPU budget was measured or accepted.

Raw baseline records:

- `.local/s0/logs/horizon-prepare-build.log` and `horizon-client-build.log`.
- `.local/s0/logs/horizon-prepare.log`: interrupted Windows-drive attempt, not a pass.
- `.local/s0/logs/horizon-prepare-native.log`: completed native-filesystem preparation.
- `.local/s0/logs/horizon-mods-unit.log`: 18-test result.
- `.local/s0/horizon-runs/5ffe4118-81b0-43f0-b856-ddbf8cd0428f/`: smoke command,
  environment, console, exit code and report.
- `.local/s0/logs/horizon-lan-baseline.log` and `.local/s0/lan-baseline/results.json`:
  existing chest and machine checks, screenshots and retained game logs.
- `.local/s0/logs/horizon-persistent-build.log` and `.local/s0/build-manifest.json`:
  retained executable build, sizes and SHA-256 identities. Executables are in
  `.local/bin/`; the Linux target cache is `$HOME/.cache/create-horizon/target`.
- `.local/s0/horizon-runs/46fec229-e284-4c14-9e53-590dcc6cb289/`: retained-binary
  smoke with workspace resources; OpenGL fallback after a nonexistent hardcoded
  Vulkan ICD path. The earlier smoke discovered its actual ICD and used Vulkan.

Those paths are relative to the parent workspace. Tests used fresh scratch homes,
worlds, XDG/launcher paths, audio disabled and telemetry disabled. Both LAN test
clients quit after their checks. No playable save was used.

## Reference capture status

The original pinned Create project configured with JDK 21, Gradle 8.14.3 and
NeoForge 21.1.219. The original `classes` build passed in 32m18s, with 66 upstream
deprecation/removal warnings. The development observer's `classes` build also
passed. The accepted classpath lock contains 102 compile and 109 runtime files,
including generated Minecraft/NeoForge jars. Capture validates it before launch.

Two fresh runs of the six initial fixtures passed all terminal assertions and
compare equal: 730 samples per run, exact source/JDK/adapter identities, 211
classpath records and 15 configuration-file receipts. Raw receipts are verified
against their files. Only Java's generated date-comment line in `server.properties`
is excluded from comparison; no actual setting or machine observation is ignored.

Accepted raw runs, relative to the parent workspace:

- `.local/s0/reference-runs/2bcc793c-5fa8-4ba5-91ed-114518d69028` (capture C).
- `.local/s0/reference-runs/68a60685-e10c-4ccf-865c-a528a8b08b10` (capture D).

The earlier A/B captures passed their six game tests, but exposed a generated
timestamp difference and incomplete configuration coverage. They are calibration
runs; the accepted C/D pair uses the corrected collector and includes FTB SNBT.

| Observed case | Result in the declared scheduled-callback samples |
| --- | --- |
| Direct drive/reverse/disconnect | +16 reaches the load; -16 reversal reaches it; disconnected load is stopped by tick 70 |
| Small/small | Settled target -16 RPM for +16 driver |
| Large/small | Settled target -32 RPM for +16 driver |
| Small/large | Settled target -8 RPM for +16 driver |
| Milling at +16 | Input and initialized work first observed at tick 13; gravel first observed at tick 263 |
| Milling at +256 | Input and initialized work first observed at tick 13; gravel first observed at tick 29 |

Both milling cases retire the dropped input entity, consume the one input and
finish with exactly one gravel and no other output. At 256 RPM the remaining-work
value reaches -6 at completion; this is a reference observation, not a value to
silently clamp. Timings use the fixture's sample phase and include a separate
tick-10 dropped-item action. They are not blanket durations from placement or
drop, and they do not establish Rust behavior.

The comparator's controls use derived copies of actual capture C, explicitly
marked as software controls rather than new game runs. Changed ratio, work and
item result return a first-difference failure. Missing samples, duplicate machine
identity and an invalid artifact hash return invalid-evidence failures. A changed
generated date is accepted; a changed real server setting is detected. Records
are in `.local/s0/comparator-controls/`. These controls verify the comparator;
there is no native port to run a differential conformance suite against yet.

## Expanded edge and interaction results

Two fresh expanded captures pass all 15 fixtures and compare equal across 1,730
samples, 211 classpath records and 15 config files, with the same narrow generated
date-comment exclusion. Accepted raw runs are
`b531ce69-28ce-4a5a-a8a2-e5ea3b10ebe2` and
`d0528c63-86c7-416b-9dbf-5ee33c1e6a50` under `.local/s0/reference-runs/`.
The extended comparator also preserves equality of the original six-case C/D pair.
[EDGE_CASES.md](EDGE_CASES.md) defines the added schedules and observation limits.

| Case | Observed terminal result |
| --- | --- |
| Finite stress | 128/256 runs, 256/256 equality runs, 384/256 stops effective rotation while retaining theoretical RPM; removing the third load recovers; expired crank stops |
| Interrupted/reversed milling | Timer/input retained while disconnected; resumes at −16 and yields exactly one gravel |
| One full output slot | Slot 0 holds 64 gravel; completion puts one more in another slot, total 65 |
| All nine output slots full | Handler limit 99, ordinary gravel stacks 64; one cobblestone consumed, output remains 576 gravel: the uninsertable result is lost |
| Changed/removed input | Extracted cobblestone retained, no output; replacement barrier is preserved |
| Empty-hand retrieval | First use returns 5 gravel and leaves 3 input cobblestone; second use returns input |
| Full-player retrieval | 2,304 iron unchanged; 5 gravel and 3 cobblestone dropped, machine empty, totals conserved |
| Motor value-settings handler | Default +16; UI magnitude zero becomes +1/−1 according to sign row; both ±256 endpoints reach the shaft |

The owner performed the physical motor edit in the isolated client and reported
the rotation changing. Its saved motor has setting and speed −256. This is a
human-assisted client observation, separate from the FakePlayer server-handler
fixtures. Those fixtures establish neither packet validation nor reach checks.

The full-output loss is an explicit owner-selected compatibility choice, recorded
as [R01](COMPATIBILITY_DECISIONS.md). It is not a conservation success. Preserve the
pinned behavior; do not silently substitute a backpressure rule in P04.

Nine derived controls are local at `.local/s0/edge-comparator-controls/results.json`.
Changed ratio (−32 to −16), finite stress (384 to 383) and remaining work produce
first-difference failures with the numeric types preserved. Missing rows, boolean
stack counts, a string slot limit, lost overflow items, an impossible motor setting
and duplicate machine identity are rejected as invalid evidence. These are
comparator controls derived from a live run, not additional game executions.

The earlier 15-case pair (`9a6c2753-85fa-467d-88ed-09ce21d2ea6b` /
`05f4544d-8d46-4b64-98ed-daeef0c9ccb9`) already showed exact zero at overload,
but review found its assertion accepted any effective magnitude other than 32.
The current pair was rebuilt and rerun with an exact-zero stopped assertion.
Adapter receipts match the current source. The numerical comparator controls were
derived from the earlier live capture and remain labeled as such.

## Contract and gate status

[H01](../design/HOST_CONTRACT.md) is the experimental direction covering paired executable/content
ownership, authoritative object identity, lifecycle/tick order, transactional
effects, coherent recovery, exact-match networking and retained presentation.
The pinned runtime's trap/output ordering and player-save error handling require
explicit machinery integration; current deferred writes and a sidecar save are
insufficient to establish the proposed guarantees.

The G0 engineering decision is recorded in [G0_DECISION.md](../design/G0_DECISION.md).
H01 is not an implemented/stable API, and no H02-H14 capability or P02-P06 gameplay
implementation is claimed by this packet. The Rust domain library is still empty
at this packet's boundary. G1/G2/G3 and performance qualification remain open.
