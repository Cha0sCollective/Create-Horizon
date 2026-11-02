# Retained Session close: exact parent deadline and Retry

Status: HELD proposal, not an engine source release. Parent is clean accepted
engine `e6e584c53ff404b84fb364681d49d41701f9492f` (tree `b6d23d8a`), reviewed
public branch synced; private results `d41a866` synced. S1/G1 remain open.

## Concrete next change

The actual retained decoder and remote streamer accept an absolute Instant and
an exact attempt for Retry. RetainedSession in net.rs instead accepts Duration,
computes its deadline after acquiring State, and renews it whenever stopped.
A Game parent cannot preserve one exact deadline by converting it to Duration;
unrestricted Retry can also renew the wrong or still-active wait. Align this
existing dormant experimental owner before composing it into the original Game.

Maximum source scope is ONE existing file:
`crates/horizon-client/src/net.rs`, including its existing retained tests. Use
logical branch `codex/s1-session-close-alignment` and evidence prefix `.local/s1sc1`
in the same stable checkout and dedicated warm target. No new module, worker,
executor, transport, owner facade, dependency, lockfile or public documentation.

Replace, rather than duplicate, the dormant retained close signatures:

```
begin_close(&mut self, deadline: Instant) -> RetainedClose
retry_close(&mut self, expected_attempt: u64, deadline: Instant)
    -> Result<u64, RetainedRetryRefusal>
```

Add an attempt counter to the original State, initially zero, and expose it in
RetainedObservation. First begin_close installs exactly the supplied Instant and
attempt 1, including when a terminal actor already stopped admission. Repeated
begin_close must preserve both. A past first deadline is valid and immediately
Blocked when original handles still require joining. Prepared/unstarted closure
can be Quiescent through its existing actual native conditions.

Retry succeeds only for the same owner's current expired blocked close, exact
attempt, and a future replacement Instant. Use checked increment; refuse State,
Attempt, Deadline or Exhausted without altering deadline, attempt, admission,
transport or any retained payload. Perform validation and update under the same
State lock. Snapshot time after obtaining that lock; lock delay must not turn a
stale future deadline into accepted future waiting. A successful Retry extends
waiting and re-requests the original stop/wake only. It never restarts actors,
replaces endpoints, clears errors, reopens admission or replays bodies/events.
Repeated/stale UI actions cannot renew an attempt. Quiescent closure refuses Retry.

Keep existing native acquisition, original interrupt capability, claim/stop
ordering, finished-only checked joins, panic custody, cookie/transfer ownership
and consuming ClosedSession semantics. Ordinary Session and every Game factory
remain unchanged. Repository-use audit must confirm retained API callers are only
the existing net.rs tests; any external path requires exact scope review first.
No compatibility wrapper may retain a second unrestricted Retry route.

Actual TCP IO is outside State. This change does not claim that State locking,
encoding/allocation, or MemoryPipe shutdown is try-only or has hard UI latency.
The future parent must account for these indivisible calls; no new worker is
justified solely by blocked socket IO. Inbound events and MemoryPipe byte queues
remain unbounded; outbound queue limits are not global memory limits.

## Focused validation

Migrate existing retained test calls to explicit Instants and tokens. Extend the
existing real late-registration/after-registration/pending-acquisition cases to
check exact first Instant, repeated-close preservation, early and stale Retry,
expired replacement, checked overflow, successful current Retry and original
reader identity. Check refusal after genuine joined completion. Reuse current
private test gates and Fixture; do not add test-only native actors, echo workers,
visibility expansion or a new abstraction just for these checks.

The smallest runtime set is the affected retained close/Retry tests plus existing
real stalled MemoryPipe and TCP partial-read closures and the reader-panic/checked-
join case. Select exact observed names after source review. No decoder/streamer,
Game/window/Java or fatal battery replay: their implementations and the fatal
Drop path are unchanged. Add a specific regression only if source review finds a
changed dependency. Counts are a plan, not a cap.

Root owns serial explicit-file formatting, paired all-target Clippy, feature-off
compilation, actual lib-test artifact build, exact cases and public-tree checks.
Bind actual artifact/profile/features/binary and isolated runtime inputs. A timeout,
missing case, nonzero exit or missing native observation fails. These network
fixtures do not require prepared Minecraft resources; do not load/hash that large
inventory absent an actual resource dependency. Existing component evidence stays
at its actual pins, not presented as fresh execution.

## Gates and limits

Fresh independent Sol High DESIGN review of this exact proposal and assignment,
synced private design checkpoint and root release precede source. SOURCE/tool
clearance precedes formatter/compiler/runtime; renew post-format source binding.
Preserve all packet source and prefix artifacts before every correction. Archive
mutable execution/config/recipe/instruction inputs; qualified live bytes must
match archives and committed Git content. Preserve all predecessor freezes.
Different fresh FINAL/privacy review precedes reviewed public branch sync; fresh
visibility/exact origin and normal hooks apply. No PR, merge, tag or release.

This aligns the actual Session close contract. It does not activate a retained
Game/client/default path, decide outbound prediction/replay, bound inbound memory,
establish whole-client closure or performance, close G1 or authorize live S2.
