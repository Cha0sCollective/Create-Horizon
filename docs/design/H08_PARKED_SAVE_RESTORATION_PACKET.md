# H08 parked-save active-dimension restoration

Accepted bounded design after a DIFFERENT fresh independent read-only Sol High
review with no blocking findings. Reviewed draft SHA256:
c4058acaf89e170a0ffdbb10b8bdf07da0471fd543e5d9ca9b15f34f5bd62180.
Implement against qualified local engine
3ded3a2f8b7e012eae08dfaf06e6985686e95912, tree
cffabea129c8e95fa74c19f4ac489a7c41fb2ac4, on
codex/h08-entity-owner-observation. Source/resource/native/broad and final independent
qualification passes. Private Y/equal-cog sandbox and final evidence qualification
also finish with this engine unchanged, releasing that temporary source freeze.
Coordinator assignment controls exact implementation scope. Follow actual source and prior accepted
ownership/raid results, not the earlier feasibility audit's stale error claims.

## Concrete failure and bounded fix

Game::save_parked_levels currently enters a parked dimension, calls
save_level(wait), and returns immediately through question mark on either error.
If a parked save fails after a successful swap, that parked dimension remains
active. The original active World/Streaming/SaveDir/entities are still retained
inside Levels.parked, but subsequent callers see the wrong active dimension and
hub encoding context. This packet corrects that failed-save restoration boundary.
It does not roll back writes or undo normal native producer side effects.

Production change ONLY game/levels.rs::save_parked_levels, plus one gated module
declaration for NEW game/levels/parked_save_tests.rs. No other existing body or
helper changes. Keep Levels/Parked private and reuse the existing enter_level,
swap_level, save_level, SaveDir and player/slot owners. No new fields, service,
identity ledger, callbacks, injected fault switch, custom transition, journal,
recovery format, public API or engine publication.

Current levels.rs SHA256:
6c11ec8d7b726cab0e90a54b647c517c048d85876e3927aa69cceef0f386cea7.
Preserve all entity-owner additions/macros/tests and their qualified lib helper.

## Exact result and restoration semantics

- Keep the no-parked-level early Ok behavior. Preserve the existing captured
  dimension list/iteration policy; do not sort or retry dimensions as an incidental
  behavior change. The active level remains the caller's to save.
- Record the original self.dim. Run the existing per-dimension enter_level then
  save_level sequence inside a local Result-producing block/closure, or equivalent
  explicit loop break. Stop at the FIRST error, with no later parked saves or
  metadata phase forced after it. Preserve that actual error object.
- After the loop has produced either Ok or Err, ALWAYS attempt the ORIGINAL
  native enter_level(None, original_dim) before returning from the method. This
  is evaluated even when the loop failed. Do not short-circuit restoration behind
  and_then/question mark on the loop result. No tick/control/guest/pregen pump or
  external callback interleaves with the operation.
- If the loop failed and restoration succeeds, return the original loop error.
  If both succeed, return Ok. If only restoration fails, return that error.
  If both fail, return the original loop error and explicitly log the secondary
  restoration error with requested original dimension and ACTUAL current self.dim.
  Do not silently discard a second error or replace the primary with it. This
  follows the existing save/flush precedence style in integrated shutdown.
- The secondary diagnostic must accurately say that restoration failed and name
  the current active dimension; it must not claim rollback or successful return.
  Existing Game/Levels owners stay in their actual current locations. Never take,
  clear, default, remove, rebuild, drop or force-swap owners merely to make the
  restoration outcome appear successful. No panic/abort recovery claim is added.

Using an explicit loop_result and restore_result, then reporting the latter and
combining with ordinary Result precedence, is sufficient. There is no need for a
new reusable error/guard framework or a Drop implementation performing a fallible
transition. Do not use a raw saved World/SaveDir swap to bypass enter_level's hub,
clock/weather/rule, watched-state and existing native per-level exchange behavior.

## Why native restoration failure is not a new runtime oracle

At this base, enter_level's only Result failure is open_level when the target is
ABSENT from Levels.parked. A target which already exists is moved by the existing
native swap without another fallible open/save. The first successful iteration
parks the original active dimension. save_level does not remove parked entries;
subsequent synchronous iterations keep that original owner parked. Thus restoration
to the original dimension has no fallible open under the normal invariants of this
particular method. If no swap happened, returning to self.dim is already a no-op.

Retain the Result and defensive secondary-error handling for existing/future
native failures, but qualify this reachability limit honestly. Do not delete a
parked original owner, inject a callback between loop and restore, mock the native
transition, poison unrelated state or add production fault machinery to manufacture
a supposedly real restoration failure. The packet's executed faults are actual
parked SAVE failures followed by actual native restoration. Source review covers
the unreachable-at-this-base defensive double-error branch; no executed two-error
restoration proof is claimed. If new source reveals a real reachable restoration
failure, report its exact path before changing this scope or its evidence claims.

## Existing callers and limits

save_game_with saves the active level, then parked levels, then metadata. Keep
that order and its early error propagation. A parked-save error must still skip
save_game_metadata; the difference is that the original active identity is now
restored before propagation. Do not save metadata anyway to mask the error.

Both direct synchronous saving and autosave call this shared path, including
server-thread/dedicated callers. wait=false remains enqueue/producer semantics,
not a new disk-completion guarantee. In-frame Games normally have no parked
levels and retain their current behavior. Normal shutdown may later destroy the
Game or discard other owners; this small correction does not retain a failed
Game beyond those unchanged caller lifetimes.

close_idle_levels has a SIMILAR early-return restoration gap, but it also removes
successfully closed levels and retires counter trials. It is OUT OF SCOPE here.
Changing that distinct close/teardown lifecycle requires its own bounded contract.
Do not factor both paths together incidentally. Command/tick dimension selection,
player transition ownership, lazy raids, ordinary entity codecs and prior failed
Left/unload/reconnect behavior are unchanged.

## Actual native acceptance surface

New tests live only in levels/parked_save_tests.rs, under cfg(test) and
paired-packages as needed for the qualified Fixture/slot helpers. Reuse the
existing transition_ownership_tests::Fixture/new/game visibility; do not edit
its resources, Options, construction or cleanup. Existing entity_owner_tests::ready,
players and rich may be CALLED unchanged for native two-channel-backed-player
setup. Do not widen or rewrite them. Do not reuse its passive whole snapshot as
an exact post-save oracle: a real save legitimately changes dirty markers, files,
UUID projections, hub synchronization state and HashMap entry addresses.

Run actual resource-backed Game methods in a fresh owned process cwd/HOME/game/
launcher/data/config/cache/temp, session removed, pinned resources required by
expect, serial execution. Obtain parked levels via actual enter_level transitions
before installing obstructions; restore the intended original active dimension.
Keep every Game/SaveDir and all player receivers alive through assertions.
Capture native identities by logical dimension and retained Arc/payload identity,
not the stack address of a World/Parked struct which the native swap moves.

Required meaningful cases:

1. EARLY checked producer failure, for wait=true AND wait=false. Start with one
   real parked level, unique resident blocks/items/metadata in both dimensions,
   and passing actual two-player preflight. Put a nonempty forced set in the
   parked Streaming. Use THAT dimension's actual SaveDir.root and create a
   directory at data/minecraft/chunk_tickets.dat_new. save_forced_chunks is the
   first producer in save_level and its real write_dat call must fail there.
   Call save_parked_levels directly; require the native IO error, original dim,
   original active SaveDir Arc, same complete parked-dimension set and original
   per-dimension World/chunk/entity/slot payload owners. Hub.config.dimension must
   again name the original dimension, and player inventories/body/play/UUIDs
   must remain owned and unchanged. Pending slot retirements stay pending.
   Do not require unavailable path context from a raw fs::write error; inspect
   the native error kind/message actually returned and the installed obstruction.

2. REPAIR only that fresh metadata obstruction. Remove the owned directory, call
   the actual same method again, and require success and restored original dim.
   Wait for each participating native writer only where needed for assertions;
   use NEW independent SaveDir readers rooted at each exact dimension directory
   to prove the distinct block/entity records actually reached disk. Verify the
   native forced-ticket NBT file and nonempty entity readback. An originating
   SaveDir's pending read is NOT disk proof. No whole-epoch durability claim.

3. LATER sticky region/entity writer failure. With one parked dimension containing
   nonempty rich entity data and dirty resident chunks, obstruct its entities
   directory with a file, then call save_parked_levels(true). This must reach the
   real producer/enqueue/flush error, restore the original active dimension and
   keep both native owners and the SAME parked SaveDir Arc alive. Check retained
   pending data versus a NEW independent disk reader and preserve the sticky
   writer error. Removing the obstruction does NOT clear that native error; an
   actual retry which still refuses must again restore the original dimension.
   Do not replace/reset SaveDir or claim a successful repair forbidden by its API.
   No async wait=false disk error is claimed to be returned before it occurs.

4. FULL CALLER propagation. Use the early metadata obstruction through actual
   save_game_with(wait), with the active level otherwise eligible to save. It must
   return the parked error after restoration and still skip metadata publication.
   Seed a valid existing level.dat sentinel beforehand and compare its bytes and
   in-memory Level/player extras; do not infer metadata execution from a message.
   Allow the already-completed active-level save's normal chunk/entity writes.
   Verify both external Guest owners survive; this does NOT fix their omission
   from ordinary common saves or claim they were captured to disk.

5. SUCCESS across THREE real loaded dimensions and the empty-parked no-op. Seed
   distinguishable resident blocks/entities and forced metadata per dimension;
   after success all participating disk readers see their own records and the
   starting active dimension/SaveDir/hub context is restored. Do not assert a
   HashMap iteration order. The no-parked call preserves owners and creates no IO.

For the early failure, snapshot directory entries (including empty directories)
and file bytes before/after: no successful save producer should have run in the
blocked parked level. Preserve normal setup/open files separately. Later/caller
and success cases must instead assert the exact permitted native write footprint;
they cannot require an unchanged disk tree. Keep original sentinels in unrelated
paths and prove that no record is written into the other dimension's directory.

Use rich distinct native dropped stacks with recursively nondefault raw
BundleSelected/component metadata and stable UUID/raw entries where save production
would otherwise assign them. Check actual payload/Arc identity, values, original
World keys and slot keys/pending payloads, not just counts. A native save may clear
accepted dirty markers, spawn pending loaded entities, prune already stale entity
metadata or update existing omen/clock projections: do not label those unchanged
native semantics rollback failures. Prefer settled supported fixtures so the exact
retained-owner comparisons remain meaningful. If a field's private runtime state
cannot be observed within this two-file surface, disclose source/prior-test support
rather than widening existing helpers or claiming an executed toggle.

## Qualification and stop condition

Coordinator owns formatting, appropriate ordinary/resource tests, all three
feature lint configurations, public-tree/normal hook, exact clean-candidate broad
checks and new normal-path Game regressions in both startup modes as applicable.
Serialize Cargo. Obtain a different fresh independent read-only source/resource/
final receipt review. Preserve failed attempts; compilation/listing is not actual
resource execution. Existing qualified tests/codecs/native owner helpers stay intact.

Stop this packet when the concrete failed parked-save restoration boundary passes.
No user decision is needed for this bounded correction. It supplies no global
checkpoint readiness, producer quiescence, all-family/scalar/byte support, retained
shutdown/error owner service, in-frame capture adaptation, machine persistence,
immutable publication/recovery, external paired-client or G1/G2/G3 acceptance.
No public engine push, merge, release or upstream contribution is authorized.
