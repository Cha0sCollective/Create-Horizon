# Native item-column delay qualification amendment

PROPOSED; no ninth-path source authority until independent exact review and a
private durable checkpoint. Effective component is the accepted proposal
0e15ae34f9423ee375a976b94a41c68600177e2329b8acf119e62aab0b5e39aa plus this amendment.
Retirement FINALab239890cd62595889f7f0302cd1396a5e2c547c and private checkpoint6a569bc
are met. Existing eight-path authoring is paused for preservation/review, not FINAL.

The defining missing/delayed item-column arrival proof needs an actual native
worker gate. Existing WorkGate and shared worker fields are private in
crates/horizon-client/src/streaming.rs, outside the accepted eight paths. Do not
replace the native generator/provider, forge provenance or synthesize arrivals.
The component maximum becomes NINE paths: original eight plus streaming.rs only.

## Narrow concrete test seam

Under existing native-writer-test-control AND debug_assertions only, expose a
hidden non-Clone NativeColumnLoadTestControl and
Streaming::arm_native_column_load_test_control((x,z)). Arming refuses an occupied
single slot. No ordinary/optimized-build branch or general callbacks/executor.

Use a private one-slot Shared gate containing target, a once-only entered signal
and a release wait. Match only the real Job::Generate(target) and its actual
Generate lease, immediately after owner.advance(Worker) at the existing WorkGate
pause location and before actual SavedGenerator.load. Never wait while holding
the queue lock. The real native provider, work lease, origin and subsequent
production load/update path remain unchanged. Existing cfg(test) WorkGate
semantics stay unchanged; a separate feature-gated slot avoids its panic timeout.

The receipt exposes bounded wait_entered(timeout) and explicit release; Drop
releases/notifies. Streaming::drop MUST release/take this gate before its existing
save-drain/join path, even while an external receipt remains alive. Poison/error
cleanup cannot strand a worker. Worker slot removal compares actual Arc identity
and never removes a newer gate. Cancellation before entry must not create a
later blocking wait; tests cover arming/release/Stream-drop ordering. Source
review must verify every cleanup/unwind route and actual helper/thread reaping.

## Defining actual recovery controls

A private feature/debug test entry in scoped session_checkpoint.rs arms only a
far VERIFIED item column immediately after genuine PreparedWorld creation and
locked validation, before first streaming.update. A test-owned receipt is declared
after prepared so it releases before prepared/Streaming destruction on failure.
An entered observer carries no owner capability; it proves the real worker is
paused at this seam. Retain the ORIGINAL RecoveryRequest.deadline.

Deadline failure must occur after observed real worker entry, before any native
item publication. Source items, UUID/raw maps and complete image stay unchanged;
release/reap actual worker/resources before final assertions. A separate explicit
release after entry must permit genuine arrival, fence, exact loaded-queue
correlation, checked install and recapture. Cover receipt drop before entry and
Streaming drop during the wait. Existing missing-resident/queue tests remain
additional controls, not substitutes for delayed native arrival.

## Exact-native profile clarification

The native NBT decoder at nbt.rs245/249 normalizes negative floating zero. The
existing exact roundtrip/no-coercion profile therefore refuses negative-zero
persistent item values before UUID preparation or native writes when their bits
would change. Raw item-region scanning refuses those bits before tolerant native
decoding. Add raw/live refusal controls; preserve ordinary +0/plain-stone and
durable nested metadata positive cases. Existing block/player scanner behavior
and non-item profiles are unchanged. This adds no new format or arbitrary-world
compatibility promise. Independent review must assess this clarification against
the original supported-item boundary and conservation guarantees.

All original source/evidence gates and limits remain. Root alone serial runtime,
fresh independent exact-source review before compilation, different FINAL review.
No publication, ordinary close/Running or G1 acceptance follows from this seam.
