# Absent-local-player tracker persistence amendment

DESIGN ACCEPTED by fresh independent native read-only Sol High review at exact
proposal SHA2564f3e2ed55e6a609881bb4d8794cfa79207ec08d3485a7bcbe90bd698ef9b8d3a.
Exact proposed text remains .local/s1sc1-local-tracker-proposal1.md. Private
checkpoint precedes source release; source/runtime/final acceptance remains open.

Extend the accepted checkpoint scope from nine to
exactly ten engine paths by adding existing game/sculk.rs. Keep the strict inert
native local-player profile unchanged. The current nine-path candidate is frozen
at core820762e0/tests2705ad3a/raids719efe1c on baseff69d5f.

## Reproduced native dependency

Run89418 passed193 ordinary and resource0..8, then the extended resource9 passed
its integrated-owner three-epoch/clean-raid/failure/retry checks but failed the
dedicated second checkpoint. It reported an inert native local record mismatch.
The fixture already has options.dedicated=Some; it is not an in-frame role error.

A focused unchanged-source reproduction18524 failed identically in103.26s. Its
whole world is verified present in persistent owned storage:
/home/arbiter/.cache/create-horizon/session-tests/s1sc1-session-resource-9-uhiuzemr/
temp/horizon-transition-411-0-session-two-epochs-retained-411-0/fixture.
The native record comparison shows both actual participant records unchanged.
The inert record differs only by warden_spawn_tracker: a compound containing exact
Int ticks_since_last_warning1, warning_level0 and cooldown_ticks0.
Raw diagnostic .local/s1sc1-inert-nbt-diff.json retains those types.

Native tick_sculk_entities unconditionally loads and ticks the Game's shared
tracker; save_game_metadata then save_warden_tracker writes it into level.player_extra.
Canonical inert validation correctly refuses that nondefault player-owned state.
Pinned vanilla ServerPlayer bytecode owns, reads, saves and ticks this field:
.local/s1sc1-serverplayer-javap.txt (field115, read342, save458, tick919).
Pinned server-game.jar SHA256a362163eec5d1612d520772bc16e5b39c09e3b234fdc045f56bf544284ee8ae6.
This is primary bytecode inspection, not an imported implementation.

## Narrow correction

Guard ONLY local warden-tracker load and save with the existing Game.no_player_here
predicate, as native advancement/statistic persistence already does. No local
player means a server context (options.dedicated=Some) outside an acting guest.
In-frame local-player and acting-guest contexts remain eligible. Do not create,
remove or normalize a tracker field in an inert record; do not erase an existing
failed record or broaden its accepted profile. Unsupported old input still refuses.

Retain the existing shared runtime tracker tick, global sculk tick counter, world
index/preparation, darkness, stepping, entity and block-event work unchanged.
Current guest warning state shares this tracker: SculkState::swap_player moves
steps/health but not the tracker, and guest warnings can modify it. Suppressing its
tick would change existing cooldown/decay and also break the default component
parity fixture's explicit scenario player. No guest tracker ownership redesign or
fixture exception is proposed. This correction prevents local-record contamination;
it is not a claim of full vanilla per-guest tracker conformance.

The checkpoint's closed owner census must also reject an actual nonzero warning
level or cooldown and invalid negative tracker counters. Its nonnegative elapsed
counter with zero warning/cooldown is inert for this no-sculk-owner experiment:
it does not create an owner, and a later first warning resets the elapsed counter.
Do not copy this shared runtime field into inert native metadata. Actual active
sculk/block/entity ownership remains unsupported and must refuse.

Only three source paths are released for this correction: game/sculk.rs, existing
game/levels/session_checkpoint.rs for the named closed-state check, and existing
game/levels/session_checkpoint_tests.rs for actual-Game qualification. Other seven
paths, including all raid helpers, remain frozen. No new API, schema, native codec,
general journal, ordinary startup/close or filled-slot caller. Do not add a generic
hook or weaken existing guards. Keep the existing21 checkpoint and one supporting
ordinary test names; expected normal count remains193.

## Qualification

- Existing repeated real-Game case must finish three epochs for both roster roles,
  preserving current raid metadata, players/items, clocks and fresh authority.
- Within existing resource controls, actual dedicated and integrated-authority
  contexts without a local actor must not load a local tracker or add its tag on
  save. The shared runtime tick must still advance. In-frame local-player and
  actual acting-guest contexts must still load/save player-owned metadata. Use
  actual native player/guest objects, not a role-only fake or owner clearing.
- Actual nonzero warning/cooldown and invalid counters must refuse checkpoint;
  elapsed-only inert state must not prevent the promised successful normal tick.
- Retain relevant library warning/cooldown and game sculk checks. Run the existing
  pinned sculk parity coverage, including warning cooldown/decay and the default
  server-thread component fixture. Do not silently replace it with in-frame-only
  qualification. Preserve source/fixture identities and all failures.
- Full checkpoint resources/retained groups, feature configurations, optimized
  type check, appropriate clean broad checks and different final review remain.

Fresh independent native read-only Sol High acceptance of this exact amendment and
a private checkpoint precede tenth-path author release. Exact source/tool delta
clearance precedes root-only serial execution. Never commit raw game bytecode,
worlds, resources or diagnostics into engine Git.

## Evidence storage correction

The previous /tmp retention paths were logged at failure but later unavailable:
WSL had restarted (uptime1minute) and the whole temporary scratch had disappeared.
Source/log/receipt archives on G remain. Do not describe those old world paths as
currently verified artifacts. No root-issued shutdown/restart/cleanup occurred.
The reviewed runner now creates fresh scratch under persistent owned session-tests
storage, including temp/home/data/config/cache/launcher paths. Focused mode is
explicitly incomplete, writes a separate receipt and cannot count as a suite pass.
Persistent repro18524 confirms actual world retention and the exact metadata fault.

No owner policy decision or reduced S1 is proposed. Ordinary lifecycle, connected
admission/replication, committed presentation, both-mode and two-client G1 still
require their own implementation and evidence.
