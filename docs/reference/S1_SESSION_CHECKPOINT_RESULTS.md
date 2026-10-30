# Native session checkpoint qualification

QUALIFYING, not final acceptance. Source is now a nine-path local engine
candidate on `codex/s1-session-checkpoint`, based on
`ff69d5fcb043500f315fd2e2452add9320ec415a`. No engine publication, ordinary
startup/close/filled-slot activation, external admission, G1 or live S2 claim.

The accepted design is `../design/S1_SESSION_CHECKPOINT_PACKET.md`, exact proposal
47cf3a13, with resident-save clarification fc93a257. The operation checkpoints one
identified native TestTerrain Overworld, one active counter and persistent slot,
and actual native participant owners. It consumes a fresh unpublished Game for
checked recovery. Existing research APIs and their pristine-history guards remain.

## Nine-path correction, not yet runtime qualified

The accepted empty-raid amendment adds paired private helpers in game/raids.rs.
Core SHA-256 `820762e04dc00bbb6ff7f424099e4a8595f081ee366ffaa15395109040db4b1f`,
test source `2705ad3a78b6719d59c8c7015cbcc9f8976390e3f8c40466f3598fd560170d06`,
raids source `719efe1cfd45fd3eb05d48ed6c41eebdfd96fc6b3ae778fdf7e879e94a827994`;
other six paths unchanged. Root formatting and both independent source-delta reviews
cleared these hashes. Validation37509 passed all three feature lints, formatting
and public-tree checks. Build71841 passed with debug test binary
`08b8bb5289edd77eb459ea31a089e78779d2e4d3b070c94111fd5bdbe9a015ae`.
Run89418 finished nonzero:193 ordinary tests and resource0..8 passed. Resource9
passed its integrated-owner three epochs, clean-raid write failure/retry and metadata
controls, then failed the dedicated second checkpoint in101.64s. The inert native
local-player record was not canonical. Remaining six new and twelve retained cases
were not run. The one supporting ordinary test was added while the existing21
checkpoint names remain unchanged. This is not a full current suite pass.

The correction preserves current clean native empty metadata, rejects every named
raid/patrol/siege/omen owner, validates primary and backup records independently,
preserves valid unloaded disk metadata, uses actual native write_dat and restores
verified state without a random draw. The extended real-Game case includes a further
genuine clean tick, raid-file EISDIR failure/retry, third recovery and corrupt native
metadata controls. All nine baselines/prefix artifacts and author source are retained.

An unchanged-source focused reproduction18524 failed identically in103.26s under
fresh persistent storage. Its retained world was verified present. Native record
comparison found both participant records unchanged; only the dedicated inert
record gained warden_spawn_tracker with exact Int elapsed1/warning0/cooldown0.
The fixture already uses dedicated options. Native global ticking loaded and later
saved a player-owned tracker through the absent local player's cache. Pinned
ServerPlayer bytecode confirms that this field belongs to a player. The bounded
tenth-path proposal is `../design/S1_SESSION_CHECKPOINT_LOCAL_TRACKER_AMENDMENT.md`;
it has independent design acceptance at exact proposal4f3e2ed5. Private checkpoint
still precedes source release; no implementation qualification is implied.

Evidence-storage correction: prior /tmp retained paths were logged at failure but
later unavailable after WSL restarted; they are not currently verified world artifacts.
G source/log/receipt archives remain. No root-issued shutdown/restart/cleanup occurred.
The independently reviewed runner now uses fresh isolated directories under
/home/arbiter/.cache/create-horizon/session-tests. Focused mode has a separate receipt
explicitly marked incomplete and cannot establish a full suite pass. Repro18524's
verified fixture is s1sc1-session-resource-9-uhiuzemr/temp/
horizon-transition-411-0-session-two-epochs-retained-411-0/fixture under that parent.

## Prior eight-path candidate and checks

Raw prefix `.local/s1sc1`. Current core SHA-256:
`c69a091f900154f88538357e51aa44faf920bf8c5a34771c17d29b0c26213d7d`.
Test source SHA-256:
`b0ea25ffd50b884ea0d4488613be3f7a86a7d0fe322daa1680b0e60c97fb0c9b`.
The other six paths are frozen in the eight-source manifest. Fresh source reviews
cleared the complete candidate and finite corrections. A separate final reviewer
has provisionally cleared source; final acceptance awaits complete evidence.

Validation56037 passed formatting, public-tree checks and all three feature-lint
configurations: paired runtime/control, paired without runtime, and default-feature
disabled without paired features. Build1880 passed. Frozen debug test binary:
`43d6de400db191cd590e56502cb763a8488e9a3e23212266ec32f4ca89d88f15`.
Library binary remains `82476822a5f34843dc77c3d7890cd9e4273521dae558ff7f758cf90c603ad4e7`.

Run22607 finished nonzero. All 192 ordinary Game tests passed (187 existing plus
five new). Nine new explicit-resource cases passed before the tenth failed. The
remaining six new and twelve retained resource cases were not run. Completed groups:

| New resource group | Result | Seconds |
| --- | --- | --- |
| Body-only observations, actual recovery and failed second install | PASS | 51.19 |
| Fourteen complete-image/native corruption and no-fallback controls | PASS | 175.44 |
| Copy/manifest/promotion/post-rename interruptions | PASS | 32.53 |
| Corrupt off-resident source header before native saving | PASS | 26.77 |
| Dedicated native UUID collision refusal | PASS | 26.70 |
| Empty dedicated roster and inert local UUID preservation | PASS | 37.73 |
| Every member and trusted recovery context mandatory | PASS | 26.49 |
| Real filesystem writer error and prior image preservation | PASS | 26.63 |
| Late actual Finish storage handoff closure | PASS | 26.70 |
| Two epochs, variable shapes and both roster roles | FAIL: second checkpoint refuses native raids.dat | 38.99 |

Remaining runtime groups, optimized type check, clean broad checks and final
commit/review are outstanding. This table is partial old-candidate evidence, not a
suite pass. The failed whole world is retained under the run's unique temporary
fixture. The live empty raid counter advances every tick but native clean saving
writes it only periodically. Merely whitelisting its disk file could lose current
metadata. The bounded ninth-path amendment has independent design acceptance at exact
proposal1c06a492 and is recorded in
`../design/S1_SESSION_CHECKPOINT_EMPTY_RAID_AMENDMENT.md`; source release followed
private checkpoint efe6c078. Design acceptance is not implementation qualification.

## Failures and corrections retained

Every correction preserved all eight source files and all prefix artifacts first.
The first callback-fixture failure11182 lost its world through the old FixtureDrop;
source and logs remain, and no retained-world claim is made for that attempt.
Subsequent fixtures preserve their whole owned root after Game/runtime destruction
on unexpected panic. No runtime ownership or pending counts are repaired for tests.

- Actual native Finish work in range only marks modified; the fixture now uses a
  real far Finish, returned via public streaming wait, whose out-of-range storage
  callback and nonempty physical region are required. Unloaded snapshots are saved
  through actual native APIs. The earlier H08 Game's zero-callback evidence is not
  relabeled as this new nonzero control.
- Guest view distance0 violated its native minimum2; the fixture argument is now2.
- The closed block-palette table expected legacy Name/Properties. It now strictly
  uses native id/properties, with an actual writer-emitted compound positive and
  legacy/wrong-type/unknown-property negatives. Unknown fields still refuse.
- Native decode does not restore lighting by itself. The validator now invokes the
  existing public load_light step, as SaveDir::load does, before unchanged strict
  section comparisons. Actual lit and archived unlit records are checked, including
  flags, exact saved layers and every block/light/biome cell.
- Run75923 published its checkpoint, then fresh recovery refused an unavailable
  advancement tree. finish_open_world resets progress and starts a new native
  prefetch; recovery now completes advancement_tree after that reset and before
  participant JSON restoration/re-projection. No progress or owner is fabricated.

Earlier compile-only corrections used a private strict CounterTarget field adapter,
two mechanical lint fixes, and a feature-correct running-package helper. No public
domain traits, warning suppression or unsupported runtime success were introduced.

## Inputs and scope limits

Rust1.99.1, Minecraft26.3/protocol777/DATA_VERSION5023. Pinned resources manifest
`f35140182f95862e5330fabe4c6a449805f2d83be94dce2f01e770a2ede68ce2`.
The five-file prebuilt counter pair is reused read-only; receipt
`32c83f6b1fbffe44696f42dd3617b7a6e6836d18a46303f0d21189dcacb940b4`.
Its old83649 guest build is explicitly retained, not described as the current
engine build. Authored Rust tests perform no nested guest/compiler builds.

Runtime worlds, process homes, data/config/cache/temp and launcher paths are fresh.
Prism, playable saves, accounts, reference snapshots and global Java settings are
untouched. Python proof checks use isolated mode with explicit optimization refusal.

These are headless actual-Game/native I/O checks, not connected-client admission or
both-mode GUI qualification. Ordinary lifecycle, retained failure owners, external
negotiation/replication and committed retained presentation remain S1 work. The
follow-on lifecycle audit is planning only. No broad journal, global codec rewrite,
save compatibility promise, relaxed conservation or reduced S1 has been adopted.
