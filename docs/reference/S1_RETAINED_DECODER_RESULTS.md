# Retained native remote decoder

FINAL ACCEPT at engine `9f6466482189583678c6a16711086c037424ae75`, tree
`f50509098c6dbe50fe79e3e551ed43e70f87d06b`, parent `60899d4`, branch
`codex/s1-retained-decoder`. Different independent final receipt `a4502d26`
also clears public privacy. The normal-hook public push was verified exact.
This accepts a dormant helper, not ordinary client activation or G1.

## Result

Only `game/remote_chunks.rs` changed: 1,050 additions, zero deletions. The original
decoding algorithm, legacy constructor, Next interface and callers remain intact.
The separate paired-only retained decoder owns the actual native handle before
releasing its start latch. Registered slots retain each original packet, format
allocation and stored result outside worker unwind, including poisoned payloads.

Eight slots count queued, active and ready owned work together. Refusal returns
the original input; successful owned handoff retires that slot. The close fence
and worker claim share one atomic ordering. Existing active work can finish late;
queued work cannot start after the fence. UI operations use nonblocking lock
attempts, and unpark tokens avoid lost notification windows.

Close retains the original owner through deadlines and exact Retry, joins only
its positively finished native handle, then permits a consuming ClosedDecoder
handoff. Native panic or controlled spawn refusal remains a visible failure.
Remaining original payloads can be explicitly disposed after checked closure;
this does not certify their delivery to Game. Unresolved destruction aborts.

## Qualification

All executions use the same formatted raw source `33fb77de`, manifest `dbb86c6d`
and qualifier `74a57024`. Actual guarded dirty-parent source was qualified before
the clean commit; neither archived bytes nor the later commit constitute a fresh
execution. The actual Game-bin test artifact has opt-level 0, line-table debug
information, debug assertions and overflow checks, with exact `mods`,
`native-writer-test-control` and `paired-packages` features.

| Check | Result |
| --- | --- |
| Format, paired all-target Clippy, feature-off compatibility, public tree | Four passed, 46.20 seconds |
| Single Game-bin test build | Passed, 47.10 seconds |
| Six defining native scenarios and existing ordered-decoder regression | Seven passed, 0.14 seconds |
| Two separate actual unresolved-owner fatal children | Expected SIGABRT observed, 0.14 seconds |

The defining scenarios exercise real wire decoding and independent coordinate,
block-state and biome expectations, format identity, capacity/refusal/handoff,
held active and queued/late work, exact Retry, malformed packets, real worker
unwind, poisoned registry/payload recovery, fence races and unfinished native exit.
The fatal children require the actual held-worker marker, disabled core dumps and
process reaping. One has normal stderr; the other has a read-only stderr descriptor.
The latter still aborts even though the diagnostic write fails. These fatal controls
are separate from the seven normal cases, not graceful native-close results.

Source review corrected two issues before any compiler/runtime execution: a
diagnostic panic could bypass abort, and registry contention could hide an expired
deadline. Drop now ignores diagnostic-write failure before unconditional abort.
Blocked reports the original attempt even when detailed registry observations are
unavailable; it does not fabricate zero counts. The corrected candidate passed all
qualification on its first attempt. No resources, Java, game/window launch or old
network/save/LOD battery was needed.

## Integrity and limits

Fifteen raw mutable inputs are archived with qualified-live/archive/Git mapping:
one changed source is raw-exact Git, ten unchanged baseline files have explicit
CRLF canonical comparisons, and four private inputs stay out of engine Git.
Different final review rehashed all 151 accepted inputs and all 2,948 historical
indexed entries initially/finally unchanged. Its accepted index is specifically
`s1dc1-final-review-input-r1.json`, SHA `53e9d762`. The earlier 106-entry draft missed
two final receipts because it was assembled too early; it remains preserved with
the correction record and is not the accepted index. No source/test data changed.

The qualified binary is
`db5ac263c9f724e4961f6a4c4383998d6317357b70c80684463bcf5234149e79`.
The reviewer assessed frozen guarded build/run identity receipts; its direct Linux
binary reopening was denied. It performed no executable rechecks.

The slot limit bounds accepted work, not wire bytes, derived columns, Session
backlog, global memory or latency. Progress observations are not transactional
delivery witnesses. Controlled spawn refusal is not an OS resource-exhaustion
qualification. The unchanged legacy regression does not prove retained closure
for the legacy constructor.

Remote Session, received-column mesh/light workers, original Game construction,
server startup/Hub ownership and integrated checked joining remain separate work.
No save/native-writer, renderer, complete-client or global closure witness is
fabricated here. S1/G1 remain open; live S2, PR, merge, tag and release are outside
this acceptance.
