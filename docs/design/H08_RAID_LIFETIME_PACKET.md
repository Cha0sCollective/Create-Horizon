# H08 retained lazy raid lifetime and dimension identity

Accepted bounded design after a DIFFERENT fresh independent read-only Sol High
review with no blocking findings. Reviewed draft SHA256:
2ea5e9f4f99dc3f38f9b783fd4b00201cbdeec671d7b1e4b2bfc6da207e0ac66.
Qualified successor base is3c4e7e278fd2b8b3776291227c2698490298053a, tree
a321f3aca20feda05f54e9b751ac23d9acbf6753. Projection resource/native/both-mode
Game/broad checks and final independent review now pass at that base. The
coordinator owns assignment, validation and final acceptance. No operating-rule
change, standing team, permission expansion or public publication is proposed.

Inspected Horizon branch: codex/h08-player-projections, HEAD
78c5a0d70824ce397d1a26bd6736a149000d99db. Its only dirty production files are
lan.rs/stats.rs plus a new player-projection test child; they are unrelated to this
packet and remain coordinator-owned. raids.rs is unchanged from accepted H08a14
b3e0d3561a8849c03cd19012ed42c4710b7ec0fe, SHA256
639384eb51ddd26b79bcc393013de028441223255c8959ef6664ccf5f395cab6.
Freeze the accepted successor revision after active qualification before starting;
do not reset, edit or silently fold in unrelated player-projection work.

## Concrete defect and one-module boundary

H08_RAID_LIFETIME_AUDIT.md matches the inspected code. H08a14 makes save_raids return
a contextual error and retain dirty data, but load_raids logs that error and then
replaces the old loaded dimension, helper maps and native Raids. A bare early return
would keep those owners while exposing old-dimension raid IDs to current-dimension
readers/mutators. Numeric IDs can collide across dimensions.

Fix that boundary entirely in crates/horizon-client/src/game/raids.rs. Add its
new cfg(test) child game/raids/lifetime_tests.rs and declaration. Preserve all
existing metadata_tests and their helpers. Reuse the already accepted TEST-ONLY
integrated transition Fixture/new/game visibility without changing fixture bodies,
resource loader or explicit Options. Do not change game.rs, levels.rs, effects.rs,
commands, creatures, save.rs, raid.rs or projection/participant/slot modules.

No general retry/cache service, new Game/Raiding field, journal, save-format change,
World-slot enablement, owner identity ledger, command/API or checkpoint architecture.

## Verified caller and ownership seams

- game.rs::switch_level saves departing chunks/entities and commits World/dimension
  replacement, but leaves raid data lazy. enter_dimension delegates to that method;
  its H08a13 player restoration on transition error remains unchanged.
- levels.rs::swap_level already swaps Raiding's raids, loaded, bar_hurts and homes
  with each parked World. Do not replace or redesign that server-level ownership.
- raids.rs::load_raids has exactly four callers: omen_tick,
  create_or_extend_raid, raid_command_create and tick_raids.
- Commands call raiding_nearby and the exposed raid helpers; creature death calls
  raider_died; bossbar presentation calls raid_boss_at. These paths can run without
  a successful current-dimension lazy load and need identity guards of their own.
- The remaining private helpers reading/mutating Raids are descendants of the
  guarded tick/creation/death/status roots. Patrol/siege/village-geometry and bottle
  helpers that do not inspect the retained Raids must keep their existing behavior.

This is native Horizon ownership work using the existing code and acceptance
contract, not source copied from Create or another mod. Ordinary successful raid
mechanics and the pinned vanilla format remain the specification; no successful
path gameplay algorithm is redesigned by this packet.

## Checked lazy replacement sequence

Change private load_raids(&mut self) to return horizon_client::Result<()>.

1. If self.raiding.loaded == Some(self.dim), return Ok immediately. This is native
   same-dimension access, including a real return to a retained old dimension.
2. If a previous dimension is loaded, call existing save_raids and propagate its
   error BEFORE changing loaded, clearing bar_hurts/homes, drawing destination
   World RNG, reading the destination raids file or replacing native Raids.
   save_raids chooses its file from raiding.loaded, not self.dim: retain that rule.
3. On successful previous-owner handoff, keep the current successful initialization
   order, destination seed draw, tolerant read/parse/default policy and helper-map
   clearing. Set the target loaded identity and return Ok. Initial loaded=None and
   clean/remote save guards retain their existing success behavior.

Preserve H08a14's in-memory save_raid_omen_position projection during an attempted
write, including when that write fails. Consequently the failure guarantee is
retention of raid payload/dirty/loaded/helper-map owners and no destination raid
load/RNG consumption; it is NOT byte-for-byte rollback of the whole Level/Game.
Do not move omen projection or add strict destination reads in this packet.

Do not add retries, reset dirty data, discard an old owner, select a recovery file,
or consume a new destination seed merely to make the failed operation continue.
The original contextual error retains the OLD target file path. Each of the four
callers handles it with concise contextual logging and its existing refusal value:

- omen_tick returns OmenTick::Continue before load_raid_omen_position, before converting
  Bad Omen, and before taking omen_position for a Raid Omen attempt.
- create_or_extend_raid returns None before raid-center creation, native raid RNG
  draw, raid mutation, statistic credit or voluntary-exile advancement. Preserve
  its existing spectator/game-rule/gameplay-attribute gates before the load.
- raid_command_create returns false before creation or omen-level/dirty mutation.
- tick_raids returns before loaded-raider registration, bar flush, wave ticking,
  actions, recruiting/banner ownership, mob/villager bookkeeping or particle RNG.

Keep caller signatures otherwise unchanged. Do not propagate a new effect/death
error through unrelated gameplay APIs. On success the repeated nested same-dimension
load in raid_command_create/create_or_extend_raid remains an inexpensive no-op.

## Direct current-dimension access guards

Add a small private borrowed identity predicate, or equivalent explicit checks:
self.raiding.loaded == Some(self.dim). No read-side IO, lazy save or mutation occurs
inside the predicate. Apply it BEFORE all work in these eight exposed entry points:

| Entry point | Result while loaded identity differs or is None |
| --- | --- |
| raiding_nearby | None |
| raid_started | false |
| raid_status_text | empty String |
| raid_command_stop | no-op |
| raid_set_omen | existing missing-raid result 0 |
| raid_glow | no-op, including destination mob effects |
| raider_died | no-op before flush_raid_bar, cache writes, health/hero/leader changes or clearing a destination mob's raid_id |
| raid_boss_at | None, preserving its existing alive/Overworld/visibility/status guards |

Do not load/save implicitly from these direct helpers or use the same numeric ID
as proof of dimension ownership. No source raid should be presented as a target raid
while replacement is refused. No destination mob with a colliding raid ID should
be altered from retained source raid data. Preserve each helper's existing normal
same-dimension behavior, including current dirty-flag behavior; unrelated command
or death-bookkeeping repairs are outside scope.

The guard does not invent a new world-instance identity. Cross-world close/reopen
lifetimes and same-dimension cross-world aliasing require their own later audit;
do not advertise this dimension guard as solving them.

## Meaningful actual-transition failure/repair tests

Add ignored native tests requiring pinned HORIZON_TEST_RESOURCES with expect,
run by the coordinator in fresh owned cwd/HOME/data/config/cache/temp with session
unset. No process-global cwd/env mutation or playable worlds. Construct the real
headless Game through Fixture, retaining the native player where the chosen caller
needs one. No closure standing in for a save/load and no dim assignment to simulate
the defect. Defining-module test access can seed native raids/helper sentinels
without making any production fields public.

Use actual switch_level for controlled lazy-transition cases. Its real chunk/entity
save and target preparation must succeed while the source raid write is obstructed.
Capture the destination World RNG AFTER that transition: the claim concerns the
failed lazy replacement, not RNG necessarily consumed by world opening. A test using
enter_dimension must finish or account for its travelling player before asserting
omen caller coverage; a missing player would otherwise return before load_raids.

For each source/target case:

- Establish distinguishable source live raid data and an older successful source
  file. Mark the new source data dirty and seed nonempty bar_hurts and homes with
  stable Arc-backed sentinel values. Include live fields not fully represented by
  NBT: leaders, bar state and other native runtime values; to_nbt equality alone
  does not prove those fields were retained.
- Prepare a distinguishable destination raids.dat with a numerically COLLIDING ID.
  Create an owned directory at the OLD source raids.dat_new, then perform the
  actual dimension transition. Do not block the target path instead.
- Require the checked loader to fail with the OLD source path. The exact source
  live raid NBT, dirty flag, loaded identity, helper contents/Arc identities and
  relevant runtime values remain; the old file bytes and destination file stay
  unchanged under this specified pre-open obstruction. Destination World RNG samples
  from clones remain identical. Do not claim all write failure windows preserve
  old file bytes or that raid internal RNG is serialized by to_nbt.

Acceptance cases must include:

1. All four caller refusal values on the actual mismatched Game with the fault
   active. Make their ordinary preconditions true so each reaches load_raids.
   In particular preload the normal gameplay-attribute cache if necessary before
   baseline snapshots, use a real native player for omen, and exercise both Bad
   Omen and duration-1 Raid Omen without taking omen_position on failure.
2. All eight direct helpers with colliding IDs. Seed source started/active/visible
   bar values that would produce an observable result without the guard; seed
   destination native raiders with health, leader/wave, hurt-sequence and raid-ID
   state. Verify no bar/helper mutation, hero/leader bookkeeping, glowing effect
   or destination raid-ID clearing. Include villager/banners/notification state
   in the tick refusal check where their paths would otherwise mutate it.
3. A Nether -> Overworld real transition, not just Overworld -> Nether:
   raid_boss_at already refuses non-Overworld and would mask the missing identity
   check in a one-direction test. Destination Overworld must otherwise qualify
   for a visible living player's bar from the retained colliding source data.
4. Repair ONLY the old owned obstruction and retry load_raids/the original caller.
   Prove current source raid data is written to its ORIGINAL dimension file,
   destination data loads, loaded becomes target, helper maps clear and normal
   target operations resume on the destination record. A clean direct loader retry
   consumes the existing one destination seed draw; later caller-specific RNG
   effects are tested separately rather than misattributed to loading.
5. While still obstructed, return through actual switch_level to the retained
   source dimension. Same-dimension load/access must work without replacement,
   without another save attempt or destination RNG draw. Do not clear dirty state
   or remove the obstruction to manufacture this case.
6. Initial load and ordinary successful lazy replacement retain current native
   formats/read defaults and success behavior. A server enter_level round trip
   should continue to preserve the existing parked raid ownership; no edits to
   its swap logic are needed. Existing H08a14 strict save/fault/repair cases pass.

Tests may compare cloned native Debug state for otherwise private runtime RNG as
additional nonmutation evidence, but exact authored field expectations, NBT/file
assertions and the observable collision triggers remain the correctness oracle.
Avoid presenting comparison of two calls to the same new predicate as coverage.
Keep new helpers local to this test child; preserve qualified metadata test helpers.

## Effect-duration, error-lifetime and checkpoint limits

OmenTick::Continue does NOT pause effect time. effects.rs::PlayerEffects::tick_with
still decrements duration on Continue; duration-1 Raid Omen can expire while loading
is refused. A native effects.tick_with call using actual Game::omen_tick can document
that boundary. This packet neither preserves/replays an expired effect nor retries
a skipped death/command/advancement. Do not silently change effects.rs or claim
full effect-lifetime retry. No old raid can be accessed as a new dimension's raid,
but skipped target bookkeeping is not a new replay queue.

The outer gameplay loop and unrelated creature/effect work may continue; only this
raid operation refuses. Tests of one raid call must not claim the entire game or
all RNG streams are frozen. Repeated attempts can log the retained failure; no new
persistent retry policy or diagnostic state is introduced.

A successful old write followed by tolerant target load remains the existing
policy. This fixes failed pre-replacement lifetime, not all-transition rollback,
shutdown-error Game lifetime retention, earlier lost owners, source-file sync,
checkpoint epoch capture, immutable publication/recovery or G1. Global player/
World/parked/off-resident/history/producer/copy predicates remain required. The
existing World slot service stays dormant.

## Coordinator qualification and stop

After a different fresh design review, bound author work to the one production
module and its resource test child. Coordinator verifies exact base/dirty files,
pinned formatting, feature-on/off compile/lint, new ignored actual-Game cases and
appropriate ordinary raid/effect/metadata regressions. Select and run applicable
upstream gameplay/parity checks where available; distinguish --list selection and
skipped parity from execution. Shared Game behavior needs appropriate both-mode
regression evidence; do not promote the headless transition tests into connected
client or complete checkpoint evidence. Obtain a different fresh independent
source/receipt review of the stable candidate.

Stop when that bounded correction and its evidence are complete. No additional
owner decision is needed for this accepted private prerequisite. No operating-rule
change, upstream contact, public submission, merge or release is authorized here.
