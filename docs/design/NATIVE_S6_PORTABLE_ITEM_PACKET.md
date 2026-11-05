# S6 — one portable item-interface connection

Implementation requires independent read-only Sol High packet review CLEAR.
The preceding audit is `NATIVE_S6_PORTABLE_ITEM_AUDIT.md`; the accepted baseline
is `a77af67`. This is one finite S6 family increment, not S6 completion.

## Playable profile

Normal generated mob-free Survival Overworld, integrated and dedicated/two
matching rendered clients. Retain an east-facing normal mechanical piston,
Z shaft and one rear pole. At base-relative X+1 place one depot holding one
plain `minecraft:iron_ingot`, all processing outputs empty; at X+2 place one
empty, unpowered east-facing `create:portable_storage_interface`. X+3 is
initially clear. At X+4 place the opposing west-facing stationary interface,
with a vanilla hopper below it leading to a chest. All assembly cells are
loaded in one chunk. Unsupported cargo, waterlogging, output items, active
interface payloads and unrelated block entities refuse capture unchanged.

Piston power captures and moves both cargo blocks one cell using the existing
private proxy and clearance services. One stationary connection exposes the
same mounted item to vanilla hopper extraction. Show actual moving projection
and connection presentation. The hopper/chest receive exactly one ingot;
the piston slot becomes empty. On safe completion/stop the depot restores
empty. Normal nonsticky return leaves the two restored cargo blocks forward.
If extraction is refused, the sole ingot stays mounted and restores to the
depot. Ordinary player placement and collection are required; fixture motor,
shaft, plain items and isolated terrain preparation are allowed and recorded.

## Representation and transfer boundary

Add an explicit finite two-block cargo variant to the private piston owner.
Keep existing stone and single-depot saved states unchanged. The new variant
uses exactly one canonical `ModBlockEntity.items` slot on the piston; no
`DepotItem` copy is saved for it. An empty slot after extraction is a valid
mounted state, distinct from invalid/missing owner metadata. The stationary
interface owns no items. Piston container rules expose no direct automation
or player menu. Refuse unexpected piston slots/items before starting a profile.

One private HEAD adapter at `Game::suck_from_container` handles only the
stationary interface above the selected vanilla hopper. This entry has mutable
Game before its ordinary view lookup, so it can call `native_neighbor_signal`
and validate current power, owner/direction, loaded geometry and connection
phase/timer immediately before extraction. Construct a stack-local logical
`View` whose `parts` refers to the canonical piston slot and delegate to the
existing `suck_from_slots(None, &view, hopper)`. Preserve the native hopper
inventory-search observation with the stationary block position. A selected
PSI refusal returns `Some(false)` so it cannot fall through to item-entity
pickup; non-PSI inputs continue through the original function unchanged.

The logical view allows output slot0 through the bottom only and no insertion.
All other interface inventory callers remain closed: no global
`Containers::view` alias, stationary `ModContainer`, eligibility cache or saved
ready marker. Use the stock `get`, `slot_mut_at`, take/put-back, cooldown and
`view_changed` path. A private change observer on the actual owner resets the
transfer timeout and syncs presentation. Do not copy items into a stationary
buffer or reimplement hopper movement.

Keep authoritative connection/timer and mounted slot on the piston owner;
stationary animation/link data is derived. A stationary block cannot reopen
access using only its saved link. A temporary absence of the ticking owner
must refuse access. Derive all client poses and reconnect state from checked
owner data. The current host update tag copies NBT, not canonical item slots:
add an explicit wire-only compound derived from slot0 on every owner update
tag, with a presence flag and item when occupied and an explicit false/empty
snapshot after extraction. Never insert that snapshot into server-saved NBT.
Client rendering reads this replica rather than assuming `items[0]` arrived.
Require initial peer join, extraction-empty update and peer reconnect to show
the matching occupied/empty moving item state.
Normal save/reopen must preserve exactly one item across the
owner/depot/destination inventories. No cross-owner crash atomicity is claimed.

Use the pinned reference's east-facing1.85 active-area encounter rule. The
moving block center starts at X+2.5, so its active point starts at X+4.35,
already inside the stationary cell; first admission is distinct from the
closest crossing near stroke offset0.15. Use this explicit owner-tick order:

1. Validate the loaded owner/physical profile, current motor direction and
   stationary match/power. A stopped/reversed motor or invalid link closes
   access before settle/retry. Decrement an existing positive keep-alive once;
   expiration clears the link, sets timer3 and skips further timer decrement
   for this tick. Otherwise clamp the existing timer to68,
   and decrement it once when greater than4, or positive with no keep-alive.
   An expired/powered link is closed. Do not decrement a timer newly admitted
   later in this tick.
2. Advance only when not stalled by the prior tick, using the existing safe
   motion planner. Process the actor at the resulting checked offset. On first
   encounter or a new active-point grid cell, find the opposing unpowered
   stationary interface at the active cell or the next cell east. A new valid
   link starts timer14. Recheck the working stationary block every actor tick
   and renew keep-alive2. If no valid encounter remains, let the existing
   keep-alive expire; closed or separated links expose no inventory.
3. Apply the reference look-ahead test: stall when the current active point's
   distance to the stationary center is less than the distance from
   `active_point + last_motion` to that center. This selects the nearest
   discrete step near offset0.15 rather than requiring an exact center hit.
   Release that stall when timer<=4. A retained
   valid link at timer4 may remain connected while motion resumes, as in the
   reference, until encounter loss/keep-alive expiration; timeout is not itself
   a second item owner or an empty-depot restoration.
4. The hopper-source view is eligible only for a live reciprocal profile/link with inclusive
   connected timer4..64 and an unpowered matching stationary block. It reads
   the last fully published owner state; the owner temporarily out of the
   world yields no view. The host retains its existing block-entity ticker
   list order: a hopper may run before or after the owner. The mutable source
   adapter revalidates live geometry/power before every selected hopper lookup;
   no stale eligibility flag survives the function call. Close access on separation.
   Do not require a current-tick stamp that would starve a hopper placed first.
   A successful actual slot extraction resets timer64 immediately in the
   owner change callback; the next owner tick applies the normal decrement.

Animation uses4 and default timeout60. Test both hopper-before-owner and
owner-before-hopper query order for identical item accounting, while qualifying
the list-order timing rather than claiming unobserved tick-exact Java parity.
Retain item custody on every refusal. This vanilla-hopper profile does not
qualify external inventory-event position parity: host callbacks identify the
canonical piston base from `view.parts`, rather than the stationary interface.

Capture, rollback, blocked restoration, base/proxy removal guards, collision
sweep and client projection must include depot and moving interface. An empty
restored depot is valid only after the canonical owner slot is known empty.
A failed write cannot fabricate successful restoration or release an item.
Preserve original interface states/metadata allowed by this finite profile.

Register the original interface ID and six-way facing, ordinary placement,
item/loot and readable local-JAR model/connection rendering using the
existing content/resource path. Keep provenance and standalone fallback art
accurate. This packet supplies interface items as fixtures and does not qualify
their Survival crafting progression. Do not add a general actor/assembly/capability framework, fluid
interface, arbitrary inventories, filters, bidirectional transfer or riders.
The reference interface inherits only FACING from WrenchableDirectionalBlock;
do not invent a waterlogged interface property. Depot waterlogging remains refused.
Stop for owner input if a broader host/API/schema/architecture change is needed.

## Focused gate

Review the exact source delta before live use. Test finite geometry/connection
admission and timer transitions, alias resolution to the sole owner slot,
empty-after-extraction restoration, malformed state/refusal and partial
capture/restore retry. Format/compile/lint the affected source. Reuse unchanged
S5/S6 evidence with one stone or depot-only regression for touched branches.

Use one isolated integrated session and one dedicated/two-client milestone.
Demonstrate ordinary placement, powered capture, mid-motion connection/stall,
one vanilla-hopper/chest extraction, timeout/resume, safe empty settle,
ordinary collection, blocked hopper destination retaining the ingot, wrong-facing or
powered refusal and one static obstruction or body collision. Query all
relevant owner/inventory/drop counts. For the blocked case, prefill all hopper
slots and its output chest with incompatible full stacks so the hopper cannot
receive the ingot or make room by forwarding; a full chest alone is insufficient.
Assert owner ingot1, hopper ingot0 and all filler/chest counts unchanged.
Distinguish server data from peer
replicas/rendering. Cold reopen normally while connected with an unextracted
ingot; verify saved owner, reconnect without a duplicate, then complete and
collect exactly one. Record fixture/ordinary actions, failed attempts and
all process exits in a fresh disposable packet. No historical broad battery.

Fresh independent read-only Sol High source/delta and completion reviews must
clear the source and evidence before this profile is accepted. A failed finite
route is a design/input boundary, not permission for an engine rewrite. No
public engine patch, PR, merge, release, cleanup or playable-save/launcher edit.
