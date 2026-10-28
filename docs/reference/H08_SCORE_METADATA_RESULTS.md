# H08a6 scoreboard metadata save results

Local engine commit `1d144d017b7d238293353e5c25c7f21080da4944`, tree
`aa4b44f95ccef1f73d023ee86b22c1255ba3e0f1`, branch
`codex/h08-score-metadata`, changes only command_storage.rs, stopwatches.rs,
game/scoreboard.rs and game.rs. Fresh independent read-only Sol High review finds
no actionable issue in the exact candidate, callers, writer semantics, loading,
border packing/comparison and failure tests. Reviewed source SHA256:

| File | SHA256 |
| --- | --- |
| command_storage.rs | `dfee6ab0aaa3e44f4297db1ca3a6bfbc5f8321de041214498350139ec24119b6` |
| stopwatches.rs | `b0604ba778cbc3f8491fff0edcc90578df5befb56f9894741650ccaa1ee7e09e` |
| game/scoreboard.rs | `80b09b831aad90aeea9835a434cf48ea6fe5417d444496519098615979341cf7` |
| game.rs | `0f8202a2f71ef0760c1629a49c079170187b2cb73113a4854a2553f8e3220aa7` |

Command storage keeps its global dirty flag until all namespaces succeed; a retry
may rewrite earlier successful namespaces. Stopwatches keep dirty state through a
failed write, including removal of the last watch, and preserve live timing fields.
Clean nonempty watches continue saving elapsed time. Scoreboard saving propagates
all four component errors in the original order before the Game success message.
An existing border file is overwritten after reset to defaults. Remote/unloaded
no-ops, sparse fresh defaults and ordinary on-disk schemas are preserved.

Targeted checks pass: three command-storage tests, two stopwatch tests, all 147
game binary tests (three ignored), feature-on/off library/binary/test Clippy,
formatting and the normal public-tree hook. Seven tests are new and use actual
blocked _new paths, retained state, repair/retry, exact ordinary NBT, empty resets,
component ordering and no-write guards. Raw logs remain `.local/h08a6-*.log`.

Frozen optimized game SHA256
`d86ad4e265c672ec8c79808d60ae90cac9cd9e8df82eab1b2f86a80a9103932e`
passes the unchanged extended counter driver in both modes:

| Mode | Run | Assertions | Final transient counter |
| --- | --- | --- | --- |
| Integrated | `640a3687-dbbe-4e21-9c9e-1550ac5f9854` | 362 | 66 |
| In-frame | `8bf723e6-8b64-4a97-9309-917508e7f098` | 347 | 26 |

All normal initial/restart processes exit zero with null cleanup records. All
expected changed/missing-package refusals exit one and preserve world bytes.
Both integrated refusal cleanup attempts encounter an exited game or absent
socket; in-frame changed-package cleanup encounters an absent socket, while its
missing-package cleanup is null. The driver SHA256 remains
`d19c480572c3fbcb0ce01afeb868efb5dc276ad64606ab01f40edd9bf2d970e0`, invoked
with launcher-session environment removed. These game runs regress the existing
counter/lifecycle/save paths; the targeted tests exercise new metadata failures.
Independent read-only receipt follow-up agrees with source/build attribution,
archived driver hashes, counts, exits and explicit counter retirement. It does not
independently recompute the Linux executable hash. Counter payload stays transient.

Broader quick verification is pending. This is not
transactional metadata, a durability barrier for the autosave HUD, general save
error coverage, player/entity ownership recovery or a coherent checkpoint. Other
metadata, raid transition and direct dirty-marker handoff gaps are recorded in
H08_CHECKPOINT_FEASIBILITY. G1 remains open; no engine push occurred.
