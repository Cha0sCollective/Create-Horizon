# H06b World-owned slot lifetime

Accepted bounded G0 implementation packet following fresh independent read-only
Sol High design review with no blocking findings. Base engine
0fd5e1ee2ef43cb8fd52835f0f282f24920bb77a passes focused tests, feature lint and
broad quick checks. This is not completed H06/H07.
No live Game enablement, guest slot ABI, persistence, item drops or G1 claim.

## Concrete source scope and relationship to the counter

Add client src/world_slot_trial.rs, a paired-packages-gated lib.rs export and the
small World field/method/hook changes in world.rs, with focused native tests.
Tests may use the module's world_slot_trial/tests.rs child to keep review practical.
No dependencies, counter_trial.rs change, new generic lifecycle service or Game edit.
World owns Option<Box<SlotTrial>>, disabled by default. SlotTrial privately owns
its configuration, one optional active allocation and one optional pending retirement.
Reuse the existing immutable counter_trial::Configuration as the captured type,
scope, position and exact state binding; do not require an enabled counter trial.
Its backend metadata does not select or invoke a callback in this unit.

This is a separate companion ownership experiment, not inventory in Copy Record.
Counter Trial::retire currently replaces Copy last_retirement; Game's receiver logs
and drops that snapshot. Neither is an item owner. Coupling slot lifetime to those
diagnostics would lose items or change qualified H05 behavior. The new trial's
allocation is independent; it does not claim counter/slot joint commit or equality
of their instance numbers. A later unified machine adapter must define that join.

## Owned values, keys and bounded retention

OwnedSlot contains private move-only SlotProjection, SlotKey and resident flag.
Snapshots clone SlotSnapshot for inspection only; they are not additional owners.
SlotKey is opaque and Clone, holding captured AuthorityScope, a private Arc trial
allocation identity compared with Arc::ptr_eq, and checked local u64 incarnation.
Expose scope/incarnation getters, not a public key constructor or pointer address.
Re-enabling creates a new trial identity; incarnation never wraps or resets within
that trial. Equal coordinates in different scopes, or a new trial with equal scope,
cannot accept an old key/plan. No saved generation or authentication is claimed.

An admission creates an EMPTY SlotProjection. It never accepts an owned ItemStack
argument that could be dropped on validation failure. Tests fill through the native
transfer kernel with a caller-owned detached player projection. No mutable machine
projection reference escapes World; inspection returns immutable snapshots only.
Incarnation exhaustion latches a finite admission fault without losing owned data.

PendingRetirement is non-Clone and owns the entire displaced OwnedSlot, plus reason
and game_time. The invariant is at most one occupied allocation across active and
pending: retirement moves active to pending and clears active. While pending exists,
all admissions and transfers refuse; repeated writes never replace that pending
value, its original reason/time, key, projection revision or raw stack metadata.
Ordinary block writes still commit normally, including further remove/place cycles.
Do not use a bounded event queue, a cloned stack, or a diagnostic last-retirement
field as an ownership substitute. Empty allocations retire the same way as nonempty.

## Minimal World APIs and synchronous hook behavior

enable_world_slot_trial(Configuration) validates already-enabled, non-air binding
and actual resident storage as enable_counter_trial does; matching storage admits
empty, resident nonmatching storage leaves no allocation. Return a read-only view.
world_slot_snapshot/read_world_slot(key) expose configuration, active snapshot,
pending metadata/snapshot and finite fault; stale or disabled reads are explicit.
No helper treats an unloaded coordinate as air or loads/generates a chunk.

prepare_world_slot_transfer(key, &player_projection, direction, amount, limits)
requires no pending/fault, matching current key, resident flag AND actual resident
storage with the exact configured state. It wraps H06a's consumed PreparedTransfer
with the scoped allocation key. commit_world_slot_transfer(plan,
&mut player_projection, fresh_limits) repeats those World checks before delegating
to the kernel; all checks/result construction precede either slot publication.
No take-machine/put-machine temporary gap and no callback, closure or fallible IO.
Removal/replacement changes the old allocation, so the old transfer cannot commit
into a new one even when block, count and position have returned to equal values.
The caller's player projection remains detached; this is not a live inventory API.

World::set currently commits resident state before counter.changed (world.rs:535).
Add the independent slot hook at that synchronous committed-write seam. A changed
configured position moves active to pending; matching state may admit only when
neither active nor pending exists and admission has not faulted. Same-state no-op
writes leave identity, contents and revision intact. Slot bookkeeping cannot reject
or undo the already committed block write; its fault only inhibits its own service.

World::remove_streamed_chunk (452) marks active nonresident, retaining key, revision
and contents. It is suspension, not retirement. World::insert_streamed_chunk (421)
already distinguishes resident replacement from absent-column arrival. Resident
replacement retires the old allocation even if its numeric state matches. Absent
arrival matching a retained allocation resumes it; mismatch moves it to pending
with ReloadMismatch, never silently replaces or discards its contents. No active
allocation plus no pending may admit empty if the arriving state matches.
All three hooks complete before their World methods return; coordinate observer
queues are not the ownership authority. SDK map lending is not a residency change.

take_world_slot_retirement() moves the pending value to a native caller exactly
once; it performs no automatic admission. Explicit reconcile_world_slot_trial()
can then admit a new empty allocation after fresh resident/binding checks. Taking
ownership is not proof that a sink accepted items; that later receiver must retain
the returned value on failure. Repeated take yields None, never a duplicate owner.
take_world_slot_trial() disables the service and returns a non-Clone teardown bundle
containing active and pending owned values/configuration/fault without cloning or
dropping their projections. Consuming accessors can return the owned SlotProjection
and metadata; this unit does not need a new extract-item API or serialize snapshots.
No automatic Drop delivery or crash-retention guarantee is asserted.

## Native tests and acceptance

Use actual World/Chunk state, Configuration and H06a projections, no renderer or
mock observer. Exercise initially air/matching/unloaded and duplicate enable, real
transfers in both directions, stale keys and changed limits; verify native identity
totals plus actual top-level/nested BundleSelected integers, not equality alone.
Prepare then set away/back: original plan refuses; pending retains exact old contents
and revision across repeated writes, matching chunk replacements and repeated take.
After moving the retirement out, reconcile admits empty under a fresh key; old plan
still refuses without affecting either current machine or caller-owned player.

Unload/reload matching retains key/contents; unloaded prepare/commit refuse unchanged.
Reload mismatch and resident replacement retain owned retirement with correct reason.
Different World scopes/trial allocations reject exchanged keys/plans at equal positions.
Test incarnation exhaustion and teardown with active, pending and unloaded allocation;
consume each returned projection once and account all items across returned owners.
Run native H05 regressions unchanged; paired support must work with runtime on/off.
Include coexistence with a faulted counter where practical: physical removal still
works and preserves the separate slot's owned contents. No joint commit is claimed.

## Following live adapter boundary and stop conditions

World maps remain publicly mutable; audit direct production writes and classify replica
and fixture paths before claiming lifecycle coverage. This unit's tests cover the three
authoritative seams, not arbitrary map mutation, gameplay drops or automatic World drop.
Keep Game unable to enable this service until a reviewed receiver handles active/parked
World loss, dimension replacement, idle close, exit/error paths and occupied sink refusal.
Level swapping must retain the same World owner. Raw spawn_item is not a checked sink.
The player side needs exclusive actual Game.inventory or placed owner Guest.inventory
access, real resolver/hand/menu/cursor/replay authority and complete retirement handling;
fresh revision-zero copies later overwritten into live inventories are insufficient.
Those are concrete later packets, not owner-approval blockers for this native unit.
Stop/rescope for Game consumers, counter joint commits, guest API, drops, dependencies,
generic transactions or persistence. Fresh independent review precedes implementation.
