# FLUIDS-001 frozen-candidate runtime plan

This is a fixture plan for the project lead's future **exact frozen combined
revision, map, and binaries**. It is independent of the team source commit.
The project lead owns the shared build, game lease, and execution. Record those
identifiers and the actual commands/results before making any runtime claim.

## Existing fixture and observation hooks

- Use a disposable isolated tank position with air on every side. Place one
  `create:fluid_tank` through ordinary Creative player placement so its block
  entity is initialized; confirm `bottom=true`, `top=true`, `shape=window`,
  `Window:1b`, `Size:1`, `Height:1`, and `TankContent:{}`. Do not treat `setblock`
  alone as proof of a valid controller.
- The existing agent/control calls support `block` for the loaded block and
  state, `inventory` for held-item accounting, `look-at`/`interact` for an
  ordinary player click, and `screenshot` for a client image. The established
  `call('command', 'data get block X Y Z')` pattern in
  `native/create/tools/engineering_checks.py` reads authoritative server NBT;
  retain its returned chat text. `block` observes the client replica and does
  not expose NBT. Run the same visual observation from the peer client.
- A deliberately seeded partial-content fixture, if needed, is an explicit
  setup mutation: `data merge block X Y Z
  {TankContent:{Fluid:{id:"minecraft:water",amount:7500}}}`. Verify the full
  resulting controller NBT with `data get block` before acting. Keep setup
  mutations distinct from the read-only observations above.
- No audio observation is required. Use server NBT before and after cold
  reopen, block state, inventory, and peer screenshots as evidence.

## Capability gate: direct calls required

The pinned host's `Game::native_fluid_view`, `native_fluid_fill_stack`, and
`native_fluid_drain_stack` are game-internal calls. The existing control/agent
socket does not expose them. The pinned `HORIZON_NATIVE_FLUID_FIXTURE` exercises
synthetic `fixture:tank` blocks, **not** `create:fluid_tank`. Its passing result
cannot certify this adapter. Before claiming this gate, the project lead must
identify an existing narrow test entrypoint that reaches the actual Create
adapter in the frozen candidate, or separately authorize a bounded test-only
fixture. No general API or harness is proposed here.

At that entrypoint, record each return value and controller NBT immediately
before and after the call, without another tick/action between a Simulate call
and its state check:

| Starting water | Direct request | Simulate / NBT | Execute / NBT |
| ---: | --- | --- | --- |
| 0 | fill plain water 9,000 mB | accept 8,000 / still 0 | accept 8,000 / 8,000 |
| 7,500 | fill plain water 1,000 mB | accept 500 / still 7,500 | accept 500 / 8,000 |
| 8,000 | fill plain water 1,000 mB | accept 0 / still 8,000 | accept 0 / still 8,000 |
| 750 | drain Any 1,000 mB | water 750 / still 750 | water 750 / empty `TankContent:{}` |
| 750 | drain Exact plain water 1,000 mB | water 750 / still 750 | water 750 / empty `TankContent:{}` |

Also check a lava fill, a water fill with nonempty components, and an Exact
lava drain: each is refused without NBT change. Recreate the stated starting
state before each row. If no actual adapter entrypoint is available, report
this gate as **pending**. Creative bucket clicks require a full 1,000 mB
exchange and cannot establish the 500 mB clipping rows.

## In-game and persistence gate

1. On the valid isolated controller, use an ordinary Creative water bucket
   click. Confirm exactly 1,000 mB in server `TankContent`, a visible water
   surface on the peer, and the Creative held bucket unchanged. Use an empty
   bucket click; confirm explicit `TankContent:{}`, no peer water surface after
   its update, and the held bucket unchanged. A failed full-bucket exchange
   at 7,500 mB should leave NBT and inventory unchanged.
2. Refill to 1,000 mB. Use a normal wrench click and verify `Window:0b`,
   `shape=plain`, and unchanged 1,000 mB; toggle back and verify the inverse.
   Save, exit, and cold reopen the same disposable world. Compare authoritative
   `TankContent`, `Window`, `Size`, `Height`, block state, and peer water view
   before/after. Drain to empty, save/reopen again, and confirm the peer remains
   empty. Preserve both server NBT reads and peer images.
3. Re-run the accepted Basin water boundary check on the frozen combined
   candidate (`create_port_basin::water_tests::canonical_water_roundtrip_and_strict_capability_boundary`),
   plus one ordinary Basin water bucket fill/drain in the disposable world if
   the lease permits. Keep tank and Basin coordinates separate. Compare Basin
   NBT and item accounting to its accepted behavior.

Filled tank wrench removal is source-inferred to yield a plain tank item and
lose stored water; it has no Java runtime trace. Record any observed result as
exploratory, not a parity gate. The ordinary Survival bucket path is likewise
pass-through by source; this packet's required player exchange is Creative.
