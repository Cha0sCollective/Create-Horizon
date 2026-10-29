# H08 native participant roster and player-owner preflight

Accepted bounded design after a different fresh independent read-only Sol High
review with no blocking findings. Reviewed draft SHA256:
3415460d240f070c8185a2444d254afc486a12920c392e45c517ecf1d5d31707.
Accepted engine base is H06d
62756231205b1b2b65fcf92c180fc29497f29715, tree
939047a8e540e178b9176830fdea8d8c64e0b8f4. H08a14 checked raid saving is currently
accepted on codex/h08-checked-raid-save at local
b3e0d3561a8849c03cd19012ed42c4710b7ec0fe, tree
822c230744135df8a63195a5b0bbcdcc029f8c31 after native, new both-mode Game, broad
and final independent receipt checks. Use that frozen successor for implementation.

## One bounded objective

Implement a dormant, synchronous native expected-roster comparison and strict
player-owner preflight for authoritative server Games. It observes every hub guest,
including active external guests, rather than relying on store_owner's owner-only
save coverage. It performs no projection, filesystem operation, network polling,
save, gameplay tick, normalization, ownership take, callback or publication.

This is deliberately the smallest useful player prerequisite. It returns Result<(),
ParticipantPreflightError>, not a capture permit, checkpoint plan or readiness token.
A successful return means only that the named current player population satisfied
these player checks during this call. It is not whole-World readiness, a persistent
barrier, proof of earlier retirement success, or a completed checkpoint. No enabling
callsite, SDK/API, command, manifest, storage schema or new Game field is introduced.
The later capture method must call it inside the SAME exclusive owner operation
that takes all projections and completes the relevant producer/copy work.

This temporary server-only research boundary is not a permanent restricted S1
scope. In-frame native-local-player capture and the server plus two external paired
client experiment remain requirements of the later applicable qualification.

## Concrete source seams and production scope

Keep all observation types and the main methods in game/lan.rs, adjacent to its
existing record_uuid and player-save helpers. That module already sees LanHost,
Owner, native Guest, Game and the canonical owner/external UUID rule. Do not widen
record_uuid, records_uuid, LanHost fields or any library/network type.

Proposed paired-packages-gated, crate-internal types:

- ParticipantRoster: private host_identity: Arc<horizon_client::lan::Shared>,
  owner_uuid: Option<u128>, level_player_uuid: Option<[i32; 4]>, and a private
  Vec<ParticipantIdentity>.
- ParticipantIdentity: conn: u32, uuid: u128, canonical dimension string, native
  sender_identity: Arc<AtomicUsize> cloned from Guest.sender.pending, and storage:
  ParticipantStorage.
- ParticipantStorage: IntegratedOwner { record_uuid: [i32; 4] } or
  External { record_uuid: [i32; 4] }. This is an in-memory mapping observation,
  not a persisted format or a routing command supplied by a guest.
- ParticipantPreflightError: finite diagnostic variants distinguishing authority,
  missing host/owner/mapping, invalid/duplicate identities or record targets,
  roster mismatch, and unsupported player state. Include conn where meaningful;
  do not turn arbitrary debug strings into an external protocol.

Proposed Game methods, visible only within crate::game:

- observe_checkpoint_participants(&self) -> Result<ParticipantRoster,
  ParticipantPreflightError>: capture the native host/owner/connection mapping,
  reject invalid or duplicate mapping and unplaced participants. No automatic
  refresh of an existing expectation in preflight.
- preflight_checkpoint_participants(&self, expected: &ParticipantRoster) ->
  Result<(), ParticipantPreflightError>: compare against that expectation and
  inspect current player-owned state without changing it.

The expectation is captured by a native diagnostic caller before its controlled
workload; a later caller retains it unchanged. It is neither guest-authored intent
nor proof that the baseline population was historically complete.

Exactly two narrow borrowed predicates are needed for private fields:

- game/lan_crafting.rs: GuestCrafting::checkpoint_menus_absent(&self) -> bool,
  true only when inventory, table and machine are all None. Even an empty cached
  menu is conservatively refused; do not clear it to make preflight pass.
- game/lan_actions.rs: GuestPlay::checkpoint_participant_idle(&self) -> bool,
  checking its private activity fields plus the already visible ownership fields
  below, delegating only to the concrete GuestCrafting predicate.

Keep all predicate fields private. No changes to lan_stations, lan_trading or
lan_mount_menu are required: reject their Option owners when present instead of
opening/serializing their private payloads. No generic owner visitor or trait.

Production changes are restricted to game/lan.rs, game/lan_actions.rs and
game/lan_crafting.rs. A child game/lan/participant_preflight_tests.rs and its gated
test declaration in lan.rs hold actual-Game tests. Reuse the accepted H08a14
TEST-ONLY Fixture/new/game visibility; preserve every existing fixture body,
loader, explicit Options literal and qualified test. Local predicate tests in
the defining modules may directly exercise their private fields.

## Exact native identity and mapping comparison

Require remote.is_none(), integrated.is_none(), and server execution role. Reject
an acting guest turn (lan_acting.is_some()) and absence of self.lan. Inspect the
whole current host population, not a selected subset. Do not assume vector order
is identity: compare by conn and require exact population equality.

Validate native dimension names exactly against the three supported dimensions;
never use Dim::from_id's fallback to accept an invalid name. Reject duplicate conn,
duplicate UUID and duplicate canonical player-record target, both in the expected
observation and current population. Native inventory length must be 36, ender
inventory length 27 and selected hotbar index less than 9.

Pin host identity with Arc::ptr_eq against Lan.shared, owner presence/UUID, and
level.player_uuid. For an integrated owner require exactly one placed matching
guest, an existing canonical level.player_uuid and Owner.saved == None; a retained
pre-admission Owner.saved projection is unsupported. If serving is present, its
native owner UUID must agree. For a dedicated host with no integrated Owner, every
guest maps to its own playerdata::uuid_ints(uuid). Reuse record_uuid directly;
never allocate or repair a missing UUID, call ensure_player_uuid, or substitute
an account UUID for the integrated owner's canonical player-file UUID.

Compare conn, UUID, exact dimension, storage variant and canonical record UUID,
and Arc::ptr_eq for the sender marker. Production lan/conn.rs::play creates a
fresh Arc<AtomicUsize> for pending bytes for each connection, and cloned
PlayerSender values share it. Compare allocation identity ONLY; network threads
may change its numeric count. Observation holds no item owner or mutable lease.

This distinguishes host replacement and ordinary fresh connection reconstruction
even if tuple values repeat. It does NOT distinguish a deliberately reconstructed
Guest that reuses the same sender marker and all tuple values. Shared::next_id uses
AtomicU32::fetch_add and is not globally unique across wrapping/restarts. Do not
claim arbitrary replacement detection, authentication, reach, replay protection,
a persistent incarnation ledger or network-session qualification. No new native
identity counter is proposed.

## Strict player-owned state refusal

For every guest, require placed and loaded, a current native motion body, no
left_reason, no dead/death-time state, and no pending teleport acknowledgement.
Reject a carried cursor, any craft_grid value or craft_result, open Guest.menu,
unfinished Guest.drag, vehicle association, sleep/bed transition, loaded_body
awaiting application, or a nonempty already-admitted Guest.inbox. Do not remove,
return, drop, sanitize or normalize any of these owners. In particular disconnected
cursor behavior is NOT the connected menu's return-to-inventory policy.

Require exactly one existing GuestPlay per current conn; reject any orphan play
record. Do not create defaults while inspecting. Its concrete predicate refuses:
mining, using, digging, delayed destruction, sign editing, merchant, station,
mount_window, lectern, hook, any GuestCrafting menu, left_game, won_game,
respawn_requested, or PortalState.process; require zero portal cooldown for this
initial conservative idle subset. Do not inspect merchant payments or manufacture
computed results. Numeric notification/cooldown bookkeeping that owns no omitted
item value need not be reset; no stats/advancement events or dirty sets are consumed.

Reject self.server_trips or self.level_teleports being nonempty and pending_save,
pending_entry, world_loader or rest_loader being present, or saved_for_exit. These
observable transition/shutdown states cannot be passed by choosing a smaller roster.
Reject self.player being Some and any server-local inventory/equipment/cursor/
crafting/scratch-slot/ender-inventory value or open server-local container/station/ender view.
Server-local ownership is outside this packet; do not mistake a headless fixture's
synthetic local player for the integrated owner's real hub guest.

These checks cover the observable PLAYER ownership subset. Orphan play/left_game
state is refused, but no current structure proves that every earlier failed Left
save was retained. Missing or changed expected participants refuse. A participant
lost before observation, a queued transport Left not yet applied to Hub, or an
indistinguishable reconstruction is NOT proven absent by this method. Do not call
Lan::poll, drain inboxes or handle a disconnect as part of observing readiness.

## Deferred global checks are required, not implicit passes

No code in this packet certifies global pending retirements, entity ownership,
parked levels, chunk residency, producer completion, native stack serialization,
World-slot teardown, disk success or durable recovery. Their absence cannot be
inferred from a matching idle roster. In particular an off-resident dropped entity
or an active/pending World slot can coexist with a successful PLAYER preflight;
that result MUST NOT be exposed as checkpoint-ready or used to start publication.

A later full-capture preflight must strictly refuse or include all such owners.
The inspected private boundaries explain why they are separate scope:

- levels.rs owns private Levels.parked and Parked world/entity/save fields.
  parkeds' public dimension list alone cannot certify ownership/residency.
- persist.rs::EntityMemory keeps unknown and anonymous entity records private;
  checking Game.dropped alone omits other item-bearing/opaque entity owners.
- nether_portal.rs::Portals privately owns entity portal state and owned Arrival
  payloads; its visible travel field alone does not inspect arrivals.
- World slot snapshots and all active/parked owners require explicit assessment;
  no enabling callsite is added, no diagnostic retirement is consumed.

Do not add broad getters or change those modules in this packet. If a consumer
needs global readiness now, this packet is insufficient; author its concrete
owner predicates/capture seam as the next reviewed unit instead of asserting it.

## Existing serialization seams for the next unit

No payload projection is implemented here. The next borrowed all-player projection
can use the following actual existing code, with separate review of byte coverage:

- lan/playerdata.rs::store(&Guest, &mut Level) borrows live Guest and projects
  inventory, armor/offhand, ender items, motion/saved body, effects, spawn and opaque
  saved NBT. Level::player_tag(&self) builds existing player NBT without writing.
  External players use a fresh Level with their canonical UUID; an integrated
  owner starts from a cloned Level retaining the existing canonical player UUID.
  Never call Level::write or playerdata::save merely to obtain bytes.
- game/lan.rs::record_uuid/records_uuid supply the integrated-owner versus external
  identity rule. store_owner captures only that owner; it is not all-player capture.
- advancements::Progress::to_json(&Tree, DATA_VERSION) is the borrowed projection
  already called by game/advancements.rs::write_progress. Its writer has I/O and
  cannot be called by an observation-only method. Preserve current placed/tree/
  canonical-UUID guards and define explicit refusal for unavailable needed data.
- game/stats.rs::Stats::write constructs JSON from its borrowed values map before
  writing. There is NO existing standalone Stats JSON/byte projection and Stats is
  not Clone. A later small factoring must reuse that exact JSON construction;
  do not read an older disk file or consume notification dirty/events as a snapshot.

The later projection must cover all matched participants in one owner operation,
without as_guest swaps, no payload take and no native normalization. It still needs
root/nested representability checks and exact retained unknown data coverage.

## Meaningful acceptance tests and stop conditions

Use ignored tests requiring pinned HORIZON_TEST_RESOURCES with expect and an owned
isolated process cwd/HOME/data/config/cache/temp with session unset. Never change
process globals. Construct actual headless Game through the existing Fixture.
Use existing lan_host with listen=false, announce=false and online=None. Add at
least two ordinary channel-backed Guest::new values to the actual public hub;
retain receivers so no transport or authentication is needed. Populate real native
motion through the existing actual as_guest swap once where useful, then test
between turns with the hub restored. No mock admission callback or cloned roster
substitute is accepted. Keep a fixture's removed synthetic local body owned by the
test while establishing a server-with-hub-players baseline.

- Dedicated-style two-external-guest success and integrated-owner-plus-external
  success. Use DIFFERENT owner account and canonical level UUIDs. Assert exact
  mapping, order-independent matching and no inventory/body/metadata/notification
  or save-file change. Test both guests even when only one owns rich native items.
- Missing/extra guest, changed conn/UUID/dimension, missing canonical UUID,
  owner/saved-admission mismatch and canonical target collision all refuse.
- Recreate a host with the same visible tuple values: native host identity refuses.
  Replace a guest sender with a fresh pending Arc but the same tuple: refusal.
  A clone sharing the original native sender marker is not a new connection.
  Document rather than falsely test arbitrary same-marker reconstruction detection.
- Cursor (including raw rich nested components), crafting grid/result, native open
  table/workstation/merchant ownership, orphan play and marked retirement, admitted
  inbox, active as_guest and pending teleport/transition/shutdown each refuse
  without changing either guest or Game owners. Use actual opening APIs where
  feasible; defining-module tests cover private cached-menu/activity predicates.
- Repeated refusal/success inspection leaves raw top-level and nested BundleSelected
  integers intact. Equality alone ignores that runtime field; inspect it explicitly.
- Add an off-resident native item and demonstrate that the documented player-only
  result is not a global checkpoint certificate; this is an explicit boundary
  test, not a conservation pass. No file is copied and no manifest is produced.

Coordinator owns exact-base formatting, feature-on/off lint and tests, ignored
resource execution and fresh independent source/receipt review. Stop this unit
when those checks and the bounded evidence are complete. It makes no persistent
counter/machine claim, no coherent checkpoint/recovery claim and no G1 pass.

## Barrier and next dependency

Integrated::run_tasks executes one closure serially between ticks. A later
synchronous Game capture operation can exclude owner-thread ticks, packet handling,
streaming reconciliation and pregeneration until it returns, provided it does not
pump any of them itself. server-hold keeps run_tasks alive and
wait_for_next_server_tick can pump pregeneration; it is not this barrier.
Integrated::call times out after ten seconds WITHOUT cancellation. A timeout is
not proof that its queued/running operation stopped; no capture protocol is added
around that behavior here.

After this packet, qualify borrowed all-player NBT/JSON projection and the global
owner/preflight predicates. Only then combine supported producer drain, all-level
SaveDir barriers and checked metadata with synchronized physical immutable copies
and manifest-last publication/recovery. Expected roster must be rechecked within
that same owner operation, and all missing/unsupported owners must refuse before
publishing. Failed retirement/reconnect history, shutdown lifetimes and interrupted
publication remain explicit work. No general journal or save-format change is
implied. No new owner decision is required for this dormant precursor; scope must
not be relabelled as a smaller permanent S1 deliverable. Public release is separate.
