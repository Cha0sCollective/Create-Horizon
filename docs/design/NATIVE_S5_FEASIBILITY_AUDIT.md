# S5 — moving-assembly feasibility and proposed boundary

S4 is complete and privately synced at
`b88eb47bbb83138e9fe62684f2fcf2f1969e3702`. This is a read-only feasibility audit,
not S5 acceptance or authorization to build a general physics/contraption engine.
No S5 game/host code, resources, build or runtime experiment has been changed.

## Current implementations checked

Public port main is still10cb93e6a13b837edc2293d3669ee6cbdb4d5863. Its experimental
branch advanced during S4 toeb8b96e8ea1db698b79bb61fd1caf6c963f31652
(0.7.0-alpha.1 developer checkpoint). README/ROADMAP explicitly leave contraptions
pending; its tree has no mechanical-piston/bearing/gantry/pulley implementation.
The newer branch does add processing/transport/source-event work that should be
reused selectively for later families. It uses Horizon9c7f9b22/Mixinse8cd0aa3,
not our tested officialfc021038/a1e13326. Do not wholesale upgrade or redo its work.

Official upstream/main is now78a7e6a7120e787704925c5c00158579c710617b. The
tested checkout staysfc02103818c212920fd24aa34a2b2f91ba49e56f. Recent official
work adds useful host helpers, including chunk tickets and render/inspection
capabilities. No complete Create assembly implementation was found there.

## What can be reused

| Requirement | Existing implementation | Remaining S5 work |
|---|---|---|
| Power/control | Native kinetic network and ordinary source controls | Connect one actuator's linear speed/direction to the selected Java behavior |
| Saved machine state and replication | Native mod block entities, chunk saves and update tags | Define one actuator's captured payload, progress and ownership transitions |
| Moving visual | BlockEntityDraw translation/rotation and draw_block | Draw the captured payload at its interpolated transform |
| Static-world collision | Existing block-collision queries and geometry | Sweep the bounded payload and stop before an occupied destination |
| Moving-solid/entity contact | Vanilla moving-piston collision and pushing | Bind a mod-controlled move to those services, or introduce a reviewed narrow extension |
| Return to real blocks | Existing set_block/edit_blocks and ordinary updates | Preflight destination, restore once, and account for cancel/removal |

Pinned source anchors are `native_hooks.rs` block-entity/save/sync helpers,
`native_block_entity_draw.rs` pose/draw support, `native_entities.rs` display
movement, `horizon-world/src/level/pistons.rs`, `piston_shape.rs`,
`horizon-client/src/game/moving_pistons.rs` and `piston_push.rs`.

Rendering a translated block is already possible; it does not create a solid
moving assembly. `native_move_entity` operates on existing retained entities such
as displays/markers/interactions. The inspected native catalog does not expose a
general moving-solid collision provider. Vanilla's moving piston has a specific
owner/type and automatically advances progress by0.5/tick, then materializes the
block. It is not a drop-in RPM-driven, pausable, reversible Create contraption.
Native mod block entities are explicitly not moved by a vanilla piston.

The existing piston geometry/pushing is a promising reuse target. A private exact-
pinned mixin might adapt its lifecycle for a strictly owned payload. That is an
unproved implementation route, not evidence that an engine fork or a new general
collision system is necessary. No native capability is declared impossible based
only on an absent hook name.

## Concrete next packet proposed for review

Use one linear mechanical-piston family actuator with one inert full-cube vanilla
payload and one-cell travel, before inventories, fluids, glue graphs, rotating
assemblies, riders, mounted interactions or cross-chunk movement. First prove
whether existing moving-piston collision/render/save primitives can carry an
actuator-owned, RPM-driven paused move through a bounded private adapter. Compare
selected motion/stop/disassembly behavior with the retained Java6.0.11 source;
do not relabel an ordinary redstone piston as a ported Create actuator.

The feasibility spike must resolve ownership during capture/motion, world and
player collision, saved mid-motion continuation and blocked/safe disassembly.
One combined isolated game milestone follows any focused source checks. Reuse
S1–S4's unchanged host/save/package evidence; do not repeat their full batteries.

Before implementing a new moving-solid host contract or persistent assembly
representation, decide its owner and limits. If the existing piston adapter can
stay bounded within the adopted native architecture, propose that exact packet
and independent review. If it requires a broad collision/ownership service or new
acceptance guarantee, obtain owner input under the stage progression boundary.
This audit alone neither accepts S5 nor proves that such a broad change is needed.
