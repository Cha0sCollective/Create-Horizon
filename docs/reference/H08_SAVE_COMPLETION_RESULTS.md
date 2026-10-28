# H08a1 scoped generation-save completion

Local engine commit is `1e87673f3ee88b9799b41d240486af9bd7d4fcfa`, tree
`2ede08b80d184d450751768f998aa6e2a12356bd`, branch
`codex/h08-save-completion`, based on qualified counter commit `28ded16`.
The four-file change makes Streaming save/poll
return values to `Result`, reconciles relevant queued/deferred completion once,
and propagates errors through the three production Game calls before dirty
snapshots are consumed. It preserves unrelated deferred order and checked-out
worker/proto ownership. Asynchronous saves remain nonblocking.

Three explicit resource-backed controlled tests pass: timeout preserves an
outstanding checked-out proto, another unsaved proto/written key and World dirty
state; returning the worker through deferred results permits retry without a
duplicate write; queued/deferred decoration/finish/save results reconcile while
unrelated order survives; disconnected required work errors, while idle completion
and synchronous direct proto storage remain possible. Resource availability is
asserted rather than silently skipped. The gate has a five-second bound even if
an assertion fails. Raw result: `.local/h08a-completion-tests.log`.

Fresh independent read-only Sol High source review finds no issue in the exact
formatted candidate and all repository Rust callers. Feature-on/off lint checks,
all three resource-backed generation-save regressions, 33 disk-save regressions
and 137 binary tests pass (three binary tests ignored). Formatting and the normal
public-tree commit hook pass. Raw logs are `.local/h08a-*.log`.

Frozen fast-profile executable SHA256 is
`5b83011061c15c94f44e0f0064ed062f515b780c535416a3feee25bfc63f42e9`, with the same
pinned Rust, feature selection, owned resources and exact full counter driver as
H05a. Integrated rerun `d98fbd6c-7fab-444d-bb8b-c47bd93e0380` passes 362 assertions;
in-frame `e63f2400-b3eb-4199-a667-24621af01795` passes 347. They repeat full
counter/activation/state/mutation/SDK/dimension/residency/save/restart/admission
controls. Final counter retirements report integrated value 70 at time 472 and
in-frame value 23 at time 242; the old in-frame scope retires on the dimension
switch with value 17 at time 179.

Initial/restart exits are 0 with no cleanup error in both runs. Changed/missing
package phases explicitly refuse and exit 1, preserving saved bytes. Integrated
missing-package cleanup records the quit RPC reporting that the refused game
already exited; do not describe every phase cleanup as null. Other cleanup errors
are null. Independent read-only receipt review corroborates both actual runs and
normal dimension/save/restart/close behavior, with the cleanup caveat preserved.
Broader quick verification passes from the clean native Linux clone at this exact
commit: `tools/pr_ready.py --quick --base 28ded16ed9c3d71328d829d149bb624c322f075a`
reports 369 Python and 1,307 client tests, formatting, Clippy, doc tests (zero),
public tree and unchanged Cargo.lock. Prepared resources are present for tests
which use them. Untouched content/launcher checks are skipped; vanilla parity is
explicitly not run by quick mode. Raw record: `.local/h08a-pr-ready.log`.
No engine push occurred.

The polling prefix scan is linear in existing deferred length, not a strict
two-millisecond bound. The receive deadline does not bound direct generator
storage callbacks. Plain generation jobs and generator callback storage errors
remain outside the scoped drain. No complete producer/writer quiescence,
machine persistence, coherent checkpoint/recovery, inventory or G1 claim follows.
Game failure propagation is source-reviewed, not injected in an actual game.
See `../design/H08_SAVE_COMPLETION_PACKET.md` for scope and stop conditions.
