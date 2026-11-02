# C0e2a: retained complete native LOD family

Status: REVISED PROPOSAL; no source authority. The original one-path proposal
80067c93/assignment56a6eb36 was WITHHELD before edits because Vanilla sampling
creates a hidden Far decoration worker. Original drafts/review are preserved
under.local/s1ld1-design-attempt0. Accepted predecessor
c73abbef3f8175077abc5cd6e9e13e57c40cf401, tree
16a38ec0c9a5504ad29f3ff88b1aa3af82041017, different FINAL33db8d13.
S1/G1 and ordinary activation remain open.

## Exact revised source and physical checkout

SEVEN paths under crates/horizon-client/src, including focused inline tests:
lod.rs, streaming.rs, vanillagen.rs, vanillagen/lod.rs, save.rs, flatgen.rs,
testgen.rs. No lod/mesh.rs, horizon-world, Game, renderer, control, Cargo or lock
changes. Revise accepted scope before any other path. This replaces the rejected
one-path budget, not a standing assignment for window integration.

Use CLEAN new active.local/worktrees/s1-native-close-composition atc73, branch
codex/s1-retained-lod, dedicated EMPTY Linux target
/home/arbiter/.cache/create-horizon/target-s1nc1. Original checkout/frozen inputs
remain untouched. Freshly accepted IMMUTABLE_SOURCE_FREEZE_LAYOUT_ADOPTION applies:
immutable source-input archives/maps, actual guarded live qualification and
independent live/archive/commit comparisons. No target sharing/copying or cleanup.
A cold build is inspected/planned, never mistaken for runtime or new stage proof.

## Explicit source capability BEFORE acquisition

Add paired-only retained capability routing through ChunkGenerator and the
LodSource blanket implementation. Default is UNSUPPORTED, without calling worker,
sampler, fork or a legacy factory. Known providers supply an opaque privately
constructed capability bound to the exact ORIGINAL source allocation; validate
identity and retain that strong source before acquisition. No public constructor
or flag can declare an arbitrary Arc<dyn LodSource> helper-free/safe. Use fixed
concrete factories, not a general closure executor or new scheduler framework.

Explicit providers: VanillaTerrain, Persistent<G> only when its actual inner
provider supports retention, and known FlatTerrain/TestTerrain implementations.
Keep NoTerrain/unknown providers unsupported. Refusal preserves original source
and context; it does not create a helper and then drop it. Persistent retains the
exact original wrapper, inner provider, SaveDir, assets and biomes rather than
reconstructing a same-path substitute. Legacy methods/default/tolerant behavior
remain unchanged, including ordinary Lod::new.

## Complete native worker and data custody

Opaque RetainedLod keeps original Lod/frame state and registry. Its no-worker
constructor owns validated capability/context; start mutates SAME owner. Own
ORIGINAL Far terrain, each actual one-to-three Far sampling child AND actual
Far decoration handle. Allocate pending slots before spawn; registry owns each
handle before releasing its start latch. Preserve original pending source,
sampler/backend inputs across native spawn refusal. Outer panic cannot drop child
handles. A joined outer cannot stand for joined descendants or private Drop.

The retained Vanilla factory creates its real VanillaLodSampler/Builder under
this SAME group, registering decoration before it performs initialization/work.
Retained Builder Drop never invokes the legacy unchecked blocking join. Retain
decoration requests, current batch, finished keys, result ownership and terminal
history. Its actual initialization/generation failures become checked outcomes,
not logs alone or fabricated successful LodDetail. A permanent group/request fence
prevents late work after close.

decorated_batch currently owns generated proto chunks, neighbors and partial
maps locally. Retained mode must preserve original queued/active state and late
completed details through stop/panic rather than clear/discard those maps. Keep
shared/owned active-state custody outside an unwinding worker; no large unnecessary
voxel copies, no source sampler/callback while holding a registry lock needed by
UI close/status. Legacy speculative decoration remains isolated from gameplay and
normal saves; algorithms and seed/order/cache limits stay unchanged.

The outer/sampling loops retain queued jobs, observed Arc<Chunk> inputs,
Arc-backed snapshots, active inputs and late sample/detail/observe/mesh results.
Do not clear jobs/observed/results/pending sets as quiescence proof. ClosedLod keeps
original state and remaining payloads/outcomes through explicit closed handoff.
Opaque sampler-derived state may be uncertain after a panic; record that failure,
never pretend a successful result or replay work automatically.

## Truthful Persistent saved-column reads

Current SaveDir::lod_surfaces maps mutex/region/read/decompression/NBT/decode
failure to None and then procedural fallback. Add a RETAINED read path that
distinguishes genuine absence/ineligible saved surface from failure, retains
original relevant pending record or owned read input/state, and reports exact
SaveDir/root/column/stage with actual error. Native read/decode failure cannot be
claimed as successful procedural fallback or synthetic empty terrain.

Reuse existing native formats/codecs and pending-record authority. Reads remain
read-only, never create/truncate a region, dirty a record or acknowledge a writer.
Actual synchronous reads can block: retain original sampling actor and report
Blocked until it actually finishes. Retained sampler/factory interfaces carry
Result or equivalent checked failure; a forced legacy None sentinel must NOT be
published as successful output after a recorded failure. Legacy tolerant reading
and fallback remain unchanged.

Existing Streaming NativeWorkOrigin/OwnershipBook, retained save/load/proto and
writer receipts remain independent. LOD is not covered by Streaming readiness
merely because it shares a generator. SaveDir writer/streamer are original caller
owners, not silently absorbed/dropped by this LOD group; future composition must
close LOD before storage/world handoff.

## Fixed running and close interface

Expose fixed running operations needed by existing Lod callers, no mutable Lod,
sampler, registry, raw handle or arbitrary closure escape. Use observe_chunk for
chunk producer work and status/observe_close for lifecycle to avoid name collision.
update_frame receives camera/settings and owned covered set; rejected operations
return original owned Arc<Chunk>/covered input as applicable. No silent input loss.
Queries/mesh progress are not termination proof. Stop seals all producer/child/
backend admission with one permanent fence.

begin_close requests stop and notifies real waits without joining. poll_close
takes only positively finished handles, joins once outside locks and preserves
actual outcomes/unfinished owners. Existing factory/fork/grid/detail/native IO
can be noninterruptible; deadline does not cancel it. Only explicit Retry renews
the deadline/reissues stop; no replacement worker, reopened admission or job replay.
try_closed consumes actual complete group quiescence, with failures still visible.
Unresolved retained Drop fails loudly; ordinary legacy Drop behavior stays default.

## Focused real qualification

Root serially owns all fmt/compiler/runtime/Git. Authors native scoped TEXT only.
Fresh exact SOURCE/tool CLEAR before execution, different FINAL/private checkpoint
before use/push. Preserve ALL SEVEN sources plus prefix before corrections. Use
pinned Rust, offline --locked and this checkout's dedicated target. Test processes
spawn native workers only, no compiler/Java/Python children, no playable data.

Use actual Far terrain/sampling/decoration loops and production Vanilla/Persistent
factories with pinned real prepared worldgen/assets/WorkerContext. Missing inputs
FAIL, not skip. Source provenance and full resource inventory pinned at batch
boundaries, not per case. Fixed trait fixtures/gates may prove actor/race controls;
they cannot substitute for production decoration/read proof or claim worldgen
parity. Known Flat/Test capability paths need actual positive/refusal coverage.

Defining cases: no-fork and one-to-three-child groups; actual decoration startup,
request/active work/late details; factory/fork/acquisition racing stop; all native
outer/sampling/decoration spawn refusal and panic including partial startup and
siblings; queued/active/late sample/detail/observe/mesh state; pending and native
saved-column read success, genuine absence, actual IO and malformed decode failure
without fallback; unsupported/foreign capability refusal before factory; original
owned producer-input refusal; finished-but-unjoined refusal; deadline/nonrenewing
poll/Retry with same owners; typed closed payload/history handoff.

Plan around16-24 defining names plus small affected regressions and existing real
mesh/water/edited-surface sentinel if shared setup changes. The expanded controls
address newly found real ownership/IO seams, not old network/save/Game batteries.
Exact selection/tool reviewed at source freeze; 30case/10-15min is a planning
prompt, not cap. Targeted format, all-target paired Clippy, relevant feature
checks and public-source/privacy. Separate unresolved-owner fatal case(s) use
exact markers/diagnostics/SIGABRT/core0/reaping. Finite backstops fail qualification,
never provide successful completion. No hard latency, delivery or global memory
bound beyond declared existing limits.

## Stop and next boundary

Stop at complete native LOD family FINAL, clean private checkpoint and reviewed
public sync. No window/ordinary gameplay activation, NativeReady guard relaxation,
Game getter, global CloseCommit or Game/Guest durability claim. Next separately
bounded native control, then honest window/preparation/Remote composition and
actual integrated/dedicated owners. No C1/G1/live S2, PR, merge, tag or release.
