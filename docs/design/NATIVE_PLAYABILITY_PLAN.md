# Adopted native playability plan

Owner adopted this route on November 4, 2026 after reviewing
`docs/reference/S1_EXISTING_CREATE_PORT_EVALUATION_2026_11_04.md`. This supersedes
the active route in S1_COMPLETION_PLAN, not its historical evidence or original
G1 acceptance record. Old G1 remains unpassed historical work; native S1 starts
OPEN against the bounded outcomes below. No release authority.

## Reuse boundary

Subsequent owner decision: after accepted dry mixing, a separate bounded candidate
host refresh is authorized by `NATIVE_S6_HOST_REFRESH_PACKET.md`. Its pins apply
only to the candidate; the original adoption and qualified baseline below remain
historical evidence, not a new migration pass.

Use the unchanged tested Create port `16c5d454b1b9860da01b78474ea5206f0664d384`
(MIT), official Horizon `fc02103818c212920fd24aa34a2b2f91ba49e56f`, Mixins
`a1e13326306c6bc074b11e6a4926deb551c4083e`, native API 359 and Rust 1.99.1.
Import the port into private `native/create/` as a separate Cargo workspace.
Keep original notices and exact provenance. The existing package's extra content
stays intact but is not added to supported scope. Do not copy it into public Horizon.

Our reference remains Create 6.0.11 `7ff2760e4924d35d4bc2e22c4b19923f0178cd16`,
NeoForge 21.1.219 and Minecraft 1.21.1. The candidate source/art bootstrap is
6.0.10 `ac0c444d9828da3453ae8cc65338e8de063286fb`; local art JAR SHA-256
`ef87fe5709f1ba1f5b8bb20a2925b5afb4669e178fd6d8bf10c167759eefe37a`.
Keep this difference explicit. The bounded source comparison found unchanged
selected kinetic/tick/recipe logic except millstone clearContent output clearing;
do not generalize that comparison to full Create compatibility.

## Native S1 acceptance

Supported world: fixed mob-free Creative TestTerrain Overworld. Supported modes:
integrated singleplayer and one dedicated server with two real matching rendered
clients. Open to LAN remains deferred. No world import or broader profile.

1. Native package loads on the exact compatible host. Ordinary player placement
   works for the motor, shaft, both cog sizes and millstone. Server processing is
   authoritative; shared world observation and player collection work.
2. A bounded chain has expected ratios, stop/reverse/resume behavior and converts
   plain cobblestone into exactly the selected gravel results. Diagnostic layout
   and item creation are identified separately from actual player actions.
3. Normal removal/collection and save/reopen preserve supported item ownership,
   motor setting and paused progress. A real cold process restart is used;
   dedicated also restarts both clients. Peer disconnect/rejoin is covered.
4. Both clients render the actual native machine and its motion. State queried
   from the server is labeled accordingly; it does not prove independent client
   inventory NBT resync or tick-anchored phase agreement.
5. Match the accepted Java millstone controls: one 64-count output slot permits
   processing into free slots; nine full 64-count outputs consume the input and
   discard the uninsertable output. Keep output stack insertion capped at 64.
   A handler limit of 99 controls the processing guard; do not replace insertion
   limits with 99 or describe intentional reference loss as a transport/save loss.

Use the evaluation's valid unchanged results with attribution. Its integrated
overall FALSE remains FALSE despite passed gameplay/reopen components. Its final
dedicated trial passed; that unchanged-parent result is not a new patched run.
Run focused changed behavior and the affected native milestone after integration.
Mark native S1 complete only after the checklist is supported and independently
reviewed; record limits and stage accounting without marking old G1 passed.

## Explicit replacements and deferrals

The owner accepted native compiled code instead of the old Wasm host. Removed
in-frame execution is no longer an acceptance topology. Wasm sandbox/fuel/trap
rollback, custom generic counter/slot framework and its admission/generation/
replay protocol are not prerequisites for native playability.

The custom cross-owner interrupted-crash checkpoint framework and extra hardened
stale/replay guarantees are deferred. Native code is trusted; ordinary saves are
not an atomic publication across all world/player files. Do not claim crash-safe
ownership, malicious-native-code isolation or unqualified reconnect/resync.
Chunk untrack/retrack, concurrent collection, fault injection, large graphs,
overspeed breakage parity and full hardware/performance certification are not
part of this bounded S1 claim. Repair a concrete ordinary-play bug if encountered;
ask before turning it into a general framework or expanding supported behavior.

Later S2/S3 reuse the existing kinetic/milling implementations and relevant
reference fixtures, rather than rebuilding the same algorithms. This adoption
does not mark S2/S3 or the full modpack complete. Survival, contraptions, trains,
fluids, broad additional machinery and Java/world compatibility remain future work.

## First bounded packet

1. Record the adopted rules and plan following one fresh read-only Sol High review.
2. Import exact tracked upstream source, preserving MIT notices and upstream Git
   attribution in a provenance file; no generated inputs or assets. Preserve the
   root Rust workspace as legacy. No code is imported into the public engine.
3. Correct only the selected milling guard, its native adapter and affected
   existing oracle/test expectations. Separate handler limit 99 from insertion
   limit 64. Reuse existing inventory insertion and persistence paths.
4. Add only the developer path overrides needed to reuse the exact official
   checkout, installed Rust and warm target; do not install a new launcher or
   change user instances. Keep bootstrap defaults available with explicit pins.
5. Run focused milling regressions and the small integrated/dedicated milestone
   covering ordinary progress, the two full-output controls and normal cold
   reopen. Review the stable delta/results, checkpoint privately and report.

Before adding a prerequisite, identify the actual missing behavior, failing caller
and smallest repair. Check upstream for existing work. No new host API, general
mod framework, native ABI/profile expansion or public publishing is included.
Stop for a genuine blocker, required input, or scope expansion.

## Preserved legacy work

Private adoption branch begins at `a19e237b3c8bf59bbbf599dbf31f005732202f33`.
Accepted engine checkpoint `9f7d5ecb7abee68b0f3af6363fbdb3e6b0547219` and all
accepted/public branches remain. The existing worktree stays on
`codex/s1-configured-operation` with its five frozen dirty files and import HOLD;
source map `.local/s1op1-source-sha256.json` is unchanged. No formatting, compilation,
cleanup, reset or migration of that candidate is part of adoption. Old raw runs,
reference snapshots and the prior planning baseline remain preserved.
