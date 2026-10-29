# H08 all-loaded-level entity-owner observation

Accepted bounded design after a DIFFERENT fresh independent read-only Sol High
design-delta review with no remaining blocking findings. Reviewed corrected draft
SHA256:6e8539f3869901560b478cbb87d8841f6a12da3ea2a505b14502ca4ab3a4b142.
Implement against qualified clean local engine63a7d62fae27dcd9ce8bfbe1209616f4e4988607,
tree 571ad7e2cc20d8178898c6fdd9a0244252f8dcad, on
codex/h08-retained-raid-lifetime. Its resource/native/both-mode Game/broad and final
independent receipt qualification passes. Geometry sandbox qualification and final
independent evidence review also finish with this engine unchanged, releasing
that temporary source freeze. Coordinator assignment still owns exact scope.
Read H08_GLOBAL_OWNER_AUDIT.md and the actual native source, not the audit alone.

## One bounded objective

Add a dormant synchronous borrowed observation of the ACTIVE and EVERY Parked
level's named native ENTITY owners. The only supported live native entity family
in this packet is resident dropped ItemEntity values. All other native entity
families, pending loaded-entity data, retained EntityMemory maps, whole World slot
trials and portal travel/arrivals refuse with finite causes. Empty supported
populations are legitimate observations. Two or more ordinary external players
remain compatible with this diagnostic; there is no owner-only admission shortcut.

Name the helper and report explicitly for ENTITY owners. Do not call it a global
checkpoint preflight or return a readiness/lease/publication token. Success means
only that these named current entity owners match the SMALL support boundary
below at the time of the call. It says nothing about block/container inventories,
other Game fields, past ownership loss, producer completion or byte serialization.
This is useful groundwork for the later player/world-drop transfer experiment,
not a permanent restriction of S1/G1 or a completed checkpoint operation.

The conservative boundary is deliberate. entity_chunks writes resident chunk
keys only (persist.rs::save_entities), omits tridents and rockets entirely, and
only serializes creatures.projectiles when an arrow/fireball encoder returns
Some. A position-only test cannot establish all-family coverage. Expanding native
entity families, mount/passenger graphs, opaque metadata or codecs is later work
with a concrete supported-value contract, not an incidental addition here.

## Concrete source and visibility surface

Production additions only, with existing bodies unchanged:

- game/levels.rs: defining-module Game/Levels/Parked borrowed inspection, finite
  error/report types, and gated declarations for its new test child. This module
  can directly borrow both active Game fields and private Parked fields. Keep
  Parked fields private. Do not widen parked_chunks, swap_level or enter_level.
- game/persist.rs: one paired-packages-gated, pub(in crate::game) or narrower
  borrowed EntityMemory predicate reporting the first nonempty map category.
  Its defining module can inspect private unknown and anon without exposing them.
- game/nether_portal.rs: one paired-packages-gated, pub(in crate::game) or narrower
  borrowed Portals predicate observing travel/arrivals. Preserve all existing
  portal/save/arrival methods and private fields.
- lib world_slot_trial.rs: one paired-packages-gated public borrowed Boolean
  World::has_world_slot_trial(&self) presence method, exactly observing whether
  the private trial Option is Some. The Game binary is a separate crate and
  cannot access the library's pub(crate) field directly. Keep field/Trial privacy,
  no cloning, IO, mutation or payload/status exposure. This is the sole added
  library surface; it confers no ownership or publication authority.

All added production types/methods are paired-packages-gated and visible at most
within crate::game, except the library Boolean presence method above. No new
Game field, World field, identity/history ledger,
serialization schema, crate dependency, SDK/control/network command, scheduler,
save cadence, runtime enabling callsite or public publication.

Tests may add exactly these child files and their gated module declarations:
game/levels/entity_owner_tests.rs, game/persist/entity_owner_tests.rs,
game/nether_portal/entity_owner_tests.rs. Keep defining-module private-map/portal
seeding and snapshots in those children. A small NEW cfg(test) helper in the
levels test child may expose its ordinary no-listener Game/guest setup and
assertions to the other two test children. It is test-only, not a production
framework. Add focused tests within lib world_slot_trial.rs's defining test module
for absent false and present empty/active/pending/faulted true; preserve all existing
test/helper bodies. Reuse the existing integrated transition Fixture/new/game visibility;
do not edit its loader, Options, fields or any qualified test/helper body.

No changes to Game's field list, lib World fields or existing behavior, Streaming,
SaveDir, item/entity codecs,
slot count walkers, existing player roster/projection methods or existing raid,
transition, saving, retirement or transport bodies are permitted by this packet.

Relevant source hashes at the proposed base:
- levels.rs: f1b4d090c44d10bbe99248c69c9a0bbcc35ffd1541e1463c6d35acae7b493efd
- persist.rs: 5d0676944dafcc2e2895a0e0702486b311d7e22e1890a90f92e3d526975ab403
- nether_portal.rs: bb938f99f7f3bb94138414e17be77cb4734581189be28412757e8c0e9bbc1e51
- lib world_slot_trial.rs: 5d621e713fc964c3944967be5e8e2628fab70c0c8991cef01ba86a503acc9951

Initial independent design review found the library/binary field-access conflict;
the original draft SHA256 is7833ae027a21d640e07c434fbc95b727bad47fcb6c4b8c32736bcb2b9db42d03
and is preserved at workspace-root .local/h08g1-design-initial.md. The narrow
presence method above corrects that proposed surface and passes independent
design-delta review at the corrected draft hash recorded above.

## Proposed native call and result

Game::observe_checkpoint_entity_owners(&self)
  -> Result<EntityOwnerObservation, EntityOwnerObservationError>.

The finite error categories are proposed native implementation details, not a
protocol: NotAuthoritative, ActingGuest, DuplicateActiveDimension,
WorldSlotTrial { dimension }, LoadedEntities { dimension },
EntityMemory { dimension, kind }, UnsupportedEntityFamily { dimension, family },
InvalidDroppedPosition { dimension, id }, OffResidentDropped { dimension, id },
PortalTravel, PortalArrivals, and Reservation.

Use finite local enums for memory kind (Uuids, Raw, Unknown, Anonymous) and
unsupported family. Never stringify arbitrary state into a guest-facing error.
A report contains only local small rows: dimension and resident dropped-item
count, sorted deterministically by dimension ID. Keep fields private to the
module/test descendants until a separately reviewed native consumer needs them.
No payload clone, Tag, UUID projection, raw pointer, authority scope, opaque lease
or retained owner escapes in the report. Reservation failure is explicit where
new report vectors use try_reserve; do not claim all Rust allocation failure is
recoverable.

## Exact borrowed checks and operation boundaries

1. Refuse remote/integrated-client Games and client execution_role, as the native
   diagnostic is intended for an authoritative Game. Refuse lan_acting.is_some().
   This does NOT replace the accepted PLAYER roster/preflight. The diagnostic
   need not require a published host merely to inspect server entity ownership.
2. Refuse levels.parked containing the active self.dim. Then inspect active and
   every Parked record using their actual dimensions and World references. No
   swapping/opening/loading a dimension, mutating a map, default creation,
   callbacks, ticks, connection polling, normalization, ownership take, pruning,
   reconciliation or IO. Include an empty parked level in the successful report.
3. For each World call the new borrowed has_world_slot_trial presence method and
   refuse Some-trial presence. An empty active
   slot, pending retirement, fault or disabled residency is still a trial owner.
   Do not call world_slot_snapshot, read/take a slot or clone its payload to check.
4. Refuse ANY nonempty World.loaded_entities vector, even a row with an empty Tag
   list. Do not call spawn_loaded_entities or interpret/rewrite pending Tags.
5. Refuse EntityMemory if ANY of uuids, raw, unknown or anon is nonempty, even a
   retained map entry containing an empty value. Return a stable finite map kind.
   This intentionally also refuses otherwise ordinary loaded dropped entities
   which retain UUID/raw metadata. The packet does not resolve orphan identity,
   unknown passenger or anonymous-position matching semantics. Never prune,
   normalize, clone or remove any entry. Positive support is fresh native drops
   without EntityMemory, not universal dropped-entity save qualification.
6. Refuse nonempty fields for every modeled non-drop per-level native family:
   falling, paintings, item_frames, armor_stands, cushions, others, boats, carts,
   bolts, crystals, dragon (Some), dragon_fireballs, breath_clouds, tridents,
   rockets, ominous_spawners, tnts, mobs, projectiles and orbs. For the active Game,
   projectiles means creatures.projectiles; Parked owns projectiles directly.
   Also conservatively refuse nonempty per-level knot_ids, retaining leash-entity
   identity instead of inventing a separate saving contract. No family is made
   safe because its position is resident, because it is non-item-bearing or
   because its current ordinary encoder can produce a Tag.
7. For EACH dropped item in each level, require finite x/y/z. Before floor-to-i32
   conversion, require floor(x) and floor(z) within the inclusive i32 range.
   Then compute the SAME native column mapping (floor, cast, arithmetic right-shift
   by FOUR bits, into 16-block chunks) as persist.rs::chunk_of and require that
   exact key in THAT World's chunks. No float-to-int saturation is accepted as a key.
   Do not compare against the active World for a parked item, streaming demand,
   is_lit, a neighboring chunk, entity ID allocation or an invented residency flag.
   The named check does not inspect stack counts/components or serialize an item.
8. Outside the per-level records, refuse self.hook.is_some() as an unsupported
   server-local fishing entity, plus self.end_portal_travel or portals.travel.
   Use Portals' defining-module predicate to refuse nonempty arrivals. Existing
   per-entity cooldown/process/before maps are observations of existing entities,
   not additional item payloads; they are preserved and not normalized here.

EntityOwnerObservation is an immediately detached diagnostic. The continuous
&self borrow prevents owner-thread mutation DURING this call, not after return.
No generation/transport worker is stopped by it. A later full capture must repeat
this check plus actual PLAYER checks inside the SAME exclusive owner operation.
Tests must separately call the real player observation/preflight to demonstrate
compatibility; the helper may not silently create or refresh a player expectation.

## Meaningful native acceptance

Use ignored actual resource Game tests with pinned HORIZON_TEST_RESOURCES required
by expect, fresh isolated cwd/HOME/game/data/config/cache/temp, session removed
and serial execution. Reuse the existing Fixture with real resource tables and
no-listener lan_host. Use at least two actual placed, loaded channel-backed
Guest::new values with real play/motion records and retained receivers. Remove
and keep the synthetic local body in the test, as the qualified predecessor does.
The actual observe/preflight_checkpoint_participants methods must pass before
EACH eligible global success or independent ownership refusal, not just once at
test startup. Authority/acting-guest negative cases deliberately also fail PLAYER
preflight; keep those separate from the passing-PLAYER ownership cases.

Required coverage:

- Success with active and parked empty levels, then DIFFERENT fresh resident
  dropped items in both. Obtain parked levels by actual enter_level transitions
  and restore the intended active level before observation. Report every loaded
  dimension and the exact drop counts. Use normal bounded rich stacks, opaque
  components and recursive raw BundleSelected values; inspect all originals
  before/after repeated calls and after discarding/modifying the local report.
- Actual positive and negative chunk-boundary positions, especially -0.1 ->
  block -1/chunk -1. Independently author expected keys. Missing exact columns
  refuse even if a neighbor/demanded column exists. Include an off-resident item
  active-only and parked-only while the other level is fully eligible.
- NaN and both infinities in every coordinate, finite out-of-i32 floor(x/z),
  and legal near-boundary values. Refusal must precede saturating casts; no
  coordinate or live item is clamped, moved or discarded.
- Independently populate and refuse EVERY non-drop field in the finite list,
  not a hand-authored parallel Boolean mask which the test never connects to
  native fields. At least a real item-bearing trident and rocket, and a projectile
  kind not supported by the ordinary save encoders, must exercise the global
  method itself. Include parked-only unsupported owners after real level swaps.
  A source-reviewed native field adapter may consolidate the common per-level
  checks, but tests must reach actual Game/Parked owners and verify both adapters.
- Each EntityMemory map kind: defining-module tests seed actual native maps with
  rich opaque Tags/UUID/position records and demonstrate global refusal while
  PLAYER passes. Preserve complete map state and native payloads, not merely
  lengths. Keep seeding helpers in NEW test-only code; no production field
  widening, Debug/Clone implementation or existing helper edits just for tests.
- Pending loaded rows, including an empty pending row, refuse without consuming
  the vector. Whole World trials refuse while empty, active and pending, in active
  and parked Worlds. Use the real enable/change/trial APIs; no take operation is
  permitted in the observation. Retain original slot payload metadata/selections.
- Portal travel and nonempty opaque arrival records independently refuse via
  the actual global method. Defining-module tests retain original plans/Tags,
  dirty/loaded state and helper maps. Include local hook/end travel where feasible;
  source-only coverage of a difficult concrete field must be disclosed rather
  than listed as an executed toggle. No new ordinary portal behavior is implied.
- Non-authoritative and acting-guest calls refuse. A deliberately duplicated
  active-dimension parked key refuses without changing either owner.
- Before/after success and refusal compare native drop values/allocations/raw
  selections recursively, relevant World slot owners, retained pending/opaque
  data, Guest/play/Level values, active dimension and ALL parked keys/owners.
  Snapshot file paths/bytes AND directory entries (including empty directories).
  The diagnostic must not open/create directories, read/write files, mutate RNG,
  save/flush/poll, normalize menus, spawn entities or change retry notifications.

Coordinator owns formatting, compile/feature-on/off lints and appropriate ordinary,
resource and broad checks. Obtain a different fresh independent source/receipt
review before qualifying the packet. Existing ordinary gameplay and codec tests
remain unchanged. A successful build/listing is not executed resource evidence.
No new runtime/SDK activation or both-path capture proof is part of acceptance.

## Separate hard gates and stop condition

Stop this packet once the small diagnostic and its named current-owner checks
are qualified. No owner decision is needed for this dormant observation; its
refusals do not accept a permanently restricted S1 or G1 deliverable.

Explicitly NOT covered by success:

- World block/container inventories and block-entity saved/runtime consistency;
  pending spill::Pending maps, spill::Spill spawns/removed blocks/orbs, level events
  and other pending item-producing actions. Those can own payloads outside the
  named entity vectors and need their own exact predicate or retained capture.
- Other native entity family serialization, mount/passenger links, orphan/raw or
  anonymous ownership, item IDs/counts/scalars/components and complete NBT/JSON
  byte encoding. Fresh resident drops are not proof of arbitrary lossless codecs.
- In-flight generation, deferred results, streaming errors/failed snapshots,
  pregeneration, queued saves and writer completion. Streaming::busy omits saving,
  written, generation_save_error and failed_finished. SaveDir::writes_finished
  observes only already queued work. A later checked phase must cover EVERY level.
- Prior failed Left/unload/portal retirement or stale reconnect history. A clean
  present owner set cannot detect earlier loss. Retained failure/reconnect ordering
  or an explicitly tracked fresh experimental lifetime remains necessary.
- ordinary in-frame local-player adaptation: the accepted PLAYER helper refuses
  client execution-role/local-body ownership. Ordinary both-mode gameplay tests do
  not enable either helper as an in-frame capture path.
- Machine/counter persistence, all-level save/error restoration, definite owner
  task completion, synchronized physical copies, manifest-last publication,
  previous-checkpoint retention and interrupted recovery/conservation proof.

No owner-thread tick/task/pregen/control pump may interleave with a future full
capture. server-hold still serves control tasks; Integrated::call timeout does
NOT cancel the queued/running task. This observation creates no such barrier.
Server plus two external paired clients, retained animation/performance, coherent
recovery and G1/G2/G3 remain requirements. Engine commits remain LOCAL ONLY;
public release, publication and upstream contribution remain separate decisions.
