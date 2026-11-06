# Machinery 004 — R2 fixture-only launch correction

This is a proposed **new-run control delta**, not a gameplay PASS or a replay of
the held item. Preserve the original [runtime packet](MACHINERY_004_RUNTIME_PACKET.md),
[HOLD](MACHINERY_004_RUNTIME_HOLD.md), driver SHA-256
`5242e0834a6472b5b45b7cc77010b17806382adef5c98c8f14c5acd6c7c997cf`,
and receipts. The sole first item in `integrated-ed08kkz7` stopped at X=9.125,
outside that controller's X=8.25..8.75 selection window. No valid contact was
tested, so the adapter remains unqualified.

## Exact R2 delta

- In the setup control, add one `minecraft:smooth_stone` throwing pad at
  `(X+1,Y,Z)`. The original support and machine coordinates, two selected
  X-axis wheels, controller, clutch, output column, and all block assertions
  remain. The earlier `(X+2,Y,Z)` support stays as an inert fixture block.
- For each **new** ordinary one-cobblestone input, teleport the actor to
  `(X+1.08,Y+1,Z+0.5)` on the new pad and look at the unchanged controller top target
  `(X+0.5,Y+0.8125,Z+0.5)`. The actor's reported `state` must confirm the
  position and eye within 0.001 block, westward yaw and calculated downward
  pitch within 0.02 degree, and a ray hit on the controller's top. Validate
  this before `give`, then again after
  `select` and before the only `drop`. A failed check stops before input or
  drop; retain its receipt as HOLD. Record the state including `on_ground`.
- Give exactly one cobblestone and use the pinned host's ordinary `drop` once.
  Keep every existing intake, clock, stopped-refusal, cold save/reopen, output,
  removal, actor, and dedicated-peer assertion and image case. Do not move,
  replace or re-drop the old cobblestone or write item position, velocity,
  age, controller inventory, timing, or the selected contact window.

The pinned host at `9c7f9b2215dfed497aba5522c44f38926628648d` supplies
the trajectory estimate. `crates/horizon-client/src/game.rs` `throw_from_eyes`
starts the item at eye minus 0.3 Y and applies view velocity 0.3, upward 0.1,
random horizontal strength at most 0.02 and vertical jitter at most 0.1.
`crates/horizon-client/src/entities.rs` gives the 0.25-wide item gravity 0.04
per tick before movement and 0.98 air drag. The selected adapter in
`native/create/mixins/create/crushing.rs` (combined007 map) requires item center
X/Z in `+0.25..+0.75`, a just-landed item at Y+0.8125, and a valid controller.
For the new X+1.08 launch, a conservative free-flight calculation using the
full independent horizontal ±0.02 and vertical ±0.1 bounds plus the allowed
pose tolerances crosses that top plane in 5..7 ticks at approximately
X+0.340..+0.740 and Z+0.365..+0.635. Those rounded bounds
fall inside the existing selection window. The calculation predicts the
fixture; only an actual freshly spawned item and server contact/custody
receipts can establish it. Actor placement and block collision can still
invalidate the prediction, so the driver stops on an actual miss.

## Root-controlled execution after a fresh game reservation

Use the copied driver
`.local/machinery-004/wheel004_milestone_r2.py`, SHA-256
`0fceca7ea9a848c03ee939eb2a5bb7cfbc725fd9c9bd3bc1e3542b12f3b3f184`.
It keeps the original source/tree/map and game/control binary identity guards.
Root creates a fresh isolated world and session with the existing
`.local/native-team-runtime-007/session.py`; there is no game or build lease
in this preparation. Run each case with:

```text
python3 '/mnt/g/Create Rusty/.local/teams/machinery/.local/machinery-004/wheel004_milestone_r2.py' SESSION_JSON '/home/arbiter/.cache/create-horizon/team-integration-007-bin-fb8ec9b36084/horizon-ctl' CASE --origin X Y Z
```

Use `setup`, `pause` on index 00; close normally, cold reopen the **same** world
as index 01, then run `cold`, `resume`, `removal`. Repeat separately in an
integrated and a dedicated/two-client fresh world. The new runner writes
separate receipts by run name and refuses an occupied receipt path. Its
`passed_control_steps` is only finite control completion; root must reconcile
actual server, actor and peer evidence and normal exit receipts before any
gameplay PASS. Stop at a concrete adapter/host gap or another failed criterion.
