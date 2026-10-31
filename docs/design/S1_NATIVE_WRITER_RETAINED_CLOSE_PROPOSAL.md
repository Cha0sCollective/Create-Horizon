# Retained native writer close and repair retry: C0b

Status: PROPOSED bounded FIRST prerequisite to C0a. No source authority, ordinary
activation, publication or S1/G1 acceptance follows. Independent design acceptance
and exact clean prerequisites precede source work.

## Exact input, path budget and output

One production path: crates/horizon-client/src/save.rs. All focused tests and
fixed native test controls remain INLINE in this same file. No other engine,
private domain, schema, codec, journal or executor source is authorized.

Research used unchanged save.rs SHA256
4999705141e8d64d2d8cee1972417b6faa78846948671935cb85bf3dbedd21c9
in the nine-path dropped-item worktree based on
ab239890cd62595889f7f0302cd1396a5e2c547c. Login clean candidate is
5b41f44fe83b53fc6b7c4dfe550cc0bc1aca5b83. B/D are NOT a combined FINAL input.
Before source assignment require B FINAL, D FINAL, independently accepted clean
combined successor with exact parent/tree/source pins, and a private durable
checkpoint. Refresh this save.rs and its D call sites at that pin. If source
qualification requires another path, amend/review this bounded proposal first.

The planning-only configuration proposal is
95484fa4861f26d8f66c6756bccaa7cbe2bc73c70a98aa5507315dec4d3c3550;
C0a draft3ef2bed2eec24a3221238248b0e1bc23f5b1fb35ca14af5edc173ce35a2d1e8f
identified this prerequisite. C0b supplies a genuine native writer completion
service, not a zero-outstanding diagnostic. C0a then uses it AFTER native producer
sealing/joining; C0d/C0e later integrate complete world snapshots and ordinary close.

Output: an opt-in SaveDir can seal native region/entity/POI write admission,
retain the original accepted records and actual writer handles through failure,
repair selected filesystem errors on explicit Retry, and return a non-Clone
allocation-bound writer-closed owner ONLY after actual handle harvest and native
pending/batch reconciliation. Dropping the resulting owner invokes no hidden
SaveDir writer join because its writer slot is already empty and permanently sealed.
This is writer completion, not complete Game/producer/metadata/image close.

## Actual existing seams

save.rs:1626 RegionCache owns Arc<Mutex<RegionFile>> per region; negative cache
entries and already-open handles matter during repair. cached_region1631 may
reuse an open handle; path repair alone need not repair its object.
write_region_records1668 resolves/compresses native records, writes region slots
and calls native RegionFile::sync. compress_records may start scoped encoding
helpers, joined by the actual writer thread before its batch returns.

Record1768 variants are Tag, Chunk with original Arc<Encoding>, Poi with native
SaveTables, FinishedEntities with original Arc<Chunk>, and Delete. Do not flatten
these into a replacement JSON/NBT format or prematurely encode them for retries.
PendingWrites1905 holds latest (Folder,x,z)->(seq,Record) and supplies pending reads.
Folder covers Region, Entities and Poi. WriteBatch1906 currently is a native folder
and Vec of key/Record/seq. Writer2018 privately owns sender+JoinHandle; SaveDir2032
owns that Writer slot plus outstanding count/error/condvar and native caches.

queue2193 currently updates pending/sequence/count before obtaining writer lock
and send; start_writer2231 has spawn.expect. A failed send changes count/error.
Worker errors retain failed latest snapshots, decrement completed batch count,
set sticky error and continue later batches. Successful batches remove pending
only if its sequence still matches. Worker panic can strand outstanding counts.
writes_finished2330 only checks count+sticky error. Drop2526 closes sender and
UNCONDITIONALLY joins its private handle. None is an explicit retained close API.

## Explicit opt-in and concrete API sketch

Names are internal proposals, not stable host APIs. With paired-packages, provide
`SaveDir::new_retained(root)` for future retained native construction. It begins
with no writer, no pending record and a private retained lifecycle allocation.
The default `new` keeps its existing behavior/legacy queue/flush/Drop semantics.
No API retrofits a legacy already-writing SaveDir using a count/identity claim.

Possible concrete services:

- `begin_retained_writer_close(self: &Arc<SaveDir>, deadline)` -> Busy or an owned
  non-Clone NativeWriterClose retaining the SAME Arc and actual close identity.
- `NativeWriterClose::poll(&mut self)` -> Pending, Blocked(reason), or completed
  result available for checked one-shot harvest. Poll never silently retries.
- `NativeWriterClose::retry(new_absolute_deadline)` starts one explicit repair
  attempt only when the previous real writer attempt has finished and been joined.
- `NativeWriterClose::finish(self)` -> NativeWriterClosed or the same unresolved
  owner. A completed token retains its same SaveDir allocation and private identity.

SaveDir's retained lifecycle is concrete in-memory ownership state, not a generic
journal: Open, SealedDraining, Blocked, Retrying, Closed; one current close owner,
monotonic epoch/batch identifiers, exact accepted/completed batch facts, original
latest-record ownership and cumulative failure history. It is installed at
construction, BEFORE the first retained queue. Later record successes never erase
unexplained prior failures. No public setter manufactures a completed token.

Only the unique typed close owner controls Retry/finish. All Arc aliases see the
same admission seal and terminal state. Keeping/dropping an extra SaveDir Arc
cannot restart its writer or issue another successful begin-close. No strong_count
or weak_count, status Boolean, zero count or copied pointer constitutes authority.
The completed token proves the native writer slot/retained handle set empty and
permanently sealed. External synchronous readers/other native owners are outside
this token; C0a must actually stop its own producer/read workers, not infer that
from this writer token. Region caches may remain for ordinary pending/disk reads.

## Queue transaction and seal race

Retained queue takes the SAME private admission gate used by begin-close. Use a
single documented lock order, e.g. admission -> writer -> pending -> outstanding
count -> current-error/retained batch facts. Worker code must not acquire admission
while holding another one of those locks. Source review traces every such edge.
Default queue remains the existing path. The retained path must check Open under
this gate; post-seal enqueue returns error with no pending/count/sequence mutation
and no implicit start_writer call, even through another Arc or empty wait request.

Before any accepted publication, validate exact sequence capacity for ALL records,
batch identifier/count increments, writer spawn result, all needed lock outcomes
and existing native invariants. Prepare owned records/batch so every early return
can return or retain those exact records, not drop them after partial mutation.
An enqueue started before close either commits its entire accepted native batch
under the gate, or leaves a correlated owned failure. Close cannot slip between
pending insertion and its count/send accounting. A partial outer save_later that
accepted Region then failed Poi remains explicit: retained Region acceptance and
caller dirty markers are not rolled back or reported as a complete save.

A retained-only fallible writer-start service returns native spawn errors without
panic, nonexistent counts or lost Records. Do not make planned spawn failure an
unexpected unwind. Default infallible start behavior need not change. Sender send
failure returns its original unsent WriteBatch; correlate its accepted snapshot,
sequence and count exactly once. A proved never-enqueued batch may remain a
repairable retained attempt only if no worker-side ownership is ambiguous. An
unexpected receiver death/panic is not converted into a repairable filesystem error.

No premature success from an empty input bypasses seal/closed checks. Sequence
exhaustion and count overflow refuse before mutation; no fetch_add wrap/reset.
Current public wait=true/default flush may block by native design; retained close
poll itself never calls them or holds its gate across a condvar wait or disk IO.

## Bounded close and actual worker harvest

Call begin ONLY after the caller has accepted its final native snapshots and
stopped ordinary producers. This caller condition does not waive the intrinsic
admission seal: queue races/aliases still fail closed.

Begin uses try_lock on close's admission/writer transitions. If locks are busy,
return Busy without surrendering ownership or reporting sealed completion.
Once both are owned, atomically mark SealedDraining, take the actual Writer from
SaveDir's slot and retain its real JoinHandle in the close owner. Drop its sender
so the worker consumes all ALREADY accepted batches then exits on channel EOF.
The channel receiver owns queued work meanwhile; the original latest pending map
remains the failure/read owner. No new producer enqueue can restart a writer.
Healthy no-writer/no-pending construction completes after the same invariant checks
without starting a dummy thread.

Poll checks is_finished on the retained handle, exact count/pending/batch facts
with try_lock, and original absolute deadline. It never sleeps, waits on condvar,
performs native filesystem work, waits for a mutex, calls flush, runs encoding,
invalidates file caches or joins an unfinished handle. Contention is Busy/Pending;
deadline retains Blocked with all handles and records. One indivisible finished
handle join is permitted ONLY AFTER is_finished, with its actual outcome checked.
A completed handle returning panic is an owned fault, never a normal write success.
Native writer scoped compression helpers are finished before their parent handle
is finished; no untracked writer helper is spawned by this new service.

Finite per-turn bookkeeping uses at most16 batch/record facts or2ms between
indivisible actions. Keep a cursor instead of cloning/scanning an unbounded pending
map on the sole owner loop. No hard OS/allocation/one-record2ms guarantee. Existing
record encoding/native IO stays on the real native writer worker.

Success requires accepted batch facts accounted once, no outstanding count, no
pending latest record, no unexplained sender/worker/receipt fault and actual writer
joined. Record-deletion success means the existing native writer contract, including
existing absent-region no-op. No wider durability guarantee than current native
RegionFile write/sync is added. A Blocked IO error can have count0 and joined handle
but retained pending records; this is NOT Closed and cannot release a world lock.
Repeated close/cancel/status calls do not renew the deadline or retry automatically.

## Explicit repair retry of original records

The repairable profile is a CORRELATED normal native filesystem/codec-resolution
Result error whose batch completion is known, with all original accepted records
represented in latest pending or proved successfully superseded/durable facts.
Unreadable old POI can be repaired only by actual operator/fixture repair; it cannot
be treated as absent, have claims cleared or be defaulted to pass. Unknown poison,
stranded count after panic, lost record or ambiguous batch ownership remains owned
Blocked. No reset counts, synthetic acknowledgments or blanket clear_error method.

Before Retry the prior writer must be actually finished, joined/check-harvested,
its sender closed and all accepted/completed batch counts reconciled. If still
unfinished, Retry refuses and retains Busy; no competing writer is spawned. The
previous error stays in cumulative history. Retrying an already Closed owner fails;
a terminal SaveDir is not reopened by obtaining another Arc or calling new API.

Freeze the SAME latest pending (Folder,x,z,seq,Record) ownership under the retained
seal. Use original Record variants/Arc payload/context and original sequence values;
no independent domain item copy or alternate serializer. Retry order is deterministic
by original sequence across ALL three Folder kinds and keys. Preserve native order
within each emitted same-folder batch; coalesce only adjacent compatible records.
A newer pending sequence supersedes an older one; never replay a stale failed batch
over a newer accepted snapshot. Earlier success plus later failure, earlier failure
plus later success, and later batches continuing after sticky error require exact
latest-sequence correlation, not a list of first-failed batches.

Spawn the next fallible REAL native writer attempt only after previous harvest.
The worker receives the retained frozen retry records; pending reads still observe
those original latest snapshots while IO is blocked/failing. Retry batch completion
uses a new checked in-memory attempt epoch, preserving original record sequence.
A failed Retry retains those same records, handle and error history. Success removes
pending only by matching that exact current sequence, proves native result, closes
its sender, harvests the finished handle and only then publishes Closed. A precisely
correlated CURRENT attempt error may be retired on success; cumulative prior error
history remains observable. Ordinary sticky flush errors are not silently waived.

## Cached RegionFile repair

Once the old writer has actually joined, the new native retry worker refreshes
relevant Region/Entities/Poi cache entries BEFORE their retry writes/strict POI reads.
Remove retained stale/negative cache entries and reopen repaired paths through the
SAME native RegionFile routines; all native filesystem open/drop/refresh work runs
on that worker, never the sole owner poll. An already-open file handle does not
prove it addresses the repaired path. Do not adopt an old header cache by copying
its contents, turn an unreadable POI into None, or change RegionFile/codec files.

Concurrent synchronous readers may finish against their existing read handles;
that is not writer authority. Cache mutexes serialize new entry publication and
normal readers, and no previous writer can write an old handle after its joined
proof. Later actual native reads must use refreshed caches and match disk. Tests
retain an old cache/read handle across actual path replacement to verify this
case. C0a separately proves all world-owned producer/read threads finished before
its complete close; no Arc-count shortcut is part of either proof.

If per-entry refresh cannot preserve native strict POI/header/read rules in this
single path, STOP and revise the proposal; no whole-cache error reset or silent
loss of pending records is an approved fallback. No arbitrary external filesystem
replacement/concurrent world writer correctness promise is made.

## Lifetimes, errors and defaults

The unresolved close owner keeps its same Arc<SaveDir>, original records and all
actual handles. Its normal Drop is not a drain/join algorithm. Follow the adopted
unexpected-ownership-loss backstop for armed unresolved destruction; expected
deadline, IO error and Retry refusal preserve the owner instead. Do not release
native SessionLock in C0b: that lock belongs to the caller and may be released only
after this writer witness plus actual other native completion witnesses.

A completed writer owner can safely be relinquished: SaveDir's writer slot and
retained handle set are empty and admission remains Closed even if other Arcs
remain. Default SaveDir Drop remains unchanged for legacy instances. No optimization
that skips a writer join merely because an outstanding counter is0 is allowed.

No global Java/Prism/launcher/world/cache paths are touched. No public source push,
PR, merge, release, Stage2 activation or G1 claim. Source requires exact independent
CLEAR before Root compile/runtime; final evidence requires DIFFERENT reviewer.

## Required genuine qualification

Root alone runs serial build/fmt/runtime/Git with isolated scratch roots and pinned
resources. Tests live inline save.rs, may use existing fixed native writer controls
and narrowly extend them under cfg(test) or native-writer-test-control AND debug
assertions for cross-bin access. No arbitrary production callback/readiness forge.
No Rust test spawns compiler/Python/Java/test children. All controlled workers are
released and polled/reaped before assertions/fixture teardown that can unwind.

1. Healthy no-writer close, healthy three-folder real writes including Delete,
   actual finished-only join/closed witness, native fresh-cache/RegionFile reread.
   Keep extra SaveDir Arcs through closure and prove all new enqueue/start/close/
   Retry attempts refuse without changing native files or spawning a helper.
2. Hold the real writer batch with the existing native control; multiple close
   polls progress/return while actual thread remains unfinished. Deadline yields
   same owner Blocked, pending-read snapshots/record identity/count conserved,
   no Drop/unfinished join and no retry worker. Release then explicit Retry or
   continuation according to the stored state, actual finished/joined proof.
3. Real filesystem IO failure THEN actual path repair THEN explicit Retry for
   Region, Entities and Poi independently and in a mixed-folder close. Include
   partial region-prefix write/sync error, retained FinishedEntities/Chunk encoding,
   POI claims/empty sections/unreadable old POI, entity deletion and native decode.
   Never satisfy repair by manually clearing an error or replacing pending Records.
4. Pre-existing cached file handle/negative entry, repaired path replacement and
   subsequent native reread through refreshed cache. Keep old handle live as a
   control; no old writer remains capable of writing it. Strict POI claims and
   unrelated existing chunk slots survive repair; original failed artifacts kept.
5. Latest-key supersession before seal, failed older batch then successful newer,
   successful older then failed newer, later unrelated batch success after sticky
   error, same coordinates across all3folders and deterministic original sequence
   replay. Prove no stale overwrite, original latest Arc/context identity retained
   and pending reads expose correct newest snapshot until actual retry success.
6. Seal/enqueue races with multiple Arc callers; accepted batch exactly once or
   refused unchanged. Test partial outer Region acceptance/Poi refusal, unsent
   batch ownership/send failure, first/later spawn failure, zero-record waits,
   sequence/count/batch-id exhaustion and no nonexistent outstanding count.
7. Actual worker panic and disconnected receiver, poisoned pending/count/error/
   lifecycle/cache locks and unexplained count/receipt inconsistency. Poll returns
   retained Blocked promptly, no fake recovery/count reset/error clear; actual
   finished handles harvested even though payload-accounting fault remains. A
   backstop-negative child may be used only by an external Root-owned harness,
   never a test-spawned Cargo/Python/test subprocess or planned data-loss escape.
8. Retry while old handle unfinished, repeated Retry/begin/finish, stale/wrong
   close owner, equal-value new SaveDir allocation and error-at-every handoff.
   No duplicated writer, cross-allocation token acceptance or closed restart.
9. Existing save/POI/generation/pending-read/flush/Drop tests and D native item
   save/restore regressions. Default constructor and feature-off builds preserve
   prior behavior; paired/no-runtime/full-runtime feature checks with tests, fmt,
   public-tree and appropriate broad checks. Calendar/parity limits stay honest.

Freeze exact one-path source and qualification tool/input manifests. Preserve every
failed source/tool attempt before correction. Qualify exact clean local commit,
obtain DIFFERENT FINAL and private durable result checkpoint. Then refresh C0a
against its actual accepted API; C0b alone enables no ordinary paired startup or
complete world teardown. S1/G1 and full C0/C1 outcomes remain open.