# H09/H10 peer contract source audit

READ-ONLY planning evidence at basec0ad9e0 during native-fence qualification. No
wire/schema/API adoption or source release. The fence five remain untouched by
this audit. H09 external admission should precede H10 replication; current local
package pins are not external negotiation.

Reusable source boundaries (relative horizon/crates/horizon-client/src unless noted):

- game/mod_packages.rs:17-101 retains captured catalog, exact executable SHA256
  and sorted package identities; check_world137/record_world184 govern local opens.
- horizon-mods/src/paired.rs:185-253 identity covers captured descriptor, both Wasm
  executables and content. Its source explicitly excludes peer admission.
- horizon-mods/src/runtime.rs:638,696,715,822,1133-1140 has typed preflight/prepared
  activation, side/running inspection and captured-owner counter evaluators.
- mod_content.rs:1456-1512 deterministic materialization/cache stamping exists.
  Cache stamp is NOT an accepted final network registry digest.
- lan/data.rs:60-71,126-190 SyncData has actual configuration registry mappings/
  packets; no complete frozen network registry identity currently exists.

Current barrier: Packages::external_network at104 rejects selected pairs; callers
game/remote.rs344, game/lan.rs1530/1572 and game/integrated.rs610 cover external
client, publication and dedicated network startup. Shared integrated memory data
does not establish independently captured peer agreement. Keep this barrier until
a separately accepted complete admission path replaces it.

Transport seams: lan/conn.rs779-884 sends registry packets847 then configuration
finish859 before Joined. Agreement must complete BEFORE registry send and Joined.
lan/mod.rs51-73 Joined has no validated paired receipt; NetConfig114 is an immutable
input seam. Existing LoginGate authenticates/access-checks profiles, not packages.
net.rs462 answers login custom queries with no payload;597-670 only handles brand/
quiet configuration payloads. Protocol login query/answer and configuration/play
CustomPayload envelopes exist; no new Minecraft packet ID is inherently necessary.

Smallest next design investigation: one selected generic package, SAME exact
artifact on server/clients, one current machine schema; complete package/content/
final-registry/network-contract agreement with bounded lengths/timeouts and
field-specific refusals. Internal validated state must precede registry/Joined;
retain no-pair behavior. Claimed fingerprints are compatibility, not remote code
attestation. A package declaration digest cannot prove final registry numbering.
Envelope/schema/state-machine choices require fresh reviewed packet acceptance.

Impact map only: game/mod_packages.rs, lan/{mod,conn,data}.rs, net.rs, required
game/{lan,remote,integrated,dedicated}.rs startup inputs, one small shared contract
module. Exact scope must be reviewed before release. No save/fence code changes.

H10 extraction seams: world/counter_restore.rs490-520/864-938 placement identity
and checked committed reads; world_slot_trial.rs62-78/376-394 and
world/slot_restore.rs883 slot revision/payload/residency/persistent context.
Counter/slot sampling needs an explicit authoritative boundary. game/counter_control
209-318 and game.rs5444 are commit/tail seams; never publish speculative proposals.
lan/hub.rs1419-1452 chunk tracking and Guest.sent_chunks exist, but machine
subscription generations do not. Session/PlayerSender/Hub transport and remote
packet/respawn/disconnect paths are reusable. Do not serialize runtime authority
keys as client mutation capabilities.

Later H10 outcome: one read-only replica, initial snapshot, base/revision checked
deltas, retirement/untracking, bounded resync and fresh session/subscription epochs.
Prove late join, committed state, abort silence, gap recovery, stale/duplicate
rejection, replacement/retrack/reconnect; integrated then dedicated/two exact
clients. Dedicated renderer allocation stays zero. H09 can overlap persistence;
H10 needs accepted H09/stable committed observations, not general-world recovery.
Ordinary activation is still unavailable; any closed fixture seam or activation
packet needs separate review. No current owner decision required for design work.

Root verified unchanged SHA256 anchors:

- game/mod_packages.rs e6cc4ec2b9efc70a3f77a57fe1112893e5a3d35f4e6f905982953fec5f85b05f
- horizon-mods/src/paired.rs 6ed423943eb8c5808693ef770df58af8e0ae3514588b9de611d990281d29f034
- lan/conn.rs a8dbca9aeb0600d477720161aee4c84dee5012c56f049438134d2823189b81c5
- net.rs d6fd47049f77ceff02356827352997084c51d6e89f0167b0ce623b92fdde03cf
