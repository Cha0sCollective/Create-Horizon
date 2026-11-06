# Machinery 004 — stationary Crushing Wheels owner decision

**PROPOSED / HOLD for root reconciliation and owner decision. Design only.**
No source, content, schema, host, build or game change is allocated by this
document. The selected first wheel outcome would transfer one ordinary dropped
`minecraft:cobblestone` through a valid stationary wheel-pair controller and
return exactly one `minecraft:gravel` world item. The recipe is Create's
**milling fallback**, so this would qualify a Crushing Wheels item path, not
a `create:crushing` recipe or general crusher parity.

## Frozen discovery inputs and reuse

Machinery was clean at `b23742a31bfec218b9922d6d892cda0384073d86`
when this design began. Accepted native source is
`55339ce3347b2ce667281e8a3e6318ec856144e4`. The current physical
staging checkout was `9651dba12d163896d45a8cc4ba1b0b3b0da1feff`
and primary navigation checkout `31424655aa6dad00a0b6405503b205d8b4f575fb`;
neither tree has a native Crushing Wheel path. The primary had an unrelated
dirty `operations/team-registry.json`; this lane did not touch it. Pinned Java
Create 6.0.11 is clean at
`7ff2760e4924d35d4bc2e22c4b19923f0178cd16` and pinned host is
`9c7f9b2215dfed497aba5522c44f38926628648d`.

At the audit, [current Delete228 main](https://github.com/Delete228/create-horizon/tree/dd0f3141a33851ac20fa98b807a6532471dba727)
was `dd0f3141a33851ac20fa98b807a6532471dba727`. Its
[complete 992-path tree](https://api.github.com/repos/Delete228/create-horizon/git/trees/940663dcc12459f6a9a847c50c9811c0e41952eb?recursive=1)
was not truncated and had no path containing `crush`; recipe-map blob
`4d3b9687c338b60f9364a00408f96c12043203bf` had no crushing recipe,
and `tools/catalog.py` omitted the block. Accepted and staged native trees
also lack a Crushing Wheel block/controller, item/model/loot, kinetic kind and
`create:crushing` recipe. Their only `crushing_1/2` references are Millstone
ambient sound IDs. There is no current upstream wheel implementation to import.

| Existing source | Reuse for the proposed item path | Boundary |
| --- | --- | --- |
| `native/create/content/data/create/recipe/milling/cobblestone.json`, blob `649da01a4d385c47fd0b77a47c845aba6f006046` | One cobblestone → one gravel, nominal 250 processing time | `create:milling`, not `create:crushing` |
| `native/create/mixins/create/millstone.rs`, blob `cb0663690677cf3e6f5b98bc01f159e1fa928a98` | One-input recipe lookup, saved canonical slots and processing clock patterns | Single-block owner; not a wheel controller |
| `native/create/mixins/create/saw.rs`, accepted blob `c6c5eb71` | Confirmed item-entity reduction, inventory mirror/restore and world-item ejection patterns | Saw top contact cannot be copied as midpoint contact |
| `native/create/mixins/create/world_items.rs` and pinned host item query/set/spawn calls | Existing item transfer primitives | No middle-controller custody or anti-reentry rule |
| `native/create/mixins/create/engine/src/core/kinetics.rs`, blob `079f92ecdbe14551a3490120df97a0d92908a05c` | Loaded-only speed/stress solver | No Crushing Wheel kind, axial ports or 8 SU/RPM load |

## Selected Java contract and finite outcome

`CrushingWheelBlock.updateControllers` at pinned
`CrushingWheelBlock.java:60–126` creates a middle controller only between
same-axis wheels with a replaceable gap. Opposite, nonzero **effective** wheel
speeds make it valid; stopped or same-direction wheels leave it invalid and
nonprocessing. Removal of a wheel removes its adjacent controller. A concrete
downward-output fixture is two X-axis wheels at `(0,0,-1)` and `(0,0,1)` with
the controller at `(0,0,0)`, the first at positive effective RPM and the
second at negative RPM. The Java block's stress impact is 8 SU/RPM per wheel
(`AllBlocks.java:685–697`); the two axial shaft ports are in
`CrushingWheelBlock.java:169–172`. Actual RPM, controller facing/validity and
capacity/stress must be observed, not inferred from motor settings.

`CrushingWheelControllerBlockEntity.findRecipe` at `:326–331` selects
`create:crushing` first and `create:milling` second. The native packaged
cobblestone recipe has one deterministic gravel result. Supply only one
cobblestone item, drop it by ordinary Survival interaction into the gap,
observe one confirmed intake, then the saved processing clock, one gravel
world output and ordinary actor pickup. Pause after captured inventory,
close normally and cold-reopen the same world in integrated and dedicated/
two-client modes before completing the cycle. Record server block, item and
controller state separately from actual actor/peer wheel, gap and output views.
Run one independent removal case while the controller holds one item to verify
exactly one spill and no later output; preserve failed receipts.

The first **distinct crushing recipe** is the pinned
`crushing/glowstone.json`: one glowstone, 150 nominal time, three guaranteed
glowstone dust and one bonus dust at 50% chance. It requires adding a recipe
resource and random-output accounting, so it is outside this first outcome.
Unsupported inputs are not a refusal fixture: pinned Java applies its
100-unit no-recipe path and clears them.

## Custody, save and implementation boundary

The world item owns the selected input until a confirmed host item-entity
mutation transfers that one item to the middle controller. Thereafter the
controller's canonical `ModBlockEntity.items` owns input slot 0 and then the
output slot; NBT mirrors its inventory and clock for client/save visibility.
On normal ejection, remove the output from the controller once and create one
world item; after actor pickup, inventory owns it. Never represent the same
item as both a live world entity and controller-owned stack. The Java
controller has a 32-slot `ProcessingInventory`, saved inventory, speed and
optional in-flight entity UUID (`CrushingWheelControllerBlockEntity.java:54–57,
336–352`). The selected cold claim begins **after** confirmed inventory
capture; Java's mid-contact UUID reacquisition and crash atomicity are deferred.

The pinned host [saves Mod NBT and canonical items](https://github.com/Cha0sCollective/Horizon/blob/9c7f9b2215dfed497aba5522c44f38926628648d/crates/horizon-client/src/save.rs#L617-L633).
Its [pre-removal spill](https://github.com/Cha0sCollective/Horizon/blob/9c7f9b2215dfed497aba5522c44f38926628648d/crates/horizon-client/src/game/spill.rs#L480-L505)
requires a registered Mod container. A new controller must register that
container, keep canonical contents in `items`, define saved timer/applied
state, and verify that every selected wheel/controller removal uses the host
spill path exactly once. An NBT-only mirror is insufficient. Existing
`native_item_entities`, `native_set_item_entity` and `spawn_item` can support
the selected transfer; a midpoint contact/anti-reentry rule and the pairing
lifecycle are new machine-owned behavior. No new host API is established by
this audit. Stop and return a concrete gap if those existing callbacks cannot
observe or safely commit the selected item transfer/removal.

If allocated, candidate-owned surfaces would be a bounded kinetic wheel kind
and impact/ports, a small controller cycle, a machine adapter, and the wheel/
controller block, item, model and loot declarations. Shared runtime/placement
wiring, registration and catalog integration go to the project lead by explicit
handoff. Do not add a general processing framework, new host API, broad
kinetic propagation eligibility or unreviewed recipes. Entity damage, belt/
funnel input, sided extraction, moving wheels, other orientations, large stacks,
other target recipes and public release are outside this first packet. The
controller's new block owner, persisted 32-slot inventory/clock and removal
contract require an explicit owner/schema and root architecture decision **before
any production edit**. A DESIGN review does not supply that decision.

If the owner defers the controller, a smaller separate packet can qualify the
already packaged `create:milling/gravel.json` (blob
`bd16539e43afb98a9056a40e7fe3cc1b7c506f06`): one gravel → one flint at
nominal 250 time in the existing Millstone, including ordinary input, saved
output, cold reopen and collection. That adds no owner or schema and makes no
Crushing Wheels claim.

Read-only discovery specialists `/root/crushing_reuse_audit` worked
02:46:14–02:49:31 UTC (197 s), and `/root/crushing_java_host_audit` worked
02:46:19–02:51:30 UTC (about 311 s). Neither edited files or ran Cargo,
Java, game or control processes. Verified token counters remain unknown.
