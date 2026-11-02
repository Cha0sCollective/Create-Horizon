# C0e1a: close-only native transport capability

Status: PROPOSED; no source authority until independent design acceptance and a
private source-assignment checkpoint. C0d is accepted at engine
114626f09afb77a4da97325e5bd4ae9f13dbcb25, tree
64558c3469f71340ef9685192cf4e3866671d9a0. S1/G1 remain open.

## Bounded outcome

Give the owner of an actual Connection a clonable close-only capability before
handshake IO or split. It interrupts the same TCP socket or MemoryPipe without
locking ConnWriter, sending protocol packets, acquiring packet codec state, or
joining a thread. It exposes no Read/Write/socket/Connection access. Calling it
requests shutdown; it NEVER proves worker termination, successful packet delivery,
native save durability, guest disposal or global CloseCommit.

This is the first small source packet within C0e1, replacing no later gate.
C0e1b will separately assign tracked LAN accept/connection/writer/Pinger ownership;
C0e1c will separately assign retained client Session acquisition/reader/writer
ownership. Their source assignments require fresh review at the actual successor.
No ordinary constructor, admission or application-close activation in C0e1a.
The old C0e1 map is refined into these packets, not expanded into standing authority.

## Exact source scope and implementation

ONE path, including focused inline tests:
crates/horizon-protocol/src/connection.rs. No Cargo/dependency/feature changes,
general transport trait, executor, new thread, unsafe code or raw-descriptor API.

Add public opaque ConnectionShutdown, Clone, and
Connection::shutdown_handle(&self) -> ConnectionShutdown, with
ConnectionShutdown::shutdown(&self) -> std::io::Result<()>.

For TCP, Stream::Tcp holds Arc<TcpStream>; each actual reader/writer Stream clone
holds a strong reference to that same socket. Read and Write delegate through
&TcpStream, preserving the blocking pipeline and OS-shared socket options. The
capability holds only Weak<TcpStream>. Shutdown upgrades it for the duration of
TcpStream::shutdown(Both), returns the actual OS result, and releases it. If all
real endpoint halves have gone, return Ok without IO. No extra duplicated socket
is retained by the capability. A concurrent shutdown call can briefly keep the
endpoint alive during its syscall; do not claim otherwise. Repeated calls can
return platform-specific already-disconnected errors; no success normalization of
an actual OS error. Existing ConnWriter::shutdown behavior stays compatible.

For memory, delegate to the existing PipeShutdown, whose channel references do not
keep Pipe::End alive. Existing Pipe/ConnReader/ConnWriter/drop/compression/encryption,
total deadline and counter semantics stay unchanged. The new capability itself
has no Drop shutdown action. It remains valid across Connection::split and cannot
be redirected to a later connection or a reused numeric socket identifier.

Memory shutdown uses the existing short Channel state locks. Arc upgrade and OS
shutdown are synchronous operations; no hard per-call real-time latency guarantee
is claimed. No writer mutex or worker join is involved.

## Qualification selected by changed behavior

Use the existing physical target serially, pinned Rust1.99.1/MSRV1.90, offline.
No Minecraft resources, prepared worlds, Java, game process or renderer is needed.
Root performs all tooling; tests spawn only native fixture threads and retain/join
them. Missing socket setup, unsupported essential behavior or watchdog expiry is
a failure, never a skipped positive. IPv6-specific pre-existing tests retain their
own explicit qualification limits.

Defining controls: actual TCP and Pipe interruption before split and after split;
partial frame read; actual full TCP send queue with a blocked writer; both halves
closed; clones and repeated requests; dropping capability leaves live endpoints
usable; dropping actual endpoint with capability held still produces peer EOF;
closing an old endpoint cannot close a separate live endpoint. Channels/watchdogs
and real queue WouldBlock establish preconditions, not sleep-only assumptions.
Fixture read/write timeouts are backstops; success requires shutdown-triggered
completion before those backstops and checked joins. Cleanup wakes/reaps fixture
threads even on failed observations, rather than dropping handles on assertion.

Run the complete small connection::tests module once, including existing total
deadline/timeout, compression/encryption, counters and Pipe regressions. Run
horizon-protocol formatting and all-target Clippy with warnings denied. Type-check
the direct client dependent under no-default-features plus mods,paired-packages
using this same target to cover its actual callers. Broader client/game batteries
belong at changed composition, not this socket-storage component. No old result is
relabeled fresh. SOURCE/tool review precedes compiler/runtime; different FINAL
checks the exact clean successor and actual evidence before public consumption.

## Follow-on boundaries discovered during audit

The server packet must include lan/discovery.rs if announcement is supported,
because its Pinger has a real blocking Drop join. Listener stop, accept/spawn/
registration, handshake/configuration including resource-pack scoped workers,
play writer panic/spawn/send/EOF, queued Joined/Left events and UUID custody need
concrete tracked outcomes. Online services-key fetch cannot be treated as absent;
support requires real ownership or a pre-acquisition constructor restriction.

The client packet must register the close capability immediately after acquiring
the actual transport, before handshake IO, with a latched stop/registration race.
DNS/connect work can remain pending with its original handle retained; absence of
a writer is not completion. friend: tunnel and online-auth helpers require real
ownership or rejection before spawn/acquisition. Preserve ordinary default and B
diagnostic behavior; reject rather than silently downgrade unsupported profiles.
Remote ChunkDecoder, Friends, LOD, application/control/integrated/dedicated workers
remain later explicitly assigned owners. This primitive proves none of them done.

## Stop condition

Accepted one-path source, focused checks, clean successor, different component
FINAL, private checkpoint and separately authorized reviewed public branch sync.
Then propose the next exact source packet. No C1, G1, S2, PR, merge, tag or release.
