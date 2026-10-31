# Native dropped-item checkpoint proposal

Status: PROPOSED component design, not source authority or an S1/G1 decision.

## Prerequisites and outcome

Research foundation is frozen-window FINAL engine
173ad2a7b496b11b15e1492ad57c4ce4da9cf21d. The active persistent-retirement
successor has six SOURCE-CLEAR paths (manifest 810bb70691f4f50d415fdac8c561195d8a257a17aeeaca187e6b63389c4dc53b),
but its actual Game qualification and FINAL are still pending at authoring.
Implementation requires that packet's accepted clean exact FINAL successor, its
private durable checkpoint, a fresh verified clean source pin, and independent
Sol High acceptance of this design. Recheck every cited seam at that pin. Do not
implement against or merge the active dirty retirement candidate implicitly.
A different independent FINAL review follows source review and qualification.

The outcome is real checked persistent retirement delivery followed by coherent
native session capture, restore and recapture while the delivered ItemEntity
remains owned. Preserve its UUID, counts, represented nested components and
native persistent entity fields. Use the existing native entities/*.mca writer,
write fence, complete inventory hashes and session envelope. There is no
host/drops file, journal, new machine codec or portable save format. Preserve
package/world/authority pins, newest-epoch selection and publication semantics.
No ordinary close/Running, player authorization, registered binding, external
admission, animation-phase or full G1 claim follows from this component.

## Actual source seams

Paths below are under crates/horizon-client/src at the inspected foundation;
retirement adds lines to session_checkpoint.rs but does not alter these services.

| Path and method | Relevant ownership behavior |
| --- | --- |
| game/levels.rs:674 inspect_entity_level_owners | Refuses loaded entity queues and every EntityMemory category before observing resident drops. |
| game/levels.rs:1187 inspect_session_checkpoint_owners | Reuses that census, then separately refuses any dropped item. |
| game/persist.rs:599 uuid_for; :638 entity_chunks | Assigns a UUID on first projection; remove=false keeps the actual live item. |
| game/persist.rs:988 save_entities | Spawns queued entities, projects resident native items, prunes memory, then queues the native writer. This normalization must never hide an unsupported owner. |
| save.rs:1805 entity_record; :2498 save_entities | Native record is DataVersion, Position and Entities; an empty entity list deletes the previous record. Enqueue success is not disk completion. |
| save.rs:4104 and streaming.rs:1249 | Native arrivals place tags into World::loaded_entities. |
| game/persist.rs:147 spawn_loaded_entities | Current item path increments next_entity unchecked and draws world RNG for bob phase. Do not use it for checked recovery. |
| entity_nbt.rs:1327/:1362 | Native item encoder/decoder, including scalar coercions and recreated transient fields. |
| game/levels/session_checkpoint.rs:1052 columns | Rejects entity-region members; already supplies strict raw region/NBT scans and complete tree bounds. |
| game/levels/session_checkpoint.rs:1696/:1805 | Immutable baseline and continuous exclusive Game capture through native save, fences, copy and publication. |
| game.rs:11489 save_level | Queues actual entities alongside chunk writes through save_game_with(false). |
| game.rs:8959 finish_open_world | Publishes prepared world then clears old runtime entity collections; does not spawn loaded entities. Recovery can install checked items afterward without ordinary ticks. |
| game/lan.rs:929 project_session_participant_rows | Bounds live player item graphs, but equality ignores runtime BundleSelected. Add the explicit selection check here. |
| game/nether_portal.rs:90 Portals::entities; :666/:681 cooldown setters | Actual private per-entity PortalState owns cooldown/process independently of encoded item NBT. |
| game/nether_portal.rs:1377 checkpoint_entity_owner | Observes travel/arrivals only; existing general census intentionally permits the other portal maps. |

## Adopted native-faithful profile

This proposal chooses a bounded resident Overworld minecraft:item profile. Every
live dropped item is finite, in a resident chunk and uniquely identified. All
existing unrelated entity-family, deferred-producer, dimension, raid, player,
slot-health and authority checks remain. Nonempty off-resident saved entity
records refuse this component; every such record is inspected, never skipped.
Supporting dormant off-resident item custody is a later reachable extension, not
an excuse to delete records or silently omit their items here.

Persistent represented values are stable native UUID, exact stack item/count/
damage and adopted nested component graph, position, motion, OnGround, health,
fire ticks, age, pickup delay, Thrower and Owner. Health/fire must fit i16 exactly;
age and pickup delay must be 0..=32767. Reject live values which native writing
would truncate, clamp or default. Require finite positions and motion and checked
floor-to-column conversion. Age -32768 special semantics are outside this first
profile. Runtime entity IDs are freshly allocated on restore; UUID is the durable
identity. Restore previous=position, ticks=0 and bob_offset=0.0. These recreate
presentation/merge timing according to an explicit native-load profile, without
claiming exact previous/bob/tick phase. Do not draw world RNG or alter saved clocks.

Raw native fields have a closed admitted top-level set: id, UUID, Pos, Motion,
Rotation, OnGround, fall_distance, Fire, Air, Invulnerable, PortalCooldown, Item,
Health, Age, PickupDelay, Thrower and Owner. Match native encoder types exactly;
Rotation must be two finite float zeros and fall_distance a finite double zero.
Air/Invulnerable/PortalCooldown must be native default values (300/0/0) in their
native types. Unknown top-level entity behavior, passengers, links and alternate
entity kinds refuse. The native encoder's required fields must be present; only
Thrower/Owner are optional, each exactly four ints. UUID is exactly four ints.
The full checked tag must roundtrip through the actual native decoder/encoder
without semantic change, including signed floating zero; no tolerant coercion
is accepted. A checked encoded PortalCooldown=0 alone is insufficient: live
Portals::entities may retain nondefault state even when Base::tag supplies a
default zero. The live portal service below must also pass. Future raw-field
support needs a bounded review, not a blanket raw
NBT passthrough. This profile covers ordinary newly delivered items and its own
restored items without changing entity_nbt.rs or save.rs.

Nested item data uses existing native item encoding after a bounded live graph
walk and strict encoded payload checks. Enforce registered non-air item IDs,
positive count and representable damage, unique container cells and component
keys, finite numeric values, valid bounded strings, nested container/charged/
bundle/pot traversal, and exact native re-encoding. Keep the current unsupported
attribute-modifier component refusal. Bound graph depth 32, total nodes 16,384,
string units 4,096 and structured serialized record 1 MiB; enforce aggregate
item/entity bounds within the existing 64 MiB image and 256-member envelope.
Adopt at most 1,024 dropped items per image; reject overflow before reservations
or codec calls. No budgets are silently enlarged.

BundleSelected is intentionally runtime-only and omitted by native persistence;
its PartialEq always returns true (horizon-items/src/item_data.rs:56-77).
Slot capture already recursively refuses values other than -1
(world/slot_restore.rs:534). Require the same explicit recursive -1 check for
all dropped stacks AND every participant inventory/armor/offhand/ender graph in
lan.rs before encoding. Do not clear the selection or infer preservation from
ItemStack equality. Ordinary plain stone, default selections, and durable rich
nested metadata must successfully roundtrip. No native codec fix is proposed.
Supporting nondefault selections later would require an explicitly reviewed
bounded host representation and its version semantics; no existing slot/player
selection sidecar can be reused. That extension is not an owner blocker for
this routine native-faithful profile, nor an accepted general save promise.

## Narrow services and memory accounting

Add a private child module of levels::session_checkpoint at
levels/session_checkpoint_items.rs. It contains owned, bounded UUID-keyed
projection/validation values, not a generic entity framework. Proposed names:

- ItemProjection: canonical BTreeMap<[i32; 4], ItemRow>; each row has its checked
  column and full canonical native item tag. ItemRow also retains checked stack
  information needed for strict field comparison; list iteration order is not
  identity. Explicitly compare represented fields and component tags, not only
  ItemStack equality.
- CheckedSessionItems: non-Clone, non-deserializable private result of complete
  image validation. Carries the UUID-keyed rows and expected resident set; it is
  data validation, not a player/world authority token.
- PreparedSessionItems: private non-Clone staged concrete installer, with owned
  item vector/maps and checked ID head. It cannot be constructed from a UUID or
  caller-supplied count alone and cannot install into an arbitrary existing Game.
- Game::prepare_session_checkpoint_item_identities(&mut self) -> SResult<usize>:
  explicit identity preparation; returns newly assigned count.
- Game::project_session_checkpoint_items(&self) -> SResult<ItemProjection>:
  immutable, performs no UUID allocation, pruning, queue spawning or RNG draws.
- read_session_checkpoint_items(game, root, residents) -> SResult<CheckedSessionItems>:
  strict all-record native reader.
- Game::prepare_session_checkpoint_item_install(&self, checked, queued)
  -> SResult<PreparedSessionItems>, followed by a private infallible
  commit_session_checkpoint_item_install(&mut self, prepared).

Keep the actual UUID/raw maps and unknown/anonymous checks in persist.rs through
narrow pub(in crate::game) services. Do not expose mutable EntityMemory maps,
Game internals or an arbitrary callback to external consumers. Borrow live
items/maps/resources for preflight, then return owned bounded projections or a
staged value; release those references before mutably borrowing Game for native
save or commit. No mutable alias or unsafe operation is needed.

Session-specific accounting permits uuids/raw entries only for exact live
admitted item IDs. Every live item must have exactly one UUID after preparation;
UUIDs must be unique across drops and must not collide with participating player
profile/record UUIDs. Duplicate live entity IDs refuse. Each raw entry must have
its matching UUID, type and validated raw item record. Loaded raw modelled fields
may be stale after legitimate live movement or stack changes; validate raw syntax
and identity, then project with the live fields using the actual native encoder.
Do not incorrectly require stale raw Item/Pos to equal current live values.
Unknown/anonymous categories and orphan UUID/raw entries still refuse; do not
call prune_entity_memory to conceal them. Newly delivered items may have no raw
entry. Recovery may install raw entries only for the verified item subset.

Keep observe_checkpoint_entity_owners and its general conservative census
unchanged. Refactor only the session-specific path in levels.rs to replace its
blanket dropped/UUID/raw refusal with the checked item accounting service while
retaining loaded-queue, other entity-family, fishing and portal checks. Never
waive all EntityMemory because dropped.len() happens to be nonzero.


### Live portal ownership check

Add ONLY a session-specific immutable service in game/nether_portal.rs:
Portals::project_session_item_portal_owners(&self, live_items: &[ItemEntity])
-> Result<SessionItemPortalProjection, SessionItemPortalError>. Keep the result
and error pub(in crate::game), with private fields. It returns owned bounded
observations, never a mutable map, normalization, encoder or authority grant.
Use the already bounded unique live item IDs; bound map traversal before copying.

Inspect the ACTUAL private entities map. Every entry must name exactly one live
admitted dropped item. Require cooldown==0 and process.is_none(); negative as
well as positive cooldown refuses, and ANY Process refuses even when time==0 or
inside==false. Orphan/unmatched entries refuse even if their state is default;
other entity kinds are outside this item profile and already separately refuse.
An absent entry is native default. A matched explicit PortalState default is also
allowed without removal: handle_entity_portals normally removes it on a later
tick, but this component must neither tick nor clean it up. Canonical persistent
meaning is zero/no process in both cases. Capture the exact set of explicit
entries as an immutable before/after-save observation so native saving cannot
silently discard a live map entry. Recovery may recreate the equivalent absent
default entry; exact map allocation and transient phase are not promised.

Also inspect the private before map, which holds pre-move positions used by
find_entity_portals. Each key must match a live admitted item and each Vec3 must
be finite; orphan/nonfinite entries refuse without removal. Matched finite
entries are acknowledged transient observations, not additional item owners.
Do not require them to equal current position: actual movement can legitimately
separate those values. Retain their exact key/value-bit projection across native
save; recovery recreates no previous-move samples. This explicitly accounts for
the private map without inventing persistent previous-phase semantics. Existing
travel/arrivals refusal remains, and no general observer or player portal policy
is weakened or reinterpreted. loaded/dirty/client_inside/section-cache fields are
not normalized by this read service.

Invoke the live service during session preflight BEFORE any UUID preparation;
store its projection in Snapshot and invoke it again AFTER native save/fence.
Require exact before/after observations, alongside item/memory projections. A
nondefault live cooldown/process must refuse even if the raw/encoded tag has
PortalCooldown=0 or lacks that field. Raw nonzero cooldown still independently
refuses. The fresh checked installer requires this service to report empty maps
before commit and creates no cooldown/process/before entry for default-profile
items. Final verification checks their admitted default meaning again. No call
to set_portal_cooldown, restore_portal_cooldown, handle_entity_portals or
find_entity_portals occurs in capture/recovery.

## Capture ordering and failure ownership

The continuous exclusive &mut Game borrow remains the save barrier. No world,
client or entity tick, pickup, merge or external producer runs within capture.
After context/path/epoch checks and the first native fence, strictly inspect all
old source block AND entity regions before submitting writes. Old resident
records can legitimately differ from the current live projection, but must be
well formed and supported; off-resident nonempty entity records refuse. Corrupt
old source records must not be repaired into acceptance by the native writer.

Validate the complete live/profile/player graph, memory ownership and actual
private item portal maps first, including the new immutable portal service.
Identity preparation stages missing UUID assignments using existing native
save::random_uuid entropy, checks uniqueness against all existing/reserved
identities, and reserves map capacity before committing any assignment. Use a
bounded collision retry count (32 per missing ID); failure leaves identities
unchanged. Commit all staged assignments together, retaining existing UUIDs.
This is an explicit in-memory mutation before the immutable baseline, never an
immutable snapshot side effect. It moves no item and draws no world RNG.

checkpoint_session_steps invokes that named stage before session_snapshot.
Record its assigned count in a new in-memory SessionCheckpointError field
item_identities_assigned (default zero), including later failures. The method's
documentation states that UUID preparation can persist after a failed capture;
publication status retains its current separate meaning. Retry reuses the same
UUIDs rather than regenerating them. This error addition changes no disk format.

Extend Snapshot with the immutable item projection. Baseline precedes real
save_game_with(false); its ordinary save_entities path must now have no loaded
queue to spawn, orphan to prune or UUID to allocate. After the second fence,
compare live before/after item projections AND exact UUID/raw ownership accounting,
the exact live portal map observation, players, slot/counter, clocks, world RNG
and residency. Scan post-save source
entity records strictly and require the UUID-keyed projection to equal live
baseline, including exact owner cardinality and columns. Then copy through the
existing complete native tree process. verify_image must scan copied entity
records and return CheckedSessionItems; compare it with baseline and source
projection before promotion. Source inventory hashes are rechecked as today.
Any original stale record must be overwritten/deleted by the actual native writer,
not selectively filtered from the copied inventory. No entity overlay is written.

Keep every original live item, stack and UUID map throughout writer failure,
Copied/Promoted fault and timeout. Never take or clear dropped to obtain capture.
Existing pending/publication-uncertain handling, monotonic epochs and prior-image
retention remain. A Promoted failure may have published a valid complete image;
report uncertainty and explicitly retry at a fresh epoch. No automatic rollback
or ownership release is inferred. Save may have changed source files before an
error; preserve evidence and describe that existing native-save behavior.

## Strict physical validation and recovery installation

Share existing tree/read bounds, raw_region, raw_nbt, compression and member-hash
helpers. Extend columns' region classification to permit only the exact
Overworld region/r.x.z.mca and entities/r.x.z.mca families. Dispatch entity
members to the item reader; do not let the block reader silently continue past
arbitrary .mca files. Keep MCC, external-record, links, backups and unsupported
dimension/family refusals. Check canonical region filename/coordinates and
checked region*32+local arithmetic before native constructors. Validate every
location/header/compression/length and every bounded raw NBT record, including
off-resident records, duplicate NBT keys, trailing data and malformed UTF.

An entity record has exactly DataVersion=current, Position=int[2] matching its
physical slot and Entities=list of supported item compounds. Scan every record,
not just manifest residents or target coverage. Validate item Pos belongs to the
record column, that the column has a validated native block record, and that it
is in the manifest resident set. Reject duplicate UUID anywhere in the image,
unsupported entities and malformed empty/deleted representations. No silent
unknown-entity preservation is introduced. Recheck member hashes after reads.
Existing valid header-only region files are allowed; the native writer normally
removes records whose entity list is empty. Empty lists, if present in a native
record, must still pass exact root/type/coordinate validation and introduce no
owner. Preserve the existing complete manifest format/version and native format
pins; added entity members are already representable hashed world members.

Recovery first verifies the complete selected image before any native opening,
then compares locked destination entity projection with the selected image.
Build a bounded sorted load-center union from the existing player/target centers
AND every verified item column. Keep the existing required player/target native
coverage, and separately require a validated native block record for each item
column; do not invent a 3x3 saved-neighborhood requirement for an item-only center.
Put every load center in the prepared streaming.extra zero-radius retention set,
visit each through the actual streaming.update path, and wait under the existing
total recovery deadline until its column arrives. Run the native fence and then
assert ALL verified item columns remain resident before staging or committing
items. No generation fallback, synthetic chunk insertion or skipping a distant
item is permitted. The union and pending work remain within existing bounds.

After streaming/fence, validate that prepared.world.loaded_entities is EXACTLY
the subset expected for arrived native columns, with no unknown, missing,
duplicate or extra queued owner. Additional arrived columns must also match the
verified image; do not let tolerant loading manufacture an item. Require every
image item column resident before installation. This is a frozen recovery
component, so image items cannot be left as unaccounted dormant queues.

Use the existing owned fresh Box<Game> route. After finish_open_world, require
fresh empty runtime item/maps/other entity state and correlate the retained
queue again. Stage all decoded items from checked tags; use checked_add for the
whole ID range from current next_entity, check collisions with runtime IDs, and
reserve/build every vector and UUID/raw map before mutation. Use bob_offset=0.0,
previous=position and ticks=0 with no world RNG call. Require the fresh portal
item maps empty through the immutable live service; this profile permits only
zero/no-process and publishes no portal map entries. Stage no implicit cooldown.
All fallible validation, allocation and ID exhaustion checks precede taking the
queue or publishing any item. The final commit consumes only the exactly checked
queue and swaps/appends fully staged concrete owners/maps and next_entity with
no IO, callbacks, allocation or fallible work afterward. A preflight refusal
leaves the fresh Game, queued owners and ID head unchanged. No use of ordinary
spawn_loaded_entities, unchecked increments or parser-default success paths.

Carry CheckedSessionItems through Verified and NativeRecoveryImage. Final
recovery verification and immediate recapture compare the installed UUID-keyed
native projection, represented fields, nested components and all existing
session keys/pins. Run no simulation tick to make equality pass. Recovery failure
returns no ready Game; the recovery owner remains responsible for closing its
fresh resources, while the original source Game and accepted images survive.
ID equality across processes is not promised; UUID-to-one-owner identity is.

## Eight-path proposed budget

All paths below are crates/horizon-client/src; this is a proposed maximum for
fresh design review, not author authorization.

1. game/levels.rs: session-specific owner census only.
2. game/levels/session_checkpoint.rs: item stage/projection, error mutation count,
   native member dispatch, fences/correlation and recovery wiring.
3. NEW game/levels/session_checkpoint_items.rs: bounded item/raw record validator
   and concrete checked values; no SDK or general entity API.
4. NEW game/levels/session_checkpoint_items_tests.rs: validator and staging controls,
   declared as that child module's test child.
5. game/persist.rs: checked identity, memory projection/accounting and concrete
   fresh installer services beside existing native helpers.
6. game/lan.rs: explicit recursive player BundleSelected refusal in existing
   borrowed live graph preflight; default networking behavior unchanged.
7. game/levels/session_checkpoint_tests.rs: actual Game retirement/save/recovery
   integration using genuine resources, native writer and accepted fixtures.
8. game/nether_portal.rs: session-specific immutable private item cooldown/process/
   before-map refusal and projection service; focused inline tests and actual-Game
   portal controls may live inline here, reusing the existing session fixture with
   minimum cfg(test) visibility in the already scoped checkpoint/test paths.

No changes to save.rs, entity_nbt.rs, horizon-items codecs, game.rs, world_slot_trial,
retirement World APIs, net.rs or ordinary login are planned. Before authoring,
enumerate SessionCheckpointError literals and test expectations, module visibility,
recovery queues, live private portal services and installer state; report an actual
extra-path dependency rather
than broadening silently. If reviewed native writer/fence behavior cannot retain
and correlate this profile within these paths, stop source authoring and return
the precise seam for design revision. There is no known representation blocker
for ordinary adopted items. Nondefault runtime selection or dormant nonresident
custody requires a separate representation/lifecycle decision, not data deletion.

## Qualification and stop condition

Root alone executes all isolated serial qualification after fresh independent
exact-source review. No compiler/process spawning is added inside Rust tests.
Required actual Game evidence:

- Start with the retirement successor's genuine active slot and two participants.
  Move a rich durable nested stack through real remove/repeated-write/checked
  native delivery, keep its original ItemEntity, explicitly reconcile the new
  empty slot, and capture through native entity writer/fences. Include normal
  plain-stone success, nontrivial counts, damage and nested container/charged/
  bundle/default-selection metadata within the adopted profile. Count all player,
  slot and dropped nested owners before capture, after save, restore and recapture.
- Restore into a distinct fresh native root and real fresh Game. Prove stable UUIDs,
  one owner per UUID, represented scalar/component equality, fresh checked IDs,
  unchanged clocks/RNG, empty consumed arrival queue and actual entities/*.mca
  inventory coverage. Recapture a second epoch and repeat restoration without
  item deletion, native writer substitution or forged owner tokens.
- Keep two independent real clients on genuine integrated-owner and dedicated
  participant fixture variants where existing session helpers support them.
  Existing participant/lease/source-image controls remain; this does not qualify
  ordinary dedicated live admission or playable Running.
- Refuse top-level/nested runtime selections in EACH player owner location and
  dropped graph without changing selections/items/maps; retain slot refusal.
  Exercise coercive counts/damage/scalars, duplicate cells/components, graph and
  entity bounds, nonfinite/signed-zero fields, invalid item/UUID/type/coordinates,
  duplicate live IDs/UUIDs, player UUID collision and entropy-collision exhaustion.
- Exercise all raw entity headers/compression/size/sector overlaps, duplicate/trailing
  NBT, wrong DataVersion/Position, unsupported entity/field, malformed raw memory,
  orphan UUID/raw/unknown/anonymous owners, and malformed or valid nonempty
  off-resident records. Test both capture-source and resealed complete-image
  controls; source corruption must refuse before writer normalization.
- Actual Game set_portal_cooldown on a genuine delivered item refuses before UUID
  preparation even when native projected PortalCooldown is zero. Exercise a live
  process with inside true/false and zero/nonzero time, negative cooldown, orphan
  default/nondefault entities entries, orphan/nonfinite before-map entries, and
  matched known-item default entries plus finite pre-move samples. Refusals must
  preserve original items, IDs, RNG, UUID/raw maps and all portal map/state bytes;
  zero UUID assignments and no writer submission. Matched defaults and ordinary
  delivered drops must capture/restore/recapture successfully without a cleanup
  tick. Verify exact portal observations before/after native save. Keep existing
  general census tests that intentionally preserve nonpayload portal maps passing.
- Put a supported genuine item in a verified resident column beyond every initial
  player/target load center. Actual recovery must visit that item column and
  retain it through the fence, consume its exact queued tag once, and recapture
  its UUID/payload. Missing/delayed item-column arrival must refuse on the same
  total deadline without partial item publication or owner abandonment.
- Loaded queues refuse ordinary capture. Recovery queue missing/extra/reordered/
  duplicate tags, ID exhaustion and allocation/preflight failure must publish no
  partial items and preserve queued ownership. Test repeated installer attempts
  and final correspondence. Ordering alone may vary; UUID-keyed content may not.
- Actual native writer error/block/deadline and Copied/Promoted controls retain
  original Game items, exact stable UUIDs, previous accepted epochs and locks.
  Resolve writer/helper ownership before assertions; explicitly retry at a new
  epoch and prove no duplicate item. Inspect both reported mutation count and
  publication status; identity preparation failure before commit changes nothing.
- Rerun accepted empty-entity session, retirement, slot/player selection, connected
  checkpoint, frozen recovery/managed close and default entity persistence checks,
  plus applicable feature/format/lint/broad gates. No optimized/hardware or broad
  exact-vanilla-entity claim follows merely from these component results.

Completion requires captured exact source/tool inputs, concrete successful and
refusal evidence, a different independent FINAL, a clean local engine pin and
private durable result record. Full S1 still includes ordinary close/departure
custody, placement/unload/reload/Running, player authorization, registered binding,
network admission/replication and committed animation qualification. This packet
preserves that trajectory without reducing G1 or promising general save compatibility.