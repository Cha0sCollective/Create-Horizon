# H08 W1 WORLD-owner observation packet

DESIGN ACCEPTED by a DIFFERENT fresh independent read-only Sol High review of
draft SHA25652fa6895d12ec40f918912347ce3c7bd7cdec3398baf26d76de7cac40ec45e82.
Implementation is RELEASED on codex/h08-world-owner-observation at the qualified
base below. Source and actual qualification remain unperformed. This packet uses
Horizon sources only. Read
H08_WORLD_ITEM_OWNER_AUDIT.md for source/lifecycle findings and retained limits.

## Base and bounded outcome

Implement only against clean qualified local engine
`b41427c6b20d0a7458a3de730dc532d3a68d4081`, tree
`6142ddc068e22801d6150277e65eadedf6605f8d`, on the coordinator's assigned branch.
The coordinator reports six native cases/current counter receipts, native feature
checks, both Game modes, broad369/1358/0 and final independent review accepted.
This document's author did not execute or independently qualify those results.
The engine remains untouched during authorship. Private branch
`codex/h02-smoke-h03-notes`, inspected HEAD
`fda69a4ab244c73ab72faedd5b900c025b3bdb89`; preserve the separate proposed WORLD
audit and coordinator idle-close result/status work.

Add one dormant synchronous borrowed observation of named WORLD owners in the
active and EVERY loaded Parked world, plus immediate named Game effect queues.
Initially refuse ALL native BlockEntity values, empty or populated, without
calling slots()/contents(). Refuse staged/raw/opaque data and the finite queue
categories below. Empty named populations are a valid observation. Existing
PLAYER and E1 helpers, successful native lifetimes and all gameplay remain intact.

Success states only that the named W1 causes were absent at that instant. W1 is
neither an inventory census nor a completion preflight, readiness token, barrier,
permission to save or permanent restriction of S1. PLAYER+E1+W1 success MUST NOT
be reported as global checkpoint readiness. Hidden Ticks/updater/functions/mod/
lent-world/streaming ownership, callbacks already holding local payloads, IO and
historical loss remain explicitly outside this packet.

## Verified visibility and exact five-path surface

Four core paths (three production files plus a new test child) and ONE additional
test-only defining file comprise the exact surface.
Do not edit existing qualified methods/helpers or add enabling callsites.

1. `crates/horizon-client/src/block_entities.rs`: new paired-packages-gated
   `pub(crate)` immutable accessor and a crate-visible finite enum. Its defining
   module alone reads private out, pending and added. Add focused inline tests.
   No field widening, promotion, sweep, token changes, normalization or mutation.
2. `crates/horizon-client/src/world.rs`: new paired-gated public immutable World
   accessor and small scalar diagnostic types; inline lib tests. World is in the
   library, while Game is in the binary: this public gated output seam is necessary,
   not a pub(crate) field shortcut. The BlockEntities accessor stays crate-private.
3. `crates/horizon-client/src/game/levels.rs`: new paired-gated, at most
   `pub(in crate::game)` dormant Game helper and detached report/error types, plus
   the gated test-child declaration. Private Parked/Levels fields stay private.
4. NEW `crates/horizon-client/src/game/levels/world_owner_tests.rs`: gated actual
   native Game resource tests, using unchanged Fixture/ready/players/rich helpers.
5. `crates/horizon-client/src/game/brush.rs`: ONLY the two tiny paired+test seed/
   snapshot helpers specified below. NO production getter, field or behavior change.

Visibility was checked against actual source, not inferred from the audit:

- World.watched_writes, chunks, moving_pistons and moving_piston_fallback are pub;
  Chunk and ChunkExtras named fields are pub. No new getter is needed for them.
- BlockEntities Deref/IntoIterator exposes borrowed map entries, but out, pending,
  added, next_slot and other ticker state are private to block_entities.rs. Add
  the defining accessor; never read those fields from world.rs or the binary.
- Pending/Spill and every named map/vector field are pub(super), visible in game
  and descendants, including levels.rs and its child. No spill.rs edit is needed.
- Brushing = HashMap<[i32;3],BrushState> and BrushState are pub(super). Therefore
  levels.rs can borrow is_empty()/keys() without inspecting private counters.
  The counters count/resets_at/cooldown_ends_at/hit_face are private and BrushState
  has only Default, with no Debug/Clone/Eq. A four-path Game test cannot meaningfully
  seed/snapshot them exactly. This justifies the fifth TEST-ONLY path.
- CommandExtras.pregen is game-visible pub; is_some() needs neither PregenTask's
  private constructor nor its internal pregen/paused/dimension state. No command
  feature flag, environment variable or pregen.rs edit is proposed.
- Existing levels::entity_owner_tests is pub(super); ready, players, rich and
  snapshot are pub(in crate::game). Fixture::new/game are pub(in crate::game).
  Reuse them unmodified. The old snapshot omits W1 data, so supplement it in the
  NEW child rather than claiming its existing view covers these additions.

All new production items are cfg(feature = "paired-packages"). Inline/new tests
are cfg(all(test, feature = "paired-packages")). Library cfg(test) items are NOT
available to the binary's resource tests: private ticker-state exact assertions
must live in block_entities.rs inline lib tests, not via an imaginary test-only
library interface. Brush test helpers live in the SAME game binary and are legal.

Inspected base SHA256:
- world.rs:78b2b714df6f3deaba6a500e7bc504135d067e259373451ea89fe46a66fd1fa7
- block_entities.rs:52818bdf99ba4ebdad6c56536e5861ac23e5d4c458a27cd12e6554d1da024855
- levels.rs:8f42ebe09718c04090fab4d71204dcc3aecb13dbbf85000688e0bc98797b848e
- brush.rs:7001fadaa5dacab2e2a3dd666ade6ff2ef9d961f3e3e1e56040546ef86e999ef
- unchanged entity_owner_tests.rs:205d51ba77941077887fb41d30d94e7c8df42a2f59b7c718ca669e009bcb1647

No Game/World persistent fields, journal, token, history ledger, dependency, lock,
codec, serialization/wire format, SDK/control command, save/callback body, runtime
hook, streaming/scheduler change or production helper outside these four paths.

## Proposed native interfaces and finite output

In block_entities.rs:

    pub(crate) enum CheckpointBlockEntityPending { Out, Pending, Added }
    pub(crate) fn checkpoint_pending(&self)
        -> Option<(CheckpointBlockEntityPending, Pos)>

The method belongs to BlockEntities. For each category use the minimum borrowed
position, with Out before Pending before Added. Added includes canceled/stale
entries still in its Vec. A live ticker, states table, freed flag or next_slot
alone is not a named pending cause; preserve them rather than normalizing them.
Return only copied enums/coordinates, never an entity reference or clone.

In world.rs, public only under paired-packages:

    enum WorldOwnerKind { /* exact ordered categories 0..15 below */ }
    enum WorldOwnerLocation { Block([i32;3]), Column([i32;2]) }
    struct WorldOwnerPresence { pub kind: WorldOwnerKind,
                                pub location: WorldOwnerLocation }
    World::checkpoint_world_owner(&self) -> Option<WorldOwnerPresence>

The enum/struct declarations are `pub`, and scalar types derive Copy/Debug/Eq;
kind/location may derive Ord to implement the specified order. These are native
experimental diagnostics, not a stable public protocol. No borrowed payloads or
opaque NBT are returned. Construct only the category's prescribed location form.

In levels.rs, paired-gated and at most pub(in crate::game):

    Game::observe_checkpoint_world_owners(&self)
        -> Result<WorldOwnerObservation, WorldOwnerObservationError>
    WorldOwnerObservation { levels: [Option<WorldOwnerLevel>; 3] }
    WorldOwnerLevel { dimension: Dim, resident_columns: usize }

The three slots always mean Overworld, Nether, End, in that order; absent unloaded
dimensions are None. Every active or parked loaded level contributes exactly one
row. Dim is a closed three-variant enum; refuse a duplicate active/parked dimension.
Use fixed stack storage for candidates and result rows. NO heap allocation is
needed, including no Vec of keys/references/results. This improves the audit's
provisional reservation-error suggestion: there is no fallible reservation path
to manufacture or test, and no allocation-based proof is claimed. Report contents
are owned scalar copies, not leases; changing/dropping a report cannot affect Game.

Finite Game errors:

- NotAuthoritative: same borrowed remote/integrated/client-role alternatives as E1.
- ActingGuest: lan_acting is Some.
- DuplicateActiveDimension: parked contains the active Dim.
- World { dimension, cause: WorldOwnerPresence }.
- LevelEvents { dimension }.
- Pending { kind: PendingKind, position: [i32;3] }, where kind is exactly
  Shulker, Banner, Names, Pot or Copied.
- Spill { kind: SpillKind }, exactly Spawns, Removed or Orbs.
- Brushing { position: [i32;3] }, HopperPass, Pregen.

No permission/success implication is inherited from PLAYER/E1. Do not call their
helpers inside W1; tests deliberately demonstrate separate observations. Preserve
their code and authority/error semantics. The W1 authority/guest guards are copied
simple checks, not roster normalization or owner-only admission.

## Exact observation and deterministic priority

Evaluate the three guards above in that order. Then select the minimum named cause
by `(category rank, dimension rank when applicable, local coordinate)`. Dimension
rank is Overworld0, Nether1, End2 irrespective of which is active or HashMap order.
Block coordinate order is signed `(x,y,z)`; column order is signed `(x,z)`. No float
coordinate conversion, payload parse or residency assumption is needed. Global
Pending/Spill/Brushing/Hopper/Pregen causes have no invented dimension attribution.

World categories, with their exact source condition/location:

| Rank/kind | Borrowed condition | Location |
| --- | --- | --- |
| 0 BorrowedBlockEntity | BlockEntities.out nonempty | minimum Block position |
| 1 PendingBlockEntityTicker | private pending nonempty | minimum Block key |
| 2 AddedBlockEntityTicker | private added nonempty, canceled included | minimum Block position |
| 3 NativeBlockEntity | ANY map entry, no kind/content whitelist | minimum Block key, off-resident/synthetic included |
| 4 StagedBlockEntities | resident Chunk.block_entities nonempty | minimum affected map Column key |
| 5 StagedEntities | resident Chunk.entities nonempty | minimum affected map Column key |
| 6 SavedBlockEntities | saved.as_deref().block_entities nonempty | Column key |
| 7 SavedBlockEntitySources | saved.block_entity_sources nonempty | Column key |
| 8 RolledLoot | saved.rolled_loot nonempty | Column key |
| 9 OpaqueChunkRoot | saved.root nonempty | Column key |
| 10 Postprocess | Chunk.postprocess nonempty | Column key |
| 11 SavedBlockTicks | saved.block_ticks nonempty | Column key |
| 12 SavedFluidTicks | saved.fluid_ticks nonempty | Column key |
| 13 WatchedWrites | World.watched_writes nonempty | minimum tuple position |
| 14 MovingPistons | World.moving_pistons nonempty | minimum buffer Block position |
| 15 MovingPistonFallback | World.moving_piston_fallback nonempty | minimum buffer Block position |

A Some(empty ChunkExtras) does not refuse merely for being Some. block_entity_order
alone, block_names/chest_openers, dirty/light/mesh bookkeeping and nonpending live
tickers are not physical inventories and are outside the W1 refusal set. Tests
must ensure these benign examples do not silently become broad blanket refusals.
Piston buffers are refused as conservative shape-state observations, not counted
as stacks; the real Ticks owner remains excluded. Saved source/rolled/opaque data
is refused without asserting that it is a second physical inventory.

Game categories follow, after all World ranks:16 LevelEvents (any variant; active
and every Parked);17 PendingShulker;18 PendingBanner;19 PendingNames;20 PendingPot;
21 PendingCopied;22 SpillSpawns;23 SpillRemoved;24 SpillOrbs;25 Brushing;26 HopperPass;
27 Pregen. For Pending and Brushing select the minimum borrowed signed key. For
Spill and LevelEvents return presence only; never inspect/cast possibly nonfinite
float payloads or choose a coordinate from them. Pending entries with empty Vec,
empty Compound, empty String or all-None slots still count as present. item_columns
is_some() refuses even when its internal cache is empty. pregen.is_some() refuses
even if the task is paused; its production guard is source-reviewed only.

Compare all applicable level candidates by category THEN dimension THEN location;
do not stop at the active level's lower-priority cause before inspecting parked
levels. A World accessor can return its per-world minimum; Game considers that
candidate and level_events for each loaded dimension using the same rank order.
Success requires every named condition to be absent, with all loaded rows filled.
No new data structure is needed for iteration over the fixed Dim list.

The observer is &self, pure borrowed checks plus scalar result construction. Never
swap/enter/tick/load/save/unload a level; promote/sweep block entities; clone an
Arc/Chunk/ItemStack/Tag; take/clear maps/Vecs; call spill attach/flush; canonicalize
components; query disk; pump pregen/streaming/guests/mods; replay LevelEvents; or
change RNG, counters, directory contents or task lifetimes. Do not use slots(),
contents(), saved_entity(), save encoders or get-or-insert accessors.

## Fifth file: exact test-only brush access

Existing Default makes only an all-zero state. The meaningful native brush path
brush_block is private and rolls loot, advances timers, schedules ticks and may
change dusted state; invoking it would introduce unrelated effects. Propose only:

    #[cfg(all(test, feature = "paired-packages"))]
    pub(in crate::game) fn world_owner_test_brush_state() -> BrushState
    #[cfg(all(test, feature = "paired-packages"))]
    pub(in crate::game) fn world_owner_test_brushing_snapshot(value: &Brushing)
      -> Vec<([i32;3], usize, i32, i64, i64, Option<[i32;3]>)>

Constructor values: count7, resets_at123456, cooldown_ends_at123499,
hit_faceSome([-1,0,0]). The snapshot copies each signed key, the native BrushState
value address as usize, and those four exact private values, then sorts by key.
These are test-only fixture views, not candidate-side payload clones or new
production hooks. Tests insert the returned state through the normal HashMap;
no public fields, production constructor/getter or existing qualified helper body
changes. The snapshot must cover the whole map, including multiple entries.

## Native/resource acceptance before any result claim

Source review MUST precede candidate execution. Coordinator owns ALL formatting,
Cargo/build/test/runtime/commit work, serialized against every engine/native/Wasm
job. This packet author performs none. Freeze exact five-source diff/hashes and
existing-helper hashes for fresh independent source review and subsequent receipts.

Add focused inline library tests for the immutable BlockEntities accessor's Out,
Pending, Added, ordering and empty behavior. Use native insert/take/put/remove,
including canceled added entries. Assert exact private map/ticker/live/added/pending/
next_slot/freed/out/states values and capacities/pointers before/after repeated
queries. Normal fixture-only native ticker promotion may establish a settled map
BE before the assertion baseline; NEVER promote during observation or to repair a
refusal. Add World tests for every kind, signed-key ties, cross-category priority,
empty extras and benign metadata. All variants remain automatically refused by
map presence rather than an incomplete enum whitelist.

Proposed actual resource tests in the new child, each ignored unless run explicitly
with pinned resources and fresh owned process cwd/HOME/temp:

1. world_owners_report_empty_active_and_all_parked_levels: unchanged Fixture/ready;
   open real Nether/End during SETUP, return to a chosen active dimension, then
   freeze. Assert exact fixed row slots/counts, repeated detached reports, PLAYER
   observation/preflight and E1 success. Repeat with another active dimension.
2. world_owners_refuse_all_native_block_entities_and_lent_states: every existing
   BlockEntity variant via legal native constructors/for_block; rich stored stacks
   and producer-only families. Exercise active AND real parked records; off-resident
   and synthetic positions. Settle setup ticker metadata only before baseline when
   isolating NativeBlockEntity. Native take keeps the entity in a local variable
   across two W1 calls; require BorrowedBlockEntity, preserve its payload/address,
   put it back natively and require the expected restored cause. Distinguish Pending
   and canceled Added separately. W1 must not call ticker promotion.
3. world_owners_refuse_every_staged_saved_and_opaque_chunk_category: each rank4..12
   independently active and parked, rich raw/sources/rolled data, malformed and
   nested Tags, raw Float/Double NaN/negative-zero bits and unpaired UTF16 strings.
   No decoding is needed; Some(empty extras)/order-only metadata remain passing.
4. world_owners_refuse_watched_shape_and_every_level_event: ranks13..16 independently
   active and parked; each LevelEvent enum variant and every PlayerEffect variant,
   including sound/no-op/mutation-only events. Use noncanonical NaN float payloads
   in shape/sound/TNT positions to prove presence checks preserve exact bits.
5. world_owners_refuse_all_pending_and_spill_categories: each of five Pending maps
   and three Spill vectors independently, empty payload entries as well as rich
   stacks/components/removed BlockEntity and orbs; retain nonfinite float bits in
   Spawn/orb payloads without conversion. These are Game-global causes, no fabricated
   dimension. PLAYER and E1 still succeed with the injected W1-only cause.
6. world_owners_refuse_brush_and_hopper_bookkeeping_without_advancing: nondefault
   test-only brush states at multiple signed keys, exact counter/address snapshot;
   item_columnsSome(Default::default()) refuses without touching private cache.
   pregen remains source-only; no private task construction, pump or global flag.
7. world_owners_have_deterministic_priority_and_authority_guards: mixed World/global
   causes across all dimensions and reversed insertion order demonstrate category,
   dimension, position ordering; independently exercise NotAuthoritative, actual
   as_guest turn and duplicate active/parked key using existing native setup.

Each eligible owner case invokes unchanged players() and E1 before/after, proving
these existing diagnostics can pass while W1 refuses. Guard cases assert their
specific authority limitations rather than forcing incompatible PLAYER success.
Before two repeated calls, snapshot the complete finite seeded W1 values and raw
Vec/map keys, capacities and value/buffer/Arc/ItemData addresses. Supplement unchanged
entity_owner_tests::snapshot with NEW local views, including recursive rich metadata,
raw BundleSelected, saved Tag integer float bits, native float to_bits(), all queue
orders, signed keys and body/parked identities. Do not rely on Debug formatting of
native floats to preserve NaN payloads. Existing snapshot covers player/play/raw
selection, level/dimension, RNG clone-sequence probes and directory/file bytes;
new views cover W1. Inline defining-module tests cover private lib ticker fields.
Brush helper covers otherwise-invisible private counters. Test-only snapshots may
allocate scalar/view records; the production observer never allocates or clones
payloads. Remove ONLY the injected cause after the frozen assertions and verify
the previous result returns; never clear unrelated owners or pump to obtain it.

Retain PLAYER/E1, transition/raid/parked-save/idle-close and world-slot regressions;
compile/test paired and nonpaired relevant configurations; all affected feature
Clippy/format/public-tree checks; ordinary both-mode Game smoke and applicable
broad checks. Resource tests must be exact-name enumerated and run serially in
separate fresh isolated processes, with source/binary and the pinned prepared-
resource MANIFEST checked before and after. This does not claim an independently
rehashed entire resource tree. A successful compilation or empty filter is not
executed test evidence.
Root selects commands matching the final frozen suite, not this prose alone.

Independent read-only design-delta review accepts reusing the just-qualified
b41427c ordinary integrated/in-frame evidence ONLY if frozen-diff review confirms
strictly dormant additions, unchanged existing production bodies and no enabling
callsite. Reused Game SHA256 is
8e33586ad5bfe57780229de6e71a8f8f2238ade47e1bb03152883fccabdc5270;
original runs are e504307e-a29c-409f-a5f4-160ebe59d768 and
869d9d9d-2a65-4fd9-861d-004dfb2db34a. Fresh focused/resource tests, relevant feature
checks and broad checks still qualify W1. Any changed normal path or new concern
requires fresh GUI validation. Never claim a new ordinary Game binary was executed
when this baseline is reused. The manifest distinction and five-path wording also
pass that bounded delta review; implementation scope and G1 remain unchanged.

## Stop and limits

Return for review on any additional defining-module getter, private-field access
problem, hidden producer claim, test fixture/body rewrite, extra production path,
new persistent field or runtime enabling caller. Coordinator assignment controls
the exact source release; design acceptance is not a runtime result. W1 cannot
detect or reconstruct earlier failed Left,
unload/portal/queue-tail loss, certify arbitrary byte/components, machine records,
all producers/writers, exclusive capture, immutable copies/manifest-last recovery,
external paired peers, ordinary in-frame checkpoint adaptation or G1. No later
stage, public push, release or upstream submission is authorized.
