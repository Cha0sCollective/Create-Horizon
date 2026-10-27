# H01: stationary machinery host contract

Status: version 0.1 accepted as the G0 experimental design direction. This records
the first integration boundary; it does not introduce an implemented/supported API. The active scope is the
five-component creative-mode subset in the product specification. No moving
assemblies, general loader, Java compatibility or arbitrary addon API is implied.

## Baseline and integration route

Horizon is pinned to `cf62fe270b243780f9aa86c49b456d9dd9078b42`. The separate
private port owns kinetic rules, source arbitration, stress and milling semantics.
Generic content, lifecycle, authoritative transactions, persistence, replication
and retained rendering belong in the engine fork. Initial experiments use a local
integration branch on that exact engine. Upstream submission and public publication
remain separate owner decisions; upstream merge is not a research prerequisite.

The source-informed personal reference process and no-Create-contributions intent
are recorded in AGENTS.md. This contract makes no clean-room or redistribution claim.
Until asset selection is accepted, use original simple diagnostic geometry for host
experiments. Reference assets remain local; do not put them in engine commits.

### Existing capabilities and gaps

Paths below are relative to the pinned Horizon checkout, not a moving main branch.

| Area | Existing source | Required extension |
| --- | --- | --- |
| Package execution | `crates/horizon-mods/src/manifest.rs`, `runtime.rs`; server API 0.3 selects the server side exclusively | One package owns content once and declares separate client/server executables |
| Block content | `crates/horizon-mods/src/content.rs`, client `mod_content.rs` | Explicit finite state properties, shapes and machine type bindings; preserve vanilla IDs |
| Queries and writes | SDK `wit/horizon.wit` world interface; `WorldView` in runtime | Typed loaded-world machine/item access and staged, validated operations |
| Lifecycle | client `game/server_mods.rs` reports selected player placements/breaks | Every committed world mutation cause, dimension, generation, load and removal |
| Tick | client `game/levels.rs::tick_server` and `game.rs` | Machine phase tied to each active level's simulation tick, respecting pause/freeze |
| Failure | `runtime.rs::Mods::call` appends `data.out` before matching callback result | New machinery effects must stay private until success and validation |
| Saves | client `containers.rs`, `save.rs`, `game/saving.rs` | Generic machine records and coherent accounting with chunks/entities/players |
| Network | client `lan/conn.rs`, `net.rs`, `game/remote.rs` | Content negotiation before admission; generation-scoped snapshots and deltas |
| Rendering | SDK render interface; client `block_entity_render.rs` | Mod-owned retained world meshes, transformed instances, server-timestamped state |

On October 27, 2026, [PR 1189](https://github.com/ludovic111/horizon/pull/1189)
was still an open draft at `847210b1e764638c97fce84489b2e9c368668388`. Its block
entity API and sandbox/harness work can inform type registration and batched
callbacks. The PR explicitly excludes actual world creation, ticking and saving.
Its reported furnace results do not establish this port's behavior. Reconcile the
type/slot API before implementation; do not merge the branch wholesale as an S0 step.

[PR 2888](https://github.com/ludovic111/horizon/pull/2888) was an open draft at
`b4a411a394bd346918843a94d8424bf1acd0e652`. Its particles, queries, sounds,
atmosphere and locally observed events are client facilities. They are neither an
authoritative server lifecycle nor a retained moving-mesh interface. Reuse relevant
quota and cleanup conventions if that API lands; do not make it a prerequisite.

## Package identity and compatibility

Proposed package ID and native namespace: `create_horizon`. A bundle contains one
content manifest plus separate server and client Wasm components. Content has one
owner and is registered once, including in the integrated server process. A
dedicated server never instantiates client hooks or a renderer. A client never runs
the authoritative machine callback, including in ordinary singleplayer.

The loader validates package ID, owned namespaces, required capabilities, content
digest, executable digests, host contract revision, network schema and save schema.
Reject duplicate namespace owners, missing required halves, illegal imports and
unsupported schema versions before opening a writable world. Initial compatibility
is exact engine/port/content match; version ranges require later evidence.

Numeric block and item IDs are session-local implementation details. Persist and
compare namespaced IDs with sorted properties and canonical item components. The
world records its required packages and content/save schema digests. A missing or
mismatched package offers an actionable refusal or read-only inspection, never a
silent replacement with air or loss of inaccessible inventory.

## Authority and object identity

The host owns world mutation, item stacks, machine records, loaded chunks and
authoritative time. The guest owns domain calculations and a replaceable cache.
Guest storage is not the authority for machine inventories or durable world state.

A machine key is `(world_id, dimension_id, block_position, generation)`. Generation
identifies the placement, survives unload/reload, and changes on replacement even
with the same block type. The world persists the generation allocation state with
its checkpoint so a restored session cannot alias another placement. Reconnection
also changes the transport session epoch. A stale key cannot access a new object.

Every command validates package ownership, capability, active dimension/chunk,
generation and record version. Player requests additionally validate session,
reach, permissions, held item and operation-specific preconditions. Knowing a key
does not grant permission. Unknown or unloaded positions return unavailable, not
air, and queries never generate chunks implicitly.

## Lifecycle and tick ordering

Events carry world and dimension identity, simulation tick, ordered sequence,
position, generation, old/new canonical state and cause. Capture these when the
mutation commits, not by resolving a player's dimension at callback time. Causes
include player action, command, explosion, piston, other host/mod operation and
load/recovery. A movement identifies both old and new locations; it cannot look
like two unrelated inventory creations.

Separate notifications cover record creation/load, block-state change, removal,
chunk activation/deactivation, unload and world close. Inactive loaded chunks are
not the same as unloaded chunks. Unload retains inventory and generation; removal
retires the old generation and disposes of its contents exactly once through the
host. Neighbor/topology invalidations include both sides of an affected boundary.

Proposed per-level phase, to be verified against reference traces:

1. Accept ordinary simulation work and validated player requests in the host's
   deterministic ingress order. Run vanilla block/entity work for this level.
2. Collect committed lifecycle changes and candidate item/player interaction
   intents with captured context. Drop stale handles; coalesce only invalidations
   whose loss cannot hide a required lifecycle transition.
3. Call the owning server component once with an ordered batch and a read view of
   active records. Compute topology changes, then machine work in a documented
   stable order. The batch uses a single dimension and simulation tick.
4. Validate and atomically commit the staged result. Lifecycle changes caused by
   this commit enter the next batch; do not recursively reenter a component.
5. Publish committed state to tracking clients. A checkpoint is permitted only
   after all dimension batches and relevant vanilla mutations have settled.

Pause/freeze means no machine work advancement. Chunk tickets and level activity
determine eligibility; a global mod tick is insufficient. Snapshot resends and
presentation may continue without simulation. Cross-machine order and the delay
between intake and first processing tick remain explicit observations for P01;
the proposal must change if traces contradict it. Wall-clock time is never recipe
work. Do not silently skip overloaded batches and report successful simulation.

## Transactions and failure

One machine callback batch creates a transaction with a read-set and ordered
operations. The read-set includes relevant machine versions, block generations,
inventories, player slots and dropped-item entity versions. Staged writes are
invisible to the game and other components until commit. A read-your-writes view
inside the transaction is permitted and must be documented.

Narrow initial operations are replace-owned-record, validated owned-block-state
change, transfer/split a stack, consume a dropped item, deliver to player inventory
and spawn a checked remainder. Host inventory APIs preserve item components and
maximum stack sizes. No generic arbitrary entity mutation or guest NBT editing is
needed for the first experiment.

On success, validate quotas, ownership, generations, read versions, item accounting
and every operation, then publish all affected records and inventories together.
If any precondition fails, reject the whole batch and reconcile from a fresh host
snapshot. A replay key `(session_epoch, request_sequence)` deduplicates player
intents; the committed result, including overflow, is returned on a replay.

On trap, fuel exhaustion or invalid output, discard staged effects, retain the
previous committed host state and mark the component faulted. Recreate/reset the
guest cache from committed state before retry or reenable: rolling back only host
writes leaves a divergent guest. Diagnostics may survive a failed call; gameplay
outputs may not. Do not silently reinterpret existing API 0.3 output behavior as
transactional. New machinery calls need their own success/abort boundary.

Removal is a host operation that atomically retires the record, handles its owned
inventory and changes the block. If a guest callback fails during removal, the
host must still have a bounded inventory-preserving outcome; it may not call a
failed guest to recover the only copy of items. Faults cannot duplicate drops.

## Persistence and recovery

A durable record contains type and schema IDs, machine key, canonical block state,
domain payload and host inventory. Dirty tracking belongs to the host. The guest
cannot create an independent save for the same inventory.

Items can cross machine, player and dropped-entity boundaries. Saving just a
machine sidecar atomically is insufficient. The initial experiment must choose a
coherent checkpoint across every participating owner, or a journal whose recovery
establishes the same invariant. A checkpoint manifest becomes authoritative only
after its chunk/entity/player/machine data is durable; retain the previous complete
checkpoint until promotion. Recovery never mixes generations of those files.

This is an acceptance constraint, not a completed design. First map the engine's
actual save ownership and asynchronous writes. If bounded checkpoint integration
is not feasible, stop and propose a smaller research experiment; do not advertise
G1 persistence from a sidecar prototype. Unsupported migrations refuse writable
load. Backups establish rollback; arbitrary downgrade compatibility is not promised.

The initial ownership audit identifies these concrete seams at the pinned revision:

- `game.rs::save_game_with` saves the active level, parked levels, then metadata.
  Autosave can queue work without waiting; this is not an atomic multi-file epoch.
- `game/levels.rs::save_parked_levels` enters and saves each dimension separately.
- `SaveDir` in `save.rs` owns one dimension's region/entity/POI caches and writer.
  `save_later`/`save_entities_later` queue snapshots; `flush` and `writes_finished`
  provide a useful durability barrier and retain errors. Region records are
  written in place, so a pointer to an old directory alone is not a checkpoint.
- `game/persist.rs::save_entities` captures dropped entities; chunk unload has a
  separate save path that also needs to participate in machinery accounting.
- Integrated-owner state passes through `game/lan.rs::store_owner` into level
  metadata. Guest state also uses `lan/playerdata.rs::save`; that function currently
  discards directory/write errors. A successful region barrier cannot certify
  successful player persistence.

H08 must route all participating item-owner writes and errors through one coherent
boundary, including disconnect/unload saves. First investigate an explicit save
epoch with staged files and a complete manifest; account for in-place region caches
before selecting it. Do not implement a broad journal or change vanilla save
compatibility as an incidental part of the one-slot machine experiment. If that
scope expands materially, return the concrete options and cost at the gate.

## Replication and client presentation

Negotiate engine build, package/component digests, required capabilities and
content/network schemas before registry-dependent packets or editable-world
admission. Reject mismatches with the differing field. Transport authentication
remains the server's existing concern; package matching is not authentication.

Send a full snapshot on initial tracking/join and dimension transition, then
ordered versioned deltas. Include world/dimension, object generation, server tick,
snapshot revision and base revision. Discard stale sessions/generations. A missing
base or gap requests resynchronization; do not apply it approximately. Untracking
destroys client instances without destroying the server machine. Reconnect cannot
reuse a previous session's intents or animation handles.

Register immutable mesh/material resources on content load, create retained
instances per tracked machine, and update compact transform/animation parameters
in batches. Avoid replacing ordinary block meshes every frame or drawing both an
ordinary dynamic part and a retained duplicate. The guest does not traverse the
kinetic graph from a frame callback.

The server publishes signed RPM, effective status and phase anchored to a server
simulation tick. The renderer interpolates/extrapolates presentation only; stop and
reverse updates preserve the authoritative anchor. Pause freezes simulation time.
Unload, package disable, reconnect and resource reload retire handles. Dedicated
servers do not allocate GPU resources. Axis, sign and phase conventions must be
measured in P01 before being declared reference-compatible.

## Narrow recipe boundary

The host exposes validated namespaced recipe/tag resources and canonical item
facts. The mod decides milling progress and completion from its reference-derived
domain rules. Recipe `processing_time = 250` is work, not a blanket tick count.
The server revalidates current input/recipe and output constraints at completion;
it does not consume or reserve input at start. The initial inventory is one input
and nine output slots. Dropped-item intake and empty-hand retrieval use the same
transaction and overflow path as other item moves.

The pinned all-nine-full reference case consumes input and discards the uninsertable
recipe result rather than blocking completion. The owner explicitly selected this
compatibility behavior in `docs/reference/COMPATIBILITY_DECISIONS.md` R01. Output
constraint validation must follow that observed rule, not invent backpressure.
This explained recipe-result loss does not relax transfer, abort or recovery
accounting. Native P04 needs a named test for it and for the one-full-slot case.

## First experiment and backlog

The next implementation packet, after G0 acceptance, is one original generic
counter machine, not a Create port. Bound it to one machine type, one owned slot,
one dimension-scoped transaction batch, one current schema, and one retained
diagnostic instance. Exercise the default integrated server first, then a dedicated
server with two exact-match clients. No broad machine framework is required.

| Step | Work | Required evidence before claiming it works |
| --- | --- | --- |
| H02/H03 | Paired executables and deterministic content | Correct side only; missing half/collision rejected; vanilla IDs unchanged |
| H04/H05 | Lifecycle and owned records | Player and command replacement; chunk inactive/unload/reload; correct dimensions and generations |
| H06/H07 | Transactions and one slot | Trap before/after each staged write; stale/replayed intents; item totals unchanged on abort |
| H08 | Coherent save | Restart after each interrupted write/promotion; consistent player/entity/machine accounting |
| H09/H10 | Handshake and replication | Mismatch refused before world admission; join/reconnect/gap/replacement with two clients |
| H11/H12 | Retained presentation | One instance, cleanup, stop/reverse/pause; no renderer in dedicated mode |

Record host-call count, batch size, fuel, memory, tick cost and allocation/instance
counts for this machine. Limits and performance targets remain provisional until
measured on declared hardware. Reuse existing engine test facilities for each
proof; do not build a general experiment platform.

The repeated reference, observed controls/overflow, experimental integration route,
asset choice and named development hardware are recorded in `G0_DECISION.md`.
Performance budgets remain provisional; G1 and the host capabilities above still
require implementation and their own evidence.
