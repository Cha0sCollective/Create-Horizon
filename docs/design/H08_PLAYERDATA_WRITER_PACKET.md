# H08a12 checked borrowed network-player NBT writer

Design draft after H08a11 engine ecb93cc57f749037dfdb8e26ccad0123e1ba5240
(tree a872296669da48fe499f8abb6534d094cb14f663). Fresh independent read-only
Sol High design review accepted the design without blocking findings before
implementation. Source/native/both-mode/broad and final independent receipt review
now pass at f7f8990; see ../reference/H08_PLAYERDATA_WRITER_RESULTS.md. This packet exposes
existing synchronous playerdata write failures; it does not retain retired players
or establish a checkpoint, recovery, item-conservation or G1 claim.

## Bounded source change

Own exactly these engine files:

- crates/horizon-client/src/lan/playerdata.rs: checked writer and native tests.
- crates/horizon-client/src/game/lan.rs: existing save_guest caller handling only.
- crates/horizon-client/src/game/dedicated.rs: existing shutdown caller handling only.

Change the existing library signature to
`pub fn save(guest: &Guest, world: &Path) -> crate::Result<()>`.
`crate::Result<T>` reexports horizon_client_core::Result<T>, currently
`std::result::Result<T, Box<dyn std::error::Error>>`. Do not change that alias,
add Send/Sync requirements, or introduce an asynchronous writer API.

Preserve the existing projection: temporary Level with guest UUID converted by
uuid_ints, store(guest, &mut level), then level.player_tag() through write_dat at
the existing canonical players/data/<uuid>.dat path. Check mkdir and write_dat
results and contextualize every returned failure with the full canonical target
path. Return Ok only after write_dat succeeds. Borrow Guest immutably throughout.
Keep load/apply/store/path, NBT fields, compression, numerical conversions and
write_dat's existing _new/_old replacement behavior unchanged. No generic I/O
framework, schema, strict loading, atomicity or fsync policy change.

Game::save_guest retains its existing owner branch: a placed owner is copied into
self.level, and only the external-player branch calls playerdata::save. Explicitly
log a checked external-player error and continue to save_guest_records. Keep its
existing JSON error handling and order. stop_dedicated explicitly logs each direct
playerdata failure, continues remaining players and the separate JSON loop, then
attempts the existing kicks and world save in their original order. Do not insert
question marks or change either caller's return type. Preserve all current guards,
including the library writer's absence of a placed guard; add no remote/owner gate.
Integrated shutdown, common metadata, lan_close, HostEvent::Left, menus and
admission are outside this change.

## Meaningful native checks

Use unique resource-free temporary directories and native Guest/Level/NBT types,
without a renderer/Game fixture, closure fake, sockets or prepared game resources.

- A real file obstructing players/data must return a full-target-path error;
  repair only that obstruction and retry the same borrowed guest successfully.
- Write a baseline player record, obstruct its canonical <uuid>.dat_new with a
  directory, change borrowed guest state, then require Err and unchanged baseline
  bytes for this specific pre-open obstruction. Repair and retry; parse the actual
  gzip NBT and require the updated projection. No general old-byte preservation
  promise follows from other write/rename failures.
- Verify canonical UUID path and serialized UUID, position/dimension, inventory,
  armor/offhand/ender items, stack components and representative opaque saved NBT.
  Compare the source guest's relevant state before/after failure and success.
  Include empty/reset inventory over a previous record and ordinary replacement;
  do not remove normal _old backups to make multiwrite tests pass.
- Preserve existing playerdata tests. Source review must verify both direct
  callers handle Result, later JSON attempts remain reachable after NBT failure,
  and later players/kicks/world-save attempts are unchanged. Native library tests
  establish the checked writer, not the runtime teardown continuation by themselves.

Coordinator serializes pinned Rust 1.99.1 formatting, targeted playerdata and
relevant binary tests, feature-on/off checks and Clippy, public-tree checks and
the appropriate broad quick pipeline. Freeze the reviewed candidate and run the
inherited full native-counter smoke tests in integrated and in-frame modes using
fresh owned homes and reviewed sanitizing drivers. Those game regressions are not
external-player NBT fault coverage: normal in-frame/local and integrated-owner
playerdata uses Level::write instead. Do not modify qualified drivers or claim
their success exercises this newly checked external-player writer.

## Stop conditions and remaining ownership gap

Stop and rescope if implementing this needs retained Guest/Level snapshots, a new
Game field, retry latch, admission/reconnect policy, menu normalization, all-active
player saving, shutdown redesign or changes outside the three files. Review the
stable implementation independently before qualification; no public push or gate
promotion is authorized by this packet.

HostEvent::Left already receives a removed guest; after save_guest, its guest and
play state are discarded. lan_close can discard the entire host, and server Game
drop ends any borrowed-state retry opportunity. Common saves omit active external
guests. Cursor/menu ownership and retired-owner state can also escape the current
projection. Logging and a borrowed Result cannot repair these boundaries. A later
retained immutable player-record/normalization packet and coherent capture boundary
must address them before H08 can certify participating item owners. This precursor
needs no owner decision about broader retention or supported multiplayer scope.
