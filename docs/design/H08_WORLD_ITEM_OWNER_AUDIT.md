# H08 WORLD item-owner audit

Status: PROPOSED source audit only. No implementation, independent design acceptance,
checkpoint admission, G1 claim or reduction of the required S1 owner scope follows.
This audit uses Horizon sources only; no Create reference informed it.

## Inspected context

Private branch `codex/h02-smoke-h03-notes`, HEAD
`fda69a4ab244c73ab72faedd5b900c025b3bdb89`, initially clean. Read effective AGENTS,
CURRENT_WORK, HOST_CONTRACT, H08_GLOBAL_OWNER_AUDIT and accepted
H08_ENTITY_OWNER_OBSERVATION_RESULTS. HOST_CONTRACT's historical engine baseline
remains `cf62fe270b243780f9aa86c49b456d9dd9078b42`; this audit inspects the qualified
local engine, not that old baseline.

Engine branch `codex/h08-idle-close-restoration`, HEAD
`aa170bc4fd3214790a2363732d0c7cdd57ba6451`. The only observed engine changes are
`crates/horizon-client/src/game/levels.rs` and untracked
`crates/horizon-client/src/game/levels/idle_close_tests.rs`, the coordinator's
separate idle-close candidate. Do not treat that candidate as qualified here.
Only its save/close boundary was relevant: save each selected level, restore the
original dimension, then retire/remove a successfully closed owner; failed and
unprocessed owners remain native. Qualified `save_parked_levels` also restores the
original dimension with first-error precedence. Neither helper is an inspection
iterator; the proposed observation must never enter/swap/save/close a level.

At final context recheck, the coordinator had committed that isolated idle-close
change as `b41427c6b20d0a7458a3de730dc532d3a68d4081` and the engine was clean.
This audit does not independently qualify that commit. The private HEAD remained
`fda69a4ab244c73ab72faedd5b900c025b3bdb89`; an unrelated coordinator result note
`docs/reference/H08_IDLE_CLOSE_RESTORATION_RESULTS.md` appeared untracked and was
preserved. The one authored output is this audit.

All paths below are engine-relative. Line anchors describe the inspected source;
the methods/fields are the durable lookup keys. No runtime, imports, builds, tests,
formatting, network, engine edits or commits were performed for this audit.

## Findings: current owners outside PLAYER and E1

`game/levels.rs::Parked` owns a separate `World`, `Ticks` and `level_events` for each
parked dimension; `swap_level` exchanges these with Game. Borrow active and every
parked record in levels.rs. `Game.pending`, `Game.spill`, `Game.brushing`, function
state and mod queues are not Parked fields and are not swapped: they are separate
Game-level owners, not evidence that parked levels are empty.

| Defining source and exact owner | Lifecycle and implication |
| --- | --- |
| `world.rs:70` World.block_entities; `block_entities.rs:47` BlockEntities.map | Native live block entities at arbitrary positions, including synthetic vehicle-menu keys. Insert/load creates the live map owner; removal/unload retires it. Scan the entire map, not just resident chunk keys: `save.rs::chunks_with_block_entities` filters to the requested columns, so off-resident map entries can escape that save snapshot. |
| `block_entities.rs` private out: FxHashSet<Pos> | `take` removes an entity from map and marks its position out; `put` returns it. An empty map does not establish no borrowed/local block-entity payload. `tickers` can sweep/clear out, so invoking it to inspect would destroy precisely the evidence sought. |
| Same type: added: Vec<(Pos,u32)>, pending: FxHashMap<Pos,u32>, tickers/live, freed | Pending ticker bookkeeping is not a second inventory. It does establish unresolved deferred activation, including canceled added entries. Inspect only borrowed presence if refusing it; no promotion, next-token allocation, sweep or repair. Live ticker positions alone must not be counted as additional stacks. |
| `chunk.rs:152` Chunk.block_entities and Chunk.entities | Staged decoded block entities and raw entity Tags arriving with a chunk. `streaming.rs:802-811` takes them into World.block_entities/loaded_entities before resident insertion on the normal handoff. A direct/native intermediate chunk can still hold them; inspect every resident chunk's vectors instead of assuming that handoff happened. E1 already refuses World.loaded_entities, but does not inspect these chunk-local vectors. |
| `chunk.rs:361` ChunkExtras.block_entities: Vec<Tag> | Unmodeled block entities, including raw inventories, decorated-pot item/sherds/loot and suspicious-block item/loot. `spill.rs::saved_entity`, `remove_saved_entity`, `put_saved_entity` are the raw-storage path; they clone/take/mutate and are unsuitable observer APIs. The proposed scan can borrow the raw vector's presence without parsing or normalizing it. |
| ChunkExtras.block_entity_sources: Vec<(Pos,Tag)> | Original modeled block entities, retaining fields not modeled in the live Rust value. `save.rs:1144-1155` overlays those originals back into save output. They are retained source data, not additional physical copies of live inventory to sum; ignoring them loses opaque payload coverage. |
| ChunkExtras.rolled_loot: Vec<RolledLoot>; RolledLoot.items: Tag, table, seed, pos | Horizon rolls container loot on generation/load; while live contents equal the remembered Items, RolledLoot::restore writes LootTable/Seed instead. A remembered roll is provenance/template data, not a second physical inventory. Empty live slots alone cannot certify this owner absent or loot-generation behavior closed. |
| ChunkExtras.root: Compound; saved: Option<Arc<ChunkExtras>> | Decode keeps arbitrary root keys outside GENERATED_KEYS. A syntactically non-item-looking chunk is not a complete semantic proof about unknown NBT. Presence of the Option alone is not an item, but an observer that has no root-byte/semantic support must report nonempty opaque root explicitly, including no recognized block entity. Do not canonicalize or deserialize/re-encode to decide. |
| Chunk.postprocess: Vec<[i32;3]>; ChunkExtras.block_ticks/fluid_ticks | `postprocess.rs::postprocess_chunks` waits for lit chunks and neighbors, takes marks, syncs tick containers, runs shape/fluid behavior and processes resulting effects. SavedTick lists are pending registration/unpack, not completed work. They carry no ItemStack directly but can lead to removal/drops/other producers. |
| `world.rs` watched_writes: Vec<(Pos,u32,u32)> | Deferred notifications consumed by native sculk/block-entity synchronization, not item payloads. Do not clear them to make a scan pass. Their presence prevents a claim that all downstream native world reactions are complete. |
| World.moving_pistons and moving_piston_fallback | Collision/render shape copies made from Ticks.moving_pistons by Game::sync_moving_pistons. Not independent inventories. Active piston state is in Ticks; a stale render fallback alone must not be counted as another physical item owner. A conservative current-state diagnostic may identify/refuse these shape buffers without pretending they are items. |

`ChunkExtras.block_entity_order`, `block_names`, chest_openers, dirty/light/mesh logs,
POIs, inhabited time and shape cache entries are not themselves stack inventories.
Do not count every retained metadata row as a conserved item. Opaque data remains
unsupported until byte support and its effect on restore are established.

### Exact native block-entity families

The exhaustive enum is `containers.rs:75` BlockEntity. A borrowed match must cover
all variants explicitly. Neither `contents()` nor `slots()` is a complete census:
`contents()` clones payloads and returns an empty Vec for the spawner/vault/hive
families; `slots()` omits even Campfire, Jukebox and Lectern.

- Direct stored stacks: Chest(Vec<Option<ItemStack>>), Furnace.items[3],
  Brewing(BrewingStand.items[5]), Jukebox.record: Option<ItemStack>, Lectern.book:
  Option<ItemStack>, Crafter.items[9], Campfire.items[4], ChiseledBookshelf.items[6].
  The Chest variant represents several native container kinds through block state
  and slot count; BlockEntity::id() is not an exact chest/barrel/dropper/shulker
  identity. An empty slot vector does not establish the correct size/block identity.
- Vault.items_to_eject: Vec<ItemStack> is an actual pending ejection owner, popped
  from the end. config.key_item and display_item are rule/display stacks, not extra
  transferable inventory. `game/vault.rs::vault_events` turns Eject(stack,motion)
  into Game::spawn_item after native state mutation.
- TrialSpawner.dispensing: Option<Vec<ItemStack>> is cached rolled ominous items and
  their weights (counts), explicitly not saved. It is a production template, not a
  pile of physical dropped items. next_spawn_data: Option<SpawnData>,
  ejecting_loot_table: Option<String>, normal/ominous ConfigSlot.saved: Option<Tag>,
  config.spawn_potentials, loot_tables_to_eject and items_to_drop_when_ominous retain
  opaque or future-producing state. `game/trial_spawner.rs::tick_trial_spawner`
  creates local planned mobs, ejected stacks and TrialEvent::ItemSpawner work.
- Spawner.potentials: Vec<(SpawnData,i32)> and private next: Option<Next> retain
  spawn/equipment data. SpawnData.entity: Compound and equipment: Option<Tag> can
  carry equipment/inventory/passengers; no currently spawned Mob is needed for
  this producer to exist. Do not try to inspect private next from Game.
- Beehive.bees: Vec<Occupant>, Occupant.entity_data: Tag retains a full opaque bee
  record. Release reconstructs an entity; dropped hive copy_components can retain
  the bees in a future item. This is not an ItemStack-free safety proof.
- Banner.components: Compound, Skull.profile/custom_name: Option<Tag> and
  Sign(SignData) retain component/text/profile payloads that can affect copied
  dropped items. They are metadata, not extra physical stack counts. Unknown bytes
  and components cannot be certified by an inventory-slot-only scan.
- CommandBlock.command plus powered/auto/condition_met and native scheduled ticks
  can run arbitrary world/item commands; there is no ItemStack field to find.
- Sculk, CreakingHeart, Beacon, EndGateway and DaylightDetector have no direct
  stored ItemStack field in this enum's structs. Their active effects or later
  block destruction can still change world state and eventually create items.
  Their existence is not proof of a pending transfer, nor proof of completion.

`game/machines.rs::lend_items/reclaim_items` temporarily lends vehicle inventory to
synthetic Chest entries in World.block_entities. `game/storage.rs` similarly uses
an ender-chest placeholder for the open ender inventory. PLAYER refuses relevant
open menu/player owners and E1 refuses boats/carts, but a leftover synthetic map
entry must not escape merely because its position is not a resident real block.

## Pending outputs and effects

`game/spill.rs:30` Pending has five separately meaningful maps: shulker
HashMap<Pos,Vec<(u8,ItemStack)>>, pot HashMap<Pos,[Option<ItemStack>;4]>, banner
HashMap<Pos,Banner>, names HashMap<Pos,(String,String)>, copied
HashMap<Pos,(String,Compound)>. `record/record_saved` retain copied drop data;
`attach` consumes selected entries into a matching future dropped item; `clear`
removes leftovers for that block. An entry with an empty payload is still pending
ownership/bookkeeping. Names/copied/banner are future item components even when no
explicit ItemStack exists. Refuse nonempty maps without attaching, clearing,
encoding components or resolving saved data.

`Spill.spawns: Vec<Spawn>` owns stack, position, optional motion and pickup delay.
`Spill.removed: Vec<(Pos,BlockEntity)>` owns removed native entities until follow-up;
`Spill.orbs: Vec<(position,u32,i32)>` owns experience side effects, not item stacks.
`GameLevel` borrows both Pending and Spill. Game::with_level flushes spill on return;
`flush_spill` takes it, spawns items/orbs, then closes removed-container/jukebox UI.
Do not flush to normalize a diagnostic. Nonempty removed/orbs must not pass just
because spawns is empty.

`Game.level_events` and each Parked.level_events are Vec<LevelEvent> (`horizon-world
level/mod.rs:219`). Inspect all variants, or more safely refuse any queued event:

- DropResources{pos,state} evaluates block loot; FallingBlock{pos,state} creates an
  entity that can later drop; PrimeTnt{pos,rot} creates an explosion producer.
- Player{pos,state,effect}: GiveItem transfers or pops overflow. DropItem and
  InteractLoot are normally intercepted immediately by GameLevel::emit into Spill;
  they remain effect variants and must not be silently treated as impossible data.
  Sleep and RespawnAnchor can explode blocks in the wrong dimension.
- BlockEvent{pos,state,kind}: dispense, command_block and brushable dispatch to
  producers without carrying a stack. Hatch/portal/spawn events produce entities.
  GameEvent{pos,event,state} feeds sculk reactions. GrowTree/PlaceFeature are
  mutation request variants, but the inspected Game::process_level_events currently
  ignores them (and unmatched BlockEvent kinds); do not claim a completed feature
  implementation or invent item effects for those current no-op arms.
  BlockEntityEvent is block-entity callback work.
- LevelEvent{id,pos,data} and Sound are mostly presentation notifications; no
  need to declare them item payloads. Initial refusal of every queued variant is
  justified by unresolved effect ordering, with no invented special safe whitelist.

`GameLevel::emit` ordinarily resolves DropResources, pop_block_item, DropItem and
InteractLoot inline. That optimization does not make an externally/native-populated
queue harmless. `process_level_events` takes the whole Vec, then loops with fallible
branches (`dispense_from`, bed/anchor behavior). The remaining
local iterator is invisible to a scan of Game.level_events and can be dropped after
an error. This is a source-visible loss opportunity, NOT an executed historical-loss
finding in this audit. A later empty queue cannot prove those effects completed.

## Deferred producers and completion: separate unresolved dependencies

1. `horizon-world/level/ticks.rs::Ticks` owns blocks/fluid LevelTicks, updater,
   moving_pistons, piston_pushes, block_events, handling_tick and capture:
   Option<Vec<CapturedTick>>. `pistons.rs::BlockEventData` carries pos/block/params;
   MovingPiston carries moved_state/direction/progress/identity, not ItemStack.
   Pending piston movement can still break blocks and issue resource effects.
2. `horizon-core/ticks/mod.rs::LevelTicks` private chunks, containers_to_tick,
   to_run, already_run, to_run_set and collecting matter. count() sums only
   ChunkTicks::count across chunks; after begin_tick moves due ticks into to_run,
   zero count is not an idle/completion proof. ChunkTicks.pending: Option<Vec<SavedTick>>
   is not visible through queued(). Do not pack/unpack, finish_tick or poll to inspect.
3. NeighborUpdater private stack/added/count (`level/update.rs:97`) holds Shape,
   Simple, Full and Multi continuations. It is private to the world crate. A native
   read predicate in its defining module, composed by Ticks, is required before
   claiming absence of in-progress neighbor propagation. A `Game` accessor cannot
   legally reach these fields, and empty public block_events does not cover them.
4. `Game.brushing: brush::Brushing` maps positions to private count/reset/cooldown/
   hit_face, while item/loot lives in raw saved entities. It is Game-global, not
   Parked. Brush completion creates a stack; reset ticks also change dusted state.
   Refuse nonempty bookkeeping rather than advancing/resetting it.
5. `functions.rs::FnState` owns schedule: Vec<Scheduled{id,time,seq}> and private
   load_pending. Even an empty schedule can leave minecraft:load pending; tick-tag
   functions are recurring producers. tick_functions removes due entries before
   running commands and clears load_pending before callbacks. Its defining-module
   borrowed predicate must distinguish these pending/recurring sources; no function
   execution or schedule IO belongs in a diagnostic. Function tables are programs,
   not extra inventories; do not recursively interpret them to infer item safety.
6. Under mods, `ModsState` owns events, block_writes, private commands and quiet;
   horizon-mods runtime owns private Mods.out, per-call State.out and WorldSlot.
   Output::Give, Command/QuietCommands and SetBlocks can make items/world effects.
   server_mods::LentWorld temporarily owns active AND parked ChunkMaps outside Game.
   A boolean mods.world denotes capability, not current lending/completion.
   A defining-module borrowed check (and runtime seam if later claimed) is needed;
   do not take_world, run callbacks or route outputs merely to inspect.
7. Streaming retains Job::Decorate/Finish/Save and Done::Generated/Proto/Decorated/
   Saved, protos, deferred results, failed_finished and worker/channel ownership;
   these may contain block entities/raw entity records before resident activation.
   busy() omits saving, written, generation_save_error and failed_finished. Even
   a borrowed present-state summary is not an IO/writer barrier. SaveDir barriers
   cover queued writes only; retain the earlier global-owner audit's producer and
   writer requirements. Refuse cmds.pregen.is_some() even while paused; do not pump.
8. E1 already refuses falling, TNT, ominous spawners, mobs, projectiles, carts,
   boats, tridents, rockets, memory/portal owners and the other named families.
   Those protections must remain separate and unchanged. SpawnerState.displays
   caches Mob values for display; those are not fresh physical mobs to sum. Local
   spawner Planned/ejected vectors and other taken payloads exist during callbacks
   even when their destination vectors are empty. Read-only access does not itself
   establish a permitted between-callback boundary.

PLAYER already refuses pending save/entry/world/rest loaders, actual guest turns,
menus, carried/crafting/ender/player scratch state and nonidle guest inbox/play
state. That does not cover these WORLD queues. Integrated::call timeout still does
not cancel the operation; neither tick freeze nor a server hold establishes
exclusive producer completion. Future mutable ingress, callback-local payloads,
worker ownership and writer/error lifetime need their own definite boundary.

## Smallest useful next surface: a dormant WORLD refusal complement

Recommend a reviewed W1 packet that answers only: at this borrowed instant, are
any of the named resident/retained WORLD owners or immediate effect queues present?
Like E1, this is a detached diagnostic, not a token, barrier or permission to save.
Return dimension/count-only success rows and finite typed refusal causes; never
return item payloads. A clean result must be named narrowly; PLAYER+E1+W1 is still
NOT a global checkpoint readiness result because the dependencies above remain.

The smallest honest W1 can refuse every live native BlockEntity (including empty
containers and harmless variants) until a later borrowed typed-payload observation
is justified. This avoids a fake `slots().is_empty()` whitelist, and catches
producer-only spawners/command blocks without exposing their internals. Refuse
BlockEntities.out first, then pending ticker metadata, then any live map entry.
Scan every resident Chunk for staged block_entities/entities, raw saved block
entities/sources, rolled_loot, nonempty opaque root, postprocess and saved ticks.
An empty Some(ChunkExtras) alone need not refuse; no parsing/normalization is needed
for those named fields. Inspect World.watched_writes and identify active piston
shape buffers conservatively. Do not mislabel these data as item totals.

At Game level inspect every active/parked level_events vector and all five Pending
maps plus all three Spill vectors. Refuse global brushing, item_columns.is_some()
(a hopper-pass marker containing indices, not stacks), and pregen presence without
pumping. Preserve E1/PLAYER authority/guest-turn guards and duplicate active/parked
dimension refusal; do not call their mutating/save helpers or use dimension swaps.
Scan ordering should have fixed category/dimension priority, with any reported
position chosen deterministically from borrowed keys; allocate only small result
rows, never cloned payload snapshots. Include a bounded reservation-error result.

This W1 does NOT claim the hidden scheduled/neighbor/function/mod/streaming state
empty. Adding that claim requires the defining-module dependencies listed above,
not relabeling public count()/busy() as completion. Keeping W1 separately named
is necessary to avoid a false aggregate pass, not a permanent restriction on S1.
If a proposed caller requires global admission now, the correct answer is blocked
on that larger completion design; this audit does not authorize it.

### Proposed file/test surface for that diagnostic only

Four engine paths suffice for review of W1; no implementation is authorized here:

- `crates/horizon-client/src/world.rs`: paired-gated borrowed first-WORLD-owner
  accessor/types that inspect existing public chunk/map/queue fields; inline tests
  for every chunk-extra/staged-data category. Do not use saved_entity or save encoders.
- `crates/horizon-client/src/block_entities.rs`: paired-gated immutable pending/out
  predicate in its defining module; inline tests for take/put and canceled pending
  entries. No ticker promotion, token changes or map repair.
- `crates/horizon-client/src/game/levels.rs`: dormant Game method, typed errors and
  active/every-Parked traversal; immediate Game queues can be borrowed directly
  because Pending/Spill fields are pub(super). Add only a gated child declaration.
- New `crates/horizon-client/src/game/levels/world_owner_tests.rs`: actual isolated
  native Game resource tests reusing the existing entity-owner test setup patterns.

No new persistent Game/World fields, generations, leases, journals, saved schema,
wire format, general owner registry, host hook, enabling callsite or CLI mode.
The lower-crate Ticks/LevelTicks/updater, functions and Mods runtime seams would
expand this surface materially: require a separate reviewed packet rather than
silently adding them to W1. No new bare public field exposure is recommended.

Meaningful future qualification: native fresh active+real parked worlds; success
with no named owners; independent populated WORLD refusal while actual PLAYER and
E1 still pass; off-resident/synthetic map entries; each direct-stack and
producer-only BlockEntity variant; out-of-map take state and put restoration;
all raw/staged/rolled/opaque categories including malformed Tags without decoding;
each Pending map with empty and rich payload; every Spill vector; all LevelEvent
variants in active and parked levels; global brushing/pregen/hopper-pass state;
authority/guest-turn/duplicate dimension/reservation handling. Check native pointer,
value/NaN-bit, RNG, directory entry/file byte, participant and map/ticker metadata
preservation before/after success and refusal. Removing only the injected cause
must restore the prior diagnostic result without pumping or normalization.
Retain E1 and PLAYER regressions, paired/nonpaired feature checks, ordinary both
Game modes and applicable engine lints/broad checks under coordinator execution.

## What remains unproved

No observed present-owner report certifies earlier failed Left, unload, portal,
callback or queue-tail losses. No absent-owner report reconstructs them. This audit
establishes source locations and proposed refusals only; it does not report an
executed loss or successful new observer. It cannot certify component scalar/full
byte support, machine persistence, complete future producers, writer completion,
exclusive capture, immutable copies, manifest-last recovery, external paired peers,
ordinary in-frame capture adaptation, retained animation/performance or G1.
