# Original remote Game: implementation gaps

Read-only research at accepted engine `e72475864c13720fe066d5bd6335d652ce5cd2bf`,
tree `4a5567d1`, private results checkpoint `0d5b1859`. This is an inventory, not
an accepted design or source release. S1/G1 remain open.

The retained Session, decoder and received-column streamer are qualified separate
owners. Their existence does not make the original Game constructor or callers
retained. The next design must account for the following concrete boundaries.

## Construction and representation

`Game.streaming` is a concrete `Streaming` (`game.rs:1899`). Its fields and methods
are used throughout Game, including native fixtures and preparation owners that
should remain raw Streaming. An opaque remote wrapper cannot supply an inner
Streaming reference, even an immutable one. A private owner enum with explicit
native access and remote dispatch is the concrete migration direction under
investigation; an enum Deref or second placeholder pool does not solve it.

`InitialGameWorld`/`Game::new_with_world` currently select only Legacy or retained
native ColdPreparedWorld (`game.rs:14082` onward). Legacy construction starts a
title terrain worker and supplies a title SaveDir. A retained remote recipe must
be selected before that branch, retaining original CPU/resource and Game inputs.

`join_server` (`game/remote.rs:335` onward) first creates a legacy Session, then
replaces streaming with a legacy NoTerrain pool and installs a SaveDir named
`horizon-remote-unused`. Calling that path and replacing its owners afterward
cannot establish retained startup. The new recipe must represent absence of
local storage honestly; Game's concrete save field needs an explicit treatment.

The existing legacy leave and dimension paths replace pools again
(`remote.rs:570`, `1056`). Unsupported replacement, reconnect, transfer, dimension
and resource changes need refusal at their entry before mutation, or an explicitly
reviewed ownership-preserving transition. Pack reload currently reaches streaming
after earlier resource mutations (`game/packs.rs:240`); guarding only that last
call would be too late.

## Transport and decoding

`Remote` currently contains legacy `Session` and `ChunkDecoder`
(`remote.rs:109` onward). RetainedSession is in **net.rs**, not session.rs. Its
offline Memory and Address recipes, actual checked handle closure and retained
events/bodies are relevant existing foundations.

The original `Game::send` is void (`remote.rs:612`), while retained sending returns
explicit admitted/refused outcomes and original encoded bodies. The original
remote poll also uses legacy decoder submit/next and an ordered event backlog
(`remote.rs:804` onward). Integration must choose explicit admission/backpressure
and preserve ordering; a void adapter that discards retained refusals is inadequate.
There are also direct Session operations in dialogs and server-pack handling.

Existing RetainedSession public polls, sending and close acquire its State mutex
(`net.rs:1383` onward). Native socket writes happen outside that mutex, but those
APIs are not try-only. The actual client UI design must address that distinction;
standalone closure evidence is not evidence of nonblocking Game/UI composition.
No new latency or global memory bound follows from this inventory.

## Streaming, rendering and closure

Game's normal update mixes received events with native tickets/generation and
legacy streaming (`game.rs:12851` onward). A retained remote branch must schedule
real Mesh/Light, handle capacity and stale outputs explicitly, and reuse existing
mesh upload/renderer behavior rather than forwarding legacy update.

The qualified wrapper still lacks the common readiness, checked forget and mesh
configuration surface required by those callers. Initial immutable configuration
selected before start is a possible bounded direction; runtime configuration
changes would need explicit revision/invalidation semantics. A settings setter
alone cannot make an old-settings mesh current.

Legacy remote unload/dimension code calls `chunk::free_columns`
(`remote.rs:1073`, `1142`), which starts a separate freeing thread at 64 columns.
A retained recipe must explicitly account for or exclude that reachable native
acquisition; it cannot assume that dropping received columns is always inline.

Original Game/renderer/window ownership and all acquired native actors must remain
in the actual close recipe. Separate ClosedSession, ClosedDecoder and
ClosedRemoteStreaming values are not whole-client or native writer witnesses.
External remote LOD currently differs from integrated-server LOD; retained shared
server source custody and LOD-before-server-storage ordering remain separate.
Original server Game startup, Hub transport and scoped checked joining are not
supplied by these client components.

Static input hashes at this research boundary: game.rs `a82db14f`, remote.rs
`bac0947e`, net.rs `4c4b2188`, streaming.rs `356396c6`. No engine source, runtime,
Java, playable settings, dependencies, source references or stage gates changed.
