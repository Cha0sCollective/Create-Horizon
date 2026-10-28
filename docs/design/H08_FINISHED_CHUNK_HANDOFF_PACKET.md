# H08a3 checked finished-chunk storage handoff

Follow H08a2 with the next bounded generator ownership gap. An out-of-range fresh
finished Chunk is currently consumed by a void callback; Persistent removes its
entity tags and can log an error after only part of the region/POI/entity handoff.
Changing the return type alone would not preserve an unqueued entity snapshot.

Use an immutable borrowed `Arc<Chunk>` for the native finished callback, returning
a diagnostic result. Streaming wraps the completed out-of-range fresh Chunk once,
retains that exact Arc if the callback fails, and latches the shared first save
fault/producer halt. If already faulted, retain further fresh finished snapshots
from existing jobs without retrying callbacks. Keep them outside World: no fake
residency, block-entity activation or normal insertion. There is no reset/retry,
cancellation, disk quarantine or absolute new retention quota.

Persistent keeps the original Chunk, entities, block entities and extras immutable
through handoff. Reuse region/POI save_later with the same Arc, then add a narrow
entity enqueue accepting that Arc. A private Record variant can hold it and make
the existing entity-file NBT only on encode/pending-read paths. Avoid cloning every
Chunk or moving away the only entity vector. Preserve read-your-pending-save and
empty-entity deletion semantics and existing on-disk formats. Region encoding
already regenerates/removes the legacy entities key; it must not duplicate the
separate entity-file payload.

Success means accepted by every required enqueue, not cross-folder atomicity or
durable disk completion. Before pending insertion, SaveDir may own nothing; after
region/POI acceptance, entity enqueue may still fail. Streamer's retained complete
Arc covers either failure. Existing writer error retention and flush/completion
barriers remain required after successful enqueue.

Keep the implementation to streaming.rs, save.rs and affected existing test
generators. Tests inject callback failure before handoff and after partial enqueue;
assert exact immutable block/native block-entity/entity-tag/extras ownership;
prove no World insertion/residency event; verify sticky refusal/no implicit retry,
finite old-work reconciliation and no new producers; test successful pending
entity reads, empty deletion, writer failure retention and normal finished saves.
Prepared-data tests must assert availability and run explicitly. Existing proto,
generation, disk-save and both actual game-mode regressions remain applicable.

Stop/rescope before native entity-unload or disconnected-player ownership changes,
general retry/replay, checkpoint publication or cross-owner transactions. Require
a fresh independent read-only Sol High review of the exact candidate. Coordinator
owns all Cargo, integration and commits. No public engine push or G1 promotion.
