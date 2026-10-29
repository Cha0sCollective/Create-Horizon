# H08 capture integration audit

PROPOSED source audit and next experiment, not an accepted design, implemented
capture API, writable machine recovery result, S1/G1 pass or reduced deliverable.
The coordinator must qualify/pin the W1 successor and obtain a different fresh
independent design review before releasing any implementation below. This audit
uses Horizon sources only. No code execution, imports, WSL, builds, tests, format,
network, engine edits or commits were performed.

## Recommendation

Build ONE end-to-end capture/publication/recovery mechanism experiment next,
instead of another detached owner observer. Use a fresh resource-backed native
Game with two channel-backed participating players, finite authored inventories,
small resident worlds and an item-free native counter. Run the whole operation
as one actual owner-thread task. Compose PLAYER/E1/W1, close the applicable native
producer/writer path, project records, physically copy the saved native files into
a new immutable epoch, overlay the captured player/counter records, synchronize
it, publish its manifest last, and recover only that complete epoch into a NEW
working directory. Interrupt publication and prove the previous epoch survives.

The initial caller stays in an explicitly fresh, isolated native Game fixture;
no gameplay, SDK, control command, permanent fresh-world flag or broad checkpoint
admission API is needed. This is useful integration code exercised on real native
owners/writers/files, not an assertion that empty diagnostic reports suffice.
It produces an archive/recovery mechanism cell: recovered native world/player
state and an exact detached counter record. It does NOT install that record into
a running persistent machine or qualify machine generation restoration. That
missing native restore seam is stated below, not simulated by replaying ticks.

For this first cell, refuse any dropped entity, World slot trial or unsupported
PLAYER/W1 state. PLAYER may hold actual positive plain item stacks; this is not
an empty-inventory proof. The counter itself has no item inventory. This avoids
inventing an exception to qualified E1 or treating a copied SlotKey's Arc as a
durable generation. These are temporary experiment inputs, never the permanent
S1 admission rules. The accepted dedicated/two-client, one-slot, cross-owner,
unload/reconnect and ordinary in-frame requirements remain on the development
path. Do not call the result coherent item-bearing machine persistence or G1.

No owner decision on journaling or conservation reduction is needed to investigate
this bounded mechanism. The next packet still needs normal explicit design/scope
acceptance. If its author cannot make the proposed built-in producer closure true
within the named files, return that concrete source finding before enlarging it.

## Inspected revisions and evidence boundary

Private branch codex/h02-smoke-h03-notes initially HEAD
fe940659e103394eaf49fe3f652072af65b2c090, initially clean. Read effective AGENTS,
CURRENT_WORK, workspace pins and docs/design/HOST_CONTRACT.md. The contract's
historical baseline cf62fe2 describes the original direction, not today's code.
Read accepted PLAYER projection, E1, save completion/finished handoff and idle
close result notes. Older feasibility prose was used only to locate issues;
current source and accepted results decide whether an issue remains.

Engine initially codex/h08-world-owner-observation at qualified
b41427c6b20d0a7458a3de730dc532d3a68d4081, tree
6142ddc068e22801d6150277e65eadedf6605f8d, with exactly five W1 paths under
qualification: block_entities.rs, world.rs, game/levels.rs, game/brush.rs and new
game/levels/world_owner_tests.rs. During this audit the coordinator committed
those same inspected sources as afb38260b562c2df1b12e36aec304ccf51eb0e31,
tree 3131fc796880a1a04d94c4a8de7a5af6c094ad2c; the engine became clean. This audit
independently establishes neither W1 runtime acceptance nor that commit's final
qualification. Preserve the coordinator's evidence/status.

Selected inspected SHA256 (engine paths relative to crates/horizon-client/src):

| Path | SHA256 |
| --- | --- |
| game/integrated.rs | fb901d6bed8b9b282d74a3ed93484179067a53b219b90a174947853519426a18 |
| streaming.rs | 8e524b6358c335718a2baeef86008c4cef6091e650992990402f7f014d890175 |
| game/lan.rs | df2e041580e3ab456d58ab47240b45250b641caf41e7e9c14285675720fc3d16 |
| game/levels.rs | 1c4803d92d0e34e9280d09ce11d0cfbc5179df77261719df2908daed82788b98 |
| save.rs | 688ae37142aca312fec253d815fb4af8dfb014aad5d808d4b23b08f6aa6abe04 |
| world.rs | cd079e5a6a11604ab2dd5f791292c1cc880bf6b1db73d240d511f1fdfb68adab |
| block_entities.rs | fe440ea43a2e759b2ec87032b07c84e6aa021dd5cb54c9107641ef7bd9acc2d7 |
| game/brush.rs | 3fc7483340e67c8bae01a9543069ae40a47f0dd47f885065e1a3d6437b913fef |
| game/levels/world_owner_tests.rs | f741ec67d89753b938a3a66b130645f7fc626ab950d330cccdc907beefa0e8e2 |

## What can actually be composed

The three checks must execute in the SAME continuous authoritative Game operation,
with the actual native roster freshly observed there. None is a lasting permit.
Do not save an earlier report, resume/pump the world, then treat it as authority.

| Existing seam | Reusable guarantee | Boundary it does not supply |
| --- | --- | --- |
| game/lan.rs::observe_checkpoint_participants / preflight_checkpoint_participants | Current authoritative, non-acting-guest PLAYER identities and matched host/play ownership; rejects unsupported scratch/player transient state; preserves canonical integrated-owner versus external record identity. | Queued connection events, prior detached Left owners, future reconnect or a stable roster after the call. |
| game/lan.rs::project_checkpoint_participants | Calls actual preflight FIRST again; checks every actual root/count and saved compound before any player encoder; returns concrete detached Tag/advancement JSON/stats JSON in roster order. Owner uses a local Level clone; external uses the native temporary defaults. | Encoded bytes, byte/depth/compression bounds, complete arbitrary scalar/component support, file writing, producer closure or omitted history. |
| game/levels.rs::observe_checkpoint_entity_owners (E1) | Current active AND every parked level's named ENTITY owners; finite resident fresh drops may pass; other families/maps, loaded rows, complete slot trials and portal owners refuse. | No drop projection, stack scalar validator, hidden native queues, retained history, block inventories or machine state. |
| game/levels.rs::observe_checkpoint_world_owners (W1 candidate) | Current finite named World/Chunk/BE/Pending/Spill/brush/hopper/pregen causes; fixed three-dimension report. All native BEs and opaque/staged owner cases refuse. | Ticks/updater/function/mod/lent-world owners, worker channels/protos, history, a producer barrier or general world serialization readiness. |
| save_game_with(true), save_level(true), save_parked_levels(true) | Checked native save path; per-dimension writers drained; parked failure restores original active dimension with original-error precedence. Idle close separately preserves partial completed closure policy. | One atomic file epoch, active external guest inclusion, immutability, whole Game lifetime preservation, or closure against new later enqueues. |

Two concrete composition traps are visible now:

1. ParticipantProjections is pub(in crate::game), but its players field and
   ProjectedParticipant fields are PRIVATE to game/lan.rs. An actual archive
   consumer in levels cannot legally read them. Propose a narrow consuming
   exporter there which moves already-built values into typed archive rows;
   retain native identities and do not rebuild them from filenames/connection IDs.
   Do not make all internals public or rerun a different formatter as the oracle.
2. E1::inspect_entity_level refuses ANY World slot trial and ANY retained
   EntityMemory UUID/raw map. game/persist.rs::entity_chunks(&mut self, false)
   calls uuid_for, which inserts random UUIDs in persist.uuids even for fresh
   drops. Therefore E1 can pass before a save and legitimately refuse afterward.
   Never normalize/clear those maps, take the slot, or ignore that refusal to get
   a passing combined flag. The recommended first cell explicitly has NO drops
   or item-bearing machine slot. Later admission of those owners needs a reviewed
   capture-aware projection/identity boundary, not a hidden E1 exception.

PLAYER checks known recursive count casts, not every item scalar. Its existing
count helper deliberately says so. W1 presence checks can preserve unsupported
NaN/UTF16 payloads while refusing them; this does not qualify their round trips.
The first cell's item/component/body values must be explicitly authored, finite,
size-bounded and round-tripped with independent expectations. Encoder success is
necessary but not a universal canonicalization/conservation proof. Runtime-only
BundleSelected remains native transient state, not an invented disk field.

## Owner-thread completion and producer closure

Integrated::call at game/integrated.rs:95 submits a FnOnce Task and waits only ten
seconds. None conflates queue failure, timeout and server cessation. A timed-out
Task can still mutate the Game later; no cancellation occurs. run_tasks at line 565
collects queued tasks and executes each synchronously before apply_publish and
the next task/tick. A blocking task therefore supplies a useful continuous owner
interval, provided the task itself does NOT pump control, network, functions,
streaming update, pregen, callbacks or ticks during its capture/copy operation.

For the experiment, use an OUTER one-shot completion channel whose only sender
is owned by the submitted closure, and wait for its final Result even if the
existing call observer times out. A timeout is Pending, never Failed/Cancelled,
and must not trigger retry or release ownership. Sender drop/thread failure is
not a successful capture receipt; preserve the previous checkpoint. Keep the
experiment's native owner thread and completion handles until it is settled.
No generic Integrated::call behavior change, ten-second extension or new Game
field is required. The new defining-module test must exercise actual Task/
Serving/run_tasks dispatch, including an observer timeout while the task remains
admitted, a later task excluded until completion, and exactly one final receipt.
The test can coordinate its own Task with channels; add no production fault hook.
A bounded input size is not a hard OS-I/O latency guarantee. A hung task remains
unresolved; an external harness timeout must not be relabeled safe cancellation.

server-hold alone is NOT this interval: run_integrated still calls run_tasks.
Pause executes tick_server(true), which services connections. In-frame's ordinary
Game retains self.player and scratch inventory and fails current PLAYER admission;
its saving screen pumps the event loop. This first caller is the authoritative
native server Game fixture, not an ordinary in-frame adaptation or GUI proof.

The current built-in writer graph is narrow enough to investigate closure:

- world_loading.rs::make_streaming constructs Persistent over Flat/Test/Vanilla
  providers. Streaming Job::Save invokes save_protos from a worker; owner-side
  receive_generated can invoke save_finished for an out-of-range fresh Chunk.
  Generate/Decorate/Finish for these providers compute/read state, but their
  returned Proto/Chunk can carry future entities/loot. Arbitrary ChunkGenerator
  implementations are NOT covered by this built-in source argument.
- Streaming::save(true) / save_until reconciles required completion, waits for
  decorating/finishing/saving, and queues written protos. It now RETURNS timeout,
  disconnect and generation-save errors. It does not drain every generating job
  or declare all results/deferred/proto owners absent. busy() also omits saving,
  written, failed_finished and other retained storage state. Neither is a whole
  capture gate by itself.
- SaveDir::flush waits outstanding count zero then returns the retained native
  writer error. Future queue calls are still possible; successful retries can
  write while the old error remains. Failed pending snapshots stay readable.
  The source must stop upstream producers before the last relevant flush.
- A proposed combined operation can conservatively require zero generating,
  decorating, finishing, saving and queued work, empty retained protos/written/
  failed-finished/deferred storage owners, no generation-save error, and no
  disconnected/unreconciled required completion. Add ONE defining-module
  Streaming helper used directly by capture, with native tests for real queue,
  in-flight and returned-result ownership. Do not advertise busy()==false or
  paused=true as proof, consume an unseen result to hide it, drop/join workers
  to lose payloads, or expand to a scheduler service. Examine result-channel
  reachability and the provider binding in exact implementation review.
- Within the admitted synchronous fresh fixture, no update/pregen/mod/function
  pump runs after that check; built-in workers have no outstanding storage job;
  flush all exact native SaveDir owners after native saving and before copying.
  A second coupled check must refuse any remaining ownership gap. This helper
  is subordinate to this operation, not another standalone readiness token.

Native Ticks, FnState private load_pending, updater state and ModsState queues are
NOT certified by W1. The initial caller's fresh Fixture construction plus an
explicit authored list of native setup operations supplies that limited no-
history/no-pump premise; it cannot be supplied by a caller boolean for arbitrary
live Games. No menus, processing ticks, callbacks, unloads, Left/rejoin, portal
travel or mod activation may precede the claimed capture interval in that cell.
Positive PLAYER inventories/counter are seeded directly before the baseline.
A subsequent general runtime caller must map and settle those additional producers
and historical lifetime boundaries before it can reuse this capture operation.

## Immutable image, physical copy and publication

Native region storage is mutable in place: save.rs::write_region_records calls
RegionFile.write/clear and region.sync (sync_data). An Arc<SaveDir>, old directory
name, cached Chunk, backup alias or hardlink to an MCA file is not an immutable
epoch. save.rs::write_dat and JSON helpers use ordinary replacements/writes,
without the cross-file/directory synchronization required for this new claim.

Capture sequence proposed for the single packet:

1. Validate fixed experiment provenance, limits and authoritative context; freshly
   observe/preflight PLAYER, E1 and qualified W1 in this task. For this cell
   require zero resident drops and zero slot trials, all loaded dimensions included.
   Require the native Level player UUID already present/stable, rather than
   letting its first write invent a canonical identity after projection.
   Snapshot typed item-free counter values without take/retirement. Counter state
   uses explicit type/schema/dimension/position plus captured runtime scope and
   instance; mark the latter as runtime identity, not persistent generation.
2. Close the bounded built-in storage producers as above. Native reconciliation
   can change owners: repeat the actual checks/projections AFTER any such work.
   If it admits a refused owner, fail capture retaining it; do not clean it away.
   Obtain actual immutable PLAYER rows and machine values at this final point.
3. Invoke checked native save_game_with(true) while continuously owning Game;
   restore its starting dimension on error via the qualified path. Flush every
   participating SaveDir. Do not call close_idle_levels or retire machine state.
   Ordinary common saves do NOT include all active external guests, so they are
   not a substitute for the explicit captured rows. Save failure publishes none.
4. Still without pumping producers, recursively PHYSICALLY copy the exact owned
   working world files into a fresh epoch staging directory OUTSIDE world_root.
   Reject symlinks/reparse traversal, traversal paths, duplicate paths, existing
   target epochs and configured count/depth/byte overflow. Exclude the native
   session lock by its explicit policy; recovery obtains a new lock. No hardlinks,
   reflink assumptions or copying into the live tree. Preserve complete native
   relative paths, including dimensions/<namespace>/<path>, POI and metadata.
5. Overlay each captured PLAYER Tag and JSON row at its canonical native target;
   use players/data/<uuid>.dat and native advancements/statistics paths, including
   the actual integrated-owner mapping. Native Level::write also writes its own
   player data: do not leave a contradictory duplicate owner row. Record projected
   counter values in one explicitly experimental, item-free archive member.
   Serialization happens only on detached records; enforce declared NBT/JSON and
   total image limits and compare independent readback expectations.
6. Finish all staging writers, then synchronize every copied/overlaid file and
   each directory in a documented bottom-up order. Write a bounded manifest with
   exact normalized paths, lengths/digests, epoch, engine/content/schema identity
   and canonical participant/machine descriptors ONLY after all member data is
   complete. Sync its temporary file, rename to its final name, sync its parent;
   if a staging-directory rename is the publication point, sync the containing
   epoch root too. Define the Linux filesystem durability claim explicitly.
   No existing writer's success substitutes for syncing the NEW physical copies.
7. A committed epoch is immutable and complete by manifest verification. Keep
   the old epoch. Avoid a separate latest pointer in this experiment: select only
   complete verified published epochs, with explicit sequence and refusal/fallback
   policy; pending directories are not eligible. A partial or mismatched generation
   can never contribute individual files to another epoch.
8. Recover by validating ONE complete matching manifest, copying it into another
   fresh working directory, checking native records there and acquiring a NEW
   native save/session lifetime. Never overlay the old live directory or mix old
   player files with new chunks. Native Level/SaveDir/playerdata readback can
   validate real world/player recovery. Counter JSON readback proves its exact
   archival projection only; it is not a live counter restoration implementation.

Current counter_trial::Snapshot is readable, but no native persistence decoder/installer
exists. counter_trial.rs allocates runtime instances from a process-global atomic;
world_slot_trial::SlotKey contains Arc identity and an in-memory incarnation.
Re-enabling a counter allocates another runtime instance; cloning/encoding these
keys does not implement HOST_CONTRACT's durable world/generation allocation state.
Do not fake machine recovery by replaying ticks, restoring an Arc address or
calling enable and asserting the fresh instance equals the saved one. A later
explicit machine-record restore packet must install committed payload/generation
state with a fresh transport session epoch and correct allocator rules.

## One proposed file/test surface

Tentative cap: SEVEN Horizon source paths, accepted before implementation:

- game/levels.rs: paired-gated module declarations only; existing methods unchanged.
- NEW game/levels/capture_integration.rs: actual bounded Game operation, immutable
  image/experimental counter projection, physical copy/manifest/recovery helpers.
  Being a levels child permits borrowing actual active/Parked owners without
  field-publicity changes or fake dimension reconstruction.
- game/lan.rs: narrow consuming exporter for EXISTING ParticipantProjections;
  preserve preflight/projection/writer bodies and canonical identity rules.
- streaming.rs: one operation-coupled built-in producer-closure helper and focused
  defining-module native tests; no new scheduler, fields or worker cancellation.
- NEW game/levels/capture_integration_tests.rs: actual Game/disk/recovery cases.
- game/integrated.rs: paired+test module declaration only; generic call unchanged.
- NEW game/integrated/capture_completion_tests.rs: actual owner Task dispatch and
  separate definite-completion receiver tests. Existing Fixture bodies unchanged.

All names/signatures/schema/limits are PROPOSALS, not existing accepted interfaces.
No dependency, counter restore API, SDK/control/CLI command, new Game field,
retirement ledger, native save schema change or public promotion is preauthorized.
If the publisher is too large for one readable child, propose a mechanical child
split BEFORE exceeding the cap; do not build a general checkpoint framework.

Minimum executable evidence in that ONE packet:

- Actual two-player/channel-backed authoritative Game, real parked levels and
  distinct native blocks, authored positive inventories and native counter values;
  capture then independent native world/player reload plus exact counter projection
  readback. Change the live source AFTER completion; old epoch bytes/digests remain
  identical, with physically distinct files. No claim of external paired peers.
- PLAYER roster change/unsupported inventory, E1 drop/slot, W1 pending BE/spill,
  native writer sticky failure and producer outstanding work each prevent epoch
  publication; old complete checkpoint stays valid and current owners retained.
- Real source/native write and staging mkdir/file/copy/sync/rename failures with
  exact finite write-footprint assertions. No false 'Err means nothing was
  written' rule; staging may be partial and live native saves may have progressed.
- Restart/fail-stop publication checks at data-copy, member-sync, manifest-temp,
  manifest-final and directory-promotion boundaries. Use isolated native child
  processes/test-only phase coordination or real filesystem obstructions; no
  production callback/mock that changes successful capture semantics. Recovery
  must use one complete epoch or refuse, never assemble a mixed generation.
- Manifest truncation/corruption, missing/extra/changed member, wrong build/content,
  invalid path, symlink, stale pending directory and size-limit negatives.
- Actual Task timeout remains pending, no intervening task/tick before completion,
  one receipt/no implicit retry; failed/disconnected completion never reports a
  committed artifact. Tests must keep native owners alive through each assertion.

Coordinator owns source freeze, fresh independent design/source/evidence review,
all actual isolated native/resource execution and appropriate existing regressions.
A passed library publisher unit test is not a Game capture/recovery result.
Initial target is the already accepted Linux validation environment; directory-
sync portability and Windows publication semantics are not implied.

## Open history and decision boundary

Current checked playerdata::save now returns Result, Streaming completion now
fails on timeout, dirty snapshots stay until enqueue acceptance, raid transition
retention and save/idle original-dimension restoration have been repaired. Old
notes saying those precise failures are still silently ignored are stale.
Their CALLER lifetime problem is still real: Hub::poll removes a guest before
HostEvent::Left; game/lan.rs handles it, logs save errors and removes play state.
A same-batch rejoin can load stale files. Cursor/merchant owners can be omitted;
unload_entities uses remove=true before a potentially failing handoff; portal and
queue-tail histories remain separate. Neither a present empty report nor a new
successful physical copy recovers an item already lost in that history.

If the owner asks to make the limited archive cell the permanent S1 result, or
requires arbitrary existing live worlds immediately, that is a concrete decision:

| Option | Cost/surface | Tradeoff |
| --- | --- | --- |
| Keep HOST_CONTRACT and proceed with the bounded cell, then integrate real machine generation/slot and failed lifetime/admission | Proposed seven-path mechanism packet now; later explicit machine restoration and the actual Left/unload/portal/menu/producers that the code demonstrates | Delivers executable capture plumbing now while retaining full S1 obligations; no estimate that the small cell completes G1. Recommended. |
| Immediately implement general retained ownership/journal | Hub::poll/Left dispatch, GuestPlay/menus, disconnect/reconnect ordering, unload/persist, portal transfers, shutdown/Game drop, machine slots, admission and recovery all participate; journal replay/dedup/version rules become new durable design | Substantially broader source and failure matrix; requires explicit owner architecture acceptance before implementation. Not forced merely by difficulty with the bounded cell. |
| Permanently restrict worlds/owners or allow failed-transfer/recovery loss | Changes HOST_CONTRACT persistence and lifecycle/conservation promises plus specification/G1 criteria | Less implementation only by reducing the product guarantee; requires an explicit owner choice. This audit does not recommend or authorize it. |

No current source finding requires asking the owner to choose these broader options
before the first bounded cell can be designed and reviewed. The missing general
history/admission, machine-generation installer and in-frame adapter are specific
open work, not an excuse to launch a standing team or a long detached-observer
series. Stop this audit at the proposal; implement nothing until release.
