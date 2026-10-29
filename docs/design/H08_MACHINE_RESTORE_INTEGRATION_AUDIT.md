# H08 native machine restore integration audit

PROPOSED source analysis only. No new field/API/schema, runtime enabling, native
machine restoration or S1/G1 acceptance follows. Horizon sources only; native
text/Git/hash reads and this document write, no execution/build/test/import/WSL/
network/formatting/engine edits/commits. Preserve coordinator status/results.

Inspected CLEAN engine codex/h08-capture-mechanism at
88bfcb05c4ac9c8b817c171e28ea6e5527b11cb5, tree
ffb1c243f7d630f85a364b893845305857f058b1. Coordinator reports six capture Game
groups and three feature lints pass; broad/final qualification is PENDING at this
audit's assignment, not independently accepted here. Private HEAD4d95e714624caca56f011df74b5d041a347f4ec9;
CURRENT_WORK and H08_CAPTURE_MECHANISM_RESULTS are coordinator changes, preserved.
Read HOST_CONTRACT and current capture/recovery/native counter/slot sources.

## Recommendation: restore a real counter, not another observer

Next propose ONE bounded native durable-counter restore cell: a real authoritative
Game advances the native counter to a nonzero value through its actual counter
phase; one complete epoch includes world identity, placement generation, allocator
head, nonzero payload and last_advanced; recovery builds a fresh Game from that
ONE epoch, validates its loaded world/type/state/clock, installs committed counter
state with NEW runtime/session identity, and resumes the native Game counter phase.

Do not implement only Record{counter,last_advanced} decoding and call it durable
machine identity. The native code has no durable world ID or generation owner.
Add a small opt-in metadata owner for this one-counter experiment alongside the
existing sole live Trial payload, plus a checked defining-module restore factory.
These are concrete proposed changes requiring fresh design review, not an already
accepted API or permission to generalize into a machine registry/journal.

Counter-only work can advance S1 without foreclosing inventories: use a durable
key shape compatible with the contract and keep payload/session identities
separate. Keep E1's whole item-slot refusal and item conservation obligations.
There is no need to decide on a broad journal or reduce conservation first.

## Actual seams and missing ownership

All source paths below are under horizon/crates/horizon-client/src unless noted.

| Source | Current behavior and implication |
| --- | --- |
| counter_trial.rs::Record / Trial::admit | Private instance/counter/last_advanced/resident fields; admit allocates global LAST_INSTANCE then ALWAYS sets counter0,last_advancedNone. There is no persistence decoder/installer. |
| counter_trial.rs::allocate_instance | Process-global AtomicU64 CAS, overflow refuses; restart resets it. It is a runtime allocation mechanism, not a durable placement-generation allocator. Do not deserialize a saved instance into it or bump it from disk values. |
| mutation_observer.rs::AuthorityScope | Explicitly transient NonZeroU64 world_instance plus dimension; host-supplied. It is neither world UUID nor generation. Reusing scope41 across processes can coincide with a restarted runtime instance; raw values alone cannot prove cross-process stale-key refusal. |
| world.rs::enable_counter_trial | AlreadyEnabled/air/residency checks, then Trial::new. A restore needs a separate checked factory preserving previous World on refusal; enable then patch/replay is wrong. |
| world.rs::advance_counter_trial; counter_trial.rs::advance | Reads current state and game_time; skips paused/ineligible/nonresident and EXACT same last_advanced time, checks increment overflow. Restore last_advanced verbatim so the captured tick cannot execute twice. |
| game/counter_control.rs::tick_counter_trial | Reuses actual authority, ticket and ticking-chunk eligibility. Counter snapshot/archive alone does not establish this live phase. |
| game/world_loading.rs | Native World.game_time comes from loaded Level.time. Restore that clock BEFORE installing/resuming; day_time and wall time are not this counter's work clock. |
| save.rs::Level / singleplayer_uuid | player_uuid names the canonical single-player record. ensure_player_uuid can derive offline Player identity. Seed/name/path/player UUID are not stable world identity; no separate durable world UUID or placement-generation allocator was found. |
| game/levels/capture_integration.rs | Current CounterArchive marks live_generation_restore=false; recover returns detached CounterArchive plus NEW SessionLock. It does NOT create a Game, install Trial, or restore generation. Keep this v1 archival behavior/claims unchanged. |
| counter_control.rs::counter_binding | Validates selected package catalog, unique single-state iron template binding, admitted registry state and schema1. Configuration::new alone only checks namespace/schema/properties/height; native fixture restore must also validate the actual prepared binding/package digest. |

Selected source hashes: counter_trial.rs a9dfe1a312abaf5d90658349640dd0a8f84087a1e48b06ec364faa39693c5bb66;
world.rs cd079e5a6a11604ab2dd5f791292c1cc880bf6b1db73d240d511f1fdfb68adab;
mutation_observer.rs746fc0617c94118e46a56d6ed3337d3d0e7b05bfe3dad3a5614f98f8e2cf258a;
world_slot_trial.rsc03bb3fbf3d3a207555d70a72d460d7ea5ad983359a842c32ffc9c15e30e881b.

## Narrow proposed durable domain and restore seam

Use a NEW versioned experimental COUNTER frame, separate from v1's archival
record. One world/dimension, one placement and optional absent/retired placement
state; no general map or item schema. Proposed concepts/signatures, not accepted:

    DurableWorldId([u8;16])
    PlacementKey { world: DurableWorldId, dimension, position, generation:u64 }
    CounterDomain { world, dimension, generation_high_water:u64 }
    CounterFrameV2 { domain, active: Option<CommittedPlacement>, saved_game_time,
                     package_identity, type_id, save_schema, canonical_state }
    CommittedPlacement { key, counter:u64, last_advanced: Option<i64> }
    RuntimeCounterKey { fresh_session, placement, fresh_runtime_instance }
    World::initialize_counter_domain(...) -> Result<DomainSnapshot, ...>
    World::restore_counter_frame(configuration, frame, fresh_session)
        -> Result<RuntimeCounterKey, RestoreError>
    World::read_restored_counter(&RuntimeCounterKey) -> Result<Record, ...>

One paired-gated optional World field should own ONLY durable identity/allocation
metadata and retain it when Trial is absent/taken; the existing Trial remains the
ONE live payload owner. Default None preserves normal transient trials. The
placement binding must be attached to native owner lifecycle, not a capture-test
constant or detached variable that silently survives unrelated replacements.
Exact field/type placement and error behavior need the packet's source review.

Generate a world ID ONCE for a newly initialized experimental domain with existing
horizon_protocol::crypto::random_bytes, preserving it in every coherent epoch.
That existing API uses getrandom and reports failure. No new dependency or seed/
path/player-ID derivation is needed. Recovery obtains it from the selected frame
and checks agreement with a new manifest world-ID field. Fresh session/transport
identity is independently generated per recovered owner and NEVER installed from
disk. An opaque runtime handle may additionally use fresh Arc identity, never
serialized. Existing native instance allocation must produce a new instance;
old scope/instance/Arc values are diagnostics, not a restore capability.

Proposed authoritative frame path: world/data/horizon/counter_checkpoint.json in
the immutable epoch, containing world/domain/allocator AND placement/payload in
ONE bounded member. It belongs to the recovered native working tree, unlike v1
research/counter.json. It is not an independent sidecar save authority: the same
manifest-last checkpoint promotes it together with PLAYER/chunks/metadata. Never
patch an old complete epoch or Level.player_uuid. Missing/wrong version/domain/
package/schema refuses install; do not create default generation/value on error.
A normal existing-world migration or runtime creation path is a later enabling
boundary, not a user fresh=true input to this fixture.

Restore should validate EVERYTHING before publishing Trial or domain metadata:
selected complete epoch, identity/digest/type/schema/binding, exactly matching
world/dimension/position, generation>0 and <=high_water, required nullable fields,
clock relation, resident chunk and resolved canonical physical state. Reject
last_advanced in the future of saved_game_time for the positive-clock cell. Preserve
exact nonzero counter/last_advanced; allocate fresh runtime instance only after
validation. Build local state, then install both owners without a later fallible
step. Occupied target, mismatch or overflow leaves the old live owner intact.
Do not expose Record's private fields to the binary just to set them.

Generation is an independent checked allocator: restore its HIGH WATER, not just
active_generation+1. On a genuinely new placement allocate high_water+1; preserve
it across ordinary unload/reload. Retain the head even with no active placement,
so removal/take followed by re-enable cannot reuse an old generation. Exhaustion
must refuse or latch a reviewed fault while retaining old retirement/domain state,
not publish an unstamped fresh zero counter. New frame supports absent/tombstone
state if the test exercises capture after removal. No global-atomic reset in tests.

Stale reads/requests use fresh session AND durable placement AND current runtime
instance; an old handle from the source Game must fail against recovered Game even
when both Games still exist in the same process. Old generation fails after actual
remove/re-place. A request that only supplies a saved placement key is not session
authentication. Restore/rollback follows the selected checkpoint's allocator head;
post-checkpoint abandoned handles are rejected by their old session, not a promise
that a number can never recur across every possible rollback branch.

## Native lifecycle constraints that the packet must address

World::set invokes Trial::changed only when previous state != new state. That
retires and may admit a new zero-valued runtime instance. For this cell use actual
air -> iron removal/re-placement to prove new generation, even though the new block
type matches the old. A same-state World::set is a native NO-OP, not evidence of a
new semantic placement. The broader same-type replacement/cause contract remains
open until the engine has a real placement lifecycle beyond scalar equality.

remove_streamed_chunk marks the current counter nonresident; reinsert with
replaced=false and same state preserves current Record/payload. Replaced=true
retires/re-admits even when state matches. An opt-in durable binding must have an
explicit policy for that replacement path: refuse/fault and retain ambiguous
history, or validate matching saved placement metadata before restoring it. Do
not silently equate arbitrary chunk replacement with unchanged placement, and do
not change ordinary transient-trial semantics incidentally. Defining-module tests
can execute these native seams without widening them into public test controls.

Explicit take_counter_trial currently retires before returning its Snapshot.
Idle close emits retirement then removes World; that is not a durable generation
service. The new opt-in metadata must survive supported native removal/reload
lifetime or refuse unsupported teardown; counter-only tests must not claim that
current World slot payloads survive that close. Full Game destruction after failure
and all earlier disconnect/portal/item histories are still outside this cell.

## Minimum executable actual Game trial and one source-surface surprise

Game::tick_counter_trial uses ticking_chunk: center must be lit and ALL3x3
neighbors resident, plus actual ChunkTickets levels. Current capture cell admits
exactly one column. Therefore a direct World.advance(true,true) test followed by
archive readback would MISS the real Game resume seam.

The new cell should explicitly admit one Overworld 3x3 patch (9 columns), authored
complete/lit during setup, and use native ChunkTickets::update(Sources with actual
players at0,0) to obtain ticket eligibility. Marking authored fixture chunks lit
is a declared eligibility fixture, not proof of asynchronous lighting completion.
No streamer update/submission/pump is needed. Expand the exact native region
footprint to rx/rz in {-1,0}x{-1,0}; preserve old1/3-dimension capture regressions.
PLAYER positions stay in actual resident columns; WORLD/E1 restrictions remain.

Drive real Game counter phase at three declared increasing game_time values,
assert counter3 and last_advancedT. Capture v2 via the actual owner Task and same
physical-copy/manifest/fsync pipeline. Recover fresh files, load native Level time
and actual chunks through SaveDir, construct a fresh authoritative Game/host with
the same canonical PLAYER mapping, and checked-install the native frame. AtT the
same actual phase must skip; atT+1 it advances to4 exactly once. Paused/ineligible
and unloaded positions do not advance. These are finite native host phases, not
replay to reconstruct saved payload, a whole world-tick/performance claim or a new
ordinary in-frame/SDK runtime caller.

A further real boundary: native decode retains InhabitedTime, structures and
PostProcessing in ChunkExtras.root (save.rs GENERATED_KEYS omits them); native
encode writes such defaults. W1 refuses nonempty opaque root. Do NOT clear saved
extras to recapture the recovered Game or blindly ignore W1. A first restore/
install/resume trial can stop before a SECOND capture and explicitly retain that
limit. If the packet also recaptures after resume/replacement, it must separately
qualify the exact native default metadata shape/provenance and a capture-aware
adapter, preserving original W1 behavior and unknown-data refusals. This is
concrete additional source work, not proof that a journal is required.

## Tentative packet surface, tests and decision boundary

Recommend the first bounded packet stop after real install/resume plus native
allocator/lifecycle/stale-key proofs, rather than bundling general recapture or
item-slot restore. Proposed paths: counter_trial.rs (private factory/binding),
world.rs (opt-in domain field/install and exact native lifecycle hooks), existing
lib.rs only if a tiny new identity/frame defining module is justified; NEW
counter_restore.rs for the bounded domain/frame (or defining counter child),
current game/levels/capture_integration.rs + its new dedicated restore test child,
and current capture_integration_tests.rs ONLY narrowly shared test visibility or
supplemental fixture (prefer preserve old helpers). New tests must access private
lib state through defining-module children. No mutation_observer/AuthorityScope
semantics change, SDK, general journal, loader or ordinary runtime caller needed.
Freeze exact file list/signatures/field ownership before implementation release.

Minimum independent tests: nonzero value+last_advanced full restore at same tick/
next tick; canonical package/type/schema/state/world/dimension/position mismatch;
occupied target and malformed/required-null/overflow images leave owner unchanged;
high_water greater than active generation then next placement allocates above it;
same-process source and recovered Games reject each other's runtime keys; new
session across isolated recovery; real unload/reload retains key/payload, real
remove/re-place changes generation and stale reads fail; exhausted generation
retains/refuses correctly; interruption/uncertain publication never installs a
mixed epoch; positive PLAYER/world data remains exact alongside the native counter.
No atomics overwritten to fake allocator behavior; no saved instance/Arc restored.

This is viable bounded experimental implementation, pending design acceptance.
Architecture/owner choice is needed only if asked to make a counter-only cell the
permanent S1 result, require arbitrary historical worlds now, relax ownership
conservation, or adopt a broad journal/history/branch policy. Full item-bearing
WorldSlot needs its move-only slot/pending ownership projected and restored under
the same coherent identity/epoch with fresh native SlotKey; its current Arc and
last_incarnation are transient. This counter frame must not be presented as that
item restoration or bypass E1/W1. Counter-only native progress preserves the full
S1 direction, while proving an actual state install/resume instead of only bytes.
