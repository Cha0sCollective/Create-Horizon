# S5 — bind the carrier to a mechanical piston

**Reviewed bounded implementation packet; S5 remains OPEN.** A fresh
independent read-only Sol High review first found the normal piston's
nonsticky retraction and pole/head accounting gap, then cleared this revision.
The owner authorized work through S6 under the adopted native route. S4 is
complete at `b88eb47`. The private trial-motor
carrier on `codex/native-s5-feasibility` has provisional game evidence in
`docs/reference/NATIVE_S5_CARRIER_PROBE_RESULTS.md`. Its corrected source map is
`ac29436c602acdbe7bbb07d3f09194654dbccef47d88a4ea8222f6b6354304fb`.
The trial proves a possible adapter path; it is not the actual Create actuator.

## Selected Create behavior

Use Java Create 6.0.11 at `7ff2760e` for this finite profile. Its
`MechanicalPistonBlock` is a directional-axis kinetic block with `facing`,
`axis_along_first` and retracted/moving/extended state. The normal block has
ID `create:mechanical_piston`; one `create:piston_extension_pole` behind it
provides one unit of travel. The `MechanicalPistonBlockEntity` uses the
`LinearActuatorBlockEntity` speed and reversal math used by the pure carrier
core; its assembly and stop behavior still need binding. Java registers 4
stress impact per RPM. The piston has two
shaft ports along its rotation axis, perpendicular to its facing. A one-pole,
east-facing example with its shaft axis Z (`axis_along_first=false`) is the
initial supported fixture.

`PistonContraption.collectExtensions` also models a moving head/pole and can
search connected structures. In the selected fixture the piston base is at
`x`, one **initially placed** pole is at `x-1`, and one inert full-cube stone is
at `x+1`. Extension moves the stone to `x+2` and leaves the head at `x+1`;
the initially placed pole is represented by that head, not duplicated as an
extra item. The normal nonsticky piston does **not** pull the stone back:
retraction leaves stone at `x+2`, removes the extended head and restores the
pole at `x-1`. This packet does not claim general Java contraption
assembly, glue/chassis graphs, sticky retraction, inventories, other faces or
more poles. The head/pole unit must be visually and collision-accountably
present with exactly one pole item owner throughout capture, motion, settled
extension, retraction and removal. Its derived rendering must not create
another saved/ticked owner. The head has no separately supplied item; Java's
head drop maps back to the pole. No fabricated stone or pole payload NBT.

For the default `MOVE_PLACE` mode, zero speed while running schedules
disassembly at the reference grid offset; the trial motor's indefinite
fractional pause is **not** the actual piston stop behavior. A nonstalled
sign reversal rounds offset to 1/16 and stops the old contraption before the
new assembly direction is chosen. Qualify the selected one-pole outcome
against that reference; refuse other movement modes in this first profile.

## Bounded implementation

1. Register the three exact Create IDs needed by the finite normal-piston
   example: mechanical piston, extension pole and head. Use the existing
   content generator/imported Create art pipeline and preserve the local art
   JAR/source provenance. Register the piston as a native mod block entity;
   do not introduce a separate persistence system. Provide ordinary placement
   and a usable visual, collision and item representation for the supported
   states. Keep unsupported orientations/assemblies fail-closed rather than
   silently moving them.
2. Feed `Speed` from the existing kinetic network through real shaft connection
   and stress arithmetic. The shaft axis follows the selected Java facing/
   axis-alignment rule; a finite actuator kind with two axial shaft ports and
   impact4 is preferable to presenting the piston as a one-sided consumer.
   Check the actual sign: east-facing `-64 RPM` gives `+0.125` block/tick.
   The creative motor may supply test power, but it no longer owns trial
   carrier state or receives a fabricated `Speed` value.
3. Move the stone and one pole into the piston's finite namespaced machine
   state while each is absent from the world. Reuse the reviewed one-cell
   carrier projection, body push, update-tag, ordinary save, static native
   collision-shape sweep and failed-write retry. The corrected trial source
   already covers a fence extending into the swept cube. Preserve separate
   stone/pole accounting: restore exactly one real stone and one pole/head
   representation at the selected endpoint, or retain their captured owner.
   Do not claim interrupted-crash atomicity.
4. Bind the one-pole head/extension representation to the same piston and phase.
   At extension and retraction settle the world forms above; on blocked
   return or actuator removal retain ownership until both parts can be safely
   materialized or otherwise accounted for once. Do not pull the stone on
   nonsticky retraction. Do not register a vanilla moving-piston owner or let
   its 0.5/tick ticker advance the Create carrier.
   Keep unsupported block-entity payloads, fluids,
   attachments, other directions, cross-chunk movement, riders and mounted
   interactions refused in this first profile. An unexpectedly necessary
   general collision provider, assembly schema or host API change is an input
   boundary, not an implementation instruction.

## Single affected milestone and gate

Run focused core/format/compile/lint checks and one exact-pinned game milestone
with a real placed mechanical piston, one pole, a real powered shaft/motor and
one real stone. Show network RPM/stress, one-cell extension capture, moving
pose, reference zero-speed placement, full extension and nonsticky retraction
that leaves stone at `x+2`, obstruction and peer collision, safe blocked
removal, normal cold save/reopen while motion is active, peer rejoin and exact
stone/pole/head accounting. Include one ordinary
vanilla piston regression if source hooks change. Distinguish server queries
from rendered client state and real player actions. Reuse unchanged S1–S4
evidence and the carrier spike where its source/behavior is unchanged; do not
replay broad batteries.

Fresh independent Sol High review must clear substantive source/delta and the
bounded result. No public engine work, merge, release, upstream
contact, cleanup, launcher/playable-save edits or general S6 work follows from
this packet. S5 remains open until the real actuator milestone and review pass.
