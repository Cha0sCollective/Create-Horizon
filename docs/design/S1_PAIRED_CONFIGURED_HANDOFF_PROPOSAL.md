# Configured connection handoff into existing retained owners

Bounded deliverable 1 integration at engine 9bb5510c966e43659aa5f9273c19c01c61939697,
tree 623364687026f666da04d54084b60ed813756f12. Investigation only until independent
design acceptance, private assignment checkpoint and coordinator source release.
Prefix s1cfh1; proposed branch codex/s1-paired-config-handoff in the existing
.local/worktrees/s1-native-close-composition checkout.

## Objective and limits

Continue the completed strict configuration on the SAME actual connection into
Horizon's existing retained client reader/writer and server outer/writer actors.
Publish opaque, one-use configured metadata and consume it through its actual
retained Session or Hub before exposing Dynamic/player authority. This is a
dormant native integration service, not ordinary paired startup or C0/C1/G1/S1
acceptance. Ordinary constructors, legacy required-negotiation B refusal and all
ordinary activation/content/gameplay guards remain closed. Open to LAN remains
deferred. No new transport framework, public wire/schema/channel, supported world
profile, renderer, player-control endpoint or save format is proposed.

The existing strict core, native configuration codecs, actual counters and
shutdown handles, retained queues, actor registration, checked joins, Hub admit
and native play encoders are reused. No peer-provided configured Boolean,
disconnected success record or dummy connection can grant admission.

## Exact source budget

Seven existing production paths beneath crates/horizon-client/src:

- lan/login_negotiation.rs: consuming internal transfer and original live witness
- net.rs: explicit paired retained constructor, real actor path and Session consume
- lan/mod.rs: explicit paired retained policy, actual attempt binding/reservation
- lan/conn.rs: strict branch and opaque publication through existing actors
- lan/hub.rs: exhaustive legacy/staged refusal custody, no activation
- lan/hub/retained.rs: explicit paired constructor and consume-before-Guest
- game/remote.rs: explicit retained/legacy refusal custody, no Dynamic install

Two existing test paths: lan/login_negotiation_tests.rs and
lan/hub/retained_tests.rs. Existing inline tests in net.rs, lan/mod.rs and
lan/conn.rs may be extended. No protocol/data/Guest/Game schema path is released.
If this budget cannot preserve ownership or implement a meaningful focused test,
stop source expansion and specify the smallest amendment first.

## Transfer and original authority

Make NegotiationConnection's actual connection optional internally and add an
explicit Transferred state. Only the configured internal transfer may take it.
Validate role, exact expectation Arc, UUID, required-runtime Arc allocation,
Configured state, ORIGINAL absolute deadline, original Live and actual registered
connection-counter Arc before transfer. Refusal keeps the entire original scope;
no raw extraction escape hatch. All existing close/refusal/Drop behavior remains
closing, except successful transfer must not Drop-close the moved connection.

Transfer yields original Connection plus an opaque owner/witness pair with
private fields. They retain the ORIGINAL Live, expectation/runtime allocations,
deadline, counter allocation and native profile/information/registry/transcript.
No public claim constructor, Clone/serialization of a consuming envelope, or
Dynamic extraction before validated consumption. Borrowed consumption may leave
an empty one-use tombstone after success; failed/wrong-owner consumption leaves
the original envelope untouched. Inspection never establishes admission.

Three distinct lifetimes apply: configured scope owns unsplit connection;
transferred/pending original actors own the halves while metadata retains its
original deadline; established admission begins at one successful consumption.
Only established play is independent of the expired configuration deadline.
Original Live/runtime and actual owner closure still invalidate established
evidence. Actor-owner abandonment invalidates Live; dropping an observer or
refused envelope must not close a different genuine attempt.

## Client path

Use a separately named retained paired constructor with RequiredNegotiation;
reuse existing settings/target validation and native offline profile. Ordinary
retained constructors and legacy connect_required_negotiation remain unchanged.
Existing Controller::register retains the real c.reader.counters allocation
BEFORE negotiation. Run negotiate_client/configure_client in its existing reader
actor on the original unsplit connection, transfer only against those counters,
then start the existing retained writer on the original split half.

Install the original live owner under the Controller mutex before publication;
close winning registration, configuration, transfer or publication races refuses
admission. Publish a distinct feature-gated Event::PairedJoined opaque envelope.
Never substitute generic Joined. Retain refused publication/envelope/failure and
native handle custody in the actual existing state. Gate application play sends
until actual RetainedSession consumption; actor-owned automatic responses retain
their existing purpose. Consumption validates the actual Controller, counters,
required runtime/expectation, original live/deadline, reader/writer readiness and
one-use state under that owner's mutex, then yields checked metadata.

Controller stop and either actor terminal invalidate ORIGINAL Live immediately,
including panic, before queued proof can authorize installation. Do not turn raw
EOF/Receive/native IO into successful close. Reconfiguration/transfer cannot
silently reuse established paired evidence; refuse through existing custody.

## Server and Hub path

Add the real reader counter Arc to the registered Attempt; no protocol API is
needed. A separately named retained paired policy/constructor carries the exact
RequiredNegotiation. It must retain the actual sync owner. Place the strict
branch before existing B refusal and preserve B behavior for all old callers.
Use native offline profile, ClientInformation, brand and configured registry data.

Validate configured proof against this real RetainedContext/attempt/counters and
reserve UUID under the SAME existing registry mutex. Never use Shared's legacy
UUID-only reservation, replace a player, disconnect a genuine duplicate or let a
foreign attempt release another reservation. Configuration has already completed:
duplicate/full refusals use PLAY Disconnect or actual shutdown, not LOGIN packets.
Reservation is not Guest/native installation. Its release still requires BOTH
actual original actor joins, including pending/spawn-failure ownership.

Reuse play_retained's actual outbound queue, actor start/installation and read
loop. Publish distinct feature-gated NetEvent::PairedJoined with opaque metadata
bound to this attempt AND actual retained outbound sender. Publication must reject
close/terminal races while retaining the original rejected event. A terminal or
attempt interrupt invalidates only that attempt Live, never the shared required
runtime. Whole retained LAN close invalidates its required runtime before native
endpoint shutdown. Queued proof becomes invalid on either original actor terminal.

Hub's explicit paired constructor consumes ONLY its own PairedJoined. Validate
identity, counter/runtime/expectation, actual sender, reservation, live/deadline,
one-use state and existing Hub capacity/entity/spawn constraints before Guest.
Keep refused event at the original queue head. Make successful consumption the
admission linearization: close before it refuses; close after it invalidates live
evidence while preserving the already-admitted Guest and original custody.
Store established configured evidence with OriginalConnection for later separately
assigned native Game installation; generic matches_guest is insufficient evidence.
Ordinary retained Hub rejects paired envelopes; paired Hub rejects generic Joined.
Legacy/staged Hub and Game consumers explicitly refuse/retain unexpected paired
envelopes. No native Game or Dynamic authority is installed by this packet.

## Focused validation and stop conditions

Review exact source and qualifier before formatter/compiler/runtime. Root alone
runs serial writes/builds/tests. Use pinned toolchain/features/warm target and
fresh isolated fixture data. Reuse valid strict-core mutation/accounting results
unless the changed ownership dependency makes a focused rerun necessary. Do not
replay Step B windows or historical full batteries.

Exercise actual Memory and compressed loopback TCP through the retained client
reader/writer, server outer/writer and Hub, actual consume, native play traffic
and finished-only checked close. Assert actual counter allocation registration to
scope to split; wrong real Session/Hub/attempt refuses with original envelope
retained. Show transfer does not Drop-close via subsequent actual traffic.

Controlled existing test gates cover close before/after registration, pending
configuration/publication/consume, and original actor stop invalidating queued
proof. Assert no Dynamic/Guest before consumption; replay refuses. Expiry while
pending refuses, successful establishment survives crossing the old deadline and
continues actual play. Exercise real writer spawn refusal/panic after transfer,
retained original failures and no valid publication. Concurrent duplicate UUID
admits at most one without replacement or foreign release; no release until both
original actors join. Preserve ordinary/B refusal relevant negative coverage.

Use actual original IO and metadata, no synthetic success flags. An invalid or
missing assertion/zero selected tests does not pass. Different fresh FINAL/privacy
review before public sync; all changed paths and run-prefix artifacts preserved
before corrections. Frozen FINAL corrections need a new prefix. Checkpoint only
the accepted result; no PR/merge/release or owner scope/acceptance change.
