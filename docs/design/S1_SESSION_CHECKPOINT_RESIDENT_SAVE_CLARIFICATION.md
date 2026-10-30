# Current resident native save submission

DESIGN CLARIFICATION ACCEPTED by independent native read-only Sol High review at
exact proposal SHAfc93a257bd0f86be4106b5cd5ce5caed65cd1e67702249110169982e49e57bed.
Exact snapshot workspace.local/s1sc1-resident-save-proposal1.md. Source correction
requires private checkpoint and exact delta review before any execution. This is
a narrow clarification to accepted session checkpoint proposal47cf3a13. Same eight
paths, same identified TestTerrain scope; no native SaveDir/streaming/schema change,
ordinary caller, journal, permission expansion, relaxed conservation or G1 claim.

Source review of frozen core124bd34b4c797b8e15e663eb196735dc72d0b22697e38c0948ffc53423c79573
found that save_game_with(false) alone does not persist every current resident field.
An ordinary spawn_natural pass increments world.inhabited even with mob spawning
disabled, without marking the chunk modified (natural_spawning.rs487). Native
with_entities/attach_save_data projects current inhabited metadata only into cloned
save snapshots (persist.rs75-103). The normal save path writes modified residents,
so disk can retain an older inhabited time after valid clean-chunk history.

Clarify the capture save phase under the same continuous exclusive Game borrow:

1. First native producer/writer fence, then complete current owner/roster/counter/
   slot/player/resident native projections, exactly as already designed.
2. Call save_game_with(false), preserving native errors and existing metadata/entity
   behavior. Do not call save(true) or flush.
3. Obtain current native resident snapshots through actual Game::with_entities and
   its attach_save_data, then submit them through existing checked asynchronous
   SaveDir::save_later with the actual registry/biome inputs. Every resident current
   native column is a real queued write, not a fabricated callback/count/readiness
   result. Preserve outstanding ownership and errors on failure.
4. The existing second fence, with the SAME deadline, closes both ordinary save
   submissions and these current resident submissions before copy/publication.
5. Re-project/check stable semantic owners and require full current native resident
   saved-field equality with the copied image, including current LastUpdate,
   InhabitedTime, ticks, sections and all other supported saved metadata. No stripping
   inhabited fields, blocks-only comparisons, clearing world ownership/history,
   synthetic dirty markers or forced pristine eligibility.

This adds bounded checkpoint-specific native I/O, not a change to ordinary autosave
semantics or an overlay of invented region records. Initial image/residency bounds
and unsupported-owner refusals remain in force. Off-resident records retain their
strictly verified genuine historical timestamps; only actual current residents are
submitted at the checkpoint's current saved clock. Performance remains unqualified.

Decisive regression: after a genuine native save/fence leaves the target resident
clean, run an actual Game tick with natural mob spawning disabled and random ticks
off. Require actual inhabited time to increase while its modified marker remains
absent. Checkpoint/recover, then advance/transfer/recapture/recover with metadata,
item totals and fresh authority intact. No manual clock bump substitutes for this
case. Controlled writer failures still refuse publication and retain previous
complete images and actual writer ownership observations.

The same core review found two implementation bugs under already accepted bounds:
enforce the shared 256-entry tree budget WHILE enumerating, before collection/sort;
and require dedicated inert native singleplayer_uuid disjoint from ALL participant
records plus canonical inert-file verification before publication/checked brand
issuance. These restore the existing design, with defining negative tests. They do
not add source paths or new supported state.
