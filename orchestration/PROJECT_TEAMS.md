# Create teams — operating plan

Status: adopted November5 after fresh independent read-only Sol High review CLEAR,
including the Transport HOLD activation correction. Actual lane/chat setup is
tracked in `operations/team-registry.json` and `operations/STATUS.md`.

On November5 the owner explicitly requested nested teams for the six Create
categories, made the coordinator lead for operations/integration, requested
per-team token/time accounting, and asked for a separate coordinated chat for
each team. This overrides the former default against a standing team for this
project. Scope remains Create; other mods are addressed individually later.

## Roles and lanes

The primary chat is project lead and owns operations, scheduling, dependency
contracts, cross-team integration, shared build/test resources, acceptance and
private synchronization. Each additional chat is one team lead, with a separate
private Git worktree/branch and bounded packets. Team leads split useful work
among nested specialists, reconcile their results and return one coherent
candidate integration at a time. A lead does not mark the whole category complete
after one packet. All teams begin by checking current upstream implementations.

| Team | Independent outcome | First assignment and dependency |
| --- | --- | --- |
| Machinery and production | Complete processing machines and recipes | Audit remaining machines/recipe gaps; propose the smallest useful selected processing integration on accepted services. Own machine-specific core/adapters; coordinate Basin interfaces with fluids. |
| Transport and logistics | Complete item transport, filtering and factory logistics | Repair and requalify the isolated held43cef7dd belt candidate's stale peer Depot presentation after hopper extraction. Start the next logistics packet after acceptance. Preserve exact item ownership; coordinate transported processing callbacks with machinery. |
| Fluids and power | Complete fluid handling and rotation/control sources | Audit existing host fluid views and upstream code; define first water-only tank/transfer or rotation-source packet and shared contracts. Broad networks/schema/host changes require the existing owner gate. |
| Contraptions | General assemblies, actuators, actors and moving storage | Assess latest upstream portable core versus accepted finite piston; return a concrete native ownership/transform/contact plan and first bounded adapter. No broad framework implementation before owner approval where required. |
| Trains and schematics | Train systems and complete schematic workflow | Split train dependencies from reusable schematic implementation. Deliver a bounded schematic/track packet where existing services permit; train assembly depends on contraption contracts. Java/world import remains outside authorization. |
| Experience and parity | Progression, presentation and verifiable Create parity coverage | Build a source-backed feature matrix and choose a small progression/UI/effect integration. Coordinate recipe/material ownership with machinery and fluids; do not repeat every team's tests. |

## Isolation and permissions

Create six named local chats in the existing Create Rusty project. Use separate
Git worktrees under `.local/teams/<team>` and `codex/team-<team>` branches; the
project folder itself is a navigation container, not a Git repository. Each chat
must explicitly use its assigned worktree. Workers edit only their named files
in isolated worker worktrees/branches, or receive read-only research/review tasks.
No simultaneous writers share a checkout or index. Keep author and reviewer apart.

Team leads may commit and integrate their workers locally in their own lane.
The project lead owns integration into the primary candidate and all remote
pushes. Local candidate integration is authorized by the owner's new request;
GitHub merges, public releases and maintainer contact remain unauthorized.

Only the project lead changes effective orchestration/AGENTS, active host pins,
shared acceptance, root registry/accounting and cross-team contracts. Teams may
propose shared runtime/manifest/export/catalog/version wiring in their isolated
candidate, identifying it in the handoff; project lead resolves shared changes
serially. Do not silently rewrite another team's domain API.

Existing public/private boundaries, reference/art provenance, real permission
limits, protected playable saves/Prism settings and preservation rules remain.
No role grants additional sandbox, network, billing or machine access. Initial
team assignments are native read-only discovery/design until a bounded writable
packet and isolation are assigned. Team leads can perform the existing private
port work within authorized S6 outcomes after project-lead packet allocation;
new architecture/schema/framework/acceptance boundaries still go to the owner.

## Scheduling and validation

Each chat respects its actual agent concurrency cap; this root currently has
four active slots including itself. Separate chats have independent contexts;
do not assume unlimited total workers. Start with at most two specialists per
team lead, selected for separable work, and reduce active lanes if resource or
usage pressure warrants. Keep the user-selected model/effort; no automatic Ultra
or global configuration/limit changes. Review remains fresh read-only Sol High.

There is one host-wide heavy build/runtime reservation, controlled here. Team
leads request a reservation before Cargo/staging/linking, Java reference runs or
game/server processes; they cannot claim or steal it. Read-only work and isolated
edits can progress concurrently. Formatter use stays within owned files. Use
isolated output/instance paths; no shared mutable Cargo target or source staging
without the reservation. Do not start duplicate test batteries in each team.

Use focused tests for changed behavior; reuse unchanged evidence with attribution.
Project lead composes affected integration checks and one appropriate real-game
milestone. Substantive candidates get one fresh independent read-only Sol High
review; repeat only the repair delta or changed composition. A scaffold, source
audit or compiler pass is not gameplay parity. No new general test framework.

## Handoff and coordination

Each packet reports exact base/head, changed files, contract dependencies,
implementation summary, executed checks/receipts, remaining limits, review state
and resource/time records. Each team owns its handoff/status files in its lane.
The project lead polls compact thread snapshots and sends scoped follow-ups as
authorized by the owner's request to create and coordinate these chats. Team
leads can report through their normal final/status outputs and handoff files;
do not assume an orchestrator message independently authorizes messaging another
chat. Do not send external Slack/email/maintainer messages.

## Token and time accounting

Start team accounting at chat enrollment; record actual thread/agent IDs and
descendant relationships. Project lead aggregates reported per-session token
counters when available. Capture input, cached-input, output, reasoning-output
and total separately; cached/reasoning are subsets, not additional tokens.
Deduplicate agent/session IDs and cumulative counters; team totals include the
lead plus its descendants once. Missing/delayed telemetry is null/unknown, never
zero or a model estimate. These are reported usage counters, not a final bill.
Do not substitute account-wide usage-limit percentages for per-team token counts.

Record UTC phase intervals: implementation, build, test, review, coordination,
waiting. Implementation includes design/code/assets; build is staging/compiler/
linking/package work; test is validation commands and game/reference checks.
Report build/implementation and test separately, with review/coordination/waiting
visible. Tool receipts provide measured command durations; do not label an open
idle game or all agent lifetime as test execution. Agent-seconds can overlap;
project elapsed time is not the sum of concurrent team durations. Unclassified
historical time stays unclassified. Per-team token totals and project time split
do not require speculative per-phase token allocation.

Keep a small local registry, CSV/JSON counters and summary in the private project;
raw Codex transcripts and game runs remain outside Git. Read only necessary usage
metadata from accessible local logs; no credentials or unrelated conversation
content. If telemetry is unavailable, record that limitation and keep measured
time tracking. No new external billing/API integration is proposed.

## Activation

Finish/checkpoint the current belt battery, preserving its actual acceptance
state. Its final dedicated image retains gold after authoritative hopper removal,
so the belt candidate is HOLD, not accepted. Use accepted water02d029f3 as the
stable baseline for five teams; Transport receives the isolated held43cef7dd
candidate and this observed replica defect as its first integration. Obtain
independent operating-rule review, update effective project/workspace navigation
instructions, and make the appropriate exact baseline available to each lane. Create each
requested chat with its charter, explicit path, first task and accounting rules.
Verify each chat starts and returns a bounded initial status; then schedule
implementation by dependency and integration/resource ownership. Preserve the
prior orchestration source copies and record this project-specific adoption.
