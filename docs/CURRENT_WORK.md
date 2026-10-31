# Current work

Owner resumed work until S1 is complete, with periodic updates; stop only for a
genuine blocker or required owner decision. Earlier pauses/cutoff superseded.
Existing scope, provenance, stage and publication gates remain.

## Completed and accepted

Native checkpoint 872dac2, managed authority 9388598, managed-stop worker
23e09dd1f0ec3c2a0a69e6e002ddab4a92736b11 independently FINAL accepted. See
reference/S1_SESSION_AUTHORITY_RESULTS.md and S1_MANAGED_STOP_WORKER_RESULTS.md.
Managed stop: 203 ordinary +2 actual-worker cases; separate expected SIGABRT;
optimized typecheck only; exact clean broad 369 Python/1375 client/0 doc failures.
Prior failed builds and snapshots preserved. Old s1ms1 dirty-base scripts MUST NOT
run against committed23e09dd. Engine remains LOCAL: no public push/PR/merge/release.

## Active: connected recovery validation

Worktree .local/worktrees/s1-connected-recovery, branch codex/s1-connected-recovery,
BASE23e09dd. Exactly EIGHT paths under engine:
- crates/horizon-client/src/game/lan.rs
- crates/horizon-client/src/game/lan/connected_recovery.rs (new)
- crates/horizon-client/src/game/lan/connected_recovery_tests.rs (new)
- crates/horizon-client/src/game/levels/session_checkpoint.rs
- crates/horizon-client/src/lan/mod.rs, conn.rs and hub.rs
- crates/horizon-protocol/src/connection.rs

Accepted design docs/design/S1_CONNECTED_RECOVERY_PROPOSAL.md inputb2adefe5;
close-only PipeShutdown eighth-path delta independently accepted inputd2e9f38d.
Design backup930fa07 pushed privately. Initial timeout polling could not interrupt
a trickled partial frame; preserved/replaced before compile. No pipe-memory or
uniform close-latency claim. Exact source SOURCE CLEAR at manifest
c11b8b2f0465fb3b5b1c8dfa66ca5df6789f2cdc9ba38c966599217c4b272c5b.
Fresh independent source, positive-tool and exception-tool reviewers are separate.

Root implemented core/coordinator/tests/protocol; bounded authors finished transport
and Hub. All authors released. Source review corrections include namespace route
separation, actual retained keys/players/GuestPlay/core observations, actual client
readiness, genuine Joined barrier, missing-profile deadline, publication retries
and nine real native queue/body-accounting tests. No synthetic Guest/readiness.

Validation16249 FINISHED0: fmt, three feature clippies, public tree PASS. Earlier
24188 deprecated API,26887 MSRV conflict,94170 native-part move/private-test-helper
compile failures preserved; corrected and independently source-cleared. Test build
62724 RUNNING through reviewed s1cr1-build-tests.sh. NO tests/game pass claimed yet.
Tools .local/s1cr1-*; reviewed builderf74ca6e7, guard6ec18719, positive runnerbd785ef3,
exception orchestrator873332d8. Suite13 actual-resource cases +9 library accounting
cases +full protocol suite including3 new close-capability tests;203 bin ordinary.
FOUR exception children are separate and never positive test counts. Current source
and suite bindings refreshed; source guard enforces exact8 paths/base/branch.

Before every correction preserve ALL EIGHT sources and ALL s1cr1 prefix evidence.
Snapshotsattempt0-12 retained. Root alone formats and serializes all Cargo/tests/game
processes. Build freezes actual bin/lib/protocol executables before isolated runs;
no nested compiler/guest/test process in Rust. Python proof tools use -I, reject
optimized assertions; child contexts isolate cwd/home/config/data/cache/temp/game/
launcher/trash/clipboard/desktop and clear Git/session/RUST_MIN_STACK/fatal flags.
Typed post-copy/post-rename faults are not OS IO/power-loss durability evidence.

## Next bounded packet (no source release yet)

S1_MANAGED_CONNECTED_WORKER_BRIDGE_PROPOSAL.md independently DESIGN ACCEPT at input
026d3415869a27f7b1726f16b1724adac1d51cc1a6a0749cd680ab1e46432f42, snapshot
.local/s1cw1-proposal2.md. Six-path worker bridge only, conditional on current FINAL,
clean engine commit, private checkpoint and successor pin. Keep recover_connected
wholly on worker; forward one-shot pipes/coalesced status/typed close-retry only.
No Game escape or freely droppable owner. Outer ordinary UI/process holder is the
following separate packet. Do not stop owner-authorized S1 work at a packet gate.

## Stage and operating boundaries

S1/G1 OPEN: ordinary machine lifecycle, authorized player-slot/retirement/departure
custody, ordinary persistent lifecycle/status/retry, live admission/replication,
committed animation, combined integrated/dedicated(two exact clients)/in-frame gate.
Current packet is frozen two-offline-profile memory recovery; no gameplay, ingress,
TCP or ordinary close caller. Live S2 requires G1. S3 exactly motor/shaft/small and
large cogs/millstone, cobblestone to gravel; no gearbox. R01 preserves pinned Java
full-output item loss. No broad journal/global codec/reduced S1/save-compat promise.

Reviewers procedurally READONLY in write-capable runtime: native PS/GetContent/rg/
Git reads/hashes/text only; no WSL/scripts/import execution/runtime/writes/network/
escalation/delegation. Authors assigned native files only, no runtime/fmt/Git writes.
Fresh independent Sol High design/source review, different final evidence reviewer.
Private branch codex/h02-smoke-h03-notes. Every private push: fresh GitHub PRIVATE
metadata plus exact origin https://github.com/Cha0sCollective/Create-Horizon.git.
Normal hooks/agent footer. No raw runtime/private plans in engine Git. Preserve
Prism/playable saves/accounts/reference snapshots/global Java; no memory writes.
