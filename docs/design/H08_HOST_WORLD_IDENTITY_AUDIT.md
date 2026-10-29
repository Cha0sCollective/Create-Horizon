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
