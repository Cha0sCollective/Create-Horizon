# Dormant managed-stop worker proposal

Status: DESIGN ACCEPTED at exact proposal SHA256
000d1599f8be409b1305d916f72e26654ac8b89894801d9cb18b4afd2b6c85fc by a fresh
independent native read-only Sol High review. Exact accepted snapshot is workspace
.local/s1ms1-design-revision3.md. Earlier80aec659 was withheld for supervisor unwind
detaching a live worker; the explicit fatal guard correction resolved that P2.
October 30, 2026. This acceptance is NOT source release, ordinary lifecycle evidence
or an S1/G1 pass; the current authority final/checkpoint prerequisites still apply.
The coordinator decides technical details and obtains the required independent
review. No new owner policy decision is assumed.

## Prerequisite and bounded result

Do not release implementation source until the current authority precursor has a
fresh independent final acceptance, a clean engine commit, and its private evidence
checkpoint. Pin that resulting revision before authoring this packet. This proposal
requires a fresh independent Sol High design review and an accepted checkpoint of
its disposition before implementation is released. Exact-source independent review
still precedes execution; root alone runs validation and commits.

The useful result is a dormant, explicitly configured closed-profile worker that
owns a real server Game, real native host, actual supported counter/empty slot and
native/UUID authority leases. A failed or uncertain checkpoint stop leaves those
same owners alive and frozen. One explicit retry can later publish successfully,
retire the owners and finish. Existing ordinary launch, UI, command harness and
component callers keep their current behavior. Filled-item ingress stays disabled.

This is a worker/lifetime component, not ordinary integrated activation. It does
not establish positive-player recovery, an in-frame local owner, dedicated normal
shutdown, connected-client behavior, or G1. Those outcomes remain required by S1.

The initial fixture/session has zero participants and no active client pipes. That
is an explicit supported component profile, not a fabricated guest, synthetic
owner, substitute for positive-owner recovery, or permission to remove any S1 mode.
Use genuine new-world bootstrap and the accepted native provider; do not clear an
unsupported Game until it looks eligible. Existing package/resource configuration
and isolated coordinator fixture configuration can opt in; no CLI flag or ordinary
startup route is added in this packet.

## Why the existing lifetime cannot be reused unchanged

At the inspected authority worktree, integrated.rs stop_integrated takes Serving,
awards leave statistics, saves/kicks guests and calls lan_close before its result
returns. run_integrated returns that result. serve then retires trials on both
success and error and drops the actual Game. Keeping session_lock in an error
branch therefore does not keep any live ownership beyond the stack return.

Integrated::stop consumes its handle; halt requests stop and joins; Drop calls
halt. A retained worker would turn that implicit join into an indefinite wait.
The saving screen recognizes only thread completion, so it would show an endless
saving spinner. Its error path exits the event loop. save_for_exit retires trials
on error, and leave_server(Some(reason)) takes the handle independently of its
normal running-server guard. These facts prohibit routing this dormant component
through the existing stop/Drop/UI path.

A supervisor outside a freely droppable control handle is a prerequisite even for
this component. A new process-wide service framework is not a prerequisite: a
bounded synchronous supervisor scope is sufficient for the dormant entry.
Ordinary UI activation will separately need a holder outside the window Game.

## Exact owner and holder lifetime

Add a private synchronous entry, provisionally run_owned_managed_stop, in a small
integrated/managed_stop.rs module. It owns the worker JoinHandle, supervisor control
endpoint and status receiver as local values. It has no public owning handle with a
Drop implementation. An internal supervisor-frame abort guard described below owns
the JoinHandle solely to prevent a programmer panic/early-return from detaching it. The function remains on the calling coordinator thread until
a confirmed stopped result, or a fatal process/worker failure outside this
component's live-memory guarantee. It explicitly joins only a worker that has
reported Stopped and actually completed. Join cannot be used to wait for a
retained state.

The spawned worker calls the existing server_game factory with its own retained
resources and constructs the real closed-profile Game on that worker. It keeps
Game, native host and session together in that same lexical scope. Game is never
sent through a channel, reconstructed from status, or returned from the thread.
The resource Arc and factory inputs remain owned by the worker closure for that
entire scope. A narrowly scoped test-only setup adapter may invoke existing native
initialization methods before Ready; it is not a runtime Task queue or public Game
callback. Setup failure before Ready is an unpublished fresh candidate failure,
retaining its disk evidence; the entry accepts no previously live or filled owner.

The caller supplies a bounded supervisor driver owned by this synchronous scope.
Its concrete interface is a private FnMut(&SupervisorView, &ManagedStopControl)
returning DriverCommand: Wait, RequestStop or RetryStop with the typed attempt
fields below. It has no Exit, Cancel or Discard return variant. The supervisor
calls it between bounded status waits and retains it until Stopped. The driver
must return each callback promptly; it sees only status/control, never mutable
Game. The fixture driver has a finite command schedule and explicit successful
resolution. A driver panic is an ACTUAL process-fatal failure enforced by the private frame
guard below, not merely a diagnostic label, ordinary return or shutdown receipt. No separately droppable driver task is necessary.

The supervisor retains its own command endpoint independently of any observer or
control handles. Losing/dropping an observer/control only revokes that observer;
it neither requests shutdown nor joins nor detaches the worker. Dropped completion
observers never erase an accepted attempt. The still-owned synchronous driver sees
the retained status and can ask the supervisor for a replacement non-owning control
through its borrowed control argument. It records changed status through the
coordinator's existing diagnostic sink, including incomplete stop and refused exit,
so this is an explicit supervision loop rather than a hidden Drop wait. A caller
without this bounded status/command driver is refused before spawning. Tests drop
and replace an observer/control, then explicitly retry successfully. No timeout,
disconnected observer, close request or absent observer discards Game.

There must be no early return or question-mark propagation between worker spawn
and confirmed terminal ownership resolution in this supervisor. Startup failure is
joined only after the worker has positively reported that no Ready session was
exposed. Worker panic/status loss is a fatal diagnostic, never a stopped receipt.
Ordinary code cannot obtain or drop the supervisor's JoinHandle. This packet adds
no detach operation and no join-on-Drop behavior.

## Typed states, commands and observation

Use closed enums and bounded channels, not a general task/executor interface.
Names below describe concrete responsibilities rather than an adopted public API.

Worker status:

- Preparing: fresh unpublished setup, no session Ready acknowledgment yet.
- Ready: complete closed-profile owned Game; gameplay does not run in this packet.
- Stopping { attempt_id, epoch }: one checkpoint attempt is active.
- RetainedFailure { attempt_id, epoch, publication, detail }: the same Game/host/
  slots/leases remain owned, with NotPublished or Uncertain preserved exactly.
- Stopped { attempt_id, receipt, cleanup_diagnostic }: publication is confirmed and
  live ownership has been deliberately retired/released.

Supervisor-only status adds FatalWorkerFailure for worker panic/status loss. It is
never reported as success. The synchronous driver itself cannot ordinarily vanish
because its owned callback remains in the supervisor scope until Stopped.

Commands are RequestStop { attempt_id } from Ready and RetryStop { attempt_id,
failed_attempt_id } from RetainedFailure. Read-only status/subscription lives in
the supervisor and need not enter the worker's mutable Game scope. Permit only one
outstanding worker command. Repeated IDs, a stale failed_attempt_id and requests
received while Stopping return an explicit refusal; they do not schedule another
attempt. A bounded acknowledgment distinguishes command acceptance from checkpoint
completion. Observer timeout never cancels, retries or declares rollback.

No resume-play, force-discard, generic closure, packet dispatch or mutation command
exists. The worker does not call run_tasks, tick, tick_server(true), tick_lan,
apply_publish or wait_for_next_server_tick while waiting or retained. Paused ticking
is insufficient: it still streams, applies mod writes and processes connections;
the normal wait helper can pump pre-generation. Accepted checkpoint fences remain
the only permitted native producer-draining work during an attempt.

Each accepted command runs exactly one existing checkpoint_session invocation
under exclusive mutable Game ownership. Catch its returned error inside the worker
loop. Do not return through run_integrated/serve on either publication error, take
Serving or LanHost, invoke lan_close, retire a trial, or release a lock. Preserve
native files, partial pending directories and any promoted image as evidence.

The precursor has no live connections. Extended transport retention and bounded
network queues/liveness are requirements for the later real-player packet, not an
unproved claim of this zero-participant component.

## Epoch allocation and retry

Allocate an epoch immediately before each accepted attempt while UUID authority
is retained. Reuse the checkpoint archive's existing strict path/name/entry-bound
rules. Inspect every valid epoch-N and .pending-N name and choose checked(max+1),
starting at 1. Account for pending attempts even when they never published. Refuse
malformed entries, links, overflow, exhausted bounds or existing selected names;
never remove evidence or reuse an old pending epoch. The final primitive still
performs its own exclusive-name and monotonically increasing published checks.

No archive moving pointer, persistent reservation registry or generic journal is
needed. The UUID lease serializes cooperating managed attempts. This component
supports only the configured archive belonging to that managed session; it does
not arbitrate independently configured archives or hostile filesystem writers.

A NotPublished retry is explicit and uses a fresh epoch. An Uncertain retry also
uses a fresh epoch without inferring whether the earlier rename happened, choosing
an older epoch, deleting the prior image or replacing the still-owned Game with a
recovered copy. Existing authoritative reopen rules remain unchanged.

## Confirmed publication and teardown choice

Choose the following bounded rule: a successful CheckpointReceipt is the sole
commit boundary. After it, no simulation, item/player mutation, leave-stat update,
or new native save is allowed before retiring the supported owners and closing the
host. Do not call the existing stop_integrated as a second save/teardown operation.
That function changes players and performs a separate fallible native save.

For this closed zero-participant profile, use a small release-after-receipt path:
retire through existing native retirement helpers, release the actual host, end
Serving if one was installed, and let the complete Game/session scope release its
locks. The checkpoint already fenced the native writes. Do not write a second
native-only successor or mutate the confirmed archive. Any transport/resource
cleanup diagnostic after this boundary is reported with the confirmed receipt as
Stopped cleanup_diagnostic; it is not relabeled as a publication failure and does
not trigger another checkpoint. A panic is a fatal failure, not a receipt.

Tests must establish exactly when locks release. The supervisor reports completion
only after worker completion and explicit join. A receipt alone cannot make a
still-running worker disappear. Later positive-roster shutdown must separately
review leave-event/stat ordering and actual connection teardown; this component
does not invent an owner or silently reuse the destructive legacy path.

## Proposed source packet

After pinning the accepted authority successor, bound implementation to:

- crates/horizon-client/src/game/integrated.rs: declare the dormant managed-stop
  module and expose only the needed existing actual server factory to its child;
  existing launch, stop, halt, Drop and ordinary behavior remain unchanged.
- crates/horizon-client/src/game/integrated/managed_stop.rs: synchronous supervisor,
  private non-owning typed controls/status, closed-profile worker loop and
  release-after-receipt path. Keep Game and its resources in the worker scope.
- crates/horizon-client/src/game/integrated/managed_stop_tests.rs: deterministic
  supervisor controls and isolated actual-Game native-resource cases.
- crates/horizon-client/src/game/levels/session_checkpoint.rs: only the needed
  strict next-unused-epoch helper and narrow cfg(test) publication-phase fault
  adapter around the existing primitive. No weakening of snapshot, ownership,
  provider, identity, native coverage, player, byte or publication checks.
The source scope is these FOUR paths only. Epoch-name/overflow/link tests belong in
the new managed_stop_tests.rs and call the narrowly exposed helper. Existing
session_checkpoint_tests.rs remains unchanged.

Do not modify saving.rs, game.rs exit handlers, remote.rs, control.rs, normal
Integrated::Drop, ordinary configuration parsing, authority acquisition semantics,
slot transfer/admission or guest staging in this packet. No new public engine API,
standalone supervisor service, persistence format or UI framework is proposed.

A later ordinary integration will need nonconsuming managed-stop status in the save
screen, an explicit retry/status allowlist, rejection of ordinary quit while
retained, and a supervisor outside window Game. Its inventory includes
poll_closing_save, begin_closing_save, leave_server(Some(reason)), save_for_exit,
launch connect_local errors and outer event-loop returns. That later work must not
route a managed handle into legacy stop(self)/Drop. It is deliberately inactive here.

## Meaningful finite validation

Use fresh isolated roots, pinned native resources and the existing prebuilt package
configuration. Root runs all validation only after exact-source review. Keep every
failed attempt and raw artifact ignored. No pristine-clearing or synthetic owner
fixture can make a failed profile pass.

Required actual-resource cases:

1. Create a real identified TestTerrain Game through existing managed preparation,
   install the actual supported counter and empty persistent slot through genuine
   new-world eligibility, and create a real closed native host with zero guests.
   Retain resource/factory provenance, native root, archive and authority namespace.
   Show Ready owns the actual objects; no fake Game or owner is introduced.
2. Complete a baseline checkpoint, then inject a deterministic error at an existing
   pre-promotion checkpoint phase after real pending bytes exist. Receive the
   primitive's NotPublished classification. Observe the same world/host identity,
   counter/slot keys and projections, unchanged game clocks and retained native and
   UUID lock exclusion. Do not obtain these observations through a mutable Game
   Task; use narrowly authored immutable test observations at stable states.
3. Explicitly retry with a fresh attempt ID and epoch beyond the retained pending
   name. Remove only the one-shot test fault, not disk evidence or Game state. The
   real checkpoint succeeds; verify the receipt/archive with existing checks,
   worker completion and join, then verify both native and UUID leases release.
4. Inject a deterministic error at the existing Promoted phase after real rename
   succeeds and before parent-directory sync. Observe Uncertain, the retained
   actual Game/host/slot/leases and the promoted directory. Explicitly retry at the
   next unused epoch and reach confirmed completion. Keep the first image intact.
5. Drop a command observer/control while retained, prove the supervisor still owns
   a responsive worker and excludes alternate authority/native opens, obtain a
   replacement control, then explicitly retry successfully. Every test has a
   bounded observation deadline and a predetermined success resolution; no test
   relies on an indefinite Drop join or detaches a worker on an ordinary path.
6. Prove repeated/stale/in-flight requests do not cause extra attempts; overflow,
   malformed archive entries and occupied pending names refuse without cleanup.
   Confirm no ordinary task closure, tick, packet or publish callback runs frozen.

The injected failures are post-phase callback errors, not assertions that the OS
returned a rename error or that disk/power-loss durability was demonstrated.
Promoted-phase injection proves actual promoted bytes plus the primitive's
Uncertain handling; it does not reproduce a failing rename with unknown outcome.
Document those bounds. Unit-level channel mocks may supplement these cases, never
replace the actual Game/host/slot/lease failure-and-successful-retry cases.

Assertions must be collected so the fixture performs its explicit finite resolution
before reporting an ordinary test failure. An unexpected worker panic or inability
to resolve within the authored diagnostic bound is a fatal fixture failure with
preserved evidence, not permission for production to force-discard. Root must
review the precise test failure cleanup before execution; do not add a hidden
Drop path that clears owners just to prevent a test timeout.

## Exception policy and remaining technical decisions

Normal requested stop/close while retained is rejected as incomplete; only explicit
retry/status is supported. This requires no new force-discard policy. Killing or
crashing the process, power loss and unrecoverable runtime failure cannot preserve
live memory. Subsequent startup uses the existing newest complete checkpoint rules;
partial/pending artifacts remain evidence, and a corrupt complete successor refuses
rather than falling back silently. No live-memory-survival claim covers those cases.

Before source authoring, root must settle three engineering details in the reviewed
candidate: the concrete finite coordinator driver implementing the bounded
FnMut status/control contract; the minimal real closed-profile startup helper and
immutable state observations; and whether the archive helper/fault adapter needs
the optional checkpoint test file. None is an owner-policy question. If these cannot
be made concrete without a freely droppable owner, arbitrary frozen Game callbacks,
a synthetic player, or an unresolved test worker, stop and narrow/review the design
instead of claiming the lifetime problem is solved.


## Coordinator engineering disposition before review

Use the synchronous supervisor scope above. The exact finite resource-test driver
is an owned closure matching the typed status and a small local case state: wait
for Ready, issue exactly one RequestStop, wait for the configured RetainedFailure,
record immutable observations and observer loss/replacement, issue exactly one
RetryStop against that failed attempt, then wait for Stopped. Other stale/repeated
commands are additional predetermined schedule entries. The closure collects
assertion discrepancies instead of panicking; report them after explicit successful
resolution and join. No driver has a Game reference or a generic worker closure.

All production worker construction remains concrete: clone the existing server
factory inputs into the spawned closure, set the actual DedicatedServer execution
role (no local player), call server_game, start the configured packages, prepare a
genuinely new identified native TestTerrain world with with_session_authority,
finish_open_world, and establish real native residency. Resolve the one configured
counter type through the actual catalog/binding, install its block/counter and empty
persistent slot through existing new-world bootstrap. Start a real closed LanHost
through lan_host with listen=false, no reserved local owner and zero guests; do not
substitute a struct that asserts a fake profile. Retain the actual inert native UUID
and satisfy current native metadata checks. Setup errors remain unpublished failures.

Immutable test observations are cfg(test) snapshots authored at Ready/RetainedFailure:
world UUID/root, actual counter/slot keys and projected records, clock, actual host
identity/census, plus separately attempted real native/authority acquisitions from
the supervisor side. No mutable test Task enters a frozen Game. One-shot cfg(test)
phase faults are finite configuration carried into the worker, not callable runtime
closures; they leave pending/promoted bytes and are removed only by exhausting the
specified first attempt. Capture/install profiles and current checkpoint assertions
remain unchanged. All four paths and this concrete factory/observation seam need
exact-source review before execution.

Strengthen fatal handling: disconnected status is NOT permission to return while
the worker is live. The sole status sender stays in the worker lexical scope until
its terminal return. If disconnection is observed, retain the JoinHandle and scope,
report fatal status, and wait for actual thread completion before joining/reporting
fatal failure. A genuinely live unresponsive worker remains supervised and never
detaches on an ordinary return. Unexpected fatal fixture timeout/process termination
is a failed test with preserved disk/log evidence, not a successful stop or a
production force-discard operation. No early-return propagation bypasses this rule.

These coordinator technical choices are part of the independently accepted design.
The four-path source release still waits for the current authority final checkpoint
and an independent design acceptance at an exact frozen proposal hash.

## Fatal supervisor unwind correction

Initial proposal80aec659 was WITHHELD for a concrete P2: ordinary Rust unwind from
the FnMut callback or a diagnostic/assertion panic drops a bare JoinHandle and
detaches the live retained worker, even inside the test harness. The production
profiles do not globally abort on panic. Calling that event fatal was insufficient.

Choose a private, supervisor-frame-only AbortUnresolvedWorker guard. It takes
ownership of the JoinHandle immediately after spawn and is never returned or
cloned. While its worker can still be live, unwinding or any early return through
this frame prints a minimal best-effort fatal diagnostic and calls
std::process::abort BEFORE the JoinHandle can drop. The guard does not join, detach,
retry, mutate Game, discard owners through a normal command, or run arbitrary
callbacks from Drop. Its guard path uses no expect/unwrap or fallible logging that
could panic before abort; write a static stderr message best-effort, then abort.
Only the supervisor's explicit checked terminal join consumes/disarms the guard.
No ordinary error/status/timeout return can disarm it while the worker is live.
A positively finished worker may be joined for either confirmed stop, unpublished
setup failure or fatal worker failure; each outcome retains its distinct meaning.

This is an explicit process-fatal invariant violation for the already excluded
unexpected supervisor panic/programmer error case, not a new ordinary forced-exit
or force-discard feature and not an invisible Drop join. An ordinary failed save
still remains live, visible and retryable under the supervisor. Forced abort cannot
preserve live memory; recovery uses the last complete archive with existing strict
newest-image validation. The future UI must not use panic to implement cancellation.
The dormant entry is unreachable from current ordinary callers.

Root must validate this exception separately in a freshly isolated child invocation
of the already built resource-test executable. An ignored dedicated case constructs
the actual worker and baseline archive, intentionally panics in the supervisor driver
while Ready/RetainedFailure, and MUST terminate that child with the expected abort
signal/fatal marker. Root is the only subprocess orchestrator; no Rust test spawns a
compiler, Python, Java or another test process. The child is expected to fail fatally,
not yield a libtest passing count or a Stopped receipt. Root confirms no surviving
worker process, records preserved baseline/pending artifacts and the explicit fatal
outcome, and keeps that result separate from successful finite retry cases. Disable
core dumps only in that isolated child shell to avoid a large core artifact. Exact
source and proof-tool review must cover signal/status matching and refusal of a
vacuous success. Do not weaken ordinary test assertions or treat timeout as abort.

Clarify the baseline test: create its first baseline image during unpublished setup,
before Ready and outside a stop attempt. A successful STOP receipt alone triggers
receipt-only teardown; the earlier setup baseline does not close its new session.
This makes the finite failure/retry cases coherent without a second live Game or
releasing/reconstructing any current owner.

## Startup route clarification

The concrete new-session factory knows its namespace, new world and archive roots.
Preflight their complete pairwise separation and all existing path components BEFORE
any native/authority lock acquisition or new-world mutation. Do not rely on
WorldRequest::check_new_world alone for archive separation: that existing precursor
entry knows only namespace and world, while its trusted configuration precondition
also requires the archive to be elsewhere. A misselected namespace inside an existing
archive must not add authority keys to that archive. The managed worker must seal
this configuration before starting the new Game.

This packet starts a genuinely new identity only: require an absent new-world root
with a checked existing real parent, an existing real EMPTY archive root and the
existing real namespace root, all nonoverlapping and without linked/reparse path
components. No cleanup or reuse of prior failed roots. After this read-only route
check, existing managed preparation validates/acquires its native and UUID locks.
Create the pre-Ready baseline in that same new archive after actual setup. Existing
archive recovery is not a startup mode for this stop-worker precursor. Add a focused
wrong-root regression that leaves an existing archive's names and bytes unchanged.
Implement this bounded factory preflight in managed_stop.rs; the FOUR source paths
stay unchanged and authority acquisition semantics are not modified.

DELTA DESIGN ACCEPTED by the same independent read-only reviewer at exact input
SHA2561bcefe0228665d8d845fcf34a9c163cac22a0b9d96d4acd9d720176d377e0139;
snapshot .local/s1ms1-startup-route-clarification.md. This does not release source
or change the current authority packet's already reviewed implementation.
