# H08 bounded native producer-to-writer fence proof

QUEUED FOR OWNER RESUME. Owner PAUSED after bootstrap qualification. Final bootstrap
acceptance/private checkpoint does NOT release this packet while paused. Obtain a
fresh owner resume and verify the accepted clean source pins before release.

DESIGN ACCEPTED, NOT RELEASED. Fresh independent read-only Sol High review accepts
revised proposalSHA f770c99a0d3e9cd800892af71386272539bd773dad0d35eb19581561fad4e4e3,
no remaining blocking findings. Exact snapshot .local/h08-native-write-fence-proposal2.md.
Current host-counter bootstrap qualification is pending.
Original proposal427a54af was WITHHELD for pending worker-initialization and final
post-writer validation gaps. This revised proposal addresses both; fresh review
accepted the revision above. Original snapshot .local/h08-native-write-fence-proposal1.md retained.
Release requires its different final acceptance, a pushed private checkpoint, an
exact clean accepted successor revision recorded by root, and fresh independent
Sol High design acceptance. Source pins are in H08_NATIVE_WRITE_FENCE_AUDIT.md;
recheck them at release. No ordinary capture admission or G1 promotion is proposed.

## Objective and four paths

Prove a bounded native write-producer fence under exclusive actual Game ownership,
followed by the participating SaveDir writer barrier, in ONE used-streamer fixture.
Historical PreviouslySubmitted capture refusal MUST remain unchanged. This is a
producer/writer prerequisite, not a coherent checkpoint, native metadata snapshot,
complete item-owner inventory, recovery path or general plug-in capability.

Proposed paths relative crates/horizon-client/src:

1. streaming.rs: small no-rescheduling native fence scope and defining controls.
2. save.rs: focused deterministic writer test controls/proofs, preserving ordinary
   production writer semantics. No new durability format or retry policy.
3. game/levels/slot_epoch_integration.rs: test-child declaration ONLY.
4. NEW game/levels/native_write_fence_tests.rs: real used-streamer actual Task proof.

No capture_integration.rs, admission guard, SDK, schema, dependency, queue payload
or general Game scheduling change. Report a concrete outside-path dependency or
need for production writer lifetime redesign before expanding the packet. Root
owns all formatting/Cargo/tests/integration; one author, no shared writers.

## Closure of this named producer boundary

Use a small exclusive borrowed fence scope or callback, retained through producer
closure and writer polling. Do not expose a serializable/transferable readiness
receipt or allow ordinary update/rescheduling while that scope is held. Actual
Game Task ownership supplies the independent exclusion of ticks, other Tasks,
connections and owner-side producers for the entire experiment. Pause/hold alone
and Integrated::call's observer timeout do not establish that exclusion/completion.

Drain/reconcile supported native completions without scheduling new terrain,
lighting, meshing or generation work. Reuse native ownership accounting and retain
every payload. Generate cannot simply be ignored: it may return Done::Generated,
whose owner reconciliation can hand a fresh finished chunk to storage. Done::Proto
can be transferred to owned proto state without scheduling its future jobs. Preserve
unrelated Light/Mesh payloads in arrival order; their fixed native implementations
do not enqueue storage. Do not drop work or clear maps/counts/errors to reach zero.

Fence success requires no queued/worker-held/returned/reconciling write-capable
Generate/Decorate/Finish/Save work or storage callbacks, no pending native handoff,
and no unresolved accounting, receipt, worker init/exit, stopped/disconnect or
generation-storage fault. Settle real completed leases only after their payloads
have moved to the correct native owner. Flush any owned written protos by the
existing storage contract before closing this boundary. A callback accepting an
enqueue is a handoff, never writer completion. Error/partial enqueue retains all
existing payload ownership and sticky failure semantics; no rollback/repair/retry.

Before closing that producer boundary, require completed initialization of EVERY
actual pool worker, comparing initialized_workers with actual pool size under this
same borrowed scope and failing deadline. Pending generator.worker() has no work
lease or fault yet; zero counts or initialized_workers>0 is insufficient. Delayed
initialization and initialization panic must be exercised deterministically.

Only inspected compiled built-in producer/callback behavior is in scope. Establish
the actual fixture provider independently. Arbitrary ChunkGenerator callbacks can
write or block independently and cannot gain this premise by supplying an enum or
claiming no current work. No general capability label or new provider registry.

## Writer ordering, deadlines and honest limits

AFTER that producer boundary is closed, poll writes_finished for every SaveDir
participating in this one-world fixture while the same exclusive scope remains
held. Establish that no external/native producer can enqueue to those directories
during the proof. The fixture may require no parked dimensions or other writers;
prove that restriction rather than implying a cross-world epoch. Never replace
polling with unbounded flush, infer completion from handoff/batch counts, or
silently ignore a sticky writer error.

AFTER the final writer poll and BEFORE success, freshly revalidate under the SAME
exclusive borrow: every worker initialized, no write-capable ownership or pending
handoff, accounting/receipt consistency, worker init/exit faults, stopped/disconnect
state and generation-storage error. A fault arriving during writer polling must
refuse, even if an earlier validation and the writer count were clear. Test that
ordering with a deterministic fault-during-writer-poll handshake. No promise of
future worker health or global readiness follows from this bounded observation.

Use an explicit failing deadline for waiting/polling. Current writer panic/exit can
leave outstanding work without a named exit guard: the accepted outcome for that
case is retained payload plus failure/timeout, never completed durability. A new
production lifetime guard is outside this proposal unless separately reviewed.
The deadline bounds deliberate waits, not arbitrary user callbacks, blocking OS
I/O, mutex acquisition or thread destruction; no hard real-time guarantee.

Normal reconciliation may legitimately install completed native chunks/protos and
enqueue their pending saves. Do not call that preservation of pre-fence world bytes.
Record the post-fence baseline, then prove the subsequent capture Task still refuses
PreviouslySubmitted/NotPublished without changing those owners/source/archive.
Producer closure plus this writer completion is not full capture eligibility.

## Required evidence and acceptance

Deterministic native handshakes must cover queued, worker-held, returned and
reconciling work; a late finished callback after an initial writer-zero observation;
deferred unrelated payload identity/order; partial enqueue and ordinary I/O failure;
worker/accounting/disconnect faults; timeout and writer panic/exit with retained
pending snapshots. No sleeping as sole synchronization or unsafe global mutation.
Native worker unwind can destroy its worker-held payload before the fence sees
the fault; existing accounting does not recover it. Worker-panic evidence therefore
establishes retained fault/unresolved accounting and refusal, not payload recovery.
Ordinary failed handoff, writer failure and non-unwinding timeout controls must
prove the retained payloads their actual owners still possess. Do not fabricate a
snapshot or add a new worker-payload recovery mechanism to satisfy the test.
Record the original outcome before cleanup. Release/reap all owned test threads;
cleanup rescue must never convert an original timeout/failure into success.

Real resource proof uses an actual used-streamer Game inside an integrated Task,
explicit operation completion independent of call's observer timeout, native
producer-to-writer ordering, and unchanged actual capture refusal. Counter/slot/
PLAYER heads remain accounted; capture compares against the post-fence baseline.
No nested Cargo/Java/Python/runtime scripts in tests. Fresh owned roots/homes only.

Freeze exact defining names and source hashes, independent exact-source clearance
before execution, three feature lints/fmt/tree/scope, focused and retained native/
resource/identity/bootstrap/owner suites, ordinary baseline, fresh BOTH-mode Game
smoke and broad qualification, then different final independent review. Preserve
all E1/W1/history/scalar/native metadata/ordinary recovery/performance gates and
honest failures. A passed packet does not loosen PreviouslySubmitted or publish
anything. Stop only on a concrete unsupported dependency or necessary owner input.
