# Machinery 001 — bounded fan washing over a Depot

Status: allocated for planning; no game/build reservation and no acceptance yet.
Lane: `codex/team-machinery` at `76dfaf78370f41b2a5c68620c55be6ecab96f16f`,
based on accepted water `02d029f3bea09c62480e07e315e23d4868f9180a`.
Keep the host, Mixins, API, Rust, native/content and art pins in `workspace.toml`.

## Why this packet is new

Accepted S4 qualifies a water-powered hopper/millstone factory and explicitly
leaves fans for later. Accepted S6 water qualifies flour plus 1000 mB water in a
**Basin**, not a fan air current. Pressing qualifies a press over a Depot, not
fan processing. The current native port contains an encased fan and a deterministic
splashing recipe, but no accepted private result for catalyst detection and
processing a Depot-held item, saved fan progress or peer presentation. This
packet qualifies only that distinct path; it does not repeat the S4 crafting,
kinetic or Basin bucket batteries.

## Reuse and selected behavior

- Existing machine-owned paths: `native/create/mixins/create/fan.rs` and
  `native/create/mixins/create/engine/src/core/fan.rs`. Reuse existing fan
  registration/kinetics/rendering, Depot slots and sync, ordinary save and
  checked-in `content/data/create/recipe/splashing/wheat_flour.json` with
  `content/data/c/tags/item/flours/wheat.json`. No source or content edit is
  planned. Do not import the newer upstream fan wholesale.
- Pinned Java Create 6.0.11 `7ff2760e4924d35d4bc2e22c4b19923f0178cd16`:
  generated `data/create/recipe/splashing/wheat_flour.json` and
  `data/c/tags/item/flours/wheat.json` select one `create:wheat_flour` to one
  `create:dough`. `AllFanProcessingTypes`, `AirCurrent`, `FanProcessing`,
  `DepotBehaviour` and `TransportedItemStack` cover water-catalyzed airflow,
  the handler one block below the air current, a default 150-tick processing
  base scaled by `ceil(stack count / 16)`, and held-item fan progress NBT.
  The selected test uses one item; it makes no stack-threshold or tick-exact
  parity claim.
- A local upstream source snapshot at `236c87a889cd76f7e663d1d182c28dc1145f7b14`
  contains this bounded fan path. The November 5 default branch
  `dd0f3141a33851ac20fa98b807a6532471dba727` is one license-notice
  commit later; its compare lists no fan or recipe file change. Neither source
  snapshot is a current-native gameplay receipt.
- The historical `native/create/tools/fan_checks.py` supplies a useful
  geometry: horizontal fan, waterlogged oak fence in the current, Depot one
  block below the downstream air cell. Its earlier smoke-world assertions
  are setup guidance only; do not relabel them as acceptance on this lane or
  replay its entire fan battery.

## Smallest outcome and evidence

Use isolated, disposable normal host-generated Survival worlds with the
recorded mob-free rule. Supply the motor, fan, Depot, plain flour and catalyst
as explicit fixtures; qualify ordinary fan/Depot placement and item interaction,
not material acquisition or ordinary fence waterlogging. Record the exact
positions, RPM, air direction, selected recipe, item counts, machine NBT, game
tick order, player actions and process exits. A fixture may create the
waterlogged fence after ordinary machine placement; label it as a fixture.

In one integrated session, show these machine-specific controls:

1. With one plain flour held by the Depot, no water catalyst, zero RPM and an
   unsupported input each leave their own input intact and produce no dough.
   Clear each case before the next; do not infer refusal from a command reply.
2. With a powered fan and waterlogged fence in the current, one plain flour
   becomes exactly one dough in the Depot. Ordinary collection empties the
   Depot. Account for player, Depot and dropped-item owners without duplicates
   or unexplained loss.
3. Stop power partway through a fresh one-item conversion. The held item and
   `FanProcessingType`/`FanProcessingTime` remain stable while stopped; resume
   and finish once. Treat observed timing as measured behavior, not an
   unverified exact Java event-order match.

In one dedicated session with two matching rendered clients, use the same
selected machine/recipe and stop during processing. After normal cold close and
reopen of server and clients, verify saved held item and fan progress, resume,
collect exactly one dough, and show the peer's actual occupied-to-product-to-empty
view. Separate authoritative server counts/NBT from client images or render
traces. Reuse unchanged host, kinetic, Depot and general save evidence from
accepted results; do not run a broad regression battery.

Use an unchanged accepted game binary only if a fresh lane-correct input map,
binary hashes, content and launch-driver checks prove it matches this HEAD and
the isolated runtime. Otherwise request the central build reservation and run
only the required focused compile/validation. A game/server or Java reference
run also needs the project lead's explicit host-wide reservation. Keep worlds,
generated resources, binaries and raw runs outside Git; preserve every FALSE/HOLD
attempt and normal process-exit receipt. A completed packet gets a concise result
with exact inputs, evidence and limits plus the applicable independent read-only
Sol High completion review.

## Ownership and stop conditions

Machinery owns this packet/result and any defect-specific edit in the two fan
files named above. Transport owns shared Depot/transport callbacks; do not edit
them while Transport 001 is active. A defect requiring shared Depot behavior,
runtime wiring, catalogs, pins, schema, host API or a different acceptance policy
returns to the project lead for a named allocation before production work.
Any narrow fan-owned behavior repair needs focused checks and fresh independent
read-only Sol High source review. Belts, loose item entities, other fan media,
chanced/stack recipes, Basin fluids, crafting progression, general compatibility,
interrupted-crash guarantees and release are outside this packet. S6 remains open.

References: `docs/reference/NATIVE_S4_FACTORY_RESULTS.md`,
`docs/reference/NATIVE_S6_PRESSING_RESULTS.md`,
`docs/reference/NATIVE_S6_WATER_BASIN_RESULTS.md`,
`docs/CURRENT_WORK.md` and `native/create/tools/fan_checks.py`.
