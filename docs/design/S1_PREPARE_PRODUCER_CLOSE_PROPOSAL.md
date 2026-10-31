# Retained preparation and native producer close: C0a

Status: PROPOSED bounded component. This document grants no source authority.
S1/G1 remain open. Ordinary startup, networking and close activation stay dormant.

## Inputs and dependency order

This refines items 11,22-24,44 of the planning-only paired configuration proposal,
SHA256 95484fa4861f26d8f66c6756bccaa7cbe2bc73c70a98aa5507315dec4d3c3550.
Read-only research used dropped-item worktree base ab239890cd62595889f7f0302cd1396a5e2c547c
with its uncommitted nine-path D candidate, and login clean candidate
5b41f44fe83b53fc6b7c4dfe550cc0bc1aca5b83. Neither is an accepted combined input.
Before source assignment require B FINAL, D FINAL, a clean independently accepted
combined successor with exact parent/tree/source inventory, and a private durable
checkpoint. Refresh all locations and D's native-record interfaces at that pin.

Concrete prerequisite discovered here: C0b retained native writer FINAL must
precede C0a terminal preparation cleanup. Current SaveDir::writes_finished only
polls outstanding work/sticky error. It neither closes the writer sender nor
joins its actual handle. SaveDir::Drop unconditionally joins that private handle
(save.rs:2526). No change within the five C0a paths can replace that lifecycle.
Reorder C0b before C0a; obtain independent acceptance of this dependency first.
A producer-only SealedStream is explicitly nonterminal and cannot release a raw
lock or claim that a PreparedWorld is closed. There is no hidden destructor join,
Arc-count readiness test, detached cleanup worker or permanent refusal substitute.

C0b must expose checked close/retry/completion over the SAME SaveDir allocation,
original pending native Records and actual writer handle, including a successful
no-writer case. C0a consumes that capability after its producers finish. If its
accepted API cannot support this sequence, revise this bounded proposal before
source work; the larger 46-path map supplies no implicit permission to add paths.

## Exact source budget and outcome

Only five paths under crates/horizon-client/src:

1. game.rs: dormant retained preparation field/constructor initialization and
   internal checked harvest/adoption entry; WorldRestLoader finished-only poll/
   harvest primitive. No ordinary route, event, signal or external guard changes.
2. streaming.rs: retained partial start owner, paired submission seal, bounded
   all-origin close cursor and intentional worker stop/finished-only joins;
   focused inline real-worker tests and narrowly gated native test controls.
3. game/world_loading.rs: PrepareOwner, paired retained start/outcome/cancel/
   poll/close; native generator construction variant with partial owner handoff.
4. game/world_loading/host_identity.rs: borrowed retained raw-lock finish, with
   post-acquisition errors retaining preflight/lock/authority/native artifacts.
5. NEW game/world_loading/retained_prepare_tests.rs: actual native preparation,
   delayed outcomes, ownership, close and no-disposal cancellation controls.

The new test module is declared inside world_loading.rs; game.rs already declares
world_loading. No new general executor, native codec, protocol, journal, content
schema, persisted state field or save compatibility promise. Generic streaming
services remain in the engine; private domain code does not enter public source.
Independent exact design acceptance precedes authoring, exact source/tool CLEAR
precedes Root runtime, and a DIFFERENT FINAL follows actual qualification.

Output: genuine native resources may be prepared, canceled, harvested once and
explicitly closed without dropping unfinished work/locks. Healthy producer close
and healthy C0b writer completion yield a terminal preparation-close witness.
World/player authority, ordinary Game teardown and paired network admission are
not outputs. Later C0e wires these primitives into all real application paths.

## Concrete ownership and API sketch

Names are proposed internal services, not stable/public host APIs.

PrepareOwner is created before first native acquisition. It contains request/
intent, optional raw SessionLock, IdentityPreflight, acquired WorldAuthority,
WorldSession, Arc<SaveDir>, retained partial/full Streaming, Level/World when
present, rand sequences/locator/native origin, and stored original error. Its
armed lifetime guard precedes acquired fields. Planned failure returns an owned
PrepareFailure containing this SAME owner; no Result<String> discards locals.
A canceled result remains an outcome, not a second world entry or forced adoption.

WorldRequest::prepare_retained runs native acquisition on the actual retained
initialization worker. Each acquired helper is put in PrepareOwner before the
next fallible step. Native identity/package checks keep their order and strict
existing-world fallback refusals. Filesystem work remains on this worker; polling
its status performs no identity write/read/sync. On an expected error return the
owned outcome. Partial artifacts remain. Unexpected panic/unwind is a fault and
uses the adopted ownership backstop, never a planned cleanup path or success.

IdentityPreflight::finish_retained(root, &mut Option<SessionLock>) performs all
fallible creation/write/sync while borrowing the original lock. Only after success
may it take that lock into WorldSession. Failure returns the preflight/intent with
the raw lock still present. No new identity is recreated on close Retry. Existing
consuming finish stays on the unchanged legacy path. install_authority retains
its existing borrowed-option behavior; an authority error cannot lose that owner.

WorldLoader gains a separate paired retained variant (or equally narrow concrete
RetainedWorldLoader). It owns the actual initialization JoinHandle and a cancel
intent; its result is PreparedOutcome::Ready(PreparedWorld) or Failed(PrepareFailure).
`poll_retained_outcome` checks is_finished before join and consumes the result
ONCE. Pending preserves the handle even beyond the original close deadline.
`request_cancel` changes intent only; it never calls existing cancel's disposal
thread or drop(self.finish()). A late Ready is harvested into the retained owner
and selected for checked adoption only while the original entry token is current
and no close intent exists. Otherwise its original resources are closed.

The internal Game adoption entry rejects stale entry/replacement/closing states
BEFORE moving the PreparedWorld; refusal returns that same object. All fallible
prechecks happen before its commit. Where existing finish_open_world performs
further fallible work, do not call it speculatively then lose its consumed input:
C0a supplies only a dormant checked transfer seam, and C0e must separately qualify
the complete normal adoption path. A transfer witness is not ordinary startup.

WorldRestLoader is resource-only but still owns a real helper. Add a nonblocking
finished-only harvest primitive in game.rs, retaining its handle after errors or
close intent until actual completion; no blocking join_world_rest/sleep in a poll.
C0e remains responsible for wiring it into every covered real return path.

## Retained Streaming construction

Add `Streaming::start_retained` distinct from existing `new`. Allocate actual
Shared, result receiver, vectors and partial owner BEFORE the first spawn. Register
each successful actual JoinHandle immediately; a subsequent spawn failure returns
StreamingStartFailure with that exact partial owner. Do not collect/expect workers
and destroy prior handles on normal failure. Worker initialization can remain
blocked in ChunkGenerator::worker while the partial owner is polled and retained.
Initialized, stopped, panicked and joined are separate observations.

make_native_streaming's retained branch installs SaveDir in PrepareOwner BEFORE
fallible generator construction or metadata reads. Streaming is retained before
post-streamer metadata/identity failures. A partial prepare with no World yet has
submitted no native jobs; it can stop/join those idle/initializing workers without
inventing a World. A Ready or active owner uses its original World for reconciliation.
If the actual implementation submits work before World exists, stop and revise
that invariant rather than constructing a fake World or clearing returned payloads.

The vanilla generator branch currently calls optional search_rings_behind at
world_loading.rs:514. That helper detaches its successful JoinHandle and can hold
a strong terrain reference while searching (vanillagen.rs:363-399); a weak initial
capture does not establish finished-thread proof. The retained paired branch does
NOT start this optional background precomputation. It uses the same VanillaTerrain
and native synchronous/query/worker generation methods; legacy/default construction
keeps its current optimization. No new vanillagen.rs path is authorized. TestTerrain
and flat providers retain their genuine native construction. If another detached
native helper is found on this retained route, revise scope before authoring; do
not advertise all-helper close merely from Streaming's thread list.

## Bounded producer close and retry

`begin_retained_close` installs a non-Clone close state in the actual Streaming.
It records same allocation, original worker membership, world identity, original
absolute deadline and monotonic failure history. Seal ordinary generation,
decoration, mesh, light, distance/pregen and save submission paths. An accepted
close-specific native storage handoff is explicit and cannot reopen scheduling.
Do not set shared.stop while any queued job still owns unique native payloads:
workers test stop BEFORE queue.pop in the current implementation.

`poll_retained_close(&mut original_world, budget)` does only try_recv, bounded
retained/deferred cursor work, nonwaiting lock transitions and is_finished checks.
At most16 result actions or2ms between indivisible actions per turn. No unbounded
take_deferred scan, recv_timeout, sleep, update, poll_save or native_write_fence_until.
Do not hide a blocking shared.queue/ownership/returned lock behind a poll name:
contention returns Pending with unchanged ownership. A native encode/callback/
one record action can remain indivisible; this is no hard2ms OS/encoding bound.
Later owned metadata/image IO remains C0c/C0d, not smuggled into this service.

Track all eight actual NativeWorkOrigin values, not merely currently generating
columns: Generate, Decorate, Finish, Light, Mesh, Save, SaveProtosCallback and
SaveFinishedCallback. Carry original WorkLease and full payload through each step.

- Proto/Decorated/Saved restore the exact checked-out proto/region owners to their
  corresponding map or explicit close-retained handoff. Preserve written markers
  until actual native storage acceptance. A failed handoff retains the same records.
- Generate/Finish retain the entire native finished column and entity/block-entity
  payload. Reuse the accepted D record/staging/classification rules, actual writer
  and ambiguity refusals after refreshing its exact APIs. Do not call tolerant
  receive_generated into newly live entities, overwrite a resident machine column,
  discard off-range records, regenerate a missing proto or substitute air.
- Light matches its native column/sequence and reconciles saved lighting on the
  original frozen resident owner without scheduling. Stale derived light receives
  an explicit settled disposition under native sequence rules. Mesh likewise
  checks its origin/sequence and settles derived display output without GPU upload.
  Their real queued/held jobs must finish; no count omission or queue destruction.

For every origin, submitted facts equal queued + worker-held + returned +
reconciling + explicitly settled/retained disposition. Distinguish storage accepted,
writer durable and actual thread joined. Existing cumulative failure facts stay
visible; no resetting accounting/disconnected/receipt faults to create readiness.
Uncorrelatable missing unique payload remains owned Blocked with its actual error.
Repairable native writer errors use accepted C0b retry with original Records.

Once ALL origins/queues/results/deferred cursors have reconciled and native
handoffs are accepted, publish intentional stop while holding the SAME queue mutex
used for worker wait, then notify_all and release it. Poll actual handles; join
only finished handles and check their result. Initialization blocked/queue lock
contended/worker held/unfinished handle -> Pending, then retained Blocked at original
deadline. Do not destroy queue/deferred payloads, use Drop to join or treat a joined
panic as successful initialization. Partial constructor spawn failure closes all
successfully started handles through this same service before returning its stored
startup error. SealedStream records exact allocation/membership and joined producer
outcome, without claiming writer durability or complete native close.

## Terminal preparation cleanup after C0b

A canceled/failed PreparedOutcome remains in the retained lexical owner. Poll
producer close first; after SealedStream, consume the accepted C0b writer close
service on that SAME SaveDir allocation. Retry is a typed explicit call, allowed
only after the previous actual operation is harvested; it does not replay world
creation, identity creation, generation, simulation or canceled adoption. Both
producer/writer unfinished handles retain their original owner and native lock.

After successful writer close, verify no pending native payload/record/handle,
acknowledge actual producer and writer witnesses, then release SessionLock/
WorldSession/authority once and publish PreparationClosed with stored original
startup/cancel result. Native no-writer/no-job partial failures must close too.
For Ready canceled worlds preserve generated/native artifacts; success is cleanup,
not a claim that a canceled fresh world was published or game-saved. A live active
Game's final resident snapshot/image and coherent close belong to C0d/C0e.

The retained service's Drop is never its normal cleanup algorithm. An armed
unresolved ownership backstop reports unexpected destruction; planned error,
deadline, cancellation and return paths retain state for explicit resolution.
No intentionally stalled fixture is allowed to unwind into that backstop as normal
test cleanup: controls release, then poll and reap actual workers before assertions
that can fail or before the harness may relinquish its owner.

## Required concrete qualification

Root alone runs all build/fmt/test/game/Git work serially with fresh isolated roots,
resource manifest and genuine prepared package fixtures. Native tests spawn no
Cargo/Python/Java/test children. Test-only native controls are fixed-action,
non-Clone receipts gated native-writer-test-control AND debug assertions where
cross-bin access is necessary; inline tests may use cfg(test). No arbitrary
callback/public fake readiness or substitute provider constitutes a positive.

- Healthy retained prepare/adoption seam and healthy canceled Ready close; same
  native resource/lock/authority identity before/after, actual C0b join, fresh lock
  reacquisition only AFTER PreparationClosed. Include TestTerrain, native flat and
  VanillaTerrain path; confirm paired branch spawns no detached Ring search.
- Controlled first and later worker initialization, later thread-spawn failure
  and actual queue mutex contention. Poll repeatedly without blocking; deadline
  retains original partial owner/handles. Release and explicitly Retry, then
  checked actual joins, no extra worker and correct stored startup error.
- Genuine queued/held/returned/deferred Generate, Decorate, Finish, Save, Light
  and Mesh jobs, plus native storage callbacks. Hold real initialization and EACH
  work origin independently. Poll/status observations across turns; release and
  prove exact all-origin conservation and actual finished/reaped handles. Native
  Light/Mesh results use real inputs, not invented Done or disabled mesh count.
- Stale light/mesh sequence, out-of-range finished native records, saved/reloaded
  columns, D-supported entity payloads, returned mixed-origin ordering and
  receipt/accounting faults. Preserve unique original payloads and native UUIDs;
  no resident machine replacement or tolerant native-record filtering.
- Prepare errors before acquisition, immediately after raw SessionLock, identity
  write/sync/authority failure, immediately after SaveDir, during worker init and
  after Streaming but before final level metadata completion. Each post-acquisition
  expected error returns actual owners; no next-world lock acquisition until close.
- Cancel before entry, while native acquire is pending, after acquisition and late
  Ready/late Failed. Actual initialization thread finishes and outcome harvest is
  once-only; no disposal thread, implicit Drop join, stale adoption or second
  outcome. Cover explicit replacement/entry-token invalidation through dormant
  Game seam, separately from still-unimplemented ordinary UI wiring.
- Original close deadline expiry, repeated cancel/close intents (not implicit
  Retry), explicit Retry with newer bounded attempt but SAME retained native owners,
  writer held/IO error then real repair/retry using C0b. Before retry, no queued
  native records removed, lock released or new-world entry granted.
- WorldRestLoader actual delayed completion and finished-only one-shot harvest;
  resource-only failure is distinct from native world acquisition. No poll sleeps.
- Re-run D delayed native column, raw native-record/save/restore, generation-save
  handoff and existing Streaming Drop tests. Legacy WorldRequest prepare/cancel,
  identity startup/authority and feature-off paths remain separately qualified.
  Appropriate touched-feature lint/fmt, public tree and honest broad/calendar
  limits apply; no parity/performance/UI responsiveness claim from headless tests.

Source review must trace EVERY new fallible acquisition and all constructors;
missing owned return edges block compile/runtime. Freeze source/tool manifests,
retain failed attempts, qualify exact clean candidate, then obtain DIFFERENT FINAL
and private checkpoint. Stop this component at accepted primitives; do not enable
ordinary paired start/stop/networking or mark G1. Continue the S1 plan afterward.

## Research appendix: recommended first C0b bounded proposal

This appendix is NOT accepted C0b source authority. Prepare/review a separate exact
proposal for ONE engine production path, crates/horizon-client/src/save.rs, with
focused tests inline in that path; no new scheduler/serialization or broad read model.
Use the refreshed combined B/D pin, preserve current SaveDir/native RegionFile code.

Current concrete seams: Record variants and lazy encoders around1770; PendingWrites
map around1905; Writer private send+JoinHandle2018; queue2189 increments outstanding
before spawning/sending; start_writer2231 returns infallibly via expect; failed
writes retain latest pending Records while completing batch counts; write_result
is sticky; SaveDir Drop2526 closes sender then unconditionally joins.

Proposed narrow services: begin_retained_writer_close, poll_retained_writer_close,
retry_retained_writer_close, and an allocation-bound non-Clone completed witness.
Keep normal default queue/flush/Drop semantics. Retained mode seal rejects new
external enqueues after producers have already stopped; all Arc aliases observe
same state. Poll never waits on native FS, condvar or mutex; contention Pending.
Close actual sender after its accepted batches have finished, retain handle,
poll is_finished then join/check once. Healthy no-writer path completes without
spawning one. Poison/accounting/panic/disconnected states are explicit owned
failures; no clearing an unknown error into successful closure.

Repair retry must use the ORIGINAL latest pending (Folder,x,z,seq,Record) objects,
including Chunk encoding context, POI claims logic, FinishedEntities and Delete.
Drain/harvest any actual outstanding batch/old handle before starting a retry;
no competing writers, count reset or pretending clone data is new domain custody.
Keep cumulative failure history; clear only a precisely correlated active attempt
error AFTER its retained records durably succeeded and actual handle joined.
Retry selection is deterministic by original sequence across ALL Folder kinds and
keys; it must not replay a superseded record over a newer accepted snapshot.
Current worker errors are sticky while later batches continue: correlate the
retained latest pending map with all accepted/completed batches before replay,
not the first failed batch alone. A repaired filesystem path is not sufficient
proof if a cached RegionFile handle still addresses an obsolete/error object;
exercise actual cache resolution and native disk reads with the original codec.

queue currently mutates pending/increments outstanding BEFORE writer lock/send.
A failed send must reconcile that exact accepted batch/count once without losing
its readable retained snapshots. A worker panic can strand outstanding counts;
that is terminal accounting uncertainty/Blocked, not permission to zero counts or
restart a competing writer. Prevalidate sequence/count/spawn/lock transitions
before publication where possible, and explicitly own every partial outcome.
Sequence exhaustion/counter increment/spawn failure must not partially lose records
or leave counts that claim nonexistent work. Preserve same native write ordering,
newest-record behavior and unreadable-old-POI refusal. No durability claim beyond
existing native writer contract without separately reviewed sync changes.

Required controls: actual native batch stalls, filesystem error then path repair,
partial region/entity/POI/delete success+failure, repeated-key newest writes,
no-writer, sender/worker spawn failure, panic/disconnection/poison/count uncertainty,
cloned Arc enqueue-after-seal and concurrent close/retry refusal. Successful retry
must preserve pending-read snapshots and prove native disk decode plus actual
finished/joined handle, not merely zero outstanding. Control release/reaping is
mandatory before fixture teardown; no writer Drop joins count as explicit closure.
C0b FIRST FINAL then refresh this C0a proposal/API and independently accept source.