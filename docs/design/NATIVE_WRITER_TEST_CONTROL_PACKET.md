# Closed debug-only native writer test control

PROPOSED ONLY, no source release. Concrete amendment prerequisite to strict actual
Game production-checkpoint writer-fault evidence. Existing cfg(test) library hook
is absent/private in the library linked by binary tests. Do not expose its callback.

Exactly TWO paths: horizon/crates/horizon-client/Cargo.toml and src/save.rs.
Inspected stable source3ac2d4c: Cargo.toml SHA0cca7a50e1f7b8154102393792076141b9b1854dee1011d845eb43111c42c6e2;
save.rs SHA977c129d1cd9fa5f5501d764065c7fd4d044bb8f69c1173658b3722c6c784599.
Keep all production/default behavior and existing native_fence_writer_step unchanged.
No SDK/WIT, CLI/environment flag, network surface, callback/path/code/error-string
input, fake writer result/count repair, new dependency or publication permission.

Add empty NONDEFAULT feature native-writer-test-control. New public items ONLY
cfg(all(feature="native-writer-test-control",debug_assertions)); document hidden
developer fixture API, not stable host capability. Extend ONLY existing private
WriterTestStep alias/field/init/clone/invocation cfg to any(test,all(feature,debug)).
The old pub(crate) cfg(test) arbitrary closure setter stays exactly cfg(test).

Proposed concrete public debug API, private fields/no Clone/Default/Deserialize:

- NativeWriterTestAction::{Block,Panic}.
- SaveDir::arm_native_writer_test_control(action)->Result<NativeWriterTestControl>.
- Control::wait_entered(&mut self,Duration)->Result<()> and trigger(&mut self)->Result<()>.
- SaveDir::observe_native_writer_for_test()->Result<NativeWriterTestObservation>.
- Observation copies actual outstanding_batches/pending_records/sticky_error and
  actual JoinHandle::is_finished (Option when no worker). It is non-atomic diagnostic
  data, never a durability/readiness capability. Locks taken/released separately.

Arming inserts ONE closed internal closure in the existing private slot, refuses
occupied slot, no overwrite/rearm framework. Actual writer receives an ACTUAL queued
batch, acknowledges entry without unwrap, waits for receipt command. Explicit trigger
continues Block or executes fixed Panic. Receipt Drop closes/releases gate and causes
CONTINUE WITHOUT PANIC; never synthesizes error/completion or modifies counters.
Canceled unconsumed hook may remain until next batch, then harmlessly continues.
Own receipt must release before Game/SaveDir destruction, including panic unwinding.

## Existing panic semantics and test teardown

Hook is before fallible IO closure/completion tail. There is NO panic catch there.
Controlled panic terminates actual writer, retains pending records, strands actual
outstanding count, leaves sticky error empty, makes writes_finished Ok(false), and
deadline fence times out. Do not call it immediate panic detection/recovery.
flush waits indefinitely on stranded count. SaveDir Drop closes sender/joins and
does not flush, so directly dropping a headless actual Game is supported teardown.
After panic never call ordinary close/save_game(true)/stop_integrated/flush. Preserve
original failure observations before cleanup; no fake count reset. Release/cancel
gates, settle owned tasks, drop streaming/Game/SaveDir clones before scratch removal.
No universal hard IO/lock/join deadline; root test-process supervision remains.
Promising ordinary close after writer panic requires SEPARATE production policy.

## Required evidence and boundaries

Root alone formats/Cargo/tests serial across worktrees. Fresh design acceptance and
private checkpoint before author release, exact-source independent clearance before
execution, different final review. No actual checkpoint API/G1 follows this seam.

New library controls: actual native queued batch enters Block, observation sees real
pending/outstanding; trigger writes real records; cancellation before/after entry
continues normally; occupied arm refuses. Panic actual worker finishes with pending/
count retained/no sticky error/Ok(false) poll, direct Drop does not hang. Keep old
H08 native hook tests/regressions. No new nested Cargo/subprocess fixtures in Rust.
Coordinator-owned temporary consumer probes demonstrate cross-crate positive API
availability and exact unresolved-name negatives, not mere successful build:

| Build | Feature | Public seam |
| --- | --- | --- |
| Debug | Off | Absent |
| Debug | On | Present |
| Release/fast | Off | Absent |
| Release/fast | On | Absent |

Release library UNIT tests retain old private cfg(test) hook; new public gate remains
absent. Future binary fault cases use all(test,paired-packages,native-writer-test-
control,debug_assertions) with suitable target guard. Release binary tests must not
refer to unavailable items. Verify source/public tree/lock/features and appropriate
existing feature and fast/release checks, scope2 only.

Later checkpoint packet must supply actual authoritative Game/native open/roster,
real successful storage activity and native save submission, timeout under held
real writer, controlled actual panic, real filesystem IO obstruction/sticky error,
late built-in Finish handoff and coherent copy/recovery/accounting. Hook entry alone
does NOT prove durability. Public observation and actual callback deltas must prove
nonzero work. Existing H08 late Generate remains separate evidence; do not relabel
late Finish as Generate. That seven-path production packet remains unadopted.

Stop before extra paths, default/runtime behavior change, generalized fault service,
arbitrary callback export, repaired panic accounting, ordinary ingress or weakening
pristine/recovery guards. No owner policy decision or permission expansion adopted.

DESIGN ACCEPTED at exactproposalSHAaa471a58f82c6f67da81ebbae82f4182163c7852f93c970ae4649ca3cdbf146c by fresh independent native-read-only Sol High review. Source confirms normal fast/release debugassertions off. Acceptance limitedexact2paths/closed controls/privatehooks/debug gate/no repairedpanicaccounting; authoritativeheadlessfixture MUST have NOliveIntegrated wrapper and settledtasks onpanic teardown. Privatecheckpoint stillrequiredbeforesourceauthorrelease; exactsource/qualification/differentfinalreview remain. No productioncheckpoint/G1 acceptance.
