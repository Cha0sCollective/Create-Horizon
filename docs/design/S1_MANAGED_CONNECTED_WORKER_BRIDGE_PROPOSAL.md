# Managed connected-worker bridge proposal

Status: PROPOSED. Connected c11b8b2f: SOURCE CLEAR, validating; no FINAL/source release/runtime/G1 claim.
Start no bridge source until connected recovery has independent FINAL acceptance,
a clean engine commit and private evidence checkpoint. Pin that successor; obtain
fresh independent Sol High acceptance of this exact design before source authoring.

## Bounded outcome

Run the existing recover_connected coordinator wholly on a managed worker, with
its actual two-client memory recovery, native Game/Guest/GuestPlay and leases still
owned by its existing OwnedRecovery frame. The calling synchronous supervisor owns
the worker JoinHandle and drives two real headless client Games from owned pipe
offers. It can request coherent close/retry without consuming the worker handle.
No Game, host, mutable callback, transport shutdown capability or raw task escapes.
Keep gameplay, counter stepping and machine-slot ingress disabled. ReadyFrozen is
not permission to play, mutate or drop owners. Late departure remains retained/
CleanupBlocked; this packet adds no reconnect, forced release or offline owner.

Only the fixed MemoryOwner/MemoryGuest offline profiles and actual memory protocol
are supported. Matching offline names/UUIDs is not cryptographic authentication,
external package admission or proof for TCP, a different roster/single-owner mode,
ordinary singleplayer, in-frame operation or all S1. UI/process-holder activation
is the next separate packet; no normal launch/close caller is introduced here.

## Actual seam and proposed private API

Current integrated/managed_stop.rs has run_owned_managed_stop, owned_worker,
ManagedStart, ManagedStopControl::request/view, SupervisorView, DriverCommand,
WorkerEnd and AbortUnresolvedWorker. Its factory is new_closed_game and cannot
construct a recovered positive roster. Current lan/connected_recovery.rs has private
recover_connected(Box<Game>, &AuthorityNamespace, ManagedRecoveryRequest,
&continuation, timeout, FnMut(Observation, Option<[Pipe; 2]>) -> Command).
It returns End::Closed(receipt) or End::Refused(reason); OwnedRecovery never escapes.
Preserve that ownership and the old closed-profile entry/guards.

Add a private ManagedConnectedStart carrying existing factory resources/pack/
options/settings plus namespace root, ManagedRecoveryRequest, separate continuation
root and bounded timeout. Add a private run_owned_managed_connected entry whose
driver receives (&SupervisorView, &ManagedStopControl, Option<[Pipe; 2]>).
The driver sees only immutable status, non-owning typed controls and one owned pipe
bundle; no references to the worker's Game or lifetime owner. A narrowly factored
supervisor loop may dispatch exactly NewClosed or Connected start variants; retain
the old run_owned_managed_stop signature as its unchanged closed-profile wrapper.
Do not build a generic worker factory or Task executor.

The connected worker retains its start/resources for its entire lexical scope,
creates a fresh boxed headless candidate through existing server_game, starts/
checks the actual configured packages and calls recover_connected there. Use its
required nonclient native role and existing fresh-candidate constraints; do not
call new_closed_game, create another identity, attach a fabricated host or clear
candidate state to pass. Existing namespace/path/preparation checks stay inside
the connected entry and run before its one-shot pipes are offered.

Expose the connected module/function and only needed typed observation/command/end
fields with pub(in crate::game), not a new public engine API. Mapping remains in
the worker: RequestStop -> Close; RetryStop -> RetryClose; otherwise Wait.
The coordinator revalidates actual state/attempt IDs and owns checkpoint selection,
publication classification, real native close/joins and receipt-only retirement.

## Channels, admission and status

Reuse the one-slot command channel, nonconsuming ManagedStopControl and monotonic
attempt admission. Extend the view with an explicit connected phase: Joining,
Admitted, Loading, ReadyFrozen, Refusing, CleanupBlocked or Closing. Do not label
transport readiness as successful stop. Permit initial close only at ReadyFrozen;
retry only at the matching RetainedFailure. One accepted request causes one attempt.
Existing zero/stale/repeated/in-flight refusals remain. Close admission stays
in-flight until the coordinator classifies that exact attempt: publishing an older
ReadyFrozen observation must not overwrite Stopping and admit a second request.

One separate sync_channel(1) carries the single [Pipe; 2] offer. Move the bundle
once; never clone it into shared status, replay it to replacement observers, or
wait synchronously for clients on the worker. The supervisor delivers it at most
once to its owned driver, which constructs/polls the two actual client Games.
If handoff fails before delivery, drop the returned owned endpoints and continue
the coordinator through its existing pre-Ready refusal/native cleanup; never
return early with OwnedRecovery or its connection threads unresolved. A duplicated
bundle is an invariant failure, not an extra connection offer.

Shared status contains bounded scalar phase/attempt/publication, clocks, guest/
GuestPlay counts, transport counters and at most256 UTF-8 diagnostic bytes. Reuse
existing immutable observations to derive it. Full player/frame test observations
may be cfg(test), bounded by existing checkpoint limits and coalesced to one latest
view. Never enqueue an observation backlog or expose a mutable Game test closure.

The coordinator's FnMut callback on the worker forwards/coalesces current status,
forwards the one-shot bundle nonblockingly and consumes at most one typed command.
It performs no client polling, world tick or arbitrary driver execution. The
supervisor's driver stays on the caller thread and returns promptly between bounded
status waits. Sender/observer loss cannot cancel, retry, detach or discard owners;
the supervisor retains its own control sender and offers no Exit/Discard command.

## Completion and exceptional lifetime

Keep AbortUnresolvedWorker immediately around the spawned JoinHandle. Reuse its
fatal-unresolved/no-join-in-Drop discipline and WorkerEnd where possible: after
recover_connected returns Closed, publish matching Stopped(attempt, receipt) and
return WorkerEnd::Stopped; after Refused, publish startup refusal and return a
non-success startup result. Preserve the distinct refusal meaning in status/logs.
Errors before transport exposure remain unpublished startup failures. Once the
coordinator owns live parts, no question-mark return bypasses its cleanup scope.

Only is_finished permits explicit removal/join, followed by matching terminal
status/receipt validation. Closing(receipt), sender disconnect, deadline or an
observer timeout is not finished/join authorization. A post-Ready departure still
retains Guest AND GuestPlay/leases and refuses normal release under the coordinator;
this bridge must not reinterpret CleanupBlocked as startup refusal or success.
Supervisor unwind with a live worker is process-fatal as already accepted. Fatal
worker loss is never a stop receipt. No detach, hidden join or new policy is added.

## Source scope and meaningful proof

Propose SIX paths only, against the subsequently pinned FINAL connected base:
1. game/integrated/managed_stop.rs: connected start/dispatch, bounded offer/status
   bridge and focused common supervisor/control reuse; keep legacy factory behavior.
2. game/integrated/managed_connected_tests.rs: new actual-worker/two-client tests.
3. game/lan.rs: narrowly expose the connected module to the sibling worker.
4. game/lan/connected_recovery.rs: only required crate-game visibility and a narrow
   cfg(test) one-shot close-fault setter executed on the worker, not a Game callback.
5. game/lan/connected_recovery_tests.rs: narrowly expose/reuse the existing actual
   source fixture/owned receipts for worker tests; preserve its lifetime guards.
6. game/integrated/managed_stop_tests.rs: regression/helper adjustments only if the
   shared view/control extension requires them; include in the frozen source scope.
No game.rs/saving.rs/remote.rs/control.rs, ordinary UI, protocol, save schema or
network-policy edits. Revisit design if another production path becomes necessary.

Test a real worker and two ordinary client Games: restore actual stone3/stone5
player inventories, actual terrain/teleport/PlayerLoaded, frozen clocks and empty
machine slot; close through typed control, validate the continuation and join,
then prove native/UUID exclusion releases. Run both existing one-shot publication
faults ON THE WORKER: retain real keys/rows/GuestPlay and leases, drop/replace a
non-owning control observer, explicitly retry with a fresh attempt/epoch and finish.
Check stale/repeated/in-flight requests and one-shot offer delivery. Include one
finite pre-Ready refusal proving all native connection attempts finish before the
worker/supervisor returns. Collect discrepancies; resolve explicitly before asserts.
A separate isolated child proves supervisor unwind with the actual connected
worker is fatal; late departure remains a distinct retained exception observation,
not a finite successful close. Reuse source fixtures, never fake Guest/readiness.

Root preserves ALL six scoped sources and ALL packet-prefix evidence before every
correction, freezes exact hashes, obtains independent exact-source Sol High review
before compiling/executing, and serializes all Cargo/tests/game processes. Use
fresh isolated data/config/temp roots and pinned resources/prebuilt packages; no
playable saves, accounts, reference changes, public push or release. A different
final reviewer assesses positive and exception evidence. No new MSRV, compatibility
or performance promise. Stop at FINAL/local commit/private checkpoint of this
bridge; ordinary outer UI/process holder and later Running/departure adapters need
separate accepted packets. Stop earlier for a genuine technical blocker; do not
weaken ownership, add a general framework or substitute another zero-roster proof.

Independent design review: DESIGN ACCEPT at exact proposal2 input
026d3415869a27f7b1726f16b1724adac1d51cc1a6a0749cd680ab1e46432f42.
All stated connected-FINAL/private-checkpoint/successor-pin prerequisites remain.
