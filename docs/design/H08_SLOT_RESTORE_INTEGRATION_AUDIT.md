# H08 item-bearing World slot restore integration audit

PROPOSED source analysis, not design or implementation acceptance. Prepared while
the counter-only cell is under qualification. Engine HEAD88bfcb05 on
codex/h08-machine-restore has the reviewed eight-path counter candidate; its
counter_restore.rs SHA256 is07fdb405a7041c109604caa518bf2133fe4249428681d8ee555bd06c2286977b.
No slot production code was changed or executed for this audit. Coordinator owns
any next packet, source freeze, independent Sol High reviews and all execution.

## Smallest useful next cell

First prove a library-only positive native slot codec and checked installation:
restore an actual seven-item payload with components/revision into its sole owner,
fresh SlotKey/projection authority and retained slot allocation high-water. Keep
E1/W1 and whole-epoch capture unchanged. This is a prerequisite, not a permanent
reduction of S1's item-bearing recovery requirement or a G1 decision.

world_slot_trial::Trial already holds exclusive active OwnedSlot or pending
PendingRetirement. Its admit() creates an empty projection; enabling then filling
that empty slot is not direct restoration. A checked defining-module factory must
consume decoded owned input only after all binding/world/placement/native storage
and scalar checks. slot_transfer::SlotProjection needs a defining-module consuming
constructor preserving exact revision and fresh authority, plus borrowed export
access; its diagnostic snapshot() clones ItemStack and must not become restore input.

take_world_slot_retirement() removes the owner from World tracking, while
take_world_slot_trial() also removes last_incarnation. Persistent mode therefore
needs retained metadata/head and a conservative detached latch. Returning an owned
payload to a caller must never make an apparently empty export/admission safe.
Payloads remain solely in OwnedSlot/PendingRetirement; metadata must not cache a
second live item owner. Counter placement generation and SlotKey incarnation are
different: repeated replacement while retirement is pending advances the former
while slot admission remains inhibited. Correlate them explicitly, never equate.

The likely bounded implementation seams are world_slot_trial.rs, slot_transfer.rs,
World lifecycle hooks, retained counter-domain metadata and a defining World child.
Choose exact interfaces/file scope only in a separately reviewed packet. Existing
native removal/reinsert hooks must drive lifecycle proofs; direct map editing is
not an unload test. No new Game slot enabling caller is implied.

## Codec constraints established by current source

Native item_nbt decoding is a tolerant gameplay persistence path: count casts
u32 to signed NBT and decoding clamps/defaults. Component persistence similarly
casts damage/repair scalars, filters malformed nested entries and normalizes some
representations. StackData.other retains unknown native Tags, so nested byte,
integer, floating, string/list/compound/array support cannot be inferred from
positive-count validation alone. slot_retirement::check_counts is not a complete
component codec validator.

BundleSelected deliberately has equality that ignores its value and is documented
as runtime-only, never saved/sent. ItemStack equality alone therefore cannot prove
exact native state preservation. The packet must explicitly handle or refuse
unsupported selection/nullable representations, distinguish durable saved semantics
from live runtime state, and keep broader S1 obligations visible. Do not silently
invent a generic item format or normalize unsupported values as a shortcut.

Consider existing bounded NBT primitives with strict canonical input re-encoding
and native scalar/structural checks. The decoder accepts duplicate compound keys
with last-wins behavior, so typed decode success is insufficient. Bounds must be
enforced before retaining decoded collections. Candidate limits (not accepted):
1MiB payload,16384nodes,depth32,4096string/key units,16384array elements. The eventual
packet needs concrete fixtures proving supported component/scalar/byte forms and
refusal before an existing owner is consumed. Live moves preserve allocations;
durable decode establishes values/bytes, not the original Arc addresses.

## Required proof direction and remaining admission boundary

Test direct positive payload/revision restoration with components; new key and
projection identity; source keys/prepared plans cannot commit against the restored
owner; occupied/wrong-target/clock/binding refusals preserve current allocations,
RNG, chunks and clocks. Native removal must yield exactly one pending owner that
repeated replacements cannot overwrite. Pending take/teardown must return exact
move-owned payloads while persistent export/admission refuses. Test actual unload/
reload and allocation/revision maximums without reset or zero reconstruction.

Current game/levels.rs::inspect_entity_level rejects ANY has_world_slot_trial(),
and capture_profile_checks invokes it before save. Temporarily taking/hiding a slot
to pass E1 would lose ownership accounting. A later positive-slot epoch needs a
separately reviewed accounted-slot extension retaining all unrelated E1/W1 refusals.
Native reloaded opaque ChunkExtras still prevent recapture; do not clear them.
Existing slot_player::CellLend moves/restores the actual inventory cell via Drop;
slot_retirement delivery moves pending ownership into an ItemEntity after fallible
checks. These useful Game seams remain dormant outside tests.

No additional product choice blocks this bounded library direction. Exact design,
codec support and full epoch owner coverage still need their packet/review gates.

## Audit anchors

SHA256 at audit (unchanged source unless noted counter candidate above):

| Source under crates | SHA256 |
|---|---|
| horizon-client/src/world_slot_trial.rs | c03bb3fbf3d3a207555d70a72d460d7ea5ad983359a842c32ffc9c15e30e881b |
| horizon-client/src/slot_transfer.rs | 9ae98c925cd74d28079ffcd91b25fe409c70482f49e1a4eb7ddccd826b577bc8 |
| horizon-client/src/game/slot_player.rs | 07968eb16ea98830b1df8fb1ce3291719fcffdcf757306fbdcaebb4209845b6d |
| horizon-client/src/game/slot_retirement.rs | 8728e89c014cbb545241438e9fd543f22e47a9d60a6e20b7aa4eb19dca55feb2 |
| horizon-client/src/game/levels.rs | 0ba5a20e922b765ea3618ba3818db30c56083486a664e1a22cf8778632685cce |
| horizon-items/src/item_nbt.rs | c6df4eb8a41c4122cde5be86115b0d41974a07701938b799409c3a5903599c02 |
| horizon-items/src/item_data.rs | ea1e77504937a768de3b91e2443229e45b951f32da47105e819eb4573c4bfd21 |
| horizon-world/src/nbt.rs | a32fd0ac8fe94ab1b23c000b51c3a81b28d40c518c5befb5272818792d1cf1ac |
