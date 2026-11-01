# Persistent registered Guest counter and faulted storage proposal

Status: PROPOSED DESIGN ONLY. Full bounded bridge, including healthy AND finite
faulted durable recovery. No engine source authority, ordinary Running caller,
G1/S2 acceptance, public release or compatibility promise follows from this file.

## Actual pins and source-start gate

Research inspected clean combined LOCAL1fa55c619a2f4666a505461822a97efa5732db01
and the seven-file draft in .local/worktrees/s1-registered-counter-binding based
on that same commit. The registered draft is not an accepted FINAL. The combined
FINAL and registered FINAL must both be accepted on an exact clean lineage before
this source starts; record their actual hashes, do not invent a combined Guest pin.
Require a private durable checkpoint, fresh actual API/resource/schema inspection
at that resulting pin, and fresh independent Sol High acceptance of this exact
proposal/budget. Any source-parent/API change requires renewed scoped review.
Root alone performs all runtime, formatting, fixture compilation and Git writes.

This component closes the persistent Guest execution/storage gap exposed by the
Native-only committed observer a460f875. H05 transient Guest success and the
Native-only observer do not discharge it. Ordinary lifecycle, C0/C1, endpoint
replication, player authorization and retained presentation remain mandatory S1
composition work. No smaller G1 is selected here.

## Concrete baseline gaps

Existing reusable Game path is counter_control::tick_guest_counter_trial:
prepare -> actual captured server evaluator -> fresh checked commit/fail, without
native fallback. Mods::{prepare_counter_evaluator,counter_evaluator} checks the
exact captured owner/digest, real paired entry, Running plus side_run(true), and
compiled evaluator identity on every invocation. Reuse those APIs unchanged.

CounterEvaluator is import-free and creates/discards one fresh Store per call.
One fuel/memory budget covers start, realloc/lowering, body, lifting and canonical
post-return; no successful proposal escapes before those complete. No mutable
Guest cache must be repaired after failure: the next explicitly allowed call
would instantiate a fresh Store. Ordinary paired server activation is a separate
live prerequisite, not the ephemeral counter Store or a serialized authority.

Persistent NativeBinding::configuration constructs Backend::Native;
Domain::configuration_matches requires Native; Trial::restore_native rejects
Guest. World guest prepare/completion lacks the persistent phase guard used by
advance_counter_trial. Registered3ec deliberately preserves those restrictions.
Additionally world_slot_trial::Trial::restore_checked requires Native: a genuine
Guest-bound slot restore cannot fit the earlier nine-path estimate.

A failed Guest phase latches Trial.fault. Counter checkpoint/read, checked slot
anchor and registered recovery final-key reads currently refuse faults. Existing
CounterFrame2/SlotFrame1 cannot preserve a fault latch or distinguish computation
backend from the same captured package. Clearing the fault to archive, restoring
as Native, or decoding a healthy record then flipping its backend is forbidden.

## Exact ten-path DESIGN maximum

All engine paths are relative to crates/horizon-client/src:

1. counter_trial.rs: additive checked persistent Guest construction/restoration,
   finite execution-fault correlation and opaque phase support; inline tests.
2. world.rs: additive persistent Guest prepare/complete/fail dispatch with fresh
   actual domain/key/slot guards; retain transient and Native behavior.
3. world/counter_restore.rs: immutable execution binding, Guest counter3/status,
   checked storage view, atomic paired bootstrap, fresh recovery and inline tests.
4. world/slot_restore.rs: correlated Guest slot kind, additive checked storage
   anchor/capture/restore and strict backend/profile/generation correlation.
5. world/slot_restore_tests.rs: Guest/Native kinds, fault-state storage and
   generation/custody refusal tests; original Native semantics retained.
6. game/counter_control.rs: dormant genuine Guest bootstrap, actual evaluator
   selection and persistent phase wiring; no cross-sibling test declaration.
7. game/levels/session_checkpoint.rs: Guest image adapter, full native owner
   capture/fences, checked healthy/faulted recovery receipts and final correlation.
8. game/levels/session_checkpoint_registered_tests.rs: owned materialized Game
   healthy/faulted archive/recovery/recapture families and helper reuse; declare
   path9 as a child module, retaining private fixture/ancestor access.
9. NEW game/persistent_guest_counter_tests.rs: genuine persistent Game callback,
   read-set, bootstrap and stale completion/failure tests.
10. world_slot_trial.rs: additive checked Guest restore using original owned input
    and a fresh SlotKey/projection identity; existing Native restore unchanged.

Path10 is a required DESIGN expansion: without it restore_checked rejects Guest,
and backend flipping would be the only nine-path workaround. Root authorized
including this tenth path for independent proposal review, not implementation.
No source path outside these ten, runtime/SDK/WIT, save.rs, item codec, network,
World mutation API or ordinary lifecycle caller is authorized. Stop for a precise
amendment if a genuine helper/authority boundary cannot fit; do not weaken it.

Exactly one captured package, one schema1 registered propertyless counter, one
host-owned slot and one target per World in this Guest profile. No general machine
framework, multi-package Guest execution mode, journal or portable inventory codec.

## Immutable execution identity and proposed schema decision

Preserve NativeBindingProfile::{LegacyIron,RegisteredCube} as PHYSICAL profiles.
Add a distinct closed computation profile, provisionally CounterExecutionProfile
Native OR RegisteredGuestV1. Existing Native factories/kinds/CounterFrameV2 and
Native Slot1 representation remain unchanged, with no optional-field defaults
that reinterpret an old Native record as Guest.

A new checked from_registered_guest_prepared factory accepts the actual retained
Arc<PreparedBundle>, exact type and actual registered BlockRegistry. It performs
all registered factory checks, retains immutable captured ownership/code identity,
and selects RegisteredGuestV1. It is metadata/provenance, not activation or
bootstrap authority. Configuration construction uses Configuration::new followed
by the existing checked with_guest_package on THAT retained capture. No setter
changes an installed Configuration or existing runtime key. Domain matching
requires exact execution profile, captured primary owner/digest, machine type,
physical alias, position, scope and schema. Guest and Native for the same digest
remain distinct execution choices.

Adopt only after independent acceptance these NEW closed storage profiles:

| Family | Format / kind | Additional meaning |
| --- | --- | --- |
| Existing Native counter | Existing2 / existing Native kinds | Unchanged; no Guest status |
| Existing Native slot | Existing1 / existing Native kinds | Unchanged |
| Registered Guest counter | 3 / registered-guest-counter-checkpoint | Required execution descriptor and finite status |
| Registered Guest slot | 1 / registered-guest-slot-checkpoint | Existing native slot field shape, explicit Guest backend kind |
| Guest coherent image | 1 / horizon-registered-guest-session-checkpoint | Guest counter/slot members, native coherent owner envelope |

This is a format/profile extension, NOT migration or compatibility. Older readers
may refuse it. Guest slot1 is deliberately a new closed kind despite retaining
field shapes. Counter and slot physical/execution profiles must agree. Unknown
format/kind/status, mixed backend, Native counter+Guest slot, implicit missing
Guest status or mismatched coherent-image kind refuse before writable load/copy.
No fallback to Native or older epoch is allowed.

Use a separate GuestCounterFrameV3 Rust representation with strict unknown-field
refusal, preserving CounterFrameV2 serialization and validation. Within the scoped
session module, a closed internal Native/Guest frame adapter may share existing
native copy/fence/hash routines; it is not a general codec/backend registry.

Guest counter3 retains the counter2 world/dimension/position, captured package,
type/schema, binding, observed physical descriptor, independent machine head,
active/dormant placement and retirement history fields. Required execution is:
profile=registered-guest-v1, actual primary owner, and status Ready OR
Faulted { code, observed_tick }. Package identity is the existing exact capture
identity covering both executables/content. It does not deserialize a Store,
runtime key, live-server grant, evaluator handle or admitted actor.

Slot payload remains the existing native item codec with full accepted metadata;
no duplicate fault authority is stored there. Its new Guest kind, package/type,
world/target/schema, machine head/generation and independent slot head/generation
must correlate exactly with counter3 and the coherent image. Healthy active and
healthy paired Dormant are supported. Faulted storage in this packet is a stopped,
structurally coherent ACTIVE counter+slot pair; unsupported faulted removal or
pending custody remains Blocked as specified below, never fabricated Dormant.

## Finite actual fault authority and stopped recovery

Current actual Fault variants are DispatchMismatch, GuestFailure(GuestFailure),
InstanceExhausted, CounterExhausted, PersistentDomain, ReloadMismatch(actual) and
ResidentMismatch(actual). DispatchMismatch is a dispatch refusal, not a durable
fault. The finite GuestFailure variants are Trap, Fuel, Memory, Rejected,
InvalidReply, Unavailable and Evaluation.

The Guest storage allowlist is EXACT actual GuestFailure variants plus actual
CounterExhausted. Encode distinct finite codes guest-trap, guest-fuel,
guest-memory, guest-rejected, guest-invalid-reply, guest-unavailable,
guest-evaluation and counter-exhausted. observed_tick is nonnegative and no later
than saved_game_time. When last_advanced exists, require the strict ordering
last_advanced < observed_tick <= saved_game_time; the failed phase cannot be the
already committed phase. Previous committed counter/time remain valid. Defining
malformed-clock families refuse negative ticks, observed_tick before/equal to
last_advanced, observed_tick beyond saved_game_time and inconsistent missing
last_advanced/counter facts, preserving the actual original owners and observer.
CounterExhausted requires the real counter exhaustion facts, not merely that code.
All other structural/ownership/runtime-instance faults remain storage refusals
with original owners retained. Do not reinterpret PersistentDomain or a native
state mismatch as a repairable Guest computation failure.

Game's CURRENT real selector/evaluator results yield Unavailable or Evaluation;
invalid scalar output from CounterEvaluator also yields Evaluation. Do not parse
an error String to manufacture Trap/Fuel/Memory/Rejected. Host proposal validation
has a separate actual InvalidReply defense. Other finite enum cases may be tested
at the native host boundary, but only a real Wasm fixture establishes that Wasm
failure stage. Diagnostics remain bounded non-authoritative text outside storage.

Store one private domain execution-status witness, issued ONLY by valid current
persistent prepare/complete/fail, or reconstructed during fresh checked Guest3
recovery. It correlates exact current domain allocation, trial configuration,
record instance, first actual fault and its observed tick. Preserve the first
latched fault; later failures cannot overwrite it or make an unknown mismatch
look correlated. Trial.fault and this witness must agree; any missing/contradictory
state is structural Blocked, not an invitation to synthesize a status code.

Domain structural validity and execution readiness remain distinct private facts.
A Guest execution fault must make DomainSnapshot/runtime health report Faulted;
Domain::healthy, healthy counter/key reads, prepare, player-slot transfer and
ordinary healthy slot guards all refuse. A checked storage view may validate the
structurally intact domain while accepting ONLY that exact correlated execution
fault. It never sets Domain/Trial/slot state Healthy, clears a latch or lends a
healthy gameplay capability. Do not rely on a publicly reported Healthy status
while retaining a hidden execution fault.

Fresh faulted recovery reconstructs BOTH the original typed Trial fault and its
correlated domain execution witness BEFORE publication of the candidate Game.
Counter value/last-advanced, slot revision/payload and generation heads are the
last committed state. The stopped state persists through subsequent ticks,
archive/recapture and another recovery, with zero Guest evaluations and no Native
fallback. No automatic retry/rearm/reset-on-load is included.

An explicit rearm would need separate authority and review: same actual retained
capture/live evaluator, current domain/slot/lifecycle identities, no pending owner,
current native target, checked reason eligibility, fresh Store and a deliberate
state transition. This packet exposes no generic clear_fault API or public valid
Boolean. Replacing code/package cannot evade exact world/image identity pins.

## Bootstrap and genuine persistent phase APIs

Proposed bounded API names, to be checked against accepted source:

```rust
NativeBinding::from_registered_guest_prepared(&Arc<PreparedBundle>, type_id, registry);
Game::bootstrap_registered_guest_counter(type_id, position);
World::initialize_registered_guest_pair_from_host(binding, target, host_grant);
World::prepare_persistent_guest_counter_phase(running, eligible);
World::commit_persistent_guest_counter_phase(token, proposal, running, eligible);
World::fail_persistent_guest_counter_phase(token, failure, running, eligible);
```

Game resolves the one actual retained capture/type/registered alias, requires its
actual materialized resources and execution role, then calls existing
Mods::prepare_counter_evaluator and actual counter_evaluator selector. Preflight
or evaluator preparation failure leaves original World, native block, grants,
slot ownership and generation heads unchanged. Only afterward may the genuine
constructor-issued HostCounterBootstrap be spent. A caller-supplied digest or
preflighted component alone does not establish live server activation.

The new World pair initializer stages the genuine counter domain/trial AND empty
owned slot/metadata off-World, including all fallible allocations, new runtime
instances, session/scope and positive independent heads. Validate actual native
registered state, existing host grant Arc identity, dimension/new identified-world
eligibility and vacant owners. Publish both owners/heads and spend the real grant
once only after all checks succeed. Do not call today's counter initializer,
spend its grant, then perform a fallible slot initialization. Existing Native
initializer and one-use bootstrap behavior stay unchanged.

PersistentGuestPhaseToken is opaque/non-Clone/non-deserializable. It retains the
actual RuntimeCounterKey/session allocation and invocation-local PhaseToken,
plus the actual coupled SlotKey/generation/revision read-set. No cloned item
payload is needed for that read-set. Preparation checks structural persistent
guards, execution readiness, current package/backend/binding, real runtime key,
slot context and original borrowed slot metadata, then the actual running/ticket/
storage/time/native block facts. Counter-exhaustion during prepare must publish
its exact correlated finite fault witness while retaining committed fields.

Game continues to derive running and counter_position_eligibility itself. Select
actual captured server evaluator immediately before evaluation and recheck its
availability/identity before completion; no event pumping, ordinary output drain,
reentrant Game call or arbitrary callback between evaluation and host publication.
The import-free Guest receives only CounterRequest from the real token. It owns
no slot/inventory mutation; this packet changes counter/last-advanced only.

Completion first compares the ORIGINAL domain/session/key/configuration/instance
and slot read-set against the current native owners. A stale success OR failure
returns Stale without latching a fault on a replacement or touching any newer
owner. Then run persistent guards again, fresh running/eligibility/native checks,
proposal validation and checked arithmetic. Publish counter and last-advanced
atomically on success; on real failure preserve them and all native item owners,
then correlate the typed fault/witness in the same exclusive World operation.
A dropped token is an abort, not a queued callback. No dispatch fallback to Native.

Do not weaken existing transient APIs or admit transient Guest keys as persistent
ones. Native and Guest exact mode follows each World through real swaps/recovery,
not a Game-global selection alias. Recreated configurations and slot projections
always have fresh runtime allocation identities; persisted heads are not keys.

## Checked storage-only view and coherent native image

New scoped World methods issue a non-Clone immutable GuestStorageView from actual
counter domain/trial/slot owners. Its constructor checks the exact immutable
Guest binding/capture, session allocation, native physical state/residency, current
clock, independent heads and absence of structural fault/pending retirement.
It has three explicit checked paths: ReadyActive requires complete active paired
ownership with positive correlated generations and no fault/witness; FaultedActive
requires that same intact active pair plus the exact allowed fault/witness;
ReadyDormant requires BOTH actual allocations absent, coherent retained heads/
retirement history, the genuine current dormant physical descriptor and no
fault/witness. ReadyDormant yields no healthy runtime keys. A mixed active/dormant
pair refuses; faulted storage is ACTIVE ONLY and never uses the dormant path.
The active paths borrow original native slot data before encoding. No public
world_slot_snapshot/read that clones arbitrary nested payload first, backend flip
or fake healthy RuntimeCounterKey is needed.

The storage anchor bypasses ONLY execution readiness for that checked faulted
Guest view. Original structural Domain fault, Detached, SlotStatus::Faulted,
pending owner, nonresidency, key/generation mismatch and unexplained witness
remain refusals. Do not weaken ordinary checked_slot_anchor/persistent_slot_guard
or normal player/counter reads to make faulted encoding work. Any shared private
validator must take a closed internal mode unavailable to decoded callers.

Proposed Game adapters within session_checkpoint.rs:

```rust
Game::checkpoint_registered_guest_session(archive, epoch, deadline);
Game::prepare_registered_guest_session_checkpoint(self: Box<Self>, request);
Game::recover_registered_guest_session_checkpoint(self, request, retained_host);
// New receipt: ReadyActive { counter, slot } | FaultedActive { storage_receipt }
//             | ReadyDormant; never a fake healthy key for FaultedActive.
```

Keep existing Native/registered Native receipts and adapter semantics. FaultedActive
has an opaque storage-validation receipt bound to actual recovered owners, not a
constructor/default or conversion to healthy runtime authority. Its final check
uses the checked storage view and exact stored fault; existing healthy-key final
validation must NOT be called and then ignored. ReadyActive uses real checked
runtime keys. ReadyDormant preserves heads/history and exposes no invented keys.

Preserve the existing coherent native pipeline: session lock/origin, package world
pin, actual native block palettes and aliases, complete player rows/inventories,
D rich dropped-item identities/components/portal arrival custody, all participating
columns, native write fences, staged complete image, strict reread/hash/manifest,
publication order, retained previous epoch, owned candidate and final correlation.
Counter/slot/status are sampled before and after saving from this same checked
storage context. They must match exactly with all native owner projections.
Do not save a fault by dropping its slot, ignoring D owners, choosing an older
counter frame or publishing a machine-only sidecar.

### Running-package boundary for Unavailable storage

Current session_context always calls session_running_packages. A real Unavailable
fault caused by a stopped server would therefore be unarchivable without an
explicit narrow storage context. Add a private GuestFaultedStorage context ONLY
for this exactly-one-retained-captured-pair profile and an already correlated
Guest fault. It retains every native lifecycle/session/lock/origin/owner fence,
check_world/package file pin and exact retained captured artifact/type/registry
check. It may omit ONLY the requirement that this faulted pair is currently
Running; it neither executes the pair nor claims its runtime is available.
No missing/changed capture, mixed catalog, unknown owner, Native frame or healthy
Guest state can select this context. No user-provided status selects it.

Normal bootstrap, healthy Guest capture and ALL fresh recovery candidates require
actual paired server/evaluator preparation and exact live selector checks before
spending/installing authority. Thus faulted Unavailable can be safely archived
from retained host state, but recovery with missing runtime/client-only server or
wrong identity still refuses without publishing a Game. Existing Native
session_context/check_running remains unchanged. This is not a blanket waiver of
running-package/ordinary output ownership requirements.

### Faulted removal and structural stop

The existing persistent_counter_changed turns an already faulted counter into a
structural fault; slot mutation can retain a PendingRetirement which healthy
lifecycle guards refuse. Do not silently treat that situation as faulted-active
storage or clear it into Dormant. This packet adds no ordinary removal/placement
caller. Its dormant adapter must refuse a requested faulted removal before writes;
a genuine direct native mutation/replacement negative must prove original item
custody remains retained and archive/recovery refuses the inconsistent image.

Faulted active archive/recovery/recapture is required HERE. Safe ordinary faulted
removal/delivery/readmission is a HARD separate lifecycle-composition dependency
before ordinary Running/G1; resolve it with an explicit inventory-preserving host
operation or retained stopped owner, not another failed Guest call. An attempted
unsupported removal is Blocked with actual pending owners/lock, never successful
close or resource disposal. This document does not claim the ten paths solve all
ordinary removal behavior. No unreconciled pending payload may be erased to pass.

## Genuine fixtures and actual stage evidence

Reuse existing unpublished guest-counter normal/trap/reject/invalid SDK pairs,
canonical WIT and standard componentizer. They contain zero imports, ordinary
inert server hooks and the pure counter step. Existing features fail at input3;
normal commits +1. Each artifact is independently captured and pinned, so a fault
variant is not swapped into a running normal world by path or mutable flag.

Separate explicit PRIVATE fixture proposal budget, not engine source authority:
- tools/fixtures/guest-counter/Cargo.toml
- tools/fixtures/guest-counter/src/lib.rs
Only a mutually exclusive fuel-in-step-at3 variant is anticipated in those two
paths. Do not modify SDK WIT, add host imports or launch compilers from Rust tests.
Root builds outside tests, inspects actual exports/imports, captures both halves
and validates ordinary pair activation and evaluator behavior before Game tests.

No existing real pair supplies fuel/start/post-return faults. Existing
horizon-mods runtime counter tests establish the shared real evaluator start,
realloc/lowering, body, lifting, post-return and fuel boundaries. Preserve their
scope and rerun them unchanged for that contract; they do not by themselves prove
a particular failure stage occurred in a persistent registered Game.

A true component-start trap may also prevent ordinary server activation, in
which case the correct genuine test is bootstrap refusal before grant spend,
NOT a fabricated already-running persistent Guest. Evaluator-only start-budget
failure and counter-export-specific post-return failure need genuinely reachable
prebuilt paired fixtures with canonical activation exports. This two-path fixture
budget does not assume wit-bindgen exposes a post-return override. Before claiming
those Game stages, Root must obtain independent acceptance of the actual fixture
source/ABI and any additional exact private paths. If no reachable real fixture
exists yet, record the precise limit on that additional Game-stage claim; do not
substitute an injected String/GuestFailure or replay a proposal/diagnostic as
execution. Absence of every possible Game-stage fixture is not an automatic or
indefinite bridge/G1 blocker.

Mandatory persistent Game families are genuine normal/trap/reject/invalid and
the separately accepted fuel variant, alongside the healthy/faulted storage and
other concrete defining tests below. Their missing genuine evidence prevents
component acceptance. Shared evaluator boundary tests establish their own claims.
Additional Game-stage claims (including evaluator-only start/realloc/lifting/
post-return failure) require actual independently reviewed reachable evidence
before being made; those claims are not silently added as mandatory families.
No statement that all stages passed may be made from native host enums alone.

## Required defining tests and regression evidence

Use actual owned materialized resources and Game::new/start_mods, as the registered
FINAL fixture establishes. Require intended Loaded.root, no fallback/pack_error,
registered alias physical ID/palette, collision/occlusion tables and nonfallback
cube CPU model; vanilla cached Fixture does not qualify. Path9 is a child of path8, using its private owned fixture and ancestor native helpers without visibility widening or fixture cloning. Default legacy helpers remain unchanged. Resources outlive Games. Run each artifact/resource family in its
own isolated pinned process; do not mutate process environment after resource
statics initialize. Root records exact source/API/fixture pins and actual census.
No compiler/Java/test child inside a Rust test.

Both actual IntegratedServer and DedicatedServer Game roles must establish:

1. Genuine captured Guest bootstrap preflight/prepare before real grant spend,
   paired owner publication all-or-none; wrong type/profile/alias/dimension,
   missing evaluator, client-only Running, wrong capture and failed activation
   leave grant/block/heads/owners/files unchanged. Native bootstrap unchanged.
2. Actual Wasm +1 committed phases, exact last-advanced/tick; no duplicate tick,
   pause/freeze/no-ticket/unloaded catch-up. Counter and slot same actual Guest
   configuration, original opaque runtime identities, exact selected artifact.
3. Prepare followed by real unload/native replacement/retirement/recovery or
   slot-version change: stale success and stale failure cannot commit/fault the
   new owner. Wrong world/session/package/execution mode cannot use an old token.
4. Real trap/reject/invalid (and accepted fuel) preserve last committed value/time,
   all exact native rich slot representation and D player/drop ownership. First
   actual typed fault/status retained; zero later evaluations or Native fallback.
   Separately distinguish host InvalidReply defense from evaluator Evaluation.
5. Healthy active native save/archive/fresh recover/recapture; paired Dormant where
   supported; alias preserved; independent heads/history, slot revision/payload and
   D item identities/portal custody preserved. Old keys fail in the fresh Game.
6. Faulted ACTIVE archive/recover/recapture retains the exact finite latch and
   observed tick, last committed payload and all owners; stays stopped through
   ticks/restart/second recovery. Runtime healthy reads/player transfer refuse;
   checked storage receipt succeeds without manufacturing healthy keys/status.
7. Actual Unavailable via genuine server-side removal/stop using already accepted
   runtime controls: faulted-storage archive succeeds from retained exact capture;
   no Running claim. Missing runtime/client-only/wrong identity fresh recovery
   refuses before copy/publication. If the required genuine stop control is not
   accessible without an unscoped runtime edit, stop for a bounded amendment;
   do not use a forged failure Boolean as the defining availability proof.
8. Fault/witness mismatch, unknown structural fault, clock rewind, mixed frame
   profile/status/kind, native-v2 fields carrying Guest3 data, wrong primary owner,
   package/schema/head/generation and native alias failures refuse without owner
   loss or fallback. Actual unsupported faulted removal retains pending custody
   and refuses archive; report the separate ordinary lifecycle dependency.
9. Native filesystem/save/copy/promotion failure and explicit retry retain exact
   status plus all original owners; no success from a partial image. Preserve D's
   native reread/complete item representation tests, not shallow item Eq.
10. Feature-off/paired-without-runtime refuse Guest capability cleanly; Native v2/
    Slot1 bytes, factories, healthy/refusal/retirement/recovery cases remain valid.
    Existing transient H05 and native counter checks still qualify their own scope.

Read rich native values without pre-budget deep clone; keep BundleSelected and
nested raw metadata in exact in-memory assertions where native equality omits it.
Storage uses existing accepted native codec semantics and complete D projections,
not the Native-only replica view as an item owner or portable save representation.
No new clone-based hidden custody or relaxed raw NBT validation.

Independent exact SOURCE review precedes compilation/runtime; a DIFFERENT FINAL
reviewer evaluates the actual results and outstanding stage/lifecycle limits.
Root alone runs isolated focused/broad checks, all applicable features/test lints,
format/public-tree checks and original Native/D/registered regressions serially.
Source or helper expansion requires an amended review, not a test waiver.

## Stop and subsequent composition

Stop this bounded component after FULL healthy+faulted persistent Guest storage,
the explicitly mandatory genuine Game families above, the concrete defining
checks, accepted schema decisions and a truthful FINAL record. Additional optional
Game-stage claims remain limited to their actual evidence; missing every possible
stage fixture does not add an indefinite acceptance gate. There is no accepted
healthy-only substitute for this full bridge.

Later separately reviewed composition must resolve faulted ordinary removal,
source adoption with the committed observer, coherent retained close services,
C1 actual configured allocation/held liveness, real TCP AND MemoryPipe two-client
replication, player intent/departure custody and retained animation. These remain
S1/G1 requirements; no automatic Running/S2/publication follows from this proposal.

## Fresh registered-candidate test-module placement refinement

Read-only inspection of clean Rf6a3c6a found session_checkpoint_tests.rs declares
private mod registered. That parent file is outside this ten-path budget. Making
Materialized public cannot give a counter_control sibling access through private
registered. Declare the new file ONLY from already-budgeted path8 instead:

```rust
#[cfg(feature = "mods")]
#[path = "../persistent_guest_counter_tests.rs"]
mod persistent_guest;
```

The physical path9 stays game/persistent_guest_counter_tests.rs; actual prefix is
game::levels::session_checkpoint::tests::registered::persistent_guest:: .
The enclosing test/paired/Linux gates remain; mods gates genuine runtime cases.
No eleventh file, visibility widening or fixture cloning is allowed. Use
Materialized::new/fresh(owner) and ancestor native-owner helpers, but not existing
Native binding/session/recover adapters as Guest setup. Add a small real Guest
orchestration helper around the genuine atomic Guest pair bootstrap/recovery.
Never bootstrap Native then flip backend. This refines test placement only; all
production scope, ten paths, healthy/faulted semantics and source-start gates stay.
R remains a candidate until its actual FINAL. Fresh source review must enumerate
the resulting exact case names. No early source authority follows.
