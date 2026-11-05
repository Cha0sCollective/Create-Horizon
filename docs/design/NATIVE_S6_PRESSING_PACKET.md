# S6 — first additional processing family: press and depot

The owner authorized stage progression through S6 until the next requested break.
Native S1–S5 are complete on the private, reviewed branch
`codex/native-s5-mechanical-piston` at `9b8068372ae2819b75eae0bae9edf4705c30c701`.
This packet selects **one** additional processing family. It is not acceptance of
all S6 families or of the imported port's unqualified content.

## Reuse and reference

Keep the compatible native port, Horizon, Mixins, API, Rust and art pins recorded
in `docs/CURRENT_WORK.md`. Existing native code already registers
`create:mechanical_press`, `create:depot`, `create:iron_sheet`, their recipes,
kinetic press stress, a press cycle, depot held/output slots, ordinary interaction,
rendering, replication and save. Inspect these paths before patching; do not
replace them with a new processing framework. No engine change is planned.

The selected Java reference is Create 6.0.11 at `7ff2760e`:
`PressingBehaviour` uses a 240-unit cycle, speed-scaled steps, and applies at
120; `MechanicalPressBlockEntity` selects a pressing recipe and defaults to
one item per strike because bulk pressing is off; `BeltPressingCallbacks` keeps
the unpressed remainder held and sends results to the depot output buffer.
`DepotBehaviour` stores one held stack and eight output slots. The selected
recipe is one `#c:ingots/iron` to one `create:iron_sheet`. The checked-in
content uses that tag, but `native/create/tools/content.py` would regenerate
the selected recipe with a plain `minecraft:iron_ingot` ingredient. This
packet tests only plain vanilla iron ingots, a member of the reference tag;
it makes no general tag-ingredient parity claim. Do not regenerate content
until the generator and checked-in recipe are reconciled and reviewed.
Compare the local behavior with the pinned Java source. The earlier
experimental port reported a one-tick pressing mismatch; determine whether
it affects this selected reference path before making a parity claim.

## Bounded outcome

In disposable normal host-generated, mob-free Survival worlds, ordinarily
place a real press two blocks above a depot, connect it to existing kinetic
power, and give the depot a plain iron-ingot input through ordinary interaction
or existing vanilla automation. Fixture-provided Create blocks and iron are
allowed; record them. Check real `Speed`, stress and a visible moving head.
One ingot becomes exactly one sheet with no duplicate or lost stack. A two-item
input takes two strikes with the unprocessed ingot held between strikes and
the first sheet in the output buffer. A stopped press must retain that state;
resuming must produce the second sheet. A nonmatching item must remain
unprocessed. Ordinary pickup or existing vanilla inventory transfer must
recover the sheets. Compare cycle timing and material accounting to the pinned
reference, identifying any supported profile difference rather than silently
tolerating it.

Use one integrated session for the full interaction/processing sequence and one
dedicated session with two matching rendered clients for affected replication,
peer rejoin and normal cold save/reopen. In the dedicated session, stop after
the first of two strikes with exactly one ingot still held and one sheet in the
output buffer. Cold close and reopen that state, inspect both the server and
peer after rejoin, then restore power and recover exactly two sheets. Reuse
unchanged S1–S5 host and kinetic
evidence; only inspect press/depot-specific UI, replica and item state anew.
Retain raw actions and observations, including false starts. Normal save/reopen
is an ordinary host guarantee, not interrupted-crash atomicity.

## Limits and gate

This first family does not include belts, basins/compacting, sequenced assembly,
chanced recipes, bulk pressing, multi-input recipes, fluids, mounted inventories,
trains or general modpack/world compatibility. Do not widen the engine API,
save schema, architecture or acceptance guarantees under this packet. If a
concrete need reaches one of those boundaries, stop for owner input.

Review this packet independently before implementation. Prefer the unchanged S5
binary and existing game/control tools if no production source changes are
needed. For any changed source, use focused core checks, formatting, compile and
lint, then the single affected game milestone. Obtain a fresh independent
read-only Sol High review of substantive source/delta and final evidence.
No public engine push, PR, merge, release, upstream contact, cleanup, launcher
change or playable-save edit follows from this packet.
