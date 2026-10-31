# Retained native metadata families: C0c1 and later C0c2

Status: PROPOSED DESIGN ONLY. This document grants no engine source authority,
runtime permission, ordinary caller activation, publication or S1/G1 acceptance.
Independent review must accept a bounded packet before assignment. The larger
46-path C0/C1 map is a dependency map, never an implementation assignment.

## Inputs and source-start gates

This refines the metadata-family split in
S1_PAIRED_CONFIGURATION_ADMISSION_PROPOSAL.md, SHA256
95484fa4861f26d8f66c6756bccaa7cbe2bc73c70a98aa5507315dec4d3c3550,
and respects S1_PREPARE_PRODUCER_CLOSE_PROPOSAL.md, SHA256
3ef2bed2eec24a3221238248b0e1bc23f5b1fb35ca14af5edc173ce35a2d1e8f.

Read-only research inspected clean combined engine C
1fa55c619a2f4666a505461822a97efa5732db01, tree
d0b2c224047fac6d98bdfc729b8367918511bd76, sole parent D
6a5a16e2b5b65e952e10f2b106fcc10d7838a3da. Separate writer research inspected
save.rs raw SHA256 e4c2ecec1305252449435be62cfeeef8c44d9b2a6fbc49c35d47eaf1c2b2d21b
in the native-writer worktree based on ab239890. That observation was not writer
FINAL, writer adoption, combined-source qualification or C0a completion. Locations
below are research anchors, not the eventual source-start pin.

Before C0c1 source assignment require ALL of:

1. The retained writer component's independent FINAL and separately accepted
   writer-adoption FINAL on the actual combined predecessor, including fresh D/B
   regressions. A writer-only result on ab239890 does not satisfy adoption.
2. C0a retained preparation/producer-close FINAL at its actual accepted pin,
   including the adopted writer APIs and genuine retained partial/ready outcomes.
3. A clean exact accepted predecessor P containing the required components, with
   actual commit/tree/lineage, source inventories and private durable checkpoint.
   Do not overlay concurrently qualifying work or infer a composed acceptance.
4. Fresh read-only actual API/caller/field/module review at P, independent Sol High
   acceptance of this exact bounded proposal and its refreshed path budget, then
   a private durable checkpoint and explicit Root assignment. Refresh any changed
   anchor and explain the delta against each accepted predecessor.

If a prerequisite changes, HOLD and preserve the scoped baselines/drafts and all
packet-prefix artifacts before correction/rebase/replacement. An extra path or
different ownership contract needs revised bounded design and fresh review. No
source-start gate is satisfied by research or approval of this document alone.

## C0c1 outcome and exact eleven-path maximum

C0c1 produces dormant native metadata snapshot/IO/acknowledgment primitives and a
real retained one-job IO owner. Its success says only that the selected family
batch completed through existing native codecs and matching acknowledgments.
It is not CloseCommit, a saved world/image, a closed Game, authority to release a
world lock or admission/Running readiness. Normal save/close/network callers stay
unchanged until their separately accepted C0d/C0e assignments.

Paths are under crates/horizon-client/src:

1. game.rs: MODULE DECLARATION ONLY for the dormant paired_close module. No Game
   field, constructor, startup guard, event loop, save call or teardown change.
2. NEW game/paired_close.rs: fixed concrete metadata batch/attempt owner, typed
   results, finished-only harvest, retained failure/retry and checked family ack.
3. game/trading.rs: actual world/merchant random-sequence snapshot and native IO
   split, preserving loan/conflict semantics and ordinary save behavior.
4. game/stats.rs: native stats JSON preparation/write input, actual local guards
   and unchanged notification/event state.
5. game/advancements.rs: native progress/tree preparation/write input and retained
   ownership/finished-only harvest of the actual pending tree helper.
6. game/scoreboard.rs: board/storage/border/stopwatch preparation and checked ack,
   preserving existing loaded/default/sparse semantics.
7. game/bossbars.rs: native packed bars and worker-side existing-file decision.
8. game/functions.rs: native ordered schedule projection and sparse-file decision.
9. game/chat.rs: native clock-controls/ticks input; strict native clock merge on
   the IO worker, with the Level-writer ordering boundary below.
10. stopwatches.rs: narrow pure exact elapsed-millisecond snapshot and checked ack;
    same native save encoding, fixed sampling instant and retry input.
11. NEW game/paired_close_metadata_tests.rs: actual family owners/native files,
    worker stalls/failures/retries, matching/mismatched ack and state preservation.

Declare the new test module inside paired_close.rs. No other test declaration
path is needed. The module is dormant and feature-scoped consistently with its
accepted predecessor. If C0a or another accepted predecessor already declares the
same paired_close module, use that actual declaration and explicitly shrink this
packet's budget at fresh review; do not retain a nominal game.rs edit or make an
unreviewed declaration change in another packet.

The eleven-path maximum explicitly adds the declaration and stopwatch seam to
the older planning subset. It is not eleven-path source authority. No save.rs,
command_storage.rs, game/lan.rs, levels.rs, networking, session image codec,
producer implementation or ordinary caller change is included.

## Three separate responsibilities

Each family adapter separates pure preparation, owned native IO and checked
acknowledgment. Preparation borrows actual retained Game/family state, applies its
existing loaded/remote/player/UUID guards and captures owned native data. It does
no read/write/try_exists/directory scan/sync, lazy load, RNG advance, simulation,
as_guest, dimension swap or menu closure. A skipped family is an explicit native
guard outcome, not a fake successful write. Capture enough private original-state
information to refuse mismatched acknowledgment without altering the live owner.

The IO attempt receives only fixed typed native payloads, native path context and
immutable owners needed by those codecs. It receives no Game reference/pointer,
renderer, arbitrary closure, authority reconstructed from claims or generic task
interface. Filesystem-dependent existence/default-replacement decisions and all
native reads/merges/writes occur on this worker. Keep existing JSON/NBT field
shapes, gzip/rotation behavior, file names and error propagation. Existing write_dat
does not establish a new atomic-batch, fsync or power-loss durability guarantee.

Acknowledgment runs against the original frozen family owner and the exact
prepared batch/attempt identity after successful checked harvest. Validate ALL
affected family snapshots, loaded/dirty state, routing and private owner identity
before any live mutation. A mismatch, including equal bytes from another batch
or owner, leaves every original state and marker unchanged. A hash, Boolean,
serialized receipt or equal projection alone is not acknowledgment authority.
No callable constructor can mint a successful ack from public record data.
The dormant caller retains a private non-Clone preparation guard over the original
Game/family scope; it keeps that state frozen and owns the job/result association.
Only owned payload/context crosses to the worker. Do not use an address cast,
public UUID assertion or content equality as a substitute for this lexical
same-owner association. The retained guard/job must remain usable across status
turns without adding an ordinary Game field or callback in this packet.

Existing normal save APIs keep their behavior; new retained adapters may share
private pure encoders, but do not silently change normal partial-save semantics.

## Fixed job-owned IO and unfinished-handle rules

Use one concrete NativeMetadata job containing the selected fixed C0c1 family
set, its private batch identity and immutable retry projection. Do not implement
CoherentImage work here; C0d may add its separately reviewed fixed variant later.
One retained owner holds the original projection throughout every attempt and
at most one actual JoinHandle. An attempt result carries explicit completed
family/file evidence, native error/artifact information and private acknowledgment
material. There is no shared executor, journal, arbitrary callback worker or
general command framework.

Own the job BEFORE spawning. A suitable concrete handoff is a fixed worker entry
waiting on a one-job channel: register the successful handle in its owner before
dispatch, keep the immutable original projection in the owner, and send only the
worker's attempt reference/context. Spawn failure leaves the original projection
in place; send failure returns its input and retains/reaps the actual handle.
Never move the sole job into a spawn closure whose failed spawn drops it. Keep
the immutable retry input outside worker-local mutable state so unexpected panic
cannot erase it. Any native snapshot cloned for an attempt has no live authority.

Polling performs only bounded status work and is_finished checks. It does not
wait, sleep, recv_timeout, join an unfinished thread, run native FS helpers or
acquire a potentially waiting mutex. Contention is Pending. Harvest once through
is_finished -> checked join -> validated owned outcome. A panic is a joined
failure, never successful IO or implicit acknowledgment. Retain its immutable
original input and mark any unproven per-file outcome honestly unknown.

The original absolute close-attempt deadline can publish Blocked while the actual
IO handle is still running. Keep the SAME handle, input, original family owners,
native session/authority locks and failure history. Repeated status/cancel/close
requests do not renew the deadline or create a second job. Retry while a handle
is unfinished is RetainedBusy. A late successful result may be harvested but
remains Blocked until explicit Retry validates that exact harvested result or
starts a new bounded attempt. It must not silently acknowledge state or restart
an expired attempt. There is no separate automatic late-success acceptance route.

After an expected partial IO error, retain the original fixed projection and
artifact/error evidence. Explicit retry after external repair reuses that input
and existing native replacement rules; it cannot reproject later live state,
resample stopwatches or claim interrupted writes rolled back. No second worker
starts until the old actual handle is harvested. Preserve monotonic failure facts.
An uncorrelatable/panicked outcome requires explicit checked revalidation/replay
of the fixed native operations before a successful ack, never inferred success.

For this bounded retained batch, do not apply any live dirty/saved acknowledgment
if ANY selected family operation fails. Earlier disk writes can have succeeded;
retain that partial evidence and replay the same projection on retry. Attempt-local
clones may clear their own markers, but original Game markers stay unchanged.
Validate the whole acknowledgment set before committing its infallible CPU marker
updates, so an ack mismatch cannot partially clean the Game. This does not promise
atomic filesystem changes and does not alter the ordinary adapters' partial acks.

The returned owner is retained by the explicit dormant caller across every poll,
error and timeout. Planned Result errors must return/retain that same owner rather
than drop it. No disposal thread, timeout detach, hidden destructor join or lock
release counts as cleanup. Drop is only the adopted unexpected unresolved-owner
backstop; qualification must release controls and explicitly harvest before any
assertion/unwind can abandon an intentionally stalled owner.

## C0c1 actual family contracts

Research anchors are relative to clean C, to be refreshed at P:

- trading.rs:127-197 already clones RandomSequences and reconciles actual local
  station and host-play merchant loans. Matching state_words do not advance RNG;
  conflicting same-ID states refuse. Capture this exact clone/to_nbt before IO;
  never take/give back real loans to make preparation convenient.
- stats.rs:135-174 has to_json and loaded/remote/no-player/UUID guards. Preserve
  the actual record routing and pretty JSON/DataVersion. Network dirty sets,
  phantom state and scoreboard events are not disk saved markers and must not
  be consumed or cleared by preparation/ack.
- advancements.rs:105-135 uses actual Progress and the retained Tree to produce
  existing JSON. Preserve loaded/player guards, progress, recipes, toasts,
  changed/unlocked/announced sets and tree identity. Do not lazy-load progress or
  tree files on a poll merely to save an unloaded family.
- scoreboard.rs:44-86 saves native CommandStorage, Stopwatches, border then board.
  CommandStorage is Clone/PartialEq; an owned clone may call its existing native
  save on the worker. Its global dirty flag intentionally retries all namespaces
  after partial failure. Keep empty namespace replacement behavior. The original
  storage is acknowledged only against the exact original snapshot after the
  whole selected batch succeeds. Border and default-board existing-file checks
  stay on the worker so reset replaces existing nondefault data and fresh defaults
  remain sparse. Preserve loaded state, recorded criteria and all unrelated data.
- stopwatches.rs:59-80 currently samples current_time inside save and has private
  save_at/timer fields. Add a pure typed projection using one captured native
  millisecond instant, exact i64 elapsed values and existing dirty/empty decision;
  retain this exact projection across attempts. Encode through the same native
  data/stopwatches Long fields. Checked acknowledgment clears only the appropriate
  original dirty marker; preserve creation_time/accumulated timers and running
  semantics. Do not round through elapsed_seconds f64, rebuild watches from a
  load, advance timers, or resample on worker/retry. Keep normal save unchanged.
- bossbars.rs:29-44 packs actual loaded nonremote bars; empty bars require an
  existing-file decision on the worker. Preserve sent/viewer state and native
  custom_boss_events.dat representation.
- functions.rs:63-96 preserves loaded/remote guards, native function/tag callback
  fields and stable (time,seq) ordering. Empty schedule/existing-file behavior is
  a worker decision. Keep schedule, next_seq, enabled datapacks, store and pending
  load state; no execution/reload is part of preparation or ack.
- chat.rs:76-89 captures WorldClock controls and actual world day ticks. Use the
  existing strict save::write_world_clocks read/merge/write on the worker, retaining
  other clocks and unknown native fields and existing default-control removal.
  C0c1 must not run concurrently with Level::write or another clock merge. C0d
  must explicitly order its Level write and subsequent Game clock-controls merge
  on the same frozen world; no last-writer assumption or new replacement codec.

## Advancement helper and player-routing limits

AdvState.pending_tree at advancements.rs:100 owns a real prefetch JoinHandle.
advancement_tree currently takes/joins it without a finished check, and replacing
AdvState can drop/detach it. Add a narrow retained finished-only observation and
one-shot checked harvest in the scoped advancements.rs adapter. An unfinished
helper stays in the original AdvState and blocks completion without blocking a
poll; deadline/cancel does not take/drop it. A joined panic/spawn failure is an
explicit failure, not permission for a synchronous Tree::load fallback on a poll.
Preserve loaded versus unloaded progress and do not fabricate a ready tree.
The retained helper-start seam must report its own real spawn failure rather
than inherit ordinary prefetch's spawn(...).ok() ambiguity. Existing absence of
pending_tree does not prove a requested helper started/completed successfully.
Keep ordinary prefetch/tree-loading callers unchanged. C0e must separately wire
all real state-reset/world-replacement/return paths to this prerequisite.

These adapters do not establish complete local/remote/playerdata persistence.
game/lan.rs:1447-1500 and2643-2705 owns guest/owner JSON routing, private record-UUID
selection and store_owner's Level mutation; those consumers are not in C0c1.
New family payload APIs may accept actual trusted internal routing inputs, but
must not claim new player authority, reuse UUID claims as authority, move guests
or borrow as_guest to obtain a close snapshot. C0d's separately budgeted lan.rs
work must bind real retained owner/guest routing and native player records.
No remote Session or ordinary network behavior changes follow this packet.

## C0c2: separate future seven-path design budget

C0c2 requires C0c1 FINAL/private checkpoint and a fresh exact predecessor/API
review plus its own accepted bounded design before source assignment. Its maximum
is seven paths under crates/horizon-client/src: game/paired_close.rs,
game/maps.rs, game/dragon.rs, game/wandering_trader.rs, game/nether_portal.rs,
game/raids.rs and game/paired_close_metadata_tests.rs. This document supplies no
C0c2 source authority and does not add its families to a C0c1 success claim.

The later design must preserve these actual contracts:

- Maps: native per-map to_nbt plus last_id under data/minecraft/maps. Current
  save clears each successfully written map before later map/index failures;
  retained preparation must not clear live flags. Preserve texture/holder/atlas/
  missing state and capture the exact map-ID membership/index. Define checked
  batch acknowledgment and partial-write retry explicitly, not HashMap-order
  success. Do not load missing maps or allocate IDs during preparation.
- Fight: loaded+dirty gate, existing End-dimension path, optional exit-portal and
  respawn fields and native floor conversion. Preserve scan/crystal/blast/runtime
  fields; no forced load, respawn tick or simulation to obtain save data.
- Trader: loaded+dirty gate, spawn_delay/chance native fields and unchanged live
  tick_delay. No forced initialization or sampling RNG.
- Portal arrivals: dirty+nonremote guard is independent of loaded. Retain exact
  arrival tags, opaque payloads and waiting/planned ownership; never remove or
  normalize entities to serialize them. Dirty clears only on matching success.
- Raids: use the retained loaded dimension and actual Raids::to_nbt, not a guessed
  current dimension. Current save also projects raid_omen_position into Level
  data even when IO fails; separate that CPU mutation/ack explicitly. Do not call
  load_raids while preparing: it can save another retained owner and advance world
  RNG. Empty-session raid helpers are not a substitute for these full native
  family semantics. Direct active/parked borrowing remains C0d levels.rs work;
  swapping dimensions can invalidate native checkpoint provenance.

## Qualification and stop condition

Root alone performs all formatting/build/test/runtime/Git operations serially after
fresh independent exact SOURCE/tool CLEAR. Use genuine pinned resources and
isolated owned roots/home/data/config/cache/temp. No Rust test launches Cargo,
compiler, Java, Python or another test process. Test controls are fixed-action,
non-Clone native receipts with the required feature/debug gates; no arbitrary
worker closure or fake IO-completion input constitutes a positive.

Required C0c1 defining evidence includes:

- Every selected family with real nondefault native data, unloaded/guard-skipped
  states, fresh-default sparsity and existing-file reset. Decode actual output
  using existing native readers and compare fields/ordering/opaque data. Preserve
  genuine source/resource inputs and all live owners/notification state.
- Exact stopwatch milliseconds sampled once before a controlled delay; failed
  write, external repair and retry produce that same snapshot while original
  running timer state is retained. No f64 precision loss or retry-time advance.
- Actual per-family native IO stalls and partial-file/namespace failure, including
  strict clock read/merge failure and existing replacement artifacts. Poll/status
  remains nonblocking; actual unfinished handle and original locks/payload remain.
- Spawn refusal, dispatch failure, actual worker error/panic, deadline then late
  success/failure, RetainedBusy during unfinished retry, explicit retry only after
  harvest, once-only joins/results/acks and no competing worker. Release controls
  and reap actual workers before test teardown/assertion can relinquish them.
- Matching ack succeeds once; stale/replayed/wrong-attempt/wrong-owner/equal-byte
  foreign ack and changed live snapshot refuse before ANY original marker changes.
  Earlier disk success plus later-family failure leaves all live batch markers
  dirty/unacknowledged; retry retains original fixed input and failure evidence.
- Actual advancement prefetch delayed completion, finished-only harvest, panic
  and unloaded-state behavior. Preparation/poll never calls a waiting join or
  synchronous fallback load. No state replacement detaches an unfinished helper.
- Genuine local/integrated-owner/dedicated guards for the adapters actually
  covered, without claiming unimplemented guest/playerdata/remote caller wiring.
  Regression checks preserve existing normal family-save failure/retry/sparse
  behavior, notification state and feature-off builds. Existing stopwatch and
  CommandStorage tests remain relevant to the exact library seams.

Record exact source/tool/input/binary pins, required feature/lint/format/public-tree
checks and honest broad/calendar limits. Missing resources or skipped defining
positives do not pass. Freeze and preserve all scoped sources/prefix artifacts
before every correction. Complete a clean local candidate, DIFFERENT independent
FINAL and private durable checkpoint before C0c1 is consumed by another packet.

Writer/producer terminal close APIs are prerequisite inputs, not outputs invented
by this IO service. Their actual retained allocation/witness/lock contracts must
be refreshed after writer-adoption and C0a FINAL; never substitute writes_finished,
Arc counts, a joined metadata worker or serialized receipts for terminal producer/
writer completion. C0d/C0e and the C0 composite gate remain separate. Ordinary
startup/save/close, player authorization, full configuration/admission, registered
lifecycle, committed animation and explicit G1 qualification remain S1 work.
