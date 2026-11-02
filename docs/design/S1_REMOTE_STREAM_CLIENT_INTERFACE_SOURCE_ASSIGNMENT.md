# Held source assignment: fixed remote streaming client interface

Read S1_REMOTE_STREAM_CLIENT_INTERFACE_PROPOSAL.md. HELD until exact independent
DESIGN acceptance, synced private checkpoint and root release. Parent engine
9d3ecdc510adbeb70840371fec2c0b9db4c23746. ONLY allowed source is
crates/horizon-client/src/streaming.rs, including its existing inline tests.
Proposed branch codex/s1-remote-stream-interface, evidence prefix .local/s1rsapi1.

Implement the fixed pre-spawn profile and explicit received-column/light/forget
operations plus selective latest-stamp retirement in the actual opaque owner.
Preserve original workers, inputs, receipts, slots, native paths and closure.
No Game dispatch, new worker/owner facade, callbacks, storage witness or runtime
configuration revision scheme. Stop before another path or broader API.

Bounded author is native TEXT only after release: reads/rg/hashes/Git reads and
direct edits in the one source. No scripts/import execution/formatters/Cargo/
compiler/tests/game/WSL/Git writes/network/escalation/delegation. Root owns all
serial execution/Git writes. Reviewers are procedural native READ-ONLY under
inherited workspace-write, not enforced sandboxing. Preserve the whole source and
prefix before every correction; frozen FINAL correction requires a new prefix.
Return exact paths, semantics, meaningful case names and source hash.
