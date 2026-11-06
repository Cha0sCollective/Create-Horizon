# FLUIDS-005 endpoint-transfer worker handoff

**Scope:** private bounded source candidate only. Branch
`codex/worker-fluids-005-transfer`, base accepted
`55339ce3347b2ce667281e8a3e6318ec856144e4`. This is not an integrated,
compiled, tested-in-game, or gameplay-qualified pump. The lead owns candidate
composition and Operations owns the shared heavy build/runtime queue.

## Owned source

- `native/create/mixins/create/engine/src/core/pump_transfer.rs` implements
  the Java-derived steady `max(1, floor(abs(RPM)/2))` mB/tick rate and the
  narrow two-owner simulation/receipt/census decisions. It rejects nonfinite,
  zero or out-of-range RPM and guards malformed arithmetic. Six focused unit
  tests cover signed rate, first-input capacity refusal with water left,
  simulated endpoint disagreement, a real-delta/receipt mismatch, and a
  rejected destination after source drain, and malformed arithmetic refusal.
- `native/create/mixins/create/engine/src/lib.rs` exposes only that core module.
- `native/create/mixins/create/fluid_pump_transfer.rs` is the native endpoint
  adapter. It accepts only a loaded Tank, facing Pump, same-axis straight Pipe,
  and downward Basin in one line. It rejects adjacent extra pump/pipe/Tank/Basin
  blocks or exposed native fluid owners, a nonempty Basin item/output profile,
  second input fluid, foreign components, connected/foreign Tank state and a
  Mixer two blocks above Basin. No pump or pipe BE owns water.

The adapter reads exact endpoint NBT and the existing native fluid views,
simulates Basin fill and exact Tank drain, then checks both endpoint snapshots
were untouched. It executes exact Tank drain before Basin fill, re-reads strict
canonical amounts, requires both receipts and deltas to match, and verifies
the total. Success permits only TankContent and whole Basin InputTanks to
change. Basin `InputTanks` includes nested `Level.Target`/`Level.Speed`, so the
rollback snapshot retains the full NBT field, not just water amount. On any
receipt/state mismatch, it attempts to restore **both** full endpoint NBT and
items in the same server turn, re-syncs them and reads back exact prior states
and entity identity. If either restoration or readback fails it panics as a
hard custody fault. That exceptional panic may stop the process; it is not a
cross-owner crash/replay guarantee and needs independent implementation review
before runtime.

Accepted `basin.rs:93-109,684-764` shows fill's `contents_changed` cache flag
is consumed by ordinary Basin tick and calls `mixer::schedule` at Basin+2Y.
Accepted `mixer.rs:85-94` immediately returns unless that block is an actual
Mixer. The adapter rejects a Mixer at that location and requires empty Basin
items, so the queued notification cannot start a recipe in the declared
profile. Ordinary Basin animation can advance `Level.Value` on later ticks;
each transfer snapshots the complete current InputTanks at its own tick start.

## Required shared composition (not edited here)

1. Add a `[[module]]` entry in `native/create/mixins/create/horizon-mixin.toml`
   for `create_port_fluid_pump_transfer` from `fluid_pump_transfer.rs`, in
   `crates/horizon-client/src/game.rs` after the accepted fluid codecs.
2. In `runtime.rs`, own/tick `create:mechanical_pump`, create its ordinary
   kinetic BE, map its facing to worker A's dedicated small-cog Pump kind, and
   call `create_port_fluid_pump_transfer::tick(game, pos, rpm)` **after**
   `network.speed(pos)` is calculated. The ticking Pump BE is detached from
   `world.block_entities`; the adapter deliberately reads its block state and
   only the Tank/Basin BEs still in the map. Pipe needs no fluid BE or saved
   flow metadata. Check the hard-fault policy and returned `TickResult` when
   wiring this call.
3. Compose Pump `facing` and straight Pipe `axis` blockstate definitions,
   placement/wrench behavior and shared content/catalog/cache wiring serially.
   The adapter expects lowercase `facing` values (`east`, etc.) and Pipe
   `axis=x|y|z`. It does not add a general network, host API or save schema.

## Focused validation once Operations grants a heavy slot

Run the engine's `pump_transfer` unit filter, then the selected native
build/lint and a four-block integrated + dedicated/two-client profile on exact
accepted pins after shared composition. Suggested X fixture: Tank `(0,0,0)`,
Pump `(1,0,0)` facing east, Pipe `(2,0,0)` axis x, Basin `(3,0,0)` facing
down; power Pump through a side cog. The lead's proposed Cog `(1,0,1)` axis x
and Creative Motor `(2,0,1)` facing west must be checked against actual
placement/kinetic states before claiming runtime. At 16 signed RPM expect
8 mB per active tick and at least 125 successful ticks for 1,000 mB plus
startup. Check 0-RPM refusal, branch/other-endpoint refusal, empty Basin
transfer, a partial stopped normal save/exit/reopen census of 1,000 mB and
resume to Tank 0/Basin 1,000. Refill Tank to 1,000 after Basin's first input
is full and verify a separate 2,000 mB census remains unchanged under power.
Observe actual peer block images separately from server NBT/query receipts;
no pipe-fluid visual or exact flow-phase parity is claimed. A focused injected
endpoint-failure check or equivalent reviewed failure-path proof is still
needed before qualification; the pure tests do not exercise host callback
mutation/rollback.

## Checks and limits

`git diff --check` passed. Pinned `rustfmt` was not available on PATH in this
worker environment. No Cargo, staging, Java, game, peer, or host process was
run; no heavy lease was held. Source compilation, callback rollback behavior,
selected content identity, ordinary cold persistence and peer observations
remain unverified. No public engine, Tank/Basin adapter, manifest, catalog,
runtime, host pin or other team's file was edited.

Implementation phase: 2026-11-06 02:07:49.081619–02:20:00.911143 UTC,
**731.83 seconds**, from the local phase event. Per-agent token telemetry is
unknown in this worker context; do not infer a count.
