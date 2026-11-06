# Machinery 004 — R3 saved-spill pickup continuation

**Prepared only.** R2's fresh integrated `integrated-5on914mw` setup, one-item
intake/pause, normal cold reopen and gravel resume finished their finite
controls. R2's separate occupied removal spilled exactly one cobblestone but
failed ordinary actor pickup. Preserve the [R2 plan](MACHINERY_004_RUNTIME_PACKET_R2.md),
original [HOLD](MACHINERY_004_RUNTIME_HOLD.md), both R2 session exits and all
raw receipts/images. No dedicated Wheel004 case or R3 runtime has run.

## Diagnosis and bounded source comparison

R2 `pickup(COBBLE)` used the host ctl `tp` at the initial spill position
`(8.745299322127803,80.17177955847347,8.375311494096817)` while frozen.
The pinned host `game/control.rs` `tp` writes the local player position; it
does not itself issue the existing server `tp @s` command. This pose places
the player width-0.6 body partly inside the new `(9,80,8)` throwing pad.
After the frozen wait, `tick unfreeze` and another wait, actor inventory still
had no cobblestone. The bounded server census found the same cobblestone at
`(8.875,79,8.125)`, on ground, delay 0, while actor status was back at
`(11.5,79,11.5)`, the spill-camera position. The server log contains two
`moved wrongly` warnings earlier in the removal sequence, but neither is a
click-time pickup trace; do not assign either warning as the proven cause.

The pinned `game.rs` item tick decrements delay, moves the item and checks
player overlap and `pickup_delay==0`. The post-unfreeze item aged and settled,
so unadvanced item physics is not the final explanation. Actor convergence
and the colliding teleport pose are concrete risks. The immediate frozen
wait alone cannot establish item pickup opportunity. R3 uses the existing
server command, a clear floor pose, observed actor/server convergence and
bounded world steps; it changes no native source or item/controller state.

Read-only decoding of the **closed** saved integrated world scanned all 24
entity chunks in its four Overworld entity regions. Its complete canonical
cobblestone/gravel census contains exactly one item:

| Item | Count | Saved position | UUID NBT | Age | Delay | Grounded |
| --- | ---: | --- | --- | ---: | ---: | --- |
| `minecraft:cobblestone` | 1 | `(8.875,79,8.125)` | `[I; 1146242168, -1976155571, -1729057143, -1798240893]` | 1581 | 0 | yes |

The saved TeamActor is at `(11.5,79,11.5)` with one gravel and one iron
pickaxe, no cobblestone. The final live read-only block census had controller
`(8,80,8)` and broken second wheel `(8,80,9)` air. The first R2 spill
receipt and saved item share the exact UUID. A saved item age below the host's
6000-tick expiry gives a bounded chance to collect after a prompt reopen;
the R3 control stops if the item is missing, altered or already expired.

## Exact new-copy control delta

The ignored local driver `.local/machinery-004/wheel004_milestone_r3.py`,
SHA-256 `ee46f75355470698775803f8348fbfb2693289190d211d8fe7d8ab2b7a505b49`,
is a copy of R2 SHA-256
`0fceca7ea9a848c03ee939eb2a5bb7cfbc725fd9c9bd3bc1e3542b12f3b3f184`.
R2 setup, selected X-axis intake/contact, clock, clutch, save/reopen, Apply,
Eject, gravel pickup and images remain. Only the **spilled cobblestone** pickup
at the end of `removal` uses the new method. The method asserts controller
and broken wheel air, existing output floor and pad, one world cobblestone,
zero world gravel, pickup delay 0 and age below 6000. If the new spill is
airborne, it advances at most 24 frozen world steps until grounded on the
output floor. It does not write item position, velocity, age or inventory.
Pinned `horizon-world/src/level/removal.rs` starts a container spill within
Y..Y+0.75 at vertical speed `0.2±0.11485`; host gravity and drag predict an
unobstructed floor crossing by about step 20. The extra four steps cover
tick/collision ordering. A spill that lands elsewhere remains HOLD.

The safe player feet pose is `(X+0.5,Y-1,Z+0.5)`, the center of the existing
output floor. At `(8.5,79,8.5)` its player box clears the new pad at X=9 and
the remaining first wheel at Z=7; the saved item is within the pinned host's
player/item overlap radius. R3 uses ordinary `command "tp @s ..."`, then
checks both server `data get entity @s Pos` and actor `status` position before
waiting for ordinary pickup. At most six single frozen world steps follow if
needed, then one bounded 20-tick unfreeze fallback. Missing convergence,
missing/extra item, wrong count or any other failure stays HOLD. It never
re-drops or replaces material.

The additional `recover` case is restricted to the **same** integrated run
`integrated-5on914mw`, origin `(8,80,8)`, session index 02, zero prior exit
and R2 failed-removal receipt. It requires the exact saved spill UUID, actor
gravel 1/pickaxe 1/cobblestone 0, then performs only the ordinary pickup and
five empty-state world steps. One actual actor collected/empty image and
normal save follow success. It does not replay setup, capture, pause, cold,
Apply, Eject or wheel removal. If the saved world differs, stop and report.

## Root-controlled use after a new game reservation

Root's reviewed normal007 helper can reopen the **same** saved run with
`--run '/home/arbiter/.cache/create-horizon/team-runtime-007/integrated-5on914mw'`
after confirming index-01 exit code 0. It will issue session index 02.
Use the Linux run-path session JSON, as the driver requires:

```text
python3 '/mnt/g/Create Rusty/.local/teams/machinery/.local/machinery-004/wheel004_milestone_r3.py' '/home/arbiter/.cache/create-horizon/team-runtime-007/integrated-5on914mw/session-02.json' '/home/arbiter/.cache/create-horizon/team-integration-007-bin-fb8ec9b36084/horizon-ctl' recover --origin 8 80 8
```

For a later **fresh dedicated/two-client** world under a separate root game
grant, use the R3 copy for the original `setup`, `pause`, normal cold reopen,
`cold`, `resume`, `removal` order. The dedicated run has no `recover` case.
Root reconciles actual server, actor and peer evidence and normal exits before
any gameplay PASS. A R3 code receipt alone is finite control completion.
No Cargo, Java, game, source, schema, API, pin or acceptance change belongs to
this preparation.
