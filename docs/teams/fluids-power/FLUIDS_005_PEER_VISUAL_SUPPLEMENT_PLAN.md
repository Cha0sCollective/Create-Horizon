# FLUIDS-005 peer Basin view: source diagnosis and one same-world supplement

Status: **VISUAL HOLD; plan only.** No process was started and no game or control
source was changed for this diagnosis. The dedicated world is the normally closed
`G:\Create Rusty\.local\native-fluids-005-runtime\dedicated-d1m1sey4` run,
after session 01. A further session requires the project lead's heavy runtime
reservation and a reviewed finite control. It must preserve the prior images and
raw result; it does not replace the already passing rate, reverse, stop, cold,
capacity, and process-exit evidence.

## Concrete diagnosis

* Current peer framing used `tp TeamPeer 26.5 66 23.5` and
  `look-at 23.5 65.6 20.5` for Basin `(23,65,20)`. Its recorded peer `look-at`
  pitch was **13.518 degrees**. The previously accepted dedicated Basin water
  view used `tp @s 7.5 66 12.5` and `look-at 6.5 65.4 11.5` for Basin
  `(6,65,11)`, an offset of only `(+1,+1)` and recorded pitch **57.501 degrees**.
  Its water and empty peer images show a visible blue interior and dark empty
  interior. The current offset `(+3,+3)` hides most of that interior behind the
  rim. This is a strong camera explanation,
  but the current three frames alone cannot prove peer replication.
* The executed Pump fill calls the Basin `fluid_fill` callback; a successful
  `Execute` calls `commit_fluids`, which writes `InputTanks` and calls
  `native_block_entity_sync`. Basin bucket drain follows the same commit path.
  The host queues that position, broadcasts `BlockEntityData` to viewers of its
  loaded section, and the Create update-tag callback sends a snapshot of the
  entity NBT. The remote client merges that tag. Basin `client_tick` reads the
  received `InputTanks` and chases its fluid level; `draw_fluids` draws the water
  box if rendered units reach 1 mB. The source route therefore has a sync path,
  but source inspection cannot prove packets reached this peer in session 01.
* The level chases toward its target with speed 0.25 per client tick. The prior
  empty capture followed only 12 explicit peer waits before the screenshot's
  own 12 waits. A stable high-angle view should give the peer at least 40 waits
  after each state change. Do not use a server `data get block` reply as a client
  replica observation: it reads authoritative server state.
* Session 01 saved **Tank 1000 mB, Basin 0 mB, stopped Pump**. The actual actor
  inventory before and after the Creative Basin drain was exactly one
  `minecraft:bucket` in the selected hand, with no `water_bucket`. This follows
  `bucket_click`: Creative executes the Basin drain and leaves the held bucket
  unchanged. That previous 1000 mB was deleted by a Creative interaction, not
  held in a water bucket. The next supplement can use the existing empty bucket;
  no new bucket, fluid, or NBT fixture is needed.

## Proposed finite controls, session 02 in the same dedicated world

Use the frozen session runner and `call.py` control path already used for
sessions 00/01. Record each request/response in `calls-02.jsonl`, the three
peer PNGs, authoritative NBT/inventory snapshots, a result receipt, and normal
server/actor/peer exit receipt. Do not begin if the mirror/input hashes, prior
normal exits, or same saved-world path fail their existing checks.

1. Reopen the same dedicated world; wait ready for actor and peer. Before any
   mutation, verify the same Tank/Pump/Pipe/Basin blocks, Pump speed 0, Tank
   1000 water, Basin input/output `[0,0]`, empty Basin item slots, and actor
   inventory `{minecraft:bucket:1}` with that bucket held. Verify peer inventory
   empty. Record source-water census **1000 mB**. If any differs, stop without
   water seeding, item giving, or fixture repair.
2. Move only the peer to `24.3 66 21.3`; wait until positioned, look at
   `23.5 65.4 20.5`, and record returned yaw/pitch. The accepted relative pose
   was `(+1,+1)` at pitch 57.501; this world has a lower floor, so the slightly
   closer `(+0.8,+0.8)` pose should restore a steep view. Require actual pitch
   at least 45 degrees and a manually inspected, unobstructed Basin interior.
   After 40 peer waits, capture the high-angle **empty-before** PNG. Keep this
   peer pose and image dimensions fixed for both later captures. If the Basin
   top is occluded or clipped, correct only the peer camera before pumping.
3. Freeze authoritative server ticks, set the existing motor to
   `ScrollValue:16`, step 2 ticks, assert Pump speed magnitude 16, then step
   the already qualified bounded 150 ticks. Assert Tank 0/Basin 1000, exactly
   **1000 mB** total and no changed item slots. Set motor to `ScrollValue:0`,
   step 2 ticks, assert Pump speed 0 and unchanged tanks. Unfreeze, give the
   peer 40 waits, recheck Tank 0/Basin 1000, and capture **filled** PNG. No
   water `data merge`, new item, or source edit is permitted in this step.
4. With Pump still stopped, select the actor's *existing* plain empty bucket
   and use the accepted ordinary south-face Basin interaction once. Verify
   Tank 0/Basin 0, Pump speed 0, and actor inventory still `{bucket:1}` with
   no water bucket, as expected for Creative. Label the 1000 mB endpoint loss
   **Creative bucket drain**, outside the Pump custody claim. After 40 peer
   waits, recheck the empty Basin and capture **empty-after** PNG at the same
   pose. Do not claim water-item conservation for this Creative step.
5. Save and quit normally, verify all three exits 0, and compare the three
   images directly. Accept peer presentation only if a visible blue interior
   appears in filled and disappears in empty-after while both empty images
   remain empty. If the authoritative NBT is correct but the high-angle peer
   image fails, keep visual HOLD and investigate the actual replica/update
   payload or client renderer. Do not broaden the gameplay battery or alter
   source on this result alone.

Source pointers: `native/create/mixins/create/basin.rs` (`fluid_fill`,
`fluid_drain`, `commit_fluids`, `bucket_click`, `client_tick`, `draw_fluids`),
`native/create/mixins/create/runtime.rs::update_tag`, and pinned host
`native_hooks.rs::native_block_entity_sync`, `lan/hub.rs` block-log broadcaster,
`game/remote.rs` block entity merge. The accepted camera controls are in
`.local/native-s6-water-basin-01/peer-water-cold.json`.
