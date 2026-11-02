# Game streaming ownership migration: held proposal

Status: HELD design, not an engine source assignment. This document proposes a
representation change and identifies an explicit decision for fresh independent
review. No retained remote Game is activated by this packet. S1/G1 remain open.

## Parent and purpose

Use clean engine `e72475864c13720fe066d5bd6335d652ce5cd2bf`, tree
`4a5567d16d154f8b720a8d8c6d82bf69a28f2ca5`, branch
`codex/s1-retained-remote-stream`, in the existing physical checkout
`.local/worktrees/s1-native-close-composition`. Reuse only that checkout's warm
`/home/arbiter/.cache/create-horizon/target-s1nc1`; no target or checkout copy.
Private results checkpoint `0d5b1859c3739016bcf9064b94eea8b1ee075965` is synced.
Accepted predecessor records are `.local/s1rw1-final-review.json` ce3dae78,
accepted index `.local/s1rw1-final-review-input.json` 30ae3427, public sync
8c5315f4 and private sync 285bfed4. Those records remain immutable.

The actual obstacle is `Game.streaming: Streaming` (`game.rs:1899`). Original
native preparation and today's ordinary remote client both install that concrete
type. The qualified `RetainedRemoteStreaming` deliberately exposes no inner
Streaming, so an honest later client cannot preserve existing field syntax with
Deref or a placeholder pool. This packet makes the owner distinction explicit
without also migrating Session, decoder, save representation, packet dispatch,
render scheduling or startup/close composition.

## Proposed representation and exact ownership operations

Add a small private `game/streaming_owner.rs`, declared by `game.rs`. Proposed
non-Clone enum: `GameStreaming::Existing(Streaming)` and, with paired-packages,
`GameStreaming::RetainedRemote(Box<RetainedRemoteStreaming>)`. The Existing value
may also be boxed if needed for a sensible enum size; boxing moves the same owner,
not its actors or storage allocations. No inline-layout or performance promise is
needed. Boxing the remote wrapper does not extract its inner owner or allocate a
second pool. Do not add Empty, placeholder, fake writer, SaveDir or general owner
framework. Neither variant denotes native checkpoint authority: Existing also
contains today's legacy remote NoTerrain pools.

The small checked owner API has these semantics (names are proposed):

- `from_existing(Streaming) -> GameStreaming` moves the original owner once.
- `as_existing()/as_existing_mut()` return Result references to Existing only;
  RetainedRemote returns a fixed Unsupported error without opening its wrapper.
- `try_into_existing(self) -> Result<Streaming, Self>` returns the complete
  original enum on refusal. No taking a field before checking the variant.
- `try_replace_existing(&mut self, incoming: Streaming)` returns the old
  Streaming on success or the original incoming Streaming on refusal. Refusal
  leaves the slot's original remote owner unchanged. Success neither joins nor
  drops the old owner inside the helper; its caller owns the old value.
- `try_swap_existing(&mut self, parked: &mut Streaming)` checks first, then
  swaps the two original Existing/native-level values. On refusal both owners
  and all fields remain in place. No temporary substitute or extractor guard.

Do not implement Deref, AsRef<Streaming>, trait-based generic streaming, a remote
inner getter, a callback executor, or a conversion from RetainedRemoteStreaming
to Streaming. RetainedRemote exposes only its actual qualified wrapper when an
explicit variant match is needed. No mutable Game accessor is added. Any narrow
dead-code expectation for the deliberately dormant variant must name that reason;
do not suppress unrelated unused code warnings.

NativeCloseAuthority, NativeCheckpointOrigin and SaveDir/session allocation checks
remain byte-for-byte equivalent in meaning. An Existing reference proves only
which enum variant is held. It cannot replace any native provenance predicate.
PreparedWorld, PrepareOwner, NativeStreaming and Parked retain raw Streaming.
Wrap at Game adoption; use checked Existing swap at the active/parked boundary.

## Review decision: factory invariant now, or full remote guards now

Recommended minimum is a factory-invariant migration. ALL production Game
initializers and replacement sites still install Existing. In particular,
InitialGameWorld::Legacy and ::Retained at `game.rs:14082-14222`, native open and
dimension changes, and the legacy NoTerrain remote setup remain Existing. There
is no RetainedRemote InitialGameWorld variant, factory argument, command, option,
environment switch, or production assignment to Game.streaming. The remote
variant can be exercised as an owner slot in focused tests without constructing
a remotely running Game.

Under that minimum, current Game routines use explicitly named
`expect_existing_streaming` / `expect_existing_streaming_mut` assertions, or
equally explicit matches, tied to those audited current factories. This is an
assertion of the current construction invariant, not a remote refusal API and
not native provenance. Existing operations, return signatures, loading counters,
settings, update/save order and old-owner disposal order remain unchanged. Avoid
claims that every production route already supports a remote owner. A panic in
one of these assertions would reveal a broken construction invariant; it is
not acceptable behavior for a later supported remote Game. No panic-based inner
fallback is hidden behind Deref or common-looking methods.

This is an explicit alternative to the stronger initial idea of adding checked
remote refusal to every Game entry now. Fresh design review must choose; this
document does not silently waive that requirement. Before ANY production Game
factory may install RetainedRemote, all reachable production operations must
dispatch honestly or refuse before side effects, native acquisition, taking an
owned input or losing a refused payload. Replace all reachable factory-invariant
assertions at that later boundary. Known native fixtures may retain assertions.

Doing the full guard migration now is materially larger than the representation
change. For example, apply_settings currently returns () and terrain_distance
returns i32; their actual callers include options.rs:198 and render_tick.rs:101.
More importantly, apply_staged_pack(packs.rs:174) takes owned StagedPack and mutates
renderer/resources before stream reload at line 240. A correct remote refusal
must preserve that original input and occur before those mutations; server_packs.rs:484
is an additional caller needing explicit owned-refusal handling. Returning an
error after silently dropping StagedPack is insufficient. These extra paths and
API changes are NOT part of the recommended minimum whitelist below. If review
requires full guards now, revise the assignment and inventory first.

The representation-only choice has a real cost: broad mechanical churn before
remote player behavior changes, plus a later guard/dispatch pass. Its benefit is
an independently checkable owner boundary while native/legacy factories and
behavior remain unchanged. Combining it with actual remote initialization avoids
that second pass, but couples this churn to Session/decoder adoption, no-local-save
representation, new client scheduling and close ownership. Do not describe either
choice as a four-path change or force artificial proof helpers to justify a split.

## Actual field-use inventory and proposed source whitelist

Native-text inventory at the exact parent searched direct self/game/g.streaming
uses and then distinguished the actual owners. There are 39 candidate paths.
One, game/world_loading.rs, uses the separate preparation owners rather than
Game.streaming and is EXCLUDED. Its retained_prepare_tests.rs also exercises
PreparedOutcome owners and needs no change. The remaining 38 existing paths,
plus the one new private module, form the proposed maximum source whitelist.
Paths below are relative to `crates/horizon-client/src/`; no wildcard permission.
A path may stay unchanged when the source author proves it unnecessary.

| Allowed existing path | Actual reason |
| --- | --- |
| game.rs | Game field, constructors/adoption, native replacement/save, settings, view/loading queries, tick/render/LOD, report |
| game/command_harness.rs | Existing busy/readiness/center/pending fixture queries, lines 136-152 |
| game/commands/execute.rs | Existing loaded-column readiness, line 914 |
| game/commands/pregen.rs | Generation band, readiness, forced work and wait, lines 142-257 |
| game/commands/terrain_cmds.rs | Forced tickets and rank_forced, lines 44-124 |
| game/control.rs | Existing value/status queries, lines 1056-1063 |
| game/debug_keys.rs | Existing forced-ticket diagnostic, line 327 |
| game/dedicated.rs | Existing radius/center, lines 588 and 785 |
| game/dragon.rs | Native arena readiness, line 892 |
| game/end.rs | Native End readiness, line 692 |
| game/entity_culling.rs | Actual client is_meshing query, line 53; not a native-only classification |
| game/events.rs | Initial-ready witness uses actual pending queue, line 187 |
| game/initial_spawn.rs | Native readiness/center and retry state, lines 113-305 |
| game/lan.rs | Existing producer statistics and guest extra areas, lines 2594, 2821, 3629 |
| game/integrated/managed_stop.rs | Actual managed native Game preparation/update, lines 889-897 |
| game/levels.rs | Active native-level references, fixture seeding, active/parked swap, forced eligibility |
| game/levels/capture_integration.rs | Active native capture reference alongside raw parked Streaming, line 1715 |
| game/levels/session_checkpoint.rs | Actual native write fence, line 3559 |
| game/maps.rs | Client biome-preview zoom, line 875 |
| game/nether_portal.rs | Native portal readiness, line 1389 |
| game/packs.rs | Existing resource reload, line 240; no remote reload implementation |
| game/remote.rs | Legacy NoTerrain construction/replacement plus readiness/config/forget/view distance |
| game/remote_chunks.rs | Existing received-column insertion, line 934; no new retained packet path |
| game/saving.rs | Existing poll_save and forced-ticket publication, lines 141 and 182 |
| game/server_travel.rs | Native destination readiness, line 402 |
| game/window_close.rs | Original fixed native Game preparation/update, lines 318-321 and 997 |
| game/host_counter_bootstrap_tests.rs | Known native fixture wait, line 165 |
| game/lan/connected_recovery_tests.rs | Known native fixture center/radius/update/readiness |
| game/integrated/transition_ownership_tests/slot_retirement_tests.rs | Known native fixture replacement/update/wait |
| game/levels/capture_integration_tests.rs | Known native fixture replacement/update |
| game/levels/counter_restore_integration_tests.rs | Known native fixture capture guard and replacement |
| game/levels/idle_close_tests.rs | Known native fixture forced set and raw snapshot reference |
| game/levels/native_write_fence_tests.rs | Known native fixture capture, fence and ownership observations |
| game/levels/parked_save_tests.rs | Known native fixture forced set and raw snapshot reference |
| game/levels/session_checkpoint_tests.rs | Known native fixtures, wait/observations/view/extra areas |
| game/levels/slot_advance_capture_tests.rs | Known native fixture capture guard/replacement |
| game/levels/slot_epoch_integration_tests.rs | Known native fixture capture guards |
| game/levels/used_streamer_tick_tests.rs | Known native fixture capture guards and ownership observations |

New allowed path: `game/streaming_owner.rs`, with focused inline tests. Total
budget: 39 paths. Existing algorithm files, lib streaming.rs, net.rs, integrated.rs,
world_loading.rs, options.rs, render_tick.rs and server_packs.rs are not released
by this proposal. No dependencies, locks, public documentation or runtime tools
are included. If explicit accessor use requires changed signatures outside this
list, stop and return the exact dependency instead of inventing a facade.

Mechanical edits must distinguish Game.streaming from local raw streaming,
Parked.streaming, PreparedOutcome.owner.streaming and unrelated fields. Preserve
split field borrows: do not replace a field borrow with a whole-&mut-Game helper
that prevents borrowing World, parked levels or other original fields separately.
Use the enum's checked field-local accessors where needed. At levels.rs:1196,
check the active Existing variant before the multi-owner swap begins; never swap
world or invalidate provenance first and discover the stream mismatch afterward.

At owned replacement sites, incoming refusal stays owned by the caller. For
current factory-invariant sites an explicitly audited assertion is only the
minimum-choice behavior described above, never future supported refusal. Successful
replacement must dispose the old Existing owner at the same original boundary,
without an added join, new worker start, detach or broadened retained capability.

## Deferred real remote behavior

Do not add default values masquerading as remote view/status or forward a legacy
operation to the wrapper. The actual later common-client subset includes:

- Render center/radius, biome sampling settings and loading readiness; ordinary
  client entity culling also needs a real meshing-state policy.
- Packet insertion with owned backpressure, light-update mutation preflight,
  checked forget/untrack and view-distance changes.
- Explicit bounded Mesh/Light admission, original refused-input custody,
  reconciliation, current/stale handoff disposition and actual renderer uploads.
- Nonblocking optional observations in control/events; unavailable census cannot
  become zero pending work or a completed loading witness.
- Close/status/exact Retry and consuming ClosedRemoteStreaming with remaining
  payload disposition, without native writer/SealedStream completion.

Those APIs are not supplied by this enum. The qualified wrapper currently has
no readiness, forget or configuration interface. Mesh/light inputs capture private
settings (streaming.rs:5474-5520); current() at 5827 checks original region
allocations and sequence, not a configuration revision. A later initial profile
must capture settings before start or add real tested invalidation semantics;
a naive setter could accept an old-settings result as current. Pack reload and
dimension replacement are unsupported until their ownership paths are specified.

Legacy remote.rs remains legacy: it creates Session, a NoTerrain pool and
`horizon-remote-unused` SaveDir. Keeping that existing behavior is not adopting
it as the retained recipe or validating a fabricated storage witness. A later
actual retained initializer needs an honest no-local-storage Game representation
alongside RetainedSession and the retained decoder. Retained Hub/server startup,
scoped checked server joining and client LOD closure before shared storage sealing
remain separate. No integrated MemoryPipe, TCP-client, window or G1 claim follows.

## Focused validation and stop

After design and source release, use a few meaningful grouped checks. The new
module is GAME BIN code: it cannot call a lib testcfg-only fixture such as
streaming::native_work_tests. Reuse the already available bin
`game::integrated::transition_ownership_tests::Fixture::new/game` (both are
pub(in crate::game), with module visibility already pub(super)); its actual Game
provides pack.context from the pinned HORIZON_TEST_RESOURCES load. Do not change
the fixture module or widen that module's visibility. Standalone Existing owners
can use the actual TestTerrain/production constructor with that context; test
controls must not replace it with an echo worker. Original identity means the
same Shared/SaveDir/context allocations and owned payloads, not the address of
a movable outer enum/Streaming struct.
Do not add identity-extraction getters to lib Streaming for these tests. Where
those allocations are private, exact source review verifies the move/swap body
and the existing cold authority regression checks the actual SaveDir association;
report that distinction instead of claiming a scalar or address assertion proves it.

1. Original Existing owners survive checked borrow, consuming extraction,
   replacement and active/parked swap. Check original payload/ownership facts,
   not an echo fake worker or scalar-only replacement simulation. Successful
   replacement leaves the old owner with its caller for explicit disposition.
2. An actual inert RetainedRemoteStreaming prepared with pinned WorkerContext
   refuses Existing borrow/extraction/replacement/swap. Return the same incoming
   and original slot owners intact, then close the original prepared wrapper
   through its genuine checked API. Do not start a second pool or fabricate a
   remote Game to test this representation.
3. Audit every production Game constructor/replacement as Existing and make the
   dormant-activation prohibition explicit in source review. There is no runtime
   claim that all future remote Game routes reject safely under the minimum choice.

Choose the smallest affected existing regression: the actual original cold
factory/authority/checked-close test
`game::window_close::tests::cold_native_close_authority_binds_original_and_all_three_gates`
(observed module namespace). It covers original native Game adoption and the
three real native gates. New owner tests cover the raw swap itself. Only if the
delta changes active/parked swap order or disposal, add the specific affected case
`game::levels::parked_save_tests::parked_save_success_preserves_three_dimension_owners_and_independent_disk_records`
(observed module namespace). Reuse native fixtures rather than creating new
save/network machinery. Do not replay historical decoder,
streamer, shutdown-fatal, Java, save-failure or full window batteries absent a
changed dependency or concrete concern. If mechanical edits alter disposal,
acquisition or guard order, select the specific additional affected case.

Root owns serial formatting, paired all-target Clippy/compile and the feature-off
check, then exact test processes and ordinary public-tree checks. Compile all
affected targets, including native fixture paths, rather than assuming tests under
different cfgs still type-check. Bind real resources and report missing fixtures
as failure; counts are planning estimates, not a cap. No new performance claim.

Before engine source: fresh independent Sol High DESIGN acceptance of this exact
proposal and bounded assignment, a synced private design checkpoint and root
release gate. Source/tool review precedes any execution; preserve the entire
candidate and prefix artifacts before corrections. Root binds post-format bytes
and obtains DIFFERENT FINAL before use or reviewed public branch sync. Preserve
all predecessor evidence. This proposal grants no engine edit, script/runtime,
Git mutation, publication, PR/merge/tag/release, default activation or stage advance.
