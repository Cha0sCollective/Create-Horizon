# Retained received-column streaming prerequisite

Status: HELD PROPOSAL ONLY. No engine source, execution or activation authority.
Exact proposed parent: decoder FINAL 9f6466482189583678c6a16711086c037424ae75,
tree f50509098c6dbe50fe79e3e551ed43e70f87d06b. Different FINAL receipt
.local/s1dc1-final-review.json SHA256
a4502d268876c1632e0d68bbbebe458b5b191e2a5675598930c7c911319c132d accepts it;
reviewed public branch codex/s1-retained-decoder is synced. The accepted immutable
index is .local/s1dc1-final-review-input-r1.json SHA256
53e9d762ea1f05307ddd43911738053dcc92889ef30a5ebeef1e20eac36e282c.
The original index without -r1 is a preserved incomplete draft, not accepted input.
Decoder private results checkpoint 74d4767e95b7fdd100811793ab73ed4db50ed113 is
synced. S1/G1 remain OPEN.

## Actual gap and one-path budget

Proposed sole engine path: crates/horizon-client/src/streaming.rs, including inline
tests. Use the same stable s1-native-close-composition physical checkout and its
own warm /home/arbiter/.cache/create-horizon/target-s1nc1, serially. Provisional
logical branch codex/s1-retained-remote-stream and prefix .local/s1rw1; no new
physical worktree, target copying or shared target is needed.

Existing Streaming::start_retained requires an actual Arc<SaveDir> (:1298-1308).
Its factory rejects a generator without that exact retained save allocation
(:1460-1467). RetainedStream.save (:871), SealedStream (:845-853), writer
acknowledgement and finish_retained_close (:4393-4470) deliberately belong to
native storage producers. remote::NoTerrain has no such binding. Supplying an
unused SaveDir, creating an empty native writer or setting remote=true cannot
establish the missing remote-client completion. Zero requested legacy threads
still starts a worker (:1469), and legacy Drop waits/joins (:4746 onward).

The received-column client needs real Mesh work. Streaming::update also has a
reachable Light submission path (:2330-2351); remote=true does not exclude it.
Support BOTH Light and Mesh in the fixed profile. This packet must not silently
turn off lighting or use an unsupported-kind panic as its admission policy.

Reuse existing queue/pool, mesher, light algorithm, original context owners and
ownership/result reconciliation. A narrowly shared Mesh/Light execution helper
inside this file may preserve existing native/legacy behavior. Do not implement
a second generic thread pool, executor, scheduler, storage abstraction or module
tree. No Game/remote/net/LAN/decoder/renderer/dependency/lock edits or caller
activation. If the actual implementation needs another path, withhold source
and propose the exact gap and revised budget before editing it.

## Fixed remote construction and intrinsic admission

Add a distinct paired received-column construction recipe selected before any
native acquisition. Proposed API direction: RetainedRemoteStreaming::prepare
accepts the actual Arc<WorkerContext> and bounded valid worker count, creates an
inert wrapper privately owning the original concrete Streaming, and an explicit
start(&mut self) operation acquires its workers. Partial startup failure remains
in that same wrapper. Do not return the existing StreamingStartFailure with its
public inner Streaming field. Existing construct starts threads.max(1) workers;
it cannot serve as inert preparation unchanged. Install the remote mode and
unresolved-owner fatal protection before the first spawn. Use
a fixed internal remote provider; accept no caller-supplied generator, native
storage owner, callbacks or general recovery executor. Refusal before acquisition
returns supplied original inputs. Exact names and representation require source
review, not a new general host API or compatibility promise.

Proposed fixed bounds for this dormant profile: one through eight native workers,
with zero or more than eight refused before acquisition and the original context
returned. At most 64 original Mesh/Light jobs may be owned across queued, active
and completed work awaiting reconciliation together. Reserve capacity before
moving an input into admission; full/busy/closed refusal preserves its original
owned input, or leaves caller-owned world data untouched if no ownership moved.
Retire a slot only on an owned handoff or retain it in the closed owner. These
counts are provisional profile policy, not a byte/global-memory or performance
qualification. Later caller activation/backpressure remains separately reviewed.

The opaque wrapper exposes ONLY checked received-column admission, light updates,
settings needed by that fixed profile, try-only observation/reconciliation and
close/Retry/completion. Checked owned-input refusal returns the original directly;
borrowed-world operations preflight before mutation. No retained-refusal queue is
needed. Resource reload/world replacement remain unsupported in this packet.
There is no borrowed or owned inner Streaming escape: no getter (even immutable),
Deref, AsRef, conversion, extractor or public startup-failure owner. In particular
legacy insert_remote/reload_resources void APIs, blocking observe_native_work/
pending/busy/stages, lod_source, save and native_write_fence_until are inaccessible.
No opaque callback is accepted or invoked; no callback-capture retention promise
is made. Later Game composition must adopt this wrapper or a separately reviewed
enum representation; this packet promises no directly extractable Streaming.

The private constructor-issued mode must intrinsically restrict work to received
columns, their Light/Mesh work, fixed result collection and close. Mutable public
legacy fields such as remote, mesh, forced areas or band settings cannot grant
generation, decoration, finish or storage work. Every route that can enqueue an
unsupported job, invoke a storage callback or enter a native save/fence operation
must refuse BEFORE acquisition, callback execution, ownership loss or work
admission. Include private scheduling helpers and any shared save/poll/fence paths
reachable internally, not only wrapper visibility or the new public constructor.
Keep legacy/native public signatures and behavior unchanged. The wrapper never
forwards unsupported calls to them. A refusal is explicit; it does not fabricate
a successful empty generation/save receipt. Preserve original refused inputs.

The fixed mode is never an upgrade of a legacy Streaming and never native-world
provenance. Existing native retained factories keep their real SaveDir allocation
checks, SealedStream and matching NativeWriterClosed requirements unchanged.
An unrelated native closure cannot close this remote owner, and its remote
completion cannot satisfy native producer/writer acknowledgement. Do not relax
the native predicate to accommodate a missing save allocation.

Keep original concrete Streaming ownership private within the wrapper for later
original Game composition without exporting a mutable Game or accepting arbitrary
native owners. The remote mode and lifecycle state remain private and permanent. This
packet supplies no conversion back to legacy/native, callback setter, restart,
active worker handle, mutable actor payload or broadly callable completion mint.

## Original worker, input and result custody

Prepare the complete original context, actor inputs and partial-start owner before
spawning. Register every actual JoinHandle before releasing its fixed start latch.
A failure starting worker N retains the same owner, already started workers,
context and failure; no Drop join, detached handles, replacement workers or
reconstruction of an equivalent pool. Prepared-but-unstarted close is valid.

The existing loop removes Job from its queue into the worker stack (:1551-1560).
Job::Light and Job::Mesh own their Box<Region>, sequence, coordinates and settings
(:482-486), and execution constructs Done::Lit/Done::Meshed (:1667-1693). WorkLease
counts alone cannot retain those actual inputs through unwind. In the new remote
recipe, register the original queued/active region and exact context association
outside the worker's unwind stack. Actual algorithms borrow that owned input.
Keep completed light vectors/SectionMesh, original sequence/settings association,
queued work, active failed work and late results in registered custody before
subsequent fallible native actions. Poison recovery must retain those owners.

Distinguish parser/mesher temporary allocations inside an interrupted pure call
from original job inputs and successfully published results. Do not promise a
snapshot of every internal temporary. Do not describe an already handed-off
mesh as still owned, or a failed/unprocessed job as successfully uploaded/applied.
Preserve existing result sequencing and stale-result decisions through explicit
owned reconciliation. A newer column or resource revision is not permission to
silently lose an old retained input/result. Resource reload and world replacement
are outside this fixed component unless separately accepted.

Use fixed received-column and light-update operations plus the existing actual
meshing/light execution; no synthetic result echo or empty mesh shortcut. Caller
world admission and authoritative simulation freeze remain separate obligations.
This helper owns its actual job references and results, not the whole Game,
network backlog, renderer uploads or a native world checkpoint.

## Pollable close and truthful typed completion

Close permanently seals the fixed remote admission/claim policy. Define one
linearized ordering between claiming an original queued job and publishing the
close fence: work claimed before sealing may finish late, queued work must not
start afterward. Stop/wake cannot be lost while close/status stay nonblocking;
do not notify a Condvar without accounting for its predicate-lock race. Retain
queued work explicitly unprocessed, or reconcile it through the fixed close
disposition; do not schedule generation/storage to drain a remote close.

The wrapper's try_observe remote census is the qualified observation surface;
unavailable is represented explicitly. Legacy observations are not forwarded.
UI-facing observation/close/Retry/poll must not wait on an actor payload lock
held during real Light/Mesh execution or join an unfinished thread. Busy census
is explicitly unavailable, never fabricated zero. An expired original deadline
still reports Blocked with the actual attempt even when census locks are busy.
Repeated close preserves the first deadline. Exact-attempt Retry extends waiting
only; it does not reopen admission, replay a job, clear original failure or start
replacement workers. Claim no hard time bound on algorithms, allocation, mutex
operations, OS acquisition or scheduling.

Only positively finished original native handles may be taken and joined, once,
with checked outcomes. Distinct opaque ClosedRemoteStreaming is issued only by
the original remote recipe after all its actual worker closure conditions hold.
It retains original unprocessed/failed jobs, late/stored outputs and checked
failure outcomes for explicit caller disposition. It is not SealedStream,
NativeWriterClosed, ClosedSession or a native save/whole-client completion witness.
No native writer acknowledgement is needed or forged for work that owns no writer.
Disarm the inner remote destructor only after its actual finished handles are
checked and joined and its remaining original payloads have transferred to the
closed owner; never trigger the inner legacy blocking Drop as a remote shortcut.

Checked native panic or startup refusal can yield a visible FAILURE handoff after
actual native quiescence; unfinished work stays owned and Blocked. Do not offer
an impossible Retry as repair for an already terminal failed actor. No completed
work is inferred from a stop request, empty queue, elapsed time, channel loss or
test marker. Unresolved remote destruction uses a fatal backstop before field
drop could detach handles; failed stderr logging cannot prevent unconditional
abort. Legacy and native-retained Drop behavior remain separately preserved.

## Defining checks and cost

Use actual pinned WorkerContext/resources and real received columns with a known
neighborhood. Existing streaming native_work_tests::context (:6834 onward) loads
real Assets, RenderAssets, MeshTables, LightTables and Biomes; missing prepared
resources FAIL. Use the established pinned resource inventory at batch boundaries
and process isolation for actual resource cases. No new resource preparation,
Java oracle, Game/window/Session or network launch is needed for this component.

Aim for a few grouped production-worker scenarios, not a broad old battery:

1. Actual received-column Mesh AND Light execution with independently expected
   nonempty geometry/light observations and original context/region/sequence
   association; result handoff and ordinary stale-result disposition are distinct.
2. Inert preparation, invalid-input refusal, partial native startup failure and
   initialization failure retain original context, every acquired handle and
   original failure through checked closure.
3. Held real active work, queued work and a stored/late result exercise sealing,
   busy census, expired deadline, unchanged repeated close and exact Retry; all
   original handles finish and join before typed remote completion.
4. Controlled actual-worker unwind before completion and after storing a real
   result preserve original active/queued/published payloads; no job replay,
   replacement or discarded failed region is hidden by counts.
5. Intrinsic profile refusal rejects generation/storage/native close operations
   before callbacks or work. Wrapper visibility prevents access to legacy mutable
   flags; private inline tests also exercise internal fixed-mode guards.
   A native writer witness cannot substitute for remote closure or vice versa.

Exercise all 64 slots across queued, active, stored and reconciling phases together,
original-input capacity refusal, and retirement only on explicit owned handoff.
Use deterministic actual-worker claim/fence and idle-wakeup controls. The wrapper
must not turn slot saturation into silent world mutation or input/result loss.

Fixed controls hold actual native actors; they cannot declare a worker finished
or replace Mesher::mesh/light_chunk with an echo. Add separate actual unresolved
owner fatal children with entered marker, expected diagnostic/SIGABRT, disabled
cores and real process reaping. Include the meaningful failed-stderr variant
without another test framework. Watchdog expiry is failure, never completion.

Choose the smallest affected native-stream regression covering the preserved
factory/writer association and the shared Mesh/Light execution path. Reuse older
qualified native/decoder/window evidence only at its actual pin; do not rerun
full Game/window/network/save batteries absent an affected dependency or concrete
unresolved concern. Counts are planning estimates, not caps. Appropriate format,
paired all-target lint/compile, feature-off and public-tree checks still apply.
Bind actual Cargo artifacts/features/profile/binary and input identities.

Existing metadata scans and native wire/body allocations have no new global byte
or total-memory guarantee here. Source review verifies the fixed worker/job policy;
a later client activation must supply its own network/backlog
backpressure. Do not claim a latency/performance qualification from these checks.

## Gates and later composition

Decoder private evidence checkpoint comes first. Then require fresh independent
read-only Sol High DESIGN acceptance of this exact proposal and bounded assignment,
a synced private design checkpoint and root source release. Preserve the entire
one source path and packet-prefix artifacts before corrections, and all accepted
historic archives/indices including the decoder -r1 index. Fresh exact SOURCE/tool
review precedes formatter/compiler/runtime; a different FINAL and private
checkpoint precede use or reviewed public branch sync. Keep normal hooks, fresh
repository visibility/origin checks and existing permission limits.

Stop this packet at its bounded FINAL and checkpoints/public reviewed branch.
The real remote-client initializer still needs the already qualified RetainedSession
in net.rs, retained decoder adoption, original remote initial Game construction
and fixed same-owner close coordination. True scoped integrated composition also
needs original server Game startup custody, retained LAN-to-Hub transport, native
participant/save ownership and checked original server-thread completion. Its
client LOD must finish before sealing shared server storage. Current blocking
ready.recv and arbitrary Integrated::call are not supplied by this helper.

No default activation, ordinary integrated/dedicated/two-client behavior, machine
lifecycle, committed animation, G1, live S2, PR, merge, tag, upstream contribution
or release follows from this component. No new policy or authority is adopted by
this draft.
