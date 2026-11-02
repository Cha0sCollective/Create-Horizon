# Coherent native close: bounded C0d design

Status: PROPOSED only. No engine edit, source assignment, compiler/runtime,
ordinary activation, public push or stage acceptance is granted by this document.
Fresh exact design acceptance and a private assignment checkpoint are required.

## Accepted input and refreshed scope

P=485d4b1914c639d2a6cfc68f6fa7191ae5adbdaf,
treeec926d221833e8c1279f7063e5cb43ff02dd940d, soleb7cdb422.
Different C0c2 FINAL831b56f5 accepted this clean candidate and input11a5b1ec
(60 hashes),34normal/onefatal/sixchecks. Both metadata packets and the earlier
writer, producer/preparation, acquisition and Guest components are accepted at
their actual pins. Public source and private authored7858f0b checkpoint are synced.
Do not overlay an unaccepted component or relabel earlier execution.

This refines the C0d row in S1_PAIRED_CONFIGURATION_ADMISSION_PROPOSAL.md and its
producer/native-writer/metadata contracts. Fresh s1_close_api_review found the
original seven-path map insufficient. Proposed maximum FOURTEEN paths:

1. crates/horizon-client/src/game/lan.rs: pure actual owner/participant projection,
   owned native owner/player save context and checked CPU acknowledgment.
2. crates/horizon-client/src/game.rs: concrete native Level/owner projection and
   handoff entry, preserving ordinary save behavior; no ordinary close activation.
3. crates/horizon-client/src/game/paired_close.rs: private metadata-state extraction,
   fixed owned Game close cursor/job variants/status/retry/completion.
4. crates/horizon-client/src/game/levels.rs: fixed direct active/parked membership,
   native borrowed views and actual producer/writer witness custody, no level swap.
5. crates/horizon-client/src/game/levels/session_checkpoint.rs: owned immutable
   read/validation/image context and cursor using existing complete native format.
6. NEW crates/horizon-client/src/game/paired_close_tests.rs: genuine component tests,
   declared only inside paired_close.rs.
7. crates/horizon-client/src/game/levels/session_checkpoint_tests.rs: existing
   rich D/native image/recovery positives and relevant regressions.
8. crates/horizon-client/src/game/levels/session_checkpoint_items.rs: narrow
   concrete immutable resource-context decoder/validator factor, ordinary wrappers.
9. crates/horizon-client/src/game/persist.rs: native borrowed entity/chunk-extra/
   UUID preparation and fixed payload, preserving all existing codec behavior.
10. crates/horizon-client/src/game/world_loading.rs: reuse actual C0a retained
    constructor for dormant parked native ownership; no legacy-owner upgrade.
11. crates/horizon-client/src/game/raids.rs: borrowed per-level native raid adapter,
    aggregate markers and one original-player prepared omen context.
12. crates/horizon-client/src/game/leads.rs: native borrowed leash/holder UUID
    encoder factor called by persist; keep ordinary Game wrapper.
13. crates/horizon-client/src/game/paired_close_metadata_tests.rs: mechanical
    existing-test adaptation to state extraction and affected regressions only.
14. crates/horizon-client/src/save.rs: allocation-bound staged close writer
    durability/repair/final-submission capability; inline focused tests.

No streaming.rs, host_identity.rs, ordinary saving/control/network/UI loop,
new executor, portable image format or extra source path is granted. A further
required seam means HOLD and minimal reviewed amendment before source. This
fourteen-path design is bounded to native/coherent close components, not all46
paths or C0e/C1 activation. No upstream contact or public unreviewed work.

## Ownership and fixed jobs

The current NativeMetadata guard exclusively borrows Game across turns. Extract
its private prepared-state/handle/result mechanics without weakening existing
borrowed callers. Keep that public-in-Game borrowed API as a thin exclusive
wrapper. New private CloseOwner owns the ORIGINAL Box<Game> and private state,
never a self-referential guard beside its own Box. Only temporary actual Game
borrows occur inside prepare/preflight/commit. No mutable Game escape while
Closing/Blocked, address-cast identity, unsafe self-reference, copied authority
or arbitrary callback executor. Private context constructors derive from the
actual owned Game/resources/participants; equal claims cannot mint readiness.

Retain original native session/authority locks, Game/world/entities/items,
fixed level membership, SealedStream tokens, staged writer controllers, immutable
projections and every actual handle. One concrete CloseIo owner has fixed
NativeMetadata and CoherentImage job variants; the image variant has fixed
ReadPreflight and BuildVerifyPublish phases. It is not a general worker service.
One actual IO handle at a time, registered before dispatch, original input retained
outside the worker. Worker receives concrete owned immutable resource/package/
registry/advancement-tree/native payload contexts, never Game/UI/renderer pointers.

Owned native IO completion is distinct from whole-process/transport completion.
A native completion retains the original Game/locks for subsequent C0e composition
unless all remaining owners are actually proven terminal/absent. Do not issue a
global CloseCommit or permit Game/lock release merely because native IO finished.
For dormant positive fixtures, actual local owners with no transport/integrated
handles can establish terminal native disposal; populated participant snapshots
remain intact, not disconnected to get through this gate. C0e transport/control/
outer-loop ownership and ordinary activation remain separate dependencies.

Expected errors return or retain the same owner. Drop is only the unexpected
unresolved-owner abort backstop, before fields can dispose resources; no hidden
join/disposal thread/timeout detach. Finished-only join and private actual attempt
association. The absolute attempt deadline never renews on status/cancel/close;
unfinished Retry is Busy. Late success stays Blocked until explicit Retry validates
that exact returned result. Error/panic retains input, artifacts and monotonic
failure facts. No guessed loss of pending records or reconstruction from receipts.

## Exact phase ordering

1. Freeze ordinary Game mutation/scheduling and capture actual active/parked
   membership and original active dimension. Retain every original owner in place.
2. Reconcile/seal original producer work through accepted C0a; keep real original
   SealedStream witnesses. No enter_level/swap_level or provenance destruction.
3. Seal ordinary writer admission under the NEW staged controller; harvest the
   actual historical writer. On its native IO error, retain all original records
   and repair via checked existing retry machinery. Establish NONTERMINAL native
   durability, with no ability to reopen ordinary producer admission.
4. Run complete old-source read preflight on the owned IO worker, using the
   original native/D validators and actual immutable context. This precedes any
   checkpoint-specific final identity preparation or final native submissions.
5. Prepare permitted native identities/required fields once, preserving/reporting
   legitimate preparation effects through later errors. Freeze actual entity/
   anonymous UUID payloads and original expectations only AFTER those preparations.
   Prepare final Level/player/owner/native metadata projections without live marker
   acknowledgment. No simulation, dimension swap, RNG advance or filesystem calls.
6. The owned NativeMetadata job submits fixed native resident/entity/POI/forced/
   Level/player/metadata payloads through native codecs and the close-only writer
   capability. Earlier disk effects do not acknowledge original markers. Repair
   and retry use the SAME frozen payload, including anonymous UUIDs and stopwatch
   milliseconds. No regenerated identities or resampled clocks.
7. Establish final writer durability/join and terminal NativeWriterClosed for each
   original allocation. Retain matching SealedStream until that exact proof is
   checked. Coherent image work reads/copies/hashes/verifies only stable native
   output through the owned worker and existing complete validators/promotion.
8. Pure same-owner CPU revalidation covers all final projections, fields, private
   attempts, level membership and actual witnesses. Only then aggregate infallible
   CPU acknowledgment of native dirty markers, prepared fields and metadata.
   No worker-returned Level clone replaces live owners wholesale. Native completion
   retains Game/locks until the remaining real close owners are accounted for.

Producer/source boundary is intentionally identical to accepted checkpoint:
its old session_fence also reconciles finished/proto storage before source scanning.
C0a callbacks settle pre-Freeze native obligations; they are NOT new final snapshot
writes. We do not invent a stronger before-historical-callback disk-validation
promise. New final snapshots/identity changes occur only after old-source preflight.

The accepted C0a native encode/callback/enqueue actions remain indivisible,
including their existing in-memory mutex/get_or_init caveat. No retrospective
C0a rejection/replay, zero-wait or hard UI/OS latency claim. All NEW expensive FS,
final native submissions, durability waits, source reads, copies, hashes and complete
verification run on owned IO workers. Owner polling uses bounded CPU cursor work,
nonwaiting status operations and is_finished; never calls ordinary flush,
writes_finished, native_write_fence_until or a filesystem helper. Any new operation
that needs a waiting transition must be moved to the fixed worker or amended
explicitly; no blocking API hidden behind a poll name.

## Staged writer capability in save.rs

Existing flush returns sticky error. Existing begin_retained_writer_close seals
Open admission, and successful recovery becomes terminal Closed; ordinary queues
then reject all new submissions. It cannot provide a repairable preflight fence
followed by final snapshot submission. Do not claim it can or reopen it silently.

Add a concrete allocation-bound non-Clone staged controller: seal ordinary
admission; harvest/retry historical pending Record owners with existing codecs/
ledger accounting; issue checked NONTERMINAL durability for read preflight;
accept fixed final native records only through its close-specific capability;
then final durability/join and terminal NativeWriterClosed. Ordinary queue APIs
remain sealed the entire time. New final submissions invalidate intermediate
proofs before acceptance. No new SaveDir, public clear_error, empty-world restart,
guessed counters, reconstructed owner or replacement of lost pending records.

Partial final admission retains accepted native record evidence AND unaccepted
frozen inputs. Native IO error repair preserves all latest retained payloads and
old failure facts. Spawn/send/panic/poison/deadline are checked, never success.
Reuse existing staged batches, sequence receipts, reservation/ledger and exact
allocation checks; preserve existing terminal NativeWriterClose API behavior.
The staged capability is not a library-wide general transaction or journal.

## Native views, level data and raid omen

Direct fixed active/parked borrowing must serialize full native data through
existing persist/leash codecs, including passenger recursion, raw unknown/anonymous
memory and actual holder/player UUID context. Identity preparation is explicit;
freeze generated anonymous/vehicle identities in the retained payload. No codec
duplication, cleared entities, D-item-only NativeWorld shortcut, lazy loads,
level swaps or arbitrary Game callbacks in a worker.

Current parked constructors are legacy. Reuse the C0a retained constructor through
a narrow world_loading seam; no upgrade/relabel of an existing legacy Streaming/
SaveDir. NativeWorld binds actual fixed Dim/World/SaveDir plus retained construction/
producer/writer witnesses. Existing NativeCheckpointOrigin permits initial
Overworld only and cannot stand in for parked provenance. CoherentSession retains
its single authoritative initial Overworld/no-parked restriction.

Maps/fight/trader/arrivals and core metadata are shared original Game owners,
snapshotted once. Parked Raiding owns only its per-level raids/loaded/bar_hurts/
homes. Omen/patrol/siege remain active Game/player state because native swap_level
exchanges only those four fields. Factor borrowed per-level raid prepare/preflight/
ack inside raids.rs, retaining existing Game wrappers and native ordering.
Aggregate all selected level raid markers plus ONE matching prepared current-player
extra replacement. Never read parked unused omen defaults as player authority.

The exact prepared omen replacement must be applied to the OWNED Level projection
before native Level IO; otherwise deferred live ack would save stale omen data.
Combine native warden/player/owner projections through native packers, preserving
unknown extra fields and raw state. Preserve guard-dependent None/empty/non-Compound
semantics and ordinary ordering. Native Level::write can assign player UUID and
world_spawn; prepare required fields before final expectations or return/ack only
explicit validated fields. Never install a returned Level over original owners.

## Complete owned image context and publication

Refactor complete session_checkpoint/items validators into one concrete private
immutable context derived from actual Resources, pack/assets/registries, package
capture, advancement Arc<Tree>, frozen counter/slot/participants/items/resident/
clock/RNG/raid projection and original identity/path target. Existing ordinary
Game wrappers retain their restrictions/behavior. Move identity/package/pin reads,
effective raid disk fallback, native-local-player UUID lookup, columns/items,
full inventory/member/region/NBT/promotion verification onto the worker. No reduced
validator, skipped D item, guessed saved row, fake player or new image schema.

Old-source item/player UUID collision checks precede identity preparation and
new final writes, after the historical durability boundary above. Preserve all
accepted off-resident/raw-region/sidecar/player/native package checks. CPU snapshots
and actual package/resource same-instance checks stay separate from disk evidence.
Preserve registered/Guest variants to the extent their accepted existing wrappers
already support them; no new gameplay/activation/format compatibility promise.

Keep immutable native projections outside each worker attempt. Retain actual epoch,
pending/published paths and publication progress OUTSIDE the disposable return
before relevant filesystem actions. Set Uncertain BEFORE rename; a panic cannot
claim NotPublished/rollback. Poll immutable/atomic progress without waiting locks.
A new explicit image retry uses a fresh monotonically increasing native image
epoch/path, retains all previous artifacts/uncertainty, and reuses the SAME semantic
native projection. Metadata retry does not resample/reproject. A genuine late
fully verified image may be accepted only by explicit Retry plus pure same-owner
revalidation, never automatic deadline renewal. Do not overwrite an uncertain
published artifact or manufacture a successful receipt from file presence.

## Defining positives, reuse and source gates

Require a genuine counterless active+parked NativeWorld with distinct native
records, rich native entity/raw memory/leashes/raid routes/shared metadata, unchanged
membership/dimension across failure and repaired retry. Require a healthy initial
Overworld CoherentSession with actual counter/slot, original participants and rich
D items, complete image and existing recovery positive. No cleared/refused
metadata family, fake witness, manufactured absent configuration or empty-session
substitute. Full participant snapshots must survive; transport activation stays C0e.

Defining failures cover historical and final writer errors/repair, partial final
submission, wrong allocation/stale durability proof, worker spawn/send/panic,
stalled native/read/copy/verify/promotion phases, absolute deadline/Busy/late result,
original locks/items/payload retention, malformed old source BEFORE new writes,
publication uncertainty before/after rename, same-projection/fresh-image-epoch
retry, all-marker preflight refusal and terminal same-writer witness checks.
Actual no-transport native disposal and native-ready retained-owner composition
must be distinct; no global CloseCommit proof from native-only progress.

Use the adopted validation workflow: cheap features first, defining new cases and
changed dependencies, truthful actual-pin reuse of metadata/C0a/writer/Guest.
No automatic full earlier batteries. Counts over30 require a concrete risk
explanation, not arbitrary omission or cap. Fixed native controls, genuine pinned
resources and fresh isolated Game processes; no nested compiler/Java/test process.
Release controls/reap before assertions. Root alone ALL execution/Git serial;
authors scoped native TEXT only. No standing team or permission expansion.

Fresh independent exact design acceptance at P, bounded private source assignment
checkpoint, ALL14/prefix baseline preservation and explicit Root release precede
source. Preserve all scoped sources/artifacts before every correction. Fresh
SolHigh SOURCE/tool before compiler/runtime; clean candidate, DIFFERENT FINAL and
private checkpoint before composition/public push. No ordinary C0e/C1 callers,
configuration admission, registered lifecycle, authorization, replication,
committed animation, connected G1 or live S2 is established. No PR/merge/release.
