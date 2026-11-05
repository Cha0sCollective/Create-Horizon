# Contraptions 002 — first live bearing decision

Status: read-only design for the project lead and owner, 5 November 2026.
No custody, save schema, moving block, contact, render path, host API or
acceptance rule is implemented by this document. The independent
Contraptions-001 read-only preflight candidate is `494b40de492395695359c3984562ae13c5b50288`;
root has composed its closed source seam in integration but has not yet run
the shared compile/core checks. This proposal does not expand that packet.

## Decision requested

Approve one **private, versioned, client-synced chunk-attachment owner** for a
first live same-chunk Mechanical Bearing assembly, plus a conservative
rotating-clearance and world-render contract. The attachment stores exactly
one `AssemblyRecord` per chunk for this milestone. Its controller position is
a reference, not the owner. The server record alone decides custody, pose,
retry and restoration; clients receive a read-only copy to draw the two
blocks even if the controller disappears. This is a new owner/save-schema
decision and needs the existing owner gate before code.

The pinned Horizon host already has typed `AttachmentType<T>` with codecs and
`Holder::Chunk`, so this narrow owner needs **no new public host API**.
`crates/horizon-client/src/game/mod_api_host.rs:36-54` checks that the chunk
exists and marks it modified; `crates/horizon-client/src/save.rs:1499-1508,1759-1761`
writes and reloads its attachment in the same chunk record as the blocks.
`docs/native-mods.md:3208-3278` documents `get_existing`, `set`, `remove`,
codec errors and chunk-tracker sync behavior. The encoded two-block record
must stay below the host's 1 MiB sync limit; later large assemblies need a
separate compact view. A level attachment also exists but saves in a
separate `horizon_attachments.dat` file (`docs/native-mods.md:3262`); it is not
needed for this one-chunk bearing. A controller block entity alone is smaller
in code but loses the only owner if a non-player replacement removes it.

## Bounded scene and canonical record

One upward-facing `create:mechanical_bearing` holds one front slime block and
one adjacent oak log, with no other attached blocks or block entities. The
controller, both source cells, all swept cells and both 90-degree landing
cells must be inside one loaded chunk and the world boundary. One record per
chunk avoids identity and overlapping-reservation ambiguity in this first
milestone. A second bearing in that chunk refuses assembly while it is owned.
No cross-chunk movement, glue, chassis, inventories, fluids, actors, riders,
passengers, trains or general bearing directions enter this packet.
The private content currently registers no live Mechanical Bearing; the
preflight's controller check is intentionally closed. A later implementation
packet must register its block, block entity, model and shaft connection,
then feed the existing private kinetic service's effective speed/stress into
the portable `Bearing`. That is mod adapter work, not a missing Horizon API.

The proposed private `AssemblyRecord` has a codec version and stable
assembly ID, dimension/chunk/controller identity, source-cell positions and
full block-state names/properties, local coordinates relative to one anchor,
axis/pivot, prior and current continuous angle with tick epoch, intended
quarter-turn landing states, and a state machine: `CapturePending`, `Moving`,
`RestorePending`, `Blocked`. It also records which source removals or target
placements have been verified so retry is idempotent. The codec rejects
missing, malformed, duplicate or unsupported entries instead of interpreting
them as an empty assembly. Only inert block states are encoded here; no item
or block-entity payload is silently dropped. A missing or undecodable owner
stops motion and requires an explicit recovery path, never automatic
recapture over a possibly moving structure.

`Assembly::capture` and `Assembly::landing` from the staged portable core
remain planning operations. One `Rotation` anchor, axis and angle governs
server contact and the display projection; snapped rotation changes the log
axis Z to X at the east landing. The source Java bearing starts at the block
in front of the controller and excludes the controller
(`BearingContraption.java:37-59`). At 32 RPM the recorded nine-tick trace
reaches 86.4 degrees before stop and a 90-degree landing
(`native/create/mixins/create/engine/src/core/contraption/tests.rs:210-266`).
That test is a pinned 6.0.10 trace; selected Java classes are byte-identical
to pinned 6.0.11, but no 6.0.11 runtime trace is claimed.
Java removes the captured blocks and spawns a controlled contraption entity
(`MechanicalBearingBlockEntity.java:167-205`); its controller saves running
angle while the entity saves structure, controller link, axis and angle
(`MechanicalBearingBlockEntity.java:76-98`,
`AbstractContraptionEntity.java:611-620`,
`ControlledContraptionEntity.java:80-97`). The proposed chunk record is the
native equivalent of that separate structure custody, within this much
smaller one-chunk scope. It deliberately retains custody if the controller
disappears; Java's entity may discard itself when it cannot reattach to a
loaded controller (`ControlledContraptionEntity.java:154-173,293-309`).

## World commit and recovery

1. On an authoritative level tick, the existing read-only bridge identifies
   both exact source states, empty/loaded destinations, controller state and
   the one-chunk boundary. Preflight the initial swept region for solids and
   bodies before removing sources, allowing only the two exact source states
   that are about to be captured. Recheck source and target identities
   immediately before every world write. Reject a competing record or
   reserved cell.
2. Encode and set `CapturePending` with both snapshots in the chunk attachment
   **before** removing either block. Remove each through checked
   `native_set_block`, then read back and record verified progress. If either
   write fails or a source has changed unexpectedly, retain the owner and
   retry or restore only cells whose identity is proved. Never manufacture
   a second copy of a still-present source.
3. Only after both source cells are absent may the record become `Moving`.
   Before each angle advance, validate the whole loaded swept envelope and
   body clearance. On obstruction, hold the previous angle and owner; stopping
   requests snapped restoration, not owner deletion.
4. Before restoration, compute all rotated states and locations from the
   record, revalidate every target and check authoritative body clearance
   immediately before materializing each block. The same check applies to
   capture rollback and cold-load recovery. Mark `RestorePending`; place each
   state with a checked `native_set_block` and readback. An occupied target or
   partial write leaves the record in place for idempotent retry. Clear the
   attachment and call
   `Bearing::disassembled()` only when both target blocks are present exactly
   once and the original cells no longer contain duplicate cargo.

`native_set_block` (`native_hooks.rs:8349-8381`) changes one state at a time
and returns a boolean; it offers no two-block atomic transaction. The
attachment and both cells share one chunk save, which supports ordinary
unload/reopen recovery. Abrupt crash consistency still needs a separate fault
injection result; this proposal does not claim an atomic crash transaction.
The bulk `native_edit_blocks` helper skips unloaded runs and rejects NBT
(`native_world_edit.rs:32-55`), so it does not replace the checked commit.

`native_destroy_block` can cancel normal player mining (API 14). While any
assembly record owns blocks, cancel a controller-removal attempt and mark
`RemovalRequested`; restore first, then permit a fresh removal attempt. An
obstructed landing keeps the controller and owner for retry, matching the
accepted finite piston's no-loss removal outcome. Other `Level.setBlock`
changes can be observed through `native_block_changed`, but that event is
after commit and cannot cancel it (`native_hooks.rs:1093-1101`); the chunk
record must survive a controller lost to command or other replacement, stop
motion, and recover the captured blocks when a safe target exists. Direct
storage edits bypass that observer. `native_chunk_load` and
`native_chunk_unload` (API 57), plus a per-level tick independent of the
controller block entity, must rebuild/reconcile the loaded owner index and
resume retries. World close does not fire chunk-unload, so normal shutdown
relies on the chunk's saved attachment and cold-load reconciliation. A
block-entity tick alone cannot recover after controller loss, and its own
entity is temporarily out of the world during its callback
(`native_hooks.rs:3072-3083,6804-6815`).

## Rotating clearance and presentation

For this first milestone, use a conservative swept envelope of the two
rotated full cubes between prior and proposed angles. Every intersected cell
must be loaded and inside the admitted chunk. During initial capture preflight,
the two exact source blocks are the only permitted solids. After capture, a
reserved source or landing cell is allowed only when its current state exactly
matches a block that the record verified as already materialized in the
current commit phase; reservation alone never permits occupancy. Any
third-party solid, including
one placed in an emptied source cell, stalls before angle advance. Check
authoritative players and entities against the same envelope
and stall without pushing, carrying or damaging them. This is deliberate
over-refusal at the envelope edge, to be measured in the test scene. It is
not full Create collision. The existing `s5_clearance.rs:43-55` caps a
linear piston at three moving shapes and four final bounds, and Horizon's
`piston_push.rs:377-425` supplies body enumeration but piston-specific
movement. A narrow two-block rotating-clearance adapter can reuse body
queries; physical push/rider behavior needs a later reviewed contract.

Set the record's attachment with `sync: true`. The host sends a copy to
players tracking its chunk, including on reconnect, and drops that copy when
the chunk leaves (`docs/native-mods.md:3265-3279`). The client reads the
copy's two exact states/local offsets, anchor, axis and previous/current
angles; its interpolated pose is presentation only. For these two full cubes,
draw atlas faces through `native_world_submit` and `native_world_quad` under
`NativeWorldDraw`'s world-relative pose. That hook is independent of block
entities and source blocks (`native_hooks.rs:3166-3174`;
`native_world_draw.rs:1-62,125-164`). Its `&self` render path can read the
local chunk attachment or the remote client's synced attachment copy without
creating a default; the adapter must validate the codec before drawing.
The chunk copy, not a controller update tag, keeps the captured slime/log
visible while restoration is blocked after controller loss. Syncing the tiny
record once per server tick also carries the pose correction on stall/stop;
the client must never use it as contact authority. Peer images must verify
the selected slime/log textures, face orientation, lighting and cleanup when
the owner clears. The BE `native_block_entity_submit` path can draw baked
models while the controller exists, but it cannot cover controller-loss
recovery alone (`native_hooks.rs:3104-3165`). Display entities likewise do
not establish solid contact (`docs/native-mods.md:2543-2551`).

## Acceptance and alternatives

After an owner-approved packet, check normal powered assembly, nine-tick
motion and 90-degree stop, correct log-axis restoration, static obstruction
stall and retry, player/entity envelope refusal, blocked controller mining
and retry, controller replacement recovery, normal mid-motion chunk unload
and cold reopen, and actual integrated/dedicated peer rendering. Verify the
finite accepted piston stone/depot/portable receipts are unaffected; its BE
owner, codec, clearance and renderer remain independent. Require source delta
review and root's host-wide build/runtime reservation before calling this
accepted gameplay. The shared Contraptions-001 compile is a separate gate.

| Option | Result | Decision |
| --- | --- | --- |
| **One saved, synced chunk attachment** (recommended) | Same-chunk custody and client presentation survive controller loss; existing typed host API and chunk save; new private codec, retry and two-cube renderer still required. | Request owner approval for the narrow schema/contact/render packet. |
| Controller BE as sole owner | Reuses the finite piston pattern but normal block replacement deletes its only saved owner; blocking player mining alone is insufficient. | Do not use for this bearing. |
| Level attachment | Independent owner works across chunks, but a separate SavedData file and broad index/reconciliation policy enlarge the first failure boundary. | Revisit for cross-chunk general assemblies or trains. |
| Custom moving entity/host API | Could support broader Create contact and lifetime; current display/marker movement has no solid collision contract. | Defer until a concrete host gap remains after the bounded bearing. |

If the project lead or owner declines this owner/schema/contact decision,
Contraptions-001 stays a read-only planning seam. There is no authorized
custody or motion implementation under the current team packet.
