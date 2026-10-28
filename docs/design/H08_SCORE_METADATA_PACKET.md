# H08a6 scoreboard metadata save errors and retry

After H08a5, bound the next save prerequisite to game.rs, game/scoreboard.rs,
command_storage.rs and stopwatches.rs. No other metadata or player/entity
retirement changes. Preserve file schemas and remote/unloaded no-op behavior.

CommandStorage::save and Stopwatches::save must return their actual write errors.
Clear dirty state only after all required writes succeed. Command storage's one
global flag may conservatively retry earlier successful namespaces after a later
failure; no per-namespace tracking is needed. Removing the last entry/watch must
remain persistable and retryable. Stopwatch saving must not change live creation
times or accumulated values; a private explicit-now seam can make tests stable.

Game::save_scoreboard must propagate storage, stopwatch, border and scoreboard
errors through save_game_metadata before its success message. Preserve the normal
order. A border reset to default must overwrite an existing nondefault border
file; skipping only a default value with no existing file preserves sparse saves.
Keep loaded in-memory state on failure and use ordinary retries. Do not broaden
this into transactional cross-file rollback or change tolerant loading behavior.

Use real blocked _new paths in isolated temporary directories. Cover a namespace
failure after an earlier success, retained global dirty state and exact repair/
retry; deletion persistence; last-watch removal failure/retry and unchanged live
watch state; border and scoreboard failures and exact retry; default border and
empty scoreboard overwrites; unloaded/remote no-write guards. Existing roundtrips
must check Result. Run focused tests, feature-on/off lint, frozen fullcounter in
both game modes and the appropriate broad regression checks, serialized by the
coordinator. These game checks cover inherited save/lifecycle behavior; targeted
tests establish the new error cases.

Stop/rescope if this needs player/entity ownership repair, generic I/O frameworks,
new persistence formats or runtime timer policy changes. Metadata remains nonatomic,
other save helpers and coherent checkpoint publication/recovery remain open.
Require a fresh independent read-only Sol High review of the exact candidate.
No public engine push, PR, release or new gate promotion.
