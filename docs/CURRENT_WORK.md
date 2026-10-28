# Current work

The latest owner resume authorizes development to a genuine blocker or required
input, superseding the old cutoff/break. No public publication, promotion or
standing team is authorized. Keep source-informed personal-use provenance and R01.
Require fresh independent read-only Sol High review for substantive candidates.

- Private branch: `codex/h02-smoke-h03-notes`; checkpoints backed up privately.
- Engine branch: `codex/h08-bars-schedule-metadata`, based on reviewed/tested local commit
  `1d144d017b7d238293353e5c25c7f21080da4944`, tree
  `aa4b44f95ccef1f73d023ee86b22c1255ba3e0f1`. H08a7 source worker owns only
  game.rs, game/bossbars.rs and game/functions.rs. Coordinator owns native Cargo,
  validation and commits; no worker Cargo or commits.
- Engine commits stay local; normal hooks enabled, upstream push disabled.
- Create reference stays clean detached `7ff2760e4924d35d4bc2e22c4b19923f0178cd16`.
  Pins are in workspace.toml. Never alter Prism/playable saves/reference snapshots.

Completed bounded foundations: H05a transient counter and H08a1-a6 save
prerequisites have source review, focused tests, real games in both modes and
broad quick verification. Latest frozen game SHA256
`d86ad4e265c672ec8c79808d60ae90cac9cd9e8df82eab1b2f86a80a9103932e` passes:

- Integrated `640a3687-dbbe-4e21-9c9e-1550ac5f9854`: 362 assertions.
- In-frame `8bf723e6-8b64-4a97-9309-917508e7f098`: 347 assertions.

Same qualified driver `tools/run_counter_game_smoke.py`, SHA256
`d19c480572c3fbcb0ce01afeb868efb5dc276ad64606ab01f40edd9bf2d970e0`.
Counter remains transient; loading its saved block does not restore payload.
See H05_COUNTER_RESULTS and H08_SAVE_COMPLETION/PROTO_SAVE_ERRORS/
FINISHED_CHUNK_HANDOFF/STRICT_POI_WRITER_RESULTS in docs/reference. They preserve
failed preflight/calibration, red raw-NBT test, precise refusal cleanup races and
all unqualified limits. Latest quick check passes 369 Python/1,325 client tests
plus format/lint/doc/public-tree. Vanilla parity is explicitly skipped.

Separate focused vanilla 26.3 reader run `ad33ad7f-f997-4f2c-832c-f220e6d7733b`
passes on frozen a75184e/325e1f53..., proving this chest23/pig fixture's shared
save format reads correctly in actual Java25/Minecraft. Both processes exit0,
no helper-filtered problems or cleanup errors. See H08_VANILLA_SAVE_READBACK_RESULTS.
It is not full parity, out-of-range callback execution or coherent recovery.

H08a5 map/random results preserve immutable local/LAN loan snapshots and refuse
conflicting stream states without choosing an ownership winner. H08a6 scoreboard
results cover storage/stopwatch dirty retention, all four component write errors
and default-border reset. Both have independent source/receipt review and targeted,
both-mode and broad verification. See their result files. Metadata remains
nonatomic; live loan/reset policy and other metadata/retirement gaps remain open.
H08_CHECKPOINT_FEASIBILITY records direct dirty-marker and raid transition gaps,
plus the frozen-owner audit: tick-freeze/server-hold alone cannot guard capture.

Active packet: `docs/design/H08_BARS_SCHEDULE_METADATA_PACKET.md` (H08a7). Propagate
bossbar/schedule write errors while preserving live borrowed state, guards, ordering,
empty reset and sparse defaults. Source edits are not yet reviewed/validated.

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
