# H08 native slot payload restore cell

DESIGN ACCEPTED FOR BOUNDED IMPLEMENTATION after fresh independent read-only
Sol High review of proposal SHA256
eca506a3d317e4a973b15bb28904e50761df23d9aba833461e4e80a106dc85b9.
No implementation/source/evidence acceptance, epoch recovery or S1/G1 acceptance
follows. The coordinator
owns the release, exact source freeze, subsequent reviews and all execution.

Drafted against clean local engine f90d369f43567042d7729d08026633bd67015356,
tree 59c0f71c4f89f5ea5028f51e2ab3a8cd00cf7d2e, branch
codex/h08-machine-restore. Counter native/Game/feature/fresh both-mode GUI/broad
qualification and different final independent review ACCEPT f90d369. Recheck
acceptance/revisions before starting. Private acceptance checkpoint:
82b2da2b4211402c986ec8d5a44a73f30485d965 (pushed to verified private repository).

Inputs: H08_SLOT_RESTORE_INTEGRATION_AUDIT.md SHA256
17963edcae55119fd2f468dacd03ce0e3dfeb71201ce41ffd058627c02e0d221;
H08_MACHINE_RESTORE_PACKET.md SHA256
a8b44451d18ec26fbeecbbb83a7a380de31492b86ef5a66eb0f5bbae88ddfa57.
The audit is source analysis; the counter packet does not authorize slot work.
This draft is the author's only write. No engine edits, imports, execution,
WSL, Cargo, tests, formatting, scripts, network or commits were performed.

## Objective, boundary and stop

Restore one actual native World slot containing seven items, nonzero slot
revision, modelled components and a bounded nested binary component into its
sole native OwnedSlot. Install the exact decoded value directly through a
checked defining-module factory, with a fresh SlotKey/projection identity,
retained slot allocation head and explicit machine-placement correlation.
Do not reconstruct seven by admitting an empty slot and transferring/refilling.

This is LIBRARY ONLY. It writes no world member or archive and has no Game,
Task, filesystem publication, ordinary loader, SDK, callback or network caller.
No Game file, E1/W1 observer, PLAYER codec, v1 archive or v2 counter frame schema
changes. Existing E1 must continue refusing ANY world_slot_trial. Never take or
hide a slot to pass capture. Reloaded opaque ChunkExtras remain refused by W1.
The source/recovery Worlds in tests are separate fixture authorities; durable
bytes and diagnostic snapshots are not additional live inventory owners.

Stop after exact payload install, native lifecycle/ownership and refusal proofs.
Whole-epoch item conservation, pending/detached epoch recovery, actual player
admission, failed shutdown retention, disconnect history and two-client recovery
remain later work. Active/dormant library support below is no tombstone epoch
claim. S1's full item scalar/component/byte obligation is unchanged; unsupported
initial representations must refuse explicitly, not become a reduced S1 scope.

## Exact source cap: six library paths

Relative to horizon/crates/horizon-client/src, paired-packages only:

1. slot_transfer.rs: immutable borrowed projection view and crate-private
   consuming restoration constructor preserving exact revision and fresh Arc
   identity; focused inline tests. Preserve existing transfer semantics.
2. world_slot_trial.rs: checked private Trial/OwnedSlot factory, borrowed export
   view, persistent-mode guards and metadata notifications for admission,
   retirement, take, reconcile and teardown; inline factory tests. Preserve
   public returned owned types and the existing tests child unchanged.
3. world.rs: new defining child declaration and minimal opt-in routing around
   existing set/insert/remove lifecycle hooks. No new World field. Do not alter
   unpaired or default transient behavior.
4. world/counter_restore.rs: one optional metadata-only slot substate in the
   existing retained Domain, initialized None, plus narrow pub(super) metadata
   access/configuration helpers required by the sibling child. No new public
   raw Domain constructor, registry service or payload cache. Existing counter
   bytes and healthy counter-only behavior remain compatible.
5. NEW world/slot_restore.rs: fixed slot frame codec, typed errors, native World
   initialization/export/install and metadata lifecycle glue. World is already
   public; no lib.rs edit.
6. NEW world/slot_restore_tests.rs: defining-child test module for the fixed
   codec, positive install and actual native lifecycle groups below. Declare it
   from slot_restore.rs with an explicit path; no binary test/API widening.

No dependency, horizon-items, horizon-world, Game or normal native item codec
edit is proposed. Report a concrete path/function blocker before increasing
this cap. No broad generic item serialization framework or mechanical split.

## Sole owners and retained metadata

Trial.active: Option<OwnedSlot> and Trial.pending: Option<PendingRetirement>
remain the only live World slot payload owners. They are mutually exclusive.
OwnedSlot owns exactly one move-only SlotProjection, which owns Option<ItemStack>.
No live item value, SlotProjection or OwnedSlot is cached in Domain metadata.

The existing counter Domain is the native durable world/machine anchor. Add
slot: Option<SlotDomain> with these bounded METADATA ONLY fields:

    SlotDomain { high_water:u64,
                 allocation: Dormant
                           | Active{slot_generation:u64,machine_generation:u64}
                           | Pending{slot_generation:u64,machine_generation:u64},
                 status: Healthy | Faulted | Detached }

No runtime Arc, ItemStack, retirement payload or item bytes belong here. The
native Trial retains exact last_incarnation and the actual allocation; export
checks it against SlotDomain.high_water. The retained Domain survives trial
removal, preventing ordinary enable from resetting the head or hiding a removed
owner. Domain.slot None leaves current transient slot behavior unchanged.

Slot allocation generation uses the existing incarnation scalar in persistent
mode; SlotKey additionally contains a fresh Trial identity Arc and transient
AuthorityScope. Restoring the scalar does NOT restore the old key. Never encode
or deserialize that Arc, scope world_instance or a prepared transfer identity.
Fresh SlotProjection identity invalidates old projection snapshots/plans even
when value and revision match. Slot revision is independent of both generations.

Machine placement generation and slot incarnation are NOT equal identities.
The allocation records which actual counter placement generation owned it.
Repeated remove/re-place while Pending can advance counter placement generations
while the old slot generation/payload stays Pending. Preserve its original
machine_generation; never stamp that pending payload as the latest placement.

Persistent initialization requires an existing healthy native counter Domain
and no slot metadata/trial. It validates actual NativeBinding, independent host
CounterTarget, resident state, scope/configuration and native clock. Use the
anchor's already fresh transient native scope for Configuration; construct fresh
Trial/projection Arcs. A resident matching active counter placement admits an
EMPTY slot at checked slot high_water+1. A healthy dormant counter placement
initializes Dormant with head0 and no slot allocation. This explicit setup is
separate from direct positive restoration.

## Proposed native interfaces and atomic install

Names are proposed, not stable APIs. NativeBinding retains its existing checked
PreparedBundle/type/default-properties/iron-template/static-registry validation.
Items is the actual caller-supplied native immutable item catalogue, never
constructed from frame content; pass it to codec validation rather than adding
another stored catalogue field. Native mini-catalogue fixtures use the existing
Items parser. Known item identity is checked recursively for every nested stack;
component payloads retain their existing native semantics.

    SlotProjection::restored(stack:Option<ItemStack>,revision:u64) -> Self
        // crate-private, consumes stack; new projection identity, no Clone
    SlotProjection::borrowed() -> (u64,Option<&ItemStack>)
        // crate-private immutable owner view
    decode_slot_frame(bytes:&[u8],items:&Items)
        -> Result<DecodedSlotFrame,SlotRestoreError>
    World::initialize_persistent_world_slot(binding:&NativeBinding,
                                            target:&CounterTarget)
        -> Result<Option<SlotKey>,SlotRestoreError>
    World::slot_checkpoint_bytes(items:&Items,saved_game_time:i64)
        -> Result<Vec<u8>,SlotRestoreError>
    World::restore_persistent_world_slot(binding:&NativeBinding,items:&Items,
          expected_world:WorldId,target:&CounterTarget,frame:DecodedSlotFrame)
        -> Result<Option<SlotKey>,RejectedSlotFrame>
    World::persistent_world_slot_metadata() -> Option<SlotMetadataSnapshot>
        // head/status/generation association only; no cloned ItemStack

DecodedSlotFrame has private construction, no Clone, and owns at most one decoded
ItemStack. RejectedSlotFrame returns BOTH finite error and the same move-owned
decoded frame. Validation failure must not discard a caller's owned input.
Existing owner-return APIs still return their original PendingRetirement/Teardown,
not a serialized replacement or cloned item. Snapshot types remain diagnostics.

Install requires existing healthy counter Domain but absent slot metadata AND
absent slot Trial; even a dormant/detached existing slot mode is Occupied.
Validate expected durable world, independent canonical dimension/position,
binding package/type/schema/template/properties, current counter placement/head,
resident canonical state and World.game_time==frame.saved_game_time BEFORE any
World mutation. Host target is never derived from frame coordinates, a matching
block scan or player identity. Compare to actual counter Domain, not just two
mutually consistent candidate records. The selected counter placement/context
is established independently before this slot-only call; no claim of atomic
counter+slot or whole-epoch installation is made.

Validate slot arithmetic/head/ownership, native item catalogue and complete
lossless-codec checks before moving input into a new local Trial. Construct all
metadata, result keys, scope and local owner before publication. The factory
consumes exact stack and revision, never SlotSnapshot::stack().clone() and never
Trial::new followed by refill. Publish Trial and metadata with no subsequent
fallible operation. All refusals preserve existing owner/stack box allocations,
counter/domain metadata, chunks, clocks, RNG and revision. No global allocator
reset or reservation is needed for slot incarnations; checked arithmetic remains.

Finite error family: InvalidFrame, UnsupportedStack, CodecLoss, Limit,
IdentityMismatch, BindingMismatch, ClockMismatch, Occupied, NotResident,
StateMismatch, PendingRetirement, Faulted, Detached, GenerationExhausted.
Narrow persistent fault/error additions must not reinterpret transient defaults.

## Fixed slot wire projection and exact native-codec limits

Use ONE strict canonical named native NBT root, empty root name, format1 and
kind native-slot-checkpoint. This is a slot-specific envelope around native
item NBT, separate from ordinary tolerant item_nbt persistence; do not modify
that gameplay codec or pretend it is generally lossless. No JSON/base64 wrapper,
generic extensible schema, alternate encodings or additional registry format.

The root has EXACTLY these required keys, with exact NBT types:

    format:Int(1), kind:String("native-slot-checkpoint"),
    world_uuid:String(canonical-v4), dimension:String(canonical-native-ID),
    position:IntArray(exactly3), package_identity:String(lowercase-SHA256),
    type_id:String, save_schema:Int(1), block_id:String,
    template:String("minecraft:iron_block"), properties:Compound(empty),
    observed_state:Compound{name:String,properties:Compound(empty)},
    saved_game_time:Long(nonnegative),
    machine_high_water:ByteArray(exactly8,unsigned-big-endian),
    slot_high_water:ByteArray(exactly8,unsigned-big-endian),
    ownership:Compound(tagged closed union below)

Unsigned generation/head/revision scalars use exact eight-byte big-endian arrays,
preserving 0..=u64::MAX without signed cast, floating or decimal coercion.

    ownership Active := {kind:String("active"),
        slot_generation:ByteArray8, machine_generation:ByteArray8,
        revision:ByteArray8,
        stack:Compound{kind:String("some"),value:Compound(native-item-NBT)}
           OR Compound{kind:String("none")}}
    ownership Dormant := {kind:String("dormant")}

All listed fields are required; none/nullability is represented only by the
explicit closed stack discriminator. Empty Compound, missing stack, missing
kind/value, unknown/duplicate keys and wrong scalar widths/types refuse. Native
item value requires id:String and count:Int; native components absence is the
canonical native no-components representation, not a default for a missing
required outer stack field. Preserve Option<Box<StackData>> representation by
value/roundtrip validation; Some(default) that normalizes to None REFUSES.

Active requires 0<slot_generation<=slot_high_water and positive
machine_generation equal to the independently established live counter active
placement generation, which is <=machine_high_water. The latter head must equal
the anchor's current head. Resident physical state must be the actual template.
Dormant has no item/revision/generation allocation fields, actual healthy dormant
counter placement, matching observed nonmachine state and retained slot head.
Pending is NOT a wire variant in this first cell: export/decode/install refuse
PendingRetirement. Healthy live pending ownership is still fully retained and
tested. No fake active record may stand in for pending or detached data.

Bounds before retained collections: total frame<=1MiB, native NBT nodes<=16384,
depth<=32; every UTF-16 string/key<=4096units, every array<=16384elements and every
retained list/compound/node total within the same node budget. Header canonical
IDs additionally <=256UTF-8bytes. Root/field counts are fixed. Use existing
nbt::read_named/write_named with explicit Limits; use a bounded no-allocation
wire preflight in this one child to check collection/string lengths and structural
budget before the native reader retains them. It checks the existing NBT grammar,
not a second retained Tag decoder. Reject duplicate compound keys via strict
canonical byte re-encoding; do not rely on the native reader's last-wins map.
Require used==input.len(), empty root name and reencoded complete frame==input.

Before export encodes any live stack, visit borrowed native stack/component data
with the same budget so a huge native structure cannot allocate an unbounded
temporary item Tag before failing. No source owner is taken during export.
Then validate BOTH directions:

- Borrowed live stack -> native item NBT -> bounded bytes -> native decoded stack
  must reproduce all native fields and recursively checked runtime selection.
- Candidate canonical item NBT -> native decoded stack -> native item NBT must
  reproduce every exact Tag/value/byte and the closed envelope. No unknown native
  item-root key can be silently discarded by item_from_nbt.

Do not rely on count-only check_counts, Debug strings, aggregate item totals or
ItemStack PartialEq alone. Compare complete modelled native value plus exact Tag
forms, Option representation, nested stacks and the selection rule below. Roundtrip
checks are read-only candidate/diagnostic validation; those decoded values never
become a second authoritative live owner alongside the original World slot.

Initial supported scalar domain is explicit: every present root/nested count
1..=i32::MAX, damage and repair cost0..=i32::MAX, and other modelled scalars only
when their exact native encoder/decoder roundtrip succeeds. Empty slot is None;
Some(count0), u32 counts/damage/repair above the signed codec range, narrowing,
clamping, filtered nested entries, malformed known components and allocation-
presence normalization REFUSE before consuming any live owner. No saturation.
Actual catalogue item must exist and not be minecraft:air at every stack node.

StackData.bundle_selected is documented runtime-only and deliberately excluded
from equality. This cell admits only its native default -1, checked recursively
in container, charged, bundle and pot-decoration stacks. Any nondefault REFUSES;
never silently normalize it. Do not invent persistence for this runtime cursor.
Unknown component Tags may contain exact bounded Byte/Short/Int/Long,
ByteArray/IntArray/LongArray, valid-UTF16 String, List and Compound. Finite floating
values with exact bit roundtrip may pass; negative zero and nonfinite floats/
doubles REFUSE. Unknown payload bytes are not interpreted as inventory owners.
Unsupported malformed UTF16, normalization or any unproven native form refuses.
No assertion of complete S1 scalar/full-byte support follows from this subset.

## Exact persistent native lifecycle

For Domain.slot None, run the original transient slot paths unchanged. Persistent
mode routes native slot lifecycle AFTER counter metadata reconciliation so a new
allocation is associated with the actual newly committed placement generation.
This ordering change is opt-in only. Do not infer a new placement from a same-state
write. Physical block/chunk commits are not rolled back on slot errors.

| Native event | Persistent slot result |
| --- | --- |
| Unwatched write/same-state no-op | No owner/key/revision/head change. |
| Watched removal | Move active OwnedSlot once into PendingRetirement; retain exact item/projection/revision and original generation association. |
| Repeated removal/re-place while Pending | Keep the ONE pending owner, original retirement reason/time and slot head. Counter placement may advance independently. No empty replacement slot. |
| Dormant -> actual new matching placement | Validate current counter placement; checked slot_head+1, fresh SlotKey and empty projection; publish head/key together. On error latch fault; no unstamped zero. |
| Actual unload | Retain active owner/key/head and mark nonresident; transfer/read/export unavailable. |
| Insert matching chunk, replaced=false | Same allocation/payload/revision/head becomes resident. A mismatch moves active owner to native PendingRetirement with ReloadMismatch. |
| Insert replaced=true | Move active owner to native PendingRetirement with ChunkReplaced; counter domain may also fault conservatively. Never destroy or re-admit a positive slot. |
| Successful take_world_slot_retirement | Return the ORIGINAL PendingRetirement by move; retain slot head/association, latch Detached for this World lifetime, even for empty payload. |
| take_world_slot_trial | Return original Teardown and all owners by move; retain slot head/association and latch Detached for lifetime. No default re-enable/reset. |
| Counter domain take/fault | Slot operations/export refuse; native slot owners remain retained and can still be explicitly returned by take/teardown. Never drop them merely because the counter stopped. |
| Generation/revision exhaustion | Checked refusal; retain current/pending values, head and fault. Native block write may already have committed. |

All persistent public read/prepare/commit/reconcile/export/restore paths consult
retained mode/status. Ordinary enable_world_slot_trial refuses whenever slot
metadata is Some, including after Trial take and Dormant/Detached modes. Pending
take with no pending value is a no-op, not a new detach. Once Detached, ordinary
reconcile, insertion and removal cannot clear it or admit another slot. There is
no acknowledgement/reset service in this packet. Caller owns any returned
payload and must retain it if its sink refuses it.

The existing counter-only Detached/fault rules are not weakened. Pending slot
export is refused even when counter metadata is otherwise healthy. This deliberately
stops before proving pending recovery; live owner retention is not a durable epoch.

## Required defining-module native test groups

Freeze concrete names at implementation source review; proposed groups:

- native_slot_restore_positive_seven_exact_revision_and_components: actual native
  transfer setup reaches7/revision>0; borrowed export leaves source allocations
  unchanged; decode/direct install restores exact7/revision into OwnedSlot with
  no empty-admit/refill. Fixture includes custom name, damage, nested known stack,
  and an opaque component containing Byte/Short/Int/Long, byte/int/long arrays,
  string/list/compound and finite positive/negative float bit patterns.
- native_slot_restore_codec_refuses_loss_and_malformed_frames: exact nullable
  wrappers/None versus Some(default), zero/unsigned overflow, malformed native
  nested entries/unknown root fields, required field absence, duplicate keys at
  root/nested component levels, trailing bytes, wrong type/width/length, every
  byte/node/depth/string/array boundary. Explicit nondefault selection at root
  and EACH nested owner family; source allocation/head/key/value unchanged.
- native_slot_restore_factory_moves_owned_input_and_failure_returns_it: private
  factory preserves ItemStack extra Box allocation when consuming decoded/live
  owned input; rejected install returns same owned frame/Box. Durable decode is
  tested by values/bytes, never by claiming old serialized heap addresses survive.
- native_slot_restore_fresh_keys_and_stale_plans: source/restored Worlds coexist;
  source SlotKey and prepared TransferPlan fail against restored World; same
  generation/revision/value do not recreate Arc identity. Supported transfer after
  install uses existing prepare/commit and preserves item totals/components.
- native_slot_restore_binding_target_clock_occupied_nonmutation: independently
  wrong world/dimension/position despite identical template blocks, package/type/
  schema/properties, counter generation/head, clock, state/residency and existing
  transient/persistent/dormant/detached targets. No RNG/clock/chunk/counter/source
  allocation changes. No candidate-derived target or constant scope.
- native_slot_restore_pending_repeated_placement_and_detached_returns: actual
  World::set removal/re-place drives Pending while counter generations advance;
  ONE exact pending projection/Box/reason/time persists. Export refuses; take
  returns original owner, latches Detached, old enable/reconcile/restore cannot
  admit, and retained caller value is still recoverable after repeated writes.
- native_slot_restore_real_unload_reinsert_and_teardown: defining-module calls
  remove_streamed_chunk/insert_streamed_chunk, not direct map edits, prove same
  owner across unload/matching reload and owned pending on mismatch/replacement.
  Teardown returns exact active/nonresident/pending owner and retains detach/head.
- native_slot_restore_dormant_heads_and_exhaustion: codec/install Dormant with
  positive prior slot head and actual dormant counter anchor; next real placement
  allocates above that head even when counter generation differs. HeadMAX refuses
  no zero; active revisionMAX remains exact and next transfer refuses atomically
  without changing either slot. No global allocator reset or fabricated callbacks.

All test fixtures are isolated native library Worlds with actual parsed native
metadata/catalogues. Existing default transient slot, transfer and counter tests
remain regressions. Invalid/unsupported cases prove refusal, not partial support.
No test says E1/W1 passes with a slot or claims a complete saved epoch.

## Qualification and remaining decisions

Coordinator serializes every Cargo operation and owns native tests, all required
feature checks/lints, formatting/public-tree/hooks, broader regressions and exact
source/evidence retention. World constructor/mutation routing changes require
fresh ordinary integrated AND in-frame Game validation unless the final exact
source paths demonstrably avoid such changes; no stale GUI evidence assumption.
Require fresh independent Sol High source review before execution and final
evidence review before acceptance. Compilation alone does not prove payload
installation, ownership retention or a future coherent epoch.

The six-path cap, optional metadata anchored to existing counter Domain,
active/dormant-only wire, conservative lifetime Detached policy and strict
roundtrip-supported native item subset are accepted bounded DESIGN CHOICES,
not permanent product-scope or S1 reductions. If
the bounded preflight cannot safely enforce early native NBT limits or a
supported item cannot roundtrip exactly through existing codecs, report the
specific representation/function; refuse it or return to design review before
proposing a separate item format or expanding dependencies/paths. Full native
component support and pending/detached whole-epoch ownership remain explicit
later gates. No public publication, promotion, S2 or S3 work is authorized here.
