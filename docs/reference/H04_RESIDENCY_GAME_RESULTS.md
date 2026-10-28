# H04c actual authoritative residency qualification

Local engine `a4470cd4a1bb1ef776a11fdf687b709c32dbe358`, tree
`c0fd17abb8ff4854962c21734a042be3d90c1bb1`, adds explicit residency controls using
the same checked owner/dimension routing as mutation controls. No automatic
observer, machine reconstruction or guest interface is introduced.

Frozen fast-profile executable, Rust 1.99.1, no defaults, mods+paired-packages:
`295e2e6c6e2ad75d5690ee3e252c23353cfca10255c1ce3874ada1d17a762adb`.
Build receipt is `.local/h04c-build.json`. Driver
`cacda5cbd3a080e90bac62c99c8d336745031cdd135531b5ffa6b0728ac579f2`
and unchanged base
`9cb17515a22cdcb74a4e550370b2ecc51c94810b4aeb32094d31b34c837f00fd`
are archived with selected package/component hashes in each run.

| Mode | Accepted complete run | Assertions |
| --- | --- | --- |
| Integrated | `e6837c81-d4d8-4908-8c31-1bcb10c34f0b` | 228 pass |
| In-frame | `1476794b-1fa7-4ae2-819a-b18a703c161f` | 212 pass |

Both use fresh isolated game/launcher/XDG homes, owned native resources with
manifest `f35140182f95862e5330fabe4c6a449805f2d83be94dce2f01e770a2ede68ce2`, and
llvmpipe. Raw runs remain `.local/h02d/game-runs/<UUID>`, outside Git. No performance
qualification follows.

The actual owner travels through non-spawn chunks 8 -> 40 -> 8. The driver waits
for actual player position and authoritative watched membership, not an unchanged
dimension string. Initial sorted/deduplicated snapshots remain separate from
history. Raw sequences 1..4 report the exact per-chunk unload/load history with
captured scope/time. Independent cross-chunk I/O completion order is not fixed.

A marker saved normally in chunk 8 returns with its exact tuple after unload and
reload. Ordinary save and actual SDK ground-query map lending produce empty
residency batches. The integrated window replica stays disabled while the server
observer is enabled. Capacity-one overflow remains sticky while actual membership
continues changing, including both watched chunks becoming absent. Drains never
claim complete history after fault.

The observer is deliberately enabled before save/exit and disabled after restart.
Both runs repeat original package activation, six states/default/player placement,
ordered writes/no-op/filtering, piston and deferred SDK writes, actual dimension
round trip, captured reload, settings, restart and admission controls. Initial/
restart exits are zero, expected changed/missing package refusals exit one, and
all final cleanup errors are null. The inspected driver asserts saved-world byte
equality across refusal; separate raw hash manifests are not retained.

Source/driver and final actual receipts received fresh independent read-only Sol
High review with no remaining findings. Reviewer checked unchanged controller
hashes, archive identities, raw snapshots/events, phase outcomes and check logs.
It performed no imports, builds, game launches or writes. Executable identity uses
recorded receipts, not an independent reviewer Linux binary rehash.

Controller checks pass: 137 binary tests, three ignored; feature-on/off Clippy and
formatting. The initial constant-pair chunks_exact lint is retained separately and
excluded; as_chunks preserves the even-length guard and declared Rust minimum.
Normal public-tree commit hook passed; no engine push occurred.

This proves bounded coordinate residency, not recovery of unloaded contents from
history, activation, World-lifetime retirement, persistent keys/generations,
transactions, inventories, coherent checkpoints, external clients or G1. Next is
the explicitly transient inventory-free counter trial in
`docs/design/H05_NATIVE_COUNTER_PACKET.md`.
