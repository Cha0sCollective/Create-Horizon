# H11a retained diagnostic proposal

PROPOSED ONLY. Independent source-informed design from engine ea96036, not a
source release or H11/H12/G1 acceptance. May overlap dormant H09 work in disjoint
paths. All H09 and protected H08 paths remain out of scope.

Build ONE explicitly enabled original diagnostic pointer, ONE immutable model,
ONE retained instance and bounded compact pose updates. It is a client-only
diagnostic placement with its own serial, not a machine-generation subscription.
No server simulation/counter lookup, automatic animation or wall-clock phase.
H12 and machine attachment await H10 committed timestamped tracking inputs.

## Proposed source scope

Relative crates/horizon-client/src:

- NEW gfx/retained_diagnostic.rs: fixed resource/slot, geometry, compact parameters,
  draw and production allocation/upload diagnostics.
- gfx.rs: module/optional owner, pipeline and entity-stage draw, resource methods.
- entity.wgsl: Y-rotation vertex entry; reuse existing lit/depth-tested fragment.
- NEW game/retained_diagnostic.rs: client diagnostic owner/controls/status.
- game.rs: optional owner, frame preparation, world/dimension/close/unload retirement.
- game/control.rs: existing control transport route for on/pose/replace/off/status.
- game/packs.rs: successful resource swap invalidation; preserve old on failure.
- game/remote.rs: connect/leave/dimension/chunk forget/range shrink retirement;
  conservative invalidation on intercepted remote block updates at the anchor.

No SDK/WIT, dependency, package schema, mesher, content, physical binding, save,
fence or networking admission changes. Concrete extra path requires reviewed
amendment before edits. No generic renderer/allocator framework.

## Resource and ownership

Reuse gfx/lightning.rs retained local Vertex/instance/camera conventions, ordinary
entity fragment/material/depth behavior, shared quad indices and generated atlas
white texel. Do not use additive lightning material or per-frame entity geometry.

Original asymmetric box pointer: six quads/24 vertices, fixed face colors. Relative
to pivot X=[0,3/8], Y=[-1/32,1/32], Z=[-1/32,1/32]. Pivot at
(anchor.x+1/2,anchor.y+5/4,anchor.z+1/2), right-handed positive-Y rotation with
documented formula for positions AND normals. Its complete sweep is above the
iron fixture cube: narrow disjoint nonduplication, not arbitrary static suppression.
Record coordinates, original author/revision, vertex bytes/hash in results. No
Minecraft/Create/Flywheel asset bytes in Git.

One optional immutable GPU model and fixed compact instance buffer (target32bytes,
layout verified against shader); model upload once per active resource epoch.
Pose/frame/camera/light changes update parameters only. CPU subtracts camera in
f64 before small relative f32 conversion. Finite pose and explicit bounded ranges.

Stable client owner holds checked, nonwrapping resource epoch, client-world epoch
and instance serial. Exact handles include all three. Create refuses occupied slot;
replace requires current handle and issues fresh serial; update/retire reject stale
handles without mutation. Exhaustion refuses safely. Logical retire immediately
removes draw eligibility; wgpu owns already-submitted buffer lifetime.

Create requires client role, existing renderer, open client world, loaded anchor
column and selected diagnostic pair. Validation failure preserves current state.
No lazy renderer creation. Server/dedicated Game owners remain absent, including
integrated server Game. No diagnostic GPU allocation before explicit enable.

Retire on off/replacement, untrack/unload/range shrink, session,
dimension/world replacement, close and package disable. Retirement is logical at
the PROCESSED lifecycle entry point even without drawing; queued network packets
await existing Game processing. Network-arrival-time retirement while minimized
is not claimed. Frame checks additionally
reject stale epochs/missing column/package. Never infer machine identity from
block equality or stale section-mesh block-entity lists. Immediate anchor-update
invalidation covers intercepted remote packet/prediction paths only. Local diagnostic
off/replace commands retire their own model placement synchronously; other local
block mutation entry points are outside this packet. Universal in-frame simulation
mutation retirement is NOT claimed: the diagnostic is manually owned, with no
authoritative machine attachment. H10/H12 must supply machine-generation history
before that stronger lifecycle contract. No World mutation observer is added.

Resource reload stages replacement before commit. Failure retains old epoch/model/
instance. Successful swap invalidates old handles and drops old renderer resources;
rebuild one resource only while still enabled/selected, with fresh epoch/serial.
No reuse/forced destruction while submitted GPU commands may reference buffers.
Include six diagnostic quads in shared-index capacity; use entity texture binding
and normal alpha/depth-write/GreaterEqual material, never lightning additive state.
Intercept both remote chunk delivery paths before calling existing chunk handlers,
including replacement of a loaded column. Package-disable retirement uses existing
Game::apply_settings; captured paired owners are startup-fixed. Pipeline allocation
itself is gated by explicit enable/client role, as are model and instance buffers.

## Qualification and stop conditions

Root alone formats/builds/tests, serial Cargo/tests. Fresh exact-source independent
Sol High clearance before execution, then different final review. Defining cases
must exercise production calls and real Game lifecycle boundaries, not helpers
that merely increment counters. Existing isolated control/screenshot facilities;
scratch worlds/homes, no owner Prism/saves/session environment.

Integrated window and in-frame: visible original pointer, one model/instance after
warmup; varied explicit poses/camera/light retain geometry hash and allocation,
compact writes only. Replace changes identity; stale/occupied/nonfinite/unloaded/
wrong-role/no-renderer/exhaustion failures preserve state and upload counts.

Observe immediate retirement through actual chunk forget/range shrink/dimension/
world/disconnect/close/package-disable and minimized paths; stale handles cannot
resurrect. Successful resource reload creates new epoch/one upload; failed staging
preserves old visible pointer/epoch. Dedicated and integrated server: absent owner,
zero diagnostic GPU allocation attempts. Presentation controls never alter machine/
slot state. Baseline feature checks, source scope/public tree and appropriate
regression checks; no blanket unrelated reruns.

This proves retained diagnostic mechanics only. Automatic machine attachment,
same-type replacement identity, committed stop/reverse/freeze continuity, remote
two-client presentation, general static/dynamic exclusion, H12/G1 and performance
qualification remain separate. Stop for reviewed amendment if protected/H09 paths,
WIT/schema, general allocator, mutation observers or another owner are required.

DESIGN ACCEPTED at proposalSHA877534e18bac9389ef2726cbd7d393a36e6d7d5e0ebb8611be17a06b2a27ab0d by fresh independent native-read-only Sol High follow-up. Originalbb70538b was withheld solely for overbroad anchor-update retirement; revised packet limits manually owned diagnostic scope accurately. No implementation/H11/H12/G1 acceptance. Source author will use a separate clean worktree at ea96036, exact eight paths; coordinator alone runs all formatting/Cargo/tests serially across both worktrees. Exact-source review before execution remains required.
