# Strict paired configuration core

PROPOSED, no source authority. Parent accepted/public
21471ef835da8a3d462186de29be20a5d916b9fd, tree
7b5d86c3a6580a02db75e882f110a155285765dd. This is a bounded component toward
adopted deliverable 1. It does not adopt the older full C1 proposal or its topology.
Open to LAN remains deferred; ordinary paired activation stays closed.

Proposed prefix s1cfg1, branch codex/s1-paired-config-core, same physical engine
worktree and warm target used serially. Three existing source paths only:

- crates/horizon-client/src/lan/login_negotiation.rs
- crates/horizon-client/src/lan/login_negotiation_tests.rs
- crates/horizon-client/src/lan/data.rs

No Session, Hub, Game, protocol codec, renderer, transport-stop or new module
is released. If a necessary seam does not fit this budget, obtain a bounded design
amendment; a broader framework/profile/acceptance change returns to the owner.

## Concrete result and ownership

Extend the existing login service with a distinct paired-only strict configuration
transition. Existing B diagnostic callers still negotiate and intentionally refuse
at CONFIGURATION_DISABLED. Do not change default constructors or ordinary guards.

Inputs are the actual NegotiationConnection, its real AgreementReceipt, the same
endpoint/expectation/UUID binding and concrete native profile/client-information
inputs. Validate the actual receipt before advancing Agreed into configuration.
Do not use AgreementReceipt::consume's returned transcript data as authority.
Validate bounded native profile/option inputs before changing the connection.
The server's native LoginFinished UUID must match the agreement; the client checks
the actual native login profile against its expected profile. Authentication remains
upstream of this service; this packet adds no account/authentication mode.

Return an opaque non-Clone ConfiguredEndpoint that owns the ENTIRE original
NegotiationConnection, actual Live allocation, same absolute deadline, checked
profile/client information, session/transcript binding and private Dynamic.
No raw connection extraction, splitting, dummy transport, public claims constructor
or configured Boolean. Its close/Drop invalidates the same original Live/transport.
Before any later admission, its local liveness includes original runtime-open,
role/expectation/UUID allocation and the unchanged deadline. It grants no Guest,
world access, Session Joined event or writable authority.

Refusal preserves the original scope in a concrete failure owner and its actual
reason. Close the failed scope using the existing close semantics; no result may
abandon an unresolved acquired owner through an unchecked transfer. No fallible
transport-extraction seam is needed: whole-value movement is sufficient here.
Later actor handoff and post-transfer invalidation need a separate exact assignment.

## Native wire sequence and validation

Keep the existing channel, WireContract, login nonces/transcript and MAX_BUDGET.
One absolute deadline covers existing negotiation, every partial IO, native phase
transition, final acknowledgment and unconsumed configured handoff. Never renew it.
Established play lifetime is a later consumer's concern; this component cannot
issue an established-play receipt or remove its pre-admission deadline.

Use the same native Connection and existing login/configuration packet encoders:

1. Continue native LoginFinished/LoginAcknowledged. TCP-only native compression
   may be selected using the existing Compression packet/state; Memory remains
   uncompressed. Do not replace the connection or discard buffered/cipher state.
2. Use a fixed explicit native prefix: server minecraft:brand, exact enabled
   features and native core SelectKnownPacks; client ClientInformation,
   minecraft:brand and an EMPTY SelectKnownPacks answer. Include the existing
   native quiet-command indicator only as part of the reviewed fixed Horizon
   brand prefix. No new channel/schema or permissive unrelated-packet loop.
3. Send/validate EVERY registry_packets(false) body from the actual expectation's
   retained SyncData, full NBT present, in native order; then exact native tags,
   FinishConfiguration and one exact acknowledgment.

Derive expected packets only from the retained expectation, not substitute tables
or remote hashes. The bounded amendment adds ONE configuration-specific capture
wrapper beside data.rs's existing borrowed Budget/Measure code, returning the
existing RegistrySnapshot. Borrow actual private Entries JSON BEFORE native
projection or capture's native hashing allocations. Charge Tag nodes, list storage,
compound entries, UTF-16 keys/strings, registry-entry/name vectors, private Dynamic
and map storage, overlapping moved entries/vector capacities and simultaneous
expected/native/peer buffers. Checked charges plus bounded profile/prefix reserves
feed the existing capture aggregate policy; compare its unchanged snapshot with
the retained contract before registry_packets(false). Ordinary capture behavior
and digest construction stay unchanged; expose no private JSON/mutable budget.

Public Dynamic currently stores None payloads and cannot measure private JSON
shape. A fixed wire-buffer reserve cannot replace this variable allocation
preflight. Existing frame bounds can supply genuinely fixed buffer reservations.
This is conservative admission accounting under the existing 128 MiB policy,
not a hard process/RSS or underlying Pipe-backlog guarantee. No new format or
general resource framework is proposed. Prefer per-packet comparison over another
full wire cache. Validate packet ID and full payload before tolerant decoding
or Dynamic::add. Strict expected sequence catches duplicate/missing/reordered
groups/entries, absent NBT, changed tags/features, unknown packets and trailing
bytes; no fallback IDs or set-only/digest-only authorization.

Build Dynamic privately from checked expected entries. Incremental private
construction is allowed after each exact comparison; nothing is published until
the complete sequence and final acknowledgment succeed. All later failure leaves
that partial Dynamic private. Native ClientInformation uses existing codecs plus
bounded field/order validation and exact no-trailing-data checks. Profile/brand,
known-pack and acknowledgment packets likewise require complete checked bodies.
Resource-pack requests, links, transfer, reconfiguration and extra side exchanges
are outside this fixed core and refuse rather than being ignored.

## Focused validation and stop condition

Use existing captured-input fixture and complete pinned native resources, with
independent server/client captures and actual Memory/TCP Connections. Reuse the
existing test module and native codecs. Required positives: full strict Memory
completion, TCP completion including selected native compression, private Dynamic
and metadata match actual retained inputs, liveness and explicit close invalidate
the exact configured owner. A constructed wrapper without complete IO is not a pass.

Focused negatives cover original deadline expiry across phases, wrong role/UUID/
expectation/live allocation, actual runtime close, wrong native profile, duplicate/
missing/reordered registry or absent NBT, changed features/tags and trailing bytes.
Use a few grouped concrete mutations and distinct positive/asserted observations;
do not multiply a portable fuzz/fault matrix. Keep unchanged B refusal and one
ordinary native configuration regression. Missing resources do not pass or skip.
Focused inline data.rs checks additionally establish refusal BEFORE projection
for compact/node-heavy JSON and aggregate retained-copy/buffer exhaustion, and
unchanged accepted snapshot digests. No broader registry/codec rewrite is released.

Fresh independent exact DESIGN review precedes a private source-assignment
checkpoint and Root release. Preserve all three sources/prefix before changes; fresh
SOURCE/tool review before formatter/compiler/runtime, different FINAL/privacy
before public sync. Use the pinned toolchain/warm target serially; focused checks
and actual IO cases, no Step B or historical full-battery replay.

Completion is this dormant strict-core component only. Ordinary net/conn/retained
Hub consumption, actual C0 startup/native authority/checkpoint/departure wiring,
normal controls and precise gameplay admission remain required before ordinary
paired activation. Full C1, G1 and S1 remain OPEN. No extra diagnostic endpoint,
release or later-stage gameplay is authorized.
