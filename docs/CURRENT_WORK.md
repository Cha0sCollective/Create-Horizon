# Current work

The latest owner resume authorizes development to a genuine blocker or required
input, superseding the old cutoff/break. No public publication, promotion or
standing team is authorized. Keep source-informed personal-use provenance and R01.
Require fresh independent read-only Sol High review for substantive candidates.

- Private branch: `codex/h02-smoke-h03-notes`; checkpoints backed up privately.
- Engine branch: `codex/h08-score-metadata`, based on reviewed/tested local commit
  `c161eaa1f361cd4e423c4cb07120c2f9c1d529ab`, tree
  `5c1cc7723d4ee5ea7f53906f4ea3d65f53b3c6b0`. Bounded source worker owns only
  game.rs, game/scoreboard.rs, command_storage.rs and stopwatches.rs for H08a6.
  Coordinator owns native Cargo and validation; no worker Cargo or commits.
- Engine commits stay local; normal hooks enabled, upstream push disabled.
- Create reference stays clean detached `7ff2760e4924d35d4bc2e22c4b19923f0178cd16`.
  Pins are in workspace.toml. Never alter Prism/playable saves/reference snapshots.

Completed bounded foundations: H05a transient counter and H08a1-a5 save
prerequisites have source review, focused tests, real games in both modes and
broad quick verification. Latest frozen game SHA256
`a8570b081c3c754f1b5760eab52b24c8f0ff3e26f03d88033955d9640a458cbf` passes:

- Integrated `1e1b15fb-540d-4f05-907b-0602b5cf3d45`: 362 assertions.
- In-frame `91f3d9c2-14bb-4e33-b92d-e47404819683`: 347 assertions.

Same qualified driver `tools/run_counter_game_smoke.py`, SHA256
`d19c480572c3fbcb0ce01afeb868efb5dc276ad64606ab01f40edd9bf2d970e0`.
Counter remains transient; loading its saved block does not restore payload.
See H05_COUNTER_RESULTS and H08_SAVE_COMPLETION/PROTO_SAVE_ERRORS/
FINISHED_CHUNK_HANDOFF/STRICT_POI_WRITER_RESULTS in docs/reference. They preserve
failed preflight/calibration, red raw-NBT test, precise refusal cleanup races and
all unqualified limits. Latest quick check passes 369 Python/1,311 client tests
plus format/lint/doc/public-tree; H08a5 adds seven for 1,318 client tests. Vanilla
parity is explicitly skipped.

Separate focused vanilla 26.3 reader run `ad33ad7f-f997-4f2c-832c-f220e6d7733b`
passes on frozen a75184e/325e1f53..., proving this chest23/pig fixture's shared
save format reads correctly in actual Java25/Minecraft. Both processes exit0,
no helper-filtered problems or cleanup errors. See H08_VANILLA_SAVE_READBACK_RESULTS.
It is not full parity, out-of-range callback execution or coherent recovery.

H08a5: `docs/design/H08_MAP_RANDOM_METADATA_PACKET.md`. Reviewed
implementation covers map/index/random metadata error propagation and immutable RNG
snapshot including local/LAN merchant loans. Identical duplicate streams may
merge; conflicting states must refuse before the random-file write. No runtime
loan ownership/reset-policy repair, menu closing or new schema. Fresh source review,
144 binary tests (including seven new cases), RNG regression and on/off Clippy pass.
Frozen game a8570b08... passes 362/347 counter checks in runs
1e1b15fb-540d-4f05-907b-0602b5cf3d45 and 91f3d9c2-14bb-4e33-b92d-e47404819683;
normal initial/restart exits are clean. Independent receipt review agrees. Broad
quick checks pass 369 Python/1,318 client tests plus fmt/lint/doc/public-tree in
the immutable native clone; parity explicitly skipped. See
`docs/reference/H08_MAP_RANDOM_METADATA_RESULTS.md`.

Active packet: `docs/design/H08_SCORE_METADATA_PACKET.md` (H08a6), bounded to
scoreboard/storage/stopwatch/border errors, retryable flags and default-border
overwrite. Candidate edits are not yet reviewed or validated. Other metadata and player/native-entity retirement remain
open. Next scope must stay bounded; see H08_CHECKPOINT_FEASIBILITY for the frozen
owner/writer audit, including why tick-freeze/server-hold alone is insufficient.

Coordinator owns the native Cargo target. Serialize every Cargo operation and
its test execution. Use native Linux clones for broad CI; never export
GIT_DIR/GIT_WORK_TREE into tests. Current clone path is in .local/h05-ci-checkout.txt.
Clear HORIZON_SESSION_FILE in every test child environment; invoke immutable older
smoke drivers through an environment-sanitizing wrapper. Do not modify their
qualified bytes. Fresh independent Sol High review cleared this restriction and
the readback-driver guard; current Windows/WSL presence checks are both false.
All runtime worlds/homes/launcher/XDG folders stay fresh and isolated; generated
JARs/assets/worlds/raw evidence never enter Git.

Full H02/H03/H04/H05, saved generations, guest callbacks, staged transactions,
inventories, coherent checkpoints, external paired networking, retained animation,
performance and G1/G2/G3 remain open. Earlier S0/P02/B0 and R01 evidence stays in
its result records; this work does not promote those gates.
