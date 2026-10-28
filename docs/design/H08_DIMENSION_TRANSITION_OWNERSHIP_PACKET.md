# H08a13 synchronous dimension-transition ownership

Fresh independent read-only Sol High design review finds no blocking issue at
engine 1e1b2243934289d92a2193de88ec58311d0af069. H06b native and broad quick
checks pass; final receipt review is separate. Coordinator accepts the bounded
packet and frozen test seam; implementation still needs stable source review.
This unit repairs synchronous departure ownership; it enables no Game slot service
and makes no checkpoint, crash-durability, all-transition or G1 claim.

## Scope and observed boundary

Production scope: client src/game.rs and src/game/persist.rs. Test-only scope is
a module declaration in game/integrated.rs and new
game/integrated/transition_ownership_tests.rs, a descendant with access to the
existing private server_game constructor. Construct Options with an explicit
test-only struct literal; no production defaults/parser refactor or widened
constructor visibility. Freeze this seam in fresh design review. No SaveDir writer,
SDK, guest, schema, async-unload, slot/drop policy or normal-shutdown redesign.

save_dimension_entities has one caller: Game::switch_level. It currently captures
entity_chunks(keys, remove=true), resets EntityMemory, then waits for saving. The
old World/SaveDir remain if saving or subsequent target make_streaming fails, but
the native entities and metadata have already been removed. enter_dimension also
takes PlayerMotion before switch_level and currently loses it on an error.

Current SaveDir::queue inserts all records into pending before channel send and
keeps failed disk snapshots readable. Ordinary disk/flush failure is NOT immediate
snapshot loss. Removed native entities can produce an empty later snapshot for the
same resident keys, superseding failed NBT on direct producer retry or asynchronous
entity saving. A whole switch retry can stop at its earlier chunk-flush barrier
while the error is sticky; do not assume it reaches the entity producer. Target
preparation failure after successful saving has no such sticky-error protection.
These are source control-flow risks, not executed reproductions. Pending-lock
failure before insertion is a separate preacceptance boundary.

## Proposed synchronous ownership sequence

save_dimension_entities collects the exact resident coordinate set, captures with
remove=false, calls the existing synchronous SaveDir::save_entities, and returns
the captured set only on success (e.g. Result<HashSet<(i32,i32)>>). It must not
remove native entities, unknown records or reset EntityMemory while preparing or
waiting. Existing UUID assignments made by snapshotting may remain; preserve all
pre-existing UUID/raw mappings and stable identities across retry.

switch_level keeps that captured set while computing destination and preparing
the target streaming/storage/world. Only after departure saving AND every fallible
target preparation succeeds may it apply departure cleanup and replace the World.
Keep cleanup adjacent to that replacement, with no new fallible work between them.
Do not move removal merely to immediately after save_dimension_entities: target
make_streaming can still fail afterward. Preserve existing successful ordering and
target semantics, including coordinate scaling, shared clocks and watched state.

Factor the existing remove=true rules into one concrete private cleanup helper.
Include unknown-record removal as well as existing entity pools, rider/root-vehicle
exceptions, anon entries and UUID/raw removals. entity_chunks(remove=true) retains
its existing behavior for async unload by calling that helper at its old logical
boundary. Preserve the existing success-only full EntityMemory reset for dimension
departure; do not broaden which entities/chunks are selected or rewrite schemas.
Avoid a generic snapshot transaction, callback seam or second removal algorithm.

enter_dimension matches switch_level's result and restores the SAME taken player
on Err before returning it, like commands.rs::teleport_player_to_dimension does. On
failure, do not create travelling state, trigger dimension advancement or report
success. Preserve the current successful player handoff and portal semantics.

Include the nearby synchronous dirty-marker handoff in this packet: capture from
World.modified.iter(), retain the existing SaveDir::save call and its synchronous
Region/POI ordering, and clear only actually submitted chunk coordinates after it
returns Ok. Thus every failure, including partial acceptance, retains those markers;
unrelated/missing-chunk markers remain. This conservatively waits beyond acceptance
without a new SaveDir API. A later entity/target failure need not restore markers
whose chunk save already succeeded. This is not all-or-nothing world rollback.

## Validation using actual Game and entity methods

Use explicit ignored tests requiring pinned HORIZON_TEST_RESOURCES with expect,
never silent early success. Construct real Resources/PackResources and a headless
Game through existing loading code and integrated::server_game (silent audio,
no window/renderer). Options has no Default and Options::read consumes process
arguments: use only the explicit test struct literal with fixed names/paths,
empty packages and no inherited session. Do not call Options::read or parse test
runner flags. Build the test artifact without running it, then execute its ignored
tests from a fresh owned process cwd/HOME/data/config/cache/temp; the constructor's
temporary title SaveDir must also stay inside that cwd. Do not change global cwd
inside a shared test process. Drive actual enter_dimension,
switch_level, save_dimension_entities and entity_chunks, not mock closures or a
parallel implementation. Seed unknown entities through actual loaded-entity
ingestion rather than making EntityMemory internals publicly mutable for tests.

- Real entity-folder I/O obstruction: seed a resident chunk, dropped item with
  component-rich data, representative native entity, UUID/raw data and opaque
  unknown/anon tags, plus a real player. Fail actual enter_dimension/switch_level.
  Assert source dimension, native owners, pre-existing metadata and player remain;
  travelling/advancement success state must not appear. Inspect the accepted
  pending entity snapshot separately from native retention.
- Repair only the owned obstruction and retry actual save_dimension_entities
  directly, distinguishing it from a whole switch retry blocked by earlier flush.
  Because writer errors remain sticky, this may still return Err; require fresh
  entity readback after completed writes to remain nonempty with original IDs,
  components and opaque fields, never an empty delete. Keep native owners retained;
  do not clear or bypass the error to manufacture success.
- Successful departure save, then actual target-preparation failure: after Game
  construction, point only that Game's options.resources at an owned missing root
  and target the Nether so neither flat nor test-terrain bypasses VanillaTerrain.
  VanillaTerrain's cache is keyed by directory. Assert target worldgen failure,
  saved entity readback, retained source owners/player/metadata and no World swap.
  Restore that per-Game option and test a successful transition separately.
- Successful commit: exact selected departure cleanup/rider exceptions, target
  dimension/destination and player/travelling behavior match existing semantics.
  Include unknown records and entities outside captured coordinates in helper
  cleanup tests; whole successful departure retains its existing EntityMemory reset.
- Actual chunk-save failure retains submitted dirty keys; successful chunk saving
  clears only submitted keys and leaves missing/unrelated keys. Preserve existing
  save/generation regressions and async-unload behavior.

Run tests in sanitized owned homes/TMPDIR with session file unset; resources stay
read-only and playable worlds/Prism/reference sources untouched. Never mutate
process-global cwd/environment concurrently with other tests. Coordinator owns
serialized pinned formatting, targeted native/resource tests, feature-on/off lint,
frozen H05 full-counter integrated and in-frame games, and broad quick checks.
The repaired switch path is primarily in-frame; integrated regressions alone do
not exercise it. Stop/rescope if faithful actual-Game tests or retained cleanup
need broader production ownership changes. No new owner decision is needed for
this narrow correction. Async unload acceptance, shutdown-error lifetime, dormant
slot delivery, coherent recovery and live slot enablement remain separate packets.
