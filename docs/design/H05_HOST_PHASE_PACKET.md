# H05c proposed host-only counter phase

Fresh independent read-only Sol High design review found no blocking issue;
coordinator accepts bounded host-only implementation. Stable source review and
native validation remain required. Source: counter_trial.rs/world.rs at engine
`f7f8990dbeaad2b4d036f2062d5a6dc0194c2cf4`. The separate import-free evaluator
packet does not qualify these host seams. Own no Game, SDK, evaluator or tool
changes here. Keep the native trial, native controls and qualified drivers intact.

## Scope and binding

Bound implementation to crates/horizon-client/src/counter_trial.rs and narrow
world.rs methods, with native tests in the existing counter test module. No
Wasmtime types, runner, callback, pending transaction, saved generation or queue.
The first consumer will be a separately reviewed authoritative Game packet.

Extend immutable Configuration with a small execution identity: Native by default,
or Guest { primary_owner, captured_package_identity: [u8;32] }. Configuration::new
continues producing Native with its present validation/error order. A separate
consuming guest-binding constructor accepts PreparedBundle, derives its identity
and primary owner, and checks its exact declared MachineType/schema and single
bound block against this configuration. It reads captured metadata only; it does
not certify a running evaluator or zero imports. Those are later Game prerequisites.
Do not accept a caller-provided owner/path/digest as equivalent captured provenance.

Configuration already derives Clone/Debug/PartialEq/Eq and Snapshot holds its Arc;
the added plain identity can participate in exact equality and ordinary snapshots.
No Component/Store/runner belongs in Configuration, Snapshot or a Game-global
backend selector. Per-World trial ownership makes binding follow existing swaps
and retirement. No native configuration is silently converted after enablement.

## Concrete ephemeral API

Proposed new World methods (names subject to review) are prepare_guest_counter_phase,
commit_guest_counter_phase and fail_guest_counter_phase. Prepare returns a checked
Result<Option<PhaseToken>, PhaseError>. Commit/fail consume the token by value and
return an explicit committed/faulted versus skipped/stale disposition. Distinguish
wrong backend and existing fault from ordinary no-work. No boolean may disguise
an invalid output as a successful commit. Native advance remains its existing API.

PhaseToken has private fields, no public constructor and no Clone/Copy. It carries
Arc<Configuration>, exact Record (instance/counter/last_advanced/resident) and current
World.game_time. Thus scope/dimension, type/schema, position/canonical state/state ID,
package identity and prior phase are captured. Expose only read-only request getters;
do not hand a mutable Trial or authoritative payload to the evaluator. A plain
untrusted fixed-scalar Proposal carries the evaluator's echoed world/instance/time,
expected counter and proposed next counter, without a runtime feature dependency.

World resolves actual resident chunk/block on every prepare and completion. The
authoritative caller supplies fresh running and combined ticket/storage eligibility
on BOTH calls: World cannot infer Tickets from chunk residency. This is the same
explicit authority boundary as advance_counter_trial, not a cached active flag.

## Ordering and outcomes

For a guest-bound trial, preserve native advance's relevant check precedence:
existing fault first; absent record -> no work; stopped/ineligible/nonresident ->
no work; absent storage -> no work; mismatched resident block -> sticky
ResidentMismatch; same last_advanced/game_time -> no work; checked +1 exhaustion ->
sticky CounterExhausted. Prepare changes neither counter nor last_advanced. Disabled
World returns no work; a Native trial refuses guest preparation without mutation.
Only a returned token permits evaluation. Duplicate/frozen/ineligible phases make
zero evaluator calls. Do not impose monotonic time: the native test explicitly
permits a different earlier time; equality alone identifies the duplicate phase.

Completion first compares full configuration, record and current time to the
token. Disabled/reconfigured/replaced/advanced/time-changed trials make it stale;
no effect or fault may reach a new instance, even if its block/type is identical.
Do not overwrite an already latched fault. Then recheck fresh running/eligibility,
residency/storage and bound block in the order above. Lost eligibility/storage
skips without commit; a genuine resident mismatch preserves native fault semantics.

For a matching eligible token, commit requires every reply scalar to equal its
captured counterpart and next == captured.counter.checked_add(1). Counter and
last_advanced change together only after complete validation. The first commit
invalidates another token prepared for the same phase; a consumed token cannot
be replayed through the safe public API. No reservation/pending field is needed.

Fail consumes the same token and performs the same identity/current-state checks.
Only the still-matching eligible trial can latch a finite GuestFailure reason
(trap/fuel/memory/rejected/invalid reply/unavailable evaluator); retain exact
counter and last_advanced. Invalid commit replies use this same latch boundary.
Stale failures cannot poison a replacement, native trial or new configuration.
Faults block further prepare calls; removal/retirement still works. Off/on allocates
a fresh transient instance through existing behavior, rather than retrying a token.

Leave Trial::advance's existing native algorithm and precedence unchanged. The
World native wrapper must explicitly refuse a guest-bound trial without calling
native advance or falling back to it; use a distinct dispatch-mismatch outcome
whose handling cannot claim successful advancement. This new guest-only case is
not permission to reorder native errors or change native diagnostics/status JSON.

## Native evidence and limits

Use existing actual World/Chunk/MachineType fixture seams, not a renderer/Game or
closure transaction framework. Test native regression equality first, including
fault-before-pause, mismatch-before-duplicate, time rewind and overflow behavior.
Test captured binding refusal for wrong owner/type/schema/block, and independent
package identities with identical visible type/state. Check configuration equality.
Prepare frozen/ineligible/dormant/missing/duplicate/no-record cases with zero tokens;
the later Game unit proves zero real evaluator calls, not these host tests alone.
Test correct commit, wrong echoed fields/+0/+2, overflow, duplicate preparations,
double completion, advanced counter/phase/time, every identity/configuration change,
replacement, off/on and stale failure after replacement. Compare whole snapshots:
invalid/stale work cannot modify committed fields or a new trial's fault state.
Exercise live storage removal/mismatch and changed eligibility between prepare and
completion; test sticky faults, exact retained last phase and removal while faulted.
Feature checks keep paired native support working without runtime; no evaluator
dependency is added. Coordinator owns serialized checks after source review.

No Game backend selection, component activation, real callback proof, persistence,
slot transaction, G1 or performance claim follows. If this needs stored pending
calls, mutable backend rebinding, runtime-owned records or more source areas, stop
and rescope. These bounded correctness choices need no new owner policy; broader
machinery or materially expanded persistence still returns to its separate gate.
