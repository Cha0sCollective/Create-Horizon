# Retained client Session: FINAL

Enginec73abbef3f8175077abc5cd6e9e13e57c40cf401, tree
16a38ec0c9a5504ad29f3ff88b1aa3af82041017, sole parent
909452923f9f6b83224714eefdc139e665c902ac. Different independent
retained_client_final ACCEPT FINAL/PUBLIC SOURCE PRIVACY CLEAR, receipt33db8d13.
Frozen input5ba6535a contains53 files independently hashed before/after review.
One canonical raw net.rs equals index and committed blob1e1fed09. Normal commit
and full-history public push hooks passed; exact codex/s1-retained-client remote
revision verified. No PR, merge, tag or release.

The dormant offline RetainedSession owns original setup/reader and independent
writer handles before their start latches. Native input and pending writer remain
owned across actual spawn refusal. Permanent close/registration fencing interrupts
late acquired transports before handshake IO. Pending DNS/connect stays on the
same original actor; Retry does not create replacements or reopen admission.
Actual finished handles are joined once, outside locks.

Outgoing UI calls perform no native IO or worker wait. One FIFO lock orders planned
wire phases and packet admission; separate actual write receipts gate Joined and
Reconfigured on the reader worker. Queue budget8MiB/256 counts active plus queued
bodies; explicit refusals return original bodies. Failed active/queued bodies,
unconsumed events and terminal history remain owned. Consumed events transfer to
the caller rather than creating an unbounded past-packet payload history. Transfer
and cookies have value-only handoff after actual native quiescence.

Construction excludes auth/Friends before acquisition and preserves original Pipe
on refusal. Ordinary Session/default/B paths are untouched; the one-file public
diff is2,438 additions and zero deletions. No ordinary launch selects this mode.

## Actual qualification

24 normal cases passed in0.48seconds (19 defining+5 existing networking regressions),
receipt977ebe0f. Coverage includes actual TCP/Pipe login and compression, partial
native reads, late registration and deadline custody, configuration FIFO and actual
acknowledgments, Ping/teleport/KeepAlive ordering, byte/body budget and automatic
reply refusal, real native backpressure, spawn/panic/encoding/IO failures, retries,
constructor input ownership and both-phase transfer/cookie handoff.

One separate fatal-owner process passed in0.07seconds, receiptCB003da9: exact armed
marker and unresolved-owner diagnostic, SIGABRT/-6, core0 and checked reaping.
No native normal or fatal case failed. Six fresh static checks41a79464 passed in
160.84seconds, including targeted formatting, all-target Clippy-Dwarnings, three
feature configurations and public-source check. Actual Cargo library build55.88s,
receipt4ac87807; executable87b38c2a hashed before/after use. Features mods,
paired-packages,native-writer-test-control; opt0, line-table debug, debug assertions
and overflow checks. Linux UID1000, isolated paths, pinned Rust1.99.1, offline and
--locked, same physical target. No prepared resources, Game or Java required.

Fresh independent design/source/tool review preceded execution. One initial SOURCE
fixture finding was corrected before compilation: actual client TCP read/write
timeouts are disabled, with peer waits and direct-process watchdogs failing the
case. Exact formatter delta reviewed. Source/prefix attempts0..3 preserve prior
versions. All dirty-parent qualifier modes are retired for the clean successor.

## Limits and follow-on

Acquisition gates prove original actor/input custody and deadline behavior, not DNS
cancellation. Backpressure supports blocked-write custody and checked reap; peer
clone cleanup prevents an exclusive close-causation claim. Native acknowledgment
completion proves no peer processing or delivery. No hard latency, Windows runtime,
Game/Guest durability, ordinary activation, global CloseCommit or G1 claim follows.

Server/client transport primitives are now separately FINAL; full composition is
still required. Next: actual LOD outer+sampling ownership, native control workers,
then bounded window/in-frame integration and honest Remote/preparation seams.
Integrated/dedicated and connected G1 remain later work. S1/G1 and live S2 gates
remain open.
