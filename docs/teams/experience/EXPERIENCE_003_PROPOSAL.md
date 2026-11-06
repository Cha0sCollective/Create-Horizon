# Experience 003 — Engineer's Goggles workflow and selected kinetic HUD

Status: source-backed proposal for project-lead allocation; no production edit or
gameplay result in this lane. The Experience lane starts at
`727b3e091920d726dc4147926d0979d246bfebf2`. The accepted primary source
inspected for this proposal is `8470fc515d460a95a0cacabc644160f9e7aba4b5`
(native source `55339ce3347b2ce667281e8a3e6318ec856144e4`, package `.11`,
cache14). The primary checkout had concurrent dirty operations/tool files during
this audit; none were changed here. The active Java 6.0.11 reference is
`7ff2760e4924d35d4bc2e22c4b19923f0178cd16`. The public port main was
inspected at `dd0f3141a33851ac20fa98b807a6532471dba727`; use its source as
a reuse lead, not as private gameplay evidence.

## What is present and what is missing

- `native/create/content/data/create/recipe/crafting/kinetics/goggles.json`
  declares one shaped Goggles result from two `#c:glass_blocks`, one
  `#c:plates/gold` and one `#c:strings`. The pinned Java generated recipe has
  the same pattern and ingredients. The native gold plate tag names
  `create:golden_sheet`, and `recipe/pressing/gold_ingot.json` is packaged.
- `native/create/content/data/create/horizon/item/goggles.json` declares a
  one-item stack with `minecraft:equippable` in the head slot. The item model
  selector at `native/create/content/assets/create/items/goggles.json` chooses
  `create:block/goggles` in the head display context. Java's `GogglesItem`
  equips to the head slot through the equipment swap path. Current public port
  main has identical recipe, item, model and gold plate tag blobs.
- `native/create/mixins/create/presentation.rs` has a Goggles HUD path. It
  requires worn Goggles, a Create kinetic node and an eight-block raycast, then
  displays absolute RPM and network stress/capacity from the block entity.
  `motor_hud.rs` connects it to the named HUD hook. The pinned Horizon caller
  invokes `draw_extras` only while the HUD is visible, so F1 already suppresses
  this path.
- The accepted evidence matrix records Goggles craft/equip/HUD as unqualified.
  Iron pressing was qualified; gold-sheet production was not. The coal/heating
  result explicitly excludes hats, Goggles and crafting progression. Packaged
  JSON and a source-level HUD path therefore support neither an obtainable
  Goggles claim nor an observed player/peer overlay claim.

The Java 6.0.11 `GoggleOverlayRenderer` calls block-specific providers. Its
`KineticBlockEntity` consumer text reports nonzero stress impact at current
theoretical speed, and `GeneratingKineticBlockEntity` reports supplied
capacity; other hover providers also exist. The private HUD currently prints
the same network totals on every kinetic node, including shafts. Those totals
are written into each block entity by `native/create/mixins/create/runtime.rs`
and are not per-block impact or supplied capacity.
This is a bounded presentation difference, not evidence that network simulation
is wrong. Public port main has a newer `mixins/create/goggles.rs` provider path;
assess its selected logic before writing a duplicate or importing its wider UI.

## Proposed finite packet

1. Qualify a disposable Survival crafting/equipment flow with **declared fixture
   stock**: one `create:golden_sheet`, two glass blocks and one string. Use the
   ordinary crafting UI, observe one Goggles output and exact input decrement,
   equip to the head slot, and observe actor and peer head rendering. Exercise
   unequip/reequip and a normal cold reopen. This establishes the supplied-sheet
   path only. Machinery owns a separate gold-ingot pressing qualification before
   any world-obtainable or ingot-to-Goggles progression claim.
2. For the same fixture, observe the existing HUD on a selected operating
   millstone consumer and water-wheel generator in integrated and dedicated
   play: client panel text, server RPM/network fields, peer/client view, absence
   without Goggles, absence with F1, and retained behavior after cold reopen.
   Keep the HUD observation distinct from crafting and equipment observations.
3. Make one bounded client-only correction in `presentation.rs` if allocated:
   derive millstone impact from the existing four SU/RPM `Kind::Millstone`
   coefficient and its theoretical RPM; derive selected water-wheel capacity
   from `Kind::WaterWheel { rpm, capacity }` and the Java generator rule. The
   private NBT directly syncs network totals, not provider totals; verify the
   theoretical RPM is present on a dedicated client before calculating an
   overstressed or stopped value. Use zero/idle handling and localized labels.
   Apply Java's spectator guard locally. The pinned host already handles F1,
   but verify that in-game. Preserve the
   existing network totals only where explicitly identified as network totals;
   do not label them as Java per-block impact/capacity. Keep shafts without a
   fabricated provider claim. Reuse the current public-port provider path only
   to the extent it fits the accepted host services and selected machines.
   Review the concrete diff before adding more providers or UI infrastructure.

The source/asset files above are already packaged, so no recipe, item, model or
gold-pressing production edit is proposed without a reproduced failure. No
shared HUD framework, host pin, manifest/cache change, general advancement or
Ponder work belongs to this packet. The project lead controls cache composition,
integration and the single heavy build/game reservation. Machinery owns actual
gold pressing; this team owns the crafting/equipment/presentation observations.

## Checks and stopping point

Before runtime, compare the selected Java provider formulas and current public
port provider implementation against the exact native node fields and client
replica. For a correction, run focused formatting, a source-level HUD test for
consumer/generator/shaft/zero states, a compile check and fresh independent
read-only Sol High review of the stable diff. Under a project-lead reservation,
run one disposable both-mode gameplay milestone with separate launcher/game
paths, recorded source/binary/cache keys, actual client views and normal cold
reopen. Record failed cases as failures, and do not infer visuals from server
queries or traces alone.

Stop at the selected supplied-sheet Goggles workflow and selected kinetic HUD.
Do not claim complete Java overlay behavior, global recipe progression, a
world-obtainable golden sheet, full Create UI parity, or S6 completion. If the
first gameplay check exposes a crafting, equipment or render failure, repair
that observed path before presenting this as qualified. A wider provider or
shared client API need gets a separate dependency/approval decision.
