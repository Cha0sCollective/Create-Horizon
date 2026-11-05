# Machinery 002 — stationary Mechanical Saw item cutting

Base: accepted private `1dd69f988b6c1c3edd9cc98289d4f724dd238183`
(`0.5.0-ch4os.9`, Horizon `9c7f9b22`, Mixins `e8cd0aa3`, API 359).
Machinery lane: `codex/team-machinery`. This is a separate S6 source packet;
the accepted fan evidence and first cohort remain historical inputs.

## Selected reference and behavior

Pinned Create 6.0.11 reference `7ff2760e4924d35d4bc2e22c4b19923f0178cd16`:
`SawBlockEntity`, `SawBlock`, `ProcessingInventory`, `CuttingRecipe`,
`AllBlocks`, and generated `data/create/recipe/cutting/andesite_alloy.json`.
Current Delete228/create-horizon main `dd0f3141a33851ac20fa98b807a6532471dba727`
has no saw/cutting implementation in its 992-path tree; its sole difference
from local upstream snapshot `236c87a8` is a license-notice change.

The selected upward-facing saw accepts one ordinary dropped item while powered
into its own inventory. Its selected `create:cutting` recipe consumes one
`create:andesite_alloy` and yields exactly six `create:shaft` with no chance
roll. `processing_time: 200` is base processing units, not fixed world ticks;
Java decrements by `clamp(abs(RPM)/24, 1, 128)` per powered tick. A distinct
20-unit post-application phase precedes ejection. At zero RPM progress pauses.
An unsupported item takes the source's 10-unit no-recipe path and is ejected
unchanged. The saw has 32 saved processing slots, one-item default input,
processing time, recipe duration and applied flag. Empty-hand collection may
retrieve held contents. The selected integration admits no automation face,
belt, filter or bulk path.
Native intake uses the existing grounded-item top-contact query; Java invokes
its fall callback. This packet tests an item landing while already powered and
does not claim identical behavior for an item left resting before power starts.

The upward saw has two opposing horizontal shaft ports chosen by
`axis_along_first`, with 4.0 SU/RPM impact. Item travel is perpendicular to the
selected shaft axis, with its direction determined by signed speed. The
horizontal block/tree-breaking and entity-damage paths are separate and not
implemented in this packet;
non-up saw states are inert and expose no kinetic node here.

## Ownership and proposed source

Machinery owns the new pure saw cycle, `engine/src/core/saw.rs`, and native
adapter `mixins/create/saw.rs`, including item capture, recipe decode,
inventory persistence, result ejection and selected blade rendering. Reuse
existing motor/network, item-entity, recipe, generic container, save/update,
model and player collection services. A narrow saw kinetic kind may be needed
for two-sided ports and impact; no host/API/global schema change is planned.

The lane may propose shared registration and placement changes in
`engine/src/lib.rs`, `mixins/create/runtime.rs`, `placement_runtime.rs`,
`horizon-mixin.toml`, data-refresh hooks, `tools/content.py`,
`tools/catalog.py`, generated content/catalog, manifest/version and notices.
The project lead resolves these shared files serially across cohorts. Depot,
belt transport and their callbacks remain Transport-owned and untouched.
Original Create art remains an offline import from the player's pinned
6.0.10 JAR; committed fallback models use vanilla texture IDs.
The pinned Java crafting recipe (iron plates, iron ingot and andesite casing)
is included so the selected saw block remains obtainable without a new material.

## Source and acceptance checks

Pure tests check 200/20-unit RPM scaling, zero-power pause, single-item output,
and unsupported-item unchanged ejection. Focused native checks cover source
recipes, ports/stress, placement, item custody, save and render declarations.
After a host-wide lease, one isolated integrated and dedicated/two-client
milestone should show ordinary input, exact six-shaft output and collection,
unsupported return, mid-cut stop and normal cold reopen, actual peer-visible
blade/input/product/empty state and normal exits. Preserve raw failures and
separate server authority from client appearance.

Stop for a host/API or global-schema gap, broader machine framework,
changed acceptance, or another team's shared contract. No tree felling,
entity damage, belts, filters, bulk or alternate cutting, stonecutting,
general world compatibility, crash-atomicity or release claim follows.
