# P02a settled single-source kernel

Status: reviewed P02a implementation after G0. This is a
small pure Rust calculation, not playable Horizon machinery or a G2 pass.

`create-horizon-core` has no runtime dependencies and owns no world, item, file,
network or renderer state. The caller supplies a complete topology with stable
node IDs, explicit oriented transmissions, impact/capacity facts and at most one
source per component. The host adapter and its generation/lifecycle rules remain
separate. Node IDs here are not raw Minecraft registry IDs.

The kernel recomputes sorted connected components, propagates signed shaft/cog
ratios and distinguishes theoretical from effective speed. Stress uses the absolute
local speed of each load; capacity uses source magnitude. Equality runs; strict
overload sets every effective speed to zero. A removed edge or expired source
cannot keep cached power because the function retains no previous solution.

Motor settings default to +16 and signed settings require magnitudes 1..256. The
value-board conversion chooses sign and clamps its magnitude to that range.
Stopped operation is represented by missing/inactive source or topology, not a
zero player motor setting.

## Evidence

Ten integration-test groups pass, covering all 512 nonzero signed motor settings,
board boundaries, four ratios in both directions/signs, local-speed stress,
equality/overload/recovery/expiry, split/rejoin/reversal, input-order determinism,
consistent/conflicting cycles, invalid topology/facts, arithmetic overflow and
underflow in ratio propagation, capacity products and load products.
Rustfmt and Clippy with warnings denied pass. The library also checks for
`wasm32-unknown-unknown`; this is compilation only, not execution in a Wasm host.
The subsequent P02b packet now records actual selected sandbox execution separately
in [P02_WASM_RESULTS.md](P02_WASM_RESULTS.md); this packet's original compilation
result alone did not establish that claim.

The development JSON example `kinetic_probe` and `tools/compare_core.py` compare
the following settled checkpoints from **both** accepted strengthened E/F runs:

| Fixture | Selected scheduled-callback ticks |
| --- | --- |
| direct_stop_reverse | 10, 30, 70 |
| small_to_small / large_to_small / small_to_large | 20 each |
| finite_stress | 20, 50, 70, 90, 120 |
| motor_value_settings | 10, 20, 30, 40, 60 |

All 16 checkpoints/62 machine observations match theoretical/effective signed RPM,
stress/capacity and derived operating state. Source RPM, sign-row actions and
connections come from the authored fixture schedule; expected native results are
not populated from the reference speeds. The finite crank's down-facing axis is
−32 at its source, consistent with the observed sign convention. Its capacity 8
and millstone impact 4 per RPM are explicit facts; the motor capacity fact is
16,384 per RPM. Item data and milling progress are outside this kernel.

The negative control deliberately maps the large-to-small adapter edge as equal
cogs. It fails with exit 1 at `large_to_small/020 node 2 theoretical_rpm:
reference -32.0 != native -16.0`. This is a wrong-rule comparison-adapter control,
not a modified Java game or an additional native implementation.

From this repo in WSL:

```sh
./tools/dev.sh cargo test --workspace --locked --offline
./tools/dev.sh cargo build --example kinetic_probe --locked --offline
python3 tools/compare_core.py <accepted-run> --probe target/debug/examples/kinetic_probe
# Expected exit 1, useful difference:
python3 tools/compare_core.py <accepted-run> --probe target/debug/examples/kinetic_probe --negative-control
./tools/dev.sh cargo check --lib --target wasm32-unknown-unknown --locked --offline
```

The comparator first validates the full accepted reference, then requires every
selected probe case and node identity and finite typed values. Invalid/missing
evidence exits 2; a differing state exits 1; full selected equality exits 0. The
example's JSON dependency is development-only and locked; it is not linked into
the player library.

Seven additional derived probe controls are local at
`.local/p02/controls/results.json` in the parent workspace. Boolean numeric/ID
values, NaN, missing cases/nodes and duplicate JSON keys are rejected with exit 2;
a deliberately different state string fails with a useful exit-1 difference.
These controls wrap the real probe and alter only its reported output. They are
software-validator checks, not additional game runs or conformance fixtures.

Independent Sol High review found positive capacity/load multiplication underflow
could silently round to zero. Both paths now return a numerical-range error;
regressions cover the fix. The final follow-up on candidate tree
`9f8427c873fa1522b0039a876da8f9d467106ff5` found no remaining issues. That pass was
strictly read-only; the initial inspection had invoked `git write-tree` despite
the no-write instruction, disclosed without source/index edits. Native execution
was coordinator-verified; the reviewer could not invoke WSL under its permissions.

## Explicit limits

Multiple sources in one component, contradictory ratios, and speeds outside the
measured ±256 envelope return unsupported errors. Non-finite/negative facts,
duplicate/unknown IDs, self/duplicate edges, arithmetic overflow and underflow are
rejected without returning a partial solution. These errors are software boundary
behavior, not claims about which blocks Java Create destroys or how it arbitrates
sources. An inactive source still counts toward the one-source input restriction.

The first packet does not discover any block orientation/geometry, reproduce
network warm-up/rebuild ticks, retain unloaded capacity, arbitrate sources, process
items, save state or instantiate a mod. The full 1,730-sample Java trace is not
claimed to match native timing. P02 as a whole remains incomplete; P03 and live
G2 evidence still depend on the S1 host foundations and G1.
