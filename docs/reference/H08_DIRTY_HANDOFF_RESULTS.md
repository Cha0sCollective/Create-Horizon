# H08a8 direct dirty-marker handoff results

Local engine commit `b9b32da57e124221110c82b28a3fd95e0624cf4a`, tree
`42f0d4b50b0bc7f5ae5ce4750c3abec164fdde96`, branch
`codex/h08-dirty-handoff`, changes only save.rs and game.rs. Fresh independent
read-only Sol High design and stable code review finds no issue in the concrete
API, actual callers/queue/ownership paths or five resource-backed tests.
Reviewed source SHA256:

| File | SHA256 |
| --- | --- |
| save.rs | `ce6e4cc07dacad6813f56f4b95b96656b8d20b9013c0fefeacf7963fbf3eb35f` |
| game.rs | `7a851935589e6464f29679f703554263acb616a6ad23ef36ee987e175eaf030d` |

Game's direct level save builds the same borrowed with_entities snapshots without
draining modified keys. SaveDir::save_modified_later calls the real queue operation
before removing only submitted coordinates. Errors leave every dirty key; success
preserves unrelated/absent-chunk keys. The small public Rust helper crosses the
library/binary boundary, without a WIT or persistent-format change.

All eight explicitly enabled owned-resource save tests pass: the five new handoff
cases plus three strict-POI regressions. The new cases require pinned resources
and fail rather than silently skip if missing. They establish:

- Successful/empty submission, exact submitted-key release, retained other keys,
  original snapshots and fresh Region/POI disk round-trip.
- Real disconnected-send error, count reconciliation, retained original Arc and
  repeated sticky writer/barrier errors.
- Poisoned pending lock before acceptance, with retained dirty keys and no
  invented sticky writer fault.
- Real Region acceptance then POI enqueue failure: controlled real writer,
  bounded state waits, poisoned pending lock and released interlocks before
  joins/assertions. Every marker remains and the Region Arc is retained; a fresh
  SaveDir reads its completed Region write. The poisoned original cannot use
  normal pending reads, and its flush can legitimately succeed.
- Accepted asynchronous disk failure: submitted markers clear, failed pending
  ownership remains, and repeated flush/poll calls report the writer error.

The 37 regular save tests pass (eight explicit resource tests ignored there),
three generation/save regressions pass, and all 153 game tests pass (three
ignored). Feature-on/off library/binary/test Clippy, formatting and normal
public-tree checks pass. Raw records remain `.local/h08a8-*.log`.

Frozen optimized game SHA256
`5691d4efaa5e24157fdb7f38d7e373670ba91788fa4dc8a38a2fb23943b6b0b5`
passes the unchanged extended counter driver:

| Mode | Run | Assertions | Final transient counter |
| --- | --- | --- | --- |
| Integrated | `64c7d520-1088-41dc-993a-160c1bcdf91a` | 362 | 61 |
| In-frame | `57ad47e3-06fe-4fb1-892c-aa51d8599252` | 347 | 17 |

All normal initial/restart processes exit zero with null cleanup errors. Expected
changed/missing-package refusals exit one and preserve world bytes. Only integrated
missing-package cleanup attempts an already-removed socket; other refusal cleanup
records are null. Driver SHA256 remains
`d19c480572c3fbcb0ce01afeb868efb5dc276ad64606ab01f40edd9bf2d970e0`, with
launcher-session environment removed. Game checks regress inherited counter/
lifecycle/save behavior; real queue faults are exercised by the targeted tests.

Independent read-only receipt follow-up confirms unchanged source/build/driver
attribution, all targeted logs, both game counts/exits and raw retirement records.
It does not independently rehash the Linux executable or inspect the parent's
top-level process-session completion; those remain coordinator/driver evidence.
Broad quick qualification passes in the immutable native clone at b9b32da:
369 Python/1,331 client tests, formatting, workspace/feature-off Clippy, zero doc
tests and public-tree checks. The five new resource cases are explicitly run
above, not counted as default quick passes. Cargo.lock is unchanged; content/
theme/launcher are unchanged-scope skips and parity is explicitly skipped by
--quick. This protects chunk
enqueue acceptance, not complete save success or durability. Later entity/raid/
barrier errors can occur after accepted chunk markers clear. switch_level's
separate dirty drain, ordinary unloading and raid/player/entity retirement remain
unchanged. Counter payload is transient, coherent checkpoints/G1 remain open,
and no engine push occurred.
