# Frozen window API audit

Read-only implementation audit by s1cw_lan_author at parent7a08 plus the pollable
owner draft. This maps existing APIs for the accepted application design; it grants
no additional source scope or runtime/rendering qualification.

The current service is Arc<winit::Window>, Renderer and UiResources; there is no
WindowCtx type. Native helpers already fit the accepted application/view/game.rs
path budget without a renderer/protocol/saving/remote implementation change.

| Purpose | Existing seam |
| --- | --- |
| Native window | game.rs Loading::resumed, around14982: ActiveEventLoop::create_window, wrapped in Arc |
| Minimal renderer | game.rs loading_renderer, around14670: window,false,Some(gui),None,graphics_api |
| Full atlas | loaded.gui or UiResources::load, game.rs around1293 |
| HUD geometry | hud.rs HudBatch::new([renderer.config.width,height]), around676 |
| Literal diagnostics | HudBatch::text_styled with Text::literal; hud.rs around996 |
| Background | HudBatch::fill, hud.rs around937 |
| Buttons | ui::button, ui.rs around308; geometry/widget metadata only |
| Submit frame | game.rs loading_frame(&hud), around14729; Renderer::render, gfx.rs around3518 |
| Resize | Renderer::resize, gfx.rs around2816 |
| Wake | Window::request_redraw; ControlFlow::WaitUntil |

Re-resolve line numbers at the accepted stage successor. loading_frame constructs a
fixed-clock graphics frame with empty world/entity lists; it is not a title-menu
controller. Renderer suppresses terrain for frame.title and draws panorama/HUD.
An opaque status background can cover it. This qualifies only a status panel with
actual native client connection facts, not world rendering or machine animation.

HudAtlas::fonts deliberately empties its pixels (hud.rs around344), so it is
unsuitable for the window. Retain full loaded UiResources for the visible client;
fonts-only remains appropriate for the headless guest.

Keep the Resources Arc in the enclosing run scope and borrow it through Game::new,
following the existing run_event_loop(Game<'_>) lifetime. No new static leak is
needed. FrozenApplication owns both actual client Games, pollable worker, window,
renderer and full UI resources. Replaceable view state holds only text/selection/
hitboxes. Create window/renderer before worker spawn when possible in resumed;
later failures remain under the lifetime holder.

Each about_to_wait polls the owner first, moves the one offer, stationary-pumps
eligible clients, requests redraw and schedules bounded wake. Continue supervision
while minimized or when render returns Ok(false). A skipped frame is not failure.
Native GPU acquire/present can block: no hard frame-latency guarantee. Keep optional
screenshot readback/offscreen pacing out of production polling; drain timing data.

If using ui::button, call widgets::end_frame and consume/reset its cursor request;
use pointer coordinates divided by HudBatch::scale. Clip and bound diagnostics.
Use literal text rather than HudBatch::text, which interprets legacy formatting.

Do not call Loading::menu_event, StartupMenu, Game::ApplicationHandler, Game::render,
apply_settings, start_game or normal redraw/tick/extras/pad/control handlers. Those
have unrelated startup, persistence, input or gameplay side effects. Handle native
Close/Retry controls and selected physical keys locally. No unsafe UI operation or
new generic graphics layer appears necessary. Actual window/signal/failure evidence
and fresh exact-source review remain mandatory in the later window packet.

## Startup and routing audit

A second read-only audit found two concrete implementation hazards. Adding a field
to Game Options requires a tenth path (the complete literal in
integrated/transition_ownership_tests.rs around92). Only game::run calls Options::read,
so keep Options unchanged and return a private ordinary/frozen launch wrapper in
game.rs. This remains within the accepted path budget; no default initializer edits
outside it are needed.

Collect/classify managed arguments before the ordinary parser's dedicated remainder,
quick-play filesystem checks, pregen global mutation and AuthSession::load branch.
Recognize every session-file token and malformed equals form. Use var_os to reject
HORIZON_SESSION_FILE presence, including empty/non-Unicode values. Return a fixed
message without echoing account paths. Refuse auth::current before resources; no
reset API is added. Explicit MemoryOwner avoids player_name fallback writing
horizon-player.txt. Existing main.rs collects Unicode arguments before game::run;
invalid-Unicode argv can panic there before account/resource/worker effects. Do not
claim a new graceful guarantee for that untouched baseline or exceed nine paths
without fresh design review.

A strict managed whitelist should admit only fixed root, resources, window size,
exact offline name and pinned package input. Reject duplicates, conflicting values
and world/server/quick-play/dedicated/in-frame/benchmark/control/offscreen/frame/
screenshot/report exits/creative/time/weather/dimension/look/pregen modes. Refuse
command/net/component hooks and auxiliary settings before launch. Examples include
HORIZON_COMMAND_SCRIPT/OUTPUT, HORIZON_DISPENSE_FIXTURE, HORIZON_SCULK_FIXTURE and
HORIZON_NET_REPORT; re-enumerate actual hooks at the exact successor.

Branch immediately after parsing, before heartbeat, legacy signals and Discord.
Select the accepted retained signal policy. Reuse isolated Settings::load,
Packages::read, packs::startup_root, mods::content_root and
load_resources(menu=false). The latter returns full GUI and synchronously fills
Resources::rest, without the unfinished ordinary loading path. Keep loaded.resources
in the enclosing scope; borrow into clients and clone its Arc into worker startup.
Never call start_game or ordinary selection/control/extras startup.

load_resources can fall back to vanilla on pack failure. Require no fallback error
and the expected final prepared content root before admission; do not assume the
returned root still denotes the captured pair. Refuse pack startup errors rather
than copying the ordinary fallback. Narrow client construction still needs explicit
package-role/startup checks.

The fixed coordinator ROOT layout is source-world/source-archive/authority/fresh
recovered/empty continuation. Expected target is trusted fixed Overworld [2,120,2];
expected type comes from the exactly required configured prepared fixture package,
not an arbitrary first catalog entry or decoded image. Existing namespace owner
anchor grants recovery routing; image manifest UUID is not a grant. Existing verifier
then correlates physical/native/package/player/core data. No portable config format
or arbitrary-world discovery is needed. These are implementation notes, not source
release or a change to the accepted frozen profile.
