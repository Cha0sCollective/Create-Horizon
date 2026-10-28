# H08 checkpoint feasibility audit

Read-only code audit against engine 0264c18 plus the H04 observer additions.
These findings support planning; no coherent checkpoint or recovery qualification
has been implemented. H04, type binding and inventory-free record work can proceed.
Do not claim G1 persistence from ordinary saves or a machine sidecar.

## Existing seams

- `game.rs::save_game_with` saves active level, parked levels, then metadata.
  `game/levels.rs::save_parked_levels` saves other dimensions sequentially.
- `save.rs::SaveDir::{flush,writes_finished}` are reusable per-directory barriers
  with retained errors. They do not supply one epoch across owners/dimensions.
- `save.rs` and `horizon-world/region.rs` update live region files in place.
  Previous checkpoints must use physical or genuine copy-on-write copies;
  hardlinks to live region files are unsafe.
- `save.rs::write_dat` rotates `_new`/`_old` files without file/directory sync or
  cross-file publication. A completed rename is not the proposed durable epoch.
- `lan/playerdata.rs::save` discards directory/write failures. Integrated-owner
  inventory belongs to level metadata; other players use separate player files.
- `game/persist.rs::save_entities` and chunk unload have separate snapshot
  producers. Failures can be logged after removing live state.
- `streaming.rs::save(wait=true)` warns and continues after its timeout, returning
  no failure. Reuse polling with an explicit failing deadline instead of inferring
  producer quiescence from save_game success.
- Carried/crafting menu stacks need normalization or inclusion before snapshot.
  `game/lan_actions.rs::guest_carried_back` is an existing normalization seam.
- `game/saving.rs` freezes in-frame simulation while preparing snapshots.
  Integrated pause still handles connections: use the existing server-hold fixture
  barrier or a bounded authoritative blocking operation for this experiment.

## Proposed bounded experiment

For fresh, size-bounded opted-in test worlds, freeze all participating writers;
drain generation with a failing deadline; snapshot every item owner; flush all
dimensions; physically copy and synchronize the complete world into a new
immutable directory; publish a complete manifest last. Retain the previous
complete checkpoint. Recovery chooses a complete checkpoint and copies it into a
fresh working directory before writable open. Ordinary MCA/NBT formats can stay.

This requires separate reviewed packets:

1. Producer quiescence and propagation of participating save failures.
2. Immutable publication/recovery, including interrupted-copy/publication tests.
3. Host-record and transfer integration with faults at player/entity/machine
   boundaries, in both startup modes.

No schema, retention policy, save cadence or performance budget is accepted by
this note. Existing ZIP backup traversal may inform implementation, but is not
durability or coherent recovery evidence. Selective journals and versioned-region
redesigns would expand scope materially and need an owner decision before work.
The bounded experiment above can be investigated before that decision is needed;
it must still prove the specified completed-checkpoint conservation contract.

## Later bounded producer audit

The inspected compiled Persistent/Vanilla/Test/Flat providers keep Generate,
Decorate and Finish jobs in memory or perform saved-data reads. Job::Save queues
proto records; owner-side finished-result reconciliation can queue finished chunk
records. LOD/structure search uses reads or in-memory caches; pregeneration pumps
the ordinary owner path. With the owner genuinely frozen, tracked write-capable
work reconciled and every participating SaveDir flushed, additional plain
generation drain is not required merely to prevent world-file writes. This is a
source conclusion for these built-ins, not arbitrary ChunkGenerator code.

Capture must prevent subsequent streaming update/reconciliation, pregeneration and
mutating control tasks. Tick freeze is insufficient, and server-hold still serves
control tasks; use a blocking owner operation or an equivalent admission barrier.
H08a1/a2/a3 improve generation completion/errors but do not establish that barrier.
Strict writer-side POI reads, suppressed metadata writes and player/native-entity
retirement ownership remain separate prerequisites. No checkpoint is qualified.

## Remaining metadata audit after H08a5

The c161eaa audit scopes H08a6 to scoreboard/storage/stopwatches/border. Storage
and stopwatches clear dirty state before writing; default borders skip existing
files and can restore stale nondefault settings. Their bounded repair is in
H08_SCORE_METADATA_PACKET. The remaining helpers need distinct handling:

- Bossbars and scheduled events serialize borrowed state and suppress write
  failures; their existing empty-with-existing-file checks preserve resets.
- World clocks merge old NBT with current fields. Suppressed writes and tolerant
  merge reads are separate concerns: a failed old-file read can lose unknown fields.
- Statistics and advancements serialize borrowed JSON. Stats dirty/events track
  notifications rather than disk completion; Stats::write also serves LAN records.
- Fight, trader and portal arrivals already clear dirty flags only on success,
  but callers do not receive failure. Dragon fight also saves eagerly on a kill;
  that gameplay call requires explicit handling rather than incidental propagation.
- Owner/guest records and native entity retirement require retained failed
  snapshots; converting their calls to question marks alone cannot prove ownership.

This is a source audit, not completed failure coverage or checkpoint acceptance.
