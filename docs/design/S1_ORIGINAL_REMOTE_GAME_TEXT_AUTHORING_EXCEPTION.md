# Proposed packet-only disjoint text authoring exception

Status: HELD operating proposal. No parallel engine source mutation is enabled
until fresh independent Sol High review and coordinator adoption/checkpoint.
This changes scheduling only, not source scope, tool permissions or acceptance.

The adopted immutable source-freeze layout says: "No running compiler, test,
source mutation or Git write is parallelized." Keep that historical document and
all indexed evidence unchanged. For the FIRST UNFROZEN TEXT DRAFT of s1ogrc1 only,
permit two explicitly bounded native-text authors on DISJOINT files in the SAME
24-path packet and branch. This is one integrated candidate, not two independent
production lanes. No new worktree, target, issue, executor or standing team.

Purpose: actual original Game adoption couples IO, input/frame extraction and
window ownership across 24 paths. The bounded author can implement IO while the
coordinator implements the original factory/window, reducing serial author time.
Explicit shared signatures and file ownership precede any coordinator source edit.
Until then the existing sole author continues serially on its own files.

## Exact split

Bounded author `/root/window_preparation_author` exclusively owns:

```
crates/horizon-client/src/skins.rs
crates/horizon-client/src/game/retained_remote.rs
crates/horizon-client/src/game/retained_remote_tests.rs
crates/horizon-client/src/game/streaming_owner.rs
crates/horizon-client/src/game/remote.rs
crates/horizon-client/src/game/remote_chunks.rs
```

The coordinator exclusively owns the remaining 18 paths already released by
`.local/s1ogrc1-parent-gate.json` (`80e056ca`), including game.rs, window_close.rs
and necessary listed mechanical callers. Neither author edits the other's files.
If another file or changed contract is needed, stop that dependent edit and
coordinate its exact ownership; do not silently transfer or expand permissions.

Both agree a small concrete native-text signature list for factory/IO/streamer
close/input-frame integration. The coordinator owns integration and every Git
write. Parallel authors use only native reads/rg/hashes and direct TEXT edits.
The bounded author still cannot execute scripts/imports/formatters/compiler/
tests/game/WSL/network/Git writes or escalate/delegate. Root execution authority
does not permit running those operations while either author is mutating source.
Read-only research/tool review may continue; exact SOURCE review waits for freeze.

## Quiescence and unchanged gates

Before ANY formatter/compiler/test/game/WSL runtime or Git write, BOTH authors
must stop source mutation and acknowledge their current paths/state. This also
applies to the operating adoption checkpoint: pause the sole author before those
Git writes, then explicitly activate the split. No simultaneous compile/source/
Git activity, source editing during a running exec cell, or extra privilege.

When either author submits for SOURCE review, quiesce both, assemble one actual
candidate/manifest and preserve ALL 24 packet paths (including absent/new markers)
and all prefix artifacts. The exception ends at that first freeze. Every later
correction stays serial, with full preservation before edits and exact delta review.
No review or failed check can be repaired under concurrent writing.

Root alone runs all pinned serial formatting, checks, actual game and Git writes.
Existing fresh SOURCE/tool, post-format binding, actual guarded live execution,
raw archive/Git comparisons, separate failed evidence, different FINAL/privacy and
fresh visibility/origin/normal-hook publication boundaries remain unchanged.
No source change to already indexed files. No private material enters public Git.

This exception expires for later packets. It gives no library/schema/source path
expansion, default/C1/G1/S2 activation, publication, PR, merge, tag or release.
If disjoint ownership cannot be maintained, revert to sole serial authoring.
