# Native S6 pressing family — PASS within the declared scope

This is the first bounded S6 additional-processing family from the independently
reviewed `docs/design/NATIVE_S6_PRESSING_PACKET.md`. Fresh independent
read-only Sol High completion review was CLEAR. It qualifies ordinary
mechanical pressing of plain vanilla iron ingots on one depot. It does not
complete S6 or qualify Create's other processing families.

## Reuse and reference

No production code, engine API, recipe file or save format changed. The existing
native port already supplied press/depot registration, recipe lookup, 240-unit
speed-scaled cycle, one-item strikes, eight-slot depot output, model/head
rendering, network integration, interaction and normal save. The real-game runs
used the accepted S5 input map SHA-256
`58adc2a9f85ce9870828dafa630e51d20f5a9ec14f50e00b969efe18fedef860`,
game binary `1d0cbf80a950e70eba34c7db7fd527576eded2f4ea1d76a1cc7ed099cc8833d6`
and ctl `507b23e1a8f68a3f11da9c677fd2ec2bf52f93e73d7ebfe2affcc9f7a7863500`.
Each launcher checked exact source/resource and binary hashes. Unchanged S5
compile, format, lint and core checks are reused; no rebuild or broad test
battery was needed.

Pinned Java Create 6.0.11 `7ff2760e` has the selected
`#c:ingots/iron` -> `create:iron_sheet` recipe. The checked-in native recipe
uses Horizon's `#tag` ingredient syntax with the same tag and result, while
`native/create/tools/content.py` would regenerate a vanilla-only ingredient.
The packet limits the tested input to plain `minecraft:iron_ingot`, and
requires generator reconciliation before regeneration or a tag-parity claim.
Java `PressingBehaviour` uses `CYCLE=240`, applies at 120 and scales tick speed
with RPM; `MechanicalPressBlockEntity` defaults `bulkPressing=false` and its
depot callback retains unprocessed input and buffers output. The native core
uses those cycle/one-item rules. This game run did not establish exact
Java-vs-Rust event ordering at the one-tick boundary reported by the earlier
experimental port; no tick-exact parity claim is made.

## Integrated Survival observation

Disposable normal seed-5 generated world `integrated-b3btkqgy`, with the
`spawn_mobs=false` rule and a leveled stone plot. A fixture supplied one
creative motor and shaft, positioned with native commands, plus a depot,
press, iron and cobblestone to the Survival player. The player ordinarily
placed the depot and press. The first press-aim attempt failed because the
temporary support top face was not visible; moving the player onto a small
temporary platform succeeded in the same world. All fixture blocks were
isolated from user worlds. Mob spawning was disabled after startup; the
initial world had mobs, which were killed. No empty-population claim follows.

One ingot was inserted by ordinary depot interaction. At `-64 RPM` the press
reported `512 SU` stress against `1,048,576 SU` capacity and converted it to
one held sheet, with no buffered duplicate. A powered cobblestone swap left
the cobblestone unprocessed and put the sheet in the player's inventory.
After clearing those test items, the actor inserted two iron ingots. At
`16 RPM` motor setting (negative press speed), one strike left exactly one
iron ingot held and one sheet in output slot 0. Stopping power held
`Ticks=146` and those counts over 25 more ticks. The paused rendered press
head was inspected. Resuming power converted the remaining ingot; ordinary
depot interaction emptied it and gave the actor exactly two iron sheets.
The integrated process exited 0. Raw commands, response log, screenshot and
exit receipt are in `.local/native-s6-press-01`.

## Dedicated cold reopen and peer

Disposable normal seed-5 Survival world `dedicated-8z2xcots` used an isolated
loopback offline server, two whitelisted matching rendered clients and the
same fixture/ordinary-placement boundary. `spawn_mobs=false` was set; initial
entities were removed before the relevant processing. The actor inserted two
ingots by ordinary depot interaction. After the first powered strike, the
motor was set to zero: authoritative depot data showed one ingot held and
one sheet in output slot 0; press `Ticks=148`, `Applied=1`, `Running=1`,
`Speed=0`. A further 25 ticks preserved the state. The peer saw both block
replicas and the lowered press head, and its own render trace recorded the
held iron ingot.

Both clients and the server closed normally, each with exit code 0. A cold
launch reopened the same server world and both clients. Server data retained
one ingot held, one sheet buffered, the press at `Ticks=148`, and motor
`ScrollValue=0`. The peer again saw press/depot block replicas and its own
render trace began with the held ingot. After power resumed, server data
showed one sheet held and one sheet buffered. The peer's ordinary depot
interaction reported `create:iron_sheet: +2`; its inventory contained exactly
two sheets, the actor had none, and a final server query showed the depot
empty. The peer's client render trace progressed from ingot to sheet to
empty. Both cold-run clients and the server also exited 0. Raw commands,
client logs, screenshots and both three-process exit receipts are in
`.local/native-s6-press-dedicated-01`.

## Limits

The observations cover this plain-iron/depot path, ordinary clean restart and
the existing matched client pair. They do not cover exact Java tick phase,
general tag inputs, belts, basin compacting, bulk/chanced/multi-input or
sequenced recipes, mounted transfer, fluids, trains, imported Java worlds,
interrupted-crash atomicity or release readiness. Screen images and client
render traces demonstrate replica/presentation; item counts and machine NBT
were confirmed by authoritative server queries and ordinary inventories.
