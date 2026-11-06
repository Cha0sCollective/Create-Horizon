# Third-party work

Create reference: Creators-of-Create/Create, commit
`ac0c444d9828da3453ae8cc65338e8de063286fb`, release 6.0.10 (Minecraft 1.21.1).
Copyright (c) The Create Team / The Creators of Create.

The following MIT code informed or was adapted into the Rust engine:

- `content/kinetics/KineticNetwork.java`: capacity and stress scale with absolute RPM.
- `content/kinetics/RotationPropagator.java`: supported coaxial and small/large cog connections.
- `content/kinetics/press/PressingBehaviour.java`: 240-unit cycle, midpoint processing,
  clamped speed interpolation, integer truncation and the rendered head's motion curve.
- `content/kinetics/press/BeltPressingCallbacks`, `MechanicalPressBlockEntity`,
  `logistics/depot/DepotBehaviour`, `DepotItemHandler` and `SharedDepotBlockMethods`:
  one-at-a-time default pressing, output buffering, automation and landed items.
- `content/kinetics/simpleRelays/GearboxBlockEntity`, `ClutchBlockEntity`,
  `GearshiftBlockEntity`, `crank/HandCrankBlock[Entity]`, `motor/CreativeMotorBlockEntity`,
  `foundation/block/IWrenchable` and encasing behaviors: implemented transmission,
  placement, wrench, hand crank and motor controls. Native UI drawing is our own.
- `base/KineticBlockEntityRenderer`, `KineticBlockEntityVisual`, `gearbox/GearboxRenderer`,
  `transmission/SplitShaftRenderer`, `crank/HandCrankRenderer`, `logistics/depot/DepotRenderer`:
  positive world-axis transforms, phase, crank interpolation and item piles.
- `gauge/GaugeBlock[Entity]`, `SpeedGaugeBlockEntity`, `StressGaugeBlockEntity`,
  `GaugeRenderer`, `base/DirectionalAxisKineticBlock`, `gearbox/VerticalGearboxItem`,
  `equipment/goggles/GogglesItem`, `GogglesModel`: placement, instrument targets,
  comparator output, equipment and display contexts.
- `waterwheel/WaterWheelBlock[Entity]`, `LargeWaterWheelBlock[Entity]`,
  `LargeWaterWheelBlockItem`, `WaterWheelStructuralBlock`, `WaterWheelRenderer`:
  flow projection/scoring, generated speed, capacity, materials and structural occupancy.
- `content/kinetics/millstone/MillstoneBlock`, `MillstoneBlockEntity`,
  `MillstoneRenderer`, `MillingRecipe`, and `AllShapes`: fixed-axis/cog connections,
  collision, filtered input/output inventories, timer, collection and moving part.
- `content/kinetics/fan/EncasedFanBlock`, `EncasedFanBlockEntity`,
  `IAirCurrentSource`, `AirCurrent`, `EncasedFanRenderer`, `FanVisual`,
  `fan/processing/FanProcessing`, `AllFanProcessingTypes`, `SplashingRecipe` and
  `HauntingRecipe`: placement and shaft direction, airflow range/segments,
  obstruction sampling, dropped/depot processing timers, catalyst rules,
  furnace recipe selection and propeller speed.
- `content/processing/recipe/ProcessingOutput`: per-output-item probability rolls
  and result construction. `content/kinetics/belt/transport/TransportedItemStack` supplies the
  transported fan-processing field conventions.
- `content/logistics/funnel/FunnelItem`, `FunnelBlock`,
  `AbstractFunnelBlock`, `FunnelBlockEntity`, and `infrastructure/config/CLogistics`:
  selected downward Andesite Funnel placement, redstone pause, one-item
  extraction, outlet obstruction and cooldown. Belt Funnel remains unported.
- `content/kinetics/drill/DrillBlock`, `DrillBlockEntity`, `DrillRenderer`, and
  `content/kinetics/base/BlockBreakingKineticBlockEntity`: stationary front-block
  selection, break progress/retry timing, source NBT names, ordinary loot, rear
  shaft and rotating head. Contraption movement and entity damage are not ported.
- Pinned Create 6.0.11 `7ff2760e4924d35d4bc2e22c4b19923f0178cd16`:
  `content/kinetics/saw/SawBlock`, `SawBlockEntity`, `SawRenderer`,
  `content/processing/recipe/ProcessingInventory`, and generated
  `data/create/recipe/cutting/andesite_alloy.json`: selected upward-facing
  one-item saw clock, 4 SU/RPM ports, inventory fields and exact shaft recipe.
  Tree cutting, entity damage, belts, filters and bulk processing are absent.
- `AllSoundEvents` and infrastructure configs: selected composite sound events,
  default gains/pitches, stress and speed parameters.
- 123 MIT datapack recipes and 35 behavior/ingredient tags, exported from
  `src/generated/resources/data/` at the pinned revision. Ingredient syntax is
  adapted to Horizon/Minecraft 26.3. Missing vanilla convention tags are our own
  definitions and supplement Create's tag additions with vanilla entries.
  Fan block tags omit unimplemented Create blocks. Recipes requiring unavailable
  items or other mods are excluded.

Paths above are under `src/main/java/com/simibubi/create/`, except the recipe/tag data under
`src/generated/resources/data/`. The MIT notice is in LICENSE.
Version 0.5 remains incomplete and does not claim full behavioral parity.
Validation evidence and its limits are recorded in `VALIDATION.md`.
Rust code organization is native to Horizon; source behavior is the reference,
not one Rust type for every Java class.

Create's original `src/main/resources/assets/` has a separate All Rights Reserved
license. Development inputs and port-pack carry-over stay under `.local/`.
The optional offline importer reads a JAR supplied by the player in their instance's
`horizon-mods/create-resources/`, then caches the required resources under that
instance's `.horizon-cache/create-assets/`. It does not download or execute the JAR,
modify the installed mod or grant rights to redistribute the imported resources.
The input JAR and generated caches are excluded from Git and all release archives.
Local extraction is not a guarantee of legal clearance. See `ASSETS.md`. The default
content uses our provisional JSON models and refers to vanilla textures by ID;
it embeds no Mojang textures or original Create textures.

The resource bootstrap is the published Create **6.0.10** release `UjX6dr61` on
Modrinth. Its provenance and verified hashes are in `.local/import-provenance.json`.
Its source, behavior oracle and resource baseline now match the same published
6.0.10 release. Every locale is imported from the local JAR without redistribution.

Additional MIT references used in the placement/depot adapter:
- `RotatedPillarKineticBlock`, `DirectionalKineticBlock`, `CreativeMotorBlock`,
  `CogWheelBlock`: placement choice and neighboring shaft ports.
- `SharedDepotBlockMethods.onUse`, `DepotBehaviour`, `TransportedItemStack`:
  top-face exchange and saved field names. Java method bodies used by the oracle
  are AST-exported into `.local` and are not shipped as player code.
- Horizon `horizon-world/src/level/place.rs` (MIT, horizon-rust contributors):
  the nearest-looking-direction calculation, adapted to the placement adapter.

Horizon and Horizon Mixins are MIT. The current source pins Horizon
`9c7f9b2215dfed497aba5522c44f38926628648d` and reviewed Mixins
`ac5d9e766380d575838c8250f2a7982281c1f44e`, based on official Mixins
`e8cd0aa38cffef49cb2e088ef716baa107148c02`. The local compatibility overlay
includes MIT compiler replacement sources under the build tools, with their
notices preserved. This compiler profile is source-workspace-only; a generic
official archive does not provide it. Build portability remains unfinished.
These source pins and local adaptations do not imply upstream approval.

Private S5 feasibility increment: `engine/src/core/linear_carrier.rs` is informed
by Create6.0.11/7ff2760e `KineticBlockEntity.convertToLinear`,
`MechanicalPistonBlockEntity.getMovementSpeed` and
`LinearActuatorBlockEntity.onSpeedChanged`. Its scope is selected one-cell motion
math, not a complete piston/contraption port. `s5_clearance.rs` adapts/reuses MIT
Horizonfc021038 `game/piston_push.rs` body selection, piston displacement limits,
collision resolution and movement application for the bounded private probe.
