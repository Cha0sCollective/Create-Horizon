# QUILL-003 focused integrated retest handoff

Status: prepared, **not run**. Root owns the next exclusive game lease and
launches the saved integrated run `integrated-9vkqdlrp` with the frozen005
session runner. No source rebuild or fixture reset is needed. The earlier
integrated full/cold-reopen and dedicated full/cold-reopen receipts remain raw
evidence; their automated green labels do not override the visual HOLD in
`QUILL_003_RUNTIME_R2_STATUS.md`.

Preparation checks: Python AST parse and `git diff --check` passed. Fresh
strict independent read-only Sol High review of adapter candidate `8ead75f`
returned CLEAR after 120.7 seconds, including the exact saved-fixture gate.
No new game session was started for this preparation.

Use `tools/quill_cancel_quick_retest.py` only against a newly launched session
index 02 or later in that exact saved run. The script is packet-local and
launches no process. It admits only integrated actor metadata with the pinned
source revision, 711-file map, two binary hashes and Create JAR hash, then
checks all source-file bytes and those binary/JAR bytes before and after the
actions. It requires the original `integrated-pre-unload.json` fixture receipt,
origin `(40,65,20)`, four target blocks and actor Quill/stick counts. A failed
gate stops the retest and is not a gameplay result.

After root starts the session under its lease, invoke in WSL from the team
worktree with the new session JSON:

```text
python3 tools/quill_cancel_quick_retest.py \
  '/mnt/g/Create Rusty/.local/native-team-runtime-005/integrated-9vkqdlrp/session-02.json' \
  /home/arbiter/.cache/create-horizon/team-integration-005-bin-b862982154b5/horizon-ctl \
  '/mnt/g/Create Rusty/.local/native-team-integration-005/source-inputs-r3.json' \
  '/mnt/g/Create Rusty/.local/evaluations/create-horizon-05/inputs/create-1.21.1-6.0.10.jar' \
  --origin 40 65 20
```

Replace `session-02.json` with the actual new session index if needed. The
adapter first establishes a new completed `(56,65,20)` to `(41,65,20)`
selection and captures `before-cancel`; this does not assume any retained HUD
state from the cold reopen. It presses Shift, asserts sneak, right-clicks,
waits three ticks with Shift still down, releases it and captures
`after-cancel`. It then aims at `(56,65,20)`, sends two separate quick right
clicks and captures `quick-edges`. Expected visible outcomes are completed
`16 x 1 x 1`, “Selection canceled” with the first-corner prompt, and completed
`1 x 1 x 1`, respectively. The region and actor counts must remain unchanged.

The new JSON receipt and three PNGs appear under the saved run's `quill/`
directory, named `integrated-<index>-cancel-quick.json` and
`integrated-<index>-{before-cancel,after-cancel,quick-edges}-actor.png`.
`AUTOMATED_CHECKS_PASS_HUD_INSPECTION_PENDING` is only a command/fixture result.
Root must inspect the three new frames directly, retain the old r2 frames,
and record the actor's normal process exit. Completion also needs root review
of the reused r2 images and final independent read-only completion review.
No Save/export/upload/placement or train behavior is tested.
