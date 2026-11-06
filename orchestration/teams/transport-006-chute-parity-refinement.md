# Transport-006 parity refinement: lower chest timing and input count

Status: **read-only source refinement, no chute implementation or runtime authorization**.
This supplements the reviewed decision packet at Transport commit
`232d1643b16fb82f570856b2552b8dafbd12ca0a` without editing it. For the
owner decision, replace that packet's provisional **commit-at-0.5** and
**hardcoded one-item extraction** proposals with the narrower source-backed
route below. Its one-slot owner, saved `ItemPosition`, fixed Creative-only
route, retention, removal, asset budget, client evidence and approval gates
remain proposals. No owner has approved the new block or saved field.

## Fixed plain-chest output

For this profile, require the lower block to be `minecraft:chest` with
`type=single`; use `native_block` and `native_block_property` to verify it.
The pinned Horizon host gives a normal chest all slots through its upper face
and permits placement in them. Read the upward-facing slot snapshot with
`native_container_slots(dimension, below, Direction::Up)`. For a nonempty held
stack, a read-only capacity probe succeeds if any reachable slot is empty or
zero-count, or contains the same item **and components/damage** with count
strictly below the incoming stack's maximum. The `can_take` flag is unrelated
to insertion. A missing snapshot or a lower block outside this exact profile
means no known lower capacity; retain the held stack. The probe's maximum must
match the pinned host's `Containers::max_stack`: an integer
`minecraft:max_stack_size` component clamped to 1..99, otherwise registered
item maximum or 64, then at least 1. The pinned `Game::stack_max_size` plus a
minimum-1 clamp appears suitable, subject to source/compile verification;
Candidate007's `stack_maximum` has different malformed-component behavior and
must not be assumed equivalent.

Keep Java's no-fan downward timing: start a newly extracted stack at position
1, advance by the reference motion, and when the *next* position falls below
0.5, run the read-only capacity probe. Clamp at 0.5 if it fails. If it passes,
continue motion; only when the next position falls below 0 call the existing
committing `native_insert_item(below, Direction::Up, held)`. That call's
returned remainder is authoritative. Clear the chute slot only when all
items were accepted; otherwise keep the exact returned stack in the canonical
slot and preserve the current position for retry. Reprobe on later ticks. A
chest can change between snapshot and commit, so the probe never transfers
ownership. If commit accepts only part of the stack, the lower chest owns
that part and the chute owns precisely the remainder. This follows Java's
`ChuteBlockEntity` probe at `nextOffset < .5`, commit at `< 0`, and retained
remainder at the prior `ItemPosition`. It does not promise crash-atomic
transfer across two owners.

The host has **no generic insertion simulation API**. This local probe is
justified by the ordinary single chest's exposed slots and unconditional
placement rule. It must not silently apply to double chests, mod inventories,
furnaces, other sided containers or open air. Should actual source/compile or
focused parity checks invalidate the plain-chest equivalence, stop and report
the precise host/API gap before changing the packet or timing.

## Input amount and qualification

Java's normal chute asks for **up to 16** matching items when empty. Its
`ItemHelper.extract` also caps each source-slot request by that slot stack's
`getMaxStackSize()`. The existing `native_extract_item` accepts a count and
otherwise caps only by source count. From the first eligible occupied slot
of the upper chest, request `min(16, source stack maximum)` using the
same component-aware maximum rule as the pinned host. Store the returned
count and components unchanged in the canonical chute slot. The one-gold
route remains the first finite gameplay trial, **not** an extraction limit.
Java's `ItemHelper.extract` may
combine matching items across several source slots; the first native profile
still reads one slot per pull. Name this remaining parity limit in the owner
decision rather than implying full Java extraction parity. No extraction is
attempted while the chute holds any remainder.

Before claiming 16-count or partial-output behavior, check empty lower chest,
matching partly filled stack, completely full chest, component maximum
override, probe/commit capacity change, and partially accepted remainder.
Check input16, an overstacked component-limited input slot, and the
single-slot/multislot distinction; then cold blocked restore,
one-spill removal, and client-local occupied/empty evidence under the original
packet's gates. The first integrated and dedicated gameplay route may still
use gold1; it cannot alone qualify stack16 or changing-capacity cases.

## Exact source basis and boundary

- Create 6.0.10 and 6.0.11 local `ChuteBlockEntity.java:170-192,345-385`
  and `ItemHelper.java:183-221`: probe/commit timing, up-to extraction,
  per-source-slot maximum and retained remainder; the two
  snapshots' chute sources were byte-identical in the reviewed packet.
- Pinned Horizon `9c7f9b2215dfed497aba5522c44f38926628648d`:
  `machines/native_items.rs:73-171` (snapshot, count extraction, committing
  insertion/remainder), `machines.rs:86-111,162-172,294-355,607-639`
  (normal chest slot/placement and component-aware insertion), and
  `native_hooks.rs:8951-8961` (no insertion simulation flag). The exact
  pinned source was fetched read-only; a local Horizon checkout was at a
  different revision and was not treated as the pin.
- Frozen Candidate007 remains detached at
  `f732bd481cfc3d9c4720bc51e4a31e7385e18020`; this document does not
  change its source, host/Mixins pins, cache, assets or world.

This is a revised decision recommendation, not a build, test, acceptance or
implementation result. Operations still needs the owner's approval for the
new block-local item owner and saved position, selected narrow behavior and
remaining Java differences. No public release, host API change or pin change
is proposed.
