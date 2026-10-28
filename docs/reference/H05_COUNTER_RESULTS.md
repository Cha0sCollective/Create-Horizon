# H05a transient native counter

The bounded native core is committed locally in Horizon at
`8cff24d434dbc7466abf7030a0894887031cd712`, tree
`ebf96157948c466d412553d21a81c50fe3dc876d`. It follows captured machine metadata
and qualified residency observations; it does not complete persistent H05 or G1.
Public engine publication remains unauthorized.

One explicitly configured typed block coordinate owns a native u64 counter and
checked process-local instance identity. Committed writes admit/retire immediately;
no-ops preserve identity. Matching streamed reload resumes a dormant record;
unexpected state mismatch faults explicitly. Whole-chunk replacement retires the
old instance. SDK map lending does not simulate unloading. Advancement checks
running, eligibility, residency and matching state, increments once per game time,
and never catches up elapsed time. Arithmetic/allocation exhaustion is sticky and
does not fail ordinary world writes. Explicit take returns final retirement state.

Six focused counter test groups pass, alongside eight mutation tests, the residency
filter (five observer tests and one incidental match), one explicit-resource
streaming seam and the World regression with paired packages enabled and disabled.
Library/test Clippy passes in both feature configurations. Raw logs remain under
workspace `.local/h05a-*.log`; no generated resources enter Git. Independent
read-only Sol High review cleared the core and the checked-CAS allocator delta.

The subsequent Game controller and lifetime wiring resolve the selected captured
binding against the frozen registry and schema, reuse checked owner-level routing,
and gate the tail phase by running, block-ticking tickets and lit resident storage.
Replicas stay disabled. Explicit retirement precedes actual World replacement and
normal teardown; returned save errors are preserved. Stderr receipts are best
effort, with no crash-delivery guarantee. Independent source review found no issue;
Game source is committed locally at `28ded16ed9c3d71328d829d149bb624c322f075a`,
tree `c381f9fdbecead76e896b97c28e283b4ab325f77`. Formatting, paired-on/off binary
Clippy, and all 137 binary tests pass (three ignored). The frozen fast-profile
executable SHA256 is
`92a1e554b9dee74150c39ddeb40918c8c65b0f0aac9ef1ab5dad44268f0c19a7`.
The reviewed full counter driver SHA256 is
`d19c480572c3fbcb0ce01afeb868efb5dc276ad64606ab01f40edd9bf2d970e0`;
its unchanged mutation/base parents are `cacda5cb...79f2` and `9cb17515...00fd`.
The native prepared-resource manifest SHA256 is
`f35140182f95862e5330fabe4c6a449805f2d83be94dce2f01e770a2ede68ce2`.
All games use fresh game/launcher/XDG homes, software rendering and disposable
worlds. Prism, playable saves, reference sources and original prepared assets
are untouched. An initial missing
sibling build receipt stopped preflight before any game started; that setup log
is retained and excluded from qualification. Independent driver review also
corrected an out-of-reach placement setup before launch.

Initial integrated attempt `b8e9e38a-83aa-4a91-a0fc-9663aa18c236` passed placement,
replacement/stale keys, exact single steps and pause/freeze, then correctly failed
the ticket-only suspension predicate. The driver requested simulation distance 2,
but supported options clamp it to at least 5; distance four remained eligible.
This run is failed/excluded, with exit 0 and no cleanup error. The corrected driver
preserves distance 5 and tests chunk distance seven (simulation ticket level 33)
with render distance nine to retain the storage neighborhood. No engine change
or weakened predicate was needed.

## Actual game results

| Mode | Run | Assertions | Result |
| --- | --- | --- | --- |
| Integrated | `ad45af86-dd8c-422b-842c-800cf527eed2` | 362 | Pass |
| In-frame | `a09f3b77-0945-41ff-a16e-5b380d8f3662` | 347 | Pass |

Raw receipts and the three exact archived drivers remain in workspace
`.local/h02d/game-runs/<run>/`. Both runs include the complete qualified paired
activation/static-state, resident mutation, piston/SDK writes, dimension and
residency controls. The counter is a separately declared captured
`paired_activation:counter` schema-1 iron-template cube at `[133,112,3]`, using a
vanilla yellow texture. It does not reuse the static marker or its state plan.

Actual item placement admits a frozen zero record. Changed writes retire and
replace identity; no-op writes preserve it. Exact stale keys and invalid bindings,
coordinates and duplicate enables refuse explicitly. Two individual simulation
steps each increment exactly once; running advances, pause/freeze hold value and
identity, and resume advances. With its complete storage neighborhood resident,
the ticket-excluded counter holds value while the world clock continues. Ordinary
unload marks it dormant; matching reload retains key/value without elapsed-time
catch-up, and a later single step increments once.

Integrated dimension travel retains scope 12/instance 3 and frozen value 19 in the
parked Overworld. Its final `integrated-exit` retirement reports value 65 at game
time 468. In-frame travel emits `dimension-switch` retirement for scope 12/instance
3, value 17 at time 174. Returning leaves the trial disabled; explicit re-enable
creates scope 13/instance 4 with zero payload, then advances normally. Final
`application-exit` retirement reports value 18 at time 232. These transient numbers
identify these runs, not a stable cross-process identity contract.

Both initial and restart game phases exit 0 with no cleanup error. Changed/missing
package phases intentionally exit 1, explicitly refuse admission and preserve all
saved world bytes. Restart loads the ordinary saved bound cube while the counter
remains disabled: no payload persistence is implied. Expected RPC errors also
include invalid/duplicate trial configuration, stale reads and inherited observer
fault/refusal probes. Independent final receipt review finds no issue and qualifies
the bounded native trial. Dedicated,
idle-level and save-error retirement paths have source review, not real-game
execution evidence.

## Broader verification

A fresh native Linux clone at the exact counter commit passes
`tools/pr_ready.py --quick --base 0264c18fd268ff369af64dcaa3dd8d94ddfdecd6`:
public tree, 369 Python tests, formatting, Clippy, 1,381 client/mod tests, doc tests
(zero selected examples), launcher formatting/Clippy and 197 launcher tests.
Cargo.lock is unchanged. This aggregates several separately reviewed local
packets; its 23-file scope warning is not a single proposed upstream change.
Vanilla parity harnesses are explicitly not run by `--quick`.

The clone initially has no prepared resources; its five content-root tests
therefore return without resource coverage. A separate explicit eight-test
paired-on run with the owned prepared data is in progress. Raw records are
`.local/h05-pr-ready.log`, `.local/h05-ci-checkout.txt` and
`.local/h05-owned-resource-tests.log`.

No inventory, guest callback, saved payload/generation, coherent checkpoint,
retained animation, Create processing conformance or performance claim follows.
See `../design/H05_NATIVE_COUNTER_PACKET.md` and `../design/H05_COUNTER_GAME_PACKET.md`.
