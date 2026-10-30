# Native empty-raid metadata amendment

DESIGN ACCEPTED by fresh independent native read-only Sol High review at exact
proposal SHA2561c06a492c754a80e631898516697bc4224cd928cc38accba8ac0a43b9c45b555.
The exact proposed text remains .local/s1sc1-empty-raid-proposal1.md. Private
checkpoint, exact source/tool reviews and execution qualification still precede
acceptance of an implementation. Native write_dat rotates files nontransactionally;
errors must remain visible with in-memory ownership retained, without disk rollback
claims. No ninth-path source author has been released yet.

Narrow amendment to accepted checkpoint
proposal47cf3a13 and resident-save clarificationfc93a257. Current candidate is the
eight-path worktree on baseff69d5f, corec69a091f and testsb0ea25ff. Run22607 passed
192 ordinary tests and new resource0..8, then resource9's second checkpoint refused
dimensions/minecraft/overworld/data/minecraft/raids.dat after a real Game tick.
All eight sources and all prefix artifacts were preserved before any correction in
.local/s1sc1-native-raids-sidecar-attempt1; the whole failed world is retained.

## Concrete native dependency

Native tick_raids runs with mob spawning disabled. Raids::default is dirty and its
tick counter increments every tick; clean empty data is ordinarily written only
every200 ticks (raid.rs772-781,843-901). The first native save emits current-version
saved data whose data compound contains next_id:Int and tick:Int. Empty native
encoding omits raids entirely (raid.rs909-934). Merely allowing its filename would
lose current clean metadata. Lazy load_raids consumes world.random.next_long even
for an empty map (game/raids.rs271-288); recovery must not use that lazy load.

Add exactly one ninth existing source path, game/raids.rs, to the eight-path packet.
Only paired-packages-gated private checkpoint helpers and their bounded tests are
released there. No changes to library raid/save/streaming codecs, ordinary raid
save/tick behavior, public APIs, native file schema or default gameplay. The two
existing checkpoint files integrate the helpers and qualification; other six paths
remain frozen. Root proof tools must explicitly adopt the nine-file manifest and
any named supporting test before execution. No ordinary lifecycle caller is added.

## Closed live and native profile

Inspect actual private Raiding state. Require initial Overworld, loaded either None
or Some(Overworld), both native raid map and native ID order empty, zero patrol
countdown, every siege field at its inert default (including spot/countdown/count),
no omen position, no homes or bar bookkeeping. Immutable resource-derived cached
attributes are not an owner. Omen-loaded with no position may be treated only as
an inert cache; player tags still reject unsupported omen fields. Unloaded live
Raids must retain native default next_id1/tick0; never substitute those defaults
for a record already present on disk. Loaded empty state supports positive Int
next_id and nonnegative Int tick, preserving both exactly, without a fixed-history
or value cap beyond their native types and the existing nonnegative world clock.

Validate the primary and every backup independently as exact current-version NBT:
root has precisely DataVersion:Int(current) and data:Compound; data has precisely
next_id:Int(positive) and tick:Int(nonnegative). Reject unknown keys, wrong types,
missing fields, explicit raids lists (even empty), active raids and coercions.
After strict validation, native from_nbt(data,0) plus exact to_nbt comparison may
confirm canonical encoding. Backups are historical and may have different counters;
they are not used as a fallback or substituted for the primary current record.

## Capture, physical verification and recovery

Derive a checked effective native projection under the same exclusive Game borrow:
loaded current state supplies its actual native Tag; unloaded state preserves the
strictly checked primary disk Tag when present, otherwise absence. Validate disk
metadata before a native write could hide unsupported input. Include this effective
projection in the before/after semantic snapshots and owner census. Do not create a
default sidecar just because unloaded memory has default Raids values.

After ordinary native save and alongside current-resident submissions, write the
actual loaded empty current projection through existing checked save::write_dat
regardless of its ordinary dirty flag. Never fake a dirty marker, reset a timer,
clear an owner or invent an empty state. Preserve dirty/ownership on failure; any
saved-state bookkeeping changes follow successful native I/O only. Preserve _old
history and native errors. This synchronous write is included before the same
second fence/deadline and complete physical copy. Verify the copied primary exactly
matches the actual effective projection, including current tick and next_id.

Image verification returns the strictly verified optional empty-raid metadata in
its private checked image. After finish_open_world, install that verified state
directly into the still-unpublished consumed candidate through a private helper.
Use native decoding with inert seed0 only after the closed validation; no active
raid requires a random seed. Set current-dimension loaded bookkeeping consistently
when a record exists; preserve absent/unloaded state when none exists. Do not call
load_raids, tick the simulation, draw from world.random, fabricate a Guest or clear
the live source. Final installed projection and native copied primary must equal
the selected image before returning the Game. Any failure returns no Game.

## Decisive checks and release order

- Preserve all earlier successes/failures as exact old-source evidence, not a new
  suite pass. Full accepted candidate qualification remains outstanding.
- Existing actual two-epoch resource case must succeed for both roster roles and
  different resident shapes with its real clean-chunk tick and conservation.
- Extend actual qualification with a further real tick while empty raids are clean,
  prove the current native raid timer advances without dirty, checkpoint/recover
  and compare exact metadata. Preserve next_id, clock, inventory totals and fresh
  authority. No manual timer bump substitutes for this case.
- One bounded supporting ordinary test in game/raids.rs may exercise closed native
  shapes and actual private owner/counter guards, including map/order, patrol,
  siege and omen bookkeeping refusals. Register its exact name in root's expected
  suite and keep the existing21 checkpoint test names unchanged.
- Positive recovery must demonstrate no world.random draw from direct metadata
  installation; current native field types and backups remain strictly checked.
- Real native write failure must refuse publication, retain source state and prior
  complete image, without repairing counters or removing failed evidence.

Fresh independent native read-only Sol High design acceptance of this exact
amendment and a private checkpoint precede ninth-path author release. Exact-source
and proof-tool delta reviews precede formatting-qualified execution. Different final
review, optimized type check and appropriate clean broad checks remain required.

This authorizes no general raid support, arbitrary native saved data, extra
dimensions, generic journal, save compatibility change, relaxed conservation,
ordinary filled-slot startup/close, G1 or public release. Existing unsupported-owner
refusals and the owner's first-experiment scope remain intact. No new owner policy
decision is proposed; the amendment repairs a native dependency of the already
promised real-tick checkpoint behavior.
