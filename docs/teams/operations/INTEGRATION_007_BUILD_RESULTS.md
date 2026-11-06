# Candidate007 — source/build gates closed, gameplay open

Frozen sourcef732bd481cfc3d9c4720bc51e4a31e7385e18020, native tree
471136a9f27049a58bac2707c75b18466e94e17c,742 files, source-map SHA-256
0c97fc70d5ce5a8dc81bed1b5ccc21887fbc69628c94e0b0d1a6aff015c70f8a.
Physical source is `.local/integration/create-teams-001`; exact005 source is
preserved detached at `create-teams-005-frozen` with its old maps/binaries/runs.
Version .15/cache18,499 entries, host9c7f9b22/private Mixinsac5d9e7 are unchanged.

The reviewed composition includes water-line2208701, wheel/controller5ed8101/
shared wiringf05d8cc, byte-equal release tooling8180771, and reviewed existing-API
owner VIEW tracking repaire4184ab/dd5c175. Final composition/driver delta review
CLEAR. The only later source edit is equivalent Rustfmt wrapping. No new schema,
host API/pin, world capture or general framework is implied.

## Actual composed results

All rows refer to the same frozen source/map. Original partial/HOLD runs remain
unchanged; no single failed run is relabeled PASS.

| Gate | Actual result and wall seconds | Local receipt |
| --- | --- | --- |
| Selected formatting/distribution |PASS,1.074 /6.891|checks-001|
| Transform |PASS,84 injections/40 modules;111.207 total,95.669 transformation|checks-001/stage|
| Audio-enabled Clippy |PASS,55 warnings,76.698|checks-001/clippy|
| Client test-target compile |PASS,116.674|checks-002/client-test-build|
| Native core/assets test-target compile |PASS,18.694|core-pure-003|
| Fixed pure checks |14 passed,0 failed;0.056 total command seconds|core-pure-003 six test receipts|
| Playable client/control build |PASS,114.617|build-001|
| Private base resource copy |28,176 files/624,369,581 bytes,8.154|restoration-resource-setup-001|
| Headless resource preparation |exit0,6.979; not Create restoration|restoration-resource-preparation-001|
| Real Create failed-receipt restoration |1 passed,0 failed,1.817|restoration-001|

Pure selection is Pump kinetics2, endpoint transfer6, asset model1, wheel clock3,
wheel kinetics1, and changed owner attachment round-trip1. The ignored real test
is `pump_execute_basin_zero_receipt_restores_real_owners`, run alone against the
actual prepared Create Tank/Basin states/callbacks. It supports selected same-turn
restoration on a failed receipt, not interrupted-crash atomicity or game reopen.
Unchanged historical tests are attributed rather than replayed.

## Preserved driver findings and repair

The initial exact wheel filter named the wrong test module; static review caught
it before execution. It was corrected to `kinetics::crushing_wheel_tests`.
Redundant source-path resolution caused roughly55 seconds between commands.
Reviewed equivalent traversal first validated each canonical directory, then
validated each direct child with one non-following stat. Every redirect/special
entry is still rejected, the entire file set/SHA is still checked before and after
every command, and general artifact/resource path checks are unchanged. Observed
gaps fell to roughly15 seconds. No source-hash or test-count check was removed.

Root stopped checks-001 only between commands with no child, retaining successful
receipts. Its repeated cached stage in checks-002 took5.872s; cached Clippy29.523s.
The latter run then correctly remained HOLD when staged assets tests could not
find their source-layout sibling `data/content.json`; it ran no pure tests.

Reviewed driver r5 SHA
e6a66d66b1be5b09e2839d4381d14e518051a88e941cd756f3559019003e3605
compiles core/assets from the existing native workspace, whose root package uses
the same engine source. The exact successful client compile receipt/log are
SHA-bound and reused. Its finite core-and-pure continuation executes all14 fixed
checks, validates executable bytes, and claims only its own result. There was no
client recompilation or production repair for this path error. Original scripts,
failures and review findings remain in the ignored local007 packet.

## Frozen runtime inputs and limits

Game SHA-256:
fb8ec9b36084b3a5d4d77e56ea4dcb7cd5f1b49eac4d52fc2998f50cdc2ce2db.
Control SHA-256:
170e866057d97e73b9231c8084547d4a4a02346238db35901ef64bf3d0d6d449.
Build receipt SHA2ec5726527df08817663fa847df0b2dfdbb783deecc31d961c92a4ebae5074f5.
Compiled test receipt SHAe768343630710065c52ee8abb87a1d231bb96597cc07a49bf285043049cce087.
Restoration prerequisites SHA43de03a356d6deb4d76465730f1c75098c4083e6120d865a518b349c95715396.

The complete owned base copy must stay available because composed resources link
unchanged loader data. Raw resources, local original JAR, binaries, worlds,
generated output and execution streams stay outside Git. These gates establish
build/test readiness only. Actual owner initial/cold peer sync, water-line
gameplay/peer/cold behavior and wheel capture/clock/spill/reopen remain separate
queued milestones. No S6/full-Create parity or public-release pass follows.
