# Live admission seam audit (not source authorization)

Bounded read-only audit against connected branch base23e09dd, current source1dffa36f.
Current real memory clients use PRECONFIGURED inputs. H09a/H09b.1 are accepted data
foundations, not live negotiation/authority. No current source or policy is changed.
Exact independent design/source acceptance is required for a following S1 packet.

## Existing concrete seams

- game/mod_packages.rs: Packages::freeze_peer_inputs retains actual Packages,
  Arc<SyncData> and static BlockRegistry in binary-private FrozenHostInputs.
  A narrow trusted transport expectation/receipt boundary is needed; a decoded
  WireContract is not validated-host authority or a substitute for retained owners.
- lan/peer_contract.rs: WireContract from_claims/claims/encode/decode/compare,
  nine mismatch classes,8192-byte codec ceiling,2366-byte grammar maximum. Equality
  remains data only; no production freeze/decode caller currently negotiates it.
- Server lan/conn.rs: authenticate/offline UUID first, agreement BEFORE Shared::try_admit
  (which can replace an existing duplicate profile). Existing login CustomQuery/
  CustomQueryAnswer envelopes can carry the exchange without a new Minecraft ID.
- Server configuration must use exact retained SyncData, require agreement before
  first registry packet and full configuration completion before Parts/Joined.
  registry_packets(known_pack) omits NBT on known-pack true; paired validation cannot
  silently inherit that omission. Current client answers empty known packs.
- Client net.rs currently accepts LoginFinished, answers queries None, overwrites
  duplicate registry groups through Dynamic::add and ignores tags/features.
  A paired path must require agreement and validate exact ordered groups/entries/
  data/tags/features incrementally before Event::Joined/registry publication.
- Hub Guest/entity creation and game/remote registry installation need live receipt
  consumption. A queued Joined after disconnect cannot create authority. Reconfiguration/
  transfer remain refused until separately covered.

## Gaps and proof boundaries

No challenge/session/UUID/transcript binding or nonserializable connection-live
receipt exists. Cookies or equal decoded contracts do not grant admission. Keep
actual retained owner provenance in the binary while crossing the library seam.

Ordinary channels remain unbounded. Current staged64-event/256KiB-in/16MiB-out/
three-attempt recovery limits are not live-admission bounds. Native recv loops
partial reads; an idle timeout is not a TOTAL negotiation deadline. Native framing/
decompression can allocate roughly2MiB/8MiB before the contract decoder's8KiB gate.
PipeShutdown interrupts native memory readers but neither bounds pipe buffers nor
implements a total TCP negotiation deadline. No general memory-bound claim.

H10 initial machine snapshots/deltas/session-generations/gap recovery remain
separate. Vanilla player terrain/startup acknowledgements do not prove machine
replication. Do not relax external-network refusals based on current memory proof.

Meaningful future checks: independent server and both-client captures, zero server
registry transmission on agreement rejection, zero client registry publication,
zero Guest/machine-snapshot admission; each mismatch, malformed/replayed/reordered
exchange, trickled frames, disconnect at every transition, stale queued Joined,
finite cleanup/fresh reconnect. Then external integrated LAN and dedicated TCP with
two exact clients. Native headless Game connect_local/poll_stationary_client provide
real fixture seams; synthetic agreement values alone cannot qualify them.

This audit found no unavoidable owner-policy decision. Ordinary lifecycle/worker/UI,
Creative attributes and live admission/replication remain distinct required S1 work.
No public release, schema promise, broad framework or later-stage authority follows.

## Follow-up source map at173ad2a (research only)

The smallest useful next component is live login negotiation before profile
reservation, followed by separately reviewed configuration validation and receipt
consumption. Existing default network behavior remains closed for paired packages.

Current source locations: Packages::freeze_peer_inputs game/mod_packages.rs:161;
WireContract codec lan/peer_contract.rs:160/207/235; authenticated profile
lan/conn.rs:884; Shared::try_admit call:901; duplicate-profile eviction
lan/mod.rs:535; client query response net.rs:463; first server registry:1038;
server Joined publication:1188; Hub admission lan/hub.rs:1181; client Joined
net.rs:505; remote registry installation game/remote.rs:865. Native
connection.rs:300 loops partial reads and lacks a total negotiation deadline.

A candidate bounded service would retain actual endpoint expectation, profile
UUID, fresh connection identity and phase in an AdmissionAttempt. Successful
live confirmation alone constructs a non-Clone/nonserializable AgreementReceipt;
decoded contract equality cannot construct it. A later consumer must verify the
same live connection and retained input instance after full configuration. Either
endpoint's termination invalidates the attempt. Existing WireContract bytes stay
unchanged. Challenges/transcript binding remain unadopted protocol work requiring
an exact proposal; do not append fields to the strict contract decoder silently.

Approximately eight paths: new lan/admission.rs and focused TCP tests, lan/mod.rs,
lan/conn.rs, net.rs, game/mod_packages.rs, frozen-input tests and narrow
horizon-protocol/src/connection.rs deadline support. This is not a source budget
authorization. Preserve ordinary/default login constructors and behavior.

Refuse mismatches, malformed/replayed/reordered exchange, missing support and total
deadline before try_admit: zero registry transmission, Guest/entity/player/world
admission and replacement of an existing profile. Release only reservations
actually acquired. Reap attempt-owned transport work; observer timeout alone is
not cleanup. A subsequent exact fresh connection must succeed.

Do not relax Packages::external_network in that first component. The next must
force full registry NBT, incrementally validate exact groups/entries/data/tags/
features, reject unsupported reconfiguration/transfer and consume receipts at Hub
and remote installation. Exercise independent TCP endpoint captures, all nine
mismatch classes, trickled frames, stale/replayed challenges, disconnect at every
phase and queued Joined after socket death. Combined integrated/dedicated with
two exact clients remains required. Research by read-only s1cw_lan_author.
