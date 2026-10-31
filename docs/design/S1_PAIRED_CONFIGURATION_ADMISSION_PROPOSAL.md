# Paired configuration and admission proposal

Status: PROPOSED full component, not source authority or an S1/G1 decision.

## Prerequisites and concrete outcome

Research uses the active login-negotiation eight-path candidate in
.local/worktrees/s1-login-negotiation, frozen source manifest b8bd7497 (SOURCE
and tool CLEAR; build active, no FINAL at authoring). Its base is clean frozen
window173ad2a7b496b11b15e1492ad57c4ce4da9cf21d. Persistent retirement has a
separate accepted FINAL ab239890.../treecf2c3c55... reported by the coordinator;
its private checkpoint is pending. These abbreviated research identifiers are
not an implementation pin. Require login packet B's actual accepted clean exact
FINAL pin, private durable checkpoint, a fresh full seam/constructor inventory,
and independent Sol High acceptance of this exact design before source work.
Record any retirement integration and exact parent/tree explicitly. Freeze and
independently review source before runtime; obtain a DIFFERENT FINAL afterward.

This packet replaces the paired startup refusal ONLY for a complete retained-input
configuration/admission route. Real selected paired packages must join through
normal integrated memory owner, published integrated LAN and dedicated TCP with
two independently captured exact clients. Include the promised ordinary in-frame
host comparison. No additional diagnostic-only primitive counts as completion.
Keep B's explicit login-only diagnostic constructors operational and refusing
configuration; default vanilla constructors and behavior stay unchanged.

No new packet ID/channel, WireContract change, machine codec, H10 replication,
journal, save format or general protocol framework. Compatibility is not hostile
binary attestation. Online account authentication remains native; offline UUIDs
remain offline identities. Full S1/G1, lifecycle/Running, registered binding and
committed animation remain open until their combined evidence and explicit gate.

## Actual seams and route selection

Source locations are candidate-relative and must be refreshed at the clean pin.

| Seam | Required change |
| --- | --- |
| lan/login_negotiation.rs:161 NegotiationConnection | Retain its actual Live allocation and unsplit connection through configuration, then transfer ownership explicitly. Current Drop closes it. |
| lan/login_negotiation.rs:284 AgreementReceipt | Current consume returns transcript data; data alone must not construct the new configured receipt. |
| lan/conn.rs:898 required branch | Currently sends CONFIGURATION_DISABLED and returns before try_admit. Add a distinct configured policy branch. |
| lan/conn.rs:942 Parts; :950 login_and_configure | Carry concrete configured handoff alongside existing fields; full paired configuration has its own finite strict state sequence. |
| lan/conn.rs:1054 registry_packets(known_pack) | Paired route MUST call registry_packets(false) from the exact retained SyncData. |
| net.rs:694 configure | Currently mutates Dynamic immediately, ignores features/tags and permits unrelated packets. Keep this default route; add strict paired configuration. |
| horizon-protocol/src/registry.rs:94 Dynamic::add | HashMap insertion overwrites duplicate groups. Validation must precede every add and publication. No change to this general method. |
| lan/mod.rs:509 try_admit; :559 release | UUID-only reservation can evict/release another connection. Paired route uses owner-matched concrete reservations instead. |
| lan/hub.rs:684 admit; :1181 poll | Receipt consumption must precede next_entity mutation and Guest::new. Staged poll at1314 needs the same paired gate. |
| game/remote.rs:865 Event::Joined | Consume exact live configured receipt before set_registries, joined/profile publication or processing dependent play packets. |
| game/lan.rs:2501 build_detached_lan_host | Capture actual final SyncData/static registry and retain FrozenHostInputs in the resulting LanHost. |
| game/remote.rs:344 join_server | Capture independently from actual local packages/resources/assets and retain in Remote with the actual Session. |
| game/integrated.rs:369/:386 | Initial listen=false host must already require paired policy before the owner memory pipe or later listen_on. |
| game/integrated.rs:614 publish; game/lan.rs:2446 lan_start | Replace blanket refusal only when the existing retained configured policy is installed and matches actual owners. |
| game.rs:15467 early server/dedicated guard | Select the reviewed pending paired launch route BEFORE resource materialization; do not merely delete this guard or call a successful data comparison permission. |
| game/lan/connected_recovery.rs:405-414 | Existing staged memory recovery injects Lan::start_staged through the detached builder; propagate the same captured configured policy here. |

Add a narrow package preflight method in mod_packages.rs, for example
preflight_paired_network_launch(role), that checks selected prepared pair/profile,
feature availability and supported launch role before materialization. It returns
no connection/world authority and does not create FrozenHostInputs prematurely.
The early game.rs guard branches on actual selected packages: ordinary path still
calls external_network; paired path must pass this explicit preflight and later
construct the configured route. Leave Packages::external_network itself refusing
paired packages so forgotten consumers fail closed. There is no Boolean global
switch saying paired networking is safe.

At actual host construction, load paired SyncData afresh from the FINAL active
resource root. Do not reuse the unkeyed game::lan::SYNC cache from a different
resource root. Use that SAME Arc for encoder, Hub, Lan and freeze_peer_inputs with
pack.assets.registry, then the holder's cached expectation. Server capture must
precede any listener/pipe acceptance. Store the original non-Clone FrozenHostInputs
in LanHost, plus only narrow expectation access; no into_parts or claims constructor.
The client independently loads actual local SyncData and captures against its
actual static assets/packages before starting its Session. Remote's Builtin is
cloned from that retained SyncData.builtin (Builtin is Clone), not reread from an
unrelated report after capture. Remote owns the original FrozenHostInputs and
actual Session for their whole connection, including queued installation.

Keep policy private to Shared, not NetConfig. Add a distinct RequiredPaired policy
alongside B's RequiredNegotiation/login-only policy. Proposed explicit constructors:
Lan::start_paired(sync, net, required, reserved_host),
Lan::start_staged_paired(sync, net, profiles, required),
Session::connect_paired(address, name, auth, view, lang, required), and
Session::connect_local_paired(pipe, name, view, lang, required).
RequiredPaired is created from the retained expectation and bounded native budget,
never decoded peer bytes. Constructor checks exact SyncData owner identity.
Normal binary connect_local/join_server choose it when packages are paired;
vanilla/default library constructors remain unchanged. No paired branch may
silently fall back to ordinary Joined, local preconfiguration or transfer.

## Same protocol for the memory owner

Choose the full negotiation AND configuration path over native Pipe. This is a
concrete routine option: serve_local already calls serve_connection with
Connection::local, Session::connect_local already sends handshake/login, and B's
recv_until/send_until already implement memory read timeouts and immediate pipe
writes. No new memory channel/transport file is required; extend the existing
connection.rs close capability as described below. Memory buffers remain unbounded;
no total memory, scheduler or native allocation-latency claim is introduced.

The integrated host installs RequiredPaired while listen=false, then offers its
memory pipe. Owner profile gate/ops/placement remain native. Later publish checks
that LanHost still retains the exact holder/expectation/policy before listen_on;
no policy replacement or uncaptured listener is allowed. In-frame hosting retains
its native local player as world owner; that local player is not a synthetic
network Guest. Every external peer still uses configured admission. This is the
ordinary in-frame comparison, not a headless replacement for it.

Do not break or bypass existing frozen/staged memory recovery. Extend its detached
builder callback to receive the freshly captured optional RequiredPaired, and use
start_staged_paired for selected pairs. Keep its existing bounded roster/mailbox/
placement/cleanup checks AS ADDITIONAL constraints. Its paired PairedJoined also
requires configured receipt consumption before staged Guest creation. Existing
managed application's client.connect_local caller can stay unchanged because
that Game method now chooses the real paired Session constructor. Update genuine
connected-recovery fixture producers to capture actual expectations; do not forge
receipts or mark an ordinary memory pipe trusted through a cookie/Boolean.

## Exact finite paired configuration

Add lan/paired_configuration.rs as a concrete login/configuration service. Keep
B's challenge, UUID/nonces/transcript bytes, channel and unchanged WireContract
v1. Use B's SAME NegotiationConnection allocation and fixed total deadline from
first required login wait through configuration completion and queued admission.
Never renew budget on phase transition, heartbeat, partial frame or handoff.
MAX_BUDGET remains120seconds. This is an admission deadline; after successful
one-shot consumption an established live play connection has no120second expiry.

ExpectedConfiguration is derived only from the expectation's actual retained
SyncData after the existing captured borrowed-budget checks, not a remote digest.
Its finite ordered native packet projection contains exact enabled features,
every registry group in registry_packets(false) order, every entry in order with
Some(full NBT), and exact tags including group/tag/member ordering. Cache bounded
expected native encoded packet bodies once per expectation if useful. Reuse the
existing native encoders, bounded table/NBT preflight and SHA256 only for existing
identity projections. No additional wire schema or new configuration digest is
needed. Configuration order is stricter than H09's canonical unordered-set hash:
equal digest claims alone do not authorize an alternate wire ordering.

Use this paired-only sequence after agreement, leaving default configure untouched:

1. Server obtains a concrete paired pending reservation after native gate/limit
   checks, then sends at most one native Compression packet for TCP if configured;
   memory remains uncompressed. LoginFinished follows with the negotiated exact
   profile UUID/name. Client rejects wrong identity, repeated compression/finish,
   unexpected authentication restart, cookie or unrelated login query. Its native
   LoginAcknowledged is exact and occurs once. Compression is native pipeline
   configuration, not a replacement for input agreement.
2. Client sends one bounded native ClientInformation and exact existing horizon
   brand payload. Server checks those expected forms once. No permissive ignore
   loop accepts arbitrary packets. Both ends remain unsplit and use total-deadline
   send/recv. Native view/locale settings remain client settings, not package input.
3. Server sends native brand payload, exact retained UpdateEnabledFeatures,
   SelectKnownPacks with the existing core pack descriptor. Client validates the
   descriptor and answers EMPTY known packs once. Server requires the empty answer,
   regardless of actual installed core pack. It may then send the existing empty
   horizon quiet-capability marker in its fixed position because the native paired
   client sent horizon brand; validate it exactly and defer its publication.
4. Server emits EVERY sync.registry_packets(false) packet and one exact UpdateTags.
   Never choose known_pack=true, infer NBT from agreement or elide zero-entry groups.
5. Server sends native FinishConfiguration exactly once. Client accepts it only
   after ALL expected groups/entries/full NBT/tags/features and required prefix
   packets are complete, then sends exactly one FinishConfiguration acknowledgment.
   Server accepts only that exact completion after its full emission and expected
   client prefix. No Parts or configured receipt exists before this point.

For this first complete paired profile, refuse server resource-pack offers,
server-links configuration and other interactive configuration tasks at paired
host construction before listening. Default empty/None settings succeed. Reject
cookies/store-cookie, dialogs, code-of-conduct, transfer, unexpected custom payload,
registry reset/reconfiguration, unknown and out-of-order packets in the paired
exchange. No user prompt or extra external resource dependency is inserted.
B login-only diagnostics still deliberately Disconnect after agreement instead.

Client validation uses an ordered index against the actual local expected native
packet bytes BEFORE calling Dynamic::add. Compare actual packet ID and complete
uncompressed payload bytes against the native expected body; this catches duplicate
NBT keys, reordered fields, absent data and trailing bytes which tolerant decoding
or HashMap overwrite could hide. Also check explicit group/entry cardinality,
uniqueness, full Some(data), and native semantic projection while constructing
the private checked result. Do not decode then re-encode the peer as the only
raw validation. Different native canonical byte order must be tested using actual
independent captures; refuse unsupported noncanonical order rather than silently
normalizing it. Stage validated packets privately, add them to a PRIVATE Dynamic
only after the entire sequence is complete, and publish no Dynamic/registry
Event before a configured receipt is ready. Even a private intermediate Dynamic
must not mask duplicate groups. Preserve existing frame/decompression bounds;
this packet adds finite expected sequence bounds, not a general precodec-memory
claim. Check deadline after final completion and immediately before consumption.

## Connection, receipts and handoff

AgreementReceipt must be consumed by the concrete configuration transition on
its SAME scope and retained expectation, not reduced to public transcript data
and then treated as authority. Only that successful configuration helper can
create opaque non-Clone/non-deserializable ConfiguredReceipt. Private fields bind
endpoint role, exact expectation Arc, profile UUID, negotiation session/transcript,
actual Live allocation, completed configuration and original admission deadline.
No Default, claims/hash constructor, public valid Boolean or transferable cookie.

Extend NegotiationConnection with explicit state progression
Fresh -> Negotiating -> Agreed -> Configuring -> Configured -> HandedOff;
any failure -> Closed. HandedOff is a move of its concrete connection AND original
Live allocation, not a new liveness token. Store the Connection in an Option so
handoff can take it without a Drop closing the transferred connection. Before
handoff Drop still closes; after handoff concrete reader/writer owners take over.
Do not expose a mutable Connection escape hatch. Add only needed native send/recv,
compression and final split operations within this state machine.

PairedPlayReader/PairedPlayWriter retain the original Live allocation around
actual ConnReader/ConnWriter. I/O error, EOF, explicit shutdown, callback unwind,
writer/reader return or destruction, spawn failure, cancellation, transfer or
reconfiguration terminally invalidates it BEFORE queuing terminal events. Both
halves' guards close the actual transport. Ordinary reader/writer paths are
unchanged; private endpoint enums may delegate existing methods rather than turn
this into a general transport trait framework. Client Session writer storage must
retain the paired writer wrapper, so Session::send failure cannot bypass liveness.
Server writer thread likewise uses the guarded writer. Do not hand out a raw
writer that can silently fail while a queued configured receipt stays live.

Add a narrow close-only ConnectionShutdown in protocol connection.rs, covering
TCP and existing PipeShutdown, obtainable while unsplit without read/write access.
TCP needs a duplicated native socket solely for Shutdown::Both; it must be
explicitly shut down by its owner on cancellation and then dropped, since that
descriptor otherwise retains a native endpoint. Memory close handle must not
retain a Pipe End. Store it with the actual in-progress Session/Lan attempt so
local close interrupts partial login/config frames immediately; an atomic latch
alone would otherwise leave disconnect joining until the admission deadline.
This is an honest extra protocol path, not an invented side transport service.

ConfiguredReceipt consumption must linearize with terminal invalidation using
one private short-lived native synchronization gate, not a check-then-act AtomicBool.
A server configured join carries its real guard and reservation; a client one
carries the actual Session identity and checked Dynamic. begin-consume validates
same live allocation/role/profile/expectation, completion, deadline, endpoint
session/connection identity and unconsumed state, then spends the receipt exactly
once into a specific admission guard. Terminal invalidation racing this operation
wins before consumption or takes effect after the local admission commit. No
arbitrary user callback runs while the gate is held. Poison/unwind closes; no
valid receipt is reconstructed. Observed terminal state, not unknowable instant
remote socket death, is the linearization boundary; tests must establish the
terminal latch before asserting a queued event is stale.

Hub preflights entity-ID checked_add/i32 range, duplicate conn/profile, vector
capacity and host expectation before consume. AFTER consume, construct the real
Guest with the prospective ID, initialize native permissions/dimension, then
commit next_entity and guest publication. Guest construction may allocate/panic;
its unwinding admission guard invalidates/closes and releases only its own ticket,
and the ID head is not committed first. Never call legacy admit first and validate
later. No Guest/world authority or HostEvent::Joined on failed consume. Client
preflights retained Remote/Session identity and checked registries, consumes before
set_registries, then marks joined/profile and releases staged metadata/play events.
Keep client world/registry install tied to its actual Session, not address/name.

Add explicit envelopes instead of fields on legacy Joined:
NetEvent::PairedJoined(Box<PairedJoined>) and
net::Event::PairedJoined(Box<PairedClientJoined>), both with private payload access
and actual configured receipt. Existing Joined structures/literals remain intact.
Hub and staged Hub reject unqualified NetEvent::Joined whenever a paired policy
is required. Remote rejects ordinary Event::Joined/Reconfigured when it owns paired
inputs, and rejects a PairedJoined in an unpaired/mismatched Session. No old queued
join may install after a replacement Remote has equal UUID/address/contract bytes.
Guard dependent play packets until that endpoint's admission succeeds; discard
queued failed-attempt packets and close. Reject paired StartConfiguration and
Transfer in the actual play reader BEFORE emitting a usable reconfiguration/
transfer event or clearing/republishing client registries. A fresh full connection
is required for reconnect; no transfer-cookie shortcut or receipt reuse.

## Reservation and concrete cleanup ownership

Introduce private PairedReservation in Shared, non-Clone/nonserializable, with a
fresh native owner allocation and exact UUID/attempt binding. After agreement,
under one paired reservation-table lock apply gate/limits and atomically reserve
capacity/profile; count pending AND admitted peers. A duplicate pending/active
UUID refuses promptly without NetEvent::Duplicate, kick or waiting for replacement.
Check in-frame reserved host UUID before admission; supply reserved_host at paired
Lan construction BEFORE listen so there is no accept-before-reserve window.
Integrated owner is admitted through its actual memory connection; it is not put
in the in-frame reserved-host set. Allocate paired connection IDs with checked
exhaustion, never wrapping Shared::next_id into another live connection identity.
This connection ID is distinct from the Game entity head.

Promote the ticket only at successful Hub consume; terminal guard keeps it until
actual transport termination. Owner-matched release removes only the same native
reservation entry (Arc identity plus UUID), so a delayed old thread cannot release
a reconnected player's slot. No paired branch calls UUID-only try_admit/release.
Default legacy duplicate replacement remains unchanged on nonpaired hosts. Status
counts reflect the paired table without double-counting mirrored online state.
Every configuration failure, canceled queued join and failed spawn releases its
own ticket; pre-reservation mismatch releases nothing. Test ABA/reconnect ordering.

Track actual paired outer accepted/local connection JoinHandles and close handles
in Lan before an offered attempt can escape. A successful spawn must be registered
atomically with listener stop; spawn failure closes native handles and reservation.
Keep listener, connection reader and writer ownership through explicit completion;
server writer join already belongs to the outer connection but must be checked
and panic-safe, not optional .spawn(...).ok() success. Use focused paired thread
owners, not a generic executor. Lan paired close first stops accepting, invalidates
policy/receipts, shuts all actual connections and closes writer queues. Explicit
poll reaps ONLY is_finished handles; unfinished handles stay owned. Drop with an
unresolved paired handle is fatal as in the existing staged ownership rule, never
a hidden join or detach. Do not infer cleanup from online==0 or a sleep.

## Required close foundation before admission activation

Phase C0 is a separate concrete prerequisite, followed by full configuration C1.
C0 implements and qualifies ordinary retained close ownership on the same native
Game/worker paths; C1 may not relax any paired external guard until C0 is FINAL.
This is a source dependency, not an audit TODO or activation waiver. The original
frozen fixed-two-client application is a reference for typed status/retry and
finished-only joins, not a replacement owner for arbitrary ordinary Games.

### Retained owner, requests and states

The retained owner is the actual existing Game on its current lexical thread.
LanHost, Remote, Integrated, active/parked levels, native SaveDir values, guests,
items and session locks stay in their Game fields. Extend pending_save/ClosingSave
with a concrete paired-close mode; put a small PairedLifetime guard field before
native owner fields in Game's declaration, armed when paired native world/session
ownership is acquired. The guard has no Game reference or generic callback. It is
disarmed only after Closed; unexpected unresolved Drop fails fatally before those
owners are dropped. Planned timeout NEVER drops/returns that owner and NEVER
invokes this backstop. No Game reference is sent outside its worker.

Add game/paired_close.rs, private to game. Proposed concrete APIs:

- request_paired_close(intent: CloseIntent, attempt: u64) -> CloseAdmission;
- retry_paired_close(expected_attempt: u64, attempt: u64) -> CloseAdmission;
- paired_close_view(&self) -> PairedCloseView;
- poll_paired_close(&mut self, now: Instant) -> CloseProgress;
- take_paired_close_completion(&mut self) -> Option<PairedCloseCompletion>.

CloseIntent is Quit, ReturnToTitle, Disconnect, Replace(PendingWorldEntry), or
ReturnServer(original_result). Replacement contains existing owned entry data,
not a FnOnce(&mut Game). Error/result detail is retained as data until completion.
CloseAdmission reports Accepted/InFlight/Stale/WrongAttempt/AlreadyClosed; no
second request or stop signal silently retries. Count attempts with checked_add.
Views are owned bounded status data; commands are typed and non-owning. Close
completion is one-shot; a displayed Closed view alone cannot consume/join a worker.

States: Open -> Closing{attempt,stage} -> Closed{attempt}; any expected stage
error/deadline -> Blocked{attempt,stage,detail,owners_retained:true}. Explicit
Retry for that exact Blocked attempt resumes a new bounded attempt. Remain frozen
between attempts; never resume gameplay automatically. Each attempt has one fixed
<=120second deadline; native snapshot work uses existing per-turn16-snapshot/2ms
budgets, with an indivisible native call acknowledged as such. Deadline means
Blocked, not resource release, function return, event_loop.exit or cancellation
of owned native work. While Blocked, poll only concrete helper completion/observed
status and UI/control input; do not resubmit failed writes automatically.

Paired ClosingSave stages, in order:

1. Freeze: stop new admissions, reject gameplay/tasks/replacement, clear local
   held input, freeze simulation, invalidate pending configured receipts. Retain
   Hub guests and all their payloads even when transport terminal events arrive;
   do not process Left into guest destruction before saved custody is complete.
2. PeerSnapshot: project/store native participant records from retained guests,
   close menus through existing item-preserving services exactly once, preserve
   cursors on failure. All player/metadata writes are checked, not log-and-continue
   save_guest calls. A failed player save keeps the same guest/body and lock.
3. NativeSave: existing Generation/Chunks/Entities/metadata snapshot services,
   followed by the native producer fence for ACTIVE AND PARKED Game level SaveDir
   owners. Keep exact queued snapshots and dirty/cursor state across errors. Native
   terminal queues and late finishes must be accounted, not erased to close.
4. Writes: require every native writer's durable success, including metadata and
   guest results. Then explicitly stop each writer submission owner, observe its
   actual thread is_finished, join/check it, and retain its result. No SaveDir Drop
   join is the defining completion. Same for all owned level writer instances.
5. Network: close actual client/server transports, writer queues and listeners;
   poll/reap every accepted/local/read/write helper and actual Session. A quiescent
   peer set or100ms sleep is not completion. Pending helpers retain the same fields.
6. Integrated: where present, wait for the server worker's own Closed state AND
   thread.is_finished, then join/check once. Do not take Integrated out first.
7. ControlFinal: retain the polled control listener through EVERY fallible stage
   and Blocked state. Only after all native/helper/worker completion, finish or
   boundedly cancel remaining command responses and close native control sockets
   without waiting for peer reads. There are no paired control background threads
   or reply-wait joins. No fallible resource stage follows this final socket close.
8. Commit: only now mark saved_for_exit, clear/release native locks/owners, retire
   runtime trials for this completed close, publish close completion, and perform
   the stored intent. World replacement starts ONLY after this point. Quit/Return
   Server can now relinquish the lexical Game; title/disconnect creates fresh inert
   state. An error original_result is returned only after the owned close completed.

The window that proxies an integrated worker follows that worker's snapshot/save
stages instead of independently saving its server world. It keeps its Remote and
Integrated while the worker closes; worker network shutdown can close the client
pipe, but the client Session must also be explicitly reaped. Server and client
completion remain separate observations. Do not release either owner from a
shared status flag or discard a failed writer to unblock the other side.

### Native writer failure and explicit retry

Actual save.rs writer errors are sticky (:2291/:2424); failed Record snapshots
remain in pending, and current SaveDir::Drop joins unconditionally. Simply polling
writes_finished again cannot establish a successful retry. Include save.rs in C0
for one bounded native retained-close service, without changing disk codecs.

Introduce a concrete non-Clone NativeCloseWriter token for one actual Arc<SaveDir>,
with close/poll/retry methods and retained generation identity. Freeze/finish all
Game producers before obtaining it. Ordinary queues/flush remain unchanged outside
this explicit service. Stage/bound the retained pending Record snapshots first;
never drop/rebuild them from empty world state. Close the old writer's submission
sender, retain its actual JoinHandle in SaveDir/token state, and only join after
is_finished. Verify no old worker/in-flight batch remains before a retry can
replace writer accounting. Preserve failure result and all pending records.

An explicit close Retry, after the filesystem obstruction is fixed, may resubmit
ALL latest retained pending region/entity/POI records to a fallibly started native
writer. Keep the same native record resolution/writer implementation. Stage every
batch/reservation before state change; establish new outstanding counts BEFORE
retiring the old sticky error, so writes_finished cannot transiently report
success. Preserve/restore the sticky failure on spawn/enqueue failure; pending
records stay readable and owned. A new writer error becomes Blocked again. Clear
success only after actual resubmission acknowledgment/durability and checked
writer completion. No public clear_error or pretend-success operation.

Poison, unprovable panic accounting or missing retained records stays explicitly
Blocked with owners; do not repair counters by guess or restart from lost records.
A known completed writer panic may be checked/joined without dropping the Game;
no promise that every panic is retryable. Tests require ordinary native IO error
repair/retry success plus retained panic/error refusal. This is not a crash/journal
or general save-compatibility promise. SaveDir's paired token is fully resolved
before its last Arc release, so its old Drop join is never the planned drain.

### Exact window, control and signal behavior

Extend saving.rs begin_closing_save/poll_closing_save for paired mode. The void
lan_close(false) at70-76 is NOT called before Generation. Add
begin_paired_lan_close/poll_paired_lan_close returning CloseProgress; it borrows
self.lan in place. Existing void lan_close is legacy-only and refuses programmer
misuse on an unresolved paired owner. Likewise remote writes success at188-204
requires explicit Session drain and, if integrated, checked worker completion.
Never set saved_for_exit from Writes alone. poll_closing_save returns Ok(false)
for Closing/Blocked and Ok(true) for quit ONLY after Commit. Expected paired close
errors become Blocked views, not Err propagated into event_loop.exit.

The existing closing HUD displays stage, bounded error and retained state. Add a
Retry button plus nonrepeated R and exact close-status/close-retry control commands;
CloseRequested only records quit intent. While closing, bypass normal gameplay,
agent task execution and mutating controls. game/control.rs handles these two
commands BEFORE its current pending_save rejection and continues bounded control
polling during close. Command retry syntax is close-retry EXPECTED_ATTEMPT NEW_ATTEMPT;
status is read-only. Requests acknowledge admission, not completed close. Native
renderer failure records an output failure and keeps event/control polling and
owners; it does not call exit. Normal visible status remains native HUD, not a
title-only substitute. If the surface cannot draw, the retained control endpoint
remains an exceptional observation/retry surface.

Select horizon_threads::catch_retained_stop_signals for the paired process BEFORE
any call to legacy catch_stop_signals. Move package/settings classification ahead
of handler choice in game::run, retaining ordinary legacy handler semantics on
vanilla. Paired dedicated uses the retained handler too. Existing sticky retained
policy cannot be downgraded later in the process; no threads crate change is
required. First caught signal records Quit; repeated caught signals are counts/
intents only, never retry or _exit. While Blocked, Retry remains explicit. SIGKILL,
external crash and unsupported platform-native forced termination remain crashes,
not successful close. No new unsafe UI/platform hook is proposed.

All paired programmatic exit sites in Game::ApplicationHandler, including render,
resume/entry errors, frame/timeout exits, menu Quit, CloseRequested and caught
signals, call request_paired_close and keep the same Game in the live event loop.
Store the original error until close completion. about_to_wait polls close first
and requests native exit ONLY after Commit; it does not run gameplay while closing.
Replacement methods open_world/load_world/start_world_entry/join_server/finish_open_world
and finish_close_world check the gate: queued replacement data is retained,
WorldRequest::prepare/new Session starts only after old ownership is resolved.
No early saved_for_exit/title check bypasses a live paired Remote/Lan/Integrated.

At game.rs:9377 save_for_exit_attempt, a paired path never treats saved_for_exit
alone as permission to return while live owners remain. Normal close uses the
pollable gate; its synchronous legacy loop is not a paired shutdown service.
ApplicationHandler::exiting (:13595) only verifies prior paired Commit and records
completion; it neither tries a last hidden join nor logs failure then continues.
Irreversible native event-loop exit before gated completion, event-loop error,
or unwinding with unresolved owners is an UNEXPECTED fatal backstop. Planned
close timeout is handled earlier while the loop is still alive and cannot reach
this path. run_event_loop owns Game across run_app, verifies paired completion
before report/error return, and cannot relinquish an unresolved paired Game.
Platform activation must qualify native Quit/CloseRequested gating: where the
platform can deliver an ordinary irrevocable Quit without an interceptable close
request, refuse paired-window activation at startup on that platform until the
native safe intercept is implemented/qualified. Linux is the current defining
qualification platform; this is an explicit platform support bound, not a claim
that macOS applicationWillTerminate can be vetoed from exiting. Default platform
behavior remains unchanged. No OS quit is represented as successful retained close.

### Integrated worker stays owned on its worker

Extend Flags with immutable close view sharing and a bounded typed close command
channel; it carries RequestClose/RetryClose, never arbitrary Game work. Existing
ordinary Task/call remains for Running, but is not pumped while paired closing.
Integrated::request_stop submits the first stop intent only. Add
paired_close_view, retry_close(expected,new), and poll_paired_completion(&mut self).
The last joins/checks ONLY after thread.is_finished and worker Closed, consumes
completion once and leaves an unfinished handle in Integrated. Do not use paired
stop(self), take().stop or Drop::halt as a planned close path; paired unresolved
Drop is only an unexpected ownership violation. Vanilla stop/halt stays unchanged.

Refactor integrated.rs serve (:779-788): once server_game owns paired native
resources, EVERY ordinary run_integrated Ok/Err, initialization/settle/tick/save
failure, ready-channel failure and stop request transitions to a worker-local
retained close loop. The actual server Game remains a local variable in that
worker stack. Keep publishing CloseView and receiving typed retry requests while
polling concrete completion (short waits, no world tick). The loop returns the
stored original Result ONLY after Commit; then retire trials and let Game go.
Neither server_game lifetime nor resource borrows escape to the UI thread. A
failure before acquiring any native paired owner can return directly with a
resolved/disarmed lifetime guard. Panic/unwind is unexpected/fatal, never converted
into a planned timeout escape. The UI Stage::Server keeps Integrated and Remote
until BOTH actual Session drain and this worker's finished-only join complete.

### Dedicated loop and concrete retry surface

Refactor run_dedicated to establish its retained control input state before world
ownership, then execute startup/running inside a result-capturing inner routine.
Keep actual Game, ServerState and input receiver local in the SAME dedicated
thread after any ordinary Result error or stop intent. No save_game()? may escape
that lexical owner after native ownership was acquired, including startup save,
empty-pause autosave and tick failures. The retained close loop replaces the final
stop_dedicated return, not a borrowed reference returned to a supervisor.

For this paired dedicated profile require the existing --control-socket endpoint
in the paired-only polled Server mode before opening a world. Use the SAME native
endpoint/auth/grammar, not a new administration service. Do not start the legacy
detached blocking-stdin console thread for this profile: it cannot be interrupted/
joined by the proposed gate. Native vanilla console stays unchanged.
The existing control grammar server-status and console <line> remains; add direct
close-status/close-retry EXPECTED NEW in dedicated/status.rs, and console close-status/
close-retry routes the SAME typed handlers in dedicated.rs. The endpoint does not
execute arbitrary console/gameplay commands while closing; console stop only sets
intent and never retries. server-status includes CloseView with retained state.
Keep pumping only these handlers plus actual close polls until Closed; loss of a
control client does not cancel ownership or stop the listener. Require an actual
usable endpoint (No AI/control refusal is a startup refusal, not silently rewritten
settings or an unusable retained process). No account/environment rewrite.

stop_dedicated (:1388-1390) becomes request/poll logic over the same close stages;
its player writes are checked, all retained native writes and actual transports
must complete before log_saved_levels/final success. At game.rs:14914-14918,
run_dedicated can return Ok/Err ONLY after paired Commit (or no acquired owner).
The outer stack must verify completion before retiring/dropping Game. A failed
save/drain keeps the process/control loop alive in Blocked indefinitely until an
explicit retry succeeds or an external crash/kill occurs; it does not return.

### Retained polled mode of the existing control service

control.rs currently detaches listener/client workers and can block on stream or
reply reads. C0 MUST NOT use that mode for paired owners. Include control.rs and
game/agent_settings.rs for Server::start_polled(path), using the SAME Unix/TCP
endpoint selection, private TCP token authentication, line grammar and JSON replies.
Default Server::start and its threaded mode remain byte-for-byte in behavior;
never change global socket settings, stdin mode or another process's control path.
No new command protocol, port discovery format, executor or administration service.

The polled Server owns its concrete nonblocking listener and a bounded collection
of nonblocking accepted native streams directly; it spawns NO control listener or
client threads. Platform-specific set_nonblocking is applied ONLY to sockets
created/accepted by start_polled. TCP/Unix close handling stays in the existing
native Stream implementation. Registration and capacity checks occur before an
accepted stream can escape. Failed admission closes only that new control stream.
This Server remains in Game.control while Running/Closing/Blocked and has no
borrow or callback to mutable Game from another thread.

Add poll_io(&mut self, now) and finish_close(&mut self). poll_io does bounded
nonblocking accept/auth/read/write work and feeds the existing Request/reply
channels without blocking response.recv. Admit at most ONE outstanding request per
client (eight total); do not parse/read its next request while awaiting its bounded
reply/output completion. This preserves the existing serial request/reply grammar
and prevents the internal channel from accumulating a per-turn flood. Retain
per-client partial line/auth and output cursors; use try_recv for Game replies. Fixed paired-only limits: at most8
clients,32 accept/read/write operations and8 completed requests per Game turn,
2ms between indivisible native IO calls,4096-byte input line,1MiB output per
client and8MiB aggregate pending response bytes. Bound/check lengths BEFORE
buffer growth; checked allocation failure closes/refuses that control request,
never drops Game. A nonreading peer cannot consume unbounded CPU/storage or
extend an auth/partial-line/response deadline: one fixed10second deadline per
such operation, checked on every poll, then close that client. These are explicit
application bounds, not bounds on OS buffers/scheduling/native allocation latency.
Existing native auth token comparison and endpoint permissions are reused.

Before each close-loop control dispatch, pump poll_io, then allow only status,
exact typed retry and idempotent close intent; no generic queued gameplay/agent
command is executed. In Running, supported ordinary control requests retain their
existing semantics within these documented paired-only IO bounds. If a response
exceeds the output limit, return a fixed bounded control error where possible and
close that client; do not report world-close success or discard item owners.
The listener and all other usable clients remain available while a stage is
Blocked. Client loss/timeout never changes the world-close attempt or retries it.
A new control client can reconnect to the SAME retained endpoint for status/retry.

ControlFinal starts only when every fallible native save, writer/helper join,
transport drain and integrated completion is resolved. Close/retry responses
acknowledge command ADMISSION, not resource completion. They therefore need not
wait for final close to unblock a handler: there is no handler thread. Final
queued output is attempted nonblocking within the already bounded turn budget;
unread responses can be canceled by closing their stream. Then finish_close
shuts/drops all owned native client/listener sockets and removes the owned endpoint
file, with no peer-read wait and no background worker to join. File-removal error
is recorded as a diagnostic, not a reason to pretend active sockets/resources
were retained or to reopen an already closed retry cycle. Endpoint closure and
Game Commit occur in the same terminal nonwaiting path after ALL fallible owner
work; there is no planned Blocked transition after the sole retry listener closes.
Native socket destruction is not a claim that a remote client read an acknowledgment.

Game/agent_settings selects start_polled for paired lifetime. Turning an endpoint
off in Running uses the same bounded native socket finalizer; while close is
pending it may not remove the sole retry surface. The paired dedicated profile
pins its explicitly requested endpoint until terminal ControlFinal. Existing No AI
preferences remain; startup refusal is used instead of rewriting preferences or
creating an unusable dedicated retained owner. A UI with no control endpoint uses
its native Retry/status HUD; renderer failure is observable through any retained
configured endpoint and never licenses dropping an unresolved Game.
### Return/write-site obligations and C0 evidence

| Existing unsafe-to-return seam | Paired required behavior |
| --- | --- |
| saving.rs:70-76 | Keep LanHost in place; Freeze/PeerSnapshot first; Generation cannot follow void drop. |
| saving.rs:188-204 | Writes alone is insufficient; Remote/Integrated/writers/control all drain before saved_for_exit/true. |
| game.rs:9377 early success | Verify complete owner gate, not title/saved flag; no live Remote shortcut. |
| integrated.rs:779-788 stack result | Capture result, stay in worker-local close/retry loop, return only Closed. |
| dedicated.rs:1388-1390 and game.rs:14914-14918 | Capture every save/tick/start error; retain dedicated Game+input loop; no ? dropping owned resources. |
| game.rs:13595-13599 exiting; run_event_loop | Normal exit requested only after Commit; unexpected irreversible exit/unwind is fatal, not log-and-return cleanup. |
| Game world/server replacement and remote leave | Store intent, keep old owners and locks until Commit; then create replacement. |

Qualify C0 before C1 with actual Game/native resources and existing memory/TCP
fixtures without relaxing external guards: checked guest/metadata save failure,
real sticky native region/entity/POI writer error with retained pending snapshots,
filesystem repair then explicit retry, blocked writer/connection drain and hostile
partial/auth/nonreading control clients,
all active/parked SaveDir completions, and integrated/dedicated early return/error.
Prove no released lock, saved_for_exit, replacement, worker return or process exit
while Blocked; retry monotonicity, no automatic retry, stable item/guest ownership
and exactly-one completion after actual joins. Actual UI remains responsive with
Retry/status; dedicated endpoint answers after stop failure and permits fixed
retry. Saturate all polled IO/request/output bounds and prove per-turn progress,
new-client reconnect while Blocked, default threaded controls unchanged, and no
planned Blocked state after final retry-listener closure. Separate child controls cover unexpected unwind/irreversible-exit backstop
and two caught signals while Open AND Blocked (no hard exit/retry/release). Resolve
or externally terminate/reap intentionally retained children; do not drop them as
test cleanup. No default ordinary/platform signal behavior changes are inferred.

## Honest provisional source budget

Twenty-one production paths (under crates/horizon-client/src except item13):

1. lan/login_negotiation.rs: original live allocation, states and guarded handoff.
2. NEW lan/paired_configuration.rs: finite sequence, checked result/receipt services.
3. lan/mod.rs: private policy, owner reservation, envelopes, tracked paired attempts.
4. lan/conn.rs: server configuration, completion/handoff, guarded play I/O.
5. lan/hub.rs: ordinary AND staged receipt consumers before entity/Guest creation.
6. net.rs: paired TCP/memory constructors, strict unsplit configuration, client
   envelope and guarded Session writer/cancellation/terminal play handling.
7. game/mod_packages.rs: pre-materialization route preflight and narrow actual
   holder/expectation access; keep external_network refusing as fallback.
8. game/lan.rs: exact capture/retained LanHost, paired host start, publish guard,
   explicit transport close ownership and detached builder's policy argument.
9. game/remote.rs: independent client capture/retained Remote, paired Session
   selection and live receipt consume before registries; covered close paths.
10. game/integrated.rs: initial memory policy, publish guard, owner and stop drain.
11. game.rs: explicit early server/dedicated route preflight and covered outer
    teardown refusal needed to retain unresolved paired transports.
12. game/lan/connected_recovery.rs: route the same captured configured policy into
    the actual staged memory producer; do not change frozen semantics/roster policy.
13. crates/horizon-protocol/src/connection.rs: native close-only unsplit handle,
    focused TCP/memory/deadline cancellation controls inline.
14. game/saving.rs: concrete paired ClosingSave stages, retained errors/retry,
    native status HUD and no premature successful Writes result.
15. game/dedicated.rs: retained startup/running/stop result loop and existing
    console-command handlers; paired control-only input profile.
16. NEW game/paired_close.rs: concrete Game close state/control/lifetime service,
    owned status/completion and no arbitrary task framework.
17. game/control.rs: exact status/retry before pending-save rejection; bounded
    closing-only polling without gameplay/agent actions.
18. game/dedicated/status.rs: existing endpoint close status/retry dispatch.
19. control.rs: paired-only polled native listener/client ownership with bounded
    IO and terminal socket finalizer; default threaded service unchanged.
20. game/agent_settings.rs: select retained control construction for paired owner,
    preserve/pin its polled retry endpoint through every Blocked state.
21. save.rs: explicit retained native writer close/retry over original pending
    Record owners, checked completion and no hidden planned Drop join.

Five test paths:

22. NEW lan/paired_configuration_tests.rs: direct owned TCP/memory defining tests,
    ordered native packet and stale receipt/reservation controls.
23. game/peer_contract_snapshot_tests.rs: independent actual retained captures,
    pointer provenance and startup guard tests; no claims-only positive authority.
24. NEW game/paired_admission_tests.rs: actual integrated/dedicated/two-client and
    in-frame game flows, declared from already scoped game.rs.
25. game/lan/connected_recovery_tests.rs: genuine paired staged fixture starts and
    existing frozen positive/refusal regressions; retain test-only fixture helpers.
26. NEW game/paired_close_tests.rs: actual native close/retry/retained-loop tests,
    declared in paired_close.rs; focused writer/control service tests stay inline.

This is26 paths, NOT an eight-path packet. Split execution into concrete bounded
C0 then C1, with independent exact-source/actual qualification/different FINAL
for each. C0's16 production paths are items3,4,6,8-11,13-21 plus test26; they add
retained close ownership without enabling paired admission. C1 completes items1-13
and22-25 on that accepted clean foundation. The union is21 production +5 test
paths, with overlapping native services reviewed again for C1 effects. Intermediate
configuration primitives never relax external guards. C1 is atomic activation:
no full-admission claim before strict config, consumers AND C0 teardown work.

Constructor inventory: Parts has one definition/literal (conn.rs:942/:1075);
Joined has two production literals (:1188/:1302) plus twelve fixture paths, hence
the separate envelope. NetConfig has its Self initializer (lan/mod.rs:419), a full
dedicated.rs:602 literal and fourteen update literals: NO new field is planned.
Session has one production Self and one test literal in net.rs. LanHost has one
literal (game/lan.rs:2545), Remote one (remote.rs:452); their added holder fields
are in budget. Game's retained guard initializes in Game::new; ClosingSave, Flags,
ServerState, control::Server polled/native state and writer owner/state literals all remain
within the explicit expanded paths. Refresh exhaustive Event matches and exact
constructors at the accepted pin; mechanical extra paths require revised scope
before authoring, never a bypass. The listed close paths now have required named
owners/stages instead of a future audit waiver. No new source authority follows.

## Required actual qualification and acceptance

Root alone performs all isolated serial builds/runtime/fmt/Git. Use sanitized fresh
resource/package/input captures and isolated native roots, never live saves/accounts.
Use real native TCP sockets and pipes, actual Game/Hub/Remote/Session and prebuilt
paired modules; Rust tests spawn no compiler/test/Python/Java child. Define all
resource requirements explicitly; missing fixtures refuse qualification.

- Matching full negotiation plus full configuration on owned TCP AND memory;
  inspect every emitted registry entry has full NBT with registry_packets(false),
  exact features/tags and complete ordering. Literal independent malformed packet
  controls must not be generated solely by the helper under test. Prove private
  Dynamic stays unpublished until receipt consumer, then actual registry install.
- All nine agreement mismatches (protocol/game, executable, package/content,
  machine schema, builtins, blocks, synchronized registry data, tags, features),
  missing peer/local support, stale challenge/profile/transcript/expectation and
  every B deadline/trickle/replay control. Zero registry emission on pre-agreement
  failure; zero Guest, Game ID change or client registry authority on any failure.
- Missing/duplicate/reordered registry groups AND entries, elided/None NBT,
  tampered nested NBT, duplicate raw keys/trailing bytes, extra/missing features,
  tag groups/names/IDs/order, premature/repeated completion, nonempty known packs,
  wrong prefix/client settings/brand, forbidden config tasks and unknown packets.
  No Dynamic HashMap overwrite can turn these into success.
- Disconnect/cancel/unwind/spawn error at every phase and both sides of handoff;
  delay PairedJoined in server Hub and client backlog until actual terminal latch
  or admission expiry, replace Session/expectation with equal-value new allocation,
  then consume: zero failed admission. One-shot duplicate/replay consumes refuse.
  Exercise reader EOF AND writer failure while a join is queued.
- Concurrent same-profile attempts, pending vs active duplicate, capacity boundary,
  reserved host, checked conn/entity ID exhaustion, old release after fresh
  reconnect and close/accept race. Preserve the legitimate owner's reservation
  and entity; observe actual paired helper completion/join, not just status count.
- Real total deadline across login+config, partial prefix/body and trickled config,
  blocked write, silent first query, unfinished completion/queued consume. Memory
  close interrupts partial frames; verify timeout restoration and both actual
  connection halves/counters with encryption/compression. No claim of bounded
  unbounded-channel memory or hard OS scheduling latency.
- Actual normal integrated owner first succeeds over negotiated memory while not
  published; later publish accepts TWO independent exact TCP clients. Verify their
  retained input identity, real Guest placement, terrain and ordinary packet flow,
  profile uniqueness and fresh disconnect/reconnect. Dedicated actual Game admits
  TWO independent exact clients through the same path and owns cleanup. Paired
  default in-frame host comparison uses the genuine local owner plus real peers,
  with equivalent applicable state/input/configuration observations. A headless
  title-only or frozen-window substitute does not fulfill this comparison.
- Re-run existing genuine frozen/staged recovery and managed window/close checks
  through the now configured memory route. No ReadyFrozen->Running shortcut or
  invented late-departure/checkpoint policy. Re-run B login-only diagnostics,
  vanilla login/query-None/known-pack/compression/reconfiguration/transfer and
  package/world startup guards, appropriate feature/lint/format/broad checks.

Completion requires exact source/tool manifests, actual qualified real-game
outputs, independent different FINAL, clean local engine pin and private durable
results. No public publication is authorized. H10 snapshots/deltas/gaps/resync/
untracking/reconnect semantics, full ordinary lifecycle/Creative/Running, player
permission/revision custody and committed animation remain REQUIRED S1 work.
Successful vanilla terrain/player packets are not machine replication evidence.