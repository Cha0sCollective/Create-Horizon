# Experience 003 — selected Goggles HUD consumer repair

Status: isolated source candidate, fresh independent read-only Sol High review
**CLEAR**. No primary integration, Cargo test, build or game qualification is
claimed. The previous `EXPERIENCE_003_HELD_HANDOFF.md` remains the record of why
the original reader at `d2bc736` was held.

## Exact source and contract

- Accepted native base `55339ce3347b2ce667281e8a3e6318ec856144e4`;
  isolated worker branch `codex/experience-003-goggles` in
  `G:\Create Rusty\.local\teams\experience-goggles` is clean at
  `5dc780d46b3db40451fed07effc240fb0338f422`. The matching consumer
  repair is the stable `d2bc7365b2eef4f657b0063cb0d2d5b38591f30d..5dc780d`
  delta. Only `native/create/mixins/create/presentation.rs` changed; its final
  blob is `286748b396487860ac2b529cb1fd691353e30b0e`.
- Owner-approved field contract is primary
  `docs/design/GOGGLES_SOLVER_SPEED_FIELD_PACKET.md` at
  `781ceb56b193eb56657fe96f78f3b6ac33e2538b`, file SHA256
  `58ac7cb8e4b6124c7db490710561354f7f79c9e1f48bcf13a472d378084b56de`.
  Machinery owns the separate `runtime.rs` producer and its solver tests. Root
  composes the two deltas serially.

The consumer now reads only top-level
`create_horizon:solver_theoretical_speed`. It accepts a finite Float when
`NetworkStatus` is `Running`, `Overstressed` or `Idle`. An explicit Byte(0)
tombstone, missing/malformed/non-finite value or other status withholds the
selected provider; it never falls back to effective `Speed` or the old
five-type diagnostic. It retains the selected millstone and small-water-wheel
Java formulas, spectator guard, plain-shaft suppression, and existing gauge
trace. Localized stress-impact/capacity labels no longer gain a second colon.
No recipe, model, equipment, host API, schema or other provider changed here.

## Validation and remaining work

Pinned Rust 1.99.1 formatting and `git diff --check` passed. New focused tests
exercise invalid/status values, rejection of the old diagnostic, and
valid → Byte(0) → valid recovery through the existing
`ModBlockEntity::merge_update_tag` method. Those tests were added but **not
run**: no shared heavy lease was granted. Reviewer
`/root/goggles_consumer_review` checked the whole candidate and repair delta
against pinned Java 6.0.11 and host `9c7f9b22`; result CLEAR, approximately
1m57s, strict read-only. Worker `/root/goggles_code` recorded 108.776 seconds
for this repair phase. Token telemetry is unknown.

Root must integrate the Machinery producer and this consumer together, then
use its shared lease for focused compile/tests and the selected integrated and
dedicated client milestone in the approved field packet. Verify server field
publication, invalidation and actual client text, including zero/overstress and
normal cold reopen. The separate
`EXPERIENCE_003_CRAFT_EQUIP_CHECKLIST.md` covers supplied-sheet crafting and
head/peer rendering without a HUD claim. No complete Java tooltip or Survival
progression claim follows from this source review.
