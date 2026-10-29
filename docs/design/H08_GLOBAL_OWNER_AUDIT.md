# Current global-owner admission audit

Read-only audit at engine base 78c5a0d, with the unrelated player projections now
locally committed as 3c4e7e2. This note proposes a later bounded packet; it accepts
no implementation, global checkpoint readiness or reduction of S1/G1 scope.

The smallest useful next global step is a dormant borrowed all-loaded-level
supported-owner observation. It should refuse unsupported owners with finite
causes, without swapping dimensions, normalization, take, repair, IO, serialization
or a lasting permit. Actual Game tests should show a passing PLAYER preflight
coexisting with each independently refused global owner, including a parked-only
owner and an off-resident dropped item. A fresh design packet/review is required.

## Native boundaries to inspect together

- game/levels.rs:42-88 gives every Parked a separate World, Streaming, SaveDir,
  EntityMemory and entity vectors. swap_level:202-246 exchanges these owners.
  Borrow active and parked records in their defining module; do not enter_level
  merely to inspect. save_parked_levels:496-509 can return on failure before
  restoring the original active dimension, so it is not yet a safe capture iterator.
- game/persist.rs:987-995 saves entity chunks only for World.chunks keys. Current
  off-resident item/mob/vehicle owners can therefore be omitted. EntityMemory's
  private unknown and anon records (:21-32) also need explicit coverage/refusal.
  World.loaded_entities needs coverage or refusal without spawning pending data.
  Check finite positions before floor/cast chunk mapping.
- Residency alone is insufficient. entity_chunks omits Game.tridents and rockets,
  whose ThrownTrident.stack and Rocket.item own item payloads (trident.rs:21-24,
  fireworks.rs:22-43). It emits creatures.projectiles only when an existing arrow
  or fireball encoder succeeds (persist.rs:690-709). A first bounded admission can
  refuse unsupported families rather than incidentally expanding codecs.
- Refuse World.world_slot_trial.is_some() directly (world.rs:91), including parked
  worlds. Do not clone an inspection snapshot, take pending retirement owners or
  mistake an empty slot for absence of trial/fault/configuration ownership.
- game/nether_portal.rs:82-98 separately owns travel and arrivals. Use a defining-
  module borrowed predicate; nonempty unsupported arrivals can refuse. Earlier
  loss is separate: resolve_arrivals removes an arrival before exit resolution,
  then continues on failed nether_exit_portal without restoring it (:953-966).

## Producer completion remains a distinct phase

Streaming::busy (streaming.rs:904) omits saving, written, generation_save_error and
failed_finished. pending/stages are not writer barriers either. A defining-module
borrowed state check must inspect those fields for EVERY active/parked streamer.
save_in_flight (:1507) covers decoration/finish/save; save/poll_save perform checked
mutable reconciliation. SaveDir::writes_finished (save.rs:2229) retains errors
but only covers work already queued. These observations cannot replace checked
producer completion and all-level saves in the eventual exclusive operation.

Built-in providers originate in world_loading::make_streaming:284-348. Conclusions
about harmless plain generation apply to those inspected providers, not arbitrary
ChunkGenerator implementations. Conservatively refuse cmds.pregen.is_some();
pregen_level also observes paused tasks. A future capture may not pump pregen,
tick, poll guests, reconcile unrelated owners or serve mutating tasks during its
exclusive capture/save/copy operation.

## Hard gaps a current observation cannot solve

Failed Left already discards Guest/play state (lan.rs:2082-2104). A later clean
roster cannot detect or reconstruct that history; retained retirement/reconnect
ordering or an explicitly tracked fresh experimental lifetime remains necessary.
Earlier unload/portal losses cannot be certified away by observing empty owners.

Current participant preflight refuses ordinary in-frame client-role/local-body
state (lan.rs:256-270,341-363). Both-path capture needs a separate local-player
adaptation; passing ordinary gameplay regressions does not enable the helper.
All player/entity component scalars and full byte encoding, machine persistence,
all-level error ownership, definite task completion, synchronized physical copies,
manifest-last publication and interrupted recovery remain open. Integrated::call
timeout does not cancel the queued/running task. Conservative diagnostic refusal
is not acceptance of a permanently restricted S1 deliverable.
