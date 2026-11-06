# Experience 003 — Goggles HUD source candidate held

Status: HOLD after independent read-only Sol High review. Do not integrate the
worker code as a qualified correction. The earlier
`EXPERIENCE_003_PROPOSAL.md` remains a scoped planning input, not a result.

## Exact candidate and checks

- Isolated worker worktree: `G:\Create Rusty\.local\teams\experience-goggles`,
  branch `codex/experience-003-goggles`, clean at
  `d2bc7365b2eef4f657b0063cb0d2d5b38591f30d`. Base is accepted native
  `55339ce3347b2ce667281e8a3e6318ec856144e4`.
- Only `native/create/mixins/create/presentation.rs` changed. The worker added
  selected millstone impact, small-wheel capacity, a spectator guard, a plain
  shaft suppression, and focused formula/field tests while retaining the
  existing Speedometer trace and other network panels. Pinned Rust 1.99.1
  formatting and `git diff --check` passed. No Cargo build/test or game run was
  authorized or performed; the added unit tests have not run.
- Worker `/root/goggles_code` recorded 425.696 seconds in the implementation
  phase; token telemetry is unknown. Reviewer `/root/goggles_code_review`
  used approximately four minutes in strict read-only Sol High review.

## Blocking source finding

The worker verified that a diagnostic can cross the pinned Horizon update-tag
and client merge path, but the private producer does not create that diagnostic
for the selected water-wheel case. `kinetic_runtime.rs:supported()` lists only
creative motor, shaft, cogwheel, large cogwheel and millstone. Its component
eligibility requires every node to be supported, so a water wheel and any
millstone connected to it are excluded. `diagnostics()` reads only those
eligible propagation nodes; `runtime.rs` removes missing diagnostics. The new
HUD requires `create_horizon:kinetic.TheoreticalSpeed` and returns when it is
absent. It would therefore hide the selected wheel and wheel-driven millstone
panel indefinitely. The focused tests inject the field and cannot establish
the real producer path. Actual `Speed` is not an acceptable fallback because it
is zero under overstress while Java's impact uses theoretical speed.

The reviewer also found a localized punctuation defect: the candidate appends
`:` to stress-impact/capacity labels whose supplied translations already end
with `:`. This is a minor repair only after the blocking field question is
resolved. The selected formulas, spectator/shaft/gauge selection and trace
fields had no other actionable review finding.

## Disposition and dependency

Keep `d2bc736` isolated and unmerged. The project lead owns the next bounded
decision with kinetics/Machinery: supply selected theoretical-speed information
through an already supported, reviewable server/client path, or narrow the
feature target. This team will not add a shared API, host pin or schema under
the present allocation. A later repair needs a new stable diff, fresh
independent read-only Sol High review and the shared heavy validation lease.

The disposable Survival runtime plan remains: fixture one golden sheet, two
glass blocks and one string; craft in the ordinary UI, check exact consumption,
head-slot equip/unequip, actor/peer rendering and normal cold reopen. Observe
selected millstone and small-wheel HUD text against server diagnostics on actual
integrated and dedicated client views, including zero/overstressed cases,
Goggles off, F1, spectator, plain shaft and retained Speedometer. These are
planned observations, not passes. Golden-sheet production remains
Machinery-owned; supplied-sheet crafting cannot establish world-obtainable
progression or full UI parity.
