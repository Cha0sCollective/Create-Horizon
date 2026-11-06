# Create port progress

Updated November 6, 2026. S1–S5 are completed **bounded milestones**; S6 contains
the remaining feature families. They do not divide full Create into equal parts,
and no overall parity percentage is claimed.

## Checked source snapshot

This snapshot copies the 742 tracked native files from accepted candidate008,
`aff11f9af1515dfbb065cd203b9a0de10740d2ab`, with implementation content unchanged. One publication-only notice correction
updates stale compiler provenance; it does not change the tested implementation.
Package 0.5.0-ch4os.15, native asset-import schema 18. The candidate was built on
Horizon 9c7f9b22 / Mixins ac5d9e7 and checked with matching game/control binaries.
Earlier accepted workflows retain their attributed results; their entire test
batteries were not rerun for this publication update.

Java behavior target: Create 6.0.11/7ff2760e, Minecraft 1.21.1 / NeoForge 21.1.219.
Imported port/art target: Create 6.0.10/ac0c444d. Native host: Minecraft 26.3.

| Family | Accepted finite workflows | Major remaining work |
| --- | --- | --- |
| Kinetics/progression | Motor/shaft/cogs/milling, selected arbitration/stress, crafted Survival water-wheel factory | Broader network/source/control combinations, world-obtainable materials and complete progression |
| Processing | Selected Press, Millstone, Basin/Mixer, heated brass, dough, fan washing, upward Saw, stationary Drill and Wheels | Broader recipes/orientations, deploying, filling/draining, compacting, sequenced assembly and crafting |
| Logistics | Selected belt/Depot handoff and Funnel custody route | Chutes, filters/tunnels/vaults, arms, stock/package networks and broad automation |
| Fluids/heat | Water Tank, Basin, selected Burner fuel behavior and Tank/Pump/Pipe/Basin line | Multiblock Tanks, general networks, other fluids, hose handling and steam systems |
| Moving machines | Finite piston, selected carried Depot/portable transfer, inert saved-owner foundation | General capture/actors/contact/riders, broader payloads/chunks and disassembly |
| Trains/schematics | Straight Track placement/lifecycle and Quill selection HUD | Train graph/curves/stations/signals/schedules/cargo and schematic save/preview/deployment |
| Presentation | Selected models/motion/audio, goggles and HUD | Ponder, complete effects/UI/configuration/advancements and performance coverage |

## Latest completed checks

- **Crushing Wheels:** stationary X-axis pair, cobblestone→gravel fallback,
  exact stopped/cold clocks, output collection and occupied-removal recovery in
  both modes. 15 contributing process exits 0. Other recipes/axes/entity damage
  and automatic belt/Funnel wheel input remain outside this result.
- **Drill:** selected east-facing stone path at 32 RPM / 128 stress, ordinary drop
  collection, integrated bedrock refusal, stopped/cold clocks and resume in both
  modes. 10 contributing process exits 0. Moving Drills and broader materials remain.
- **Water line:** selected Tank/Pump/Pipe/Basin transfer, refusal/recovery and
  normal cold retention, with actual peer observations. 11 contributing exits 0.
  Creative bucket drain behavior does not establish Survival fluid conservation.
- **Tank lifecycle:** normal diamond-pick break and Shift-wrench removal passed
  integrated and dedicated checks. Mining produces one plain Tank world item for
  ordinary collection; wrenching returns one directly. Both replace empty, with
  old block entity/water absence and empty dedicated-peer inventory checked.
  Four process exits 0; four controller cases totaled 46.418 seconds, excluding
  startup/review. This adds no new cold-reopen or multiblock result.

Original failed attempts remain failed records. Later corrections and successful
trials do not retroactively change them. Normal save/reopen is not a claim of
atomic crash recovery across world and player files.

## Next integration and route to parity

The approved Chute packet reuses upstream c92212b's existing dry vertical core,
including multislot up-to-16 extraction and Java output timing. Its isolated source
adds one canonical slot and saved progress, with single-chest/stacked-Chute transfer,
blocked reopen and normal removal checks planned. It is not gameplay-qualified or
included in this source snapshot yet.

The development route is to consolidate accepted work, complete usable automated
factories, broaden fluids/power, finish moving machines, then complete advanced
logistics/trains/schematics and residual content/presentation. Existing upstream
modules take priority over new implementations; overlapping owners and saves are
reconciled before migration. Current builds are for development in disposable
worlds. No public release or compatibility promise follows from this checkpoint.

Inherited native README, PLAN and launcher documents describe earlier upstream
baselines and may contain obsolete implementation or installation statements.
The root README, this progress report and exact compatibility pins describe the
current checkpoint. Use the development build limitations in the root README.
