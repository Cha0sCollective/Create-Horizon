# Connected memory-protocol recovery proposal

Status: DESIGN ACCEPTED by fresh independent native read-only Sol High review at
exact proposalb2adefe56107c05a412cfa162cee1e3b26f64a58cb08979d18ba58b8e4cde456.
Snapshot .local/s1cr1-proposal2.md. Qualified sourcebase23e09dd1f0ec3c2a0a69e6e002ddab4a92736b11.
This is not runtime acceptance,
source authorization, ordinary startup/close coverage, TCP qualification or an
S1/G1 pass. The next source base is not yet established: the managed-stop candidate
is still under root's serial validation. Do not release this packet until that
candidate has independent FINAL acceptance, a clean engine commit and its private
evidence checkpoint, followed by a fresh independent Sol High review and accepted
checkpoint of this exact design. Root pins that accepted source successor before
authoring; exact-source review precedes execution. Root alone executes and commits.

## Chosen first packet and claim

Build one dormant, opt-in connected recovery adapter. It demonstrates an actual
integrated owner and one actual guest, each using an ordinary headless client Game,
through real memory-pipe login/configuration, native participant restoration,
placement, recovered terrain and actual teleport/PlayerLoaded acknowledgments.
Then re-project and checkpoint the recovered positive roster while the server is
frozen. The machine slot remains empty and ordinary machine-item ingress disabled.

The first packet does NOT activate ordinary launch/UI, accept external sockets,
run recovered gameplay, solve offline-owner reconnect, or extend the current worker
into a normal playing server. This is positive-roster protocol/recovery evidence,
not another zero-roster-only case. Both exact client Games must observe their own
recovered nonzero supported player inventory, position and loaded terrain. No Guest
is constructed by a test, no readiness bit is fabricated, and cached player bodies
or manifest claims cannot mint restoration authority.

This packet precedes ordinary UI/holder work because the actual recovered roster
and its readiness protocol must first be concrete. A later ordinary holder and a
reviewed live-departure adapter are prerequisites for releasing gameplay or filled
machine ingress. No S1 mode or conservation requirement is removed by this split.
No new owner policy is needed; the coordinator resolves implementation choices.

## Factual source seams

The native audit inspected .local/worktrees/s1-managed-stop at base
938859881792438b39e5f597eb331a829e266199 plus its four scoped candidate files.
These are planning anchors to reverify against the accepted successor:

- lan/mod.rs:503 connects actual local pipes. net.rs:130 uses that pipe for the
  ordinary client network code; login/configuration acknowledgments are in
  net.rs:423,664. lan/conn.rs:988 emits Joined only after those stages complete.
- lan/hub.rs:643 admits Joined into a real fresh Guest. poll at1135 also removes
  departures and handles duplicate logins; it is not a recovery barrier.
- game/lan.rs:2435 constructs a host and immediately attaches it to Game.
  lan_guest_joined at3040 loads native player files and places the guest; it cannot
  be reused unchanged for archive restoration.
- game/lan.rs:1063,1173 installs checked rows but requires already placed/loaded
  empty guests. This component entry and its checks remain intact.
- session_checkpoint.rs:2168 acquires managed authority, runs complete recovery and
  attaches authority. Its existing restoration calls the old participant installer
  at2378 before final player/frame/snapshot correlation.
- Hub::place_new_player at687 sends real Login, abilities, teleport and inventory,
  and sets placed. PlayerLoaded is accepted in generic processing at2160.
  remote.rs:779 poll_stationary_client sends it only after actual lit terrain.
- Hub::remove_guest at1257 returns the actual Guest in HostEvent::Left. Ordinary
  game/lan.rs:2953-2975 then performs leave mutations, invokes logging-only guest
  saves and removes GuestPlay. That is a custody boundary, not a safe failure path.
- Lan events and writer queues currently use unbounded channels
  (lan/mod.rs:415, lan/conn.rs:955). PlayerSender accounts pending bytes but currently
  allocates/enqueues before the writer checks MAX_PENDING. Lan::poll drains all
  available events into a Vec. The ordinary local-connection JoinHandle is discarded.

## Concrete ownership and API shape

Keep all new game-facing types private under game::lan::connected_recovery, except
narrow visibility required by the sibling checkpoint implementation. No public
Game callback, arbitrary Task executor or general session framework is introduced.

Proposed responsibilities, with final names subject to exact design review:

- ConnectedRecoveryRequest: existing managed owner/source/destination routing,
  target/type, fixed offline profile names, and one bounded setup deadline. It also
  names a separate fresh continuation archive. The names are local fixture
  configuration; their derived UUIDs must match the verified checkpoint roster.
- PendingManagedRecovery: non-clone owner of the fresh candidate Game, native
  session, UUID lease, selected checked image/rows, detached host, real connection
  endpoints/handles, expected roster, readiness observations and deadline.
- ConnectedRecoveryStatus: PreparingNative, AwaitingConfiguration, InstallingRows,
  AwaitingTerrainAcknowledgments, ReadyFrozen, Refusing, CleanupBlocked, or Refused.
  Failure carries a typed stage/detail, not a playable Game or successful receipt.
- ConnectedSessionOwner: non-clone owner returned only after the final existing
  semantic correlations and real protocol readiness. It contains the same Game,
  actual host/GuestPlay/guests and leases, with gameplay permanently frozen in this
  packet. Its API exposes immutable observations and explicit checkpoint-and-close;
  it exposes no into_raw_game, tick, resume, inventory mutation or arbitrary closure.

begin_managed_connected_recovery consumes a fresh headless candidate and acquires
managed authority BEFORE selecting an epoch. Reuse existing newest-complete-image,
identity, package, physical/native/frame and coverage verification. Split the new
route after native counter/slot restoration but before participant installation.
PendingManagedRecovery owns everything across that split; no playable state or
borrowed Game escapes. Install the UUID lease in its actual WorldSession before
any transport is offered, and retain the selected owner native lock as today.

The existing recover_managed_session_checkpoint, recover_session_checkpoint and
placed/loaded component installer keep their current caller contracts. Factor only
shared checked restoration/projection helpers as necessary. Do not bypass a final
check or weaken the existing profile merely to share code. The new route must reach
the same final native clocks, counter/slot, raids, player rows and owner inspection
checks after real readiness. CheckedSessionPlayers remains constructible only by
complete image verification, never from a decoded UUID list or a client claim.

A bounded owning coordinator function drives PendingManagedRecovery synchronously.
It has a typed transport-offer channel and status/command interface; callbacks see
only owned pipe offers, immutable status and typed commands, never mutable Game.
Use the accepted managed supervisor's no-detach/checked-terminal discipline, without
routing through legacy Integrated::stop/Drop. This first packet need not modify
managed_stop.rs or ordinary UI to obtain that discipline.

A frame guard prevents unexpected unwind/early return from dropping unresolved
connection handles or a ReadyFrozen ConnectedSessionOwner. Its exceptional path
is a minimal best-effort fatal marker followed by process abort, consistent with
the accepted managed-stop fatal guard. It does not join, detach, clear owners or
implement ordinary cancellation in Drop. Ordinary startup refusal and successful
close instead take the explicit finite paths below. A new positive-session guard
requires its own exact-source review and isolated fatal-child verification if it
is needed; the old worker's guard does not automatically cover another frame.

## Genuine detached host and staging

Factor host construction from lan_host into a private builder. The ordinary
lan_host wrapper still installs block/light logs and attaches its result exactly
as before. The connected route builds a detached real LanHost using recovered
native level settings, actual resources/encoder tables and an integrated owner
mapping obtained from verified rows plus configured actual offline profiles.
It does not load playerdata from the destination or place a guest.

For this packet require exactly two rows: one integrated_owner and one external
guest, both in the supported Overworld. Require their configured profile-name UUIDs
to match exactly and require the native integrated-owner cache/mapping checks that
the old installer already enforces. Set host.owner from that real expected profile
with saved=None; no synthetic saved local player or fresh default body becomes the
owner. Owner identity is later confirmed by the genuine Joined connection.

Offer two actual local pipes only after checked native preparation. Each offered
pipe is single-use owned transport; a cloned status view cannot recreate it. The
ordinary client Game calls connect_local and poll_stationary_client. It completes
the existing handshake/login/configuration code. Returning a transport offer is
not ReadyFrozen or authorization to apply gameplay.

A separate staged Hub poll consumes bounded raw events without ordinary tick or
HostEvent dispatch. Use Hub::admit on genuine Joined to create guests. Match actual
UUID/name/connection and sender identity against the expected exact roster. Reject
missing, duplicate, extra, wrong-mode or wrong-dimension participants. All guests
must still be unplaced/unloaded and otherwise fresh/empty when rows are installed.
Do not assign readiness bits or use ordinary lan_guest_joined to make them eligible.

A new staged installer applies CheckedSessionPlayers to those actual guests while
they remain unplaced/unloaded. It must preserve the old validation of native local
cache, storage UUID mapping, idle GuestPlay/menu/items/effects, exact row coverage,
body values, advancements and statistics. Require no preexisting motion/loaded_body;
apply the VERIFIED row through the native playerdata/body conversion, consume any
newly produced native staging once, then compare direct actual body/player
projections with that row. Neither a cached body nor freshly cleared fields prove
eligibility. Move the true host into Game and clear redundant local caches only
following the same admitted-row reasoning as the old installer, never as preflight
preparation. The old component installer continues requiring placed/loaded guests.

After all rows pass, call Hub::place_new_player for each actual connection using a
TickCtx for the recovered world. This is the only transition to placed. It sends the
normal protocol Login/inventory/teleport. Native participant bodies are already
installed; no playerdata load or spawn adjustment may overwrite them.

## Readiness pump and frozen continuation

Add a narrowly gated Hub method for recovered initial terrain and acknowledgments.
It reuses actual chunk encoding/view/batch helpers on already verified/resident
columns; it does not call tick_level, process_inbox, tick_server(true), Game::tick,
run_tasks, apply_publish or wait_for_next_server_tick. No simulation/native clock,
physics, block effect, mod action, guest command or pre-generation work runs.

After placement, permit only the actual stationary client's required packets:
ClientInformation with the configured bounded view, exact AcceptTeleportation,
ChunkBatchReceived, PlayerLoaded, and harmless required protocol control such as
Pong/ClientTickEnd if observed in the exact client path. KeepAlive remains handled
by the connection reader. The exact allowlist is closed in source and tested; do
not add Unknown, movement, actions, chat/commands or inventory packets to pass a
fixture. Unsupported packets refuse staging before any handler mutates Game.

Validate teleport ID and finite returned position/rotation against the sent
recovered teleport. Do not use the generic movement/position_change handler.
Clear the corresponding transport wait only on the actual matching packet.
Accept PlayerLoaded only after placement and terrain delivery; only the handler
for that actual packet may set loaded. Require every expected guest's current
connection, acknowledged teleport, actual loaded acknowledgment, bounded/drained
inboxes and drained sender state. Client observations must separately show actual
Login, recovered inventory/position, lit recovered terrain and loaded_sent.

Only then run the unchanged final recovery correlations and produce
ConnectedSessionOwner/ReadyFrozen. ReadyFrozen certifies the protocol/candidate
result, not a gameplay-enabled server. It remains frozen and supports a checked
continuation capture into a DISTINCT fresh archive, then receipt-only close.
That operation re-projects actual native/body/player ownership and saves the real
Game; it does not reuse serialized manifest rows as current state. No counter step,
normal tick, recipe operation or other state-changing continuation is included:
the continued evidence is coherent recapture of the actual restored owners. Any
future checked step must remain inside the same retained owner through its next
checkpoint and both publication errors, or be separately deferred/reviewed. The source
archive/selected image stays read-only throughout. Compare the complete source
archive tree before and after every success/refusal/continuation case.

The continuation archive is preflighted separately from namespace, source archive,
trusted owner and destination, with existing strict path and namespace rules. Use
existing unused-epoch allocation and checkpoint publication checks. On a confirmed
continuation receipt, retire supported owners, close actual transports and release
Game/leases without a second native-only save or leave-stat mutation. On either
publication error retain ConnectedSessionOwner and permit explicit retry/status
only. No caller can drop it through an ordinary error return; use the owning scope
and fatal misuse guard above. This gives a concrete post-recovery continuation
without claiming ordinary close/UI integration.

## Bounded opt-in transport; legacy behavior unchanged

Add one feature-gated local staging entry to Lan rather than replacing ordinary
networking or exporting a generic queue framework. It requires listen=false,
announce=false, online=None, no external resource-pack flow, and the sealed exact
profile policy. Existing Lan::start_with/connect_local/poll and ordinary sender
behavior keep their semantics. The new game adapter calls only staged entrypoints.

Chosen initial bounds are TWO admitted profiles, at most THREE concurrent local
connection attempts including one rejection probe, incoming play payloads at most
64 KiB, at most64 queued events and256 KiB queued incoming payload bytes per host,
and at most16 MiB pending staged outbound bodies. A poll handles at most16 events
and one chunk batch per client, then yields to the owning coordinator. One setup
request uses a finite deadline not exceeding the existing120-second component
bound. Root may adjust concrete constants before source freeze only with evidence
and a reviewed delta; they are not promises about general multiplayer capacity.

Implement a private staged mailbox alongside the legacy event channel. Production
emit sites in the staged reader route through it; ordinary sites retain the old
channel. Charge payload/count budget before retaining an event; release its exact
charge on dequeue/drop. The existing protocol frame cap bounds one in-flight
receive/decode allocation per admitted connection; the64-KiB staging check occurs
immediately after frame receipt and before play decode/enqueue. Disclose that queue
bounds do not replace the existing wire-frame allocation bound.

Account outgoing bodies before they enter PlayerSender held/post/writer queues.
A staged sender has its own optional budget; reserve before append/send_many,
release correctly on writer consumption/error/drop, and limit an in-flight write
separately. Current MAX_PENDING checked after dequeue is not this guarantee. No
Holding/post-thread batching is needed by the staged adapter. Actual chunks still
use the existing encoder and packet formats.

Do not block the Game owner on a full data queue. Overflow records a bounded sticky
failure and closes that connection. Reserve a separate bounded terminal-notice slot
per local connection so a full packet queue cannot erase Left/overflow/decode-error
information. Record bounded reasons once. Terminal notices are checked before data
polling and before ReadyFrozen; they do not remove a Guest or GuestPlay. All maps,
reasons and connection handles have the fixed connection bound.

The staged login policy checks expected names and derived UUIDs, atomically reserves
one admission per profile, and rejects duplicate admission immediately. It must
not enter ordinary Shared::try_admit's Duplicate event/30-second replacement loop
or kick the original peer. A duplicate attempt refuses this staging request; it
does not silently replace an admitted connection. This is local offline profile
consistency, not cryptographic authentication, external compatibility admission,
secure chat verification or authorization of a player item operation. TCP, online
accounts and externally hostile admission remain separate later work.

Track the local connection JoinHandles for this opt-in entry; today's ordinary
connect_local discards them. Keep them in a bounded private owner. Disconnecting
sends/forces native transport close without invoking guest gameplay/leave saves;
check actual completion before explicit join. Do not add join-on-Drop or detach.
A staged Lan cannot be ordinarily dropped with unresolved tracked handles.

## Refusal and custody, before and after ReadyFrozen

Before ReadyFrozen, missing/wrong/duplicate profiles, prohibited packets, invalid
acknowledgments, deadline, decode errors, overflow and departure all enter Refusing.
Stop admitting/placing, keep candidate/host/GuestPlay/guests/leases together, initiate
actual transport closure and pump only finite cleanup/status. Do not invoke
lan_guest_joined, HostEvent::Left ordinary dispatch, save_guest or lan_close.

After every staged connection thread is positively finished, explicitly join it,
then discard the entire unpublished candidate and release its locks, returning
Refused. Preserve the source archive, destination/pending native bytes and failure
evidence. Do not reset a failed Game or retry into an existing destination; a new
attempt starts from a fresh candidate/destination. While no gameplay was enabled,
all positive saved state remains in the unchanged checked authoritative archive.

A cleanup deadline is not permission to drop unresolved handles. Report
CleanupBlocked while the same owning coordinator retains everything until actual
closure; the controlled fixture has an explicit finite closure schedule. Unexpected
unresponsive cleanup is a failed/fatal isolated fixture diagnostic, never successful
recovery, normal forced-discard or a hidden background detach.

At/after ReadyFrozen, a transport departure or prohibited packet records a retained
failure while preserving actual Guest AND GuestPlay, machine/slot/native/session
state and both leases. The staged poll never calls Hub::remove_guest or deletes
host.play. Do not serialize this incomplete/disconnected roster as a successful
continuation: existing owner/profile checks remain; a new ordinary detached-owner
format is outside this packet. A disconnect cannot be repaired by setting loaded,
inventing an owner, replacing sender identity or reattaching a new socket silently.

Success tests close only after a checked continuation receipt. Post-Ready departure
safety is checked in an isolated fatal-boundary child: show immutable retained
Guest/GuestPlay/lease observations and explicit refusal of close/resume, then root
terminates that isolated child as an expected exception with preserved archive.
It is NOT counted as a successful retry/close or authorization of a production kill
path. Ordinary finite negative cases occur before ReadyFrozen and complete actual
cleanup. This explicit split avoids inventing an offline-owner resolution solely
to make a test terminate. General participant-departure/reconnect custody must have
its own accepted adapter before ordinary gameplay or filled ingress is enabled.

## Exact proposed source scope

The first candidate is SEVEN source paths, reverified against the accepted base:

1. crates/horizon-client/src/game/lan.rs: declare the child module; factor detached
   real host construction; expose narrow checked row/body projection helpers for
   the separate staged installer. Preserve ordinary wrapper/component guards.
2. crates/horizon-client/src/game/lan/connected_recovery.rs: private request,
   PendingManagedRecovery orchestration, separate staged installer, owning frozen
   completion/continuation/cleanup and typed status/transport offers.
3. crates/horizon-client/src/game/lan/connected_recovery_tests.rs: two actual client
   Games, source capture and success/refusal/retained-custody resource cases.
4. crates/horizon-client/src/game/levels/session_checkpoint.rs: narrow owned split
   before participant install and shared final correlations. Existing complete
   component entries retain their checks and caller semantics.
5. crates/horizon-client/src/lan/mod.rs: opt-in local staging policy, bounded mailbox
   and terminal notices, limited dequeue and tracked local connection ownership.
6. crates/horizon-client/src/lan/conn.rs: staged admission/emission, payload/queue
   accounting and native closure; optional staged PlayerSender budgeting.
7. crates/horizon-client/src/lan/hub.rs: genuine staged admit, row-ready placement,
   bounded initial terrain/ack pump, and no-removal departure reporting.

No changes to game.rs, saving.rs, remote.rs, net.rs, ordinary configuration parsing,
managed_stop.rs, slot ingress/transfer, schemas or public mod APIs are included.
Tests use existing ordinary client Game methods without substituting protocol
stubs. Rust visibility from the game binary to the client library may require
feature-gated narrow public staging types; that does not adopt a stable public
engine API. A required eighth path, unrelated networking rewrite, new wire format
or bypass of existing semantic checks requires returning to design review rather
than expanding this list while exploring. This is proposed scope only.

## Finite actual-resource proof

Root orchestrates isolated invocations with pinned resources/prebuilt packages,
existing toolchain and fresh separate roots. No test compiles or launches tools.
Collect discrepancies and reach the predetermined explicit closure before reporting
ordinary assertion failures. Preserve all failed attempts outside Git.

Source fixture:

- Construct a genuinely new managed native TestTerrain server with actual counter
  bootstrap and empty persistent slot. Use the normal integrated-owner mapping and
  a real host, never Guest/Joined test constructors or manually assigned motion.
- Connect two ordinary client Games through genuine local handshakes. During this
  SOURCE SETUP ONLY, run a bounded recorded normal server tick/dispatch schedule
  until real placement/teleport/PlayerLoaded and actual motion owners exist.
- Before the first authorized source mutation, put the source Game/host/GuestPlay
  and leases under the same explicit no-drop/fatal-misuse lifetime discipline.
  Source setup is not a freely droppable test Game once positive owners exist.
  A command/assertion discrepancy is collected; it cannot return through question-
  mark propagation and drop changed state. The fixture's predetermined finite
  resolution captures the actual remaining supported state, closes after its
  confirmed receipt, then reports the discrepancy. Unsupported state is retained
  and diagnosed; never clear it to manufacture a successful cleanup checkpoint.
- Use the actual authorized owner command path over that real connection to give
  a fixed small supported item/count to each named player. Verify the same counts
  in both client inventories and actual server projections. Do not set Guest
  inventory directly, write player NBT as a substitute, or use machine-slot ingress.
  Record exact commands, identities, tick order and expected cells/counts.
- Enter the explicit frozen capture barrier, drain supported pending acknowledgments,
  and capture/check the positive roster with the existing native checkpoint path.
  Release this source session only after its confirmed receipt and explicit
  transport closure. The source closes before recovery acquires its UUID authority.

Success/continuation:

- Start a fresh candidate/destination and two fresh ordinary client Games using
  the same configured real profiles. Observe AwaitingConfiguration with authentic
  unplaced/unloaded guests and no movement/action dispatch.
- Restore/install checked rows, place normally, serve recovered native chunks and
  observe both actual teleport and PlayerLoaded acknowledgments. Prove inventory,
  position, dimension and supported player/body/advancement/stat projections match
  the source rows without simulated readiness. Record both clients' loaded terrain
  and inventory observations, not only a server-side bit.
- Obtain ReadyFrozen. Confirm the server game clock, counter/slot and inventory
  did not advance during staging. Perform checked real continuation into its
  distinct archive while still frozen, validate that image with existing native
  checks, close after its receipt and prove all tracked threads/native/UUID leases
  release. Reopen/verify the continuation through the same real-client route in a
  separate finite case if needed to prove the output is consumable rather than
  only inspecting a receipt.

Finite pre-Ready failures each use a new destination: one missing profile, wrong
profile, duplicate profile, disconnect after Joined/after row installation/before
last acknowledgment, withheld PlayerLoaded, wrong teleport ID, gameplay/inventory
packet, and configured queue overflow. Each must refuse without running the
ordinary packet/leave handler, complete actual transport cleanup, preserve the
entire source archive and release only the unpublished candidate. A bounded poll
case proves no call drains an arbitrary backlog; accounting cases include error
and closed-writer paths so budget cannot leak or be bypassed through send_many.

Do not withhold PlayerLoaded by editing client/server readiness bits. A narrow
test-only transport packet gate may delay that actual outbound message, then close
or release it on the finite schedule; document the intervention. All successful
ready cases use the unmodified ordinary client emission. Negative prohibited-packet
injection is transport input to genuine connections, not a fabricated Guest.

A separate isolated post-Ready departure child establishes retained custody and
refusal as described above; record its expected external termination distinctly.
Fatal misuse-guard testing likewise requires an actual isolated abort/fatal marker,
not treating a timeout as proof. These are failed-process exception observations,
not passing finite recovery counts or normal close evidence.

## Subsequent accepted packets and limits

After this packet, adopt the real connected ReadyFrozen owner in the managed worker,
then place that supervisor outside the window Game and route ordinary close through
its nonconsuming request/status/retry API. Keep owners before lan_close, leave saves,
trial retirement or legacy Integrated::stop/Drop. The ordinary UI must reject normal
exit while retained and provide narrow status/retry. Its source scope is separate.

Before gameplay/filled ingress, settle live departure/replacement custody, offline
participants and transport liveness through an explicit reviewed adapter; also keep
the promised dedicated/two-client and in-frame local-owner work. The current
placed/loaded primitive and memory-only proof cannot stand in for those outcomes.

Normal protocol/refusal/close handling must preserve ownership as specified. Crash,
forced process termination, power loss and fatal programmer invariant failure cannot
preserve live memory. Their restart path selects and validates the newest complete
archive with existing strict refusal behavior; partial/pending evidence remains.
No new normal force-discard, external admission policy or owner decision is assumed.

Root must decide before source release: the exact helper factoring that preserves
all existing final correlations; the concrete staged mailbox/sender accounting and
three-handle closure implementation within these seven paths; and the finite source
command/client observation schedule supported by the pinned resources. If a proposed
implementation needs synthetic participants, clearing unsupported state, unbounded
queues, hidden detach or an unresolved ordinary failure return, revise and review
this design instead of weakening the claim.


## Coordinator engineering choices before independent review

Factor a private NativeRecoveryParts owner in session_checkpoint.rs at the existing
boundary immediately before install_session_checkpoint_participants. It retains
Box<Game>, verified image/rows/manifest, selected path, clocks and fresh counter/slot
receipt keys. Common preparation keeps all current physical/native/provider/package/
raid/coverage checks and advancement prefetch. A common consuming finalizer performs
all existing final player/counter/slot/raid/native correlations. The old complete
entries invoke the unchanged placed/loaded installer between these phases. The new
route invokes the separate sealed staged installer. Do not use an accept-unplaced
boolean or manufacture CheckedSessionPlayers outside complete verification.

Implement the opt-in transport inside Shared with an optional private staged state,
initialized only by Lan::start_staged. Keep NetConfig and ordinary constructors'
fields/contracts unchanged. Bounded mailbox permits charge before retention and
release on dequeue/drop; terminal slots are separate and fixed per connection.
PlayerSender's staged budget includes queued AND currently-written bodies until
completion/error/drop. A bounded encode/in-flight allocation is disclosed separately;
the queue budget is not a claim that arbitrary serialization allocates nothing.
Use no staged Holding/post-thread batching. Native play already joins its writer
before returning; track the outer local reader handle too and verify both completion
and sender-budget release before cleanup reports Refused/Closed. Require all three
possible staged connection handles to resolve, never only the successful profiles.

Use fixed offline fixture names MemoryOwner and MemoryGuest; derive UUIDs through
the existing native profile/login path. Source new-world configuration enables
owner commands and disables native spawn_mobs BEFORE any normal source tick; no
unsupported owner is cleared later. Both clients run their ordinary connect_local
and poll_stationary_client path (remote.rs explicitly sends no movement there).
Source setup alternates one bounded normal server tick with stationary polling for
each client, at most512 recorded rounds and within the120-second setup deadline.
After real readiness, the actual owner calls handle_chat_input with exactly
/give MemoryOwner minecraft:stone 3 and /give MemoryGuest minecraft:stone 5, sequentially.
Require actual server and respective client inventory observations after each before
freezing/capture; do not call the local run_command path or assign inventories.
The recovered staging phase has NO normal server ticks: only the closed allowlist
and existing native terrain encoding are pumped. Names/commands/tick indices and
observed cells/counts belong in the authored runtime receipts.

ReadyFrozen remains under the synchronous owning coordinator; it never escapes as
a freely droppable live-session handle. Only immutable observation, typed status,
explicit recapture/retry/close and transport pumping are permitted. No counter step
or inventory action follows recovery in this first packet. Late departure preserves
Guest AND GuestPlay and refuses ordinary release; the distinct isolated exception
proof must not be miscounted as successful normal shutdown. Normal live-departure
resolution, ordinary UI and TCP remain subsequent required S1 work.

## Review constraints retained for source and evidence

Independent design review found no blocking finding. Preserve these concrete traps:
Hub::next_batch needs a new initial-staging eligibility rule; never advance native
Hub/server ticks to satisfy placed_at+1. Staged sender charges remain held through
actual write completion/error/drop. The underlying ordinary memory pipe has an
unbounded buffer: this packet's mailbox/body budgets do NOT establish total
process-memory or pipe-buffer bounds. Track and close all three possible local
attempts, including rejected/pre-Joined threads, with native writer completion.
This acceptance releases only the seven-path design after prerequisite private
checkpoints; exact-source review and actual-resource qualification still apply.

## Native-close design delta for independent review

The seven-path draft cannot prove finite pre-Joined closure universally: native
ConnReader::recv loops internally and its read timeout resets on each read, so a
trickled incomplete frame can prevent the outer close check from running.
Expand this packet by exactly one path: crates/horizon-protocol/src/connection.rs.
Add a cloneable PipeShutdown capability with private incoming/outgoing Arc<Channel>
fields and shutdown only; no Read, Write, endpoint reconstruction or transport clone.
Pipe::shutdown_handle clones Channel Arcs rather than Arc<End>, preserving endpoint
last-clone drop semantics. Staged attempts retain the capability before reader spawn.
Global failure and explicit close snapshot at most three capabilities under the
stage mutex, release that mutex, then close channels. Remove staged string-based
timeout retries and retain normal native decoding. Future writes fail once channels close. The pipe buffer is unbounded before close;
after close a receive returns one bounded frame (wire maximum 2MiB minus one,
uncompressed maximum 8MiB) or drains a finite incomplete prefix and reaches EOF.
This does not bound the pipe buffer or change ordinary pipe behavior. Close/finished/
join still require all tracked readers, their joined writers, and zero body charges.
Also reject staged Lan::listen before TcpListener::bind, not only in listen_on.
This delta needs independent acceptance before the eighth-path source is written.

Independent native-close delta review: DESIGN DELTA ACCEPT for exact proposal4
d2e9f38d45cb139aad639afb96076a784def5e593559bbe2db89207a07caa213.
Registration/closing races, checks around receives, all-reader/writer joins and
zero charges remain exact-source gates. No total memory or uniform latency claim.
