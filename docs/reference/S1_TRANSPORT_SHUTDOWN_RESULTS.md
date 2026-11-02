# Native connection shutdown component

Candidate engine906f91b10683224532f8b300dd6a4a654acbe88a, tree
9a349b3dd986c9718cb8ac0f9b769b3136e32891, sole parent
114626f09afb77a4da97325e5bd4ae9f13dbcb25. Component FINAL pending different
independent review. S1/G1 remain open; ordinary application close not activated.

One public source path changed, protocol connection.rs:248 additions/7 deletions.
Raw qualified source42e6d8a matches the staged and committed blob exactly.
Opaque ConnectionShutdown is acquired before split. TCP endpoint halves share
Arc<TcpStream>; the capability holds Weak and upgrades only during the shutdown
syscall. Memory uses existing PipeShutdown. Capability drop is inert, and neither
clone nor retained capability prolongs the original endpoint's lifetime. Shutdown
requests interruption and reports OS errors; it proves no worker completion or
delivery. Existing codecs/deadlines/counters and ordinary APIs remain compatible.

Fresh independent transport_shutdown_design_review accepted design22ea9f15 and
assignment70023f77, private455680b. It cleared final source42e6d8a, manifeste6356db0,
toolae34b7a1 and gate4ffb9e07. Tool review initially required a finite watchdog;
the preserved initial tool was not executed. Corrected tool directly owns the
native test executable with90-second kill/wait timeout and failed expiry evidence.
Formatting correction was preserved and independently checked as wrapping only.

## Actual qualification

All20 connection tests passed in0.62seconds: five new defining tests and fifteen
existing regression tests. Native TCP and MemoryPipe cover partial-frame read
interruption before/after split, both split halves, inert capability destruction,
endpoint lifetime/isolation and cloned/repeated shutdown. The TCP write control
observed native WouldBlock/full send queue and unresolved writer before shutdown.
Fixture workers were woken/reaped and joined before assertions; completion was
required before seven-second fallback timeouts. Existing total IO deadlines,
encryption/compression, timeout restoration, counters and Pipe tests also passed.

Actual Cargo test artifact399507c5 was hashed before/after direct invocation;
opt0, line-table debug, debug assertions/overflow checks, protocol features empty.
Build29.59seconds. No game, Minecraft resource, world, Java or renderer was needed.
Tests ran as Linux UID1000, isolated paths, pinned Rust1.99.1, offline, serially in
the existing physical target. No native child processes inside the test binary.

Fresh focused checks: protocol fmt1.72seconds, all-target Clippy with warnings
denied12.68seconds, client lib+game typecheck under no-default-features plus
mods,paired-packages59.05seconds, public-source check0.10seconds. All passed.
Normal commit public-tree hook also passed. No unchanged earlier game batteries
were replayed or relabeled fresh. Local checks/tests receipts and logs remain
outside Git under s1ts1; snapshots0..4 preserve pre-correction source and tools.

Limits: existing two IPv6-specific tests can return early if loopback IPv6 is
unavailable; no IPv6 qualification claim. Windows runtime not exercised. Shutdown
uses native synchronous calls and existing Pipe locks, without a hard latency
guarantee. Actual LAN/Session/listener/writer/Pinger worker tracking and close
composition need subsequent separately reviewed packets. No CloseCommit, C1,
connected G1, live S2, PR, merge, tag or release follows from this component.
