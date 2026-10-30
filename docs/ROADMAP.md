# Development stages

The supplied specification is the planning baseline for this setup. It does not
authorize starting all stages at once. Proposed interfaces, budgets and decisions
remain proposed until their corresponding gate accepts them.

| Stage | Outcome | Current state |
| --- | --- | --- |
| S0 | Runnable pinned reference, repeatable traces and bounded host contract | G0 accepted for bounded experiments; performance qualification open |
| S1 | Persistent synchronized generic machine with retained animation | Bounded host/counter/slot/identity foundations qualified; normal supported-machine lifecycle/coherent recovery, external admission/replication and retained presentation/G1 remain open |
| S2 | Verified motor, shafts and two cog sizes | P02a/b, reference observations, static XYZ/equal-cog geometry and declared three-path event arbitration accepted in native/Wasm; general arbitration and full P02/integration open, live integration requires G1 |
| S3 | Five-component playable milling MVP and two-client evidence | Planned; G3 open |
| S4 | Obtainable survival power and a small factory | Future scope |
| S5 | One controlled moving assembly with collision and recovery | Separate feasibility decision |
| S6 | Additional machinery, trains and logistics | Separate feature decisions |

The S3 component boundary excludes a gearbox. The earlier audit's suggested gearbox
demonstration is superseded by the supplied five-component scope.

## Next bounded packet

The active packet is native session checkpoint qualification in
`docs/design/S1_SESSION_CHECKPOINT_PACKET.md`, with the accepted current-resident
save clarification in `docs/design/S1_SESSION_CHECKPOINT_RESIDENT_SAVE_CLARIFICATION.md`.
The exact branch, current source and test status are in `docs/CURRENT_WORK.md`.
It uses actual native streaming/save history and participating player/counter/slot
owners; its save/recover success is not yet qualified. No ordinary startup, close
or filled-slot caller is enabled by this packet.

Prerequisites now have separate local final acceptance: producer/writer fence
(ea96036), H09a compatibility snapshot (3ac2d4c), H09b.1 frozen inputs/wire data
(765dc8d), H11a retained diagnostic (70fd6aa), and closed debug writer controls
(ff69d5f). Their respective result summaries under `docs/reference/` preserve
scope, checks and limitations. Data-only H09 and manually owned H11a do not provide
live admission, replication, committed presentation or G1.

Remaining S1 outcomes and the first-experiment bounds are mapped in
`docs/design/S1_REMAINING_WORK_AUDIT.md`: ordinary machine lifecycle and coherent
save/recovery, external admission/replication, retained committed presentation,
then integrated and dedicated with two exact-match clients. G1 remains open;
live S2 integration requires its acceptance.

## Foundation history

The initial P01/H01 packet made the pinned environments runnable, rechecked the
overlapping Horizon drafts, repeated six direct-drive/cog/milling fixtures and
drafted the host contract. See `docs/reference/S0_RESULTS.md` and sections 8 to 11 of the specification
for requirements and acceptance identifiers. The trace schema is an S0 design output,
not a supported public format. Its expanded corpus and gate decision follow below.

Record what G0 needs to decide: implementation provenance, engine integration route,
namespace/package ownership, benchmark hardware and asset choice. The user's personal
reference-use authorization and no-Create-contributions intent are already recorded
in AGENTS.md. Horizon upstream acceptance and asset publication are not assumed.

The expanded 15-case corpus repeats with 1,730 samples; the owner-assisted motor
check and retrieval/overflow cases are recorded. G0 accepts the experimental H01
direction, local required engine build, original diagnostic assets and named D0
hardware in `docs/design/G0_DECISION.md`. B2/GPU budget ratification remains explicitly
deferred; it is not a performance qualification or release waiver. The first P02
kernel and selected native/Wasm checkpoints are complete. H02 local discovery and
activation and H03b six-state materialization now pass integrated/in-frame real-game
smoke. See `docs/reference/H03_STATIC_MATERIALIZER_RESULTS.md`. The completed
`docs/design/H05_NATIVE_COUNTER_PACKET.md` and its Game proof pass 362/347
assertions in integrated/in-frame modes. See `docs/reference/H05_COUNTER_RESULTS.md`.
The save-completion prerequisite now passes both modes in
`docs/reference/H08_SAVE_COMPLETION_RESULTS.md`; proto callback errors pass in
`docs/reference/H08_PROTO_SAVE_ERRORS_RESULTS.md`. Finished handoff and strict POI
writer results now pass, with a separate focused vanilla chest/entity reader.
Map/random, scoreboard and bossbar/schedule metadata now pass both modes and broad
regressions. Direct dirty-marker handoff also passes, with explicit real queue
faults. Shared world clocks pass, including actual blocked-write/repair in both
modes. Borrowed fight/trader/arrival errors also pass. Player JSON native, both-mode
counter, focused integrated-owner fault, broad checks and final independent receipt
review pass in `docs/reference/H08_PLAYER_JSON_METADATA_RESULTS.md`. Checked external
player NBT writer native faults, both-mode regressions and broad checks pass in
`docs/reference/H08_PLAYERDATA_WRITER_RESULTS.md`. Evaluator and host phases pass
their focused checks, native both-mode games and expanded broad pipeline in
H05_GUEST_EVALUATOR_RESULTS/H05_HOST_PHASE_RESULTS. Real Rust SDK componentizer,
paired activation and public evaluator prerequisites also pass. Actual guest Game
dispatch, normal/fault trials in both modes and final independent receipt reviews
pass in `docs/reference/H05_GUEST_GAME_RESULTS.md`. The detached native two-slot
core passes source review, focused tests and broad checks; see
`docs/reference/H06_NATIVE_SLOT_TRANSFER_RESULTS.md`. Actual World slot ownership
passes native tests, broad checks and final independent receipt review;
see `docs/reference/H06_WORLD_SLOT_OWNERSHIP_RESULTS.md`. Synchronous dimension-
transition ownership also passes actual headless Game faults, both-mode full
counter tests, broad checks and final independent receipt review in
`docs/reference/H08_DIMENSION_TRANSITION_OWNERSHIP_RESULTS.md`. The
resident checked receiver also passes native, broad and final independent review in
`docs/reference/H06_CHECKED_RETIREMENT_RECEIVER_RESULTS.md`. The dormant real
player-cell adapter passes in `docs/reference/H06_PLAYER_CELL_ADAPTER_RESULTS.md`.
Checked raid-save propagation passes in `docs/reference/H08_RAID_METADATA_RESULTS.md`.
Native player-owner roster preflight passes in
`docs/reference/H08_PARTICIPANT_PREFLIGHT_RESULTS.md`. Borrowed player NBT/JSON
projections pass in `docs/reference/H08_PLAYER_PROJECTION_RESULTS.md`. Retained lazy
raid ownership passes in `docs/reference/H08_RAID_LIFETIME_RESULTS.md`. A bounded
entity-owner observation passes in `docs/reference/H08_ENTITY_OWNER_OBSERVATION_RESULTS.md`;
it is not a complete checkpoint barrier. Bounded private static geometry passes
in `docs/reference/P02_STATIC_GEOMETRY_RESULTS.md`; selected Y/equal-cog results
pass in `docs/reference/P02_VERTICAL_EQUAL_COG_RESULTS.md`. Failed parked saves
now restore the starting dimension, accepted in
`docs/reference/H08_PARKED_SAVE_RESTORATION_RESULTS.md`. The analogous idle-close
correction now passes independently reviewed native, both-mode Game and broad
qualification in `docs/reference/H08_IDLE_CLOSE_RESTORATION_RESULTS.md`. The next
WORLD/container/deferred-state observation now passes focused, feature and broad
qualification with final independent acceptance in
`docs/reference/H08_WORLD_OWNER_OBSERVATION_RESULTS.md`. It is named presence only,
not global checkpoint readiness. A bounded native capture/recovery mechanism cell
now passes native/feature/broad qualification and final independent review in
`docs/reference/H08_CAPTURE_MECHANISM_RESULTS.md`. The bounded native counter
identity/restore cell now passes native/Game/feature/fresh both-mode GUI/broad checks
and different final independent review in `docs/reference/H08_MACHINE_RESTORE_RESULTS.md`:
actual3/T state restores with fresh runtime authority and resumes4/T+1. Runtime
admission, general machine/item-slot recovery and recapture remain open. The next
library slot codec/install/lifecycle prerequisite now passes native/retained Game/
feature/fresh both-mode GUI/broad checks and different final independent review in
`docs/reference/H08_NATIVE_SLOT_RESTORE_RESULTS.md`. It restores a positive7-item
payload with fresh authority and retained heads. Its bounded active Game epoch
now passes all acceptance gates in `docs/reference/H08_SLOT_EPOCH_RESULTS.md`,
accounting the slot while preserving all remaining E1/W1 refusals. Pending wire,
general runtime/history/full scalar support remain open. S1 deliverables are
unchanged. The private declared-
event arbitration trial now passes native/Wasm against both retained Java captures,
controls and regressions, with final independent acceptance in
`docs/reference/P02_DECLARED_ARBITRATION_RESULTS.md`. It remains a closed-path,
settled-state experiment with declared native scheduling and domain effect records.
Coherent recovery remains separate in `docs/design/H08_CHECKPOINT_FEASIBILITY.md`;
completed arbitration and horizontal observations are in
`docs/reference/P02_ARBITRATION_RESULTS.md` and
`docs/reference/P02_HORIZONTAL_RESULTS.md`.
G1/G2/G3 remain open.
