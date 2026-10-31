# Dropped-item checkpoint seam audit

Read-only research at local engine173ad2a7b496b11b15e1492ad57c4ce4da9cf21d.
This is not source authority or an accepted representation/API change. The
persistent-retirement six-path component remains the next proposed packet, gated
on frozen-window FINAL acceptance. S1/G1 remain open.

The normal supported machine must preserve items across removal and completed
save/reopen. A delivered ItemEntity cannot be deleted to obtain a checkpoint.
The existing checkpoint envelope can cover native entity-region members, so a
bounded native-faithful item profile appears possible without a new journal or
substitute host drops file. It still requires reviewed validation and recovery.

## Existing paths and refusals

Paths below are under crates/horizon-client/src at the inspected revision.

| Seam | Current behavior |
| --- | --- |
| game/levels.rs:1195 | Session census refuses dropped items. |
| game/levels.rs:686 | Shared census refuses EntityMemory categories; ordinary item save/load creates UUID/raw entries. |
| game/levels/session_checkpoint.rs:1053 | Physical .mca validation permits only Overworld block-region files, not entity regions. |
| game/persist.rs:638 | entity_chunks(keys,false) projects actual items without removing them; uuid_for at599 assigns UUIDs. |
| game/persist.rs:988 | save_entities queues actual native entity projections. |
| save.rs:2498 | Native writer stores entities/*.mca; empty records delete previous contents. |
| save.rs:1805 | Entity records contain DataVersion, exact Position and Entities. |
| save.rs:4104, streaming.rs:1249 | Persistent loading stages native entity tags in World::loaded_entities. |
| game/persist.rs:147 | spawn_loaded_entities creates actual Game::dropped values. |
| game.rs:11490 | save_level queues both chunks and entities; the existing session fence can drain them. |

## Candidate boundary to review

Support stable, finite, resident native ItemEntity owners only. Retain other
entity-family, anonymous/orphan identity and unresolved loaded-queue refusals.
Prepare stable unique UUIDs explicitly before taking an immutable baseline:
session_snapshot(&self) cannot call today's mutating uuid_for. Failure after this
preparation must accurately disclose the identity mutation and retain all owners.

Account each UUID/raw memory entry only when it correlates exactly with an
admitted live item. Validate exact Overworld entities/r.x.z.mca members using raw
header/compression/size/coordinate checks before tolerant native constructors.
Inspect every saved record, including off-resident records. Reject duplicate
UUIDs, malformed owners or unsupported saved entities instead of omitting them.

Compare canonical UUID-keyed live projections before and after native save, and
against copied entity-region records. Keep exclusive Game access through writer
fences, projection comparison, physical copy, validation and publication. Member
hashes in the existing inventory cover native entity-region files.

Recover using a staged checked installer: validate/reserve IDs, stacks and maps
before publishing actual items exactly once. Do not use today's unchecked ID
increment or world-RNG draw in spawn_loaded_entities. Explicitly define native
persistent fields versus recreated transient presentation fields.

Likely paths are session_checkpoint.rs, a small child item helper and test child,
levels.rs census, persist.rs checked identity/projection/install services, and
session_checkpoint_tests.rs. This is a research map, not a six-path authorization.

## Representation limits

Native item encoding omits BundleSelected and decoding restores its default
(item_data.rs:873). The existing slot checkpoint validator already rejects
nondefault selections recursively (world/slot_restore.rs:503). Ordinary item
equality is not proof that such selections survived. A rich retirement case can
prove original payload ownership while still refusing checkpoint capture; normal
supported-machine save must succeed for its adopted representable item profile.
Supporting additional runtime-only selections needs an explicit bounded design.

Native ItemEntity loading recreates previous position, ticks and bob phase.
Exact transient phase preservation is not established by native persistence.
Do not promise it or consume world RNG silently. Neither limit authorizes a
schema change, a conservation waiver or a reduced G1 experiment.

## Required next evidence

Start with real checked retirement delivery; keep the delivered item owned.
Capture through the actual native writer/fence, restore and recapture. Compare
stable UUIDs, represented scalar fields, nested components, counts and total
participating items. Include malformed/coercive NBT, duplicate identity,
off-resident records, runtime-only metadata refusals, ID exhaustion, writer
failure, Copied/Promoted retention and explicit retry. Never clear entity lists
or persistence maps to obtain a passing result.

Research by bounded read-only agent s1cw_lan_author; coordinator recorded the map.
No source edits, execution or accepted API result follow from this document.
