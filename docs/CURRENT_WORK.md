# Current work

The latest owner resume authorizes development to a genuine blocker or required
input, superseding the earlier cutoff/break. No public publication, promotion or
standing team is authorized. Require fresh independent Sol High review for each
substantive candidate. Preserve source-informed personal-use provenance and R01.

- Private branch `codex/h02-smoke-h03-notes`; checkpoints backed up privately.
- Engine `codex/h08-finished-chunk-handoff` based at
  `928f43c2db203049c6266baf3682b16a806f028a`, tree
  `a0d9537b4e044716318ce4c00f4b319a45175ca3`; one bounded worker now owns three
  finished-chunk handoff files. H08a2 is independently reviewed and passes targeted
  checks, both actual modes and broad quick verification. H08a1
  completed all checks at `1e87673`.
  The qualified counter branch remains `codex/h05-counter-integration`.
  Commits remain local only;
  normal hooks enabled, upstream push disabled.
- Create reference clean detached `7ff2760e4924d35d4bc2e22c4b19923f0178cd16`.
  Pins remain in workspace.toml. Never change Prism/playable saves or references.

H05a transient native counter is independently source/receipt reviewed and
qualified in both actual game modes. Frozen game SHA256
`92a1e554b9dee74150c39ddeb40918c8c65b0f0aac9ef1ab5dad44268f0c19a7`:

- Integrated `ad45af86-dd8c-422b-842c-800cf527eed2`: 362 assertions.
- In-frame `a09f3b77-0945-41ff-a16e-5b380d8f3662`: 347 assertions.

Reviewed driver `tools/run_counter_game_smoke.py` SHA256
`d19c480572c3fbcb0ce01afeb868efb5dc276ad64606ab01f40edd9bf2d970e0`.
Actual placement, exact steps, stale keys, pause/freeze, ticket-only suspension,
dormancy/reload, parked retention versus replacement, final retirement and loaded
cube restart-disabled pass. All inherited paired/state/mutation/piston/SDK/
dimension/residency/admission controls pass. Initial/restart exit 0; intentional
admission refusals exit 1; all cleanup errors null. See
`docs/reference/H05_COUNTER_RESULTS.md` for identities, limits and excluded failed
preflight/calibration. Normal block saving is not counter persistence.

Core `8cff24d` passes focused native/regression/resource tests and feature-on/off
library/test Clippy. Game `28ded16` passes 137 binary tests (three ignored),
feature-on/off binary Clippy and fmt. Broader `pr_ready.py --quick --base 0264c18...`
passes from a fresh native Linux clone: 369 Python, 1,381 Rust and 197 launcher
tests plus formatting/lints/public tree. Parity is explicitly skipped. Its resource
tests lacked prepared data; eight explicit paired-on owned-resource tests now
pass. `.local/h05-pr-ready.log` and `.local/h05-owned-resource-tests.log` are raw
records. No Cargo by workers while the coordinator owns the native target. Serialize all
Cargo including test execution; never export GIT_DIR/GIT_WORK_TREE into tests.

Completed H08a1 packet is `docs/design/H08_SAVE_COMPLETION_PACKET.md`: honest
synchronous generation-save completion, a prerequisite from
`docs/design/H08_CHECKPOINT_FEASIBILITY.md`.
Timeout/disconnected workers must not silently report success or consume the
remaining dirty snapshots. This is preparation, not coherent persistence. No
worker may run Cargo until the coordinator releases the target. Three controlled
failure tests, three generation-save tests, 33 disk-save tests, 137 binary tests
and feature-on/off Clippy pass. New frozen game `5b830110...3f42e9` passes integrated
362/in-frame 347 full assertions; broad quick checks pass 369 Python/1,307 client
tests plus format/Clippy/doc/public-tree checks (parity skipped). Independent receipt review clears both, noting
one expected refusal cleanup race accurately. See
`docs/reference/H08_SAVE_COMPLETION_RESULTS.md`; this is not a checkpoint barrier.

Completed H08a2 packet `docs/design/H08_PROTO_SAVE_ERRORS_PACKET.md` changes only native
proto callback errors/ownership with a sticky fault that stops new terrain producer
jobs while draining old results. Reviewed three-file change passes six controlled
resource groups, three generation-save tests, 33 disk-save tests, 137 binary tests
and on/off Clippy; frozen game `66a51ee6...21fc81d` passes integrated 362/in-frame
347 assertions with independent receipt review; broad checks pass 369 Python/
1,307 client tests and format/lint/doc/public-tree checks (parity skipped). See
`docs/reference/H08_PROTO_SAVE_ERRORS_RESULTS.md`. Active next packet is
`docs/design/H08_FINISHED_CHUNK_HANDOFF_PACKET.md`: worker owns streaming.rs,
save.rs and generation_saves.rs, source only, no Cargo/formatting/commits. Its
draft is unreviewed/unqualified.
No player/native-entity-unload/checkpoint expansion.

Earlier qualified residency proof: a4470cd, integrated 228/in-frame 212; see
`docs/reference/H04_RESIDENCY_GAME_RESULTS.md`. The in-frame lighting failure and
reviewed fix remain accurately recorded. Other S0/P02/B0, H02/H03/H04 and PR1189
reconciliation evidence remain in their result/design records.

Full H02/H03/H04/H05, saved generations, guest callbacks, transactions/inventories,
coherent recovery, external paired networking, retained animation, performance and
G1/G2/G3 stay open. Dedicated/idle/save-error retirement is source-reviewed only.
