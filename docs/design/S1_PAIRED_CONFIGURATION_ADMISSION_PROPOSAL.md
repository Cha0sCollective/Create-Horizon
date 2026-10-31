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
Record retirement and dropped-custody integration and exact parent/tree explicitly.
The active dropped-custody D packet is NOT FINAL at this revision; its nine-source
candidate includes streaming/native-record work. C0 must use a freshly accepted
clean combined successor accounting for those exact changes, not this older B
streaming source or an unreviewed overlay. Its coherent profile requires the
accepted D-supported item ownership profile. Freeze/review source before runtime;
obtain a DIFFERENT FINAL afterward. No active B/D worktree is source authority here.

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
| streaming.rs:1989 native_write_fence_until; :2562 Drop | Fence waits/sleeps and permits Light/Mesh; actual worker stop/join currently happens in Drop. Neither is a close poll service. |
| game/world_loading.rs:113 prepare; :330 cancel | Partial preparation owns locks/Streaming/SaveDir before returning; cancel creates a disposal thread which drops the prepared result. |
| game/world_loading/host_identity.rs:391 finish | Takes SessionLock by value before a fallible identity write; paired partial-failure ownership needs a borrowed retained-lock variant. |
| game/levels.rs:207 swap_level | Clears native_checkpoint_origin. Close must borrow parked owners directly instead of swapping dimensions and destroying coherent eligibility. |

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
2. PeerSnapshot: prepare owned native participant records from retained guests,
   close menus through existing item-preserving services exactly once, preserve
   cursors on failure. Owned CloseIo performs/acknowledges filesystem writes;
   no direct IO occurs on this poll. All results are checked, not log-and-continue
   save_guest calls. A failed player save keeps the same guest/body and lock.
3. Producers: poll retained preparation outcomes, then reconcile/seal/stop ALL
   actual active/parked/client streamers through the pollable services below.
   Every unique native result stays accounted; include Light/Mesh and actual
   worker initialization/completion. No blocking native_write_fence_until call.
   NativeSave then follows the explicit NativeWorld/CoherentSession profile below;
   preserve exact snapshot/dirty cursors and all late native record payloads.
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

### Pollable streaming close and exact level membership

Add streaming.rs to C0. Existing native_write_fence_until loops/sleeps (:1989),
poll_save may scan a whole deferred prefix, and the existing fence explicitly
excludes Light/Mesh from quiescence. These methods cannot be called from a close
poll and described as responsive. Existing Streaming::Drop stops workers, waits
and unconditionally joins; paired close must resolve those exact handles BEFORE
any destructor. Preserve legacy save/fence/Drop behavior for unpaired users.

Introduce a concrete NativeStreamClose owned in the actual Streaming (or its
paired close token), with begin_retained_close, poll_retained_close and exact
retry methods. It retains original thread count, per-origin work receipts,
deferred cursor, retained native record/proto handoffs and stage. No arbitrary
executor, external callback or reset-to-empty readiness API. Begin blocks ALL
ordinary submission/scheduling for this streamer; update/render/mesh/distance/
pregen changes cannot schedule through the gate. It does NOT set shared.stop
while queued jobs still own data, because workers check stop BEFORE queue.pop.

Poll uses try_recv and bounded retained/deferred cursors, at most16 results or2ms
between indivisible result actions per turn. Do not take_deferred into an unbounded
prefix scan, recv_timeout, sleep, call update, or call the blocking fence/save(true).
Use nonwaiting lock acquisition for close's queue/receipt transitions; a contended
worker lock returns Pending, not a blocking wait on the UI/control turn. Ordinary
native callbacks/one record encode can still be indivisible; report that limit.
The same fixed close-attempt deadline applies, and expiry retains Blocked state.

Allow existing queued/worker-held jobs to RETURN while no new ordinary jobs are
submitted. Reconcile each actual origin explicitly:

- Proto/Decorated/Save return the SAME owned proto regions to their exact maps or
  close-retained handoff records. Keep written markers until native storage
  acceptance, then await actual SaveDir durability separately. No missing proto
  is replaced by generation or default air.
- Generated/Finish returns retain the WHOLE native finished record, including
  original entity/block-entity payload, before any tolerant decoding/filtering.
  During close, do not activate it via ordinary update/receive_generated into
  newly live entities or silently replace a resident machine column. Reuse the
  accepted D NativeChunkRecord staging, classification and retry rules exactly;
  preserve resident saved semantics and retain late/off-range records for the
  actual native finished-column/entity writer. No narrowed native record codec,
  host drops file, owner clear or alternate classification is introduced.
- Light results reconcile matching saved-light sequence data for their existing
  frozen resident owner without scheduling more jobs; stale derived light is
  explicitly classified/settled under native rules. Mesh results are derived
  display data: acknowledge/settle their exact receipt without GPU upload after
  checking origin/sequence. Queued and worker-held Light/Mesh still must finish;
  do not merely omit their outstanding counts or clear their payload queue.

At every terminal handoff check submitted = queued + worker-held + returned +
reconciling + deliberately settled/retained facts, with no unexplained receipt,
unique payload or unresolved owner. Storage acceptance is not durability. Keep
monotonic failure history and D's exact retained record custody through explicit
retry; do not zero accounting errors to create a witness. A provably retained
native writer failure uses the native Record retry service below. Uncorrelatable
generation/receipt faults remain owned Blocked; there is no invented recovery
from a lost payload. Routine healthy worlds and repairable writer errors must
close successfully; this is not a normal-operation refusal substitute.

Only when all queued/held/returned/deferred work INCLUDING Light/Mesh is reconciled
and every native handoff has been accepted, set shared.stop under the SAME queue
mutex ordering as worker wait, notify_all, and release it. Capture intentional
stop in the close state; the old running fence's stopped==fault predicate is not
reused. Poll each actual worker handle with is_finished, join/check only finished
handles, and retain all unfinished handles/receipts/payloads. Worker initialization
may still be blocked inside generator.worker; deadline yields Blocked with that
worker/streamer owned, never constructor/Drop cleanup. An initialization failure
or joined panic is reported, not counted as a normal initialized worker. An
unprovable lost unique payload is not repaired. No queue.clear/Drop destruction
is a successful close step. A typed SealedStream witness records exact streamer
allocation/membership and successful intentional stop/join; it grants no world
or player authority. All native writer acknowledgments must still complete.

Use NEW narrow direct per-level services in game/levels.rs, not enter_level.
At Freeze snapshot original dimension and sorted exact active+parked membership,
streamer/save native allocation identities, plus the Remote client's streamer
when present. A LevelCloseCursor visits those existing owners in place, borrowing
Parked.world/streaming/save inside levels.rs and Game fields for the active level.
Do not open a missing level, swap Game into another dimension, mutate idle state,
call close_idle_levels or clear native_checkpoint_origin. Preserve the original
dimension, clocks, persistent frames, native origin and item ownership throughout
errors/retry. Membership change after Freeze refuses while retaining ALL owners.
This avoids a fallible restore-dimension escape and preserves coherent capture
eligibility; levels.rs is therefore an honest extra path. After all witnesses
exist, native snapshot/write stages iterate the same fixed owners without running
server/world ticks. Client-only streamers also stop/join before world replacement.

### Retained world preparation, harvest and cancellation

Add world_loading.rs AND host_identity.rs. The Game lifetime guard alone cannot
protect WorldRequest::prepare's worker-local partially acquired resources, nor
can a JoinHandle prevent prepare's ? paths from dropping them. Paired construction
uses an explicit PrepareOwner from BEFORE the first SessionLock/authority/native
streamer acquisition. It retains Options/intent and optional raw SessionLock,
WorldSession/authority, World/Level, Streaming, SaveDir and native record owners
as each is acquired. A lifetime backstop precedes those fields. Unexpected panic
is fatal; normal Result errors return an OWNED PrepareFailure, never String with
the owners discarded. Every fallible step after acquisition transfers the SAME
owner value on failure. No error cleanup worker or hidden destructor join.

Add a borrowed retained-lock identity finish service: perform all fallible identity
creation checks/write/sync BEFORE taking raw_lock: &mut Option<SessionLock> into
WorldSession. A failed write leaves the raw lock, preflight/identity intent and
partial native artifacts in PrepareOwner. Existing consuming finish remains
legacy-only. install_authority already borrows its Option; keep that property.
No recreation/regeneration retry of a partially created identity is introduced.
A close Retry retries owned drain/storage, not creation of another world identity.

Streaming construction in paired prepare must itself be retained: create its
owner/state BEFORE spawning the first worker, register each actual handle before
it can escape, and return a partial StreamingStartFailure owner on a later spawn/
initialization error. No collect/expect panic drops already spawned handles as
planned error handling. Workers blocked during initialization remain owned by
that partial streamer. SaveDir is immediately stored in PrepareOwner before any
further fallible generator/metadata setup. Late metadata failure after streamer
creation returns that SAME partial native owner for close; it cannot drop locals.
Arbitrary unexpected worker/generator panic remains a fault/backstop, not success.

WorldLoader's paired variant owns the actual initialization handle, non-owning
cancel intent and eventual PreparedOutcome: Ready(PreparedWorld) or
Failed(PrepareFailure). Worker-local PrepareOwner is retained until that owned
outcome is transferred; it is not represented only by a thread completion bit.
Add poll_retained_outcome which returns Pending until is_finished, then explicitly
joins/checks and harvests the owned result ONCE. Paired cancellation never calls
WorldLoader::cancel's disposal-thread route (:330-336), never drop(self.finish()).
Game retains the loader while it is pending, including after close-attempt expiry.
Keep publishing close status and accepting Retry without waiting for initialization.

Quit/Escape/replacement during preparation records intent. A late Ready result
is HARVESTED into Game's pending preparation owner and routed either to ADOPT
only if the original entry is still current/no close is pending, or to CLOSE of
its exact world/streamer/save/lock resources. Adoption is the single checked move
into Game; failure retains the original owner rather than dropping PreparedWorld.
Failed preparation after native acquisition is harvested and closed through the
same per-owner producer/writer stages; close completion returns the stored
initialization error, not a ready Game or save-success claim. Artifacts remain.
Partial preparation without a complete World does not synthesize a Game/column;
it closes exact acquired helpers and accepted native writes, then releases its
locks at terminal Commit. Initialization failures before any native acquisition
can return without an armed native owner. Test both distinctions.

The same retained prepare entry is used by paired synchronous load_world on the
integrated/dedicated worker. It can poll inline in that worker's retained loop,
not return on expected error with a worker-local resource destroyed. game.rs
resume_world_entry, WindowEvent Escape/CloseRequested, queued replacement and
loading-error branches all use this harvest/adopt/close route. WorldRestLoader
(resource preparation only) is also retained/polled/joined before Game relinquish;
no world loader or resource loader is canceled by dropping a handle. Default
vanilla preparation/cancel behavior remains unchanged.

### Explicit native and coherent close profiles

Select profile from actual native ownership, not merely packages.has_pairs or an
assumption that every paired world has an active counter. Store it with the
retained Game close configuration before enabling machines/connections:

- NativeWorld: fresh paired world with NO persistent counter/slot domain or
  machine payload owner. Use actual native player/world/entity records, polled
  producer closure and checked writes/helper joins. This MUST succeed for a real
  new paired world with no counter; do not call counter_checkpoint_frame and
  convert the missing counter into permanent Blocked.
- CoherentSession: actual identified authority plus healthy persistent counter/
  slot domain. Preserve active keys, generation/revision/payload and the D-supported
  item/player owners through successful existing-format coherent checkpoint.
  No retirement/clearing/replacement of those owners is used to obtain eligibility.
  Pending/faulted/detached custody cannot fall back to NativeWorld; retain Blocked
  with its actual reason and the same owners. This does not invent late-departure
  recovery or silently acknowledge a pending item.

Close storage is concrete trusted local configuration: at native paired world
attachment, derive the archive directory as world_root.parent()/
.horizon-session-checkpoints/<actual-world-UUID>, checking canonical nonoverlap,
identity/package binding and retained native authority before use. It is a local
host path convention, not a new format/configuration file or a decoded-peer path.
The Game factory records this request with actual target/type from its retained
binding when a persistent domain is installed. Healthy active domains therefore
have a usable coherent close strategy; the tests must not manufacture a missing
archive option to refuse ordinary machine close. Existing explicit managed
recovery continuation paths remain caller-owned and unchanged. NativeWorld needs
no counter target or invented frame. Persistent startup/recovery selection remains
its separately required S1 outcome; close keeps the complete authoritative image.

Add session_checkpoint.rs and its existing actual-Game tests to C0 for a PRIVATE
begin_close_session_checkpoint/poll_close_session_checkpoint cursor. Do not call
its current blocking checkpoint_session or session_fence from poll_paired_close.
The cursor retains baseline and exact staged image/publication state, while the
concrete owned CloseIo job below performs filesystem work. Game polls only job
status/completion and bounded CPU snapshot preparation; it never performs copy,
read/hash/verification or waits/sleeps for a producer on the status/retry thread. The final receipt
and existing manifest/native formats remain unchanged; synchronous existing
wrappers may drive the same stages under their original exclusive-call contract.

The cursor accepts only the exact SealedStream set of THIS retained Game/level
membership with actual finished/joined producers, not the running fence's
observation or caller-supplied Boolean. It revalidates world/package/authority,
frames, players and D's native item record profile, compares native source/copy
and promotes under existing rules. No configured packet/game task/world tick may
mutate Game between polls; the Game's close gate provides the same exclusive
ownership premise across event turns, and each poll checks that exact epoch/
identity/membership. No cursor/mutable Game escape is public. Keep peers alive
through coherent snapshot while input is frozen; actual departure retains the
existing custody refusal and owners, not an invented late-departure policy.

Producer stop does NOT mean its native writes are durable. Coherent profile uses
the same polled SaveDir durability/retry service and all original pending native
records, then image validation/promotion, then explicit writer close/join before
Commit. Complete capture can be uncertain after promotion; retain error/publication
and explicitly retry at a fresh epoch. Do not release native owners merely because
an image name appeared. Native profile performs normal native writes; coherent
profile adds the required image without a new machine/journal/save-compat schema.
Both profiles must have actual positive close/failed-write-repair/retry evidence.

The existing session_context rejects pending_save and expects running producers;
C0 therefore adds a PRIVATE session_context_for_close with a concrete CloseContext,
not a bypass Boolean. It validates the exact armed close epoch/ClosingSave mode,
retained authority/origin, immutable frame/participant projections and sealed
producer membership. All preparation owners have already been harvested/sealed;
replacement intent is stored in CloseIntent, not a live pending_entry. The ordinary
session_context predicate and synchronous capture route remain unchanged. Split
its filesystem identity/package/pin validation into the owned CloseImageContext
job; do not quietly execute those reads on the status/retry thread. Other accepted
coherent-profile restrictions, including its current single authoritative initial
Overworld representation, remain explicit. Counterless native active+parked close
and coherent single-Overworld close are separately positive; no implicit new
multi-dimension machine representation or save-compatibility claim is made.

### Concrete owned filesystem work, not blocking poll wrappers

A poll name does not make filesystem IO responsive. Game::save_game_metadata
also dispatches direct IO in maps, trading/random sequences, dragon fight,
wandering trader, portal arrivals, stats, advancements, scoreboard/storage/border/
stopwatches, bossbars, functions, and chat clock state; save_level also writes raids.
Include those TWELVE actual metadata adapter paths below rather than calling them
unchanged from a UI poll. They are a bounded prerequisite split, not a new format.

In each adapter, separate native state preparation from filesystem action and
successful dirty-state acknowledgment. prepare_close_record borrows the real
frozen state and produces only that family's owned native save payload/decision,
using its existing native pack/to_nbt/JSON methods. It performs NO read, write,
path.try_exists, lazy load or sync. Existence-dependent sparsity/replace decisions
remain explicit in the job and are resolved on the IO worker. Preserve unloaded
versus loaded and dirty flags; do not load absent families just to save them.
Where existing native types such as CommandStorage/Stopwatches are Clone, an
owned native snapshot may use the same save method on the worker; return its
checked acknowledgment instead of mutating the original state early. No maps,
scoreboard, schedules or other owner are cleared/refused to avoid this work.
Player/owner Level, advancements/stats and native clock state use the same rule.

Define ONE concrete CloseIoOwner in game/paired_close.rs, with a fixed enum of
NativeMetadata and CoherentImage jobs and owned result/ack types. It is not an
arbitrary closure executor, task service or a worker accepting &mut Game. It owns
one actual JoinHandle, native cancellation intent, bounded immutable status and
the job's captured native payload/context. Only close code constructs these jobs.
The Game retains actual guests/items/metadata state, world/authority locks,
SealedStream set and pending close cursor while the job runs; no write authority
is reconstructed from serialized receipts. A job retains its immutable native
Resources/pack/package context needed for native IO validation, with same-instance
checks when harvested. It holds no borrowed Game pointer or UI/renderer object.

All expensive filesystem existence/read/write/sync/directory enumeration, physical
copy, raw region/NBT read, hash and complete image verification run on this owned
worker (or the already owned SaveDir writer for native queued records). Refactor
session_checkpoint helpers to accept its private owned CloseImageContext: actual
retained resource/package/registry owners plus the validated frozen session
projection, expected identity/target/type and native path configuration. Do not
replace Game provenance with an arbitrary claims-only public constructor.
The worker's coherent image uses the exact existing complete-inventory/member/
format/promotion checks, including D records; no shortened validator or new format.

The main UI/dedicated/integrated close loop performs only bounded CPU snapshot
preparation, nonblocking command/status work, is_finished checks and result harvest.
Native serialization of one bounded graph can still be indivisible CPU work;
there is no hard real-time latency promise. It does not call filesystem helpers
inside the poll and then excuse a stuck UI as an indivisible IO call. A stalled
metadata/read/copy/verify/sync operation leaves the ACTUAL CloseIoOwner running;
the close-attempt deadline publishes Blocked with that same worker/payload/locks
retained. Status/retry requests continue on the owning event/worker loop. Retry
while that IO handle is unfinished is RetainedBusy; it cannot start a competing
job, drop the handle or release the lock. After actual completion, harvest only
with is_finished -> checked join -> owned outcome. Stay Blocked until explicit
Retry if the attempt already expired; no silent new attempt on late success.

NativeMetadata output acknowledges exact prepared family snapshots. Apply dirty/
saved markers to the retained frozen Game ONLY after successful matching worker
receipt; any partial failure keeps all original state and retry payloads. Retry
uses the same native projection after external filesystem repair, without item
loss or a pretend Saved flag. CoherentImage output retains publication state and
pending/published path evidence on every outcome. Record Uncertain BEFORE the
native rename attempt, so a worker error/panic cannot be misreported as a definite
rollback. After final artifact receipt, revalidate the retained Game's relevant
identity/projection without filesystem work on the main turn. Complete source/
image re-reading is performed by the owned validator job while Game stays frozen.

Spawn failure before a job starts preserves all prepared payloads in the close
cursor and returns Blocked. Expected IO errors preserve their actual payloads and
artifacts; unexpected worker panic yields a checked joined failure and no ready
receipt. Do not release native locks/owners merely because a job thread finished.
A completed job whose full outcome cannot be validated remains owned Blocked.
All CloseIo handles are explicitly joined before any metadata owner, lock or Game
can be relinquished; Drop is only an unexpected invariant backstop. No hard kill,
timeout-based detach, new journal or claim that interrupted OS writes rolled back.
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

Additional REQUIRED C0 cases are actual generation, Light and Mesh worker-held
stalls and contended initialization/stop gates. Poll close and status through
multiple turns; deadline must retain Blocked, all payloads/handles/locks and no
queue destruction. Release the controlled native work, explicitly Retry and
observe exact all-origin reconciliation, actual worker completion/join and writer
acknowledgment before Closed. Exercise active AND parked native writer IO errors,
repair and Retry independently, while preserving original dimension/native origin.
A remote client's own streamer must also be stopped/reaped before replacement.

Control preparation at: before native acquisition; immediately after raw lock;
post-authority/identity-write failure; after SaveDir; during worker initialization;
post-streamer level metadata failure; and late successful PreparedWorld after
Quit/Escape/replacement. Hold each actual worker, observe responsive polled control/
HUD through Blocked, release it and harvest once. Verify no disposal worker,
no implicit destructor wait, no early lock release or next-world lock acquisition,
no adopted canceled result and no partially acquired resource lost on Err. Keep
original failed native artifacts. Normal negative startup can finish only after
its actual acquired helpers have closed; it is never published as a ready Game.

Require two separate positive close profiles: (1) genuine fresh paired World with
NO counter/slot domain, native player/entity save, actual worker/writer close and
new native reopen checks; (2) active healthy coherent owner, real D-supported rich
item custody, unchanged active frame keys/payload through actual checkpoint and
restore/recapture. Include coherent save/promotion failure followed by explicit
new-epoch Retry, ordinary native writer repair, live exact peers and no generation/
clock catch-up. No retiring/clearing counter, dropping items or permanent missing-
counter refusal can satisfy either case. No source authority or format promise.
Owned CloseIo qualification must STALL actual native metadata IO, source read,
physical copy, member hash/verification and sync separately while the main
polled endpoint/window processes bounded status and rejected/accepted retry turns.
No direct filesystem call on the status/retry thread counts as a poll. Prove
RetainedBusy while the actual IO handle is unfinished; after late completion,
remain Blocked until explicit Retry, then harvest/check actual handle and preserve
all source/previous image/dirty metadata owners. Also prove spawn failure and
partial per-family native write error retain their original snapshot/ack state.
The fixed-context image result must preserve package/world/authority/native item
checks and existing format bytes; no self-generated summary receipt replaces them.
## Honest provisional source budget

Thirty-eight production paths (under crates/horizon-client/src except item13):

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
22. streaming.rs: retained partial construction, bounded all-origin producer
    reconciliation/seal/stop and finished-only joins; preserve D record semantics.
23. game/world_loading.rs: paired retained PrepareOwner/outcomes and harvest/adopt/
    close cancellation, no result-disposal thread or worker-local error destruction.
24. game/world_loading/host_identity.rs: retained borrowed-lock identity finish,
    preserving actual lock on post-acquisition identity-write failure.
25. game/levels.rs: direct fixed-membership active/parked close cursor services,
    no enter_level/swap_level/provenance destruction or fallible dimension restore.
26. game/levels/session_checkpoint.rs: private pollable close-image cursor over
    actual sealed producers and owned IO context; existing formats/wrappers retained.
27. game/maps.rs: pure native map/id save preparation and checked dirty acknowledgment.
28. game/trading.rs: native random-sequence snapshot, no live RNG advance/inline IO.
29. game/dragon.rs: native fight save projection and checked acknowledgment.
30. game/wandering_trader.rs: native trader state projection/acknowledgment.
31. game/nether_portal.rs: native arrival save projection/acknowledgment, no removal.
32. game/stats.rs: owned native player stats save input/receipt.
33. game/advancements.rs: owned native advancement save input/receipt.
34. game/scoreboard.rs: native board/storage/border/stopwatch save snapshots/ack.
35. game/bossbars.rs: native bossbar projection with original loaded/sparsity rules.
36. game/functions.rs: native scheduled-event snapshot with original ordering.
37. game/chat.rs: native clock save snapshot, no direct filesystem call in close poll.
38. game/raids.rs: native raid save projection/acknowledgment for fixed level owners.

Eight test paths:

39. NEW lan/paired_configuration_tests.rs: direct owned TCP/memory defining tests,
    ordered native packet and stale receipt/reservation controls.
40. game/peer_contract_snapshot_tests.rs: independent actual retained captures,
    pointer provenance and startup guard tests; no claims-only positive authority.
41. NEW game/paired_admission_tests.rs: actual integrated/dedicated/two-client and
    in-frame game flows, declared from already scoped game.rs.
42. game/lan/connected_recovery_tests.rs: genuine paired staged fixture starts and
    existing frozen positive/refusal regressions; retain test-only fixture helpers.
43. NEW game/paired_close_tests.rs: actual native close/retry/retained-loop tests,
    declared in paired_close.rs; focused writer/control/streaming tests stay inline.
44. NEW game/world_loading/retained_prepare_tests.rs: partial acquisition, late
    Ready/error and no-disposal cancellation controls using actual native owners.
45. game/levels/session_checkpoint_tests.rs: genuine no-counter native close and
    active coherent owner close/capture/failure/retry results, not cleared owners.
46. NEW game/paired_close_metadata_tests.rs: real native per-family metadata/save
    receipts, partial failure, ownership and blocked-IO responsiveness controls.

This46-path UNION is a design dependency map, NEVER an atomic implementation
assignment or permission to author all C0. No source edits follow this document.
Each packet below needs its own bounded fresh design/source acceptance, actual
qualification, DIFFERENT independent FINAL and clean/private checkpoint. Pin the
accepted combined B/D successor before the first overlapping native packet;
recheck D's streaming/native-record changes before every dependent source freeze.
Primitives must have their real ownership outcome, not a diagnostic refusal proof.

| Packet | Exact planned path subset (numbers above) | Input, concrete outcome and stop condition |
| --- | --- | --- |
| C0a retained producer/preparation | 11,22-24,44 | Actual paired native WorldRequest/worker resources. Bounded all-origin Streaming close including initialization/Light/Mesh and retained partial prepare/late outcome. Actual lock/helper completion and no disposal-thread proof. Stop after accepted clean component FINAL; no UI/network guard relaxation. |
| C0b retained native writer retry | 21 (focused inline tests) | Actual native SaveDir pending Record batches. Explicit close/join and repaired IO retry preserving region/entity/POI payloads. Positive durability + blocked/failure controls. Stop at component FINAL; no gameplay or format change. |
| C0c1 core native metadata IO inputs | 16,28,32-37,46 | Real frozen stats/advancements/scoreboard/storage/border/stopwatch/bossbar/schedule/clock/random state. Fixed owned native projection + same-codec IO + checked acknowledgment, actual partial-failure/retry. No live Game callback. Stop at this family-set FINAL. |
| C0c2 remaining native metadata IO inputs | 16,27,29-31,38,46 | Real maps/fight/trader/portal-arrival/raid owners. Preserve native fields/dirty/sparsity and resolve actual filesystem work on owned CloseIo. Stop at this family-set FINAL; no dropping an unsupported family to pass. |
| C0d fixed level and coherent image IO | 8,11,16,25,26,43,45 | Accepted producer/writer/metadata primitives plus real retained NativeWorld AND CoherentSession. Direct active/parked borrowing preserves origin/dim; fixed owned CloseIo performs full native metadata/image reads/copy/hash/verify. Counterless native close and active coherent close/restore both succeed. Stop at source/component FINAL without external admission. |
| C0e1 concrete transport ownership | 3,4,6,13,43 (inline native tests too) | Actual TCP/Pipe/read/write/listener handles. Close-only interruption, complete tracked finished-only reaping and terminal latch proof; no online-count substitute. Stop at actual ownership FINAL, default/B diagnostic behavior retained. |
| C0e2 window/in-frame owner/control | 8,9,11,14,16,17,19,20,43 | Accepted native/IO/transport primitives, actual window Game and same polled control grammar. All normal close/error/replacement routes retain owners, Retry/status stays responsive under stalled real IO. Stop at owner-wiring FINAL; no paired external startup enablement. |
| C0e3 integrated/dedicated owners | 10,11,14-16,18,43 | Actual worker-local Game, UI Integrated handle and dedicated control loop. Ordinary Ok/Err returns wait for CloseCommit; failed save/drain and late prepare retain worker/locks until explicit retry and actual joins. Stop at worker owner FINAL. |
| C0 composite gate | No unreviewed new source; exact accepted combined tree | Requalify all native/control/thread/error seams together, both positive close profiles and accepted frozen memory recovery. Independent composite FINAL/private checkpoint is REQUIRED before C1. |
| C1 full configuration activation | 1-13,39-42; accepted C0 paths only for freshly reviewed necessary integration deltas | Exact B agreement + full ordered native configuration + live one-shot admission/reservation consumers on actual integrated memory/TCP, dedicated/two exact clients and in-frame comparison. Enable only after complete real qualification/different FINAL. |

Any changed shared path must be reviewed against the accepted exact predecessor,
not overlaid onto a concurrently qualifying tree. The union is38 production +8
test paths; C0 uses33 production plus tests43-46, in the bounded packets above.
C1 completes the13 production admission paths plus tests39-42 on that foundation.
A packet dependency discovered during concrete source prep changes THAT bounded
proposal before edits; the46-path map is not standing permission or a new executor.
Primitive entry points remain dormant to ordinary launch until the retained
owner-wiring packet is qualified. Direct genuine native resource/Game component
tests establish their actual ownership outcomes without granting partial ordinary
admission. S1 continues across these gates; none substitutes for full admission.
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