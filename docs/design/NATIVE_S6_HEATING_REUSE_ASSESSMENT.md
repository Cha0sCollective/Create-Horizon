# S6 — heating reuse and the next host decision

Assessment after accepted dry-mixing `3ad300e`; no heating implementation or pin
change is included. Independent read-only Sol High assessment review CLEAR;
the host route still needs the owner's decision.
Upstream source236c87a889cd76f7e663d1d182c28dc1145f7b14
and its exact target host9c7f9b2215dfed497aba5522c44f38926628648d/Mixins
e8cd0aa38cffef49cb2e088ef716baa107148c02 are separate from our tested
hostfc021038/Mixinsa1e13326. Both report API359; that number does not describe
the newer private native fragments or prove source compatibility.

## What can be reused

| Existing upstream code | Finding |
| --- | --- |
| `engine/src/core/burner.rs` | Existing fuel resolution, simulation/insertion, normal/special/creative heat and source transition ordering; no need to rewrite its algorithm |
| `burner.rs` | Existing authoritative furnace/data-map/tag fuel resolution, held-item consumption/remainders, saved `fuelLevel`/`burnTimeRemaining`/`isCreative` and block-state heat transitions |
| Basin/Mixer core and adapters | Dry packet already carries the heat predicate/cache and recipe compiler structure; heated recipe admission is a small explicit extension after integration prerequisites |
| `content/data/create/recipe/mixing/brass_ingot.json` | Existing deterministic heated recipe: copper ingot1 + zinc ingot1 -> brass ingot2; one result stack, no fluid/remainder prerequisite |
| `burner_visual.rs`, `burner_presentation.rs` | Existing source animation/parts, phase, lighting, rod/head/flame and client-observation instrumentation; presentation is already implemented upstream |
| `burner_interactions.rs`, prediction and synchronization modules | Existing empty/occupied cage interactions, capture and server-derived client fuel definitions; these are larger dependencies than a fuel timer |
| Content/import tools | Existing block states, shapes/light, models, ordinary item placement and loot; select entries and preserve our content delta rather than importing the entire catalog |

The retained Java6.0.11 reference has no relevant Basin/Mixer/burner/heat-condition
source diff from this port's6.0.10 base. This supports source reuse, not a claim
that upstream's full burner has passed our integrated/dedicated acceptance.

## Verified compatibility boundary

Pinned host `Game::new_block_entity(&mut self, name: &str)` calls the old
`native_block_entity(name, &mut entity)`. Its construction callback has no actual
block properties. New upstream uses both `native_stateful_block_entity` and
`GameLevel::native_block_entity_from_state(block, properties, out)`: an empty cage
has no new heater entity, whereas a captured burner has `create:blaze_heater`.
An existing entity's survival through same-block property changes is a separate
part of the newer creation contract. Neither new function exists in our pristine
host export. The exact newer hooks were read at the upstream target revision,
not inferred from an API integer.

The current export also lacks `NativeModelRenderOptions`, `NativeModelShade`,
`NativeModelUvShift` and `native_draw_terrain_model`, all used by the upstream
burner presentation. Old plain model drawing does not implement its selected
shading/UV animation semantics. Newer collision-context declaration fields also
remain unsupported. Client fuel prediction depends on upstream's synchronized
server-derived definitions; a copied furnace fuel table or local-pack guess is
not equivalent.

This does not prove that a reduced coal-only demonstration is impossible on the
old host. It shows why copying the complete burner is not a routine mod import.
A captured-burner-only name-based factory or simplified/server-only presentation
would introduce explicit capability/acceptance limits. Backporting the full
creation/rendering services would instead duplicate host work already done
upstream and cross the current packet's host/API boundary. Neither route is
silently adopted by the dry-mixing result.

Read-only evidence is local in `.local/native-s6-dry-mixing-01/`:
`heating-host-factory.txt`, `heating-factory-cache.txt`,
`heating-host-types.txt` and `heating-new-host-hooks.txt`. The empty type-search
output records no matching definitions across the pristine source crate trees;
the upstream manifest/presentation and exact hook excerpts record their use.
No public engine, game pin, launcher, save or production source was changed.

A shallow read-only candidate snapshot now lives at
`.local/evaluations/horizon-heating-refresh/source`, detached at exact9c7f9b22
with its push URL disabled. Static `hook-inventory.json`/`hook-summary.json`
outside that source checkout enumerate45 current injections (28 named hooks,
17 explicit targets) and24 support-module files. All targeted function names and
module paths exist in the newer tree. The one aggregate signature-set difference
comes from unrelated new functions named `view`; our actual
`game/machines.rs:Containers::view` signature is unchanged. This is source
preflight, not Mixins resolution, compilation, behavioral compatibility or a
migration pass. Upstream now provides `ModRevision::same_entity`, so the private
generated compatibility method must be removed on that candidate to avoid a
duplicate inherent method. New transient client-update fields also require a
specific save/update-path review rather than replaying our old encoder override
without checking its effects.

## Recommended next packet, pending owner decision

Implement a bounded candidate host refresh to the port's exact9c7f9b22/e8cd0aa3 pair in a
separate development branch/owned generated directory. Preserve accepted `.5`
sources, maps, binaries and disposable saves. Do not import all of0.7 or replace
our accepted kinetic, inventory, piston or portable-transfer behavior.

1. Complete the pin-pair semantic delta review against accepted families, using
   the static inventory above. Stage and compile the retained mod first; list
   concrete required adapter edits.
   If the migration needs a broad redesign, stop with its actual diff/cost.
2. Review the bounded compatibility edits before game use. Reuse unaffected
   evidence and run focused composition checks only for changed bindings. Record
   a candidate host decision; no old qualification is relabeled as a new pass.
3. Then define a coal/heated-brass packet: ordinary supplied occupied-burner
   placement and coal insertion, stock burn-time/heat transitions, heat-required
   refusal, exact two-brass output, ordinary collection and normal save/reopen in
   integrated and dedicated/two matching clients, with actual peer presentation.
4. Keep fluids, blaze capture, creative/special fuels, hats/goggles, general
   contraptions and the crafting/acquisition chain outside that first heating
   packet unless separately chosen. Retain original saved IDs/fields and refuse
   unsupported state; do not invent a replacement schema or effects protocol.

The owner must choose the host route before pin/API/acceptance changes under the
adopted plan and AGENTS boundary. Recommended choice: authorize this isolated
candidate pin change and bounded migration work, with another stop if it requires
a broader redesign. Alternatives are an explicitly reduced old-host heating
packet with chosen factory/presentation limits, or keeping the accepted host and
deferring live heating. This is an architecture/scope decision, not an
automatic-approval failure or a reason to rebuild the heat algorithm.
