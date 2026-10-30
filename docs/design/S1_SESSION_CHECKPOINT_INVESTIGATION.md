# Ordinary S1 checkpoint investigation

READ-ONLY planning against engine ea96036. NOT API/design adoption, source release,
ordinary recovery or G1 evidence. Existing H08 research acceptance/guards stand.

Ordinary game save_game_with writes native chunks/entities/metadata, not host
counter/slot frames (game.rs11411-11471). Current epoch capture/copy/manifest/recovery
integration is test-only Linux code (game/levels.rs1047). Fixed owner11/external12,
nine-column/pristine/no-drops/counter4/PLAYER3/slot7/epoch values are fixture inputs,
not ordinary supported-world facts. require_pristine_fixture_capture still refuses
submission history. Native fence success closes inspected producers/writer under
one exclusive borrow, granting no transferable general readiness.

bootstrap_host_counter is dormant (game/counter_control.rs160); ordinary WorldRequest
and integrated startup select IdentityIntent::Legacy (world_loading.rs42,
integrated.rs213). Internal create/reopen identity seams exist. NativeBinding maps
symbolic alias to physical iron fixture deliberately (world/counter_restore.rs209),
whereas transient counter binding resolves alias registry ID: choose explicitly,
do not interchange constructors. Player-cell lend and slot retirement adapters have
no ordinary authorization/lifecycle caller; a runtime key is not authorization.

Smallest proposed dependency order:

1. Separate bounded PRODUCTION session-checkpoint operation permitting actual native
   streaming/save history, actual identities/current counter+slot values/roster,
   positive fence/native save/projection/immutable copy/complete publication and
   checked fresh reopen/install. Preserve existing pristine research APIs unchanged.
2. Then explicit opt-in startup/placement/authorized ingress, pending retirement,
   close/open callers and full ordinary lifecycle. Do not enable a filled live slot
   whose only copy ordinary close may discard.

Proposed first-packet source scope ONLY after independent design acceptance:
game/levels.rs; newgame/levels/session_checkpoint.rs and session_checkpoint_tests.rs;
game/lan.rs participant projections; game/world_loading.rs and
game/world_loading/host_identity.rs checked install/identity correlation; game.rs
bounded save/open state/seams. Existing streaming/save/native codec are dependencies,
not preauthorized edits. Demonstrate precise extra dependency before amendment.

Required reachable inputs/histories for eventual promised machine:

- World/dimension/position, allocation high-water marks, active/dormant generation,
  record/slot revisions, counter/last advance, unload versus removal; fresh restored
  runtime authority, stale session keys rejected.
- All actual participating player inventories/mappings. Dedicated cannot use a
  synthetic integrated owner; departed/reconnected guests retain failed ownership.
  lan.rs2082 currently logs guest save failure then discards departure state.
- Authorized transfer checks current player/session/dimension/residency/generation/
  revision/reach/permission/held item/count and deduplicates session-epoch sequence.
- Pending retirement is a real owner; slot persistence currently refuses it
  (world/slot_restore.rs1167). Removal must deliver exactly once or durably retain
  pending ownership, including save/close failures. Dropped items are reachable.
- Ordinary native chunk metadata must be inspected/projected, not all opaque roots
  accepted. Current research alternatives accept only exact empty metadata at
  current/previous tick (world.rs1149-1251).
- Finite native player values include signed zero. Fixture bit-roundtrip tag_bounds
  rejects -0 while native reader canonicalizes it and writer canonicalizes NaNs.
  Adopt an explicit bounded native-semantic projection policy; no universal codec
  project for arbitrary NaNs/exotic malformed component graphs is implied.
- Genuine replacement/retirement history remains necessary for eventual G1.
  First active-resident checkpoint may defer it explicitly, not forever refuse it.

First positive proof must use authoritative actual Game, captured package, REAL
nonzero native submission/write history, nonzero counter/positive slot/player shares
and actual roster: fence -> native saves/projections -> immutable physical copy ->
complete synchronized manifest -> fresh recovery directory -> checked native reopen.
Advance/transfer, recapture and reopen with unchanged total accounting/fresh keys.
Negative writer timeout/error/panic/late handoff; roster/projection change; wrong
world/package/schema; corrupt/missing member; interrupted copy/manifest/promotion;
post-rename failure. Retain previous complete checkpoint and distinguish definite
nonpublication from uncertain publication. No success-shaped fixture substitution.

Subsequent ordinary qualification adds actual player/command replacement, inactive/
unload/reload/pause/removal/drop/pickup/callback abort/stale replay/save-close-reopen/
failed disconnect-reconnect, then dedicated+two admitted exact clients. Admission
and retained presentation are separate dependencies.

No current owner policy choice is required for technical investigation/design review.
New broad journal, changed save compatibility, reduced promised S1 deliverable or
relaxed conservation would require a separate concrete decision. This document
adopts none of those and loosens no existing guard.
