# Contraptions 003 — reusable entity owner design

Status: design for project-lead review, 5 November 2026. The owner selected
entity-owner design before any live bearing implementation. This document
proposes the first bounded private packet; it implements no entity, custody,
save schema, host change, world write, contact or gameplay. The earlier
[read-only preflight](CONTRAPTIONS_001_HANDOFF.md),
[chunk-bearing proposal](CONTRAPTIONS_002_LIVE_BEARING_PROPOSAL.md) and
[entity comparison](CONTRAPTIONS_002_ENTITY_OWNER_ADDENDUM.md) remain evidence,
not an adopted chunk owner. Accepted primary native source is
`55339ce3347b2ce667281e8a3e6318ec856144e4` (.11/cache14), with pinned
Horizon `9c7f9b2215dfed497aba5522c44f38926628648d`, Mixins
`e8cd0aa38cffef49cb2e088ef716baa107148c02` and Java Create
`7ff2760e4924d35d4bc2e22c4b19923f0178cd16`. The team lane is based on
its prior `9b874083231c816873a723c2d7601cbf9b584967`; root owns serial
composition against the newer primary source.

## Recommended authority and first carrier

Give each captured assembly one stable `AssemblyId` independent of its
storage backing. For the first bearing, save the one canonical
`AssemblyRecord` as a private typed `Holder::Entity` attachment on an entity
carrier. The carrier has its **own** persistent UUID; its runtime numeric ID
is only a loaded-level handle. The controller refers to the `AssemblyId`
and may cache the carrier UUID as a lookup hint; it never owns the captured
cells. A separate synced `AssemblyView` is derived from the record and may
be rebuilt or discarded without losing custody. This is an entity lifetime
and storage contract for the bearing, not an assertion that every later
assembly has an entity as its sole save owner or that the display supplies
Create physics.

For the first proof, use a known vanilla `minecraft:block_display` as the
carrier, with no `block_state` (air by default), so it is tracked but does
not draw an extra block. `native_summon` already creates it in a loaded
chunk; `native_move_entity` can follow the assembly anchor
(`game/native_entities.rs:139-215`). The host preserves `OtherEntity` raw NBT,
pose and UUID in the entity-chunk save (`game/persist.rs:889-935,1371-1398`),
and `Holder::Entity` attachments are UUID-backed and encoded in that entity
NBT (`game/mod_api_host.rs:60-87,122-133`;
`game/persist.rs:1240-1246,1388-1398`). A block display has a nonzero
tracking range (`crates/horizon-client/src/lan/tracking.rs:23`), while a marker
has range zero (`:95`), so marker attachment sync is inadequate for ordinary
peers. `lan_entities.rs:431-457` sends air when a block display has no block
state. This carrier choice must be checked in actual peer rendering before
source blocks are removed.

The design does **not** declare a new simulated `create:` entity. The content
system permits only snowball-templated, item-thrown entity types
(`crates/horizon-mods/src/content.rs:445-462,906-930`). A real new Horizon
kind requires the fixed `entity_lists!` entry and `impl Entity`, which drive
load/save and typed dispatch (`game/entity.rs:1-8,331-348,606-638`). An
arbitrary unregistered ID may survive server-side as `OtherEntity`, but peer
pairing still needs a registry entity-type ID (`lan/hub.rs:500-510,2755-2770`).
The carrier may later be replaced by a true simulated kind without changing
`AssemblyId`, snapshot meaning or custody transitions; that replacement is
a separate bounded host assignment if required by riders, broad collision
or rendering.

## Canonical record and invariants

The private versioned record has: an independently generated `AssemblyId`,
the current carrier UUID, dimension and controller position/type/facing;
drive kind; assembly anchor;
bounded local cell snapshots (relative position, exact block name/properties,
and an explicit payload kind); prior/current server pose and tick; capture
and restore progress; removal request; and last verified source/target
identities. The first codec accepts **only two inert state-only cells** and
`DriveKind::Bearing`, but represents a length-checked cell collection rather
than two named fixture fields. Unknown payloads, malformed or duplicate
local positions, non-finite poses, unsupported states, or a carrier UUID that
does not match the hosting entity fail closed. Item, fluid and actor payloads require
their domain owners' later contracts; an optional field is never silently
treated as empty. The `AssemblyId`, local cell and custody vocabulary can
later be used with a train-owned backing; they do not prescribe where a
train or its cargo is saved.

At most one server record owns a captured cell or reserved landing cell.
Creation checks loaded owner entities and the controller link before capture;
a missing or stale link cannot authorize recapture. The controller BE stores
only `AssemblyId` and an optional carrier UUID lookup hint as advisory links
beside drive input, never a second
structure snapshot. If the controller is removed or replaced, the owner
entity and its record remain. A new controller at the same position cannot
claim the old record merely by matching coordinates. A loaded owner's
attachment must remain decodable before motion or writes; corruption stalls
with an explicit diagnostic and no speculative restoration.

The state sequence is `Preparing` (owner and snapshots saved in memory before
any source removal), `Captured`, `Moving`, `RestorePending`, then `Settled`
and owner removal. A `Blocked` reason annotates any retained state rather
than erasing its verified progress. Every source removal and target placement
uses a planned exact identity, revalidation immediately before the write,
checked `native_set_block` and readback. Update verified progress in the
canonical record before the next write. Never remove the owner entity before
all original material is present exactly once in the world and checked.
`Assembly::capture` and `Assembly::landing` plan but do not transact
(`native/create/mixins/create/engine/src/core/contraption/assembly.rs:101-202`);
`native_set_block` is one state at a time, not an atomic multi-block commit
(`game/native_hooks.rs:8349-8381`). Normal save/reopen is a required test;
interrupted entity-file/chunk-file atomicity is explicitly outside this
packet, as the adopted native plan defers a custom crash framework.

## Lifecycle, deletion and ordinary reopen

The private owner adapter must discover loaded carriers by an exact private
tag plus carrier UUID, decode and index their `AssemblyRecord` by independent
`AssemblyId`, and reject duplicate IDs or overlapping claims. It rebuilds
its **transient** index after the
host loads entity records; `native_entities_load` reports chunk/count only,
so a private `Game::others` scan is needed (`game/native_hooks.rs:5839-5854`;
`game/persist.rs:342-405`). Drive owners from one authoritative per-level
tick, even when the controller BE is gone. `native_level_tick_post` exists
(`game/native_hooks.rs:2331-2337`); `OtherEntity` is absent from
`entity_lists!`, and native entity tick/join hooks exclude the unsimulated
display/marker group (`game/entity.rs:331-348`;
`game/native_hooks.rs:7133-7145`; `docs/native-mods.md:1187`). Wait if the
controller or required cells are unloaded; never infer air from an unloaded
query. On normal same-chunk unload, freeze and let the host save the carrier
and block chunk; on load, reconcile every cell against the retained phase
before advancing, restoring or showing motion. The carrier's entity chunk
and the two cells must be the same chunk in the first live test.

Explicit deletion must be guarded while custody is active. `native_remove_entity`
and `/kill`'s `Target::Other` path are separate removals
(`game/native_entities.rs:254-261`; `game/commands.rs:2144-2207`);
`native_entity_join_level` and leave callbacks do not cover these
`OtherEntity` instances (`game/native_hooks.rs:7109-7145`). If a requested
deletion cannot first restore all cells safely, refuse it, retain the owner
and retry after the obstruction clears. Normal chunk unload is not a delete.
The attachment is pruned after actual entity loss
(`game/mod_api_host.rs:75-87`), so detecting deletion on the following tick
cannot recover the sole snapshots. **Before any live capture**, a focused
private Mixins feasibility check must prove cancellation of every relevant
explicit removal path at this pinned host. If Mixins cannot provide a
complete guard, stop and request a bounded host removal hook; no cargo may
be entrusted to a deletable display. Ordinary `/kill`, native removal,
controller mining, command replacement, unload and cold reopen are distinct
test cases. No claim is made for malicious private code or abrupt process
failure.

Relocation is another owner-lifetime mutation. `/tp` writes
`Target::Other.position` directly (`game/commands.rs:1510-1515,1643-1653`),
and `native_move_entity` changes an `OtherEntity` position without checking
the destination chunk is loaded (`game/native_entities.rs:192-215`). The
first bearing has a fixed anchor: reject **all external carrier relocation**
while it owns cells, and verify that the carrier still occupies its recorded
same chunk before every tick, write and save reconciliation. Test both `/tp`
and native movement attempts. If either cannot be intercepted reliably in
private Mixins, stop before live capture; later train-owned motion needs an
explicit checked relocation path and cross-chunk lifetime contract.

## Pose, replication, rendering and contact

The server record is the only pose authority. A drive computes prior/current
finite local-to-world transforms in blocks; the bearing variant uses one
anchor at the front slime cell, axis and continuous angle, with snapped
quarter turn only for final block placement. The portable core's
`Rotation::to_world` and snapped state-axis mapping are the bearing
implementation (`core/contraption/transform.rs:20-60,88-110`). The same
transformed local cells feed static clearance, body clearance, client
presentation and restoration. A controller's visual angle or an entity
network interpolation never changes the server's contact decision.

Keep the full `AssemblyRecord` unsynced. Derive a small, versioned
`AssemblyView` attachment on the **same entity**: `AssemblyId`, state-only visual
cells/local positions, anchor, axis, previous/current angle, tick and phase.
The host sends synced entity attachments to tracking clients and snapshots
on tracking start (`game/attachment_sync.rs:18-25,39-57,85-125`). The first
client renderer uses depth-tested world atlas quads from `native_world_submit`
and `native_world_quad`, independently of the controller BE
(`game/native_hooks.rs:3166-3174`;
`game/native_world_draw.rs:1-62,125-164`). It draws the two inert slime/log
shapes as textured cube faces from the derived view, including while controller-removal
restoration is blocked; actual peer images must verify textures, orientation,
occlusion and cleanup. The entity itself is an air display, not duplicate
cargo. The renderer never reads canonical server custody. Broader Create
models and large assemblies need a later render/streaming packet, not a
2,048-block attachment broadcast disguised as a scalable solution.

For the first live bearing, conservatively refuse any angle step whose
two-cube swept envelope meets a third-party solid or authoritative body.
Initial capture permits only the two exact source blocks in that envelope;
after capture, a reserved cell is permitted only if the record verified its
own materialization. Check body clearance before capture rollback, every
restore placement and cold-load recovery. Stall without pushing or carrying
players/entities, and never advance or materialize a block into an observed
body. The moving blocks are otherwise visual and non-solid: a player can
enter a stationary displayed shape, after which the next movement or restore
must refuse. Existing `s5_clearance.rs:43-55` is a
finite linear piston path, not rotating contact. The pinned host's
`piston_push.rs:377-425` enumerates bodies for a narrow private clearance
adapter; it does not provide general contraption collision. Java uses
previous/current transformed points and much broader collision
(`AbstractContraptionEntity.java:824-831`;
`ContraptionCollider.java:77-119`). Riders and actors remain unimplemented.

## Reuse and first implementation packet

Java's controlled contraption entity saves its structure, controller link,
axis and angle, and the Java bearing captures from the cell in front of the
controller (`ControlledContraptionEntity.java:47-53,80-97`;
`BearingContraption.java:37-59`). Its controller removal invokes disassembly,
but Java's generic restoration can replace/drop occupied targets and its
entity may discard itself when it cannot reattach
(`MechanicalBearingBlockEntity.java:69-73`;
`Contraption.java:1104-1160`;
`ControlledContraptionEntity.java:154-173`). This design deliberately
retains custody and retries on obstruction, matching the accepted finite
piston's no-loss outcome rather than claiming exact Java behavior.

The finite S5 piston keeps its accepted base BE owner, codec, fixed linear
clearance and renderer. It can reuse the vocabulary of snapshots, verified
progress and prior/current pose only if a later migration is separately
authorized and tested; this design does not move it. The train team's
read-only requirements distinguish a different owner: pinned Java
`RailwaySavedData.java:24-26,33-66` saves train and graph authority, while
`CarriageContraptionEntity.java:319-330,469-479,719-723` rebinds from
`TrainId`/`CarriageIndex` and is explicitly not saved. `Carriage.java:75-93,652-665`
owns cargo. A later native train can reuse `AssemblyId`, local snapshots,
checked custody and prior/current pose/contact vocabulary, but its graph,
train and cargo owner must be a separate reviewed backing; carriage entities
are rebindable projections. This bearing design does not force every train
carriage's sole authority into a display attachment. Graph/bogey pose,
multi-chunk loading, tracking, moving storage, riders and stations remain
train-specific contracts. The first bearing's same-chunk restriction and
two-cube clearance are validation limits, not structure fields or a general
acceptance claim.

**Next writable packet proposed to the project lead:** in an isolated team
candidate, implement the private owner carrier, versioned record/view codecs,
UUID discovery, per-level tick, and deletion and relocation guards **without
removing any world blocks**. Exercise actual entity creation, save/reopen,
tracking/peer view, `/kill`, native removal, `/tp` and native movement refusal
with a synthetic inert record; no fake captured cargo may be accepted as
gameplay. A fresh read-only Sol High
source review and root-reserved build/runtime gate must clear this foundation.
If every removal path cannot be guarded, stop at the exact pinned Mixins gap
and seek a bounded host assignment. The following separately reviewed live
packet would register one upward Bearing, connect the accepted kinetic
source, and use the same entity owner to capture, rotate at 32 RPM for nine
ticks, stop at 90 degrees, restore slime and oak log exactly once, stall and
retry on static/body obstruction, survive controller removal and normal
mid-motion cold reopen, and show actual integrated and dedicated peer views.
No inventories, actors, fluids, passengers, riders, glue, chassis,
cross-chunk motion or general bearing axes are claimed by that test.

The owner's current direction authorizes this **design only**. The project
lead must obtain a bounded implementation decision for the new private
entity attachment schema and lifecycle hooks, with the exact deletion and
relocation interception surfaces demonstrated. The controller-block/kinetic
binding and finite contact/refusal rule belong in the later live packet.
Broader host API, multi-chunk lifetime, train motion, storage/actor
interfaces, Java replace/drop parity and crash coherence need separate
bounded decisions. This design grants none of those implementations.

## Read-only evidence and validation plan

Nested audits: `/root/upstream_core` for pinned host lifecycle/services and
`/root/java_behavior` for pinned Java/portable semantics. Their source
receipts are incorporated above. The train authority distinction also uses
the Trains team's read-only draft
`docs/teams/trains-schematics/TRAINS_002_ENTITY_OWNER_REQUIREMENTS.md`,
checked against its cited Java classes; that draft is not yet a final train
acceptance result. No Cargo, Java run, game, network, source
integration or host resource reservation was used for this design. Before
returning a writable candidate, run the focused owner codec/lifecycle tests,
native client test compilation and the exact real-world cases named above
under root's one heavy lease. Reuse the already passed accepted-source
seven portable core and five native preflight checks by attribution; a
design document does not turn them into live contraption evidence.
