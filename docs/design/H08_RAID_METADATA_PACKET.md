# H08a14 checked raid metadata save

Bounded successor accepted by fresh independent read-only Sol High design review
with no blocking findings, conditional on H06d acceptance before implementation.
Reviewed draft SHA256: 16da4e42abf857c707d167dd0588f4036e8ae4c0a5afa643284df7c2ccd079f3.
Accepted base
62756231205b1b2b65fcf92c180fc29497f29715 passes native/resource/broad and final
independent receipt review. This closes
one false-success disk writer, not coherent checkpoint/recovery or live slot enablement.

## Concrete problem and exact scope

Game::save_level propagates forced chunks, Streaming, modified chunks, entities
and flush errors but calls void save_raids. The raid writer retains dirty on a
failed write_dat yet suppresses its error, so both synchronous and incremental
save callers can acknowledge completion without the requested raids.dat write.
No strict write_metadata helper exists in the checked source; use existing write_dat.

Production scope: game/raids.rs, the save_level call in game.rs and the Entities
stage in game/saving.rs. Preserve native raid format, dimension path, read policy,
remote/unloaded/clean guards, save cadence, dependencies and all slot/counter code.
No journal, new record format, publication, control command or general save service.

Tests: add a resource-backed child game/raids/metadata_tests.rs with declaration
in raids.rs, so native Raiding fields stay private in production. Reuse the existing
actual headless Game fixture through TEST-ONLY visibility changes: integrated.rs's
cfg(test) transition_ownership_tests module becomes pub(super); Fixture and its
new/game methods become pub(in crate::game). Keep fields and shared loader private,
explicit Options literal and all prior test bodies unchanged. No production field
or constructor widening; no renderer/event-loop/account configuration changes.

## Result and caller sequencing

Make save_raids return the existing horizon_client::Result<()>. Guards return Ok.
Preserve original raid Tag/DataVersion and canonical raids_path. Add that full path
to a write failure. Clear dirty only after success. Preserve the existing logical
save_raid_omen_position call even when the attempted write fails; this is in-memory
projection, not another writer. Return the write result after that projection.

In save_level, capture raid_result, still perform the existing flush when wait=true,
then return the earlier raid error if raid and flush both fail. Queued region/entity
work must not be abandoned merely because the later metadata write failed. Keep
all earlier producer ordering and early error behavior unchanged. For wait=false,
propagate raid failure without inventing a new flush.

In incremental saving, propagate save_raids failure before forced chunks/metadata
and Stage::Writes. Preserve poll_closing_save's current error response and retained
pending_save. Do not claim outer about_to_wait/shutdown failure lifetime is fixed.

The third caller in load_raids must explicitly handle the Result. A contextual
warning preserves its existing policy; lazy old-dimension replacement after a
failed save is a separate known ownership bug, NOT repaired/qualified here. Do not
quietly ignore a must-use Result or claim general raid lifetime conservation.
If design review concludes retaining that existing policy is unacceptable for this
bounded change, return a concrete minimal alternative before adding callers/scope.

## Meaningful acceptance

Run actual Game tests with pinned resources in fresh owned cwd/HOME/data/config/
cache/temp, session removed, inherited display runtime and no process-global edits.
Use native Raids create_or_extend or equivalent real domain operations to seed
nonempty distinct state; test module can inspect its ancestor's native fields.

1. Obstruct the actual raids.dat_new path with an owned directory. save_raids
   returns a contextual error, preserves exact native raid data and dirty; repair
   and retry writes the exact expected current Tag and then clears dirty.
2. Unloaded, clean and remote guards are no-op Ok and create no raid file. Check
   the existing in-memory omen projection boundary on success/failure as relevant.
3. Actual save_level(true) reports the raid failure while previously enqueued
   chunk/entity data finishes; prove it using a NEW SaveDir disk reader and a
   nonempty native entity, not game.save's pending-snapshot cache. Repair/retry
   succeeds. Include raid-versus-flush
   error precedence using real owned disk obstructions if it can be deterministic,
   not a fake writer replacing production sequencing. Use a separate fixture for
   simultaneous raid/flush failure: obstruct entities with a file and raids.dat_new
   with a directory. The writer error is sticky; do not bypass/clear it or claim
   repair/retry success for that dual-failure fixture.
4. Actual begin_control_save/poll_closing_save reaches the writer error, emits no
   success acknowledgement and retains pending_save. Repair allows completion;
   distinguish a completed retry from an already-consumed error reply channel.

Root owns formatting, serialized builds/tests, feature checks, hooks, broad checks
and integration. New production save paths also receive isolated full counter Game
regressions in integrated/in-frame modes using the immutable stable fixture wrapper.
Use a new binary/run receipt; do not relabel prior H08a13 executions. Run appropriate
parity selection and retain explicit skips/limits. Fresh independent source and
receipt follow-ups review the exact candidate and any delta.

## Stop boundary and following critical path

No owner decision is needed for this routine error-reporting precursor. Stop and
return a concrete revised packet if correct sequencing requires a general retry,
retirement/reconnect or storage service. Known outer lifetime, lazy raid replacement,
active external-player capture, off-resident entity ownership, all-dimension barriers
and coherent publication/recovery remain open. After this writer prerequisite,
consider a dormant owner-thread expected-roster preflight and borrowed all-player
projection with NO writes. Its output is a boundary observation, not a persistent
barrier after normal gameplay resumes. Integrated::call timeout does not cancel a
queued/running task; server hold and wait_for_next_server_tick can keep doing work.
G1 still requires coherent recovery and the specified separate-client evidence.
