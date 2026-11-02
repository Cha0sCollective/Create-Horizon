# Coherent close: native source profile and completion custody

Status: PROPOSED. Extends only the accepted C0d fourteen-path packet at485d4b1.
Fresh independent design acceptance and private checkpoint precede source release.

## NativeWorld read boundary

The complete CoherentSession old-source reader requires a counter, slot, real
participants and initial Overworld. NativeWorld must not fake these or inherit
the closed D entity/item restrictions. Existing native readers tolerate malformed
input and cannot prove complete original-source validation. Add a separate bounded
native structural/readability profile inside the existing image/items/test paths.
This establishes readability, routing and preservation of opaque native payloads,
not arbitrary entity semantic conformance or a new world/import compatibility claim.
The existing CoherentSession and D validators retain their full restrictions.

Game prepares an opaque Arc<NativeWorldReadContext> from the actual fixed
RetainedNativeLevels, original SaveDir allocations/shapes, native dimension roots,
world root, resources/packages and expected retained identity when present.
Context creation/matching/preflight are CPU-only; actual original lock custody
remains with Game. Context creates owned NativeWorldReadInput(deadline); worker
run returns a private non-Clone NativeWorldReadPost bound to that exact context.
Root's fixed read job runs only after producer sealing/historical durability and
before final identities or writes. No Game/UI pointer or arbitrary callback.

Worker inventories bounded real files/directories without links/traversal, retains
hashes and requires an unchanged inventory after scanning. Reuse existing bounds
explicitly; oversized input is a retained refusal, not proof of normal full-world
scalability. Verify actual identity and package records where applicable. Parse
known current and _old native Level/player/shared/per-dimension metadata files
with bounded compression/NBT, exact consumption, duplicate-key/depth/node guards
and appropriate native envelopes. Unknown regular files are inventoried/hashed,
not arbitrarily interpreted as NBT; known authoritative paths cannot fall through
as unknown files. Unsupported external records/backup naming refuse explicitly.

For every actual retained dimension, scan all region/entities/poi records.
Validate raw location tables BEFORE tolerant native RegionFile opening, correlate
enumeration, bound/decompress every present record with compressed-suffix checks,
and check complete native NBT/routing/envelopes against actual coordinates/shape.
Retain arbitrary entity kinds/passengers/raw fields, float bits and Java strings
as opaque native data; no D-only field lists, entity spawning, palette coercion,
loot/RNG draw or invented semantic roundtrip equality. Factor native structural
and compressed-consumption helpers while keeping existing D wrappers unchanged.

Post retains current entity UUID locations and player-reserved identities, distinct
from historical backup versions. Reject duplicate current owners/current entity-
player collisions; repeated UUIDs across current and backup versions are normal.
The existing levels/persist author may feed the source UUID reservoir to first
identity preparation so new identities cannot collide with untouched old owners.
Existing live UUIDs remain unchanged. Before final submissions, reject a final UUID
also present in an old entity row outside the exact final overwritten row set;
an existing owner's own covered old row is allowed. Do not silently delete or
rewrite off-resident records to manufacture success. Retain all original custody
on refusal. No new entropy during retry or preflight.

## Native completion and truthful disposal

CloseOwner::try_native_ready consumes a ready owner into an opaque NativeReady
guard retaining that SAME original armed owner/Game/locks. Require NativeReady
phase, no block/actual IO/helper, all level acknowledgments and consumed writer
controllers. No Game/Lan/Shared reference, mutable accessor or into_game escape.
Future C0e can consume the private guard; its unresolved Drop still aborts.

NativeReady::try_dispose_dormant may disarm/drop only after actual absence of
LAN, remote, integrated, serving, friends, control, world/resource loaders,
preparation/entry owners, pending save and LOD worker owners, plus completed native
proofs. Return NativeDisposed, not global CloseCommit. Failure returns the same
guard. Never clear/disconnect a populated owner to satisfy absence.

For genuine populated component tests only, change the existing coherent fixture
helper to construct fixed setup and enter CloseOwner before returning an opaque
test custody object. It retains original receivers and a private constructor token
bound to original Game custody and actual LAN Shared allocation. The uninterrupted
constructor uses its real no-listener/no-announcement policy and channel-only
participants; no connector/network methods are exposed or invoked afterward.
Expose fixed observation DTOs and fixed test controls/setup variants, not Game,
LAN, Shared or an arbitrary fixture callback. Consuming the complete test custody
after NativeReady may dispose its constructor-proven dormant LAN together with
its receivers, subject to every other actual async-owner absence check. This is
fixture provenance only, never generic production transport proof or C0e activation.

## Retry and final acknowledgment corrections

Retain structured failed native attempts, including actual metadata/maps evidence,
submission receipts, Level effects and errors; panic uncertainty is recorded outside
disposable worker returns before dispatch. Successful retry never erases history.
All-source/Level/metadata/image preflights precede any marker acknowledgment; check
the original absolute owner deadline again immediately before the first commit.

An explicit Root retry after its actual IO handle is reaped may renew staged
Submitting writer deadline even before old expiry. The controller requires future
deadline, idle nonwaiting gate, matching original Sealed/Submitting admission and
intact correlated accounting/no owner fault. It changes only deadline, retaining
capability/input/receipts/ledger/history/native writer handles. No poll or phase
transition renewal, error clearing or guessed repair. Existing terminal writer
retry semantics remain unchanged. Busy returns the same original custody.

CPU preparation/revalidation still uses real native serializers. Split fixed
per-level work across owner turns where practical, with explicit bounded-profile
limits and rechecked deadline. Full pure aggregate equality/packing passes are
indivisible CPU operations; this packet claims no hard per-poll latency. This is
a qualification for NEW CPU work, distinct from the accepted C0a mutex caveat.
All new filesystem/durability work remains worker-only. No general scheduler.

## Required focused evidence

Counterless active+parked rich native positive, malformed off-resident region/
entity/POI and current/backup metadata refusal before identities/writes, compressed/
NBT suffixes/duplicate keys, unknown-file inventory and UUID current/backup rules,
actual held-reader/deadline/panic, foreign context/allocation, same fixed retry.
Saved falling active+parked records retain exact raw UUID/payload. Populated
CoherentSession reaches native-ready with original participants/items/locks intact,
verified image and existing recovery; opaque fixture cleanup proves actual custody.
Production populated/async-owner disposal refuses. Check deadline before ack and
structured failed-attempt retention. Reuse unchanged predecessor evidence; no
automatic full battery. Whole SOURCE/tool before compiler/runtime and DIFFERENT
FINAL remain required. No fifteenth path, ordinary activation, G1/live S2 or release.
