# H08 used streamer ownership and one native tick

DESIGN ACCEPTED after fresh independent read-only Sol High review of clarified
packetSHAdf88d50c4b9f98c05d0fe69304e9f5cba9be5415baca79105077b07c701cd09f,
following conditional review of originala15cc5dece0f9f7a391ebce6bd36ac32bcbd2124d758db2c5377c8a9366b03be
and audit3062e4a2f50f4ef0fa8f1e6c398ec861960b59016636b502fd7f670020178dcb.
RELEASE CONDITIONS SATISFIED: advance-capture has different final acceptance.
Accepted clean input LOCAL bb6f718cecb41ff0ffd563699dd3d731f9b3fe53/tree
a243ac6bd646ed8152eaa5ebe6be846c1f0cf878 matches the reviewed frozen candidate,
.local/h08sa1-source-sha256.json. Coordinator may release ONLY the three paths
below on fresh codex/h08-used-streamer after recording/pushing this private
checkpoint. Exact-source review must precede execution. The explicit handoff
obligations below remain part of the accepted design. Source analysis:
H08_USED_STREAMER_OWNERSHIP_AUDIT.md.
No capture admission, ordinary startup recovery, G1 or publication acceptance.

## Objective and bounded source paths

Observe named streamer work ownership across existing asynchronous handoffs and
exercise it through ONE actual native tick_server(false) under the retained
fixture context. Prove current capture refuses, without treating empty visible
maps or a zero count as global readiness. This is a prerequisite for future real
runtime capture, not another fixed scalar profile or a replacement barrier.

Proposed three paths relative to crates/horizon-client/src:

1. streaming.rs: narrowly scoped ownership lifetime accounting, borrowed named
   observation and defining tests of native enqueue/worker/result/reconciliation.
2. game/levels/slot_epoch_integration.rs: declaration of the new test child ONLY;
   keep every current production capture/recovery/projection path unchanged.
3. NEW game/levels/used_streamer_tick_tests.rs: actual one-tick Game/refusal proof.

No world/save/codec/registry/journal/SDK/native Task ABI or normal activation change.
Verify all actual enqueue/result/error/drop paths and required visibility before
editing; a concrete dependency outside three files requires a reviewed amendment,
not a silent scope increase. Root owns all builds/tests/fmt/integration/commits;
one bounded author may own these three files only after release. Read-only reviewers.

## Ownership observation, not readiness

Keep require_pristine_fixture_capture and its zero-historical-submission policy
unchanged. Add a separate borrowed report of named current work within one actual
Streaming instance: submitted/enqueued, worker-held, queued-result, and retained
unresolved error/disconnection states, with enough producer identity to distinguish
generation, lighting, meshing and existing decoration/finishing/save paths.
Do not call it complete, safe, ready or an admission token; no global world claim.

Account before a request becomes externally visible. Transfer responsibility
through the existing worker closure/result message until normal reconciliation
consumes or explicitly disposes that work. A cancelled map entry, forget/reload,
stale generation, early return, worker failure/panic or failed send must not make
unobserved work look absent. Normal stale-result disposal may settle the corresponding
ownership only when its actual result/payload is reconciled, not when an index is
removed. Retain a named fault when no normal completion/disposition is known.
Use checked bounded accounting; overflow is a retained refusal/fault, never wrap
to zero. Streamer destruction must not fabricate successful reconciliation.

Use the smallest concrete lifetime mechanism supported by the existing native
task/channel paths. Do not build a general task scheduler, retry journal, portable
manifest, telemetry framework or cross-process coordinator. Preserve enqueue order,
native work/results, cancellation behavior, rendering and all current callers.
Report actual scope gaps before choosing a mechanism. A borrowed observer must not
poll receivers, drain queues, wait, cancel, mutate ownership or clone owning payloads.

Visible maps and save_in_flight are insufficient individually: cancelled lighting/
mesh indexes can disappear while workers/results remain, and save_in_flight does
not cover every producer. Do not infer completion from time elapsed or map lengths.

### Source-reviewed handoff obligations

Carry request origin explicitly: Generate and Finish both return Done::Generated,
so result kind cannot reconstruct ownership origin. Cover update, save_until,
reconcile_ready_save_results, receive_save_result, poll_save_budget, wait and Drop,
including deferred/unprocessed result destruction and worker initialization panic.

Cover synchronous callbacks too: receive_generated -> save_finished and save_until
-> save_protos. Preserve generation_save_error and failed_finished facts. These
Persistent callbacks return asynchronous SaveDir ENQUEUE ACCEPTANCE, not writer
completion. save_finished can accept region/POI work before entity enqueue fails.
The report may name accepted external handoff, never writer completion or absence
of writer owners. Tracking beyond this boundary requires reviewed scope amendment.

The sibling child cannot use private advance_tests::History/history. Keep bounded
local setup via visible source/loaded/Fixture/dispatch/install APIs; do not include
the old module wholesale, edit old bodies or widen visibility without amendment.

## One actual Game tick

Reuse the qualified actual epoch2 native3/T, PLAYER3 andslot7 setup with expected
WorldId retained independently from its original context. Invoke tick_server(false)
through the actual existing method, without assigning time, freezing producers,
mocking native success or suppressing work to retain pristine admission. Observe
the actual Game clock/counter and named ownership before/after; a path that never
reaches the real tick cannot pass as a completed step.

Record PLAYER/slot state, world.inhabited and relevant native/save metadata and
all named producer observations. Natural spawning updates inhabited before checking
spawn rules; disabling mobs does not justify expecting unchanged inhabited time.
Do not clear/reset this metadata or remove owners to satisfy an older profile.

Call the unchanged pristine API directly to establish PreviouslySubmitted after
real native submission. Exercise current capture through its actual Task and
assert explicit NotPublished refusal with complete post-tick state/source/archive
preserved. Earlier E1/W1/native-metadata refusal may take precedence over Streamer;
do not alter guards merely to force a particular error rank. Observe streamer
presence independently. Actual work may complete quickly; no claim requires a
particular transient count at an uncontrolled instant. Zero still does not admit.

Keep producer latches/controlled channels in native defining tests for negative
lifetime proofs, not as a purported Game capture barrier. Preserve exact native
payload values/allocations while held or queued; no queue drain to manufacture zero.

## Required validation

### Discovered idle shutdown correction, pending exact source review

This amendment has fresh independent DESIGN ACCEPT at packetSHA8f61b477bb09a846045e7b8d43c8aa12eabca3ae620085ef61243982c664b073.
Its corrected source/test assertion has full independent execution clearance;
runtime/clean/GUI/broad/final acceptance remains separate. The exact interrupted
process's program counter remains unknown, as stated below.

An unchanged retained save-budget test stalls on the corrected candidate after
earlier runs passed. Root preserves its multi-minute futex wait and explicitly
stops ONLY its verified isolated process; no pass or spontaneous test failure is
claimed. Source inspection identifies a possible pre-existing lost-wakeup window:
worker checks stop while holding the queue mutex, then waits, but Drop publishes
stop/notifies without that mutex. Scheduling inference alone is not root-cause proof.

The bounded author may diagnose and correct this within streaming.rs only. If
confirmed, publish stop/notify under the SAME queue mutex used by the worker's
check-and-wait, release it before join, preserve existing save drain/error/result/
abandoned ownership semantics and add an actual native idle wait/shutdown handshake
regression within the existing defining group where feasible. No old test edits,
retries-to-pass, new scheduler, timeout increase or readiness inference. Exact
source/design-delta independent review must clear the correction before execution.
All prior source-specific runtime evidence remains historical until requalification.

### Original defining and retained gates

Freeze exact names at source review. Native defining groups must exercise actual
existing enqueue/worker/result mechanics with deterministic handshakes:

- Observation is borrowed/repeated/nonmutating; empty/historical-used distinction.
- Enqueued and worker-held work remain present when visible entries are forgotten
  or reload invalidates them; stale work retains its proper instance/generation.
- Queued results retain ownership until actual normal reconciliation/disposal,
  including cancelled/stale/error result paths and representative native payloads.
- Failed enqueue/send/disconnection/worker failure and checked-count exhaustion
  remain named faults until explicitly resolved; no accidental zero/readiness.

Actual-Game proof invokes ONE real server tick, records actual metadata/work and
then demonstrates unchanged capture refusal. Never count a no-op fixture or test
child without an explicit bounded plan as success. No auxiliary process is proposed
unless a concrete isolation need is source-reviewed first.

Retain current advance4, recapture4, slot6, counter6/capture6/entity8/W1seven and
180 ordinary tests; native59 names each feature mode plus new defining tests.
Run existing streaming tests and appropriate native task/drop/error regressions.
Three feature lints/fmt/exact scope/public-tree and broad qualification are required.
Production streaming/task lifetime changes require fresh ordinary BOTH-mode GUI,
not old advance GUI reuse. Root serializes all Cargo/tests and preserves every
failed attempt under a verified-unused prefix. Exact-source review before candidate
execution; different final evidence acceptance after all gates. No public push.

## Explicitly deferred

This packet neither accepts used-streamer capture nor establishes a global barrier.
Coherent capture of full-tick metadata, ordinary generic-machine activation and
host-owned durable expected WorldId are separate packets. Reading expected-world
from the candidate frame itself is forbidden. General ownership/history/scalars,
retirement/close failure, two external peers, animation/performance and G1 remain
open. No owner product decision is presently needed to review this prerequisite.
