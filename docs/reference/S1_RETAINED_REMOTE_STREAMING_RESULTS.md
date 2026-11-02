# Retained remote chunk streaming

Qualification complete at engine `e72475864c13720fe066d5bd6335d652ce5cd2bf`,
tree `4a5567d16d154f8b720a8d8c6d82bf69a28f2ca5`, parent `9f64664`, branch
`codex/s1-retained-remote-stream`. Different independent FINAL receipt `ce3dae78`
accepts it and clears public privacy. The normal-hook public branch push was
verified exact. This dormant helper does not establish client activation or G1.

## Result

Only `streaming.rs` changed: 1,627 additions and 28 deletions. The paired-only
opaque `RetainedRemoteStreaming` privately owns the original concrete Streaming.
It reuses the original queue, worker handles, ownership receipts and real mesh
and light algorithms. It requires no fabricated SaveDir or native writer witness.
Legacy and native factories and public signatures remain intact.

Preparation is inert. Explicit start registers each actual handle before releasing
its start latch; partial startup retains the same owner. The fixed profile accepts
one through eight workers and at most 64 admitted jobs across queued, active,
stored and reconciling phases together. Busy/full/closed refusal returns original
owned inputs, and refused column insertion leaves the caller's world unchanged.

Registered slots retain each original region, context, settings, admitted sequence
and successfully stored result outside worker unwind. A handoff explicitly returns
the original input and sequence, with any real output. Current-result checks compare
all nine original neighborhood allocations and the job sequence. Stale results
remain available for explicit disposition. A slot retires only on owned handoff.

Close and worker claim share one atomic ordering. Thread unpark tokens preserve
wakeup across idle races. UI polls use try-locks; an unavailable census is explicit,
and an expired original deadline still reports its blocked attempt. Exact Retry
extends waiting without replay, replacement workers or reopening admission.

Only positively finished original handles are joined, with checked outcomes.
Distinct `ClosedRemoteStreaming` retains queued, failed and late payloads after
native quiescence. Native failure remains visible. This remote completion cannot
satisfy native producer or writer closure. Unresolved destruction aborts even when
writing the diagnostic to stderr fails.

## Qualification

All successful checks and executions use raw source `356396c6`, manifest `cf013287`
and qualifier `513d1d69`. The actual library test artifact uses opt-level 0,
line-table debug information, debug assertions, overflow checks and exact `mods`,
`native-writer-test-control` and `paired-packages` features.

| Check | Result |
| --- | --- |
| Format, paired all-target Clippy, feature-off compilation, public tree | Four passed, 82.22 seconds |
| Library test build | Passed, 58.26 seconds |
| Six defining resource groups and two affected native regressions | Eight passed, 42.66 seconds |
| Separate ordinary/read-only-stderr fatal controls | Expected SIGABRT, 11.59 seconds |

Real prepared Minecraft 26.3 resources supply the WorkerContext. All 28,176 pinned
resource entries are checked before and after both runtime batches. The real mesh
fixture independently expects a stone cube's 24 solid vertices and positions;
the light fixture checks sky and opaque-block values. Other groups exercise partial
startup and initialization failure, exact 64-slot mixed-phase saturation, original
refusal identity, deterministic claim/fence/idle/native-exit races, real worker
unwind before and after storing outputs, and intrinsic generation/storage refusal.
The two existing regressions cover native SaveDir association and shared real
Light/Mesh stale-result ownership. Older decoder/window/network batteries were not
replayed. Fatal controls require an actual held-worker marker, disabled cores and
reaping; they are not graceful close results.

Source review corrected three P2 findings before execution: neighbor-only changes
were missing from stale checks, queued/failed handoffs omitted their sequence, and
separate census scans could count one transitioning job twice. After formatting,
the first compile attempt failed on a fixture's typed block ID and Clippy's large
owned refusal result. That failed receipt remains `verified=false` at source
`85d61686`; no runtime or test build ran there. The exact two-line reviewed fix
uses the numeric ID and documents the intentional original-column return. All
successful qualification then ran at `356396c6`.

## Integrity and limits

The clean commit is raw-exact to the guarded live source that was tested. Fifteen
mutable source/config/recipe/instruction inputs are archived and mapped: one
changed raw-exact source, ten explicit unchanged baseline CRLF canonical Git
comparisons, and four private pairs. Supplemental workspace/current-work context
was copied after qualification, separately from those executable recipe inputs.
The source map is `679ece89`; the frozen 163-entry review index is `30ae3427`.
All 3,099 historical indexed entries remained unchanged before final review.
The different reviewer independently rehashed all 163 current inputs initially
and finally, all 3,099 historical entries and all 15 live/archive/Git mappings,
with zero mismatches. It performed native read-only inspection, with no execution.
No execution of archived bytes or fresh execution of the clean commit is claimed.

The qualified binary is
`25af239527b73d0ae5d185817b3ff6345ff81f9c3ddf291ba06b4cbed83c4448`.
The reviewer did not reopen the Linux-only binary/resources; its runtime conclusion
relies on the frozen guarded build/run receipts and matching logs.
The guarded qualifier is naturally retired by its original-parent check after
commit; it must not be edited for a future packet.

The 64-job policy is not a global byte/memory bound or a latency qualification.
Census values partition single per-slot observations, not a globally simultaneous
snapshot. Controlled spawn refusal does not qualify actual OS exhaustion. A
payload handoff is not Game application, renderer upload or delivery to a server.

Real client integration must adopt the opaque streamer, retained decoder and
RetainedSession in an original Game construction and close recipe. Server startup,
LAN-to-Hub ownership, shared LOD/storage ordering and scoped checked server joining
remain separate work. S1/G1 stay open; default integrated and dedicated/two-client
qualification, live S2, PR, merge, tag and release do not follow from this helper.
