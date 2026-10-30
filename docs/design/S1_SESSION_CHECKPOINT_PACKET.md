# Native session checkpoint proposal

Accepted current-resident save clarification fc93a257 is recorded in
S1_SESSION_CHECKPOINT_RESIDENT_SAVE_CLARIFICATION.md: checkpoint-specific real native
resident submissions follow save_game_with(false) and precede the same second fence.
Original proposal47cf3a13 remains preserved as its exact accepted snapshot.

DESIGN ACCEPTED at exact amended proposal
47cf3a13e2ebdf136c6dab372249ffd2783c64b4158fe3ff56ad10f6e01a0997 by fresh independent
native read-only Sol High review. Original4024fcc4 was withheld; all three corrections
and their source basis are retained below. Exact accepted snapshot is workspace
.local/s1-session-checkpoint-proposal2.md; no production source released yet.
This follows the closed native writer test-control packet, still under qualification.
Its different final acceptance, exact clean successor revision and private checkpoint
must precede author release. Design acceptance grants no source or runtime acceptance.
It does not enable ordinary
filled-slot gameplay, startup, commands, close, networking, or G1. Research capture
APIs and their pristine-history guards remain unchanged.

## Scope and intended result

Add an internal synchronous checkpoint and checked fresh-recovery primitive for
one identified native TestTerrain session, one initial Overworld, one healthy active
persistent counter and its active persistent slot, actual participating inventories,
and current counter/slot schemas. Support genuine previous generation and save
history with variable resident coverage. Do not impose nine columns, fixed player
UUIDs, fixed counter/slot counts, or a pristine-streamer prerequisite.

Deferred supported-machine lifecycle work remains necessary for S1: ordinary startup
and authorization, replacement/retirement/drop ownership, disconnect failure,
close/open callers, actual admitted peers, replication and committed presentation.
No broad journal, native save-format change, global codec rewrite or relaxed item
conservation is proposed.

Candidate engine base is the accepted writer-test-control successor of local
765dc8d0fa893d5ee89c24fd02fa394f434961eb; root must record its exact clean revision
before release. Exactly these eight paths, relative to crates/horizon-client/src:

1. game/levels.rs: module declaration and session-specific owner inspection.
2. NEW game/levels/session_checkpoint.rs: the internal operation, local envelope,
   bounded physical image, validation, publication and recovery preparation.
3. NEW game/levels/session_checkpoint_tests.rs: defining controls and regressions.
4. game/lan.rs: production detached rows from validated participant projections.
5. game/world_loading.rs: constructor-issued native provider provenance and checked
   unpublished recovery preparation; preserve the existing make_streaming wrapper.
6. game/world_loading/host_identity.rs: trusted expected-world comparison under lock.
7. game.rs: provenance ownership and internal capture/open integration.
8. game/counter_control.rs: invalidate provenance in common retire_counter_trial.

The eighth path addresses existing remote/fixture world replacements outside the
original seven-path proposal. Comparing only save Arc, root and dimension cannot
detect a new World installed beside the same SaveDir. No remote/fixture test files
are released. Preserve the existing three-value make_streaming wrapper because
three existing research test files destructure it; use a separate private witnessed
constructor, rather than changing those tests.

## Identity and ownership

Capture derives world UUID, target, type binding, package identity, dimensions,
roster and values from the actual authoritative Game. Callers supply only archive
root, unused monotonically selected epoch and deadline. No caller-supplied build
hash, claimed package match or decoded wire contract grants provenance.

A private non-clone witness must be issued by the actual native Persistent<TestTerrain>
construction, retained with its corresponding prepared/current World, SaveDir and
root/dimension, and invalidated on all retirement/world replacement/level swaps.
No eligibility recreation from options.test_terrain, a successful fence, UUID bytes,
or swapping back to an earlier SaveDir. A fresh identified native open is required
after invalidation in this bounded primitive. Subsequent ordinary lifecycle work
must deliberately extend that policy rather than silently inferring provenance.

Require actual identified WorldSession/session.lock, no remote client or acting
guest, no pending open/entry/travel/save operation, no parked dimensions, no locator,
pregeneration or external writer, and no unsettled menu/crafting/participant state.
Project the healthy counter and persistent slot through their existing checked
APIs. Pending retirement and unsupported entity owners refuse without mutation.

Add production SessionPlayerRow/into_session_rows alongside, not by enabling, the
test-only CapturePlayerRow. Retain real UUID/record UUID/storage-role/dimension,
player NBT, advancements and stats. Reuse actual participant mappings and preflight,
including a dedicated roster without a fabricated integrated owner. A player-cell
adapter is not authorization evidence; ordinary ingress remains dormant.

## Capture transaction

Hold one continuous exclusive &mut Game through producer reconciliation, native
save, projection, copy and publication. No ticking, task pumping, connection
processing or transferable readiness token inside this operation.

1. Validate context/provenance/package pins through existing check_world.
2. Close existing producers with native_write_fence_until and real SaveDir
   writes_finished, using the request deadline. Reconcile genuine late Finish work.
3. Inspect named owners and take the actual roster, current counter/slot/player
   projections and resident-coordinate set AFTER that first fence.
4. save_game_with(false), propagating native errors. Never save(true) or flush.
5. Fence again under the SAME deadline, closing writes submitted by this save.
6. Recheck provenance, identity, packages, roster, owners and projections; semantic
   values and resident membership must still agree. Native saved-field projections
   include actual ticks and metadata; do not compare only blocks or raw stale chunks.
7. Copy the complete bounded source tree, excluding session.lock, into a fresh
   sibling pending directory. Recheck source inventory and hashes after copying.
8. Overlay detached participant files and current counter/slot frames ONLY in the
   pending copy. Never require writing host frames into the working native root.
9. Sync files/directories, complete the manifest last, verify physical and semantic
   contents, then attempt directory promotion and sync the archive parent.

The clock in both frames and native level.dat must correlate. Resolve prepared type
through the captured catalog; NativeBinding uses its deliberately physical iron
mapping, not a transient alias-registry ID. Require the existing bounded package pin
file before and after copy/recovery and bind its actual member hash in the envelope.

## Closed native and player projection

Do not ignore an OpaqueChunkRoot result from the existing minimum-refusal observer:
that can conceal another owner. The session-specific levels.rs census repeats the
full named owner checks, admitting only separately validated native metadata.
Entity inspection permits exactly the slot already accounted by checked persistent
slot projection, preserving all other refusals; no research slot_epoch_projection.

Inspect EVERY copied region record, including off-resident entries. Independently
validate each raw region BEFORE RegionFile::chunks/read: complete 8 KiB header,
every nonzero location has nonzero sector count, starts beyond the header and lies
wholly within the bounded physical file, no overlapping referenced sectors, valid
payload length/compression and no untracked external ownership. This first profile
refuses external-record flags and .mcc files rather than following external paths.
Do not allow the native constructor to zero a malformed location or pad a truncated
header into an apparent empty region. Check zero/unused locations separately from
malformed nonzero entries; fail on corruption anywhere, not just required targets.
Use existing RegionFile::chunks/read only after this physical preflight.
Native decoding alone is not validation: malformed
or unsupported fields cannot be made safe by a reader that omits/coerces them.
The raw full root field table is closed: DataVersion:Int(current), xPos/zPos:Int
matching region coordinates, yPos:Int matching Overworld min section, Status:String
(minecraft:full only), LastUpdate:Long, isLightOn:Byte(0/1), sections:List, empty
block_entities:List, optional empty entities:List, Heightmaps:
Compound of native generated heightmap names with correctly sized LongArrays,
empty block_ticks/fluid_ticks:Lists, InhabitedTime:Long, structures:Compound with
exact empty References/starts Compounds, and PostProcessing:List containing exactly
the Overworld section count of empty Lists. Require each generated section's exact
native fields/types, unique in-range Y, valid palette/data lengths and registry IDs;
SkyLight/BlockLight arrays have native nibble sizes. No arbitrary section compounds
or unknown palette entries. Enforce decompression/node limits before decoding; use
the existing native decoder only after raw shape checks, then verify supported
save-projection semantics. Absent optional generated fields must match the actual
current encoder's no-table/lightless forms, never silently default malformed fields.
Recognize typed nonnegative LastUpdate <= saved clock and nonnegative
InhabitedTime; recognized empty structures and empty postprocessing; reject unknown
opaque roots, ticks, entities, item/loot/block-entity owners and unresolved sources.
Fresh live chunks may lack saved extras. No current/previous-tick-only restriction.
The actual TestTerrain provider implements neither saved_proto nor restore_proto.
Therefore this profile explicitly REFUSES every saved proto/status other than full,
including off-resident records, and requires full native records for recovery
coverage. Do not archive a proto and later regenerate it as a successful restore.
Supporting saved proto ownership needs a separate scoped generator packet; normal
successful TestTerrain full-generation/save history remains required here.

Participant projection accepts finite signed zero under the native writer/read-back
semantics, preserving finite nonzero values. Reject nonfinite and unsupported item
graphs before encoding. Do not reuse research tag_bounds' bit-exact -0 refusal or
change the shared native codec to support arbitrary malformed graphs.
Session-specific lan.rs projection MUST derive body state from actual live bodies,
not merely clone cached self.level or accept playerdata::store's omitted fields.
Explicitly project and restore supported native saved body fields, including Motion
velocity, food exhaustion and food tick timer as well as health/absorption/fire/air/
fall/food values handled by load_saved_state_once. Reject unsupported nondefault
saved fields rather than accepting a cached value that was never installed. This
also applies during capture before overlay; existing research projections stay
unchanged. Compare direct live-body state, then its native-semantic projection.

## Local envelope and publication

Propose local format/kind horizon-native-session-checkpoint, version 1. This is not
a network format or a change to counter V2/slot schemas. Strict fields: format/kind,
epoch, world UUID, dimension, saved clock, counter/slot relative paths, actual resident
coordinates, participant descriptors, exact sorted directory/file inventory, byte
sizes and SHA-256 hashes. No arbitrary build claims or authenticated-peer meaning.

Reuse concrete initial bounds from the existing small local image: 256 total tree
entries, 32 MiB/file, 64 MiB/image, 256 KiB manifest, 1 MiB JSON or decoded NBT,
16,384 structured nodes. Bounds apply during reads/serialization, with checked sums.
This is a bounded small-world primitive, not an unlimited-world promise. Root names
are .pending-<epoch> and epoch-<epoch>; manifest.json is completed last. Counter and
slot members are host/counter.json (existing bounded V2 encoding) and host/slot.nbt
(existing bounded slot encoding) outside the copied world subtree.

Reject invalid/overlapping paths, existing destinations, symlinks/nonregular entries,
duplicate/unknown envelope fields, inconsistent inventories and unsupported native
content. Physical copies only, no hard links. Validate source files before/after
copy and the complete candidate before promotion. Preserve prior complete epochs
and failed pending evidence. No automatic destructive cleanup/retry under one epoch.

Errors before final directory rename are NotPublished. From the rename ATTEMPT
onward use Uncertain unless nonpublication is actually established, including parent
sync errors. Never report definite rollback just because a rename call failed.

## Fresh checked recovery

Request supplies trusted expected world UUID, expected target/type, archive root,
exclusive nonexistent destination and deadline. Select the newest published complete
epoch in that identity context; ignore incomplete pending directories. Refuse an
incompatible/corrupt complete successor rather than silently skipping it or mixing
members from another epoch.

Validate every selected member and native/package/frame correlation before copying.
Create the destination exclusively, acquire/retain its native session lock and verify
again under lock. Use RequireExistingIdentity; missing native level data cannot fall
back to a fresh world. Reject dimension/time/weather/look overrides. Verify required
native record coverage before a loader could regenerate a missing/corrupt target.
Drive actual PreparedWorld streaming.update(&mut world) to target/participant
residency without simulation ticks or fabricated chunks, then fence and perform the
final census. Additional actual arrivals are possible; recovery does not assume
manifest and restored resident sets are identical. Restore counter then slot into
this UNPUBLISHED candidate World, without requiring light completion.

Recovery consumes a fresh headless Game by value and a fresh actual native LanHost
roster, returning Game/receipt only after every check. Require no live Integrated
wrapper, existing session/world authority, remote client, LAN host, local item owner
or pending lifecycle operation. Keep ownership through incremental finish_open_world
and participant restoration; every failure drops the whole candidate. Never return
a partially installed Game or promise rollback into a reused live Game.

Add a separate internal lan.rs checked row installer. Match the supplied actual
profile/record UUID/storage-role roster, permit fresh connection identities, and
require correct integrated-owner mapping (none fabricated for dedicated mode),
fresh empty containers/play state and no unresolved packet/menu/transfer ownership.
Strictly verified rows come from image verification, not arbitrary caller items.
Use playerdata::apply only after empty-owner checks, and public PlayerMotion::new /
load_saved_state_once for body restoration. Do not use no-op as_guest: leave_guest
can apply a totem and send queued packets. Consume staged body/absorption once,
explicitly restore velocity and food exhaustion/timer through their public fields,
restore advancements/stats and re-project all actual rows under native-semantic
equality. Any silent native default/coercion is an error. Clear only the redundant
native local-player cache while moving its verified owner into the attached host;
reproject counter/slot/players and account every item before returning fresh keys.
Old session keys refuse. This supplies no ordinary join, reconnect or admission path.

## Decisive qualification and stop boundary

Root alone formats/builds/tests, serially, in isolated scratch environments. Fresh
independent design acceptance, private checkpoint and exact clean source base precede
author release; exact-source clearance precedes execution; different final review.

- Actual identified native Game, genuine residency, physical bootstrap, empty slot,
  actual participant transfer and later successful native I/O. Nonzero counter,
  positive slot/player shares, two different resident shapes, no nine-column gate.
- Capture/recover, advance/transfer, recapture/recover again; conservation, persisted
  revisions/high-water identity and fresh runtime keys/stale-key refusal.
- Real writer Block timeout/release/retry, controlled actual Panic, real filesystem
  sticky error and late built-in Finish storage handoff, with no false publication.
  Panic fixture has NO live Integrated wrapper, releases controls and settles owned
  tasks, then directly drops without flush. Never repair pending/counts for a pass.
- Roster/projection/world/package/schema/native-root mismatch, missing/corrupt member,
  signed-zero success/nonfinite refusal and failed second install preservation.
- Raw off-resident malformed region location, overlapping/out-of-bounds sector,
  truncated header and external-record refusal; native enumeration cannot hide any
  entry. Structurally valid saved proto refuses before regeneration under TestTerrain.
- Nonzero native body velocity/exhaustion/timer survive recovery; changing only the
  actual restored body while cached Level remains identical must fail verification.
- Every common retirement/world-only reset and level swap away/back invalidates
  provenance. Existing streaming wrapper/test consumers remain unchanged. Required
  native target corruption refuses before regeneration; recovery advances no clocks.
- Actual owner/external native guest objects restore with fresh connections; missing
  row, wrong mapping, coercive JSON/body failure or any post-counter install failure
  returns no Game. No fixture roster is labeled actual network admission.
- Copy, manifest, promotion and post-rename sync interruptions: previous complete
  image preserved; definite nonpublication distinguished from uncertainty.
- Relevant retained library/actual Game/feature/fast checks and ordinary upstream
  self-check; both game modes only for behavior actually activated by this packet.

Stop before extra paths, active ordinary filled-slot caller, generalized journal,
native save-compatibility change, global codec redesign or relaxed conservation.
No owner policy decision or permission expansion is adopted by this proposal.
