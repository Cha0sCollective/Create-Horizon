# C0a original loaded-record custody amendment

Status: ACCEPTED by fresh s1_composition_api_review at proposed document SHA256
6bcc3ac8be31ace130baf3d4785c3ce9c540f78ff22aa54b861c451eb6d246a0.
Source outside the original six paths remains held until the private durable
assignment update; this status update changes no accepted scope.

Read-only mapping at composed predecessor1956eb49 found that Persistent::load
uses tolerant SaveDir reads and replaces entity data from a tolerant sidecar
reader. A loaded !fresh Chunk and factory origin cannot prove the original raw
record's validity. Re-encoding cannot recover discarded input. Permanent loaded
column refusal would also fail the accepted healthy saved/reloaded close case.

Expand the source budget to exactly EIGHT files:the accepted six, plus
crates/horizon-client/src/save.rs and
crates/horizon-client/src/game/levels/session_checkpoint_items.rs.
No other path or ordinary activation is granted.

## Separate retained read route

Add an opaque non-Clone retained-load carrier, constructed by actual Persistent/
SaveDir code, bound to the same SaveDir allocation, dimension and requested
coordinate. It owns original column and entity-sidecar evidence and an outcome:
Absent, Loaded(complete Chunk), or Refused(original error). Retain every artifact
already acquired on refusal, even if no Chunk can be decoded.

Evidence distinguishes original disk ChunkRecord plus correlated raw region
locations/header facts; original pending folder/key/sequence/Record with actual
Arc owners and encoding context; and proven missing file/slot or pending Delete.
Any structured pending projection supplements the original Record, never replaces
it or pretends to be disk bytes. Capture before decode, compaction, sidecar
replacement or other lossy transformations. Read limits apply before allocation.

Only genuine absence permits generation. Present proto, malformed full record,
decompression/decode failure, refusal or unavailable evidence cannot fall through
to ordinary Persistent::proto/SaveDir::load_proto or regeneration. The retained
trait entry fails closed for providers unable to supply evidence. Ordinary
load/proto/cache/writer behavior remains unchanged.

Read/capture is worker-side IO. Correlate original header/record facts with the
same allocation's cache, writer and pending state, before any constructor could
normalize corrupt locations. Negative cache entries or an independent path read
followed by an unrelated cached read do not supply that proof. This retains the
existing trusted-path boundary; no new hostile rename/inode defense is promised.

Original read provenance does not prove that the original is still current.
AlreadyStored also requires relevant latest accepted sequence/storage state and
final same-allocation writer completion. New read-only per-key receipt/stamp
accessors may inspect the existing retained writer ledger; preserve admission,
accounting, error, retry and write semantics. Poll-side transitions use try-lock
and retain the same carrier on contention. Newer conflicting writes require
explicit reconciliation; unrelated column writes do not fabricate staleness.

## Reuse existing strict validation

Move the existing generic raw region-header validator unchanged into save.rs;
the existing game/levels/session_checkpoint.rs raw_region delegates with only
error-type adaptation. Preserve validation profile and existing callers.
Within that already-scoped Game file, also expose existing raw_nbt within
crate::game, body unchanged. Keep the previously accepted validate_column
visibility change. Do not substitute the different item scanner for columns.

The eighth file changes ONLY read_item_record visibility within crate::game;
preserve its signature, body and item profile. The Game-side close adapter
validates original compressed/structured column and sidecar records through the
existing strict parsers and validators. Preserve root/version/position checks,
item UUIDs/coordinates/duplicates and complete original/projection conservation.
Missing and malformed sidecars remain distinct. Keep the full Chunk, original
evidence and original work lease together until actual checked disposition.
No new portable codec, record format, validator duplication or profile expansion.

## Ownership and qualification

Root owns save.rs additive implementation and focused inline controls. Streaming
author owns streaming.rs carrier transport/retained trait entry. Preparation
author owns its existing five paths plus eighth-file visibility only, including
strict Game adapter and sixth-file parser visibility/delegation. Authors do not
edit one another's files. Root preserves all eight sources/prefix before changes
to a frozen draft. Existing Root-only execution/review/FINAL rules remain.

Focused controls cover healthy saved full columns/items, absent sidecars,
pending Chunk/Tag/FinishedEntities/Delete, newer relevant sequences, cached
negative recovery, corrupt raw headers, malformed compression/NBT and present
proto refusal without generation. Reuse unchanged writer/component evidence at
its actual revision; inspect new read-lock ordering and relevant native writer
interactions. Exact all-eight-source/tool review precedes compilation/runtime.
Ordinary lifecycle and eventual full connected S1/G1 obligations remain open.
