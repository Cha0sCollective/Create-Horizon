# Create coverage — evidence matrix

Snapshot: November 5, 2026. This is a planning index for **Create only**, not
an overall parity score or a claim that S6 is complete. Other mods are deferred
individually. An imported item, recipe, model or compiled adapter is **packaged
content**; a reviewed source candidate still needs the named build/game checks;
**qualified gameplay** means only the finite cases in the linked result.

The Java behavior reference is Create 6.0.11 at `7ff2760e4924d35d4bc2e22c4b19923f0178cd16`
(`references/create` in this workspace). Its `AllBlocks.java`, `AllItems.java`
and `AllRecipeTypes.java` define the reference families; they do not imply
all entries should be copied at once. The native source/art input is 6.0.10
`ac0c444d9828da3453ae8cc65338e8de063286fb`, with original assets loaded
only from the player's local JAR. Selected 6.0.11 comparisons are recorded per
packet; full-version equivalence is not established.

The accepted private snapshot is primary `1dd69f988b6c1c3edd9cc98289d4f724dd238183`.
The separate reviewed source cohort is `7845ceb82ea5a4f07d9c239e7429ac843aeb24b6`;
its build and gameplay were pending at this snapshot. Public Rust port main was
inspected at `dd0f3141a33851ac20fa98b807a6532471dba727` ([catalog](https://github.com/Delete228/create-horizon/blob/dd0f3141a33851ac20fa98b807a6532471dba727/mixins/create/assets/src/catalog.rs),
[README](https://github.com/Delete228/create-horizon/blob/dd0f3141a33851ac20fa98b807a6532471dba727/README.md)).
Its advertised or packaged features are reuse leads, not private acceptance.

| Create family | Qualified gameplay on accepted primary | Packaged / reviewed source and remaining coverage |
| --- | --- | --- |
| Kinetics and power | [S1–S3](../reference/NATIVE_S3_MVP_RESULTS.md) qualify the finite motor, shaft, cog and millstone profile; [S4](../reference/NATIVE_S4_FACTORY_RESULTS.md) qualifies one water-wheel Survival factory. | The native catalog includes controls, gauges and larger wheels. Java `AllBlocks` also has many sources and controls, including steam power. Mixed sources, arbitrary networks and the full source/controller set are not qualified. |
| Materials, crafting and progression | S4 compares eight selected recipes and proves one iron-nugget andesite-alloy-to-millstone crafting chain from declared vanilla fixture stock. Selected pressing, dry mixing, heated brass and water dough have separate [S6 results](../reference/NATIVE_S6_COAL_HEATING_RESULTS.md). | Accepted primary bundles 127 recipe JSONs, including 38 crafting declarations. These are data availability, not 127 passes. Java `AllRecipeTypes` also registers crushing, cutting, compacting, deploying, filling/emptying, mechanical crafting and sequenced assembly. World-obtainable zinc and a complete Create progression have not been shown. Recipe/material changes need Machinery and Fluids coordination. |
| Processing machines | [Milling](../reference/NATIVE_S1_PLAYABILITY_RESULTS.md), selected [iron pressing](../reference/NATIVE_S6_PRESSING_RESULTS.md), [dry Basin mixing](../reference/NATIVE_S6_DRY_MIXING_RESULTS.md), [coal/heated brass](../reference/NATIVE_S6_COAL_HEATING_RESULTS.md) and [water dough](../reference/NATIVE_S6_WATER_BASIN_RESULTS.md) have bounded results. Primary `docs/teams/machinery/MACHINERY_001_RESULTS.md` adds one-item water-catalyzed fan/Depot flour-to-dough with cold reopen. | Millstone, press, fan, stationary drill, Basin, Mixer and Burner are packaged, with different evidence limits. Java crushing wheels, saw, deployer, spout and broader recipe/media combinations remain outside these accepted cases. The current upstream port has more implementation to assess before adding duplicates. |
| Item logistics and factory handling | Depot/vanilla-hopper paths, finite [mounted Depot](../reference/NATIVE_S6_MOUNTED_DEPOT_RESULTS.md) and [portable interface](../reference/NATIVE_S6_PORTABLE_ITEM_RESULTS.md) are accepted within their packets. Primary `docs/reference/NATIVE_S6_BELT_TRANSPORT_RESULTS.md` accepts one bare horizontal belt and the repaired peer Depot clear. | Belt/connector content is packaged. Java belts, funnels, tunnels, chutes, vaults, filtering and stock logistics are much broader. Transport 002's two-controller handoff is **pending root integration/qualification**, not an accepted extension here. |
| Fluids and thermal systems | The [water-only Basin](../reference/NATIVE_S6_WATER_BASIN_RESULTS.md) qualifies ordinary bucket exchange, selected flour-to-dough and cold reopen; [heated brass](../reference/NATIVE_S6_COAL_HEATING_RESULTS.md) qualifies a finite coal/charcoal Burner path. | Cohort `7845ceb` adds a source-reviewed **stationary water tank** candidate via existing fluid services. Build, transfer/save and client checks were pending. Java pumps, pipes, drains, spouts, fluid networks, other fluids and steam power remain unqualified. |
| Moving contraptions | [S5](../reference/NATIVE_S5_ACTUATOR_RESULTS.md) qualifies one finite piston assembly; mounted Depot and portable item results add selected storage transfer. | Cohort `7845ceb` includes a reviewed **read-only preflight** seam, without a live bearing or custody/motion claim. General assembly, actors, riders, moving storage, collision and trains need separate contracts and evidence. |
| Tracks, trains and schematics | No train or schematic workflow is accepted. | Cohort `7845ceb` carries source-reviewed **straight stationary track X/Z**; shared build and game checks were pending. Java track curves, stations, signals, trains and schematic table/cannon/import remain outside this candidate. Java/world import is outside authorized scope. |
| Player experience and presentation | Selected rendered client and UI observations accompany each accepted packet, not full Create UI parity. | Goggles item/recipe and kinetic HUD are packaged but lack a recorded craft/equip/HUD qualification. Cohort `7845ceb` carries reviewed **single-millstone Ambient audio** with runtime and actual-output evidence pending. Java Ponder, advancement tree, general soundscape grouping, complete effects and client configuration are not qualified; no native advancement files are bundled. |

The candidate sources above do not inherit an accepted result because they are
in the same cohort. Retain each packet's exact source map, binary, client view,
server query, cold-reopen and failed-attempt limits. Do not promote a result
from a recipe count, source review, successful compile, or another team's test.

Coordinator update, November5: shared cohort001 advanced mechanically to abf48a5;
stage/Clippy/game and control build plus23 selected tests PASS. Runtime is pending;
ordinary integrated tank interactions currently fail. This adds no gameplay pass.
Transport002 is now accepted in primary at f3cb1e9; see
[the bounded handoff result](../../orchestration/teams/transport-002-results.md).
Saw002 is source-reviewed in separate candidate9823168, without compilation or
gameplay qualification. The table retains its earlier exact source snapshot.
