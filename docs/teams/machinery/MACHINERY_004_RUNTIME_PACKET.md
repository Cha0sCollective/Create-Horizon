# Machinery 004 — source-only wheel runtime packet

**READY FOR ROOT'S RESERVED GAME LEASE; NOT RUN.** This is the finite runtime
qualification of the owner-approved one-item Crushing Wheels outcome. Root
supplies the compiled map, binaries, disposable worlds and session sockets.
Machinery has not built, started or driven a game. Preserve every failed receipt.

## Bound inputs and authority

- Run the integrated actor and the dedicated server with actor plus **actual
  peer** from the same root-frozen source map and binary hashes. Pin the
  combined source at `f71992cb9fce067f96dfa736a318147929846538`
  (integration of reviewed `f05d8ccfb1e164d865eb923f7156c68546780fd2`).
  Record root's compiled map/binary hashes, save path, mode, socket roles and
  current source HEAD in each receipt before acting. Do not substitute this
  team's older checkout or a different cache.
- The reference host is Horizon `9c7f9b2215dfed497aba5522c44f38926628648d`;
  Java Create is `7ff2760e4924d35d4bc2e22c4b19923f0178cd16`.
  Pinned host `game/commands/misc.rs` implements `tick freeze` and
  `tick step N`; `game.rs` and `game/native_hooks.rs` state that players still
  tick while frozen. A frozen **world** step advances the item/controller;
  actor commands and ordinary interactions may occur between steps. Confirm
  `time query gametime` is stable between steps and advances by exactly `N`
  after `tick step N`, as in the existing host counter smoke.
- Use the established `horizon-ctl --socket <role-socket>` interface and the
  existing session/receipt runner. `command`, `block`, read-only `container`,
  `inventory`, `entities`, `tp`, `look-at`, `interact`, `drop`, `break`, `save`,
  `quit`, `screenshot` and `wait-ready` are pinned host controls. For dedicated
  teleport, use `command "tp @s ..."` and wait for the actor/peer position,
  following the Drill 003 control. No new framework is needed.

## One fixed fixture

Choose an isolated loaded center `(X,Y,Z)` in a mob-free disposable Overworld.
Keep a 12-block item census radius around it free of preexisting drops. Use
fixture setup only for support, power, a lever and two wheels; the controller
must be created by the wheel-pair lifecycle. The center column `(X,Y-1,Z)` is
**air** with floor `(X,Y-2,Z)` so downward output can exist and be picked up.
All relative coordinates below are added to `(X,Y,Z)`:

| Relative position | Fixture block / purpose |
| --- | --- |
| `(0,0,-1)` | `create:crushing_wheel[axis=x]`, first wheel |
| `(0,0,1)` | `create:crushing_wheel[axis=x]`, second wheel |
| `(0,0,0)` | Initially air; auto-created `create:crushing_wheel_controller[facing=down,valid=true]` |
| `(-3,0,-1)` | `create:creative_motor[facing=east]`, default `ScrollValue:16` |
| `(-2,0,-1)` | `create:clutch[axis=x]`, initially unpowered |
| `(-1,0,-1)` | `create:shaft[axis=x]` |
| `(-2,1,-1)` | `minecraft:lever[face=floor,facing=north,powered=false]`, on the clutch |
| `(1,0,1)` | `create:creative_motor[facing=west]`, default `ScrollValue:16` |
| `(2,0,0)` | Solid actor throwing platform, with actor feet at `(X+2.5,Y+1,Z+0.5)` |
| `(0,-2,0)` | Solid output floor; keep center `(0,-1,0)` air |

Add supports below motors and wheels as needed without filling the center
output column. The east motor should drive first wheel at effective `+16` RPM;
the west motor should drive second at effective `-16` RPM. **Assert both wheel
`Speed` values from server block data**, their `NetworkStatus: "Running"`,
their stress/capacity (expected 128 SU load per isolated 16 RPM wheel) and
the controller's `facing=down,valid=true`; motor settings alone are
insufficient. With lever on, the clutch splits its X halves,
first wheel speed becomes zero and controller `valid=false`, while second
wheel remains negative. If actual signs, RPM or controller state differ, stop
and report the fixture gap; do not reorient the selected pair.

Use `horizon-ctl ... block X Y Z` for each state, and `command "data get block
X Y Z"` for server NBT. The existing Drill 003 `tellraw` begin/end markers
around each query avoid interleaved chat in a two-client run. Query the
controller's read-only `container X Y Z` where supported; this reads the
host's actual slots in the queried process. In a dedicated client, label that
result a client replica unless root has a server control socket. Never write
its inventory,
`ProcessingTime`, `RecipeTime`, `AppliedRecipe`, `Speed`, bridge NBT or a world
item directly. The only one-item input is actor `drop`; the output collection
is actor pickup. Actor must be Survival with exactly one cobblestone for each
case. Record a before/after census: actor inventory, controller's 32 slots
where directly observable, authoritative `Inventory.Items` NBT, and every
cobblestone/gravel world item in the bounded plot. For dedicated authority,
run the read-only server command `execute positioned X Y Z as
@e[type=minecraft:item,distance=..12] run data get entity @s` between unique
chat markers; its per-entity feedback supplies server item NBT. The actor's
`entities 12 item` result and peer view are client replicas. Retain the
server item UUIDs, positions and counts, with client IDs separately. If
direct server slots are unavailable in dedicated mode, say so; use the saved
NBT, removal spill and item census to support
custody without claiming a direct canonical-slot query.
The server query covers a **local radius** centered on the fixture, not a
global absence claim. `entities 12 item` is centered on the actor; record
the actor position for that replica query. Keep the plot clear so a
zero-result server command is meaningful.

## Normal cold cycle — repeat once per mode

1. With pair running and empty controller, take server state and actual actor
   screenshots of **both wheels and the gap**; add actual peer frames in
   dedicated mode. Assert 32 empty slots,
   no plot cobblestone/gravel entity, and empty output floor. Freeze the world:
   `command "tick freeze"`. Confirm stable game time before the drop.
2. `command "give @s minecraft:cobblestone 1"`; select cobblestone, stand
   on the east throwing platform and look at the center top. Use ordinary
   `drop` once. Confirm actor count falls by one and one cobblestone world
   item appears. Step **one world tick at a time** (`command "tick step 1"`,
   then wait until `time query gametime` advances once) until controller slot
   0 first holds exactly one cobblestone. Allow up to 40 flight/contact steps
   as an operational diagnostic ceiling, since the actor's throw velocity
   varies; report a fixture gap if it misses. This ceiling is unrelated to
   the twelve valid **controller** clock ticks. The item must leave the world
   census on that same capture; `Inventory.Size=32`, `ProcessingTime=250`, `RecipeTime=250`,
   `AppliedRecipe=0`, and no other slot occupied. Save a server NBT/container
   receipt and actor **input** views (plus dedicated peer). Do not save/reopen
   before capture:
   the contact ID is transient and mid-contact cold recovery is deferred.
3. Step three valid clock ticks while frozen. Because capture itself does not
   decrement the clock, the expected partial `ProcessingTime` is `190`
   (250 minus 3×20), slot 0 still holds cobblestone and output is zero. If
   it already advanced during capture, stop and record the actual sequence.
4. Actor ordinarily clicks the lever at `(-2,1,-1)` via `interact`. Step one
   world tick at a time until clutch/redstone and the controller settle.
   Assert lever and clutch powered,
   first effective RPM zero, second negative, controller `valid=false` and
   `Speed=0`. Record the clock at this settled stopped point. Step three
   more world ticks. **This is the one selected stopped/invalid refusal:**
   the canonical item, clock, applied flag and zero output remain unchanged.
   If the controller takes one final valid decrement while the clutch
   updates, count it among the twelve valid ticks; do not report it as
   stopped processing.
   Do not add an unsupported-input or alternate-axis refusal.
5. Save normally while stopped. For integrated, use the session's `save` then
   `quit`; for dedicated, server `save-all` and its normal shutdown. Retain
   the save/exit receipts. Restart the **same** world with the **same** map and
   binaries. Before touching the lever, assert canonical slot 0, its mirror,
   the saved partial clock/flag, stopped wheel and no output. Startup may run
   unfrozen ticks; the powered clutch must hold the clock throughout. Capture
   actor **cold input** frames and actual dedicated peer frames, not a server render substitute.
6. Freeze again, prove stable game time, then actor clicks the lever off.
   Step one tick at a time to settle and verify both effective signs and
   `valid=true`. Count **valid controller ticks**, including any decrement
   during either lever transition, from the capture receipt. Each pre-apply
   valid tick subtracts 20; the twelfth applies the recipe: slot 0 becomes
   empty, slot 1 holds exactly one gravel, `ProcessingTime=0`,
   `RecipeTime=0`, `AppliedRecipe=1`, with no world gravel yet. Check after
   each step if needed; do not silently accept a different boundary. The
   **next** valid tick ejects one gravel below the controller, clears all 32
   slots and clock, and leaves no cobblestone. Step only enough for the
   dropped item to settle; record server state and actor **output** frames,
   plus dedicated peer frames.
7. Actor moves to the gravel item's observed position and picks it up by
   ordinary collision. If the frozen world prevents settlement/pickup,
   unfreeze only after the single ejection, then collect. Assert the actor
   gains exactly one gravel, no plot gravel/cobblestone remains, and the
   controller stays empty with no second output. Record a final server census
   and actor empty/output views, plus dedicated peer views.

## Separate occupied removal — repeat once per mode

After the completed cycle, use the same fixed geometry with an empty
controller and isolate this case's item counts from the actor's earlier gravel.
Freeze, give/drop **one new** cobblestone ordinarily, and single-step to
confirmed slot-0 capture. Record one canonical cobblestone, its NBT mirror,
zero world cobblestone/gravel and `AppliedRecipe=0`. Do not advance to Apply.
Give the actor one `minecraft:iron_pickaxe`, select it, stand within reach,
and use pinned `break X Y Z`
against the **second wheel** `(0,0,1)` while the world is frozen. Then step
one world tick for the level reconciliation. If frozen player breaking cannot
finish, hold the clutch stopped first, use ordinary `break` while world ticks,
and record that contingency; never alter controller NBT. The expected result
is wheel air, controller air, **one** spilled cobblestone world item from
canonical custody, no gravel and no later processing. The broken wheel's
own item loot is a separate ID/count, not part of the cobblestone census.
Record the break receipt, server block/item state and actual actor spill
views, plus dedicated peer views. Collect the cobblestone ordinarily; assert
one new actor cobblestone
and no duplicate plot item after several further world ticks. A failed
break, missing spill, extra spill or output is a failed outcome, not a reason
to edit inventory or replay silently.

## Evidence and stop conditions

For each mode keep one receipt sequence with source/binary/save identity,
command results, exact `game_time` changes, controller `block`/`container`/
NBT at every custody boundary, world-entity census, actor inventory and
actual actor PNGs (plus dedicated peer PNGs). Tag observations `server`,
`actor` or `peer`.
Integrated and dedicated/two-client cold reopen are distinct claims; neither
is established by compilation or a client-only screenshot. If a pinned
control is unavailable in the supplied binaries, record its help/error and
return the concrete gap to root. No public release, production/schema/pin
change, Cargo, Java, WSL or game run is part of this source-only preparation.
