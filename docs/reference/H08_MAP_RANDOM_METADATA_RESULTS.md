# H08a5 map and named-random metadata results

Local engine commit `c161eaa1f361cd4e423c4cb07120c2f9c1d529ab`, tree
`5c1cc7723d4ee5ea7f53906f4ea3d65f53b3c6b0`, branch
`codex/h08-map-random-metadata`, changes game.rs and its maps/trading modules.
Map and random metadata errors propagate before the existing save-success message.
Failed/unvisited map flags and the map index remain retryable; a successful map
file clears only its own dirty flag. Remote saves remain no-ops.

Random saving builds a separate world clone plus all stored local/LAN merchant
loan states, without advancing, returning or draining live state. Identical
persisted RNG words can share an id; conflicting owners produce an explicit error
before random-file writing. Empty current collections serialize their defaults and
empty sequence map, preventing old data from returning after a no-loans reset.
Live ownership/reset-during-loan semantics are not repaired by this packet.

Fresh independent read-only Sol High review finds no issue in the exact candidate,
callers, loan collection, persisted state comparison and failure paths. Reviewed
source SHA256:

| File | SHA256 |
| --- | --- |
| game.rs | `1dce310b309d58dd93bf692de168432ffb24f3435eca95169996ac619c395d72` |
| game/maps.rs | `751cc3ae1cff4316e15d9adbe5cd5eebe8b99c6e3855138c39ae5bbb644c0370` |
| game/trading.rs | `272accee5ba11c08cea888c09c7d8f11f4719970e5d2212de7169e5a96966a58` |

All 144 binary tests pass (three ignored), including seven new resource-free cases
for real blocked _new writes, repair/retry, retained map/index/live loan states,
identical/conflicting streams, empty/reset defaults and remote no-write behavior.
The named-random library regression passes. Formatting, feature-on/off library/
binary/test Clippy and the normal public-tree hook pass. Broader quick verification
passes in the immutable native Linux clone at c161eaa: 369 Python and 1,318 client
tests, formatting, workspace/feature-off lint, zero doc tests and public-tree checks.
Cargo.lock is unchanged. Vanilla parity is explicitly skipped by --quick, and
content/launcher/theme checks are unchanged-scope skips. Raw records remain
`.local/h08a5-*.log`.

Frozen optimized game SHA256
`a8570b081c3c754f1b5760eab52b24c8f0ff3e26f03d88033955d9640a458cbf`
passes the unchanged extended counter driver in both modes:

| Mode | Run | Assertions | Final transient counter |
| --- | --- | --- | --- |
| Integrated | `1e1b15fb-540d-4f05-907b-0602b5cf3d45` | 362 | 65 |
| In-frame | `91f3d9c2-14bb-4e33-b92d-e47404819683` | 347 | 19 |

Both normal initial/restart processes exit zero with no cleanup error. Expected
changed/missing-package refusal processes exit one and preserve world bytes.
Integrated missing-package and in-frame changed-package cleanup attempts encounter
an already-removed control socket; other refusal cleanup records are null. These
are recorded refusal/quit races, not normal shutdown failures. The driver SHA256
remains `d19c480572c3fbcb0ce01afeb868efb5dc276ad64606ab01f40edd9bf2d970e0`;
launcher-session environment is removed before invocation. These are regressions
of counter/lifecycle/save paths, not game-driven map/RNG failure injection.
Independent read-only receipt review confirms source/build/run attribution,
checks, retirement receipts, exits and recorded refusal races; it did not recompute
the Linux executable hash from its bytes.

Conflicting live merchant streams remain intentionally unsavable. This is safe
refusal without choosing an ownership winner; it is not support for every merchant
state. Metadata files are not atomic, write_dat replacement windows are unchanged,
autosave HUD text is not a region-writer durability barrier, other metadata helpers
still need error coverage, and player/native-entity ownership, coherent checkpoints
and G1 remain open. No engine push occurred.
