# H08a10 borrowed dirty metadata results

Local engine commit `19c86308d069d4d6a316cba72406336a8ccdb86b`, tree
`4764b36738453ebf3da4131eae9d6f856c5703ca`, changes only game.rs and its dragon,
wandering_trader and nether_portal modules. Fresh independent read-only Sol High
source, test-only delta and receipt review finds no issue. Final source SHA256:

| File | SHA256 |
| --- | --- |
| game.rs | `546738235b800b5598c2da739175220fd8d687a7c5667306a7b3dc786f07f4aa` |
| game/dragon.rs | `34e8bb9ac481a1cebfa38f6ab7df11a350878e3ce2e7a17dde9e8f780b39aeca` |
| game/wandering_trader.rs | `f5307aa1fdb7b07810b8fff68804311bf50ac2fba33c7dc930ee3a963522aa2a` |
| game/nether_portal.rs | `bb938f99f7f3bb94138414e17be77cb4734581189be28412757e8c0e9bbc1e51` |

Fight, trader and pending-arrival saves return contextual errors through common
metadata before its success message. They borrow payloads and clear dirty state
only after success. Existing schemas and guards remain: no new fight/trader remote
guards, and dirty arrivals still save when unloaded. Arrivals keep the existing
root arrivals list without a new wrapper. The eager dragon-killed save warns on
failure after committed effects and retains dirty state for ordinary retry.

All 164 game tests pass (three ignored), including eight new resource-free blocked
_new failure/repair, exact NBT, runtime/payload retention, optional-field, guard,
opaque nested arrival/parser and deletion cases. Preserve initial failed test run
163 passed/one failed: its expectation truncated negative crystal Y to -65, while
the baseline and candidate both floor -65.9 to -66. Only the expected literal was
corrected, independently reviewed, then the full binary and on/off lint checks
were rerun. Production serialization never changed. Formatting/public-tree pass.

Frozen optimized game SHA256
`2ce695b1acb5f4e13d3a58fca82eba8dc2a7ab11c012ee6f5ebf629cab8b0832`
passes the unchanged extended counter driver:

| Mode | Run | Checks | Final transient counter |
| --- | --- | --- | --- |
| Integrated | `04b52b69-98ff-4775-8c42-ffe2599d7f0b` | 362 | 63 |
| In-frame | `051ac4b9-1aa0-425f-9bf8-612362820c7a` | 347 | 21 |

Normal initial/restart exits are zero with null cleanup errors. Expected package
refusals exit one and preserve saved bytes. Only integrated missing-package cleanup
finds an already-exited game; other refusal cleanup records are null. Driver SHA
remains d19c4805... (full CURRENT_WORK pin), with session-file environment removed.
These are inherited counter/lifecycle/save regressions, not actual dragon-death or
portal-transfer conformance. Native tests establish the new failure/retry cases.

Broad quick checks pass in the immutable clone at 19c8630: 369 Python/1,347 client
tests, format, workspace/feature-off lint, zero doc tests and public-tree checks.
Cargo.lock is unchanged; soft scope warning is 560 insertions/66 deletions across
four files including tests/moves, with no PR proposed. Content/theme/launcher are
unchanged-scope skips; parity is explicitly skipped. CI update and source inspection
resolve the footer's merge-base label. Composite counter/quick exit codes are zero.

Receipt review cross-checks source/build/driver/run identities, final counters,
raw retirement, native results, exits and the preserved initial failure. It did
not execute candidates or rehash inaccessible frozen Linux binary bytes; matching
build/environment/results and driver validation support that attribution.

Fight state can still survive same-process world changes through its loaded guard;
portal entity/arrival removal and exit resolution remain ownership boundaries.
This packet does not repair those paths, atomicity, persistent counter payload,
coherent checkpoints or G1. No engine push occurred. Raw records remain .local/
h08a10-* and the named game-run folders.
