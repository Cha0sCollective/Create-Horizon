# Current work

The latest owner resume authorizes development to a genuine blocker or required
input, superseding the old cutoff/break. No public publication, promotion or
standing team is authorized. Keep source-informed personal-use provenance and R01.
Require fresh independent read-only Sol High review for substantive candidates.

- Private branch: `codex/h02-smoke-h03-notes`; checkpoints backed up privately.
- Engine branch: `codex/h08-world-clock-metadata`, based on reviewed/tested local commit
  `b9b32da57e124221110c82b28a3fd95e0624cf4a`, tree
  `42f0d4b50b0bc7f5ae5ce4750c3abec164fdde96`. H08a9 source worker owns only
  save.rs, game/chat.rs and game.rs. Harness worker released the new private
  tools/run_clock_save_error_smoke.py; coordinator owns it and serialized Cargo/commits.
- Engine commits stay local; normal hooks enabled, upstream push disabled.
- Create reference stays clean detached `7ff2760e4924d35d4bc2e22c4b19923f0178cd16`.
  Pins are in workspace.toml. Never alter Prism/playable saves/reference snapshots.

Completed bounded foundations: H05a transient counter and H08a1-a8 save
prerequisites have source review, focused tests, real games in both modes and
broad quick verification. Latest frozen game SHA256
`5691d4efaa5e24157fdb7f38d7e373670ba91788fa4dc8a38a2fb23943b6b0b5` passes:

- Integrated `64c7d520-1088-41dc-993a-160c1bcdf91a`: 362 assertions.
- In-frame `57ad47e3-06fe-4fb1-892c-aa51d8599252`: 347 assertions.

Same qualified driver `tools/run_counter_game_smoke.py`, SHA256
`d19c480572c3fbcb0ce01afeb868efb5dc276ad64606ab01f40edd9bf2d970e0`.
Counter remains transient; loading its saved block does not restore payload.
See H05_COUNTER_RESULTS and H08_SAVE_COMPLETION/PROTO_SAVE_ERRORS/
FINISHED_CHUNK_HANDOFF/STRICT_POI_WRITER_RESULTS in docs/reference. They preserve
failed preflight/calibration, red raw-NBT test, precise refusal cleanup races and
all unqualified limits. Latest quick check passes 369 Python/1,331 client tests
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

H08a7: `docs/design/H08_BARS_SCHEDULE_METADATA_PACKET.md`. Propagate
bossbar/schedule write errors while preserving live borrowed state, guards, ordering,
empty reset and sparse defaults. Fresh independent source review and targeted
checks pass (153 binary tests, three ignored; feature-on/off lint/fmt/public-tree).
The parser move preserves existing load/reload policy. Frozen game 9421b04a... passes
362/347 counter checks; normal initial/restart exits are clean. Independent receipt
follow-up agrees; broad quick checks pass (369 Python/1,331 client and fmt/lint/doc/
public-tree; parity skipped). See H08_BARS_SCHEDULE_METADATA_RESULTS.

H08a8: `docs/design/H08_DIRTY_HANDOFF_PACKET.md` has fresh read-only
design review. Retain direct-save dirty markers through chunk enqueue, clear only
submitted coordinates on acceptance, and distinguish real queue/partial-POI errors
from asynchronous disk failure. Fresh independent code review finds no issue in
the exact candidate and five new owned-resource tests. All targeted checks pass:
8 explicit resource-save/37 regular save/3 generation/153 bin tests, on/off lint,
fmt/public-tree. Frozen game 5691d4ef... passes 362/347 counter checks with clean
normal initial/restart exits. Independent receipt follow-up agrees; broad quick
passes 369 Python/1,331 client plus fmt/lint/doc/public-tree (parity skipped). See
H08_DIRTY_HANDOFF_RESULTS.

Active packet: `docs/design/H08_WORLD_CLOCK_METADATA_PACKET.md` (H08a9), shared
strict clock merge for both Level/Game writers and explicit error propagation,
plus a separately reviewed isolated game-error harness. Fresh initial design review
passes. Harness source SHA071648b1... passes fresh static security review, with
base/NBT/control hashes recorded by that review; no imports/games executed.
Engine source/cross-boundary review and all runtime validation remain pending.
Transition/unload/player/entity/raid ownership stays separate.

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
