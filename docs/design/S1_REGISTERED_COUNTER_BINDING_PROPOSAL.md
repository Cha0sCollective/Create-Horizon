# Registered propertyless counter binding proposal

Status: PROPOSED DESIGN ONLY. This document grants no source authority, ordinary
placement/Running activation, public publication or S1/G1 acceptance.

## Prerequisites and bounded outcome

Implementation requires independent FINAL acceptance of both the login component
at clean LOCAL5b41f44fe83b53fc6b7c4dfe550cc0bc1aca5b83 and the dropped-item
successor, their explicitly reviewed clean combined engine pin, a private durable
checkpoint, fresh actual API/palette/materialization checks at that combined pin,
and fresh independent Sol High acceptance of this exact bounded proposal. Neither
the current dropped-item SOURCE CLEAR/build nor login focused results substitute
for those FINAL/combination prerequisites. Re-resolve all source anchors below;
this research inspected the frozen dropped-item worktree based on retirement
FINALab239890cd62595889f7f0302cd1396a5e2c547c. No implicit merge is authorized.

Bind one genuine captured schema1 machine type to its real registered custom
block: a propertyless, single-state cube inheriting the approved iron template
without property overrides. Preserve active AND dormant counter/slot domains,
independent generation heads, retirement history and retained native dropped
items through actual session capture, fresh recovery, readmission and recapture.
The physical custom alias must survive the native block palette; a symbolic
claim or an iron block standing in for it does not qualify.

Legacy iron binding remains an explicit supported research profile with its
existing factory, kinds, active-only session adapters, receipt types and callers.
This packet adds no player/command placement discovery, ordinary loader/close/
Running caller, external admission, generic multi-machine store, orientation,
state properties, custom shapes, S2 activation or S3 axis semantics.

## Representation decision: closed kinds identify physical profile

The same PreparedBundle currently supports the deliberate legacy iron factory
and can also materialize its registered alias. Both have block_id=the alias and
template=minecraft:iron_block. A dormant observed air descriptor therefore does
NOT identify their previous physical binding. Registry presence or the captured
declaration alone cannot distinguish them. Do not infer mode from observed air,
registry numeric IDs, whether an alias happens to exist, or a guessed migration.

Adopt these exact closed kind values in the existing fields:

| Physical profile | Counter format/kind | Slot format/kind |
| --- | --- | --- |
| LegacyIron | 2 / native-counter-checkpoint | 1 / native-slot-checkpoint |
| RegisteredCube | 2 / native-registered-counter-checkpoint | 1 / native-registered-slot-checkpoint |

The existing session envelope remains format1, kind
horizon-native-session-checkpoint. Counter/slot kinds MUST select the same
profile. Unknown kinds, unknown format numbers and mixed profiles refuse before
native world opening or owner installation. There is no default-to-registered or
default-to-legacy branch for unknown input. Existing legacy kind semantics and
legacy positive/refusal cases remain unchanged.

This is an explicit FORMAT-PROFILE EXTENSION despite retaining field shapes and
format numbers. It is not a promise of migration, backward compatibility,
arbitrary-world support or a new native codec. An older reader refusing the new
kinds is expected. No image/record is rewritten from one profile to the other.
The existing CounterFrameV2 Rust type may retain its name for its version2 field
shape; document the new closed kind semantics without a repository-wide rename.

For BOTH profiles retain truthful binding.template=minecraft:iron_block and
empty binding.properties: that field identifies the captured source geometry,
not the registered physical name. binding.block_id remains the captured custom
alias. The observed descriptor remains the actual native physical state.

Validation uses the kind-selected expected bound descriptor:

- LegacyIron: name minecraft:iron_block, empty properties, as today.
- RegisteredCube: name binding.block_id, empty properties.
- Active requires exactly that expected descriptor and correlated positive
  allocation generations. Dormant requires a different resolved propertyless
  observed descriptor, no active counter/slot allocations and coherent retained
  heads/history. Registered dormancy after replacement by either air OR builtin
  iron must work; builtin iron cannot be mistaken for the registered bound cube.
- Pending retirement, mixed active/dormant ownership, nonresidency, Faulted and
  Detached remain refusals. Deliver/acknowledge the real pending owner and
  reconcile the slot before a dormant image can be captured.

Thus the profile survives complete images and standalone counter/slot frames
while dormant. This proposal does not defer that ambiguity to a later active-only
cell. Ordinary startup/activation will still need a separately accepted caller;
the serialized profile will already be explicit when that work is proposed.

## Proposed binding and host APIs

All names below are proposed bounded APIs, not existing accepted source changes.
In world/counter_restore.rs add a closed NativeBindingProfile enum with
LegacyIron and RegisteredCube. Store it privately in NativeBinding. Expose only
read-only profile/expected-physical-descriptor accessors needed for correlation;
the enum is data, not world or player authority.

```rust
NativeBinding::from_registered_prepared(
    prepared: &PreparedBundle,
    type_id: &str,
    registry: &'static BlockRegistry,
) -> Result<NativeBinding, RestoreError>;
NativeBinding::profile(&self) -> NativeBindingProfile;
NativeBinding::physical_descriptor(&self) -> &BlockState;
DecodedSlotFrame::binding_profile(&self) -> NativeBindingProfile;
```

Keep NativeBinding::from_prepared explicitly LegacyIron. Share only private
captured-declaration validation where useful. The registered factory must check:

1. Actual retained PreparedBundle identity, canonical primary-owner type/block
   IDs, schema1, exactly one selected matching block and no ambiguous owner/type.
2. Captured block template exactly minecraft:iron_block, state_plan absent and
   BlockProperties exactly default. No static finite-state declaration, hidden
   property default, shape/physics override or alternate template is admitted.
3. Exact physical descriptor {name: captured block.id, properties: {}}. Require
   default_state(alias), id(descriptor) and state(resolved ID) to agree; exactly
   one same-name state must exist. Refuse air, an iron numeric ID substituted for
   the alias, absent/foreign aliases and nonempty/extra registered properties.

The caller must supply the actual Game.pack.assets.registry loaded from the
captured materialization, not a synthetic map, another Game's numeric ID or a
record-supplied descriptor. This factory is a checked binding constructor, not
cryptographic attestation of arbitrary resource directories. Genuine materialized
assets and matching actual frozen registry inputs must establish the host seam
and qualification; matching strings alone do not establish that provenance.

Counter frame emission derives kind from the installed domain's actual binding;
slot Header retains the corresponding profile from its anchor. Decoding derives
profile only from the closed kind. Restore requires decoded profile equal the
supplied checked binding profile in addition to existing package/type/schema/
block/world/target/clock checks. Neither serialized kinds nor successful decode
construct the genuine host bootstrap grant.

In game/counter_control.rs add a dormant internal adapter:

```rust
Game::bootstrap_registered_host_counter(
    &mut self, type_id: &str, position: [i32; 3],
) -> Result<RuntimeCounterKey>;
```

It selects the actual captured catalog uniquely, constructs the registered
binding with this Game's registry and reuses the existing identified session's
virgin initial-World bootstrap grant and physical resident-target checks. Keep
bootstrap_host_counter legacy behavior unchanged. No command, guest callback,
automatic placement hook or ordinary reopen path calls the new method.

## Session correlation and dormant recovery ownership

game/levels/session_checkpoint.rs must select the named factory from the known
counter kind, then correlate the slot kind/profile, exact captured identity,
type, schema, source template, alias and observed physical descriptor. Preserve
all existing package/world/authority pins, owner/player/entity checks, native
fences, complete inventory hashes, monotonic epochs and publication semantics.

Legacy adapters remain active-only. Add explicit registered adapters within this
same scoped file, accepting the existing trusted request routing fields:

```rust
Game::checkpoint_registered_session(&mut self, archive, epoch, deadline)
    -> SResult<CheckpointReceipt>;
Game::prepare_registered_session_checkpoint(self: Box<Self>, RecoveryRequest)
    -> SResult<RegisteredNativeRecoveryParts>;
Game::recover_registered_session_checkpoint(self, RecoveryRequest, LanHost)
    -> SResult<(Self, RegisteredRecoveryReceipt)>;
Game::recover_managed_registered_session_checkpoint(
    self, &AuthorityNamespace, ManagedRecoveryRequest, LanHost,
) -> SResult<(Self, RegisteredRecoveryReceipt)>;
```

RegisteredRecoveryReceipt has epoch plus a closed state:
Active { counter: RuntimeCounterKey, slot: SlotKey } OR Dormant. No fabricated
keys, zero IDs, reused source keys or Some/None mixed allocation are allowed.
RegisteredNativeRecoveryParts owns the unpublished fresh Box<Game> and the real
verified image/resources until its same final correlation/participant/authority
gates pass. Its key observer returns the explicit active/dormant state.

Current RecoveryReceipt and NativeRecoveryParts require concrete active keys and
have callers outside this budget. Preserve those types/signatures and their
legacy adapters. A private concrete recovery core may share the existing actual
copy/open/stream/fence/checked-item preparation and final verification, returning
the checked paired allocation state to the two adapters. Do not introduce a
general recovery framework or change unscoped legacy/frozen-window callers.
If actual reuse cannot fit these paths without weakening ownership or inventing
readiness, stop for a bounded revision rather than expanding silently.

For registered images permit exactly paired Active OR paired Dormant. Active
requires the native target equal binding.state_id(). Dormant requires the native
target equal the frame's resolved observed descriptor AND differ from that bound
ID. Preserve resident target/player native coverage and D's complete item-column
union/arrival checks. Restore counter/slot None results only for verified paired
Dormant; their histories, independent heads and binding remain installed in the
fresh World. The registered adapter must not apply today's active-key ok_or
checks or erase the domain to get through them.

After recovery of Dormant, an actual write of the bound alias may re-admit the
counter through existing mutation hooks; explicit slot reconciliation then
returns a fresh empty slot. Heads advance by their existing rules, stale keys
refuse and retained dropped items remain owned. This is explicit qualification
of existing transitions at a fixed target, not ordinary placement discovery.

## Actual native palette and resource provenance

No save.rs or native palette codec edit is proposed. Current SaveTables::new
derives names/defaults from the actual registry; encode_with writes the alias's
real default name or descriptor. Native save loading is deliberately tolerant
and can fall back to default states or air. Before any trusted recovery open,
retain the existing strict all-record validate_column palette checks: a default
name must resolve in the actual registry and a compound descriptor must match
registry.id exactly. Then correlate the target's decoded physical state with the
selected binding/profile. Unknown aliases, alternate properties and iron/alias
substitution must refuse before SavedGenerator can normalize anything.

Use genuine captured content through the existing H03 route:
Packages::read -> retained PreparedBundle.content -> ContentMod ->
mod_content::resource_root -> actual load_resources/Assets/RenderAssets/
PackResources -> Game::new. Existing H09 materialized-table/frozen-input resource
tests supply a concrete example. Actual FrozenHostInputs/RegistrySnapshot data
can correlate supplied registry mappings; it is not admission or bootstrap
authority. Do not introduce a parallel materializer or a claimed numeric-ID API.

The existing transition/session Fixture caches vanilla Resources and is not a
registered-materialization positive. Use an owned materialized-resource fixture
which outlives its Games. Load it once before constructing its Games in an
isolated qualification process; do not change process environment or reuse the
base-resource OnceLock as proof of registration. A small explicit-resource Game
constructor helper may be extracted in the scoped session test file, retaining
the default legacy helper behavior.

load_resources can silently fall back to the vanilla root on overlay failures.
Require Loaded.root equal the intended materialized root, no pack_error/fallback,
the alias's exact default descriptor and distinct physical ID, actual collision/
occlusion table identity, and successful nonfallback cube model collection from
pack.render.model_catalog for that ID. Check inherited cube geometry and valid
texture/model references through the actual loaders. Headless renderer=None is
compatible with loaded CPU renderer assets; it proves no visible-frame result.

The fallback negative must exercise the real loader with a damaged owned overlay
and a valid base, then refuse registered binding before world/bootstrap ownership
is acquired. Never edit linked/shared base resources in place. Preserve/hash the
prebuilt pair and base inputs, use fresh owned caches/copies, and verify inputs
unchanged. No compiler, Java or test subprocess is spawned from a Rust test.

## Exact seven-path proposed maximum

Paths are under crates/horizon-client/src. This is a design budget only.

1. world/counter_restore.rs: binding profile/factory; closed counter kinds;
   active/dormant emission, validation and restore correlation; inline tests.
2. world/slot_restore.rs: private Header profile and read-only decoded accessor;
   closed slot kinds; matching active/dormant validation/emission/restore.
3. game/counter_control.rs: explicit registered host-bootstrap adapter only.
4. game/levels/session_checkpoint.rs: profile/physical correlation, registered
   active+dormant adapters/receipts, native pre-open checks and shared concrete
   owned recovery wiring while legacy APIs remain unchanged.
5. world/slot_restore_tests.rs: strict registered frame/profile/dormancy controls
   and legacy behavior preservation.
6. game/levels/session_checkpoint_tests.rs: minimum explicit-resource/helper
   extraction and child-module declaration; default legacy cases unchanged.
7. NEW game/levels/session_checkpoint_registered_tests.rs: actual materialized
   Game/resource/retirement/native-save/restore/recapture and refusal controls.

No changes to mod_content.rs, save.rs, entity_nbt.rs, game.rs, World mutation
hooks, item codecs, networking, D's item/portal/streaming helpers or ordinary
placement/lifecycle callers are proposed. Before source authority, enumerate all
NativeBinding/Header/frame/view/receipt literals and callers at the combined pin.
Any actual eighth-path dependency requires a revised exact bounded design and
fresh review. Do not use casts, fake IDs, fabricated capabilities or silently
changed legacy receipt semantics to evade this budget.

## Defining evidence and refusals

Root alone executes isolated serial qualification after fresh independent exact
SOURCE CLEAR. Include the following concrete tests, with exact source/tool/input
pins and genuine resources; absent resources or skipped positives do not pass.

- Materialize two independent captures of the genuine prebuilt schema1 package.
  Verify actual alias/default/reverse/collision/model mappings, unchanged vanilla
  IDs and matching actual frozen registry inputs. Neither handcrafted registry
  JSON nor asserted symbolic names establish the positive.
- Bootstrap a real propertyless registered target under genuine integrated-owner
  and dedicated constructor roles, with the existing two real participants.
  Capture through the actual native palette/entity writers and fences. Inspect
  the new counter/slot kinds, truthful iron source template, real alias palette,
  actual physical target, independent generation heads, clocks and UUID pins.
  Restore fresh, recapture and restore again. Preserve existing legacy cases.
- Remove the real registered target while it holds a rich durable nested stack.
  Use D's genuine checked delivery, keep its original ItemEntity, reconcile while
  the target is air and capture a healthy paired Dormant registered image. Repeat
  dormancy with builtin iron as the nonbound replacement. Recover Dormant with
  no invented keys; recapture Dormant; write the genuine alias, reconcile and
  capture/recover Active again with fresh keys and advancing heads. Count all
  player/slot/drop/nested owners and preserve dropped UUID/component projections
  throughout. Pending payload cannot be deleted to obtain Dormant.
- Wrong type/owner/package/digest/schema, missing alias, wrong actual registry,
  nondefault/extra properties, extra same-name states, non-iron source template,
  physical iron substituted for alias and mismatched actual numeric mappings must
  refuse without bootstrap spend, owner mutation or new accepted image.
- Exercise the actual loader fallback negative and failures in collision/model
  resource identity. Failed/fallback materialization must not create a registered
  Game authority by borrowing vanilla assets or a fabricated alias ID.
- Decode/reseal counter/slot unknown kinds/formats, mixed profiles, contradictory
  active/dormant descriptors, stale generations, missing/wrong native target
  palettes, malformed defaults and wrong packages. Refuse before tolerant native
  opening; retain previous accepted epochs and original live owners.
- Verify both profiles' real binding factories remain distinct even with both
  physical names present in the same actual materialized registry. Registered
  frames cannot restore with a legacy binding or vice versa. Dormant air does
  not select a profile; the closed kind does.
- Reuse D's checked dropped-item profile, native entity hashes/fences, writer and
  Copied/Promoted failure/retry semantics, coherent players and all-item-column
  arrival requirements. No codec/profile loosening, fabricated owner token or
  omission of delivered items is allowed for this positive.
- Run required focused legacy counter/slot/bootstrap/session/retirement/D groups,
  applicable feature/format/lint/native save/materialization/frozen-registry gates,
  and the accepted broad checks. Keep actual normal-network/Running/visible-frame
  claims separate; those later S1 obligations remain open.

Completion requires exact clean local source/tool pins, successful defining
evidence, a different independent FINAL review and a private durable result
checkpoint. Ordinary placement/departure/close/Running, authorized player actions,
full admission/replication and visible committed animation remain S1 work. The
closed registered profile removes the persistence-mode ambiguity without claiming
that those callers or G1 already exist.
