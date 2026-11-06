# Machinery 004 — integrated wheel intake HOLD

**HOLD, not gameplay acceptance.** Root granted the exclusive
`machinery-crushing-wheels007-runtime` lease for the frozen combined007
source. Machinery used only the reviewed `ad254a2` plan and packet-local
driver SHA-256
`5242e0834a6472b5b45b7cc77010b17806382adef5c98c8f14c5acd6c7c997cf`.
No production, schema, host, pin or resource change occurred. The first
integrated one-item input missed the selected controller; the driver stopped
after its 40-step diagnostic ceiling. There was no replacement input, retry,
dedicated run, cold reopen, clutch or removal claim.

## Frozen run and receipts

- Source `f732bd481cfc3d9c4720bc51e4a31e7385e18020`, native tree
  `471136a9f27049a58bac2707c75b18466e94e17c`, 742 mapped files;
  source-map SHA-256
  `0c97fc70d5ce5a8dc81bed1b5ccc21887fbc69628c94e0b0d1a6aff015c70f8a`.
- Game SHA-256 `fb8ec9b36084b3a5d4d77e56ea4dcb7cd5f1b49eac4d52fc2998f50cdc2ce2db`;
  control SHA-256
  `170e866057d97e73b9231c8084547d4a4a02346238db35901ef64bf3d0d6d449`;
  actual actor JAR SHA-256
  `ef87fe5709f1ba1f5b8bb20a2925b5afb4669e178fd6d8bf10c167759eefe37a`.
  The owned resources link resolves to `26.3-cf62fe-smoke` and its
  `facts.json` SHA-256 is
  `1d24883ee807b0b71473f0bbf8ed548ddaf712148caeb3fcc7aa13ab973a97b0`.
- Fresh integrated run `integrated-ed08kkz7`, center `(8,80,8)`.
  Packet-local receipts:
  `.local/machinery-004/receipts/integrated-integrated-ed08kkz7/00-setup-01.json`
  (PASS) and `00-pause-01.json` (HOLD). The only actual actor frame is
  `00-setup-01-running-wheels-gap-actor.png`; it predates the failed throw.
  No post-HOLD image was captured before normal exit, so there is no visual
  claim for the item at the final tick.
- Setup's marker-bounded server rule feedback changed
  `spawn_mobs` from `true` to `false`. It verified the fixture blocks,
  selected pair, controller, support and open output column. The world was
  frozen at game time `1067`. `pause` completed 40 exact one-tick steps,
  each with stable time between steps, through `1107`; all 1,287 control
  subcommands returned code 0, but the case assertion failed.
- The actor's ordinary `give` and single `drop` reduced inventory cobblestone
  from one to zero. The first server item query recorded one world cobblestone
  at `Pos [10.5,82.32000000476837,8.5]`,
  `Motion [-0.22528179473281187,-0.14263823683806964,0.0035730797735700295]`,
  `Age 0`, `PickupDelay 40`, count one, UUID
  `18e4950c-64dc-4932-a548-c2e4190751b5`.
- After normal save/quit, read-only decoding of the owned saved entity chunk
  at `LastUpdate 1107` found the **same** cobblestone UUID and count one,
  `Pos [9.125,65.0,8.581107166317278]`,
  `Motion [0.0,-0.12,0.0004125768558443268]`, `Age 40`,
  `PickupDelay 0`. The actor inventory was empty. The final controller block
  query remained `facing=down,valid=true`; its saved primary Inventory and
  bridge both held `Size 32`, `Items []`, `ProcessingTime 0`,
  `RecipeTime 0`, `AppliedRecipe 0`, and `Speed 16`.
- The saved first wheel `(8,80,7)` was `Running`, `Speed +16`, stress 128,
  capacity 262144. The saved second `(8,80,9)` was `Running`, `Speed -16`,
  stress 128, capacity 262144. No gravel appeared in the actor's radius-32
  item census. The unrelated red mushroom in that census was outside the
  fixture. Integrated `save` and `quit` succeeded; root helper's
  `exit-00.json` reports actor code `0`. No game process remains in this
  Machinery slot, and the lease is released for root scheduling.

## Source-only interpretation and narrow next proposal

The selected adapter accepts a cobblestone world item only when it lands on
the down-facing controller top at `Y+0.8125`, with its center within
`X+0.25..0.75` and `Z+0.25..0.75`. This fixture's X window was
`8.25..8.75`. The item began at `x=10.5` with westward velocity about
`0.2253` and finished at `x=9.125`, exactly one item half-width (0.125)
east of the controller's `x=9` face. The pinned host gives items width
0.25. Its initial downward motion reaches the top plane within about six
world ticks, before enough westward travel to enter the selected X window.
The final side-adjacent position and fall to ground support a **fixture
trajectory miss**. The run did not exercise confirmed contact or capture,
so it cannot establish whether the adapter would succeed with a valid landing.

The frozen compiled stage contains the HEAD calls to
`crushing_item_tick_listened` and `crushing_item_tick_pre` in the host's
actual `native_entity_tick_listened` and `native_entity_tick_pre` bodies.
`native_tick_round` calls the listener on the server before item Pre; the
injected call runs before dispatch to the initially empty trait-mod list.
The actual pair/controller runtime also executed. Thus an empty trait-mod
list does not by itself prove a missing listener. This run has no trace of a
contact callback for a qualifying item, so it is not an adapter PASS either.

The smallest proposed correction for a **newly reviewed and separately
granted** run is fixture-only: add a throwing pad at `(X+1,Y,Z)` and throw
from its center `(X+1.5,Y+1,Z+0.5)` toward the same controller top. That
shortens horizontal travel and steepens the aim while retaining the exact
one-item recipe, pair, clock, clutch, save and spill criteria. Inspect the
first item's server trajectory and stop if it still misses; do not write
item position/motion/age or controller NBT. No source, host, API, schema,
pin, recipe, axis or framework change is proposed by this HOLD.
