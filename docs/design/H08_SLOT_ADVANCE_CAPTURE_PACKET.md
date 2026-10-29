# H08 capture after one counter advance

DESIGN ACCEPTED after fresh independent read-only Sol High review of proposal
SHA2566317a8edcec410badbd9b70ded2760d9203c048ccd5cced107d02f7af6ef6255
and auditb3a7661f550f2adc92624fcfee4465fe712d25292a9dd125c3402bbe7f2364ef.
The prerequisite H08_SLOT_RECAPTURE_PACKET now has different final acceptance.
Accepted clean input: c05401ecf869ea471c68986ff9b7709567e1c8c0/tree
d915d07e85afbdd9a8ab5edaf6cd407eed1812b3, matching the reviewed frozen input.
Coordinator releases ONLY the six paths below on a fresh codex/h08-slot-advance
branch after recording this private checkpoint. Exact-source review precedes
execution; source/runtime/final acceptance remains open.
Original reviewed input: exact six-file recapture candidate at acceptedbfa5ced plus manifest
71de0cee4274a84100db3de25594609a0880847af9d9f3314b6174cada5385b7. The implementation
input is the accepted clean commit/tree above.
Source facts: H08_SLOT_ADVANCE_CAPTURE_AUDIT.md. No ordinary runtime/history/full
scalar, S1/G1 or publication acceptance is implied.

## Objective and fixed boundary

Recover the qualified epoch2 at3/T, supply the native clockT+1 and execute the real
qualified counter phase once, then publish epoch3 at4/T+1 through the actual Task.
Recover that complete image freshly, with PLAYER3 and slot7/revision1 unchanged.
This is one counter phase, not a complete running Game tick or natural-history
claim. Do not add arbitrary counter values, item changes, revisions or clocks.

Six paths relative to crates/horizon-client/src:

1. world.rs: closed previous-tick empty-root observation, sharing the full existing
   traversal; defining tests and existing current-time/ordinary APIs preserved.
2. game/levels.rs: fixed progressed E1 projection selection and World observation,
   sharing ALL remaining checks and priorities.
3. game/levels/capture_integration.rs: explicit private progressed capture/recovery
   profile selection, using existing material/sidecar/publication mechanisms.
4. game/levels/slot_epoch_integration.rs: closed baseline/current correlation,
   progressed entry and recovery disposition, new test child declaration.
5. NEW game/levels/slot_advance_capture_tests.rs: actual native fixture proof.
6. game/integrated/capture_completion_tests.rs: fixed actual-Task dispatch/timeout.

No codec/factory/save/chunk representation, journal, registry, new Game field or
old test-body changes. Use existing native read APIs for the freshT+1 loader and
any pre-copy metadata checks. Report a concrete visibility/path blocker before
widening scope. Root owns formatting, builds/tests/integration/commits. Bounded
author may own these six paths only after release; reviewers read only.

## Closed phase selection and shared ownership checks

Retain initial/restored3 capture behavior and conservative ordinary/parked World
observation. Add a distinct fixed progressed route, not generic boolean/callback
admission. Account the progressed slot by validating its current4 projection, then
run the SAME complete E1 remainder: entities/memory/families/drops/loaded rows,
hook/portal/maps/authority/guest/duplicate-dimension checks. Never catch the first
slot error and bypass the rest. Share the entire W1 remainder and priority too.

Require the same exact nine resident native saved columns, supported binding,
independent world/host target and actual Items. Preserve the three exact empty
root shapes, all empty owner/order/tick/postprocessing vectors and every unknown/
extra/mistyped/nonempty refusal. Their last_update must be exactly the checked
previous native time, not an arbitrary older time. Use checked arithmetic; overflow
and inconsistent clocks refuse. Current-time root and ordinary APIs are unchanged.

The counter phase does not dirty chunks. Require the working native world to
retain the qualified clean-column condition and prove exact nine-root/region-byte
preservation. Do not dirty chunks, rewrite resident extras, clear owners or perform
a preparatory save to change LastUpdate. Native save may update Level.Time while
clean region LastUpdate stays at the baseline time.

## Baseline and current pairs are different

Before native saving, validate BOTH inherited regular bounded sidecars as a strict
canonical baseline: counter3/T and slot7/revision1/T. Validate current live records
separately: counter4/T+1 and slot7/revision1/T+1, with current last_advanced=T+1.
Require current_time.checked_sub(1)==baseline time, and unchanged world/dimension/
position/package/type/state/schema, placement generation, machine/slot high-water,
slot incarnation and supported full payload/revision. PLAYER3 stays unchanged.
Retain source baseline bytes exactly. Do not relabel their times, normalize fields
or use incomplete ItemStack Eq to hide component differences.

E1/W1, PLAYER, streamer, diagnostics and frame correlations remain checked before
native work. After native save, require current live projections unchanged and
source baseline files identical. Reuse InheritedSlotFiles and pending replacement:
compare both copied files with the validated baseline before replacing either,
write ONLY pending copies with current4/T+1 records, recheck source and then use
existing overlay/sync/manifest/rename/acknowledgement. Preserve original source,
epoch1/2 and rejected owned input. Partial pending files are never success or a
claim of global rollback. Successful native-save writes use the existing explicit
working-world footprint; clean region files should remain byte-identical.

## Closed recovery and explicit older-image disposition

Keep SlotManifest2 closed and unchanged if sufficient. Add an explicit private
progressed recovery caller; existing3 callers must refuse a selected coherent4
image BEFORE copy and never silently choose an older3 image. Physical corruption
may use the existing whole-older-complete fallback, but its outcome is explicit:

- Selected coherent epoch3/current4 returns a typed progressed4 disposition.
- A genuine physical failure of the newer image may select whole epoch2/baseline3;
  return a distinct typed baseline3 fallback disposition, including selected epoch.
- Corrected-hash selected semantic/profile errors refuse before copy. Never mix a
  baseline member into a progressed image, downgrade a caller silently or turn a
  selected semantic error into physical fallback.

Specify scalar/profile classification in the existing rank/select flow so known
slot3/4 errors remain nonfallback. Validate Level.Time and all current frame/V1
diagnostic correlations, independent world/target and supported native metadata
for the selected disposition before copying. Current4 can legitimately pair
Level.Time=T+1 with clean region LastUpdate=T; it cannot excuse other timestamp or
owner differences. Use native readers, not new wire parsers or unchecked constructors.
If existing read APIs cannot establish this within six paths, report that concrete
design blocker rather than weaken pre-copy checks.

The new defining loader may load the fixedT+1 native result without changing the
ancestor loaded() helper'sT assertion. Read actual nine chunks and canonical PLAYER.
Install both decoded owners sequentially into a fresh private Game; retain rejected
payload/partial diagnostics on failure. Report success only after both installs.
Prove fresh keys; atT+1 skip duplicate advancement, then atT+2 advance to5 AFTER
the final capture. A baseline fallback resumes at3/T with its own coherent clock.
Alternative histories are not summed inventories. No capture5/general scalar claim.

## Required validation and stop conditions

Freeze exact names at source review; four primary groups plus at most one explicit
bounded process child, never extra primary credit:

- Actual epoch1/2 chain, native3/T, qualified counter4/T+1, Task epoch3 and fresh
  actual recovery/disposition; keys, PLAYER3/slot7, final same-time skip/continuation.
- Exact baseline/current/root/clean-region observations and all malformed scalar,
  time, identity, target, generation/head/revision/payload/owner controls, with full
  value/allocation/clock/RNG/disk preservation and old profiles still conservative.
- Missing/malformed/mismatched baseline and copied-input/overlay I/O failures before
  publication, source/epochs1/2 unchanged; no sidecar repair or fake native success.
- Actual ten-second observer timeout/definite completion and real interruption,
  whole epoch3 physical corruption yielding typed whole baseline3 fallback, and
  corrected-hash selected semantic4 refusal without copying or mixing. Test old3
  caller versus selected4 and all new caller/disposition boundaries explicitly.

Retain recapture4, slot6, counter6/capture6/entity8/W1seven, ordinary180, native56
names EACH mods+paired and paired-only plus new defining tests. Three feature
lints, fmt, exact scope, public-tree, fresh ordinary BOTH-mode GUI and broad native
clone qualification remain required for production World/levels changes. Root
serializes ALL Cargo/tests and preserves failed attempts under a verified-unused
prefix. Fresh exact-source review precedes execution; different final acceptance
follows all gates. No public push/PR/release.

Stop for an unexpected actual shape, unsupported clean-region behavior, concrete
path/visibility gap, inability to preserve/reject ownership or a new product
decision. Investigate source/fixture mismatches without weakening guards. No owner
decision is currently needed to review this bounded proposal.
