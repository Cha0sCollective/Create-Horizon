# H05d authoritative guest-counter Game wiring

Implemented and qualified locally at d2816f6d7f03cec0ab326ec9ad6f1b5cd4a0ed40.
Both-mode normal/fault/native results and final independent receipt reviews pass;
see docs/reference/H05_GUEST_GAME_RESULTS.md. The bounded scope below remains the
acceptance boundary, with no persistence, inventory, performance or G1 claim.

Fresh independent read-only Sol High design review finds no blocking issue.
Actual SDK prerequisites below now pass; coordinator accepts bounded implementation.
Inspect engine `83649b8e6e80e9b8db0507009b81721816a094bf`
(host phases), including evaluator commit `fc6811a`. Native both-mode/broad checks
and private fixture buildability are separate qualification work, not this draft's
evidence. No engine/tool edits or execution are authorized by writing this file.

## Prerequisite and source boundary

Before Game code, build the reviewed tools/fixtures/guest-counter sources with the
pinned toolchain, standard componentizer and byte-identical copied SDK WIT. Inspect
the actual captured server component: zero imports, canonical server-events export,
fixture-owned inert canonical players export, and unqualified counter-trial.step.
Both ordinary paired preflights/activation and CounterEvaluator::from_pair must
succeed. The inert players export grants no host capability. Source-only or bare
WAT evaluator tests cannot establish this SDK/componentizer integration. Stop if
this requires live imports, modified canonical SDK WIT or a third artifact.

Minimal production files: game/counter_control.rs; horizon-mods/src/runtime.rs
for compiled cache/checked selection; client counter_trial.rs only for additive
GuestFailure::Evaluation. Existing world.rs host APIs, tick_world tail hook,
mutation_control.rs forwarding, ordinary SDK/API versions and persistence stay
unchanged. Runtime/native tests may use existing runtime counter tests or paired
activation test seams. Freeze that test file choice before implementation review;
split a large integration delta rather than add a general fixture framework.

## Explicit selection and immutable cache

Keep `counter-trial on TYPE X Y Z` and its native JSON unchanged. Add explicit
`counter-trial guest-on TYPE X Y Z`; ordinary activation never enables a trial.
Reuse counter_binding's exact selected single-state iron binding, resolve its
captured PreparedBundle from the admitted catalog, and call Configuration::new
then with_guest_package. The primary owner/digest come from that capture, not
from a command path, name alias, caller-provided hash or a client component.

Before World enablement, require the matching runtime paired entry and its actual
`side_run(true)` to exist, plus the same primary owner/captured identity. Status
Running alone is insufficient: client-only paired entries can be Running. Warm
one optional immutable CounterEvaluator cache on that Entry using from_pair after
ordinary checked pair preflight. Cache only compiled code/preflight metadata;
retain no Store, guest global state or per-World runner. No cache map is necessary.
Cache preparation failure refuses guest-on before enabling/changing the World.

A small Mods selector returning a borrowed &CounterEvaluator must recheck captured
owner/digest, evaluator.identity(), paired entry and live side_run(true) on EVERY
invocation. Keep availability failure distinct from evaluate's String result using
the lookup Result followed by evaluate; no error-message parsing is needed. A
stale compiled cache cannot authorize execution after server stop/failure/reload.
Paired reload already requires a catalog restart. Clear caches with entry reset
where practical, while live-side checks remain the authority even if code survives.

Backend identity lives in each World Configuration and follows existing swaps,
dormancy and retirement. There is no Game-global selected backend alias. Guest
snapshots may add a backend owner/digest field only for Guest configurations;
Native snapshots/status and retirement JSON retain their existing shape.

The current host guest-binding constructor refuses namespace aliases: this bounded
trial supports the primary namespace only. Resolve the actual captured primary
owner before calling it; do not infer a primary owner from a namespace string or
broaden the host restriction as part of Game wiring.

## Authoritative dispatch and failure boundary

tick_counter_trial keeps its current remote/integrated-client refusal. Match the
active World's Configuration backend: Native uses the unchanged native call;
Guest follows prepare -> isolated evaluate -> fresh checked commit/fail. In-frame
selection still uses the server half, never Entry's primary client run. Builds
without mods refuse guest-on; native paired support continues without Wasmtime.

Prepare uses fresh running and counter_position_eligibility's ticket && storage.
No token means no lookup/evaluation. Project CounterRequest exclusively from the
token's configuration, record and time, including full scope/dimension/type/schema/
position. Use the runtime selector immediately before evaluate; it must not pump
events, call ordinary Mods hooks, lend World maps or drain ordinary outputs.

Convert a successful runtime CounterProposal field-for-field into the host's plain
Proposal. Recompute eligibility before World commit/fail, using the still-current
running phase flag (no event pumping/reentry may change it during evaluation).
World rechecks configuration, instance, counter, last phase, time, actual storage
and block. The synchronous call admits no simulation interleaving, but the checked
host boundary remains required. Never native-fallback after unavailable/failed guest.

Missing live server/cache -> GuestFailure::Unavailable. An Err(String) from
evaluate -> additive GuestFailure::Evaluation, with clipped contextual diagnostics.
Do not guess Trap/Fuel/Rejected/InvalidReply from text: the evaluator already
rejects malformed scalar output, so a +2 fixture reaches Evaluation. Host proposal
validation retains InvalidReply as its separate defence. Existing host sticky
failure logic retains counter/last_advanced; stale results cannot fault another
trial. Log only the newly latched failure, with finite category and bounded detail.

## Bounded invocation receipts and meaningful proof

Use explicit opt-in stderr receipts, e.g. HORIZON_COUNTER_GUEST followed by JSON,
emitted once only when evaluate was actually called, including an error during
start/lowering/body/post-return. Availability refusal has no invocation receipt.
Include captured package identity, scope/dimension/type, instance, game_time,
input counter/request, valid proposal or null, and final commit/skipped/stale/fault
outcome. Clip error context; bound each receipt and ignore sink failure without
changing committed gameplay. Default off. Add no World counters, maps or queues
solely for testing. A receipt proves evaluator invocation, not necessarily reaching
the Wasm step body. Parse full identity/time keys and reject duplicates in evidence.

Use the explicit HORIZON_COUNTER_GUEST_RECEIPTS=1 environment gate, cached once per
process for logging only. Keep each serialized receipt within 8 KiB; unavailable
output must not change gameplay or masquerade as evidence of a call.

Native tests must cover selector refusal for client-only Running, wrong capture,
missing/failed server and stale cache after server removal, using actual component/
activation seams. Check fresh host request/proposal conversion and native JSON
compatibility. GuestFailure::Evaluation is additive; existing host fault tests persist.

Run the immutable full native counter driver in both modes. A new private focused
driver uses fresh homes/worlds, frozen game/build/resources/fixture hashes and real
controls: explicit guest-on; exact +1 step/phase; invocation identity/count; zero
receipts during pause/freeze, inactive/dormant phases and after fault; replacement/
stale keys; unload/reload without catch-up; integrated replica disabled; dimension
swap/destruction/retirement and restart disabled. Assert actual position and current
ticket/storage eligibility, not only a teleport acknowledgement. Keep old drivers.

Separate reviewed fixture features may trap, reject or propose +2 at input counter
3. Build/inspect/capture each variant independently; all stay import-free and inert
outside step. Each actual failure must retain committed counter 3/last phase,
stop later invocations, allow ordinary removal and restart disabled. No imported
command/storage/world/log fallback or runtime fault-mode control is permitted.

Stop/rescope for new lifecycle ownership, retained guest Stores, another artifact,
general callback/transaction API or persistence. Coordinator owns serialized builds,
games and commits after review. This remains transient inventory-free G0 research:
no H06/H07 slot, H08 checkpoint, retained animation, external peers, G1 or public push.
