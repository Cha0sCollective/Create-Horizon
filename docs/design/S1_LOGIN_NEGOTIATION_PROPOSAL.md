# Bounded paired login negotiation proposal

PROPOSED; no source authority or admission/G1 claim. Research source is clean
engine173ad2a7b496b11b15e1492ad57c4ce4da9cf21d. Before implementation, finish its
frozen-window FINAL gate/private checkpoint, choose and record a clean accepted
successor, and obtain fresh independent Sol High acceptance of this exact design.
Implementation may follow the retirement component or use a separate branch only
after the coordinator verifies actual source disjointness and serialization.
Existing full-S1 direction remains; this packet does not reduce its outcomes.

## Concrete outcome and stop

Run an actual bounded login compatibility exchange on a live TCP connection,
after its existing online authentication or offline profile derivation and BEFORE
Shared::try_admit. Prove matching retained inputs, fresh connection/profile-bound
transcript agreement, typed refusal and owned cleanup. Introduce no player/world
mutation, registry publication, new Minecraft packet ID or default launch route.

This packet ends after agreement and an explicit startup refusal: paired
configuration/receipt consumption is not enabled. Successful negotiation is not
permission to call try_admit or send LoginFinished/RegistryData. Keep all existing
Packages::external_network startup refusals. A later independently reviewed packet
must validate full configuration, consume live receipts in Hub/remote and qualify
integrated LAN plus dedicated/two exact clients before covered refusals change.
No machine codec, journal, general protocol executor or save-compatibility promise.

## Existing seams

- game/mod_packages.rs:161 freeze_peer_inputs retains actual Packages/Catalog,
  exact Arc<SyncData>, actual immutable BlockRegistry and startup-derived contract.
- lan/peer_contract.rs provides unchanged WireContract v1 encode/decode/compare.
  Nine mismatch classes, whole-contract ceiling8192; the current grammar's valid
  maximum is2366. It is unauthenticated DATA, never local capture authority.
- lan/conn.rs:884 derives/authenticates UUID; :901 calls Shared::try_admit. Insert
  required negotiation between them. try_admit can evict an existing duplicate.
- net.rs:463 currently answers login CustomQuery with None; :422 accepts
  LoginFinished. Add an explicit required-negotiation path, leaving default paths.
- connection.rs:300 recv loops partial reads; current idle timeouts do not establish
  a total deadline. send_contents at336 uses write_all and needs a bounded variant.

## Retained expectations and trust boundary

Keep FrozenHostInputs binary-private and non-Clone, without a constructor from
claims/digests/paths or into_parts. Add one narrow method creating/caching the
transport expectation from this already validated holder. Retain the holder for
the entire attempt; replacing actual startup inputs requires a new attempt.

The transport's concrete NegotiationExpectation retains the SAME SyncData Arc,
actual static registry, actual selected PreparedBundle Arc and canonical contract
bytes. It has a fresh native contract-instance allocation identity. Capture must
check the selected prepared package/type/schema and the actual synchronized,
block, builtin, tag and feature projections against the frozen contract; do not
merely clone a decoded peer contract into local expected state. The executable
identity comes from the retained startup capture, never a claimed remote digest.

A library constructor may accept those native retained owners plus the binary's
captured contract as explicit trusted Rust configuration. It is not an unforgeable
host-capture capability or a writable-world grant; arbitrary native code is not
this threat boundary. The binary-only freeze method and its exact owner checks
establish provenance. No constructor accepting ONLY WireContract/bytes/digests is
provided, and no transport expectation can construct FrozenHostInputs.

The holder owns/caches one Arc<NegotiationExpectation>, so repeated method calls
preserve its instance identity. Expose only the narrow transport object needed by
connection/session code; no mutable package/registry/path access and no generic
host-construction trait, callback or Any owner capsule.

## Exact proposed exchange bytes

Use existing login Clientbound::CustomQuery and Serverbound::CustomQueryAnswer.
Query identifier: horizon:paired_login. This is a separate negotiation subprotocol,
NOT an extension of WireContract. Keep its encoding byte-for-byte unchanged.

All integers below are big-endian except the enclosing Minecraft packet fields,
which keep their existing native codec. A UUID is the existing authenticated/
offline u128 rendered as16 big-endian bytes, not a string. No normalization.
Every negotiation payload starts with exact8 ASCII bytes HZPLOGIN, u16 version1,
and u8 phase. No unknown versions/phases, padding or trailing bytes.

The server obtains server_nonce32 and a nonnegative31-bit query transaction q0
from existing crypto::random_bytes. Let q1=q0+1, except q0=i32::MAX gives q1=0;
they are distinct. Both are the existing CustomQuery transaction integers.
The client independently obtains client_nonce32. Entropy failure refuses; no
clock/RNG fallback, nonce reuse, cookies or cross-connection cache.

1. Server CustomQuery(q0, horizon:paired_login): phase1 prefix, profile_uuid16,
   server_nonce32, q1:i32, server_contract_length:u16, server_contract bytes.
2. Client CustomQueryAnswer(q0, Some(payload)): phase2 prefix, profile_uuid16,
   echoed server_nonce32, client_nonce32, client_contract_length:u16,
   client_contract bytes, transcript32. The client first strictly decodes and
   compares the server contract with its actual retained expectation. Missing or
   mismatching local input does not send a success answer.
3. Server CustomQuery(q1, horizon:paired_login): phase3 prefix, transcript32.
   Send only after validating phase2, q0, UUID/nonce, the entire peer contract,
   all nine component comparisons and the locally recomputed transcript.
4. Client CustomQueryAnswer(q1, Some(payload)): phase4 prefix, transcript32.
   Send only after exact phase3 confirmation. The server finishes only after
   exact phase4/q1 confirmation. No next-state packet is allowed in between.

Define session_id32 = SHA256 of the exact concatenation:

    ASCII("horizon-login-session-1") || 0x00 ||
    server_nonce32 || client_nonce32 || profile_uuid16 || i32(q0) || i32(q1)

Define transcript32 = SHA256 of the exact concatenation:

    ASCII("horizon-login-transcript-1") || 0x00 || session_id32 ||
    u16(len(server_contract)) || server_contract ||
    u16(len(client_contract)) || client_contract

Use existing sha2 SHA256 and native entropy, not a new dependency/cryptosystem.
Equality is compatibility confirmation, not attestation of an actively dishonest
peer's executable. Online authentication remains the existing profile boundary;
offline UUIDs remain offline identity. Nonces/transcript prevent stale/cross-profile/
wrong-phase acceptance; they do not make offline names authenticated accounts.

Enforce whole negotiation payload<=8192 BEFORE payload decoding, checked lengths
before allocations, inner WireContract ceilings and canonical decode/re-encode
identity. The four payloads are bounded under the existing2366-byte grammar; no
extra extension space is accepted. Exactly two queries and two answers occur.
Duplicate/unsolicited paired queries, None responses, wrong transactions, repeated
phases, alternate channel, early LoginFinished/ack/configuration/play packets,
truncation, malformed fields and trailing data refuse. Preserve unrelated legacy
query behavior only when no required paired attempt is active.

## Receipt and connection lifetime

Negotiation helpers return a non-Clone opaque AgreementReceipt with private
fields: endpoint role, exact expectation instance, UUID, session/transcript IDs,
and the actual attempt's connection-live allocation. No Default/Deserialize or
Boolean/hash-only constructor. Only the final successful live helper constructs
it. Client construction occurs after sending phase4; server construction occurs
after receiving and checking it. This is endpoint-local agreement, not distributed
proof that both application consumers have acted.

Attempt liveness is shared only as a private native identity/terminal latch. Any
read/write failure, deadline, shutdown or attempt termination invalidates the
receipt, including a receipt sitting in a queue. Endpoint expectations and the
connection allocation cannot be substituted at equal digest/UUID values.
Later consumption must be one-shot and match role/profile/contract instance plus
live connection. A receipt cannot independently grant Hub, player or world access.

In THIS packet a production required-negotiation branch sends the ordinary login
Disconnect after successful agreement, with a fixed diagnostic that paired
configuration admission is not enabled, then closes. A matching client likewise
must not publish Event::Joined or Dynamic. Tests can inspect endpoint receipts
while holding the actual connection scope, then deliberately exercise terminal
invalidation. Returning a receipt after shutting its connection is not a positive
live receipt. Do not retain a secretly usable admission capability across refusal.

## Total deadline and failure cleanup

Add narrow Connection::recv_until(deadline) and send_until(deadline, packet)
operations, leaving existing recv/send unchanged. Their helpers use Instant and
existing native frame/encryption/compression code. Recompute remaining duration
before EVERY underlying TCP read/write, check expiration within partial-frame and
partial-write loops and after completion; a peer cannot renew the deadline by
trickling bytes. Do not wrap an existing unbounded recv/write_all and call its
idle timeout a total deadline. Reject zero/expired or excessive coordinator timeout;
use one fixed attempt budget<=120seconds, not a fresh budget per phase.

Preserve prior socket timeout settings on every return; restoration failure is an
attempt error followed by shutdown, never permission to continue with altered
ordinary I/O policy. The connection is unsplit throughout this exchange. Existing
online encryption may already be active; negotiation precedes login compression
and LoginFinished. Existing decrypt/frame handling must remain intact.

The deadline limits application I/O waiting, subject to native scheduling and OS
behavior; it is not a hard wall-clock or total-memory guarantee. Existing generic
frame/decompression allocations precede the negotiation codec's8KiB check. This
packet does not claim smaller preallocation bounds or change global frame limits.
Actual trickled-frame and blocked-write controls must establish the implemented
bound rather than infer it from helper names.

Before negotiation succeeds, no reservation has been acquired: refusal must NOT
call Shared::release(uuid), which could release an existing genuine player. No
Duplicate event, eviction, guest creation or entity-ID allocation is allowed.
After later reservation support is introduced, only that actual reservation's
owner may release it; it is outside this packet's startup-refusing path.

Each attempt owns its connection and any local helper JoinHandle through actual
closure/completion; no timeout-based detach. Use direct connection helpers with
owned real loopback TcpListener/TcpStream handles for defining tests. Existing
ordinary LAN outer reader threads remain separately qualified: do not claim their
join merely from online==0. Any new attempt service must report terminal state
and preserve its concrete handle until explicit completion/join. Do not add a
general task executor or introduce a worker carrying mutable Game callbacks.

## Proposed eight-path source budget

1. NEW crates/horizon-client/src/lan/login_negotiation.rs: concrete expectations,
   fixed exchange/transcript, endpoint receipts and typed bounded refusals.
2. NEW crates/horizon-client/src/lan/login_negotiation_tests.rs: defining codec/
   transcript and actual owned TCP helper tests, declared by that module.
3. crates/horizon-client/src/lan/mod.rs: module and explicit optional required-
   negotiation policy, default None; native default constructors remain unchanged.
4. crates/horizon-client/src/lan/conn.rs: server hook after authentication and
   before try_admit, fixed post-agreement startup refusal and correct cleanup.
5. crates/horizon-client/src/net.rs: explicit required-negotiation connection path,
   strict login ordering and bounded outcome observation; default Session paths
   and login Event enum need not change. No paired Joined publication.
6. crates/horizon-client/src/game/mod_packages.rs: frozen expectation bridge,
   actual retained owner/instance checks and no external_network relaxation.
7. crates/horizon-client/src/game/peer_contract_snapshot_tests.rs: genuine capture/
   expectation ownership tests and integration through the concrete TCP seams.
8. crates/horizon-protocol/src/connection.rs: focused deadline-aware native I/O
   and inline partial read/write controls; no default recv/send change.

Before authoring, enumerate every full NetConfig/Session literal affected by new
fields. Prefer narrow constructors or separate internal outcome storage to avoid
changing existing Joined/Event representations. If actual constructor updates,
transport ownership, byte-boundary helpers or test visibility require another
path, report the exact blocker and obtain a revised bounded design before edits.
Do not bypass private receipt construction in tests to avoid mechanical changes.

No hub.rs, game/remote.rs, registry publication, integrated/dedicated launch,
protocol packet IDs, gameplay/renderer/save/SDK/H10 edits. Those omissions are why
external_network remains refused. A later complete admission packet will need
those concrete consumers; negotiation alone is not an external paired game join.

## Required proof and gates

Use actual independently captured local expectations, exact executable and
prebuilt packages; no claimed digest stand-ins for positive provenance. Root runs
all builds/tests/TCP processes serially in fresh owned cwd/home/data/config/cache/
temp roots, with sanitized sessions and no playable/account/reference access.
Rust tests spawn no compiler/Java/Python/test child. Existing native entropy/hash
and packet codecs are reused; no external service/authentication call is needed
for the bounded offline loopback cells.

- Independent literal byte/transcript vectors, all truncations, checked length
  and phase/version/channel/transaction/trailing errors; do not rely on self-
  roundtrip or both endpoints sharing the same faulty expectation encoder.
- Actual TCP matching negotiation, both endpoint receipts while sockets are live,
  then deliberate startup refusal with zero RegistryData/Joined/Guest/world access.
- Each nine ContractMismatch class using independently changed actual captured
  build/package/schema/registry/tag/feature inputs where representable. Malformed
  unsupported schema is a syntax refusal, not a successful mismatch comparison.
- Missing local/remote support, None/unsolicited query, wrong/reordered/repeated
  phases, replay across fresh server/client nonce, UUID, transaction and contract
  instance; early LoginFinished/configuration never publishes client registries.
- Disconnect before/after every transition, client phase4 write failure, server
  phase4 receive failure, stale queued receipt after terminal latch, count/state
  preservation of an already online duplicate UUID and a fresh exact retry.
- Real total-deadline proof with partial length prefix/body, trickled frames,
  stalled reads and writes. Observe each terminal result, actual close and owned
  helper reap. No missing-resource early return, leaked socket or sleeping detached
  worker can count as a pass.
- Verify default no-pair query-None/compression/known-pack/login/reconfiguration
  behavior unchanged and existing startup/world/build/package guards retained.

Root freezes the exact candidate and obtains fresh independent exact-source Sol
High clearance BEFORE compilation/execution. Preserve every scoped source and
packet artifact before corrections. Run focused feature/lint/format/protocol and
ordinary regressions appropriate to the change, then a DIFFERENT independent
FINAL evidence review. No design/source pass alone authorizes the next packet.

The next configuration component must send full registries from the same retained
SyncData, validate exact ordered groups/entries/NBT/tags/features before client
publication, carry live receipts through server Parts/Joined/Hub and client
Session/Event/remote, reject stale/disconnected consumption and unsupported paired
transfer/reconfiguration, then qualify real integrated LAN and dedicated/two exact
clients. H10 replication and the complete S1/G1 experiment remain required.
