# Local-player and durable own-world adapter map

Read-only planning against accepted89dff650/tree67460e58, independently mapped by
s1_local_restart_adapter_map. This is technical input for following adopted S1
work, not source release, a new gate or acceptance of an implementation. Atomic
placement packet s1plc1 remains separately bounded; ordinary activation stays closed.

Status update: the historical LOCAL participant/body/owned recovery gaps below are
implemented by accepted/public0db9427; see S1_LOCAL_COHERENT_RESULTS.md. Durable OWN
remains unimplemented. The later planning section records fresh independent mapping
against that component's sources; it is not a source assignment or design decision.

## Actual local participant

game/lan.rs checkpoint_participant_host requires LanHost and rejects local body/
inventory; session_context_cpu in game/levels/session_checkpoint.rs and
inspect_session_checkpoint_owners in game/levels.rs reject the client role.
game/window_close.rs's Phase::Running selects NativeWorld even with a domain.

Add an explicit LOCAL alternative backed by the original NativeCloseAuthority,
session, native UUID and PlayerMotion. Do not substitute a dummy network Guest or
empty roster. Reuse project_sync_level's actual local inventory/components/armor/
offhand/ender projection and existing pack_player_level/body/tag validators.
The Level cache omits live motion/ground/food exhaustion/timer updates: overlay
the actual local body as the Guest projector does before coherent capture. Existing
CoherentSession CloseOwner already retains producers, writers and publication.

## Actual local recovery

OrdinaryPreparation::prepare_inner refuses persistent in-frame recovery without a
NativeReopenClaim. fresh_session_candidate requires a headless non-client candidate;
prepare_steps always builds ServerGameState; ordinary activation expects configured
network Guests. Removing one refusal is not a local recovery adapter.

Use a LOCAL branch of the existing unpublished owned recovery work, validate one
actual native local row and complete owner census, install inventory/progress/stats/
body, then publish to the original ordinary in-frame frontend. Reuse native local
inventory restoration, finish_open_world and spawn_player. PlayerMotion's ordinary
load_saved_state_once also omits motion/ground/food exhaustion/timer, so the checked
local installer must set and verify those actual values explicitly. Keep original
failure/cancelled worker and authority custody; renderer remains outside recovery.

## Durable own-world routing

closed_worlds/NativeReopenClaim, coherent Arc association and OrdinaryRecoverySource
are intentionally process-bound. Actual ordinary integrated/dedicated creation also
supplies continuation_archive=None. An identity-file read does not recreate their
original owners or certify a complete authoritative image.

Add only a bounded durable OWN association established by actual ordinary creation,
reopened under existing native/namespace locks. Bind exact logical world, supported
package/profile, native player mapping, claimed target/type and checkpoint/archive
route. Validate fresh startup/resource owners against that durable association and
reuse image verification, complete-epoch selection, fresh destination and under-lock
recheck. Existing IdentityPreflight/IdentityAnchor/WorldAuthority are building blocks.
Do not reconstruct former Arc objects or pretend serialized hashes are old claims.
No arbitrary-world import, general journal or regional format redesign is needed.

Never-placed restart is a real additional case of that adapter: HostCounterBootstrap
exists only for PreflightKind::New. Reopened identity contains no unused/claimed
history. Current coherent manifest requires target/type and a counter domain; its
counter format permits Dormant/high-water0 but no current initializer constructs
that state. A click-selected first target has no honest target before first placement.

Keep click-selected placement and design bounded durable unused/claimed provenance
with publication/recovery correlation. A stale unused record after a claimed native
save must not authorize a new bootstrap. Do not mint for_new_host_world from an
existing UUID or serialize/clone the old grant. A fixed preselected target would be
a separate product choice; it is not implicitly adopted to avoid this adapter.

Focused later checks: actual local body differing from Level caches; local counter/
slot/coherent close/recovery; one failed publication and original retry; second OS
process with freshly loaded resources; never-placed close/restart/first placement;
claimed/retired restart without a generation reset; conflicting route/identity/
package/mob rule/player mapping refusal with preservation. Reuse unchanged network
coherent worker evidence. These are required outcomes within the adopted plan, not
a historical full test matrix or expanded game profile.

## Latest durable OWN mapping

Independent read-only s1_local_restart_adapter_map recommends ONE stable separate
archive per logical owned world, plus a strict association created only by actual
ordinary new-world construction under original native/namespace locks. Bind logical
UUID, canonical original anchor, supported role/native player mapping, fixed mob-free
TestTerrain profile, captured Guest pair/type/schema, existing native package PIN,
archive and permitted fresh-destination parent. Existing WorldPackages PIN already
contains exact executable build SHA256 and package identities; reuse it.

Restart uses anchor identity only to locate the record, then validates it under
actual locks with freshly loaded packages/resources/startup. A distinct durable
source retains NEW actual per-run validation owners. It must not reconstruct old
Arcs or OrdinaryClosedWorld. Existing process-owned sources keep their strict
empty/separate continuation checks; a narrow durable source branch can append to
its explicitly bound stable archive. Existing newest-complete selection, full
inventory hashes, native copy/read/fence, unpublished native/player correlation
and original cancellation ownership are reusable. Current bounds remain: fewer
than256 archive entries and64MiB image; exhaustion refuses. No automatic cleanup.

Never-placed requires an explicit Unplaced complete image: NO target/counter/slot
allocation, but the SAME complete native player/item/world/clock inventory and
captured policy. Verify absence of domain/trial/slot/heads and physical registered
alias in all saved columns. Keep claimed GuestV3/slot generations unchanged; an
outer owned-image distinction avoids weakening SessionCounter's current contract.
After full verified Unplaced installation under held locks, a distinct recovery
constructor may issue a NEW session first-placement admission tied to that checked
World/history. Do not serialize/clone the previous grant or invoke
for_new_host_world(existingUUID). The actual first click still chooses the target.

A separate stale unused=true marker mixed with newer native records is unsafe.
Writing claimed before physical placement also leaves a crash window. Instead,
the selected complete image carries unused/claimed provenance AND every item owner.
An older complete Unplaced image restores all its old native/player/item state;
a complete Claimed image restores its target/generations. Preserve newer failed
working files; never combine their records with old unused authority. Zero complete
epochs refuse/preserve. Supporting recovery before first-ever checkpoint would
also require initial live Unplaced publication at actual player-ready boundary;
terminalizing CloseOwner cannot silently act as that autosave.

Likely minimum core budget, NOT released: host_identity.rs,
host_identity/session_authority.rs, world_loading.rs, session_checkpoint.rs,
paired_close.rs, game.rs, world/counter_restore.rs and existing registered tests.
Ordinary dispatch later explicitly needs window_close.rs and retained_server.rs;
current closed_worlds/check_ordinary_native_entry still require process-local
claims and ordinary startup supplies no persistent archive route.

Focused validation can use two child processes and two isolated worlds. A creates
and closes genuine Unplaced and Claimed worlds, then leaves an interrupted successor
for the latter. B independently loads resources/startup, restores Unplaced and
chooses its first target exactly once; restores Claimed from last complete image,
preserves counter/slot/player totals and generations, rejects virgin bootstrap and
old runtime keys. Check namespace exclusion, native record mapping and original
failed-file preservation. Reuse unchanged corruption/newest-complete/network tests;
no general restart framework or new supported simulation profile is proposed.
