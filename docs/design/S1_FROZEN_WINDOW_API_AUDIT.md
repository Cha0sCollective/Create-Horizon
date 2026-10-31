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
