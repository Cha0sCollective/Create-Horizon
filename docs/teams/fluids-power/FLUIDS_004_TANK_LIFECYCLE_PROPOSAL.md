# FLUIDS-004 — regular Tank Survival and filled-removal proposal

**Status:** source audit and finite proposal only, November 5, 2026. No source,
schema, host pin, build, game, or acceptance change is in this packet. The
accepted gameplay checkpoint is private `55339ce3347b2ce667281e8a3e6318ec856144e4`
(`.11`, cache 14); the primary documentation checkpoint is `8470fc515d460a95a0cacabc644160f9e7aba4b5`.
`FLUIDS_003_COHORT_RUNTIME_HANDOFF.md` qualifies one Creative-player,
one-block water Tank and ordinary cold water retention. Filled removal and
Survival fluid play remain **unqualified**.

## Contract found in the pinned sources

| Question | Pinned Java Create 6.0.11 | Accepted native and Horizon service | Consequence |
| --- | --- | --- | --- |
| Regular Tank plus Survival bucket | `FluidTankBlock.useItemOn` returns `PASS_TO_DEFAULT_BLOCK_INTERACTION` before fluid capability or item exchange (`FluidTankBlock.java:155-183`). The separately registered creative Tank is an exception (`AllBlocks.java:984,1000-1001`). | `fluid_tank.rs:201-213` gates its bucket adapter on `game.creative`; `runtime.rs:390-424` sends nonmatching use onward. | A direct Survival bucket-to-Tank exchange would change the Java regular-Tank contract. Do not borrow the Basin's separately implemented bucket behavior. |
| What a passed bucket does | The Tank block has not claimed the use. | Host `game.rs:13445-13542` passes bucket `UseItemOn` to vanilla `UseItem`. Empty buckets use world `bucket_pickup` (liquid source, waterlogged block, snow or bubble column); filled buckets target world placement, generally beside a solid Tank (`horizon-world/src/level/buckets.rs:38-61,103-163`; `game.rs:13028-13052`). Neither path calls TankContent or the native block-fluid capability. | An empty bucket aimed at stored Tank water has no Tank drain path. A full bucket can affect adjacent world water and its held item without filling the Tank. Record both owners separately in any live check. |
| Available transfer service | Eligible Java exchanges and other fluid machinery use a fluid capability; the regular-Tank Survival block click is not that machinery. | The accepted adapter already exposes a strict one-block, plain-water tank view and simulate/execute fill/drain (`fluid_tank.rs:20-187`; `fluid_tank_codec.rs:17-115`). Horizon has explicit native block-fluid calls and item-fluid helpers (`game/machines/native_fluids.rs:312-479`; `game/native_item_fluids.rs:183-235`). Its vanilla bucket hook is a cancellation hook (`game/native_hooks.rs:6725-6733`), not an automatic capability bridge. | Reuse the existing capability when a source-backed Survival machine reaches it. A new direct-click bridge is not justified by these sources. |
| Filled normal break and sneak wrench | The generated Tank loot table gives one plain `create:fluid_tank`, with no block-entity-data copy. `IWrenchable.onSneakWrenched` requests that loot, returns it to inventory, then destroys without another drop (`IWrenchable.java:48-72`). `FluidTankBlock.onRemove` removes the entity on block-type change (`FluidTankBlock.java:260-267`). | The accepted loot table is also one plain item (`native/create/content/data/create/loot_table/blocks/fluid_tank.json`). Normal break runs the host loot route (`game.rs:6207-6299`); a valid Shift-wrench uses the generic plain-item/remove route (`fluid_tank.rs:271-285`; `interactions.rs:160-193`). | One ordinary dropped/returned Tank item carries no water; the old controller's water ends with removal. This is a source inference, not a live result. |
| Same-block state update and later placement | Java `onRemove` retains the entity when the block remains a Tank with a block entity. `FluidTankItem.updateCustomBlockEntityTag` sanitizes and caps data **if an item already has** `BLOCK_ENTITY_DATA`; it does not create that data on ordinary loot (`FluidTankItem.java:43-66`). | `native_set_block_property` retains the existing entity; `fluid_tank.rs:287-309` toggles window with that service. A newly placed plain item gets the empty one-block controller (`runtime.rs:135-159`). The codec refuses foreign, connected and opaque data (`fluid_tank_codec.rs:26-93`). | A window toggle should keep stored water; placing the ordinary plain drop should start empty. Component-bearing pick-block or edited items are separate cases. |

All Java references above are under the pinned
`G:\Create Rusty\references\create\src\main\java\com\simibubi\create` tree,
except the generated loot table under `src\generated\resources`.
All `native/create/...` references above are to accepted Git revision
`55339ce3347b2ce667281e8a3e6318ec856144e4` in the private repo (inspect
with `git show 55339ce:<path>`). The physical integration checkout may advance
to a later candidate and is not the accepted source identity. Host references are to
the pinned `9c7f9b22` source at
`G:\Create Rusty\.local\evaluations\horizon-heating-refresh\source`.
The locally preserved newer upstream port snapshot at
`G:\Create Rusty\.local\evaluations\create-horizon-08-review\source`
at `dd0f3141a33851ac20fa98b807a6532471dba727`
has Basin fluid hooks in `mixins/create/fluid-custom_fluid_tanks.rs:1-5`,
but no Tank, pipe, or pump adapter to import. The current public
[Create development Tank source](https://github.com/Creators-of-Create/Create/blob/mc1.21.1/dev/src/main/java/com/simibubi/create/content/fluids/tank/FluidTankBlock.java)
still shows the same Survival pass-through; it is a cross-check, not a new pin.

## Smallest next packet for project-lead reconciliation

**FLUIDS-004A, evidence only:** reserve one shared runtime slot for a finite
regular, isolated, one-block Tank profile on the exact frozen `.11` source
inputs, input map and binary from cohort003, or on a separately reviewed
candidate explicitly identified before launch. Reuse the
cohort's normal integrated and dedicated/two-client harness if the project lead
chooses both modes; do not replay the cohort003 fixtures or relabel its results.
Use ordinary Creative clicks to put exactly 1,000 mB water in each test Tank,
then switch the actor to Survival for the Survival and removal observations.
This setup proves the initial content without writing TankContent directly.

1. With a filled Tank, toggle its window using the normal wrench. Check the
   state, controller amount and peer view; the same position still has exactly
   1,000 mB. Record a normal cold reopen if the slot includes persistence.
2. On an isolated regular Tank, aim an empty bucket and then a full water bucket
   in Survival. Check main/offhand inventory, TankContent, block at the clicked
   position and every adjacent world-water target. Expected TankContent is
   unchanged; a world placement, if it occurs, is counted as world fluid.
3. With a separately filled Tank, use an ordinary correct, unenchanted tool in
   Survival. Check block removal, exactly one plain Tank loot item, its complete
   components, old block-entity absence, nearby fluid/item entities and any
   world-water spill. Collect and place that item at a new position; expect an
   empty Tank. Keep actor, peer and world item ownership distinct.
4. Repeat on another filled Tank with Shift-wrench and free inventory space.
   Check that one plain Tank returns to inventory, no second world item appears,
   the old controller is gone and its water is absent. Place the returned item
   and inspect its empty controller.

The packet passes only when the observed inventories, world fluid, block
entities and drops reconcile exactly and the ordinary cold/replacement claims
are stated only for the sessions that actually check them. A discrepancy is a
targeted repair candidate for the observed path; retain the failed receipt and
send it to the project lead before source edits. No multiblock, boiler, pipe,
network, generic container, custom crash, or new host framework belongs here.

**Later Survival use:** a filled/drained regular Tank in Survival needs a
separate concrete Create transfer device or other source-backed capability
consumer. The present native tank view/fill/drain service can be reused for
that future packet, but the current audit supplies no ordinary player path
through the vanilla bucket. Choose the first finite producer/consumer and its
owner/capacity/cold checks only after project-lead reconciliation. The accepted
Creative Tank and Basin outcomes remain separate claims.

## Work accounting

Lead `/root` worked in `codex/team-fluids-power` from clean base `5a87f94c`.
Read-only specialists: `/root/survival_bucket_contract` checked the Survival
and vanilla bucket boundary; `/root/filled_removal_custody` checked Java and
native loot, removal and same-block retention. Neither edited production or
ran a heavy job. Their recorded read-only intervals were 01:21:50–01:27:29 UTC
(5m39s) and 227.009 seconds respectively. Independent read-only Sol High
source/scope review of this proposal cleared after the revision-identity
correction. The project lead then allocated an unrun packet-local checklist
and driver; their separate read-only Sol High delta review is CLEAR.
The project lead owns integration, runtime reservation and acceptance.
