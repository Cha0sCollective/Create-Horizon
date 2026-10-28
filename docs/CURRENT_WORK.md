# Current work

The owner resumed development through a genuine blocker or required input. H02d
real-game activation qualification now passes in both modes. The active bounded
packet is `docs/design/H03_STATIC_MATERIALIZER_PACKET.md`; its isolated worker owns
parser/materializer/tests, and the coordinator owns integration, actual game
checks and independent review. No standing team or public publication is authorized.

- Private branch: `codex/h02-smoke-h03-notes`, following checkpoint `f19e0f9`.
- Engine branch: local `codex/h03-state-plan-integration` at
  `b1ced4fcd26fe9ecae5403b6968500342162ca1c`, tree
  `ada6ce48c66937f9b48ea58cf8a32b63a99f0256`. Reviewed immutable state-plan module;
  custom state admission remains unsupported until the materializer is integrated.
- H03b worker: `.local/worktrees/h03-static-materializer` on
  `codex/h03-static-materializer`, based on b1ced4f. No worker commits/staging/pushes.
- Frozen tested game remains engine `60050caa3b70b3496f40cf2080b1c3c8354a67a6`,
  tree `e13629cc51c0e95d026a9e2db1b71a1ca7c4ce49`; its executable/build receipt
  stays separate from the developing checkout. See H02_GAME_SMOKE_RESULTS.md.
- Create reference remains clean, detached at
  `7ff2760e4924d35d4bc2e22c4b19923f0178cd16`. Source/toolchain pins are in
  workspace.toml. Effective operating instructions remain AGENTS.md and
  orchestration/ADOPTION.md; no operating-rule or permission expansion occurred.

H02d accepted runs under `.local/h02d/game-runs/`:

- Integrated: `0cafe3ba-1d3c-4566-ba89-f21af219733f`.
- In-frame: `f20bd80e-5b61-4fec-b745-1675ed735b43`.
- Both pass all 34 assertions, including client/server commands, one installed
  row, side settings, forced captured-asset rebuild after source corruption,
  same-build restart/custom-block save, changed/missing-package refusal with all
  saved-world bytes unchanged. Fresh independent Sol High review checked driver
  and actual receipts. Earlier calibration failures are retained and excluded.
- These prove local package activation/admission, not server-settings restart
  semantics, rendered appearance, machine inventories, H06/H08 or performance.
- External paired clients/LAN/dedicated networking still refuse pending H09.
  Settings storage remains nontransactional. Never use Prism/playable saves.

H03a StatePlan validation is reviewed and locally committed; its 35 paired-feature
and 18 feature-off library tests plus lint checks passed. See
`docs/reference/H03_STATE_PLAN_RESULTS.md`. The next materializer must handle every
JSON/NBT state table, explicit nonfirst defaults, static identity transforms,
paired-captured-only admission, an iron-block template restriction and waterlogged
rejection. Bound eager state expansion per package before retaining plans; apply
existing global state limits again with vanilla/legacy states included. Custom
shape/orientation/placement and machine behavior remain later packets.

After the candidate is ready: independent review, focused real-resource regressions,
fresh feature build, and actual six-state default/explicit-state/save checks in
both game modes. Then run appropriate broader regression/parity checks on the final
integrated candidate. Do not present skipped missing-resource tests as evidence.

Earlier accepted S0/P02/reference/B0 results remain in their reference records.
R01 preserves the owner's explicit full-output millstone item-loss decision.
G0 allows bounded experimental work; full H02/H03/P02 and G1/G2/G3, authoritative
machine lifecycle/transactions/persistence and retained animation remain open.
Publication, upstream submission, promotion and release are separate decisions.
The latest resume instruction supersedes the earlier time cutoff and break.
