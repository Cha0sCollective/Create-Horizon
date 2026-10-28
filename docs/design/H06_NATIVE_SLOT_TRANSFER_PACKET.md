# H06a native two-slot transfer core

Fresh independent read-only Sol High design review finds no blocking issue;
the coordinator accepts this bounded implementation packet. Base engine
`d2816f6d7f03cec0ab326ec9ad6f1b5cd4a0ed40`; H05d normal/fault/native games and
final independent receipt reviews pass. This
unit investigates in-memory conservation under G0, not completed H06/H07 or G1.

## Exact source and ownership scope

Propose one small client library module, src/slot_transfer.rs, its focused native
tests in that file, and one paired-packages-gated export in src/lib.rs. Reuse
crate::items::ItemStack and std types; no dependencies, renderer, resources, Game
constructor, WIT, Wasmtime or items-crate behavior changes. It models exactly one
detached machine-slot projection and one detached player-slot projection. These
are owned native values, not a live inventory adapter or authentication capability.

SlotProjection privately owns Option<ItemStack>, a checked u64 revision and an
opaque allocation identity (e.g. private Arc<()>, compared with Arc::ptr_eq rather
than Arc's value equality). No Clone or mutable stack
access is exported. snapshot clones the complete stack and retains identity plus
revision; a checked replace operation is the only native input-update seam. A
native-unequal replacement increments revision before publication. A native-equal
replacement retains the original value entirely and does not advance revision:
native equality deliberately ignores runtime bundle selection, so it must not
publish the supplied replacement on that path. Revision exhaustion returns Err
with all state retained. A new projection
has a different identity even if its contents/revision match an old one. Do not
expose addresses as durable keys or invent a global player inventory revision.

## Minimal proposed API

Use a concrete Direction { IntoMachine, IntoPlayer }, ResolvedLimits { machine,
player }, immutable SlotSnapshot, opaque non-Clone PreparedTransfer, and explicit
NoMove/TransferError/CommittedTransfer results. Names may change in design review.
prepare(&machine, &player, direction, requested, limits) returns either a no-move
reason or a prepared positive transfer. Reject aliased projection identity. There
is no slot index list, arbitrary entity operation, callback or factory-wide batch.

The consumed plan captures both allocation identities, revisions, complete raw
stack values, direction, requested amount and resolved limits; privately owns both
proposed after-values; exposes only read-only outcome getters. commit(self,
&mut machine, &mut player, fresh_limits) validates both full read sets and limits,
checks both next revisions/arithmetic, then installs both after-values together.
No fallible operation or notification follows the first published slot update.
Dropping the plan aborts with no mutation; no rollback of a partially applied plan
is necessary. A stale or invalid completion changes neither slot nor revision.

Duplicate prepared plans may exist, but the first positive commit invalidates the
second through revisions. The consumed public plan cannot itself be submitted
twice. A -> B -> A through checked replace is stale even though full values match.
Reconstructing an identical projection is stale through allocation identity. These
are protections for this encapsulated model ONLY. A future adapter must maintain
real mutation ownership/revisions or exclusively borrow actual owners throughout
evaluation; constructing fresh revision-zero copies of Guest.inventory does not
establish live ABA/replay protection. Network session/sequence deduplication is not
implemented or implied by this plan's single-use lifetime.

## Canonical stacks, capacity and existing unusual inputs

Use ItemStack::same_item_same_components for merge compatibility: item, damage
and native StackData equality, including unknown/opaque components. Use complete
ItemStack PartialEq for read-set validation, including count. Clone native values;
do not serialize/rebuild stacks from item ID/count or tidy component boxes as an
incidental normalization. Preserve components on split/remainder and destination
metadata on compatible merge. None versus an all-default component box follows
the existing native predicate; do not invent additional equivalence rules.
Native equality excludes BundleSelected's actual i32. Preserve cloned selection
fields and test them directly, including nested stacks; native equality alone is
not evidence of literal raw metadata equality. Encapsulation and unchanged
revisions ensure no ignored field changes between prepare and commit.

Limits are authoritative native inputs, never guest output. ResolvedLimits rejects
zero; it does not impose a newly invented universal 64/99 registry ceiling. Future
Game code must resolve fresh limits from the actual stack, registry and slot rules:
game/commands/items.rs::stack_max_size uses an integer max_stack_size component
clamped to 1..99, otherwise registry max_stack (existing fallback 64). That method
is evidence for later resolution, not something this module duplicates or proves.
Changing either resolved limit invalidates an outstanding plan. No registry/air/
unknown-item or player-permission validation is claimed by accepting native values.

Follow menu.rs::split/safe_insert/tidy: move min(requested, source count, remaining
destination capacity); compatible full/oversized destinations accept zero, using
non-underflowing capacity arithmetic. An oversized source may be drained gradually;
do not reject, clamp or discard its retained remainder merely because it exceeds
the current limit. Never create or worsen an oversized destination. Every positive
destination result respects its supplied limit; an untouched pre-existing oversized
destination remains unchanged. Use checked arithmetic, not wrapping sums.

None and Some(stack with count 0) have no transferable source items. A present
zero-count destination still uses the native merge predicate; do not silently
erase an incompatible identity. Zero requested amount, empty source, incompatible
destination or no capacity returns explicit no-move without changing representations
or revisions. On a positive transfer, depleted source becomes None; a compatible
zero-count destination keeps its metadata through ordinary merge. No-op,
stale and abort paths must not silently tidy away even zero-count source metadata.
This is transfer accounting for a generic test slot, not Create recipe backpressure.

## Meaningful resource-free evidence

Use actual native ItemStack/StackData/NBT components and real encapsulated projection
mutations, not mocked closures or an alternative reference implementation. Check
exact fit, partial capacity, partial source/request, full destination, mismatched
item/damage/components, component-rich splits and opaque metadata retention. Exercise
supplied limits 1/64/99, changed limits and rejected zero limits; this proves capacity
use, not Game's component/registry resolver. Include Some(count=0), both None,
oversized source/destination and requested/count u32::MAX without overflow or loss.

Compare before/after totals with widened arithmetic for each canonical stack
identity, not only item name. Require exact unchanged snapshots on no-move, abort,
stale/duplicate, identity replacement, A/B/A and revision-exhaustion failures. A
successful positive commit changes both slots and revisions once; test both
directions. No live player inventory, entity, world, file or guest is touched.

## Explicit later boundaries and stop conditions

Do not attach a nonempty slot to today's Copy counter Record or overwriteable
diagnostic last_retirement. Live ownership needs separate reviewed removal/teardown,
player/session authority, reach/hand validation, replay deduplication, menu/cursor,
overflow/drop ownership and real callback fault tests. H08 coherent saving remains
separate. This kernel must not enable transfers or advertise inventory APIs in Game.
Stop/rescope if it needs live hooks, a generic transaction system, new dependencies
or persistence. Coordinator owns serialized validation after source review. No
public push, stable SDK promise, acceptance expansion or new owner policy is needed.
