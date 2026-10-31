# Managed application lifetime holder proposal

Status: PROPOSED; planning only, no source authority. The connected-worker bridge
is SOURCE CLEAR at the coordinator's current source3d64 input and is validating;
it has no FINAL acceptance or committed successor pin yet. Begin no holder source
until that bridge has independent FINAL acceptance, a clean local engine commit
and a private evidence checkpoint. Record that exact successor and obtain fresh
independent Sol High acceptance of this exact proposal before authoring source.

The owner's continuing full-S1 instruction remains active. Packet boundaries are
review gates, not permission to abandon remaining S1 work. No public publication,
later-stage authority, ordinary-playable activation or G1 completion follows.

## Outcome and explicit limits

Make the existing managed worker usable by an actual window event loop without
moving its native Game, Guest/GuestPlay, authority or connection-thread ownership
out of the worker. A process-scoped lexical holder survives presentation changes,
shows current close/status/retry state and joins only an actually finished worker.

The activation is an explicit isolated frozen test application: MemoryOwner is
the real window client's offline profile; the same application explicitly owns a
second real headless MemoryGuest client. Both use the existing fixed memory pair,
preconfigured pinned packages and a previously captured positive Survival source.
A status window with the actual owner's native connection is sufficient for this
packet; terrain/player admission must be observed in that real client, but a world
scene, animation or interactive gameplay is not claimed merely because it is loaded.

Do not add another headless CLI route whose only purpose is to repeat synchronous
supervisor tests. The new outcome is persistent pollable ownership plus a real
window, frame event routing, explicit controls and application-exit handling.

No Running transition, simulation tick, machine-slot ingress, arbitrary player
roster, authenticated profile, live TCP admission, reconnect, forced release or
late-departure cleanup policy is added. ReadyFrozen remains frozen. CleanupBlocked
remains retained. Creative attributes and live admission have their own audits
and require separately accepted designs. The ordinary default UI and playable
saves remain on their existing paths until a later roster/Running adapter.

## Actual seams and conflicts

These line references describe c95ba7e plus the bridge draft; re-resolve them at
its clean accepted successor before source work.

- game/integrated/managed_stop.rs:243/250 owns a synchronous supervisor loop.
  Worker spawn is immediately guarded at272; only is_finished permits taking and
  joining at291. A blocking EventLoop::run_app inside the driver would prevent
  bounded supervisor turns, so it cannot serve as the UI integration.
- game/integrated.rs:65 defines arbitrary Task(FnOnce(&mut Game)); call at99
  exposes that mutable worker callback. Integrated::halt at123 joins without a
  completion precheck and Drop at141 calls halt. The managed worker must not be
  installed in Integrated, Serving or that task channel.
- game.rs:15202 App contains Loading or a replaceable Game. run_event_loop at14951
  owns Game directly; run_loading at15278 replaces presentation on the same event
  loop. The managed lifetime owner must surround presentation rather than become
  an optional field that world-close, App replacement or Game teardown can take.
- game.rs:9318/9331 save_for_exit retires trials even when its attempt fails,
  consumes Integrated at9343 and can disconnect a remote at9355. Game::exiting
  at13558 calls it after the event loop is already exiting. These are unsuitable
  for an unresolved managed session.
- saving.rs:47 and120 request/finish legacy integrated stop. remote.rs:539/556
  disconnects the client and takes Integrated; remote disconnect events at945
  invoke that cleanup. Managed UI close cannot enter these paths.
- Game::window_event processes debug keys and mod input before ordinary screen
  handling; its redraw at13290 advances clocks, ticks, extras and input. Ordinary
  remote.rs:3510 tick_remote sends ClientTickEnd. Frozen recovery prohibits that
  packet. Pausing, a boolean screen flag or disabling WASD is insufficient.
- remote.rs:783 poll_stationary_client is the existing actual packet/terrain/
  teleport/PlayerLoaded pump without movement ticks. It is the proposed client
  pump, with current coalesced-state checks before use.
- game.rs:506 Options::read calls AuthSession::load, which reads and deletes the
  supplied session file. Refusing authenticated mode after Options::read would
  already be too late. remote.rs:360 also substitutes auth::current for supplied
  offline names, so changing --name does not establish the fixed profile.
- horizon-threads/src/lib.rs:178 installs a handler that calls libc::_exit(130)
  on the second caught SIGINT/SIGTERM/SIGHUP. That bypasses every Rust holder,
  UI exit check and destructor. Merely polling stop_requests cannot contain it.
  Managed opt-in therefore requires a narrow process signal-policy change.
- Ordinary open_world_with_identity uses Legacy and may choose integrated launch;
  managed NewClosed has no positive roster while Connected restores the fixed
  two-owner roster. Neither is ordinary launch authority for a signed-in window.

## Private pollable worker primitive

Narrowly factor the existing synchronous loop into one private concrete
ManagedWorker, retaining exactly NewClosed and Connected starts. No generic
executor, worker factory, Task, FnMut(&mut Game), host reference or public API.

Proposed operations, with names finalized during exact design review:

- start_closed(ManagedStart) / start_connected(ManagedConnectedStart): construct
  channels, spawn and immediately install the existing AbortUnresolvedWorker
  guard. Return no unguarded JoinHandle at any point.
- poll(&mut self) -> WorkerPoll: nonblocking snapshot of bounded immutable status,
  at most one owned pipe bundle and an optional classified terminal result. The
  bundle is moved once and never stored in shared status or replayed.
- control(&self) -> ManagedStopControl: a non-owning clone with the existing
  request/view operations and monotonic one-slot admission semantics.
- take_completion(&mut self): returns a terminal classification at most once,
  only after poll observed is_finished, explicitly joined and checked the actual
  returned attempt/receipt against terminal shared status. Calling it early
  returns Pending and cannot consume the handle or initiate a join.

The handle, channel receivers and completion bookkeeping stay private. The type
is not Clone and offers no into_inner, cancellation, detach, transport shutdown,
Exit or Discard. Drop never joins. Preserve the accepted live-worker fatal guard;
an unexpected unwind/return with that live worker must abort. A finished but
unjoined handle is not success: normal control flow still explicitly polls,
joins and classifies it before releasing the lexical owner. No hidden cleanup in
Drop or success based only on Stopped/Closing status is permitted.

The existing synchronous entry signatures remain wrappers around this same owner
and its bounded wait, preserving all closed and connected behavior. This is an
ownership primitive needed by the real event loop, not a second runtime route.

## Process and presentation ownership

Add a concrete FrozenApplication<'resources> lexical owner around the event loop.
It owns the ManagedWorker, both actual client Games, non-owning controls and
pending exit/error intent. It also owns the actual window client's presentation
resources. A replaceable FrozenView holds only screen/selection/diagnostic state;
replacing it cannot drop either client, pipes, worker, native lock or authority.
A lost renderer/window cannot silently replace the application with ordinary App.

Resources outlive this lexical owner using the existing process resource lifetime
pattern, or an explicit enclosing resource borrow for the synchronous entry.
Do not construct a self-referential Arc+borrowed Game struct or introduce unsafe
operations in the UI/client holder, or leak a new resource copy per screen replacement. The existing one-copy
process-lifetime resource precedent in Loading::finish is not permission for an
unbounded series of leaks. The worker continues to retain its own start resources.

Keep the accepted pollable worker independent of winit and of client Games. A
separate application driver owns the two clients and consumes its one-shot offer.
No Game reference crosses into the worker or into a public observer. The special
ApplicationHandler dispatches only to frozen application methods; it does not
forward managed events to Game's ordinary ApplicationHandler, Integrated or
ClosingSave. Ordinary App/Game behavior remains unchanged outside explicit opt-in.

All window/renderer creation and fallible client preparation should occur before
worker spawn where possible. A failure after spawn records pending error intent
and continues owner polling; no question-mark return can leave that live owner.
If a pre-Ready offer cannot be consumed, drop the returned endpoints and let the
existing coordinator refuse and complete native cleanup. If an error or client
loss occurs after Ready, retain the application and its actual owner state;
CleanupBlocked must not be relabeled startup failure or released.

## Opt-in entry and offline preflight

Propose one clearly named experimental argument, --managed-frozen-recovery ROOT,
parsed in Options::read. ROOT is a coordinator-provisioned isolated test root
containing the fixed source-world, source-archive, authority, fresh recovered
location and empty continuation layout. These are explicit trusted test inputs;
no scanning for a likely save, auto-copy from ordinary saves, migration, fallback
world creation, schema adoption or portable configuration format is introduced.
Require real absolute unlinked paths and all existing authority/separation checks.
Provisioning and the eventual engine argument spellings are reviewed with the
exact source packet; missing or invalid inputs refuse before transport admission.

Before the existing AuthSession::load branch, refuse if this mode accompanies ANY
--session-file option or HORIZON_SESSION_FILE configuration, including empty or
malformed values. Do not read, delete, log, rewrite or clear the file/account.
Also refuse auth::current().is_some before resources or admission. Enforce exact
offline MemoryOwner for the window and MemoryGuest for the owned headless client;
conflicting --name is an error, not silently overwritten account configuration.

Refuse ordinary world/server/quick-play, dedicated, in-frame, benchmark, control
socket, command/net/component harness, frame-limited exit and other incompatible
launch modes. Require the explicit isolated home/data/config/cache/temp roots used
by qualification; verify the owned root layout before resource loading. The entry
must not create or touch ordinary saves, account files or launcher state. No public
external-network refusal is weakened. Do not select --dedicated to obtain a
headless client; that would select an unrelated network/policy path.

Select the managed mode immediately after Options::read and its offline refusal,
before game::run's existing heartbeat and catch_stop_signals branches. Options::read
already precedes the ordinary handler installation at game.rs:15358. Install the
retained signal policy described below for this route; never install the legacy
handler first or overwrite the retained policy later. Skip ordinary heartbeat,
Discord presence, title loader, agent-control and hot-reload extras for this mode.
After the existing resource/package loading needed for the window, route directly
to the frozen application before start_game's normal world selection. No normal
default-launch selection is changed.

## Retained stop-signal policy

Add one narrowly named horizon_threads::catch_retained_stop_signals entry for the
explicit managed route. Reuse the existing handler installation, caught signal
set and stop-request atomics, factoring only the second-request policy selection.
The existing catch_stop_signals entry must preserve its current second-signal
immediate _exit(130) behavior for all default callers. Do not globally disable
hard exit or silently change ordinary client/dedicated semantics.

The retained entry selects count/intents-only behavior before registering the
existing handlers and before any managed worker starts. On the existing caught
Unix targets, every caught SIGINT/SIGTERM/SIGHUP updates the stop-request indication
and returns from the handler, including the second and later requests. They do
not call _exit, abort, a UI function, a save, a join, channel operations, allocation
or locks. Use existing atomics and a fixed process policy selected once; do not
reconfigure or reset it from Drop, screen changes or a UI callback. Define count
saturation/overflow behavior explicitly so a burst cannot clear a pending intent.

The frozen application translates any positive indication into its existing quit
intent. Count increases cannot submit another close while one is in flight, retry
a failed publication, release CleanupBlocked owners, or exit the process. Its
normal typed command/admission rules remain authoritative. Installing the legacy
policy while a retained application exists is outside this concrete route and
must be excluded by the single startup selection, not exposed as a UI option.

The existing non-Unix stub behavior is not upgraded to a Windows console-control
or uncatchable-signal guarantee. The UI/client holder introduces no unsafe OS
operations. The existing platform crate already contains unsafe handler/OS calls;
review the exact focused factoring and its signal-safety reasoning rather than
claiming that crate has no unsafe changes. SIGKILL and externally forced process
termination remain external crashes, never stop receipts.

## Event-loop turns, status and controls

Each about_to_wait performs a bounded owner poll, delivers the pipe bundle once,
then conditionally pumps both actual clients with poll_stationary_client. Use the
current view, not an older connected_observation: do not pump clients or inspect
native ownership as if Ready while state is Stopping/Stopped/Closing. No tick,
tick_remote, tick_mods, extras-frame callback, arbitrary package UI, player input,
world generation, movement prediction or machine transaction is invoked.

Use WaitUntil with a bounded frame/poll interval, preserving responsive native
window events without a status backlog or a busy infinite callback. Minimize,
occlusion and a lost drawable surface must not suspend worker supervision or
client admission. Keep frozen server/core clocks and hub ticks observable.

The window initially shows real connection phase plus bounded diagnostics and
native client facts: Joining/Admitted/Loading, ReadyFrozen, closing attempt,
NotPublished/Uncertain retained failure, CleanupBlocked, or completed result.
Show actual stopped status only after join/receipt validation. Render the status
panel from the existing native HUD/renderer primitives; ordinary Game::render and
redraw behavior are not presumed safe entry points. A terrain scene is optional
only after a focused no-tick/no-send rendering audit and within the same reviewed
path budget. Do not count an overlay screenshot as world rendering or animation.

Allow only native window resize/focus/redraw, status navigation, Close and Retry
input. Suppress keyboard/debug/mod callbacks, mouse look/buttons/wheel, device
motion, gamepad polling, inventory/chat/command entry and normal shortcuts before
ordinary Game code can handle them. Do not synthesize PlayerLoaded; the real
stationary helper observes loaded terrain and emits it normally.

Close records quit intent and submits one fresh RequestStop only at ReadyFrozen.
While stopping it shows progress and does not submit another attempt. During
startup it records intent and continues actual admission; once Ready it submits
that one close. A finite startup refusal instead completes with a non-success
startup result after cleanup and join. No unreviewed pre-Ready cancel is added.

RetainedFailure keeps the window/application alive and exposes explicit Retry for
the exact failed attempt with a strictly newer attempt ID. Repeated Close, Escape,
timeout, observer replacement and repeated OS stop requests cannot auto-retry.
CleanupBlocked disables close/retry release and retains both owners and leases.
The UI may explain that retained state, but offers no force-close/discard option.

## Exit and error containment

The frozen route has one explicit exit decision, after classified joined
completion. Native CloseRequested and signals caught under the retained policy set exit intent;
ordinary presentation errors record their error and keep owner supervision. A
renderer error must not invoke Game::save_for_exit, leave_server, trial retirement
or event_loop.exit while the managed worker remains unresolved. Keep a minimal
status/error view when drawing remains possible.

Intercept frame limits, ordinary Action::Quit, pending-world cancellation, control
quit, renderer/window errors and all ordinary Game exit branches by refusing those
modes or never dispatching their handlers. There is no general interception hook
for all programs; the claim is the explicit frozen entry's concrete control flow.
After run_app returns, the enclosing lexical owner must still inspect completion:
normal return/reporting to game::run or main is permitted only after explicit
finished join/classification, so main's process::exit(1) cannot abandon a worker.

An OS/application exiting callback may be unable to resume an event loop. It must
not perform a hidden join, native save or automatic retry. If no actual worker
remains, perform classified final reporting. If a worker is live, invoke the same
explicit fatal-unresolved discipline and marker as the accepted holder; do not
return as a clean shutdown. Unexpected event-loop return/error or driver unwind
with a live worker follows that rule too. SIGKILL/process termination remains an
external crash, not a close receipt or portable shutdown guarantee.

Renderer failure before Ready need not force retention forever: endpoint loss
may resolve through the existing finite pre-Ready cleanup. After Ready, a broken
window or departed client can remain CleanupBlocked; this packet provides no
policy that releases it. A required forced-quit UX is a later explicit decision.

## Proposed source budget and staging

No source is released by this document. Against the subsequently pinned FINAL
bridge successor, propose the following NINE-path maximum frozen source scope:

1. game/integrated/managed_stop.rs: concrete pollable owner extraction, unchanged
   synchronous wrappers and narrowly scoped visibility for the application.
2. game/integrated/managed_stop_tests.rs: existing closed-profile regressions and
   deterministic poll/completion/admission checks affected by extraction.
3. game/integrated/managed_connected_tests.rs: preserve actual connected regressions
   and add direct pollable-lifetime proof using the existing source fixture.
4. game/integrated.rs: declare/expose only the concrete application module.
5. game.rs: explicit argument/offline preflight and post-resource route, plus only
   minimal access needed by the frozen native presentation. Ordinary routes stay
   unchanged and managed events never enter their handler.
6. new game/integrated/managed_application.rs: lexical process owner, fixed pair
   client construction, frozen event routing and terminal/error/exit containment.
7. new game/integrated/managed_application_view.rs: bounded native status/close/retry
   view, using existing HUD/renderer facilities without gameplay callbacks.
8. new game/integrated/managed_application_tests.rs: actual application/window
   qualification plus isolated exceptional children.
9. crates/horizon-threads/src/lib.rs: focused opt-in retained signal policy using
   existing installation/handler/atomics, preserving legacy second hard exit and
   keeping focused policy tests inline.

No auth.rs, main.rs, saving.rs, remote.rs, world_loading.rs, startup_menu.rs,
protocol, schema or network-policy edits are proposed. Auth refusal occurs before
Options::read's existing auth consumption. Existing stationary client APIs are
reused. If native window/renderer setup or event containment requires another
production path, stop source expansion and revise/review this design first.

A two-stage source sequence is acceptable: first extract and independently qualify
the pollable ownership primitive while preserving both synchronous wrappers, then
activate the real frozen window using that exact accepted primitive. Both need
bounded review/source/evidence gates. The extraction is not claimed to qualify UI;
a new headless runtime command is not an acceptable substitute for stage two.

## Meaningful acceptance gates

- Pollable ownership: actual connected worker, actual two clients, nonblocking
  Pending polls, one moved offer, one classified completion, no early take/join,
  and original closed/connected regression behavior. Preserve deterministic stale
  Ready/older-retained publication race and bounded UTF-8 status proofs.
- Offline refusal: managed launch plus present/empty/malformed session-file options
  or environment refuses before resources/admission; sentinel account/session
  files remain byte-identical and unread. Existing ordinary parsing is unchanged.
  In-memory auth refuses; no account reset or signing-out operation is performed.
- Actual native window: MemoryOwner packet/terrain/teleport/PlayerLoaded completion
  with stone3 plus owned headless MemoryGuest stone5; source rows/core/slot/keys,
  zero recovery ticks and native/UUID exclusion remain. Capture real event-loop
  frame/status observations, input suppression and responsive resize/minimize.
- Close through native UI and OS stop intent: one accepted typed attempt, actual
  native close and staged joins, matching continuation validation and final worker
  join before client/window release or main returns. Repeated close is not retry.
- Actual caught-signal children on a supported isolated Unix target: prove two
  separately delivered/observed signals while the real window application owns
  ReadyFrozen, and separately while each real Copied/Promoted failure is retained.
  Use an explicit test-only command-submission barrier for the Ready proof if
  needed so both signals are actually observed before the first close can finish;
  keep polling the genuine owner/clients. Do not depend on a timing race or signals
  that may coalesce: observe the first count before delivering the second.
  After the second, prove the process remains alive, native/UUID locks and positive
  owners remain held, no extra attempt or automatic retry occurred and no stop
  receipt was fabricated. Release the Ready barrier for one typed close; retained
  cases require an explicit Retry before eventual verified joined completion.
  Separately prove the unchanged legacy handler still exits130 on its second
  caught signal in a child with no managed/native owner. Inline platform tests
  cover policy selection and overflow/saturation without shared global-state races;
  synthetic counts alone cannot replace the actual isolated two-signal proofs.
- Copied and Promoted faults on the real worker while the real window persists:
  same owners/rows/keys/locks, replace non-owning observer/view, explicit Retry with
  fresh attempt/epoch, then continuation verification and joined completion.
- A finite pre-Ready handoff/client refusal completes native cleanup before the
  application returns. Record discrepancies, resolve the authorized close/retry
  where possible, and assert normal-case outcomes after joined completion.
- Focused fault injection covers renderer/window failure before and after Ready,
  ordinary exit bypasses, unexpected run_app return and presentation replacement.
  No route may call legacy saves, disconnect positive clients or retire live
  trials merely to make the window close.
- Separate root-orchestrated children establish process-fatal application unwind/
  irreversible event-loop exit with a real live worker, and retained late departure
  through ten actual CleanupBlocked observations before explicit external fixture
  termination. These exceptional results are not ordinary libtest passes.

Root preserves ALL scoped sources and ALL packet-prefix evidence before EVERY
correction; obtains fresh independent exact-source Sol High clearance before
compilation/execution; serializes all Cargo/tests/games; uses isolated owned
homes/resources/prebuilt packages; and freezes raw/Git/tested byte identities.
A different final reviewer assesses actual window and exceptional evidence.
No public engine push/PR/merge/release, reference mutation or account operation.

## Risks and open technical checks

The design can fit a bounded path set by keeping the managed application separate
from ordinary App/Game dispatch. A transparent retrofit into ordinary Game close,
remote disconnect, input and tick behavior cannot honestly fit this scope.

Winit's exact exiting/OS surface-loss behavior needs qualification on the actual
native target; no cross-platform responsive-quit guarantee is presumed. The
existing second-signal hard exit must be removed only for the explicit retained
route before that route can claim lifetime containment. Continuing
headless supervision after window loss must never reclassify a post-Ready departure
as safe cleanup. Resource lifetime, event routing and suppression of package/client
callbacks require focused source review. Stationary polling itself handles remote
disconnect events; after Ready any resulting local teardown must leave the worker's
positive native owners/leases retained, with no success interpretation.

No unavoidable owner-policy question blocks the explicitly isolated fixed-pair
frozen application. Supporting a normal signed-in player, choosing a different
roster, entering Running or releasing a departed owner remains a later accepted
adapter. If a desired close UX requires forced post-Ready release or silently
rerouting ordinary playables, that is an actual scope/policy conflict and must be
reported rather than implemented here.

Independent design review: DESIGN ACCEPT at exact input
cb1f1496b9404120bd44f734cd015a91b5fa5fffa2bc1345ed8af12b0c156db9,
preserved as .local/s1app1-proposal2.md. Nine-path scope and primitive-first/window-
second sequence accepted. The second-signal escape was resolved in design only;
all bridge FINAL/clean-successor/private-checkpoint and per-stage source/evidence
prerequisites remain. No source release or Running/rendering/G1 claim follows.

Connected-worker prerequisite is now independently FINAL at clean local successor
7a08dc25c0e3135e3a0e0a4eaae7c11af5e1d99e, source17432ff7, reviewed result44cfad2a.
Pin this exact successor for the holder. First bounded stage is the pollable owner
extraction in exactly managed_stop.rs, managed_stop_tests.rs and
managed_connected_tests.rs under game/integrated, with meaningful direct actual
worker/client ownership proof and unchanged synchronous wrapper regressions. The
remaining application/window/signal paths stay outside this first source packet.
Private FINAL evidence checkpoint must finish before source authoring starts.

Primitive stage is now independently FINAL at exact clean successor
 a56e0136a87ef723d4866c341ccbe11ec3678bd9 (tree3f604010), source2b627334,
reviewed result9e9c7930. Pin that successor for actual frozen-window activation.
Private FINAL evidence checkpoint must finish before new window source starts.
Proposed narrowed window scope is eight of the accepted nine maximum paths:
game.rs, game/integrated.rs, game/integrated/managed_stop.rs,
game/integrated/managed_connected_tests.rs (test verifier visibility only),
new game/integrated/managed_application.rs, managed_application_view.rs,
managed_application_tests.rs, and crates/horizon-threads/src/lib.rs.
Keep Game Options unchanged; private parsed-launch wrapper avoids a tenth fixture
path. Reuse native loading_renderer/loading_frame/HUD services and full loaded GUI
atlas. The frozen window is not gameplay/world rendering or G1 qualification.
