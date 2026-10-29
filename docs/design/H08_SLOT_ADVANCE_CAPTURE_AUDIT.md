# H08 capture after a counter advance audit

READ-ONLY SOURCE ANALYSIS, no implementation release. Current recapture candidate:
engine codex/h08-slot-recapture at acceptedbfa5ced912bed16f955e65824317a0b259c2cb6a
plus six frozen sources in workspace-root .local/h08sr1-source-sha256.json,
manifest SHA25671de0cee4274a84100db3de25594609a0880847af9d9f3314b6174cada5385b7.
At audit time its four primaries/native56each/features passed while later gates
were pending. The same six sources now have full acceptance at clean LOCAL
c05401ecf869ea471c68986ff9b7709567e1c8c0/tree d915d07e85afbdd9a8ab5edaf6cd407eed1812b3.
The separate next proposal has fresh independent design acceptance. No ordinary runtime/G1 or public
publication claim. The bounded author performed native read-only inspection only.

## Current guards reject a real counter step

The smallest useful next cell would recover epoch2 at3/T, invoke the qualified
counter phase atT+1 to reach4, publish epoch3 and recover that whole image freshly.
The current APIs intentionally accept only their3/T cell:

- game/levels.rs761 accounts the slot through slot_epoch_projection. That reaches
  slot_epoch_integration.rs105, where correlate_slot requires counter3. E1 therefore
  rejects the progressed slot before reaching the W1 timestamp check.
- world.rs1151 requires saved.last_update equal the current native World time.
  The nine inherited roots still carryT after the qualified step suppliesT+1.
- slot_epoch_integration.rs437-475 validates inherited sidecars and requires byte/
  value equality with current live projections. Inherited3/T legitimately differs
  from live4/T+1. Those files must become an explicitly validated archived baseline,
  never be relabelled current or silently repaired.

Line anchors identify the frozen candidate; reverify after formatting/commits.
Initial and restored3 callers must retain their existing behavior. A new fixed
progressed variant must account its actual supported slot, then share ALL remaining
family/drop/memory/hook/portal and W1 checks with the same refusal priorities.

## Native region time can stay older than level time

World::advance_counter_trial (world.rs341) updates its domain without dirtying
chunks. The current defining test's continuation is the real qualified counter
phase with the native clock supplied atT+1 (slot_recapture_integration_tests.rs618),
not a complete running Game tick. Ancestor loaded() constructs a clean World
(counter_restore_integration_tests.rs383). save_level (game.rs11395) snapshots only
world.modified; with_entities (game.rs8716) and persist.rs75-104 stamp detached save
copies rather than resident chunk Arcs.

Consequently a coherent progressed image can have Level.Time=T+1 and current4/T+1
sidecars while its unchanged clean region LastUpdate remainsT. A proposal must
retain and prove that exact baseline time and unchanged nine native roots/region
bytes. Do not mark chunks dirty, rewrite extras or run a preparatory save merely
to make their timestamps equal the current clock.

## Two distinct checked record pairs

Before native saving, a progressed profile should validate separately:

1. The inherited canonical counter3/T and strict slot7/revision1/T baseline, with
   actual Items/binding, independent world/target and checked current_time-1.
2. Current counter4/T+1 and slot7/revision1/T+1, requiring identical persistent
   world/target/package/type/state, placement generation, machine/slot high-water
   and slot incarnation. PLAYER3 and slot payload remain unchanged.

Compare all supported fields and canonical representations, not ItemStack Eq's
incomplete component comparison. Overflow, incorrect intervals, stale generations,
changed revisions/payloads and unaccounted owners must refuse before saving.

No codec/factory change appears needed: counter_checkpoint_frame
(world/counter_restore.rs830) and slot_checkpoint_bytes (world/slot_restore.rs1153)
demand current native time and support counter4. Restore factories independently
check target Game time/anchors (counter_restore.rs671, slot_restore.rs1068).

The same six-path structure is plausible: five current production paths plus a
NEW defining progression child. Closed variants can select projection/E1,
previous-tick empty-root observation, capture origin and actual Task dispatch.
Reuse InheritedSlotFiles and pending replacement: retain exact baseline bytes,
compare source and both pending inputs, replace ONLY pending copies with current
records, recheck live/source after native save, then use unchanged publication.
No source-sidecar deletion, new Game field, general profile or normal admission.

The existing native loader helper hard-assertsT at line503. A small actual native
loader in the new child can load the fixedT+1 result without editing old helpers.
Its nine chunks and canonical PLAYER must still come from actual native files.

## Recovery classification and required proof

SlotManifest2 can remain closed and unchanged, but the new private recovery caller
must explicitly classify physical epoch3 corruption and whole older epoch2 at3/T.
Selected corrected-hash semantic4 errors must still refuse before copy. Existing3
callers should refuse a selected4 image, without silently choosing an older3 image.
This is a concrete design choice to specify/review, not a schema expansion excuse.

Minimum proof: actual epoch2->native3/T->counter4/T+1->Task epoch3->fresh owned
recovery; fresh keys/same-time skip/PLAYER3+slot7; exact nine roots and clean-region
preservation; Level.Time/current-frame coherence; unchanged baseline and epochs1/2;
wrong scalar/time/identity/target/head/revision controls; unknown roots/remaining
owners; pending-copy I/O, actual timeout/interruption, whole physical fallback and
selected semantic refusal. Full history/scalars, ordinary running Game admission,
two external peers, animation/performance and G1 remain open.
