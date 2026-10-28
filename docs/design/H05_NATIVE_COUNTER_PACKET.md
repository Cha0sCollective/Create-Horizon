# H05a one transient native typed counter trial

After bounded residency qualification, advance the original S1 experiment with an
actual inventory-free host record. This is one explicitly opted-in H03c-bound cube
and position, not general persistent machines or a Create behavior. Do not add
another passive observer merely to count phases.

The World owns the committed type/schema, canonical block state, position, checked
transient instance key, u64 counter and last advanced game time. Configuration uses
captured type metadata; the later Game controller must validate the selected block
binding/registry against that catalog before enabling. No static marker reuse.
No inventory, Wasm callback, sidecar, schema migration or saved generation.

Maintain the one-position record synchronously at existing seams:

- Successful World::set admission/removal/replacement; no-op preserves identity.
- Streamed admission reconciles immediately while the chunk exists. Ordinary
  reload can resume a retained dormant record only when its expected bound state
  matches. An unexpected reload mismatch must be explicit, not silently reset.
- Whole-chunk replacement retires the prior instance before a new admission.
- Streamed removal marks the retained record dormant before unload consumers.
- Temporary SDK map lending has no effect.

No world scan or retained Chunk snapshot is needed. Keep one bounded last-retirement
receipt for diagnostics, not a delivery queue. Scope plus instance distinguishes
stale keys across replacement/reconfiguration. Arithmetic/allocation exhaustion
must be explicit and sticky; preserve the last committed counter and ordinary
world writes. Removing a block still retires a faulted record.

Native advance is exactly one per eligible phase, with checked arithmetic and a
duplicate-game-time guard, never elapsed-time catch-up. Accept running/eligibility
from the later authoritative Game hook and recheck residency/bound state. The hook
belongs at tick_world's tail after current native writes, gated by running,
tickets.block_ticking and ticking_chunk (lit with 3x3 resident neighbours).
Pause and skipped parked phases do not advance. Do not expose a stale cached
active flag as current simulation state.

Expose explicit take/retire returning the final snapshot before World destruction.
Retain through World swaps. Later Game wiring handles successful switch/open/close,
idle level removal and shutdown, without Drop callbacks or masking save errors.
Restart is intentionally disabled/nonpersistent for this trial.

Split review units: first a small counter_trial module, World ownership/hooks and
native tests; then per-level phase, typed configuration/retirement controls and
actual two-mode tests. Tests cover placement/no-op/duplicate phase, stale keys,
replacement, loaded-but-ineligible/frozen suspension, dormant reload without
catch-up, mismatch faults, exhaustion, SDK lending, swaps and explicit retirement.
Coordinate observations alone are not the record consumer.

This remains a bounded diagnostic experiment under G0. H05 persistence, H06/H07
transactions/slot, H08 coherent checkpoints and G1 stay open. Coordinator owns
integration/commits and later Game proof; isolated worker owns only module/World
changes. Require fresh independent read-only Sol High review. Keep Cargo stopped
until residency game runs complete and coordinator releases the target.
