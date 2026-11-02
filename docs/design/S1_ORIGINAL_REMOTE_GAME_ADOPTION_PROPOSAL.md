# Original remote Game: fixed running client and retained close

Status: HELD proposal for fresh independent review. No source or activation
release. Parent is clean engine `e52f0def9f4cfd297133c9c8d018e8061a7fe328`, tree
`3efbc31c`, reviewed public branch synced, private results `9c453a4` synced.
Prefix `.local/s1ogrc1` was checked empty before its first research artifact.
Proposed branch `codex/s1-original-remote-client`; reuse the stable physical
checkout and its dedicated warm target. S1/G1 remain open.

## Concrete outcome and admitted profile

Run the ORIGINAL Box<Game>, renderer and window as an actual unpaired offline
TCP client, using original RetainedSession, RetainedChunkDecoder and
RetainedRemoteStreaming owners. Process real login, terrain, movement and rendering,
then close/retry those same owners with the window/control service retained.
Do not use a title Game, legacy join followed by replacement, dummy SaveDir,
substitute streamer, static rendering mock or a status-only fake world.

This is the next C0e2 component, not C1 admission or G1. Initial route is an
explicit Linux-only `--retained-client-window ADDRESS` launch with a fresh absolute
`--retained-client-root PATH`, resources, optional control socket/window size and
bounded frame/time limits. Ordinary/default launches remain unchanged. Select this
route before ordinary heartbeat, mods/extras, menu loaders or native title/world
acquisition. Reject conflicting/harness/session/auth/unsupported arguments before
acquisition. Validate offline name and resource/profile inputs explicitly.

First profile: no selected mods/pairs/extras, no custom skin fetches, server packs,
resource reload, native world entry/replacement, reconnect/transfer, reconfiguration
or dimension replacement. Use Audio::silent, controllers feature-off or an explicit
pre-acquisition refusal, and existing retained INLINE renderer construction.
Admit LOD-disabled settings only; do not invent an external/native LOD source.
Capture actual initial Login dimension and already-obfuscated biome seed before
starting the prepared streamer. Unsupported transitions fail into owned close
before destructive Game mutation or new helper acquisition. This profile does
not remove those features from ordinary Game or promise them for this component.

Memory/integrated and original server ownership remain C0e3 work. The existing
paired configuration plan requires C0 composite FINAL before C1; no package
external guard, required-negotiation diagnostic or admission rule is relaxed.
No additional login-only diagnostic is proposed as paired admission.

## Construction and representation

Prepare complete actual resources/GUI/CPU inputs synchronously with menu=false.
Keep absent rest/menu/world/pack loaders explicit. No native-world install or
advancement prefetch just to obtain a client Game. Create the platform event loop
before client actors where feasible; preserve all partially acquired inputs on
ordinary setup failure, with fatal ownership backstop only for genuinely
unrecoverable platform/unwind failure. Do not silently drop prepared owners.

Extend InitialGameWorld with a private typed remote recipe. Directly install
GameStreaming's actual retained owner and save=None before any legacy constructor.
Current Legacy/native arms keep their original Existing/Some allocations and
native authority/provenance. Preloaded CPU must be established before an owned
remote recipe can reach a fallible loading branch. Construct one original Game;
its initial empty client World is not a native world or storage witness.

Keep Remote common player/protocol metadata, but group original legacy or retained
connection/decoder state explicitly, e.g. private RemoteIo variants. The retained
variant owns the original Session/decoder, pending decoded input, refusals and
checked closed outcomes. Preserve original owners on failed consuming transition;
ordinary factories remain legacy. Small Option slots for consuming original/closed
states are acceptable; no temporary pool, fake completion or actor replacement.
GameStreaming likewise needs an honest way to keep original active OR closed
remote ownership in the actual Game during closure, without an inner escape.

Audit direct ordinary Session uses in Game exit, debug, dialogs, server packs,
managed application and the connected-recovery fixture. Legacy-only assertions
may remain only where admitted constructors prove them unreachable to this recipe.
Never take/drop Remote and discover its retained owner afterward. Do not expose
raw writer/handle/cookie Arcs or reinterpret legacy labels as native provenance.

## Actual ordered client dispatch

Retained processing preserves original event/packet order. Handle Joined and
play Login barriers BEFORE chunk admission; current eager legacy submission uses
the placeholder format and repairs it synchronously. Keep that legacy behavior,
but never use its synchronous decoder fallback in the retained recipe.

At a chunk barrier, retain the original packet/format on Busy/Full and retry in a
later frame; later events cannot overtake it. Keep returned RetainedDecoded owned
through insertion refusal, restoring the exact refused Chunk. A wrong format is
an explicit retained failure or original re-admission, never inline re-decoding.
Install block entities only AFTER successful received-column admission. Apply
light and forget through the qualified checked APIs before related World/renderer
mutation. Dispose original replaced/forgotten columns deliberately and in bounded
turns; do not call the legacy >=64-column freeing-thread helper.

Provide a retained update branch that admits real Mesh/Light, preserves refused
inputs, reconciles/takes actual outputs, applies only current results and requeues
appropriate stale dirty work. Keep dirtiness until admission succeeds and avoid
starving renderable interior work behind permanently missing edge neighbors.
Reuse existing upload/sort/renderer behavior where separable. Do not run native
generation, entity unload/persistence or SaveDir paths. Received readiness,
conservative mesh pending, actual view center/radius and profile-based biome
sampling must replace all REACHABLE Existing assertions. Unavailable census stays
unavailable, never zero or loaded; Joined alone is not rendered readiness.

Existing void Game::send may remain only with an explicit, visible owned failure
bridge: preserve every actual refused encoded body; fence further admission and
retain any subsequent unattempted typed intents until the parent enters close.
Alternatively propagate typed outcomes through actual callers. Do not silently
discard, pretend send success, re-encode/replay a refused body or claim wire
delivery from queue admission. Dialog/pack direct sends need the same treatment.
Process the failure before continuing another frame/backlog batch or admitting
new gameplay. Client prediction remains client data, not server authority.

Guard settings/packs/world/host transitions at entry BEFORE earlier settings,
renderer or resource mutation and before taking owned staged inputs. Current
native-only paths may remain unchanged only with a concrete unreachable-call
audit. No save=None assertion may be reached after an entity/item mutation.

## Helper acquisition boundaries

Add immutable no-fetch construction modes to the actual skins::Resolver and
Downloader. Preserve default constructors and behavior. Disabled state requests
must return explicit unavailable/error/default-fallback behavior before maps,
cache IO, HTTP or thread acquisition; never claim downloaded data or a joined
native actor. This central guard covers remote players/mannequins, skull blocks
and held/item skull rendering. Offline login alone does not exclude these paths.

Refuse PackPush/accept/download/reload and recovery restore BEFORE staging or
Queue::open/start_pack_reload. Cover keyboard/UI/resource reload producers, not
only the eventual stream reload. No post-hoc helper drop or ignored channel.
Use complete synchronous resource preparation; bypass ordinary moved-extras and
heartbeat startup. Controller admission and silent audio must be actual code/
feature boundaries, not an assumption about user settings. Keep the inline
renderer provenance already used by the retained native window application.

## Original running window and close

Extend the existing window Application with distinct running/closing/ready remote
phases and the original Box<Game>. Native frozen presentation and its construction,
NativeWorld CloseOwner, ClientCompletion and storage gates remain native and
unchanged. A remote completion must hold the actual original Game and genuine
closed component owners; it cannot fabricate native Construction/WorldId/SaveDir.

While remotely running, route ordinary Winit gameplay input/tick/render through
the admitted Game. Existing retained control may remain its fixed status/close/
Retry grammar; no new generic RPC framework is needed. Actual input/pixels can
be exercised through the isolated native X11 window. Do not re-enable legacy
background RPC handlers. Status must not relabel remote native quiescence as
world durability: native world_closed stays false for this no-storage recipe.

Close on disconnect, user leave/quit, window close, signal, limits, startup error,
send/decode/mesh failure or render error. Permanently fence Game admission, freeze
gameplay and request the original Session/decoder/streamer with absolute deadlines.
Continue per-frame control/status and drawing when possible while polling only
finished original handles. Keep original Game/resources/window/renderer and all
queued/active/stored/refused/unprocessed payloads owned until checked outcomes.
No legacy disconnect join, new executor, replacement worker, fake NativeReady or
successful exit from an elapsed timeout/Disconnected marker.

Repeated close preserves original waits. Retry requires the current parent
attempt and applies only to currently blocked original component waiting; retain
per-component actual attempts and expose partial/still-blocked status honestly.
Never reopen admission, restart, replay or erase errors. Quiescence has priority
over expired waits. Control can seal only after its final required response/frame
service, then retain its actual closed endpoint outcome. Terminal error remains
visible for deliberate acknowledgement using existing genuine-input rules. Ready
leave may retain an inert disconnected view, not create a new title worker.

Session State operations, encoding/allocation, CPU decode/light and native surface
acquire/present are indivisible; no hard per-call latency/global quiescence or
memory claim follows. Actual native IO remains on original owned actors.

## Maximum source whitelist

Exactly 24 paths under crates/horizon-client/src; unchanged paths can be omitted.
No wildcard, library net/streaming/control, protocol/Hub, dependency/lock, launcher,
public docs, reference/playable or mod-domain changes are released here.

```
game.rs
skins.rs
game/remote.rs
game/remote_chunks.rs
game/streaming_owner.rs
game/window_close.rs
game/window_close_tests.rs
game/control.rs
game/debug_keys.rs
game/dialogs.rs
game/server_packs.rs
game/packs.rs
game/options.rs
game/screens.rs
game/render_tick.rs
game/events.rs
game/entity_culling.rs
game/maps.rs
game/lan.rs
game/persist.rs
game/integrated/managed_application.rs
game/lan/connected_recovery_tests.rs
game/retained_remote.rs (new)
game/retained_remote_tests.rs (new, declared only by the new module)
```

Stop for exact scope review before another path or a required change to a recently
qualified library contract. Keep domain/machine rules in the private mod.

## Meaningful qualification and stop

Use actual preloaded resources, original Game factory and original native actors.
Group bin tests around preflight/constructor absence of native storage/title pool,
ordered Joined/Login/chunk admission and owned Busy/refusal, fixed profile/helper
guards, and original partial/error/deadline/Retry/closed transitions. Use real
TCP/codec peers for concrete stalls, not echo decoder/mesh workers. Validate
default skin helper behavior with a focused existing local-HTTP regression and
disabled behavior without contacting external services.

Run an actual isolated Horizon server and ORIGINAL nested-X11 client window for
normal login/terrain/real movement/rendering and deliberate close; distinguish
the fixture server's legacy cleanup from C0e3 server ownership. Add selected real
window startup/disconnect/render-failure/blocked-owner checks needed by changed
paths. Reuse valid component tests at actual pins; do not replay all prior save/
Java/decoder/streamer/network/fatal batteries. New ownership backstop changes need
their concrete exception control; unchanged library Drop proofs remain at pins.
Select the smallest native-window/cold-authority regression for shared App changes.

Root alone runs serial pinned formatting, paired all-target lint, feature-off
compatibility, actual bin artifact build, exact tests/smoke and public-tree checks.
Bind sources/recipe/profile/features/binary/resources and exact peer/window PIDs;
missing fixtures, timeout, nonzero/missing pass or unobserved native completion
fails. No process timeout stands for graceful native closure. Counts are a plan,
not a cap; no performance gate or G1 decision from this component.

Fresh independent Sol High DESIGN acceptance, synced private checkpoint and exact
root release precede source. SOURCE/tool precedes any formatting/compiler/runtime;
renew formatted binding. Preserve every packet source/prefix before corrections,
archive mutable execution/config/recipe/instruction inputs and distinguish raw
Git equality from unchanged baseline canonical comparisons. Preserve previous
freezes. DIFFERENT FINAL/privacy before reviewed public sync, with fresh exact
visibility/origin and normal hooks. No PR, merge, tag or release.
