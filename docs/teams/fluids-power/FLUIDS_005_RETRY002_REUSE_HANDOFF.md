# FLUIDS-005 retry002 same-world continuation handoff

**Status:** corrected driver ready; no new game run or lease. The exact
one-line correction is clean commit
`69a4f80397271f1b545f18a986b49670f9f86ba1`, changing only
`tools/fluids_005_runtime/case.py::shot` to pass `str(path)` into the
existing control recorder. Driver SHA-256 is
`59b7bf8602e9af0d14c96b541b418037966f54cc4ac1534ff15d71555c2672fb`.
The actual captured screenshot call was serialized offline with that boundary
change; AST, whitespace, and typed numeric retention checks passed. No
production source, host API, resource, pin, schema, fixture or acceptance
criterion changed.

The original dedicated retry002 reporting HOLD remains in
`FLUIDS_005_RUNTIME_007_RETRY002_HOLD.md` and its raw run mirror
`G:\Create Rusty\.local\native-fluids-005-runtime\dedicated-d1m1sey4`.
There is **no** `before-cold-result-00.json` PASS or HOLD file; do not create
one retroactively. The process traceback reached the `else` result serializer
after `before_cold()` returned, then raised `TypeError: Object of type
PosixPath is not JSON serializable`. The sole screenshot record, raw
`calls-00.jsonl` (208 zero-code calls), `pre-cold-amounts.json`, initial peer
PNG, server log, and `exit-00.json` remain untouched. The peer PNG still
requires Operations' canonical visual inspection.

Fresh independent read-only Sol High review is **CLEAR** for the exact
correction commit above. The reviewer verified the raw record and found
the before-cold assertions supported: 1,000/0 unchanged over 16 stopped
ticks; at 16 RPM, 984/16 → 904/96 over ten active ticks; branch refusal
unchanged 904/96; removal and ten steps → 824/176; opposite signed speed
and ten steps 808/192 → 728/272; then Speed 0 and 728/272 stable after
unfreeze. The normal save/quit sequence completed and server, actor and
peer exited 0. Review interval: 2026-11-06 07:16:49–07:18:13 UTC (84 s).
This is independently verified **raw before-cold evidence**, not a newly
manufactured script PASS receipt.

**Reuse decision:** a new root-granted game slot may cold-reopen this same
owned run at index01. Use the frozen 007 arguments from the local launch
runbook and `--run '/home/arbiter/.cache/create-horizon/team-fluids-005/dedicated-d1m1sey4'`
with `mode=dedicated`; run only `case.py` `after-cold` against the new
`session-01.json`. Do not replay before-cold, start another dedicated world,
or touch the old integrated setup HOLD. The cold case must independently
verify the saved 728/272 pair, `spawn_mobs=false`, complete transfer,
full-input refusal, actual peer filled/empty images, and normal three-process
exit. Preserve any new HOLD and leave final peer-frame acceptance to
Operations. No runtime validation follows from this source-only review.
