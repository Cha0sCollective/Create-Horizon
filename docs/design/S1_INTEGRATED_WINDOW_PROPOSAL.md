# Joint integrated window and server lifetime

PROPOSED DESIGN, no source release. Parent accepted/public5d9eaa14a65d831301e9ae25fa8859193871848c,
tree848f1889, Step A native integrated owner complete. This is Step B within
adopted deliverable 1. Open to LAN deferred; original G1 remains OPEN. New prefix
s1iw1 and branch codex/s1-integrated-window are proposed after checking absence.

## Exact proposed budget and reuse

Five existing paths under crates/horizon-client/src: game.rs,
game/integrated.rs, game/window_close.rs, game/retained_server.rs and
game/window_close_tests.rs. Reuse original remote Game/RetainedIo, Memory Pipe,
decoder/streamer, native owner adapter, server supervisor, ColdUi presentation,
renderer function and one existing application/control endpoint. No edits to
transport/decoder/renderer/codec or a new task executor/loader/Serving owner.

An explicit integrated qualification entry in RemoteLaunch can select the new
component alongside the unchanged TCP entry. It uses the same fresh application
root with a fixed server child directory and fixed valid offline owner name;
there is no additional arbitrary server/world path or Open to LAN capability.
Ordinary selected-pair default startup and C1 stay separately gated. Qualification
entry alone cannot establish default integrated acceptance.

## Frontend preparation and actual Pipe attachment

Before any server worker starts, fully prepare frontend settings/options, CPU,
registries/language/name, resource/GUI bundles, event loop and inert control.
Use an explicit integrated-preparing application phase owning these inputs and
ColdUi. Build actual window/renderer with the existing renderer function, then
start and immediately retain the server supervisor in Application. If frontend
construction fails, no server has acquired a world.

Poll actual ready without blocking the event thread. Request exactly one actual
Pipe through the existing Offer channel and retain that receiver. Split the
existing Game frontend factory into preflight and attachment. Attachment accepts
RetainedTarget::Memory and preserves BOTH original target and prepared inputs on
Session validation refusal, as the library already does. No map_err dropping the
target, fake address, dummy Pipe or title-world Game. After successful Session
validation, transfer preloaded CPU/session/registry/language into the original
Game without further ordinary fallible work, then transfer original window/
renderer and start its existing remote actors. Preserve TCP wrapper behavior.

The integrated-client role is private/typed and supplied only with the actual
owned supervisor's Memory attachment. It affects frontend singleplayer/pause
classification, never server mutation authority. Reuse RetainedIo unchanged.

## Joint lifetime, errors and pause

Application owns supervisor, pending Offer receiver and any refused/late returned
Pipe through cancellation/failure. Close admission freezes both sides. Drain a
late successful Offer: shut down and retain its actual Pipe until the server
joins; retain a pending receiver until a real reply or terminal channel closure.
Reuse existing failed-reply custody in the server; do not invent a second owner.

Client close finishes actual Session/decoder/streamer. Server close completes
native owner/world metadata and actual outer worker join. Poll separately; neither
client-ready nor worker-spawned observations permit joint completion. Only when
both finish and pending endpoint custody is settled may control seal and the
application exit. Startup without a Game may use its original preparing UI to
report failure and close acquired owners. No ordinary error return drops a
running supervisor/endpoint. Irreversible event-loop loss retains the existing
explicit fatal backstop rather than claiming safe shutdown.

Server failures initiate client close; client failures request server close.
Preserve both actual diagnostics. Native completion/join does not make raw
EOF/Receive/ENOTCONN clean. Retry routes to actual blocked owner using its own
current attempt; client/application and server attempt numbers are not equal by
assumption. Preserve status presentation/explicit error acknowledgment semantics.
Error-free ordinary stop remains an explicit later acceptance issue if this
component still exposes the original diagnostics.

Use typed frontend role in has_singleplayer_server. Integrated Escape uses normal
pause/resume rather than TCP departure. Forward pause to the actual supervisor
and reuse tick_server_timed(paused): network continues while authoritative world
time holds. Resume does not catch up elapsed wall time. Do not install a legacy
Integrated handle or Serving executor. Keep fixed view/profile/resource guards.

## Required evidence and stop conditions

Actual original Game/application with owned isolated X11 window, Memory backend,
terrain/mesh pixels and observed movement/rotation; native owner UUID/player record
readback; pause holds actual backend world clock and resume advances it; joint
closure waits for real server native completion/join. Include actual native save
IO failure/Retry and cancellation during preparation and pending Offer. Tests use
fixed cfg-test gates only where needed, no arbitrary Game callback executor.
Adapt existing s1ogrc1-window.py facilities without another harness platform.

Run focused source checks and affected TCP/in-frame regressions; no historical
full battery. Distinguish clean completion from original diagnostic/error results.
No public push before different FINAL/privacy; no release. If APIs cannot provide
the above in this budget, identify the exact seam and obtain bounded review;
any broad redesign or supported-profile expansion goes to owner BEFORE coding.
