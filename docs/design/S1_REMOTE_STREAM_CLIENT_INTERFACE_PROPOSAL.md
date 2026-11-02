# Retained remote streaming: fixed client interface

Status: HELD design only; no source, runtime or Game activation release. Parent
is clean engine `9d3ecdc510adbeb70840371fec2c0b9db4c23746`, tree `7443d75e`,
reviewed public branch synced, private results `6219583` synced. S1/G1 stay open.

## Scope and purpose

The opaque owner already runs actual Mesh/Light without native storage, but Game
still needs nondefault mesh settings, received readiness, checked light/untracking
and original replaced-column disposition. Add that concrete interface in ONE
existing source: `crates/horizon-client/src/streaming.rs`, including its existing
retained-remote fixtures/tests. Proposed branch `codex/s1-remote-stream-interface`,
unique evidence prefix `.local/s1rsapi1`, same physical checkout and warm target.
No new module, actor, generator, owner wrapper, callback facade or storage witness.
Ordinary Streaming/Game constructors and all native paths remain unchanged.

## Profile known after Login

Keep `prepare(context, workers)` inert. Replace the dormant remote owner's
`start()` with `start(profile: RemoteMeshProfile)`. The public Copy value contains
sky_light, nether_shading, obfuscated_biome_seed: i64, biome_blend: i32,
smooth_lighting and cutout_leaves. Validate blend in 0..=7, the existing UI range;
invalid profiles refuse before mutation/spawn. Return the stored profile as an
optional value snapshot, never an inner owner reference.

Install the complete original profile, including `BiomeZoom::new` from the already
obfuscated server seed, before any original handle can start. No additional hash
of that seed. Repeated start cannot change it; partial start keeps the same profile
and acquired handles for checked closure. No runtime setter or revision framework.
Existing tests choose their formerly implicit defaults explicitly; audit all
repository callers before changing signatures. Stop if another source is needed.

This lets an actual future Session acquire and receive play Login first while
the streamer is inert, then start with its real dimension and seed. It does not
solve Game ordering: current poll_remote eagerly admits chunks before handling
earlier Joined/Login events in the same batch, and legacy remote_chunk can decode
synchronously to repair the placeholder format. Retained Game must process those
barriers first or retain/re-admit originals; no synchronous repair claim here.

## Received-world operations

Extend only the opaque fixed remote owner. No inner Streaming borrow, native
progress stage, generic update forwarding or value invented for unavailable work.

- `received_ready(&World, at) -> bool` means a resident received column with
  installed server light, checked against its actual lit/light_loaded flags and
  the owner's received tracking while running and unsealed. It is not mesh/
  renderer readiness, whole-client completion or native generation progress.
- `mesh_pending(section) -> bool` means a latest tracked mesh stamp still awaiting
  handoff for that section. It is conservative scheduling information, not proof
  that its region snapshot remains current. Actual current/stale remains the
  existing handoff comparison. Aggregate unavailable census remains None.
- `try_insert_column` keeps returning the original incoming Chunk on refusal,
  but success now returns `Option<Arc<Chunk>>` containing the original replaced
  allocation for caller disposition. No silent replaced-column drop.
- `try_forget_column(&mut World, at) -> Result<Option<Arc<Chunk>>, RemoteRefusal>`
  preflights before mutation, clears applicable received/current-work/dirty
  tracking, removes and returns the original resident allocation. Queued, active,
  stored and reconciling JobOwners remain retained; forgetting never cancels or
  drops their original snapshots/receipts. Late outputs become stale under the
  existing allocation/sequence comparison. No chunk-freeing thread.
- Checked `try_apply_light` accepts borrowed concrete protocol SectionLight
  layers and position, uses the stored profile skylight and the existing
  `light::update_light_layers` plus `Streaming::light_updated` algorithms. Validate
  the fixed client section count and each present nibble layer's length before
  mutation; malformed input refuses explicitly. Borrowed layers stay caller-owned
  on every outcome. Use a typed Missing/Unchanged/Replaced outcome. Unchanged
  preserves the World Arc; Replaced returns the original Arc plus changed section
  indices, after installing a copy-on-write replacement and marking actual dirty
  neighbors. Do not mutate the returned original allocation in place. Missing
  column is a typed no-application outcome, never a claimed successful update.

For all mutators, closed/not-running/busy or invalid refusal precedes changes to
World, readiness, current-job maps and dirty tracking. Use the existing try-only
lock discipline where preflight is needed; no actor lock wait. Close cannot be
turned into fake success, zero census or native provenance. The field-local
queries can read owner-thread state directly; no worker payload may escape.

`try_take` must compute current before retiring the job's coordinate stamp, then
remove a meshing/lighting mapping ONLY if it still equals that handed-off sequence.
Never erase a newer submission when handing off an older result. Keep original
slot/receipt retirement, 64-job capacity and stale handoff disposition. A pending
map must not remain forever after its corresponding handoff. Checked forget or
column replacement may invalidate tracked work, but never its retained JobOwner.

Center/radius remain client view state, not worker settings. Future Game must
source them honestly and refuse post-start profile/resource/dimension changes
before mutation. This packet does not add a common Game dispatch facade.

## Focused checks

Reuse actual pinned-resource retained-remote fixtures and original Mesh/Light
workers, not an echo worker or a parallel settings mirror. A few grouped cases
should cover: nondefault profile affecting real work and immutable across start/
partial refusal; original replacement/forget/light Arc custody and mutation-free
closed/busy/malformed paths; older handoff retirement preserving a newer tracked
job; stale late results after checked light/forget. Check unchanged-light identity
and actual changed light/remesh effects. Preserve existing real owner close.

Prefer extending relevant existing real-mesh, partial-start and capacity/handoff
groups plus one focused received-world group. Select exact names at SOURCE review.
No network/decoder/Game/window/Java/fatal replay or broad historical matrix without
a changed dependency or concrete concern. Ordinary/native constructors are not
changed. Counts are a plan, not a cap.

Root performs serial explicit-file formatting, paired all-target Clippy, feature-
off compilation, actual lib-test build, exact cases, public-tree checks and pinned
resource inventory before/after the batch. Bind actual artifact/profile/features/
binary. Missing fixtures, timeout, nonzero exit or absent pass/native observation
fail. Do not claim Game rendering or whole-client integration from library tests.

## Gates and limits

Fresh independent Sol High DESIGN acceptance of this exact proposal/assignment,
synced private checkpoint and root release precede source. Exact SOURCE/tool
clearance precedes formatting/compiler/runtime; renew post-format source binding.
Preserve all packet source and prefix artifacts before each correction. Archive
mutable execution/config/recipe/instruction inputs; raw live/archive/Git comparison
and explicitly unchanged CRLF baseline comparisons must remain distinct. Preserve
all prior freezes. DIFFERENT FINAL/privacy precedes reviewed public sync; fresh
visibility/exact origin and normal hooks apply. No PR, merge, tag or release.

Actual client Session/decoder/backlog ordering, sending prediction/refusal, Game
construction/dispatch/close, no-storage guards, server composition, admission/
replication, ordinary machine lifecycle and committed rendering remain later work.
No Game/default activation, whole-client completion, hard UI latency, global
memory/performance bound, G1 or live S2 follows from this interface.
