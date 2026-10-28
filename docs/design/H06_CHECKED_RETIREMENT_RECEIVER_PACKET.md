# H06c checked resident retirement receiver

Accepted bounded G0 implementation packet following fresh independent read-only
Sol High design review with no blocking findings. H06b and H08a13 are qualified;
base engine dc12bd60aeaee33f09b18d9e8ffee4d763a52355 passes native, both-mode
full-counter, broad and final receipt checks. No Game enabling callsite here.
This is actual Game item-vector ownership transfer, not another detached kernel.

## Exact source and fixture scope

Add paired-packages-gated game/slot_retirement.rs and its module declaration in
game.rs. Add only SlotProjection::into_stack(self) -> Option<ItemStack> in the client
library slot_transfer.rs; it moves the original private value without normalization.
World's existing snapshot/take APIs suffice; no new mutable projection visibility,
World retirement mutation, counter change, dependency, WIT, disk or network API.

Tests can be a child integrated/transition_ownership_tests/slot_retirement_tests.rs
with one gated declaration in transition_ownership_tests.rs after a13 is frozen.
The child reuses that private Fixture::game/server_game/explicit Options/resource
loader unchanged; no production visibility widening or renderer constructor.
Its ignored tests run only in the reviewed isolated actual-Game test process with
pinned HORIZON_TEST_RESOURCES, sanitized session state and fresh cwd/homes/temp.
Do not alter the four qualified transition tests or silently inherit account paths.

## Concrete receiver and source authority

Propose private Game::deliver_pending_world_slot(expected: &SlotKey) returning a
small typed Result<Delivery, DeliveryError>. Delivery distinguishes EmptyAcknowledged
and Delivered { entity_id }; finite errors identify no pending, wrong key/context,
non-authoritative execution, nonresident source, unsupported stack, ID exhaustion
and reservation failure. No control command, timer, callback or automatic caller.

Reject remote.is_some() or integrated.is_some(), like authoritative counter dispatch.
ExecutionRole::Client alone is not refusal: in-frame is a client execution role.
Inspect self.world.world_slot_snapshot() without taking the owned retirement.
Require its current pending key exactly equals expected, its key scope equals its
captured configuration scope, and that dimension equals self.dim.id(), the active
level selector. Position comes only from that current captured configuration.
This includes opaque trial allocation identity/incarnation, not just printed numbers.
levels::swap_level exchanges World/dropped/SaveDir, then enter_level changes dim;
it does NOT swap Level metadata. level.dimension is not this authority check and
may legitimately differ during integrated parked-level work.

There is no intrinsic Game/World SourceScope field to compare. mutation_control's
observation_scope() allocates a fresh source instance per call; NEVER call it during
delivery or fabricate a replacement scope. The native activation context supplies
expected; current World pending-key equality anchors the allocation being delivered.
That selector is not authentication, player permission or a saved world generation.
Later activation must establish captured source binding; this receiver does not.

Require actual self.world.chunks membership for the ORIGINAL position's column.
Do not use World::block's unloaded-as-air behavior or stale allocation.resident as
current residency evidence. A pending allocation may have retired on reload mismatch
and still have resident=false while its column is now resident. Conversely it may
have retired resident=true then unloaded. No force-load, relocation or dormant-region
entity write is allowed. Do not require the block still equal the old binding:
ordinary removal/change is precisely why this retirement exists.

## Canonical stack and native representation policy

None is an empty acknowledgment after the same context/residency checks: consume
the empty retirement once, with no entity ID, reservation, RNG draw or vector entry.
Some(count=0) is NOT empty: refuse and retain its native components/selection metadata.
For positive delivery require exact non-air registered item ID via res.items().get;
do not normalize unknown names or reconstruct the stack from an ID/count pair.
No registry max-stack or universal 64/99 cap is invented. Oversized positive inputs
within the supported representation move intact into one entity, never clamp/split.

Native ItemEntity.stack.count is u32, but item_nbt::encode_item writes Tag::Int via
count as i32; the reader turns a negative count into one. Check count in
1..=i32::MAX for the root and exactly the four modeled native child collections:
container, charged, bundle_contents and present pot_decorations stacks. Refuse any
unrepresentable count while keeping World pending ownership. Use a small borrowed
walk before consume, with checked temporary-worklist reservation; no serialization,
component reconstruction, opaque-tag interpretation or newly invented depth policy.
This closes that concrete known count boundary, not full component-codec validation.
All modeled/opaque components, damage and runtime selections still move unchanged.
Do not claim arbitrary component/NBT round-trip conformance from this receiver.
Some(count=0) is unreachable through current empty admission/positive-transfer paths;
test consuming projection extraction on that form and review the defensive Game
guard without adding a visibility-breaking test injector to manufacture it in World.

## Preflight, consume once, then infallible publication

Keep exclusive &mut Game throughout; no callback, IO, tick or level swap can interleave.
Before taking pending: finish all context/stack/residency checks; compute checked
next_entity + 1; self.dropped.try_reserve(1); clone the current particle LegacyRandom
and draw the ordinary spawn_item bob offset on that clone; construct scalar output
and entity fields not requiring the owned stack. Every refusal leaves pending's
actual key/projection/raw metadata, existing entity entries, next_entity and RNG intact.
Successful reservation may change capacity; that is not an item ownership change.

For this diagnostic sink use the original block centre [x+.5,y+.5,z+.5], zero
velocity and pickup_delay=10; match spawn_item's health5/fire0/no thrower or target,
previous=position/on_ground=false/age0/ticks0 and staged bob field. This is not a
claim of Block.popResource positioning/motion or Create removal behavior. Do not
call pop_resource/pending.attach, which can change metadata, or unchecked spawn_item.

Then take_world_slot_retirement exactly once. None at this point may return NoPending
because it consumed no owner; exclusive access makes that defensive case unreachable
after a successful peek. After a successful take, destructure the owned allocation,
consume its SlotProjection::into_stack, and move that ORIGINAL stack into ItemEntity.
The already-validated Some branch is invariant-backed, not another fallible decision.
Publish checked next_entity, staged particle RNG and dropped.push into reserved space;
return the preconstructed success value. No fallible operation, clone of the item
payload, notification, serialization or callback follows successful ownership take.
Do not reconcile/readmit automatically; receiver success only clears pending.

## Meaningful acceptance and limits

Seed actual World trial through its existing native transfer APIs, retire with real
World::set, then invoke this actual Game method. Use original captured test metadata
and explicit native binding in the fixture; this tests receiver ownership, not new
package/block admission. Scope comes from that actual Game's observation_scope().
Require exactly one new self.dropped entry, unchanged damage/modelled/opaque fields,
actual top-level and nested BundleSelected integers, count and correct source position.
Check original pending removed, caller-owned projections conserved, ID increments once,
and a second delivery cannot append another entity. Exercise None acknowledgment,
positive oversized count, unknown/air IDs and count above i32::MAX retained unchanged.
Exercise unrepresentable zero/overflow counts in each modeled nested collection,
including a deeper child, and prove pending ownership/raw selections unchanged.

Test wrong opaque key (including same numbers/new allocation), mismatched scope
dimension versus Game.dim, remote refusal, ID=u32::MAX and source-column removal:
each keeps actual pending ownership, entity entries, ID and RNG. Reinsert the original
column through the authoritative seam and prove successful retry exactly once.
Compare cloned RNG draws in assertions without mutating the production source on
refusal. Actual try_reserve(1) allocation failure has no deterministic resource-safe
fixture here: source-review its ordering; do not mock an allocator or exhaust memory.
Use actual enter_level swaps to show pending ownership stays with its parked World,
delivery selects its matching active dim, and unchanged level.dimension cannot refuse it.

Before ANY Game slot enablement, separately qualify every active/parked World loss,
dimension replacement, idle close, exit/error path and retained refusal receiver.
H08a13 protects synchronous departure producers; it does not preserve this pending
owner automatically, fix shutdown errors, or establish coherent checkpoints. Normal
entity saving selects resident coordinates: that is why this sink refuses unloaded
sources. Future live player borrowing/resolved limits/replay/cursor/menu authority
remain separate, as do guest slot proposals, drops policy and persistence. No owner
decision is needed for this bounded dormant receiver; broader lifetime scope returns
to a reviewed packet. Stop/rescope rather than add enabling or general recovery here.
