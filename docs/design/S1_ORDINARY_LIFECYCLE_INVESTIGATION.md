# Ordinary S1 lifecycle follow-on investigation

Read-only native source audit of the active session-checkpoint worktree at base
ff69d5f on codex/s1-session-checkpoint, October 30, 2026. This is planning input,
not design acceptance, API adoption, source release or runtime/G1 evidence. Current
checkpoint qualification remains the prerequisite; unsupported inputs must still
refuse and cannot replace successful normal operation of the promised machine.

Recommended dependency order: explicit opt-in lifecycle coordinator with unpublished
real-roster recovery and save/close ownership, then placement/authorized transfer.
Do not enable a filled ordinary slot before every release path preserves its owner.

## Startup and recovery

Ordinary entry selects Legacy identity in game.rs:8816 and integrated.rs:204.
Explicit identified bootstrap exists in counter_control.rs:160; empty persistent
slot installation exists in world/slot_restore.rs:1031. Current checkpoint recovery
consumes a fresh Game and supplied native host; its row installer requires actual
placed/loaded guest records (game/lan.rs:1063,1173). Integrated startup currently
opens the world, creates the host and exposes its connection before that roster
exists (integrated.rs:326-405). A follow-on design must stage actual unpublished
connections/roster before positive-owner recovery and playable authority exposure.
No fake owner or zero-roster substitute establishes positive-owner recovery.

## Save, close and authoritative reopen

begin_closing_save calls lan_close before native saving (saving.rs:31-77), while
lan_close destroys the roster (lan.rs:2532-2540). save_for_exit retires trials even
on save error (game.rs:9318-9323), as does integrated serve (integrated.rs:775-780).
Checkpoint must precede these owner releases. Failed or uncertain publication must
retain Game, host, native lock and machine ownership. Confirmed publication can
then retire. Ordinary reopen must select the coherent archive image, not silently
prefer newer native-only files. Likely bounded paths: game/saving.rs, game.rs,
game/integrated.rs, game/lan.rs, and a small dedicated lifecycle module. Exact
source scope still requires an accepted proposal and independent review.

## Positive ingress and other release paths

The player-cell key is a selector, not authorization (slot_player.rs:64). Before
lending a real inventory cell, an ordinary adapter needs actual session/owner,
target/dimension, residency/generation/revision, reach/permission, cell/count and
replay checks. Likely later touchpoints are counter_control.rs, slot_player.rs,
lan_actions.rs and one explicit opt-in command/configuration entry.

Replacement creates pending retirement (world/slot_restore.rs:1225-1258), which
checkpoint projection refuses; current delivery creates a dropped entity
(slot_retirement.rs:64) outside this checkpoint profile. Guest departure logs save
failure and discards guest/play ownership (lan.rs:2953-2975,2560-2569). Both paths
need retained failure ownership or an explicit supported restriction before filled
ordinary ingress. Do not clear them merely to make checkpoint validation pass.

Unpublished roster staging, archive/reopen routing and release-path ownership are
technical design dependencies. The audit identifies no new owner policy decision.
It does not authorize a general journal, global codec/save redesign, reduction of
S1, relaxed conservation or additional later-stage work. Reverify line anchors
against the accepted successor before authoring a follow-on packet.

## Real connection staging feasibility

A subsequent bounded native read-only audit found a protocol staging point, not an
implemented recovery path. connect_local creates an actual memory connection
(lan/mod.rs:503-515); the client performs its ordinary handshake (remote.rs:329).
Completed login/configuration emits NetEvent::Joined (lan/conn.rs:708,972), and
Hub::poll creates the actual Guest and HostEvent::Joined (lan/hub.rs:643,1135).
Stop at that event before lan_guest_joined, placement, simulation or actions.

A credible follow-on design would distinguish transport readiness from recovered
playable readiness. Return the actual pipe so configuration can complete, retain
the existing Loading terrain display while no player exists (remote.rs:865,5855),
match exact verified checkpoint UUIDs against real joined connections, and install
rows into still-unplaced/unloaded empty native guests. This needs a separate private
staged-roster capability and installer; artificially setting placed/loaded would
conceal the dependency. Factor detached host construction from lan_host
(game/lan.rs:2435-2487). Current lan_guest_joined loads native files and immediately
places a player (3056-3065,3123), so it cannot be reused unchanged.

After complete unpublished recovery, Hub::place_new_player sends Login, abilities,
teleport and inventory (lan/hub.rs:687). Serve recovered chunks and require the real
PlayerLoaded acknowledgment (2160) before releasing the explicit gameplay barrier.
Bound expected UUIDs, deadline, liveness and packet queues; do not dispatch movement
or actions during staging. Hub::poll also handles departures/duplicates, which need
explicit failure ownership. Ordinary paused ticking still runs tick_level and is
not this barrier.

For dedicated positive-roster recovery, every checkpoint participant would have to
actually reconnect under the current exact-roster primitive. Missing/offline owners
need timeout/refusal or a separately designed retained owner, never fabrication.
In-frame local Game.player has no such real guest connection and conflicts with the
current fresh-headless recovery profile. It needs its own actual local-owner adapter
before the promised in-frame lifecycle can pass; this is not permission to remove
that mode from S1. This feasibility result resolves no implementation or acceptance
gate and authorizes no changes by itself.

## Authority and failure lifetime precursor

A further native read-only planning audit identified an ordinary coordination gap:
the current destination WorldSession lock does not prevent two recovered copies of
the same world from running. Before ordinary activation, a non-clone session owner
must bind the live Game/host/native lock to a separate authority lock in one stable
trusted coordinator namespace keyed by expected WorldUUID. Existing
save::SessionLock::acquire (save.rs:138-161) supplies the OS-held lifetime primitive.
Keep it outside the archive, whose enumeration deliberately permits only epochs and
pending attempts. Derive expected identity from the trusted selected owner folder,
not a caller-supplied archive manifest or claimed hash. One configured namespace
must map the same UUID to the same lock regardless of disposable destination paths.

Acquire this authority before selecting an epoch or opening a recovered candidate.
Under it, ordinary reopen must exclusively select a checked complete archive image,
never fall forward to native-only files. Reserve unused epochs accounting for failed
pending names. No moving-pointer record or generic journal is necessary. Initial
identified creation/bootstrap must acquire authority before playable exposure.

Keep the Game, actual host, native session and authority together on failed or
uncertain checkpoint publication. A failed close must freeze the owned session for
inspection/retry, not return through a scope that drops its Game. This needs actual
server-worker lifetime changes: save_for_exit retires after errors and integrated
serve retires when its loop returns. Merely inserting a checkpoint before save_game
does not establish retained failure ownership. The server handle/worker must survive
an unsuccessful stop request until an explicit supported resolution.

The smallest proposed dependency order is private authority/lifetime ownership with
item ingress disabled, then real positive-owner staging, then the actual in-frame
local-owner adapter. A candidate first scope is a new game/session_lifecycle.rs and
focused tests, game.rs configuration/ownership, integrated.rs readiness/failed-stop
lifetime, saving.rs checkpoint-before-release, and only needed trusted routing seams
in world_loading.rs/host_identity.rs. This is a source-path proposal, not release.
The exact state transitions and namespace/layout need a concrete accepted design.
Zero-roster recovery cannot substitute for the later positive-owner or in-frame
outcomes. The audit identifies technical design dependencies, no new owner policy
decision, new portable save format or permission to reduce S1.
