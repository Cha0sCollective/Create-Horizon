# FLUIDS-005 — bounded pump and pipe source handoff

**Status:** reviewed source candidate, not yet composed, compiled or qualified
in game. Operations allocated the private four-block water line after the
design review. The exact candidate is `codex/fluids-005-candidate` at
`3f4e6cd16503c1e98ab22aaca82315165c6e3191`, in
`.local/workers/fluids-005-candidate`, based on accepted native source
`55339ce3347b2ce667281e8a3e6318ec856144e4`. The team lane remains
separate at `60d8478` before this handoff; its reviewed FLUIDS-004 source
and pending runtime packet are unchanged. The host remains pinned at
`9c7f9b2215dfed497aba5522c44f38926628648d`.

## Source delivered

Two isolated worker branches, each based on `55339ce`, were composed locally:

- `codex/worker-fluids-005-kinetics-content` at `2f909058` supplies
  `Kind::Pump` in `native/create/mixins/create/engine/src/core/kinetics.rs`:
  small-cog side mesh, 4 SU/RPM stress and no axial shaft ports. It also
  supplies a standalone pump/pipe content generator and 12 authored JSON
  blockstate, model, item, block and loot files. Pump uses six-value `facing`;
  straight Pipe uses `axis=x|y|z`. Its provisional models reference vanilla
  texture IDs; no original Create art is copied into Git.
- `codex/worker-fluids-005-transfer` at `5febb599` supplies the pure
  `pump_transfer` rate/custody decisions and six focused tests, plus
  `fluid_pump_transfer.rs`, the endpoint-specific native adapter. It reads
  only the loaded Tank, facing Pump, matching-axis Pipe and downward Basin;
  rejects branches, other fluid owners, foreign water components, nonempty
  Basin items/output/second input, and a Mixer above the Basin. It uses the
  accepted Tank/Basin views and simulate/execute callbacks, then requires
  exact receipt, real-volume delta and total-water agreement. Mismatches
  restore both complete endpoint NBT/items in the same server turn and verify
  identity/census. Restoration failure panics as a hard custody fault. There
  is no extra persistent water owner or pipe pressure/flow schema.

The candidate also cherry-picks Operations' `ec4c560` nonkinetic Funnel
placement fix, then adds only `create:fluid_pipe` and
`create:mechanical_pump` to the `placement_runtime.rs::level_ports` exclusion.
Both new blocks would otherwise be inferred as axial shaft neighbors after
runtime ownership is wired. Keep all three exclusions in final composition.
The Pump still receives kinetic power from its side cog through the proper
core graph.

Worker handoffs: `FLUIDS_005_KINETICS_CONTENT_HANDOFF.md` and
`FLUIDS_005_TRANSFER_HANDOFF.md` in the candidate's `docs/teams/fluids-power/`.
Those files name each source contract and its limits. The independent
read-only Sol High review is **CLEAR for this uncompiled source packet** at
exact `3f4e6cd`, with no critical or major source finding. Its interval was
02:21:34–02:25:56 UTC (262 seconds). CLEAR is not a runtime, rollback or
save/reopen pass.

## Serial shared composition owned by Operations

1. Add the transfer adapter module to `horizon-mixin.toml`, and wire
   `runtime.rs` ownership, placement and ticking. `create:mechanical_pump`
   needs an ordinary kinetic block entity, a `Kind::Pump` node whose axis
   comes from `facing`, and a call to the adapter **after** signed network
   RPM is calculated. The Pump entity is detached during its tick; the
   adapter reads its block state and the two endpoint entities still in the
   world map. `create:fluid_pipe` needs block ownership/placement but no
   block entity, kinetic node or tick. Targeted Pump wrench behavior flips
   `facing` to its opposite. Preserve the Funnel, Pipe and Pump placement
   shaft-port exclusions above.
2. Register the selected Pump/Pipe IDs and model parts in the private
   catalog, invoke `fluid_pump_content.generate` from `tools/content.py`
   after generic block generation, add language/creative entries and
   regenerate the bundle/catalog index. **Do not import the original
   6.0.10 Pipe blockstate unchanged.** The original is multipart over six
   connection booleans; this candidate declares only `axis`. Adding Pipe
   to catalog `BLOCKS` causes `assets/src/lib.rs:537-550` to import the
   original blockstate/item model. Retain the authored bundled axis-only
   state/model by a selected importer exclusion, or add a bounded
   axis-compatible mapping as Track already does. Review exact pack
   priority and selected asset outputs before a shared stage. Operations
   owns the catalog, importer, generated index, cache key and pins.
3. Review the composed hard-fault policy. The adapter tries both endpoint
   restorations and readback before `panic!`; a panic may stop the process.
   No interrupted-tick crash atomicity or replay guarantee is claimed.
   Basin fill can leave an in-memory `contents_changed` notification after
   restoration. Accepted Basin code later calls `mixer::schedule`, which
   returns without a Mixer at Basin+2Y; the adapter enforces that absence
   and empty items for this profile. Verify the actual callback failure
   paths before runtime qualification.

No public host change, generic fluid network, reusable transaction framework,
extra saved flow field, new canonical volume owner, host pin or public release
is in this packet. Stop and return to Operations if composition needs one.

## Focused validation after the shared heavy lease

Run only the changed engine filters for Pump kinetics and `pump_transfer`,
then the selected native compile/lint and a focused endpoint failure check.
The pure tests check the rate, side mesh/no axial port, first Basin input
capacity refusal with water remaining, matching simulations, receipt/real
delta mismatch, and a destination refusal after source drain. They do **not**
exercise host callback mutation, full NBT restoration or actual scheduling.
Inject or otherwise reproduce a failed destination commit after source drain,
verify both endpoint fields and the 1,000 mB census are restored in the same
server turn, and show Basin notification cannot start a recipe in the
declared no-Mixer profile. Keep any process panic evidence separate.

For the finite integrated and dedicated/two-client runtime recipe, place Tank
`(0,0,0)`, Pump `(1,0,0)` facing east, Pipe `(2,0,0)` axis x, and empty Basin
`(3,0,0)` facing down. A side Cog `(1,0,1)` axis x and Creative Motor
`(2,0,1)` facing west are a proposed 16 RPM supply; confirm actual placement
and signed RPM. Transfer should be 8 mB per active tick, with at least 125
successful ticks plus startup to move 1,000 mB. Check 0-RPM and branch
refusals. Stop at a measured partial volume, save/exit normally, reopen and
verify `TankContent + Basin InputTanks = 1,000 mB`, then resume to Tank 0 /
Basin 1,000. Refill the Tank to 1,000 with the first Basin input still full;
powered refusal must leave a **separate 2,000 mB census** unchanged. Compare
server queries/saved NBT and actual peer images as separate observations.
Pipe flow visuals and exact flow-phase parity remain unqualified.

## Checks and accounting

Worker A checked JSON parsing and whitespace; worker B checked whitespace.
The candidate's Git tree is clean. No Cargo, stage, Java, game, client or
server run occurred, and no heavy lease was held. The focused tests are
source only, not executed. Worker implementation intervals were 317.455 +
11.880 seconds and 731.830 seconds respectively. Lead phase timing is in
the local fluids-power phase log. Per-agent token telemetry was unavailable
in this team context; it remains unknown.
