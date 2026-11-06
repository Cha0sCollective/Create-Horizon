# QUILL-003 frozen005 UI milestone handoff

Status: packet-local driver and checklist reviewed CLEAR as source, not yet
executed or gameplay qualified. Root owns the forthcoming frozen005 integrated
and dedicated two-client lease. The team lane
`codex/team-trains-schematics` began this packet at `b0fee7d`, created the
driver/checklist at `0f9aa24`, and repaired the review findings at
`993917cf972ab54991fc46fce2e9087c1ed7e3f4`.

Changed files are `tools/quill_ui_milestone.py` and
`docs/teams/trains-schematics/QUILL_003_UI_MILESTONE_PLAN.md`. The driver
uses the root's actual frozen session JSON and matching `horizon-ctl`, hashes
the two binaries against session metadata, and writes raw receipts and PNGs
under that session's mirror, outside Git. It never builds or launches a game.
Root chooses a safe origin in the disposable world, runs `full` for each mode,
normally unloads/exits, then runs `reopen` on a new session index in the
same run. The checklist gives exact calls and visual evidence requirements.

The critical source finding is that generic `interact` calls `play_mouse`
without Quill's native key event. The driver uses `look-at` for aiming and
explicit `mouse right down/up/click` for every selection. It covers held
repeat versus a new edge, quick edges, reversed inclusive 16/17 bounds,
third click/no Save, cancel, item and Creative-screen resets, ordinary stick
lever use, unchanged selected-line blocks and item counts, dedicated peer
selection isolation, same-process unload/rejoin, and a separate cold reopen.
The dedicated setup waits for observed server results rather than treating
the command send acknowledgment as completion.

The initial independent strict read-only Sol High review held on a wrong
Creative-screen assertion and dedicated setup races (218.6 seconds). The
focused repair-delta review returned CLEAR with no remaining source finding
(85.6 seconds). `git diff --check` passed. Python syntax execution was
unavailable in this Windows session (the local Python aliases were unusable
and WSL access was denied); the reviewer likewise performed source-only
inspection. No game, Cargo, stage/link, Java, network or host-wide lease ran.
UTC phase intervals are in `.local/team-operations/trains-schematics/phases/`;
per-session token counters are unknown here.

The driver labels successful command assertions
`AUTOMATED_CHECKS_PASS_HUD_INSPECTION_PENDING`. Root must inspect every
rendered HUD frame for the listed text, coordinates, colors and absence where
expected, verify all process exits and source/binary receipts, and complete
the independent runtime review before calling the milestone PASS. Save,
upload, placement, Java file import and train behavior remain outside scope.
