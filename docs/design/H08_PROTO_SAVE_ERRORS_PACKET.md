# H08a2 proto storage failure and retained ownership

Follow reviewed H08a1 commit `1e87673` with one native generator-contract change:
make proto storage failures visible without losing the proto snapshots. This is
a new bounded packet under the existing save feasibility audit. It does not
extend H08a1's completion result into a full checkpoint claim.

`ChunkGenerator::save_protos` and its erased Saves adapter currently return unit.
Persistent logs the SaveDir error; worker completion always reports Saved. Thus
synchronous save can clear dirty proto keys despite failure, and an evicted batch
can return as if clean. Change only proto callbacks to return a result. A worker
must return its entire owned batch plus the storage outcome. Restore ownership
and failed dirty keys before latching the first producer error. Later success
cannot erase that error. Direct failure retains written keys. Callback success
still means successful enqueue; existing SaveDir completion/error barriers remain
necessary for actual disk writes.

The fault is sticky for this Streamer's lifetime: further save requests refuse,
and new terrain producer jobs stop. Continue reconciling already submitted results
and ordinary lighting/mesh bookkeeping. Guard the entire producer section after
completion handling: far-proto eviction/save checkout, finalization, decoration
and new generation. Guard other save submissions before draining written keys or
taking ownership. Do not set paused before completion handling or silently resume
production after later success. Document the resulting terrain-streaming halt;
there is no automatic retry/reset policy or absolute new memory quota.

A per-key failed set alone is insufficient: neighboring decoration can consume
those protos, and continued generation could grow unsaved retention indefinitely.
Stopping new jobs preserves a finite already-submitted workload. Existing finished
chunks with their void callback remain a separate ownership/error gap; this packet
does not solve it or claim complete quiescence.

Keep production scope to streaming.rs, Persistent in save.rs, and the existing
generation-save test generator/API updates, plus focused tests. Do not change WIT,
world formats, inventories, player ownership, replication or record persistence.

Inject direct and worker callback errors. Prove intact returned ownership, sticky
refusal in sync/async/poll paths, preservation through repeated far/in-range update
and center movement, no retry/finalization/decoration/new generation after fault,
and continued reconciliation of pre-existing work/unrelated deferred results.
Retain normal generation, writer and actual two-mode save/restart regressions.
Tests requiring prepared data must assert its availability and run explicitly.

Stop/rescope before adding worker cancellation, automatic recovery/replay,
failed-finished-chunk/entity storage, or cross-owner checkpoint behavior. These
need separate packets. Require a fresh independent read-only Sol High source
review. Coordinator owns all Cargo, commits and integration. No public push.
