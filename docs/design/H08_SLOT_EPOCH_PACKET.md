# H08 active native slot epoch cell

PROPOSED DESIGN ONLY. No implementation release or new E1 exception is authorized
by this draft. Start only after slot-library b17c44d8841fa8659ec96a850c039734c56e4435
finishes fresh GUI/broad/final qualification AND this exact design receives a
fresh independent read-only Sol High acceptance and coordinator release.
Counter f90d369f43567042d7729d08026633bd67015356 is accepted. At drafting, clean
local engine codex/h08-native-slot-restore is b17c44d, tree
d0d9f858d73a6e450c74408a437e397c97d02340. Coordinator qualification and different
final independent overall review now ACCEPT this exact slot-library checkpoint.
Recheck actual revisions, dirty files and receipts before implementation.

Source inputs: H08_SLOT_EPOCH_INTEGRATION_AUDIT.md SHA256
9f3b93463067faa4ed0a0d400686ccca97fb3a95efa8253fe57504e20d392d46;
accepted H08_NATIVE_SLOT_RESTORE_PACKET.md SHA256
44139f7618a578a87188a7e3cc88252a5aa6254c8de2e5e1fbaf6b18e4f15bd6.
H08_NATIVE_SLOT_RESTORE_RESULTS.md records that acceptance. The author
wrote only this private proposal; no engine changes or execution were performed.

## Objective and explicit stop

One actual authoritative Game, two channel-backed players and one pristine lit
nine-column OW patch reach counter3/last_advancedT and native slot7/revision1.
The authored owner inventory baseline is plain stone10. Use actual as_guest and
transfer_world_slot_inventory_cell to move7, leaving PLAYER3. Establish the
capture baseline AFTER that real move and after leaving the guest turn. No
detached donor, slot refill or post-restore reconstruction is allowed.

One actual native owner Task captures counter, slot, canonical PLAYER and native
world at that same time. Publish one immutable epoch; recover that entire epoch
into a fresh PRIVATE Game with PLAYER3, slot7 and exact placement/slot head/revision.
Install the owned decoded slot directly after native counter installation. Source
and recovered keys differ; atT skips, T+1 advances counter to4. A real subsequent
slot->player transfer may prove conservation without altering the immutable epoch.

ACTIVE healthy slot only. Pending, Dormant, Faulted, Detached, absent/unproven
payloads refuse capture/recovery in this cell. Stop BEFORE recapture: native
reloaded ChunkExtras still fail W1; never clear them or hide/take an owner.
This is not ordinary Game admission, a network protocol, history completeness,
in-frame capture, full item-component support or S1/G1 acceptance. The richer
component/byte proofs belong to the retained library suite. No SDK, registry,
journal, generic experiment system, public publication or S2/S3 work follows.

## Hard cap: eight exact source paths

Paths under horizon/crates/horizon-client/src:

1. world/slot_restore.rs: small immutable borrowed DecodedSlotFrame inspection
   API; preserve its codec, owning constructor, failure return and lifecycle rules.
2. game/levels.rs: fixed Linux+test+paired accounted-slot E1 helper and small
   internal factor. Ordinary E1 conservative presence/refusal remains unchanged.
3. game/levels/capture_integration.rs: fixed slot capture profile, shared physical
   envelope verification/selection/copy factoring and bounded manifest publication
   factor below. Preserve exact v1 decoder and existing wrappers' legacy behavior.
4. game/levels/counter_restore_integration.rs: declare the new descendant adapter,
   factor native counter correlation over a borrowed envelope view as needed, and
   enforce counter-only recovery profile. Preserve successful counter-only cells.
5. NEW game/levels/slot_epoch_integration.rs: Linux+test+paired descendant of the
   counter adapter. It reuses ancestor private publisher/recovery primitives;
   owns strict slot manifest, context correlation, Game capture/recovery glue.
6. NEW game/levels/slot_epoch_integration_tests.rs: descendant actual Game tests,
   six primary groups and at most one explicitly controlled auxiliary child.
7. game/levels/counter_restore_integration_tests.rs: visibility ONLY for source,
   loaded and tickets helpers needed by new descendant tests. Bodies and all six
   existing primary tests remain unchanged. No full recovery-helper duplication.
8. game/integrated/capture_completion_tests.rs: fixed Slot dispatch kind and
   ordinary/observer-timeout wrappers, same Task/run_tasks/owned completion pattern.

No other library/Game/item codec/dependency/source edits. No new Game/World fields.
No normal runtime enabling caller. Report an exact function/path blocker BEFORE
widening scope. No generic callback, Integrated::call change or publisher rewrite.

## Borrowed native inspection and independent context

Proposed paired public native API, no unchecked constructor or serialized key:

    DecodedSlotFrame::view(&self) -> SlotFrameView<'_>
    SlotFrameView { world:WorldId,dimension:&str,position:[i32;3],
        package_identity:&str,type_id:&str,block_id:&str,save_schema:u32,
        observed_state:&str,saved_game_time:i64,machine_high_water:u64,
        slot_high_water:u64,allocation:SlotFrameAllocation<'_> }
    SlotFrameAllocation = Dormant | Active{slot_generation:u64,
        machine_generation:u64,revision:u64,stack:Option<&ItemStack>}

All references borrow the sole decoded candidate; no cloned ItemStack, Arc or
mutable payload is exposed. Fixed template/empty-property guarantees remain
enforced by the existing strict decoder. Game must not implement another NBT
decoder to inspect private slot fields. Restore still consumes DecodedSlotFrame
and retains RejectedSlotFrame on failure.

NativeBinding comes from the actual PreparedBundle and Game's immutable native
registry. Items comes from actual game.res.items(), never candidate bytes. Host
target is the fixed declared position[2,70,2] plus actual Game.dim; expected WorldId
is separately retained host context established before image corruption/recovery.
No candidate-derived target, matching-block scan or player UUID as world identity.

Correlate slot view with counterV2 active generation/head/world/dimension/position,
actual binding/package/manifest identity, v1 archival scalar/type/state diagnostics,
and ACTUAL selected native Level.Time. Require slot machine_generation equal to
counter active generation; slot_generation is independent and <=slot high_water.
Require active supported plain stone7 and exact revision/head; keep nullable item
shape explicit. Preserve all native scalar/codec/registry checks, including the
retained anchor's authoritative physical catalogue. Native install independently
rechecks target clock/chunks/binding after native Game load.

## Account the slot, then run every remaining E1 check

The ordinary observe_checkpoint_entity_owners stays conservative, including
TrialNone with retained persistent participation. No boolean permission argument
or arbitrary token is exposed to ordinary callers. Add one fixture-only method:

    Game::observe_slot_epoch_entity_owners()
        -> Result<EntityOwnerObservation,EntityOwnerObservationError>

It validates the actual active OW World slot's healthy/resident metadata and full
borrowed export/strict decoded supported projection against native counter/host
context, and records one explicit accounted-slot count in its internal level row.
The common inspection body then runs loaded_entities, EntityMemory, unsupported
families, finite/resident drops, hook, portal travel/arrivals and all existing
authority/guest/dimension checks. Every other dimension still uses ordinary slot
refusal. Do not catch the first WorldSlotTrial error and return success.

Only the new NativeSlotPatch capture profile calls this method; old archival and
counter profiles call ordinary E1 unchanged. W1 and PLAYER checks remain exactly
in force. The profile additionally requires exactly one loaded OW, lit3x3[-1..1]^2,
two actual players in resident columns, no drops and pristine Streaming order0.
Use real ChunkTickets::update(Sources) and actual tick_counter_trial to reach3/T,
including same-tick duplicate proof. No lighting/update pump or fresh=true claim.

## One payload member and explicit profile-safe envelopes

New exact member: world/data/horizon/slot_checkpoint.nbt, raw strict native slot
frame<=1MiB. CounterV2 remains world/data/horizon/counter_checkpoint.json<=16KiB;
research/counter.json remains unchanged archival diagnostics. Add only the exact
slot member/parents to declared footprints; no new directory-prefix permission.

Current verify_image accepts only Manifest schema1/kind fresh-native-capture-cell,
while generic world/* members could include a slot file. Current selection treats
verification errors as corruption fallback. Therefore simply rejecting a slot in
old verify_image is NOT sufficient: it could silently fall back on profile mismatch.

Propose a SEPARATE strict SlotManifest (deny unknown/duplicate fields, same existing
bounded JSON reader) with schema2, kind fresh-native-slot-capture-cell, required
slot_path:"world/data/horizon/slot_checkpoint.nbt", and the existing epoch/identity/
players/dimensions/counter/directories/files fields. One payload member, no extra
marker file. Existing Manifest type/decoder/schema1/kind checks remain exact.
This is a fixed cell discriminator, not an extensible public manifest framework.

Factor common identity/player/path/member/hash/physical verification into a private
borrowed envelope view with exactly V1 and Slot variants. verify_image remains the
strict V1 wrapper. A bounded common header/physical-field probe recognizes the
known schema2/kind independently of slot_path; the STRICT closed SlotManifest
decoder finishes after selection. Do not deserialize SlotManifest first and send
its missing-field/type errors into the old generic corruption-fallback arm.
Wrong build/content identity remains a hard refusal across all candidates.

Selection ranks physically complete whole envelopes by epoch. For an unambiguous
known Slot discriminator, required-profile-field/member failures are retained as
finite NON-FALLBACK refusal candidates in that same ordering, never as successful
images. This covers a missing declared slot entry even when the actual reserved
file is present but unlisted: its profile error takes precedence over generic
tree-mismatch fallback. If such a refusal is the newest eligible candidate, return
it before destination creation; do not select an older complete epoch instead.
An older refused semantic candidate does not override a newer valid complete one.
This is a fixed two-case selection result, not a callback/decoder registry.
Only genuine declared physical corruption with otherwise coherent profile fields
enters existing whole-envelope fallback. Unknown/unparseable physical-envelope
behavior remains existing fallback; never reinterpret a recognizable known-profile
failure as unknown merely because its required slot_path is missing or malformed.

Apply explicit caller-profile checks AFTER whole-envelope selection and BEFORE
copy_selected_epoch creates a destination; retain the fixed profile through the
private copy path so an old wrapper cannot omit that check. The bounded cases are
ArchiveOnly, CounterOnly and ActiveSlot; no callback/trait plug-in mechanism.

| Selected envelope or known-profile refusal | ArchiveOnly recovery | CounterOnly recovery | ActiveSlot recovery |
| --- | --- | --- | --- |
| V1, no native frame/slot member | Existing archival copy | Refuse missing counterV2 | Refuse missing members/profile |
| V1 plus valid counterV2, no slot member | Existing archival copy; no live restore claim | Existing counter install | Refuse missing slot/profile |
| SlotManifest plus both members | Refuse incompatible profile | Refuse incompatible profile | Require all semantic/native checks, then copy/install |
| Known Slot schema2/kind, slot_path missing | Refuse profile failure | Refuse profile failure | Refuse required-field failure; no fallback/copy |
| Known Slot schema2/kind, slot_path wrong type/value | Refuse profile failure | Refuse profile failure | Refuse strict-field failure; no fallback/copy |
| SlotManifest files list missing reserved slot entry, file absent OR unlisted | Refuse profile failure | Refuse profile failure | Refuse required-member failure; no fallback/copy |
| Slot bytes invalid NBT or semantically inconsistent, hashes recomputed correctly | Refuse selected incompatible profile | Refuse selected incompatible profile | Refuse selected semantic failure; no fallback/copy |
| Legacy V1 with reserved slot member present | Refuse selected incompatible owner member | Refuse selected incompatible owner member | Refuse missing proper Slot profile |
| Proper Slot profile/member declaration, declared file physically missing/hash-bad | Existing complete-WHOLE-epoch fallback | Same whole-envelope fallback | Same whole-envelope fallback, then profile checks |

For the missing-member semantic row, remove the file AND its manifest member and
recompute the manifest while retaining SlotManifest's required slot_path discriminator;
physical verification succeeds and the new adapter must refuse without fallback.
Deleting a declared hashed file is genuine physical corruption and follows the
last row. Test required slot_path missing, Int instead of String, and an incorrect
String separately with the other files/identity/epoch intact. Test invalid slot
NBT with corrected member length/digest separately from stale-digest corruption.
Use a newer bad candidate plus older valid image to prove each no-fallback row;
also prove genuine physical corruption falls back to one complete older envelope.
Do not change v1 decoding into a permissive schema2 superset. Old capture
publishers must also refuse a reserved slot member copied from a source tree.

## Actual Task publication and private recovery

Proposed entry points inside the test-only Game adapter:

    Game::capture_slot_fixture_epoch(CaptureRequest) -> CaptureCompletion
    recover_slot_fixture_epoch(archive,identity,destination,binding,items,
                               expected_world,host_target) -> SlotRecovery

Capture runs the fixed profile checks, obtains counterV2 and borrowed slot bytes,
projects canonical PLAYER in the same continuous Task, performs native save/flush,
and reruns profile/projection equality after save. Reuse Material.copy/overlay;
write the two exact native frame members before existing sync_members/manifest
temp/final/rename/ack phases. Recheck the completed physical Slot envelope and
all cross-record/native-clock semantics immediately before rename. No new fake
native-success hook. Uncertain after attempted rename remains Uncertain until
durable acknowledgement, using the existing qualified mechanism/fsync order.

Recovery selects one complete envelope, checks caller profile and all semantics
before copy/open, copies only that epoch, acquires fresh SessionLock, then uses
actual native nine-chunk/PLAYER load. Factor existing helper access rather than
calling old machine-only recovery and appending a slot from elsewhere.

Counter and slot install are SEQUENTIAL, not an atomic two-owner transaction.
Keep the fresh loaded Game private/unpublished until both succeed. Install counter
first, then consume the already decoded slot using the actual target Items. Return
the successful pair of new opaque keys only after both installs. A slot refusal
retains its owned RejectedSlotFrame and keeps the private partial Game/destination
quarantined for diagnostics; never expose it as recovered success, overwrite the
old live Game, drop a caller's returned item or claim filesystem rollback. Native
entropy/resource/install failures after valid copy may leave a fresh diagnostic
destination; the no-destination guarantee applies to PRE-COPY validation refusals.

## Six actual Game primary groups and retained gates

Freeze exact primary names at source review; proposed names:

- slot_epoch_native_task_conserves_ten_as_player_three_and_slot_seven: actual
  as_guest/cell lend10->3+7; actual counter3/T; Task publication; exact canonical
  owner/external PLAYER, native9chunks and both frames; native fresh load/direct
  install; new keys; Tskip/T+1=4; optional real transfer preserves total10; archive
  bytes unchanged. No post-load refill or second capture.
- slot_epoch_accounted_owner_keeps_every_other_refusal: otherwise valid slot7
  plus each existing E1 family/map/loaded/hook/portal refusal, W1 named categories,
  PLAYER/acting-guest and nonpristine streamer checks; exact before/after owner,
  payload allocations, heads, player values, clocks/RNG and disk footprints.
- slot_epoch_rejects_unsupported_and_detached_slot_modes: Pending/Dormant/Faulted/
  Detached and TrialNone retained participation, wrong resident/state/binding,
  malformed/unsupported projections; ordinary E1 still refuses active AND retained
  detached mode. No take/hide/clear or unstamped empty record to gain admission.
- slot_epoch_profile_matrix_and_semantic_corruption_refuse_before_copy: exercise
  every matrix cell; recompute file digests for wrong world/dimension/position/
  package/type/generation/head/slot count and counterV2/v1/slot/native Time mismatch.
  Use identical physical blocks to isolate independent host target checks; selected
  bad slot never stitched from an older epoch; refusal destination remains absent.
- slot_epoch_whole_image_interruptions_and_definite_completion: reuse real
  publisher/copy interruptions, complete whole-epoch hash-corruption fallback,
  actual ten-second observer timeout followed by definite completion and controlled
  Uncertain receipt handling. No fabricated native result. At most one ignored
  auxiliary child in this same test source, selected only by an explicit bounded
  parent plan; never counted as an additional primary success.
- slot_epoch_private_install_failure_and_fresh_authority: stale source counter/
  slot keys and plans refuse against recovered Game; occupied/wrong-clock native
  targets preserve their owners; rejected owned frame retained, private partial
  candidate never reported successful; source and immutable epoch unchanged.

Keep all six old counter groups, six old capture groups, slot53 native regressions,
PLAYER/E1/W1/lifetime tests and feature/lint checks. Source fixtures may author10
instead of old7 locally; old helper bodies/expectations remain unchanged. Source
and recovered authorities are alternative test histories, not summed inventories.

Coordinator owns exact source/binary/resource manifests before/after, isolated
directories, serialized Cargo/native/resource/feature/broad checks and retained
failed attempts. Require fresh independent Sol High design review before release,
source review before execution, and different final evidence/overall review.
Decide fresh ordinary both-mode GUI by exact changed source paths: any production
levels/owner-observation body change requires fresh GUI; test-only adapter changes
do not by themselves establish ordinary behavior. Do not assume old b17/f90 reuse.

Concrete design choices needing review: separate SlotManifest/profile factoring,
borrowed slot inspection, the one explicit accounted E1 path, helper visibility
and private sequential-install failure ownership. None is an accepted implementation
yet. Report concrete path/ownership blockers before widening the eight-path cap.
No additional owner product decision is presently needed for this bounded proposal.
