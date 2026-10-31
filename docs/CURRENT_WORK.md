# Current work

Owner resumed work until S1 is complete. Continue with periodic updates; stop only
for a genuine blocker or required owner decision. Earlier pause/cutoff superseded.
Existing scope, reference pins and publication gates remain.

## Completed and accepted

Native checkpoint 872dac2, managed authority 9388598, and managed-stop worker
23e09dd1f0ec3c2a0a69e6e002ddab4a92736b11 are independently FINAL accepted.
See docs/reference/S1_SESSION_AUTHORITY_RESULTS.md and
S1_MANAGED_STOP_WORKER_RESULTS.md for exact source and evidence identities.
Managed stop: 203 ordinary + 2 actual-worker resource cases; separate expected
SIGABRT proof; optimized typecheck only; exact clean broad 369 Python / 1375 client /
0 doc failures. Earlier failed builds and source/log snapshots remain preserved.
Old s1ms1 dirty-base scripts MUST NOT run against committed 23e09dd.
Engine commits remain LOCAL; no public push, PR, merge or release.

## Active: connected recovery

Accepted design input b2adefe56107c05a412cfa162cee1e3b26f64a58cb08979d18ba58b8e4cde456:
docs/design/S1_CONNECTED_RECOVERY_PROPOSAL.md. Private checkpoint 49f4753 pushed.
Worktree .local/worktrees/s1-connected-recovery, branch codex/s1-connected-recovery,
base 23e09dd. Exactly eight source paths, all under crates/horizon-client/src/:

- Root: game/lan.rs, new game/lan/connected_recovery.rs,
  new game/lan/connected_recovery_tests.rs, game/levels/session_checkpoint.rs.
- Bounded transport author: lan/mod.rs and lan/conn.rs.
- Bounded hub author: lan/hub.rs.
- Root: crates/horizon-protocol/src/connection.rs close-only PipeShutdown capability.

Initial all-source/prefix snapshot .local/s1cr1-attempt0. Before every correction,
preserve ALL eight paths and ALL s1cr1 prefix evidence in a new named snapshot.
Authors edit assigned native files only; no runtime, formatting, Git writes,
network, escalation or further delegation. Root alone formats and serializes all
Cargo/tests/game processes. Exact source requires fresh independent Sol High review
before compilation/runtime; a different final reviewer assesses complete evidence.
Reviewers are procedurally read-only in a write-capable runtime: native reads,
Git read, hashes and text inspection only; no scripts/imports/WSL/runtime/writes.

Implement actual two-client offline memory joins, verified nonzero PLAYER inventory,
sealed unplaced-roster installation before normal placement, genuine terrain and
startup acknowledgements, no recovery ticks, owned ReadyFrozen recapture/close and
finite pre-Ready cleanup of all three possible connection attempts. Ordinary paths
retain their behavior. Native memory-pipe buffering is unbounded; staged mailbox and
body budgets do not establish total memory bounds. Late departure and fatal misuse
are separate isolated exception proofs, never normal-close successes.
Native-close design delta independently ACCEPTED at proposal4 d2e9f38d45cb139aad639afb96076a784def5e593559bbe2db89207a07caa213. Initial timeout workaround was rejected before compilation and preserved. attempt3/4/5 contain source/prefix snapshots. Source and resource/exception tests are drafted; exact source review is next. No connected-recovery compile/runtime validation has run yet.

## Stage and operating boundaries

S1/G1 remains OPEN: ordinary lifecycle/UI/process-exit routing, connected recovery,
positive roster and in-frame ownership, authorized interaction/departure handling,
live admission/replication and committed presentation; then integrated and dedicated
with TWO exact clients and promised in-frame checks. This packet is one prerequisite.
Live S2 requires G1. S3 exactly motor/shaft/small and large cogs/millstone,
cobblestone to gravel; no gearbox. R01 matches the pinned Java full-output item loss.
No broad journal/global codec/reduced S1 or save compatibility promise adopted.

Private branch codex/h02-smoke-h03-notes. Every private push requires fresh GitHub
PRIVATE metadata and exact origin https://github.com/Cha0sCollective/Create-Horizon.git.
Normal Git hooks and agent footer apply. Preserve isolated runtime data, all source
provenance, Prism/playable data/accounts/reference snapshots/global Java. No memory
writes. Coordinator implements directly; only bounded useful delegation.
