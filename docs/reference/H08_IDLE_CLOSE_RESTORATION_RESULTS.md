# H08 idle-close restoration results

ACCEPTED after different fresh independent design, source/resource and final
evidence reviews. Native/feature, both Game modes and broad qualification pass.
Contract: `docs/design/H08_IDLE_CLOSE_RESTORATION_PACKET.md`.
Qualified engine base: `aa170bc4fd3214790a2363732d0c7cdd57ba6451`.
Candidate branch: codex/h08-idle-close-restoration.
Clean local candidate commit is b41427c6b20d0a7458a3de730dc532d3a68d4081,
tree6142ddc068e22801d6150277e65eadedf6605f8d. Normal public-tree commit hook passes.

## Concrete boundary

The candidate preserves native occupancy, idle counters and captured close order.
Every completed close still saves, returns to the starting dimension, retires its
counter, then removes that level. The loop stops at its first error; a final native
return restores the starting dimension before propagating that error. Earlier
completed closes remain closed, while failed/unprocessed owners stay retained.
Secondary restoration failure is diagnosed without replacing the primary error.
Only close_idle_levels and a gated new child declaration change in existing code.
All other qualified bodies/helpers/tests, including save_parked_levels, are intact.

## Native cases and identity evidence

The formatted two-file source passes different fresh independent review. Source
SHA256 values: levels.rs
`8f42ebe09718c04090fab4d71204dcc3aecb13dbbf85000688e0bc98797b848e`
and levels/idle_close_tests.rs
`a87a233667f4a634267c6c6fd3b64e6f3902090ce0b667de489fd152788b553e`.
The first --no-run build succeeds in49.99s; frozen test binary is
`/home/arbiter/.cache/create-horizon/bin/h08a17-resource-tests-u3gla0g9/horizon-client-tests`,
SHA256 `ba639deb49f48162ed8a38c826b026a27af6b40e5303534a94ed74743b86500c`.

All SIX ignored actual Game cases pass, separately and serially, with fresh owned
cwd/home/data/config/cache/temp. Summed case duration is93.29s. Cases cover early
empty-forced ticket fault with pending-slot values, real300-call repair/retry,
sticky entity-writer failure/retry, second-level failure after a completed first
close, first failure with later producer/counter untouched, and native guest/
forced/threshold/Overworld eligibility. Disk reads use fresh independent readers
after native completion and a finite exact file/POI write whitelist. Repair does
not reset a sticky writer error; successful retry writes and retained errors are
checked separately. Immediate post-repair no-op is not counted as a close.

Actual counter allocation tags record returned scope/dimension/instance keys.
NEW native idle-level-close receipts bind19allocations and4completed retirements:
per-case counts0/2/0/1/0/1. Successful keys are Nether instances2and4 in case01,
End instance3 in case03, and Nether instance2 in case05 (each process has its own
native allocator context; these values were observed, not hardcoded inputs).
The partial-close receipt identifies an allocated owner; its captured ordinal is
also established by passing in-test removed-first/unchanged-failed counter checks.
The receipt alone is not independent ordinal evidence.

The coordinator's first receipt checker failed AFTER all six tests passed: it
accessed the live record field, but native take_counter_trial returns record=null
and keeps the Explicit resident record under last_retirement. The original checker
and successful resource receipts remain at workspace-root
`.local/h08a17-receipt-attempt1`. Correct only that task-local parser, verify the
retired snapshot shape and join its actual instance to allocation tags. This
correction passes using the SAME six valid original runs; candidate source and
test binary are unchanged. No failed Game case is relabelled and no unnecessary
resource rerun is claimed. Independent read-only review accepts the raw resource
logs and corrected counter proof, including the partial-close evidence limit.

Raw original evidence: workspace-root .local/h08a17-resource-tests-result.json,
case00..05 logs, build/source manifests, and h08a17-counter-retirement-proof.json.
The runner checks binary/source before and after; prepared-resource manifest
verification is pre-run only. Launcher session is removed in every child.

## Remaining qualification and limits

Ordinary bin tests pass177/65ignored in32.77s. All three feature lints pass in
39.07/44.28/49.55s, formatting/public-tree/hook pass. A new frozen both-mode full
Game proof now passes. Fast build completes in1m09s and frozen Game SHA256 is
`8e33586ad5bfe57780229de6e71a8f8f2238ade47e1bb03152883fccabdc5270`.

| Mode | Original run ID | Checks | Final committed counter |
| --- | --- | --- | --- |
| Integrated | e504307e-a29c-409f-a5f4-160ebe59d768 | 368 | 70 |
| In-frame | 869d9d9d-2a65-4fd9-861d-004dfb2db34a | 353 | 28 |

Four normal initial/restart exits are0 and four intended changed/missing-package
refusals are1, with all cleanup_error fields null. Original Game environments bind
the exact candidate/tree/digest; immutable entrypoints and qualified parents are
unchanged. Current coordinate-specific restoration command feedback is verified,
with no case/restore error. Original .local/h02d/game-runs data is authoritative;
.local/h08a17-game-run-index.json is a summary only. Counters are timing-dependent
functional evidence, not performance results. Exact-commit broad checks now pass:
369Python tests/1358client tests/zero doc tests, formatting/lint/public tree and
unchanged Cargo.lock,8m36s total. The923-line scope warning remains; --quick skips
parity and untouched content/theme/launcher checks are not rerun. Broad log SHA256:
`f230e68775b7af5f6bc4c7771c8b5b929688fc7743f3a2a9058232976f85e7c7`.
Both final statuses and the outer qualification exit are0. Different fresh
independent final review verifies the exact clean candidate/source, all original
resource/native/Game evidence and completed broad receipts, with no actionable
findings. The reviewer performs native read-only inspection only.
Unchanged lib source remains supported by the qualified base's native/broad proof;
no new standalone lib resource execution is claimed here.

Pregen eligibility (including paused tasks) and unreachable inner/final return
errors are source-supported only; no process-global controls or manufactured
double-failure path is added. World-slot snapshots prove key/payload values, not
private payload addresses. Pending item-slot failure retention and ordinary-drop
successful closure are separate fixtures; successful slot conservation is not
claimed. Counters are transient and their retirement is not machine persistence.
Outer Game teardown, other historical losses, producer quiescence, coherent
checkpoints/recovery, external clients, performance and G1/G2/G3 remain open.
No public push, merge or release is authorized.
