# H08 native fence Task test seam amendment

DESIGN ACCEPTED, NOT RELEASED until private checkpoint is pushed. Fresh independent
read-only Sol High review accepts proposalSHA
06369385f46210d86f493e6e3b6df15998358365b0aed85e9408159a148c967f,
no blocking findings. Original snapshot .local/h08nf1-task-seam-amendment1.md retained.
Original design f770c99a remains accepted; author may work
only within its four released paths until this amendment has fresh independent
Sol High acceptance and the private checkpoint is pushed. No production scheduler,
capture admission or world startup changes are proposed.

Verified base: LOCAL c0ad9e09e1e2a641716d978600153fe142702ae7,
tree d5f5e6a371c5520ef7db9134a401752b77e6d348. Private resume checkpointdbc8019
is pushed. The author reported this dependency before any out-of-scope edits.

The accounted counter/slot/PLAYER cell uses the existing source/loaded/epoch
fixtures. Its actual native owner Task dispatch lives in
game/integrated/capture_completion_tests.rs and exposes only fixed capture kinds.
Levels descendants cannot access integrated-private Flags, Serving, Task queue or
run_tasks directly. Adding a new operation through declaration-only slot_epoch
cannot create an integrated descendant. Do not use macro/layering tricks or change
a running Game's execution role to bypass that boundary.

The existing fresh Client/open/integrated.call bootstrap pattern is real and stays
available, but rebuilding/importing this accounted cell into a newly launched
server would add a separate live activation/recovery assignment. Prefer preserving
the established cell and its actual Task queue for this bounded fence experiment.

Proposed minimum amendment: add FIFTH source path
crates/horizon-client/src/game/integrated/capture_completion_tests.rs, TEST ONLY.
Add a closed native-write-fence Task dispatch operation using the existing native
Integrated::call/run_tasks path and fixture exclusion. No public/general arbitrary
Task callback API, runtime bridge, scheduler/Flags/Serving visibility widening or
change to existing capture kinds/bodies. The levels declaration stays declaration
only; visibility for the NEW test child/owned result may be limited to crate::game
only if the closed operation needs it. Avoid broad old helper visibility changes.

Keep borrowed Game on its owning test thread; only owned handles/channels/request/
result data cross threads. Prove the fence and subsequent capture refusal execute
inside the actual queued owner operation with one exclusive native Game lifetime.
Retain definite completion independently of Integrated::call's observer timeout,
original result before cleanup/rescue, and release/reap every owned test thread on
success/error/timeout. Do not retry a timed-out observer operation. A test assertion
or channel failure must not strand a client thread or fake completed ownership.

Original f770c99a requirements stay unchanged: all workers initialized before
closure, fresh final producer/fault validation after writer polling under the same
exclusive borrow, honest callback/deadline/unwind limits, post-fence baseline,
retained counter/slot/PLAYER heads and PreviouslySubmitted/NotPublished refusal.
No ordinary capture/recovery, global readiness/epoch or G1 claim.

Exact-source review must verify the new seam is compiled only in test scope and
existing production startup/scheduler paths are unchanged. Add defining real-Task
evidence, retain all prior Task observer-timeout/exclusive-completion controls and
all original packet qualification. Root owns execution/integration/commits; no
additional source release until this exact amendment is independently accepted.

Accepted concrete seam: fixed dispatch_native_write_fence(&mut Game, owned_request)
returning owned completion, no supplied callback. Define new owned test-only
request/result types in capture_completion_tests.rs, at most crate::game visible.
Implement the fixed operation as a pub(in crate::game) Game method in the new child;
this avoids exposing its module or old helpers. Release the borrowed native fence
before post-fence baseline/capture checks, while retaining actual Task exclusion.
Exact fifth source pin cf7b8e46c15c110a8e6e24a1ae87fa3b979a5e2ecc7e92aa9e4e5f516ca4be6e.
