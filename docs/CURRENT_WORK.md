# Current work

The latest owner resume authorizes development to a genuine blocker or required
input, superseding the old cutoff/break. No public publication, promotion or
standing team is authorized. Keep source-informed personal-use provenance and R01.
Require fresh independent read-only Sol High review for substantive candidates.

- Private branch: codex/h02-smoke-h03-notes; checkpoints backed up privately.
- Engine branch: codex/h08-borrowed-dirty-metadata, based on qualified local commit
  0028c89d31438d159ee088eafe14827ba6c76488, tree
  b24c06a7221e56818f04b1c9765bb70dcf7bb2ad. H08a10 source worker owns only game.rs,
  game/dragon.rs, game/wandering_trader.rs and game/nether_portal.rs. No worker Cargo.
- Coordinator owns serialized Cargo, validation and commits. Engine commits stay
  local; normal hooks enabled, upstream push disabled. No public engine push.
- Create reference stays clean detached 7ff2760e4924d35d4bc2e22c4b19923f0178cd16.
  Pins are in workspace.toml. Never alter Prism/playable saves/reference snapshots.

Completed bounded foundations: H05a transient counter and H08a1-a9 save
prerequisites have independent source/receipt review, focused tests, actual games
and broad quick checks. Latest frozen game SHA256
00d38a8a24efc2ab7e78568e6c9348b59f01067c14d6d4e5c6c4c08551e0852e passes:

- Integrated 64ec267b-cbc6-418a-833a-6967ea1dddef: 362 counter checks.
- In-frame 19c8af4d-c1c4-4b07-8d24-ead875940edf: 347 counter checks.

Qualified tools/run_counter_game_smoke.py SHA256
d19c480572c3fbcb0ce01afeb868efb5dc276ad64606ab01f40edd9bf2d970e0 stays immutable.
Normal initial/restart exits are clean; precise expected-refusal cleanup races
remain in the result notes. Counter payload stays transient. Latest broad quick
passes 369 Python/1,339 client tests plus fmt/lint/doc/public-tree; parity skipped.
Scope has a disclosed soft warning for clock tests/mechanical moves; no PR proposed.

H08a9 focused actual clock-error runs pass with no cleanup errors:

- Integrated 152e9ad3-6f1d-47b1-bfde-9ef7f2c42212: 19 checks, repair/exit0.
- In-frame 96dd95e0-ecb6-4315-b9ac-b1625ebdd5f6: 21 checks, natural1, reopen/exit0.

Qualified tools/run_clock_save_error_smoke.py SHA256
e29e1b9d823a077be0172cac4b4a296e33c3643059f54778227f3c80125f5f30 stays immutable.
Preserve failed071648 startup2de93d5d...: scratch XDG_RUNTIME_DIR hid WSLg before
world readiness/injection. Reviewed correction keeps the display-service runtime
connection and fresh save/data/config/cache homes. Original failed and corrected
status records remain separate. See H08_WORLD_CLOCK_METADATA_RESULTS.

Earlier results preserve actual queue faults, raw-NBT red-test correction, strict
POI reads, RNG loan-conflict refusal and nonatomic metadata limits. See H05_COUNTER
and H08_*_RESULTS in docs/reference. Focused actual Java26.3 chest/pig readback on
older a75184e/325e1f53... passes in H08_VANILLA_SAVE_READBACK_RESULTS; not full parity.

Active packet: docs/design/H08_BORROWED_DIRTY_METADATA_PACKET.md (H08a10).
Propagate borrowed fight/trader/portal-arrival write errors; retain dirty state and
schemas/guards. Eager dragon-death failure logs and retries through later saves,
without aborting already committed gameplay. Source not yet reviewed/validated.
H08_CHECKPOINT_FEASIBILITY records separate fight-world lifetime, raid, portal,
unload and player/entity ownership gaps. Tick-freeze/server-hold alone cannot
protect a checkpoint capture. No coherent checkpoint or G1 is qualified.

Serialize every Cargo operation including test execution. Use native Linux clones
for broad CI; never export GIT_DIR/GIT_WORK_TREE into tests. Clone path remains in
.local/h05-ci-checkout.txt. Clear HORIZON_SESSION_FILE in every test child; invoke
immutable older drivers through a sanitizing wrapper. Reviewed new drivers also
clear it internally. Keep game/launcher/data/config/cache homes fresh and isolated;
leave the reviewed display-service runtime connection intact. No JARs, assets,
worlds, account data or raw generated evidence enter Git.

Full H02/H03/H04/H05, persistent generations, guest callbacks, transactions,
inventories, coherent recovery, external paired networking, retained animation,
performance and G1/G2/G3 remain open. S0/P02/B0 and R01 evidence stays in its records.
