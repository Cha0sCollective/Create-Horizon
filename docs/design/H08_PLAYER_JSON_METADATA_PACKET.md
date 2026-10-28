# H08a11 checked borrowed player JSON metadata

Planned after H08a10. Bound scope to game.rs, game/stats.rs, game/advancements.rs,
game/lan.rs, minimal game/integrated.rs cleanup and handling of the existing
game/dedicated.rs caller. Check
borrowed statistics/advancement writes in common and integrated-owner metadata
saving. Do not claim all guest retirement/shutdown ownership is repaired.

Stats::write returns Result, preserving its existing {stats,DataVersion} JSON.
Use one concrete checked advancement writer for local and LAN records, retaining
Progress::to_json(tree, DATA_VERSION). Propagate local save_stats/save_advancements,
save_guest_records and store_owner results through common metadata before success.
Integrated servers can skip local-player writers, so the owner path is required.
This is private binary Rust plumbing, without a public library/WIT/schema change.

Evaluate local stats/advancements and owner records at their existing positions,
but retain these JSON results until remaining existing Level::write and world-clock
metadata attempts finish. Return the first JSON error in execution order before
the saved message. Keep existing non-JSON short-circuit behavior and precedence.
This prevents an owner JSON error from newly skipping required world metadata.

Preserve all identity/guard behavior: local/integrated-owner records use the
existing level UUID; other guests use their own converted UUID; missing identity
creates no filename. Keep local loaded/remote/no-player guards. Guest records
still require guest.placed and a nonempty advancement tree for BOTH records; do
not split that pre-existing gate as an incidental behavior change. Missing host
play state still skips its statistics record. Preserve advancement-then-statistics
order and attempt both independent records even if one fails before returning a
checked error. Do not introduce new loss at an already unretained guest boundary.

Current save_guest and shutdown/disconnect flows remain void/log-only boundaries.
Explicitly log checked record errors there; preserve later player/world saving,
kicks and cleanup attempts. Do not add a naive question mark that exits teardown
early. stop_dedicated's existing Result is not permission to skip those attempts.
Integrated shutdown must capture save_game's result, call lan_close(false), and
capture save.flush regardless of save failure before returning. Preserve the save
error first and log a secondary flush failure; return flush failure when saving
succeeded. Release the explicit session-lock field only when both succeeded; keep
it owned until server Game drop on error. Do not unconditionally unlock a failed
or poisoned writer. The server owns/drops its Game before thread completion.

The separate void playerdata writer, removed guests, all-owner snapshots and
retained failed shutdown data remain future packets.

Saving must not clear Stats network dirty entries or scoreboard events. Preserve
values, loaded flags, advancement criteria/tree Arc identity, changed/unlocked/
announced state and presentation queues. Keep zero statistic resets and empty/
revoked advancement projections persistable over previously saved progress.
No new strict loading, JSON format, atomic replacement or generic I/O framework.
Include contextual paths in mkdir/serialization/write errors.

Use resource-free concrete state/path seams and real directory/leaf obstructions,
repair and retry with the same borrowed state. Parse exact JSON and verify state
retention after failure/success, last-statistic zero and last-criterion revocation,
all existing guards with sentinel files/directories, canonical signed-int UUID
formatting, shared LAN advancement output and actual integrated-owner selection/
checked propagation. Exercise both guest record attempts when the first fails.
Do not build a renderer/Game fixture or duplicate an unchecked LAN JSON writer.

Add a focused private real integrated-owner JSON fault probe using qualified base
control/readback helpers without modifying qualified drivers. Baseline save, then
make distinct owner inventory/block/clock mutations. An owned JSON-leaf obstruction
must return a contextual synchronous save error while the process stays alive,
yet parsed level/player/clock records must contain those world-save mutations.
Apply another distinct mutation and quit under the persistent obstruction; require
natural failure without forced termination. Repair only the owned obstruction,
reopen and verify shutdown-era owner/world changes. This tests real routing; natural
exit/reopen establishes termination and eventual lock release, while source review
checks each cleanup call's ordering. Use canonical owned paths, archive/restore
only harness-owned bytes, fresh data homes, internal session-file sanitizing and
the reviewed inherited display connection. No new debug API/test framework.

Direct fs::write can truncate or partially write before failing: do not promise
unchanged old bytes for arbitrary write errors. Fixtures may prove only their
specified pre-open obstruction behavior. Borrowed-state retries do not establish
atomic JSON, unknown-field preservation, guest retention or coherent checkpoints.
Require fresh independent read-only Sol High design/source review, targeted tests,
feature-on/off checks, frozen fullcounter in both modes and broad regressions.
Coordinator owns serialized Cargo/commits. No public push or gate promotion.

Fresh independent read-only design review found an initial P1 gap: store_owner
errors could reappear during stop_integrated after its earlier log-only save_guest,
skipping world metadata and cleanup. The revised deferred-JSON/common ordering and
minimal integrated cleanup above close that design gap; no owner decision is needed.
Implementation and runtime qualification remain pending.
