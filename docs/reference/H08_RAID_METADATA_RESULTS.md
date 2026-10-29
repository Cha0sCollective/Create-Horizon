# H08a14 checked raid metadata save results

Local engine candidate b3e0d3561a8849c03cd19012ed42c4710b7ec0fe, tree
822c230744135df8a63195a5b0bbcdcc029f8c31, based on accepted H06d
62756231205b1b2b65fcf92c180fc29497f29715. The normal public-tree commit hook
passes; no public engine push. Six files change +317/-16 (333 changed lines),
including a 291-line resource-backed actual-Game test module.

save_raids now returns contextual write failure while retaining the native
format, dimension path, unloaded/clean/remote guards and success-only dirty clear.
The existing in-memory omen projection occurs after either attempted-write result.
Synchronous save_level still flushes previously queued region/entity work and
returns the earlier raid error if both operations fail. Incremental saving reports
the error before metadata/Stage::Writes and retains its pending-save object.

The lazy load_raids caller now logs the error explicitly but keeps its existing
replacement policy. Loss of prior dimension raid state on that lazy error remains
UNFIXED. An early return alone would expose old-dimension state to new-dimension
callers. This packet qualifies strict save error reporting, not general raid
lifetime, outer shutdown/error lifetime or coherent checkpoint recovery.

Fresh independent read-only Sol High design/source reviews find no blocking
issues. Existing shared loader, explicit Options and earlier test bodies remain
unchanged. Only cfg(test) module/Fixture/new/game visibility is widened so the new
raid child module can access actual Game construction while native fields stay private.

Frozen precommit test artifact SHA256:
f544f30bb0cb379f4e269e8d1fb451a6f02221e0d009fa26e8f781304d7ac92b.
Build and runtime receipts bind all six formatted source hashes. This is not a
clean-commit test-binary claim. Isolated execution checks source/binary/resource
hashes and all five exact names, creates fresh cwd/HOME/data/config/cache/temp,
removes the session and preserves inherited /run/user/1000. Five pass, zero ignored,
exit 0, no timeout, in 13.93 seconds:

- Real raids.dat_new obstruction returns its canonical path, keeps exact nonempty
  native raid data and dirty state, and still projects omen metadata. Repair/retry
  writes the expected current Tag and clears dirty.
- Unloaded, clean and actual memory-client guards write nothing.
- Actual save_level(true) returns the raid error while a changed chunk and nonempty
  native entity reach disk. A NEW SaveDir reader excludes pending/cache false passes.
- Separate simultaneous raid/entity write obstruction returns the earlier raid
  error and retains the writer's sticky failure. No repair success is fabricated.
- Actual begin_control_save/poll_closing_save emits an error and retains pending_save;
  repairing the raid-only obstruction permits completion. The consumed error channel
  is not relabelled a later success acknowledgement.

Independent read-only resource receipt review confirms these claims and unchanged
source hashes. Native validation passes: 173 ordinary binary tests (26 ignored),
all three feature lint configurations, formatting and public-tree checks. The five
explicit raid cases are a separate execution, not silently counted among ignored
cases. Unchanged prior resource cases are not new executions.

Parity selection reports 356 dependent harnesses/355 selected, with the whole LAN
integration suite left named-only. This was --list inspection only: no vanilla
parity harness execution is claimed.

New clean-commit frozen Game SHA256:
01a55d6ec222c0a389f97235ac647d0c430c9cfa35bbeafa7c0f1e47606646cb.
The fast-profile build records exact commit/tree and pinned Rust/features. Both
full counter suites use the immutable stable SDK-target wrapper and fresh owned
homes, component copies and worlds, with session removed:

- Integrated 0814a185-60e7-4f98-b441-3bc96a7d1be4: 368 checks, final counter 65.
- In-frame 75395689-ae25-43d3-9153-07ac0aaa0b0f: 353 checks, final counter 27.

Both results pass and bind that same Game/entrypoint hash. Initial and restart
processes exit 0 with null cleanup error in both modes. All four changed/missing
package refusals exit 1 as intended; integrated changed-package cleanup records
an already-exited-game quit race. The other refusal cleanup errors are null.
Fresh environment/build/exit/fixture/result receipts remain at their recorded raw
paths. These are new normal-path regressions, not actual in-game raid-fault injection;
the separate five headless Game tests supply that focused fault coverage.

Broad --quick passes at the same candidate: 369 Python tests, 1,357 client tests,
zero doc tests, fmt/Clippy/public-tree and unchanged locks. Scope check passes at
six files/+317/-16. Content-root/theme/launcher are skipped as untouched; parity
is explicitly skipped. Composite counter/quick exit codes are both 0.
Broad log SHA256:
e33067893c1d733010d551135546881578dba2e96eb460fd29b3d4fd76fea44f.
Final independent read-only receipt follow-up finds no blocking issues, confirms
the exact source/commit/tree, native/hook and both-mode evidence plus checkout-bound
broad results. The bounded strict raid writer prerequisite is accepted.
Raw receipts, logs, owned worlds and binaries stay outside Git at workspace-root
.local/h08a14-* and their recorded native scratch paths.

This precursor establishes neither coherent cross-owner rollback/publication,
active external-player capture, roster history, off-resident entity coverage,
all-owner teardown, player replay/authority, machine persistence, Create conformance,
retained animation, performance qualification nor G1. See H08_CHECKPOINT_FEASIBILITY
and the next H08_PARTICIPANT_PREFLIGHT_PACKET for remaining bounded work.
