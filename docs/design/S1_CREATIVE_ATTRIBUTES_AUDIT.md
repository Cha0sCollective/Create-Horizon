# Idle Creative attribute recovery audit (not a source release)

Input from bounded native read-only audit during connected Survival validation.
Current eight-path source manifest1dffa36f is frozen; change no current guards.
This is a subsequent required S1 adapter input. A separate exact proposal and fresh
independent design/source reviews precede code. No new save-format/API promise.

Native game/lan_attributes.rs:update_player_attributes derives exactly two Creative
transient AddValue modifiers from verified game mode1:
- minecraft:block_interaction_range / minecraft:creative_mode_block_range / +0.5
- minecraft:entity_interaction_range / minecraft:creative_mode_entity_range / +2.0
It also lazily instantiates default movement speed/reach in other modes. Presence
of a default instance is not nondefault saved semantics.

The current strict session_player_level rejects any modifier; a real Creative
source therefore refused as intended. Survival0 BEFOREadmission is the qualified
source fixture. Never remove modifiers from an existing captured body to pass.

A narrow future route can retain the format: playerGameType/abilities already
verify the mode. Native Level/playerdata do not serialize the live attribute map;
current closed NBT rejects arbitrary attributes. Extract a fallible native helper
on PlayerMotion; derive the two additions only on a FRESH reconstructed body after
mode/body flags, before projection/placement. Capture must independently validate
catalog-default bases and only the exact permitted transient ID/amount/operation/
attribute combinations. No permanent modifiers or extra/stale operation entries.
Use the same validation for final body correlations. Existing scalar-only
session_body_matches_level does not establish attribute equality.

Inspect permanent_modifiers and operation buckets explicitly: updating an existing
transient ID to another operation may leave an older operation bucket. Displayed
modifier lists or equal final numerical reach do not prove the live map is exact.

Client delivery is also required. Normal Hub::sync_player_state sends dirty native
UpdateAttributes; frozen recovery never runs that ordinary tick. Send a bounded
initial native attribute snapshot through the staged sender, and require actual
client PlayerMotion attribute/ability observations before readiness. Existing
remote.rs native UpdateAttributes handling supplies the client seam. Do not call
normal/paused ticks merely to derive or sync attributes.

Keep other native states unsupported in this small profile:
- Sprint: minecraft:sprinting uses f64::from(0.3_f32), AddMultipliedTotal. Sprint
  intent is not part of the accepted saved player representation; mode is insufficient.
- Frost: minecraft:powder_snow transient movement modifier; nonzero frozen state
  is already rejected. No generic native-modifier whitelist.
- Equipment/enchantments/effects: even ordinary armor can add modifiers; reconstruction
  needs verified equipment and GuestPlay/potion bookkeeping, a separate bounded scope.
- Commands/nondefault bases/permanent modifiers and unsupported saved fields refuse.
LAN waypoint hiding currently computes range directly; it adds no crouch modifier
on this live map. Default waypoint instances remain ordinary defaults.

Likely focused files: game/lan_attributes.rs, game/lan.rs, lan/hub.rs and connected
recovery tests. Actual-client Creative3/5-item capture/recovery/recapture/close,
other-mode controls, idempotent derivation, wrong amount/operation/permanence/base/
stale buckets/equipment/effect negatives and both publication retries are meaningful
proofs. Preserve zero recovery ticks, actual keys/leases/slot accounting and all
source/prefix evidence. No owner-policy decision was identified by this audit.
