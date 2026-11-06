# FLUIDS-005 — powered water transfer reuse assessment

**Status:** discovery and design only, November 5, 2026. Base team lane
`codex/team-fluids-power` at clean `f297f989d62b7b9d323721248906dcfbf8979f9d`.
Pinned Java is Create 6.0.11 `7ff2760e`; accepted private native source is
`55339ce3347b2ce667281e8a3e6318ec856144e4` (`.11`, cache14), and pinned
Horizon is `9c7f9b2215dfed497aba5522c44f38926628648d`. The physical
integration checkout is an advancing candidate; native claims below refer to
accepted Git revision `55339ce` (`git show 55339ce:<path>`). FLUIDS-004 remains
separately reviewed, unrun and unchanged. This document makes no new gameplay
or Survival Tank claim.

## Reuse boundary

| Source | Available now | Gap for this path |
| --- | --- | --- |
| [Current public Delete228 port](https://github.com/Delete228/create-horizon) at `dd0f3141` and its clean local snapshot under `.local/evaluations/create-horizon-08-review/source` | Basin fluid hooks and the generic native placement/wrench/kinetic patterns. Its [roadmap](https://github.com/Delete228/create-horizon/blob/dd0f3141/docs/roadmap.md) still leaves pumps and pipe networks open. | No powered pump, fluid pipe adapter, or staged pump/pipe content to import. Do not relabel the accepted Tank adapter as upstream. |
| Accepted native Tank (`fluid_tank.rs:20-187`, `fluid_tank_codec.rs:17-115`) | One isolated regular controller, exact plain-water `TankContent`, 8,000 mB capacity, all-side native view and simulate/execute fill/drain. | A regular Tank Survival bucket click intentionally passes through. It is not the transfer mechanism. |
| Accepted native Basin (`basin.rs:490-654`, core `basin.rs:753-792`) | Plain-water custom fluid view and simulate/execute fill into input tanks; output insertion refused. Two 1,000 mB input segments exist, but incremental same-water fills stop when their first matching segment reaches 1,000 mB. | A pump needs to use the actual simulated acceptance, not assume all 2,000 mB are available. Keep the first profile's Basin empty of items and output fluid. |
| Pinned Horizon API54/55 (`game/machines/native_fluids.rs:308-483,701-747`) | Custom Tank/Basin view, fill and drain with validated amounts; ordinary storage has a rollback-aware move helper. | `native_move_fluids` explicitly rejects `FluidAt::Custom` at either endpoint (`:718-724`), so it cannot move this Tank into this Basin. Individual custom calls do not supply an atomic cross-owner commit or rollback (`:361-366`). |
| Accepted kinetic core/runtime (`engine/src/core/kinetics.rs:24-124`; `runtime.rs:574-741`) | Source speed, network stress and a small-cog mesh model for existing machines. | No `mechanical_pump` node or content. Java Pump is a cogwheel with axis along its facing and stress impact 4 (`AllBlocks.java:929-938`), with no axial shaft ports (`KineticBlock.java:51-53`); the existing one-sided `Kind::Consumer` is not a matching side-cog power path. |

All native paths in this table are under `native/create/mixins/create/` at
`55339ce` except the staged content tree. Host paths are under the pinned
`crates/horizon-client/src/` at `9c7f9b22`. Pinned Java paths are under
`references/create/src/main/java/com/simibubi/create/`.

## Pinned Java custody and rate

The smallest source-backed physical line is **one ordinary filled Tank → one
Mechanical Pump → one straight Fluid Pipe → one empty Basin**, all loaded in
one dimension. The pump's back faces the Tank and its front faces through the
pipe toward the Basin. A cog meshes at the pump's side to provide speed; its
front/back fluid ports are not a replacement for that kinetic connection.
For an isolated X-axis fixture, put Tank at `(0,0,0)`, Pump facing east at
`(1,0,0)`, straight Pipe at `(2,0,0)` and Basin at `(3,0,0)`; place the
motor/cog supply off that fluid line.
Pump `FACING` determines pull and push; a negative speed does not reverse the
fluid direction (`PumpBlockEntity.java:119-124,307-355`). The Java Pump is
active at any nonzero speed. Its pressure magnitude is `abs(Speed)` and the
single path's steady rate is `max(1, floor(pressure/2))` mB per active network
tick (`FluidNetwork.java:65-81`). At 16 RPM this is 8 mB per active tick, so
1,000 mB takes at least 125 successful transfers plus startup/propagation.

Java `FluidNetwork` simulates a source drain and target fill before executing
them (`FluidNetwork.java:184-267`). The Tank and Basin are the **only canonical
bulk-water owners**. A pipe `Flow.fluid` is a simulated 1 mB identity sample,
not another stored volume (`FlowSource.java:33-55`, `PipeConnection.java:155-160`).
Per-face pressure, flow identity/direction and progress have Java NBT read/write
paths (`PipeConnection.java:212-280`, `FluidTransportBehaviour.java:145-173`),
but the `FluidNetwork` graph object is explicitly not serialized
(`PipeConnection.java:38-48`). These fields are presentation/propagation
metadata and must not be added to the water census.

## Proposed first implementation packet, pending project-lead decisions

Choose a **single four-block line**, one straight direction and one source and
target. Permit exactly one water-only Tank controller (initial 1,000 mB from
an ordinary Creative click), one facing Pump, one straight Pipe and an empty
Basin. Refuse branches, loops, open ends, unloaded blocks, other dimensions,
other fluids/components, connected Tanks and additional fluid targets. The
source's water and target's input tank are the only canonical volume fields.
Use existing native fluid view/fill/drain at both endpoints and existing
block-entity persistence/sync; no host pin or generic fluid network change is
proposed. A new pump kinetic kind must participate as a small cog with stress
impact 4 and no axial shaft ports, rather than borrowing the current one-sided
consumer node or the default two-ended shaft-port behavior.

For each powered server tick, verify the whole loaded line and nonzero finite
RPM. Derive rate from `abs(RPM)` and the pinned formula. Read strict Tank and
Basin controller snapshots and their exact plain-water amounts. Simulate
destination fill and source exact drain for the same bounded amount;
simulation must leave both NBT owners unchanged. Execute the exact source
drain, then destination fill, and re-read both canonical amounts. Accept
progress only when the Tank decrement and Basin increment both equal the
same returned amount and the total remains unchanged. If either result or
actual state differs, restore the complete touched endpoint NBT fields from
their pre-tick snapshots in that same server turn, resync, and verify the
original census. Basin `InputTanks` changes also update nested `Level.Target`
and `Level.Speed` (`fluid_codec.rs:162-206`); restoring only `TankContent` is
insufficient. Basin fill also raises its in-memory `contents_changed` flag,
which can schedule a later operator check; demonstrate that this notification
is harmless with empty items and no active Mixer. A failed restoration is a
custody fault and cannot be accepted as transfer. Do not trust an API return
value alone or silently discard water.
This is a bounded **proposed** mod-owned paired transaction, not an atomic
host operation or a custom crash/replay guarantee; the host warns that
individual custom callbacks may have side effects beyond validated returns.
The transaction design needs project-lead reconciliation before source edits;
its eventual implementation and failure paths need fresh read-only review
before gameplay qualification.

At the finite endpoint, one empty Basin accepts successive small water steps
until its first input segment reaches 1,000 mB; later same-water steps refuse
with the Tank unchanged. Do not generalize that to every Basin configuration.
Cold evidence should stop pump power at a measured partial amount, save/exit
normally, reopen, verify `TankContent + Basin InputTanks = 1,000 mB`, then
restore power and reach Tank 0 / Basin 1,000 mB. A stopped 0-RPM line and a
full-first-input line must make zero progress. For the full-input refusal
guard, refill that Tank to 1,000 mB after the Basin reaches 1,000 mB, then
power the line again and verify both owners stay at 1,000 mB (a separate
2,000 mB census). An empty-source result from the first fixture alone would
not prove target-capacity refusal. Keep Basin ingredients/Mixer inactive so
no recipe consumes the measured water. Peer images and block
queries are separate observations; 1,000 mB in a Tank may sit below its
front window. No active-tick crash atomicity or general pipe-network claim.

## Decisions before source edits

1. **Cross-owner transfer:** select the bounded mod-owned paired snapshot,
   simulate/execute and same-turn restoration path above, or separately authorize a host API55
   custom-endpoint transaction. The latter changes the public engine and is
   outside this team packet. The accepted host's move helper cannot simply be
   called for these endpoints.
2. **Pipe flow save/presentation:** the smallest functional packet can derive
   flow after restart and persist no extra bulk-fluid owner. Decide whether
   to add Java-style per-face `Pressure`/`Flow` metadata in the new private
   pipe block entity for cold flow presentation. That is new mod-level save
   schema and peer-sync work, even though it is not extra canonical water.
   Until selected and qualified, make no pipe-fluid visual or exact flow-phase
   cold claim.
3. **Kinetic/content boundary:** allocate the small-cog pump kind (impact 4,
   no axial shaft ports),
   targeted `FACING`/opposite-wrench handling, one straight Pipe state, and
   selected pump/pipe block, item, model, loot and asset-catalog entries.
   Use the pinned 6.0.10 art source with provenance; no whole upstream import.
   Source and content identity must be composed by the project lead before a
   shared build/runtime lease.

The first implementation should begin only after those choices are reconciled
against this design. No Cargo, staging, Java, game, runtime driver or source
edit is part of FLUIDS-005 discovery. FLUIDS-004 may run independently when
the project lead grants its heavy slot.

## Work accounting and limits

Lead `/root` audited pinned host and accepted native services. Read-only
specialists `/root/upstream_pump_reuse` and `/root/java_fluid_contract`
separately checked current port reuse and pinned Java transfer/custody.
Their read-only intervals were 344.078 seconds (through 01:53:40 UTC) and
01:47:58–01:52:14 UTC respectively. The shared heavy queue remained with
Operations; no accepted outcome changed.
