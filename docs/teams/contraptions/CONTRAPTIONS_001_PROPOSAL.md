# Contraptions 001 — staged core reuse and first native bridge

Status: proposal for project-lead allocation, 5 November 2026. No behavior,
acceptance, pin, owner format, or host interface has changed. The team lane is
`codex/team-contraptions` at `76dfaf78370f41b2a5c68620c55be6ecab96f16f`,
based on accepted water `02d029f3bea09c62480e07e315e23d4868f9180a`.

## Corrected source inventory

The public port main at `dd0f3141a33851ac20fa98b807a6532471dba727`
still describes moving contraptions as planned. That was an incomplete reuse
assessment. The existing read-only staging ref
`origin/stage/contraption-core` in
`G:\Create Rusty\.local\evaluations\create-horizon-08-review\source`
resolves to **`affd76f1301b61b264c08072f9a7aa8d3be0343c`**. Its complete
commit object and files were inspected by immutable ref without switching the
staging checkout. Do not wholesale import its other branch changes.

At that ref, `mixins/create/engine/src/core/contraption/` provides:

| Component | Reusable behavior | Boundary |
| --- | --- | --- |
| `assembly.rs:65-70,99-169` | Generic read-only `CaptureView`, host-supplied movement/adhesion/glue rules, bounded graph discovery and local snapshots | Host must classify real blocks and preserve full snapshots; core performs no world writes. |
| `assembly.rs:180-202` | Complete rotated landing plan with host rotation and destination preflight | Host must revalidate and commit; plan alone grants no custody or restoration. |
| `transform.rs:20-60,88-110` | Continuous local/world point rotation and snapped quarter-turn placement/state axes | Host must use one anchor/pose for rendering, contact and placement. |
| `bearing.rs:41-109` | Bearing start/stop, RPM angle, stall and requested disassembly | Host must own the assembly and acknowledge disassembly only after a successful commit. |
| `tests.rs:71-267` | Six source tests for adhesion, refusal, retry, transforms and a recorded three-axis bearing trace | These are pure tests, not native motion, persistence, collision or rendered-client evidence. |

The staged [contraption document](https://github.com/Delete228/create-horizon/blob/affd76f1301b61b264c08072f9a7aa8d3be0343c/docs/contraptions.md)
explicitly leaves native owner/save/unload, world commit, client state, contact,
rendering, actors and inventories open. Source search found no native `Game`
caller of this core. `Pos`, `Axis` and `Direction` have compatible visible
definitions in the accepted private engine, but compilation against that engine
has not been run. The staged trace names Create 6.0.10; the six selected Java
bearing/contraption/transform classes were SHA-256 identical in the local
6.0.10 snapshot and pinned 6.0.11 `7ff2760e` checkout. This supports this
selected source comparison, not a new 6.0.11 Java run or full-version parity.

## First useful adapter within the current boundary

Allocate a **read-only native bearing preflight bridge** to the staged portable
core, then review its exact source delta. Selectively carry the four core source
files and their tests with provenance; retain the existing private modules and
pins. Implement a `CaptureView` over the pinned native `Game` for one loaded,
same-chunk Mechanical Bearing scene: one slime block bonded to one oak log.
The bridge reads live block states and load/bounds status, maps only verified
movability and slime adhesion rules, and produces full in-memory state snapshots
for these two inert blocks. Unknown blocks, block entities, glue/chassis and
unsupported states fail closed. If the pinned host lacks a read-only loaded or
bounds query, report that exact gap and refuse capture rather than loading a
chunk or adding a host API under this packet. No block is removed or converted
to a proxy.

The same bridge calls `Assembly::capture`, `Rotation`, and `Assembly::landing`
to report the discovered local cells and rotated log-axis landing targets on
X/Y/Z. It must refuse unloaded/out-of-bounds and occupied targets. Its result
is a plan for review, never a write or a saved assembly. Test the actual native
classification/target mapping against the staged cases and pinned Java source;
check the core's `OutsideWorld` handling and any fail-open default before using
capture to remove a block. This is reusable graph/pose integration for the next
drives, not another fixed piston cargo variant. It proves neither moving
contraptions nor gameplay acceptance.

The accepted east-facing finite piston remains on its own owner, codec,
collision and render path. Its stone/depot/portable regression is a separate
compatibility check once a later native runtime is composed; it is not migrated
into the new assembly representation by this packet.

## Contracts for the gated live assembly step

- **Ownership:** Before capture removes world blocks, decide one stable
  level-owned assembly identity and saved structure/pose record, with a
  controller reference that can disappear without losing custody. Define
  source identity revalidation, save/chunk unload, failed-write retry and
  duplicate-owner refusal. The finite piston base remains its existing owner.
- **Transform:** Store local block coordinates and one anchor/axis/continuous
  pose. Use `Rotation::to_world` for visual and contact points and snapped
  `QuarterTurn` plus block-state rotation for placement. The source uses a
  center pivot; no independent client transform should redefine it.
- **Contact:** Derive previous/current transformed local points from the same
  pose; compare a swept body against world and player/entity movement before
  advancing. The finite `s5_clearance.rs` path is capped at three moving
  shapes, so it does not qualify arbitrary assemblies. Rider, passenger and
  actor contact need separate reviewed semantics.
- **Restoration:** Preflight every loaded target and rotated snapshot, then
  revalidate before one server-authoritative commit. Retain the assembly on
  obstruction or failed write; call `Bearing::disassembled()` only after the
  physical blocks agree. Resolve Create's configured replace/drop behavior
  explicitly instead of silently changing the accepted piston's retry rule.
- **Storage and actors:** A mounted item/tank must have one canonical assembly
  owner and a checked access view, never a second stationary copy. Transport
  owns the item-view interface, Fluids the tank interface, Machinery the actor
  processing contract, and Trains/Schematics the train assembly consumer.

The first live milestone after an owner-approved owner/save/contact design
would be one Bearing with the slime/log assembly: assemble, rotate, stop,
obstruction/retry, ordinary cold reopen, controller removal and actual client
render/contact observations. That is a later packet, not authorized by this
read-only bridge. A new general owner/schema or collision framework, broader
host API, changed acceptance, or public release needs the existing owner gate.

## Checks and handoff

Executed: lane/ref/dirty-state inspection, immutable staged source and six
core-test inspection, private finite-piston source comparison, and SHA-256
identity checks of the six selected Java files. No Cargo, build, Java run,
game/server, source edit outside this proposal, resource reservation or remote
write. Nested read-only tasks: `/root/upstream_core` and
`/root/java_behavior`; operations records their phase durations and collects
actual token telemetry. A future code packet needs isolated worker ownership,
focused checks, fresh independent read-only Sol High review, and a project-lead
host-wide build/test reservation. No lease is requested for this proposal.
