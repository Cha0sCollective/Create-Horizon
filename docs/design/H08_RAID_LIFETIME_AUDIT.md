# Lazy raid replacement ownership audit

Read-only proposal against accepted local engine b3e0d3561a8849c03cd19012ed42c4710b7ec0fe.
raids.rs SHA256: 639384eb51ddd26b79bcc393013de028441223255c8959ef6664ccf5f395cab6.
No implementation or design acceptance follows from this note. Participant
preflight is the current packet; this is another dependency before full capture.

H08a14 reports strict save errors but intentionally preserves load_raids' existing
lazy policy: a failed previous-dimension write is logged, then old native state is
replaced. A bare early return would retain data but expose old-dimension raids to
new-dimension operations. A bounded future fix can remain in raids.rs plus its
resource test child; it does not require a general retry or storage service.

Proposed contract:

- load_raids returns the existing Result. Same-dimension access returns Ok.
  Propagate save_raids failure before changing loaded, clearing helper maps,
  consuming destination World RNG or replacing/reading destination data. Keep
  H08a14's existing in-memory omen projection during an attempted write.
- Four callers stop their raid operation with contextual logging: omen_tick
  returns Continue, create_or_extend_raid returns None, raid_command_create returns
  false, tick_raids returns before registration/bars/waves/mob-villager updates.
- Direct current-dimension entry points must also require loaded == Some(self.dim):
  raiding_nearby None; raid_started false; raid_status_text empty; raid_command_stop,
  raid_glow and raider_died no-op; raid_set_omen existing missing result0;
  raid_boss_at None. Guard death before bar/cache changes and mob raid-ID clearing.
  Numeric raid IDs can collide across dimensions. Private helpers reached only
  through these guarded roots need no separate loading policy.

Meaningful future tests use actual switch_level/enter_dimension rather than
hand-assigning dim. That transition saves chunks/entities but leaves raid replacement
lazy. Block the OLD loaded dimension's raids.dat_new; require exact old NBT, dirty,
loaded dimension, sentinel helper maps and post-transition RNG retained. Cover all
four callers and every direct helper with colliding IDs/destination native mobs.
Include Nether -> Overworld because raid_boss_at's existing non-Overworld refusal
would hide the missing identity check in an Overworld -> Nether-only test.

Repair only the obstruction, retry and prove old raid data reaches its original
file, target data loads, loaded changes, helper maps clear and destination operations
resume. Returning to the retained old dimension while still blocked should restore
same-dimension access without requiring replacement.

Scope limits: Continue does NOT pause effect duration in effects.rs. A duration-1
Raid Omen can still expire; this proposal does not replay skipped effects/deaths or
promise effect-lifetime retry. Outer shutdown, coherent checkpoints, earlier lost
owners and general player/entity retention remain separate. A fresh bounded packet
and different independent design/source review are required before implementation.
