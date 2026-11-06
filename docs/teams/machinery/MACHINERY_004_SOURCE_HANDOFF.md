# Machinery 004 — Crushing Wheels source handoff

**Source candidate ready for project-lead integration; runtime outcome unverified.**
The owner approved the exact bounded controller packet described in
`MACHINERY_004_CRUSHING_WHEELS_DECISION.md` after that design document was
written. This handoff does not change that document's original DESIGN/HOLD
provenance. No public action is allocated.

## Exact candidate

- Composition base: current005 `9651dba12d163896d45a8cc4ba1b0b3b0da1feff`.
- Isolated worktree: `G:\Create Rusty\.local\workers\machinery-004-candidate`;
  branch `codex/machinery-004-candidate`; HEAD `5ed810133b2c8974472d95ec6146f6ac47bbca11`.
- Root should integrate the candidate commit range **after** `9651dba` in its
  serial shared queue. The source commits are `ac5b8c8` (core), `8768e52`
  (content), `d69b9d4` (adapter), `53b0fbb` (wheel model orientation),
  `4f51811` (current solved RPM), `26beacd` (midpoint item transit),
  `11acc93` (pinned pickup-delay width), `1ac1d92` (finite pickup delay), and
  `5ed8101` (conditional item Pre opt-in).
- Candidate-owned paths: `engine/src/core/kinetics.rs`, new
  `engine/src/core/crushing.rs`, new `mixins/create/crushing.rs`, and nine
  `content/` JSON declarations for the wheel/controller, item, model and loot.
  All paths are under `native/create/`.

The selected pair is two X-axis wheels at middle Z−1 and Z+1. The first must
have positive **effective** RPM and the second negative effective RPM. Each
wheel has ordinary axial shaft ports and 8 SU/RPM. The controller is down
facing and valid only while the pair counterrotates. Its canonical 32 slots
and clock save one cobblestone after confirmed world-item removal, apply the
existing `create:milling/cobblestone` fallback to one gravel, then eject one
world item downward. No `create:crushing` recipe is added.

The pre-tick contact adapter keeps the selected item world-owned. It cancels
only that item's normal tick while advancing it in 0.16-block steps from the
controller's Y+13/16 collision top. `catch` accepts its tracked ID only below
Y+0.25; a successful `native_set_item_entity` moves custody into controller
slot 0. Invalid pair/recipe/owner restores the world item above the top and
releases the transient contact. The contact ID is **not persisted**; cold
reopen qualification begins after canonical inventory capture. This is a
bounded native motion path, not a general Create entity-motion claim.

## Shared-file delta for project lead

The following changes are **not** in the candidate. They belong to serial
primary integration; their exact placement must preserve existing tick order.

1. In `native/create/mixins/create/engine/src/lib.rs`, export
   `pub mod crushing;`. In `native/create/mixins/create/horizon-mixin.toml`,
   declare `create_port_crushing` from `crushing.rs` in `game.rs`.
2. In `native/create/mixins/create/runtime.rs`, add both
   `create:crushing_wheel` and internal `create:crushing_wheel_controller` to
   `owns`, which makes the normal entity/tick registration path available.
   Map only the wheel to `Kind::CrushingWheel` in `node`. Dispatch
   `create_port_crushing::tick_controller(game,pos,entity)` and sync when it
   returns true **before** the `node` early return; the controller is not a
   kinetic Node. After a wheel's current solver RPM is written in kinetic
   `tick`, call `wheel_changed(game,pos,rpm)` while its entity is detached.
   Add `container(block,container)`, `models(models)`, and
   `draw(game,pos,entity,draw)` hooks. Draw both IDs before generic kinetic
   drawing and before the generic `node` guard. The model is authored along Y
   and the adapter orients it to the selected X shaft.
3. In `native/create/mixins/create/block_changed.rs`, call
   `create_port_crushing::changed(dimension,pos)`. In
   `native/create/mixins/create/motor_level.rs`, call
   `create_port_crushing::level_tick(game)` after kinetic level reconciliation
   and before block-entity ticks. It discovers/validates pairs and destroys an
   orphaned controller while its ModBlockEntity remains attached, using the
   pinned host's registered-container spill path.
4. Bind existing pinned host API 146 in `horizon-mixin.toml`: a HEAD injection
   for `Game::native_entity_tick_listened` calls
   `create_port_crushing::item_tick_listened(game,listened)`; a HEAD injection
   for `Game::native_entity_tick_pre` calls
   `create_port_crushing::item_tick_pre(game,dimension,entity,kind,canceled)`.
   Pass `game` to both. The adapter checks the actual item kind, server and
   Create enablement; it does not need a new host API. The host calls item Pre
   while `game.dropped` is attached, skips canceled normal item ticks, and
   later ticks block entities.
5. In `native/create/mixins/create/placement_runtime.rs`, exclude the
   internal controller from `level_ports`; it has no shaft port. The selected
   wheel defaults to axis X in its content and can be placed with that state.
   Do not broaden other orientations in this packet.
6. Add wheel registration to `native/create/tools/catalog.py` and its
   generated `native/create/mixins/create/assets/src/catalog.rs` (`BLOCKS`),
   plus `crushing_wheel/block` in `PARTS` for the pinned original OBJ overlay.
   Keep the controller internal with no item. **Update
   `native/create/tools/content.py` to regenerate the nine authored JSONs
   after its generic BLOCKS pass**; merely adding the wheel to `BLOCKS`
   overwrites the selected axis, shape, visibility and loot declarations.
   Include its creative-tab/language entry without regenerating unrelated
   content. The importer already accepts `neoforge:obj` for local original
   assets; the tracked fallback uses only authored geometry and vanilla IDs.

## Review and remaining gate

An independent read-only Sol High review of candidate `4f51811` found a
top-contact contract mismatch and two source issues. The latter were repaired
in `53b0fbb` and `4f51811`; the midpoint repair is `26beacd` through
`5ed8101`. Fresh follow-up review through `5ed8101` returned **CLEAR for the
source delta and its effects**. The reviewer did not edit, build or run a
game. The static `git diff --check` and parse of all nine JSON files pass.

Shared wiring, compilation, container spill on both-wheel removal, ordinary
actor drop/pickup, integrated and dedicated/two-client cold reopen, and actual
client motion remain for project-lead integration and the Transport-owned
heavy build/game lease. Preserve failures and distinguish server state from
client views. No Cargo, WSL, Java, staging or game process ran in this team.
