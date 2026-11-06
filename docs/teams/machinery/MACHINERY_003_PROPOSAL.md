# Machinery 003 — one stationary Mechanical Drill

**PROPOSED for project-lead reconciliation; no new runtime source or game run.**
Qualify the already packaged Drill as one fixed, east-facing stone breaker in
an unprotected disposable world. One actor-placed dry stone directly in front
becomes one cobblestone world item through the no-player break path; a Survival
actor ordinarily picks it up. This is a finite machinery outcome, not a
general mining, automation or moving-contraption claim. Fan and Saw results
remain accepted on their own frozen inputs and are not replayed here.

## Exact audit inputs and reuse

The Machinery lane was clean at `c33aad5815b7f053c31f32a686b571cb6ee8cfe9`
before this proposal; its merge base with the primary's accepted native source
is `67e7bba1bc646e03606bce1493f04756b9674e6b`. The audit targets the
immutable accepted native source commit
`55339ce3347b2ce667281e8a3e6318ec856144e4`, package `.11`/cache14;
the physical integration checkout has since advanced. Any allocated runtime
must freeze and report its actual source, resource and binary inputs.
Primary docs checkpoint `8470fc515d460a95a0cacabc644160f9e7aba4b5`
records both-mode Saw acceptance and the shared queue. The pinned Java Create
6.0.11 reference is clean at `7ff2760e4924d35d4bc2e22c4b19923f0178cd16`.

Current [Delete228 main](https://github.com/Delete228/create-horizon/tree/dd0f3141a33851ac20fa98b807a6532471dba727)
is `dd0f3141`; its [complete 992-path tree](https://api.github.com/repos/Delete228/create-horizon/git/trees/dd0f3141a33851ac20fa98b807a6532471dba727?recursive=1)
contains the same Git blobs as accepted native source for `mixins/create/drill.rs`
(`9d72d14dff84cc95dc3c141767a269fab1fa953b`),
`engine/src/core/drilling.rs` (`a1440189c07843035f0a40be8c357c9e0cc70d01`),
the Drill block declaration (`096fbba0538232db4be474fe0c5d5d13230d6bd1`)
and loot table (`515a8d1a2574659ec0d6e913e6f36d2a5e4afa17`). No newer upstream Drill
implementation needs importing for this selected outcome.

| Existing source at accepted `55339ce` | Selected role |
| --- | --- |
| `native/create/mixins/create/drill.rs` | Adjacent target, server tick, saved `Progress`/`NextTick`/`Breaking`, crack updates, no-player destruction and head draw |
| `native/create/mixins/create/engine/src/core/drilling.rs` | `abs(RPM)/100` break speed, hardness clock, pause, refusal; four pure tests |
| `native/create/mixins/create/runtime.rs`, `placement_runtime.rs`, `horizon-mixin.toml` | Ownership/tick/draw, placement, rear-only kinetic input at 4 SU/RPM, module registration |
| `native/create/content/` and `tools/catalog.py` | Six-facing block/model/item, crafting and self-drop declarations |
| Exact pinned Horizon `9c7f9b22` | `destroy_block_dropping` uses `Level.destroyBlock` without an acting player; `block_drops` and no-player loot produce world item entities |

The pinned host paths are
[commands.rs:2571–2578](https://github.com/Cha0sCollective/Horizon/blob/9c7f9b2215dfed497aba5522c44f38926628648d/crates/horizon-client/src/game/commands.rs#L2571-L2578),
[level/update.rs:418–452](https://github.com/Cha0sCollective/Horizon/blob/9c7f9b2215dfed497aba5522c44f38926628648d/crates/horizon-world/src/level/update.rs#L418-L452), and
[spill.rs:401–430](https://github.com/Cha0sCollective/Horizon/blob/9c7f9b2215dfed497aba5522c44f38926628648d/crates/horizon-client/src/game/spill.rs#L401-L430).
The separately checked local `horizon/` HEAD is `3ac2d4c`; it is not the
accepted `9c7f9b22` pin. The packaged `drilling_checks.py` and
`drilling_oracle.py` are development probes, not current accepted gameplay
receipts. The gap is qualification of ordinary Survival placement, drops,
collection, peer view and cold progress, not a missing Drill adapter.

## Pinned Java contract

`DrillBlockEntity` targets the one block along `FACING`; `DrillBlock` accepts
one shaft behind it and has 4.0 SU/RPM impact. The stationary
`BlockBreakingKineticBlockEntity` runs at nonzero RPM, advances progress by
hardness, and saves `Progress`, `NextTick` and `Breaking`. Air, liquid,
hardness `-1` and `create:non_breakable` refuse. Replacing one breakable target
with another preserves accumulated progress; an invalid target clears it.
Normal destruction calls `BlockHelper.destroyBlock` with null player and empty
tool, so the break does not use the actor's mining permission or held tool.
The pinned 1.21.1 stone loot table has a no-Silk-Touch cobblestone branch
without a tool predicate, yielding one cobblestone when drops are enabled.
Source references in the clean pinned reference:
`DrillBlockEntity.java:35–53`, `DrillBlock.java:81–86` (neighbor wake) and
`:93–95` (rear shaft), `BlockBreakingKineticBlockEntity.java:60–75` (save)
and `:85–145` (break), `BlockHelper.java:208–245`, and
`AllBlocks.java:1350` (stress). Inspected stone resource JAR SHA-256:
`40fd171da75b9e4ab0011c8dacc047ab09704f2d4c5377b69271772bb11ba536`.

## Proposed finite milestone

Use a fresh Survival actor in each of integrated and dedicated/two-client
modes. Supply the Drill, one stone, motor and shaft as labelled test inputs;
do not claim a crafted acquisition chain. Place the Drill and stone by ordinary
actor interaction on a clear, dry plot. Drive the east-facing Drill from its
west/rear shaft with a labelled existing motor fixture at verified 32 RPM;
record actual motor, network and Drill RPM plus `facing=east` and 4 SU/RPM
stress. Motor GUI/power controls reuse their accepted evidence.

With the stone directly east, record server-authoritative target block,
`Progress`/`NextTick`, one cobblestone world item after the break, and zero
stone/cobblestone duplication in the selected plot. The Survival actor then
ordinarily collects that drop: one cobblestone in inventory, stone input zero,
target air and no selected cobblestone entity remaining. The pure cycle's
stone/32-RPM fixture records nine Hit actions and Break at zero-based tick 45,
counting its first hit at tick 0; live tick
alignment and item pickup must be observed rather than assumed from that unit
test. Capture actual rendered head/crack/target/drop views, including a
dedicated peer view distinct from server NBT.

For the same machine, stop rotation after partial progress, verify the
remaining delay and progress stay unchanged, close normally, cold-reopen the
same world and clients, and verify the saved target/progress before restoring
the motor fixture. Resume to exactly one dropped and collected cobblestone.
Run that cold path in both modes; record all process exits. One integrated
negative check places bedrock at the target and observes no break/drop with
progress cleared. Liquid/air/tag refusal are source boundaries, not extra
runtime batteries in this packet.

## Ownership, validation and stop conditions

The proposed first implementation step is **no production source change**:
reuse accepted Drill and host services under the project lead's future shared
runtime lease. A packet-local control sequence may be prepared after allocation;
it must preserve failures and freeze the actual candidate binaries/resources.
Reuse exact-input core/preflight evidence where valid; do not rebuild or replay
Fan/Saw or the old broad Drill smoke battery merely for this proposal. If the
focused run reveals a concrete difference, confine a repair proposal to
machine-owned `drill.rs` or `engine/src/core/drilling.rs` before any code edit.
Shared runtime wiring, catalog, pins, host API and acceptance changes return
to the project lead. No new architecture is currently required, so the
host-gap fallback gate is not triggered.

The no-player break path has no actor permission check. Qualify only an
unprotected disposable world, not claims/protected regions or general
permission parity. Defer entity-contact damage, moving/contraption drills,
waterlogging, cobble-generator optimization, belt/hopper/chute collection,
all other target blocks, player-tool modifiers, multi-block mining, crash
atomicity, public release and full-S6 coverage. Stop if selected loot, saved
ownership or ordinary placement needs a new host/entity-owner architecture;
report the exact gap before changing it.

Read-only specialists: `/root/drill_source_reuse` (01:21:12–01:25:19 UTC,
247 s) and `/root/drill_java_contract` (01:21:19–01:24:39 UTC, 200 s).
Their observations were reconciled here; neither edited files or ran a build,
Java process or game. Team phases are recorded locally by
`tools/operations/phase.ps1`; verified token counters remain unknown.
