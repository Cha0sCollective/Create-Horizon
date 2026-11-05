# Contraptions 002 — entity owner and long-term reuse addendum

Status: read-only source comparison, 5 November 2026. This does not change
the Contraptions-001 preflight candidate `494b40de492395695359c3984562ae13c5b50288`,
implement custody or motion, register an entity kind, or expand the
Contraptions-002 approval request. The pinned host is Horizon
`9c7f9b2215dfed497aba5522c44f38926628648d`; the pinned Create Java
reference is `7ff2760e4924d35d4bc2e22c4b19923f0178cd16`.

## What the existing entity paths actually provide

| Path | Verified capability | Limit for a sole assembly owner |
| --- | --- | --- |
| Content-declared entity type | `crates/horizon-mods/src/content.rs:445-462,906-930` accepts one template, `minecraft:snowball`, tied to a mod item. | A native content declaration cannot create a controlled contraption entity with custom ticking, save and removal behavior. |
| New simulated Horizon entity kind | `crates/horizon-client/src/game/entity.rs:1-8,331-348,606-638` uses a fixed `entity_lists!` entry plus `impl Entity`; that list drives typed walk, save and load. | A true new kind needs host-source integration or substantial pinned private Mixins work. It is not a content JSON or generic `native_summon` feature. |
| Existing marker/display | `game/native_entities.rs:139-180,192-259` can summon a known kind in a loaded chunk, move a retained marker/display, pose a display and remove it. `game/persist.rs:889-935,1371-1381` retains raw NBT, current pose and UUID and saves it in the entity chunk. Unknown unsimulated kinds with valid position fall back to `OtherEntity` on load (`persist.rs:390-403`). | This is an unsimulated `OtherEntity`, not a new Create entity class. An unregistered arbitrary ID also lacks a network entity-type mapping (`lan/hub.rs:500-510,2755-2770`), so server retention does not imply peer pairing. |
| Entity attachment | `game/mod_api_host.rs:60-87,122-133` admits an `OtherEntity` as `Holder::Entity(id)` and keeps its typed attachment by UUID; `game/persist.rs:1240-1246,1388-1398` reads and writes it into entity NBT. `game/attachment_sync.rs:18-25,39-57,85-125` sends synced values to entity trackers and a full snapshot when tracking begins. | A marker or display can carry a saved `AssemblyRecord`, but the entity file is separate from the block chunk. Sync reaches only actual trackers: vanilla marker has tracking range 0, so it offers no ordinary peer attachment stream (`lan/tracking.rs:95`; `lan/entity.rs:817-825`). |
| Remote presentation | `game/lan_entities.rs:406-421,431-457` sends tracked `OtherEntity` state and display data; `game/remote.rs:2537-2592` tracks an incoming kind and pose. Vanilla display types have nonzero tracking range (`lan/tracking.rs:23,82,145`). | Raw owner NBT is not automatically a rendered contraption. A display can provide a visible projection, but supplies no moving-solid contact or deletion protection. |

The Java bearing instead removes source blocks and spawns a real controlled
contraption entity (`MechanicalBearingBlockEntity.java:167-205`). That entity
saves its captured structure, controller link, axis and angle
(`AbstractContraptionEntity.java:611-620`;
`ControlledContraptionEntity.java:80-97`) and participates in its own
controller/reload/disassembly lifecycle. A Horizon marker with an attachment
can resemble the **storage location** of this Java model; it does not inherit
that entity's lifecycle or physics.

## Can an existing entity be the canonical owner without a public host change?

**For data storage, yes in principle. For the complete no-loss live contract,
not with the exposed entity services alone.** An existing `minecraft:marker`
or `minecraft:block_display` can be spawned with saved NBT and receive a
typed saved `Holder::Entity` attachment. The marker's range-zero tracking
means ordinary peers do not receive its attachment; a tracked display or
separate reviewed projection transport is needed for presentation. A
private Mixin could drive the owner from `native_level_tick_post` or another
pinned Game tick, find loaded `Game::others`, reconcile UUIDs after entity
load, and supply the client projection. This does not inherently require a
new public Horizon API. It is, however, a separate private integration
packet with an explicit owner/save/contact review, not a ready-made native
mod entity declaration.

The missing lifecycle pieces are concrete:

- `entity_lists!` omits `OtherEntity` (`game/entity.rs:331-348`). The
  `native_entity_tick_pre/post` path covers selected simulated kinds and
  does not independently tick markers/displays (`docs/native-mods.md:1187`).
- `native_entity_join_level` explicitly excludes unsimulated displays,
  markers and interactions (`game/native_hooks.rs:7126-7143`), so that hook
  cannot rediscover an owner on cold load. `native_entities_load` reports a
  chunk and count, not an assembly ID (`native_hooks.rs:5839-5841`). A
  private loaded-entity scan or an independently saved index is required.
- A marker can be removed through `native_remove_entity`
  (`game/native_entities.rs:254-261`) or the `Target::Other` command removal
  path (`game/commands.rs:2207`). There is no pre-removal owner callback on
  `OtherEntity` in these paths. If its attachment is the only saved structure,
  deletion loses custody unless a reviewed private guard/recovery path is
  added. A second authoritative chunk copy would cease to be a sole entity
  owner.
- `native_entity_merge_nbt` does not mutate `OtherEntity`
  (`game/native_entity_data.rs:28-45`). Updating a marker's owner data uses
  the typed attachment service or pinned private internals; raw summon NBT
  alone is insufficient for changing pose and retry state.
- Entity NBT and chunk blocks have distinct save/load paths
  (`game/persist.rs:1371-1398`; `crates/horizon-client/src/save.rs:1499-1508`).
  An entity-backed owner must define ordering, normal unload/reopen and
  crash-recovery tests across those files. It must also decide what happens
  when the entity's chunk unloads while assembly cells remain visible.

None of these limits proves that a private marker owner is impossible. They
do show why equating it with Java's controlled entity now would skip owner
removal, rediscovery and save obligations. A true simulated Create entity
would additionally need the fixed dispatch integration and client/runtime
behavior; moving-solid contact is an independent contract for either owner.

## What the bounded chunk owner contributes to full Create

The proposed two-block chunk record is useful if its *payload and behavior*
remain separate from the chunk key. Reusable pieces are: the staged generic
`Assembly::capture`/`landing`, rotation and bearing state; local coordinates,
exact source snapshots, stable assembly identity and versioned codec; the
capture/restore progress journal with checked writes and no-loss retries;
source/target validation; and the rule that one server pose drives contact and
client presentation. These are needed whether the eventual owner is a chunk,
level record or a real contraption entity.

The **backing and scope do not automatically scale**. One record per chunk,
same-chunk swept-area refusal, chunk-tracker sync, two-cube atlas rendering
and a controller at a fixed neighboring cell are first-milestone limits. A
cross-chunk assembly could leave its owner's chunk loaded state or span
chunks whose saves complete at different times. Before that milestone, the
owner must select and test a level or entity lifetime, record migration by
stable ID and codec version, loaded-chunk policy, cross-file recovery,
broader rendering and a general contact policy. The accepted S5 piston keeps
its own BE owner; no migration is implied for it.

**Recommendation:** retain the saved chunk attachment as the canonical
owner for the explicitly same-chunk slime/log bearing and keep marker/display
entities, if used, derived and replaceable. Approve the portable record,
checked commit and pose/contact semantics as the reusable part; treat chunk
storage as a bounded backing, not the final answer for full Create. Record a
separate owner decision before cross-chunk assemblies. If the owner instead
prefers entity authority now, require a concrete private lifecycle packet
covering UUID rediscovery, all removal paths, entity/chunk save ordering,
remote projection, independent tick and no-loss restoration. No public host
API is proven necessary for a finite private marker prototype, while a new
simulated entity kind is host integration rather than a current content API.
