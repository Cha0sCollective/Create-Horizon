# Committed generic machine image and replica proposal

Status: PROPOSED DESIGN ONLY. Exactly five engine paths are proposed below. This
is a dormant, in-process NATIVE persistent observation/reducer packet, not transport, configuration
admission, ordinary gameplay activation, retained animation, G1 or publication.
No source authority follows from this document.

## Prerequisites and actual baseline

Research inspected clean LOCAL1fa55c619a2f4666a505461822a97efa5732db01, tree
d0b2c224, in .local/worktrees/s1-login-checkpoint-combined. Its combined broader
qualification/independent FINAL remains pending at authoring time. Registered
binding proposal3ec3363a has conditional design acceptance and draft source only;
its final runtime/API acceptance does not yet exist. No future combined or
registered FINAL pin is fabricated here.

SOURCE START requires: accepted exact clean combined FINAL; separately accepted
registered binding FINAL on the reviewed lineage; a durable private checkpoint;
fresh independently reviewed actual APIs/resource profiles at that resulting
pin; and fresh independent Sol High acceptance of this exact proposal. Re-resolve
all source anchors. A change of parent or actual API requires a scoped amendment
and renewed review before authoring. Root alone owns runtime, formatting and Git.

C1 actual full configuration/admission and its held allocation liveness gate are
UNMET prerequisites for subsequent endpoint activation. They are NOT emulated by
this packet. Genuine in-process Game observation can be qualified independently
once the preceding source-start prerequisites hold.

## Closed outcome and exact five-path budget

All paths are relative to crates/horizon-client/src:

1. lib.rs: feature-gated export of machine_replication only.
2. NEW machine_replication.rs: native image data, bounded validation/exact native
   view comparison, checked borrowed World/Trial route before any payload clone,
   dormant extractor state, typed reducer and focused inline tests.
3. game.rs: module declaration ONLY for game/machine_replication.rs. No fields,
   constructor changes, tick hook, activation, UI, transport or shutdown edits.
4. NEW game/machine_replication.rs: dormant genuine Game extraction adapter using
   actual checked World APIs; owns no Game and supplies no fabricated authority.
5. NEW game/machine_replication_tests.rs: actual Game/materialized-resource tests;
   declared as the child test module of game/machine_replication.rs.

No other module, existing test helper, World/core/native item type, runtime/SDK,
checkpoint, observer queue or transport path may change. Inline tests remain in
path2; all Game tests are path5. If real resource construction or fresh APIs cannot
be used within these paths, stop and request a precise amended budget. Do not
claim synthetic-only success or widen helper visibility outside this budget.

Exactly one NATIVE persistent captured registered propertyless counter, its one owned slot and one
target per World. No general replication framework, portable inventory codec,
arbitrary machine store, Java-client compatibility or new protocol packet ID.
Defaults and existing dormant counter/slot callers retain their current behavior.

## Existing checked observations and trust boundary

The actual baseline has World, not a NativeWorld type. NativeWorld in the close
proposal is a storage profile, not a source constructor. There is no Actor.slot
API here. Preserve the actual H06 player-cell read-set and callback-abort rules;
this observer must not replace them with a client-view authority.

Existing source seams:

- world/counter_restore.rs490-520: PlacementKey and opaque RuntimeCounterKey.
  persistent_counter_key()908 and read_persistent_counter()874 validate actual
  session allocation, active placement, binding, instance, native state and
  residency. session_nonce() is observation only, not key construction authority.
- counter_trial.rs: Snapshot::{configuration,record,last_retirement,fault} and
  Record::{instance,counter,last_advanced,resident} observe committed host state.
  Guest Proposal and PhaseToken are not replication inputs.
- world_slot_trial.rs439-496: public world_slot_snapshot()/read_world_slot()
  CLONE the native payload. They are NOT the pre-budget extraction route.
  The new LIB module has existing crate access to World.world_slot_trial,
  Trial::borrowed(), AllocationView::{key,revision,stack,resident} and
  World::persistent_slot_guard(). These borrow the original native payload.
  SlotKey retains private allocation identity; incarnation alone is not authority.
- world/slot_restore.rs914/984: persistent slot metadata/guard correlate active
  machine generation, independent slot generation, scope, profile, native target,
  pending ownership, status and residency. Pending is not empty or dormant.
- game/counter_control.rs209-318 has separate Native and Guest phase dispatch.
  The persistent domain is NATIVE ONLY: NativeBinding::configuration constructs
  Backend::Native, Trial::restore_native rejects Guest, and
  Domain::configuration_matches requires Backend::Native (counter_restore591).
  Registered binding3ec preserves this restriction. The transient Guest trial
  does not establish a persistent registered Guest machine. Do not mix them.

The Game adapter takes a synchronous immutable Game borrow and reads these actual
objects itself. It accepts no caller-supplied counter value, stack, scope, registry
ID, claimed key/session, profile, successful callback Boolean or admitted actor.
It refuses remote/window replicas (remote or integrated client ownership), wrong
execution role, closing/faulted/detached/pending states and mismatched dimension,
physical registered binding or package/type/schema provenance. It correlates the
actual retained package/catalog and materialized registry, not request strings.
Fresh exact APIs must establish these checks before source start; unsupported
opaque-owner access is a blocker rather than permission for a fake constructor.

The new LIB module performs a checked BORROWED route. First inspect actual counter
configuration/backend and persistent domain/key/record, plus scalar slot metadata;
require persistent RegisteredCube/Backend::Native. Call persistent_slot_guard()
and obtain world.world_slot_trial.as_deref().borrowed(). Require its actual
active AllocationView, no pending/fault, current residency, positive opaque-key
incarnation and metadata-correlated machine/slot generation, same actual
configuration scope/position/type/schema/backend and current registered native
physical state. The guard already validates original key identity and persistent
anchor; retain that validation instead of reconstructing a key from view scalars.
The Game adapter derives/passes NativeBinding from its actual retained captured
package and registered registry factory, then correlates profile/resources;
the lib route rechecks the actual native context, not a client assertion.

Walk the ORIGINAL borrowed AllocationView.stack and all nested data with checked
budgets BEFORE world_slot_snapshot(), read_world_slot(), SlotSnapshot::clone(),
ItemStack::clone() or any other owned payload snapshot. Public getters that clone
must never run first, even for preliminary context/status. No borrowed TrialView
or AllocationView escapes the synchronous World borrow; Game bin code need not
and cannot access the crate-private slot internals directly. The lib helper
returns only its fully validated bounded cloned image/refusal to the Game adapter.

After all header/native-payload budget and identity checks succeed, clone the
borrowed native stack ONCE for the candidate view. Recheck persistent_slot_guard,
actual opaque key/read version/configuration and checked counter read before
observer publication while the same immutable borrow is held. No unbounded public
snapshot/read is needed even after this check. This supplies a stable committed
read-set, never a detached cross-tick transfer plan. Copies of runtime keys may be
retained privately by extractor state solely to detect replacement; they are never
public view fields or accepted from the reducer. If a fresh API changes crate
visibility/guard behavior, stop for a scoped amendment rather than cloning first.

## Proposed dormant API and native image

Names are proposals to be rechecked, not statements of existing API:

```rust
// lib module: explicit caller-owned observer, not installed in Game.
CommittedImageExtractor::new() -> Self;
CommittedImageExtractor::current(&self) -> Option<&CommittedMachineImage>;
CommittedImageExtractor::reset_observation(&mut self); // no world mutation
// LIB route performs original borrowed payload bounds before any stack clone.
CommittedImageExtractor::observe_native_world(
    &mut self, world: &World, binding: &NativeBinding,
) -> Result<Observation, ObservationError>;

// game module: actual authority checks and synchronous native extraction.
Game::observe_committed_machine(
    &self, extractor: &mut CommittedImageExtractor,
) -> Result<Observation, ObservationError>;

// Observation = Unchanged | Published(&image), expressed with suitable lifetimes.
// Failures return typed unavailable/refusal, never a fabricated empty image.
Replica::apply(&mut self, event: ReplicaEvent) -> Result<ApplyOutcome, ReplicaError>;
```

Extractor state is caller-owned and bounded to one current image plus its exact
private authority correlation. No new Game field or ordinary call site is added.
Reset drops only cloned views and observation correlations; it cannot retire the
machine, restore inventory, acknowledge custody or admit an endpoint.

An immutable CommittedMachineImage contains closed observation schema1, binding
profile RegisteredCube, captured package identity/type and save schema1, WorldId,
dimension, position, positive machine generation, positive slot generation,
checked native counter instance observation, actual counter value/last-advanced,
actual DomainStatus Healthy, no trial fault, resident=true, slot revision, native
Option<ItemStack> view, sampled server simulation tick, and checked replication
revision. An unavailable/fault/pending result is a separate typed observation
failure, not a successful image with an empty inventory. The prior image remains
an historical committed view; callers must not present a failed new observation
as current readiness or manufacture a new fault-state commit from a failed guest.

Only AFTER the checked lib borrowed route and all budgets pass, the native
ItemStack is cloned as a view, including complete extra data. It is
not moved from a cell, installed as a SlotProjection, used for player transfer or
written back. No serializer, save-frame encoding, item_sync conversion, network
registry numbering, wire digest or claimed lossless presentation codec is added.

Proposed finite data bounds for this closed in-process packet:

- schema exactly1; package identity exactly32bytes; WorldId exactly16bytes.
- dimension/type/physical descriptor identifiers each at most256 UTF-8 bytes,
  nonempty with actual native identity validation; position obeys native height.
- all generations positive, revisions checked u64; tick nonnegative; no wrapping.
- one image per extractor and one replica record plus one tombstone floor per
  reducer; one outstanding resync request; no unbounded history/duplicate ledger.
- before cloning/comparing native payload: maximum depth16,4096 aggregate native
  nodes/stack entries/NBT nodes, and1MiB conservative owned-data budget using
  checked length arithmetic. Count native strings/UTF-16/arrays/collections and
  nesting across container, charged, bundle and pot-decorations ownership. Refuse
  unsupported/exhausted observations before publishing or advancing revision.

These are proposal bounds, not a portable wire format or revision of host item
semantics. Use exhaustive native field matching so new native variants/fields
require an explicit source decision, not silent omission. No process-global
allocator changes or recursive unbounded pre-clone inspection.

## Exact native payload comparison

ItemStack/StackData PartialEq is insufficient as full view proof. In the inspected
pin, BundleSelected::eq always returns true, including nested stacks. Preserve
actual bundle_selected.0 at every container/charged/bundle/pot nesting level.
Use a private bounded exact-native comparison: native field comparison PLUS
explicit recursive comparison of those runtime-selection fields, exhaustively
matching the current native structures. NBT Tag retains float/double bit patterns
and UTF-16 JavaString; preserve both, raw other components, ordering, optional
representation and all native modeled fields. Do not normalize through save NBT,
Debug text, JSON, item_sync, gameplay equality or a guessed component whitelist.
Fresh API review must check for additional Eq omissions at the accepted pin.

Full/duplicate comparison and before/after assertions use this exact comparator.
Tests must establish a pair that native ItemStack equality calls equal but exact
view comparison distinguishes, both top-level and nested. The view remains an
exact native observation even though a later network presentation may deliberately
omit runtime-only fields; that later contract is not selected here.

## Phase, revision and failure semantics

This packet does not hook an ordinary phase. In defining Game tests, run the real
existing counter phase and any actual synchronous slot transfer, then invoke the
observer. The immutable Game borrow can see only published host values, never a
guest Proposal or staged transfer output. This packet supports only actual Native
persistent phases; transient Native/Guest trials are refusal controls. No caller-created PhaseComplete token,
Boolean latch or diagnostics message proves phase completion.

A later caller must sample at one explicitly defined per-dimension post-commit
boundary. Current game.rs5442 tick_lan precedes tick_counter_trial5483; therefore
Hub::tick_level is not that boundary. Ordinary tail/paused wiring requires a
separate packet including the actual affected paths. Here no Hub publication is
added, early or otherwise. Defined coalescing means only the final committed
counter+slot state of that phase is sampled; it does not promise one delta for
every intermediate operation or reconstruct transient transactions from logs.

First successful active observation allocates replication revision1. Subsequent
successful observations of a changed exact image (including a later sampled
simulation tick) checked-increment the replication revision. Repeated sampling
of the exact same state/tick returns Unchanged without allocating a revision.
Counter value, World.revision (block edits), server tick and slot revision are
independent fields; none alone serves as the combined replication revision.

Complete all native read-set/provenance/budget/exact-comparison checks and reserve
the new view before replacing extractor.current or incrementing its revision.
Failure preserves the original image/revision/correlations. Actual Native phase
failure (for example real counter exhaustion) must return unavailable/refuse before
any new image/revision; prior committed fields and exact native slot payload remain
intact. Transient/Guest extraction refuses, including after a real transient Guest
abort; this is NOT persistent Guest callback-abort qualification. Dropped staged
transfer plans likewise publish no changed view. Revision exhaustion refuses
without changing either native Game state or observer state.

No newtick/no counter advancement is inferred while paused. Resending an existing
image and servicing pure reducer resync are possible without machine advancement.
A successful new observation can be requested after the real paused boundary;
this packet proves only explicit dormant calls, not automatic paused scheduling.
On native retirement/nonresidency/pending/fault the extraction refuses active
publication and preserves its last committed image. A caller must explicitly
retire the view through the data reducer. A new genuine allocation/generation is
a replacement, never an in-place continuation inferred from equal value/position.

## Typed replica reducer: data only, no endpoint authority

ReplicaEvent is a closed in-process enum, not decoded wire input. Proposed events:
BeginSession, Full, Delta, Untrack, ExpectResync, EndSession. Fixed header fields
include observational session epoch, subscription epoch, world/dimension/position,
positive machine+slot generations, image/base/next revision and finite request ID.
Constructing these values is allowed in pure data tests and conveys NO admission,
endpoint liveness, mutation, registry-install or machine capability. Do not name
a public data constructor admitted/authorized or expose a fake Guest constructor.

- BeginSession explicitly replaces reducer data context, clears prior client view
  and animation association, and installs a caller-supplied observational epoch.
  Equal IDs do not establish actual allocation identity. Later C1 integration must
  issue/validate the epoch from its genuine endpoint owner before calling this.
- First Full creates the single view only for the current session/dimension and a
  strictly newer subscription epoch than its retired floor. It has base absent,
  positive revision and coherent bounded image/header fields. All validation and
  reserve work precedes reducer publication.
- Delta must name the exact active session/subscription/world/dimension/position/
  machine+slot generations. Require base=current and next=checked(base+1), with
  coherent full resulting native image in this bounded first packet. This is a
  typed versioned replacement delta, not a general field-patch language.
- A byte/field-exact duplicate of the current accepted Full/Delta has no effect.
  A different payload under the same current revision is a conflict/refusal.
  An older revision cannot mutate the current view. No claim of remembering all
  historical payloads is made; old revisions are stale, not verified duplicates.
- Missing base/gap retains the last accepted view and returns NeedResync. At most
  one ExpectResync request is retained, naming current subscription, generation,
  current base and checked request sequence. It cannot change image authority.
- A resync Full may jump revisions only when correlated to that exact outstanding
  request/context and not older than the retained current image. It spends the
  request only on successful publication. Unsolicited or stale Full cannot roll
  back a current view; same revision requires exact equality. Resync generation
  change requires explicit replacement/untrack and a fresh subscription.
- Untrack retires the active instance without touching server state and records
  that subscription epoch as a monotonic tombstone floor. Repeated matching
  Untrack is harmless; old-context events cannot retire a newer view. A late Full
  at/below the floor cannot resurrect it. Do not clear the floor to recover a gap.
- Replacement requires retired old subscription and strictly newer subscription;
  changed actual machine/slot generation receives a Full. Same-position equality
  never substitutes for generation. Exhausted epoch/request/revision fails closed.
- EndSession drops view/resync data and blocks later old-session events. Reconnect
  uses a fresh session data context supplied by future genuine C1 owner; equal
  profile/name/address or package identity cannot reactivate the prior instance.

No network sender, GPU handle, timer, executor or ordinary player intent is called
by this reducer. Future renderer integration owns actual handle retirement.

## Genuine Game tests and focused data tests

Path5 must use an actual headless Game built with owned, materialized registered
resources. The existing transition_ownership_tests::Fixture caches vanilla
Resources and is not a registered positive. Either reuse a genuinely accessible
explicit-resource constructor accepted in registered FINAL, or build the narrow
owned helper within path5. If helper access requires another file, stop for an
amendment. No fake Game/machine key factory, fabricated configured Guest or
synthetic liveness Boolean qualifies actual Game extraction.

Materialization evidence: retained captured PreparedBundle/content through real
resource_root/load_resources/Assets/RenderAssets/PackResources; Loaded.root exact
intended root; no pack_error/fallback; registered descriptor and distinct alias
ID; actual collision/occlusion tables and nonfallback cube model collection. Keep
that fixture alive across its Games; load once per isolated qualification process
without global environment mutation or base-resource OnceLock contamination.

Each defining family runs in its own explicit pinned resource/package/profile
process when native resource statics require it. Root records exact source/API
pin and actual test inventory/census, feature/profile and resource identity before
and after; no suite-count claim from a different binary, LegacyIron profile or
reused global fixture. Resource-dependent cases may be ignored by default but
are mandatory explicit qualifications, not silently skipped. No compiler, Java,
Python or nested Rust test subprocess from Rust tests.

Required actual Game cases:

1. Registered active counter+slot genuine initial image: positive independent
   generations, actual checked keys, current native alias, exact native payload,
   tick/value/slot revision and observer revision1. Second identical read unchanged.
2. Real persistent NATIVE successful phases; sample only after commit; exact
   value/last-advanced/tick and checked new combined revision. Multiple real Native
   counter/slot operations in the declared phase coalesce to its final image.
3. Real available Native failure: restore/initialize a genuine accepted native
   counter at its real exhaustion boundary using supported actual frame APIs,
   invoke the actual Native phase, observe its real fault, and preserve previous
   image/revision/payload. Real state mismatch/stale read/retirement and failed
   slot-transfer read-set supply additional failures. Actual transient Native and
   actual transient Guest trials are extraction REFUSAL negatives only; no keys,
   backend rewrite, forged persistent state or diagnostic replay turns them into
   a persistent Guest positive. Persistent Guest trap/fuel/rejection controls
   belong to the mandatory separately accepted bridge packet described below.
4. Actual synchronous player-cell transfer through existing Game service followed
   by observation; preserve exact all-owner accounting and per-cell read-set.
   Dropped prepared plan/real stale-plan refusal has no changed view. No ordinary
   player authorization claim or weakening of the intended held C1 lease.
5. Wrong scope/dimension/package/profile/schema/physical state; real nonresident
   target/chunk replacement/unload; counter fault/detach; pending slot retirement;
   active/dormant mismatch. Refuse before image/revision replacement and preserve
   all retained native payload/custody. Dormant is not fabricated active emptiness.
6. Actual retirement/delivery/reconcile and fresh generation/recovery where actual
   registered APIs support it: old runtime keys fail; equal-value restored/new
   placement has fresh native authority; old view cannot become new authority.
7. Rich native payload with custom data, UTF-16/float-bit NBT, components and
   nested bundle/container/charged/pot values within supported native profile;
   bundle-selected differences retained despite native Eq, original cells intact.
   Build genuine original over-depth/over-node/over-byte native payloads by moves
   and verify rejection through the lib BORROWED route before any payload clone,
   public snapshot/read or publication. Include pending/nonresident originals
   and exact boundary positives. Focused test-only clone-site accounting in path2
   may corroborate ordering; independent source review must verify the actual
   pre-clone path rather than relying on a claimed counter. Assert exact original
   representation/ownership and unchanged observer state, not gameplay Eq alone.
8. Explicit real paused/frozen/no-ticket phases: no counter catch-up; identical
   observation stable; no automatic wire or phase-scheduling claim. Remote replica
   Game refuses as actual authority. Legacy/default behavior remains unchanged.

Path2 pure tests cover finite headers/schema/budgets; initial Full; matching Delta;
exact current duplicate versus same-revision conflict; stale revision; gap/base
mismatch retaining old view; exactly correlated full resync; stale/wrong request;
untrack epoch floor and late resurrection; replacement generations; session end/
reconnect data isolation; checked revision/subscription/request exhaustion; and
native Eq-omission payload conflict. They establish reducer behavior only, not
real transport/session authority or two-client evidence.

## Mandatory persistent Guest bridge and later callback qualification

The existing persistent domain and registered3ec binding are Native-only. This
five-path component cannot honestly test a persistent registered Guest callback
or its trap/fuel/invalid-output abort. It does not alter that backend contract,
restore mode, native phase dispatch or runtime ABI. Transient Guest positives
elsewhere do not fill this gap and may not be presented as H10 persistent proof.

A separately accepted, bounded S1 packet MUST bridge the real captured Guest
computation into the genuine persistent registered counter+slot host lifecycle,
including exact runtime/binding/schema/read-set correlation, actual successful
commit, trap/fuel/invalid-output abort retaining previous committed host state,
guest cache reset/recreation and no speculative replica image/revision. It needs
fresh source/API budget and genuine persistent Game tests, then composition with
this observer and later network activation. No fabricated keys, Native-to-Guest
metadata flip or diagnostic receipt replay can substitute. This deferral is a
component split, NOT a reduction of S1/G1 or permission to enter S2 early.

## Deferred transport and hard dependencies

A later bounded packet must independently qualify actual native item presentation.
Existing Hub::stack/guest::stack_to_network and item_sync::from_network are reuse
candidates, not a lossless guarantee: item-ID fallback0, filtered registry entries
and unsupported component handling require real controls. Select a concrete
supported presentation subset or explicit unavailable result; never strip data
silently, alter server inventory or introduce a general portable inventory codec.

That packet must use accepted C1 configured endpoint ownership and held terminal
liveness lease, same actual Session/Lan allocation and guarded TCP/Pipe I/O. No
public observational epoch in this packet substitutes for those services. Actual
terminal-before-publication refuses; terminal-after-publication invalidates that
endpoint and retires its view with retained departure custody where applicable.
Fresh actual API/budget acceptance is required; C1 admission/close foundations
remain unmet here.

Existing transport seams: CustomPayload envelope already in protocol play; net
forwards non-brand payload through Event::Packet; Guest::send/PlayerSender supply
native order; Hub::here and sent_chunks give concrete tracking evidence. Later
source must attach full image AFTER actual chunk tracking, deltas AFTER per-level
commit, untrack at native forget/respawn/dimension/terminal/reload boundaries, and
bound resync requests. game.rs ordinary tail and levels.rs paused path are outside
this five-path service packet. No Java-client compatibility follows.

Later required real tests: default integrated owner over MemoryPipe AND second
TCP client; dedicated two TCP clients; matching real configuration admission;
late join/current full, successful Native commits and actual Native failures and the separately
bridged real persistent Guest callbacks, independent distance
tracking, duplicate/gap/full-resync, replacement/unload/retrack/dimension changes,
reconnect with delayed old allocation events, held actual close-versus-publication,
resource reload/handle retirement, and dedicated renderer allocation zero. In-frame
comparison and retained animation have separately scoped callers/evidence.

## Acceptance and stop condition

Independent exact SOURCE review precedes all compilation/runtime; a different
FINAL reviewer accepts the actual bounded results and limitations. Root performs
appropriate feature-off/paired/no-runtime/full checks, genuine resource-dependent
Game families and actual pure reducer inventory with isolated data. No unrelated
source/fmt corrections or expanded helper/transport work without amended scope.

Stop this packet after its dormant five-path component FINAL and truthful result
summary. It supplies committed NATIVE persistent observations and data-reducer behavior,
not ordinary H10 activation, connected clients, admitted actor authority, retained
presentation, complete S1 or G1. Continue toward later S1 packets under the owner's
existing instruction only through separately concrete reviewed source authority.



