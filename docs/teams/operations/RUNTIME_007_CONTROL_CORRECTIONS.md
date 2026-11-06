# Frozen007 finite runtime control corrections

These corrections change disposable test controls only. Production source
remains clean at `f732bd481cfc3d9c4720bc51e4a31e7385e18020`, 742 native
files, source-map SHA `0c97fc70d5ce5a8dc81bed1b5ccc21887fbc69628c94e0b0d1a6aff015c70f8a`
and binary-receipt SHA `2ec5726527df08817663fa847df0b2dfdbb783deecc31d961c92a4ebae5074f5`.
No mod rebuild, host API/pin/schema change or historical battery follows.
Raw controls, worlds and receipts remain ignored. Every original HOLD remains.

## Drill003 placement convergence

Fresh integrated `integrated-sg_jjyu7` setup reported client facing down
immediately after ordinary placement, before processing. Later inspection of
the same world reported East. Actor exit0 and frozen inputs matched. This is
a setup HOLD, not a qualified facing, processing or persistence result.

R1 control SHA `73859bc8e8700270a938a7faefdd0b62622fcd9b66813239f65b03b060a9d97b`
is preserved. Separate `.local/native-drill-007/drill003_milestone_r2.py` has
SHA `23c78a571377e5afc3f0b7a64dcd9dfce7c900ecdd851cb002c264b9adf04b4f`.
It retains the immediate observation, requires fresh bounded server feedback
from an actual `execute if block ...[facing=east]` predicate, then waits
boundedly for client convergence. ACK alone cannot pass. Pose, placement,
stock, speed/stress, bedrock refusal, stopped35-tick and cold/resume cases
remain unchanged. This does not qualify immediate prediction latency.

## Tank004 relevant-material census

Fresh integrated `integrated-q68abcho` stopped before placing or supplying a
Tank: a global all-item guard found unrelated red mushroom litter outside
the clear pad. Actor inventory was empty and the Tank position air. Actor
exit0 and frozen inputs matched; no lifecycle case ran.

R1 SHA `3838a934c690b42946071ddce38e25151d35b00b292e3f993572bfd6eec6bc32`
is preserved. Separate `.local/native-tank-lifecycle-007/fluids004_tank_lifecycle_r2.py`
has SHA `77ee018602f2ae315d2adf1727e35316f6c73294af6bbff4fcd35e990caa9550`.
Every acceptance census now uses unbounded loaded-level server selectors
for the finite test materials: Tank, bucket, water bucket, wrench and pickaxe.
Tank must be absent except exactly one plain count1 drop between normal break
and ordinary pickup; other relevant world materials must be absent. Unrelated
items are separately counted and reported, never deleted or moved. Existing
produced-break counts, complete actor/peer inventory, drop NBT, local water
and replacement assertions remain. No unloaded-save census is claimed.

## Review and execution boundary

Operations worker `/root/ops_mixins_release` authored the separate copies.
Fresh independent read-only Sol High `/root/ops_mixins_release/r2_fixture_review`
returned CLEAR on each exact hash. Review used native/Git reads and its own
timing metadata only. AST, extracted captured-response and explicitly modeled
positive/timeout/selector/serialization checks passed; no positive live
selector, real convergence or gameplay result follows from those checks.
Both R2 game retries remain unexecuted at this checkpoint and need the
root-granted single heavy lease and genuinely valid disposable baselines.
