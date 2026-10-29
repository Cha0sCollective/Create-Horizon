# H08 fresh native capture mechanism cell

DESIGN ACCEPTED FOR BOUNDED IMPLEMENTATION after fresh independent Sol High review
of proposal SHA256 6d3ed73ee45409218474953101b6b97104a319c0552455db0af594d98e934af3.
Source/runtime/evidence acceptance remains pending. The initial proposal review
withheld release because its save-footprint list omitted native random_sequences;
the bounded amendment above resolves that finding and the noted clarifications.
Exact
clean engine base afb38260b562c2df1b12e36aec304ccf51eb0e31, tree
3131fc796880a1a04d94c4a8de7a5af6c094ad2c (fully accepted W1). Coordinator owns
all execution, review, qualification and commits. This packet uses Horizon source
only. No runtime, imports, WSL, builds, tests, formatting, network or engine edits
were performed to prepare it. Preserve the coordinator's private result/status
work. The motivating audit is H08_CAPTURE_INTEGRATION_AUDIT.md, SHA256
767b9c56e3fca74a8f9c97974a882616e54e75a7bf3d794a4302b91bdb332783.

## Result and deliberate research boundary

Implement and EXECUTE one actual native Game capture -> physical immutable epoch
-> manifest-last publication -> recovery into a fresh working tree. Its continuous
authoritative owner operation combines existing PLAYER/E1/W1, native saves and
writers, concrete canonical PLAYER output and an item-free counter archive. It
must prove real positive player inventory and native world readback together,
interrupted publication and definite task completion. This is one mechanism cell,
not another detached observer or a general checkpoint framework.

All Game capture/exporter code and enabling tests are gated
cfg(all(test, feature = "paired-packages", target_os = "linux")). The library's
small defining Streaming accessor is PUBLIC under paired-packages because lib
cfg(test) code is unavailable to binary tests. Its ordinary body/callers stay
unchanged. Linux-only fsync/MetadataExt/process-kill code stays inside Linux gates;
other native targets must compile. No normal Game/SDK/CLI/control enabling caller.

Fresh provenance is the reviewed NEW test setup itself, not a user fresh=true
argument, new persistent field, runtime permission token or inferred lack of
history. Create Game through unchanged Fixture on the OWNER test thread; THEN
replace its original active streamer during SETUP through existing
world_loading::make_streaming at the exact fixture world_root, with
options.test_terrain=true for Overworld and the returned matching SaveDir/locator.
The original Game::new streamer is paused bare TestTerrain, NOT bound Persistent
storage; Fixture.game alone is not that coverage. No outstanding work exists when
replacing this pristine setup owner. Native
host/Guest constructors and as_guest establish two real channel-backed players.
Keep the synthetic local body OUTSIDE Game and hold all output receivers. Setup
opens real parked dimensions, installs finite authored blocks, positive plain
items, one native counter and a tiny explicit advancement tree. It never runs a
world tick, network poll, streaming update, pregen, callback, function, unload,
portal or Left/reconnect. No test-only changes to existing Fixture/helper bodies.
An intentional negative test may violate one premise and must refuse capture.

This does not establish runtime admission, item-bearing machine persistence,
live counter/generation installation, history preservation, ordinary in-frame
capture, external paired networking, power-loss testing or S1/G1. The dedicated
two-client/one-slot/cross-owner/reconnect requirements remain unchanged. A later
runtime enabling packet must establish actual lifetime/admission and producer
closure beyond this fixed fresh setup; these restrictions are not the permanent
S1 product. No journaling or conservation reduction is requested.

## Seven source paths; no silent expansion

All engine paths below are under crates/horizon-client/src:

1. game/levels.rs: Linux+test+paired pub(in crate::game) module declaration only,
   permitting the sibling integrated test child to reach the new capture types.
2. NEW game/levels/capture_integration.rs: actual operation, bounded detached
   records, physical copy, publication and recovery functions; parent-module child
   visibility permits borrowing actual Parked fields without changing privacy.
3. game/lan.rs: Linux+test+paired consuming exporter and its detached row type;
   all existing projection/preflight/writer bodies and fields stay unchanged.
4. streaming.rs: paired public pristine-streamer accessor/types, defining-module
   tests; no fields, scheduler/cancellation, existing method/body changes.
5. NEW game/levels/capture_integration_tests.rs: child of capture_integration via
   explicit path attribute, so tests can exercise private real publisher phases.
6. game/integrated.rs: Linux+test+paired pub(super) test-module declaration only.
7. NEW game/integrated/capture_completion_tests.rs: defining-module pub(in crate::game) dispatch helper for the POSITIVE capture cases and actual Task/Serving/run_tasks
   completion tests with Game remaining on its test owner thread.

No proposed extra file is needed initially. Keep the publisher specialized to
this finite fixture. If a mechanical source child split is necessary for readable
implementation, submit its exact path/reason before release; no framework, new
SDK, dependency, field, normal caller, native schema or generic error alias change.
Existing sha2/serde/serde_json/flate2/native NBT suffice; verify their current API
rather than adding crates or enabling experimental platform features.

## Proposed signatures and cross-thread types

The following are NEW proposed test interfaces, not existing APIs:

    // game/lan.rs; derives Debug, no live references/owners
    pub(in crate::game) struct CapturePlayerRow {
        conn: u32, profile_uuid: u128, record_uuid: [i32;4],
        integrated_owner: bool, dimension: String,
        player: Tag, advancements: serde_json::Value, stats: serde_json::Value,
    }
    impl ParticipantProjections {
        pub(in crate::game) fn into_capture_rows(self) -> Vec<CapturePlayerRow>;
    }

Row fields are pub(in crate::game) only under the test/Linux gate. The exporter
MOVES existing values, matching private ParticipantStorage in lan.rs; it does not
rebuild identity or format a second player projection. No change to the existing
private structs/fields is needed. Existing project_checkpoint_participants still
runs actual same-call preflight FIRST and validates all live roots/counts before
encoding. Capture freshly observes the roster and calls that method itself.

    // game/levels/capture_integration.rs; test+Linux module
    pub(in crate::game) struct CaptureRequest { archive_root: PathBuf, epoch: u64, identity: Identity }
    pub(in crate::game) struct CaptureReceipt { epoch: u64, manifest_sha256: [u8;32], members: u32,
                            image_bytes: u64 }
    enum Publication { NotPublished, Uncertain }
    pub(in crate::game) struct CaptureError { phase: Phase, publication: Publication, code: Failure,
                          relative_path: Option<String>, detail: String }
    pub(in crate::game) type CaptureCompletion = std::result::Result<CaptureReceipt, CaptureError>;
    impl Game<'_> {
        pub(in crate::game) fn capture_fixture_epoch(
            &mut self, request: CaptureRequest) -> CaptureCompletion;
    }
    fn recover_fixture_epoch(archive_root: &Path, identity: &Identity,
        destination: &Path) -> std::result::Result<RecoveryReceipt, CaptureError>;

Types are pub(in crate::game) only when needed by the sibling completion tests.
Identity is owned strings/digests, never an Arc<Game>, native error or closure.
Phase is one of Dispatch, Validate, Project, Save, Copy, Overlay, SyncMembers,
Manifest, Promote, SyncArchive, Recover. Failure is one of CompletionLost,
Context, Player, Entity, World, Streamer, Limit, Path, Identity, NativeSave, Io,
InvalidImage. A narrow pub(in crate::game) CaptureError::completion_lost(detail)
constructor produces Dispatch/CompletionLost/Uncertain for the sibling dispatch
helper; expose other constructors or query methods only where actually needed.
No existing type's privacy changes. Error detail is a
clipped diagnostic (<=4096 UTF-8 bytes at a character boundary); correctness uses
typed code/phase and filesystem verification, never message matching alone.

Native horizon_client::Result aliases Box<dyn Error> WITHOUT Send. It must NEVER
cross the completion channel or become Integrated::call's result. Convert native
errors to this owned error on the owner thread, preserving phase/path/detail.
CaptureReceipt/Error contain only owned scalars/strings/paths/enums/digests and
are Send. Add compile-time assert_send checks AND actual channel dispatch. The
inner existing Integrated::call returns T=(), not a native Result.

## Actual Task completion, without moving Game

Generic Integrated::call remains byte-for-byte unchanged: it sends a Task, waits
ten seconds and returns None on timeout without cancelling work. In the NEW
integrated test child, keep Fixture/Game on the MAIN TEST OWNER thread. Construct
Serving with native private fields in its defining child, and a client-side
Integrated handle with thread=None, matching flags/tasks and owner UUID. That
handle/channels alone move to a client thread; no Game, borrowed Fixture resource,
renderer or Box<dyn Error> is required to be Send.

Client creates a separate sync_channel<CaptureCompletion>(1), then submits:

    integrated.call(move |g| {
        let result = g.capture_fixture_epoch(request);
        let _ = completed.send(result);
    }); // Option<()> is ONLY an observer result
    // Always settle the independent receiver; None is not cancellation.

The new integrated child exposes a narrowly fixed pub(in crate::game)
dispatch_capture(game: &mut Game, request: CaptureRequest) -> CaptureCompletion
TEST helper. It constructs native Serving/Integrated and completion channels and
drives actual run_tasks on that same owner thread; the levels test child can call
it because the new module declaration is pub(super). Its client alone is spawned.
Keep request/completion/flags owned and Send; do not pass Game into spawn. EVERY
minimum positive capture goes through this helper: its actual capture operation
executes INSIDE the real submitted Task, not directly before/after a Task stub.
The owner invokes ACTUAL Game::run_tasks, not a direct substitute closure. Its
native Task snapshot/loop serializes the full operation before apply_publish,
a later task or tick. Completion receiver waits for the final result even if the
10-second observer returned None; no retry, new epoch or release is permitted
based on that timeout. An outer test watchdog is a failed/UNRESOLVED test, not a
capture cancellation guarantee. No hard OS-I/O latency promise is made.

For the timeout case ONLY, the test's submitted Task wrapper sends an admitted
signal and waits on a test-owned channel BEFORE invoking the real capture. Client
observes actual call timeout, queues a later native Task, verifies it has not run,
then releases its wrapper and waits for the completion. Owner run_tasks remains
blocked throughout; a subsequent native batch runs the later Task. Capture itself
has NO phase callback, delay, fault injection or test-mode conditional. Test
channel guards release on assertion failure so the owner cannot be stranded.
No process-global clock/command setting or sleep-based production behavior.

If the completion sender is dropped (queue dropped/panic/thread failure), classify
caller state CompletionLost/Uncertain for that requested epoch: publication may
already have occurred. Do NOT infer NotPublished or automatically retry. Inspect
verified recovery state. A final receipt is a settled task outcome; losing the
receipt is not evidence that the task never ran.

## Exact fresh Streaming closure

Proposed paired public library signature:

    enum CapturePristineError { PreviouslySubmitted, QueuePoisoned, Stopped,
                                InconsistentState }
    Streaming::require_pristine_fixture_capture(&self)
        -> Result<(), CapturePristineError>;

This operation-coupled check is not a global lease or arbitrary-generator claim.
Check order != 0 first (PreviouslySubmitted). Then borrow shared.queue with a
checked lock, require shared.stop=false, queue empty; generating/finishing empty,
decorating=saving=0; protos/written/failed_finished/deferred empty; no generation
save error; lighting/meshing/light_snapshots empty. No recv/try_recv/pump/take/clear,
owner clone or native state repair. Defensive inconsistent/poison/stopped branches
are source-supported unless a genuine native path reaches them; do not fake
successful closure by resetting counters or swapping results channels.

Why the unobservable result channel is closed for THIS premise:

- Streaming::new sets order=0 and creates workers waiting on shared.queue; the
  sole production push is submit, which increments order BEFORE pushing any Job.
  No production reset/decrement exists. requeue only reranks queued values.
- Every Generate/Decorate/Finish/Light/Mesh/Save goes through submit. Worker code
  sends Done only after taking a Job. Without a native submission, there can be
  no in-flight job, deferred completion or hidden results-channel message.
- Source follows Generate's generating set through Done::Proto/Generated,
  Finish's finishing set, Decorate's decorating count and Save's saving count.
  save_until/poll_save reconcile tracked/deferred work but omit some generating
  and unrelated results. busy also omits saving/written/failed owners. We use
  neither method nor their counters alone to certify a historical streamer's
  channel. Light/Mesh tracking can be cleared while stale work still exists;
  order!=0 conservatively refuses that entire history instead.
- order is u64. submit uses +=1, so release overflow could eventually wrap to0;
  debug can panic. This is NOT a durable never-submitted bit. Actual fresh setup
  executes ZERO submissions, not 2^64, and never mutates order/queue/results.
  No arbitrary live Game is admitted on order==0. No overflow-based history or
  custom ChunkGenerator guarantee is asserted.
- Explicit replacement of the original Fixture active bare streamer with make_streaming at the fixture root is required during setup. The returned active SaveDir must be the SAME Arc held by its Persistent provider; native parked entry creates matching bindings. Fixture/server_game and make_streaming constructors do not update
  streaming; enter_level/apply_settings only establishes settings/owners. The
  inspected native Persistent Flat/Test/Vanilla providers/worker constructors do
  not independently enqueue saves. No fixture exposes Saves to an outside writer.
  Existing search-ring work is in-memory/read-only for this file-write argument.
- With empty written/protos and no submission, native Streaming::save(true)'s
  reconciliation/wait/proto-save work is vacuous. Owner captures perform no
  update/pregen/function/network/mod pump that could submit later. Recheck each
  actual active and Parked streamer immediately before AND after native saving.
  If native behavior changes, refuse/stop; do not expand this proof implicitly.

SaveDir is a SEPARATE writer: native saving may queue region/entity/POI snapshots.
Flush every actual participating SaveDir after native saves. Its error is sticky;
no future external alias producer exists in the authored setup, and zero worker
submissions prevents asynchronous future storage enqueue. A successful writer
flush alone does not replace the Streaming/provenance argument. No worker drop,
join, cancellation, channel drain or new producer service is introduced.

## Supported cell and finite limits

Limits are constant packet bounds, not user-tunable quotas for a general API:

| Domain | Exact bound |
| --- | --- |
| Players/dimensions/chunks | Exactly 2 players; 1..3 native dimensions; exactly 1 resident column (0,0) per loaded dimension; no unloaded owner history |
| Counter | Exactly 1 native item-free active counter; no fault/pending/last retirement, resident record; archival only |
| Items | Inventory36 + ender27 per player; armor/offhand empty; known root stone/dirt/cobblestone/gravel only, count1..64, damage0, extra=None; no nested components or transient selections |
| Source/image traversal | <=256 entries including directories; <=12 path components; <=240 ASCII bytes per relative path; file<=32MiB; aggregate regular-file bytes<=64MiB |
| Projected NBT | <=1MiB uncompressed, <=16384 nodes, depth<=32, string<=4096 UTF16 units; gzip output<=2MiB |
| JSON row/counter | <=1MiB each, <=16384 nodes, depth<=32, string<=4096 UTF8 bytes; no float values in archive descriptors/counter/manifest |
| Manifest/discovery | manifest<=256KiB;<=256 files+directories;<=16 published epoch directories; epoch1..u64::MAX; checked arithmetic throughout |

Plain items are a new EXPERIMENT admission limit, not a change to PLAYER's existing
formatter/count helper. Refuse live unsupported item roots/extra/damage BEFORE
project_checkpoint_participants, then call its own preflight FIRST as usual.
Require actual Guest/Body/Level values produced by the fixed authored finite setup,
not imported arbitrary player files. Do not claim projected Tag validation can
undo a prior lossy scalar cast. The fixed body values use representable positions,
finite rotations/health/food/xp/fire/air and native defaults; tests independently
author expected full player Tags/JSON, including inherited owner-only fields.

Before NBT serialization walk the detached Tag within those node/depth/string
bounds: valid UTF16 only; finite Float/Double and no negative zero; integer scalars
retain their native widths; no End payload except permitted native empty framing.
Use native write_named with explicit Limits, require strict bounded read_named
consumption/equality for the supported output, then native gzip encode. No native
codec is changed. JSON serialization uses a writer that stops BEFORE exceeding
its byte budget; readers check metadata length, use bounded read (cap+1), then
bounded decoding. Do not fs::read/read_to_end unbounded then check length. Use a
local recursive serde DeserializeSeed/Visitor pass enforcing depth/node/string
and duplicate-key bounds before constructing retained collections, then convert
the bounded value to the exact typed schema. Require complete input consumption.
Keep serde_json's existing recursion protection; no configurable depth-32 setter
or extra feature is assumed. Parser scratch is bounded by the prior input-byte
cap. Do not deserialize unrestricted Value and validate afterward.
Native Gzip.decode accepts max_output; use the explicit 1MiB limit on projected
player verification, not tolerant read_dat's gzip-failure/raw fallback.

Recovery verifies all manifest lengths/digests FIRST with fixed-buffer streaming.
Native Level/SaveDir/playerdata readback in tests follows this finite image check;
projected player NBT is also strictly decoded before native apply. Native region
records use bounded compressed file sizes and explicit NBT Limits when checked
through RegionFile/ChunkRecord::nbt. This is not arbitrary MCA/opaque-byte support;
only the fresh native generated cell is accepted for semantic reload. File/hash
verification and native reload must never be advertised as a hostile-image parser.

## Canonical row mapping and immutable schemas

Set native level.player_uuid BEFORE projection to [-1,-2,3,4]. Use logical owner
profile UUID11 and external UUID12; actual host.owner is owner11/saved=None.
Serving.owner must match11 or existing PLAYER correctly refuses OwnerMismatch.
Native players/data owner record uses ffffffff-ffff-fffe-0000-000300000004; external
uses 00000000-0000-0000-0000-00000000000c. Native owner profile alias ...000b is NOT
another file to invent. Use existing canonical formatters in code, and authored
literal target/UUID assertions in tests. Require no conflicting alias file in the
fresh source; do not remove an arbitrary stale alias to repair admission.

Epoch layout:

    archive/.pending-<20-digit epoch>/world/<all copied native relative paths>
    archive/.pending-<20-digit epoch>/research/counter.json
    archive/.pending-<20-digit epoch>/manifest.json
    archive/epoch-<20-digit epoch>/...       # only eligible published layout

Copy all regular files/empty directories from native world_root except EXACT
root session.lock. Native paths include dimensions/minecraft/{overworld,the_nether,
the_end}/{region,entities,poi}/r.0.0.mca and saved-data paths. Never use DIM-1/DIM1.
Overlays are exactly world/players/{data,advancements,stats}/<record UUID>.{dat,json}.
Level::write already writes players/data/<level UUID>.dat separate from level.dat;
replace that SAME current owner row with detached projection, not another alias.
Overlay only those six mapped files; preserve copied native _old backups as
backups, not separate current inventories. Generate gzip NBT/pretty native JSON
from the detached rows with bounded byte writers; no live Guest/Level defaulting.

Counter JSON (all fields required, reject unknown/duplicate fields on read):

    {"schema":1,"kind":"item-free-counter-archive","scope":{"world_instance":u64,
     "dimension":string},"type":string,"save_schema":u32,"position":[i32;3],
     "state":{"name":string,"properties":sorted-string-map},
     "runtime_instance":u64,"counter":u64,"last_advanced":i64-or-null,
     "resident":true,"live_generation_restore":false}

Read actual native Snapshot accessors; do not persist a numeric block-state ID or
Arc address. Runtime instance is an archival identity only. Current counter/slot
APIs have no persistent installer; recovery returns this detached record. It does
not re-enable a new counter and pretend that new instance restored a generation.

Manifest JSON (typed structs, deny unknown fields; duplicate keys/paths refuse):

    {"schema":1,"kind":"fresh-native-capture-cell","epoch":u64,
     "identity":{"engine_source":sha256hex,"test_binary":sha256hex,
                 "resource_manifest":sha256hex,"counter_content":sha256hex},
     "players":[{"profile_uuid":uuid-text,"record_uuid":uuid-text,
                 "integrated_owner":bool,"dimension":string}],
     "dimensions":[native-id...],"counter":"research/counter.json",
     "directories":[normalized-path...],
     "files":[{"path":normalized-path,"bytes":u64,"sha256":sha256hex}...]}

Players ordered by canonical record UUID; dimensions fixed OW/Nether/End order;
files/directories strictly sorted UTF8/ASCII with no duplicates, no manifest member
listed as its own file, no directory/file type conflict. Every actual image entry
must match one manifest entry (apart from manifest.json itself). Manifest length
is bounded independently. Identity comes from the trusted isolated harness's
pinned source/binary/resource/content identities; recovery requires exact expected
Identity and filename epoch equal manifest epoch, not merely syntactic digests.
This is integrity/version matching, not cryptographic authentication.

## Operation, publication point and failure footprints

Preflight canonicalize trusted owned archive parent/world roots; archive is outside
and nonoverlapping world_root. Reject any symlink ancestor/entry, absolute/traversal
path, empty/dot/dotdot component, backslash/NUL/colon or nonregular source entry.
Check each parent with symlink_metadata before opening, and recheck opened regular
file metadata/size. The isolated owned-parent premise excludes adversarial parent
replacement/TOCTOU; do not claim race-proof hostile-path handling from std paths.
Use create_new for new epoch/manifest files and require new destination roots.
No deletion of an existing epoch, hardlink, latest pointer or live overwrite.

Pipeline: Validate -> Project -> Save -> Copy -> Overlay -> SyncMembers ->
Manifest -> Promote -> SyncArchive -> Receipt. Project after coupled PLAYER/E1/W1/
streamer checks. Native save_game_with(true) is allowed to change its usual dirty/
metadata bookkeeping, but may not retire/remove an owner. Qualified parked save
restores active dimension. Flush actual participating writers and recheck current
owner checks/streamers; no owner pump interleaves. No dropped entities means native
save cannot introduce the fresh-drop UUID-memory/E1 conflict. Projected PLAYER rows
and counter scalars are detached before any archive write; source files become the
frozen world image only after successful native save+flush and producer exclusion.

Source footprint: early admission/projection refusal writes NOTHING. Native save
failure may leave native partial writes. For this fresh/default cell, allowed
exact source files are each loaded dimension's region/r.0.0.mca, optional
poi/r.0.0.mca, entities/r.0.0.mca only if an actual native producer reaches it;
root level.dat; data/minecraft/{weather,world_clocks,world_gen_settings,game_rules,
random_sequences}.dat;
the mapped OWNER data/advancements/stats files; and their exact native _new/_old
companions when reached. The exact random_sequences.dat file is demonstrated by
game.rs::save_game_metadata -> trading.rs::save_random_sequences_to, which writes
even empty/default authoritative state. Independently author its default-state
NBT readback and assert this exact file/companion footprint, preserving native
save bodies. Empty forced/raid/fight/trader/map/schedule/bossbar defaults
must not gain broad allowance. Tests seed one existing chunk_tickets.dat only for
its native obstruction case; permit only that exact dimension's record/companions.
Create only exact ancestor directories for those files. Expand no whitelist until
an actual producer is demonstrated; unknown creation/deletion/sentinel/type change
fails before/after full disk-map comparison. Active external rows are NOT written
by the ordinary common native save; their six-row inclusion is via archive overlays.

Copy/overlay/sync failure can leave PARTIAL staging bytes under this new pending
epoch, but cannot change source after Save, a previous epoch or recovery target.
After Copy, freeze a source disk snapshot and compare it through all remaining
phases. No cleanup deletion is required on error; pending is ignored by discovery.
Overlay is restricted to its six exact files plus research/counter.json. All
remaining copied entries/bytes/type stay equal. Manifest/temp paths are the only
new files afterward. Every file is a physical new copy; test Linux dev/inode
identity differs from source and editing source later cannot change old epochs.

Linux durability order: finish all member writes, sync_all each member file;
sync_all each image directory bottom-up. Serialize bounded manifest to
manifest.json.tmp with create_new, sync_all it, rename to manifest.json, sync_all
its pending parent. THEN attempt rename .pending-N -> epoch-N under SAME archive
parent (target must not exist in this owned environment). Sync_all archive root
before emitting success. No latest pointer is updated. File/directory fsync or
rename errors propagate; unsupported directory sync is not ignored.

Set Publication::Uncertain BEFORE attempting final directory rename. Any failure
from that attempt through archive-root fsync or delivery of acknowledgement may
have a published valid epoch. Err NEVER means 'nothing published'. Before this
attempt an error is NotPublished for THIS new epoch (old epochs remain). A crashed
caller or lost receipt is Uncertain regardless of last observed stage. Recovery
inspects the filesystem rather than blindly retrying a possibly committed epoch.
Process-kill tests show restart behavior at these boundaries. The fsync ordering
is the Linux power-loss DURABILITY ARGUMENT; process kill is NOT simulated power
loss, and no hardware/filesystem guarantee beyond that argument is claimed.

## Recovery and exact native evidence

Enumerate <=16 published epoch-N names, rejecting malformed/duplicate numeric
IDs and traversal/symlink parents. Ignore correctly named .pending-N directories
as incomplete attempts; do not merge their files. For each published candidate,
bound manifest file length BEFORE read/decode, validate strict schema/order/count/
paths/identity/epoch, then verify exact entry set, type, lengths and SHA256 through
bounded reads. A structurally invalid/corrupt candidate is INELIGIBLE. Pick the
highest fully verified matching complete epoch, with an explicit diagnostic list
of rejected candidates; if none, refuse. A published epoch with wrong expected
build/content identity is a compatibility refusal, not an invitation to silently
fall back to another build. Tests distinguish corruption fallback from identity
refusal. Never take some files from a rejected newer candidate and others from old.

Recover into a destination that does NOT exist: create only there, physically
copy the selected epoch's world/ files, reverify byte hashes, sync files/dirs,
then acquire a NEW native SessionLock for that destination. Keep its lifetime in
RecoveryReceipt on the owner thread, or scope it locally; DO NOT send SessionLock
in CaptureReceipt. Read native Level/SaveDir and mapped player records there;
owner mapping uses canonical record UUID, not playerdata::load(profile11) on the
wrong path. Counter readback is the separate archive record, not a file silently
installed in the native world. Recovery failure may leave partial NEW destination
but cannot alter archive or original world; never overlay/delete an existing live
world. Native file readers follow bounded verification; no writable open on failed
verification. No record/history recovery is claimed beyond this fixture epoch.

Private publisher phases may be ordinary methods on one local pending-image
value (copy, overlay, sync, write_manifest, promote). They are also the steps the
real capture executes. New child tests can stop after one real phase, leave the
partial native tree and rerun recovery. A Linux native child process may notify
after a phase and wait on its TEST harness pipe, allowing parent fail-stop. The
full capture method receives no callback/fault-stage parameter; no production
phase hook, extra CLI or scheduler mechanism is introduced.

## Minimum tests, honest reachability and qualification

NEW ignored PRIMARY actual-resource tests (final exact names frozen before
execution; positive capture always uses the defining integrated dispatch_capture
helper and real native owner11 plus external12):

- capture_cell_roundtrips_two_players_native_world_and_archival_counter: real
  owner/external identities and positive plain inventory;1- and3-dimension cases,
  exact native paths/independently authored Tag/JSON/counter/block expectations;
  change live values/files afterward and prove epoch byte/inode independence.
- capture_cell_refuses_current_owners_and_nonpristine_streamers_without_publication:
  remove external12 and require fixed two-player admission refusal (or retain its
  native play entry and assert actual OrphanPlay), unsupported positive stack
  extra/count/damage, E1 drop/slot,
  W1 BE/spill, actual native streamer submission; exact untouched archive/source
  for admission refusals. Fresh observation followed immediately by projection
  cannot itself produce RosterMismatch; introduce no stale expected-roster
  parameter or manufactured interleaving. No fake counter/channel reset or
  readiness report reuse.
- capture_cell_native_save_and_staging_io_faults_keep_previous_epoch: real native
  region-writer obstruction and ticket _new obstruction, same writer/sticky error/
  native owners; real copy/open/overlay/rename destination obstruction where legal;
  exact source/staging/old-epoch disk footprints. Err may retain partial bytes.
- capture_cell_manifest_validation_and_whole_epoch_recovery: length/depth/byte,
  path/symlink/duplicate/type/digest/epoch/build/content corruption, extra/missing
  members, malformed published names, bounded discovery and explicit fallback;
  fresh destination only; no native writable open before verification.
- capture_cell_real_phase_restart_preserves_complete_epoch_selection: real private
  phase transitions plus isolated child fail-stop after copy/member sync/manifest
  temp/manifest final/promotion; recovery old OR complete new according to publish
  point, never mixed. Keep kill and durability claims separate.
- capture_task_observer_timeout_keeps_owner_exclusive_until_definite_completion:
  actual run_tasks plus main-thread Game/client-thread handle; actual 10-second
  call timeout, no cancellation/retry, later Task excluded, one Send completion.
  Queue/receipt loss is Uncertain; no manufactured committed receipt.

Declare one separate ignored AUXILIARY test entrypoint capture_cell_phase_child.
It requires a bounded parent-authored plan and test pipe; absence/malformed plan
FAILS explicitly, never a passing no-op. Root enumerates primary plus auxiliary
names but executes only primary cases; a primary may launch the exact auxiliary
name in its own fresh child. Each child gets fresh cwd/HOME/game/launcher/data/
config/cache/temp, clears HORIZON_SESSION_FILE and preserves display-runtime
policy. No no-env auxiliary run counts as evidence. The parent kills only that
owned Linux child after its phase handshake and retains native output/scratch
for recovery assertions; test helpers stay cfg Linux. Process kill proves restart
selection, not empirical power-loss durability.

Focused inline Streaming tests construct real native jobs/completions using its
existing defining-module seams and verify PreviouslySubmitted refuses queued,
in-flight AND already-returned work without draining/normalizing it. Actual pristine
constructor succeeds repeatedly without mutations. Poisoned/Stopped/Inconsistent
branches may remain source-only; no public helper to corrupt private state. Real
file/copy/open/rename failures are executable. Directory fsync failure after a
successful rename and power-loss behavior are source-backed error handling unless
a genuine supported filesystem fault is available; do NOT inject synthetic sync
success/failure and call it a native durability test. No assertion that Uncertain
must mean absence or presence of the new epoch.

Freeze source/suite/manifest hashes, compile no-run then fresh isolated serial
resource/native tests, relevant three feature lints and broad checks. Existing
qualified helpers/bodies must remain byte-for-byte unchanged. Ordinary b414
integrated/in-frame Game evidence MAY be reused explicitly only if frozen-diff
review confirms strictly gated additions, unchanged existing production bodies
and no runtime enabling caller; do not claim a new GUI binary ran. Any normal-path
change or new concern requires fresh Game validation. New resource Game/completion,
Streaming native, feature and broad evidence is always fresh. Coordinator obtains
a different fresh independent source and final evidence review before acceptance.
No public push, merge, stage promotion or runtime capture enabling follows.
