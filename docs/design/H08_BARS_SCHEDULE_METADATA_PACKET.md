# H08a7 bossbar and scheduled-event save errors

After H08a6 qualifies, bound the next borrowed-metadata prerequisite to game.rs,
game/bossbars.rs and game/functions.rs. Their save helpers have one caller each
in common save_game_metadata. Return actual write errors through those calls
before success is announced; keep existing file formats and save order.

Preserve unloaded/remote no-ops and empty fresh-file sparsity. Empty live state
must overwrite an existing file so removed bars/events do not return on reopen.
Use fallible existence checks for skip decisions. Save borrowed bossbar data and
a separately sorted schedule snapshot without changing runtime bar/viewer UUIDs,
live event order, next_seq or loaded flags. No generic I/O framework or dirty flags
are needed. Private state/path seams may support resource-free tests.

Exercise actual blocked _new writes, retained live state, repair and exact ordinary
NBT retry for both helpers. Bossbar data covers names, viewers, values and style/
flags; runtime UUIDs are not persisted. Schedule data covers unsorted equal-time
events with distinct sequence values, function and function-tag callback encoding,
and ordering by time/sequence. Cover failed last-bar/event deletion then successful
empty overwrite, unloaded/remote no-write guards and fresh empty sparsity.
Preserve existing tolerant loading and reconstructed schedule order; exact numeric
sequence counters are not a persistence promise.

Existing reload_datapacks can reload an old saved schedule when the live vector is
empty. Changing that runtime /reload behavior is outside this save-error packet.
World clocks need a separate strict merge-read policy; player/entity retirement,
raids transitions, direct dirty-marker handoff and coherent checkpoints remain
open. This packet does not establish metadata atomicity or recovery.

Coordinator owns source integration, serialized Cargo, frozen both-mode fullcounter
checks and appropriate broad regressions. Require fresh independent read-only Sol
High review. No public engine push, release, promotion or permission expansion.
