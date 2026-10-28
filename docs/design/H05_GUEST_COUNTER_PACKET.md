# H05b proposed import-free guest counter research

Design proposal, not an implemented or accepted callback API. The coordinator
selected inventory-free staged guest-counter research under the existing G0
direction while H08a12 qualification finishes. Source inspected at engine
`f7f8990dbeaad2b4d036f2062d5a6dc0194c2cf4`, tree
`8db0afb2ddf457b3ec529ab96f9e49fdc1b12ae1`. Require fresh independent read-only Sol
High design review accepted the isolation/token model without blocking findings.
Coordinator accepts only the evaluator unit below for initial implementation;
host-token and Game wiring each need their own stable candidate review. No public
publication or persistence is authorized.

Advance the original generic counter using one optional pure computation export
from its captured paired server component. Preserve the existing native trial and
qualified driver as the comparison. One type, one full cube, one committed u64
counter, one eligible per-level phase; no slot, inventory, Create behavior, save,
durable generation, migration, external networking or retained animation claim.
Restart remains disabled/transient. H06/H07, H08 and G1 remain open.

## Existing source constraints

- `horizon-mods/src/runtime.rs::checked_component` already examines
  `Component::component_type().imports(engine)`, checks manifest permissions and
  API levels, builds `instantiate_pre`, and checks exact synchronous export types.
  `CheckedPair::component(Side::Server)` exposes the compiled captured server half;
  `PreparedBundle::{identity,code,bundle,content}` expose captured identity/code.
- API line 0.3 still uses WIT package `horizon:mods@0.1.0` and the mandatory
  `server-events` interface. `check_optional_callback_types` currently handles
  client UI exports only. No machinery callback currently exists.
- Paired `call_side` now discards queued `Output` on trap, but that is insufficient
  for this experiment: `storage::{set,delete}` and `log.write` act immediately.
  Merely declining to lend a WorldView does not remove those imports. Existing
  `start_component` calls ordinary `server-events.init` and builds full host State.
- `counter_trial.rs::Trial::advance` owns eligibility/state/duplicate-phase checks
  and commits counter plus last_advanced together. `World::advance_counter_trial`
  resolves actual resident storage. `game/counter_control.rs::tick_counter_trial`
  runs at tick_world's tail and requires running, ticket block-ticking and actual
  lit storage with its resident neighbourhood. It refuses remote/client replicas.
- Captured `content/machines.rs` owns type/schema metadata and primary-owner
  binding; callbacks must not register a competing type. Current configuration
  requires save-schema 1, no state properties and one captured iron-block binding.

## Selected diagnostic isolation

Use the **same captured server.wasm**, with an optional export, in a separate
empty Wasmtime linker and fresh Store for each eligible invocation. Reuse the
runtime's Wasmtime Engine/compiled Component facilities; do not compile on every
tick. Store data contains only StoreLimits. It has no host State, paths, output
queue, WorldSlot, players, storage, clock, randomness, log, WASI or other imports.
Reject any actual component import, including log, before instantiation. An empty
linker is the enforcement boundary, not a manifest claim of purity.

The evaluator prototype limits each request identifier string to 256 UTF-8 bytes
before Store creation and clips engine diagnostics through the existing 512-character
helper. This is a local diagnostic bound, not a general identifier policy. Its
fresh Store allows one memory, two tables, 16,384 table elements and 16 instances,
with the existing 64 MiB per-memory limit and trapping growth refusal.

The ordinary paired package must still pass both mandatory side preflights and
normal activation. Optional diagnostic selection additionally requires the exact
captured server half to have zero imports and the exact synchronous diagnostic
signature. Missing export or imports refuse diagnostic enablement. They do not
make unrelated ordinary API 0.3 packages invalid or auto-enable machinery.
Client-only copies of the export never satisfy server selection. No third artifact,
new package format, general capability or API 0.4 adoption is needed.

Instantiation can run component/core start code: set fuel and the limiter BEFORE
it. Install one existing FUEL_TICK budget before instantiation and do not refill
through step or required post-return work. Fuel meters Wasm execution, not all
host work; fixed-scalar replies bound host result validation. Propose the existing
64 MiB per-memory limit and 16-instance limit, with one linear memory, two tables
and 16,384 elements per table for this new isolated Store. These diagnostic limits
need concrete fixture/limit tests, not a stable SDK promise; stop and review a
necessary limit change rather than claim an aggregate cap from memory_size alone.
No init, ordinary tick, command or frame callback is called in this Store. Finish
post-return and drop the Store before making a host mutation. No failed Store or
guest globals survive. Host-generated bounded failure diagnostics may be recorded;
guest-authored log writes are impossible through this linker.

The current paired_activation server imports storage/players/world and therefore
cannot serve as this pure fixture unchanged. Keep it and the frozen native smoke
driver unchanged. Author a minimal paired fixture with mandatory server-events,
no registered ordinary hooks/commands, zero actual imports, and the extra export.
Use actual component type inspection to prove zero imports; do not assume Rust
dead-code removal or WIT declarations establish it. Stop before Game wiring if the
normal SDK/componentizer cannot produce that component without broader changes.

Fresh instantiation deliberately trades efficiency for a small isolation proof.
Record its cost without claiming a performance target. If it is impractical,
return a separate proposal for a retained isolated evaluator; do not quietly
reuse the ordinary mutable server Store or introduce a runner registry.

## Proposed local diagnostic wire shape

Define an explicitly experimental optional `counter-trial` export through local
WIT/bindgen or derived component records, without editing shared SDK WIT or its
generated reference. Do not add it as a required export of existing
horizon-server-mod or change ServerMod defaults. The test guest can bind a fixture
world containing both exports without a general SDK framework. Reviewed shape:

The local exported instance name is literally `counter-trial`, without a package
qualifier. The WIT below describes its structural types; the later real guest
fixture must prove the exact literal export through an inline export/binding.

```wit
interface counter-trial {
    record request {
        world-instance: u64,
        dimension: string,
        type-id: string,
        schema: u32,
        x: s32, y: s32, z: s32,
        instance: u64,
        counter: u64,
        game-time: s64,
    }
    record proposal {
        world-instance: u64,
        instance: u64,
        game-time: s64,
        expected-counter: u64,
        next-counter: u64,
    }
    enum rejection { rejected }
    step: func(input: request) -> result<proposal, rejection>;
}
world counter-trial-host { export counter-trial; }
```

Only host-generated validated strings enter the request; the reply contains no
strings, lists, arbitrary bytes, world operations or payload schema. Accept only
`next-counter == expected-counter.checked_add(1)` and exact echoed scalar identity
and time. Type, schema, dimension, position and component identity remain bound by
the private host token; they are not delegated to a guest claim. Overflow faults
before invocation, preserving current native behavior. The caller cannot choose
another package or executable path through reply data.

This is a local diagnostic ABI, not a stable public SDK promise. No existing
host-import function, shared SDK WIT, generated reference or manifest API line changes.
Explicit synchronous structural typechecking must precede instantiation; a later
get_typed_func check alone cannot establish refusal before start executes.

## Host prepare, evaluate, checked commit

Introduce a concrete private phase token/read projection in counter_trial.rs,
not a general transaction framework. It captures full configuration identity
(including authority scope/dimension, type/schema, position/state, selected package
digest for guest mode), instance, current counter, previous last_advanced and the
current game_time. It is invocation-local and not a saved generation/replay key.

Prepare checks current fault, running, ticket/storage eligibility, record presence,
residency, actual bound block and duplicate game_time before any guest invocation.
No token means no invocation. Do not mark last_advanced or increment during prepare.
The native path must retain its observable behavior; native advance may share
concrete checks only after its existing regression tests establish equivalence.

Evaluate uses only the exact selected running paired server entry and matching
PreparedBundle identity. Return a staged proposal or a bounded failure. It never
calls Mods::call/call_side, drains Mods::out, lends world maps or issues commands.

Commit re-resolves current authoritative World state and rechecks the entire token,
running/eligibility, exact configuration/package identity, instance/counter/previous
last phase/current time, resident storage and bound block. Validate every reply
field plus checked +1, then change counter and last_advanced together. A duplicated
or stale token cannot advance twice. A stale result cannot fault or alter a newly
reconfigured/replaced record. There is no automatic retry from guest local state.

A trap, fuel/memory failure, guest rejection, invalid reply or unavailable selected
server leaves committed counter and last_advanced unchanged. Latch a concrete
diagnostic fault only against the still-matching trial; do not silently fall back
to native advancement. A failed trial makes no later calls. Explicit off/on is
the existing retirement/new-transient-instance route, not a retry of an old token.
Ordinary block replacement/removal must remain effective even when faulted.

## Review and implementation split

**First unit: pure evaluator and actual components, no host/Game qualification.**
Bound source to a small horizon-mods runtime counter evaluator module and minimal
runtime.rs/lib.rs wiring, plus concrete component tests. A test-only direct wat
dependency may reuse locked 1.254.2 for authored component fixtures; no broad
fixture framework or production dependency is implied. Freeze the exact file set
before source review. Do not change shared SDK WIT, existing guests, native counter,
World, Game or qualified tools. Public construction must select only the captured
server Component from a checked pair; bare component seams may remain private for
actual Wasm unit tests. Prove ordinary SDK/paired activation buildability before
the later Game unit, not from bare component evaluator tests.

**Second unit: host token/World, no Game qualification.**
Add the concrete prepare/checked-commit seams and failure handling described above
in counter_trial.rs and narrow world.rs methods/tests. Preserve native advance and
controls. Review the stable delta and re-run relevant native regressions before
integration. No general transaction framework or saved records.

Meaningful checks use actual components, native World/state seams and real limits:

- Correct +1 and exact request projection; two fresh evaluations demonstrate no
  guest global counter survives. A guest deliberately dependent on retained
  globals cannot become an alternative authority.
- Missing/wrong/async optional exports, client-only export and each prohibited
  host import (world, storage, log, players and WASI) refuse before start executes.
  Existing paired activation/preflight cases remain unchanged.
- Trap at start, trap before return, infinite loop/fuel, memory/count/table limits and
  failed post-return/lifting never yield an accepted result. Check sentinel world,
  storage and log sinks/output queues remain unchanged; refusal cannot execute a
  malicious imported start. Do not substitute a native closure for all Wasm cases.
  Include an exported realloc loop/trap during request lowering to verify the
  shared fuel budget. Sentinel assertions cover diagnostic preflight/evaluation,
  not ordinary paired activation's existing effects.
- Fixed invalid scalar cases: wrong world/instance/time/expected value and +0/+2/
  overflow proposal. On failure retain exact committed counter and last phase.
- Native token tests: replacement/reconfigure, wrong scope/type/schema/state,
  changed counter/phase/time, unloaded/ineligible state, duplicate commit and a
  stale failure after a new instance. Assert no mutation/fault of the new record.
- Duplicate/ineligible/frozen/no-record phases make zero evaluator calls; native
  exhaustion remains explicit. Existing native lifecycle tests continue passing.
- Feature combinations keep paired native counter usable without runtime; runtime
  evaluator exists only with both runtime and paired features. No no-mods fallback
  may accept a requested guest backend or execute a component.

**Third unit: separately reviewed Game selection and lifecycle proof.**
Bound likely wiring to game/counter_control.rs, its existing observation routing,
the per-World trial mode/configuration and the runtime's captured-entry selector.
Keep `counter-trial on TYPE X Y Z` native. Proposed explicit `guest-on` selection
must resolve the captured primary owner/type and reject unsupported imports,
missing/failed/elsewhere server activation, wrong digest and unavailable runtime.
Never substitute the primary client Store in in-frame mode. Require an actual
running server half: paired Status::Running can describe client-only activation
and is insufficient by itself.

Backend identity belongs to each World-owned trial and follows its swaps/retirement,
not one Game-global selected runner. Cache only compiled immutable code if needed;
fresh call Stores are ephemeral. Existing dimension switch, idle-level close,
world close/open and shutdown retirement paths remain authoritative. Decide the
exact cached-program field/selector seam in this second packet after the first
unit proves the evaluator; no framework/map of live guest machines is required.
Status should expose explicit backend/call/fault evidence without changing the
existing native fixture contract. No automatic guest mode at package activation.

Use a new focused private driver, fresh worlds/homes and frozen build/fixture/source
identities. Keep existing qualified counter/parent drivers immutable. Run default
integrated server first and in-frame second: actual bound-block admission, exact
one-step results, native comparison, pause/freeze, loaded-but-ineligible, dormant
reload/no catch-up, replacement/stale key and dimension swap/destruction. Prove
integrated client local-status stays disabled. Trap/rejection/invalid-output fixture
runs must leave the last committed record unchanged, stop later calls, preserve
ordinary block removal and restart disabled. Capture real callback-count evidence,
not merely visually identical rotation or a successful command hook.

Coordinator owns serialized builds/tests and actual games after fresh reviews.
No candidate execution, engine edits, publication, public push or gate promotion
belongs to this design assignment. A general callback/slot API, retained evaluator,
rollback of existing API 0.3 effects or persistence requires its own reviewed packet;
material expansion beyond the accepted experiment returns concrete options/cost
to the owner. The restricted pure-compute diagnostic needs no new owner policy.
