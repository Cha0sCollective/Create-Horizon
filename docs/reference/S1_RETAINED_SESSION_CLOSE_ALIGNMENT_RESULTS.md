# Retained Session close alignment

FINAL ACCEPT at `9d3ecdc510adbeb70840371fec2c0b9db4c23746`, tree
`7443d75edda4e4565e5bf8caa06468b77c5154f3`, sole parent `e6e584c`, branch
`codex/s1-session-close-alignment`. Different independent FINAL receipt `74740bb3`
also clears public privacy. Normal-hook public push passed and remote HEAD was
verified exact; public-sync receipt `f56a8421`. S1/G1 remain open.

## Result

One existing file, net.rs, changes by 235 additions and 31 deletions. The dormant
experimental RetainedSession now accepts the parent's exact close Instant and an
expected attempt for Retry. First explicit close records attempt 1, including
after an actor-terminal stop. Repeated close preserves its original deadline and
attempt. Observation exposes the attempt.

Retry validates blocked state, exact attempt, future replacement deadline and
checked increment under one State lock, taking its time snapshot after acquiring
the lock. State, Attempt, Deadline and Exhausted refusals leave ownership and
waiting unchanged. In particular, refusal neither polls nor consumes a finished
but unjoined handle. Successful Retry extends waiting and reissues the original
stop request; it does not restart actors, replace transport, replay payloads or
clear errors. Existing native quiescence takes precedence over elapsed time.

Ordinary Session, every Game factory, transport acquisition, interrupt capabilities,
claim/admission order, queue and active-body custody, checked joins, cookies,
transfer and fatal Drop remain unchanged. All repository callers of the changed
retained API are its existing net.rs tests. No compatibility wrapper retains the
old unrestricted Retry route.

## Qualification

Fresh independent DESIGN and SOURCE/tool reviews accepted the bounded change.
Formatting-only renewal precedes all compilation. There were no semantic source
corrections or failed checks. The source manifest is `c940719a`, qualifier
`c5187c7c`, expected definitions `928ea849`; all execution used the same guarded
live source and actual library-test artifact.

| Check | Result |
| --- | --- |
| Explicit-file formatting, paired all-target Clippy, feature-off compilation, public tree | Four passed, 74.45 seconds |
| Library-test build | Passed, 52.59 seconds |
| Eight exact existing network cases | Passed, 0.15 seconds total |

The cases exercise prepared/unstarted closure, original late registration and
pending acquisition, exact/repeated deadlines, pre-begin/early/stale/expired/
overflow refusals, successful current Retry, original reader identity, finished-
but-unjoined custody, genuine checked completion, terminal panic before explicit
close, real stalled MemoryPipe/TCP partial reads and actual active TCP backpressure.
No new test actor or case, prepared-resource inventory, Game/window/Java, broad or
fatal battery was needed. The unchanged fatal destructor was not replayed.

Build settings were opt-level 0, line-table debug information, debug assertions,
overflow checks and exact features `mods`, `native-writer-test-control`,
`paired-packages`. Actual copied test binary SHA256 is
`7d9777f167cf45656a9ebb9cee38d75f4f2c680117b4121a053a9b669b1e34c0`.
Each exact case used isolated directories and a direct-process timeout. Nonzero
exit, timeout, ignored/missing case or absent one-pass output fails qualification.

## Integrity and limits

Fifteen raw live/archive pairs map in `248cab80`: one changed source matches
committed Git raw-exact, ten unchanged baseline inputs have explicit CRLF canonical
comparisons, and four private recipe/instruction inputs match raw. Guarded
dirty-parent source was subsequently committed raw-exact; no archive execution
or fresh clean-commit execution is claimed. The qualifier is retired by its
original-parent guard. Supplemental instruction/current-work context was copied
after qualification.

Before source work, the initially proposed s1sc1 evidence prefix collided with
older unrelated history. No existing files were overwritten. The abandoned new
records and older files remain preserved; independent DESIGN renewal confirmed
the sole prefix correction to unique s1sda1. No source ran under the abandoned
prefix. FINAL input index `09ca07aa` freezes 86 entries; all 3,607 preceding indexed
entries were checked unchanged by the coordinator before and after qualification.
Different FINAL independently repeated all 86 frozen and 3,607 historical entries
initially/finally, recomputed all 15 raw/archive/Git comparisons and inspected the
full source, recipe, actual build artifact metadata, assertions and logs, with
zero mismatches or findings. It used native read-only inspection and did not
reopen the Linux binary or execute tests. Runtime conclusions rely on the guarded
receipts and matching logs, rather than a new binary rehash by that reviewer.

This aligns one actual owner's close contract. State locking, encoding/allocation
and MemoryPipe shutdown remain indivisible calls with no hard UI/mutex latency
promise. Inbound events and MemoryPipe byte queues remain unbounded; outbound
budgets are not an overall memory bound. Real client construction, outbound
prediction/refusal handling, decoder/streamer dispatch, original Game/window
closure and server composition remain open. No client/default activation, G1,
live S2, whole-client/performance qualification, PR, merge, tag or release follows.
