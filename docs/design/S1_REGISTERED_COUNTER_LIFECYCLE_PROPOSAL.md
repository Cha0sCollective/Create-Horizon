# Registered counter placement and retained lifecycle proposal

Status: PROPOSED DESIGN ONLY. No engine source authority, ordinary Running,
startup/close activation, public publication or S1/G1 acceptance is granted.

## Prerequisites and bounded outcome

Require independent FINAL acceptance of registered binding proposal
S1_REGISTERED_COUNTER_BINDING_PROPOSAL.md, exact proposal SHA256
3ec3363a04a772cec0a595b4f9f1a7847b8792de7b1573b9ff94ed93e02b3033,
on its accepted clean B/D combined foundation. Login input
5b41f44fe83b53fc6b7c4dfe550cc0bc1aca5b83 and dropped-item input must each have
their own FINAL acceptance; their composition, registered successor and genuine
materialized Game fixture need exact clean accepted pins and a private durable
checkpoint. Refresh the actual APIs, literals, resources and fixture availability
at that successor before this proposal receives independent Sol High review or
source authority. Conditional conceptual acceptance, SOURCE CLEAR or a build is
not any of these FINAL gates. No implicit merge is authorized.

This research inspected the frozen dropped-item worktree based on retirement
FINAL ab239890cd62595889f7f0302cd1396a5e2c547c. The registered APIs and fixture
referenced here are future prerequisite results, not present implementation.

Discover exactly one registered schema1 propertyless iron-template counter from
an ordinary authoritative player or command placement. Keep its fixed identity,
independent counter/slot generation heads, original retirement custody and D's
real dropped ItemEntity through removal, replacement, actual chunk inactivity,
unload/reload and checked active/dormant recovery. First placement must publish
physical block, counter, slot and one-use grant spending together. It cannot
place an unowned alias and then report success after fallible bootstrap.

One captured type and one claimed target per World remain the closed profile.
This is not a multi-machine map, arbitrary property system, placement scan,
general adoption framework or inference of generation history from block data.
Default/Legacy behavior and the explicit LegacyIron research profile stay intact.

## Existing routes and retained limits

Re-resolve these source anchors at the accepted prerequisite pin:

- Player BlockItem placement in game.rs calls with_level and
  Behaviours::place_block. GameLevel::set_raw calls World::set. Behaviours uses
  set_raw refusal to return PlaceOutcome::Failed; the successful placement branch
  alone reaches consume_held. Actual mining calls player_destroy_block through
  the same LevelAccess adapter.
- commands.rs::place_state uses the behavior library through level_set or its
  strict set_block branch. It currently ignores the write result and returns
  true; it also removes an old block entity before writing. This caller needs
  truthful refusal and deferred covered-alias cleanup until the staged raw write
  actually succeeds. A successful read-only preflight alone is insufficient.
- World::set is the committed resident storage seam and already notifies
  installed counter/slot owners. Those hooks do not discover a first target.
- Streaming calls World::insert_streamed_chunk/remove_streamed_chunk. Matching
  unload/reload retains an admitted record and key; resident replacement and
  mismatch fault. Chunk admission must not discover or adopt a machine.
- tick_counter_trial uses actual ticket block-ticking and storage-neighborhood
  eligibility. An inactive resident target holds value; matching reload performs
  no elapsed-time catch-up. Preserve that eligibility calculation.

H04_RESIDENCY_GAME_RESULTS.md and H05_COUNTER_RESULTS.md qualify real mutation,
ticket inactivity and streaming routes, principally for transient trials.
H08_HOST_COUNTER_BOOTSTRAP_RESULTS.md establishes the virgin grant and initial
World marker; H08_MACHINE_RESTORE_RESULTS.md establishes checked native identity
and restoration. S1_PERSISTENT_RETIREMENT_RESULTS.md establishes checked original
slot take, delivery and explicit slot reconcile. None by itself qualifies this
ordinary persistent caller. In particular current registered-binding proposal
allows fixed-target counter readmission before explicit slot reconcile; this
successor deliberately changes RegisteredCube ordinary lifecycle to paired staged
readmission. LegacyIron behavior and its tests must remain unchanged.

## Authentic native context and proposed API seam

Names below are proposals for concrete native APIs, not installed capabilities.

```rust
Game::arm_registered_counter_placement(&mut self, type_id: &str) -> Result<()>;
Game::preflight_registered_placement(&self, position: [i32; 3], state: BlockStateId)
    -> Result<()>;
Game::service_registered_counter_lifecycle(&mut self) -> Result<()>;
World::install_registered_placement_policy(&mut self, binding: &NativeBinding)
    -> Result<(), RestoreError>;
World::set_from_registered_host_level(
    &mut self, position: [i32; 3], state: BlockStateId,
    grant: Option<&mut HostCounterBootstrap>,
) -> Result<(), RestoreError>;
```

The internal Game arming entry selects exactly one genuine retained prepared
catalog owner/type, constructs RegisteredCube through from_registered_prepared
with this Game's actual materialized registry and verifies its real block-item
route. It runs only on the authoritative initial Overworld or an already checked
recovered registered domain. Remote/frontend replicas cannot arm it. Arming
installs an immutable closed binding policy, allocates no placement generation,
spends no grant and writes no block. Wrong profile, ambiguous catalog, different
existing policy/domain or absent genuine materialization refuses unchanged.

The policy is private World state, absent by default and initialized in World's
existing constructor. It is data, not authority. Keep the captured package/type/
schema/profile/physical descriptor pinned; do not accept a command-supplied
binding or raw numeric ID as its replacement. Repeated identical policy arming
may be idempotent; changing it after installation refuses.

preflight_registered_placement is read-only screening only. Its Result<()> is
not a prepared admission capability and does not guarantee later counter-instance,
entropy or reservation work will succeed. No caller may authorize destructive
cleanup on the strength of that result.

GameLevel borrows the actual WorldSession's existing non-Clone
HostCounterBootstrap during with_level. The session retains the native lock and
strong incarnation marker. No grant is moved into World, cloned, serialized,
reconstructed from UUID or created by this packet. Initial first placement needs
that live grant and its existing strong/weak World correlation. Actual guest
player execution may borrow the host-owned context through the existing actor
swap; guest-supplied data supplies no authority. Do not exclude that ordinary
server-player route merely because lan_acting is temporarily populated.

For restored domains the genuine installed domain's session/target identity is
the readmission authority; it does not regain a virgin grant. Ordinary reopened
Worlds with no checked domain cannot adopt an alias. Policy installation or an
empty owner field is never evidence of unused history.

GameLevel::set_raw calls the new native setter. World::set and that setter must
share one private resident-write implementation, with explicit staged admission
input rather than recursive bypass or a second storage implementation. Plain
World::set refuses a policy-covered alias write requiring first admission or
readmission when no valid stage is supplied. Existing unarmed/default Worlds
retain their current storage behavior. No observer queue, callback, SDK map
lending, chunk insertion or stale frame may manufacture a stage.

## Stage before physical publication

The private non-Clone admission stage never escapes the immediate exclusive
World operation. Before the physical write, validate all of the following and
prepare every fallible admission operation:

1. Exact installed RegisteredCube policy, captured binding identity and actual
   alias descriptor/ID; requested position is valid resident Overworld storage.
2. First placement has current air, vacant counter/domain/slot owners, a valid
   nonnegative clock, and this World's live unspent virgin grant. A pre-existing
   alias, an unloaded-as-air read or an unrelated resident replacement is not
   virgin placement. No-op writes cannot discover a target.
3. Readmission has the same installed target, healthy registered domain/session,
   coherent retained heads/history, no active owner and no pending slot custody.
   Its current state is the exact checked nonbound propertyless descriptor from
   that live domain, such as air or builtin iron. Require a changed write to the
   genuine alias; a loaded/no-op alias is not an adoption route.
4. Check both generation increments before either head changes. First admission
   starts the existing legal initial heads; later admission advances each head
   according to its own history, never by assuming they are equal.
5. Prepare the actual native authority/session, Configuration, counter record,
   complete Domain, empty slot trial/allocation, keys and all needed owned
   bookkeeping. Entropy, checked arithmetic, validation, fallible reservation
   and ID reservation precede physical publication. Do not draw World RNG.
6. Recheck exact current storage, live World/session marker, clock, unchanged
   policy/target and owner state in the same exclusive operation before commit.
   No user callbacks, awaits or worker publication occur between check and set.

Counter Trial currently admits immediately. Add private prepare/publish helpers
that stage its fresh record without replacing the current record or retained
retirement. Reuse the slot's existing prepare/publish construction and validation;
expose only the small sibling-module access needed by the paired World stage.
Do not rebuild a Trial merely to change its retained retirement instance or erase
history. Unpublished process-local instance reservations may leave gaps after a
later preparation refusal; they do not advance persistent heads, spend the grant
or publish a runtime key.

After the resident block changes, publish the already prepared counter/domain
and empty slot, their correlated heads/keys and first-use grant spending without
another Result-producing operation, entropy request, checked increment or
callback. Coordinate existing mutation hooks so the just-published first owner
is not immediately retired by its own placement. Retain ordinary World dirty,
lighting, block-log, observer and block-entity semantics. Required fallible
bookkeeping reservations must happen before the write, not be hidden in a new
post-write helper. This is the normal returned-error contract, not a new general
panic/crash transaction framework.

Every returned preparation/refusal error leaves the physical state, original
owners/payload, persistent heads and grant unspent/unchanged. Missing authority
does not downgrade to placing an unowned alias. A second target refuses before
write; the claimed target is not freed by dormancy or unload. Faulted, Detached,
pending, inconsistent or exhausted state refuses rather than resetting history.

## Removal, custody and explicit paired reconcile

For RegisteredCube, a changed write away from the bound alias retires its active
counter and original slot exactly once. Preserve the counter's last retirement
and both persistent heads. The slot remains Pending until the existing checked
receiver acknowledges empty custody or moves the exact original stack into one
real native ItemEntity. A later alias write must not call today's implicit
persistent_counter_changed admission while Pending remains.

The Game lifecycle service borrows the actual pending key from current World
state, checks the actual dimension/authority and invokes
deliver_pending_persistent_world_slot. Preserve existing item/count/ID/capacity
checks, original stack movement and D's UUID/native persistence requirements.
Delivery failure retains original Pending custody and prevents readmission.
Empty acknowledgment changes no entity ID or particle RNG. Successful delivery
leaves healthy paired Dormant state when the physical target remains nonbound.

Readmission is an explicit paired reconcile step inside the native placement
operation after checked custody completion. Stage the next counter and empty
slot together; do not publish one and then call a fallible initializer for the
other. A recovery-created dormant domain follows the same rules without a new
bootstrap grant. A bare physical alias arriving from an unexpected chunk or
plain setter cannot trigger reconcile. No new phase may tick a counter while
its slot is Pending or while only one of the pair is published.

Call the concrete Game service at an appropriate authoritative tick boundary
before counter advancement, including when simulation is frozen so custody is
not silently stranded by pause. It must not use simulation eligibility to grant
new placement authority or increment the counter. On service failure, retain the
owned pending payload and return/report the real failure; do not clear it to
keep the game or checkpoint apparently healthy. Existing checkpoint Pending,
fault, detached and nonresident refusals remain effective.

## Ordinary player and command behavior

Player placement must use the actual PlaceContext, collision/survival checks and
Behaviours::place_block. An authority/staging failure makes set_raw return None,
which produces PlaceOutcome::Failed. Assert no successful placement statistic,
placed-block trigger, successful place sound or held-item consumption is added
by the refused placement branch. Existing unrelated use/failure behavior remains
vanilla. Use a survival control to prove successful placement consumes exactly
one and refusal consumes none; Creative correctly retains its held count.

For commands, read-only preflight may reject an obviously invalid policy-covered
alias attempt early, but all covered-alias old-owner cleanup must be deferred
until the actual staged raw write succeeds. In particular do not run
place_state's early jukebox_removed or block_entities.remove after preflight and
before fallible admission staging. A healthy Dormant target may be a resolved
propertyless beacon or end gateway with a real native block entity; later staging
failure must preserve that physical block AND its complete old native owner.

The existing behavior implementation in level/update.rs::chunk_set_block_state
already calls level.set_raw before remove_block_entity. Reuse that ordering for
the covered-alias command branch: perform successful staged physical/counter/slot
commit first, then remove/clean up the old block entity exactly once through the
native command-appropriate path. Preserve the command's existing skip-block-entity-
side-effects/update semantics, including strict mode; simply allowing a normal
neighbor-update branch to spill old contents is not an equivalent fix. Coordinate
the native World's state_changed hook and behavior cleanup so positive replacement
neither retains stale old-owner metadata nor duplicates removal or side effects.
This is a bounded commands.rs/GameLevel/World ordering change, not permission to
edit the behavior library. The default/nonregistered command path remains intact.

Propagate the actual behavior-layer write outcome, including strict mode,
instead of reporting true unconditionally. Do not treat a failed write as success
because the desired alias was already present without owned admission. Keep
legitimate NBT-only command results. Every returned admission failure, including
one after successful preflight, preserves old physical state, full native block-
entity bytes and related old-owner metadata, custody, heads and grant state.

/setblock destroy is already a multi-step operation: destroy_block_dropping can
remove the original block and produce normal physical side effects before its
subsequent placement attempt. This proposal does NOT roll back that earlier
removal when replacement refuses. Its counter retirement and original slot must
remain Pending, or its exact delivered stack must remain a genuine D item if
delivery already succeeded. Report the failed replacement truthfully; do not
claim full command rollback. Test actual air/nonbound storage, ordinary destroy
events/drops and machine custody separately, counting every actual item owner.
Do not delete normal block drops or machine drops to simplify those assertions.

fill/clone reach place_state in part but also have multi-step paths. Do not claim
atomic region operations or general clone admission. Every policy-covered raw
alias write still fails closed at the shared World seam. If preserving their
actual existing semantics safely requires another source path, stop for review.

## Genuine block item and Creative fixture evidence

Source evidence supplies a concrete route, not a runtime result: content.rs
BlockFile.item defaults true; mod_content Plan copies the iron template's block
item and rewrites items.json and block-behaviours.json. Behaviours::item_binding
returns the actual ItemBinding used by placement. At the accepted pin require the
genuine captured fixture to declare/retain item=true and assert:

- Actual Game Items contains the alias with expected ordinary block-item facts;
  actual registry/command mappings agree with that captured materialization.
- res.behaviours().item_binding(alias) exists, has ordinary ItemKind::Block and
  resolves to the same block as Behaviours::block(binding.state_id()). Its wall,
  attachment and game-master restrictions match the approved simple cube.
- The actual player holds that registered ItemStack and real placement resolves
  the actual alias, not builtin iron or a test-only fabricated numeric ID.

Creative::load currently reads creative-inventory.json or creative-tabs.json.
Neither table is in mod_content's JSON_TABLES, so this audit does not establish
automatic custom-alias Creative tab/search membership. Measure actual loaded
Creative search_items/order_of in the fixture and record the result honestly.
For this bounded positive, equip the Creative player through the existing
ordinary /give command using the verified registered item, then use the actual
player placement route. Also run the survival consumption control. This proves
Creative-player placement, not Creative tab discoverability or ordinary Running
Creative UX. Do not inject a fake registry ID or hand-build a Creative list and
call it materialization. If actual block-item lookup/equipping cannot work at the
prerequisite pin, stop: a materializer/item/Creative production correction is an
explicit additional-path or separate prerequisite proposal, never a hidden ninth
edit. No such production change is authorized by this eight-path budget.

## Exact eight-path maximum

All paths are relative to crates/horizon-client/src; this is design scope only.

1. world.rs: private verified registered policy; constructor default; one shared
   resident storage commit seam with staged admission and ordinary hooks.
2. world/counter_restore.rs: policy checks, virgin/readmission stages, live
   grant/session/target validation and RegisteredCube retirement rules; inline
   native failure/nonmutation/history tests.
3. world/slot_restore.rs: minimal sibling access to existing checked slot staging
   and paired lifecycle validation/publication; no generic custody framework.
4. counter_trial.rs: private staged fresh-record preparation/publication keeping
   retained retirement intact; inline exhaustion/stale-stage controls.
5. game.rs: actual GameLevel native context borrowing and raw-set route, truthful
   player result and authoritative lifecycle service call before advancement.
6. game/counter_control.rs: genuine catalog/binding/item checks, explicit dormant
   policy arming, command preflight and concrete custody/lifecycle service.
7. game/commands.rs: read-only alias screening, covered-alias old-owner cleanup
   deferred until successful staged raw commit, and truthful strict/normal
   result while retaining command skip-side-effect/update and multi-step semantics.
8. game/levels/session_checkpoint_registered_tests.rs: reuse the prerequisite's
   existing owned materialized Game fixture; add actual player/command/chunk/
   generation/recovery tests. No new test module/file declaration is needed.

Before authoring, enumerate all World/GameLevel/Trial/Domain/metadata literals
and callers of changed APIs at the actual accepted pin. Do not add required Game
fields that force unbudgeted fixture edits. The eighth test path must genuinely
exist after registered FINAL and provide its real owned-resource fixture. If it
cannot host the controls without another helper-path edit, declare that exact
ninth dependency and obtain a revised bounded design/fresh review before source.
No save.rs, streaming.rs, world_slot_trial.rs, host_identity.rs, materializer,
Creative, item codec, D portal/item helper, network or startup/close edits fit this
budget. Reuse existing APIs honestly rather than faking capabilities to avoid it.

## Defining tests and completion gate

Root alone runs isolated serial qualification after fresh independent exact
SOURCE CLEAR; no Rust test spawns Cargo, Java, Python or another test process.
Missing real resources or skipped positive cases do not pass.

1. In both real IntegratedServer and DedicatedServer Game constructor roles,
   use the genuine materialized package/renderer assets and real host grant.
   Arm policy, equip verified block item by ordinary command, and place through
   actual player use. Repeat first placement through ordinary setblock, including
   strict. Check physical palette alias, zero counter, empty slot, correlated
   fresh keys/heads and one-use grant spending together.
2. Missing/spent/wrong-World grant, replacement World, wrong dimension, Legacy/
   reopen without recovery, wrong package/type/schema/profile/registry/item
   binding, non-air first target, absent column, invalid height, clock error,
   occupied owner, second target and admission exhaustion refuse before alias
   publication. Compare original block/owner/payload/heads/grant plus held counts,
   command result and old-owner cleanup state. Include a genuine healthy Dormant
   target whose actual propertyless beacon/end-gateway state has a real native
   block entity with nondefault persisted data. For BOTH normal and strict
   commands, first prove read-only preflight succeeds, then inject a bounded
   refusal in the actual later admission staging path, such as counter-instance
   reservation failure. Compare the complete native encoded block-entity bytes,
   physical state and related owner metadata, retirement custody, independent
   heads and grant state before/after; a shallow metadata/Eq comparison is not
   sufficient. Use a narrow per-World cfg(test) stage-refusal control or existing
   allocator seam within the scoped paths, not a fake preflight capability,
   callback, process-global exhaustion or extra source file. Positive normal and
   strict replacements must also demonstrate exactly-once native old-owner cleanup
   and unchanged command skip-side-effect/update semantics. Keep /setblock destroy
   as the separately disclosed intentional-removal control below. Plain World::set cannot
   bypass virgin admission or dormant reconcile. Preserve all default/legacy
   counter/bootstrap/slot behavior.
3. Fill the actual slot with a supported durable nested stack, remove by real
   mining and by ordinary command, and observe stale old keys and original
   Pending ownership. Fail receiver ID/capacity/item/context preparation and
   immediately attempt alias replacement: no physical alias or fresh owner may
   appear. Deliver exactly once; preserve the genuine D ItemEntity. Then perform
   explicit paired readmission and prove independent advancing heads, empty new
   slot, preserved retirement history and no accepted old keys.
4. Exercise /setblock destroy followed by refused placement, including Pending
   custody. Assert the already performed destruction and its genuine physical
   side effects, truthful replacement refusal, intact retired slot or retained
   delivered D item, and later successful retry after checked custody. Do not
   assert whole-command rollback or merge physical block drops with machine
   contents accounting.
5. Drive actual Game/Streaming membership through a nonspawn target unload and
   matching reload. Wait on observed authoritative membership and saved column
   arrival, not direct chunk-map manipulation as the positive. Prove same keys,
   payload and heads, no catch-up, real ticket-inactive resident storage with
   continuing world clock, then one eligible step. Keep resident replacement and
   mismatched saved/reloaded state as fault/refusal negatives with no discovery,
   reset or new grant. Native unit controls may additionally target hook edges.
6. Capture/recover/recapture actual registered Active and healthy paired Dormant
   histories using the prerequisite's strict native palette checks and genuine
   participant/item-column arrivals. Cover dormant air and builtin iron, stale
   pre-recovery keys, explicit readmission after recovery without a new grant,
   independent heads and retained rich D item UUID/count/component projections.
   Ordinary load of a physical alias alone must remain unowned/refused; do not
   adopt it by examining a UUID or empty runtime fields.
7. Run the registered materialization/fallback/profile, D owner/native fence,
   retirement/bootstrap/counter/slot/session and applicable feature/lint/format/
   broad regressions unchanged, with fresh exact source/tool/resource pins.

The headless constructor roles are not real-client/default integrated-window or
dedicated network qualification. Later ordinary activation must repeat actual
player and command placement/removal, inactivity/streaming/history and failure
custody with default integrated window/server and dedicated real clients, plus
the promised in-frame coverage. This packet provides no external admission,
player reach/permissions/replay authorization, ordinary startup/reopen selection,
coherent close/retry or Running caller; those remain distinct S1 obligations.

Completion requires the exact clean local candidate, defining successful tests,
different independent FINAL evidence review and private durable checkpoint. No
public release, compatibility/migration promise, S2 activation or G1 promotion
follows from this bounded lifecycle acceptance.
