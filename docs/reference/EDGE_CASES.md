# P01 edge and interaction corpus

This extends the accepted initial six cases without changing production Create.
The current `s0-edges-1` adapter emits schema 2: schema 1's observations plus typed
fixture `context` and the millstone output handler's declared slot limit. The set
contains 15 fixtures and 1,730 scheduled samples. A capture is accepted only after
all game assertions pass and a second fresh capture compares equal.

Every scheduled callback samples before its action/assertion. Relative tick 10
therefore records the pre-action state; the first possible observation of that
action is tick 11. The phase is not asserted to be a world-tick boundary.

| Added fixture | Action and terminal proof |
| --- | --- |
| finite_stress, 120 samples | Down-facing hand crank, Y shaft/cogs, one/two/three empty millstones. Turn every five ticks through 95; add loads after 30/50, remove the third after 70. Check stress 128/256/384 against capacity 256, equality active, overload stopped with theoretical speed retained, recovery and crank expiry. Real configured capacity/impact must be 8/4 per RPM. |
| milling_interrupt_reverse, 340 | Drop at 10; remove the shaft after 40, restore after 80 with motor −16. Work/input must survive the pause; one gravel appears after resumed processing and input/entity are consumed once. |
| milling_full_output, 100 | At 10 fill all nine output slots with 64 gravel each and drop one cobblestone at 256 RPM. Record the pinned handler limit and actual input/result behavior. This is a reference observation, not a conservation guarantee. |
| milling_one_full_slot, 100 | Fill only output slot 0 with 64 gravel; other slots remain empty. Completion must leave 65 total gravel and consume the one input. |
| milling_input_changed, 100 | At 20 extract the active cobblestone to fixture-owned storage and insert one barrier, which has no milling recipe. No output may be manufactured; removed cobblestone and barrier remain accounted for. |
| milling_input_removed, 100 | Same extraction, without replacement. No output; retain the removed cobblestone. |
| retrieval_empty_hand, 40 | Unpowered mill, input 3 cobblestone/output 5 gravel seeded after 10. Invoke the real empty-hand block handler: first use returns output and preserves input, second use after 20 returns input. Player ends with 5 gravel and 3 cobblestone. |
| retrieval_overflow, 40 | Same interactions; all 36 main player slots hold 64 iron ingots. Count player items and dropped entities in this fixture. End with 2,304 iron unchanged, 5 gravel/3 cobblestone on the ground, empty machine and no duplication. |
| motor_value_settings, 60 | Invoke the existing motor value-settings handler: default +16; positive/negative row with magnitude zero become +1/−1; both signed 256 endpoints reach the connected shaft. |

Retrieval uses a distinct NeoForge FakePlayer identity per fixture. The actor is
not an online client and does not tick/pick up drops. An original stone floor
keeps overflow within the empty 16×8×16 fixture; the actor stands away from the
millstone so the thrown items cannot re-enter it. These cases establish the real
server handler/inventory path, not reach validation, packet admission or a physical
mouse gesture. Fixtures do not share players or invent an overflow algorithm.

## Physical motor interaction

The isolated development client used the pinned Create/NeoForge/JDK environment,
fresh creative superflat world, seed 0 and structures disabled. Its local root is
`../.local/s0/client-reference/gesture-1/`. The motor was placed at `(2,-60,2)`,
facing east, and viewed from its south control face with an empty hand.

The coordinator observed default 16 and the rendered “Generated Speed in RPM /
Click and hold to edit” hint. The computer-use API has no documented right-button
hold, so the owner performed that gesture manually and reported changing speed
and seeing rotation change. A read-only extraction of the saved chunk found the
`create:motor` at those coordinates with `ScrollValue = -256` and `Speed = -256`.
No command or NBT editor changed the motor setting. This is a human-assisted
integrated-client observation, separate from automated handler assertions.

The region SHA-256 was
`bf30d751785434d4a95fe7bda0aadc05f0104ec91b0be279dbff9100f132d864`.
The raw saved-machine observation is local at `gesture-1/saved-motor.json`; the
client log and provenance record identify the run. The client disappeared during
a WSL instance restart after its saved-world observation; no clean-shutdown pass
is inferred from that disappearance. No owner playable world was used.

## Calibration failures

The first expanded run (`59992791-8eac-4014-b75f-f4e661da4cd0`) passed ten cases
and failed an incorrect expectation that one 64-item output stack blocks the
millstone. Its trace instead showed 64 gravel in slot 0 and one in slot 1.
The pinned NeoForge handler reports an absolute slot limit of 99, while ordinary
gravel insertion respects its item stack size of 64. The corrected corpus separates
one-full-slot and all-nine-full cases; neither outcome is inferred from the old
failed assertion. A subsequent compile-only failure was an observer AABB constructor
mismatch, corrected to use Vec3 bounds. Neither failed attempt is accepted evidence.

## Remaining scope

Even a repeated pass does not prove native Create behavior. Lifecycle/restart,
cross-chunk accounting, competing players/entities, all supported orientations,
source arbitration, faulty saves and network admission remain later fixtures.
New item components require canonical observation before arbitrary stack parity
can be claimed. The current tiny corpus deliberately uses plain known items.
