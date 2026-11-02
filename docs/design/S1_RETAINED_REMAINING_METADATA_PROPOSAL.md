# Remaining native metadata: bounded C0c2 design

Status: PROPOSED only. No engine edits, compiler/runtime, source release,
publication or stage acceptance follows from this document alone.

## Exact accepted predecessor and budget

P=b7cdb4220029c7f5aff6afb18f3813fd13fdcc76,
tree64223bde93195136434dc197e30015126b67b8c3, sole33e947a9.
Different C0c1 FINAL876d8a80 accepted exact P and final input58ec85d2 (72 hashes),
44 normal/onefatal/sixchecks. All eleven core files are raw byte-exact commit
objects; public branch and private authored checkpointa014deb0301bafe83d05fef089739a2f81a41b30
are synced. No dirty overlay is needed. C0a/writer/acquisition/Guest evidence
remains attributed to accepted predecessors and is not relabeled or replayed.

This refines the deferred C0c2 budget in
S1_RETAINED_METADATA_FAMILIES_PROPOSAL.md exact1a1518f1b8014192cbb4a5ea288e1ef0f8f40a20087abaab402ce21f0fce2823.
Fresh actual API review at P precedes acceptance. Maximum seven existing paths,
all under crates/horizon-client/src/game/:

- paired_close.rs: extend the existing concrete job/guard, evidence and whole ack.
- maps.rs: native per-map/index projection, IO and checked map-marker ack.
- dragon.rs: native fight projection, IO and checked fight-marker ack.
- wandering_trader.rs: native spawner projection, IO and checked marker ack.
- nether_portal.rs: native arrivals projection, IO and checked marker ack.
- raids.rs: full native raids projection/IO plus prepared CPU omen acknowledgment.
- paired_close_metadata_tests.rs: new defining actual-owner/native-file cases.

No game.rs edit, constructor, Game field, eighth path, generic executor, portable
codec, levels traversal/swap, CoherentImage, Level::write or ordinary caller.
Keep existing ordinary API guards, errors, ordering and partial-marker behavior.
Strictly equivalent private pure encoders may be shared if source review verifies
that normal behavior is preserved. Never call an ordinary mutating saver to
construct retained input or acknowledge the stronger whole batch.

## Shared fixed batch

Extend existing Expected/Input/Written and MetadataFamily/effects with exactly
Maps/Fight/Trader/Raids/Arrivals. Keep one exclusive original Game borrow, private
batch/attempt identities, immutable projection outside the worker, registered
actual handle, finished-only harvest and explicit Retry. Append the five native
operations after the existing core in fixed order Maps, Fight, Trader, Raids, Arrivals; this permits an actual later arrivals failure after successful raid IO without applying the prepared omen. No second
executor or competing job. All fifteen family preflights precede ANY infallible
CPU commit; no family cleans an original marker on native IO error or mismatch.

Unchanged lifecycle/deadline/panic contracts from accepted C0c1 remain mandatory.
Pure capture performs no filesystem calls, lazy load, simulation, ID allocation,
RNG draw, menu closure, dimension swap or retry-time resampling. Context includes
actual routes/loaded/dirty/remote guards, original marker sets and native payloads.
Private per-projection association ties each expected/input/post; the lexical Game
guard ties them to the original owner. Equal bytes/UUID/address/hash cannot grant ack.

Return real partial native evidence. In particular Maps owns its explicit write
loop, so retain actual per-map and last_id outcomes through error, not just an
aggregate unknown. Use small concrete map-ID/index evidence (non-authoritative,
cloneable for failure history) and a private post token for successful ack.
No receipt/data constructor can mint acknowledgment authority. Worker panic makes
unreturned subfile effects unknown; retain the immutable original projection.
Artifacts and failures are monotonic; no rollback, atomic/fsync/powerloss promise.

## Family contracts refreshed at P

Maps::save_to currently iterates HashMap, clears each map after write and writes
last_id afterward. Retained capture fixes sorted dirty map IDs, exact full map-ID
membership, last_id and all original relevant map/holder/texture/slots/missing
state. Native MapData::to_nbt and native last_id wrapper/path stay unchanged.
Worker writes dirty map files in sorted order, then selected last_id even when
there are no dirty maps. No map lookup/load, ID allocation or texture upload.
Preflight the full expected set/route/state; clear only exactly captured original
dirty flags after all fifteen families succeed. Later map/index failure leaves
ALL original dirty flags untouched, including files already written. Empty/noindex
and remote outcomes are explicit skips; existing native replacement stays native.

Fight gate is loaded&&dirty, with no added remote guard. Preserve fixed End path,
needs_scan/killed/previously_killed/gateways, optional portal/respawn, native floor
conversion and optional fields. Capture actual scan/crystal/player/respawn/blast
state with float bits where exact matching matters; never tick/load/normalize it.
Commit only the matching dirty marker; keep live runtime objects unchanged.

Trader gate is loaded&&dirty, with no added remote guard. Native fields remain
spawn_delay and spawn_chance with existing wrapper/path. Keep live tick_delay,
loaded state and all clocks intact; never initialize or draw RNG to prepare.

Arrivals gate is dirty&&!remote, independent of loaded. Native path is
data/horizon_portal_arrivals.dat; root is arrivals list WITHOUT data/DataVersion
wrapper. Preserve original ordered arrivals and complete opaque tags/plans,
travel/player/entity/before observations and waiting ownership. Never remove,
normalize, schedule or load an entity merely to project it. PortalSections is
opaque and remains the SAME untouched owner under lexical Game guard; no new
independent-equality or cache-serialization claim. Commit only matching dirty.

Raids gate is loaded Some && raids.dirty && !remote; route uses retained loaded
Dim even when Game.dim differs. Use full actual Raids::to_nbt; empty-session
helpers do not qualify full families. Validate public ids/order membership against
map keys before calling native to_nbt, which indexes by its private native order.
Do not sort or reconstruct native ordering. Preserve counters, loaded dimension,
omen state, patrol/siege/bar/homes/attributes and original player_extra. Private
Raid RNG/heroes/hash-order objects stay in place untouched under lexical guard;
no independent equality/accessor claim. If implementation needs such accessors,
HOLD for a reviewed explicit path amendment instead of weakening ownership.

Ordinary save_raids applies save_raid_omen_position only AFTER passing those three
selection guards, even if its native write failed. Retained behavior intentionally
acknowledges no marker/omen CPU change after ANY batch failure. Prepare the exact
complete Option<Tag> replacement for current Level.player_extra before IO, even
when loaded raid Dim differs from current level. omen_loaded false is no-op;
omen_loaded true with None extra creates Some(empty Compound) even for no omen;
Compound insert/remove preserves opaque keys; non-Compound stays unchanged.
Preflight original extra and every relevant raid/omen/route state before ALL
commits, then move the prebuilt replacement and clear selected raid dirty with
no new fallible work. Do not call save_raid_omen_position or Level::write in worker
or on failure. No claim this CPU projection already reached disk or closed world.

## Bounded qualification and gates

Fresh independent exact design/API acceptance at P, private durable assignment
checkpoint, ALL7/prefix preserved baseline and explicit Root release are required
before source. Root owns coordinator/tests; disjoint bounded native-text family
authors can work in parallel. No standing team. Root alone formatting/compiler/
runtime/Git writes serial. Authors no scripts/imports/WSL/runtime/Git/network.
Reviewers procedural native READONLY, permissions verified as inherited rather
than assumed enforced. Fresh SolHigh SOURCE/tool before compiler/runtime, then
clean candidate and DIFFERENT FINAL/private checkpoint before consumption.

Use fixed feature/debug-gated native test actions, genuine original owners/native
files, fresh isolated roots and pinned actual resources. No fake completion or
nested compiler/Java/test subprocesses. Release controls and explicitly reap actual
handles before assertion/unwind. Existing native normal-save regressions remain
relevant, especially their intentionally different ordinary partial acknowledgments.

Defining new cases cover all five nondefault native outputs and exact guards;
map index-only/empty/reset, partial later-map/index failure and marker/texture/
holder preservation; fight optional/respawn/floor and runtime state; trader clocks;
dirty+unloaded and empty/opaque mixed arrivals; nonempty raids, loaded-vs-current
dimension route, order/map mismatch refusal, all omen None/insert/remove/non-Compound
cases and original state on any failure; held actual IO, native errors/repair,
whole15-family stale/foreign preflight before ANY marker, fixed-input retry.

Adopt VALIDATION_WORKFLOW8631c054. Select defining new family cases plus relevant
changed shared-job/core regressions; reuse unchanged C0c1 family/lifecycle and
C0a/writer/Guest evidence at real pins, not automatic full batteries. New actual
shared-batch interactions must be checked, not assumed. State exact selection and
why counts over30 add distinct risk before launching. No arbitrary test cap,
new general qualification framework or hard UI/OS latency promise. Preserve all
seven source paths and every packet-prefix artifact before EVERY correction.

Stop condition is accepted dormant remaining-family batch qualification only.
C0d coherent native save/parked levels, C0e ordinary lifecycle, admission,
authorization, replication, registered lifecycle, committed animation and full
DEFAULT MemoryPipe/dedicated TWO TCP/in-frame G1 remain later S1 work. No liveS2,
public unreviewed source, PR, merge, tag or release follows from this proposal.
