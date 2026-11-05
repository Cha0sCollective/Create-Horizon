# Native S6 — coal-heating source checkpoint

Historical source/build checkpoint at5ac874f. The owner subsequently selected
Java's full-cap coal consumption, and gameplay completion review CLEAR is
recorded in `NATIVE_S6_COAL_HEATING_RESULTS.md`. The pending state below describes
this earlier checkpoint, not the current accepted increment.

Candidate only; heating gameplay and S6 remain OPEN. Branch
`codex/native-s6-coal-heating`, base accepted/private-synced host refresh
`792d7a7867b474935440d2129f705127539fa8bd`. Native/content `.7`, asset schema11,
unchanged host9c7f9b22/Mixinse8cd0aa3. Packet
`docs/design/NATIVE_S6_COAL_HEATING_PACKET.md`; source/provenance
`native/UPSTREAM.md`. Accepted `.6` branches/maps/binaries/saves are preserved.

Fresh independent read-only Sol High source/effects and repair-delta reviews
CLEAR. Upstream236c87a8 supplies the burner simulation/visual core, presentation,
state-aware factory, server-derived fuel-data sync/protocol and state-aware loot.
The selected adapter accepts only plain coal and supported normal saved state;
special/creative/cosmetic/capture/ignition actions are absent. Existing source
field names, authoritative furnace fuel resolution and server-owned inventory
consumption remain. Minimal particle math/clock selections avoid importing the
unrelated particle/fan framework. No host changes or new private save/effects
framework were required.

Exact heated recipe copper1 + zinc1 -> brass2 is admitted beside the unchanged
exact dry recipe. Existing Basin heat semantics accept FADING as heated and
refuse NONE/SMOULDERING; no fluid/filter/spout/crafted-progression claim follows.
Empty cages create no heater entity; occupied placement creates blaze_heater.
Burner is passive rather than a kinetic node. Presentation refuses unsupported
entities before particle/animation rows and model submission. Original art still
comes only from the pinned player-supplied6.0.10 JAR; committed fallback JSON is
authored cuboid geometry with vanilla texture identifiers.

## Actual source checks

- Formatting and diff check PASS. All60 injections/29 modules resolve using the
  accepted host pair. Four version manifests agree at0.5.0-ch4os.7.
- Seven imported burner/visual-core tests PASS. They include retained source
  special/creative and fallback-renderer cases; those tests are not live
  qualification of those deferred capabilities.
- All15 resource-import tests PASS after repairing a stale synthetic fixture's
  already-required piston-head blockstate and adding the two scroll fixtures.
- Five actual staged client-library tests PASS: two selected factory/plain-state/
  coal-boundary cases and three upstream fuel-sync protocol cases for complete
  snapshot/reload/generation and malformed/partial/duplicate/limit rejection.
- One actual staged recipe-boundary test PASS: exact brass and unchanged dry
  admission, altered/foreign recipe refusal and source FADING heat predicate.
- Clippy PASS with35 nonfatal warnings; locked binary build PASS (126.81s,
  compiler8warnings). The additional unused capture-query helper belongs to the
  intact upstream sync dependency; capture is not installed or qualified.

Total28 executed focused tests; zero-test executable/doc-test targets are not
counted. No historical full-stage battery was replayed. Raw commands/receipts and
preserved failures: `.local/native-s6-coal-heating-01` outside Git.

Frozen candidate source map
`341f7a89baf048c272c287e0868344e1f8cb49f85996d526a8b629b8facf1212`,607 native
files. Embedded content406 ->422 entries:16 selected additions, language files,
creative tab and embedded version changed;402 other baseline entries unchanged.
Every embedded entry agrees with its declared source file. Copper/zinc tag
dependencies already existed and remain unchanged.

Held candidate game
`70b1bfd5a66a7c2970fa73c66897fb1c9eb61b61d85228372926915d8a1c9672`, control
`e1458e7a4b5d8cfe1f80198b2b5defd6abec5ed74bc4277a332859099fe4c0bb`;
`candidate-built-inputs.json` records their isolated Linux cache directory.
All607 source hashes were verified after build, and the accepted `.6` held game
and control hashes still match their prior input map. No game is running.

## Repairs and remaining decision

Review found the now-reachable generic sneak-wrench pickup would turn an empty
cage into a filled burner. The candidate selects upstream's actual state-aware
loot path before generic pickup; drops are given only after successful removal,
and fresh filled items retain no old fuel storage. Empty/occupied pickup still
needs actual game observation in the upcoming milestone.

The initial importer attempt failed on its stale piston-head fixture; its raw
7PASS/8FAIL log remains. The first adapter compilation failed because the new
guard used as_str on JavaString. Exact allocation-free UTF-16 key comparison
repairs that binding while continuing to refuse foreign keys and unpaired
surrogates; fresh delta review and all five actual adapter/sync tests pass.
Neither failed attempt is relabeled as a pass.

Owner input is still required: retain Java's consumption of coal at the full
10,000-tick buffer, adding no time, or refuse ineffective fuel. The reused core
currently contains reference behavior, and its inherited cap test demonstrates
it; that is not adoption of the new gameplay policy. The prior millstone-loss
decision is not generalized. After the choice, finalize the selected adapter and
run one composed integrated/dedicated milestone for ordinary placement/fueling,
heat-required mixing/refusal, actual peer visuals, item accounting, pickup and
normal cold saved-fuel/input reopen. No heating game has been launched for this
candidate. No public publication, cleanup, playable-save or launcher edit follows.
