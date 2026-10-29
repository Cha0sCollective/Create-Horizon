# H08 parked-save restoration results

ACCEPTED after different fresh independent read-only source, resource, delta and
final evidence reviews. This bounded result does not advance G1.
The bounded contract is `docs/design/H08_PARKED_SAVE_RESTORATION_PACKET.md`.
Engine base is local `3ded3a2f8b7e012eae08dfaf06e6985686e95912`.
Candidate local commit is `aa170bc4fd3214790a2363732d0c7cdd57ba6451`, tree
`1dec4b02fe540af63c4453ff2ff7e1c221a1056f`, on codex/h08-parked-save-restoration.
Normal public-tree commit hook passes; the engine checkout is clean.

## Behavior and scope

The candidate preserves the parked-dimension iteration policy, stops at the first
enter/save error, and always attempts the native return to the starting dimension
before returning its result. The original error takes precedence. A secondary
restoration failure is logged with requested and actual dimensions. Existing
native owners, codecs, metadata ordering and writer error lifetime are unchanged.
Only save_parked_levels and a gated new test module declaration change in existing
source; close_idle_levels is outside this packet.

The six resource cases cover early ticket-write failure and filesystem repair,
late entity-writer failure and retry, full-caller metadata skipping, three-dimension
save routing, empty no-op, and stopping before later parked producers. Both wait
modes are exercised where meaningful. Native writer completion precedes independent
disk readback; wait=false alone does not promise disk completion. Successful routing
uses the active save producer followed by parked producers, while the failed full
caller uses actual save_game_with. Finite exact-path checks preserve unrelated file
bytes, directory entries and sentinels.

## Preserved qualification attempts

- `.local/h08a16-build-attempt1`: initial formatted source compiled in48.19s;
  no resource case ran. Independent review corrected a false expectation that an
  entity retry could not write successfully while the writer retained an error.
- `.local/h08a16-build-attempt2`: that test-only correction compiled in23.33s;
  no resource case ran. Independent review requested complete permitted-write
  footprint checks instead of selected readbacks/absences alone.
- `.local/h08a16-resource-attempt1`: the footprint candidate compiled in26.56s.
  The first actual case failed in13.22s during setup, before production saving:
  the fixture already had native SaveTables. The runner stopped, leaving the other
  cases and native validation unexecuted. The correction retains native tables
  and permits their exact POI file/parent paths rather than changing native setup.
- `.local/h08a16-clippy-attempt1`: the POI correction passed all six resource
  cases, both9-case lib configurations and177ordinary bin tests (59ignored).
  The first feature lint then rejected two unnecessary PathBuf clones in new
  tests. Replace only those two one-element slices with slice::from_ref; production
  is unchanged. Passing native tests remain valid and are reused explicitly.

Original sources, compiler receipts and runtime failure logs remain preserved.
None of these attempts is relabelled as a successful resource run.

The POI-corrected source passed independent delta review and compiled in23.48s.
All SIX actual resource cases passed, each in a separate serial exact-name
process with fresh cwd/home/data/config/cache/temp. Their durations sum to97.87s.
The frozen test binary is
`/home/arbiter/.cache/create-horizon/bin/h08a16-resource-tests-igyp8r01/horizon-client-tests`,
SHA256 `3bf59d29cdc2227b4fb730eb8f05e846c0ececd0a1a5ed8c6b122a167e9510a4`.
That run's source hashes are levels.rs
`7215b3c0e28f6722ff015d989fc0275208c1692fd800f49311ab1c9029db288a`
and levels/parked_save_tests.rs
`efbb32bd865c5ab45a5d8a3139b878791212d597172ebd7a666a51265a2fca15`.
Source and binary hashes are checked before/after; the prepared-resource manifest
is checked before execution only. Receipts and all six individual logs are at
workspace-root `.local/h08a16-resource-tests-result.json` and
`.local/h08a16-resource-case-00.log` through `-05.log`.
The archived copy under clippy-attempt1 preserves these original receipts; current
top-level logs will describe the final mechanical correction's rerun separately.

The two-slice mechanical correction passed independent review, compiled in25.60s,
and all SIX cases passed again (98.65s summed case durations). Final test binary
SHA256 `b4b91b19312e456bd49ed27c4b4f71b62ddca022a9ef2d7ed846e04eb228379f`
is frozen at
`/home/arbiter/.cache/create-horizon/bin/h08a16-resource-tests-57i2u6y4/horizon-client-tests`.
Final test source SHA256 is
`785886eaf18961410710a790ab4e547ee3a1723219a8610e58a190f335644d30`;
production remains the same levels.rs hash above. Current resource receipts/logs
describe this exact final binary. The resume script reuses the valid ordinary
native tests and repeats formatting, all three lints and the public-tree check.
Those checks now pass:177ordinary bin tests/59ignored (33.76s),9lib tests in
each paired feature configuration, and3feature lints (39.32/44.37/50.44s).
The clean candidate's fast Game build passes in2m41s; new frozen Game SHA256 is
`52ed94ea1e7575d88b8abf26c15409c959d81d989014a675d8728eacf19a268c`.
Both full counter Game modes now pass:

| Mode | Original run ID | Checks | Final committed counter |
| --- | --- | --- | --- |
| Integrated | aa962658-0466-4853-9baf-bfe08059e478 | 368 | 71 |
| In-frame | 8cb6d72b-fe2d-461a-8452-c02e368c0ef3 | 353 | 27 |

All four normal initial/restart exits are0 and all four intended changed/missing
package refusals are1; every cleanup_error is null. Original proofs remain under
workspace-root .local/h02d/game-runs; .local/h08a16-game-run-index.json is a summary,
not replacement evidence. Every Game child removes the launcher session and uses
fresh owned paths. Qualified counter entrypoints and their parents are unchanged.
Counter values are timing-dependent functional evidence, not performance results.
Exact-commit broad checks now pass:369Python tests,1358client tests and zero doc
tests, formatting/lint/public-tree, unchanged Cargo.lock,8m45s total. The763-line
scope warning remains; parity is explicitly skipped by --quick and untouched
content/theme/launcher checks are not rerun. Final broad log SHA256:
`1d8b74a12b8912a1c079fa017e47b29426ce866c955cafced971e79df1696690`.
Both .local/h08a16-run-status.json statuses and outer qualification exit are0.
A DIFFERENT fresh independent final reviewer verified exact source/clean commit,
raw resource/native/Game artifacts, current restoration feedback and final broad
receipts with no blocking findings. No reviewer executed tests or changed source.

## Qualification limits

Current synchronous invariants retain the original dimension in Levels.parked,
so restoring it cannot naturally reach open_level failure. Defensive secondary
error handling is source-reviewed, not an executed double-error proof. Slot checks
observe retained SlotKey identity and snapshot payload values; private slot payload
allocation addresses rely on source/prior defining tests. Other observed native
SaveDir/chunk/item component addresses are checked directly where accessible.

No global checkpoint, historical owner-loss repair, machine persistence, immutable
publication/recovery, external-client, performance or G1/G2/G3 claim follows.
Engine publication, merge and release remain unauthorized.
