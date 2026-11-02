# Held source assignment: retained Session close alignment

Read S1_RETAINED_SESSION_CLOSE_ALIGNMENT_PROPOSAL.md. This assignment is HELD
until exact independent DESIGN acceptance, a synced private checkpoint and root
release gate. Parent engine e6e584c53ff404b84fb364681d49d41701f9492f. Only allowed
source is crates/horizon-client/src/net.rs. Proposed branch is
codex/s1-session-close-alignment; evidence prefix .local/s1sc1.

Implement the proposal's replacement absolute-deadline/exact-attempt API in the
actual existing RetainedSession and migrate only its existing test callers.
Retain native actors, transport, queues, joins, refusal payloads and ordinary
Session/Game behavior. Stop for an exact scope review before another path or a
new production API unrelated to close/Retry. No source release for Game activation,
send replay, inbound budgets, new actors or another wrapper.

Bounded author is native TEXT only: reads/rg/hashing/Git reads and direct text
edits in the one allowed source AFTER release. No scripts/import execution,
formatters/Cargo/compiler/tests/game/WSL/Git writes/network/escalation/delegation.
Root alone owns those serial operations. Reviewers use procedural native read-only
under inherited workspace-write, not an enforced read-only profile. Preserve the
whole source and prefix before every correction; frozen FINAL needs a new prefix.
Return exact changed paths, semantics, test names and remaining limitations.
