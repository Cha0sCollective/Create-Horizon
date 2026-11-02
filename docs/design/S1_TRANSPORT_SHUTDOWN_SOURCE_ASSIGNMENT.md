# C0e1a source assignment

Status: HELD pending independent acceptance of S1_TRANSPORT_SHUTDOWN_PROPOSAL.md
and a private committed checkpoint. Root releases a local gate with those exact
hashes and checkpoint before any engine edit.

Predecessor: engine114626f09afb77a4da97325e5bd4ae9f13dbcb25, clean physical
.local/worktrees/s1-retained-acquisition. Create a codex/ child branch at this pin.

Objective: opaque close-only ConnectionShutdown for actual TCP/Pipe, valid before
split and independent of ConnWriter, with lifetime/cross-endpoint/blocked IO tests.
Exact source scope: crates/horizon-protocol/src/connection.rs ONLY, inline tests.
No activation or additional paths. Implementation and validation follow the exact
accepted proposal. Root owns edits, serial fmt/compiler/tests/Git; fresh independent
Sol High SOURCE/tool and DIFFERENT FINAL remain required. Preserve the full one-path
source plus packet-prefix local artifacts before every correction. No changes to
the frozen s1cl1 evidence. New packet prefix: .local/s1ts1.

Reviewers use native read-only inspection only; inherited workspace-write is not
enforced read-only. No scripts/imports/WSL/runtime/network/edits/escalation/delegation.
No public push until exact candidate FINAL/privacy clearance; retain normal hooks
and fresh repository visibility/origin checks. No PR, merge, tag or release.
