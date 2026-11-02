# C0e2c: original in-frame window close composition

Status: HELD PROPOSAL, no source/execution authority. Predecessorbd0647a6aef9efc2484629fb36a954167b738b30,
tree471bbec26f30f863db77d6f1baf654fa5f7a29f7, control FINALFd730d8a and LOD FINAL5a388caf;
reviewed public branches and private7c8cbb6 checkpoint synced. S1/G1 stay OPEN.
Read S1_WINDOW_CONTROL_TERMINAL_AMENDMENT.md with this proposal. This is a bounded
composition component, not ordinary startup, integrated, dedicated or G1 acceptance.

## Exact source budget and physical workspace

ELEVEN paths under crates/horizon-client/src (ten production, one defining test):
1. game.rs: explicit opt-in ParsedLaunch/profile branch, original production Game
   construction and owned Box event-loop ingress; compatible private LOD/control
   adapters; actual admitted intents/error/signal/exit routes.
2. NEW game/window_close.rs: opaque original App/Box close coordinator, fixed
   presentation/restricted control operations and typed client/native handoff.
3. game/paired_close.rs: bounded production bridge to original CloseOwner and
   NativeReady, fixed UI projection and constructor-bound client completion.
4. game/saving.rs: retained close-intent routing/status HUD, no legacy save loop.
5. game/control.rs: fixed status/exact Retry/idempotent close before save rejection.
6. game/agent_settings.rs: choose known retained control before acquisition; pin
   original endpoint through close, refuse unsafe toggles/preferences changes.
7. game/world_loading.rs: production cold-prepared ownership transfer/factory,
   failure custody; do not call test-only transfer or upgrade a legacy title worker.
8. control.rs: fixed read-only adapter queries and bounded profile control policy
   needed by the real window; existing unbounded primitive/legacy modes preserved.
9. NEW game/window_close_tests.rs: defining tests declared only by window_close.rs.
10. gfx.rs: fixed paired renderer constructor selecting inline CPU occlusion
    before any renderer-owned CPU worker acquisition; preserve legacy new.
11. gfx/occlusion.rs: fixed inline constructor that never calls Worker::spawn;
    reuse the existing synchronous search fallback, no general policy framework.

No lod.rs/horizon-world/CLI-other-file/dependency/lock/default activation
change without revised scope acceptance. Existing RetainedLod fixed APIs cover
required callers. Private adapters must preserve legacy signatures/callers; if a
caller such as dedicated/status.rs actually needs an edit, revise budget first.
Use stable s1-native-close-composition, new codex/s1-window-close atbd0647a and its
warm dedicated target-s1nc1. Preserve all174 control,252 LOD and300 historical
frozen inputs; future FINAL archives mutable source/config/recipes with exact raw
live/archive/Git and actual-execution mapping. Prefix.local/s1wc1 is new.

## Explicit production profile before native acquisition

Add one opt-in launch wrapper, provisionally --paired-native-window-close, in
ParsedLaunch within game.rs, before ordinary argument/auth handling and run's
heartbeat/Discord/default signal/startup branches. Linux, client, fully synchronous
resources, fresh owned single-world NativeWorld/in-frame scope only. Plain
--in-frame does not select or prove it. No new field in every Options literal.

Before acquisition reject external authentication/account/session, server or
dedicated/default integrated topology, benchmark, command/net/component hooks,
ordinary menus/loading, packs/mods/extras/audio/controllers/network startup and
other incompatible entry. Honor No AI/control preferences by refusal, never by
rewriting user preferences. Use actual retained signal policy before any helper.
Load pinned complete resources synchronously with menu=false. Its bounded scoped
resource readers may spawn, but their actual checked joins must finish before
resource loading returns; no asynchronous rest-loader tail is admitted. Require
full GUI/rest/expected root, no fallback/static leaks.
Keep enclosing loaded.resources alive through the whole event loop/handoff.

Use fresh isolated test data; never touch playable worlds or launchers. Production
profile owns a real session/root and exact prepared SaveDir/Streaming provenance.
Derive the initial Game directly from its ORIGINAL cold-prepared retained owners,
with failure ownership retained from first acquisition. A common fixed constructor
may preserve default initialization while selecting owned inputs before the FIRST
legacy title storage/Streaming constructor. Swapping a retained streamer into an
already constructed legacy Game is not a continuous ownership proof.

Construct actual RetainedLod from the SAME streaming.lod_source allocation and
WorkerContext/capability before any Lod::new, storing the owner before start.
Construct actual RetainedControl before any Server::start. Unknown/legacy states
refuse before new acquisition, not via fields-None or path/count inference.
Startup failure after acquisition stays in an opaque retained preparation/client
failure state with live status/close service; it is not a new Game or discarded
native input. Existing accepted prepared/failed owners supply their actual receipts.

The profile is a fixed world view with real retained streaming/LOD and native
window presentation. Prevent unsupported constructors BEFORE invoking ordinary
UI/control handlers: Friends/LAN/Remote, pack reload, world replacement/dimension
entry, background loaders/prefetch, deferred Agent/UiWait/Events/screenshots/bench
or other owned work outside this budget. Ordinary gameplay/default activation and
four-family async preparation remain later. Do not call ordinary tick/handlers
that can acquire these helpers and then infer absence from resulting fields.

## Original Box, presentation and native completion

The real application retains ONE original Box<Game> from construction through
Running, closing CloseOwner, NativeReady and terminal typed handoff. Do not pass
by value through ordinary run_event_loop, reconstruct Game, clone a world, leak
resources static, expose mutable Game, or add callback/closure executor recovery.
Ownership proof derives from fixed constructor-issued objects/receipts, not an
address, free bool/enum, metadata projection or setting a field to None.

An opaque App/close boundary supplies only fixed window/event/presentation and
restricted control operations. It may hold actual UI/client owners outside the
sealed native-world owner only through a fixed original-Game extraction/receipt
path that cannot swap foreign owners or resume world mutation. No arbitrary Game
method dispatch after sealing. Freeze simulation/producer admission before capture;
UI-only resize/HUD operations must not mutate frozen save metadata.

Close real LOD before save/storage handoff; retain its original ClosedLod/failures
and all queued/active/late data. Drive original C0d CloseOwner/native writer/level/
metadata/IO/image receipts and retain original NativeReady. Preserve
try_dispose_dormant's existing guard. Add a DIFFERENT typed profile client
completion path that consumes original library closures and original window/UI
custody, rather than silently relaxing that guard or using test-only disposal.
No mutation, loader/controller/ticket or mutable Game escapes final handoff.

Route actual admitted CloseRequested, physical/control close, frame/time limit,
caught signal, render/event/entry failure and quit escalation into that same owner
and original intent/error. Repeated close changes intent only where allowed; it
does not renew deadlines or replay jobs. Explicit exact-attempt Retry is the only
retry trigger. Unsupported ordinary routes refuse in the profile. Preserve fixed
status and native HUD before generic pending-save early returns/rejections. Never
fall through legacy LAN teardown, save_for_exit or a direct expected exit.
Irrevocable process exit/unwind with unresolved owners remains fatal, not graceful.

Use the existing real closing HUD/renderer. Retain original renderer/window on
surface skip, minimize, resize or draw failure, continue close/control polls and
allow fixed Retry input. Avoid blocking screenshot readback/offscreen pacing in
close turns. Actual acquire/present and constructor/capture CPU work are indivisible;
claim no hard per-turn/GPU teardown latency. Dispose actual renderer/window only
on their owning thread after accepted typed world/client handoffs. No new GPU
shutdown witness or global driver-thread quiescence is claimed by this component.
The ordinary Renderer constructor unconditionally starts a detached CPU occlusion
worker, even with culling disabled or HUD-only rendering. This is not a GPU driver
thread and cannot be hidden by that carveout. Select a separate fixed paired
renderer constructor BEFORE acquisition which uses Occlusion's inline search
implementation and never calls Worker::spawn. No legacy renderer upgrade, runtime
toggle, replaced renderer or disabled user setting establishes this provenance.
Preserve ordinary Renderer::new and its behavior. Inline CPU search remains an
indivisible operation without a hard latency bound. All explicit profile renderer
construction, including startup/error HUD, must use this fixed nonworker path.

## Control and explicit terminal phase

Keep the SAME original control endpoint live and restricted through all world/
LOD/native-save Blocked and retry stages, with new-client reconnect. Before any
pending-save rejection dispatch only fixed bounded status, exact Retry and
idempotent close; never advance Agent/UiWait/Events or generic gameplay commands.
Control close acknowledgments mean admission, not completed ownership/durability.

The window profile must not inherit unbounded native-control admission accidentally.
Use a fixed bounded profile policy selected BEFORE acquisition: at most8 clients,
4096-byte input lines,1MiB output/client and8MiB aggregate owned pending response
bytes; checked refusal before growth, fixed10second auth/partial-line/response
operation deadlines not renewed by traffic. Keep UI dispatch to8 commands and2ms
between indivisible operations per turn. The fixed bounded constructor uses
nonblocking actual sockets and a per-frame native credit generation: at most3
accept/read/write operations per each of8 client slots, plus8 listener operations
(32 total), with a shared2ms turn deadline checked BETWEEN indivisible calls.
Unused credits expire, never accumulate; fixed slot/operation policy is not a
public scheduler/executor. Auth/partial-line/response deadlines remain independent
and are checked even while waiting for a grant; stop is out-of-band and cannot
wait for another credit. Native reads may fill an owned bounded buffer, preserving
post-auth/line tails, rather than requiring one syscall per byte. Checked slot
retirement/reuse and queued response accounting cannot hide unfinished actors or
late receivers. Refuse over-limit native/local admission before buffer growth.
Reserve an actual slot BEFORE accept: accepted-but-not-registered sockets count
toward the eight. Permit exactly ONE outstanding command/reply per native slot,
eight total across queued, handed-off, awaiting-write and late-response custody.
Bounded reads may preserve tails, but may not parse/admit a second command before
the prior request and its reservation actually retire. Preserve slot generation
until actual finished-only worker join and all late receiver ownership resolves;
no recycling an index while its old worker or reply can still act. The window
profile rejects local request injection entirely, including before native startup;
it does not grant a second local request pool or raw sender bypass.

Fixed supported status/error replies remain bounded before construction, with
one actual reply per handoff; account external/channel and queued bytes honestly,
not just bytes already copied into a writer. A caller cannot use a raw Sender or
misstated partial-line timer to bypass the profile. Preserve the accepted primitive's
unbounded mode and all legacy behavior outside this new fixed profile. If this
concrete32-operation/2ms policy cannot be implemented truthfully under actual
worker scheduling/socket semantics, withhold and revise the design before source
execution; no silent bound waiver or broader scheduler framework.
In bounded mode ordinary try_recv and RetainedSender::try_send must not expose or
inject the raw Request/mpsc::Sender path. Use a fixed typed request/reply handoff
with one-shot bounded replies and byte reservation before response allocation;
unbounded ordinary APIs refuse bounded mode. Account queued and late response
ownership until actual retirement, not only native bytes written.

WorldLeave may finish in an inert closed view with original control/UI retained;
no editable title/new world or mutable NativeReady recovery. AppQuit final control
seal happens only after fallible WORLD work is complete. The separate amendment
explicitly distinguishes later native-control cleanup/join Blocked from world
close and provides original native UI status/Retry after RPC is deliberately
sealed. Do not claim RPC after RetainedControl.begin_close, reopen/rebind a socket,
or hide failed/ambiguous endpoint release as success. If no usable native retry
surface is available, refuse terminal seal while the original RPC is still live.
After actual native quiescence, failed/ambiguous/replaced/refused endpoint outcomes
produce an original ClosedControl FAILURE handoff, not a successful release. Keep
the exact original error/outcome in an inert terminal-error window/title until a
fixed explicit native Exit acknowledgement; then dispose on the owning thread and
return the original failure. Retry is not offered for completed cleanup and cannot
repair it, respawn a cleanup worker, rebind, or delete a foreign endpoint. Only
unfinished actual work remains Blocked with exact-attempt Retry extending the wait.
Successful release or never-acquired-without-failure follows normal final exit.

## Focused qualification and gates

Fresh independent Sol High accepts exact proposal, terminal amendment and
assignment; private checkpoint/root release precede source. Authors scoped native
TEXT, root alone all serial fmt/compiler/tests/WSL/game/Git. Reviewer procedural
native READONLY under inherited write: no scripts/import execution/runtime/WSL/
edits/network/escalation/delegation. Preserve ALL ELEVEN source paths (including
absent-file markers) and prefix before corrections. Fresh SOURCE/tool before any
execution, different FINAL before consumption/public sync; normal hooks/visibility.

Select defining ownership/admission/receipt/control-budget tests plus the smallest
affected C0d/preparation/LOD/control regressions. Around12-20 new defining names
and2-3 actual-window smoke scenarios is a planning estimate, not a cap. Real pinned
resources/WorkerContext/worlds, full inventory at batch boundaries, process-isolated
resource cases. Missing data fails. Native direct cases spawn no compiler/Java.
Use native-writer controls only as fixed controls of actual workers; no stubbed UI,
echo server or test-only Game transfer as a responsiveness/native closure proof.

Run an ACTUAL visible Linux window with original Game, real LOD/streaming and both
Unix/authenticated TCP endpoint variants. Stall real native IO while resizing/
redrawing/status/reconnecting and issue exact-attempt Retry; test draw failure/
minimize, caught signals and admitted intents, closure custody and pending/late
payloads. Qualify the final control-seal phase via native UI/Retry as explicitly
amended. Driver/process deadlines fail runs, never complete owners for them.
Meaningful format/paired all-target compile/feature-off/source privacy checks,
actual pinned binary/artifact and separate unresolved-owner fatal controls apply.
No old full network/save/Game batteries. Thirty cases/10-15min warm remains a
planning prompt, not a cap; inspect costs before cold/large validation.

Stop at this explicit profile's FINAL/checkpoint/reviewed public sync. Default
integrated/dedicated/Friends/stdin/Remote ChunkDecoder, ordinary async preparation,
failed Guest and connected ordinary G1 stay unqualified. No global CloseCommit,
performance/GPU latency, C1/liveS2/PR/merge/tag/release or broader activation follows.
