# Persistent retirement and readmission proposal

PROPOSED; no source authority. Begin implementation only after independent FINAL
acceptance of the frozen-window packet, its clean local engine successor and a
private evidence checkpoint. Record that exact successor and obtain fresh
independent Sol High acceptance of this proposal before source edits. The current
research base is a56e013 plus the reviewed frozen-window candidate; re-resolve
symbols against its accepted successor.

The owner directs full S1 completion. This is a bounded component, not reduced G1,
ordinary gameplay activation or permission for public publication.

## Concrete outcome

Preserve a healthy persistent counter/slot domain through real removal, retained
retirement, checked native item delivery and explicit fresh slot readmission.
Keep both independent generation heads and the original pending owner across
repeated physical writes. Existing mutation and residency hooks are reused.

Two proposed native World APIs in world/slot_restore.rs:

```rust
World::take_persistent_world_slot_retirement(
    &mut self,
    expected: &SlotKey,
) -> Result<PendingRetirement, SlotLifecycleError>;

World::reconcile_persistent_world_slot(
    &mut self,
) -> Result<Option<SlotKey>, SlotLifecycleError>;
```

The opaque key selects an existing allocation; neither API grants player
authorization. Use narrow typed refusals for missing persistent participation or
pending owner, stale key, Pending, nonresidency, binding/context/clock mismatch,
Faulted/Detached and generation exhaustion.

## Checked ownership transitions

Checked take requires healthy retained counter and slot domains, correlated
trial/configuration/scope, no trial fault or active slot and exact Pending
metadata/key correspondence. Validate current physical state/residency and
counter correlation through checked_slot_anchor without invoking the ordinary
persistent_slot_guard, whose Pending refusal is intentional.

Require BlockChanged retirement, a valid retirement clock, the pending slot key
generation within its retained slot head and the original machine generation
within the machine head. Do not require the original pending machine generation
to equal the latest counter retirement: repeated writes legitimately advance
that counter while preserving the first pending slot owner.

Finish every fallible check before taking anything. Then move the original
PendingRetirement out and set metadata to healthy Dormant without resetting
either head or allocation identity. No allocation, IO, callback or automatic
readmission follows. Every refusal preserves payload ownership, metadata, heads
and clocks. Keep the existing raw take_world_slot_retirement detach behavior.

Explicit reconcile is necessary because a replacement may already exist while
Pending; a later same-state World::set is a no-op. Make persistent Dormant
readmission explicit-only: suppress the existing persistent_slot_changed Dormant
automatic admission after a matching changed write. Its active retirement and
Pending retention behavior, counter generation hooks and explicit fault semantics
remain intact. A replacement written AFTER delivery must also stay healthy
Dormant and uncapturable until explicit reconcile. Update the existing Dormant
heads/exhaustion test to require that call; do not change transient admission or
initial active slot installation. Refuse Pending, Faulted,
Detached and nonresident state. Healthy Dormant plus a nonmatching target and
Dormant counter returns None unchanged. A matching target and healthy active
counter admits one fresh empty slot at checked slot_head+1, correlated with the
CURRENT machine generation. Existing Active returns its validated key without
allocating. Stage key/projection allocation before publishing metadata; add a
narrow crate-private Trial helper instead of mutate-then-check admission.

Example: slot1/machine1 remains pending while writes reach machine4. Delivery
leaves slot head1/Dormant. Reconcile admits slot2/machine4. Preserve counter head,
generation and counter value. Old slot1 plans and counter1 keys still refuse.
The temporary Dormant/active-counter gap remains uncapturable and unusable for
ordinary transfers until successful reconcile; do not loosen those guards.

## Actual Game receiver and history

Add deliver_pending_persistent_world_slot(expected) beside the existing Game
receiver. Reuse authority, dimension, residency, count/registry, entity-ID,
reservation and RNG preflight. Finish that work before checked take. After take,
move the original stack into the staged native ItemEntity and publish staged
ID/RNG/vector state with nothing fallible afterward. Empty acknowledgment creates
no entity and changes no RNG/ID. Keep transient delivery behavior unchanged and
do not reconcile implicitly.

Session checkpoint correlation currently rejects every counter.last_retired.
Accept only the existing-format bounded BlockChanged history, retaining all
CounterFrameV2 validation and envelope/package/world/clock/generation correlation.
Reject Explicit/ChunkReplaced, nonpreceding/invalid generation, future retirement
clock and malformed payload. No schema/codec change or general history promise.

## Six-path budget and validation

1. crates/horizon-client/src/world_slot_trial.rs
2. crates/horizon-client/src/world/slot_restore.rs
3. crates/horizon-client/src/world/slot_restore_tests.rs
4. crates/horizon-client/src/game/slot_retirement.rs
5. crates/horizon-client/src/game/levels/session_checkpoint.rs
6. crates/horizon-client/src/game/levels/session_checkpoint_tests.rs

Use actual identified Game resources and existing trusted session helpers. A rich
filled-slot case must remove, repeat writes while blocked, refuse failed sinks
without change, then deliver exactly once into a genuine native ItemEntity with
original nested metadata/payload and conserved totals. KEEP that entity owned;
successful session capture remains refused while it exists. Never delete the
delivered entity to make the checkpoint pass.

A separate EMPTY-retirement case must deliver, reconcile and genuinely
checkpoint/restore/recapture, preserving higher independent heads, BlockChanged
history and stale-key refusal. Cover replacement before/after delivery, duplicate
calls, wrong keys, missing residency, clock/configuration mismatch, sink ID/count/
reservation refusals and independent slot/counter exhaustion. Actual matching
streamer unload/reload keeps keys/content with no catch-up. Resident replacement
and mismatched reload retain existing explicit fault semantics.

Run focused native and actual-Game checks, appropriate feature/format/lint and
broad checks, then obtain a different independent FINAL review. Preserve failures
and every scoped source/tool attempt under the adopted rules.

## Open after this component

Dropped-owner coherent checkpoint custody, registered custom-block binding,
ordinary Game close/world replacement, filled-removal normal close, Running,
authorized player transfers, external admission/replication, committed animation
and the combined integrated/dedicated/two-client G1 experiment remain separate
required outcomes. This proposal adds no journal, generic executor, player
permission waiver, forced release or save-compatibility promise.
