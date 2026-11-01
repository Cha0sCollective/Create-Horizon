# Failed Guest allocation disposal proposal

Status: PROPOSED DESIGN ONLY. Ending an old allocation's execution fault through
proven disposal is a PROPOSED semantic decision requiring independent acceptance.
It is not an active clear/rearm API, user-policy waiver, source authority, ordinary
Running/G1 decision, schema migration or public release.

## Prerequisites and exact scope

Source starts only after persistent Guest bridge FINAL
(S1_PERSISTENT_GUEST_COUNTER_PROPOSAL.md8cd402c6), registered lifecycle FINAL
(S1_REGISTERED_COUNTER_LIFECYCLE_PROPOSAL.md4ae01a12), their accepted exact clean
combined predecessor, a fresh actual API/resource inspection, durable private
checkpoint and independent Sol High acceptance of this exact bounded proposal.
Record the real resulting pins; do not infer that separately accepted components
already compose. Current registered candidate on1fa55c6 is Native-only and supplies
no persistent Guest execution/disposal proof. Root owns all runtime/fmt/Git.

Exactly one supported registered Guest counter and one host-owned slot at its
claimed target. Preserve captured identity/backend/schema, native physical-write
semantics, first execution fault while the failed allocation or its pending
machine custody exists, retirement history, independent heads and D item owners.
No new Guest3 field/kind/format, general rearm, journal, portable item codec,
framework or permission to edit unrelated lifecycle/transport/close paths.

The following seven paths under crates/horizon-client/src are the DESIGN maximum:

1. world.rs: checked failed-retirement preparation in the existing shared staged
   resident-write seam; default/native setter behavior retained.
2. world/counter_restore.rs: failed allocation/witness correlation, checked
   retirement and disposal transition; scalar history/heads preserved; inline tests.
3. counter_trial.rs: private retirement-disposal/fresh-admission helpers preserving
   last retirement; no general clear_fault or reconstructed Trial shortcut.
4. world/slot_restore.rs: checked borrowed failed-Guest pending view, original
   owner take and opaque prevalidated disposal stage/receipt.
5. game/slot_retirement.rs: actual native sink/empty acknowledgment and exclusive
   once-only disposal finalization after its real sink commit.
6. game/counter_control.rs: actual captured Guest lifecycle selection/service,
   readiness/refusal and fresh paired readmission integration.
7. game/levels/session_checkpoint_registered_tests.rs: existing owned materialized
   Game fixture; actual both-role controls and native coherent image cases.

The maximum is conditional on actual prerequisite APIs and test-helper access.
No new Game fields, test module, world_slot_trial.rs, command, save, item codec,
D portal helper or schema edit is presumed. If an exclusive infallible native
commit or original-owner helper cannot fit, stop for an exact amended budget.
The accepted lifecycle's native command ordering is a prerequisite, not permission
to omit it or independently expand this packet.

## Actual traps to resolve

At the inspected Native-only candidate, World::set commits the physical state,
then invokes persistent_counter_changed and persistent_slot_changed. The former
sees Trial.fault, marks structural DomainStatus::Faulted and returns WITHOUT
retiring the counter; the latter can still move the original slot into Pending.
checked_slot_lifecycle subsequently fails through checked_slot_anchor's rejection
of counter.fault. Trial::retire preserves the fault, and persistent_admit refuses
it. Calling today's take/delivery/reconcile cannot safely produce a new healthy
Guest allocation from this state.

Use the prerequisite Guest bridge's actual finite execution-fault witness and
immutable execution binding. Only structurally coherent allowed Guest execution
faults qualify. Domain/slot structural fault, Detached, missing/mismatched witness,
wrong key/profile/package/clock, unexpected replacement/nonresidency and unknown
custody remain refusals. Do not convert an already unexplained structural fault
into an execution fault or use a public snapshot/status as authority.

## Closed ownership transition

FailedActive: the genuine counter/slot pair remains allocated with its first
correlated execution fault. Healthy reads, callbacks, player transfer and native
fallback remain refused. A same-alias/no-op write cannot reset anything.

FailedPending: an authorized CHANGED native write away from the actual bound alias
retires the counter and original slot once. Retain the exact fault/witness,
retired counter record/instance/generation, original pending SlotKey/projection,
reason/time and independent heads. No new allocation can tick or be readmitted.
Pending checkpoint is Blocked; no fabricated empty/dormant frame is permitted.

Disposed/ReadyDormant: after the ORIGINAL pending slot has either become one real
native ItemEntity or been genuinely acknowledged empty, the failed machine
allocation and its pending machine custody have ended. Only that ownership
transition may end the old execution fault. Keep counter scalar retirement
history, claimed target/session/capture, physical nonbound descriptor and both
heads. There is no active record to reenable and no healthy runtime key to invent.
The delivered ItemEntity is an independent native owner of the same original
stack; it is never deleted merely to make a machine image appear empty.

This proposed lifetime rule does not promise a permanent fault-history ledger.
The former counter's value/time/generation/reason remain in existing retirement
history; its ended execution latch is not copied into an active replacement.
No operation clears a fault on an existing active allocation. There is no API
accepting only a generation, status Boolean or error string to dispose it.

## Native authorization and physical ordering

The existing registered lifecycle's actual GameLevel/player/command authority
and immutable policy select the target and actual registered Guest binding.
Do not add a separate permission rule or treat SlotKey as player authentication.
For a failed active target, the shared resident-write stage must validate the
actual old alias/state, session/target, finite fault witness, counter instance,
slot allocation/revision, residency, clock and all heads BEFORE physical mutation.
Prepare any fallible reservation/bookkeeping at that point.

Reject invalid/structurally faulted cases before the write. On an accepted changed
write, publish the prevalidated counter retirement and Pending original slot
without another fallible stage, entropy request or callback. Preserve native dirty,
lighting, block-log, observer and block-entity transitions exactly once. No changed
write or plain raw setter may implicitly admit the alias while pending custody
exists. Chunk load/replacement is not host-authorized disposal or placement.

Command old-owner/block-entity cleanup remains AFTER actual successful staged
resident commit, as required by the accepted lifecycle. A read-only preflight is
not a capability or permission for destructive cleanup. /setblock destroy retains
its actual multi-step behavior: earlier native removal/drops are not rolled back
when later alias placement refuses. Report refusal truthfully and preserve every
machine/native drop owner. No atomic fill/clone or broad rollback is promised.

## Exclusive original-owner sink commit

Proposed API names describe a narrow concrete stage, not existing methods:

```rust
World::prepare_failed_guest_disposal(expected: &SlotKey)
    -> Result<FailedGuestDisposal, LifecycleError>;
Game::deliver_pending_failed_guest_slot(expected: &SlotKey)
    -> Result<Delivery, DeliveryError>;
```

FailedGuestDisposal is opaque, non-Clone and non-deserializable. It binds the actual
World/domain session allocation, retired counter instance/generation/fault witness,
pending slot identity/revision/head, native target/reason/time and original owner.
It is not a public healthy key or a readmission grant. Its preparation and use
occur within the same exclusive Game/native operation, with an exclusive World
borrow or equally concrete current-API ownership proof preventing interleaving.
A caller-asserted valid Boolean or check-then-act status is insufficient.

Before take, the Game sink validates actual authority/dimension/residency, supported
native item and all nested counts, checked entity-ID space, destination capacity
and every needed reservation. Use an original borrowed pending view rather than
unbounded diagnostic snapshot clones. All context/read-set checks are repeated
at checked take. A refusal leaves original Pending owner, fault/witness, ID/RNG,
dropped entities, heads and native files unchanged.

For a nonempty original, retain the existing sink's exact movement: consume the
pending owner/projection into its original ItemStack, construct one native
ItemEntity with precomputed position/ID/RNG facts, publish it and then finalize
old-allocation disposal using the opaque prevalidated receipt. No callback, await,
allocation, checked increment or Result-producing work may occur after take and
before this terminal commit. Source review must verify the actual exclusive
field borrows and ordering, not rely on a diagnostic marker.

For a genuinely empty original, consume/acknowledge its actual owned slot and
finalize disposal in that exclusive commit; no entity, ID or particle RNG change.
Do not infer emptiness from a failed read or absent snapshot.

Finalization ends the first execution fault/witness ONLY AFTER the sink publication
or empty acknowledgment. It preserves scalar retirement history and both heads,
leaves both allocations absent and returns ReadyDormant. Merely returning an owner
from World::take, allocating an entity ID or preparing a receipt is not sink success.
A stale/replayed receipt cannot clear a newer allocation's fault or deliver twice.

The concrete stage must make the post-take native commit infallible under the
existing returned-error contract. This packet does not introduce a general
panic/crash recovery framework. If actual prerequisite ownership APIs require a
fallible post-take step or extra retained owner field, stop for an amended design;
do not drop the payload, claim success or silently clear its fault to fit seven
paths. No runtime callback or decoded request can create a disposal receipt.

## Fresh placement and persistence

After successful disposal, a later authorized CHANGED placement of the genuine
alias may stage a fresh paired allocation through the existing accepted lifecycle.
Require actual ReadyDormant state, original policy/session/target/capture,
coherent retained retirement/heads, no pending owner/fault/witness and a live
captured server evaluator. Prepare all fallible counter/slot construction and
both checked head increments before physical publication. Advance each head from
its own history, not from an assumption that the two are equal. The new counter
instance and SlotKey must be fresh; old keys/successes/failures remain stale.
No virgin bootstrap grant is recreated and no old failed record is resumed.

Pending disposal continues to refuse checkpoint/close success while retaining
actual ownership. Once ReadyDormant and the real D item are established, use the
existing Guest3 ReadyDormant path with no execution fault witness and no healthy
keys. The coherent image must include that genuine ItemEntity, its D identity,
complete native metadata and all participant owners, with the existing native
fences/copy/hash/promotion rules. A save failure retains the delivered entity;
retry does not rerun delivery or restore the same stack into the old slot.

This does NOT expand Guest3 to FaultedDormant or serialize pending machine custody.
If saving the still-faulted retired allocation before successful delivery becomes
a requirement, return for an explicit schema/profile decision; no kind reinterpretation
or unsupported-field waiver is authorized here.

## Defining actual evidence

Use genuine materialized captured resources and actual Game in BOTH
IntegratedServer and DedicatedServer roles, at accepted Guest/lifecycle pins.
The current Native-only registered fixture cannot establish these positives.
Root records exact source/fixture/profile/test census and runs each resource family
in its isolated process. No compiler/Java/test subprocess from Rust tests.

Required focused controls:

1. Actual allowed Guest failure with a rich original slot: first fault and exact
   value/time persist until allocation disposal; no later callback/native fallback.
   Different/structural fault and mismatched witness refuse before physical write.
2. Ordinary native changed removal yields exactly one counter retirement and one
   original Pending slot; no-op alias does nothing. Preserve ordinary block drops,
   logs and block-entity cleanup ordering, including truthful destroy replacement
   refusal rather than a fictional command-wide rollback.
3. Real dimension/residency/count/ID/reservation failures retain the SAME pending
   payload and fault, with unchanged entity/ID/RNG/heads. Repair and retry delivers
   once. Check exact rich/native nested representation, including fields gameplay
   equality omits; prove original stack ownership rather than cloned equality.
4. Empty acknowledgment creates no entity/ID/RNG change. Nonempty success creates
   exactly one genuine D ItemEntity; repeated delivery/receipt use refuses and
   cannot affect another allocation. Custody service still works while paused.
5. New alias placement while Pending refuses; after disposal it stages both fresh
   allocations and independent head increments atomically. Old keys, stale Guest
   successes/failures and old disposal receipts cannot commit/fault the replacement.
   Exhausted/stale preparation leaves physical state, owners and heads unchanged.
6. Pending checkpoint refuses; successful ReadyDormant plus delivered D entity
   archives, freshly recovers and recaptures with identical item accounting,
   independent heads/history and no invented keys. Copy/write/promotion failure
   and retry preserve the actual current owner without duplicate delivery.
7. Native/default/healthy retirement behavior and prior Guest faulted-ACTIVE
   storage/recovery remain valid. Unsupported chunk replacement, Detached and
   structural/pending inconsistency retain existing refusal/ownership semantics.

Independent exact SOURCE review precedes all compilation/runtime; a different
FINAL reviewer evaluates these concrete claims and original relevant regressions.
No all-possible-fixture or new framework gate is introduced. Unavailable required
API/helper access triggers a precise scope amendment before source edits.

## Completion boundary

This packet supplies host-authorized failed-allocation disposal and fresh-generation
readmission, if accepted and actually qualified. It does not rearm a failed active
machine or expand its storage format. Ordinary startup/retained close, player
intents/departure, C1 admission and real two-client TCP/MemoryPipe replication,
retained animation and explicit G1 remain separate mandatory S1 composition gates.
