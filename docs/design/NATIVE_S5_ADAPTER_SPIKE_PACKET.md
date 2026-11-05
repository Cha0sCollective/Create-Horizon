# S5 — one-cell carrier feasibility spike

The owner authorized progression through S6. S4 is accepted/private-syncedb88eb47.
Read `NATIVE_S5_FEASIBILITY_AUDIT.md`; fresh independent Sol High review found a
bounded private adapter plausible and no demonstrated need for general host APIs.
This packet still needs review before implementation. S5 remains OPEN.

## Question and finite profile

Can one actuator-owned full-cube block reuse native machine saves/update tags and
Horizon's existing moving-piston collision/render/push primitives, while its
motion follows selected Java linear-actuator rules? Resolve this before new
actuator assets, glue graphs, general assembly entities or a public engine fork.

Use one inert `minecraft:stone` payload, eastward axisX, one-cell travel entirely
inside one chunk, Overworld, isolated mob-free Creative TestTerrain fixture. No inventory,
fluids, attachments, riders, mounted actions or cross-chunk behavior. Reject other
payloads/occupied paths. Reuse the current motor's real signed RPM controls as
the temporary test actuator/owner so the spike does not first build an unrelated
content pipeline. This is a feasibility probe, not a renamed motor advertised as
a ported mechanical piston or S5 completion. The proven carrier must subsequently
be bound to one actual Create actuator before S5 acceptance.

## Reference and ownership

Retain Java6.0.11/7ff2760e. `KineticBlockEntity.convertToLinear` divides RPM by512.
`MechanicalPistonBlockEntity.getMovementSpeed` clamps to±0.49 blocks/tick, applies
facing/axis sign, then clamps to the remaining travel. Eastward facing reverses
the RPM sign: −64RPM gives+0.125 blocks/tick. On a nonstalled reversal the parent
linear actuator rounds offset to1/16, then stops the old movement. Read the exact
reference before selecting stop/disassembly semantics; do not invent a requirement
to freeze a fractional pose if the selected reference instead places its blocks.

The existing motor's mod block entity is the sole authoritative trial owner.
Store a namespaced finite compound inside that existing machine: explicit trial
phase, source/destination, captured block-state name/properties, offset/previous
offset, signed RPM and last authoritative tick. This is machine data using normal
native saves/update tags, not a new world format or general assembly schema.
Enable only the one explicitly configured disposable trial motor; leave ordinary
motors and vanilla pistons unchanged. Enable/configure through existing fixture
commands, but capture the actual world block without manufacturing payload NBT.

Capture must remove exactly one payload without loot and reserve both affected
cells. The motor owns that payload until it is restored once as a real world block
or, if the bounded removal path requires it, one real item. A visual/collision
carrier is derived state, never another item owner. Failed capture changes nothing.
Refuse disassembly into another block and retain the single captured owner.

## Private adapter boundaries

Prefer derived carrier shapes/render entries reconstructed from owner data rather
than adding independently saved/ticked vanilla moving-piston owners. Existing
`sync_moving_pistons` replaces the shape list; a guarded private tail injection can
append the one derived carrier after ordinary piston synchronization. If proxy
block states are needed for the collision scan, reserve/protect them and never
give them a second payload-owning vanilla record. Guard projection replacement,
mining and actuator removal explicitly. Do not let vanilla auto-finish the probe.

Keep collision and appearance consistent through stop/reverse. Existing pushing
requires positive progress delta: reverse by opposite direction and mapped anchor/
progress, not by passing a decreasing progress to that helper. Use the existing
world collision/push math and reference speed cap. Before accepting new progress,
detect blocked movement/blocked player escape; retain a safe position rather than
advance the solid through a body. Any newly required general safety guarantee is
an input boundary, not a license to rewrite physics.

Server owns capture, motion and restoration. Clients reconstruct only from the
owner's replicated state, with explicit correction and previous/current poses.
Do not let the remote vanilla ticker independently finish it. Reconstruct before
the first relevant collision after chunk receipt/rejoin/cold load. Update-tag
extraction must read the entity alone; no cross-thread world lookup. Normal save
retains the one captured owner/progress; do not rely on vanilla `progress_o` NBT.

## Focused deliverables and stopping rule

1. Reference-backed pure motion/reversal/carrier-mapping functions and meaningful
   focused checks, plus the finite owner transitions used by the adapter.
2. One private exact-pinned carrier adapter, with a distinct private build version
   in the existing metadata while retaining `create:` IDs. Preserve S4 binaries/
   resources/raw evidence before any build reuse; use a fresh source/binary-bound
   S5 run packet.
3. One combined affected game milestone: capture/accounting, RPM motion, stop/
   reversal, world obstruction, moving/paused player collision and blocked escape,
   safe removal/refused blocked restoration, peer rejoin and normal mid-motion
   cold save/reopen. Distinguish authoritative state from client shapes/render/UI.

Fresh independent Sol High review covers packet/source/driver and feasibility
result. Run focused format/compile/lint checks for changed code; reuse unchanged
S1–S4 evidence without replaying it. No generic trial framework or broad battery.
Keep failed probes and raw outputs. Do not claim the actual actuator/full S5 until
its remaining binding/reference outcome is separately supported.

Stop for a concrete need for broader collision providers, generalized assembly
ownership/persistence, a changed public host contract, unsupported safety guarantee
or unresolved product behavior. Present the exact failed seam and smallest
expansion for owner input. No merge/release/upstream contact, public publishing,
cleanup, reference/legacy changes or playable/Prism/Java changes are authorized.
