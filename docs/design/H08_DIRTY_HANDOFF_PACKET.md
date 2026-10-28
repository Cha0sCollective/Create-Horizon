# H08a8 direct save dirty-marker handoff

Planned after H08a7; scope game.rs::save_level and save.rs only. Direct save_level
currently drains World.modified before SaveDir accepts its resident snapshots.
Use the existing with_entities snapshots, keep dirty keys until acceptance, and
clear only coordinates actually submitted after successful enqueue. Preserve
unrelated or absent-chunk markers. This matches incremental closing-save ordering.

Choose one concrete SaveDir::save_modified_later helper taking the dirty HashSet,
ordinary Arc<Chunk> snapshots, registry and biomes. It calls real save_later before
releasing submitted coordinates. Public visibility is required across lib/bin;
this is a narrow experimental internal Rust API, not a WIT or supported host ABI.
Do not introduce a generic closure transaction framework.

Test the production helper with explicit owned-resource fixtures inside save.rs,
using existing private writer access for actual disconnected-channel and poisoned
pending-lock failures. Verify original Arc/state and all dirty keys survive error,
including any partial acceptance that returns Err. Successful enqueue clears only
submitted coordinates, preserves unrelated/missing keys, and ordinary persistence
round-trips the intended snapshot. A separately induced asynchronous disk failure
must demonstrate pending/error retention after accepted markers are cleared:
acceptance is not durability. Disk-path blockage alone does not prove pre-enqueue
retention. Do not substitute the infallible generation_saves Recorder mirror.

Leave switch_level's separate synchronous dirty drain and ordinary unload outside
this packet; unload also removes native entities/block entities and logs errors.
Marker retention alone cannot restore those owners. Raid transitions are likewise
separate, especially in-frame lazy state replacement and portal player ownership
on transition error. Do not claim all transition/retry paths are repaired.

Require fresh independent read-only Sol High review, focused resource-backed
failure tests, normal save/generation regressions, feature-on/off checks, frozen
fullcounter in both modes and broad regression checks. Coordinator owns serialized
Cargo and commits; no public engine push or checkpoint/G1 promotion.
