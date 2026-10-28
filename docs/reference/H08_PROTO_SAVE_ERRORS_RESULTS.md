# H08a2 proto callback errors and owned snapshots

Local engine commit `928f43c2db203049c6266baf3682b16a806f028a`, tree
`a0d9537b4e044716318ce4c00f4b319a45175ca3`, branch
`codex/h08-proto-save-errors`, changes exactly streaming.rs, Persistent's
proto adapter in save.rs, and the existing generation-save test generator.
Native proto callbacks return an owned diagnostic error. Worker completion always
returns its entire owned proto batch and the outcome. Reconciliation restores
ownership and failed dirty keys before retaining the first error.

The fault is sticky for the Streamer's lifetime. Direct, asynchronous and polling
save requests refuse while retaining dirty data. New terrain producer jobs stop
before eviction, finalization, neighbor decoration or generation can consume it.
Existing completion results, lighting/mesh bookkeeping and ordinary resident
unloading continue. This is an explicit terrain-production halt, with no retry,
reset or cancellation policy. The existing terrain producer body is extracted
without changing its successful behavior.

Fresh independent read-only Sol High review verifies the exact three-file
candidate, unchanged extracted/moved bodies, error/ownership guards, all callers
and meaningful test coverage, with no finding. Three previous controlled
completion tests and three new proto-failure tests pass using explicitly required
owned prepared resources. They cover direct failure, pointer-stable worker batch
return, first-error retention, deterministic zero-budget deferred-tail ownership,
ready finalization/neighbor decoration, repeated center movement with no new
producer jobs, and continued pre-existing Proto/Decorated/Generated/Lit/Meshed
reconciliation. Normal generation-save tests (three) and disk-save tests (33)
pass. All 137 game binary tests pass (three ignored), along with feature-on/off
library/binary/test Clippy, formatting and the normal public-tree commit hook.
A frozen fast-profile executable SHA256
`66a51ee6a2c3a8ecea1601fd956720cab3671da90fd0d4f8275ec067c21fc81d` passes the same
complete counter driver and owned resources in both fresh isolated game modes:

| Mode | Run | Assertions |
| --- | --- | --- |
| Integrated | `319ba123-d21c-41b7-8f1c-e663c91211a8` | 362 pass |
| In-frame | `40c1c696-6b3b-44db-8a1d-aefa0260e670` | 347 pass |

Normal save, dimension, restart and close receipts corroborate final counter
retirements 63/20. In-frame dimension retirement is 17 before a new zero record.
Initial/restart exits are 0 with no cleanup error. Admission refusals exit 1 and
preserve saved bytes. Integrated missing-package cleanup finds the already absent
socket when sending quit; retain this expected refusal cleanup race. Other cleanup
errors are null. Independent read-only receipt review confirms identities, exact
archived drivers and the decisive raw controls, with no new finding.

Broader quick verification passes at this exact commit:
`tools/pr_ready.py --quick --base 1e87673f3ee88b9799b41d240486af9bd7d4fcfa`
reports 369 Python and 1,307 client tests, formatting, Clippy, doc checks (zero),
public tree and unchanged Cargo.lock. The scope warning includes the coherent
170-line producer-body extraction; independent review compared both moved bodies
exactly. No public PR is proposed. Untouched content/launcher checks and vanilla
parity are not run by this quick invocation. Raw logs remain
`.local/h08a2-*.log`; raw game archives remain under `.local/h02d/game-runs/`.
No engine push occurred.

A successful callback means enqueue acceptance, not durable disk completion;
SaveDir writer barriers still apply. Already-submitted work is finite; no absolute
new memory quota is introduced. Plain generation, void finished-chunk callbacks,
player/entity/metadata ownership and coherent checkpoint/recovery remain open.
This is no machine persistence, saved generation, transaction, inventory or G1
qualification. Actual callback errors have not been injected into a running Game.
