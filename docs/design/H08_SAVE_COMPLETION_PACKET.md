# H08 preparation: honest generation-save completion

After the qualified H05a transient counter, fix one existing save-completion gap
identified in `H08_CHECKPOINT_FEASIBILITY.md`. This is a bounded prerequisite,
not persistent machine storage, complete producer quiescence or coherent recovery.
No WIT, inventory, network, region format, checkpoint policy or permission change.

`Streaming::save(wait=true)` currently warns on a 30-second receive timeout or
disconnected worker, continues and reports success. Game then consumes dirty chunk
snapshots. Also `Streaming::wait` can have put relevant completed work in
`deferred`, while the save drains only the receive channel. A checked-out proto
is owned by its worker until its Saved result is reconciled; it cannot be copied,
cancelled or silently discarded on failure.

Make synchronous completion return an explicit error at its bounded deadline or
worker disconnection, and propagate it through the two Game callers before they
consume `World.modified`. Process relevant deferred completion messages exactly
once, preserving unrelated lighting/mesh/proto messages for ordinary update and
their order. Share reconciliation with incremental polling where practical.
Preserve normal asynchronous behavior and existing writer-error reporting.
Do not claim that plain generation jobs or fallible proto/entity/player/metadata
callbacks are all drained by this change.

Tests must prove: queued/deferred completion succeeds without false timeout;
deadline and disconnection fail explicitly before final snapshot consumption;
outstanding worker/proto ownership and dirty state remain recoverable after a
timeout; unrelated deferred work survives; successful asynchronous/incremental
save behavior remains correct. Use deterministic injected deadlines/channels or
controlled workers rather than sleeping for the production timeout. Existing
resource-backed generation-save tests and feature-on/off compile/lint checks
remain applicable. No public error-injection game control or persistent sidecar.

Stop this packet if correct ownership needs broad worker cancellation, changing
generator storage contracts, all-owner barriers or format redesign. Record the
remaining limits and re-scope before that work. Fresh independent read-only Sol
High review precedes committing the exact substantive candidate. The coordinator
owns integration, commits and the serialized Cargo target. No public engine push.
