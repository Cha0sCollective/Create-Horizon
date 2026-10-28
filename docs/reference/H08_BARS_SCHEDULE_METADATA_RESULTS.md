# H08a7 bossbar and scheduled-event save results

Local engine commit `dcea02b5fbf58b895c6052f316a6d0a75b97dae9`, tree
`d769407b0eb3af50f180372604c7c773a24a906d`, branch
`codex/h08-bars-schedule-metadata`, changes only game.rs, game/bossbars.rs and
game/functions.rs. Fresh independent read-only Sol High review finds no issue in
the exact diff, callers, common/incremental/integrated/dedicated/control save
propagation, writer semantics, schemas, schedule parser and six new tests.
Reviewed source SHA256:

| File | SHA256 |
| --- | --- |
| game.rs | `a3ab01d5ae0c2c958236fd07e36ebb5619ab32f7c99828e9da7461f64a44a485` |
| game/bossbars.rs | `21c8e764d9c24394cd3967f9008205ff9623c9e20424f15d908bde9358ed53f6` |
| game/functions.rs | `d939488ce2c506a624767e2502b73ecdc7ef3f8301ba1a4b896f202b503a9d1a` |

Both saves return actual errors through the common metadata save before its
success message. Borrowed bossbar UUID/viewer state and live schedule ordering,
sequence counters and flags remain unchanged. Snapshot sorting, ordinary NBT,
loaded/remote no-ops, empty existing-file overwrites and sparse new empty files
are preserved. Schedule loading moved mechanically into a private testable seam;
tolerant reading, append/reconstruction and the existing /reload policy stay intact.

All 153 game binary tests pass (three ignored), including six new resource-free
blocked _new failure/repair/retry, last-entry deletion and guard/sparsity cases.
Tests check ordinary NBT, runtime UUID/viewer retention and real-parser schedule
ordering, including equal-time events whose lexical order differs from sequence
order. Feature-on/off library/binary/test Clippy, formatting and normal public-tree
checks pass. Raw records remain `.local/h08a7-*.log`.

Frozen optimized game SHA256
`9421b04a1bea650e5d5f6887c17ccca0f0e90dba1d7f0e57774aa08eb8f4f5f0`
passes the unchanged extended counter driver:

| Mode | Run | Assertions | Final transient counter |
| --- | --- | --- | --- |
| Integrated | `0b4bbf5d-9d5c-45ab-b35b-0426e057a59c` | 362 | 62 |
| In-frame | `a7549be7-4430-49ed-9652-28482f7ccc03` | 347 | 25 |

All normal initial/restart processes exit zero with null cleanup records. Expected
changed/missing-package refusals exit one and preserve saved world bytes. Only the
integrated changed-package cleanup attempts an already-removed control socket;
all other refusal cleanup records are null. The unchanged driver SHA256 is
`d19c480572c3fbcb0ce01afeb868efb5dc276ad64606ab01f40edd9bf2d970e0`, invoked
with launcher-session environment removed. These game checks regress existing
counter/lifecycle/save behavior; the targeted tests exercise the new write errors.

Independent read-only receipt follow-up confirms source/build/driver attribution,
counts, targeted logs, exits and explicit retirement counters. It did not
independently recompute the Linux executable hash. Broad quick verification passes
in the immutable native clone at dcea02b: 369 Python and 1,331 client tests,
formatting, workspace/feature-off Clippy, zero doc tests and public-tree checks.
Cargo.lock is unchanged; content/theme/launcher are unchanged-scope skips and
parity is explicitly skipped by --quick. Metadata atomicity,
world-clock merge reads, other save errors, dirty-marker/retirement ownership,
coherent checkpoints and G1 remain open. Counter payload remains transient; no
engine push occurred.
