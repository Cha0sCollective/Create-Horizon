# H08 host world identity source audit

SOURCE AUDIT ONLY, not implementation release or design acceptance. Finite native
read-only audit against clean LOCAL engine bb6f718cecb41ff0ffd563699dd3d731f9b3fe53,
tree a243ac6bd646ed8152eaa5ebe6be846c1f0cf878. Root confirms the shared preparation
and package-admission source. Current advance and next used-streamer qualifications
remain separate; no ordinary activation/capture/recovery is accepted here.

## Existing authority

Paths relative to horizon/crates/horizon-client/src:

- game.rs8814/8865/8872: in-frame request/load/finish_open_world.
- game/integrated.rs305 and game/dedicated.rs563: authoritative load_world.
- game/world_loading.rs41: shared WorldRequest::prepare. Package admission precedes
  upgrade/migration; native SessionLock follows them. Package recording and native
  Level/world/streaming creation follow the lock. PreparedWorld carries the lock,
  path, Level and owners, but no durable machine-world UUID.
- save.rs2826/2925: Level models no world UUID. Tolerant Option-returning Level::read
  and default/coercing fields cannot serve as strict identity admission. Preparation
  may treat a failed read as a new Level. Level::write3360 preserves unknown Data
  fields, but generic write_dat2492 uses replacement/backup semantics.
- game/mod_packages.rs137/184: bounded closed build/package record checked before
  world changes, create_new/sync_all under the native lock. Its v1 schema is exact;
  adding world UUID would change compatibility, not merely reuse a spare field.
- world/counter_restore.rs626/638: current initializer allocates a fresh UUID per
  domain. restore_counter_frame671 requires independent expected_world first.
- game/levels/counter_restore_integration_tests.rs108 retains authority from the
  original live key. Child plans convey that independent retained value.
- game/levels/capture_integration.rs37 identifies executable/resources/content,
  not a durable world identity.

Folder, LevelName and seed are copyable/renameable; player UUID is player identity.
SessionLock establishes exclusive open lifetime, not durable UUID (fixed snowman
bytes). Observation world_instance is process-local allocation. Candidate counter/
slot frame UUID is an assertion to validate, never its own expected authority.

Pinned source SHA256:

- world_loading.rs 08b2b851c9c5d8bdd3106db104116c44662487d82e3c7f2dcba71c368de9d41f
- save.rs 688ae37142aca312fec253d815fb4af8dfb014aad5d808d4b23b08f6aa6abe04
- mod_packages.rs e6cc4ec2b9efc70a3f77a57fe1112893e5a3d35f4e6f905982953fec5f85b05f
- counter_restore.rs 289ac07b80603d46a0729a9e480a1240dde7bee12db34447989a2c422111ffa5

## Proposed smaller prerequisite

Consider an immutable bounded native host identity file for explicitly opted-in
NEW worlds, allocated before machine records exist. Strictly reopen it without
regeneration, repair or adopting candidate bytes. Validate before upgrade/migration/
world writes, recheck under SessionLock, then retain it through PreparedWorld into
actual authoritative Game. No ordinary machine activation or capture admission yet.

A separate record avoids coupling to tolerant Level decoding/replacement or package
v1 build admission. It is local provenance, not protection against replacement of
all host files. Whole-directory clones retain logical identity unless an explicit
later fork operation supplies another identity. Recovery must retain the original
host context BEFORE selection; reading the copied candidate anchor as expected
would be circular and is forbidden.

A fresh proposal/review must specify explicit new-versus-required-reopen intent,
missing-record refusal, duplicate/type/size/link/nonregular handling, checked I/O,
create failure, retained lock/context lifetime and all old unopted behavior. A
missing optional file alone cannot prove whether identity had been required before;
do not claim global missing-anchor detection without independently carried intent.
Native initialization still generates per-domain IDs, so merely transporting an
anchor does not solve activation. Dimension identity, initialization adaptation,
migration and recovery-target adoption remain separate contracts.

Likely bounded future paths: game/world_loading.rs, a strict identity child module,
game.rs for transport/lifetime and a focused actual-startup test child. Verify exact
visibility, construction, failure and cleanup paths before fixing scope. Prove actual
shared new/reopen preparation and authoritative integrated/in-frame transport plus
mismatch refusal before copy. Retain current native/actual-Game/streaming checks,
fresh both-mode ordinary GUI and broad qualification for production lifecycle changes.
No owner product choice is needed merely to draft and review this prerequisite.

## Concrete ordering question for the proposal

Root identifies a package-admission interaction in the source above: check_world
treats ANY non-session.lock entry as existing when package pins are absent. If a
new separate identity anchor is written before record_world, new paired-package
admission may reject its own root as an unmarked existing world. If identity is
created after package recording, identity-create failure may leave package-pin
side effects even though no playable level was published. A proposal must resolve
this ordering and partial-failure contract explicitly, including unpaired worlds.
Do not silently extend package v1, ignore arbitrary new files, claim zero writes
before all checks or promise atomic rollback. This is a source-level dependency
question for a reviewed future scope, not an owner blocker or current scope change.

## Strict native-header preparation audit

Fresh finite read-only preparation audit against cleanb0f1bf6 confirms existing
public APIs suffice inside the proposed child; no parser/save.rs change is needed.
save.rs2357 read_dat uses unbounded fs::read, gzip decoding capped64MiB but raw
fallback on ANY decompression error, default NBT limits, ignores consumed tail and
returns Option. Required-reopen admission must bypass this tolerant helper.
horizon_world::DATA_VERSION reexports the current constant5023 (lib.rs126,
horizon-core/version.rs11). Public Compression::Gzip.decode, nbt::read_named with
explicit Limits and nbt::write_named are available to the new child.

NBT reader checks available bytes/array counts before allocation (nbt.rs181), list
node budget and remaining bytes before capacity allocation (253-269), bounded
wire-string length (197), and depth/node budgets (229). Defaults are64MiB/1million
nodes/depth512. Compounds keep last duplicate (294); some list wrappers unwrap and
negative float zero normalizes. Decoded tags alone therefore do not prove unique/
canonical wire encoding. Native re-encoding equality is a POSSIBLE restrictive
check, not an already accepted general-world rule; native-writer roundtrip and
unsupported readable inputs must be explicit before adopting it.

Candidate limits for independent review:1MiB compressed and inflated level.dat,
16384 NBT nodes, depth64. Require exact gzip success with no raw fallback, bounded
regular read and exact consumed inflated length. Candidate native header matches
Level::write3456: empty root name/Compound, Data Compound, DataVersion Int5023,
Time Long, LevelName String, GameType Int, Version Compound with Id Int5023.
Unknown bounded Data fields may remain preserved; this is not a complete Level schema.

Refuse migration layouts independently of version: upgrade_older_world170 is
tolerant/coercing; migrate_legacy195 relocates root region/entities/poi and DIM-1/
DIM1 even at current version. Data.Player/WorldGenSettings and obsolete keys listed
in Level::write3370 identify legacy conversion inputs. Required reopen retains and
compares exact preflight bytes under lock, then demands Level::read success without
fresh-world fallback. Bounded level.dat does NOT bound/strictly validate every
saved-data/player startup input; do not extend the claim that far.

Additional sourceSHA pins: nbt.rsa32fd0ac8fe94ab1b23c000b51c3a81b28d40c518c5befb5272818792d1cf1ac;
compression.rs4db73e6ca9a0d552d415ce60e7e88b89c9cf2b7d060a669ff8328f518316e101;
version.rs7a6cd0ca8f69b22d70ffc63e16e0b2872f3b128b25deb6604aa493d5ca2177ef.
Original save/world_loading pins above remain unchanged. These are source facts/
recommendations only; limits/canonical policy still need reviewed acceptance.

Independent concrete-policy review finds a native counterexample to canonical
equality: Level::write3477-3478 preserves spawn.yaw/pitch f32::to_bits(-0.0), while
NBT reader245/249 normalizes signed zero; setworldspawn may supply that rotation.
Proposal642220da is NOT accepted. Minimal revised policy omits equality, adopts
native-reader normalization/last-duplicate-wins for level.dat and claims only
bounded typed current header, not canonical/duplicate-free wire. Strict duplicate
rejection remains on closed identity JSON. Require native -0.0 positive control,
preserve original bytes under lock and never rewrite merely to gain admission.
Fresh exact revised-policy review remains required before implementation release.

Revised policy2cfa88adb700954a8510640ee73ed170a00b64c921a330495ef49450302026f8
now has fresh independent DESIGN ACCEPT, no findings: native normalization is
preserved, original-byte checks remain, bounds/header APIs fit five paths and no
extra native-header wire-uniqueness rule is required. Accepted clean successor
input is b0f1bf6/tree015636 after used-streamer FINAL ACCEPT. This is design/source
preparation, not implementation/runtime or G1 acceptance.
