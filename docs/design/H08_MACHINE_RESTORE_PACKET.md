# H08 native counter identity and restore cell

DESIGN ACCEPTED FOR BOUNDED IMPLEMENTATION after fresh independent Sol High review
of amended proposal SHA256126948241b04eb23c3021d0001675ab138dfea24f84bdcac32d8e76628896899.
The initial proposal lacked independent host target context and explicit native
payload chronology invariants; the reviewed amendment resolves both. Source,
runtime/final evidence, S1/G1 and publication acceptance remain pending.
Exact clean accepted base88bfcb05c4ac9c8b817c171e28ea6e5527b11cb5,
tree ffb1c243f7d630f85a364b893845305857f058b1. Prior capture mechanism is fully
accepted; this proposal changes no gate. Private accepted checkpoint67b1bdd is
coordinator-owned. Read native sources only; this one document was the sole author write.
No engine edits/execution/import/WSL/Cargo/tests/fmt/network/commits. Audit:
H08_MACHINE_RESTORE_INTEGRATION_AUDIT.md SHA256
f4fb246cbbf112941f0a25d521edbc5028617cc5a22d6d20ba0386da90d0e447.

## Objective, exact limit and stop

One actual authoritative Game reaches counter3,last_advancedT through its REAL
native counter phase. Its first immutable epoch contains coherent PLAYER/world
files, the existing v1 archival diagnostic, and a NEW durable frame. Recover that
ONE complete epoch into a fresh Game, validate native chunks/clock/type/binding,
install native committed state with the same durable placement and NEW runtime
identity, skip atT, then advance exactly once to4 atT+1. No replay to reconstruct3.

First cell publishes ACTIVE healthy fresh placements only. Absent/retired allocator
frames are supported by the defining-module codec/checked World restore tests,
NOT claimed whole-epoch tombstone capture. Existing v1 material requires active
Record/no last retirement; do not fake an active archival record for a tombstone.
Stop before a SECOND capture of the recovered Game. Reloaded ChunkExtras.root
contains native InhabitedTime/structures/PostProcessing; W1 correctly refuses it.
No clearing extras, hidden whitelist or W1 bypass is authorized by this packet.

This is counter-only native persistence/install/resume, not item-slot restoration,
ordinary runtime admission, a registry/SDK/journal, existing-world migration,
in-frame capture, automatic callbacks or S1/G1 acceptance. Full item-bearing,
transaction, history, disconnect/unload and two-client goals remain unchanged.
No architecture/owner decision beyond normal bounded design acceptance is needed.

## Hard cap: eight source paths

Paths relative to horizon/crates/horizon-client/src:

1. counter_trial.rs: checked private Record/Trial restoration/admission helpers,
   bounded runtime-ID reservation and opt-in fault support; no public Record fields.
2. world.rs: ONE paired optional metadata field initialized None, new defining
   child declaration and minimal opt-in lifecycle hooks/enable guard. Preserve
   existing default-None transient behavior.
3. NEW world/counter_restore.rs: defining World child owns metadata/types/native
   initialization/restore/read/export and focused inline native tests. Public
   paired interfaces are needed by binary tests; Domain itself stays pub(super).
   No lib.rs change: World is already a public module.
4. game/levels/capture_integration.rs: small PRIVATE fixed capture-profile factor,
   selected-epoch/copy extraction and new child declaration. Reuse its actual
   bounded codecs/copy/manifest/fsync phases; retain v1 decoder semantics.
5. NEW game/levels/counter_restore_integration.rs: Linux+test+paired child of the
   capture module; strict v2 adapter, fixed additional-member phase, Game capture/
   recover/install glue. No ordinary runtime caller.
6. NEW game/levels/counter_restore_integration_tests.rs: child of that adapter,
   actual resource-backed Game tests below.
7. game/levels/capture_integration_tests.rs: ONLY pub(super) visibility for setup,
   expected_player, expected_advancements, expected_stats. Preserve their bodies
   and every original test. New tests extend setup locally before their baseline.
8. game/integrated/capture_completion_tests.rs: fixed machine-capture dispatch
   helper, same native Task/run_tasks/Send completion discipline. No callback API
   and no generic Integrated::call change.

NO mechanical publisher split is needed: new adapter is a descendant of the
existing capture module and can call its private primitives. Factor the current
recovery selector/copy once; do not duplicate its1488-line source. New capture
wrapper has the same short phase sequence plus one fixed frame-member write.
Private CaptureProfile has exactly ArchivalOneColumn and NativeCounterPatch;
only the latter admits the explicit9-column OW profile. V1 wrapper always selects
ArchivalOneColumn. If readability requires another child, report exact path/reason
BEFORE widening this cap. No dependencies, normal loader, network schema or SDK.

World constructor/mutation bodies change, even behind default-None hooks. Require
FRESH ordinary integrated AND in-frame Game validation; do not reuse b414 GUI
proof on the assumption that all additions are dormant.

## Owners, keys and proposed interfaces

All library additions cfg(feature="paired-packages"); Game/filesystem/test adapter
cfg(all(test,feature="paired-packages",target_os="linux")). Other native targets
must still compile. Names below are proposed interfaces, not accepted stable APIs.

    WorldId([u8;16])                    // canonical v4 UUID, durable
    PlacementKey { world:WorldId, dimension:String, position:[i32;3], generation:u64 }
    RuntimeCounterKey { session:opaque, placement:PlacementKey, instance:u64 }
    Domain { world, dimension, high_water, active_key, retired_key,
             runtime_session, status }  // METADATA ONLY, private defining child
    World.counter_domain: Option<Box<counter_restore::Domain>>

Trial::Record/last_retirement remain the SOLE live active/retired counter payloads.
Domain never caches another counter value. A frame is an immutable projection,
not a second live owner. Domain survives Trial removal/take for this World
lifetime, retaining high_water even without an active record. Default None leaves
existing transient trial behavior unchanged. No Game field is added.

    NativeBinding::from_prepared(prepared:&PreparedBundle,type_id:&str,
                                registry:&BlockRegistry) -> Result<Self,RestoreError>
    CounterTarget { dimension:canonical-native-ID, position:[i32;3] }
    World::initialize_counter_domain(binding:&NativeBinding,target:&CounterTarget)
        -> Result<RuntimeCounterKey,...>
    World::restore_counter_frame(binding:&NativeBinding,expected_world:WorldId,
                                target:&CounterTarget,frame:&CounterFrameV2)
        -> Result<Option<RuntimeCounterKey>,...>
    World::read_persistent_counter(&RuntimeCounterKey) -> Result<Record,...>
    World::counter_checkpoint_frame(saved_game_time:i64) -> Result<CounterFrameV2,...>
    World::counter_domain_snapshot() -> Option<DomainSnapshot>  // metadata only

NativeBinding has private fields/no unchecked public constructor. It checks actual
PreparedBundle identity, exactly one selected machine binding, owned namespaced
block/type, schema1, empty properties/state plan and physical template
minecraft:iron_block, resolved through horizon_world::registry::BlockRegistry.
This remains the existing cell's declared template experiment: canonical symbolic
block ID and physical vanilla template are distinct fields, not a claim that this
fixture activated a registered custom block. No numeric registry ID is persisted.
Native backend ONLY; Guest backend restore refuses until separately qualified.

Initialize requires no existing domain/trial and resident matching state. It
obtains world UUID and an independently fresh session via existing
horizon_protocol::crypto::random_bytes. Failure refuses; no seed/path/player UUID,
clock-based random fallback, constant41 or user fresh=true. AuthorityScope retains
its existing transient meaning; construct a fresh nonzero transient scope for the
runtime configuration. No mutation_observer semantics change.

Runtime key has private construction and fresh Arc/session identity, placement
and runtime instance. Check ALL before returning current Record, plus healthy
resident state. A stale source key fails against recovered Game even when source
World remains alive. A serialized prior session descriptor cannot become the new
opaque key. Fresh session generation is not a new transport protocol; real network
epoch negotiation remains open. Knowing a native key is not player authorization.

Restore validates frame, metadata binding, world/dimension/position, observed
canonical state, record/allocator arithmetic, healthy target and native clock
BEFORE changing World or reserving runtime IDs. Reject an existing Domain or Trial,
including dormant domain; restore may not overwrite a higher live allocation head.
CounterTarget is independent trusted HOST input, never derived from candidate
frame/V1 fields or a scan for matching blocks. World has no dimension and package
metadata has no placement. The Game adapter takes dimension from actual Game.dim
and supplies its declared selected position. Compare BOTH to the frame before
entropy, runtime reservation or publication, alongside independent expected_world.
Generate the fresh transient AuthorityScope world instance only after validation;
its dimension comes from the host target, retaining existing scope semantics.
Target world.game_time MUST equal frame.saved_game_time; first cell uses nonnegative
clock. For EVERY active and retired native scalar payload, counter==0 IFF
last_advanced==None. Present advancement times are nonnegative. Active
last_advanced<=saved_game_time; retired last_advanced<=retirement.game_time
<=saved_game_time, with nonnegative retirement time. If both payloads exist,
retired generation must strictly precede active generation; both are positive
and <=high_water. Preserve valid exact u64 values including u64::MAX; its next
advancement faults normally, never normalize the saved value. Resident native state must equal the
frame's observed_state; Active also requires the physical machine template.

Then build local metadata/Trial, using a defining-module private factory in
counter_trial.rs. It restores exact counter and required-nullable last_advanced,
not saved instance/Arc/residency authority. Reserve fresh runtime IDs only after
validation with checked CAS (at most active+one retired diagnostic). Exhaustion
returns error, no partial World publication. Publish prepared Domain and Trial
with no subsequent fallible step. Refusal preserves prior Trial/Domain/chunk/
RNG/clock/PLAYER values and identities; global runtime reservations are never reset.
Use local AtomicU64 in defining allocator tests, not mutation of LAST_INSTANCE.

RestoreError finite cases: InvalidFrame, IdentityMismatch, BindingMismatch,
ClockMismatch, Occupied, NotResident, StateMismatch, GenerationExhausted,
RuntimeExhausted, Faulted, Detached, Entropy. Binary completions convert errors on
owner thread to owned Send diagnostics; never send native Box<dyn Error>.

## Exact opt-in lifecycle/failure policy

Existing World hooks delegate to new metadata-aware behavior ONLY for DomainSome;
DomainNone executes the old paths. Ordinary enable_counter_trial refuses when
DomainSome, including absent Trial, preventing an unstamped transient zero record.

| Native event | DomainSome behavior |
| --- | --- |
| Successful set at another position or same-state no-op | No generation/payload change. Same-state write is NOT a semantic replacement proof. |
| Watched iron -> air/other state | Retire current Record through existing native retirement ownership; move only its generation key to retired metadata, clear active key, retain high_water. Native last_retirement retains scalar payload. Repeated nonmachine writes do not overwrite a nonexistent active owner. |
| Watched nonmachine -> matching iron | Compute checked high_water+1 and reserve fresh runtime ID before publishing new Record0/last_advancedNone + key + head together. The physical block write is already committed; on allocation error retain old retirement/head, latch explicit domain/trial fault, admit NO unstamped record. Do not claim block rollback. |
| Unload via remove_streamed_chunk | Existing Record becomes nonresident; retain durable generation/value/head. Reads/advance unavailable; no unload-as-air. |
| Insert matching chunk with replaced=false | Restore residency of SAME placement/payload/head; mismatching state faults and retains old value/key, does not normalize it. |
| Insert with replaced=true | Conservative AmbiguousReplacement fault: retain old Record payload/key/head, mark nonresident, refuse read/advance/export. Native chunk insertion itself is not rolled back. Never call ordinary retire/re-admit and silently make zero. |
| Explicit take_counter_trial | Preserve ordinary returned owned Snapshot; metadata becomes Detached, retains head/retired key, blocks export/restore/ordinary enable/re-admit for that lifetime. Returned payload stays caller-owned. No reset/ack service in this packet. |
| Generation/runtime exhaustion | Checked refusal/fault, old retirement/head retained, no default generation1 or zero-valued active owner. Faults cannot be cleared by ordinary enable. |

The metadata status/fault is checked by persistent read/export and active native
phase; a faulted opt-in Trial cannot advance through legacy phase methods. A small
new persistent-domain Fault variant may report it through the existing native
counter phase; default transient diagnostics remain unchanged. Exact take path
must set Detached when it transfers the native Snapshot, not clone payload into
Domain. Full Game destruction/failed idle shutdown retention is still NOT supplied
by this field; no ordinary Game path enables it here.

A native remove/re-place air->iron must change generation even when new type equals
old type. Preserve generation across actual unload/reload. The unresolved broader
same-type placement/cause API is not invented from World::set scalar equality.

## V2 frame, v1 compatibility and publication

New bounded world member: world/data/horizon/counter_checkpoint.json. V1
research/counter.json remains exactly archival with live_generation_restore=false.
Existing v1 manifest schema/decoder remains exact, not a permissive V2 superset.
Its ordinary hashed world-member list can include the new file without a format
change. V2 adapter has a SEPARATE strict decoder and must require that exact member.

CounterFrameV2 typed closed schema, deny unknown/duplicate fields, bounded serde
Seed/Visitor before retained collections, all nullable fields explicitly required:

    {format:2,kind:"native-counter-checkpoint",world_uuid:canonical-UUID,
     dimension:native-ID,position:[i32;3],allocation_high_water:u64,
     package_identity:sha256,type_id:namespaced-ID,save_schema:1,
     binding:{block_id:symbolic-ID,template:"minecraft:iron_block",properties:{}},
     observed_state:{name:canonical-ID,properties:sorted-map},saved_game_time:i64,
     placement: Active{generation:u64,counter:u64,last_advanced:i64-or-null}
                | Dormant,
     last_retired: null | {generation:u64,counter:u64,last_advanced:i64-or-null,
                           reason:closed-native-reason,game_time:i64}}

Use a tagged object for placement; no implicit Option default. Active generation
and retired generation are positive and <=head; current and retired generations
cannot alias different placements. Native export/restore correlates Trial values
with Domain keys, enforces the explicit payload/time/generation invariants above
and retains exact u64 values. All nullable fields remain explicitly required.
DomainNone,
Detached or Faulted cannot produce a valid frame. No items/components are encoded.

ACTIVE epoch adapter: fixed NativeCounterPatch profile requires exactly one OW
World containing3x3 coordinates[-1..1]^2, no other loaded dimensions, healthy
active opt-in counter/no last retirement, two actual players in resident columns,
all prior PLAYER/E1/W1 refusals and zero Streaming submissions. It projects both
records in the SAME continuous native owner Task. Require equality of V2 active
counter/last_advanced/type/schema/symbolic state/position/dimension with v1 archival
record AND package_identity with manifest Identity.counter_content/actual binding.
V1 scope/instance remain diagnostics, never restored credentials. Contradictory
records refuse before publication/install, even with recomputed file digests.

Use existing Material.copy/overlay, then one fixed v2 member write before existing
sync_members/manifest-temp/final/rename/ack phases. No callback argument. Strictly
recheck the V2 member against completed v1 envelope before rename. Preserve
Uncertain outcome after attempted rename through durable acknowledgement; same
Linux fsync order, no latest pointer/hardlink/live overwrite/mixed epoch.

For recovery extract the existing private select-complete-epoch and copy-selected
steps once. V2 verifies the selected whole v1 envelope and v2 cross-record/clock/
identity checks BEFORE copying/opening a destination. Missing/invalid/mismatched
V2 on the selected complete envelope is a compatibility/refusal outcome in this
cell, never an invitation to take a frame from an older epoch. Existing v1
corruption fallback semantics stay unchanged. Copy one verified epoch to fresh
destination, obtain NEW SessionLock; native Game install happens only afterward.
The pre-copy V2 clock comparison uses the actual selected epoch's native
Level.time, not just agreement between V1 and V2. Test internally consistent V1/V2
values that disagree with native saved time, with destination still absent. Also
test checked install independently against a target World with the wrong clock.

Dormant/retired Frame roundtrip + native World install/high-water tests are LIB
proofs only in this packet. V1 active-only material is NOT extended to publish
dormant epochs. General tombstone epoch publication and recapture after reload
remain explicit next integration work; this cell establishes live nonzero install/
resume without falsely claiming those cases.

## Actual Game test setup, resume and bounds

Reuse unchanged Fixture/PLAYER expectations via the four tiny visibility changes.
Locally take the initial TRANSIENT zero trial before the new baseline (hold its
Snapshot), build8additional authored empty columns, mark authored patch lit, mark
all9modified, then initialize the persistent domain. Active Persistent/SaveDir
binding remains correct and all Streaming.order remain0. No native load/update/
lighting pump is disguised as setup; this is a declared complete storage fixture.
Use actual ChunkTickets::update(Sources with the two real players at0,0, distances2)
and Game::tick_counter_trial. Its real authority/ticket/storage checks select work.
At three declared increasing game_time values reach3,last_advancedT, with a repeated
same-tick call proving no extra work. Counter phase alone schedules no streaming,
BE/mod/function/network work; explicitly source-review that bounded premise.

Capture through the actual Task/run_tasks helper. New fixed dispatch_machine_capture
uses the same independent Send completion, not an observer timeout as cancellation.
Recover native Level/chunks/player data into a fresh authoritative Game without
calling the old setup that creates another trial. Set game_time from Level.time,
not day_time/wall time; load actual9chunks via SaveDir. Preserve saved extras;
activation for resume may use native insertion before Trial install. Initialize
native player motions through existing canonical playerdata/as_guest path.
Checked-install is the final publish step after all potentially failing setup.
AtT native phase skips, atT+1 reaches4 once; false running, absent tickets and
unloaded center skip. No second capture, guest evaluator or ordinary tick loop.
Absent-ticket setup uses actual update(Some(Sources{players:[],forced:[],...}));
update(None) means permission to tick everywhere and is NOT an absent-ticket test.

Retain current entry/file/NBT/JSON/manifest bounds; new frame<=16KiB, nodes<=512,
depth<=8, strings<=256, properties empty for this prototype; at most one active
and one native retired scalar record. One OW patch means four region coordinate
files(rx,rz in{-1,0}x{-1,0}); allow exact POI/entity files only for demonstrated
native producers, plus existing precise metadata/player/random_sequences paths.
No directory-prefix permission. V2 adds only its exact world/data/horizon file
and parents in staging/recovery; source authoritative domain remains in World
until checkpoint. Preserve exact source/old epoch/staging/destination footprints.

## Required tests and qualification

Bound defining-module native test groups: private factory/allocator, actual private
remove_streamed_chunk/insert_streamed_chunk domain lifecycle/keys, exhaustion and
frame validation. Those pub(crate) library seams are NOT callable from binary
resource tests; do not widen them for testing or substitute direct ChunkMap edits. Bound resource groups (six primary ignored names to freeze):

- machine_restore_nonzero_epoch_installs_and_resumes_native_game: real Task,
  counter3/T, positive canonical PLAYER files+native9chunks, exact durable key/head,
  fresh session/runtime key, atTskip/T+1=4, no replay and no second capture.
- machine_restore_rejects_frame_binding_clock_and_occupied_targets: wrong world,
  dimension/position/state/type/schema/package/clock, missing/null violations,
  overflow/duplicate fields and existing owner; exact pre-existing state unchanged.
  Wrong target dimension/position must also be tested with frame, V1 and digests
  mutually consistent, yet disagreeing with independently supplied host target;
  use identical resident template blocks where useful so physical mismatch cannot
  mask missing target validation. Explicitly test zero+Some, nonzero+None, negative/
  future active times, retired times after retirement/retirement after saved time,
  and retired generation >=active generation, all nullable fields present. Include
  direct native factory tests so dual-record mismatches cannot mask these checks.
- machine_restore_runtime_and_generation_keys_are_not_reused: source/recovered
  Games coexist; old session/runtime key refused, air->iron changes gen, old gen
  refused, high-water>active-generation restore allocates above head. Include an
  isolated recovery process/fresh nonce proof; never deserialize Arc or set globals.
- machine_restore_take_and_enable_refusal_keep_metadata: actual PUBLIC native
  take retains its returned Snapshot while World metadata becomes Detached; old
  enable/restore refuses and head stays unchanged. Private unload/reinsert,
  replaced=true and exhausted-head cases run in the defining LIBRARY groups above.
  Dormant codec/install cases are not completed epoch publication.
- machine_restore_dual_record_and_whole_epoch_failures_do_not_install: contradict
  v1/v2 fields with recomputed hashes, missing frame, interrupted copy/publication,
  Uncertain completion; no destination or old World mutation before validation.
- machine_restore_ineligible_and_faulted_native_phases_do_not_advance: actual
  running/tickets/residency gates and latched faults; retained key/value/head and
  PLAYER/world data; immutable epoch unchanged after live resume.

Native codec/factory tests must cover nonzero u64 and required nullable values,
no default reconstruction, occupied-target nonmutation and CAS exhaustion without
global allocator edits. Some entropy/OS sync failures may be source-only; no fake
native-success hooks. Include actual resource fixture context, precise expected
values and old-source retention; compilation alone does not prove install/resume.

Coordinator owns exact frozen source, fresh independent source/evidence reviews,
serialized builds/resource/native/feature/broad checks, and NEW both-mode ordinary
GUI checks for changed World construction/mutation paths. Keep prior failure
receipts and all v1/PLAYER/E1/W1/lifetime/slot regressions. Do not run later stages
or publish the engine. No full S1 reduction follows from successful counter-only
restore; item-bearing WorldSlot still needs move-only payload/pending owners and
a fresh native SlotKey under the same coherent epoch/generation discipline.
