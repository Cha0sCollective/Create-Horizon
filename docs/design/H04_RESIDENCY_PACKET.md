# H04c bounded authoritative residency observations

The counter's next lifecycle prerequisite is whole-chunk load/unload, which bypasses
World::set. Ordinary authoritative streaming has two seams: receive_generated
inserts both generated and saved final chunks; update removes distant chunks.
Capture these committed changes with small World wrappers. Do not infer membership
from revision changes (same-key replacement does not increment them), and do not
interpret temporary SDK ChunkMap lending as unload.

Add a separate feature-gated, default-disabled World-owned passive observer. It
watches at most 64 explicitly supplied chunk coordinates and a bounded event queue
using the existing checked-sequence/sticky-fault pattern. Duplicate enable refuses.
Enable returns a sorted current residency snapshot distinguished from later
history; it neither loads chunks nor emits fabricated historical load events.
Events capture immutable AuthorityScope, coordinates, actual World.game_time,
sequence and loaded/unloaded/replaced kind. Same-key insertion is replaced;
absent removal is a no-op. Overflow/exhaustion never rejects an already committed
world change, and drain never returns a faulted partial batch as complete.

Route only the two ordinary authoritative streaming writes through the wrappers.
Replica insertion/forget stay explicitly separate and default-disabled; initial
scene/unit/runtime fixture inserts do not establish ordinary lifecycle coverage.
Keep H04b's state-observer API and behavior unchanged. Do not retain thousands of
whole Chunk snapshots, add a generic event framework or expose a guest capability.

Coordinate events prove the seam; they cannot reconstruct a machine that loads and
unloads before consumption. A later record consumer must initialize synchronously
at admission or retain an appropriate bounded snapshot. No records, generations,
inventories, callbacks, saves or simulation-active claims belong to this packet.
Observation travels with World across swaps, and explicit teardown ends it. World
destruction drops it; no Drop callback or invented per-chunk unload delivery.
Owner-controlled lifetime retirement must precede complete H04 record claims.

Test watched filtering, sorted pre-enable snapshot, generated/saved insertion,
replacement, absent removal, unload/reload, exact captured scope/time/order,
overflow/exhaustion without failed writes, World swaps/default-disabled and a
temporary take/restore of ChunkMap without false events. Verify actual streaming
hooks using a focused existing-resource test when practical. Actual game control
wiring/probes are a separate coordinator follow-up before runtime qualification.

Eligibility follows later: tickets.block_ticking AND ticking_chunk (lit with 3x3
resident neighbours), plus explicit simulation-running/pause/parked handling.
Tick registration and loaded_ticking alone are not active/inactive lifecycle.

One isolated worker owns new residency module, World APIs, two streaming hooks and
focused tests. Coordinator owns integration/commits and must release shared Cargo
after actual game runs. Fresh independent read-only Sol High review is required.
Keep it small enough to review; private planning material stays outside Horizon.
