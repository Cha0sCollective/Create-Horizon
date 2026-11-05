# S6 — one mounted depot held stack

The S6 stage authorizes a bounded mounted-storage increment. The reviewed
feasibility audit is `NATIVE_S6_MOUNTED_TRANSFER_AUDIT.md`. S5 normal-piston
and the first S6 pressing family remain accepted only within their stated
profiles. This packet adds one second cargo kind to the same one-pole piston;
it does not implement a portable storage interface or a general contraption.

## Selected reference and profile

Pinned Java Create 6.0.11 `7ff2760e` mounts a depot's held item as one
`DepotMountedStorage` slot and restores it to a depot on unmount. Its stationary
eight-slot processing output buffer is not in that mounted storage. Keep the
selected S5 piston geometry: east-facing base, Z shaft, one initially placed
pole, one-cell extension in a single loaded chunk, normal nonsticky return.
Replace the inert stone at `x+1` with a normal `create:depot` containing one
plain `minecraft:iron_ingot` in its held slot. Require every output slot empty,
no incoming/other item owner, no waterlogging and no unsupported block-entity
payload. Refuse any other inventory block or occupied-output depot before
capture, with original block and item intact. The depot at `x+2` after
extension is not pulled back on normal retraction.

The item has exactly one owner at every observed phase: the stationary depot
before capture, the piston owner while the depot is absent/moving, and the
restored depot after placement. No duplicate drop or fabricated empty depot
may appear. Preserve its item identity/count and ordinary client presentation.
The pole/head accounting and existing stone case continue to follow S5.

## Implementation boundary

Use the existing private piston owner, saved namespaced NBT, static collision
planner, proxy projection, save and update-tag path. Extend it with an explicit
finite cargo discriminator and one validated depot item payload; preserve
readability of the already accepted S5 stone state. Avoid a generic moving
block-entity schema or new Horizon API. Before capture, validate the actual
depot block entity and all item slots. Replacing its world block may remove
the block entity, so record the item in the piston owner and verify block
replacement and rollback carefully; do not assume vanilla container-drop
behavior. If depot-to-proxy succeeds but a later capture write fails,
rollback must leave exactly one item in the world depot or piston owner,
with no duplicate drop. On restoration, create the real depot block entity
with the held item once, sync it, then release the piston owner's item claim
only after the result is observable. A failed depot materialization or item
installation retains that claim for retry; it must not count an empty depot
as restored. Guard base/proxy mining while either
the depot item or pole is still owner-held. Do not let an empty default
depot entity overwrite the saved item on normal reopen.

Do not capture a depot with processing output or a concurrent player/hopper
action. If the existing private owner and host callbacks cannot make the
single-item transfer safe within ordinary save/reopen, stop for a new reviewed
design or owner input before broad host/schema/architecture changes. No
interrupted-crash atomicity is claimed.

## One affected milestone and gate

Review this packet independently before code. Use focused source tests for
the cargo discriminator, item encode/decode, refusal and ownership transition,
including partial capture after depot-to-proxy and failed depot
materialization/item installation with exact single-owner/no-drop assertions;
then format/compile/lint. Reuse S5 stone/collision evidence where code and
behavior are unchanged, with one focused stone regression for modified
branches. Use one isolated normal-terrain Survival integrated session and
one dedicated server/two matching rendered clients. Fixture motor/shaft and
plain ingot are allowed, but ordinary player depot/piston/pole placement and
depot item insertion/collection must be distinguished from fixtures. Show
power/stress, one extension, moving client projection, normal settle with
exact held item, nonsticky retraction and occupied-output refusal. Query
dropped item entities around capture and settle, and exercise one static
obstruction or moving-depot body collision; the depot projection is new. Cold
reopen a captured depot during motion with item custody, then settle and
collect the single ingot through ordinary interaction. Check both
authoritative item data and peer render/update state. All test worlds are
disposable; record raw failed attempts and process exits. No historical full
game battery, public engine work, PR, merge, release, upstream contact,
cleanup, launcher change or playable-save edit follows from this packet.

Fresh independent read-only Sol High source/delta and completion reviews
must clear the changed behavior and evidence before acceptance. This single
mounted-depot step leaves portable interface item transfer for a separate
reviewed packet and does not complete S6.
