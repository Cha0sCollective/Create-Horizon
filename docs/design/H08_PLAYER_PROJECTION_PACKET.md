# H08 borrowed all-player native NBT and JSON projections

Accepted bounded design after a different fresh independent read-only Sol High
review with no blocking findings. Reviewed draft SHA256:
38c376e453816e756f178a73b4bf3455d1b47429615e7ba187f550eadef0e6ce.
Accepted engine base is
78c5a0d70824ce397d1a26bd6736a149000d99db, tree
13d6c9ad9d22f006f9e08b5883f38b56aebbc2eb, on
codex/h08-participant-preflight. Native/resource/broad and final independent receipt
qualification now passes at that accepted revision. The predecessor is
H08_PARTICIPANT_PREFLIGHT_PACKET.md, read together with the actual committed code.
This is a dormant PLAYER-only precursor; broader capture authority is not implied.

## One concrete objective and payload boundary

Add one synchronous borrowed Game method that rechecks the expected native roster
and idle player-owner state, validates the named count boundary, and returns one
existing-format player Tag plus advancement/statistics JSON Values for EVERY matched
guest. Include integrated owner and external players with their existing canonical
record mapping. Do not silently fall back to owner-only capture or skip a row.

Return structured Tag/JSON values, not encoded/compressed files or bytes. This
packet implements no NBT compression, JSON text protocol, directory layout, manifest,
publication, copying, recovery, transaction bus, generic projection framework or new
Game field. Existing ordinary writers retain their current behavior and formats.
No gameplay enabling callsite, SDK/guest API, network/control command, save cadence,
World slot enablement or public release is introduced.

## Exact source scope and practical native types

Production changes: game/lan.rs and game/stats.rs only. All added Game types and
methods are paired-packages-gated and visible at most within crate::game. Keep the
new player-row fields private; a future native consumer can be added with its own
bounded design. No widening of ParticipantRoster, ParticipantIdentity,
ParticipantStorage, record_uuid or private gameplay fields is necessary because
assembly lives in their defining lan module.

Proposed in-memory values in game/lan.rs:

- ParticipantProjections { players: Vec<ProjectedParticipant> }.
- ProjectedParticipant { conn: u32, uuid: u128, dimension: String,
  storage: ParticipantStorage, player: Tag, advancements: serde_json::Value,
  stats: serde_json::Value }.
- ParticipantProjectionError with finite causes: Preflight(ParticipantPreflightError),
  UnavailableAdvancementTree, UnsupportedPlayerData { conn },
  UnsupportedStack { conn }, and Reservation { conn }. Preserve the native cause
  category; no error string becomes a guest-facing protocol.

Reuse the EXISTING private, Copy ParticipantStorage variants:
IntegratedOwner { record_uuid: [i32; 4] } and External { record_uuid: [i32; 4] }.
This metadata observes existing file ownership; it is not a new storage schema or
an instruction to write a chosen path.

Proposed Game::project_checkpoint_participants(&self, expected: &ParticipantRoster)
-> Result<ParticipantProjections, ParticipantProjectionError>.

Proposed game/stats.rs::Stats::to_json(&self) -> serde_json::Value, pub(super),
extracting ONLY the borrowed JSON construction currently inside Stats::write.
Stats::write calls that same method, then retains its existing parent creation,
pretty serialization, write and contextual error behavior. Keep this helper
available in feature-off builds because the ordinary writer uses it. No Clone
implementation, trait, generic formatter, disk read or dirty/event draining.

Tests: new game/lan/player_projection_tests.rs, with its gated declaration in
lan.rs, plus focused tests in the existing stats.rs test module. Reuse the existing
TEST-ONLY integrated transition Fixture/new/game visibility. Preserve every
qualified preflight test/helper body and predicate. New child-test setup helpers
may independently construct ordinary native channel-backed guests, as in the
predecessor; do not widen its sibling test helpers just to share a fixture.

## One-call sequence and failure ownership

1. Call the actual preflight_checkpoint_participants(expected) FIRST, in this same
   synchronous call. Propagate its exact cause. Do not observe a replacement roster
   on failure, synthesize missing GuestPlay entries, normalize a menu, act as a guest,
   tick, poll a connection, wait/join a tree loader or call an external callback.
2. Require a nonempty current self.adv.tree and no pending_tree before producing
   JSON. The ordinary guest writer's empty-tree guard skips BOTH records; this
   diagnostic must refuse unavailable shared metadata instead of returning a row
   with silently missing JSON. Do not mutate/reload the tree to pass this check.
3. Before invoking any item encoder, validate EVERY participant's supported roots
   and recursive modeled counts as specified below. A present guest.saved must be
   a Compound; do not accept a malformed saved scalar/list and silently replace
   it with an empty player compound. Missing saved data remains ordinary None.
4. Borrow the actual host and matched guest/play records. Iterate the expected
   identities for stable row order, finding each actual guest by conn. The existing
   exact roster comparison makes duplicates, missing rows and canonical collisions
   refusals. No entry.or_default or missing-data repair is permitted.
5. Construct only local projection values, call the existing borrowed native
   serializers below, append each complete row, and return only the complete set.
   A later failure drops partial local projections; the live Game/Guest/GuestPlay
   owners, pointers, raw metadata and notifications remain unchanged.

Only existing known reservation failures are reported explicitly. Ordinary Rust
allocation failure/panic/abort is not newly claimed recoverable. Never consume an
original item to make allocation or encoder failure paths easier to handle.

The method borrows Game continuously; no owner-thread task/tick can mutate its
non-atomic player state between these checks and projection. Native pending byte
counters may change on transport threads; retain the predecessor's allocation-only
identity rule. No new identity ledger or session-authentication/replay guarantee.
Returned rows are detached observations, not authoritative item owners, revision
leases or a barrier that remains held after this call returns.

## Existing player serialization and canonical identity

For an external row, reproduce lan/playerdata.rs::save's exact LOCAL projection
construction: Level { player_uuid: Some(canonical_record_uuid), ..Level::default() },
then playerdata::store(&guest, &mut local_level), then local_level.player_tag().
This temporary value is required for compatibility: cloning Game.level for an
external guest leaks owner-only fields which store does not overwrite. It creates
no live default Game, Guest, GuestPlay, UUID, missing record or admission state.

For an integrated-owner row, clone self.level LOCALLY, preserve its already checked
canonical player_uuid, then call that same borrowed store and player_tag. Do not
call self.sync_level, self.store_owner or Level::write. In particular do not replace
the canonical player-file UUID with the owner's distinct account UUID. Preserve
fields the native owner writer intentionally inherits from Level; do not invent
new live body-field projection logic in this packet.

playerdata::store already borrows player inventory, armor/offhand, ender inventory,
effects, spawn/respawn, recipe book, and motion/saved-body observations into Level.
Level::player_tag rewrites its existing known player keys over a cloned saved
compound. Unknown player fields outside those rewrite keys and noncolliding opaque
component Tags follow the existing codec unchanged. This is existing native
serialization behavior, not a new promise to preserve arbitrary duplicate/shadowed
component encodings or every transient body field.

For every row call guest.advancements.to_json(&self.adv.tree, DATA_VERSION) and
play.stats.to_json(). Reuse record_uuid/ParticipantStorage from the already matched
expected/current native mapping. Do not parse a previously saved JSON/NBT file.
The guest record writer deliberately does NOT require Stats.loaded or
Guest.advancements_loaded: a newly admitted player's legitimate empty in-memory
records are still serializable. Preserve that distinction from local-player guards;
do not reject or manufacture a load solely because either flag is false.

Progress::to_json retains its existing criteria/date/done/DataVersion behavior,
including unknown advancement IDs with done=false. Stats::to_json retains zero and
negative i32 values, omits empty statistic-type maps, preserves all nonempty keys,
and writes the same DataVersion. Neither operation consumes network dirty sets,
scoreboard events, recipe unlocks, advancement changes, toast state or loaded flags.

## Named native count boundary BEFORE casts

The actual native item_nbt::encode_item writes count as Tag::Int(count as i32).
item_data::template_tag uses the same cast, omitting count only when it is one.
Player inventory, equipment and ender items all reach these functions through
Level::player_tag. Templates also encode nested container, bundle and pot stacks;
charged projectiles use item_to_nbt. Some(count=0) is unsupported, not an empty cell
and not silently normalized to None. Its native decoder otherwise clamps it to one.

Visit all present roots in Guest.inventory, Guest.armor, Guest.offhand and
Guest.ender_items. For each root require the same exact registered non-air root ID
boundary used by the qualified H06 native adapters, then reuse the UNCHANGED
slot_retirement::check_counts borrowed walk. It already checks root and the four
modeled child collections (container, charged, bundle_contents, pot_decorations),
positive counts <= i32::MAX, checked child-length arithmetic and try_reserve.
Map Reservation separately from unsupported count. No payload clone precedes this
walk. Do not add a second recursive count algorithm or modify its existing callers.

A positive over-stack-limit count within this encoding bound is not clamped or
redistributed. This method is not an inventory movement or native stack-limit
repair. Nested item IDs and arbitrary Tags inside opaque components are not
recursively reinterpreted or normalized. Count-one template omission is ordinary
codec output, not missing count data. The full write/parse byte codec's depth/size
limits remain unqualified because this packet returns Tags/Values, not bytes.

Other modeled non-count scalars (for example damage, repair_cost and enchantment
levels) have their existing native integer conversions. This packet does not claim
a universal scalar/component validator or lossless arbitrary-malformed-data round
trip. Those supported-value constraints must be refused or qualified in a later
full checkpoint admission/byte-encoding packet; do not use successful Tag projection
as evidence they passed. Preserve normal bounded scalar values in the compatibility
fixtures rather than changing shared codecs incidentally.

BundleSelected is a runtime-only native field whose equality ignores its raw
integer; the item save encoder does not persist it. map_post_processing is also
explicitly transient. Do not add either to the saved format. Tests must separately
prove original top-level and recursively nested BundleSelected integers remain
unchanged; they must not expect an invented serialized selection field.

## Actual-Game acceptance and independent expected values

Use ignored tests with pinned HORIZON_TEST_RESOURCES required by expect, a fresh
owned process cwd/HOME/data/config/cache/temp and session unset. No process-global
mutation. Reuse the native resource-backed Fixture and no-listener lan_host setup
(listen=false, announce=false, online=None). Use at least two actual placed and
loaded channel-backed Guest::new values with actual motion/play records; retain
receivers. Any as_guest setup finishes before projection, with the hub restored.
Keep the fixture's removed synthetic local body owned by the test. No mock roster,
mock projection callback or alternate ownership implementation.

- Two external players and integrated-owner-plus-external cases. Use distinct
  account/canonical owner UUIDs, rich DIFFERENT player state and inventory in both
  guests. Author explicit expected signed UUID arrays, slot numbers and item ids/
  counts, equipment names, ender Slot values, positions/dimension, selected slot,
  body fire/air/fall distance, spawn/respawn and opaque nested player/component
  fields. Verify exact mapping and one complete NBT/advancement/stats row per
  participant. Reorder the hub and require expected-order output.
- Seed self.level-only sentinel fields which store does not overwrite. Prove the
  external output uses ordinary external defaults rather than leaking that owner
  state; owner output preserves the native owner baseline. Do not claim that these
  unmodified writer semantics newly qualify all live-body fields.
- Independently hand-author expected advancement JSON: known satisfied and
  unsatisfied requirements, explicit criterion dates, unknown ID done=false and
  DataVersion. Use the existing test Tree helper or a locally authored native Tree.
  Independently author expected statistics JSON for multiple types/players,
  zero reset, negative values and empty-type omission. Do not merely compare two
  methods which now call the same new Stats formatter.
- Retain old ordinary-writer compatibility coverage: Stats::write must still emit
  the old pretty JSON layout, paths and errors. Add a manually specified expected
  JSON text fixture (with the existing DataVersion substituted) independent of
  Stats::to_json, plus the existing real obstruction/repair tests. Legacy NBT and
  advancement writers may be exercised separately in owned test directories for
  compatibility, but are not the only oracle for the new method. The projection
  call itself must not change any file bytes or create a directory.
- Cover every persisted root collection and all four modeled nested collections.
  Zero/i32::MAX+1 counts at root and each nested location refuse before projection;
  positive i32::MAX is emitted exactly, and count-one template omission is checked
  against an explicit expected Tag. None stays absent. Supported over-limit counts
  remain exact. Unknown/air root IDs and malformed saved-compound shapes refuse.
- Test absent/pending advancement tree refusal and legitimate per-player loaded
  flags false with explicit in-memory progress/stats. No GuestPlay default is made.
- After obtaining an expectation, separately introduce a cursor/menu, replace a
  sender allocation, remove/change a participant, or add pending transition state.
  Call the NEW projection method directly and require the original preflight cause.
  The test must not call preflight separately and assume its result remains valid.
- Before and after success and refusal capture raw metadata allocations/native
  selections, inventory/equipment/ender state, body fields, Guest.saved, Level,
  advancement criteria/change/unlock queues, statistics values/events/dirty flags,
  play state and disk tree. Defining-module statistics tests inspect private dirty
  state without exposing it. A returned projection can be changed or discarded
  without changing any live player. Repeated inspection does not clear retry or
  notification state and cannot return a partial player set.

Coordinator owns formatting, feature-on/off lint and meaningful ordinary/resource
regressions, plus a different fresh source/receipt review. Existing writer guard,
failure and retry tests remain. No Game/client performance, encoded-size/depth or
compressed byte-equivalence claim follows from these structured-value tests.

## Hard next dependencies and stop condition

Stop when the small production change and actual-Game/compatibility checks above
are qualified. No owner decision is required for this dormant successor. No
permanent owner-only/server-only S1 scope or relaxation of conservation is accepted.

Matching players and projected Tags/Values still say nothing about whole-World
pending retirements, parked levels, off-resident/native/opaque entity owners,
World-slot lifetimes, earlier failed Left records, reconnect history, producer
completion, save errors, all-level flushing, immutable copying or recovery. These
remain hard dependencies before a checkpoint may be published. No global owner is
consumed or declared safe by this method.

A later reviewed synchronous owner operation must call the real roster/preflight
and projection together with those global predicates, then keep the owner occupied
through supported producer completion, checked player/world/metadata saves,
all-level SaveDir barriers, synchronized physical immutable copy and manifest-last
publication. Returning these observations releases no persistent barrier because
none is created. server-hold continues control work and can pump pregeneration;
Integrated::call's ten-second timeout does NOT cancel its queued/running task.
Neither mechanism is silently promoted into a capture guarantee.

The server plus two external paired clients, interrupted publication/recovery and
cross-owner conservation fault proof remain part of later full G1 work. Public
engine publication remains separate. Sources used here are the inspected Horizon
checkout and its existing native serialization seams; no Create source or assets
are incorporated into the proposed generic engine changes.
