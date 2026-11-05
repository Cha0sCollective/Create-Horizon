# S6 Basin/Mixer and heating reuse audit

Assessment starts at accepted portable-interface `6defc66746509cebd2213a41f917bd5e1a051a93`.
Fresh independent read-only Sol High reuse and audit/packet reviews are CLEAR
for the bounded dry packet below; wholesale0.7 import remains HOLD. No engine
upgrade follows from this audit.

## Verified inputs and reuse

Fresh November5 upstream reads found Delete228/create-horizon main still at
[`236c87a8`](https://github.com/Delete228/create-horizon/tree/236c87a889cd76f7e663d1d182c28dc1145f7b14),
preview0.7.0-alpha.2. A separate clean source checkout is local
`.local/evaluations/create-horizon-07/source`; the accepted0.5 snapshot is intact.
Official Horizon main was freshly read at
[`85d8e283`](https://github.com/horizon-rust/horizon/commit/85d8e283f6d97359ad0ff2fc78f7d9b78c898abe).
Keep tested hostfc021038 and Mixinsa1e13326 rather than upgrading automatically.
Upstream0.7 targets host9c7f9b22/Mixinse8cd0aa3. Its API359 integer alone does
not establish source compatibility with our API359 build.

The upstream [roadmap](https://github.com/Delete228/create-horizon/blob/236c87a889cd76f7e663d1d182c28dc1145f7b14/docs/roadmap.md)
records bounded dry mixing, Basin water/lava handling and heated brass, with
burner interaction/presentation and combined networking still incomplete.
These are useful implementations, not private qualification evidence.

| Component under upstream `mixins/create/` | Reuse and boundary |
| --- | --- |
| `engine/src/core/basin.rs` | Existing recipe/inventory simulation, heat predicates and Mixer state machine; reuse exact core before any necessary reviewed correction |
| `basin.rs`, `mixer.rs`, `basin_recipes.rs` | Backport the dry inventory, scheduling, recipe, cycle and rendering paths; exclude unavailable fluid calls and unrelated Press/Basin activation |
| `fluid_codec.rs` | NBT-only canonical tank codec has no newer NativeFluid dependency; reuse to recognize/refuse nonempty or malformed tanks without flattening them |
| `inventory_codec.rs` and helpers | Integrate Basin-only load/save/packet mapping; retain accepted Depot/Millstone paths and sole piston slot unchanged |
| `engine/src/core/transport.rs` | Only its small `SessionCache` utility is needed; extract with attribution rather than importing belts |
| `engine/src/core/burner.rs`, `burner.rs` | Reusable fuel/heat logic for a later bounded heating packet; full adapter also requires newer factory/capture/prediction/sync support |
| Assets/content | Select only Basin/Mixer authored definitions and local-JAR model entries; do not replace the whole catalog/content generator |

The Java6.0.10 baseac0c444d and our6.0.11 reference7ff2760e have no source diff
across `content/processing/basin/`, `content/kinetics/mixer/`,
`content/processing/burner/` and `content/processing/recipe/HeatCondition.java`.
The source declares nine input/nine output item slots, output-first/combined
capabilities as implemented upstream, a two-block vertical operator spacing,
minimum Mixer30RPM, and transient `processingTicks`/`currentRecipe` distinct from
saved `Running`/`Ticks`. Ordinary restart must retain the source's transient reset;
it must not invent serialization of an active recipe or remaining processing.

## Actual compatibility differences

Pinned Horizon already supplies native inventories, recipe compilation/lists,
server tags, recipe remainders, data maps, furnace burn times, block-entity sync,
model/item drawing, sound/particles and ordinary save/hosted play. Reuse those.

Pinned `horizon-client-render/src/chunk/block_entity.rs` has `ModRevision` but
lacks upstream `same_entity`. Full revision equality includes every mutation;
using it would reset Mixer transients during ordinary updates. A tiny private
generated-build module in the defining file can reuse the upstream `born`
comparison as an inherent method, without a new saved identity or official
checkout/public-fork change. Pinned Mixins supports child support modules in
that file; actual stage/index/build validation remains required.

The newer `NativeFluidAction/Drain/RenderOptions/Stack/TankView`, canonical
custom-fluid hooks and fluid renderer are absent from fc021038. The old generic
`ModFluidTanks` interface is not an equivalent replacement. Full fluid reuse is
therefore a separate host/architecture decision, not a reason to rebuild fluids
privately or pretend an empty capability implements them.

Pinned `horizon-mods/src/content.rs` strictly rejects unknown block-definition
fields. Upstream Basin's `collision_context_shapes` and Mixer's
`outline_context_shapes` are unsupported. Import supported fixed shapes only;
contextual physical-item intake remains outside the first hopper-fed profile.
Upstream Mixer tint depends on newer presentation state; retain our existing
white/default tint rather than importing its unrelated effect system.

The upstream kinetic runtime is substantially different. Backport only Mixer's
fixed-Y small-cog/no-shaft connection and4SU/RPM impact, with its local scheduling
callbacks. Preserve accepted S2 solver/source policy and other machine timers.
Add only Basin-local inventory helpers; do not replace shared loaders wholesale.
Keep existing portable-owner change notification when adding Basin notification.

A concrete upstream integration gap was also found: a hopper can modify live
slots after the Basin tick's canonical NBT refresh. The normal save encoder then
writes fresh generic Items alongside stale InputItems/OutputItems, while the
canonical loader ignores generic Items. A Basin-only read-only snapshot at save
and client-update construction derives both mirrors from the live slots, retaining
unknown siblings and refusing opaque sizes. This repair received independent
Sol High CLEAR. Pinned Mixins rejects RETURN on the encoder's nested import, so
the compatible cancellable HEAD handles only supported Basin entities using the
exact pinned Mod encoder branch plus that snapshot; other entities continue.
No live mutation, extra notification or generic durability framework is added.

## Behavior decisions and proposed sequence

The core intentionally reproduces source output overbooking/partial-consumption
and fluid-drain quirks. The owner selected millstone full-output loss only; that
does not decide these additional cases. First use the deterministic dry recipe
`create:mixing/andesite_alloy`: one plain andesite + one plain iron nugget gives
one alloy, with no fluid, random output or container remainder. Admit only this
definition and these plain inputs, including no effective remainder outputs.
Refuse unsupported recipe/payload state without consuming it. That bounds reuse
without silently deciding a new loss policy or claiming complete recipe support.

First complete `NATIVE_S6_DRY_MIXING_PACKET.md`, using ordinary hopper input,
empty-hand collection, native inventories/rendering and normal restart in both
game modes. Then assess a separate heated-brass/ordinary-coal packet, reusing the
burner core and exact source heat requirements. Full blaze capture, client fuel
prediction protocol, fluids/pumps/pipes and their host changes are later decisions.
No broad package import, host pin change, public engine work or new general
machine/lifecycle framework is proposed here.
