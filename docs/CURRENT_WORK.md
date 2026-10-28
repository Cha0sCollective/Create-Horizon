# Current work

The latest owner resume authorizes development to a genuine blocker or required
input, superseding the old cutoff/break. No public publication, promotion or
standing team is authorized. Keep source-informed personal-use provenance and R01.
Require fresh independent read-only Sol High review for substantive candidates.

- Private branch: `codex/h02-smoke-h03-notes`; checkpoints backed up privately.
- Engine branch: `codex/h08-map-random-metadata`, reviewed/tested local commit
  `c161eaa1f361cd4e423c4cb07120c2f9c1d529ab`, tree
  `5c1cc7723d4ee5ea7f53906f4ea3d65f53b3c6b0`, clean. Source worker released files;
  coordinator owns the optimized frozen build and validation. No worker Cargo.
- Engine commits stay local; normal hooks enabled, upstream push disabled.
- Create reference stays clean detached `7ff2760e4924d35d4bc2e22c4b19923f0178cd16`.
  Pins are in workspace.toml. Never alter Prism/playable saves/reference snapshots.

Completed bounded foundations: H05a transient counter and H08a1/a2/a3/a4 save
prerequisites have source review, focused tests, real games in both modes and
broad quick verification. Latest frozen game SHA256
`f83f449607ba6df59f1fe73379f66f576cbd1b94c765a2f8ad084725148e80c7` passes:

- Integrated `dc4d7411-897d-4aab-b914-820eee24a36c`: 362 assertions.
- In-frame `d94885cd-f3be-41d7-8853-9b4ee60fc0ef`: 347 assertions.

Same qualified driver `tools/run_counter_game_smoke.py`, SHA256
`d19c480572c3fbcb0ce01afeb868efb5dc276ad64606ab01f40edd9bf2d970e0`.
Counter remains transient; loading its saved block does not restore payload.
See H05_COUNTER_RESULTS and H08_SAVE_COMPLETION/PROTO_SAVE_ERRORS/
FINISHED_CHUNK_HANDOFF/STRICT_POI_WRITER_RESULTS in docs/reference. They preserve
failed preflight/calibration, red raw-NBT test, precise refusal cleanup races and
all unqualified limits. Latest quick check passes 369 Python/1,311 client tests
plus format/lint/doc/public-tree; vanilla parity is explicitly skipped.

Separate focused vanilla 26.3 reader run `ad33ad7f-f997-4f2c-832c-f220e6d7733b`
passes on frozen a75184e/325e1f53..., proving this chest23/pig fixture's shared
save format reads correctly in actual Java25/Minecraft. Both processes exit0,
no helper-filtered problems or cleanup errors. See H08_VANILLA_SAVE_READBACK_RESULTS.
It is not full parity, out-of-range callback execution or coherent recovery.

Active packet: `docs/design/H08_MAP_RANDOM_METADATA_PACKET.md` (H08a5). Source-only
implementation covers map/index/random metadata error propagation and immutable RNG
snapshot including local/LAN merchant loans. Identical duplicate streams may
merge; conflicting states must refuse before the random-file write. No runtime
loan ownership/reset-policy repair, menu closing or new schema. Fresh source review,
144 binary tests (including seven new cases), RNG regression and on/off Clippy pass.
Frozen game a8570b08... passes 362/347 counter checks in runs
1e1b15fb-540d-4f05-907b-0602b5cf3d45 and 91f3d9c2-14bb-4e33-b92d-e47404819683;
normal initial/restart exits are clean. Broad quick checks remain pending. See
`docs/reference/H08_MAP_RANDOM_METADATA_RESULTS.md`. Other metadata and player/native-entity retirement remain
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
