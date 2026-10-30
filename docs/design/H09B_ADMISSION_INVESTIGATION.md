# H09b admission investigation

READ-ONLY planning at engine ea96036 while H09a is qualified; NOT API adoption,
source release or external-admission acceptance. H09a candidate remains dormant.

Use existing login CustomQuery/CustomQueryAnswer before try_admit, not a new packet
ID. try_admit may evict a duplicate profile before configuration (lan/conn.rs668-733).
Hub consumes Joined and allocates Guest/entity/world authority (lan/hub.rs643/1140),
so validated receipt consumption there is unavoidable. Client net.rs currently
answers custom queries None, accepts Dynamic groups (duplicate replacement), ignores
tags/features and publishes Joined without paired validation; remote installs it.

Smallest useful phases, each requiring its own accepted packet/source review:

1. Freeze actual admission inputs and adopt a separate small bounded wire codec;
   existing refusal unchanged. Own exact Arc<SyncData>, immutable actual block
   registry, retained capture/build/current schema and new contract-instance ID.
   Do not expose H09a internal encoding as a wire format accidentally. Proposed
   wire bounds: one pair/schema,32-byte digests,1024-byte IDs,8KiB envelope; not yet
   accepted constants. No generic networking/normalization framework.
2. Negotiate authenticated connection before try_admit using fresh cryptographic
   server/client challenges, fixed transaction phases and transcript confirmation
   bound to profile UUID/contract/session. Mint private nonserializable receipts
   only after full confirmation; no Boolean/digest-only constructor, downgrade,
   cookie inheritance or stale-session reuse. Still retain startup refusal until
   all consumption/configuration paths are complete.
3. Server sends full configuration from frozen inputs. Client incrementally validates
   exact registry group/entry order/data, tags/features and bounds before publishing
   Dynamic/Joined. Reject missing/extra/duplicate/malformed/noncanonical data and
   paired play reconfiguration until separately supported. Carry connection-live
   receipt through Parts/Joined/Hub and Session/Event/remote; validate before world
   mutation or client registry publication. Only then replace covered startup
   refusal callers with required admission policy, never local equality permission.

No-pair login/query-None/known-pack/reconfiguration behavior remains unchanged.
Required paired client refuses LoginFinished without agreement. Missing pair at
either endpoint refuses. Equality is compatibility assertion, not remote attestation
against an actively lying endpoint. Keep world/package/build/check_running guards.

Frozen owner facts: Shared/Hub Arc<SyncData> has no relevant interior mutability;
runtime BlockRegistry is immutable leaked reference; Catalog is Arc-retained.
game/packs.rs staging reuses assets/facts; Assets::reload retains registry, so no
blanket pack ban is needed if these exact owners remain pinned. Replacing actual
inputs must close/restart session. Disk edits cannot alter retained capture.

Proof must include real external integrated LAN and dedicated+two independently
captured exact clients using one exact executable. Observe negotiation, full config
validation, receipt consumption and successful ordinary join/play, not memory-only
equality. Individually change actual package/build/schema/registries/data/tags/features;
exercise missing/extra profiles, malformed/oversized/version/trailing payloads,
wrong/reordered/repeated transactions, replay across sessions/UUIDs and disconnects
at every transition including queued Joined after socket death. Tampered config
after claimed equality cannot publish client registries or admit a server Guest.
Boundary assertions: zero RegistryData when server negotiation fails, zero client
publication, zero world admission; cleanup then fresh reconnect success.

Likely core source: game/mod_packages.rs; lan/{mod,conn,data,hub}.rs; net.rs;
game/{lan,remote,integrated,dedicated}.rs; new shared module under lan declaration.
Joined field changes require enumerated mechanical constructor updates across
existing fixtures; never expose production receipt construction to avoid that work.
Renderer/save/fence/SDK/H10 remain excluded.

A real TOTAL negotiation deadline is not supplied by idle read_timeout: existing
ConnReader::recv loops partial reads. If required, review narrow deadline-aware
connection.rs receive/send support while leaving old recv unchanged. Application
8KiB parsing does not bound earlier frame/decompression allocation; any stronger
preallocation claim needs an explicit narrow reader limit. No hard scheduler/I/O
bound claimed. Exact scope and constants remain proposals, pending H09a acceptance.
